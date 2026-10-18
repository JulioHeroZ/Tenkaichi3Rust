#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Pad_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Pad_Init", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
        { let _ = F::sceDbcInit(c); };
        { let _ = F::scePad2Init(c, 0i32); };
        { let _ = F::memset(c, sym::gPad, 0i32, 896u32); };
        { let _ = F::memset(c, fp, 0i32, 32u32); };
        { let v_ = ((2i32) as u32); let a_ = fp; c.m.w32(a_, v_); };
        { let v_ = 0i32; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = F::scePad2CreateSocket(c, fp, (sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))); let a_ = ((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(256); c.m.w32(a_, (v_) as u32); };
        { let _ = F::memset(c, fp, 0i32, 32u32); };
        { let v_ = 1i32; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((2i32) as u32); let a_ = fp; c.m.w32(a_, v_); };
        { let v_ = 0i32; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = F::scePad2CreateSocket(c, fp, (sym::gPad).wrapping_add((((1i32) as u32)).wrapping_mul(448))); let a_ = ((sym::gPad).wrapping_add((((1i32) as u32)).wrapping_mul(448))).wrapping_add(256); c.m.w32(a_, (v_) as u32); };
        {
            l_i_1 = 0i32;
            'l1: loop {
                if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = 20i32; let a_ = ((sym::gPad).wrapping_add((((l_i_1) as u32)).wrapping_mul(448))).wrapping_add(412); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 1i32; let a_ = ((sym::gPad).wrapping_add((((l_i_1) as u32)).wrapping_mul(448))).wrapping_add(416); c.m.w32(a_, (v_) as u32); };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Pad_Init", &PROBE_N, &[], true); }
    r_
}

pub fn Pad_Stub0(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Pad_Stub0", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Pad_Stub0", &PROBE_N, &[], true); }
    r_
}

pub fn Pad_Stub1(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Pad_Stub1", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Pad_Stub1", &PROBE_N, &[], true); }
    r_
}

pub fn Pad_Stub2(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Pad_Stub2", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Pad_Stub2", &PROBE_N, &[], true); }
    r_
}

pub fn Pad_Stub3(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Pad_Stub3", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Pad_Stub3", &PROBE_N, &[], true); }
    r_
}

pub fn Pad_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Pad_Update", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_pad_2: u32 = 0;
    let mut l_btn_3: u32 = 0;
    let mut l_len_4: F32 = F32(0);
    let mut l_inv_5: F32 = F32(0);
    let r_ = 'f: {
    l_pad_2 = sym::gPad;
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = { let a0_ = (c.m.r32((l_pad_2).wrapping_add(256)) as i32); F::scePad2GetState(c, a0_) }; let a_ = (l_pad_2).wrapping_add(260); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32((l_pad_2).wrapping_add(260)) as i32)) == (1i32)) as i32)) != 0) {
                    { let v_ = 0i32; let a_ = (l_pad_2).wrapping_add(384); c.m.w32(a_, (v_) as u32); };
                    's3: {
                        let sv6: i32 = (c.m.r32((l_pad_2).wrapping_add(264)) as i32);
                        'k6: {
                        'k5: {
                        'k4: {
                            match sv6 { 0i32 => break 'k4, 1i32 => break 'k5, 2i32 => break 'k6, _ => break 's3, }
                        }
                        { let _ = F::Pad_GetVibProfile(c, l_i_1); };
                        }
                        { let v_ = { let a0_ = (c.m.r32((l_pad_2).wrapping_add(256)) as i32); F::scePad2GetButtonProfile(c, a0_, (l_pad_2).wrapping_add(268)) }; let a_ = (l_pad_2).wrapping_add(272); c.m.w32(a_, (v_) as u32); };
                        if ((((((c.m.r32((l_pad_2).wrapping_add(272)) as i32)) >= (0i32)) as i32)) != 0) {
                            { let a_ = (l_pad_2).wrapping_add(264); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        }
                        break 's3;
                        }
                        { let v_ = { let a0_ = (c.m.r32((l_pad_2).wrapping_add(256)) as i32); F::scePad2Read(c, a0_, (l_pad_2).wrapping_add(276)) }; let a_ = (l_pad_2).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                        l_btn_3 = (((((((((c.m.r8(((l_pad_2).wrapping_add(276)).wrapping_add(((1i32) as u32)))) as i32)).wrapping_shl((8i32) as u32)) | (((c.m.r8(((l_pad_2).wrapping_add(276)).wrapping_add(((0i32) as u32)))) as i32)))) ^ (65535i32))) as u32);
                        { let v_ = { let a0_ = c.m.r8(((l_pad_2).wrapping_add(276)).wrapping_add(((2i32) as u32))); F::Pad_StickToFloat(c, a0_, 50i32) }; let a_ = ((l_pad_2).wrapping_add(312)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                        { let v_ = { let a0_ = c.m.r8(((l_pad_2).wrapping_add(276)).wrapping_add(((3i32) as u32))); F::Pad_StickToFloat(c, a0_, 50i32) }; let a_ = ((l_pad_2).wrapping_add(312)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                        { let v_ = { let a0_ = c.m.r8(((l_pad_2).wrapping_add(276)).wrapping_add(((4i32) as u32))); F::Pad_StickToFloat(c, a0_, 50i32) }; let a_ = ((l_pad_2).wrapping_add(304)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                        { let v_ = { let a0_ = c.m.r8(((l_pad_2).wrapping_add(276)).wrapping_add(((5i32) as u32))); F::Pad_StickToFloat(c, a0_, 50i32) }; let a_ = ((l_pad_2).wrapping_add(304)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                        l_len_4 = rt::fadd(rt::fmul(F32(c.m.r32(((l_pad_2).wrapping_add(312)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_pad_2).wrapping_add(312)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32(((l_pad_2).wrapping_add(312)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_pad_2).wrapping_add(312)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))));
                        if (((rt::flt(F32(0x3f800000), l_len_4) as i32)) != 0) {
                            l_inv_5 = rt::fdiv(F32(0x3f800000), F::Port_sqrtf(c, l_len_4));
                            { let r_ = l_inv_5; let a_ = ((l_pad_2).wrapping_add(312)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            { let r_ = l_inv_5; let a_ = ((l_pad_2).wrapping_add(312)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        }
                        l_len_4 = rt::fadd(rt::fmul(F32(c.m.r32(((l_pad_2).wrapping_add(304)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_pad_2).wrapping_add(304)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32(((l_pad_2).wrapping_add(304)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_pad_2).wrapping_add(304)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))));
                        if (((rt::flt(F32(0x3f800000), l_len_4) as i32)) != 0) {
                            l_inv_5 = rt::fdiv(F32(0x3f800000), F::Port_sqrtf(c, l_len_4));
                            { let r_ = l_inv_5; let a_ = ((l_pad_2).wrapping_add(304)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            { let r_ = l_inv_5; let a_ = ((l_pad_2).wrapping_add(304)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        }
                        if (((rt::fgt({ let a0_ = F32(c.m.r32(((l_pad_2).wrapping_add(304)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); rt::ffabs(a0_) }, { let a0_ = F32(c.m.r32(((l_pad_2).wrapping_add(304)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); rt::ffabs(a0_) }) as i32)) != 0) {
                            if (((rt::flt(F32(0x3f000000), F32(c.m.r32(((l_pad_2).wrapping_add(304)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))) as i32)) != 0) {
                                { let r_ = ((131072i32) as u32); let n_ = ((l_btn_3) | (r_)); l_btn_3 = n_; };
                            }
                            if (((rt::flt(F32(c.m.r32(((l_pad_2).wrapping_add(304)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0xbf000000)) as i32)) != 0) {
                                { let r_ = ((65536i32) as u32); let n_ = ((l_btn_3) | (r_)); l_btn_3 = n_; };
                            }
                        } else {
                            if (((rt::flt(F32(0x3f000000), F32(c.m.r32(((l_pad_2).wrapping_add(304)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))) as i32)) != 0) {
                                { let r_ = ((262144i32) as u32); let n_ = ((l_btn_3) | (r_)); l_btn_3 = n_; };
                            }
                            if (((rt::flt(F32(c.m.r32(((l_pad_2).wrapping_add(304)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(0xbf000000)) as i32)) != 0) {
                                { let r_ = ((524288i32) as u32); let n_ = ((l_btn_3) | (r_)); l_btn_3 = n_; };
                            }
                        }
                        if (((rt::fgt({ let a0_ = F32(c.m.r32(((l_pad_2).wrapping_add(312)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); rt::ffabs(a0_) }, { let a0_ = F32(c.m.r32(((l_pad_2).wrapping_add(312)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); rt::ffabs(a0_) }) as i32)) != 0) {
                            if (((rt::flt(F32(0x3f000000), F32(c.m.r32(((l_pad_2).wrapping_add(312)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))) as i32)) != 0) {
                                { let r_ = ((2097152i32) as u32); let n_ = ((l_btn_3) | (r_)); l_btn_3 = n_; };
                            }
                            if (((rt::flt(F32(c.m.r32(((l_pad_2).wrapping_add(312)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0xbf000000)) as i32)) != 0) {
                                { let r_ = ((1048576i32) as u32); let n_ = ((l_btn_3) | (r_)); l_btn_3 = n_; };
                            }
                        } else {
                            if (((rt::flt(F32(0x3f000000), F32(c.m.r32(((l_pad_2).wrapping_add(312)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))) as i32)) != 0) {
                                { let r_ = ((4194304i32) as u32); let n_ = ((l_btn_3) | (r_)); l_btn_3 = n_; };
                            }
                            if (((rt::flt(F32(c.m.r32(((l_pad_2).wrapping_add(312)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(0xbf000000)) as i32)) != 0) {
                                { let r_ = ((8388608i32) as u32); let n_ = ((l_btn_3) | (r_)); l_btn_3 = n_; };
                            }
                        }
                        { let v_ = l_btn_3; let a_ = (l_pad_2).wrapping_add(328); c.m.w32(a_, v_); };
                        { let v_ = ((l_btn_3) & ((!(c.m.r32((l_pad_2).wrapping_add(332)))))); let a_ = (l_pad_2).wrapping_add(336); c.m.w32(a_, v_); };
                        { let v_ = l_btn_3; let a_ = (l_pad_2).wrapping_add(332); c.m.w32(a_, v_); };
                        { let v_ = { let a1_ = c.m.r32((l_pad_2).wrapping_add(336)); let a4_ = (c.m.r32((l_pad_2).wrapping_add(412)) as i32); let a5_ = (c.m.r32((l_pad_2).wrapping_add(416)) as i32); F::Pad_CalcRepeat(c, l_btn_3, a1_, (l_pad_2).wrapping_add(344), (l_pad_2).wrapping_add(348), a4_, a5_) }; let a_ = (l_pad_2).wrapping_add(340); c.m.w32(a_, v_); };
                        if ((F::Pad_IsDigitalOnly(c, l_i_1)) != 0) {
                            F::Pad_Reset(c, l_i_1);
                        }
                        F::Pad_UpdateGameButtons(c, l_i_1);
                        break 's3;
                    }
                } else {
                    { let v_ = 0i32; let a_ = (l_pad_2).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                    F::Pad_Reset(c, l_i_1);
                }
                F::Pad_SendActuator(c, l_i_1);
            }
            { { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; }; { let o_ = l_pad_2; let n_ = (o_).wrapping_add(448); l_pad_2 = n_; }; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Pad_Update", &PROBE_N, &[], true); }
    r_
}

pub fn Pad_IsHeld(c: &mut Ctx, mut p_pad_1: i32, mut p_mask_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Pad_IsHeld", &PROBE_N, &[(p_pad_1) as u32, (p_mask_2) as u32], false); }
    let r_ = 'f: {
    break 'f { let a1_ = c.m.r32(((sym::gPad).wrapping_add((((p_pad_1) as u32)).wrapping_mul(448))).wrapping_add(328)); F::Pad_TestMask(c, p_pad_1, a1_, p_mask_2) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Pad_IsHeld", &PROBE_N, &[], true); }
    r_
}

pub fn Pad_IsPressed(c: &mut Ctx, mut p_pad_1: i32, mut p_mask_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Pad_IsPressed", &PROBE_N, &[(p_pad_1) as u32, (p_mask_2) as u32], false); }
    let r_ = 'f: {
    break 'f { let a1_ = c.m.r32(((sym::gPad).wrapping_add((((p_pad_1) as u32)).wrapping_mul(448))).wrapping_add(336)); F::Pad_TestMask(c, p_pad_1, a1_, p_mask_2) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Pad_IsPressed", &PROBE_N, &[], true); }
    r_
}

pub fn Pad_IsRepeat(c: &mut Ctx, mut p_pad_1: i32, mut p_mask_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Pad_IsRepeat", &PROBE_N, &[(p_pad_1) as u32, (p_mask_2) as u32], false); }
    let r_ = 'f: {
    break 'f { let a1_ = c.m.r32(((sym::gPad).wrapping_add((((p_pad_1) as u32)).wrapping_mul(448))).wrapping_add(340)); F::Pad_TestMask(c, p_pad_1, a1_, p_mask_2) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Pad_IsRepeat", &PROBE_N, &[], true); }
    r_
}

pub fn Pad_GetSticks(c: &mut Ctx, mut p_pad_1: i32, mut p_left_2: u32, mut p_right_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Pad_GetSticks", &PROBE_N, &[(p_pad_1) as u32, (p_left_2) as u32, (p_right_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = F32(c.m.r32((((sym::gPad).wrapping_add((((p_pad_1) as u32)).wrapping_mul(448))).wrapping_add(304)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_left_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((sym::gPad).wrapping_add((((p_pad_1) as u32)).wrapping_mul(448))).wrapping_add(304)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (p_left_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((sym::gPad).wrapping_add((((p_pad_1) as u32)).wrapping_mul(448))).wrapping_add(312)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_right_3).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((sym::gPad).wrapping_add((((p_pad_1) as u32)).wrapping_mul(448))).wrapping_add(312)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (p_right_3).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Pad_GetSticks", &PROBE_N, &[], true); }
    r_
}

pub fn Pad_AddVibration(c: &mut Ctx, mut p_pad_1: i32, mut p_small_2: i32, mut p_power_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Pad_AddVibration", &PROBE_N, &[(p_pad_1) as u32, (p_small_2) as u32, (p_power_3).0], false); }
    let mut l_p_4: u32 = 0;
    let r_ = 'f: {
    l_p_4 = (sym::gPad).wrapping_add((((p_pad_1) as u32)).wrapping_mul(448));
    { let r_ = p_power_3; let a_ = (l_p_4).wrapping_add(352); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    { let r_ = (((p_small_2) != (0i32)) as i32); let a_ = (l_p_4).wrapping_add(356); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    if (((rt::flt(F32(0x3f800000), F32(c.m.r32((l_p_4).wrapping_add(352)))) as i32)) != 0) {
        { let v_ = F32(0x3f800000); let a_ = (l_p_4).wrapping_add(352); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Pad_AddVibration", &PROBE_N, &[], true); }
    r_
}

pub fn Pad_GetVibProfile(c: &mut Ctx, mut p_pad_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Pad_GetVibProfile", &PROBE_N, &[(p_pad_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
    l_p_2 = (sym::gPad).wrapping_add((((p_pad_1) as u32)).wrapping_mul(448));
    break 'f { let a0_ = (c.m.r32((l_p_2).wrapping_add(256)) as i32); F::sceVibGetProfile(c, a0_, (l_p_2).wrapping_add(302)) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Pad_GetVibProfile", &PROBE_N, &[], true); }
    r_
}

pub fn Pad_SendActuator(c: &mut Ctx, mut p_pad_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Pad_SendActuator", &PROBE_N, &[(p_pad_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let mut l_power_3: i32 = 0;
    let r_ = 'f: {
    l_p_2 = (sym::gPad).wrapping_add((((p_pad_1) as u32)).wrapping_mul(448));
    l_power_3 = rt::f2i(rt::fmul(F32(c.m.r32((l_p_2).wrapping_add(352))), F32(0x42fe0000)));
    if (((((l_power_3) >= (128i32)) as i32)) != 0) {
        l_power_3 = 127i32;
    }
    if (((((l_power_3) < (0i32)) as i32)) != 0) {
        l_power_3 = 0i32;
    }
    { let r_ = 1i32; let n_ = (l_power_3).wrapping_shl((r_) as u32); l_power_3 = n_; };
    { let r_ = (c.m.r32((l_p_2).wrapping_add(356)) as i32); let n_ = ((l_power_3) | (r_)); l_power_3 = n_; };
    { let v_ = ((l_power_3) as u16); let a_ = (l_p_2).wrapping_add(300); c.m.w16(a_, v_); };
    { let _ = { let a0_ = (c.m.r32((l_p_2).wrapping_add(256)) as i32); F::sceVibSetActParam(c, a0_, 1i32, (l_p_2).wrapping_add(302), 2i32, (l_p_2).wrapping_add(300)) }; };
    { let v_ = F32(0x00000000); let a_ = (l_p_2).wrapping_add(352); c.m.w32(a_, (v_).0); };
    { let v_ = 0i32; let a_ = (l_p_2).wrapping_add(356); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Pad_SendActuator", &PROBE_N, &[], true); }
    r_
}

pub fn Pad_Reset(c: &mut Ctx, mut p_pad_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Pad_Reset", &PROBE_N, &[(p_pad_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
    l_p_2 = (sym::gPad).wrapping_add((((p_pad_1) as u32)).wrapping_mul(448));
    { let v_ = 255i32; let a_ = ((sym::gPad).wrapping_add((((p_pad_1) as u32)).wrapping_mul(448))).wrapping_add(384); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((0i32) as u32); let a_ = ((sym::gPad).wrapping_add((((p_pad_1) as u32)).wrapping_mul(448))).wrapping_add(392); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = ((sym::gPad).wrapping_add((((p_pad_1) as u32)).wrapping_mul(448))).wrapping_add(396); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = ((sym::gPad).wrapping_add((((p_pad_1) as u32)).wrapping_mul(448))).wrapping_add(400); c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = ((sym::gPad).wrapping_add((((p_pad_1) as u32)).wrapping_mul(448))).wrapping_add(404); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((0i32) as u32); let a_ = ((sym::gPad).wrapping_add((((p_pad_1) as u32)).wrapping_mul(448))).wrapping_add(408); c.m.w32(a_, v_); };
    { let _ = F::memset(c, (l_p_2).wrapping_add(276), 0i32, 18u32); };
    { let v_ = ((255i32) as u8); let a_ = ((l_p_2).wrapping_add(276)).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((255i32) as u8); let a_ = ((l_p_2).wrapping_add(276)).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((128i32) as u8); let a_ = ((l_p_2).wrapping_add(276)).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((128i32) as u8); let a_ = ((l_p_2).wrapping_add(276)).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((128i32) as u8); let a_ = ((l_p_2).wrapping_add(276)).wrapping_add(((4i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((128i32) as u8); let a_ = ((l_p_2).wrapping_add(276)).wrapping_add(((5i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = F32(0x00000000); let a_ = ((l_p_2).wrapping_add(304)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = ((l_p_2).wrapping_add(304)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = ((l_p_2).wrapping_add(312)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = ((l_p_2).wrapping_add(312)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = 0i32; let a_ = (l_p_2).wrapping_add(320); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (l_p_2).wrapping_add(324); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_2).wrapping_add(328); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_2).wrapping_add(332); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_2).wrapping_add(336); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_2).wrapping_add(340); c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = (l_p_2).wrapping_add(344); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_2).wrapping_add(348); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Pad_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn Pad_IsDigitalOnly(c: &mut Ctx, mut p_pad_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Pad_IsDigitalOnly", &PROBE_N, &[(p_pad_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
    l_p_2 = (sym::gPad).wrapping_add((((p_pad_1) as u32)).wrapping_mul(448));
    break 'f (((((((((((((((((c.m.r8(((l_p_2).wrapping_add(268)).wrapping_add(((0i32) as u32)))) as i32)) == (249i32)) as i32)) != 0) && (((((((c.m.r8(((l_p_2).wrapping_add(268)).wrapping_add(((1i32) as u32)))) as i32)) == (255i32)) as i32)) != 0)) as i32)) != 0) && (((((((c.m.r8(((l_p_2).wrapping_add(268)).wrapping_add(((2i32) as u32)))) as i32)) == (0i32)) as i32)) != 0)) as i32)) != 0) && (((((((c.m.r8(((l_p_2).wrapping_add(268)).wrapping_add(((3i32) as u32)))) as i32)) == (0i32)) as i32)) != 0)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Pad_IsDigitalOnly", &PROBE_N, &[], true); }
    r_
}

pub fn Pad_TestMask(c: &mut Ctx, mut p_pad_1: i32, mut p_bits_2: u32, mut p_mask_3: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Pad_TestMask", &PROBE_N, &[(p_pad_1) as u32, (p_bits_2) as u32, (p_mask_3) as u32], false); }
    let r_ = 'f: {
    break 'f (((((p_bits_2) & (p_mask_3))) != (((0i32) as u32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Pad_TestMask", &PROBE_N, &[], true); }
    r_
}

pub fn Pad_StickToFloat(c: &mut Ctx, mut p_raw_1: u8, mut p_deadzone_2: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Pad_StickToFloat", &PROBE_N, &[(p_raw_1) as u32, (p_deadzone_2) as u32], false); }
    let mut l_value_3: F32 = F32(0);
    let r_ = 'f: {
    l_value_3 = rt::fsub(rt::i2f((p_raw_1) as i32), F32(0x42ff0000));
    if (((rt::flt(F32(0x00000000), l_value_3) as i32)) != 0) {
        { let r_ = rt::i2f(p_deadzone_2); let n_ = rt::fsub(l_value_3, r_); l_value_3 = n_; };
        if (((rt::flt(l_value_3, F32(0x00000000)) as i32)) != 0) {
            l_value_3 = F32(0x00000000);
        }
    } else {
        { let r_ = rt::i2f(p_deadzone_2); let n_ = rt::fadd(l_value_3, r_); l_value_3 = n_; };
        if (((rt::flt(F32(0x00000000), l_value_3) as i32)) != 0) {
            l_value_3 = F32(0x00000000);
        }
    }
    break 'f rt::fdiv(l_value_3, rt::fsub(F32(0x42ff0000), rt::i2f(p_deadzone_2)));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "Pad_StickToFloat", &PROBE_N, &[], true); }
    r_
}
