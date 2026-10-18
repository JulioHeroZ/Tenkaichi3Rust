#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Train_IsCleared(c: &mut Ctx, mut p_class_1: i32, mut p_lesson_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Train_IsCleared", &PROBE_N, &[(p_class_1) as u32, (p_lesson_2) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3596)).wrapping_add((((p_class_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & ((1i32).wrapping_shl((p_lesson_2) as u32)));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Train_IsCleared", &PROBE_N, &[], true); }
    r_
}

pub fn Train_IsClassCleared(c: &mut Ctx, mut p_class_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Train_IsClassCleared", &PROBE_N, &[(p_class_1) as u32], false); }
    let mut l_n_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    if (((((p_class_1) == (0i32)) as i32)) != 0) {
        l_n_2 = { let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(7088)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
    } else {
        if (((((p_class_1) == (1i32)) as i32)) != 0) {
            l_n_2 = { let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(7088)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
        } else {
            if (((((p_class_1) == (2i32)) as i32)) != 0) {
                l_n_2 = { let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(7088)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
            } else {
                l_n_2 = 0i32;
            }
        }
    }
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (l_n_2)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((!((F::Train_IsCleared(c, p_class_1, l_i_3)) != 0)) as i32)) != 0) {
                    break 'f 0i32;
                }
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Train_IsClassCleared", &PROBE_N, &[], true); }
    r_
}

pub fn Train_IsAllCleared(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Train_IsAllCleared", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((((((((F::Train_IsClassCleared(c, 0i32)) != 0) && ((F::Train_IsClassCleared(c, 1i32)) != 0)) as i32)) != 0) && ((F::Train_IsClassCleared(c, 2i32)) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Train_IsAllCleared", &PROBE_N, &[], true); }
    r_
}

pub fn Train_CopyCursor(c: &mut Ctx, mut p_dst_1: u32, mut p_src_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Train_CopyCursor", &PROBE_N, &[(p_dst_1) as u32, (p_src_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (5i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = (c.m.r32((p_src_2).wrapping_add((((l_i_3) as u32)).wrapping_mul(4))) as i32); let a_ = (p_dst_1).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Train_CopyCursor", &PROBE_N, &[], true); }
    r_
}

pub fn menu_h_d__Train_ClipGoto(c: &mut Ctx, mut p_fmt_1: u32, mut p_n_2: i32, mut p_on_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_h_d__Train_ClipGoto", &PROBE_N, &[(p_fmt_1) as u32, (p_n_2) as u32, (p_on_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(272) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 272);
    let mut l_flash_4: u32 = 0;
    let r_ = 'f: {
        l_flash_4 = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        { let _ = F::sprintf(c, fp.wrapping_add(8), p_fmt_1, &[(((p_n_2).wrapping_add(1i32)) as i64 as u64)]); };
        F::Flash_FindLabel(c, l_flash_4, ((0i32) as u32), fp.wrapping_add(8), fp);
        if ((p_on_3) != 0) {
            F::Flash_ClipGotoLabel(c, l_flash_4, fp, sym::menu_h_d__str0);
        } else {
            F::Flash_ClipGotoLabel(c, l_flash_4, fp, sym::menu_h_d__str1);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "menu_h_d__Train_ClipGoto", &PROBE_N, &[], true); }
    r_
}

pub fn Train_PlateGoto(c: &mut Ctx, mut p_plate_1: i32, mut p_on_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Train_PlateGoto", &PROBE_N, &[(p_plate_1) as u32, (p_on_2) as u32], false); }
    let r_ = 'f: {
    F::menu_h_d__Train_ClipGoto(c, sym::menu_h_d__str2, p_plate_1, p_on_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Train_PlateGoto", &PROBE_N, &[], true); }
    r_
}

pub fn Train_Plate2Goto(c: &mut Ctx, mut p_plate_1: i32, mut p_on_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Train_Plate2Goto", &PROBE_N, &[(p_plate_1) as u32, (p_on_2) as u32], false); }
    let r_ = 'f: {
    F::menu_h_d__Train_ClipGoto(c, sym::menu_h_d__str3, p_plate_1, p_on_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Train_Plate2Goto", &PROBE_N, &[], true); }
    r_
}

pub fn Train_CursorGoto(c: &mut Ctx, mut p_on_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Train_CursorGoto", &PROBE_N, &[(p_on_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); (c.m.r32(a_) as i32) };
        match sv2 {
            0i32 | 1i32 => {
                { let a0_ = { let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::Train_PlateGoto(c, a0_, p_on_1) };
                break 's1;
            }
            2i32 => {
                { let a0_ = { let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1100)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::Train_Plate2Goto(c, a0_, p_on_1) };
                break 's1;
            }
            3i32 | 4i32 | 5i32 => {
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Train_CursorGoto", &PROBE_N, &[], true); }
    r_
}

pub fn Train_DrawClearIcon(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Train_DrawClearIcon", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_flash_1: u32 = 0;
    let r_ = 'f: {
        l_flash_1 = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        if ((((((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); (c.m.r32(a_) as i32) }) == (3i32)) as i32)) != 0) || ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); (c.m.r32(a_) as i32) }) == (4i32)) as i32)) != 0)) as i32)) != 0) {
            if (({ let a0_ = { let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a1_ = ({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1120); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1116); (c.m.r32(a_) as i32) }); F::Train_IsCleared(c, a0_, a1_) }) != 0) {
                F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_h_d__str4, fp);
                { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                F::Flash_FindLabel(c, l_flash_1, sym::menu_h_d__str5, sym::menu_h_d__str6, fp);
                { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
            } else {
                F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_h_d__str7, fp);
                { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                F::Flash_FindLabel(c, l_flash_1, sym::menu_h_d__str8, sym::menu_h_d__str9, fp);
                { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
            }
        } else {
            if ((((((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); (c.m.r32(a_) as i32) }) == (5i32)) as i32)) != 0) || ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0)) as i32)) != 0) {
                F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_h_d__str10, fp);
                { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                F::Flash_FindLabel(c, l_flash_1, sym::menu_h_d__str11, sym::menu_h_d__str12, fp);
                { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Train_DrawClearIcon", &PROBE_N, &[], true); }
    r_
}

pub fn menu_h_d__Train_SetUv(c: &mut Ctx, mut p_flash_1: u32, mut p_parent_2: u32, mut p_name_3: u32, mut p_uv_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_h_d__Train_SetUv", &PROBE_N, &[(p_flash_1) as u32, (p_parent_2) as u32, (p_name_3) as u32, (p_uv_4) as u32], false); }
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
    if rt::probing() { crate::host::harness::probe(c, "menu_h_d__Train_SetUv", &PROBE_N, &[], true); }
    r_
}

pub fn Train_DrawPlates(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Train_DrawPlates", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(288) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 288);
    let mut l_flash_1: u32 = 0;
    let mut l_tex_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
        l_flash_1 = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        's1: {
            let sv4: i32 = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); (c.m.r32(a_) as i32) };
            match sv4 {
                0i32 => {
                    l_tex_2 = 0i32;
                    break 's1;
                }
                1i32 => {
                    l_tex_2 = ({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); (c.m.r32(a_) as i32) }).wrapping_mul(2i32);
                    break 's1;
                }
                _ => {
                    l_tex_2 = 0i32;
                    break 's1;
                }
            }
        }
        {
            l_i_3 = 0i32;
            'l2: loop {
                if !(((((l_i_3) < (6i32)) as i32)) != 0) { break 'l2; }
                'c3: {
                    { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_h_d__str13, &[(((l_i_3).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(8), sym::menu_h_d__str14, fp);
                    F::Flash_ClipSetTex(c, l_flash_1, fp, l_tex_2);
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(8), sym::menu_h_d__str15, fp);
                    F::Flash_ClipSetTex(c, l_flash_1, fp, l_tex_2);
                    { let v_ = 0i32; let a_ = fp.wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 512i32; let a_ = (fp.wrapping_add(264)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (l_i_3).wrapping_mul(32i32); let a_ = (fp.wrapping_add(264)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((l_i_3).wrapping_mul(32i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(264)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    F::menu_h_d__Train_SetUv(c, l_flash_1, fp.wrapping_add(8), sym::menu_h_d__str16, fp.wrapping_add(264));
                    F::menu_h_d__Train_SetUv(c, l_flash_1, fp.wrapping_add(8), sym::menu_h_d__str17, fp.wrapping_add(264));
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(8), sym::menu_h_d__str18, fp);
                    { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Train_DrawPlates", &PROBE_N, &[], true); }
    r_
}

pub fn Train_DrawList(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Train_DrawList", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(272) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 272);
    let mut l_flash_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_lesson_3: i32 = 0;
    let mut l_line_4: i32 = 0;
    let r_ = 'f: {
        l_flash_1 = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (5i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_h_d__str19, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                    if (((((l_i_2) == (3i32)) as i32)) != 0) {
                        l_lesson_3 = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1124); (c.m.r32(a_) as i32) };
                    } else {
                        if (((((l_i_2) == (4i32)) as i32)) != 0) {
                            l_lesson_3 = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1128); (c.m.r32(a_) as i32) };
                        } else {
                            l_lesson_3 = ({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1116); (c.m.r32(a_) as i32) }).wrapping_add(l_i_2);
                        }
                    }
                    l_line_4 = { let a_ = (((c.m.r32(sym::gTrain)).wrapping_add(6548)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(60))).wrapping_add((((l_lesson_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(8), sym::menu_h_d__str20, fp);
                    { let a5_ = ((c.m.r32(sym::gTrain)).wrapping_add(272)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, l_line_4, a5_) };
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(8), sym::menu_h_d__str21, fp);
                    if (((((l_i_2) != (4i32)) as i32)) != 0) {
                        if (({ let a0_ = { let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::Train_IsCleared(c, a0_, l_lesson_3) }) != 0) {
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                        } else {
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                        }
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Train_DrawList", &PROBE_N, &[], true); }
    r_
}

pub fn Train_ScissorOn(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Train_ScissorOn", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Sprite_SetScissor(c, 0i32, 512i32, 79i32, 303i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Train_ScissorOn", &PROBE_N, &[], true); }
    r_
}

pub fn Train_ScissorOff(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Train_ScissorOff", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Sprite_SetScissor(c, 0i32, 511i32, 0i32, 447i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Train_ScissorOff", &PROBE_N, &[], true); }
    r_
}

pub fn Train_SetClip(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Train_SetClip", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_FindLabel(c, a0_, ((0i32) as u32), sym::menu_h_d__str22, fp) };
        { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a2_ = (c.m.r32(sym::gTrain)).wrapping_add(7120); let a3_ = (c.m.r32(sym::gTrain)).wrapping_add(7124); F::Flash_ClipGetPos(c, a0_, fp, a2_, a3_) };
        { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_ClipSetCallbackA(c, a0_, fp, fid::Train_ScissorOn, ((0i32) as u32)) };
        { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_ClipSetCallbackB(c, a0_, fp, fid::Train_ScissorOff, ((0i32) as u32)) };
        { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_FindLabel(c, a0_, ((0i32) as u32), sym::menu_h_d__str23, fp) };
        { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a2_ = (c.m.r32(sym::gTrain)).wrapping_add(7120); let a3_ = (c.m.r32(sym::gTrain)).wrapping_add(7124); F::Flash_ClipGetPos(c, a0_, fp, a2_, a3_) };
        { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_ClipSetCallbackA(c, a0_, fp, fid::Train_ScissorOn, ((0i32) as u32)) };
        { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_ClipSetCallbackB(c, a0_, fp, fid::Train_ScissorOff, ((0i32) as u32)) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Train_SetClip", &PROBE_N, &[], true); }
    r_
}

pub fn Train_HideArrow(c: &mut Ctx, mut p_up_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Train_HideArrow", &PROBE_N, &[(p_up_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(528) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 528);
    let r_ = 'f: {
        if ((p_up_1) != 0) {
            { let _ = F::strcpy(c, fp.wrapping_add(8), sym::menu_h_d__str24); };
            { let _ = F::strcpy(c, fp.wrapping_add(264), sym::menu_h_d__str25); };
        } else {
            { let _ = F::strcpy(c, fp.wrapping_add(8), sym::menu_h_d__str26); };
            { let _ = F::strcpy(c, fp.wrapping_add(264), sym::menu_h_d__str27); };
        }
        { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_FindLabel(c, a0_, fp.wrapping_add(8), fp.wrapping_add(264), fp) };
        { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); { let _ = F::Flash_ClipSetFlags(c, a0_, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); } };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Train_HideArrow", &PROBE_N, &[], true); }
    r_
}

pub fn Train_DrawArrows(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Train_DrawArrows", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let r_ = 'f: {
        { let v_ = 0i32; let a_ = fp; c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::menu_h_d__Train_SetUv(c, a0_, sym::menu_h_d__str28, sym::menu_h_d__str29, fp) };
        { let v_ = 32i32; let a_ = fp; c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::menu_h_d__Train_SetUv(c, a0_, sym::menu_h_d__str30, sym::menu_h_d__str31, fp) };
        { let v_ = 32i32; let a_ = fp; c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::menu_h_d__Train_SetUv(c, a0_, sym::menu_h_d__str32, sym::menu_h_d__str33, fp) };
        if ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1116); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            F::Train_HideArrow(c, 1i32);
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1116); (c.m.r32(a_) as i32) }) == (({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(7088)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(3i32))) as i32)) != 0) {
                F::Train_HideArrow(c, 0i32);
            }
        }
        {
            { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_FindLabel(c, a0_, ((0i32) as u32), sym::menu_h_d__str34, fp.wrapping_add(20)) };
            if ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1088); (c.m.r32(a_) as i32) }) >= (({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(7128); (c.m.r32(a_) as i32) }).wrapping_sub(1i32))) as i32)) != 0) {
                { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); { let _ = F::Flash_ClipSetFlags(c, a0_, fp.wrapping_add(20), 2i32, ((((0i32) as i64 as u64)) as u8)); } };
            } else {
                { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); { let _ = F::Flash_ClipSetFlags(c, a0_, fp.wrapping_add(20), 2i32, ((((1i32) as i64 as u64)) as u8)); } };
            }
            { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_FindLabel(c, a0_, ((0i32) as u32), sym::menu_h_d__str35, fp.wrapping_add(20)) };
            { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); { let _ = F::Flash_ClipSetFlags(c, a0_, fp.wrapping_add(20), 2i32, ((((0i32) as i64 as u64)) as u8)); } };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Train_DrawArrows", &PROBE_N, &[], true); }
    r_
}

pub fn menu_h_d__Train_Clamp(c: &mut Ctx, mut p_v_1: u32, mut p_lo_2: i32, mut p_hi_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_h_d__Train_Clamp", &PROBE_N, &[(p_v_1) as u32, (p_lo_2) as u32, (p_hi_3) as u32], false); }
    let r_ = 'f: {
    if ((((((c.m.r32(p_v_1) as i32)) < (p_lo_2)) as i32)) != 0) {
        { let v_ = p_lo_2; let a_ = p_v_1; c.m.w32(a_, (v_) as u32); };
        break 'f 1i32;
    }
    if ((((((c.m.r32(p_v_1) as i32)) > (p_hi_3)) as i32)) != 0) {
        { let v_ = p_hi_3; let a_ = p_v_1; c.m.w32(a_, (v_) as u32); };
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_h_d__Train_Clamp", &PROBE_N, &[], true); }
    r_
}

pub fn menu_h_d__Train_IsOutside(c: &mut Ctx, mut p_v_1: u32, mut p_lo_2: i32, mut p_hi_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_h_d__Train_IsOutside", &PROBE_N, &[(p_v_1) as u32, (p_lo_2) as u32, (p_hi_3) as u32], false); }
    let r_ = 'f: {
    if ((((((c.m.r32(p_v_1) as i32)) < (p_lo_2)) as i32)) != 0) {
        break 'f 1i32;
    }
    if ((((((c.m.r32(p_v_1) as i32)) > (p_hi_3)) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_h_d__Train_IsOutside", &PROBE_N, &[], true); }
    r_
}

pub fn menu_h_d__Train_Wrap(c: &mut Ctx, mut p_v_1: u32, mut p_lo_2: i32, mut p_hi_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_h_d__Train_Wrap", &PROBE_N, &[(p_v_1) as u32, (p_lo_2) as u32, (p_hi_3) as u32], false); }
    let r_ = 'f: {
    if ((((((c.m.r32(p_v_1) as i32)) < (p_lo_2)) as i32)) != 0) {
        { let v_ = p_hi_3; let a_ = p_v_1; c.m.w32(a_, (v_) as u32); };
    } else {
        if ((((((c.m.r32(p_v_1) as i32)) > (p_hi_3)) as i32)) != 0) {
            { let v_ = p_lo_2; let a_ = p_v_1; c.m.w32(a_, (v_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_h_d__Train_Wrap", &PROBE_N, &[], true); }
    r_
}

pub fn Train_MoveRow(c: &mut Ctx, mut p_dir_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Train_MoveRow", &PROBE_N, &[(p_dir_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(272) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 272);
    let mut l_line_2: i32 = 0;
    let mut l_hi_3: i32 = 0;
    let r_ = 'f: {
        l_hi_3 = ({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(7088)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(1i32);
        { let v_ = (({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1120); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1116); (c.m.r32(a_) as i32) })).wrapping_add(p_dir_1); let a_ = fp.wrapping_add(256); c.m.w32(a_, (v_) as u32); };
        if ((F::menu_h_d__Train_IsOutside(c, fp.wrapping_add(256), 0i32, l_hi_3)) != 0) {
            break 'f 0i32;
        }
        { let a0_ = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1120); (c.m.r32(a_) as i32) }; F::Train_Plate2Goto(c, a0_, 0i32) };
        { let r_ = p_dir_1; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1120); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
        if (({ let a0_ = (c.m.r32(sym::gTrain)).wrapping_add(1120); F::menu_h_d__Train_Clamp(c, a0_, 0i32, (3i32).wrapping_sub(1i32)) }) != 0) {
            { let r_ = p_dir_1; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1116); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
            if ((((!(({ let a0_ = (c.m.r32(sym::gTrain)).wrapping_add(1116); let a2_ = ({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(7088)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(3i32); F::menu_h_d__Train_Clamp(c, a0_, 0i32, a2_) }) != 0)) as i32)) != 0) {
                's1: {
                    let sv4: i32 = p_dir_1;
                    match sv4 {
                        1i32 => {
                            { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_h_d__str36, ((((1i32) as i64 as u64)) as u8)) };
                            { let v_ = ({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1116); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1124); c.m.w32(a_, (v_) as u32); };
                            break 's1;
                        }
                        (-1i32) => {
                            { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_h_d__str37, ((((1i32) as i64 as u64)) as u8)) };
                            { let v_ = ({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1116); (c.m.r32(a_) as i32) }).wrapping_add(3i32); let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1124); c.m.w32(a_, (v_) as u32); };
                            break 's1;
                        }
                        _ => {
                        }
                    }
                }
            }
        }
        { let _ = { let a2_ = ({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1120); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp, sym::menu_h_d__str38, &[((a2_) as i64 as u64)]) }; };
        { let a0_ = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1120); (c.m.r32(a_) as i32) }; F::Train_Plate2Goto(c, a0_, 1i32) };
        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
        l_line_2 = { let a_ = (((c.m.r32(sym::gTrain)).wrapping_add(6728)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(60))).wrapping_add((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1120); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1116); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
        if ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            { let a0_ = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1048); c.m.r32(a_) }; F::Voice_PlayWithSubtitle(c, a0_, 34749i32, l_line_2) };
        } else {
            { let a0_ = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1052); c.m.r32(a_) }; F::Voice_PlayWithSubtitle(c, a0_, 33400i32, l_line_2) };
        }
        { let v_ = l_line_2; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1084); c.m.w32(a_, (v_) as u32); };
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Train_MoveRow", &PROBE_N, &[], true); }
    r_
}

pub fn Train_NextRow(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Train_NextRow", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(256) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 256);
    let mut l_line_1: i32 = 0;
    let r_ = 'f: {
        { let a0_ = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1120); (c.m.r32(a_) as i32) }; F::Train_Plate2Goto(c, a0_, 0i32) };
        { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1120); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        if (({ let a0_ = (c.m.r32(sym::gTrain)).wrapping_add(1120); F::menu_h_d__Train_Clamp(c, a0_, 0i32, (3i32).wrapping_sub(1i32)) }) != 0) {
            { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1116); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        if ((((!(({ let a0_ = (c.m.r32(sym::gTrain)).wrapping_add(1116); let a2_ = ({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(7088)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(3i32); F::menu_h_d__Train_Clamp(c, a0_, 0i32, a2_) }) != 0)) as i32)) != 0) {
            { let v_ = ({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1116); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1124); c.m.w32(a_, (v_) as u32); };
        }
        { let _ = { let a2_ = ({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1120); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp, sym::menu_h_d__str39, &[((a2_) as i64 as u64)]) }; };
        { let a0_ = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1120); (c.m.r32(a_) as i32) }; F::Train_Plate2Goto(c, a0_, 1i32) };
        l_line_1 = { let a_ = (((c.m.r32(sym::gTrain)).wrapping_add(6728)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(60))).wrapping_add((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1120); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1116); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
        if ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            { let a0_ = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1048); c.m.r32(a_) }; F::Voice_PlayWithSubtitle(c, a0_, 34749i32, l_line_1) };
        } else {
            { let a0_ = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1052); c.m.r32(a_) }; F::Voice_PlayWithSubtitle(c, a0_, 33400i32, l_line_1) };
        }
        { let v_ = l_line_1; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1084); c.m.w32(a_, (v_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Train_NextRow", &PROBE_N, &[], true); }
    r_
}

pub fn Train_MoveCursor(c: &mut Ctx, mut p_dir_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Train_MoveCursor", &PROBE_N, &[(p_dir_1) as u32], false); }
    let mut l_d_2: i32 = 0;
    let mut l_line_3: i32 = 0;
    let r_ = 'f: {
    l_d_2 = 0i32;
    's1: {
        let sv4: i32 = p_dir_1;
        match sv4 {
            0i32 => {
                l_d_2 = (1i32).wrapping_neg();
                break 's1;
            }
            1i32 => {
                l_d_2 = 1i32;
                break 's1;
            }
            _ => {
            }
        }
    }
    's2: {
        let sv5: i32 = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); (c.m.r32(a_) as i32) };
        match sv5 {
            0i32 => {
                F::Train_CursorGoto(c, 0i32);
                { let r_ = l_d_2; let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); F::menu_h_d__Train_Wrap(c, a0_, 0i32, 1i32) };
                F::Train_CursorGoto(c, 1i32);
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                l_line_3 = ({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(2i32);
                if ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    { let a0_ = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1048); c.m.r32(a_) }; F::Voice_PlayWithSubtitle(c, a0_, 34749i32, l_line_3) };
                } else {
                    { let a0_ = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1052); c.m.r32(a_) }; F::Voice_PlayWithSubtitle(c, a0_, 33400i32, l_line_3) };
                }
                { let v_ = l_line_3; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1084); c.m.w32(a_, (v_) as u32); };
                break 's2;
            }
            1i32 => {
                F::Train_CursorGoto(c, 0i32);
                { let r_ = l_d_2; let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); F::menu_h_d__Train_Wrap(c, a0_, 0i32, 2i32) };
                F::Train_CursorGoto(c, 1i32);
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                l_line_3 = ({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32);
                if ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    { let a0_ = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1048); c.m.r32(a_) }; F::Voice_PlayWithSubtitle(c, a0_, 34749i32, l_line_3) };
                } else {
                    { let a0_ = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1052); c.m.r32(a_) }; F::Voice_PlayWithSubtitle(c, a0_, 33400i32, l_line_3) };
                }
                { let v_ = l_line_3; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1084); c.m.w32(a_, (v_) as u32); };
                break 's2;
            }
            2i32 => {
                { let _ = F::Train_MoveRow(c, l_d_2); };
                break 's2;
            }
            3i32 | 4i32 | 5i32 => {
                break 's2;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Train_MoveCursor", &PROBE_N, &[], true); }
    r_
}

pub fn menu_h_d__Train_Say(c: &mut Ctx, mut p_line_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_h_d__Train_Say", &PROBE_N, &[(p_line_1) as u32], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1048); c.m.r32(a_) }; F::Voice_PlayWithSubtitle(c, a0_, 34749i32, p_line_1) };
    } else {
        { let a0_ = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1052); c.m.r32(a_) }; F::Voice_PlayWithSubtitle(c, a0_, 33400i32, p_line_1) };
    }
    { let v_ = p_line_1; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1084); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_h_d__Train_Say", &PROBE_N, &[], true); }
    r_
}

pub fn Train_RestoreScroll(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Train_RestoreScroll", &PROBE_N, &[], false); }
    let mut l_scroll_1: u32 = 0;
    let mut l_top_2: i32 = 0;
    let r_ = 'f: {
    l_scroll_1 = (c.m.r32(sym::gTrain)).wrapping_add(1116);
    { let v_ = { let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1100)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (l_scroll_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    l_top_2 = ({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(7088)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(3i32);
    if (((((l_top_2) < ({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1100)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
        { let v_ = l_top_2; let a_ = (l_scroll_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = (3i32).wrapping_sub(({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(7088)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1100)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })); let a_ = (l_scroll_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    } else {
        { let v_ = 0i32; let a_ = (l_scroll_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Train_RestoreScroll", &PROBE_N, &[], true); }
    r_
}

pub fn Train_SaveCursor(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Train_SaveCursor", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = ({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1120); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1116); (c.m.r32(a_) as i32) }); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1100)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Train_SaveCursor", &PROBE_N, &[], true); }
    r_
}

pub fn menu_h_d__Train_ClassLessons(c: &mut Ctx, mut p_tbl_1: u32, mut p_class_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_h_d__Train_ClassLessons", &PROBE_N, &[(p_tbl_1) as u32, (p_class_2) as u32], false); }
    let mut l_p_3: u32 = 0;
    let r_ = 'f: {
    l_p_3 = ((0i32) as u32);
    's1: {
        let sv4: i32 = p_class_2;
        match sv4 {
            0i32 => {
                l_p_3 = c.m.r32(p_tbl_1);
                break 's1;
            }
            1i32 => {
                l_p_3 = (c.m.r32(p_tbl_1)).wrapping_add((((13i32) as u32)).wrapping_mul(16));
                break 's1;
            }
            2i32 => {
                l_p_3 = ((c.m.r32(p_tbl_1)).wrapping_add((((13i32) as u32)).wrapping_mul(16))).wrapping_add((((11i32) as u32)).wrapping_mul(16));
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f l_p_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_h_d__Train_ClassLessons", &PROBE_N, &[], true); }
    r_
}

pub fn menu_h_d__Train_IsSkipped(c: &mut Ctx, mut p_tbl_1: u32, mut p_page_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_h_d__Train_IsSkipped", &PROBE_N, &[(p_tbl_1) as u32, (p_page_2) as u32], false); }
    let mut l_found_3: i32 = 0;
    let mut l_i_4: u32 = 0;
    let r_ = 'f: {
    l_found_3 = 0i32;
    {
        l_i_4 = ((0i32) as u32);
        'l1: loop {
            if !(((((l_i_4) < (c.m.r32((p_tbl_1).wrapping_add(8)))) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((({ let a_ = (c.m.r32((p_tbl_1).wrapping_add(4))).wrapping_add((l_i_4).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)) == (p_page_2)) as i32)) != 0) {
                    l_found_3 = 1i32;
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    break 'f l_found_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_h_d__Train_IsSkipped", &PROBE_N, &[], true); }
    r_
}

pub fn Train_BuildLists(c: &mut Ctx, mut p_tbl_1: u32, mut p_out_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Train_BuildLists", &PROBE_N, &[(p_tbl_1) as u32, (p_out_2) as u32], false); }
    let mut l_class_3: i32 = 0;
    let mut l_p_4: u32 = 0;
    let mut l_count_5: i32 = 0;
    let mut l_n_6: i32 = 0;
    let mut l_page_7: u32 = 0;
    let r_ = 'f: {
    {
        l_class_3 = 0i32;
        'l1: loop {
            if !(((((l_class_3) < (3i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_p_4 = F::menu_h_d__Train_ClassLessons(c, p_tbl_1, l_class_3);
                l_count_5 = 0i32;
                'l3: while ((1i32) != 0) {
                    l_n_6 = 0i32;
                    {
                        l_page_7 = (((c.m.r32((l_p_4).wrapping_add(4)) as i32)) as u32);
                        'l4: loop {
                            if !(((((l_page_7) < ((((((c.m.r8((l_p_4).wrapping_add(2))) as i32)).wrapping_add((c.m.r32((l_p_4).wrapping_add(4)) as i32))) as u32))) as i32)) != 0) { break 'l4; }
                            'c5: {
                                if ((((!((F::menu_h_d__Train_IsSkipped(c, p_tbl_1, l_page_7)) != 0)) as i32)) != 0) {
                                    { let v_ = ((l_page_7) as i32); let a_ = (((p_out_2).wrapping_add((((l_class_3) as u32)).wrapping_mul(1800))).wrapping_add((((l_count_5) as u32)).wrapping_mul(120))).wrapping_add((((l_n_6) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                    { let o_ = l_n_6; let n_ = (o_).wrapping_add(1); l_n_6 = n_; };
                                }
                            }
                            { let o_ = l_page_7; let n_ = (o_).wrapping_add(1); l_page_7 = n_; };
                        }
                    }
                    if (((((l_n_6) != (0i32)) as i32)) != 0) {
                        { let v_ = l_n_6; let a_ = (((p_out_2).wrapping_add(5760)).wrapping_add((((l_class_3) as u32)).wrapping_mul(60))).wrapping_add((((l_count_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r16(l_p_4)) as i32); let a_ = (((p_out_2).wrapping_add(5400)).wrapping_add((((l_class_3) as u32)).wrapping_mul(60))).wrapping_add((((l_count_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        { let v_ = (c.m.r32((l_p_4).wrapping_add(8)) as i32); let a_ = (((p_out_2).wrapping_add(5580)).wrapping_add((((l_class_3) as u32)).wrapping_mul(60))).wrapping_add((((l_count_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        { let o_ = l_count_5; let n_ = (o_).wrapping_add(1); l_count_5 = n_; };
                    }
                    if ((((((c.m.r8((l_p_4).wrapping_add(3))) as i32)) & (4i32))) != 0) {
                        break 'l3;
                    }
                    { let o_ = l_p_4; let n_ = (o_).wrapping_add(16); l_p_4 = n_; };
                }
                { let v_ = l_count_5; let a_ = ((p_out_2).wrapping_add(5940)).wrapping_add((((l_class_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            }
            { let o_ = l_class_3; let n_ = (o_).wrapping_add(1); l_class_3 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Train_BuildLists", &PROBE_N, &[], true); }
    r_
}

pub fn Train_DrawTitle(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Train_DrawTitle", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_flash_1: u32 = 0;
    let r_ = 'f: {
        l_flash_1 = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        { let v_ = ({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_mul(64i32); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32((fp).wrapping_add(4)) as i32)).wrapping_add(64i32); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = fp; c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        F::menu_h_d__Train_SetUv(c, l_flash_1, ((0i32) as u32), sym::menu_h_d__str40, fp);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Train_DrawTitle", &PROBE_N, &[], true); }
    r_
}

pub fn Train_DrawIconWin(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Train_DrawIconWin", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) {
        F::IconWin_SetIcon(c, 2i32);
    } else {
        F::IconWin_SetIcon(c, 0i32);
    }
    F::IconWin_Draw(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Train_DrawIconWin", &PROBE_N, &[], true); }
    r_
}

pub fn Train_DrawBg(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Train_DrawBg", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let r_ = 'f: {
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_FindLabel(c, a0_, ((0i32) as u32), sym::menu_h_d__str41, fp) };
        { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a3_ = (c.m.r32(sym::gTrain)).wrapping_add(1060); F::FlashAnim_Scroll(c, a0_, fp, fp.wrapping_add(8), a3_, ((0i32) as u32), F32(0x3d422e44), F32(0x00000000)) };
        { let a0_ = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(52); c.m.r32(a_) }; F::Sprite_DrawPicture(c, a0_, 0i32, 0i32, 128i32) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Train_DrawBg", &PROBE_N, &[], true); }
    r_
}

pub fn Train_DrawGuides(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Train_DrawGuides", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(1808) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 1808);
    let mut l_who_1: i32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp.wrapping_add(8), 0, 512);
        rt::memset(c, fp.wrapping_add(8).wrapping_add(0), 0, 256);
        rt::memcpy(c, fp.wrapping_add(8).wrapping_add(0), sym::menu_h_d__str42, 18);
        rt::memset(c, fp.wrapping_add(8).wrapping_add(256), 0, 256);
        rt::memcpy(c, fp.wrapping_add(8).wrapping_add(256), sym::menu_h_d__str43, 15);
        rt::memset(c, fp.wrapping_add(520), 0, 512);
        rt::memset(c, fp.wrapping_add(520).wrapping_add(0), 0, 256);
        rt::memcpy(c, fp.wrapping_add(520).wrapping_add(0), sym::menu_h_d__str44, 1);
        rt::memset(c, fp.wrapping_add(520).wrapping_add(256), 0, 256);
        rt::memcpy(c, fp.wrapping_add(520).wrapping_add(256), sym::menu_h_d__str45, 22);
        rt::memset(c, fp.wrapping_add(1032), 0, 512);
        rt::memset(c, fp.wrapping_add(1032).wrapping_add(0), 0, 256);
        rt::memcpy(c, fp.wrapping_add(1032).wrapping_add(0), sym::menu_h_d__str46, 27);
        rt::memset(c, fp.wrapping_add(1032).wrapping_add(256), 0, 256);
        rt::memcpy(c, fp.wrapping_add(1032).wrapping_add(256), sym::menu_h_d__str47, 24);
        l_who_1 = ((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); (c.m.r32(a_) as i32) }) != (0i32)) as i32);
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let _ = { let a2_ = (if ((((({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1032)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) { 98i32 } else { 97i32 }); F::sprintf(c, fp.wrapping_add(1544), (fp.wrapping_add(520)).wrapping_add((((l_i_2) as u32)).wrapping_mul(256)), &[((a2_) as i64 as u64)]) }; };
                    { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_FindLabel(c, a0_, ((0i32) as u32), fp.wrapping_add(1544), fp) };
                    { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); { let _ = F::Flash_ClipSetFlags(c, a0_, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); } };
                    { let _ = { let a2_ = ({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1032)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(97i32); F::sprintf(c, fp.wrapping_add(1544), (fp.wrapping_add(520)).wrapping_add((((l_i_2) as u32)).wrapping_mul(256)), &[((a2_) as i64 as u64)]) }; };
                    if (((((l_who_1) == (l_i_2)) as i32)) != 0) {
                        { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_FindLabel(c, a0_, ((0i32) as u32), fp.wrapping_add(1544), fp) };
                        { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a2_ = (((c.m.r32(sym::gTrain)).wrapping_add(248)).wrapping_add(8)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); F::FlashAnim_Blink(c, a0_, fp, a2_, 0i32) };
                        { let _ = { let a2_ = (if ((((({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1032)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) { 98i32 } else { 97i32 }); F::sprintf(c, fp.wrapping_add(1544), (fp.wrapping_add(1032)).wrapping_add((((l_i_2) as u32)).wrapping_mul(256)), &[((a2_) as i64 as u64)]) }; };
                        { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_FindLabel(c, a0_, ((0i32) as u32), fp.wrapping_add(1544), fp) };
                        { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); { let _ = F::Flash_ClipSetFlags(c, a0_, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); } };
                        { let _ = { let a2_ = ({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1032)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(97i32); F::sprintf(c, fp.wrapping_add(1544), (fp.wrapping_add(1032)).wrapping_add((((l_i_2) as u32)).wrapping_mul(256)), &[((a2_) as i64 as u64)]) }; };
                        { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_FindLabel(c, a0_, ((0i32) as u32), fp.wrapping_add(1544), fp) };
                        if (((((F::Voice_GetStat(c, 0i32)) == (3i32)) as i32)) != 0) {
                            { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a2_ = (((c.m.r32(sym::gTrain)).wrapping_add(248)).wrapping_add(16)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); F::FlashAnim_Talk(c, a0_, fp, a2_, 0i32) };
                        } else {
                            { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::FlashAnim_ShowNext2(c, a0_, fp, 0i32) };
                        }
                    }
                    { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_FindLabel(c, a0_, ((0i32) as u32), (fp.wrapping_add(8)).wrapping_add((((l_i_2) as u32)).wrapping_mul(256)), fp) };
                    { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a2_ = { let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1032)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::Flash_ClipSetTex(c, a0_, fp, a2_) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Train_DrawGuides", &PROBE_N, &[], true); }
    r_
}

pub fn menu_h_d__Train_Section(c: &mut Ctx, mut p_n_1: i32, mut p_host_2: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_h_d__Train_Section", &PROBE_N, &[(p_n_1) as u32, (p_host_2) as u32], false); }
    let r_ = 'f: {
    break 'f ({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((p_n_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_h_d__Train_Section", &PROBE_N, &[], true); }
    r_
}

pub fn Train_Init(c: &mut Ctx, mut p_section_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Train_Init", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = F::Heap_Alloc(c, 7136i32, ((32i32) as u32), 0i32, 2i32); let a_ = sym::gTrain; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gTrain); F::memset(c, a0_, 0i32, ((7136i32) as u32)) }; };
        { let v_ = (c.m.r32(sym::gMenuArc6)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc6)).wrapping_add((((p_section_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = c.m.r32(sym::gTrain); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gTrain); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = F::menu_h_d__Train_Section(c, 2i32, sym::menu_h_d__str48); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = c.m.r32(fp); let a_ = (c.m.r32(sym::gTrain)).wrapping_add(52); c.m.w32(a_, v_); };
        { let v_ = F::menu_h_d__Train_Section(c, 7i32, sym::menu_h_d__str49); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(56)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(56)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(56)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(56)).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(56)).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(56)).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((7i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(56)).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((8i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(56)).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((9i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(56)).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((10i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(56)).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((11i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(56)).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((12i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(56)).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((13i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(56)).wrapping_add((((30i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((14i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(56)).wrapping_add((((32i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((15i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(56)).wrapping_add((((33i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::menu_h_d__Train_Section(c, 6i32, sym::menu_h_d__str50); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(56)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::menu_h_d__Train_Section(c, 9i32, sym::menu_h_d__str51); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(56)).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(56)).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::menu_h_d__Train_Section(c, 8i32, sym::menu_h_d__str52); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(56)).wrapping_add((((31i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::menu_h_d__Train_Section(c, 4i32, sym::menu_h_d__str53); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(56)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(56)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(56)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::menu_h_d__Train_Section(c, 3i32, sym::menu_h_d__str54); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(56)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(56)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(56)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(56)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((8i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(56)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(56)).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a1_ = F::menu_h_d__Train_Section(c, 1i32, sym::menu_h_d__str55); let a2_ = (c.m.r32(sym::gTrain)).wrapping_add(56); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let v_ = F::menu_h_d__Train_Section(c, 5i32, sym::menu_h_d__str56); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let a0_ = F::menu_h_d__Train_Section(c, 10i32, sym::menu_h_d__str57); let a1_ = c.m.r32(fp); F::IconWin_Init(c, a0_, a1_) };
        { let v_ = F::menu_h_d__Train_Section(c, 14i32, sym::menu_h_d__str58); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1040)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::menu_h_d__Train_Section(c, 15i32, sym::menu_h_d__str59); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1040)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = F::menu_h_d__Train_Section(c, 11i32, sym::menu_h_d__str60); let a1_ = { let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1040)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a3_ = (((c.m.r32(sym::gTrain)).wrapping_add(972)) as i32); let _ = a3_; F::MsgWin_Init(c, a0_, a1_, 0i32) };
        { let v_ = F::menu_h_d__Train_Section(c, 12i32, sym::menu_h_d__str61); let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1048); c.m.w32(a_, v_); };
        { let v_ = F::menu_h_d__Train_Section(c, 13i32, sym::menu_h_d__str62); let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1052); c.m.w32(a_, v_); };
        { let v_ = F::menu_h_d__Train_Section(c, 16i32, sym::menu_h_d__str63); let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1056); c.m.w32(a_, v_); };
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (5i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(272)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); let a1_ = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1056); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, ((2i32) as u32)) };
                    { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(272)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); F::TextBox_SetLineOffsets(c, a0_, 9i32, 0i32, 0i32, 0i32, 0i32) };
                    { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(272)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); F::TextBox_SetMaxSize(c, a0_, 256i32, 64i32) };
                    { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(272)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); F::TextBox_SetUnkC(c, a0_, 10i32, 4i32) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = F::menu_h_d__Train_Section(c, 17i32, sym::menu_h_d__str64); let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1076); c.m.w32(a_, v_); };
        { let v_ = F::menu_h_d__Train_Section(c, 22i32, sym::menu_h_d__str65); let a_ = (c.m.r32(sym::gTrain)).wrapping_add(7100); c.m.w32(a_, v_); };
        { let v_ = (F::menu_h_d__Train_Section(c, 24i32, sym::menu_h_d__str66)).wrapping_add(((16i32) as u32)); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(7100)).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = { let a_ = F::menu_h_d__Train_Section(c, 24i32, sym::menu_h_d__str67); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(7100)).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = (F::menu_h_d__Train_Section(c, 26i32, sym::menu_h_d__str68)).wrapping_add(((16i32) as u32)); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(7100)).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = { let a_ = F::menu_h_d__Train_Section(c, 26i32, sym::menu_h_d__str69); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(7100)).wrapping_add(16); c.m.w32(a_, v_); };
        { let v_ = F::Heap_Alloc(c, 57344i32, ((64i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1064); c.m.w32(a_, v_); };
        { let v_ = F::Heap_Alloc(c, 67584i32, ((32i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1068); c.m.w32(a_, v_); };
        { let v_ = F::Heap_Alloc(c, 67584i32, ((32i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1072); c.m.w32(a_, v_); };
        {
            l_i_2 = 0i32;
            'l3: loop {
                if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let v_ = ((F::Rand_Range(c, ((32i32) as u32))) as i32); let a_ = (((c.m.r32(sym::gTrain)).wrapping_add(248)).wrapping_add(8)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = 30i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1132); c.m.w32(a_, (v_) as u32); };
        { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1084); c.m.w32(a_, (v_) as u32); };
        { let a0_ = (c.m.r32(sym::gTrain)).wrapping_add(7100); let a1_ = (c.m.r32(sym::gTrain)).wrapping_add(1148); F::Train_BuildLists(c, a0_, a1_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Train_Init", &PROBE_N, &[], true); }
    r_
}

pub fn Train_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Train_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Destroy(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1064); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1064); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1064); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1068); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1068); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1068); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1072); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1072); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1072); c.m.w32(a_, v_); };
    }
    F::MsgWin_Term(c);
    F::IconWin_Term(c);
    if ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gTrain)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gTrain)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gTrain); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gTrain; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Train_Term", &PROBE_N, &[], true); }
    r_
}

pub fn Train_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Train_Update", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Advance(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((!(((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1144); (c.m.r32(a_) as i32) }) & (32i32))) != 0)) as i32)) != 0) {
        { let r_ = 32i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1144); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        F::Train_Plate2Goto(c, 4i32, 1i32);
        if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(2004); (c.m.r32(a_) as i32) }) & (4i32))) != 0) {
            { let a0_ = (c.m.r32(sym::gTrain)).wrapping_add(1092); let a1_ = (c.m.r32(sym::gProgress)).wrapping_add(2012); F::Train_CopyCursor(c, a0_, a1_) };
            F::Train_RestoreScroll(c);
            { let a0_ = { let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1040)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::MsgWin_SetText(c, a0_) };
            { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_h_d__str70, ((((1i32) as i64 as u64)) as u8)) };
            { let v_ = 5i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); c.m.w32(a_, (v_) as u32); };
            { let v_ = ({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1120); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1116); (c.m.r32(a_) as i32) }); let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1128); c.m.w32(a_, (v_) as u32); };
            F::MsgWin_Open(c);
        } else {
            if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(2004); (c.m.r32(a_) as i32) }) & (8i32))) != 0) {
                { let a0_ = (c.m.r32(sym::gTrain)).wrapping_add(1092); let a1_ = (c.m.r32(sym::gProgress)).wrapping_add(2012); F::Train_CopyCursor(c, a0_, a1_) };
                F::Train_RestoreScroll(c);
                { let a0_ = { let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1040)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::MsgWin_SetText(c, a0_) };
                { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_h_d__str71, ((((1i32) as i64 as u64)) as u8)) };
                { let v_ = 2i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); c.m.w32(a_, (v_) as u32); };
                F::Train_RestoreScroll(c);
                F::Train_NextRow(c);
                F::MsgWin_Open(c);
            } else {
                F::MsgWin_Open(c);
                F::IconWin_Open(c);
                { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_h_d__str72, ((((1i32) as i64 as u64)) as u8)) };
                { let a0_ = { let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1040)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::MsgWin_SetText(c, a0_) };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); c.m.w32(a_, (v_) as u32); };
            }
        }
    } else {
        if (((({ let a_ = (((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0) {
            if ((((!(((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1144); (c.m.r32(a_) as i32) }) & (64i32))) != 0)) as i32)) != 0) {
                { let r_ = 64i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1144); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(2004); (c.m.r32(a_) as i32) }) & (16i32))) != 0) {
                    { let a0_ = ({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(2i32); F::menu_h_d__Train_Say(c, a0_) };
                    F::Train_CursorGoto(c, 1i32);
                } else {
                    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(2004); (c.m.r32(a_) as i32) }) & (8i32))) != 0) {
                        { let a0_ = { let a_ = (((c.m.r32(sym::gTrain)).wrapping_add(6728)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(60))).wrapping_add((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1120); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1116); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::menu_h_d__Train_Say(c, a0_) };
                    } else {
                        if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(2004); (c.m.r32(a_) as i32) }) & (4i32))) != 0) {
                            F::menu_h_d__Train_Say(c, 57i32);
                        } else {
                            F::menu_h_d__Train_Say(c, 0i32);
                            F::Train_CursorGoto(c, 1i32);
                        }
                    }
                }
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(2004); c.m.w32(a_, (v_) as u32); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Train_Update", &PROBE_N, &[], true); }
    r_
}

pub fn Train_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Train_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
        l_i_1 = 0i32;
        { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_FindLabel(c, a0_, ((0i32) as u32), sym::menu_h_d__str73, fp) };
        { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a2_ = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1140); F32(c.m.r32(a_)) }; F::Flash_ClipSetAlpha(c, a0_, fp, a2_) };
        F::Train_DrawPlates(c);
        F::Train_DrawArrows(c);
        F::Train_DrawList(c);
        F::Train_DrawClearIcon(c);
        if ((((!(((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1144); (c.m.r32(a_) as i32) }) & (8i32))) != 0)) as i32)) != 0) {
            F::Train_SetClip(c);
        }
        F::Train_DrawGuides(c);
        F::Train_DrawBg(c);
        F::Train_DrawTitle(c);
        {
            'l1: loop {
                if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        F::Train_DrawIconWin(c);
        { let a2_ = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1084); (c.m.r32(a_) as i32) }; F::MsgWin_Draw(c, 0i32, 0i32, a2_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Train_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn Train_CheckLeave(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Train_CheckLeave", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1144); (c.m.r32(a_) as i32) }) & (8i32))) != 0) {
        if ((((!(((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1144); (c.m.r32(a_) as i32) }) & (16i32))) != 0)) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1084); (c.m.r32(a_) as i32) }) != (6i32)) as i32)) != 0) {
                F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                { let r_ = 16i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1144); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            } else {
                if (((((((((((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) || ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0)) as i32)) != 0) || ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0)) as i32)) != 0) {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                        }
                    }
                    F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                    { let r_ = 16i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1144); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
        }
        if (((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1144); (c.m.r32(a_) as i32) }) & (16i32))) != 0) {
            if ((F::ColorFade_IsFadingOut(c)) != 0) {
                F::Voice_FadeOutStep(c, 0i32);
                if ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1080); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::Bgm_FadeOutStep(c);
                }
            } else {
                break 'f 1i32;
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Train_CheckLeave", &PROBE_N, &[], true); }
    r_
}

pub fn menu_h_d__Train_FindLesson(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_h_d__Train_FindLesson", &PROBE_N, &[], false); }
    let mut l_p_1: u32 = 0;
    let mut l_num_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    l_p_1 = ((0i32) as u32);
    l_num_2 = 0i32;
    's1: {
        let sv4: i32 = { let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
        match sv4 {
            0i32 => {
                l_p_1 = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(7100); c.m.r32(a_) };
                l_num_2 = 13i32;
                break 's1;
            }
            1i32 => {
                l_num_2 = 11i32;
                l_p_1 = ({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(7100); c.m.r32(a_) }).wrapping_add((((13i32) as u32)).wrapping_mul(16));
                break 's1;
            }
            2i32 => {
                l_num_2 = 14i32;
                l_p_1 = (({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(7100); c.m.r32(a_) }).wrapping_add((((13i32) as u32)).wrapping_mul(16))).wrapping_add((((11i32) as u32)).wrapping_mul(16));
                break 's1;
            }
            _ => {
            }
        }
    }
    {
        l_i_3 = 0i32;
        'l2: loop {
            if !(((((l_i_3) < (l_num_2)) as i32)) != 0) { break 'l2; }
            'c3: {
                if (((((((c.m.r16((l_p_1).wrapping_add((((l_i_3) as u32)).wrapping_mul(16)))) as i32)) == ({ let a_ = (((c.m.r32(sym::gTrain)).wrapping_add(6548)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(60))).wrapping_add((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1120); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1116); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                    break 'f (l_p_1).wrapping_add((((l_i_3) as u32)).wrapping_mul(16));
                }
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_h_d__Train_FindLesson", &PROBE_N, &[], true); }
    r_
}

pub fn menu_h_d__Train_FindLessonIndex(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_h_d__Train_FindLessonIndex", &PROBE_N, &[], false); }
    let mut l_p_1: u32 = 0;
    let mut l_num_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    l_p_1 = ((0i32) as u32);
    l_num_2 = 0i32;
    's1: {
        let sv4: i32 = { let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
        match sv4 {
            0i32 => {
                l_p_1 = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(7100); c.m.r32(a_) };
                l_num_2 = 13i32;
                break 's1;
            }
            1i32 => {
                l_num_2 = 11i32;
                l_p_1 = ({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(7100); c.m.r32(a_) }).wrapping_add((((13i32) as u32)).wrapping_mul(16));
                break 's1;
            }
            2i32 => {
                l_num_2 = 14i32;
                l_p_1 = (({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(7100); c.m.r32(a_) }).wrapping_add((((13i32) as u32)).wrapping_mul(16))).wrapping_add((((11i32) as u32)).wrapping_mul(16));
                break 's1;
            }
            _ => {
            }
        }
    }
    {
        l_i_3 = 0i32;
        'l2: loop {
            if !(((((l_i_3) < (l_num_2)) as i32)) != 0) { break 'l2; }
            'c3: {
                if (((((((c.m.r16((l_p_1).wrapping_add((((l_i_3) as u32)).wrapping_mul(16)))) as i32)) == ({ let a_ = (((c.m.r32(sym::gTrain)).wrapping_add(6548)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(60))).wrapping_add((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1120); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1116); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                    break 'l2;
                }
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    break 'f l_i_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_h_d__Train_FindLessonIndex", &PROBE_N, &[], true); }
    r_
}

pub fn menu_h_d__Train_PageHasImage(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_h_d__Train_PageHasImage", &PROBE_N, &[], false); }
    let mut l_i_1: u32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = ((0i32) as u32);
        'l1: loop {
            if !(((((l_i_1) < ({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(7100)).wrapping_add(16); c.m.r32(a_) })) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1084); (c.m.r32(a_) as i32) }) == ({ let a_ = ({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(7100)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((l_i_1).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                    break 'f 0i32;
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_h_d__Train_PageHasImage", &PROBE_N, &[], true); }
    r_
}

pub fn Train_Input(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Train_Input", &PROBE_N, &[], false); }
    let mut l_done_1: i32 = 0;
    let mut l_flags_2: u8 = 0;
    let r_ = 'f: {
    if ((((!(((({ let a_ = (((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f;
    }
    if (((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1144); (c.m.r32(a_) as i32) }) & (8i32))) != 0) {
        break 'f;
    }
    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
        break 'f;
    }
    's1: {
        let sv3: i32 = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); (c.m.r32(a_) as i32) };
        match sv3 {
            0i32 => {
                if (((((((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1084); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                    F::menu_h_d__Train_Say(c, 1i32);
                } else {
                    if (((((((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1084); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0)) as i32)) != 0) {
                        { let a0_ = ({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(2i32); F::menu_h_d__Train_Say(c, a0_) };
                    }
                }
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                    F::Train_MoveCursor(c, 0i32);
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                        F::Train_MoveCursor(c, 1i32);
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                            's2: {
                                let sv4: i32 = { let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                                match sv4 {
                                    0i32 => {
                                        { let v_ = 45i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1080); c.m.w32(a_, (v_) as u32); };
                                        { let r_ = 8i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1144); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                        { let a0_ = (c.m.r32(sym::gProgress)).wrapping_add(2012); let a1_ = (c.m.r32(sym::gTrain)).wrapping_add(1092); F::Train_CopyCursor(c, a0_, a1_) };
                                        { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_h_d__str74, ((((1i32) as i64 as u64)) as u8)) };
                                        { let r_ = 16i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(2004); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                        break 's2;
                                    }
                                    1i32 => {
                                        F::Train_CursorGoto(c, 0i32);
                                        { let v_ = 1i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); c.m.w32(a_, (v_) as u32); };
                                        { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_h_d__str75, ((((1i32) as i64 as u64)) as u8)) };
                                        F::Train_CursorGoto(c, 1i32);
                                        F::menu_h_d__Train_Say(c, 0i32);
                                        { let a0_ = { let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1040)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::MsgWin_SetText(c, a0_) };
                                        break 's2;
                                    }
                                    _ => {
                                    }
                                }
                            }
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_h_d__str76, ((((1i32) as i64 as u64)) as u8)) };
                                { let r_ = 8i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1144); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1080); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                                F::menu_h_d__Train_Say(c, 6i32);
                            }
                        }
                    }
                }
                break 's1;
            }
            1i32 => {
                if (((((((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1084); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                    { let a0_ = ({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::menu_h_d__Train_Say(c, a0_) };
                }
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                    F::Train_MoveCursor(c, 0i32);
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                        F::Train_MoveCursor(c, 1i32);
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                            F::Train_CursorGoto(c, 0i32);
                            F::Train_RestoreScroll(c);
                            { let v_ = 2i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); c.m.w32(a_, (v_) as u32); };
                            { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_h_d__str77, ((((1i32) as i64 as u64)) as u8)) };
                            F::IconWin_Close(c);
                            { let a0_ = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1120); (c.m.r32(a_) as i32) }; F::Train_Plate2Goto(c, a0_, 1i32) };
                            F::menu_h_d__Train_Say(c, 9i32);
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                F::Train_CursorGoto(c, 0i32);
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); c.m.w32(a_, (v_) as u32); };
                                { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_h_d__str78, ((((1i32) as i64 as u64)) as u8)) };
                                F::Train_CursorGoto(c, 1i32);
                                { let a0_ = ({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(2i32); F::menu_h_d__Train_Say(c, a0_) };
                                { let a0_ = { let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1040)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::MsgWin_SetText(c, a0_) };
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                            }
                        }
                    }
                }
                break 's1;
            }
            2i32 => {
                if (((((((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1084); (c.m.r32(a_) as i32) }) == (9i32)) as i32)) != 0)) as i32)) != 0) {
                    { let a0_ = { let a_ = (((c.m.r32(sym::gTrain)).wrapping_add(6728)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(60))).wrapping_add((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1120); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1116); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::menu_h_d__Train_Say(c, a0_) };
                }
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                    F::Train_MoveCursor(c, 0i32);
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                        F::Train_MoveCursor(c, 1i32);
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                            { let a0_ = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1120); (c.m.r32(a_) as i32) }; F::Train_Plate2Goto(c, a0_, 0i32) };
                            { let v_ = 3i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); c.m.w32(a_, (v_) as u32); };
                            { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_h_d__str79, ((((1i32) as i64 as u64)) as u8)) };
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                            { let a0_ = (((F::Rand_Range(c, ((4i32) as u32))).wrapping_add(((53i32) as u32))) as i32); F::menu_h_d__Train_Say(c, a0_) };
                            { let v_ = 1i32; let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1032)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            { let v_ = ({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1120); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1116); (c.m.r32(a_) as i32) }); let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1128); c.m.w32(a_, (v_) as u32); };
                        }
                    }
                }
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                    { let a0_ = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1120); (c.m.r32(a_) as i32) }; F::Train_Plate2Goto(c, a0_, 0i32) };
                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); c.m.w32(a_, (v_) as u32); };
                    { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_h_d__str80, ((((1i32) as i64 as u64)) as u8)) };
                    F::Train_CursorGoto(c, 1i32);
                    F::Train_SaveCursor(c);
                    F::IconWin_Open(c);
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                    { let a0_ = ({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::menu_h_d__Train_Say(c, a0_) };
                }
                break 's1;
            }
            3i32 => {
                if ((((((((((((((((((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) || (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1084); (c.m.r32(a_) as i32) }) == (53i32)) as i32)) != 0)) as i32)) != 0) || ((((((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) || (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1084); (c.m.r32(a_) as i32) }) == (54i32)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) || ((((((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) || (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1084); (c.m.r32(a_) as i32) }) == (55i32)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) || ((((((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) || (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1084); (c.m.r32(a_) as i32) }) == (56i32)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                    { let v_ = 4i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); c.m.w32(a_, (v_) as u32); };
                    { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_h_d__str81, ((((1i32) as i64 as u64)) as u8)) };
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1088); c.m.w32(a_, (v_) as u32); };
                    F::MsgWin_SetBoxParam(c, 0i32, 5i32);
                    F::FontIcon_SetPadType(c, 0i32);
                    { let a0_ = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1076); c.m.r32(a_) }; F::MsgWin_SetText(c, a0_) };
                    { let v_ = { let a_ = ((((c.m.r32(sym::gTrain)).wrapping_add(1148)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1800))).wrapping_add((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1120); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1116); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(120))).wrapping_add(((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1088); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1084); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a_ = (((c.m.r32(sym::gTrain)).wrapping_add(6908)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(60))).wrapping_add((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1120); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1116); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(7128); c.m.w32(a_, (v_) as u32); };
                    F::FontIcon_ResetAnim(c);
                    { let r_ = 1i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1144); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
                break 's1;
            }
            4i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                    { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1088); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1032)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    if ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1088); (c.m.r32(a_) as i32) }) < ({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(7128); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        { let v_ = { let a_ = ((((c.m.r32(sym::gTrain)).wrapping_add(1148)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1800))).wrapping_add((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1120); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1116); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(120))).wrapping_add(((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1088); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1084); c.m.w32(a_, (v_) as u32); };
                        if ((F::menu_h_d__Train_PageHasImage(c)) != 0) {
                            { let r_ = 1i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1144); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_h_d__str82, ((((1i32) as i64 as u64)) as u8)) };
                        }
                        F::FontIcon_ResetAnim(c);
                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 41i32); };
                    } else {
                        l_flags_2 = { let a_ = (F::menu_h_d__Train_FindLesson(c)).wrapping_add(3); c.m.r8(a_) };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                        if ((((((((((((((((((l_flags_2) as i32)) & (1i32))) as u8)) as i32)) != 0) || ((((((l_flags_2) as i32)) & (2i32))) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1084); (c.m.r32(a_) as i32) }) == (40i32)) as i32)) != 0)) as i32)) != 0) {
                            { let v_ = 1i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1080); c.m.w32(a_, (v_) as u32); };
                            F::Train_SaveCursor(c);
                            { let a0_ = (c.m.r32(sym::gProgress)).wrapping_add(2012); let a1_ = (c.m.r32(sym::gTrain)).wrapping_add(1092); F::Train_CopyCursor(c, a0_, a1_) };
                            { let r_ = 8i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1144); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            if (({ let a0_ = { let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a1_ = ({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1120); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1116); (c.m.r32(a_) as i32) }); F::Train_IsCleared(c, a0_, a1_) }) != 0) {
                                { let r_ = 8i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(2004); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            } else {
                                { let r_ = 4i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(2004); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            }
                        } else {
                            if ((((((((((((((l_flags_2) as i32)) & (1i32))) as u8)) as i32)) != 0) || ((((((l_flags_2) as i32)) & (2i32))) != 0)) as i32)) != 0) {
                                { let v_ = 40i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1084); c.m.w32(a_, (v_) as u32); };
                                { let a0_ = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1056); c.m.r32(a_) }; F::MsgWin_SetText(c, a0_) };
                            } else {
                                { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1084); c.m.w32(a_, (v_) as u32); };
                                F::MsgWin_SetBoxParam(c, 0i32, 0i32);
                                { let a0_ = { let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1040)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::MsgWin_SetText(c, a0_) };
                                if (({ let a0_ = { let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a1_ = ({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1120); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1116); (c.m.r32(a_) as i32) }); F::Train_IsCleared(c, a0_, a1_) }) != 0) {
                                    { let v_ = 2i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); c.m.w32(a_, (v_) as u32); };
                                    { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_h_d__str83, ((((1i32) as i64 as u64)) as u8)) };
                                    F::Train_NextRow(c);
                                    { let a0_ = { let a_ = (((c.m.r32(sym::gTrain)).wrapping_add(6728)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(60))).wrapping_add((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1120); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1116); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::menu_h_d__Train_Say(c, a0_) };
                                } else {
                                    { let v_ = 5i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); c.m.w32(a_, (v_) as u32); };
                                    { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_h_d__str84, ((((1i32) as i64 as u64)) as u8)) };
                                    F::menu_h_d__Train_Say(c, 57i32);
                                }
                            }
                        }
                    }
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                        { let v_ = 0i32; let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1032)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 2i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); c.m.w32(a_, (v_) as u32); };
                        { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_h_d__str85, ((((1i32) as i64 as u64)) as u8)) };
                        { let a0_ = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1120); (c.m.r32(a_) as i32) }; F::Train_Plate2Goto(c, a0_, 1i32) };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                        F::MsgWin_SetBoxParam(c, 0i32, 0i32);
                        { let a0_ = { let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1040)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::MsgWin_SetText(c, a0_) };
                        { let a0_ = { let a_ = (((c.m.r32(sym::gTrain)).wrapping_add(6728)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(60))).wrapping_add((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1120); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1116); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::menu_h_d__Train_Say(c, a0_) };
                    }
                }
                break 's1;
            }
            5i32 => {
                if ((((!(((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1144); (c.m.r32(a_) as i32) }) & (512i32))) != 0)) as i32)) != 0) {
                    { let r_ = (1i32).wrapping_shl((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1120); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1116); (c.m.r32(a_) as i32) })) as u32); let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3596)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let r_ = 512i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1144); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
                l_done_1 = 0i32;
                if ((((((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) || (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1084); (c.m.r32(a_) as i32) }) == (57i32)) as i32)) != 0)) as i32)) != 0) {
                    if (({ let a0_ = { let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::Train_IsClassCleared(c, a0_) }) != 0) {
                        { let a0_ = ({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(58i32); F::menu_h_d__Train_Say(c, a0_) };
                    } else {
                        l_done_1 = 1i32;
                    }
                } else {
                    if ((((((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) || (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1084); (c.m.r32(a_) as i32) }) == (({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(58i32))) as i32)) != 0)) as i32)) != 0) {
                        if ((F::Train_IsAllCleared(c)) != 0) {
                            F::menu_h_d__Train_Say(c, 61i32);
                        } else {
                            l_done_1 = 1i32;
                        }
                    } else {
                        if ((((((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) || (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1084); (c.m.r32(a_) as i32) }) == (61i32)) as i32)) != 0)) as i32)) != 0) {
                            if ((F::Train_IsAllCleared(c)) != 0) {
                                F::menu_h_d__Train_Say(c, 62i32);
                            } else {
                                l_done_1 = 1i32;
                            }
                        } else {
                            if ((((((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) || (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1084); (c.m.r32(a_) as i32) }) == (62i32)) as i32)) != 0)) as i32)) != 0) {
                                l_done_1 = 1i32;
                            }
                        }
                    }
                }
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                }
                if ((l_done_1) != 0) {
                    { let v_ = 2i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); c.m.w32(a_, (v_) as u32); };
                    { let a0_ = ((c.m.r32(sym::gTrain)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_h_d__str86, ((((1i32) as i64 as u64)) as u8)) };
                    if (((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1144); (c.m.r32(a_) as i32) }) & (512i32))) != 0) {
                        { let r_ = 512i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1144); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    F::Train_NextRow(c);
                    { let a0_ = { let a_ = (((c.m.r32(sym::gTrain)).wrapping_add(6728)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(60))).wrapping_add((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1120); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1116); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::menu_h_d__Train_Say(c, a0_) };
                }
                break 's1;
            }
            _ => {
            }
        }
    }
    if (((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) != (((0i32) as u32))) as i32)) != 0) || (((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1136); c.m.w32(a_, (v_) as u32); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1136); (c.m.r32(a_) as i32) }) > (3600i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1136); c.m.w32(a_, (v_) as u32); };
        { let r_ = 128i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1144); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            { let a0_ = (((F::Rand_Range(c, ((2i32) as u32))).wrapping_add(((4i32) as u32))) as i32); F::menu_h_d__Train_Say(c, a0_) };
            if ((((({ let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1032)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1032)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = 1i32; let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1032)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            }
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); (c.m.r32(a_) as i32) }) < (3i32)) as i32)) != 0) {
                { let a0_ = (((F::Rand_Range(c, ((2i32) as u32))).wrapping_add(((4i32) as u32))) as i32); F::menu_h_d__Train_Say(c, a0_) };
            }
        }
    } else {
        { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1136); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    }
    if (((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1144); (c.m.r32(a_) as i32) }) & (128i32))) != 0) {
        if ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1112); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                if ((((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1084); (c.m.r32(a_) as i32) }) as u32)) == ((F::Rand_Range(c, ((2i32) as u32))).wrapping_add(((4i32) as u32)))) as i32)) != 0) {
                    { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 40i32); };
                    if (((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1144); (c.m.r32(a_) as i32) }) & (128i32))) != 0) {
                        { let r_ = 128i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1144); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Train_Input", &PROBE_N, &[], true); }
    r_
}

pub fn Train_UpdateImage(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Train_UpdateImage", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        if (((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1144); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
            { let v_ = F32(0x00000000); let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1140); c.m.w32(a_, (v_).0); };
            if (((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1144); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                { let r_ = 1i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1144); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
            }
            { let r_ = 2i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1144); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            F::File_CancelRequests(c);
            { let _ = { let a0_ = ({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1084); (c.m.r32(a_) as i32) }).wrapping_add(1026i32); let a1_ = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1064); c.m.r32(a_) }; let _ = 57344i32; F::File_Request(c, a0_, a1_) }; };
        }
        if (((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1144); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
            if ((F::File_UpdateRequests(c)) != 0) {
                { let v_ = ((0i32) as u32); let a_ = fp; c.m.w32(a_, v_); };
                if (((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1144); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
                    { let r_ = 2i32; let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1144); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                }
                { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1064); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1068); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, a1_, ((0i32) as u32)) }; };
                { let v_ = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1068); c.m.r32(a_) }; let a_ = fp; c.m.w32(a_, v_); };
                { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
                { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(56)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            }
        } else {
            { let r_ = F32(0x3d4ccccc); let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1140); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::fgt({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1140); F32(c.m.r32(a_)) }, F32(0x3f800000)) as i32)) != 0) {
                { let v_ = F32(0x3f800000); let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1140); c.m.w32(a_, (v_).0); };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Train_UpdateImage", &PROBE_N, &[], true); }
    r_
}

pub fn Train_Leave(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Train_Leave", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(160) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 160);
    let mut l_flags_1: u8 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_rec_3: u32 = 0;
    let mut l_bgm_4: i32 = 0;
    let mut l_stage_5: i32 = 0;
    let mut l_cpuLevel_6: i32 = 0;
    let mut l_chara_7: u32 = 0;
    let mut l_index_8: i32 = 0;
    let r_ = 'f: {
        if ((((({ let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1080); (c.m.r32(a_) as i32) }) != (1i32)) as i32)) != 0) {
            break 'f;
        }
        l_flags_1 = { let a_ = (F::menu_h_d__Train_FindLesson(c)).wrapping_add(3); c.m.r8(a_) };
        if ((((((l_flags_1) as i32)) & (2i32))) != 0) {
            rt::memset(c, fp, 0, 96);
            rt::memset(c, fp.wrapping_add(0), 0, 12);
            { let v_ = 7i32; c.m.w32(fp.wrapping_add(0).wrapping_add(0), (v_) as u32); }
            { let v_ = (1i32).wrapping_neg(); c.m.w32(fp.wrapping_add(0).wrapping_add(4), (v_) as u32); }
            { let v_ = 137i32; c.m.w32(fp.wrapping_add(0).wrapping_add(8), (v_) as u32); }
            rt::memset(c, fp.wrapping_add(12), 0, 12);
            { let v_ = 17i32; c.m.w32(fp.wrapping_add(12).wrapping_add(0), (v_) as u32); }
            { let v_ = 15i32; c.m.w32(fp.wrapping_add(12).wrapping_add(4), (v_) as u32); }
            { let v_ = 153i32; c.m.w32(fp.wrapping_add(12).wrapping_add(8), (v_) as u32); }
            rt::memset(c, fp.wrapping_add(24), 0, 12);
            { let v_ = 21i32; c.m.w32(fp.wrapping_add(24).wrapping_add(0), (v_) as u32); }
            { let v_ = (1i32).wrapping_neg(); c.m.w32(fp.wrapping_add(24).wrapping_add(4), (v_) as u32); }
            { let v_ = 137i32; c.m.w32(fp.wrapping_add(24).wrapping_add(8), (v_) as u32); }
            rt::memset(c, fp.wrapping_add(36), 0, 12);
            { let v_ = 22i32; c.m.w32(fp.wrapping_add(36).wrapping_add(0), (v_) as u32); }
            { let v_ = (1i32).wrapping_neg(); c.m.w32(fp.wrapping_add(36).wrapping_add(4), (v_) as u32); }
            { let v_ = 137i32; c.m.w32(fp.wrapping_add(36).wrapping_add(8), (v_) as u32); }
            rt::memset(c, fp.wrapping_add(48), 0, 12);
            { let v_ = 25i32; c.m.w32(fp.wrapping_add(48).wrapping_add(0), (v_) as u32); }
            { let v_ = 15i32; c.m.w32(fp.wrapping_add(48).wrapping_add(4), (v_) as u32); }
            { let v_ = 153i32; c.m.w32(fp.wrapping_add(48).wrapping_add(8), (v_) as u32); }
            rt::memset(c, fp.wrapping_add(60), 0, 12);
            { let v_ = 26i32; c.m.w32(fp.wrapping_add(60).wrapping_add(0), (v_) as u32); }
            { let v_ = (1i32).wrapping_neg(); c.m.w32(fp.wrapping_add(60).wrapping_add(4), (v_) as u32); }
            { let v_ = 137i32; c.m.w32(fp.wrapping_add(60).wrapping_add(8), (v_) as u32); }
            rt::memset(c, fp.wrapping_add(72), 0, 12);
            { let v_ = 27i32; c.m.w32(fp.wrapping_add(72).wrapping_add(0), (v_) as u32); }
            { let v_ = 15i32; c.m.w32(fp.wrapping_add(72).wrapping_add(4), (v_) as u32); }
            { let v_ = 154i32; c.m.w32(fp.wrapping_add(72).wrapping_add(8), (v_) as u32); }
            rt::memset(c, fp.wrapping_add(84), 0, 12);
            { let v_ = 28i32; c.m.w32(fp.wrapping_add(84).wrapping_add(0), (v_) as u32); }
            { let v_ = (1i32).wrapping_neg(); c.m.w32(fp.wrapping_add(84).wrapping_add(4), (v_) as u32); }
            { let v_ = 137i32; c.m.w32(fp.wrapping_add(84).wrapping_add(8), (v_) as u32); }
            l_rec_3 = F::menu_h_d__Train_FindLesson(c);
            l_bgm_4 = ((c.m.r8((l_rec_3).wrapping_add(13))) as i32);
            l_stage_5 = ((c.m.r8((l_rec_3).wrapping_add(12))) as i32);
            l_i_2 = 0i32;
            { let _ = F::memset(c, fp.wrapping_add(96), 0i32, 16u32); };
            F::Battle_ClearWork(c);
            l_cpuLevel_6 = (999i32).wrapping_neg();
            {
                'l1: loop {
                    'c2: {
                        if (((((l_i_2) >= (8i32)) as i32)) != 0) {
                            break 'l1;
                        }
                        if ((((((c.m.r32(((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) == (((c.m.r16(l_rec_3)) as i32))) as i32)) != 0) {
                            l_cpuLevel_6 = (c.m.r32(((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32);
                            { let v_ = (((c.m.r32(((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u16); let a_ = (fp.wrapping_add(96)).wrapping_add((((7i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
                            break 'l1;
                        }
                    }
                    { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                }
            }
            if (((((l_cpuLevel_6) != ((999i32).wrapping_neg())) as i32)) != 0) {
                F::ItemSet_GetStats(c, fp.wrapping_add(96), fp.wrapping_add(112), fp.wrapping_add(144), 114i32);
            }
            F::BattleSetup_SetRule(c, 0i32, 5i32, l_bgm_4, 0i32, 7i32, l_stage_5, 0i32);
            F::BattleSetup_SetSide(c, 0i32, 0i32, 0i32, 1i32, 1i32, 1i32, 0i32, ((((0i32) as i64 as u64)) as u32));
            F::BattleSetup_SetSide(c, 1i32, 2i32, 1i32, 1i32, 1i32, 1i32, 0i32, ((((0i32) as i64 as u64)) as u32));
            {
                l_chara_7 = (l_rec_3).wrapping_add(14);
                {
                    l_i_2 = 0i32;
                    'l3: loop {
                        if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l3; }
                        'c4: {
                            { let a2_ = ((c.m.r8({ let o_ = l_chara_7; let n_ = (o_).wrapping_add(1); l_chara_7 = n_; o_ })) as i32); F::BattleSetup_SetMember(c, l_i_2, 0i32, a2_, 0i32, 0i32, l_cpuLevel_6, F32(0x42c80000), fp.wrapping_add(96)) };
                        }
                        { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                    }
                }
            }
            F::BattleSetup_Finish(c);
            { let r_ = 2i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(2004); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if ((((((((l_flags_1) as i32)) & (1i32))) as u8)) != 0) {
            l_index_8 = F::menu_h_d__Train_FindLessonIndex(c);
            's5: {
                let sv9: i32 = { let a_ = ((c.m.r32(sym::gTrain)).wrapping_add(1092)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                match sv9 {
                    0i32 => {
                        { let v_ = l_index_8; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(2036); c.m.w32(a_, (v_) as u32); };
                        break 's5;
                    }
                    1i32 => {
                        { let v_ = (l_index_8).wrapping_add(13i32); let a_ = (c.m.r32(sym::gProgress)).wrapping_add(2036); c.m.w32(a_, (v_) as u32); };
                        break 's5;
                    }
                    2i32 => {
                        { let v_ = ((l_index_8).wrapping_add(13i32)).wrapping_add(11i32); let a_ = (c.m.r32(sym::gProgress)).wrapping_add(2036); c.m.w32(a_, (v_) as u32); };
                        break 's5;
                    }
                    _ => {
                    }
                }
            }
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(2040); c.m.w32(a_, (v_) as u32); };
            { let r_ = 1i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(2004); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Train_Leave", &PROBE_N, &[], true); }
    r_
}

pub fn Train_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Train_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let mut l_result_2: i32 = 0;
    let r_ = 'f: {
    F::Train_Init(c, p_section_1);
    F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
    'l1: while ((1i32) != 0) {
        F::Gfx_BeginFrame(c);
        F::Pad_Update(c);
        F::ColorFade_Update(c);
        F::Snd_Update(c);
        F::Train_Update(c);
        F::Train_Draw(c);
        F::ColorFade_Draw(c);
        F::Gfx_EndFrame(c, 1i32);
        F::Dma_Flush(c);
        F::File_Stub264D90(c);
        if ((F::Train_CheckLeave(c)) != 0) {
            break 'l1;
        }
        F::Train_Input(c);
        F::Train_UpdateImage(c);
    }
    l_result_2 = { let a_ = (c.m.r32(sym::gTrain)).wrapping_add(1080); (c.m.r32(a_) as i32) };
    F::Train_Leave(c);
    F::Train_Term(c);
    F::Dma_ResetBuffers(c);
    break 'f l_result_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Train_Run", &PROBE_N, &[], true); }
    r_
}
