#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn TourMenu_CheckInvite(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TourMenu_CheckInvite", &PROBE_N, &[], false); }
    let mut l_hour_1: i32 = 0;
    let r_ = 'f: {
    l_hour_1 = { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2572); (c.m.r32(a_) as i32) };
    if (((((((((l_hour_1) >= (7i32)) as i32)) != 0) && (((((l_hour_1) <= (12i32)) as i32)) != 0)) as i32)) != 0) {
        if ((((!(((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2568); (c.m.r32(a_) as i32) }) & ((1i32).wrapping_shl((0i32) as u32)))) != 0)) as i32)) != 0) {
            { let r_ = 16i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
            { let r_ = (!(31i32)); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2568); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            { let r_ = (1i32).wrapping_shl((0i32) as u32); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2568); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            { let v_ = ((F::Rand_Range(c, ((3i32) as u32))) as i32); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2576); c.m.w32(a_, (v_) as u32); };
        }
    } else {
        if (((((((((l_hour_1) >= (13i32)) as i32)) != 0) && (((((l_hour_1) <= (18i32)) as i32)) != 0)) as i32)) != 0) {
            if ((((!(((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2568); (c.m.r32(a_) as i32) }) & ((1i32).wrapping_shl((1i32) as u32)))) != 0)) as i32)) != 0) {
                { let r_ = 16i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 1i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
                { let r_ = (!(31i32)); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2568); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = (1i32).wrapping_shl((1i32) as u32); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2568); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = ((F::Rand_Range(c, ((3i32) as u32))) as i32); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2576); c.m.w32(a_, (v_) as u32); };
            }
        } else {
            if (((((((((l_hour_1) >= (19i32)) as i32)) != 0) && (((((l_hour_1) <= (23i32)) as i32)) != 0)) as i32)) != 0) {
                if ((((!(((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2568); (c.m.r32(a_) as i32) }) & ((1i32).wrapping_shl((2i32) as u32)))) != 0)) as i32)) != 0) {
                    { let r_ = 16i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 2i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
                    { let r_ = (!(31i32)); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2568); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                    { let r_ = (1i32).wrapping_shl((2i32) as u32); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2568); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = ((F::Rand_Range(c, ((3i32) as u32))) as i32); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2576); c.m.w32(a_, (v_) as u32); };
                }
            } else {
                if (((((((((l_hour_1) >= (0i32)) as i32)) != 0) && (((((l_hour_1) <= (4i32)) as i32)) != 0)) as i32)) != 0) {
                    if ((((!(((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2568); (c.m.r32(a_) as i32) }) & ((1i32).wrapping_shl((3i32) as u32)))) != 0)) as i32)) != 0) {
                        { let r_ = 16i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        { let v_ = 3i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
                        { let r_ = (!(31i32)); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2568); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                        { let r_ = (1i32).wrapping_shl((3i32) as u32); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2568); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        { let v_ = ((F::Rand_Range(c, ((3i32) as u32))) as i32); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2576); c.m.w32(a_, (v_) as u32); };
                    }
                } else {
                    if (((((((((l_hour_1) >= (5i32)) as i32)) != 0) && (((((l_hour_1) <= (6i32)) as i32)) != 0)) as i32)) != 0) {
                        if ((((!(((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2568); (c.m.r32(a_) as i32) }) & ((1i32).wrapping_shl((4i32) as u32)))) != 0)) as i32)) != 0) {
                            { let r_ = 16i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            { let v_ = 4i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
                            { let r_ = (!(31i32)); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2568); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                            { let r_ = (1i32).wrapping_shl((4i32) as u32); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2568); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            { let v_ = ((F::Rand_Range(c, ((3i32) as u32))) as i32); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2576); c.m.w32(a_, (v_) as u32); };
                        }
                    }
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "TourMenu_CheckInvite", &PROBE_N, &[], true); }
    r_
}

pub fn TourMenu_Init(c: &mut Ctx, mut p_section_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TourMenu_Init", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = F::Heap_Alloc(c, ((328u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gTourMenu; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gTourMenu); F::memset(c, a0_, 0i32, 328u32) }; };
        { let v_ = (c.m.r32(sym::gMenuArc4)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc4)).wrapping_add((((p_section_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = c.m.r32(sym::gTourMenu); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gTourMenu); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.w32(a_, v_); };
        if ((((!(((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2568); (c.m.r32(a_) as i32) }) & (32i32))) != 0)) as i32)) != 0) {
            { let v_ = 7i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2572); c.m.w32(a_, (v_) as u32); };
        }
        F::TourMenu_CheckInvite(c);
        { let v_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = c.m.r32(fp); let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(112); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(116)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(116)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(116)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(116)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(116)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(116)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(116)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(116)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(116)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(116)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(116)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(116)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(116)).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(116)).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(116)).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(116)).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(116)).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(116)).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(116)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(116)).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(116)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(116)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(116)).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(116)).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(116)).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(116)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(116)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(116)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(116)).wrapping_add((((30i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(116)).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gTourMenu)).wrapping_add(116); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let v_ = F::Heap_Alloc(c, 61440i32, ((64i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = F::Heap_Alloc(c, 266240i32, ((32i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(12); c.m.w32(a_, v_); };
        if (((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(264); (c.m.r32(a_) as i32) }) & (16i32))) != 0) {
            { let _ = { let a0_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(304); (c.m.r32(a_) as i32) }).wrapping_add(964i32); let a1_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(8); c.m.r32(a_) }; F::File_LoadSync(c, a0_, a1_, 61440i32) }; };
            { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(8); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(12); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, a1_, ((0i32) as u32)) }; };
            { let v_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(12); c.m.r32(a_) }; let a_ = fp; c.m.w32(a_, v_); };
            { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
            { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(240)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(240)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            { let v_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
            { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
            { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(240)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(240)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            { let v_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
            { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
            { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(240)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        }
        { let a0_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(24)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gTourMenu)).wrapping_add(240); F::Flash_Create(c, a0_, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = c.m.r32(fp); F::IconWin_Init(c, a0_, a1_) };
        F::IconWin_Open(c);
        { let v_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(16); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(20); c.m.w32(a_, v_); };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(16); c.m.r32(a_) }; let _ = 0i32; F::MsgWin_Init(c, a0_, a1_, 0i32) };
        F::MsgWin_Open(c);
        { let v_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(324); c.m.w32(a_, v_); };
        { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
        F::StreamSe_PlayDefault(c, 0i32, 68575i32);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "TourMenu_Init", &PROBE_N, &[], true); }
    r_
}

pub fn TourMenu_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TourMenu_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    F::MsgWin_Term(c);
    F::IconWin_Term(c);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(24)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Destroy(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(12); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(12); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(12); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(8); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(8); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(8); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gTourMenu)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gTourMenu); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gTourMenu; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "TourMenu_Term", &PROBE_N, &[], true); }
    r_
}

pub fn TourMenu_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TourMenu_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_i_1: i32 = 0;
    let mut l_flash_2: u32 = 0;
    let r_ = 'f: {
        { let a0_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(112); c.m.r32(a_) }; F::Sprite_DrawPicture(c, a0_, 0i32, 0i32, 128i32) };
        l_flash_2 = ((c.m.r32(sym::gTourMenu)).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_j_b__str0, fp);
        { let a2_ = (c.m.r32(sym::gTourMenu)).wrapping_add(316); F::FlashAnim_Blink(c, l_flash_2, fp, a2_, 0i32) };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_j_b__str1, fp);
        { let a2_ = (c.m.r32(sym::gTourMenu)).wrapping_add(320); F::FlashAnim_Talk(c, l_flash_2, fp, a2_, 0i32) };
        { let a5_ = { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2572); (c.m.r32(a_) as i32) }; F::Num_DrawChild(c, l_flash_2, sym::menu_j_b__str2, sym::menu_j_b__str3, 2i32, 2i32, a5_, 64i32, 64i32, 1i32, 0i32) };
        {
            l_i_1 = 0i32;
            'l1: loop {
                if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_j_b__str4, &[((l_i_1) as i64 as u64)]); };
                    { let a5_ = { let a_ = (((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(324); c.m.r32(a_) }).wrapping_add(((({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(100))).wrapping_add(16)).wrapping_add((((l_i_1) as u32)).wrapping_mul(12))).wrapping_add(((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2576); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a8_ = (if ((((({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) { 2i32 } else { 0i32 }); F::Num_DrawChild(c, l_flash_2, fp.wrapping_add(28), sym::menu_j_b__str5, 0i32, 7i32, a5_, 32i32, 32i32, a8_, 0i32) };
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
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_j_b__str6, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_j_b__str7, fp);
                    F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                    F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_j_b__str8, fp);
                    F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_j_b__str9, fp);
        { let a2_ = (c.m.r32(sym::gTourMenu)).wrapping_add(292); let a3_ = (c.m.r32(sym::gTourMenu)).wrapping_add(296); F::FlashAnim_Sheet(c, l_flash_2, fp, a2_, a3_, fp.wrapping_add(8), 4i32, 1i32, 4i32) };
        {
            l_i_1 = 0i32;
            'l5: loop {
                if !(((((l_i_1) < (5i32)) as i32)) != 0) { break 'l5; }
                'c6: {
                    { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((l_i_1).wrapping_mul(32i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((l_i_1).wrapping_mul(32i32)).wrapping_add(96i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_j_b__str10, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(28), fp);
                    if ((((((((({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != (1i32)) as i32)) != 0) && ((((!(((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2568); (c.m.r32(a_) as i32) }) & ((((1u32).wrapping_shl((l_i_1) as u32)) as i32)))) != 0)) as i32)) != 0)) as i32)) != 0) {
                        F::Flash_ClipSetColor(c, l_flash_2, fp, F32(0x3f000000));
                    } else {
                        F::Flash_ClipSetColor(c, l_flash_2, fp, F32(0x3f800000));
                    }
                    F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_j_b__str11, fp);
                    F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                    F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_j_b__str12, fp);
                    F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (l_i_1).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_j_b__str13, fp);
                    F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        { let v_ = (rt::rem_i32({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }, 4i32)).wrapping_mul(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_2, sym::menu_j_b__str14, sym::menu_j_b__str15, fp);
        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
        { let a2_ = rt::div_i32({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }, 4i32); F::Flash_ClipSetTex(c, l_flash_2, fp, a2_) };
        { let v_ = (rt::div_i32({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }, 4i32)).wrapping_mul(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = (rt::rem_i32({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }, 4i32)).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_2, sym::menu_j_b__str16, sym::menu_j_b__str17, fp);
        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_j_b__str18, fp);
        { let a2_ = { let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::Flash_ClipSetTex(c, l_flash_2, fp, a2_) };
        {
            l_i_1 = 0i32;
            'l7: loop {
                if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l7; }
                'c8: {
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (l_i_1).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    F::Flash_FindLabel(c, l_flash_2, sym::menu_j_b__str19, (if (((((l_i_1) != (0i32)) as i32)) != 0) { sym::menu_j_b__str20 } else { sym::menu_j_b__str21 }), fp);
                    F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                    { let a3_ = ((({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (1i32)) as i32); { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((a3_) as i64 as u64)) as u8)); } };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        {
            l_i_1 = 0i32;
            'l9: loop {
                if !(((((l_i_1) < (3i32)) as i32)) != 0) { break 'l9; }
                'c10: {
                    { let v_ = (l_i_1).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_j_b__str22, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_j_b__str23, fp);
                    F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                    F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_j_b__str24, fp);
                    F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        { let v_ = (rt::div_i32(({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32), 4i32)).wrapping_mul(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = (rt::rem_i32(({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32), 4i32)).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_j_b__str25, fp);
        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
        {
            l_i_1 = 0i32;
            'l11: loop {
                if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l11; }
                'c12: {
                    { let v_ = (((l_i_1) ^ (1i32))).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_j_b__str26, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(28), fp);
                    F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        {
            l_i_1 = 1i32;
            'l13: loop {
                if !(((((l_i_1) < (3i32)) as i32)) != 0) { break 'l13; }
                'c14: {
                    { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (l_i_1).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_j_b__str27, &[((l_i_1) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(28), fp);
                    F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        {
            l_i_1 = 0i32;
            'l15: loop {
                if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l15; }
                'c16: {
                    { let a0_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(24)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        F::IconWin_Draw(c);
        { let a2_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }; F::MsgWin_Draw(c, 0i32, 0i32, a2_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "TourMenu_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn TourMenu_ClipGoto(c: &mut Ctx, mut p_movie_1: i32, mut p_kind_2: i32, mut p_label_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TourMenu_ClipGoto", &PROBE_N, &[(p_movie_1) as u32, (p_kind_2) as u32, (p_label_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_flash_4: u32 = 0;
    let r_ = 'f: {
        l_flash_4 = ((c.m.r32(sym::gTourMenu)).wrapping_add(24)).wrapping_add((((p_movie_1) as u32)).wrapping_mul(44));
        's1: {
            let sv5: i32 = p_kind_2;
            match sv5 {
                0i32 => {
                    { let _ = { let a2_ = ({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add((((p_kind_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_j_b__str28, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                1i32 => {
                    { let _ = { let a2_ = ({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add((((p_kind_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_j_b__str29, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                2i32 => {
                    { let _ = F::strcpy(c, fp.wrapping_add(8), sym::menu_j_b__str30); };
                    break 's1;
                }
                3i32 => {
                    { let _ = { let a2_ = ({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add((((p_kind_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_j_b__str31, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                4i32 => {
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::Flash_FindLabel(c, l_flash_4, ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, l_flash_4, fp, p_label_3);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "TourMenu_ClipGoto", &PROBE_N, &[], true); }
    r_
}

pub fn TourMenu_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TourMenu_Update", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    if (((((((((((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(264); (c.m.r32(a_) as i32) }) & (8i32))) != 0) && ((((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }) == (3i32)) as i32)) != 0)) as i32)) != 0) && (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0)) as i32)) != 0) {
        { let v_ = ({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(14i32); let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
        { let a0_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(20); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34259i32, a2_) };
    }
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(24)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Advance(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "TourMenu_Update", &PROBE_N, &[], true); }
    r_
}

pub fn TourMenu_Input(c: &mut Ctx, mut p_result_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TourMenu_Input", &PROBE_N, &[(p_result_1) as u32], false); }
    let r_ = 'f: {
    if ((((!(((({ let a_ = (((c.m.r32(sym::gTourMenu)).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f;
    }
    if ((((!(((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(264); (c.m.r32(a_) as i32) }) & (4i32))) != 0)) as i32)) != 0) {
        { let r_ = 4i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    's1: {
        let sv2: i32 = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); (c.m.r32(a_) as i32) };
        match sv2 {
            0i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                    F::TourMenu_ClipGoto(c, 0i32, 0i32, sym::menu_j_b__str32);
                    { let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    if ((((({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                        { let v_ = 1i32; let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    F::TourMenu_ClipGoto(c, 0i32, 0i32, sym::menu_j_b__str33);
                    { let v_ = ({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(14i32); let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                    { let a0_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(20); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34259i32, a2_) };
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                        F::TourMenu_ClipGoto(c, 0i32, 0i32, sym::menu_j_b__str34);
                        { let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= (2i32)) as i32)) != 0) {
                            { let v_ = 0i32; let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        F::TourMenu_ClipGoto(c, 0i32, 0i32, sym::menu_j_b__str35);
                        { let v_ = ({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(14i32); let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                        { let a0_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(20); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34259i32, a2_) };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                            { let a0_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_j_b__str36, ((((1i32) as i64 as u64)) as u8)) };
                            F::TourMenu_ClipGoto(c, 0i32, 0i32, sym::menu_j_b__str37);
                            F::TourMenu_ClipGoto(c, 0i32, 1i32, sym::menu_j_b__str38);
                            if ((((({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                                { let v_ = 17i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                { let a0_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(20); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34259i32, a2_) };
                            }
                            { let v_ = 1i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                                { let v_ = 0i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
                                { let a0_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_j_b__str39, ((((1i32) as i64 as u64)) as u8)) };
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                            }
                        }
                    }
                }
                break 's1;
            }
            1i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                    F::TourMenu_ClipGoto(c, 0i32, 1i32, sym::menu_j_b__str40);
                    { let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    if ((((({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                        { let v_ = (5i32).wrapping_sub(1i32); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    F::TourMenu_ClipGoto(c, 0i32, 1i32, sym::menu_j_b__str41);
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                        F::TourMenu_ClipGoto(c, 0i32, 1i32, sym::menu_j_b__str42);
                        { let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= (5i32)) as i32)) != 0) {
                            { let v_ = 0i32; let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        F::TourMenu_ClipGoto(c, 0i32, 1i32, sym::menu_j_b__str43);
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                            's2: {
                                let sv3: i32 = { let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                                match sv3 {
                                    0i32 => {
                                        if (((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2568); (c.m.r32(a_) as i32) }) & ((((1u32).wrapping_shl(({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)) as i32)))) != 0) {
                                            { let a0_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_j_b__str44, ((((1i32) as i64 as u64)) as u8)) };
                                            F::TourMenu_ClipGoto(c, 0i32, 1i32, sym::menu_j_b__str45);
                                            F::TourMenu_ClipGoto(c, 0i32, 2i32, sym::menu_j_b__str46);
                                            { let v_ = (({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_mul(50i32)).wrapping_add(151i32); let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = 2i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                        } else {
                                            { let v_ = 28i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                            { let a0_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(20); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34259i32, a2_) };
                                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                                        }
                                        break 's2;
                                    }
                                    1i32 => {
                                        { let a0_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_j_b__str47, ((((1i32) as i64 as u64)) as u8)) };
                                        F::TourMenu_ClipGoto(c, 0i32, 1i32, sym::menu_j_b__str48);
                                        F::TourMenu_ClipGoto(c, 0i32, 2i32, sym::menu_j_b__str49);
                                        { let v_ = (({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_mul(50i32)).wrapping_add(151i32); let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); c.m.w32(a_, (v_) as u32); };
                                        { let v_ = 2i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                        break 's2;
                                    }
                                    _ => {
                                    }
                                }
                            }
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                { let a0_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_j_b__str50, ((((1i32) as i64 as u64)) as u8)) };
                                F::TourMenu_ClipGoto(c, 0i32, 1i32, sym::menu_j_b__str51);
                                F::TourMenu_ClipGoto(c, 0i32, 0i32, sym::menu_j_b__str52);
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                            }
                        }
                    }
                }
                break 's1;
            }
            2i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                    { let a0_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_j_b__str53, ((((1i32) as i64 as u64)) as u8)) };
                    { let v_ = 29i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                    if ((((({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != (1i32)) as i32)) != 0) {
                        { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let v_ = { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2576); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    F::TourMenu_ClipGoto(c, 0i32, 3i32, sym::menu_j_b__str54);
                    { let a0_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(20); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34259i32, a2_) };
                    { let v_ = 3i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                        { let a0_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_j_b__str55, ((((1i32) as i64 as u64)) as u8)) };
                        F::TourMenu_ClipGoto(c, 0i32, 2i32, sym::menu_j_b__str56);
                        { let v_ = 1i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                    }
                }
                break 's1;
            }
            3i32 => {
                if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) && ((((({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0)) as i32)) != 0) {
                    F::TourMenu_ClipGoto(c, 0i32, 3i32, sym::menu_j_b__str57);
                    { let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    if ((((({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                        { let v_ = 2i32; let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    F::TourMenu_ClipGoto(c, 0i32, 3i32, sym::menu_j_b__str58);
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                } else {
                    if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) && ((((({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0)) as i32)) != 0) {
                        F::TourMenu_ClipGoto(c, 0i32, 3i32, sym::menu_j_b__str59);
                        { let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= (3i32)) as i32)) != 0) {
                            { let v_ = 0i32; let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        F::TourMenu_ClipGoto(c, 0i32, 3i32, sym::menu_j_b__str60);
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                            { let a0_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_j_b__str61, ((((1i32) as i64 as u64)) as u8)) };
                            { let v_ = 31i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                            if ((((({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != (1i32)) as i32)) != 0) {
                                { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            }
                            F::TourMenu_ClipGoto(c, 0i32, 3i32, sym::menu_j_b__str62);
                            { let a0_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(20); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34259i32, a2_) };
                            { let v_ = 4i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                { let a0_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_j_b__str63, ((((1i32) as i64 as u64)) as u8)) };
                                F::TourMenu_ClipGoto(c, 0i32, 3i32, sym::menu_j_b__str64);
                                { let v_ = (({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_mul(50i32)).wrapping_add(151i32); let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 2i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                            }
                        }
                    }
                }
                break 's1;
            }
            4i32 => {
                if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) && ((((({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0)) as i32)) != 0) {
                    { let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    if ((((({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                        { let v_ = (8i32).wrapping_sub(1i32); let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                } else {
                    if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) && ((((({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0)) as i32)) != 0) {
                        { let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= (8i32)) as i32)) != 0) {
                            { let v_ = 0i32; let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                            { let v_ = 451i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); c.m.w32(a_, (v_) as u32); };
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                { let a0_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_j_b__str65, ((((1i32) as i64 as u64)) as u8)) };
                                { let v_ = 29i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                if ((((({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != (1i32)) as i32)) != 0) {
                                    { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                }
                                { let a0_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(20); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34259i32, a2_) };
                                { let v_ = 3i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
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
    if rt::probing() { crate::host::harness::probe(c, "TourMenu_Input", &PROBE_N, &[], true); }
    r_
}

pub fn TourMenu_UpdateSeq(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TourMenu_UpdateSeq", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        break 'f;
    }
    's1: {
        let sv1: i32 = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); (c.m.r32(a_) as i32) };
        match sv1 {
            1i32 => {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(20); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34259i32, a2_) };
                break 's1;
            }
            3i32 => {
                { let v_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }).wrapping_add(1i32); let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(20); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34259i32, a2_) };
                break 's1;
            }
            5i32 => {
                { let v_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }).wrapping_add(1i32); let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(20); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34259i32, a2_) };
                break 's1;
            }
            7i32 => {
                { let r_ = 32i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2568); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                if (((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(264); (c.m.r32(a_) as i32) }) & (16i32))) != 0) {
                    { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                    F::Voice_StopWithLip(c);
                    { let v_ = 51i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let v_ = 3i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                    { let a0_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(20); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34259i32, a2_) };
                    { let a0_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_j_b__str66, ((((1i32) as i64 as u64)) as u8)) };
                    F::TourMenu_ClipGoto(c, 0i32, 0i32, sym::menu_j_b__str67);
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            2i32 | 4i32 | 6i32 => {
                if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                }
                break 's1;
            }
            51i32 => {
                { let a0_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(24)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
                { let a0_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(24)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_j_b__str68, ((((1i32) as i64 as u64)) as u8)) };
                { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            52i32 => {
                if (((({ let a_ = (((c.m.r32(sym::gTourMenu)).wrapping_add(24)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
                    { let v_ = 53i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            53i32 => {
                { let v_ = 4i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(20); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34259i32, a2_) };
                break 's1;
            }
            54i32 => {
                if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                }
                break 's1;
            }
            55i32 => {
                { let a0_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(24)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_j_b__str69, ((((1i32) as i64 as u64)) as u8)) };
                { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            56i32 => {
                if (((({ let a_ = (((c.m.r32(sym::gTourMenu)).wrapping_add(24)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
                    { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                }
                break 's1;
            }
            57i32 => {
                if ((((!(((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2568); (c.m.r32(a_) as i32) }) & (64i32))) != 0)) as i32)) != 0) {
                    { let v_ = 101i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let v_ = 3i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                    { let a0_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(20); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34259i32, a2_) };
                    { let a0_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_j_b__str70, ((((1i32) as i64 as u64)) as u8)) };
                    F::TourMenu_ClipGoto(c, 0i32, 0i32, sym::menu_j_b__str71);
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            101i32 => {
                { let v_ = 5i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(20); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34259i32, a2_) };
                break 's1;
            }
            103i32 => {
                { let v_ = 9i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(20); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34259i32, a2_) };
                break 's1;
            }
            102i32 | 104i32 => {
                if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                }
                break 's1;
            }
            105i32 => {
                { let r_ = 64i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2568); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 3i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(20); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34259i32, a2_) };
                { let a0_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_j_b__str72, ((((1i32) as i64 as u64)) as u8)) };
                F::TourMenu_ClipGoto(c, 0i32, 0i32, sym::menu_j_b__str73);
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            151i32 => {
                { let v_ = 18i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(20); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34259i32, a2_) };
                break 's1;
            }
            152i32 => {
                if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                            { let v_ = 401i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); c.m.w32(a_, (v_) as u32); };
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                        }
                    }
                }
                break 's1;
            }
            153i32 => {
                { let v_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }).wrapping_add(1i32); let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); c.m.w32(a_, (v_) as u32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(20); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34259i32, a2_) };
                break 's1;
            }
            201i32 => {
                { let v_ = 20i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(20); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34259i32, a2_) };
                break 's1;
            }
            202i32 => {
                if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                            { let v_ = 401i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); c.m.w32(a_, (v_) as u32); };
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                        }
                    }
                }
                break 's1;
            }
            203i32 => {
                { let v_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }).wrapping_add(1i32); let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); c.m.w32(a_, (v_) as u32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(20); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34259i32, a2_) };
                break 's1;
            }
            251i32 => {
                { let v_ = 22i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(20); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34259i32, a2_) };
                break 's1;
            }
            252i32 => {
                if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                            { let v_ = 401i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); c.m.w32(a_, (v_) as u32); };
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                        }
                    }
                }
                break 's1;
            }
            253i32 => {
                { let v_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }).wrapping_add(1i32); let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); c.m.w32(a_, (v_) as u32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(20); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34259i32, a2_) };
                break 's1;
            }
            301i32 => {
                { let v_ = 24i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(20); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34259i32, a2_) };
                break 's1;
            }
            302i32 => {
                if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                            { let v_ = 401i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); c.m.w32(a_, (v_) as u32); };
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                        }
                    }
                }
                break 's1;
            }
            303i32 => {
                { let v_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }).wrapping_add(1i32); let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); c.m.w32(a_, (v_) as u32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(20); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34259i32, a2_) };
                break 's1;
            }
            351i32 => {
                { let v_ = 26i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(20); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34259i32, a2_) };
                break 's1;
            }
            352i32 => {
                if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                            { let v_ = 401i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); c.m.w32(a_, (v_) as u32); };
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                        }
                    }
                }
                break 's1;
            }
            353i32 => {
                { let v_ = ({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }).wrapping_add(1i32); let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); c.m.w32(a_, (v_) as u32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(20); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34259i32, a2_) };
                break 's1;
            }
            401i32 => {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); c.m.w32(a_, (v_) as u32); };
                { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                F::Voice_StopWithLip(c);
                { let a0_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_j_b__str74, ((((1i32) as i64 as u64)) as u8)) };
                F::TourMenu_ClipGoto(c, 0i32, 2i32, sym::menu_j_b__str75);
                { let v_ = 1i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            451i32 => {
                { let v_ = 33i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(20); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34259i32, a2_) };
                break 's1;
            }
            452i32 => {
                if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                }
                break 's1;
            }
            453i32 => {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); c.m.w32(a_, (v_) as u32); };
                { let r_ = 1i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = 2i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 15i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(288); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "TourMenu_UpdateSeq", &PROBE_N, &[], true); }
    r_
}

pub fn TourMenu_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TourMenu_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = 1i32; c.m.w32(fp, (v_) as u32); }
        F::TourMenu_Init(c, p_section_1);
        F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
        'l1: while ((1i32) != 0) {
            F::Gfx_BeginFrame(c);
            F::Pad_Update(c);
            F::Snd_Update(c);
            F::ColorFade_Update(c);
            if ((((!(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0)) as i32)) != 0) {
                F::TourMenu_Update(c);
                F::TourMenu_UpdateSeq(c);
            }
            F::TourMenu_Draw(c);
            F::ColorFade_Draw(c);
            F::Gfx_EndFrame(c, 1i32);
            F::Dma_Flush(c);
            F::File_Stub264D90(c);
            if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
                continue 'l1;
            }
            if ((((((((!(((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(264); (c.m.r32(a_) as i32) }) & (8i32))) != 0)) as i32)) != 0) && (((({ let a_ = (((c.m.r32(sym::gTourMenu)).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
                { let r_ = 8i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                if ((((!(((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2568); (c.m.r32(a_) as i32) }) & (32i32))) != 0)) as i32)) != 0) {
                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); c.m.w32(a_, (v_) as u32); };
                } else {
                    if (((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(264); (c.m.r32(a_) as i32) }) & (16i32))) != 0) {
                        { let v_ = 51i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); c.m.w32(a_, (v_) as u32); };
                    } else {
                        { let v_ = 3i32; let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                        { let a0_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(20); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34259i32, a2_) };
                        { let a0_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_j_b__str76, ((((1i32) as i64 as u64)) as u8)) };
                        F::TourMenu_ClipGoto(c, 0i32, 0i32, sym::menu_j_b__str77);
                    }
                }
            }
            if ((F::ColorFade_IsFadingOut(c)) != 0) {
                F::Voice_FadeOutStep(c, 0i32);
                F::Bgm_FadeOutStep(c);
                F::StreamSe_FadeOutStep(c, 0i32);
                continue 'l1;
            }
            if ((F::ColorFade_IsOutDone(c)) != 0) {
                break 'l1;
            }
            if (((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(264); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(288); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) == ((1i32).wrapping_neg())) as i32)) != 0) {
                    F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                    { let v_ = { let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(124)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(124)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(124)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ({ let a_ = ((c.m.r32(sym::gTourMenu)).wrapping_add(268)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(124)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                }
            } else {
                if ((((({ let a_ = (c.m.r32(sym::gTourMenu)).wrapping_add(308); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::TourMenu_Input(c, fp);
                }
            }
        }
        F::TourMenu_Term(c);
        F::Dma_ResetBuffers(c);
        break 'f (c.m.r32(fp) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "TourMenu_Run", &PROBE_N, &[], true); }
    r_
}
