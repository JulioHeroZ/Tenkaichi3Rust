#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn ItemPanel_ClipGoto(c: &mut Ctx, mut p_side_1: i32, mut p_kind_2: i32, mut p_label_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ItemPanel_ClipGoto", &PROBE_N, &[(p_side_1) as u32, (p_kind_2) as u32, (p_label_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_p_4: u32 = 0;
    let mut l_flash_5: u32 = 0;
    let r_ = 'f: {
        l_p_4 = c.m.r32((sym::gItemPanel).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)));
        l_flash_5 = (l_p_4).wrapping_add(12);
        's1: {
            let sv6: i32 = p_kind_2;
            match sv6 {
                0i32 => {
                    { let _ = ((((F::strcpy(c, fp.wrapping_add(8), sym::menu_g__str0)) as u64)) as i32); };
                    break 's1;
                }
                1i32 => {
                    { let _ = { let a2_ = ((c.m.r32((l_p_4).wrapping_add(172)) as i32)).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_g__str1, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::Flash_FindLabel(c, l_flash_5, ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, l_flash_5, fp, p_label_3);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ItemPanel_ClipGoto", &PROBE_N, &[], true); }
    r_
}

pub fn ItemPanel_Init(c: &mut Ctx, mut p_pack_1: u32, mut p_side_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ItemPanel_Init", &PROBE_N, &[(p_pack_1) as u32, (p_side_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_p_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = F::Heap_Alloc(c, 1736i32, ((32i32) as u32), 0i32, 2i32); let a_ = (sym::gItemPanel).wrapping_add((((p_side_2) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32((sym::gItemPanel).wrapping_add((((p_side_2) as u32)).wrapping_mul(4))); F::memset(c, a0_, 0i32, ((1736i32) as u32)) }; };
        l_p_3 = c.m.r32((sym::gItemPanel).wrapping_add((((p_side_2) as u32)).wrapping_mul(4)));
        { let v_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((16i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((l_p_3).wrapping_add(56)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((((if ((p_side_2) != 0) { 1i32 } else { 0i32 })).wrapping_mul(64i32)) as u32)); let a_ = ((l_p_3).wrapping_add(56)).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((((if ((p_side_2) != 0) { 3i32 } else { 2i32 })).wrapping_mul(64i32)) as u32)); let a_ = ((l_p_3).wrapping_add(56)).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((l_p_3).wrapping_add(56)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((3i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((l_p_3).wrapping_add(56)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((l_p_3).wrapping_add(56)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((6i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((l_p_3).wrapping_add(56)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (if ((p_side_2) != 0) { ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)) } else { ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)) }); let a_ = ((l_p_3).wrapping_add(56)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((7i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((l_p_3).wrapping_add(56)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((8i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((l_p_3).wrapping_add(56)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((l_p_3).wrapping_add(56)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((l_p_3).wrapping_add(56)).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((11i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((l_p_3).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((l_p_3).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((9i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((l_p_3).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((12i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((l_p_3).wrapping_add(56)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((l_p_3).wrapping_add(56)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((10i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((l_p_3).wrapping_add(56)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((l_p_3).wrapping_add(56)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a1_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add(((((4i32).wrapping_add(p_side_2)) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::Flash_Create(c, (l_p_3).wrapping_add(12), a1_, (l_p_3).wrapping_add(56)) };
        F::Flash_Play(c, (l_p_3).wrapping_add(12), 1i32);
        { let v_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((13i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((14i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((15i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (l_p_3).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (l_p_3).wrapping_add(1732); c.m.w32(a_, v_); };
        { let a1_ = c.m.r32((l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(4))); F::TextBox_Init(c, (l_p_3).wrapping_add(192), a1_, (((if ((p_side_2) != 0) { 2i32 } else { 1i32 })) as u32)) };
        F::TextBox_SetUnk80(c, (l_p_3).wrapping_add(192), 1i32);
        { let a1_ = c.m.r32((l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(4))); F::TextBox_Init(c, (l_p_3).wrapping_add(332), a1_, (((if ((p_side_2) != 0) { 4i32 } else { 3i32 })) as u32)) };
        F::TextBox_SetUnk80(c, (l_p_3).wrapping_add(332), 1i32);
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < (8i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a1_ = c.m.r32((l_p_3).wrapping_add((((2i32) as u32)).wrapping_mul(4))); F::TextBox_Init(c, ((l_p_3).wrapping_add(472)).wrapping_add((((l_i_4) as u32)).wrapping_mul(140)), a1_, ((6i32) as u32)) };
                    F::TextBox_SetUnk80(c, ((l_p_3).wrapping_add(472)).wrapping_add((((l_i_4) as u32)).wrapping_mul(140)), 1i32);
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ItemPanel_Init", &PROBE_N, &[], true); }
    r_
}

pub fn ItemPanel_Term(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ItemPanel_Term", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32((sym::gItemPanel).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)))).wrapping_add(12); F::Flash_Destroy(c, a0_) };
    if (((((c.m.r32((sym::gItemPanel).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32((sym::gItemPanel).wrapping_add((((p_side_1) as u32)).wrapping_mul(4))); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (sym::gItemPanel).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ItemPanel_Term", &PROBE_N, &[], true); }
    r_
}

pub fn ItemPanel_Draw(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ItemPanel_Draw", &PROBE_N, &[(p_side_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_p_2: u32 = 0;
    let mut l_flash_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
        l_p_2 = c.m.r32((sym::gItemPanel).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)));
        if ((((!(((((c.m.r32((l_p_2).wrapping_add(148)) as i32)) & (8i32))) != 0)) as i32)) != 0) {
            break 'f;
        }
        l_flash_3 = (l_p_2).wrapping_add(12);
        { let v_ = (p_side_1).wrapping_mul(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_3, sym::menu_g__str2, (if ((p_side_1) != 0) { sym::menu_g__str3 } else { sym::menu_g__str4 }), fp);
        F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32((l_p_2).wrapping_add(156)) as i32)).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_3, sym::menu_g__str5, sym::menu_g__str6, fp);
        F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32((l_p_2).wrapping_add(156)) as i32)).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_3, sym::menu_g__str7, sym::menu_g__str8, fp);
        F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
        F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), sym::menu_g__str9, fp);
        { let a4_ = (c.m.r32((l_p_2).wrapping_add(152)) as i32); F::TextBox_AttachLine(c, l_flash_3, fp, 0i32, 0i32, a4_, (l_p_2).wrapping_add(192)) };
        F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), sym::menu_g__str10, fp);
        { let a4_ = (c.m.r32((l_p_2).wrapping_add(152)) as i32); F::TextBox_AttachLine(c, l_flash_3, fp, 0i32, 0i32, a4_, (l_p_2).wrapping_add(332)) };
        { let v_ = (rt::rem_i32((c.m.r32((l_p_2).wrapping_add(164)) as i32), 4i32)).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = (rt::div_i32((c.m.r32((l_p_2).wrapping_add(164)) as i32), 4i32)).wrapping_mul(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), sym::menu_g__str11, fp);
        F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
        { let v_ = (rt::rem_i32((c.m.r32((l_p_2).wrapping_add(160)) as i32), 4i32)).wrapping_mul(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = (rt::div_i32((c.m.r32((l_p_2).wrapping_add(160)) as i32), 4i32)).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), sym::menu_g__str12, fp);
        F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < (8i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_g__str13, &[(((l_i_4).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_g__str14, fp);
                    if (((((((((l_i_4) < ((c.m.r32((l_p_2).wrapping_add(168)) as i32))) as i32)) != 0) || (((((l_i_4) == (7i32)) as i32)) != 0)) as i32)) != 0) {
                        if (((((((c.m.r16(((l_p_2).wrapping_add(176)).wrapping_add((((l_i_4) as u32)).wrapping_mul(2)))) as i32)) != (0i32)) as i32)) != 0) {
                            F::Flash_ClipSetTex(c, l_flash_3, fp, 4i32);
                        } else {
                            F::Flash_ClipSetTex(c, l_flash_3, fp, 5i32);
                        }
                    } else {
                        F::Flash_ClipSetTex(c, l_flash_3, fp, 7i32);
                    }
                    F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_g__str15, fp);
                    if (((((((c.m.r16(((l_p_2).wrapping_add(176)).wrapping_add((((l_i_4) as u32)).wrapping_mul(2)))) as i32)) != (0i32)) as i32)) != 0) {
                        F::Flash_ClipSetTex(c, l_flash_3, fp, 0i32);
                    } else {
                        F::Flash_ClipSetTex(c, l_flash_3, fp, 1i32);
                    }
                    F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_g__str16, fp);
                    if ((((((c.m.r32((l_p_2).wrapping_add(172)) as i32)) == (l_i_4)) as i32)) != 0) {
                        { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 64i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                    } else {
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    }
                    F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_g__str17, fp);
                    if (((((((c.m.r16(((l_p_2).wrapping_add(176)).wrapping_add((((l_i_4) as u32)).wrapping_mul(2)))) as i32)) != (0i32)) as i32)) != 0) {
                        if ((((((({ let a_ = ((c.m.r32((l_p_2).wrapping_add(1732))).wrapping_add(((((((c.m.r16(((l_p_2).wrapping_add(176)).wrapping_add((((l_i_4) as u32)).wrapping_mul(2)))) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(40))).wrapping_add(3); c.m.r8(a_) }) as i32)) != (0i32)) as i32)) != 0) {
                            { let v_ = (((({ let a_ = ((c.m.r32((l_p_2).wrapping_add(1732))).wrapping_add(((((((c.m.r16(((l_p_2).wrapping_add(176)).wrapping_add((((l_i_4) as u32)).wrapping_mul(2)))) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(40))).wrapping_add(3); c.m.r8(a_) }) as i32)).wrapping_mul(32i32)).wrapping_sub(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let v_ = ({ let a0_ = (((c.m.r16(((l_p_2).wrapping_add(176)).wrapping_add((((l_i_4) as u32)).wrapping_mul(2)))) as i32)).wrapping_sub(1i32); let a1_ = c.m.r32((l_p_2).wrapping_add(1732)); F::ItemTbl_GetClass(c, a0_, a1_) }).wrapping_mul(42i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                            { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(42i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                            F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                        } else {
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                        }
                    } else {
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    }
                    F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_g__str18, fp);
                    if (((((((c.m.r16(((l_p_2).wrapping_add(176)).wrapping_add((((l_i_4) as u32)).wrapping_mul(2)))) as i32)) != (0i32)) as i32)) != 0) {
                        { let v_ = ((({ let a_ = (c.m.r32((l_p_2).wrapping_add(1732))).wrapping_add(((((((c.m.r16(((l_p_2).wrapping_add(176)).wrapping_add((((l_i_4) as u32)).wrapping_mul(2)))) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(40)); c.m.r8(a_) }) as i32)).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                        F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                    } else {
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    }
                    if (((((((c.m.r16(((l_p_2).wrapping_add(176)).wrapping_add((((l_i_4) as u32)).wrapping_mul(2)))) as i32)) != (0i32)) as i32)) != 0) {
                        F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_g__str19, fp);
                        { let a4_ = (((c.m.r16(((l_p_2).wrapping_add(176)).wrapping_add((((l_i_4) as u32)).wrapping_mul(2)))) as i32)).wrapping_sub(1i32); F::TextBox_AttachLine(c, l_flash_3, fp, 0i32, 0i32, a4_, ((l_p_2).wrapping_add(472)).wrapping_add((((l_i_4) as u32)).wrapping_mul(140))) };
                    }
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        F::Flash_Draw(c, l_flash_3);
        F::Font_FlushAll(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ItemPanel_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn ItemPanel_Update(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ItemPanel_Update", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
    l_p_2 = c.m.r32((sym::gItemPanel).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)));
    if (((((c.m.r32((l_p_2).wrapping_add(148)) as i32)) & (8i32))) != 0) {
        F::Flash_Advance(c, (l_p_2).wrapping_add(12));
        if ((((!(((((c.m.r32((l_p_2).wrapping_add(148)) as i32)) & (4i32))) != 0)) as i32)) != 0) {
            F::ItemPanel_ClipGoto(c, p_side_1, 0i32, sym::menu_g__str20);
            F::ItemPanel_ClipGoto(c, p_side_1, 1i32, sym::menu_g__str21);
            { let r_ = 4i32; let a_ = (l_p_2).wrapping_add(148); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if (((((c.m.r32((l_p_2).wrapping_add(148)) as i32)) & (16i32))) != 0) {
            if ((((c.m.r32(((l_p_2).wrapping_add(12)).wrapping_add(12))) & (((2i32) as u32)))) != 0) {
                { let r_ = (!(8i32)); let a_ = (l_p_2).wrapping_add(148); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = (!(16i32)); let a_ = (l_p_2).wrapping_add(148); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ItemPanel_Update", &PROBE_N, &[], true); }
    r_
}

pub fn ItemPanel_Input(c: &mut Ctx, mut p_side_1: i32, mut p_pad_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ItemPanel_Input", &PROBE_N, &[(p_side_1) as u32, (p_pad_2) as u32], false); }
    let mut l_p_3: u32 = 0;
    let mut l_result_4: i32 = 0;
    let r_ = 'f: {
    l_p_3 = c.m.r32((sym::gItemPanel).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)));
    l_result_4 = 0i32;
    if ((((c.m.r32(((sym::gPad).wrapping_add((((p_pad_2) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
        F::ItemPanel_ClipGoto(c, p_side_1, 1i32, sym::menu_g__str22);
        if ((((((c.m.r32((l_p_3).wrapping_add(172)) as i32)) == (7i32)) as i32)) != 0) {
            { let v_ = ((c.m.r32((l_p_3).wrapping_add(168)) as i32)).wrapping_sub(1i32); let a_ = (l_p_3).wrapping_add(172); c.m.w32(a_, (v_) as u32); };
        } else {
            { let a_ = (l_p_3).wrapping_add(172); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
            if ((((((c.m.r32((l_p_3).wrapping_add(172)) as i32)) < (0i32)) as i32)) != 0) {
                { let v_ = 7i32; let a_ = (l_p_3).wrapping_add(172); c.m.w32(a_, (v_) as u32); };
            }
        }
        F::ItemPanel_ClipGoto(c, p_side_1, 1i32, sym::menu_g__str23);
        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
    } else {
        if ((((c.m.r32(((sym::gPad).wrapping_add((((p_pad_2) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
            F::ItemPanel_ClipGoto(c, p_side_1, 1i32, sym::menu_g__str24);
            if ((((((c.m.r32((l_p_3).wrapping_add(172)) as i32)) == (7i32)) as i32)) != 0) {
                { let v_ = 0i32; let a_ = (l_p_3).wrapping_add(172); c.m.w32(a_, (v_) as u32); };
            } else {
                { let a_ = (l_p_3).wrapping_add(172); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                if ((((((c.m.r32((l_p_3).wrapping_add(172)) as i32)) >= ((c.m.r32((l_p_3).wrapping_add(168)) as i32))) as i32)) != 0) {
                    { let v_ = 7i32; let a_ = (l_p_3).wrapping_add(172); c.m.w32(a_, (v_) as u32); };
                }
            }
            F::ItemPanel_ClipGoto(c, p_side_1, 1i32, sym::menu_g__str25);
            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
        } else {
            if ((((c.m.r32(((sym::gPad).wrapping_add((((p_pad_2) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                F::ItemPanel_ClipGoto(c, p_side_1, 1i32, sym::menu_g__str26);
                if ((((((({ let a_ = ((l_p_3).wrapping_add(176)).wrapping_add(((((c.m.r32((l_p_3).wrapping_add(172)) as i32)) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as i32)) != (0i32)) as i32)) != 0) {
                    l_result_4 = (({ let a_ = ((l_p_3).wrapping_add(176)).wrapping_add(((((c.m.r32((l_p_3).wrapping_add(172)) as i32)) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as i32);
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                } else {
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                }
            } else {
                if (((((((((((((p_side_1) == (0i32)) as i32)) != 0) && ((((c.m.r32(((sym::gPad).wrapping_add((((p_pad_2) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((((1024i32) | (1i32))) as u32)))) != 0)) as i32)) != 0) || (((((((((p_side_1) != (0i32)) as i32)) != 0) && ((((c.m.r32(((sym::gPad).wrapping_add((((p_pad_2) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((((1024i32) | (2i32))) as u32)))) != 0)) as i32)) != 0)) as i32)) != 0) {
                    F::ItemPanel_ClipGoto(c, p_side_1, 1i32, sym::menu_g__str27);
                    F::ItemPanel_ClipGoto(c, p_side_1, 0i32, sym::menu_g__str28);
                    l_result_4 = (1i32).wrapping_neg();
                }
            }
        }
    }
    break 'f l_result_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ItemPanel_Input", &PROBE_N, &[], true); }
    r_
}

pub fn ItemPanel_SetChara(c: &mut Ctx, mut p_side_1: i32, mut p_chara_2: i32, mut p_slot_3: i32, mut p_set_4: i32, mut p_fromRec_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ItemPanel_SetChara", &PROBE_N, &[(p_side_1) as u32, (p_chara_2) as u32, (p_slot_3) as u32, (p_set_4) as u32, (p_fromRec_5) as u32], false); }
    let mut l_p_6: u32 = 0;
    let r_ = 'f: {
    l_p_6 = c.m.r32((sym::gItemPanel).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)));
    if ((p_fromRec_5) != 0) {
        { let s_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11584)).wrapping_add((((p_slot_3) as u32)).wrapping_mul(28)); let d_ = (l_p_6).wrapping_add(176); rt::memcpy(c, d_, s_, 16); };
    } else {
        { let s_ = (((c.m.r32(sym::gSaveData)).wrapping_add(6152)).wrapping_add((((p_slot_3) as u32)).wrapping_mul(56))).wrapping_add((((p_set_4) as u32)).wrapping_mul(16)); let d_ = (l_p_6).wrapping_add(176); rt::memcpy(c, d_, s_, 16); };
    }
    { let v_ = p_chara_2; let a_ = (l_p_6).wrapping_add(152); c.m.w32(a_, (v_) as u32); };
    { let v_ = (p_set_4).wrapping_add(1i32); let a_ = (l_p_6).wrapping_add(156); c.m.w32(a_, (v_) as u32); };
    { let v_ = F::ChrTbl_GetLevel(c, p_chara_2, p_slot_3, p_fromRec_5); let a_ = (l_p_6).wrapping_add(160); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a1_ = c.m.r32((l_p_6).wrapping_add(1732)); let a2_ = (c.m.r32((l_p_6).wrapping_add(160)) as i32); F::ItemSet_Fit(c, (l_p_6).wrapping_add(176), a1_, a2_, (l_p_6).wrapping_add(168)) }; let a_ = (l_p_6).wrapping_add(164); c.m.w32(a_, (v_) as u32); };
    if ((p_fromRec_5) != 0) {
        { let v_ = 10i32; let a_ = (l_p_6).wrapping_add(164); c.m.w32(a_, (v_) as u32); };
        { let v_ = 10i32; let a_ = (l_p_6).wrapping_add(160); c.m.w32(a_, (v_) as u32); };
    }
    { let v_ = 0i32; let a_ = (l_p_6).wrapping_add(172); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ItemPanel_SetChara", &PROBE_N, &[], true); }
    r_
}

pub fn ItemPanel_Show(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ItemPanel_Show", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
    l_p_2 = c.m.r32((sym::gItemPanel).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)));
    F::Flash_GotoLabel(c, (l_p_2).wrapping_add(12), sym::menu_g__str29, ((((1i32) as i64 as u64)) as u8));
    { let r_ = 8i32; let a_ = (l_p_2).wrapping_add(148); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    { let r_ = (!(16i32)); let a_ = (l_p_2).wrapping_add(148); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    if (((((c.m.r32((l_p_2).wrapping_add(148)) as i32)) & (4i32))) != 0) {
        F::ItemPanel_ClipGoto(c, p_side_1, 0i32, sym::menu_g__str30);
        F::ItemPanel_ClipGoto(c, p_side_1, 1i32, sym::menu_g__str31);
    }
    { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 4i32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ItemPanel_Show", &PROBE_N, &[], true); }
    r_
}

pub fn ItemPanel_Hide(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ItemPanel_Hide", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
    l_p_2 = c.m.r32((sym::gItemPanel).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)));
    F::Flash_GotoLabel(c, (l_p_2).wrapping_add(12), sym::menu_g__str32, ((((1i32) as i64 as u64)) as u8));
    { let r_ = 16i32; let a_ = (l_p_2).wrapping_add(148); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 4i32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ItemPanel_Hide", &PROBE_N, &[], true); }
    r_
}
