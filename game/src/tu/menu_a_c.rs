#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Title_Idle(c: &mut Ctx, mut p_result_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Title_Idle", &PROBE_N, &[(p_result_1) as u32], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(196); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) > (1800i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gTitle)).wrapping_add(196); c.m.w32(a_, (v_) as u32); };
        { let r_ = 1i32; let a_ = (c.m.r32(sym::gTitle)).wrapping_add(168); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let r_ = 2i32; let a_ = (c.m.r32(sym::gTitle)).wrapping_add(168); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let v_ = 15i32; let a_ = (c.m.r32(sym::gTitle)).wrapping_add(176); c.m.w32(a_, (v_) as u32); };
        's1: {
            let sv2: i32 = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(40); (c.m.r32(a_) as i32) };
            match sv2 {
                0i32 | 1i32 => {
                    F::Demo_SetupBattle(c);
                    { let v_ = 1i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
                    { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(40); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    break 's1;
                }
                _ => {
                    { let v_ = 2i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Title_Idle", &PROBE_N, &[], true); }
    r_
}

pub fn Title_Init(c: &mut Ctx, mut p_section_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Title_Init", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        if (((((c.m.r32(sym::gMenuArc1)) == (((0i32) as u32))) as i32)) != 0) {
            { let v_ = { let a0_ = ({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::File_LoadSync(c, a0_, ((0i32) as u32), 0i32) }; let a_ = sym::gMenuArc1; c.m.w32(a_, v_); };
        }
        { let v_ = F::Heap_Alloc(c, 208i32, ((32i32) as u32), 0i32, 2i32); let a_ = sym::gTitle; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gTitle); F::memset(c, a0_, 0i32, ((208i32) as u32)) }; };
        { let v_ = (c.m.r32(sym::gMenuArc1)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc1)).wrapping_add((((p_section_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = c.m.r32(sym::gTitle); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gTitle); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gTitle)).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = c.m.r32(fp); let a_ = (c.m.r32(sym::gTitle)).wrapping_add(56); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTitle)).wrapping_add(60)).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTitle)).wrapping_add(60)).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTitle)).wrapping_add(60)).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTitle)).wrapping_add(60)).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTitle)).wrapping_add(60)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTitle)).wrapping_add(60)).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTitle)).wrapping_add(60)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTitle)).wrapping_add(60)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTitle)).wrapping_add(60)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTitle)).wrapping_add(60)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTitle)).wrapping_add(60)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTitle)).wrapping_add(60)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTitle)).wrapping_add(60)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = c.m.r32(fp); let a_ = (c.m.r32(sym::gTitle)).wrapping_add(164); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTitle)).wrapping_add(60)).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTitle)).wrapping_add(60)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTitle)).wrapping_add(60)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTitle)).wrapping_add(60)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTitle)).wrapping_add(60)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTitle)).wrapping_add(60)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTitle)).wrapping_add(60)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTitle)).wrapping_add(60)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTitle)).wrapping_add(60)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTitle)).wrapping_add(60)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTitle)).wrapping_add(60)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gTitle)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gTitle)).wrapping_add(60); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gTitle)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::Dialog_Init(c, a0_, ((0i32) as u32), 0i32) };
        { let _ = 1i32; F::McFlow_Init(c) };
        if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (8i32))) != 0) {
            { let v_ = 2i32; let a_ = (c.m.r32(sym::gTitle)).wrapping_add(180); c.m.w32(a_, (v_) as u32); };
            { let v_ = 1i32; let a_ = (c.m.r32(sym::gTitle)).wrapping_add(172); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 1i32; let a_ = (c.m.r32(sym::gTitle)).wrapping_add(180); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gTitle)).wrapping_add(172); c.m.w32(a_, (v_) as u32); };
        }
        { let v_ = ((F::Rand_Range(c, ((6i32) as u32))) as i32); let a_ = (c.m.r32(sym::gTitle)).wrapping_add(192); c.m.w32(a_, (v_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Title_Init", &PROBE_N, &[], true); }
    r_
}

pub fn Title_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Title_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    F::Dialog_Term(c);
    F::McFlow_Term(c);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gTitle)).wrapping_add(12)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Destroy(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gTitle)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gTitle)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gTitle)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gTitle); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gTitle; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Title_Term", &PROBE_N, &[], true); }
    r_
}

pub fn Title_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Title_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_i_1: i32 = 0;
    let mut l_flash_2: u32 = 0;
    let r_ = 'f: {
        { let a0_ = { let a_ = (c.m.r32(sym::gTitle)).wrapping_add(56); c.m.r32(a_) }; F::Sprite_DrawPicture(c, a0_, 0i32, 0i32, 128i32) };
        l_flash_2 = ((c.m.r32(sym::gTitle)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        {
            l_i_1 = 0i32;
            'l1: loop {
                if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_a_c__str0, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(28), fp);
                    { let a3_ = ((c.m.r32(sym::gTitle)).wrapping_add(200)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); F::FlashAnim_Scroll(c, l_flash_2, fp, fp.wrapping_add(8), a3_, ((0i32) as u32), (if ((l_i_1) != 0) { F32(0xbf888888) } else { F32(0xbf360b60) }), F32(0x00000000)) };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        {
            l_i_1 = 0i32;
            'l3: loop {
                if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let v_ = (l_i_1).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_a_c__str1, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(28), fp);
                    { let a2_ = (if (((((l_i_1) < ({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(180); (c.m.r32(a_) as i32) })) as i32)) != 0) { F32(0x3f800000) } else { F32(0x3f000000) }); F::Flash_ClipSetColor(c, l_flash_2, fp, a2_) };
                    F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_a_c__str2, fp);
                    F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                    F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_a_c__str3, fp);
                    F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        {
            l_i_1 = 0i32;
            'l5: loop {
                if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l5; }
                'c6: {
                    { let a0_ = ((c.m.r32(sym::gTitle)).wrapping_add(12)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Title_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn Title_ClipGoto(c: &mut Ctx, mut p_idx_1: i32, mut p_kind_2: i32, mut p_label_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Title_ClipGoto", &PROBE_N, &[(p_idx_1) as u32, (p_kind_2) as u32, (p_label_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_flash_4: u32 = 0;
    let r_ = 'f: {
        l_flash_4 = ((c.m.r32(sym::gTitle)).wrapping_add(12)).wrapping_add((((p_idx_1) as u32)).wrapping_mul(44));
        's1: {
            let sv5: i32 = p_kind_2;
            match sv5 {
                1i32 => {
                    { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_a_c__str4, &[]); };
                    break 's1;
                }
                2i32 => {
                    { let _ = { let a2_ = ({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(172); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_a_c__str5, &[((a2_) as i64 as u64)]) }; };
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
    if rt::probing() { crate::host::harness::probe(c, "Title_ClipGoto", &PROBE_N, &[], true); }
    r_
}

pub fn Title_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Title_Update", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gTitle)).wrapping_add(12)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Advance(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if (((({ let a_ = (((c.m.r32(sym::gTitle)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(16); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 55i32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Title_Update", &PROBE_N, &[], true); }
    r_
}

pub fn Title_OnCardDone(c: &mut Ctx, mut p_result_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Title_OnCardDone", &PROBE_N, &[(p_result_1) as u32], false); }
    let r_ = 'f: {
    if (((((p_result_1) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = 0i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
    }
    { let r_ = 1i32; let a_ = (c.m.r32(sym::gTitle)).wrapping_add(168); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    { let r_ = 2i32; let a_ = (c.m.r32(sym::gTitle)).wrapping_add(168); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    { let v_ = 15i32; let a_ = (c.m.r32(sym::gTitle)).wrapping_add(176); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Title_OnCardDone", &PROBE_N, &[], true); }
    r_
}

pub fn Title_OnCardCancel(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Title_OnCardCancel", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Title_ClipGoto(c, 0i32, 2i32, sym::menu_a_c__str6);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Title_OnCardCancel", &PROBE_N, &[], true); }
    r_
}

pub fn Title_Input(c: &mut Ctx, mut p_result_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Title_Input", &PROBE_N, &[(p_result_1) as u32], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(184); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((4096i32) as u32)))) != 0) {
            { let a0_ = ((c.m.r32(sym::gTitle)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_a_c__str7, ((((1i32) as i64 as u64)) as u8)) };
            { let v_ = 1i32; let a_ = (c.m.r32(sym::gTitle)).wrapping_add(184); c.m.w32(a_, (v_) as u32); };
            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
            break 'f;
        }
    }
    if ((((!(((({ let a_ = (((c.m.r32(sym::gTitle)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f;
    }
    if ((((!(((({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(168); (c.m.r32(a_) as i32) }) & (4i32))) != 0)) as i32)) != 0) {
        F::Title_ClipGoto(c, 0i32, 1i32, sym::menu_a_c__str8);
        { let v_ = 1i32; let a_ = (c.m.r32(sym::gTitle)).wrapping_add(184); c.m.w32(a_, (v_) as u32); };
        { let r_ = 4i32; let a_ = (c.m.r32(sym::gTitle)).wrapping_add(168); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        if (((((F::Rand_Range(c, ((7i32) as u32))) != (((0i32) as u32))) as i32)) != 0) {
            { let a2_ = { let a_ = (c.m.r32(sym::gTitle)).wrapping_add(192); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, ((0i32) as u32), 34553i32, a2_) };
        } else {
            F::Voice_PlayWithSubtitle(c, ((0i32) as u32), 34553i32, 64i32);
        }
    }
    's1: {
        let sv2: i32 = { let a_ = (c.m.r32(sym::gTitle)).wrapping_add(184); (c.m.r32(a_) as i32) };
        match sv2 {
            1i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((4096i32) as u32)))) != 0) {
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gTitle)).wrapping_add(196); c.m.w32(a_, (v_) as u32); };
                    { let a0_ = ((c.m.r32(sym::gTitle)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_a_c__str9, ((((1i32) as i64 as u64)) as u8)) };
                    F::Title_ClipGoto(c, 0i32, 1i32, sym::menu_a_c__str10);
                    F::Title_ClipGoto(c, 0i32, 2i32, sym::menu_a_c__str11);
                    { let v_ = 2i32; let a_ = (c.m.r32(sym::gTitle)).wrapping_add(184); c.m.w32(a_, (v_) as u32); };
                    { let a2_ = ({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(192); (c.m.r32(a_) as i32) }).wrapping_add(6i32); F::Voice_PlayWithSubtitle(c, ((0i32) as u32), 34553i32, a2_) };
                    { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 56i32); };
                } else {
                    F::Title_Idle(c, p_result_1);
                }
                break 's1;
            }
            2i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gTitle)).wrapping_add(196); c.m.w32(a_, (v_) as u32); };
                    if ((((({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(180); (c.m.r32(a_) as i32) }) >= (2i32)) as i32)) != 0) {
                        F::Title_ClipGoto(c, 0i32, 2i32, sym::menu_a_c__str12);
                        { let a_ = (c.m.r32(sym::gTitle)).wrapping_add(172); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(172); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                            { let v_ = ({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(180); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = (c.m.r32(sym::gTitle)).wrapping_add(172); c.m.w32(a_, (v_) as u32); };
                        }
                        F::Title_ClipGoto(c, 0i32, 2i32, sym::menu_a_c__str13);
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    }
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gTitle)).wrapping_add(196); c.m.w32(a_, (v_) as u32); };
                        if ((((({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(180); (c.m.r32(a_) as i32) }) >= (2i32)) as i32)) != 0) {
                            F::Title_ClipGoto(c, 0i32, 2i32, sym::menu_a_c__str14);
                            { let a_ = (c.m.r32(sym::gTitle)).wrapping_add(172); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            if ((((({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(172); (c.m.r32(a_) as i32) }) >= ({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(180); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gTitle)).wrapping_add(172); c.m.w32(a_, (v_) as u32); };
                            }
                            F::Title_ClipGoto(c, 0i32, 2i32, sym::menu_a_c__str15);
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                        }
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gTitle)).wrapping_add(196); c.m.w32(a_, (v_) as u32); };
                            F::Title_ClipGoto(c, 0i32, 2i32, sym::menu_a_c__str16);
                            's2: {
                                let sv3: i32 = { let a_ = (c.m.r32(sym::gTitle)).wrapping_add(172); (c.m.r32(a_) as i32) };
                                match sv3 {
                                    0i32 => {
                                        F::McFlow_Start(c, 3i32);
                                        F::McFlow_SetDoneCb(c, 0i32, fid::Title_OnCardDone, ((((p_result_1) as u64)) as i32));
                                        F::McFlow_SetDoneCb(c, 1i32, fid::Title_OnCardCancel, ((((((0i32) as u32)) as u64)) as i32));
                                        break 's2;
                                    }
                                    1i32 => {
                                        { let v_ = 0i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
                                        { let r_ = 1i32; let a_ = (c.m.r32(sym::gTitle)).wrapping_add(168); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                        { let r_ = 2i32; let a_ = (c.m.r32(sym::gTitle)).wrapping_add(168); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                        { let v_ = 15i32; let a_ = (c.m.r32(sym::gTitle)).wrapping_add(176); c.m.w32(a_, (v_) as u32); };
                                        break 's2;
                                    }
                                    _ => {
                                    }
                                }
                            }
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                        } else {
                            F::Title_Idle(c, p_result_1);
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
    if rt::probing() { crate::host::harness::probe(c, "Title_Input", &PROBE_N, &[], true); }
    r_
}

pub fn Title_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Title_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_wait_2: i32 = 0;
    let r_ = 'f: {
        { let v_ = 1i32; c.m.w32(fp, (v_) as u32); }
        l_wait_2 = 0i32;
        F::Title_Init(c, p_section_1);
        F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
        'l1: while ((1i32) != 0) {
            F::Gfx_BeginFrame(c);
            F::Pad_Update(c);
            F::Snd_Update(c);
            F::ColorFade_Update(c);
            { let a0_ = { let a_ = (c.m.r32(sym::gTitle)).wrapping_add(164); c.m.r32(a_) }; F::Sprite_DrawPicture(c, a0_, 0i32, 0i32, 128i32) };
            F::ColorFade_Draw(c);
            F::Gfx_EndFrame(c, 1i32);
            F::Dma_Flush(c);
            F::File_Stub264D90(c);
            if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
                continue 'l1;
            }
            if ((F::ColorFade_IsInDone(c)) != 0) {
                { let r_ = 8i32; let a_ = (c.m.r32(sym::gTitle)).wrapping_add(168); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
            if ((F::ColorFade_IsFadingOut(c)) != 0) {
                continue 'l1;
            }
            if ((F::ColorFade_IsOutDone(c)) != 0) {
                break 'l1;
            }
            if (((({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(168); (c.m.r32(a_) as i32) }) & (8i32))) != 0) {
                { let o_ = l_wait_2; let n_ = (o_).wrapping_add(1); l_wait_2 = n_; };
                if (((((l_wait_2) > (300i32)) as i32)) != 0) {
                    F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                }
            }
        }
        F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
        {
            'l2: loop {
                'c3: {
                    F::Gfx_BeginFrame(c);
                    F::Pad_Update(c);
                    F::Snd_Update(c);
                    F::ColorFade_Update(c);
                    if ((((!(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0)) as i32)) != 0) {
                        F::Title_Update(c);
                    }
                    F::Title_Draw(c);
                    { let v_ = F::McFlow_Update(c); let a_ = (c.m.r32(sym::gTitle)).wrapping_add(188); c.m.w32(a_, (v_) as u32); };
                    F::ColorFade_Draw(c);
                    F::Gfx_EndFrame(c, 1i32);
                    F::Dma_Flush(c);
                    F::File_Stub264D90(c);
                    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
                        break 'c3;
                    }
                    if ((F::ColorFade_IsFadingOut(c)) != 0) {
                        F::Voice_FadeOutStep(c, 0i32);
                        F::Bgm_FadeOutStep(c);
                        break 'c3;
                    }
                    if ((F::ColorFade_IsOutDone(c)) != 0) {
                        break 'l2;
                    }
                    if (((({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(168); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
                        if ((((({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(176); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) == ((1i32).wrapping_neg())) as i32)) != 0) {
                            F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                        }
                    } else {
                        if ((((({ let a_ = (c.m.r32(sym::gTitle)).wrapping_add(188); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                            F::Title_Input(c, fp);
                        }
                    }
                }
            }
        }
        if ((((((c.m.r32(fp) as i32)) == (0i32)) as i32)) != 0) {
            { let r_ = 8i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        { let _ = F::sceGsSyncPath(c, 0i32, ((0i32) as u16)); };
        F::Title_Term(c);
        F::Dma_ResetBuffers(c);
        break 'f (c.m.r32(fp) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Title_Run", &PROBE_N, &[], true); }
    r_
}
