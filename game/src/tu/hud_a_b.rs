#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn HudTeam_GsBeginMask(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_GsBeginMask", &PROBE_N, &[], false); }
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
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_GsBeginMask", &PROBE_N, &[], true); }
    r_
}

pub fn HudTeam_GsEndMask(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_GsEndMask", &PROBE_N, &[], false); }
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
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_GsEndMask", &PROBE_N, &[], true); }
    r_
}

pub fn HudTeam_UpdateRoot(c: &mut Ctx, mut p_node_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_UpdateRoot", &PROBE_N, &[(p_node_1) as u32], false); }
    let mut l_slide_2: u32 = 0;
    let mut l_flipA_3: u32 = 0;
    let mut l_flipB_4: u32 = 0;
    let mut l_group_5: u32 = 0;
    let mut l_x_6: F32 = F32(0);
    let mut l_y_7: F32 = F32(0);
    let r_ = 'f: {
    l_slide_2 = ((c.m.r32(sym::gHudTeam)).wrapping_add(112)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(24));
    l_flipA_3 = ((c.m.r32(sym::gHudTeam)).wrapping_add(168)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(24));
    l_flipB_4 = ((c.m.r32(sym::gHudTeam)).wrapping_add(216)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(24));
    if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
        { let _ = F::Ramp_Step(c, l_slide_2); };
        if ((((({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(12); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            { let _ = { let a0_ = (c.m.r32(sym::gHudTeam)).wrapping_add(304); F::Ramp_Step(c, a0_) }; };
        }
    }
    { let a1_ = ((rt::f2i(rt::fmul(F32(c.m.r32((l_slide_2).wrapping_add(16))), F32(0xc3800000)))).wrapping_add(rt::f2i(rt::fmul({ let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(304)).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0xc3800000))))).wrapping_add(12i32); let a2_ = ((rt::f2i(rt::fmul(F32(c.m.r32((l_slide_2).wrapping_add(16))), F32(0xc3600000)))).wrapping_add(rt::f2i(rt::fmul({ let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(304)).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0xc3600000))))).wrapping_add(66i32); F::HudNode_SetPos(c, p_node_1, a1_, a2_) };
    if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
        if ((F::Ramp_Step(c, l_flipA_3)) != 0) {
            F::Ramp_Stop(c, l_flipA_3);
        }
        if ((F::Ramp_Step(c, l_flipB_4)) != 0) {
            F::Ramp_Stop(c, l_flipB_4);
            { let v_ = 1i32; let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(264)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(264)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
    }
    if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_flipA_3).wrapping_add(16)))) as i32)) != 0) {
        l_x_6 = rt::fneg(F32(c.m.r32((l_flipA_3).wrapping_add(16))));
        l_y_7 = F32(c.m.r32((l_flipA_3).wrapping_add(16)));
    } else {
        if (((rt::flt(F32(c.m.r32((l_flipA_3).wrapping_add(16))), F32(0x00000000)) as i32)) != 0) {
            l_y_7 = F32(c.m.r32((l_flipA_3).wrapping_add(16)));
            l_x_6 = F32(c.m.r32((l_flipA_3).wrapping_add(16)));
        } else {
            l_y_7 = F32(0x00000000);
            l_x_6 = F32(0x00000000);
        }
    }
    { let r_ = F32(0x43000000); let n_ = rt::fmul(l_x_6, r_); l_x_6 = n_; };
    { let r_ = F32(0x42000000); let n_ = rt::fmul(l_y_7, r_); l_y_7 = n_; };
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(56)); F::HudNode_SetPos(c, a0_, rt::f2i(l_x_6), rt::f2i(l_y_7)) };
    if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_flipB_4).wrapping_add(16)))) as i32)) != 0) {
        l_x_6 = rt::fneg(F32(c.m.r32((l_flipB_4).wrapping_add(16))));
        l_y_7 = F32(c.m.r32((l_flipB_4).wrapping_add(16)));
    } else {
        if (((rt::flt(F32(c.m.r32((l_flipB_4).wrapping_add(16))), F32(0x00000000)) as i32)) != 0) {
            l_y_7 = F32(c.m.r32((l_flipB_4).wrapping_add(16)));
            l_x_6 = F32(c.m.r32((l_flipB_4).wrapping_add(16)));
        } else {
            l_y_7 = F32(0x00000000);
            l_x_6 = F32(0x00000000);
        }
    }
    l_group_5 = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(56));
    { let r_ = F32(0x43000000); let n_ = rt::fmul(l_x_6, r_); l_x_6 = n_; };
    { let r_ = F32(0x42000000); let n_ = rt::fmul(l_y_7, r_); l_y_7 = n_; };
    F::HudNode_SetPos(c, l_group_5, rt::f2i(l_x_6), rt::f2i(l_y_7));
    if (({ let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(264)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != 0) {
        F::HudNode_Show(c, l_group_5, 0i32);
    } else {
        F::HudNode_Show(c, l_group_5, 1i32);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_UpdateRoot", &PROBE_N, &[], true); }
    r_
}

pub fn HudTeam_UpdateFaceA(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_UpdateFaceA", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (((c.m.r32(sym::gHudTeam)).wrapping_add(32)).wrapping_add(8)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= (2i32)) as i32)) != 0) {
        { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(56)); F::HudNode_SetPos(c, a0_, 14i32, 0i32) };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(56)); F::HudNode_Show(c, a0_, 1i32) };
    } else {
        { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(56)); F::HudNode_SetPos(c, a0_, 0i32, 0i32) };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(56)); F::HudNode_Show(c, a0_, 0i32) };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(48)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_UpdateFaceA", &PROBE_N, &[], true); }
    r_
}

pub fn HudTeam_UpdateFaceB(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_UpdateFaceB", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (((c.m.r32(sym::gHudTeam)).wrapping_add(32)).wrapping_add(8)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= (2i32)) as i32)) != 0) {
        { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(56)); F::HudNode_SetPos(c, a0_, 14i32, 0i32) };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(56)); F::HudNode_Show(c, a0_, 1i32) };
    } else {
        { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(56)); F::HudNode_SetPos(c, a0_, 0i32, 0i32) };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(56)); F::HudNode_Show(c, a0_, 0i32) };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(48)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_UpdateFaceB", &PROBE_N, &[], true); }
    r_
}

pub fn HudTeam_UpdateHpA(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_UpdateHpA", &PROBE_N, &[], false); }
    let mut l_spr_1: u32 = 0;
    let mut l_rate_2: F32 = F32(0);
    let r_ = 'f: {
    l_spr_1 = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(28));
    l_rate_2 = rt::fdiv(rt::i2f({ let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(16)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }), rt::i2f({ let a_ = (((c.m.r32(sym::gHudTeam)).wrapping_add(16)).wrapping_add(8)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }));
    F::HudSprite_SetRect(c, l_spr_1, 0i32, 32i32, 0i32, 16i32);
    F::HudSprite_Move(c, l_spr_1, rt::f2i(rt::fmul(l_rate_2, F32(0x41f00000))), 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_UpdateHpA", &PROBE_N, &[], true); }
    r_
}

pub fn HudTeam_UpdateHpB(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_UpdateHpB", &PROBE_N, &[], false); }
    let mut l_spr_1: u32 = 0;
    let mut l_rate_2: F32 = F32(0);
    let r_ = 'f: {
    l_spr_1 = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(28));
    l_rate_2 = rt::fdiv(rt::i2f({ let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(288)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }), rt::i2f({ let a_ = (((c.m.r32(sym::gHudTeam)).wrapping_add(288)).wrapping_add(8)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }));
    F::HudSprite_SetRect(c, l_spr_1, 0i32, 32i32, 0i32, 16i32);
    F::HudSprite_Move(c, l_spr_1, rt::f2i(rt::fmul(l_rate_2, F32(0x41f00000))), 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_UpdateHpB", &PROBE_N, &[], true); }
    r_
}

pub fn HudTeam_DrawHpA(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_DrawHpA", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(28)); F::HudSprite_DrawAlphaClear(c, a0_) };
    F::HudGfx_CallBegin(c, fid::HudTeam_GsBeginMask);
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudTeam); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 1i32) };
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudTeam); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
    F::HudGfx_CallEnd(c, fid::HudTeam_GsEndMask);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_DrawHpA", &PROBE_N, &[], true); }
    r_
}

pub fn HudTeam_DrawHpB(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_DrawHpB", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(28)); F::HudSprite_DrawAlphaClear(c, a0_) };
    F::HudGfx_CallBegin(c, fid::HudTeam_GsBeginMask);
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudTeam); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 1i32) };
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudTeam); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
    F::HudGfx_CallEnd(c, fid::HudTeam_GsEndMask);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_DrawHpB", &PROBE_N, &[], true); }
    r_
}

pub fn HudTeam_UpdateGauge(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_UpdateGauge", &PROBE_N, &[], false); }
    let mut l_full_1: i32 = 0;
    let mut l_gauge_2: i32 = 0;
    let mut l_spr_3: u32 = 0;
    let mut l_flash_4: u32 = 0;
    let mut l_rate_5: F32 = F32(0);
    let mut l_done_6: i32 = 0;
    let r_ = 'f: {
    l_full_1 = 100000i32;
    l_gauge_2 = { let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(32)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(28));
    l_flash_4 = ((c.m.r32(sym::gHudTeam)).wrapping_add(56)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(24));
    l_rate_5 = rt::fdiv(rt::i2f(l_gauge_2), rt::i2f(l_full_1));
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 32i32, 0i32, 16i32);
    F::HudSprite_Move(c, l_spr_3, rt::f2i(rt::fmul(l_rate_5, F32(0x41f00000))), 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(28));
    if (((((l_gauge_2) == (l_full_1)) as i32)) != 0) {
        F::HudSprite_Show(c, l_spr_3, 1i32);
        if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
            l_done_6 = F::Ramp_Step(c, l_flash_4);
        } else {
            l_done_6 = 0i32;
        }
        if ((l_done_6) != 0) {
            if (((rt::feq(F32(c.m.r32((l_flash_4).wrapping_add(16))), F32(0x00000000)) as i32)) != 0) {
                F::Ramp_Start(c, l_flash_4, F32(0x3f800000), F32(0x00000000), F32(0x3f800000));
            } else {
                F::Ramp_Start(c, l_flash_4, F32(0x3f800000), F32(0x3f800000), F32(0x00000000));
            }
        }
        { let a4_ = ((((rt::f2i(rt::fmul(F32(c.m.r32((l_flash_4).wrapping_add(16))), F32(0x43000000)))) as u8)) as i32); F::HudSprite_SetColor(c, l_spr_3, 128i32, 128i32, 128i32, a4_) };
    } else {
        F::HudSprite_Show(c, l_spr_3, 0i32);
        F::HudSprite_SetColor(c, l_spr_3, 128i32, 128i32, 128i32, 128i32);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_UpdateGauge", &PROBE_N, &[], true); }
    r_
}

pub fn HudTeam_DrawGauge(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_DrawGauge", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(28)); F::HudSprite_DrawAlphaClear(c, a0_) };
    F::HudGfx_CallBegin(c, fid::HudTeam_GsBeginMask);
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudTeam); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 1i32) };
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudTeam); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
    F::HudGfx_CallEnd(c, fid::HudTeam_GsEndMask);
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudTeam); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_DrawGauge", &PROBE_N, &[], true); }
    r_
}

pub fn HudTeam_DrawSprites(c: &mut Ctx, mut p_node_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_DrawSprites", &PROBE_N, &[(p_node_1) as u32], false); }
    let mut l_i_2: u32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = ((0i32) as u32);
        'l1: loop {
            if !(((((l_i_2) < (c.m.r32((p_node_1).wrapping_add(32)))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = { let a_ = (c.m.r32((p_node_1).wrapping_add(36))).wrapping_add((l_i_2).wrapping_mul(4)); c.m.r32(a_) }; let a1_ = { let a_ = c.m.r32(sym::gHudTeam); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_DrawSprites", &PROBE_N, &[], true); }
    r_
}

pub fn HudTeam_ClearFaceTexMark(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_ClearFaceTexMark", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_tex_2: u32 = 0;
    let r_ = 'f: {
    if ((((({ let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(48)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        l_tex_2 = { let a_ = ({ let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(48)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(16); c.m.r32(a_) };
        if (((((l_tex_2) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = 0i32; let a_ = (l_tex_2).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_ClearFaceTexMark", &PROBE_N, &[], true); }
    r_
}

pub fn HudTeam_DrawFaceA(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_DrawFaceA", &PROBE_N, &[], false); }
    let mut l_spr_1: u32 = 0;
    let r_ = 'f: {
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudTeam); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
    if ((((({ let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= (0i32)) as i32)) != 0) {
        { let v_ = { let a_ = ({ let a0_ = { let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(12); (c.m.r32(a_) as i32) }; let a1_ = { let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::BattleSide_GetMember(c, a0_, a1_) }).wrapping_add(88); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(48)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        if ((((({ let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(48)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
            { let a0_ = { let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(12); (c.m.r32(a_) as i32) }; F::HudTeam_ClearFaceTexMark(c, a0_) };
            l_spr_1 = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(28));
            { let a1_ = { let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(48)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_spr_1, a1_, 0i32, 0i32) };
            F::HudSprite_Move(c, l_spr_1, (10i32).wrapping_neg(), (11i32).wrapping_neg());
            { let a1_ = { let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(48)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::HudSprite_DrawAt(c, l_spr_1, a1_, ((((0i32) as i64 as u64)) as u8), 11264i32, 11472i32) };
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_DrawFaceA", &PROBE_N, &[], true); }
    r_
}

pub fn HudTeam_DrawFaceB(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_DrawFaceB", &PROBE_N, &[], false); }
    let mut l_spr_1: u32 = 0;
    let r_ = 'f: {
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudTeam); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
    if ((((({ let a_ = (((c.m.r32(sym::gHudTeam)).wrapping_add(272)).wrapping_add(8)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= (0i32)) as i32)) != 0) {
        { let v_ = { let a_ = ({ let a0_ = { let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(12); (c.m.r32(a_) as i32) }; let a1_ = { let a_ = (((c.m.r32(sym::gHudTeam)).wrapping_add(272)).wrapping_add(8)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::BattleSide_GetMember(c, a0_, a1_) }).wrapping_add(88); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(48)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        if ((((({ let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(48)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
            { let a0_ = { let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(12); (c.m.r32(a_) as i32) }; F::HudTeam_ClearFaceTexMark(c, a0_) };
            l_spr_1 = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(28));
            { let a1_ = { let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(48)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_spr_1, a1_, 0i32, 0i32) };
            F::HudSprite_Move(c, l_spr_1, (10i32).wrapping_neg(), (11i32).wrapping_neg());
            { let a1_ = { let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(48)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::HudSprite_DrawAt(c, l_spr_1, a1_, ((((0i32) as i64 as u64)) as u8), 11264i32, 11472i32) };
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_DrawFaceB", &PROBE_N, &[], true); }
    r_
}

pub fn HudTeam_SelectSide(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_SelectSide", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_mirror_2: i32 = 0;
    let r_ = 'f: {
    l_mirror_2 = (((p_side_1) != (0i32)) as i32);
    { let v_ = p_side_1; let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    F::Port_GsMarker(c, (if ((p_side_1) != 0) { 17i32 } else { 16i32 }));
    { let v_ = (((({ let a_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(56)); c.m.r32(a_) }) & ((((!(2i32))) as u32)))) | ((((l_mirror_2).wrapping_shl((1i32) as u32)) as u32))); let a_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(56)); c.m.w32(a_, v_); };
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(28)); F::HudSprite_SetMirror(c, a0_, l_mirror_2) };
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(28)); F::HudSprite_SetMirror(c, a0_, l_mirror_2) };
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(56)); let a1_ = { let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(104)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::HudNode_Show(c, a0_, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_SelectSide", &PROBE_N, &[], true); }
    r_
}

pub fn HudTeam_SetTargetHp(c: &mut Ctx, mut p_side_1: i32, mut p_hp_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_SetTargetHp", &PROBE_N, &[(p_side_1) as u32, (p_hp_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_hp_2; let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(16)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_SetTargetHp", &PROBE_N, &[], true); }
    r_
}

pub fn HudTeam_SetTargetHpMax(c: &mut Ctx, mut p_side_1: i32, mut p_hpMax_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_SetTargetHpMax", &PROBE_N, &[(p_side_1) as u32, (p_hpMax_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_hpMax_2; let a_ = (((c.m.r32(sym::gHudTeam)).wrapping_add(16)).wrapping_add(8)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_SetTargetHpMax", &PROBE_N, &[], true); }
    r_
}

pub fn HudTeam_SetSwitchGauge(c: &mut Ctx, mut p_side_1: i32, mut p_gauge_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_SetSwitchGauge", &PROBE_N, &[(p_side_1) as u32, (p_gauge_2) as u32], false); }
    let r_ = 'f: {
    if ((((((((({ let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(32)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != (100000i32)) as i32)) != 0) && (((((p_gauge_2) == (100000i32)) as i32)) != 0)) as i32)) != 0) {
        { let a0_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(56)).wrapping_add((((p_side_1) as u32)).wrapping_mul(24)); F::Ramp_Start(c, a0_, F32(0x3f800000), F32(0x3f800000), F32(0x00000000)) };
    }
    { let v_ = p_gauge_2; let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(32)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_SetSwitchGauge", &PROBE_N, &[], true); }
    r_
}

pub fn HudTeam_SetTarget(c: &mut Ctx, mut p_side_1: i32, mut p_member_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_SetTarget", &PROBE_N, &[(p_side_1) as u32, (p_member_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_member_2; let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(272)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_SetTarget", &PROBE_N, &[], true); }
    r_
}

pub fn HudTeam_SetReserveCount(c: &mut Ctx, mut p_side_1: i32, mut p_count_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_SetReserveCount", &PROBE_N, &[(p_side_1) as u32, (p_count_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_count_2; let a_ = (((c.m.r32(sym::gHudTeam)).wrapping_add(32)).wrapping_add(8)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = (((p_count_2) != (0i32)) as i32); let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(104)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_SetReserveCount", &PROBE_N, &[], true); }
    r_
}

pub fn HudTeam_SlideOut(c: &mut Ctx, mut p_seconds_1: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_SlideOut", &PROBE_N, &[(p_seconds_1).0], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32(sym::gHudTeam)).wrapping_add(304); F::Ramp_Start(c, a0_, p_seconds_1, F32(0x00000000), F32(0x3f800000)) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_SlideOut", &PROBE_N, &[], true); }
    r_
}

pub fn HudTeam_SlideIn(c: &mut Ctx, mut p_seconds_1: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_SlideIn", &PROBE_N, &[(p_seconds_1).0], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32(sym::gHudTeam)).wrapping_add(304); F::Ramp_Start(c, a0_, p_seconds_1, F32(0x3f800000), F32(0x00000000)) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_SlideIn", &PROBE_N, &[], true); }
    r_
}

pub fn HudTeam_SetSwitching(c: &mut Ctx, mut p_side_1: i32, mut p_on_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_SetSwitching", &PROBE_N, &[(p_side_1) as u32, (p_on_2) as u32], false); }
    let r_ = 'f: {
    if ((((({ let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(160)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != (p_on_2)) as i32)) != 0) {
        if ((p_on_2) != 0) {
            { let a0_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(112)).wrapping_add((((p_side_1) as u32)).wrapping_mul(24)); F::Ramp_Start(c, a0_, F32(0x3e999999), F32(0x00000000), F32(0x3f800000)) };
        } else {
            { let a0_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(112)).wrapping_add((((p_side_1) as u32)).wrapping_mul(24)); F::Ramp_Start(c, a0_, F32(0x3e999999), F32(0x3f800000), F32(0x00000000)) };
        }
    }
    { let v_ = p_on_2; let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(160)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_SetSwitching", &PROBE_N, &[], true); }
    r_
}

pub fn HudTeam_StartFlipNext(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_StartFlipNext", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_flipA_2: u32 = 0;
    let mut l_flipB_3: u32 = 0;
    let r_ = 'f: {
    l_flipA_2 = ((c.m.r32(sym::gHudTeam)).wrapping_add(168)).wrapping_add((((p_side_1) as u32)).wrapping_mul(24));
    l_flipB_3 = ((c.m.r32(sym::gHudTeam)).wrapping_add(216)).wrapping_add((((p_side_1) as u32)).wrapping_mul(24));
    F::Ramp_Start(c, l_flipA_2, F32(0x3e800000), F32(0x3f800000), F32(0x00000000));
    F::Ramp_Start(c, l_flipB_3, F32(0x3e800000), F32(0x00000000), F32(0xbf800000));
    { let v_ = { let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(272)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (((c.m.r32(sym::gHudTeam)).wrapping_add(272)).wrapping_add(8)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(16)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(288)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a_ = (((c.m.r32(sym::gHudTeam)).wrapping_add(16)).wrapping_add(8)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (((c.m.r32(sym::gHudTeam)).wrapping_add(288)).wrapping_add(8)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(264)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_StartFlipNext", &PROBE_N, &[], true); }
    r_
}

pub fn HudTeam_StartFlipPrev(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_StartFlipPrev", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_flipA_2: u32 = 0;
    let mut l_flipB_3: u32 = 0;
    let r_ = 'f: {
    l_flipA_2 = ((c.m.r32(sym::gHudTeam)).wrapping_add(168)).wrapping_add((((p_side_1) as u32)).wrapping_mul(24));
    l_flipB_3 = ((c.m.r32(sym::gHudTeam)).wrapping_add(216)).wrapping_add((((p_side_1) as u32)).wrapping_mul(24));
    F::Ramp_Start(c, l_flipA_2, F32(0x3e800000), F32(0xbf800000), F32(0x00000000));
    F::Ramp_Start(c, l_flipB_3, F32(0x3e800000), F32(0x00000000), F32(0x3f800000));
    { let v_ = { let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(272)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (((c.m.r32(sym::gHudTeam)).wrapping_add(272)).wrapping_add(8)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(16)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(288)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a_ = (((c.m.r32(sym::gHudTeam)).wrapping_add(16)).wrapping_add(8)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (((c.m.r32(sym::gHudTeam)).wrapping_add(288)).wrapping_add(8)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(264)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_StartFlipPrev", &PROBE_N, &[], true); }
    r_
}

pub fn HudTeam_Init(c: &mut Ctx, mut p_out_1: u32, mut p_res_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_Init", &PROBE_N, &[(p_out_1) as u32, (p_res_2) as u32], false); }
    let mut l_spr_3: u32 = 0;
    let mut l_node_4: u32 = 0;
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, ((328u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gHudTeam; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gHudTeam); F::memset(c, a0_, 0i32, 328u32) }; };
    { let v_ = F::Heap_Alloc(c, (((((12i32) as u32)).wrapping_mul(28u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.w32(a_, v_); };
    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }; F::memset(c, a0_, 0i32, (((12i32) as u32)).wrapping_mul(28u32)) }; };
    { let v_ = F::Heap_Alloc(c, (((((9i32) as u32)).wrapping_mul(56u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.w32(a_, v_); };
    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }; F::memset(c, a0_, 0i32, (((9i32) as u32)).wrapping_mul(56u32)) }; };
    { let v_ = p_res_2; let a_ = c.m.r32(sym::gHudTeam); c.m.w32(a_, v_); };
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28));
    F::HudSprite_Show(c, l_spr_3, 1i32);
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 23i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 94i32, 0i32, 20i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 94i32, 0i32, 20i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(28));
    F::HudSprite_Show(c, l_spr_3, 1i32);
    F::HudSprite_InitPlain(c, l_spr_3, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8));
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 32i32, 0i32, 32i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(28));
    F::HudSprite_Show(c, l_spr_3, 1i32);
    F::HudSprite_InitPlain(c, l_spr_3, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8));
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 32i32, 0i32, 32i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(28));
    F::HudSprite_Show(c, l_spr_3, 1i32);
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 24i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 32i32, 0i32, 16i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 32i32, 16i32, 32i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(28));
    F::HudSprite_Show(c, l_spr_3, 1i32);
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 24i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 32i32, 0i32, 16i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 32i32, 0i32, 16i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(28));
    F::HudSprite_Show(c, l_spr_3, 1i32);
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 24i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 32i32, 0i32, 16i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 32i32, 16i32, 32i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(28));
    F::HudSprite_Show(c, l_spr_3, 1i32);
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 24i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 32i32, 0i32, 16i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 32i32, 0i32, 16i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(28));
    F::HudSprite_Show(c, l_spr_3, 1i32);
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 24i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 32i32, 0i32, 16i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 32i32, 16i32, 32i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(28));
    F::HudSprite_Show(c, l_spr_3, 1i32);
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 24i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 32i32, 0i32, 16i32);
    F::HudSprite_SetUv(c, l_spr_3, 64i32, 96i32, 0i32, 16i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(28));
    F::HudSprite_Show(c, l_spr_3, 0i32);
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 17i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 48i32, 0i32, 16i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 48i32, 32i32, 48i32);
    F::HudSprite_Move(c, l_spr_3, (3i32).wrapping_neg(), (3i32).wrapping_neg());
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((10i32) as u32)).wrapping_mul(28));
    F::HudSprite_Show(c, l_spr_3, 1i32);
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 23i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 30i32, 0i32, 20i32);
    F::HudSprite_SetUv(c, l_spr_3, 96i32, 126i32, 0i32, 20i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((11i32) as u32)).wrapping_mul(28));
    F::HudSprite_Show(c, l_spr_3, 1i32);
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 24i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 16i32, 0i32, 16i32);
    F::HudSprite_SetUv(c, l_spr_3, 64i32, 80i32, 16i32, 32i32);
    F::HudSprite_Move(c, l_spr_3, (1i32).wrapping_neg(), 3i32);
    l_node_4 = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(56));
    { let v_ = 46i32; let a_ = (l_node_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = 4i32; let a_ = (l_node_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((2i32) as u32); let a_ = (l_node_4).wrapping_add(32); c.m.w32(a_, v_); };
    { let v_ = F::Heap_Alloc(c, (((((2i32) as u32)).wrapping_mul(4u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (l_node_4).wrapping_add(36); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_node_4).wrapping_add(36)); let a2_ = (c.m.r32((l_node_4).wrapping_add(32))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = fid::HudTeam_UpdateHpA; let a_ = (l_node_4).wrapping_add(48); c.m.w32(a_, v_); };
    { let v_ = fid::HudTeam_DrawHpA; let a_ = (l_node_4).wrapping_add(52); c.m.w32(a_, v_); };
    l_node_4 = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(56));
    { let v_ = 46i32; let a_ = (l_node_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = 4i32; let a_ = (l_node_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((2i32) as u32); let a_ = (l_node_4).wrapping_add(32); c.m.w32(a_, v_); };
    { let v_ = F::Heap_Alloc(c, (((((2i32) as u32)).wrapping_mul(4u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (l_node_4).wrapping_add(36); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_node_4).wrapping_add(36)); let a2_ = (c.m.r32((l_node_4).wrapping_add(32))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = fid::HudTeam_UpdateHpB; let a_ = (l_node_4).wrapping_add(48); c.m.w32(a_, v_); };
    { let v_ = fid::HudTeam_DrawHpB; let a_ = (l_node_4).wrapping_add(52); c.m.w32(a_, v_); };
    l_node_4 = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(56));
    { let v_ = 41i32; let a_ = (l_node_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((3i32) as u32); let a_ = (l_node_4).wrapping_add(32); c.m.w32(a_, v_); };
    { let v_ = 12i32; let a_ = (l_node_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = F::Heap_Alloc(c, (((((3i32) as u32)).wrapping_mul(4u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (l_node_4).wrapping_add(36); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_node_4).wrapping_add(36)); let a2_ = (c.m.r32((l_node_4).wrapping_add(32))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = fid::HudTeam_UpdateGauge; let a_ = (l_node_4).wrapping_add(48); c.m.w32(a_, v_); };
    { let v_ = fid::HudTeam_DrawGauge; let a_ = (l_node_4).wrapping_add(52); c.m.w32(a_, v_); };
    l_node_4 = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(56));
    { let v_ = ((2i32) as u32); let a_ = (l_node_4).wrapping_add(32); c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = (l_node_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (l_node_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = F::Heap_Alloc(c, (((((2i32) as u32)).wrapping_mul(4u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (l_node_4).wrapping_add(36); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_node_4).wrapping_add(36)); let a2_ = (c.m.r32((l_node_4).wrapping_add(32))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((10i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((11i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = fid::HudTeam_DrawSprites; let a_ = (l_node_4).wrapping_add(52); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_node_4).wrapping_add(48); c.m.w32(a_, v_); };
    l_node_4 = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(56));
    { let v_ = 14i32; let a_ = (l_node_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((2i32) as u32); let a_ = (l_node_4).wrapping_add(32); c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = (l_node_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = F::Heap_Alloc(c, (((((2i32) as u32)).wrapping_mul(4u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (l_node_4).wrapping_add(36); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_node_4).wrapping_add(36)); let a2_ = (c.m.r32((l_node_4).wrapping_add(32))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((2i32) as u32); let a_ = (l_node_4).wrapping_add(40); c.m.w32(a_, v_); };
    { let v_ = F::Heap_Alloc(c, (((((2i32) as u32)).wrapping_mul(4u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (l_node_4).wrapping_add(44); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_node_4).wrapping_add(44)); let a2_ = (c.m.r32((l_node_4).wrapping_add(40))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(56)); let a_ = (c.m.r32((l_node_4).wrapping_add(44))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(56)); let a_ = (c.m.r32((l_node_4).wrapping_add(44))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = fid::HudTeam_UpdateFaceA; let a_ = (l_node_4).wrapping_add(48); c.m.w32(a_, v_); };
    { let v_ = fid::HudTeam_DrawFaceA; let a_ = (l_node_4).wrapping_add(52); c.m.w32(a_, v_); };
    l_node_4 = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(56));
    { let v_ = 14i32; let a_ = (l_node_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((2i32) as u32); let a_ = (l_node_4).wrapping_add(32); c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = (l_node_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = F::Heap_Alloc(c, (((((2i32) as u32)).wrapping_mul(4u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (l_node_4).wrapping_add(36); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_node_4).wrapping_add(36)); let a2_ = (c.m.r32((l_node_4).wrapping_add(32))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((2i32) as u32); let a_ = (l_node_4).wrapping_add(40); c.m.w32(a_, v_); };
    { let v_ = F::Heap_Alloc(c, (((((2i32) as u32)).wrapping_mul(4u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (l_node_4).wrapping_add(44); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_node_4).wrapping_add(44)); let a2_ = (c.m.r32((l_node_4).wrapping_add(40))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(56)); let a_ = (c.m.r32((l_node_4).wrapping_add(44))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(56)); let a_ = (c.m.r32((l_node_4).wrapping_add(44))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = fid::HudTeam_UpdateFaceB; let a_ = (l_node_4).wrapping_add(48); c.m.w32(a_, v_); };
    { let v_ = fid::HudTeam_DrawFaceB; let a_ = (l_node_4).wrapping_add(52); c.m.w32(a_, v_); };
    l_node_4 = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(56));
    { let v_ = ((2i32) as u32); let a_ = (l_node_4).wrapping_add(40); c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = (l_node_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (l_node_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = F::Heap_Alloc(c, (((((2i32) as u32)).wrapping_mul(4u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (l_node_4).wrapping_add(44); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_node_4).wrapping_add(44)); let a2_ = (c.m.r32((l_node_4).wrapping_add(40))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(56)); let a_ = (c.m.r32((l_node_4).wrapping_add(44))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(56)); let a_ = (c.m.r32((l_node_4).wrapping_add(44))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = fid::HudTeam_DrawSprites; let a_ = (l_node_4).wrapping_add(52); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_node_4).wrapping_add(48); c.m.w32(a_, v_); };
    l_node_4 = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(56));
    { let v_ = ((2i32) as u32); let a_ = (l_node_4).wrapping_add(40); c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = (l_node_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (l_node_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = F::Heap_Alloc(c, (((((2i32) as u32)).wrapping_mul(4u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (l_node_4).wrapping_add(44); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_node_4).wrapping_add(44)); let a2_ = (c.m.r32((l_node_4).wrapping_add(40))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(56)); let a_ = (c.m.r32((l_node_4).wrapping_add(44))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(56)); let a_ = (c.m.r32((l_node_4).wrapping_add(44))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = fid::HudTeam_DrawSprites; let a_ = (l_node_4).wrapping_add(52); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_node_4).wrapping_add(48); c.m.w32(a_, v_); };
    l_node_4 = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(56));
    { let v_ = 66i32; let a_ = (l_node_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = 12i32; let a_ = (l_node_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((2i32) as u32); let a_ = (l_node_4).wrapping_add(40); c.m.w32(a_, v_); };
    { let v_ = F::Heap_Alloc(c, (((((2i32) as u32)).wrapping_mul(4u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (l_node_4).wrapping_add(44); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_node_4).wrapping_add(44)); let a2_ = (c.m.r32((l_node_4).wrapping_add(40))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(56)); let a_ = (c.m.r32((l_node_4).wrapping_add(44))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(56)); let a_ = (c.m.r32((l_node_4).wrapping_add(44))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = fid::HudTeam_DrawSprites; let a_ = (l_node_4).wrapping_add(52); c.m.w32(a_, v_); };
    { let v_ = fid::HudTeam_UpdateRoot; let a_ = (l_node_4).wrapping_add(48); c.m.w32(a_, v_); };
    { let v_ = { let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }; let a_ = p_out_1; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_Init", &PROBE_N, &[], true); }
    r_
}

pub fn HudTeam_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
    }
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (9i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((l_i_1) as u32)).wrapping_mul(56))).wrapping_add(36); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
                    { let a0_ = { let a_ = (({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((l_i_1) as u32)).wrapping_mul(56))).wrapping_add(36); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
                }
                if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((l_i_1) as u32)).wrapping_mul(56))).wrapping_add(44); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
                    { let a0_ = { let a_ = (({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((l_i_1) as u32)).wrapping_mul(56))).wrapping_add(44); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gHudTeam)).wrapping_add(8); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
    }
    if (((((c.m.r32(sym::gHudTeam)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gHudTeam); F::Heap_Free(c, a0_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_Term", &PROBE_N, &[], true); }
    r_
}

pub fn HudTeam_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_Reset", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(16)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (((c.m.r32(sym::gHudTeam)).wrapping_add(16)).wrapping_add(8)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(32)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (((c.m.r32(sym::gHudTeam)).wrapping_add(32)).wrapping_add(8)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(104)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(160)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(264)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(272)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (((c.m.r32(sym::gHudTeam)).wrapping_add(272)).wrapping_add(8)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(288)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (((c.m.r32(sym::gHudTeam)).wrapping_add(288)).wrapping_add(8)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let _ = { let a0_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(56)).wrapping_add((((l_i_1) as u32)).wrapping_mul(24)); F::memset(c, a0_, 0i32, 24u32) }; };
                { let _ = { let a0_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(112)).wrapping_add((((l_i_1) as u32)).wrapping_mul(24)); F::memset(c, a0_, 0i32, 24u32) }; };
                { let _ = { let a0_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(168)).wrapping_add((((l_i_1) as u32)).wrapping_mul(24)); F::memset(c, a0_, 0i32, 24u32) }; };
                { let _ = { let a0_ = ((c.m.r32(sym::gHudTeam)).wrapping_add(216)).wrapping_add((((l_i_1) as u32)).wrapping_mul(24)); F::memset(c, a0_, 0i32, 24u32) }; };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    { let _ = { let a0_ = (c.m.r32(sym::gHudTeam)).wrapping_add(304); F::memset(c, a0_, 0i32, 24u32) }; };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTeam_Reset", &PROBE_N, &[], true); }
    r_
}
