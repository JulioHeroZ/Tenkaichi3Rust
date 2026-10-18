#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn ReplayMenu_PlayNoData(c: &mut Ctx, mut p_label_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_PlayNoData", &PROBE_N, &[(p_label_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let a0_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_FindLabel(c, a0_, ((0i32) as u32), sym::menu_za_d__str0, fp) };
        { let a0_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_ClipGotoLabel(c, a0_, fp, p_label_1) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_PlayNoData", &PROBE_N, &[], true); }
    r_
}

pub fn ReplayMenu_DrawChips(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_DrawChips", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(272) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 272);
    let mut l_team_1: i32 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_chara_3: i32 = 0;
    let mut l_res_4: u32 = 0;
    let mut l_slot_5: i32 = 0;
    let r_ = 'f: {
        {
            l_team_1 = 0i32;
            'l1: loop {
                if !(((((l_team_1) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    {
                        l_i_2 = 0i32;
                        'l3: loop {
                            if !(((((l_i_2) < (5i32)) as i32)) != 0) { break 'l3; }
                            'c4: {
                                l_chara_3 = { let a_ = ((((c.m.r32(sym::gProgress)).wrapping_add(1692)).wrapping_add(((({ let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(240); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(44))).wrapping_add(4)).wrapping_add((((((l_team_1).wrapping_mul(5i32)).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                                l_res_4 = ({ let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(8); c.m.r32(a_) }).wrapping_add(((((l_chara_3).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
                                l_slot_5 = (l_i_2).wrapping_add((l_team_1).wrapping_mul(5i32));
                                { let v_ = c.m.r32((l_res_4).wrapping_add(16)); let a_ = (((c.m.r32(sym::gReplayMenu)).wrapping_add(140)).wrapping_add((((l_team_1) as u32)).wrapping_mul(20))).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_za_d__str1, &[((l_slot_5) as i64 as u64)]); };
                                { let a0_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_FindLabel(c, a0_, ((0i32) as u32), fp.wrapping_add(8), fp) };
                                { let a0_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a2_ = (({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1692)).wrapping_add(((({ let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(240); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(44)); (c.m.r32(a_) as i32) }) & (1i32)); F::Flash_ClipSetTex(c, a0_, fp, a2_) };
                            }
                            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                        }
                    }
                }
                { let o_ = l_team_1; let n_ = (o_).wrapping_add(1); l_team_1 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_DrawChips", &PROBE_N, &[], true); }
    r_
}

pub fn ReplayMenu_ConfirmPlate(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_ConfirmPlate", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let a0_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_FindLabel(c, a0_, ((0i32) as u32), sym::menu_za_d__str2, fp) };
        { let a0_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_ClipGotoLabel(c, a0_, fp, sym::menu_za_d__str3) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_ConfirmPlate", &PROBE_N, &[], true); }
    r_
}

pub fn ReplayMenu_LightPlate(c: &mut Ctx, mut p_on_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_LightPlate", &PROBE_N, &[(p_on_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let a0_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_FindLabel(c, a0_, ((0i32) as u32), sym::menu_za_d__str4, fp) };
        if ((p_on_1) != 0) {
            { let a0_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_ClipGotoLabel(c, a0_, fp, sym::menu_za_d__str5) };
        } else {
            { let a0_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_ClipGotoLabel(c, a0_, fp, sym::menu_za_d__str6) };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_LightPlate", &PROBE_N, &[], true); }
    r_
}

pub fn ReplayMenu_SetPlayable(c: &mut Ctx, mut p_on_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_SetPlayable", &PROBE_N, &[(p_on_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let a0_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_FindLabel(c, a0_, ((0i32) as u32), sym::menu_za_d__str7, fp) };
        if ((p_on_1) != 0) {
            { let r_ = 16i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(248); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            if ((((!(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1676); (c.m.r32(a_) as i32) }) & (1i32))) != 0)) as i32)) != 0) {
                { let a0_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_ClipGotoLabel(c, a0_, fp, sym::menu_za_d__str8) };
            }
        } else {
            if (((({ let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(248); (c.m.r32(a_) as i32) }) & (16i32))) != 0) {
                { let r_ = 16i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(248); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
            }
            if ((((!(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1676); (c.m.r32(a_) as i32) }) & (1i32))) != 0)) as i32)) != 0) {
                { let a0_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_ClipGotoLabel(c, a0_, fp, sym::menu_za_d__str9) };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_SetPlayable", &PROBE_N, &[], true); }
    r_
}

pub fn menu_za_d__ReplayMenu_HasData(c: &mut Ctx, mut p_slot_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_za_d__ReplayMenu_HasData", &PROBE_N, &[(p_slot_1) as u32], false); }
    let r_ = 'f: {
    if (((({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1692)).wrapping_add((((p_slot_1) as u32)).wrapping_mul(44)); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_za_d__ReplayMenu_HasData", &PROBE_N, &[], true); }
    r_
}

pub fn ReplayMenu_RefreshSlot(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_RefreshSlot", &PROBE_N, &[], false); }
    let mut l_has_1: i32 = 0;
    let r_ = 'f: {
    l_has_1 = { let a0_ = { let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(240); (c.m.r32(a_) as i32) }; F::menu_za_d__ReplayMenu_HasData(c, a0_) };
    if ((((((((!(((({ let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(248); (c.m.r32(a_) as i32) }) & (16i32))) != 0)) as i32)) != 0) && ((((!((l_has_1) != 0)) as i32)) != 0)) as i32)) != 0) {
        F::ReplayMenu_PlayNoData(c, sym::menu_za_d__str10);
        break 'f;
    } else {
        if (((((((({ let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(248); (c.m.r32(a_) as i32) }) & (16i32))) != 0) && ((l_has_1) != 0)) as i32)) != 0) {
            break 'f;
        } else {
            if ((((((((!(((({ let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(248); (c.m.r32(a_) as i32) }) & (16i32))) != 0)) as i32)) != 0) && ((l_has_1) != 0)) as i32)) != 0) {
                F::ReplayMenu_PlayNoData(c, sym::menu_za_d__str11);
            } else {
                if (((((((({ let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(248); (c.m.r32(a_) as i32) }) & (16i32))) != 0) && ((((!((l_has_1) != 0)) as i32)) != 0)) as i32)) != 0) {
                    F::ReplayMenu_PlayNoData(c, sym::menu_za_d__str12);
                }
            }
        }
    }
    F::ReplayMenu_SetPlayable(c, l_has_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_RefreshSlot", &PROBE_N, &[], true); }
    r_
}

pub fn menu_za_d__ReplayMenu_Leave(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_za_d__ReplayMenu_Leave", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let r_ = 1i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(248); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1676); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
        { let v_ = (if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1572); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) { 40i32 } else { 39i32 }); let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(224); c.m.w32(a_, (v_) as u32); };
    } else {
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(224); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_za_d__ReplayMenu_Leave", &PROBE_N, &[], true); }
    r_
}

pub fn ReplayMenu_OnLoadDone(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_OnLoadDone", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(236); c.m.w32(a_, (v_) as u32); };
    { let v_ = 1i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(224); c.m.w32(a_, (v_) as u32); };
    { let r_ = 1i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(248); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_OnLoadDone", &PROBE_N, &[], true); }
    r_
}

pub fn ReplayMenu_OnLoadFailed(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_OnLoadFailed", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(232); c.m.w32(a_, (v_) as u32); };
    F::Progress_ClearTeams(c);
    F::ReplayMenu_RefreshSlot(c);
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(232); c.m.w32(a_, (v_) as u32); };
    F::menu_za_d__ReplayMenu_Leave(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_OnLoadFailed", &PROBE_N, &[], true); }
    r_
}

pub fn ReplayMenu_OnLoadRefused(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_OnLoadRefused", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(232); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_OnLoadRefused", &PROBE_N, &[], true); }
    r_
}

pub fn ReplayMenu_OnSaveDone(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_OnSaveDone", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(236); c.m.w32(a_, (v_) as u32); };
    { let v_ = (if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1572); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) { 40i32 } else { 39i32 }); let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(224); c.m.w32(a_, (v_) as u32); };
    { let r_ = 1i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(248); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_OnSaveDone", &PROBE_N, &[], true); }
    r_
}

pub fn ReplayMenu_OnSaveFailed(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_OnSaveFailed", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(232); c.m.w32(a_, (v_) as u32); };
    F::Progress_ClearTeams(c);
    F::ReplayMenu_RefreshSlot(c);
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(232); c.m.w32(a_, (v_) as u32); };
    F::menu_za_d__ReplayMenu_Leave(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_OnSaveFailed", &PROBE_N, &[], true); }
    r_
}

pub fn ReplayMenu_OnSaveRefused(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_OnSaveRefused", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(232); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_OnSaveRefused", &PROBE_N, &[], true); }
    r_
}

pub fn ReplayMenu_OnScanDone(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_OnScanDone", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(232); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(236); c.m.w32(a_, (v_) as u32); };
    { let r_ = 8i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(248); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    F::ReplayMenu_RefreshSlot(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_OnScanDone", &PROBE_N, &[], true); }
    r_
}

pub fn ReplayMenu_OnScanFailed(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_OnScanFailed", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Progress_ClearTeams(c);
    F::ReplayMenu_RefreshSlot(c);
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(232); c.m.w32(a_, (v_) as u32); };
    F::menu_za_d__ReplayMenu_Leave(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_OnScanFailed", &PROBE_N, &[], true); }
    r_
}

pub fn ReplayMenu_OnScanRefused(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_OnScanRefused", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_OnScanRefused", &PROBE_N, &[], true); }
    r_
}

pub fn ReplayMenu_OnAskYes(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_OnAskYes", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(232); c.m.w32(a_, (v_) as u32); };
    F::ReplayMenu_RefreshSlot(c);
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(232); c.m.w32(a_, (v_) as u32); };
    F::menu_za_d__ReplayMenu_Leave(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_OnAskYes", &PROBE_N, &[], true); }
    r_
}

pub fn ReplayMenu_OnAskFailed(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_OnAskFailed", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(232); c.m.w32(a_, (v_) as u32); };
    F::ReplayMenu_RefreshSlot(c);
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(232); c.m.w32(a_, (v_) as u32); };
    F::menu_za_d__ReplayMenu_Leave(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_OnAskFailed", &PROBE_N, &[], true); }
    r_
}

pub fn ReplayMenu_OnAskNo(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_OnAskNo", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(232); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_OnAskNo", &PROBE_N, &[], true); }
    r_
}

pub fn ReplayMenu_OnRemovedDone(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_OnRemovedDone", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(232); c.m.w32(a_, (v_) as u32); };
    F::ReplayMenu_RefreshSlot(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_OnRemovedDone", &PROBE_N, &[], true); }
    r_
}

pub fn ReplayMenu_OnRemovedFailed(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_OnRemovedFailed", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(232); c.m.w32(a_, (v_) as u32); };
    F::menu_za_d__ReplayMenu_Leave(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_OnRemovedFailed", &PROBE_N, &[], true); }
    r_
}

pub fn ReplayMenu_OnRemovedRefused(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_OnRemovedRefused", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_OnRemovedRefused", &PROBE_N, &[], true); }
    r_
}

pub fn ReplayMenu_OnCardRemoved(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_OnCardRemoved", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Progress_ClearTeams(c);
    F::ReplayMenu_RefreshSlot(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_OnCardRemoved", &PROBE_N, &[], true); }
    r_
}

pub fn menu_za_d__ReplayMenu_Section(c: &mut Ctx, mut p_n_1: i32, mut p_host_2: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_za_d__ReplayMenu_Section", &PROBE_N, &[(p_n_1) as u32, (p_host_2) as u32], false); }
    let r_ = 'f: {
    break 'f ({ let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((p_n_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_za_d__ReplayMenu_Section", &PROBE_N, &[], true); }
    r_
}

pub fn ReplayMenu_Init(c: &mut Ctx, mut p_section_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_Init", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = F::Heap_Alloc(c, ((260u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gReplayMenu; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gReplayMenu); F::memset(c, a0_, 0i32, 260u32) }; };
        { let v_ = (c.m.r32(sym::gMenuArc8)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc8)).wrapping_add((((p_section_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = c.m.r32(sym::gReplayMenu); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gReplayMenu); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = F::menu_za_d__ReplayMenu_Section(c, 6i32, sym::menu_za_d__str13); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = c.m.r32(fp); let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(56); c.m.w32(a_, v_); };
        { let v_ = F::menu_za_d__ReplayMenu_Section(c, 7i32, sym::menu_za_d__str14); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(60)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(60)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(60)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(60)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(60)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(60)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(60)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::menu_za_d__ReplayMenu_Section(c, 3i32, sym::menu_za_d__str15); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(60)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(60)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(60)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(60)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(60)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(60)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(60)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((7i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(60)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((8i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(180)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((9i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(180)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((10i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(180)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((11i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(180)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((12i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(180)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((13i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(180)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::menu_za_d__ReplayMenu_Section(c, 4i32, sym::menu_za_d__str16); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(60)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(60)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(60)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(60)).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(180)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(180)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(180)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::menu_za_d__ReplayMenu_Section(c, 2i32, sym::menu_za_d__str17); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(60)).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::menu_za_d__ReplayMenu_Section(c, 8i32, sym::menu_za_d__str18); let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(8); c.m.w32(a_, v_); };
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (165i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = ({ let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(8); c.m.r32(a_) }).wrapping_add(((((l_i_2).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let a0_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a1_ = F::menu_za_d__ReplayMenu_Section(c, 1i32, sym::menu_za_d__str19); let a2_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(60); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let v_ = F::menu_za_d__ReplayMenu_Section(c, 9i32, sym::menu_za_d__str20); let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(216); c.m.w32(a_, v_); };
        { let a0_ = F::menu_za_d__ReplayMenu_Section(c, 5i32, sym::menu_za_d__str21); F::Dialog_Init(c, a0_, ((0i32) as u32), 0i32) };
        { let _ = 1i32; F::McFlow_Init(c) };
        F::McFlow_SetModeCb(c, 7i32, 0i32, fid::ReplayMenu_OnScanDone, 0i32);
        F::McFlow_SetModeCb(c, 7i32, 1i32, fid::ReplayMenu_OnScanFailed, 0i32);
        F::McFlow_SetModeCb(c, 7i32, 2i32, fid::ReplayMenu_OnScanRefused, 0i32);
        F::McFlow_SetModeCb(c, 7i32, 3i32, fid::ReplayMenu_OnCardRemoved, 0i32);
        F::McFlow_SetModeCb(c, 8i32, 0i32, fid::ReplayMenu_OnAskYes, 0i32);
        F::McFlow_SetModeCb(c, 8i32, 1i32, fid::ReplayMenu_OnAskFailed, 0i32);
        F::McFlow_SetModeCb(c, 8i32, 2i32, fid::ReplayMenu_OnAskNo, 0i32);
        F::McFlow_SetModeCb(c, 8i32, 3i32, fid::ReplayMenu_OnCardRemoved, 0i32);
        F::McFlow_SetModeCb(c, 9i32, 0i32, fid::ReplayMenu_OnRemovedDone, 0i32);
        F::McFlow_SetModeCb(c, 9i32, 1i32, fid::ReplayMenu_OnRemovedFailed, 0i32);
        F::McFlow_SetModeCb(c, 9i32, 2i32, fid::ReplayMenu_OnRemovedRefused, 0i32);
        F::McFlow_SetModeCb(c, 9i32, 3i32, fid::ReplayMenu_OnCardRemoved, 0i32);
        F::McFlow_SetModeCb(c, 6i32, 0i32, fid::ReplayMenu_OnLoadDone, 0i32);
        F::McFlow_SetModeCb(c, 6i32, 1i32, fid::ReplayMenu_OnLoadFailed, 0i32);
        F::McFlow_SetModeCb(c, 6i32, 2i32, fid::ReplayMenu_OnLoadRefused, 0i32);
        F::McFlow_SetModeCb(c, 6i32, 3i32, fid::ReplayMenu_OnCardRemoved, 0i32);
        F::McFlow_SetModeCb(c, 5i32, 0i32, fid::ReplayMenu_OnSaveDone, 0i32);
        F::McFlow_SetModeCb(c, 5i32, 1i32, fid::ReplayMenu_OnSaveFailed, 0i32);
        F::McFlow_SetModeCb(c, 5i32, 2i32, fid::ReplayMenu_OnSaveRefused, 0i32);
        F::McFlow_SetModeCb(c, 5i32, 3i32, fid::ReplayMenu_OnCardRemoved, 0i32);
        F::Progress_ClearTeams(c);
        { let v_ = 30i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(244); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1688); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(240); c.m.w32(a_, (v_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_Init", &PROBE_N, &[], true); }
    r_
}

pub fn ReplayMenu_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_Update", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(12)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Advance(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((!(((({ let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(248); (c.m.r32(a_) as i32) }) & (4i32))) != 0)) as i32)) != 0) {
        F::Progress_ClearTeams(c);
        F::ReplayMenu_PlayNoData(c, sym::menu_za_d__str22);
        { let a0_ = (({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1676); (c.m.r32(a_) as i32) }) & (1i32)); F::ReplayMenu_LightPlate(c, a0_) };
        { let v_ = 1i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(232); c.m.w32(a_, (v_) as u32); };
        F::Dialog_SetLayout(c, 1i32);
        F::McFlow_Start(c, 7i32);
        { let r_ = 4i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(248); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_Update", &PROBE_N, &[], true); }
    r_
}

pub fn ReplayMenu_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    F::Dialog_Term(c);
    F::McFlow_Term(c);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(12)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Destroy(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gReplayMenu)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gReplayMenu); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gReplayMenu; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_Term", &PROBE_N, &[], true); }
    r_
}

pub fn menu_za_d__ReplayMenu_SetUv(c: &mut Ctx, mut p_parent_1: u32, mut p_name_2: u32, mut p_uv_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_za_d__ReplayMenu_SetUv", &PROBE_N, &[(p_parent_1) as u32, (p_name_2) as u32, (p_uv_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let a0_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_FindLabel(c, a0_, p_parent_1, p_name_2, fp) };
        { let a0_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_ClipSetUv(c, a0_, fp, p_uv_3) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "menu_za_d__ReplayMenu_SetUv", &PROBE_N, &[], true); }
    r_
}

pub fn menu_za_d__ReplayMenu_SetCell(c: &mut Ctx, mut p_uv_1: u32, mut p_col_2: i32, mut p_row_3: i32, mut p_w_4: i32, mut p_h_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_za_d__ReplayMenu_SetCell", &PROBE_N, &[(p_uv_1) as u32, (p_col_2) as u32, (p_row_3) as u32, (p_w_4) as u32, (p_h_5) as u32], false); }
    let r_ = 'f: {
    { let v_ = (p_col_2).wrapping_mul(p_w_4); let a_ = p_uv_1; c.m.w32(a_, (v_) as u32); };
    { let v_ = ((c.m.r32(p_uv_1) as i32)).wrapping_add(p_w_4); let a_ = (p_uv_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = (p_row_3).wrapping_mul(p_h_5); let a_ = (p_uv_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((c.m.r32((p_uv_1).wrapping_add(4)) as i32)).wrapping_add(p_h_5); let a_ = (p_uv_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_za_d__ReplayMenu_SetCell", &PROBE_N, &[], true); }
    r_
}

pub fn ReplayMenu_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(544) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 544);
    let mut l_num_1: i32 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_digit_3: i32 = 0;
    let r_ = 'f: {
        {
            F::menu_za_d__ReplayMenu_SetCell(c, fp.wrapping_add(520), 0i32, 1i32, 32i32, 32i32);
            F::menu_za_d__ReplayMenu_SetUv(c, sym::menu_za_d__str23, sym::menu_za_d__str24, fp.wrapping_add(520));
            F::menu_za_d__ReplayMenu_SetCell(c, fp.wrapping_add(520), 1i32, 1i32, 32i32, 32i32);
            F::menu_za_d__ReplayMenu_SetUv(c, sym::menu_za_d__str25, sym::menu_za_d__str26, fp.wrapping_add(520));
        }
        if ((((({ let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(240); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            { let _ = ((((F::strcpy(c, fp.wrapping_add(8), sym::menu_za_d__str27)) as u64)) as i32); };
            { let _ = ((((F::strcpy(c, fp.wrapping_add(264), sym::menu_za_d__str28)) as u64)) as i32); };
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(240); (c.m.r32(a_) as i32) }) == ((7i32).wrapping_sub(1i32))) as i32)) != 0) {
                { let _ = ((((F::strcpy(c, fp.wrapping_add(8), sym::menu_za_d__str29)) as u64)) as i32); };
                { let _ = ((((F::strcpy(c, fp.wrapping_add(264), sym::menu_za_d__str30)) as u64)) as i32); };
            }
        }
        { let a0_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_FindLabel(c, a0_, fp.wrapping_add(8), fp.wrapping_add(264), fp) };
        { let a0_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); { let _ = F::Flash_ClipSetFlags(c, a0_, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); } };
        l_num_1 = ({ let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(240); (c.m.r32(a_) as i32) }).wrapping_add(1i32);
        {
            l_i_2 = 1i32;
            'l1: loop {
                if !(((((l_i_2) >= (0i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_za_d__str31, &[((l_i_2) as i64 as u64)]); };
                    l_digit_3 = rt::div_i32(l_num_1, rt::d2i(F::pow(c, f64::from_bits(0x4024000000000000), ((l_i_2) as f64))));
                    { let r_ = (l_digit_3).wrapping_mul(rt::d2i(F::pow(c, f64::from_bits(0x4024000000000000), ((l_i_2) as f64)))); let n_ = (l_num_1).wrapping_sub(r_); l_num_1 = n_; };
                    F::menu_za_d__ReplayMenu_SetCell(c, fp.wrapping_add(520), rt::rem_i32(l_digit_3, 4i32), rt::div_i32(l_digit_3, 4i32), 32i32, 32i32);
                    F::menu_za_d__ReplayMenu_SetUv(c, ((0i32) as u32), fp.wrapping_add(8), fp.wrapping_add(520));
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_sub(1); l_i_2 = n_; };
            }
        }
        {
            { let a2_ = (({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1676); (c.m.r32(a_) as i32) }) & (1i32)); F::menu_za_d__ReplayMenu_SetCell(c, fp.wrapping_add(520), 0i32, a2_, 256i32, 32i32) };
            F::menu_za_d__ReplayMenu_SetUv(c, sym::menu_za_d__str32, sym::menu_za_d__str33, fp.wrapping_add(520));
            { let a2_ = (({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1676); (c.m.r32(a_) as i32) }) & (1i32)); F::menu_za_d__ReplayMenu_SetCell(c, fp.wrapping_add(520), 0i32, a2_, 256i32, 32i32) };
            F::menu_za_d__ReplayMenu_SetUv(c, sym::menu_za_d__str34, sym::menu_za_d__str35, fp.wrapping_add(520));
        }
        F::ReplayMenu_DrawChips(c);
        { let a0_ = { let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(56); c.m.r32(a_) }; F::Sprite_DrawPicture(c, a0_, 0i32, 0i32, 128i32) };
        { let a0_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
        { let v_ = F::McFlow_Update(c); let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(256); c.m.w32(a_, (v_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn ReplayMenu_WatchCard(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_WatchCard", &PROBE_N, &[], false); }
    let mut l_ret_1: i32 = 0;
    let r_ = 'f: {
    if ((((((((({ let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(232); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) || ((((({ let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(232); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0)) as i32)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(232); (c.m.r32(a_) as i32) }) == (4i32)) as i32)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(232); (c.m.r32(a_) as i32) }) == (3i32)) as i32)) != 0) {
        break 'f;
    }
    if (((({ let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(248); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(256); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
        break 'f;
    }
    l_ret_1 = F::McFlow_PollCard(c);
    if (((((l_ret_1) == (0i32)) as i32)) != 0) {
        break 'f;
    }
    's1: {
        let sv2: i32 = l_ret_1;
        match sv2 {
            2i32 => {
                if (((({ let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(248); (c.m.r32(a_) as i32) }) & (8i32))) != 0) {
                    F::Progress_ClearTeams(c);
                    F::ReplayMenu_RefreshSlot(c);
                    { let v_ = 4i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(232); c.m.w32(a_, (v_) as u32); };
                    F::Dialog_SetLayout(c, 1i32);
                    F::McFlow_Start(c, 9i32);
                }
                break 's1;
            }
            1i32 => {
                if ((((((((!(((({ let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(248); (c.m.r32(a_) as i32) }) & (8i32))) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(232); (c.m.r32(a_) as i32) }) != (3i32)) as i32)) != 0)) as i32)) != 0) {
                    { let r_ = 8i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(248); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = l_ret_1; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(232); c.m.w32(a_, (v_) as u32); };
                    F::Dialog_SetLayout(c, 1i32);
                    F::McFlow_Start(c, 7i32);
                }
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_WatchCard", &PROBE_N, &[], true); }
    r_
}

pub fn menu_za_d__ReplayMenu_ClampCursor(c: &mut Ctx, mut p_cursor_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_za_d__ReplayMenu_ClampCursor", &PROBE_N, &[(p_cursor_1) as u32], false); }
    let mut l_max_2: i32 = 0;
    let r_ = 'f: {
    l_max_2 = (7i32).wrapping_sub(1i32);
    if ((((((c.m.r32(p_cursor_1) as i32)) < (0i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = p_cursor_1; c.m.w32(a_, (v_) as u32); };
        break 'f 0i32;
    }
    if ((((((c.m.r32(p_cursor_1) as i32)) > (l_max_2)) as i32)) != 0) {
        { let v_ = l_max_2; let a_ = p_cursor_1; c.m.w32(a_, (v_) as u32); };
        break 'f 0i32;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_za_d__ReplayMenu_ClampCursor", &PROBE_N, &[], true); }
    r_
}

pub fn ReplayMenu_Input(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_Input", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((((((({ let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(232); (c.m.r32(a_) as i32) }) == (3i32)) as i32)) != 0) && ((F::Dialog_IsClosed(c)) != 0)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(232); c.m.w32(a_, (v_) as u32); };
    }
    if ((((!(((({ let a_ = (((c.m.r32(sym::gReplayMenu)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f;
    }
    if (((({ let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(248); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
        break 'f;
    }
    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(256); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(232); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
        break 'f;
    }
    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
        { let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(240); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
        if (({ let a0_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(240); F::menu_za_d__ReplayMenu_ClampCursor(c, a0_) }) != 0) {
            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
            F::ReplayMenu_RefreshSlot(c);
            { let a0_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_za_d__str36, ((((1i32) as i64 as u64)) as u8)) };
        }
    } else {
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
            { let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(240); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            if (({ let a0_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(240); F::menu_za_d__ReplayMenu_ClampCursor(c, a0_) }) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                F::ReplayMenu_RefreshSlot(c);
                { let a0_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_za_d__str37, ((((1i32) as i64 as u64)) as u8)) };
            }
        } else {
            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1676); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                    { let v_ = 2i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(232); c.m.w32(a_, (v_) as u32); };
                    F::Dialog_SetLayout(c, 1i32);
                    { let a0_ = { let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(240); (c.m.r32(a_) as i32) }; F::McFlow_SetSlot(c, a0_) };
                    F::McFlow_Start(c, 5i32);
                    F::ReplayMenu_ConfirmPlate(c);
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                } else {
                    if (((({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1692)).wrapping_add(((({ let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(240); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(44)); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                        { let v_ = 2i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(232); c.m.w32(a_, (v_) as u32); };
                        F::Dialog_SetLayout(c, 1i32);
                        { let a0_ = { let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(240); (c.m.r32(a_) as i32) }; F::McFlow_SetSlot(c, a0_) };
                        F::McFlow_Start(c, 6i32);
                        F::ReplayMenu_ConfirmPlate(c);
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    } else {
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                    }
                }
            } else {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1676); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                        { let v_ = 3i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(232); c.m.w32(a_, (v_) as u32); };
                        F::Dialog_SetLayout(c, 1i32);
                        F::McFlow_Start(c, 8i32);
                    } else {
                        { let r_ = 1i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(248); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(224); c.m.w32(a_, (v_) as u32); };
                    }
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_Input", &PROBE_N, &[], true); }
    r_
}

pub fn ReplayMenu_CheckLeave(c: &mut Ctx, mut p_result_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_CheckLeave", &PROBE_N, &[(p_result_1) as u32], false); }
    let r_ = 'f: {
    if (((((((({ let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(248); (c.m.r32(a_) as i32) }) & (1i32))) != 0) && ((((({ let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(232); (c.m.r32(a_) as i32) }) != (3i32)) as i32)) != 0)) as i32)) != 0) {
        if ((((!(((({ let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(248); (c.m.r32(a_) as i32) }) & (2i32))) != 0)) as i32)) != 0) {
            { let r_ = 2i32; let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(248); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            { let a0_ = ((c.m.r32(sym::gReplayMenu)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_za_d__str38, ((((1i32) as i64 as u64)) as u8)) };
            F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
        }
        if ((((({ let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(244); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) == (0i32)) as i32)) != 0) {
            { let v_ = { let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(224); (c.m.r32(a_) as i32) }; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
            { let v_ = { let a_ = (c.m.r32(sym::gReplayMenu)).wrapping_add(240); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1688); c.m.w32(a_, (v_) as u32); };
            break 'f 1i32;
        }
        if ((F::ColorFade_IsFadingOut(c)) != 0) {
            F::Bgm_FadeOutStep(c);
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_CheckLeave", &PROBE_N, &[], true); }
    r_
}

pub fn ReplayMenu_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        F::ReplayMenu_Init(c, p_section_1);
        F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
        'l1: while ((1i32) != 0) {
            F::Gfx_BeginFrame(c);
            F::Pad_Update(c);
            F::ColorFade_Update(c);
            F::Snd_Update(c);
            F::ReplayMenu_Update(c);
            F::ReplayMenu_Draw(c);
            F::ColorFade_Draw(c);
            F::Gfx_EndFrame(c, 1i32);
            F::Dma_Flush(c);
            F::File_Stub264D90(c);
            if ((F::ReplayMenu_CheckLeave(c, fp)) != 0) {
                break 'l1;
            }
            F::ReplayMenu_Input(c);
            F::ReplayMenu_WatchCard(c);
        }
        F::ReplayMenu_Term(c);
        F::Dma_ResetBuffers(c);
        if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1676); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
            { let r_ = 1i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1676); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
        }
        break 'f (c.m.r32(fp) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ReplayMenu_Run", &PROBE_N, &[], true); }
    r_
}
