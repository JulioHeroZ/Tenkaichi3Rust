#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn __kernel_rem_pio2f(c: &mut Ctx, mut p_x_1: u32, mut p_y_2: u32, mut p_e0_3: i32, mut p_nx_4: i32, mut p_prec_5: i32, mut p_ipio2_6: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "__kernel_rem_pio2f", &PROBE_N, &[(p_x_1) as u32, (p_y_2) as u32, (p_e0_3) as u32, (p_nx_4) as u32, (p_prec_5) as u32, (p_ipio2_6) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(320) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 320);
    let mut l_jz_7: i32 = 0;
    let mut l_jx_8: i32 = 0;
    let mut l_jv_9: i32 = 0;
    let mut l_jp_10: i32 = 0;
    let mut l_jk_11: i32 = 0;
    let mut l_carry_12: i32 = 0;
    let mut l_n_13: i32 = 0;
    let mut l_i_14: i32 = 0;
    let mut l_j_15: i32 = 0;
    let mut l_k_16: i32 = 0;
    let mut l_m_17: i32 = 0;
    let mut l_q0_18: i32 = 0;
    let mut l_ih_19: i32 = 0;
    let mut l_z_20: F32 = F32(0);
    let mut l_fw_21: F32 = F32(0);
    let mut sv_22: i32 = 0;
    let mut sv_23: i32 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
        'fl1: loop {
            match bb_ {
                0 => {
                    l_jk_11 = (c.m.r32((sym::kf_rem_pio2__init_jk).wrapping_add((((p_prec_5) as u32)).wrapping_mul(4))) as i32);
                    l_jp_10 = l_jk_11;
                    l_jx_8 = (p_nx_4).wrapping_sub(1i32);
                    l_jv_9 = rt::div_i32((p_e0_3).wrapping_sub(3i32), 8i32);
                    if (((((l_jv_9) < (0i32)) as i32)) != 0) { bb_ = 1; continue 'fl1; } else { bb_ = 2; continue 'fl1; }
                }
                1 => {
                    l_jv_9 = 0i32;
                    bb_ = 2; continue 'fl1;
                }
                2 => {
                    l_q0_18 = (p_e0_3).wrapping_sub((8i32).wrapping_mul((l_jv_9).wrapping_add(1i32)));
                    l_j_15 = (l_jv_9).wrapping_sub(l_jx_8);
                    l_m_17 = (l_jx_8).wrapping_add(l_jk_11);
                    l_i_14 = 0i32;
                    bb_ = 3; continue 'fl1;
                }
                3 => {
                    if (((((l_i_14) <= (l_m_17)) as i32)) != 0) { bb_ = 4; continue 'fl1; } else { bb_ = 6; continue 'fl1; }
                }
                4 => {
                    { let v_ = (if (((((l_j_15) < (0i32)) as i32)) != 0) { F32(0x00000000) } else { rt::i2f((c.m.r32((p_ipio2_6).wrapping_add((((l_j_15) as u32)).wrapping_mul(4))) as i32)) }); let a_ = (fp.wrapping_add(80)).wrapping_add((((l_i_14) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    bb_ = 5; continue 'fl1;
                }
                5 => {
                    { { let o_ = l_i_14; let n_ = (o_).wrapping_add(1); l_i_14 = n_; }; { let o_ = l_j_15; let n_ = (o_).wrapping_add(1); l_j_15 = n_; }; };
                    bb_ = 3; continue 'fl1;
                }
                6 => {
                    l_i_14 = 0i32;
                    bb_ = 7; continue 'fl1;
                }
                7 => {
                    if (((((l_i_14) <= (l_jk_11)) as i32)) != 0) { bb_ = 8; continue 'fl1; } else { bb_ = 10; continue 'fl1; }
                }
                8 => {
                    { l_j_15 = 0i32; l_fw_21 = F32(0x00000000); };
                    bb_ = 11; continue 'fl1;
                }
                9 => {
                    { let o_ = l_i_14; let n_ = (o_).wrapping_add(1); l_i_14 = n_; };
                    bb_ = 7; continue 'fl1;
                }
                10 => {
                    l_jz_7 = l_jk_11;
                    bb_ = 15; continue 'fl1;
                }
                11 => {
                    if (((((l_j_15) <= (l_jx_8)) as i32)) != 0) { bb_ = 12; continue 'fl1; } else { bb_ = 14; continue 'fl1; }
                }
                12 => {
                    { let r_ = rt::fmul(F32(c.m.r32((p_x_1).wrapping_add((((l_j_15) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(80)).wrapping_add((((((l_jx_8).wrapping_add(l_i_14)).wrapping_sub(l_j_15)) as u32)).wrapping_mul(4))))); let n_ = rt::fadd(l_fw_21, r_); l_fw_21 = n_; };
                    bb_ = 13; continue 'fl1;
                }
                13 => {
                    { let o_ = l_j_15; let n_ = (o_).wrapping_add(1); l_j_15 = n_; };
                    bb_ = 11; continue 'fl1;
                }
                14 => {
                    { let v_ = l_fw_21; let a_ = (fp.wrapping_add(240)).wrapping_add((((l_i_14) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    bb_ = 9; continue 'fl1;
                }
                15 => {
                    { { l_i_14 = 0i32; l_j_15 = l_jz_7; }; l_z_20 = F32(c.m.r32((fp.wrapping_add(240)).wrapping_add((((l_jz_7) as u32)).wrapping_mul(4)))); };
                    bb_ = 16; continue 'fl1;
                }
                16 => {
                    if (((((l_j_15) > (0i32)) as i32)) != 0) { bb_ = 17; continue 'fl1; } else { bb_ = 19; continue 'fl1; }
                }
                17 => {
                    l_fw_21 = rt::i2f(rt::f2i(rt::fmul(F32(0x3b800000), l_z_20)));
                    { let v_ = rt::f2i(rt::fsub(l_z_20, rt::fmul(F32(0x43800000), l_fw_21))); let a_ = (fp).wrapping_add((((l_i_14) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_z_20 = rt::fadd(F32(c.m.r32((fp.wrapping_add(240)).wrapping_add(((((l_j_15).wrapping_sub(1i32)) as u32)).wrapping_mul(4)))), l_fw_21);
                    bb_ = 18; continue 'fl1;
                }
                18 => {
                    { { let o_ = l_i_14; let n_ = (o_).wrapping_add(1); l_i_14 = n_; }; { let o_ = l_j_15; let n_ = (o_).wrapping_sub(1); l_j_15 = n_; }; };
                    bb_ = 16; continue 'fl1;
                }
                19 => {
                    l_z_20 = F::Port_scalbnf(c, l_z_20, l_q0_18);
                    { let r_ = rt::fmul(F32(0x41000000), F::Port_floorf(c, rt::fmul(l_z_20, F32(0x3e000000)))); let n_ = rt::fsub(l_z_20, r_); l_z_20 = n_; };
                    l_n_13 = rt::f2i(l_z_20);
                    { let r_ = rt::i2f(l_n_13); let n_ = rt::fsub(l_z_20, r_); l_z_20 = n_; };
                    l_ih_19 = 0i32;
                    if (((((l_q0_18) > (0i32)) as i32)) != 0) { bb_ = 20; continue 'fl1; } else { bb_ = 22; continue 'fl1; }
                }
                20 => {
                    l_i_14 = ((c.m.r32((fp).wrapping_add(((((l_jz_7).wrapping_sub(1i32)) as u32)).wrapping_mul(4))) as i32)).wrapping_shr(((8i32).wrapping_sub(l_q0_18)) as u32);
                    { let r_ = l_i_14; let n_ = (l_n_13).wrapping_add(r_); l_n_13 = n_; };
                    { let r_ = (l_i_14).wrapping_shl(((8i32).wrapping_sub(l_q0_18)) as u32); let a_ = (fp).wrapping_add(((((l_jz_7).wrapping_sub(1i32)) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                    l_ih_19 = ((c.m.r32((fp).wrapping_add(((((l_jz_7).wrapping_sub(1i32)) as u32)).wrapping_mul(4))) as i32)).wrapping_shr(((7i32).wrapping_sub(l_q0_18)) as u32);
                    bb_ = 21; continue 'fl1;
                }
                21 => {
                    if (((((l_ih_19) > (0i32)) as i32)) != 0) { bb_ = 28; continue 'fl1; } else { bb_ = 29; continue 'fl1; }
                }
                22 => {
                    if (((((l_q0_18) == (0i32)) as i32)) != 0) { bb_ = 23; continue 'fl1; } else { bb_ = 25; continue 'fl1; }
                }
                23 => {
                    l_ih_19 = ((c.m.r32((fp).wrapping_add(((((l_jz_7).wrapping_sub(1i32)) as u32)).wrapping_mul(4))) as i32)).wrapping_shr((8i32) as u32);
                    bb_ = 24; continue 'fl1;
                }
                24 => {
                    bb_ = 21; continue 'fl1;
                }
                25 => {
                    if (((rt::fge(l_z_20, F32(0x3f000000)) as i32)) != 0) { bb_ = 26; continue 'fl1; } else { bb_ = 27; continue 'fl1; }
                }
                26 => {
                    l_ih_19 = 2i32;
                    bb_ = 27; continue 'fl1;
                }
                27 => {
                    bb_ = 24; continue 'fl1;
                }
                28 => {
                    { let r_ = 1i32; let n_ = (l_n_13).wrapping_add(r_); l_n_13 = n_; };
                    l_carry_12 = 0i32;
                    l_i_14 = 0i32;
                    bb_ = 30; continue 'fl1;
                }
                29 => {
                    if (((rt::feq(l_z_20, F32(0x00000000)) as i32)) != 0) { bb_ = 51; continue 'fl1; } else { bb_ = 52; continue 'fl1; }
                }
                30 => {
                    if (((((l_i_14) < (l_jz_7)) as i32)) != 0) { bb_ = 31; continue 'fl1; } else { bb_ = 33; continue 'fl1; }
                }
                31 => {
                    l_j_15 = (c.m.r32((fp).wrapping_add((((l_i_14) as u32)).wrapping_mul(4))) as i32);
                    if (((((l_carry_12) == (0i32)) as i32)) != 0) { bb_ = 34; continue 'fl1; } else { bb_ = 36; continue 'fl1; }
                }
                32 => {
                    { let o_ = l_i_14; let n_ = (o_).wrapping_add(1); l_i_14 = n_; };
                    bb_ = 30; continue 'fl1;
                }
                33 => {
                    if (((((l_q0_18) > (0i32)) as i32)) != 0) { bb_ = 39; continue 'fl1; } else { bb_ = 40; continue 'fl1; }
                }
                34 => {
                    if (((((l_j_15) != (0i32)) as i32)) != 0) { bb_ = 37; continue 'fl1; } else { bb_ = 38; continue 'fl1; }
                }
                35 => {
                    bb_ = 32; continue 'fl1;
                }
                36 => {
                    { let v_ = (255i32).wrapping_sub(l_j_15); let a_ = (fp).wrapping_add((((l_i_14) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    bb_ = 35; continue 'fl1;
                }
                37 => {
                    l_carry_12 = 1i32;
                    { let v_ = (256i32).wrapping_sub(l_j_15); let a_ = (fp).wrapping_add((((l_i_14) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    bb_ = 38; continue 'fl1;
                }
                38 => {
                    bb_ = 35; continue 'fl1;
                }
                39 => {
                    sv_22 = l_q0_18;
                    bb_ = match sv_22 { 1i32 => 43, 2i32 => 45, _ => 41, }; continue 'fl1;
                }
                40 => {
                    if (((((l_ih_19) == (2i32)) as i32)) != 0) { bb_ = 47; continue 'fl1; } else { bb_ = 48; continue 'fl1; }
                }
                41 => {
                    bb_ = 40; continue 'fl1;
                }
                42 => {
                    bb_ = 43; continue 'fl1;
                }
                43 => {
                    { let r_ = 127i32; let a_ = (fp).wrapping_add(((((l_jz_7).wrapping_sub(1i32)) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                    bb_ = 41; continue 'fl1;
                }
                44 => {
                    bb_ = 45; continue 'fl1;
                }
                45 => {
                    { let r_ = 63i32; let a_ = (fp).wrapping_add(((((l_jz_7).wrapping_sub(1i32)) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                    bb_ = 41; continue 'fl1;
                }
                46 => {
                    bb_ = 41; continue 'fl1;
                }
                47 => {
                    l_z_20 = rt::fsub(F32(0x3f800000), l_z_20);
                    if (((((l_carry_12) != (0i32)) as i32)) != 0) { bb_ = 49; continue 'fl1; } else { bb_ = 50; continue 'fl1; }
                }
                48 => {
                    bb_ = 29; continue 'fl1;
                }
                49 => {
                    { let r_ = F::Port_scalbnf(c, F32(0x3f800000), l_q0_18); let n_ = rt::fsub(l_z_20, r_); l_z_20 = n_; };
                    bb_ = 50; continue 'fl1;
                }
                50 => {
                    bb_ = 48; continue 'fl1;
                }
                51 => {
                    l_j_15 = 0i32;
                    l_i_14 = (l_jz_7).wrapping_sub(1i32);
                    bb_ = 53; continue 'fl1;
                }
                52 => {
                    if (((rt::feq(l_z_20, F32(0x00000000)) as i32)) != 0) { bb_ = 72; continue 'fl1; } else { bb_ = 74; continue 'fl1; }
                }
                53 => {
                    if (((((l_i_14) >= (l_jk_11)) as i32)) != 0) { bb_ = 54; continue 'fl1; } else { bb_ = 56; continue 'fl1; }
                }
                54 => {
                    { let r_ = (c.m.r32((fp).wrapping_add((((l_i_14) as u32)).wrapping_mul(4))) as i32); let n_ = ((l_j_15) | (r_)); l_j_15 = n_; };
                    bb_ = 55; continue 'fl1;
                }
                55 => {
                    { let o_ = l_i_14; let n_ = (o_).wrapping_sub(1); l_i_14 = n_; };
                    bb_ = 53; continue 'fl1;
                }
                56 => {
                    if (((((l_j_15) == (0i32)) as i32)) != 0) { bb_ = 57; continue 'fl1; } else { bb_ = 58; continue 'fl1; }
                }
                57 => {
                    l_k_16 = 1i32;
                    bb_ = 59; continue 'fl1;
                }
                58 => {
                    bb_ = 52; continue 'fl1;
                }
                59 => {
                    if ((((((c.m.r32((fp).wrapping_add(((((l_jk_11).wrapping_sub(l_k_16)) as u32)).wrapping_mul(4))) as i32)) == (0i32)) as i32)) != 0) { bb_ = 60; continue 'fl1; } else { bb_ = 62; continue 'fl1; }
                }
                60 => {
                    bb_ = 61; continue 'fl1;
                }
                61 => {
                    { let o_ = l_k_16; let n_ = (o_).wrapping_add(1); l_k_16 = n_; };
                    bb_ = 59; continue 'fl1;
                }
                62 => {
                    l_i_14 = (l_jz_7).wrapping_add(1i32);
                    bb_ = 63; continue 'fl1;
                }
                63 => {
                    if (((((l_i_14) <= ((l_jz_7).wrapping_add(l_k_16))) as i32)) != 0) { bb_ = 64; continue 'fl1; } else { bb_ = 66; continue 'fl1; }
                }
                64 => {
                    { let v_ = rt::i2f((c.m.r32((p_ipio2_6).wrapping_add(((((l_jv_9).wrapping_add(l_i_14)) as u32)).wrapping_mul(4))) as i32)); let a_ = (fp.wrapping_add(80)).wrapping_add(((((l_jx_8).wrapping_add(l_i_14)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { l_j_15 = 0i32; l_fw_21 = F32(0x00000000); };
                    bb_ = 67; continue 'fl1;
                }
                65 => {
                    { let o_ = l_i_14; let n_ = (o_).wrapping_add(1); l_i_14 = n_; };
                    bb_ = 63; continue 'fl1;
                }
                66 => {
                    { let r_ = l_k_16; let n_ = (l_jz_7).wrapping_add(r_); l_jz_7 = n_; };
                    bb_ = 15; continue 'fl1;
                }
                67 => {
                    if (((((l_j_15) <= (l_jx_8)) as i32)) != 0) { bb_ = 68; continue 'fl1; } else { bb_ = 70; continue 'fl1; }
                }
                68 => {
                    { let r_ = rt::fmul(F32(c.m.r32((p_x_1).wrapping_add((((l_j_15) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(80)).wrapping_add((((((l_jx_8).wrapping_add(l_i_14)).wrapping_sub(l_j_15)) as u32)).wrapping_mul(4))))); let n_ = rt::fadd(l_fw_21, r_); l_fw_21 = n_; };
                    bb_ = 69; continue 'fl1;
                }
                69 => {
                    { let o_ = l_j_15; let n_ = (o_).wrapping_add(1); l_j_15 = n_; };
                    bb_ = 67; continue 'fl1;
                }
                70 => {
                    { let v_ = l_fw_21; let a_ = (fp.wrapping_add(240)).wrapping_add((((l_i_14) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    bb_ = 65; continue 'fl1;
                }
                71 => {
                    bb_ = 58; continue 'fl1;
                }
                72 => {
                    { let r_ = 1i32; let n_ = (l_jz_7).wrapping_sub(r_); l_jz_7 = n_; };
                    { let r_ = 8i32; let n_ = (l_q0_18).wrapping_sub(r_); l_q0_18 = n_; };
                    bb_ = 75; continue 'fl1;
                }
                73 => {
                    l_fw_21 = F::Port_scalbnf(c, F32(0x3f800000), l_q0_18);
                    l_i_14 = l_jz_7;
                    bb_ = 81; continue 'fl1;
                }
                74 => {
                    l_z_20 = F::Port_scalbnf(c, l_z_20, (l_q0_18).wrapping_neg());
                    if (((rt::fge(l_z_20, F32(0x43800000)) as i32)) != 0) { bb_ = 78; continue 'fl1; } else { bb_ = 80; continue 'fl1; }
                }
                75 => {
                    if ((((((c.m.r32((fp).wrapping_add((((l_jz_7) as u32)).wrapping_mul(4))) as i32)) == (0i32)) as i32)) != 0) { bb_ = 76; continue 'fl1; } else { bb_ = 77; continue 'fl1; }
                }
                76 => {
                    { let o_ = l_jz_7; let n_ = (o_).wrapping_sub(1); l_jz_7 = n_; };
                    { let r_ = 8i32; let n_ = (l_q0_18).wrapping_sub(r_); l_q0_18 = n_; };
                    bb_ = 75; continue 'fl1;
                }
                77 => {
                    bb_ = 73; continue 'fl1;
                }
                78 => {
                    l_fw_21 = rt::i2f(rt::f2i(rt::fmul(F32(0x3b800000), l_z_20)));
                    { let v_ = rt::f2i(rt::fsub(l_z_20, rt::fmul(F32(0x43800000), l_fw_21))); let a_ = (fp).wrapping_add((((l_jz_7) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let r_ = 1i32; let n_ = (l_jz_7).wrapping_add(r_); l_jz_7 = n_; };
                    { let r_ = 8i32; let n_ = (l_q0_18).wrapping_add(r_); l_q0_18 = n_; };
                    { let v_ = rt::f2i(l_fw_21); let a_ = (fp).wrapping_add((((l_jz_7) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    bb_ = 79; continue 'fl1;
                }
                79 => {
                    bb_ = 73; continue 'fl1;
                }
                80 => {
                    { let v_ = rt::f2i(l_z_20); let a_ = (fp).wrapping_add((((l_jz_7) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    bb_ = 79; continue 'fl1;
                }
                81 => {
                    if (((((l_i_14) >= (0i32)) as i32)) != 0) { bb_ = 82; continue 'fl1; } else { bb_ = 84; continue 'fl1; }
                }
                82 => {
                    { let v_ = rt::fmul(l_fw_21, rt::i2f((c.m.r32((fp).wrapping_add((((l_i_14) as u32)).wrapping_mul(4))) as i32))); let a_ = (fp.wrapping_add(240)).wrapping_add((((l_i_14) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let r_ = F32(0x3b800000); let n_ = rt::fmul(l_fw_21, r_); l_fw_21 = n_; };
                    bb_ = 83; continue 'fl1;
                }
                83 => {
                    { let o_ = l_i_14; let n_ = (o_).wrapping_sub(1); l_i_14 = n_; };
                    bb_ = 81; continue 'fl1;
                }
                84 => {
                    l_i_14 = l_jz_7;
                    bb_ = 85; continue 'fl1;
                }
                85 => {
                    if (((((l_i_14) >= (0i32)) as i32)) != 0) { bb_ = 86; continue 'fl1; } else { bb_ = 88; continue 'fl1; }
                }
                86 => {
                    { l_fw_21 = F32(0x00000000); l_k_16 = 0i32; };
                    bb_ = 89; continue 'fl1;
                }
                87 => {
                    { let o_ = l_i_14; let n_ = (o_).wrapping_sub(1); l_i_14 = n_; };
                    bb_ = 85; continue 'fl1;
                }
                88 => {
                    sv_23 = p_prec_5;
                    bb_ = match sv_23 { 0i32 => 95, 1i32 => 101, 2i32 => 102, 3i32 => 112, _ => 93, }; continue 'fl1;
                }
                89 => {
                    if (((((((((l_k_16) <= (l_jp_10)) as i32)) != 0) && (((((l_k_16) <= ((l_jz_7).wrapping_sub(l_i_14))) as i32)) != 0)) as i32)) != 0) { bb_ = 90; continue 'fl1; } else { bb_ = 92; continue 'fl1; }
                }
                90 => {
                    { let r_ = rt::fmul(F32(c.m.r32((sym::kf_rem_pio2__PIo2).wrapping_add((((l_k_16) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(240)).wrapping_add(((((l_i_14).wrapping_add(l_k_16)) as u32)).wrapping_mul(4))))); let n_ = rt::fadd(l_fw_21, r_); l_fw_21 = n_; };
                    bb_ = 91; continue 'fl1;
                }
                91 => {
                    { let o_ = l_k_16; let n_ = (o_).wrapping_add(1); l_k_16 = n_; };
                    bb_ = 89; continue 'fl1;
                }
                92 => {
                    { let v_ = l_fw_21; let a_ = (fp.wrapping_add(160)).wrapping_add(((((l_jz_7).wrapping_sub(l_i_14)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    bb_ = 87; continue 'fl1;
                }
                93 => {
                    break 'f ((l_n_13) & (7i32));
                }
                94 => {
                    bb_ = 95; continue 'fl1;
                }
                95 => {
                    l_fw_21 = F32(0x00000000);
                    l_i_14 = l_jz_7;
                    bb_ = 96; continue 'fl1;
                }
                96 => {
                    if (((((l_i_14) >= (0i32)) as i32)) != 0) { bb_ = 97; continue 'fl1; } else { bb_ = 99; continue 'fl1; }
                }
                97 => {
                    { let r_ = F32(c.m.r32((fp.wrapping_add(160)).wrapping_add((((l_i_14) as u32)).wrapping_mul(4)))); let n_ = rt::fadd(l_fw_21, r_); l_fw_21 = n_; };
                    bb_ = 98; continue 'fl1;
                }
                98 => {
                    { let o_ = l_i_14; let n_ = (o_).wrapping_sub(1); l_i_14 = n_; };
                    bb_ = 96; continue 'fl1;
                }
                99 => {
                    { let v_ = (if (((((l_ih_19) == (0i32)) as i32)) != 0) { l_fw_21 } else { rt::fneg(l_fw_21) }); let a_ = (p_y_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    bb_ = 93; continue 'fl1;
                }
                100 => {
                    bb_ = 101; continue 'fl1;
                }
                101 => {
                    bb_ = 102; continue 'fl1;
                }
                102 => {
                    l_fw_21 = F32(0x00000000);
                    l_i_14 = l_jz_7;
                    bb_ = 103; continue 'fl1;
                }
                103 => {
                    if (((((l_i_14) >= (0i32)) as i32)) != 0) { bb_ = 104; continue 'fl1; } else { bb_ = 106; continue 'fl1; }
                }
                104 => {
                    { let r_ = F32(c.m.r32((fp.wrapping_add(160)).wrapping_add((((l_i_14) as u32)).wrapping_mul(4)))); let n_ = rt::fadd(l_fw_21, r_); l_fw_21 = n_; };
                    bb_ = 105; continue 'fl1;
                }
                105 => {
                    { let o_ = l_i_14; let n_ = (o_).wrapping_sub(1); l_i_14 = n_; };
                    bb_ = 103; continue 'fl1;
                }
                106 => {
                    { let v_ = (if (((((l_ih_19) == (0i32)) as i32)) != 0) { l_fw_21 } else { rt::fneg(l_fw_21) }); let a_ = (p_y_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    l_fw_21 = rt::fsub(F32(c.m.r32((fp.wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_fw_21);
                    l_i_14 = 1i32;
                    bb_ = 107; continue 'fl1;
                }
                107 => {
                    if (((((l_i_14) <= (l_jz_7)) as i32)) != 0) { bb_ = 108; continue 'fl1; } else { bb_ = 110; continue 'fl1; }
                }
                108 => {
                    { let r_ = F32(c.m.r32((fp.wrapping_add(160)).wrapping_add((((l_i_14) as u32)).wrapping_mul(4)))); let n_ = rt::fadd(l_fw_21, r_); l_fw_21 = n_; };
                    bb_ = 109; continue 'fl1;
                }
                109 => {
                    { let o_ = l_i_14; let n_ = (o_).wrapping_add(1); l_i_14 = n_; };
                    bb_ = 107; continue 'fl1;
                }
                110 => {
                    { let v_ = (if (((((l_ih_19) == (0i32)) as i32)) != 0) { l_fw_21 } else { rt::fneg(l_fw_21) }); let a_ = (p_y_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    bb_ = 93; continue 'fl1;
                }
                111 => {
                    bb_ = 112; continue 'fl1;
                }
                112 => {
                    l_i_14 = l_jz_7;
                    bb_ = 113; continue 'fl1;
                }
                113 => {
                    if (((((l_i_14) > (0i32)) as i32)) != 0) { bb_ = 114; continue 'fl1; } else { bb_ = 116; continue 'fl1; }
                }
                114 => {
                    l_fw_21 = rt::fadd(F32(c.m.r32((fp.wrapping_add(160)).wrapping_add(((((l_i_14).wrapping_sub(1i32)) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(160)).wrapping_add((((l_i_14) as u32)).wrapping_mul(4)))));
                    { let r_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(160)).wrapping_add(((((l_i_14).wrapping_sub(1i32)) as u32)).wrapping_mul(4)))), l_fw_21); let a_ = (fp.wrapping_add(160)).wrapping_add((((l_i_14) as u32)).wrapping_mul(4)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let v_ = l_fw_21; let a_ = (fp.wrapping_add(160)).wrapping_add(((((l_i_14).wrapping_sub(1i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    bb_ = 115; continue 'fl1;
                }
                115 => {
                    { let o_ = l_i_14; let n_ = (o_).wrapping_sub(1); l_i_14 = n_; };
                    bb_ = 113; continue 'fl1;
                }
                116 => {
                    l_i_14 = l_jz_7;
                    bb_ = 117; continue 'fl1;
                }
                117 => {
                    if (((((l_i_14) > (1i32)) as i32)) != 0) { bb_ = 118; continue 'fl1; } else { bb_ = 120; continue 'fl1; }
                }
                118 => {
                    l_fw_21 = rt::fadd(F32(c.m.r32((fp.wrapping_add(160)).wrapping_add(((((l_i_14).wrapping_sub(1i32)) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(160)).wrapping_add((((l_i_14) as u32)).wrapping_mul(4)))));
                    { let r_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(160)).wrapping_add(((((l_i_14).wrapping_sub(1i32)) as u32)).wrapping_mul(4)))), l_fw_21); let a_ = (fp.wrapping_add(160)).wrapping_add((((l_i_14) as u32)).wrapping_mul(4)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let v_ = l_fw_21; let a_ = (fp.wrapping_add(160)).wrapping_add(((((l_i_14).wrapping_sub(1i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    bb_ = 119; continue 'fl1;
                }
                119 => {
                    { let o_ = l_i_14; let n_ = (o_).wrapping_sub(1); l_i_14 = n_; };
                    bb_ = 117; continue 'fl1;
                }
                120 => {
                    { l_fw_21 = F32(0x00000000); l_i_14 = l_jz_7; };
                    bb_ = 121; continue 'fl1;
                }
                121 => {
                    if (((((l_i_14) >= (2i32)) as i32)) != 0) { bb_ = 122; continue 'fl1; } else { bb_ = 124; continue 'fl1; }
                }
                122 => {
                    { let r_ = F32(c.m.r32((fp.wrapping_add(160)).wrapping_add((((l_i_14) as u32)).wrapping_mul(4)))); let n_ = rt::fadd(l_fw_21, r_); l_fw_21 = n_; };
                    bb_ = 123; continue 'fl1;
                }
                123 => {
                    { let o_ = l_i_14; let n_ = (o_).wrapping_sub(1); l_i_14 = n_; };
                    bb_ = 121; continue 'fl1;
                }
                124 => {
                    if (((((l_ih_19) == (0i32)) as i32)) != 0) { bb_ = 125; continue 'fl1; } else { bb_ = 127; continue 'fl1; }
                }
                125 => {
                    { let v_ = F32(c.m.r32((fp.wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_y_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((fp.wrapping_add(160)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (p_y_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = l_fw_21; let a_ = (p_y_2).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    bb_ = 126; continue 'fl1;
                }
                126 => {
                    bb_ = 93; continue 'fl1;
                }
                127 => {
                    { let v_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (p_y_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(160)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = (p_y_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fneg(l_fw_21); let a_ = (p_y_2).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    bb_ = 126; continue 'fl1;
                }
                128 => {
                    break 'fl1;
                }
                _ => unreachable!(),
            }
        }
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "__kernel_rem_pio2f", &PROBE_N, &[], true); }
    r_
}
