#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EftLine_SetTex(c: &mut Ctx, mut p_w_1: u32, mut p_tex_2: u32, mut p_a_3: i32, mut p_b_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLine_SetTex", &PROBE_N, &[(p_w_1) as u32, (p_tex_2) as u32, (p_a_3) as u32, (p_b_4) as u32], false); }
    let r_ = 'f: {
    { let s_ = (p_tex_2).wrapping_add((((p_a_3) as u32)).wrapping_mul(16)); let d_ = (p_w_1).wrapping_add(280); rt::memcpy(c, d_, s_, 16); };
    { let s_ = (p_tex_2).wrapping_add((((p_b_4) as u32)).wrapping_mul(16)); let d_ = (p_w_1).wrapping_add(296); rt::memcpy(c, d_, s_, 16); };
    { let v_ = p_b_4; let a_ = (p_w_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftLine_SetTex", &PROBE_N, &[], true); }
    r_
}

pub fn EftLine_LoadTex(c: &mut Ctx, mut p_w_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLine_LoadTex", &PROBE_N, &[(p_w_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    if ((((!(((({ let a_ = (c.m.r32((p_arg_2).wrapping_add(48))).wrapping_add(260); c.m.r32(a_) }) & ((((1i32).wrapping_shl(((c.m.r32((p_arg_2).wrapping_add(36)) as i32)) as u32)) as u32)))) != 0)) as i32)) != 0) {
        { let v_ = F::EftVram_AddImage(c, (p_w_1).wrapping_add(280), 1i32, 0i32); let a_ = (p_w_1).wrapping_add(312); c.m.w64(a_, v_); };
        { let r_ = (F::EftVram_AddClut(c, (p_w_1).wrapping_add(296))).wrapping_shl((37i32) as u32); let a_ = (p_w_1).wrapping_add(312); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
        { let v_ = c.m.r64((p_w_1).wrapping_add(312)); let a_ = (c.m.r32((p_arg_2).wrapping_add(48))).wrapping_add(((((c.m.r32((p_arg_2).wrapping_add(36)) as i32)) as u32)).wrapping_mul(16)); c.m.w64(a_, v_); };
        { let r_ = (((1i32).wrapping_shl(((c.m.r32((p_arg_2).wrapping_add(36)) as i32)) as u32)) as u32); let a_ = (c.m.r32((p_arg_2).wrapping_add(48))).wrapping_add(260); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    } else {
        { let v_ = { let a_ = (c.m.r32((p_arg_2).wrapping_add(48))).wrapping_add(((((c.m.r32((p_arg_2).wrapping_add(36)) as i32)) as u32)).wrapping_mul(16)); c.m.r64(a_) }; let a_ = (p_w_1).wrapping_add(312); c.m.w64(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftLine_LoadTex", &PROBE_N, &[], true); }
    r_
}

pub fn EftLine_SetKey(c: &mut Ctx, mut p_out_1: u32, mut p_arg_2: u32, mut p_key_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLine_SetKey", &PROBE_N, &[(p_out_1) as u32, (p_arg_2) as u32, (p_key_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_def_4: u32 = 0;
    let mut l_def2_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let r_ = 'f: {
        l_def_4 = c.m.r32((p_arg_2).wrapping_add(52));
        l_def2_5 = c.m.r32((p_arg_2).wrapping_add(56));
        F::Vec4_Copy(c, p_out_1, (l_def2_5).wrapping_add((((p_key_3) as u32)).wrapping_mul(16)));
        {
            l_i_6 = 0i32;
            'l1: loop {
                if !(((((l_i_6) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = F32(c.m.r32((((l_def2_5).wrapping_add(48)).wrapping_add((((p_key_3) as u32)).wrapping_mul(8))).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)))); let a_ = (fp).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((((l_def2_5).wrapping_add(72)).wrapping_add((((p_key_3) as u32)).wrapping_mul(8))).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)))); let a_ = (fp.wrapping_add(16)).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((((l_def2_5).wrapping_add(96)).wrapping_add((((p_key_3) as u32)).wrapping_mul(8))).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)))); let a_ = (fp.wrapping_add(32)).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
        { let v_ = F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add(16); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (p_out_1).wrapping_add(28); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add(20); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add(24); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (p_out_1).wrapping_add(32); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (p_out_1).wrapping_add(36); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_def2_5).wrapping_add(120)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x41f00000)); let a_ = (p_out_1).wrapping_add(40); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_def_4).wrapping_add(28)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x40490fda)); let a_ = (p_out_1).wrapping_add(44); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_def_4).wrapping_add(40)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add(48); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_def_4).wrapping_add(52)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add(52); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_def_4).wrapping_add(64)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_def_4).wrapping_add(76)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add(56); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (p_out_1).wrapping_add(60); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_def_4).wrapping_add(88)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x41f00000)); let a_ = (p_out_1).wrapping_add(64); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_def_4).wrapping_add(100)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_def_4).wrapping_add(112)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add(68); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (p_out_1).wrapping_add(72); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_def_4).wrapping_add(124)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x41f00000)); let a_ = (p_out_1).wrapping_add(76); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftLine_SetKey", &PROBE_N, &[], true); }
    r_
}

pub fn EftLine_Animate(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLine_Animate", &PROBE_N, &[(p_w_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_out_2: u32 = 0;
    let mut l_def_3: u32 = 0;
    let mut l_def2_4: u32 = 0;
    let mut l_t_5: F32 = F32(0);
    let mut l_k0_6: i32 = 0;
    let mut l_k1_7: i32 = 0;
    let mut l_i_8: i32 = 0;
    let r_ = 'f: {
        l_out_2 = (p_w_1).wrapping_add(64);
        l_def_3 = c.m.r32((p_w_1).wrapping_add(52));
        l_def2_4 = c.m.r32((p_w_1).wrapping_add(56));
        if (((rt::flt(F32(c.m.r32((p_w_1).wrapping_add(220))), F32(c.m.r32((p_w_1).wrapping_add(228)))) as i32)) != 0) {
            l_t_5 = rt::fdiv(F32(c.m.r32((p_w_1).wrapping_add(220))), F32(c.m.r32((p_w_1).wrapping_add(228))));
            l_k0_6 = 0i32;
            l_k1_7 = 1i32;
        } else {
            l_t_5 = rt::fsub(F32(c.m.r32((p_w_1).wrapping_add(220))), F32(c.m.r32((p_w_1).wrapping_add(228))));
            l_k0_6 = 1i32;
            l_k1_7 = 2i32;
            { let r_ = rt::fsub(F32(c.m.r32((p_w_1).wrapping_add(224))), F32(c.m.r32((p_w_1).wrapping_add(228)))); let n_ = rt::fdiv(l_t_5, r_); l_t_5 = n_; };
        }
        F::Vec4_Sub(c, fp, (l_def2_4).wrapping_add((((l_k1_7) as u32)).wrapping_mul(16)), (l_def2_4).wrapping_add((((l_k0_6) as u32)).wrapping_mul(16)));
        { let v_ = rt::fadd(F32(c.m.r32(((l_def2_4).wrapping_add((((l_k0_6) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (l_out_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def2_4).wrapping_add((((l_k0_6) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (l_out_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def2_4).wrapping_add((((l_k0_6) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (l_out_2).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def2_4).wrapping_add((((l_k0_6) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((3i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (l_out_2).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        {
            l_i_8 = 0i32;
            'l1: loop {
                if !(((((l_i_8) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = rt::fsub(F32(c.m.r32((((l_def2_4).wrapping_add(48)).wrapping_add((((l_k1_7) as u32)).wrapping_mul(8))).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def2_4).wrapping_add(48)).wrapping_add((((l_k0_6) as u32)).wrapping_mul(8))).wrapping_add((((l_i_8) as u32)).wrapping_mul(4))))); let a_ = fp; c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(F32(c.m.r32((((l_def2_4).wrapping_add(72)).wrapping_add((((l_k1_7) as u32)).wrapping_mul(8))).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def2_4).wrapping_add(72)).wrapping_add((((l_k0_6) as u32)).wrapping_mul(8))).wrapping_add((((l_i_8) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(F32(c.m.r32((((l_def2_4).wrapping_add(96)).wrapping_add((((l_k1_7) as u32)).wrapping_mul(8))).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def2_4).wrapping_add(96)).wrapping_add((((l_k0_6) as u32)).wrapping_mul(8))).wrapping_add((((l_i_8) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32((((l_def2_4).wrapping_add(48)).wrapping_add((((l_k0_6) as u32)).wrapping_mul(8))).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(fp)), l_t_5)); let a_ = (fp.wrapping_add(16)).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32((((l_def2_4).wrapping_add(72)).wrapping_add((((l_k0_6) as u32)).wrapping_mul(8))).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add(4))), l_t_5)); let a_ = (fp.wrapping_add(32)).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32((((l_def2_4).wrapping_add(96)).wrapping_add((((l_k0_6) as u32)).wrapping_mul(8))).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add(8))), l_t_5)); let a_ = (fp.wrapping_add(48)).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
            }
        }
        { let v_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (l_out_2).wrapping_add(16); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (l_out_2).wrapping_add(28); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (l_out_2).wrapping_add(20); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (l_out_2).wrapping_add(24); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (l_out_2).wrapping_add(32); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (l_out_2).wrapping_add(36); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def2_4).wrapping_add(120)).wrapping_add((((l_k1_7) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def2_4).wrapping_add(120)).wrapping_add((((l_k0_6) as u32)).wrapping_mul(4))))); let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_def2_4).wrapping_add(120)).wrapping_add((((l_k0_6) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(fp)), l_t_5)), F32(0x41f00000)); let a_ = (l_out_2).wrapping_add(40); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(28)).wrapping_add((((l_k1_7) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(28)).wrapping_add((((l_k0_6) as u32)).wrapping_mul(4))))); let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(28)).wrapping_add((((l_k0_6) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(fp)), l_t_5)), F32(0x40490fda)); let a_ = (l_out_2).wrapping_add(44); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(40)).wrapping_add((((l_k1_7) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(40)).wrapping_add((((l_k0_6) as u32)).wrapping_mul(4))))); let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(52)).wrapping_add((((l_k1_7) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(52)).wrapping_add((((l_k0_6) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(40)).wrapping_add((((l_k0_6) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(fp)), l_t_5)); let a_ = (l_out_2).wrapping_add(48); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(52)).wrapping_add((((l_k0_6) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add(4))), l_t_5)); let a_ = (l_out_2).wrapping_add(52); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(64)).wrapping_add((((l_k1_7) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(64)).wrapping_add((((l_k0_6) as u32)).wrapping_mul(4))))); let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(76)).wrapping_add((((l_k1_7) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(76)).wrapping_add((((l_k0_6) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(88)).wrapping_add((((l_k1_7) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(88)).wrapping_add((((l_k0_6) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(64)).wrapping_add((((l_k0_6) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(fp)), l_t_5)); let a_ = (fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(76)).wrapping_add((((l_k0_6) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add(4))), l_t_5)); let a_ = (fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (l_out_2).wrapping_add(56); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (l_out_2).wrapping_add(60); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(88)).wrapping_add((((l_k0_6) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add(8))), l_t_5)), F32(0x41f00000)); let a_ = (l_out_2).wrapping_add(64); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(100)).wrapping_add((((l_k1_7) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(100)).wrapping_add((((l_k0_6) as u32)).wrapping_mul(4))))); let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(112)).wrapping_add((((l_k1_7) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(112)).wrapping_add((((l_k0_6) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(124)).wrapping_add((((l_k1_7) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(124)).wrapping_add((((l_k0_6) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(100)).wrapping_add((((l_k0_6) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(fp)), l_t_5)); let a_ = (fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(112)).wrapping_add((((l_k0_6) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add(4))), l_t_5)); let a_ = (fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (l_out_2).wrapping_add(68); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (l_out_2).wrapping_add(72); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(124)).wrapping_add((((l_k0_6) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add(8))), l_t_5)), F32(0x41f00000)); let a_ = (l_out_2).wrapping_add(76); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftLine_Animate", &PROBE_N, &[], true); }
    r_
}

pub fn EftLine_DrawSprite(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLine_DrawSprite", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(336) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 336);
    let mut l_def_2: u32 = 0;
    let mut l_w_3: u32 = 0;
    let mut l_pk_4: u32 = 0;
    let mut l_e_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_z_7: i32 = 0;
    let mut l_layer_8: i32 = 0;
    let mut l_rhw_9: F32 = F32(0);
    let r_ = 'f: {
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        l_def_2 = c.m.r32((l_w_3).wrapping_add(52));
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(544); F::Vec4_Copy(c, fp.wrapping_add(192), a1_) };
        F::Vec3_Sub(c, fp.wrapping_add(208), l_w_3, fp.wrapping_add(192));
        F::Vec3_Cross(c, fp.wrapping_add(208), (l_w_3).wrapping_add(16), fp.wrapping_add(208));
        F::Vec3_Normalize(c, fp.wrapping_add(208), fp.wrapping_add(208));
        { let a2_ = F32(c.m.r32((l_w_3).wrapping_add(240))); F::Vec3_Scale(c, fp.wrapping_add(224), fp.wrapping_add(208), a2_) };
        F::Vec3_Add(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), l_w_3, fp.wrapping_add(224));
        F::Vec3_Sub(c, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), l_w_3, fp.wrapping_add(224));
        { let a2_ = F32(c.m.r32((l_w_3).wrapping_add(244))); F::Vec3_Scale(c, fp.wrapping_add(240), (l_w_3).wrapping_add(16), a2_) };
        F::Vec3_Add(c, (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), fp.wrapping_add(240));
        F::Vec3_Add(c, (fp).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), fp.wrapping_add(240));
        if ((F::Vu0Cur_ProjectPoints(c, fp.wrapping_add(256), fp, 4i32)) != 0) {
            l_pk_4 = c.m.r32(sym::gOtCur);
            { let v_ = (l_pk_4).wrapping_add((((1i32) as u32)).wrapping_mul(144)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
            if (((((l_pk_4) != (((0i32) as u32))) as i32)) != 0) {
                if (((((((c.m.r8((l_def_2).wrapping_add(4))) as i32)) < (2i32)) as i32)) != 0) {
                    { let v_ = ((92i32) as u64); let a_ = (l_pk_4).wrapping_add(32); c.m.w64(a_, v_); };
                    { let v_ = ((536870920i32) as u32); let a_ = l_pk_4; c.m.w32(a_, v_); };
                    { let v_ = ((268435456i32) as u32); let a_ = (l_pk_4).wrapping_add(8); c.m.w32(a_, v_); };
                    { let v_ = ((1342177288i32) as u32); let a_ = (l_pk_4).wrapping_add(12); c.m.w32(a_, v_); };
                    { let v_ = 16429131440647602177u64; let a_ = (l_pk_4).wrapping_add(16); c.m.w64(a_, v_); };
                    { let v_ = ((18599481537995104i64) as u64); let a_ = (l_pk_4).wrapping_add(24); c.m.w64(a_, v_); };
                    { let v_ = ((0i32) as u32); let a_ = (l_pk_4).wrapping_add(4); c.m.w32(a_, v_); };
                } else {
                    { let v_ = ((604i32) as u64); let a_ = (l_pk_4).wrapping_add(32); c.m.w64(a_, v_); };
                    { let v_ = ((536870920i32) as u32); let a_ = l_pk_4; c.m.w32(a_, v_); };
                    { let v_ = ((268435456i32) as u32); let a_ = (l_pk_4).wrapping_add(8); c.m.w32(a_, v_); };
                    { let v_ = ((1342177288i32) as u32); let a_ = (l_pk_4).wrapping_add(12); c.m.w32(a_, v_); };
                    { let v_ = 16429131440647602177u64; let a_ = (l_pk_4).wrapping_add(16); c.m.w64(a_, v_); };
                    { let v_ = ((18599481537995120i64) as u64); let a_ = (l_pk_4).wrapping_add(24); c.m.w64(a_, v_); };
                    { let v_ = ((0i32) as u32); let a_ = (l_pk_4).wrapping_add(4); c.m.w32(a_, v_); };
                }
                {
                    l_i_6 = 0i32;
                    'l1: loop {
                        if !(((((l_i_6) < (4i32)) as i32)) != 0) { break 'l1; }
                        'c2: {
                            l_rhw_9 = rt::fdiv(F32(0x3f800000), rt::i2f((c.m.r32(((fp.wrapping_add(256)).wrapping_add((((l_i_6) as u32)).wrapping_mul(16))).wrapping_add(12)) as i32)));
                            { let v_ = rt::fadd(rt::fmul(rt::i2f(rt::rem_i32(l_i_6, 2i32)), F32(c.m.r32((l_w_3).wrapping_add(188)))), F32(c.m.r32((l_w_3).wrapping_add(160)))); let a_ = (fp.wrapping_add(64)).wrapping_add((((l_i_6) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                            { let v_ = rt::fadd(rt::fmul(rt::i2f(rt::div_i32(l_i_6, 2i32)), F32(c.m.r32((l_w_3).wrapping_add(192)))), F32(c.m.r32((l_w_3).wrapping_add(168)))); let a_ = ((fp.wrapping_add(64)).wrapping_add((((l_i_6) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(64)).wrapping_add((((l_i_6) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(0x00000000); let a_ = ((fp.wrapping_add(64)).wrapping_add((((l_i_6) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                            F::Vec3_Scale(c, (fp.wrapping_add(128)).wrapping_add((((l_i_6) as u32)).wrapping_mul(16)), (fp.wrapping_add(64)).wrapping_add((((l_i_6) as u32)).wrapping_mul(16)), l_rhw_9);
                        }
                        { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                    }
                }
                if (((((c.m.r32((l_w_3).wrapping_add(176)) as i32)) & (32i32))) != 0) {
                    {
                        l_i_6 = 3i32;
                        'l3: loop {
                            if !(((((l_i_6) >= (0i32)) as i32)) != 0) { break 'l3; }
                            'c4: {
                                { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(256)).wrapping_add((((l_i_6) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            }
                            { let o_ = l_i_6; let n_ = (o_).wrapping_sub(1); l_i_6 = n_; };
                        }
                    }
                }
                l_z_7 = (((((c.m.r32(((fp.wrapping_add(256)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)).wrapping_add((c.m.r32(((fp.wrapping_add(256)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add((c.m.r32(((fp.wrapping_add(256)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add((c.m.r32(((fp.wrapping_add(256)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_shr((10i32) as u32);
                F::Vec4_ToInt(c, fp.wrapping_add(320), (l_w_3).wrapping_add(144));
                { let v_ = (((c.m.r32(fp.wrapping_add(320)) as i32)) as u8); let a_ = ((l_pk_4).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
                { let v_ = (((c.m.r32((fp.wrapping_add(320)).wrapping_add(4)) as i32)) as u8); let a_ = (((l_pk_4).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
                { let v_ = (((c.m.r32((fp.wrapping_add(320)).wrapping_add(8)) as i32)) as u8); let a_ = (((l_pk_4).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
                { let v_ = (((c.m.r32((fp.wrapping_add(320)).wrapping_add(12)) as i32)) as u8); let a_ = (((l_pk_4).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
                { let v_ = F32(c.m.r32(((fp.wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_pk_4).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = (((c.m.r32(fp.wrapping_add(320)) as i32)) as u8); let a_ = ((l_pk_4).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
                { let v_ = (((c.m.r32((fp.wrapping_add(320)).wrapping_add(4)) as i32)) as u8); let a_ = (((l_pk_4).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
                { let v_ = (((c.m.r32((fp.wrapping_add(320)).wrapping_add(8)) as i32)) as u8); let a_ = (((l_pk_4).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
                { let v_ = (((c.m.r32((fp.wrapping_add(320)).wrapping_add(12)) as i32)) as u8); let a_ = (((l_pk_4).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
                { let v_ = F32(c.m.r32(((fp.wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_pk_4).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = (((c.m.r32(fp.wrapping_add(320)) as i32)) as u8); let a_ = ((l_pk_4).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
                { let v_ = (((c.m.r32((fp.wrapping_add(320)).wrapping_add(4)) as i32)) as u8); let a_ = (((l_pk_4).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
                { let v_ = (((c.m.r32((fp.wrapping_add(320)).wrapping_add(8)) as i32)) as u8); let a_ = (((l_pk_4).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
                { let v_ = (((c.m.r32((fp.wrapping_add(320)).wrapping_add(12)) as i32)) as u8); let a_ = (((l_pk_4).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
                { let v_ = F32(c.m.r32(((fp.wrapping_add(128)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_pk_4).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = (((c.m.r32(fp.wrapping_add(320)) as i32)) as u8); let a_ = ((l_pk_4).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
                { let v_ = (((c.m.r32((fp.wrapping_add(320)).wrapping_add(4)) as i32)) as u8); let a_ = (((l_pk_4).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
                { let v_ = (((c.m.r32((fp.wrapping_add(320)).wrapping_add(8)) as i32)) as u8); let a_ = (((l_pk_4).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
                { let v_ = (((c.m.r32((fp.wrapping_add(320)).wrapping_add(12)) as i32)) as u8); let a_ = (((l_pk_4).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
                { let v_ = F32(c.m.r32(((fp.wrapping_add(128)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_pk_4).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32((fp.wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(16)))); let a_ = (((l_pk_4).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((fp.wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = ((((l_pk_4).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32((fp.wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(16)))); let a_ = (((l_pk_4).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((fp.wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = ((((l_pk_4).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32((fp.wrapping_add(128)).wrapping_add((((2i32) as u32)).wrapping_mul(16)))); let a_ = (((l_pk_4).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((fp.wrapping_add(128)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = ((((l_pk_4).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32((fp.wrapping_add(128)).wrapping_add((((3i32) as u32)).wrapping_mul(16)))); let a_ = (((l_pk_4).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((fp.wrapping_add(128)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = ((((l_pk_4).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = (((c.m.r32((fp.wrapping_add(256)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_pk_4).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                { let v_ = (((c.m.r32(((fp.wrapping_add(256)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_pk_4).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                { let v_ = (((c.m.r32(((fp.wrapping_add(256)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_pk_4).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                { let v_ = ((255i32) as u64); let a_ = ((((l_pk_4).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                { let v_ = (((c.m.r32((fp.wrapping_add(256)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_pk_4).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                { let v_ = (((c.m.r32(((fp.wrapping_add(256)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_pk_4).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                { let v_ = (((c.m.r32(((fp.wrapping_add(256)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_pk_4).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                { let v_ = ((255i32) as u64); let a_ = ((((l_pk_4).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                { let v_ = (((c.m.r32((fp.wrapping_add(256)).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_pk_4).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                { let v_ = (((c.m.r32(((fp.wrapping_add(256)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_pk_4).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                { let v_ = (((c.m.r32(((fp.wrapping_add(256)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_pk_4).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                { let v_ = ((255i32) as u64); let a_ = ((((l_pk_4).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                { let v_ = (((c.m.r32((fp.wrapping_add(256)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_pk_4).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                { let v_ = (((c.m.r32(((fp.wrapping_add(256)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_pk_4).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                { let v_ = (((c.m.r32(((fp.wrapping_add(256)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_pk_4).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                { let v_ = ((255i32) as u64); let a_ = ((((l_pk_4).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                { let v_ = c.m.r64((l_w_3).wrapping_add(312)); let a_ = (l_pk_4).wrapping_add(40); c.m.w64(a_, v_); };
                l_layer_8 = ((c.m.r8((l_def_2).wrapping_add(4))) as i32);
                if (((((l_layer_8) >= (2i32)) as i32)) != 0) {
                    { let r_ = 2i32; let n_ = (l_layer_8).wrapping_sub(r_); l_layer_8 = n_; };
                }
                if (((((l_z_7) < (0i32)) as i32)) != 0) {
                    l_e_5 = ((c.m.r32(sym::gOtZ)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((l_layer_8) as u32)).wrapping_mul(8));
                } else {
                    if (((((l_z_7) >= (4096i32)) as i32)) != 0) {
                        l_e_5 = ((c.m.r32(sym::gOtZ)).wrapping_add((((4095i32) as u32)).wrapping_mul(16))).wrapping_add((((l_layer_8) as u32)).wrapping_mul(8));
                    } else {
                        l_e_5 = ((c.m.r32(sym::gOtZ)).wrapping_add((((l_z_7) as u32)).wrapping_mul(16))).wrapping_add((((l_layer_8) as u32)).wrapping_mul(8));
                    }
                }
                { let v_ = l_pk_4; let a_ = (c.m.r32((l_e_5).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
                { let v_ = l_pk_4; let a_ = (l_e_5).wrapping_add(4); c.m.w32(a_, v_); };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftLine_DrawSprite", &PROBE_N, &[], true); }
    r_
}

pub fn EftLine_DrawClipped(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLine_DrawClipped", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(688) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 688);
    let mut l_def_2: u32 = 0;
    let mut l_w_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_z_5: i32 = 0;
    let r_ = 'f: {
        l_i_4 = 0i32;
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        l_def_2 = c.m.r32((l_w_3).wrapping_add(52));
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(544); F::Vec4_Copy(c, fp.wrapping_add(128), a1_) };
        F::Vec3_Sub(c, fp.wrapping_add(144), l_w_3, fp.wrapping_add(128));
        F::Vec3_Cross(c, fp.wrapping_add(144), (l_w_3).wrapping_add(16), fp.wrapping_add(144));
        F::Vec3_Normalize(c, fp.wrapping_add(144), fp.wrapping_add(144));
        { let a2_ = F32(c.m.r32((l_w_3).wrapping_add(240))); F::Vec3_Scale(c, fp.wrapping_add(160), fp.wrapping_add(144), a2_) };
        F::Vec3_Add(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), l_w_3, fp.wrapping_add(160));
        F::Vec3_Sub(c, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), l_w_3, fp.wrapping_add(160));
        { let a2_ = F32(c.m.r32((l_w_3).wrapping_add(244))); F::Vec3_Scale(c, fp.wrapping_add(176), (l_w_3).wrapping_add(16), a2_) };
        F::Vec3_Add(c, (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), fp.wrapping_add(176));
        F::Vec3_Add(c, (fp).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), fp.wrapping_add(176));
        {
            'l1: loop {
                if !(((((l_i_4) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = rt::fadd(rt::fmul(rt::i2f(rt::rem_i32(l_i_4, 2i32)), F32(c.m.r32((l_w_3).wrapping_add(188)))), F32(c.m.r32((l_w_3).wrapping_add(160)))); let a_ = (fp.wrapping_add(64)).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(rt::fmul(rt::i2f(rt::div_i32(l_i_4, 2i32)), F32(c.m.r32((l_w_3).wrapping_add(192)))), F32(c.m.r32((l_w_3).wrapping_add(168)))); let a_ = ((fp.wrapping_add(64)).wrapping_add((((l_i_4) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(64)).wrapping_add((((l_i_4) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x00000000); let a_ = ((fp.wrapping_add(64)).wrapping_add((((l_i_4) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        { let _ = F::Vu0Cur_ProjectPoints(c, fp.wrapping_add(192), fp, 4i32); };
        l_z_5 = (((((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)).wrapping_add((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_shr((10i32) as u32);
        {
            l_i_4 = 0i32;
            'l3: loop {
                if !(((((l_i_4) < (2i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    F::ClipVtx_Set(c, (fp.wrapping_add(256)).wrapping_add((((0i32) as u32)).wrapping_mul(48)), (fp).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)), (fp.wrapping_add(64)).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)), (l_w_3).wrapping_add(144));
                    F::ClipVtx_Set(c, (fp.wrapping_add(256)).wrapping_add((((1i32) as u32)).wrapping_mul(48)), (fp).wrapping_add(((((l_i_4).wrapping_add(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(64)).wrapping_add(((((l_i_4).wrapping_add(1i32)) as u32)).wrapping_mul(16)), (l_w_3).wrapping_add(144));
                    F::ClipVtx_Set(c, (fp.wrapping_add(256)).wrapping_add((((2i32) as u32)).wrapping_mul(48)), (fp).wrapping_add(((((l_i_4).wrapping_add(2i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(64)).wrapping_add(((((l_i_4).wrapping_add(2i32)) as u32)).wrapping_mul(16)), (l_w_3).wrapping_add(144));
                    { let a1_ = ((c.m.r8((l_def_2).wrapping_add(4))) as i32); let a4_ = ((((c.m.r32((l_w_3).wrapping_add(176)) as i32)).wrapping_shr((5i32) as u32)) & (1i32)); let a6_ = c.m.r64((l_w_3).wrapping_add(312)); F::EftGfx_DrawPolyFixedZ(c, fp.wrapping_add(256), a1_, 0i32, 0i32, a4_, 0i32, a6_, l_z_5) };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftLine_DrawClipped", &PROBE_N, &[], true); }
    r_
}

pub fn eft_z__EftGndDust_FreeParts(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_z__EftGndDust_FreeParts", &PROBE_N, &[(p_w_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
    l_p_2 = F::List_GetHead(c, (p_w_1).wrapping_add(204));
    'l1: while (((((l_p_2) != (((0i32) as u32))) as i32)) != 0) {
        { let r_ = (!(2i32)); let a_ = (l_p_2).wrapping_add(188); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        l_p_2 = { let a1_ = c.m.r32((p_w_1).wrapping_add(200)); F::EftGndDust_MoveNode(c, (p_w_1).wrapping_add(204), a1_, l_p_2) };
        { let a_ = (c.m.r32(sym::gEftGndDust)).wrapping_add(25268); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "eft_z__EftGndDust_FreeParts", &PROBE_N, &[], true); }
    r_
}

pub fn EftBill_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBill_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let mut l_def_4: u32 = 0;
    let mut l_key_5: u32 = 0;
    let mut l_a_6: F32 = F32(0);
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    l_def_4 = c.m.r32((p_arg_2).wrapping_add(52));
    l_key_5 = (l_w_3).wrapping_add(64);
    { let _ = F::memset(c, l_w_3, 0i32, 320u32); };
    { let s_ = p_arg_2; let d_ = l_w_3; rt::memcpy(c, d_, s_, 64); };
    { let r_ = 1i32; let a_ = (l_w_3).wrapping_add(176); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    if (((rt::fle(F32(c.m.r32((p_arg_2).wrapping_add(40))), F32(0x00000000)) as i32)) != 0) {
        { let r_ = 16i32; let a_ = (l_w_3).wrapping_add(176); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        { let v_ = rt::fmul(F32(c.m.r32((p_arg_2).wrapping_add(40))), F32(0x41f00000)); let a_ = (l_w_3).wrapping_add(204); c.m.w32(a_, (v_).0); };
    }
    if (((((c.m.r32(l_def_4) as i32)) & (1i32))) != 0) {
        { let v_ = rt::fmul(F32(c.m.r32((l_def_4).wrapping_add(20))), F32(0x41f00000)); let a_ = (l_w_3).wrapping_add(224); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32((l_w_3).wrapping_add(224))), F32(c.m.r32((l_def_4).wrapping_add(24)))); let a_ = (l_w_3).wrapping_add(228); c.m.w32(a_, (v_).0); };
        { let _ = D::call(c, fid::EftLine_SetKey, &[((l_key_5) as u64), ((l_w_3) as u64), ((0i32) as i64 as u64)]); };
    } else {
        { let _ = D::call(c, fid::EftLine_SetKey, &[((l_key_5) as u64), ((l_w_3) as u64), ((2i32) as i64 as u64)]); };
    }
    { let v_ = rt::fmul(rt::fmul(F32(c.m.r32((l_def_4).wrapping_add(140))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff))), F32(0x40490fda)); let a_ = (l_w_3).wrapping_add(232); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(rt::fmul(F32(c.m.r32((l_def_4).wrapping_add(136))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff))), F32(0x40490fda)); let a_ = (l_w_3).wrapping_add(236); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_key_5).wrapping_add(48))); let a_ = (l_w_3).wrapping_add(240); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_key_5).wrapping_add(52))); let a_ = (l_w_3).wrapping_add(244); c.m.w32(a_, (v_).0); };
    { let _ = D::call(c, fid::Vec3_Copy, &[(((l_w_3).wrapping_add(144)) as u64), ((l_key_5) as u64)]); };
    { let v_ = F32(0x00000000); let a_ = ((l_w_3).wrapping_add(144)).wrapping_add(12); c.m.w32(a_, (v_).0); };
    l_a_6 = F32(c.m.r32(((l_w_3).wrapping_add(144)).wrapping_add(12)));
    { let v_ = rt::fmul(F32(c.m.r32((l_def_4).wrapping_add(12))), F32(0x41f00000)); let a_ = (l_w_3).wrapping_add(268); c.m.w32(a_, (v_).0); };
    if (((rt::fle(F32(c.m.r32((l_w_3).wrapping_add(268))), l_a_6) as i32)) != 0) {
        { let r_ = 64i32; let a_ = (l_w_3).wrapping_add(176); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    { let v_ = rt::fmul(F32(c.m.r32((l_def_4).wrapping_add(16))), F32(0x41f00000)); let a_ = (l_w_3).wrapping_add(272); c.m.w32(a_, (v_).0); };
    if (((rt::fle(F32(c.m.r32((l_w_3).wrapping_add(272))), l_a_6) as i32)) != 0) {
        { let r_ = 128i32; let a_ = (l_w_3).wrapping_add(176); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        { let v_ = F32(c.m.r32((l_w_3).wrapping_add(272))); let a_ = (l_w_3).wrapping_add(264); c.m.w32(a_, (v_).0); };
    }
    if (((((c.m.r32(l_def_4) as i32)) & (8i32))) != 0) {
        { let v_ = (((c.m.r8((l_def_4).wrapping_add(6))) as i32)).wrapping_mul(((c.m.r8((l_def_4).wrapping_add(7))) as i32)); let a_ = (l_w_3).wrapping_add(184); c.m.w32(a_, (v_) as u32); };
        { let v_ = rt::fmul(F32(c.m.r32((l_def_4).wrapping_add(8))), F32(0x41f00000)); let a_ = (l_w_3).wrapping_add(200); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(F32(0x3f800000), rt::i2f(((c.m.r8((l_def_4).wrapping_add(6))) as i32))); let a_ = (l_w_3).wrapping_add(188); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(F32(0x3f800000), rt::i2f(((c.m.r8((l_def_4).wrapping_add(7))) as i32))); let a_ = (l_w_3).wrapping_add(192); c.m.w32(a_, (v_).0); };
    } else {
        { let v_ = F32(0x00000000); let a_ = (l_w_3).wrapping_add(160); c.m.w32(a_, (v_).0); };
        { let v_ = { let v_ = { let v_ = { let v_ = F32(0x3f800000); let a_ = (l_w_3).wrapping_add(188); c.m.w32(a_, (v_).0); v_ }; let a_ = (l_w_3).wrapping_add(192); c.m.w32(a_, (v_).0); v_ }; let a_ = (l_w_3).wrapping_add(164); c.m.w32(a_, (v_).0); v_ }; let a_ = (l_w_3).wrapping_add(172); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (l_w_3).wrapping_add(168); c.m.w32(a_, (v_).0); };
    }
    { let a1_ = c.m.r32((p_arg_2).wrapping_add(48)); let a2_ = (c.m.r32((p_arg_2).wrapping_add(36)) as i32); let a3_ = (c.m.r32((p_arg_2).wrapping_add(36)) as i32); { let _ = D::call(c, fid::EftLine_SetTex, &[((l_w_3) as u64), ((a1_) as u64), ((a2_) as i64 as u64), ((a3_) as i64 as u64)]); } };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBill_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftBill_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBill_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    { let v_ = 0i32; let a_ = (l_w_2).wrapping_add(176); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBill_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftBill_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBill_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_def_3: u32 = 0;
    let mut l_key_4: u32 = 0;
    let mut l_r_5: F32 = F32(0);
    let mut l_k_6: F32 = F32(0);
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_def_3 = c.m.r32((l_w_2).wrapping_add(52));
    if ((((!(({ let a0_ = (c.m.r32((l_w_2).wrapping_add(32)) as i32); let a1_ = ((c.m.r8((l_w_2).wrapping_add(180))) as i32); F::BtlScene_IsEffectStopped(c, a0_, a1_) }) != 0)) as i32)) != 0) {
        if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(208))), F32(0x00000000)) as i32)) != 0) {
            l_key_4 = (l_w_2).wrapping_add(64);
            { let r_ = rt::fadd(F32(c.m.r32((l_key_4).wrapping_add(44))), F32(c.m.r32((l_w_2).wrapping_add(236)))); let a_ = (l_w_2).wrapping_add(232); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let v_ = { let a0_ = F32(c.m.r32((l_w_2).wrapping_add(232))); F::EftMath_WrapAngle(c, a0_) }; let a_ = (l_w_2).wrapping_add(232); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32((l_key_4).wrapping_add(48))); let a_ = (l_w_2).wrapping_add(240); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32((l_key_4).wrapping_add(52))); let a_ = (l_w_2).wrapping_add(244); c.m.w32(a_, (v_).0); };
            if (((((c.m.r32(l_def_3) as i32)) & (4i32))) != 0) {
                if (((rt::fgt(F32(c.m.r32((l_key_4).wrapping_add(64))), F32(0x00000000)) as i32)) != 0) {
                    l_r_5 = rt::fdiv(F32(c.m.r32((l_w_2).wrapping_add(248))), F32(c.m.r32((l_key_4).wrapping_add(64))));
                    l_k_6 = rt::fadd(F32(c.m.r32((l_key_4).wrapping_add(56))), rt::fmul(F32(c.m.r32((l_key_4).wrapping_add(60))), l_r_5));
                    { let r_ = l_k_6; let a_ = (l_w_2).wrapping_add(240); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if ((((!(((((c.m.r32((l_w_2).wrapping_add(176)) as i32)) & (256i32))) != 0)) as i32)) != 0) {
                        { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(248); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        if (((rt::fle(F32(c.m.r32((l_key_4).wrapping_add(64))), F32(c.m.r32((l_w_2).wrapping_add(248)))) as i32)) != 0) {
                            { let v_ = F32(c.m.r32((l_key_4).wrapping_add(64))); let a_ = (l_w_2).wrapping_add(248); c.m.w32(a_, (v_).0); };
                            { let r_ = 256i32; let a_ = (l_w_2).wrapping_add(176); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                    } else {
                        { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(248); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(248))), F32(0x00000000)) as i32)) != 0) {
                            { let v_ = F32(0x00000000); let a_ = (l_w_2).wrapping_add(248); c.m.w32(a_, (v_).0); };
                            { let r_ = (!(256i32)); let a_ = (l_w_2).wrapping_add(176); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                    }
                }
                if (((rt::fgt(F32(c.m.r32((l_key_4).wrapping_add(76))), F32(0x00000000)) as i32)) != 0) {
                    l_r_5 = rt::fdiv(F32(c.m.r32((l_w_2).wrapping_add(252))), F32(c.m.r32((l_key_4).wrapping_add(76))));
                    l_k_6 = rt::fadd(F32(c.m.r32((l_key_4).wrapping_add(68))), rt::fmul(F32(c.m.r32((l_key_4).wrapping_add(72))), l_r_5));
                    { let r_ = l_k_6; let a_ = (l_w_2).wrapping_add(244); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if ((((!(((((c.m.r32((l_w_2).wrapping_add(176)) as i32)) & (512i32))) != 0)) as i32)) != 0) {
                        { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(252); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        if (((rt::fle(F32(c.m.r32((l_key_4).wrapping_add(76))), F32(c.m.r32((l_w_2).wrapping_add(252)))) as i32)) != 0) {
                            { let v_ = F32(c.m.r32((l_key_4).wrapping_add(76))); let a_ = (l_w_2).wrapping_add(252); c.m.w32(a_, (v_).0); };
                            { let r_ = 512i32; let a_ = (l_w_2).wrapping_add(176); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                    } else {
                        { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(252); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(252))), F32(0x00000000)) as i32)) != 0) {
                            { let v_ = F32(0x00000000); let a_ = (l_w_2).wrapping_add(252); c.m.w32(a_, (v_).0); };
                            { let r_ = (!(512i32)); let a_ = (l_w_2).wrapping_add(176); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                    }
                }
            }
            { let r_ = F32(c.m.r32((l_w_2).wrapping_add(44))); let a_ = (l_w_2).wrapping_add(240); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = F32(c.m.r32((l_w_2).wrapping_add(44))); let a_ = (l_w_2).wrapping_add(244); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((((c.m.r32(l_def_3) as i32)) & (8i32))) != 0) {
                { let v_ = rt::fmul(rt::i2f(rt::rem_i32(((c.m.r8((l_w_2).wrapping_add(183))) as i32), ((c.m.r8((l_def_3).wrapping_add(6))) as i32))), F32(c.m.r32((l_w_2).wrapping_add(188)))); let a_ = (l_w_2).wrapping_add(160); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((l_w_2).wrapping_add(160))), F32(c.m.r32((l_w_2).wrapping_add(188)))); let a_ = (l_w_2).wrapping_add(164); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul(rt::i2f(rt::div_i32(((c.m.r8((l_w_2).wrapping_add(183))) as i32), ((c.m.r8((l_def_3).wrapping_add(6))) as i32))), F32(c.m.r32((l_w_2).wrapping_add(192)))); let a_ = (l_w_2).wrapping_add(168); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((l_w_2).wrapping_add(168))), F32(c.m.r32((l_w_2).wrapping_add(192)))); let a_ = (l_w_2).wrapping_add(172); c.m.w32(a_, (v_).0); };
                { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(196); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(200))), F32(c.m.r32((l_w_2).wrapping_add(196)))) as i32)) != 0) {
                    { let r_ = F32(c.m.r32((l_w_2).wrapping_add(200))); let a_ = (l_w_2).wrapping_add(196); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let a_ = (l_w_2).wrapping_add(183); let o_ = c.m.r8(a_); let n_ = (o_).wrapping_add(1); c.m.w8(a_, n_); };
                    if (((((((c.m.r8((l_w_2).wrapping_add(183))) as i32)) >= ((c.m.r32((l_w_2).wrapping_add(184)) as i32))) as i32)) != 0) {
                        { let v_ = ((0i32) as u8); let a_ = (l_w_2).wrapping_add(183); c.m.w8(a_, v_); };
                    }
                }
            }
            { let _ = D::call(c, fid::Vec4_Copy, &[(((l_w_2).wrapping_add(144)) as u64), ((l_key_4) as u64)]); };
            if ((((!(((((c.m.r32((l_w_2).wrapping_add(176)) as i32)) & (64i32))) != 0)) as i32)) != 0) {
                l_r_5 = rt::fdiv(F32(c.m.r32((l_w_2).wrapping_add(260))), F32(c.m.r32((l_w_2).wrapping_add(268))));
                { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(260); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = l_r_5; let a_ = ((l_w_2).wrapping_add(144)).wrapping_add(12); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(268))), F32(c.m.r32((l_w_2).wrapping_add(260)))) as i32)) != 0) {
                    { let r_ = 64i32; let a_ = (l_w_2).wrapping_add(176); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = F32(c.m.r32((l_key_4).wrapping_add(12))); let a_ = ((l_w_2).wrapping_add(144)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                }
            }
            if (((((((((c.m.r32((l_w_2).wrapping_add(176)) as i32)) & (2i32))) != 0) && (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(212))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                if ((((!(((((c.m.r32((l_w_2).wrapping_add(176)) as i32)) & (128i32))) != 0)) as i32)) != 0) {
                    l_r_5 = rt::fdiv(F32(c.m.r32((l_w_2).wrapping_add(264))), F32(c.m.r32((l_w_2).wrapping_add(272))));
                    { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(264); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let r_ = l_r_5; let a_ = ((l_w_2).wrapping_add(144)).wrapping_add(12); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(264))), F32(0x00000000)) as i32)) != 0) {
                        { let v_ = F32(0x00000000); let a_ = ((l_w_2).wrapping_add(144)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        { let r_ = 128i32; let a_ = (l_w_2).wrapping_add(176); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                } else {
                    { let v_ = F32(0x00000000); let a_ = ((l_w_2).wrapping_add(144)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                }
            }
            if (((((((((c.m.r32(l_def_3) as i32)) & (16i32))) != 0) && (((rt::fgt(F32(c.m.r32((l_key_4).wrapping_add(40))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                l_r_5 = rt::fdiv(F32(c.m.r32((l_w_2).wrapping_add(256))), F32(c.m.r32((l_key_4).wrapping_add(40))));
                l_k_6 = rt::fadd(F32(c.m.r32(((l_key_4).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_key_4).wrapping_add(28)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_r_5));
                { let r_ = l_k_6; let a_ = (l_w_2).wrapping_add(144); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                l_k_6 = rt::fadd(F32(c.m.r32(((l_key_4).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_key_4).wrapping_add(28)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_r_5));
                { let r_ = l_k_6; let a_ = ((l_w_2).wrapping_add(144)).wrapping_add(4); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                l_k_6 = rt::fadd(F32(c.m.r32(((l_key_4).wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_key_4).wrapping_add(28)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_r_5));
                { let r_ = l_k_6; let a_ = ((l_w_2).wrapping_add(144)).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if ((((!(((((c.m.r32((l_w_2).wrapping_add(176)) as i32)) & (1024i32))) != 0)) as i32)) != 0) {
                    { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(256); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if (((rt::fle(F32(c.m.r32((l_key_4).wrapping_add(40))), F32(c.m.r32((l_w_2).wrapping_add(256)))) as i32)) != 0) {
                        { let v_ = F32(c.m.r32((l_key_4).wrapping_add(40))); let a_ = (l_w_2).wrapping_add(256); c.m.w32(a_, (v_).0); };
                        { let r_ = 1024i32; let a_ = (l_w_2).wrapping_add(176); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                } else {
                    { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(256); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(256))), F32(0x00000000)) as i32)) != 0) {
                        { let v_ = F32(0x00000000); let a_ = (l_w_2).wrapping_add(256); c.m.w32(a_, (v_).0); };
                        { let r_ = (!(1024i32)); let a_ = (l_w_2).wrapping_add(176); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                }
            }
            if (((((((((((rt::fle(F32(c.m.r32(((l_w_2).wrapping_add(144)).wrapping_add(12))), F32(0x00000000)) as i32)) != 0) || (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(240))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) || (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(244))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                { let r_ = (!(8i32)); let a_ = (l_w_2).wrapping_add(176); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            } else {
                { let r_ = 8i32; let a_ = (l_w_2).wrapping_add(176); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
            if (((((((((c.m.r32(l_def_3) as i32)) & (1i32))) != 0) && (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(220))), F32(c.m.r32((l_w_2).wrapping_add(224)))) as i32)) != 0)) as i32)) != 0) {
                { let _ = D::call(c, fid::EftLine_Animate, &[((l_w_2) as u64)]); };
                { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(220); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(224))), F32(c.m.r32((l_w_2).wrapping_add(220)))) as i32)) != 0) {
                    { let _ = D::call(c, fid::EftLine_SetKey, &[((l_key_4) as u64), ((l_w_2) as u64), ((2i32) as i64 as u64)]); };
                }
            }
            if ((((!(((((c.m.r32((l_w_2).wrapping_add(176)) as i32)) & (16i32))) != 0)) as i32)) != 0) {
                { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(204); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(204))), F32(0x00000000)) as i32)) != 0) {
                    { let r_ = 2i32; let a_ = (l_w_2).wrapping_add(176); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    if (((rt::fgt(F32(c.m.r32((l_w_2).wrapping_add(216))), F32(0x00000000)) as i32)) != 0) {
                        { let r_ = 2048i32; let a_ = (l_w_2).wrapping_add(176); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                }
            }
        } else {
            { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(208); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
        if (((((c.m.r32((l_w_2).wrapping_add(176)) as i32)) & (2i32))) != 0) {
            if (((rt::fgt(F32(c.m.r32((l_w_2).wrapping_add(212))), F32(0x00000000)) as i32)) != 0) {
                { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(212); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            }
            if (((((((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(212))), F32(0x00000000)) as i32)) != 0) && (((rt::fle(F32(c.m.r32(((l_w_2).wrapping_add(144)).wrapping_add(12))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                { let r_ = 4i32; let a_ = (l_w_2).wrapping_add(176); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
    }
    if (((((c.m.r32((l_w_2).wrapping_add(176)) as i32)) & (4i32))) != 0) {
        F::BtlTask_SetDead(c, p_task_1);
    } else {
        { let _ = D::call(c, fid::EftLine_LoadTex, &[((l_w_2) as u64), ((l_w_2) as u64)]); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBill_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftBill_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBill_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBill_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftBill_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBill_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBill_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftBill_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBill_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_def_3: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_def_3 = c.m.r32((l_w_2).wrapping_add(52));
    if (({ let a0_ = (c.m.r32((l_w_2).wrapping_add(32)) as i32); let a1_ = ((c.m.r8((l_w_2).wrapping_add(180))) as i32); F::BtlScene_IsEffectHidden(c, a0_, a1_) }) != 0) {
        break 'f;
    }
    if (((((((((c.m.r32((l_w_2).wrapping_add(176)) as i32)) & (32i32))) != 0) && ((((!(({ let a0_ = (c.m.r32((l_w_2).wrapping_add(32)) as i32); F::BtlScene_IsCharInView(c, a0_) }) != 0)) as i32)) != 0)) as i32)) != 0) {
        break 'f;
    }
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(176)) as i32)) & (8i32))) != 0)) as i32)) != 0) {
        break 'f;
    }
    F::Vu0Cur_Push(c);
    { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
    's1: {
        let sv4: i32 = ((c.m.r8((l_def_3).wrapping_add(5))) as i32);
        match sv4 {
            0i32 => {
                if (((((c.m.r32(l_def_3) as i32)) & (2i32))) != 0) {
                    { let a2_ = ((c.m.r8((l_def_3).wrapping_add(4))) as i32); let a3_ = (((c.m.r32((l_w_2).wrapping_add(176)) as i32)) & (32i32)); let a4_ = c.m.r64((l_w_2).wrapping_add(312)); let a5_ = rt::fmul(F32(c.m.r32((l_w_2).wrapping_add(240))), F32(0x3d800000)); let a6_ = rt::fmul(F32(c.m.r32((l_w_2).wrapping_add(244))), F32(0x3d800000)); let a7_ = F32(c.m.r32((l_w_2).wrapping_add(160))); let a8_ = F32(c.m.r32((l_w_2).wrapping_add(168))); let a9_ = F32(c.m.r32((l_w_2).wrapping_add(164))); let a10_ = F32(c.m.r32((l_w_2).wrapping_add(172))); let a11_ = F32(c.m.r32((l_w_2).wrapping_add(232))); F::EftPrim_DrawQuadDepth(c, l_w_2, (l_w_2).wrapping_add(144), a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_, a10_, a11_) };
                } else {
                    { let a2_ = F32(c.m.r32((l_w_2).wrapping_add(240))); let a3_ = F32(c.m.r32((l_w_2).wrapping_add(244))); let a4_ = F32(c.m.r32((l_w_2).wrapping_add(160))); let a5_ = F32(c.m.r32((l_w_2).wrapping_add(168))); let a6_ = F32(c.m.r32((l_w_2).wrapping_add(164))); let a7_ = F32(c.m.r32((l_w_2).wrapping_add(172))); let a8_ = F32(c.m.r32((l_w_2).wrapping_add(232))); let a9_ = ((c.m.r8((l_def_3).wrapping_add(4))) as i32); let a10_ = (((c.m.r32((l_w_2).wrapping_add(176)) as i32)) & (32i32)); let a11_ = c.m.r64((l_w_2).wrapping_add(312)); F::EftGfx_DrawSprite(c, l_w_2, (l_w_2).wrapping_add(144), a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_, a10_, a11_) };
                }
                break 's1;
            }
            1i32 => {
                if (((((c.m.r32(l_def_3) as i32)) & (2i32))) != 0) {
                    { let _ = D::call(c, fid::EftLine_DrawClipped, &[((p_task_1) as u64)]); };
                } else {
                    { let _ = D::call(c, fid::EftLine_DrawSprite, &[((p_task_1) as u64)]); };
                }
                break 's1;
            }
            _ => {
            }
        }
    }
    F::Vu0Cur_Pop(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBill_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftBillMgr_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBillMgr_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, 4i32) }; let a_ = sym::gEftBillUnused; c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = c.m.r32(sym::gEftBillUnused); c.m.w32(a_, (v_) as u32); };
    { let v_ = F::BtlTask_CreateChildList(c, p_task_1, ((((40i32) as i64 as u64)) as u32), ((((((320u32) as i32)) as i64 as u64)) as u32)); let a_ = sym::gEftBillList; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBillMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftBillMgr_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBillMgr_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftBillUnused); F::BtlPool_Free(c, a0_, a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftBillUnused; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBillMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftBillMgr_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBillMgr_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBillMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftBillMgr_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBillMgr_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBillMgr_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftBill_Create(c: &mut Ctx, mut p_arg_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBill_Create", &PROBE_N, &[(p_arg_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let r_ = 'f: {
        { let s_ = p_arg_1; rt::memcpy(c, fp, s_, 64); }
        break 'f { let a0_ = c.m.r32(sym::gEftBillList); ((D::call(c, fid::BtlTaskList_AddTail, &[((a0_) as u64), ((sym::D_002C40F0) as u64), ((fp) as u64)])) as u32) };
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBill_Create", &PROBE_N, &[], true); }
    r_
}

pub fn eft_z__EftBill_GetWork(c: &mut Ctx, mut p_task_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_z__EftBill_GetWork", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    if (((((((((p_task_1) != (((0i32) as u32))) as i32)) != 0) && ((((({ let a_ = c.m.r32((p_task_1).wrapping_add(40)); c.m.r32(a_) }) == (fid::EftBill_Update)) as i32)) != 0)) as i32)) != 0) {
        break 'f c.m.r32((p_task_1).wrapping_add(56));
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "eft_z__EftBill_GetWork", &PROBE_N, &[], true); }
    r_
}

pub fn EftBill_Stop(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBill_Stop", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = c.m.r32((p_task_1).wrapping_add(40)); c.m.r32(a_) }) != (fid::EftBill_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(176)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let r_ = 2i32; let a_ = (l_w_2).wrapping_add(176); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    if (((rt::fgt(F32(c.m.r32((l_w_2).wrapping_add(216))), F32(0x00000000)) as i32)) != 0) {
        { let r_ = 2048i32; let a_ = (l_w_2).wrapping_add(176); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBill_Stop", &PROBE_N, &[], true); }
    r_
}

pub fn EftBill_Kill(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBill_Kill", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = c.m.r32((p_task_1).wrapping_add(40)); c.m.r32(a_) }) != (fid::EftBill_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(176)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let r_ = 4i32; let a_ = (l_w_2).wrapping_add(176); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBill_Kill", &PROBE_N, &[], true); }
    r_
}

pub fn EftBill_IsAlive(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBill_IsAlive", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = c.m.r32((p_task_1).wrapping_add(40)); c.m.r32(a_) }) != (fid::EftBill_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    break 'f (if (((((c.m.r32((l_w_2).wrapping_add(176)) as i32)) & (1i32))) != 0) { ((((c.m.r32((l_w_2).wrapping_add(176)) as i32)) != (0i32)) as i32) } else { 0i32 });
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBill_IsAlive", &PROBE_N, &[], true); }
    r_
}

pub fn EftBill_SetPos(c: &mut Ctx, mut p_task_1: u32, mut p_pos_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBill_SetPos", &PROBE_N, &[(p_task_1) as u32, (p_pos_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = c.m.r32((p_task_1).wrapping_add(40)); c.m.r32(a_) }) != (fid::EftBill_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(176)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let _ = D::call(c, fid::Vec3_Copy, &[((l_w_3) as u64), ((p_pos_2) as u64)]); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBill_SetPos", &PROBE_N, &[], true); }
    r_
}

pub fn EftBill_SetDir(c: &mut Ctx, mut p_task_1: u32, mut p_dir_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBill_SetDir", &PROBE_N, &[(p_task_1) as u32, (p_dir_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = c.m.r32((p_task_1).wrapping_add(40)); c.m.r32(a_) }) != (fid::EftBill_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(176)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let _ = D::call(c, fid::Vec3_Normalize, &[(((l_w_3).wrapping_add(16)) as u64), ((p_dir_2) as u64)]); };
    { let v_ = F32(0x3f800000); let a_ = ((l_w_3).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBill_SetDir", &PROBE_N, &[], true); }
    r_
}

pub fn EftBill_SetSize(c: &mut Ctx, mut p_task_1: u32, mut p_size_2: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBill_SetSize", &PROBE_N, &[(p_task_1) as u32, (p_size_2).0], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = c.m.r32((p_task_1).wrapping_add(40)); c.m.r32(a_) }) != (fid::EftBill_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(176)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let v_ = p_size_2; let a_ = (l_w_3).wrapping_add(44); c.m.w32(a_, (v_).0); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBill_SetSize", &PROBE_N, &[], true); }
    r_
}

pub fn EftBill_SetDelay(c: &mut Ctx, mut p_task_1: u32, mut p_frames_2: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBill_SetDelay", &PROBE_N, &[(p_task_1) as u32, (p_frames_2).0], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = c.m.r32((p_task_1).wrapping_add(40)); c.m.r32(a_) }) != (fid::EftBill_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(176)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let v_ = p_frames_2; let a_ = (l_w_3).wrapping_add(208); c.m.w32(a_, (v_).0); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBill_SetDelay", &PROBE_N, &[], true); }
    r_
}

pub fn EftBill_SetEndDelay(c: &mut Ctx, mut p_task_1: u32, mut p_frames_2: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBill_SetEndDelay", &PROBE_N, &[(p_task_1) as u32, (p_frames_2).0], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = c.m.r32((p_task_1).wrapping_add(40)); c.m.r32(a_) }) != (fid::EftBill_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(176)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let v_ = p_frames_2; let a_ = (l_w_3).wrapping_add(212); c.m.w32(a_, (v_).0); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBill_SetEndDelay", &PROBE_N, &[], true); }
    r_
}

pub fn EftBill_SetUnkD8(c: &mut Ctx, mut p_task_1: u32, mut p_v_2: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBill_SetUnkD8", &PROBE_N, &[(p_task_1) as u32, (p_v_2).0], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = c.m.r32((p_task_1).wrapping_add(40)); c.m.r32(a_) }) != (fid::EftBill_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(176)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let v_ = p_v_2; let a_ = (l_w_3).wrapping_add(216); c.m.w32(a_, (v_).0); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBill_SetUnkD8", &PROBE_N, &[], true); }
    r_
}

pub fn EftBill_SetTexture(c: &mut Ctx, mut p_task_1: u32, mut p_a1_2: i32, mut p_a2_3: i32, mut p_a3_4: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBill_SetTexture", &PROBE_N, &[(p_task_1) as u32, (p_a1_2) as u32, (p_a2_3) as u32, (p_a3_4) as u32], false); }
    let mut l_w_5: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = c.m.r32((p_task_1).wrapping_add(40)); c.m.r32(a_) }) != (fid::EftBill_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_5 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_5).wrapping_add(176)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let _ = D::call(c, fid::EftLine_SetTex, &[((l_w_5) as u64), ((((p_a1_2) as u32)) as u64), ((p_a2_3) as i64 as u64), ((p_a3_4) as i64 as u64)]); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBill_SetTexture", &PROBE_N, &[], true); }
    r_
}

pub fn EftBill_SetFront(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBill_SetFront", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = c.m.r32((p_task_1).wrapping_add(40)); c.m.r32(a_) }) != (fid::EftBill_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(176)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let r_ = 32i32; let a_ = (l_w_2).wrapping_add(176); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBill_SetFront", &PROBE_N, &[], true); }
    r_
}

pub fn EftBill_SetType(c: &mut Ctx, mut p_task_1: u32, mut p_type_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBill_SetType", &PROBE_N, &[(p_task_1) as u32, (p_type_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = c.m.r32((p_task_1).wrapping_add(40)); c.m.r32(a_) }) != (fid::EftBill_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(176)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let v_ = ((p_type_2) as u8); let a_ = (l_w_3).wrapping_add(180); c.m.w8(a_, v_); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBill_SetType", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDust_Create(c: &mut Ctx, mut p_arg_1: u32, mut p_kind_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_Create", &PROBE_N, &[(p_arg_1) as u32, (p_kind_2) as u32], false); }
    let mut l_task_3: u32 = 0;
    let r_ = 'f: {
    l_task_3 = ((0i32) as u32);
    if (((((c.m.r32(sym::gEftGndDust)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    's1: {
        let sv4: i32 = p_kind_2;
        match sv4 {
            0i32 => {
                l_task_3 = { let a0_ = c.m.r32(sym::gEftGndDustList); ((D::call(c, fid::BtlTaskList_AddTail, &[((a0_) as u64), ((sym::D_002C4120) as u64), ((p_arg_1) as u64)])) as u32) };
                break 's1;
            }
            1i32 => {
                l_task_3 = { let a0_ = c.m.r32(sym::gEftGndDustList); ((D::call(c, fid::BtlTaskList_AddTail, &[((a0_) as u64), ((sym::D_002C4138) as u64), ((p_arg_1) as u64)])) as u32) };
                break 's1;
            }
            2i32 => {
                l_task_3 = { let a0_ = c.m.r32(sym::gEftGndDustList); ((D::call(c, fid::BtlTaskList_AddTail, &[((a0_) as u64), ((sym::D_002C4150) as u64), ((p_arg_1) as u64)])) as u32) };
                break 's1;
            }
            3i32 => {
                l_task_3 = { let a0_ = c.m.r32(sym::gEftGndDustList); ((D::call(c, fid::BtlTaskList_AddTail, &[((a0_) as u64), ((sym::D_002C4168) as u64), ((p_arg_1) as u64)])) as u32) };
                break 's1;
            }
            4i32 => {
                l_task_3 = { let a0_ = c.m.r32(sym::gEftGndDustList); ((D::call(c, fid::BtlTaskList_AddTail, &[((a0_) as u64), ((sym::D_002C4180) as u64), ((p_arg_1) as u64)])) as u32) };
                break 's1;
            }
            5i32 => {
                l_task_3 = { let a0_ = c.m.r32(sym::gEftGndDustList); ((D::call(c, fid::BtlTaskList_AddTail, &[((a0_) as u64), ((sym::D_002C4198) as u64), ((p_arg_1) as u64)])) as u32) };
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f l_task_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_Create", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDust_SpawnDebris(c: &mut Ctx, mut p_pos_1: u32, mut p_scale_2: F32, mut p_unused_3: F32, mut p_bright_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_SpawnDebris", &PROBE_N, &[(p_pos_1) as u32, (p_scale_2).0, (p_unused_3).0, (p_bright_4).0], false); }
    let mut l_arg_5: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftGndDust)) != (((0i32) as u32))) as i32)) != 0) {
        l_arg_5 = F::EftGndDust_GetTemplate(c);
        { let s_ = p_pos_1; let d_ = l_arg_5; rt::memcpy(c, d_, s_, 16); };
        { let v_ = p_scale_2; let a_ = (l_arg_5).wrapping_add(96); c.m.w32(a_, (v_).0); };
        { let v_ = p_bright_4; let a_ = (l_arg_5).wrapping_add(76); c.m.w32(a_, (v_).0); };
        { let _ = F::EftGndDust_Create(c, l_arg_5, 0i32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_SpawnDebris", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDust_SetSlide(c: &mut Ctx, mut p_objId_1: i32, mut p_stop_2: i32, mut p_bright_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_SetSlide", &PROBE_N, &[(p_objId_1) as u32, (p_stop_2) as u32, (p_bright_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let mut l_w_4: u32 = 0;
    let r_ = 'f: {
        if (((((c.m.r32(sym::gEftGndDust)) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if ((((({ let a_ = ((c.m.r32(sym::gEftGndDust)).wrapping_add(25392)).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
            { let _ = F::memset(c, fp, 0i32, 112u32); };
            { let _ = D::call(c, fid::Vec4_Set, &[((fp) as u64), ((F32(0x00000000)).0 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
            { let _ = D::call(c, fid::Vec4_Set, &[(((fp).wrapping_add(16)) as u64), ((F32(0x00000000)).0 as u64), ((F32(0xbf800000)).0 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
            { let _ = D::call(c, fid::Vec4_Set, &[(((fp).wrapping_add(32)) as u64), ((F32(0x00000000)).0 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
            F::EftGndDust_GetLightColors(c, (fp).wrapping_add(48), (fp).wrapping_add(52));
            { let v_ = ((p_objId_1) as i8); let a_ = (fp).wrapping_add(56); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((1i32) as i8); let a_ = (fp).wrapping_add(57); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((0i32) as i8); let a_ = (fp).wrapping_add(58); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((0i32) as i8); let a_ = (fp).wrapping_add(59); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((50i32) as i16); let a_ = (fp).wrapping_add(60); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((15i32) as i16); let a_ = (fp).wrapping_add(62); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((10i32) as i16); let a_ = (fp).wrapping_add(64); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((0i32) as i16); let a_ = (fp).wrapping_add(66); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((0i32) as i16); let a_ = (fp).wrapping_add(68); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((0i32) as i16); let a_ = (fp).wrapping_add(70); c.m.w16(a_, (v_) as u16); };
            { let v_ = F32(0x3dcccccc); let a_ = (fp).wrapping_add(72); c.m.w32(a_, (v_).0); };
            { let v_ = p_bright_3; let a_ = (fp).wrapping_add(76); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f8ccccc); let a_ = (fp).wrapping_add(80); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(84); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(88); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(92); c.m.w32(a_, (v_).0); };
            { let v_ = F::BtlScene_GetCharScale(c, p_objId_1); let a_ = (fp).wrapping_add(96); c.m.w32(a_, (v_).0); };
            { let v_ = F::EftGndDust_Create(c, fp, 1i32); let a_ = ((c.m.r32(sym::gEftGndDust)).wrapping_add(25392)).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            if ((((({ let a_ = ((c.m.r32(sym::gEftGndDust)).wrapping_add(25392)).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
                break 'f;
            }
        }
        l_w_4 = { let a_ = ({ let a_ = ((c.m.r32(sym::gEftGndDust)).wrapping_add(25392)).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(56); c.m.r32(a_) };
        's1: {
            let sv5: i32 = p_stop_2;
            match sv5 {
                0i32 => {
                    { let r_ = 2i32; let a_ = (l_w_4).wrapping_add(180); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let r_ = (!(4i32)); let a_ = (l_w_4).wrapping_add(180); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                    break 's1;
                }
                1i32 => {
                    { let r_ = (!(2i32)); let a_ = (l_w_4).wrapping_add(180); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                    { let r_ = 4i32; let a_ = (l_w_4).wrapping_add(180); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    break 's1;
                }
                _ => {
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_SetSlide", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDust_SetDash(c: &mut Ctx, mut p_objId_1: i32, mut p_stop_2: i32, mut p_bright_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_SetDash", &PROBE_N, &[(p_objId_1) as u32, (p_stop_2) as u32, (p_bright_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let mut l_w_4: u32 = 0;
    let r_ = 'f: {
        if (((((c.m.r32(sym::gEftGndDust)) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if ((((({ let a_ = ((c.m.r32(sym::gEftGndDust)).wrapping_add(25400)).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
            { let _ = F::memset(c, fp, 0i32, 112u32); };
            { let _ = D::call(c, fid::Vec4_Set, &[((fp) as u64), ((F32(0x00000000)).0 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
            { let _ = D::call(c, fid::Vec4_Set, &[(((fp).wrapping_add(16)) as u64), ((F32(0x00000000)).0 as u64), ((F32(0xbf800000)).0 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
            { let _ = D::call(c, fid::Vec4_Set, &[(((fp).wrapping_add(32)) as u64), ((F32(0x00000000)).0 as u64), ((F32(0xbf800000)).0 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
            F::EftGndDust_GetLightColors(c, (fp).wrapping_add(48), (fp).wrapping_add(52));
            { let v_ = ((p_objId_1) as i8); let a_ = (fp).wrapping_add(56); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((1i32) as i8); let a_ = (fp).wrapping_add(57); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((0i32) as i8); let a_ = (fp).wrapping_add(58); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((0i32) as i8); let a_ = (fp).wrapping_add(59); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((100i32) as i16); let a_ = (fp).wrapping_add(60); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((25i32) as i16); let a_ = (fp).wrapping_add(62); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((10i32) as i16); let a_ = (fp).wrapping_add(64); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((2i32) as i16); let a_ = (fp).wrapping_add(66); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((0i32) as i16); let a_ = (fp).wrapping_add(68); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((0i32) as i16); let a_ = (fp).wrapping_add(70); c.m.w16(a_, (v_) as u16); };
            { let v_ = F32(0x3d4ccccc); let a_ = (fp).wrapping_add(72); c.m.w32(a_, (v_).0); };
            { let v_ = p_bright_3; let a_ = (fp).wrapping_add(76); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f8ccccc); let a_ = (fp).wrapping_add(80); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3c23d70a); let a_ = (fp).wrapping_add(84); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(88); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3ba3d70a); let a_ = (fp).wrapping_add(92); c.m.w32(a_, (v_).0); };
            { let v_ = F::BtlScene_GetCharScale(c, p_objId_1); let a_ = (fp).wrapping_add(96); c.m.w32(a_, (v_).0); };
            { let v_ = F::EftGndDust_Create(c, fp, 2i32); let a_ = ((c.m.r32(sym::gEftGndDust)).wrapping_add(25400)).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            if ((((({ let a_ = ((c.m.r32(sym::gEftGndDust)).wrapping_add(25400)).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
                break 'f;
            }
        }
        l_w_4 = { let a_ = ({ let a_ = ((c.m.r32(sym::gEftGndDust)).wrapping_add(25400)).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(56); c.m.r32(a_) };
        's1: {
            let sv5: i32 = p_stop_2;
            match sv5 {
                0i32 => {
                    { let r_ = 2i32; let a_ = (l_w_4).wrapping_add(180); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    break 's1;
                }
                1i32 => {
                    { let r_ = (!(2i32)); let a_ = (l_w_4).wrapping_add(180); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                    break 's1;
                }
                _ => {
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_SetDash", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDust_SpawnBurst(c: &mut Ctx, mut p_objId_1: i32, mut p_scale_2: F32, mut p_bright_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_SpawnBurst", &PROBE_N, &[(p_objId_1) as u32, (p_scale_2).0, (p_bright_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let r_ = 'f: {
        if (((((c.m.r32(sym::gEftGndDust)) != (((0i32) as u32))) as i32)) != 0) {
            { let _ = F::memset(c, fp, 0i32, 112u32); };
            { let _ = D::call(c, fid::Vec4_Set, &[((fp) as u64), ((F32(0x00000000)).0 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
            { let _ = D::call(c, fid::Vec4_Set, &[(((fp).wrapping_add(16)) as u64), ((F32(0x00000000)).0 as u64), ((F32(0xbf800000)).0 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
            { let _ = D::call(c, fid::Vec4_Set, &[(((fp).wrapping_add(32)) as u64), ((F32(0x00000000)).0 as u64), ((F32(0xbf800000)).0 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
            F::EftGndDust_GetLightColors(c, (fp).wrapping_add(48), (fp).wrapping_add(52));
            { let v_ = ((p_objId_1) as i8); let a_ = (fp).wrapping_add(56); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((1i32) as i8); let a_ = (fp).wrapping_add(57); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((0i32) as i8); let a_ = (fp).wrapping_add(58); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((0i32) as i8); let a_ = (fp).wrapping_add(59); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((20i32) as i16); let a_ = (fp).wrapping_add(60); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((15i32) as i16); let a_ = (fp).wrapping_add(62); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((10i32) as i16); let a_ = (fp).wrapping_add(64); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((2i32) as i16); let a_ = (fp).wrapping_add(66); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((0i32) as i16); let a_ = (fp).wrapping_add(68); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((0i32) as i16); let a_ = (fp).wrapping_add(70); c.m.w16(a_, (v_) as u16); };
            { let v_ = p_bright_3; let a_ = (fp).wrapping_add(76); c.m.w32(a_, (v_).0); };
            { let v_ = F::BtlScene_GetCharScale(c, p_objId_1); let a_ = (fp).wrapping_add(96); c.m.w32(a_, (v_).0); };
            if (((rt::flt(F32(0x3f800000), F32(c.m.r32((fp).wrapping_add(96)))) as i32)) != 0) {
                { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(96); c.m.w32(a_, (v_).0); };
            }
            { let r_ = p_scale_2; let a_ = (fp).wrapping_add(96); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let _ = F::EftGndDust_Create(c, fp, 3i32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_SpawnBurst", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDust_SpawnLanding(c: &mut Ctx, mut p_objId_1: i32, mut p_pos_2: u32, mut p_normal_3: u32, mut p_bright_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_SpawnLanding", &PROBE_N, &[(p_objId_1) as u32, (p_pos_2) as u32, (p_normal_3) as u32, (p_bright_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(128) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 128);
    let r_ = 'f: {
        rt::memset(c, fp.wrapping_add(112), 0, 16);
        rt::memset(c, fp.wrapping_add(112).wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(112).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(112).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(112).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(112).wrapping_add(0).wrapping_add(12), (v_).0); }
        if (((((c.m.r32(sym::gEftGndDust)) != (((0i32) as u32))) as i32)) != 0) {
            { let _ = F::memset(c, fp, 0i32, 112u32); };
            { let _ = D::call(c, fid::Vec4_Copy, &[((fp) as u64), ((p_pos_2) as u64)]); };
            { let _ = D::call(c, fid::Vec4_Copy, &[(((fp).wrapping_add(16)) as u64), ((fp.wrapping_add(112)) as u64)]); };
            { let v_ = p_bright_4; let a_ = (fp).wrapping_add(76); c.m.w32(a_, (v_).0); };
            { let v_ = ((p_objId_1) as i8); let a_ = (fp).wrapping_add(56); c.m.w8(a_, (v_) as u8); };
            { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(96); c.m.w32(a_, (v_).0); };
            { let _ = F::EftGndDust_Create(c, fp, 4i32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_SpawnLanding", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDust_SpawnLandingScaled(c: &mut Ctx, mut p_objId_1: i32, mut p_pos_2: u32, mut p_dir_3: u32, mut p_bright_4: F32, mut p_scale_5: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_SpawnLandingScaled", &PROBE_N, &[(p_objId_1) as u32, (p_pos_2) as u32, (p_dir_3) as u32, (p_bright_4).0, (p_scale_5).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(128) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 128);
    let r_ = 'f: {
        rt::memset(c, fp.wrapping_add(112), 0, 16);
        rt::memset(c, fp.wrapping_add(112).wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(112).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(112).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(112).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(112).wrapping_add(0).wrapping_add(12), (v_).0); }
        if (((((c.m.r32(sym::gEftGndDust)) != (((0i32) as u32))) as i32)) != 0) {
            { let _ = F::memset(c, fp, 0i32, 112u32); };
            { let _ = D::call(c, fid::Vec4_Copy, &[((fp) as u64), ((p_pos_2) as u64)]); };
            { let _ = D::call(c, fid::Vec4_Copy, &[(((fp).wrapping_add(16)) as u64), ((fp.wrapping_add(112)) as u64)]); };
            { let v_ = p_bright_4; let a_ = (fp).wrapping_add(76); c.m.w32(a_, (v_).0); };
            { let v_ = ((p_objId_1) as i8); let a_ = (fp).wrapping_add(56); c.m.w8(a_, (v_) as u8); };
            { let v_ = p_scale_5; let a_ = (fp).wrapping_add(96); c.m.w32(a_, (v_).0); };
            { let _ = F::EftGndDust_Create(c, fp, 4i32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_SpawnLandingScaled", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDust_Stub(c: &mut Ctx, mut p_objId_1: i32, mut p_scale_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_Stub", &PROBE_N, &[(p_objId_1) as u32, (p_scale_2).0], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_Stub", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDust_SpawnImpact(c: &mut Ctx, mut p_objId_1: i32, mut p_pos_2: u32, mut p_bright_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_SpawnImpact", &PROBE_N, &[(p_objId_1) as u32, (p_pos_2) as u32, (p_bright_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(128) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 128);
    let r_ = 'f: {
        rt::memset(c, fp.wrapping_add(112), 0, 16);
        rt::memset(c, fp.wrapping_add(112).wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(112).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(112).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(112).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(112).wrapping_add(0).wrapping_add(12), (v_).0); }
        if (((((c.m.r32(sym::gEftGndDust)) != (((0i32) as u32))) as i32)) != 0) {
            { let _ = F::memset(c, fp, 0i32, 112u32); };
            { let _ = D::call(c, fid::Vec4_Copy, &[((fp) as u64), ((p_pos_2) as u64)]); };
            { let _ = D::call(c, fid::Vec4_Copy, &[(((fp).wrapping_add(16)) as u64), ((fp.wrapping_add(112)) as u64)]); };
            { let v_ = p_bright_3; let a_ = (fp).wrapping_add(76); c.m.w32(a_, (v_).0); };
            { let v_ = ((p_objId_1) as i8); let a_ = (fp).wrapping_add(56); c.m.w8(a_, (v_) as u8); };
            { let _ = F::EftGndDust_Create(c, fp, 5i32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_SpawnImpact", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDust_GetTemplate(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_GetTemplate", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftGndDust)) != (((0i32) as u32))) as i32)) != 0) {
        break 'f (c.m.r32(sym::gEftGndDust)).wrapping_add(25280);
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_GetTemplate", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDust_InitTemplate(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_InitTemplate", &PROBE_N, &[], false); }
    let mut l_t_1: u32 = 0;
    let mut l_stg_2: u32 = 0;
    let r_ = 'f: {
    l_t_1 = (c.m.r32(sym::gEftGndDust)).wrapping_add(25280);
    if (((((c.m.r32(sym::gEftGndDust)) != (((0i32) as u32))) as i32)) != 0) {
        { let _ = D::call(c, fid::Vec4_Set, &[((l_t_1) as u64), ((F32(0x00000000)).0 as u64), ((F32(0xc2c80000)).0 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
        { let _ = D::call(c, fid::Vec4_Set, &[(((l_t_1).wrapping_add(16)) as u64), ((F32(0x00000000)).0 as u64), ((F32(0xbf800000)).0 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
        { let _ = D::call(c, fid::Vec4_Set, &[(((l_t_1).wrapping_add(32)) as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f800000)).0 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
        { let v_ = ((80i32) as u8); let a_ = ((l_t_1).wrapping_add(48)).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = ((80i32) as u8); let a_ = ((l_t_1).wrapping_add(48)).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = ((35i32) as u8); let a_ = ((l_t_1).wrapping_add(48)).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = ((0i32) as i8); let a_ = (l_t_1).wrapping_add(57); c.m.w8(a_, (v_) as u8); };
        { let v_ = F32(0x00000000); let a_ = (l_t_1).wrapping_add(84); c.m.w32(a_, (v_).0); };
        { let v_ = ((80i32) as u8); let a_ = ((l_t_1).wrapping_add(52)).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = ((55i32) as u8); let a_ = ((l_t_1).wrapping_add(48)).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = ((65i32) as u8); let a_ = ((l_t_1).wrapping_add(52)).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = ((45i32) as u8); let a_ = ((l_t_1).wrapping_add(52)).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = ((30i32) as u8); let a_ = ((l_t_1).wrapping_add(52)).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = ((0i32) as i8); let a_ = (l_t_1).wrapping_add(58); c.m.w8(a_, (v_) as u8); };
        { let v_ = ((0i32) as i8); let a_ = (l_t_1).wrapping_add(59); c.m.w8(a_, (v_) as u8); };
        { let v_ = (((1i32).wrapping_neg()) as i8); let a_ = (l_t_1).wrapping_add(56); c.m.w8(a_, (v_) as u8); };
        { let v_ = F32(0x3f000000); let a_ = (l_t_1).wrapping_add(80); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f666666); let a_ = (l_t_1).wrapping_add(88); c.m.w32(a_, (v_).0); };
        { let v_ = ((0i32) as i16); let a_ = (l_t_1).wrapping_add(66); c.m.w16(a_, (v_) as u16); };
        { let v_ = ((500i32) as i16); let a_ = (l_t_1).wrapping_add(60); c.m.w16(a_, (v_) as u16); };
        { let v_ = F32(0x3c23d70a); let a_ = (l_t_1).wrapping_add(72); c.m.w32(a_, (v_).0); };
        { let v_ = ((30i32) as i16); let a_ = (l_t_1).wrapping_add(62); c.m.w16(a_, (v_) as u16); };
        { let v_ = F32(0x3f800000); let a_ = (l_t_1).wrapping_add(76); c.m.w32(a_, (v_).0); };
        { let v_ = ((30i32) as i16); let a_ = (l_t_1).wrapping_add(64); c.m.w16(a_, (v_) as u16); };
        { let v_ = F32(0x00000000); let a_ = (l_t_1).wrapping_add(92); c.m.w32(a_, (v_).0); };
        { let v_ = ((17i32) as i16); let a_ = (l_t_1).wrapping_add(68); c.m.w16(a_, (v_) as u16); };
        { let v_ = ((40i32) as i16); let a_ = (l_t_1).wrapping_add(70); c.m.w16(a_, (v_) as u16); };
        { let v_ = F32(0x3f800000); let a_ = (l_t_1).wrapping_add(96); c.m.w32(a_, (v_).0); };
        l_stg_2 = F::BtlStage_GetList68(c);
        if ((((((c.m.r32(l_stg_2) as i32)) != (0i32)) as i32)) != 0) {
            { let v_ = c.m.r8(((l_stg_2).wrapping_add(4)).wrapping_add(((0i32) as u32))); let a_ = ((l_t_1).wrapping_add(48)).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = c.m.r8(((l_stg_2).wrapping_add(4)).wrapping_add(((1i32) as u32))); let a_ = ((l_t_1).wrapping_add(48)).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = c.m.r8(((l_stg_2).wrapping_add(4)).wrapping_add(((2i32) as u32))); let a_ = ((l_t_1).wrapping_add(48)).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = c.m.r8(((l_stg_2).wrapping_add(4)).wrapping_add(((3i32) as u32))); let a_ = ((l_t_1).wrapping_add(48)).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = c.m.r8(((l_stg_2).wrapping_add(8)).wrapping_add(((0i32) as u32))); let a_ = ((l_t_1).wrapping_add(52)).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = c.m.r8(((l_stg_2).wrapping_add(8)).wrapping_add(((1i32) as u32))); let a_ = ((l_t_1).wrapping_add(52)).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = c.m.r8(((l_stg_2).wrapping_add(8)).wrapping_add(((2i32) as u32))); let a_ = ((l_t_1).wrapping_add(52)).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = c.m.r8(((l_stg_2).wrapping_add(8)).wrapping_add(((3i32) as u32))); let a_ = ((l_t_1).wrapping_add(52)).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = F32(c.m.r32((l_stg_2).wrapping_add(12))); let a_ = (l_t_1).wrapping_add(80); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32((l_stg_2).wrapping_add(16))); let a_ = (l_t_1).wrapping_add(84); c.m.w32(a_, (v_).0); };
            { let v_ = ((c.m.r16((l_stg_2).wrapping_add(20))) as i16); let a_ = (l_t_1).wrapping_add(60); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((c.m.r8((l_stg_2).wrapping_add(22))) as i16); let a_ = (l_t_1).wrapping_add(68); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((c.m.r8((l_stg_2).wrapping_add(23))) as i16); let a_ = (l_t_1).wrapping_add(70); c.m.w16(a_, (v_) as u16); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_InitTemplate", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDustMgr_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustMgr_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((25408u32) as i32)) }; let a_ = sym::gEftGndDust; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gEftGndDust); F::memset(c, a0_, 0i32, 25408u32) }; };
    { let v_ = c.m.r32(sym::gEftGndDust); let a_ = ((c.m.r32(sym::gEftGndDust)).wrapping_add(24600)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let a0_ = ((c.m.r32(sym::gEftGndDust)).wrapping_add(24576)).wrapping_add((((0i32) as u32)).wrapping_mul(12)); F::List_Init(c, a0_) };
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (64i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gEftGndDust)).wrapping_add(24576)).wrapping_add((((0i32) as u32)).wrapping_mul(12)); let a1_ = ({ let a_ = ((c.m.r32(sym::gEftGndDust)).wrapping_add(24600)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((l_i_2) as u32)).wrapping_mul(192)); F::List_PushBack(c, a0_, a1_) };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    { let v_ = (c.m.r32(sym::gEftGndDust)).wrapping_add(12288); let a_ = ((c.m.r32(sym::gEftGndDust)).wrapping_add(24600)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let a0_ = ((c.m.r32(sym::gEftGndDust)).wrapping_add(24576)).wrapping_add((((1i32) as u32)).wrapping_mul(12)); F::List_Init(c, a0_) };
    {
        l_i_2 = 0i32;
        'l3: loop {
            if !(((((l_i_2) < (63i32)) as i32)) != 0) { break 'l3; }
            'c4: {
                { let a0_ = ((c.m.r32(sym::gEftGndDust)).wrapping_add(24576)).wrapping_add((((1i32) as u32)).wrapping_mul(12)); let a1_ = ({ let a_ = ((c.m.r32(sym::gEftGndDust)).wrapping_add(24600)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((l_i_2) as u32)).wrapping_mul(192)); F::List_PushBack(c, a0_, a1_) };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    { let a0_ = (c.m.r32(sym::gEftGndDust)).wrapping_add(25128); let a1_ = F::BtlScene_GetCommonEntry(c, 11i32); F::EftTexSet_Load8(c, a0_, a1_) };
    { let v_ = (c.m.r32(sym::gEftGndDust)).wrapping_add(25128); let a_ = (c.m.r32(sym::gEftGndDust)).wrapping_add(25264); c.m.w32(a_, v_); };
    { let v_ = F::BtlTask_CreateChildList(c, p_task_1, ((((16i32) as i64 as u64)) as u32), ((((((224u32) as i32)) as i64 as u64)) as u32)); let a_ = sym::gEftGndDustList; c.m.w32(a_, v_); };
    F::EftGndDust_InitTemplate(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDustMgr_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustMgr_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftGndDust); F::BtlPool_Free(c, a0_, a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftGndDust; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDustMgr_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustMgr_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftGndDust)) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gEftGndDust)).wrapping_add(25264); c.m.r32(a_) }).wrapping_add(132); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDustPuff_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustPuff_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let mut l_p_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_rMin_6: F32 = F32(0);
    let mut l_rMax_7: F32 = F32(0);
    let mut l_spin_8: F32 = F32(0);
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let _ = F::memset(c, l_w_3, 0i32, 224u32); };
    { let s_ = p_arg_2; let d_ = l_w_3; rt::memcpy(c, d_, s_, 112); };
    F::List_Init(c, (l_w_3).wrapping_add(204));
    { let v_ = { let a0_ = (((c.m.r8((p_arg_2).wrapping_add(57)) as i8)) as i32); F::EftGndDust_GetFreeList(c, a0_) }; let a_ = (l_w_3).wrapping_add(200); c.m.w32(a_, v_); };
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((l_i_5) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_p_4 = { let a1_ = c.m.r32((l_w_3).wrapping_add(200)); F::EftGndDust_AllocPart(c, (l_w_3).wrapping_add(204), a1_) };
                if (((((l_p_4) == (((0i32) as u32))) as i32)) != 0) {
                    break 'f;
                }
                { let _ = D::call(c, fid::Vec4_Copy, &[(((l_p_4).wrapping_add(16)) as u64), ((p_arg_2) as u64)]); };
                { let _ = D::call(c, fid::Vec4_Copy, &[(((l_p_4).wrapping_add(48)) as u64), (((p_arg_2).wrapping_add(16)) as u64)]); };
                { let _ = D::call(c, fid::Vec4_Set, &[(((l_p_4).wrapping_add(64)) as u64), ((F32(0x43000000)).0 as u64), ((F32(0x43000000)).0 as u64), ((F32(0x43000000)).0 as u64), ((F32(0x43000000)).0 as u64)]); };
                { let _ = D::call(c, fid::Vec4_Set, &[(((l_p_4).wrapping_add(32)) as u64), ((F32(0x00000000)).0 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
                { let _ = D::call(c, fid::Vec4_Set, &[(((l_p_4).wrapping_add(96)) as u64), ((F32(0x3f800000)).0 as u64), ((F32(0x3f800000)).0 as u64), ((F32(0x3f800000)).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
                { let a1_ = F32(c.m.r32((p_arg_2).wrapping_add(72))); let a2_ = F32(c.m.r32((p_arg_2).wrapping_add(72))); { let _ = D::call(c, fid::Vec4_Set, &[(((l_p_4).wrapping_add(112)) as u64), ((a1_).0 as u64), ((a2_).0 as u64), ((F32(0x3f800000)).0 as u64), ((F32(0x3f800000)).0 as u64)]); } };
                { let a2_ = rt::fadd(rt::fmul(F32(c.m.r32((p_arg_2).wrapping_add(92))), F32(c.m.r32((p_arg_2).wrapping_add(96)))), F32(c.m.r32((p_arg_2).wrapping_add(84)))); { let _ = D::call(c, fid::Vec3_Scale, &[(((l_p_4).wrapping_add(128)) as u64), (((p_arg_2).wrapping_add(32)) as u64), ((a2_).0 as u64)]); } };
                { let s_ = (p_arg_2).wrapping_add(48); let d_ = (l_p_4).wrapping_add(144); rt::memcpy(c, d_, s_, 4); };
                { let s_ = (p_arg_2).wrapping_add(52); let d_ = (l_p_4).wrapping_add(148); rt::memcpy(c, d_, s_, 4); };
                { let v_ = rt::fsub(rt::fmul(F::Rand_Float01(c), F32(0x40000000)), F32(0x3f800000)); let a_ = (l_p_4).wrapping_add(48); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fsub(rt::fmul(F::Rand_Float01(c), F32(0x40000000)), F32(0x3f800000)); let a_ = ((l_p_4).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fsub(rt::fmul(F::Rand_Float01(c), F32(0x40000000)), F32(0x3f800000)); let a_ = ((l_p_4).wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = ((l_p_4).wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                { let _ = D::call(c, fid::Vec3_Normalize, &[(((l_p_4).wrapping_add(48)) as u64), (((l_p_4).wrapping_add(48)) as u64)]); };
                l_rMin_6 = rt::fmul(rt::i2f((((c.m.r16((p_arg_2).wrapping_add(68)) as i16)) as i32)), F32(c.m.r32((p_arg_2).wrapping_add(96))));
                l_rMax_7 = rt::fmul(rt::i2f((((c.m.r16((p_arg_2).wrapping_add(70)) as i16)) as i32)), F32(c.m.r32((p_arg_2).wrapping_add(96))));
                { let v_ = rt::fadd(F32(c.m.r32(p_arg_2)), rt::fmul(F32(c.m.r32((l_p_4).wrapping_add(48))), rt::fadd(l_rMin_6, rt::fmul(F::Rand_Float01(c), rt::fsub(l_rMax_7, l_rMin_6))))); let a_ = (l_p_4).wrapping_add(16); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((p_arg_2).wrapping_add(4))), rt::fmul(F32(c.m.r32(((l_p_4).wrapping_add(48)).wrapping_add(4))), rt::fadd(l_rMin_6, rt::fmul(F::Rand_Float01(c), rt::fsub(l_rMax_7, l_rMin_6))))); let a_ = ((l_p_4).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((p_arg_2).wrapping_add(8))), rt::fmul(F32(c.m.r32(((l_p_4).wrapping_add(48)).wrapping_add(8))), rt::fadd(l_rMin_6, rt::fmul(F::Rand_Float01(c), rt::fsub(l_rMax_7, l_rMin_6))))); let a_ = ((l_p_4).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = ((l_p_4).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((p_arg_2).wrapping_add(80))), rt::fmul(F32(c.m.r32((p_arg_2).wrapping_add(84))), F::Rand_Float01(c))); let a_ = (l_p_4).wrapping_add(172); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32((p_arg_2).wrapping_add(88))); let a_ = (l_p_4).wrapping_add(176); c.m.w32(a_, (v_).0); };
                { let v_ = { let v_ = ((rt::f2i(rt::i2f(((c.m.r16((p_arg_2).wrapping_add(62)) as i16)) as i32))) as i16); let a_ = (l_p_4).wrapping_add(152); c.m.w16(a_, (v_) as u16); v_ }; let a_ = (l_p_4).wrapping_add(154); c.m.w16(a_, (v_) as u16); };
                { let v_ = { let v_ = ((rt::f2i(rt::i2f(((c.m.r16((p_arg_2).wrapping_add(64)) as i16)) as i32))) as i16); let a_ = (l_p_4).wrapping_add(158); c.m.w16(a_, (v_) as u16); v_ }; let a_ = (l_p_4).wrapping_add(156); c.m.w16(a_, (v_) as u16); };
                { let v_ = F32(0x3f800000); let a_ = (l_p_4).wrapping_add(180); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = (l_p_4).wrapping_add(184); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = (l_p_4).wrapping_add(164); c.m.w32(a_, (v_).0); };
                { let v_ = (c.m.r16((p_arg_2).wrapping_add(60)) as i16); let a_ = (l_p_4).wrapping_add(160); c.m.w16(a_, (v_) as u16); };
                l_spin_8 = rt::fmul(rt::i2f((((c.m.r16((p_arg_2).wrapping_add(66)) as i16)) as i32)), F32(0x3dcccccc));
                { let v_ = l_spin_8; let a_ = (l_p_4).wrapping_add(168); c.m.w32(a_, (v_).0); };
                { let v_ = (if (((rt::fgt(F::Rand_Float01(c), F32(0x3f000000)) as i32)) != 0) { rt::fneg(l_spin_8) } else { l_spin_8 }); let a_ = (l_p_4).wrapping_add(168); c.m.w32(a_, (v_).0); };
                { let a2_ = F32(c.m.r32((l_p_4).wrapping_add(172))); { let _ = D::call(c, fid::Vec3_Scale, &[(((l_p_4).wrapping_add(80)) as u64), (((l_p_4).wrapping_add(48)) as u64), ((a2_).0 as u64)]); } };
                { let v_ = 2i32; let a_ = (l_p_4).wrapping_add(188); c.m.w32(a_, (v_) as u32); };
                { let a_ = (c.m.r32(sym::gEftGndDust)).wrapping_add(25268); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustPuff_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDustPuff_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustPuff_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    F::eft_z__EftGndDust_FreeParts(c, l_w_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustPuff_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDustPuff_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustPuff_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustPuff_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDustPuff_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustPuff_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!((F::BtlScene_IsTimeStopped(c)) != 0)) as i32)) != 0) {
        F::EftGndDust_UpdateParts(c, l_w_2);
    }
    if (((((F::List_GetHead(c, (l_w_2).wrapping_add(204))) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a0_ = { let a_ = (c.m.r32(sym::gEftGndDust)).wrapping_add(25264); c.m.r32(a_) }; let a1_ = (((c.m.r8((l_w_2).wrapping_add(59)) as i8)) as i32); F::EftGndDust_GetTex(c, a0_, a1_) }; let a_ = (l_w_2).wrapping_add(184); c.m.w64(a_, v_); };
    } else {
        F::BtlTask_SetDead(c, p_task_1);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustPuff_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDustPuff_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustPuff_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_w_2: u32 = 0;
    let mut l_p_3: u32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_p_3 = F::List_GetHead(c, (l_w_2).wrapping_add(204));
        F::Vu0Cur_Push(c);
        { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
        'l1: while (((((l_p_3) != (((0i32) as u32))) as i32)) != 0) {
            { let a1_ = rt::fmul(F32(c.m.r32((l_p_3).wrapping_add(64))), F32(c.m.r32((l_w_2).wrapping_add(76)))); let a2_ = rt::fmul(F32(c.m.r32(((l_p_3).wrapping_add(64)).wrapping_add(4))), F32(c.m.r32((l_w_2).wrapping_add(76)))); let a3_ = rt::fmul(F32(c.m.r32(((l_p_3).wrapping_add(64)).wrapping_add(8))), F32(c.m.r32((l_w_2).wrapping_add(76)))); let a4_ = rt::fmul(F32(c.m.r32(((l_p_3).wrapping_add(64)).wrapping_add(12))), F32(c.m.r32((l_p_3).wrapping_add(180)))); { let _ = D::call(c, fid::Vec4_Set, &[((fp) as u64), ((a1_).0 as u64), ((a2_).0 as u64), ((a3_).0 as u64), ((a4_).0 as u64)]); } };
            { let _ = D::call(c, fid::Vec3_Add, &[((fp.wrapping_add(16)) as u64), (((l_p_3).wrapping_add(16)) as u64), (((l_p_3).wrapping_add(32)) as u64)]); };
            { let a2_ = rt::fmul(rt::fmul(rt::i2f(((c.m.r16((l_p_3).wrapping_add(160)) as i16)) as i32), F32(c.m.r32((l_w_2).wrapping_add(96)))), F32(c.m.r32((l_p_3).wrapping_add(96)))); let a3_ = rt::fmul(rt::fmul(rt::i2f(((c.m.r16((l_p_3).wrapping_add(160)) as i16)) as i32), F32(c.m.r32((l_w_2).wrapping_add(96)))), F32(c.m.r32(((l_p_3).wrapping_add(96)).wrapping_add(4)))); let a8_ = F32(c.m.r32((l_p_3).wrapping_add(164))); let a9_ = (((c.m.r8((l_w_2).wrapping_add(58)) as i8)) as i32); let a11_ = c.m.r64((l_w_2).wrapping_add(184)); F::EftGfx_DrawSprite(c, fp.wrapping_add(16), fp, a2_, a3_, F32(0x00000000), F32(0x00000000), F32(0x3f800000), F32(0x3f800000), a8_, a9_, 0i32, a11_) };
            l_p_3 = F::List_GetNext(c, l_p_3);
        }
        F::Vu0Cur_Pop(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustPuff_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDustSlide_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustSlide_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let _ = F::memset(c, l_w_3, 0i32, 224u32); };
    { let s_ = p_arg_2; let d_ = l_w_3; rt::memcpy(c, d_, s_, 112); };
    F::List_Init(c, (l_w_3).wrapping_add(204));
    { let v_ = { let a0_ = (((c.m.r8((p_arg_2).wrapping_add(57)) as i8)) as i32); F::EftGndDust_GetFreeList(c, a0_) }; let a_ = (l_w_3).wrapping_add(200); c.m.w32(a_, v_); };
    { let _ = F::EftGndDust_SpawnPiece(c, l_w_3, p_arg_2, 48i32, F32(0x3f800000), F32(0x00000000)); };
    { let a0_ = (((c.m.r8((p_arg_2).wrapping_add(56)) as i8)) as i32); F::BtlCharApi_GetPos(c, a0_, (l_w_3).wrapping_add(144)) };
    { let a0_ = (((c.m.r8((p_arg_2).wrapping_add(56)) as i8)) as i32); F::BtlCharApi_GetDir(c, a0_, (l_w_3).wrapping_add(160)) };
    { let _ = D::call(c, fid::Vec3_Scale, &[(((l_w_3).wrapping_add(160)) as u64), (((l_w_3).wrapping_add(160)) as u64), ((F32(0xbf800000)).0 as u64)]); };
    { let _ = D::call(c, fid::Vec3_Normalize, &[(((l_w_3).wrapping_add(160)) as u64), (((l_w_3).wrapping_add(160)) as u64)]); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustSlide_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDustSlide_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustSlide_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    F::eft_z__EftGndDust_FreeParts(c, l_w_2);
    { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gEftGndDust)).wrapping_add(25392)).wrapping_add(((((c.m.r8((l_w_2).wrapping_add(56)) as i8)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustSlide_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDustSlide_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustSlide_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustSlide_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDustSlide_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustSlide_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_period_2: i32 = 0;
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
        l_period_2 = 5i32;
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        if ((((!((F::BtlScene_IsTimeStopped(c)) != 0)) as i32)) != 0) {
            { let a0_ = (((c.m.r8((l_w_3).wrapping_add(56)) as i8)) as i32); F::BtlCharApi_GetPos(c, a0_, (l_w_3).wrapping_add(144)) };
            { let a0_ = (((c.m.r8((l_w_3).wrapping_add(56)) as i8)) as i32); F::BtlCharApi_GetDir(c, a0_, (l_w_3).wrapping_add(160)) };
            { let _ = D::call(c, fid::Vec3_Scale, &[(((l_w_3).wrapping_add(160)) as u64), (((l_w_3).wrapping_add(160)) as u64), ((F32(0xbf800000)).0 as u64)]); };
            { let _ = D::call(c, fid::Vec3_Normalize, &[(((l_w_3).wrapping_add(160)) as u64), (((l_w_3).wrapping_add(160)) as u64)]); };
            { let v_ = { let a0_ = (((c.m.r8((l_w_3).wrapping_add(56)) as i8)) as i32); F::BtlScene_GetCharScale(c, a0_) }; let a_ = (l_w_3).wrapping_add(96); c.m.w32(a_, (v_).0); };
            if (((rt::flt(F32(0x40133333), F32(c.m.r32((l_w_3).wrapping_add(96)))) as i32)) != 0) {
                { let v_ = F32(0x40133333); let a_ = (l_w_3).wrapping_add(96); c.m.w32(a_, (v_).0); };
            }
            if (((rt::flt(F32(c.m.r32((l_w_3).wrapping_add(96))), F32(0x3f4ccccc)) as i32)) != 0) {
                { let v_ = F32(0x3f4ccccc); let a_ = (l_w_3).wrapping_add(96); c.m.w32(a_, (v_).0); };
            }
            if (((((c.m.r32((l_w_3).wrapping_add(180)) as i32)) & (2i32))) != 0) {
                { let a_ = (l_w_3).wrapping_add(112); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                if (((((rt::rem_i32((c.m.r32((l_w_3).wrapping_add(112)) as i32), l_period_2)) == (0i32)) as i32)) != 0) {
                    { let _ = F::EftGndDust_SpawnPiece(c, l_w_3, l_w_3, 48i32, F32(0x3f800000), F32(0x00000000)); };
                    F::EftGndDust_SpawnChip(c, l_w_3, l_w_3, (l_w_3).wrapping_add(160), (l_w_3).wrapping_add(48), (l_w_3).wrapping_add(52), 5i32, 10i32);
                }
            }
            { let a0_ = (((c.m.r8((l_w_3).wrapping_add(56)) as i8)) as i32); F::BtlCharApi_GetFrameMove(c, a0_, fp) };
            { let _ = D::call(c, fid::Vec3_Normalize, &[((fp) as u64), ((fp) as u64)]); };
            if (((((((rt::flt(F::Vec3_Dot(c, (l_w_3).wrapping_add(128), fp), F32(0x00000000)) as i32)) != 0) || (((((c.m.r32((l_w_3).wrapping_add(180)) as i32)) & (4i32))) != 0)) as i32)) != 0) {
                { let r_ = 8i32; let a_ = (l_w_3).wrapping_add(180); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 16i32; let a_ = (l_w_3).wrapping_add(176); c.m.w32(a_, (v_) as u32); };
            } else {
                { let r_ = (!(8i32)); let a_ = (l_w_3).wrapping_add(180); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            }
            { let _ = D::call(c, fid::Vec4_Copy, &[(((l_w_3).wrapping_add(128)) as u64), ((fp) as u64)]); };
            F::EftGndDust_UpdateParts(c, l_w_3);
        }
        if (((((F::List_GetHead(c, (l_w_3).wrapping_add(204))) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = { let a0_ = { let a_ = (c.m.r32(sym::gEftGndDust)).wrapping_add(25264); c.m.r32(a_) }; F::EftGndDust_GetTex(c, a0_, 0i32) }; let a_ = (l_w_3).wrapping_add(184); c.m.w64(a_, v_); };
            { let v_ = { let a0_ = { let a_ = (c.m.r32(sym::gEftGndDust)).wrapping_add(25264); c.m.r32(a_) }; F::EftGndDust_GetTex(c, a0_, 2i32) }; let a_ = (l_w_3).wrapping_add(192); c.m.w64(a_, v_); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustSlide_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDustSlide_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustSlide_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_w_2: u32 = 0;
    let mut l_p_3: u32 = 0;
    let mut l_far_4: i32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_p_3 = F::List_GetHead(c, (l_w_2).wrapping_add(204));
        F::Vu0Cur_Push(c);
        { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
        'l1: while (((((l_p_3) != (((0i32) as u32))) as i32)) != 0) {
            l_far_4 = 0i32;
            if (((((c.m.r32((l_p_3).wrapping_add(188)) as i32)) & (64i32))) != 0) {
                l_far_4 = 1i32;
            }
            { let a1_ = rt::fmul(F32(c.m.r32((l_p_3).wrapping_add(64))), F32(c.m.r32((l_w_2).wrapping_add(76)))); let a2_ = rt::fmul(F32(c.m.r32(((l_p_3).wrapping_add(64)).wrapping_add(4))), F32(c.m.r32((l_w_2).wrapping_add(76)))); let a3_ = rt::fmul(F32(c.m.r32(((l_p_3).wrapping_add(64)).wrapping_add(8))), F32(c.m.r32((l_w_2).wrapping_add(76)))); let a4_ = rt::fmul(F32(c.m.r32(((l_p_3).wrapping_add(64)).wrapping_add(12))), F32(c.m.r32((l_p_3).wrapping_add(180)))); { let _ = D::call(c, fid::Vec4_Set, &[((fp) as u64), ((a1_).0 as u64), ((a2_).0 as u64), ((a3_).0 as u64), ((a4_).0 as u64)]); } };
            { let _ = D::call(c, fid::Vec3_Add, &[((fp.wrapping_add(16)) as u64), (((l_p_3).wrapping_add(16)) as u64), (((l_p_3).wrapping_add(32)) as u64)]); };
            { let a3_ = (((c.m.r8((l_w_2).wrapping_add(58)) as i8)) as i32); let a4_ = (if ((l_far_4) != 0) { c.m.r64((l_w_2).wrapping_add(192)) } else { c.m.r64((l_w_2).wrapping_add(184)) }); let a6_ = rt::fmul(rt::fmul(rt::i2f(((c.m.r16((l_p_3).wrapping_add(160)) as i16)) as i32), F32(c.m.r32((l_w_2).wrapping_add(96)))), F32(c.m.r32((l_p_3).wrapping_add(96)))); let a7_ = rt::fmul(rt::fmul(rt::i2f(((c.m.r16((l_p_3).wrapping_add(160)) as i16)) as i32), F32(c.m.r32((l_w_2).wrapping_add(96)))), F32(c.m.r32(((l_p_3).wrapping_add(96)).wrapping_add(4)))); let a8_ = F32(c.m.r32((l_p_3).wrapping_add(164))); F::EftGndDust_DrawPiece(c, fp.wrapping_add(16), fp, (l_p_3).wrapping_add(48), a3_, a4_, l_far_4, a6_, a7_, a8_) };
            l_p_3 = F::List_GetNext(c, l_p_3);
        }
        F::Vu0Cur_Pop(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustSlide_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDustDash_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustDash_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let _ = F::memset(c, l_w_3, 0i32, 224u32); };
    { let s_ = p_arg_2; let d_ = l_w_3; rt::memcpy(c, d_, s_, 112); };
    F::List_Init(c, (l_w_3).wrapping_add(204));
    { let v_ = { let a0_ = (((c.m.r8((p_arg_2).wrapping_add(57)) as i8)) as i32); F::EftGndDust_GetFreeList(c, a0_) }; let a_ = (l_w_3).wrapping_add(200); c.m.w32(a_, v_); };
    { let a0_ = (((c.m.r8((p_arg_2).wrapping_add(56)) as i8)) as i32); F::BtlCharApi_GetPos(c, a0_, (l_w_3).wrapping_add(144)) };
    { let a0_ = (((c.m.r8((p_arg_2).wrapping_add(56)) as i8)) as i32); F::BtlCharApi_GetDir(c, a0_, (l_w_3).wrapping_add(160)) };
    { let _ = D::call(c, fid::Vec3_Scale, &[(((l_w_3).wrapping_add(160)) as u64), (((l_w_3).wrapping_add(160)) as u64), ((F32(0xbf800000)).0 as u64)]); };
    { let _ = D::call(c, fid::Vec3_Normalize, &[(((l_w_3).wrapping_add(160)) as u64), (((l_w_3).wrapping_add(160)) as u64)]); };
    { let _ = D::call(c, fid::Vec4_Copy, &[(((p_arg_2).wrapping_add(16)) as u64), (((l_w_3).wrapping_add(160)) as u64)]); };
    { let _ = { let a4_ = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40490fda)); F::EftGndDust_SpawnPiece(c, l_w_3, p_arg_2, 16i32, F32(0x3f800000), a4_) }; };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustDash_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDustDash_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustDash_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    F::eft_z__EftGndDust_FreeParts(c, l_w_2);
    { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gEftGndDust)).wrapping_add(25400)).wrapping_add(((((c.m.r8((l_w_2).wrapping_add(56)) as i8)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustDash_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDustDash_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustDash_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustDash_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDustDash_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustDash_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_period_2: i32 = 0;
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
        l_period_2 = 6i32;
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        if ((((!((F::BtlScene_IsTimeStopped(c)) != 0)) as i32)) != 0) {
            { let a0_ = (((c.m.r8((l_w_3).wrapping_add(56)) as i8)) as i32); F::BtlCharApi_GetPos(c, a0_, (l_w_3).wrapping_add(144)) };
            { let a0_ = (((c.m.r8((l_w_3).wrapping_add(56)) as i8)) as i32); F::BtlCharApi_GetDir(c, a0_, (l_w_3).wrapping_add(160)) };
            { let _ = D::call(c, fid::Vec3_Scale, &[(((l_w_3).wrapping_add(160)) as u64), (((l_w_3).wrapping_add(160)) as u64), ((F32(0xbf800000)).0 as u64)]); };
            { let _ = D::call(c, fid::Vec3_Normalize, &[(((l_w_3).wrapping_add(160)) as u64), (((l_w_3).wrapping_add(160)) as u64)]); };
            { let v_ = { let a0_ = (((c.m.r8((l_w_3).wrapping_add(56)) as i8)) as i32); F::BtlScene_GetCharScale(c, a0_) }; let a_ = (l_w_3).wrapping_add(96); c.m.w32(a_, (v_).0); };
            if (((rt::flt(F32(0x40133333), F32(c.m.r32((l_w_3).wrapping_add(96)))) as i32)) != 0) {
                { let v_ = F32(0x40133333); let a_ = (l_w_3).wrapping_add(96); c.m.w32(a_, (v_).0); };
            }
            if (((rt::flt(F32(c.m.r32((l_w_3).wrapping_add(96))), F32(0x3f4ccccc)) as i32)) != 0) {
                { let v_ = F32(0x3f4ccccc); let a_ = (l_w_3).wrapping_add(96); c.m.w32(a_, (v_).0); };
            }
            if (((((c.m.r32((l_w_3).wrapping_add(180)) as i32)) & (2i32))) != 0) {
                { let a_ = (l_w_3).wrapping_add(112); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                if (((((rt::rem_i32((c.m.r32((l_w_3).wrapping_add(112)) as i32), l_period_2)) == (0i32)) as i32)) != 0) {
                    { let _ = D::call(c, fid::Vec4_Copy, &[(((l_w_3).wrapping_add(16)) as u64), (((l_w_3).wrapping_add(160)) as u64)]); };
                    { let _ = { let a4_ = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40490fda)); F::EftGndDust_SpawnPiece(c, l_w_3, l_w_3, 16i32, F32(0x3f800000), a4_) }; };
                }
            }
            { let a0_ = (((c.m.r8((l_w_3).wrapping_add(56)) as i8)) as i32); F::BtlCharApi_GetFrameMove(c, a0_, fp) };
            { let _ = D::call(c, fid::Vec3_Normalize, &[((fp) as u64), ((fp) as u64)]); };
            if (((rt::flt(F::Vec3_Dot(c, (l_w_3).wrapping_add(128), fp), F32(0x00000000)) as i32)) != 0) {
                { let v_ = 16i32; let a_ = (l_w_3).wrapping_add(176); c.m.w32(a_, (v_) as u32); };
                { let r_ = 8i32; let a_ = (l_w_3).wrapping_add(180); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            } else {
                { let r_ = (!(8i32)); let a_ = (l_w_3).wrapping_add(180); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            }
            { let _ = D::call(c, fid::Vec4_Copy, &[(((l_w_3).wrapping_add(128)) as u64), ((fp) as u64)]); };
            F::EftGndDust_UpdateParts(c, l_w_3);
        }
        if (((((F::List_GetHead(c, (l_w_3).wrapping_add(204))) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = { let a0_ = { let a_ = (c.m.r32(sym::gEftGndDust)).wrapping_add(25264); c.m.r32(a_) }; let a1_ = (((c.m.r8((l_w_3).wrapping_add(59)) as i8)) as i32); F::EftGndDust_GetTex(c, a0_, a1_) }; let a_ = (l_w_3).wrapping_add(184); c.m.w64(a_, v_); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustDash_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDustDash_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustDash_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_w_2: u32 = 0;
    let mut l_p_3: u32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_p_3 = F::List_GetHead(c, (l_w_2).wrapping_add(204));
        F::Vu0Cur_Push(c);
        { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
        'l1: while (((((l_p_3) != (((0i32) as u32))) as i32)) != 0) {
            { let a1_ = rt::fmul(F32(c.m.r32((l_p_3).wrapping_add(64))), F32(c.m.r32((l_w_2).wrapping_add(76)))); let a2_ = rt::fmul(F32(c.m.r32(((l_p_3).wrapping_add(64)).wrapping_add(4))), F32(c.m.r32((l_w_2).wrapping_add(76)))); let a3_ = rt::fmul(F32(c.m.r32(((l_p_3).wrapping_add(64)).wrapping_add(8))), F32(c.m.r32((l_w_2).wrapping_add(76)))); let a4_ = rt::fmul(F32(c.m.r32(((l_p_3).wrapping_add(64)).wrapping_add(12))), F32(c.m.r32((l_p_3).wrapping_add(180)))); { let _ = D::call(c, fid::Vec4_Set, &[((fp) as u64), ((a1_).0 as u64), ((a2_).0 as u64), ((a3_).0 as u64), ((a4_).0 as u64)]); } };
            { let _ = D::call(c, fid::Vec3_Add, &[((fp.wrapping_add(16)) as u64), (((l_p_3).wrapping_add(16)) as u64), (((l_p_3).wrapping_add(32)) as u64)]); };
            { let a2_ = rt::fmul(rt::fmul(rt::i2f(((c.m.r16((l_p_3).wrapping_add(160)) as i16)) as i32), F32(c.m.r32((l_w_2).wrapping_add(96)))), F32(c.m.r32((l_p_3).wrapping_add(96)))); let a3_ = rt::fmul(rt::fmul(rt::i2f(((c.m.r16((l_p_3).wrapping_add(160)) as i16)) as i32), F32(c.m.r32((l_w_2).wrapping_add(96)))), F32(c.m.r32(((l_p_3).wrapping_add(96)).wrapping_add(4)))); let a8_ = F32(c.m.r32((l_p_3).wrapping_add(164))); let a9_ = (((c.m.r8((l_w_2).wrapping_add(58)) as i8)) as i32); let a11_ = c.m.r64((l_w_2).wrapping_add(184)); F::EftGfx_DrawSprite(c, fp.wrapping_add(16), fp, a2_, a3_, F32(0x00000000), F32(0x00000000), F32(0x3f800000), F32(0x3f800000), a8_, a9_, 0i32, a11_) };
            l_p_3 = F::List_GetNext(c, l_p_3);
        }
        F::Vu0Cur_Pop(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustDash_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDustBurst_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustBurst_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let _ = F::memset(c, l_w_3, 0i32, 224u32); };
    { let s_ = p_arg_2; let d_ = l_w_3; rt::memcpy(c, d_, s_, 112); };
    F::List_Init(c, (l_w_3).wrapping_add(204));
    { let v_ = { let a0_ = (((c.m.r8((p_arg_2).wrapping_add(57)) as i8)) as i32); F::EftGndDust_GetFreeList(c, a0_) }; let a_ = (l_w_3).wrapping_add(200); c.m.w32(a_, v_); };
    { let _ = F32(0x3f800000); F::EftGndDust_SpawnBodyDust(c, l_w_3, l_w_3, F32(0x3f800000)) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustBurst_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDustBurst_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustBurst_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    F::eft_z__EftGndDust_FreeParts(c, l_w_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustBurst_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDustBurst_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustBurst_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustBurst_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDustBurst_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustBurst_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!((F::BtlScene_IsTimeStopped(c)) != 0)) as i32)) != 0) {
        { let a0_ = (((c.m.r8((l_w_2).wrapping_add(56)) as i8)) as i32); F::BtlCharApi_GetPos(c, a0_, (l_w_2).wrapping_add(144)) };
        { let a0_ = (((c.m.r8((l_w_2).wrapping_add(56)) as i8)) as i32); F::BtlCharApi_GetDir(c, a0_, (l_w_2).wrapping_add(160)) };
        { let _ = D::call(c, fid::Vec3_Scale, &[(((l_w_2).wrapping_add(160)) as u64), (((l_w_2).wrapping_add(160)) as u64), ((F32(0xbf800000)).0 as u64)]); };
        { let _ = D::call(c, fid::Vec3_Normalize, &[(((l_w_2).wrapping_add(160)) as u64), (((l_w_2).wrapping_add(160)) as u64)]); };
        F::EftGndDust_UpdateParts(c, l_w_2);
    }
    if (((((F::List_GetHead(c, (l_w_2).wrapping_add(204))) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a0_ = { let a_ = (c.m.r32(sym::gEftGndDust)).wrapping_add(25264); c.m.r32(a_) }; let a1_ = (((c.m.r8((l_w_2).wrapping_add(59)) as i8)) as i32); F::EftGndDust_GetTex(c, a0_, a1_) }; let a_ = (l_w_2).wrapping_add(184); c.m.w64(a_, v_); };
    } else {
        F::BtlTask_SetDead(c, p_task_1);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustBurst_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDustBurst_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustBurst_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_w_2: u32 = 0;
    let mut l_p_3: u32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        { let _ = F::memset(c, fp.wrapping_add(32), 0i32, 16u32); };
        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        l_p_3 = F::List_GetHead(c, (l_w_2).wrapping_add(204));
        F::Vu0Cur_Push(c);
        { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
        'l1: while (((((l_p_3) != (((0i32) as u32))) as i32)) != 0) {
            { let a1_ = rt::fmul(F32(c.m.r32((l_p_3).wrapping_add(64))), F32(c.m.r32((l_w_2).wrapping_add(76)))); let a2_ = rt::fmul(F32(c.m.r32(((l_p_3).wrapping_add(64)).wrapping_add(4))), F32(c.m.r32((l_w_2).wrapping_add(76)))); let a3_ = rt::fmul(F32(c.m.r32(((l_p_3).wrapping_add(64)).wrapping_add(8))), F32(c.m.r32((l_w_2).wrapping_add(76)))); let a4_ = rt::fmul(F32(c.m.r32(((l_p_3).wrapping_add(64)).wrapping_add(12))), F32(c.m.r32((l_p_3).wrapping_add(180)))); { let _ = D::call(c, fid::Vec4_Set, &[((fp) as u64), ((a1_).0 as u64), ((a2_).0 as u64), ((a3_).0 as u64), ((a4_).0 as u64)]); } };
            { let _ = D::call(c, fid::Vec3_Add, &[((fp.wrapping_add(16)) as u64), (((l_p_3).wrapping_add(16)) as u64), (((l_p_3).wrapping_add(32)) as u64)]); };
            { let a3_ = (((c.m.r8((l_w_2).wrapping_add(58)) as i8)) as i32); let a4_ = c.m.r64((l_w_2).wrapping_add(184)); let a5_ = ((((c.m.r32((l_p_3).wrapping_add(188)) as i32)).wrapping_shr((6i32) as u32)) & (1i32)); let a6_ = rt::fmul(rt::fmul(rt::i2f(((c.m.r16((l_p_3).wrapping_add(160)) as i16)) as i32), F32(c.m.r32((l_w_2).wrapping_add(96)))), F32(c.m.r32((l_p_3).wrapping_add(96)))); let a7_ = rt::fmul(rt::fmul(rt::i2f(((c.m.r16((l_p_3).wrapping_add(160)) as i16)) as i32), F32(c.m.r32((l_w_2).wrapping_add(96)))), F32(c.m.r32(((l_p_3).wrapping_add(96)).wrapping_add(4)))); let a8_ = F32(c.m.r32((l_p_3).wrapping_add(164))); F::EftGndDust_DrawPiece(c, fp.wrapping_add(16), fp, fp.wrapping_add(32), a3_, a4_, a5_, a6_, a7_, a8_) };
            l_p_3 = F::List_GetNext(c, l_p_3);
        }
        F::Vu0Cur_Pop(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustBurst_Draw", &PROBE_N, &[], true); }
    r_
}
