#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn SimTop_ShowStar(c: &mut Ctx, mut p_parent_1: u32, mut p_on_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimTop_ShowStar", &PROBE_N, &[(p_parent_1) as u32, (p_on_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_flash_3: u32 = 0;
    let r_ = 'f: {
        l_flash_3 = ((c.m.r32(sym::gSimTop)).wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        F::Flash_FindLabel(c, l_flash_3, p_parent_1, sym::menu_r_d__str0, fp);
        if ((p_on_2) != 0) {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
        } else {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SimTop_ShowStar", &PROBE_N, &[], true); }
    r_
}

pub fn SimTop_Init(c: &mut Ctx, mut p_section_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimTop_Init", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = F::Heap_Alloc(c, ((456u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gSimTop; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gSimTop); F::memset(c, a0_, 0i32, 456u32) }; };
        { let v_ = (c.m.r32(sym::gMenuArc3)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc3)).wrapping_add((((p_section_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = c.m.r32(sym::gSimTop); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gSimTop); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = c.m.r32(fp); let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(204); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((32i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((33i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((8i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((34i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((31i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((7i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((8i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((30i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((9i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((10i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((11i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((12i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((13i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((14i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((15i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((16i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((17i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((18i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((19i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((20i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((21i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((22i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((23i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((24i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((25i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((35i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((26i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((36i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((27i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((37i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((28i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((38i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((29i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((39i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(8); c.m.w32(a_, v_); };
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (165i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = ({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(8); c.m.r32(a_) }).wrapping_add(((((l_i_2).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let a0_ = ((c.m.r32(sym::gSimTop)).wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gSimTop)).wrapping_add(208); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gSimTop)).wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = c.m.r32(fp); F::IconWin_Init(c, a0_, a1_) };
        F::IconWin_Open(c);
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(16); c.m.w32(a_, v_); };
        { let a0_ = (c.m.r32(sym::gSimTop)).wrapping_add(20); let a1_ = { let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(12); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, ((0i32) as u32)) };
        { let a0_ = (c.m.r32(sym::gSimTop)).wrapping_add(20); F::TextBox_SetUnk50(c, a0_, 0i32) };
        { let a0_ = (c.m.r32(sym::gSimTop)).wrapping_add(20); F::TextBox_SetSpacing(c, a0_, 0i32, 4i32) };
        { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(416); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(440); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(444); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(412); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1612); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 100i32; let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        {
            l_i_2 = 0i32;
            'l3: loop {
                if !(((((l_i_2) < (3i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let v_ = (1i32).wrapping_neg(); let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(24)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = 0i32; let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
        { let v_ = 1i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1604); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1608); c.m.w32(a_, (v_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SimTop_Init", &PROBE_N, &[], true); }
    r_
}

pub fn SimTop_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimTop_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    F::IconWin_Term(c);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gSimTop)).wrapping_add(160)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Destroy(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gSimTop)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gSimTop); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gSimTop; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SimTop_Term", &PROBE_N, &[], true); }
    r_
}

pub fn SimTop_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimTop_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_flash_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_x_3: i32 = 0;
    let mut l_n_4: i32 = 0;
    let mut l_score_5: i32 = 0;
    let mut l_star_6: i32 = 0;
    let mut l_chara_7: i32 = 0;
    let mut l_face_8: u32 = 0;
    let r_ = 'f: {
        { let a0_ = { let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(204); c.m.r32(a_) }; F::Sprite_DrawPicture(c, a0_, 0i32, 0i32, 128i32) };
        l_flash_1 = ((c.m.r32(sym::gSimTop)).wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        F::Flash_FindLabel(c, l_flash_1, sym::menu_r_d__str1, sym::menu_r_d__str2, fp);
        { let a2_ = (c.m.r32(sym::gSimTop)).wrapping_add(420); F::FlashAnim_Blink(c, l_flash_1, fp, a2_, 0i32) };
        F::Flash_FindLabel(c, l_flash_1, sym::menu_r_d__str3, sym::menu_r_d__str4, fp);
        { let a2_ = (c.m.r32(sym::gSimTop)).wrapping_add(424); F::FlashAnim_Talk(c, l_flash_1, fp, a2_, 0i32) };
        l_x_3 = ({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(452); (c.m.r32(a_) as i32) }).wrapping_shl((6i32) as u32);
        { let v_ = l_x_3; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = (l_x_3).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_r_d__str5, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        { let r_ = 1i32; let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(452); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
        {
            l_i_2 = 1i32;
            'l1: loop {
                if !(((((l_i_2) < (3i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (l_i_2).wrapping_shl((5i32) as u32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((l_i_2).wrapping_shl((5i32) as u32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_r_d__str6, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_r_d__str7, fp);
                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_r_d__str8, fp);
                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_r_d__str9, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        if ((((({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(440); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
        } else {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
        }
        { let v_ = 32i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_r_d__str10, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        if ((((({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(440); (c.m.r32(a_) as i32) }) == (5i32)) as i32)) != 0) {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
        } else {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
        }
        {
            l_i_2 = 0i32;
            'l3: loop {
                if !(((((l_i_2) < (6i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_r_d__str11, &[((l_i_2) as i64 as u64)]); };
                    if (((((l_i_2) != (5i32)) as i32)) != 0) {
                        l_n_4 = (({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(440); (c.m.r32(a_) as i32) }).wrapping_add(l_i_2)).wrapping_add(1i32);
                    } else {
                        l_n_4 = ({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(444); (c.m.r32(a_) as i32) }).wrapping_add(1i32);
                    }
                    { let v_ = (rt::rem_i32(l_n_4, 4i32)).wrapping_shl((6i32) as u32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (rt::div_i32(l_n_4, 4i32)).wrapping_shl((6i32) as u32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((rt::rem_i32(l_n_4, 4i32)).wrapping_shl((6i32) as u32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((rt::div_i32(l_n_4, 4i32)).wrapping_shl((6i32) as u32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_r_d__str12, fp);
                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_r_d__str13, fp);
                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
                    if (((((l_i_2) != (5i32)) as i32)) != 0) {
                        l_chara_7 = { let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(528)).wrapping_add((((({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(440); (c.m.r32(a_) as i32) }).wrapping_add(l_i_2)) as u32)).wrapping_mul(12)); (c.m.r32(a_) as i32) };
                        l_face_8 = ({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(8); c.m.r32(a_) }).wrapping_add(((((l_chara_7).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
                        { let v_ = c.m.r32((l_face_8).wrapping_add(16)); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add(((((c.m.r32((sym::gSimTopFaceTex).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    } else {
                        l_chara_7 = { let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(528)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(444); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(12)); (c.m.r32(a_) as i32) };
                        l_face_8 = ({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(8); c.m.r32(a_) }).wrapping_add(((((l_chara_7).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
                        { let v_ = c.m.r32((l_face_8).wrapping_add(16)); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(208)).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    }
                    if (((((l_i_2) != (5i32)) as i32)) != 0) {
                        l_score_5 = { let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(528)).wrapping_add((((({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(440); (c.m.r32(a_) as i32) }).wrapping_add(l_i_2)) as u32)).wrapping_mul(12))).wrapping_add(4); (c.m.r32(a_) as i32) };
                    } else {
                        l_score_5 = { let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(528)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(444); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(12))).wrapping_add(4); (c.m.r32(a_) as i32) };
                    }
                    F::Num_DrawChild(c, l_flash_1, fp.wrapping_add(28), sym::menu_r_d__str14, 0i32, 9i32, (l_score_5).wrapping_mul(100i32), 32i32, 32i32, 0i32, 0i32);
                    if (((((l_i_2) != (5i32)) as i32)) != 0) {
                        l_star_6 = (({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(528)).wrapping_add((((({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(440); (c.m.r32(a_) as i32) }).wrapping_add(l_i_2)) as u32)).wrapping_mul(12))).wrapping_add(8); c.m.r8(a_) }) as i32);
                    } else {
                        l_star_6 = (({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(528)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(444); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(12))).wrapping_add(8); c.m.r8(a_) }) as i32);
                    }
                    F::SimTop_ShowStar(c, fp.wrapping_add(28), l_star_6);
                    { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_r_d__str15, fp);
                    { let a2_ = (c.m.r32(sym::gSimTop)).wrapping_add(432); let a3_ = (c.m.r32(sym::gSimTop)).wrapping_add(436); F::FlashAnim_Sheet(c, l_flash_1, fp, a2_, a3_, fp.wrapping_add(8), 2i32, 2i32, 128i32) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_r_d__str16, fp);
        { let a4_ = { let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(448); (c.m.r32(a_) as i32) }; let a5_ = (c.m.r32(sym::gSimTop)).wrapping_add(20); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, a4_, a5_) };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_r_d__str17, fp);
        if ((((({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(448); (c.m.r32(a_) as i32) }) >= (10i32)) as i32)) != 0) {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
        } else {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
        }
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_r_d__str18, fp);
        { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
        {
            l_i_2 = 0i32;
            'l5: loop {
                if !(((((l_i_2) < (1i32)) as i32)) != 0) { break 'l5; }
                'c6: {
                    { let a0_ = ((c.m.r32(sym::gSimTop)).wrapping_add(160)).wrapping_add((((l_i_2) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        F::IconWin_Draw(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SimTop_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn SimTop_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimTop_Update", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(396); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
        { let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(396); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    }
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gSimTop)).wrapping_add(160)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Advance(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SimTop_Update", &PROBE_N, &[], true); }
    r_
}

pub fn SimTop_UpdateVoice(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimTop_UpdateVoice", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(412); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        if ((((({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(404); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            break 'f;
        }
        if ((((((((((((({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(416); (c.m.r32(a_) as i32) }) != ((1i32).wrapping_neg())) as i32)) != 0) && (((((F::Voice_GetStat(c, 0i32)) != (5i32)) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(404); (c.m.r32(a_) as i32) }) == ({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(408); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                { let v_ = 1i32; let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(412); c.m.w32(a_, (v_) as u32); };
            }
            break 'f;
        }
    } else {
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(412); c.m.w32(a_, (v_) as u32); };
    }
    's1: {
        let sv1: i32 = { let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(404); (c.m.r32(a_) as i32) };
        match sv1 {
            1i32 => {
                { let v_ = 17i32; let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(416); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = 18i32; let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(416); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            3i32 => {
                { let v_ = 19i32; let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(416); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            4i32 => {
                { let v_ = 20i32; let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(416); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            5i32 => {
                { let v_ = 21i32; let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(416); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(404); c.m.w32(a_, (v_) as u32); };
    { let a0_ = { let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(16); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(416); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34661i32, a2_) };
    { let v_ = { let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(404); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(408); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SimTop_UpdateVoice", &PROBE_N, &[], true); }
    r_
}

pub fn SimTop_Input(c: &mut Ctx, mut p_result_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimTop_Input", &PROBE_N, &[(p_result_1) as u32], false); }
    let mut l_up_2: i32 = 0;
    let mut l_down_3: i32 = 0;
    let mut l_right_4: i32 = 0;
    let mut l_confirm_5: i32 = 0;
    let mut l_cancel_6: i32 = 0;
    let r_ = 'f: {
    l_up_2 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) as i32);
    l_down_3 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) as i32);
    l_right_4 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) as i32);
    l_confirm_5 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) as i32);
    l_cancel_6 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) as i32);
    if ((((!(((({ let a_ = (((c.m.r32(sym::gSimTop)).wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f;
    }
    if ((((!(((({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(368); (c.m.r32(a_) as i32) }) & (4i32))) != 0)) as i32)) != 0) {
        F::SimTop_ClipGoto(c, 0i32, 0i32, sym::menu_r_d__str19);
        { let v_ = 1i32; let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(404); c.m.w32(a_, (v_) as u32); };
        { let r_ = 4i32; let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(368); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    's1: {
        let sv7: i32 = { let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(400); (c.m.r32(a_) as i32) };
        match sv7 {
            0i32 => {
                if ((l_up_2) != 0) {
                    F::SimTop_ClipGoto(c, 0i32, 0i32, sym::menu_r_d__str20);
                    if ((((({ let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(372)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(400); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) < (0i32)) as i32)) != 0) {
                        { let v_ = (3i32).wrapping_sub(1i32); let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(372)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(400); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    F::SimTop_ClipGoto(c, 0i32, 0i32, sym::menu_r_d__str21);
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    { let v_ = { let a_ = (sym::gSimTopPlateVoice).wrapping_add(((({ let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(372)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(400); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(404); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(428); c.m.w32(a_, (v_) as u32); };
                } else {
                    if ((l_down_3) != 0) {
                        F::SimTop_ClipGoto(c, 0i32, 0i32, sym::menu_r_d__str22);
                        if ((((({ let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(372)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(400); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) >= (3i32)) as i32)) != 0) {
                            { let v_ = 0i32; let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(372)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(400); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        F::SimTop_ClipGoto(c, 0i32, 0i32, sym::menu_r_d__str23);
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                        { let v_ = { let a_ = (sym::gSimTopPlateVoice).wrapping_add(((({ let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(372)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(400); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(404); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(428); c.m.w32(a_, (v_) as u32); };
                    } else {
                        if ((l_confirm_5) != 0) {
                            's2: {
                                let sv8: i32 = { let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(372)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                                match sv8 {
                                    0i32 => {
                                        { let r_ = 1i32; let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(368); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                        { let r_ = 2i32; let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(368); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                        { let v_ = 15i32; let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(396); c.m.w32(a_, (v_) as u32); };
                                        break 's2;
                                    }
                                    1i32 => {
                                        { let v_ = 1i32; let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(400); c.m.w32(a_, (v_) as u32); };
                                        { let a0_ = ((c.m.r32(sym::gSimTop)).wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_r_d__str24, ((((1i32) as i64 as u64)) as u8)) };
                                        break 's2;
                                    }
                                    2i32 => {
                                        { let v_ = 2i32; let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(400); c.m.w32(a_, (v_) as u32); };
                                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(448); c.m.w32(a_, (v_) as u32); };
                                        { let a0_ = ((c.m.r32(sym::gSimTop)).wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_r_d__str25, ((((1i32) as i64 as u64)) as u8)) };
                                        break 's2;
                                    }
                                    _ => {
                                    }
                                }
                            }
                            F::Voice_StopWithLip(c);
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(428); c.m.w32(a_, (v_) as u32); };
                        } else {
                            if ((l_cancel_6) != 0) {
                                F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                                { let v_ = 0i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
                                { let a0_ = ((c.m.r32(sym::gSimTop)).wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_r_d__str26, ((((1i32) as i64 as u64)) as u8)) };
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                            } else {
                                { let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(428); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                if ((((({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(428); (c.m.r32(a_) as i32) }) == (3600i32)) as i32)) != 0) {
                                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(428); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = { let a_ = (sym::gSimTopIdleVoice).wrapping_add((F::Rand_Range(c, ((2i32) as u32))).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(404); c.m.w32(a_, (v_) as u32); };
                                }
                            }
                        }
                    }
                }
                break 's1;
            }
            1i32 => {
                if ((((((l_up_2) != 0) && ((((({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(440); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0)) as i32)) != 0) {
                    { let a0_ = ((c.m.r32(sym::gSimTop)).wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_r_d__str27, ((((1i32) as i64 as u64)) as u8)) };
                    { let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(440); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = ({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(440); (c.m.r32(a_) as i32) }).wrapping_add(5i32); let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(444); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                } else {
                    if ((((((l_down_3) != 0) && ((((((({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(440); (c.m.r32(a_) as i32) }) as u32)) < (((5i32) as u32))) as i32)) != 0)) as i32)) != 0) {
                        { let a0_ = ((c.m.r32(sym::gSimTop)).wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_r_d__str28, ((((1i32) as i64 as u64)) as u8)) };
                        { let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(440); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let v_ = ({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(440); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(444); c.m.w32(a_, (v_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    } else {
                        if ((l_cancel_6) != 0) {
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(400); c.m.w32(a_, (v_) as u32); };
                            { let a0_ = ((c.m.r32(sym::gSimTop)).wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_r_d__str29, ((((1i32) as i64 as u64)) as u8)) };
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                        }
                    }
                }
                break 's1;
            }
            2i32 => {
                if ((((((l_confirm_5) != 0) || ((l_right_4) != 0)) as i32)) != 0) {
                    if ((((({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(448); (c.m.r32(a_) as i32) }) < (10i32)) as i32)) != 0) {
                        { let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(448); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    } else {
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(400); c.m.w32(a_, (v_) as u32); };
                        { let a0_ = ((c.m.r32(sym::gSimTop)).wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_r_d__str30, ((((1i32) as i64 as u64)) as u8)) };
                    }
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                } else {
                    if ((l_cancel_6) != 0) {
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(400); c.m.w32(a_, (v_) as u32); };
                        { let a0_ = ((c.m.r32(sym::gSimTop)).wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_r_d__str31, ((((1i32) as i64 as u64)) as u8)) };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                    }
                }
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SimTop_Input", &PROBE_N, &[], true); }
    r_
}

pub fn SimTop_ClipGoto(c: &mut Ctx, mut p_movie_1: i32, mut p_level_2: i32, mut p_label_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimTop_ClipGoto", &PROBE_N, &[(p_movie_1) as u32, (p_level_2) as u32, (p_label_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_flash_4: u32 = 0;
    let r_ = 'f: {
        l_flash_4 = ((c.m.r32(sym::gSimTop)).wrapping_add(160)).wrapping_add((((p_movie_1) as u32)).wrapping_mul(44));
        { let _ = { let a2_ = ({ let a_ = ((c.m.r32(sym::gSimTop)).wrapping_add(372)).wrapping_add((((p_level_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_r_d__str32, &[((a2_) as i64 as u64)]) }; };
        F::Flash_FindLabel(c, l_flash_4, ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, l_flash_4, fp, p_label_3);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SimTop_ClipGoto", &PROBE_N, &[], true); }
    r_
}

pub fn SimTop_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimTop_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = 1i32; c.m.w32(fp, (v_) as u32); }
        F::SimTop_Init(c, p_section_1);
        F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
        'l1: while ((1i32) != 0) {
            F::Gfx_BeginFrame(c);
            F::Pad_Update(c);
            F::Snd_Update(c);
            F::ColorFade_Update(c);
            F::SimTop_Update(c);
            F::SimTop_UpdateVoice(c);
            F::SimTop_Draw(c);
            F::ColorFade_Draw(c);
            F::Gfx_EndFrame(c, 1i32);
            F::Dma_Flush(c);
            F::File_Stub264D90(c);
            if ((F::ColorFade_IsInDone(c)) != 0) {
                if ((((((((!(((({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(368); (c.m.r32(a_) as i32) }) & (8i32))) != 0)) as i32)) != 0) && (((({ let a_ = (((c.m.r32(sym::gSimTop)).wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
                    { let r_ = 8i32; let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(368); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
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
            if (((({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(368); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(396); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                }
            } else {
                if ((((({ let a_ = (c.m.r32(sym::gSimTop)).wrapping_add(404); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::SimTop_Input(c, fp);
                }
            }
        }
        F::SimTop_Term(c);
        F::Dma_ResetBuffers(c);
        break 'f (c.m.r32(fp) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SimTop_Run", &PROBE_N, &[], true); }
    r_
}
