#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlReplay_Rewind(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlReplay_Rewind", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_trk_2: u32 = 0;
    let r_ = 'f: {
    l_trk_2 = (F::BattleReplay_GetData(c)).wrapping_add(((((c.m.r32(p_chr_1) as i32)) as u32)).wrapping_mul(54008));
    { let v_ = 0i32; let a_ = (l_trk_2).wrapping_add(54004); c.m.w32(a_, (v_) as u32); };
    if ((((!((F::BattleReplay_IsActive(c)) != 0)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (l_trk_2).wrapping_add(54000); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlReplay_Rewind", &PROBE_N, &[], true); }
    r_
}

pub fn BtlReplay_Record(c: &mut Ctx, mut p_chr_1: u32, mut p_buttons_2: u32, mut p_commands_3: u32, mut p_stick_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlReplay_Record", &PROBE_N, &[(p_chr_1) as u32, (p_buttons_2) as u32, (p_commands_3) as u32, (p_stick_4) as u32], false); }
    let mut l_data_5: u32 = 0;
    let mut l_trk_6: u32 = 0;
    let r_ = 'f: {
    l_data_5 = F::BattleReplay_GetData(c);
    l_trk_6 = (l_data_5).wrapping_add(((((c.m.r32(p_chr_1) as i32)) as u32)).wrapping_mul(54008));
    if ((((((c.m.r32((l_trk_6).wrapping_add(54000)) as i32)) >= (9000i32)) as i32)) != 0) {
        { let r_ = 1i32; let a_ = (l_data_5).wrapping_add(108016); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        break 'f;
    }
    { let v_ = c.m.r8((p_stick_4).wrapping_add(((0i32) as u32))); let a_ = ((l_trk_6).wrapping_add(((((c.m.r32((l_trk_6).wrapping_add(54000)) as i32)) as u32)).wrapping_mul(2))).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = c.m.r8((p_stick_4).wrapping_add(((1i32) as u32))); let a_ = ((l_trk_6).wrapping_add(((((c.m.r32((l_trk_6).wrapping_add(54000)) as i32)) as u32)).wrapping_mul(2))).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = p_buttons_2; let a_ = ((l_trk_6).wrapping_add(18000)).wrapping_add(((((c.m.r32((l_trk_6).wrapping_add(54000)) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let a_ = (l_trk_6).wrapping_add(54000); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlReplay_Record", &PROBE_N, &[], true); }
    r_
}

pub fn BtlReplay_Play(c: &mut Ctx, mut p_chr_1: u32, mut p_buttons_2: u32, mut p_stick_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlReplay_Play", &PROBE_N, &[(p_chr_1) as u32, (p_buttons_2) as u32, (p_stick_3) as u32], false); }
    let mut l_data_4: u32 = 0;
    let mut l_trk_5: u32 = 0;
    let r_ = 'f: {
    l_data_4 = F::BattleReplay_GetData(c);
    l_trk_5 = (l_data_4).wrapping_add(((((c.m.r32(p_chr_1) as i32)) as u32)).wrapping_mul(54008));
    if ((((((c.m.r32((l_trk_5).wrapping_add(54004)) as i32)) >= ((c.m.r32((l_trk_5).wrapping_add(54000)) as i32))) as i32)) != 0) {
        { let r_ = 1i32; let a_ = (l_data_4).wrapping_add(108016); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let v_ = ((0i32) as u32); let a_ = p_buttons_2; c.m.w32(a_, v_); };
        { let v_ = { let v_ = ((127i32) as u8); let a_ = (p_stick_3).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); v_ }; let a_ = (p_stick_3).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
        break 'f;
    }
    { let v_ = { let a_ = ((l_trk_5).wrapping_add(((((c.m.r32((l_trk_5).wrapping_add(54004)) as i32)) as u32)).wrapping_mul(2))).wrapping_add(((0i32) as u32)); c.m.r8(a_) }; let a_ = (p_stick_3).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = { let a_ = ((l_trk_5).wrapping_add(((((c.m.r32((l_trk_5).wrapping_add(54004)) as i32)) as u32)).wrapping_mul(2))).wrapping_add(((1i32) as u32)); c.m.r8(a_) }; let a_ = (p_stick_3).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = { let a_ = ((l_trk_5).wrapping_add(18000)).wrapping_add(((((c.m.r32((l_trk_5).wrapping_add(54004)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = p_buttons_2; c.m.w32(a_, v_); };
    { let a_ = (l_trk_5).wrapping_add(54004); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlReplay_Play", &PROBE_N, &[], true); }
    r_
}

pub fn BtlReplay_ResetViewer(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlReplay_ResetViewer", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlChars)) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = ((0i32) as u64); let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(304); c.m.w64(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlReplay_ResetViewer", &PROBE_N, &[], true); }
    r_
}

pub fn BtlReplay_UpdateViewer(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlReplay_UpdateViewer", &PROBE_N, &[], false); }
    let mut l_pad_1: u32 = 0;
    let mut l_v_2: u32 = 0;
    let r_ = 'f: {
    l_pad_1 = (sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448));
    if (((((c.m.r32(sym::gBtlChars)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    l_v_2 = (c.m.r32(sym::gBtlChars)).wrapping_add(304);
    if ((((!((F::BattleReplay_IsActive(c)) != 0)) as i32)) != 0) {
        break 'f;
    }
    if (((({ let a_ = (F::BattleReplay_GetData(c)).wrapping_add(108016); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
        break 'f;
    }
    if ((((c.m.r32((l_pad_1).wrapping_add(396))) & (((8i32) as u32)))) != 0) {
        { let a_ = l_v_2; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    }
    if ((((c.m.r32((l_pad_1).wrapping_add(396))) & (((4i32) as u32)))) != 0) {
        { let a_ = l_v_2; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    }
    if ((((((c.m.r32(l_v_2) as i32)) < (0i32)) as i32)) != 0) {
        { let v_ = 1i32; let a_ = l_v_2; c.m.w32(a_, (v_) as u32); };
    }
    if ((((((c.m.r32(l_v_2) as i32)) >= (2i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = l_v_2; c.m.w32(a_, (v_) as u32); };
    }
    if ((((((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) && ((((!((F::BtlMenu_IsActive(c)) != 0)) as i32)) != 0)) as i32)) != 0) {
        if ((((c.m.r32((l_pad_1).wrapping_add(396))) & (((8i32) as u32)))) != 0) {
            { let a_ = sym::gBtlReplayHudMode; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
        }
        if ((((c.m.r32((l_pad_1).wrapping_add(396))) & (((4i32) as u32)))) != 0) {
            { let a_ = sym::gBtlReplayHudMode; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        if ((((((c.m.r32(sym::gBtlReplayHudMode) as i32)) >= (3i32)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = sym::gBtlReplayHudMode; c.m.w32(a_, (v_) as u32); };
        }
        if ((((((c.m.r32(sym::gBtlReplayHudMode) as i32)) < (0i32)) as i32)) != 0) {
            { let v_ = 2i32; let a_ = sym::gBtlReplayHudMode; c.m.w32(a_, (v_) as u32); };
        }
        { let a0_ = (c.m.r32(sym::gBtlReplayHudMode) as i32); F::Hud_SetReplayMode(c, a0_) };
        { let v_ = 0i32; let a_ = l_v_2; c.m.w32(a_, (v_) as u32); };
    } else {
        { let v_ = 0i32; let a_ = l_v_2; c.m.w32(a_, (v_) as u32); };
    }
    if ((((c.m.r32((l_pad_1).wrapping_add(396))) & (((1i32) as u32)))) != 0) {
        { let a_ = (l_v_2).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    }
    if ((((c.m.r32((l_pad_1).wrapping_add(396))) & (((2i32) as u32)))) != 0) {
        { let a_ = (l_v_2).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    }
    if ((((((c.m.r32(l_v_2) as i32)) == (0i32)) as i32)) != 0) {
        if ((((((c.m.r32((l_v_2).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
            { let v_ = 1i32; let a_ = (l_v_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        }
        if ((((((c.m.r32((l_v_2).wrapping_add(4)) as i32)) >= (2i32)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = (l_v_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlReplay_UpdateViewer", &PROBE_N, &[], true); }
    r_
}

pub fn BtlReplay_GetViewSide(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlReplay_GetViewSide", &PROBE_N, &[], false); }
    let mut l_v_1: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlChars)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_v_1 = (c.m.r32(sym::gBtlChars)).wrapping_add(304);
    if ((((((c.m.r32(l_v_1) as i32)) == (0i32)) as i32)) != 0) {
        break 'f (c.m.r32((l_v_1).wrapping_add(4)) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlReplay_GetViewSide", &PROBE_N, &[], true); }
    r_
}
