#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn ItemHelp_Init(c: &mut Ctx, mut p_pack_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ItemHelp_Init", &PROBE_N, &[(p_pack_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_box_2: u32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = F::Heap_Alloc(c, ((384u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gItemHelp; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gItemHelp); F::memset(c, a0_, 0i32, 384u32) }; };
        { let v_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gItemHelp)).wrapping_add(52)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gItemHelp)).wrapping_add(52)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((8i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gItemHelp)).wrapping_add(52)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((9i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gItemHelp)).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((3i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gItemHelp)).wrapping_add(52)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gItemHelp)).wrapping_add(52)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gItemHelp)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a1_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gItemHelp)).wrapping_add(52); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gItemHelp)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let v_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gItemHelp)).wrapping_add(100); c.m.w32(a_, v_); };
        { let v_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((6i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = c.m.r32(sym::gItemHelp); c.m.w32(a_, v_); };
        { let v_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((5i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gItemHelp)).wrapping_add(4); c.m.w32(a_, v_); };
        l_box_2 = (c.m.r32(sym::gItemHelp)).wrapping_add(104);
        { let a1_ = { let a_ = c.m.r32(sym::gItemHelp); c.m.r32(a_) }; F::TextBox_Init(c, l_box_2, a1_, ((6i32) as u32)) };
        F::TextBox_SetUnk80(c, l_box_2, 1i32);
        l_box_2 = (c.m.r32(sym::gItemHelp)).wrapping_add(244);
        { let a1_ = { let a_ = (c.m.r32(sym::gItemHelp)).wrapping_add(4); c.m.r32(a_) }; F::TextBox_Init(c, l_box_2, a1_, ((0i32) as u32)) };
        F::TextBox_SetUnk50(c, l_box_2, 0i32);
        F::TextBox_SetUnk80(c, l_box_2, 1i32);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ItemHelp_Init", &PROBE_N, &[], true); }
    r_
}

pub fn ItemHelp_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ItemHelp_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gItemHelp)).wrapping_add(8)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Destroy(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if (((((c.m.r32(sym::gItemHelp)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gItemHelp); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gItemHelp; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ItemHelp_Term", &PROBE_N, &[], true); }
    r_
}

pub fn ItemHelp_Draw(c: &mut Ctx, mut p_item_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ItemHelp_Draw", &PROBE_N, &[(p_item_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_flash_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < (1i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a0_ = ((c.m.r32(sym::gItemHelp)).wrapping_add(8)).wrapping_add((((l_i_3) as u32)).wrapping_mul(44)); F::Flash_Advance(c, a0_) };
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        l_flash_2 = ((c.m.r32(sym::gItemHelp)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_v_c__str0, fp);
        { let a5_ = (c.m.r32(sym::gItemHelp)).wrapping_add(104); F::TextBox_AttachLine(c, l_flash_2, fp, 0i32, 0i32, p_item_1, a5_) };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_v_c__str1, fp);
        { let a5_ = (c.m.r32(sym::gItemHelp)).wrapping_add(244); F::TextBox_AttachLine(c, l_flash_2, fp, 0i32, 0i32, p_item_1, a5_) };
        { let v_ = ((({ let a_ = ({ let a_ = (c.m.r32(sym::gItemHelp)).wrapping_add(100); c.m.r32(a_) }).wrapping_add((((p_item_1) as u32)).wrapping_mul(40)); c.m.r8(a_) }) as i32)).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_v_c__str2, fp);
        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_v_c__str3, fp);
        if ((((((({ let a_ = (({ let a_ = (c.m.r32(sym::gItemHelp)).wrapping_add(100); c.m.r32(a_) }).wrapping_add((((p_item_1) as u32)).wrapping_mul(40))).wrapping_add(3); c.m.r8(a_) }) as i32)) != (0i32)) as i32)) != 0) {
            { let v_ = (((({ let a_ = (({ let a_ = (c.m.r32(sym::gItemHelp)).wrapping_add(100); c.m.r32(a_) }).wrapping_add((((p_item_1) as u32)).wrapping_mul(40))).wrapping_add(3); c.m.r8(a_) }) as i32)).wrapping_mul(32i32)).wrapping_sub(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = ({ let a1_ = { let a_ = (c.m.r32(sym::gItemHelp)).wrapping_add(100); c.m.r32(a_) }; F::ItemTbl_GetClass(c, p_item_1, a1_) }).wrapping_mul(42i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(42i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
            F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
        } else {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
        }
        { let v_ = ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gItemHelp)).wrapping_add(100); c.m.r32(a_) }).wrapping_add((((p_item_1) as u32)).wrapping_mul(40))).wrapping_add(2); c.m.r8(a_) }) as i32)) & (1i32))).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = (((({ let a_ = (({ let a_ = (c.m.r32(sym::gItemHelp)).wrapping_add(100); c.m.r32(a_) }).wrapping_add((((p_item_1) as u32)).wrapping_mul(40))).wrapping_add(2); c.m.r8(a_) }) as i32)).wrapping_shr((1i32) as u32)).wrapping_mul(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_v_c__str4, fp);
        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_v_c__str5, fp);
        { let a2_ = (({ let a_ = (({ let a_ = (c.m.r32(sym::gItemHelp)).wrapping_add(100); c.m.r32(a_) }).wrapping_add((((p_item_1) as u32)).wrapping_mul(40))).wrapping_add(2); c.m.r8(a_) }) as i32); F::Flash_ClipSetTex(c, l_flash_2, fp, a2_) };
        { let a0_ = ((c.m.r32(sym::gItemHelp)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
        F::Font_FlushAll(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ItemHelp_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn ItemHelp_Open(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ItemHelp_Open", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = ((c.m.r32(sym::gItemHelp)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_v_c__str6, ((((1i32) as i64 as u64)) as u8)) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ItemHelp_Open", &PROBE_N, &[], true); }
    r_
}

pub fn ItemHelp_Close(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ItemHelp_Close", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = ((c.m.r32(sym::gItemHelp)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_v_c__str7, ((((1i32) as i64 as u64)) as u8)) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ItemHelp_Close", &PROBE_N, &[], true); }
    r_
}
