#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn __ieee754_sqrtf(c: &mut Ctx, mut p_x_1: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "__ieee754_sqrtf", &PROBE_N, &[(p_x_1).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_z_2: F32 = F32(0);
    let mut l_sign_3: i32 = 0;
    let mut l_r_4: u32 = 0;
    let mut l_hx_5: u32 = 0;
    let mut l_ix_6: i32 = 0;
    let mut l_s_7: i32 = 0;
    let mut l_q_8: i32 = 0;
    let mut l_m_9: i32 = 0;
    let mut l_t_10: i32 = 0;
    let mut l_i_11: i32 = 0;
    let r_ = 'f: {
        l_sign_3 = ((2147483648u32) as i32);
        'l1: loop {
            'c2: {
                { let v_ = p_x_1; let a_ = fp; c.m.w32(a_, (v_).0); };
                l_ix_6 = ((c.m.r32(fp)) as i32);
            }
            if !((0i32) != 0) { break 'l1; }
        }
        l_hx_5 = ((((l_ix_6) & (2147483647i32))) as u32);
        if ((((!(((((l_hx_5) < (((2139095040i32) as u32))) as i32)) != 0)) as i32)) != 0) {
            break 'f rt::fadd(rt::fmul(p_x_1, p_x_1), p_x_1);
        }
        if (((((l_hx_5) == (((0i32) as u32))) as i32)) != 0) {
            break 'f p_x_1;
        }
        if (((((l_ix_6) < (0i32)) as i32)) != 0) {
            break 'f rt::fdiv(rt::fsub(p_x_1, p_x_1), rt::fsub(p_x_1, p_x_1));
        }
        l_m_9 = (l_ix_6).wrapping_shr((23i32) as u32);
        if (((((l_hx_5) < (((8388608i32) as u32))) as i32)) != 0) {
            {
                l_i_11 = 0i32;
                'l3: loop {
                    if !(((((((l_ix_6) & (8388608i32))) == (0i32)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        { let r_ = 1i32; let n_ = (l_ix_6).wrapping_shl((r_) as u32); l_ix_6 = n_; };
                    }
                    { let o_ = l_i_11; let n_ = (o_).wrapping_add(1); l_i_11 = n_; };
                }
            }
            { let r_ = (l_i_11).wrapping_sub(1i32); let n_ = (l_m_9).wrapping_sub(r_); l_m_9 = n_; };
        }
        { let r_ = 127i32; let n_ = (l_m_9).wrapping_sub(r_); l_m_9 = n_; };
        l_ix_6 = ((((l_ix_6) & (8388607i32))) | (8388608i32));
        if ((((l_m_9) & (1i32))) != 0) {
            { let r_ = l_ix_6; let n_ = (l_ix_6).wrapping_add(r_); l_ix_6 = n_; };
        }
        { let r_ = 1i32; let n_ = (l_m_9).wrapping_shr((r_) as u32); l_m_9 = n_; };
        { let r_ = l_ix_6; let n_ = (l_ix_6).wrapping_add(r_); l_ix_6 = n_; };
        l_q_8 = { l_s_7 = 0i32; l_s_7 };
        l_r_4 = ((16777216i32) as u32);
        'l5: while (((((l_r_4) != (((0i32) as u32))) as i32)) != 0) {
            l_t_10 = (((((l_s_7) as u32)).wrapping_add(l_r_4)) as i32);
            if (((((l_t_10) <= (l_ix_6)) as i32)) != 0) {
                l_s_7 = (((((l_t_10) as u32)).wrapping_add(l_r_4)) as i32);
                { let r_ = l_t_10; let n_ = (l_ix_6).wrapping_sub(r_); l_ix_6 = n_; };
                { let r_ = l_r_4; let n_ = (((((l_q_8) as u32)).wrapping_add(r_)) as i32); l_q_8 = n_; };
            }
            { let r_ = l_ix_6; let n_ = (l_ix_6).wrapping_add(r_); l_ix_6 = n_; };
            { let r_ = 1i32; let n_ = (l_r_4).wrapping_shr((r_) as u32); l_r_4 = n_; };
        }
        if (((((l_ix_6) != (0i32)) as i32)) != 0) {
            l_z_2 = F32(0x3f800000);
            if (((rt::fge(l_z_2, F32(0x3f800000)) as i32)) != 0) {
                l_z_2 = F32(0x3f800000);
                if (((rt::fgt(l_z_2, F32(0x3f800000)) as i32)) != 0) {
                    { let r_ = 2i32; let n_ = (l_q_8).wrapping_add(r_); l_q_8 = n_; };
                } else {
                    { let r_ = ((l_q_8) & (1i32)); let n_ = (l_q_8).wrapping_add(r_); l_q_8 = n_; };
                }
            }
        }
        l_ix_6 = ((l_q_8).wrapping_shr((1i32) as u32)).wrapping_add(1056964608i32);
        { let r_ = (l_m_9).wrapping_shl((23i32) as u32); let n_ = (l_ix_6).wrapping_add(r_); l_ix_6 = n_; };
        'l6: loop {
            'c7: {
                { let v_ = ((l_ix_6) as u32); let a_ = fp.wrapping_add(4); c.m.w32(a_, v_); };
                l_z_2 = F32(c.m.r32(fp.wrapping_add(4)));
            }
            if !((0i32) != 0) { break 'l6; }
        }
        break 'f l_z_2;
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "__ieee754_sqrtf", &PROBE_N, &[], true); }
    r_
}
