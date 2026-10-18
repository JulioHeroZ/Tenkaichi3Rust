#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn TextBox_SetMaxWidth(c: &mut Ctx, mut p_box_1: u32, mut p_w_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TextBox_SetMaxWidth", &PROBE_N, &[(p_box_1) as u32, (p_w_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_w_2; let a_ = (p_box_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
    { let r_ = 8i32; let a_ = p_box_1; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "TextBox_SetMaxWidth", &PROBE_N, &[], true); }
    r_
}

pub fn TextBox_SetMaxHeight(c: &mut Ctx, mut p_box_1: u32, mut p_h_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TextBox_SetMaxHeight", &PROBE_N, &[(p_box_1) as u32, (p_h_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_h_2; let a_ = (p_box_1).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
    { let r_ = 16i32; let a_ = p_box_1; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "TextBox_SetMaxHeight", &PROBE_N, &[], true); }
    r_
}

pub fn TextBox_SetMaxSize(c: &mut Ctx, mut p_box_1: u32, mut p_w_2: i32, mut p_h_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TextBox_SetMaxSize", &PROBE_N, &[(p_box_1) as u32, (p_w_2) as u32, (p_h_3) as u32], false); }
    let r_ = 'f: {
    F::TextBox_SetMaxWidth(c, p_box_1, p_w_2);
    F::TextBox_SetMaxHeight(c, p_box_1, p_h_3);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "TextBox_SetMaxSize", &PROBE_N, &[], true); }
    r_
}

pub fn TextBox_SetLineOffsets(c: &mut Ctx, mut p_box_1: u32, mut p_y1_2: i32, mut p_y2_3: i32, mut p_y3_4: i32, mut p_y4_5: i32, mut p_y5_6: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TextBox_SetLineOffsets", &PROBE_N, &[(p_box_1) as u32, (p_y1_2) as u32, (p_y2_3) as u32, (p_y3_4) as u32, (p_y4_5) as u32, (p_y5_6) as u32], false); }
    let r_ = 'f: {
    { let r_ = 32i32; let a_ = p_box_1; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    { let v_ = p_y1_2; let a_ = ((p_box_1).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_y2_3; let a_ = ((p_box_1).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_y3_4; let a_ = ((p_box_1).wrapping_add(20)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_y4_5; let a_ = ((p_box_1).wrapping_add(20)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_y5_6; let a_ = ((p_box_1).wrapping_add(20)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "TextBox_SetLineOffsets", &PROBE_N, &[], true); }
    r_
}

pub fn TextBox_SetSpacing(c: &mut Ctx, mut p_box_1: u32, mut p_x_2: i32, mut p_y_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TextBox_SetSpacing", &PROBE_N, &[(p_box_1) as u32, (p_x_2) as u32, (p_y_3) as u32], false); }
    let r_ = 'f: {
    { let r_ = 64i32; let a_ = p_box_1; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    { let v_ = p_x_2; let a_ = (p_box_1).wrapping_add(72); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_y_3; let a_ = (p_box_1).wrapping_add(76); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "TextBox_SetSpacing", &PROBE_N, &[], true); }
    r_
}

pub fn view_b__TextBox_ClampByte(c: &mut Ctx, mut p_c_1: i32) -> u8 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "view_b__TextBox_ClampByte", &PROBE_N, &[(p_c_1) as u32], false); }
    let r_ = 'f: {
    break 'f (((if (((((p_c_1) < (0i32)) as i32)) != 0) { 0i32 } else { (if (((((p_c_1) > (255i32)) as i32)) != 0) { 255i32 } else { p_c_1 }) })) as u8);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "view_b__TextBox_ClampByte", &PROBE_N, &[], true); }
    r_
}

pub fn TextBox_DrawClip(c: &mut Ctx, mut p_draw_1: u32, mut p_prop_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TextBox_DrawClip", &PROBE_N, &[(p_draw_1) as u32, (p_prop_2) as u32], false); }
    let mut l_shadow_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_prop_2) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = rt::f2i(rt::fadd(rt::i2f((c.m.r32((p_draw_1).wrapping_add(8)) as i32)), F32(c.m.r32((p_prop_2).wrapping_add(56))))); let a_ = (p_draw_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = rt::f2i(rt::fadd(rt::i2f((c.m.r32((p_draw_1).wrapping_add(12)) as i32)), F32(c.m.r32((p_prop_2).wrapping_add(68))))); let a_ = (p_draw_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = (rt::f2i(rt::fmul(rt::i2f((c.m.r8(((p_draw_1).wrapping_add(24)).wrapping_add(((0i32) as u32)))) as i32), F32(c.m.r32(((p_prop_2).wrapping_add(84)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))))).wrapping_add((((c.m.r16(((p_prop_2).wrapping_add(100)).wrapping_add((((0i32) as u32)).wrapping_mul(2))) as i16)) as i32)); F::view_b__TextBox_ClampByte(c, a0_) }; let a_ = ((p_draw_1).wrapping_add(24)).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = { let a0_ = (rt::f2i(rt::fmul(rt::i2f((c.m.r8(((p_draw_1).wrapping_add(24)).wrapping_add(((1i32) as u32)))) as i32), F32(c.m.r32(((p_prop_2).wrapping_add(84)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))))).wrapping_add((((c.m.r16(((p_prop_2).wrapping_add(100)).wrapping_add((((1i32) as u32)).wrapping_mul(2))) as i16)) as i32)); F::view_b__TextBox_ClampByte(c, a0_) }; let a_ = ((p_draw_1).wrapping_add(24)).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = { let a0_ = (rt::f2i(rt::fmul(rt::i2f((c.m.r8(((p_draw_1).wrapping_add(24)).wrapping_add(((2i32) as u32)))) as i32), F32(c.m.r32(((p_prop_2).wrapping_add(84)).wrapping_add((((2i32) as u32)).wrapping_mul(4))))))).wrapping_add((((c.m.r16(((p_prop_2).wrapping_add(100)).wrapping_add((((2i32) as u32)).wrapping_mul(2))) as i16)) as i32)); F::view_b__TextBox_ClampByte(c, a0_) }; let a_ = ((p_draw_1).wrapping_add(24)).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = { let a0_ = (rt::f2i(rt::fmul(rt::i2f((c.m.r8(((p_draw_1).wrapping_add(24)).wrapping_add(((3i32) as u32)))) as i32), F32(c.m.r32(((p_prop_2).wrapping_add(84)).wrapping_add((((3i32) as u32)).wrapping_mul(4))))))).wrapping_add((((c.m.r16(((p_prop_2).wrapping_add(100)).wrapping_add((((3i32) as u32)).wrapping_mul(2))) as i16)) as i32)); F::view_b__TextBox_ClampByte(c, a0_) }; let a_ = ((p_draw_1).wrapping_add(24)).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = { let a0_ = (rt::f2i(rt::fmul(rt::i2f((c.m.r8(((p_draw_1).wrapping_add(28)).wrapping_add(((0i32) as u32)))) as i32), F32(c.m.r32(((p_prop_2).wrapping_add(84)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))))).wrapping_add((((c.m.r16(((p_prop_2).wrapping_add(100)).wrapping_add((((0i32) as u32)).wrapping_mul(2))) as i16)) as i32)); F::view_b__TextBox_ClampByte(c, a0_) }; let a_ = ((p_draw_1).wrapping_add(28)).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = { let a0_ = (rt::f2i(rt::fmul(rt::i2f((c.m.r8(((p_draw_1).wrapping_add(28)).wrapping_add(((1i32) as u32)))) as i32), F32(c.m.r32(((p_prop_2).wrapping_add(84)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))))).wrapping_add((((c.m.r16(((p_prop_2).wrapping_add(100)).wrapping_add((((1i32) as u32)).wrapping_mul(2))) as i16)) as i32)); F::view_b__TextBox_ClampByte(c, a0_) }; let a_ = ((p_draw_1).wrapping_add(28)).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = { let a0_ = (rt::f2i(rt::fmul(rt::i2f((c.m.r8(((p_draw_1).wrapping_add(28)).wrapping_add(((2i32) as u32)))) as i32), F32(c.m.r32(((p_prop_2).wrapping_add(84)).wrapping_add((((2i32) as u32)).wrapping_mul(4))))))).wrapping_add((((c.m.r16(((p_prop_2).wrapping_add(100)).wrapping_add((((2i32) as u32)).wrapping_mul(2))) as i16)) as i32)); F::view_b__TextBox_ClampByte(c, a0_) }; let a_ = ((p_draw_1).wrapping_add(28)).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = { let a0_ = (rt::f2i(rt::fmul(rt::i2f((c.m.r8(((p_draw_1).wrapping_add(28)).wrapping_add(((3i32) as u32)))) as i32), F32(c.m.r32(((p_prop_2).wrapping_add(84)).wrapping_add((((3i32) as u32)).wrapping_mul(4))))))).wrapping_add((((c.m.r16(((p_prop_2).wrapping_add(100)).wrapping_add((((3i32) as u32)).wrapping_mul(2))) as i16)) as i32)); F::view_b__TextBox_ClampByte(c, a0_) }; let a_ = ((p_draw_1).wrapping_add(28)).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
    }
    if (((((((c.m.r8(((p_draw_1).wrapping_add(24)).wrapping_add(((3i32) as u32)))) as i32)) != (0i32)) as i32)) != 0) {
        l_shadow_3 = ((((((((c.m.r8(((p_draw_1).wrapping_add(28)).wrapping_add(((0i32) as u32)))) as i32)) | ((((c.m.r8(((p_draw_1).wrapping_add(28)).wrapping_add(((2i32) as u32)))) as i32)).wrapping_shl((16i32) as u32)))) | ((((((c.m.r8(((p_draw_1).wrapping_add(28)).wrapping_add(((3i32) as u32)))) as i32)).wrapping_shl((24i32) as u32)) | ((((c.m.r8(((p_draw_1).wrapping_add(28)).wrapping_add(((1i32) as u32)))) as i32)).wrapping_shl((8i32) as u32)))))) as u32);
        F::Font_PushStyle(c);
        { let a0_ = (c.m.r32(p_draw_1) as i32); F::Font_SetAlign(c, a0_) };
        { let a0_ = F32(c.m.r32((p_draw_1).wrapping_add(16))); let a1_ = F32(c.m.r32((p_draw_1).wrapping_add(20))); F::Font_SetScaleXY(c, a0_, a1_) };
        { let a0_ = ((c.m.r8(((p_draw_1).wrapping_add(24)).wrapping_add(((0i32) as u32)))) as i32); let a1_ = ((c.m.r8(((p_draw_1).wrapping_add(24)).wrapping_add(((1i32) as u32)))) as i32); let a2_ = ((c.m.r8(((p_draw_1).wrapping_add(24)).wrapping_add(((2i32) as u32)))) as i32); let a3_ = ((c.m.r8(((p_draw_1).wrapping_add(24)).wrapping_add(((3i32) as u32)))) as i32); F::Font_SetColorRGBA(c, ((((a0_) as i64 as u64)) as u8), ((((a1_) as i64 as u64)) as u8), ((((a2_) as i64 as u64)) as u8), a3_) };
        F::Font_SetShadowMode(c, 2i32);
        F::Font_SetShadowOffset(c, 1i32, 2i32);
        F::Font_SetShadowColor(c, l_shadow_3);
        { let a0_ = (c.m.r32(((p_draw_1).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a1_ = (c.m.r32(((p_draw_1).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); let a2_ = (c.m.r32(((p_draw_1).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32); let a3_ = (c.m.r32(((p_draw_1).wrapping_add(32)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32); F::Font_SetClip(c, a0_, a1_, a2_, a3_) };
        { let a0_ = (c.m.r32((p_draw_1).wrapping_add(52)) as i32); let a1_ = (c.m.r32((p_draw_1).wrapping_add(56)) as i32); F::Font_SetSpacing(c, a0_, a1_) };
        { let a0_ = (c.m.r32((p_draw_1).wrapping_add(8)) as i32); let a1_ = (c.m.r32((p_draw_1).wrapping_add(12)) as i32); let a2_ = c.m.r32((p_draw_1).wrapping_add(4)); F::Font_PrintAt(c, a0_, a1_, a2_) };
        F::Font_PopStyle(c);
        if ((((((c.m.r32((p_draw_1).wrapping_add(48)) as i32)) != (1i32)) as i32)) != 0) {
            F::Font_FlushAll(c);
            F::Gfx_AddDefaultEnv(c);
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "TextBox_DrawClip", &PROBE_N, &[], true); }
    r_
}

pub fn TextBox_AttachLine(c: &mut Ctx, mut p_flash_1: u32, mut p_ref_2: u32, mut p_x_3: i32, mut p_y_4: i32, mut p_line_5: i32, mut p_box_6: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TextBox_AttachLine", &PROBE_N, &[(p_flash_1) as u32, (p_ref_2) as u32, (p_x_3) as u32, (p_y_4) as u32, (p_line_5) as u32, (p_box_6) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_size_7: i32 = 0;
    let mut l_alpha_8: F32 = F32(0);
    let mut l_str_9: u32 = 0;
    let r_ = 'f: {
        if (((((((((p_line_5) < (0i32)) as i32)) != 0) || ((((((c.m.r32(p_ref_2) as i32)) < (0i32)) as i32)) != 0)) as i32)) != 0) {
            F::Flash_ClipSetCallbackC(c, p_flash_1, p_ref_2, ((0i32) as u32), ((0i32) as u32));
            break 'f;
        }
        { let v_ = 0i32; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        l_alpha_8 = F::Flash_ClipGetAlpha(c, p_flash_1, p_ref_2);
        l_str_9 = (c.m.r32((p_box_6).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_box_6).wrapping_add(4))).wrapping_add(((((p_line_5).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
        if (((((c.m.r32(p_box_6) as i32)) & (8i32))) != 0) {
            l_size_7 = F::Font_GetWidth(c, l_str_9);
            if ((((((c.m.r32((p_box_6).wrapping_add(40)) as i32)) < (l_size_7)) as i32)) != 0) {
                { let v_ = rt::fdiv(rt::i2f((c.m.r32((p_box_6).wrapping_add(40)) as i32)), rt::i2f(l_size_7)); let a_ = (fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            } else {
                { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            }
        } else {
            { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        }
        if (((((c.m.r32(p_box_6) as i32)) & (16i32))) != 0) {
            l_size_7 = F::Font_GetHeight(c, l_str_9);
            if ((((((c.m.r32((p_box_6).wrapping_add(44)) as i32)) < (l_size_7)) as i32)) != 0) {
                { let v_ = rt::fdiv(rt::i2f((c.m.r32((p_box_6).wrapping_add(44)) as i32)), rt::i2f(l_size_7)); let a_ = (fp.wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            } else {
                { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            }
        } else {
            { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        }
        if (((((c.m.r32(p_box_6) as i32)) & (32i32))) != 0) {
            's1: {
                let sv10: i32 = F::Font_CountLines(c, l_str_9);
                match sv10 {
                    1i32 => {
                        { let r_ = (c.m.r32(((p_box_6).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                        break 's1;
                    }
                    2i32 => {
                        { let r_ = (c.m.r32(((p_box_6).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                        break 's1;
                    }
                    3i32 => {
                        { let r_ = (c.m.r32(((p_box_6).wrapping_add(20)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                        break 's1;
                    }
                    4i32 => {
                        { let r_ = (c.m.r32(((p_box_6).wrapping_add(20)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                        break 's1;
                    }
                    5i32 => {
                        { let r_ = (c.m.r32(((p_box_6).wrapping_add(20)).wrapping_add((((4i32) as u32)).wrapping_mul(4))) as i32); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
        }
        { let v_ = (((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)).wrapping_add((c.m.r32((p_box_6).wrapping_add(12)) as i32))).wrapping_add(p_x_3); let a_ = ((p_box_6).wrapping_add(80)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = (((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)).wrapping_add((c.m.r32((p_box_6).wrapping_add(16)) as i32))).wrapping_add(p_y_4); let a_ = ((p_box_6).wrapping_add(80)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_box_6).wrapping_add(80)).wrapping_add(16); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((p_box_6).wrapping_add(80)).wrapping_add(20); c.m.w32(a_, (v_).0); };
        if (((((c.m.r32(p_box_6) as i32)) & (2i32))) != 0) {
            { let v_ = c.m.r8(((p_box_6).wrapping_add(48)).wrapping_add(((0i32) as u32))); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(24)).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = c.m.r8(((p_box_6).wrapping_add(48)).wrapping_add(((1i32) as u32))); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(24)).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = c.m.r8(((p_box_6).wrapping_add(48)).wrapping_add(((2i32) as u32))); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(24)).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = ((rt::f2u(rt::fmul(rt::i2f(((c.m.r8(((p_box_6).wrapping_add(48)).wrapping_add(((3i32) as u32)))) as i32)), l_alpha_8))) as u8); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(24)).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
        } else {
            { let v_ = ((255i32) as u8); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(24)).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = ((255i32) as u8); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(24)).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = ((255i32) as u8); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(24)).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = ((rt::f2u(rt::fmul(l_alpha_8, F32(0x43000000)))) as u8); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(24)).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
        }
        if (((((c.m.r32(p_box_6) as i32)) & (4i32))) != 0) {
            { let v_ = c.m.r8(((p_box_6).wrapping_add(52)).wrapping_add(((0i32) as u32))); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(28)).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = c.m.r8(((p_box_6).wrapping_add(52)).wrapping_add(((1i32) as u32))); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(28)).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = c.m.r8(((p_box_6).wrapping_add(52)).wrapping_add(((2i32) as u32))); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(28)).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = ((rt::f2u(rt::fmul(rt::i2f(((c.m.r8(((p_box_6).wrapping_add(52)).wrapping_add(((3i32) as u32)))) as i32)), l_alpha_8))) as u8); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(28)).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
        } else {
            { let v_ = ((32i32) as u8); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(28)).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = ((32i32) as u8); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(28)).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = ((32i32) as u8); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(28)).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = ((rt::f2u(rt::fmul(l_alpha_8, F32(0x42800000)))) as u8); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(28)).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
        }
        if (((((c.m.r32(p_box_6) as i32)) & (1i32))) != 0) {
            { let v_ = (c.m.r32(((p_box_6).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (c.m.r32(((p_box_6).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (c.m.r32(((p_box_6).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (c.m.r32(((p_box_6).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(32)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 511i32; let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = 447i32; let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(32)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
        if (((((c.m.r32(p_box_6) as i32)) & (64i32))) != 0) {
            { let v_ = (c.m.r32((p_box_6).wrapping_add(72)) as i32); let a_ = ((p_box_6).wrapping_add(80)).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
            { let v_ = (c.m.r32((p_box_6).wrapping_add(76)) as i32); let a_ = ((p_box_6).wrapping_add(80)).wrapping_add(56); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 0i32; let a_ = ((p_box_6).wrapping_add(80)).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = ((p_box_6).wrapping_add(80)).wrapping_add(56); c.m.w32(a_, (v_) as u32); };
        }
        { let v_ = l_str_9; let a_ = ((p_box_6).wrapping_add(80)).wrapping_add(4); c.m.w32(a_, v_); };
        F::Flash_ClipSetCallbackC(c, p_flash_1, p_ref_2, fid::TextBox_DrawClip, (p_box_6).wrapping_add(80));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "TextBox_AttachLine", &PROBE_N, &[], true); }
    r_
}

pub fn TextBox_AttachString(c: &mut Ctx, mut p_flash_1: u32, mut p_ref_2: u32, mut p_x_3: i32, mut p_y_4: i32, mut p_str_5: u32, mut p_box_6: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TextBox_AttachString", &PROBE_N, &[(p_flash_1) as u32, (p_ref_2) as u32, (p_x_3) as u32, (p_y_4) as u32, (p_str_5) as u32, (p_box_6) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_size_7: i32 = 0;
    let mut l_alpha_8: F32 = F32(0);
    let r_ = 'f: {
        if ((((((c.m.r32(p_ref_2) as i32)) < (0i32)) as i32)) != 0) {
            F::Flash_ClipSetCallbackC(c, p_flash_1, p_ref_2, ((0i32) as u32), ((0i32) as u32));
            break 'f;
        }
        { let v_ = 0i32; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        l_alpha_8 = F::Flash_ClipGetAlpha(c, p_flash_1, p_ref_2);
        if (((((c.m.r32(p_box_6) as i32)) & (8i32))) != 0) {
            l_size_7 = F::Font_GetWidth(c, p_str_5);
            if ((((((c.m.r32((p_box_6).wrapping_add(40)) as i32)) < (l_size_7)) as i32)) != 0) {
                { let v_ = rt::fdiv(rt::i2f((c.m.r32((p_box_6).wrapping_add(40)) as i32)), rt::i2f(l_size_7)); let a_ = (fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            } else {
                { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            }
        } else {
            { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        }
        if (((((c.m.r32(p_box_6) as i32)) & (16i32))) != 0) {
            l_size_7 = F::Font_GetHeight(c, p_str_5);
            if ((((((c.m.r32((p_box_6).wrapping_add(44)) as i32)) < (l_size_7)) as i32)) != 0) {
                { let v_ = rt::fdiv(rt::i2f((c.m.r32((p_box_6).wrapping_add(44)) as i32)), rt::i2f(l_size_7)); let a_ = (fp.wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            } else {
                { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            }
        } else {
            { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        }
        if (((((c.m.r32(p_box_6) as i32)) & (32i32))) != 0) {
            's1: {
                let sv9: i32 = F::Font_CountLines(c, p_str_5);
                match sv9 {
                    1i32 => {
                        { let r_ = (c.m.r32(((p_box_6).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                        break 's1;
                    }
                    2i32 => {
                        { let r_ = (c.m.r32(((p_box_6).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                        break 's1;
                    }
                    3i32 => {
                        { let r_ = (c.m.r32(((p_box_6).wrapping_add(20)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                        break 's1;
                    }
                    4i32 => {
                        { let r_ = (c.m.r32(((p_box_6).wrapping_add(20)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                        break 's1;
                    }
                    5i32 => {
                        { let r_ = (c.m.r32(((p_box_6).wrapping_add(20)).wrapping_add((((4i32) as u32)).wrapping_mul(4))) as i32); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
        }
        { let v_ = (((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)).wrapping_add((c.m.r32((p_box_6).wrapping_add(12)) as i32))).wrapping_add(p_x_3); let a_ = ((p_box_6).wrapping_add(80)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = (((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)).wrapping_add((c.m.r32((p_box_6).wrapping_add(16)) as i32))).wrapping_add(p_y_4); let a_ = ((p_box_6).wrapping_add(80)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_box_6).wrapping_add(80)).wrapping_add(16); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((p_box_6).wrapping_add(80)).wrapping_add(20); c.m.w32(a_, (v_).0); };
        if (((((c.m.r32(p_box_6) as i32)) & (2i32))) != 0) {
            { let v_ = c.m.r8(((p_box_6).wrapping_add(48)).wrapping_add(((0i32) as u32))); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(24)).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = c.m.r8(((p_box_6).wrapping_add(48)).wrapping_add(((1i32) as u32))); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(24)).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = c.m.r8(((p_box_6).wrapping_add(48)).wrapping_add(((2i32) as u32))); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(24)).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = ((rt::f2u(rt::fmul(rt::i2f(((c.m.r8(((p_box_6).wrapping_add(48)).wrapping_add(((3i32) as u32)))) as i32)), l_alpha_8))) as u8); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(24)).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
        } else {
            { let v_ = ((255i32) as u8); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(24)).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = ((255i32) as u8); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(24)).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = ((255i32) as u8); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(24)).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = ((rt::f2u(rt::fmul(l_alpha_8, F32(0x43000000)))) as u8); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(24)).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
        }
        if (((((c.m.r32(p_box_6) as i32)) & (4i32))) != 0) {
            { let v_ = c.m.r8(((p_box_6).wrapping_add(52)).wrapping_add(((0i32) as u32))); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(28)).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = c.m.r8(((p_box_6).wrapping_add(52)).wrapping_add(((1i32) as u32))); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(28)).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = c.m.r8(((p_box_6).wrapping_add(52)).wrapping_add(((2i32) as u32))); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(28)).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = ((rt::f2u(rt::fmul(rt::i2f(((c.m.r8(((p_box_6).wrapping_add(52)).wrapping_add(((3i32) as u32)))) as i32)), l_alpha_8))) as u8); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(28)).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
        } else {
            { let v_ = ((32i32) as u8); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(28)).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = ((32i32) as u8); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(28)).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = ((32i32) as u8); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(28)).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = ((rt::f2u(rt::fmul(l_alpha_8, F32(0x42800000)))) as u8); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(28)).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
        }
        if (((((c.m.r32(p_box_6) as i32)) & (1i32))) != 0) {
            { let v_ = (c.m.r32(((p_box_6).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (c.m.r32(((p_box_6).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (c.m.r32(((p_box_6).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (c.m.r32(((p_box_6).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32); let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(32)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 511i32; let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = 447i32; let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(32)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (((p_box_6).wrapping_add(80)).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
        if (((((c.m.r32(p_box_6) as i32)) & (64i32))) != 0) {
            { let v_ = (c.m.r32((p_box_6).wrapping_add(72)) as i32); let a_ = ((p_box_6).wrapping_add(80)).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
            { let v_ = (c.m.r32((p_box_6).wrapping_add(76)) as i32); let a_ = ((p_box_6).wrapping_add(80)).wrapping_add(56); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 0i32; let a_ = ((p_box_6).wrapping_add(80)).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = ((p_box_6).wrapping_add(80)).wrapping_add(56); c.m.w32(a_, (v_) as u32); };
        }
        { let v_ = p_str_5; let a_ = ((p_box_6).wrapping_add(80)).wrapping_add(4); c.m.w32(a_, v_); };
        F::Flash_ClipSetCallbackC(c, p_flash_1, p_ref_2, fid::TextBox_DrawClip, (p_box_6).wrapping_add(80));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "TextBox_AttachString", &PROBE_N, &[], true); }
    r_
}
