#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn HudGauge_GsBeginMask(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_GsBeginMask", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 96);
        { let v_ = ((((268435456i32) | (5i32))) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((((1342177280i32) | (5i32))) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((((32768i32) | (4i32))) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = ((212992i32) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((71i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { ((((((((112i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) } else { ((((((((0i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) })) as u32); c.m.w32(fp.wrapping_add(48), v_); }
        { let v_ = ((16777215i32) as u32); c.m.w32(fp.wrapping_add(52), v_); }
        { let v_ = ((77i32) as u32); c.m.w32(fp.wrapping_add(56), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(60), v_); }
        { let v_ = ((1i32) as u32); c.m.w32(fp.wrapping_add(64), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(68), v_); }
        { let v_ = ((75i32) as u32); c.m.w32(fp.wrapping_add(72), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(76), v_); }
        { let v_ = ((196623i32) as u32); c.m.w32(fp.wrapping_add(80), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(84), v_); }
        { let v_ = ((72i32) as u32); c.m.w32(fp.wrapping_add(88), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(92), v_); }
        F::Dma_AddData(c, fp, ((96u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_GsBeginMask", &PROBE_N, &[], true); }
    r_
}

pub fn HudGauge_GsEndMask(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_GsEndMask", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 96);
        { let v_ = ((((268435456i32) | (5i32))) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((((1342177280i32) | (5i32))) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((((32768i32) | (4i32))) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = ((196608i32) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((71i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { ((((((((112i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) } else { ((((((((0i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) })) as u32); c.m.w32(fp.wrapping_add(48), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(52), v_); }
        { let v_ = ((77i32) as u32); c.m.w32(fp.wrapping_add(56), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(60), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(64), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(68), v_); }
        { let v_ = ((75i32) as u32); c.m.w32(fp.wrapping_add(72), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(76), v_); }
        { let v_ = ((196608i32) as u32); c.m.w32(fp.wrapping_add(80), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(84), v_); }
        { let v_ = ((72i32) as u32); c.m.w32(fp.wrapping_add(88), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(92), v_); }
        F::Dma_AddData(c, fp, ((96u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_GsEndMask", &PROBE_N, &[], true); }
    r_
}

pub fn HudGauge_GsBeginMask2(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_GsBeginMask2", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(128) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 128);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 128);
        { let v_ = ((((268435456i32) | (7i32))) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((((1342177280i32) | (7i32))) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((((32768i32) | (6i32))) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = ((72i32) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((67i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        { let v_ = ((245760i32) as u32); c.m.w32(fp.wrapping_add(48), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(52), v_); }
        { let v_ = ((72i32) as u32); c.m.w32(fp.wrapping_add(56), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(60), v_); }
        { let v_ = ((198651i32) as u32); c.m.w32(fp.wrapping_add(64), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(68), v_); }
        { let v_ = ((71i32) as u32); c.m.w32(fp.wrapping_add(72), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(76), v_); }
        { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { ((((((((112i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) } else { ((((((((0i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) })) as u32); c.m.w32(fp.wrapping_add(80), v_); }
        { let v_ = ((16777215i32) as u32); c.m.w32(fp.wrapping_add(84), v_); }
        { let v_ = ((76i32) as u32); c.m.w32(fp.wrapping_add(88), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(92), v_); }
        { let v_ = ((1i32) as u32); c.m.w32(fp.wrapping_add(96), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(100), v_); }
        { let v_ = ((74i32) as u32); c.m.w32(fp.wrapping_add(104), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(108), v_); }
        { let v_ = ((1i32) as u32); c.m.w32(fp.wrapping_add(112), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(116), v_); }
        { let v_ = ((75i32) as u32); c.m.w32(fp.wrapping_add(120), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(124), v_); }
        F::Dma_AddData(c, fp, ((128u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_GsBeginMask2", &PROBE_N, &[], true); }
    r_
}

pub fn HudGauge_GsEndMask2(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_GsEndMask2", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(128) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 128);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 128);
        { let v_ = ((((268435456i32) | (7i32))) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((((1342177280i32) | (7i32))) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((((32768i32) | (6i32))) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { ((((((((112i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) } else { ((((((((0i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) })) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((77i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        { let v_ = ((68i32) as u32); c.m.w32(fp.wrapping_add(48), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(52), v_); }
        { let v_ = ((67i32) as u32); c.m.w32(fp.wrapping_add(56), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(60), v_); }
        { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { ((((((((112i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) } else { ((((((((0i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) })) as u32); c.m.w32(fp.wrapping_add(64), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(68), v_); }
        { let v_ = ((76i32) as u32); c.m.w32(fp.wrapping_add(72), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(76), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(80), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(84), v_); }
        { let v_ = ((74i32) as u32); c.m.w32(fp.wrapping_add(88), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(92), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(96), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(100), v_); }
        { let v_ = ((75i32) as u32); c.m.w32(fp.wrapping_add(104), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(108), v_); }
        { let v_ = ((196608i32) as u32); c.m.w32(fp.wrapping_add(112), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(116), v_); }
        { let v_ = ((72i32) as u32); c.m.w32(fp.wrapping_add(120), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(124), v_); }
        F::Dma_AddData(c, fp, ((128u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_GsEndMask2", &PROBE_N, &[], true); }
    r_
}

pub fn HudGauge_UpdateHp(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_UpdateHp", &PROBE_N, &[], false); }
    let mut l_bar_1: u32 = 0;
    let mut l_tip_2: u32 = 0;
    let mut l_side_3: i32 = 0;
    let mut l_rem_4: i32 = 0;
    let mut l_bars_5: i32 = 0;
    let mut l_rem2_6: i32 = 0;
    let mut l_bars2_7: i32 = 0;
    let mut l_w_8: i32 = 0;
    let r_ = 'f: {
    l_side_3 = { let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) };
    l_rem_4 = rt::rem_i32({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(40)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }, 10000i32);
    l_bars_5 = rt::div_i32({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(40)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }, 10000i32);
    if (((((((((l_rem_4) == (0i32)) as i32)) != 0) && (((((l_bars_5) != (0i32)) as i32)) != 0)) as i32)) != 0) {
        l_rem_4 = 10000i32;
        { let o_ = l_bars_5; let n_ = (o_).wrapping_sub(1); l_bars_5 = n_; };
    }
    if ((((({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(40)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != ({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(48)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
        l_tip_2 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(28));
        l_bar_1 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(28));
        { let s_ = (l_tip_2).wrapping_add(8); let d_ = (l_bar_1).wrapping_add(8); rt::memcpy(c, d_, s_, 8); };
        l_w_8 = rt::div_i32((l_rem_4).wrapping_mul(160i32), 10000i32);
        if (((((((((((((((((l_w_8) < (3i32)) as i32)) != 0) && (((((l_rem_4) != (0i32)) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(48)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0)) as i32)) != 0) && (((((l_bars_5) == (0i32)) as i32)) != 0)) as i32)) != 0) {
            F::HudSprite_Move(c, l_bar_1, 3i32, 0i32);
        } else {
            F::HudSprite_Move(c, l_bar_1, l_w_8, 0i32);
        }
        F::HudSprite_Show(c, l_tip_2, 1i32);
        F::HudSprite_Show(c, l_bar_1, 1i32);
    } else {
        l_tip_2 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(28));
        l_bar_1 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(28));
        F::HudSprite_Show(c, l_tip_2, 0i32);
        F::HudSprite_Show(c, l_bar_1, 0i32);
    }
    l_rem2_6 = rt::rem_i32({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(48)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }, 10000i32);
    l_bars2_7 = rt::div_i32({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(48)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }, 10000i32);
    if (((((((((l_rem2_6) == (0i32)) as i32)) != 0) && (((((l_bars2_7) != (0i32)) as i32)) != 0)) as i32)) != 0) {
        l_rem2_6 = 10000i32;
        { let o_ = l_bars2_7; let n_ = (o_).wrapping_sub(1); l_bars2_7 = n_; };
    }
    l_tip_2 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(28));
    l_bar_1 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(28));
    { let s_ = (l_tip_2).wrapping_add(8); let d_ = (l_bar_1).wrapping_add(8); rt::memcpy(c, d_, s_, 8); };
    l_w_8 = rt::div_i32((l_rem2_6).wrapping_mul(160i32), 10000i32);
    if (((((l_bars2_7) < (l_bars_5)) as i32)) != 0) {
        l_w_8 = 0i32;
    }
    if (((((((((((((l_w_8) < (3i32)) as i32)) != 0) && (((((l_rem2_6) != (0i32)) as i32)) != 0)) as i32)) != 0) && (((((l_bars_5) == (0i32)) as i32)) != 0)) as i32)) != 0) {
        F::HudSprite_Move(c, l_bar_1, 3i32, 0i32);
    } else {
        F::HudSprite_Move(c, l_bar_1, l_w_8, 0i32);
    }
    F::HudSprite_Show(c, l_tip_2, 1i32);
    F::HudSprite_Show(c, l_bar_1, 1i32);
    if (((((l_bars_5) >= (7i32)) as i32)) != 0) {
        F::HudSprite_SetTex(c, l_tip_2, 8i32, 5i32);
    } else {
        if (((((l_bars_5) >= (2i32)) as i32)) != 0) {
            F::HudSprite_SetTex(c, l_tip_2, 8i32, 1i32);
        } else {
            if (((((l_bars_5) > (0i32)) as i32)) != 0) {
                F::HudSprite_SetTex(c, l_tip_2, 8i32, 4i32);
            } else {
                F::HudSprite_SetTex(c, l_tip_2, 8i32, 3i32);
            }
        }
    }
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(28)); F::HudSprite_Show(c, a0_, 1i32) };
    if (((((l_bars_5) == (l_bars2_7)) as i32)) != 0) {
        { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(28)); F::HudSprite_Show(c, a0_, 0i32) };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(28)); F::HudSprite_Show(c, a0_, 0i32) };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(28)); F::HudSprite_Show(c, a0_, 1i32) };
    } else {
        { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(28)); F::HudSprite_Show(c, a0_, 1i32) };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(28)); F::HudSprite_Show(c, a0_, 1i32) };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(28)); F::HudSprite_Show(c, a0_, 1i32) };
        l_bar_1 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(28));
        { let s_ = (({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(28))).wrapping_add(8); let d_ = (l_bar_1).wrapping_add(8); rt::memcpy(c, d_, s_, 8); };
        if (((((l_bars_5) == ((l_bars2_7).wrapping_add(1i32))) as i32)) != 0) {
            F::HudSprite_Move(c, l_bar_1, rt::div_i32((l_rem2_6).wrapping_mul(160i32), 10000i32), 0i32);
        }
    }
    l_bar_1 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(28));
    if ((((((l_bars_5).wrapping_sub(1i32)) >= (7i32)) as i32)) != 0) {
        F::HudSprite_SetTex(c, l_bar_1, 8i32, 5i32);
    } else {
        if ((((((l_bars_5).wrapping_sub(1i32)) >= (2i32)) as i32)) != 0) {
            F::HudSprite_SetTex(c, l_bar_1, 8i32, 1i32);
        } else {
            if ((((((l_bars_5).wrapping_sub(1i32)) > (0i32)) as i32)) != 0) {
                F::HudSprite_SetTex(c, l_bar_1, 8i32, 4i32);
            } else {
                if ((((((l_bars_5).wrapping_sub(1i32)) >= (0i32)) as i32)) != 0) {
                    F::HudSprite_SetTex(c, l_bar_1, 8i32, 3i32);
                } else {
                    F::HudSprite_Show(c, l_bar_1, 0i32);
                    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(28)); F::HudSprite_Show(c, a0_, 0i32) };
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_UpdateHp", &PROBE_N, &[], true); }
    r_
}
