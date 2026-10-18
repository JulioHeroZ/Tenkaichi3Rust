#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn MainMenu_StartBallLoops(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MainMenu_StartBallLoops", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_i_1: i32 = 0;
    let mut l_flash_2: u32 = 0;
    let r_ = 'f: {
        l_flash_2 = ((c.m.r32(sym::gMainMenu)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        {
            l_i_1 = 0i32;
            'l1: loop {
                if !(((((l_i_1) < (7i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_a__str0, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(8), fp);
                    F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_a__str1);
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "MainMenu_StartBallLoops", &PROBE_N, &[], true); }
    r_
}

pub fn MainMenu_Init(c: &mut Ctx, mut p_section_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MainMenu_Init", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        if (((((c.m.r32(sym::gMenuArc1)) == (((0i32) as u32))) as i32)) != 0) {
            { let v_ = { let a0_ = ({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::File_LoadSync(c, a0_, ((0i32) as u32), 0i32) }; let a_ = sym::gMenuArc1; c.m.w32(a_, v_); };
        }
        { let v_ = F::Heap_Alloc(c, 392i32, ((32i32) as u32), 0i32, 2i32); let a_ = sym::gMainMenu; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gMainMenu); F::memset(c, a0_, 0i32, ((392i32) as u32)) }; };
        { let v_ = (c.m.r32(sym::gMenuArc1)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc1)).wrapping_add((((p_section_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = c.m.r32(sym::gMainMenu); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gMainMenu); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = c.m.r32(fp); let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(60); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((41i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((43i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((42i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((44i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((45i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((46i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((47i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((48i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((49i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((30i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((31i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((32i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((33i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((38i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((39i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((40i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((35i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((34i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((37i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(64)).wrapping_add((((36i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gMainMenu)).wrapping_add(64); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = c.m.r32(fp); F::IconWin_Init(c, a0_, a1_) };
        F::IconWin_Open(c);
        { let v_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(12); c.m.w32(a_, v_); };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = { let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(8); c.m.r32(a_) }; let _ = 0i32; F::MsgWin_Init(c, a0_, a1_, 1i32) };
        F::MsgWin_Open(c);
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (11i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((((((l_i_2) == (4i32)) as i32)) != 0) && ((((!((F::Port_NetMenuListed(c)) != 0)) as i32)) != 0)) as i32)) != 0) {
                        break 'c2;
                    }
                    if (((((((((l_i_2) == (10i32)) as i32)) != 0) && ((((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(8); (c.m.r32(a_) as i32) }) & (127i32))) != (127i32)) as i32)) != 0)) as i32)) != 0) {
                        break 'c2;
                    }
                    { let v_ = l_i_2; let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(280)).wrapping_add(((({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(324); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); o_ }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(324); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(328); c.m.w32(a_, (v_) as u32); };
        {
            l_i_2 = 0i32;
            'l3: loop {
                if !(((((l_i_2) < ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(324); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l3; }
                'c4: {
                    if ((((({ let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(280)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == ({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(44); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        { let v_ = (l_i_2).wrapping_sub(1i32); let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(328); c.m.w32(a_, (v_) as u32); };
                        if ((((({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(328); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                            { let r_ = { let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(324); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(328); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                        }
                        break 'l3;
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(276); c.m.w32(a_, (v_) as u32); };
        {
            l_i_2 = 0i32;
            'l5: loop {
                if !(((((l_i_2) < (4i32)) as i32)) != 0) { break 'l5; }
                'c6: {
                    { let v_ = ((F::Rand_Range(c, ((32i32) as u32))) as i32); let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(352)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "MainMenu_Init", &PROBE_N, &[], true); }
    r_
}

pub fn MainMenu_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MainMenu_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    F::MsgWin_Term(c);
    F::IconWin_Term(c);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(16)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Destroy(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gMainMenu)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gMainMenu); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gMainMenu; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "MainMenu_Term", &PROBE_N, &[], true); }
    r_
}

pub fn MainMenu_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MainMenu_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(160) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 160);
    let mut l_i_1: i32 = 0;
    let mut l_item_2: i32 = 0;
    let mut l_flash_3: u32 = 0;
    let r_ = 'f: {
        { let a0_ = { let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(60); c.m.r32(a_) }; F::Sprite_DrawPicture(c, a0_, 0i32, 0i32, 128i32) };
        l_flash_3 = ((c.m.r32(sym::gMainMenu)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        {
            l_i_1 = 0i32;
            'l1: loop {
                if !(((((l_i_1) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    's3: {
                        let sv4: i32 = l_i_1;
                        match sv4 {
                            0i32 => {
                                F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), sym::menu_a__str2, fp.wrapping_add(92));
                                F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), sym::menu_a__str3, fp.wrapping_add(100));
                                break 's3;
                            }
                            1i32 => {
                                F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), sym::menu_a__str4, fp.wrapping_add(92));
                                F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), sym::menu_a__str5, fp.wrapping_add(100));
                                break 's3;
                            }
                            3i32 => {
                                F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), sym::menu_a__str6, fp.wrapping_add(92));
                                F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), sym::menu_a__str7, fp.wrapping_add(100));
                                break 's3;
                            }
                            2i32 => {
                                F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), sym::menu_a__str8, fp.wrapping_add(92));
                                F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), sym::menu_a__str9, fp.wrapping_add(100));
                                break 's3;
                            }
                            _ => {
                            }
                        }
                    }
                    { let a2_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(352)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); F::FlashAnim_Blink(c, l_flash_3, fp.wrapping_add(92), a2_, 0i32) };
                    if ((((({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(348); (c.m.r32(a_) as i32) }) == (l_i_1)) as i32)) != 0) {
                        { let a2_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(368)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); F::FlashAnim_Talk(c, l_flash_3, fp.wrapping_add(100), a2_, 0i32) };
                    } else {
                        F::FlashAnim_ShowNext2(c, l_flash_3, fp.wrapping_add(100), 0i32);
                    }
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        {
            rt::memset(c, fp.wrapping_add(108), 0, 44);
            { let v_ = 4i32; c.m.w32(fp.wrapping_add(108).wrapping_add(0), (v_) as u32); }
            { let v_ = 4i32; c.m.w32(fp.wrapping_add(108).wrapping_add(4), (v_) as u32); }
            { let v_ = 4i32; c.m.w32(fp.wrapping_add(108).wrapping_add(8), (v_) as u32); }
            { let v_ = 5i32; c.m.w32(fp.wrapping_add(108).wrapping_add(12), (v_) as u32); }
            { let v_ = 5i32; c.m.w32(fp.wrapping_add(108).wrapping_add(16), (v_) as u32); }
            { let v_ = 5i32; c.m.w32(fp.wrapping_add(108).wrapping_add(20), (v_) as u32); }
            { let v_ = 5i32; c.m.w32(fp.wrapping_add(108).wrapping_add(24), (v_) as u32); }
            { let v_ = 4i32; c.m.w32(fp.wrapping_add(108).wrapping_add(28), (v_) as u32); }
            { let v_ = 5i32; c.m.w32(fp.wrapping_add(108).wrapping_add(32), (v_) as u32); }
            { let v_ = 5i32; c.m.w32(fp.wrapping_add(108).wrapping_add(36), (v_) as u32); }
            { let v_ = 4i32; c.m.w32(fp.wrapping_add(108).wrapping_add(40), (v_) as u32); }
            {
                l_i_1 = 0i32;
                'l4: loop {
                    if !(((((l_i_1) < (6i32)) as i32)) != 0) { break 'l4; }
                    'c5: {
                        l_item_2 = { let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(280)).wrapping_add((((rt::rem_i32(({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(328); (c.m.r32(a_) as i32) }).wrapping_add(l_i_1), { let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(324); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = (rt::rem_i32(l_item_2, 8i32)).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_a__str10, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                        F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_a__str11, fp);
                        F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                        F::Flash_ClipSetTex(c, l_flash_3, fp, rt::div_i32(l_item_2, 8i32));
                        F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_a__str12, fp);
                        F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                        F::Flash_ClipSetTex(c, l_flash_3, fp, rt::div_i32(l_item_2, 8i32));
                        { let v_ = (rt::rem_i32(l_item_2, 4i32)).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = (rt::div_i32(l_item_2, 4i32)).wrapping_mul(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_a__str13, fp);
                        F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = (rt::rem_i32(l_item_2, 10i32)).wrapping_mul(48i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 48i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(48i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_a__str14, fp);
                        F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                        F::Flash_ClipSetTex(c, l_flash_3, fp, rt::div_i32(l_item_2, 10i32));
                        if (((((l_item_2) == ({ let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(280)).wrapping_add((((rt::rem_i32((({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(328); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(268); (c.m.r32(a_) as i32) })).wrapping_add(1i32), { let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(324); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                            { let a2_ = (c.m.r32(sym::gMainMenu)).wrapping_add(340); let a3_ = (c.m.r32(sym::gMainMenu)).wrapping_add(344); let a5_ = { let a_ = (fp.wrapping_add(108)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(280)).wrapping_add((((rt::rem_i32((({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(328); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(268); (c.m.r32(a_) as i32) })).wrapping_add(1i32), { let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(324); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a7_ = (if ((((({ let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(280)).wrapping_add((((rt::rem_i32((({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(328); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(268); (c.m.r32(a_) as i32) })).wrapping_add(1i32), { let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(324); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (3i32)) as i32)) != 0) { 3i32 } else { 6i32 }); F::FlashAnim_Sheet(c, l_flash_3, fp, a2_, a3_, fp.wrapping_add(8), a5_, 1i32, a7_) };
                        }
                    }
                    { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
                }
            }
        }
        {
            l_item_2 = { let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(280)).wrapping_add(((({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(336); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
            { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = (rt::rem_i32(l_item_2, 8i32)).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
            F::Flash_FindLabel(c, l_flash_3, sym::menu_a__str15, sym::menu_a__str16, fp);
            F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
            F::Flash_ClipSetTex(c, l_flash_3, fp, rt::div_i32(l_item_2, 8i32));
            F::Flash_FindLabel(c, l_flash_3, sym::menu_a__str17, sym::menu_a__str18, fp);
            F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
            F::Flash_ClipSetTex(c, l_flash_3, fp, rt::div_i32(l_item_2, 8i32));
            { let v_ = (rt::rem_i32(l_item_2, 4i32)).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = (rt::div_i32(l_item_2, 4i32)).wrapping_mul(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
            F::Flash_FindLabel(c, l_flash_3, sym::menu_a__str19, sym::menu_a__str20, fp);
            F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
            { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = (rt::rem_i32(l_item_2, 10i32)).wrapping_mul(48i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            { let v_ = 48i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(48i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
            F::Flash_FindLabel(c, l_flash_3, sym::menu_a__str21, sym::menu_a__str22, fp);
            F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
            F::Flash_ClipSetTex(c, l_flash_3, fp, rt::div_i32(l_item_2, 10i32));
        }
        {
            l_i_1 = 0i32;
            'l6: loop {
                if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l6; }
                'c7: {
                    { let v_ = (l_i_1).wrapping_mul(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), (if ((l_i_1) != 0) { sym::menu_a__str23 } else { sym::menu_a__str24 }), fp);
                    F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        {
            l_i_1 = 0i32;
            'l8: loop {
                if !(((((l_i_1) < (7i32)) as i32)) != 0) { break 'l8; }
                'c9: {
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_a__str25, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                    if (((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(8); (c.m.r32(a_) as i32) }) & ((((1u32).wrapping_shl((l_i_1) as u32)) as i32)))) != 0) {
                        F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), fp.wrapping_add(28), fp);
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                        { let v_ = (rt::rem_i32(l_i_1, 4i32)).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = (rt::div_i32(l_i_1, 4i32)).wrapping_mul(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_a__str26, fp);
                        F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                    } else {
                        F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), fp.wrapping_add(28), fp);
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    }
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        {
            l_i_1 = 0i32;
            'l10: loop {
                if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l10; }
                'c11: {
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), (if ((l_i_1) != 0) { sym::menu_a__str27 } else { sym::menu_a__str28 }), fp);
                    { let a3_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(384)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); F::FlashAnim_Scroll(c, l_flash_3, fp, fp.wrapping_add(8), a3_, ((0i32) as u32), (if ((l_i_1) != 0) { F32(0xbe422e45) } else { F32(0xbeb60b60) }), F32(0x00000000)) };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        {
            l_i_1 = 0i32;
            'l12: loop {
                if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l12; }
                'c13: {
                    { let a0_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(16)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        F::IconWin_Draw(c);
        { let a2_ = { let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(276); (c.m.r32(a_) as i32) }; F::MsgWin_Draw(c, 0i32, 0i32, a2_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "MainMenu_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn MainMenu_PlateGoto(c: &mut Ctx, mut p_idx_1: i32, mut p_label_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MainMenu_PlateGoto", &PROBE_N, &[(p_idx_1) as u32, (p_label_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_flash_3: u32 = 0;
    let r_ = 'f: {
        l_flash_3 = ((c.m.r32(sym::gMainMenu)).wrapping_add(16)).wrapping_add((((p_idx_1) as u32)).wrapping_mul(44));
        { let _ = { let a2_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(268); (c.m.r32(a_) as i32) }).wrapping_add(2i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_a__str29, &[((a2_) as i64 as u64)]) }; };
        F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, l_flash_3, fp, p_label_2);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "MainMenu_PlateGoto", &PROBE_N, &[], true); }
    r_
}

pub fn MainMenu_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MainMenu_Update", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(264); (c.m.r32(a_) as i32) }) & (8i32))) != 0) {
        if ((((((((({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(276); (c.m.r32(a_) as i32) }) == (({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(348); (c.m.r32(a_) as i32) }).wrapping_add(12i32))) as i32)) != 0) || ((((({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(276); (c.m.r32(a_) as i32) }) == (({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(348); (c.m.r32(a_) as i32) }).wrapping_add(60i32))) as i32)) != 0)) as i32)) != 0) {
            if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                { let v_ = ((F::Rand_Range(c, ((4i32) as u32))) as i32); let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(348); c.m.w32(a_, (v_) as u32); };
                { let v_ = ((({ let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(280)).wrapping_add((((rt::rem_i32((({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(328); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(268); (c.m.r32(a_) as i32) })).wrapping_add(1i32), { let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(324); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_mul(4i32)).wrapping_add({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(348); (c.m.r32(a_) as i32) })).wrapping_add(16i32); let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(276); c.m.w32(a_, (v_) as u32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(276); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34553i32, a2_) };
            }
        }
    }
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(16)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Advance(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "MainMenu_Update", &PROBE_N, &[], true); }
    r_
}

pub fn MainMenu_Input(c: &mut Ctx, mut p_result_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MainMenu_Input", &PROBE_N, &[(p_result_1) as u32], false); }
    let mut sv_2: i32 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
    'fl1: loop {
        match bb_ {
            0 => {
                if ((((!(((({ let a_ = (((c.m.r32(sym::gMainMenu)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) { bb_ = 1; continue 'fl1; } else { bb_ = 2; continue 'fl1; }
            }
            1 => {
                break 'f;
            }
            2 => {
                if ((((!(((({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(264); (c.m.r32(a_) as i32) }) & (4i32))) != 0)) as i32)) != 0) { bb_ = 4; continue 'fl1; } else { bb_ = 5; continue 'fl1; }
            }
            3 => {
                bb_ = 2; continue 'fl1;
            }
            4 => {
                F::MainMenu_PlateGoto(c, 0i32, sym::menu_a__str30);
                if ((((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(8); (c.m.r32(a_) as i32) }) & (127i32))) == (127i32)) as i32)) != 0) { bb_ = 6; continue 'fl1; } else { bb_ = 7; continue 'fl1; }
            }
            5 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) { bb_ = 8; continue 'fl1; } else { bb_ = 10; continue 'fl1; }
            }
            6 => {
                F::MainMenu_StartBallLoops(c);
                bb_ = 7; continue 'fl1;
            }
            7 => {
                { let r_ = 4i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                bb_ = 5; continue 'fl1;
            }
            8 => {
                F::MainMenu_PlateGoto(c, 0i32, sym::menu_a__str31);
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(340); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(344); c.m.w32(a_, (v_) as u32); };
                { let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(268); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                if ((((({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(268); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) { bb_ = 11; continue 'fl1; } else { bb_ = 12; continue 'fl1; }
            }
            9 => {
                break 'fl1;
            }
            10 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) { bb_ = 17; continue 'fl1; } else { bb_ = 19; continue 'fl1; }
            }
            11 => {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(268); c.m.w32(a_, (v_) as u32); };
                { let a0_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_a__str32, ((((1i32) as i64 as u64)) as u8)) };
                { let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(328); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                if ((((({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(328); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) { bb_ = 13; continue 'fl1; } else { bb_ = 14; continue 'fl1; }
            }
            12 => {
                bb_ = 15; continue 'fl1;
            }
            13 => {
                { let r_ = { let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(324); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(328); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                bb_ = 14; continue 'fl1;
            }
            14 => {
                { let v_ = rt::rem_i32(({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(328); (c.m.r32(a_) as i32) }).wrapping_add(6i32), { let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(324); (c.m.r32(a_) as i32) }); let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(336); c.m.w32(a_, (v_) as u32); };
                bb_ = 12; continue 'fl1;
            }
            15 => {
                F::MainMenu_PlateGoto(c, 0i32, sym::menu_a__str35);
                { let v_ = ((F::Rand_Range(c, ((4i32) as u32))) as i32); let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(348); c.m.w32(a_, (v_) as u32); };
                { let v_ = ((({ let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(280)).wrapping_add((((rt::rem_i32((({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(328); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(268); (c.m.r32(a_) as i32) })).wrapping_add(1i32), { let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(324); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_mul(4i32)).wrapping_add({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(348); (c.m.r32(a_) as i32) })).wrapping_add(16i32); let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(276); c.m.w32(a_, (v_) as u32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(276); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34553i32, a2_) };
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                bb_ = 18; continue 'fl1;
            }
            16 => {
                bb_ = 9; continue 'fl1;
            }
            17 => {
                F::MainMenu_PlateGoto(c, 0i32, sym::menu_a__str33);
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(340); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(344); c.m.w32(a_, (v_) as u32); };
                { let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(268); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                if ((((({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(268); (c.m.r32(a_) as i32) }) >= (4i32)) as i32)) != 0) { bb_ = 20; continue 'fl1; } else { bb_ = 21; continue 'fl1; }
            }
            18 => {
                bb_ = 9; continue 'fl1;
            }
            19 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) { bb_ = 24; continue 'fl1; } else { bb_ = 26; continue 'fl1; }
            }
            20 => {
                { let v_ = (4i32).wrapping_sub(1i32); let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(268); c.m.w32(a_, (v_) as u32); };
                { let a0_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_a__str34, ((((1i32) as i64 as u64)) as u8)) };
                { let v_ = { let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(328); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(336); c.m.w32(a_, (v_) as u32); };
                { let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(328); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                if ((((({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(328); (c.m.r32(a_) as i32) }) >= ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(324); (c.m.r32(a_) as i32) })) as i32)) != 0) { bb_ = 22; continue 'fl1; } else { bb_ = 23; continue 'fl1; }
            }
            21 => {
                bb_ = 15; continue 'fl1;
            }
            22 => {
                { let r_ = { let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(324); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(328); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                bb_ = 23; continue 'fl1;
            }
            23 => {
                bb_ = 21; continue 'fl1;
            }
            24 => {
                sv_2 = { let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(280)).wrapping_add((((rt::rem_i32((({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(328); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(268); (c.m.r32(a_) as i32) })).wrapping_add(1i32), { let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(324); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                bb_ = match sv_2 { 0i32 => 29, 1i32 => 33, 2i32 => 37, 3i32 => 41, 4i32 => 45, 5i32 => 47, 6i32 => 51, 7i32 => 55, 8i32 => 59, 9i32 => 63, 10i32 => 67, _ => 27, }; continue 'fl1;
            }
            25 => {
                bb_ = 18; continue 'fl1;
            }
            26 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) { bb_ = 71; continue 'fl1; } else { bb_ = 72; continue 'fl1; }
            }
            27 => {
                F::MainMenu_PlateGoto(c, 0i32, sym::menu_a__str46);
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                bb_ = 25; continue 'fl1;
            }
            28 => {
                bb_ = 29; continue 'fl1;
            }
            29 => {
                { let r_ = 1i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = 2i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 15i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(272); c.m.w32(a_, (v_) as u32); };
                { let v_ = 6i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let a0_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_a__str36, ((((1i32) as i64 as u64)) as u8)) };
                { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2572); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                if ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2572); (c.m.r32(a_) as i32) }) >= (24i32)) as i32)) != 0) { bb_ = 30; continue 'fl1; } else { bb_ = 31; continue 'fl1; }
            }
            30 => {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2572); c.m.w32(a_, (v_) as u32); };
                bb_ = 31; continue 'fl1;
            }
            31 => {
                bb_ = 27; continue 'fl1;
            }
            32 => {
                bb_ = 33; continue 'fl1;
            }
            33 => {
                { let r_ = 1i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = 2i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 15i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(272); c.m.w32(a_, (v_) as u32); };
                { let v_ = 13i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let a0_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_a__str37, ((((1i32) as i64 as u64)) as u8)) };
                { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2572); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                if ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2572); (c.m.r32(a_) as i32) }) >= (24i32)) as i32)) != 0) { bb_ = 34; continue 'fl1; } else { bb_ = 35; continue 'fl1; }
            }
            34 => {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2572); c.m.w32(a_, (v_) as u32); };
                bb_ = 35; continue 'fl1;
            }
            35 => {
                bb_ = 27; continue 'fl1;
            }
            36 => {
                bb_ = 37; continue 'fl1;
            }
            37 => {
                { let r_ = 1i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = 2i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 15i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(272); c.m.w32(a_, (v_) as u32); };
                { let v_ = 33i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let a0_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_a__str38, ((((1i32) as i64 as u64)) as u8)) };
                { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2572); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                if ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2572); (c.m.r32(a_) as i32) }) >= (24i32)) as i32)) != 0) { bb_ = 38; continue 'fl1; } else { bb_ = 39; continue 'fl1; }
            }
            38 => {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2572); c.m.w32(a_, (v_) as u32); };
                bb_ = 39; continue 'fl1;
            }
            39 => {
                bb_ = 27; continue 'fl1;
            }
            40 => {
                bb_ = 41; continue 'fl1;
            }
            41 => {
                { let r_ = 1i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = 2i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 15i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(272); c.m.w32(a_, (v_) as u32); };
                { let v_ = 38i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let a0_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_a__str39, ((((1i32) as i64 as u64)) as u8)) };
                { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2572); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                if ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2572); (c.m.r32(a_) as i32) }) >= (24i32)) as i32)) != 0) { bb_ = 42; continue 'fl1; } else { bb_ = 43; continue 'fl1; }
            }
            42 => {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2572); c.m.w32(a_, (v_) as u32); };
                bb_ = 43; continue 'fl1;
            }
            43 => {
                bb_ = 27; continue 'fl1;
            }
            44 => {
                bb_ = 45; continue 'fl1;
            }
            45 => {
                F::Port_NetMenuOpen(c);
                bb_ = 27; continue 'fl1;
            }
            46 => {
                bb_ = 47; continue 'fl1;
            }
            47 => {
                { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2572); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                if ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2572); (c.m.r32(a_) as i32) }) >= (24i32)) as i32)) != 0) { bb_ = 48; continue 'fl1; } else { bb_ = 49; continue 'fl1; }
            }
            48 => {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2572); c.m.w32(a_, (v_) as u32); };
                bb_ = 49; continue 'fl1;
            }
            49 => {
                { let r_ = 1i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = 2i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 15i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(272); c.m.w32(a_, (v_) as u32); };
                { let v_ = 48i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let a0_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_a__str40, ((((1i32) as i64 as u64)) as u8)) };
                bb_ = 27; continue 'fl1;
            }
            50 => {
                bb_ = 51; continue 'fl1;
            }
            51 => {
                { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2572); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                if ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2572); (c.m.r32(a_) as i32) }) >= (24i32)) as i32)) != 0) { bb_ = 52; continue 'fl1; } else { bb_ = 53; continue 'fl1; }
            }
            52 => {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2572); c.m.w32(a_, (v_) as u32); };
                bb_ = 53; continue 'fl1;
            }
            53 => {
                { let r_ = 1i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = 2i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 15i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(272); c.m.w32(a_, (v_) as u32); };
                { let v_ = 44i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let a0_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_a__str41, ((((1i32) as i64 as u64)) as u8)) };
                bb_ = 27; continue 'fl1;
            }
            54 => {
                bb_ = 55; continue 'fl1;
            }
            55 => {
                { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2572); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                if ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2572); (c.m.r32(a_) as i32) }) >= (24i32)) as i32)) != 0) { bb_ = 56; continue 'fl1; } else { bb_ = 57; continue 'fl1; }
            }
            56 => {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2572); c.m.w32(a_, (v_) as u32); };
                bb_ = 57; continue 'fl1;
            }
            57 => {
                { let r_ = 1i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = 2i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 15i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(272); c.m.w32(a_, (v_) as u32); };
                { let v_ = 53i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let a0_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_a__str42, ((((1i32) as i64 as u64)) as u8)) };
                bb_ = 27; continue 'fl1;
            }
            58 => {
                bb_ = 59; continue 'fl1;
            }
            59 => {
                { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2572); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                if ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2572); (c.m.r32(a_) as i32) }) >= (24i32)) as i32)) != 0) { bb_ = 60; continue 'fl1; } else { bb_ = 61; continue 'fl1; }
            }
            60 => {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2572); c.m.w32(a_, (v_) as u32); };
                bb_ = 61; continue 'fl1;
            }
            61 => {
                { let r_ = 1i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = 2i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 15i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(272); c.m.w32(a_, (v_) as u32); };
                { let v_ = 60i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let a0_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_a__str43, ((((1i32) as i64 as u64)) as u8)) };
                bb_ = 27; continue 'fl1;
            }
            62 => {
                bb_ = 63; continue 'fl1;
            }
            63 => {
                { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2572); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                if ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2572); (c.m.r32(a_) as i32) }) >= (24i32)) as i32)) != 0) { bb_ = 64; continue 'fl1; } else { bb_ = 65; continue 'fl1; }
            }
            64 => {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2572); c.m.w32(a_, (v_) as u32); };
                bb_ = 65; continue 'fl1;
            }
            65 => {
                { let r_ = 1i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = 2i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 15i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(272); c.m.w32(a_, (v_) as u32); };
                { let v_ = 62i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let a0_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_a__str44, ((((1i32) as i64 as u64)) as u8)) };
                bb_ = 27; continue 'fl1;
            }
            66 => {
                bb_ = 67; continue 'fl1;
            }
            67 => {
                { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2572); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                if ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2572); (c.m.r32(a_) as i32) }) >= (24i32)) as i32)) != 0) { bb_ = 68; continue 'fl1; } else { bb_ = 69; continue 'fl1; }
            }
            68 => {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2572); c.m.w32(a_, (v_) as u32); };
                bb_ = 69; continue 'fl1;
            }
            69 => {
                { let r_ = 1i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = 2i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 15i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(272); c.m.w32(a_, (v_) as u32); };
                { let v_ = 70i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let a0_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_a__str45, ((((1i32) as i64 as u64)) as u8)) };
                bb_ = 27; continue 'fl1;
            }
            70 => {
                bb_ = 27; continue 'fl1;
            }
            71 => {
                { let a0_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_a__str47, ((((1i32) as i64 as u64)) as u8)) };
                F::IconWin_Close(c);
                F::MsgWin_Close(c);
                F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                { let v_ = 0i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                bb_ = 72; continue 'fl1;
            }
            72 => {
                bb_ = 25; continue 'fl1;
            }
            _ => unreachable!(),
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "MainMenu_Input", &PROBE_N, &[], true); }
    r_
}

pub fn MainMenu_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MainMenu_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = 1i32; c.m.w32(fp, (v_) as u32); }
        if ((F::Port_NetSession(c)) != 0) {
            F::Port_NetLeave(c);
        }
        F::MainMenu_Init(c, p_section_1);
        F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
        'l1: while ((1i32) != 0) {
            F::Gfx_BeginFrame(c);
            F::Pad_Update(c);
            F::Snd_Update(c);
            F::ColorFade_Update(c);
            if ((((!(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0)) as i32)) != 0) {
                F::MainMenu_Update(c);
            }
            F::MainMenu_Draw(c);
            F::ColorFade_Draw(c);
            F::Gfx_EndFrame(c, 1i32);
            F::Dma_Flush(c);
            F::File_Stub264D90(c);
            if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
                continue 'l1;
            }
            if ((F::ColorFade_IsInDone(c)) != 0) {
                if ((((((((!(((({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(264); (c.m.r32(a_) as i32) }) & (8i32))) != 0)) as i32)) != 0) && (((({ let a_ = (((c.m.r32(sym::gMainMenu)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
                    { let r_ = 8i32; let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = ((F::Rand_Range(c, ((4i32) as u32))) as i32); let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(348); c.m.w32(a_, (v_) as u32); };
                    if ((((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(8); (c.m.r32(a_) as i32) }) & (127i32))) == (127i32)) as i32)) != 0) {
                        { let v_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(348); (c.m.r32(a_) as i32) }).wrapping_add(60i32); let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(276); c.m.w32(a_, (v_) as u32); };
                    } else {
                        { let v_ = ({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(348); (c.m.r32(a_) as i32) }).wrapping_add(12i32); let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(276); c.m.w32(a_, (v_) as u32); };
                    }
                    { let a0_ = { let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(276); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34553i32, a2_) };
                }
            }
            if ((F::ColorFade_IsFadingOut(c)) != 0) {
                F::Voice_FadeOutStep(c, 0i32);
                F::Bgm_FadeOutStep(c);
                continue 'l1;
            }
            if ((F::ColorFade_IsOutDone(c)) != 0) {
                break 'l1;
            }
            if (((({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(264); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(272); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) == ((1i32).wrapping_neg())) as i32)) != 0) {
                    F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                    { let v_ = { let a_ = ((c.m.r32(sym::gMainMenu)).wrapping_add(280)).wrapping_add((((rt::rem_i32((({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(328); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(268); (c.m.r32(a_) as i32) })).wrapping_add(1i32), { let a_ = (c.m.r32(sym::gMainMenu)).wrapping_add(324); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                }
            } else {
                F::MainMenu_Input(c, fp);
            }
        }
        { let _ = F::sceGsSyncPath(c, 0i32, ((0i32) as u16)); };
        F::MainMenu_Term(c);
        F::Dma_ResetBuffers(c);
        break 'f (c.m.r32(fp) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "MainMenu_Run", &PROBE_N, &[], true); }
    r_
}
