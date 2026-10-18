#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn MisResult_MarkCleared(c: &mut Ctx, mut p_mission_1: i32, mut p_count_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MisResult_MarkCleared", &PROBE_N, &[(p_mission_1) as u32, (p_count_2) as u32], false); }
    let mut l_isNew_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_isNew_3 = 0i32;
    { let v_ = 0i32; let a_ = p_count_2; c.m.w32(a_, (v_) as u32); };
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (100i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(640)).wrapping_add(12)).wrapping_add((((l_i_4) as u32)).wrapping_mul(12)); c.m.r8(a_) }) as i32)) != (0i32)) as i32)) != 0) {
                    { let a_ = p_count_2; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if (((((p_mission_1) == (l_i_4)) as i32)) != 0) {
                        l_isNew_3 = 1i32;
                        { let v_ = ((l_isNew_3) as u8); let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(640)).wrapping_add(12)).wrapping_add((((l_i_4) as u32)).wrapping_mul(12)); c.m.w8(a_, v_); };
                        { let a_ = p_count_2; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    }
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    if ((((((((((c.m.r32(p_count_2) as i32)) == (100i32)) as i32)) != 0) && ((l_isNew_3) != 0)) as i32)) != 0) {
        { let r_ = 4i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(520); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    break 'f l_isNew_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "MisResult_MarkCleared", &PROBE_N, &[], true); }
    r_
}

pub fn MisResult_ClipScissorOn(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MisResult_ClipScissorOn", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Sprite_SetScissor(c, 132i32, 476i32, 90i32, 416i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "MisResult_ClipScissorOn", &PROBE_N, &[], true); }
    r_
}

pub fn MisResult_ClipScissorOff(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MisResult_ClipScissorOff", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Sprite_SetScissor(c, 0i32, 511i32, 0i32, 447i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "MisResult_ClipScissorOff", &PROBE_N, &[], true); }
    r_
}

pub fn MisResult_Init(c: &mut Ctx, mut p_section_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MisResult_Init", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_2: i32 = 0;
    let mut l_total_3: i32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = F::Heap_Alloc(c, ((2080u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gMisResult; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gMisResult); F::memset(c, a0_, 0i32, 2080u32) }; };
        { let v_ = (c.m.r32(sym::gMenuArc3)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc3)).wrapping_add((((p_section_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = c.m.r32(sym::gMisResult); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gMisResult); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = c.m.r32(fp); let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1052); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((39i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((40i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((41i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((30i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((7i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((8i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((9i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((10i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((12i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((38i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((13i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((35i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((14i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((36i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((15i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((33i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((16i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((32i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((17i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((34i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((18i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((19i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((20i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((21i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((22i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((23i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((25i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((26i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((27i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((28i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((29i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((30i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((31i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((32i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((33i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((34i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((35i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((36i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((37i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((38i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((39i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((43i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((31i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((44i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((45i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((45i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((42i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((46i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((43i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((47i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((44i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((37i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1056)).wrapping_add((((46i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gMisResult)).wrapping_add(1056); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let v_ = rt::rem_i32(F::Rand_Libc(c), 32i32); let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1340); c.m.w32(a_, (v_) as u32); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = c.m.r32(fp); F::IconWin_Init(c, a0_, a1_) };
        F::IconWin_Open(c);
        F::IconWin_SetIcon(c, 1i32);
        { let v_ = ({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(16); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(20); c.m.w32(a_, v_); };
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (6i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a0_ = ((c.m.r32(sym::gMisResult)).wrapping_add(24)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); let a1_ = { let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(12); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, ((0i32) as u32)) };
                    if (((((((((l_i_2) == (4i32)) as i32)) != 0) || (((((l_i_2) == (5i32)) as i32)) != 0)) as i32)) != 0) {
                        { let a0_ = ((c.m.r32(sym::gMisResult)).wrapping_add(24)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); F::TextBox_SetUnk50(c, a0_, 1i32) };
                    } else {
                        { let a0_ = ((c.m.r32(sym::gMisResult)).wrapping_add(24)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); F::TextBox_SetUnk50(c, a0_, 0i32) };
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let a0_ = (c.m.r32(sym::gMisResult)).wrapping_add(864); let a1_ = { let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(20); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, ((0i32) as u32)) };
        { let a0_ = (c.m.r32(sym::gMisResult)).wrapping_add(864); F::TextBox_SetUnk50(c, a0_, 0i32) };
        { let a0_ = (c.m.r32(sym::gMisResult)).wrapping_add(864); F::TextBox_SetLineOffsets(c, a0_, 12i32, 0i32, 0i32, 0i32, 0i32) };
        { let a0_ = ((c.m.r32(sym::gMisResult)).wrapping_add(24)).wrapping_add((((3i32) as u32)).wrapping_mul(140)); F::TextBox_SetMaxWidth(c, a0_, 235i32) };
        { let a0_ = ((c.m.r32(sym::gMisResult)).wrapping_add(24)).wrapping_add((((3i32) as u32)).wrapping_mul(140)); F::TextBox_SetLineOffsets(c, a0_, 0i32, (10i32).wrapping_neg(), 0i32, 0i32, 0i32) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1004); c.m.w32(a_, v_); };
        { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1332); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1352); c.m.w32(a_, (v_) as u32); };
        { let v_ = 15i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1304); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((0i32) as u8); let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1380); c.m.w8(a_, v_); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(2068); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(2072); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1328); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1372); c.m.w32(a_, (v_) as u32); };
        { let v_ = (({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1596); (c.m.r32(a_) as i32) }).wrapping_mul(5i32)).wrapping_add({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1600); (c.m.r32(a_) as i32) }); let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1384); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a1_ = (c.m.r32(sym::gMisResult)).wrapping_add(1388); let a2_ = (c.m.r32(sym::gMisResult)).wrapping_add(1348); F::UbScore_Fill(c, 1i32, a1_, a2_) }; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1356); c.m.w32(a_, (v_) as u32); };
        { let a0_ = { let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1004); c.m.r32(a_) }; let a1_ = (c.m.r32(sym::gMisResult)).wrapping_add(1388); F::UbScore_CalcPoints(c, a0_, a1_) };
        l_total_3 = { let a1_ = (c.m.r32(sym::gMisResult)).wrapping_add(1388); F::UbScore_CalcRank(c, 1i32, a1_) };
        if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1356); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            { let v_ = { let a0_ = { let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1384); (c.m.r32(a_) as i32) }; let a2_ = (c.m.r32(sym::gMisResult)).wrapping_add(1388); F::UbScore_SaveMissionBest(c, a0_, l_total_3, a2_) }; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1364); c.m.w32(a_, (v_) as u32); };
            if (({ let a0_ = { let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1384); (c.m.r32(a_) as i32) }; let a1_ = (c.m.r32(sym::gMisResult)).wrapping_add(1368); F::MisResult_MarkCleared(c, a0_, a1_) }) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1368); (c.m.r32(a_) as i32) }) == (30i32)) as i32)) != 0) {
                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1372); c.m.w32(a_, (v_) as u32); };
                    { let r_ = 1i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(520); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1368); (c.m.r32(a_) as i32) }) == (50i32)) as i32)) != 0) {
                        { let v_ = 2i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1372); c.m.w32(a_, (v_) as u32); };
                        { let v_ = F::UbScore_GetRewardItem(c, 1i32); let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1376); c.m.w32(a_, (v_) as u32); };
                        { let a0_ = { let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1376); (c.m.r32(a_) as i32) }; F::Save_AddItem(c, a0_) };
                    } else {
                        if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1368); (c.m.r32(a_) as i32) }) == (100i32)) as i32)) != 0) {
                            { let v_ = 1i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1360); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 3i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1372); c.m.w32(a_, (v_) as u32); };
                            { let v_ = F::UbScore_GetRewardItem(c, 2i32); let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1376); c.m.w32(a_, (v_) as u32); };
                            { let a0_ = { let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1376); (c.m.r32(a_) as i32) }; F::Save_AddItem(c, a0_) };
                        }
                    }
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "MisResult_Init", &PROBE_N, &[], true); }
    r_
}

pub fn MisResult_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MisResult_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    F::IconWin_Term(c);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1008)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Destroy(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gMisResult)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gMisResult); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gMisResult; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "MisResult_Term", &PROBE_N, &[], true); }
    r_
}

pub fn MisResult_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MisResult_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_flash_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_mission_3: i32 = 0;
    let r_ = 'f: {
        { let a0_ = { let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1052); c.m.r32(a_) }; F::Sprite_DrawPicture(c, a0_, 0i32, 0i32, 128i32) };
        l_flash_1 = ((c.m.r32(sym::gMisResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        F::Flash_FindLabel(c, l_flash_1, sym::menu_p_b__str0, sym::menu_p_b__str1, fp);
        { let a2_ = (c.m.r32(sym::gMisResult)).wrapping_add(1340); F::FlashAnim_Blink(c, l_flash_1, fp, a2_, 0i32) };
        F::Flash_FindLabel(c, l_flash_1, sym::menu_p_b__str2, sym::menu_p_b__str3, fp);
        { let a2_ = (c.m.r32(sym::gMisResult)).wrapping_add(1344); F::FlashAnim_Talk(c, l_flash_1, fp, a2_, 0i32) };
        l_mission_3 = (({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1596); (c.m.r32(a_) as i32) }).wrapping_mul(5i32)).wrapping_add({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1600); (c.m.r32(a_) as i32) });
        F::Flash_FindLabel(c, l_flash_1, sym::menu_p_b__str4, sym::menu_p_b__str5, fp);
        { let a5_ = ((c.m.r32(sym::gMisResult)).wrapping_add(24)).wrapping_add((((3i32) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, (l_mission_3).wrapping_add(216i32), a5_) };
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_p_b__str6, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_p_b__str7, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_p_b__str8, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1352); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
        } else {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
        }
        { let v_ = 32i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_p_b__str9, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        if ((((((((({ let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1388)).wrapping_add(656); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1352); (c.m.r32(a_) as i32) }) < ({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1348); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
        } else {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
        }
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < ({ let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1388)).wrapping_add(652); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_p_b__str10, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                    { let a5_ = { let a_ = (((c.m.r32(sym::gMisResult)).wrapping_add(1388)).wrapping_add(16)).wrapping_add((((l_i_2) as u32)).wrapping_mul(12)); (c.m.r32(a_) as i32) }; F::Num_DrawChild(c, l_flash_1, fp.wrapping_add(28), sym::menu_p_b__str11, 0i32, 5i32, a5_, 32i32, 32i32, 0i32, 0i32) };
                    { let a5_ = ({ let a_ = ((((c.m.r32(sym::gMisResult)).wrapping_add(1388)).wrapping_add(16)).wrapping_add((((l_i_2) as u32)).wrapping_mul(12))).wrapping_add(8); (c.m.r32(a_) as i32) }).wrapping_mul(100i32); F::Num_DrawChild(c, l_flash_1, fp.wrapping_add(28), sym::menu_p_b__str12, 0i32, 6i32, a5_, 32i32, 32i32, 0i32, 0i32) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        {
            l_i_2 = 0i32;
            'l3: loop {
                if !(((((l_i_2) < (3i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_p_b__str13, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(28), fp);
                    if (((((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1352); (c.m.r32(a_) as i32) }).wrapping_mul(3i32)).wrapping_add(l_i_2)).wrapping_sub(3i32)) >= ({ let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1388)).wrapping_add(656); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    } else {
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                        F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_p_b__str14, fp);
                        { let a4_ = ({ let a_ = (((c.m.r32(sym::gMisResult)).wrapping_add(1388)).wrapping_add(64)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gMisResult)).wrapping_add(1388)).wrapping_add(640)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(12)); (c.m.r32(a_) as i32) }).wrapping_add(32i32); let a5_ = ((c.m.r32(sym::gMisResult)).wrapping_add(24)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, a4_, a5_) };
                        { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_p_b__str15, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                        { let a5_ = ({ let a_ = ((((c.m.r32(sym::gMisResult)).wrapping_add(1388)).wrapping_add(64)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gMisResult)).wrapping_add(1388)).wrapping_add(640)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(12))).wrapping_add(8); (c.m.r32(a_) as i32) }).wrapping_mul(100i32); F::Num_DrawChild(c, l_flash_1, fp.wrapping_add(28), sym::menu_p_b__str16, 0i32, 5i32, a5_, 32i32, 32i32, 0i32, 0i32) };
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
                    { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_p_b__str17, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_p_b__str18, fp);
                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_p_b__str19, fp);
                    { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let a4_ = (({ let a_ = (((c.m.r32(sym::gMisResult)).wrapping_add(1388)).wrapping_add(660)).wrapping_add(4); (c.m.r16(a_) as i16) }) as i32); F::Num_Draw(c, l_flash_1, sym::menu_p_b__str20, 4i32, 2i32, a4_, 32i32, 32i32, 1i32) };
        { let a4_ = (({ let a_ = (((c.m.r32(sym::gMisResult)).wrapping_add(1388)).wrapping_add(660)).wrapping_add(6); (c.m.r16(a_) as i16) }) as i32); F::Num_Draw(c, l_flash_1, sym::menu_p_b__str21, 2i32, 2i32, a4_, 32i32, 32i32, 1i32) };
        { let a4_ = (({ let a_ = (((c.m.r32(sym::gMisResult)).wrapping_add(1388)).wrapping_add(660)).wrapping_add(8); (c.m.r16(a_) as i16) }) as i32); F::Num_Draw(c, l_flash_1, sym::menu_p_b__str22, 0i32, 2i32, a4_, 32i32, 32i32, 1i32) };
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 160i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_p_b__str23, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 192i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_p_b__str24, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        { let a4_ = { let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1388)).wrapping_add(8); (c.m.r32(a_) as i32) }; F::Num_Draw(c, l_flash_1, sym::menu_p_b__str25, 0i32, 6i32, a4_, 32i32, 32i32, 0i32) };
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 96i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_p_b__str26, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        { let a4_ = { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); (c.m.r32(a_) as i32) }; F::Num_Draw(c, l_flash_1, sym::menu_p_b__str27, 0i32, 9i32, a4_, 32i32, 32i32, 0i32) };
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_p_b__str28, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        { let a4_ = ({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1388); (c.m.r32(a_) as i32) }).wrapping_mul(100i32); F::Num_Draw(c, l_flash_1, sym::menu_p_b__str29, 0i32, 9i32, a4_, 32i32, 32i32, 0i32) };
        { let v_ = ({ let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1388)).wrapping_add(676); (c.m.r32(a_) as i32) }).wrapping_shl((6i32) as u32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_p_b__str30, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_p_b__str31, fp);
        if (({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1364); (c.m.r32(a_) as i32) }) != 0) {
            { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
            F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
        } else {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
        }
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_p_b__str32, fp);
        F::Flash_ClipSetCallbackA(c, l_flash_1, fp, fid::MisResult_ClipScissorOn, ((0i32) as u32));
        F::Flash_ClipSetCallbackB(c, l_flash_1, fp, fid::MisResult_ClipScissorOff, ((0i32) as u32));
        if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1372); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
            F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_p_b__str33, fp);
            { let a4_ = ({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1372); (c.m.r32(a_) as i32) }).wrapping_add(15i32); let a5_ = ((c.m.r32(sym::gMisResult)).wrapping_add(24)).wrapping_add((((4i32) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, a4_, a5_) };
            's7: {
                let sv4: i32 = { let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1372); (c.m.r32(a_) as i32) };
                match sv4 {
                    1i32 => {
                        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_p_b__str34, fp);
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_p_b__str35, fp);
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_p_b__str36, fp);
                        { let a5_ = ((c.m.r32(sym::gMisResult)).wrapping_add(24)).wrapping_add((((5i32) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, 19i32, a5_) };
                        break 's7;
                    }
                    2i32 | 3i32 => {
                        F::Flash_FindLabel(c, l_flash_1, sym::menu_p_b__str37, sym::menu_p_b__str38, fp);
                        { let a4_ = { let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1376); (c.m.r32(a_) as i32) }; let a5_ = (c.m.r32(sym::gMisResult)).wrapping_add(864); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, a4_, a5_) };
                        break 's7;
                    }
                    _ => {
                    }
                }
            }
        }
        {
            l_i_2 = 0i32;
            'l8: loop {
                if !(((((l_i_2) < (1i32)) as i32)) != 0) { break 'l8; }
                'c9: {
                    { let a0_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1008)).wrapping_add((((l_i_2) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        F::IconWin_Draw(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "MisResult_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn MisResult_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MisResult_Update", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_flash_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        l_flash_1 = ((c.m.r32(sym::gMisResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
            break 'f;
        }
        if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1304); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            { let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1304); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
        }
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (1i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a0_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1008)).wrapping_add((((l_i_2) as u32)).wrapping_mul(44)); F::Flash_Advance(c, a0_) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        if (((({ let a_ = (((c.m.r32(sym::gMisResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1316); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                { let v_ = 1i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(2076); c.m.w32(a_, (v_) as u32); };
            }
        }
        if (((({ let a_ = (((c.m.r32(sym::gMisResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((2i32) as u32)))) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1316); (c.m.r32(a_) as i32) }) == (4i32)) as i32)) != 0) {
                F::Flash_GotoLabel(c, l_flash_1, sym::menu_p_b__str39, ((((1i32) as i64 as u64)) as u8));
                { let v_ = 5i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1316); c.m.w32(a_, (v_) as u32); };
            }
        }
        if (((({ let a_ = (((c.m.r32(sym::gMisResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((4i32) as u32)))) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1316); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) {
                { let v_ = 1i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(2068); c.m.w32(a_, (v_) as u32); };
            }
            { let a0_ = (c.m.r32(sym::gMisResult)).wrapping_add(1388); let a1_ = { let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1352); (c.m.r32(a_) as i32) }; F::UbScore_SetPage(c, a0_, a1_) };
        }
        if (((({ let a_ = (((c.m.r32(sym::gMisResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((8i32) as u32)))) != 0) {
            's3: {
                let sv3: i32 = { let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1316); (c.m.r32(a_) as i32) };
                match sv3 {
                    0i32 => {
                        if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1356); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                            F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_p_b__str40, fp);
                            F::Flash_ClipGotoLabel(c, l_flash_1, fp, sym::menu_p_b__str41);
                            if (({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1360); (c.m.r32(a_) as i32) }) != 0) {
                                { let v_ = 11i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1320); c.m.w32(a_, (v_) as u32); };
                            } else {
                                { let v_ = 1i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1320); c.m.w32(a_, (v_) as u32); };
                            }
                        } else {
                            F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_p_b__str42, fp);
                            F::Flash_ClipGotoLabel(c, l_flash_1, fp, sym::menu_p_b__str43);
                            { let v_ = 6i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1320); c.m.w32(a_, (v_) as u32); };
                        }
                        break 's3;
                    }
                    3i32 => {
                        F::Flash_GotoLabel(c, l_flash_1, sym::menu_p_b__str44, ((((1i32) as i64 as u64)) as u8));
                        break 's3;
                    }
                    4i32 => {
                        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_p_b__str45, fp);
                        F::Flash_ClipGotoLabel(c, l_flash_1, fp, sym::menu_p_b__str46);
                        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_p_b__str47, fp);
                        F::Flash_ClipGotoLabel(c, l_flash_1, fp, sym::menu_p_b__str48);
                        break 's3;
                    }
                    5i32 => {
                        F::Flash_GotoLabel(c, l_flash_1, sym::menu_p_b__str49, ((((1i32) as i64 as u64)) as u8));
                        { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((2i32) as u32)) as u64), ((6i32) as i64 as u64)])) as i32); };
                        break 's3;
                    }
                    _ => {
                    }
                }
            }
        }
        if (({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1380); c.m.r8(a_) }) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1304); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1352); (c.m.r32(a_) as i32) }) < ({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1348); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                    if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1352); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                        F::Flash_GotoLabel(c, l_flash_1, sym::menu_p_b__str50, ((((1i32) as i64 as u64)) as u8));
                    } else {
                        F::Flash_GotoLabel(c, l_flash_1, sym::menu_p_b__str51, ((((1i32) as i64 as u64)) as u8));
                    }
                    { let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1352); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1352); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                        F::Flash_GotoLabel(c, l_flash_1, sym::menu_p_b__str52, ((((1i32) as i64 as u64)) as u8));
                    } else {
                        F::Flash_GotoLabel(c, l_flash_1, sym::menu_p_b__str53, ((((1i32) as i64 as u64)) as u8));
                    }
                    { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((2i32) as u32)) as u64), ((19i32) as i64 as u64)])) as i32); };
                    if ((((({ let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1388)).wrapping_add(676); (c.m.r32(a_) as i32) }) == (4i32)) as i32)) != 0) {
                        { let v_ = 16i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1320); c.m.w32(a_, (v_) as u32); };
                    } else {
                        if (((((F::Rand_Range(c, ((2i32) as u32))) != (((0i32) as u32))) as i32)) != 0) {
                            { let v_ = 17i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1320); c.m.w32(a_, (v_) as u32); };
                        } else {
                            { let v_ = 18i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1320); c.m.w32(a_, (v_) as u32); };
                        }
                    }
                }
                { let v_ = ((0i32) as u8); let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1380); c.m.w8(a_, v_); };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "MisResult_Update", &PROBE_N, &[], true); }
    r_
}

pub fn MisResult_UpdateVoice(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MisResult_UpdateVoice", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1328); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1320); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            break 'f;
        }
        if ((((((((((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1332); (c.m.r32(a_) as i32) }) != ((1i32).wrapping_neg())) as i32)) != 0) && (((((F::Voice_GetStat(c, 0i32)) != (5i32)) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1320); (c.m.r32(a_) as i32) }) == ({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1324); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((1i32) as i64 as u64)])) as i32); };
                F::Voice_StopWithLip(c);
                { let v_ = 1i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1328); c.m.w32(a_, (v_) as u32); };
            }
            break 'f;
        }
    } else {
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1328); c.m.w32(a_, (v_) as u32); };
    }
    's1: {
        let sv1: i32 = { let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1320); (c.m.r32(a_) as i32) };
        match sv1 {
            1i32 => {
                { let v_ = 56i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1332); c.m.w32(a_, (v_) as u32); };
                { let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1320); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            6i32 => {
                { let v_ = 57i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1332); c.m.w32(a_, (v_) as u32); };
                { let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1320); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            16i32 => {
                { let v_ = 58i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1332); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            17i32 => {
                { let v_ = 59i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1332); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            18i32 => {
                { let v_ = 60i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1332); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            19i32 => {
                { let v_ = 61i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1332); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            11i32 => {
                { let v_ = 62i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1332); c.m.w32(a_, (v_) as u32); };
                { let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1320); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 | 7i32 | 12i32 => {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1320); c.m.w32(a_, (v_) as u32); };
                { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1332); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1320); (c.m.r32(a_) as i32) }) >= (16i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1320); c.m.w32(a_, (v_) as u32); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1332); (c.m.r32(a_) as i32) }) != ((1i32).wrapping_neg())) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(16); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1332); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34661i32, a2_) };
        { let v_ = { let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1320); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1324); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "MisResult_UpdateVoice", &PROBE_N, &[], true); }
    r_
}

pub fn MisResult_Input(c: &mut Ctx, mut p_result_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MisResult_Input", &PROBE_N, &[(p_result_1) as u32], false); }
    let mut l_up_2: i32 = 0;
    let mut l_down_3: i32 = 0;
    let mut l_confirm_4: i32 = 0;
    let mut l_isBonus_5: i32 = 0;
    let mut l_n_6: i32 = 0;
    let mut l_i_7: i32 = 0;
    let mut l_index_8: i32 = 0;
    let mut l_isBonus_9: i32 = 0;
    let mut sv_10: i32 = 0;
    let mut sv_11: i32 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
    'fl1: loop {
        match bb_ {
            0 => {
                l_up_2 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) as i32);
                l_down_3 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) as i32);
                l_confirm_4 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) as i32);
                if ((((((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1316); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) && (({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(2076); (c.m.r32(a_) as i32) }) != 0)) as i32)) != 0) { bb_ = 1; continue 'fl1; } else { bb_ = 2; continue 'fl1; }
            }
            1 => {
                if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1356); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) { bb_ = 3; continue 'fl1; } else { bb_ = 5; continue 'fl1; }
            }
            2 => {
                if ((((!(((({ let a_ = (((c.m.r32(sym::gMisResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) { bb_ = 6; continue 'fl1; } else { bb_ = 7; continue 'fl1; }
            }
            3 => {
                { let r_ = 1i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1244); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = 2i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1244); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 15i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1304); c.m.w32(a_, (v_) as u32); };
                { let v_ = 6i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1316); c.m.w32(a_, (v_) as u32); };
                bb_ = 4; continue 'fl1;
            }
            4 => {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(2076); c.m.w32(a_, (v_) as u32); };
                bb_ = 2; continue 'fl1;
            }
            5 => {
                { let a0_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_p_b__str54, ((((1i32) as i64 as u64)) as u8)) };
                { let v_ = 1i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1316); c.m.w32(a_, (v_) as u32); };
                bb_ = 4; continue 'fl1;
            }
            6 => {
                break 'f;
            }
            7 => {
                if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) { bb_ = 9; continue 'fl1; } else { bb_ = 10; continue 'fl1; }
            }
            8 => {
                bb_ = 7; continue 'fl1;
            }
            9 => {
                break 'f;
            }
            10 => {
                if ((((!(((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1244); (c.m.r32(a_) as i32) }) & (4i32))) != 0)) as i32)) != 0) { bb_ = 12; continue 'fl1; } else { bb_ = 13; continue 'fl1; }
            }
            11 => {
                bb_ = 10; continue 'fl1;
            }
            12 => {
                { let r_ = 4i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1244); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                bb_ = 13; continue 'fl1;
            }
            13 => {
                sv_10 = { let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1316); (c.m.r32(a_) as i32) };
                bb_ = match sv_10 { 1i32 => 16, 2i32 => 67, 3i32 => 90, 4i32 => 143, 5i32 => 147, 6i32 => 151, _ => 14, }; continue 'fl1;
            }
            14 => {
                break 'fl1;
            }
            15 => {
                bb_ = 16; continue 'fl1;
            }
            16 => {
                if (({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(2068); (c.m.r32(a_) as i32) }) != 0) { bb_ = 17; continue 'fl1; } else { bb_ = 18; continue 'fl1; }
            }
            17 => {
                l_isBonus_5 = ((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1352); (c.m.r32(a_) as i32) }) != (0i32)) as i32);
                if (({ let a0_ = (c.m.r32(sym::gMisResult)).wrapping_add(1388); let a2_ = ({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(2072); (c.m.r32(a_) as i32) }).wrapping_sub((l_isBonus_5).wrapping_mul({ let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1388)).wrapping_add(652); (c.m.r32(a_) as i32) })); F::UbScore_CountLine(c, a0_, l_isBonus_5, a2_, 15i32) }) != 0) { bb_ = 19; continue 'fl1; } else { bb_ = 20; continue 'fl1; }
            }
            18 => {
                if ((l_confirm_4) != 0) { bb_ = 35; continue 'fl1; } else { bb_ = 36; continue 'fl1; }
            }
            19 => {
                { let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(2072); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                if ((l_isBonus_5) != 0) { bb_ = 21; continue 'fl1; } else { bb_ = 23; continue 'fl1; }
            }
            20 => {
                bb_ = 18; continue 'fl1;
            }
            21 => {
                l_n_6 = 3i32;
                if (((((rt::rem_i32({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(2072); (c.m.r32(a_) as i32) }, l_n_6)) != (0i32)) as i32)) != 0) { bb_ = 24; continue 'fl1; } else { bb_ = 25; continue 'fl1; }
            }
            22 => {
                bb_ = 31; continue 'fl1;
            }
            23 => {
                if (((((rt::rem_i32({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(2072); (c.m.r32(a_) as i32) }, { let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1388)).wrapping_add(652); (c.m.r32(a_) as i32) })) != (0i32)) as i32)) != 0) { bb_ = 28; continue 'fl1; } else { bb_ = 29; continue 'fl1; }
            }
            24 => {
                bb_ = 26; continue 'fl1;
            }
            25 => {
                bb_ = 22; continue 'fl1;
            }
            26 => {
                if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(2072); (c.m.r32(a_) as i32) }) == (({ let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1388)).wrapping_add(656); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1388)).wrapping_add(652); (c.m.r32(a_) as i32) }))) as i32)) != 0) { bb_ = 33; continue 'fl1; } else { bb_ = 34; continue 'fl1; }
            }
            27 => {
                bb_ = 25; continue 'fl1;
            }
            28 => {
                bb_ = 26; continue 'fl1;
            }
            29 => {
                bb_ = 22; continue 'fl1;
            }
            30 => {
                bb_ = 29; continue 'fl1;
            }
            31 => {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(2068); c.m.w32(a_, (v_) as u32); };
                { let v_ = ((1i32) as u8); let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1380); c.m.w8(a_, v_); };
                bb_ = 34; continue 'fl1;
            }
            32 => {
                bb_ = 26; continue 'fl1;
            }
            33 => {
                bb_ = 31; continue 'fl1;
            }
            34 => {
                bb_ = 20; continue 'fl1;
            }
            35 => {
                l_i_7 = 0i32;
                bb_ = 37; continue 'fl1;
            }
            36 => {
                if (({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1380); c.m.r8(a_) }) != 0) { bb_ = 47; continue 'fl1; } else { bb_ = 48; continue 'fl1; }
            }
            37 => {
                if (((((l_i_7) < (({ let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1388)).wrapping_add(656); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1388)).wrapping_add(652); (c.m.r32(a_) as i32) }))) as i32)) != 0) { bb_ = 38; continue 'fl1; } else { bb_ = 40; continue 'fl1; }
            }
            38 => {
                l_index_8 = 0i32;
                l_isBonus_9 = 0i32;
                if (((((l_i_7) < ({ let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1388)).wrapping_add(652); (c.m.r32(a_) as i32) })) as i32)) != 0) { bb_ = 41; continue 'fl1; } else { bb_ = 43; continue 'fl1; }
            }
            39 => {
                { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
                bb_ = 37; continue 'fl1;
            }
            40 => {
                { let v_ = ((1i32) as u8); let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1380); c.m.w8(a_, v_); };
                { let v_ = { let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1348); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1352); c.m.w32(a_, (v_) as u32); };
                { let v_ = ({ let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1388)).wrapping_add(656); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1388)).wrapping_add(652); (c.m.r32(a_) as i32) }); let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(2072); c.m.w32(a_, (v_) as u32); };
                { let v_ = { let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1348); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1352); c.m.w32(a_, (v_) as u32); };
                { let a0_ = (c.m.r32(sym::gMisResult)).wrapping_add(1388); let a1_ = { let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1352); (c.m.r32(a_) as i32) }; F::UbScore_SetPage(c, a0_, a1_) };
                bb_ = 36; continue 'fl1;
            }
            41 => {
                l_isBonus_9 = 0i32;
                l_index_8 = l_i_7;
                bb_ = 42; continue 'fl1;
            }
            42 => {
                bb_ = 44; continue 'fl1;
            }
            43 => {
                l_index_8 = (l_i_7).wrapping_sub({ let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1388)).wrapping_add(652); (c.m.r32(a_) as i32) });
                l_isBonus_9 = 1i32;
                bb_ = 42; continue 'fl1;
            }
            44 => {
                if ((((!(({ let a0_ = (c.m.r32(sym::gMisResult)).wrapping_add(1388); F::UbScore_CountLine(c, a0_, l_isBonus_9, l_index_8, 9999i32) }) != 0)) as i32)) != 0) { bb_ = 45; continue 'fl1; } else { bb_ = 46; continue 'fl1; }
            }
            45 => {
                bb_ = 44; continue 'fl1;
            }
            46 => {
                bb_ = 39; continue 'fl1;
            }
            47 => {
                if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1352); (c.m.r32(a_) as i32) }) < ({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1348); (c.m.r32(a_) as i32) })) as i32)) != 0) { bb_ = 49; continue 'fl1; } else { bb_ = 51; continue 'fl1; }
            }
            48 => {
                if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(2072); (c.m.r32(a_) as i32) }) == (({ let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1388)).wrapping_add(656); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1388)).wrapping_add(652); (c.m.r32(a_) as i32) }))) as i32)) != 0) { bb_ = 64; continue 'fl1; } else { bb_ = 65; continue 'fl1; }
            }
            49 => {
                if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1352); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) { bb_ = 52; continue 'fl1; } else { bb_ = 54; continue 'fl1; }
            }
            50 => {
                { let v_ = ((0i32) as u8); let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1380); c.m.w8(a_, v_); };
                bb_ = 48; continue 'fl1;
            }
            51 => {
                if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1352); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) { bb_ = 55; continue 'fl1; } else { bb_ = 57; continue 'fl1; }
            }
            52 => {
                { let a0_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_p_b__str55, ((((1i32) as i64 as u64)) as u8)) };
                bb_ = 53; continue 'fl1;
            }
            53 => {
                { let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1352); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                bb_ = 50; continue 'fl1;
            }
            54 => {
                { let a0_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_p_b__str56, ((((1i32) as i64 as u64)) as u8)) };
                bb_ = 53; continue 'fl1;
            }
            55 => {
                { let a0_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_p_b__str57, ((((1i32) as i64 as u64)) as u8)) };
                bb_ = 56; continue 'fl1;
            }
            56 => {
                { let _ = 0i32; };
                if ((((({ let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1388)).wrapping_add(676); (c.m.r32(a_) as i32) }) == (4i32)) as i32)) != 0) { bb_ = 58; continue 'fl1; } else { bb_ = 60; continue 'fl1; }
            }
            57 => {
                { let a0_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_p_b__str58, ((((1i32) as i64 as u64)) as u8)) };
                bb_ = 56; continue 'fl1;
            }
            58 => {
                { let v_ = 16i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1320); c.m.w32(a_, (v_) as u32); };
                bb_ = 59; continue 'fl1;
            }
            59 => {
                bb_ = 50; continue 'fl1;
            }
            60 => {
                if (((((F::Rand_Range(c, ((2i32) as u32))) != (((0i32) as u32))) as i32)) != 0) { bb_ = 61; continue 'fl1; } else { bb_ = 63; continue 'fl1; }
            }
            61 => {
                { let v_ = 17i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1320); c.m.w32(a_, (v_) as u32); };
                bb_ = 62; continue 'fl1;
            }
            62 => {
                bb_ = 59; continue 'fl1;
            }
            63 => {
                { let v_ = 18i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1320); c.m.w32(a_, (v_) as u32); };
                bb_ = 62; continue 'fl1;
            }
            64 => {
                { let v_ = 2i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1316); c.m.w32(a_, (v_) as u32); };
                bb_ = 65; continue 'fl1;
            }
            65 => {
                bb_ = 14; continue 'fl1;
            }
            66 => {
                bb_ = 67; continue 'fl1;
            }
            67 => {
                if ((l_up_2) != 0) { bb_ = 68; continue 'fl1; } else { bb_ = 70; continue 'fl1; }
            }
            68 => {
                if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1352); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) { bb_ = 71; continue 'fl1; } else { bb_ = 72; continue 'fl1; }
            }
            69 => {
                bb_ = 14; continue 'fl1;
            }
            70 => {
                if ((l_down_3) != 0) { bb_ = 76; continue 'fl1; } else { bb_ = 78; continue 'fl1; }
            }
            71 => {
                { let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1352); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1352); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) { bb_ = 73; continue 'fl1; } else { bb_ = 75; continue 'fl1; }
            }
            72 => {
                bb_ = 69; continue 'fl1;
            }
            73 => {
                { let a0_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_p_b__str59, ((((1i32) as i64 as u64)) as u8)) };
                bb_ = 74; continue 'fl1;
            }
            74 => {
                { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                bb_ = 72; continue 'fl1;
            }
            75 => {
                { let a0_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_p_b__str60, ((((1i32) as i64 as u64)) as u8)) };
                bb_ = 74; continue 'fl1;
            }
            76 => {
                if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1352); (c.m.r32(a_) as i32) }) < ({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1348); (c.m.r32(a_) as i32) })) as i32)) != 0) { bb_ = 79; continue 'fl1; } else { bb_ = 80; continue 'fl1; }
            }
            77 => {
                bb_ = 69; continue 'fl1;
            }
            78 => {
                if ((l_confirm_4) != 0) { bb_ = 84; continue 'fl1; } else { bb_ = 85; continue 'fl1; }
            }
            79 => {
                { let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1352); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1352); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) { bb_ = 81; continue 'fl1; } else { bb_ = 83; continue 'fl1; }
            }
            80 => {
                bb_ = 77; continue 'fl1;
            }
            81 => {
                { let a0_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_p_b__str61, ((((1i32) as i64 as u64)) as u8)) };
                bb_ = 82; continue 'fl1;
            }
            82 => {
                { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                bb_ = 80; continue 'fl1;
            }
            83 => {
                { let a0_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_p_b__str62, ((((1i32) as i64 as u64)) as u8)) };
                bb_ = 82; continue 'fl1;
            }
            84 => {
                { let v_ = 3i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1316); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(2072); c.m.w32(a_, (v_) as u32); };
                { let v_ = 60i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1304); c.m.w32(a_, (v_) as u32); };
                if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1352); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) { bb_ = 86; continue 'fl1; } else { bb_ = 88; continue 'fl1; }
            }
            85 => {
                bb_ = 77; continue 'fl1;
            }
            86 => {
                { let a0_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_p_b__str63, ((((1i32) as i64 as u64)) as u8)) };
                bb_ = 87; continue 'fl1;
            }
            87 => {
                { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((1i32) as i64 as u64)])) as i32); };
                bb_ = 85; continue 'fl1;
            }
            88 => {
                { let a0_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_p_b__str64, ((((1i32) as i64 as u64)) as u8)) };
                bb_ = 87; continue 'fl1;
            }
            89 => {
                bb_ = 90; continue 'fl1;
            }
            90 => {
                sv_11 = { let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(2072); (c.m.r32(a_) as i32) };
                bb_ = match sv_11 { 0i32 => 93, 1i32 => 97, 2i32 => 101, 3i32 => 105, 4i32 => 111, _ => 91, }; continue 'fl1;
            }
            91 => {
                if ((((((((((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(2072); (c.m.r32(a_) as i32) }) >= (1i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(2072); (c.m.r32(a_) as i32) }) <= (3i32)) as i32)) != 0)) as i32)) != 0) && ((l_confirm_4) != 0)) as i32)) != 0) { bb_ = 124; continue 'fl1; } else { bb_ = 125; continue 'fl1; }
            }
            92 => {
                bb_ = 93; continue 'fl1;
            }
            93 => {
                if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1304); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) { bb_ = 94; continue 'fl1; } else { bb_ = 95; continue 'fl1; }
            }
            94 => {
                { let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(2072); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let a0_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::UbScore_PlateGoto(c, a0_, 0i32, 1i32) };
                { let v_ = { let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1388); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1388)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = 19i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1320); c.m.w32(a_, (v_) as u32); };
                bb_ = 95; continue 'fl1;
            }
            95 => {
                bb_ = 91; continue 'fl1;
            }
            96 => {
                bb_ = 97; continue 'fl1;
            }
            97 => {
                if (({ let a0_ = (c.m.r32(sym::gMisResult)).wrapping_add(1388); F::UbScore_ConvertStep(c, a0_, 64i32) }) != 0) { bb_ = 98; continue 'fl1; } else { bb_ = 99; continue 'fl1; }
            }
            98 => {
                { let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(2072); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 15i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1304); c.m.w32(a_, (v_) as u32); };
                { let a0_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::UbScore_PlateGoto(c, a0_, 0i32, 0i32) };
                { let v_ = { let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1388)).wrapping_add(4); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1388)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                bb_ = 99; continue 'fl1;
            }
            99 => {
                bb_ = 91; continue 'fl1;
            }
            100 => {
                bb_ = 101; continue 'fl1;
            }
            101 => {
                if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1304); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) { bb_ = 102; continue 'fl1; } else { bb_ = 103; continue 'fl1; }
            }
            102 => {
                { let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(2072); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let a0_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::UbScore_PlateGoto(c, a0_, 1i32, 1i32) };
                bb_ = 103; continue 'fl1;
            }
            103 => {
                bb_ = 91; continue 'fl1;
            }
            104 => {
                bb_ = 105; continue 'fl1;
            }
            105 => {
                if (({ let a0_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1388)).wrapping_add(8); let a1_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); F::UbScore_Transfer(c, a0_, a1_, 64i32, F32(0x3f800000)) }) != 0) { bb_ = 106; continue 'fl1; } else { bb_ = 107; continue 'fl1; }
            }
            106 => {
                { let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(2072); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let a0_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::UbScore_PlateGoto(c, a0_, 1i32, 0i32) };
                bb_ = 107; continue 'fl1;
            }
            107 => {
                if ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); (c.m.r32(a_) as i32) }) > (9999999i32)) as i32)) != 0) { bb_ = 108; continue 'fl1; } else { bb_ = 109; continue 'fl1; }
            }
            108 => {
                { let v_ = 9999999i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); c.m.w32(a_, (v_) as u32); };
                bb_ = 109; continue 'fl1;
            }
            109 => {
                bb_ = 91; continue 'fl1;
            }
            110 => {
                bb_ = 111; continue 'fl1;
            }
            111 => {
                if ((l_confirm_4) != 0) { bb_ = 112; continue 'fl1; } else { bb_ = 113; continue 'fl1; }
            }
            112 => {
                { let a0_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_p_b__str65, ((((1i32) as i64 as u64)) as u8)) };
                if (({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1360); (c.m.r32(a_) as i32) }) != 0) { bb_ = 114; continue 'fl1; } else { bb_ = 116; continue 'fl1; }
            }
            113 => {
                bb_ = 91; continue 'fl1;
            }
            114 => {
                { let v_ = 4i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1316); c.m.w32(a_, (v_) as u32); };
                bb_ = 115; continue 'fl1;
            }
            115 => {
                { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((1i32) as i64 as u64)])) as i32); };
                bb_ = 113; continue 'fl1;
            }
            116 => {
                if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1372); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) { bb_ = 117; continue 'fl1; } else { bb_ = 119; continue 'fl1; }
            }
            117 => {
                { let v_ = 5i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1316); c.m.w32(a_, (v_) as u32); };
                if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1372); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) { bb_ = 120; continue 'fl1; } else { bb_ = 122; continue 'fl1; }
            }
            118 => {
                bb_ = 115; continue 'fl1;
            }
            119 => {
                { let r_ = 1i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1244); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = 2i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1244); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 15i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1304); c.m.w32(a_, (v_) as u32); };
                { let v_ = 6i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1316); c.m.w32(a_, (v_) as u32); };
                bb_ = 118; continue 'fl1;
            }
            120 => {
                { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((2i32) as u32)) as u64), ((7i32) as i64 as u64)])) as i32); };
                bb_ = 121; continue 'fl1;
            }
            121 => {
                bb_ = 118; continue 'fl1;
            }
            122 => {
                { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((2i32) as u32)) as u64), ((6i32) as i64 as u64)])) as i32); };
                bb_ = 121; continue 'fl1;
            }
            123 => {
                bb_ = 91; continue 'fl1;
            }
            124 => {
                if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(2072); (c.m.r32(a_) as i32) }) < (2i32)) as i32)) != 0) { bb_ = 126; continue 'fl1; } else { bb_ = 128; continue 'fl1; }
            }
            125 => {
                bb_ = 14; continue 'fl1;
            }
            126 => {
                bb_ = 129; continue 'fl1;
            }
            127 => {
                if ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); (c.m.r32(a_) as i32) }) > (9999999i32)) as i32)) != 0) { bb_ = 140; continue 'fl1; } else { bb_ = 141; continue 'fl1; }
            }
            128 => {
                if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(2072); (c.m.r32(a_) as i32) }) < (4i32)) as i32)) != 0) { bb_ = 135; continue 'fl1; } else { bb_ = 136; continue 'fl1; }
            }
            129 => {
                if ((((!(({ let a0_ = (c.m.r32(sym::gMisResult)).wrapping_add(1388); F::UbScore_ConvertStep(c, a0_, 9999999i32) }) != 0)) as i32)) != 0) { bb_ = 130; continue 'fl1; } else { bb_ = 131; continue 'fl1; }
            }
            130 => {
                bb_ = 129; continue 'fl1;
            }
            131 => {
                { let v_ = { let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1388)).wrapping_add(4); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1388)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                bb_ = 132; continue 'fl1;
            }
            132 => {
                if ((((!(({ let a0_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1388)).wrapping_add(8); let a1_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); F::UbScore_Transfer(c, a0_, a1_, 9999999i32, F32(0x3f800000)) }) != 0)) as i32)) != 0) { bb_ = 133; continue 'fl1; } else { bb_ = 134; continue 'fl1; }
            }
            133 => {
                bb_ = 132; continue 'fl1;
            }
            134 => {
                bb_ = 127; continue 'fl1;
            }
            135 => {
                bb_ = 137; continue 'fl1;
            }
            136 => {
                bb_ = 127; continue 'fl1;
            }
            137 => {
                if ((((!(({ let a0_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1388)).wrapping_add(8); let a1_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); F::UbScore_Transfer(c, a0_, a1_, 9999999i32, F32(0x3f800000)) }) != 0)) as i32)) != 0) { bb_ = 138; continue 'fl1; } else { bb_ = 139; continue 'fl1; }
            }
            138 => {
                bb_ = 137; continue 'fl1;
            }
            139 => {
                bb_ = 136; continue 'fl1;
            }
            140 => {
                { let v_ = 9999999i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); c.m.w32(a_, (v_) as u32); };
                bb_ = 141; continue 'fl1;
            }
            141 => {
                { let a0_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::UbScore_PlateGoto(c, a0_, 0i32, 0i32) };
                { let a0_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::UbScore_PlateGoto(c, a0_, 1i32, 0i32) };
                { let v_ = 4i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(2072); c.m.w32(a_, (v_) as u32); };
                bb_ = 125; continue 'fl1;
            }
            142 => {
                bb_ = 143; continue 'fl1;
            }
            143 => {
                if ((l_confirm_4) != 0) { bb_ = 144; continue 'fl1; } else { bb_ = 145; continue 'fl1; }
            }
            144 => {
                F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                { let v_ = 0i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
                { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((1i32) as i64 as u64)])) as i32); };
                bb_ = 145; continue 'fl1;
            }
            145 => {
                bb_ = 14; continue 'fl1;
            }
            146 => {
                bb_ = 147; continue 'fl1;
            }
            147 => {
                if ((l_confirm_4) != 0) { bb_ = 148; continue 'fl1; } else { bb_ = 149; continue 'fl1; }
            }
            148 => {
                { let a0_ = ((c.m.r32(sym::gMisResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_p_b__str66, ((((1i32) as i64 as u64)) as u8)) };
                { let r_ = 1i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1244); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = 2i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1244); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 15i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1304); c.m.w32(a_, (v_) as u32); };
                { let v_ = 6i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1316); c.m.w32(a_, (v_) as u32); };
                { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((1i32) as i64 as u64)])) as i32); };
                bb_ = 149; continue 'fl1;
            }
            149 => {
                bb_ = 14; continue 'fl1;
            }
            150 => {
                bb_ = 151; continue 'fl1;
            }
            151 => {
                bb_ = 14; continue 'fl1;
            }
            152 => {
                bb_ = 14; continue 'fl1;
            }
            _ => unreachable!(),
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "MisResult_Input", &PROBE_N, &[], true); }
    r_
}

pub fn MisResult_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MisResult_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = 1i32; c.m.w32(fp, (v_) as u32); }
        F::MisResult_Init(c, p_section_1);
        F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
        'l1: while ((1i32) != 0) {
            F::Gfx_BeginFrame(c);
            F::Pad_Update(c);
            F::Snd_Update(c);
            F::ColorFade_Update(c);
            F::MisResult_Update(c);
            F::MisResult_UpdateVoice(c);
            F::MisResult_Draw(c);
            F::ColorFade_Draw(c);
            F::Gfx_EndFrame(c, 1i32);
            F::Dma_Flush(c);
            F::File_Stub264D90(c);
            if ((F::ColorFade_IsInDone(c)) != 0) {
                if ((((((((!(((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1244); (c.m.r32(a_) as i32) }) & (8i32))) != 0)) as i32)) != 0) && (((({ let a_ = (((c.m.r32(sym::gMisResult)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
                    { let r_ = 8i32; let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1244); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
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
            if (((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1244); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1304); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                }
            } else {
                if ((((({ let a_ = (c.m.r32(sym::gMisResult)).wrapping_add(1320); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::MisResult_Input(c, fp);
                }
            }
        }
        F::MisResult_Term(c);
        F::Dma_ResetBuffers(c);
        break 'f (c.m.r32(fp) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "MisResult_Run", &PROBE_N, &[], true); }
    r_
}
