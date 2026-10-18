#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Option_Init(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Option_Init", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(72), v_); }
        { let v_ = F::Heap_Alloc(c, ((556u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gOption; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gOption); F::memset(c, a0_, 0i32, 556u32) }; };
        { let v_ = (c.m.r32(sym::gMenuArc10)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc10)).wrapping_add((((p_section_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = c.m.r32(sym::gOption); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gOption); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.w32(a_, v_); };
        'l1: loop {
            'c2: {
                { let v_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((30i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(72); c.m.w32(a_, v_); };
                { let a1_ = c.m.r32(fp.wrapping_add(72)); let a2_ = c.m.r32(fp.wrapping_add(72)); F::Res_RelocateOffsets(c, fp.wrapping_add(72), a1_, a2_) };
                l_i_2 = 0i32;
                { let v_ = c.m.r32(fp.wrapping_add(72)); let a_ = (c.m.r32(sym::gOption)).wrapping_add(52); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(72); c.m.w32(a_, v_); };
                { let a1_ = c.m.r32(fp.wrapping_add(72)); let a2_ = c.m.r32(fp.wrapping_add(72)); F::Res_RelocateOffsets(c, fp.wrapping_add(72), a1_, a2_) };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(72); c.m.w32(a_, v_); };
                { let a1_ = c.m.r32(fp.wrapping_add(72)); let a2_ = c.m.r32(fp.wrapping_add(72)); F::Res_RelocateOffsets(c, fp.wrapping_add(72), a1_, a2_) };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(72); c.m.w32(a_, v_); };
                { let a1_ = c.m.r32(fp.wrapping_add(72)); let a2_ = c.m.r32(fp.wrapping_add(72)); F::Res_RelocateOffsets(c, fp.wrapping_add(72), a1_, a2_) };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(72); c.m.w32(a_, v_); };
                { let a1_ = c.m.r32(fp.wrapping_add(72)); let a2_ = c.m.r32(fp.wrapping_add(72)); F::Res_RelocateOffsets(c, fp.wrapping_add(72), a1_, a2_) };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((32i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((31i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(72); c.m.w32(a_, v_); };
                { let a1_ = c.m.r32(fp.wrapping_add(72)); let a2_ = c.m.r32(fp.wrapping_add(72)); F::Res_RelocateOffsets(c, fp.wrapping_add(72), a1_, a2_) };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((33i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(72); c.m.w32(a_, v_); };
                { let a1_ = c.m.r32(fp.wrapping_add(72)); let a2_ = c.m.r32(fp.wrapping_add(72)); F::Res_RelocateOffsets(c, fp.wrapping_add(72), a1_, a2_) };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(72); c.m.w32(a_, v_); };
                { let a1_ = c.m.r32(fp.wrapping_add(72)); let a2_ = c.m.r32(fp.wrapping_add(72)); F::Res_RelocateOffsets(c, fp.wrapping_add(72), a1_, a2_) };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(72); c.m.w32(a_, v_); };
                { let a1_ = c.m.r32(fp.wrapping_add(72)); let a2_ = c.m.r32(fp.wrapping_add(72)); F::Res_RelocateOffsets(c, fp.wrapping_add(72), a1_, a2_) };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(72); c.m.w32(a_, v_); };
                { let a1_ = c.m.r32(fp.wrapping_add(72)); let a2_ = c.m.r32(fp.wrapping_add(72)); F::Res_RelocateOffsets(c, fp.wrapping_add(72), a1_, a2_) };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(72); c.m.w32(a_, v_); };
                { let a1_ = c.m.r32(fp.wrapping_add(72)); let a2_ = c.m.r32(fp.wrapping_add(72)); F::Res_RelocateOffsets(c, fp.wrapping_add(72), a1_, a2_) };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((37i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(72); c.m.w32(a_, v_); };
                { let a1_ = c.m.r32(fp.wrapping_add(72)); let a2_ = c.m.r32(fp.wrapping_add(72)); F::Res_RelocateOffsets(c, fp.wrapping_add(72), a1_, a2_) };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((36i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((38i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(72); c.m.w32(a_, v_); };
                { let a1_ = c.m.r32(fp.wrapping_add(72)); let a2_ = c.m.r32(fp.wrapping_add(72)); F::Res_RelocateOffsets(c, fp.wrapping_add(72), a1_, a2_) };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((46i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((47i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((48i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((50i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(72); c.m.w32(a_, v_); };
                { let a1_ = c.m.r32(fp.wrapping_add(72)); let a2_ = c.m.r32(fp.wrapping_add(72)); F::Res_RelocateOffsets(c, fp.wrapping_add(72), a1_, a2_) };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((49i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(72); c.m.w32(a_, v_); };
                { let a1_ = c.m.r32(fp.wrapping_add(72)); let a2_ = c.m.r32(fp.wrapping_add(72)); F::Res_RelocateOffsets(c, fp.wrapping_add(72), a1_, a2_) };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((30i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((34i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(72); c.m.w32(a_, v_); };
                { let a1_ = c.m.r32(fp.wrapping_add(72)); let a2_ = c.m.r32(fp.wrapping_add(72)); F::Res_RelocateOffsets(c, fp.wrapping_add(72), a1_, a2_) };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(72); c.m.w32(a_, v_); };
                { let a1_ = c.m.r32(fp.wrapping_add(72)); let a2_ = c.m.r32(fp.wrapping_add(72)); F::Res_RelocateOffsets(c, fp.wrapping_add(72), a1_, a2_) };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(72); c.m.w32(a_, v_); };
                { let a1_ = c.m.r32(fp.wrapping_add(72)); let a2_ = c.m.r32(fp.wrapping_add(72)); F::Res_RelocateOffsets(c, fp.wrapping_add(72), a1_, a2_) };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(72); c.m.w32(a_, v_); };
                { let a1_ = c.m.r32(fp.wrapping_add(72)); let a2_ = c.m.r32(fp.wrapping_add(72)); F::Res_RelocateOffsets(c, fp.wrapping_add(72), a1_, a2_) };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(72); c.m.w32(a_, v_); };
                { let a1_ = c.m.r32(fp.wrapping_add(72)); let a2_ = c.m.r32(fp.wrapping_add(72)); F::Res_RelocateOffsets(c, fp.wrapping_add(72), a1_, a2_) };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(72); c.m.w32(a_, v_); };
                { let a1_ = c.m.r32(fp.wrapping_add(72)); let a2_ = c.m.r32(fp.wrapping_add(72)); F::Res_RelocateOffsets(c, fp.wrapping_add(72), a1_, a2_) };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((40i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((39i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((42i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((9i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((41i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(72); c.m.w32(a_, v_); };
                { let a1_ = c.m.r32(fp.wrapping_add(72)); let a2_ = c.m.r32(fp.wrapping_add(72)); F::Res_RelocateOffsets(c, fp.wrapping_add(72), a1_, a2_) };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((45i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(72); c.m.w32(a_, v_); };
                { let a1_ = c.m.r32(fp.wrapping_add(72)); let a2_ = c.m.r32(fp.wrapping_add(72)); F::Res_RelocateOffsets(c, fp.wrapping_add(72), a1_, a2_) };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((43i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(72); c.m.w32(a_, v_); };
                { let a1_ = c.m.r32(fp.wrapping_add(72)); let a2_ = c.m.r32(fp.wrapping_add(72)); F::Res_RelocateOffsets(c, fp.wrapping_add(72), a1_, a2_) };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((44i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(72); c.m.w32(a_, v_); };
                { let a1_ = c.m.r32(fp.wrapping_add(72)); let a2_ = c.m.r32(fp.wrapping_add(72)); F::Res_RelocateOffsets(c, fp.wrapping_add(72), a1_, a2_) };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((34i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(72); c.m.w32(a_, v_); };
                { let a1_ = c.m.r32(fp.wrapping_add(72)); let a2_ = c.m.r32(fp.wrapping_add(72)); F::Res_RelocateOffsets(c, fp.wrapping_add(72), a1_, a2_) };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(72); c.m.w32(a_, v_); };
                { let a1_ = c.m.r32(fp.wrapping_add(72)); let a2_ = c.m.r32(fp.wrapping_add(72)); F::Res_RelocateOffsets(c, fp.wrapping_add(72), a1_, a2_) };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((35i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(72); c.m.w32(a_, v_); };
                { let a1_ = c.m.r32(fp.wrapping_add(72)); let a2_ = c.m.r32(fp.wrapping_add(72)); F::Res_RelocateOffsets(c, fp.wrapping_add(72), a1_, a2_) };
                { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(72))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gOption)).wrapping_add(56)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = (({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((37i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32))).wrapping_add(((16i32) as u32)); let a_ = (c.m.r32(sym::gOption)).wrapping_add(412); c.m.w32(a_, v_); };
                { let v_ = (({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((37i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) as i32); let a_ = (c.m.r32(sym::gOption)).wrapping_add(416); c.m.w32(a_, (v_) as u32); };
                { let a_ = (c.m.r32(sym::gOption)).wrapping_add(416); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                { let r_ = 4i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(416); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                { let a0_ = (c.m.r32(sym::gOption)).wrapping_add(416); let a1_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(412); c.m.r32(a_) }; F::BgmList_ApplyUnlocks(c, a0_, a1_) };
                { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gOption)).wrapping_add(56); F::Flash_Create(c, a0_, a1_, a2_) };
                { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
                { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_FindLabel(c, a0_, ((0i32) as u32), fp.wrapping_add(8), fp) };
                { let a0_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((35i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::Dialog_Init(c, a0_, ((0i32) as u32), 0i32) };
                { let v_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((38i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gOption)).wrapping_add(424); c.m.w32(a_, v_); };
                { let _ = 1i32; F::McFlow_Init(c) };
                F::McFlow_SetDoneCb(c, 0i32, fid::Option_OnSaved, ((((((0i32) as u32)) as u64)) as i32));
                { let v_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((36i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gOption)).wrapping_add(428); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(72); c.m.w32(a_, v_); };
                { let a1_ = c.m.r32(fp.wrapping_add(72)); let a2_ = c.m.r32(fp.wrapping_add(72)); F::Res_RelocateOffsets(c, fp.wrapping_add(72), a1_, a2_) };
                { let a0_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((32i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = c.m.r32(fp.wrapping_add(72)); F::IconWin_Init(c, a0_, a1_) };
                F::IconWin_Open(c);
                { let v_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((33i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gOption)).wrapping_add(420); c.m.w32(a_, v_); };
                { let a0_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((31i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(420); c.m.r32(a_) }; let a3_ = (((c.m.r32(sym::gOption)).wrapping_add(432)) as i32); let _ = a3_; F::MsgWin_Init(c, a0_, a1_, 0i32) };
                F::MsgWin_Open(c);
                { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); c.m.w32(a_, (v_) as u32); };
                { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                { let v_ = { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5800); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(272); c.m.w32(a_, (v_) as u32); };
                { let v_ = { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5804); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(276); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(268); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(308); c.m.w32(a_, (v_) as u32); };
                { let v_ = 6i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(288); c.m.w32(a_, (v_) as u32); };
                { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gOption)).wrapping_add(372); c.m.w32(a_, (v_) as u32); };
                { let v_ = 5i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(352); c.m.w32(a_, (v_) as u32); };
                {
                    'l3: loop {
                        if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l3; }
                        'c4: {
                            { let v_ = 2i32; let a_ = (((c.m.r32(sym::gOption)).wrapping_add(492)).wrapping_add((((l_i_2) as u32)).wrapping_mul(32))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 1i32; let a_ = (((c.m.r32(sym::gOption)).wrapping_add(492)).wrapping_add((((l_i_2) as u32)).wrapping_mul(32))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 0i32; let a_ = (((c.m.r32(sym::gOption)).wrapping_add(492)).wrapping_add((((l_i_2) as u32)).wrapping_mul(32))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 3i32; let a_ = (((c.m.r32(sym::gOption)).wrapping_add(492)).wrapping_add((((l_i_2) as u32)).wrapping_mul(32))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 4i32; let a_ = (((c.m.r32(sym::gOption)).wrapping_add(492)).wrapping_add((((l_i_2) as u32)).wrapping_mul(32))).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 5i32; let a_ = (((c.m.r32(sym::gOption)).wrapping_add(492)).wrapping_add((((l_i_2) as u32)).wrapping_mul(32))).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 6i32; let a_ = (((c.m.r32(sym::gOption)).wrapping_add(492)).wrapping_add((((l_i_2) as u32)).wrapping_mul(32))).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 7i32; let a_ = (((c.m.r32(sym::gOption)).wrapping_add(492)).wrapping_add((((l_i_2) as u32)).wrapping_mul(32))).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                    }
                }
            }
            if !((0i32) != 0) { break 'l1; }
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Option_Init", &PROBE_N, &[], true); }
    r_
}

pub fn Option_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Option_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let r_ = 'f: {
    { let _ = F::Option_Init(c, p_section_1); };
    F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
    'l1: while (((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(392); (c.m.r32(a_) as i32) }) != 0) || ((F::Option_Input(c)) != 0)) as i32)) != 0) {
        F::Gfx_BeginFrame(c);
        F::Pad_Update(c);
        F::Snd_Update(c);
        F::ColorFade_Update(c);
        F::Option_Draw(c);
        F::File_Stub264D90(c);
        F::Gfx_EndFrame(c, 1i32);
        F::Dma_Flush(c);
    }
    F::Option_Term(c);
    F::Dma_ResetBuffers(c);
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Option_Run", &PROBE_N, &[], true); }
    r_
}

pub fn Option_Input(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Option_Input", &PROBE_N, &[], false); }
    let mut l_ret_1: i32 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_cur_3: i32 = 0;
    let mut l_o_4: u32 = 0;
    let r_ = 'f: {
    l_ret_1 = 1i32;
    if ((((!(((({ let a_ = (((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    's1: {
        let sv5: i32 = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) };
        match sv5 {
            (-1i32) => {
                if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(396); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::Option_PlateGoto(c, 0i32, 0i32, sym::menu_x_c__str0);
                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(396); c.m.w32(a_, (v_) as u32); };
                    { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(428); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34618i32, a2_) };
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            0i32 => {
                if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(400); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                        { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                        { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(428); c.m.r32(a_) }; F::Voice_PlayWithSubtitle(c, a0_, 34618i32, 1i32) };
                        { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(400); c.m.w32(a_, (v_) as u32); };
                    }
                }
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) == (5i32)) as i32)) != 0) {
                        F::Option_PlateGoto(c, 0i32, 3i32, sym::menu_x_c__str1);
                    } else {
                        F::Option_PlateGoto(c, 0i32, 0i32, sym::menu_x_c__str2);
                    }
                    { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) <= (0i32)) as i32)) != 0) {
                        { let v_ = 5i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); c.m.w32(a_, (v_) as u32); };
                    }
                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) == (5i32)) as i32)) != 0) {
                        F::Option_PlateGoto(c, 0i32, 3i32, sym::menu_x_c__str3);
                    } else {
                        F::Option_PlateGoto(c, 0i32, 0i32, sym::menu_x_c__str4);
                    }
                    { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                    { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                    { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(428); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34618i32, a2_) };
                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(400); c.m.w32(a_, (v_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) == (5i32)) as i32)) != 0) {
                            F::Option_PlateGoto(c, 0i32, 3i32, sym::menu_x_c__str5);
                        } else {
                            F::Option_PlateGoto(c, 0i32, 0i32, sym::menu_x_c__str6);
                        }
                        { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) > (5i32)) as i32)) != 0) {
                            { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); c.m.w32(a_, (v_) as u32); };
                        }
                        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) == (5i32)) as i32)) != 0) {
                            F::Option_PlateGoto(c, 0i32, 3i32, sym::menu_x_c__str7);
                        } else {
                            F::Option_PlateGoto(c, 0i32, 0i32, sym::menu_x_c__str8);
                        }
                        { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                        { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                        { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(428); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34618i32, a2_) };
                        { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(400); c.m.w32(a_, (v_) as u32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                            if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) != (5i32)) as i32)) != 0) {
                                F::Option_PlateGoto(c, 0i32, 0i32, sym::menu_x_c__str9);
                            }
                            { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                            { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((1i32) as i64 as u64)])) as i32); };
                            if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) {
                                if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(400); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                                    { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(428); c.m.r32(a_) }; F::Voice_PlayWithSubtitle(c, a0_, 34618i32, 1i32) };
                                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(400); c.m.w32(a_, (v_) as u32); };
                                }
                                { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_x_c__str10, ((((1i32) as i64 as u64)) as u8)) };
                                { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                                F::Option_PlateGoto(c, 0i32, 4i32, sym::menu_x_c__str11);
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                                F::Option_PlateGoto(c, 0i32, 4i32, sym::menu_x_c__str12);
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
                            } else {
                                if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) {
                                    { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_x_c__str13, ((((1i32) as i64 as u64)) as u8)) };
                                    F::Option_PlateGoto(c, 0i32, 0i32, sym::menu_x_c__str14);
                                    { let v_ = 7i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                                    F::Option_PlateGoto(c, 0i32, 1i32, sym::menu_x_c__str15);
                                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(268); c.m.w32(a_, (v_) as u32); };
                                    { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(428); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34618i32, a2_) };
                                } else {
                                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) == (3i32)) as i32)) != 0) {
                                        { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_x_c__str16, ((((1i32) as i64 as u64)) as u8)) };
                                        F::Option_PlateGoto(c, 0i32, 0i32, sym::menu_x_c__str17);
                                        { let v_ = 12i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); c.m.w32(a_, (v_) as u32); };
                                        { let v_ = 11i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                                        F::Option_PlateGoto(c, 0i32, 2i32, sym::menu_x_c__str18);
                                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                        { let v_ = 2i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(268); c.m.w32(a_, (v_) as u32); };
                                        { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(428); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34618i32, a2_) };
                                    } else {
                                        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) == (4i32)) as i32)) != 0) {
                                            { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_x_c__str19, ((((1i32) as i64 as u64)) as u8)) };
                                            F::Option_PlateGoto(c, 0i32, 0i32, sym::menu_x_c__str20);
                                            { let v_ = 17i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = 23i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                                            F::Option_PlateGoto(c, 0i32, 10i32, sym::menu_x_c__str21);
                                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = 3i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(268); c.m.w32(a_, (v_) as u32); };
                                            { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(428); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34618i32, a2_) };
                                        }
                                    }
                                }
                            }
                            { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(400); c.m.w32(a_, (v_) as u32); };
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) == (5i32)) as i32)) != 0) {
                                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(408); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                        { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(404); c.m.w32(a_, (v_) as u32); };
                                        { let v_ = (2i32).wrapping_neg(); let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                                        F::McFlow_Start(c, 4i32);
                                        { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(392); c.m.w32(a_, (v_) as u32); };
                                        { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((2i32) as i64 as u64)])) as i32); };
                                        break 's1;
                                    }
                                    F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                                    { let v_ = 5i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                                } else {
                                    F::Option_PlateGoto(c, 0i32, 0i32, sym::menu_x_c__str22);
                                    { let v_ = 5i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                                    F::Option_PlateGoto(c, 0i32, 3i32, sym::menu_x_c__str23);
                                }
                                { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((2i32) as i64 as u64)])) as i32); };
                                { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(428); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34618i32, a2_) };
                            }
                        }
                    }
                }
                break 's1;
            }
            2i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) == (10i32)) as i32)) != 0) {
                        F::Option_PlateGoto(c, 0i32, 3i32, sym::menu_x_c__str24);
                    } else {
                        F::Option_PlateGoto(c, 0i32, 1i32, sym::menu_x_c__str25);
                    }
                    { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) < (7i32)) as i32)) != 0) {
                        { let v_ = 10i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); c.m.w32(a_, (v_) as u32); };
                    }
                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) == (10i32)) as i32)) != 0) {
                        F::Option_PlateGoto(c, 0i32, 3i32, sym::menu_x_c__str26);
                    } else {
                        F::Option_PlateGoto(c, 0i32, 1i32, sym::menu_x_c__str27);
                    }
                    { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                    { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                    { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(428); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34618i32, a2_) };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) == (10i32)) as i32)) != 0) {
                            F::Option_PlateGoto(c, 0i32, 3i32, sym::menu_x_c__str28);
                        } else {
                            F::Option_PlateGoto(c, 0i32, 1i32, sym::menu_x_c__str29);
                        }
                        { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) > (10i32)) as i32)) != 0) {
                            { let v_ = 7i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); c.m.w32(a_, (v_) as u32); };
                        }
                        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) == (10i32)) as i32)) != 0) {
                            F::Option_PlateGoto(c, 0i32, 3i32, sym::menu_x_c__str30);
                        } else {
                            F::Option_PlateGoto(c, 0i32, 1i32, sym::menu_x_c__str31);
                        }
                        { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                        { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                        { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(428); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34618i32, a2_) };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                            l_cur_3 = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) };
                            if (((((l_cur_3) == (7i32)) as i32)) != 0) {
                                F::Option_PlateGoto(c, 0i32, 1i32, sym::menu_x_c__str32);
                                { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                                { let v_ = { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5780); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(332); c.m.w32(a_, (v_) as u32); };
                                { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(332); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(376); c.m.w32(a_, (v_) as u32); };
                                { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_x_c__str33, ((((1i32) as i64 as u64)) as u8)) };
                                { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((1i32) as i64 as u64)])) as i32); };
                            } else {
                                if (((((l_cur_3) == (8i32)) as i32)) != 0) {
                                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                                    F::Option_PlateGoto(c, 0i32, 4i32, sym::menu_x_c__str34);
                                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                                    F::Option_PlateGoto(c, 0i32, 4i32, sym::menu_x_c__str35);
                                    { let v_ = { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5784); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                                    F::Option_PlateGoto(c, 0i32, 4i32, sym::menu_x_c__str36);
                                    { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(376); c.m.w32(a_, (v_) as u32); };
                                    F::Option_PlateGoto(c, 0i32, 1i32, sym::menu_x_c__str37);
                                    { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 4i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
                                    { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_x_c__str38, ((((1i32) as i64 as u64)) as u8)) };
                                    { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((1i32) as i64 as u64)])) as i32); };
                                } else {
                                    if (((((l_cur_3) == (9i32)) as i32)) != 0) {
                                        { let v_ = { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5788); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(324); c.m.w32(a_, (v_) as u32); };
                                        { let v_ = { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5792); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(328); c.m.w32(a_, (v_) as u32); };
                                        F::Option_PlateGoto(c, 0i32, 1i32, sym::menu_x_c__str39);
                                        F::IconWin_Close(c);
                                        F::MsgWin_Close(c);
                                        { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                                        { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_x_c__str40, ((((1i32) as i64 as u64)) as u8)) };
                                        { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((1i32) as i64 as u64)])) as i32); };
                                    } else {
                                        if (((((l_cur_3) == (10i32)) as i32)) != 0) {
                                            F::Option_PlateGoto(c, 0i32, 3i32, sym::menu_x_c__str41);
                                            { let v_ = l_cur_3; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                                        }
                                    }
                                }
                            }
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                F::Option_PlateGoto(c, 0i32, 1i32, sym::menu_x_c__str42);
                                F::Option_PlateGoto(c, 0i32, 3i32, sym::menu_x_c__str43);
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 2i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); c.m.w32(a_, (v_) as u32); };
                                { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(268); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                F::Option_PlateGoto(c, 0i32, 0i32, sym::menu_x_c__str44);
                                { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_x_c__str45, ((((1i32) as i64 as u64)) as u8)) };
                                { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((2i32) as i64 as u64)])) as i32); };
                                { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(428); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34618i32, a2_) };
                            }
                        }
                    }
                }
                break 's1;
            }
            7i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1i32) as u32)))) != 0) {
                    { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(332); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(336); c.m.w32(a_, (v_) as u32); };
                    { let a_ = (c.m.r32(sym::gOption)).wrapping_add(332); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(332); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                        { let v_ = 2i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(332); c.m.w32(a_, (v_) as u32); };
                    }
                    { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_x_c__str46, ((((1i32) as i64 as u64)) as u8)) };
                    { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((2i32) as u32)))) != 0) {
                        { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(332); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(336); c.m.w32(a_, (v_) as u32); };
                        { let a_ = (c.m.r32(sym::gOption)).wrapping_add(332); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(332); (c.m.r32(a_) as i32) }) > (2i32)) as i32)) != 0) {
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(332); c.m.w32(a_, (v_) as u32); };
                        }
                        { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_x_c__str47, ((((1i32) as i64 as u64)) as u8)) };
                        { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                            if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(376); (c.m.r32(a_) as i32) }) != ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(332); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(408); c.m.w32(a_, (v_) as u32); };
                            }
                            { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(332); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5780); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 2i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                            F::Option_PlateGoto(c, 0i32, 1i32, sym::menu_x_c__str48);
                            { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_x_c__str49, ((((1i32) as i64 as u64)) as u8)) };
                            { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((1i32) as i64 as u64)])) as i32); };
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                { let v_ = 2i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                                F::Option_PlateGoto(c, 0i32, 1i32, sym::menu_x_c__str50);
                                { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_x_c__str51, ((((1i32) as i64 as u64)) as u8)) };
                                { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((2i32) as i64 as u64)])) as i32); };
                                { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(428); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34618i32, a2_) };
                            }
                        }
                    }
                }
                break 's1;
            }
            9i32 => {
                if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) && ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5788); (c.m.r32(a_) as i32) }) > ((16i32).wrapping_neg())) as i32)) != 0)) as i32)) != 0) {
                    { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5788); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) && ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5788); (c.m.r32(a_) as i32) }) < (16i32)) as i32)) != 0)) as i32)) != 0) {
                        { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5788); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    } else {
                        if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) && ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5792); (c.m.r32(a_) as i32) }) > ((16i32).wrapping_neg())) as i32)) != 0)) as i32)) != 0) {
                            { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5792); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                        } else {
                            if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) && ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5792); (c.m.r32(a_) as i32) }) < (16i32)) as i32)) != 0)) as i32)) != 0) {
                                { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5792); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            } else {
                                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((4096i32) as u32)))) != 0) {
                                    { let v_ = 11i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                                } else {
                                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                                        if ((((((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5788); (c.m.r32(a_) as i32) }) != ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(324); (c.m.r32(a_) as i32) })) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5792); (c.m.r32(a_) as i32) }) != ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(328); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
                                            { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(408); c.m.w32(a_, (v_) as u32); };
                                        }
                                        F::Option_PlateGoto(c, 0i32, 1i32, sym::menu_x_c__str52);
                                        { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_x_c__str53, ((((1i32) as i64 as u64)) as u8)) };
                                        { let v_ = 2i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                                        F::IconWin_Open(c);
                                        F::MsgWin_Open(c);
                                        { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((1i32) as i64 as u64)])) as i32); };
                                    } else {
                                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                            { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_x_c__str54, ((((1i32) as i64 as u64)) as u8)) };
                                            { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(324); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5788); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(328); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5792); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = 2i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                                            F::MsgWin_Open(c);
                                            F::IconWin_Open(c);
                                            F::Option_PlateGoto(c, 0i32, 1i32, sym::menu_x_c__str55);
                                            { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((2i32) as i64 as u64)])) as i32); };
                                            { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(428); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34618i32, a2_) };
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                break 's1;
            }
            3i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) >= (16i32)) as i32)) != 0) {
                        F::Option_PlateGoto(c, 0i32, 3i32, sym::menu_x_c__str56);
                    } else {
                        F::Option_PlateGoto(c, 0i32, 2i32, sym::menu_x_c__str57);
                    }
                    { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) < (12i32)) as i32)) != 0) {
                        { let v_ = 16i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); c.m.w32(a_, (v_) as u32); };
                    }
                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) >= (16i32)) as i32)) != 0) {
                        F::Option_PlateGoto(c, 0i32, 3i32, sym::menu_x_c__str58);
                    } else {
                        F::Option_PlateGoto(c, 0i32, 2i32, sym::menu_x_c__str59);
                    }
                    { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) == (12i32)) as i32)) != 0) {
                        { let v_ = 11i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                    } else {
                        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) == (13i32)) as i32)) != 0) {
                            { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                        } else {
                            if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) == (14i32)) as i32)) != 0) {
                                { let v_ = 17i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                            } else {
                                if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) == (16i32)) as i32)) != 0) {
                                    { let v_ = 10i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                                } else {
                                    { let v_ = 18i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                                }
                            }
                        }
                    }
                    { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(428); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34618i32, a2_) };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) >= (16i32)) as i32)) != 0) {
                            F::Option_PlateGoto(c, 0i32, 3i32, sym::menu_x_c__str60);
                        } else {
                            F::Option_PlateGoto(c, 0i32, 2i32, sym::menu_x_c__str61);
                        }
                        { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) > (16i32)) as i32)) != 0) {
                            { let v_ = 12i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); c.m.w32(a_, (v_) as u32); };
                        }
                        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) >= (16i32)) as i32)) != 0) {
                            F::Option_PlateGoto(c, 0i32, 3i32, sym::menu_x_c__str62);
                        } else {
                            F::Option_PlateGoto(c, 0i32, 2i32, sym::menu_x_c__str63);
                        }
                        { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) == (12i32)) as i32)) != 0) {
                            { let v_ = 11i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                        } else {
                            if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) == (13i32)) as i32)) != 0) {
                                { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                            } else {
                                if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) == (14i32)) as i32)) != 0) {
                                    { let v_ = 17i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                                } else {
                                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) == (16i32)) as i32)) != 0) {
                                        { let v_ = 10i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                                    } else {
                                        { let v_ = 18i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                                    }
                                }
                            }
                        }
                        { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(428); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34618i32, a2_) };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                            if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) == (12i32)) as i32)) != 0) {
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                                F::Option_PlateGoto(c, 0i32, 4i32, sym::menu_x_c__str64);
                                { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                                F::Option_PlateGoto(c, 0i32, 4i32, sym::menu_x_c__str65);
                                F::Option_PlateGoto(c, 0i32, 2i32, sym::menu_x_c__str66);
                                { let v_ = { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5796); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                                { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(376); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 3i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
                                { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                                { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_x_c__str67, ((((1i32) as i64 as u64)) as u8)) };
                                F::Option_PlateGoto(c, 0i32, 4i32, sym::menu_x_c__str68);
                                { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((1i32) as i64 as u64)])) as i32); };
                            } else {
                                if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) == (13i32)) as i32)) != 0) {
                                    F::Option_PlateGoto(c, 0i32, 2i32, sym::menu_x_c__str69);
                                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 7i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5800); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(272); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5804); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(276); c.m.w32(a_, (v_) as u32); };
                                    F::Option_PlateGoto(c, 0i32, 8i32, sym::menu_x_c__str70);
                                    F::Option_PlateGoto(c, 0i32, 9i32, sym::menu_x_c__str71);
                                    { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                                    { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_x_c__str72, ((((1i32) as i64 as u64)) as u8)) };
                                    { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((1i32) as i64 as u64)])) as i32); };
                                } else {
                                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) == (14i32)) as i32)) != 0) {
                                        F::Option_PlateGoto(c, 0i32, 2i32, sym::menu_x_c__str73);
                                        F::MsgWin_Close(c);
                                        { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                                        { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_x_c__str74, ((((1i32) as i64 as u64)) as u8)) };
                                        F::Option_PlateGoto(c, 0i32, 7i32, sym::menu_x_c__str75);
                                        F::Bgm_Stop(c);
                                        { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((1i32) as i64 as u64)])) as i32); };
                                    } else {
                                        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) == (15i32)) as i32)) != 0) {
                                            F::Option_PlateGoto(c, 0i32, 2i32, sym::menu_x_c__str76);
                                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                                            F::Option_PlateGoto(c, 0i32, 4i32, sym::menu_x_c__str77);
                                            { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                                            F::Option_PlateGoto(c, 0i32, 4i32, sym::menu_x_c__str78);
                                            { let v_ = (((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); (c.m.r32(a_) as i32) }) ^ (1i32))) & (1i32)); let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                                            F::Option_PlateGoto(c, 0i32, 4i32, sym::menu_x_c__str79);
                                            { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = 6i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
                                            { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_x_c__str80, ((((1i32) as i64 as u64)) as u8)) };
                                            { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((1i32) as i64 as u64)])) as i32); };
                                        } else {
                                            if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) == (16i32)) as i32)) != 0) {
                                                F::Option_PlateGoto(c, 0i32, 3i32, sym::menu_x_c__str81);
                                                { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) != (16i32)) as i32)) != 0) {
                                    F::Option_PlateGoto(c, 0i32, 2i32, sym::menu_x_c__str82);
                                } else {
                                    F::Option_PlateGoto(c, 0i32, 3i32, sym::menu_x_c__str83);
                                }
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 3i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); c.m.w32(a_, (v_) as u32); };
                                { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(268); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 2i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(428); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34618i32, a2_) };
                                { let v_ = 3i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                F::Option_PlateGoto(c, 0i32, 0i32, sym::menu_x_c__str84);
                                { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_x_c__str85, ((((1i32) as i64 as u64)) as u8)) };
                                { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((2i32) as i64 as u64)])) as i32); };
                            }
                        }
                    }
                }
                break 's1;
            }
            13i32 => {
                if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((8i32) as u32)))) != 0) && ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0)) as i32)) != 0) {
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                    F::Option_PlateGoto(c, 0i32, 8i32, sym::menu_x_c__str86);
                    F::Option_PlateGoto(c, 0i32, 9i32, sym::menu_x_c__str87);
                    { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                } else {
                    if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((4i32) as u32)))) != 0) && ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); (c.m.r32(a_) as i32) }) != (1i32)) as i32)) != 0)) as i32)) != 0) {
                        { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                        F::Option_PlateGoto(c, 0i32, 8i32, sym::menu_x_c__str88);
                        F::Option_PlateGoto(c, 0i32, 9i32, sym::menu_x_c__str89);
                        { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                            F::Option_PlateGoto(c, 0i32, 8i32, sym::menu_x_c__str90);
                            F::Option_PlateGoto(c, 0i32, 9i32, sym::menu_x_c__str91);
                            F::Option_PlateGoto(c, 0i32, 2i32, sym::menu_x_c__str92);
                            { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(272); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5800); c.m.w32(a_, (v_) as u32); };
                            { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(276); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5804); c.m.w32(a_, (v_) as u32); };
                            F::Bgm_SetVolume(c, 64i32);
                            { let v_ = 3i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                            { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_x_c__str93, ((((1i32) as i64 as u64)) as u8)) };
                            { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((2i32) as i64 as u64)])) as i32); };
                            { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(428); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34618i32, a2_) };
                        } else {
                            if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) {
                                    F::Option_PlateGoto(c, 0i32, 8i32, sym::menu_x_c__str94);
                                    if ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5800); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) < (0i32)) as i32)) != 0) {
                                        { let v_ = 9i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5800); c.m.w32(a_, (v_) as u32); };
                                    }
                                    F::Option_PlateGoto(c, 0i32, 8i32, sym::menu_x_c__str95);
                                    F::Bgm_SetVolume(c, 64i32);
                                    { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                                } else {
                                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) {
                                        F::Option_PlateGoto(c, 0i32, 8i32, sym::menu_x_c__str96);
                                        if ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5800); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) > (9i32)) as i32)) != 0) {
                                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5800); c.m.w32(a_, (v_) as u32); };
                                        }
                                        F::Option_PlateGoto(c, 0i32, 8i32, sym::menu_x_c__str97);
                                        F::Bgm_SetVolume(c, 64i32);
                                        { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                                    } else {
                                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                                            if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(272); (c.m.r32(a_) as i32) }) != ({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5800); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                                { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(408); c.m.w32(a_, (v_) as u32); };
                                            }
                                            { let v_ = { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5800); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(272); c.m.w32(a_, (v_) as u32); };
                                            F::Option_PlateGoto(c, 0i32, 8i32, sym::menu_x_c__str98);
                                            { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((1i32) as i64 as u64)])) as i32); };
                                        }
                                    }
                                }
                            } else {
                                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) {
                                    F::Option_PlateGoto(c, 0i32, 9i32, sym::menu_x_c__str99);
                                    if ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5804); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) < (0i32)) as i32)) != 0) {
                                        { let v_ = 9i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5804); c.m.w32(a_, (v_) as u32); };
                                    }
                                    F::Option_PlateGoto(c, 0i32, 9i32, sym::menu_x_c__str100);
                                    { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                                } else {
                                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) {
                                        F::Option_PlateGoto(c, 0i32, 9i32, sym::menu_x_c__str101);
                                        if ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5804); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) > (9i32)) as i32)) != 0) {
                                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5804); c.m.w32(a_, (v_) as u32); };
                                        }
                                        F::Option_PlateGoto(c, 0i32, 9i32, sym::menu_x_c__str102);
                                        { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                                    } else {
                                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                                            if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(276); (c.m.r32(a_) as i32) }) != ({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5804); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                                { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(408); c.m.w32(a_, (v_) as u32); };
                                            }
                                            { let v_ = { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5804); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(276); c.m.w32(a_, (v_) as u32); };
                                            F::Option_PlateGoto(c, 0i32, 9i32, sym::menu_x_c__str103);
                                            { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((1i32) as i64 as u64)])) as i32); };
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                break 's1;
            }
            14i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(280); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                        F::Option_PlateGoto(c, 0i32, 7i32, sym::menu_x_c__str104);
                        { let a_ = (c.m.r32(sym::gOption)).wrapping_add(280); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(284); (c.m.r32(a_) as i32) }) > ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(280); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                            { let v_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(288); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = (c.m.r32(sym::gOption)).wrapping_add(292); c.m.w32(a_, (v_) as u32); };
                            { let a_ = (c.m.r32(sym::gOption)).wrapping_add(284); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                            { let a_ = (c.m.r32(sym::gOption)).wrapping_add(288); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                            { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_x_c__str105, ((((1i32) as i64 as u64)) as u8)) };
                        }
                        F::Option_PlateGoto(c, 0i32, 7i32, sym::menu_x_c__str106);
                        { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                    }
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(280); (c.m.r32(a_) as i32) }) < (({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(416); (c.m.r32(a_) as i32) }).wrapping_sub(1i32))) as i32)) != 0) {
                            F::Option_PlateGoto(c, 0i32, 7i32, sym::menu_x_c__str107);
                            { let a_ = (c.m.r32(sym::gOption)).wrapping_add(280); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(288); (c.m.r32(a_) as i32) }) <= ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(280); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(284); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(292); c.m.w32(a_, (v_) as u32); };
                                { let a_ = (c.m.r32(sym::gOption)).wrapping_add(284); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                { let a_ = (c.m.r32(sym::gOption)).wrapping_add(288); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_x_c__str108, ((((1i32) as i64 as u64)) as u8)) };
                            }
                            F::Option_PlateGoto(c, 0i32, 7i32, sym::menu_x_c__str109);
                            { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                        }
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                            F::Option_PlateGoto(c, 0i32, 7i32, sym::menu_x_c__str110);
                            if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(412); c.m.r32(a_) }).wrapping_add(((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(280); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != (25i32)) as i32)) != 0) {
                                { let a0_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(412); c.m.r32(a_) }).wrapping_add(((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(280); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(68374i32); F::Bgm_Play(c, a0_) };
                                { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((1i32) as i64 as u64)])) as i32); };
                            } else {
                                { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((7i32) as i64 as u64)])) as i32); };
                            }
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                F::MsgWin_Open(c);
                                { let v_ = 3i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 14i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); c.m.w32(a_, (v_) as u32); };
                                { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_x_c__str111, ((((1i32) as i64 as u64)) as u8)) };
                                F::Option_PlateGoto(c, 0i32, 2i32, sym::menu_x_c__str112);
                                F::Bgm_Play(c, 68376i32);
                                { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((2i32) as i64 as u64)])) as i32); };
                                { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(428); c.m.r32(a_) }; F::Voice_PlayWithSubtitle(c, a0_, 34618i32, 17i32) };
                            }
                        }
                    }
                }
                break 's1;
            }
            4i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) == (19i32)) as i32)) != 0) {
                        F::Option_PlateGoto(c, 0i32, 3i32, sym::menu_x_c__str113);
                    } else {
                        F::Option_PlateGoto(c, 0i32, 10i32, sym::menu_x_c__str114);
                    }
                    { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) < (17i32)) as i32)) != 0) {
                        { let v_ = 19i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); c.m.w32(a_, (v_) as u32); };
                    }
                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) == (19i32)) as i32)) != 0) {
                        F::Option_PlateGoto(c, 0i32, 3i32, sym::menu_x_c__str115);
                    } else {
                        F::Option_PlateGoto(c, 0i32, 10i32, sym::menu_x_c__str116);
                    }
                    { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) == (17i32)) as i32)) != 0) {
                        { let v_ = 23i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                    } else {
                        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) == (18i32)) as i32)) != 0) {
                            { let v_ = 24i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                        } else {
                            { let v_ = 10i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                        }
                    }
                    { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(428); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34618i32, a2_) };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) == (19i32)) as i32)) != 0) {
                            F::Option_PlateGoto(c, 0i32, 3i32, sym::menu_x_c__str117);
                        } else {
                            F::Option_PlateGoto(c, 0i32, 10i32, sym::menu_x_c__str118);
                        }
                        { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) > (19i32)) as i32)) != 0) {
                            { let v_ = 17i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); c.m.w32(a_, (v_) as u32); };
                        }
                        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) == (19i32)) as i32)) != 0) {
                            F::Option_PlateGoto(c, 0i32, 3i32, sym::menu_x_c__str119);
                        } else {
                            F::Option_PlateGoto(c, 0i32, 10i32, sym::menu_x_c__str120);
                        }
                        { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) == (17i32)) as i32)) != 0) {
                            { let v_ = 23i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                        } else {
                            if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }) == (18i32)) as i32)) != 0) {
                                { let v_ = 24i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                            } else {
                                { let v_ = 10i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                            }
                        }
                        { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(428); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34618i32, a2_) };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                            { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                            F::Option_DimPickers(c);
                            if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) == (18i32)) as i32)) != 0) {
                                F::Option_PlateGoto(c, 0i32, 10i32, sym::menu_x_c__str121);
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(320); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 2i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
                                { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_x_c__str122, ((((1i32) as i64 as u64)) as u8)) };
                                F::Option_PlateGoto(c, 0i32, 4i32, sym::menu_x_c__str123);
                                F::Option_PlateGoto(c, 0i32, 5i32, sym::menu_x_c__str124);
                                { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((1i32) as i64 as u64)])) as i32); };
                            } else {
                                if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) == (17i32)) as i32)) != 0) {
                                    F::Option_PlateGoto(c, 0i32, 10i32, sym::menu_x_c__str125);
                                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(320); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 2i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
                                    { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_x_c__str126, ((((1i32) as i64 as u64)) as u8)) };
                                    F::Option_PlateGoto(c, 0i32, 4i32, sym::menu_x_c__str127);
                                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                    F::Option_PlateGoto(c, 0i32, 6i32, sym::menu_x_c__str128);
                                    { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((1i32) as i64 as u64)])) as i32); };
                                } else {
                                    F::Option_PlateGoto(c, 0i32, 3i32, sym::menu_x_c__str129);
                                }
                            }
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                { let v_ = 4i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                F::Option_PlateGoto(c, 0i32, 10i32, sym::menu_x_c__str130);
                                F::Option_PlateGoto(c, 0i32, 3i32, sym::menu_x_c__str131);
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 4i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); c.m.w32(a_, (v_) as u32); };
                                { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                                { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(428); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34618i32, a2_) };
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(268); c.m.w32(a_, (v_) as u32); };
                                F::Option_PlateGoto(c, 0i32, 0i32, sym::menu_x_c__str132);
                                { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_x_c__str133, ((((1i32) as i64 as u64)) as u8)) };
                                { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((2i32) as i64 as u64)])) as i32); };
                            }
                        }
                    }
                }
                break 's1;
            }
            17i32 => {
                if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1i32) as u32)))) != 0) || ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
                    F::Option_PlateGoto(c, 0i32, 6i32, sym::menu_x_c__str134);
                    F::Option_PlateGoto(c, 0i32, 4i32, sym::menu_x_c__str135);
                    { let v_ = { let v_ = (({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); (c.m.r32(a_) as i32) }) ^ (1i32)); let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); c.m.w32(a_, (v_) as u32); v_ }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                    F::Option_PlateGoto(c, 0i32, 4i32, sym::menu_x_c__str136);
                    F::Option_PlateGoto(c, 0i32, 6i32, sym::menu_x_c__str137);
                    { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((4i32) as u32)))) != 0) {
                        F::Option_PlateGoto(c, 0i32, 4i32, sym::menu_x_c__str138);
                        F::Option_PlateGoto(c, 0i32, 6i32, sym::menu_x_c__str139);
                        { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                            if (((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); (c.m.r32(a_) as i32) }) & ((8i32).wrapping_shl((0i32) as u32)))) != 0) {
                                { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                            } else {
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                            }
                            { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(376); c.m.w32(a_, (v_) as u32); };
                        } else {
                            if (((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); (c.m.r32(a_) as i32) }) & ((8i32).wrapping_shl((1i32) as u32)))) != 0) {
                                { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                            } else {
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                            }
                            { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(376); c.m.w32(a_, (v_) as u32); };
                        }
                        { let v_ = 21i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                        { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((1i32) as i64 as u64)])) as i32); };
                        {
                            l_o_4 = c.m.r32(sym::gOption);
                            if ((((((c.m.r32((l_o_4).wrapping_add(296)) as i32)) != (0i32)) as i32)) != 0) {
                                { let v_ = 31i32; let a_ = (l_o_4).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                            } else {
                                { let v_ = 30i32; let a_ = (l_o_4).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                            }
                        }
                        { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(428); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34618i32, a2_) };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                            { let v_ = 4i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 17i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 23i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                            { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(428); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34618i32, a2_) };
                            F::Option_PlateGoto(c, 0i32, 10i32, sym::menu_x_c__str140);
                            { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_x_c__str141, ((((1i32) as i64 as u64)) as u8)) };
                            { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((2i32) as i64 as u64)])) as i32); };
                        }
                    }
                }
                break 's1;
            }
            21i32 => {
                if ((((((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1i32) as u32)))) != 0) || ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((2i32) as u32)))) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(384); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                    F::Option_PlateGoto(c, 0i32, 6i32, sym::menu_x_c__str142);
                    { let r_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                        if (((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); (c.m.r32(a_) as i32) }) & ((8i32).wrapping_shl((0i32) as u32)))) != 0) {
                            { let r_ = (!((8i32).wrapping_shl((0i32) as u32))); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                        } else {
                            { let r_ = (8i32).wrapping_shl((0i32) as u32); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                    } else {
                        if (((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); (c.m.r32(a_) as i32) }) & ((8i32).wrapping_shl((1i32) as u32)))) != 0) {
                            { let r_ = (!((8i32).wrapping_shl((1i32) as u32))); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                        } else {
                            { let r_ = (8i32).wrapping_shl((1i32) as u32); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                    }
                    F::Option_PlateGoto(c, 0i32, 6i32, sym::menu_x_c__str143);
                    { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                        { let v_ = 31i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                    } else {
                        { let v_ = 30i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                    }
                    { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(428); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34618i32, a2_) };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((8i32) as u32)))) != 0) {
                        if (((((((c.m.r32(sym::gSaveData)).wrapping_add(5644)).wrapping_add(((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(32))) != (((c.m.r32(sym::gSaveData)).wrapping_add(5708)).wrapping_add(((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(32)))) as i32)) != 0) {
                            { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(408); c.m.w32(a_, (v_) as u32); };
                        }
                        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                            { let v_ = 2i32; let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(5644)).wrapping_add(((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(32))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 1i32; let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(5644)).wrapping_add(((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(32))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 0i32; let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(5644)).wrapping_add(((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(32))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 3i32; let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(5644)).wrapping_add(((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(32))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 4i32; let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(5644)).wrapping_add(((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(32))).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 5i32; let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(5644)).wrapping_add(((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(32))).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 6i32; let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(5644)).wrapping_add(((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(32))).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 7i32; let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(5644)).wrapping_add(((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(32))).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        } else {
                            {
                                l_i_2 = 0i32;
                                'l2: loop {
                                    if !(((((l_i_2) < (8i32)) as i32)) != 0) { break 'l2; }
                                    'c3: {
                                        { let v_ = { let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(5708)).wrapping_add((((0i32) as u32)).wrapping_mul(32))).wrapping_add(((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); (c.m.r32(a_) as i32) }).wrapping_mul(8i32)).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(5644)).wrapping_add((((0i32) as u32)).wrapping_mul(32))).wrapping_add(((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); (c.m.r32(a_) as i32) }).wrapping_mul(8i32)).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                    }
                                    { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                                }
                            }
                        }
                        F::Option_PlateGoto(c, 0i32, 6i32, sym::menu_x_c__str144);
                        { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 17i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                        F::Option_PlateGoto(c, 0i32, 4i32, sym::menu_x_c__str145);
                        { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((2i32) as i64 as u64)])) as i32); };
                    } else {
                        if (((((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); (c.m.r32(a_) as i32) }) & ((8i32).wrapping_shl((0i32) as u32)))) != 0) || (((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); (c.m.r32(a_) as i32) }) & ((8i32).wrapping_shl((1i32) as u32)))) != 0)) as i32)) != 0) {
                            if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) && ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0)) as i32)) != 0) {
                                F::Option_PlateGoto(c, 0i32, 6i32, sym::menu_x_c__str146);
                                { let v_ = 22i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(384); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 3i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(352); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 2i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(380); c.m.w32(a_, (v_) as u32); };
                                { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(352); (c.m.r32(a_) as i32) }; F::Option_SetKeyMark(c, a0_) }; };
                                { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((1i32) as i64 as u64)])) as i32); };
                            }
                        }
                    }
                }
                break 's1;
            }
            22i32 => {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(388); c.m.w32(a_, (v_) as u32); };
                if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1i32) as u32)))) != 0) && ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(372); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0)) as i32)) != 0) {
                    { let r_ = 4i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(380); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(380); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                        { let r_ = 8i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(380); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                    }
                    { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                } else {
                    if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((2i32) as u32)))) != 0) && ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(372); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0)) as i32)) != 0) {
                        { let r_ = 4i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(380); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(380); (c.m.r32(a_) as i32) }) > (7i32)) as i32)) != 0) {
                            { let r_ = 8i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(380); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                        }
                        { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                    } else {
                        if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((8i32) as u32)))) != 0) && ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(372); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0)) as i32)) != 0) {
                            if ((((((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(380); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) || ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(380); (c.m.r32(a_) as i32) }) == (6i32)) as i32)) != 0)) as i32)) != 0) {
                                { let a_ = (c.m.r32(sym::gOption)).wrapping_add(380); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            } else {
                                { let a_ = (c.m.r32(sym::gOption)).wrapping_add(380); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                            }
                            { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                        } else {
                            if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((4i32) as u32)))) != 0) && ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(372); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0)) as i32)) != 0) {
                                if ((((((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(380); (c.m.r32(a_) as i32) }) == (3i32)) as i32)) != 0) || ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(380); (c.m.r32(a_) as i32) }) == (7i32)) as i32)) != 0)) as i32)) != 0) {
                                    { let a_ = (c.m.r32(sym::gOption)).wrapping_add(380); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                                } else {
                                    { let a_ = (c.m.r32(sym::gOption)).wrapping_add(380); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                }
                                { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                            } else {
                                if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) && ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(372); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0)) as i32)) != 0) {
                                    if (((((((((((c.m.r32(sym::gOption)).wrapping_add(492)).wrapping_add((((0i32) as u32)).wrapping_mul(32))) != (((c.m.r32(sym::gSaveData)).wrapping_add(5708)).wrapping_add((((0i32) as u32)).wrapping_mul(32)))) as i32)) != 0) || (((((((c.m.r32(sym::gOption)).wrapping_add(492)).wrapping_add((((1i32) as u32)).wrapping_mul(32))) != (((c.m.r32(sym::gSaveData)).wrapping_add(5708)).wrapping_add((((1i32) as u32)).wrapping_mul(32)))) as i32)) != 0)) as i32)) != 0) {
                                        { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(408); c.m.w32(a_, (v_) as u32); };
                                    }
                                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(384); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 21i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                                    F::Option_PlateGoto(c, 0i32, 6i32, sym::menu_x_c__str147);
                                    { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((2i32) as i64 as u64)])) as i32); };
                                }
                            }
                        }
                    }
                }
                if (((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) != (((0i32) as u32))) as i32)) != 0) {
                    's4: {
                        let sv6: i32 = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(380); (c.m.r32(a_) as i32) };
                        match sv6 {
                            0i32 => {
                                { let v_ = 5i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(352); c.m.w32(a_, (v_) as u32); };
                                break 's4;
                            }
                            1i32 => {
                                { let v_ = 4i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(352); c.m.w32(a_, (v_) as u32); };
                                break 's4;
                            }
                            2i32 => {
                                { let v_ = 3i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(352); c.m.w32(a_, (v_) as u32); };
                                break 's4;
                            }
                            3i32 => {
                                { let v_ = 2i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(352); c.m.w32(a_, (v_) as u32); };
                                break 's4;
                            }
                            4i32 => {
                                { let v_ = 7i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(352); c.m.w32(a_, (v_) as u32); };
                                break 's4;
                            }
                            5i32 => {
                                { let v_ = 6i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(352); c.m.w32(a_, (v_) as u32); };
                                break 's4;
                            }
                            6i32 => {
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(352); c.m.w32(a_, (v_) as u32); };
                                break 's4;
                            }
                            7i32 => {
                                { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(352); c.m.w32(a_, (v_) as u32); };
                                break 's4;
                            }
                            _ => {
                            }
                        }
                    }
                    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(352); (c.m.r32(a_) as i32) }; F::Option_SetKeyMark(c, a0_) }; };
                }
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(392))) & (((512i32) as u32)))) != 0) {
                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(388); c.m.w32(a_, (v_) as u32); };
                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(372); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                        { let v_ = { let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(5708)).wrapping_add(((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(32))).wrapping_add(((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(352); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(372); c.m.w32(a_, (v_) as u32); };
                    }
                }
                if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(392))) & (((512i32) as u32)))) != 0) && ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1i32) as u32)))) != 0)) as i32)) != 0) {
                    if ((((({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(5708)).wrapping_add(((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(32))).wrapping_add(((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(352); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) < (0i32)) as i32)) != 0) {
                        { let v_ = 3i32; let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(5708)).wrapping_add(((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(32))).wrapping_add(((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(352); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                } else {
                    if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(392))) & (((512i32) as u32)))) != 0) && ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
                        if ((((({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(5708)).wrapping_add(((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(32))).wrapping_add(((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(352); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) > (3i32)) as i32)) != 0) {
                            { let v_ = 0i32; let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(5708)).wrapping_add(((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(32))).wrapping_add(((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(352); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                    }
                }
                if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(388); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(372); (c.m.r32(a_) as i32) }) >= (0i32)) as i32)) != 0) {
                        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(372); (c.m.r32(a_) as i32) }) != ({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(5708)).wrapping_add(((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(32))).wrapping_add(((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(352); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                            {
                                l_i_2 = 0i32;
                                'l5: loop {
                                    if !(((((l_i_2) < (8i32)) as i32)) != 0) { break 'l5; }
                                    'c6: {
                                        if (((((l_i_2) != ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(352); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                            if ((((({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(5708)).wrapping_add(((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(32))).wrapping_add(((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(352); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == ({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(5708)).wrapping_add(((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(32))).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                                { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(372); (c.m.r32(a_) as i32) }; let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(5708)).wrapping_add(((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(32))).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                                break 'l5;
                                            }
                                        }
                                    }
                                    { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                                }
                            }
                        }
                        { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gOption)).wrapping_add(372); c.m.w32(a_, (v_) as u32); };
                    }
                }
                break 's1;
            }
            18i32 => {
                if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1i32) as u32)))) != 0) || ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
                    F::Option_PlateGoto(c, 0i32, 5i32, sym::menu_x_c__str148);
                    F::Option_PlateGoto(c, 0i32, 4i32, sym::menu_x_c__str149);
                    { let v_ = { let v_ = (({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); (c.m.r32(a_) as i32) }) ^ (1i32)); let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); c.m.w32(a_, (v_) as u32); v_ }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                    F::Option_PlateGoto(c, 0i32, 4i32, sym::menu_x_c__str150);
                    F::Option_PlateGoto(c, 0i32, 5i32, sym::menu_x_c__str151);
                    { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((4i32) as u32)))) != 0) {
                        F::Option_PlateGoto(c, 0i32, 4i32, sym::menu_x_c__str152);
                        F::Option_PlateGoto(c, 0i32, 5i32, sym::menu_x_c__str153);
                        { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 20i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                            if (((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); (c.m.r32(a_) as i32) }) & ((2i32).wrapping_shl((0i32) as u32)))) != 0) {
                                { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                            } else {
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                            }
                            { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(376); c.m.w32(a_, (v_) as u32); };
                        } else {
                            if (((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); (c.m.r32(a_) as i32) }) & ((2i32).wrapping_shl((1i32) as u32)))) != 0) {
                                { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                            } else {
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                            }
                            { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(376); c.m.w32(a_, (v_) as u32); };
                        }
                        { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((1i32) as i64 as u64)])) as i32); };
                        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                            { let v_ = 39i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                        } else {
                            { let v_ = 40i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                        }
                        { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(428); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34618i32, a2_) };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                            { let v_ = 4i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 18i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); c.m.w32(a_, (v_) as u32); };
                            { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_x_c__str154, ((((1i32) as i64 as u64)) as u8)) };
                            F::Option_PlateGoto(c, 0i32, 10i32, sym::menu_x_c__str155);
                            { let v_ = 24i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                            { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(428); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34618i32, a2_) };
                            { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((2i32) as i64 as u64)])) as i32); };
                        }
                    }
                }
                break 's1;
            }
            20i32 => {
                if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1i32) as u32)))) != 0) || ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
                    F::Option_PlateGoto(c, 0i32, 5i32, sym::menu_x_c__str156);
                    { let r_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                        if (((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); (c.m.r32(a_) as i32) }) & ((2i32).wrapping_shl((0i32) as u32)))) != 0) {
                            { let r_ = (!((2i32).wrapping_shl((0i32) as u32))); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                        } else {
                            { let r_ = (2i32).wrapping_shl((0i32) as u32); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                    } else {
                        if (((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); (c.m.r32(a_) as i32) }) & ((2i32).wrapping_shl((1i32) as u32)))) != 0) {
                            { let r_ = (!((2i32).wrapping_shl((1i32) as u32))); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                        } else {
                            { let r_ = (2i32).wrapping_shl((1i32) as u32); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                    }
                    F::Option_PlateGoto(c, 0i32, 5i32, sym::menu_x_c__str157);
                    { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                        { let v_ = 39i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                    } else {
                        { let v_ = 40i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                    }
                    { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(428); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34618i32, a2_) };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((8i32) as u32)))) != 0) {
                        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(376); (c.m.r32(a_) as i32) }) != ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                            { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(408); c.m.w32(a_, (v_) as u32); };
                        }
                        F::Option_PlateGoto(c, 0i32, 5i32, sym::menu_x_c__str158);
                        { let v_ = 18i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                        { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                        F::Option_PlateGoto(c, 0i32, 4i32, sym::menu_x_c__str159);
                        { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((2i32) as i64 as u64)])) as i32); };
                    }
                }
                break 's1;
            }
            1i32 | 8i32 | 12i32 | 15i32 => {
                if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1i32) as u32)))) != 0) || ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
                    F::Option_PlateGoto(c, 0i32, 4i32, sym::menu_x_c__str160);
                    { let r_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                    F::Option_PlateGoto(c, 0i32, 4i32, sym::menu_x_c__str161);
                    { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        F::Option_PlateGoto(c, 0i32, 4i32, sym::menu_x_c__str162);
                        { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((1i32) as i64 as u64)])) as i32); };
                        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) == (12i32)) as i32)) != 0) {
                            if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(376); (c.m.r32(a_) as i32) }) != ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(408); c.m.w32(a_, (v_) as u32); };
                            }
                            { let v_ = 3i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                            { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5796); c.m.w32(a_, (v_) as u32); };
                            F::SndOpt_Apply(c);
                            { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_x_c__str163, ((((1i32) as i64 as u64)) as u8)) };
                            F::Option_PlateGoto(c, 0i32, 2i32, sym::menu_x_c__str164);
                        } else {
                            if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) == (8i32)) as i32)) != 0) {
                                if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(376); (c.m.r32(a_) as i32) }) != ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(408); c.m.w32(a_, (v_) as u32); };
                                }
                                { let v_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5784); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 2i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 8i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); c.m.w32(a_, (v_) as u32); };
                                { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_x_c__str165, ((((1i32) as i64 as u64)) as u8)) };
                                F::Option_PlateGoto(c, 0i32, 1i32, sym::menu_x_c__str166);
                            } else {
                                if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) {
                                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                                        F::McFlow_Start(c, 0i32);
                                        { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(392); c.m.w32(a_, (v_) as u32); };
                                    } else {
                                        F::McFlow_Start(c, 1i32);
                                        { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(392); c.m.w32(a_, (v_) as u32); };
                                    }
                                } else {
                                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) == (15i32)) as i32)) != 0) {
                                        { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(408); c.m.w32(a_, (v_) as u32); };
                                        { let v_ = 3i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                                        { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_x_c__str167, ((((1i32) as i64 as u64)) as u8)) };
                                        F::Option_PlateGoto(c, 0i32, 2i32, sym::menu_x_c__str168);
                                        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                            { let r_ = (!(1i32)); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                                        } else {
                                            { let r_ = 1i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                            { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((2i32) as i64 as u64)])) as i32); };
                            F::Option_PlateGoto(c, 0i32, 4i32, sym::menu_x_c__str169);
                            if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) == (12i32)) as i32)) != 0) {
                                { let v_ = { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5796); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                                F::Option_PlateGoto(c, 0i32, 4i32, sym::menu_x_c__str170);
                                { let v_ = 3i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                                F::Option_PlateGoto(c, 0i32, 2i32, sym::menu_x_c__str171);
                                { let v_ = 11i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                                { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(428); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34618i32, a2_) };
                            } else {
                                if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) == (8i32)) as i32)) != 0) {
                                    { let v_ = { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5784); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                                    F::Option_PlateGoto(c, 0i32, 4i32, sym::menu_x_c__str172);
                                    { let v_ = 2i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                                    F::Option_PlateGoto(c, 0i32, 1i32, sym::menu_x_c__str173);
                                    { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(428); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34618i32, a2_) };
                                } else {
                                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) {
                                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                                        { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); c.m.w32(a_, (v_) as u32); };
                                        F::Option_PlateGoto(c, 0i32, 0i32, sym::menu_x_c__str174);
                                        { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(428); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34618i32, a2_) };
                                    } else {
                                        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) == (15i32)) as i32)) != 0) {
                                            { let v_ = 3i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = 15i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); c.m.w32(a_, (v_) as u32); };
                                            F::Option_PlateGoto(c, 0i32, 2i32, sym::menu_x_c__str175);
                                            { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(428); c.m.r32(a_) }; F::Voice_PlayWithSubtitle(c, a0_, 34618i32, 18i32) };
                                        }
                                    }
                                }
                            }
                            { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_x_c__str176, ((((1i32) as i64 as u64)) as u8)) };
                        }
                    }
                }
                break 's1;
            }
            10i32 | 11i32 | 16i32 | 19i32 => {
                F::Option_UpdateReset(c);
                break 's1;
            }
            5i32 => {
                if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(408); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                    { let v_ = (2i32).wrapping_neg(); let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                    F::McFlow_Start(c, 4i32);
                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(392); c.m.w32(a_, (v_) as u32); };
                } else {
                    F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                    { let v_ = 6i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            (-2i32) => {
                F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                { let v_ = 6i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            6i32 => {
                if ((F::ColorFade_IsFadingOut(c)) != 0) {
                    F::Voice_FadeOutStep(c, 0i32);
                    F::Bgm_FadeOutStep(c);
                }
                if ((F::ColorFade_IsOutDone(c)) != 0) {
                    l_ret_1 = 0i32;
                }
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f l_ret_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Option_Input", &PROBE_N, &[], true); }
    r_
}

pub fn Option_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Option_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let mut l_flash_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_text_3: i32 = 0;
    let mut l_num_4: i32 = 0;
    let mut l_y_5: i32 = 0;
    let mut l_rows4_6: i32 = 0;
    let mut l_typeY_7: i32 = 0;
    let mut l_prevY_8: i32 = 0;
    let mut l_bgmY_9: i32 = 0;
    let mut l_baseY_10: i32 = 0;
    let mut l_ctrlY_11: i32 = 0;
    let mut l_row_12: i32 = 0;
    let mut l_col_13: i32 = 0;
    let mut l_id_14: i32 = 0;
    let mut l_scale_15: F32 = F32(0);
    let mut l_fy_16: F32 = F32(0);
    let mut l_rows_17: F32 = F32(0);
    let mut l_len_18: F32 = F32(0);
    let r_ = 'f: {
        l_text_3 = 0i32;
        l_num_4 = 0i32;
        l_flash_1 = (c.m.r32(sym::gOption)).wrapping_add(8);
        { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(52); c.m.r32(a_) }; F::Sprite_DrawPicture(c, a0_, 0i32, 0i32, 128i32) };
        { let a0_ = (c.m.r32(sym::gOption)).wrapping_add(8); F::Flash_Advance(c, a0_) };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_x_c__str177, fp.wrapping_add(8));
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_x_c__str178, fp.wrapping_add(16));
        { let a2_ = (c.m.r32(sym::gOption)).wrapping_add(340); F::FlashAnim_Blink(c, l_flash_1, fp.wrapping_add(8), a2_, 0i32) };
        { let a2_ = (c.m.r32(sym::gOption)).wrapping_add(344); F::FlashAnim_Talk(c, l_flash_1, fp.wrapping_add(16), a2_, 0i32) };
        if ((((((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) || ((((((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) >= (7i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) <= (11i32)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
            F::IconWin_SetIcon(c, 1i32);
        } else {
            if ((((((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) == (3i32)) as i32)) != 0) || ((((((((((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) >= (12i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) <= (16i32)) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) != (14i32)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
                F::IconWin_SetIcon(c, 2i32);
            } else {
                if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) == (14i32)) as i32)) != 0) {
                    F::IconWin_SetIcon(c, 3i32);
                } else {
                    if ((((((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) == (4i32)) as i32)) != 0) || ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) >= (17i32)) as i32)) != 0)) as i32)) != 0) {
                        F::IconWin_SetIcon(c, 4i32);
                    } else {
                        F::IconWin_SetIcon(c, 0i32);
                    }
                }
            }
        }
        's1: {
            let sv19: i32 = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(268); (c.m.r32(a_) as i32) };
            match sv19 {
                0i32 => {
                    l_rows4_6 = 4i32;
                    {
                        l_i_2 = 0i32;
                        'l2: loop {
                            if !(((((l_i_2) < (l_rows4_6)) as i32)) != 0) { break 'l2; }
                            'c3: {
                                { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                                { let v_ = (l_i_2).wrapping_mul(32i32); let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 512i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                { let v_ = ((l_i_2).wrapping_mul(32i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str179, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                                F::Option_SetMenuText(c, l_flash_1, fp, fp.wrapping_add(24), fp.wrapping_add(88));
                            }
                            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                        }
                    }
                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(316); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) {
                        {
                            l_i_2 = 0i32;
                            'l4: loop {
                                if !(((((l_i_2) < (3i32)) as i32)) != 0) { break 'l4; }
                                'c5: {
                                    l_baseY_10 = (l_rows4_6).wrapping_shl((5i32) as u32);
                                    { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = ((l_i_2).wrapping_mul(32i32)).wrapping_add(l_baseY_10); let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 512i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = ((l_i_2).wrapping_mul(32i32)).wrapping_add(160i32); let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                    { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str180, &[(((l_i_2).wrapping_add(5i32)) as i64 as u64)]); };
                                    F::Option_SetMenuText(c, l_flash_1, fp, fp.wrapping_add(24), fp.wrapping_add(88));
                                }
                                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                            }
                        }
                        { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str181, &[((8i32) as i64 as u64)]); };
                        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(24), fp);
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    } else {
                        if ((((((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(316); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) || ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(316); (c.m.r32(a_) as i32) }) == (3i32)) as i32)) != 0)) as i32)) != 0) {
                            { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 224i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 256i32; let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 512i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str182, &[((5i32) as i64 as u64)]); };
                            F::Option_SetMenuText(c, l_flash_1, fp, fp.wrapping_add(24), fp.wrapping_add(88));
                            {
                                l_i_2 = 0i32;
                                'l6: loop {
                                    if !(((((l_i_2) < (3i32)) as i32)) != 0) { break 'l6; }
                                    'c7: {
                                        { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                                        { let v_ = (l_i_2).wrapping_mul(32i32); let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                        { let v_ = 512i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                        { let v_ = ((l_i_2).wrapping_mul(32i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                        { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str183, &[(((l_i_2).wrapping_add(6i32)) as i64 as u64)]); };
                                        F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(24), sym::menu_x_c__str184, fp);
                                        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
                                        F::Flash_ClipSetTex(c, l_flash_1, fp, 1i32);
                                        F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(24), sym::menu_x_c__str185, fp);
                                        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
                                        F::Flash_ClipSetTex(c, l_flash_1, fp, 1i32);
                                    }
                                    { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                                }
                            }
                            if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(316); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) {
                                { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str186, &[((8i32) as i64 as u64)]); };
                                F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(24), fp);
                                { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                            }
                        } else {
                            {
                                l_i_2 = 0i32;
                                'l8: loop {
                                    if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l8; }
                                    'c9: {
                                        { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                                        { let v_ = ((l_i_2).wrapping_mul(32i32)).wrapping_add(96i32); let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                        { let v_ = 512i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                        { let v_ = ((l_i_2).wrapping_mul(32i32)).wrapping_add(128i32); let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                        { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str187, &[(((l_i_2).wrapping_add(5i32)) as i64 as u64)]); };
                                        F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(24), sym::menu_x_c__str188, fp);
                                        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
                                        F::Flash_ClipSetTex(c, l_flash_1, fp, 1i32);
                                        F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(24), sym::menu_x_c__str189, fp);
                                        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
                                        F::Flash_ClipSetTex(c, l_flash_1, fp, 1i32);
                                    }
                                    { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                                }
                            }
                            { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str190, &[((7i32) as i64 as u64)]); };
                            F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(24), fp);
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                            { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str191, &[((8i32) as i64 as u64)]); };
                            F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(24), fp);
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                        }
                    }
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 32i32; let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 256i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str192, &[((1i32) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(24), sym::menu_x_c__str193, fp);
                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(24), sym::menu_x_c__str194, fp);
                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
                    { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 32i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 64i32; let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 256i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str195, &[((2i32) as i64 as u64)]); };
                    F::Option_SetBottomText(c, l_flash_1, fp, fp.wrapping_add(24), fp.wrapping_add(88));
                    break 's1;
                }
                1i32 => {
                    {
                        l_i_2 = 0i32;
                        'l10: loop {
                            if !(((((l_i_2) < (3i32)) as i32)) != 0) { break 'l10; }
                            'c11: {
                                { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                                { let v_ = ((l_i_2).wrapping_mul(32i32)).wrapping_add(128i32); let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 512i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                { let v_ = ((l_i_2).wrapping_mul(32i32)).wrapping_add(160i32); let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str196, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                                F::Option_SetMenuText(c, l_flash_1, fp, fp.wrapping_add(24), fp.wrapping_add(88));
                            }
                            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                        }
                    }
                    { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str197, &[((4i32) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(24), fp);
                    { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    {
                        l_i_2 = 0i32;
                        'l12: loop {
                            if !(((((l_i_2) < (4i32)) as i32)) != 0) { break 'l12; }
                            'c13: {
                                { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 512i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                { let v_ = ((l_i_2).wrapping_mul(32i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                { let v_ = (l_i_2).wrapping_mul(32i32); let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str198, &[(((l_i_2).wrapping_add(5i32)) as i64 as u64)]); };
                                F::Option_SetMenuText(c, l_flash_1, fp, fp.wrapping_add(24), fp.wrapping_add(88));
                            }
                            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                        }
                    }
                    F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_x_c__str199, fp);
                    { let a2_ = ({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5780); (c.m.r32(a_) as i32) }).wrapping_mul(2i32); F::Flash_ClipSetTex(c, l_flash_1, fp, a2_) };
                    F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_x_c__str200, fp);
                    F::Flash_ClipSetTex(c, l_flash_1, fp, 0i32);
                    { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 32i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 256i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 64i32; let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str201, &[((1i32) as i64 as u64)]); };
                    F::Option_SetBottomText(c, l_flash_1, fp, fp.wrapping_add(24), fp.wrapping_add(88));
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 256i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 32i32; let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str202, &[((2i32) as i64 as u64)]); };
                    F::Option_SetBottomText(c, l_flash_1, fp, fp.wrapping_add(24), fp.wrapping_add(88));
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 256i32; let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 256i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str203, &[((1i32) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(24), fp);
                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
                    l_typeY_7 = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(332); (c.m.r32(a_) as i32) }).wrapping_mul(40i32);
                    { let v_ = l_typeY_7; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 512i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (l_typeY_7).wrapping_add(40i32); let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str204, &[((1i32) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(24), fp);
                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
                    l_prevY_8 = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(336); (c.m.r32(a_) as i32) }).wrapping_mul(40i32);
                    { let v_ = l_prevY_8; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 512i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (l_prevY_8).wrapping_add(40i32); let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str205, &[((2i32) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(24), fp);
                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 256i32; let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 256i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str206, &[((1i32) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(24), fp);
                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
                    { let a2_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(332); (c.m.r32(a_) as i32) }; F::Flash_ClipSetTex(c, l_flash_1, fp, a2_) };
                    { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str207, &[((2i32) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(24), fp);
                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
                    { let a2_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(336); (c.m.r32(a_) as i32) }; F::Flash_ClipSetTex(c, l_flash_1, fp, a2_) };
                    break 's1;
                }
                2i32 => {
                    { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 224i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 256i32; let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 512i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str208, &[((1i32) as i64 as u64)]); };
                    F::Option_SetMenuText(c, l_flash_1, fp, fp.wrapping_add(24), fp.wrapping_add(88));
                    {
                        l_i_2 = 0i32;
                        'l14: loop {
                            if !(((((l_i_2) < (3i32)) as i32)) != 0) { break 'l14; }
                            'c15: {
                                { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                                { let v_ = (l_i_2).wrapping_mul(32i32); let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 512i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                { let v_ = ((l_i_2).wrapping_mul(32i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str209, &[(((l_i_2).wrapping_add(2i32)) as i64 as u64)]); };
                                F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(24), sym::menu_x_c__str210, fp);
                                F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
                                F::Flash_ClipSetTex(c, l_flash_1, fp, 1i32);
                                F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(24), sym::menu_x_c__str211, fp);
                                F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
                                F::Flash_ClipSetTex(c, l_flash_1, fp, 1i32);
                            }
                            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                        }
                    }
                    {
                        l_i_2 = 0i32;
                        'l16: loop {
                            if !(((((l_i_2) < (4i32)) as i32)) != 0) { break 'l16; }
                            'c17: {
                                { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 512i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                { let v_ = ((l_i_2).wrapping_mul(32i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                { let v_ = (l_i_2).wrapping_mul(32i32); let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str212, &[(((l_i_2).wrapping_add(5i32)) as i64 as u64)]); };
                                F::Option_SetMenuText(c, l_flash_1, fp, fp.wrapping_add(24), fp.wrapping_add(88));
                                F::Flash_ClipSetTex(c, l_flash_1, fp, 0i32);
                            }
                            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                        }
                    }
                    { let v_ = 32i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 256i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 64i32; let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str213, &[((1i32) as i64 as u64)]); };
                    F::Option_SetBottomText(c, l_flash_1, fp, fp.wrapping_add(24), fp.wrapping_add(88));
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 32i32; let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 256i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str214, &[((2i32) as i64 as u64)]); };
                    F::Option_SetBottomText(c, l_flash_1, fp, fp.wrapping_add(24), fp.wrapping_add(88));
                    break 's1;
                }
                3i32 => {
                    {
                        l_i_2 = 0i32;
                        'l18: loop {
                            if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l18; }
                            'c19: {
                                { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                                { let v_ = ((l_i_2).wrapping_mul(32i32)).wrapping_add(96i32); let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 512i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                { let v_ = ((l_i_2).wrapping_mul(32i32)).wrapping_add(128i32); let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str215, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                                F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(24), sym::menu_x_c__str216, fp);
                                F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
                                F::Flash_ClipSetTex(c, l_flash_1, fp, 1i32);
                                F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(24), sym::menu_x_c__str217, fp);
                                F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
                                F::Flash_ClipSetTex(c, l_flash_1, fp, 1i32);
                            }
                            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                        }
                    }
                    {
                        l_i_2 = 0i32;
                        'l20: loop {
                            if !(((((l_i_2) < (4i32)) as i32)) != 0) { break 'l20; }
                            'c21: {
                                { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 512i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                { let v_ = ((l_i_2).wrapping_mul(32i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                { let v_ = (l_i_2).wrapping_mul(32i32); let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str218, &[(((l_i_2).wrapping_add(5i32)) as i64 as u64)]); };
                                F::Option_SetMenuText(c, l_flash_1, fp, fp.wrapping_add(24), fp.wrapping_add(88));
                            }
                            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                        }
                    }
                    { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str219, &[((3i32) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(24), fp);
                    { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str220, &[((4i32) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(24), fp);
                    { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    { let v_ = 32i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 256i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 64i32; let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str221, &[((1i32) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(24), sym::menu_x_c__str222, fp);
                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(24), sym::menu_x_c__str223, fp);
                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 32i32; let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 256i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str224, &[((2i32) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(24), sym::menu_x_c__str225, fp);
                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(24), sym::menu_x_c__str226, fp);
                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 160i32; let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 48i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 96i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_x_c__str227, fp);
                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
                    {
                        l_i_2 = 0i32;
                        'l22: loop {
                            if !(((((l_i_2) < (8i32)) as i32)) != 0) { break 'l22; }
                            'c23: {
                                { let v_ = 0i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                                's24: {
                                    let sv20: i32 = { let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(5708)).wrapping_add(((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(32))).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                                    match sv20 {
                                        2i32 => {
                                            { let v_ = 80i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = 128i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                                            break 's24;
                                        }
                                        1i32 => {
                                            { let v_ = 120i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = 128i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                                            break 's24;
                                        }
                                        0i32 => {
                                            { let v_ = 40i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = 128i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                                            break 's24;
                                        }
                                        3i32 => {
                                            { let v_ = 0i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = 128i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                                            break 's24;
                                        }
                                        4i32 => {
                                            break 's24;
                                        }
                                        5i32 => {
                                            { let v_ = 40i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                            break 's24;
                                        }
                                        6i32 => {
                                            { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = 80i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                            break 's24;
                                        }
                                        7i32 => {
                                            { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = 120i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                            break 's24;
                                        }
                                        _ => {
                                        }
                                    }
                                }
                                's25: {
                                    let sv21: i32 = l_i_2;
                                    match sv21 {
                                        0i32 => {
                                            l_text_3 = 6i32;
                                            break 's25;
                                        }
                                        1i32 => {
                                            l_text_3 = 7i32;
                                            break 's25;
                                        }
                                        2i32 => {
                                            l_text_3 = 5i32;
                                            break 's25;
                                        }
                                        3i32 => {
                                            l_text_3 = 4i32;
                                            break 's25;
                                        }
                                        4i32 => {
                                            l_text_3 = 1i32;
                                            break 's25;
                                        }
                                        5i32 => {
                                            l_text_3 = 0i32;
                                            break 's25;
                                        }
                                        6i32 => {
                                            l_text_3 = 3i32;
                                            break 's25;
                                        }
                                        7i32 => {
                                            l_text_3 = 2i32;
                                            break 's25;
                                        }
                                        _ => {
                                        }
                                    }
                                }
                                { let v_ = ((c.m.r32((fp.wrapping_add(88)).wrapping_add(4)) as i32)).wrapping_add(40i32); let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                { let v_ = ((c.m.r32(fp.wrapping_add(88)) as i32)).wrapping_add(128i32); let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str228, &[((l_text_3) as i64 as u64)]); };
                                F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(24), fp);
                                F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
                            }
                            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                        }
                    }
                    if ((((((((((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) == (17i32)) as i32)) != 0) || ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) == (21i32)) as i32)) != 0)) as i32)) != 0) || ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) == (4i32)) as i32)) != 0)) as i32)) != 0) {
                        'l26: loop {
                            'c27: {
                                if ((((((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) == (17i32)) as i32)) != 0) || ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) == (4i32)) as i32)) != 0)) as i32)) != 0) {
                                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                                        if (((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); (c.m.r32(a_) as i32) }) & (8i32))) != 0) {
                                            break 'l26;
                                        }
                                    } else {
                                        if (((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); (c.m.r32(a_) as i32) }) & (16i32))) != 0) {
                                            break 'l26;
                                        }
                                    }
                                }
                                if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) == (21i32)) as i32)) != 0) {
                                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                        break 'l26;
                                    }
                                }
                                {
                                    l_i_2 = 0i32;
                                    'l28: loop {
                                        if !(((((l_i_2) < (8i32)) as i32)) != 0) { break 'l28; }
                                        'c29: {
                                            { let v_ = 0i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                                            's30: {
                                                let sv22: i32 = { let a_ = (((c.m.r32(sym::gOption)).wrapping_add(492)).wrapping_add(((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(32))).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                                                match sv22 {
                                                    2i32 => {
                                                        { let v_ = 80i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                                        { let v_ = 128i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                                                        break 's30;
                                                    }
                                                    1i32 => {
                                                        { let v_ = 120i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                                        { let v_ = 128i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                                                        break 's30;
                                                    }
                                                    0i32 => {
                                                        { let v_ = 40i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                                        { let v_ = 128i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                                                        break 's30;
                                                    }
                                                    3i32 => {
                                                        { let v_ = 0i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                                        { let v_ = 128i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                                                        break 's30;
                                                    }
                                                    4i32 => {
                                                        break 's30;
                                                    }
                                                    5i32 => {
                                                        { let v_ = 40i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                                        break 's30;
                                                    }
                                                    6i32 => {
                                                        { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                                                        { let v_ = 80i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                                        break 's30;
                                                    }
                                                    7i32 => {
                                                        { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                                                        { let v_ = 120i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                                        break 's30;
                                                    }
                                                    _ => {
                                                    }
                                                }
                                            }
                                            's31: {
                                                let sv23: i32 = l_i_2;
                                                match sv23 {
                                                    5i32 => {
                                                        l_text_3 = 0i32;
                                                        break 's31;
                                                    }
                                                    4i32 => {
                                                        l_text_3 = 1i32;
                                                        break 's31;
                                                    }
                                                    7i32 => {
                                                        l_text_3 = 2i32;
                                                        break 's31;
                                                    }
                                                    6i32 => {
                                                        l_text_3 = 3i32;
                                                        break 's31;
                                                    }
                                                    3i32 => {
                                                        l_text_3 = 4i32;
                                                        break 's31;
                                                    }
                                                    2i32 => {
                                                        l_text_3 = 5i32;
                                                        break 's31;
                                                    }
                                                    0i32 => {
                                                        l_text_3 = 6i32;
                                                        break 's31;
                                                    }
                                                    1i32 => {
                                                        l_text_3 = 7i32;
                                                        break 's31;
                                                    }
                                                    _ => {
                                                    }
                                                }
                                            }
                                            { let v_ = ((c.m.r32((fp.wrapping_add(88)).wrapping_add(4)) as i32)).wrapping_add(40i32); let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = ((c.m.r32(fp.wrapping_add(88)) as i32)).wrapping_add(128i32); let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                            { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str229, &[((l_text_3) as i64 as u64)]); };
                                            F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(24), fp);
                                            F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
                                        }
                                        { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                                    }
                                }
                            }
                            if !((0i32) != 0) { break 'l26; }
                        }
                    }
                    if (({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(384); (c.m.r32(a_) as i32) }) != 0) {
                        's32: {
                            let sv24: i32 = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(352); (c.m.r32(a_) as i32) };
                            match sv24 {
                                0i32 => {
                                    { let v_ = 80i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 48i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                                    break 's32;
                                }
                                1i32 => {
                                    { let v_ = 120i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 48i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                                    break 's32;
                                }
                                2i32 => {
                                    { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 120i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                    break 's32;
                                }
                                3i32 => {
                                    { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 80i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                    break 's32;
                                }
                                4i32 => {
                                    { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 40i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                    break 's32;
                                }
                                5i32 => {
                                    { let v_ = 0i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                                    break 's32;
                                }
                                6i32 => {
                                    { let v_ = 40i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 48i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                                    break 's32;
                                }
                                7i32 => {
                                    { let v_ = 0i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 48i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                                    break 's32;
                                }
                                _ => {
                                }
                            }
                        }
                        { let v_ = ((c.m.r32((fp.wrapping_add(88)).wrapping_add(4)) as i32)).wrapping_add(40i32); let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r32(fp.wrapping_add(88)) as i32)).wrapping_add(48i32); let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_x_c__str230, fp);
                        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
                        { let a2_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(356); (c.m.r32(a_) as i32) }; let a3_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(360); (c.m.r32(a_) as i32) }; F::Flash_ClipSetOffset(c, l_flash_1, fp, a2_, a3_) };
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                    } else {
                        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_x_c__str231, fp);
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
        if ((((((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(304); (c.m.r32(a_) as i32) }) == (4i32)) as i32)) != 0) || ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(304); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0)) as i32)) != 0) {
            l_baseY_10 = 0i32;
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(304); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                l_baseY_10 = 64i32;
            } else {
                if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(304); (c.m.r32(a_) as i32) }) == (3i32)) as i32)) != 0) {
                    l_baseY_10 = 128i32;
                } else {
                    l_baseY_10 = 192i32;
                }
            }
        }
        {
            l_i_2 = 0i32;
            'l33: loop {
                if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l33; }
                'c34: {
                    { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((l_baseY_10).wrapping_add((l_i_2).wrapping_mul(32i32))).wrapping_add(32i32); let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 256i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (l_baseY_10).wrapping_add((l_i_2).wrapping_mul(32i32)); let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str232, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(304); (c.m.r32(a_) as i32) }) < (7i32)) as i32)) != 0) {
                        F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(24), sym::menu_x_c__str233, fp);
                        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
                        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(304); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) {
                            F::Flash_ClipSetTex(c, l_flash_1, fp, 1i32);
                        }
                        F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(24), sym::menu_x_c__str234, fp);
                        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
                        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(304); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) {
                            F::Flash_ClipSetTex(c, l_flash_1, fp, 1i32);
                        }
                    } else {
                        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(24), fp);
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        l_ctrlY_11 = 0i32;
        if (({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(320); (c.m.r32(a_) as i32) }) != 0) {
            l_ctrlY_11 = 64i32;
        }
        {
            l_i_2 = 0i32;
            'l35: loop {
                if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l35; }
                'c36: {
                    { let v_ = (l_ctrlY_11).wrapping_add((l_i_2).wrapping_mul(32i32)); let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((l_ctrlY_11).wrapping_add((l_i_2).wrapping_mul(32i32))).wrapping_add(32i32); let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 256i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str235, &[(((l_i_2).wrapping_add(3i32)) as i64 as u64)]); };
                    if ((((((((((((((((((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(304); (c.m.r32(a_) as i32) }) != (3i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(304); (c.m.r32(a_) as i32) }) != (4i32)) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(304); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(304); (c.m.r32(a_) as i32) }) != (6i32)) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(304); (c.m.r32(a_) as i32) }) < (7i32)) as i32)) != 0)) as i32)) != 0) {
                        F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(24), sym::menu_x_c__str236, fp);
                        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
                        { let a2_ = ((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(320); (c.m.r32(a_) as i32) }) != (0i32)) as i32); F::Flash_ClipSetTex(c, l_flash_1, fp, a2_) };
                        F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(24), sym::menu_x_c__str237, fp);
                        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
                        { let a2_ = ((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(320); (c.m.r32(a_) as i32) }) != (0i32)) as i32); F::Flash_ClipSetTex(c, l_flash_1, fp, a2_) };
                    } else {
                        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(24), fp);
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        {
            l_i_2 = 0i32;
            'l37: loop {
                if !(((((l_i_2) < (3i32)) as i32)) != 0) { break 'l37; }
                'c38: {
                    {
                        l_col_13 = 0i32;
                        'l39: loop {
                            if !(((((l_col_13) < (4i32)) as i32)) != 0) { break 'l39; }
                            'c40: {
                                { let v_ = (l_i_2).wrapping_mul(32i32); let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                { let v_ = (l_col_13).wrapping_mul(32i32); let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                                { let v_ = ((l_col_13).wrapping_mul(32i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                { let v_ = ((l_i_2).wrapping_mul(32i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str238, &[((l_num_4) as i64 as u64)]); };
                                if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(304); (c.m.r32(a_) as i32) }) < (7i32)) as i32)) != 0) {
                                    F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(24), fp);
                                    { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                                } else {
                                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(24), sym::menu_x_c__str239, fp);
                                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
                                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(24), sym::menu_x_c__str240, fp);
                                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
                                }
                                { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str241, &[((l_num_4) as i64 as u64)]); };
                                if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(304); (c.m.r32(a_) as i32) }) < (7i32)) as i32)) != 0) {
                                    F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(24), fp);
                                    { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                                } else {
                                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(24), sym::menu_x_c__str242, fp);
                                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
                                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(24), sym::menu_x_c__str243, fp);
                                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
                                }
                                { let o_ = l_num_4; let n_ = (o_).wrapping_add(1); l_num_4 = n_; };
                                if (((((l_num_4) >= (10i32)) as i32)) != 0) {
                                    break 'l39;
                                }
                            }
                            { let o_ = l_col_13; let n_ = (o_).wrapping_add(1); l_col_13 = n_; };
                        }
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(304); (c.m.r32(a_) as i32) }) < (7i32)) as i32)) != 0) {
            F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_x_c__str244, fp);
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
            F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_x_c__str245, fp);
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
            F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_x_c__str246, fp);
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
        }
        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) == (14i32)) as i32)) != 0) {
            {
                l_i_2 = 0i32;
                'l41: loop {
                    if !(((((l_i_2) < (6i32)) as i32)) != 0) { break 'l41; }
                    'c42: {
                        { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str247, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                        if (((((((((l_i_2) == (0i32)) as i32)) != 0) || (((((l_i_2) == (5i32)) as i32)) != 0)) as i32)) != 0) {
                            F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(24), fp);
                            F::Flash_ClipSetCallbackA(c, l_flash_1, fp, fid::Option_SetBgmScissor, ((0i32) as u32));
                            F::Flash_ClipSetCallbackB(c, l_flash_1, fp, fid::Option_ResetScissor, ((0i32) as u32));
                        }
                        l_bgmY_9 = (rt::rem_i32({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(412); c.m.r32(a_) }).wrapping_add((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(284); (c.m.r32(a_) as i32) }).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }, 8i32)).wrapping_mul(32i32);
                        { let v_ = l_bgmY_9; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 512i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = (l_bgmY_9).wrapping_add(32i32); let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        { let v_ = rt::div_i32({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(412); c.m.r32(a_) }).wrapping_add((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(284); (c.m.r32(a_) as i32) }).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }, 8i32); let a_ = (fp.wrapping_add(88)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                        { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_x_c__str248, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                        F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(24), sym::menu_x_c__str249, fp);
                        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
                        { let a2_ = (c.m.r32((fp.wrapping_add(88)).wrapping_add(16)) as i32); F::Flash_ClipSetTex(c, l_flash_1, fp, a2_) };
                        F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(24), sym::menu_x_c__str250, fp);
                        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
                        { let a2_ = (c.m.r32((fp.wrapping_add(88)).wrapping_add(16)) as i32); F::Flash_ClipSetTex(c, l_flash_1, fp, a2_) };
                    }
                    { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                }
            }
            F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_x_c__str251, fp);
            F::Flash_ClipSetCallbackA(c, l_flash_1, fp, fid::Option_SetBgmScissor, ((0i32) as u32));
            F::Flash_ClipSetCallbackB(c, l_flash_1, fp, fid::Option_ResetScissor, ((0i32) as u32));
            l_rows_17 = F32(0x40c00000);
            l_bgmY_9 = (rt::rem_i32({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(412); c.m.r32(a_) }).wrapping_add(((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(292); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }, 8i32)).wrapping_mul(32i32);
            { let v_ = (l_bgmY_9).wrapping_add(32i32); let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
            { let v_ = 512i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = l_bgmY_9; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
            { let v_ = rt::div_i32({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(412); c.m.r32(a_) }).wrapping_add(((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(292); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }, 8i32); let a_ = (fp.wrapping_add(88)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
            F::Flash_FindLabel(c, l_flash_1, sym::menu_x_c__str252, sym::menu_x_c__str253, fp);
            F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
            { let a2_ = (c.m.r32((fp.wrapping_add(88)).wrapping_add(16)) as i32); F::Flash_ClipSetTex(c, l_flash_1, fp, a2_) };
            F::Flash_FindLabel(c, l_flash_1, sym::menu_x_c__str254, sym::menu_x_c__str255, fp);
            F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
            { let a2_ = (c.m.r32((fp.wrapping_add(88)).wrapping_add(16)) as i32); F::Flash_ClipSetTex(c, l_flash_1, fp, a2_) };
            l_len_18 = F32(0x40e99999);
            l_y_5 = rt::f2i(rt::fmul(rt::fdiv(F32(0x437f7fff), rt::i2f({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(416); (c.m.r32(a_) as i32) })), rt::i2f({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(284); (c.m.r32(a_) as i32) })));
            l_scale_15 = rt::fmul(rt::fdiv(l_rows_17, rt::i2f({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(416); (c.m.r32(a_) as i32) })), l_len_18);
            F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_x_c__str256, fp);
            F::Flash_ClipSetScale(c, l_flash_1, fp, F32(0x3f800000), l_scale_15);
            F::Flash_ClipSetOffset(c, l_flash_1, fp, 0i32, l_y_5);
        }
        { let v_ = 0i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_x_c__str257, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
        { let a2_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(272); (c.m.r32(a_) as i32) }).wrapping_mul(35i32); F::Flash_ClipSetOffset(c, l_flash_1, fp, a2_, 0i32) };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_x_c__str258, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
        { let a2_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(276); (c.m.r32(a_) as i32) }).wrapping_mul(35i32); F::Flash_ClipSetOffset(c, l_flash_1, fp, a2_, 0i32) };
        F::Flash_FindLabel(c, l_flash_1, sym::menu_x_c__str259, sym::menu_x_c__str260, fp);
        { let a3_ = ((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(284); (c.m.r32(a_) as i32) }) != (0i32)) as i32); { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((a3_) as i64 as u64)) as u8)); } };
        F::Flash_FindLabel(c, l_flash_1, sym::menu_x_c__str261, sym::menu_x_c__str262, fp);
        { let a3_ = ((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(288); (c.m.r32(a_) as i32) }) < ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(416); (c.m.r32(a_) as i32) })) as i32); { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((a3_) as i64 as u64)) as u8)); } };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, sym::menu_x_c__str263, sym::menu_x_c__str264, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
        { let v_ = 32i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, sym::menu_x_c__str265, sym::menu_x_c__str266, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
        { let v_ = 0i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, sym::menu_x_c__str267, sym::menu_x_c__str268, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
        if ((((((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) == (13i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0)) as i32)) != 0) {
            F::Flash_ClipSetOffset(c, l_flash_1, fp, 0i32, 112i32);
            F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_x_c__str269, fp);
            F::Flash_ClipSetOffset(c, l_flash_1, fp, 0i32, 112i32);
        } else {
            if ((((((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) == (22i32)) as i32)) != 0) && (({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(388); (c.m.r32(a_) as i32) }) != 0)) as i32)) != 0) {
                F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_x_c__str270, fp);
                { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                { let a2_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(356); (c.m.r32(a_) as i32) }; let a3_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(360); (c.m.r32(a_) as i32) }; F::Flash_ClipSetOffset(c, l_flash_1, fp, a2_, a3_) };
                F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_x_c__str271, fp);
                { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                { let a2_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(356); (c.m.r32(a_) as i32) }; let a3_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(360); (c.m.r32(a_) as i32) }; F::Flash_ClipSetOffset(c, l_flash_1, fp, a2_, a3_) };
            } else {
                F::Flash_ClipSetOffset(c, l_flash_1, fp, 0i32, 0i32);
                F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_x_c__str272, fp);
                F::Flash_ClipSetOffset(c, l_flash_1, fp, 0i32, 0i32);
            }
        }
        if ((((((((((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) != (13i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) != (7i32)) as i32)) != 0)) as i32)) != 0) && ((((!(({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(388); (c.m.r32(a_) as i32) }) != 0)) as i32)) != 0)) as i32)) != 0) {
            F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_x_c__str273, fp);
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
            F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_x_c__str274, fp);
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
        }
        {
            l_i_2 = 0i32;
            'l43: loop {
                if !(((((l_i_2) < (1i32)) as i32)) != 0) { break 'l43; }
                'c44: {
                    { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((l_i_2) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        F::IconWin_Draw(c);
        { let a2_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(300); (c.m.r32(a_) as i32) }; F::MsgWin_Draw(c, 0i32, 0i32, a2_) };
        F::ColorFade_Draw(c);
        if (({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(392); (c.m.r32(a_) as i32) }) != 0) {
            { let v_ = F::McFlow_Update(c); let a_ = (c.m.r32(sym::gOption)).wrapping_add(392); c.m.w32(a_, (v_) as u32); };
            if ((((!(({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(392); (c.m.r32(a_) as i32) }) != 0)) as i32)) != 0) {
                F::Option_PlateGoto(c, 0i32, 4i32, sym::menu_x_c__str275);
            }
        } else {
            F::Dialog_Draw(c, 1i32);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Option_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn Option_PlateGoto(c: &mut Ctx, mut p_unused_1: i32, mut p_clip_2: i32, mut p_label_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Option_PlateGoto", &PROBE_N, &[(p_unused_1) as u32, (p_clip_2) as u32, (p_label_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_flash_4: u32 = 0;
    let r_ = 'f: {
        l_flash_4 = (c.m.r32(sym::gOption)).wrapping_add(8);
        's1: {
            let sv5: i32 = p_clip_2;
            match sv5 {
                0i32 => {
                    { let _ = { let a2_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }; F::sprintf(c, fp.wrapping_add(8), sym::menu_x_c__str276, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                1i32 => {
                    { let _ = { let a2_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }).wrapping_sub(6i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_x_c__str277, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                2i32 => {
                    { let _ = { let a2_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }).wrapping_sub(11i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_x_c__str278, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                3i32 => {
                    { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_x_c__str279, &[((1i32) as i64 as u64)]); };
                    break 's1;
                }
                4i32 => {
                    { let _ = { let a2_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(296); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_x_c__str280, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                5i32 => {
                    { let _ = { let a2_ = (if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) { (if (((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); (c.m.r32(a_) as i32) }) & (2i32))) != 0) { 3i32 } else { 4i32 }) } else { (if (((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); (c.m.r32(a_) as i32) }) & (4i32))) != 0) { 3i32 } else { 4i32 }) }); F::sprintf(c, fp.wrapping_add(8), sym::menu_x_c__str281, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                6i32 => {
                    { let _ = { let a2_ = (if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(312); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) { (if (((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); (c.m.r32(a_) as i32) }) & (8i32))) != 0) { 4i32 } else { 3i32 }) } else { (if (((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); (c.m.r32(a_) as i32) }) & (16i32))) != 0) { 4i32 } else { 3i32 }) }); F::sprintf(c, fp.wrapping_add(8), sym::menu_x_c__str282, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                7i32 => {
                    { let _ = { let a2_ = (({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(280); (c.m.r32(a_) as i32) }).wrapping_sub({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(284); (c.m.r32(a_) as i32) })).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_x_c__str283, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                8i32 => {
                    { let _ = { let a2_ = { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5800); (c.m.r32(a_) as i32) }; F::sprintf(c, fp.wrapping_add(8), sym::menu_x_c__str284, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                9i32 => {
                    { let _ = { let a2_ = { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5804); (c.m.r32(a_) as i32) }; F::sprintf(c, fp.wrapping_add(8), sym::menu_x_c__str285, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                10i32 => {
                    { let _ = { let a2_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(260); (c.m.r32(a_) as i32) }).wrapping_sub(16i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_x_c__str286, &[((a2_) as i64 as u64)]) }; };
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
    if rt::probing() { crate::host::harness::probe(c, "Option_PlateGoto", &PROBE_N, &[], true); }
    r_
}

pub fn Option_SetKeyMark(c: &mut Ctx, mut p_row_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Option_SetKeyMark", &PROBE_N, &[(p_row_1) as u32], false); }
    let mut l_ret_2: i32 = 0;
    let r_ = 'f: {
    l_ret_2 = 0i32;
    's1: {
        let sv3: i32 = p_row_1;
        match sv3 {
            0i32 => {
                l_ret_2 = 0i32;
                { let v_ = 180i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(356); c.m.w32(a_, (v_) as u32); };
                { let v_ = 80i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(360); c.m.w32(a_, (v_) as u32); };
                { let v_ = 180i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(364); c.m.w32(a_, (v_) as u32); };
                { let v_ = 80i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(368); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            1i32 => {
                l_ret_2 = 1i32;
                { let v_ = 180i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(356); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(360); c.m.w32(a_, (v_) as u32); };
                { let v_ = 180i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(364); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(368); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            2i32 => {
                l_ret_2 = 2i32;
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(356); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(360); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(364); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(368); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            3i32 => {
                l_ret_2 = 3i32;
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(356); c.m.w32(a_, (v_) as u32); };
                { let v_ = 80i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(360); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(364); c.m.w32(a_, (v_) as u32); };
                { let v_ = 80i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(368); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            4i32 => {
                l_ret_2 = 4i32;
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(356); c.m.w32(a_, (v_) as u32); };
                { let v_ = 40i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(360); c.m.w32(a_, (v_) as u32); };
                { let v_ = (180i32).wrapping_neg(); let a_ = (c.m.r32(sym::gOption)).wrapping_add(364); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(368); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            5i32 => {
                l_ret_2 = 5i32;
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(356); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(360); c.m.w32(a_, (v_) as u32); };
                { let v_ = (180i32).wrapping_neg(); let a_ = (c.m.r32(sym::gOption)).wrapping_add(364); c.m.w32(a_, (v_) as u32); };
                { let v_ = (40i32).wrapping_neg(); let a_ = (c.m.r32(sym::gOption)).wrapping_add(368); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            6i32 => {
                l_ret_2 = 6i32;
                { let v_ = 180i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(356); c.m.w32(a_, (v_) as u32); };
                { let v_ = 40i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(360); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(364); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(368); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            7i32 => {
                l_ret_2 = 7i32;
                { let v_ = 180i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(356); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(360); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(364); c.m.w32(a_, (v_) as u32); };
                { let v_ = (40i32).wrapping_neg(); let a_ = (c.m.r32(sym::gOption)).wrapping_add(368); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f l_ret_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Option_SetKeyMark", &PROBE_N, &[], true); }
    r_
}

pub fn Option_UpdateReset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Option_UpdateReset", &PROBE_N, &[], false); }
    let mut l_answer_1: i32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    's1: {
        let sv3: i32 = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(348); (c.m.r32(a_) as i32) };
        match sv3 {
            0i32 => {
                { let a0_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((35i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::Dialog_Init(c, a0_, ((0i32) as u32), 0i32) };
                F::Dialog_SetCursor(c, 1i32);
                { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(424); c.m.r32(a_) }; F::Dialog_SetMsgTable(c, a0_) };
                F::Dialog_SetMsg(c, 0i32);
                F::Dialog_SetChoices(c, 1i32);
                F::Dialog_Start(c, 0i32);
                { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(348); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            1i32 => {
                l_answer_1 = F::Dialog_Input(c, 1i32);
                if (((((l_answer_1) < (0i32)) as i32)) != 0) {
                    { let v_ = 3i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(348); c.m.w32(a_, (v_) as u32); };
                } else {
                    if (((((l_answer_1) > (0i32)) as i32)) != 0) {
                        { let v_ = 2i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(348); c.m.w32(a_, (v_) as u32); };
                    }
                }
                break 's1;
            }
            2i32 => {
                if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) == (10i32)) as i32)) != 0) {
                    if ((((((((((((((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5788); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) || ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5792); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0)) as i32)) != 0) || ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5780); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0)) as i32)) != 0) || ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5784); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0)) as i32)) != 0) {
                        { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(408); c.m.w32(a_, (v_) as u32); };
                    }
                    { let v_ = { let v_ = 0i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5792); c.m.w32(a_, (v_) as u32); v_ }; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5788); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5780); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5784); c.m.w32(a_, (v_) as u32); };
                } else {
                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) == (11i32)) as i32)) != 0) {
                        if ((((((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5788); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) || ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5792); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0)) as i32)) != 0) {
                            { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(408); c.m.w32(a_, (v_) as u32); };
                        }
                        { let v_ = { let v_ = 0i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5792); c.m.w32(a_, (v_) as u32); v_ }; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5788); c.m.w32(a_, (v_) as u32); };
                    } else {
                        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) == (16i32)) as i32)) != 0) {
                            if ((((((((((((((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5796); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) || ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5800); (c.m.r32(a_) as i32) }) != (9i32)) as i32)) != 0)) as i32)) != 0) || ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5804); (c.m.r32(a_) as i32) }) != (9i32)) as i32)) != 0)) as i32)) != 0) || ((((!(((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); (c.m.r32(a_) as i32) }) & (1i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
                                { let v_ = 1i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(408); c.m.w32(a_, (v_) as u32); };
                            }
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5796); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 9i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5800); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 9i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5804); c.m.w32(a_, (v_) as u32); };
                            { let r_ = 1i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            F::SndOpt_Apply(c);
                        } else {
                            if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) == (19i32)) as i32)) != 0) {
                                if (((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); (c.m.r32(a_) as i32) }) & (8i32))) != 0) {
                                    { let r_ = (!(8i32)); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                                }
                                if (((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); (c.m.r32(a_) as i32) }) & (16i32))) != 0) {
                                    { let r_ = (!(16i32)); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                                }
                                if ((((!(((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); (c.m.r32(a_) as i32) }) & (2i32))) != 0)) as i32)) != 0) {
                                    { let r_ = 2i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                }
                                if ((((!(((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); (c.m.r32(a_) as i32) }) & (4i32))) != 0)) as i32)) != 0) {
                                    { let r_ = 4i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                }
                                {
                                    l_i_2 = 0i32;
                                    'l2: loop {
                                        if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l2; }
                                        'c3: {
                                            { let v_ = 2i32; let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(5708)).wrapping_add((((l_i_2) as u32)).wrapping_mul(32))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = 1i32; let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(5708)).wrapping_add((((l_i_2) as u32)).wrapping_mul(32))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = 0i32; let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(5708)).wrapping_add((((l_i_2) as u32)).wrapping_mul(32))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = 3i32; let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(5708)).wrapping_add((((l_i_2) as u32)).wrapping_mul(32))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = 4i32; let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(5708)).wrapping_add((((l_i_2) as u32)).wrapping_mul(32))).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = 5i32; let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(5708)).wrapping_add((((l_i_2) as u32)).wrapping_mul(32))).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = 6i32; let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(5708)).wrapping_add((((l_i_2) as u32)).wrapping_mul(32))).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = 7i32; let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(5708)).wrapping_add((((l_i_2) as u32)).wrapping_mul(32))).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                        }
                                        { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                                    }
                                }
                            }
                        }
                    }
                }
                { let v_ = 3i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(348); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            3i32 => {
                F::Dialog_SetChoices(c, 0i32);
                F::Dialog_Start(c, 1i32);
                { let v_ = 4i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(348); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            4i32 => {
                if ((F::Dialog_IsClosed(c)) != 0) {
                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) == (10i32)) as i32)) != 0) {
                        { let v_ = 2i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                    } else {
                        if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) == (11i32)) as i32)) != 0) {
                            { let v_ = 9i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                        } else {
                            if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) == (16i32)) as i32)) != 0) {
                                { let v_ = 3i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                            } else {
                                { let v_ = 4i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                            }
                        }
                    }
                    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(264); (c.m.r32(a_) as i32) }) != (9i32)) as i32)) != 0) {
                        F::Option_PlateGoto(c, 0i32, 3i32, sym::menu_x_c__str287);
                    }
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(348); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Option_UpdateReset", &PROBE_N, &[], true); }
    r_
}

pub fn Option_SetBottomText(c: &mut Ctx, mut p_flash_1: u32, mut p_ref_2: u32, mut p_name_3: u32, p_uv_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Option_SetBottomText", &PROBE_N, &[(p_flash_1) as u32, (p_ref_2) as u32, (p_name_3) as u32, (p_uv_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    rt::memcpy(c, fp, p_uv_4, 20);
    let r_ = 'f: {
        F::Flash_FindLabel(c, p_flash_1, p_name_3, sym::menu_x_c__str288, p_ref_2);
        F::Flash_ClipSetUv(c, p_flash_1, p_ref_2, fp);
        F::Flash_ClipSetTex(c, p_flash_1, p_ref_2, 0i32);
        F::Flash_FindLabel(c, p_flash_1, p_name_3, sym::menu_x_c__str289, p_ref_2);
        F::Flash_ClipSetUv(c, p_flash_1, p_ref_2, fp);
        F::Flash_ClipSetTex(c, p_flash_1, p_ref_2, 0i32);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Option_SetBottomText", &PROBE_N, &[], true); }
    r_
}

pub fn Option_SetMenuText(c: &mut Ctx, mut p_flash_1: u32, mut p_ref_2: u32, mut p_name_3: u32, p_uv_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Option_SetMenuText", &PROBE_N, &[(p_flash_1) as u32, (p_ref_2) as u32, (p_name_3) as u32, (p_uv_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    rt::memcpy(c, fp, p_uv_4, 20);
    let r_ = 'f: {
        F::Flash_FindLabel(c, p_flash_1, p_name_3, sym::menu_x_c__str290, p_ref_2);
        F::Flash_ClipSetUv(c, p_flash_1, p_ref_2, fp);
        F::Flash_ClipSetTex(c, p_flash_1, p_ref_2, 0i32);
        F::Flash_FindLabel(c, p_flash_1, p_name_3, sym::menu_x_c__str291, p_ref_2);
        F::Flash_ClipSetUv(c, p_flash_1, p_ref_2, fp);
        F::Flash_ClipSetTex(c, p_flash_1, p_ref_2, 0i32);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Option_SetMenuText", &PROBE_N, &[], true); }
    r_
}

pub fn Option_DimPickers(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Option_DimPickers", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_flash_1: u32 = 0;
    let r_ = 'f: {
        l_flash_1 = (c.m.r32(sym::gOption)).wrapping_add(8);
        { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_x_c__str292, &[((1i32) as i64 as u64)]); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, l_flash_1, fp, sym::menu_x_c__str293);
        { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_x_c__str294, &[((2i32) as i64 as u64)]); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, l_flash_1, fp, sym::menu_x_c__str295);
        { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_x_c__str296, &[((3i32) as i64 as u64)]); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, l_flash_1, fp, sym::menu_x_c__str297);
        { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_x_c__str298, &[((4i32) as i64 as u64)]); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, l_flash_1, fp, sym::menu_x_c__str299);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Option_DimPickers", &PROBE_N, &[], true); }
    r_
}

pub fn Option_SetBgmScissor(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Option_SetBgmScissor", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Sprite_SetScissor(c, 0i32, 512i32, 69i32, 372i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Option_SetBgmScissor", &PROBE_N, &[], true); }
    r_
}

pub fn Option_ResetScissor(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Option_ResetScissor", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Sprite_SetScissor(c, 0i32, 511i32, 0i32, 447i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Option_ResetScissor", &PROBE_N, &[], true); }
    r_
}

pub fn Option_OnSaved(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Option_OnSaved", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(404); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gOption)).wrapping_add(408); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Option_OnSaved", &PROBE_N, &[], true); }
    r_
}

pub fn Option_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Option_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    F::Dialog_Term(c);
    F::MsgWin_Term(c);
    F::IconWin_Term(c);
    F::McFlow_Term(c);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gOption)).wrapping_add(8)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Destroy(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gOption)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gOption)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gOption); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gOption; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Option_Term", &PROBE_N, &[], true); }
    r_
}
