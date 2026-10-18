#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn menu_za_b__PassWin_Section(c: &mut Ctx, mut p_pack_1: u32, mut p_n_2: i32, mut p_host_3: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_za_b__PassWin_Section", &PROBE_N, &[(p_pack_1) as u32, (p_n_2) as u32, (p_host_3) as u32], false); }
    let r_ = 'f: {
    break 'f (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((p_n_2) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_za_b__PassWin_Section", &PROBE_N, &[], true); }
    r_
}

pub fn menu_za_b__PassWin_SetCell(c: &mut Ctx, mut p_uv_1: u32, mut p_col_2: i32, mut p_row_3: i32, mut p_w_4: i32, mut p_h_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_za_b__PassWin_SetCell", &PROBE_N, &[(p_uv_1) as u32, (p_col_2) as u32, (p_row_3) as u32, (p_w_4) as u32, (p_h_5) as u32], false); }
    let r_ = 'f: {
    { let v_ = (p_col_2).wrapping_mul(p_w_4); let a_ = p_uv_1; c.m.w32(a_, (v_) as u32); };
    { let v_ = ((c.m.r32(p_uv_1) as i32)).wrapping_add(p_w_4); let a_ = (p_uv_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = (p_row_3).wrapping_mul(p_h_5); let a_ = (p_uv_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((c.m.r32((p_uv_1).wrapping_add(4)) as i32)).wrapping_add(p_h_5); let a_ = (p_uv_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_za_b__PassWin_SetCell", &PROBE_N, &[], true); }
    r_
}

pub fn menu_za_b__PassWin_SetUv(c: &mut Ctx, mut p_flash_1: u32, mut p_parent_2: u32, mut p_name_3: u32, mut p_uv_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_za_b__PassWin_SetUv", &PROBE_N, &[(p_flash_1) as u32, (p_parent_2) as u32, (p_name_3) as u32, (p_uv_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        F::Flash_FindLabel(c, p_flash_1, p_parent_2, p_name_3, fp);
        F::Flash_ClipSetUv(c, p_flash_1, fp, p_uv_4);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "menu_za_b__PassWin_SetUv", &PROBE_N, &[], true); }
    r_
}

pub fn PassWin_Init(c: &mut Ctx, mut p_pack_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "PassWin_Init", &PROBE_N, &[(p_pack_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = F::Heap_Alloc(c, ((60u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gPassWin; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gPassWin); F::memset(c, a0_, 0i32, 60u32) }; };
        { let v_ = F::menu_za_b__PassWin_Section(c, p_pack_1, 2i32, sym::menu_za_b__str0); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gPassWin)).wrapping_add(44)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gPassWin)).wrapping_add(44)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gPassWin)).wrapping_add(44)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::menu_za_b__PassWin_Section(c, p_pack_1, 3i32, sym::menu_za_b__str1); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gPassWin)).wrapping_add(44)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = (c.m.r32(sym::gPassWin)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a1_ = F::menu_za_b__PassWin_Section(c, p_pack_1, 1i32, sym::menu_za_b__str2); let a2_ = (c.m.r32(sym::gPassWin)).wrapping_add(44); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = (c.m.r32(sym::gPassWin)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "PassWin_Init", &PROBE_N, &[], true); }
    r_
}

pub fn PassWin_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "PassWin_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = (c.m.r32(sym::gPassWin)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Destroy(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if (((((c.m.r32(sym::gPassWin)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gPassWin); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gPassWin; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "PassWin_Term", &PROBE_N, &[], true); }
    r_
}

pub fn PassWin_FindKey(c: &mut Ctx, mut p_c_1: i8, mut p_col_2: u32, mut p_row_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "PassWin_FindKey", &PROBE_N, &[(p_c_1) as u32, (p_col_2) as u32, (p_row_3) as u32], false); }
    let mut l_i_4: i32 = 0;
    let mut l_j_5: i32 = 0;
    let mut l_x__6: i32 = 0;
    let r_ = 'f: {
    if (((((((((((p_c_1) as i32)) >= (97i32)) as i32)) != 0) && (((((((p_c_1) as i32)) <= (122i32)) as i32)) != 0)) as i32)) != 0) {
        p_c_1 = (({ l_x__6 = ((p_c_1) as i32); (if ((((((c.m.r8(((sym::_ctype_).wrapping_add(((1i32) as u32))).wrapping_add(((l_x__6) as u32)))) as i32)) & (2i32))) != 0) { ((l_x__6).wrapping_sub(97i32)).wrapping_add(65i32) } else { l_x__6 }) }) as i8);
    }
    {
        l_j_5 = 0i32;
        'l1: loop {
            if !(((((l_j_5) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                {
                    l_i_4 = 0i32;
                    'l3: loop {
                        if !(((((l_i_4) < (14i32)) as i32)) != 0) { break 'l3; }
                        'c4: {
                            if (((((((p_c_1) as i32)) == ((((c.m.r8(((sym::gPassKeys).wrapping_add((((l_j_5) as u32)).wrapping_mul(14))).wrapping_add(((l_i_4) as u32))) as i8)) as i32))) as i32)) != 0) {
                                { let v_ = l_i_4; let a_ = p_col_2; c.m.w32(a_, (v_) as u32); };
                                { let v_ = l_j_5; let a_ = p_row_3; c.m.w32(a_, (v_) as u32); };
                                break 'f;
                            }
                        }
                        { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                    }
                }
            }
            { let o_ = l_j_5; let n_ = (o_).wrapping_add(1); l_j_5 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "PassWin_FindKey", &PROBE_N, &[], true); }
    r_
}

pub fn PassWin_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "PassWin_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(288) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 288);
    let mut l_text_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_c_3: i8 = 0;
    let r_ = 'f: {
        l_i_2 = 0i32;
        l_text_1 = F::ChrPass_GetText(c);
        { let a0_ = (c.m.r32(sym::gPassWin)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Advance(c, a0_) };
        'l1: while ((((((((c.m.r8((l_text_1).wrapping_add(((l_i_2) as u32))) as i8)) as i32)) != (0i32)) as i32)) != 0) {
            l_c_3 = (c.m.r8((l_text_1).wrapping_add(((l_i_2) as u32))) as i8);
            F::PassWin_FindKey(c, l_c_3, fp.wrapping_add(276), fp.wrapping_add(280));
            if (((((((((((l_c_3) as i32)) >= (97i32)) as i32)) != 0) && (((((((l_c_3) as i32)) <= (122i32)) as i32)) != 0)) as i32)) != 0) {
                { let a1_ = (c.m.r32(fp.wrapping_add(276)) as i32); let a2_ = ((c.m.r32(fp.wrapping_add(280)) as i32)).wrapping_add(4i32); F::menu_za_b__PassWin_SetCell(c, fp, a1_, a2_, 32i32, 40i32) };
            } else {
                { let a1_ = (c.m.r32(fp.wrapping_add(276)) as i32); let a2_ = (c.m.r32(fp.wrapping_add(280)) as i32); F::menu_za_b__PassWin_SetCell(c, fp, a1_, a2_, 32i32, 40i32) };
            }
            if (((((l_i_2) <= (16i32)) as i32)) != 0) {
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                { let _ = F::sprintf(c, fp.wrapping_add(20), sym::menu_za_b__str3, &[((1i32) as i64 as u64), ((l_i_2) as i64 as u64)]); };
            } else {
                { let _ = F::sprintf(c, fp.wrapping_add(20), sym::menu_za_b__str4, &[((2i32) as i64 as u64), (((l_i_2).wrapping_sub(16i32)) as i64 as u64)]); };
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
            { let a0_ = (c.m.r32(sym::gPassWin)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::menu_za_b__PassWin_SetUv(c, a0_, ((0i32) as u32), fp.wrapping_add(20), fp) };
            if (((((l_i_2) > (34i32)) as i32)) != 0) {
                break 'l1;
            }
        }
        { let a0_ = (c.m.r32(sym::gPassWin)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "PassWin_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn PassWin_Open(c: &mut Ctx, mut p_items_1: u32, mut p_chara_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "PassWin_Open", &PROBE_N, &[(p_items_1) as u32, (p_chara_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < (8i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = ((c.m.r16((p_items_1).wrapping_add((((l_i_3) as u32)).wrapping_mul(2)))) as i32); let a_ = ((fp).wrapping_add(4)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        { let v_ = p_chara_2; let a_ = fp; c.m.w32(a_, (v_) as u32); };
        { let _ = F::ChrPass_Encode(c, fp); };
        { let a0_ = (c.m.r32(sym::gPassWin)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_za_b__str5, ((((1i32) as i64 as u64)) as u8)) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "PassWin_Open", &PROBE_N, &[], true); }
    r_
}

pub fn PassWin_OpenEx(c: &mut Ctx, mut p_items_1: u32, mut p_chara_2: i32, mut p_extraSlots_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "PassWin_OpenEx", &PROBE_N, &[(p_items_1) as u32, (p_chara_2) as u32, (p_extraSlots_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < (8i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = ((c.m.r16((p_items_1).wrapping_add((((l_i_4) as u32)).wrapping_mul(2)))) as i32); let a_ = ((fp).wrapping_add(4)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        { let v_ = p_chara_2; let a_ = fp; c.m.w32(a_, (v_) as u32); };
        { let v_ = p_extraSlots_3; let a_ = (fp).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        { let _ = F::ChrPass_Encode(c, fp); };
        { let a0_ = (c.m.r32(sym::gPassWin)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_za_b__str6, ((((1i32) as i64 as u64)) as u8)) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "PassWin_OpenEx", &PROBE_N, &[], true); }
    r_
}

pub fn PassWin_Close(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "PassWin_Close", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32(sym::gPassWin)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_za_b__str7, ((((1i32) as i64 as u64)) as u8)) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "PassWin_Close", &PROBE_N, &[], true); }
    r_
}
