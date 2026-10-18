#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn menu_y_b__DcView_SetChildUv(c: &mut Ctx, mut p_v_1: u32, mut p_parent_2: u32, mut p_name_3: u32, mut p_uv_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_y_b__DcView_SetChildUv", &PROBE_N, &[(p_v_1) as u32, (p_parent_2) as u32, (p_name_3) as u32, (p_uv_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        F::Flash_FindLabel(c, p_v_1, p_parent_2, p_name_3, fp);
        F::Flash_ClipSetUv(c, p_v_1, fp, p_uv_4);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "menu_y_b__DcView_SetChildUv", &PROBE_N, &[], true); }
    r_
}

pub fn menu_y_b__DcView_ShowChild(c: &mut Ctx, mut p_v_1: u32, mut p_parent_2: u32, mut p_name_3: u32, mut p_on_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_y_b__DcView_ShowChild", &PROBE_N, &[(p_v_1) as u32, (p_parent_2) as u32, (p_name_3) as u32, (p_on_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        F::Flash_FindLabel(c, p_v_1, p_parent_2, p_name_3, fp);
        { let _ = F::Flash_ClipSetFlags(c, p_v_1, fp, 2i32, ((((p_on_4) as i64 as u64)) as u8)); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "menu_y_b__DcView_ShowChild", &PROBE_N, &[], true); }
    r_
}

pub fn menu_y_b__DcUv_SetRow(c: &mut Ctx, mut p_uv_1: u32, mut p_row_2: i32, mut p_w_3: i32, mut p_h_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_y_b__DcUv_SetRow", &PROBE_N, &[(p_uv_1) as u32, (p_row_2) as u32, (p_w_3) as u32, (p_h_4) as u32], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = p_uv_1; c.m.w32(a_, (v_) as u32); };
    { let v_ = (p_row_2).wrapping_mul(p_h_4); let a_ = (p_uv_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_w_3; let a_ = (p_uv_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((p_row_2).wrapping_mul(p_h_4)).wrapping_add(p_h_4); let a_ = (p_uv_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_y_b__DcUv_SetRow", &PROBE_N, &[], true); }
    r_
}

pub fn DcView_InitBlink(c: &mut Ctx, mut p_v_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcView_InitBlink", &PROBE_N, &[(p_v_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((F::Rand_Range(c, ((32i32) as u32))) as i32); let a_ = (p_v_1).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcView_InitBlink", &PROBE_N, &[], true); }
    r_
}

pub fn DcChars_Load(c: &mut Ctx, mut p_c_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcChars_Load", &PROBE_N, &[(p_c_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let mut l_off_3: i32 = 0;
    let r_ = 'f: {
    { let v_ = (1i32).wrapping_neg(); let a_ = (p_c_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (14i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_off_3 = ((((((l_i_2) as u32)).wrapping_mul(28u32)).wrapping_add(((11584i32) as u32))) as i32);
                { let s_ = (c.m.r32(sym::gSaveData)).wrapping_add(((l_off_3) as u32)); let d_ = ((p_c_1).wrapping_add(12)).wrapping_add((((l_i_2) as u32)).wrapping_mul(28)); rt::memcpy(c, d_, s_, 28); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcChars_Load", &PROBE_N, &[], true); }
    r_
}

pub fn DcStatus_Calc(c: &mut Ctx, mut p_s_1: u32, mut p_table_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcStatus_Calc", &PROBE_N, &[(p_s_1) as u32, (p_table_2) as u32], false); }
    let r_ = 'f: {
    F::ItemSet_GetBonus(c, (p_s_1).wrapping_add(28), p_table_2, p_s_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcStatus_Calc", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_Reset(c: &mut Ctx, mut p_d_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_Reset", &PROBE_N, &[(p_d_1) as u32], false); }
    let r_ = 'f: {
    F::DcView_InitBlink(c, (p_d_1).wrapping_add(20));
    F::DcChars_Load(c, (p_d_1).wrapping_add(3248));
    { let v_ = 30i32; let a_ = (p_d_1).wrapping_add(3708); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcList_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_UpdateGuide(c: &mut Ctx, mut p_d_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_UpdateGuide", &PROBE_N, &[(p_d_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_flash_2: u32 = 0;
    let r_ = 'f: {
        l_flash_2 = (p_d_1).wrapping_add(20);
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_y_b__str0, fp);
        F::FlashAnim_Blink(c, l_flash_2, fp, ((p_d_1).wrapping_add(20)).wrapping_add(312), 0i32);
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_y_b__str1, fp);
        if (((((F::Voice_GetStat(c, 0i32)) != (5i32)) as i32)) != 0) {
            F::FlashAnim_Talk(c, l_flash_2, fp, ((p_d_1).wrapping_add(20)).wrapping_add(316), 0i32);
        } else {
            F::FlashAnim_ShowNext2(c, l_flash_2, fp, 0i32);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcList_UpdateGuide", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_ScrollCloud(c: &mut Ctx, mut p_d_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_ScrollCloud", &PROBE_N, &[(p_d_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_flash_2: u32 = 0;
    let r_ = 'f: {
        l_flash_2 = (p_d_1).wrapping_add(20);
        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_y_b__str2, fp);
        F::FlashAnim_Scroll(c, l_flash_2, fp, fp.wrapping_add(8), (p_d_1).wrapping_add(3224), ((0i32) as u32), F32(0xbe1c09c0), F32(0x00000000));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcList_ScrollCloud", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_SetListScissor(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_SetListScissor", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Sprite_SetScissor(c, 175i32, 511i32, 106i32, 313i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcList_SetListScissor", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_ResetScissor(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_ResetScissor", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Sprite_SetScissor(c, 0i32, 511i32, 0i32, 447i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcList_ResetScissor", &PROBE_N, &[], true); }
    r_
}

pub fn DcView_SetRowCallbacks(c: &mut Ctx, mut p_v_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcView_SetRowCallbacks", &PROBE_N, &[(p_v_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(272) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 272);
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_y_b__str3, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, p_v_1, ((0i32) as u32), fp.wrapping_add(8), fp);
                    F::Flash_ClipSetCallbackA(c, p_v_1, fp, fid::DcList_SetListScissor, ((0i32) as u32));
                    F::Flash_ClipSetCallbackB(c, p_v_1, fp, fid::DcList_ResetScissor, ((0i32) as u32));
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcView_SetRowCallbacks", &PROBE_N, &[], true); }
    r_
}

pub fn DcView_HideArrow(c: &mut Ctx, mut p_v_1: u32, mut p_up_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcView_HideArrow", &PROBE_N, &[(p_v_1) as u32, (p_up_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(528) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 528);
    let r_ = 'f: {
        if ((p_up_2) != 0) {
            { let _ = F::strcpy(c, fp.wrapping_add(8), sym::menu_y_b__str4); };
            { let _ = F::strcpy(c, fp.wrapping_add(264), sym::menu_y_b__str5); };
        } else {
            { let _ = F::strcpy(c, fp.wrapping_add(8), sym::menu_y_b__str6); };
            { let _ = F::strcpy(c, fp.wrapping_add(264), sym::menu_y_b__str7); };
        }
        F::Flash_FindLabel(c, p_v_1, fp.wrapping_add(8), fp.wrapping_add(264), fp);
        { let _ = F::Flash_ClipSetFlags(c, p_v_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcView_HideArrow", &PROBE_N, &[], true); }
    r_
}

pub fn DcView_SetArrows(c: &mut Ctx, mut p_v_1: u32, mut p_c_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcView_SetArrows", &PROBE_N, &[(p_v_1) as u32, (p_c_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_top_3: i32 = 0;
    let r_ = 'f: {
        { let v_ = 0i32; let a_ = fp; c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::menu_y_b__DcView_SetChildUv(c, p_v_1, sym::menu_y_b__str8, sym::menu_y_b__str9, fp);
        { let v_ = 32i32; let a_ = fp; c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::menu_y_b__DcView_SetChildUv(c, p_v_1, sym::menu_y_b__str10, sym::menu_y_b__str11, fp);
        l_top_3 = (c.m.r32(p_c_2) as i32);
        if (((((l_top_3) == (0i32)) as i32)) != 0) {
            F::DcView_HideArrow(c, p_v_1, 1i32);
        } else {
            if (((((l_top_3) == (11i32)) as i32)) != 0) {
                F::DcView_HideArrow(c, p_v_1, 0i32);
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcView_SetArrows", &PROBE_N, &[], true); }
    r_
}

pub fn DcView_SetScrollBar(c: &mut Ctx, mut p_v_1: u32, mut p_c_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcView_SetScrollBar", &PROBE_N, &[(p_v_1) as u32, (p_c_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_n_3: i32 = 0;
    let mut l_y_4: i32 = 0;
    let r_ = 'f: {
        l_n_3 = 14i32;
        l_y_4 = rt::div_i32(((c.m.r32(p_c_2) as i32)).wrapping_mul(181i32), l_n_3);
        F::Flash_FindLabel(c, p_v_1, ((0i32) as u32), sym::menu_y_b__str12, fp);
        F::Flash_ClipSetScale(c, p_v_1, fp, F32(0x3f800000), F32(0x3f9b3ee6));
        F::Flash_ClipSetOffset(c, p_v_1, fp, 0i32, l_y_4);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcView_SetScrollBar", &PROBE_N, &[], true); }
    r_
}

pub fn DcView_LightRow(c: &mut Ctx, mut p_v_1: u32, mut p_c_2: u32, mut p_on_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcView_LightRow", &PROBE_N, &[(p_v_1) as u32, (p_c_2) as u32, (p_on_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(272) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 272);
    let r_ = 'f: {
        { let _ = { let a2_ = ((c.m.r32((p_c_2).wrapping_add(4)) as i32)).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_y_b__str13, &[((a2_) as i64 as u64)]) }; };
        F::Flash_FindLabel(c, p_v_1, ((0i32) as u32), fp.wrapping_add(8), fp);
        if ((p_on_3) != 0) {
            F::Flash_ClipGotoLabel(c, p_v_1, fp, sym::menu_y_b__str14);
        } else {
            F::Flash_ClipGotoLabel(c, p_v_1, fp, sym::menu_y_b__str15);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcView_LightRow", &PROBE_N, &[], true); }
    r_
}

pub fn DcChars_ClampTop(c: &mut Ctx, mut p_c_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcChars_ClampTop", &PROBE_N, &[(p_c_1) as u32], false); }
    let r_ = 'f: {
    if ((((((c.m.r32(p_c_1) as i32)) < (0i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = p_c_1; c.m.w32(a_, (v_) as u32); };
        break 'f 0i32;
    }
    if ((((((c.m.r32(p_c_1) as i32)) > (11i32)) as i32)) != 0) {
        { let v_ = 11i32; let a_ = p_c_1; c.m.w32(a_, (v_) as u32); };
        break 'f 0i32;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "DcChars_ClampTop", &PROBE_N, &[], true); }
    r_
}

pub fn DcChars_ClampRow(c: &mut Ctx, mut p_c_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcChars_ClampRow", &PROBE_N, &[(p_c_1) as u32], false); }
    let r_ = 'f: {
    if ((((((c.m.r32((p_c_1).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (p_c_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        break 'f 1i32;
    }
    if ((((((c.m.r32((p_c_1).wrapping_add(4)) as i32)) > ((3i32).wrapping_sub(1i32))) as i32)) != 0) {
        { let v_ = (3i32).wrapping_sub(1i32); let a_ = (p_c_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "DcChars_ClampRow", &PROBE_N, &[], true); }
    r_
}

pub fn DcView_SetRows(c: &mut Ctx, mut p_v_1: u32, mut p_c_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcView_SetRows", &PROBE_N, &[(p_v_1) as u32, (p_c_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(272) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 272);
    let mut l_i_3: i32 = 0;
    let mut l_chara_4: i32 = 0;
    let r_ = 'f: {
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_y_b__str16, &[(((l_i_3).wrapping_add(1i32)) as i64 as u64)]); };
                    if (((((l_i_3) != (3i32)) as i32)) != 0) {
                        l_chara_4 = { let a_ = (((p_c_2).wrapping_add(12)).wrapping_add(((((l_i_3).wrapping_add((c.m.r32(p_c_2) as i32))) as u32)).wrapping_mul(28))).wrapping_add(24); (c.m.r32(a_) as i32) };
                    } else {
                        l_chara_4 = { let a_ = (((p_c_2).wrapping_add(12)).wrapping_add(((((c.m.r32((p_c_2).wrapping_add(8)) as i32)) as u32)).wrapping_mul(28))).wrapping_add(24); (c.m.r32(a_) as i32) };
                    }
                    if (((((l_i_3) == ((c.m.r32((p_c_2).wrapping_add(4)) as i32))) as i32)) != 0) {
                        if (((((l_chara_4) == ((1i32).wrapping_neg())) as i32)) != 0) {
                            F::Flash_FindLabel(c, p_v_1, ((0i32) as u32), fp.wrapping_add(8), fp);
                            F::Flash_ClipSetAlpha(c, p_v_1, fp, F32(0x3f800000));
                            break 'c2;
                        }
                    } else {
                        if (((((l_chara_4) == ((1i32).wrapping_neg())) as i32)) != 0) {
                            F::Flash_FindLabel(c, p_v_1, ((0i32) as u32), fp.wrapping_add(8), fp);
                            F::Flash_ClipSetAlpha(c, p_v_1, fp, F32(0x3f000000));
                            break 'c2;
                        }
                    }
                    F::Flash_FindLabel(c, p_v_1, fp.wrapping_add(8), sym::menu_y_b__str17, fp);
                    F::TextBox_AttachLine(c, p_v_1, fp, 0i32, 0i32, l_chara_4, ((p_v_1).wrapping_add(320)).wrapping_add((((l_i_3) as u32)).wrapping_mul(140)));
                    F::Flash_FindLabel(c, p_v_1, fp.wrapping_add(8), sym::menu_y_b__str18, fp);
                    F::TextBox_AttachLine(c, p_v_1, fp, 0i32, 0i32, l_chara_4, ((p_v_1).wrapping_add(880)).wrapping_add((((l_i_3) as u32)).wrapping_mul(140)));
                    F::Flash_FindLabel(c, p_v_1, ((0i32) as u32), fp.wrapping_add(8), fp);
                    F::Flash_ClipSetAlpha(c, p_v_1, fp, F32(0x3f800000));
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcView_SetRows", &PROBE_N, &[], true); }
    r_
}

pub fn DcChars_GetCursor(c: &mut Ctx, mut p_c_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcChars_GetCursor", &PROBE_N, &[(p_c_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((c.m.r32(p_c_1) as i32)).wrapping_add((c.m.r32((p_c_1).wrapping_add(4)) as i32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "DcChars_GetCursor", &PROBE_N, &[], true); }
    r_
}

pub fn DcChars_GetRec(c: &mut Ctx, sret: u32, mut p_c_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcChars_GetRec", &PROBE_N, &[(sret) as u32, (p_c_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let s_ = ((p_c_1).wrapping_add(12)).wrapping_add((((F::DcChars_GetCursor(c, p_c_1)) as u32)).wrapping_mul(28)); rt::memcpy(c, sret, s_, 28); sret };
       sret
    };
    if rt::probing() { crate::host::harness::probe(c, "DcChars_GetRec", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_PickRec(c: &mut Ctx, mut p_d_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_PickRec", &PROBE_N, &[(p_d_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_s_2: u32 = 0;
    let mut l_tbl_3: u32 = 0;
    let r_ = 'f: {
        l_s_2 = (p_d_1).wrapping_add(3652);
        { let s_ = F::DcChars_GetRec(c, fp, (p_d_1).wrapping_add(3248)); let d_ = ((p_d_1).wrapping_add(3652)).wrapping_add(28); rt::memcpy(c, d_, s_, 28); };
        { let a1_ = c.m.r32((p_d_1).wrapping_add(3712)); F::DcStatus_Calc(c, l_s_2, a1_) };
        l_tbl_3 = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
        { let v_ = (((((({ let a_ = ((l_tbl_3).wrapping_add(((((c.m.r32(((l_s_2).wrapping_add(28)).wrapping_add(24)) as i32)) as u32)).wrapping_mul(60))).wrapping_add(8); c.m.r16(a_) }) as i32)) ^ (1i32))) & (1i32)); let a_ = (l_s_2).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcList_PickRec", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_RowOk(c: &mut Ctx, mut p_d_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_RowOk", &PROBE_N, &[(p_d_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(272) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 272);
    let mut l_flash_2: u32 = 0;
    let r_ = 'f: {
        { let _ = { let a2_ = ((c.m.r32(((p_d_1).wrapping_add(3248)).wrapping_add(4)) as i32)).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_y_b__str19, &[((a2_) as i64 as u64)]) }; };
        l_flash_2 = (p_d_1).wrapping_add(20);
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_y_b__str20);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcList_RowOk", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_DrawList(c: &mut Ctx, mut p_d_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_DrawList", &PROBE_N, &[(p_d_1) as u32], false); }
    let mut l_v_2: u32 = 0;
    let mut l_c_3: u32 = 0;
    let r_ = 'f: {
    l_v_2 = (p_d_1).wrapping_add(20);
    l_c_3 = (p_d_1).wrapping_add(3248);
    F::DcView_SetRowCallbacks(c, l_v_2);
    F::DcView_SetArrows(c, l_v_2, l_c_3);
    F::DcView_SetScrollBar(c, l_v_2, l_c_3);
    F::DcView_SetRows(c, l_v_2, l_c_3);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcList_DrawList", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_InputList(c: &mut Ctx, mut p_d_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_InputList", &PROBE_N, &[(p_d_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_c_2: u32 = 0;
    let mut l_v_3: u32 = 0;
    let mut l_c_4: u32 = 0;
    let mut l_v_5: u32 = 0;
    let mut l_v_6: u32 = 0;
    let mut l_top_7: i32 = 0;
    let mut l_c_8: u32 = 0;
    let mut l_top_9: i32 = 0;
    let mut l_c_10: u32 = 0;
    let r_ = 'f: {
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
            l_c_2 = (p_d_1).wrapping_add(3248);
            if (((((F::DcChars_GetCursor(c, l_c_2)) != (0i32)) as i32)) != 0) {
                l_v_3 = (p_d_1).wrapping_add(20);
                F::DcView_LightRow(c, l_v_3, l_c_2, 0i32);
                { let a_ = ((p_d_1).wrapping_add(3248)).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                if ((F::DcChars_ClampRow(c, l_c_2)) != 0) {
                    { let a_ = l_c_2; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    if ((F::DcChars_ClampTop(c, l_c_2)) != 0) {
                        F::Flash_GotoLabel(c, l_v_3, sym::menu_y_b__str21, ((((1i32) as i64 as u64)) as u8));
                        { let v_ = ((c.m.r32(l_c_2) as i32)).wrapping_add(3i32); let a_ = ((p_d_1).wrapping_add(3248)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    }
                }
                F::DcView_LightRow(c, l_v_3, l_c_2, 1i32);
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                break 'f;
            }
        }
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
            l_c_4 = (p_d_1).wrapping_add(3248);
            if (((((F::DcChars_GetCursor(c, l_c_4)) != ((14i32).wrapping_sub(1i32))) as i32)) != 0) {
                l_v_5 = (p_d_1).wrapping_add(20);
                F::DcView_LightRow(c, l_v_5, l_c_4, 0i32);
                { let a_ = ((p_d_1).wrapping_add(3248)).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                if ((F::DcChars_ClampRow(c, l_c_4)) != 0) {
                    { let a_ = l_c_4; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    if ((F::DcChars_ClampTop(c, l_c_4)) != 0) {
                        F::Flash_GotoLabel(c, l_v_5, sym::menu_y_b__str22, ((((1i32) as i64 as u64)) as u8));
                        { let v_ = ((c.m.r32(l_c_4) as i32)).wrapping_sub(1i32); let a_ = ((p_d_1).wrapping_add(3248)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    }
                }
                F::DcView_LightRow(c, l_v_5, l_c_4, 1i32);
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                break 'f;
            }
        }
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            { let s_ = F::DcChars_GetRec(c, fp.wrapping_add(28), (p_d_1).wrapping_add(3248)); let d_ = fp; rt::memcpy(c, d_, s_, 28); };
            if ((((((c.m.r32((fp).wrapping_add(24)) as i32)) >= (0i32)) as i32)) != 0) {
                F::DcList_RowOk(c, p_d_1);
                l_v_6 = (p_d_1).wrapping_add(20);
                F::Flash_GotoLabel(c, l_v_6, sym::menu_y_b__str23, ((((1i32) as i64 as u64)) as u8));
                F::MsgWin_Close(c);
                { let v_ = 1i32; let a_ = (p_d_1).wrapping_add(3236); c.m.w32(a_, (v_) as u32); };
                { let a1_ = (c.m.r32((p_d_1).wrapping_add(3240)) as i32); F::DcView_LightMenu(c, l_v_6, a1_, 0i32) };
                { let v_ = 0i32; let a_ = (p_d_1).wrapping_add(3240); c.m.w32(a_, (v_) as u32); };
                F::DcView_LightMenu(c, l_v_6, 0i32, 1i32);
                F::DcList_PickRec(c, p_d_1);
                { let r_ = 1i32; let a_ = (p_d_1).wrapping_add(3716); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
            } else {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
            }
            F::DcList_RowOk(c, p_d_1);
            break 'f;
        }
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
            { let v_ = 0i32; let a_ = (p_d_1).wrapping_add(3228); c.m.w32(a_, (v_) as u32); };
            { let r_ = 16i32; let a_ = (p_d_1).wrapping_add(3716); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            F::Flash_GotoLabel(c, (p_d_1).wrapping_add(20), sym::menu_y_b__str24, ((((1i32) as i64 as u64)) as u8));
            F::MsgWin_Close(c);
            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
            break 'f;
        }
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1i32) as u32)))) != 0) {
            l_top_7 = (c.m.r32((p_d_1).wrapping_add(3248)) as i32);
            if ((((((l_top_7).wrapping_add(2i32)) >= (3i32)) as i32)) != 0) {
                { let v_ = (l_top_7).wrapping_sub(3i32); let a_ = (p_d_1).wrapping_add(3248); c.m.w32(a_, (v_) as u32); };
                l_c_8 = (p_d_1).wrapping_add(3248);
                { let _ = F::DcChars_ClampTop(c, l_c_8); };
                F::Flash_GotoLabel(c, (p_d_1).wrapping_add(20), sym::menu_y_b__str25, ((((1i32) as i64 as u64)) as u8));
                { let v_ = ((c.m.r32(l_c_8) as i32)).wrapping_add(3i32); let a_ = ((p_d_1).wrapping_add(3248)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                break 'f;
            }
        }
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((2i32) as u32)))) != 0) {
            l_top_9 = (c.m.r32((p_d_1).wrapping_add(3248)) as i32);
            if (((((l_top_9) != (11i32)) as i32)) != 0) {
                { let v_ = (l_top_9).wrapping_add(3i32); let a_ = (p_d_1).wrapping_add(3248); c.m.w32(a_, (v_) as u32); };
                l_c_10 = (p_d_1).wrapping_add(3248);
                { let _ = F::DcChars_ClampTop(c, l_c_10); };
                F::Flash_GotoLabel(c, (p_d_1).wrapping_add(20), sym::menu_y_b__str26, ((((1i32) as i64 as u64)) as u8));
                { let v_ = ((c.m.r32(l_c_10) as i32)).wrapping_sub(1i32); let a_ = ((p_d_1).wrapping_add(3248)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcList_InputList", &PROBE_N, &[], true); }
    r_
}

pub fn DcView_SetMenuText(c: &mut Ctx, mut p_v_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcView_SetMenuText", &PROBE_N, &[(p_v_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(288) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 288);
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (3i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = 0i32; let a_ = fp; c.m.w32(a_, (v_) as u32); };
                    { let v_ = (l_i_2).wrapping_mul(32i32); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 256i32; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((l_i_2).wrapping_mul(32i32)).wrapping_add(32i32); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(20), sym::menu_y_b__str27, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                    F::menu_y_b__DcView_SetChildUv(c, p_v_1, fp.wrapping_add(20), sym::menu_y_b__str28, fp);
                    F::menu_y_b__DcView_SetChildUv(c, p_v_1, fp.wrapping_add(20), sym::menu_y_b__str29, fp);
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcView_SetMenuText", &PROBE_N, &[], true); }
    r_
}

pub fn DcView_SetStatus(c: &mut Ctx, mut p_v_1: u32, mut p_s_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcView_SetStatus", &PROBE_N, &[(p_v_1) as u32, (p_s_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(800) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 800);
    let mut l_i_3: i32 = 0;
    let mut l_k_4: i32 = 0;
    let mut l_val_5: i32 = 0;
    let r_ = 'f: {
        { let _ = F::sprintf(c, fp.wrapping_add(20), sym::menu_y_b__str30, &[((2i32) as i64 as u64), ((1i32) as i64 as u64)]); };
        {
            l_i_3 = (c.m.r32((p_s_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32);
            'l1: loop {
                if !(((((l_i_3) < (7i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let _ = F::sprintf(c, fp.wrapping_add(532), sym::menu_y_b__str31, &[(((l_i_3).wrapping_add(4i32)) as i64 as u64)]); };
                    F::menu_y_b__DcView_ShowChild(c, p_v_1, fp.wrapping_add(20), fp.wrapping_add(532), 0i32);
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        F::menu_y_b__DcUv_SetRow(c, fp, l_i_3, 128i32, 20i32);
        { let _ = F::sprintf(c, fp.wrapping_add(532), sym::menu_y_b__str32, &[((1i32) as i64 as u64)]); };
        F::menu_y_b__DcView_SetChildUv(c, p_v_1, fp.wrapping_add(20), fp.wrapping_add(532), fp);
        {
            l_i_3 = 0i32;
            'l3: loop {
                if !(((((l_i_3) < (4i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    l_val_5 = (c.m.r32((p_s_2).wrapping_add(((((1i32).wrapping_add(l_i_3)) as u32)).wrapping_mul(4))) as i32);
                    { let _ = F::sprintf(c, fp.wrapping_add(276), sym::menu_y_b__str33, &[((1i32) as i64 as u64), (((l_i_3).wrapping_add(1i32)) as i64 as u64)]); };
                    {
                        l_k_4 = 0i32;
                        'l5: loop {
                            if !(((((l_k_4) < (4i32)) as i32)) != 0) { break 'l5; }
                            'c6: {
                                { let _ = F::sprintf(c, fp.wrapping_add(532), sym::menu_y_b__str34, &[(((l_k_4).wrapping_add(1i32)) as i64 as u64)]); };
                                F::menu_y_b__DcView_ShowChild(c, p_v_1, fp.wrapping_add(276), fp.wrapping_add(532), 0i32);
                                { let _ = F::sprintf(c, fp.wrapping_add(532), sym::menu_y_b__str35, &[(((l_k_4).wrapping_add(1i32)) as i64 as u64)]); };
                                F::menu_y_b__DcView_ShowChild(c, p_v_1, fp.wrapping_add(276), fp.wrapping_add(532), 0i32);
                                if (((((((((((((l_val_5) < (0i32)) as i32)) != 0) && (((((l_val_5) < ((l_k_4).wrapping_neg())) as i32)) != 0)) as i32)) != 0) && ((((((l_k_4).wrapping_neg()) <= (0i32)) as i32)) != 0)) as i32)) != 0) {
                                    { let _ = F::sprintf(c, fp.wrapping_add(532), sym::menu_y_b__str36, &[(((l_k_4).wrapping_add(1i32)) as i64 as u64)]); };
                                    F::menu_y_b__DcView_ShowChild(c, p_v_1, fp.wrapping_add(276), fp.wrapping_add(532), 1i32);
                                } else {
                                    if (((((((((((((l_val_5) > (0i32)) as i32)) != 0) && (((((l_k_4) >= (0i32)) as i32)) != 0)) as i32)) != 0) && (((((l_k_4) < (l_val_5)) as i32)) != 0)) as i32)) != 0) {
                                        { let _ = F::sprintf(c, fp.wrapping_add(532), sym::menu_y_b__str37, &[(((l_k_4).wrapping_add(1i32)) as i64 as u64)]); };
                                        F::menu_y_b__DcView_ShowChild(c, p_v_1, fp.wrapping_add(276), fp.wrapping_add(532), 1i32);
                                    }
                                }
                            }
                            { let o_ = l_k_4; let n_ = (o_).wrapping_add(1); l_k_4 = n_; };
                        }
                    }
                    F::menu_y_b__DcUv_SetRow(c, fp, (l_i_3).wrapping_add(1i32), 128i32, 20i32);
                    { let _ = F::sprintf(c, fp.wrapping_add(532), sym::menu_y_b__str38, &[((1i32) as i64 as u64)]); };
                    F::menu_y_b__DcView_SetChildUv(c, p_v_1, fp.wrapping_add(276), fp.wrapping_add(532), fp);
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        {
            F::Flash_FindLabel(c, p_v_1, ((0i32) as u32), sym::menu_y_b__str39, fp.wrapping_add(788));
            { let a2_ = (c.m.r32((p_s_2).wrapping_add(24)) as i32); F::Flash_ClipSetTex(c, p_v_1, fp.wrapping_add(788), a2_) };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcView_SetStatus", &PROBE_N, &[], true); }
    r_
}

pub fn DcView_LightMenu(c: &mut Ctx, mut p_v_1: u32, mut p_item_2: i32, mut p_on_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcView_LightMenu", &PROBE_N, &[(p_v_1) as u32, (p_item_2) as u32, (p_on_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(272) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 272);
    let r_ = 'f: {
        { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_y_b__str40, &[(((p_item_2).wrapping_add(1i32)) as i64 as u64)]); };
        F::Flash_FindLabel(c, p_v_1, ((0i32) as u32), fp.wrapping_add(8), fp);
        if ((p_on_3) != 0) {
            F::Flash_ClipGotoLabel(c, p_v_1, fp, sym::menu_y_b__str41);
        } else {
            F::Flash_ClipGotoLabel(c, p_v_1, fp, sym::menu_y_b__str42);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcView_LightMenu", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_WrapMenu(c: &mut Ctx, mut p_cursor_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_WrapMenu", &PROBE_N, &[(p_cursor_1) as u32], false); }
    let mut l_max_2: i32 = 0;
    let r_ = 'f: {
    l_max_2 = 2i32;
    if ((((((c.m.r32(p_cursor_1) as i32)) < (0i32)) as i32)) != 0) {
        { let v_ = l_max_2; let a_ = p_cursor_1; c.m.w32(a_, (v_) as u32); };
    } else {
        if ((((((c.m.r32(p_cursor_1) as i32)) > (l_max_2)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = p_cursor_1; c.m.w32(a_, (v_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcList_WrapMenu", &PROBE_N, &[], true); }
    r_
}

pub fn DcView_HideNamePlates(c: &mut Ctx, mut p_v_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcView_HideNamePlates", &PROBE_N, &[(p_v_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        F::Flash_FindLabel(c, p_v_1, sym::menu_y_b__str43, sym::menu_y_b__str44, fp);
        { let _ = F::Flash_ClipSetFlags(c, p_v_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
        F::Flash_FindLabel(c, p_v_1, sym::menu_y_b__str45, sym::menu_y_b__str46, fp);
        { let _ = F::Flash_ClipSetFlags(c, p_v_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcView_HideNamePlates", &PROBE_N, &[], true); }
    r_
}

pub fn DcView_SetNameText(c: &mut Ctx, mut p_v_1: u32, mut p_line_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcView_SetNameText", &PROBE_N, &[(p_v_1) as u32, (p_line_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        F::Flash_FindLabel(c, p_v_1, ((0i32) as u32), sym::menu_y_b__str47, fp);
        F::TextBox_AttachLine(c, p_v_1, fp, 0i32, 0i32, p_line_2, (p_v_1).wrapping_add(1440));
        F::Flash_FindLabel(c, p_v_1, ((0i32) as u32), sym::menu_y_b__str48, fp);
        F::TextBox_AttachLine(c, p_v_1, fp, 0i32, 0i32, p_line_2, (p_v_1).wrapping_add(1580));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcView_SetNameText", &PROBE_N, &[], true); }
    r_
}

pub fn menu_y_b__DcView_SetPictureAlpha(c: &mut Ctx, mut p_v_1: u32, mut p_alpha_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_y_b__DcView_SetPictureAlpha", &PROBE_N, &[(p_v_1) as u32, (p_alpha_2).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        F::Flash_FindLabel(c, p_v_1, ((0i32) as u32), sym::menu_y_b__str49, fp);
        F::Flash_ClipSetAlpha(c, p_v_1, fp, p_alpha_2);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "menu_y_b__DcView_SetPictureAlpha", &PROBE_N, &[], true); }
    r_
}

pub fn DcView_MenuOk(c: &mut Ctx, mut p_v_1: u32, mut p_item_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcView_MenuOk", &PROBE_N, &[(p_v_1) as u32, (p_item_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(272) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 272);
    let r_ = 'f: {
        { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_y_b__str50, &[(((p_item_2).wrapping_add(1i32)) as i64 as u64)]); };
        F::Flash_FindLabel(c, p_v_1, ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, p_v_1, fp, sym::menu_y_b__str51);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcView_MenuOk", &PROBE_N, &[], true); }
    r_
}

pub fn DcRec_IsSlotEmpty(c: &mut Ctx, mut p_items_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcRec_IsSlotEmpty", &PROBE_N, &[(p_items_1) as u32, (p_slot_2) as u32], false); }
    let mut l_empty_3: i32 = 0;
    let r_ = 'f: {
    l_empty_3 = 0i32;
    { let r_ = p_slot_2; let n_ = (p_items_1).wrapping_add((((r_) as u32)).wrapping_mul(2)); p_items_1 = n_; };
    if ((((((((((((c.m.r16(p_items_1)) as i32)).wrapping_sub(1i32)) as u16)) as i32)) >= (350i32)) as i32)) != 0) {
        l_empty_3 = (((p_slot_2) != (7i32)) as i32);
    }
    break 'f l_empty_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "DcRec_IsSlotEmpty", &PROBE_N, &[], true); }
    r_
}

pub fn DcView_LightItem(c: &mut Ctx, mut p_v_1: u32, mut p_row_2: i32, mut p_on_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcView_LightItem", &PROBE_N, &[(p_v_1) as u32, (p_row_2) as u32, (p_on_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(272) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 272);
    let mut l_flash_4: u32 = 0;
    let r_ = 'f: {
        { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_y_b__str52, &[(((p_row_2).wrapping_add(1i32)) as i64 as u64)]); };
        l_flash_4 = (p_v_1).wrapping_add((((1i32) as u32)).wrapping_mul(44));
        F::Flash_FindLabel(c, l_flash_4, ((0i32) as u32), fp.wrapping_add(8), fp);
        if ((p_on_3) != 0) {
            F::Flash_ClipGotoLabel(c, l_flash_4, fp, sym::menu_y_b__str53);
        } else {
            F::Flash_ClipGotoLabel(c, l_flash_4, fp, sym::menu_y_b__str54);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcView_LightItem", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_InputMenu(c: &mut Ctx, mut p_d_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_InputMenu", &PROBE_N, &[(p_d_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_v_2: u32 = 0;
    let mut l_v_3: u32 = 0;
    let mut l_cursor_4: u32 = 0;
    let mut l_v_5: u32 = 0;
    let mut l_cursor_6: u32 = 0;
    let r_ = 'f: {
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            l_v_2 = (p_d_1).wrapping_add(20);
            { let a1_ = (c.m.r32((p_d_1).wrapping_add(3240)) as i32); F::DcView_MenuOk(c, l_v_2, a1_) };
            's1: {
                let sv7: i32 = (c.m.r32((p_d_1).wrapping_add(3240)) as i32);
                match sv7 {
                    0i32 => {
                        { let v_ = 2i32; let a_ = (p_d_1).wrapping_add(3236); c.m.w32(a_, (v_) as u32); };
                        F::Flash_GotoLabel(c, ((p_d_1).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44)), sym::menu_y_b__str55, ((((1i32) as i64 as u64)) as u8));
                        { let a1_ = (c.m.r32((p_d_1).wrapping_add(3244)) as i32); F::DcView_LightItem(c, l_v_2, a1_, 0i32) };
                        { let v_ = 0i32; let a_ = (p_d_1).wrapping_add(3244); c.m.w32(a_, (v_) as u32); };
                        'l2: while (({ let a1_ = (c.m.r32((p_d_1).wrapping_add(3244)) as i32); F::DcRec_IsSlotEmpty(c, ((p_d_1).wrapping_add(3652)).wrapping_add(28), a1_) }) != 0) {
                            { let a_ = (p_d_1).wrapping_add(3244); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        }
                        { let a1_ = (c.m.r32((p_d_1).wrapping_add(3244)) as i32); F::DcView_LightItem(c, l_v_2, a1_, 1i32) };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                        break 's1;
                    }
                    1i32 => {
                        { let s_ = F::DcChars_GetRec(c, fp.wrapping_add(28), (p_d_1).wrapping_add(3248)); let d_ = fp; rt::memcpy(c, d_, s_, 28); };
                        { let a1_ = (c.m.r32((fp).wrapping_add(24)) as i32); let a2_ = ((c.m.r16((fp).wrapping_add(20))) as i32); F::PassWin_OpenEx(c, fp, a1_, a2_) };
                        { let v_ = 4i32; let a_ = (p_d_1).wrapping_add(3236); c.m.w32(a_, (v_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                        break 's1;
                    }
                    2i32 => {
                        { let v_ = 5i32; let a_ = (p_d_1).wrapping_add(3236); c.m.w32(a_, (v_) as u32); };
                        F::Dialog_SetMsg(c, 2i32);
                        F::Dialog_Start(c, 0i32);
                        F::Dialog_SetCursor(c, 1i32);
                        F::Dialog_SetChoices(c, 1i32);
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
        } else {
            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                F::Flash_GotoLabel(c, (p_d_1).wrapping_add(20), sym::menu_y_b__str56, ((((1i32) as i64 as u64)) as u8));
                F::MsgWin_Open(c);
                { let v_ = 0i32; let a_ = (p_d_1).wrapping_add(3236); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
            } else {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                    l_v_3 = (p_d_1).wrapping_add(20);
                    { let a1_ = (c.m.r32((p_d_1).wrapping_add(3240)) as i32); F::DcView_LightMenu(c, l_v_3, a1_, 0i32) };
                    { let a_ = (p_d_1).wrapping_add(3240); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    l_cursor_4 = (p_d_1).wrapping_add(3240);
                    F::DcList_WrapMenu(c, l_cursor_4);
                    { let a1_ = (c.m.r32(l_cursor_4) as i32); F::DcView_LightMenu(c, l_v_3, a1_, 1i32) };
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                        l_v_5 = (p_d_1).wrapping_add(20);
                        { let a1_ = (c.m.r32((p_d_1).wrapping_add(3240)) as i32); F::DcView_LightMenu(c, l_v_5, a1_, 0i32) };
                        { let a_ = (p_d_1).wrapping_add(3240); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        l_cursor_6 = (p_d_1).wrapping_add(3240);
                        F::DcList_WrapMenu(c, l_cursor_6);
                        { let a1_ = (c.m.r32(l_cursor_6) as i32); F::DcView_LightMenu(c, l_v_5, a1_, 1i32) };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    }
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcList_InputMenu", &PROBE_N, &[], true); }
    r_
}

pub fn menu_y_b__DcList_Say(c: &mut Ctx, mut p_list_1: u32, mut p_line_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_y_b__DcList_Say", &PROBE_N, &[(p_list_1) as u32, (p_line_2) as u32], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32((p_list_1).wrapping_add(3208)); F::Voice_PlayWithSubtitle(c, a0_, 33688i32, p_line_2) };
    { let v_ = p_line_2; let a_ = (p_list_1).wrapping_add(3232); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_y_b__DcList_Say", &PROBE_N, &[], true); }
    r_
}

pub fn menu_y_b__DcList_SetUv(c: &mut Ctx, mut p_flash_1: u32, mut p_parent_2: u32, mut p_name_3: u32, mut p_uv_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_y_b__DcList_SetUv", &PROBE_N, &[(p_flash_1) as u32, (p_parent_2) as u32, (p_name_3) as u32, (p_uv_4) as u32], false); }
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
    if rt::probing() { crate::host::harness::probe(c, "menu_y_b__DcList_SetUv", &PROBE_N, &[], true); }
    r_
}

pub fn menu_y_b__DcList_SetCell(c: &mut Ctx, mut p_uv_1: u32, mut p_col_2: i32, mut p_row_3: i32, mut p_w_4: i32, mut p_h_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_y_b__DcList_SetCell", &PROBE_N, &[(p_uv_1) as u32, (p_col_2) as u32, (p_row_3) as u32, (p_w_4) as u32, (p_h_5) as u32], false); }
    let r_ = 'f: {
    { let v_ = (p_col_2).wrapping_mul(p_w_4); let a_ = p_uv_1; c.m.w32(a_, (v_) as u32); };
    { let v_ = (p_row_3).wrapping_mul(p_h_5); let a_ = (p_uv_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((c.m.r32(p_uv_1) as i32)).wrapping_add(p_w_4); let a_ = (p_uv_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((c.m.r32((p_uv_1).wrapping_add(4)) as i32)).wrapping_add(p_h_5); let a_ = (p_uv_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_y_b__DcList_SetCell", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_WrapItemCursor(c: &mut Ctx, mut p_cursor_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_WrapItemCursor", &PROBE_N, &[(p_cursor_1) as u32], false); }
    let mut l_max_2: i32 = 0;
    let r_ = 'f: {
    l_max_2 = (8i32).wrapping_sub(1i32);
    if ((((((c.m.r32(p_cursor_1) as i32)) < (0i32)) as i32)) != 0) {
        { let v_ = l_max_2; let a_ = p_cursor_1; c.m.w32(a_, (v_) as u32); };
    } else {
        if ((((((c.m.r32(p_cursor_1) as i32)) > (l_max_2)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = p_cursor_1; c.m.w32(a_, (v_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcList_WrapItemCursor", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_IsRowSkipped(c: &mut Ctx, mut p_items_1: u32, mut p_row_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_IsRowSkipped", &PROBE_N, &[(p_items_1) as u32, (p_row_2) as u32], false); }
    let mut l_ret_3: i32 = 0;
    let r_ = 'f: {
    l_ret_3 = 0i32;
    { let r_ = p_row_2; let n_ = (p_items_1).wrapping_add((((r_) as u32)).wrapping_mul(2)); p_items_1 = n_; };
    if ((((((((((((c.m.r16(p_items_1)) as i32)).wrapping_sub(1i32)) as u16)) as i32)) >= (350i32)) as i32)) != 0) {
        l_ret_3 = (((p_row_2) != ((8i32).wrapping_sub(1i32))) as i32);
    }
    break 'f l_ret_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "DcList_IsRowSkipped", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_DrawNames(c: &mut Ctx, mut p_view_1: u32, mut p_chara_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_DrawNames", &PROBE_N, &[(p_view_1) as u32, (p_chara_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_flash_3: u32 = 0;
    let r_ = 'f: {
        l_flash_3 = (p_view_1).wrapping_add((((1i32) as u32)).wrapping_mul(44));
        F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), sym::menu_y_b__str57, fp);
        F::TextBox_AttachLine(c, l_flash_3, fp, 0i32, 0i32, p_chara_2, (p_view_1).wrapping_add(1720));
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::menu_y_b__DcList_SetUv(c, l_flash_3, ((0i32) as u32), sym::menu_y_b__str58, fp.wrapping_add(8));
        F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), sym::menu_y_b__str59, fp);
        F::TextBox_AttachLine(c, l_flash_3, fp, 0i32, 0i32, p_chara_2, (p_view_1).wrapping_add(1860));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcList_DrawNames", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_LightItemRow(c: &mut Ctx, mut p_view_1: u32, mut p_row_2: i32, mut p_on_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_LightItemRow", &PROBE_N, &[(p_view_1) as u32, (p_row_2) as u32, (p_on_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(272) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 272);
    let mut l_flash_4: u32 = 0;
    let r_ = 'f: {
        { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_y_b__str60, &[(((p_row_2).wrapping_add(1i32)) as i64 as u64)]); };
        l_flash_4 = (p_view_1).wrapping_add((((1i32) as u32)).wrapping_mul(44));
        F::Flash_FindLabel(c, l_flash_4, ((0i32) as u32), fp.wrapping_add(8), fp);
        if ((p_on_3) != 0) {
            F::Flash_ClipGotoLabel(c, l_flash_4, fp, sym::menu_y_b__str61);
        } else {
            F::Flash_ClipGotoLabel(c, l_flash_4, fp, sym::menu_y_b__str62);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcList_LightItemRow", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_DrawItems(c: &mut Ctx, mut p_list_1: u32, mut p_cursor_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_DrawItems", &PROBE_N, &[(p_list_1) as u32, (p_cursor_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(288) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 288);
    let mut l_flash_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_item_5: u16 = 0;
    let mut l_slots_6: i32 = 0;
    let r_ = 'f: {
        l_flash_3 = ((p_list_1).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44));
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < (8i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_item_5 = c.m.r16((((p_list_1).wrapping_add(3652)).wrapping_add(28)).wrapping_add((((l_i_4) as u32)).wrapping_mul(2)));
                    l_slots_6 = (({ let a_ = ((c.m.r32((p_list_1).wrapping_add(3712))).wrapping_add(((((((l_item_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(40))).wrapping_add(3); c.m.r8(a_) }) as i32);
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_y_b__str63, &[(((l_i_4).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_y_b__str64, fp);
                    if (((((((l_item_5) as i32)) != (0i32)) as i32)) != 0) {
                        F::TextBox_AttachLine(c, l_flash_3, fp, 0i32, 0i32, (((l_item_5) as i32)).wrapping_sub(1i32), (((p_list_1).wrapping_add(20)).wrapping_add(2000)).wrapping_add((((l_i_4) as u32)).wrapping_mul(140)));
                    }
                    F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_y_b__str65, fp);
                    if (((((l_i_4) == (p_cursor_2)) as i32)) != 0) {
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                    } else {
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    }
                    { let v_ = 64i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_y_b__str66, fp);
                    F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                    if (((((((l_item_5) as i32)) != (0i32)) as i32)) != 0) {
                        if (((((l_slots_6) != (0i32)) as i32)) != 0) {
                            { let a1_ = ((({ let a_ = ((c.m.r32((p_list_1).wrapping_add(3712))).wrapping_add(((((((l_item_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(40))).wrapping_add(3); c.m.r8(a_) }) as i32)).wrapping_sub(1i32); let a2_ = { let a1_ = c.m.r32((p_list_1).wrapping_add(3712)); F::ItemTbl_GetClass(c, (((l_item_5) as i32)).wrapping_sub(1i32), a1_) }; F::menu_y_b__DcList_SetCell(c, fp.wrapping_add(8), a1_, a2_, 32i32, 42i32) };
                            F::menu_y_b__DcList_SetUv(c, l_flash_3, fp.wrapping_add(28), sym::menu_y_b__str67, fp.wrapping_add(8));
                            F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_y_b__str68, fp);
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                        } else {
                            F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_y_b__str69, fp);
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                        }
                        { let v_ = ((({ let a_ = (c.m.r32((p_list_1).wrapping_add(3712))).wrapping_add(((((((l_item_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(40)); c.m.r8(a_) }) as i32)).wrapping_shl((6i32) as u32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        F::menu_y_b__DcList_SetUv(c, l_flash_3, fp.wrapping_add(28), sym::menu_y_b__str70, fp.wrapping_add(8));
                        F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_y_b__str71, fp);
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                        F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_y_b__str72, fp);
                        F::Flash_ClipSetTex(c, l_flash_3, fp, 1i32);
                        F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_y_b__str73, fp);
                        F::Flash_ClipSetTex(c, l_flash_3, fp, 0i32);
                    } else {
                        F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_y_b__str74, fp);
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                        F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_y_b__str75, fp);
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                        F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_y_b__str76, fp);
                        F::Flash_ClipSetTex(c, l_flash_3, fp, 5i32);
                        F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_y_b__str77, fp);
                        F::Flash_ClipSetTex(c, l_flash_3, fp, 3i32);
                    }
                    if (((((l_i_4) == ((8i32).wrapping_sub(1i32))) as i32)) != 0) {
                        F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_y_b__str78, fp);
                        F::Flash_ClipSetTex(c, l_flash_3, fp, 4i32);
                        F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_y_b__str79, fp);
                        F::Flash_ClipSetTex(c, l_flash_3, fp, 3i32);
                    }
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcList_DrawItems", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_DrawLimitNums(c: &mut Ctx, mut p_list_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_DrawLimitNums", &PROBE_N, &[(p_list_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_flash_2: u32 = 0;
    let r_ = 'f: {
        l_flash_2 = ((p_list_1).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44));
        F::menu_y_b__DcList_SetCell(c, fp, 2i32, 2i32, 64i32, 64i32);
        F::menu_y_b__DcList_SetUv(c, l_flash_2, ((0i32) as u32), sym::menu_y_b__str80, fp);
        F::menu_y_b__DcList_SetCell(c, fp, 2i32, 2i32, 32i32, 32i32);
        F::menu_y_b__DcList_SetUv(c, l_flash_2, ((0i32) as u32), sym::menu_y_b__str81, fp);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcList_DrawLimitNums", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_GetCursorItem(c: &mut Ctx, mut p_list_1: u32) -> u16 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_GetCursorItem", &PROBE_N, &[(p_list_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = (((p_list_1).wrapping_add(3652)).wrapping_add(28)).wrapping_add(((((c.m.r32((p_list_1).wrapping_add(3244)) as i32)) as u32)).wrapping_mul(2)); c.m.r16(a_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "DcList_GetCursorItem", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_FlashItemRow(c: &mut Ctx, mut p_list_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_FlashItemRow", &PROBE_N, &[(p_list_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(272) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 272);
    let mut l_flash_2: u32 = 0;
    let r_ = 'f: {
        { let _ = { let a2_ = ((c.m.r32((p_list_1).wrapping_add(3244)) as i32)).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_y_b__str82, &[((a2_) as i64 as u64)]) }; };
        l_flash_2 = ((p_list_1).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44));
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_y_b__str83);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcList_FlashItemRow", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_DrawItemPage(c: &mut Ctx, mut p_list_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_DrawItemPage", &PROBE_N, &[(p_list_1) as u32], false); }
    let r_ = 'f: {
    { let a1_ = (c.m.r32((((p_list_1).wrapping_add(3652)).wrapping_add(28)).wrapping_add(24)) as i32); F::DcList_DrawNames(c, (p_list_1).wrapping_add(20), a1_) };
    { let a1_ = (c.m.r32((p_list_1).wrapping_add(3244)) as i32); F::DcList_DrawItems(c, p_list_1, a1_) };
    F::DcList_DrawLimitNums(c, p_list_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcList_DrawItemPage", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_InputItems(c: &mut Ctx, mut p_list_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_InputItems", &PROBE_N, &[(p_list_1) as u32], false); }
    let r_ = 'f: {
    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
        F::DcList_FlashItemRow(c, p_list_1);
        if (((((((F::DcList_GetCursorItem(c, p_list_1)) as i32)) != (0i32)) as i32)) != 0) {
            F::ItemHelp_Open(c);
            { let v_ = 3i32; let a_ = (p_list_1).wrapping_add(3236); c.m.w32(a_, (v_) as u32); };
            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
        } else {
            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
        }
    } else {
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
            { let v_ = 1i32; let a_ = (p_list_1).wrapping_add(3236); c.m.w32(a_, (v_) as u32); };
            F::Flash_GotoLabel(c, ((p_list_1).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44)), sym::menu_y_b__str84, ((((1i32) as i64 as u64)) as u8));
            { let a1_ = (c.m.r32((p_list_1).wrapping_add(3244)) as i32); F::DcList_LightItemRow(c, (p_list_1).wrapping_add(20), a1_, 0i32) };
            { let v_ = 0i32; let a_ = (p_list_1).wrapping_add(3244); c.m.w32(a_, (v_) as u32); };
            { let a1_ = (c.m.r32((p_list_1).wrapping_add(3244)) as i32); F::DcList_LightItemRow(c, (p_list_1).wrapping_add(20), a1_, 1i32) };
            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
        } else {
            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                { let a1_ = (c.m.r32((p_list_1).wrapping_add(3244)) as i32); F::DcList_LightItemRow(c, (p_list_1).wrapping_add(20), a1_, 0i32) };
                'l1: loop {
                    'c2: {
                        { let a_ = (p_list_1).wrapping_add(3244); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                        F::DcList_WrapItemCursor(c, (p_list_1).wrapping_add(3244));
                    }
                    if !(({ let a1_ = (c.m.r32((p_list_1).wrapping_add(3244)) as i32); F::DcList_IsRowSkipped(c, ((p_list_1).wrapping_add(3652)).wrapping_add(28), a1_) }) != 0) { break 'l1; }
                }
                { let a1_ = (c.m.r32((p_list_1).wrapping_add(3244)) as i32); F::DcList_LightItemRow(c, (p_list_1).wrapping_add(20), a1_, 1i32) };
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
            } else {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                    { let a1_ = (c.m.r32((p_list_1).wrapping_add(3244)) as i32); F::DcList_LightItemRow(c, (p_list_1).wrapping_add(20), a1_, 0i32) };
                    'l3: loop {
                        'c4: {
                            { let a_ = (p_list_1).wrapping_add(3244); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            F::DcList_WrapItemCursor(c, (p_list_1).wrapping_add(3244));
                        }
                        if !(({ let a1_ = (c.m.r32((p_list_1).wrapping_add(3244)) as i32); F::DcList_IsRowSkipped(c, ((p_list_1).wrapping_add(3652)).wrapping_add(28), a1_) }) != 0) { break 'l3; }
                    }
                    { let a1_ = (c.m.r32((p_list_1).wrapping_add(3244)) as i32); F::DcList_LightItemRow(c, (p_list_1).wrapping_add(20), a1_, 1i32) };
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcList_InputItems", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_DrawItemHelp(c: &mut Ctx, mut p_list_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_DrawItemHelp", &PROBE_N, &[(p_list_1) as u32], false); }
    let mut l_item_2: i32 = 0;
    let r_ = 'f: {
    l_item_2 = (((F::DcList_GetCursorItem(c, p_list_1)) as i32)).wrapping_sub(1i32);
    if (((((l_item_2) < (0i32)) as i32)) != 0) {
        l_item_2 = 0i32;
    }
    F::ItemHelp_Draw(c, l_item_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcList_DrawItemHelp", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_InputItemHelp(c: &mut Ctx, mut p_list_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_InputItemHelp", &PROBE_N, &[(p_list_1) as u32], false); }
    let r_ = 'f: {
    if (((((((F::DcList_GetCursorItem(c, p_list_1)) as i32)) != (0i32)) as i32)) != 0) {
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            { let v_ = 2i32; let a_ = (p_list_1).wrapping_add(3236); c.m.w32(a_, (v_) as u32); };
            F::ItemHelp_Close(c);
            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
        } else {
            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                { let v_ = 2i32; let a_ = (p_list_1).wrapping_add(3236); c.m.w32(a_, (v_) as u32); };
                F::ItemHelp_Close(c);
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcList_InputItemHelp", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_DrawPassword(c: &mut Ctx, mut p_list_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_DrawPassword", &PROBE_N, &[(p_list_1) as u32], false); }
    let r_ = 'f: {
    F::PassWin_Draw(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcList_DrawPassword", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_InputPassword(c: &mut Ctx, mut p_list_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_InputPassword", &PROBE_N, &[(p_list_1) as u32], false); }
    let r_ = 'f: {
    if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) || ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0)) as i32)) != 0) {
        F::PassWin_Close(c);
        { let v_ = 1i32; let a_ = (p_list_1).wrapping_add(3236); c.m.w32(a_, (v_) as u32); };
        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcList_InputPassword", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_DeleteRec(c: &mut Ctx, mut p_list_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_DeleteRec", &PROBE_N, &[(p_list_1) as u32], false); }
    let mut l_idx_2: i32 = 0;
    let r_ = 'f: {
    l_idx_2 = F::DcChars_GetCursor(c, (p_list_1).wrapping_add(3248));
    { let v_ = { let v_ = (1i32).wrapping_neg(); let a_ = ((((p_list_1).wrapping_add(3248)).wrapping_add(12)).wrapping_add((((F::DcChars_GetCursor(c, (p_list_1).wrapping_add(3248))) as u32)).wrapping_mul(28))).wrapping_add(24); c.m.w32(a_, (v_) as u32); v_ }; let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(11584)).wrapping_add((((l_idx_2) as u32)).wrapping_mul(28))).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
    { let r_ = 1i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcList_DeleteRec", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_DrawDialog(c: &mut Ctx, mut p_list_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_DrawDialog", &PROBE_N, &[(p_list_1) as u32], false); }
    let r_ = 'f: {
    F::Dialog_Draw(c, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcList_DrawDialog", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_InputDialog(c: &mut Ctx, mut p_list_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_InputDialog", &PROBE_N, &[(p_list_1) as u32], false); }
    let mut l_result_2: i32 = 0;
    let r_ = 'f: {
    if ((F::Dialog_IsClosed(c)) != 0) {
        { let v_ = (c.m.r32((p_list_1).wrapping_add(3720)) as i32); let a_ = (p_list_1).wrapping_add(3236); c.m.w32(a_, (v_) as u32); };
        if ((((((c.m.r32((p_list_1).wrapping_add(3236)) as i32)) == (6i32)) as i32)) != 0) {
            F::Flash_GotoLabel(c, ((p_list_1).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), sym::menu_y_b__str85, ((((1i32) as i64 as u64)) as u8));
            F::menu_y_b__DcList_Say(c, p_list_1, 27i32);
            F::MsgWin_Open(c);
        }
    }
    l_result_2 = F::Dialog_Input(c, 0i32);
    if (((((l_result_2) == (1i32)) as i32)) != 0) {
        F::DcList_DeleteRec(c, p_list_1);
        { let v_ = 6i32; let a_ = (p_list_1).wrapping_add(3720); c.m.w32(a_, (v_) as u32); };
        F::Dialog_Start(c, 1i32);
        F::Dialog_SetChoices(c, 0i32);
    } else {
        if (((((l_result_2) == ((2i32).wrapping_neg())) as i32)) != 0) {
            { let v_ = 1i32; let a_ = (p_list_1).wrapping_add(3720); c.m.w32(a_, (v_) as u32); };
            F::Dialog_Start(c, 1i32);
            F::Dialog_SetChoices(c, 0i32);
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcList_InputDialog", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_InputDeleted(c: &mut Ctx, mut p_list_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_InputDeleted", &PROBE_N, &[(p_list_1) as u32], false); }
    let r_ = 'f: {
    if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) || (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (p_list_1).wrapping_add(3236); c.m.w32(a_, (v_) as u32); };
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
        }
        F::menu_y_b__DcList_Say(c, p_list_1, 26i32);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcList_InputDeleted", &PROBE_N, &[], true); }
    r_
}

pub fn menu_y_b__DcList_RequestFace(c: &mut Ctx, mut p_list_1: u32, mut p_status_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_y_b__DcList_RequestFace", &PROBE_N, &[(p_list_1) as u32, (p_status_2) as u32], false); }
    let r_ = 'f: {
    F::File_CancelRequests(c);
    { let _ = { let a0_ = ((c.m.r32(((p_status_2).wrapping_add(28)).wrapping_add(24)) as i32)).wrapping_add(761i32); let a1_ = c.m.r32((p_list_1).wrapping_add(8)); let _ = 92160i32; F::File_Request(c, a0_, a1_) }; };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_y_b__DcList_RequestFace", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_UpdateFace(c: &mut Ctx, mut p_list_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_UpdateFace", &PROBE_N, &[(p_list_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_flags_2: i32 = 0;
    let r_ = 'f: {
        if (((((c.m.r32((p_list_1).wrapping_add(3716)) as i32)) & (1i32))) != 0) {
            { let s_ = F::DcChars_GetRec(c, fp.wrapping_add(4), (p_list_1).wrapping_add(3248)); let d_ = ((p_list_1).wrapping_add(3652)).wrapping_add(28); rt::memcpy(c, d_, s_, 28); };
            F::menu_y_b__DcList_RequestFace(c, p_list_1, (p_list_1).wrapping_add(3652));
            l_flags_2 = (c.m.r32((p_list_1).wrapping_add(3716)) as i32);
            { let v_ = F32(0x00000000); let a_ = (p_list_1).wrapping_add(3724); c.m.w32(a_, (v_).0); };
            { let v_ = ((((l_flags_2) ^ (1i32))) | (2i32)); let a_ = (p_list_1).wrapping_add(3716); c.m.w32(a_, (v_) as u32); };
            if (((((c.m.r32((p_list_1).wrapping_add(3716)) as i32)) & (64i32))) != 0) {
                { let r_ = 64i32; let a_ = (p_list_1).wrapping_add(3716); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
            }
        } else {
            if (((((c.m.r32((p_list_1).wrapping_add(3716)) as i32)) & (2i32))) != 0) {
                if ((F::File_UpdateRequests(c)) != 0) {
                    { let v_ = ((0i32) as u32); let a_ = fp; c.m.w32(a_, v_); };
                    { let _ = { let a0_ = c.m.r32((p_list_1).wrapping_add(8)); let a1_ = c.m.r32((p_list_1).wrapping_add(12)); F::Sprite_Unpack(c, a0_, a1_, ((0i32) as u32)) }; };
                    { let v_ = c.m.r32((p_list_1).wrapping_add(12)); let a_ = fp; c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
                    { let v_ = { let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }; let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = (((((c.m.r32((p_list_1).wrapping_add(3716)) as i32)) ^ (2i32))) | (64i32)); let a_ = (p_list_1).wrapping_add(3716); c.m.w32(a_, (v_) as u32); };
                }
            }
        }
        if (((((c.m.r32((p_list_1).wrapping_add(3716)) as i32)) & (64i32))) != 0) {
            if (((rt::flt(F32(c.m.r32((p_list_1).wrapping_add(3724))), F32(0x3f800000)) as i32)) != 0) {
                { let r_ = F32(0x3d4ccccc); let a_ = (p_list_1).wrapping_add(3724); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcList_UpdateFace", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_Update(c: &mut Ctx, mut p_list_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_Update", &PROBE_N, &[(p_list_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                F::Flash_Advance(c, ((p_list_1).wrapping_add(20)).wrapping_add((((l_i_2) as u32)).wrapping_mul(44)));
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcList_Update", &PROBE_N, &[], true); }
    r_
}

pub fn menu_y_b__DcList_Section(c: &mut Ctx, mut p_list_1: u32, mut p_n_2: i32, mut p_host_3: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_y_b__DcList_Section", &PROBE_N, &[(p_list_1) as u32, (p_n_2) as u32, (p_host_3) as u32], false); }
    let r_ = 'f: {
    break 'f (c.m.r32((p_list_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_list_1).wrapping_add(4))).wrapping_add((((p_n_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_y_b__DcList_Section", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_Init(c: &mut Ctx, mut p_list_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_Init", &PROBE_N, &[(p_list_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_2: i32 = 0;
    let mut l_j_3: i32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = (c.m.r32(sym::gMenuArc8)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc8)).wrapping_add(((((c.m.r32((p_list_1).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = p_list_1; c.m.w32(a_, v_); };
        { let v_ = { let a0_ = c.m.r32(p_list_1); F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (p_list_1).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = F::Heap_Alloc(c, 92160i32, ((64i32) as u32), 0i32, 2i32); let a_ = (p_list_1).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = F::Heap_Alloc(c, 133120i32, ((32i32) as u32), 0i32, 2i32); let a_ = (p_list_1).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = F::menu_y_b__DcList_Section(c, p_list_1, 1i32, sym::menu_y_b__str86); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = c.m.r32(fp); let a_ = ((p_list_1).wrapping_add(20)).wrapping_add(88); c.m.w32(a_, v_); };
        { let _ = { let a0_ = (F::DcChars_GetCursor(c, (p_list_1).wrapping_add(3248))).wrapping_add(761i32); let a1_ = c.m.r32((p_list_1).wrapping_add(8)); F::File_LoadSync(c, a0_, a1_, 92160i32) }; };
        { let _ = { let a0_ = c.m.r32((p_list_1).wrapping_add(8)); let a1_ = c.m.r32((p_list_1).wrapping_add(12)); F::Sprite_Unpack(c, a0_, a1_, ((0i32) as u32)) }; };
        { let v_ = F::menu_y_b__DcList_Section(c, p_list_1, 19i32, sym::menu_y_b__str87); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::menu_y_b__DcList_Section(c, p_list_1, 20i32, sym::menu_y_b__str88); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::menu_y_b__DcList_Section(c, p_list_1, 3i32, sym::menu_y_b__str89); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((7i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((8i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((9i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((10i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((11i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((12i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((13i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((14i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((32i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((15i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((33i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((16i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((34i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::menu_y_b__DcList_Section(c, p_list_1, 4i32, sym::menu_y_b__str90); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::menu_y_b__DcList_Section(c, p_list_1, 2i32, sym::menu_y_b__str91); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((35i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((36i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((37i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((30i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((31i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a1_ = F::menu_y_b__DcList_Section(c, p_list_1, 5i32, sym::menu_y_b__str92); F::Flash_Create(c, ((p_list_1).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), a1_, ((p_list_1).wrapping_add(20)).wrapping_add(92)) };
        F::Flash_Play(c, ((p_list_1).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), 1i32);
        { let v_ = F::menu_y_b__DcList_Section(c, p_list_1, 7i32, sym::menu_y_b__str93); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(244)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(244)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(244)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(244)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(244)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(244)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(244)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((7i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(244)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((8i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(244)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((9i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(244)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((10i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(244)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::menu_y_b__DcList_Section(c, p_list_1, 8i32, sym::menu_y_b__str94); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(244)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::menu_y_b__DcList_Section(c, p_list_1, 6i32, sym::menu_y_b__str95); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(244)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(244)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(244)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(244)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (((p_list_1).wrapping_add(20)).wrapping_add(244)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a1_ = F::menu_y_b__DcList_Section(c, p_list_1, 9i32, sym::menu_y_b__str96); F::Flash_Create(c, ((p_list_1).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44)), a1_, ((p_list_1).wrapping_add(20)).wrapping_add(244)) };
        F::Flash_Play(c, ((p_list_1).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44)), 1i32);
        { let v_ = F::menu_y_b__DcList_Section(c, p_list_1, 17i32, sym::menu_y_b__str97); let a_ = (p_list_1).wrapping_add(3212); c.m.w32(a_, v_); };
        { let v_ = F::menu_y_b__DcList_Section(c, p_list_1, 18i32, sym::menu_y_b__str98); let a_ = (p_list_1).wrapping_add(3216); c.m.w32(a_, v_); };
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a1_ = c.m.r32((p_list_1).wrapping_add(3212)); F::TextBox_Init(c, (((p_list_1).wrapping_add(20)).wrapping_add(320)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)), a1_, ((2i32) as u32)) };
                    F::TextBox_SetRect(c, (((p_list_1).wrapping_add(20)).wrapping_add(320)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)), 175i32, 511i32, 106i32, 313i32);
                    F::TextBox_SetUnk80(c, (((p_list_1).wrapping_add(20)).wrapping_add(320)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)), 1i32);
                    { let a1_ = c.m.r32((p_list_1).wrapping_add(3216)); F::TextBox_Init(c, (((p_list_1).wrapping_add(20)).wrapping_add(880)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)), a1_, ((4i32) as u32)) };
                    F::TextBox_SetRect(c, (((p_list_1).wrapping_add(20)).wrapping_add(880)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)), 175i32, 511i32, 106i32, 313i32);
                    F::TextBox_SetUnk80(c, (((p_list_1).wrapping_add(20)).wrapping_add(880)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)), 1i32);
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let a1_ = c.m.r32((p_list_1).wrapping_add(3212)); F::TextBox_Init(c, ((p_list_1).wrapping_add(20)).wrapping_add(1440), a1_, ((1i32) as u32)) };
        F::TextBox_SetUnk80(c, ((p_list_1).wrapping_add(20)).wrapping_add(1440), 1i32);
        { let a1_ = c.m.r32((p_list_1).wrapping_add(3216)); F::TextBox_Init(c, ((p_list_1).wrapping_add(20)).wrapping_add(1580), a1_, ((3i32) as u32)) };
        F::TextBox_SetUnk80(c, ((p_list_1).wrapping_add(20)).wrapping_add(1580), 1i32);
        { let a1_ = c.m.r32((p_list_1).wrapping_add(3212)); F::TextBox_Init(c, ((p_list_1).wrapping_add(20)).wrapping_add(1720), a1_, ((1i32) as u32)) };
        F::TextBox_SetUnk80(c, ((p_list_1).wrapping_add(20)).wrapping_add(1720), 1i32);
        { let a1_ = c.m.r32((p_list_1).wrapping_add(3216)); F::TextBox_Init(c, ((p_list_1).wrapping_add(20)).wrapping_add(1860), a1_, ((3i32) as u32)) };
        F::TextBox_SetUnk80(c, ((p_list_1).wrapping_add(20)).wrapping_add(1860), 1i32);
        { let v_ = F::menu_y_b__DcList_Section(c, p_list_1, 21i32, sym::menu_y_b__str99); let a_ = (p_list_1).wrapping_add(3220); c.m.w32(a_, v_); };
        {
            l_j_3 = 0i32;
            'l3: loop {
                if !(((((l_j_3) < (8i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let a1_ = c.m.r32((p_list_1).wrapping_add(3220)); F::TextBox_Init(c, (((p_list_1).wrapping_add(20)).wrapping_add(2000)).wrapping_add((((l_j_3) as u32)).wrapping_mul(140)), a1_, ((2i32) as u32)) };
                    F::TextBox_SetUnk80(c, (((p_list_1).wrapping_add(20)).wrapping_add(2000)).wrapping_add((((l_j_3) as u32)).wrapping_mul(140)), 1i32);
                }
                { let o_ = l_j_3; let n_ = (o_).wrapping_add(1); l_j_3 = n_; };
            }
        }
        { let v_ = F::menu_y_b__DcList_Section(c, p_list_1, 13i32, sym::menu_y_b__str100); let a_ = (p_list_1).wrapping_add(3200); c.m.w32(a_, v_); };
        { let a0_ = F::menu_y_b__DcList_Section(c, p_list_1, 11i32, sym::menu_y_b__str101); let a1_ = c.m.r32((p_list_1).wrapping_add(3200)); let _ = (((p_list_1).wrapping_add(3140)) as i32); F::MsgWin_Init(c, a0_, a1_, 0i32) };
        F::MsgWin_Open(c);
        { let v_ = (1i32).wrapping_neg(); let a_ = (p_list_1).wrapping_add(3232); c.m.w32(a_, (v_) as u32); };
        { let v_ = F::menu_y_b__DcList_Section(c, p_list_1, 22i32, sym::menu_y_b__str102); let a_ = (p_list_1).wrapping_add(3204); c.m.w32(a_, v_); };
        { let a0_ = F::menu_y_b__DcList_Section(c, p_list_1, 12i32, sym::menu_y_b__str103); let a1_ = c.m.r32((p_list_1).wrapping_add(3204)); F::Dialog_Init(c, a0_, a1_, 0i32) };
        { let a0_ = F::menu_y_b__DcList_Section(c, p_list_1, 10i32, sym::menu_y_b__str104); F::PassWin_Init(c, a0_) };
        { let a0_ = F::menu_y_b__DcList_Section(c, p_list_1, 16i32, sym::menu_y_b__str105); F::ItemHelp_Init(c, a0_) };
        { let v_ = F::menu_y_b__DcList_Section(c, p_list_1, 15i32, sym::menu_y_b__str106); let a_ = (p_list_1).wrapping_add(3712); c.m.w32(a_, v_); };
        { let v_ = F::menu_y_b__DcList_Section(c, p_list_1, 14i32, sym::menu_y_b__str107); let a_ = (p_list_1).wrapping_add(3208); c.m.w32(a_, v_); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcList_Init", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_Draw(c: &mut Ctx, mut p_list_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_Draw", &PROBE_N, &[(p_list_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_view_2: u32 = 0;
    let r_ = 'f: {
        if ((((!(((((c.m.r32((p_list_1).wrapping_add(3716)) as i32)) & (4i32))) != 0)) as i32)) != 0) {
            F::Flash_GotoLabel(c, ((p_list_1).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), sym::menu_y_b__str108, ((((1i32) as i64 as u64)) as u8));
            { let r_ = 4i32; let a_ = (p_list_1).wrapping_add(3716); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        l_view_2 = (p_list_1).wrapping_add(20);
        if ((((!(((((c.m.r32((p_list_1).wrapping_add(3716)) as i32)) & (8i32))) != 0)) as i32)) != 0) {
            if ((((c.m.r32((((p_list_1).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8))) & (((2i32) as u32)))) != 0) {
                F::DcView_LightRow(c, l_view_2, (p_list_1).wrapping_add(3248), 1i32);
                { let a1_ = (c.m.r32((p_list_1).wrapping_add(3240)) as i32); F::DcView_LightMenu(c, l_view_2, a1_, 1i32) };
                { let a1_ = (c.m.r32((p_list_1).wrapping_add(3244)) as i32); F::DcList_LightItemRow(c, l_view_2, a1_, 1i32) };
                F::menu_y_b__DcList_Say(c, p_list_1, 26i32);
                { let r_ = 8i32; let a_ = (p_list_1).wrapping_add(3716); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
        F::DcList_DrawList(c, p_list_1);
        F::Flash_FindLabel(c, (l_view_2).wrapping_add((((0i32) as u32)).wrapping_mul(44)), ((0i32) as u32), sym::menu_y_b__str109, fp);
        { let a2_ = F32(c.m.r32((p_list_1).wrapping_add(3724))); F::Flash_ClipSetAlpha(c, (l_view_2).wrapping_add((((0i32) as u32)).wrapping_mul(44)), fp, a2_) };
        F::DcView_HideNamePlates(c, l_view_2);
        F::DcView_SetMenuText(c, l_view_2);
        F::DcView_SetStatus(c, l_view_2, (p_list_1).wrapping_add(3652));
        { let a1_ = (c.m.r32((((p_list_1).wrapping_add(3652)).wrapping_add(28)).wrapping_add(24)) as i32); F::DcView_SetNameText(c, l_view_2, a1_) };
        F::DcList_DrawItemPage(c, p_list_1);
        F::DcList_UpdateGuide(c, p_list_1);
        F::DcList_ScrollCloud(c, p_list_1);
        { let a0_ = c.m.r32(((p_list_1).wrapping_add(20)).wrapping_add(88)); F::Sprite_DrawPicture(c, a0_, 0i32, 0i32, 128i32) };
        F::Flash_Draw(c, (l_view_2).wrapping_add((((0i32) as u32)).wrapping_mul(44)));
        { let a2_ = (c.m.r32((p_list_1).wrapping_add(3232)) as i32); F::MsgWin_Draw(c, 0i32, 0i32, a2_) };
        F::Flash_Draw(c, ((p_list_1).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44)));
        F::DcList_DrawDialog(c, p_list_1);
        F::DcList_DrawPassword(c, p_list_1);
        F::DcList_DrawItemHelp(c, p_list_1);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcList_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_Term(c: &mut Ctx, mut p_list_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_Term", &PROBE_N, &[(p_list_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                F::Flash_Destroy(c, ((p_list_1).wrapping_add(20)).wrapping_add((((l_i_2) as u32)).wrapping_mul(44)));
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    if (((((c.m.r32((p_list_1).wrapping_add(4))) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32((p_list_1).wrapping_add(4)); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (p_list_1).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32((p_list_1).wrapping_add(12))) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32((p_list_1).wrapping_add(12)); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (p_list_1).wrapping_add(12); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32((p_list_1).wrapping_add(8))) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32((p_list_1).wrapping_add(8)); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (p_list_1).wrapping_add(8); c.m.w32(a_, v_); };
    }
    F::PassWin_Term(c);
    F::Dialog_Term(c);
    F::MsgWin_Term(c);
    F::ItemHelp_Term(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcList_Term", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_Input(c: &mut Ctx, mut p_list_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_Input", &PROBE_N, &[(p_list_1) as u32], false); }
    let r_ = 'f: {
    if ((((!((((c.m.r32((((p_list_1).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8))) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f;
    }
    if (((((c.m.r32((p_list_1).wrapping_add(3716)) as i32)) & (16i32))) != 0) {
        break 'f;
    }
    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
        break 'f;
    }
    's1: {
        let sv2: i32 = (c.m.r32((p_list_1).wrapping_add(3236)) as i32);
        match sv2 {
            0i32 => {
                F::DcList_InputList(c, p_list_1);
                break 's1;
            }
            1i32 => {
                F::DcList_InputMenu(c, p_list_1);
                break 's1;
            }
            2i32 => {
                F::DcList_InputItems(c, p_list_1);
                break 's1;
            }
            3i32 => {
                F::DcList_InputItemHelp(c, p_list_1);
                break 's1;
            }
            4i32 => {
                F::DcList_InputPassword(c, p_list_1);
                break 's1;
            }
            5i32 => {
                F::DcList_InputDialog(c, p_list_1);
                break 's1;
            }
            6i32 => {
                F::DcList_InputDeleted(c, p_list_1);
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcList_Input", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_CheckLeave(c: &mut Ctx, mut p_list_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_CheckLeave", &PROBE_N, &[(p_list_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32((p_list_1).wrapping_add(3716)) as i32)) & (16i32))) != 0) {
        if ((((!(((((c.m.r32((p_list_1).wrapping_add(3716)) as i32)) & (32i32))) != 0)) as i32)) != 0) {
            F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
            { let r_ = 32i32; let a_ = (p_list_1).wrapping_add(3716); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if ((F::ColorFade_IsFadingOut(c)) != 0) {
            F::Voice_FadeOutStep(c, 0i32);
            F::Bgm_FadeOutStep(c);
        } else {
            break 'f 1i32;
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "DcList_CheckLeave", &PROBE_N, &[], true); }
    r_
}

pub fn DcList_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcList_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(3728) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 3728);
    let r_ = 'f: {
        { let _ = F::memset(c, fp, 0i32, 3728u32); };
        F::DcList_Reset(c, fp);
        { let v_ = p_section_1; let a_ = (fp).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
        F::DcList_Init(c, fp);
        F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
        'l1: while ((1i32) != 0) {
            F::Gfx_BeginFrame(c);
            F::Pad_Update(c);
            F::ColorFade_Update(c);
            F::Snd_Update(c);
            F::DcList_Update(c, fp);
            F::DcList_Draw(c, fp);
            F::ColorFade_Draw(c);
            F::Gfx_EndFrame(c, 1i32);
            F::Dma_Flush(c);
            F::File_Stub264D90(c);
            if ((F::DcList_CheckLeave(c, fp)) != 0) {
                break 'l1;
            }
            F::DcList_Input(c, fp);
            F::DcList_UpdateFace(c, fp);
        }
        F::DcList_Term(c, fp);
        F::Dma_ResetBuffers(c);
        break 'f (c.m.r32((fp).wrapping_add(3228)) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcList_Run", &PROBE_N, &[], true); }
    r_
}
