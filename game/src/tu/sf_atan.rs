#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Port_atanf(c: &mut Ctx, mut p_x_1: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Port_atanf", &PROBE_N, &[(p_x_1).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_2: F32 = F32(0);
    let mut l_s1_3: F32 = F32(0);
    let mut l_s2_4: F32 = F32(0);
    let mut l_z_5: F32 = F32(0);
    let mut l_ix_6: i32 = 0;
    let mut l_hx_7: i32 = 0;
    let mut l_id_8: i32 = 0;
    let r_ = 'f: {
        'l1: loop {
            'c2: {
                { let v_ = p_x_1; let a_ = fp; c.m.w32(a_, (v_).0); };
                l_hx_7 = ((c.m.r32(fp)) as i32);
            }
            if !((0i32) != 0) { break 'l1; }
        }
        l_ix_6 = ((l_hx_7) & (2147483647i32));
        if (((((l_ix_6) >= (1350565888i32)) as i32)) != 0) {
            if (((((l_ix_6) > (2139095040i32)) as i32)) != 0) {
                break 'f rt::fadd(p_x_1, p_x_1);
            }
            if (((((l_hx_7) > (0i32)) as i32)) != 0) {
                break 'f F32(0x3fc90fdb);
            } else {
                break 'f F32(0xbfc90fdb);
            }
        }
        if (((((l_ix_6) < (1054867456i32)) as i32)) != 0) {
            if (((((l_ix_6) < (822083584i32)) as i32)) != 0) {
                if (((rt::fgt(rt::fadd(F32(0x7149f2c9), p_x_1), F32(0x3f800000)) as i32)) != 0) {
                    break 'f p_x_1;
                }
            }
            l_id_8 = (1i32).wrapping_neg();
        } else {
            p_x_1 = F::Port_fabsf(c, p_x_1);
            if (((((l_ix_6) < (1066926080i32)) as i32)) != 0) {
                if (((((l_ix_6) < (1060110336i32)) as i32)) != 0) {
                    l_id_8 = 0i32;
                    p_x_1 = rt::fdiv(rt::fsub(rt::fmul(F32(0x40000000), p_x_1), F32(0x3f800000)), rt::fadd(F32(0x40000000), p_x_1));
                } else {
                    l_id_8 = 1i32;
                    p_x_1 = rt::fdiv(rt::fsub(p_x_1, F32(0x3f800000)), rt::fadd(p_x_1, F32(0x3f800000)));
                }
            } else {
                if (((((l_ix_6) < (1075576832i32)) as i32)) != 0) {
                    l_id_8 = 2i32;
                    p_x_1 = rt::fdiv(rt::fsub(p_x_1, F32(0x3fc00000)), rt::fadd(F32(0x3f800000), rt::fmul(F32(0x3fc00000), p_x_1)));
                } else {
                    l_id_8 = 3i32;
                    p_x_1 = rt::fdiv(F32(0xbf800000), p_x_1);
                }
            }
        }
        l_z_5 = rt::fmul(p_x_1, p_x_1);
        l_w_2 = rt::fmul(l_z_5, l_z_5);
        l_s1_3 = rt::fmul(l_z_5, rt::fadd(F32(0x3eaaaaab), rt::fmul(l_w_2, rt::fadd(F32(0x3e124924), rt::fmul(l_w_2, rt::fadd(F32(0x3dba2e6e), rt::fmul(l_w_2, rt::fadd(F32(0x3d886b34), rt::fmul(l_w_2, rt::fadd(F32(0x3d4bda59), rt::fmul(l_w_2, F32(0x3c8569d7))))))))))));
        l_s2_4 = rt::fmul(l_w_2, rt::fadd(F32(0xbe4ccccc), rt::fmul(l_w_2, rt::fadd(F32(0xbde38e38), rt::fmul(l_w_2, rt::fadd(F32(0xbd9d8795), rt::fmul(l_w_2, rt::fadd(F32(0xbd6ef16a), rt::fmul(l_w_2, F32(0xbd15a221))))))))));
        if (((((l_id_8) < (0i32)) as i32)) != 0) {
            break 'f rt::fsub(p_x_1, rt::fmul(p_x_1, rt::fadd(l_s1_3, l_s2_4)));
        } else {
            l_z_5 = rt::fsub(F32(c.m.r32((sym::sf_atan__atanhi).wrapping_add((((l_id_8) as u32)).wrapping_mul(4)))), rt::fsub(rt::fsub(rt::fmul(p_x_1, rt::fadd(l_s1_3, l_s2_4)), F32(c.m.r32((sym::sf_atan__atanlo).wrapping_add((((l_id_8) as u32)).wrapping_mul(4))))), p_x_1));
            break 'f (if (((((l_hx_7) < (0i32)) as i32)) != 0) { rt::fneg(l_z_5) } else { l_z_5 });
        }
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Port_atanf", &PROBE_N, &[], true); }
    r_
}
