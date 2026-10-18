#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Dialog_DrawText(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dialog_DrawText", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_flash_1: u32 = 0;
    let mut l_str_2: u32 = 0;
    let mut l_alpha_3: F32 = F32(0);
    let mut l_lines_4: i32 = 0;
    let r_ = 'f: {
        if ((((({ let a_ = (c.m.r32(sym::gDialog)).wrapping_add(4); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if ((((({ let a_ = (c.m.r32(sym::gDialog)).wrapping_add(108); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
            break 'f;
        }
        l_flash_1 = (c.m.r32(sym::gDialog)).wrapping_add(8);
        's1: {
            let sv5: i32 = { let a_ = (c.m.r32(sym::gDialog)).wrapping_add(116); (c.m.r32(a_) as i32) };
            match sv5 {
                0i32 => {
                    F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::dialog__str0, fp);
                    break 's1;
                }
                1i32 => {
                    F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::dialog__str1, fp);
                    break 's1;
                }
                _ => {
                }
            }
        }
        if ((((((c.m.r32(fp) as i32)) < (0i32)) as i32)) != 0) {
            break 'f;
        }
        F::Flash_ClipGetPos(c, l_flash_1, fp, fp.wrapping_add(8), fp.wrapping_add(12));
        l_alpha_3 = F::Flash_ClipGetAlpha(c, l_flash_1, fp);
        l_str_2 = ({ let a_ = (c.m.r32(sym::gDialog)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((rt::div_u32({ let a_ = ({ let a_ = (c.m.r32(sym::gDialog)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((({ let a_ = (c.m.r32(sym::gDialog)).wrapping_add(108); (c.m.r32(a_) as i32) }).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }, ((4i32) as u32))).wrapping_mul(((4i32) as u32)));
        l_lines_4 = F::Font_CountLines(c, l_str_2);
        's2: {
            let sv6: i32 = { let a_ = (c.m.r32(sym::gDialog)).wrapping_add(116); (c.m.r32(a_) as i32) };
            match sv6 {
                0i32 => {
                    's3: {
                        let sv7: i32 = l_lines_4;
                        match sv7 {
                            1i32 => {
                                { let r_ = 65i32; let a_ = fp.wrapping_add(12); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                                break 's3;
                            }
                            2i32 => {
                                { let r_ = 52i32; let a_ = fp.wrapping_add(12); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                                break 's3;
                            }
                            3i32 => {
                                { let r_ = 39i32; let a_ = fp.wrapping_add(12); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                                break 's3;
                            }
                            4i32 => {
                                { let r_ = 26i32; let a_ = fp.wrapping_add(12); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                                break 's3;
                            }
                            5i32 => {
                                { let r_ = 13i32; let a_ = fp.wrapping_add(12); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                                break 's3;
                            }
                            6i32 => {
                                break 's3;
                            }
                            _ => {
                            }
                        }
                    }
                    break 's2;
                }
                1i32 => {
                    's4: {
                        let sv8: i32 = l_lines_4;
                        match sv8 {
                            6i32 => {
                                { let r_ = 26i32; let a_ = fp.wrapping_add(12); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                                break 's4;
                            }
                            7i32 => {
                                { let r_ = 13i32; let a_ = fp.wrapping_add(12); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                                break 's4;
                            }
                            8i32 => {
                                break 's4;
                            }
                            _ => {
                            }
                        }
                    }
                    break 's2;
                }
                _ => {
                }
            }
        }
        F::Font_PushStyle(c);
        F::Font_SetAlign(c, 1i32);
        F::Font_SetScaleXY(c, F32(0x3f800000), F32(0x3f800000));
        F::Font_SetColorRGBA(c, ((((255i32) as i64 as u64)) as u8), ((((255i32) as i64 as u64)) as u8), ((((255i32) as i64 as u64)) as u8), ((((rt::f2u(rt::fmul(l_alpha_3, F32(0x43000000)))) as u8)) as i32));
        F::Font_SetShadowMode(c, 2i32);
        F::Font_SetShadowOffset(c, 1i32, 2i32);
        F::Font_SetShadowColor(c, (((rt::f2u(rt::fmul(l_alpha_3, F32(0x42800000)))).wrapping_shl((24i32) as u32)) | (((2105376i32) as u32))));
        { let a0_ = (c.m.r32(fp.wrapping_add(8)) as i32); let a1_ = (c.m.r32(fp.wrapping_add(12)) as i32); F::Font_PrintAt(c, a0_, a1_, l_str_2) };
        F::Font_PopStyle(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Dialog_DrawText", &PROBE_N, &[], true); }
    r_
}

pub fn Dialog_DrawTitle(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dialog_DrawTitle", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_flash_1: u32 = 0;
    let mut l_str_2: u32 = 0;
    let mut l_alpha_3: F32 = F32(0);
    let r_ = 'f: {
        if ((((({ let a_ = c.m.r32(sym::gDialog); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if ((((({ let a_ = (c.m.r32(sym::gDialog)).wrapping_add(124); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
            break 'f;
        }
        l_flash_1 = (c.m.r32(sym::gDialog)).wrapping_add(8);
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::dialog__str2, fp);
        if ((((((c.m.r32(fp) as i32)) < (0i32)) as i32)) != 0) {
            break 'f;
        }
        F::Flash_ClipGetPos(c, l_flash_1, fp, fp.wrapping_add(8), fp.wrapping_add(12));
        l_alpha_3 = F::Flash_ClipGetAlpha(c, l_flash_1, fp);
        l_str_2 = ({ let a_ = c.m.r32(sym::gDialog); c.m.r32(a_) }).wrapping_add((rt::div_u32({ let a_ = ({ let a_ = c.m.r32(sym::gDialog); c.m.r32(a_) }).wrapping_add((((({ let a_ = (c.m.r32(sym::gDialog)).wrapping_add(124); (c.m.r32(a_) as i32) }).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }, ((4i32) as u32))).wrapping_mul(((4i32) as u32)));
        F::Font_PushStyle(c);
        F::Font_SetAlign(c, 1i32);
        F::Font_SetScaleXY(c, F32(0x3f800000), F32(0x3f800000));
        F::Font_SetColorRGBA(c, ((((255i32) as i64 as u64)) as u8), ((((255i32) as i64 as u64)) as u8), ((((255i32) as i64 as u64)) as u8), ((((rt::f2u(rt::fmul(l_alpha_3, F32(0x43000000)))) as u8)) as i32));
        F::Font_SetShadowMode(c, 2i32);
        F::Font_SetShadowOffset(c, 1i32, 2i32);
        F::Font_SetShadowColor(c, (((rt::f2u(rt::fmul(l_alpha_3, F32(0x42800000)))).wrapping_shl((24i32) as u32)) | (((2105376i32) as u32))));
        { let a0_ = (c.m.r32(fp.wrapping_add(8)) as i32); let a1_ = (c.m.r32(fp.wrapping_add(12)) as i32); F::Font_PrintAt(c, a0_, a1_, l_str_2) };
        F::Font_PopStyle(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Dialog_DrawTitle", &PROBE_N, &[], true); }
    r_
}

pub fn Dialog_DrawBody(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dialog_DrawBody", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_flash_1: u32 = 0;
    let mut l_str_2: u32 = 0;
    let mut l_alpha_3: F32 = F32(0);
    let mut l_scale_4: F32 = F32(0);
    let r_ = 'f: {
        if ((((({ let a_ = c.m.r32(sym::gDialog); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if ((((({ let a_ = (c.m.r32(sym::gDialog)).wrapping_add(108); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
            break 'f;
        }
        l_flash_1 = (c.m.r32(sym::gDialog)).wrapping_add(8);
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::dialog__str3, fp);
        if ((((((c.m.r32(fp) as i32)) < (0i32)) as i32)) != 0) {
            break 'f;
        }
        F::Flash_ClipGetPos(c, l_flash_1, fp, fp.wrapping_add(8), fp.wrapping_add(12));
        l_alpha_3 = F::Flash_ClipGetAlpha(c, l_flash_1, fp);
        l_str_2 = ({ let a_ = c.m.r32(sym::gDialog); c.m.r32(a_) }).wrapping_add((rt::div_u32({ let a_ = ({ let a_ = c.m.r32(sym::gDialog); c.m.r32(a_) }).wrapping_add((((({ let a_ = (c.m.r32(sym::gDialog)).wrapping_add(108); (c.m.r32(a_) as i32) }).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }, ((4i32) as u32))).wrapping_mul(((4i32) as u32)));
        's1: {
            let sv5: i32 = F::Font_CountLines(c, l_str_2);
            match sv5 {
                1i32 => {
                    { let r_ = 40i32; let a_ = fp.wrapping_add(12); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                    break 's1;
                }
                2i32 => {
                    { let r_ = 32i32; let a_ = fp.wrapping_add(12); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                    break 's1;
                }
                3i32 => {
                    { let r_ = 20i32; let a_ = fp.wrapping_add(12); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                    break 's1;
                }
                4i32 => {
                    { let r_ = 10i32; let a_ = fp.wrapping_add(12); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                    break 's1;
                }
                5i32 => {
                    { let r_ = 4i32; let a_ = fp.wrapping_add(12); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                    break 's1;
                }
                6i32 => {
                    break 's1;
                }
                _ => {
                }
            }
        }
        l_scale_4 = F32(0x3f800000);
        { let _ = F::Font_GetWidth(c, l_str_2); };
        F::Font_PushStyle(c);
        F::Font_SetAlign(c, 1i32);
        F::Font_SetScaleXY(c, l_scale_4, l_scale_4);
        F::Font_SetColorRGBA(c, ((((255i32) as i64 as u64)) as u8), ((((255i32) as i64 as u64)) as u8), ((((255i32) as i64 as u64)) as u8), ((((rt::f2u(rt::fmul(l_alpha_3, F32(0x43000000)))) as u8)) as i32));
        F::Font_SetShadowMode(c, 2i32);
        F::Font_SetShadowOffset(c, 1i32, 2i32);
        F::Font_SetShadowColor(c, (((rt::f2u(rt::fmul(l_alpha_3, F32(0x42800000)))).wrapping_shl((24i32) as u32)) | (((2105376i32) as u32))));
        { let a0_ = (c.m.r32(fp.wrapping_add(8)) as i32); let a1_ = (c.m.r32(fp.wrapping_add(12)) as i32); F::Font_PrintAt(c, a0_, a1_, l_str_2) };
        F::Font_PopStyle(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Dialog_DrawBody", &PROBE_N, &[], true); }
    r_
}

pub fn Dialog_PlayCursorPlate(c: &mut Ctx, mut p_flash_1: i32, mut p_label_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dialog_PlayCursorPlate", &PROBE_N, &[(p_flash_1) as u32, (p_label_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_obj_3: u32 = 0;
    let r_ = 'f: {
        l_obj_3 = ((c.m.r32(sym::gDialog)).wrapping_add(8)).wrapping_add((((p_flash_1) as u32)).wrapping_mul(44));
        { let _ = { let a2_ = ({ let a_ = (c.m.r32(sym::gDialog)).wrapping_add(104); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::dialog__str4, &[((a2_) as i64 as u64)]) }; };
        F::Flash_FindLabel(c, l_obj_3, ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, l_obj_3, fp, p_label_2);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Dialog_PlayCursorPlate", &PROBE_N, &[], true); }
    r_
}

pub fn Dialog_Init(c: &mut Ctx, mut p_file_1: u32, mut p_msgTbl_2: u32, mut p_size_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dialog_Init", &PROBE_N, &[(p_file_1) as u32, (p_msgTbl_2) as u32, (p_size_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = F::Heap_Alloc(c, ((140u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gDialog; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gDialog); F::memset(c, a0_, 0i32, 140u32) }; };
        { let v_ = (p_file_1).wrapping_add((rt::div_u32(c.m.r32((p_file_1).wrapping_add((((1i32) as u32)).wrapping_mul(4))), ((4i32) as u32))).wrapping_mul(((4i32) as u32))); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = { let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }; let a_ = (c.m.r32(sym::gDialog)).wrapping_add(52); c.m.w32(a_, v_); };
        { let v_ = { let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }; let a_ = (c.m.r32(sym::gDialog)).wrapping_add(96); c.m.w32(a_, v_); };
        { let v_ = (p_file_1).wrapping_add((rt::div_u32(c.m.r32((p_file_1).wrapping_add((((4i32) as u32)).wrapping_mul(4))), ((4i32) as u32))).wrapping_mul(((4i32) as u32))); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = { let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gDialog)).wrapping_add(52)).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((64i32) as u32)); let a_ = ((c.m.r32(sym::gDialog)).wrapping_add(52)).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = (p_file_1).wrapping_add((rt::div_u32(c.m.r32((p_file_1).wrapping_add((((2i32) as u32)).wrapping_mul(4))), ((4i32) as u32))).wrapping_mul(((4i32) as u32))); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = { let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gDialog)).wrapping_add(52)).wrapping_add(24); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((64i32) as u32)); let a_ = ((c.m.r32(sym::gDialog)).wrapping_add(52)).wrapping_add(32); c.m.w32(a_, v_); };
        { let v_ = (p_file_1).wrapping_add((rt::div_u32(c.m.r32((p_file_1).wrapping_add((((3i32) as u32)).wrapping_mul(4))), ((4i32) as u32))).wrapping_mul(((4i32) as u32))); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = { let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gDialog)).wrapping_add(52)).wrapping_add(28); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((64i32) as u32)); let a_ = ((c.m.r32(sym::gDialog)).wrapping_add(52)).wrapping_add(36); c.m.w32(a_, v_); };
        { let a0_ = (c.m.r32(sym::gDialog)).wrapping_add(8); let a1_ = (p_file_1).wrapping_add((rt::div_u32(c.m.r32((p_file_1).wrapping_add((((5i32) as u32)).wrapping_mul(4))), ((4i32) as u32))).wrapping_mul(((4i32) as u32))); let a2_ = (c.m.r32(sym::gDialog)).wrapping_add(52); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = (c.m.r32(sym::gDialog)).wrapping_add(8); F::Flash_Play(c, a0_, 1i32) };
        { let v_ = (p_file_1).wrapping_add((rt::div_u32(c.m.r32((p_file_1).wrapping_add((((6i32) as u32)).wrapping_mul(4))), ((4i32) as u32))).wrapping_mul(((4i32) as u32))); let a_ = c.m.r32(sym::gDialog); c.m.w32(a_, v_); };
        { let v_ = p_msgTbl_2; let a_ = (c.m.r32(sym::gDialog)).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gDialog)).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
        { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gDialog)).wrapping_add(124); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gDialog)).wrapping_add(136); c.m.w32(a_, (v_) as u32); };
        { let v_ = p_size_3; let a_ = (c.m.r32(sym::gDialog)).wrapping_add(116); c.m.w32(a_, (v_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Dialog_Init", &PROBE_N, &[], true); }
    r_
}

pub fn Dialog_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dialog_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32(sym::gDialog)).wrapping_add(8); F::Flash_Destroy(c, a0_) };
    if (((((c.m.r32(sym::gDialog)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gDialog); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gDialog; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Dialog_Term", &PROBE_N, &[], true); }
    r_
}

pub fn Dialog_Draw(c: &mut Ctx, mut p_visible_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dialog_Draw", &PROBE_N, &[(p_visible_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let mut l_flash_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
        { let a0_ = (c.m.r32(sym::gDialog)).wrapping_add(8); F::Flash_Advance(c, a0_) };
        if (((((((({ let a_ = (c.m.r32(sym::gDialog)).wrapping_add(100); (c.m.r32(a_) as i32) }) & (8i32))) != 0) && (((({ let a_ = (((c.m.r32(sym::gDialog)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); (c.m.r32(a_) as i32) }) & (8i32))) != 0)) as i32)) != 0) {
            { let r_ = 2i32; let a_ = (c.m.r32(sym::gDialog)).wrapping_add(100); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if ((((((((!(((({ let a_ = (c.m.r32(sym::gDialog)).wrapping_add(100); (c.m.r32(a_) as i32) }) & (16i32))) != 0)) as i32)) != 0) && (((({ let a_ = (((c.m.r32(sym::gDialog)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); (c.m.r32(a_) as i32) }) & (2i32))) != 0)) as i32)) != 0) {
            F::Dialog_PlayCursorPlate(c, 0i32, sym::dialog__str5);
            { let r_ = 16i32; let a_ = (c.m.r32(sym::gDialog)).wrapping_add(100); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        l_flash_2 = (c.m.r32(sym::gDialog)).wrapping_add(8);
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let _ = F::sprintf(c, fp.wrapping_add(40), sym::dialog__str6, &[(((l_i_3).wrapping_add(1i32)) as i64 as u64)]); };
                    if ((((({ let a_ = (c.m.r32(sym::gDialog)).wrapping_add(132); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(40), fp);
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                        { let v_ = (l_i_3).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((l_i_3).wrapping_mul(32i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(40), sym::dialog__str7, fp);
                        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                        F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(40), sym::dialog__str8, fp);
                        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                    } else {
                        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(40), fp);
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    }
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        's3: {
            let sv4: i32 = { let a_ = (c.m.r32(sym::gDialog)).wrapping_add(120); (c.m.r32(a_) as i32) };
            match sv4 {
                0i32 => {
                    F::Dialog_DrawText(c);
                    break 's3;
                }
                1i32 => {
                    F::Dialog_DrawTitle(c);
                    F::Dialog_DrawBody(c);
                    break 's3;
                }
                _ => {
                }
            }
        }
        { let v_ = (if (((((p_visible_1) != (0i32)) as i32)) != 0) { { let a_ = (c.m.r32(sym::gDialog)).wrapping_add(96); c.m.r32(a_) } } else { ((0i32) as u32) }); let a_ = (c.m.r32(sym::gDialog)).wrapping_add(52); c.m.w32(a_, v_); };
        { let a0_ = (c.m.r32(sym::gDialog)).wrapping_add(8); F::Flash_Draw(c, a0_) };
        F::Font_FlushAll(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Dialog_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn Dialog_Start(c: &mut Ctx, mut p_cmd_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dialog_Start", &PROBE_N, &[(p_cmd_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = p_cmd_1;
        match sv2 {
            0i32 => {
                if (((({ let a_ = (c.m.r32(sym::gDialog)).wrapping_add(100); (c.m.r32(a_) as i32) }) & (4i32))) != 0) {
                    break 'f;
                }
                { let v_ = 4i32; let a_ = (c.m.r32(sym::gDialog)).wrapping_add(100); c.m.w32(a_, (v_) as u32); };
                { let v_ = { let a_ = (c.m.r32(sym::gDialog)).wrapping_add(112); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gDialog)).wrapping_add(104); c.m.w32(a_, (v_) as u32); };
                's2: {
                    let sv3: i32 = { let a_ = (c.m.r32(sym::gDialog)).wrapping_add(116); (c.m.r32(a_) as i32) };
                    match sv3 {
                        0i32 => {
                            { let a0_ = (c.m.r32(sym::gDialog)).wrapping_add(8); F::Flash_GotoLabel(c, a0_, sym::dialog__str9, ((((1i32) as i64 as u64)) as u8)) };
                            break 's2;
                        }
                        1i32 => {
                            { let a0_ = (c.m.r32(sym::gDialog)).wrapping_add(8); F::Flash_GotoLabel(c, a0_, sym::dialog__str10, ((((1i32) as i64 as u64)) as u8)) };
                            break 's2;
                        }
                        _ => {
                        }
                    }
                }
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 4i32); };
                break 'f;
            }
            1i32 => {
                if (((({ let a_ = (c.m.r32(sym::gDialog)).wrapping_add(100); (c.m.r32(a_) as i32) }) & (8i32))) != 0) {
                    break 'f;
                }
                { let r_ = 8i32; let a_ = (c.m.r32(sym::gDialog)).wrapping_add(100); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = (!(4i32)); let a_ = (c.m.r32(sym::gDialog)).wrapping_add(100); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                's3: {
                    let sv4: i32 = { let a_ = (c.m.r32(sym::gDialog)).wrapping_add(116); (c.m.r32(a_) as i32) };
                    match sv4 {
                        0i32 => {
                            { let a0_ = (c.m.r32(sym::gDialog)).wrapping_add(8); F::Flash_GotoLabel(c, a0_, sym::dialog__str11, ((((1i32) as i64 as u64)) as u8)) };
                            break 's3;
                        }
                        1i32 => {
                            { let a0_ = (c.m.r32(sym::gDialog)).wrapping_add(8); F::Flash_GotoLabel(c, a0_, sym::dialog__str12, ((((1i32) as i64 as u64)) as u8)) };
                            break 's3;
                        }
                        _ => {
                        }
                    }
                }
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 5i32); };
                break 'f;
            }
            2i32 => {
                { let v_ = 4i32; let a_ = (c.m.r32(sym::gDialog)).wrapping_add(100); c.m.w32(a_, (v_) as u32); };
                { let v_ = { let a_ = (c.m.r32(sym::gDialog)).wrapping_add(112); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gDialog)).wrapping_add(104); c.m.w32(a_, (v_) as u32); };
                's4: {
                    let sv5: i32 = { let a_ = (c.m.r32(sym::gDialog)).wrapping_add(116); (c.m.r32(a_) as i32) };
                    match sv5 {
                        0i32 => {
                            { let a0_ = (c.m.r32(sym::gDialog)).wrapping_add(8); F::Flash_GotoLabel(c, a0_, sym::dialog__str13, ((((1i32) as i64 as u64)) as u8)) };
                            break 'f;
                        }
                        1i32 => {
                            { let a0_ = (c.m.r32(sym::gDialog)).wrapping_add(8); F::Flash_GotoLabel(c, a0_, sym::dialog__str14, ((((1i32) as i64 as u64)) as u8)) };
                            break 'f;
                        }
                        _ => {
                        }
                    }
                }
                break 'f;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Dialog_Start", &PROBE_N, &[], true); }
    r_
}

pub fn Dialog_Input(c: &mut Ctx, mut p_allowCancel_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dialog_Input", &PROBE_N, &[(p_allowCancel_1) as u32], false); }
    let mut l_result_2: i32 = 0;
    let r_ = 'f: {
    l_result_2 = 0i32;
    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
        break 'f 0i32;
    }
    if ((((!(((({ let a_ = (((c.m.r32(sym::gDialog)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); (c.m.r32(a_) as i32) }) & (2i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32(sym::gDialog)).wrapping_add(132); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((({ let a_ = ((sym::gPad).wrapping_add(((({ let a_ = (c.m.r32(sym::gDialog)).wrapping_add(136); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(448))).wrapping_add(400); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
        F::Dialog_PlayCursorPlate(c, 0i32, sym::dialog__str15);
        { let a_ = (c.m.r32(sym::gDialog)).wrapping_add(104); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gDialog)).wrapping_add(104); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
            { let v_ = 1i32; let a_ = (c.m.r32(sym::gDialog)).wrapping_add(104); c.m.w32(a_, (v_) as u32); };
        }
        F::Dialog_PlayCursorPlate(c, 0i32, sym::dialog__str16);
        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
    } else {
        if (((({ let a_ = ((sym::gPad).wrapping_add(((({ let a_ = (c.m.r32(sym::gDialog)).wrapping_add(136); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(448))).wrapping_add(400); c.m.r32(a_) }) & (((2i32) as u32)))) != 0) {
            F::Dialog_PlayCursorPlate(c, 0i32, sym::dialog__str17);
            { let a_ = (c.m.r32(sym::gDialog)).wrapping_add(104); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            if ((((({ let a_ = (c.m.r32(sym::gDialog)).wrapping_add(104); (c.m.r32(a_) as i32) }) >= (2i32)) as i32)) != 0) {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gDialog)).wrapping_add(104); c.m.w32(a_, (v_) as u32); };
            }
            F::Dialog_PlayCursorPlate(c, 0i32, sym::dialog__str18);
            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
        } else {
            if (((({ let a_ = ((sym::gPad).wrapping_add(((({ let a_ = (c.m.r32(sym::gDialog)).wrapping_add(136); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(448))).wrapping_add(396); c.m.r32(a_) }) & (((512i32) as u32)))) != 0) {
                's1: {
                    let sv3: i32 = { let a_ = (c.m.r32(sym::gDialog)).wrapping_add(104); (c.m.r32(a_) as i32) };
                    match sv3 {
                        0i32 => {
                            l_result_2 = 1i32;
                            { let r_ = 1i32; let a_ = (c.m.r32(sym::gDialog)).wrapping_add(100); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            break 's1;
                        }
                        1i32 => {
                            l_result_2 = (2i32).wrapping_neg();
                            { let r_ = 1i32; let a_ = (c.m.r32(sym::gDialog)).wrapping_add(100); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            break 's1;
                        }
                        _ => {
                        }
                    }
                }
                F::Dialog_PlayCursorPlate(c, 0i32, sym::dialog__str19);
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
            } else {
                if (((({ let a_ = ((sym::gPad).wrapping_add(((({ let a_ = (c.m.r32(sym::gDialog)).wrapping_add(136); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(448))).wrapping_add(396); c.m.r32(a_) }) & (((1024i32) as u32)))) != 0) {
                    if (((((p_allowCancel_1) != (0i32)) as i32)) != 0) {
                        F::Dialog_PlayCursorPlate(c, 0i32, sym::dialog__str20);
                        l_result_2 = (1i32).wrapping_neg();
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                    }
                }
            }
        }
    }
    break 'f l_result_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Dialog_Input", &PROBE_N, &[], true); }
    r_
}

pub fn Dialog_SetChoices(c: &mut Ctx, mut p_on_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dialog_SetChoices", &PROBE_N, &[(p_on_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_on_1; let a_ = (c.m.r32(sym::gDialog)).wrapping_add(132); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Dialog_SetChoices", &PROBE_N, &[], true); }
    r_
}

pub fn Dialog_SetPort(c: &mut Ctx, mut p_port_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dialog_SetPort", &PROBE_N, &[(p_port_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_port_1; let a_ = (c.m.r32(sym::gDialog)).wrapping_add(136); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Dialog_SetPort", &PROBE_N, &[], true); }
    r_
}

pub fn Dialog_SetLayout(c: &mut Ctx, mut p_layout_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dialog_SetLayout", &PROBE_N, &[(p_layout_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_layout_1; let a_ = (c.m.r32(sym::gDialog)).wrapping_add(120); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Dialog_SetLayout", &PROBE_N, &[], true); }
    r_
}

pub fn Dialog_SetMsgTable(c: &mut Ctx, mut p_tbl_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dialog_SetMsgTable", &PROBE_N, &[(p_tbl_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_tbl_1; let a_ = (c.m.r32(sym::gDialog)).wrapping_add(4); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Dialog_SetMsgTable", &PROBE_N, &[], true); }
    r_
}

pub fn Dialog_SetMsg(c: &mut Ctx, mut p_idx_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dialog_SetMsg", &PROBE_N, &[(p_idx_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_idx_1; let a_ = (c.m.r32(sym::gDialog)).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Dialog_SetMsg", &PROBE_N, &[], true); }
    r_
}

pub fn Dialog_SetTitle(c: &mut Ctx, mut p_idx_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dialog_SetTitle", &PROBE_N, &[(p_idx_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_idx_1; let a_ = (c.m.r32(sym::gDialog)).wrapping_add(124); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Dialog_SetTitle", &PROBE_N, &[], true); }
    r_
}

pub fn Dialog_SetCursor(c: &mut Ctx, mut p_choice_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dialog_SetCursor", &PROBE_N, &[(p_choice_1) as u32], false); }
    let mut l_old_2: i32 = 0;
    let r_ = 'f: {
    l_old_2 = { let a_ = (c.m.r32(sym::gDialog)).wrapping_add(104); (c.m.r32(a_) as i32) };
    if (((((l_old_2) == (((p_choice_1) ^ (1i32)))) as i32)) != 0) {
        F::Dialog_PlayCursorPlate(c, 0i32, sym::dialog__str21);
    } else {
        if (((((l_old_2) != ({ let a_ = (c.m.r32(sym::gDialog)).wrapping_add(112); (c.m.r32(a_) as i32) })) as i32)) != 0) {
            { let r_ = 2i32; let n_ = rt::div_i32(l_old_2, r_); l_old_2 = n_; };
        }
    }
    { let v_ = { let v_ = p_choice_1; let a_ = (c.m.r32(sym::gDialog)).wrapping_add(112); c.m.w32(a_, (v_) as u32); v_ }; let a_ = (c.m.r32(sym::gDialog)).wrapping_add(104); c.m.w32(a_, (v_) as u32); };
    F::Dialog_PlayCursorPlate(c, 0i32, sym::dialog__str22);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Dialog_SetCursor", &PROBE_N, &[], true); }
    r_
}

pub fn Dialog_IsClosed(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dialog_IsClosed", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f ((({ let a_ = (c.m.r32(sym::gDialog)).wrapping_add(100); (c.m.r32(a_) as i32) }).wrapping_shr((1i32) as u32)) & (1i32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Dialog_IsClosed", &PROBE_N, &[], true); }
    r_
}

pub fn Dialog_IsOpen(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dialog_IsOpen", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f ((({ let a_ = (((c.m.r32(sym::gDialog)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); (c.m.r32(a_) as i32) }).wrapping_shr((1i32) as u32)) & (1i32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Dialog_IsOpen", &PROBE_N, &[], true); }
    r_
}

pub fn Dialog_GetTitleTable(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dialog_GetTitleTable", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = c.m.r32(sym::gDialog); c.m.r32(a_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Dialog_GetTitleTable", &PROBE_N, &[], true); }
    r_
}
