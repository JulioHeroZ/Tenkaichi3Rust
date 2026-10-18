#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn menu_z_d__DcPass_Wrap(c: &mut Ctx, mut p_value_1: i32, mut p_lo_2: i32, mut p_hi_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_z_d__DcPass_Wrap", &PROBE_N, &[(p_value_1) as u32, (p_lo_2) as u32, (p_hi_3) as u32], false); }
    let r_ = 'f: {
    if (((((p_value_1) < (p_lo_2)) as i32)) != 0) {
        break 'f p_hi_3;
    }
    if (((((p_value_1) > (p_hi_3)) as i32)) != 0) {
        break 'f p_lo_2;
    }
    break 'f p_value_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_z_d__DcPass_Wrap", &PROBE_N, &[], true); }
    r_
}

pub fn menu_z_d__DcPass_SetUv(c: &mut Ctx, mut p_flash_1: u32, mut p_parent_2: u32, mut p_name_3: u32, mut p_uv_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_z_d__DcPass_SetUv", &PROBE_N, &[(p_flash_1) as u32, (p_parent_2) as u32, (p_name_3) as u32, (p_uv_4) as u32], false); }
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
    if rt::probing() { crate::host::harness::probe(c, "menu_z_d__DcPass_SetUv", &PROBE_N, &[], true); }
    r_
}

pub fn menu_z_d__DcPass_SetCell(c: &mut Ctx, mut p_uv_1: u32, mut p_col_2: i32, mut p_row_3: i32, mut p_w_4: i32, mut p_h_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_z_d__DcPass_SetCell", &PROBE_N, &[(p_uv_1) as u32, (p_col_2) as u32, (p_row_3) as u32, (p_w_4) as u32, (p_h_5) as u32], false); }
    let r_ = 'f: {
    { let v_ = (p_col_2).wrapping_mul(p_w_4); let a_ = p_uv_1; c.m.w32(a_, (v_) as u32); };
    { let v_ = (p_row_3).wrapping_mul(p_h_5); let a_ = (p_uv_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((c.m.r32(p_uv_1) as i32)).wrapping_add(p_w_4); let a_ = (p_uv_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((c.m.r32((p_uv_1).wrapping_add(4)) as i32)).wrapping_add(p_h_5); let a_ = (p_uv_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_z_d__DcPass_SetCell", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_Wrap3(c: &mut Ctx, mut p_value_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_Wrap3", &PROBE_N, &[(p_value_1) as u32], false); }
    let r_ = 'f: {
    break 'f F::menu_z_d__DcPass_Wrap(c, p_value_1, 0i32, 2i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "DcPass_Wrap3", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_GetKey(c: &mut Ctx, mut p_pos_1: u32) -> i8 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_GetKey", &PROBE_N, &[(p_pos_1) as u32], false); }
    let mut l_key_2: i8 = 0;
    let r_ = 'f: {
    l_key_2 = { let a_ = (((sym::gDcPassKeys).wrapping_add(((((c.m.r32(p_pos_1) as i32)) as u32)).wrapping_mul(70))).wrapping_add(((((c.m.r32((p_pos_1).wrapping_add(4)) as i32)) as u32)).wrapping_mul(14))).wrapping_add((((c.m.r32((p_pos_1).wrapping_add(8)) as i32)) as u32)); (c.m.r8(a_) as i8) };
    if (((((((l_key_2) as i32)) == (4i32)) as i32)) != 0) {
        l_key_2 = ((48i32) as i8);
    }
    break 'f l_key_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "DcPass_GetKey", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_WrapPos(c: &mut Ctx, mut p_pos_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_WrapPos", &PROBE_N, &[(p_pos_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = { let a0_ = (c.m.r32(p_pos_1) as i32); F::menu_z_d__DcPass_Wrap(c, a0_, 0i32, (2i32).wrapping_sub(1i32)) }; let a_ = p_pos_1; c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a0_ = (c.m.r32((p_pos_1).wrapping_add(4)) as i32); F::menu_z_d__DcPass_Wrap(c, a0_, 0i32, (5i32).wrapping_sub(1i32)) }; let a_ = (p_pos_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a0_ = (c.m.r32((p_pos_1).wrapping_add(8)) as i32); F::menu_z_d__DcPass_Wrap(c, a0_, 0i32, (14i32).wrapping_sub(1i32)) }; let a_ = (p_pos_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcPass_WrapPos", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_GotoButton(c: &mut Ctx, mut p_button_1: i32, mut p_pos_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_GotoButton", &PROBE_N, &[(p_button_1) as u32, (p_pos_2) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv3: i32 = p_button_1;
        match sv3 {
            2i32 => {
                { let v_ = (5i32).wrapping_sub(1i32); let a_ = (p_pos_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = (p_pos_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            1i32 => {
                { let v_ = (5i32).wrapping_sub(1i32); let a_ = (p_pos_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = 6i32; let a_ = (p_pos_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcPass_GotoButton", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_IsSameKeyCol(c: &mut Ctx, mut p_pos_1: u32, mut p_col_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_IsSameKeyCol", &PROBE_N, &[(p_pos_1) as u32, (p_col_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let s_ = p_pos_1; rt::memcpy(c, fp, s_, 12); }
        { let v_ = p_col_2; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        break 'f (((((F::DcPass_GetKey(c, p_pos_1)) as i32)) == (((F::DcPass_GetKey(c, fp)) as i32))) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcPass_IsSameKeyCol", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_IsSameKeyRow(c: &mut Ctx, mut p_pos_1: u32, mut p_row_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_IsSameKeyRow", &PROBE_N, &[(p_pos_1) as u32, (p_row_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let s_ = p_pos_1; rt::memcpy(c, fp, s_, 12); }
        { let v_ = p_row_2; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        break 'f (((((F::DcPass_GetKey(c, p_pos_1)) as i32)) == (((F::DcPass_GetKey(c, fp)) as i32))) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcPass_IsSameKeyRow", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_GetClipPos(c: &mut Ctx, mut p_pos_1: u32, mut p_out_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_GetClipPos", &PROBE_N, &[(p_pos_1) as u32, (p_out_2) as u32], false); }
    let r_ = 'f: {
    { let s_ = p_pos_1; let d_ = p_out_2; rt::memcpy(c, d_, s_, 12); };
    if ((((((c.m.r32((p_pos_1).wrapping_add(4)) as i32)) == (1i32)) as i32)) != 0) {
        if ((((((c.m.r32((p_pos_1).wrapping_add(8)) as i32)) >= (12i32)) as i32)) != 0) {
            { let a_ = (p_out_2).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
    } else {
        if ((((((c.m.r32((p_pos_1).wrapping_add(4)) as i32)) == (3i32)) as i32)) != 0) {
            { let r_ = 2i32; let a_ = (p_out_2).wrapping_add(8); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
            if ((((((c.m.r32((p_out_2).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                { let v_ = 0i32; let a_ = (p_out_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            }
            if ((((((c.m.r32((p_pos_1).wrapping_add(8)) as i32)) >= (12i32)) as i32)) != 0) {
                { let v_ = 10i32; let a_ = (p_out_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            }
        } else {
            if ((((((c.m.r32((p_pos_1).wrapping_add(4)) as i32)) == (4i32)) as i32)) != 0) {
                { let v_ = ((F::DcPass_GetKey(c, p_pos_1)) as i32); let a_ = (p_out_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcPass_GetClipPos", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_FindKey(c: &mut Ctx, sret: u32, mut p_c_1: i8) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_FindKey", &PROBE_N, &[(sret) as u32, (p_c_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        if (((((((((((p_c_1) as i32)) >= (48i32)) as i32)) != 0) && (((((((p_c_1) as i32)) <= (57i32)) as i32)) != 0)) as i32)) != 0) {
            { let v_ = p_c_1; let a_ = (fp.wrapping_add(12)).wrapping_add(((0i32) as u32)); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((0i32) as i8); let a_ = (fp.wrapping_add(12)).wrapping_add(((1i32) as u32)); c.m.w8(a_, (v_) as u8); };
            { let v_ = 0i32; let a_ = fp; c.m.w32(a_, (v_) as u32); };
            { let v_ = 3i32; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            { let v_ = F::atoi(c, fp.wrapping_add(12)); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            break 'f { let s_ = fp; rt::memcpy(c, sret, s_, 12); sret };
        }
        {
            { let v_ = 0i32; let a_ = fp; c.m.w32(a_, (v_) as u32); };
            'l1: loop {
                if !((((((c.m.r32(fp) as i32)) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    {
                        { let v_ = 0i32; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        'l3: loop {
                            if !((((((c.m.r32((fp).wrapping_add(4)) as i32)) < (5i32)) as i32)) != 0) { break 'l3; }
                            'c4: {
                                {
                                    { let v_ = 0i32; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                    'l5: loop {
                                        if !((((((c.m.r32((fp).wrapping_add(8)) as i32)) < (14i32)) as i32)) != 0) { break 'l5; }
                                        'c6: {
                                            if (((((((p_c_1) as i32)) == (((F::DcPass_GetKey(c, fp)) as i32))) as i32)) != 0) {
                                                break 'f { let s_ = fp; rt::memcpy(c, sret, s_, 12); sret };
                                            }
                                        }
                                        { let a_ = (fp).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                    }
                                }
                            }
                            { let a_ = (fp).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        }
                    }
                }
                { let a_ = fp; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
        }
        break 'f { let s_ = fp; rt::memcpy(c, sret, s_, 12); sret };
       sret
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcPass_FindKey", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_DrawKeys(c: &mut Ctx, mut p_flash_1: u32, mut p_cur_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_DrawKeys", &PROBE_N, &[(p_flash_1) as u32, (p_cur_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(304) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 304);
    let mut l_key_3: i8 = 0;
    let mut l_shift_4: i32 = 0;
    let r_ = 'f: {
        { let v_ = (c.m.r32(p_cur_2) as i32); let a_ = fp.wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        {
            { let v_ = 0i32; let a_ = (fp.wrapping_add(12)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            'l1: loop {
                if !((((((c.m.r32((fp.wrapping_add(12)).wrapping_add(4)) as i32)) < ((5i32).wrapping_sub(1i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    {
                        { let v_ = 0i32; let a_ = (fp.wrapping_add(12)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        'l3: loop {
                            if !((((((c.m.r32((fp.wrapping_add(12)).wrapping_add(8)) as i32)) < (14i32)) as i32)) != 0) { break 'l3; }
                            'c4: {
                                l_key_3 = F::DcPass_GetKey(c, p_cur_2);
                                F::DcPass_GetClipPos(c, fp.wrapping_add(12), fp);
                                l_shift_4 = (if (((((((((((l_key_3) as i32)) >= (97i32)) as i32)) != 0) && (((((((l_key_3) as i32)) <= (122i32)) as i32)) != 0)) as i32)) != 0) { 4i32 } else { 0i32 });
                                { let a1_ = (c.m.r32((fp).wrapping_add(8)) as i32); let a2_ = ((c.m.r32((fp).wrapping_add(4)) as i32)).wrapping_add(l_shift_4); F::menu_z_d__DcPass_SetCell(c, fp.wrapping_add(24), a1_, a2_, 32i32, 40i32) };
                                { let _ = { let a2_ = ((c.m.r32((fp).wrapping_add(4)) as i32)).wrapping_add(1i32); let a3_ = ((c.m.r32((fp).wrapping_add(8)) as i32)).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(44), sym::menu_z_d__str0, &[((a2_) as i64 as u64), ((a3_) as i64 as u64)]) }; };
                                if (((((((((((l_key_3) as i32)) == (5i32)) as i32)) != 0) || (((((((l_key_3) as i32)) == (6i32)) as i32)) != 0)) as i32)) != 0) {
                                    F::menu_z_d__DcPass_SetUv(c, p_flash_1, fp.wrapping_add(44), sym::menu_z_d__str1, fp.wrapping_add(24));
                                } else {
                                    if (((((((l_key_3) as i32)) == (3i32)) as i32)) != 0) {
                                        F::menu_z_d__DcPass_SetUv(c, p_flash_1, fp.wrapping_add(44), sym::menu_z_d__str2, fp.wrapping_add(24));
                                    } else {
                                        F::menu_z_d__DcPass_SetUv(c, p_flash_1, fp.wrapping_add(44), sym::menu_z_d__str3, fp.wrapping_add(24));
                                    }
                                }
                            }
                            { let a_ = (fp.wrapping_add(12)).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        }
                    }
                }
                { let a_ = (fp.wrapping_add(12)).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcPass_DrawKeys", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_DrawButtons(c: &mut Ctx, mut p_flash_1: u32, mut p_page_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_DrawButtons", &PROBE_N, &[(p_flash_1) as u32, (p_page_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(288) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 288);
    let r_ = 'f: {
        F::menu_z_d__DcPass_SetCell(c, fp.wrapping_add(256), 0i32, 1i32, 256i32, 64i32);
        { let _ = F::sprintf(c, fp, sym::menu_z_d__str4, &[((1i32) as i64 as u64)]); };
        F::menu_z_d__DcPass_SetUv(c, p_flash_1, fp, sym::menu_z_d__str5, fp.wrapping_add(256));
        F::menu_z_d__DcPass_SetUv(c, p_flash_1, fp, sym::menu_z_d__str6, fp.wrapping_add(256));
        if (((((p_page_2) == (0i32)) as i32)) != 0) {
            F::menu_z_d__DcPass_SetCell(c, fp.wrapping_add(256), 0i32, 3i32, 256i32, 64i32);
        } else {
            { let v_ = 0i32; let a_ = fp.wrapping_add(256); c.m.w32(a_, (v_) as u32); };
            { let v_ = 256i32; let a_ = (fp.wrapping_add(256)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = 128i32; let a_ = (fp.wrapping_add(256)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            { let v_ = 192i32; let a_ = (fp.wrapping_add(256)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        }
        { let _ = F::sprintf(c, fp, sym::menu_z_d__str7, &[((2i32) as i64 as u64)]); };
        F::menu_z_d__DcPass_SetUv(c, p_flash_1, fp, sym::menu_z_d__str8, fp.wrapping_add(256));
        F::menu_z_d__DcPass_SetUv(c, p_flash_1, fp, sym::menu_z_d__str9, fp.wrapping_add(256));
        F::menu_z_d__DcPass_SetCell(c, fp.wrapping_add(256), 0i32, 0i32, 256i32, 64i32);
        { let _ = F::sprintf(c, fp, sym::menu_z_d__str10, &[((3i32) as i64 as u64)]); };
        F::menu_z_d__DcPass_SetUv(c, p_flash_1, fp, sym::menu_z_d__str11, fp.wrapping_add(256));
        F::menu_z_d__DcPass_SetUv(c, p_flash_1, fp, sym::menu_z_d__str12, fp.wrapping_add(256));
        { let v_ = 0i32; let a_ = fp.wrapping_add(256); c.m.w32(a_, (v_) as u32); };
        { let v_ = 512i32; let a_ = (fp.wrapping_add(256)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = (p_page_2).wrapping_shl((7i32) as u32); let a_ = (fp.wrapping_add(256)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((p_page_2).wrapping_shl((7i32) as u32)).wrapping_add(128i32); let a_ = (fp.wrapping_add(256)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::menu_z_d__DcPass_SetUv(c, p_flash_1, ((0i32) as u32), sym::menu_z_d__str13, fp.wrapping_add(256));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcPass_DrawButtons", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_LightKey(c: &mut Ctx, mut p_flash_1: u32, mut p_pos_2: u32, mut p_on_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_LightKey", &PROBE_N, &[(p_flash_1) as u32, (p_pos_2) as u32, (p_on_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(288) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 288);
    let r_ = 'f: {
        F::DcPass_WrapPos(c, p_pos_2);
        F::DcPass_GetClipPos(c, p_pos_2, fp.wrapping_add(8));
        { let _ = { let a2_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(1i32); let a3_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(8)) as i32)).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(20), sym::menu_z_d__str14, &[((a2_) as i64 as u64), ((a3_) as i64 as u64)]) }; };
        F::Flash_FindLabel(c, p_flash_1, ((0i32) as u32), fp.wrapping_add(20), fp);
        if ((p_on_3) != 0) {
            F::Flash_ClipGotoLabel(c, p_flash_1, fp, sym::menu_z_d__str15);
        } else {
            F::Flash_ClipGotoLabel(c, p_flash_1, fp, sym::menu_z_d__str16);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcPass_LightKey", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_FlashKey(c: &mut Ctx, mut p_flash_1: u32, mut p_pos_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_FlashKey", &PROBE_N, &[(p_flash_1) as u32, (p_pos_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(288) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 288);
    let r_ = 'f: {
        F::DcPass_GetClipPos(c, p_pos_2, fp.wrapping_add(8));
        { let _ = { let a2_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(1i32); let a3_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(8)) as i32)).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(20), sym::menu_z_d__str17, &[((a2_) as i64 as u64), ((a3_) as i64 as u64)]) }; };
        F::Flash_FindLabel(c, p_flash_1, ((0i32) as u32), fp.wrapping_add(20), fp);
        F::Flash_ClipGotoLabel(c, p_flash_1, fp, sym::menu_z_d__str18);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcPass_FlashKey", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_DrawKeyboard(c: &mut Ctx, mut p_pass_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_DrawKeyboard", &PROBE_N, &[(p_pass_1) as u32], false); }
    let mut l_flash_2: u32 = 0;
    let mut l_pos_3: u32 = 0;
    let r_ = 'f: {
    l_flash_2 = ((p_pass_1).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
    l_pos_3 = (p_pass_1).wrapping_add(2260);
    F::DcPass_DrawKeys(c, l_flash_2, l_pos_3);
    { let a1_ = (c.m.r32(l_pos_3) as i32); F::DcPass_DrawButtons(c, l_flash_2, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcPass_DrawKeyboard", &PROBE_N, &[], true); }
    r_
}

pub fn DcPassText_Clear(c: &mut Ctx, mut p_text_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPassText_Clear", &PROBE_N, &[(p_text_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (34i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = ((32i32) as i8); let a_ = (p_text_1).wrapping_add(((l_i_2) as u32)); c.m.w8(a_, (v_) as u8); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    { let v_ = ((0i32) as i8); let a_ = (p_text_1).wrapping_add(((34i32) as u32)); c.m.w8(a_, (v_) as u8); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcPassText_Clear", &PROBE_N, &[], true); }
    r_
}

pub fn DcPassText_Put(c: &mut Ctx, mut p_pos_1: u32, mut p_text_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPassText_Put", &PROBE_N, &[(p_pos_1) as u32, (p_text_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = F::DcPass_GetKey(c, p_pos_1); let a_ = (p_text_2).wrapping_add((((c.m.r32((p_text_2).wrapping_add(72)) as i32)) as u32)); c.m.w8(a_, (v_) as u8); };
    if ((((((c.m.r32((p_text_2).wrapping_add(72)) as i32)) < ((34i32).wrapping_sub(1i32))) as i32)) != 0) {
        { let a_ = (p_text_2).wrapping_add(72); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcPassText_Put", &PROBE_N, &[], true); }
    r_
}

pub fn DcPassText_Delete(c: &mut Ctx, mut p_text_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPassText_Delete", &PROBE_N, &[(p_text_1) as u32], false); }
    let mut bb_: u32 = 0;
    let r_ = 'f: {
    'fl1: loop {
        match bb_ {
            0 => {
                if ((((((c.m.r32((p_text_1).wrapping_add(72)) as i32)) == (0i32)) as i32)) != 0) { bb_ = 1; continue 'fl1; } else { bb_ = 3; continue 'fl1; }
            }
            1 => {
                if ((((((((c.m.r8((p_text_1).wrapping_add(((0i32) as u32))) as i8)) as i32)) == (32i32)) as i32)) != 0) { bb_ = 4; continue 'fl1; } else { bb_ = 5; continue 'fl1; }
            }
            2 => {
                { let v_ = ((32i32) as i8); let a_ = (p_text_1).wrapping_add((({ let a_ = (p_text_1).wrapping_add(72); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); o_ }) as u32)); c.m.w8(a_, (v_) as u8); };
                bb_ = 9; continue 'fl1;
            }
            3 => {
                if ((((((({ let a_ = (p_text_1).wrapping_add((((c.m.r32((p_text_1).wrapping_add(72)) as i32)) as u32)); (c.m.r8(a_) as i8) }) as i32)) == (32i32)) as i32)) != 0) { bb_ = 7; continue 'fl1; } else { bb_ = 8; continue 'fl1; }
            }
            4 => {
                break 'f 1i32;
            }
            5 => {
                bb_ = 2; continue 'fl1;
            }
            6 => {
                bb_ = 5; continue 'fl1;
            }
            7 => {
                { let a_ = (p_text_1).wrapping_add(72); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = ((32i32) as i8); let a_ = (p_text_1).wrapping_add((((c.m.r32((p_text_1).wrapping_add(72)) as i32)) as u32)); c.m.w8(a_, (v_) as u8); };
                bb_ = 9; continue 'fl1;
            }
            8 => {
                bb_ = 2; continue 'fl1;
            }
            9 => {
                if ((((((c.m.r32((p_text_1).wrapping_add(72)) as i32)) < (0i32)) as i32)) != 0) { bb_ = 11; continue 'fl1; } else { bb_ = 12; continue 'fl1; }
            }
            10 => {
                bb_ = 8; continue 'fl1;
            }
            11 => {
                { let v_ = 0i32; let a_ = (p_text_1).wrapping_add(72); c.m.w32(a_, (v_) as u32); };
                bb_ = 12; continue 'fl1;
            }
            12 => {
                break 'f 0i32;
            }
            13 => {
                break 'fl1;
            }
            _ => unreachable!(),
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "DcPassText_Delete", &PROBE_N, &[], true); }
    r_
}

pub fn DcPassText_IsFull(c: &mut Ctx, mut p_text_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPassText_IsFull", &PROBE_N, &[(p_text_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (34i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((((c.m.r8((p_text_1).wrapping_add(((l_i_2) as u32))) as i8)) as i32)) == (32i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "DcPassText_IsFull", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_DrawText(c: &mut Ctx, mut p_flash_1: u32, mut p_text_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_DrawText", &PROBE_N, &[(p_flash_1) as u32, (p_text_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(304) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 304);
    let mut l_i_3: i32 = 0;
    let mut l_c_4: i8 = 0;
    let r_ = 'f: {
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < ((34i32).wrapping_add(1i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_c_4 = (c.m.r8((p_text_2).wrapping_add(((l_i_3) as u32))) as i8);
                    { let s_ = F::DcPass_FindKey(c, fp.wrapping_add(288), l_c_4); let d_ = fp.wrapping_add(276); rt::memcpy(c, d_, s_, 12); };
                    if (((((((((((l_c_4) as i32)) >= (97i32)) as i32)) != 0) && (((((((l_c_4) as i32)) <= (122i32)) as i32)) != 0)) as i32)) != 0) {
                        { let a1_ = (c.m.r32((fp.wrapping_add(276)).wrapping_add(8)) as i32); let a2_ = ((c.m.r32((fp.wrapping_add(276)).wrapping_add(4)) as i32)).wrapping_add(4i32); F::menu_z_d__DcPass_SetCell(c, fp, a1_, a2_, 32i32, 40i32) };
                    } else {
                        { let a1_ = (c.m.r32((fp.wrapping_add(276)).wrapping_add(8)) as i32); let a2_ = (c.m.r32((fp.wrapping_add(276)).wrapping_add(4)) as i32); F::menu_z_d__DcPass_SetCell(c, fp, a1_, a2_, 32i32, 40i32) };
                    }
                    if (((((l_i_3) < (17i32)) as i32)) != 0) {
                        { let _ = F::sprintf(c, fp.wrapping_add(20), sym::menu_z_d__str19, &[((1i32) as i64 as u64), (((l_i_3).wrapping_add(1i32)) as i64 as u64)]); };
                    } else {
                        { let _ = F::sprintf(c, fp.wrapping_add(20), sym::menu_z_d__str20, &[((2i32) as i64 as u64), (((l_i_3).wrapping_sub(16i32)) as i64 as u64)]); };
                    }
                    F::menu_z_d__DcPass_SetUv(c, p_flash_1, ((0i32) as u32), fp.wrapping_add(20), fp);
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcPass_DrawText", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_DrawCursor(c: &mut Ctx, mut p_flash_1: u32, mut p_text_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_DrawCursor", &PROBE_N, &[(p_flash_1) as u32, (p_text_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(288) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 288);
    let r_ = 'f: {
        if ((((((c.m.r32((p_text_2).wrapping_add(72)) as i32)) < (17i32)) as i32)) != 0) {
            { let _ = { let a3_ = ((c.m.r32((p_text_2).wrapping_add(72)) as i32)).wrapping_add(1i32); F::sprintf(c, fp, sym::menu_z_d__str21, &[((1i32) as i64 as u64), ((a3_) as i64 as u64)]) }; };
        } else {
            { let _ = { let a3_ = ((c.m.r32((p_text_2).wrapping_add(72)) as i32)).wrapping_sub(16i32); F::sprintf(c, fp, sym::menu_z_d__str22, &[((2i32) as i64 as u64), ((a3_) as i64 as u64)]) }; };
        }
        F::Flash_FindLabel(c, p_flash_1, ((0i32) as u32), fp, fp.wrapping_add(256));
        F::Flash_ClipGetPos(c, p_flash_1, fp.wrapping_add(256), fp.wrapping_add(264), fp.wrapping_add(268));
        F::Flash_FindLabel(c, p_flash_1, ((0i32) as u32), sym::menu_z_d__str23, fp.wrapping_add(256));
        F::Flash_ClipSetOffset(c, p_flash_1, fp.wrapping_add(256), 0i32, 0i32);
        F::Flash_ClipGetPos(c, p_flash_1, fp.wrapping_add(256), fp.wrapping_add(272), fp.wrapping_add(276));
        { let r_ = (c.m.r32(fp.wrapping_add(272)) as i32); let a_ = fp.wrapping_add(264); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
        { let r_ = (c.m.r32(fp.wrapping_add(276)) as i32); let a_ = fp.wrapping_add(268); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
        F::Flash_FindLabel(c, p_flash_1, ((0i32) as u32), sym::menu_z_d__str24, fp.wrapping_add(256));
        { let a2_ = (c.m.r32(fp.wrapping_add(264)) as i32); let a3_ = ((c.m.r32(fp.wrapping_add(268)) as i32)).wrapping_add(10i32); F::Flash_ClipSetOffset(c, p_flash_1, fp.wrapping_add(256), a2_, a3_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcPass_DrawCursor", &PROBE_N, &[], true); }
    r_
}

pub fn DcPassText_Pack(c: &mut Ctx, mut p_text_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPassText_Pack", &PROBE_N, &[(p_text_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !((((((((c.m.r8((p_text_1).wrapping_add(((l_i_2) as u32))) as i8)) as i32)) != (0i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = (c.m.r8((p_text_1).wrapping_add(((l_i_2) as u32))) as i8); let a_ = ((p_text_1).wrapping_add(35)).wrapping_add(((l_i_2) as u32)); c.m.w8(a_, (v_) as u8); };
                if ((((((((((((c.m.r8((p_text_1).wrapping_add(((l_i_2) as u32))) as i8)) as i32)) == (32i32)) as i32)) != 0) && (((((l_i_2) < (32i32)) as i32)) != 0)) as i32)) != 0) {
                    break 'f 1i32;
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    { let v_ = ((0i32) as i8); let a_ = ((p_text_1).wrapping_add(35)).wrapping_add(((l_i_2) as u32)); c.m.w8(a_, (v_) as u8); };
    if ((((((((c.m.r8(((p_text_1).wrapping_add(35)).wrapping_add(((32i32) as u32))) as i8)) as i32)) == (32i32)) as i32)) != 0) {
        if ((((((((c.m.r8(((p_text_1).wrapping_add(35)).wrapping_add(((33i32) as u32))) as i8)) as i32)) == (32i32)) as i32)) != 0) {
            { let v_ = ((0i32) as i8); let a_ = ((p_text_1).wrapping_add(35)).wrapping_add(((32i32) as u32)); c.m.w8(a_, (v_) as u8); };
        } else {
            break 'f 1i32;
        }
    }
    { let v_ = ((F::strlen(c, (p_text_1).wrapping_add(35))) as i32); let a_ = (p_text_1).wrapping_add(76); c.m.w32(a_, (v_) as u32); };
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "DcPassText_Pack", &PROBE_N, &[], true); }
    r_
}

pub fn DcPassList_Refresh(c: &mut Ctx, mut p_list_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPassList_Refresh", &PROBE_N, &[(p_list_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (14i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = { let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(11584)).wrapping_add((((l_i_2) as u32)).wrapping_mul(28))).wrapping_add(24); (c.m.r32(a_) as i32) }; let a_ = ((p_list_1).wrapping_add(12)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcPassList_Refresh", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_StoreRec(c: &mut Ctx, mut p_status_1: u32, mut p_list_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_StoreRec", &PROBE_N, &[(p_status_1) as u32, (p_list_2) as u32], false); }
    let mut l_idx_3: i32 = 0;
    let r_ = 'f: {
    l_idx_3 = ((c.m.r32(p_list_2) as i32)).wrapping_add((c.m.r32((p_list_2).wrapping_add(4)) as i32));
    { let v_ = (c.m.r32(p_status_1) as i32); let a_ = ((p_list_2).wrapping_add(12)).wrapping_add((((l_idx_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let s_ = (p_status_1).wrapping_add(28); let d_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11584)).wrapping_add((((l_idx_3) as u32)).wrapping_mul(28)); rt::memcpy(c, d_, s_, 28); };
    { let r_ = 1i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcPass_StoreRec", &PROBE_N, &[], true); }
    r_
}

pub fn menu_z_d__DcPass_IsListed(c: &mut Ctx, mut p_pass_1: u32, mut p_chara_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_z_d__DcPass_IsListed", &PROBE_N, &[(p_pass_1) as u32, (p_chara_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < ((c.m.r32((p_pass_1).wrapping_add(2416)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((({ let a_ = (c.m.r32((p_pass_1).wrapping_add(2412))).wrapping_add((((l_i_3) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) }) == (p_chara_2)) as i32)) != 0) {
                    break 'f 1i32;
                }
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_z_d__DcPass_IsListed", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_Decode(c: &mut Ctx, mut p_pass_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_Decode", &PROBE_N, &[(p_pass_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(144) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 144);
    let mut l_chrTbl_2: u32 = 0;
    let mut l_attr_3: u16 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
        { let _ = F::memset(c, fp, 0i32, 44u32); };
        if (((((F::DcPassText_Pack(c, (p_pass_1).wrapping_add(2152))) != (0i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        'l1: loop {
            'c2: {
                if ((((((c.m.r32(((p_pass_1).wrapping_add(2152)).wrapping_add(76)) as i32)) == (32i32)) as i32)) != 0) {
                    { let _ = F::memset(c, fp.wrapping_add(44), 0i32, 48u32); };
                    if (((((F::OldPass_DecodeChar(c, fp.wrapping_add(44), ((p_pass_1).wrapping_add(2152)).wrapping_add(35))) != (1i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if ((((!((F::PassChk_IsOldValid(c, fp.wrapping_add(44))) != 0)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    { let _ = fp.wrapping_add(44); F::PassChk_ConvertOld(c, fp.wrapping_add(92), fp) };
                } else {
                    if ((((((c.m.r32(((p_pass_1).wrapping_add(2152)).wrapping_add(76)) as i32)) == (34i32)) as i32)) != 0) {
                        if (((((F::ChrPass_Decode(c, fp, ((p_pass_1).wrapping_add(2152)).wrapping_add(35))) == (1i32)) as i32)) != 0) {
                            if ((((!((F::PassChk_IsValid(c, fp)) != 0)) as i32)) != 0) {
                                break 'f 0i32;
                            }
                        } else {
                            break 'f 0i32;
                        }
                    } else {
                        break 'f 0i32;
                    }
                }
                if ((((!(({ let a1_ = (c.m.r32(fp) as i32); F::menu_z_d__DcPass_IsListed(c, p_pass_1, a1_) }) != 0)) as i32)) != 0) {
                    break 'f 0i32;
                }
            }
            if !((0i32) != 0) { break 'l1; }
        }
        { let v_ = (c.m.r32(fp) as i32); let a_ = (((p_pass_1).wrapping_add(2340)).wrapping_add(28)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
        { let v_ = (c.m.r32(fp) as i32); let a_ = (p_pass_1).wrapping_add(2340); c.m.w32(a_, (v_) as u32); };
        {
            l_i_4 = 0i32;
            'l3: loop {
                if !(((((l_i_4) < (8i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let v_ = (((c.m.r32(((fp).wrapping_add(4)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))) as i32)) as u16); let a_ = (((p_pass_1).wrapping_add(2340)).wrapping_add(28)).wrapping_add((((l_i_4) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        { let v_ = (((c.m.r32((fp).wrapping_add(36)) as i32)) as u16); let a_ = (((p_pass_1).wrapping_add(2340)).wrapping_add(28)).wrapping_add(20); c.m.w16(a_, v_); };
        { let a1_ = c.m.r32((p_pass_1).wrapping_add(2408)); F::ItemSet_GetBonus(c, ((p_pass_1).wrapping_add(2340)).wrapping_add(28), a1_, (((p_pass_1).wrapping_add(2340)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) };
        l_chrTbl_2 = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
        l_attr_3 = { let a_ = ((l_chrTbl_2).wrapping_add(((((c.m.r32((p_pass_1).wrapping_add(2340)) as i32)) as u32)).wrapping_mul(60))).wrapping_add(8); c.m.r16(a_) };
        { let r_ = 8i32; let a_ = (p_pass_1).wrapping_add(2404); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let v_ = ((((((l_attr_3) as i32)) ^ (1i32))) & (1i32)); let a_ = ((p_pass_1).wrapping_add(2340)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
        F::DcPassList_Refresh(c, (p_pass_1).wrapping_add(2272));
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcPass_Decode", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_UpdateFace(c: &mut Ctx, mut p_pass_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_UpdateFace", &PROBE_N, &[(p_pass_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        if (((((c.m.r32((p_pass_1).wrapping_add(2404)) as i32)) & (8i32))) != 0) {
            F::File_CancelRequests(c);
            { let _ = { let a0_ = ((c.m.r32((p_pass_1).wrapping_add(2340)) as i32)).wrapping_add(761i32); let a1_ = c.m.r32((p_pass_1).wrapping_add(8)); let _ = 92160i32; F::File_Request(c, a0_, a1_) }; };
            if (((((c.m.r32((p_pass_1).wrapping_add(2404)) as i32)) & (8i32))) != 0) {
                { let r_ = 8i32; let a_ = (p_pass_1).wrapping_add(2404); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
            }
            { let r_ = 16i32; let a_ = (p_pass_1).wrapping_add(2404); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            { let v_ = F32(0x00000000); let a_ = (p_pass_1).wrapping_add(2424); c.m.w32(a_, (v_).0); };
            if (((((c.m.r32((p_pass_1).wrapping_add(2404)) as i32)) & (32i32))) != 0) {
                { let r_ = 32i32; let a_ = (p_pass_1).wrapping_add(2404); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
        if (((((c.m.r32((p_pass_1).wrapping_add(2404)) as i32)) & (16i32))) != 0) {
            if ((F::File_UpdateRequests(c)) != 0) {
                { let v_ = ((0i32) as u32); let a_ = fp; c.m.w32(a_, v_); };
                { let _ = { let a0_ = c.m.r32((p_pass_1).wrapping_add(8)); let a1_ = c.m.r32((p_pass_1).wrapping_add(12)); F::Sprite_Unpack(c, a0_, a1_, ((0i32) as u32)) }; };
                { let v_ = c.m.r32((p_pass_1).wrapping_add(12)); let a_ = fp; c.m.w32(a_, v_); };
                { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
                { let v_ = { let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }; let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                if (((((c.m.r32((p_pass_1).wrapping_add(2404)) as i32)) & (16i32))) != 0) {
                    { let r_ = 16i32; let a_ = (p_pass_1).wrapping_add(2404); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                }
                { let r_ = 32i32; let a_ = (p_pass_1).wrapping_add(2404); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
        if (((((c.m.r32((p_pass_1).wrapping_add(2404)) as i32)) & (32i32))) != 0) {
            if (((rt::flt(F32(c.m.r32((p_pass_1).wrapping_add(2424))), F32(0x3f800000)) as i32)) != 0) {
                { let r_ = F32(0x3d4ccccc); let a_ = (p_pass_1).wrapping_add(2424); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcPass_UpdateFace", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_DrawTextLine(c: &mut Ctx, mut p_pass_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_DrawTextLine", &PROBE_N, &[(p_pass_1) as u32], false); }
    let mut l_flash_2: u32 = 0;
    let mut l_text_3: u32 = 0;
    let r_ = 'f: {
    l_flash_2 = ((p_pass_1).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
    l_text_3 = (p_pass_1).wrapping_add(2152);
    F::DcPass_DrawText(c, l_flash_2, l_text_3);
    F::DcPass_DrawCursor(c, l_flash_2, l_text_3);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcPass_DrawTextLine", &PROBE_N, &[], true); }
    r_
}

pub fn menu_z_d__DcPass_SetVisible(c: &mut Ctx, mut p_flash_1: u32, mut p_parent_2: u32, mut p_name_3: u32, mut p_on_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_z_d__DcPass_SetVisible", &PROBE_N, &[(p_flash_1) as u32, (p_parent_2) as u32, (p_name_3) as u32, (p_on_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        F::Flash_FindLabel(c, p_flash_1, p_parent_2, p_name_3, fp);
        { let _ = F::Flash_ClipSetFlags(c, p_flash_1, fp, 2i32, ((((p_on_4) as i64 as u64)) as u8)); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "menu_z_d__DcPass_SetVisible", &PROBE_N, &[], true); }
    r_
}

pub fn menu_z_d__DcPass_SetTex(c: &mut Ctx, mut p_flash_1: u32, mut p_parent_2: u32, mut p_name_3: u32, mut p_tex_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_z_d__DcPass_SetTex", &PROBE_N, &[(p_flash_1) as u32, (p_parent_2) as u32, (p_name_3) as u32, (p_tex_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        F::Flash_FindLabel(c, p_flash_1, p_parent_2, p_name_3, fp);
        F::Flash_ClipSetTex(c, p_flash_1, fp, p_tex_4);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "menu_z_d__DcPass_SetTex", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_DrawStatus(c: &mut Ctx, mut p_view_1: u32, mut p_status_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_DrawStatus", &PROBE_N, &[(p_view_1) as u32, (p_status_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(544) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 544);
    let mut l_i_3: i32 = 0;
    let mut l_j_4: i32 = 0;
    let mut l_value_5: i32 = 0;
    let r_ = 'f: {
        {
            l_i_3 = (c.m.r32(((p_status_2).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32);
            'l1: loop {
                if !(((((l_i_3) < (7i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let _ = F::sprintf(c, fp, sym::menu_z_d__str25, &[(((l_i_3).wrapping_add(4i32)) as i64 as u64)]); };
                    F::menu_z_d__DcPass_SetVisible(c, (p_view_1).wrapping_add((((1i32) as u32)).wrapping_mul(44)), sym::menu_z_d__str26, fp, 0i32);
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        F::menu_z_d__DcPass_SetCell(c, fp.wrapping_add(512), 0i32, l_i_3, 128i32, 20i32);
        F::menu_z_d__DcPass_SetUv(c, (p_view_1).wrapping_add((((1i32) as u32)).wrapping_mul(44)), sym::menu_z_d__str27, sym::menu_z_d__str28, fp.wrapping_add(512));
        {
            l_i_3 = 0i32;
            'l3: loop {
                if !(((((l_i_3) < (4i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    l_value_5 = (c.m.r32(((p_status_2).wrapping_add(4)).wrapping_add(((((l_i_3).wrapping_add(1i32)) as u32)).wrapping_mul(4))) as i32);
                    { let _ = F::sprintf(c, fp, sym::menu_z_d__str29, &[(((l_i_3).wrapping_add(1i32)) as i64 as u64)]); };
                    {
                        l_j_4 = 0i32;
                        'l5: loop {
                            if !(((((l_j_4) < (4i32)) as i32)) != 0) { break 'l5; }
                            'c6: {
                                { let _ = F::sprintf(c, fp.wrapping_add(256), sym::menu_z_d__str30, &[(((l_j_4).wrapping_add(1i32)) as i64 as u64)]); };
                                F::menu_z_d__DcPass_SetVisible(c, (p_view_1).wrapping_add((((1i32) as u32)).wrapping_mul(44)), fp, fp.wrapping_add(256), 0i32);
                                { let _ = F::sprintf(c, fp.wrapping_add(256), sym::menu_z_d__str31, &[(((l_j_4).wrapping_add(1i32)) as i64 as u64)]); };
                                F::menu_z_d__DcPass_SetVisible(c, (p_view_1).wrapping_add((((1i32) as u32)).wrapping_mul(44)), fp, fp.wrapping_add(256), 0i32);
                                if (((((((((((((l_value_5) < (0i32)) as i32)) != 0) && (((((l_value_5) < ((l_j_4).wrapping_neg())) as i32)) != 0)) as i32)) != 0) && ((((((l_j_4).wrapping_neg()) <= (0i32)) as i32)) != 0)) as i32)) != 0) {
                                    { let _ = F::sprintf(c, fp.wrapping_add(256), sym::menu_z_d__str32, &[(((l_j_4).wrapping_add(1i32)) as i64 as u64)]); };
                                    F::menu_z_d__DcPass_SetVisible(c, (p_view_1).wrapping_add((((1i32) as u32)).wrapping_mul(44)), fp, fp.wrapping_add(256), 1i32);
                                } else {
                                    if (((((((((((((l_value_5) > (0i32)) as i32)) != 0) && (((((l_j_4) >= (0i32)) as i32)) != 0)) as i32)) != 0) && (((((l_j_4) < (l_value_5)) as i32)) != 0)) as i32)) != 0) {
                                        { let _ = F::sprintf(c, fp.wrapping_add(256), sym::menu_z_d__str33, &[(((l_j_4).wrapping_add(1i32)) as i64 as u64)]); };
                                        F::menu_z_d__DcPass_SetVisible(c, (p_view_1).wrapping_add((((1i32) as u32)).wrapping_mul(44)), fp, fp.wrapping_add(256), 1i32);
                                    }
                                }
                            }
                            { let o_ = l_j_4; let n_ = (o_).wrapping_add(1); l_j_4 = n_; };
                        }
                    }
                    F::menu_z_d__DcPass_SetCell(c, fp.wrapping_add(512), 0i32, l_i_3, 128i32, 20i32);
                    { let _ = F::sprintf(c, fp, sym::menu_z_d__str34, &[((l_i_3) as i64 as u64)]); };
                    F::menu_z_d__DcPass_SetUv(c, (p_view_1).wrapping_add((((1i32) as u32)).wrapping_mul(44)), fp, sym::menu_z_d__str35, fp.wrapping_add(512));
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        {
            F::Flash_FindLabel(c, (p_view_1).wrapping_add((((1i32) as u32)).wrapping_mul(44)), ((0i32) as u32), sym::menu_z_d__str36, fp.wrapping_add(532));
            { let a2_ = (c.m.r32((p_status_2).wrapping_add(24)) as i32); F::Flash_ClipSetTex(c, (p_view_1).wrapping_add((((1i32) as u32)).wrapping_mul(44)), fp.wrapping_add(532), a2_) };
        }
        F::menu_z_d__DcPass_SetCell(c, fp.wrapping_add(512), 0i32, l_i_3, 128i32, 20i32);
        { let _ = F::sprintf(c, fp, sym::menu_z_d__str37, &[((l_i_3) as i64 as u64)]); };
        F::menu_z_d__DcPass_SetUv(c, (p_view_1).wrapping_add((((1i32) as u32)).wrapping_mul(44)), fp, sym::menu_z_d__str38, fp.wrapping_add(512));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcPass_DrawStatus", &PROBE_N, &[], true); }
    r_
}

pub fn menu_z_d__DcPass_SetFaceAlpha(c: &mut Ctx, mut p_pass_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_z_d__DcPass_SetFaceAlpha", &PROBE_N, &[(p_pass_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_flash_2: u32 = 0;
    let r_ = 'f: {
        l_flash_2 = ((p_pass_1).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44));
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_z_d__str39, fp);
        { let a2_ = F32(c.m.r32((p_pass_1).wrapping_add(2424))); F::Flash_ClipSetAlpha(c, l_flash_2, fp, a2_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "menu_z_d__DcPass_SetFaceAlpha", &PROBE_N, &[], true); }
    r_
}

pub fn menu_z_d__DcPass_Say(c: &mut Ctx, mut p_pass_1: u32, mut p_line_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_z_d__DcPass_Say", &PROBE_N, &[(p_pass_1) as u32, (p_line_2) as u32], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32(((p_pass_1).wrapping_add(20)).wrapping_add(376)); F::Voice_PlayWithSubtitle(c, a0_, 33688i32, p_line_2) };
    { let v_ = p_line_2; let a_ = (p_pass_1).wrapping_add(2248); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_z_d__DcPass_Say", &PROBE_N, &[], true); }
    r_
}

pub fn menu_z_d__DcPass_SetCellB(c: &mut Ctx, mut p_uv_1: u32, mut p_col_2: i32, mut p_row_3: i32, mut p_w_4: i32, mut p_h_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_z_d__DcPass_SetCellB", &PROBE_N, &[(p_uv_1) as u32, (p_col_2) as u32, (p_row_3) as u32, (p_w_4) as u32, (p_h_5) as u32], false); }
    let r_ = 'f: {
    { let v_ = (p_col_2).wrapping_mul(p_w_4); let a_ = p_uv_1; c.m.w32(a_, (v_) as u32); };
    { let v_ = ((c.m.r32(p_uv_1) as i32)).wrapping_add(p_w_4); let a_ = (p_uv_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = (p_row_3).wrapping_mul(p_h_5); let a_ = (p_uv_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((c.m.r32((p_uv_1).wrapping_add(4)) as i32)).wrapping_add(p_h_5); let a_ = (p_uv_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_z_d__DcPass_SetCellB", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_SetListClip(c: &mut Ctx, mut p_view_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_SetListClip", &PROBE_N, &[(p_view_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(272) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 272);
    let mut l_flash_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
        l_flash_2 = (p_view_1).wrapping_add((((1i32) as u32)).wrapping_mul(44));
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_z_d__str40, &[(((l_i_3).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(8), fp);
                    F::Flash_ClipSetCallbackA(c, l_flash_2, fp, fid::DcPass_ClipBegin, ((0i32) as u32));
                    F::Flash_ClipSetCallbackB(c, l_flash_2, fp, fid::DcPass_ClipEnd, ((0i32) as u32));
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcPass_SetListClip", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_HideArrow(c: &mut Ctx, mut p_view_1: u32, mut p_up_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_HideArrow", &PROBE_N, &[(p_view_1) as u32, (p_up_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(528) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 528);
    let mut l_flash_3: u32 = 0;
    let r_ = 'f: {
        l_flash_3 = (p_view_1).wrapping_add((((1i32) as u32)).wrapping_mul(44));
        if ((p_up_2) != 0) {
            { let _ = F::strcpy(c, fp.wrapping_add(8), sym::menu_z_d__str41); };
            { let _ = F::strcpy(c, fp.wrapping_add(264), sym::menu_z_d__str42); };
        } else {
            { let _ = F::strcpy(c, fp.wrapping_add(8), sym::menu_z_d__str43); };
            { let _ = F::strcpy(c, fp.wrapping_add(264), sym::menu_z_d__str44); };
        }
        F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(8), fp.wrapping_add(264), fp);
        { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcPass_HideArrow", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_DrawArrows(c: &mut Ctx, mut p_view_1: u32, mut p_list_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_DrawArrows", &PROBE_N, &[(p_view_1) as u32, (p_list_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_flash_3: u32 = 0;
    let mut l_top_4: i32 = 0;
    let r_ = 'f: {
        l_flash_3 = (p_view_1).wrapping_add((((1i32) as u32)).wrapping_mul(44));
        F::menu_z_d__DcPass_SetCellB(c, fp, 0i32, 1i32, 32i32, 32i32);
        F::menu_z_d__DcPass_SetUv(c, l_flash_3, sym::menu_z_d__str45, sym::menu_z_d__str46, fp);
        F::menu_z_d__DcPass_SetCellB(c, fp, 1i32, 1i32, 32i32, 32i32);
        F::menu_z_d__DcPass_SetUv(c, l_flash_3, sym::menu_z_d__str47, sym::menu_z_d__str48, fp);
        l_top_4 = (c.m.r32(p_list_2) as i32);
        if (((((l_top_4) == (0i32)) as i32)) != 0) {
            F::DcPass_HideArrow(c, p_view_1, 1i32);
        } else {
            if (((((l_top_4) == ((14i32).wrapping_sub(3i32))) as i32)) != 0) {
                F::DcPass_HideArrow(c, p_view_1, 0i32);
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcPass_DrawArrows", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_DrawScrollBar(c: &mut Ctx, mut p_view_1: u32, mut p_list_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_DrawScrollBar", &PROBE_N, &[(p_view_1) as u32, (p_list_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_flash_3: u32 = 0;
    let mut l_range_4: i32 = 0;
    let mut l_num_5: i32 = 0;
    let mut l_y_6: i32 = 0;
    let r_ = 'f: {
        l_flash_3 = (p_view_1).wrapping_add((((1i32) as u32)).wrapping_mul(44));
        l_range_4 = 181i32;
        l_num_5 = 14i32;
        l_y_6 = rt::div_i32(((c.m.r32(p_list_2) as i32)).wrapping_mul(l_range_4), l_num_5);
        F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), sym::menu_z_d__str49, fp);
        F::Flash_ClipSetScale(c, l_flash_3, fp, F32(0x3f800000), F32(0x3f9b3ee6));
        F::Flash_ClipSetOffset(c, l_flash_3, fp, 0i32, l_y_6);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcPass_DrawScrollBar", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_LightRow(c: &mut Ctx, mut p_view_1: u32, mut p_list_2: u32, mut p_on_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_LightRow", &PROBE_N, &[(p_view_1) as u32, (p_list_2) as u32, (p_on_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(272) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 272);
    let mut l_flash_4: u32 = 0;
    let r_ = 'f: {
        l_flash_4 = (p_view_1).wrapping_add((((1i32) as u32)).wrapping_mul(44));
        { let _ = { let a2_ = ((c.m.r32((p_list_2).wrapping_add(4)) as i32)).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_z_d__str50, &[((a2_) as i64 as u64)]) }; };
        F::Flash_FindLabel(c, l_flash_4, ((0i32) as u32), fp.wrapping_add(8), fp);
        if ((p_on_3) != 0) {
            F::Flash_ClipGotoLabel(c, l_flash_4, fp, sym::menu_z_d__str51);
        } else {
            F::Flash_ClipGotoLabel(c, l_flash_4, fp, sym::menu_z_d__str52);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcPass_LightRow", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_ClampTop(c: &mut Ctx, mut p_list_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_ClampTop", &PROBE_N, &[(p_list_1) as u32], false); }
    let r_ = 'f: {
    if ((((((c.m.r32(p_list_1) as i32)) < (0i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = p_list_1; c.m.w32(a_, (v_) as u32); };
        break 'f 0i32;
    }
    if ((((((c.m.r32(p_list_1) as i32)) > ((14i32).wrapping_sub(3i32))) as i32)) != 0) {
        { let v_ = (14i32).wrapping_sub(3i32); let a_ = p_list_1; c.m.w32(a_, (v_) as u32); };
        break 'f 0i32;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "DcPass_ClampTop", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_DrawRows(c: &mut Ctx, mut p_view_1: u32, mut p_list_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_DrawRows", &PROBE_N, &[(p_view_1) as u32, (p_list_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(272) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 272);
    let mut l_i_3: i32 = 0;
    let mut l_chara_4: i32 = 0;
    let mut l_buf_5: u32 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
        'fl1: loop {
            match bb_ {
                0 => {
                    l_buf_5 = fp.wrapping_add(8);
                    l_i_3 = 0i32;
                    bb_ = 1; continue 'fl1;
                }
                1 => {
                    if (((((l_i_3) < (4i32)) as i32)) != 0) { bb_ = 2; continue 'fl1; } else { bb_ = 4; continue 'fl1; }
                }
                2 => {
                    { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_z_d__str53, &[(((l_i_3).wrapping_add(1i32)) as i64 as u64)]); };
                    if (((((l_i_3) != (3i32)) as i32)) != 0) { bb_ = 5; continue 'fl1; } else { bb_ = 7; continue 'fl1; }
                }
                3 => {
                    { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                    bb_ = 1; continue 'fl1;
                }
                4 => {
                    break 'fl1;
                }
                5 => {
                    l_chara_4 = { let a_ = ((p_list_2).wrapping_add(12)).wrapping_add(((((l_i_3).wrapping_add((c.m.r32(p_list_2) as i32))) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                    if (((((l_i_3) == ((c.m.r32((p_list_2).wrapping_add(4)) as i32))) as i32)) != 0) { bb_ = 8; continue 'fl1; } else { bb_ = 10; continue 'fl1; }
                }
                6 => {
                    bb_ = 3; continue 'fl1;
                }
                7 => {
                    l_chara_4 = { let a_ = ((p_list_2).wrapping_add(12)).wrapping_add(((((c.m.r32((p_list_2).wrapping_add(8)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                    if (((((l_chara_4) == ((1i32).wrapping_neg())) as i32)) != 0) { bb_ = 18; continue 'fl1; } else { bb_ = 19; continue 'fl1; }
                }
                8 => {
                    if (((((l_chara_4) == ((1i32).wrapping_neg())) as i32)) != 0) { bb_ = 11; continue 'fl1; } else { bb_ = 12; continue 'fl1; }
                }
                9 => {
                    F::Flash_FindLabel(c, (p_view_1).wrapping_add((((1i32) as u32)).wrapping_mul(44)), fp.wrapping_add(8), sym::menu_z_d__str54, fp);
                    F::TextBox_AttachLine(c, (p_view_1).wrapping_add((((1i32) as u32)).wrapping_mul(44)), fp, 0i32, 0i32, l_chara_4, ((p_view_1).wrapping_add(440)).wrapping_add((((l_i_3) as u32)).wrapping_mul(140)));
                    F::Flash_FindLabel(c, (p_view_1).wrapping_add((((1i32) as u32)).wrapping_mul(44)), l_buf_5, sym::menu_z_d__str55, fp);
                    F::TextBox_AttachLine(c, (p_view_1).wrapping_add((((1i32) as u32)).wrapping_mul(44)), fp, 0i32, 0i32, l_chara_4, ((p_view_1).wrapping_add(860)).wrapping_add((((l_i_3) as u32)).wrapping_mul(140)));
                    F::Flash_FindLabel(c, (p_view_1).wrapping_add((((1i32) as u32)).wrapping_mul(44)), ((0i32) as u32), fp.wrapping_add(8), fp);
                    F::Flash_ClipSetAlpha(c, (p_view_1).wrapping_add((((1i32) as u32)).wrapping_mul(44)), fp, F32(0x3f800000));
                    bb_ = 6; continue 'fl1;
                }
                10 => {
                    if (((((l_chara_4) == ((1i32).wrapping_neg())) as i32)) != 0) { bb_ = 14; continue 'fl1; } else { bb_ = 15; continue 'fl1; }
                }
                11 => {
                    l_buf_5 = fp.wrapping_add(8);
                    F::Flash_FindLabel(c, (p_view_1).wrapping_add((((1i32) as u32)).wrapping_mul(44)), ((0i32) as u32), fp.wrapping_add(8), fp);
                    F::Flash_ClipSetAlpha(c, (p_view_1).wrapping_add((((1i32) as u32)).wrapping_mul(44)), fp, F32(0x3f800000));
                    bb_ = 3; continue 'fl1;
                }
                12 => {
                    bb_ = 9; continue 'fl1;
                }
                13 => {
                    bb_ = 12; continue 'fl1;
                }
                14 => {
                    bb_ = 16; continue 'fl1;
                }
                15 => {
                    bb_ = 9; continue 'fl1;
                }
                16 => {
                    F::Flash_FindLabel(c, (p_view_1).wrapping_add((((1i32) as u32)).wrapping_mul(44)), ((0i32) as u32), fp.wrapping_add(8), fp);
                    F::Flash_ClipSetAlpha(c, (p_view_1).wrapping_add((((1i32) as u32)).wrapping_mul(44)), fp, F32(0x3f000000));
                    bb_ = 3; continue 'fl1;
                }
                17 => {
                    bb_ = 15; continue 'fl1;
                }
                18 => {
                    bb_ = 16; continue 'fl1;
                }
                19 => {
                    F::Flash_FindLabel(c, (p_view_1).wrapping_add((((1i32) as u32)).wrapping_mul(44)), fp.wrapping_add(8), sym::menu_z_d__str56, fp);
                    F::TextBox_AttachLine(c, (p_view_1).wrapping_add((((1i32) as u32)).wrapping_mul(44)), fp, 0i32, 0i32, l_chara_4, (p_view_1).wrapping_add(1280));
                    F::Flash_FindLabel(c, (p_view_1).wrapping_add((((1i32) as u32)).wrapping_mul(44)), fp.wrapping_add(8), sym::menu_z_d__str57, fp);
                    F::TextBox_AttachLine(c, (p_view_1).wrapping_add((((1i32) as u32)).wrapping_mul(44)), fp, 0i32, 0i32, l_chara_4, (p_view_1).wrapping_add(1420));
                    F::Flash_FindLabel(c, (p_view_1).wrapping_add((((1i32) as u32)).wrapping_mul(44)), ((0i32) as u32), fp.wrapping_add(8), fp);
                    F::Flash_ClipSetAlpha(c, (p_view_1).wrapping_add((((1i32) as u32)).wrapping_mul(44)), fp, F32(0x3f800000));
                    bb_ = 6; continue 'fl1;
                }
                20 => {
                    bb_ = 19; continue 'fl1;
                }
                _ => unreachable!(),
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcPass_DrawRows", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_DrawList(c: &mut Ctx, mut p_pass_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_DrawList", &PROBE_N, &[(p_pass_1) as u32], false); }
    let r_ = 'f: {
    F::DcPass_SetListClip(c, (p_pass_1).wrapping_add(20));
    F::DcPass_DrawArrows(c, (p_pass_1).wrapping_add(20), (p_pass_1).wrapping_add(2272));
    F::DcPass_DrawScrollBar(c, (p_pass_1).wrapping_add(20), (p_pass_1).wrapping_add(2272));
    F::DcPass_DrawRows(c, (p_pass_1).wrapping_add(20), (p_pass_1).wrapping_add(2272));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcPass_DrawList", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_AnimGuide(c: &mut Ctx, mut p_pass_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_AnimGuide", &PROBE_N, &[(p_pass_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_flash_2: u32 = 0;
    let r_ = 'f: {
        l_flash_2 = ((p_pass_1).wrapping_add(20)).wrapping_add(((((c.m.r32((p_pass_1).wrapping_add(2252)) as i32)) as u32)).wrapping_mul(44));
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_z_d__str58, fp);
        F::FlashAnim_Blink(c, l_flash_2, fp, ((p_pass_1).wrapping_add(20)).wrapping_add(360), 0i32);
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_z_d__str59, fp);
        if (((((F::Voice_GetStat(c, 0i32)) != (5i32)) as i32)) != 0) {
            F::FlashAnim_Talk(c, l_flash_2, fp, ((p_pass_1).wrapping_add(20)).wrapping_add(364), 0i32);
        } else {
            F::FlashAnim_ShowNext2(c, l_flash_2, fp, 0i32);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcPass_AnimGuide", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_AnimBg(c: &mut Ctx, mut p_pass_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_AnimBg", &PROBE_N, &[(p_pass_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::Flash_FindLabel(c, ((p_pass_1).wrapping_add(20)).wrapping_add((((l_i_2) as u32)).wrapping_mul(44)), ((0i32) as u32), sym::menu_z_d__str60, fp);
                    F::FlashAnim_Scroll(c, ((p_pass_1).wrapping_add(20)).wrapping_add((((l_i_2) as u32)).wrapping_mul(44)), fp, fp.wrapping_add(8), ((p_pass_1).wrapping_add(20)).wrapping_add(2128), ((0i32) as u32), F32(0xbe1c09c0), F32(0x00000000));
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcPass_AnimBg", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_DrawNames(c: &mut Ctx, mut p_pass_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_DrawNames", &PROBE_N, &[(p_pass_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_flash_2: u32 = 0;
    let mut l_chara_3: i32 = 0;
    let r_ = 'f: {
        l_flash_2 = ((p_pass_1).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44));
        l_chara_3 = (c.m.r32((p_pass_1).wrapping_add(2340)) as i32);
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_z_d__str61, fp);
        F::TextBox_AttachLine(c, l_flash_2, fp, 0i32, 0i32, l_chara_3, ((p_pass_1).wrapping_add(20)).wrapping_add(1560));
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_z_d__str62, fp);
        F::TextBox_AttachLine(c, l_flash_2, fp, 0i32, 0i32, l_chara_3, ((p_pass_1).wrapping_add(20)).wrapping_add(1700));
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_z_d__str63, fp);
        F::TextBox_AttachLine(c, l_flash_2, fp, 0i32, 0i32, l_chara_3, ((p_pass_1).wrapping_add(20)).wrapping_add(1840));
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_z_d__str64, fp);
        F::TextBox_AttachLine(c, l_flash_2, fp, 0i32, 0i32, l_chara_3, ((p_pass_1).wrapping_add(20)).wrapping_add(1980));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcPass_DrawNames", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_IsPrevPressed(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_IsPrevPressed", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(384)) as i32)) == (0i32)) as i32)) != 0) && ((((c.m.r64(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((((16384i32) | (32768i32))) as u64)))) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "DcPass_IsPrevPressed", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_IsNextPressed(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_IsNextPressed", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(384)) as i32)) == (0i32)) as i32)) != 0) && ((((c.m.r64(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((((65536i32) | (131072i32))) as u64)))) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "DcPass_IsNextPressed", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_ResetBlink(c: &mut Ctx, mut p_view_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_ResetBlink", &PROBE_N, &[(p_view_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((F::Rand_Range(c, ((32i32) as u32))) as i32); let a_ = (p_view_1).wrapping_add(360); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcPass_ResetBlink", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_ResetText(c: &mut Ctx, mut p_text_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_ResetText", &PROBE_N, &[(p_text_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = (100i32).wrapping_neg(); let a_ = (p_text_1).wrapping_add(80); c.m.w32(a_, (v_) as u32); };
    { let v_ = 1i32; let a_ = (p_text_1).wrapping_add(88); c.m.w32(a_, (v_) as u32); };
    { let v_ = (100i32).wrapping_neg(); let a_ = (p_text_1).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
    F::DcPassText_Clear(c, p_text_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcPass_ResetText", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_Start(c: &mut Ctx, mut p_pass_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_Start", &PROBE_N, &[(p_pass_1) as u32], false); }
    let r_ = 'f: {
    F::DcPass_ResetBlink(c, (p_pass_1).wrapping_add(20));
    F::DcPass_ResetText(c, (p_pass_1).wrapping_add(2152));
    { let v_ = ((0i32) as u32); let a_ = (p_pass_1).wrapping_add(2256); c.m.w32(a_, v_); };
    { let v_ = 30i32; let a_ = (p_pass_1).wrapping_add(2396); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcPass_Start", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_Update(c: &mut Ctx, mut p_pass_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_Update", &PROBE_N, &[(p_pass_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                F::Flash_Advance(c, ((p_pass_1).wrapping_add(20)).wrapping_add((((l_i_2) as u32)).wrapping_mul(44)));
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcPass_Update", &PROBE_N, &[], true); }
    r_
}

pub fn menu_z_d__DcPass_Section(c: &mut Ctx, mut p_pass_1: u32, mut p_n_2: i32, mut p_host_3: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_z_d__DcPass_Section", &PROBE_N, &[(p_pass_1) as u32, (p_n_2) as u32, (p_host_3) as u32], false); }
    let r_ = 'f: {
    break 'f (c.m.r32((p_pass_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_pass_1).wrapping_add(4))).wrapping_add((((p_n_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_z_d__DcPass_Section", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_Init(c: &mut Ctx, mut p_pass_1: u32, mut p_section_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_Init", &PROBE_N, &[(p_pass_1) as u32, (p_section_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = (c.m.r32(sym::gMenuArc8)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc8)).wrapping_add((((p_section_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = p_pass_1; c.m.w32(a_, v_); };
        { let v_ = { let a0_ = c.m.r32(p_pass_1); F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (p_pass_1).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = F::menu_z_d__DcPass_Section(c, p_pass_1, 10i32, sym::menu_z_d__str65); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = c.m.r32(fp); let a_ = ((p_pass_1).wrapping_add(20)).wrapping_add(88); c.m.w32(a_, v_); };
        { let v_ = F::menu_z_d__DcPass_Section(c, p_pass_1, 15i32, sym::menu_z_d__str66); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::menu_z_d__DcPass_Section(c, p_pass_1, 15i32, sym::menu_z_d__str67); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::menu_z_d__DcPass_Section(c, p_pass_1, 16i32, sym::menu_z_d__str68); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::menu_z_d__DcPass_Section(c, p_pass_1, 16i32, sym::menu_z_d__str69); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((30i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((31i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((32i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::menu_z_d__DcPass_Section(c, p_pass_1, 11i32, sym::menu_z_d__str70); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::menu_z_d__DcPass_Section(c, p_pass_1, 11i32, sym::menu_z_d__str71); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((34i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((35i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((36i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::menu_z_d__DcPass_Section(c, p_pass_1, 1i32, sym::menu_z_d__str72); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((7i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((8i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((9i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::menu_z_d__DcPass_Section(c, p_pass_1, 2i32, sym::menu_z_d__str73); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(92)).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a1_ = F::menu_z_d__DcPass_Section(c, p_pass_1, 17i32, sym::menu_z_d__str74); F::Flash_Create(c, ((p_pass_1).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), a1_, ((p_pass_1).wrapping_add(20)).wrapping_add(92)) };
        F::Flash_Play(c, ((p_pass_1).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), 1i32);
        { let v_ = F::menu_z_d__DcPass_Section(c, p_pass_1, 3i32, sym::menu_z_d__str75); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((7i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((8i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((9i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((10i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((11i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((12i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((13i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::menu_z_d__DcPass_Section(c, p_pass_1, 5i32, sym::menu_z_d__str76); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::menu_z_d__DcPass_Section(c, p_pass_1, 4i32, sym::menu_z_d__str77); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((33i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (((p_pass_1).wrapping_add(20)).wrapping_add(212)).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a1_ = F::menu_z_d__DcPass_Section(c, p_pass_1, 18i32, sym::menu_z_d__str78); F::Flash_Create(c, ((p_pass_1).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44)), a1_, ((p_pass_1).wrapping_add(20)).wrapping_add(212)) };
        F::Flash_Play(c, ((p_pass_1).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44)), 1i32);
        { let v_ = F::menu_z_d__DcPass_Section(c, p_pass_1, 13i32, sym::menu_z_d__str79); let a_ = ((p_pass_1).wrapping_add(20)).wrapping_add(2120); c.m.w32(a_, v_); };
        { let v_ = F::menu_z_d__DcPass_Section(c, p_pass_1, 14i32, sym::menu_z_d__str80); let a_ = ((p_pass_1).wrapping_add(20)).wrapping_add(2124); c.m.w32(a_, v_); };
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < (3i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a1_ = c.m.r32(((p_pass_1).wrapping_add(20)).wrapping_add(2120)); F::TextBox_Init(c, (((p_pass_1).wrapping_add(20)).wrapping_add(440)).wrapping_add((((l_i_3) as u32)).wrapping_mul(140)), a1_, ((2i32) as u32)) };
                    F::TextBox_SetRect(c, (((p_pass_1).wrapping_add(20)).wrapping_add(440)).wrapping_add((((l_i_3) as u32)).wrapping_mul(140)), 175i32, 511i32, 106i32, 306i32);
                    F::TextBox_SetUnk80(c, (((p_pass_1).wrapping_add(20)).wrapping_add(440)).wrapping_add((((l_i_3) as u32)).wrapping_mul(140)), 1i32);
                    { let a1_ = c.m.r32(((p_pass_1).wrapping_add(20)).wrapping_add(2124)); F::TextBox_Init(c, (((p_pass_1).wrapping_add(20)).wrapping_add(860)).wrapping_add((((l_i_3) as u32)).wrapping_mul(140)), a1_, ((4i32) as u32)) };
                    F::TextBox_SetRect(c, (((p_pass_1).wrapping_add(20)).wrapping_add(860)).wrapping_add((((l_i_3) as u32)).wrapping_mul(140)), 175i32, 511i32, 106i32, 306i32);
                    F::TextBox_SetUnk80(c, (((p_pass_1).wrapping_add(20)).wrapping_add(860)).wrapping_add((((l_i_3) as u32)).wrapping_mul(140)), 1i32);
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        { let a1_ = c.m.r32(((p_pass_1).wrapping_add(20)).wrapping_add(2120)); F::TextBox_Init(c, ((p_pass_1).wrapping_add(20)).wrapping_add(1280), a1_, ((2i32) as u32)) };
        F::TextBox_SetRect(c, ((p_pass_1).wrapping_add(20)).wrapping_add(1280), 175i32, 511i32, 106i32, 306i32);
        F::TextBox_SetUnk80(c, ((p_pass_1).wrapping_add(20)).wrapping_add(1280), 1i32);
        { let a1_ = c.m.r32(((p_pass_1).wrapping_add(20)).wrapping_add(2124)); F::TextBox_Init(c, ((p_pass_1).wrapping_add(20)).wrapping_add(1420), a1_, ((4i32) as u32)) };
        F::TextBox_SetRect(c, ((p_pass_1).wrapping_add(20)).wrapping_add(1420), 175i32, 511i32, 106i32, 306i32);
        F::TextBox_SetUnk80(c, ((p_pass_1).wrapping_add(20)).wrapping_add(1420), 1i32);
        { let a1_ = c.m.r32(((p_pass_1).wrapping_add(20)).wrapping_add(2120)); F::TextBox_Init(c, ((p_pass_1).wrapping_add(20)).wrapping_add(1560), a1_, ((1i32) as u32)) };
        { let a1_ = c.m.r32(((p_pass_1).wrapping_add(20)).wrapping_add(2124)); F::TextBox_Init(c, ((p_pass_1).wrapping_add(20)).wrapping_add(1700), a1_, ((1i32) as u32)) };
        { let a1_ = c.m.r32(((p_pass_1).wrapping_add(20)).wrapping_add(2120)); F::TextBox_Init(c, ((p_pass_1).wrapping_add(20)).wrapping_add(1840), a1_, ((2i32) as u32)) };
        { let a1_ = c.m.r32(((p_pass_1).wrapping_add(20)).wrapping_add(2124)); F::TextBox_Init(c, ((p_pass_1).wrapping_add(20)).wrapping_add(1980), a1_, ((2i32) as u32)) };
        { let v_ = F::Heap_Alloc(c, 92160i32, ((64i32) as u32), 0i32, 2i32); let a_ = (p_pass_1).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = F::Heap_Alloc(c, 133120i32, ((32i32) as u32), 0i32, 2i32); let a_ = (p_pass_1).wrapping_add(12); c.m.w32(a_, v_); };
        { let _ = { let a0_ = ((c.m.r32((p_pass_1).wrapping_add(2340)) as i32)).wrapping_add(761i32); let a1_ = c.m.r32((p_pass_1).wrapping_add(8)); F::File_LoadSync(c, a0_, a1_, 92160i32) }; };
        { let v_ = F::menu_z_d__DcPass_Section(c, p_pass_1, 9i32, sym::menu_z_d__str81); let a_ = ((p_pass_1).wrapping_add(20)).wrapping_add(376); c.m.w32(a_, v_); };
        { let v_ = F::menu_z_d__DcPass_Section(c, p_pass_1, 8i32, sym::menu_z_d__str82); let a_ = ((p_pass_1).wrapping_add(20)).wrapping_add(368); c.m.w32(a_, v_); };
        { let a0_ = F::menu_z_d__DcPass_Section(c, p_pass_1, 6i32, sym::menu_z_d__str83); let a1_ = c.m.r32(((p_pass_1).wrapping_add(20)).wrapping_add(368)); let _ = ((((p_pass_1).wrapping_add(20)).wrapping_add(380)) as i32); F::MsgWin_Init(c, a0_, a1_, 0i32) };
        { let v_ = F::menu_z_d__DcPass_Section(c, p_pass_1, 21i32, sym::menu_z_d__str84); let a_ = ((p_pass_1).wrapping_add(20)).wrapping_add(372); c.m.w32(a_, v_); };
        { let a0_ = F::menu_z_d__DcPass_Section(c, p_pass_1, 7i32, sym::menu_z_d__str85); let a1_ = c.m.r32(((p_pass_1).wrapping_add(20)).wrapping_add(372)); F::Dialog_Init(c, a0_, a1_, 0i32) };
        { let v_ = F::menu_z_d__DcPass_Section(c, p_pass_1, 12i32, sym::menu_z_d__str86); let a_ = (p_pass_1).wrapping_add(2408); c.m.w32(a_, v_); };
        { let a0_ = F::menu_z_d__DcPass_Section(c, p_pass_1, 19i32, sym::menu_z_d__str87); F::PassChk_Init(c, a0_) };
        { let v_ = (F::menu_z_d__DcPass_Section(c, p_pass_1, 20i32, sym::menu_z_d__str88)).wrapping_add(((16i32) as u32)); let a_ = (p_pass_1).wrapping_add(2412); c.m.w32(a_, v_); };
        { let v_ = { let a_ = F::menu_z_d__DcPass_Section(c, p_pass_1, 20i32, sym::menu_z_d__str89); (c.m.r32(a_) as i32) }; let a_ = (p_pass_1).wrapping_add(2416); c.m.w32(a_, (v_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcPass_Init", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_Term(c: &mut Ctx, mut p_pass_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_Term", &PROBE_N, &[(p_pass_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    F::MsgWin_Term(c);
    F::Dialog_Term(c);
    F::PassChk_Term(c);
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                F::Flash_Destroy(c, ((p_pass_1).wrapping_add(20)).wrapping_add((((l_i_2) as u32)).wrapping_mul(44)));
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    if (((((c.m.r32((p_pass_1).wrapping_add(4))) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32((p_pass_1).wrapping_add(4)); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (p_pass_1).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32((p_pass_1).wrapping_add(8))) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32((p_pass_1).wrapping_add(8)); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (p_pass_1).wrapping_add(8); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32((p_pass_1).wrapping_add(12))) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32((p_pass_1).wrapping_add(12)); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (p_pass_1).wrapping_add(12); c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcPass_Term", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_Draw(c: &mut Ctx, mut p_pass_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_Draw", &PROBE_N, &[(p_pass_1) as u32], false); }
    let mut l_flash0_2: u32 = 0;
    let r_ = 'f: {
    l_flash0_2 = ((p_pass_1).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
    if ((((!(((((c.m.r32((p_pass_1).wrapping_add(2404)) as i32)) & (4i32))) != 0)) as i32)) != 0) {
        F::DcPass_LightKey(c, (p_pass_1).wrapping_add(20), (p_pass_1).wrapping_add(2260), 1i32);
        F::DcPass_LightRow(c, (p_pass_1).wrapping_add(20), (p_pass_1).wrapping_add(2272), 1i32);
        F::Flash_GotoLabel(c, l_flash0_2, sym::menu_z_d__str90, ((((1i32) as i64 as u64)) as u8));
        { let r_ = 4i32; let a_ = (p_pass_1).wrapping_add(2404); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    F::DcPass_DrawKeyboard(c, p_pass_1);
    F::DcPass_DrawTextLine(c, p_pass_1);
    F::menu_z_d__DcPass_SetFaceAlpha(c, p_pass_1);
    F::DcPass_DrawStatus(c, (p_pass_1).wrapping_add(20), (p_pass_1).wrapping_add(2340));
    F::DcPass_DrawList(c, p_pass_1);
    F::DcPass_DrawNames(c, p_pass_1);
    F::DcPass_AnimGuide(c, p_pass_1);
    F::DcPass_AnimBg(c, p_pass_1);
    { let a0_ = c.m.r32(((p_pass_1).wrapping_add(20)).wrapping_add(88)); F::Sprite_DrawPicture(c, a0_, 0i32, 0i32, 128i32) };
    { let a0_ = ((p_pass_1).wrapping_add(20)).wrapping_add(((((c.m.r32((p_pass_1).wrapping_add(2252)) as i32)) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
    { let a2_ = (c.m.r32((p_pass_1).wrapping_add(2248)) as i32); F::MsgWin_Draw(c, 0i32, 0i32, a2_) };
    F::Dialog_Draw(c, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcPass_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_ConfirmRow(c: &mut Ctx, mut p_pass_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_ConfirmRow", &PROBE_N, &[(p_pass_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(272) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 272);
    let mut l_flash_2: u32 = 0;
    let r_ = 'f: {
        l_flash_2 = ((p_pass_1).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44));
        { let _ = { let a2_ = ((c.m.r32(((p_pass_1).wrapping_add(2272)).wrapping_add(4)) as i32)).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_z_d__str91, &[((a2_) as i64 as u64)]) }; };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_z_d__str92);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcPass_ConfirmRow", &PROBE_N, &[], true); }
    r_
}

pub fn menu_z_d__DcPass_ClampCursor(c: &mut Ctx, mut p_list_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_z_d__DcPass_ClampCursor", &PROBE_N, &[(p_list_1) as u32], false); }
    let r_ = 'f: {
    if ((((((c.m.r32((p_list_1).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (p_list_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        break 'f 1i32;
    }
    if ((((((c.m.r32((p_list_1).wrapping_add(4)) as i32)) > ((3i32).wrapping_sub(1i32))) as i32)) != 0) {
        { let v_ = (3i32).wrapping_sub(1i32); let a_ = (p_list_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_z_d__DcPass_ClampCursor", &PROBE_N, &[], true); }
    r_
}

pub fn menu_z_d__DcPass_GetSlot(c: &mut Ctx, mut p_list_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_z_d__DcPass_GetSlot", &PROBE_N, &[(p_list_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((c.m.r32(p_list_1) as i32)).wrapping_add((c.m.r32((p_list_1).wrapping_add(4)) as i32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_z_d__DcPass_GetSlot", &PROBE_N, &[], true); }
    r_
}

pub fn menu_z_d__DcPass_SwitchMovie(c: &mut Ctx, mut p_pass_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_z_d__DcPass_SwitchMovie", &PROBE_N, &[(p_pass_1) as u32], false); }
    let r_ = 'f: {
    if ((((c.m.r32((((p_pass_1).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12))) & (((1i32) as u32)))) != 0) {
        { let v_ = 1i32; let a_ = (p_pass_1).wrapping_add(2252); c.m.w32(a_, (v_) as u32); };
        F::Flash_GotoLabel(c, ((p_pass_1).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44)), sym::menu_z_d__str93, ((((1i32) as i64 as u64)) as u8));
    } else {
        if ((((c.m.r32((((p_pass_1).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add(12))) & (((1i32) as u32)))) != 0) {
            { let v_ = 0i32; let a_ = (p_pass_1).wrapping_add(2252); c.m.w32(a_, (v_) as u32); };
            F::Flash_GotoLabel(c, ((p_pass_1).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), sym::menu_z_d__str94, ((((1i32) as i64 as u64)) as u8));
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_z_d__DcPass_SwitchMovie", &PROBE_N, &[], true); }
    r_
}

pub fn menu_z_d__DcPass_Leave(c: &mut Ctx, mut p_pass_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_z_d__DcPass_Leave", &PROBE_N, &[(p_pass_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = (p_pass_1).wrapping_add(2244); c.m.w32(a_, (v_) as u32); };
    { let r_ = 1i32; let a_ = (p_pass_1).wrapping_add(2404); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    F::Flash_GotoLabel(c, ((p_pass_1).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), sym::menu_z_d__str95, ((((1i32) as i64 as u64)) as u8));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_z_d__DcPass_Leave", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_Input(c: &mut Ctx, mut p_pass_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_Input", &PROBE_N, &[(p_pass_1) as u32], false); }
    let mut l_ret_2: i32 = 0;
    let mut sv_3: u32 = 0;
    let mut sv_4: i32 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
    'fl1: loop {
        match bb_ {
            0 => {
                if ((((!((((c.m.r32((((p_pass_1).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8))) & (((2i32) as u32)))) != 0)) as i32)) != 0) { bb_ = 1; continue 'fl1; } else { bb_ = 2; continue 'fl1; }
            }
            1 => {
                break 'f;
            }
            2 => {
                if (((((c.m.r32((p_pass_1).wrapping_add(2404)) as i32)) & (1i32))) != 0) { bb_ = 4; continue 'fl1; } else { bb_ = 5; continue 'fl1; }
            }
            3 => {
                bb_ = 2; continue 'fl1;
            }
            4 => {
                break 'f;
            }
            5 => {
                if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) { bb_ = 7; continue 'fl1; } else { bb_ = 8; continue 'fl1; }
            }
            6 => {
                bb_ = 5; continue 'fl1;
            }
            7 => {
                break 'f;
            }
            8 => {
                F::menu_z_d__DcPass_SwitchMovie(c, p_pass_1);
                sv_3 = c.m.r32((p_pass_1).wrapping_add(2256));
                bb_ = match sv_3 { 0u32 => 12, 1u32 => 97, 2u32 => 103, 3u32 => 111, 4u32 => 140, 5u32 => 151, 6u32 => 162, _ => 10, }; continue 'fl1;
            }
            9 => {
                bb_ = 8; continue 'fl1;
            }
            10 => {
                break 'fl1;
            }
            11 => {
                bb_ = 12; continue 'fl1;
            }
            12 => {
                if ((((((c.m.r32((p_pass_1).wrapping_add(2252)) as i32)) == (1i32)) as i32)) != 0) { bb_ = 13; continue 'fl1; } else { bb_ = 14; continue 'fl1; }
            }
            13 => {
                bb_ = 10; continue 'fl1;
            }
            14 => {
                if ((F::DcPass_IsPrevPressed(c)) != 0) { bb_ = 16; continue 'fl1; } else { bb_ = 18; continue 'fl1; }
            }
            15 => {
                bb_ = 14; continue 'fl1;
            }
            16 => {
                if ((((((c.m.r32(((p_pass_1).wrapping_add(2152)).wrapping_add(72)) as i32)) > (0i32)) as i32)) != 0) { bb_ = 19; continue 'fl1; } else { bb_ = 20; continue 'fl1; }
            }
            17 => {
                bb_ = 10; continue 'fl1;
            }
            18 => {
                if ((F::DcPass_IsNextPressed(c)) != 0) { bb_ = 21; continue 'fl1; } else { bb_ = 23; continue 'fl1; }
            }
            19 => {
                { let a_ = ((p_pass_1).wrapping_add(2152)).wrapping_add(72); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                bb_ = 20; continue 'fl1;
            }
            20 => {
                bb_ = 17; continue 'fl1;
            }
            21 => {
                if ((((((c.m.r32(((p_pass_1).wrapping_add(2152)).wrapping_add(72)) as i32)) < (33i32)) as i32)) != 0) { bb_ = 24; continue 'fl1; } else { bb_ = 25; continue 'fl1; }
            }
            22 => {
                bb_ = 17; continue 'fl1;
            }
            23 => {
                if ((((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(384)) as i32)) == (1i32)) as i32)) != 0) && ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(392))) & (((2048i32) as u32)))) != 0)) as i32)) != 0) { bb_ = 26; continue 'fl1; } else { bb_ = 28; continue 'fl1; }
            }
            24 => {
                { let a_ = ((p_pass_1).wrapping_add(2152)).wrapping_add(72); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                bb_ = 25; continue 'fl1;
            }
            25 => {
                bb_ = 22; continue 'fl1;
            }
            26 => {
                bb_ = 10; continue 'fl1;
            }
            27 => {
                bb_ = 22; continue 'fl1;
            }
            28 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) { bb_ = 30; continue 'fl1; } else { bb_ = 32; continue 'fl1; }
            }
            29 => {
                bb_ = 27; continue 'fl1;
            }
            30 => {
                F::DcPass_LightKey(c, (p_pass_1).wrapping_add(20), (p_pass_1).wrapping_add(2260), 0i32);
                bb_ = 33; continue 'fl1;
            }
            31 => {
                bb_ = 27; continue 'fl1;
            }
            32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) { bb_ = 36; continue 'fl1; } else { bb_ = 38; continue 'fl1; }
            }
            33 => {
                { let a_ = ((p_pass_1).wrapping_add(2260)).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                bb_ = 34; continue 'fl1;
            }
            34 => {
                if (({ let a1_ = ((c.m.r32(((p_pass_1).wrapping_add(2260)).wrapping_add(8)) as i32)).wrapping_add(1i32); F::DcPass_IsSameKeyCol(c, (p_pass_1).wrapping_add(2260), a1_) }) != 0) { bb_ = 33; continue 'fl1; } else { bb_ = 35; continue 'fl1; }
            }
            35 => {
                F::DcPass_LightKey(c, (p_pass_1).wrapping_add(20), (p_pass_1).wrapping_add(2260), 1i32);
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                bb_ = 31; continue 'fl1;
            }
            36 => {
                F::DcPass_LightKey(c, (p_pass_1).wrapping_add(20), (p_pass_1).wrapping_add(2260), 0i32);
                bb_ = 39; continue 'fl1;
            }
            37 => {
                bb_ = 31; continue 'fl1;
            }
            38 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) { bb_ = 42; continue 'fl1; } else { bb_ = 44; continue 'fl1; }
            }
            39 => {
                { let a_ = ((p_pass_1).wrapping_add(2260)).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                bb_ = 40; continue 'fl1;
            }
            40 => {
                if (({ let a1_ = ((c.m.r32(((p_pass_1).wrapping_add(2260)).wrapping_add(8)) as i32)).wrapping_sub(1i32); F::DcPass_IsSameKeyCol(c, (p_pass_1).wrapping_add(2260), a1_) }) != 0) { bb_ = 39; continue 'fl1; } else { bb_ = 41; continue 'fl1; }
            }
            41 => {
                F::DcPass_LightKey(c, (p_pass_1).wrapping_add(20), (p_pass_1).wrapping_add(2260), 1i32);
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                bb_ = 37; continue 'fl1;
            }
            42 => {
                F::DcPass_LightKey(c, (p_pass_1).wrapping_add(20), (p_pass_1).wrapping_add(2260), 0i32);
                bb_ = 45; continue 'fl1;
            }
            43 => {
                bb_ = 37; continue 'fl1;
            }
            44 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) { bb_ = 48; continue 'fl1; } else { bb_ = 50; continue 'fl1; }
            }
            45 => {
                { let a_ = ((p_pass_1).wrapping_add(2260)).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                bb_ = 46; continue 'fl1;
            }
            46 => {
                if (({ let a1_ = ((c.m.r32(((p_pass_1).wrapping_add(2260)).wrapping_add(4)) as i32)).wrapping_add(1i32); F::DcPass_IsSameKeyRow(c, (p_pass_1).wrapping_add(2260), a1_) }) != 0) { bb_ = 45; continue 'fl1; } else { bb_ = 47; continue 'fl1; }
            }
            47 => {
                F::DcPass_LightKey(c, (p_pass_1).wrapping_add(20), (p_pass_1).wrapping_add(2260), 1i32);
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                bb_ = 43; continue 'fl1;
            }
            48 => {
                F::DcPass_LightKey(c, (p_pass_1).wrapping_add(20), (p_pass_1).wrapping_add(2260), 0i32);
                bb_ = 51; continue 'fl1;
            }
            49 => {
                bb_ = 43; continue 'fl1;
            }
            50 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((256i32) as u32)))) != 0) { bb_ = 54; continue 'fl1; } else { bb_ = 56; continue 'fl1; }
            }
            51 => {
                { let a_ = ((p_pass_1).wrapping_add(2260)).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                bb_ = 52; continue 'fl1;
            }
            52 => {
                if (({ let a1_ = ((c.m.r32(((p_pass_1).wrapping_add(2260)).wrapping_add(4)) as i32)).wrapping_sub(1i32); F::DcPass_IsSameKeyRow(c, (p_pass_1).wrapping_add(2260), a1_) }) != 0) { bb_ = 51; continue 'fl1; } else { bb_ = 53; continue 'fl1; }
            }
            53 => {
                F::DcPass_LightKey(c, (p_pass_1).wrapping_add(20), (p_pass_1).wrapping_add(2260), 1i32);
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                bb_ = 49; continue 'fl1;
            }
            54 => {
                { let a_ = (p_pass_1).wrapping_add(2260); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::DcPass_WrapPos(c, (p_pass_1).wrapping_add(2260));
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                bb_ = 55; continue 'fl1;
            }
            55 => {
                bb_ = 49; continue 'fl1;
            }
            56 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((4096i32) as u32)))) != 0) { bb_ = 57; continue 'fl1; } else { bb_ = 59; continue 'fl1; }
            }
            57 => {
                F::DcPass_LightKey(c, (p_pass_1).wrapping_add(20), (p_pass_1).wrapping_add(2260), 0i32);
                F::DcPass_GotoButton(c, 2i32, (p_pass_1).wrapping_add(2260));
                F::DcPass_LightKey(c, (p_pass_1).wrapping_add(20), (p_pass_1).wrapping_add(2260), 1i32);
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                bb_ = 58; continue 'fl1;
            }
            58 => {
                bb_ = 55; continue 'fl1;
            }
            59 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) { bb_ = 60; continue 'fl1; } else { bb_ = 62; continue 'fl1; }
            }
            60 => {
                F::DcPass_FlashKey(c, (p_pass_1).wrapping_add(20), (p_pass_1).wrapping_add(2260));
                sv_4 = ((F::DcPass_GetKey(c, (p_pass_1).wrapping_add(2260))) as i32);
                bb_ = match sv_4 { 1i32 => 65, 2i32 => 67, 3i32 => 72, 0i32 => 75, 5i32 => 77, 6i32 => 81, _ => 85, }; continue 'fl1;
            }
            61 => {
                bb_ = 58; continue 'fl1;
            }
            62 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) { bb_ = 92; continue 'fl1; } else { bb_ = 93; continue 'fl1; }
            }
            63 => {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                bb_ = 61; continue 'fl1;
            }
            64 => {
                bb_ = 65; continue 'fl1;
            }
            65 => {
                { let a_ = (p_pass_1).wrapping_add(2260); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::DcPass_WrapPos(c, (p_pass_1).wrapping_add(2260));
                bb_ = 63; continue 'fl1;
            }
            66 => {
                bb_ = 67; continue 'fl1;
            }
            67 => {
                if ((F::DcPass_Decode(c, p_pass_1)) != 0) { bb_ = 68; continue 'fl1; } else { bb_ = 70; continue 'fl1; }
            }
            68 => {
                { let v_ = ((2i32) as u32); let a_ = (p_pass_1).wrapping_add(2256); c.m.w32(a_, v_); };
                F::Flash_GotoLabel(c, ((p_pass_1).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), sym::menu_z_d__str96, ((((1i32) as i64 as u64)) as u8));
                bb_ = 69; continue 'fl1;
            }
            69 => {
                bb_ = 63; continue 'fl1;
            }
            70 => {
                F::menu_z_d__DcPass_Say(c, p_pass_1, 19i32);
                { let v_ = ((1i32) as u32); let a_ = (p_pass_1).wrapping_add(2256); c.m.w32(a_, v_); };
                F::Flash_GotoLabel(c, ((p_pass_1).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), sym::menu_z_d__str97, ((((1i32) as i64 as u64)) as u8));
                F::MsgWin_Open(c);
                bb_ = 69; continue 'fl1;
            }
            71 => {
                bb_ = 72; continue 'fl1;
            }
            72 => {
                if ((F::DcPassText_Delete(c, (p_pass_1).wrapping_add(2152))) != 0) { bb_ = 73; continue 'fl1; } else { bb_ = 74; continue 'fl1; }
            }
            73 => {
                bb_ = 75; continue 'fl1;
            }
            74 => {
                bb_ = 63; continue 'fl1;
            }
            75 => {
                F::menu_z_d__DcPass_Leave(c, p_pass_1);
                bb_ = 74; continue 'fl1;
            }
            76 => {
                bb_ = 77; continue 'fl1;
            }
            77 => {
                if ((((((c.m.r32(((p_pass_1).wrapping_add(2152)).wrapping_add(72)) as i32)) > (0i32)) as i32)) != 0) { bb_ = 78; continue 'fl1; } else { bb_ = 79; continue 'fl1; }
            }
            78 => {
                { let a_ = ((p_pass_1).wrapping_add(2152)).wrapping_add(72); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                bb_ = 79; continue 'fl1;
            }
            79 => {
                bb_ = 63; continue 'fl1;
            }
            80 => {
                bb_ = 81; continue 'fl1;
            }
            81 => {
                if ((((((c.m.r32(((p_pass_1).wrapping_add(2152)).wrapping_add(72)) as i32)) < (33i32)) as i32)) != 0) { bb_ = 82; continue 'fl1; } else { bb_ = 83; continue 'fl1; }
            }
            82 => {
                { let a_ = ((p_pass_1).wrapping_add(2152)).wrapping_add(72); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                bb_ = 83; continue 'fl1;
            }
            83 => {
                bb_ = 63; continue 'fl1;
            }
            84 => {
                bb_ = 85; continue 'fl1;
            }
            85 => {
                if ((((((c.m.r32(((p_pass_1).wrapping_add(2152)).wrapping_add(72)) as i32)) == (33i32)) as i32)) != 0) { bb_ = 86; continue 'fl1; } else { bb_ = 88; continue 'fl1; }
            }
            86 => {
                F::DcPassText_Put(c, (p_pass_1).wrapping_add(2260), (p_pass_1).wrapping_add(2152));
                if ((F::DcPassText_IsFull(c, (p_pass_1).wrapping_add(2152))) != 0) { bb_ = 89; continue 'fl1; } else { bb_ = 90; continue 'fl1; }
            }
            87 => {
                bb_ = 63; continue 'fl1;
            }
            88 => {
                F::DcPassText_Put(c, (p_pass_1).wrapping_add(2260), (p_pass_1).wrapping_add(2152));
                bb_ = 87; continue 'fl1;
            }
            89 => {
                F::DcPass_LightKey(c, (p_pass_1).wrapping_add(20), (p_pass_1).wrapping_add(2260), 0i32);
                F::DcPass_GotoButton(c, 2i32, (p_pass_1).wrapping_add(2260));
                F::DcPass_LightKey(c, (p_pass_1).wrapping_add(20), (p_pass_1).wrapping_add(2260), 1i32);
                bb_ = 90; continue 'fl1;
            }
            90 => {
                bb_ = 87; continue 'fl1;
            }
            91 => {
                bb_ = 63; continue 'fl1;
            }
            92 => {
                if ((F::DcPassText_Delete(c, (p_pass_1).wrapping_add(2152))) != 0) { bb_ = 94; continue 'fl1; } else { bb_ = 95; continue 'fl1; }
            }
            93 => {
                bb_ = 61; continue 'fl1;
            }
            94 => {
                F::menu_z_d__DcPass_Leave(c, p_pass_1);
                bb_ = 95; continue 'fl1;
            }
            95 => {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                bb_ = 93; continue 'fl1;
            }
            96 => {
                bb_ = 97; continue 'fl1;
            }
            97 => {
                if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) || (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0)) as i32)) != 0) { bb_ = 98; continue 'fl1; } else { bb_ = 99; continue 'fl1; }
            }
            98 => {
                { let v_ = ((0i32) as u32); let a_ = (p_pass_1).wrapping_add(2256); c.m.w32(a_, v_); };
                F::Flash_GotoLabel(c, ((p_pass_1).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), sym::menu_z_d__str98, ((((1i32) as i64 as u64)) as u8));
                F::MsgWin_Close(c);
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) { bb_ = 100; continue 'fl1; } else { bb_ = 101; continue 'fl1; }
            }
            99 => {
                bb_ = 10; continue 'fl1;
            }
            100 => {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                F::Voice_StopWithLip(c);
                bb_ = 101; continue 'fl1;
            }
            101 => {
                bb_ = 99; continue 'fl1;
            }
            102 => {
                bb_ = 103; continue 'fl1;
            }
            103 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) { bb_ = 104; continue 'fl1; } else { bb_ = 105; continue 'fl1; }
            }
            104 => {
                if ((((((c.m.r32((p_pass_1).wrapping_add(2252)) as i32)) == (1i32)) as i32)) != 0) { bb_ = 106; continue 'fl1; } else { bb_ = 107; continue 'fl1; }
            }
            105 => {
                bb_ = 10; continue 'fl1;
            }
            106 => {
                { let v_ = ((3i32) as u32); let a_ = (p_pass_1).wrapping_add(2256); c.m.w32(a_, v_); };
                F::Flash_GotoLabel(c, ((p_pass_1).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44)), sym::menu_z_d__str99, ((((1i32) as i64 as u64)) as u8));
                F::menu_z_d__DcPass_Say(c, p_pass_1, 20i32);
                F::MsgWin_Open(c);
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) { bb_ = 108; continue 'fl1; } else { bb_ = 109; continue 'fl1; }
            }
            107 => {
                bb_ = 105; continue 'fl1;
            }
            108 => {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                bb_ = 109; continue 'fl1;
            }
            109 => {
                bb_ = 107; continue 'fl1;
            }
            110 => {
                bb_ = 111; continue 'fl1;
            }
            111 => {
                if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) && (((((F::menu_z_d__DcPass_GetSlot(c, (p_pass_1).wrapping_add(2272))) != (0i32)) as i32)) != 0)) as i32)) != 0) { bb_ = 112; continue 'fl1; } else { bb_ = 114; continue 'fl1; }
            }
            112 => {
                F::DcPass_LightRow(c, (p_pass_1).wrapping_add(20), (p_pass_1).wrapping_add(2272), 0i32);
                { let a_ = ((p_pass_1).wrapping_add(2272)).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                if ((F::menu_z_d__DcPass_ClampCursor(c, (p_pass_1).wrapping_add(2272))) != 0) { bb_ = 115; continue 'fl1; } else { bb_ = 116; continue 'fl1; }
            }
            113 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) { bb_ = 134; continue 'fl1; } else { bb_ = 135; continue 'fl1; }
            }
            114 => {
                if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) && (((((F::menu_z_d__DcPass_GetSlot(c, (p_pass_1).wrapping_add(2272))) != ((14i32).wrapping_sub(1i32))) as i32)) != 0)) as i32)) != 0) { bb_ = 119; continue 'fl1; } else { bb_ = 121; continue 'fl1; }
            }
            115 => {
                { let a_ = (p_pass_1).wrapping_add(2272); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                if ((F::DcPass_ClampTop(c, (p_pass_1).wrapping_add(2272))) != 0) { bb_ = 117; continue 'fl1; } else { bb_ = 118; continue 'fl1; }
            }
            116 => {
                F::DcPass_LightRow(c, (p_pass_1).wrapping_add(20), (p_pass_1).wrapping_add(2272), 1i32);
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                bb_ = 113; continue 'fl1;
            }
            117 => {
                F::Flash_GotoLabel(c, ((p_pass_1).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44)), sym::menu_z_d__str100, ((((1i32) as i64 as u64)) as u8));
                { let v_ = ((c.m.r32((p_pass_1).wrapping_add(2272)) as i32)).wrapping_add(3i32); let a_ = ((p_pass_1).wrapping_add(2272)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                bb_ = 118; continue 'fl1;
            }
            118 => {
                bb_ = 116; continue 'fl1;
            }
            119 => {
                F::DcPass_LightRow(c, (p_pass_1).wrapping_add(20), (p_pass_1).wrapping_add(2272), 0i32);
                { let a_ = ((p_pass_1).wrapping_add(2272)).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                if ((F::menu_z_d__DcPass_ClampCursor(c, (p_pass_1).wrapping_add(2272))) != 0) { bb_ = 122; continue 'fl1; } else { bb_ = 123; continue 'fl1; }
            }
            120 => {
                bb_ = 113; continue 'fl1;
            }
            121 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) { bb_ = 126; continue 'fl1; } else { bb_ = 128; continue 'fl1; }
            }
            122 => {
                { let a_ = (p_pass_1).wrapping_add(2272); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                if ((F::DcPass_ClampTop(c, (p_pass_1).wrapping_add(2272))) != 0) { bb_ = 124; continue 'fl1; } else { bb_ = 125; continue 'fl1; }
            }
            123 => {
                F::DcPass_LightRow(c, (p_pass_1).wrapping_add(20), (p_pass_1).wrapping_add(2272), 1i32);
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                bb_ = 120; continue 'fl1;
            }
            124 => {
                F::Flash_GotoLabel(c, ((p_pass_1).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44)), sym::menu_z_d__str101, ((((1i32) as i64 as u64)) as u8));
                { let v_ = ((c.m.r32((p_pass_1).wrapping_add(2272)) as i32)).wrapping_sub(1i32); let a_ = ((p_pass_1).wrapping_add(2272)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                bb_ = 125; continue 'fl1;
            }
            125 => {
                bb_ = 123; continue 'fl1;
            }
            126 => {
                { let v_ = ((4i32) as u32); let a_ = (p_pass_1).wrapping_add(2256); c.m.w32(a_, v_); };
                F::Dialog_SetMsg(c, 1i32);
                F::Dialog_Start(c, 0i32);
                F::Dialog_SetCursor(c, 1i32);
                F::Dialog_SetChoices(c, 1i32);
                F::Voice_StopWithLip(c);
                bb_ = 127; continue 'fl1;
            }
            127 => {
                bb_ = 120; continue 'fl1;
            }
            128 => {
                if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1i32) as u32)))) != 0) && (((((((c.m.r32((p_pass_1).wrapping_add(2272)) as i32)).wrapping_add(2i32)) >= (3i32)) as i32)) != 0)) as i32)) != 0) { bb_ = 129; continue 'fl1; } else { bb_ = 131; continue 'fl1; }
            }
            129 => {
                { let r_ = 3i32; let a_ = (p_pass_1).wrapping_add(2272); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                { let _ = F::DcPass_ClampTop(c, (p_pass_1).wrapping_add(2272)); };
                F::Flash_GotoLabel(c, ((p_pass_1).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44)), sym::menu_z_d__str102, ((((1i32) as i64 as u64)) as u8));
                { let v_ = ((c.m.r32((p_pass_1).wrapping_add(2272)) as i32)).wrapping_add(3i32); let a_ = ((p_pass_1).wrapping_add(2272)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                bb_ = 130; continue 'fl1;
            }
            130 => {
                bb_ = 127; continue 'fl1;
            }
            131 => {
                if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((2i32) as u32)))) != 0) && ((((((c.m.r32((p_pass_1).wrapping_add(2272)) as i32)) != ((14i32).wrapping_sub(3i32))) as i32)) != 0)) as i32)) != 0) { bb_ = 132; continue 'fl1; } else { bb_ = 133; continue 'fl1; }
            }
            132 => {
                { let r_ = 3i32; let a_ = (p_pass_1).wrapping_add(2272); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                { let _ = F::DcPass_ClampTop(c, (p_pass_1).wrapping_add(2272)); };
                F::Flash_GotoLabel(c, ((p_pass_1).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44)), sym::menu_z_d__str103, ((((1i32) as i64 as u64)) as u8));
                { let v_ = ((c.m.r32((p_pass_1).wrapping_add(2272)) as i32)).wrapping_sub(1i32); let a_ = ((p_pass_1).wrapping_add(2272)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                bb_ = 133; continue 'fl1;
            }
            133 => {
                bb_ = 130; continue 'fl1;
            }
            134 => {
                F::DcPass_ConfirmRow(c, p_pass_1);
                if ((((({ let a_ = (((p_pass_1).wrapping_add(2272)).wrapping_add(12)).wrapping_add((((F::menu_z_d__DcPass_GetSlot(c, (p_pass_1).wrapping_add(2272))) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == ((1i32).wrapping_neg())) as i32)) != 0) { bb_ = 136; continue 'fl1; } else { bb_ = 138; continue 'fl1; }
            }
            135 => {
                bb_ = 10; continue 'fl1;
            }
            136 => {
                { let v_ = ((6i32) as u32); let a_ = (p_pass_1).wrapping_add(2256); c.m.w32(a_, v_); };
                F::DcPass_StoreRec(c, (p_pass_1).wrapping_add(2340), (p_pass_1).wrapping_add(2272));
                F::menu_z_d__DcPass_Say(c, p_pass_1, 21i32);
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                bb_ = 137; continue 'fl1;
            }
            137 => {
                bb_ = 135; continue 'fl1;
            }
            138 => {
                { let v_ = ((5i32) as u32); let a_ = (p_pass_1).wrapping_add(2256); c.m.w32(a_, v_); };
                F::Dialog_SetMsg(c, 0i32);
                F::Dialog_Start(c, 0i32);
                F::Dialog_SetCursor(c, 1i32);
                F::Dialog_SetChoices(c, 1i32);
                bb_ = 137; continue 'fl1;
            }
            139 => {
                bb_ = 140; continue 'fl1;
            }
            140 => {
                if ((F::Dialog_IsClosed(c)) != 0) { bb_ = 141; continue 'fl1; } else { bb_ = 142; continue 'fl1; }
            }
            141 => {
                { let v_ = c.m.r32((p_pass_1).wrapping_add(2420)); let a_ = (p_pass_1).wrapping_add(2256); c.m.w32(a_, v_); };
                if (((((c.m.r32((p_pass_1).wrapping_add(2256))) == (((0i32) as u32))) as i32)) != 0) { bb_ = 143; continue 'fl1; } else { bb_ = 144; continue 'fl1; }
            }
            142 => {
                l_ret_2 = F::Dialog_Input(c, 0i32);
                if (((((l_ret_2) == (1i32)) as i32)) != 0) { bb_ = 145; continue 'fl1; } else { bb_ = 147; continue 'fl1; }
            }
            143 => {
                F::Flash_GotoLabel(c, ((p_pass_1).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44)), sym::menu_z_d__str104, ((((1i32) as i64 as u64)) as u8));
                F::Flash_GotoLabel(c, ((p_pass_1).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44)), sym::menu_z_d__str105, ((((1i32) as i64 as u64)) as u8));
                F::MsgWin_Close(c);
                bb_ = 144; continue 'fl1;
            }
            144 => {
                bb_ = 142; continue 'fl1;
            }
            145 => {
                { let v_ = ((0i32) as u32); let a_ = (p_pass_1).wrapping_add(2420); c.m.w32(a_, v_); };
                F::Dialog_Start(c, 1i32);
                F::Dialog_SetChoices(c, 0i32);
                bb_ = 146; continue 'fl1;
            }
            146 => {
                bb_ = 10; continue 'fl1;
            }
            147 => {
                if (((((l_ret_2) == ((2i32).wrapping_neg())) as i32)) != 0) { bb_ = 148; continue 'fl1; } else { bb_ = 149; continue 'fl1; }
            }
            148 => {
                { let v_ = ((3i32) as u32); let a_ = (p_pass_1).wrapping_add(2420); c.m.w32(a_, v_); };
                F::Dialog_Start(c, 1i32);
                F::Dialog_SetChoices(c, 0i32);
                bb_ = 149; continue 'fl1;
            }
            149 => {
                bb_ = 146; continue 'fl1;
            }
            150 => {
                bb_ = 151; continue 'fl1;
            }
            151 => {
                if ((F::Dialog_IsClosed(c)) != 0) { bb_ = 152; continue 'fl1; } else { bb_ = 153; continue 'fl1; }
            }
            152 => {
                { let v_ = c.m.r32((p_pass_1).wrapping_add(2420)); let a_ = (p_pass_1).wrapping_add(2256); c.m.w32(a_, v_); };
                if (((((c.m.r32((p_pass_1).wrapping_add(2256))) == (((6i32) as u32))) as i32)) != 0) { bb_ = 154; continue 'fl1; } else { bb_ = 155; continue 'fl1; }
            }
            153 => {
                l_ret_2 = F::Dialog_Input(c, 0i32);
                if (((((l_ret_2) == (1i32)) as i32)) != 0) { bb_ = 156; continue 'fl1; } else { bb_ = 158; continue 'fl1; }
            }
            154 => {
                F::menu_z_d__DcPass_Say(c, p_pass_1, 21i32);
                bb_ = 155; continue 'fl1;
            }
            155 => {
                bb_ = 153; continue 'fl1;
            }
            156 => {
                { let v_ = ((6i32) as u32); let a_ = (p_pass_1).wrapping_add(2420); c.m.w32(a_, v_); };
                F::Dialog_Start(c, 1i32);
                F::Dialog_SetChoices(c, 0i32);
                F::DcPass_StoreRec(c, (p_pass_1).wrapping_add(2340), (p_pass_1).wrapping_add(2272));
                bb_ = 157; continue 'fl1;
            }
            157 => {
                bb_ = 10; continue 'fl1;
            }
            158 => {
                if (((((l_ret_2) == ((2i32).wrapping_neg())) as i32)) != 0) { bb_ = 159; continue 'fl1; } else { bb_ = 160; continue 'fl1; }
            }
            159 => {
                { let v_ = ((3i32) as u32); let a_ = (p_pass_1).wrapping_add(2420); c.m.w32(a_, v_); };
                F::Dialog_Start(c, 1i32);
                F::Dialog_SetChoices(c, 0i32);
                bb_ = 160; continue 'fl1;
            }
            160 => {
                bb_ = 157; continue 'fl1;
            }
            161 => {
                bb_ = 162; continue 'fl1;
            }
            162 => {
                if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) || (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0)) as i32)) != 0) { bb_ = 163; continue 'fl1; } else { bb_ = 164; continue 'fl1; }
            }
            163 => {
                if ((((!(((((c.m.r32((p_pass_1).wrapping_add(2404)) as i32)) & (1i32))) != 0)) as i32)) != 0) { bb_ = 165; continue 'fl1; } else { bb_ = 166; continue 'fl1; }
            }
            164 => {
                bb_ = 10; continue 'fl1;
            }
            165 => {
                F::MsgWin_Close(c);
                F::Flash_GotoLabel(c, ((p_pass_1).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44)), sym::menu_z_d__str106, ((((1i32) as i64 as u64)) as u8));
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) { bb_ = 167; continue 'fl1; } else { bb_ = 168; continue 'fl1; }
            }
            166 => {
                { let v_ = 0i32; let a_ = (p_pass_1).wrapping_add(2244); c.m.w32(a_, (v_) as u32); };
                { let r_ = 1i32; let a_ = (p_pass_1).wrapping_add(2404); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                bb_ = 164; continue 'fl1;
            }
            167 => {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                bb_ = 168; continue 'fl1;
            }
            168 => {
                bb_ = 166; continue 'fl1;
            }
            169 => {
                bb_ = 10; continue 'fl1;
            }
            _ => unreachable!(),
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcPass_Input", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_CheckLeave(c: &mut Ctx, mut p_pass_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_CheckLeave", &PROBE_N, &[(p_pass_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32((p_pass_1).wrapping_add(2404)) as i32)) & (1i32))) != 0) {
        if ((((!(((((c.m.r32((p_pass_1).wrapping_add(2404)) as i32)) & (2i32))) != 0)) as i32)) != 0) {
            { let r_ = 2i32; let a_ = (p_pass_1).wrapping_add(2404); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
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
    if rt::probing() { crate::host::harness::probe(c, "DcPass_CheckLeave", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(2432) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 2432);
    let r_ = 'f: {
        { let _ = F::memset(c, fp, 0i32, 2428u32); };
        F::DcPass_Start(c, fp);
        F::DcPass_Init(c, fp, p_section_1);
        F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
        'l1: while ((1i32) != 0) {
            F::Gfx_BeginFrame(c);
            F::Pad_Update(c);
            F::ColorFade_Update(c);
            F::Snd_Update(c);
            F::DcPass_Update(c, fp);
            F::DcPass_Draw(c, fp);
            F::ColorFade_Draw(c);
            F::Gfx_EndFrame(c, 1i32);
            F::Dma_Flush(c);
            F::File_Stub264D90(c);
            if ((F::DcPass_CheckLeave(c, fp)) != 0) {
                break 'l1;
            }
            F::DcPass_Input(c, fp);
            F::DcPass_UpdateFace(c, fp);
        }
        F::DcPass_Term(c, fp);
        F::Dma_ResetBuffers(c);
        break 'f (c.m.r32((fp).wrapping_add(2244)) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DcPass_Run", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_ClipBegin(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_ClipBegin", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Sprite_SetScissor(c, 175i32, 511i32, 106i32, 306i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcPass_ClipBegin", &PROBE_N, &[], true); }
    r_
}

pub fn DcPass_ClipEnd(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcPass_ClipEnd", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Sprite_SetScissor(c, 0i32, 511i32, 0i32, 447i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcPass_ClipEnd", &PROBE_N, &[], true); }
    r_
}
