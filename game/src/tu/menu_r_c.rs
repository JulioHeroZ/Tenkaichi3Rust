#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn SimResult_YesNoGoto(c: &mut Ctx, mut p_label_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimResult_YesNoGoto", &PROBE_N, &[(p_label_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_flash_2: u32 = 0;
    let r_ = 'f: {
        l_flash_2 = ((c.m.r32(sym::gSimResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        { let _ = { let a2_ = ({ let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1256)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_r_c__str0, &[((a2_) as i64 as u64)]) }; };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, l_flash_2, fp, p_label_1);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SimResult_YesNoGoto", &PROBE_N, &[], true); }
    r_
}

pub fn SimResult_Init(c: &mut Ctx, mut p_section_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimResult_Init", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_i_2: i32 = 0;
    let mut l_span_3: i32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = F::Heap_Alloc(c, ((2092u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gSimResult; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gSimResult); F::memset(c, a0_, 0i32, 2092u32) }; };
        { let v_ = (c.m.r32(sym::gMenuArc3)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc3)).wrapping_add((((p_section_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = c.m.r32(sym::gSimResult); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gSimResult); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = c.m.r32(fp); let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1052); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((46i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((47i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((8i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((48i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((7i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((8i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((9i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((39i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((10i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((38i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((11i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((40i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((12i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((37i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((13i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((34i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((14i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((35i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((15i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((32i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((16i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((31i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((17i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((33i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((19i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((20i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((21i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((22i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((23i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((25i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((26i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((27i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((28i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((29i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((30i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((31i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((32i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((33i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((34i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((35i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((36i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((37i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((38i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((39i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((40i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((43i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((41i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((44i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((42i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((45i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((43i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((30i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((36i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((41i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1056)).wrapping_add((((42i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gSimResult)).wrapping_add(1056); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let v_ = rt::rem_i32(F::Rand_Libc(c), 32i32); let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1372); c.m.w32(a_, (v_) as u32); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = c.m.r32(fp); F::IconWin_Init(c, a0_, a1_) };
        F::IconWin_Open(c);
        F::IconWin_SetIcon(c, 0i32);
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(16); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(20); c.m.w32(a_, v_); };
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (6i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a0_ = ((c.m.r32(sym::gSimResult)).wrapping_add(24)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); let a1_ = { let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(12); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, ((0i32) as u32)) };
                    { let a0_ = ((c.m.r32(sym::gSimResult)).wrapping_add(24)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); F::TextBox_SetUnk50(c, a0_, 0i32) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let a0_ = (c.m.r32(sym::gSimResult)).wrapping_add(864); let a1_ = { let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(20); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, ((0i32) as u32)) };
        { let a0_ = (c.m.r32(sym::gSimResult)).wrapping_add(864); F::TextBox_SetUnk50(c, a0_, 0i32) };
        { let a0_ = (c.m.r32(sym::gSimResult)).wrapping_add(864); F::TextBox_SetLineOffsets(c, a0_, 12i32, 0i32, 0i32, 0i32, 0i32) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1004); c.m.w32(a_, v_); };
        { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1364); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1384); c.m.w32(a_, (v_) as u32); };
        { let v_ = 15i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1336); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((0i32) as u8); let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1400); c.m.w8(a_, v_); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(2084); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(2088); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1360); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1396); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a1_ = (c.m.r32(sym::gSimResult)).wrapping_add(1404); let a2_ = (c.m.r32(sym::gSimResult)).wrapping_add(1380); F::UbScore_Fill(c, 0i32, a1_, a2_) }; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1388); c.m.w32(a_, (v_) as u32); };
        { let a0_ = { let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1004); c.m.r32(a_) }; let a1_ = (c.m.r32(sym::gSimResult)).wrapping_add(1404); F::UbScore_CalcPoints(c, a0_, a1_) };
        { let _ = { let a1_ = (c.m.r32(sym::gSimResult)).wrapping_add(1404); F::UbScore_CalcRank(c, 0i32, a1_) }; };
        if ((((({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(40); (c.m.r32(a_) as i32) }) >= (69i32)) as i32)) != 0) {
            { let v_ = 1i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1392); c.m.w32(a_, (v_) as u32); };
            if ((((!(({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(648); c.m.r8(a_) }) != 0)) as i32)) != 0) {
                { let v_ = ((1i32) as u8); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(648); c.m.w8(a_, v_); };
                { let v_ = 1i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1396); c.m.w32(a_, (v_) as u32); };
                { let a0_ = F::UbScore_GetRewardItem(c, 0i32); F::Save_AddItem(c, a0_) };
            }
        } else {
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1392); c.m.w32(a_, (v_) as u32); };
        }
        { let v_ = { let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1404); c.m.w32(a_, (v_) as u32); };
        {
            rt::memset(c, fp.wrapping_add(4), 0, 28);
            { let v_ = 4i32; c.m.w32(fp.wrapping_add(4).wrapping_add(0), (v_) as u32); }
            { let v_ = 1i32; c.m.w32(fp.wrapping_add(4).wrapping_add(4), (v_) as u32); }
            { let v_ = 2i32; c.m.w32(fp.wrapping_add(4).wrapping_add(8), (v_) as u32); }
            { let v_ = 0i32; c.m.w32(fp.wrapping_add(4).wrapping_add(12), (v_) as u32); }
            { let v_ = 6i32; c.m.w32(fp.wrapping_add(4).wrapping_add(16), (v_) as u32); }
            { let v_ = 4i32; c.m.w32(fp.wrapping_add(4).wrapping_add(20), (v_) as u32); }
            { let v_ = 8i32; c.m.w32(fp.wrapping_add(4).wrapping_add(24), (v_) as u32); }
            l_span_3 = 10i32;
            { let a0_ = ({ let a_ = (fp.wrapping_add(4)).wrapping_add((((rt::div_i32({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(40); (c.m.r32(a_) as i32) }, l_span_3)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(68374i32); F::Bgm_Play(c, a0_) };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SimResult_Init", &PROBE_N, &[], true); }
    r_
}

pub fn SimResult_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimResult_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    F::IconWin_Term(c);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1008)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Destroy(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gSimResult)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gSimResult); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gSimResult; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SimResult_Term", &PROBE_N, &[], true); }
    r_
}

pub fn SimResult_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimResult_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_flash_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_x_3: i32 = 0;
    let mut l_line_4: i32 = 0;
    let r_ = 'f: {
        { let a0_ = { let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1052); c.m.r32(a_) }; F::Sprite_DrawPicture(c, a0_, 0i32, 0i32, 128i32) };
        l_flash_1 = ((c.m.r32(sym::gSimResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        F::Flash_FindLabel(c, l_flash_1, sym::menu_r_c__str1, sym::menu_r_c__str2, fp);
        { let a2_ = (c.m.r32(sym::gSimResult)).wrapping_add(1372); F::FlashAnim_Blink(c, l_flash_1, fp, a2_, 0i32) };
        F::Flash_FindLabel(c, l_flash_1, sym::menu_r_c__str3, sym::menu_r_c__str4, fp);
        { let a2_ = (c.m.r32(sym::gSimResult)).wrapping_add(1376); F::FlashAnim_Talk(c, l_flash_1, fp, a2_, 0i32) };
        if ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1388); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            F::Flash_FindLabel(c, l_flash_1, sym::menu_r_c__str5, sym::menu_r_c__str6, fp);
            { let a5_ = ((c.m.r32(sym::gSimResult)).wrapping_add(24)).wrapping_add((((3i32) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, 321i32, a5_) };
            F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_r_c__str7, fp);
            { let a5_ = ((c.m.r32(sym::gSimResult)).wrapping_add(24)).wrapping_add((((4i32) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, 321i32, a5_) };
        } else {
            F::Flash_FindLabel(c, l_flash_1, sym::menu_r_c__str8, sym::menu_r_c__str9, fp);
            { let a5_ = ((c.m.r32(sym::gSimResult)).wrapping_add(24)).wrapping_add((((3i32) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, 322i32, a5_) };
            F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_r_c__str10, fp);
            { let a5_ = ((c.m.r32(sym::gSimResult)).wrapping_add(24)).wrapping_add((((4i32) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, 322i32, a5_) };
        }
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_r_c__str11, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_r_c__str12, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_r_c__str13, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        if ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1384); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
        } else {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
        }
        { let v_ = 32i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_r_c__str14, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        if ((((((((({ let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1404)).wrapping_add(656); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1384); (c.m.r32(a_) as i32) }) < ({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1380); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
        } else {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
        }
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < ({ let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1404)).wrapping_add(652); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_r_c__str15, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                    { let a5_ = { let a_ = (((c.m.r32(sym::gSimResult)).wrapping_add(1404)).wrapping_add(16)).wrapping_add((((l_i_2) as u32)).wrapping_mul(12)); (c.m.r32(a_) as i32) }; F::Num_DrawChild(c, l_flash_1, fp.wrapping_add(28), sym::menu_r_c__str16, 0i32, 5i32, a5_, 32i32, 32i32, 0i32, 0i32) };
                    { let a5_ = ({ let a_ = ((((c.m.r32(sym::gSimResult)).wrapping_add(1404)).wrapping_add(16)).wrapping_add((((l_i_2) as u32)).wrapping_mul(12))).wrapping_add(8); (c.m.r32(a_) as i32) }).wrapping_mul(100i32); F::Num_DrawChild(c, l_flash_1, fp.wrapping_add(28), sym::menu_r_c__str17, 0i32, 6i32, a5_, 32i32, 32i32, 0i32, 0i32) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        {
            l_i_2 = 0i32;
            'l3: loop {
                if !(((((l_i_2) < (3i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_r_c__str18, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(28), fp);
                    if (((((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1384); (c.m.r32(a_) as i32) }).wrapping_mul(3i32)).wrapping_add(l_i_2)).wrapping_sub(3i32)) >= ({ let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1404)).wrapping_add(656); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    } else {
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                        F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_r_c__str19, fp);
                        { let a4_ = ({ let a_ = (((c.m.r32(sym::gSimResult)).wrapping_add(1404)).wrapping_add(64)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gSimResult)).wrapping_add(1404)).wrapping_add(640)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(12)); (c.m.r32(a_) as i32) }).wrapping_add(32i32); let a5_ = ((c.m.r32(sym::gSimResult)).wrapping_add(24)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, a4_, a5_) };
                        { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_r_c__str20, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                        { let a5_ = ({ let a_ = ((((c.m.r32(sym::gSimResult)).wrapping_add(1404)).wrapping_add(64)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gSimResult)).wrapping_add(1404)).wrapping_add(640)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(12))).wrapping_add(8); (c.m.r32(a_) as i32) }).wrapping_mul(100i32); F::Num_DrawChild(c, l_flash_1, fp.wrapping_add(28), sym::menu_r_c__str21, 0i32, 5i32, a5_, 32i32, 32i32, 0i32, 0i32) };
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        {
            l_i_2 = 1i32;
            'l5: loop {
                if !(((((l_i_2) < (4i32)) as i32)) != 0) { break 'l5; }
                'c6: {
                    { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (l_i_2).wrapping_shl((5i32) as u32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((l_i_2).wrapping_shl((5i32) as u32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_r_c__str22, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_r_c__str23, fp);
                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_r_c__str24, fp);
                    { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let a4_ = (({ let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1404)).wrapping_add(664); (c.m.r16(a_) as i16) }) as i32); F::Num_Draw(c, l_flash_1, sym::menu_r_c__str25, 4i32, 2i32, a4_, 32i32, 32i32, 1i32) };
        { let a4_ = (({ let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1404)).wrapping_add(666); (c.m.r16(a_) as i16) }) as i32); F::Num_Draw(c, l_flash_1, sym::menu_r_c__str26, 2i32, 2i32, a4_, 32i32, 32i32, 1i32) };
        { let a4_ = (({ let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1404)).wrapping_add(668); (c.m.r16(a_) as i16) }) as i32); F::Num_Draw(c, l_flash_1, sym::menu_r_c__str27, 0i32, 2i32, a4_, 32i32, 32i32, 1i32) };
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 160i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_r_c__str28, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 192i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_r_c__str29, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_r_c__str30, &[((2i32) as i64 as u64)]); };
        F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_r_c__str31, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_r_c__str32, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_r_c__str33, fp);
        { let a5_ = ((c.m.r32(sym::gSimResult)).wrapping_add(24)).wrapping_add((((5i32) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, 15i32, a5_) };
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_r_c__str34, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 192i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_r_c__str35, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_r_c__str36, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        { let a4_ = { let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1404)).wrapping_add(8); (c.m.r32(a_) as i32) }; F::Num_Draw(c, l_flash_1, sym::menu_r_c__str37, 0i32, 6i32, a4_, 32i32, 32i32, 0i32) };
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 96i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_r_c__str38, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        { let a4_ = { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); (c.m.r32(a_) as i32) }; F::Num_Draw(c, l_flash_1, sym::menu_r_c__str39, 0i32, 9i32, a4_, 32i32, 32i32, 0i32) };
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_r_c__str40, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        { let a4_ = ({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1404); (c.m.r32(a_) as i32) }).wrapping_mul(100i32); F::Num_Draw(c, l_flash_1, sym::menu_r_c__str41, 0i32, 9i32, a4_, 32i32, 32i32, 0i32) };
        l_x_3 = ({ let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1404)).wrapping_add(676); (c.m.r32(a_) as i32) }).wrapping_shl((6i32) as u32);
        { let v_ = l_x_3; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = (l_x_3).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_r_c__str42, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        if (({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1396); (c.m.r32(a_) as i32) }) != 0) {
            l_line_4 = F::UbScore_GetRewardItem(c, 0i32);
            F::Flash_FindLabel(c, l_flash_1, sym::menu_r_c__str43, sym::menu_r_c__str44, fp);
            { let a5_ = (c.m.r32(sym::gSimResult)).wrapping_add(864); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, l_line_4, a5_) };
        }
        {
            l_i_2 = 0i32;
            'l7: loop {
                if !(((((l_i_2) < (1i32)) as i32)) != 0) { break 'l7; }
                'c8: {
                    { let a0_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1008)).wrapping_add((((l_i_2) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        F::IconWin_Draw(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SimResult_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn SimResult_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimResult_Update", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_flash_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
            break 'f;
        }
        l_flash_1 = ((c.m.r32(sym::gSimResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        if ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1336); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            { let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1336); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
        }
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (1i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a0_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1008)).wrapping_add((((l_i_2) as u32)).wrapping_mul(44)); F::Flash_Advance(c, a0_) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        if (((({ let a_ = (((c.m.r32(sym::gSimResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1388); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) {
                { let r_ = 1i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1252); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = 2i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1252); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 15i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1336); c.m.w32(a_, (v_) as u32); };
                { let v_ = 9i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1348); c.m.w32(a_, (v_) as u32); };
            } else {
                if ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1348); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::Flash_GotoLabel(c, l_flash_1, sym::menu_r_c__str45, ((((1i32) as i64 as u64)) as u8));
                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1348); c.m.w32(a_, (v_) as u32); };
                }
            }
        }
        if (((({ let a_ = (((c.m.r32(sym::gSimResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((4i32) as u32)))) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1348); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) {
                { let v_ = 1i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(2084); c.m.w32(a_, (v_) as u32); };
            }
            { let a0_ = (c.m.r32(sym::gSimResult)).wrapping_add(1404); let a1_ = { let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1384); (c.m.r32(a_) as i32) }; F::UbScore_SetPage(c, a0_, a1_) };
        }
        if (((({ let a_ = (((c.m.r32(sym::gSimResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((8i32) as u32)))) != 0) {
            's3: {
                let sv3: i32 = { let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1348); (c.m.r32(a_) as i32) };
                match sv3 {
                    0i32 => {
                        if ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1388); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                            F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_r_c__str46, fp);
                            F::Flash_ClipGotoLabel(c, l_flash_1, fp, sym::menu_r_c__str47);
                        } else {
                            F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_r_c__str48, fp);
                            F::Flash_ClipGotoLabel(c, l_flash_1, fp, sym::menu_r_c__str49);
                        }
                        break 's3;
                    }
                    4i32 => {
                        F::Flash_GotoLabel(c, l_flash_1, sym::menu_r_c__str50, ((((1i32) as i64 as u64)) as u8));
                        F::SimResult_YesNoGoto(c, sym::menu_r_c__str51);
                        break 's3;
                    }
                    3i32 => {
                        F::Flash_GotoLabel(c, l_flash_1, sym::menu_r_c__str52, ((((1i32) as i64 as u64)) as u8));
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1404); c.m.w32(a_, (v_) as u32); };
                        break 's3;
                    }
                    5i32 => {
                        F::Flash_GotoLabel(c, l_flash_1, sym::menu_r_c__str53, ((((1i32) as i64 as u64)) as u8));
                        break 's3;
                    }
                    7i32 => {
                        F::Flash_GotoLabel(c, l_flash_1, sym::menu_r_c__str54, ((((1i32) as i64 as u64)) as u8));
                        break 's3;
                    }
                    6i32 => {
                        F::Flash_GotoLabel(c, l_flash_1, sym::menu_r_c__str55, ((((1i32) as i64 as u64)) as u8));
                        break 's3;
                    }
                    8i32 => {
                        F::Flash_GotoLabel(c, l_flash_1, sym::menu_r_c__str56, ((((1i32) as i64 as u64)) as u8));
                        if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1116); (c.m.r32(a_) as i32) }) == (102i32)) as i32)) != 0) {
                            { let v_ = 21i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1352); c.m.w32(a_, (v_) as u32); };
                        } else {
                            { let v_ = 20i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1352); c.m.w32(a_, (v_) as u32); };
                        }
                        break 's3;
                    }
                    1i32 | 2i32 => {
                        break 's3;
                    }
                    _ => {
                    }
                }
            }
        }
        if (((((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1400); c.m.r8(a_) }) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1336); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0)) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1384); (c.m.r32(a_) as i32) }) < ({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1380); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1384); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::Flash_GotoLabel(c, l_flash_1, sym::menu_r_c__str57, ((((1i32) as i64 as u64)) as u8));
                } else {
                    F::Flash_GotoLabel(c, l_flash_1, sym::menu_r_c__str58, ((((1i32) as i64 as u64)) as u8));
                }
                { let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1384); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            } else {
                if ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1384); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::Flash_GotoLabel(c, l_flash_1, sym::menu_r_c__str59, ((((1i32) as i64 as u64)) as u8));
                } else {
                    F::Flash_GotoLabel(c, l_flash_1, sym::menu_r_c__str60, ((((1i32) as i64 as u64)) as u8));
                }
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 19i32); };
            }
            { let v_ = ((0i32) as u8); let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1400); c.m.w8(a_, v_); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SimResult_Update", &PROBE_N, &[], true); }
    r_
}

pub fn SimResult_UpdateTalk(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimResult_UpdateTalk", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1360); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        if ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1352); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            break 'f;
        }
        if ((((((((((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1364); (c.m.r32(a_) as i32) }) != ((1i32).wrapping_neg())) as i32)) != 0) && (((((F::Voice_GetStat(c, 0i32)) != (5i32)) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1352); (c.m.r32(a_) as i32) }) == ({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1356); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                { let v_ = 1i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1360); c.m.w32(a_, (v_) as u32); };
            }
            break 'f;
        }
    } else {
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1360); c.m.w32(a_, (v_) as u32); };
    }
    's1: {
        let sv1: i32 = { let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1352); (c.m.r32(a_) as i32) };
        match sv1 {
            1i32 => {
                { let v_ = 31i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1364); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = 32i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1364); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            3i32 => {
                { let v_ = 33i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1364); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            4i32 => {
                { let v_ = 34i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1364); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            5i32 => {
                { let v_ = 35i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1364); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            6i32 => {
                { let v_ = 36i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1364); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            7i32 => {
                { let v_ = 37i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1364); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            8i32 => {
                { let v_ = 38i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1364); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            9i32 => {
                { let v_ = 39i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1364); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            10i32 => {
                { let v_ = 40i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1364); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            11i32 => {
                { let v_ = 41i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1364); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            12i32 => {
                { let v_ = 42i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1364); c.m.w32(a_, (v_) as u32); };
                { let v_ = 14i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1352); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            13i32 => {
                { let v_ = 43i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1364); c.m.w32(a_, (v_) as u32); };
                { let v_ = 14i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1352); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            14i32 => {
                F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1352); c.m.w32(a_, (v_) as u32); };
                break 'f;
            }
            15i32 => {
                { let v_ = 44i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1364); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            16i32 => {
                { let v_ = 45i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1364); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            17i32 => {
                { let v_ = 46i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1364); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            18i32 => {
                { let v_ = 47i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1364); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            19i32 => {
                { let v_ = 48i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1364); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            20i32 => {
                { let v_ = 49i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1364); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            21i32 => {
                { let v_ = 50i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1364); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1352); (c.m.r32(a_) as i32) }) != (14i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1352); c.m.w32(a_, (v_) as u32); };
    }
    { let a0_ = { let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(16); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1364); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34661i32, a2_) };
    { let v_ = { let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1352); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1356); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SimResult_UpdateTalk", &PROBE_N, &[], true); }
    r_
}

pub fn SimResult_Input(c: &mut Ctx, mut p_result_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimResult_Input", &PROBE_N, &[(p_result_1) as u32], false); }
    let mut l_up_2: i32 = 0;
    let mut l_down_3: i32 = 0;
    let mut l_ok_4: i32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_idx_6: i32 = 0;
    let mut l_kind_7: i32 = 0;
    let mut l_rows_8: i32 = 0;
    let r_ = 'f: {
    l_up_2 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) as i32);
    l_down_3 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) as i32);
    l_ok_4 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) as i32);
    if ((((!(((({ let a_ = (((c.m.r32(sym::gSimResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f;
    }
    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
        break 'f;
    }
    if ((((!(((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1252); (c.m.r32(a_) as i32) }) & (4i32))) != 0)) as i32)) != 0) {
        { let r_ = 4i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1252); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    's1: {
        let sv9: i32 = { let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1348); (c.m.r32(a_) as i32) };
        match sv9 {
            1i32 => {
                if (({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(2084); (c.m.r32(a_) as i32) }) != 0) {
                    l_idx_6 = ((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1384); (c.m.r32(a_) as i32) }) != (0i32)) as i32);
                    if (({ let a0_ = (c.m.r32(sym::gSimResult)).wrapping_add(1404); let a2_ = ({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(2088); (c.m.r32(a_) as i32) }).wrapping_sub((l_idx_6).wrapping_mul({ let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1404)).wrapping_add(652); (c.m.r32(a_) as i32) })); F::UbScore_CountLine(c, a0_, l_idx_6, a2_, 15i32) }) != 0) {
                        { let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(2088); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        if ((((((((((((((l_idx_6) != 0) && (({ l_rows_8 = 3i32; (((rt::rem_i32({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(2088); (c.m.r32(a_) as i32) }, l_rows_8)) == (0i32)) as i32) }) != 0)) as i32)) != 0) || ((((((((!((l_idx_6) != 0)) as i32)) != 0) && (((((rt::rem_i32({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(2088); (c.m.r32(a_) as i32) }, { let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1404)).wrapping_add(652); (c.m.r32(a_) as i32) })) == (0i32)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) || ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(2088); (c.m.r32(a_) as i32) }) == (({ let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1404)).wrapping_add(656); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1404)).wrapping_add(652); (c.m.r32(a_) as i32) }))) as i32)) != 0)) as i32)) != 0) {
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(2084); c.m.w32(a_, (v_) as u32); };
                            { let v_ = ((1i32) as u8); let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1400); c.m.w8(a_, v_); };
                        }
                    }
                }
                if ((l_ok_4) != 0) {
                    {
                        l_i_5 = 0i32;
                        'l2: loop {
                            if !(((((l_i_5) < (({ let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1404)).wrapping_add(656); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1404)).wrapping_add(652); (c.m.r32(a_) as i32) }))) as i32)) != 0) { break 'l2; }
                            'c3: {
                                l_idx_6 = 0i32;
                                l_kind_7 = 0i32;
                                if (((((l_i_5) < ({ let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1404)).wrapping_add(652); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                    l_kind_7 = 0i32;
                                    l_idx_6 = l_i_5;
                                } else {
                                    l_kind_7 = 1i32;
                                    l_idx_6 = (l_i_5).wrapping_sub({ let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1404)).wrapping_add(652); (c.m.r32(a_) as i32) });
                                }
                                'l4: while ((((!(({ let a0_ = (c.m.r32(sym::gSimResult)).wrapping_add(1404); F::UbScore_CountLine(c, a0_, l_kind_7, l_idx_6, 9999i32) }) != 0)) as i32)) != 0) {
                                }
                            }
                            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
                        }
                    }
                    { let v_ = ((1i32) as u8); let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1400); c.m.w8(a_, v_); };
                    { let v_ = { let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1380); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1384); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ({ let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1404)).wrapping_add(656); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1404)).wrapping_add(652); (c.m.r32(a_) as i32) }); let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(2088); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1380); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1384); c.m.w32(a_, (v_) as u32); };
                    { let a0_ = (c.m.r32(sym::gSimResult)).wrapping_add(1404); let a1_ = { let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1384); (c.m.r32(a_) as i32) }; F::UbScore_SetPage(c, a0_, a1_) };
                }
                if (({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1400); c.m.r8(a_) }) != 0) {
                    if ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1384); (c.m.r32(a_) as i32) }) < ({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1380); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        if ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1384); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                            { let a0_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_r_c__str61, ((((1i32) as i64 as u64)) as u8)) };
                        } else {
                            { let a0_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_r_c__str62, ((((1i32) as i64 as u64)) as u8)) };
                        }
                        { let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1384); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    } else {
                        if ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1384); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                            { let a0_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_r_c__str63, ((((1i32) as i64 as u64)) as u8)) };
                        } else {
                            { let a0_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_r_c__str64, ((((1i32) as i64 as u64)) as u8)) };
                        }
                        if ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1388); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                            if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1116); (c.m.r32(a_) as i32) }) == (102i32)) as i32)) != 0) {
                                { let v_ = 4i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1352); c.m.w32(a_, (v_) as u32); };
                            } else {
                                if ((((({ let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1404)).wrapping_add(676); (c.m.r32(a_) as i32) }) == (4i32)) as i32)) != 0) {
                                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1352); c.m.w32(a_, (v_) as u32); };
                                } else {
                                    { let v_ = (rt::rem_i32(F::Rand_Libc(c), 2i32)).wrapping_add(2i32); let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1352); c.m.w32(a_, (v_) as u32); };
                                }
                            }
                        } else {
                            if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1116); (c.m.r32(a_) as i32) }) == (102i32)) as i32)) != 0) {
                                { let v_ = 7i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1352); c.m.w32(a_, (v_) as u32); };
                            } else {
                                { let v_ = (rt::rem_i32(F::Rand_Libc(c), 2i32)).wrapping_add(5i32); let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1352); c.m.w32(a_, (v_) as u32); };
                            }
                        }
                    }
                    { let v_ = ((0i32) as u8); let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1400); c.m.w8(a_, v_); };
                }
                if ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(2088); (c.m.r32(a_) as i32) }) == (({ let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1404)).wrapping_add(656); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1404)).wrapping_add(652); (c.m.r32(a_) as i32) }))) as i32)) != 0) {
                    { let v_ = 2i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1348); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            2i32 => {
                if ((l_up_2) != 0) {
                    if ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1384); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                        { let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1384); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1384); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                            { let a0_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_r_c__str65, ((((1i32) as i64 as u64)) as u8)) };
                        } else {
                            { let a0_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_r_c__str66, ((((1i32) as i64 as u64)) as u8)) };
                        }
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    }
                } else {
                    if ((l_down_3) != 0) {
                        if ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1384); (c.m.r32(a_) as i32) }) < ({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1380); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                            { let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1384); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            if ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1384); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                                { let a0_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_r_c__str67, ((((1i32) as i64 as u64)) as u8)) };
                            } else {
                                { let a0_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_r_c__str68, ((((1i32) as i64 as u64)) as u8)) };
                            }
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                        }
                    } else {
                        if ((l_ok_4) != 0) {
                            if ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1388); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                                if (({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1392); (c.m.r32(a_) as i32) }) != 0) {
                                    { let v_ = 6i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1348); c.m.w32(a_, (v_) as u32); };
                                    if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1116); (c.m.r32(a_) as i32) }) == (102i32)) as i32)) != 0) {
                                        { let v_ = 19i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1352); c.m.w32(a_, (v_) as u32); };
                                    } else {
                                        { let v_ = (((F::Rand_Range(c, ((2i32) as u32))).wrapping_add(((17i32) as u32))) as i32); let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1352); c.m.w32(a_, (v_) as u32); };
                                    }
                                } else {
                                    { let v_ = 4i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1348); c.m.w32(a_, (v_) as u32); };
                                    if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1116); (c.m.r32(a_) as i32) }) == (102i32)) as i32)) != 0) {
                                        { let v_ = 11i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1352); c.m.w32(a_, (v_) as u32); };
                                    } else {
                                        { let v_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_add(((8i32) as u32))) as i32); let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1352); c.m.w32(a_, (v_) as u32); };
                                    }
                                }
                            } else {
                                { let v_ = 3i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1348); c.m.w32(a_, (v_) as u32); };
                            }
                            if ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1384); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                                { let a0_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_r_c__str69, ((((1i32) as i64 as u64)) as u8)) };
                            } else {
                                { let a0_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_r_c__str70, ((((1i32) as i64 as u64)) as u8)) };
                            }
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                        }
                    }
                }
                break 's1;
            }
            3i32 => {
                if ((l_ok_4) != 0) {
                    { let a0_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_r_c__str71, ((((1i32) as i64 as u64)) as u8)) };
                    { let r_ = 1i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1252); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let r_ = 2i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1252); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 15i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1336); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                }
                break 's1;
            }
            4i32 => {
                if ((l_up_2) != 0) {
                    if ((((({ let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1256)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1348); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                        F::SimResult_YesNoGoto(c, sym::menu_r_c__str72);
                        { let v_ = 0i32; let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1256)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1348); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        F::SimResult_YesNoGoto(c, sym::menu_r_c__str73);
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    }
                } else {
                    if ((l_down_3) != 0) {
                        if ((((({ let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1256)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1348); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                            F::SimResult_YesNoGoto(c, sym::menu_r_c__str74);
                            { let v_ = 1i32; let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1256)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1348); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            F::SimResult_YesNoGoto(c, sym::menu_r_c__str75);
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                        }
                    } else {
                        if ((l_ok_4) != 0) {
                            if ((((({ let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1256)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1348); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                                { let v_ = 0i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
                                { let v_ = (((F::Rand_Range(c, ((2i32) as u32))).wrapping_add(((12i32) as u32))) as i32); let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1352); c.m.w32(a_, (v_) as u32); };
                                { let v_ = { let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1404); (c.m.r32(a_) as i32) }; let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            } else {
                                { let a0_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_r_c__str76, ((((1i32) as i64 as u64)) as u8)) };
                                { let v_ = 5i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1348); c.m.w32(a_, (v_) as u32); };
                                { let v_ = (((F::Rand_Range(c, ((2i32) as u32))).wrapping_add(((15i32) as u32))) as i32); let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1352); c.m.w32(a_, (v_) as u32); };
                            }
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                        }
                    }
                }
                break 's1;
            }
            5i32 => {
                if ((l_ok_4) != 0) {
                    { let a0_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_r_c__str77, ((((1i32) as i64 as u64)) as u8)) };
                    { let v_ = 7i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1348); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(2088); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 60i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1336); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                }
                break 's1;
            }
            6i32 => {
                if ((l_ok_4) != 0) {
                    { let a0_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_r_c__str78, ((((1i32) as i64 as u64)) as u8)) };
                    { let v_ = 7i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1348); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(2088); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 60i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1336); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                }
                break 's1;
            }
            7i32 => {
                's5: {
                    let sv10: i32 = { let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(2088); (c.m.r32(a_) as i32) };
                    match sv10 {
                        0i32 => {
                            if ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1336); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                                { let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(2088); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                { let a0_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::UbScore_PlateGoto(c, a0_, 0i32, 1i32) };
                                { let v_ = { let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1404); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1404)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                { let a0_ = { let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1404)).wrapping_add(4); (c.m.r32(a_) as i32) }; let a1_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1116); (c.m.r32(a_) as i32) }; let a2_ = { let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1392); (c.m.r32(a_) as i32) }; F::UbScore_AddRanking(c, a0_, a1_, a2_) };
                            }
                            break 's5;
                        }
                        1i32 => {
                            if (({ let a0_ = (c.m.r32(sym::gSimResult)).wrapping_add(1404); F::UbScore_ConvertStep(c, a0_, 63i32) }) != 0) {
                                { let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(2088); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                { let v_ = 15i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1336); c.m.w32(a_, (v_) as u32); };
                                { let a0_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::UbScore_PlateGoto(c, a0_, 0i32, 0i32) };
                                { let v_ = { let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1404)).wrapping_add(4); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1404)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            }
                            break 's5;
                        }
                        2i32 => {
                            if ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1336); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                                { let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(2088); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                { let a0_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::UbScore_PlateGoto(c, a0_, 1i32, 1i32) };
                            }
                            break 's5;
                        }
                        3i32 => {
                            if (({ let a0_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1404)).wrapping_add(8); let a1_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); F::UbScore_Transfer(c, a0_, a1_, 63i32, F32(0x3f800000)) }) != 0) {
                                { let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(2088); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                { let a0_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::UbScore_PlateGoto(c, a0_, 1i32, 0i32) };
                            }
                            if ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); (c.m.r32(a_) as i32) }) > (9999999i32)) as i32)) != 0) {
                                { let v_ = 9999999i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); c.m.w32(a_, (v_) as u32); };
                            }
                            break 's5;
                        }
                        4i32 => {
                            if ((l_ok_4) != 0) {
                                { let a0_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_r_c__str79, ((((1i32) as i64 as u64)) as u8)) };
                                if ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1396); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                                    { let r_ = 1i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1252); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                    { let r_ = 2i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1252); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                    { let v_ = 15i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1336); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 9i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1348); c.m.w32(a_, (v_) as u32); };
                                } else {
                                    { let v_ = 8i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1348); c.m.w32(a_, (v_) as u32); };
                                    { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 6i32); };
                                }
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                            }
                            break 's5;
                        }
                        _ => {
                        }
                    }
                }
                if ((((((((((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(2088); (c.m.r32(a_) as i32) }) >= (1i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(2088); (c.m.r32(a_) as i32) }) <= (3i32)) as i32)) != 0)) as i32)) != 0) && ((l_ok_4) != 0)) as i32)) != 0) {
                    if ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(2088); (c.m.r32(a_) as i32) }) < (2i32)) as i32)) != 0) {
                        'l6: while ((((!(({ let a0_ = (c.m.r32(sym::gSimResult)).wrapping_add(1404); F::UbScore_ConvertStep(c, a0_, 9999999i32) }) != 0)) as i32)) != 0) {
                        }
                        { let v_ = { let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1404)).wrapping_add(4); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1404)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        'l7: while ((((!(({ let a0_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1404)).wrapping_add(8); let a1_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); F::UbScore_Transfer(c, a0_, a1_, 9999999i32, F32(0x3f800000)) }) != 0)) as i32)) != 0) {
                        }
                    } else {
                        if ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(2088); (c.m.r32(a_) as i32) }) < (4i32)) as i32)) != 0) {
                            'l8: while ((((!(({ let a0_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1404)).wrapping_add(8); let a1_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); F::UbScore_Transfer(c, a0_, a1_, 9999999i32, F32(0x3f800000)) }) != 0)) as i32)) != 0) {
                            }
                        }
                    }
                    if ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); (c.m.r32(a_) as i32) }) > (9999999i32)) as i32)) != 0) {
                        { let v_ = 9999999i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); c.m.w32(a_, (v_) as u32); };
                    }
                    { let a0_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::UbScore_PlateGoto(c, a0_, 0i32, 0i32) };
                    { let a0_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::UbScore_PlateGoto(c, a0_, 1i32, 0i32) };
                    { let v_ = 4i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(2088); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            8i32 => {
                if ((l_ok_4) != 0) {
                    { let a0_ = ((c.m.r32(sym::gSimResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_r_c__str80, ((((1i32) as i64 as u64)) as u8)) };
                    { let r_ = 1i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1252); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let r_ = 2i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1252); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 15i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1336); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 9i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1348); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                }
                break 's1;
            }
            9i32 => {
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SimResult_Input", &PROBE_N, &[], true); }
    r_
}

pub fn SimResult_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimResult_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = 1i32; c.m.w32(fp, (v_) as u32); }
        F::SimResult_Init(c, p_section_1);
        F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
        'l1: while ((1i32) != 0) {
            F::Gfx_BeginFrame(c);
            F::Pad_Update(c);
            F::Snd_Update(c);
            F::ColorFade_Update(c);
            F::SimResult_Update(c);
            F::SimResult_UpdateTalk(c);
            F::SimResult_Draw(c);
            F::ColorFade_Draw(c);
            F::Gfx_EndFrame(c, 1i32);
            F::Dma_Flush(c);
            F::File_Stub264D90(c);
            if ((F::ColorFade_IsInDone(c)) != 0) {
                if ((((((((!(((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1252); (c.m.r32(a_) as i32) }) & (8i32))) != 0)) as i32)) != 0) && (((({ let a_ = (((c.m.r32(sym::gSimResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
                    { let r_ = 8i32; let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1252); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
            if ((F::ColorFade_IsFadingOut(c)) != 0) {
                F::Bgm_FadeOutStep(c);
                F::Voice_FadeOutStep(c, 0i32);
                continue 'l1;
            }
            if ((F::ColorFade_IsOutDone(c)) != 0) {
                break 'l1;
            }
            if (((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1252); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1336); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                }
            } else {
                if ((((({ let a_ = (c.m.r32(sym::gSimResult)).wrapping_add(1352); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::SimResult_Input(c, fp);
                }
            }
        }
        F::SimResult_Term(c);
        F::Dma_ResetBuffers(c);
        break 'f (c.m.r32(fp) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SimResult_Run", &PROBE_N, &[], true); }
    r_
}
