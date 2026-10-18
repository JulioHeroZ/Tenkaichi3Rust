#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Progress_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Progress_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, ((2044u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gProgress; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gProgress); F::memset(c, a0_, 0i32, 2044u32) }; };
    { let v_ = 449i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = F::Heap_Alloc(c, 12288i32, ((64i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gProgress)).wrapping_add(8); c.m.w32(a_, v_); };
    { let v_ = F::Heap_Alloc(c, 26624i32, ((32i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gProgress)).wrapping_add(12); c.m.w32(a_, v_); };
    { let v_ = F::Heap_Alloc(c, 896i32, ((32i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gProgress)).wrapping_add(16); c.m.w32(a_, v_); };
    { let v_ = 1i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
    { let r_ = 64i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    F::Progress_ClearSession(c);
    { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gProgress)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Progress_Init", &PROBE_N, &[], true); }
    r_
}

pub fn Progress_ClearSession(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Progress_ClearSession", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gProgress)) != (((0i32) as u32))) as i32)) != 0) {
        { let _ = { let a0_ = (c.m.r32(sym::gProgress)).wrapping_add(48); F::memset(c, a0_, 0i32, 76u32) }; };
        { let _ = { let a0_ = (c.m.r32(sym::gProgress)).wrapping_add(124); F::memset(c, a0_, 0i32, 964u32) }; };
        { let _ = { let a0_ = (c.m.r32(sym::gProgress)).wrapping_add(1088); F::memset(c, a0_, 0i32, ((500i32) as u32)) }; };
        { let _ = { let a0_ = (c.m.r32(sym::gProgress)).wrapping_add(1588); F::memset(c, a0_, 0i32, 88u32) }; };
        { let _ = { let a0_ = (c.m.r32(sym::gProgress)).wrapping_add(1676); F::memset(c, a0_, 0i32, 324u32) }; };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(2000); c.m.w32(a_, (v_) as u32); };
        { let _ = { let a0_ = (c.m.r32(sym::gProgress)).wrapping_add(2004); F::memset(c, a0_, 0i32, 40u32) }; };
        {
            l_i_1 = 0i32;
            'l1: loop {
                if !(((((l_i_1) < (5i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = 0i32; let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1088)).wrapping_add((((l_i_1) as u32)).wrapping_mul(48)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 1i32; let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1328)).wrapping_add((((l_i_1) as u32)).wrapping_mul(48)); c.m.w32(a_, (v_) as u32); };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Progress_ClearSession", &PROBE_N, &[], true); }
    r_
}

pub fn FlashAnim_Blink(c: &mut Ctx, mut p_flash_1: u32, mut p_ref_2: u32, mut p_timer_3: u32, mut p_frame_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashAnim_Blink", &PROBE_N, &[(p_flash_1) as u32, (p_ref_2) as u32, (p_timer_3) as u32, (p_frame_4) as u32], false); }
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
        break 'f;
    }
    if ((((((c.m.r32(p_ref_2) as i32)) < (0i32)) as i32)) != 0) {
        break 'f;
    }
    if ((((((c.m.r32(p_timer_3) as i32)) < (36i32)) as i32)) != 0) {
        { let _ = F::Flash_ClipSetFlags(c, p_flash_1, p_ref_2, 2i32, ((((0i32) as i64 as u64)) as u8)); };
    } else {
        if ((((((c.m.r32(p_timer_3) as i32)) < (40i32)) as i32)) != 0) {
            { let _ = F::Flash_ClipSetFlags(c, p_flash_1, p_ref_2, 2i32, ((((1i32) as i64 as u64)) as u8)); };
            F::Flash_ClipSetTex(c, p_flash_1, p_ref_2, p_frame_4);
        } else {
            if ((((((c.m.r32(p_timer_3) as i32)) < (44i32)) as i32)) != 0) {
                { let _ = F::Flash_ClipSetFlags(c, p_flash_1, p_ref_2, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                F::Flash_ClipSetTex(c, p_flash_1, p_ref_2, (p_frame_4).wrapping_add(1i32));
            } else {
                if ((((((c.m.r32(p_timer_3) as i32)) < (48i32)) as i32)) != 0) {
                    { let _ = F::Flash_ClipSetFlags(c, p_flash_1, p_ref_2, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                    F::Flash_ClipSetTex(c, p_flash_1, p_ref_2, p_frame_4);
                } else {
                    { let _ = F::Flash_ClipSetFlags(c, p_flash_1, p_ref_2, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                }
            }
        }
    }
    { let a_ = p_timer_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    if ((((((c.m.r32(p_timer_3) as i32)) >= (136i32)) as i32)) != 0) {
        if (((((((F::Rand_Range(c, ((12i32) as u32))) as i32)) >= (8i32)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = p_timer_3; c.m.w32(a_, (v_) as u32); };
        }
    }
    if ((((((c.m.r32(p_timer_3) as i32)) > (300i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = p_timer_3; c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashAnim_Blink", &PROBE_N, &[], true); }
    r_
}

pub fn FlashAnim_ShowNext(c: &mut Ctx, mut p_flash_1: u32, mut p_ref_2: u32, mut p_frame_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashAnim_ShowNext", &PROBE_N, &[(p_flash_1) as u32, (p_ref_2) as u32, (p_frame_3) as u32], false); }
    let r_ = 'f: {
    if ((((((c.m.r32(p_ref_2) as i32)) >= (0i32)) as i32)) != 0) {
        { let _ = F::Flash_ClipSetFlags(c, p_flash_1, p_ref_2, 2i32, ((((1i32) as i64 as u64)) as u8)); };
        F::Flash_ClipSetTex(c, p_flash_1, p_ref_2, (p_frame_3).wrapping_add(1i32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashAnim_ShowNext", &PROBE_N, &[], true); }
    r_
}

pub fn FlashAnim_Talk(c: &mut Ctx, mut p_flash_1: u32, mut p_ref_2: u32, mut p_timer_3: u32, mut p_frame_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashAnim_Talk", &PROBE_N, &[(p_flash_1) as u32, (p_ref_2) as u32, (p_timer_3) as u32, (p_frame_4) as u32], false); }
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
        break 'f;
    }
    if ((((((c.m.r32(p_ref_2) as i32)) < (0i32)) as i32)) != 0) {
        break 'f;
    }
    F::LipSync_Update(c);
    if (((((F::LipSync_IsOpen(c)) != (0i32)) as i32)) != 0) {
        { let a_ = p_timer_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        if (((((rt::rem_i32((c.m.r32(p_timer_3) as i32), 6i32)) == (0i32)) as i32)) != 0) {
            's1: {
                let sv5: i32 = ((F::Rand_Range(c, ((3i32) as u32))) as i32);
                match sv5 {
                    0i32 => {
                        { let _ = F::Flash_ClipSetFlags(c, p_flash_1, p_ref_2, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                        F::Flash_ClipSetTex(c, p_flash_1, p_ref_2, p_frame_4);
                        break 's1;
                    }
                    1i32 => {
                        { let _ = F::Flash_ClipSetFlags(c, p_flash_1, p_ref_2, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                        F::Flash_ClipSetTex(c, p_flash_1, p_ref_2, (p_frame_4).wrapping_add(1i32));
                        break 's1;
                    }
                    2i32 => {
                        { let _ = F::Flash_ClipSetFlags(c, p_flash_1, p_ref_2, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                        F::Flash_ClipSetTex(c, p_flash_1, p_ref_2, p_frame_4);
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
        }
    } else {
        { let _ = F::Flash_ClipSetFlags(c, p_flash_1, p_ref_2, 2i32, ((((1i32) as i64 as u64)) as u8)); };
        F::Flash_ClipSetTex(c, p_flash_1, p_ref_2, (p_frame_4).wrapping_add(1i32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashAnim_Talk", &PROBE_N, &[], true); }
    r_
}

pub fn FlashAnim_ShowNext2(c: &mut Ctx, mut p_flash_1: u32, mut p_ref_2: u32, mut p_frame_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashAnim_ShowNext2", &PROBE_N, &[(p_flash_1) as u32, (p_ref_2) as u32, (p_frame_3) as u32], false); }
    let r_ = 'f: {
    if ((((((c.m.r32(p_ref_2) as i32)) >= (0i32)) as i32)) != 0) {
        { let _ = F::Flash_ClipSetFlags(c, p_flash_1, p_ref_2, 2i32, ((((1i32) as i64 as u64)) as u8)); };
        F::Flash_ClipSetTex(c, p_flash_1, p_ref_2, (p_frame_3).wrapping_add(1i32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashAnim_ShowNext2", &PROBE_N, &[], true); }
    r_
}

pub fn FlashAnim_Sheet(c: &mut Ctx, mut p_flash_1: u32, mut p_ref_2: u32, mut p_timer_3: u32, mut p_frame_4: u32, mut p_uv_5: u32, mut p_cols_6: i32, mut p_rows_7: i32, mut p_period_8: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashAnim_Sheet", &PROBE_N, &[(p_flash_1) as u32, (p_ref_2) as u32, (p_timer_3) as u32, (p_frame_4) as u32, (p_uv_5) as u32, (p_cols_6) as u32, (p_rows_7) as u32, (p_period_8) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_count_9: i32 = 0;
    let mut l_row_10: i32 = 0;
    let mut l_col_11: i32 = 0;
    let r_ = 'f: {
        l_count_9 = (p_cols_6).wrapping_mul(p_rows_7);
        if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
            break 'f;
        }
        if ((((((c.m.r32(p_ref_2) as i32)) < (0i32)) as i32)) != 0) {
            break 'f;
        }
        { let a_ = p_timer_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        if (((((rt::rem_i32((c.m.r32(p_timer_3) as i32), p_period_8)) == (0i32)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = p_timer_3; c.m.w32(a_, (v_) as u32); };
            { let a_ = p_frame_4; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            if ((((((c.m.r32(p_frame_4) as i32)) >= (l_count_9)) as i32)) != 0) {
                { let v_ = 0i32; let a_ = p_frame_4; c.m.w32(a_, (v_) as u32); };
            }
        }
        if ((((((c.m.r32(p_frame_4) as i32)) != (0i32)) as i32)) != 0) {
            l_col_11 = rt::rem_i32((c.m.r32(p_frame_4) as i32), p_cols_6);
            l_row_10 = rt::div_i32((c.m.r32(p_frame_4) as i32), p_cols_6);
        } else {
            l_row_10 = 0i32;
            l_col_11 = 0i32;
        }
        { let v_ = ((c.m.r32(p_uv_5) as i32)).wrapping_add(((c.m.r32((p_uv_5).wrapping_add(8)) as i32)).wrapping_mul(l_col_11)); let a_ = fp; c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32((p_uv_5).wrapping_add(4)) as i32)).wrapping_add(((c.m.r32((p_uv_5).wrapping_add(12)) as i32)).wrapping_mul(l_row_10)); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32((p_uv_5).wrapping_add(8)) as i32)).wrapping_add(((c.m.r32((p_uv_5).wrapping_add(8)) as i32)).wrapping_mul(l_col_11)); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32((p_uv_5).wrapping_add(12)) as i32)).wrapping_add(((c.m.r32((p_uv_5).wrapping_add(12)) as i32)).wrapping_mul(l_row_10)); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
        F::Flash_ClipSetUv(c, p_flash_1, p_ref_2, fp);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "FlashAnim_Sheet", &PROBE_N, &[], true); }
    r_
}

pub fn FlashAnim_Scroll(c: &mut Ctx, mut p_flash_1: u32, mut p_ref_2: u32, mut p_uv_3: u32, mut p_x_4: u32, mut p_y_5: u32, mut p_dx_6: F32, mut p_dy_7: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashAnim_Scroll", &PROBE_N, &[(p_flash_1) as u32, (p_ref_2) as u32, (p_uv_3) as u32, (p_x_4) as u32, (p_y_5) as u32, (p_dx_6).0, (p_dy_7).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let r_ = 'f: {
        if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
            break 'f;
        }
        if ((((((c.m.r32(p_ref_2) as i32)) < (0i32)) as i32)) != 0) {
            break 'f;
        }
        { let s_ = p_uv_3; let d_ = fp; rt::memcpy(c, d_, s_, 20); };
        if (((((p_x_4) != (((0i32) as u32))) as i32)) != 0) {
            { let r_ = rt::f2i(F32(c.m.r32(p_x_4))); let a_ = fp; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
            { let r_ = rt::f2i(F32(c.m.r32(p_x_4))); let a_ = (fp).wrapping_add(8); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
            { let r_ = p_dx_6; let a_ = p_x_4; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::flt(p_dx_6, F32(0x00000000)) as i32)) != 0) {
                if (((rt::flt(F32(c.m.r32(p_x_4)), F32(0x00000000)) as i32)) != 0) {
                    { let r_ = rt::i2f((c.m.r32((p_uv_3).wrapping_add(8)) as i32)); let a_ = p_x_4; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                }
            } else {
                if (((rt::fle(rt::i2f((c.m.r32((p_uv_3).wrapping_add(8)) as i32)), F32(c.m.r32(p_x_4))) as i32)) != 0) {
                    { let r_ = rt::i2f((c.m.r32((p_uv_3).wrapping_add(8)) as i32)); let a_ = p_x_4; let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                }
            }
        }
        if (((((p_y_5) != (((0i32) as u32))) as i32)) != 0) {
            { let r_ = rt::f2i(F32(c.m.r32(p_y_5))); let a_ = (fp).wrapping_add(4); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
            { let r_ = rt::f2i(F32(c.m.r32(p_y_5))); let a_ = (fp).wrapping_add(12); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
            { let r_ = p_dy_7; let a_ = p_y_5; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::flt(p_dy_7, F32(0x00000000)) as i32)) != 0) {
                if (((rt::flt(F32(c.m.r32(p_y_5)), F32(0x00000000)) as i32)) != 0) {
                    { let r_ = rt::i2f((c.m.r32((p_uv_3).wrapping_add(12)) as i32)); let a_ = p_y_5; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                }
            } else {
                if (((rt::fle(rt::i2f((c.m.r32((p_uv_3).wrapping_add(12)) as i32)), F32(c.m.r32(p_y_5))) as i32)) != 0) {
                    { let r_ = rt::i2f((c.m.r32((p_uv_3).wrapping_add(12)) as i32)); let a_ = p_y_5; let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                }
            }
        }
        { let _ = F::Flash_ClipSetFlags(c, p_flash_1, p_ref_2, 512i32, ((((1i32) as i64 as u64)) as u8)); };
        F::Flash_ClipSetUv(c, p_flash_1, p_ref_2, fp);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "FlashAnim_Scroll", &PROBE_N, &[], true); }
    r_
}

pub fn Num_ToDigits(c: &mut Ctx, mut p_digits_1: u32, mut p_value_2: i32, mut p_count_3: i32, mut p_zeroPad_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Num_ToDigits", &PROBE_N, &[(p_digits_1) as u32, (p_value_2) as u32, (p_count_3) as u32, (p_zeroPad_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_started_5: i32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_d_7: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 32);
        { let v_ = 1i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
        { let v_ = 10i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
        { let v_ = 100i32; c.m.w32(fp.wrapping_add(8), (v_) as u32); }
        { let v_ = 1000i32; c.m.w32(fp.wrapping_add(12), (v_) as u32); }
        { let v_ = 10000i32; c.m.w32(fp.wrapping_add(16), (v_) as u32); }
        { let v_ = 100000i32; c.m.w32(fp.wrapping_add(20), (v_) as u32); }
        { let v_ = 1000000i32; c.m.w32(fp.wrapping_add(24), (v_) as u32); }
        { let v_ = 10000000i32; c.m.w32(fp.wrapping_add(28), (v_) as u32); }
        l_started_5 = 0i32;
        if (((((p_value_2) == (0i32)) as i32)) != 0) {
            {
                l_i_6 = 0i32;
                'l1: loop {
                    if !(((((l_i_6) < ((p_count_3).wrapping_sub(1i32))) as i32)) != 0) { break 'l1; }
                    'c2: {
                        if (((((p_zeroPad_4) != (0i32)) as i32)) != 0) {
                            { let v_ = ((0i32) as u8); let a_ = p_digits_1; c.m.w8(a_, v_); };
                        } else {
                            { let v_ = ((10i32) as u8); let a_ = p_digits_1; c.m.w8(a_, v_); };
                        }
                        { let o_ = p_digits_1; let n_ = (o_).wrapping_add(1); p_digits_1 = n_; };
                    }
                    { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                }
            }
            { let v_ = ((0i32) as u8); let a_ = (p_digits_1).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = ((255i32) as u8); let a_ = (p_digits_1).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
        } else {
            l_i_6 = (p_count_3).wrapping_sub(1i32);
            p_value_2 = (if (((((p_value_2) < (0i32)) as i32)) != 0) { (p_value_2).wrapping_neg() } else { p_value_2 });
            {
                'l3: loop {
                    if !(((((l_i_6) > (0i32)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        l_d_7 = rt::div_i32(p_value_2, (c.m.r32((fp).wrapping_add((((l_i_6) as u32)).wrapping_mul(4))) as i32));
                        if (((((((((l_d_7) == (0i32)) as i32)) != 0) && ((l_started_5) != 0)) as i32)) != 0) {
                            { let v_ = ((0i32) as u8); let a_ = p_digits_1; c.m.w8(a_, v_); };
                            { let o_ = p_digits_1; let n_ = (o_).wrapping_add(1); p_digits_1 = n_; };
                        } else {
                            if (((((l_d_7) != (0i32)) as i32)) != 0) {
                                { let v_ = ((l_d_7) as u8); let a_ = p_digits_1; c.m.w8(a_, v_); };
                                { let o_ = p_digits_1; let n_ = (o_).wrapping_add(1); p_digits_1 = n_; };
                                { let r_ = (l_d_7).wrapping_mul((c.m.r32((fp).wrapping_add((((l_i_6) as u32)).wrapping_mul(4))) as i32)); let n_ = (p_value_2).wrapping_sub(r_); p_value_2 = n_; };
                                l_started_5 = 1i32;
                            } else {
                                if (((((p_zeroPad_4) != (0i32)) as i32)) != 0) {
                                    { let v_ = ((0i32) as u8); let a_ = p_digits_1; c.m.w8(a_, v_); };
                                    { let o_ = p_digits_1; let n_ = (o_).wrapping_add(1); p_digits_1 = n_; };
                                } else {
                                    { let v_ = ((10i32) as u8); let a_ = p_digits_1; c.m.w8(a_, v_); };
                                    { let o_ = p_digits_1; let n_ = (o_).wrapping_add(1); p_digits_1 = n_; };
                                }
                            }
                        }
                    }
                    { let o_ = l_i_6; let n_ = (o_).wrapping_sub(1); l_i_6 = n_; };
                }
            }
            { let v_ = ((p_value_2) as u8); let a_ = (p_digits_1).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = ((255i32) as u8); let a_ = (p_digits_1).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Num_ToDigits", &PROBE_N, &[], true); }
    r_
}

pub fn Num_DrawDigits(c: &mut Ctx, mut p_flash_1: u32, mut p_refs_2: u32, mut p_a2_3: i32, mut p_a3_4: i32, mut p_cells_5: u32, mut p_digits_6: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Num_DrawDigits", &PROBE_N, &[(p_flash_1) as u32, (p_refs_2) as u32, (p_a2_3) as u32, (p_a3_4) as u32, (p_cells_5) as u32, (p_digits_6) as u32], false); }
    let mut l_i_7: i32 = 0;
    let r_ = 'f: {
    {
        l_i_7 = 0i32;
        'l1: loop {
            if !(((((((c.m.r8(p_digits_6)) as i32)) != (255i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((((c.m.r8(p_digits_6)) as i32)) != (10i32)) as i32)) != 0) {
                    { let a2_ = (p_cells_5).wrapping_add((((c.m.r8(p_digits_6)) as u32)).wrapping_mul(20)); F::Flash_ClipSetUv(c, p_flash_1, (p_refs_2).wrapping_add((((l_i_7) as u32)).wrapping_mul(8)), a2_) };
                    { let _ = F::Flash_ClipSetFlags(c, p_flash_1, (p_refs_2).wrapping_add((((l_i_7) as u32)).wrapping_mul(8)), 2i32, ((((1i32) as i64 as u64)) as u8)); };
                } else {
                    { let _ = F::Flash_ClipSetFlags(c, p_flash_1, (p_refs_2).wrapping_add((((l_i_7) as u32)).wrapping_mul(8)), 2i32, ((((0i32) as i64 as u64)) as u8)); };
                }
                F::Flash_ClipSetOffset(c, p_flash_1, (p_refs_2).wrapping_add((((l_i_7) as u32)).wrapping_mul(8)), p_a2_3, p_a3_4);
                { let o_ = p_digits_6; let n_ = (o_).wrapping_add(1); p_digits_6 = n_; };
            }
            { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Num_DrawDigits", &PROBE_N, &[], true); }
    r_
}

pub fn Num_Pow(c: &mut Ctx, mut p_base_1: i32, mut p_exp_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Num_Pow", &PROBE_N, &[(p_base_1) as u32, (p_exp_2) as u32], false); }
    let mut l_result_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_result_3 = p_base_1;
    {
        l_i_4 = 1i32;
        'l1: loop {
            if !(((((l_i_4) < (p_exp_2)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let r_ = p_base_1; let n_ = (l_result_3).wrapping_mul(r_); l_result_3 = n_; };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    break 'f l_result_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Num_Pow", &PROBE_N, &[], true); }
    r_
}

pub fn Num_Draw(c: &mut Ctx, mut p_flash_1: u32, mut p_fmt_2: u32, mut p_first_3: i32, mut p_count_4: i32, mut p_value_5: i32, mut p_w_6: i32, mut p_h_7: i32, mut p_mode_8: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Num_Draw", &PROBE_N, &[(p_flash_1) as u32, (p_fmt_2) as u32, (p_first_3) as u32, (p_count_4) as u32, (p_value_5) as u32, (p_w_6) as u32, (p_h_7) as u32, (p_mode_8) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_started_9: i32 = 0;
    let mut l_i_10: i32 = 0;
    let mut l_d_11: i32 = 0;
    let mut l_digit_12: i32 = 0;
    let r_ = 'f: {
        l_started_9 = 0i32;
        if (((((p_count_4) == (0i32)) as i32)) != 0) {
            p_count_4 = F::Num_CountDigits(c, p_value_5);
        }
        l_i_10 = (p_count_4).wrapping_sub(1i32);
        p_value_5 = (if (((((p_value_5) < (0i32)) as i32)) != 0) { (p_value_5).wrapping_neg() } else { p_value_5 });
        {
            'l1: loop {
                if !(((((l_i_10) >= (0i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let _ = F::sprintf(c, fp.wrapping_add(8), p_fmt_2, &[(((l_i_10).wrapping_add(p_first_3)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, p_flash_1, ((0i32) as u32), fp.wrapping_add(8), fp);
                    if (((((p_mode_8) != (2i32)) as i32)) != 0) {
                        if (((((l_i_10) != (0i32)) as i32)) != 0) {
                            l_d_11 = rt::div_i32(p_value_5, F::Num_Pow(c, 10i32, l_i_10));
                        } else {
                            l_d_11 = p_value_5;
                        }
                        if (((((((((l_d_11) == (0i32)) as i32)) != 0) && ((l_started_9) != 0)) as i32)) != 0) {
                            l_digit_12 = 0i32;
                        } else {
                            if (((((l_d_11) != (0i32)) as i32)) != 0) {
                                if (((((l_i_10) != (0i32)) as i32)) != 0) {
                                    { let r_ = (l_d_11).wrapping_mul(F::Num_Pow(c, 10i32, l_i_10)); let n_ = (p_value_5).wrapping_sub(r_); p_value_5 = n_; };
                                } else {
                                    { let r_ = l_d_11; let n_ = (p_value_5).wrapping_sub(r_); p_value_5 = n_; };
                                }
                                l_digit_12 = l_d_11;
                                l_started_9 = 1i32;
                            } else {
                                if (((((((((p_mode_8) != (0i32)) as i32)) != 0) || (((((l_i_10) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                                    l_digit_12 = 0i32;
                                } else {
                                    { let _ = F::Flash_ClipSetFlags(c, p_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                                    break 'c2;
                                }
                            }
                        }
                    } else {
                        l_digit_12 = 10i32;
                    }
                    { let v_ = (p_h_7).wrapping_mul(rt::div_i32(l_digit_12, 4i32)); let a_ = (fp.wrapping_add(72)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((p_h_7).wrapping_mul(rt::div_i32(l_digit_12, 4i32))).wrapping_add(p_h_7); let a_ = (fp.wrapping_add(72)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (p_w_6).wrapping_mul(rt::rem_i32(l_digit_12, 4i32)); let a_ = fp.wrapping_add(72); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((p_w_6).wrapping_mul(rt::rem_i32(l_digit_12, 4i32))).wrapping_add(p_w_6); let a_ = (fp.wrapping_add(72)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::Flash_ClipSetFlags(c, p_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                    F::Flash_ClipSetUv(c, p_flash_1, fp, fp.wrapping_add(72));
                }
                { let o_ = l_i_10; let n_ = (o_).wrapping_sub(1); l_i_10 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Num_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn Num_DrawChild(c: &mut Ctx, mut p_flash_1: u32, mut p_parent_2: u32, mut p_fmt_3: u32, mut p_first_4: i32, mut p_count_5: i32, mut p_value_6: i32, mut p_w_7: i32, mut p_h_8: i32, mut p_mode_9: i32, mut p_parentFmt_10: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Num_DrawChild", &PROBE_N, &[(p_flash_1) as u32, (p_parent_2) as u32, (p_fmt_3) as u32, (p_first_4) as u32, (p_count_5) as u32, (p_value_6) as u32, (p_w_7) as u32, (p_h_8) as u32, (p_mode_9) as u32, (p_parentFmt_10) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_started_11: i32 = 0;
    let mut l_i_12: i32 = 0;
    let mut l_d_13: i32 = 0;
    let mut l_digit_14: i32 = 0;
    let r_ = 'f: {
        l_started_11 = 0i32;
        if (((((p_count_5) == (0i32)) as i32)) != 0) {
            p_count_5 = F::Num_CountDigits(c, p_value_6);
        }
        l_i_12 = (p_count_5).wrapping_sub(1i32);
        p_value_6 = (if (((((p_value_6) < (0i32)) as i32)) != 0) { (p_value_6).wrapping_neg() } else { p_value_6 });
        {
            'l1: loop {
                if !(((((l_i_12) >= (0i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((p_parentFmt_10) != (0i32)) as i32)) != 0) {
                        { let _ = F::sprintf(c, fp.wrapping_add(8), p_parent_2, &[(((l_i_12).wrapping_add(p_first_4)) as i64 as u64)]); };
                        F::Flash_FindLabel(c, p_flash_1, fp.wrapping_add(8), p_fmt_3, fp);
                    } else {
                        { let _ = F::sprintf(c, fp.wrapping_add(8), p_fmt_3, &[(((l_i_12).wrapping_add(p_first_4)) as i64 as u64)]); };
                        F::Flash_FindLabel(c, p_flash_1, p_parent_2, fp.wrapping_add(8), fp);
                    }
                    if (((((p_mode_9) != (2i32)) as i32)) != 0) {
                        if (((((l_i_12) != (0i32)) as i32)) != 0) {
                            l_d_13 = rt::div_i32(p_value_6, F::Num_Pow(c, 10i32, l_i_12));
                        } else {
                            l_d_13 = p_value_6;
                        }
                        if (((((((((l_d_13) == (0i32)) as i32)) != 0) && ((l_started_11) != 0)) as i32)) != 0) {
                            l_digit_14 = 0i32;
                        } else {
                            if (((((l_d_13) != (0i32)) as i32)) != 0) {
                                if (((((l_i_12) != (0i32)) as i32)) != 0) {
                                    { let r_ = (l_d_13).wrapping_mul(F::Num_Pow(c, 10i32, l_i_12)); let n_ = (p_value_6).wrapping_sub(r_); p_value_6 = n_; };
                                } else {
                                    { let r_ = l_d_13; let n_ = (p_value_6).wrapping_sub(r_); p_value_6 = n_; };
                                }
                                l_digit_14 = l_d_13;
                                l_started_11 = 1i32;
                            } else {
                                if (((((((((p_mode_9) != (0i32)) as i32)) != 0) || (((((l_i_12) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                                    l_digit_14 = 0i32;
                                } else {
                                    { let _ = F::Flash_ClipSetFlags(c, p_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                                    break 'c2;
                                }
                            }
                        }
                    } else {
                        l_digit_14 = 10i32;
                    }
                    { let v_ = (p_h_8).wrapping_mul(rt::div_i32(l_digit_14, 4i32)); let a_ = (fp.wrapping_add(72)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((p_h_8).wrapping_mul(rt::div_i32(l_digit_14, 4i32))).wrapping_add(p_h_8); let a_ = (fp.wrapping_add(72)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (p_w_7).wrapping_mul(rt::rem_i32(l_digit_14, 4i32)); let a_ = fp.wrapping_add(72); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((p_w_7).wrapping_mul(rt::rem_i32(l_digit_14, 4i32))).wrapping_add(p_w_7); let a_ = (fp.wrapping_add(72)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::Flash_ClipSetFlags(c, p_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                    F::Flash_ClipSetUv(c, p_flash_1, fp, fp.wrapping_add(72));
                }
                { let o_ = l_i_12; let n_ = (o_).wrapping_sub(1); l_i_12 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Num_DrawChild", &PROBE_N, &[], true); }
    r_
}

pub fn Num_CountDigits(c: &mut Ctx, mut p_value_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Num_CountDigits", &PROBE_N, &[(p_value_1) as u32], false); }
    let mut l_digits_2: i32 = 0;
    let r_ = 'f: {
    l_digits_2 = 1i32;
    p_value_1 = (p_value_1).wrapping_abs();
    'l1: while (((((p_value_1) >= (F::Num_Pow(c, 10i32, l_digits_2))) as i32)) != 0) {
        { let o_ = l_digits_2; let n_ = (o_).wrapping_add(1); l_digits_2 = n_; };
    }
    break 'f l_digits_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Num_CountDigits", &PROBE_N, &[], true); }
    r_
}

pub fn Num_DrawEx(c: &mut Ctx, mut p_flash_1: u32, mut p_style_2: u32, mut p_a_3: u32, mut p_b_4: u32, mut p_value_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Num_DrawEx", &PROBE_N, &[(p_flash_1) as u32, (p_style_2) as u32, (p_a_3) as u32, (p_b_4) as u32, (p_value_5) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_digit_6: i32 = 0;
    let mut l_sign_7: i32 = 0;
    let mut l_started_8: i32 = 0;
    let mut l_signPos_9: i32 = 0;
    let mut l_count_10: i32 = 0;
    let mut l_i_11: i32 = 0;
    let mut l_d_12: i32 = 0;
    let mut l_flags_13: i32 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
        'fl1: loop {
            match bb_ {
                0 => {
                    l_digit_6 = 0i32;
                    l_sign_7 = 0i32;
                    l_started_8 = 0i32;
                    l_signPos_9 = 0i32;
                    if ((((((c.m.r32((p_style_2).wrapping_add(24)) as i32)) == (0i32)) as i32)) != 0) { bb_ = 1; continue 'fl1; } else { bb_ = 3; continue 'fl1; }
                }
                1 => {
                    l_count_10 = F::Num_CountDigits(c, p_value_5);
                    bb_ = 2; continue 'fl1;
                }
                2 => {
                    if (((((c.m.r32(p_style_2) as i32)) & (2i32))) != 0) { bb_ = 4; continue 'fl1; } else { bb_ = 5; continue 'fl1; }
                }
                3 => {
                    l_count_10 = (c.m.r32((p_style_2).wrapping_add(24)) as i32);
                    bb_ = 2; continue 'fl1;
                }
                4 => {
                    if (((((p_value_5) <= (0i32)) as i32)) != 0) { bb_ = 6; continue 'fl1; } else { bb_ = 8; continue 'fl1; }
                }
                5 => {
                    l_i_11 = (l_count_10).wrapping_sub(1i32);
                    p_value_5 = (if (((((p_value_5) < (0i32)) as i32)) != 0) { (p_value_5).wrapping_neg() } else { p_value_5 });
                    bb_ = 12; continue 'fl1;
                }
                6 => {
                    if (((((p_value_5) < (0i32)) as i32)) != 0) { bb_ = 9; continue 'fl1; } else { bb_ = 11; continue 'fl1; }
                }
                7 => {
                    l_signPos_9 = F::Num_CountDigits(c, p_value_5);
                    bb_ = 5; continue 'fl1;
                }
                8 => {
                    l_sign_7 = 1i32;
                    bb_ = 7; continue 'fl1;
                }
                9 => {
                    l_sign_7 = (1i32).wrapping_neg();
                    bb_ = 10; continue 'fl1;
                }
                10 => {
                    bb_ = 7; continue 'fl1;
                }
                11 => {
                    l_sign_7 = 0i32;
                    bb_ = 10; continue 'fl1;
                }
                12 => {
                    if (((((l_i_11) >= (0i32)) as i32)) != 0) { bb_ = 13; continue 'fl1; } else { bb_ = 15; continue 'fl1; }
                }
                13 => {
                    if (((((p_b_4) == (((0i32) as u32))) as i32)) != 0) { bb_ = 16; continue 'fl1; } else { bb_ = 18; continue 'fl1; }
                }
                14 => {
                    { let o_ = l_i_11; let n_ = (o_).wrapping_sub(1); l_i_11 = n_; };
                    bb_ = 12; continue 'fl1;
                }
                15 => {
                    break 'fl1;
                }
                16 => {
                    { let _ = { let a2_ = (l_i_11).wrapping_add((c.m.r32((p_style_2).wrapping_add(20)) as i32)); F::sprintf(c, fp.wrapping_add(8), p_a_3, &[((a2_) as i64 as u64)]) }; };
                    F::Flash_FindLabel(c, p_flash_1, ((0i32) as u32), fp.wrapping_add(8), fp);
                    bb_ = 17; continue 'fl1;
                }
                17 => {
                    l_flags_13 = (c.m.r32(p_style_2) as i32);
                    if ((((l_flags_13) & (16i32))) != 0) { bb_ = 24; continue 'fl1; } else { bb_ = 26; continue 'fl1; }
                }
                18 => {
                    if (((((c.m.r32(p_style_2) as i32)) & (8i32))) != 0) { bb_ = 19; continue 'fl1; } else { bb_ = 21; continue 'fl1; }
                }
                19 => {
                    { let _ = { let a2_ = (l_i_11).wrapping_add((c.m.r32((p_style_2).wrapping_add(20)) as i32)); F::sprintf(c, fp.wrapping_add(8), p_a_3, &[((a2_) as i64 as u64)]) }; };
                    F::Flash_FindLabel(c, p_flash_1, fp.wrapping_add(8), p_b_4, fp);
                    bb_ = 20; continue 'fl1;
                }
                20 => {
                    bb_ = 17; continue 'fl1;
                }
                21 => {
                    if (((((c.m.r32(p_style_2) as i32)) & (4i32))) != 0) { bb_ = 22; continue 'fl1; } else { bb_ = 23; continue 'fl1; }
                }
                22 => {
                    { let _ = { let a2_ = (l_i_11).wrapping_add((c.m.r32((p_style_2).wrapping_add(20)) as i32)); F::sprintf(c, fp.wrapping_add(8), p_b_4, &[((a2_) as i64 as u64)]) }; };
                    F::Flash_FindLabel(c, p_flash_1, p_a_3, fp.wrapping_add(8), fp);
                    bb_ = 23; continue 'fl1;
                }
                23 => {
                    bb_ = 20; continue 'fl1;
                }
                24 => {
                    l_digit_6 = 10i32;
                    bb_ = 25; continue 'fl1;
                }
                25 => {
                    bb_ = 46; continue 'fl1;
                }
                26 => {
                    if (((((l_i_11) != (0i32)) as i32)) != 0) { bb_ = 27; continue 'fl1; } else { bb_ = 29; continue 'fl1; }
                }
                27 => {
                    l_d_12 = rt::div_i32(p_value_5, F::Num_Pow(c, 10i32, l_i_11));
                    bb_ = 28; continue 'fl1;
                }
                28 => {
                    if (((((((((l_d_12) == (0i32)) as i32)) != 0) && ((l_started_8) != 0)) as i32)) != 0) { bb_ = 30; continue 'fl1; } else { bb_ = 32; continue 'fl1; }
                }
                29 => {
                    l_d_12 = p_value_5;
                    bb_ = 28; continue 'fl1;
                }
                30 => {
                    l_digit_6 = 0i32;
                    bb_ = 31; continue 'fl1;
                }
                31 => {
                    bb_ = 25; continue 'fl1;
                }
                32 => {
                    if (((((l_d_12) != (0i32)) as i32)) != 0) { bb_ = 33; continue 'fl1; } else { bb_ = 35; continue 'fl1; }
                }
                33 => {
                    if (((((l_i_11) != (0i32)) as i32)) != 0) { bb_ = 36; continue 'fl1; } else { bb_ = 38; continue 'fl1; }
                }
                34 => {
                    bb_ = 31; continue 'fl1;
                }
                35 => {
                    if ((((((((l_flags_13) & (1i32))) != 0) || (((((l_i_11) == (0i32)) as i32)) != 0)) as i32)) != 0) { bb_ = 39; continue 'fl1; } else { bb_ = 41; continue 'fl1; }
                }
                36 => {
                    { let r_ = (l_d_12).wrapping_mul(F::Num_Pow(c, 10i32, l_i_11)); let n_ = (p_value_5).wrapping_sub(r_); p_value_5 = n_; };
                    bb_ = 37; continue 'fl1;
                }
                37 => {
                    l_digit_6 = l_d_12;
                    l_started_8 = 1i32;
                    bb_ = 34; continue 'fl1;
                }
                38 => {
                    { let r_ = l_d_12; let n_ = (p_value_5).wrapping_sub(r_); p_value_5 = n_; };
                    bb_ = 37; continue 'fl1;
                }
                39 => {
                    l_digit_6 = 0i32;
                    bb_ = 40; continue 'fl1;
                }
                40 => {
                    bb_ = 34; continue 'fl1;
                }
                41 => {
                    if ((((((((((((l_flags_13) & (2i32))) != 0) && (((((l_signPos_9) == (l_i_11)) as i32)) != 0)) as i32)) != 0) && (((((l_sign_7) != (0i32)) as i32)) != 0)) as i32)) != 0) { bb_ = 42; continue 'fl1; } else { bb_ = 43; continue 'fl1; }
                }
                42 => {
                    if (((((l_sign_7) > (0i32)) as i32)) != 0) { bb_ = 44; continue 'fl1; } else { bb_ = 45; continue 'fl1; }
                }
                43 => {
                    { let _ = F::Flash_ClipSetFlags(c, p_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    bb_ = 14; continue 'fl1;
                }
                44 => {
                    l_digit_6 = 10i32;
                    bb_ = 46; continue 'fl1;
                }
                45 => {
                    if (((((l_sign_7) < (0i32)) as i32)) != 0) { bb_ = 48; continue 'fl1; } else { bb_ = 49; continue 'fl1; }
                }
                46 => {
                    { let v_ = ((c.m.r32((p_style_2).wrapping_add(16)) as i32)).wrapping_mul(rt::div_i32(l_digit_6, 4i32)); let a_ = (fp.wrapping_add(72)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (((c.m.r32((p_style_2).wrapping_add(16)) as i32)).wrapping_mul(rt::div_i32(l_digit_6, 4i32))).wrapping_add((c.m.r32((p_style_2).wrapping_add(16)) as i32)); let a_ = (fp.wrapping_add(72)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32((p_style_2).wrapping_add(12)) as i32)).wrapping_mul(rt::rem_i32(l_digit_6, 4i32)); let a_ = fp.wrapping_add(72); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (((c.m.r32((p_style_2).wrapping_add(12)) as i32)).wrapping_mul(rt::rem_i32(l_digit_6, 4i32))).wrapping_add((c.m.r32((p_style_2).wrapping_add(12)) as i32)); let a_ = (fp.wrapping_add(72)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::Flash_ClipSetFlags(c, p_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                    F::Flash_ClipSetUv(c, p_flash_1, fp, fp.wrapping_add(72));
                    bb_ = 14; continue 'fl1;
                }
                47 => {
                    bb_ = 45; continue 'fl1;
                }
                48 => {
                    l_digit_6 = 10i32;
                    bb_ = 49; continue 'fl1;
                }
                49 => {
                    bb_ = 46; continue 'fl1;
                }
                50 => {
                    bb_ = 43; continue 'fl1;
                }
                51 => {
                    bb_ = 40; continue 'fl1;
                }
                _ => unreachable!(),
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Num_DrawEx", &PROBE_N, &[], true); }
    r_
}

pub fn ChrGrid_IsSelectable(c: &mut Ctx, mut p_cells_1: u32, mut p_index_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrGrid_IsSelectable", &PROBE_N, &[(p_cells_1) as u32, (p_index_2) as u32], false); }
    let mut l_ok_3: i32 = 0;
    let r_ = 'f: {
    l_ok_3 = 1i32;
    if ((((((c.m.r32((p_cells_1).wrapping_add((((p_index_2) as u32)).wrapping_mul(36))) as i32)) >= (162i32)) as i32)) != 0) {
        l_ok_3 = ((((c.m.r32((p_cells_1).wrapping_add((((p_index_2) as u32)).wrapping_mul(36))) as i32)) == (163i32)) as i32);
    }
    break 'f l_ok_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrGrid_IsSelectable", &PROBE_N, &[], true); }
    r_
}

pub fn ChrGrid_MoveRight(c: &mut Ctx, mut p_cells_1: u32, mut p_col_2: u32, mut p_row_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrGrid_MoveRight", &PROBE_N, &[(p_cells_1) as u32, (p_col_2) as u32, (p_row_3) as u32], false); }
    let mut l_start_4: i32 = 0;
    let mut l_ok_5: i32 = 0;
    let mut l_id_6: i32 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
    'fl1: loop {
        match bb_ {
            0 => {
                l_start_4 = (c.m.r32(p_col_2) as i32);
                l_ok_5 = 1i32;
                bb_ = 1; continue 'fl1;
            }
            1 => {
                bb_ = 2; continue 'fl1;
            }
            2 => {
                { let a_ = p_col_2; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                if ((((((c.m.r32(p_col_2) as i32)) >= (7i32)) as i32)) != 0) { bb_ = 5; continue 'fl1; } else { bb_ = 6; continue 'fl1; }
            }
            3 => {
                bb_ = 1; continue 'fl1;
            }
            4 => {
                bb_ = 15; continue 'fl1;
            }
            5 => {
                { let v_ = 0i32; let a_ = p_col_2; c.m.w32(a_, (v_) as u32); };
                bb_ = 6; continue 'fl1;
            }
            6 => {
                if (((((l_start_4) == ((c.m.r32(p_col_2) as i32))) as i32)) != 0) { bb_ = 7; continue 'fl1; } else { bb_ = 8; continue 'fl1; }
            }
            7 => {
                l_ok_5 = 0i32;
                bb_ = 4; continue 'fl1;
            }
            8 => {
                l_id_6 = { let a_ = (p_cells_1).wrapping_add((((((p_row_3).wrapping_mul(7i32)).wrapping_add((c.m.r32(p_col_2) as i32))) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) };
                if (((((l_id_6) < (162i32)) as i32)) != 0) { bb_ = 10; continue 'fl1; } else { bb_ = 11; continue 'fl1; }
            }
            9 => {
                bb_ = 8; continue 'fl1;
            }
            10 => {
                bb_ = 4; continue 'fl1;
            }
            11 => {
                if (((((l_id_6) == (163i32)) as i32)) != 0) { bb_ = 13; continue 'fl1; } else { bb_ = 14; continue 'fl1; }
            }
            12 => {
                bb_ = 11; continue 'fl1;
            }
            13 => {
                bb_ = 15; continue 'fl1;
            }
            14 => {
                bb_ = 3; continue 'fl1;
            }
            15 => {
                break 'f l_ok_5;
            }
            16 => {
                bb_ = 14; continue 'fl1;
            }
            17 => {
                break 'fl1;
            }
            _ => unreachable!(),
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrGrid_MoveRight", &PROBE_N, &[], true); }
    r_
}

pub fn ChrGrid_MoveLeft(c: &mut Ctx, mut p_cells_1: u32, mut p_col_2: u32, mut p_row_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrGrid_MoveLeft", &PROBE_N, &[(p_cells_1) as u32, (p_col_2) as u32, (p_row_3) as u32], false); }
    let mut l_start_4: i32 = 0;
    let mut l_ok_5: i32 = 0;
    let mut l_id_6: i32 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
    'fl1: loop {
        match bb_ {
            0 => {
                l_start_4 = (c.m.r32(p_col_2) as i32);
                l_ok_5 = 1i32;
                bb_ = 1; continue 'fl1;
            }
            1 => {
                bb_ = 2; continue 'fl1;
            }
            2 => {
                { let a_ = p_col_2; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                if ((((((c.m.r32(p_col_2) as i32)) < (0i32)) as i32)) != 0) { bb_ = 5; continue 'fl1; } else { bb_ = 6; continue 'fl1; }
            }
            3 => {
                bb_ = 1; continue 'fl1;
            }
            4 => {
                bb_ = 15; continue 'fl1;
            }
            5 => {
                { let v_ = (7i32).wrapping_sub(1i32); let a_ = p_col_2; c.m.w32(a_, (v_) as u32); };
                bb_ = 6; continue 'fl1;
            }
            6 => {
                if (((((l_start_4) == ((c.m.r32(p_col_2) as i32))) as i32)) != 0) { bb_ = 7; continue 'fl1; } else { bb_ = 8; continue 'fl1; }
            }
            7 => {
                l_ok_5 = 0i32;
                bb_ = 4; continue 'fl1;
            }
            8 => {
                l_id_6 = { let a_ = (p_cells_1).wrapping_add((((((p_row_3).wrapping_mul(7i32)).wrapping_add((c.m.r32(p_col_2) as i32))) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) };
                if (((((l_id_6) < (162i32)) as i32)) != 0) { bb_ = 10; continue 'fl1; } else { bb_ = 11; continue 'fl1; }
            }
            9 => {
                bb_ = 8; continue 'fl1;
            }
            10 => {
                bb_ = 4; continue 'fl1;
            }
            11 => {
                if (((((l_id_6) == (163i32)) as i32)) != 0) { bb_ = 13; continue 'fl1; } else { bb_ = 14; continue 'fl1; }
            }
            12 => {
                bb_ = 11; continue 'fl1;
            }
            13 => {
                bb_ = 15; continue 'fl1;
            }
            14 => {
                bb_ = 3; continue 'fl1;
            }
            15 => {
                break 'f l_ok_5;
            }
            16 => {
                bb_ = 14; continue 'fl1;
            }
            17 => {
                break 'fl1;
            }
            _ => unreachable!(),
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrGrid_MoveLeft", &PROBE_N, &[], true); }
    r_
}

pub fn ChrGrid_MoveDown(c: &mut Ctx, mut p_cells_1: u32, mut p_col_2: u32, mut p_row_3: u32, mut p_rows_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrGrid_MoveDown", &PROBE_N, &[(p_cells_1) as u32, (p_col_2) as u32, (p_row_3) as u32, (p_rows_4) as u32], false); }
    let mut l_id_5: i32 = 0;
    let r_ = 'f: {
    'l1: loop {
        'c2: {
            { let a_ = p_row_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            if ((((((c.m.r32(p_row_3) as i32)) > ((p_rows_4).wrapping_sub(1i32))) as i32)) != 0) {
                { let v_ = 0i32; let a_ = p_row_3; c.m.w32(a_, (v_) as u32); };
            }
            l_id_5 = { let a_ = (p_cells_1).wrapping_add(((((((c.m.r32(p_row_3) as i32)).wrapping_mul(7i32)).wrapping_add((c.m.r32(p_col_2) as i32))) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) };
            if (((((l_id_5) < (162i32)) as i32)) != 0) {
                break 'f;
            }
            if (((((l_id_5) == (163i32)) as i32)) != 0) {
                break 'f;
            }
        }
        if !((((({ let a2_ = (c.m.r32(p_row_3) as i32); F::ChrGrid_MoveRight(c, p_cells_1, p_col_2, a2_) }) == (0i32)) as i32)) != 0) { break 'l1; }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrGrid_MoveDown", &PROBE_N, &[], true); }
    r_
}

pub fn ChrGrid_MoveUp(c: &mut Ctx, mut p_cells_1: u32, mut p_col_2: u32, mut p_row_3: u32, mut p_rows_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrGrid_MoveUp", &PROBE_N, &[(p_cells_1) as u32, (p_col_2) as u32, (p_row_3) as u32, (p_rows_4) as u32], false); }
    let mut l_id_5: i32 = 0;
    let r_ = 'f: {
    'l1: loop {
        'c2: {
            { let a_ = p_row_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
            if ((((((c.m.r32(p_row_3) as i32)) < (0i32)) as i32)) != 0) {
                { let v_ = (p_rows_4).wrapping_sub(1i32); let a_ = p_row_3; c.m.w32(a_, (v_) as u32); };
            }
            l_id_5 = { let a_ = (p_cells_1).wrapping_add(((((((c.m.r32(p_row_3) as i32)).wrapping_mul(7i32)).wrapping_add((c.m.r32(p_col_2) as i32))) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) };
            if (((((l_id_5) < (162i32)) as i32)) != 0) {
                break 'f;
            }
            if (((((l_id_5) == (163i32)) as i32)) != 0) {
                break 'f;
            }
        }
        if !((((({ let a2_ = (c.m.r32(p_row_3) as i32); F::ChrGrid_MoveRight(c, p_cells_1, p_col_2, a2_) }) == (0i32)) as i32)) != 0) { break 'l1; }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrGrid_MoveUp", &PROBE_N, &[], true); }
    r_
}

pub fn ChrGrid_NextForm(c: &mut Ctx, mut p_forms_1: u32, mut p_index_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrGrid_NextForm", &PROBE_N, &[(p_forms_1) as u32, (p_index_2) as u32], false); }
    let mut l_start_3: i32 = 0;
    let r_ = 'f: {
    l_start_3 = (c.m.r32(p_index_2) as i32);
    'l1: loop {
        'c2: {
            { let a_ = p_index_2; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            if ((((((c.m.r32(p_index_2) as i32)) >= (7i32)) as i32)) != 0) {
                { let v_ = 0i32; let a_ = p_index_2; c.m.w32(a_, (v_) as u32); };
            }
            if (((((l_start_3) == ((c.m.r32(p_index_2) as i32))) as i32)) != 0) {
                break 'l1;
            }
        }
        if !((((({ let a_ = (p_forms_1).wrapping_add(((((c.m.r32(p_index_2) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= (161i32)) as i32)) != 0) { break 'l1; }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrGrid_NextForm", &PROBE_N, &[], true); }
    r_
}

pub fn ChrGrid_PrevForm(c: &mut Ctx, mut p_forms_1: u32, mut p_index_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrGrid_PrevForm", &PROBE_N, &[(p_forms_1) as u32, (p_index_2) as u32], false); }
    let mut l_start_3: i32 = 0;
    let r_ = 'f: {
    l_start_3 = (c.m.r32(p_index_2) as i32);
    'l1: loop {
        'c2: {
            { let a_ = p_index_2; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
            if ((((((c.m.r32(p_index_2) as i32)) < (0i32)) as i32)) != 0) {
                { let v_ = (7i32).wrapping_sub(1i32); let a_ = p_index_2; c.m.w32(a_, (v_) as u32); };
            }
            if (((((l_start_3) == ((c.m.r32(p_index_2) as i32))) as i32)) != 0) {
                break 'l1;
            }
        }
        if !((((({ let a_ = (p_forms_1).wrapping_add(((((c.m.r32(p_index_2) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= (161i32)) as i32)) != 0) { break 'l1; }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrGrid_PrevForm", &PROBE_N, &[], true); }
    r_
}

pub fn ChrGrid_FixCursor(c: &mut Ctx, mut p_cells_1: u32, mut p_col_2: u32, mut p_row_3: u32, mut p_rows_4: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrGrid_FixCursor", &PROBE_N, &[(p_cells_1) as u32, (p_col_2) as u32, (p_row_3) as u32, (p_rows_4) as u32], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (p_cells_1).wrapping_add(((((((c.m.r32(p_row_3) as i32)).wrapping_mul(7i32)).wrapping_add((c.m.r32(p_col_2) as i32))) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) }) >= (161i32)) as i32)) != 0) {
        if ((((({ let a2_ = (c.m.r32(p_row_3) as i32); F::ChrGrid_MoveRight(c, p_cells_1, p_col_2, a2_) }) == (0i32)) as i32)) != 0) {
            F::ChrGrid_MoveDown(c, p_cells_1, p_col_2, p_row_3, p_rows_4);
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrGrid_FixCursor", &PROBE_N, &[], true); }
    r_
}

pub fn StgGrid_IsSelectable(c: &mut Ctx, mut p_ids_1: u32, mut p_index_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgGrid_IsSelectable", &PROBE_N, &[(p_ids_1) as u32, (p_index_2) as u32], false); }
    let r_ = 'f: {
    break 'f ((((c.m.r32((p_ids_1).wrapping_add((((p_index_2) as u32)).wrapping_mul(4))) as i32)) < (62i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "StgGrid_IsSelectable", &PROBE_N, &[], true); }
    r_
}

pub fn StgGrid_MoveRight(c: &mut Ctx, mut p_ids_1: u32, mut p_col_2: u32, mut p_row_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgGrid_MoveRight", &PROBE_N, &[(p_ids_1) as u32, (p_col_2) as u32, (p_row_3) as u32], false); }
    let r_ = 'f: {
    'l1: loop {
        'c2: {
            { let a_ = p_col_2; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            if ((((((c.m.r32(p_col_2) as i32)) >= (7i32)) as i32)) != 0) {
                { let v_ = 0i32; let a_ = p_col_2; c.m.w32(a_, (v_) as u32); };
            } else {
                if ((((((c.m.r32(p_col_2) as i32)) == (6i32)) as i32)) != 0) {
                    break 'l1;
                }
            }
        }
        if !((((({ let a_ = (p_ids_1).wrapping_add((((((p_row_3).wrapping_mul(6i32)).wrapping_add((c.m.r32(p_col_2) as i32))) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= (62i32)) as i32)) != 0) { break 'l1; }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgGrid_MoveRight", &PROBE_N, &[], true); }
    r_
}

pub fn StgGrid_MoveLeft(c: &mut Ctx, mut p_ids_1: u32, mut p_col_2: u32, mut p_row_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgGrid_MoveLeft", &PROBE_N, &[(p_ids_1) as u32, (p_col_2) as u32, (p_row_3) as u32], false); }
    let r_ = 'f: {
    {
        'l1: loop {
            'c2: {
                { let a_ = p_col_2; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                if ((((((c.m.r32(p_col_2) as i32)) < (0i32)) as i32)) != 0) {
                    { let v_ = 6i32; let a_ = p_col_2; c.m.w32(a_, (v_) as u32); };
                    break 'f;
                }
                if ((((({ let a_ = (p_ids_1).wrapping_add((((((p_row_3).wrapping_mul(6i32)).wrapping_add((c.m.r32(p_col_2) as i32))) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (62i32)) as i32)) != 0) {
                    break 'f;
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgGrid_MoveLeft", &PROBE_N, &[], true); }
    r_
}

pub fn StgGrid_MoveRightWrap(c: &mut Ctx, mut p_ids_1: u32, mut p_col_2: u32, mut p_row_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgGrid_MoveRightWrap", &PROBE_N, &[(p_ids_1) as u32, (p_col_2) as u32, (p_row_3) as u32], false); }
    let mut l_start_4: i32 = 0;
    let mut l_ok_5: i32 = 0;
    let r_ = 'f: {
    l_start_4 = (c.m.r32(p_col_2) as i32);
    l_ok_5 = 1i32;
    'l1: loop {
        'c2: {
            { let a_ = p_col_2; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            if ((((((c.m.r32(p_col_2) as i32)) >= (6i32)) as i32)) != 0) {
                { let v_ = 0i32; let a_ = p_col_2; c.m.w32(a_, (v_) as u32); };
            }
            if (((((l_start_4) == ((c.m.r32(p_col_2) as i32))) as i32)) != 0) {
                l_ok_5 = 0i32;
                break 'l1;
            }
        }
        if !((((({ let a_ = (p_ids_1).wrapping_add((((((p_row_3).wrapping_mul(6i32)).wrapping_add((c.m.r32(p_col_2) as i32))) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= (62i32)) as i32)) != 0) { break 'l1; }
    }
    break 'f l_ok_5;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "StgGrid_MoveRightWrap", &PROBE_N, &[], true); }
    r_
}

pub fn StgGrid_MoveDown(c: &mut Ctx, mut p_ids_1: u32, mut p_col_2: u32, mut p_row_3: u32, mut p_rows_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgGrid_MoveDown", &PROBE_N, &[(p_ids_1) as u32, (p_col_2) as u32, (p_row_3) as u32, (p_rows_4) as u32], false); }
    let r_ = 'f: {
    {
        'l1: loop {
            'c2: {
                { let a_ = p_row_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                if ((((((c.m.r32(p_row_3) as i32)) > ((p_rows_4).wrapping_sub(1i32))) as i32)) != 0) {
                    { let v_ = 0i32; let a_ = p_row_3; c.m.w32(a_, (v_) as u32); };
                }
                if ((((({ let a_ = (p_ids_1).wrapping_add(((((((c.m.r32(p_row_3) as i32)).wrapping_mul(6i32)).wrapping_add((c.m.r32(p_col_2) as i32))) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (62i32)) as i32)) != 0) {
                    break 'l1;
                }
                if ((((({ let a2_ = (c.m.r32(p_row_3) as i32); F::StgGrid_MoveRightWrap(c, p_ids_1, p_col_2, a2_) }) != (0i32)) as i32)) != 0) {
                    break 'f;
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgGrid_MoveDown", &PROBE_N, &[], true); }
    r_
}

pub fn StgGrid_MoveUp(c: &mut Ctx, mut p_ids_1: u32, mut p_col_2: u32, mut p_row_3: u32, mut p_rows_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgGrid_MoveUp", &PROBE_N, &[(p_ids_1) as u32, (p_col_2) as u32, (p_row_3) as u32, (p_rows_4) as u32], false); }
    let r_ = 'f: {
    {
        'l1: loop {
            'c2: {
                { let a_ = p_row_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                if ((((((c.m.r32(p_row_3) as i32)) < (0i32)) as i32)) != 0) {
                    { let v_ = (p_rows_4).wrapping_sub(1i32); let a_ = p_row_3; c.m.w32(a_, (v_) as u32); };
                }
                if ((((({ let a_ = (p_ids_1).wrapping_add(((((((c.m.r32(p_row_3) as i32)).wrapping_mul(6i32)).wrapping_add((c.m.r32(p_col_2) as i32))) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (62i32)) as i32)) != 0) {
                    break 'l1;
                }
                if ((((({ let a2_ = (c.m.r32(p_row_3) as i32); F::StgGrid_MoveRightWrap(c, p_ids_1, p_col_2, a2_) }) != (0i32)) as i32)) != 0) {
                    break 'f;
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgGrid_MoveUp", &PROBE_N, &[], true); }
    r_
}

pub fn ChrGrid_Build(c: &mut Ctx, mut p_outCount_1: u32, mut p_out_2: u32, mut p_inCount_3: u32, mut p_in_4: u32, mut p_customCount_5: u32, mut p_custom_6: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrGrid_Build", &PROBE_N, &[(p_outCount_1) as u32, (p_out_2) as u32, (p_inCount_3) as u32, (p_in_4) as u32, (p_customCount_5) as u32, (p_custom_6) as u32], false); }
    let mut l_flags_7: i32 = 0;
    let mut l_mode_8: i32 = 0;
    let mut l_i_9: i32 = 0;
    let mut l_j_10: i32 = 0;
    let mut l_pad_11: i32 = 0;
    let r_ = 'f: {
    l_flags_7 = 0i32;
    if ((((((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); (c.m.r32(a_) as i32) }) >= (38i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); (c.m.r32(a_) as i32) }) < (42i32)) as i32)) != 0)) as i32)) != 0) {
        if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); (c.m.r32(a_) as i32) }) == (40i32)) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1572); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) {
                { let r_ = 4i32; let n_ = ((l_flags_7) | (r_)); l_flags_7 = n_; };
            }
        }
    }
    l_mode_8 = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); (c.m.r32(a_) as i32) };
    if (((((((((l_mode_8) >= (33i32)) as i32)) != 0) && (((((l_mode_8) < (36i32)) as i32)) != 0)) as i32)) != 0) {
        { let r_ = 2i32; let n_ = ((l_flags_7) | (r_)); l_flags_7 = n_; };
    }
    if (((((((((l_mode_8) >= (13i32)) as i32)) != 0) && (((((l_mode_8) < (31i32)) as i32)) != 0)) as i32)) != 0) {
        { let r_ = 2i32; let n_ = ((l_flags_7) | (r_)); l_flags_7 = n_; };
        { let r_ = 4i32; let n_ = ((l_flags_7) | (r_)); l_flags_7 = n_; };
        if (((((l_mode_8) == (21i32)) as i32)) != 0) {
            { let r_ = 8i32; let n_ = ((l_flags_7) | (r_)); l_flags_7 = n_; };
        }
    }
    { let v_ = 0i32; let a_ = p_outCount_1; c.m.w32(a_, (v_) as u32); };
    if (((((((((l_mode_8) >= (48i32)) as i32)) != 0) && (((((l_mode_8) < (51i32)) as i32)) != 0)) as i32)) != 0) {
        { let r_ = 2i32; let n_ = ((l_flags_7) | (r_)); l_flags_7 = n_; };
        { let r_ = 4i32; let n_ = ((l_flags_7) | (r_)); l_flags_7 = n_; };
        { let r_ = 8i32; let n_ = ((l_flags_7) | (r_)); l_flags_7 = n_; };
    }
    {
        l_i_9 = 0i32;
        'l1: loop {
            if !(((((l_i_9) < ((c.m.r32(p_inCount_3) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32(((p_in_4).wrapping_add((((l_i_9) as u32)).wrapping_mul(36))).wrapping_add(4)) as i32)) != (0i32)) as i32)) != 0) {
                    { let v_ = 0i32; let a_ = ((p_out_2).wrapping_add(((((c.m.r32(p_outCount_1) as i32)) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    {
                        l_j_10 = 0i32;
                        'l3: loop {
                            if !(((((l_j_10) < ((c.m.r32(((p_in_4).wrapping_add((((l_i_9) as u32)).wrapping_mul(36))).wrapping_add(4)) as i32))) as i32)) != 0) { break 'l3; }
                            'c4: {
                                if ((((((((l_flags_7) & (1i32))) != 0) || ((((((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3088)).wrapping_add((((rt::div_i32((c.m.r32((((p_in_4).wrapping_add((((l_i_9) as u32)).wrapping_mul(36))).wrapping_add(8)).wrapping_add((((l_j_10) as u32)).wrapping_mul(4))) as i32), 64i32)) as u32)).wrapping_mul(8)); c.m.r64(a_) }).wrapping_shr((rt::rem_i32((c.m.r32((((p_in_4).wrapping_add((((l_i_9) as u32)).wrapping_mul(36))).wrapping_add(8)).wrapping_add((((l_j_10) as u32)).wrapping_mul(4))) as i32), 64i32)) as u32)) & (((1i32) as u64)))) as i32)) != 0)) as i32)) != 0) {
                                    { let v_ = (c.m.r32((((p_in_4).wrapping_add((((l_i_9) as u32)).wrapping_mul(36))).wrapping_add(8)).wrapping_add((((l_j_10) as u32)).wrapping_mul(4))) as i32); let a_ = (((p_out_2).wrapping_add(((((c.m.r32(p_outCount_1) as i32)) as u32)).wrapping_mul(36))).wrapping_add(8)).wrapping_add(((({ let a_ = ((p_out_2).wrapping_add(((((c.m.r32(p_outCount_1) as i32)) as u32)).wrapping_mul(36))).wrapping_add(4); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                    { let a_ = ((p_out_2).wrapping_add(((((c.m.r32(p_outCount_1) as i32)) as u32)).wrapping_mul(36))).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                }
                            }
                            { let o_ = l_j_10; let n_ = (o_).wrapping_add(1); l_j_10 = n_; };
                        }
                    }
                    if ((((({ let a_ = ((p_out_2).wrapping_add(((((c.m.r32(p_outCount_1) as i32)) as u32)).wrapping_mul(36))).wrapping_add(4); (c.m.r32(a_) as i32) }) >= (2i32)) as i32)) != 0) {
                        if ((((l_flags_7) & (8i32))) != 0) {
                            { let v_ = 0i32; let a_ = ((p_out_2).wrapping_add(((((c.m.r32(p_outCount_1) as i32)) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                            { let v_ = { let a_ = (((p_out_2).wrapping_add(((((c.m.r32(p_outCount_1) as i32)) as u32)).wrapping_mul(36))).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (p_out_2).wrapping_add(((((c.m.r32(p_outCount_1) as i32)) as u32)).wrapping_mul(36)); c.m.w32(a_, (v_) as u32); };
                            { let a_ = p_outCount_1; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            break 'c2;
                        }
                        { let v_ = { let a_ = (((p_out_2).wrapping_add(((((c.m.r32(p_outCount_1) as i32)) as u32)).wrapping_mul(36))).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (p_out_2).wrapping_add(((((c.m.r32(p_outCount_1) as i32)) as u32)).wrapping_mul(36)); c.m.w32(a_, (v_) as u32); };
                    } else {
                        if ((((({ let a_ = ((p_out_2).wrapping_add(((((c.m.r32(p_outCount_1) as i32)) as u32)).wrapping_mul(36))).wrapping_add(4); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) {
                            { let v_ = 0i32; let a_ = ((p_out_2).wrapping_add(((((c.m.r32(p_outCount_1) as i32)) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                            { let v_ = { let a_ = (((p_out_2).wrapping_add(((((c.m.r32(p_outCount_1) as i32)) as u32)).wrapping_mul(36))).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (p_out_2).wrapping_add(((((c.m.r32(p_outCount_1) as i32)) as u32)).wrapping_mul(36)); c.m.w32(a_, (v_) as u32); };
                            { let a_ = p_outCount_1; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            break 'c2;
                        } else {
                            { let v_ = 162i32; let a_ = (p_out_2).wrapping_add(((((c.m.r32(p_outCount_1) as i32)) as u32)).wrapping_mul(36)); c.m.w32(a_, (v_) as u32); };
                        }
                    }
                } else {
                    's5: {
                        let sv12: i32 = (c.m.r32((p_in_4).wrapping_add((((l_i_9) as u32)).wrapping_mul(36))) as i32);
                        match sv12 {
                            163i32 => {
                                if ((((l_flags_7) & (2i32))) != 0) {
                                    { let v_ = 164i32; let a_ = (p_out_2).wrapping_add(((((c.m.r32(p_outCount_1) as i32)) as u32)).wrapping_mul(36)); c.m.w32(a_, (v_) as u32); };
                                } else {
                                    { let s_ = (p_in_4).wrapping_add((((l_i_9) as u32)).wrapping_mul(36)); let d_ = (p_out_2).wrapping_add(((((c.m.r32(p_outCount_1) as i32)) as u32)).wrapping_mul(36)); rt::memcpy(c, d_, s_, 36); };
                                }
                                break 's5;
                            }
                            161i32 => {
                                if ((((l_flags_7) & (4i32))) != 0) {
                                    { let v_ = 164i32; let a_ = (p_out_2).wrapping_add(((((c.m.r32(p_outCount_1) as i32)) as u32)).wrapping_mul(36)); c.m.w32(a_, (v_) as u32); };
                                } else {
                                    { let s_ = (p_in_4).wrapping_add((((l_i_9) as u32)).wrapping_mul(36)); let d_ = (p_out_2).wrapping_add(((((c.m.r32(p_outCount_1) as i32)) as u32)).wrapping_mul(36)); rt::memcpy(c, d_, s_, 36); };
                                }
                                break 's5;
                            }
                            _ => {
                                if ((((((((l_flags_7) & (1i32))) != 0) || ((((((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3088)).wrapping_add((((rt::div_i32((c.m.r32((p_in_4).wrapping_add((((l_i_9) as u32)).wrapping_mul(36))) as i32), 64i32)) as u32)).wrapping_mul(8)); c.m.r64(a_) }).wrapping_shr((rt::rem_i32((c.m.r32((p_in_4).wrapping_add((((l_i_9) as u32)).wrapping_mul(36))) as i32), 64i32)) as u32)) & (((1i32) as u64)))) as i32)) != 0)) as i32)) != 0) {
                                    { let s_ = (p_in_4).wrapping_add((((l_i_9) as u32)).wrapping_mul(36)); let d_ = (p_out_2).wrapping_add(((((c.m.r32(p_outCount_1) as i32)) as u32)).wrapping_mul(36)); rt::memcpy(c, d_, s_, 36); };
                                } else {
                                    { let v_ = 162i32; let a_ = (p_out_2).wrapping_add(((((c.m.r32(p_outCount_1) as i32)) as u32)).wrapping_mul(36)); c.m.w32(a_, (v_) as u32); };
                                }
                                break 's5;
                            }
                        }
                    }
                }
                { let a_ = p_outCount_1; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            { let o_ = l_i_9; let n_ = (o_).wrapping_add(1); l_i_9 = n_; };
        }
    }
    l_pad_11 = (rt::div_i32(((c.m.r32(p_outCount_1) as i32)).wrapping_add((7i32).wrapping_sub(1i32)), 7i32)).wrapping_mul(7i32);
    if (((((l_pad_11) != (0i32)) as i32)) != 0) {
        { let r_ = (c.m.r32(p_outCount_1) as i32); let n_ = (l_pad_11).wrapping_sub(r_); l_pad_11 = n_; };
        {
            l_i_9 = 0i32;
            'l6: loop {
                if !(((((l_i_9) < (l_pad_11)) as i32)) != 0) { break 'l6; }
                'c7: {
                    { let v_ = 164i32; let a_ = (p_out_2).wrapping_add(((((c.m.r32(p_outCount_1) as i32)) as u32)).wrapping_mul(36)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = ((p_out_2).wrapping_add(((((c.m.r32(p_outCount_1) as i32)) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let a_ = p_outCount_1; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                }
                { let o_ = l_i_9; let n_ = (o_).wrapping_add(1); l_i_9 = n_; };
            }
        }
    }
    if ((((((((!((((l_flags_7) & (2i32))) != 0)) as i32)) != 0) && (((((p_custom_6) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = p_customCount_5; c.m.w32(a_, (v_) as u32); };
        {
            l_i_9 = 0i32;
            'l8: loop {
                if !(((((l_i_9) < (14i32)) as i32)) != 0) { break 'l8; }
                'c9: {
                    { let v_ = 162i32; let a_ = (p_custom_6).wrapping_add((((l_i_9) as u32)).wrapping_mul(36)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = ((p_custom_6).wrapping_add((((l_i_9) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    if ((((({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(11584)).wrapping_add((((l_i_9) as u32)).wrapping_mul(28))).wrapping_add(24); (c.m.r32(a_) as i32) }) >= (0i32)) as i32)) != 0) {
                        {
                            l_j_10 = 0i32;
                            'l10: loop {
                                if !(((((l_j_10) < ((c.m.r32(p_inCount_3) as i32))) as i32)) != 0) { break 'l10; }
                                'c11: {
                                    if ((((({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(11584)).wrapping_add((((l_i_9) as u32)).wrapping_mul(28))).wrapping_add(24); (c.m.r32(a_) as i32) }) == ((c.m.r32((p_in_4).wrapping_add((((l_j_10) as u32)).wrapping_mul(36))) as i32))) as i32)) != 0) {
                                        { let s_ = (p_out_2).wrapping_add((((l_j_10) as u32)).wrapping_mul(36)); let d_ = (p_custom_6).wrapping_add((((l_i_9) as u32)).wrapping_mul(36)); rt::memcpy(c, d_, s_, 36); };
                                        { let v_ = { let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(11584)).wrapping_add((((l_i_9) as u32)).wrapping_mul(28))).wrapping_add(24); (c.m.r32(a_) as i32) }; let a_ = (p_custom_6).wrapping_add((((l_i_9) as u32)).wrapping_mul(36)); c.m.w32(a_, (v_) as u32); };
                                        { let v_ = { let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(11584)).wrapping_add((((l_i_9) as u32)).wrapping_mul(28))).wrapping_add(24); (c.m.r32(a_) as i32) }; let a_ = (((p_custom_6).wrapping_add((((l_i_9) as u32)).wrapping_mul(36))).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                        { let a_ = p_customCount_5; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                        break 'l10;
                                    }
                                }
                                { let o_ = l_j_10; let n_ = (o_).wrapping_add(1); l_j_10 = n_; };
                            }
                        }
                    }
                }
                { let o_ = l_i_9; let n_ = (o_).wrapping_add(1); l_i_9 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrGrid_Build", &PROBE_N, &[], true); }
    r_
}

pub fn StgGrid_ApplyUnlocks(c: &mut Ctx, mut p_count_1: u32, mut p_ids_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgGrid_ApplyUnlocks", &PROBE_N, &[(p_count_1) as u32, (p_ids_2) as u32], false); }
    let mut l_flags_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_id_5: i32 = 0;
    let r_ = 'f: {
    l_flags_3 = 0i32;
    if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); (c.m.r32(a_) as i32) }) == (40i32)) as i32)) != 0) {
        l_flags_3 = 4i32;
    }
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < ((c.m.r32(p_count_1) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                l_id_5 = (c.m.r32((p_ids_2).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))) as i32);
                's3: {
                    let sv6: i32 = l_id_5;
                    match sv6 {
                        35i32 => {
                            if ((((l_flags_3) & (2i32))) != 0) {
                                { let v_ = 63i32; let a_ = (p_ids_2).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            }
                            break 's3;
                        }
                        4i32 | 27i32 => {
                            if ((((!((((l_flags_3) & (1i32))) != 0)) as i32)) != 0) {
                                if ((((!((((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(3112); c.m.r64(a_) }).wrapping_shr((l_id_5) as u32)) & (((1i64) as u64)))) as i32)) != 0)) as i32)) != 0) {
                                    { let v_ = 62i32; let a_ = (p_ids_2).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                } else {
                                    if ((((l_flags_3) & (4i32))) != 0) {
                                        { let v_ = 63i32; let a_ = (p_ids_2).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                    }
                                }
                            }
                            break 's3;
                        }
                        _ => {
                            if ((((!((((l_flags_3) & (1i32))) != 0)) as i32)) != 0) {
                                if ((((!((((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(3112); c.m.r64(a_) }).wrapping_shr((l_id_5) as u32)) & (((1i64) as u64)))) as i32)) != 0)) as i32)) != 0) {
                                    { let v_ = 62i32; let a_ = (p_ids_2).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                }
                            }
                            break 's3;
                        }
                    }
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgGrid_ApplyUnlocks", &PROBE_N, &[], true); }
    r_
}

pub fn BgmList_ApplyUnlocks(c: &mut Ctx, mut p_count_1: u32, mut p_ids_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BgmList_ApplyUnlocks", &PROBE_N, &[(p_count_1) as u32, (p_ids_2) as u32], false); }
    let mut l_flags_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_flags_3 = 0i32;
    if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); (c.m.r32(a_) as i32) }) == (62i32)) as i32)) != 0) {
        l_flags_3 = 2i32;
    }
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < ((c.m.r32(p_count_1) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32(p_ids_2) as i32)) == (24i32)) as i32)) != 0) {
                    if ((((l_flags_3) & (2i32))) != 0) {
                        { let v_ = 25i32; let a_ = p_ids_2; c.m.w32(a_, (v_) as u32); };
                    }
                } else {
                    if ((((!((((l_flags_3) & (1i32))) != 0)) as i32)) != 0) {
                        if ((((!(((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(3120); c.m.r32(a_) }) & ((((1i32).wrapping_shl(((c.m.r32(p_ids_2) as i32)) as u32)) as u32)))) != 0)) as i32)) != 0) {
                            { let v_ = 25i32; let a_ = p_ids_2; c.m.w32(a_, (v_) as u32); };
                        }
                    }
                }
            }
            { { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; }; { let o_ = p_ids_2; let n_ = (o_).wrapping_add(4); p_ids_2 = n_; }; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BgmList_ApplyUnlocks", &PROBE_N, &[], true); }
    r_
}

pub fn TextBox_Init(c: &mut Ctx, mut p_box_1: u32, mut p_text_2: u32, mut p_preset_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TextBox_Init", &PROBE_N, &[(p_box_1) as u32, (p_text_2) as u32, (p_preset_3) as u32], false); }
    let r_ = 'f: {
    { let _ = F::memset(c, p_box_1, 0i32, 140u32); };
    { let v_ = p_text_2; let a_ = (p_box_1).wrapping_add(4); c.m.w32(a_, v_); };
    's1: {
        let sv4: u32 = p_preset_3;
        match sv4 {
            1u32 => {
                F::TextBox_SetUnk50(c, p_box_1, 2i32);
                F::TextBox_SetUnkC(c, p_box_1, 256i32, 0i32);
                F::TextBox_SetMaxWidth(c, p_box_1, 225i32);
                break 's1;
            }
            2u32 => {
                F::TextBox_SetUnk50(c, p_box_1, 0i32);
                F::TextBox_SetUnkC(c, p_box_1, 0i32, 0i32);
                F::TextBox_SetMaxWidth(c, p_box_1, 225i32);
                break 's1;
            }
            3u32 => {
                F::TextBox_SetUnk50(c, p_box_1, 2i32);
                F::TextBox_SetUnkC(c, p_box_1, 256i32, 0i32);
                F::TextBox_SetColor(c, p_box_1, 4294901888u32);
                F::TextBox_SetMaxWidth(c, p_box_1, 212i32);
                break 's1;
            }
            4u32 => {
                F::TextBox_SetUnk50(c, p_box_1, 0i32);
                F::TextBox_SetUnkC(c, p_box_1, 0i32, 0i32);
                F::TextBox_SetColor(c, p_box_1, 4294901888u32);
                F::TextBox_SetMaxWidth(c, p_box_1, 212i32);
                break 's1;
            }
            5u32 => {
                F::TextBox_SetUnk50(c, p_box_1, 0i32);
                F::TextBox_SetUnk80(c, p_box_1, 1i32);
                F::TextBox_SetUnkC(c, p_box_1, 0i32, 0i32);
                F::TextBox_SetLineOffsets(c, p_box_1, 32i32, 20i32, 10i32, 0i32, 0i32);
                break 's1;
            }
            6u32 => {
                F::TextBox_SetUnk50(c, p_box_1, 0i32);
                F::TextBox_SetMaxWidth(c, p_box_1, 352i32);
                break 's1;
            }
            0u32 => {
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "TextBox_Init", &PROBE_N, &[], true); }
    r_
}

pub fn TextBox_SetUnk50(c: &mut Ctx, mut p_box_1: u32, mut p_value_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TextBox_SetUnk50", &PROBE_N, &[(p_box_1) as u32, (p_value_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_value_2; let a_ = (p_box_1).wrapping_add(80); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "TextBox_SetUnk50", &PROBE_N, &[], true); }
    r_
}

pub fn TextBox_SetUnk80(c: &mut Ctx, mut p_box_1: u32, mut p_value_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TextBox_SetUnk80", &PROBE_N, &[(p_box_1) as u32, (p_value_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_value_2; let a_ = (p_box_1).wrapping_add(128); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "TextBox_SetUnk80", &PROBE_N, &[], true); }
    r_
}

pub fn TextBox_SetUnkC(c: &mut Ctx, mut p_box_1: u32, mut p_a_2: i32, mut p_b_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TextBox_SetUnkC", &PROBE_N, &[(p_box_1) as u32, (p_a_2) as u32, (p_b_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_a_2; let a_ = (p_box_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_b_3; let a_ = (p_box_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "TextBox_SetUnkC", &PROBE_N, &[], true); }
    r_
}

pub fn TextBox_SetRect(c: &mut Ctx, mut p_box_1: u32, mut p_a_2: i32, mut p_b_3: i32, mut p_c_4: i32, mut p_d_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TextBox_SetRect", &PROBE_N, &[(p_box_1) as u32, (p_a_2) as u32, (p_b_3) as u32, (p_c_4) as u32, (p_d_5) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_d_5; let a_ = ((p_box_1).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_a_2; let a_ = ((p_box_1).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_c_4; let a_ = ((p_box_1).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_b_3; let a_ = ((p_box_1).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let r_ = 1i32; let a_ = p_box_1; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "TextBox_SetRect", &PROBE_N, &[], true); }
    r_
}

pub fn TextBox_SetColor(c: &mut Ctx, mut p_box_1: u32, mut p_rgba_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TextBox_SetColor", &PROBE_N, &[(p_box_1) as u32, (p_rgba_2) as u32], false); }
    let r_ = 'f: {
    { let r_ = 2i32; let a_ = p_box_1; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    { let v_ = (((p_rgba_2).wrapping_shr((24i32) as u32)) as u8); let a_ = ((p_box_1).wrapping_add(48)).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = (((p_rgba_2).wrapping_shr((16i32) as u32)) as u8); let a_ = ((p_box_1).wrapping_add(48)).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = (((p_rgba_2).wrapping_shr((8i32) as u32)) as u8); let a_ = ((p_box_1).wrapping_add(48)).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((p_rgba_2) as u8); let a_ = ((p_box_1).wrapping_add(48)).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "TextBox_SetColor", &PROBE_N, &[], true); }
    r_
}

pub fn TextBox_SetColor2(c: &mut Ctx, mut p_box_1: u32, mut p_rgba_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TextBox_SetColor2", &PROBE_N, &[(p_box_1) as u32, (p_rgba_2) as u32], false); }
    let r_ = 'f: {
    { let r_ = 4i32; let a_ = p_box_1; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    { let v_ = (((p_rgba_2).wrapping_shr((24i32) as u32)) as u8); let a_ = ((p_box_1).wrapping_add(52)).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = (((p_rgba_2).wrapping_shr((16i32) as u32)) as u8); let a_ = ((p_box_1).wrapping_add(52)).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = (((p_rgba_2).wrapping_shr((8i32) as u32)) as u8); let a_ = ((p_box_1).wrapping_add(52)).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((p_rgba_2) as u8); let a_ = ((p_box_1).wrapping_add(52)).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "TextBox_SetColor2", &PROBE_N, &[], true); }
    r_
}
