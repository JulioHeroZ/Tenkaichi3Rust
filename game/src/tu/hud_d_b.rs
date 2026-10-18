#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn HudNotice_GsBeginAdd(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_GsBeginAdd", &PROBE_N, &[], false); }
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
        { let v_ = ((208923i32) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((71i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { ((((((((112i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) } else { ((((((((0i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) })) as u32); c.m.w32(fp.wrapping_add(48), v_); }
        { let v_ = 4278190080u32; c.m.w32(fp.wrapping_add(52), v_); }
        { let v_ = ((77i32) as u32); c.m.w32(fp.wrapping_add(56), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(60), v_); }
        { let v_ = ((88i32) as u32); c.m.w32(fp.wrapping_add(64), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(68), v_); }
        { let v_ = ((67i32) as u32); c.m.w32(fp.wrapping_add(72), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(76), v_); }
        F::Dma_AddData(c, fp, ((80u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_GsBeginAdd", &PROBE_N, &[], true); }
    r_
}

pub fn HudNotice_GsEndAdd(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_GsEndAdd", &PROBE_N, &[], false); }
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
        { let v_ = ((196608i32) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((71i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { ((((((((112i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) } else { ((((((((0i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) })) as u32); c.m.w32(fp.wrapping_add(48), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(52), v_); }
        { let v_ = ((77i32) as u32); c.m.w32(fp.wrapping_add(56), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(60), v_); }
        { let v_ = ((68i32) as u32); c.m.w32(fp.wrapping_add(64), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(68), v_); }
        { let v_ = ((67i32) as u32); c.m.w32(fp.wrapping_add(72), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(76), v_); }
        F::Dma_AddData(c, fp, ((80u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_GsEndAdd", &PROBE_N, &[], true); }
    r_
}

pub fn HudNotice_GsBeginAddAlpha(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_GsBeginAddAlpha", &PROBE_N, &[], false); }
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
        { let v_ = ((208923i32) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((71i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { ((((((((112i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) } else { ((((((((0i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) })) as u32); c.m.w32(fp.wrapping_add(48), v_); }
        { let v_ = ((16777215i32) as u32); c.m.w32(fp.wrapping_add(52), v_); }
        { let v_ = ((76i32) as u32); c.m.w32(fp.wrapping_add(56), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(60), v_); }
        { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { ((((((((112i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) } else { ((((((((0i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) })) as u32); c.m.w32(fp.wrapping_add(64), v_); }
        { let v_ = 4278190080u32; c.m.w32(fp.wrapping_add(68), v_); }
        { let v_ = ((77i32) as u32); c.m.w32(fp.wrapping_add(72), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(76), v_); }
        { let v_ = ((88i32) as u32); c.m.w32(fp.wrapping_add(80), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(84), v_); }
        { let v_ = ((67i32) as u32); c.m.w32(fp.wrapping_add(88), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(92), v_); }
        F::Dma_AddData(c, fp, ((96u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_GsBeginAddAlpha", &PROBE_N, &[], true); }
    r_
}

pub fn HudNotice_GsEndAddAlpha(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_GsEndAddAlpha", &PROBE_N, &[], false); }
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
        { let v_ = ((76i32) as u32); c.m.w32(fp.wrapping_add(56), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(60), v_); }
        { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { ((((((((112i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) } else { ((((((((0i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) })) as u32); c.m.w32(fp.wrapping_add(64), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(68), v_); }
        { let v_ = ((77i32) as u32); c.m.w32(fp.wrapping_add(72), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(76), v_); }
        { let v_ = ((68i32) as u32); c.m.w32(fp.wrapping_add(80), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(84), v_); }
        { let v_ = ((67i32) as u32); c.m.w32(fp.wrapping_add(88), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(92), v_); }
        F::Dma_AddData(c, fp, ((96u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_GsEndAddAlpha", &PROBE_N, &[], true); }
    r_
}

pub fn HudNotice_GsBeginMask(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_GsBeginMask", &PROBE_N, &[], false); }
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
        { let v_ = ((1i32) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((74i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { ((((((((112i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) } else { ((((((((0i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) })) as u32); c.m.w32(fp.wrapping_add(48), v_); }
        { let v_ = ((16777215i32) as u32); c.m.w32(fp.wrapping_add(52), v_); }
        { let v_ = ((76i32) as u32); c.m.w32(fp.wrapping_add(56), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(60), v_); }
        { let v_ = ((245760i32) as u32); c.m.w32(fp.wrapping_add(64), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(68), v_); }
        { let v_ = ((72i32) as u32); c.m.w32(fp.wrapping_add(72), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(76), v_); }
        { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { ((((((((112i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) } else { ((((((((0i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) })) as u32); c.m.w32(fp.wrapping_add(80), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(84), v_); }
        { let v_ = ((77i32) as u32); c.m.w32(fp.wrapping_add(88), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(92), v_); }
        F::Dma_AddData(c, fp, ((96u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_GsBeginMask", &PROBE_N, &[], true); }
    r_
}

pub fn HudNotice_GsEndMask(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_GsEndMask", &PROBE_N, &[], false); }
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
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((74i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { ((((((((112i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) } else { ((((((((0i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) })) as u32); c.m.w32(fp.wrapping_add(48), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(52), v_); }
        { let v_ = ((76i32) as u32); c.m.w32(fp.wrapping_add(56), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(60), v_); }
        { let v_ = ((196608i32) as u32); c.m.w32(fp.wrapping_add(64), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(68), v_); }
        { let v_ = ((72i32) as u32); c.m.w32(fp.wrapping_add(72), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(76), v_); }
        { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { ((((((((112i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) } else { ((((((((0i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) })) as u32); c.m.w32(fp.wrapping_add(80), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(84), v_); }
        { let v_ = ((77i32) as u32); c.m.w32(fp.wrapping_add(88), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(92), v_); }
        F::Dma_AddData(c, fp, ((96u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_GsEndMask", &PROBE_N, &[], true); }
    r_
}

pub fn HudNotice_DrawNode(c: &mut Ctx, mut p_node_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_DrawNode", &PROBE_N, &[(p_node_1) as u32], false); }
    let mut l_i_2: u32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = ((0i32) as u32);
        'l1: loop {
            if !(((((l_i_2) < (c.m.r32((p_node_1).wrapping_add(32)))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = { let a_ = (c.m.r32((p_node_1).wrapping_add(36))).wrapping_add((l_i_2).wrapping_mul(4)); c.m.r32(a_) }; let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_DrawNode", &PROBE_N, &[], true); }
    r_
}

pub fn HudNotice_DrawSpin(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_DrawSpin", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_spr_2: u32 = 0;
    let r_ = 'f: {
    F::HudGfx_CallBegin(c, fid::HudNotice_GsBeginAdd);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (14i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_spr_2 = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((l_i_1) as u32)).wrapping_mul(28));
                if (((((l_i_1) == (13i32)) as i32)) != 0) {
                    { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_Draw(c, l_spr_2, a1_, 1i32) };
                } else {
                    { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_Draw(c, l_spr_2, a1_, 0i32) };
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    F::HudGfx_CallEnd(c, fid::HudNotice_GsEndAdd);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_DrawSpin", &PROBE_N, &[], true); }
    r_
}

pub fn HudNotice_DrawReady(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_DrawReady", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_node_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        F::HudGfx_ClearAlpha(c);
        F::HudGfx_CallBegin(c, fid::HudNotice_GsBeginMask);
        F::Vu0Cur_Push(c);
        l_node_1 = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(56));
        F::Mtx_StoreIdentity(c, fp);
        { let a2_ = F32(c.m.r32((l_node_1).wrapping_add(4))); F::Mtx_RotateZ(c, fp, fp, a2_) };
        F::Vu0Cur_MulMtx(c, fp);
        { let a0_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((14i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
        F::Vu0Cur_Pop(c);
        { let a0_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 1i32) };
        F::HudGfx_CallEnd(c, fid::HudNotice_GsEndMask);
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (12i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(((((1i32).wrapping_add(l_i_2)) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_DrawReady", &PROBE_N, &[], true); }
    r_
}

pub fn HudNotice_UpdateReady(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_UpdateReady", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_word_1: u32 = 0;
    let mut l_band_2: u32 = 0;
    let mut l_node_3: u32 = 0;
    let mut l_part_4: u32 = 0;
    let mut l_done_5: i32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_t_7: F32 = F32(0);
    let mut l_v_8: F32 = F32(0);
    let mut l_a_9: F32 = F32(0);
    let r_ = 'f: {
        l_word_1 = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28));
        l_band_2 = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((14i32) as u32)).wrapping_mul(28));
        l_node_3 = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(56));
        l_part_4 = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(28));
        rt::memset(c, fp, 0, 48);
        rt::memset(c, fp.wrapping_add(0), 0, 4);
        { let v_ = ((40i32) as i16); c.m.w16(fp.wrapping_add(0).wrapping_add(0), (v_) as u16); }
        { let v_ = (((70i32).wrapping_neg()) as i16); c.m.w16(fp.wrapping_add(0).wrapping_add(2), (v_) as u16); }
        rt::memset(c, fp.wrapping_add(4), 0, 4);
        { let v_ = ((190i32) as i16); c.m.w16(fp.wrapping_add(4).wrapping_add(0), (v_) as u16); }
        { let v_ = (((40i32).wrapping_neg()) as i16); c.m.w16(fp.wrapping_add(4).wrapping_add(2), (v_) as u16); }
        rt::memset(c, fp.wrapping_add(8), 0, 4);
        { let v_ = ((190i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(0), (v_) as u16); }
        { let v_ = ((40i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(2), (v_) as u16); }
        rt::memset(c, fp.wrapping_add(12), 0, 4);
        { let v_ = ((90i32) as i16); c.m.w16(fp.wrapping_add(12).wrapping_add(0), (v_) as u16); }
        { let v_ = ((90i32) as i16); c.m.w16(fp.wrapping_add(12).wrapping_add(2), (v_) as u16); }
        rt::memset(c, fp.wrapping_add(16), 0, 4);
        { let v_ = (((109i32).wrapping_neg()) as i16); c.m.w16(fp.wrapping_add(16).wrapping_add(0), (v_) as u16); }
        { let v_ = (((61i32).wrapping_neg()) as i16); c.m.w16(fp.wrapping_add(16).wrapping_add(2), (v_) as u16); }
        rt::memset(c, fp.wrapping_add(20), 0, 4);
        { let v_ = (((160i32).wrapping_neg()) as i16); c.m.w16(fp.wrapping_add(20).wrapping_add(0), (v_) as u16); }
        { let v_ = (((20i32).wrapping_neg()) as i16); c.m.w16(fp.wrapping_add(20).wrapping_add(2), (v_) as u16); }
        rt::memset(c, fp.wrapping_add(24), 0, 4);
        { let v_ = (((210i32).wrapping_neg()) as i16); c.m.w16(fp.wrapping_add(24).wrapping_add(0), (v_) as u16); }
        { let v_ = ((10i32) as i16); c.m.w16(fp.wrapping_add(24).wrapping_add(2), (v_) as u16); }
        rt::memset(c, fp.wrapping_add(28), 0, 4);
        { let v_ = (((170i32).wrapping_neg()) as i16); c.m.w16(fp.wrapping_add(28).wrapping_add(0), (v_) as u16); }
        { let v_ = ((60i32) as i16); c.m.w16(fp.wrapping_add(28).wrapping_add(2), (v_) as u16); }
        rt::memset(c, fp.wrapping_add(32), 0, 4);
        { let v_ = ((30i32) as i16); c.m.w16(fp.wrapping_add(32).wrapping_add(0), (v_) as u16); }
        { let v_ = (((60i32).wrapping_neg()) as i16); c.m.w16(fp.wrapping_add(32).wrapping_add(2), (v_) as u16); }
        rt::memset(c, fp.wrapping_add(36), 0, 4);
        { let v_ = ((160i32) as i16); c.m.w16(fp.wrapping_add(36).wrapping_add(0), (v_) as u16); }
        { let v_ = (((30i32).wrapping_neg()) as i16); c.m.w16(fp.wrapping_add(36).wrapping_add(2), (v_) as u16); }
        rt::memset(c, fp.wrapping_add(40), 0, 4);
        { let v_ = (((150i32).wrapping_neg()) as i16); c.m.w16(fp.wrapping_add(40).wrapping_add(0), (v_) as u16); }
        { let v_ = ((30i32) as i16); c.m.w16(fp.wrapping_add(40).wrapping_add(2), (v_) as u16); }
        rt::memset(c, fp.wrapping_add(44), 0, 4);
        { let v_ = (((100i32).wrapping_neg()) as i16); c.m.w16(fp.wrapping_add(44).wrapping_add(0), (v_) as u16); }
        { let v_ = ((60i32) as i16); c.m.w16(fp.wrapping_add(44).wrapping_add(2), (v_) as u16); }
        's1: {
            let sv10: i32 = { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); (c.m.r32(a_) as i32) };
            'k7: {
            'k6: {
            'k5: {
            'k4: {
            'k3: {
            'k2: {
                match sv10 { 0i32 => break 'k2, 1i32 => break 'k3, 2i32 => break 'k4, 3i32 => break 'k5, 4i32 => break 'k6, 5i32 => break 'k7, _ => break 's1, }
            }
            F::HudSprite_Show(c, l_word_1, 1i32);
            { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_word_1, a1_, 0i32, 0i32) };
            F::HudSprite_Center(c, l_word_1);
            F::HudSprite_Show(c, l_band_2, 1i32);
            F::HudSprite_SetRect(c, l_band_2, 0i32, 32i32, 0i32, 4i32);
            F::HudSprite_Center(c, l_band_2);
            F::HudNode_SetRot(c, l_node_3, F32(0x00000000));
            { let a0_ = (c.m.r32(sym::gHudNotice)).wrapping_add(20); F::Ramp_Start(c, a0_, F32(0x3e4ccccc), F32(0x00000000), F32(0x3f800000)) };
            { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            l_done_5 = { let a0_ = (c.m.r32(sym::gHudNotice)).wrapping_add(20); F::Ramp_Step(c, a0_) };
            l_v_8 = { let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(20)).wrapping_add(16); F32(c.m.r32(a_)) };
            { let r_ = F32(0x43f00000); let n_ = rt::fmul(l_v_8, r_); l_v_8 = n_; };
            { let r_ = F32(0x42000000); let n_ = rt::fadd(l_v_8, r_); l_v_8 = n_; };
            F::HudSprite_Show(c, l_band_2, 1i32);
            F::HudSprite_SetRect(c, l_band_2, 0i32, rt::f2i(l_v_8), 0i32, 4i32);
            F::HudSprite_Center(c, l_band_2);
            if ((l_done_5) != 0) {
                { let a0_ = (c.m.r32(sym::gHudNotice)).wrapping_add(20); F::Ramp_Start(c, a0_, F32(0x3eb851eb), F32(0x00000000), F32(0x3f800000)) };
                { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            break 's1;
            }
            l_done_5 = { let a0_ = (c.m.r32(sym::gHudNotice)).wrapping_add(20); F::Ramp_Step(c, a0_) };
            l_v_8 = { let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(20)).wrapping_add(16); F32(c.m.r32(a_)) };
            { let r_ = F32(0x42f80000); let n_ = rt::fmul(l_v_8, r_); l_v_8 = n_; };
            { let r_ = F32(0x40800000); let n_ = rt::fadd(l_v_8, r_); l_v_8 = n_; };
            F::HudSprite_Show(c, l_band_2, 1i32);
            F::HudSprite_SetRect(c, l_band_2, 0i32, 512i32, 0i32, rt::f2i(l_v_8));
            F::HudSprite_Center(c, l_band_2);
            { let a1_ = rt::fmul(rt::fneg({ let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(20)).wrapping_add(16); F32(c.m.r32(a_)) }), F32(0x40490fda)); F::HudNode_SetRot(c, l_node_3, a1_) };
            if ((l_done_5) != 0) {
                { let a0_ = (c.m.r32(sym::gHudNotice)).wrapping_add(44); F::Ramp_Start(c, a0_, F32(0x3f8ccccc), F32(0x3f800000), F32(0x00000000)) };
                { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            break 's1;
            }
            if (({ let a0_ = (c.m.r32(sym::gHudNotice)).wrapping_add(44); F::Ramp_Step(c, a0_) }) != 0) {
                { let a0_ = (c.m.r32(sym::gHudNotice)).wrapping_add(44); F::Ramp_Start(c, a0_, F32(0x3edc28f5), F32(0x3f800000), F32(0x00000000)) };
                { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            break 's1;
            }
            l_done_5 = { let a0_ = (c.m.r32(sym::gHudNotice)).wrapping_add(44); F::Ramp_Step(c, a0_) };
            l_t_7 = { let a0_ = { let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(44)).wrapping_add(16); F32(c.m.r32(a_)) }; F::Port_powf(c, a0_, F32(0x40400000)) };
            F::HudSprite_Show(c, l_word_1, 1i32);
            { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_word_1, a1_, 0i32, 0i32) };
            F::HudSprite_Center(c, l_word_1);
            F::HudSprite_Scale(c, l_word_1, rt::fadd(rt::fmul(rt::fsub(F32(0x3f800000), l_t_7), F32(0x3f000000)), F32(0x3f800000)), rt::fadd(rt::fmul(rt::fsub(F32(0x3f800000), l_t_7), F32(0x3f000000)), F32(0x3f800000)));
            F::HudSprite_SetColor(c, l_word_1, 128i32, 128i32, 128i32, ((((rt::f2i(rt::fmul(l_t_7, F32(0x43000000)))) as u8)) as i32));
            F::HudSprite_Show(c, l_band_2, 1i32);
            F::HudSprite_InitPlain(c, l_band_2, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8));
            F::HudSprite_SetRect(c, l_band_2, 0i32, 512i32, 0i32, 128i32);
            F::HudSprite_Center(c, l_band_2);
            F::HudSprite_Scale(c, l_band_2, rt::fadd(rt::fmul(rt::fsub(F32(0x3f800000), l_t_7), F32(0x3f000000)), F32(0x3f800000)), rt::fadd(rt::fmul(rt::fsub(F32(0x3f800000), l_t_7), F32(0x3f000000)), F32(0x3f800000)));
            {
                l_i_6 = 0i32;
                'l8: loop {
                    if !(((((l_i_6) < (12i32)) as i32)) != 0) { break 'l8; }
                    'c9: {
                        F::HudSprite_Show(c, (l_part_4).wrapping_add((((l_i_6) as u32)).wrapping_mul(28)), 1i32);
                        { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, (l_part_4).wrapping_add((((l_i_6) as u32)).wrapping_mul(28)), a1_, 0i32, 0i32) };
                        F::HudSprite_Center(c, (l_part_4).wrapping_add((((l_i_6) as u32)).wrapping_mul(28)));
                        { let a1_ = rt::f2i(rt::fmul(rt::i2f((((c.m.r16(((fp).wrapping_add((((l_i_6) as u32)).wrapping_mul(4))).wrapping_add((((0i32) as u32)).wrapping_mul(2))) as i16)) as i32)), rt::fsub(F32(0x3f800000), l_t_7))); let a2_ = rt::f2i(rt::fmul(rt::i2f((((c.m.r16(((fp).wrapping_add((((l_i_6) as u32)).wrapping_mul(4))).wrapping_add((((1i32) as u32)).wrapping_mul(2))) as i16)) as i32)), rt::fsub(F32(0x3f800000), l_t_7))); F::HudSprite_Move(c, (l_part_4).wrapping_add((((l_i_6) as u32)).wrapping_mul(28)), a1_, a2_) };
                        if (((((l_i_6) < (6i32)) as i32)) != 0) {
                            l_a_9 = rt::fsub({ let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(44)).wrapping_add(16); F32(c.m.r32(a_)) }, rt::fdiv(rt::i2f(l_i_6), F32(0x41f00000)));
                        } else {
                            l_a_9 = rt::fsub({ let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(44)).wrapping_add(16); F32(c.m.r32(a_)) }, rt::fdiv(rt::i2f((11i32).wrapping_sub(l_i_6)), F32(0x41f00000)));
                        }
                        l_a_9 = F::Port_powf(c, (if (((rt::flt(l_a_9, F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(0x3f800000), l_a_9) as i32)) != 0) { F32(0x3f800000) } else { l_a_9 }) }), F32(0x40400000));
                        { let r_ = F32(0x42800000); let n_ = rt::fmul(l_a_9, r_); l_a_9 = n_; };
                        F::HudSprite_SetColor(c, (l_part_4).wrapping_add((((l_i_6) as u32)).wrapping_mul(28)), 128i32, 128i32, 128i32, ((((rt::f2i(l_a_9)) as u8)) as i32));
                    }
                    { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                }
            }
            if ((l_done_5) != 0) {
                { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            break 's1;
            }
            F::HudSprite_Show(c, l_word_1, 0i32);
            F::HudSprite_Show(c, l_band_2, 0i32);
            {
                l_i_6 = 0i32;
                'l10: loop {
                    if !(((((l_i_6) < (12i32)) as i32)) != 0) { break 'l10; }
                    'c11: {
                        F::HudSprite_Show(c, (l_part_4).wrapping_add((((l_i_6) as u32)).wrapping_mul(28)), 0i32);
                    }
                    { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                }
            }
            break 's1;
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_UpdateReady", &PROBE_N, &[], true); }
    r_
}

pub fn HudNotice_DrawFight(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_DrawFight", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    F::HudGfx_ClearAlpha(c);
    F::HudGfx_CallBegin(c, fid::HudNotice_GsBeginMask);
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((14i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 1i32) };
    F::HudGfx_CallEnd(c, fid::HudNotice_GsEndMask);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (12i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(((((1i32).wrapping_add(l_i_1)) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    F::HudGfx_CallBegin(c, fid::HudNotice_GsBeginAddAlpha);
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((13i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 1i32) };
    F::HudGfx_CallEnd(c, fid::HudNotice_GsEndAddAlpha);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_DrawFight", &PROBE_N, &[], true); }
    r_
}

pub fn HudNotice_UpdateFight(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_UpdateFight", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_word_1: u32 = 0;
    let mut l_part_2: u32 = 0;
    let mut l_streak_3: u32 = 0;
    let mut l_band_4: u32 = 0;
    let mut l_done_5: i32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_t_7: F32 = F32(0);
    let mut l_a_8: F32 = F32(0);
    let mut l_v_9: F32 = F32(0);
    let r_ = 'f: {
        l_word_1 = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28));
        rt::memset(c, fp, 0, 48);
        rt::memset(c, fp.wrapping_add(0), 0, 4);
        { let v_ = ((40i32) as i16); c.m.w16(fp.wrapping_add(0).wrapping_add(0), (v_) as u16); }
        { let v_ = (((70i32).wrapping_neg()) as i16); c.m.w16(fp.wrapping_add(0).wrapping_add(2), (v_) as u16); }
        rt::memset(c, fp.wrapping_add(4), 0, 4);
        { let v_ = ((190i32) as i16); c.m.w16(fp.wrapping_add(4).wrapping_add(0), (v_) as u16); }
        { let v_ = (((40i32).wrapping_neg()) as i16); c.m.w16(fp.wrapping_add(4).wrapping_add(2), (v_) as u16); }
        rt::memset(c, fp.wrapping_add(8), 0, 4);
        { let v_ = ((190i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(0), (v_) as u16); }
        { let v_ = ((40i32) as i16); c.m.w16(fp.wrapping_add(8).wrapping_add(2), (v_) as u16); }
        rt::memset(c, fp.wrapping_add(12), 0, 4);
        { let v_ = ((90i32) as i16); c.m.w16(fp.wrapping_add(12).wrapping_add(0), (v_) as u16); }
        { let v_ = ((90i32) as i16); c.m.w16(fp.wrapping_add(12).wrapping_add(2), (v_) as u16); }
        rt::memset(c, fp.wrapping_add(16), 0, 4);
        { let v_ = (((109i32).wrapping_neg()) as i16); c.m.w16(fp.wrapping_add(16).wrapping_add(0), (v_) as u16); }
        { let v_ = (((61i32).wrapping_neg()) as i16); c.m.w16(fp.wrapping_add(16).wrapping_add(2), (v_) as u16); }
        rt::memset(c, fp.wrapping_add(20), 0, 4);
        { let v_ = (((160i32).wrapping_neg()) as i16); c.m.w16(fp.wrapping_add(20).wrapping_add(0), (v_) as u16); }
        { let v_ = (((20i32).wrapping_neg()) as i16); c.m.w16(fp.wrapping_add(20).wrapping_add(2), (v_) as u16); }
        rt::memset(c, fp.wrapping_add(24), 0, 4);
        { let v_ = (((210i32).wrapping_neg()) as i16); c.m.w16(fp.wrapping_add(24).wrapping_add(0), (v_) as u16); }
        { let v_ = ((10i32) as i16); c.m.w16(fp.wrapping_add(24).wrapping_add(2), (v_) as u16); }
        rt::memset(c, fp.wrapping_add(28), 0, 4);
        { let v_ = (((170i32).wrapping_neg()) as i16); c.m.w16(fp.wrapping_add(28).wrapping_add(0), (v_) as u16); }
        { let v_ = ((60i32) as i16); c.m.w16(fp.wrapping_add(28).wrapping_add(2), (v_) as u16); }
        rt::memset(c, fp.wrapping_add(32), 0, 4);
        { let v_ = ((30i32) as i16); c.m.w16(fp.wrapping_add(32).wrapping_add(0), (v_) as u16); }
        { let v_ = (((60i32).wrapping_neg()) as i16); c.m.w16(fp.wrapping_add(32).wrapping_add(2), (v_) as u16); }
        rt::memset(c, fp.wrapping_add(36), 0, 4);
        { let v_ = ((160i32) as i16); c.m.w16(fp.wrapping_add(36).wrapping_add(0), (v_) as u16); }
        { let v_ = (((30i32).wrapping_neg()) as i16); c.m.w16(fp.wrapping_add(36).wrapping_add(2), (v_) as u16); }
        rt::memset(c, fp.wrapping_add(40), 0, 4);
        { let v_ = (((150i32).wrapping_neg()) as i16); c.m.w16(fp.wrapping_add(40).wrapping_add(0), (v_) as u16); }
        { let v_ = ((30i32) as i16); c.m.w16(fp.wrapping_add(40).wrapping_add(2), (v_) as u16); }
        rt::memset(c, fp.wrapping_add(44), 0, 4);
        { let v_ = (((100i32).wrapping_neg()) as i16); c.m.w16(fp.wrapping_add(44).wrapping_add(0), (v_) as u16); }
        { let v_ = ((60i32) as i16); c.m.w16(fp.wrapping_add(44).wrapping_add(2), (v_) as u16); }
        l_part_2 = (l_word_1).wrapping_add((((1i32) as u32)).wrapping_mul(28));
        l_streak_3 = (l_word_1).wrapping_add((((13i32) as u32)).wrapping_mul(28));
        l_band_4 = (l_word_1).wrapping_add((((14i32) as u32)).wrapping_mul(28));
        's1: {
            let sv10: i32 = { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); (c.m.r32(a_) as i32) };
            'k8: {
            'k7: {
            'k6: {
            'k5: {
            'k4: {
            'k3: {
            'k2: {
                match sv10 { 0i32 => break 'k2, 1i32 => break 'k3, 2i32 => break 'k4, 3i32 => break 'k5, 4i32 => break 'k6, 5i32 => break 'k7, 6i32 => break 'k8, _ => break 's1, }
            }
            F::HudSprite_Show(c, l_word_1, 1i32);
            { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_word_1, a1_, 1i32, 0i32) };
            F::HudSprite_Center(c, l_word_1);
            F::HudSprite_Show(c, l_band_4, 1i32);
            F::HudSprite_InitPlain(c, l_band_4, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8));
            F::HudSprite_SetRect(c, l_band_4, 0i32, 512i32, 0i32, 448i32);
            F::HudSprite_Center(c, l_band_4);
            { let a0_ = (c.m.r32(sym::gHudNotice)).wrapping_add(44); F::Ramp_Start(c, a0_, F32(0x3edc28f5), F32(0x00000000), F32(0x3f800000)) };
            { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            l_done_5 = { let a0_ = (c.m.r32(sym::gHudNotice)).wrapping_add(44); F::Ramp_Step(c, a0_) };
            l_t_7 = { let a0_ = { let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(44)).wrapping_add(16); F32(c.m.r32(a_)) }; F::Port_powf(c, a0_, F32(0x40400000)) };
            F::HudSprite_Show(c, l_word_1, 1i32);
            { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_word_1, a1_, 1i32, 0i32) };
            F::HudSprite_Center(c, l_word_1);
            F::HudSprite_Scale(c, l_word_1, rt::fadd(rt::fmul(rt::fsub(F32(0x3f800000), l_t_7), F32(0x3f000000)), F32(0x3f800000)), rt::fadd(rt::fmul(rt::fsub(F32(0x3f800000), l_t_7), F32(0x3f000000)), F32(0x3f800000)));
            F::HudSprite_SetColor(c, l_word_1, 128i32, 128i32, 128i32, ((((rt::f2i(rt::fmul(l_t_7, F32(0x43000000)))) as u8)) as i32));
            {
                l_i_6 = 0i32;
                'l9: loop {
                    if !(((((l_i_6) < (12i32)) as i32)) != 0) { break 'l9; }
                    'c10: {
                        F::HudSprite_Show(c, (l_part_2).wrapping_add((((l_i_6) as u32)).wrapping_mul(28)), 1i32);
                        { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, (l_part_2).wrapping_add((((l_i_6) as u32)).wrapping_mul(28)), a1_, 1i32, 0i32) };
                        F::HudSprite_Center(c, (l_part_2).wrapping_add((((l_i_6) as u32)).wrapping_mul(28)));
                        { let a1_ = rt::f2i(rt::fmul(rt::i2f((((c.m.r16(((fp).wrapping_add((((l_i_6) as u32)).wrapping_mul(4))).wrapping_add((((0i32) as u32)).wrapping_mul(2))) as i16)) as i32)), rt::fsub(F32(0x3f800000), l_t_7))); let a2_ = rt::f2i(rt::fmul(rt::i2f((((c.m.r16(((fp).wrapping_add((((l_i_6) as u32)).wrapping_mul(4))).wrapping_add((((1i32) as u32)).wrapping_mul(2))) as i16)) as i32)), rt::fsub(F32(0x3f800000), l_t_7))); F::HudSprite_Move(c, (l_part_2).wrapping_add((((l_i_6) as u32)).wrapping_mul(28)), a1_, a2_) };
                        if (((((l_i_6) < (6i32)) as i32)) != 0) {
                            l_a_8 = rt::fsub({ let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(44)).wrapping_add(16); F32(c.m.r32(a_)) }, rt::fdiv(rt::i2f(l_i_6), F32(0x41f00000)));
                        } else {
                            l_a_8 = rt::fsub({ let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(44)).wrapping_add(16); F32(c.m.r32(a_)) }, rt::fdiv(rt::i2f((11i32).wrapping_sub(l_i_6)), F32(0x41f00000)));
                        }
                        l_a_8 = F::Port_powf(c, (if (((rt::flt(l_a_8, F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(0x3f800000), l_a_8) as i32)) != 0) { F32(0x3f800000) } else { l_a_8 }) }), F32(0x40400000));
                        { let r_ = F32(0x42800000); let n_ = rt::fmul(l_a_8, r_); l_a_8 = n_; };
                        F::HudSprite_SetColor(c, (l_part_2).wrapping_add((((l_i_6) as u32)).wrapping_mul(28)), 128i32, 128i32, 128i32, ((((rt::f2i(l_a_8)) as u8)) as i32));
                    }
                    { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                }
            }
            if ((l_done_5) != 0) {
                {
                    l_i_6 = 0i32;
                    'l11: loop {
                        if !(((((l_i_6) < (12i32)) as i32)) != 0) { break 'l11; }
                        'c12: {
                            F::HudSprite_Show(c, (l_part_2).wrapping_add((((l_i_6) as u32)).wrapping_mul(28)), 0i32);
                        }
                        { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                    }
                }
                F::HudSprite_SetColor(c, l_word_1, 128i32, 128i32, 128i32, 128i32);
                { let a0_ = (c.m.r32(sym::gHudNotice)).wrapping_add(44); F::Ramp_Start(c, a0_, F32(0x3ecccccc), F32(0x00000000), F32(0x3f800000)) };
                { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            break 's1;
            }
            if (({ let a0_ = (c.m.r32(sym::gHudNotice)).wrapping_add(44); F::Ramp_Step(c, a0_) }) != 0) {
                { let a0_ = (c.m.r32(sym::gHudNotice)).wrapping_add(44); F::Ramp_Start(c, a0_, F32(0x3ee66666), F32(0x00000000), F32(0x3f800000)) };
                { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            break 's1;
            }
            if (({ let a0_ = (c.m.r32(sym::gHudNotice)).wrapping_add(44); F::Ramp_Step(c, a0_) }) != 0) {
                { let a0_ = (c.m.r32(sym::gHudNotice)).wrapping_add(44); F::Ramp_Start(c, a0_, F32(0x3e4ccccc), F32(0x3f800000), F32(0x00000000)) };
                { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            break 's1;
            }
            if (({ let a0_ = (c.m.r32(sym::gHudNotice)).wrapping_add(44); F::Ramp_Step(c, a0_) }) != 0) {
                { let a0_ = (c.m.r32(sym::gHudNotice)).wrapping_add(44); F::Ramp_Start(c, a0_, F32(0x3e4ccccc), F32(0x3f800000), F32(0x00000000)) };
                { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            break 's1;
            }
            l_done_5 = { let a0_ = (c.m.r32(sym::gHudNotice)).wrapping_add(44); F::Ramp_Step(c, a0_) };
            l_t_7 = { let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(44)).wrapping_add(16); F32(c.m.r32(a_)) };
            F::HudSprite_SetRect(c, l_band_4, 0i32, 512i32, 0i32, rt::f2i(rt::fmul(l_t_7, F32(0x42800000))));
            F::HudSprite_Center(c, l_band_4);
            if ((l_done_5) != 0) {
                { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            break 's1;
            }
            F::HudSprite_Show(c, l_word_1, 0i32);
            F::HudSprite_Show(c, l_streak_3, 0i32);
            break 'f;
        }
        if ((((({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) {
            { let _ = F::memset(c, fp.wrapping_add(48), 0i32, ((((((8u32) as i32)) as i64 as u64)) as u32)); };
            F::HudSprite_Show(c, l_streak_3, 1i32);
            { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_streak_3, a1_, 7i32, 0i32) };
            F::HudSprite_SetRect(c, l_streak_3, 0i32, 512i32, 0i32, 42i32);
            { let a3_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(42i32); let a4_ = (({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(42i32)).wrapping_add(42i32); F::HudSprite_SetUv(c, l_streak_3, 0i32, 512i32, a3_, a4_) };
            F::HudSprite_Center(c, l_streak_3);
            { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fsub(F32(0x3f800000), { let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(44)).wrapping_add(16); F32(c.m.r32(a_)) }); let a_ = (fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fneg(rt::fmul(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::i2f(((((c.m.r16(((l_streak_3).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(2))) as i16)) as i32)).wrapping_sub((((c.m.r16(((l_streak_3).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(2))) as i16)) as i32))))); let a_ = (fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::i2f(((((c.m.r16(((l_streak_3).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(2))) as i16)) as i32)).wrapping_sub((((c.m.r16(((l_streak_3).wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(2))) as i16)) as i32)))); let a_ = (fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let a1_ = rt::f2i(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a2_ = rt::f2i(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::HudSprite_Move(c, l_streak_3, a1_, a2_) };
            F::HudSprite_SetColor(c, l_streak_3, 192i32, 112i32, 96i32, 128i32);
            { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            if ((((({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); (c.m.r32(a_) as i32) }) >= (3i32)) as i32)) != 0) {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
            }
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); (c.m.r32(a_) as i32) }) == (3i32)) as i32)) != 0) {
                { let _ = F::memset(c, fp.wrapping_add(48), 0i32, ((((((8u32) as i32)) as i64 as u64)) as u32)); };
                F::HudSprite_Show(c, l_streak_3, 1i32);
                { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_streak_3, a1_, 7i32, 0i32) };
                F::HudSprite_SetRect(c, l_streak_3, 0i32, 512i32, 0i32, 42i32);
                { let a3_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(42i32); let a4_ = (({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(42i32)).wrapping_add(42i32); F::HudSprite_SetUv(c, l_streak_3, 0i32, 512i32, a3_, a4_) };
                F::HudSprite_Center(c, l_streak_3);
                { let v_ = F::Rand_IntRange(c, (16i32).wrapping_neg(), 16i32); let a_ = (fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let a1_ = (c.m.r32((fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a2_ = (c.m.r32((fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); F::HudSprite_Move(c, l_streak_3, a1_, a2_) };
                F::HudSprite_SetColor(c, l_streak_3, 192i32, 112i32, 96i32, 128i32);
                { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                if ((((({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); (c.m.r32(a_) as i32) }) >= (3i32)) as i32)) != 0) {
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                }
            } else {
                if ((((({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); (c.m.r32(a_) as i32) }) == (4i32)) as i32)) != 0) {
                    { let _ = F::memset(c, fp.wrapping_add(48), 0i32, ((((((8u32) as i32)) as i64 as u64)) as u32)); };
                    F::HudSprite_Show(c, l_streak_3, 1i32);
                    { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_streak_3, a1_, 7i32, 0i32) };
                    F::HudSprite_SetRect(c, l_streak_3, 0i32, 512i32, 0i32, 42i32);
                    { let a3_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(42i32); let a4_ = (({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(42i32)).wrapping_add(42i32); F::HudSprite_SetUv(c, l_streak_3, 0i32, 512i32, a3_, a4_) };
                    F::HudSprite_Center(c, l_streak_3);
                    { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(F32(0x3f800000), { let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(44)).wrapping_add(16); F32(c.m.r32(a_)) }); let a_ = (fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::i2f(((((c.m.r16(((l_streak_3).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(2))) as i16)) as i32)).wrapping_sub((((c.m.r16(((l_streak_3).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(2))) as i16)) as i32)))); let a_ = (fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::i2f(((((c.m.r16(((l_streak_3).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(2))) as i16)) as i32)).wrapping_sub((((c.m.r16(((l_streak_3).wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(2))) as i16)) as i32)))); let a_ = (fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let a1_ = rt::f2i(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a2_ = rt::f2i(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::HudSprite_Move(c, l_streak_3, a1_, a2_) };
                    {
                        l_v_9 = { let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(44)).wrapping_add(16); F32(c.m.r32(a_)) };
                        F::HudSprite_SetColor(c, l_streak_3, ((((rt::f2i(rt::fmul(l_v_9, F32(0x43400000)))) as u8)) as i32), ((((rt::f2i(rt::fmul(l_v_9, F32(0x42e00000)))) as u8)) as i32), ((((rt::f2i(rt::fmul(l_v_9, F32(0x42c00000)))) as u8)) as i32), 128i32);
                    }
                    { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    if ((((({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); (c.m.r32(a_) as i32) }) >= (3i32)) as i32)) != 0) {
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                    }
                } else {
                    F::HudSprite_Show(c, l_streak_3, 0i32);
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_UpdateFight", &PROBE_N, &[], true); }
    r_
}

pub fn HudNotice_DrawKo(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_DrawKo", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    F::HudGfx_ClearAlpha(c);
    F::HudGfx_CallBegin(c, fid::HudNotice_GsBeginMask);
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((14i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 1i32) };
    F::HudGfx_CallEnd(c, fid::HudNotice_GsEndMask);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(((((1i32).wrapping_add(l_i_1)) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    F::HudGfx_CallBegin(c, fid::HudNotice_GsBeginAddAlpha);
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((13i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 1i32) };
    F::HudGfx_CallEnd(c, fid::HudNotice_GsEndAddAlpha);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_DrawKo", &PROBE_N, &[], true); }
    r_
}

pub fn HudNotice_UpdateKo(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_UpdateKo", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_word_1: u32 = 0;
    let mut l_part_2: u32 = 0;
    let mut l_streak_3: u32 = 0;
    let mut l_band_4: u32 = 0;
    let mut l_p_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_s_7: F32 = F32(0);
    let mut l_base_8: u32 = 0;
    let mut l_v_9: F32 = F32(0);
    let r_ = 'f: {
        l_word_1 = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28));
        l_part_2 = (l_word_1).wrapping_add((((1i32) as u32)).wrapping_mul(28));
        l_streak_3 = (l_word_1).wrapping_add((((13i32) as u32)).wrapping_mul(28));
        l_band_4 = (l_word_1).wrapping_add((((14i32) as u32)).wrapping_mul(28));
        's1: {
            let sv10: i32 = { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); (c.m.r32(a_) as i32) };
            'k9: {
            'k8: {
            'k7: {
            'k6: {
            'k5: {
            'k4: {
            'k3: {
            'k2: {
                match sv10 { 0i32 => break 'k2, 1i32 => break 'k3, 2i32 => break 'k4, 3i32 => break 'k5, 4i32 => break 'k6, 5i32 => break 'k7, 6i32 => break 'k8, 7i32 => break 'k9, _ => break 's1, }
            }
            F::HudSprite_Show(c, l_word_1, 1i32);
            { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_word_1, a1_, 2i32, 0i32) };
            F::HudSprite_Center(c, l_word_1);
            F::HudSprite_Show(c, l_band_4, 1i32);
            F::HudSprite_InitPlain(c, l_band_4, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8));
            F::HudSprite_SetRect(c, l_band_4, 0i32, 512i32, 0i32, 448i32);
            F::HudSprite_Center(c, l_band_4);
            {
                l_i_6 = 3i32;
                'l10: loop {
                    if !(((((l_i_6) >= (0i32)) as i32)) != 0) { break 'l10; }
                    'c11: {
                        l_base_8 = (c.m.r32(sym::gHudNotice)).wrapping_add(4);
                        l_p_5 = (l_base_8).wrapping_add((((((16i32).wrapping_add(3i32)).wrapping_sub(l_i_6)) as u32)).wrapping_mul(4));
                        { let v_ = F32(0x00000000); let a_ = l_p_5; c.m.w32(a_, (v_).0); };
                    }
                    { let o_ = l_i_6; let n_ = (o_).wrapping_sub(1); l_i_6 = n_; };
                }
            }
            { let a0_ = (c.m.r32(sym::gHudNotice)).wrapping_add(44); let a2_ = F32(c.m.r32(l_p_5)); F::Ramp_Start(c, a0_, F32(0x3e199999), a2_, F32(0x3f000000)) };
            { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            l_s_7 = { let a0_ = rt::fmul({ let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(44)).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0x40490fda)); F::Mathf_Sin(c, a0_) };
            {
                l_i_6 = 0i32;
                'l12: loop {
                    if !(((((l_i_6) < (3i32)) as i32)) != 0) { break 'l12; }
                    'c13: {
                        { let v_ = { let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(68)).wrapping_add(((((2i32).wrapping_sub(l_i_6)) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(68)).wrapping_add(((((3i32).wrapping_sub(l_i_6)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    }
                    { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                }
            }
            { let v_ = l_s_7; let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(68)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            F::HudSprite_Show(c, l_word_1, 1i32);
            { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_word_1, a1_, 2i32, 0i32) };
            F::HudSprite_Center(c, l_word_1);
            F::HudSprite_Scale(c, l_word_1, rt::fadd(rt::fmul(l_s_7, F32(0x3ecccccc)), F32(0x3f800000)), rt::fadd(rt::fmul(l_s_7, F32(0x3ecccccc)), F32(0x3f800000)));
            l_s_7 = (if (((rt::flt(rt::fmul({ let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(44)).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0x43800000)), F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(0x43000000), rt::fmul({ let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(44)).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0x43800000))) as i32)) != 0) { F32(0x43000000) } else { rt::fmul({ let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(44)).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0x43800000)) }) });
            F::HudSprite_SetColor(c, l_word_1, 128i32, 128i32, 128i32, ((((rt::f2i(l_s_7)) as u8)) as i32));
            {
                l_i_6 = 0i32;
                'l14: loop {
                    if !(((((l_i_6) < (3i32)) as i32)) != 0) { break 'l14; }
                    'c15: {
                        l_s_7 = { let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(68)).wrapping_add(((((l_i_6).wrapping_add(1i32)) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) };
                        if (((rt::feq(l_s_7, { let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(68)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }) as i32)) != 0) {
                            F::HudSprite_Show(c, (l_part_2).wrapping_add(((((l_i_6).wrapping_add(1i32)) as u32)).wrapping_mul(28)), 0i32);
                        } else {
                            F::HudSprite_Show(c, (l_part_2).wrapping_add(((((l_i_6).wrapping_add(1i32)) as u32)).wrapping_mul(28)), 1i32);
                        }
                        { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, (l_part_2).wrapping_add(((((l_i_6).wrapping_add(1i32)) as u32)).wrapping_mul(28)), a1_, 2i32, 0i32) };
                        F::HudSprite_Center(c, (l_part_2).wrapping_add(((((l_i_6).wrapping_add(1i32)) as u32)).wrapping_mul(28)));
                        F::HudSprite_Scale(c, (l_part_2).wrapping_add(((((l_i_6).wrapping_add(1i32)) as u32)).wrapping_mul(28)), rt::fadd(rt::fmul(l_s_7, F32(0x3ecccccc)), F32(0x3f800000)), rt::fadd(rt::fmul(l_s_7, F32(0x3ecccccc)), F32(0x3f800000)));
                        l_s_7 = (if (((rt::flt(rt::fmul({ let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(44)).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0x43800000)), F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(0x42400000), rt::fmul({ let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(44)).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0x43800000))) as i32)) != 0) { F32(0x42400000) } else { rt::fmul({ let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(44)).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0x43800000)) }) });
                        F::HudSprite_SetColor(c, (l_part_2).wrapping_add(((((l_i_6).wrapping_add(1i32)) as u32)).wrapping_mul(28)), 128i32, 128i32, 128i32, ((((rt::f2i(l_s_7)) as u8)) as i32));
                    }
                    { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                }
            }
            if (({ let a0_ = (c.m.r32(sym::gHudNotice)).wrapping_add(44); F::Ramp_Step(c, a0_) }) != 0) {
                { let a0_ = (c.m.r32(sym::gHudNotice)).wrapping_add(44); F::Ramp_Start(c, a0_, F32(0x3ecccccc), F32(0x3f000000), F32(0x3f800000)) };
                { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            break 's1;
            }
            l_s_7 = { let a0_ = rt::fmul({ let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(44)).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0x40490fda)); F::Mathf_Sin(c, a0_) };
            {
                l_i_6 = 0i32;
                'l16: loop {
                    if !(((((l_i_6) < (3i32)) as i32)) != 0) { break 'l16; }
                    'c17: {
                        { let v_ = { let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(68)).wrapping_add(((((2i32).wrapping_sub(l_i_6)) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(68)).wrapping_add(((((3i32).wrapping_sub(l_i_6)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    }
                    { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                }
            }
            { let v_ = l_s_7; let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(68)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            F::HudSprite_Show(c, l_word_1, 1i32);
            { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_word_1, a1_, 2i32, 0i32) };
            F::HudSprite_Center(c, l_word_1);
            F::HudSprite_Scale(c, l_word_1, rt::fadd(rt::fmul(l_s_7, F32(0x3ecccccc)), F32(0x3f800000)), rt::fadd(rt::fmul(l_s_7, F32(0x3ecccccc)), F32(0x3f800000)));
            l_s_7 = (if (((rt::flt(rt::fmul({ let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(44)).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0x43800000)), F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(0x43000000), rt::fmul({ let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(44)).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0x43800000))) as i32)) != 0) { F32(0x43000000) } else { rt::fmul({ let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(44)).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0x43800000)) }) });
            F::HudSprite_SetColor(c, l_word_1, 128i32, 128i32, 128i32, ((((rt::f2i(l_s_7)) as u8)) as i32));
            {
                l_i_6 = 0i32;
                'l18: loop {
                    if !(((((l_i_6) < (3i32)) as i32)) != 0) { break 'l18; }
                    'c19: {
                        l_s_7 = { let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(68)).wrapping_add(((((l_i_6).wrapping_add(1i32)) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) };
                        if (((rt::feq(l_s_7, { let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(68)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }) as i32)) != 0) {
                            F::HudSprite_Show(c, (l_part_2).wrapping_add(((((l_i_6).wrapping_add(1i32)) as u32)).wrapping_mul(28)), 0i32);
                        } else {
                            F::HudSprite_Show(c, (l_part_2).wrapping_add(((((l_i_6).wrapping_add(1i32)) as u32)).wrapping_mul(28)), 1i32);
                        }
                        { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, (l_part_2).wrapping_add(((((l_i_6).wrapping_add(1i32)) as u32)).wrapping_mul(28)), a1_, 2i32, 0i32) };
                        F::HudSprite_Center(c, (l_part_2).wrapping_add(((((l_i_6).wrapping_add(1i32)) as u32)).wrapping_mul(28)));
                        F::HudSprite_Scale(c, (l_part_2).wrapping_add(((((l_i_6).wrapping_add(1i32)) as u32)).wrapping_mul(28)), rt::fadd(rt::fmul(l_s_7, F32(0x3ecccccc)), F32(0x3f800000)), rt::fadd(rt::fmul(l_s_7, F32(0x3ecccccc)), F32(0x3f800000)));
                        l_s_7 = (if (((rt::flt(rt::fmul({ let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(44)).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0x43800000)), F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(0x42400000), rt::fmul({ let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(44)).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0x43800000))) as i32)) != 0) { F32(0x42400000) } else { rt::fmul({ let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(44)).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0x43800000)) }) });
                        F::HudSprite_SetColor(c, (l_part_2).wrapping_add(((((l_i_6).wrapping_add(1i32)) as u32)).wrapping_mul(28)), 128i32, 128i32, 128i32, ((((rt::f2i(l_s_7)) as u8)) as i32));
                    }
                    { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                }
            }
            if (((((({ let a0_ = (c.m.r32(sym::gHudNotice)).wrapping_add(44); F::Ramp_Step(c, a0_) }) != 0) && ((((c.m.r32((l_part_2).wrapping_add((((l_i_6) as u32)).wrapping_mul(28)))) & (((1i32) as u32)))) != 0)) as i32)) != 0) {
                { let a0_ = (c.m.r32(sym::gHudNotice)).wrapping_add(44); F::Ramp_Start(c, a0_, F32(0x3e4ccccc), F32(0x00000000), F32(0x3f800000)) };
                { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            break 's1;
            }
            if (({ let a0_ = (c.m.r32(sym::gHudNotice)).wrapping_add(44); F::Ramp_Step(c, a0_) }) != 0) {
                { let a0_ = (c.m.r32(sym::gHudNotice)).wrapping_add(44); F::Ramp_Start(c, a0_, F32(0x3f0ccccc), F32(0x00000000), F32(0x3f800000)) };
                { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            break 's1;
            }
            if (({ let a0_ = (c.m.r32(sym::gHudNotice)).wrapping_add(44); F::Ramp_Step(c, a0_) }) != 0) {
                { let a0_ = (c.m.r32(sym::gHudNotice)).wrapping_add(44); F::Ramp_Start(c, a0_, F32(0x3ecccccc), F32(0x3f800000), F32(0x00000000)) };
                { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            break 's1;
            }
            if (({ let a0_ = (c.m.r32(sym::gHudNotice)).wrapping_add(44); F::Ramp_Step(c, a0_) }) != 0) {
                { let a0_ = (c.m.r32(sym::gHudNotice)).wrapping_add(44); F::Ramp_Start(c, a0_, F32(0x3e4ccccc), F32(0x3f800000), F32(0x00000000)) };
                { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            break 's1;
            }
            if (({ let a0_ = (c.m.r32(sym::gHudNotice)).wrapping_add(44); F::Ramp_Step(c, a0_) }) != 0) {
                { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            { let a4_ = rt::f2i(rt::fmul({ let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(44)).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0x42800000))); F::HudSprite_SetRect(c, l_band_4, 0i32, 512i32, 0i32, a4_) };
            F::HudSprite_Center(c, l_band_4);
            break 's1;
            }
            F::HudSprite_Show(c, l_word_1, 0i32);
            F::HudSprite_Show(c, l_part_2, 0i32);
            F::HudSprite_Show(c, l_streak_3, 0i32);
            break 'f;
        }
        if ((((({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); (c.m.r32(a_) as i32) }) == (3i32)) as i32)) != 0) {
            { let _ = F::memset(c, fp, 0i32, ((((((8u32) as i32)) as i64 as u64)) as u32)); };
            F::HudSprite_Show(c, l_streak_3, 1i32);
            { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_streak_3, a1_, 7i32, 0i32) };
            F::HudSprite_SetRect(c, l_streak_3, 0i32, 512i32, 0i32, 42i32);
            { let a3_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(42i32); let a4_ = (({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(42i32)).wrapping_add(42i32); F::HudSprite_SetUv(c, l_streak_3, 0i32, 512i32, a3_, a4_) };
            F::HudSprite_Center(c, l_streak_3);
            { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fsub(F32(0x3f800000), { let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(44)).wrapping_add(16); F32(c.m.r32(a_)) }); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fneg(rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::i2f(((((c.m.r16(((l_streak_3).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(2))) as i16)) as i32)).wrapping_sub((((c.m.r16(((l_streak_3).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(2))) as i16)) as i32))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::i2f(((((c.m.r16(((l_streak_3).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(2))) as i16)) as i32)).wrapping_sub((((c.m.r16(((l_streak_3).wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(2))) as i16)) as i32)))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let a1_ = rt::f2i(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a2_ = rt::f2i(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::HudSprite_Move(c, l_streak_3, a1_, a2_) };
            F::HudSprite_SetColor(c, l_streak_3, 255i32, 96i32, 96i32, 128i32);
            { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            if ((((({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); (c.m.r32(a_) as i32) }) >= (3i32)) as i32)) != 0) {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
            }
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); (c.m.r32(a_) as i32) }) == (4i32)) as i32)) != 0) {
                { let _ = F::memset(c, fp, 0i32, ((((((8u32) as i32)) as i64 as u64)) as u32)); };
                F::HudSprite_Show(c, l_streak_3, 1i32);
                { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_streak_3, a1_, 7i32, 0i32) };
                F::HudSprite_SetRect(c, l_streak_3, 0i32, 512i32, 0i32, 42i32);
                { let a3_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(42i32); let a4_ = (({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(42i32)).wrapping_add(42i32); F::HudSprite_SetUv(c, l_streak_3, 0i32, 512i32, a3_, a4_) };
                F::HudSprite_Center(c, l_streak_3);
                { let v_ = F::Rand_IntRange(c, (16i32).wrapping_neg(), 16i32); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let a1_ = (c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a2_ = (c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); F::HudSprite_Move(c, l_streak_3, a1_, a2_) };
                F::HudSprite_SetColor(c, l_streak_3, 255i32, 96i32, 96i32, 128i32);
                { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                if ((((({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); (c.m.r32(a_) as i32) }) >= (3i32)) as i32)) != 0) {
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                }
            } else {
                if ((((({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); (c.m.r32(a_) as i32) }) == (5i32)) as i32)) != 0) {
                    { let _ = F::memset(c, fp, 0i32, ((((((8u32) as i32)) as i64 as u64)) as u32)); };
                    F::HudSprite_Show(c, l_streak_3, 1i32);
                    { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_streak_3, a1_, 7i32, 0i32) };
                    F::HudSprite_SetRect(c, l_streak_3, 0i32, 512i32, 0i32, 42i32);
                    { let a3_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(42i32); let a4_ = (({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(42i32)).wrapping_add(42i32); F::HudSprite_SetUv(c, l_streak_3, 0i32, 512i32, a3_, a4_) };
                    F::HudSprite_Center(c, l_streak_3);
                    { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(F32(0x3f800000), { let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(44)).wrapping_add(16); F32(c.m.r32(a_)) }); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::i2f(((((c.m.r16(((l_streak_3).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(2))) as i16)) as i32)).wrapping_sub((((c.m.r16(((l_streak_3).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(2))) as i16)) as i32)))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::i2f(((((c.m.r16(((l_streak_3).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(2))) as i16)) as i32)).wrapping_sub((((c.m.r16(((l_streak_3).wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(2))) as i16)) as i32)))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let a1_ = rt::f2i(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a2_ = rt::f2i(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::HudSprite_Move(c, l_streak_3, a1_, a2_) };
                    {
                        l_v_9 = { let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(44)).wrapping_add(16); F32(c.m.r32(a_)) };
                        F::HudSprite_SetColor(c, l_streak_3, ((((rt::f2i(rt::fmul(l_v_9, F32(0x437f0000)))) as u8)) as i32), ((((rt::f2i(rt::fmul(l_v_9, F32(0x42c00000)))) as u8)) as i32), ((((rt::f2i(rt::fmul(l_v_9, F32(0x42c00000)))) as u8)) as i32), 128i32);
                    }
                    { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    if ((((({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); (c.m.r32(a_) as i32) }) >= (3i32)) as i32)) != 0) {
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                    }
                } else {
                    F::HudSprite_Show(c, l_streak_3, 0i32);
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_UpdateKo", &PROBE_N, &[], true); }
    r_
}

pub fn HudNotice_DrawBandA(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_DrawBandA", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::HudGfx_ClearAlpha(c);
    F::HudGfx_CallBegin(c, fid::HudNotice_GsBeginMask);
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((14i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 1i32) };
    F::HudGfx_CallEnd(c, fid::HudNotice_GsEndMask);
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_DrawBandA", &PROBE_N, &[], true); }
    r_
}

pub fn HudNotice_UpdateBandA(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_UpdateBandA", &PROBE_N, &[], false); }
    let mut l_ramp_1: u32 = 0;
    let mut l_word_2: u32 = 0;
    let mut l_band_3: u32 = 0;
    let mut l_copy_4: u32 = 0;
    let mut l_done_5: i32 = 0;
    let mut l_v_6: F32 = F32(0);
    let mut l_one_7: F32 = F32(0);
    let r_ = 'f: {
    l_ramp_1 = (c.m.r32(sym::gHudNotice)).wrapping_add(20);
    l_word_2 = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28));
    l_band_3 = (l_word_2).wrapping_add((((14i32) as u32)).wrapping_mul(28));
    l_copy_4 = (l_word_2).wrapping_add((((1i32) as u32)).wrapping_mul(28));
    's1: {
        let sv8: i32 = { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); (c.m.r32(a_) as i32) };
        'k10: {
        'k9: {
        'k8: {
        'k7: {
        'k6: {
        'k5: {
        'k4: {
        'k3: {
        'k2: {
            match sv8 { 0i32 => break 'k2, 1i32 => break 'k3, 2i32 => break 'k4, 3i32 => break 'k5, 4i32 => break 'k6, 5i32 => break 'k7, 6i32 => break 'k8, 7i32 => break 'k9, 8i32 => break 'k10, _ => break 's1, }
        }
        F::HudSprite_Show(c, l_word_2, 1i32);
        { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_word_2, a1_, 3i32, 0i32) };
        F::HudSprite_SetRect(c, l_word_2, (256i32).wrapping_neg(), 256i32, (32i32).wrapping_neg(), 32i32);
        F::HudSprite_SetUv(c, l_word_2, 0i32, 512i32, 64i32, 128i32);
        F::HudSprite_Show(c, l_copy_4, 0i32);
        { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_copy_4, a1_, 3i32, 0i32) };
        F::HudSprite_SetRect(c, l_copy_4, (256i32).wrapping_neg(), 256i32, (32i32).wrapping_neg(), 32i32);
        F::HudSprite_SetUv(c, l_copy_4, 0i32, 512i32, 64i32, 128i32);
        F::Ramp_Start(c, l_ramp_1, F32(0x3e75c28f), F32(0x00000000), F32(0x3f800000));
        { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        l_one_7 = F32(0x3f800000);
        l_done_5 = F::Ramp_Step(c, l_ramp_1);
        l_v_6 = rt::fsub(l_one_7, F32(c.m.r32((l_ramp_1).wrapping_add(16))));
        F::HudSprite_Show(c, l_band_3, 1i32);
        F::HudSprite_InitPlain(c, l_band_3, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8));
        F::HudSprite_SetRect(c, l_band_3, 0i32, 512i32, 0i32, 4i32);
        F::HudSprite_Center(c, l_band_3);
        F::HudSprite_Move(c, l_band_3, rt::f2i(rt::fmul(l_v_6, F32(0xc4000000))), 0i32);
        if ((l_done_5) != 0) {
            F::Ramp_Start(c, l_ramp_1, F32(0x3e75c28f), F32(0x00000000), l_one_7);
            { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        break 's1;
        }
        l_done_5 = F::Ramp_Step(c, l_ramp_1);
        l_v_6 = rt::fadd(rt::fmul(F32(c.m.r32((l_ramp_1).wrapping_add(16))), F32(0x42700000)), F32(0x40800000));
        F::HudSprite_InitPlain(c, l_band_3, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8));
        F::HudSprite_SetRect(c, l_band_3, 0i32, 512i32, 0i32, rt::f2i(l_v_6));
        F::HudSprite_Center(c, l_band_3);
        if ((l_done_5) != 0) {
            F::Ramp_Start(c, l_ramp_1, F32(0x3e800000), F32(0x00000000), F32(0x3f800000));
            { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        break 's1;
        }
        if ((F::Ramp_Step(c, l_ramp_1)) != 0) {
            F::Ramp_Start(c, l_ramp_1, F32(0x3f23d70a), F32(0x3f800000), F32(0x00000000));
            { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        break 's1;
        }
        l_done_5 = F::Ramp_Step(c, l_ramp_1);
        F::HudSprite_Show(c, l_copy_4, 1i32);
        { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_copy_4, a1_, 3i32, 0i32) };
        F::HudSprite_SetRect(c, l_copy_4, (256i32).wrapping_neg(), 256i32, (32i32).wrapping_neg(), 32i32);
        F::HudSprite_SetUv(c, l_copy_4, 0i32, 512i32, 64i32, 128i32);
        { let a4_ = ((((rt::f2i(rt::fmul({ let a0_ = F32(c.m.r32((l_ramp_1).wrapping_add(16))); F::Port_powf(c, a0_, F32(0x40400000)) }, F32(0x43000000)))) as u8)) as i32); F::HudSprite_SetColor(c, l_copy_4, 128i32, 128i32, 128i32, a4_) };
        l_one_7 = F32(0x3f800000);
        l_v_6 = rt::fadd(rt::fsub(l_one_7, { let a0_ = F32(c.m.r32((l_ramp_1).wrapping_add(16))); F::Port_powf(c, a0_, F32(0x40400000)) }), l_one_7);
        F::HudSprite_Scale(c, l_copy_4, l_v_6, l_v_6);
        if ((l_done_5) != 0) {
            { let a0_ = (c.m.r32(sym::gHudNotice)).wrapping_add(44); F::Ramp_Start(c, a0_, F32(0x3f199999), l_one_7, F32(0x00000000)) };
            { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        break 's1;
        }
        if ((F::Ramp_Step(c, l_ramp_1)) != 0) {
            F::Ramp_Start(c, l_ramp_1, F32(0x3e75c28f), F32(0x3f800000), F32(0x00000000));
            { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        break 's1;
        }
        l_done_5 = F::Ramp_Step(c, l_ramp_1);
        l_v_6 = rt::fadd(rt::fmul(F32(c.m.r32((l_ramp_1).wrapping_add(16))), F32(0x42700000)), F32(0x40800000));
        F::HudSprite_InitPlain(c, l_band_3, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8));
        F::HudSprite_SetRect(c, l_band_3, 0i32, 512i32, 0i32, rt::f2i(l_v_6));
        F::HudSprite_Center(c, l_band_3);
        if ((l_done_5) != 0) {
            F::Ramp_Start(c, l_ramp_1, F32(0x3e75c28f), F32(0x00000000), F32(0x3f800000));
            { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        break 's1;
        }
        l_done_5 = F::Ramp_Step(c, l_ramp_1);
        l_v_6 = F32(c.m.r32((l_ramp_1).wrapping_add(16)));
        F::HudSprite_SetRect(c, l_band_3, 0i32, 512i32, 0i32, 4i32);
        F::HudSprite_Center(c, l_band_3);
        F::HudSprite_Move(c, l_band_3, rt::f2i(rt::fmul(l_v_6, F32(0x44000000))), 0i32);
        if ((l_done_5) != 0) {
            { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        break 's1;
        }
        F::HudSprite_Show(c, l_word_2, 0i32);
        F::HudSprite_Show(c, l_band_3, 0i32);
        F::HudSprite_Show(c, l_copy_4, 0i32);
        break 's1;
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_UpdateBandA", &PROBE_N, &[], true); }
    r_
}

pub fn HudNotice_DrawBandB(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_DrawBandB", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::HudGfx_ClearAlpha(c);
    F::HudGfx_CallBegin(c, fid::HudNotice_GsBeginMask);
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((14i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 1i32) };
    F::HudGfx_CallEnd(c, fid::HudNotice_GsEndMask);
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_DrawBandB", &PROBE_N, &[], true); }
    r_
}

pub fn HudNotice_UpdateBandB(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_UpdateBandB", &PROBE_N, &[], false); }
    let mut l_ramp_1: u32 = 0;
    let mut l_word_2: u32 = 0;
    let mut l_band_3: u32 = 0;
    let mut l_copy_4: u32 = 0;
    let mut l_done_5: i32 = 0;
    let mut l_v_6: F32 = F32(0);
    let mut l_one_7: F32 = F32(0);
    let r_ = 'f: {
    l_ramp_1 = (c.m.r32(sym::gHudNotice)).wrapping_add(20);
    l_word_2 = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28));
    l_band_3 = (l_word_2).wrapping_add((((14i32) as u32)).wrapping_mul(28));
    l_copy_4 = (l_word_2).wrapping_add((((1i32) as u32)).wrapping_mul(28));
    's1: {
        let sv8: i32 = { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); (c.m.r32(a_) as i32) };
        'k10: {
        'k9: {
        'k8: {
        'k7: {
        'k6: {
        'k5: {
        'k4: {
        'k3: {
        'k2: {
            match sv8 { 0i32 => break 'k2, 1i32 => break 'k3, 2i32 => break 'k4, 3i32 => break 'k5, 4i32 => break 'k6, 5i32 => break 'k7, 6i32 => break 'k8, 7i32 => break 'k9, 8i32 => break 'k10, _ => break 's1, }
        }
        F::HudSprite_Show(c, l_word_2, 1i32);
        { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_word_2, a1_, 3i32, 0i32) };
        F::HudSprite_SetRect(c, l_word_2, (256i32).wrapping_neg(), 256i32, (32i32).wrapping_neg(), 32i32);
        F::HudSprite_SetUv(c, l_word_2, 0i32, 512i32, 0i32, 64i32);
        F::HudSprite_Show(c, l_copy_4, 0i32);
        { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_copy_4, a1_, 3i32, 0i32) };
        F::HudSprite_SetRect(c, l_copy_4, (256i32).wrapping_neg(), 256i32, (32i32).wrapping_neg(), 32i32);
        F::HudSprite_SetUv(c, l_copy_4, 0i32, 512i32, 0i32, 64i32);
        F::Ramp_Start(c, l_ramp_1, F32(0x3e75c28f), F32(0x00000000), F32(0x3f800000));
        { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        l_one_7 = F32(0x3f800000);
        l_done_5 = F::Ramp_Step(c, l_ramp_1);
        l_v_6 = rt::fsub(l_one_7, F32(c.m.r32((l_ramp_1).wrapping_add(16))));
        F::HudSprite_Show(c, l_band_3, 1i32);
        F::HudSprite_InitPlain(c, l_band_3, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8));
        F::HudSprite_SetRect(c, l_band_3, 0i32, 512i32, 0i32, 4i32);
        F::HudSprite_Center(c, l_band_3);
        F::HudSprite_Move(c, l_band_3, rt::f2i(rt::fmul(l_v_6, F32(0xc4000000))), 0i32);
        if ((l_done_5) != 0) {
            F::Ramp_Start(c, l_ramp_1, F32(0x3e75c28f), F32(0x00000000), l_one_7);
            { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        break 's1;
        }
        l_done_5 = F::Ramp_Step(c, l_ramp_1);
        l_v_6 = rt::fadd(rt::fmul(F32(c.m.r32((l_ramp_1).wrapping_add(16))), F32(0x42700000)), F32(0x40800000));
        F::HudSprite_InitPlain(c, l_band_3, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8));
        F::HudSprite_SetRect(c, l_band_3, 0i32, 512i32, 0i32, rt::f2i(l_v_6));
        F::HudSprite_Center(c, l_band_3);
        if ((l_done_5) != 0) {
            F::Ramp_Start(c, l_ramp_1, F32(0x3e800000), F32(0x00000000), F32(0x3f800000));
            { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        break 's1;
        }
        if ((F::Ramp_Step(c, l_ramp_1)) != 0) {
            F::Ramp_Start(c, l_ramp_1, F32(0x3f23d70a), F32(0x3f800000), F32(0x00000000));
            { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        break 's1;
        }
        l_done_5 = F::Ramp_Step(c, l_ramp_1);
        F::HudSprite_Show(c, l_copy_4, 1i32);
        { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_copy_4, a1_, 3i32, 0i32) };
        F::HudSprite_SetRect(c, l_copy_4, (256i32).wrapping_neg(), 256i32, (32i32).wrapping_neg(), 32i32);
        F::HudSprite_SetUv(c, l_copy_4, 0i32, 512i32, 0i32, 64i32);
        { let a4_ = ((((rt::f2i(rt::fmul({ let a0_ = F32(c.m.r32((l_ramp_1).wrapping_add(16))); F::Port_powf(c, a0_, F32(0x40400000)) }, F32(0x43000000)))) as u8)) as i32); F::HudSprite_SetColor(c, l_copy_4, 128i32, 128i32, 128i32, a4_) };
        l_one_7 = F32(0x3f800000);
        l_v_6 = rt::fadd(rt::fsub(l_one_7, { let a0_ = F32(c.m.r32((l_ramp_1).wrapping_add(16))); F::Port_powf(c, a0_, F32(0x40400000)) }), l_one_7);
        F::HudSprite_Scale(c, l_copy_4, l_v_6, l_v_6);
        if ((l_done_5) != 0) {
            { let a0_ = (c.m.r32(sym::gHudNotice)).wrapping_add(44); F::Ramp_Start(c, a0_, F32(0x3f199999), l_one_7, F32(0x00000000)) };
            { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        break 's1;
        }
        if ((F::Ramp_Step(c, l_ramp_1)) != 0) {
            F::Ramp_Start(c, l_ramp_1, F32(0x3e75c28f), F32(0x3f800000), F32(0x00000000));
            { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        break 's1;
        }
        l_done_5 = F::Ramp_Step(c, l_ramp_1);
        l_v_6 = rt::fadd(rt::fmul(F32(c.m.r32((l_ramp_1).wrapping_add(16))), F32(0x42700000)), F32(0x40800000));
        F::HudSprite_InitPlain(c, l_band_3, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8));
        F::HudSprite_SetRect(c, l_band_3, 0i32, 512i32, 0i32, rt::f2i(l_v_6));
        F::HudSprite_Center(c, l_band_3);
        if ((l_done_5) != 0) {
            F::Ramp_Start(c, l_ramp_1, F32(0x3e75c28f), F32(0x00000000), F32(0x3f800000));
            { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        break 's1;
        }
        l_done_5 = F::Ramp_Step(c, l_ramp_1);
        l_v_6 = F32(c.m.r32((l_ramp_1).wrapping_add(16)));
        F::HudSprite_SetRect(c, l_band_3, 0i32, 512i32, 0i32, 4i32);
        F::HudSprite_Center(c, l_band_3);
        F::HudSprite_Move(c, l_band_3, rt::f2i(rt::fmul(l_v_6, F32(0x44000000))), 0i32);
        if ((l_done_5) != 0) {
            { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        break 's1;
        }
        F::HudSprite_Show(c, l_word_2, 0i32);
        F::HudSprite_Show(c, l_band_3, 0i32);
        F::HudSprite_Show(c, l_copy_4, 0i32);
        break 's1;
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_UpdateBandB", &PROBE_N, &[], true); }
    r_
}

pub fn HudNotice_UpdateSpinA(c: &mut Ctx, mut p_node_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_UpdateSpinA", &PROBE_N, &[(p_node_1) as u32], false); }
    let mut l_ramp_2: u32 = 0;
    let mut l_word_3: u32 = 0;
    let mut l_streak_4: u32 = 0;
    let mut l_rot_5: F32 = F32(0);
    let mut l_done_6: i32 = 0;
    let mut l_v_7: F32 = F32(0);
    let mut l_one_8: F32 = F32(0);
    let r_ = 'f: {
    l_ramp_2 = (c.m.r32(sym::gHudNotice)).wrapping_add(20);
    l_word_3 = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28));
    l_streak_4 = (l_word_3).wrapping_add((((13i32) as u32)).wrapping_mul(28));
    l_rot_5 = F32(0x00000000);
    's1: {
        let sv9: i32 = { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); (c.m.r32(a_) as i32) };
        'k6: {
        'k5: {
        'k4: {
        'k3: {
        'k2: {
            match sv9 { 0i32 => break 'k2, 1i32 => break 'k3, 2i32 => break 'k4, 3i32 => break 'k5, 4i32 => break 'k6, _ => break 's1, }
        }
        F::HudSprite_Show(c, l_word_3, 1i32);
        { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_word_3, a1_, 4i32, 0i32) };
        F::HudSprite_SetRect(c, l_word_3, (256i32).wrapping_neg(), 256i32, (32i32).wrapping_neg(), 32i32);
        F::HudSprite_SetUv(c, l_word_3, 0i32, 512i32, 0i32, 64i32);
        F::Ramp_Start(c, l_ramp_2, F32(0x3f547ae1), F32(0x3f800000), F32(0x00000000));
        { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        l_done_6 = F::Ramp_Step(c, l_ramp_2);
        l_rot_5 = rt::fmul(rt::fmul({ let a0_ = F32(c.m.r32((l_ramp_2).wrapping_add(16))); F::Port_powf(c, a0_, F32(0x40400000)) }, F32(0x3fc00000)), F32(0x40c90fda));
        'l7: while (((rt::fgt(l_rot_5, F32(0x40490fda)) as i32)) != 0) {
            { let r_ = F32(0x40c90fda); let n_ = rt::fsub(l_rot_5, r_); l_rot_5 = n_; };
        }
        { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_word_3, a1_, 4i32, 0i32) };
        F::HudSprite_SetRect(c, l_word_3, (256i32).wrapping_neg(), 256i32, (32i32).wrapping_neg(), 32i32);
        F::HudSprite_SetUv(c, l_word_3, 0i32, 512i32, 0i32, 64i32);
        l_v_7 = rt::fsub(F32(0x3f800000), { let a0_ = F32(c.m.r32((l_ramp_2).wrapping_add(16))); F::Port_powf(c, a0_, F32(0x40400000)) });
        F::HudSprite_SetColor(c, l_word_3, 128i32, 128i32, 128i32, ((((rt::f2i(rt::fmul(l_v_7, F32(0x43000000)))) as u8)) as i32));
        l_one_8 = F32(0x3f800000);
        l_v_7 = rt::fadd({ let a0_ = F32(c.m.r32((l_ramp_2).wrapping_add(16))); F::Port_powf(c, a0_, F32(0x40400000)) }, l_one_8);
        F::HudSprite_Scale(c, l_word_3, l_v_7, l_v_7);
        if ((l_done_6) != 0) {
            F::Ramp_Start(c, l_ramp_2, l_one_8, F32(0x00000000), l_one_8);
            l_rot_5 = F32(0x00000000);
            { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        break 's1;
        }
        if ((F::Ramp_Step(c, l_ramp_2)) != 0) {
            F::Ramp_Start(c, l_ramp_2, F32(0x3e6b851e), F32(0x3f800000), F32(0x00000000));
            { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        break 's1;
        }
        l_done_6 = F::Ramp_Step(c, l_ramp_2);
        { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_word_3, a1_, 4i32, 0i32) };
        F::HudSprite_SetRect(c, l_word_3, (256i32).wrapping_neg(), 256i32, (32i32).wrapping_neg(), 32i32);
        F::HudSprite_SetUv(c, l_word_3, 0i32, 512i32, 0i32, 64i32);
        { let a4_ = ((((rt::f2i(rt::fmul({ let a0_ = F32(c.m.r32((l_ramp_2).wrapping_add(16))); F::Port_powf(c, a0_, F32(0x40400000)) }, F32(0x43000000)))) as u8)) as i32); F::HudSprite_SetColor(c, l_word_3, 128i32, 128i32, 128i32, a4_) };
        l_v_7 = rt::fsub(F32(0x40000000), { let a0_ = F32(c.m.r32((l_ramp_2).wrapping_add(16))); F::Port_powf(c, a0_, F32(0x40400000)) });
        F::HudSprite_Scale(c, l_word_3, l_v_7, l_v_7);
        if ((l_done_6) != 0) {
            { let v_ = ((0i32) as u8); let a_ = (l_word_3).wrapping_add(27); c.m.w8(a_, v_); };
            { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        break 's1;
        }
        F::HudSprite_Show(c, l_word_3, 0i32);
        F::HudSprite_Show(c, l_streak_4, 0i32);
        break 'f;
    }
    F::HudNode_SetRot(c, p_node_1, l_rot_5);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_UpdateSpinA", &PROBE_N, &[], true); }
    r_
}

pub fn HudNotice_UpdateSpinB(c: &mut Ctx, mut p_node_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_UpdateSpinB", &PROBE_N, &[(p_node_1) as u32], false); }
    let mut l_ramp_2: u32 = 0;
    let mut l_word_3: u32 = 0;
    let mut l_streak_4: u32 = 0;
    let mut l_rot_5: F32 = F32(0);
    let mut l_done_6: i32 = 0;
    let mut l_v_7: F32 = F32(0);
    let mut l_one_8: F32 = F32(0);
    let r_ = 'f: {
    l_ramp_2 = (c.m.r32(sym::gHudNotice)).wrapping_add(20);
    l_word_3 = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28));
    l_streak_4 = (l_word_3).wrapping_add((((13i32) as u32)).wrapping_mul(28));
    l_rot_5 = F32(0x00000000);
    's1: {
        let sv9: i32 = { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); (c.m.r32(a_) as i32) };
        'k6: {
        'k5: {
        'k4: {
        'k3: {
        'k2: {
            match sv9 { 0i32 => break 'k2, 1i32 => break 'k3, 2i32 => break 'k4, 3i32 => break 'k5, 4i32 => break 'k6, _ => break 's1, }
        }
        F::HudSprite_Show(c, l_word_3, 1i32);
        { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_word_3, a1_, 4i32, 0i32) };
        F::HudSprite_SetRect(c, l_word_3, (256i32).wrapping_neg(), 256i32, (32i32).wrapping_neg(), 32i32);
        F::HudSprite_SetUv(c, l_word_3, 0i32, 512i32, 64i32, 128i32);
        F::Ramp_Start(c, l_ramp_2, F32(0x3f547ae1), F32(0x3f800000), F32(0x00000000));
        { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        l_done_6 = F::Ramp_Step(c, l_ramp_2);
        l_rot_5 = rt::fmul(rt::fmul({ let a0_ = F32(c.m.r32((l_ramp_2).wrapping_add(16))); F::Port_powf(c, a0_, F32(0x40400000)) }, F32(0x3fc00000)), F32(0x40c90fda));
        'l7: while (((rt::fgt(l_rot_5, F32(0x40490fda)) as i32)) != 0) {
            { let r_ = F32(0x40c90fda); let n_ = rt::fsub(l_rot_5, r_); l_rot_5 = n_; };
        }
        { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_word_3, a1_, 4i32, 0i32) };
        F::HudSprite_SetRect(c, l_word_3, (256i32).wrapping_neg(), 256i32, (32i32).wrapping_neg(), 32i32);
        F::HudSprite_SetUv(c, l_word_3, 0i32, 512i32, 64i32, 128i32);
        l_v_7 = rt::fsub(F32(0x3f800000), { let a0_ = F32(c.m.r32((l_ramp_2).wrapping_add(16))); F::Port_powf(c, a0_, F32(0x40400000)) });
        F::HudSprite_SetColor(c, l_word_3, 128i32, 128i32, 128i32, ((((rt::f2i(rt::fmul(l_v_7, F32(0x43000000)))) as u8)) as i32));
        l_one_8 = F32(0x3f800000);
        l_v_7 = rt::fadd({ let a0_ = F32(c.m.r32((l_ramp_2).wrapping_add(16))); F::Port_powf(c, a0_, F32(0x40400000)) }, l_one_8);
        F::HudSprite_Scale(c, l_word_3, l_v_7, l_v_7);
        if ((l_done_6) != 0) {
            F::Ramp_Start(c, l_ramp_2, l_one_8, F32(0x00000000), l_one_8);
            l_rot_5 = F32(0x00000000);
            { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        break 's1;
        }
        if ((F::Ramp_Step(c, l_ramp_2)) != 0) {
            F::Ramp_Start(c, l_ramp_2, F32(0x3e6b851e), F32(0x3f800000), F32(0x00000000));
            { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        break 's1;
        }
        l_done_6 = F::Ramp_Step(c, l_ramp_2);
        { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_word_3, a1_, 4i32, 0i32) };
        F::HudSprite_SetRect(c, l_word_3, (256i32).wrapping_neg(), 256i32, (32i32).wrapping_neg(), 32i32);
        F::HudSprite_SetUv(c, l_word_3, 0i32, 512i32, 64i32, 128i32);
        { let a4_ = ((((rt::f2i(rt::fmul({ let a0_ = F32(c.m.r32((l_ramp_2).wrapping_add(16))); F::Port_powf(c, a0_, F32(0x40400000)) }, F32(0x43000000)))) as u8)) as i32); F::HudSprite_SetColor(c, l_word_3, 128i32, 128i32, 128i32, a4_) };
        l_v_7 = rt::fsub(F32(0x40000000), { let a0_ = F32(c.m.r32((l_ramp_2).wrapping_add(16))); F::Port_powf(c, a0_, F32(0x40400000)) });
        F::HudSprite_Scale(c, l_word_3, l_v_7, l_v_7);
        if ((l_done_6) != 0) {
            { let v_ = ((0i32) as u8); let a_ = (l_word_3).wrapping_add(27); c.m.w8(a_, v_); };
            { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        break 's1;
        }
        F::HudSprite_Show(c, l_word_3, 0i32);
        F::HudSprite_Show(c, l_streak_4, 0i32);
        break 'f;
    }
    F::HudNode_SetRot(c, p_node_1, l_rot_5);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_UpdateSpinB", &PROBE_N, &[], true); }
    r_
}

pub fn HudNotice_DrawTrail(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_DrawTrail", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    F::HudGfx_ClearAlpha(c);
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (8i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(((((1i32).wrapping_add(l_i_1)) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    F::HudGfx_CallBegin(c, fid::HudNotice_GsBeginAddAlpha);
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((13i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 1i32) };
    F::HudGfx_CallEnd(c, fid::HudNotice_GsEndAddAlpha);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_DrawTrail", &PROBE_N, &[], true); }
    r_
}

pub fn HudNotice_UpdateTrail(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_UpdateTrail", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_ramp_1: u32 = 0;
    let mut l_word_2: u32 = 0;
    let mut l_part_3: u32 = 0;
    let mut l_streak_4: u32 = 0;
    let mut l_spr_5: u32 = 0;
    let mut l_done_6: i32 = 0;
    let mut l_i_7: i32 = 0;
    let mut l_a_8: F32 = F32(0);
    let mut l_a_9: F32 = F32(0);
    let mut l_v_10: F32 = F32(0);
    let r_ = 'f: {
        l_ramp_1 = (c.m.r32(sym::gHudNotice)).wrapping_add(20);
        l_word_2 = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28));
        l_part_3 = (l_word_2).wrapping_add((((1i32) as u32)).wrapping_mul(28));
        l_streak_4 = (l_word_2).wrapping_add((((13i32) as u32)).wrapping_mul(28));
        's1: {
            let sv11: i32 = { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); (c.m.r32(a_) as i32) };
            'k9: {
            'k8: {
            'k7: {
            'k6: {
            'k5: {
            'k4: {
            'k3: {
            'k2: {
                match sv11 { 0i32 => break 'k2, 1i32 => break 'k3, 2i32 => break 'k4, 3i32 => break 'k5, 4i32 => break 'k6, 5i32 => break 'k7, 6i32 => break 'k8, 7i32 => break 'k9, _ => break 's1, }
            }
            F::HudSprite_Show(c, l_word_2, 0i32);
            { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_word_2, a1_, 5i32, 0i32) };
            F::HudSprite_Center(c, l_word_2);
            F::HudSprite_Show(c, l_streak_4, 0i32);
            {
                l_i_7 = 0i32;
                'l10: loop {
                    if !(((((l_i_7) < (8i32)) as i32)) != 0) { break 'l10; }
                    'c11: {
                        { let v_ = F32(0x3f800000); let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(68)).wrapping_add((((l_i_7) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                        F::HudSprite_Show(c, (l_part_3).wrapping_add((((l_i_7) as u32)).wrapping_mul(28)), 0i32);
                        { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, (l_part_3).wrapping_add((((l_i_7) as u32)).wrapping_mul(28)), a1_, 5i32, 0i32) };
                        F::HudSprite_Center(c, (l_part_3).wrapping_add((((l_i_7) as u32)).wrapping_mul(28)));
                    }
                    { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
                }
            }
            F::Ramp_Start(c, l_ramp_1, F32(0x3f6e147a), F32(0x3f800000), F32(0x00000000));
            { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            {
                l_i_7 = 6i32;
                'l12: loop {
                    if !(((((l_i_7) >= (0i32)) as i32)) != 0) { break 'l12; }
                    'c13: {
                        { let v_ = { let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(68)).wrapping_add((((l_i_7) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(68)).wrapping_add(((((l_i_7).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    }
                    { let o_ = l_i_7; let n_ = (o_).wrapping_sub(1); l_i_7 = n_; };
                }
            }
            l_done_6 = F::Ramp_Step(c, l_ramp_1);
            { let v_ = F32(c.m.r32((l_ramp_1).wrapping_add(16))); let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(68)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            {
                l_i_7 = 0i32;
                'l14: loop {
                    if !(((((l_i_7) < (8i32)) as i32)) != 0) { break 'l14; }
                    'c15: {
                        if (((((l_i_7) > (0i32)) as i32)) != 0) {
                            if (((rt::fne({ let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(68)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, { let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(68)).wrapping_add(((((l_i_7).wrapping_sub(1i32)) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }) as i32)) != 0) {
                                F::HudSprite_Show(c, (l_part_3).wrapping_add((((l_i_7) as u32)).wrapping_mul(28)), 1i32);
                            } else {
                                F::HudSprite_Show(c, (l_part_3).wrapping_add((((l_i_7) as u32)).wrapping_mul(28)), 0i32);
                            }
                        } else {
                            F::HudSprite_Show(c, (l_part_3).wrapping_add((((l_i_7) as u32)).wrapping_mul(28)), 1i32);
                        }
                        { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, (l_part_3).wrapping_add((((l_i_7) as u32)).wrapping_mul(28)), a1_, 5i32, 0i32) };
                        l_spr_5 = (l_part_3).wrapping_add((((l_i_7) as u32)).wrapping_mul(28));
                        F::HudSprite_Center(c, (l_part_3).wrapping_add((((l_i_7) as u32)).wrapping_mul(28)));
                        l_a_8 = rt::fsub(F32(0x3f800000), { let a0_ = rt::fsub(F32(0x3f800000), { let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(68)).wrapping_add((((l_i_7) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }); F::Port_powf(c, a0_, F32(0x40400000)) });
                        { let r_ = F32(0x3f800000); let n_ = rt::fadd(l_a_8, r_); l_a_8 = n_; };
                        F::HudSprite_Scale(c, (l_part_3).wrapping_add((((l_i_7) as u32)).wrapping_mul(28)), l_a_8, l_a_8);
                        { let a4_ = ((((rt::f2i(rt::fmul(rt::fsub(F32(0x3f800000), rt::fsub(F32(0x3f800000), { let a0_ = rt::fsub(F32(0x3f800000), { let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(68)).wrapping_add((((l_i_7) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }); F::Port_powf(c, a0_, F32(0x40400000)) })), F32(0x43000000)))) as u8)) as i32); F::HudSprite_SetColor(c, l_spr_5, 128i32, 128i32, 128i32, a4_) };
                    }
                    { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
                }
            }
            if ((((((l_done_6) != 0) && (((rt::feq({ let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(68)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, { let a_ = ((c.m.r32(sym::gHudNotice)).wrapping_add(68)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }) as i32)) != 0)) as i32)) != 0) {
                F::Ramp_Start(c, l_ramp_1, F32(0x3eb33333), F32(0x00000000), F32(0x3f800000));
                {
                    l_i_7 = 0i32;
                    'l16: loop {
                        if !(((((l_i_7) < (8i32)) as i32)) != 0) { break 'l16; }
                        'c17: {
                            F::HudSprite_Show(c, (l_part_3).wrapping_add((((l_i_7) as u32)).wrapping_mul(28)), 0i32);
                        }
                        { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
                    }
                }
                F::HudSprite_Show(c, l_word_2, 1i32);
                { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            break 's1;
            }
            if ((F::Ramp_Step(c, l_ramp_1)) != 0) {
                F::Ramp_Start(c, l_ramp_1, F32(0x3f4ccccc), F32(0x3f800000), F32(0x00000000));
                { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            break 's1;
            }
            if ((F::Ramp_Step(c, l_ramp_1)) != 0) {
                F::Ramp_Start(c, l_ramp_1, F32(0x3eb33333), F32(0x3f800000), F32(0x00000000));
                { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            break 's1;
            }
            if ((F::Ramp_Step(c, l_ramp_1)) != 0) {
                F::Ramp_Start(c, l_ramp_1, F32(0x3e999999), F32(0x3f800000), F32(0x00000000));
                { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            break 's1;
            }
            if ((F::Ramp_Step(c, l_ramp_1)) != 0) {
                F::Ramp_Start(c, l_ramp_1, F32(0x3e999999), F32(0x3f800000), F32(0x00000000));
                { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            break 's1;
            }
            F::HudSprite_Show(c, l_word_2, 1i32);
            { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_word_2, a1_, 5i32, 0i32) };
            F::HudSprite_Center(c, l_word_2);
            if ((F::Ramp_Step(c, l_ramp_1)) != 0) {
                { let v_ = ((0i32) as u8); let a_ = (l_word_2).wrapping_add(27); c.m.w8(a_, v_); };
                { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            } else {
                { let v_ = ((rt::f2i(rt::fmul({ let a0_ = F32(c.m.r32((l_ramp_1).wrapping_add(16))); F::Port_powf(c, a0_, F32(0x40400000)) }, F32(0x43000000)))) as u8); let a_ = (l_word_2).wrapping_add(27); c.m.w8(a_, v_); };
                l_a_9 = rt::fadd(rt::fmul(rt::fsub(F32(0x3f800000), { let a0_ = F32(c.m.r32((l_ramp_1).wrapping_add(16))); F::Port_powf(c, a0_, F32(0x40400000)) }), F32(0x3e999999)), F32(0x3f800000));
                F::HudSprite_Scale(c, l_word_2, l_a_9, l_a_9);
            }
            break 's1;
            }
            F::HudSprite_Show(c, l_word_2, 0i32);
            F::HudSprite_Show(c, l_streak_4, 0i32);
            {
                l_i_7 = 0i32;
                'l18: loop {
                    if !(((((l_i_7) < (8i32)) as i32)) != 0) { break 'l18; }
                    'c19: {
                        F::HudSprite_Show(c, (l_part_3).wrapping_add((((l_i_7) as u32)).wrapping_mul(28)), 0i32);
                    }
                    { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
                }
            }
            break 'f;
        }
        if ((((({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) {
            { let _ = F::memset(c, fp, 0i32, ((((((8u32) as i32)) as i64 as u64)) as u32)); };
            F::HudSprite_Show(c, l_streak_4, 1i32);
            { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_streak_4, a1_, 7i32, 0i32) };
            F::HudSprite_SetRect(c, l_streak_4, 0i32, 512i32, 0i32, 42i32);
            { let a3_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(42i32); let a4_ = (({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(42i32)).wrapping_add(42i32); F::HudSprite_SetUv(c, l_streak_4, 0i32, 512i32, a3_, a4_) };
            F::HudSprite_Center(c, l_streak_4);
            { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fsub(F32(0x3f800000), F32(c.m.r32((l_ramp_1).wrapping_add(16)))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fneg(rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::i2f(((((c.m.r16(((l_streak_4).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(2))) as i16)) as i32)).wrapping_sub((((c.m.r16(((l_streak_4).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(2))) as i16)) as i32))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::i2f(((((c.m.r16(((l_streak_4).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(2))) as i16)) as i32)).wrapping_sub((((c.m.r16(((l_streak_4).wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(2))) as i16)) as i32)))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let a1_ = rt::f2i(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a2_ = rt::f2i(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::HudSprite_Move(c, l_streak_4, a1_, a2_) };
            F::HudSprite_SetColor(c, l_streak_4, 192i32, 112i32, 96i32, 128i32);
            { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            if ((((({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); (c.m.r32(a_) as i32) }) >= (3i32)) as i32)) != 0) {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
            }
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); (c.m.r32(a_) as i32) }) == (3i32)) as i32)) != 0) {
                { let _ = F::memset(c, fp, 0i32, ((((((8u32) as i32)) as i64 as u64)) as u32)); };
                F::HudSprite_Show(c, l_streak_4, 1i32);
                { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_streak_4, a1_, 7i32, 0i32) };
                F::HudSprite_SetRect(c, l_streak_4, 0i32, 512i32, 0i32, 42i32);
                { let a3_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(42i32); let a4_ = (({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(42i32)).wrapping_add(42i32); F::HudSprite_SetUv(c, l_streak_4, 0i32, 512i32, a3_, a4_) };
                F::HudSprite_Center(c, l_streak_4);
                { let v_ = F::Rand_IntRange(c, (16i32).wrapping_neg(), 16i32); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let a1_ = (c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a2_ = (c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); F::HudSprite_Move(c, l_streak_4, a1_, a2_) };
                F::HudSprite_SetColor(c, l_streak_4, 192i32, 112i32, 96i32, 128i32);
                { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                if ((((({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); (c.m.r32(a_) as i32) }) >= (3i32)) as i32)) != 0) {
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                }
            } else {
                if ((((({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); (c.m.r32(a_) as i32) }) == (4i32)) as i32)) != 0) {
                    { let _ = F::memset(c, fp, 0i32, ((((((8u32) as i32)) as i64 as u64)) as u32)); };
                    F::HudSprite_Show(c, l_streak_4, 1i32);
                    { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_streak_4, a1_, 7i32, 0i32) };
                    F::HudSprite_SetRect(c, l_streak_4, 0i32, 512i32, 0i32, 42i32);
                    { let a3_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(42i32); let a4_ = (({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(42i32)).wrapping_add(42i32); F::HudSprite_SetUv(c, l_streak_4, 0i32, 512i32, a3_, a4_) };
                    F::HudSprite_Center(c, l_streak_4);
                    { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(F32(0x3f800000), F32(c.m.r32((l_ramp_1).wrapping_add(16)))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::i2f(((((c.m.r16(((l_streak_4).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(2))) as i16)) as i32)).wrapping_sub((((c.m.r16(((l_streak_4).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(2))) as i16)) as i32)))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::i2f(((((c.m.r16(((l_streak_4).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(2))) as i16)) as i32)).wrapping_sub((((c.m.r16(((l_streak_4).wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(2))) as i16)) as i32)))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let a1_ = rt::f2i(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a2_ = rt::f2i(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::HudSprite_Move(c, l_streak_4, a1_, a2_) };
                    {
                        l_v_10 = F32(c.m.r32((l_ramp_1).wrapping_add(16)));
                        F::HudSprite_SetColor(c, l_streak_4, ((((rt::f2i(rt::fmul(l_v_10, F32(0x43400000)))) as u8)) as i32), ((((rt::f2i(rt::fmul(l_v_10, F32(0x42e00000)))) as u8)) as i32), ((((rt::f2i(rt::fmul(l_v_10, F32(0x42c00000)))) as u8)) as i32), 128i32);
                    }
                    { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    if ((((({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); (c.m.r32(a_) as i32) }) >= (3i32)) as i32)) != 0) {
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                    }
                } else {
                    F::HudSprite_Show(c, l_streak_4, 0i32);
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_UpdateTrail", &PROBE_N, &[], true); }
    r_
}
