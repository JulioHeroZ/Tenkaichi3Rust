#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn SurvResult_Init(c: &mut Ctx, mut p_section_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SurvResult_Init", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = F::Heap_Alloc(c, ((2040u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gSurvResult; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gSurvResult); F::memset(c, a0_, 0i32, 2040u32) }; };
        { let v_ = (c.m.r32(sym::gMenuArc3)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc3)).wrapping_add((((p_section_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = c.m.r32(sym::gSurvResult); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gSurvResult); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = c.m.r32(fp); let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1052); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((37i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((38i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((39i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((7i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((8i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((12i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((36i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((13i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((33i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((14i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((34i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((15i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((31i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((16i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((30i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((17i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((32i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((18i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((19i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((20i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((21i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((22i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((23i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((25i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((26i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((27i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((28i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((29i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((30i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((31i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((32i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((33i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((34i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((35i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((36i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((37i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((38i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((39i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((43i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((45i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((40i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((46i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((41i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((47i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((42i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1056)).wrapping_add((((35i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1056); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let v_ = rt::rem_i32(F::Rand_Libc(c), 32i32); let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1316); c.m.w32(a_, (v_) as u32); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = c.m.r32(fp); F::IconWin_Init(c, a0_, a1_) };
        F::IconWin_Open(c);
        F::IconWin_SetIcon(c, 2i32);
        { let v_ = ({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(16); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(20); c.m.w32(a_, v_); };
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (6i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a0_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(24)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); let a1_ = { let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(12); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, ((0i32) as u32)) };
                    if (((((l_i_2) == (5i32)) as i32)) != 0) {
                        { let a0_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(24)).wrapping_add((((5i32) as u32)).wrapping_mul(140)); F::TextBox_SetUnk50(c, a0_, 1i32) };
                    } else {
                        { let a0_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(24)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); F::TextBox_SetUnk50(c, a0_, 0i32) };
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let a0_ = (c.m.r32(sym::gSurvResult)).wrapping_add(864); let a1_ = { let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(20); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, ((0i32) as u32)) };
        { let a0_ = (c.m.r32(sym::gSurvResult)).wrapping_add(864); F::TextBox_SetUnk50(c, a0_, 0i32) };
        { let a0_ = (c.m.r32(sym::gSurvResult)).wrapping_add(864); F::TextBox_SetLineOffsets(c, a0_, 12i32, 0i32, 0i32, 0i32, 0i32) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1004); c.m.w32(a_, v_); };
        { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1308); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1328); c.m.w32(a_, (v_) as u32); };
        { let v_ = 15i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1280); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((0i32) as u8); let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1344); c.m.w8(a_, v_); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(2028); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(2032); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1340); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a1_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1348); let a2_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1324); F::UbScore_Fill(c, 2i32, a1_, a2_) }; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1332); c.m.w32(a_, (v_) as u32); };
        { let a0_ = { let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1004); c.m.r32(a_) }; let a1_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1348); F::UbScore_CalcPoints(c, a0_, a1_) };
        { let v_ = { let a0_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1600); (c.m.r32(a_) as i32) }; let a1_ = { let a1_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1348); F::UbScore_CalcRank(c, 2i32, a1_) }; let a2_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1348); F::UbScore_SaveBestB(c, a0_, a1_, a2_) }; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1336); c.m.w32(a_, (v_) as u32); };
        if ((((((((({ let a_ = (((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(12)); (c.m.r32(a_) as i32) }) >= (10i32)) as i32)) != 0) && ((((!(((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(520); (c.m.r32(a_) as i32) }) & (8i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
            { let r_ = 8i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(520); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            { let v_ = 1i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1340); c.m.w32(a_, (v_) as u32); };
            { let a0_ = F::UbScore_GetRewardItem(c, 3i32); F::Save_AddItem(c, a0_) };
        }
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1304); c.m.w32(a_, (v_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SurvResult_Init", &PROBE_N, &[], true); }
    r_
}

pub fn SurvResult_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SurvResult_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    F::IconWin_Term(c);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1008)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Destroy(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gSurvResult)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gSurvResult); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gSurvResult; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SurvResult_Term", &PROBE_N, &[], true); }
    r_
}

pub fn SurvResult_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SurvResult_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_flash_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_item_3: i32 = 0;
    let r_ = 'f: {
        { let a0_ = { let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1052); c.m.r32(a_) }; F::Sprite_DrawPicture(c, a0_, 0i32, 0i32, 128i32) };
        l_flash_1 = ((c.m.r32(sym::gSurvResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        F::Flash_FindLabel(c, l_flash_1, sym::menu_s_c__str0, sym::menu_s_c__str1, fp);
        { let a2_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1316); F::FlashAnim_Blink(c, l_flash_1, fp, a2_, 0i32) };
        F::Flash_FindLabel(c, l_flash_1, sym::menu_s_c__str2, sym::menu_s_c__str3, fp);
        { let a2_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1320); F::FlashAnim_Talk(c, l_flash_1, fp, a2_, 0i32) };
        if ((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1332); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            F::Flash_FindLabel(c, l_flash_1, sym::menu_s_c__str4, sym::menu_s_c__str5, fp);
            { let a5_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(24)).wrapping_add((((3i32) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, 321i32, a5_) };
        } else {
            F::Flash_FindLabel(c, l_flash_1, sym::menu_s_c__str6, sym::menu_s_c__str7, fp);
            { let a5_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(24)).wrapping_add((((3i32) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, 322i32, a5_) };
        }
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_s_c__str8, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_s_c__str9, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_s_c__str10, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        if ((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1328); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
        } else {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
        }
        { let v_ = 32i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_s_c__str11, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        if ((((((((({ let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(656); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1328); (c.m.r32(a_) as i32) }) < ({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1324); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
        } else {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
        }
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < ({ let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(652); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_s_c__str12, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                    { let a5_ = { let a_ = (((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(16)).wrapping_add((((l_i_2) as u32)).wrapping_mul(12)); (c.m.r32(a_) as i32) }; F::Num_DrawChild(c, l_flash_1, fp.wrapping_add(28), sym::menu_s_c__str13, 0i32, 5i32, a5_, 32i32, 32i32, 0i32, 0i32) };
                    { let a5_ = ({ let a_ = ((((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(16)).wrapping_add((((l_i_2) as u32)).wrapping_mul(12))).wrapping_add(8); (c.m.r32(a_) as i32) }).wrapping_mul(100i32); F::Num_DrawChild(c, l_flash_1, fp.wrapping_add(28), sym::menu_s_c__str14, 0i32, 6i32, a5_, 32i32, 32i32, 0i32, 0i32) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        {
            l_i_2 = 0i32;
            'l3: loop {
                if !(((((l_i_2) < (3i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_s_c__str15, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(28), fp);
                    if (((((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1328); (c.m.r32(a_) as i32) }).wrapping_mul(3i32)).wrapping_add(l_i_2)).wrapping_sub(3i32)) >= ({ let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(656); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    } else {
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                        F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_s_c__str16, fp);
                        { let a4_ = ({ let a_ = (((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(64)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(640)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(12)); (c.m.r32(a_) as i32) }).wrapping_add(32i32); let a5_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(24)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, a4_, a5_) };
                        { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_s_c__str17, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                        { let a5_ = ({ let a_ = ((((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(64)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(640)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(12))).wrapping_add(8); (c.m.r32(a_) as i32) }).wrapping_mul(100i32); F::Num_DrawChild(c, l_flash_1, fp.wrapping_add(28), sym::menu_s_c__str18, 0i32, 5i32, a5_, 32i32, 32i32, 0i32, 0i32) };
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
                    { let v_ = (l_i_2).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_s_c__str19, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_s_c__str20, fp);
                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_s_c__str21, fp);
                    { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let a4_ = (({ let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(664); (c.m.r16(a_) as i16) }) as i32); F::Num_Draw(c, l_flash_1, sym::menu_s_c__str22, 4i32, 2i32, a4_, 32i32, 32i32, 1i32) };
        { let a4_ = (({ let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(666); (c.m.r16(a_) as i16) }) as i32); F::Num_Draw(c, l_flash_1, sym::menu_s_c__str23, 2i32, 2i32, a4_, 32i32, 32i32, 1i32) };
        { let a4_ = (({ let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(668); (c.m.r16(a_) as i16) }) as i32); F::Num_Draw(c, l_flash_1, sym::menu_s_c__str24, 0i32, 2i32, a4_, 32i32, 32i32, 1i32) };
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 160i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_s_c__str25, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 192i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_s_c__str26, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        { let a4_ = { let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(8); (c.m.r32(a_) as i32) }; F::Num_Draw(c, l_flash_1, sym::menu_s_c__str27, 0i32, 6i32, a4_, 32i32, 32i32, 0i32) };
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 96i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_s_c__str28, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        { let a4_ = { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); (c.m.r32(a_) as i32) }; F::Num_Draw(c, l_flash_1, sym::menu_s_c__str29, 0i32, 9i32, a4_, 32i32, 32i32, 0i32) };
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_s_c__str30, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        { let a4_ = ({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1348); (c.m.r32(a_) as i32) }).wrapping_mul(100i32); F::Num_Draw(c, l_flash_1, sym::menu_s_c__str31, 0i32, 9i32, a4_, 32i32, 32i32, 0i32) };
        { let v_ = ({ let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(676); (c.m.r32(a_) as i32) }).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_s_c__str32, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_s_c__str33, fp);
        if (({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1336); (c.m.r32(a_) as i32) }) != 0) {
            { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
            F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
        } else {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
        }
        if (({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1340); (c.m.r32(a_) as i32) }) != 0) {
            l_item_3 = F::UbScore_GetRewardItem(c, 3i32);
            F::Flash_FindLabel(c, l_flash_1, sym::menu_s_c__str34, sym::menu_s_c__str35, fp);
            { let a5_ = (c.m.r32(sym::gSurvResult)).wrapping_add(864); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, l_item_3, a5_) };
        }
        {
            l_i_2 = 0i32;
            'l7: loop {
                if !(((((l_i_2) < (1i32)) as i32)) != 0) { break 'l7; }
                'c8: {
                    { let a0_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1008)).wrapping_add((((l_i_2) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        F::IconWin_Draw(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SurvResult_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn SurvResult_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SurvResult_Update", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_flash_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        l_flash_1 = ((c.m.r32(sym::gSurvResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
            break 'f;
        }
        if ((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1280); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            { let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1280); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
        }
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (1i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a0_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1008)).wrapping_add((((l_i_2) as u32)).wrapping_mul(44)); F::Flash_Advance(c, a0_) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        if (((({ let a_ = (((c.m.r32(sym::gSurvResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1292); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                { let v_ = 1i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(2036); c.m.w32(a_, (v_) as u32); };
            }
        }
        if (((({ let a_ = (((c.m.r32(sym::gSurvResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((4i32) as u32)))) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1292); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) {
                { let v_ = 1i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(2028); c.m.w32(a_, (v_) as u32); };
            }
            { let a0_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1348); let a1_ = { let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1328); (c.m.r32(a_) as i32) }; F::UbScore_SetPage(c, a0_, a1_) };
        }
        if (((({ let a_ = (((c.m.r32(sym::gSurvResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((8i32) as u32)))) != 0) {
            's3: {
                let sv3: i32 = { let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1292); (c.m.r32(a_) as i32) };
                match sv3 {
                    0i32 => {
                        if ((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1332); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                            F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_s_c__str36, fp);
                            F::Flash_ClipGotoLabel(c, l_flash_1, fp, sym::menu_s_c__str37);
                            { let v_ = 1i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1296); c.m.w32(a_, (v_) as u32); };
                        } else {
                            F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_s_c__str38, fp);
                            F::Flash_ClipGotoLabel(c, l_flash_1, fp, sym::menu_s_c__str39);
                            { let v_ = 6i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1296); c.m.w32(a_, (v_) as u32); };
                        }
                        break 's3;
                    }
                    3i32 => {
                        F::Flash_GotoLabel(c, l_flash_1, sym::menu_s_c__str40, ((((1i32) as i64 as u64)) as u8));
                        break 's3;
                    }
                    4i32 => {
                        F::Flash_GotoLabel(c, l_flash_1, sym::menu_s_c__str41, ((((1i32) as i64 as u64)) as u8));
                        break 's3;
                    }
                    _ => {
                    }
                }
            }
        }
        if (((((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1344); c.m.r8(a_) }) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1280); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0)) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1328); (c.m.r32(a_) as i32) }) < ({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1324); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1328); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::Flash_GotoLabel(c, l_flash_1, sym::menu_s_c__str42, ((((1i32) as i64 as u64)) as u8));
                } else {
                    F::Flash_GotoLabel(c, l_flash_1, sym::menu_s_c__str43, ((((1i32) as i64 as u64)) as u8));
                }
                { let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1328); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            } else {
                if ((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1328); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::Flash_GotoLabel(c, l_flash_1, sym::menu_s_c__str44, ((((1i32) as i64 as u64)) as u8));
                } else {
                    F::Flash_GotoLabel(c, l_flash_1, sym::menu_s_c__str45, ((((1i32) as i64 as u64)) as u8));
                }
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 19i32); };
                if ((((({ let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(676); (c.m.r32(a_) as i32) }) == (4i32)) as i32)) != 0) {
                    { let v_ = 11i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1296); c.m.w32(a_, (v_) as u32); };
                } else {
                    if ((F::Rand_Range(c, ((2i32) as u32))) != 0) {
                        { let v_ = 12i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1296); c.m.w32(a_, (v_) as u32); };
                    } else {
                        { let v_ = 13i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1296); c.m.w32(a_, (v_) as u32); };
                    }
                }
            }
            { let v_ = ((0i32) as u8); let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1344); c.m.w8(a_, v_); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SurvResult_Update", &PROBE_N, &[], true); }
    r_
}

pub fn SurvResult_UpdateVoice(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SurvResult_UpdateVoice", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1304); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        if ((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1296); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            break 'f;
        }
        if ((((((((((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1308); (c.m.r32(a_) as i32) }) != ((1i32).wrapping_neg())) as i32)) != 0) && (((((F::Voice_GetStat(c, 0i32)) != (5i32)) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1296); (c.m.r32(a_) as i32) }) == ({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1300); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                { let v_ = 1i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1304); c.m.w32(a_, (v_) as u32); };
            }
            break 'f;
        }
    } else {
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1304); c.m.w32(a_, (v_) as u32); };
    }
    's1: {
        let sv1: i32 = { let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1296); (c.m.r32(a_) as i32) };
        match sv1 {
            1i32 => {
                { let v_ = 56i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1308); c.m.w32(a_, (v_) as u32); };
                { let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1296); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1296); c.m.w32(a_, (v_) as u32); };
                { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1308); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            6i32 => {
                { let v_ = 57i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1308); c.m.w32(a_, (v_) as u32); };
                { let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1296); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            7i32 => {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1296); c.m.w32(a_, (v_) as u32); };
                { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1308); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            11i32 => {
                { let v_ = 58i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1308); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            12i32 => {
                { let v_ = 59i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1308); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            13i32 => {
                { let v_ = 60i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1308); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            14i32 => {
                { let v_ = 61i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1308); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            15i32 => {
                { let v_ = 63i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1308); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1296); (c.m.r32(a_) as i32) }) >= (11i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1296); c.m.w32(a_, (v_) as u32); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1308); (c.m.r32(a_) as i32) }) == ((1i32).wrapping_neg())) as i32)) != 0) {
        break 'f;
    }
    { let a0_ = { let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(16); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1308); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34661i32, a2_) };
    { let v_ = { let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1296); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1300); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SurvResult_UpdateVoice", &PROBE_N, &[], true); }
    r_
}

pub fn SurvResult_Input(c: &mut Ctx, mut p_result_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SurvResult_Input", &PROBE_N, &[(p_result_1) as u32], false); }
    let mut l_up_2: i32 = 0;
    let mut l_down_3: i32 = 0;
    let mut l_confirm_4: i32 = 0;
    let mut l_isBonus_5: i32 = 0;
    let mut l_idx_6: i32 = 0;
    let mut l_i_7: i32 = 0;
    let mut l_idx_8: i32 = 0;
    let mut l_isBonus_9: i32 = 0;
    let r_ = 'f: {
    l_up_2 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) as i32);
    l_down_3 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) as i32);
    l_confirm_4 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) as i32);
    if ((((((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1292); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) && (({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(2036); (c.m.r32(a_) as i32) }) != 0)) as i32)) != 0) {
        if ((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1332); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) {
            { let r_ = 1i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1228); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            { let r_ = 2i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1228); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            { let v_ = 15i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1280); c.m.w32(a_, (v_) as u32); };
            { let v_ = 5i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1292); c.m.w32(a_, (v_) as u32); };
        } else {
            { let a0_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_s_c__str46, ((((1i32) as i64 as u64)) as u8)) };
            { let v_ = 1i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1292); c.m.w32(a_, (v_) as u32); };
        }
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(2036); c.m.w32(a_, (v_) as u32); };
    }
    if ((((!(((({ let a_ = (((c.m.r32(sym::gSurvResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f;
    }
    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
        break 'f;
    }
    if ((((!(((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1228); (c.m.r32(a_) as i32) }) & (4i32))) != 0)) as i32)) != 0) {
        { let r_ = 4i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1228); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    's1: {
        let sv10: i32 = { let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1292); (c.m.r32(a_) as i32) };
        match sv10 {
            1i32 => {
                if (({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(2028); (c.m.r32(a_) as i32) }) != 0) {
                    l_isBonus_5 = ((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1328); (c.m.r32(a_) as i32) }) != (0i32)) as i32);
                    l_idx_6 = ({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(2032); (c.m.r32(a_) as i32) }).wrapping_sub((l_isBonus_5).wrapping_mul({ let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(652); (c.m.r32(a_) as i32) }));
                    if (({ let a0_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1348); F::UbScore_CountLine(c, a0_, l_isBonus_5, l_idx_6, 15i32) }) != 0) {
                        { let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(2032); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        if ((((((((((((((l_isBonus_5) != 0) && (((((rt::rem_i32((l_idx_6).wrapping_add(1i32), 3i32)) == (0i32)) as i32)) != 0)) as i32)) != 0) || ((((((((!((l_isBonus_5) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(2032); (c.m.r32(a_) as i32) }) == ({ let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(652); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) || ((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(2032); (c.m.r32(a_) as i32) }) == (({ let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(656); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(652); (c.m.r32(a_) as i32) }))) as i32)) != 0)) as i32)) != 0) {
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(2028); c.m.w32(a_, (v_) as u32); };
                            { let v_ = ((1i32) as u8); let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1344); c.m.w8(a_, v_); };
                        }
                    }
                }
                if ((l_confirm_4) != 0) {
                    {
                        l_i_7 = 0i32;
                        'l2: loop {
                            if !(((((l_i_7) < (({ let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(656); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(652); (c.m.r32(a_) as i32) }))) as i32)) != 0) { break 'l2; }
                            'c3: {
                                l_idx_8 = 0i32;
                                l_isBonus_9 = 0i32;
                                if (((((l_i_7) < ({ let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(652); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                    l_isBonus_9 = 0i32;
                                    l_idx_8 = l_i_7;
                                } else {
                                    l_idx_8 = (l_i_7).wrapping_sub({ let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(652); (c.m.r32(a_) as i32) });
                                    l_isBonus_9 = 1i32;
                                }
                                'l4: while ((((!(({ let a0_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1348); F::UbScore_CountLine(c, a0_, l_isBonus_9, l_idx_8, 9999i32) }) != 0)) as i32)) != 0) {
                                }
                            }
                            { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
                        }
                    }
                    { let v_ = ((1i32) as u8); let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1344); c.m.w8(a_, v_); };
                    { let v_ = { let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1324); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1328); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ({ let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(656); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(652); (c.m.r32(a_) as i32) }); let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(2032); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1324); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1328); c.m.w32(a_, (v_) as u32); };
                    { let a0_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1348); let a1_ = { let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1328); (c.m.r32(a_) as i32) }; F::UbScore_SetPage(c, a0_, a1_) };
                }
                if (({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1344); c.m.r8(a_) }) != 0) {
                    if ((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1328); (c.m.r32(a_) as i32) }) < ({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1324); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        if ((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1328); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                            { let a0_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_s_c__str47, ((((1i32) as i64 as u64)) as u8)) };
                        } else {
                            { let a0_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_s_c__str48, ((((1i32) as i64 as u64)) as u8)) };
                        }
                        { let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1328); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    } else {
                        if ((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1328); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                            { let a0_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_s_c__str49, ((((1i32) as i64 as u64)) as u8)) };
                        } else {
                            { let a0_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_s_c__str50, ((((1i32) as i64 as u64)) as u8)) };
                        }
                        if ((((({ let a_ = (((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(12)); (c.m.r32(a_) as i32) }) >= (50i32)) as i32)) != 0) {
                            { let v_ = 15i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1296); c.m.w32(a_, (v_) as u32); };
                        } else {
                            if ((((({ let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(676); (c.m.r32(a_) as i32) }) == (4i32)) as i32)) != 0) {
                                { let v_ = 11i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1296); c.m.w32(a_, (v_) as u32); };
                            } else {
                                if ((F::Rand_Range(c, ((2i32) as u32))) != 0) {
                                    { let v_ = 12i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1296); c.m.w32(a_, (v_) as u32); };
                                } else {
                                    { let v_ = 13i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1296); c.m.w32(a_, (v_) as u32); };
                                }
                            }
                        }
                    }
                    { let v_ = ((0i32) as u8); let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1344); c.m.w8(a_, v_); };
                }
                if ((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(2032); (c.m.r32(a_) as i32) }) == (({ let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(656); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(652); (c.m.r32(a_) as i32) }))) as i32)) != 0) {
                    { let v_ = 2i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1292); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            2i32 => {
                if ((l_up_2) != 0) {
                    if ((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1328); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                        { let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1328); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1328); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                            { let a0_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_s_c__str51, ((((1i32) as i64 as u64)) as u8)) };
                        } else {
                            { let a0_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_s_c__str52, ((((1i32) as i64 as u64)) as u8)) };
                        }
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    }
                } else {
                    if ((l_down_3) != 0) {
                        if ((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1328); (c.m.r32(a_) as i32) }) < ({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1324); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                            { let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1328); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            if ((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1328); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                                { let a0_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_s_c__str53, ((((1i32) as i64 as u64)) as u8)) };
                            } else {
                                { let a0_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_s_c__str54, ((((1i32) as i64 as u64)) as u8)) };
                            }
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                        }
                    } else {
                        if ((l_confirm_4) != 0) {
                            { let v_ = 3i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1292); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(2032); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 60i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1280); c.m.w32(a_, (v_) as u32); };
                            if ((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1328); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                                { let a0_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_s_c__str55, ((((1i32) as i64 as u64)) as u8)) };
                            } else {
                                { let a0_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_s_c__str56, ((((1i32) as i64 as u64)) as u8)) };
                            }
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                        }
                    }
                }
                break 's1;
            }
            3i32 => {
                's5: {
                    let sv11: i32 = { let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(2032); (c.m.r32(a_) as i32) };
                    match sv11 {
                        0i32 => {
                            if ((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1280); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                                { let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(2032); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                { let a0_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::UbScore_PlateGoto(c, a0_, 0i32, 1i32) };
                                { let v_ = { let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1348); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 14i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1296); c.m.w32(a_, (v_) as u32); };
                            }
                            break 's5;
                        }
                        1i32 => {
                            if (({ let a0_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1348); F::UbScore_ConvertStep(c, a0_, 64i32) }) != 0) {
                                { let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(2032); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                { let v_ = 15i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1280); c.m.w32(a_, (v_) as u32); };
                                { let a0_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::UbScore_PlateGoto(c, a0_, 0i32, 0i32) };
                                { let v_ = { let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(4); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            }
                            break 's5;
                        }
                        2i32 => {
                            if ((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1280); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                                { let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(2032); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                { let a0_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::UbScore_PlateGoto(c, a0_, 1i32, 1i32) };
                            }
                            break 's5;
                        }
                        3i32 => {
                            if (({ let a0_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(8); let a1_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); F::UbScore_Transfer(c, a0_, a1_, 64i32, F32(0x3f800000)) }) != 0) {
                                { let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(2032); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                { let a0_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::UbScore_PlateGoto(c, a0_, 1i32, 0i32) };
                            }
                            if ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); (c.m.r32(a_) as i32) }) > (9999999i32)) as i32)) != 0) {
                                { let v_ = 9999999i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); c.m.w32(a_, (v_) as u32); };
                            }
                            break 's5;
                        }
                        4i32 => {
                            if ((l_confirm_4) != 0) {
                                { let a0_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_s_c__str57, ((((1i32) as i64 as u64)) as u8)) };
                                if (({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1340); (c.m.r32(a_) as i32) }) != 0) {
                                    { let v_ = 4i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1292); c.m.w32(a_, (v_) as u32); };
                                    { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 6i32); };
                                } else {
                                    { let r_ = 1i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1228); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                    { let r_ = 2i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1228); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                    { let v_ = 15i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1280); c.m.w32(a_, (v_) as u32); };
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                    { let v_ = 5i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1292); c.m.w32(a_, (v_) as u32); };
                                }
                            }
                            break 's5;
                        }
                        _ => {
                        }
                    }
                }
                if ((((((((((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(2032); (c.m.r32(a_) as i32) }) >= (1i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(2032); (c.m.r32(a_) as i32) }) <= (3i32)) as i32)) != 0)) as i32)) != 0) && ((l_confirm_4) != 0)) as i32)) != 0) {
                    if ((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(2032); (c.m.r32(a_) as i32) }) < (2i32)) as i32)) != 0) {
                        'l6: while ((((!(({ let a0_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1348); F::UbScore_ConvertStep(c, a0_, 9999999i32) }) != 0)) as i32)) != 0) {
                        }
                        { let v_ = { let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(4); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        'l7: while ((((!(({ let a0_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(8); let a1_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); F::UbScore_Transfer(c, a0_, a1_, 9999999i32, F32(0x3f800000)) }) != 0)) as i32)) != 0) {
                        }
                    } else {
                        if ((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(2032); (c.m.r32(a_) as i32) }) < (4i32)) as i32)) != 0) {
                            'l8: while ((((!(({ let a0_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1348)).wrapping_add(8); let a1_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); F::UbScore_Transfer(c, a0_, a1_, 9999999i32, F32(0x3f800000)) }) != 0)) as i32)) != 0) {
                            }
                        }
                    }
                    if ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); (c.m.r32(a_) as i32) }) > (9999999i32)) as i32)) != 0) {
                        { let v_ = 9999999i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); c.m.w32(a_, (v_) as u32); };
                    }
                    { let a0_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::UbScore_PlateGoto(c, a0_, 0i32, 0i32) };
                    { let a0_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::UbScore_PlateGoto(c, a0_, 1i32, 0i32) };
                    { let v_ = 4i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(2032); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            4i32 => {
                if ((l_confirm_4) != 0) {
                    { let a0_ = ((c.m.r32(sym::gSurvResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_s_c__str58, ((((1i32) as i64 as u64)) as u8)) };
                    { let r_ = 1i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1228); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let r_ = 2i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1228); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 15i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1280); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 5i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1292); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                }
                break 's1;
            }
            5i32 => {
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SurvResult_Input", &PROBE_N, &[], true); }
    r_
}

pub fn SurvResult_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SurvResult_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = 1i32; c.m.w32(fp, (v_) as u32); }
        F::SurvResult_Init(c, p_section_1);
        F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
        'l1: while ((1i32) != 0) {
            F::Gfx_BeginFrame(c);
            F::Pad_Update(c);
            F::Snd_Update(c);
            F::ColorFade_Update(c);
            F::SurvResult_Update(c);
            F::SurvResult_UpdateVoice(c);
            F::SurvResult_Draw(c);
            F::ColorFade_Draw(c);
            F::Gfx_EndFrame(c, 1i32);
            F::Dma_Flush(c);
            F::File_Stub264D90(c);
            if ((F::ColorFade_IsInDone(c)) != 0) {
                if ((((((((!(((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1228); (c.m.r32(a_) as i32) }) & (8i32))) != 0)) as i32)) != 0) && (((({ let a_ = (((c.m.r32(sym::gSurvResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
                    { let r_ = 8i32; let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1228); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
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
            if (((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1228); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1280); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                }
            } else {
                if ((((({ let a_ = (c.m.r32(sym::gSurvResult)).wrapping_add(1296); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::SurvResult_Input(c, fp);
                }
            }
        }
        F::SurvResult_Term(c);
        F::Dma_ResetBuffers(c);
        break 'f (c.m.r32(fp) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SurvResult_Run", &PROBE_N, &[], true); }
    r_
}
