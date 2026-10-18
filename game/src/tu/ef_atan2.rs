#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn __ieee754_atan2f(c: &mut Ctx, mut p_y_1: F32, mut p_x_2: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "__ieee754_atan2f", &PROBE_N, &[(p_y_1).0, (p_x_2).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_z_3: F32 = F32(0);
    let mut l_k_4: i32 = 0;
    let mut l_m_5: i32 = 0;
    let mut l_hx_6: i32 = 0;
    let mut l_hy_7: i32 = 0;
    let mut l_ix_8: i32 = 0;
    let mut l_iy_9: i32 = 0;
    let mut l_zh_10: u32 = 0;
    let r_ = 'f: {
        'l1: loop {
            'c2: {
                { let v_ = p_x_2; let a_ = fp; c.m.w32(a_, (v_).0); };
                l_hx_6 = ((c.m.r32(fp)) as i32);
            }
            if !((0i32) != 0) { break 'l1; }
        }
        l_ix_8 = ((l_hx_6) & (2147483647i32));
        'l3: loop {
            'c4: {
                { let v_ = p_y_1; let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_).0); };
                l_hy_7 = ((c.m.r32(fp.wrapping_add(4))) as i32);
            }
            if !((0i32) != 0) { break 'l3; }
        }
        l_iy_9 = ((l_hy_7) & (2147483647i32));
        if (((((((((l_ix_8) > (2139095040i32)) as i32)) != 0) || (((((l_iy_9) > (2139095040i32)) as i32)) != 0)) as i32)) != 0) {
            break 'f rt::fadd(p_x_2, p_y_1);
        }
        if (((((l_hx_6) == (1065353216i32)) as i32)) != 0) {
            break 'f F::Port_atanf(c, p_y_1);
        }
        l_m_5 = (((((l_hy_7).wrapping_shr((31i32) as u32)) & (1i32))) | ((((l_hx_6).wrapping_shr((30i32) as u32)) & (2i32))));
        if (((((l_iy_9) == (0i32)) as i32)) != 0) {
            's5: {
                let sv11: i32 = l_m_5;
                match sv11 {
                    0i32 | 1i32 => {
                        break 'f p_y_1;
                    }
                    2i32 => {
                        break 'f F32(0x40490fda);
                    }
                    3i32 => {
                        break 'f F32(0xc0490fda);
                    }
                    _ => {
                    }
                }
            }
        }
        if (((((l_ix_8) == (0i32)) as i32)) != 0) {
            break 'f (if (((((l_hy_7) < (0i32)) as i32)) != 0) { F32(0xbfc90fda) } else { F32(0x3fc90fda) });
        }
        if (((((l_ix_8) == (2139095040i32)) as i32)) != 0) {
            if (((((l_iy_9) == (2139095040i32)) as i32)) != 0) {
                's6: {
                    let sv12: i32 = l_m_5;
                    match sv12 {
                        0i32 => {
                            break 'f F32(0x3f490fda);
                        }
                        1i32 => {
                            break 'f F32(0xbf490fda);
                        }
                        2i32 => {
                            break 'f F32(0x4016cbe4);
                        }
                        3i32 => {
                            break 'f F32(0xc016cbe4);
                        }
                        _ => {
                        }
                    }
                }
            } else {
                's7: {
                    let sv13: i32 = l_m_5;
                    match sv13 {
                        0i32 => {
                            break 'f F32(0x00000000);
                        }
                        1i32 => {
                            break 'f F32(0x80000000);
                        }
                        2i32 => {
                            break 'f F32(0x40490fda);
                        }
                        3i32 => {
                            break 'f F32(0xc0490fda);
                        }
                        _ => {
                        }
                    }
                }
            }
        }
        if (((((l_iy_9) == (2139095040i32)) as i32)) != 0) {
            break 'f (if (((((l_hy_7) < (0i32)) as i32)) != 0) { F32(0xbfc90fda) } else { F32(0x3fc90fda) });
        }
        l_k_4 = ((l_iy_9).wrapping_sub(l_ix_8)).wrapping_shr((23i32) as u32);
        if (((((l_k_4) > (60i32)) as i32)) != 0) {
            l_z_3 = F32(0x3fc90fdb);
        } else {
            if (((((((((l_hx_6) < (0i32)) as i32)) != 0) && (((((l_k_4) < ((60i32).wrapping_neg())) as i32)) != 0)) as i32)) != 0) {
                l_z_3 = F32(0x00000000);
            } else {
                l_z_3 = { let a0_ = F::Port_fabsf(c, rt::fdiv(p_y_1, p_x_2)); F::Port_atanf(c, a0_) };
            }
        }
        's8: {
            let sv14: i32 = l_m_5;
            match sv14 {
                0i32 => {
                    break 'f l_z_3;
                }
                1i32 => {
                    {
                        'l9: loop {
                            'c10: {
                                { let v_ = l_z_3; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_).0); };
                                l_zh_10 = c.m.r32(fp.wrapping_add(8));
                            }
                            if !((0i32) != 0) { break 'l9; }
                        }
                        'l11: loop {
                            'c12: {
                                { let v_ = ((l_zh_10) ^ (2147483648u32)); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
                                l_z_3 = F32(c.m.r32(fp.wrapping_add(12)));
                            }
                            if !((0i32) != 0) { break 'l11; }
                        }
                    }
                    break 'f l_z_3;
                }
                2i32 => {
                    break 'f rt::fsub(F32(0x40490fda), rt::fsub(l_z_3, F32(0x34222168)));
                }
                _ => {
                    break 'f rt::fsub(rt::fsub(l_z_3, F32(0x34222168)), F32(0x40490fda));
                }
            }
        }
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "__ieee754_atan2f", &PROBE_N, &[], true); }
    r_
}
