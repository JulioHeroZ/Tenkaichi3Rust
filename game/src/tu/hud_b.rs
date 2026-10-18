#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn HudGauge_UpdateHpBarCount(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_UpdateHpBarCount", &PROBE_N, &[], false); }
    let mut l_side_1: i32 = 0;
    let mut l_bars_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_spr_4: u32 = 0;
    let r_ = 'f: {
    l_side_1 = { let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) };
    l_bars_2 = rt::div_i32({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(40)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }, 10000i32);
    if (((((((((rt::rem_i32({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(40)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }, 10000i32)) == (0i32)) as i32)) != 0) && (((((l_bars_2) != (0i32)) as i32)) != 0)) as i32)) != 0) {
        { let o_ = l_bars_2; let n_ = (o_).wrapping_sub(1); l_bars_2 = n_; };
    }
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (6i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_spr_4 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(((((9i32).wrapping_add(l_i_3)) as u32)).wrapping_mul(28));
                F::HudSprite_SetColor(c, l_spr_4, 128i32, 128i32, 128i32, 128i32);
                if (((((l_i_3) < (l_bars_2)) as i32)) != 0) {
                    F::HudSprite_Show(c, l_spr_4, 1i32);
                } else {
                    if (((((l_i_3) == (l_bars_2)) as i32)) != 0) {
                        if (((rt::feq({ let a_ = (((c.m.r32(sym::gHudGauge)).wrapping_add(440)).wrapping_add((((l_side_1) as u32)).wrapping_mul(24))).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0x00000000)) as i32)) != 0) {
                            F::HudSprite_Show(c, l_spr_4, 0i32);
                        } else {
                            F::HudSprite_Show(c, l_spr_4, 1i32);
                            { let a4_ = ((((rt::f2i(rt::fmul({ let a_ = (((c.m.r32(sym::gHudGauge)).wrapping_add(440)).wrapping_add((((l_side_1) as u32)).wrapping_mul(24))).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0x43000000)))) as u8)) as i32); F::HudSprite_SetColor(c, l_spr_4, 128i32, 128i32, 128i32, a4_) };
                        }
                    } else {
                        F::HudSprite_Show(c, l_spr_4, 0i32);
                    }
                }
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_UpdateHpBarCount", &PROBE_N, &[], true); }
    r_
}

pub fn HudGauge_UpdateRoot(c: &mut Ctx, mut p_node_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_UpdateRoot", &PROBE_N, &[(p_node_1) as u32], false); }
    let r_ = 'f: {
    if ((((((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0)) as i32)) != 0) {
        { let _ = { let a0_ = (c.m.r32(sym::gHudGauge)).wrapping_add(560); F::Ramp_Step(c, a0_) }; };
    }
    { let a1_ = rt::f2i(rt::fmul({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(560)).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0xc3800000))); let a2_ = rt::f2i(rt::fmul({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(560)).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0xc3600000))); F::HudNode_SetPos(c, p_node_1, a1_, a2_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_UpdateRoot", &PROBE_N, &[], true); }
    r_
}

pub fn HudGauge_UpdatePanel(c: &mut Ctx, mut p_node_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_UpdatePanel", &PROBE_N, &[(p_node_1) as u32], false); }
    let mut l_ramp_2: u32 = 0;
    let r_ = 'f: {
    l_ramp_2 = ((c.m.r32(sym::gHudGauge)).wrapping_add(512)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(24));
    if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
        { let _ = F::Ramp_Step(c, l_ramp_2); };
    }
    { let a1_ = (rt::f2i(rt::fmul(F32(c.m.r32((l_ramp_2).wrapping_add(16))), F32(0xc3800000)))).wrapping_sub(2i32); let a2_ = (rt::f2i(rt::fmul(F32(c.m.r32((l_ramp_2).wrapping_add(16))), F32(0xc3600000)))).wrapping_add(8i32); F::HudNode_SetPos(c, p_node_1, a1_, a2_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_UpdatePanel", &PROBE_N, &[], true); }
    r_
}

pub fn HudGauge_UpdateHpTrail(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_UpdateHpTrail", &PROBE_N, &[], false); }
    let mut l_side_1: i32 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_sh_3: u32 = 0;
    let mut l_grp_4: u32 = 0;
    let r_ = 'f: {
    l_side_1 = { let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) };
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (3i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_sh_3 = (((c.m.r32(sym::gHudGauge)).wrapping_add(12)).wrapping_add((((l_side_1) as u32)).wrapping_mul(12))).wrapping_add((((l_i_2) as u32)).wrapping_mul(4));
                's3: {
                    let sv5: i32 = l_i_2;
                    match sv5 {
                        0i32 => {
                            l_grp_4 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(56));
                            break 's3;
                        }
                        1i32 => {
                            l_grp_4 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(56));
                            break 's3;
                        }
                        2i32 => {
                            l_grp_4 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(56));
                            break 's3;
                        }
                        _ => {
                            break 'c2;
                        }
                    }
                }
                if (((((((c.m.r16(l_sh_3)) as i32)) != (0i32)) as i32)) != 0) {
                    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
                        break 'c2;
                    }
                    { let a_ = l_sh_3; let o_ = c.m.r16(a_); let n_ = (o_).wrapping_sub(1); c.m.w16(a_, n_); };
                    { let v_ = { let a0_ = (((c.m.r16((l_sh_3).wrapping_add(2))) as i32)).wrapping_neg(); let a1_ = ((c.m.r16((l_sh_3).wrapping_add(2))) as i32); F::Rand_IntRange(c, a0_, a1_) }; let a_ = (l_grp_4).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a0_ = (((c.m.r16((l_sh_3).wrapping_add(2))) as i32)).wrapping_neg(); let a1_ = ((c.m.r16((l_sh_3).wrapping_add(2))) as i32); F::Rand_IntRange(c, a0_, a1_) }; let a_ = (l_grp_4).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let v_ = 0i32; let a_ = (l_grp_4).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (l_grp_4).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
        { let _ = { let a0_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(440)).wrapping_add((((l_side_1) as u32)).wrapping_mul(24)); F::Ramp_Step(c, a0_) }; };
        if ((((({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(48)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < ({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(40)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
            if ((((({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(48)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(40)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(20000i32))) as i32)) != 0) {
                { let v_ = (if ((((({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(584)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (200i32)) as i32)) != 0) { 200i32 } else { { let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(584)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) } }); let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(584)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            } else {
                if ((((({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(48)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(40)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(10000i32))) as i32)) != 0) {
                    { let v_ = (if ((((({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(584)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (150i32)) as i32)) != 0) { 150i32 } else { { let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(584)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) } }); let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(584)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                } else {
                    if ((((({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(48)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(40)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(5000i32))) as i32)) != 0) {
                        { let v_ = (if ((((({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(584)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (120i32)) as i32)) != 0) { 120i32 } else { { let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(584)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) } }); let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(584)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                }
            }
            if ((((((((((((({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(40)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) > (10000i32)) as i32)) != 0) && (((((rt::rem_i32({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(40)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }, 10000i32)) > (0i32)) as i32)) != 0)) as i32)) != 0) && (((((rt::rem_i32({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(40)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }, 10000i32)) <= ({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(584)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
                { let a0_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(440)).wrapping_add((((l_side_1) as u32)).wrapping_mul(24)); F::Ramp_Start(c, a0_, F32(0x3e4ccccc), F32(0x3f800000), F32(0x00000000)) };
            }
            { let r_ = { let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(584)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(40)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
            if ((((({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(40)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < ({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(48)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                { let v_ = { let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(48)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(40)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            }
        } else {
            { let v_ = { let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(48)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(40)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = 100i32; let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(584)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_UpdateHpTrail", &PROBE_N, &[], true); }
    r_
}

pub fn HudGauge_UpdateAura(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_UpdateAura", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_n_1: i32 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_spr_3: u32 = 0;
    let mut l_ramp_4: u32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp.wrapping_add(8), 0, 80);
        rt::memset(c, fp.wrapping_add(8).wrapping_add(0), 0, 8);
        { let v_ = ((0i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(0).wrapping_add(0), (v_) as u16); }
        { let v_ = ((64i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(0).wrapping_add(2), (v_) as u16); }
        { let v_ = ((0i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(0).wrapping_add(4), (v_) as u16); }
        { let v_ = ((64i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(0).wrapping_add(6), (v_) as u16); }
        rt::memset(c, fp.wrapping_add(8).wrapping_add(8), 0, 8);
        { let v_ = ((64i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(8).wrapping_add(0), (v_) as u16); }
        { let v_ = ((128i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(8).wrapping_add(2), (v_) as u16); }
        { let v_ = ((0i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(8).wrapping_add(4), (v_) as u16); }
        { let v_ = ((64i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(8).wrapping_add(6), (v_) as u16); }
        rt::memset(c, fp.wrapping_add(8).wrapping_add(16), 0, 8);
        { let v_ = ((128i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(16).wrapping_add(0), (v_) as u16); }
        { let v_ = ((192i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(16).wrapping_add(2), (v_) as u16); }
        { let v_ = ((0i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(16).wrapping_add(4), (v_) as u16); }
        { let v_ = ((64i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(16).wrapping_add(6), (v_) as u16); }
        rt::memset(c, fp.wrapping_add(8).wrapping_add(24), 0, 8);
        { let v_ = ((192i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(24).wrapping_add(0), (v_) as u16); }
        { let v_ = ((256i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(24).wrapping_add(2), (v_) as u16); }
        { let v_ = ((0i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(24).wrapping_add(4), (v_) as u16); }
        { let v_ = ((64i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(24).wrapping_add(6), (v_) as u16); }
        rt::memset(c, fp.wrapping_add(8).wrapping_add(32), 0, 8);
        { let v_ = ((0i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(32).wrapping_add(0), (v_) as u16); }
        { let v_ = ((64i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(32).wrapping_add(2), (v_) as u16); }
        { let v_ = ((64i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(32).wrapping_add(4), (v_) as u16); }
        { let v_ = ((128i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(32).wrapping_add(6), (v_) as u16); }
        rt::memset(c, fp.wrapping_add(8).wrapping_add(40), 0, 8);
        { let v_ = ((64i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(40).wrapping_add(0), (v_) as u16); }
        { let v_ = ((128i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(40).wrapping_add(2), (v_) as u16); }
        { let v_ = ((64i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(40).wrapping_add(4), (v_) as u16); }
        { let v_ = ((128i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(40).wrapping_add(6), (v_) as u16); }
        rt::memset(c, fp.wrapping_add(8).wrapping_add(48), 0, 8);
        { let v_ = ((128i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(48).wrapping_add(0), (v_) as u16); }
        { let v_ = ((192i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(48).wrapping_add(2), (v_) as u16); }
        { let v_ = ((64i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(48).wrapping_add(4), (v_) as u16); }
        { let v_ = ((128i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(48).wrapping_add(6), (v_) as u16); }
        rt::memset(c, fp.wrapping_add(8).wrapping_add(56), 0, 8);
        { let v_ = ((192i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(56).wrapping_add(0), (v_) as u16); }
        { let v_ = ((256i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(56).wrapping_add(2), (v_) as u16); }
        { let v_ = ((64i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(56).wrapping_add(4), (v_) as u16); }
        { let v_ = ((128i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(56).wrapping_add(6), (v_) as u16); }
        rt::memset(c, fp.wrapping_add(8).wrapping_add(64), 0, 8);
        { let v_ = ((0i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(64).wrapping_add(0), (v_) as u16); }
        { let v_ = ((64i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(64).wrapping_add(2), (v_) as u16); }
        { let v_ = ((128i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(64).wrapping_add(4), (v_) as u16); }
        { let v_ = ((192i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(64).wrapping_add(6), (v_) as u16); }
        rt::memset(c, fp.wrapping_add(8).wrapping_add(72), 0, 8);
        { let v_ = ((64i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(72).wrapping_add(0), (v_) as u16); }
        { let v_ = ((128i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(72).wrapping_add(2), (v_) as u16); }
        { let v_ = ((128i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(72).wrapping_add(4), (v_) as u16); }
        { let v_ = ((192i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(72).wrapping_add(6), (v_) as u16); }
        if (((((({ let a0_ = { let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }; F::BtlSide_IsPoweredUp(c, a0_) }) != 0) && ((((!(({ let a0_ = { let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }; F::BtlCtrl_IsActiveDead(c, a0_) }) != 0)) as i32)) != 0)) as i32)) != 0) {
            { let r_ = 16i32; let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(56)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
            if ((((({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(56)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) > (128i32)) as i32)) != 0) {
                { let v_ = 128i32; let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(56)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            }
        } else {
            { let r_ = 16i32; let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(56)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
            if ((((({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(56)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(56)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            }
        }
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (10i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(((((53i32).wrapping_add(l_i_2)) as u32)).wrapping_mul(28));
                    l_n_1 = l_i_2;
                    if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
                        'l3: loop {
                            'c4: {
                                l_n_1 = F::Rand_IntRange(c, 0i32, 9i32);
                            }
                            if !((((((((((((((((((((c.m.r16(((fp.wrapping_add(8)).wrapping_add((((l_n_1) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(2))) as i16)) as i32)) == ((((c.m.r16(((l_spr_3).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(2))) as i16)) as i32))) as i32)) != 0) && ((((((((c.m.r16(((fp.wrapping_add(8)).wrapping_add((((l_n_1) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(2))) as i16)) as i32)) == ((((c.m.r16(((l_spr_3).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(2))) as i16)) as i32))) as i32)) != 0)) as i32)) != 0) && ((((((((c.m.r16(((fp.wrapping_add(8)).wrapping_add((((l_n_1) as u32)).wrapping_mul(8))).wrapping_add((((2i32) as u32)).wrapping_mul(2))) as i16)) as i32)) == ((((c.m.r16(((l_spr_3).wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(2))) as i16)) as i32))) as i32)) != 0)) as i32)) != 0) && ((((((((c.m.r16(((fp.wrapping_add(8)).wrapping_add((((l_n_1) as u32)).wrapping_mul(8))).wrapping_add((((3i32) as u32)).wrapping_mul(2))) as i16)) as i32)) == ((((c.m.r16(((l_spr_3).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(2))) as i16)) as i32))) as i32)) != 0)) as i32)) != 0) { break 'l3; }
                        }
                    }
                    { let v_ = (c.m.r16(((fp.wrapping_add(8)).wrapping_add((((l_n_1) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(2))) as i16); let a_ = ((l_spr_3).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.w16(a_, (v_) as u16); };
                    { let v_ = (c.m.r16(((fp.wrapping_add(8)).wrapping_add((((l_n_1) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(2))) as i16); let a_ = ((l_spr_3).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.w16(a_, (v_) as u16); };
                    { let v_ = (c.m.r16(((fp.wrapping_add(8)).wrapping_add((((l_n_1) as u32)).wrapping_mul(8))).wrapping_add((((2i32) as u32)).wrapping_mul(2))) as i16); let a_ = ((l_spr_3).wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(2)); c.m.w16(a_, (v_) as u16); };
                    { let v_ = (c.m.r16(((fp.wrapping_add(8)).wrapping_add((((l_n_1) as u32)).wrapping_mul(8))).wrapping_add((((3i32) as u32)).wrapping_mul(2))) as i16); let a_ = ((l_spr_3).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(2)); c.m.w16(a_, (v_) as u16); };
                    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
                        { let v_ = 0i32; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        { let v_ = (8i32).wrapping_neg(); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    } else {
                        { let v_ = F::Rand_IntRange(c, (5i32).wrapping_neg(), 5i32); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        { let v_ = (F::Rand_IntRange(c, (5i32).wrapping_neg(), 5i32)).wrapping_sub(8i32); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    F::HudSprite_SetRect(c, l_spr_3, 0i32, 64i32, 0i32, 64i32);
                    { let a1_ = ((rt::rem_i32(l_i_2, 5i32)).wrapping_mul(46i32)).wrapping_add((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)); let a2_ = ((rt::div_i32(l_i_2, 5i32)).wrapping_mul(20i32)).wrapping_add((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)); F::HudSprite_Move(c, l_spr_3, a1_, a2_) };
                    { let a4_ = (((({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(56)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u8)) as i32); F::HudSprite_SetColor(c, l_spr_3, 128i32, 128i32, 128i32, a4_) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((62i32) as u32)).wrapping_mul(28));
        { let a4_ = ((((rt::f2i(rt::fmul(rt::fmul(rt::i2f({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(56)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }), F32(0x3c000000)), F32(0x42300000)))) as u8)) as i32); F::HudSprite_InitPlain(c, l_spr_3, ((((0i32) as i64 as u64)) as u8), ((((0i32) as i64 as u64)) as u8), ((((255i32) as i64 as u64)) as u8), ((((a4_) as i64 as u64)) as u8)) };
        F::HudSprite_SetRect(c, l_spr_3, 0i32, 384i32, 0i32, 108i32);
        l_ramp_4 = ((c.m.r32(sym::gHudGauge)).wrapping_add(64)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(24));
        if ((((((((({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(48)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) <= (10000i32)) as i32)) != 0) && ((((!(({ let a0_ = { let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }; F::BtlCtrl_IsActiveDead(c, a0_) }) != 0)) as i32)) != 0)) as i32)) != 0) {
            if ((((((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) && ((F::Ramp_Step(c, l_ramp_4)) != 0)) as i32)) != 0) {
                if (((rt::feq(F32(c.m.r32((l_ramp_4).wrapping_add(16))), F32(0x00000000)) as i32)) != 0) {
                    F::Ramp_Start(c, l_ramp_4, F32(0x3f000000), F32(0x00000000), F32(0x3f800000));
                } else {
                    F::Ramp_Start(c, l_ramp_4, F32(0x3f000000), F32(0x3f800000), F32(0x00000000));
                }
            }
        } else {
            if ((((((((((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) && ((F::Ramp_Step(c, l_ramp_4)) != 0)) as i32)) != 0) && (((rt::feq(F32(c.m.r32((l_ramp_4).wrapping_add(16))), F32(0x3f800000)) as i32)) != 0)) as i32)) != 0) {
                F::Ramp_Start(c, l_ramp_4, F32(0x3f000000), F32(0x3f800000), F32(0x00000000));
            }
        }
        l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((63i32) as u32)).wrapping_mul(28));
        { let a4_ = ((((rt::f2i(rt::fmul(F32(c.m.r32((l_ramp_4).wrapping_add(16))), F32(0x42300000)))) as u8)) as i32); F::HudSprite_InitPlain(c, l_spr_3, ((((255i32) as i64 as u64)) as u8), ((((0i32) as i64 as u64)) as u8), ((((0i32) as i64 as u64)) as u8), ((((a4_) as i64 as u64)) as u8)) };
        F::HudSprite_SetRect(c, l_spr_3, 0i32, 384i32, 0i32, 108i32);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_UpdateAura", &PROBE_N, &[], true); }
    r_
}

pub fn HudGauge_UpdateKi(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_UpdateKi", &PROBE_N, &[], false); }
    let mut l_shown_1: i32 = 0;
    let mut l_ki_2: i32 = 0;
    let mut l_extra_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_lo_5: i32 = 0;
    let mut l_n_6: i32 = 0;
    let mut l_spr_7: u32 = 0;
    let r_ = 'f: {
    l_shown_1 = { let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(128)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
    l_ki_2 = { let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(120)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
    l_extra_3 = { let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(304)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
    if (((((l_ki_2) < ((l_shown_1).wrapping_sub(40000i32))) as i32)) != 0) {
        { let v_ = (if ((((({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(592)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (800i32)) as i32)) != 0) { 800i32 } else { { let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(592)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) } }); let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(592)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    } else {
        if (((((l_ki_2) < ((l_shown_1).wrapping_sub(20000i32))) as i32)) != 0) {
            { let v_ = (if ((((({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(592)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (606i32)) as i32)) != 0) { 606i32 } else { { let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(592)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) } }); let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(592)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = (if ((((({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(592)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (400i32)) as i32)) != 0) { 400i32 } else { { let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(592)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) } }); let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(592)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
    }
    if ((((((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) && (((((l_ki_2) < ({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(128)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
        { let r_ = { let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(592)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(128)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
    }
    if (((((l_ki_2) >= ({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(128)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
        { let v_ = l_ki_2; let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(128)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 400i32; let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(592)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    }
    l_shown_1 = { let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(128)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (5i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_spr_7 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(((((20i32).wrapping_add(l_i_4)) as u32)).wrapping_mul(28));
                if (((((l_shown_1) >= (((l_i_4).wrapping_mul(20000i32)).wrapping_add(20000i32))) as i32)) != 0) {
                    F::HudSprite_Show(c, l_spr_7, 1i32);
                    F::HudSprite_SetRect(c, l_spr_7, 0i32, 14i32, 0i32, 17i32);
                    F::HudSprite_SetUv(c, l_spr_7, 16i32, 30i32, 16i32, 33i32);
                } else {
                    if ((((((l_i_4).wrapping_mul(20000i32)) >= (l_shown_1)) as i32)) != 0) {
                        F::HudSprite_Show(c, l_spr_7, 0i32);
                    } else {
                        l_n_6 = rt::div_i32((rt::rem_i32(l_shown_1, 20000i32)).wrapping_mul(17i32), 20000i32);
                        F::HudSprite_Show(c, l_spr_7, 1i32);
                        F::HudSprite_SetRect(c, l_spr_7, 0i32, 14i32, (17i32).wrapping_sub(l_n_6), 17i32);
                        F::HudSprite_SetUv(c, l_spr_7, 16i32, 30i32, (33i32).wrapping_sub(l_n_6), 33i32);
                    }
                }
                F::HudSprite_Move(c, l_spr_7, (l_i_4).wrapping_mul(8i32), 0i32);
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    {
        l_i_4 = 0i32;
        'l3: loop {
            if !(((((l_i_4) < (5i32)) as i32)) != 0) { break 'l3; }
            'c4: {
                l_spr_7 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(((((25i32).wrapping_add(l_i_4)) as u32)).wrapping_mul(28));
                if (((((l_ki_2) >= (((l_i_4).wrapping_mul(20000i32)).wrapping_add(20000i32))) as i32)) != 0) {
                    F::HudSprite_Show(c, l_spr_7, 1i32);
                    F::HudSprite_SetRect(c, l_spr_7, 0i32, 14i32, 0i32, 17i32);
                    F::HudSprite_SetUv(c, l_spr_7, 16i32, 30i32, 16i32, 33i32);
                } else {
                    if ((((((l_i_4).wrapping_mul(20000i32)) >= (l_ki_2)) as i32)) != 0) {
                        F::HudSprite_Show(c, l_spr_7, 0i32);
                    } else {
                        l_n_6 = (l_ki_2).wrapping_sub((l_i_4).wrapping_mul(20000i32));
                        l_n_6 = rt::div_i32((l_n_6).wrapping_mul(17i32), 20000i32);
                        F::HudSprite_Show(c, l_spr_7, 1i32);
                        F::HudSprite_SetRect(c, l_spr_7, 0i32, 14i32, (17i32).wrapping_sub(l_n_6), 17i32);
                        F::HudSprite_SetUv(c, l_spr_7, 16i32, 30i32, (33i32).wrapping_sub(l_n_6), 33i32);
                    }
                }
                F::HudSprite_Move(c, l_spr_7, (l_i_4).wrapping_mul(8i32), 0i32);
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    if (((((({ let a0_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(600)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(24)); F::Ramp_Step(c, a0_) }) != 0) && (((((l_extra_3) != (0i32)) as i32)) != 0)) as i32)) != 0) {
        if (((rt::feq({ let a_ = (((c.m.r32(sym::gHudGauge)).wrapping_add(600)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(24))).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0x00000000)) as i32)) != 0) {
            { let a0_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(600)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(24)); F::Ramp_Start(c, a0_, F32(0x3e999999), F32(0x00000000), F32(0x3f800000)) };
        } else {
            { let a0_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(600)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(24)); F::Ramp_Start(c, a0_, F32(0x3e999999), F32(0x3f800000), F32(0x00000000)) };
        }
    }
    l_n_6 = rt::div_i32(l_extra_3, 20000i32);
    {
        l_i_4 = 0i32;
        'l5: loop {
            if !(((((l_i_4) < (5i32)) as i32)) != 0) { break 'l5; }
            'c6: {
                l_spr_7 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(((((15i32).wrapping_add(l_i_4)) as u32)).wrapping_mul(28));
                { let a1_ = ((((rt::f2i(rt::fmul({ let a_ = (((c.m.r32(sym::gHudGauge)).wrapping_add(600)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(24))).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0x43400000)))) as u8)) as i32); F::HudSprite_SetColor(c, l_spr_7, a1_, 0i32, 0i32, 64i32) };
                if (((((l_extra_3) >= (((l_i_4).wrapping_mul(20000i32)).wrapping_add(20000i32))) as i32)) != 0) {
                    F::HudSprite_Show(c, l_spr_7, 1i32);
                    F::HudSprite_SetRect(c, l_spr_7, 0i32, 14i32, 0i32, 17i32);
                    F::HudSprite_SetUv(c, l_spr_7, 16i32, 30i32, 16i32, 33i32);
                } else {
                    if ((((((l_i_4).wrapping_mul(20000i32)) >= (l_extra_3)) as i32)) != 0) {
                        F::HudSprite_Show(c, l_spr_7, 0i32);
                    } else {
                        l_n_6 = (l_extra_3).wrapping_sub((l_i_4).wrapping_mul(20000i32));
                        l_n_6 = rt::div_i32((l_n_6).wrapping_mul(17i32), 20000i32);
                        F::HudSprite_Show(c, l_spr_7, 1i32);
                        F::HudSprite_SetRect(c, l_spr_7, 0i32, 14i32, (17i32).wrapping_sub(l_n_6), 17i32);
                        F::HudSprite_SetUv(c, l_spr_7, 16i32, 30i32, (33i32).wrapping_sub(l_n_6), 33i32);
                    }
                }
                F::HudSprite_Move(c, l_spr_7, (l_i_4).wrapping_mul(8i32), 0i32);
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_UpdateKi", &PROBE_N, &[], true); }
    r_
}

pub fn HudGauge_UpdateMaxPower(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_UpdateMaxPower", &PROBE_N, &[], false); }
    let mut l_p_1: u32 = 0;
    let mut l_v_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_n_4: i32 = 0;
    let mut l_spr_5: u32 = 0;
    let r_ = 'f: {
    l_p_1 = (c.m.r32(sym::gHudGauge)).wrapping_add(232);
    l_v_2 = { let a_ = (l_p_1).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (5i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_spr_5 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(((((30i32).wrapping_add(l_i_3)) as u32)).wrapping_mul(28));
                if (((((l_v_2) >= (((l_i_3).wrapping_mul(6000i32)).wrapping_add(6000i32))) as i32)) != 0) {
                    F::HudSprite_Show(c, l_spr_5, 1i32);
                    F::HudSprite_SetRect(c, l_spr_5, 0i32, 14i32, 0i32, 17i32);
                    F::HudSprite_SetUv(c, l_spr_5, 16i32, 30i32, 16i32, 33i32);
                } else {
                    if ((((((l_i_3).wrapping_mul(6000i32)) >= (l_v_2)) as i32)) != 0) {
                        F::HudSprite_Show(c, l_spr_5, 0i32);
                    } else {
                        l_n_4 = rt::div_i32(((l_v_2).wrapping_sub((l_i_3).wrapping_mul(6000i32))).wrapping_mul(17i32), 6000i32);
                        F::HudSprite_Show(c, l_spr_5, 1i32);
                        F::HudSprite_SetRect(c, l_spr_5, 0i32, 14i32, (17i32).wrapping_sub(l_n_4), 17i32);
                        F::HudSprite_SetUv(c, l_spr_5, 16i32, 30i32, (33i32).wrapping_sub(l_n_4), 33i32);
                    }
                }
                F::HudSprite_Move(c, l_spr_5, (l_i_3).wrapping_mul(8i32), 0i32);
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_UpdateMaxPower", &PROBE_N, &[], true); }
    r_
}

pub fn HudGauge_UpdateKiLamps(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_UpdateKiLamps", &PROBE_N, &[], false); }
    let mut l_a_1: u32 = 0;
    let mut l_b_2: u32 = 0;
    let mut l_ki_3: i32 = 0;
    let mut l_bars_4: i32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_r_6: i32 = 0;
    let mut l_alpha_7: u8 = 0;
    let mut l_spr_8: u32 = 0;
    let r_ = 'f: {
    l_a_1 = ((c.m.r32(sym::gHudGauge)).wrapping_add(136)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(24));
    l_b_2 = ((c.m.r32(sym::gHudGauge)).wrapping_add(184)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(24));
    l_ki_3 = { let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(120)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
    l_bars_4 = rt::div_i32(l_ki_3, 20000i32);
    if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
        l_r_6 = F::Ramp_Step(c, l_a_1);
    } else {
        l_r_6 = (rt::feq(F32(c.m.r32((l_a_1).wrapping_add(16))), F32(c.m.r32((l_a_1).wrapping_add(20)))) as i32);
    }
    if ((l_r_6) != 0) {
        if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
            l_r_6 = F::Ramp_Step(c, l_b_2);
        } else {
            l_r_6 = 0i32;
        }
        if ((l_r_6) != 0) {
            if (((rt::feq(F32(c.m.r32((l_b_2).wrapping_add(16))), F32(0x00000000)) as i32)) != 0) {
                F::Ramp_Start(c, l_b_2, F32(0x3f800000), F32(0x00000000), F32(0x3f800000));
            } else {
                F::Ramp_Start(c, l_b_2, F32(0x3f800000), F32(0x3f800000), F32(0x00000000));
            }
        }
        if (((((l_ki_3) > (99999i32)) as i32)) != 0) {
            l_alpha_7 = ((rt::f2i(rt::fmul(F32(c.m.r32((l_b_2).wrapping_add(16))), F32(0x43000000)))) as u8);
            {
                l_i_5 = 0i32;
                'l1: loop {
                    if !(((((l_i_5) < (5i32)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        l_spr_8 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(((((65i32).wrapping_add(l_i_5)) as u32)).wrapping_mul(28));
                        F::HudSprite_Show(c, l_spr_8, 1i32);
                        F::HudSprite_SetColor(c, l_spr_8, 128i32, 128i32, 128i32, ((l_alpha_7) as i32));
                    }
                    { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
                }
            }
            break 'f;
        }
    }
    l_alpha_7 = ((rt::f2i(rt::fmul(F32(c.m.r32((l_a_1).wrapping_add(16))), F32(0x43000000)))) as u8);
    {
        l_i_5 = 0i32;
        'l3: loop {
            if !(((((l_i_5) < (5i32)) as i32)) != 0) { break 'l3; }
            'c4: {
                l_spr_8 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(((((65i32).wrapping_add(l_i_5)) as u32)).wrapping_mul(28));
                if (((((l_bars_4) == ((l_i_5).wrapping_add(1i32))) as i32)) != 0) {
                    F::HudSprite_Show(c, l_spr_8, 1i32);
                    F::HudSprite_SetColor(c, l_spr_8, 128i32, 128i32, 128i32, ((l_alpha_7) as i32));
                } else {
                    F::HudSprite_Show(c, l_spr_8, 0i32);
                }
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_UpdateKiLamps", &PROBE_N, &[], true); }
    r_
}

pub fn HudGauge_UpdateMaxPowerLamps(c: &mut Ctx, mut p_node_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_UpdateMaxPowerLamps", &PROBE_N, &[(p_node_1) as u32], false); }
    let mut l_v_2: i32 = 0;
    let mut l_b_3: u32 = 0;
    let mut l_bars_4: i32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_alpha_6: u8 = 0;
    let mut l_spr_7: u32 = 0;
    let r_ = 'f: {
    l_v_2 = { let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(232)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
    l_b_3 = ((c.m.r32(sym::gHudGauge)).wrapping_add(184)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(24));
    F::HudNode_Show(c, p_node_1, 1i32);
    l_bars_4 = rt::div_i32(l_v_2, 6000i32);
    l_alpha_6 = ((rt::f2i(rt::fmul(rt::fmul(F32(c.m.r32((l_b_3).wrapping_add(16))), F32(0x3f4ccccc)), F32(0x43000000)))) as u8);
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((l_i_5) < (5i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_spr_7 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(((((70i32).wrapping_add(l_i_5)) as u32)).wrapping_mul(28));
                if (((((l_bars_4) >= ((l_i_5).wrapping_add(1i32))) as i32)) != 0) {
                    F::HudSprite_Show(c, l_spr_7, 1i32);
                    F::HudSprite_SetColor(c, l_spr_7, 128i32, 128i32, 128i32, ((l_alpha_6) as i32));
                    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(((((65i32).wrapping_add(l_i_5)) as u32)).wrapping_mul(28)); F::HudSprite_Show(c, a0_, 0i32) };
                } else {
                    F::HudSprite_Show(c, l_spr_7, 0i32);
                }
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_UpdateMaxPowerLamps", &PROBE_N, &[], true); }
    r_
}

pub fn HudGauge_UpdateBlast(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_UpdateBlast", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_ramp_1: u32 = 0;
    let mut l_blast_2: i32 = 0;
    let mut l_max_3: i32 = 0;
    let mut l_spr_4: u32 = 0;
    let mut l_n_5: u32 = 0;
    let mut l_r_6: i32 = 0;
    let r_ = 'f: {
        l_ramp_1 = ((c.m.r32(sym::gHudGauge)).wrapping_add(240)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(24));
        l_blast_2 = { let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(112)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
        rt::memset(c, fp, 0, 64);
        rt::memset(c, fp.wrapping_add(0), 0, 8);
        { let v_ = ((0i32) as u16); c.m.w16(fp.wrapping_add(0).wrapping_add(0), v_); }
        { let v_ = ((32i32) as u16); c.m.w16(fp.wrapping_add(0).wrapping_add(2), v_); }
        { let v_ = ((0i32) as u16); c.m.w16(fp.wrapping_add(0).wrapping_add(4), v_); }
        { let v_ = ((32i32) as u16); c.m.w16(fp.wrapping_add(0).wrapping_add(6), v_); }
        rt::memset(c, fp.wrapping_add(8), 0, 8);
        { let v_ = ((32i32) as u16); c.m.w16(fp.wrapping_add(8).wrapping_add(0), v_); }
        { let v_ = ((64i32) as u16); c.m.w16(fp.wrapping_add(8).wrapping_add(2), v_); }
        { let v_ = ((0i32) as u16); c.m.w16(fp.wrapping_add(8).wrapping_add(4), v_); }
        { let v_ = ((32i32) as u16); c.m.w16(fp.wrapping_add(8).wrapping_add(6), v_); }
        rt::memset(c, fp.wrapping_add(16), 0, 8);
        { let v_ = ((64i32) as u16); c.m.w16(fp.wrapping_add(16).wrapping_add(0), v_); }
        { let v_ = ((96i32) as u16); c.m.w16(fp.wrapping_add(16).wrapping_add(2), v_); }
        { let v_ = ((0i32) as u16); c.m.w16(fp.wrapping_add(16).wrapping_add(4), v_); }
        { let v_ = ((32i32) as u16); c.m.w16(fp.wrapping_add(16).wrapping_add(6), v_); }
        rt::memset(c, fp.wrapping_add(24), 0, 8);
        { let v_ = ((96i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(0), v_); }
        { let v_ = ((128i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(2), v_); }
        { let v_ = ((0i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(4), v_); }
        { let v_ = ((32i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(6), v_); }
        rt::memset(c, fp.wrapping_add(32), 0, 8);
        { let v_ = ((0i32) as u16); c.m.w16(fp.wrapping_add(32).wrapping_add(0), v_); }
        { let v_ = ((32i32) as u16); c.m.w16(fp.wrapping_add(32).wrapping_add(2), v_); }
        { let v_ = ((32i32) as u16); c.m.w16(fp.wrapping_add(32).wrapping_add(4), v_); }
        { let v_ = ((64i32) as u16); c.m.w16(fp.wrapping_add(32).wrapping_add(6), v_); }
        rt::memset(c, fp.wrapping_add(40), 0, 8);
        { let v_ = ((32i32) as u16); c.m.w16(fp.wrapping_add(40).wrapping_add(0), v_); }
        { let v_ = ((64i32) as u16); c.m.w16(fp.wrapping_add(40).wrapping_add(2), v_); }
        { let v_ = ((32i32) as u16); c.m.w16(fp.wrapping_add(40).wrapping_add(4), v_); }
        { let v_ = ((64i32) as u16); c.m.w16(fp.wrapping_add(40).wrapping_add(6), v_); }
        rt::memset(c, fp.wrapping_add(48), 0, 8);
        { let v_ = ((64i32) as u16); c.m.w16(fp.wrapping_add(48).wrapping_add(0), v_); }
        { let v_ = ((96i32) as u16); c.m.w16(fp.wrapping_add(48).wrapping_add(2), v_); }
        { let v_ = ((32i32) as u16); c.m.w16(fp.wrapping_add(48).wrapping_add(4), v_); }
        { let v_ = ((64i32) as u16); c.m.w16(fp.wrapping_add(48).wrapping_add(6), v_); }
        rt::memset(c, fp.wrapping_add(56), 0, 8);
        { let v_ = ((96i32) as u16); c.m.w16(fp.wrapping_add(56).wrapping_add(0), v_); }
        { let v_ = ((128i32) as u16); c.m.w16(fp.wrapping_add(56).wrapping_add(2), v_); }
        { let v_ = ((32i32) as u16); c.m.w16(fp.wrapping_add(56).wrapping_add(4), v_); }
        { let v_ = ((64i32) as u16); c.m.w16(fp.wrapping_add(56).wrapping_add(6), v_); }
        l_max_3 = { let a0_ = { let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }; F::BtlSide_GetBlastMax(c, a0_) };
        l_n_5 = ((rt::div_i32(l_blast_2, 100000i32)) as u32);
        l_spr_4 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((35i32) as u32)).wrapping_mul(28));
        if (((((l_n_5) >= (((8i32) as u32))) as i32)) != 0) {
            l_n_5 = ((7i32) as u32);
        }
        F::HudSprite_SetRect(c, l_spr_4, (32i32).wrapping_neg(), 0i32, 0i32, 32i32);
        { let a1_ = ((c.m.r16(((fp).wrapping_add((l_n_5).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(2)))) as i32); let a2_ = ((c.m.r16(((fp).wrapping_add((l_n_5).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(2)))) as i32); let a3_ = ((c.m.r16(((fp).wrapping_add((l_n_5).wrapping_mul(8))).wrapping_add((((2i32) as u32)).wrapping_mul(2)))) as i32); let a4_ = ((c.m.r16(((fp).wrapping_add((l_n_5).wrapping_mul(8))).wrapping_add((((3i32) as u32)).wrapping_mul(2)))) as i32); F::HudSprite_SetUv(c, l_spr_4, a1_, a2_, a3_, a4_) };
        F::HudSprite_Move(c, l_spr_4, 10i32, (16i32).wrapping_neg());
        if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
            l_r_6 = F::Ramp_Step(c, l_ramp_1);
        } else {
            l_r_6 = 0i32;
        }
        if ((l_r_6) != 0) {
            if (((((l_blast_2) >= (l_max_3)) as i32)) != 0) {
                if (((rt::feq(F32(c.m.r32((l_ramp_1).wrapping_add(16))), F32(0x00000000)) as i32)) != 0) {
                    F::Ramp_Start(c, l_ramp_1, F32(0x3f800000), F32(0x00000000), F32(0x3f800000));
                } else {
                    F::Ramp_Start(c, l_ramp_1, F32(0x3f800000), F32(0x3f800000), F32(0x00000000));
                }
            } else {
                if (((rt::feq(F32(c.m.r32((l_ramp_1).wrapping_add(16))), F32(0x3f800000)) as i32)) != 0) {
                    F::Ramp_Start(c, l_ramp_1, F32(0x3f800000), F32(0x3f800000), F32(0x00000000));
                }
            }
        }
        l_spr_4 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((75i32) as u32)).wrapping_mul(28));
        F::HudSprite_SetRect(c, l_spr_4, (32i32).wrapping_neg(), 0i32, 0i32, 32i32);
        { let a1_ = ((c.m.r16(((fp).wrapping_add((l_n_5).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(2)))) as i32); let a2_ = ((c.m.r16(((fp).wrapping_add((l_n_5).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(2)))) as i32); let a3_ = ((c.m.r16(((fp).wrapping_add((l_n_5).wrapping_mul(8))).wrapping_add((((2i32) as u32)).wrapping_mul(2)))) as i32); let a4_ = ((c.m.r16(((fp).wrapping_add((l_n_5).wrapping_mul(8))).wrapping_add((((3i32) as u32)).wrapping_mul(2)))) as i32); F::HudSprite_SetUv(c, l_spr_4, a1_, a2_, a3_, a4_) };
        F::HudSprite_Move(c, l_spr_4, 10i32, (16i32).wrapping_neg());
        if (((rt::fgt(F32(c.m.r32((l_ramp_1).wrapping_add(16))), F32(0x00000000)) as i32)) != 0) {
            F::HudSprite_Show(c, l_spr_4, 1i32);
        } else {
            F::HudSprite_Show(c, l_spr_4, 0i32);
        }
        { let a4_ = ((((rt::f2i(rt::fmul(F32(c.m.r32((l_ramp_1).wrapping_add(16))), F32(0x43000000)))) as u8)) as i32); F::HudSprite_SetColor(c, l_spr_4, 128i32, 128i32, 128i32, a4_) };
        l_n_5 = ((rt::rem_i32(l_blast_2, 100000i32)) as u32);
        l_n_5 = rt::div_u32((l_n_5).wrapping_mul(((25i32) as u32)), ((100000i32) as u32));
        l_spr_4 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((37i32) as u32)).wrapping_mul(28));
        F::HudSprite_SetRect(c, l_spr_4, ((l_n_5) as i32), (((l_n_5).wrapping_add(((26i32) as u32))) as i32), 0i32, 6i32);
        F::HudSprite_Move(c, l_spr_4, 5i32, 3i32);
        if (((((l_blast_2) >= (l_max_3)) as i32)) != 0) {
            { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((36i32) as u32)).wrapping_mul(28)); F::HudSprite_Show(c, a0_, 0i32) };
            { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((38i32) as u32)).wrapping_mul(28)); F::HudSprite_Show(c, a0_, 0i32) };
            { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((37i32) as u32)).wrapping_mul(28)); F::HudSprite_Show(c, a0_, 0i32) };
        } else {
            { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((36i32) as u32)).wrapping_mul(28)); F::HudSprite_Show(c, a0_, 1i32) };
            { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((38i32) as u32)).wrapping_mul(28)); F::HudSprite_Show(c, a0_, 1i32) };
            { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((37i32) as u32)).wrapping_mul(28)); F::HudSprite_Show(c, a0_, 1i32) };
        }
        l_spr_4 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((48i32) as u32)).wrapping_mul(28));
        if (((((l_blast_2) >= (l_max_3)) as i32)) != 0) {
            F::HudSprite_Show(c, l_spr_4, 0i32);
        } else {
            F::HudSprite_Show(c, l_spr_4, 1i32);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_UpdateBlast", &PROBE_N, &[], true); }
    r_
}

pub fn HudGauge_UpdateStatIcons(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_UpdateStatIcons", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_side_1: i32 = 0;
    let mut l_mask_2: u32 = 0;
    let mut l_pos_3: u32 = 0;
    let mut l_prev_4: u32 = 0;
    let mut l_state_5: u32 = 0;
    let mut l_to_6: u32 = 0;
    let mut l_ramp_7: u32 = 0;
    let mut l_icon_8: u32 = 0;
    let mut l_frames_9: u32 = 0;
    let mut l_shades_10: u32 = 0;
    let mut l_from_11: u32 = 0;
    let mut l_removed_12: u32 = 0;
    let mut l_added_13: u32 = 0;
    let mut l_kept_14: u32 = 0;
    let mut l_r_15: i32 = 0;
    let mut l_i_16: i32 = 0;
    let mut l_x_17: i32 = 0;
    let mut l_c_18: u32 = 0;
    let mut l_t_19: F32 = F32(0);
    let r_ = 'f: {
        l_side_1 = { let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) };
        l_mask_2 = { let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(288)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) };
        l_pos_3 = { let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(296)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) };
        l_prev_4 = { let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(312)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) };
        l_state_5 = ((c.m.r32(sym::gHudGauge)).wrapping_add(432)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4));
        l_to_6 = ((c.m.r32(sym::gHudGauge)).wrapping_add(320)).wrapping_add((((l_side_1) as u32)).wrapping_mul(16));
        l_ramp_7 = ((c.m.r32(sym::gHudGauge)).wrapping_add(384)).wrapping_add((((l_side_1) as u32)).wrapping_mul(24));
        l_icon_8 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((43i32) as u32)).wrapping_mul(28));
        l_frames_9 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((39i32) as u32)).wrapping_mul(28));
        l_shades_10 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((49i32) as u32)).wrapping_mul(28));
        rt::memset(c, fp, 0, 8);
        rt::memset(c, fp.wrapping_add(0), 0, 4);
        { let v_ = ((128i32) as u8); c.m.w8(fp.wrapping_add(0).wrapping_add(0), v_); }
        { let v_ = ((128i32) as u8); c.m.w8(fp.wrapping_add(0).wrapping_add(1), v_); }
        { let v_ = ((128i32) as u8); c.m.w8(fp.wrapping_add(0).wrapping_add(2), v_); }
        { let v_ = ((128i32) as u8); c.m.w8(fp.wrapping_add(0).wrapping_add(3), v_); }
        rt::memset(c, fp.wrapping_add(4), 0, 4);
        { let v_ = ((64i32) as u8); c.m.w8(fp.wrapping_add(4).wrapping_add(0), v_); }
        { let v_ = ((64i32) as u8); c.m.w8(fp.wrapping_add(4).wrapping_add(1), v_); }
        { let v_ = ((64i32) as u8); c.m.w8(fp.wrapping_add(4).wrapping_add(2), v_); }
        { let v_ = ((128i32) as u8); c.m.w8(fp.wrapping_add(4).wrapping_add(3), v_); }
        l_from_11 = ((c.m.r32(sym::gHudGauge)).wrapping_add(352)).wrapping_add((((l_side_1) as u32)).wrapping_mul(16));
        l_removed_12 = ((((l_mask_2) ^ (l_prev_4))) & (l_prev_4));
        l_added_13 = ((((l_mask_2) ^ (l_prev_4))) & (l_mask_2));
        l_kept_14 = ((l_mask_2) & (l_prev_4));
        if ((((((c.m.r32(l_state_5) as i32)) == (0i32)) as i32)) != 0) {
            if (((((l_removed_12) != (((0i32) as u32))) as i32)) != 0) {
                { let v_ = 0i32; let a_ = l_state_5; c.m.w32(a_, (v_) as u32); };
            } else {
                if (((((l_added_13) != (((0i32) as u32))) as i32)) != 0) {
                    { let v_ = 2i32; let a_ = l_state_5; c.m.w32(a_, (v_) as u32); };
                } else {
                    { let v_ = (1i32).wrapping_neg(); let a_ = l_state_5; c.m.w32(a_, (v_) as u32); };
                }
            }
        }
        's1: {
            let sv20: i32 = (c.m.r32(l_state_5) as i32);
            'k8: {
            'k7: {
            'k6: {
            'k5: {
            'k4: {
            'k3: {
            'k2: {
                match sv20 { 0i32 => break 'k2, 1i32 => break 'k3, 2i32 => break 'k4, 3i32 => break 'k5, 4i32 => break 'k6, 5i32 => break 'k7, _ => break 'k8, }
            }
            F::Ramp_Start(c, l_ramp_7, F32(0x3e4ccccc), F32(0x3f800000), F32(0x00000000));
            { let a_ = l_state_5; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
                l_r_15 = F::Ramp_Step(c, l_ramp_7);
            } else {
                l_r_15 = 0i32;
            }
            {
                l_i_16 = 0i32;
                'l9: loop {
                    if !(((((l_i_16) < (4i32)) as i32)) != 0) { break 'l9; }
                    'c10: {
                        l_c_18 = (if ((((l_pos_3) & ((((1i32).wrapping_shl((l_i_16) as u32)) as u32)))) != 0) { (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)) } else { (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)) });
                        { let a1_ = { let a_ = c.m.r32(sym::gHudGauge); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_icon_8, a1_, (l_i_16).wrapping_add(19i32), 0i32) };
                        if ((((l_removed_12) & ((((1i32).wrapping_shl((l_i_16) as u32)) as u32)))) != 0) {
                            F::HudSprite_Show(c, l_icon_8, 1i32);
                            { let a1_ = ((c.m.r8((l_c_18).wrapping_add(((0i32) as u32)))) as i32); let a2_ = ((c.m.r8((l_c_18).wrapping_add(((1i32) as u32)))) as i32); let a3_ = ((c.m.r8((l_c_18).wrapping_add(((2i32) as u32)))) as i32); let a4_ = ((((rt::f2i(rt::fmul(rt::i2f((c.m.r8((l_c_18).wrapping_add(((3i32) as u32)))) as i32), F32(c.m.r32((l_ramp_7).wrapping_add(16)))))) as u8)) as i32); F::HudSprite_SetColor(c, l_icon_8, a1_, a2_, a3_, a4_) };
                            F::HudSprite_Show(c, (l_frames_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 1i32);
                            { let a4_ = ((((rt::f2i(rt::fmul(F32(c.m.r32((l_ramp_7).wrapping_add(16))), F32(0x43000000)))) as u8)) as i32); F::HudSprite_SetColor(c, (l_frames_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 128i32, 128i32, 128i32, a4_) };
                            F::HudSprite_Show(c, (l_shades_10).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32);
                        } else {
                            if ((((l_prev_4) & ((((1i32).wrapping_shl((l_i_16) as u32)) as u32)))) != 0) {
                                F::HudSprite_Show(c, l_icon_8, 1i32);
                                { let a1_ = ((c.m.r8((l_c_18).wrapping_add(((0i32) as u32)))) as i32); let a2_ = ((c.m.r8((l_c_18).wrapping_add(((1i32) as u32)))) as i32); let a3_ = ((c.m.r8((l_c_18).wrapping_add(((2i32) as u32)))) as i32); let a4_ = ((c.m.r8((l_c_18).wrapping_add(((3i32) as u32)))) as i32); F::HudSprite_SetColor(c, l_icon_8, a1_, a2_, a3_, a4_) };
                                F::HudSprite_Show(c, (l_frames_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 1i32);
                                F::HudSprite_SetColor(c, (l_frames_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 128i32, 128i32, 128i32, 128i32);
                                F::HudSprite_Show(c, (l_shades_10).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 1i32);
                            } else {
                                F::HudSprite_Show(c, l_icon_8, 0i32);
                                F::HudSprite_Show(c, (l_frames_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32);
                                F::HudSprite_Show(c, (l_shades_10).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32);
                            }
                        }
                        F::HudSprite_Move(c, l_icon_8, 5i32, (5i32).wrapping_neg());
                        { let a1_ = (c.m.r32((l_from_11).wrapping_add((((l_i_16) as u32)).wrapping_mul(4))) as i32); F::HudSprite_Move(c, l_icon_8, a1_, 0i32) };
                        F::HudSprite_SetRect(c, (l_frames_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32, 42i32, 0i32, 22i32);
                        F::HudSprite_SetUv(c, (l_frames_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32, 42i32, 22i32, 44i32);
                        F::HudSprite_Move(c, (l_frames_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 102i32, 24i32);
                        { let a1_ = (c.m.r32((l_from_11).wrapping_add((((l_i_16) as u32)).wrapping_mul(4))) as i32); F::HudSprite_Move(c, (l_frames_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), a1_, 0i32) };
                        F::HudSprite_SetRect(c, (l_shades_10).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 3i32, 39i32, 1i32, 21i32);
                        F::HudSprite_SetUv(c, (l_shades_10).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32, 36i32, 44i32, 64i32);
                        F::HudSprite_Move(c, (l_shades_10).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 102i32, 24i32);
                        { let a1_ = (c.m.r32((l_from_11).wrapping_add((((l_i_16) as u32)).wrapping_mul(4))) as i32); F::HudSprite_Move(c, (l_shades_10).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), a1_, 0i32) };
                        { let o_ = l_icon_8; let n_ = (o_).wrapping_add(28); l_icon_8 = n_; };
                    }
                    { let o_ = l_i_16; let n_ = (o_).wrapping_add(1); l_i_16 = n_; };
                }
            }
            if ((l_r_15) != 0) {
                { let a_ = l_state_5; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            break 's1;
            }
            F::Ramp_Start(c, l_ramp_7, F32(0x3e999999), F32(0x00000000), F32(0x3f800000));
            { let a_ = l_state_5; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
                l_r_15 = F::Ramp_Step(c, l_ramp_7);
            } else {
                l_r_15 = 0i32;
            }
            {
                l_i_16 = 0i32;
                'l11: loop {
                    if !(((((l_i_16) < (4i32)) as i32)) != 0) { break 'l11; }
                    'c12: {
                        l_c_18 = (if ((((l_pos_3) & ((((1i32).wrapping_shl((l_i_16) as u32)) as u32)))) != 0) { (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)) } else { (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)) });
                        { let a1_ = { let a_ = c.m.r32(sym::gHudGauge); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_icon_8, a1_, (l_i_16).wrapping_add(19i32), 0i32) };
                        if ((((l_kept_14) & ((((1i32).wrapping_shl((l_i_16) as u32)) as u32)))) != 0) {
                            F::HudSprite_Show(c, l_icon_8, 1i32);
                            F::HudSprite_Show(c, (l_frames_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 1i32);
                            F::HudSprite_Show(c, (l_shades_10).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 1i32);
                        } else {
                            F::HudSprite_Show(c, l_icon_8, 0i32);
                            F::HudSprite_Show(c, (l_frames_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32);
                            F::HudSprite_Show(c, (l_shades_10).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32);
                        }
                        l_t_19 = rt::fsub(F32(0x3f800000), { let a0_ = rt::fsub(F32(0x3f800000), F32(c.m.r32((l_ramp_7).wrapping_add(16)))); F::Port_powf(c, a0_, F32(0x40400000)) });
                        l_x_17 = rt::f2i(rt::fadd(rt::i2f((c.m.r32((l_from_11).wrapping_add((((l_i_16) as u32)).wrapping_mul(4))) as i32)), rt::fmul(rt::i2f(((c.m.r32((l_to_6).wrapping_add((((l_i_16) as u32)).wrapping_mul(4))) as i32)).wrapping_sub((c.m.r32((l_from_11).wrapping_add((((l_i_16) as u32)).wrapping_mul(4))) as i32))), l_t_19)));
                        { let a1_ = ((c.m.r8((l_c_18).wrapping_add(((0i32) as u32)))) as i32); let a2_ = ((c.m.r8((l_c_18).wrapping_add(((1i32) as u32)))) as i32); let a3_ = ((c.m.r8((l_c_18).wrapping_add(((2i32) as u32)))) as i32); let a4_ = ((c.m.r8((l_c_18).wrapping_add(((3i32) as u32)))) as i32); F::HudSprite_SetColor(c, l_icon_8, a1_, a2_, a3_, a4_) };
                        F::HudSprite_Move(c, l_icon_8, 5i32, (5i32).wrapping_neg());
                        F::HudSprite_Move(c, l_icon_8, l_x_17, 0i32);
                        F::HudSprite_SetColor(c, (l_frames_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 128i32, 128i32, 128i32, 128i32);
                        F::HudSprite_SetRect(c, (l_frames_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32, 42i32, 0i32, 22i32);
                        F::HudSprite_SetUv(c, (l_frames_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32, 42i32, 22i32, 44i32);
                        F::HudSprite_Move(c, (l_frames_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 102i32, 24i32);
                        F::HudSprite_Move(c, (l_frames_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), l_x_17, 0i32);
                        F::HudSprite_SetRect(c, (l_shades_10).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 3i32, 39i32, 1i32, 21i32);
                        F::HudSprite_SetUv(c, (l_shades_10).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32, 36i32, 44i32, 64i32);
                        F::HudSprite_Move(c, (l_shades_10).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 102i32, 24i32);
                        F::HudSprite_Move(c, (l_shades_10).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), l_x_17, 0i32);
                        { let o_ = l_icon_8; let n_ = (o_).wrapping_add(28); l_icon_8 = n_; };
                    }
                    { let o_ = l_i_16; let n_ = (o_).wrapping_add(1); l_i_16 = n_; };
                }
            }
            if ((l_r_15) != 0) {
                if (((((l_added_13) != (((0i32) as u32))) as i32)) != 0) {
                    { let a_ = l_state_5; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    { let v_ = (1i32).wrapping_neg(); let a_ = l_state_5; c.m.w32(a_, (v_) as u32); };
                }
            }
            break 's1;
            }
            F::Ramp_Start(c, l_ramp_7, F32(0x3e4ccccc), F32(0x00000000), F32(0x3f800000));
            { let a_ = l_state_5; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
                l_r_15 = F::Ramp_Step(c, l_ramp_7);
            } else {
                l_r_15 = 0i32;
            }
            {
                l_i_16 = 0i32;
                'l13: loop {
                    if !(((((l_i_16) < (4i32)) as i32)) != 0) { break 'l13; }
                    'c14: {
                        l_c_18 = (if ((((l_pos_3) & ((((1i32).wrapping_shl((l_i_16) as u32)) as u32)))) != 0) { (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)) } else { (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)) });
                        { let a1_ = { let a_ = c.m.r32(sym::gHudGauge); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_icon_8, a1_, (l_i_16).wrapping_add(19i32), 0i32) };
                        if ((((l_added_13) & ((((1i32).wrapping_shl((l_i_16) as u32)) as u32)))) != 0) {
                            F::HudSprite_Show(c, l_icon_8, 1i32);
                            { let a1_ = ((c.m.r8((l_c_18).wrapping_add(((0i32) as u32)))) as i32); let a2_ = ((c.m.r8((l_c_18).wrapping_add(((1i32) as u32)))) as i32); let a3_ = ((c.m.r8((l_c_18).wrapping_add(((2i32) as u32)))) as i32); let a4_ = ((((rt::f2i(rt::fmul(rt::i2f((c.m.r8((l_c_18).wrapping_add(((3i32) as u32)))) as i32), F32(c.m.r32((l_ramp_7).wrapping_add(16)))))) as u8)) as i32); F::HudSprite_SetColor(c, l_icon_8, a1_, a2_, a3_, a4_) };
                            F::HudSprite_Show(c, (l_frames_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 1i32);
                            { let a4_ = ((((rt::f2i(rt::fmul(F32(c.m.r32((l_ramp_7).wrapping_add(16))), F32(0x43000000)))) as u8)) as i32); F::HudSprite_SetColor(c, (l_frames_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 128i32, 128i32, 128i32, a4_) };
                            F::HudSprite_Show(c, (l_shades_10).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32);
                        } else {
                            if ((((l_mask_2) & ((((1i32).wrapping_shl((l_i_16) as u32)) as u32)))) != 0) {
                                F::HudSprite_Show(c, l_icon_8, 1i32);
                                { let a1_ = ((c.m.r8((l_c_18).wrapping_add(((0i32) as u32)))) as i32); let a2_ = ((c.m.r8((l_c_18).wrapping_add(((1i32) as u32)))) as i32); let a3_ = ((c.m.r8((l_c_18).wrapping_add(((2i32) as u32)))) as i32); let a4_ = ((c.m.r8((l_c_18).wrapping_add(((3i32) as u32)))) as i32); F::HudSprite_SetColor(c, l_icon_8, a1_, a2_, a3_, a4_) };
                                F::HudSprite_Show(c, (l_frames_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 1i32);
                                F::HudSprite_SetColor(c, (l_frames_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 128i32, 128i32, 128i32, 128i32);
                                F::HudSprite_Show(c, (l_shades_10).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 1i32);
                            } else {
                                F::HudSprite_Show(c, l_icon_8, 0i32);
                                F::HudSprite_Show(c, (l_frames_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32);
                                F::HudSprite_Show(c, (l_shades_10).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32);
                            }
                        }
                        F::HudSprite_Move(c, l_icon_8, 5i32, (5i32).wrapping_neg());
                        { let a1_ = (c.m.r32((l_to_6).wrapping_add((((l_i_16) as u32)).wrapping_mul(4))) as i32); F::HudSprite_Move(c, l_icon_8, a1_, 0i32) };
                        F::HudSprite_SetRect(c, (l_frames_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32, 42i32, 0i32, 22i32);
                        F::HudSprite_SetUv(c, (l_frames_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32, 42i32, 22i32, 44i32);
                        F::HudSprite_Move(c, (l_frames_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 102i32, 24i32);
                        { let a1_ = (c.m.r32((l_to_6).wrapping_add((((l_i_16) as u32)).wrapping_mul(4))) as i32); F::HudSprite_Move(c, (l_frames_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), a1_, 0i32) };
                        F::HudSprite_SetRect(c, (l_shades_10).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 3i32, 39i32, 1i32, 21i32);
                        F::HudSprite_SetUv(c, (l_shades_10).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32, 36i32, 44i32, 64i32);
                        F::HudSprite_Move(c, (l_shades_10).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 102i32, 24i32);
                        { let a1_ = (c.m.r32((l_to_6).wrapping_add((((l_i_16) as u32)).wrapping_mul(4))) as i32); F::HudSprite_Move(c, (l_shades_10).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), a1_, 0i32) };
                        { let o_ = l_icon_8; let n_ = (o_).wrapping_add(28); l_icon_8 = n_; };
                    }
                    { let o_ = l_i_16; let n_ = (o_).wrapping_add(1); l_i_16 = n_; };
                }
            }
            if ((l_r_15) != 0) {
                { let a_ = l_state_5; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            break 's1;
            }
            {
                l_i_16 = 0i32;
                'l15: loop {
                    if !(((((l_i_16) < (4i32)) as i32)) != 0) { break 'l15; }
                    'c16: {
                        l_c_18 = (if ((((l_pos_3) & ((((1i32).wrapping_shl((l_i_16) as u32)) as u32)))) != 0) { (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)) } else { (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)) });
                        { let a1_ = { let a_ = c.m.r32(sym::gHudGauge); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_icon_8, a1_, (l_i_16).wrapping_add(19i32), 0i32) };
                        if ((((l_mask_2) & ((((1i32).wrapping_shl((l_i_16) as u32)) as u32)))) != 0) {
                            F::HudSprite_Show(c, l_icon_8, 1i32);
                            F::HudSprite_Show(c, (l_frames_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 1i32);
                            F::HudSprite_Show(c, (l_shades_10).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 1i32);
                        } else {
                            F::HudSprite_Show(c, l_icon_8, 0i32);
                            F::HudSprite_Show(c, (l_frames_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32);
                            F::HudSprite_Show(c, (l_shades_10).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32);
                        }
                        { let a1_ = ((c.m.r8((l_c_18).wrapping_add(((0i32) as u32)))) as i32); let a2_ = ((c.m.r8((l_c_18).wrapping_add(((1i32) as u32)))) as i32); let a3_ = ((c.m.r8((l_c_18).wrapping_add(((2i32) as u32)))) as i32); let a4_ = ((c.m.r8((l_c_18).wrapping_add(((3i32) as u32)))) as i32); F::HudSprite_SetColor(c, l_icon_8, a1_, a2_, a3_, a4_) };
                        F::HudSprite_Move(c, l_icon_8, 5i32, (5i32).wrapping_neg());
                        { let a1_ = (c.m.r32((l_to_6).wrapping_add((((l_i_16) as u32)).wrapping_mul(4))) as i32); F::HudSprite_Move(c, l_icon_8, a1_, 0i32) };
                        F::HudSprite_SetColor(c, (l_frames_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 128i32, 128i32, 128i32, 128i32);
                        F::HudSprite_SetRect(c, (l_frames_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32, 42i32, 0i32, 22i32);
                        F::HudSprite_SetUv(c, (l_frames_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32, 42i32, 22i32, 44i32);
                        F::HudSprite_Move(c, (l_frames_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 102i32, 24i32);
                        { let a1_ = (c.m.r32((l_to_6).wrapping_add((((l_i_16) as u32)).wrapping_mul(4))) as i32); F::HudSprite_Move(c, (l_frames_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), a1_, 0i32) };
                        F::HudSprite_SetRect(c, (l_shades_10).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 3i32, 39i32, 1i32, 21i32);
                        F::HudSprite_SetUv(c, (l_shades_10).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32, 36i32, 44i32, 64i32);
                        F::HudSprite_Move(c, (l_shades_10).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 102i32, 24i32);
                        { let a1_ = (c.m.r32((l_to_6).wrapping_add((((l_i_16) as u32)).wrapping_mul(4))) as i32); F::HudSprite_Move(c, (l_shades_10).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), a1_, 0i32) };
                        { let o_ = l_icon_8; let n_ = (o_).wrapping_add(28); l_icon_8 = n_; };
                    }
                    { let o_ = l_i_16; let n_ = (o_).wrapping_add(1); l_i_16 = n_; };
                }
            }
            break 's1;
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_UpdateStatIcons", &PROBE_N, &[], true); }
    r_
}

pub fn HudGauge_DrawHp(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_DrawHp", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(28)); F::HudSprite_DrawAlphaClear(c, a0_) };
    F::HudGauge_UpdateHp(c);
    F::HudGfx_CallBegin(c, fid::HudGauge_GsBeginMask);
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudGauge); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudGauge); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 1i32) };
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudGauge); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
    F::HudGfx_CallEnd(c, fid::HudGauge_GsEndMask);
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(28)); F::HudSprite_DrawAlphaClear(c, a0_) };
    F::HudGfx_CallBegin(c, fid::HudGauge_GsBeginMask);
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudGauge); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudGauge); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 1i32) };
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudGauge); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
    F::HudGfx_CallEnd(c, fid::HudGauge_GsEndMask);
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(28)); F::HudSprite_DrawAlphaClear(c, a0_) };
    F::HudGfx_CallBegin(c, fid::HudGauge_GsBeginMask);
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudGauge); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 1i32) };
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudGauge); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
    F::HudGfx_CallEnd(c, fid::HudGauge_GsEndMask);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_DrawHp", &PROBE_N, &[], true); }
    r_
}

pub fn HudGauge_DrawAura(c: &mut Ctx, mut p_node_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_DrawAura", &PROBE_N, &[(p_node_1) as u32], false); }
    let mut l_spr_2: u32 = 0;
    let mut l_i_3: u32 = 0;
    let r_ = 'f: {
    l_spr_2 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((47i32) as u32)).wrapping_mul(28));
    F::HudSprite_DrawAlphaClear(c, l_spr_2);
    F::HudGfx_CallBegin(c, fid::HudGauge_GsBeginMask2);
    { let a1_ = { let a_ = c.m.r32(sym::gHudGauge); c.m.r32(a_) }; F::HudSprite_Draw(c, l_spr_2, a1_, 0i32) };
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((48i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudGauge); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((49i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudGauge); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((50i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudGauge); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((51i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudGauge); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((52i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudGauge); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((63i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudGauge); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 1i32) };
    {
        l_i_3 = ((0i32) as u32);
        'l1: loop {
            if !(((((l_i_3) < (c.m.r32((p_node_1).wrapping_add(32)))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = { let a_ = (c.m.r32((p_node_1).wrapping_add(36))).wrapping_add((l_i_3).wrapping_mul(4)); c.m.r32(a_) }; let a1_ = { let a_ = c.m.r32(sym::gHudGauge); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 1i32) };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    F::HudGfx_CallEnd(c, fid::HudGauge_GsEndMask2);
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((47i32) as u32)).wrapping_mul(28)); F::HudSprite_DrawAlphaClear(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_DrawAura", &PROBE_N, &[], true); }
    r_
}

pub fn HudGauge_DrawHpBarCount(c: &mut Ctx, mut p_node_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_DrawHpBarCount", &PROBE_N, &[(p_node_1) as u32], false); }
    let mut l_i_2: u32 = 0;
    let r_ = 'f: {
    F::HudGauge_UpdateHpBarCount(c);
    {
        l_i_2 = ((0i32) as u32);
        'l1: loop {
            if !(((((l_i_2) < (c.m.r32((p_node_1).wrapping_add(32)))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = { let a_ = (c.m.r32((p_node_1).wrapping_add(36))).wrapping_add((l_i_2).wrapping_mul(4)); c.m.r32(a_) }; let a1_ = { let a_ = c.m.r32(sym::gHudGauge); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_DrawHpBarCount", &PROBE_N, &[], true); }
    r_
}

pub fn HudGauge_DrawBlast(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_DrawBlast", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((38i32) as u32)).wrapping_mul(28)); F::HudSprite_DrawAlphaClear(c, a0_) };
    F::HudGfx_CallBegin(c, fid::HudGauge_GsBeginMask);
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((37i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudGauge); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 1i32) };
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((38i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudGauge); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
    F::HudGfx_CallEnd(c, fid::HudGauge_GsEndMask);
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((35i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudGauge); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((75i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudGauge); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_DrawBlast", &PROBE_N, &[], true); }
    r_
}

pub fn HudGauge_ClearFaceTexMark(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_ClearFaceTexMark", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_tex_2: u32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(488)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
                    l_tex_2 = { let a_ = ({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(488)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(16); c.m.r32(a_) };
                    { let r_ = l_i_1; let n_ = (l_tex_2).wrapping_add((((r_) as u32)).wrapping_mul(64)); l_tex_2 = n_; };
                    { let v_ = 0i32; let a_ = (l_tex_2).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_ClearFaceTexMark", &PROBE_N, &[], true); }
    r_
}

pub fn HudGauge_DrawFace(c: &mut Ctx, mut p_node_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_DrawFace", &PROBE_N, &[(p_node_1) as u32], false); }
    let mut l_i_2: u32 = 0;
    let mut l_spr_3: u32 = 0;
    let r_ = 'f: {
    { let v_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(496)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(80); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(488)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    if ((((({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(488)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        F::HudGauge_ClearFaceTexMark(c);
        {
            l_i_2 = ((0i32) as u32);
            'l1: loop {
                if !(((((l_i_2) < (c.m.r32((p_node_1).wrapping_add(32)))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_spr_3 = { let a_ = (c.m.r32((p_node_1).wrapping_add(36))).wrapping_add((l_i_2).wrapping_mul(4)); c.m.r32(a_) };
                    { let a1_ = { let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(488)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }; F::HudSprite_InitTex(c, l_spr_3, a1_, a2_, 0i32) };
                    if (((({ let a_ = ({ let a0_ = { let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }; F::BtlCtrl_GetObj(c, a0_) }).wrapping_add(2624); c.m.r32(a_) }) & (((262144i32) as u32)))) != 0) {
                        F::HudSprite_SetColor(c, l_spr_3, 128i32, 80i32, 112i32, 128i32);
                    } else {
                        F::HudSprite_SetColor(c, l_spr_3, 128i32, 128i32, 128i32, 128i32);
                    }
                    { let a1_ = { let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }; F::HudSprite_SetMirror(c, l_spr_3, a1_) };
                    { let a1_ = { let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(488)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::HudSprite_DrawAt(c, l_spr_3, a1_, ((((0i32) as i64 as u64)) as u8), 11264i32, 11472i32) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_DrawFace", &PROBE_N, &[], true); }
    r_
}

pub fn HudGauge_DrawSprites(c: &mut Ctx, mut p_node_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_DrawSprites", &PROBE_N, &[(p_node_1) as u32], false); }
    let mut l_i_2: u32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = ((0i32) as u32);
        'l1: loop {
            if !(((((l_i_2) < (c.m.r32((p_node_1).wrapping_add(32)))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = { let a_ = (c.m.r32((p_node_1).wrapping_add(36))).wrapping_add((l_i_2).wrapping_mul(4)); c.m.r32(a_) }; let a1_ = { let a_ = c.m.r32(sym::gHudGauge); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_DrawSprites", &PROBE_N, &[], true); }
    r_
}

pub fn HudGauge_SelectSide(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_SelectSide", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_flip_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    l_flip_2 = (((p_side_1) != (0i32)) as i32);
    { let v_ = p_side_1; let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
    F::Port_GsMarker(c, (if ((p_side_1) != 0) { 17i32 } else { 16i32 }));
    { let v_ = (((({ let a_ = { let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.r32(a_) }; c.m.r32(a_) }) & ((((!(2i32))) as u32)))) | ((((l_flip_2).wrapping_shl((1i32) as u32)) as u32))); let a_ = { let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.r32(a_) }; c.m.w32(a_, v_); };
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((35i32) as u32)).wrapping_mul(28)); F::HudSprite_SetMirror(c, a0_, l_flip_2) };
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((75i32) as u32)).wrapping_mul(28)); F::HudSprite_SetMirror(c, a0_, l_flip_2) };
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(((((43i32).wrapping_add(l_i_3)) as u32)).wrapping_mul(28)); F::HudSprite_SetMirror(c, a0_, l_flip_2) };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_SelectSide", &PROBE_N, &[], true); }
    r_
}

pub fn HudGauge_SetHp(c: &mut Ctx, mut p_side_1: i32, mut p_hp_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_SetHp", &PROBE_N, &[(p_side_1) as u32, (p_hp_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_hp_2; let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(48)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_SetHp", &PROBE_N, &[], true); }
    r_
}

pub fn HudGauge_SetKi(c: &mut Ctx, mut p_side_1: i32, mut p_ki_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_SetKi", &PROBE_N, &[(p_side_1) as u32, (p_ki_2) as u32], false); }
    let r_ = 'f: {
    if (((((rt::div_i32({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(120)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }, 20000i32)) < (rt::div_i32(p_ki_2, 20000i32))) as i32)) != 0) {
        { let a0_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(136)).wrapping_add((((p_side_1) as u32)).wrapping_mul(24)); F::Ramp_Start(c, a0_, F32(0x3f800000), F32(0x3f800000), F32(0x00000000)) };
        if (((((p_ki_2) > (99999i32)) as i32)) != 0) {
            { let a0_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(184)).wrapping_add((((p_side_1) as u32)).wrapping_mul(24)); F::Ramp_Start(c, a0_, F32(0x3f800000), F32(0x00000000), F32(0x3f800000)) };
        }
    }
    { let v_ = p_ki_2; let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(120)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_SetKi", &PROBE_N, &[], true); }
    r_
}

pub fn HudGauge_SetMaxPower(c: &mut Ctx, mut p_side_1: i32, mut p_value_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_SetMaxPower", &PROBE_N, &[(p_side_1) as u32, (p_value_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_value_2; let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(232)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_SetMaxPower", &PROBE_N, &[], true); }
    r_
}

pub fn HudGauge_SetBlast(c: &mut Ctx, mut p_side_1: i32, mut p_blast_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_SetBlast", &PROBE_N, &[(p_side_1) as u32, (p_blast_2) as u32], false); }
    let r_ = 'f: {
    if (((((rt::div_i32({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(112)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }, 100000i32)) < (rt::div_i32(p_blast_2, 100000i32))) as i32)) != 0) {
        { let a0_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(240)).wrapping_add((((p_side_1) as u32)).wrapping_mul(24)); F::Ramp_Start(c, a0_, F32(0x3f800000), F32(0x3f800000), F32(0x00000000)) };
    }
    { let v_ = p_blast_2; let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(112)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_SetBlast", &PROBE_N, &[], true); }
    r_
}

pub fn HudGauge_SetStatIcons(c: &mut Ctx, mut p_side_1: i32, mut p_mask_2: i32, mut p_pos_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_SetStatIcons", &PROBE_N, &[(p_side_1) as u32, (p_mask_2) as u32, (p_pos_3) as u32], false); }
    let mut l_n_4: i32 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
    l_n_4 = 0i32;
    if (((((((((((p_mask_2) as u32)) == ({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(288)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) })) as i32)) != 0) && ((((({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(296)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (((((((p_mask_2) & (p_pos_3))) as u32)) | ((((({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(312)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }) & ({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(296)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }))) & ((((!(p_mask_2))) as u32))))))) as i32)) != 0)) as i32)) != 0) {
        break 'f;
    }
    { let v_ = { let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(288)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(312)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((p_mask_2) as u32); let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(288)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((((p_mask_2) & (p_pos_3))) as u32)) | ((((({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(312)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }) & ({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(296)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }))) & ((((!(p_mask_2))) as u32))))); let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(296)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(432)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let _ = { let a0_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(352)).wrapping_add((((p_side_1) as u32)).wrapping_mul(16)); let a1_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(320)).wrapping_add((((p_side_1) as u32)).wrapping_mul(16)); F::memcpy(c, a0_, a1_, 16u32) }; };
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((l_i_5) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(288)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }) & ((((1i32).wrapping_shl((l_i_5) as u32)) as u32)))) != 0) {
                    { let v_ = (l_n_4).wrapping_mul(24i32); let a_ = (((c.m.r32(sym::gHudGauge)).wrapping_add(320)).wrapping_add((((p_side_1) as u32)).wrapping_mul(16))).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let o_ = l_n_4; let n_ = (o_).wrapping_add(1); l_n_4 = n_; };
                } else {
                    { let v_ = (l_n_4).wrapping_mul(24i32); let a_ = (((c.m.r32(sym::gHudGauge)).wrapping_add(320)).wrapping_add((((p_side_1) as u32)).wrapping_mul(16))).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                }
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_SetStatIcons", &PROBE_N, &[], true); }
    r_
}

pub fn HudGauge_SetKiReserve(c: &mut Ctx, mut p_side_1: i32, mut p_value_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_SetKiReserve", &PROBE_N, &[(p_side_1) as u32, (p_value_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_value_2; let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(304)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_SetKiReserve", &PROBE_N, &[], true); }
    r_
}

pub fn HudGauge_ShakeHp(c: &mut Ctx, mut p_side_1: i32, mut p_count_2: u16, mut p_amp_3: u16) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_ShakeHp", &PROBE_N, &[(p_side_1) as u32, (p_count_2) as u32, (p_amp_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_count_2; let a_ = (((c.m.r32(sym::gHudGauge)).wrapping_add(12)).wrapping_add((((p_side_1) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w16(a_, v_); };
    { let v_ = p_amp_3; let a_ = ((((c.m.r32(sym::gHudGauge)).wrapping_add(12)).wrapping_add((((p_side_1) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))).wrapping_add(2); c.m.w16(a_, v_); };
    { let v_ = p_count_2; let a_ = (((c.m.r32(sym::gHudGauge)).wrapping_add(12)).wrapping_add((((p_side_1) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w16(a_, v_); };
    { let v_ = p_amp_3; let a_ = ((((c.m.r32(sym::gHudGauge)).wrapping_add(12)).wrapping_add((((p_side_1) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4))).wrapping_add(2); c.m.w16(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_ShakeHp", &PROBE_N, &[], true); }
    r_
}

pub fn HudGauge_ShakeKi(c: &mut Ctx, mut p_side_1: i32, mut p_count_2: u16, mut p_amp_3: u16) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_ShakeKi", &PROBE_N, &[(p_side_1) as u32, (p_count_2) as u32, (p_amp_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_count_2; let a_ = (((c.m.r32(sym::gHudGauge)).wrapping_add(12)).wrapping_add((((p_side_1) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w16(a_, v_); };
    { let v_ = p_amp_3; let a_ = ((((c.m.r32(sym::gHudGauge)).wrapping_add(12)).wrapping_add((((p_side_1) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))).wrapping_add(2); c.m.w16(a_, v_); };
    { let v_ = p_count_2; let a_ = (((c.m.r32(sym::gHudGauge)).wrapping_add(12)).wrapping_add((((p_side_1) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w16(a_, v_); };
    { let v_ = p_amp_3; let a_ = ((((c.m.r32(sym::gHudGauge)).wrapping_add(12)).wrapping_add((((p_side_1) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))).wrapping_add(2); c.m.w16(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_ShakeKi", &PROBE_N, &[], true); }
    r_
}

pub fn HudGauge_SetFighter(c: &mut Ctx, mut p_side_1: i32, mut p_obj_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_SetFighter", &PROBE_N, &[(p_side_1) as u32, (p_obj_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_obj_2; let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(496)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_SetFighter", &PROBE_N, &[], true); }
    r_
}

pub fn HudGauge_SlideOut(c: &mut Ctx, mut p_seconds_1: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_SlideOut", &PROBE_N, &[(p_seconds_1).0], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32(sym::gHudGauge)).wrapping_add(560); F::Ramp_Start(c, a0_, p_seconds_1, F32(0x00000000), F32(0x3f800000)) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_SlideOut", &PROBE_N, &[], true); }
    r_
}

pub fn HudGauge_SlideIn(c: &mut Ctx, mut p_seconds_1: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_SlideIn", &PROBE_N, &[(p_seconds_1).0], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32(sym::gHudGauge)).wrapping_add(560); F::Ramp_Start(c, a0_, p_seconds_1, F32(0x3f800000), F32(0x00000000)) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_SlideIn", &PROBE_N, &[], true); }
    r_
}

pub fn HudGauge_SetSwitching(c: &mut Ctx, mut p_side_1: i32, mut p_switching_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_SetSwitching", &PROBE_N, &[(p_side_1) as u32, (p_switching_2) as u32], false); }
    let r_ = 'f: {
    if ((((({ let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(504)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != (p_switching_2)) as i32)) != 0) {
        if ((p_switching_2) != 0) {
            { let a0_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(512)).wrapping_add((((p_side_1) as u32)).wrapping_mul(24)); F::Ramp_Start(c, a0_, F32(0x3e999999), F32(0x00000000), F32(0x3f800000)) };
        } else {
            { let a0_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(512)).wrapping_add((((p_side_1) as u32)).wrapping_mul(24)); F::Ramp_Start(c, a0_, F32(0x3e999999), F32(0x3f800000), F32(0x00000000)) };
        }
        if ((((!((p_switching_2) != 0)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(40)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(128)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
    }
    { let v_ = p_switching_2; let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(504)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_SetSwitching", &PROBE_N, &[], true); }
    r_
}

pub fn HudGauge_Init(c: &mut Ctx, mut p_out_1: u32, mut p_res_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_Init", &PROBE_N, &[(p_out_1) as u32, (p_res_2) as u32], false); }
    let mut l_spr_3: u32 = 0;
    let mut l_grp_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, ((648u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gHudGauge; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gHudGauge); F::memset(c, a0_, 0i32, 648u32) }; };
    { let v_ = F::Heap_Alloc(c, (((((76i32) as u32)).wrapping_mul(28u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.w32(a_, v_); };
    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }; F::memset(c, a0_, 0i32, (((76i32) as u32)).wrapping_mul(28u32)) }; };
    { let v_ = F::Heap_Alloc(c, (((((12i32) as u32)).wrapping_mul(56u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.w32(a_, v_); };
    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.r32(a_) }; F::memset(c, a0_, 0i32, (((12i32) as u32)).wrapping_mul(56u32)) }; };
    { let v_ = p_res_2; let a_ = c.m.r32(sym::gHudGauge); c.m.w32(a_, v_); };
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((l_i_5) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = (1i32).wrapping_neg(); let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(432)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
    {
        l_i_5 = 0i32;
        'l3: loop {
            if !(((((l_i_5) < (2i32)) as i32)) != 0) { break 'l3; }
            'c4: {
                { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(488)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(496)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 6i32, 0i32);
    F::HudSprite_Move(c, l_spr_3, 0i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 160i32, 0i32, 16i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 160i32, 48i32, 64i32);
    F::HudSprite_Move(c, l_spr_3, 160i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 1i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 160i32, 0i32, 16i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 160i32, 0i32, 16i32);
    F::HudSprite_Move(c, l_spr_3, 0i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 160i32, 0i32, 16i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 160i32, 48i32, 64i32);
    F::HudSprite_Move(c, l_spr_3, 160i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 2i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 160i32, 0i32, 16i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 160i32, 0i32, 16i32);
    F::HudSprite_Move(c, l_spr_3, 0i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 160i32, 0i32, 16i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 160i32, 48i32, 64i32);
    F::HudSprite_Move(c, l_spr_3, 160i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 1i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 160i32, 0i32, 16i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 160i32, 0i32, 16i32);
    F::HudSprite_Move(c, l_spr_3, 0i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 2i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 160i32, 0i32, 16i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 160i32, 0i32, 16i32);
    F::HudSprite_Move(c, l_spr_3, 0i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 160i32, 0i32, 16i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 160i32, 0i32, 16i32);
    F::HudSprite_Move(c, l_spr_3, 0i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 1i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 16i32, 0i32, 16i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 16i32, 16i32, 32i32);
    F::HudSprite_Move(c, l_spr_3, 0i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((10i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 1i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 16i32, 0i32, 16i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 16i32, 16i32, 32i32);
    F::HudSprite_Move(c, l_spr_3, 9i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((11i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 1i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 16i32, 0i32, 16i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 16i32, 16i32, 32i32);
    F::HudSprite_Move(c, l_spr_3, 18i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((12i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 1i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 16i32, 0i32, 16i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 16i32, 16i32, 32i32);
    F::HudSprite_Move(c, l_spr_3, 27i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((13i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 1i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 16i32, 0i32, 16i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 16i32, 16i32, 32i32);
    F::HudSprite_Move(c, l_spr_3, 36i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((14i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 1i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 16i32, 0i32, 16i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 16i32, 16i32, 32i32);
    F::HudSprite_Move(c, l_spr_3, 45i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((15i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 3i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 14i32, 0i32, 17i32);
    F::HudSprite_SetUv(c, l_spr_3, 16i32, 30i32, 16i32, 33i32);
    F::HudSprite_Move(c, l_spr_3, 32i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((16i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 3i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 14i32, 0i32, 17i32);
    F::HudSprite_SetUv(c, l_spr_3, 16i32, 30i32, 16i32, 33i32);
    F::HudSprite_Move(c, l_spr_3, 24i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((17i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 3i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 14i32, 0i32, 17i32);
    F::HudSprite_SetUv(c, l_spr_3, 16i32, 30i32, 16i32, 33i32);
    F::HudSprite_Move(c, l_spr_3, 16i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((18i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 3i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 14i32, 0i32, 17i32);
    F::HudSprite_SetUv(c, l_spr_3, 16i32, 30i32, 16i32, 33i32);
    F::HudSprite_Move(c, l_spr_3, 8i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((19i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 3i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 14i32, 0i32, 17i32);
    F::HudSprite_SetUv(c, l_spr_3, 16i32, 30i32, 16i32, 33i32);
    F::HudSprite_Move(c, l_spr_3, 0i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((20i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 2i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 16i32, 0i32, 17i32);
    F::HudSprite_SetUv(c, l_spr_3, 16i32, 32i32, 16i32, 33i32);
    F::HudSprite_Move(c, l_spr_3, 28i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((21i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 2i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 16i32, 0i32, 17i32);
    F::HudSprite_SetUv(c, l_spr_3, 16i32, 32i32, 16i32, 33i32);
    F::HudSprite_Move(c, l_spr_3, 21i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((22i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 2i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 16i32, 0i32, 17i32);
    F::HudSprite_SetUv(c, l_spr_3, 16i32, 32i32, 16i32, 33i32);
    F::HudSprite_Move(c, l_spr_3, 14i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((23i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 2i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 16i32, 0i32, 17i32);
    F::HudSprite_SetUv(c, l_spr_3, 16i32, 32i32, 16i32, 33i32);
    F::HudSprite_Move(c, l_spr_3, 7i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((24i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 2i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 16i32, 0i32, 17i32);
    F::HudSprite_SetUv(c, l_spr_3, 16i32, 32i32, 16i32, 33i32);
    F::HudSprite_Move(c, l_spr_3, 0i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((25i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 1i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 16i32, 0i32, 17i32);
    F::HudSprite_SetUv(c, l_spr_3, 16i32, 32i32, 16i32, 33i32);
    F::HudSprite_Move(c, l_spr_3, 28i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((26i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 1i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 16i32, 0i32, 17i32);
    F::HudSprite_SetUv(c, l_spr_3, 16i32, 32i32, 16i32, 33i32);
    F::HudSprite_Move(c, l_spr_3, 21i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((27i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 1i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 16i32, 0i32, 17i32);
    F::HudSprite_SetUv(c, l_spr_3, 16i32, 32i32, 16i32, 33i32);
    F::HudSprite_Move(c, l_spr_3, 14i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((28i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 1i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 16i32, 0i32, 17i32);
    F::HudSprite_SetUv(c, l_spr_3, 16i32, 32i32, 16i32, 33i32);
    F::HudSprite_Move(c, l_spr_3, 7i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((29i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 1i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 16i32, 0i32, 17i32);
    F::HudSprite_SetUv(c, l_spr_3, 16i32, 32i32, 16i32, 33i32);
    F::HudSprite_Move(c, l_spr_3, 0i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((30i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 5i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 16i32, 0i32, 17i32);
    F::HudSprite_SetUv(c, l_spr_3, 16i32, 32i32, 16i32, 33i32);
    F::HudSprite_Move(c, l_spr_3, 28i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((31i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 5i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 16i32, 0i32, 17i32);
    F::HudSprite_SetUv(c, l_spr_3, 16i32, 32i32, 16i32, 33i32);
    F::HudSprite_Move(c, l_spr_3, 21i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((32i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 5i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 16i32, 0i32, 17i32);
    F::HudSprite_SetUv(c, l_spr_3, 16i32, 32i32, 16i32, 33i32);
    F::HudSprite_Move(c, l_spr_3, 14i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((33i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 5i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 16i32, 0i32, 17i32);
    F::HudSprite_SetUv(c, l_spr_3, 16i32, 32i32, 16i32, 33i32);
    F::HudSprite_Move(c, l_spr_3, 7i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((34i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 5i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 16i32, 0i32, 17i32);
    F::HudSprite_SetUv(c, l_spr_3, 16i32, 32i32, 16i32, 33i32);
    F::HudSprite_Move(c, l_spr_3, 0i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((35i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 14i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, (32i32).wrapping_neg(), 0i32, 0i32, 32i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 32i32, 0i32, 32i32);
    F::HudSprite_Move(c, l_spr_3, 10i32, (6i32).wrapping_neg());
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((36i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 15i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 48i32, 0i32, 12i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 48i32, 0i32, 12i32);
    F::HudSprite_Move(c, l_spr_3, 27i32, 46i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((37i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 26i32, 0i32, 6i32);
    F::HudSprite_SetUv(c, l_spr_3, 192i32, 218i32, 0i32, 6i32);
    F::HudSprite_Move(c, l_spr_3, 5i32, 3i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((38i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 1i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 26i32, 0i32, 6i32);
    F::HudSprite_SetUv(c, l_spr_3, 161i32, 187i32, 0i32, 6i32);
    F::HudSprite_Move(c, l_spr_3, 5i32, 3i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((39i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 15i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 42i32, 0i32, 22i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 42i32, 22i32, 44i32);
    F::HudSprite_Move(c, l_spr_3, 0i32, 0i32);
    F::HudSprite_Move(c, l_spr_3, 102i32, 24i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((40i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 15i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 42i32, 0i32, 22i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 42i32, 22i32, 44i32);
    F::HudSprite_Move(c, l_spr_3, 24i32, 0i32);
    F::HudSprite_Move(c, l_spr_3, 102i32, 24i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((41i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 15i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 42i32, 0i32, 22i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 42i32, 22i32, 44i32);
    F::HudSprite_Move(c, l_spr_3, 48i32, 0i32);
    F::HudSprite_Move(c, l_spr_3, 102i32, 24i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((42i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 15i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 42i32, 0i32, 22i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 42i32, 22i32, 44i32);
    F::HudSprite_Move(c, l_spr_3, 72i32, 0i32);
    F::HudSprite_Move(c, l_spr_3, 102i32, 24i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((43i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 19i32, 0i32);
    F::HudSprite_Move(c, l_spr_3, 5i32, (5i32).wrapping_neg());
    F::HudSprite_Move(c, l_spr_3, 0i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((44i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 20i32, 0i32);
    F::HudSprite_Move(c, l_spr_3, 5i32, (5i32).wrapping_neg());
    F::HudSprite_Move(c, l_spr_3, 24i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((45i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 21i32, 0i32);
    F::HudSprite_Move(c, l_spr_3, 5i32, (5i32).wrapping_neg());
    F::HudSprite_Move(c, l_spr_3, 48i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((46i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 22i32, 0i32);
    F::HudSprite_Move(c, l_spr_3, 5i32, (5i32).wrapping_neg());
    F::HudSprite_Move(c, l_spr_3, 72i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((47i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 7i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((48i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 15i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 48i32, 1i32, 11i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 48i32, 12i32, 22i32);
    F::HudSprite_Move(c, l_spr_3, 27i32, 46i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((49i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 15i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 3i32, 39i32, 1i32, 21i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 36i32, 44i32, 64i32);
    F::HudSprite_Move(c, l_spr_3, 0i32, 0i32);
    F::HudSprite_Move(c, l_spr_3, 102i32, 24i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((50i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 15i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 3i32, 39i32, 1i32, 21i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 36i32, 44i32, 64i32);
    F::HudSprite_Move(c, l_spr_3, 24i32, 0i32);
    F::HudSprite_Move(c, l_spr_3, 102i32, 24i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((51i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 15i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 3i32, 39i32, 1i32, 21i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 36i32, 44i32, 64i32);
    F::HudSprite_Move(c, l_spr_3, 48i32, 0i32);
    F::HudSprite_Move(c, l_spr_3, 102i32, 24i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((52i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 15i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 3i32, 39i32, 1i32, 21i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 36i32, 44i32, 64i32);
    F::HudSprite_Move(c, l_spr_3, 72i32, 0i32);
    F::HudSprite_Move(c, l_spr_3, 102i32, 24i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((53i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 16i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 64i32, 0i32, 64i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 64i32, 0i32, 64i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((54i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 16i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 64i32, 0i32, 64i32);
    F::HudSprite_SetUv(c, l_spr_3, 64i32, 128i32, 0i32, 64i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((55i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 16i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 64i32, 0i32, 64i32);
    F::HudSprite_SetUv(c, l_spr_3, 128i32, 192i32, 0i32, 64i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((56i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 16i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 64i32, 0i32, 64i32);
    F::HudSprite_SetUv(c, l_spr_3, 192i32, 256i32, 0i32, 64i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((57i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 16i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 64i32, 0i32, 64i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 64i32, 64i32, 128i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((58i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 16i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 64i32, 0i32, 64i32);
    F::HudSprite_SetUv(c, l_spr_3, 64i32, 128i32, 64i32, 128i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((59i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 16i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 64i32, 0i32, 64i32);
    F::HudSprite_SetUv(c, l_spr_3, 128i32, 192i32, 64i32, 128i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((60i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 16i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 64i32, 0i32, 64i32);
    F::HudSprite_SetUv(c, l_spr_3, 192i32, 256i32, 64i32, 128i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((61i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 16i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 64i32, 0i32, 64i32);
    F::HudSprite_SetUv(c, l_spr_3, 64i32, 128i32, 0i32, 64i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((62i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 16i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 64i32, 0i32, 64i32);
    F::HudSprite_SetUv(c, l_spr_3, 128i32, 192i32, 0i32, 64i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((64i32) as u32)).wrapping_mul(28));
    F::HudSprite_Show(c, l_spr_3, 1i32);
    F::HudSprite_InitPlain(c, l_spr_3, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8));
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 64i32, 0i32, 64i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((65i32) as u32)).wrapping_mul(28));
    F::HudSprite_Show(c, l_spr_3, 0i32);
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 17i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 19i32, 0i32, 22i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 19i32, 0i32, 22i32);
    F::HudSprite_Move(c, l_spr_3, 0i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((66i32) as u32)).wrapping_mul(28));
    F::HudSprite_Show(c, l_spr_3, 0i32);
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 17i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 19i32, 0i32, 22i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 19i32, 0i32, 22i32);
    F::HudSprite_Move(c, l_spr_3, 8i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((67i32) as u32)).wrapping_mul(28));
    F::HudSprite_Show(c, l_spr_3, 0i32);
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 17i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 19i32, 0i32, 22i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 19i32, 0i32, 22i32);
    F::HudSprite_Move(c, l_spr_3, 16i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((68i32) as u32)).wrapping_mul(28));
    F::HudSprite_Show(c, l_spr_3, 0i32);
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 17i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 19i32, 0i32, 22i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 19i32, 0i32, 22i32);
    F::HudSprite_Move(c, l_spr_3, 24i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((69i32) as u32)).wrapping_mul(28));
    F::HudSprite_Show(c, l_spr_3, 0i32);
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 17i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 19i32, 0i32, 22i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 19i32, 0i32, 22i32);
    F::HudSprite_Move(c, l_spr_3, 32i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((70i32) as u32)).wrapping_mul(28));
    F::HudSprite_Show(c, l_spr_3, 0i32);
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 17i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 19i32, 0i32, 22i32);
    F::HudSprite_SetUv(c, l_spr_3, 32i32, 51i32, 0i32, 22i32);
    F::HudSprite_Move(c, l_spr_3, 0i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((71i32) as u32)).wrapping_mul(28));
    F::HudSprite_Show(c, l_spr_3, 0i32);
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 17i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 19i32, 0i32, 22i32);
    F::HudSprite_SetUv(c, l_spr_3, 32i32, 51i32, 0i32, 22i32);
    F::HudSprite_Move(c, l_spr_3, 8i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((72i32) as u32)).wrapping_mul(28));
    F::HudSprite_Show(c, l_spr_3, 0i32);
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 17i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 19i32, 0i32, 22i32);
    F::HudSprite_SetUv(c, l_spr_3, 32i32, 51i32, 0i32, 22i32);
    F::HudSprite_Move(c, l_spr_3, 16i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((73i32) as u32)).wrapping_mul(28));
    F::HudSprite_Show(c, l_spr_3, 0i32);
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 17i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 19i32, 0i32, 22i32);
    F::HudSprite_SetUv(c, l_spr_3, 32i32, 51i32, 0i32, 22i32);
    F::HudSprite_Move(c, l_spr_3, 24i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((74i32) as u32)).wrapping_mul(28));
    F::HudSprite_Show(c, l_spr_3, 0i32);
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 17i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 19i32, 0i32, 22i32);
    F::HudSprite_SetUv(c, l_spr_3, 32i32, 51i32, 0i32, 22i32);
    F::HudSprite_Move(c, l_spr_3, 32i32, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((75i32) as u32)).wrapping_mul(28));
    F::HudSprite_Show(c, l_spr_3, 1i32);
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 18i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, (32i32).wrapping_neg(), 0i32, 0i32, 32i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 32i32, 0i32, 32i32);
    F::HudSprite_Move(c, l_spr_3, 10i32, (6i32).wrapping_neg());
    l_grp_4 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(56));
    { let v_ = 102i32; let a_ = (l_grp_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = 24i32; let a_ = (l_grp_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((4i32) as u32); let a_ = (l_grp_4).wrapping_add(32); c.m.w32(a_, v_); };
    { let v_ = { let a0_ = (((c.m.r32((l_grp_4).wrapping_add(32))).wrapping_mul(4u32)) as i32); F::Heap_Alloc(c, a0_, ((32i32) as u32), 0i32, 2i32) }; let a_ = (l_grp_4).wrapping_add(36); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_grp_4).wrapping_add(36)); let a2_ = (c.m.r32((l_grp_4).wrapping_add(32))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((43i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((44i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((45i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((46i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = fid::HudGauge_UpdateStatIcons; let a_ = (l_grp_4).wrapping_add(48); c.m.w32(a_, v_); };
    { let v_ = fid::HudGauge_DrawSprites; let a_ = (l_grp_4).wrapping_add(52); c.m.w32(a_, v_); };
    l_grp_4 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((10i32) as u32)).wrapping_mul(56));
    { let v_ = ((10i32) as u32); let a_ = (l_grp_4).wrapping_add(32); c.m.w32(a_, v_); };
    { let v_ = { let a0_ = (((c.m.r32((l_grp_4).wrapping_add(32))).wrapping_mul(4u32)) as i32); F::Heap_Alloc(c, a0_, ((32i32) as u32), 0i32, 2i32) }; let a_ = (l_grp_4).wrapping_add(36); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_grp_4).wrapping_add(36)); let a2_ = (c.m.r32((l_grp_4).wrapping_add(32))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((53i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((54i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((55i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((56i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((57i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((58i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((59i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((60i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((61i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((62i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = fid::HudGauge_UpdateAura; let a_ = (l_grp_4).wrapping_add(48); c.m.w32(a_, v_); };
    { let v_ = fid::HudGauge_DrawAura; let a_ = (l_grp_4).wrapping_add(52); c.m.w32(a_, v_); };
    l_grp_4 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(56));
    { let v_ = 29i32; let a_ = (l_grp_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = 46i32; let a_ = (l_grp_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((3i32) as u32); let a_ = (l_grp_4).wrapping_add(32); c.m.w32(a_, v_); };
    { let v_ = { let a0_ = (((c.m.r32((l_grp_4).wrapping_add(32))).wrapping_mul(4u32)) as i32); F::Heap_Alloc(c, a0_, ((32i32) as u32), 0i32, 2i32) }; let a_ = (l_grp_4).wrapping_add(36); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_grp_4).wrapping_add(36)); let a2_ = (c.m.r32((l_grp_4).wrapping_add(32))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((35i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((38i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((75i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = fid::HudGauge_UpdateBlast; let a_ = (l_grp_4).wrapping_add(48); c.m.w32(a_, v_); };
    { let v_ = fid::HudGauge_DrawBlast; let a_ = (l_grp_4).wrapping_add(52); c.m.w32(a_, v_); };
    l_grp_4 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(56));
    { let v_ = 69i32; let a_ = (l_grp_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = 9i32; let a_ = (l_grp_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((8i32) as u32); let a_ = (l_grp_4).wrapping_add(32); c.m.w32(a_, v_); };
    { let v_ = { let a0_ = (((c.m.r32((l_grp_4).wrapping_add(32))).wrapping_mul(4u32)) as i32); F::Heap_Alloc(c, a0_, ((32i32) as u32), 0i32, 2i32) }; let a_ = (l_grp_4).wrapping_add(36); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_grp_4).wrapping_add(36)); let a2_ = (c.m.r32((l_grp_4).wrapping_add(32))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = fid::HudGauge_UpdateHpTrail; let a_ = (l_grp_4).wrapping_add(48); c.m.w32(a_, v_); };
    { let v_ = fid::HudGauge_DrawHp; let a_ = (l_grp_4).wrapping_add(52); c.m.w32(a_, v_); };
    l_grp_4 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(56));
    { let v_ = 73i32; let a_ = (l_grp_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = 2i32; let a_ = (l_grp_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((6i32) as u32); let a_ = (l_grp_4).wrapping_add(32); c.m.w32(a_, v_); };
    { let v_ = { let a0_ = (((c.m.r32((l_grp_4).wrapping_add(32))).wrapping_mul(4u32)) as i32); F::Heap_Alloc(c, a0_, ((32i32) as u32), 0i32, 2i32) }; let a_ = (l_grp_4).wrapping_add(36); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_grp_4).wrapping_add(36)); let a2_ = (c.m.r32((l_grp_4).wrapping_add(32))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((10i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((11i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((12i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((13i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((14i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = fid::HudGauge_DrawHpBarCount; let a_ = (l_grp_4).wrapping_add(52); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_grp_4).wrapping_add(48); c.m.w32(a_, v_); };
    l_grp_4 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(56));
    { let v_ = (3i32).wrapping_neg(); let a_ = (l_grp_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = (3i32).wrapping_neg(); let a_ = (l_grp_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((5i32) as u32); let a_ = (l_grp_4).wrapping_add(32); c.m.w32(a_, v_); };
    { let v_ = { let a0_ = (((c.m.r32((l_grp_4).wrapping_add(32))).wrapping_mul(4u32)) as i32); F::Heap_Alloc(c, a0_, ((32i32) as u32), 0i32, 2i32) }; let a_ = (l_grp_4).wrapping_add(36); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_grp_4).wrapping_add(36)); let a2_ = (c.m.r32((l_grp_4).wrapping_add(32))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((65i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((66i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((67i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((68i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((69i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = fid::HudGauge_UpdateKiLamps; let a_ = (l_grp_4).wrapping_add(48); c.m.w32(a_, v_); };
    { let v_ = fid::HudGauge_DrawSprites; let a_ = (l_grp_4).wrapping_add(52); c.m.w32(a_, v_); };
    l_grp_4 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(56));
    { let v_ = (3i32).wrapping_neg(); let a_ = (l_grp_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = (3i32).wrapping_neg(); let a_ = (l_grp_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((5i32) as u32); let a_ = (l_grp_4).wrapping_add(32); c.m.w32(a_, v_); };
    { let v_ = { let a0_ = (((c.m.r32((l_grp_4).wrapping_add(32))).wrapping_mul(4u32)) as i32); F::Heap_Alloc(c, a0_, ((32i32) as u32), 0i32, 2i32) }; let a_ = (l_grp_4).wrapping_add(36); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_grp_4).wrapping_add(36)); let a2_ = (c.m.r32((l_grp_4).wrapping_add(32))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((70i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((71i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((72i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((73i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((74i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = fid::HudGauge_UpdateMaxPowerLamps; let a_ = (l_grp_4).wrapping_add(48); c.m.w32(a_, v_); };
    { let v_ = fid::HudGauge_DrawSprites; let a_ = (l_grp_4).wrapping_add(52); c.m.w32(a_, v_); };
    l_grp_4 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(56));
    { let v_ = ((15i32) as u32); let a_ = (l_grp_4).wrapping_add(32); c.m.w32(a_, v_); };
    { let v_ = 64i32; let a_ = (l_grp_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = 27i32; let a_ = (l_grp_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a0_ = (((c.m.r32((l_grp_4).wrapping_add(32))).wrapping_mul(4u32)) as i32); F::Heap_Alloc(c, a0_, ((32i32) as u32), 0i32, 2i32) }; let a_ = (l_grp_4).wrapping_add(36); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_grp_4).wrapping_add(36)); let a2_ = (c.m.r32((l_grp_4).wrapping_add(32))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((15i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((16i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((17i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((18i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((19i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((20i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((21i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((22i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((23i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((24i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((25i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((26i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((27i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((28i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((29i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((1i32) as u32); let a_ = (l_grp_4).wrapping_add(40); c.m.w32(a_, v_); };
    { let v_ = { let a0_ = (((c.m.r32((l_grp_4).wrapping_add(40))).wrapping_mul(4u32)) as i32); F::Heap_Alloc(c, a0_, ((32i32) as u32), 0i32, 2i32) }; let a_ = (l_grp_4).wrapping_add(44); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_grp_4).wrapping_add(44)); let a2_ = (c.m.r32((l_grp_4).wrapping_add(40))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(56)); let a_ = (c.m.r32((l_grp_4).wrapping_add(44))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = fid::HudGauge_DrawSprites; let a_ = (l_grp_4).wrapping_add(52); c.m.w32(a_, v_); };
    { let v_ = fid::HudGauge_UpdateKi; let a_ = (l_grp_4).wrapping_add(48); c.m.w32(a_, v_); };
    l_grp_4 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(56));
    { let v_ = 64i32; let a_ = (l_grp_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = 27i32; let a_ = (l_grp_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((5i32) as u32); let a_ = (l_grp_4).wrapping_add(32); c.m.w32(a_, v_); };
    { let v_ = { let a0_ = (((c.m.r32((l_grp_4).wrapping_add(32))).wrapping_mul(4u32)) as i32); F::Heap_Alloc(c, a0_, ((32i32) as u32), 0i32, 2i32) }; let a_ = (l_grp_4).wrapping_add(36); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_grp_4).wrapping_add(36)); let a2_ = (c.m.r32((l_grp_4).wrapping_add(32))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((30i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((31i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((32i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((33i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((34i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((1i32) as u32); let a_ = (l_grp_4).wrapping_add(40); c.m.w32(a_, v_); };
    { let v_ = { let a0_ = (((c.m.r32((l_grp_4).wrapping_add(40))).wrapping_mul(4u32)) as i32); F::Heap_Alloc(c, a0_, ((32i32) as u32), 0i32, 2i32) }; let a_ = (l_grp_4).wrapping_add(44); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_grp_4).wrapping_add(44)); let a2_ = (c.m.r32((l_grp_4).wrapping_add(40))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(56)); let a_ = (c.m.r32((l_grp_4).wrapping_add(44))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = fid::HudGauge_DrawSprites; let a_ = (l_grp_4).wrapping_add(52); c.m.w32(a_, v_); };
    { let v_ = fid::HudGauge_UpdateMaxPower; let a_ = (l_grp_4).wrapping_add(48); c.m.w32(a_, v_); };
    l_grp_4 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((11i32) as u32)).wrapping_mul(56));
    { let v_ = 16i32; let a_ = (l_grp_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = 3i32; let a_ = (l_grp_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((1i32) as u32); let a_ = (l_grp_4).wrapping_add(32); c.m.w32(a_, v_); };
    { let v_ = { let a0_ = (((c.m.r32((l_grp_4).wrapping_add(32))).wrapping_mul(4u32)) as i32); F::Heap_Alloc(c, a0_, ((32i32) as u32), 0i32, 2i32) }; let a_ = (l_grp_4).wrapping_add(36); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_grp_4).wrapping_add(36)); let a2_ = (c.m.r32((l_grp_4).wrapping_add(32))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((64i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = fid::HudGauge_DrawFace; let a_ = (l_grp_4).wrapping_add(52); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_grp_4).wrapping_add(48); c.m.w32(a_, v_); };
    l_grp_4 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(56));
    { let v_ = (2i32).wrapping_neg(); let a_ = (l_grp_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((6i32) as u32); let a_ = (l_grp_4).wrapping_add(32); c.m.w32(a_, v_); };
    { let v_ = 8i32; let a_ = (l_grp_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a0_ = (((c.m.r32((l_grp_4).wrapping_add(32))).wrapping_mul(4u32)) as i32); F::Heap_Alloc(c, a0_, ((32i32) as u32), 0i32, 2i32) }; let a_ = (l_grp_4).wrapping_add(36); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_grp_4).wrapping_add(36)); let a2_ = (c.m.r32((l_grp_4).wrapping_add(32))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((36i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((39i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((40i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((41i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((42i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_grp_4).wrapping_add(36))).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((8i32) as u32); let a_ = (l_grp_4).wrapping_add(40); c.m.w32(a_, v_); };
    { let v_ = { let a0_ = (((c.m.r32((l_grp_4).wrapping_add(40))).wrapping_mul(4u32)) as i32); F::Heap_Alloc(c, a0_, ((32i32) as u32), 0i32, 2i32) }; let a_ = (l_grp_4).wrapping_add(44); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_grp_4).wrapping_add(44)); let a2_ = (c.m.r32((l_grp_4).wrapping_add(40))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((10i32) as u32)).wrapping_mul(56)); let a_ = (c.m.r32((l_grp_4).wrapping_add(44))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(56)); let a_ = (c.m.r32((l_grp_4).wrapping_add(44))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(56)); let a_ = (c.m.r32((l_grp_4).wrapping_add(44))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(56)); let a_ = (c.m.r32((l_grp_4).wrapping_add(44))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(56)); let a_ = (c.m.r32((l_grp_4).wrapping_add(44))).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(56)); let a_ = (c.m.r32((l_grp_4).wrapping_add(44))).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(56)); let a_ = (c.m.r32((l_grp_4).wrapping_add(44))).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((11i32) as u32)).wrapping_mul(56)); let a_ = (c.m.r32((l_grp_4).wrapping_add(44))).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = fid::HudGauge_UpdatePanel; let a_ = (l_grp_4).wrapping_add(48); c.m.w32(a_, v_); };
    { let v_ = fid::HudGauge_DrawSprites; let a_ = (l_grp_4).wrapping_add(52); c.m.w32(a_, v_); };
    l_grp_4 = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(56));
    { let v_ = ((1i32) as u32); let a_ = (l_grp_4).wrapping_add(40); c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = (l_grp_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (l_grp_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a0_ = (((c.m.r32((l_grp_4).wrapping_add(40))).wrapping_mul(4u32)) as i32); F::Heap_Alloc(c, a0_, ((32i32) as u32), 0i32, 2i32) }; let a_ = (l_grp_4).wrapping_add(44); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_grp_4).wrapping_add(44)); let a2_ = (c.m.r32((l_grp_4).wrapping_add(40))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(56)); let a_ = (c.m.r32((l_grp_4).wrapping_add(44))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = fid::HudGauge_UpdateRoot; let a_ = (l_grp_4).wrapping_add(48); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_grp_4).wrapping_add(52); c.m.w32(a_, v_); };
    { let v_ = { let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.r32(a_) }; let a_ = p_out_1; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_Init", &PROBE_N, &[], true); }
    r_
}
