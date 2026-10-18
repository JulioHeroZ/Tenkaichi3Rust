#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn ObjGs_AddFrameMaskOpaque(c: &mut Ctx, mut p_mask_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjGs_AddFrameMaskOpaque", &PROBE_N, &[(p_mask_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 80);
        { let v_ = ((((268435456i32) | (4i32))) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((((1342177280i32) | (4i32))) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((((32768i32) | (3i32))) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = (((((((((((if (((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0) { 0i32 } else { 112i32 })) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((((p_mask_1) | (4278190080u32))) as u64)).wrapping_shl((32i32) as u32)))) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = ((((((((((((if (((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0) { 0i32 } else { 112i32 })) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((((p_mask_1) | (4278190080u32))) as u64)).wrapping_shl((32i32) as u32)))).wrapping_shr((32i32) as u32)) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((76i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        { let v_ = ((68i32) as u32); c.m.w32(fp.wrapping_add(48), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(52), v_); }
        { let v_ = ((66i32) as u32); c.m.w32(fp.wrapping_add(56), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(60), v_); }
        { let v_ = ((327680i32) as u32); c.m.w32(fp.wrapping_add(64), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(68), v_); }
        { let v_ = ((71i32) as u32); c.m.w32(fp.wrapping_add(72), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(76), v_); }
        F::Dma_AddData(c, fp, ((80u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ObjGs_AddFrameMaskOpaque", &PROBE_N, &[], true); }
    r_
}

pub fn ObjGs_AddFrameMaskBlend(c: &mut Ctx, mut p_mask_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjGs_AddFrameMaskBlend", &PROBE_N, &[(p_mask_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 80);
        { let v_ = ((((268435456i32) | (4i32))) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((((1342177280i32) | (4i32))) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((((32768i32) | (3i32))) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = (((((((((((if (((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0) { 0i32 } else { 112i32 })) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((p_mask_1) as u64)).wrapping_shl((32i32) as u32)))) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = ((((((((((((if (((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0) { 0i32 } else { 112i32 })) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((p_mask_1) as u64)).wrapping_shl((32i32) as u32)))).wrapping_shr((32i32) as u32)) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((76i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        { let v_ = ((100i32) as u32); c.m.w32(fp.wrapping_add(48), v_); }
        { let v_ = ((128i32) as u32); c.m.w32(fp.wrapping_add(52), v_); }
        { let v_ = ((66i32) as u32); c.m.w32(fp.wrapping_add(56), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(60), v_); }
        { let v_ = ((327680i32) as u32); c.m.w32(fp.wrapping_add(64), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(68), v_); }
        { let v_ = ((71i32) as u32); c.m.w32(fp.wrapping_add(72), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(76), v_); }
        F::Dma_AddData(c, fp, ((80u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ObjGs_AddFrameMaskBlend", &PROBE_N, &[], true); }
    r_
}

pub fn ObjGs_AddFlatShadowEnv(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjGs_AddFlatShadowEnv", &PROBE_N, &[], false); }
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
        { let v_ = ((822083808i32) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = ((1i32) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((78i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        { let v_ = ((344064i32) as u32); c.m.w32(fp.wrapping_add(48), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(52), v_); }
        { let v_ = ((71i32) as u32); c.m.w32(fp.wrapping_add(56), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(60), v_); }
        { let v_ = ((1i32) as u32); c.m.w32(fp.wrapping_add(64), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(68), v_); }
        { let v_ = ((74i32) as u32); c.m.w32(fp.wrapping_add(72), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(76), v_); }
        { let v_ = ((100i32) as u32); c.m.w32(fp.wrapping_add(80), v_); }
        { let v_ = ((70i32) as u32); c.m.w32(fp.wrapping_add(84), v_); }
        { let v_ = ((66i32) as u32); c.m.w32(fp.wrapping_add(88), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(92), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(96), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(100), v_); }
        { let v_ = ((26i32) as u32); c.m.w32(fp.wrapping_add(104), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(108), v_); }
        { let v_ = ((72i32) as u32); c.m.w32(fp.wrapping_add(112), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(116), v_); }
        { let v_ = ((27i32) as u32); c.m.w32(fp.wrapping_add(120), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(124), v_); }
        F::Dma_AddData(c, fp, ((128u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ObjGs_AddFlatShadowEnv", &PROBE_N, &[], true); }
    r_
}

pub fn ObjGs_AddShadowTexEnv(c: &mut Ctx, mut p_tbp_1: i32, mut p_tbw_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjGs_AddShadowTexEnv", &PROBE_N, &[(p_tbp_1) as u32, (p_tbw_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(192) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 192);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 192);
        { let v_ = ((((268435456i32) | (11i32))) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((((1342177280i32) | (11i32))) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((((32768i32) | (10i32))) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { 524400i32 } else { 524288i32 })) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((76i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        { let v_ = ((822083808i32) as u32); c.m.w32(fp.wrapping_add(48), v_); }
        { let v_ = ((1i32) as u32); c.m.w32(fp.wrapping_add(52), v_); }
        { let v_ = ((78i32) as u32); c.m.w32(fp.wrapping_add(56), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(60), v_); }
        { let v_ = ((68i32) as u32); c.m.w32(fp.wrapping_add(64), v_); }
        { let v_ = ((128i32) as u32); c.m.w32(fp.wrapping_add(68), v_); }
        { let v_ = ((66i32) as u32); c.m.w32(fp.wrapping_add(72), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(76), v_); }
        { let v_ = ((344079i32) as u32); c.m.w32(fp.wrapping_add(80), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(84), v_); }
        { let v_ = ((71i32) as u32); c.m.w32(fp.wrapping_add(88), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(92), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(96), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(100), v_); }
        { let v_ = ((20i32) as u32); c.m.w32(fp.wrapping_add(104), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(108), v_); }
        { let v_ = ((1i32) as u32); c.m.w32(fp.wrapping_add(112), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(116), v_); }
        { let v_ = ((74i32) as u32); c.m.w32(fp.wrapping_add(120), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(124), v_); }
        { let v_ = ((5i32) as u32); c.m.w32(fp.wrapping_add(128), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(132), v_); }
        { let v_ = ((8i32) as u32); c.m.w32(fp.wrapping_add(136), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(140), v_); }
        { let v_ = ((1i32) as u32); c.m.w32(fp.wrapping_add(144), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(148), v_); }
        { let v_ = ((70i32) as u32); c.m.w32(fp.wrapping_add(152), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(156), v_); }
        { let v_ = ((32896i32) as u32); c.m.w32(fp.wrapping_add(160), v_); }
        { let v_ = ((128i32) as u32); c.m.w32(fp.wrapping_add(164), v_); }
        { let v_ = ((59i32) as u32); c.m.w32(fp.wrapping_add(168), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(172), v_); }
        { let v_ = ((((((((p_tbp_1) as u64)) | ((((p_tbw_2) as u64)).wrapping_shl((14i32) as u32)))) | ((((50178i32) as u64)).wrapping_shl((19i32) as u32)))) as u32); c.m.w32(fp.wrapping_add(176), v_); }
        { let v_ = (((((((((p_tbp_1) as u64)) | ((((p_tbw_2) as u64)).wrapping_shl((14i32) as u32)))) | ((((50178i32) as u64)).wrapping_shl((19i32) as u32)))).wrapping_shr((32i32) as u32)) as u32); c.m.w32(fp.wrapping_add(180), v_); }
        { let v_ = ((6i32) as u32); c.m.w32(fp.wrapping_add(184), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(188), v_); }
        F::Dma_AddData(c, fp, ((192u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ObjGs_AddShadowTexEnv", &PROBE_N, &[], true); }
    r_
}

pub fn ObjGs_AddDefaultEnv(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjGs_AddDefaultEnv", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(208) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 208);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 208);
        { let v_ = ((((268435456i32) | (12i32))) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((((1342177280i32) | (12i32))) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((((32768i32) | (11i32))) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { 524400i32 } else { 524288i32 })) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((76i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        { let v_ = ((822083808i32) as u32); c.m.w32(fp.wrapping_add(48), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(52), v_); }
        { let v_ = ((78i32) as u32); c.m.w32(fp.wrapping_add(56), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(60), v_); }
        { let v_ = ((68i32) as u32); c.m.w32(fp.wrapping_add(64), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(68), v_); }
        { let v_ = ((66i32) as u32); c.m.w32(fp.wrapping_add(72), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(76), v_); }
        { let v_ = ((327680i32) as u32); c.m.w32(fp.wrapping_add(80), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(84), v_); }
        { let v_ = ((71i32) as u32); c.m.w32(fp.wrapping_add(88), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(92), v_); }
        { let v_ = ((96i32) as u32); c.m.w32(fp.wrapping_add(96), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(100), v_); }
        { let v_ = ((20i32) as u32); c.m.w32(fp.wrapping_add(104), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(108), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(112), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(116), v_); }
        { let v_ = ((74i32) as u32); c.m.w32(fp.wrapping_add(120), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(124), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(128), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(132), v_); }
        { let v_ = ((8i32) as u32); c.m.w32(fp.wrapping_add(136), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(140), v_); }
        { let v_ = ((1i32) as u32); c.m.w32(fp.wrapping_add(144), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(148), v_); }
        { let v_ = ((70i32) as u32); c.m.w32(fp.wrapping_add(152), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(156), v_); }
        { let v_ = 2155905152u32; c.m.w32(fp.wrapping_add(160), v_); }
        { let v_ = ((1065353216i32) as u32); c.m.w32(fp.wrapping_add(164), v_); }
        { let v_ = ((1i32) as u32); c.m.w32(fp.wrapping_add(168), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(172), v_); }
        { let v_ = ((1i32) as u32); c.m.w32(fp.wrapping_add(176), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(180), v_); }
        { let v_ = ((26i32) as u32); c.m.w32(fp.wrapping_add(184), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(188), v_); }
        { let v_ = ((28672i32) as u32); c.m.w32(fp.wrapping_add(192), v_); }
        { let v_ = ((29184i32) as u32); c.m.w32(fp.wrapping_add(196), v_); }
        { let v_ = ((24i32) as u32); c.m.w32(fp.wrapping_add(200), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(204), v_); }
        F::Dma_AddData(c, fp, ((208u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ObjGs_AddDefaultEnv", &PROBE_N, &[], true); }
    r_
}
