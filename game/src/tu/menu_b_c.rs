#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn HistOutro_GuideGoto(c: &mut Ctx, mut p_movie_1: i32, mut p_side_2: i32, mut p_out_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistOutro_GuideGoto", &PROBE_N, &[(p_movie_1) as u32, (p_side_2) as u32, (p_out_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(144) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 144);
    let mut l_flash_4: u32 = 0;
    let r_ = 'f: {
        l_flash_4 = ((c.m.r32(sym::gHistOutro)).wrapping_add(28)).wrapping_add((((p_movie_1) as u32)).wrapping_mul(44));
        if (((((p_side_2) == (0i32)) as i32)) != 0) {
            { let _ = { let a2_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(52); (c.m.r32(a_) as i32) }; F::sprintf(c, fp.wrapping_add(8), sym::menu_b_c__str0, &[((a2_) as i64 as u64)]) }; };
            { let _ = F::strcpy(c, fp.wrapping_add(72), (if ((p_out_3) != 0) { sym::menu_b_c__str1 } else { sym::menu_b_c__str2 })); };
        } else {
            { let _ = { let a2_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(232); (c.m.r32(a_) as i32) }; F::sprintf(c, fp.wrapping_add(8), sym::menu_b_c__str3, &[((a2_) as i64 as u64)]) }; };
            { let _ = F::strcpy(c, fp.wrapping_add(72), (if ((p_out_3) != 0) { sym::menu_b_c__str4 } else { sym::menu_b_c__str5 })); };
        }
        F::Flash_FindLabel(c, l_flash_4, ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, l_flash_4, fp, fp.wrapping_add(72));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HistOutro_GuideGoto", &PROBE_N, &[], true); }
    r_
}

pub fn HistOutro_Init(c: &mut Ctx, mut p_section_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistOutro_Init", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_2: i32 = 0;
    let mut l_slot_3: u32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = F::Heap_Alloc(c, 704i32, ((32i32) as u32), 0i32, 2i32); let a_ = sym::gHistOutro; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gHistOutro); F::memset(c, a0_, 0i32, ((704i32) as u32)) }; };
        { let v_ = (c.m.r32(sym::gMenuArc2)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc2)).wrapping_add((((p_section_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = c.m.r32(sym::gHistOutro); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gHistOutro); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(4); c.m.w32(a_, v_); };
        's1: {
            let sv4: i32 = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(52); (c.m.r32(a_) as i32) };
            match sv4 {
                0i32 => {
                    { let v_ = 3i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(260); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 34103i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(216); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 8i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(220); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(232); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                1i32 => {
                    { let v_ = 4i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(260); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 33827i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(216); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 4i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(220); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 2i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(232); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                2i32 => {
                    { let v_ = 7i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(260); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 33716i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(216); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 9i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(220); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 3i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(232); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                3i32 => {
                    { let v_ = 5i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(260); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 33755i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(216); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 10i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(220); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 5i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(232); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                4i32 => {
                    { let v_ = 16i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(260); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 33924i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(216); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 9i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(220); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 8i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(232); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                5i32 => {
                    { let v_ = 5i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(260); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 33856i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(216); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 10i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(220); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 6i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(232); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                6i32 => {
                    { let v_ = 4i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(260); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 33794i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(216); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 6i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(220); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 8i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(232); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                7i32 => {
                    { let v_ = 4i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(260); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 33894i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(216); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 8i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(220); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 8i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(232); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                _ => {
                }
            }
        }
        { let v_ = { let a0_ = (({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(52); (c.m.r32(a_) as i32) })).wrapping_add(16i32); F::File_LoadSync(c, a0_, ((0i32) as u32), 0i32) }; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(8); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(12); c.m.w32(a_, v_); };
        { let a0_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(12); c.m.r32(a_) }; F::ModeBg_Init(c, a0_) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        's2: {
            let sv5: i32 = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(52); (c.m.r32(a_) as i32) };
            match sv5 {
                0i32 => {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    break 's2;
                }
                1i32 => {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    break 's2;
                }
                2i32 => {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    break 's2;
                }
                3i32 => {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    break 's2;
                }
                4i32 => {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    break 's2;
                }
                5i32 => {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    break 's2;
                }
                6i32 => {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    break 's2;
                }
                7i32 => {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    break 's2;
                }
                _ => {
                }
            }
        }
        { let v_ = ({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        's3: {
            let sv6: i32 = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(232); (c.m.r32(a_) as i32) };
            match sv6 {
                0i32 => {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    break 's3;
                }
                1i32 => {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    break 's3;
                }
                2i32 => {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    break 's3;
                }
                3i32 => {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    break 's3;
                }
                4i32 => {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    break 's3;
                }
                5i32 => {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    break 's3;
                }
                6i32 => {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    break 's3;
                }
                7i32 => {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    break 's3;
                }
                8i32 => {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    break 's3;
                }
                _ => {
                }
            }
        }
        { let v_ = ({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((7i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((30i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(72)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(28)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gHistOutro)).wrapping_add(72); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(28)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(16); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(20); c.m.w32(a_, v_); };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(16); c.m.r32(a_) }; let _ = 0i32; F::MsgWin_Init(c, a0_, a1_, 1i32) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(24); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(700); c.m.w32(a_, v_); };
        { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); c.m.w32(a_, (v_) as u32); };
        {
            l_i_2 = 0i32;
            'l4: loop {
                if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l4; }
                'c5: {
                    { let v_ = ((F::Rand_Range(c, ((32i32) as u32))) as i32); let a_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(240)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        {
            l_i_2 = 0i32;
            'l6: loop {
                if !(((((l_i_2) < (3i32)) as i32)) != 0) { break 'l6; }
                'c7: {
                    { let a0_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(280)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); let a1_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(24); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, ((0i32) as u32)) };
                    { let a0_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(280)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); F::TextBox_SetUnk80(c, a0_, 1i32) };
                    { let a0_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(280)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); F::TextBox_SetUnk50(c, a0_, 1i32) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        's8: {
            let sv7: i32 = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(52); (c.m.r32(a_) as i32) };
            match sv7 {
                0i32 => {
                    F::Bgm_Play(c, 68380i32);
                    F::StreamSe_PlayDefault(c, 0i32, 68572i32);
                    break 's8;
                }
                1i32 => {
                    F::Bgm_Play(c, 68380i32);
                    F::StreamSe_PlayDefault(c, 0i32, 68573i32);
                    if ((((!(((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(16)); (c.m.r32(a_) as i32) }) & (4i32))) != 0)) as i32)) != 0) {
                        { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(4108); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let r_ = 1i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(4104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    break 's8;
                }
                2i32 => {
                    F::Bgm_Play(c, 68380i32);
                    F::StreamSe_PlayDefault(c, 0i32, 68574i32);
                    break 's8;
                }
                3i32 => {
                    F::Bgm_Play(c, 68380i32);
                    break 's8;
                }
                4i32 => {
                    F::Bgm_Play(c, 68380i32);
                    if ((((!(((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(16)).wrapping_add((((4i32) as u32)).wrapping_mul(16)); (c.m.r32(a_) as i32) }) & (4i32))) != 0)) as i32)) != 0) {
                        { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(4108); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let r_ = 1i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(4104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    break 's8;
                }
                5i32 => {
                    F::Bgm_Play(c, 68382i32);
                    if ((((!(((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(16)).wrapping_add((((5i32) as u32)).wrapping_mul(16)); (c.m.r32(a_) as i32) }) & (4i32))) != 0)) as i32)) != 0) {
                        { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(4108); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let r_ = 1i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(4104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    break 's8;
                }
                6i32 => {
                    F::Bgm_Play(c, 68392i32);
                    break 's8;
                }
                7i32 => {
                    F::Bgm_Play(c, 68380i32);
                    if ((((!(((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(16)).wrapping_add((((7i32) as u32)).wrapping_mul(16)); (c.m.r32(a_) as i32) }) & (4i32))) != 0)) as i32)) != 0) {
                        { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(4108); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let r_ = 1i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(4104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    break 's8;
                }
                _ => {
                }
            }
        }
        l_slot_3 = (c.m.r32(sym::gSaveData)).wrapping_add(16);
        { let r_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(52); (c.m.r32(a_) as i32) }; let n_ = (l_slot_3).wrapping_add((((r_) as u32)).wrapping_mul(16)); l_slot_3 = n_; };
        { let r_ = 4i32; let a_ = l_slot_3; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HistOutro_Init", &PROBE_N, &[], true); }
    r_
}

pub fn HistOutro_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistOutro_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    F::ModeBg_Term(c);
    F::MsgWin_Term(c);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(28)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Destroy(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(12); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(12); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(12); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(8); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(8); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(8); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gHistOutro)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gHistOutro); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gHistOutro; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HistOutro_Term", &PROBE_N, &[], true); }
    r_
}

pub fn HistOutro_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistOutro_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(144) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 144);
    let mut l_i_1: i32 = 0;
    let mut l_guide_2: i32 = 0;
    let mut l_flash_3: u32 = 0;
    let r_ = 'f: {
        F::ModeBg_Draw(c);
        l_flash_3 = ((c.m.r32(sym::gHistOutro)).wrapping_add(28)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        {
            l_i_1 = 0i32;
            'l1: loop {
                if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((l_i_1) != (0i32)) as i32)) != 0) {
                        l_guide_2 = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(232); (c.m.r32(a_) as i32) };
                    } else {
                        l_guide_2 = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(52); (c.m.r32(a_) as i32) };
                    }
                    { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_b_c__str6, &[((l_guide_2) as i64 as u64)]); };
                    { let _ = F::sprintf(c, fp.wrapping_add(72), sym::menu_b_c__str7, &[((l_guide_2) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(8), fp.wrapping_add(72), fp);
                    { let a2_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(240)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); F::FlashAnim_Blink(c, l_flash_3, fp, a2_, 0i32) };
                    { let _ = F::sprintf(c, fp.wrapping_add(72), sym::menu_b_c__str8, &[((l_guide_2) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(8), fp.wrapping_add(72), fp);
                    if ((((({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(236); (c.m.r32(a_) as i32) }) == (l_i_1)) as i32)) != 0) {
                        { let a2_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(248)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); F::FlashAnim_Talk(c, l_flash_3, fp, a2_, 0i32) };
                    } else {
                        F::FlashAnim_ShowNext2(c, l_flash_3, fp, 0i32);
                    }
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        {
            l_i_1 = 0i32;
            'l3: loop {
                if !(((((l_i_1) < (3i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_b_c__str9, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), fp.wrapping_add(8), fp);
                    if (((((l_i_1) >= ({ let a_ = ({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(700); c.m.r32(a_) }).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(260); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(32)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        { let a3_ = (rt::f2i({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(264); F32(c.m.r32(a_)) })).wrapping_neg(); let a5_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(280)).wrapping_add((((l_i_1) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_3, fp, 0i32, a3_, (1i32).wrapping_neg(), a5_) };
                    } else {
                        { let a3_ = (rt::f2i({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(264); F32(c.m.r32(a_)) })).wrapping_neg(); let a4_ = ({ let a_ = (({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(700); c.m.r32(a_) }).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(260); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(32))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_add(l_i_1); let a5_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(280)).wrapping_add((((l_i_1) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_3, fp, 0i32, a3_, a4_, a5_) };
                    }
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        {
            l_i_1 = 0i32;
            'l5: loop {
                if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l5; }
                'c6: {
                    { let a0_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(28)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        F::Font_FlushAll(c);
        { let a2_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); (c.m.r32(a_) as i32) }; F::MsgWin_Draw(c, 0i32, 0i32, a2_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HistOutro_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn HistOutro_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistOutro_Update", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(28)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Advance(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((((((!(((({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(200); (c.m.r32(a_) as i32) }) & (4i32))) != 0)) as i32)) != 0) && (((({ let a_ = (((c.m.r32(sym::gHistOutro)).wrapping_add(28)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
        F::MsgWin_Open(c);
        's3: {
            let sv2: i32 = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(52); (c.m.r32(a_) as i32) };
            'k6: {
            'k5: {
            'k4: {
                match sv2 { 4i32 => break 'k4, 7i32 => break 'k5, _ => break 'k6, }
            }
            F::HistOutro_GuideGoto(c, 0i32, 0i32, 0i32);
            }
            F::HistOutro_GuideGoto(c, 0i32, 1i32, 0i32);
            break 's3;
            }
            F::HistOutro_GuideGoto(c, 0i32, 0i32, 0i32);
            break 's3;
        }
    }
    if (((({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(200); (c.m.r32(a_) as i32) }) & (16i32))) != 0) {
        { let r_ = F32(0x3e955555); let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(264); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if (((rt::fge({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(264); F32(c.m.r32(a_)) }, { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(268); F32(c.m.r32(a_)) }) as i32)) != 0) {
            { let v_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(268); F32(c.m.r32(a_)) }; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(264); c.m.w32(a_, (v_).0); };
            { let r_ = (!(16i32)); let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(200); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HistOutro_Update", &PROBE_N, &[], true); }
    r_
}

pub fn HistOutro_Input(c: &mut Ctx, mut p_result_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistOutro_Input", &PROBE_N, &[(p_result_1) as u32], false); }
    let r_ = 'f: {
    if ((((!(((({ let a_ = (((c.m.r32(sym::gHistOutro)).wrapping_add(28)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f;
    }
    if ((((!(((({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(200); (c.m.r32(a_) as i32) }) & (4i32))) != 0)) as i32)) != 0) {
        { let r_ = 4i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(200); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(212); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
        break 'f;
    }
    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
        if (((({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(200); (c.m.r32(a_) as i32) }) & (16i32))) != 0) {
            F::Voice_StopWithLip(c);
            { let v_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(268); F32(c.m.r32(a_)) }; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(264); c.m.w32(a_, (v_).0); };
            { let r_ = (!(16i32)); let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(200); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        } else {
            { let r_ = 1i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(200); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            { let r_ = 2i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(200); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            { let v_ = 15i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(208); c.m.w32(a_, (v_) as u32); };
        }
        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
    } else {
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((4096i32) as u32)))) != 0) {
            { let r_ = 1i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(200); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            { let r_ = 2i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(200); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            { let v_ = 15i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(208); c.m.w32(a_, (v_) as u32); };
            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HistOutro_Input", &PROBE_N, &[], true); }
    r_
}

pub fn HistOutro_UpdateStep(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistOutro_UpdateStep", &PROBE_N, &[], false); }
    let mut l_slot_1: u32 = 0;
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        break 'f;
    }
    's1: {
        let sv2: i32 = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); (c.m.r32(a_) as i32) };
        'k60: {
        'k59: {
        'k58: {
        'k57: {
        'k56: {
        'k55: {
        'k54: {
        'k53: {
        'k52: {
        'k51: {
        'k50: {
        'k49: {
        'k48: {
        'k47: {
        'k46: {
        'k45: {
        'k44: {
        'k43: {
        'k42: {
        'k41: {
        'k40: {
        'k39: {
        'k38: {
        'k37: {
        'k36: {
        'k35: {
        'k34: {
        'k33: {
        'k32: {
        'k31: {
        'k30: {
        'k29: {
        'k28: {
        'k27: {
        'k26: {
        'k25: {
        'k24: {
        'k23: {
        'k22: {
        'k21: {
        'k20: {
        'k19: {
        'k18: {
        'k17: {
        'k16: {
        'k15: {
        'k14: {
        'k13: {
        'k12: {
        'k11: {
        'k10: {
        'k9: {
        'k8: {
        'k7: {
        'k6: {
        'k5: {
        'k4: {
        'k3: {
        'k2: {
            match sv2 { 1i32 => break 'k2, 2i32 => break 'k3, 51i32 => break 'k4, 101i32 => break 'k5, 103i32 => break 'k6, 109i32 => break 'k6, 115i32 => break 'k6, 117i32 => break 'k6, 119i32 => break 'k6, 107i32 => break 'k7, 111i32 => break 'k7, 113i32 => break 'k7, 121i32 => break 'k7, 123i32 => break 'k7, 102i32 => break 'k8, 104i32 => break 'k8, 108i32 => break 'k8, 110i32 => break 'k8, 112i32 => break 'k8, 114i32 => break 'k8, 116i32 => break 'k8, 118i32 => break 'k8, 120i32 => break 'k8, 122i32 => break 'k8, 124i32 => break 'k8, 105i32 => break 'k9, 151i32 => break 'k10, 153i32 => break 'k11, 155i32 => break 'k11, 159i32 => break 'k11, 163i32 => break 'k11, 161i32 => break 'k12, 167i32 => break 'k12, 169i32 => break 'k12, 152i32 => break 'k13, 154i32 => break 'k13, 156i32 => break 'k13, 160i32 => break 'k13, 162i32 => break 'k13, 164i32 => break 'k13, 168i32 => break 'k13, 170i32 => break 'k13, 165i32 => break 'k14, 157i32 => break 'k15, 201i32 => break 'k16, 207i32 => break 'k17, 213i32 => break 'k17, 221i32 => break 'k17, 205i32 => break 'k18, 209i32 => break 'k18, 211i32 => break 'k18, 215i32 => break 'k18, 217i32 => break 'k18, 202i32 => break 'k19, 206i32 => break 'k19, 208i32 => break 'k19, 210i32 => break 'k19, 212i32 => break 'k19, 214i32 => break 'k19, 216i32 => break 'k19, 218i32 => break 'k19, 222i32 => break 'k19, 203i32 => break 'k20, 219i32 => break 'k21, 223i32 => break 'k22, 251i32 => break 'k23, 253i32 => break 'k24, 255i32 => break 'k24, 261i32 => break 'k24, 275i32 => break 'k24, 259i32 => break 'k25, 263i32 => break 'k25, 265i32 => break 'k25, 267i32 => break 'k25, 269i32 => break 'k25, 271i32 => break 'k25, 252i32 => break 'k26, 254i32 => break 'k26, 256i32 => break 'k26, 260i32 => break 'k26, 262i32 => break 'k26, 264i32 => break 'k26, 266i32 => break 'k26, 268i32 => break 'k26, 270i32 => break 'k26, 272i32 => break 'k26, 276i32 => break 'k26, 257i32 => break 'k27, 273i32 => break 'k28, 277i32 => break 'k29, 301i32 => break 'k30, 305i32 => break 'k31, 307i32 => break 'k31, 311i32 => break 'k31, 313i32 => break 'k31, 315i32 => break 'k31, 319i32 => break 'k31, 303i32 => break 'k32, 309i32 => break 'k32, 317i32 => break 'k32, 302i32 => break 'k33, 304i32 => break 'k33, 306i32 => break 'k33, 308i32 => break 'k33, 310i32 => break 'k33, 312i32 => break 'k33, 314i32 => break 'k33, 316i32 => break 'k33, 318i32 => break 'k33, 320i32 => break 'k33, 351i32 => break 'k34, 357i32 => break 'k35, 373i32 => break 'k35, 355i32 => break 'k36, 359i32 => break 'k37, 361i32 => break 'k37, 363i32 => break 'k37, 365i32 => break 'k37, 367i32 => break 'k37, 369i32 => break 'k37, 352i32 => break 'k38, 356i32 => break 'k38, 358i32 => break 'k38, 360i32 => break 'k38, 362i32 => break 'k38, 364i32 => break 'k38, 366i32 => break 'k38, 368i32 => break 'k38, 370i32 => break 'k38, 374i32 => break 'k38, 353i32 => break 'k39, 371i32 => break 'k40, 375i32 => break 'k41, 401i32 => break 'k42, 403i32 => break 'k43, 405i32 => break 'k43, 409i32 => break 'k44, 411i32 => break 'k44, 413i32 => break 'k44, 415i32 => break 'k44, 417i32 => break 'k44, 419i32 => break 'k44, 421i32 => break 'k44, 402i32 => break 'k45, 404i32 => break 'k45, 406i32 => break 'k45, 410i32 => break 'k45, 412i32 => break 'k45, 414i32 => break 'k45, 416i32 => break 'k45, 418i32 => break 'k45, 420i32 => break 'k45, 422i32 => break 'k45, 407i32 => break 'k46, 451i32 => break 'k47, 455i32 => break 'k48, 461i32 => break 'k48, 457i32 => break 'k49, 459i32 => break 'k49, 463i32 => break 'k49, 452i32 => break 'k50, 456i32 => break 'k50, 458i32 => break 'k50, 460i32 => break 'k50, 462i32 => break 'k50, 464i32 => break 'k50, 453i32 => break 'k51, 106i32 => break 'k52, 158i32 => break 'k52, 166i32 => break 'k52, 204i32 => break 'k52, 220i32 => break 'k52, 258i32 => break 'k52, 274i32 => break 'k52, 354i32 => break 'k52, 372i32 => break 'k52, 408i32 => break 'k52, 454i32 => break 'k52, 125i32 => break 'k53, 321i32 => break 'k53, 465i32 => break 'k53, 171i32 => break 'k54, 423i32 => break 'k54, 501i32 => break 'k55, 502i32 => break 'k56, 503i32 => break 'k57, 504i32 => break 'k58, 505i32 => break 'k59, 551i32 => break 'k60, _ => break 's1, }
        }
        l_slot_1 = (c.m.r32(sym::gSaveData)).wrapping_add(16);
        { let r_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(52); (c.m.r32(a_) as i32) }; let n_ = (l_slot_1).wrapping_add((((r_) as u32)).wrapping_mul(16)); l_slot_1 = n_; };
        { let r_ = 16i32; let a_ = l_slot_1; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); c.m.w32(a_, (v_) as u32); };
        break 's1;
        }
        { let v_ = (({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(52); (c.m.r32(a_) as i32) }).wrapping_mul(50i32)).wrapping_add(101i32); let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); c.m.w32(a_, (v_) as u32); };
        break 's1;
        }
        { let a0_ = ((c.m.r32(sym::gHistOutro)).wrapping_add(28)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_StepFrames(c, a0_, 1i32) };
        F::MsgWin_Close(c);
        { let v_ = 501i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); c.m.w32(a_, (v_) as u32); };
        break 's1;
        }
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(236); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(220); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); c.m.w32(a_, (v_) as u32); };
        { let a0_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(20); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(216); (c.m.r32(a_) as i32) }; let a2_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, a1_, a2_) };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(236); c.m.w32(a_, (v_) as u32); };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        { let a0_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(20); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(216); (c.m.r32(a_) as i32) }; let a2_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, a1_, a2_) };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        { let v_ = 1i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(236); c.m.w32(a_, (v_) as u32); };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        { let a0_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(20); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(216); (c.m.r32(a_) as i32) }; let a2_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, a1_, a2_) };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
            { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        } else {
            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
            }
        }
        break 's1;
        }
        F::HistOutro_GuideGoto(c, 0i32, 1i32, 0i32);
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(236); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(220); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); c.m.w32(a_, (v_) as u32); };
        { let a0_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(20); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(216); (c.m.r32(a_) as i32) }; let a2_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, a1_, a2_) };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(236); c.m.w32(a_, (v_) as u32); };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        { let a0_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(20); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(216); (c.m.r32(a_) as i32) }; let a2_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, a1_, a2_) };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        { let v_ = 1i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(236); c.m.w32(a_, (v_) as u32); };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        { let a0_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(20); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(216); (c.m.r32(a_) as i32) }; let a2_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, a1_, a2_) };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
            { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        } else {
            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
            }
        }
        break 's1;
        }
        F::HistOutro_GuideGoto(c, 0i32, 0i32, 1i32);
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        F::HistOutro_GuideGoto(c, 0i32, 1i32, 0i32);
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(236); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(220); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); c.m.w32(a_, (v_) as u32); };
        { let a0_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(20); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(216); (c.m.r32(a_) as i32) }; let a2_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, a1_, a2_) };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(236); c.m.w32(a_, (v_) as u32); };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        { let a0_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(20); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(216); (c.m.r32(a_) as i32) }; let a2_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, a1_, a2_) };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        { let v_ = 1i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(236); c.m.w32(a_, (v_) as u32); };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        { let a0_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(20); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(216); (c.m.r32(a_) as i32) }; let a2_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, a1_, a2_) };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
            { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        } else {
            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
            }
        }
        break 's1;
        }
        F::HistOutro_GuideGoto(c, 0i32, 1i32, 0i32);
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        F::HistOutro_GuideGoto(c, 0i32, 1i32, 1i32);
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        F::HistOutro_GuideGoto(c, 0i32, 0i32, 1i32);
        { let v_ = 51i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); c.m.w32(a_, (v_) as u32); };
        break 's1;
        }
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(236); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(220); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); c.m.w32(a_, (v_) as u32); };
        { let a0_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(20); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(216); (c.m.r32(a_) as i32) }; let a2_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, a1_, a2_) };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(236); c.m.w32(a_, (v_) as u32); };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        { let a0_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(20); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(216); (c.m.r32(a_) as i32) }; let a2_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, a1_, a2_) };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        { let v_ = 1i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(236); c.m.w32(a_, (v_) as u32); };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        { let a0_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(20); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(216); (c.m.r32(a_) as i32) }; let a2_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, a1_, a2_) };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
            { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        } else {
            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
            }
        }
        break 's1;
        }
        F::HistOutro_GuideGoto(c, 0i32, 1i32, 0i32);
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        F::HistOutro_GuideGoto(c, 0i32, 1i32, 1i32);
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        F::HistOutro_GuideGoto(c, 0i32, 0i32, 1i32);
        { let v_ = 51i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); c.m.w32(a_, (v_) as u32); };
        break 's1;
        }
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(236); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(220); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); c.m.w32(a_, (v_) as u32); };
        { let a0_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(20); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(216); (c.m.r32(a_) as i32) }; let a2_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, a1_, a2_) };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(236); c.m.w32(a_, (v_) as u32); };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        { let a0_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(20); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(216); (c.m.r32(a_) as i32) }; let a2_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, a1_, a2_) };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        { let v_ = 1i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(236); c.m.w32(a_, (v_) as u32); };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        { let a0_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(20); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(216); (c.m.r32(a_) as i32) }; let a2_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, a1_, a2_) };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
            { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        } else {
            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
            }
        }
        break 's1;
        }
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(236); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(220); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); c.m.w32(a_, (v_) as u32); };
        { let a0_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(20); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(216); (c.m.r32(a_) as i32) }; let a2_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, a1_, a2_) };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(236); c.m.w32(a_, (v_) as u32); };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        { let a0_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(20); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(216); (c.m.r32(a_) as i32) }; let a2_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, a1_, a2_) };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        { let v_ = 1i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(236); c.m.w32(a_, (v_) as u32); };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        { let a0_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(20); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(216); (c.m.r32(a_) as i32) }; let a2_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, a1_, a2_) };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
            { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        } else {
            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
            }
        }
        break 's1;
        }
        F::HistOutro_GuideGoto(c, 0i32, 1i32, 0i32);
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        F::HistOutro_GuideGoto(c, 0i32, 1i32, 1i32);
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        F::HistOutro_GuideGoto(c, 0i32, 0i32, 1i32);
        { let v_ = 51i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); c.m.w32(a_, (v_) as u32); };
        break 's1;
        }
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(236); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(220); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); c.m.w32(a_, (v_) as u32); };
        { let a0_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(20); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(216); (c.m.r32(a_) as i32) }; let a2_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, a1_, a2_) };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(236); c.m.w32(a_, (v_) as u32); };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        { let a0_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(20); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(216); (c.m.r32(a_) as i32) }; let a2_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, a1_, a2_) };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        { let v_ = 1i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(236); c.m.w32(a_, (v_) as u32); };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        { let a0_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(20); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(216); (c.m.r32(a_) as i32) }; let a2_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, a1_, a2_) };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
            { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        } else {
            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
            }
        }
        break 's1;
        }
        F::HistOutro_GuideGoto(c, 0i32, 0i32, 1i32);
        F::HistOutro_GuideGoto(c, 0i32, 1i32, 0i32);
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        { let v_ = 1i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(236); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(220); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); c.m.w32(a_, (v_) as u32); };
        { let a0_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(20); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(216); (c.m.r32(a_) as i32) }; let a2_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, a1_, a2_) };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(236); c.m.w32(a_, (v_) as u32); };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        { let a0_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(20); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(216); (c.m.r32(a_) as i32) }; let a2_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, a1_, a2_) };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        { let v_ = 1i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(236); c.m.w32(a_, (v_) as u32); };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        { let a0_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(20); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(216); (c.m.r32(a_) as i32) }; let a2_ = { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(228); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, a1_, a2_) };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
            { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        } else {
            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
            }
        }
        break 's1;
        }
        F::HistOutro_GuideGoto(c, 0i32, 0i32, 0i32);
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        if (((({ let a_ = (((c.m.r32(sym::gHistOutro)).wrapping_add(28)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((4i32) as u32)))) != 0) {
            { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        break 's1;
        }
        F::HistOutro_GuideGoto(c, 0i32, 0i32, 1i32);
        }
        F::HistOutro_GuideGoto(c, 0i32, 1i32, 1i32);
        { let v_ = 51i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); c.m.w32(a_, (v_) as u32); };
        break 's1;
        }
        { let v_ = F32(0x00000000); let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(264); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(rt::fmul(rt::i2f({ let a_ = ({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(700); c.m.r32(a_) }).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(260); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(32)); (c.m.r32(a_) as i32) }), F32(0x42200000)), F32(0x43660000)); let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(268); c.m.w32(a_, (v_).0); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(272); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(276); c.m.w32(a_, (v_) as u32); };
        { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        if (((({ let a_ = (((c.m.r32(sym::gHistOutro)).wrapping_add(28)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
            { let r_ = 16i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(200); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        break 's1;
        }
        if ((((({ let a_ = ((({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(700); c.m.r32(a_) }).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(260); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(32))).wrapping_add(8)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(276); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) <= ({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(272); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); o_ })) as i32)) != 0) {
            { let a2_ = ({ let a_ = (({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(700); c.m.r32(a_) }).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(260); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(32))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(276); (c.m.r32(a_) as i32) }); F::Voice_PlayWithSubtitle(c, ((0i32) as u32), 33974i32, a2_) };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(272); c.m.w32(a_, (v_) as u32); };
            { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        break 's1;
        }
        if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
            if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(700); c.m.r32(a_) }).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(260); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(32)); (c.m.r32(a_) as i32) }) <= ({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(276); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ })) as i32)) != 0) {
                { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            } else {
                { let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
            }
        } else {
            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((4096i32) as u32)))) != 0) {
                { let v_ = 551i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
            }
        }
        break 's1;
        }
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); c.m.w32(a_, (v_) as u32); };
        break 's1;
        }
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); c.m.w32(a_, (v_) as u32); };
        { let r_ = 1i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(200); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let r_ = 2i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(200); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let v_ = 15i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(208); c.m.w32(a_, (v_) as u32); };
        break 's1;
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HistOutro_UpdateStep", &PROBE_N, &[], true); }
    r_
}

pub fn HistOutro_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistOutro_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = 1i32; c.m.w32(fp, (v_) as u32); }
        F::HistOutro_Init(c, p_section_1);
        F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
        'l1: while ((1i32) != 0) {
            F::Gfx_BeginFrame(c);
            F::Pad_Update(c);
            F::Snd_Update(c);
            F::ColorFade_Update(c);
            if ((((!(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0)) as i32)) != 0) {
                F::HistOutro_Update(c);
                F::HistOutro_UpdateStep(c);
            }
            F::HistOutro_Draw(c);
            F::ColorFade_Draw(c);
            F::Gfx_EndFrame(c, 1i32);
            F::Dma_Flush(c);
            F::File_Stub264D90(c);
            if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
                continue 'l1;
            }
            if ((F::ColorFade_IsInDone(c)) != 0) {
                if ((((((((!(((({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(200); (c.m.r32(a_) as i32) }) & (8i32))) != 0)) as i32)) != 0) && (((({ let a_ = (((c.m.r32(sym::gHistOutro)).wrapping_add(28)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
                    { let r_ = 8i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(200); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); c.m.w32(a_, (v_) as u32); };
                }
            }
            if ((F::ColorFade_IsFadingOut(c)) != 0) {
                F::Voice_FadeOutStep(c, 0i32);
                F::Bgm_FadeOutStep(c);
                F::StreamSe_FadeOutStep(c, 0i32);
                continue 'l1;
            }
            if ((F::ColorFade_IsOutDone(c)) != 0) {
                break 'l1;
            }
            if (((({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(200); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(208); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) == ((1i32).wrapping_neg())) as i32)) != 0) {
                    F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                }
            } else {
                if ((((({ let a_ = (c.m.r32(sym::gHistOutro)).wrapping_add(224); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::HistOutro_Input(c, fp);
                }
            }
        }
        if ((((((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(52); (c.m.r32(a_) as i32) }) == (6i32)) as i32)) != 0) && ((((!(((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(16)).wrapping_add((((8i32) as u32)).wrapping_mul(16)); (c.m.r32(a_) as i32) }) & (1i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
            { let v_ = 1i32; let a_ = fp; c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 0i32; let a_ = fp; c.m.w32(a_, (v_) as u32); };
        }
        F::HistOutro_Term(c);
        F::Dma_ResetBuffers(c);
        break 'f (c.m.r32(fp) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HistOutro_Run", &PROBE_N, &[], true); }
    r_
}

pub fn Hist_Main(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Hist_Main", &PROBE_N, &[], false); }
    let mut l_done_1: i32 = 0;
    let mut l_result_2: i32 = 0;
    let mut l_r_3: i32 = 0;
    let mut l_played_4: i32 = 0;
    let r_ = 'f: {
    l_done_1 = 0i32;
    l_result_2 = 1i32;
    if (((((c.m.r32(sym::gMenuArc2)) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a0_ = ({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_add(12i32); F::File_LoadSync(c, a0_, ((0i32) as u32), 0i32) }; let a_ = sym::gMenuArc2; c.m.w32(a_, v_); };
    }
    'l1: loop {
        'c2: {
            's3: {
                let sv5: i32 = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); (c.m.r32(a_) as i32) };
                match sv5 {
                    6i32 => {
                        F::Bgm_Play(c, 68376i32);
                        l_r_3 = F::HistSel_Run(c, 1i32);
                        F::Adx_StopAll(c);
                        { let v_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                        's4: {
                            let sv6: i32 = l_r_3;
                            match sv6 {
                                0i32 => {
                                    { let v_ = 4i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                                    l_result_2 = 0i32;
                                    l_done_1 = 1i32;
                                    break 's4;
                                }
                                1i32 => {
                                    { let v_ = 7i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                                    break 's4;
                                }
                                2i32 => {
                                    { let v_ = 3i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                                    break 's4;
                                }
                                _ => {
                                }
                            }
                        }
                        break 's3;
                    }
                    7i32 => {
                        if ((F::ModeMenu_Run(c, 2i32)) != 0) {
                            F::Adx_StopAll(c);
                            { let v_ = 8i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                            l_done_1 = 1i32;
                        } else {
                            F::Adx_StopAll(c);
                            { let v_ = 6i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        }
                        break 's3;
                    }
                    8i32 => {
                        l_played_4 = 1i32;
                        l_r_3 = F::HistResult_Run(c, 3i32);
                        if ((l_r_3) != 0) {
                            F::Adx_StopAll(c);
                            { let v_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                            's5: {
                                let sv7: i32 = l_r_3;
                                match sv7 {
                                    1i32 => {
                                        { let v_ = 10i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                                        break 's5;
                                    }
                                    2i32 => {
                                        { let v_ = 9i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                                        break 's5;
                                    }
                                    _ => {
                                    }
                                }
                            }
                        } else {
                            F::Adx_StopAll(c);
                            { let v_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 7i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        }
                        break 's3;
                    }
                    9i32 => {
                        l_played_4 = 1i32;
                        l_r_3 = F::HistOutro_Run(c, 2i32);
                        F::Adx_StopAll(c);
                        { let v_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                        's6: {
                            let sv8: i32 = l_r_3;
                            match sv8 {
                                0i32 => {
                                    { let v_ = 10i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                                    break 's6;
                                }
                                1i32 => {
                                    { let v_ = 3i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                                    break 's6;
                                }
                                _ => {
                                }
                            }
                        }
                        break 's3;
                    }
                    10i32 => {
                        l_played_4 = 1i32;
                        l_r_3 = F::HistSave_Run(c, 4i32);
                        F::Adx_StopAll(c);
                        { let v_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                        's7: {
                            let sv9: i32 = l_r_3;
                            match sv9 {
                                0i32 => {
                                    { let v_ = 6i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                                    break 's7;
                                }
                                1i32 => {
                                    { let v_ = 7i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                                    break 's7;
                                }
                                _ => {
                                }
                            }
                        }
                        break 's3;
                    }
                    3i32 => {
                        if (((((c.m.r32(sym::gMenuArc2)) != (((0i32) as u32))) as i32)) != 0) {
                            { let a0_ = c.m.r32(sym::gMenuArc2); F::Heap_Free(c, a0_) };
                            { let v_ = ((0i32) as u32); let a_ = sym::gMenuArc2; c.m.w32(a_, v_); };
                        }
                        F::PadWatch_SetEnabled(c, 0i32);
                        F::Movie_PlayEnding(c);
                        F::PadWatch_SetEnabled(c, 1i32);
                        F::Adx_StopAll(c);
                        { let r_ = 1i32; let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(16)).wrapping_add((((8i32) as u32)).wrapping_mul(16)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        { let r_ = ((128i32) as u32); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(3120); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                        's8: {
                            let sv10: i32 = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(28); (c.m.r32(a_) as i32) };
                            match sv10 {
                                6i32 => {
                                    { let v_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 6i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                                    break 's8;
                                }
                                9i32 => {
                                    { let r_ = 2i32; let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(16)).wrapping_add((((8i32) as u32)).wrapping_mul(16)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                    { let v_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 10i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                                    break 's8;
                                }
                                _ => {
                                }
                            }
                        }
                        if (((((c.m.r32(sym::gMenuArc2)) == (((0i32) as u32))) as i32)) != 0) {
                            { let v_ = { let a0_ = ({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_add(12i32); F::File_LoadSync(c, a0_, ((0i32) as u32), 0i32) }; let a_ = sym::gMenuArc2; c.m.w32(a_, v_); };
                        }
                        break 's3;
                    }
                    _ => {
                    }
                }
            }
            { let _ = F::sceGsSyncPath(c, 0i32, ((0i32) as u16)); };
        }
        if !((((!((l_done_1) != 0)) as i32)) != 0) { break 'l1; }
    }
    if (((((c.m.r32(sym::gMenuArc2)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gMenuArc2); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gMenuArc2; c.m.w32(a_, v_); };
    }
    break 'f l_result_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Hist_Main", &PROBE_N, &[], true); }
    r_
}
