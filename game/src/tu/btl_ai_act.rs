#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlAiMove_Dispatch(c: &mut Ctx, mut p_s_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMove_Dispatch", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_act_2: u32 = 0;
    let r_ = 'f: {
    l_act_2 = (p_s_1).wrapping_add(40);
    if ((((((((((c.m.r32((l_act_2).wrapping_add(4)) as i32)) > (0i32)) as i32)) != 0) && (({ let a1_ = { let a_ = ((l_act_2).wrapping_add(((((c.m.r32((l_act_2).wrapping_add(4)) as i32)).wrapping_shl((3i32) as u32)) as u32))).wrapping_add(((12i32) as u32)); (c.m.r32(a_) as i32) }; F::BtlAiSeq_IsInterrupted(c, p_s_1, a1_) }) != 0)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (l_act_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        break 'f;
    }
    { let fp_ = { let a_ = (sym::gBtlAiMovePhases).wrapping_add(((((c.m.r32((l_act_2).wrapping_add(8)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; { let _ = D::call(c, fp_, &[((p_s_1) as u64)]); } };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMove_Dispatch", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiPad_Clear(c: &mut Ctx, mut p_pad_1: u32, mut p_keep_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPad_Clear", &PROBE_N, &[(p_pad_1) as u32, (p_keep_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    if (((((p_keep_2) == (0i32)) as i32)) != 0) {
        {
            l_i_3 = 15i32;
            'l1: loop {
                if !(((((l_i_3) >= (0i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = 0i32; let a_ = ((p_pad_1).wrapping_add(12)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_sub(1); l_i_3 = n_; };
            }
        }
        { let v_ = 0i32; let a_ = (p_pad_1).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (p_pad_1).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
    }
    { let v_ = 0i32; let a_ = p_pad_1; c.m.w32(a_, (v_) as u32); };
    { let v_ = F32(0x00000000); let a_ = (p_pad_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (p_pad_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (p_pad_1).wrapping_add(72); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (p_pad_1).wrapping_add(76); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPad_Clear", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiPad_AddButtons(c: &mut Ctx, mut p_pad_1: u32, mut p_bits_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPad_AddButtons", &PROBE_N, &[(p_pad_1) as u32, (p_bits_2) as u32], false); }
    let r_ = 'f: {
    if ((((p_bits_2) & (1i32))) != 0) {
        { let r_ = 1i32; let a_ = p_pad_1; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if ((((p_bits_2) & (2i32))) != 0) {
        { let r_ = 2i32; let a_ = p_pad_1; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if ((((p_bits_2) & (4i32))) != 0) {
        { let r_ = 4i32; let a_ = p_pad_1; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if ((((p_bits_2) & (8i32))) != 0) {
        { let r_ = 8i32; let a_ = p_pad_1; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if ((((p_bits_2) & (16i32))) != 0) {
        { let r_ = 16i32; let a_ = p_pad_1; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if ((((p_bits_2) & (32i32))) != 0) {
        { let r_ = 32i32; let a_ = p_pad_1; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if ((((p_bits_2) & (64i32))) != 0) {
        { let r_ = 64i32; let a_ = p_pad_1; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if ((((p_bits_2) & (128i32))) != 0) {
        { let r_ = 128i32; let a_ = p_pad_1; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if ((((p_bits_2) & (256i32))) != 0) {
        { let r_ = 16384i32; let a_ = p_pad_1; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if ((((p_bits_2) & (512i32))) != 0) {
        { let r_ = 512i32; let a_ = p_pad_1; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if ((((p_bits_2) & (2048i32))) != 0) {
        { let r_ = 2048i32; let a_ = p_pad_1; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if ((((p_bits_2) & (4096i32))) != 0) {
        { let r_ = 4096i32; let a_ = p_pad_1; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if ((((p_bits_2) & (8192i32))) != 0) {
        { let r_ = 8192i32; let a_ = p_pad_1; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPad_AddButtons", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiPad_TrackStill(c: &mut Ctx, mut p_s_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPad_TrackStill", &PROBE_N, &[(p_s_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_m_2: u32 = 0;
    let mut l_len_3: F32 = F32(0);
    let r_ = 'f: {
        l_m_2 = (p_s_1).wrapping_add(192);
        if (((((c.m.r32((p_s_1).wrapping_add(616)) as i32)) & (6146i32))) != 0) {
            { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetPos(c, a0_, fp) };
            F::Vec3_Sub(c, fp.wrapping_add(16), fp, (l_m_2).wrapping_add(352));
            l_len_3 = F::Vec3_Length(c, fp.wrapping_add(16));
            if (((rt::flt(l_len_3, { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetRadius(c, a0_) }) as i32)) != 0) {
                if ((((({ let a_ = (l_m_2).wrapping_add(368); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) > (30i32)) as i32)) != 0) {
                    { let r_ = 1i32; let a_ = (l_m_2).wrapping_add(348); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
            { let s_ = fp; let d_ = (l_m_2).wrapping_add(352); rt::memcpy(c, d_, s_, 16); };
        } else {
            { let v_ = 0i32; let a_ = (l_m_2).wrapping_add(368); c.m.w32(a_, (v_) as u32); };
            { let r_ = (!(1i32)); let a_ = (l_m_2).wrapping_add(348); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPad_TrackStill", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiPad_Set(c: &mut Ctx, mut p_s_1: u32, mut p_hold_2: i32, mut p_press_3: i32, mut p_once_4: i32, mut p_special_5: i16, mut p_x_6: F32, mut p_y_7: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPad_Set", &PROBE_N, &[(p_s_1) as u32, (p_hold_2) as u32, (p_press_3) as u32, (p_once_4) as u32, (p_special_5) as u32, (p_x_6).0, (p_y_7).0], false); }
    let mut l_pad_8: u32 = 0;
    let mut l_i_9: i32 = 0;
    let mut l_bits_10: i32 = 0;
    let r_ = 'f: {
    l_pad_8 = (p_s_1).wrapping_add(616);
    {
        l_i_9 = 0i32;
        'l1: loop {
            if !(((((l_i_9) < (14i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((!(((((c.m.r32(((l_pad_8).wrapping_add(12)).wrapping_add((((l_i_9) as u32)).wrapping_mul(4))) as i32)) & (2i32))) != 0)) as i32)) != 0) {
                    l_bits_10 = ((p_once_4) & ((1i32).wrapping_shl((l_i_9) as u32)));
                    if ((l_bits_10) != 0) {
                        F::BtlAiPad_AddButtons(c, l_pad_8, l_bits_10);
                        { let r_ = 2i32; let a_ = ((l_pad_8).wrapping_add(12)).wrapping_add((((l_i_9) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                }
                if ((((!(((((c.m.r32(((l_pad_8).wrapping_add(12)).wrapping_add((((l_i_9) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0)) as i32)) != 0) {
                    F::BtlAiPad_AddButtons(c, l_pad_8, ((p_press_3) & ((1i32).wrapping_shl((l_i_9) as u32))));
                }
                F::BtlAiPad_AddButtons(c, l_pad_8, ((p_hold_2) & ((1i32).wrapping_shl((l_i_9) as u32))));
            }
            { let o_ = l_i_9; let n_ = (o_).wrapping_add(1); l_i_9 = n_; };
        }
    }
    if (((((((((((p_hold_2) & (1536i32))) == (1536i32)) as i32)) != 0) && ((((p_press_3) & (8192i32))) != 0)) as i32)) != 0) {
        { let v_ = 32768i32; let a_ = l_pad_8; c.m.w32(a_, (v_) as u32); };
    }
    's3: {
        let sv11: i32 = ((p_special_5) as i32);
        match sv11 {
            1i32 => {
                if ((((!(((((c.m.r32((l_pad_8).wrapping_add(68)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
                    { let v_ = 65536i32; let a_ = l_pad_8; c.m.w32(a_, (v_) as u32); };
                } else {
                    { let v_ = 262144i32; let a_ = l_pad_8; c.m.w32(a_, (v_) as u32); };
                }
                break 's3;
            }
            2i32 => {
                if ((((!(((((c.m.r32((l_pad_8).wrapping_add(68)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
                    { let v_ = 131072i32; let a_ = l_pad_8; c.m.w32(a_, (v_) as u32); };
                } else {
                    { let v_ = 262144i32; let a_ = l_pad_8; c.m.w32(a_, (v_) as u32); };
                }
                break 's3;
            }
            3i32 => {
                if ((((!(((((c.m.r32(((l_pad_8).wrapping_add(12)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) & (4i32))) != 0)) as i32)) != 0) {
                    { let v_ = 2i32; let a_ = (l_pad_8).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
                    { let r_ = 4i32; let a_ = ((l_pad_8).wrapping_add(12)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let r_ = (!(8i32)); let a_ = l_pad_8; let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                }
                break 's3;
            }
            _ => {
            }
        }
    }
    { let v_ = rt::fmul(rt::fadd(F32(c.m.r32((l_pad_8).wrapping_add(72))), p_x_6), F32(0x3f000000)); let a_ = (l_pad_8).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(rt::fadd(F32(c.m.r32((l_pad_8).wrapping_add(76))), p_y_7), F32(0x3f000000)); let a_ = (l_pad_8).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_pad_8).wrapping_add(4))); let a_ = (l_pad_8).wrapping_add(72); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_pad_8).wrapping_add(8))); let a_ = (l_pad_8).wrapping_add(76); c.m.w32(a_, (v_).0); };
    F::BtlAiPad_TrackStill(c, p_s_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPad_Set", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiPad_EndFrame(c: &mut Ctx, mut p_pad_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPad_EndFrame", &PROBE_N, &[(p_pad_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    if ((((((c.m.r32((p_pad_1).wrapping_add(84)) as i32)) > (0i32)) as i32)) != 0) {
        { let a_ = (p_pad_1).wrapping_add(84); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    }
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (14i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let r_ = 1i32; let a_ = ((p_pad_1).wrapping_add(12)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    { let r_ = 1i32; let a_ = (p_pad_1).wrapping_add(68); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPad_EndFrame", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiAtk_Reset(c: &mut Ctx, mut p_side_1: i32, mut p_atk_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiAtk_Reset", &PROBE_N, &[(p_side_1) as u32, (p_atk_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = p_atk_2; c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (p_atk_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = F32(0x00000000); let a_ = (p_atk_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F::BtlCharApi_GetUnk974(c, p_side_1); let a_ = (p_atk_2).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let r_ = (!(7i32)); let a_ = (p_atk_2).wrapping_add(16); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    { let v_ = 15i32; let a_ = (p_atk_2).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = 15i32; let a_ = (p_atk_2).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiAtk_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiAtk_RollCharge(c: &mut Ctx, mut p_s_1: u32, mut p_base_2: i8, mut p_range_3: i8) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiAtk_RollCharge", &PROBE_N, &[(p_s_1) as u32, (p_base_2) as u32, (p_range_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_roll_4: i32 = 0;
    let mut l_opp_5: u32 = 0;
    let mut l_kind_6: i32 = 0;
    let mut l_ret_7: F32 = F32(0);
    let mut l_thr_8: i32 = 0;
    let r_ = 'f: {
        l_roll_4 = ((F::Rand_Range(c, ((100i32) as u32))) as i32);
        rt::memset(c, fp, 0, 20);
        { let v_ = 10i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
        { let v_ = 20i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
        { let v_ = 30i32; c.m.w32(fp.wrapping_add(8), (v_) as u32); }
        { let v_ = 40i32; c.m.w32(fp.wrapping_add(12), (v_) as u32); }
        { let v_ = 50i32; c.m.w32(fp.wrapping_add(16), (v_) as u32); }
        l_opp_5 = ((c.m.r32(sym::gBtlAi)).wrapping_add(16)).wrapping_add(((((((c.m.r32(p_s_1) as i32)) ^ (1i32))) as u32)).wrapping_mul(1312));
        l_kind_6 = (c.m.r32(((l_opp_5).wrapping_add(576)).wrapping_add(28)) as i32);
        l_ret_7 = F32(0x00000000);
        if (((((l_kind_6) == (2i32)) as i32)) != 0) {
            l_thr_8 = ({ let a_ = (fp).wrapping_add(((((if ((((((c.m.r32((p_s_1).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) { 0i32 } else { rt::div_i32((c.m.r32((p_s_1).wrapping_add(8)) as i32), 6i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(20i32);
        } else {
            l_thr_8 = { let a_ = (fp).wrapping_add(((((if ((((((c.m.r32((p_s_1).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) { 0i32 } else { rt::div_i32((c.m.r32((p_s_1).wrapping_add(8)) as i32), 6i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
        }
        if (((((l_roll_4) < (l_thr_8)) as i32)) != 0) {
            if (((((l_kind_6) == (2i32)) as i32)) != 0) {
                if (((((((F::Rand_Range(c, ((100i32) as u32))) as i32)) < (l_thr_8)) as i32)) != 0) {
                    if (((((((p_range_3) as i32)) > (0i32)) as i32)) != 0) {
                        l_ret_7 = F32(0x3f800000);
                    } else {
                        l_ret_7 = F32(0x00000000);
                    }
                } else {
                    l_ret_7 = F32(0x3f8ccccc);
                }
            } else {
                l_ret_7 = rt::fmul(rt::i2f(((p_base_2) as i32)), F32(0x3dcccccc));
            }
        } else {
            if (((((((p_range_3) as i32)) != (0i32)) as i32)) != 0) {
                l_ret_7 = rt::fmul(rt::i2f(((F::Rand_Range(c, ((p_range_3) as u32))) as i32)), F32(0x3dcccccc));
            }
        }
        break 'f l_ret_7;
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAiAtk_RollCharge", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiAtk_SetInput(c: &mut Ctx, mut p_s_1: u32, mut p_kind_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiAtk_SetInput", &PROBE_N, &[(p_s_1) as u32, (p_kind_2) as u32], false); }
    let mut l_atk_3: u32 = 0;
    let mut l_action_4: i32 = 0;
    let mut l_state_5: i32 = 0;
    let mut l_row_6: i32 = 0;
    let mut l_bit_7: u8 = 0;
    let mut l_t_8: F32 = F32(0);
    let mut l_dir_9: i32 = 0;
    let r_ = 'f: {
    l_atk_3 = (p_s_1).wrapping_add(576);
    l_action_4 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetAction(c, a0_) };
    l_state_5 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetUnk974(c, a0_) };
    l_row_6 = 0i32;
    l_bit_7 = (((((({ let a_ = (c.m.r32((p_s_1).wrapping_add(24))).wrapping_add(((0i32) as u32)); c.m.r8(a_) }) as i32)) & (1i32))) as u8);
    if ((((((((((((l_state_5).wrapping_sub(55i32)) as u32)) < (((4i32) as u32))) as i32)) != 0) || ((((((((l_state_5).wrapping_sub(60i32)) as u32)) < (((4i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        l_row_6 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetParamByte84(c, a0_, (((if (((((l_state_5) < (60i32)) as i32)) != 0) { (l_state_5).wrapping_sub(55i32) } else { (l_state_5).wrapping_sub(60i32) })) as u32)) };
    } else {
        if (((((l_state_5) == (27i32)) as i32)) != 0) {
            if (((((p_kind_2) == (((0i32) as u32))) as i32)) != 0) {
                if (((((l_action_4) == (31i32)) as i32)) != 0) {
                    l_row_6 = rt::div_i32({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetParamByte8D(c, a0_) }, 2i32);
                } else {
                    l_row_6 = rt::div_i32({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetParamByte8A(c, a0_) }, 2i32);
                }
            }
        } else {
            break 'f;
        }
    }
    's1: {
        let sv10: u32 = p_kind_2;
        match sv10 {
            0u32 => {
                { let v_ = { let a1_ = (c.m.r8(((sym::btl_ai_act__sAiActChargeRows).wrapping_add((((l_row_6) as u32)).wrapping_mul(3))).wrapping_add(((l_bit_7) as u32))) as i8); let a2_ = (c.m.r8(((sym::btl_ai_act__sAiActChargeRows).wrapping_add((((l_row_6) as u32)).wrapping_mul(3))).wrapping_add(((2i32) as u32))) as i8); F::BtlAiAtk_RollCharge(c, p_s_1, a1_, a2_) }; let a_ = (l_atk_3).wrapping_add(8); c.m.w32(a_, (v_).0); };
                if (((rt::fne(F32(c.m.r32((l_atk_3).wrapping_add(8))), F32(0x00000000)) as i32)) != 0) {
                    { let r_ = 4i32; let a_ = l_atk_3; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                } else {
                    { let r_ = 4i32; let a_ = (l_atk_3).wrapping_add(4); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
                break 's1;
            }
            1u32 | 2u32 => {
                l_t_8 = { let a1_ = (c.m.r8(((sym::btl_ai_act__sAiActChargeRows).wrapping_add((((0i32) as u32)).wrapping_mul(3))).wrapping_add(((l_bit_7) as u32))) as i8); let a2_ = (c.m.r8(((sym::btl_ai_act__sAiActChargeRows).wrapping_add((((0i32) as u32)).wrapping_mul(3))).wrapping_add(((2i32) as u32))) as i8); F::BtlAiAtk_RollCharge(c, p_s_1, a1_, a2_) };
                { let v_ = l_t_8; let a_ = (l_atk_3).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let r_ = (if (((((p_kind_2) == (((1i32) as u32))) as i32)) != 0) { 16i32 } else { 32i32 }); let a_ = l_atk_3; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                if (((rt::feq(l_t_8, F32(0x00000000)) as i32)) != 0) {
                    { let r_ = 4i32; let a_ = (l_atk_3).wrapping_add(4); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                } else {
                    { let r_ = 4i32; let a_ = l_atk_3; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
                break 's1;
            }
            3u32 => {
                { let v_ = { let a1_ = (c.m.r8(((sym::btl_ai_act__sAiActChargeRows).wrapping_add((((6i32) as u32)).wrapping_mul(3))).wrapping_add(((l_bit_7) as u32))) as i8); let a2_ = (c.m.r8(((sym::btl_ai_act__sAiActChargeRows).wrapping_add((((6i32) as u32)).wrapping_mul(3))).wrapping_add(((2i32) as u32))) as i8); F::BtlAiAtk_RollCharge(c, p_s_1, a1_, a2_) }; let a_ = (l_atk_3).wrapping_add(8); c.m.w32(a_, (v_).0); };
                if (((rt::feq(F32(c.m.r32((l_atk_3).wrapping_add(8))), F32(0x00000000)) as i32)) != 0) {
                    { let v_ = F32(0x3dcccccc); let a_ = (l_atk_3).wrapping_add(8); c.m.w32(a_, (v_).0); };
                }
                { let r_ = 8i32; let a_ = l_atk_3; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                {
                    l_dir_9 = ((F::Rand_Range(c, ((4i32) as u32))) as i32);
                    { let r_ = (1i32).wrapping_shl(((l_dir_9).wrapping_add(4i32)) as u32); let a_ = l_atk_3; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
                break 's1;
            }
            4u32 => {
                { let r_ = 1i32; let a_ = l_atk_3; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            5u32 => {
                { let r_ = 1i32; let a_ = (l_atk_3).wrapping_add(4); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = (if (((((F::Rand_Range(c, ((2i32) as u32))) == (((0i32) as u32))) as i32)) != 0) { 64i32 } else { 128i32 }); let a_ = l_atk_3; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            6u32 => {
                { let r_ = 2i32; let a_ = (l_atk_3).wrapping_add(4); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            7u32 => {
                { let r_ = 1i32; let a_ = (l_atk_3).wrapping_add(16); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = 16i32; let a_ = l_atk_3; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = 2i32; let a_ = (l_atk_3).wrapping_add(4); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            8u32 => {
                { let r_ = 2i32; let a_ = (l_atk_3).wrapping_add(4); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = (if (((((F::Rand_Range(c, ((2i32) as u32))) == (((0i32) as u32))) as i32)) != 0) { 64i32 } else { 128i32 }); let a_ = l_atk_3; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiAtk_SetInput", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiAtk_PickOption(c: &mut Ctx, mut p_s_1: u32, mut p_lo_2: u32, mut p_hi_3: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiAtk_PickOption", &PROBE_N, &[(p_s_1) as u32, (p_lo_2) as u32, (p_hi_3) as u32], false); }
    let mut l_atk_4: u32 = 0;
    let mut l_st_5: u32 = 0;
    let mut l_state_6: i32 = 0;
    let mut l_step_7: i32 = 0;
    let mut l_roll_8: i32 = 0;
    let mut l_sum_9: i32 = 0;
    let r_ = 'f: {
    l_atk_4 = (p_s_1).wrapping_add(576);
    l_st_5 = (p_s_1).wrapping_add(704);
    l_state_6 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetUnk974(c, a0_) };
    l_step_7 = (if (((((l_state_6) < (60i32)) as i32)) != 0) { (l_state_6).wrapping_sub(55i32) } else { (l_state_6).wrapping_sub(60i32) });
    l_roll_8 = (((F::Rand_Range(c, ((100i32) as u32))).wrapping_add(((1i32) as u32))) as i32);
    { let v_ = (1i32).wrapping_neg(); let a_ = (l_atk_4).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
    { let r_ = (l_step_7).wrapping_mul(12i32); let n_ = (p_lo_2).wrapping_add(((r_) as u32)); p_lo_2 = n_; };
    { let r_ = (l_step_7).wrapping_mul(12i32); let n_ = (p_hi_3).wrapping_add(((r_) as u32)); p_hi_3 = n_; };
    l_sum_9 = { let a0_ = (c.m.r32((p_s_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_2).wrapping_add(((0i32) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_3).wrapping_add(((0i32) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) };
    if (((((l_sum_9) >= (l_roll_8)) as i32)) != 0) {
        F::BtlAiAtk_SetInput(c, p_s_1, ((0i32) as u32));
        { let v_ = 0i32; let a_ = (l_atk_4).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        break 'f 1i32;
    }
    { let r_ = { let a0_ = (c.m.r32((p_s_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_2).wrapping_add(((1i32) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_3).wrapping_add(((1i32) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_9).wrapping_add(r_); l_sum_9 = n_; };
    if (((((l_sum_9) >= (l_roll_8)) as i32)) != 0) {
        F::BtlAiAtk_SetInput(c, p_s_1, ((1i32) as u32));
        { let v_ = 1i32; let a_ = (l_atk_4).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        break 'f 1i32;
    }
    { let r_ = { let a0_ = (c.m.r32((p_s_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_2).wrapping_add(((2i32) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_3).wrapping_add(((2i32) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_9).wrapping_add(r_); l_sum_9 = n_; };
    if (((((l_sum_9) >= (l_roll_8)) as i32)) != 0) {
        F::BtlAiAtk_SetInput(c, p_s_1, ((2i32) as u32));
        { let v_ = 2i32; let a_ = (l_atk_4).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        break 'f 1i32;
    }
    { let r_ = { let a0_ = (c.m.r32((p_s_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_2).wrapping_add(((3i32) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_3).wrapping_add(((3i32) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_9).wrapping_add(r_); l_sum_9 = n_; };
    if (((((l_sum_9) >= (l_roll_8)) as i32)) != 0) {
        { let v_ = 3i32; let a_ = (l_atk_4).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        break 'f 1i32;
    }
    { let r_ = { let a0_ = (c.m.r32((p_s_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_2).wrapping_add(((4i32) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_3).wrapping_add(((4i32) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_9).wrapping_add(r_); l_sum_9 = n_; };
    if (((((l_sum_9) >= (l_roll_8)) as i32)) != 0) {
        F::BtlAiAtk_SetInput(c, p_s_1, ((3i32) as u32));
        { let v_ = 4i32; let a_ = (l_atk_4).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        break 'f 1i32;
    }
    { let r_ = { let a0_ = (c.m.r32((p_s_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_2).wrapping_add(((5i32) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_3).wrapping_add(((5i32) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_9).wrapping_add(r_); l_sum_9 = n_; };
    if (((((l_sum_9) >= (l_roll_8)) as i32)) != 0) {
        { let v_ = 5i32; let a_ = (l_atk_4).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        break 'f 1i32;
    }
    { let r_ = { let a0_ = (c.m.r32((p_s_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_2).wrapping_add(((6i32) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_3).wrapping_add(((6i32) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_9).wrapping_add(r_); l_sum_9 = n_; };
    if (((((l_sum_9) >= (l_roll_8)) as i32)) != 0) {
        F::BtlAiAtk_SetInput(c, p_s_1, ((5i32) as u32));
        { let v_ = 6i32; let a_ = (l_atk_4).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        break 'f 1i32;
    }
    { let r_ = { let a0_ = (c.m.r32((p_s_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_2).wrapping_add(((7i32) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_3).wrapping_add(((7i32) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_9).wrapping_add(r_); l_sum_9 = n_; };
    if (((((l_sum_9) >= (l_roll_8)) as i32)) != 0) {
        if ((((c.m.r64(l_st_5)) & (((2305843009213693952i64) as u64)))) != 0) {
            F::BtlAiAtk_SetInput(c, p_s_1, ((3i32) as u32));
        } else {
            F::BtlAiAtk_SetInput(c, p_s_1, ((6i32) as u32));
        }
        { let v_ = 7i32; let a_ = (l_atk_4).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        break 'f 1i32;
    }
    { let r_ = { let a0_ = (c.m.r32((p_s_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_2).wrapping_add(((8i32) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_3).wrapping_add(((8i32) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_9).wrapping_add(r_); l_sum_9 = n_; };
    if (((((l_sum_9) >= (l_roll_8)) as i32)) != 0) {
        F::BtlAiAtk_SetInput(c, p_s_1, ((7i32) as u32));
        { let v_ = 8i32; let a_ = (l_atk_4).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        break 'f 1i32;
    }
    { let r_ = { let a0_ = (c.m.r32((p_s_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_2).wrapping_add(((9i32) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_3).wrapping_add(((9i32) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_9).wrapping_add(r_); l_sum_9 = n_; };
    if (((((l_sum_9) >= (l_roll_8)) as i32)) != 0) {
        F::BtlAiAtk_SetInput(c, p_s_1, ((8i32) as u32));
        { let v_ = 9i32; let a_ = (l_atk_4).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        break 'f 1i32;
    }
    { let r_ = { let a0_ = (c.m.r32((p_s_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_2).wrapping_add(((10i32) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_3).wrapping_add(((10i32) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_9).wrapping_add(r_); l_sum_9 = n_; };
    if (((((l_sum_9) >= (l_roll_8)) as i32)) != 0) {
        { let v_ = 10i32; let a_ = (l_atk_4).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        { let r_ = 2i32; let a_ = (l_atk_4).wrapping_add(16); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        break 'f 1i32;
    }
    { let v_ = 11i32; let a_ = (l_atk_4).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiAtk_PickOption", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiAtk_PickLastStep(c: &mut Ctx, mut p_s_1: u32, mut p_lo_2: u32, mut p_hi_3: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiAtk_PickLastStep", &PROBE_N, &[(p_s_1) as u32, (p_lo_2) as u32, (p_hi_3) as u32], false); }
    let mut l_e_4: u32 = 0;
    let mut l_act_5: u32 = 0;
    let mut l_atk_6: u32 = 0;
    let mut l_roll_7: i32 = 0;
    let mut l_sum_8: i32 = 0;
    let r_ = 'f: {
    l_e_4 = (((p_s_1).wrapping_add(40)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(8));
    l_act_5 = (p_s_1).wrapping_add(40);
    l_atk_6 = (p_s_1).wrapping_add(576);
    l_roll_7 = (((F::Rand_Range(c, ((100i32) as u32))).wrapping_add(((1i32) as u32))) as i32);
    l_sum_8 = { let a0_ = (c.m.r32((p_s_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_2).wrapping_add(((0i32) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_3).wrapping_add(((0i32) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) };
    if (((((l_sum_8) >= (l_roll_7)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (l_act_5).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 33i32; let a_ = l_e_4; c.m.w32(a_, (v_) as u32); };
        { let v_ = ((0i32) as i8); let a_ = (l_e_4).wrapping_add(4); c.m.w8(a_, (v_) as u8); };
        { let v_ = 1i32; let a_ = (l_act_5).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        break 'f 1i32;
    }
    { let r_ = { let a0_ = (c.m.r32((p_s_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_2).wrapping_add(((1i32) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_3).wrapping_add(((1i32) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_8).wrapping_add(r_); l_sum_8 = n_; };
    if (((((l_sum_8) >= (l_roll_7)) as i32)) != 0) {
        { let r_ = 2i32; let a_ = (l_atk_6).wrapping_add(4); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiAtk_PickLastStep", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCombo_Init(c: &mut Ctx, mut p_s_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCombo_Init", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_act_2: u32 = 0;
    let mut l_tbl_3: u32 = 0;
    let mut l_cls_4: i32 = 0;
    let r_ = 'f: {
    l_act_2 = (p_s_1).wrapping_add(40);
    l_tbl_3 = { let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) };
    l_cls_4 = (({ let a_ = ((l_tbl_3).wrapping_add(904)).wrapping_add((({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetUnk974(c, a0_) }) as u32)); (c.m.r8(a_) as i8) }) as i32);
    F::BtlAiPad_Set(c, p_s_1, 0i32, 8i32, 0i32, ((0i32) as i16), F32(0x00000000), F32(0x00000000));
    if ((((((((l_cls_4).wrapping_sub(13i32)) as u32)) < (((2i32) as u32))) as i32)) != 0) {
        { let v_ = 1i32; let a_ = (l_act_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        F::BtlAiCombo_Start(c, p_s_1);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCombo_Init", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCombo_Start(c: &mut Ctx, mut p_s_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCombo_Start", &PROBE_N, &[(p_s_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_tbl_2: u32 = 0;
    let mut l_atk_3: u32 = 0;
    let mut l_picked_4: i32 = 0;
    let mut l_e_5: u32 = 0;
    let mut l_act_6: u32 = 0;
    let mut l_step_7: i32 = 0;
    let r_ = 'f: {
        l_tbl_2 = { let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) };
        l_atk_3 = (p_s_1).wrapping_add(576);
        l_picked_4 = 0i32;
        l_e_5 = (((p_s_1).wrapping_add(40)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(8));
        l_act_6 = (p_s_1).wrapping_add(40);
        { let v_ = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetUnk974(c, a0_) }; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = (((c.m.r32(p_s_1) as i32)) ^ (1i32)); F::BtlCharApi_GetUnk974(c, a0_) }; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = (({ let a_ = (((l_tbl_2).wrapping_add(((8i32) as u32))).wrapping_add(896)).wrapping_add((((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u32)); c.m.r8(a_) }) as i8); let a_ = (fp.wrapping_add(8)).wrapping_add(((0i32) as u32)); c.m.w8(a_, (v_) as u8); };
        { let v_ = (({ let a_ = (((l_tbl_2).wrapping_add(((8i32) as u32))).wrapping_add(896)).wrapping_add((((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)); c.m.r8(a_) }) as i8); let a_ = (fp.wrapping_add(8)).wrapping_add(((1i32) as u32)); c.m.w8(a_, (v_) as u8); };
        l_step_7 = (if ((((((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) < (60i32)) as i32)) != 0) { ((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)).wrapping_sub(55i32) } else { ((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)).wrapping_sub(60i32) });
        { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlAiAtk_Reset(c, a0_, l_atk_3) };
        { let v_ = 0i32; let a_ = (l_atk_3).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
        { let v_ = l_step_7; let a_ = (l_atk_3).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
        if ((((!(((((((((((((c.m.r8((fp.wrapping_add(8)).wrapping_add(((0i32) as u32))) as i8)) as u8)) as i32)).wrapping_sub(13i32)) as u32)) < (((2i32) as u32))) as i32)) != 0)) as i32)) != 0) {
            { let v_ = 3i32; let a_ = (l_act_6).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            F::BtlAiAtk_End(c, p_s_1);
            break 'f;
        }
        if (((((l_step_7) == (4i32)) as i32)) != 0) {
            if ((((((((c.m.r8((fp.wrapping_add(8)).wrapping_add(((0i32) as u32))) as i8)) as i32)) == (13i32)) as i32)) != 0) {
                { let _ = { let a1_ = (c.m.r32((p_s_1).wrapping_add(24))).wrapping_add(((68i32) as u32)); let a2_ = (c.m.r32((p_s_1).wrapping_add(24))).wrapping_add(((316i32) as u32)); F::BtlAiAtk_PickLastStep(c, p_s_1, a1_, a2_) }; };
            }
            if ((((((c.m.r32(l_e_5) as i32)) == (33i32)) as i32)) != 0) {
                break 'f;
            }
        } else {
            if ((((((((c.m.r8((fp.wrapping_add(8)).wrapping_add(((1i32) as u32))) as i8)) as i32)) == (26i32)) as i32)) != 0) {
                { let v_ = 2i32; let a_ = (l_atk_3).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                l_picked_4 = { let a1_ = (c.m.r32((p_s_1).wrapping_add(24))).wrapping_add(((136i32) as u32)); let a2_ = (c.m.r32((p_s_1).wrapping_add(24))).wrapping_add(((384i32) as u32)); F::BtlAiAtk_PickOption(c, p_s_1, a1_, a2_) };
            } else {
                if (((((((((((((c.m.r8((fp.wrapping_add(8)).wrapping_add(((1i32) as u32))) as i8)) as i32)).wrapping_sub(13i32)) as u8)) as i32)) < (2i32)) as i32)) != 0) {
                    { let v_ = 1i32; let a_ = (l_atk_3).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                    l_picked_4 = { let a1_ = (c.m.r32((p_s_1).wrapping_add(24))).wrapping_add(((184i32) as u32)); let a2_ = (c.m.r32((p_s_1).wrapping_add(24))).wrapping_add(((432i32) as u32)); F::BtlAiAtk_PickOption(c, p_s_1, a1_, a2_) };
                } else {
                    if ((((((((c.m.r8((fp.wrapping_add(8)).wrapping_add(((0i32) as u32))) as i8)) as i32)) == (13i32)) as i32)) != 0) {
                        l_picked_4 = { let a1_ = (c.m.r32((p_s_1).wrapping_add(24))).wrapping_add(((20i32) as u32)); let a2_ = (c.m.r32((p_s_1).wrapping_add(24))).wrapping_add(((268i32) as u32)); F::BtlAiAtk_PickOption(c, p_s_1, a1_, a2_) };
                    } else {
                        if ((((((((c.m.r8((fp.wrapping_add(8)).wrapping_add(((0i32) as u32))) as i8)) as i32)) == (14i32)) as i32)) != 0) {
                            l_picked_4 = { let a1_ = (c.m.r32((p_s_1).wrapping_add(24))).wrapping_add(((80i32) as u32)); let a2_ = (c.m.r32((p_s_1).wrapping_add(24))).wrapping_add(((328i32) as u32)); F::BtlAiAtk_PickOption(c, p_s_1, a1_, a2_) };
                        }
                    }
                }
            }
            if (((((l_picked_4) == (0i32)) as i32)) != 0) {
                { let r_ = 8i32; let a_ = (l_atk_3).wrapping_add(4); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
        { let v_ = 2i32; let a_ = (l_act_6).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        F::BtlAiPad_Clear(c, (p_s_1).wrapping_add(616), 0i32);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCombo_Start", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiAtk_Run(c: &mut Ctx, mut p_s_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiAtk_Run", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_act_2: u32 = 0;
    let mut l_atk_3: u32 = 0;
    let mut l_tbl_4: u32 = 0;
    let mut l_state_5: i32 = 0;
    let mut l_held_6: F32 = F32(0);
    let mut l_e_7: u32 = 0;
    let mut l_cls_8: i32 = 0;
    let r_ = 'f: {
    l_act_2 = (p_s_1).wrapping_add(40);
    l_atk_3 = (p_s_1).wrapping_add(576);
    l_tbl_4 = { let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) };
    l_state_5 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetUnk974(c, a0_) };
    l_held_6 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetChargeRate(c, a0_) };
    l_e_7 = ((l_act_2).wrapping_add(((((c.m.r32((l_act_2).wrapping_add(4)) as i32)).wrapping_shl((3i32) as u32)) as u32))).wrapping_add(((12i32) as u32));
    l_cls_8 = (((c.m.r8(((l_tbl_4).wrapping_add(904)).wrapping_add(((l_state_5) as u32))) as i8)) as i32);
    F::BtlAiPad_Clear(c, (p_s_1).wrapping_add(616), 1i32);
    if (((((c.m.r32((l_atk_3).wrapping_add(16)) as i32)) & (2i32))) != 0) {
        if (((((l_cls_8) != (0i32)) as i32)) != 0) {
            break 'f;
        }
        if ((((({ let a_ = (l_atk_3).wrapping_add(24); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
            break 'f;
        }
        { let v_ = 3i32; let a_ = (l_act_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        break 'f;
    }
    if (((((l_state_5) != ((c.m.r32((l_atk_3).wrapping_add(12)) as i32))) as i32)) != 0) {
        if (((((c.m.r32(l_atk_3) as i32)) & (((4i32) | (8i32))))) != 0) {
            if (((((((rt::fle(F32(c.m.r32((l_atk_3).wrapping_add(8))), l_held_6) as i32)) != 0) || (((((l_cls_8) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                { let v_ = 0i32; let a_ = l_atk_3; c.m.w32(a_, (v_) as u32); };
            }
            if (((((l_cls_8) == (8i32)) as i32)) != 0) {
                { let v_ = 3i32; let a_ = (l_act_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            }
        } else {
            if (((((c.m.r32((l_atk_3).wrapping_add(16)) as i32)) & (1i32))) != 0) {
                if (((((l_cls_8) == (11i32)) as i32)) != 0) {
                    { let v_ = 3i32; let a_ = (l_act_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                }
            } else {
                if ((((((c.m.r32(l_e_7) as i32)) == (26i32)) as i32)) != 0) {
                    { let v_ = 3i32; let a_ = (l_act_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let v_ = 1i32; let a_ = (l_act_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                }
                break 'f;
            }
        }
    } else {
        if ((((((c.m.r32((l_atk_3).wrapping_add(20)) as i32)) > (0i32)) as i32)) != 0) {
            { let a_ = (l_atk_3).wrapping_add(20); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
        } else {
            { let v_ = 3i32; let a_ = (l_act_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        }
    }
    { let a1_ = (c.m.r32(l_atk_3) as i32); let a2_ = (c.m.r32((l_atk_3).wrapping_add(4)) as i32); F::BtlAiPad_Set(c, p_s_1, a1_, a2_, 0i32, ((0i32) as i16), F32(0x00000000), F32(0x00000000)) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiAtk_Run", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiAtk_End(c: &mut Ctx, mut p_s_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiAtk_End", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_act_2: u32 = 0;
    let r_ = 'f: {
    l_act_2 = (p_s_1).wrapping_add(40);
    { let v_ = 0i32; let a_ = (l_act_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (l_act_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiAtk_End", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCombo_Dispatch(c: &mut Ctx, mut p_s_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCombo_Dispatch", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_act_2: u32 = 0;
    let r_ = 'f: {
    l_act_2 = (p_s_1).wrapping_add(40);
    if (({ let a1_ = { let a_ = ((l_act_2).wrapping_add(((((c.m.r32((l_act_2).wrapping_add(4)) as i32)).wrapping_shl((3i32) as u32)) as u32))).wrapping_add(((12i32) as u32)); (c.m.r32(a_) as i32) }; F::BtlAiSeq_IsInterrupted(c, p_s_1, a1_) }) != 0) {
        { let v_ = 3i32; let a_ = (l_act_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    }
    if ((F::BtlAiSense_GetRange(c, p_s_1)) != 0) {
        { let v_ = 3i32; let a_ = (l_act_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    }
    { let fp_ = { let a_ = (sym::gBtlAiComboPhases).wrapping_add(((((c.m.r32((l_act_2).wrapping_add(8)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; { let _ = D::call(c, fp_, &[((p_s_1) as u64)]); } };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCombo_Dispatch", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiFollow_Init(c: &mut Ctx, mut p_s_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiFollow_Init", &PROBE_N, &[(p_s_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = 1i32; let a_ = ((p_s_1).wrapping_add(40)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    F::BtlAiFollow_Start(c, p_s_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiFollow_Init", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiFollow_Start(c: &mut Ctx, mut p_s_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiFollow_Start", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_act_2: u32 = 0;
    let mut l_atk_3: u32 = 0;
    let mut l_roll_4: i32 = 0;
    let mut l_action_5: i32 = 0;
    let mut l_dir_6: i32 = 0;
    let mut l_hi_7: u32 = 0;
    let mut l_lo_8: u32 = 0;
    let mut l_sum_9: i32 = 0;
    let r_ = 'f: {
    l_act_2 = (p_s_1).wrapping_add(40);
    l_atk_3 = (p_s_1).wrapping_add(576);
    l_roll_4 = (((F::Rand_Range(c, ((100i32) as u32))).wrapping_add(((1i32) as u32))) as i32);
    l_action_5 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetAction(c, a0_) };
    l_dir_6 = ((F::Rand_Range(c, ((3i32) as u32))) as i32);
    if (((((l_action_5) == (31i32)) as i32)) != 0) {
        l_lo_8 = (c.m.r32((p_s_1).wrapping_add(24))).wrapping_add(((72i32) as u32));
        l_hi_7 = (c.m.r32((p_s_1).wrapping_add(24))).wrapping_add(((320i32) as u32));
    } else {
        l_lo_8 = (c.m.r32((p_s_1).wrapping_add(24))).wrapping_add(((128i32) as u32));
        l_hi_7 = (c.m.r32((p_s_1).wrapping_add(24))).wrapping_add(((376i32) as u32));
    }
    { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlAiAtk_Reset(c, a0_, l_atk_3) };
    { let v_ = 2i32; let a_ = (l_act_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    F::BtlAiPad_Clear(c, (p_s_1).wrapping_add(616), 0i32);
    l_sum_9 = { let a0_ = (c.m.r32((p_s_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((l_lo_8).wrapping_add(((0i32) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((l_hi_7).wrapping_add(((0i32) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) };
    if (((((l_sum_9) >= (l_roll_4)) as i32)) != 0) {
        F::BtlAiAtk_SetInput(c, p_s_1, ((0i32) as u32));
        break 'f;
    }
    { let r_ = { let a0_ = (c.m.r32((p_s_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((l_lo_8).wrapping_add(((1i32) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((l_hi_7).wrapping_add(((1i32) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_9).wrapping_add(r_); l_sum_9 = n_; };
    if (((((l_sum_9) >= (l_roll_4)) as i32)) != 0) {
        F::BtlAiAtk_SetInput(c, p_s_1, ((1i32) as u32));
        break 'f;
    }
    { let r_ = { let a0_ = (c.m.r32((p_s_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((l_lo_8).wrapping_add(((2i32) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((l_hi_7).wrapping_add(((2i32) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_9).wrapping_add(r_); l_sum_9 = n_; };
    if (((((l_sum_9) >= (l_roll_4)) as i32)) != 0) {
        F::BtlAiAtk_SetInput(c, p_s_1, ((2i32) as u32));
        break 'f;
    }
    { let r_ = { let a0_ = (c.m.r32((p_s_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((l_lo_8).wrapping_add(((3i32) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((l_hi_7).wrapping_add(((3i32) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_9).wrapping_add(r_); l_sum_9 = n_; };
    if (((((l_sum_9) >= (l_roll_4)) as i32)) != 0) {
        { let r_ = 8i32; let a_ = (l_atk_3).wrapping_add(4); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        if (((((l_action_5) == (31i32)) as i32)) != 0) {
            's1: {
                let sv10: i32 = l_dir_6;
                match sv10 {
                    0i32 => {
                        { let r_ = 16i32; let a_ = l_atk_3; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        break 's1;
                    }
                    1i32 => {
                        { let r_ = 32i32; let a_ = l_atk_3; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
            { let r_ = 4i32; let a_ = (l_atk_3).wrapping_add(16); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        break 'f;
    }
    { let r_ = { let a0_ = (c.m.r32((p_s_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((l_lo_8).wrapping_add(((4i32) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((l_hi_7).wrapping_add(((4i32) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_9).wrapping_add(r_); l_sum_9 = n_; };
    if (((((l_sum_9) >= (l_roll_4)) as i32)) != 0) {
        F::BtlAiAtk_SetInput(c, p_s_1, ((0i32) as u32));
        break 'f;
    }
    { let r_ = 8i32; let a_ = (l_atk_3).wrapping_add(4); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiFollow_Start", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiFollow_End(c: &mut Ctx, mut p_s_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiFollow_End", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_act_2: u32 = 0;
    let mut l_tbl_3: u32 = 0;
    let mut l_cls_4: i32 = 0;
    let mut l_atk_5: u32 = 0;
    let mut l_e_6: u32 = 0;
    let r_ = 'f: {
    l_act_2 = (p_s_1).wrapping_add(40);
    l_tbl_3 = { let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) };
    l_cls_4 = (({ let a_ = ((l_tbl_3).wrapping_add(904)).wrapping_add((({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetUnk974(c, a0_) }) as u32)); (c.m.r8(a_) as i8) }) as i32);
    l_atk_5 = (p_s_1).wrapping_add(576);
    l_e_6 = (((p_s_1).wrapping_add(40)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(8));
    if (((((l_cls_4) != (11i32)) as i32)) != 0) {
        if (((((c.m.r32((l_atk_5).wrapping_add(16)) as i32)) & (4i32))) != 0) {
            { let v_ = 0i32; let a_ = (l_act_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = 25i32; let a_ = l_e_6; c.m.w32(a_, (v_) as u32); };
            { let v_ = ((0i32) as i8); let a_ = (l_e_6).wrapping_add(4); c.m.w8(a_, (v_) as u8); };
            { let v_ = 1i32; let a_ = (l_act_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 0i32; let a_ = (l_act_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (l_act_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiFollow_End", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiFollow_Dispatch(c: &mut Ctx, mut p_s_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiFollow_Dispatch", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_act_2: u32 = 0;
    let r_ = 'f: {
    l_act_2 = (p_s_1).wrapping_add(40);
    if (({ let a1_ = { let a_ = ((l_act_2).wrapping_add(((((c.m.r32((l_act_2).wrapping_add(4)) as i32)).wrapping_shl((3i32) as u32)) as u32))).wrapping_add(((12i32) as u32)); (c.m.r32(a_) as i32) }; F::BtlAiSeq_IsInterrupted(c, p_s_1, a1_) }) != 0) {
        { let v_ = 3i32; let a_ = (l_act_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    }
    { let fp_ = { let a_ = (sym::gBtlAiFollowPhases).wrapping_add(((((c.m.r32((l_act_2).wrapping_add(8)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; { let _ = D::call(c, fp_, &[((p_s_1) as u64)]); } };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiFollow_Dispatch", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiAct36_Init(c: &mut Ctx, mut p_s_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiAct36_Init", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_act_2: u32 = 0;
    let mut l_tbl_3: u32 = 0;
    let mut l_cls_4: i32 = 0;
    let mut l_w_5: u64 = 0;
    let r_ = 'f: {
    l_act_2 = (p_s_1).wrapping_add(40);
    l_tbl_3 = { let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) };
    l_cls_4 = (({ let a_ = ((l_tbl_3).wrapping_add(904)).wrapping_add((({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetUnk974(c, a0_) }) as u32)); (c.m.r8(a_) as i8) }) as i32);
    l_w_5 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetUnk1288(c, a0_) };
    F::BtlAiPad_Set(c, p_s_1, 0i32, 4i32, 0i32, ((0i32) as i16), F32(0x00000000), F32(0x00000000));
    if ((((!((((l_w_5) & (((4096i32) as u64)))) != 0)) as i32)) != 0) {
        { let v_ = 3i32; let a_ = (l_act_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    } else {
        if (((((l_cls_4) == (18i32)) as i32)) != 0) {
            { let v_ = 1i32; let a_ = (l_act_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            F::BtlAiAct36_Start(c, p_s_1);
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiAct36_Init", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiAct36_Start(c: &mut Ctx, mut p_s_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiAct36_Start", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_act_2: u32 = 0;
    let mut l_atk_3: u32 = 0;
    let mut l_roll_4: i32 = 0;
    let mut l_param_5: u32 = 0;
    let mut l_bit_6: i32 = 0;
    let mut l_row_7: u32 = 0;
    let mut l_canPress_8: i32 = 0;
    let mut l_lo_9: u32 = 0;
    let mut l_canHold_10: i32 = 0;
    let mut l_hi_11: u32 = 0;
    let mut l_w_12: u64 = 0;
    let r_ = 'f: {
    l_act_2 = (p_s_1).wrapping_add(40);
    l_atk_3 = (p_s_1).wrapping_add(576);
    l_roll_4 = (((F::Rand_Range(c, ((100i32) as u32))).wrapping_add(((1i32) as u32))) as i32);
    l_param_5 = c.m.r32((p_s_1).wrapping_add(24));
    l_bit_6 = (((((((c.m.r8((l_param_5).wrapping_add(((0i32) as u32)))) as i32)) & (2i32))) != (0i32)) as i32);
    l_row_7 = (l_param_5).wrapping_add(((({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetBlastShots(c, a0_) }).wrapping_mul(2i32)) as u32));
    l_lo_9 = (l_row_7).wrapping_add(((230i32) as u32));
    l_hi_11 = (l_row_7).wrapping_add(((478i32) as u32));
    l_canPress_8 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetBlastRoom(c, a0_) };
    l_canHold_10 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetBlastRoomB(c, a0_) };
    l_w_12 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetUnk1288(c, a0_) };
    { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlAiAtk_Reset(c, a0_, l_atk_3) };
    { let v_ = 2i32; let a_ = (l_act_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    F::BtlAiPad_Clear(c, (p_s_1).wrapping_add(616), 0i32);
    if ((((((((({ let a0_ = (c.m.r32((p_s_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((l_lo_9).wrapping_add(((0i32) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((l_hi_11).wrapping_add(((0i32) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }) < (l_roll_4)) as i32)) != 0) || (((((((((l_canPress_8) == (0i32)) as i32)) != 0) && (((((l_canHold_10) == (0i32)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
        { let v_ = 3i32; let a_ = (l_act_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    } else {
        if ((((((((((((({ let a0_ = (c.m.r32((p_s_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((l_lo_9).wrapping_add(((1i32) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((l_hi_11).wrapping_add(((1i32) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }) >= (l_roll_4)) as i32)) != 0) && (((((l_canHold_10) != (0i32)) as i32)) != 0)) as i32)) != 0) && ((((l_w_12) & (((8192i32) as u64)))) != 0)) as i32)) != 0) {
            { let v_ = { let a1_ = (c.m.r8(((sym::btl_ai_act__sAiActChargeRows).wrapping_add((((6i32) as u32)).wrapping_mul(3))).wrapping_add(((l_bit_6) as u32))) as i8); let a2_ = (c.m.r8(((sym::btl_ai_act__sAiActChargeRows).wrapping_add((((6i32) as u32)).wrapping_mul(3))).wrapping_add(((2i32) as u32))) as i8); F::BtlAiAtk_RollCharge(c, p_s_1, a1_, a2_) }; let a_ = (l_atk_3).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let r_ = 4i32; let a_ = l_atk_3; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        } else {
            if (((((((((l_canPress_8) == (0i32)) as i32)) != 0) || ((((!((((l_w_12) & (((4096i32) as u64)))) != 0)) as i32)) != 0)) as i32)) != 0) {
                { let v_ = 3i32; let a_ = (l_act_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            } else {
                { let r_ = 4i32; let a_ = (l_atk_3).wrapping_add(4); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiAct36_Start", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiAct36_Dispatch(c: &mut Ctx, mut p_s_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiAct36_Dispatch", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_act_2: u32 = 0;
    let r_ = 'f: {
    l_act_2 = (p_s_1).wrapping_add(40);
    if (({ let a1_ = { let a_ = ((l_act_2).wrapping_add(((((c.m.r32((l_act_2).wrapping_add(4)) as i32)).wrapping_shl((3i32) as u32)) as u32))).wrapping_add(((12i32) as u32)); (c.m.r32(a_) as i32) }; F::BtlAiSeq_IsInterrupted(c, p_s_1, a1_) }) != 0) {
        { let v_ = 3i32; let a_ = (l_act_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    }
    { let fp_ = { let a_ = (sym::gBtlAiAct36Phases).wrapping_add(((((c.m.r32((l_act_2).wrapping_add(8)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; { let _ = D::call(c, fp_, &[((p_s_1) as u64)]); } };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiAct36_Dispatch", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSense_GetRange(c: &mut Ctx, mut p_s_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_GetRange", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_flags_2: u64 = 0;
    let r_ = 'f: {
    l_flags_2 = c.m.r64((p_s_1).wrapping_add(704));
    if ((((l_flags_2) & (((16i32) as u64)))) != 0) {
        break 'f 0i32;
    }
    if ((((l_flags_2) & (((32i32) as u64)))) != 0) {
        break 'f 1i32;
    }
    if ((((l_flags_2) & (((64i32) as u64)))) != 0) {
        break 'f 2i32;
    }
    break 'f (1i32).wrapping_neg();
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_GetRange", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSense_CheckBit56(c: &mut Ctx, mut p_s_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit56", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_st_2: u32 = 0;
    let mut l_cost_3: i32 = 0;
    let mut l_gauge_4: i32 = 0;
    let mut l_hp_5: i32 = 0;
    let mut l_ratio_6: F32 = F32(0);
    let mut l_limit_7: i32 = 0;
    let r_ = 'f: {
    l_st_2 = (p_s_1).wrapping_add(704);
    l_cost_3 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetTransformCost(c, a0_) };
    l_gauge_4 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlSide_GetBlast(c, a0_) };
    l_hp_5 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetHp(c, a0_) };
    l_ratio_6 = rt::fdiv(rt::i2f(l_hp_5), rt::i2f({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetHpMax(c, a0_) }));
    l_limit_7 = { let a0_ = (c.m.r32((p_s_1).wrapping_add(8)) as i32); let a1_ = (((({ let a_ = (c.m.r32((p_s_1).wrapping_add(24))).wrapping_add(((15i32) as u32)); c.m.r8(a_) }) as i8)) as i32); let a2_ = (((({ let a_ = (c.m.r32((p_s_1).wrapping_add(24))).wrapping_add(((263i32) as u32)); c.m.r8(a_) }) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) };
    if ((((!(({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetUnk1290(c, a0_) }) != 0)) as i32)) != 0) {
        if (({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_IsChangingForm(c, a0_) }) != 0) {
            { let v_ = 600i32; let a_ = (l_st_2).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
            break 'f 0i32;
        }
        if ((((({ let a_ = (l_st_2).wrapping_add(24); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) > (0i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        { let v_ = (1i32).wrapping_neg(); let a_ = (l_st_2).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
        if (((rt::flt(rt::i2f(l_limit_7), rt::fmul(l_ratio_6, F32(0x42c80000))) as i32)) != 0) {
            break 'f 0i32;
        }
    }
    if ((((!(({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_CanTransform(c, a0_) }) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((l_cost_3) == (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((l_gauge_4) < (l_cost_3)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit56", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSense_CheckBit57(c: &mut Ctx, mut p_s_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit57", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_st_2: u32 = 0;
    let mut l_hp_3: i32 = 0;
    let mut l_ratio_4: F32 = F32(0);
    let mut l_limit_5: i32 = 0;
    let r_ = 'f: {
    l_st_2 = (p_s_1).wrapping_add(704);
    l_hp_3 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetHp(c, a0_) };
    l_ratio_4 = rt::fdiv(rt::i2f(l_hp_3), rt::i2f({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetHpMax(c, a0_) }));
    l_limit_5 = ({ let a0_ = (c.m.r32((p_s_1).wrapping_add(8)) as i32); let a1_ = (((({ let a_ = (c.m.r32((p_s_1).wrapping_add(24))).wrapping_add(((15i32) as u32)); c.m.r8(a_) }) as i8)) as i32); let a2_ = (((({ let a_ = (c.m.r32((p_s_1).wrapping_add(24))).wrapping_add(((263i32) as u32)); c.m.r8(a_) }) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }).wrapping_add(20i32);
    if (({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_IsChangingForm(c, a0_) }) != 0) {
        { let v_ = 600i32; let a_ = (l_st_2).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
        break 'f 0i32;
    }
    if ((((!(({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_CanFuse(c, a0_) }) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (l_st_2).wrapping_add(28); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) > (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let v_ = (1i32).wrapping_neg(); let a_ = (l_st_2).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
    if (((rt::flt(rt::i2f(l_limit_5), rt::fmul(l_ratio_4, F32(0x42c80000))) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit57", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSense_CheckBit58(c: &mut Ctx, mut p_s_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit58", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_st_2: u32 = 0;
    let mut l_hp_3: i32 = 0;
    let mut l_gauge_4: i32 = 0;
    let mut l_type_5: u32 = 0;
    let mut l_limit_6: i32 = 0;
    let mut l_max_7: i32 = 0;
    let mut l_score_8: i32 = 0;
    let mut l_r_9: F32 = F32(0);
    let r_ = 'f: {
    l_st_2 = (p_s_1).wrapping_add(704);
    l_hp_3 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetHp(c, a0_) };
    l_gauge_4 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlSide_GetKi(c, a0_) };
    l_type_5 = { let a_ = (({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(200)).wrapping_add(((((c.m.r32((p_s_1).wrapping_add(4)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    l_limit_6 = { let a0_ = (c.m.r32((p_s_1).wrapping_add(8)) as i32); let a1_ = ((c.m.r8((l_type_5).wrapping_add(((685i32) as u32)))) as i32); let a2_ = ((c.m.r8((l_type_5).wrapping_add(((1389i32) as u32)))) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) };
    if (({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlSide_IsPoweredUp(c, a0_) }) != 0) {
        break 'f 0i32;
    }
    if ((((!(({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_CanSwitch(c, a0_) }) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (l_st_2).wrapping_add(32); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) > (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let v_ = (1i32).wrapping_neg(); let a_ = (l_st_2).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
    l_max_7 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetHpMax(c, a0_) };
    l_r_9 = rt::fdiv(rt::i2f(l_hp_3), rt::i2f(l_max_7));
    l_score_8 = rt::f2i(rt::fmul(rt::fsub(F32(0x3f800000), l_r_9), F32(0x428c0000)));
    l_r_9 = rt::fdiv(rt::i2f(l_gauge_4), F32(0x47c35000));
    { let r_ = rt::f2i(rt::fmul(rt::fsub(F32(0x3f800000), l_r_9), F32(0x41f00000))); let n_ = (l_score_8).wrapping_add(r_); l_score_8 = n_; };
    if (((((l_score_8) < (l_limit_6)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit58", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSense_CheckBit60(c: &mut Ctx, mut p_s_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit60", &PROBE_N, &[(p_s_1) as u32], false); }
    let r_ = 'f: {
    if ((((({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetArmorBreakLevel(c, a0_) }) < (4i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit60", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSense_CheckBit61(c: &mut Ctx, mut p_s_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit61", &PROBE_N, &[(p_s_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((({ let a0_ = (((c.m.r32(p_s_1) as i32)) ^ (1i32)); F::BtlCharApi_GetParamByte2(c, a0_) }) == (4i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit61", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSense_IsBehindOpponent(c: &mut Ctx, mut p_s_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_IsBehindOpponent", &PROBE_N, &[(p_s_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let r_ = 'f: {
        { let a0_ = (((c.m.r32(p_s_1) as i32)) ^ (1i32)); F::BtlCharApi_GetRot(c, a0_, fp) };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); F::Port_sinf(c, a0_) }; let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); F::Port_cosf(c, a0_) }; let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec3_Normalize(c, fp.wrapping_add(16), fp.wrapping_add(16));
        { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetPos(c, a0_, fp.wrapping_add(32)) };
        { let a0_ = (((c.m.r32(p_s_1) as i32)) ^ (1i32)); F::BtlCharApi_GetPos(c, a0_, fp.wrapping_add(48)) };
        F::Vec3_Sub(c, fp, fp.wrapping_add(48), fp.wrapping_add(32));
        { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        F::Vec3_Normalize(c, fp.wrapping_add(64), fp);
        if (((rt::flt(F32(0x00000000), F::Vec3_Dot(c, fp.wrapping_add(16), fp.wrapping_add(64))) as i32)) != 0) {
            break 'f 1i32;
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_IsBehindOpponent", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSense_CheckBit32(c: &mut Ctx, mut p_s_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit32", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_st_2: u32 = 0;
    let mut l_progress_3: F32 = F32(0);
    let mut l_attr_4: u32 = 0;
    let mut l_kind_5: i32 = 0;
    let mut l_sub_6: i32 = 0;
    let mut l_radius_7: F32 = F32(0);
    let mut l_speed_8: F32 = F32(0);
    let mut l_ready_9: i32 = 0;
    let mut l_ret_10: i32 = 0;
    let r_ = 'f: {
    l_st_2 = (p_s_1).wrapping_add(704);
    l_progress_3 = { let a0_ = (((c.m.r32(p_s_1) as i32)) ^ (1i32)); F::BtlCharApi_GetTechniqueProgress(c, a0_) };
    l_attr_4 = { let a0_ = (((c.m.r32(p_s_1) as i32)) ^ (1i32)); ((((F::BtlCharApi_GetAttackAttr(c, a0_)) as i64 as u64)) as u32) };
    l_kind_5 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetOppSkillKind(c, a0_) };
    l_sub_6 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetOppMoveKind(c, a0_) };
    l_radius_7 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetRadius(c, a0_) };
    l_speed_8 = { let a0_ = (((c.m.r32(p_s_1) as i32)) ^ (1i32)); F::BtlCharApi_GetSpeed(c, a0_) };
    l_ready_9 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_IsUnk106CLow(c, a0_) };
    if ((((((((((((l_kind_5).wrapping_sub(1i32)) as u32)) < (((2i32) as u32))) as i32)) != 0) || (((((l_kind_5) == (5i32)) as i32)) != 0)) as i32)) != 0) {
        if (((rt::flt(rt::fadd(l_radius_7, l_speed_8), { let a_ = (c.m.r32(sym::gBtlAi)).wrapping_add(4); F32(c.m.r32(a_)) }) as i32)) != 0) {
            break 'f 0i32;
        }
    } else {
        if ((((((((((((l_kind_5).wrapping_sub(3i32)) as u32)) < (((2i32) as u32))) as i32)) != 0) || (((((l_kind_5) == (6i32)) as i32)) != 0)) as i32)) != 0) {
            if ((((({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_FindIncomingBlast(c, a0_, 2i32) }) == ((1i32).wrapping_neg())) as i32)) != 0) {
                break 'f 0i32;
            }
        } else {
            if (((((l_sub_6) == (0i32)) as i32)) != 0) {
                if ((((({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_FindIncomingBlast(c, a0_, 3i32) }) == ((1i32).wrapping_neg())) as i32)) != 0) {
                    break 'f 0i32;
                }
            } else {
                if ((((!((((l_attr_4) & (((1i32) as u32)))) != 0)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if (((((((rt::flt(l_progress_3, F32(0x3f800000)) as i32)) != 0) || (((rt::fle(F32(0x40400000), l_progress_3) as i32)) != 0)) as i32)) != 0) {
                    break 'f 0i32;
                }
            }
        }
    }
    l_ret_10 = (((l_ready_9) != (0i32)) as i32);
    if (((((c.m.r32((l_st_2).wrapping_add(20)) as i32)) & (64i32))) != 0) {
        l_ret_10 = 0i32;
    }
    break 'f l_ret_10;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit32", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSense_CheckSwitch(c: &mut Ctx, mut p_s_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckSwitch", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_ret_2: i32 = 0;
    let r_ = 'f: {
    l_ret_2 = ((({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetPromptButtons(c, a0_) }) != (0i32)) as i32);
    if (((((c.m.r32(((p_s_1).wrapping_add(704)).wrapping_add(20)) as i32)) & (65536i32))) != 0) {
        l_ret_2 = 0i32;
    }
    break 'f l_ret_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckSwitch", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSense_CheckAct10(c: &mut Ctx, mut p_s_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckAct10", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_st_2: u32 = 0;
    let mut l_e_3: u32 = 0;
    let mut l_m_4: u32 = 0;
    let r_ = 'f: {
    l_st_2 = (p_s_1).wrapping_add(704);
    l_e_3 = (((p_s_1).wrapping_add(40)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(8));
    l_m_4 = (p_s_1).wrapping_add(192);
    if ((((((((((c.m.r32(((p_s_1).wrapping_add(40)).wrapping_add(4)) as i32)) == (0i32)) as i32)) != 0) || ((((((c.m.r32(l_e_3) as i32)) != (10i32)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((rt::fgt(rt::fmul(F32(c.m.r32((l_st_2).wrapping_add(36))), F32(0x3f000000)), { let a_ = (c.m.r32(sym::gBtlAi)).wrapping_add(4); F32(c.m.r32(a_)) }) as i32)) != 0) {
        if (((rt::fgt({ let a_ = (c.m.r32(sym::gBtlAi)).wrapping_add(4); F32(c.m.r32(a_)) }, F32(c.m.r32((l_m_4).wrapping_add((((1i32) as u32)).wrapping_mul(4))))) as i32)) != 0) {
            break 'f 2i32;
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckAct10", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSense_Basic(c: &mut Ctx, mut p_s_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_Basic", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_st_2: u32 = 0;
    let mut l_m_3: u32 = 0;
    let mut l_act_4: u32 = 0;
    let mut l_seq_5: i32 = 0;
    let mut l_ratio_6: F32 = F32(0);
    let mut l_tbl_7: u32 = 0;
    let mut l_cls_8: i32 = 0;
    let mut l_r_9: i32 = 0;
    let r_ = 'f: {
    l_st_2 = (p_s_1).wrapping_add(704);
    l_m_3 = (p_s_1).wrapping_add(192);
    l_act_4 = (p_s_1).wrapping_add(40);
    l_seq_5 = F::BtlSeq_GetState(c);
    l_ratio_6 = rt::fdiv(rt::i2f({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlSide_GetKi(c, a0_) }), F32(0x47c35000));
    l_tbl_7 = { let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) };
    l_cls_8 = (({ let a_ = ((l_tbl_7).wrapping_add(904)).wrapping_add((({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetUnk974(c, a0_) }) as u32)); (c.m.r8(a_) as i8) }) as i32);
    if (((((l_seq_5) == (3i32)) as i32)) != 0) {
        { let r_ = ((2i32) as u64); let a_ = l_st_2; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    } else {
        if (((((l_seq_5) == (2i32)) as i32)) != 0) {
            { let r_ = ((4i32) as u64); let a_ = l_st_2; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
        }
    }
    if ((((!(({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_TestFlag05(c, a0_) }) != 0)) as i32)) != 0) {
        { let r_ = ((8i32) as u64); let a_ = l_st_2; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if (((rt::fgt(rt::fadd(F32(c.m.r32((l_m_3).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetRadius(c, a0_) }, F32(0x40000000))), { let a_ = (c.m.r32(sym::gBtlAi)).wrapping_add(4); F32(c.m.r32(a_)) }) as i32)) != 0) {
        { let r_ = ((16i32) as u64); let a_ = l_st_2; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    } else {
        if (((rt::fgt(rt::fadd(F32(c.m.r32((l_m_3).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), rt::fmul({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetRadius(c, a0_) }, F32(0x40000000))), { let a_ = (c.m.r32(sym::gBtlAi)).wrapping_add(4); F32(c.m.r32(a_)) }) as i32)) != 0) {
            { let r_ = ((32i32) as u64); let a_ = l_st_2; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
        } else {
            { let r_ = ((64i32) as u64); let a_ = l_st_2; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
        }
    }
    { let r_ = 1i32; let a_ = (l_st_2).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    if ((((((F::BtlAiSense_IsBehindOpponent(c, p_s_1)) != 0) && ((((c.m.r64(l_st_2)) & (((16i32) as u64)))) != 0)) as i32)) != 0) {
        { let r_ = (!(1i32)); let a_ = (l_st_2).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if (((rt::flt(rt::fmul(l_ratio_6, F32(0x42c80000)), F32(0x41a00000)) as i32)) != 0) {
        { let r_ = ((36028797018963968i64) as u64); let a_ = l_st_2; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    } else {
        if (((rt::flt(rt::fmul(l_ratio_6, F32(0x42c80000)), F32(0x42700000)) as i32)) != 0) {
            { let r_ = ((18014398509481984i64) as u64); let a_ = l_st_2; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
        } else {
            if (((rt::flt(rt::fmul(l_ratio_6, F32(0x42c80000)), F32(0x42c80000)) as i32)) != 0) {
                { let r_ = ((9007199254740992i64) as u64); let a_ = l_st_2; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
            } else {
                { let r_ = ((4503599627370496i64) as u64); let a_ = l_st_2; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
            }
        }
    }
    if ((F::BtlAiSense_CheckBit56(c, p_s_1)) != 0) {
        { let r_ = ((72057594037927936i64) as u64); let a_ = l_st_2; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((F::BtlAiSense_CheckBit57(c, p_s_1)) != 0) {
        { let r_ = ((144115188075855872i64) as u64); let a_ = l_st_2; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((F::BtlAiSense_CheckBit58(c, p_s_1)) != 0) {
        { let r_ = ((288230376151711744i64) as u64); let a_ = l_st_2; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((F::BtlAiSense_CheckBit60(c, p_s_1)) != 0) {
        { let r_ = ((1152921504606846976i64) as u64); let a_ = l_st_2; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((F::BtlAiSense_CheckBit61(c, p_s_1)) != 0) {
        { let r_ = ((2305843009213693952i64) as u64); let a_ = l_st_2; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if (((((l_cls_8) == (32i32)) as i32)) != 0) {
        { let r_ = ((2097152i32) as u64); let a_ = l_st_2; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if (({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_IsInClashA(c, a0_) }) != 0) {
        { let r_ = ((4194304i32) as u64); let a_ = l_st_2; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if (((((c.m.r32(l_act_4) as i32)) & (128i32))) != 0) {
        { let r_ = ((576460752303423488i64) as u64); let a_ = l_st_2; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((F::BtlAiSense_CheckBit32(c, p_s_1)) != 0) {
        { let r_ = ((4294967296i64) as u64); let a_ = l_st_2; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((F::BtlAiSense_CheckSwitch(c, p_s_1)) != 0) {
        { let r_ = ((134217728i32) as u64); let a_ = l_st_2; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    l_r_9 = F::BtlAiSense_CheckAct10(c, p_s_1);
    if (((((l_r_9) > (0i32)) as i32)) != 0) {
        if (((((l_r_9) == (2i32)) as i32)) != 0) {
            { let r_ = 9223372036854775808u64; let a_ = l_st_2; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
        }
        { let r_ = 8i32; let a_ = (l_st_2).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_Basic", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSense_NoteOppState(c: &mut Ctx, mut p_s_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_NoteOppState", &PROBE_N, &[(p_s_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_tbl_2: u32 = 0;
    let mut l_st_3: u32 = 0;
    let r_ = 'f: {
        l_tbl_2 = { let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) };
        l_st_3 = (p_s_1).wrapping_add(704);
        { let v_ = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetUnk974(c, a0_) }; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = (((c.m.r32(p_s_1) as i32)) ^ (1i32)); F::BtlCharApi_GetUnk974(c, a0_) }; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = (({ let a_ = (((l_tbl_2).wrapping_add(((8i32) as u32))).wrapping_add(896)).wrapping_add((((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u32)); c.m.r8(a_) }) as i8); let a_ = (fp.wrapping_add(8)).wrapping_add(((0i32) as u32)); c.m.w8(a_, (v_) as u8); };
        { let v_ = (({ let a_ = (((l_tbl_2).wrapping_add(((8i32) as u32))).wrapping_add(896)).wrapping_add((((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)); c.m.r8(a_) }) as i8); let a_ = (fp.wrapping_add(8)).wrapping_add(((1i32) as u32)); c.m.w8(a_, (v_) as u8); };
        if ((((((((((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) == (151i32)) as i32)) != 0) || ((((((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) == (393i32)) as i32)) != 0)) as i32)) != 0) {
            { let r_ = 4i32; let a_ = (l_st_3).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_NoteOppState", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSense_IsBusy(c: &mut Ctx, mut p_s_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_IsBusy", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_tbl_2: u32 = 0;
    let mut l_state_3: i32 = 0;
    let mut l_busy_4: u8 = 0;
    let mut l_cls_5: i32 = 0;
    let mut l_r_6: i32 = 0;
    let mut l_st_7: u32 = 0;
    let r_ = 'f: {
    l_tbl_2 = ({ let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) }).wrapping_add(((8i32) as u32));
    l_state_3 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetUnk974(c, a0_) };
    l_busy_4 = ((((((c.m.r8(((l_tbl_2).wrapping_add(1408)).wrapping_add(((l_state_3) as u32)))) as i32)) & (1i32))) as u8);
    l_cls_5 = (((((c.m.r8(((l_tbl_2).wrapping_add(896)).wrapping_add(((l_state_3) as u32)))) as i8)) as i32)).wrapping_sub(15i32);
    l_r_6 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_CheckUnkCAC(c, a0_) };
    l_st_7 = (p_s_1).wrapping_add(704);
    if ((l_busy_4) != 0) {
        break 'f 1i32;
    }
    if (((((((l_cls_5) as u32)) < (((3i32) as u32))) as i32)) != 0) {
        if (((((l_r_6) == (1i32)) as i32)) != 0) {
            { let v_ = 4i32; let a_ = (l_st_7).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            break 'f 1i32;
        }
        if ((((({ let a_ = (l_st_7).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) > (0i32)) as i32)) != 0) {
            break 'f 1i32;
        }
        { let v_ = 0i32; let a_ = (l_st_7).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_IsBusy", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSense_CheckBit49(c: &mut Ctx, mut p_s_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit49", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_tbl_2: u32 = 0;
    let mut l_cls_3: i32 = 0;
    let mut l_act_4: u32 = 0;
    let mut l_e_5: u32 = 0;
    let r_ = 'f: {
    l_tbl_2 = { let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) };
    l_cls_3 = (({ let a_ = ((l_tbl_2).wrapping_add(904)).wrapping_add((({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetUnk974(c, a0_) }) as u32)); (c.m.r8(a_) as i8) }) as i32);
    l_act_4 = (p_s_1).wrapping_add(40);
    l_e_5 = (((p_s_1).wrapping_add(40)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(8));
    if (((((l_cls_3) == (6i32)) as i32)) != 0) {
        if ((((((c.m.r32((l_act_4).wrapping_add(4)) as i32)) == (0i32)) as i32)) != 0) {
            break 'f 1i32;
        }
        break 'f ((((c.m.r32(l_e_5) as i32)) != (80i32)) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit49", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSense_CheckBit50(c: &mut Ctx, mut p_s_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit50", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_e_2: u32 = 0;
    let r_ = 'f: {
    l_e_2 = (((p_s_1).wrapping_add(40)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(8));
    if (({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetStunTimer(c, a0_) }) != 0) {
        break 'f (if ((((((c.m.r32(l_e_2) as i32)) == (81i32)) as i32)) != 0) { 1i32 } else { 2i32 });
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit50", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSense_CheckBit51(c: &mut Ctx, mut p_s_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit51", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_e_2: u32 = 0;
    let r_ = 'f: {
    l_e_2 = (((p_s_1).wrapping_add(40)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(8));
    if (({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_TestFlagBE(c, a0_) }) != 0) {
        break 'f (if ((((((c.m.r32(l_e_2) as i32)) == (82i32)) as i32)) != 0) { 1i32 } else { 2i32 });
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit51", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSense_CheckBit43(c: &mut Ctx, mut p_s_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit43", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_w_2: u64 = 0;
    let mut l_st_3: u32 = 0;
    let mut l_other_4: u64 = 0;
    let r_ = 'f: {
    l_w_2 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetUnk1288(c, a0_) };
    l_st_3 = (p_s_1).wrapping_add(704);
    l_other_4 = ((l_w_2) & (((30786325577728i64) as u64)));
    { let r_ = ((2199023255552i64) as u64); let n_ = ((l_w_2) & (r_)); l_w_2 = n_; };
    if (((((l_w_2) == (((0i32) as u64))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((l_other_4) != (((0i32) as u64))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32((l_st_3).wrapping_add(20)) as i32)) & (524288i32))) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit43", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSense_CheckBit40(c: &mut Ctx, mut p_s_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit40", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_st_2: u32 = 0;
    let mut l_act_3: u32 = 0;
    let mut l_e_4: u32 = 0;
    let mut l_opp_5: u32 = 0;
    let mut l_tbl_6: u32 = 0;
    let mut l_oppState_7: i32 = 0;
    let mut l_cls_8: i32 = 0;
    let mut l_w_9: u64 = 0;
    let r_ = 'f: {
    l_st_2 = (p_s_1).wrapping_add(704);
    l_act_3 = (p_s_1).wrapping_add(40);
    l_e_4 = (((p_s_1).wrapping_add(40)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(8));
    l_opp_5 = ((c.m.r32(sym::gBtlAi)).wrapping_add(16)).wrapping_add(((((((c.m.r32(p_s_1) as i32)) ^ (1i32))) as u32)).wrapping_mul(1312));
    l_tbl_6 = { let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) };
    l_oppState_7 = { let a0_ = (((c.m.r32(p_s_1) as i32)) ^ (1i32)); F::BtlCharApi_GetUnk974(c, a0_) };
    l_cls_8 = (((c.m.r8(((l_tbl_6).wrapping_add(904)).wrapping_add(((l_oppState_7) as u32))) as i8)) as i32);
    l_w_9 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetUnk1288(c, a0_) };
    if ((((!(({ let a0_ = (c.m.r32(l_opp_5) as i32); F::BtlSide_IsPoweredUp(c, a0_) }) != 0)) as i32)) != 0) {
        { let r_ = (!(16i32)); let a_ = (l_st_2).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        break 'f 0i32;
    }
    if ((((!((F::BtlAiSense_IsBehindOpponent(c, l_opp_5)) != 0)) as i32)) != 0) {
        { let r_ = (!(16i32)); let a_ = (l_st_2).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        break 'f 0i32;
    }
    if ((((!((((((((l_cls_8).wrapping_sub(13i32)) as u32)) < (((2i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        { let r_ = (!(16i32)); let a_ = (l_st_2).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        break 'f 0i32;
    }
    if ((((!((((l_w_9) & (((70368744177664i64) as u64)))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32((l_st_2).wrapping_add(20)) as i32)) & (16i32))) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32((l_st_2).wrapping_add(20)) as i32)) & (1048576i32))) != 0) {
        if ((((((c.m.r32((l_st_2).wrapping_add(16)) as i32)) == (l_oppState_7)) as i32)) != 0) {
            break 'f 0i32;
        }
        { let r_ = (!(1048576i32)); let a_ = (l_st_2).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if ((((((((((c.m.r32((l_act_3).wrapping_add(4)) as i32)) > (0i32)) as i32)) != 0) && ((((((c.m.r32(l_e_4) as i32)) == (92i32)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit40", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSense_CheckBit41(c: &mut Ctx, mut p_s_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit41", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_m_2: u32 = 0;
    let mut l_act_3: u32 = 0;
    let mut l_e_4: u32 = 0;
    let r_ = 'f: {
    l_m_2 = (p_s_1).wrapping_add(192);
    l_act_3 = (p_s_1).wrapping_add(40);
    l_e_4 = (((p_s_1).wrapping_add(40)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(8));
    if ((((!(({ let a0_ = ((c.m.r32(sym::gBtlAi)).wrapping_add(16)).wrapping_add(((((((c.m.r32(p_s_1) as i32)) ^ (1i32))) as u32)).wrapping_mul(1312)); F::BtlAiSense_IsBehindOpponent(c, a0_) }) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((rt::flt(F32(c.m.r32((l_m_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), { let a_ = (c.m.r32(sym::gBtlAi)).wrapping_add(4); F32(c.m.r32(a_)) }) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((c.m.r32((l_act_3).wrapping_add(4)) as i32)) > (0i32)) as i32)) != 0) {
        if ((((((c.m.r32(l_e_4) as i32)) == (6i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((((((c.m.r32(l_e_4) as i32)) == (92i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((((((c.m.r32(l_e_4) as i32)) == (80i32)) as i32)) != 0) {
            break 'f 0i32;
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit41", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSense_CheckBit28(c: &mut Ctx, mut p_s_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit28", &PROBE_N, &[(p_s_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_tbl_2: u32 = 0;
    let mut l_st_3: u32 = 0;
    let r_ = 'f: {
        l_tbl_2 = { let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) };
        l_st_3 = (p_s_1).wrapping_add(704);
        { let v_ = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetUnk974(c, a0_) }; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = (((c.m.r32(p_s_1) as i32)) ^ (1i32)); F::BtlCharApi_GetUnk974(c, a0_) }; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = (({ let a_ = (((l_tbl_2).wrapping_add(((8i32) as u32))).wrapping_add(896)).wrapping_add((((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u32)); c.m.r8(a_) }) as i8); let a_ = (fp.wrapping_add(8)).wrapping_add(((0i32) as u32)); c.m.w8(a_, (v_) as u8); };
        { let v_ = (({ let a_ = (((l_tbl_2).wrapping_add(((8i32) as u32))).wrapping_add(896)).wrapping_add((((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)); c.m.r8(a_) }) as i8); let a_ = (fp.wrapping_add(8)).wrapping_add(((1i32) as u32)); c.m.w8(a_, (v_) as u8); };
        if ((((!((((c.m.r64(l_st_3)) & (((16i32) as u64)))) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        if (((((((((((((c.m.r8((fp.wrapping_add(8)).wrapping_add(((0i32) as u32))) as i8)) as i32)).wrapping_sub(13i32)) as u8)) as i32)) < (11i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        if (({ let a0_ = ((c.m.r32(sym::gBtlAi)).wrapping_add(16)).wrapping_add(((((((c.m.r32(p_s_1) as i32)) ^ (1i32))) as u32)).wrapping_mul(1312)); F::BtlAiSense_IsBehindOpponent(c, a0_) }) != 0) {
            break 'f 0i32;
        }
        if ((((!(((((((((((((c.m.r8((fp.wrapping_add(8)).wrapping_add(((1i32) as u32))) as i8)) as u8)) as i32)).wrapping_sub(13i32)) as u32)) < (((5i32) as u32))) as i32)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((((!(({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_IsUnkCACPaired(c, a0_) }) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        if (((((c.m.r32((l_st_3).wrapping_add(20)) as i32)) & (32i32))) != 0) {
            break 'f 0i32;
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit28", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSense_CheckBit29(c: &mut Ctx, mut p_s_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit29", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_cls_2: i32 = 0;
    let mut l_st_3: u32 = 0;
    let r_ = 'f: {
    l_cls_2 = (({ let a_ = (({ let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) }).wrapping_add(904)).wrapping_add((({ let a0_ = (((c.m.r32(p_s_1) as i32)) ^ (1i32)); F::BtlCharApi_GetUnk974(c, a0_) }) as u32)); (c.m.r8(a_) as i8) }) as i32);
    l_st_3 = (p_s_1).wrapping_add(704);
    if (((((c.m.r32((l_st_3).wrapping_add(20)) as i32)) & (512i32))) != 0) {
        break 'f 0i32;
    }
    if ((((!((((((((l_cls_2).wrapping_sub(15i32)) as u32)) < (((3i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((c.m.r64(l_st_3)) & (((16i32) as u64)))) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit29", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSense_CheckBit35(c: &mut Ctx, mut p_s_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit35", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_st_2: u32 = 0;
    let mut l_progress_3: F32 = F32(0);
    let r_ = 'f: {
    l_st_2 = (p_s_1).wrapping_add(704);
    l_progress_3 = { let a0_ = (((c.m.r32(p_s_1) as i32)) ^ (1i32)); F::BtlCharApi_GetTechniqueProgress(c, a0_) };
    if ((((!(({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetOppSkillKind(c, a0_) }) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32((l_st_2).wrapping_add(20)) as i32)) & (1024i32))) != 0) {
        break 'f 0i32;
    }
    if (((rt::feq(l_progress_3, F32(0xbf800000)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_IsUnk1070Set(c, a0_) }) != 0) {
        break 'f 0i32;
    }
    if (((rt::flt(F32(0x3f800000), l_progress_3) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit35", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSense_CheckBit31(c: &mut Ctx, mut p_s_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit31", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_st_2: u32 = 0;
    let mut l_progress_3: F32 = F32(0);
    let mut l_tbl_4: u32 = 0;
    let mut l_cls_5: i32 = 0;
    let r_ = 'f: {
    l_st_2 = (p_s_1).wrapping_add(704);
    l_progress_3 = { let a0_ = (((c.m.r32(p_s_1) as i32)) ^ (1i32)); F::BtlCharApi_GetTechniqueProgress(c, a0_) };
    l_tbl_4 = { let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) };
    l_cls_5 = (({ let a_ = ((l_tbl_4).wrapping_add(904)).wrapping_add((({ let a0_ = (((c.m.r32(p_s_1) as i32)) ^ (1i32)); F::BtlCharApi_GetUnk974(c, a0_) }) as u32)); (c.m.r8(a_) as i8) }) as i32);
    if ((((!(({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetOppSkillKind(c, a0_) }) != 0)) as i32)) != 0) {
        if (((rt::flt(l_progress_3, F32(0x00000000)) as i32)) != 0) {
            break 'f 0i32;
        }
        if (((rt::flt(F32(0x3f800000), l_progress_3) as i32)) != 0) {
            break 'f 0i32;
        }
    } else {
        if (((((l_cls_5) != (24i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        if (((((((rt::flt(l_progress_3, F32(0x3f800000)) as i32)) != 0) || (((rt::fle(F32(0x40400000), l_progress_3) as i32)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
    }
    if (((((c.m.r32((l_st_2).wrapping_add(20)) as i32)) & (128i32))) != 0) {
        break 'f 0i32;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit31", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSense_CheckBit33(c: &mut Ctx, mut p_s_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit33", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_st_2: u32 = 0;
    let r_ = 'f: {
    l_st_2 = (p_s_1).wrapping_add(704);
    if (((({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetUnk1288(c, a0_) }) & (((35184372088832i64) as u64)))) != 0) {
        if (((((c.m.r32((l_st_2).wrapping_add(20)) as i32)) & (256i32))) != 0) {
            break 'f 0i32;
        }
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit33", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSense_CheckBit34(c: &mut Ctx, mut p_s_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit34", &PROBE_N, &[(p_s_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_tbl_2: u32 = 0;
    let mut l_act_3: u32 = 0;
    let mut l_step_4: i32 = 0;
    let r_ = 'f: {
        l_tbl_2 = { let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) };
        l_act_3 = (p_s_1).wrapping_add(40);
        { let v_ = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetUnk974(c, a0_) }; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = (((c.m.r32(p_s_1) as i32)) ^ (1i32)); F::BtlCharApi_GetUnk974(c, a0_) }; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = (({ let a_ = (((l_tbl_2).wrapping_add(((8i32) as u32))).wrapping_add(896)).wrapping_add((((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u32)); c.m.r8(a_) }) as i8); let a_ = (fp.wrapping_add(8)).wrapping_add(((0i32) as u32)); c.m.w8(a_, (v_) as u8); };
        { let v_ = (({ let a_ = (((l_tbl_2).wrapping_add(((8i32) as u32))).wrapping_add(896)).wrapping_add((((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)); c.m.r8(a_) }) as i8); let a_ = (fp.wrapping_add(8)).wrapping_add(((1i32) as u32)); c.m.w8(a_, (v_) as u8); };
        l_step_4 = (if ((((((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) < (60i32)) as i32)) != 0) { ((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)).wrapping_sub(55i32) } else { ((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)).wrapping_sub(60i32) });
        if ((((((((c.m.r8((fp.wrapping_add(8)).wrapping_add(((0i32) as u32))) as i8)) as i32)) == (26i32)) as i32)) != 0) {
            if ((((!(((((((l_step_4) as u32)) < (((4i32) as u32))) as i32)) != 0)) as i32)) != 0) {
                break 'f 0i32;
            }
            if ((((((((((c.m.r32((l_act_3).wrapping_add(4)) as i32)) != (0i32)) as i32)) != 0) && ((((({ let a_ = ((l_act_3).wrapping_add(((((c.m.r32((l_act_3).wrapping_add(4)) as i32)).wrapping_shl((3i32) as u32)) as u32))).wrapping_add(((12i32) as u32)); (c.m.r32(a_) as i32) }) == (71i32)) as i32)) != 0)) as i32)) != 0) {
                break 'f 0i32;
            }
            break 'f { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_TestFlag66(c, a0_) };
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit34", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSense_CheckBit36(c: &mut Ctx, mut p_s_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit36", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_st_2: u32 = 0;
    let mut l_kind_3: i32 = 0;
    let r_ = 'f: {
    l_st_2 = (p_s_1).wrapping_add(704);
    l_kind_3 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetOppSkillKind(c, a0_) };
    if (((((l_kind_3) == ((1i32).wrapping_neg())) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((((l_kind_3).wrapping_sub(1i32)) as u32)) < (((2i32) as u32))) as i32)) != 0) {
        if (((((c.m.r32((l_st_2).wrapping_add(20)) as i32)) & (2048i32))) != 0) {
            break 'f 0i32;
        }
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit36", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSense_CheckBit37(c: &mut Ctx, mut p_s_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit37", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_st_2: u32 = 0;
    let mut l_kind_3: i32 = 0;
    let r_ = 'f: {
    l_st_2 = (p_s_1).wrapping_add(704);
    l_kind_3 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetOppSkillKind(c, a0_) };
    if (((((l_kind_3) == ((1i32).wrapping_neg())) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((l_kind_3) == (3i32)) as i32)) != 0) {
        if (((((c.m.r32((l_st_2).wrapping_add(20)) as i32)) & (4096i32))) != 0) {
            break 'f 0i32;
        }
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit37", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSense_CheckBit38(c: &mut Ctx, mut p_s_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit38", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_st_2: u32 = 0;
    let mut l_kind_3: i32 = 0;
    let r_ = 'f: {
    l_st_2 = (p_s_1).wrapping_add(704);
    l_kind_3 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetOppSkillKind(c, a0_) };
    if (((((l_kind_3) == ((1i32).wrapping_neg())) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((l_kind_3) == (0i32)) as i32)) != 0) {
        if (((((c.m.r32((l_st_2).wrapping_add(20)) as i32)) & (8192i32))) != 0) {
            break 'f 0i32;
        }
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit38", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSense_CheckBit39(c: &mut Ctx, mut p_s_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit39", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_a_2: i32 = 0;
    let mut l_b_3: i32 = 0;
    let mut l_ret_4: i32 = 0;
    let r_ = 'f: {
    l_a_2 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_IsOppSkillFlag4(c, a0_) };
    l_b_3 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetParamFlags(c, a0_) };
    l_ret_4 = (((l_a_2) != (0i32)) as i32);
    if ((((!((((l_b_3) & (2i32))) != 0)) as i32)) != 0) {
        l_ret_4 = 0i32;
    }
    break 'f l_ret_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_CheckBit39", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSense_Reactions(c: &mut Ctx, mut p_s_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_Reactions", &PROBE_N, &[(p_s_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_tbl_2: u32 = 0;
    let mut l_act_3: u32 = 0;
    let mut l_st_4: u32 = 0;
    let mut l_w_5: u64 = 0;
    let mut l_e_6: u32 = 0;
    let mut l_r_7: i32 = 0;
    let mut l_id_8: i32 = 0;
    let r_ = 'f: {
        l_tbl_2 = { let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) };
        l_act_3 = (p_s_1).wrapping_add(40);
        l_st_4 = (p_s_1).wrapping_add(704);
        l_w_5 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetUnk1288(c, a0_) };
        { let v_ = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetUnk974(c, a0_) }; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = (((c.m.r32(p_s_1) as i32)) ^ (1i32)); F::BtlCharApi_GetUnk974(c, a0_) }; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = (({ let a_ = (((l_tbl_2).wrapping_add(((8i32) as u32))).wrapping_add(896)).wrapping_add((((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u32)); c.m.r8(a_) }) as i8); let a_ = (fp.wrapping_add(8)).wrapping_add(((0i32) as u32)); c.m.w8(a_, (v_) as u8); };
        { let v_ = (({ let a_ = (((l_tbl_2).wrapping_add(((8i32) as u32))).wrapping_add(896)).wrapping_add((((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)); c.m.r8(a_) }) as i8); let a_ = (fp.wrapping_add(8)).wrapping_add(((1i32) as u32)); c.m.w8(a_, (v_) as u8); };
        l_r_7 = F::BtlAiSense_CheckBit51(c, p_s_1);
        if (((((l_r_7) != (0i32)) as i32)) != 0) {
            if (((((l_r_7) == (2i32)) as i32)) != 0) {
                { let r_ = ((2251799813685248i64) as u64); let a_ = l_st_4; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
            }
            break 'f;
        }
        l_r_7 = F::BtlAiSense_CheckBit50(c, p_s_1);
        if (((((l_r_7) != (0i32)) as i32)) != 0) {
            if (((((l_r_7) == (2i32)) as i32)) != 0) {
                { let r_ = ((1125899906842624i64) as u64); let a_ = l_st_4; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
            }
            break 'f;
        }
        if ((((((((c.m.r8((fp.wrapping_add(8)).wrapping_add(((0i32) as u32))) as i8)) as i32)) == (2i32)) as i32)) != 0) {
            l_e_6 = (((p_s_1).wrapping_add(40)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(8));
            if ((((!((((((((((c.m.r32((l_act_3).wrapping_add(4)) as i32)) > (0i32)) as i32)) != 0) && ((((((c.m.r32(l_e_6) as i32)) == (85i32)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
                { let r_ = ((4398046511104i64) as u64); let a_ = l_st_4; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
            }
        }
        if ((((((((((((c.m.r8((fp.wrapping_add(8)).wrapping_add(((0i32) as u32))) as i8)) as i32)) == (4i32)) as i32)) != 0) && ((((!(((((c.m.r32((l_st_4).wrapping_add(20)) as i32)) & (131072i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
            { let r_ = ((281474976710656i64) as u64); let a_ = l_st_4; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
        }
        if ((F::BtlAiSense_CheckBit49(c, p_s_1)) != 0) {
            { let r_ = ((562949953421312i64) as u64); let a_ = l_st_4; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
        }
        if ((F::BtlAiSense_CheckBit43(c, p_s_1)) != 0) {
            { let r_ = ((8796093022208i64) as u64); let a_ = l_st_4; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
        }
        if (((((((((((((c.m.r8((fp.wrapping_add(8)).wrapping_add(((0i32) as u32))) as i8)) as u8)) as i32)).wrapping_sub(1i32)) as u32)) < (((7i32) as u32))) as i32)) != 0) {
            { let r_ = (!(128i32)); let a_ = l_act_3; let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if ((F::BtlAiSense_CheckBit40(c, p_s_1)) != 0) {
            { let r_ = ((1099511627776i64) as u64); let a_ = l_st_4; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
        }
        if ((F::BtlAiSense_CheckBit41(c, p_s_1)) != 0) {
            { let r_ = ((2199023255552i64) as u64); let a_ = l_st_4; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
        }
        if (((((F::BtlAiSense_CheckBit28(c, p_s_1)) == (1i32)) as i32)) != 0) {
            { let r_ = ((268435456i32) as u64); let a_ = l_st_4; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
            { let r_ = 2i32; let a_ = (l_st_4).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if (((((F::BtlAiSense_CheckBit29(c, p_s_1)) == (1i32)) as i32)) != 0) {
            { let r_ = ((536870912i32) as u64); let a_ = l_st_4; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
            { let r_ = 512i32; let a_ = (l_st_4).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            { let v_ = (((c.m.r8((fp.wrapping_add(8)).wrapping_add(((1i32) as u32))) as i8)) as i32); let a_ = (l_st_4).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        }
        if ((((({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_FindIncomingBlast(c, a0_, 0i32) }) != ((1i32).wrapping_neg())) as i32)) != 0) {
            { let r_ = ((1073741824i32) as u64); let a_ = l_st_4; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
        }
        if (((((F::BtlAiSense_CheckBit35(c, p_s_1)) == (1i32)) as i32)) != 0) {
            { let r_ = ((34359738368i64) as u64); let a_ = l_st_4; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
        }
        if (((((F::BtlAiSense_CheckBit31(c, p_s_1)) == (1i32)) as i32)) != 0) {
            { let r_ = ((2147483648u32) as u64); let a_ = l_st_4; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
        }
        if ((((!(((((c.m.r32((l_st_4).wrapping_add(20)) as i32)) & (32768i32))) != 0)) as i32)) != 0) {
            if ((((l_w_5) & (((4398046511104i64) as u64)))) != 0) {
                { let r_ = ((70368744177664i64) as u64); let a_ = l_st_4; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
            }
            if ((((l_w_5) & (((8796093022208i64) as u64)))) != 0) {
                { let r_ = ((140737488355328i64) as u64); let a_ = l_st_4; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
            }
        }
        if ((((l_w_5) & (((17592186044416i64) as u64)))) != 0) {
            if ((((!(((((c.m.r32((l_st_4).wrapping_add(20)) as i32)) & (16384i32))) != 0)) as i32)) != 0) {
                l_id_8 = 0i32;
                if ((((((c.m.r32((l_act_3).wrapping_add(4)) as i32)) > (0i32)) as i32)) != 0) {
                    l_id_8 = { let a_ = ((l_act_3).wrapping_add(((((c.m.r32((l_act_3).wrapping_add(4)) as i32)).wrapping_shl((3i32) as u32)) as u32))).wrapping_add(((12i32) as u32)); (c.m.r32(a_) as i32) };
                }
                if ((((!((((((((l_id_8).wrapping_sub(83i32)) as u32)) < (((2i32) as u32))) as i32)) != 0)) as i32)) != 0) {
                    { let r_ = ((35184372088832i64) as u64); let a_ = l_st_4; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
                }
            }
        }
        if (((((F::BtlAiSense_CheckBit33(c, p_s_1)) == (1i32)) as i32)) != 0) {
            { let r_ = ((8589934592i64) as u64); let a_ = l_st_4; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
        }
        if (((((F::BtlAiSense_CheckBit34(c, p_s_1)) == (1i32)) as i32)) != 0) {
            { let r_ = ((17179869184i64) as u64); let a_ = l_st_4; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
        }
        if (((((((((((((c.m.r8((fp.wrapping_add(8)).wrapping_add(((0i32) as u32))) as i8)) as u8)) as i32)).wrapping_sub(1i32)) as u32)) < (((6i32) as u32))) as i32)) != 0) {
            { let r_ = ((17592186044416i64) as u64); let a_ = l_st_4; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
        }
        if (((((F::BtlAiSense_CheckBit36(c, p_s_1)) == (1i32)) as i32)) != 0) {
            { let r_ = ((68719476736i64) as u64); let a_ = l_st_4; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
        }
        if (((((F::BtlAiSense_CheckBit37(c, p_s_1)) == (1i32)) as i32)) != 0) {
            if (((((c.m.r32((l_st_4).wrapping_add(20)) as i32)) & (8i32))) != 0) {
                { let r_ = ((4611686018427387904i64) as u64); let a_ = l_st_4; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
            }
            { let r_ = ((137438953472i64) as u64); let a_ = l_st_4; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
        }
        if (((((F::BtlAiSense_CheckBit38(c, p_s_1)) == (1i32)) as i32)) != 0) {
            { let r_ = ((274877906944i64) as u64); let a_ = l_st_4; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
        }
        if (((((F::BtlAiSense_CheckBit39(c, p_s_1)) == (1i32)) as i32)) != 0) {
            { let r_ = ((549755813888i64) as u64); let a_ = l_st_4; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_Reactions", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSense_IsSteep(c: &mut Ctx, mut p_s_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_IsSteep", &PROBE_N, &[(p_s_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_a_2: F32 = F32(0);
    let r_ = 'f: {
        { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetPos(c, a0_, fp) };
        { let a0_ = (((c.m.r32(p_s_1) as i32)) ^ (1i32)); F::BtlCharApi_GetPos(c, a0_, fp.wrapping_add(16)) };
        if (((rt::flt(F::Vec3_Dist(c, fp.wrapping_add(16), fp), F32(0x41f00000)) as i32)) != 0) {
            break 'f 0i32;
        }
        l_a_2 = rt::fneg({ let a0_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(c.m.r32((fp).wrapping_add(4)))); let a1_ = F::Vec3_Dist(c, fp.wrapping_add(16), fp); F::Port_atan2f(c, a0_, a1_) });
        if (((rt::flt(l_a_2, F32(0x00000000)) as i32)) != 0) {
            if ((((!(((rt::flt(F32(0xbf333333), l_a_2) as i32)) != 0)) as i32)) != 0) {
                break 'f 1i32;
            }
        } else {
            if ((((!(((rt::flt(l_a_2, F32(0x3f333333)) as i32)) != 0)) as i32)) != 0) {
                break 'f 1i32;
            }
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_IsSteep", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSense_Derived(c: &mut Ctx, mut p_s_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_Derived", &PROBE_N, &[(p_s_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_tbl_2: u32 = 0;
    let mut l_st_3: u32 = 0;
    let mut l_w_4: u64 = 0;
    let mut l_oppBusy_5: i32 = 0;
    let r_ = 'f: {
        l_tbl_2 = { let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) };
        l_st_3 = (p_s_1).wrapping_add(704);
        l_w_4 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetUnk1288(c, a0_) };
        l_oppBusy_5 = { let a0_ = (((c.m.r32(p_s_1) as i32)) ^ (1i32)); F::BtlCharApi_GetStunTimer(c, a0_) };
        { let v_ = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetUnk974(c, a0_) }; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = (((c.m.r32(p_s_1) as i32)) ^ (1i32)); F::BtlCharApi_GetUnk974(c, a0_) }; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = (({ let a_ = (((l_tbl_2).wrapping_add(((8i32) as u32))).wrapping_add(896)).wrapping_add((((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u32)); c.m.r8(a_) }) as i8); let a_ = (fp.wrapping_add(8)).wrapping_add(((0i32) as u32)); c.m.w8(a_, (v_) as u8); };
        { let v_ = (({ let a_ = (((l_tbl_2).wrapping_add(((8i32) as u32))).wrapping_add(896)).wrapping_add((((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)); c.m.r8(a_) }) as i8); let a_ = (fp.wrapping_add(8)).wrapping_add(((1i32) as u32)); c.m.w8(a_, (v_) as u8); };
        if (((((c.m.r32((l_st_3).wrapping_add(20)) as i32)) & (1i32))) != 0) {
            { let r_ = ((8192i32) as u64); let a_ = l_st_3; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
        } else {
            { let r_ = ((16384i32) as u64); let a_ = l_st_3; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
        }
        if (({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlSide_IsPoweredUp(c, a0_) }) != 0) {
            { let r_ = ((32768i32) as u64); let a_ = l_st_3; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
        }
        if ((((c.m.r64(l_st_3)) & (((16i32) as u64)))) != 0) {
            { let r_ = ((128i32) as u64); let a_ = l_st_3; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
            if ((((c.m.r64(l_st_3)) & (((32768i32) as u64)))) != 0) {
                { let r_ = ((1024i32) as u64); let a_ = l_st_3; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
            }
        } else {
            if ((((c.m.r64(l_st_3)) & (((32i32) as u64)))) != 0) {
                { let r_ = ((256i32) as u64); let a_ = l_st_3; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
                if ((((c.m.r64(l_st_3)) & (((32768i32) as u64)))) != 0) {
                    { let r_ = ((2048i32) as u64); let a_ = l_st_3; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
                }
            } else {
                if ((((c.m.r64(l_st_3)) & (((64i32) as u64)))) != 0) {
                    { let r_ = ((512i32) as u64); let a_ = l_st_3; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
                    if ((((c.m.r64(l_st_3)) & (((32768i32) as u64)))) != 0) {
                        { let r_ = ((4096i32) as u64); let a_ = l_st_3; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
                    }
                }
            }
        }
        if ((F::BtlAiSense_IsSteep(c, p_s_1)) != 0) {
            { let r_ = ((8388608i32) as u64); let a_ = l_st_3; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
        }
        if (((({ let a_ = (c.m.r32(sym::gBtlAi)).wrapping_add(12); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
            { let r_ = ((16777216i32) as u64); let a_ = l_st_3; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
        }
        if ((((l_w_4) & (((273536256511i64) as u64)))) != 0) {
            { let r_ = ((33554432i32) as u64); let a_ = l_st_3; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
        }
        if ((((((((((((c.m.r8((fp.wrapping_add(8)).wrapping_add(((1i32) as u32))) as i8)) as i32)) == (6i32)) as i32)) != 0) || (((((l_oppBusy_5) > (0i32)) as i32)) != 0)) as i32)) != 0) {
            { let r_ = ((65536i32) as u64); let a_ = l_st_3; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
        }
        if (((((((((((((((((c.m.r8((fp.wrapping_add(8)).wrapping_add(((1i32) as u32))) as i8)) as i32)).wrapping_sub(3i32)) as u8)) as i32)) < (5i32)) as i32)) != 0) && ((((((((c.m.r8((fp.wrapping_add(8)).wrapping_add(((1i32) as u32))) as i8)) as i32)) != (6i32)) as i32)) != 0)) as i32)) != 0) {
            if (((((c.m.r32((l_st_3).wrapping_add(20)) as i32)) & (4i32))) != 0) {
                { let r_ = ((262144i32) as u64); let a_ = l_st_3; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
            } else {
                { let r_ = ((131072i32) as u64); let a_ = l_st_3; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
            }
        } else {
            { let r_ = (!(4i32)); let a_ = (l_st_3).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if ((((c.m.r64(l_st_3)) & (((64i32) as u64)))) != 0) {
            if ((((((((((((c.m.r8((fp.wrapping_add(8)).wrapping_add(((1i32) as u32))) as i8)) as i32)) == (7i32)) as i32)) != 0) || ((((((((c.m.r8((fp.wrapping_add(8)).wrapping_add(((1i32) as u32))) as i8)) as i32)) == (2i32)) as i32)) != 0)) as i32)) != 0) {
                { let r_ = ((524288i32) as u64); let a_ = l_st_3; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
            }
        }
        if ((((c.m.r64(l_st_3)) & (((48i32) as u64)))) != 0) {
            if ((((((((((((c.m.r8((fp.wrapping_add(8)).wrapping_add(((1i32) as u32))) as i8)) as i32)) == (2i32)) as i32)) != 0) || ((((((((c.m.r8((fp.wrapping_add(8)).wrapping_add(((1i32) as u32))) as i8)) as i32)) == (5i32)) as i32)) != 0)) as i32)) != 0) {
                { let r_ = ((1048576i32) as u64); let a_ = l_st_3; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
            }
        }
        if ((((((((c.m.r8((fp.wrapping_add(8)).wrapping_add(((0i32) as u32))) as i8)) as i32)) == (11i32)) as i32)) != 0) {
            { let r_ = ((67108864i32) as u64); let a_ = l_st_3; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_Derived", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSense_Update(c: &mut Ctx, mut p_s_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_Update", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_st_2: u32 = 0;
    let mut l_tbl_3: u32 = 0;
    let mut l_cls_4: i32 = 0;
    let mut l_react_5: i32 = 0;
    let mut l_i_6: i32 = 0;
    let r_ = 'f: {
    l_st_2 = (p_s_1).wrapping_add(704);
    l_tbl_3 = { let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) };
    l_cls_4 = (({ let a_ = ((l_tbl_3).wrapping_add(904)).wrapping_add((({ let a0_ = (((c.m.r32(p_s_1) as i32)) ^ (1i32)); F::BtlCharApi_GetUnk974(c, a0_) }) as u32)); (c.m.r8(a_) as i8) }) as i32);
    l_react_5 = (((c.m.r32((l_st_2).wrapping_add(20)) as i32)) & ((!(11i32))));
    { let v_ = ((0i32) as u64); let a_ = l_st_2; c.m.w64(a_, v_); };
    { let v_ = l_react_5; let a_ = (l_st_2).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    if (((((l_cls_4) != ((c.m.r32((l_st_2).wrapping_add(12)) as i32))) as i32)) != 0) {
        {
            l_i_6 = 5i32;
            'l1: loop {
                if !(((((l_i_6) < (21i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let r_ = (!((1i32).wrapping_shl((l_i_6) as u32))); let n_ = ((l_react_5) & (r_)); l_react_5 = n_; };
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
        { let v_ = l_react_5; let a_ = (l_st_2).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    }
    if ((F::BtlAiSense_Basic(c, p_s_1)) != 0) {
        break 'f;
    }
    F::BtlAiSense_NoteOppState(c, p_s_1);
    if ((F::BtlAiSense_IsBusy(c, p_s_1)) != 0) {
        break 'f;
    }
    F::BtlAiSense_Reactions(c, p_s_1);
    F::BtlAiSense_Derived(c, p_s_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSense_Update", &PROBE_N, &[], true); }
    r_
}
