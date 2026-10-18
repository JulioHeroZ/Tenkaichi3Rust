#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn CharRef_Init(c: &mut Ctx, mut p_section_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CharRef_Init", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_table_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = F::Heap_Alloc(c, 5688i32, ((32i32) as u32), 0i32, 2i32); let a_ = sym::gCharRef; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gCharRef); F::memset(c, a0_, 0i32, ((5688i32) as u32)) }; };
        { let v_ = (c.m.r32(sym::gMenuArc9)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc9)).wrapping_add((((p_section_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = c.m.r32(sym::gCharRef); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gCharRef); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = c.m.r32(fp); let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(156); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((39i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((32i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((42i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((43i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((33i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((8i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((34i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((35i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((36i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((30i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((40i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((37i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((38i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((41i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        l_table_2 = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
        { let v_ = l_table_2; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(3624); c.m.w32(a_, v_); };
        l_table_2 = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
        { let v_ = l_table_2; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(3628); c.m.w32(a_, v_); };
        l_table_2 = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
        { let v_ = l_table_2; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(3632); c.m.w32(a_, v_); };
        l_table_2 = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
        { let v_ = 161i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5604); c.m.w32(a_, (v_) as u32); };
        { let v_ = l_table_2; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(3620); c.m.w32(a_, v_); };
        { let v_ = F::Heap_Alloc(c, 92160i32, ((64i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = F::Heap_Alloc(c, 133120i32, ((32i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(8); c.m.w32(a_, v_); };
        { let _ = { let a1_ = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(4); c.m.r32(a_) }; F::File_LoadSync(c, 761i32, a1_, 92160i32) }; };
        { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(4); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(8); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, a1_, ((0i32) as u32)) }; };
        { let v_ = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(8); c.m.r32(a_) }; let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < (5i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(40)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(76)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(24)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(24)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(24)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gCharRef)).wrapping_add(3636)).wrapping_add((((0i32) as u32)).wrapping_mul(140)); let a1_ = { let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, ((2i32) as u32)) };
        { let a0_ = ((c.m.r32(sym::gCharRef)).wrapping_add(3636)).wrapping_add((((1i32) as u32)).wrapping_mul(140)); let a1_ = { let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(24)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, ((4i32) as u32)) };
        { let a0_ = ((c.m.r32(sym::gCharRef)).wrapping_add(3636)).wrapping_add((((2i32) as u32)).wrapping_mul(140)); let a1_ = { let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(24)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, ((2i32) as u32)) };
        { let a0_ = ((c.m.r32(sym::gCharRef)).wrapping_add(3636)).wrapping_add((((3i32) as u32)).wrapping_mul(140)); let a1_ = { let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(24)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, ((0i32) as u32)) };
        { let a0_ = ((c.m.r32(sym::gCharRef)).wrapping_add(3636)).wrapping_add((((3i32) as u32)).wrapping_mul(140)); F::TextBox_SetRect(c, a0_, 0i32, 512i32, 296i32, 408i32) };
        {
            l_i_3 = 0i32;
            'l3: loop {
                if !(((((l_i_3) < (5i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let a0_ = ((c.m.r32(sym::gCharRef)).wrapping_add(3636)).wrapping_add(((((4i32).wrapping_add(l_i_3)) as u32)).wrapping_mul(140)); let a1_ = { let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(40)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, ((2i32) as u32)) };
                    { let a0_ = ((c.m.r32(sym::gCharRef)).wrapping_add(3636)).wrapping_add(((((4i32).wrapping_add(l_i_3)) as u32)).wrapping_mul(140)); F::TextBox_SetRect(c, a0_, 0i32, 512i32, 73i32, 309i32) };
                    { let a0_ = ((c.m.r32(sym::gCharRef)).wrapping_add(3636)).wrapping_add(((((9i32).wrapping_add(l_i_3)) as u32)).wrapping_mul(140)); let a1_ = { let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(76)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, ((4i32) as u32)) };
                    { let a0_ = ((c.m.r32(sym::gCharRef)).wrapping_add(3636)).wrapping_add(((((9i32).wrapping_add(l_i_3)) as u32)).wrapping_mul(140)); F::TextBox_SetRect(c, a0_, 0i32, 512i32, 73i32, 309i32) };
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        {
            l_i_3 = 0i32;
            'l5: loop {
                if !(((((l_i_3) < (14i32)) as i32)) != 0) { break 'l5; }
                'c6: {
                    { let a0_ = ((c.m.r32(sym::gCharRef)).wrapping_add(3636)).wrapping_add((((l_i_3) as u32)).wrapping_mul(140)); F::TextBox_SetUnk80(c, a0_, 1i32) };
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(20); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gCharRef)).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gCharRef)).wrapping_add(160); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gCharRef)).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let a0_ = ((c.m.r32(sym::gCharRef)).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_h_b__str0, ((((1i32) as i64 as u64)) as u8)) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = c.m.r32(fp); F::IconWin_Init(c, a0_, a1_) };
        F::IconWin_Open(c);
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(16); c.m.w32(a_, v_); };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(16); c.m.r32(a_) }; let a3_ = (((c.m.r32(sym::gCharRef)).wrapping_add(336)) as i32); let _ = a3_; F::MsgWin_Init(c, a0_, a1_, 0i32) };
        F::MsgWin_Open(c);
        F::ChrView_Init(c);
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5672); c.m.w32(a_, (v_) as u32); };
        { let v_ = 4i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5612); c.m.w32(a_, (v_) as u32); };
        { let v_ = F32(0x3f800000); let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5684); c.m.w32(a_, (v_).0); };
        { let v_ = 9i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5656); c.m.w32(a_, (v_) as u32); };
        { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5624); c.m.w32(a_, (v_) as u32); };
        { let v_ = fid::CharRef_StateIntro; let a_ = (sym::gCharRefState).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = fid::CharRef_StateList; let a_ = (sym::gCharRefState).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = fid::CharRef_StateDetails; let a_ = (sym::gCharRefState).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = fid::CharRef_StateProfile; let a_ = (sym::gCharRefState).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = fid::CharRef_StateVoice; let a_ = (sym::gCharRefState).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = fid::CharRef_StateComment; let a_ = (sym::gCharRefState).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = fid::CharRef_StateCostume; let a_ = (sym::gCharRefState).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = fid::CharRef_StateView; let a_ = (sym::gCharRefState).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = fid::CharRef_StateViewWait; let a_ = (sym::gCharRefState).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        F::CharRef_BuildList(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "CharRef_Init", &PROBE_N, &[], true); }
    r_
}

pub fn CharRef_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CharRef_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let mut sv_2: i32 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
    'fl1: loop {
        match bb_ {
            0 => {
                F::CharRef_Init(c, p_section_1);
                F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
                bb_ = 1; continue 'fl1;
            }
            1 => {
                if ((1i32) != 0) { bb_ = 2; continue 'fl1; } else { bb_ = 3; continue 'fl1; }
            }
            2 => {
                F::CharRef_UpdateImage(c);
                F::Gfx_BeginFrame(c);
                F::Pad_Update(c);
                F::Snd_Update(c);
                F::ColorFade_Update(c);
                if ((((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5672); (c.m.r32(a_) as i32) }) == (7i32)) as i32)) != 0) { bb_ = 4; continue 'fl1; } else { bb_ = 6; continue 'fl1; }
            }
            3 => {
                bb_ = 17; continue 'fl1;
            }
            4 => {
                F::ChrView_Update(c);
                bb_ = 5; continue 'fl1;
            }
            5 => {
                F::File_Stub264D90(c);
                F::Gfx_EndFrame(c, 1i32);
                F::Dma_Flush(c);
                sv_2 = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5672); (c.m.r32(a_) as i32) };
                bb_ = match sv_2 { 9i32 => 9, _ => 20, }; continue 'fl1;
            }
            6 => {
                F::CharRef_Draw(c);
                bb_ = 5; continue 'fl1;
            }
            7 => {
                bb_ = 1; continue 'fl1;
            }
            8 => {
                bb_ = 9; continue 'fl1;
            }
            9 => {
                if ((F::ColorFade_IsFadingOut(c)) != 0) { bb_ = 10; continue 'fl1; } else { bb_ = 12; continue 'fl1; }
            }
            10 => {
                F::Voice_FadeOutStep(c, 0i32);
                F::Bgm_FadeOutStep(c);
                bb_ = 11; continue 'fl1;
            }
            11 => {
                bb_ = 7; continue 'fl1;
            }
            12 => {
                if ((F::ColorFade_IsOutDone(c)) != 0) { bb_ = 13; continue 'fl1; } else { bb_ = 14; continue 'fl1; }
            }
            13 => {
                if ((((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5612); (c.m.r32(a_) as i32) }) == (4i32)) as i32)) != 0) { bb_ = 15; continue 'fl1; } else { bb_ = 16; continue 'fl1; }
            }
            14 => {
                bb_ = 11; continue 'fl1;
            }
            15 => {
                bb_ = 17; continue 'fl1;
            }
            16 => {
                F::CharRef_UpdateImage(c);
                bb_ = 14; continue 'fl1;
            }
            17 => {
                F::Dma_ResetBuffers(c);
                F::CharRef_Term(c);
                break 'f 0i32;
            }
            18 => {
                bb_ = 16; continue 'fl1;
            }
            19 => {
                bb_ = 20; continue 'fl1;
            }
            20 => {
                if ((((!(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0)) as i32)) != 0) { bb_ = 21; continue 'fl1; } else { bb_ = 22; continue 'fl1; }
            }
            21 => {
                { let fp_ = { let a_ = (sym::gCharRefState).wrapping_add(((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5672); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }; { let _ = D::call(c, fp_, &[]); } };
                bb_ = 22; continue 'fl1;
            }
            22 => {
                bb_ = 7; continue 'fl1;
            }
            23 => {
                bb_ = 7; continue 'fl1;
            }
            24 => {
                break 'fl1;
            }
            _ => unreachable!(),
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "CharRef_Run", &PROBE_N, &[], true); }
    r_
}

pub fn CharRef_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CharRef_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let mut l_flash_1: u32 = 0;
    let mut l_state_2: i32 = 0;
    let mut l_pose_3: i32 = 0;
    let mut l_chara_4: i32 = 0;
    let mut l_line_5: i32 = 0;
    let mut l_next_6: i32 = 0;
    let mut l_c_7: i32 = 0;
    let mut l_i_8: i32 = 0;
    let mut l_n_9: i32 = 0;
    let mut l_y_10: i32 = 0;
    let mut l_tex_11: i16 = 0;
    let mut l_scale_12: F32 = F32(0);
    let mut l_tbl_13: u32 = 0;
    let r_ = 'f: {
        l_flash_1 = ((c.m.r32(sym::gCharRef)).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        l_state_2 = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5672); (c.m.r32(a_) as i32) };
        l_pose_3 = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5660); (c.m.r32(a_) as i32) };
        l_chara_4 = { let a_ = (((c.m.r32(sym::gCharRef)).wrapping_add(400)).wrapping_add(((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5596); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(20))).wrapping_add(8); (c.m.r32(a_) as i32) };
        { let _ = F::Job_Run(c); };
        { let a0_ = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(156); c.m.r32(a_) }; F::Sprite_DrawPicture(c, a0_, 0i32, 0i32, 128i32) };
        { let a0_ = ((c.m.r32(sym::gCharRef)).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Advance(c, a0_) };
        if ((l_pose_3) != 0) {
            { let v_ = { let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((43i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((32i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        } else {
            { let v_ = { let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((42i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((32i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        }
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), (if ((l_pose_3) != 0) { sym::menu_h_b__str1 } else { sym::menu_h_b__str2 }), fp.wrapping_add(8));
        { let a2_ = (c.m.r32(sym::gCharRef)).wrapping_add(5616); F::FlashAnim_Blink(c, l_flash_1, fp.wrapping_add(8), a2_, 0i32) };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), (if ((l_pose_3) != 0) { sym::menu_h_b__str3 } else { sym::menu_h_b__str4 }), fp.wrapping_add(16));
        { let a2_ = (c.m.r32(sym::gCharRef)).wrapping_add(5620); F::FlashAnim_Talk(c, l_flash_1, fp.wrapping_add(16), a2_, 0i32) };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), (if ((((!((l_pose_3) != 0)) as i32)) != 0) { sym::menu_h_b__str5 } else { sym::menu_h_b__str6 }), fp);
        { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), (if ((((!((l_pose_3) != 0)) as i32)) != 0) { sym::menu_h_b__str7 } else { sym::menu_h_b__str8 }), fp);
        { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_h_b__str9, fp);
        { let a5_ = ((c.m.r32(sym::gCharRef)).wrapping_add(3636)).wrapping_add((((0i32) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, l_chara_4, a5_) };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_h_b__str10, fp);
        { let a5_ = ((c.m.r32(sym::gCharRef)).wrapping_add(3636)).wrapping_add((((1i32) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, l_chara_4, a5_) };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_h_b__str11, fp);
        l_line_5 = (l_chara_4).wrapping_mul(2i32);
        { let a5_ = ((c.m.r32(sym::gCharRef)).wrapping_add(3636)).wrapping_add((((2i32) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, l_line_5, a5_) };
        { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_h_b__str12, &[((1i32) as i64 as u64)]); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(24), fp);
        F::Flash_ClipSetCallbackA(c, l_flash_1, fp, (if (((((l_state_2) == (1i32)) as i32)) != 0) { fid::CharRef_ScissorOn } else { ((0i32) as u32) }), ((0i32) as u32));
        F::Flash_ClipSetCallbackB(c, l_flash_1, fp, fid::CharRef_ScissorOff, ((0i32) as u32));
        { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_h_b__str13, &[((4i32) as i64 as u64)]); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(24), fp);
        F::Flash_ClipSetCallbackA(c, l_flash_1, fp, (if (((((l_state_2) == (1i32)) as i32)) != 0) { fid::CharRef_ScissorOn } else { ((0i32) as u32) }), ((0i32) as u32));
        F::Flash_ClipSetCallbackB(c, l_flash_1, fp, fid::CharRef_ScissorOff, ((0i32) as u32));
        { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_h_b__str14, &[((5i32) as i64 as u64)]); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(24), fp);
        F::Flash_ClipSetCallbackA(c, l_flash_1, fp, (if (((((l_state_2) == (1i32)) as i32)) != 0) { fid::CharRef_ScissorOn } else { ((0i32) as u32) }), ((0i32) as u32));
        F::Flash_ClipSetCallbackB(c, l_flash_1, fp, fid::CharRef_ScissorOff, ((0i32) as u32));
        {
            l_i_8 = 0i32;
            'l1: loop {
                if !(((((l_i_8) < (5i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_n_9 = (({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5596); (c.m.r32(a_) as i32) }).wrapping_sub({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5632); (c.m.r32(a_) as i32) })).wrapping_add(l_i_8);
                    if (((((l_n_9) > (({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5600); (c.m.r32(a_) as i32) }).wrapping_sub(1i32))) as i32)) != 0) {
                        { let r_ = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5600); (c.m.r32(a_) as i32) }; let n_ = (l_n_9).wrapping_sub(r_); l_n_9 = n_; };
                    } else {
                        { let r_ = ((((l_n_9) < (0i32)) as i32)).wrapping_mul({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5600); (c.m.r32(a_) as i32) }); let n_ = (l_n_9).wrapping_add(r_); l_n_9 = n_; };
                    }
                    { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_h_b__str15, &[(((l_i_8).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(24), sym::menu_h_b__str16, fp);
                    { let a4_ = { let a_ = (((c.m.r32(sym::gCharRef)).wrapping_add(400)).wrapping_add((((l_n_9) as u32)).wrapping_mul(20))).wrapping_add(8); (c.m.r32(a_) as i32) }; let a5_ = ((c.m.r32(sym::gCharRef)).wrapping_add(3636)).wrapping_add(((((4i32).wrapping_add(l_i_8)) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, a4_, a5_) };
                    { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_h_b__str17, &[(((l_i_8).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(24), sym::menu_h_b__str18, fp);
                    { let a4_ = { let a_ = (((c.m.r32(sym::gCharRef)).wrapping_add(400)).wrapping_add((((l_n_9) as u32)).wrapping_mul(20))).wrapping_add(8); (c.m.r32(a_) as i32) }; let a5_ = ((c.m.r32(sym::gCharRef)).wrapping_add(3636)).wrapping_add(((((9i32).wrapping_add(l_i_8)) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, a4_, a5_) };
                }
                { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
            }
        }
        if ((((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5664); (c.m.r32(a_) as i32) }) == (4i32)) as i32)) != 0) {
            l_n_9 = (({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5596); (c.m.r32(a_) as i32) }).wrapping_sub({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5632); (c.m.r32(a_) as i32) })).wrapping_sub(1i32);
            if ((((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5596); (c.m.r32(a_) as i32) }) < (3i32)) as i32)) != 0) {
                l_n_9 = (({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5600); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5596); (c.m.r32(a_) as i32) })).wrapping_sub(4i32);
            }
            { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_h_b__str19, &[((l_i_8) as i64 as u64)]); };
            F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(24), sym::menu_h_b__str20, fp);
            { let a4_ = { let a_ = (((c.m.r32(sym::gCharRef)).wrapping_add(400)).wrapping_add((((l_n_9) as u32)).wrapping_mul(20))).wrapping_add(8); (c.m.r32(a_) as i32) }; let a5_ = ((c.m.r32(sym::gCharRef)).wrapping_add(3636)).wrapping_add(((((4i32).wrapping_add(4i32)) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, a4_, a5_) };
            { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_h_b__str21, &[((l_i_8) as i64 as u64)]); };
            F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(24), sym::menu_h_b__str22, fp);
            { let a4_ = { let a_ = (((c.m.r32(sym::gCharRef)).wrapping_add(400)).wrapping_add((((l_n_9) as u32)).wrapping_mul(20))).wrapping_add(8); (c.m.r32(a_) as i32) }; let a5_ = ((c.m.r32(sym::gCharRef)).wrapping_add(3636)).wrapping_add(((((9i32).wrapping_add(4i32)) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, a4_, a5_) };
        }
        { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        {
            l_i_8 = 0i32;
            'l3: loop {
                if !(((((l_i_8) < (5i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let v_ = (l_i_8).wrapping_mul(32i32); let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((l_i_8).wrapping_mul(32i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_h_b__str23, &[(((l_i_8).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(24), sym::menu_h_b__str24, fp);
                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(24), sym::menu_h_b__str25, fp);
                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
                }
                { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
            }
        }
        if (((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5680); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
            { let r_ = F32(0x3d999999); let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5684); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::fge({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5684); F32(c.m.r32(a_)) }, F32(0x3f800000)) as i32)) != 0) {
                { let v_ = F32(0x3f800000); let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5684); c.m.w32(a_, (v_).0); };
            }
        } else {
            { let v_ = F32(0x00000000); let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5684); c.m.w32(a_, (v_).0); };
        }
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_h_b__str26, fp);
        { let a2_ = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5684); F32(c.m.r32(a_)) }; F::Flash_ClipSetAlpha(c, l_flash_1, fp, a2_) };
        { let v_ = 0i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_h_b__str27, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
        { let v_ = 32i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_h_b__str28, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
        { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_h_b__str29, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
        l_scale_12 = rt::fmul(rt::fdiv(F32(0x40800000), rt::i2f({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5600); (c.m.r32(a_) as i32) })), F32(0x40e00000));
        l_y_10 = rt::div_i32((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5596); (c.m.r32(a_) as i32) }).wrapping_sub({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5632); (c.m.r32(a_) as i32) })).wrapping_mul(224i32), { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5600); (c.m.r32(a_) as i32) });
        if (((((l_y_10) < (0i32)) as i32)) != 0) {
            l_y_10 = 0i32;
        } else {
            if (((((l_y_10) > (218i32)) as i32)) != 0) {
                l_y_10 = 218i32;
            }
        }
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_h_b__str30, fp);
        F::Flash_ClipSetScale(c, l_flash_1, fp, F32(0x3f800000), l_scale_12);
        F::Flash_ClipSetOffset(c, l_flash_1, fp, 0i32, l_y_10);
        l_c_7 = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5608); (c.m.r32(a_) as i32) }).wrapping_add(1i32);
        's5: {
            let sv14: i32 = l_c_7;
            match sv14 {
                0i32 => {
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (l_c_7).wrapping_add(32i32); let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (l_c_7).wrapping_add(32i32); let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    break 's5;
                }
                1i32 => {
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 32i32; let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 32i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 64i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    break 's5;
                }
                2i32 => {
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 32i32; let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 64i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 96i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    break 's5;
                }
                3i32 => {
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 32i32; let a_ = (fp.wrapping_add(88)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 96i32; let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 128i32; let a_ = (fp.wrapping_add(88)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    break 's5;
                }
                _ => {
                }
            }
        }
        { let _ = F::sprintf(c, fp.wrapping_add(24), sym::menu_h_b__str31, &[((5i32) as i64 as u64)]); };
        F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(24), sym::menu_h_b__str32, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(88));
        l_tbl_13 = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
        l_tex_11 = (((((((({ let a_ = ((l_tbl_13).wrapping_add(((({ let a_ = (((c.m.r32(sym::gCharRef)).wrapping_add(400)).wrapping_add(((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5596); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(20))).wrapping_add(8); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(60))).wrapping_add(8); c.m.r16(a_) }) as i32)) ^ (1i32))) & (1i32))) as i16);
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_h_b__str33, fp);
        F::Flash_ClipSetTex(c, l_flash_1, fp, ((l_tex_11) as i32));
        if (({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5652); (c.m.r32(a_) as i32) }) != 0) {
            l_y_10 = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5596); (c.m.r32(a_) as i32) }).wrapping_sub(1i32);
        } else {
            l_y_10 = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5596); (c.m.r32(a_) as i32) }).wrapping_add(1i32);
        }
        l_next_6 = { let a_ = (((c.m.r32(sym::gCharRef)).wrapping_add(400)).wrapping_add((((l_y_10) as u32)).wrapping_mul(20))).wrapping_add(8); (c.m.r32(a_) as i32) };
        if (((((l_y_10) >= ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5600); (c.m.r32(a_) as i32) })) as i32)) != 0) {
            l_next_6 = { let a_ = (((c.m.r32(sym::gCharRef)).wrapping_add(400)).wrapping_add((((0i32) as u32)).wrapping_mul(20))).wrapping_add(8); (c.m.r32(a_) as i32) };
        } else {
            if (((((l_y_10) < (0i32)) as i32)) != 0) {
                l_next_6 = { let a_ = (((c.m.r32(sym::gCharRef)).wrapping_add(400)).wrapping_add((((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5600); (c.m.r32(a_) as i32) }).wrapping_sub(1i32)) as u32)).wrapping_mul(20))).wrapping_add(8); (c.m.r32(a_) as i32) };
            }
        }
        l_tex_11 = ((((((((c.m.r16(((l_tbl_13).wrapping_add((((l_next_6) as u32)).wrapping_mul(60))).wrapping_add(8))) as i32)) ^ (1i32))) & (1i32))) as i16);
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_h_b__str34, fp);
        F::Flash_ClipSetTex(c, l_flash_1, fp, ((l_tex_11) as i32));
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_h_b__str35, fp);
        F::Flash_ClipSetTex(c, l_flash_1, fp, ((l_tex_11) as i32));
        if (((((l_state_2) == (3i32)) as i32)) != 0) {
            F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_h_b__str36, fp);
            { let a3_ = ((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5644); (c.m.r32(a_) as i32) }) > (0i32)) as i32); { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((a3_) as i64 as u64)) as u8)); } };
            F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_h_b__str37, fp);
            { let a3_ = ((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5644); (c.m.r32(a_) as i32) }) < (({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5648); (c.m.r32(a_) as i32) }).wrapping_sub(110i32))) as i32); { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((a3_) as i64 as u64)) as u8)); } };
        }
        {
            l_i_8 = 0i32;
            'l6: loop {
                if !(((((l_i_8) < (1i32)) as i32)) != 0) { break 'l6; }
                'c7: {
                    { let a0_ = ((c.m.r32(sym::gCharRef)).wrapping_add(112)).wrapping_add((((l_i_8) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
                }
                { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
            }
        }
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_h_b__str38, fp);
        { let a3_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5644); (c.m.r32(a_) as i32) }).wrapping_neg(); let a5_ = ((c.m.r32(sym::gCharRef)).wrapping_add(3636)).wrapping_add((((3i32) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, a3_, (l_line_5).wrapping_add(1i32), a5_) };
        F::IconWin_Draw(c);
        if (((((((((((((l_state_2) == (0i32)) as i32)) != 0) || (((((l_state_2) == (1i32)) as i32)) != 0)) as i32)) != 0) || (((((l_state_2) == (9i32)) as i32)) != 0)) as i32)) != 0) {
            { let a2_ = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5624); (c.m.r32(a_) as i32) }; F::MsgWin_Draw(c, 0i32, 0i32, a2_) };
        } else {
            if (((((l_state_2) == (2i32)) as i32)) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5676); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) {
                    { let a2_ = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5624); (c.m.r32(a_) as i32) }; F::MsgWin_Draw(c, 0i32, 0i32, a2_) };
                } else {
                    { let a2_ = (({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5624); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5656); (c.m.r32(a_) as i32) })).wrapping_add({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5640); (c.m.r32(a_) as i32) }); F::MsgWin_Draw(c, 0i32, 0i32, a2_) };
                }
            } else {
                { let a2_ = (({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5624); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5656); (c.m.r32(a_) as i32) })).wrapping_add({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5640); (c.m.r32(a_) as i32) }); F::MsgWin_Draw(c, 0i32, 0i32, a2_) };
            }
        }
        F::ColorFade_Draw(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "CharRef_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn CharRef_StateIntro(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CharRef_StateIntro", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((({ let a_ = (((c.m.r32(sym::gCharRef)).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0) {
        F::CharRef_ClipGoto(c, 0i32, 0i32, sym::menu_h_b__str39);
        { let v_ = 1i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5672); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5624); c.m.w32(a_, (v_) as u32); };
        { let a0_ = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(20); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5624); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 33463i32, a2_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "CharRef_StateIntro", &PROBE_N, &[], true); }
    r_
}

pub fn CharRef_StateList(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CharRef_StateList", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((!(((({ let a_ = (((c.m.r32(sym::gCharRef)).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f;
    }
    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
        F::CharRef_ClipGoto(c, 0i32, 0i32, sym::menu_h_b__str40);
        if ((((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5632); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) < (0i32)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5632); c.m.w32(a_, (v_) as u32); };
            { let v_ = 8i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5664); c.m.w32(a_, (v_) as u32); };
            { let a0_ = ((c.m.r32(sym::gCharRef)).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_h_b__str41, ((((1i32) as i64 as u64)) as u8)) };
        }
        if ((((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5596); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) < (0i32)) as i32)) != 0) {
            { let v_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5600); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5596); c.m.w32(a_, (v_) as u32); };
        }
        F::CharRef_ClipGoto(c, 0i32, 0i32, sym::menu_h_b__str42);
        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5636); c.m.w32(a_, (v_) as u32); };
    } else {
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
            F::CharRef_ClipGoto(c, 0i32, 0i32, sym::menu_h_b__str43);
            if ((((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5632); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) >= (4i32)) as i32)) != 0) {
                { let v_ = (4i32).wrapping_sub(1i32); let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5632); c.m.w32(a_, (v_) as u32); };
                { let v_ = 4i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5664); c.m.w32(a_, (v_) as u32); };
                { let a0_ = ((c.m.r32(sym::gCharRef)).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_h_b__str44, ((((1i32) as i64 as u64)) as u8)) };
            }
            if ((((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5596); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) > (({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5600); (c.m.r32(a_) as i32) }).wrapping_sub(1i32))) as i32)) != 0) {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5596); c.m.w32(a_, (v_) as u32); };
            }
            F::CharRef_ClipGoto(c, 0i32, 0i32, sym::menu_h_b__str45);
            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5636); c.m.w32(a_, (v_) as u32); };
        } else {
            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) {
                { let r_ = 4i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5596); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                if ((((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5596); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                    { let v_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5600); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5596); c.m.w32(a_, (v_) as u32); };
                }
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5636); c.m.w32(a_, (v_) as u32); };
            } else {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) {
                    { let r_ = 4i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5596); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                    if ((((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5596); (c.m.r32(a_) as i32) }) > (({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5600); (c.m.r32(a_) as i32) }).wrapping_sub(1i32))) as i32)) != 0) {
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5596); c.m.w32(a_, (v_) as u32); };
                    }
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5636); c.m.w32(a_, (v_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        F::CharRef_ClipGoto(c, 0i32, 0i32, sym::menu_h_b__str46);
                        { let v_ = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5672); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5676); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 2i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5672); c.m.w32(a_, (v_) as u32); };
                        F::MsgWin_Close(c);
                        F::IconWin_Close(c);
                        F::CharRef_ClipGoto(c, 0i32, 1i32, sym::menu_h_b__str47);
                        F::CharRef_ChangeImage(c);
                        { let a0_ = ((c.m.r32(sym::gCharRef)).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_h_b__str48, ((((1i32) as i64 as u64)) as u8)) };
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5636); c.m.w32(a_, (v_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                        if (((((F::Voice_GetStat(c, 0i32)) != (5i32)) as i32)) != 0) {
                            F::Voice_StopWithLip(c);
                        }
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                            F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                            { let v_ = 9i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5672); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5636); c.m.w32(a_, (v_) as u32); };
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                        } else {
                            if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                                { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5636); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                if ((((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5636); (c.m.r32(a_) as i32) }) > (300i32)) as i32)) != 0) {
                                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5636); c.m.w32(a_, (v_) as u32); };
                                    'l1: while ((1i32) != 0) {
                                        { let v_ = (((F::Rand_Range(c, ((7i32) as u32))).wrapping_add(((2i32) as u32))) as i32); let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5624); c.m.w32(a_, (v_) as u32); };
                                        if ((((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5624); (c.m.r32(a_) as i32) }) != ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5668); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                            { let v_ = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5624); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5668); c.m.w32(a_, (v_) as u32); };
                                            break 'l1;
                                        }
                                    }
                                    { let a0_ = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(20); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5624); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 33463i32, a2_) };
                                }
                            }
                        }
                    }
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "CharRef_StateList", &PROBE_N, &[], true); }
    r_
}

pub fn CharRef_StateDetails(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CharRef_StateDetails", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((!(((({ let a_ = (((c.m.r32(sym::gCharRef)).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f;
    }
    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
        F::CharRef_ClipGoto(c, 0i32, 1i32, sym::menu_h_b__str49);
        if ((((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5628); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) < (0i32)) as i32)) != 0) {
            { let v_ = 3i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5628); c.m.w32(a_, (v_) as u32); };
        }
        F::CharRef_ClipGoto(c, 0i32, 1i32, sym::menu_h_b__str50);
        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
    } else {
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
            F::CharRef_ClipGoto(c, 0i32, 1i32, sym::menu_h_b__str51);
            if ((((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5628); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) >= (4i32)) as i32)) != 0) {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5628); c.m.w32(a_, (v_) as u32); };
            }
            F::CharRef_ClipGoto(c, 0i32, 1i32, sym::menu_h_b__str52);
            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
        } else {
            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5596); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) < (0i32)) as i32)) != 0) {
                    { let v_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5600); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5596); c.m.w32(a_, (v_) as u32); };
                }
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5652); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                { let a0_ = ((c.m.r32(sym::gCharRef)).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_h_b__str53, ((((1i32) as i64 as u64)) as u8)) };
                F::CharRef_ChangeImage(c);
            } else {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) {
                    if ((((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5596); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) > (({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5600); (c.m.r32(a_) as i32) }).wrapping_sub(1i32))) as i32)) != 0) {
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5596); c.m.w32(a_, (v_) as u32); };
                    }
                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5652); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    { let a0_ = ((c.m.r32(sym::gCharRef)).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_h_b__str54, ((((1i32) as i64 as u64)) as u8)) };
                    F::CharRef_ChangeImage(c);
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        F::CharRef_ClipGoto(c, 0i32, 1i32, sym::menu_h_b__str55);
                        if ((((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5628); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5644); c.m.w32(a_, (v_) as u32); };
                            F::CharRef_MeasureText(c);
                            { let v_ = 3i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5672); c.m.w32(a_, (v_) as u32); };
                            { let a0_ = ((c.m.r32(sym::gCharRef)).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_h_b__str56, ((((1i32) as i64 as u64)) as u8)) };
                        } else {
                            if ((((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5628); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) {
                                { let a1_ = { let a_ = (((c.m.r32(sym::gCharRef)).wrapping_add(400)).wrapping_add(((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5596); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(20))).wrapping_add(8); (c.m.r32(a_) as i32) }; let a2_ = rt::rem_i32(F::Port_Rand(c), 2i32); F::Voice_PlayChara(c, 0i32, a1_, a2_) };
                                { let v_ = 4i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5672); c.m.w32(a_, (v_) as u32); };
                                { let a0_ = ((c.m.r32(sym::gCharRef)).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_h_b__str57, ((((1i32) as i64 as u64)) as u8)) };
                            } else {
                                if ((((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5628); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) {
                                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5624); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 9i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5656); c.m.w32(a_, (v_) as u32); };
                                    { let r_ = { let a_ = (((c.m.r32(sym::gCharRef)).wrapping_add(400)).wrapping_add(((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5596); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(20))).wrapping_add(12); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5656); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5640); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 5i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5672); c.m.w32(a_, (v_) as u32); };
                                    F::MsgWin_SetSide(c, 1i32);
                                    F::MsgWin_Open(c);
                                    { let v_ = { let a_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(3632); c.m.r32(a_) }).wrapping_add(((((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5656); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5640); (c.m.r32(a_) as i32) })).wrapping_sub(9i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5660); c.m.w32(a_, (v_) as u32); };
                                    { let a0_ = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(20); c.m.r32(a_) }; let a2_ = (({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5624); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5656); (c.m.r32(a_) as i32) })).wrapping_add({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5640); (c.m.r32(a_) as i32) }); F::Voice_PlayWithSubtitle(c, a0_, 33463i32, a2_) };
                                    { let a0_ = ((c.m.r32(sym::gCharRef)).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_h_b__str58, ((((1i32) as i64 as u64)) as u8)) };
                                } else {
                                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5608); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 5i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5628); c.m.w32(a_, (v_) as u32); };
                                    F::CharRef_ClipGoto(c, 0i32, 1i32, sym::menu_h_b__str59);
                                    F::CharRef_ClipGoto(c, 0i32, 2i32, sym::menu_h_b__str60);
                                    { let v_ = 6i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5672); c.m.w32(a_, (v_) as u32); };
                                    { let a0_ = ((c.m.r32(sym::gCharRef)).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_h_b__str61, ((((1i32) as i64 as u64)) as u8)) };
                                }
                            }
                        }
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                            F::MsgWin_SetSide(c, 0i32);
                            { let a0_ = ((c.m.r32(sym::gCharRef)).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_h_b__str62, ((((1i32) as i64 as u64)) as u8)) };
                            F::CharRef_ClipGoto(c, 0i32, 1i32, sym::menu_h_b__str63);
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5628); c.m.w32(a_, (v_) as u32); };
                            F::MsgWin_Open(c);
                            F::IconWin_Open(c);
                            { let v_ = 1i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5672); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5660); c.m.w32(a_, (v_) as u32); };
                            { let a0_ = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(20); c.m.r32(a_) }; F::Voice_PlayWithSubtitle(c, a0_, 33463i32, 1i32) };
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                            { let v_ = 1i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5624); c.m.w32(a_, (v_) as u32); };
                        }
                    }
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "CharRef_StateDetails", &PROBE_N, &[], true); }
    r_
}

pub fn CharRef_StateProfile(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CharRef_StateProfile", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((!(((({ let a_ = (((c.m.r32(sym::gCharRef)).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f;
    }
    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(392))) & (((8i32) as u32)))) != 0) {
        if ((((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5644); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            { let r_ = 2i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5644); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
        }
    } else {
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(392))) & (((4i32) as u32)))) != 0) {
            if ((((!(((((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5648); (c.m.r32(a_) as i32) }).wrapping_sub(110i32)) < ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5644); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
                { let r_ = 2i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5644); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
            }
        } else {
            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                { let a0_ = ((c.m.r32(sym::gCharRef)).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_h_b__str64, ((((1i32) as i64 as u64)) as u8)) };
                F::CharRef_ClipGoto(c, 0i32, 1i32, sym::menu_h_b__str65);
                { let v_ = 2i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5672); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "CharRef_StateProfile", &PROBE_N, &[], true); }
    r_
}

pub fn CharRef_StateVoice(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CharRef_StateVoice", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((!(((({ let a_ = (((c.m.r32(sym::gCharRef)).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f;
    }
    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
        F::Voice_Stop(c, 0i32);
        { let a1_ = { let a_ = (((c.m.r32(sym::gCharRef)).wrapping_add(400)).wrapping_add(((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5596); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(20))).wrapping_add(8); (c.m.r32(a_) as i32) }; let a2_ = rt::rem_i32(F::Port_Rand(c), 2i32); F::Voice_PlayChara(c, 0i32, a1_, a2_) };
    }
    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
        { let a0_ = ((c.m.r32(sym::gCharRef)).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_h_b__str66, ((((1i32) as i64 as u64)) as u8)) };
        F::CharRef_ClipGoto(c, 0i32, 1i32, sym::menu_h_b__str67);
        { let v_ = 2i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5672); c.m.w32(a_, (v_) as u32); };
        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
        F::Voice_Stop(c, 0i32);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "CharRef_StateVoice", &PROBE_N, &[], true); }
    r_
}

pub fn CharRef_StateComment(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CharRef_StateComment", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((!(((({ let a_ = (((c.m.r32(sym::gCharRef)).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f;
    }
    if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
        if ((((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5640); (c.m.r32(a_) as i32) }) < (({ let a_ = (((c.m.r32(sym::gCharRef)).wrapping_add(400)).wrapping_add(((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5596); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(20))).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_sub(1i32))) as i32)) != 0) {
            { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5640); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            { let v_ = { let a_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(3632); c.m.r32(a_) }).wrapping_add(((((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5656); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5640); (c.m.r32(a_) as i32) })).wrapping_sub(9i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5660); c.m.w32(a_, (v_) as u32); };
            { let a0_ = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(20); c.m.r32(a_) }; let a2_ = (({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5624); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5656); (c.m.r32(a_) as i32) })).wrapping_add({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5640); (c.m.r32(a_) as i32) }); F::Voice_PlayWithSubtitle(c, a0_, 33463i32, a2_) };
        } else {
            F::MsgWin_Close(c);
            { let a0_ = ((c.m.r32(sym::gCharRef)).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_h_b__str68, ((((1i32) as i64 as u64)) as u8)) };
            F::CharRef_ClipGoto(c, 0i32, 1i32, sym::menu_h_b__str69);
            { let v_ = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5672); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5676); c.m.w32(a_, (v_) as u32); };
            { let v_ = 2i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5672); c.m.w32(a_, (v_) as u32); };
            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
        }
    } else {
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5640); (c.m.r32(a_) as i32) }) < (({ let a_ = (((c.m.r32(sym::gCharRef)).wrapping_add(400)).wrapping_add(((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5596); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(20))).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_sub(1i32))) as i32)) != 0) {
                { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5640); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = { let a_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(3632); c.m.r32(a_) }).wrapping_add(((((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5656); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5640); (c.m.r32(a_) as i32) })).wrapping_sub(9i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5660); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(20); c.m.r32(a_) }; let a2_ = (({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5624); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5656); (c.m.r32(a_) as i32) })).wrapping_add({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5640); (c.m.r32(a_) as i32) }); F::Voice_PlayWithSubtitle(c, a0_, 33463i32, a2_) };
            } else {
                F::MsgWin_Close(c);
                { let a0_ = ((c.m.r32(sym::gCharRef)).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_h_b__str70, ((((1i32) as i64 as u64)) as u8)) };
                F::CharRef_ClipGoto(c, 0i32, 1i32, sym::menu_h_b__str71);
                { let v_ = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5672); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5676); c.m.w32(a_, (v_) as u32); };
                { let v_ = 2i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5672); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                F::Voice_StopWithLip(c);
            }
        } else {
            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                F::MsgWin_Close(c);
                { let a0_ = ((c.m.r32(sym::gCharRef)).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_h_b__str72, ((((1i32) as i64 as u64)) as u8)) };
                F::CharRef_ClipGoto(c, 0i32, 1i32, sym::menu_h_b__str73);
                { let v_ = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5672); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5676); c.m.w32(a_, (v_) as u32); };
                { let v_ = 2i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5672); c.m.w32(a_, (v_) as u32); };
                F::Voice_StopWithLip(c);
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "CharRef_StateComment", &PROBE_N, &[], true); }
    r_
}

pub fn CharRef_StateCostume(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CharRef_StateCostume", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((!(((({ let a_ = (((c.m.r32(sym::gCharRef)).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f;
    }
    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((8i32) as u32)))) != 0) {
        if ((((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5608); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) < (0i32)) as i32)) != 0) {
            { let v_ = ({ let a0_ = { let a_ = (((c.m.r32(sym::gCharRef)).wrapping_add(400)).wrapping_add(((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5596); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(20))).wrapping_add(8); (c.m.r32(a_) as i32) }; F::ChrTbl_WrapCostume(c, a0_, ((0i32) as u32)) }).wrapping_sub(1i32); let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5608); c.m.w32(a_, (v_) as u32); };
        }
        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
    } else {
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((4i32) as u32)))) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5608); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) > (({ let a0_ = { let a_ = (((c.m.r32(sym::gCharRef)).wrapping_add(400)).wrapping_add(((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5596); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(20))).wrapping_add(8); (c.m.r32(a_) as i32) }; F::ChrTbl_WrapCostume(c, a0_, ((0i32) as u32)) }).wrapping_sub(1i32))) as i32)) != 0) {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5608); c.m.w32(a_, (v_) as u32); };
            }
            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
        } else {
            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                F::CharRef_ClipGoto(c, 0i32, 2i32, sym::menu_h_b__str74);
                { let v_ = 8i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5672); c.m.w32(a_, (v_) as u32); };
                { let _ = { let a0_ = { let a_ = (((c.m.r32(sym::gCharRef)).wrapping_add(400)).wrapping_add(((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5596); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(20))).wrapping_add(8); (c.m.r32(a_) as i32) }; let a1_ = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5608); (c.m.r32(a_) as i32) }; F::ChrView_Show(c, a0_, a1_, 0i32) }; };
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
            } else {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                    F::CharRef_ClipGoto(c, 0i32, 2i32, sym::menu_h_b__str75);
                    { let v_ = 3i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5628); c.m.w32(a_, (v_) as u32); };
                    { let a0_ = ((c.m.r32(sym::gCharRef)).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_h_b__str76, ((((1i32) as i64 as u64)) as u8)) };
                    F::CharRef_ClipGoto(c, 0i32, 1i32, sym::menu_h_b__str77);
                    { let v_ = 2i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5672); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "CharRef_StateCostume", &PROBE_N, &[], true); }
    r_
}

pub fn CharRef_StateView(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CharRef_StateView", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((!(((({ let a_ = (((c.m.r32(sym::gCharRef)).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f;
    }
    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
        { let _ = F::ChrView_Hide(c); };
        F::CharRef_ClipGoto(c, 0i32, 2i32, sym::menu_h_b__str78);
        { let v_ = 6i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5672); c.m.w32(a_, (v_) as u32); };
        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "CharRef_StateView", &PROBE_N, &[], true); }
    r_
}

pub fn CharRef_StateViewWait(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CharRef_StateViewWait", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((({ let a_ = (((c.m.r32(sym::gCharRef)).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0) {
        if ((F::ChrView_IsVisible(c)) != 0) {
            { let v_ = 7i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5672); c.m.w32(a_, (v_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "CharRef_StateViewWait", &PROBE_N, &[], true); }
    r_
}

pub fn CharRef_ClipGoto(c: &mut Ctx, mut p_unused_1: i32, mut p_kind_2: i32, mut p_label_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CharRef_ClipGoto", &PROBE_N, &[(p_unused_1) as u32, (p_kind_2) as u32, (p_label_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_flash_4: u32 = 0;
    let r_ = 'f: {
        l_flash_4 = ((c.m.r32(sym::gCharRef)).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        's1: {
            let sv5: i32 = p_kind_2;
            match sv5 {
                1i32 => {
                    { let _ = { let a2_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5628); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_h_b__str79, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                0i32 => {
                    { let _ = { let a2_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5632); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_h_b__str80, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                2i32 => {
                    { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_h_b__str81, &[((5i32) as i64 as u64)]); };
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
    if rt::probing() { crate::host::harness::probe(c, "CharRef_ClipGoto", &PROBE_N, &[], true); }
    r_
}

pub fn CharRef_BuildList(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CharRef_BuildList", &PROBE_N, &[], false); }
    let mut l_prev_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_n_3: i32 = 0;
    let mut l_chara_4: i32 = 0;
    let mut l_line_5: i32 = 0;
    let mut l_num_6: i32 = 0;
    let mut l_page_7: i32 = 0;
    let mut l_perPage_8: i32 = 0;
    let mut l_perPage_9: i32 = 0;
    let r_ = 'f: {
    l_prev_1 = ((0i32) as u32);
    l_i_2 = 0i32;
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5600); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(396); c.m.w32(a_, v_); };
    { let _ = { let a0_ = (c.m.r32(sym::gCharRef)).wrapping_add(400); F::memset(c, a0_, 0i32, 3220u32) }; };
    l_n_3 = 0i32;
    {
        'l1: loop {
            if !(((((l_i_2) < ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5604); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l1; }
            'c2: {
                l_chara_4 = { let a_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(3620); c.m.r32(a_) }).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                l_line_5 = { let a_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(3624); c.m.r32(a_) }).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                l_num_6 = { let a_ = ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(3628); c.m.r32(a_) }).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                if (((((l_chara_4) < (0i32)) as i32)) != 0) {
                    break 'c2;
                }
                if ((((!((((((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3088)).wrapping_add(((((l_chara_4).wrapping_shr((6i32) as u32)) as u32)).wrapping_mul(8)); c.m.r64(a_) }).wrapping_shr(((l_chara_4).wrapping_sub(((l_chara_4).wrapping_shr((6i32) as u32)).wrapping_shl((6i32) as u32))) as u32)) & (((1i32) as u64)))) as i32)) != 0)) as i32)) != 0) {
                    l_perPage_8 = 3i32;
                    l_page_7 = rt::div_i32(l_i_2, l_perPage_8);
                    break 'c2;
                }
                {
                    l_perPage_9 = 3i32;
                    l_page_7 = rt::div_i32(l_i_2, l_perPage_9);
                }
                if ((((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(396); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
                    { let v_ = ((c.m.r32(sym::gCharRef)).wrapping_add(400)).wrapping_add((((l_n_3) as u32)).wrapping_mul(20)); let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(396); c.m.w32(a_, v_); };
                }
                { let v_ = l_prev_1; let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(400)).wrapping_add((((l_n_3) as u32)).wrapping_mul(20)); c.m.w32(a_, v_); };
                { let v_ = l_chara_4; let a_ = (((c.m.r32(sym::gCharRef)).wrapping_add(400)).wrapping_add((((l_n_3) as u32)).wrapping_mul(20))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = l_line_5; let a_ = (((c.m.r32(sym::gCharRef)).wrapping_add(400)).wrapping_add((((l_n_3) as u32)).wrapping_mul(20))).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                { let v_ = l_num_6; let a_ = (((c.m.r32(sym::gCharRef)).wrapping_add(400)).wrapping_add((((l_n_3) as u32)).wrapping_mul(20))).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5600); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                if (((((l_prev_1) != (((0i32) as u32))) as i32)) != 0) {
                    { let v_ = ((c.m.r32(sym::gCharRef)).wrapping_add(400)).wrapping_add((((l_n_3) as u32)).wrapping_mul(20)); let a_ = (l_prev_1).wrapping_add(4); c.m.w32(a_, v_); };
                }
                l_prev_1 = ((c.m.r32(sym::gCharRef)).wrapping_add(400)).wrapping_add((((l_n_3) as u32)).wrapping_mul(20));
                { let o_ = l_n_3; let n_ = (o_).wrapping_add(1); l_n_3 = n_; };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "CharRef_BuildList", &PROBE_N, &[], true); }
    r_
}

pub fn CharRef_MeasureText(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CharRef_MeasureText", &PROBE_N, &[], false); }
    let mut l_e_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_e_1 = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(396); c.m.r32(a_) };
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5596); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l1; }
            'c2: {
                l_e_1 = c.m.r32((l_e_1).wrapping_add(4));
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    { let v_ = ({ let a0_ = ({ let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(24)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(24)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((((((c.m.r32((l_e_1).wrapping_add(8)) as i32)).wrapping_mul(2i32)).wrapping_add(2i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::Font_GetHeight(c, a0_) }).wrapping_add(4i32); let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5648); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "CharRef_MeasureText", &PROBE_N, &[], true); }
    r_
}

pub fn CharRef_UpdateImage(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CharRef_UpdateImage", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_e_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        's1: {
            let sv3: i32 = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5612); (c.m.r32(a_) as i32) };
            match sv3 {
                4i32 => {
                    if (((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5680); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                        { let v_ = 6i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5612); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                6i32 => {
                    if (((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5680); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                        { let r_ = 1i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5680); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5612); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                1i32 => {
                    l_e_1 = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(396); c.m.r32(a_) };
                    {
                        l_i_2 = 0i32;
                        'l2: loop {
                            if !(((((l_i_2) < ({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5596); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l2; }
                            'c3: {
                                l_e_1 = c.m.r32((l_e_1).wrapping_add(4));
                            }
                            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                        }
                    }
                    F::File_CancelRequests(c);
                    { let _ = { let a0_ = ((c.m.r32((l_e_1).wrapping_add(8)) as i32)).wrapping_add(761i32); let a1_ = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(4); c.m.r32(a_) }; let _ = 92160i32; F::File_Request(c, a0_, a1_) }; };
                    { let v_ = 2i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5612); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                2i32 => {
                    if ((F::File_UpdateRequests(c)) != 0) {
                        { let v_ = 3i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5612); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                3i32 => {
                    { let v_ = ((0i32) as u32); let a_ = fp; c.m.w32(a_, v_); };
                    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(4); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(8); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, a1_, ((0i32) as u32)) }; };
                    { let v_ = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(8); c.m.r32(a_) }; let a_ = fp; c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharRef)).wrapping_add(160)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let r_ = 2i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5680); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 4i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5612); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                5i32 => {
                    { let v_ = 6i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5612); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                _ => {
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "CharRef_UpdateImage", &PROBE_N, &[], true); }
    r_
}

pub fn CharRef_ChangeImage(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CharRef_ChangeImage", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5680); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
        { let r_ = 2i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5680); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        { let v_ = 5i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5612); c.m.w32(a_, (v_) as u32); };
    }
    { let r_ = 1i32; let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(5680); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "CharRef_ChangeImage", &PROBE_N, &[], true); }
    r_
}

pub fn CharRef_ScissorOn(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CharRef_ScissorOn", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Sprite_SetScissor(c, 0i32, 512i32, 73i32, 309i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "CharRef_ScissorOn", &PROBE_N, &[], true); }
    r_
}

pub fn CharRef_ScissorOff(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CharRef_ScissorOff", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Sprite_SetScissor(c, 0i32, 511i32, 0i32, 447i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "CharRef_ScissorOff", &PROBE_N, &[], true); }
    r_
}

pub fn CharRef_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CharRef_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    F::ChrView_Term(c);
    F::MsgWin_Term(c);
    F::IconWin_Term(c);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gCharRef)).wrapping_add(112)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Destroy(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(8); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(8); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(8); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gCharRef)).wrapping_add(12); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gCharRef)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gCharRef); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gCharRef; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "CharRef_Term", &PROBE_N, &[], true); }
    r_
}
