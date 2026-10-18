#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Port_scalbnf(c: &mut Ctx, mut p_x_1: F32, mut p_n_2: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Port_scalbnf", &PROBE_N, &[(p_x_1).0, (p_n_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_k_3: i32 = 0;
    let mut l_ix_4: i32 = 0;
    let mut l_hx_5: u32 = 0;
    let r_ = 'f: {
        'l1: loop {
            'c2: {
                { let v_ = p_x_1; let a_ = fp; c.m.w32(a_, (v_).0); };
                l_ix_4 = ((c.m.r32(fp)) as i32);
            }
            if !((0i32) != 0) { break 'l1; }
        }
        l_hx_5 = ((((l_ix_4) & (2147483647i32))) as u32);
        l_k_3 = (((l_hx_5).wrapping_shr((23i32) as u32)) as i32);
        if (((((l_hx_5) == (((0i32) as u32))) as i32)) != 0) {
            break 'f p_x_1;
        }
        if ((((!(((((l_hx_5) < (((2139095040i32) as u32))) as i32)) != 0)) as i32)) != 0) {
            break 'f rt::fadd(p_x_1, p_x_1);
        }
        if (((((l_hx_5) < (((8388608i32) as u32))) as i32)) != 0) {
            { let r_ = F32(0x4c000000); let n_ = rt::fmul(p_x_1, r_); p_x_1 = n_; };
            'l3: loop {
                'c4: {
                    { let v_ = p_x_1; let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_).0); };
                    l_ix_4 = ((c.m.r32(fp.wrapping_add(4))) as i32);
                }
                if !((0i32) != 0) { break 'l3; }
            }
            l_k_3 = ((((l_ix_4) & (2139095040i32))).wrapping_shr((23i32) as u32)).wrapping_sub(25i32);
            if (((((p_n_2) < ((50000i32).wrapping_neg())) as i32)) != 0) {
                break 'f rt::fmul(F32(0x0da2425f), p_x_1);
            }
        }
        l_k_3 = (l_k_3).wrapping_add(p_n_2);
        if (((((l_k_3) > ((2139095039i32).wrapping_shr((23i32) as u32))) as i32)) != 0) {
            break 'f rt::fmul(F32(0x7149f2c9), F::Port_copysignf(c, F32(0x7149f2c9), p_x_1));
        }
        if (((((l_k_3) > (0i32)) as i32)) != 0) {
            'l5: loop {
                'c6: {
                    { let v_ = ((((((l_ix_4) as u32)) & (2155872255u32))) | ((((l_k_3).wrapping_shl((23i32) as u32)) as u32))); let a_ = fp.wrapping_add(8); c.m.w32(a_, v_); };
                    p_x_1 = F32(c.m.r32(fp.wrapping_add(8)));
                }
                if !((0i32) != 0) { break 'l5; }
            }
            break 'f p_x_1;
        }
        if (((((l_k_3) < ((22i32).wrapping_neg())) as i32)) != 0) {
            if (((((p_n_2) > (50000i32)) as i32)) != 0) {
                break 'f rt::fmul(F32(0x7149f2c9), F::Port_copysignf(c, F32(0x7149f2c9), p_x_1));
            } else {
                break 'f rt::fmul(F32(0x0da2425f), F::Port_copysignf(c, F32(0x0da2425f), p_x_1));
            }
        }
        { let r_ = 25i32; let n_ = (l_k_3).wrapping_add(r_); l_k_3 = n_; };
        'l7: loop {
            'c8: {
                { let v_ = ((((((l_ix_4) as u32)) & (2155872255u32))) | ((((l_k_3).wrapping_shl((23i32) as u32)) as u32))); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
                p_x_1 = F32(c.m.r32(fp.wrapping_add(12)));
            }
            if !((0i32) != 0) { break 'l7; }
        }
        break 'f rt::fmul(p_x_1, F32(0x33000000));
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Port_scalbnf", &PROBE_N, &[], true); }
    r_
}
