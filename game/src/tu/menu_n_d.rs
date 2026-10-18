#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn UbResult_MarkCourseCleared(c: &mut Ctx, mut p_course_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbResult_MarkCourseCleared", &PROBE_N, &[(p_course_1) as u32], false); }
    let mut l_count_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    l_count_2 = 0i32;
    { let v_ = ((1i32) as u8); let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(1920)).wrapping_add((((p_course_1) as u32)).wrapping_mul(12)); c.m.w8(a_, v_); };
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (5i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(1920)).wrapping_add((((l_i_3) as u32)).wrapping_mul(12)); c.m.r8(a_) }) != 0) {
                    { let o_ = l_count_2; let n_ = (o_).wrapping_add(1); l_count_2 = n_; };
                }
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    if (((((((((l_count_2) == (5i32)) as i32)) != 0) && ((((!(((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(520); (c.m.r32(a_) as i32) }) & (16i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
        { let r_ = 16i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(520); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "UbResult_MarkCourseCleared", &PROBE_N, &[], true); }
    r_
}

pub fn UbResult_Init(c: &mut Ctx, mut p_section_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbResult_Init", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_2: i32 = 0;
    let mut l_rank_3: i32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = F::Heap_Alloc(c, ((1924u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gUbResult; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gUbResult); F::memset(c, a0_, 0i32, 1924u32) }; };
        { let v_ = (c.m.r32(sym::gMenuArc3)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc3)).wrapping_add((((p_section_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = c.m.r32(sym::gUbResult); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gUbResult); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = c.m.r32(fp); let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(912); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((39i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((40i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((41i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((30i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((7i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((8i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((9i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((10i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((12i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((38i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((13i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((35i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((14i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((36i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((15i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((33i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((16i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((32i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((17i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((34i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((18i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((19i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((20i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((21i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((22i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((23i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((25i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((26i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((27i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((28i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((29i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((30i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((31i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((32i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((33i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((34i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((35i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((36i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((37i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((38i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((39i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((43i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((31i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((44i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((45i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((45i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((42i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((46i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((43i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((47i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((44i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((37i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(916)).wrapping_add((((46i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gUbResult)).wrapping_add(868)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gUbResult)).wrapping_add(916); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gUbResult)).wrapping_add(868)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let v_ = rt::rem_i32(F::Rand_Libc(c), 32i32); let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1192); c.m.w32(a_, (v_) as u32); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = c.m.r32(fp); F::IconWin_Init(c, a0_, a1_) };
        F::IconWin_Open(c);
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(16); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(20); c.m.w32(a_, v_); };
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a0_ = ((c.m.r32(sym::gUbResult)).wrapping_add(24)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); let a1_ = { let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(12); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, ((0i32) as u32)) };
                    { let a0_ = ((c.m.r32(sym::gUbResult)).wrapping_add(24)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); F::TextBox_SetUnk50(c, a0_, 0i32) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let a0_ = (c.m.r32(sym::gUbResult)).wrapping_add(584); let a1_ = { let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(20); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, ((0i32) as u32)) };
        { let a0_ = (c.m.r32(sym::gUbResult)).wrapping_add(584); F::TextBox_SetUnk50(c, a0_, 0i32) };
        { let a0_ = (c.m.r32(sym::gUbResult)).wrapping_add(584); F::TextBox_SetLineOffsets(c, a0_, 12i32, 0i32, 0i32, 0i32, 0i32) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(864); c.m.w32(a_, v_); };
        { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1184); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1204); c.m.w32(a_, (v_) as u32); };
        { let v_ = 15i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1156); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((0i32) as u8); let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1220); c.m.w8(a_, v_); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1912); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1916); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1216); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1180); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1600); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1224); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a1_ = (c.m.r32(sym::gUbResult)).wrapping_add(1232); let a2_ = (c.m.r32(sym::gUbResult)).wrapping_add(1200); F::UbScore_Fill(c, 3i32, a1_, a2_) }; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1208); c.m.w32(a_, (v_) as u32); };
        { let a0_ = { let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(864); c.m.r32(a_) }; let a1_ = (c.m.r32(sym::gUbResult)).wrapping_add(1232); F::UbScore_CalcPoints(c, a0_, a1_) };
        l_rank_3 = { let a1_ = (c.m.r32(sym::gUbResult)).wrapping_add(1232); F::UbScore_CalcRank(c, 3i32, a1_) };
        if ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1224); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) {
            { let v_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1672); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1228); c.m.w32(a_, (v_) as u32); };
            F::IconWin_SetIcon(c, 4i32);
            if ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1208); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                { let v_ = { let a0_ = { let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1228); (c.m.r32(a_) as i32) }; let a2_ = (c.m.r32(sym::gUbResult)).wrapping_add(1232); F::UbScore_SaveBestC(c, a0_, l_rank_3, a2_) }; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1212); c.m.w32(a_, (v_) as u32); };
                if (({ let a0_ = { let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1228); (c.m.r32(a_) as i32) }; F::UbResult_MarkCourseCleared(c, a0_) }) != 0) {
                    { let a0_ = F::UbScore_GetRewardItem(c, 5i32); F::Save_AddItem(c, a0_) };
                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1216); c.m.w32(a_, (v_) as u32); };
                }
            }
        } else {
            F::IconWin_SetIcon(c, 3i32);
            if ((((((((((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1208); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(1916); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0)) as i32)) != 0) && (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1668); (c.m.r32(a_) as i32) }) & (8i32))) != 0)) as i32)) != 0) {
                { let v_ = 1i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1216); c.m.w32(a_, (v_) as u32); };
                { let a0_ = F::UbScore_GetRewardItem(c, 4i32); F::Save_AddItem(c, a0_) };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "UbResult_Init", &PROBE_N, &[], true); }
    r_
}

pub fn UbResult_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbResult_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    F::IconWin_Term(c);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gUbResult)).wrapping_add(868)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Destroy(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gUbResult)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gUbResult); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gUbResult; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "UbResult_Term", &PROBE_N, &[], true); }
    r_
}

pub fn UbResult_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbResult_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_flash_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_item_3: i32 = 0;
    let r_ = 'f: {
        { let a0_ = { let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(912); c.m.r32(a_) }; F::Sprite_DrawPicture(c, a0_, 0i32, 0i32, 128i32) };
        l_flash_1 = ((c.m.r32(sym::gUbResult)).wrapping_add(868)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        F::Flash_FindLabel(c, l_flash_1, sym::menu_n_d__str0, sym::menu_n_d__str1, fp);
        { let a2_ = (c.m.r32(sym::gUbResult)).wrapping_add(1192); F::FlashAnim_Blink(c, l_flash_1, fp, a2_, 0i32) };
        F::Flash_FindLabel(c, l_flash_1, sym::menu_n_d__str2, sym::menu_n_d__str3, fp);
        { let a2_ = (c.m.r32(sym::gUbResult)).wrapping_add(1196); F::FlashAnim_Talk(c, l_flash_1, fp, a2_, 0i32) };
        if ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1208); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            F::Flash_FindLabel(c, l_flash_1, sym::menu_n_d__str4, sym::menu_n_d__str5, fp);
            if ((((((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1224); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) && (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1668); (c.m.r32(a_) as i32) }) & (8i32))) != 0)) as i32)) != 0) {
                { let a5_ = ((c.m.r32(sym::gUbResult)).wrapping_add(24)).wrapping_add((((3i32) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, 320i32, a5_) };
            } else {
                { let a5_ = ((c.m.r32(sym::gUbResult)).wrapping_add(24)).wrapping_add((((3i32) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, 321i32, a5_) };
            }
        } else {
            F::Flash_FindLabel(c, l_flash_1, sym::menu_n_d__str6, sym::menu_n_d__str7, fp);
            { let a5_ = ((c.m.r32(sym::gUbResult)).wrapping_add(24)).wrapping_add((((3i32) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, 322i32, a5_) };
        }
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_n_d__str8, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_n_d__str9, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_n_d__str10, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        if ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1204); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
        } else {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
        }
        { let v_ = 32i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_n_d__str11, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        if ((((((((({ let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(1232)).wrapping_add(656); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1204); (c.m.r32(a_) as i32) }) < ({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1200); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
        } else {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
        }
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < ({ let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(1232)).wrapping_add(652); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_n_d__str12, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                    { let a5_ = { let a_ = (((c.m.r32(sym::gUbResult)).wrapping_add(1232)).wrapping_add(16)).wrapping_add((((l_i_2) as u32)).wrapping_mul(12)); (c.m.r32(a_) as i32) }; F::Num_DrawChild(c, l_flash_1, fp.wrapping_add(28), sym::menu_n_d__str13, 0i32, 5i32, a5_, 32i32, 32i32, 0i32, 0i32) };
                    { let a5_ = ({ let a_ = ((((c.m.r32(sym::gUbResult)).wrapping_add(1232)).wrapping_add(16)).wrapping_add((((l_i_2) as u32)).wrapping_mul(12))).wrapping_add(8); (c.m.r32(a_) as i32) }).wrapping_mul(100i32); F::Num_DrawChild(c, l_flash_1, fp.wrapping_add(28), sym::menu_n_d__str14, 0i32, 6i32, a5_, 32i32, 32i32, 0i32, 0i32) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        {
            l_i_2 = 0i32;
            'l3: loop {
                if !(((((l_i_2) < (3i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_n_d__str15, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(28), fp);
                    if (((((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1204); (c.m.r32(a_) as i32) }).wrapping_mul(3i32)).wrapping_add(l_i_2)).wrapping_sub(3i32)) >= ({ let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(1232)).wrapping_add(656); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    } else {
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                        F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_n_d__str16, fp);
                        { let a4_ = ({ let a_ = (((c.m.r32(sym::gUbResult)).wrapping_add(1232)).wrapping_add(64)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gUbResult)).wrapping_add(1232)).wrapping_add(640)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(12)); (c.m.r32(a_) as i32) }).wrapping_add(32i32); let a5_ = ((c.m.r32(sym::gUbResult)).wrapping_add(24)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, a4_, a5_) };
                        { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_n_d__str17, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                        { let a5_ = ({ let a_ = ((((c.m.r32(sym::gUbResult)).wrapping_add(1232)).wrapping_add(64)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gUbResult)).wrapping_add(1232)).wrapping_add(640)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(12))).wrapping_add(8); (c.m.r32(a_) as i32) }).wrapping_mul(100i32); F::Num_DrawChild(c, l_flash_1, fp.wrapping_add(28), sym::menu_n_d__str18, 0i32, 5i32, a5_, 32i32, 32i32, 0i32, 0i32) };
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
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_n_d__str19, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_n_d__str20, fp);
                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_n_d__str21, fp);
                    { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let a4_ = (({ let a_ = (((c.m.r32(sym::gUbResult)).wrapping_add(1232)).wrapping_add(664)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); (c.m.r16(a_) as i16) }) as i32); F::Num_Draw(c, l_flash_1, sym::menu_n_d__str22, 4i32, 2i32, a4_, 32i32, 32i32, 1i32) };
        { let a4_ = (({ let a_ = (((c.m.r32(sym::gUbResult)).wrapping_add(1232)).wrapping_add(664)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); (c.m.r16(a_) as i16) }) as i32); F::Num_Draw(c, l_flash_1, sym::menu_n_d__str23, 2i32, 2i32, a4_, 32i32, 32i32, 1i32) };
        { let a4_ = (({ let a_ = (((c.m.r32(sym::gUbResult)).wrapping_add(1232)).wrapping_add(664)).wrapping_add((((2i32) as u32)).wrapping_mul(2)); (c.m.r16(a_) as i16) }) as i32); F::Num_Draw(c, l_flash_1, sym::menu_n_d__str24, 0i32, 2i32, a4_, 32i32, 32i32, 1i32) };
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 160i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_n_d__str25, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 192i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_n_d__str26, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        { let a4_ = { let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(1232)).wrapping_add(8); (c.m.r32(a_) as i32) }; F::Num_Draw(c, l_flash_1, sym::menu_n_d__str27, 0i32, 6i32, a4_, 32i32, 32i32, 0i32) };
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 96i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_n_d__str28, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        { let a4_ = { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); (c.m.r32(a_) as i32) }; F::Num_Draw(c, l_flash_1, sym::menu_n_d__str29, 0i32, 9i32, a4_, 32i32, 32i32, 0i32) };
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_n_d__str30, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        { let a4_ = ({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1232); (c.m.r32(a_) as i32) }).wrapping_mul(100i32); F::Num_Draw(c, l_flash_1, sym::menu_n_d__str31, 0i32, 9i32, a4_, 32i32, 32i32, 0i32) };
        { let v_ = ({ let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(1232)).wrapping_add(676); (c.m.r32(a_) as i32) }).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_n_d__str32, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        if ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1224); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_n_d__str33, fp);
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
        } else {
            F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_n_d__str34, fp);
            if (({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1212); (c.m.r32(a_) as i32) }) != 0) {
                { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
                { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
            } else {
                { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
            }
        }
        if (({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1216); (c.m.r32(a_) as i32) }) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1224); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                l_item_3 = F::UbScore_GetRewardItem(c, 4i32);
            } else {
                l_item_3 = F::UbScore_GetRewardItem(c, 5i32);
            }
            F::Flash_FindLabel(c, l_flash_1, sym::menu_n_d__str35, sym::menu_n_d__str36, fp);
            { let a5_ = (c.m.r32(sym::gUbResult)).wrapping_add(584); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, l_item_3, a5_) };
        }
        { let a0_ = ((c.m.r32(sym::gUbResult)).wrapping_add(868)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
        F::IconWin_Draw(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "UbResult_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn UbResult_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbResult_Update", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_flash_1: u32 = 0;
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
        l_flash_1 = ((c.m.r32(sym::gUbResult)).wrapping_add(868)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
            break 'f;
        }
        if ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1156); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            { let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1156); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
        }
        { let a0_ = ((c.m.r32(sym::gUbResult)).wrapping_add(868)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Advance(c, a0_) };
        {
            l_w_2 = c.m.r32(sym::gUbResult);
            if ((((c.m.r32((((l_w_2).wrapping_add(868)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12))) & (((1i32) as u32)))) != 0) {
                if ((((((c.m.r32((l_w_2).wrapping_add(1168)) as i32)) == (0i32)) as i32)) != 0) {
                    { let v_ = 1i32; let a_ = (l_w_2).wrapping_add(1920); c.m.w32(a_, (v_) as u32); };
                }
            }
            l_w_2 = c.m.r32(sym::gUbResult);
            if ((((c.m.r32((((l_w_2).wrapping_add(868)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12))) & (((4i32) as u32)))) != 0) {
                if ((((((c.m.r32((l_w_2).wrapping_add(1168)) as i32)) == (1i32)) as i32)) != 0) {
                    { let v_ = 1i32; let a_ = (l_w_2).wrapping_add(1912); c.m.w32(a_, (v_) as u32); };
                    l_w_2 = c.m.r32(sym::gUbResult);
                }
                { let a1_ = (c.m.r32((l_w_2).wrapping_add(1204)) as i32); F::UbScore_SetPage(c, (l_w_2).wrapping_add(1232), a1_) };
            }
        }
        if (((({ let a_ = (((c.m.r32(sym::gUbResult)).wrapping_add(868)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((8i32) as u32)))) != 0) {
            's1: {
                let sv3: i32 = { let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1168); (c.m.r32(a_) as i32) };
                match sv3 {
                    0i32 => {
                        if ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1208); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                            F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_n_d__str37, fp);
                            F::Flash_ClipGotoLabel(c, l_flash_1, fp, sym::menu_n_d__str38);
                            { let v_ = 1i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1172); c.m.w32(a_, (v_) as u32); };
                        } else {
                            F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_n_d__str39, fp);
                            F::Flash_ClipGotoLabel(c, l_flash_1, fp, sym::menu_n_d__str40);
                            { let v_ = 6i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1172); c.m.w32(a_, (v_) as u32); };
                        }
                        break 's1;
                    }
                    3i32 => {
                        F::Flash_GotoLabel(c, l_flash_1, sym::menu_n_d__str41, ((((1i32) as i64 as u64)) as u8));
                        break 's1;
                    }
                    4i32 => {
                        F::Flash_GotoLabel(c, l_flash_1, sym::menu_n_d__str42, ((((1i32) as i64 as u64)) as u8));
                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 6i32); };
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
        }
        if (({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1220); c.m.r8(a_) }) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1156); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1204); (c.m.r32(a_) as i32) }) < ({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1200); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                    if ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1204); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                        F::Flash_GotoLabel(c, l_flash_1, sym::menu_n_d__str43, ((((1i32) as i64 as u64)) as u8));
                    } else {
                        F::Flash_GotoLabel(c, l_flash_1, sym::menu_n_d__str44, ((((1i32) as i64 as u64)) as u8));
                    }
                    { let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1204); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1204); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                        F::Flash_GotoLabel(c, l_flash_1, sym::menu_n_d__str45, ((((1i32) as i64 as u64)) as u8));
                    } else {
                        F::Flash_GotoLabel(c, l_flash_1, sym::menu_n_d__str46, ((((1i32) as i64 as u64)) as u8));
                    }
                    { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 19i32); };
                    if (({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1216); (c.m.r32(a_) as i32) }) != 0) {
                        { let v_ = 15i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1172); c.m.w32(a_, (v_) as u32); };
                    } else {
                        if ((((({ let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(1232)).wrapping_add(676); (c.m.r32(a_) as i32) }) == (4i32)) as i32)) != 0) {
                            { let v_ = 11i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1172); c.m.w32(a_, (v_) as u32); };
                        } else {
                            if ((F::Rand_Range(c, ((2i32) as u32))) != 0) {
                                { let v_ = 12i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1172); c.m.w32(a_, (v_) as u32); };
                            } else {
                                { let v_ = 13i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1172); c.m.w32(a_, (v_) as u32); };
                            }
                        }
                    }
                }
                { let v_ = ((0i32) as u8); let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1220); c.m.w8(a_, v_); };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "UbResult_Update", &PROBE_N, &[], true); }
    r_
}

pub fn UbResult_UpdateTalk(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbResult_UpdateTalk", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1180); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        if ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1172); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            break 'f;
        }
        if ((((((((((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1184); (c.m.r32(a_) as i32) }) != ((1i32).wrapping_neg())) as i32)) != 0) && (((((F::Voice_GetStat(c, 0i32)) != (5i32)) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1172); (c.m.r32(a_) as i32) }) == ({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1176); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                { let v_ = 1i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1180); c.m.w32(a_, (v_) as u32); };
            }
            break 'f;
        }
    } else {
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1180); c.m.w32(a_, (v_) as u32); };
    }
    's1: {
        let sv1: i32 = { let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1172); (c.m.r32(a_) as i32) };
        match sv1 {
            1i32 => {
                { let v_ = 56i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1184); c.m.w32(a_, (v_) as u32); };
                { let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1172); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            6i32 => {
                { let v_ = 57i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1184); c.m.w32(a_, (v_) as u32); };
                { let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1172); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 | 7i32 => {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1172); c.m.w32(a_, (v_) as u32); };
                { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1184); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            11i32 => {
                { let v_ = 58i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1184); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            12i32 => {
                { let v_ = 59i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1184); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            13i32 => {
                { let v_ = 60i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1184); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            14i32 => {
                { let v_ = 61i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1184); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            15i32 => {
                { let v_ = 81i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1184); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1172); (c.m.r32(a_) as i32) }) > (10i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1172); c.m.w32(a_, (v_) as u32); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1184); (c.m.r32(a_) as i32) }) != ((1i32).wrapping_neg())) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(16); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1184); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34661i32, a2_) };
        { let v_ = { let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1172); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1176); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "UbResult_UpdateTalk", &PROBE_N, &[], true); }
    r_
}

pub fn UbResult_Input(c: &mut Ctx, mut p_result_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbResult_Input", &PROBE_N, &[(p_result_1) as u32], false); }
    let mut l_up_2: i32 = 0;
    let mut l_ok_3: i32 = 0;
    let mut l_down_4: i32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_row_6: i32 = 0;
    let mut l_bonus_7: i32 = 0;
    let r_ = 'f: {
    l_up_2 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) as i32);
    l_ok_3 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) as i32);
    l_down_4 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) as i32);
    if ((((((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1168); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) && (({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1920); (c.m.r32(a_) as i32) }) != 0)) as i32)) != 0) {
        if ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1208); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) {
            { let r_ = 1i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            { let r_ = 2i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            { let v_ = 15i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1156); c.m.w32(a_, (v_) as u32); };
            { let v_ = 5i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1168); c.m.w32(a_, (v_) as u32); };
        } else {
            { let a0_ = ((c.m.r32(sym::gUbResult)).wrapping_add(868)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_n_d__str47, ((((1i32) as i64 as u64)) as u8)) };
            { let v_ = 1i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1168); c.m.w32(a_, (v_) as u32); };
        }
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1920); c.m.w32(a_, (v_) as u32); };
    }
    if ((((!(((({ let a_ = (((c.m.r32(sym::gUbResult)).wrapping_add(868)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f;
    }
    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
        break 'f;
    }
    if ((((!(((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1104); (c.m.r32(a_) as i32) }) & (4i32))) != 0)) as i32)) != 0) {
        { let r_ = 4i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    's1: {
        let sv8: i32 = { let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1168); (c.m.r32(a_) as i32) };
        match sv8 {
            1i32 => {
                if (({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1912); (c.m.r32(a_) as i32) }) != 0) {
                    l_row_6 = ((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1204); (c.m.r32(a_) as i32) }) != (0i32)) as i32);
                    if (({ let a0_ = (c.m.r32(sym::gUbResult)).wrapping_add(1232); let a2_ = ({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1916); (c.m.r32(a_) as i32) }).wrapping_sub((l_row_6).wrapping_mul({ let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(1232)).wrapping_add(652); (c.m.r32(a_) as i32) })); F::UbScore_CountLine(c, a0_, l_row_6, a2_, 15i32) }) != 0) {
                        { let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1916); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        if ((((((((((((((l_row_6) != 0) && (((((rt::rem_i32({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1916); (c.m.r32(a_) as i32) }, 3i32)) == (0i32)) as i32)) != 0)) as i32)) != 0) || ((((((((!((l_row_6) != 0)) as i32)) != 0) && (((((rt::rem_i32({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1916); (c.m.r32(a_) as i32) }, { let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(1232)).wrapping_add(652); (c.m.r32(a_) as i32) })) == (0i32)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) || ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1916); (c.m.r32(a_) as i32) }) == (({ let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(1232)).wrapping_add(656); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(1232)).wrapping_add(652); (c.m.r32(a_) as i32) }))) as i32)) != 0)) as i32)) != 0) {
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1912); c.m.w32(a_, (v_) as u32); };
                            { let v_ = ((1i32) as u8); let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1220); c.m.w8(a_, v_); };
                        }
                    }
                }
                if ((l_ok_3) != 0) {
                    {
                        l_i_5 = 0i32;
                        'l2: loop {
                            if !(((((l_i_5) < (({ let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(1232)).wrapping_add(656); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(1232)).wrapping_add(652); (c.m.r32(a_) as i32) }))) as i32)) != 0) { break 'l2; }
                            'c3: {
                                l_row_6 = 0i32;
                                l_bonus_7 = 0i32;
                                if (((((l_i_5) < ({ let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(1232)).wrapping_add(652); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                    l_bonus_7 = 0i32;
                                    l_row_6 = l_i_5;
                                } else {
                                    l_row_6 = (l_i_5).wrapping_sub({ let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(1232)).wrapping_add(652); (c.m.r32(a_) as i32) });
                                    l_bonus_7 = 1i32;
                                }
                                'l4: while ((((!(({ let a0_ = (c.m.r32(sym::gUbResult)).wrapping_add(1232); F::UbScore_CountLine(c, a0_, l_bonus_7, l_row_6, 9999i32) }) != 0)) as i32)) != 0) {
                                }
                            }
                            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
                        }
                    }
                    { let v_ = ((1i32) as u8); let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1220); c.m.w8(a_, v_); };
                    { let v_ = { let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1200); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1204); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ({ let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(1232)).wrapping_add(656); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(1232)).wrapping_add(652); (c.m.r32(a_) as i32) }); let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1916); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1200); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1204); c.m.w32(a_, (v_) as u32); };
                    { let a0_ = (c.m.r32(sym::gUbResult)).wrapping_add(1232); let a1_ = { let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1204); (c.m.r32(a_) as i32) }; F::UbScore_SetPage(c, a0_, a1_) };
                }
                if (({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1220); c.m.r8(a_) }) != 0) {
                    if ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1204); (c.m.r32(a_) as i32) }) < ({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1200); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        if ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1204); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                            { let a0_ = ((c.m.r32(sym::gUbResult)).wrapping_add(868)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_n_d__str48, ((((1i32) as i64 as u64)) as u8)) };
                        } else {
                            { let a0_ = ((c.m.r32(sym::gUbResult)).wrapping_add(868)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_n_d__str49, ((((1i32) as i64 as u64)) as u8)) };
                        }
                        { let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1204); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    } else {
                        if ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1204); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                            { let a0_ = ((c.m.r32(sym::gUbResult)).wrapping_add(868)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_n_d__str50, ((((1i32) as i64 as u64)) as u8)) };
                        } else {
                            { let a0_ = ((c.m.r32(sym::gUbResult)).wrapping_add(868)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_n_d__str51, ((((1i32) as i64 as u64)) as u8)) };
                        }
                    }
                    { let v_ = ((0i32) as u8); let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1220); c.m.w8(a_, v_); };
                }
                if ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1916); (c.m.r32(a_) as i32) }) == (({ let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(1232)).wrapping_add(656); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(1232)).wrapping_add(652); (c.m.r32(a_) as i32) }))) as i32)) != 0) {
                    { let v_ = 2i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1168); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            2i32 => {
                if ((l_up_2) != 0) {
                    if ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1204); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                        { let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1204); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1204); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                            { let a0_ = ((c.m.r32(sym::gUbResult)).wrapping_add(868)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_n_d__str52, ((((1i32) as i64 as u64)) as u8)) };
                        } else {
                            { let a0_ = ((c.m.r32(sym::gUbResult)).wrapping_add(868)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_n_d__str53, ((((1i32) as i64 as u64)) as u8)) };
                        }
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    }
                } else {
                    if ((l_down_4) != 0) {
                        if ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1204); (c.m.r32(a_) as i32) }) < ({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1200); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                            { let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1204); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            if ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1204); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                                { let a0_ = ((c.m.r32(sym::gUbResult)).wrapping_add(868)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_n_d__str54, ((((1i32) as i64 as u64)) as u8)) };
                            } else {
                                { let a0_ = ((c.m.r32(sym::gUbResult)).wrapping_add(868)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_n_d__str55, ((((1i32) as i64 as u64)) as u8)) };
                            }
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                        }
                    } else {
                        if ((l_ok_3) != 0) {
                            { let v_ = 3i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1168); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1916); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 60i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1156); c.m.w32(a_, (v_) as u32); };
                            if ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1204); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                                { let a0_ = ((c.m.r32(sym::gUbResult)).wrapping_add(868)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_n_d__str56, ((((1i32) as i64 as u64)) as u8)) };
                            } else {
                                { let a0_ = ((c.m.r32(sym::gUbResult)).wrapping_add(868)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_n_d__str57, ((((1i32) as i64 as u64)) as u8)) };
                            }
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                        }
                    }
                }
                break 's1;
            }
            3i32 => {
                's5: {
                    let sv9: i32 = { let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1916); (c.m.r32(a_) as i32) };
                    match sv9 {
                        0i32 => {
                            if ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1156); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                                { let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1916); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                { let a0_ = ((c.m.r32(sym::gUbResult)).wrapping_add(868)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::UbScore_PlateGoto(c, a0_, 0i32, 1i32) };
                                { let v_ = { let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1232); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(1232)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 14i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1172); c.m.w32(a_, (v_) as u32); };
                            }
                            break 's5;
                        }
                        1i32 => {
                            if (({ let a0_ = (c.m.r32(sym::gUbResult)).wrapping_add(1232); F::UbScore_ConvertStep(c, a0_, 64i32) }) != 0) {
                                { let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1916); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                { let v_ = 15i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1156); c.m.w32(a_, (v_) as u32); };
                                { let a0_ = ((c.m.r32(sym::gUbResult)).wrapping_add(868)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::UbScore_PlateGoto(c, a0_, 0i32, 0i32) };
                                { let v_ = { let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(1232)).wrapping_add(4); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(1232)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            }
                            break 's5;
                        }
                        2i32 => {
                            if ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1156); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                                { let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1916); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                { let a0_ = ((c.m.r32(sym::gUbResult)).wrapping_add(868)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::UbScore_PlateGoto(c, a0_, 1i32, 1i32) };
                            }
                            break 's5;
                        }
                        3i32 => {
                            if (({ let a0_ = ((c.m.r32(sym::gUbResult)).wrapping_add(1232)).wrapping_add(8); let a1_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); F::UbScore_Transfer(c, a0_, a1_, 64i32, F32(0x3f800000)) }) != 0) {
                                { let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1916); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                { let a0_ = ((c.m.r32(sym::gUbResult)).wrapping_add(868)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::UbScore_PlateGoto(c, a0_, 1i32, 0i32) };
                            }
                            if ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); (c.m.r32(a_) as i32) }) > (9999999i32)) as i32)) != 0) {
                                { let v_ = 9999999i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); c.m.w32(a_, (v_) as u32); };
                            }
                            break 's5;
                        }
                        4i32 => {
                            if ((l_ok_3) != 0) {
                                { let a0_ = ((c.m.r32(sym::gUbResult)).wrapping_add(868)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_n_d__str58, ((((1i32) as i64 as u64)) as u8)) };
                                if (({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1216); (c.m.r32(a_) as i32) }) != 0) {
                                    { let v_ = 4i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1168); c.m.w32(a_, (v_) as u32); };
                                } else {
                                    { let r_ = 1i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                    { let r_ = 2i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                    { let v_ = 15i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1156); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 5i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1168); c.m.w32(a_, (v_) as u32); };
                                }
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                            }
                            break 's5;
                        }
                        _ => {
                        }
                    }
                }
                if ((((((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1916); (c.m.r32(a_) as i32) }) >= (1i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1916); (c.m.r32(a_) as i32) }) <= (3i32)) as i32)) != 0)) as i32)) != 0) {
                    if ((l_ok_3) != 0) {
                        if ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1916); (c.m.r32(a_) as i32) }) < (2i32)) as i32)) != 0) {
                            'l6: while ((((!(({ let a0_ = (c.m.r32(sym::gUbResult)).wrapping_add(1232); F::UbScore_ConvertStep(c, a0_, 9999999i32) }) != 0)) as i32)) != 0) {
                            }
                            { let v_ = { let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(1232)).wrapping_add(4); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gUbResult)).wrapping_add(1232)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            'l7: while ((((!(({ let a0_ = ((c.m.r32(sym::gUbResult)).wrapping_add(1232)).wrapping_add(8); let a1_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); F::UbScore_Transfer(c, a0_, a1_, 9999999i32, F32(0x3f800000)) }) != 0)) as i32)) != 0) {
                            }
                        } else {
                            if ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1916); (c.m.r32(a_) as i32) }) < (4i32)) as i32)) != 0) {
                                'l8: while ((((!(({ let a0_ = ((c.m.r32(sym::gUbResult)).wrapping_add(1232)).wrapping_add(8); let a1_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); F::UbScore_Transfer(c, a0_, a1_, 9999999i32, F32(0x3f800000)) }) != 0)) as i32)) != 0) {
                                }
                            }
                        }
                        if ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); (c.m.r32(a_) as i32) }) > (9999999i32)) as i32)) != 0) {
                            { let v_ = 9999999i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); c.m.w32(a_, (v_) as u32); };
                        }
                        { let a0_ = ((c.m.r32(sym::gUbResult)).wrapping_add(868)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::UbScore_PlateGoto(c, a0_, 0i32, 0i32) };
                        { let a0_ = ((c.m.r32(sym::gUbResult)).wrapping_add(868)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::UbScore_PlateGoto(c, a0_, 1i32, 0i32) };
                        { let v_ = 4i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1916); c.m.w32(a_, (v_) as u32); };
                    }
                }
                break 's1;
            }
            4i32 => {
                if ((l_ok_3) != 0) {
                    { let a0_ = ((c.m.r32(sym::gUbResult)).wrapping_add(868)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_n_d__str59, ((((1i32) as i64 as u64)) as u8)) };
                    { let r_ = 1i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let r_ = 2i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 15i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1156); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 5i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1168); c.m.w32(a_, (v_) as u32); };
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
    if rt::probing() { crate::host::harness::probe(c, "UbResult_Input", &PROBE_N, &[], true); }
    r_
}

pub fn UbResult_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbResult_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = 1i32; c.m.w32(fp, (v_) as u32); }
        F::UbResult_Init(c, p_section_1);
        F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
        'l1: while ((1i32) != 0) {
            F::Gfx_BeginFrame(c);
            F::Pad_Update(c);
            F::Snd_Update(c);
            F::ColorFade_Update(c);
            F::UbResult_Update(c);
            F::UbResult_UpdateTalk(c);
            F::UbResult_Draw(c);
            F::ColorFade_Draw(c);
            F::Gfx_EndFrame(c, 1i32);
            F::Dma_Flush(c);
            F::File_Stub264D90(c);
            if ((F::ColorFade_IsInDone(c)) != 0) {
                if ((((((((!(((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1104); (c.m.r32(a_) as i32) }) & (8i32))) != 0)) as i32)) != 0) && (((({ let a_ = (((c.m.r32(sym::gUbResult)).wrapping_add(868)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
                    { let r_ = 8i32; let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
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
            if (((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1104); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1156); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                }
            } else {
                if ((((({ let a_ = (c.m.r32(sym::gUbResult)).wrapping_add(1172); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::UbResult_Input(c, fp);
                }
            }
        }
        F::UbResult_Term(c);
        F::Dma_ResetBuffers(c);
        break 'f (c.m.r32(fp) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "UbResult_Run", &PROBE_N, &[], true); }
    r_
}
