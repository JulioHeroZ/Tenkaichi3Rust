#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Bracket_Load(c: &mut Ctx, mut p_b_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bracket_Load", &PROBE_N, &[(p_b_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        if (((((c.m.r32(p_b_1)) == (((0i32) as u32))) as i32)) != 0) {
            { let v_ = { let a0_ = (({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(132); (c.m.r32(a_) as i32) })).wrapping_add(6i32); F::File_LoadSync(c, a0_, ((0i32) as u32), 0i32) }; let a_ = p_b_1; c.m.w32(a_, v_); };
        }
        { let v_ = { let a0_ = c.m.r32(p_b_1); F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (p_b_1).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((32i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp.wrapping_add(12)); let a2_ = c.m.r32(fp.wrapping_add(12)); F::Res_RelocateOffsets(c, fp.wrapping_add(12), a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(432)).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((34i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp.wrapping_add(12)); let a2_ = c.m.r32(fp.wrapping_add(12)); F::Res_RelocateOffsets(c, fp.wrapping_add(12), a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(432)).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(432)).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(432)).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(432)).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((35i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp.wrapping_add(12)); let a2_ = c.m.r32(fp.wrapping_add(12)); F::Res_RelocateOffsets(c, fp.wrapping_add(12), a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(432)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(432)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((37i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp.wrapping_add(12)); let a2_ = c.m.r32(fp.wrapping_add(12)); F::Res_RelocateOffsets(c, fp.wrapping_add(12), a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(432)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp.wrapping_add(12)); let a2_ = c.m.r32(fp.wrapping_add(12)); F::Res_RelocateOffsets(c, fp.wrapping_add(12), a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(432)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp.wrapping_add(12)); let a2_ = c.m.r32(fp.wrapping_add(12)); F::Res_RelocateOffsets(c, fp.wrapping_add(12), a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(432)).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(432)).wrapping_add((((30i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(432)).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(432)).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(432)).wrapping_add((((35i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(432)).wrapping_add((((34i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp.wrapping_add(12)); let a2_ = c.m.r32(fp.wrapping_add(12)); F::Res_RelocateOffsets(c, fp.wrapping_add(12), a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(432)).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(432)).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(432)).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(432)).wrapping_add((((31i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp.wrapping_add(12)); let a2_ = c.m.r32(fp.wrapping_add(12)); F::Res_RelocateOffsets(c, fp.wrapping_add(12), a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(432)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp.wrapping_add(12)); let a2_ = c.m.r32(fp.wrapping_add(12)); F::Res_RelocateOffsets(c, fp.wrapping_add(12), a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(432)).wrapping_add((((37i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(432)).wrapping_add((((33i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp.wrapping_add(12)); let a2_ = c.m.r32(fp.wrapping_add(12)); F::Res_RelocateOffsets(c, fp.wrapping_add(12), a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(432)).wrapping_add((((36i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(432)).wrapping_add((((32i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp.wrapping_add(12)); let a2_ = c.m.r32(fp.wrapping_add(12)); F::Res_RelocateOffsets(c, fp.wrapping_add(12), a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(432)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(432)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(432)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(432)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(432)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(432)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(432)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((7i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(432)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((8i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(432)).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((8i32).wrapping_mul(64i32)) as u32)); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a1_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::Flash_Create(c, ((p_b_1).wrapping_add(48)).wrapping_add((((5i32) as u32)).wrapping_mul(44)), a1_, (p_b_1).wrapping_add(432)) };
        F::Flash_Play(c, ((p_b_1).wrapping_add(48)).wrapping_add((((5i32) as u32)).wrapping_mul(44)), 1i32);
        if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (16i32))) != 0) {
            F::Flash_GotoLabel(c, ((p_b_1).wrapping_add(48)).wrapping_add((((5i32) as u32)).wrapping_mul(44)), sym::menu_k_f__str0, ((((1i32) as i64 as u64)) as u8));
        }
        { let v_ = { let a0_ = ({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(132); (c.m.r32(a_) as i32) }).wrapping_add(969i32); F::File_LoadSync(c, a0_, ((0i32) as u32), 0i32) }; let a_ = (p_b_1).wrapping_add(44); c.m.w32(a_, v_); };
        { let a0_ = c.m.r32((p_b_1).wrapping_add(44)); let a1_ = (({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(132); (c.m.r32(a_) as i32) }) as u32); F::TourBg_Init(c, a0_, a1_, fp) };
        's1: {
            let sv3: i32 = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(132); (c.m.r32(a_) as i32) };
            match sv3 {
                1i32 | 2i32 => {
                    { let v_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((43i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp.wrapping_add(12)); let a2_ = c.m.r32(fp.wrapping_add(12)); F::Res_RelocateOffsets(c, fp.wrapping_add(12), a1_, a2_) };
                    { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(584)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(584)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(584)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = c.m.r32(fp.wrapping_add(12)); let a_ = ((p_b_1).wrapping_add(316)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((44i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp.wrapping_add(12)); let a2_ = c.m.r32(fp.wrapping_add(12)); F::Res_RelocateOffsets(c, fp.wrapping_add(12), a1_, a2_) };
                    { let v_ = c.m.r32(fp.wrapping_add(12)); let a_ = ((p_b_1).wrapping_add(316)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let a1_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::Flash_Create(c, ((p_b_1).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(44)), a1_, (p_b_1).wrapping_add(584)) };
                    break 's1;
                }
                0i32 | 3i32 => {
                    { let v_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((43i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp.wrapping_add(12)); let a2_ = c.m.r32(fp.wrapping_add(12)); F::Res_RelocateOffsets(c, fp.wrapping_add(12), a1_, a2_) };
                    { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(584)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(584)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let a1_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::Flash_Create(c, ((p_b_1).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(44)), a1_, (p_b_1).wrapping_add(584)) };
                    break 's1;
                }
                4i32 => {
                    { let v_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((43i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp.wrapping_add(12)); let a2_ = c.m.r32(fp.wrapping_add(12)); F::Res_RelocateOffsets(c, fp.wrapping_add(12), a1_, a2_) };
                    { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(584)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(584)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(584)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = c.m.r32(fp.wrapping_add(12)); let a_ = ((p_b_1).wrapping_add(316)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((44i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp.wrapping_add(12)); let a2_ = c.m.r32(fp.wrapping_add(12)); F::Res_RelocateOffsets(c, fp.wrapping_add(12), a1_, a2_) };
                    { let v_ = c.m.r32(fp.wrapping_add(12)); let a_ = ((p_b_1).wrapping_add(316)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((45i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp.wrapping_add(12)); let a2_ = c.m.r32(fp.wrapping_add(12)); F::Res_RelocateOffsets(c, fp.wrapping_add(12), a1_, a2_) };
                    { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(584)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(584)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(584)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let a1_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::Flash_Create(c, ((p_b_1).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(44)), a1_, (p_b_1).wrapping_add(584)) };
                    break 's1;
                }
                _ => {
                }
            }
        }
        if ((((!(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (16i32))) != 0)) as i32)) != 0) {
            F::Flash_Play(c, ((p_b_1).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(44)), 1i32);
            F::Flash_GotoLabel(c, ((p_b_1).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(44)), sym::menu_k_f__str1, ((((1i32) as i64 as u64)) as u8));
        }
        { let v_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp.wrapping_add(12)); let a2_ = c.m.r32(fp.wrapping_add(12)); F::Res_RelocateOffsets(c, fp.wrapping_add(12), a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(328)).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp.wrapping_add(12)); let a2_ = c.m.r32(fp.wrapping_add(12)); F::Res_RelocateOffsets(c, fp.wrapping_add(12), a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(328)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(328)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(620)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(620)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(644)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(644)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp.wrapping_add(12)); let a2_ = c.m.r32(fp.wrapping_add(12)); F::Res_RelocateOffsets(c, fp.wrapping_add(12), a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(328)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(328)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((30i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp.wrapping_add(12)); let a2_ = c.m.r32(fp.wrapping_add(12)); F::Res_RelocateOffsets(c, fp.wrapping_add(12), a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(328)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(328)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(328)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(328)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a1_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::Flash_Create(c, ((p_b_1).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), a1_, (p_b_1).wrapping_add(328)) };
        { let v_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp.wrapping_add(12)); let a2_ = c.m.r32(fp.wrapping_add(12)); F::Res_RelocateOffsets(c, fp.wrapping_add(12), a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(620)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(644)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp.wrapping_add(12)); let a2_ = c.m.r32(fp.wrapping_add(12)); F::Res_RelocateOffsets(c, fp.wrapping_add(12), a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(620)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a1_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::Flash_Create(c, ((p_b_1).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(44)), a1_, (p_b_1).wrapping_add(620)) };
        { let a1_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::Flash_Create(c, ((p_b_1).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(44)), a1_, (p_b_1).wrapping_add(644)) };
        { let v_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp.wrapping_add(12)); let a2_ = c.m.r32(fp.wrapping_add(12)); F::Res_RelocateOffsets(c, fp.wrapping_add(12), a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(668)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp.wrapping_add(12)); let a2_ = c.m.r32(fp.wrapping_add(12)); F::Res_RelocateOffsets(c, fp.wrapping_add(12), a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(668)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(668)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(668)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(668)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp.wrapping_add(12)); let a2_ = c.m.r32(fp.wrapping_add(12)); F::Res_RelocateOffsets(c, fp.wrapping_add(12), a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(668)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(668)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp.wrapping_add(12)); let a2_ = c.m.r32(fp.wrapping_add(12)); F::Res_RelocateOffsets(c, fp.wrapping_add(12), a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(668)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp.wrapping_add(12)); let a2_ = c.m.r32(fp.wrapping_add(12)); F::Res_RelocateOffsets(c, fp.wrapping_add(12), a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(668)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(668)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a1_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::Flash_Create(c, ((p_b_1).wrapping_add(48)).wrapping_add((((4i32) as u32)).wrapping_mul(44)), a1_, (p_b_1).wrapping_add(668)) };
        if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (16i32))) != 0) {
            F::Flash_Play(c, ((p_b_1).wrapping_add(48)).wrapping_add((((4i32) as u32)).wrapping_mul(44)), 1i32);
            F::Flash_GotoLabel(c, ((p_b_1).wrapping_add(48)).wrapping_add((((4i32) as u32)).wrapping_mul(44)), sym::menu_k_f__str2, ((((1i32) as i64 as u64)) as u8));
        }
        { let v_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (p_b_1).wrapping_add(32); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((39i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (p_b_1).wrapping_add(28); c.m.w32(a_, v_); };
        { let a0_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = c.m.r32((p_b_1).wrapping_add(32)); let _ = 0i32; F::MsgWin_Init(c, a0_, a1_, 0i32) };
        if ((((!(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (16i32))) != 0)) as i32)) != 0) {
            F::MsgWin_Open(c);
        }
        { let a0_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((36i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::Dialog_Init(c, a0_, ((0i32) as u32), 0i32) };
        { let a0_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((42i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::GetWin_Init(c, a0_, 1i32) };
        { let v_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (p_b_1).wrapping_add(36); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (p_b_1).wrapping_add(40); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (p_b_1).wrapping_add(24); c.m.w32(a_, v_); };
        {
            l_i_2 = 0i32;
            'l2: loop {
                if !(((((l_i_2) < (165i32)) as i32)) != 0) { break 'l2; }
                'c3: {
                    { let v_ = (c.m.r32((p_b_1).wrapping_add(24))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(24))).wrapping_add(((((l_i_2).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp.wrapping_add(12)); let a2_ = c.m.r32(fp.wrapping_add(12)); F::Res_RelocateOffsets(c, fp.wrapping_add(12), a1_, a2_) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = ((c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32))).wrapping_add(((16i32) as u32)); let a_ = (p_b_1).wrapping_add(760); c.m.w32(a_, v_); };
        { let v_ = (({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) as i32); let a_ = (p_b_1).wrapping_add(756); c.m.w32(a_, (v_) as u32); };
        { let v_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((31i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (p_b_1).wrapping_add(8256); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(4))).wrapping_add((((41i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (p_b_1).wrapping_add(8260); c.m.w32(a_, v_); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Bracket_Load", &PROBE_N, &[], true); }
    r_
}

pub fn Bracket_PlayCpuMatch(c: &mut Ctx, mut p_b_1: u32, mut p_match_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bracket_PlayCpuMatch", &PROBE_N, &[(p_b_1) as u32, (p_match_2) as u32], false); }
    let mut l_win_3: i32 = 0;
    let r_ = 'f: {
    l_win_3 = 0i32;
    if ((((((c.m.r16(((p_b_1).wrapping_add(7440)).wrapping_add((((p_match_2) as u32)).wrapping_mul(16)))) as i32)) & (1i32))) != 0) {
        break 'f;
    }
    if (((((({ let a_ = ((p_b_1).wrapping_add(6760)).wrapping_add((((c.m.r16(((((p_b_1).wrapping_add(7440)).wrapping_add((((p_match_2) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)))) as u32)).wrapping_mul(40)); c.m.r16(a_) }) as i32)) & (1i32))) != 0) {
        break 'f;
    }
    if (((((({ let a_ = ((p_b_1).wrapping_add(6760)).wrapping_add((((c.m.r16(((((p_b_1).wrapping_add(7440)).wrapping_add((((p_match_2) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)))) as u32)).wrapping_mul(40)); c.m.r16(a_) }) as i32)) & (1i32))) != 0) {
        break 'f;
    }
    if (((((({ let a_ = ((p_b_1).wrapping_add(6760)).wrapping_add((((c.m.r16(((((p_b_1).wrapping_add(7440)).wrapping_add((((p_match_2) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)))) as u32)).wrapping_mul(40)); c.m.r16(a_) }) as i32)) & (2i32))) != 0) {
        l_win_3 = 0i32;
    } else {
        if (((((({ let a_ = ((p_b_1).wrapping_add(6760)).wrapping_add((((c.m.r16(((((p_b_1).wrapping_add(7440)).wrapping_add((((p_match_2) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)))) as u32)).wrapping_mul(40)); c.m.r16(a_) }) as i32)) & (2i32))) != 0) {
            l_win_3 = 1i32;
        } else {
            l_win_3 = ((F::Rand_Range(c, ((2i32) as u32))) as i32);
            { let r_ = 2i32; let a_ = ((p_b_1).wrapping_add(6760)).wrapping_add((((c.m.r16(((((p_b_1).wrapping_add(7440)).wrapping_add((((p_match_2) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((((l_win_3) ^ (1i32))) as u32)).wrapping_mul(2)))) as u32)).wrapping_mul(40)); let n_ = ((((((c.m.r16(a_)) as i32)) | (r_))) as u16); c.m.w16(a_, n_); };
        }
    }
    { let v_ = ((c.m.r16((((p_b_1).wrapping_add(7440)).wrapping_add((((p_match_2) as u32)).wrapping_mul(16))).wrapping_add(8))) as i32); let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add((((c.m.r16(((((p_b_1).wrapping_add(7440)).wrapping_add((((p_match_2) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((l_win_3) as u32)).wrapping_mul(2)))) as u32)).wrapping_mul(40))).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    if ((((((c.m.r16(((p_b_1).wrapping_add(7440)).wrapping_add((((p_match_2) as u32)).wrapping_mul(16)))) as i32)) & (4i32))) != 0) {
        { let v_ = c.m.r16(((((p_b_1).wrapping_add(7440)).wrapping_add((((p_match_2) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((l_win_3) as u32)).wrapping_mul(2))); let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add((((c.m.r16((((p_b_1).wrapping_add(7440)).wrapping_add((((p_match_2) as u32)).wrapping_mul(16))).wrapping_add(6))) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
    } else {
        if ((((((c.m.r16(((p_b_1).wrapping_add(7440)).wrapping_add((((p_match_2) as u32)).wrapping_mul(16)))) as i32)) & (8i32))) != 0) {
            { let v_ = c.m.r16(((((p_b_1).wrapping_add(7440)).wrapping_add((((p_match_2) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((l_win_3) as u32)).wrapping_mul(2))); let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add((((c.m.r16((((p_b_1).wrapping_add(7440)).wrapping_add((((p_match_2) as u32)).wrapping_mul(16))).wrapping_add(6))) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
        }
    }
    { let r_ = 1i32; let a_ = ((p_b_1).wrapping_add(7440)).wrapping_add((((p_match_2) as u32)).wrapping_mul(16)); let n_ = ((((((c.m.r16(a_)) as i32)) | (r_))) as u16); c.m.w16(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Bracket_PlayCpuMatch", &PROBE_N, &[], true); }
    r_
}

pub fn Bracket_GetViewX(c: &mut Ctx, mut p_view_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bracket_GetViewX", &PROBE_N, &[(p_view_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = p_view_1;
        match sv2 {
            0i32 => {
                break 'f 0i32;
            }
            1i32 => {
                break 'f 80i32;
            }
            2i32 => {
                break 'f 196i32;
            }
            3i32 => {
                break 'f 312i32;
            }
            4i32 => {
                break 'f 428i32;
            }
            5i32 => {
                break 'f 512i32;
            }
            6i32 => {
                break 'f 138i32;
            }
            7i32 => {
                break 'f 370i32;
            }
            8i32 => {
                break 'f 22i32;
            }
            9i32 => {
                break 'f 484i32;
            }
            10i32 => {
                break 'f 256i32;
            }
            11i32 => {
                break 'f 50i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Bracket_GetViewX", &PROBE_N, &[], true); }
    r_
}

pub fn Bracket_GetPosX(c: &mut Ctx, mut p_pos_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bracket_GetPosX", &PROBE_N, &[(p_pos_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = p_pos_1;
        match sv2 {
            0i32 => {
                break 'f 51i32;
            }
            1i32 => {
                break 'f 109i32;
            }
            2i32 => {
                break 'f 167i32;
            }
            3i32 => {
                break 'f 225i32;
            }
            4i32 => {
                break 'f 283i32;
            }
            5i32 => {
                break 'f 341i32;
            }
            6i32 => {
                break 'f 399i32;
            }
            7i32 => {
                break 'f 457i32;
            }
            8i32 => {
                break 'f 515i32;
            }
            9i32 => {
                break 'f 573i32;
            }
            10i32 => {
                break 'f 631i32;
            }
            11i32 => {
                break 'f 689i32;
            }
            12i32 => {
                break 'f 747i32;
            }
            13i32 => {
                break 'f 805i32;
            }
            14i32 => {
                break 'f 863i32;
            }
            15i32 => {
                break 'f 921i32;
            }
            16i32 => {
                break 'f 80i32;
            }
            17i32 => {
                break 'f 196i32;
            }
            18i32 => {
                break 'f 311i32;
            }
            19i32 => {
                break 'f 427i32;
            }
            20i32 => {
                break 'f 543i32;
            }
            21i32 => {
                break 'f 659i32;
            }
            22i32 => {
                break 'f 774i32;
            }
            23i32 => {
                break 'f 890i32;
            }
            24i32 => {
                break 'f 138i32;
            }
            25i32 => {
                break 'f 369i32;
            }
            26i32 => {
                break 'f 601i32;
            }
            27i32 => {
                break 'f 832i32;
            }
            28i32 => {
                break 'f 253i32;
            }
            29i32 => {
                break 'f 716i32;
            }
            30i32 => {
                break 'f 84i32;
            }
            31i32 => {
                break 'f 485i32;
            }
            32i32 => {
                break 'f 286i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Bracket_GetPosX", &PROBE_N, &[], true); }
    r_
}

pub fn Bracket_GetPosY(c: &mut Ctx, mut p_pos_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bracket_GetPosY", &PROBE_N, &[(p_pos_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = p_pos_1;
        match sv2 {
            0i32 | 1i32 | 2i32 | 3i32 | 4i32 | 5i32 | 6i32 | 7i32 | 8i32 | 9i32 | 10i32 | 11i32 | 12i32 | 13i32 | 14i32 | 15i32 => {
                break 'f 269i32;
            }
            16i32 | 17i32 | 18i32 | 19i32 | 20i32 | 21i32 | 22i32 | 23i32 => {
                break 'f 220i32;
            }
            24i32 | 25i32 | 26i32 | 27i32 => {
                break 'f 201i32;
            }
            28i32 | 29i32 => {
                break 'f 167i32;
            }
            30i32 | 31i32 => {
                break 'f 137i32;
            }
            32i32 => {
                break 'f 103i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Bracket_GetPosY", &PROBE_N, &[], true); }
    r_
}

pub fn Bracket_InitMatches(c: &mut Ctx, mut p_b_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bracket_InitMatches", &PROBE_N, &[(p_b_1) as u32], false); }
    let r_ = 'f: {
    { let _ = F::memset(c, (p_b_1).wrapping_add(7440), 0i32, 256u32); };
    { let v_ = ((0i32) as u16); let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
    { let v_ = ((1i32) as u16); let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
    { let v_ = ((2i32) as u16); let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
    { let v_ = ((3i32) as u16); let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
    { let v_ = ((4i32) as u16); let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
    { let v_ = ((5i32) as u16); let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
    { let v_ = ((6i32) as u16); let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
    { let v_ = ((7i32) as u16); let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
    { let v_ = ((8i32) as u16); let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add((((4i32) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
    { let v_ = ((9i32) as u16); let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add((((4i32) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
    { let v_ = ((10i32) as u16); let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add((((5i32) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
    { let v_ = ((11i32) as u16); let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add((((5i32) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
    { let v_ = ((12i32) as u16); let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add((((6i32) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
    { let v_ = ((13i32) as u16); let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add((((6i32) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
    { let v_ = ((14i32) as u16); let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add((((7i32) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
    { let v_ = ((15i32) as u16); let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add((((7i32) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
    { let v_ = ((16i32) as u16); let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add((((15i32) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
    { let v_ = ((4i32) as u16); let a_ = ((p_b_1).wrapping_add(7440)).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w16(a_, v_); };
    { let v_ = ((8i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(6); c.m.w16(a_, v_); };
    { let v_ = ((16i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w16(a_, v_); };
    { let v_ = ((0i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(10); c.m.w16(a_, v_); };
    { let v_ = ((0i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w16(a_, v_); };
    { let v_ = ((0i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(14); c.m.w16(a_, v_); };
    { let v_ = ((8i32) as u16); let a_ = ((p_b_1).wrapping_add(7440)).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w16(a_, v_); };
    { let v_ = ((8i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(6); c.m.w16(a_, v_); };
    { let v_ = ((17i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w16(a_, v_); };
    { let v_ = ((0i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(10); c.m.w16(a_, v_); };
    { let v_ = ((0i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w16(a_, v_); };
    { let v_ = ((0i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(14); c.m.w16(a_, v_); };
    { let v_ = ((4i32) as u16); let a_ = ((p_b_1).wrapping_add(7440)).wrapping_add((((2i32) as u32)).wrapping_mul(16)); c.m.w16(a_, v_); };
    { let v_ = ((9i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(6); c.m.w16(a_, v_); };
    { let v_ = ((18i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w16(a_, v_); };
    { let v_ = ((0i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(10); c.m.w16(a_, v_); };
    { let v_ = ((1i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w16(a_, v_); };
    { let v_ = ((0i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(14); c.m.w16(a_, v_); };
    { let v_ = ((8i32) as u16); let a_ = ((p_b_1).wrapping_add(7440)).wrapping_add((((3i32) as u32)).wrapping_mul(16)); c.m.w16(a_, v_); };
    { let v_ = ((9i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(6); c.m.w16(a_, v_); };
    { let v_ = ((19i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w16(a_, v_); };
    { let v_ = ((0i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(10); c.m.w16(a_, v_); };
    { let v_ = ((2i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w16(a_, v_); };
    { let v_ = ((0i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(14); c.m.w16(a_, v_); };
    { let v_ = ((4i32) as u16); let a_ = ((p_b_1).wrapping_add(7440)).wrapping_add((((4i32) as u32)).wrapping_mul(16)); c.m.w16(a_, v_); };
    { let v_ = ((10i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((4i32) as u32)).wrapping_mul(16))).wrapping_add(6); c.m.w16(a_, v_); };
    { let v_ = ((20i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((4i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w16(a_, v_); };
    { let v_ = ((0i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((4i32) as u32)).wrapping_mul(16))).wrapping_add(10); c.m.w16(a_, v_); };
    { let v_ = ((3i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((4i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w16(a_, v_); };
    { let v_ = ((0i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((4i32) as u32)).wrapping_mul(16))).wrapping_add(14); c.m.w16(a_, v_); };
    { let v_ = ((8i32) as u16); let a_ = ((p_b_1).wrapping_add(7440)).wrapping_add((((5i32) as u32)).wrapping_mul(16)); c.m.w16(a_, v_); };
    { let v_ = ((10i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((5i32) as u32)).wrapping_mul(16))).wrapping_add(6); c.m.w16(a_, v_); };
    { let v_ = ((21i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((5i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w16(a_, v_); };
    { let v_ = ((0i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((5i32) as u32)).wrapping_mul(16))).wrapping_add(10); c.m.w16(a_, v_); };
    { let v_ = ((4i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((5i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w16(a_, v_); };
    { let v_ = ((0i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((5i32) as u32)).wrapping_mul(16))).wrapping_add(14); c.m.w16(a_, v_); };
    { let v_ = ((4i32) as u16); let a_ = ((p_b_1).wrapping_add(7440)).wrapping_add((((6i32) as u32)).wrapping_mul(16)); c.m.w16(a_, v_); };
    { let v_ = ((11i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((6i32) as u32)).wrapping_mul(16))).wrapping_add(6); c.m.w16(a_, v_); };
    { let v_ = ((22i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((6i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w16(a_, v_); };
    { let v_ = ((0i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((6i32) as u32)).wrapping_mul(16))).wrapping_add(10); c.m.w16(a_, v_); };
    { let v_ = ((5i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((6i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w16(a_, v_); };
    { let v_ = ((0i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((6i32) as u32)).wrapping_mul(16))).wrapping_add(14); c.m.w16(a_, v_); };
    { let v_ = ((((8i32) | (2i32))) as u16); let a_ = ((p_b_1).wrapping_add(7440)).wrapping_add((((7i32) as u32)).wrapping_mul(16)); c.m.w16(a_, v_); };
    { let v_ = ((11i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((7i32) as u32)).wrapping_mul(16))).wrapping_add(6); c.m.w16(a_, v_); };
    { let v_ = ((23i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((7i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w16(a_, v_); };
    { let v_ = ((0i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((7i32) as u32)).wrapping_mul(16))).wrapping_add(10); c.m.w16(a_, v_); };
    { let v_ = ((5i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((7i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w16(a_, v_); };
    { let v_ = ((0i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((7i32) as u32)).wrapping_mul(16))).wrapping_add(14); c.m.w16(a_, v_); };
    { let v_ = ((4i32) as u16); let a_ = ((p_b_1).wrapping_add(7440)).wrapping_add((((8i32) as u32)).wrapping_mul(16)); c.m.w16(a_, v_); };
    { let v_ = ((12i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((8i32) as u32)).wrapping_mul(16))).wrapping_add(6); c.m.w16(a_, v_); };
    { let v_ = ((24i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((8i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w16(a_, v_); };
    { let v_ = ((1i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((8i32) as u32)).wrapping_mul(16))).wrapping_add(10); c.m.w16(a_, v_); };
    { let v_ = ((0i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((8i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w16(a_, v_); };
    { let v_ = ((1i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((8i32) as u32)).wrapping_mul(16))).wrapping_add(14); c.m.w16(a_, v_); };
    { let v_ = ((8i32) as u16); let a_ = ((p_b_1).wrapping_add(7440)).wrapping_add((((9i32) as u32)).wrapping_mul(16)); c.m.w16(a_, v_); };
    { let v_ = ((12i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((9i32) as u32)).wrapping_mul(16))).wrapping_add(6); c.m.w16(a_, v_); };
    { let v_ = ((25i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((9i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w16(a_, v_); };
    { let v_ = ((1i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((9i32) as u32)).wrapping_mul(16))).wrapping_add(10); c.m.w16(a_, v_); };
    { let v_ = ((6i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((9i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w16(a_, v_); };
    { let v_ = ((1i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((9i32) as u32)).wrapping_mul(16))).wrapping_add(14); c.m.w16(a_, v_); };
    { let v_ = ((4i32) as u16); let a_ = ((p_b_1).wrapping_add(7440)).wrapping_add((((10i32) as u32)).wrapping_mul(16)); c.m.w16(a_, v_); };
    { let v_ = ((13i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((10i32) as u32)).wrapping_mul(16))).wrapping_add(6); c.m.w16(a_, v_); };
    { let v_ = ((26i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((10i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w16(a_, v_); };
    { let v_ = ((1i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((10i32) as u32)).wrapping_mul(16))).wrapping_add(10); c.m.w16(a_, v_); };
    { let v_ = ((7i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((10i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w16(a_, v_); };
    { let v_ = ((1i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((10i32) as u32)).wrapping_mul(16))).wrapping_add(14); c.m.w16(a_, v_); };
    { let v_ = ((((8i32) | (2i32))) as u16); let a_ = ((p_b_1).wrapping_add(7440)).wrapping_add((((11i32) as u32)).wrapping_mul(16)); c.m.w16(a_, v_); };
    { let v_ = ((13i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((11i32) as u32)).wrapping_mul(16))).wrapping_add(6); c.m.w16(a_, v_); };
    { let v_ = ((27i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((11i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w16(a_, v_); };
    { let v_ = ((1i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((11i32) as u32)).wrapping_mul(16))).wrapping_add(10); c.m.w16(a_, v_); };
    { let v_ = ((5i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((11i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w16(a_, v_); };
    { let v_ = ((1i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((11i32) as u32)).wrapping_mul(16))).wrapping_add(14); c.m.w16(a_, v_); };
    { let v_ = ((4i32) as u16); let a_ = ((p_b_1).wrapping_add(7440)).wrapping_add((((12i32) as u32)).wrapping_mul(16)); c.m.w16(a_, v_); };
    { let v_ = ((14i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((12i32) as u32)).wrapping_mul(16))).wrapping_add(6); c.m.w16(a_, v_); };
    { let v_ = ((28i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((12i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w16(a_, v_); };
    { let v_ = ((2i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((12i32) as u32)).wrapping_mul(16))).wrapping_add(10); c.m.w16(a_, v_); };
    { let v_ = ((8i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((12i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w16(a_, v_); };
    { let v_ = ((2i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((12i32) as u32)).wrapping_mul(16))).wrapping_add(14); c.m.w16(a_, v_); };
    { let v_ = ((((8i32) | (2i32))) as u16); let a_ = ((p_b_1).wrapping_add(7440)).wrapping_add((((13i32) as u32)).wrapping_mul(16)); c.m.w16(a_, v_); };
    { let v_ = ((14i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((13i32) as u32)).wrapping_mul(16))).wrapping_add(6); c.m.w16(a_, v_); };
    { let v_ = ((29i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((13i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w16(a_, v_); };
    { let v_ = ((2i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((13i32) as u32)).wrapping_mul(16))).wrapping_add(10); c.m.w16(a_, v_); };
    { let v_ = ((9i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((13i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w16(a_, v_); };
    { let v_ = ((2i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((13i32) as u32)).wrapping_mul(16))).wrapping_add(14); c.m.w16(a_, v_); };
    { let v_ = ((((8i32) | (2i32))) as u16); let a_ = ((p_b_1).wrapping_add(7440)).wrapping_add((((14i32) as u32)).wrapping_mul(16)); c.m.w16(a_, v_); };
    { let v_ = ((15i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((14i32) as u32)).wrapping_mul(16))).wrapping_add(6); c.m.w16(a_, v_); };
    { let v_ = ((31i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((14i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w16(a_, v_); };
    { let v_ = ((3i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((14i32) as u32)).wrapping_mul(16))).wrapping_add(10); c.m.w16(a_, v_); };
    { let v_ = ((10i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((14i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w16(a_, v_); };
    { let v_ = ((3i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((14i32) as u32)).wrapping_mul(16))).wrapping_add(14); c.m.w16(a_, v_); };
    { let v_ = ((((16i32) | (2i32))) as u16); let a_ = ((p_b_1).wrapping_add(7440)).wrapping_add((((15i32) as u32)).wrapping_mul(16)); c.m.w16(a_, v_); };
    { let v_ = ((32i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((15i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w16(a_, v_); };
    { let v_ = ((4i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((15i32) as u32)).wrapping_mul(16))).wrapping_add(10); c.m.w16(a_, v_); };
    { let v_ = ((11i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((15i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w16(a_, v_); };
    { let v_ = ((4i32) as u16); let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add((((15i32) as u32)).wrapping_mul(16))).wrapping_add(14); c.m.w16(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Bracket_InitMatches", &PROBE_N, &[], true); }
    r_
}

pub fn Bracket_IsPlayerOut(c: &mut Ctx, mut p_b_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bracket_IsPlayerOut", &PROBE_N, &[(p_b_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (17i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((((((c.m.r16(((p_b_1).wrapping_add(6760)).wrapping_add((((l_i_2) as u32)).wrapping_mul(40)))) as i32)) & (1i32))) != 0) && ((((!((((((c.m.r16(((p_b_1).wrapping_add(6760)).wrapping_add((((l_i_2) as u32)).wrapping_mul(40)))) as i32)) & (2i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
                    break 'l1;
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    break 'f (((l_i_2) == (17i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Bracket_IsPlayerOut", &PROBE_N, &[], true); }
    r_
}

pub fn Bracket_FillEntrants(c: &mut Ctx, mut p_b_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bracket_FillEntrants", &PROBE_N, &[(p_b_1) as u32], false); }
    let mut l_miss_2: i32 = 0;
    let mut l_boss_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_cell_5: u32 = 0;
    let mut l_chara_6: i32 = 0;
    let mut l_j_7: i32 = 0;
    let r_ = 'f: {
    l_miss_2 = 0i32;
    l_boss_3 = (1i32).wrapping_neg();
    's1: {
        let sv8: i32 = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(132); (c.m.r32(a_) as i32) };
        match sv8 {
            0i32 => {
                l_boss_3 = (if ((F::Rand_Range(c, ((2i32) as u32))) != 0) { 66i32 } else { 56i32 });
                break 's1;
            }
            1i32 => {
                l_boss_3 = 55i32;
                break 's1;
            }
            2i32 => {
                l_boss_3 = (if ((F::Rand_Range(c, ((2i32) as u32))) != 0) { 107i32 } else { 108i32 });
                break 's1;
            }
            3i32 => {
                l_boss_3 = (if ((F::Rand_Range(c, ((2i32) as u32))) != 0) { 61i32 } else { 6i32 });
                break 's1;
            }
            4i32 => {
                l_boss_3 = 26i32;
                break 's1;
            }
            _ => {
            }
        }
    }
    {
        l_i_4 = 0i32;
        'l2: loop {
            if !(((((l_i_4) < (17i32)) as i32)) != 0) { break 'l2; }
            'c3: {
                if (((((l_i_4) < ({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(140); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                    { let _ = { let a1_ = ((c.m.r32(sym::gProgress)).wrapping_add(152)).wrapping_add((((l_i_4) as u32)).wrapping_mul(40)); F::memcpy(c, ((p_b_1).wrapping_add(6760)).wrapping_add((((l_i_4) as u32)).wrapping_mul(40)), a1_, 40u32) }; };
                } else {
                    {
                        'l4: loop {
                            'c5: {
                                l_cell_5 = (({ let a0_ = (((c.m.r32((p_b_1).wrapping_add(756)) as i32)) as u32); F::Rand_Range(c, a0_) }).wrapping_mul(36u32)).wrapping_add(c.m.r32((p_b_1).wrapping_add(760)));
                                if ((((((c.m.r32(l_cell_5) as i32)) <= (160i32)) as i32)) != 0) {
                                    l_chara_6 = (c.m.r32(l_cell_5) as i32);
                                    break 'l4;
                                }
                            }
                        }
                    }
                    if ((((((c.m.r32((l_cell_5).wrapping_add(4)) as i32)) != (0i32)) as i32)) != 0) {
                        l_chara_6 = { let a_ = ((l_cell_5).wrapping_add(8)).wrapping_add(({ let a0_ = (((c.m.r32((l_cell_5).wrapping_add(4)) as i32)) as u32); F::Rand_Range(c, a0_) }).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                    }
                    if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(136); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                        if (((((((((F::ChrTbl_GetCost(c, l_chara_6)) >= (4i32)) as i32)) != 0) && (((((l_miss_2) < (15i32)) as i32)) != 0)) as i32)) != 0) {
                            { let o_ = l_miss_2; let n_ = (o_).wrapping_add(1); l_miss_2 = n_; };
                            { let o_ = l_i_4; let n_ = (o_).wrapping_sub(1); l_i_4 = n_; };
                            break 'c3;
                        }
                    } else {
                        if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(136); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) {
                            if (((((((((F::ChrTbl_GetCost(c, l_chara_6)) >= (8i32)) as i32)) != 0) && (((((l_miss_2) < (15i32)) as i32)) != 0)) as i32)) != 0) {
                                { let o_ = l_miss_2; let n_ = (o_).wrapping_add(1); l_miss_2 = n_; };
                                { let o_ = l_i_4; let n_ = (o_).wrapping_sub(1); l_i_4 = n_; };
                                break 'c3;
                            }
                        } else {
                            if (((((((((F::ChrTbl_GetCost(c, l_chara_6)) < (7i32)) as i32)) != 0) && (((((l_miss_2) < (15i32)) as i32)) != 0)) as i32)) != 0) {
                                { let o_ = l_miss_2; let n_ = (o_).wrapping_add(1); l_miss_2 = n_; };
                                { let o_ = l_i_4; let n_ = (o_).wrapping_sub(1); l_i_4 = n_; };
                                break 'c3;
                            }
                        }
                    }
                    if (((((l_i_4) > (0i32)) as i32)) != 0) {
                        {
                            l_j_7 = 0i32;
                            'l6: loop {
                                if !(((((l_j_7) < (l_i_4)) as i32)) != 0) { break 'l6; }
                                'c7: {
                                    if ((((((c.m.r32((((p_b_1).wrapping_add(6760)).wrapping_add((((l_j_7) as u32)).wrapping_mul(40))).wrapping_add(4)) as i32)) == (l_chara_6)) as i32)) != 0) {
                                        break 'l6;
                                    }
                                }
                                { let o_ = l_j_7; let n_ = (o_).wrapping_add(1); l_j_7 = n_; };
                            }
                        }
                        if (((((l_j_7) != (l_i_4)) as i32)) != 0) {
                            { let o_ = l_i_4; let n_ = (o_).wrapping_sub(1); l_i_4 = n_; };
                            break 'c3;
                        }
                    }
                    if (((((((((l_boss_3) != ((1i32).wrapping_neg())) as i32)) != 0) && (((((l_boss_3) == (l_chara_6)) as i32)) != 0)) as i32)) != 0) {
                        { let o_ = l_i_4; let n_ = (o_).wrapping_sub(1); l_i_4 = n_; };
                        break 'c3;
                    }
                    l_miss_2 = 0i32;
                    { let v_ = l_chara_6; let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add((((l_i_4) as u32)).wrapping_mul(40))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    if (((((l_boss_3) != ((1i32).wrapping_neg())) as i32)) != 0) {
        { let v_ = 30i32; let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add((((16i32) as u32)).wrapping_mul(40))).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
        { let v_ = l_boss_3; let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add((((16i32) as u32)).wrapping_mul(40))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let r_ = 4i32; let a_ = ((p_b_1).wrapping_add(6760)).wrapping_add((((16i32) as u32)).wrapping_mul(40)); let n_ = ((((((c.m.r16(a_)) as i32)) | (r_))) as u16); c.m.w16(a_, n_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Bracket_FillEntrants", &PROBE_N, &[], true); }
    r_
}

pub fn Bracket_ShuffleEntrants(c: &mut Ctx, mut p_b_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bracket_ShuffleEntrants", &PROBE_N, &[(p_b_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_i_2: i32 = 0;
    let mut l_r_3: i32 = 0;
    let r_ = 'f: {
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (16i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_r_3 = ((F::Rand_Range(c, ((16i32) as u32))) as i32);
                    { let s_ = ((p_b_1).wrapping_add(6760)).wrapping_add((((l_r_3) as u32)).wrapping_mul(40)); let d_ = fp; rt::memcpy(c, d_, s_, 40); };
                    { let s_ = ((p_b_1).wrapping_add(6760)).wrapping_add((((l_i_2) as u32)).wrapping_mul(40)); let d_ = ((p_b_1).wrapping_add(6760)).wrapping_add((((l_r_3) as u32)).wrapping_mul(40)); rt::memcpy(c, d_, s_, 40); };
                    { let s_ = fp; let d_ = ((p_b_1).wrapping_add(6760)).wrapping_add((((l_i_2) as u32)).wrapping_mul(40)); rt::memcpy(c, d_, s_, 40); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        {
            l_i_2 = 0i32;
            'l3: loop {
                if !(((((l_i_2) < (16i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    if ((((((((!((((((c.m.r16(((p_b_1).wrapping_add(6760)).wrapping_add((((l_i_2) as u32)).wrapping_mul(40)))) as i32)) & (1i32))) != 0)) as i32)) != 0) && ((((((c.m.r16(((p_b_1).wrapping_add(6760)).wrapping_add(((((l_i_2).wrapping_add(1i32)) as u32)).wrapping_mul(40)))) as i32)) & (1i32))) != 0)) as i32)) != 0) {
                        { let s_ = ((p_b_1).wrapping_add(6760)).wrapping_add(((((l_i_2).wrapping_add(1i32)) as u32)).wrapping_mul(40)); let d_ = fp; rt::memcpy(c, d_, s_, 40); };
                        { let s_ = ((p_b_1).wrapping_add(6760)).wrapping_add((((l_i_2) as u32)).wrapping_mul(40)); let d_ = ((p_b_1).wrapping_add(6760)).wrapping_add(((((l_i_2).wrapping_add(1i32)) as u32)).wrapping_mul(40)); rt::memcpy(c, d_, s_, 40); };
                        { let s_ = fp; let d_ = ((p_b_1).wrapping_add(6760)).wrapping_add((((l_i_2) as u32)).wrapping_mul(40)); rt::memcpy(c, d_, s_, 40); };
                    }
                }
                { let r_ = 2i32; let n_ = (l_i_2).wrapping_add(r_); l_i_2 = n_; };
            }
        }
        {
            l_i_2 = 0i32;
            'l5: loop {
                if !(((((l_i_2) < (16i32)) as i32)) != 0) { break 'l5; }
                'c6: {
                    { let v_ = l_i_2; let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add((((l_i_2) as u32)).wrapping_mul(40))).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Bracket_ShuffleEntrants", &PROBE_N, &[], true); }
    r_
}

pub fn Bracket_SetChipTex(c: &mut Ctx, mut p_b_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bracket_SetChipTex", &PROBE_N, &[(p_b_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 68);
        { let v_ = 6i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
        { let v_ = 9i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
        { let v_ = 10i32; c.m.w32(fp.wrapping_add(8), (v_) as u32); }
        { let v_ = 11i32; c.m.w32(fp.wrapping_add(12), (v_) as u32); }
        { let v_ = 12i32; c.m.w32(fp.wrapping_add(16), (v_) as u32); }
        { let v_ = 13i32; c.m.w32(fp.wrapping_add(20), (v_) as u32); }
        { let v_ = 14i32; c.m.w32(fp.wrapping_add(24), (v_) as u32); }
        { let v_ = 15i32; c.m.w32(fp.wrapping_add(28), (v_) as u32); }
        { let v_ = 16i32; c.m.w32(fp.wrapping_add(32), (v_) as u32); }
        { let v_ = 17i32; c.m.w32(fp.wrapping_add(36), (v_) as u32); }
        { let v_ = 18i32; c.m.w32(fp.wrapping_add(40), (v_) as u32); }
        { let v_ = 19i32; c.m.w32(fp.wrapping_add(44), (v_) as u32); }
        { let v_ = 20i32; c.m.w32(fp.wrapping_add(48), (v_) as u32); }
        { let v_ = 21i32; c.m.w32(fp.wrapping_add(52), (v_) as u32); }
        { let v_ = 22i32; c.m.w32(fp.wrapping_add(56), (v_) as u32); }
        { let v_ = 23i32; c.m.w32(fp.wrapping_add(60), (v_) as u32); }
        { let v_ = 24i32; c.m.w32(fp.wrapping_add(64), (v_) as u32); }
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (17i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = ({ let a_ = ((c.m.r32((p_b_1).wrapping_add(24))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(24))).wrapping_add((((((c.m.r32((((p_b_1).wrapping_add(6760)).wrapping_add((((l_i_2) as u32)).wrapping_mul(40))).wrapping_add(4)) as i32)).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(328)).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Bracket_SetChipTex", &PROBE_N, &[], true); }
    r_
}

pub fn Bracket_PlayCpuMatches(c: &mut Ctx, mut p_b_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bracket_PlayCpuMatches", &PROBE_N, &[(p_b_1) as u32], false); }
    let mut l_round_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    l_round_2 = 0i32;
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (16i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((((((({ let a_ = ((p_b_1).wrapping_add(6760)).wrapping_add((((c.m.r16(((((p_b_1).wrapping_add(7440)).wrapping_add((((l_i_3) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)))) as u32)).wrapping_mul(40)); c.m.r16(a_) }) as i32)) & (1i32))) != 0) || (((((({ let a_ = ((p_b_1).wrapping_add(6760)).wrapping_add((((c.m.r16(((((p_b_1).wrapping_add(7440)).wrapping_add((((l_i_3) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)))) as u32)).wrapping_mul(40)); c.m.r16(a_) }) as i32)) & (1i32))) != 0)) as i32)) != 0) {
                    if ((((((c.m.r16(((p_b_1).wrapping_add(7440)).wrapping_add((((l_i_3) as u32)).wrapping_mul(16)))) as i32)) & (2i32))) != 0) {
                        { let o_ = l_round_2; let n_ = (o_).wrapping_add(1); l_round_2 = n_; };
                        if ((((((c.m.r32((p_b_1).wrapping_add(6724)) as i32)) < (l_round_2)) as i32)) != 0) {
                            break 'f;
                        }
                    }
                } else {
                    F::Bracket_PlayCpuMatch(c, p_b_1, l_i_3);
                    if ((((((c.m.r16(((p_b_1).wrapping_add(7440)).wrapping_add((((l_i_3) as u32)).wrapping_mul(16)))) as i32)) & (2i32))) != 0) {
                        { let o_ = l_round_2; let n_ = (o_).wrapping_add(1); l_round_2 = n_; };
                        if ((((((c.m.r32((p_b_1).wrapping_add(6724)) as i32)) < (l_round_2)) as i32)) != 0) {
                            break 'f;
                        }
                    }
                }
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Bracket_PlayCpuMatches", &PROBE_N, &[], true); }
    r_
}

pub fn Bracket_NextMatch(c: &mut Ctx, mut p_b_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bracket_NextMatch", &PROBE_N, &[(p_b_1) as u32], false); }
    let r_ = 'f: {
    'l1: loop {
        'c2: {
            if (((((({ let a_ = ((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16)); c.m.r16(a_) }) as i32)) & (2i32))) != 0) {
                { let r_ = (!(32i32)); let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_b_1).wrapping_add(6724); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                if ((((((c.m.r32((p_b_1).wrapping_add(6724)) as i32)) < (5i32)) as i32)) != 0) {
                    { let v_ = 101i32; let a_ = (p_b_1).wrapping_add(6716); c.m.w32(a_, (v_) as u32); };
                }
            }
            { let a_ = (p_b_1).wrapping_add(6728); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            if (((((({ let a_ = ((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40)); c.m.r16(a_) }) as i32)) & (1i32))) != 0) {
                break 'l1;
            }
        }
        if !((((!(((((({ let a_ = ((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40)); c.m.r16(a_) }) as i32)) & (1i32))) != 0)) as i32)) != 0) { break 'l1; }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Bracket_NextMatch", &PROBE_N, &[], true); }
    r_
}

pub fn Bracket_ApplyResult(c: &mut Ctx, mut p_b_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bracket_ApplyResult", &PROBE_N, &[(p_b_1) as u32], false); }
    let mut l_flags_2: i32 = 0;
    let mut l_res_3: u32 = 0;
    let r_ = 'f: {
    l_flags_2 = F::BattleResult_GetFlags(c);
    l_res_3 = F::BattleResult_GetPtr(c);
    if ((((l_flags_2) & (1i32))) != 0) {
        if (((((({ let a_ = ((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40)); c.m.r16(a_) }) as i32)) & (1i32))) != 0) {
            { let r_ = 2i32; let a_ = ((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40)); let n_ = ((((((c.m.r16(a_)) as i32)) | (r_))) as u16); c.m.w16(a_, n_); };
            { let v_ = 0i32; let a_ = (p_b_1).wrapping_add(740); c.m.w32(a_, (v_) as u32); };
            { let r_ = 16i32; let a_ = (p_b_1).wrapping_add(732); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            if ((F::Bracket_IsPlayerOut(c, p_b_1)) != 0) {
                { let r_ = 16384i32; let a_ = (p_b_1).wrapping_add(732); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
            { let v_ = F32(c.m.r32(((l_res_3).wrapping_add(44)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(20); c.m.w32(a_, (v_).0); };
            { let r_ = 2i32; let a_ = ((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40)); let n_ = ((((((c.m.r16(a_)) as i32)) ^ (r_))) as u16); c.m.w16(a_, n_); };
        } else {
            { let r_ = 2i32; let a_ = ((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40)); let n_ = ((((((c.m.r16(a_)) as i32)) | (r_))) as u16); c.m.w16(a_, n_); };
            { let v_ = 1i32; let a_ = (p_b_1).wrapping_add(740); c.m.w32(a_, (v_) as u32); };
            { let r_ = 16i32; let a_ = (p_b_1).wrapping_add(732); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            if ((F::Bracket_IsPlayerOut(c, p_b_1)) != 0) {
                { let r_ = 16384i32; let a_ = (p_b_1).wrapping_add(732); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
            { let v_ = F32(c.m.r32(((l_res_3).wrapping_add(44)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(20); c.m.w32(a_, (v_).0); };
            { let r_ = 2i32; let a_ = ((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40)); let n_ = ((((((c.m.r16(a_)) as i32)) ^ (r_))) as u16); c.m.w16(a_, n_); };
        }
    } else {
        if ((((l_flags_2) & (2i32))) != 0) {
            if (((((({ let a_ = ((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40)); c.m.r16(a_) }) as i32)) & (1i32))) != 0) {
                { let r_ = 2i32; let a_ = ((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40)); let n_ = ((((((c.m.r16(a_)) as i32)) | (r_))) as u16); c.m.w16(a_, n_); };
                { let v_ = 1i32; let a_ = (p_b_1).wrapping_add(740); c.m.w32(a_, (v_) as u32); };
                { let r_ = 16i32; let a_ = (p_b_1).wrapping_add(732); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                if ((F::Bracket_IsPlayerOut(c, p_b_1)) != 0) {
                    { let r_ = 16384i32; let a_ = (p_b_1).wrapping_add(732); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
                { let v_ = F32(c.m.r32(((l_res_3).wrapping_add(44)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(20); c.m.w32(a_, (v_).0); };
                { let r_ = 2i32; let a_ = ((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40)); let n_ = ((((((c.m.r16(a_)) as i32)) ^ (r_))) as u16); c.m.w16(a_, n_); };
            } else {
                { let r_ = 2i32; let a_ = ((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40)); let n_ = ((((((c.m.r16(a_)) as i32)) | (r_))) as u16); c.m.w16(a_, n_); };
                { let v_ = 0i32; let a_ = (p_b_1).wrapping_add(740); c.m.w32(a_, (v_) as u32); };
                { let r_ = 16i32; let a_ = (p_b_1).wrapping_add(732); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                if ((F::Bracket_IsPlayerOut(c, p_b_1)) != 0) {
                    { let r_ = 16384i32; let a_ = (p_b_1).wrapping_add(732); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
                { let v_ = F32(c.m.r32(((l_res_3).wrapping_add(44)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(20); c.m.w32(a_, (v_).0); };
                { let r_ = 2i32; let a_ = ((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40)); let n_ = ((((((c.m.r16(a_)) as i32)) ^ (r_))) as u16); c.m.w16(a_, n_); };
            }
        }
    }
    { let r_ = (!(8i32)); let a_ = ((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40)); let n_ = ((((((c.m.r16(a_)) as i32)) & (r_))) as u16); c.m.w16(a_, n_); };
    { let r_ = (!(8i32)); let a_ = ((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40)); let n_ = ((((((c.m.r16(a_)) as i32)) & (r_))) as u16); c.m.w16(a_, n_); };
    { let r_ = 1i32; let a_ = ((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16)); let n_ = ((((((c.m.r16(a_)) as i32)) | (r_))) as u16); c.m.w16(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Bracket_ApplyResult", &PROBE_N, &[], true); }
    r_
}

pub fn Bracket_StartVs(c: &mut Ctx, mut p_b_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bracket_StartVs", &PROBE_N, &[(p_b_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_flash_2: u32 = 0;
    let mut l_res_3: u32 = 0;
    let r_ = 'f: {
        l_flash_2 = ((p_b_1).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(44));
        { let v_ = 1i32; let a_ = (p_b_1).wrapping_add(6720); c.m.w32(a_, (v_) as u32); };
        { let r_ = 64i32; let a_ = (p_b_1).wrapping_add(732); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        l_res_3 = (c.m.r32((p_b_1).wrapping_add(24))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(24))).wrapping_add((((({ let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
        { let v_ = (c.m.r32((l_res_3).wrapping_add(16))).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(620)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        l_res_3 = (c.m.r32((p_b_1).wrapping_add(24))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(24))).wrapping_add((((({ let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
        { let v_ = (c.m.r32((l_res_3).wrapping_add(16))).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(620)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = rt::fneg(rt::i2f({ let a0_ = (({ let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.r16(a_) }) as i32); F::Bracket_GetViewX(c, a0_) })); let a_ = (p_b_1).wrapping_add(6740); c.m.w32(a_, (v_).0); };
        { let a1_ = rt::f2i(F32(c.m.r32((p_b_1).wrapping_add(6740)))); F::Flash_SetOffset(c, ((p_b_1).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), a1_, 0i32) };
        { let r_ = 8i32; let a_ = ((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40)); let n_ = ((((((c.m.r16(a_)) as i32)) | (r_))) as u16); c.m.w16(a_, n_); };
        { let r_ = 8i32; let a_ = ((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40)); let n_ = ((((((c.m.r16(a_)) as i32)) | (r_))) as u16); c.m.w16(a_, n_); };
        F::Flash_Play(c, l_flash_2, 1i32);
        { let _ = { let a2_ = ((({ let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(10); c.m.r16(a_) }) as i32)).wrapping_add(1i32); F::sprintf(c, fp, sym::menu_k_f__str3, &[((a2_) as i64 as u64)]) }; };
        F::Flash_GotoLabel(c, l_flash_2, fp, ((((1i32) as i64 as u64)) as u8));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Bracket_StartVs", &PROBE_N, &[], true); }
    r_
}

pub fn Bracket_StartWin(c: &mut Ctx, mut p_b_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bracket_StartWin", &PROBE_N, &[(p_b_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_flash_2: u32 = 0;
    let mut l_res_3: u32 = 0;
    let r_ = 'f: {
        l_flash_2 = ((p_b_1).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(44));
        { let r_ = 32i32; let a_ = (p_b_1).wrapping_add(732); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        l_res_3 = (c.m.r32((p_b_1).wrapping_add(24))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(24))).wrapping_add((((({ let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
        { let v_ = (c.m.r32((l_res_3).wrapping_add(16))).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        l_res_3 = (c.m.r32((p_b_1).wrapping_add(24))).wrapping_add((({ let a_ = (c.m.r32((p_b_1).wrapping_add(24))).wrapping_add((((({ let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
        { let v_ = (c.m.r32((l_res_3).wrapping_add(16))).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(644)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = rt::fneg(rt::i2f({ let a0_ = (({ let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.r16(a_) }) as i32); F::Bracket_GetViewX(c, a0_) })); let a_ = (p_b_1).wrapping_add(6740); c.m.w32(a_, (v_).0); };
        { let a1_ = rt::f2i(F32(c.m.r32((p_b_1).wrapping_add(6740)))); F::Flash_SetOffset(c, ((p_b_1).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), a1_, 0i32) };
        { let r_ = 8i32; let a_ = ((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40)); let n_ = ((((((c.m.r16(a_)) as i32)) | (r_))) as u16); c.m.w16(a_, n_); };
        { let r_ = 8i32; let a_ = ((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40)); let n_ = ((((((c.m.r16(a_)) as i32)) | (r_))) as u16); c.m.w16(a_, n_); };
        F::Flash_Play(c, l_flash_2, 1i32);
        { let _ = { let a1_ = (if ((((((c.m.r32((p_b_1).wrapping_add(740)) as i32)) != (0i32)) as i32)) != 0) { sym::menu_k_f__str4 } else { sym::menu_k_f__str5 }); let a2_ = ((({ let a_ = (((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(14); c.m.r16(a_) }) as i32)).wrapping_add(1i32); F::sprintf(c, fp, a1_, &[((a2_) as i64 as u64)]) }; };
        F::Flash_GotoLabel(c, l_flash_2, fp, ((((1i32) as i64 as u64)) as u8));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Bracket_StartWin", &PROBE_N, &[], true); }
    r_
}

pub fn Bracket_UpdateImages(c: &mut Ctx, mut p_b_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bracket_UpdateImages", &PROBE_N, &[(p_b_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
        l_p_2 = fp;
        { let _ = F::memset(c, l_p_2, 0i32, ((8i32) as u32)); };
        's1: {
            let sv3: i32 = (c.m.r32((p_b_1).wrapping_add(6720)) as i32);
            match sv3 {
                1i32 => {
                    { let v_ = ((0i32) as u32); let a_ = ((p_b_1).wrapping_add(668)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ((0i32) as u32); let a_ = ((p_b_1).wrapping_add(668)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = { let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(4); (c.m.r32(a_) as i32) }; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(4); (c.m.r32(a_) as i32) }; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    F::File_CancelRequests(c);
                    { let _ = { let a0_ = ((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)).wrapping_add(761i32); let a1_ = c.m.r32(((p_b_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4))); let _ = 92160i32; F::File_Request(c, a0_, a1_) }; };
                    { let _ = { let a0_ = ((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)).wrapping_add(761i32); let a1_ = c.m.r32(((p_b_1).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4))); let _ = 92160i32; F::File_Request(c, a0_, a1_) }; };
                    { let v_ = 2i32; let a_ = (p_b_1).wrapping_add(6720); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                2i32 => {
                    if ((F::File_UpdateRequests(c)) != 0) {
                        { let v_ = 3i32; let a_ = (p_b_1).wrapping_add(6720); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                3i32 => {
                    { let _ = { let a0_ = c.m.r32(((p_b_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4))); let a1_ = c.m.r32(((p_b_1).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4))); F::Sprite_Unpack(c, a0_, a1_, ((0i32) as u32)) }; };
                    { let _ = { let a0_ = c.m.r32(((p_b_1).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4))); let a1_ = c.m.r32(((p_b_1).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4))); F::Sprite_Unpack(c, a0_, a1_, ((0i32) as u32)) }; };
                    { let v_ = c.m.r32(((p_b_1).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4))); let a_ = fp.wrapping_add(8); c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp.wrapping_add(8)); let a2_ = c.m.r32(fp.wrapping_add(8)); F::Res_RelocateOffsets(c, fp.wrapping_add(8), a1_, a2_) };
                    { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(8))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(668)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = c.m.r32(((p_b_1).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4))); let a_ = fp.wrapping_add(8); c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp.wrapping_add(8)); let a2_ = c.m.r32(fp.wrapping_add(8)); F::Res_RelocateOffsets(c, fp.wrapping_add(8), a1_, a2_) };
                    { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(8))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(668)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let r_ = 512i32; let a_ = (p_b_1).wrapping_add(732); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_b_1).wrapping_add(6720); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                _ => {
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Bracket_UpdateImages", &PROBE_N, &[], true); }
    r_
}

pub fn Bracket_LoadImages(c: &mut Ctx, mut p_b_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bracket_LoadImages", &PROBE_N, &[(p_b_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
        l_p_2 = fp;
        { let _ = F::memset(c, l_p_2, 0i32, ((8i32) as u32)); };
        { let v_ = { let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(4); (c.m.r32(a_) as i32) }; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(4); (c.m.r32(a_) as i32) }; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let _ = { let a0_ = ((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)).wrapping_add(761i32); let a1_ = c.m.r32(((p_b_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4))); F::File_LoadSync(c, a0_, a1_, 92160i32) }; };
        { let _ = { let a0_ = ((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)).wrapping_add(761i32); let a1_ = c.m.r32(((p_b_1).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4))); F::File_LoadSync(c, a0_, a1_, 92160i32) }; };
        { let _ = { let a0_ = c.m.r32(((p_b_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4))); let a1_ = c.m.r32(((p_b_1).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4))); F::Sprite_Unpack(c, a0_, a1_, ((0i32) as u32)) }; };
        { let _ = { let a0_ = c.m.r32(((p_b_1).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4))); let a1_ = c.m.r32(((p_b_1).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4))); F::Sprite_Unpack(c, a0_, a1_, ((0i32) as u32)) }; };
        { let v_ = c.m.r32(((p_b_1).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4))); let a_ = fp.wrapping_add(8); c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp.wrapping_add(8)); let a2_ = c.m.r32(fp.wrapping_add(8)); F::Res_RelocateOffsets(c, fp.wrapping_add(8), a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(8))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(668)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = c.m.r32(((p_b_1).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4))); let a_ = fp.wrapping_add(8); c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp.wrapping_add(8)); let a2_ = c.m.r32(fp.wrapping_add(8)); F::Res_RelocateOffsets(c, fp.wrapping_add(8), a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(8))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_b_1).wrapping_add(668)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Bracket_LoadImages", &PROBE_N, &[], true); }
    r_
}

pub fn Bracket_PickStage(c: &mut Ctx, mut p_yamcha_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bracket_PickStage", &PROBE_N, &[(p_yamcha_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(272) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 272);
    let mut l_count_2: i32 = 0;
    let mut l_i_3: u32 = 0;
    let r_ = 'f: {
        l_count_2 = 0i32;
        if ((p_yamcha_1) != 0) {
            rt::memset(c, fp.wrapping_add(144), 0, 32);
            { let v_ = 2i32; c.m.w32(fp.wrapping_add(144).wrapping_add(0), (v_) as u32); }
            { let v_ = 3i32; c.m.w32(fp.wrapping_add(144).wrapping_add(4), (v_) as u32); }
            { let v_ = 7i32; c.m.w32(fp.wrapping_add(144).wrapping_add(8), (v_) as u32); }
            { let v_ = 8i32; c.m.w32(fp.wrapping_add(144).wrapping_add(12), (v_) as u32); }
            { let v_ = 13i32; c.m.w32(fp.wrapping_add(144).wrapping_add(16), (v_) as u32); }
            { let v_ = 16i32; c.m.w32(fp.wrapping_add(144).wrapping_add(20), (v_) as u32); }
            { let v_ = 18i32; c.m.w32(fp.wrapping_add(144).wrapping_add(24), (v_) as u32); }
            { let v_ = 34i32; c.m.w32(fp.wrapping_add(144).wrapping_add(28), (v_) as u32); }
            {
                l_i_3 = ((0i32) as u32);
                'l1: loop {
                    if !(((((l_i_3) < (((8i32) as u32))) as i32)) != 0) { break 'l1; }
                    'c2: {
                        if ((((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(3104); c.m.r64(a_) }).wrapping_shr(((c.m.r32((fp.wrapping_add(144)).wrapping_add((l_i_3).wrapping_mul(4))) as i32)) as u32)) & (((1i32) as u64)))) != 0) {
                            { let v_ = (c.m.r32((fp.wrapping_add(144)).wrapping_add((l_i_3).wrapping_mul(4))) as i32); let a_ = (fp).wrapping_add((((l_count_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            { let o_ = l_count_2; let n_ = (o_).wrapping_add(1); l_count_2 = n_; };
                        }
                    }
                    { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                }
            }
            break 'f { let a_ = (fp).wrapping_add((F::Rand_Range(c, ((l_count_2) as u32))).wrapping_mul(4)); (c.m.r32(a_) as i32) };
        } else {
            rt::memset(c, fp.wrapping_add(176), 0, 92);
            { let v_ = 0i32; c.m.w32(fp.wrapping_add(176).wrapping_add(0), (v_) as u32); }
            { let v_ = 1i32; c.m.w32(fp.wrapping_add(176).wrapping_add(4), (v_) as u32); }
            { let v_ = 5i32; c.m.w32(fp.wrapping_add(176).wrapping_add(8), (v_) as u32); }
            { let v_ = 9i32; c.m.w32(fp.wrapping_add(176).wrapping_add(12), (v_) as u32); }
            { let v_ = 10i32; c.m.w32(fp.wrapping_add(176).wrapping_add(16), (v_) as u32); }
            { let v_ = 11i32; c.m.w32(fp.wrapping_add(176).wrapping_add(20), (v_) as u32); }
            { let v_ = 12i32; c.m.w32(fp.wrapping_add(176).wrapping_add(24), (v_) as u32); }
            { let v_ = 14i32; c.m.w32(fp.wrapping_add(176).wrapping_add(28), (v_) as u32); }
            { let v_ = 15i32; c.m.w32(fp.wrapping_add(176).wrapping_add(32), (v_) as u32); }
            { let v_ = 17i32; c.m.w32(fp.wrapping_add(176).wrapping_add(36), (v_) as u32); }
            { let v_ = 19i32; c.m.w32(fp.wrapping_add(176).wrapping_add(40), (v_) as u32); }
            { let v_ = 20i32; c.m.w32(fp.wrapping_add(176).wrapping_add(44), (v_) as u32); }
            { let v_ = 21i32; c.m.w32(fp.wrapping_add(176).wrapping_add(48), (v_) as u32); }
            { let v_ = 22i32; c.m.w32(fp.wrapping_add(176).wrapping_add(52), (v_) as u32); }
            { let v_ = 23i32; c.m.w32(fp.wrapping_add(176).wrapping_add(56), (v_) as u32); }
            { let v_ = 24i32; c.m.w32(fp.wrapping_add(176).wrapping_add(60), (v_) as u32); }
            { let v_ = 25i32; c.m.w32(fp.wrapping_add(176).wrapping_add(64), (v_) as u32); }
            { let v_ = 26i32; c.m.w32(fp.wrapping_add(176).wrapping_add(68), (v_) as u32); }
            { let v_ = 29i32; c.m.w32(fp.wrapping_add(176).wrapping_add(72), (v_) as u32); }
            { let v_ = 30i32; c.m.w32(fp.wrapping_add(176).wrapping_add(76), (v_) as u32); }
            { let v_ = 31i32; c.m.w32(fp.wrapping_add(176).wrapping_add(80), (v_) as u32); }
            { let v_ = 32i32; c.m.w32(fp.wrapping_add(176).wrapping_add(84), (v_) as u32); }
            { let v_ = 33i32; c.m.w32(fp.wrapping_add(176).wrapping_add(88), (v_) as u32); }
            {
                l_i_3 = ((0i32) as u32);
                'l3: loop {
                    if !(((((l_i_3) < (((23i32) as u32))) as i32)) != 0) { break 'l3; }
                    'c4: {
                        if ((((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(3104); c.m.r64(a_) }).wrapping_shr(((c.m.r32((fp.wrapping_add(176)).wrapping_add((l_i_3).wrapping_mul(4))) as i32)) as u32)) & (((1i32) as u64)))) != 0) {
                            { let v_ = (c.m.r32((fp.wrapping_add(176)).wrapping_add((l_i_3).wrapping_mul(4))) as i32); let a_ = (fp).wrapping_add((((l_count_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            { let o_ = l_count_2; let n_ = (o_).wrapping_add(1); l_count_2 = n_; };
                        }
                    }
                    { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                }
            }
            break 'f { let a_ = (fp).wrapping_add((F::Rand_Range(c, ((l_count_2) as u32))).wrapping_mul(4)); (c.m.r32(a_) as i32) };
        }
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Bracket_PickStage", &PROBE_N, &[], true); }
    r_
}

pub fn Bracket_SetupBattle(c: &mut Ctx, mut p_b_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bracket_SetupBattle", &PROBE_N, &[(p_b_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_stage_2: i32 = 0;
    let mut l_bgm_3: i32 = 0;
    let mut l_announcer_4: i32 = 0;
    let mut l_screenMode_5: i32 = 0;
    let mut l_i_6: i32 = 0;
    let r_ = 'f: {
        l_stage_2 = 0i32;
        l_bgm_3 = 0i32;
        l_announcer_4 = 0i32;
        F::Battle_ClearWork(c);
        if (((((({ let a_ = ((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40)); c.m.r16(a_) }) as i32)) & (1i32))) != 0) {
            if (((((({ let a_ = ((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40)); c.m.r16(a_) }) as i32)) & (1i32))) != 0) {
                F::BattleSetup_SetSide(c, 0i32, 0i32, 0i32, 1i32, 1i32, 1i32, 0i32, ((((0i32) as i64 as u64)) as u32));
                F::BattleSetup_SetSide(c, 1i32, 0i32, 1i32, 1i32, 1i32, 1i32, 0i32, ((((0i32) as i64 as u64)) as u32));
                { let r_ = 32768i32; let a_ = (p_b_1).wrapping_add(732); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = { let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(4); (c.m.r32(a_) as i32) }; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = { let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(4); (c.m.r32(a_) as i32) }; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = { let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(8); (c.m.r32(a_) as i32) }; let a_ = (fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = { let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(8); (c.m.r32(a_) as i32) }; let a_ = (fp.wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let s_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(24); let d_ = (fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
                { let s_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(24); let d_ = (fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
                l_screenMode_5 = 1i32;
                if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(132); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) {
                    { let v_ = (if ((((((c.m.r32((p_b_1).wrapping_add(6724)) as i32)) != (0i32)) as i32)) != 0) { rt::fadd({ let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(20); F32(c.m.r32(a_)) }, F32(0x41a00000)) } else { F32(0x42c80000) }); let a_ = (fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = (if ((((((c.m.r32((p_b_1).wrapping_add(6724)) as i32)) != (0i32)) as i32)) != 0) { rt::fadd({ let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(20); F32(c.m.r32(a_)) }, F32(0x41a00000)) } else { F32(0x42c80000) }); let a_ = (fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    if (((rt::fgt(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0x42c80000)) as i32)) != 0) {
                        { let v_ = F32(0x42c80000); let a_ = (fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    }
                    if (((rt::fgt(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(0x42c80000)) as i32)) != 0) {
                        { let v_ = F32(0x42c80000); let a_ = (fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    }
                } else {
                    { let v_ = F32(0x42c80000); let a_ = (fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x42c80000); let a_ = (fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                }
            } else {
                F::BattleSetup_SetSide(c, 0i32, 0i32, 0i32, 1i32, 1i32, 1i32, 0i32, ((((0i32) as i64 as u64)) as u32));
                F::BattleSetup_SetSide(c, 1i32, 2i32, 1i32, 1i32, 1i32, 1i32, 0i32, ((((0i32) as i64 as u64)) as u32));
                { let r_ = 65536i32; let a_ = (p_b_1).wrapping_add(732); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                l_screenMode_5 = 0i32;
                { let v_ = { let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(4); (c.m.r32(a_) as i32) }; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = { let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(4); (c.m.r32(a_) as i32) }; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = { let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(8); (c.m.r32(a_) as i32) }; let a_ = (fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = { let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(8); (c.m.r32(a_) as i32) }; let a_ = (fp.wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let s_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(24); let d_ = (fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
                { let s_ = ((c.m.r32((p_b_1).wrapping_add(8256))).wrapping_add(((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(136); (c.m.r32(a_) as i32) }).wrapping_mul(5i32)).wrapping_add((c.m.r32((p_b_1).wrapping_add(6724)) as i32))) as u32)).wrapping_mul(20))).wrapping_add(4); let d_ = (fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
                {
                    l_i_6 = 0i32;
                    'l1: loop {
                        if !(((((l_i_6) < (8i32)) as i32)) != 0) { break 'l1; }
                        'c2: {
                            { let a_ = ((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((l_i_6) as u32)).wrapping_mul(2)); let o_ = c.m.r16(a_); let n_ = (o_).wrapping_add(1); c.m.w16(a_, n_); };
                        }
                        { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                    }
                }
                if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(132); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) {
                    { let v_ = (if ((((((c.m.r32((p_b_1).wrapping_add(6724)) as i32)) != (0i32)) as i32)) != 0) { rt::fadd({ let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(20); F32(c.m.r32(a_)) }, F32(0x41a00000)) } else { F32(0x42c80000) }); let a_ = (fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x42c80000); let a_ = (fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    if (((rt::fgt(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0x42c80000)) as i32)) != 0) {
                        { let v_ = F32(0x42c80000); let a_ = (fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    }
                } else {
                    { let v_ = F32(0x42c80000); let a_ = (fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x42c80000); let a_ = (fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                }
            }
        } else {
            F::BattleSetup_SetSide(c, 0i32, 0i32, 0i32, 1i32, 1i32, 1i32, 0i32, ((((0i32) as i64 as u64)) as u32));
            F::BattleSetup_SetSide(c, 1i32, 2i32, 1i32, 1i32, 1i32, 1i32, 0i32, ((((0i32) as i64 as u64)) as u32));
            { let r_ = 131072i32; let a_ = (p_b_1).wrapping_add(732); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            l_screenMode_5 = 0i32;
            { let v_ = { let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(4); (c.m.r32(a_) as i32) }; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = { let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(4); (c.m.r32(a_) as i32) }; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = { let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(8); (c.m.r32(a_) as i32) }; let a_ = (fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = { let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(8); (c.m.r32(a_) as i32) }; let a_ = (fp.wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let s_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(24); let d_ = (fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
            { let s_ = ((c.m.r32((p_b_1).wrapping_add(8256))).wrapping_add(((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(136); (c.m.r32(a_) as i32) }).wrapping_mul(5i32)).wrapping_add((c.m.r32((p_b_1).wrapping_add(6724)) as i32))) as u32)).wrapping_mul(20))).wrapping_add(4); let d_ = (fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
            {
                l_i_6 = 0i32;
                'l3: loop {
                    if !(((((l_i_6) < (8i32)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        { let a_ = ((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((l_i_6) as u32)).wrapping_mul(2)); let o_ = c.m.r16(a_); let n_ = (o_).wrapping_add(1); c.m.w16(a_, n_); };
                    }
                    { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                }
            }
            if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(132); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) {
                { let v_ = (if ((((((c.m.r32((p_b_1).wrapping_add(6724)) as i32)) != (0i32)) as i32)) != 0) { rt::fadd({ let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(20); F32(c.m.r32(a_)) }, F32(0x41a00000)) } else { F32(0x42c80000) }); let a_ = (fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x42c80000); let a_ = (fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                if (((rt::fgt(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0x42c80000)) as i32)) != 0) {
                    { let v_ = F32(0x42c80000); let a_ = (fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                }
            } else {
                { let v_ = F32(0x42c80000); let a_ = (fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x42c80000); let a_ = (fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            }
        }
        's5: {
            let sv7: i32 = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(132); (c.m.r32(a_) as i32) };
            match sv7 {
                0i32 => {
                    l_stage_2 = (if ((((((c.m.r32((p_b_1).wrapping_add(6724)) as i32)) == (4i32)) as i32)) != 0) { 27i32 } else { 4i32 });
                    l_announcer_4 = 4i32;
                    l_bgm_3 = 18i32;
                    break 's5;
                }
                1i32 => {
                    l_announcer_4 = 5i32;
                    l_stage_2 = F::Bracket_PickStage(c, 0i32);
                    l_bgm_3 = (((F::Rand_Range(c, ((9i32) as u32))).wrapping_add(((8i32) as u32))) as i32);
                    break 's5;
                }
                2i32 => {
                    l_stage_2 = (if ((((((c.m.r32((p_b_1).wrapping_add(6724)) as i32)) == (4i32)) as i32)) != 0) { 6i32 } else { 28i32 });
                    l_announcer_4 = 6i32;
                    l_bgm_3 = (((F::Rand_Range(c, ((9i32) as u32))).wrapping_add(((8i32) as u32))) as i32);
                    break 's5;
                }
                3i32 => {
                    l_announcer_4 = 2i32;
                    l_stage_2 = F::Bracket_PickStage(c, 0i32);
                    l_bgm_3 = (((F::Rand_Range(c, ((9i32) as u32))).wrapping_add(((8i32) as u32))) as i32);
                    break 's5;
                }
                4i32 => {
                    l_announcer_4 = 3i32;
                    l_stage_2 = F::Bracket_PickStage(c, 1i32);
                    l_bgm_3 = (((F::Rand_Range(c, ((9i32) as u32))).wrapping_add(((8i32) as u32))) as i32);
                    break 's5;
                }
                _ => {
                }
            }
        }
        F::BattleSetup_SetRule(c, l_screenMode_5, 4i32, l_bgm_3, 0i32, l_announcer_4, l_stage_2, 1i32);
        {
            l_i_6 = 0i32;
            'l6: loop {
                if !(((((l_i_6) < (2i32)) as i32)) != 0) { break 'l6; }
                'c7: {
                    { let a2_ = (c.m.r32((fp).wrapping_add((((l_i_6) as u32)).wrapping_mul(4))) as i32); let a3_ = (c.m.r32((fp.wrapping_add(8)).wrapping_add((((l_i_6) as u32)).wrapping_mul(4))) as i32); let a5_ = { let a_ = (c.m.r32((p_b_1).wrapping_add(8256))).wrapping_add(((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(136); (c.m.r32(a_) as i32) }).wrapping_mul(5i32)).wrapping_add((c.m.r32((p_b_1).wrapping_add(6724)) as i32))) as u32)).wrapping_mul(20)); (c.m.r32(a_) as i32) }; let a6_ = F32(c.m.r32((fp.wrapping_add(48)).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)))); F::BattleSetup_SetMember(c, l_i_6, 0i32, a2_, a3_, 0i32, a5_, a6_, (fp.wrapping_add(16)).wrapping_add((((l_i_6) as u32)).wrapping_mul(16))) };
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
        F::BattleSetup_Finish(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Bracket_SetupBattle", &PROBE_N, &[], true); }
    r_
}

pub fn Bracket_GivePrizes(c: &mut Ctx, mut p_b_1: u32, mut p_second_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bracket_GivePrizes", &PROBE_N, &[(p_b_1) as u32, (p_second_2) as u32], false); }
    let mut l_value_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    { let _ = F::memset(c, (p_b_1).wrapping_add(8264), 0i32, 64u32); };
    { let v_ = 0i32; let a_ = (p_b_1).wrapping_add(8328); c.m.w32(a_, (v_) as u32); };
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (7i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_value_3 = { let a_ = ((((c.m.r32((p_b_1).wrapping_add(8260))).wrapping_add(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(132); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(100))).wrapping_add(16)).wrapping_add((((l_i_4) as u32)).wrapping_mul(12))).wrapping_add(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(136); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                if (((((l_value_3) < (0i32)) as i32)) != 0) {
                    break 'c2;
                }
                if (((((p_second_2) == (0i32)) as i32)) != 0) {
                    's3: {
                        let sv5: i32 = l_i_4;
                        match sv5 {
                            0i32 => {
                                { let v_ = 6i32; let a_ = ((p_b_1).wrapping_add(8264)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(8328)) as i32)) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_) as u32); };
                                { let v_ = l_value_3; let a_ = (((p_b_1).wrapping_add(8264)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(8328)) as i32)) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                { let a_ = (p_b_1).wrapping_add(8328); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                F::Save_AddMoney(c, l_value_3);
                                break 's3;
                            }
                            2i32 => {
                                if ((((!(((((({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(11968)).wrapping_add(((l_value_3) as u32)); c.m.r8(a_) }) as i32)) & (1i32))) != 0)) as i32)) != 0) {
                                    { let v_ = 2i32; let a_ = ((p_b_1).wrapping_add(8264)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(8328)) as i32)) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = l_value_3; let a_ = (((p_b_1).wrapping_add(8264)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(8328)) as i32)) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                    { let a_ = (p_b_1).wrapping_add(8328); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                    F::Save_AddItem(c, l_value_3);
                                }
                                break 's3;
                            }
                            3i32 | 4i32 => {
                                if ((((!((((((({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(3080)).wrapping_add((((rt::div_i32(l_value_3, 64i32)) as u32)).wrapping_mul(8)); c.m.r64(a_) }).wrapping_shr((rt::rem_i32(l_value_3, 64i32)) as u32)) & (((1i32) as u64)))) as i32)) != 0)) as i32)) != 0) {
                                    { let v_ = 0i32; let a_ = ((p_b_1).wrapping_add(8264)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(8328)) as i32)) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = l_value_3; let a_ = (((p_b_1).wrapping_add(8264)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(8328)) as i32)) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                    { let a_ = (p_b_1).wrapping_add(8328); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                    { let r_ = (((1i64).wrapping_shl((rt::rem_i32(l_value_3, 64i32)) as u32)) as u64); let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(3080)).wrapping_add((((rt::div_i32(l_value_3, 64i32)) as u32)).wrapping_mul(8)); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
                                }
                                break 's3;
                            }
                            5i32 => {
                                if ((((!((((((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(3104); c.m.r64(a_) }).wrapping_shr((l_value_3) as u32)) & (((1i32) as u64)))) as i32)) != 0)) as i32)) != 0) {
                                    { let v_ = 1i32; let a_ = ((p_b_1).wrapping_add(8264)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(8328)) as i32)) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = l_value_3; let a_ = (((p_b_1).wrapping_add(8264)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(8328)) as i32)) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                    { let a_ = (p_b_1).wrapping_add(8328); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                    { let r_ = (((1i64).wrapping_shl((l_value_3) as u32)) as u64); let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(3104); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
                                }
                                break 's3;
                            }
                            1i32 | 6i32 => {
                                break 's3;
                            }
                            _ => {
                            }
                        }
                    }
                } else {
                    if (((((l_i_4) == (1i32)) as i32)) != 0) {
                        { let v_ = 6i32; let a_ = ((p_b_1).wrapping_add(8264)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(8328)) as i32)) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_) as u32); };
                        { let v_ = l_value_3; let a_ = (((p_b_1).wrapping_add(8264)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(8328)) as i32)) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let a_ = (p_b_1).wrapping_add(8328); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        F::Save_AddMoney(c, l_value_3);
                    }
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    l_value_3 = { let a0_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(132); (c.m.r32(a_) as i32) }; let a1_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(136); (c.m.r32(a_) as i32) }; F::Bracket_DrawDragonBall(c, a0_, a1_, p_second_2) };
    if (((((l_value_3) >= (0i32)) as i32)) != 0) {
        { let v_ = 9i32; let a_ = ((p_b_1).wrapping_add(8264)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(8328)) as i32)) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_) as u32); };
        { let v_ = l_value_3; let a_ = (((p_b_1).wrapping_add(8264)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(8328)) as i32)) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let a_ = (p_b_1).wrapping_add(8328); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        { let r_ = (1i32).wrapping_shl((l_value_3) as u32); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(8); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Bracket_GivePrizes", &PROBE_N, &[], true); }
    r_
}

pub fn Bracket_DrawDragonBall(c: &mut Ctx, mut p_tour_1: i32, mut p_level_2: i32, mut p_second_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bracket_DrawDragonBall", &PROBE_N, &[(p_tour_1) as u32, (p_level_2) as u32, (p_second_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(160) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 160);
    let mut l_count_4: i32 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
        l_count_4 = 0i32;
        rt::memset(c, fp.wrapping_add(32), 0, 120);
        rt::memset(c, fp.wrapping_add(32).wrapping_add(0), 0, 24);
        rt::memset(c, fp.wrapping_add(32).wrapping_add(0).wrapping_add(0), 0, 12);
        { let v_ = 20i32; c.m.w32(fp.wrapping_add(32).wrapping_add(0).wrapping_add(0).wrapping_add(0), (v_) as u32); }
        { let v_ = 30i32; c.m.w32(fp.wrapping_add(32).wrapping_add(0).wrapping_add(0).wrapping_add(4), (v_) as u32); }
        { let v_ = 40i32; c.m.w32(fp.wrapping_add(32).wrapping_add(0).wrapping_add(0).wrapping_add(8), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(32).wrapping_add(0).wrapping_add(12), 0, 12);
        { let v_ = 5i32; c.m.w32(fp.wrapping_add(32).wrapping_add(0).wrapping_add(12).wrapping_add(0), (v_) as u32); }
        { let v_ = 10i32; c.m.w32(fp.wrapping_add(32).wrapping_add(0).wrapping_add(12).wrapping_add(4), (v_) as u32); }
        { let v_ = 15i32; c.m.w32(fp.wrapping_add(32).wrapping_add(0).wrapping_add(12).wrapping_add(8), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(32).wrapping_add(24), 0, 24);
        rt::memset(c, fp.wrapping_add(32).wrapping_add(24).wrapping_add(0), 0, 12);
        { let v_ = 20i32; c.m.w32(fp.wrapping_add(32).wrapping_add(24).wrapping_add(0).wrapping_add(0), (v_) as u32); }
        { let v_ = 30i32; c.m.w32(fp.wrapping_add(32).wrapping_add(24).wrapping_add(0).wrapping_add(4), (v_) as u32); }
        { let v_ = 40i32; c.m.w32(fp.wrapping_add(32).wrapping_add(24).wrapping_add(0).wrapping_add(8), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(32).wrapping_add(24).wrapping_add(12), 0, 12);
        { let v_ = 5i32; c.m.w32(fp.wrapping_add(32).wrapping_add(24).wrapping_add(12).wrapping_add(0), (v_) as u32); }
        { let v_ = 10i32; c.m.w32(fp.wrapping_add(32).wrapping_add(24).wrapping_add(12).wrapping_add(4), (v_) as u32); }
        { let v_ = 15i32; c.m.w32(fp.wrapping_add(32).wrapping_add(24).wrapping_add(12).wrapping_add(8), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(32).wrapping_add(48), 0, 24);
        rt::memset(c, fp.wrapping_add(32).wrapping_add(48).wrapping_add(0), 0, 12);
        { let v_ = 20i32; c.m.w32(fp.wrapping_add(32).wrapping_add(48).wrapping_add(0).wrapping_add(0), (v_) as u32); }
        { let v_ = 30i32; c.m.w32(fp.wrapping_add(32).wrapping_add(48).wrapping_add(0).wrapping_add(4), (v_) as u32); }
        { let v_ = 40i32; c.m.w32(fp.wrapping_add(32).wrapping_add(48).wrapping_add(0).wrapping_add(8), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(32).wrapping_add(48).wrapping_add(12), 0, 12);
        { let v_ = 5i32; c.m.w32(fp.wrapping_add(32).wrapping_add(48).wrapping_add(12).wrapping_add(0), (v_) as u32); }
        { let v_ = 10i32; c.m.w32(fp.wrapping_add(32).wrapping_add(48).wrapping_add(12).wrapping_add(4), (v_) as u32); }
        { let v_ = 15i32; c.m.w32(fp.wrapping_add(32).wrapping_add(48).wrapping_add(12).wrapping_add(8), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(32).wrapping_add(72), 0, 24);
        rt::memset(c, fp.wrapping_add(32).wrapping_add(72).wrapping_add(0), 0, 12);
        { let v_ = 20i32; c.m.w32(fp.wrapping_add(32).wrapping_add(72).wrapping_add(0).wrapping_add(0), (v_) as u32); }
        { let v_ = 30i32; c.m.w32(fp.wrapping_add(32).wrapping_add(72).wrapping_add(0).wrapping_add(4), (v_) as u32); }
        { let v_ = 40i32; c.m.w32(fp.wrapping_add(32).wrapping_add(72).wrapping_add(0).wrapping_add(8), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(32).wrapping_add(72).wrapping_add(12), 0, 12);
        { let v_ = 5i32; c.m.w32(fp.wrapping_add(32).wrapping_add(72).wrapping_add(12).wrapping_add(0), (v_) as u32); }
        { let v_ = 10i32; c.m.w32(fp.wrapping_add(32).wrapping_add(72).wrapping_add(12).wrapping_add(4), (v_) as u32); }
        { let v_ = 15i32; c.m.w32(fp.wrapping_add(32).wrapping_add(72).wrapping_add(12).wrapping_add(8), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(32).wrapping_add(96), 0, 24);
        rt::memset(c, fp.wrapping_add(32).wrapping_add(96).wrapping_add(0), 0, 12);
        { let v_ = 40i32; c.m.w32(fp.wrapping_add(32).wrapping_add(96).wrapping_add(0).wrapping_add(0), (v_) as u32); }
        { let v_ = 50i32; c.m.w32(fp.wrapping_add(32).wrapping_add(96).wrapping_add(0).wrapping_add(4), (v_) as u32); }
        { let v_ = 60i32; c.m.w32(fp.wrapping_add(32).wrapping_add(96).wrapping_add(0).wrapping_add(8), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(32).wrapping_add(96).wrapping_add(12), 0, 12);
        { let v_ = 15i32; c.m.w32(fp.wrapping_add(32).wrapping_add(96).wrapping_add(12).wrapping_add(0), (v_) as u32); }
        { let v_ = 20i32; c.m.w32(fp.wrapping_add(32).wrapping_add(96).wrapping_add(12).wrapping_add(4), (v_) as u32); }
        { let v_ = 25i32; c.m.w32(fp.wrapping_add(32).wrapping_add(96).wrapping_add(12).wrapping_add(8), (v_) as u32); }
        if ((((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(8); (c.m.r32(a_) as i32) }) & (127i32))) == (127i32)) as i32)) != 0) {
            break 'f (1i32).wrapping_neg();
        }
        if (((((((F::Rand_Range(c, ((100i32) as u32))) as i32)) >= ((c.m.r32((((fp.wrapping_add(32)).wrapping_add((((p_tour_1) as u32)).wrapping_mul(24))).wrapping_add((((p_second_3) as u32)).wrapping_mul(12))).wrapping_add((((p_level_2) as u32)).wrapping_mul(4))) as i32))) as i32)) != 0) {
            break 'f (1i32).wrapping_neg();
        }
        {
            l_i_5 = 0i32;
            'l1: loop {
                if !(((((l_i_5) < (7i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if ((((!(((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(8); (c.m.r32(a_) as i32) }) & ((((1u32).wrapping_shl((l_i_5) as u32)) as i32)))) != 0)) as i32)) != 0) {
                        { let v_ = l_i_5; let a_ = (fp).wrapping_add((((l_count_4) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        { let o_ = l_count_4; let n_ = (o_).wrapping_add(1); l_count_4 = n_; };
                    }
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
        break 'f { let a_ = (fp).wrapping_add((F::Rand_Range(c, ((l_count_4) as u32))).wrapping_mul(4)); (c.m.r32(a_) as i32) };
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Bracket_DrawDragonBall", &PROBE_N, &[], true); }
    r_
}
