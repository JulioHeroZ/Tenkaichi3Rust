#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EftPart10_LinkGroup(c: &mut Ctx, mut p_head_1: u32, mut p_tail_2: u32, mut p_g_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_LinkGroup", &PROBE_N, &[(p_head_1) as u32, (p_tail_2) as u32, (p_g_3) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(p_head_1)) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = p_g_3; let a_ = p_head_1; c.m.w32(a_, v_); };
        { let v_ = p_g_3; let a_ = p_tail_2; c.m.w32(a_, v_); };
    } else {
        { let v_ = c.m.r32(p_tail_2); let a_ = (p_g_3).wrapping_add(16); c.m.w32(a_, v_); };
        { let v_ = p_g_3; let a_ = (c.m.r32(p_tail_2)).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = p_g_3; let a_ = p_tail_2; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_LinkGroup", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_UnlinkGroup(c: &mut Ctx, mut p_head_1: u32, mut p_tail_2: u32, mut p_p_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_UnlinkGroup", &PROBE_N, &[(p_head_1) as u32, (p_tail_2) as u32, (p_p_3) as u32], false); }
    let mut l_prev_4: u32 = 0;
    let mut l_a_5: u32 = 0;
    let mut l_b_6: u32 = 0;
    let r_ = 'f: {
    l_a_5 = c.m.r32(p_head_1);
    if (((((l_a_5) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    l_prev_4 = c.m.r32((p_p_3).wrapping_add(16));
    if (((((l_prev_4) == (((0i32) as u32))) as i32)) != 0) {
        p_p_3 = c.m.r32((p_p_3).wrapping_add(12));
        if (((((p_p_3) == (((0i32) as u32))) as i32)) != 0) {
            { let v_ = ((0i32) as u32); let a_ = p_head_1; c.m.w32(a_, v_); };
            { let v_ = ((0i32) as u32); let a_ = p_tail_2; c.m.w32(a_, v_); };
        } else {
            { let v_ = p_p_3; let a_ = p_head_1; c.m.w32(a_, v_); };
            { let v_ = ((0i32) as u32); let a_ = (p_p_3).wrapping_add(16); c.m.w32(a_, v_); };
        }
    } else {
        l_a_5 = c.m.r32((p_p_3).wrapping_add(12));
        if (((((l_a_5) == (((0i32) as u32))) as i32)) != 0) {
            { let v_ = l_prev_4; let a_ = p_tail_2; c.m.w32(a_, v_); };
            { let v_ = ((0i32) as u32); let a_ = (l_prev_4).wrapping_add(12); c.m.w32(a_, v_); };
        } else {
            l_b_6 = l_a_5;
            l_a_5 = l_prev_4;
            { let v_ = l_a_5; let a_ = (l_b_6).wrapping_add(16); c.m.w32(a_, v_); };
            { let v_ = l_b_6; let a_ = (l_a_5).wrapping_add(12); c.m.w32(a_, v_); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_UnlinkGroup", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_Emit(c: &mut Ctx, mut p_em_1: u32, mut p_g_2: u32, mut p_yaw_3: F32, mut p_pitch_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_Emit", &PROBE_N, &[(p_em_1) as u32, (p_g_2) as u32, (p_yaw_3).0, (p_pitch_4).0], false); }
    let mut l_p_5: u32 = 0;
    let mut l_i_6: u8 = 0;
    let mut l_c_7: F32 = F32(0);
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gEftPart10Mgr)).wrapping_add(53412); (c.m.r32(a_) as i32) }) >= (150i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gEftPart10Mgr)).wrapping_add(53412); c.m.w32(a_, (v_) as u32); };
    }
    l_i_6 = (({ let a_ = (c.m.r32(sym::gEftPart10Mgr)).wrapping_add(53412); (c.m.r32(a_) as i32) }) as u8);
    'l1: loop {
        'c2: {
            l_p_5 = ((c.m.r32(sym::gEftPart10Mgr)).wrapping_add(3008)).wrapping_add((((l_i_6) as u32)).wrapping_mul(336));
            { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            if (((((((l_i_6) as i32)) >= (150i32)) as i32)) != 0) {
                l_i_6 = ((0i32) as u8);
            }
            if ((((((((((c.m.r32((l_p_5).wrapping_add(320)) as i32)) == (0i32)) as i32)) != 0) && ((F::EftPart10_InitPtcl(c, l_p_5, p_em_1)) != 0)) as i32)) != 0) {
                l_c_7 = F::Port_cosf(c, p_yaw_3);
                { let v_ = rt::fmul(F::Port_sinf(c, p_pitch_4), l_c_7); let a_ = (l_p_5).wrapping_add(16); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul(F::Port_cosf(c, p_pitch_4), l_c_7); let a_ = ((l_p_5).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = F::Port_sinf(c, p_yaw_3); let a_ = ((l_p_5).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = ((l_p_5).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                F::Vec4_Copy(c, l_p_5, (l_p_5).wrapping_add(16));
                { let r_ = 1i32; let a_ = (l_p_5).wrapping_add(320); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                F::EftPart10_LinkPtcl(c, (p_g_2).wrapping_add(4), (p_g_2).wrapping_add(8), l_p_5);
                { let v_ = ((l_i_6) as i32); let a_ = (c.m.r32(sym::gEftPart10Mgr)).wrapping_add(53412); c.m.w32(a_, (v_) as u32); };
                break 'f;
            }
        }
        if !(((((((l_i_6) as i32)) != ({ let a_ = (c.m.r32(sym::gEftPart10Mgr)).wrapping_add(53412); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l1; }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_Emit", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_InitPtcl(c: &mut Ctx, mut p_p_1: u32, mut p_em_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_InitPtcl", &PROBE_N, &[(p_p_1) as u32, (p_em_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_def_3: u32 = 0;
    let mut l_life_4: F32 = F32(0);
    let mut l_t_5: F32 = F32(0);
    let r_ = 'f: {
        l_def_3 = c.m.r32((p_em_2).wrapping_add(48));
        F::Vec4_Set(c, p_p_1, F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x3f800000));
        { let a1_ = F32(c.m.r32((p_em_2).wrapping_add(312))); let a2_ = F32(c.m.r32((p_em_2).wrapping_add(320))); F::Vec4_Set(c, (p_p_1).wrapping_add(32), a1_, a2_, F32(0x00000000), F32(0x3f800000)) };
        { let v_ = rt::fadd(F32(c.m.r32((p_em_2).wrapping_add(288))), { let a0_ = F32(c.m.r32((p_em_2).wrapping_add(272))); let a1_ = rt::fadd(F32(c.m.r32((p_em_2).wrapping_add(272))), F32(c.m.r32((p_em_2).wrapping_add(280)))); F::Rand_FloatRange(c, a0_, a1_) }); let a_ = (p_p_1).wrapping_add(192); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((p_em_2).wrapping_add(296))); let a1_ = rt::fadd(F32(c.m.r32((p_em_2).wrapping_add(296))), F32(c.m.r32((p_em_2).wrapping_add(304)))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (p_p_1).wrapping_add(196); c.m.w32(a_, (v_).0); };
        { let r_ = F32(c.m.r32((p_em_2).wrapping_add(96))); let a_ = (p_p_1).wrapping_add(196); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        l_life_4 = rt::fmul({ let a0_ = F32(c.m.r32((p_em_2).wrapping_add(328))); let a1_ = rt::fadd(F32(c.m.r32((p_em_2).wrapping_add(328))), F32(c.m.r32((p_em_2).wrapping_add(336)))); F::Rand_FloatRange(c, a0_, a1_) }, F32(0x41f00000));
        { let v_ = F32(0x00000000); let a_ = (p_p_1).wrapping_add(316); c.m.w32(a_, (v_).0); };
        { let v_ = l_life_4; let a_ = (p_p_1).wrapping_add(312); c.m.w32(a_, (v_).0); };
        if (((rt::fle(l_life_4, F32(0x00000000)) as i32)) != 0) {
            break 'f 0i32;
        }
        l_t_5 = rt::fmul({ let a0_ = F32(c.m.r32((p_em_2).wrapping_add(344))); let a1_ = rt::fadd(F32(c.m.r32((p_em_2).wrapping_add(344))), F32(c.m.r32((p_em_2).wrapping_add(352)))); F::Rand_FloatRange(c, a0_, a1_) }, F32(0x41f00000));
        { let v_ = 0i32; let a_ = (p_p_1).wrapping_add(320); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((0i32) as u32); let a_ = (p_p_1).wrapping_add(324); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (p_p_1).wrapping_add(328); c.m.w32(a_, v_); };
        { let v_ = F32(0x00000000); let a_ = (p_p_1).wrapping_add(200); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_p_1).wrapping_add(204); c.m.w32(a_, (v_).0); };
        { let v_ = l_t_5; let a_ = (p_p_1).wrapping_add(308); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd({ let a0_ = F32(c.m.r32((p_em_2).wrapping_add(176))); let a1_ = rt::fadd(F32(c.m.r32((p_em_2).wrapping_add(176))), F32(c.m.r32((p_em_2).wrapping_add(184)))); F::Rand_FloatRange(c, a0_, a1_) }, F32(c.m.r32((p_em_2).wrapping_add(656)))), F32(0x40c90fda)); let a_ = (p_p_1).wrapping_add(208); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul({ let a0_ = F32(c.m.r32((p_em_2).wrapping_add(192))); let a1_ = rt::fadd(F32(c.m.r32((p_em_2).wrapping_add(192))), F32(c.m.r32((p_em_2).wrapping_add(200)))); F::Rand_FloatRange(c, a0_, a1_) }, F32(0x40490fda)); let a_ = (p_p_1).wrapping_add(212); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul({ let a0_ = F32(c.m.r32((p_em_2).wrapping_add(208))); let a1_ = rt::fadd(F32(c.m.r32((p_em_2).wrapping_add(208))), F32(c.m.r32((p_em_2).wrapping_add(216)))); F::Rand_FloatRange(c, a0_, a1_) }, F32(0x40490fda)); let a_ = (p_p_1).wrapping_add(216); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul({ let a0_ = F32(c.m.r32((p_em_2).wrapping_add(224))); let a1_ = rt::fadd(F32(c.m.r32((p_em_2).wrapping_add(224))), F32(c.m.r32((p_em_2).wrapping_add(232)))); F::Rand_FloatRange(c, a0_, a1_) }, F32(0x40490fda)); let a_ = (p_p_1).wrapping_add(220); c.m.w32(a_, (v_).0); };
        if (((((c.m.r32((l_def_3).wrapping_add(500)) as i32)) & (4i32))) != 0) {
            if ((((!((((F::Port_Rand(c)) & (1i32))) != 0)) as i32)) != 0) {
                { let v_ = rt::fneg(F32(c.m.r32((p_p_1).wrapping_add(220)))); let a_ = (p_p_1).wrapping_add(220); c.m.w32(a_, (v_).0); };
            }
        }
        if (((((((((c.m.r32((l_def_3).wrapping_add(500)) as i32)) & (2i32))) != 0) && (((rt::flt(F32(c.m.r32((p_p_1).wrapping_add(220))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
            F::Vec4_Set(c, (p_p_1).wrapping_add(160), F32(0x3f7c0000), F32(0x3c800000), F32(0x3c800000), F32(0x3c800000));
            F::Vec4_Set(c, (p_p_1).wrapping_add(176), F32(0x3f7c0000), F32(0x3f7c0000), F32(0x3c800000), F32(0x3f7c0000));
            { let r_ = 512i32; let a_ = (p_p_1).wrapping_add(320); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        } else {
            F::Vec4_Set(c, (p_p_1).wrapping_add(160), F32(0x3c800000), F32(0x3c800000), F32(0x3c800000), F32(0x3f7c0000));
            F::Vec4_Set(c, (p_p_1).wrapping_add(176), F32(0x3f7c0000), F32(0x3c800000), F32(0x3f7c0000), F32(0x3f7c0000));
        }
        if (((((c.m.r32((l_def_3).wrapping_add(500)) as i32)) & (16i32))) != 0) {
            { let v_ = rt::i2f({ let a1_ = rt::f2i(rt::fsub(F32(c.m.r32((p_em_2).wrapping_add(924))), F32(0x3f800000))); F::Rand_IntRange(c, 0i32, a1_) }); let a_ = (p_p_1).wrapping_add(304); c.m.w32(a_, (v_).0); };
        } else {
            { let v_ = F32(0x00000000); let a_ = (p_p_1).wrapping_add(304); c.m.w32(a_, (v_).0); };
        }
        { let v_ = { let a0_ = F32(c.m.r32(((p_em_2).wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((p_em_2).wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_em_2).wrapping_add(152)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((p_em_2).wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((p_em_2).wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_em_2).wrapping_add(152)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((p_em_2).wrapping_add(128)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((p_em_2).wrapping_add(128)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_em_2).wrapping_add(152)).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32((fp).wrapping_add(4))), F32(c.m.r32(fp))), rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(312))), F32(c.m.r32((l_def_3).wrapping_add(496))))); let a_ = (p_p_1).wrapping_add(228); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32((fp).wrapping_add(8))), F32(c.m.r32((fp).wrapping_add(4)))), rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(312))), rt::fsub(F32(0x3f800000), F32(c.m.r32((l_def_3).wrapping_add(496)))))); let a_ = (p_p_1).wrapping_add(232); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(fp)); let a_ = (p_p_1).wrapping_add(224); c.m.w32(a_, (v_).0); };
        if (((((c.m.r32((l_def_3).wrapping_add(500)) as i32)) & (128i32))) != 0) {
            { let v_ = rt::fmul(F32(c.m.r32((p_em_2).wrapping_add(376))), F32(0x41f00000)); let a_ = (p_p_1).wrapping_add(256); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (p_p_1).wrapping_add(252); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fsub(F32(c.m.r32(((p_em_2).wrapping_add(360)).wrapping_add(4))), F32(c.m.r32((p_em_2).wrapping_add(360)))); let a_ = (p_p_1).wrapping_add(300); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32((p_em_2).wrapping_add(360))); let a_ = (p_p_1).wrapping_add(248); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32((p_em_2).wrapping_add(360))); let a_ = (p_p_1).wrapping_add(244); c.m.w32(a_, (v_).0); };
        } else {
            { let v_ = F32(0x3f800000); let a_ = (p_p_1).wrapping_add(244); c.m.w32(a_, (v_).0); };
        }
        if (((((((c.m.r8((l_def_3).wrapping_add(525))) as i32)) == (2i32)) as i32)) != 0) {
            if ((((!((((F::Port_Rand(c)) & (1i32))) != 0)) as i32)) != 0) {
                { let r_ = 128i32; let a_ = (p_p_1).wrapping_add(320); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        } else {
            if (((((((c.m.r8((l_def_3).wrapping_add(525))) as i32)) != (1i32)) as i32)) != 0) {
                { let r_ = 128i32; let a_ = (p_p_1).wrapping_add(320); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
        { let v_ = rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(312))), rt::fsub(F32(c.m.r32((p_em_2).wrapping_add(592))), F32(c.m.r32((p_em_2).wrapping_add(584))))); let a_ = (p_p_1).wrapping_add(296); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_p_1).wrapping_add(292); c.m.w32(a_, (v_).0); };
        if (((rt::fle(F32(c.m.r32((p_p_1).wrapping_add(296))), F32(c.m.r32((p_p_1).wrapping_add(292)))) as i32)) != 0) {
            { let v_ = F32(c.m.r32((p_p_1).wrapping_add(312))); let a_ = (p_p_1).wrapping_add(296); c.m.w32(a_, (v_).0); };
        }
        { let v_ = { let a0_ = F32(c.m.r32((p_em_2).wrapping_add(400))); let a1_ = rt::fadd(F32(c.m.r32((p_em_2).wrapping_add(400))), F32(c.m.r32((p_em_2).wrapping_add(432)))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (p_p_1).wrapping_add(48); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((p_em_2).wrapping_add(400)).wrapping_add(4))); let a1_ = rt::fadd(F32(c.m.r32(((p_em_2).wrapping_add(400)).wrapping_add(4))), F32(c.m.r32(((p_em_2).wrapping_add(432)).wrapping_add(4)))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = ((p_p_1).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((p_em_2).wrapping_add(400)).wrapping_add(8))); let a1_ = rt::fadd(F32(c.m.r32(((p_em_2).wrapping_add(400)).wrapping_add(8))), F32(c.m.r32(((p_em_2).wrapping_add(432)).wrapping_add(8)))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = ((p_p_1).wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((p_em_2).wrapping_add(400)).wrapping_add(12))); let a1_ = rt::fadd(F32(c.m.r32(((p_em_2).wrapping_add(400)).wrapping_add(12))), F32(c.m.r32(((p_em_2).wrapping_add(432)).wrapping_add(12)))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = ((p_p_1).wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((p_em_2).wrapping_add(464))); let a1_ = rt::fadd(F32(c.m.r32((p_em_2).wrapping_add(464))), F32(c.m.r32((p_em_2).wrapping_add(496)))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((p_em_2).wrapping_add(464)).wrapping_add(4))); let a1_ = rt::fadd(F32(c.m.r32(((p_em_2).wrapping_add(464)).wrapping_add(4))), F32(c.m.r32(((p_em_2).wrapping_add(496)).wrapping_add(4)))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((p_em_2).wrapping_add(464)).wrapping_add(8))); let a1_ = rt::fadd(F32(c.m.r32(((p_em_2).wrapping_add(464)).wrapping_add(8))), F32(c.m.r32(((p_em_2).wrapping_add(496)).wrapping_add(8)))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec4_Clamp(c, (p_p_1).wrapping_add(48), (p_p_1).wrapping_add(48), F32(0x00000000), F32(0x437f0000));
        F::Vec4_Clamp(c, fp.wrapping_add(16), fp.wrapping_add(16), F32(0x00000000), F32(0x437f0000));
        F::Vec4_Copy(c, (p_p_1).wrapping_add(80), (p_p_1).wrapping_add(48));
        { let v_ = F32(0x00000000); let a_ = ((p_p_1).wrapping_add(80)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec3_Sub(c, (p_p_1).wrapping_add(64), fp.wrapping_add(16), (p_p_1).wrapping_add(80));
        { let v_ = rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(312))), F32(c.m.r32((l_def_3).wrapping_add(488)))); let a_ = (p_p_1).wrapping_add(236); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(312))), rt::fsub(F32(0x3f800000), F32(c.m.r32((l_def_3).wrapping_add(492))))); let a_ = (p_p_1).wrapping_add(240); c.m.w32(a_, (v_).0); };
        if (((((c.m.r32((l_def_3).wrapping_add(500)) as i32)) & (256i32))) != 0) {
            { let v_ = rt::fmul(F32(c.m.r32((p_em_2).wrapping_add(576))), F32(0x41f00000)); let a_ = (p_p_1).wrapping_add(288); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (p_p_1).wrapping_add(284); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fsub(F32(c.m.r32(((p_em_2).wrapping_add(528)).wrapping_add(4))), F32(c.m.r32((p_em_2).wrapping_add(528)))); let a_ = ((p_p_1).wrapping_add(260)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fsub(F32(c.m.r32(((p_em_2).wrapping_add(544)).wrapping_add(4))), F32(c.m.r32((p_em_2).wrapping_add(544)))); let a_ = ((p_p_1).wrapping_add(260)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fsub(F32(c.m.r32(((p_em_2).wrapping_add(560)).wrapping_add(4))), F32(c.m.r32((p_em_2).wrapping_add(560)))); let a_ = ((p_p_1).wrapping_add(260)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32((p_em_2).wrapping_add(528))); let a_ = ((p_p_1).wrapping_add(272)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32((p_em_2).wrapping_add(544))); let a_ = ((p_p_1).wrapping_add(272)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32((p_em_2).wrapping_add(560))); let a_ = ((p_p_1).wrapping_add(272)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            if (((rt::fne(F32(c.m.r32(((p_p_1).wrapping_add(260)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0x00000000)) as i32)) != 0) {
                { let r_ = 8192i32; let a_ = (p_p_1).wrapping_add(320); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
            if (((rt::fne(F32(c.m.r32(((p_p_1).wrapping_add(260)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(0x00000000)) as i32)) != 0) {
                { let r_ = 16384i32; let a_ = (p_p_1).wrapping_add(320); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
            if (((rt::fne(F32(c.m.r32(((p_p_1).wrapping_add(260)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(0x00000000)) as i32)) != 0) {
                { let r_ = 32768i32; let a_ = (p_p_1).wrapping_add(320); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_InitPtcl", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_BuildCorners(c: &mut Ctx, mut p_p_1: u32, mut p_em_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_BuildCorners", &PROBE_N, &[(p_p_1) as u32, (p_em_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_pitch_3: F32 = F32(0);
    let mut l_yaw_4: F32 = F32(0);
    let r_ = 'f: {
        { let a1_ = rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(224))), F32(0xbf000000)); let a2_ = rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(224))), F32(0xbf000000)); F::Vec4_Set(c, ((p_p_1).wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), a1_, a2_, F32(0x00000000), F32(0x3f800000)) };
        { let a1_ = rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(224))), F32(0xbf000000)); let a2_ = rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(224))), F32(0x3f000000)); F::Vec4_Set(c, ((p_p_1).wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), a1_, a2_, F32(0x00000000), F32(0x3f800000)) };
        { let a1_ = rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(224))), F32(0x3f000000)); let a2_ = rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(224))), F32(0xbf000000)); F::Vec4_Set(c, ((p_p_1).wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), a1_, a2_, F32(0x00000000), F32(0x3f800000)) };
        { let a1_ = rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(224))), F32(0x3f000000)); let a2_ = rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(224))), F32(0x3f000000)); F::Vec4_Set(c, ((p_p_1).wrapping_add(96)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), a1_, a2_, F32(0x00000000), F32(0x3f800000)) };
        { let a0_ = rt::fmul(rt::fmul(F32(c.m.r32((p_em_2).wrapping_add(96))), F32(c.m.r32((p_em_2).wrapping_add(616)))), F32(c.m.r32((p_p_1).wrapping_add(244)))); F::Vu0Cur_ScaleDiagUniform(c, a0_) };
        F::Vu0Cur_Translate(c, (p_p_1).wrapping_add(32));
        { let a0_ = F32(c.m.r32((p_p_1).wrapping_add(208))); F::Vu0Cur_RotateZ(c, a0_) };
        F::Vec3_Normalize(c, fp, p_p_1);
        l_pitch_3 = { let a0_ = { let a0_ = rt::fneg(F32(c.m.r32((fp).wrapping_add(4)))); F::Mathf_Asin(c, a0_) }; F::EftMath_WrapAngle(c, a0_) };
        l_yaw_4 = { let a0_ = { let a0_ = F32(c.m.r32(fp)); let a1_ = F32(c.m.r32((fp).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) }; F::EftMath_WrapAngle(c, a0_) };
        F::Vu0Cur_RotateX(c, l_pitch_3);
        F::Vu0Cur_RotateY(c, l_yaw_4);
        F::Vu0Cur_Translate(c, p_p_1);
        F::Vu0Cur_Translate(c, (p_em_2).wrapping_add(64));
        F::Vu0Cur_MulVec4(c, ((p_p_1).wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), ((p_p_1).wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
        F::Vu0Cur_MulVec4(c, ((p_p_1).wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), ((p_p_1).wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
        F::Vu0Cur_MulVec4(c, ((p_p_1).wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), ((p_p_1).wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
        F::Vu0Cur_MulVec4(c, ((p_p_1).wrapping_add(96)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), ((p_p_1).wrapping_add(96)).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_BuildCorners", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_LinkPtcl(c: &mut Ctx, mut p_head_1: u32, mut p_tail_2: u32, mut p_p_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_LinkPtcl", &PROBE_N, &[(p_head_1) as u32, (p_tail_2) as u32, (p_p_3) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(p_head_1)) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = p_p_3; let a_ = p_head_1; c.m.w32(a_, v_); };
        { let v_ = p_p_3; let a_ = p_tail_2; c.m.w32(a_, v_); };
    } else {
        { let v_ = c.m.r32(p_tail_2); let a_ = (p_p_3).wrapping_add(328); c.m.w32(a_, v_); };
        { let v_ = p_p_3; let a_ = (c.m.r32(p_tail_2)).wrapping_add(324); c.m.w32(a_, v_); };
        { let v_ = p_p_3; let a_ = p_tail_2; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_LinkPtcl", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_UnlinkPtcl(c: &mut Ctx, mut p_head_1: u32, mut p_tail_2: u32, mut p_p_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_UnlinkPtcl", &PROBE_N, &[(p_head_1) as u32, (p_tail_2) as u32, (p_p_3) as u32], false); }
    let mut l_prev_4: u32 = 0;
    let mut l_a_5: u32 = 0;
    let mut l_b_6: u32 = 0;
    let r_ = 'f: {
    l_a_5 = c.m.r32(p_head_1);
    if (((((l_a_5) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    l_prev_4 = c.m.r32((p_p_3).wrapping_add(328));
    if (((((l_prev_4) == (((0i32) as u32))) as i32)) != 0) {
        p_p_3 = c.m.r32((p_p_3).wrapping_add(324));
        if (((((p_p_3) == (((0i32) as u32))) as i32)) != 0) {
            { let v_ = ((0i32) as u32); let a_ = p_head_1; c.m.w32(a_, v_); };
            { let v_ = ((0i32) as u32); let a_ = p_tail_2; c.m.w32(a_, v_); };
        } else {
            { let v_ = p_p_3; let a_ = p_head_1; c.m.w32(a_, v_); };
            { let v_ = ((0i32) as u32); let a_ = (p_p_3).wrapping_add(328); c.m.w32(a_, v_); };
        }
    } else {
        l_a_5 = c.m.r32((p_p_3).wrapping_add(324));
        if (((((l_a_5) == (((0i32) as u32))) as i32)) != 0) {
            { let v_ = l_prev_4; let a_ = p_tail_2; c.m.w32(a_, v_); };
            { let v_ = ((0i32) as u32); let a_ = (l_prev_4).wrapping_add(324); c.m.w32(a_, v_); };
        } else {
            l_b_6 = l_a_5;
            l_a_5 = l_prev_4;
            { let v_ = l_a_5; let a_ = (l_b_6).wrapping_add(328); c.m.w32(a_, v_); };
            { let v_ = l_b_6; let a_ = (l_a_5).wrapping_add(324); c.m.w32(a_, v_); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_UnlinkPtcl", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_DrawQuad(c: &mut Ctx, mut p_corner_1: u32, p_uv0_2: u32, p_uv1_3: u32, p_color_4: u32, mut p_layer_5: i32, mut p_texIdx_6: i32, mut p_noDepth_7: i32, mut p_tex_8: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_DrawQuad", &PROBE_N, &[(p_corner_1) as u32, (p_uv0_2) as u32, (p_uv1_3) as u32, (p_color_4) as u32, (p_layer_5) as u32, (p_texIdx_6) as u32, (p_noDepth_7) as u32, (p_tex_8) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(256) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 256);
    let mut l_p_9: u32 = 0;
    let mut l_e_10: u32 = 0;
    let mut l_abe_11: i32 = 0;
    let mut l_ctx_12: i32 = 0;
    let mut l_z_13: i32 = 0;
    let mut l_l_14: i32 = 0;
    rt::memcpy(c, fp, p_uv0_2, 16);
    rt::memcpy(c, fp.wrapping_add(16), p_uv1_3, 16);
    rt::memcpy(c, fp.wrapping_add(32), p_color_4, 16);
    let r_ = 'f: {
        l_abe_11 = 1i32;
        { let a1_ = F32(c.m.r32(fp)); let a2_ = F32(c.m.r32((fp).wrapping_add(4))); F::Vec4_Set(c, (fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), a1_, a2_, F32(0x3f800000), F32(0x3f800000)) };
        { let a1_ = F32(c.m.r32((fp).wrapping_add(8))); let a2_ = F32(c.m.r32((fp).wrapping_add(12))); F::Vec4_Set(c, (fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), a1_, a2_, F32(0x3f800000), F32(0x3f800000)) };
        { let a1_ = F32(c.m.r32(fp.wrapping_add(16))); let a2_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); F::Vec4_Set(c, (fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), a1_, a2_, F32(0x3f800000), F32(0x3f800000)) };
        { let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); let a2_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(12))); F::Vec4_Set(c, (fp.wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), a1_, a2_, F32(0x3f800000), F32(0x3f800000)) };
        if ((((!((F::Vu0Cur_ProjectPointsStq(c, fp.wrapping_add(176), fp.wrapping_add(112), p_corner_1, fp.wrapping_add(48), 4i32)) != 0)) as i32)) != 0) {
            break 'f;
        }
        l_p_9 = c.m.r32(sym::gOtCur);
        { let v_ = (l_p_9).wrapping_add((((1i32) as u32)).wrapping_mul(144)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
        if (((((l_p_9) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        l_ctx_12 = (((p_layer_5) >= (2i32)) as i32);
        { let v_ = (((((((l_abe_11) as u64)).wrapping_shl((6i32) as u32)) | ((((l_ctx_12) as u64)).wrapping_shl((9i32) as u32)))) | (((28i32) as u64))); let a_ = (l_p_9).wrapping_add(32); c.m.w64(a_, v_); };
        { let v_ = ((536870920i32) as u32); let a_ = l_p_9; c.m.w32(a_, v_); };
        { let v_ = ((268435456i32) as u32); let a_ = (l_p_9).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = ((1342177288i32) as u32); let a_ = (l_p_9).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = 16429131440647602177u64; let a_ = (l_p_9).wrapping_add(16); c.m.w64(a_, v_); };
        { let v_ = (((18599481537995104i64).wrapping_add((((l_ctx_12).wrapping_shl((4i32) as u32)) as i64))) as u64); let a_ = (l_p_9).wrapping_add(24); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (l_p_9).wrapping_add(4); c.m.w32(a_, v_); };
        l_z_13 = (((((c.m.r32(((fp.wrapping_add(176)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)).wrapping_add((c.m.r32(((fp.wrapping_add(176)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add((c.m.r32(((fp.wrapping_add(176)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add((c.m.r32(((fp.wrapping_add(176)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_shr((10i32) as u32);
        if ((p_noDepth_7) != 0) {
            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(176)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(176)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(176)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(176)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        }
        F::Vec4_ToInt(c, fp.wrapping_add(240), fp.wrapping_add(32));
        { let v_ = (((c.m.r32(fp.wrapping_add(240)) as i32)) as u8); let a_ = ((l_p_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp.wrapping_add(240)).wrapping_add(4)) as i32)) as u8); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp.wrapping_add(240)).wrapping_add(8)) as i32)) as u8); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp.wrapping_add(240)).wrapping_add(12)) as i32)) as u8); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = (((c.m.r32(fp.wrapping_add(240)) as i32)) as u8); let a_ = ((l_p_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp.wrapping_add(240)).wrapping_add(4)) as i32)) as u8); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp.wrapping_add(240)).wrapping_add(8)) as i32)) as u8); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp.wrapping_add(240)).wrapping_add(12)) as i32)) as u8); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(112)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = (((c.m.r32(fp.wrapping_add(240)) as i32)) as u8); let a_ = ((l_p_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp.wrapping_add(240)).wrapping_add(4)) as i32)) as u8); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp.wrapping_add(240)).wrapping_add(8)) as i32)) as u8); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp.wrapping_add(240)).wrapping_add(12)) as i32)) as u8); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(112)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = (((c.m.r32(fp.wrapping_add(240)) as i32)) as u8); let a_ = ((l_p_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp.wrapping_add(240)).wrapping_add(4)) as i32)) as u8); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp.wrapping_add(240)).wrapping_add(8)) as i32)) as u8); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp.wrapping_add(240)).wrapping_add(12)) as i32)) as u8); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(112)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(16)))); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(112)).wrapping_add((((1i32) as u32)).wrapping_mul(16)))); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(112)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(112)).wrapping_add((((2i32) as u32)).wrapping_mul(16)))); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(112)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(112)).wrapping_add((((3i32) as u32)).wrapping_mul(16)))); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(112)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = (((c.m.r32((fp.wrapping_add(176)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(176)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(176)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32((fp.wrapping_add(176)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(176)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(176)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32((fp.wrapping_add(176)).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(176)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(176)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32((fp.wrapping_add(176)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(176)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(176)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = c.m.r64((p_tex_8).wrapping_add((((p_texIdx_6).wrapping_shl((4i32) as u32)) as u32))); let a_ = (l_p_9).wrapping_add(40); c.m.w64(a_, v_); };
        l_l_14 = p_layer_5;
        if (((((l_l_14) >= (2i32)) as i32)) != 0) {
            { let r_ = 2i32; let n_ = (l_l_14).wrapping_sub(r_); l_l_14 = n_; };
        }
        if (((((l_z_13) < (0i32)) as i32)) != 0) {
            l_e_10 = ((c.m.r32(sym::gOtZ)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_14) as u32)).wrapping_mul(8));
        } else {
            if (((((l_z_13) >= (4096i32)) as i32)) != 0) {
                l_e_10 = ((c.m.r32(sym::gOtZ)).wrapping_add((((4095i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_14) as u32)).wrapping_mul(8));
            } else {
                l_e_10 = ((c.m.r32(sym::gOtZ)).wrapping_add((((l_z_13) as u32)).wrapping_mul(16))).wrapping_add((((l_l_14) as u32)).wrapping_mul(8));
            }
        }
        { let v_ = l_p_9; let a_ = (c.m.r32((l_e_10).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = l_p_9; let a_ = (l_e_10).wrapping_add(4); c.m.w32(a_, v_); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_DrawQuad", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_DrawQuadClipped(c: &mut Ctx, mut p_corner_1: u32, p_uv0_2: u32, p_uv1_3: u32, p_color_4: u32, mut p_layer_5: i32, mut p_texIdx_6: i32, mut p_noDepth_7: i32, mut p_tex_8: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_DrawQuadClipped", &PROBE_N, &[(p_corner_1) as u32, (p_uv0_2) as u32, (p_uv1_3) as u32, (p_color_4) as u32, (p_layer_5) as u32, (p_texIdx_6) as u32, (p_noDepth_7) as u32, (p_tex_8) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(608) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 608);
    rt::memcpy(c, fp, p_uv0_2, 16);
    rt::memcpy(c, fp.wrapping_add(16), p_uv1_3, 16);
    rt::memcpy(c, fp.wrapping_add(32), p_color_4, 16);
    let r_ = 'f: {
        { let a1_ = F32(c.m.r32(fp)); let a2_ = F32(c.m.r32((fp).wrapping_add(4))); F::Vec4_Set(c, (fp.wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), a1_, a2_, F32(0x3f800000), F32(0x3f800000)) };
        { let a1_ = F32(c.m.r32((fp).wrapping_add(8))); let a2_ = F32(c.m.r32((fp).wrapping_add(12))); F::Vec4_Set(c, (fp.wrapping_add(112)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), a1_, a2_, F32(0x3f800000), F32(0x3f800000)) };
        { let a1_ = F32(c.m.r32(fp.wrapping_add(16))); let a2_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); F::Vec4_Set(c, (fp.wrapping_add(112)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), a1_, a2_, F32(0x3f800000), F32(0x3f800000)) };
        { let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); let a2_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(12))); F::Vec4_Set(c, (fp.wrapping_add(112)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), a1_, a2_, F32(0x3f800000), F32(0x3f800000)) };
        F::Vec4_Copy(c, (fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (p_corner_1).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
        F::Vec4_Copy(c, (fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (p_corner_1).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
        F::Vec4_Copy(c, (fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (p_corner_1).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
        F::Vec4_Copy(c, (fp.wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (p_corner_1).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
        F::ClipVtx_Set(c, (fp.wrapping_add(176)).wrapping_add((((0i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), fp.wrapping_add(32));
        F::ClipVtx_Set(c, (fp.wrapping_add(176)).wrapping_add((((1i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(112)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), fp.wrapping_add(32));
        F::ClipVtx_Set(c, (fp.wrapping_add(176)).wrapping_add((((2i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(112)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), fp.wrapping_add(32));
        { let a6_ = c.m.r64((p_tex_8).wrapping_add((((p_texIdx_6).wrapping_shl((4i32) as u32)) as u32))); F::EftGfx_DrawPolyScaledZ(c, fp.wrapping_add(176), p_layer_5, 1i32, 0i32, p_noDepth_7, 0i32, a6_, F32(0x40000000)) };
        F::ClipVtx_Set(c, (fp.wrapping_add(176)).wrapping_add((((0i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(112)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), fp.wrapping_add(32));
        F::ClipVtx_Set(c, (fp.wrapping_add(176)).wrapping_add((((1i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(112)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), fp.wrapping_add(32));
        F::ClipVtx_Set(c, (fp.wrapping_add(176)).wrapping_add((((2i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(112)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), fp.wrapping_add(32));
        { let a6_ = c.m.r64((p_tex_8).wrapping_add((((p_texIdx_6).wrapping_shl((4i32) as u32)) as u32))); F::EftGfx_DrawPolyScaledZ(c, fp.wrapping_add(176), p_layer_5, 1i32, 0i32, p_noDepth_7, 0i32, a6_, F32(0x40000000)) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_DrawQuadClipped", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_DrawBillboardClipped(c: &mut Ctx, mut p_pos_1: u32, mut p_w_2: F32, mut p_h_3: F32, mut p_color_4: u32, mut p_scale_5: u32, mut p_u0_6: F32, mut p_v0_7: F32, mut p_u1_8: F32, mut p_v1_9: F32, mut p_rot_10: F32, mut p_layer_11: i32, mut p_noDepth_12: i32, mut p_tex0_13: u64, mut p_zScale_14: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_DrawBillboardClipped", &PROBE_N, &[(p_pos_1) as u32, (p_w_2).0, (p_h_3).0, (p_color_4) as u32, (p_scale_5) as u32, (p_u0_6).0, (p_v0_7).0, (p_u1_8).0, (p_v1_9).0, (p_rot_10).0, (p_layer_11) as u32, (p_noDepth_12) as u32, (p_tex0_13) as u32, (p_zScale_14).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(704) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 704);
    let mut l_i_15: i32 = 0;
    let mut l_z_16: i32 = 0;
    let r_ = 'f: {
        F::Vec4_Set(c, (fp.wrapping_add(560)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), rt::fneg(p_w_2), rt::fneg(p_h_3), F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(560)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), p_w_2, rt::fneg(p_h_3), F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(560)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), rt::fneg(p_w_2), p_h_3, F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), p_w_2, p_h_3, F32(0x00000000), F32(0x3f800000));
        { let a1_ = (c.m.r32(sym::gEftPart10Mgr)).wrapping_add(53424); F::Mtx_Translate(c, fp.wrapping_add(432), a1_, p_scale_5) };
        F::Mtx_RotateZ(c, fp.wrapping_add(432), fp.wrapping_add(432), p_rot_10);
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(64); F::Mtx_InverseRT(c, fp.wrapping_add(496), a1_) };
        { let v_ = F32(0x00000000); let a_ = ((fp.wrapping_add(496)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((fp.wrapping_add(496)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((fp.wrapping_add(496)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        F::Mtx_Mul(c, fp.wrapping_add(432), fp.wrapping_add(496), fp.wrapping_add(432));
        {
            l_i_15 = 0i32;
            'l1: loop {
                if !(((((l_i_15) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::Mtx_MulVec4(c, (fp.wrapping_add(560)).wrapping_add((((l_i_15) as u32)).wrapping_mul(16)), fp.wrapping_add(432), (fp.wrapping_add(560)).wrapping_add((((l_i_15) as u32)).wrapping_mul(16)));
                    F::Vec3_Add(c, (fp.wrapping_add(560)).wrapping_add((((l_i_15) as u32)).wrapping_mul(16)), (fp.wrapping_add(560)).wrapping_add((((l_i_15) as u32)).wrapping_mul(16)), p_pos_1);
                    { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(560)).wrapping_add((((l_i_15) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_15; let n_ = (o_).wrapping_add(1); l_i_15 = n_; };
            }
        }
        { let _ = F::Vu0Cur_ProjectPoint(c, fp.wrapping_add(688), p_pos_1); };
        l_z_16 = ((c.m.r32((fp.wrapping_add(688)).wrapping_add(8)) as i32)).wrapping_shr((8i32) as u32);
        if ((((((c.m.r32((fp.wrapping_add(688)).wrapping_add(8)) as i32)) >= (0i32)) as i32)) != 0) {
            F::Vec4_Set(c, (fp.wrapping_add(624)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), p_u0_6, p_v0_7, F32(0x3f800000), F32(0x00000000));
            F::Vec4_Set(c, (fp.wrapping_add(624)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), p_u1_8, p_v0_7, F32(0x3f800000), F32(0x00000000));
            F::Vec4_Set(c, (fp.wrapping_add(624)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), p_u0_6, p_v1_9, F32(0x3f800000), F32(0x00000000));
            F::Vec4_Set(c, (fp.wrapping_add(624)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), p_u1_8, p_v1_9, F32(0x3f800000), F32(0x00000000));
            {
                l_i_15 = 0i32;
                'l3: loop {
                    if !(((((l_i_15) < (2i32)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        F::ClipVtx_Set(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(560)).wrapping_add((((l_i_15) as u32)).wrapping_mul(16)), (fp.wrapping_add(624)).wrapping_add((((l_i_15) as u32)).wrapping_mul(16)), p_color_4);
                        F::ClipVtx_Set(c, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(560)).wrapping_add(((((l_i_15).wrapping_add(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(624)).wrapping_add(((((l_i_15).wrapping_add(1i32)) as u32)).wrapping_mul(16)), p_color_4);
                        F::ClipVtx_Set(c, (fp).wrapping_add((((2i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(560)).wrapping_add(((((l_i_15).wrapping_add(2i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(624)).wrapping_add(((((l_i_15).wrapping_add(2i32)) as u32)).wrapping_mul(16)), p_color_4);
                        F::EftGfx_DrawPolyFixedZ(c, fp, p_layer_11, 0i32, 0i32, p_noDepth_12, 0i32, p_tex0_13, rt::f2i(rt::fmul(rt::i2f(l_z_16), p_zScale_14)));
                    }
                    { let o_ = l_i_15; let n_ = (o_).wrapping_add(1); l_i_15 = n_; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_DrawBillboardClipped", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_DrawBillboard(c: &mut Ctx, mut p_pos_1: u32, mut p_color_2: u32, mut p_w_3: F32, mut p_offX_4: i32, mut p_offY_5: i32, mut p_h_6: F32, mut p_u0_7: F32, mut p_v0_8: F32, mut p_layer_9: i32, mut p_u1_10: F32, mut p_v1_11: F32, mut p_noDepth_12: i32, mut p_tex0_13: u64, mut p_rot_14: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_DrawBillboard", &PROBE_N, &[(p_pos_1) as u32, (p_color_2) as u32, (p_w_3).0, (p_offX_4) as u32, (p_offY_5) as u32, (p_h_6).0, (p_u0_7).0, (p_v0_8).0, (p_layer_9) as u32, (p_u1_10).0, (p_v1_11).0, (p_noDepth_12) as u32, (p_tex0_13) as u32, (p_rot_14).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(224) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 224);
    let mut l_scale_15: F32 = F32(0);
    let mut l_sw_16: i32 = 0;
    let mut l_sh_17: i32 = 0;
    let mut l_k_18: i32 = 0;
    let mut l_oy_19: i32 = 0;
    let mut l_z_20: i32 = 0;
    let mut l_zz_21: i32 = 0;
    let mut l_x0_22: i32 = 0;
    let mut l_y0_23: i32 = 0;
    let mut l_x1_24: i32 = 0;
    let mut l_y1_25: i32 = 0;
    let mut l_x2_26: i32 = 0;
    let mut l_y2_27: i32 = 0;
    let mut l_x3_28: i32 = 0;
    let mut l_y3_29: i32 = 0;
    let mut l_abe_30: i32 = 0;
    let mut l_ctx_31: i32 = 0;
    let mut l_l_32: i32 = 0;
    let mut l_alpha_33: u32 = 0;
    let mut l_p_34: u32 = 0;
    let mut l_e_35: u32 = 0;
    let r_ = 'f: {
        l_scale_15 = { let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(564); F32(c.m.r32(a_)) };
        l_abe_30 = 1i32;
        { let a2_ = rt::fmul(F32(0x3f955555), F::Port_WideFactor(c)); F::Vec4_Set(c, fp.wrapping_add(64), F32(0x3f800000), a2_, F32(0x3f800000), F32(0x3f800000)) };
        l_sw_16 = rt::f2i(p_w_3);
        { let v_ = F32(0x3f800000); let a_ = (p_pos_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let _ = F::Vu0Cur_ProjectPoint(c, fp.wrapping_add(144), p_pos_1); };
        { let r_ = F32(0x45800000); let n_ = rt::fmul(l_scale_15, r_); l_scale_15 = n_; };
        l_z_20 = ((c.m.r32((fp.wrapping_add(144)).wrapping_add(8)) as i32)).wrapping_shr((8i32) as u32);
        l_sh_17 = rt::f2i(p_h_6);
        { let r_ = rt::i2f((c.m.r32((fp.wrapping_add(144)).wrapping_add(12)) as i32)); let n_ = rt::fdiv(l_scale_15, r_); l_scale_15 = n_; };
        l_k_18 = rt::f2i(l_scale_15);
        l_sw_16 = ((l_sw_16).wrapping_mul(l_k_18)).wrapping_shr((12i32) as u32);
        l_sh_17 = ((l_sh_17).wrapping_mul(l_k_18)).wrapping_shr((12i32) as u32);
        if (((((p_offX_4) >= (0i32)) as i32)) != 0) {
            p_offX_4 = ((p_offX_4).wrapping_mul(l_k_18)).wrapping_shr((12i32) as u32);
        } else {
            p_offX_4 = ((((p_offX_4).wrapping_neg()).wrapping_mul(l_k_18)).wrapping_shr((12i32) as u32)).wrapping_neg();
        }
        if (((((p_offY_5) >= (0i32)) as i32)) != 0) {
            l_oy_19 = ((p_offY_5).wrapping_mul(l_k_18)).wrapping_shr((12i32) as u32);
        } else {
            l_oy_19 = ((((p_offY_5).wrapping_neg()).wrapping_mul(l_k_18)).wrapping_shr((12i32) as u32)).wrapping_neg();
        }
        if (((((l_sw_16) < (2i32)) as i32)) != 0) {
            break 'f;
        }
        if (((((l_sh_17) < (2i32)) as i32)) != 0) {
            break 'f;
        }
        F::Vec4_Set(c, fp.wrapping_add(80), rt::i2f((p_offX_4).wrapping_sub(l_sw_16)), rt::i2f((l_oy_19).wrapping_sub(l_sh_17)), F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, fp.wrapping_add(96), rt::i2f((p_offX_4).wrapping_sub(l_sw_16)), rt::i2f((l_sh_17).wrapping_add(l_oy_19)), F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, fp.wrapping_add(112), rt::i2f((l_sw_16).wrapping_add(p_offX_4)), rt::i2f((l_oy_19).wrapping_sub(l_sh_17)), F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, fp.wrapping_add(128), rt::i2f((l_sw_16).wrapping_add(p_offX_4)), rt::i2f((l_sh_17).wrapping_add(l_oy_19)), F32(0x00000000), F32(0x3f800000));
        { let a1_ = (c.m.r32(sym::gEftPart10Mgr)).wrapping_add(53424); F::Mtx_RotateZ(c, fp, a1_, p_rot_14) };
        F::Mtx_MulVec4(c, fp.wrapping_add(80), fp, fp.wrapping_add(80));
        F::Mtx_MulVec4(c, fp.wrapping_add(96), fp, fp.wrapping_add(96));
        F::Mtx_MulVec4(c, fp.wrapping_add(112), fp, fp.wrapping_add(112));
        F::Mtx_MulVec4(c, fp.wrapping_add(128), fp, fp.wrapping_add(128));
        F::Vec4_Mul(c, fp.wrapping_add(80), fp.wrapping_add(80), fp.wrapping_add(64));
        F::Vec4_Mul(c, fp.wrapping_add(96), fp.wrapping_add(96), fp.wrapping_add(64));
        F::Vec4_Mul(c, fp.wrapping_add(112), fp.wrapping_add(112), fp.wrapping_add(64));
        F::Vec4_Mul(c, fp.wrapping_add(128), fp.wrapping_add(128), fp.wrapping_add(64));
        F::Vec4_ToInt(c, fp.wrapping_add(160), fp.wrapping_add(80));
        F::Vec4_ToInt(c, fp.wrapping_add(176), fp.wrapping_add(96));
        F::Vec4_ToInt(c, fp.wrapping_add(192), fp.wrapping_add(112));
        F::Vec4_ToInt(c, fp.wrapping_add(208), fp.wrapping_add(128));
        if (({ let a0_ = ((c.m.r32(fp.wrapping_add(144)) as i32)).wrapping_add((c.m.r32(fp.wrapping_add(160)) as i32)); let a1_ = ((c.m.r32((fp.wrapping_add(144)).wrapping_add(4)) as i32)).wrapping_add((c.m.r32((fp.wrapping_add(160)).wrapping_add(4)) as i32)); let a2_ = (c.m.r32((fp.wrapping_add(144)).wrapping_add(8)) as i32); F::EftPart10_IsCornerOffScreen(c, a0_, a1_, a2_) }) != 0) {
            break 'f;
        }
        if (({ let a0_ = ((c.m.r32(fp.wrapping_add(144)) as i32)).wrapping_add((c.m.r32(fp.wrapping_add(176)) as i32)); let a1_ = ((c.m.r32((fp.wrapping_add(144)).wrapping_add(4)) as i32)).wrapping_add((c.m.r32((fp.wrapping_add(176)).wrapping_add(4)) as i32)); let a2_ = (c.m.r32((fp.wrapping_add(144)).wrapping_add(8)) as i32); F::EftPart10_IsCornerOffScreen(c, a0_, a1_, a2_) }) != 0) {
            break 'f;
        }
        if (({ let a0_ = ((c.m.r32(fp.wrapping_add(144)) as i32)).wrapping_add((c.m.r32(fp.wrapping_add(192)) as i32)); let a1_ = ((c.m.r32((fp.wrapping_add(144)).wrapping_add(4)) as i32)).wrapping_add((c.m.r32((fp.wrapping_add(192)).wrapping_add(4)) as i32)); let a2_ = (c.m.r32((fp.wrapping_add(144)).wrapping_add(8)) as i32); F::EftPart10_IsCornerOffScreen(c, a0_, a1_, a2_) }) != 0) {
            break 'f;
        }
        if (({ let a0_ = ((c.m.r32(fp.wrapping_add(144)) as i32)).wrapping_add((c.m.r32(fp.wrapping_add(208)) as i32)); let a1_ = ((c.m.r32((fp.wrapping_add(144)).wrapping_add(4)) as i32)).wrapping_add((c.m.r32((fp.wrapping_add(208)).wrapping_add(4)) as i32)); let a2_ = (c.m.r32((fp.wrapping_add(144)).wrapping_add(8)) as i32); F::EftPart10_IsCornerOffScreen(c, a0_, a1_, a2_) }) != 0) {
            break 'f;
        }
        if ((p_noDepth_12) != 0) {
            { let v_ = 16777215i32; let a_ = (fp.wrapping_add(144)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        }
        l_zz_21 = (((c.m.r32((fp.wrapping_add(144)).wrapping_add(8)) as i32)).wrapping_shr((8i32) as u32)).wrapping_shl((8i32) as u32);
        l_x0_22 = ((c.m.r32(fp.wrapping_add(144)) as i32)).wrapping_add((c.m.r32(fp.wrapping_add(160)) as i32));
        l_y0_23 = ((c.m.r32((fp.wrapping_add(144)).wrapping_add(4)) as i32)).wrapping_add((c.m.r32((fp.wrapping_add(160)).wrapping_add(4)) as i32));
        l_x1_24 = ((c.m.r32(fp.wrapping_add(144)) as i32)).wrapping_add((c.m.r32(fp.wrapping_add(176)) as i32));
        l_y1_25 = ((c.m.r32((fp.wrapping_add(144)).wrapping_add(4)) as i32)).wrapping_add((c.m.r32((fp.wrapping_add(176)).wrapping_add(4)) as i32));
        l_x2_26 = ((c.m.r32(fp.wrapping_add(144)) as i32)).wrapping_add((c.m.r32(fp.wrapping_add(192)) as i32));
        l_y2_27 = ((c.m.r32((fp.wrapping_add(144)).wrapping_add(4)) as i32)).wrapping_add((c.m.r32((fp.wrapping_add(192)).wrapping_add(4)) as i32));
        l_x3_28 = ((c.m.r32(fp.wrapping_add(144)) as i32)).wrapping_add((c.m.r32(fp.wrapping_add(208)) as i32));
        l_y3_29 = ((c.m.r32((fp.wrapping_add(144)).wrapping_add(4)) as i32)).wrapping_add((c.m.r32((fp.wrapping_add(208)).wrapping_add(4)) as i32));
        l_p_34 = c.m.r32(sym::gOtCur);
        { let v_ = (l_p_34).wrapping_add((((1i32) as u32)).wrapping_mul(128)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
        if (((((l_p_34) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        l_ctx_31 = (((p_layer_9) >= (2i32)) as i32);
        { let v_ = (((((((l_abe_30) as u64)).wrapping_shl((6i32) as u32)) | ((((l_ctx_31) as u64)).wrapping_shl((9i32) as u32)))) | (((20i32) as u64))); let a_ = (l_p_34).wrapping_add(32); c.m.w64(a_, v_); };
        { let v_ = ((536870919i32) as u32); let a_ = l_p_34; c.m.w32(a_, v_); };
        { let v_ = ((268435456i32) as u32); let a_ = (l_p_34).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = ((1342177287i32) as u32); let a_ = (l_p_34).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = 14123288431433908225u64; let a_ = (l_p_34).wrapping_add(16); c.m.w64(a_, v_); };
        { let v_ = (((268436062347616i64).wrapping_add((((l_ctx_31).wrapping_shl((4i32) as u32)) as i64))) as u64); let a_ = (l_p_34).wrapping_add(24); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (l_p_34).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32(p_color_2)))) as u8); let a_ = (l_p_34).wrapping_add(48); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_color_2).wrapping_add(4))))) as u8); let a_ = ((l_p_34).wrapping_add(48)).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_color_2).wrapping_add(8))))) as u8); let a_ = ((l_p_34).wrapping_add(48)).wrapping_add(2); c.m.w8(a_, v_); };
        l_alpha_33 = rt::f2u(F32(c.m.r32((p_color_2).wrapping_add(12))));
        { let v_ = ((l_x0_22) as u64); let a_ = (l_p_34).wrapping_add(64); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((l_y0_23) as u64); let a_ = ((l_p_34).wrapping_add(64)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        l_l_32 = p_layer_9;
        { let v_ = ((l_x1_24) as u64); let a_ = (l_p_34).wrapping_add(80); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((l_y1_25) as u64); let a_ = ((l_p_34).wrapping_add(80)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((l_x2_26) as u64); let a_ = (l_p_34).wrapping_add(96); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((l_y2_27) as u64); let a_ = ((l_p_34).wrapping_add(96)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((l_x3_28) as u64); let a_ = (l_p_34).wrapping_add(112); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((l_y3_29) as u64); let a_ = ((l_p_34).wrapping_add(112)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((l_zz_21) as u64); let a_ = ((l_p_34).wrapping_add(64)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((l_zz_21) as u64); let a_ = ((l_p_34).wrapping_add(80)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((l_zz_21) as u64); let a_ = ((l_p_34).wrapping_add(96)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((l_zz_21) as u64); let a_ = ((l_p_34).wrapping_add(112)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        if (((((l_l_32) >= (2i32)) as i32)) != 0) {
            { let r_ = 2i32; let n_ = (l_l_32).wrapping_sub(r_); l_l_32 = n_; };
        }
        { let v_ = ((l_alpha_33) as u8); let a_ = ((l_p_34).wrapping_add(48)).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = F32(0x3f800000); let a_ = ((l_p_34).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_u0_7; let a_ = (l_p_34).wrapping_add(56); c.m.w32(a_, (v_).0); };
        { let v_ = p_v0_8; let a_ = ((l_p_34).wrapping_add(56)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_u0_7; let a_ = (l_p_34).wrapping_add(72); c.m.w32(a_, (v_).0); };
        { let v_ = p_v1_11; let a_ = ((l_p_34).wrapping_add(72)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_u1_10; let a_ = (l_p_34).wrapping_add(88); c.m.w32(a_, (v_).0); };
        { let v_ = p_v0_8; let a_ = ((l_p_34).wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_u1_10; let a_ = (l_p_34).wrapping_add(104); c.m.w32(a_, (v_).0); };
        { let v_ = p_v1_11; let a_ = ((l_p_34).wrapping_add(104)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = ((255i32) as u64); let a_ = ((l_p_34).wrapping_add(64)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((l_p_34).wrapping_add(80)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((l_p_34).wrapping_add(96)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((l_p_34).wrapping_add(112)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = p_tex0_13; let a_ = (l_p_34).wrapping_add(40); c.m.w64(a_, v_); };
        if (((((l_z_20) < (0i32)) as i32)) != 0) {
            l_e_35 = ((c.m.r32(sym::gOtZ)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_32) as u32)).wrapping_mul(8));
        } else {
            if (((((l_z_20) >= (4096i32)) as i32)) != 0) {
                l_e_35 = ((c.m.r32(sym::gOtZ)).wrapping_add((((4095i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_32) as u32)).wrapping_mul(8));
            } else {
                l_e_35 = ((c.m.r32(sym::gOtZ)).wrapping_add((((l_z_20) as u32)).wrapping_mul(16))).wrapping_add((((l_l_32) as u32)).wrapping_mul(8));
            }
        }
        { let v_ = l_p_34; let a_ = (c.m.r32((l_e_35).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = l_p_34; let a_ = (l_e_35).wrapping_add(4); c.m.w32(a_, v_); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_DrawBillboard", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_InitSpin(c: &mut Ctx, mut p_em_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_InitSpin", &PROBE_N, &[(p_em_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_def_2: u32 = 0;
    let r_ = 'f: {
        l_def_2 = c.m.r32((p_em_1).wrapping_add(48));
        if (((((c.m.r32((l_def_2).wrapping_add(500)) as i32)) & (32i32))) != 0) {
            { let v_ = rt::fmul({ let a0_ = F32(c.m.r32(((l_def_2).wrapping_add(388)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((l_def_2).wrapping_add(388)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(400)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }, F32(0x40c90fda)); let a_ = (p_em_1).wrapping_add(600); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul({ let a0_ = F32(c.m.r32(((l_def_2).wrapping_add(412)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((l_def_2).wrapping_add(412)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(424)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }, F32(0x40c90fda)); let a_ = (p_em_1).wrapping_add(604); c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = F32(c.m.r32(((l_def_2).wrapping_add(436)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((l_def_2).wrapping_add(436)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(448)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (p_em_1).wrapping_add(616); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32(((l_def_2).wrapping_add(460)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(620); c.m.w32(a_, (v_).0); };
            break 'f;
        }
        { let v_ = rt::fmul(F32(c.m.r32((l_def_2).wrapping_add(472))), F32(0x41f00000)); let a_ = (p_em_1).wrapping_add(948); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32((p_em_1).wrapping_add(948))), F32(c.m.r32((l_def_2).wrapping_add(476)))); let a_ = (p_em_1).wrapping_add(952); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul({ let a0_ = F32(c.m.r32(((l_def_2).wrapping_add(388)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((l_def_2).wrapping_add(388)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(400)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }, F32(0x40c90fda)); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul({ let a0_ = F32(c.m.r32(((l_def_2).wrapping_add(388)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((l_def_2).wrapping_add(388)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(400)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }, F32(0x40c90fda)); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul({ let a0_ = F32(c.m.r32(((l_def_2).wrapping_add(388)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((l_def_2).wrapping_add(388)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(400)).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }, F32(0x40c90fda)); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), F32(c.m.r32((p_em_1).wrapping_add(952)))); let a_ = ((p_em_1).wrapping_add(624)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(600); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), rt::fsub(F32(c.m.r32((p_em_1).wrapping_add(948))), F32(c.m.r32((p_em_1).wrapping_add(952))))); let a_ = ((p_em_1).wrapping_add(624)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul({ let a0_ = F32(c.m.r32(((l_def_2).wrapping_add(412)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((l_def_2).wrapping_add(412)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(424)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }, F32(0x40c90fda)); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul({ let a0_ = F32(c.m.r32(((l_def_2).wrapping_add(412)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((l_def_2).wrapping_add(412)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(424)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }, F32(0x40c90fda)); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul({ let a0_ = F32(c.m.r32(((l_def_2).wrapping_add(412)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((l_def_2).wrapping_add(412)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(424)).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }, F32(0x40c90fda)); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), F32(c.m.r32((p_em_1).wrapping_add(952)))); let a_ = ((p_em_1).wrapping_add(632)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(604); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), rt::fsub(F32(c.m.r32((p_em_1).wrapping_add(948))), F32(c.m.r32((p_em_1).wrapping_add(952))))); let a_ = ((p_em_1).wrapping_add(632)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((l_def_2).wrapping_add(436)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((l_def_2).wrapping_add(436)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(448)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((l_def_2).wrapping_add(436)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((l_def_2).wrapping_add(436)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(448)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((l_def_2).wrapping_add(436)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((l_def_2).wrapping_add(436)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(448)).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), F32(c.m.r32((p_em_1).wrapping_add(952)))); let a_ = ((p_em_1).wrapping_add(640)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(616); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), rt::fsub(F32(c.m.r32((p_em_1).wrapping_add(948))), F32(c.m.r32((p_em_1).wrapping_add(952))))); let a_ = ((p_em_1).wrapping_add(640)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32(((l_def_2).wrapping_add(460)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(460)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), F32(c.m.r32((p_em_1).wrapping_add(952)))); let a_ = ((p_em_1).wrapping_add(648)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32(((l_def_2).wrapping_add(460)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(460)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), rt::fsub(F32(c.m.r32((p_em_1).wrapping_add(948))), F32(c.m.r32((p_em_1).wrapping_add(952))))); let a_ = ((p_em_1).wrapping_add(648)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_def_2).wrapping_add(460)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(620); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_InitSpin", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_UpdateSpin(c: &mut Ctx, mut p_em_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_UpdateSpin", &PROBE_N, &[(p_em_1) as u32], false); }
    let r_ = 'f: {
    if ((((!(((({ let a_ = (c.m.r32((p_em_1).wrapping_add(48))).wrapping_add(500); (c.m.r32(a_) as i32) }) & (32i32))) != 0)) as i32)) != 0) {
        if (((rt::flt(F32(c.m.r32((p_em_1).wrapping_add(928))), F32(c.m.r32((p_em_1).wrapping_add(952)))) as i32)) != 0) {
            { let r_ = F32(c.m.r32(((p_em_1).wrapping_add(624)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(600); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = F32(c.m.r32(((p_em_1).wrapping_add(632)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(604); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = F32(c.m.r32(((p_em_1).wrapping_add(640)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(616); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = F32(c.m.r32(((p_em_1).wrapping_add(648)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(620); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        } else {
            if (((rt::flt(F32(c.m.r32((p_em_1).wrapping_add(928))), F32(c.m.r32((p_em_1).wrapping_add(948)))) as i32)) != 0) {
                { let r_ = F32(c.m.r32(((p_em_1).wrapping_add(624)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(600); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = F32(c.m.r32(((p_em_1).wrapping_add(632)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(604); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = F32(c.m.r32(((p_em_1).wrapping_add(640)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(616); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = F32(c.m.r32(((p_em_1).wrapping_add(648)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(620); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            }
        }
    }
    { let r_ = F32(c.m.r32((p_em_1).wrapping_add(600))); let a_ = (p_em_1).wrapping_add(608); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    { let r_ = F32(c.m.r32((p_em_1).wrapping_add(604))); let a_ = (p_em_1).wrapping_add(612); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    { let v_ = { let a0_ = F32(c.m.r32((p_em_1).wrapping_add(608))); F::EftMath_WrapAngle(c, a0_) }; let a_ = (p_em_1).wrapping_add(608); c.m.w32(a_, (v_).0); };
    { let v_ = { let a0_ = F32(c.m.r32((p_em_1).wrapping_add(612))); F::EftMath_WrapAngle(c, a0_) }; let a_ = (p_em_1).wrapping_add(612); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_UpdateSpin", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_StartKeys(c: &mut Ctx, mut p_em_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_StartKeys", &PROBE_N, &[(p_em_1) as u32], false); }
    let mut l_seg_2: i32 = 0;
    let mut l_def_3: u32 = 0;
    let mut l_def2_4: u32 = 0;
    let r_ = 'f: {
    l_seg_2 = 0i32;
    l_def_3 = c.m.r32((p_em_1).wrapping_add(48));
    l_def2_4 = c.m.r32((p_em_1).wrapping_add(52));
    if (((rt::fle(F32(c.m.r32((p_em_1).wrapping_add(928))), F32(0x00000000)) as i32)) != 0) {
        l_seg_2 = 1i32;
        { let v_ = rt::fmul(F32(c.m.r32((l_def_3).wrapping_add(480))), F32(0x41f00000)); let a_ = (p_em_1).wrapping_add(384); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32((p_em_1).wrapping_add(384))), F32(c.m.r32((l_def_3).wrapping_add(484)))); let a_ = (p_em_1).wrapping_add(388); c.m.w32(a_, (v_).0); };
    }
    if (((((((rt::fle(F32(c.m.r32((p_em_1).wrapping_add(388))), F32(c.m.r32((p_em_1).wrapping_add(928)))) as i32)) != 0) && ((((!(((((c.m.r32((p_em_1).wrapping_add(960)) as i32)) & (131072i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
        l_seg_2 = 2i32;
        { let r_ = 131072i32; let a_ = (p_em_1).wrapping_add(960); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if (((((l_seg_2) > (0i32)) as i32)) != 0) {
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add((((l_seg_2) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(140)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add((((l_seg_2) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(140)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add((((l_seg_2) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(140)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_3).wrapping_add(36)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_3).wrapping_add(36)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(164)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_3).wrapping_add(36)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_3).wrapping_add(36)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(164)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_3).wrapping_add(36)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_3).wrapping_add(36)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(164)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(72)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(72)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(176)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(84)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(84)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(184)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(96)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(96)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(192)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(120)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(120)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(208)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(144)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(144)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(224)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(108)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(108)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(200)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(132)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(132)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(216)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(156)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(156)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(232)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(168)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(168)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(240)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(180)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(180)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(248)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(192)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(192)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(256)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(204)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(204)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(264)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(216)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(216)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(272)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(228)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(228)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(280)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(240)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(240)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(288)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(252)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(252)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(296)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(264)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(264)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(304)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(276)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(276)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(312)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(288)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(288)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(320)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(300)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(300)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(328)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(312)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(312)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(336)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(324)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(324)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(344)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(336)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(336)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(352)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_3).wrapping_add(348)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_3).wrapping_add(348)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (p_em_1).wrapping_add(368); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_3).wrapping_add(348)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_3).wrapping_add(348)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(368)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(372)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(372)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(376)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def2_4).wrapping_add(192)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def2_4).wrapping_add(192)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (p_em_1).wrapping_add(536); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def2_4).wrapping_add(192)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def2_4).wrapping_add(192)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(536)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def2_4).wrapping_add(216)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def2_4).wrapping_add(216)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (p_em_1).wrapping_add(552); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def2_4).wrapping_add(216)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def2_4).wrapping_add(216)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(552)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def2_4).wrapping_add(240)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def2_4).wrapping_add(240)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (p_em_1).wrapping_add(568); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def2_4).wrapping_add(240)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def2_4).wrapping_add(240)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(568)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def2_4).wrapping_add(264)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def2_4).wrapping_add(264)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(576)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def2_4).wrapping_add(276)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def2_4).wrapping_add(276)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(584)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def2_4).wrapping_add(288)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def2_4).wrapping_add(288)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((p_em_1).wrapping_add(592)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        F::Vec4_Sub(c, (p_em_1).wrapping_add(416), (l_def2_4).wrapping_add((((l_seg_2) as u32)).wrapping_mul(16)), (l_def2_4).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(16)));
        F::Vec4_Sub(c, (p_em_1).wrapping_add(480), ((l_def2_4).wrapping_add(96)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(16)), ((l_def2_4).wrapping_add(96)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(16)));
        F::Vec4_Sub(c, (p_em_1).wrapping_add(448), ((l_def2_4).wrapping_add(48)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(16)), ((l_def2_4).wrapping_add(48)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(16)));
        F::Vec4_Sub(c, (p_em_1).wrapping_add(512), ((l_def2_4).wrapping_add(144)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(16)), ((l_def2_4).wrapping_add(144)).wrapping_add(((((l_seg_2).wrapping_sub(1i32)) as u32)).wrapping_mul(16)));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_StartKeys", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_UpdateKeys(c: &mut Ctx, mut p_em_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_UpdateKeys", &PROBE_N, &[(p_em_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_seg_2: i32 = 0;
    let mut l_def_3: u32 = 0;
    let mut l_def2_4: u32 = 0;
    let mut l_t_5: F32 = F32(0);
    let mut l_p_6: u32 = 0;
    let r_ = 'f: {
        l_def_3 = c.m.r32((p_em_1).wrapping_add(48));
        l_def2_4 = c.m.r32((p_em_1).wrapping_add(52));
        l_p_6 = fp;
        { let _ = F::memset(c, l_p_6, 0i32, 16u32); };
        l_seg_2 = 0i32;
        if ((((!(((((c.m.r32((p_em_1).wrapping_add(960)) as i32)) & (131072i32))) != 0)) as i32)) != 0) {
            l_t_5 = rt::fdiv(F32(c.m.r32((p_em_1).wrapping_add(928))), F32(c.m.r32((p_em_1).wrapping_add(388))));
        } else {
            l_seg_2 = 1i32;
            l_t_5 = rt::fdiv(rt::fsub(F32(c.m.r32((p_em_1).wrapping_add(928))), F32(c.m.r32((p_em_1).wrapping_add(388)))), rt::fsub(F32(c.m.r32((p_em_1).wrapping_add(384))), F32(c.m.r32((p_em_1).wrapping_add(388)))));
        }
        l_t_5 = (if (((rt::flt(l_t_5, F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(0x3f800000), l_t_5) as i32)) != 0) { F32(0x3f800000) } else { l_t_5 }) });
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add((((l_seg_2) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(140)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = ((p_em_1).wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add((((l_seg_2) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(140)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = ((p_em_1).wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add((((l_seg_2) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(140)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = ((p_em_1).wrapping_add(128)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_3).wrapping_add(36)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(164)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = ((p_em_1).wrapping_add(152)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_3).wrapping_add(36)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(164)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = ((p_em_1).wrapping_add(152)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_3).wrapping_add(36)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(164)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = ((p_em_1).wrapping_add(152)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(72)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(176)).wrapping_add(4))), l_t_5)); let a_ = (p_em_1).wrapping_add(176); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(84)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(184)).wrapping_add(4))), l_t_5)); let a_ = (p_em_1).wrapping_add(184); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(96)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(192)).wrapping_add(4))), l_t_5)); let a_ = (p_em_1).wrapping_add(192); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(120)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(208)).wrapping_add(4))), l_t_5)); let a_ = (p_em_1).wrapping_add(208); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(144)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(224)).wrapping_add(4))), l_t_5)); let a_ = (p_em_1).wrapping_add(224); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(108)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(200)).wrapping_add(4))), l_t_5)); let a_ = (p_em_1).wrapping_add(200); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(132)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(216)).wrapping_add(4))), l_t_5)); let a_ = (p_em_1).wrapping_add(216); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(156)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(232)).wrapping_add(4))), l_t_5)); let a_ = (p_em_1).wrapping_add(232); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(168)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(240)).wrapping_add(4))), l_t_5)); let a_ = (p_em_1).wrapping_add(240); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(192)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(256)).wrapping_add(4))), l_t_5)); let a_ = (p_em_1).wrapping_add(256); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(180)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(248)).wrapping_add(4))), l_t_5)); let a_ = (p_em_1).wrapping_add(248); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(204)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(264)).wrapping_add(4))), l_t_5)); let a_ = (p_em_1).wrapping_add(264); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(216)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(272)).wrapping_add(4))), l_t_5)); let a_ = (p_em_1).wrapping_add(272); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(228)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(280)).wrapping_add(4))), l_t_5)); let a_ = (p_em_1).wrapping_add(280); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(240)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(288)).wrapping_add(4))), l_t_5)); let a_ = (p_em_1).wrapping_add(288); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(252)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(296)).wrapping_add(4))), l_t_5)); let a_ = (p_em_1).wrapping_add(296); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(264)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(304)).wrapping_add(4))), l_t_5)); let a_ = (p_em_1).wrapping_add(304); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(276)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(312)).wrapping_add(4))), l_t_5)); let a_ = (p_em_1).wrapping_add(312); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(288)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(320)).wrapping_add(4))), l_t_5)); let a_ = (p_em_1).wrapping_add(320); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(300)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(328)).wrapping_add(4))), l_t_5)); let a_ = (p_em_1).wrapping_add(328); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(312)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(336)).wrapping_add(4))), l_t_5)); let a_ = (p_em_1).wrapping_add(336); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(324)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(344)).wrapping_add(4))), l_t_5)); let a_ = (p_em_1).wrapping_add(344); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(336)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(352)).wrapping_add(4))), l_t_5)); let a_ = (p_em_1).wrapping_add(352); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_3).wrapping_add(348)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((p_em_1).wrapping_add(368))), l_t_5)); let a_ = (p_em_1).wrapping_add(360); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_3).wrapping_add(348)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(368)).wrapping_add(4))), l_t_5)); let a_ = ((p_em_1).wrapping_add(360)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(372)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(376)).wrapping_add(4))), l_t_5)); let a_ = (p_em_1).wrapping_add(376); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def2_4).wrapping_add(192)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((p_em_1).wrapping_add(536))), l_t_5)); let a_ = (p_em_1).wrapping_add(528); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def2_4).wrapping_add(192)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(536)).wrapping_add(4))), l_t_5)); let a_ = ((p_em_1).wrapping_add(528)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def2_4).wrapping_add(216)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((p_em_1).wrapping_add(552))), l_t_5)); let a_ = (p_em_1).wrapping_add(544); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def2_4).wrapping_add(216)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(552)).wrapping_add(4))), l_t_5)); let a_ = ((p_em_1).wrapping_add(544)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def2_4).wrapping_add(240)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((p_em_1).wrapping_add(568))), l_t_5)); let a_ = (p_em_1).wrapping_add(560); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def2_4).wrapping_add(240)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(568)).wrapping_add(4))), l_t_5)); let a_ = ((p_em_1).wrapping_add(560)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def2_4).wrapping_add(264)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(576)).wrapping_add(4))), l_t_5)); let a_ = (p_em_1).wrapping_add(576); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def2_4).wrapping_add(276)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(584)).wrapping_add(4))), l_t_5)); let a_ = (p_em_1).wrapping_add(584); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def2_4).wrapping_add(288)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_em_1).wrapping_add(592)).wrapping_add(4))), l_t_5)); let a_ = (p_em_1).wrapping_add(592); c.m.w32(a_, (v_).0); };
        F::Vec3_Scale(c, l_p_6, (p_em_1).wrapping_add(416), l_t_5);
        F::Vec4_Add(c, (p_em_1).wrapping_add(400), (l_def2_4).wrapping_add((((l_seg_2) as u32)).wrapping_mul(16)), l_p_6);
        F::Vec3_Scale(c, l_p_6, (p_em_1).wrapping_add(480), l_t_5);
        F::Vec4_Add(c, (p_em_1).wrapping_add(464), ((l_def2_4).wrapping_add(96)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(16)), l_p_6);
        F::Vec3_Scale(c, l_p_6, (p_em_1).wrapping_add(448), l_t_5);
        F::Vec4_Add(c, (p_em_1).wrapping_add(432), ((l_def2_4).wrapping_add(48)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(16)), l_p_6);
        F::Vec3_Scale(c, l_p_6, (p_em_1).wrapping_add(512), l_t_5);
        F::Vec4_Add(c, (p_em_1).wrapping_add(496), ((l_def2_4).wrapping_add(144)).wrapping_add((((l_seg_2) as u32)).wrapping_mul(16)), l_p_6);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_UpdateKeys", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_SetKey(c: &mut Ctx, mut p_em_1: u32, mut p_idx_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_SetKey", &PROBE_N, &[(p_em_1) as u32, (p_idx_2) as u32], false); }
    let mut l_def_3: u32 = 0;
    let mut l_def2_4: u32 = 0;
    let r_ = 'f: {
    l_def_3 = c.m.r32((p_em_1).wrapping_add(48));
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add((((p_idx_2) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_em_1).wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add((((p_idx_2) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((p_em_1).wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add((((p_idx_2) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = ((p_em_1).wrapping_add(128)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def_3).wrapping_add(36)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_em_1).wrapping_add(152)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def_3).wrapping_add(36)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((p_em_1).wrapping_add(152)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def_3).wrapping_add(36)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = ((p_em_1).wrapping_add(152)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    l_def2_4 = c.m.r32((p_em_1).wrapping_add(52));
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(72)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(176); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(84)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(184); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(96)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(192); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(108)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(200); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(120)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(208); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(132)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(216); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(144)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(224); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(156)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(232); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(168)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(240); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(180)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(248); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(192)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(256); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(204)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(264); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(216)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(272); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(228)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(280); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(240)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(288); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(252)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(296); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(264)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(304); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(276)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(312); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(288)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(320); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(300)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(328); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(312)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(336); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(324)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(344); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(336)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(352); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def_3).wrapping_add(348)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(360); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def_3).wrapping_add(348)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((p_em_1).wrapping_add(360)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(372)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(376); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def2_4).wrapping_add(192)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(528); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def2_4).wrapping_add(192)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((p_em_1).wrapping_add(528)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def2_4).wrapping_add(216)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(544); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def2_4).wrapping_add(216)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((p_em_1).wrapping_add(544)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def2_4).wrapping_add(240)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(560); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def2_4).wrapping_add(240)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((p_em_1).wrapping_add(560)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def2_4).wrapping_add(264)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(576); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def2_4).wrapping_add(276)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(584); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def2_4).wrapping_add(288)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(4)))); let a_ = (p_em_1).wrapping_add(592); c.m.w32(a_, (v_).0); };
    F::Vec4_Copy(c, (p_em_1).wrapping_add(400), (l_def2_4).wrapping_add((((p_idx_2) as u32)).wrapping_mul(16)));
    F::Vec4_Copy(c, (p_em_1).wrapping_add(464), ((l_def2_4).wrapping_add(96)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(16)));
    F::Vec4_Copy(c, (p_em_1).wrapping_add(432), ((l_def2_4).wrapping_add(48)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(16)));
    F::Vec4_Copy(c, (p_em_1).wrapping_add(496), ((l_def2_4).wrapping_add(144)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(16)));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_SetKey", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_SelectTex(c: &mut Ctx, mut p_em_1: u32, mut p_tbl_2: u32, mut p_a_3: i32, mut p_b_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_SelectTex", &PROBE_N, &[(p_em_1) as u32, (p_tbl_2) as u32, (p_a_3) as u32, (p_b_4) as u32], false); }
    let r_ = 'f: {
    if (((((p_a_3) == (p_b_4)) as i32)) != 0) {
        { let v_ = ((p_b_4) as u8); let a_ = (p_em_1).wrapping_add(956); c.m.w8(a_, v_); };
    } else {
        { let v_ = (((p_a_3).wrapping_add(p_b_4)) as u8); let a_ = (p_em_1).wrapping_add(956); c.m.w8(a_, v_); };
    }
    { let s_ = (p_tbl_2).wrapping_add((((p_a_3) as u32)).wrapping_mul(16)); let d_ = (p_em_1).wrapping_add(8); rt::memcpy(c, d_, s_, 16); };
    { let s_ = (p_tbl_2).wrapping_add((((p_b_4) as u32)).wrapping_mul(16)); let d_ = (p_em_1).wrapping_add(24); rt::memcpy(c, d_, s_, 16); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_SelectTex", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_BuildTex(c: &mut Ctx, mut p_em_1: u32, mut p_tex_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_BuildTex", &PROBE_N, &[(p_em_1) as u32, (p_tex_2) as u32], false); }
    let mut l_t_3: u64 = 0;
    let r_ = 'f: {
    if (((((p_tex_2) != (((0i32) as u32))) as i32)) != 0) {
        if ((((!((((c.m.r32((p_tex_2).wrapping_add(260))) & ((((1i32).wrapping_shl((((c.m.r8((p_em_1).wrapping_add(956))) as i32)) as u32)) as u32)))) != 0)) as i32)) != 0) {
            l_t_3 = F::EftVram_AddImage(c, (p_em_1).wrapping_add(8), 1i32, 0i32);
            { let r_ = (F::EftVram_AddClut(c, (p_em_1).wrapping_add(24))).wrapping_shl((37i32) as u32); let n_ = ((l_t_3) | (r_)); l_t_3 = n_; };
            { let v_ = l_t_3; let a_ = (p_tex_2).wrapping_add((((c.m.r8((p_em_1).wrapping_add(956))) as u32)).wrapping_mul(16)); c.m.w64(a_, v_); };
            { let r_ = (((1i32).wrapping_shl((((c.m.r8((p_em_1).wrapping_add(956))) as i32)) as u32)) as u32); let a_ = (p_tex_2).wrapping_add(260); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_BuildTex", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_Create(c: &mut Ctx, mut p_arg_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_Create", &PROBE_N, &[(p_arg_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftPart10Mgr)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    if (((((p_arg_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    break 'f { let a0_ = { let a_ = c.m.r32(sym::gEftPart10Mgr); c.m.r32(a_) }; F::BtlTaskList_AddTail(c, a0_, sym::gEftPart10Class, p_arg_1) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_Create", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_Stop(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_Stop", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_em_2: u32 = 0;
    let mut l_now_3: i32 = 0;
    let r_ = 'f: {
    l_now_3 = 1i32;
    if (((((c.m.r32(sym::gEftPart10Mgr)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftPart10_Update)) as i32)) != 0) {
        break 'f;
    }
    l_em_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_em_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((c.m.r32((l_em_2).wrapping_add(960)) as i32)) & (524288i32))) != 0) {
        break 'f;
    }
    if ((((!(((((c.m.r32((l_em_2).wrapping_add(960)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f;
    }
    if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_em_2).wrapping_add(936)))) as i32)) != 0) {
        { let r_ = 8i32; let a_ = (l_em_2).wrapping_add(960); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        l_now_3 = 0i32;
    }
    if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_em_2).wrapping_add(944)))) as i32)) != 0) {
        { let r_ = 16i32; let a_ = (l_em_2).wrapping_add(960); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        l_now_3 = 0i32;
    }
    if ((l_now_3) != 0) {
        { let r_ = 4i32; let a_ = (l_em_2).wrapping_add(960); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    { let r_ = 524288i32; let a_ = (l_em_2).wrapping_add(960); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_Stop", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_SetFade(c: &mut Ctx, mut p_task_1: u32, mut p_frames_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_SetFade", &PROBE_N, &[(p_task_1) as u32, (p_frames_2) as u32], false); }
    let mut l_em_3: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftPart10Mgr)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftPart10_Update)) as i32)) != 0) {
        break 'f;
    }
    l_em_3 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_em_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((c.m.r32((l_em_3).wrapping_add(960)) as i32)) & (524288i32))) != 0) {
        break 'f;
    }
    if (((((c.m.r32((l_em_3).wrapping_add(960)) as i32)) & (1i32))) != 0) {
        { let v_ = rt::i2f(p_frames_2); let a_ = (l_em_3).wrapping_add(944); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f(p_frames_2); let a_ = (l_em_3).wrapping_add(940); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_SetFade", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_Kill(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_Kill", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_em_2: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftPart10Mgr)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftPart10_Update)) as i32)) != 0) {
        break 'f;
    }
    l_em_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_em_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((c.m.r32((l_em_2).wrapping_add(960)) as i32)) & (1i32))) != 0) {
        { let r_ = 32i32; let a_ = (l_em_2).wrapping_add(960); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_Kill", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_SetPos(c: &mut Ctx, mut p_task_1: u32, p_pos_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_SetPos", &PROBE_N, &[(p_task_1) as u32, (p_pos_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_em_3: u32 = 0;
    rt::memcpy(c, fp, p_pos_2, 16);
    let r_ = 'f: {
        if (((((c.m.r32(sym::gEftPart10Mgr)) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftPart10_Update)) as i32)) != 0) {
            break 'f;
        }
        l_em_3 = c.m.r32((p_task_1).wrapping_add(56));
        if (((((l_em_3) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if (((((c.m.r32((l_em_3).wrapping_add(960)) as i32)) & (1i32))) != 0) {
            F::Vec4_Copy(c, (l_em_3).wrapping_add(64), fp);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_SetPos", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_Warp(c: &mut Ctx, mut p_task_1: u32, p_pos_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_Warp", &PROBE_N, &[(p_task_1) as u32, (p_pos_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_em_3: u32 = 0;
    let mut l_g_4: u32 = 0;
    let mut l_p_5: u32 = 0;
    rt::memcpy(c, fp, p_pos_2, 16);
    let r_ = 'f: {
        if (((((c.m.r32(sym::gEftPart10Mgr)) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftPart10_Update)) as i32)) != 0) {
            break 'f;
        }
        l_em_3 = c.m.r32((p_task_1).wrapping_add(56));
        if (((((l_em_3) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if ((((!(((((c.m.r32((l_em_3).wrapping_add(960)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
            break 'f;
        }
        F::Vu0Cur_Push(c);
        F::Vec4_Copy(c, (l_em_3).wrapping_add(64), fp);
        {
            l_g_4 = c.m.r32((l_em_3).wrapping_add(964));
            'l1: loop {
                if !(((((l_g_4) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    {
                        l_p_5 = c.m.r32((l_g_4).wrapping_add(4));
                        'l3: loop {
                            if !(((((l_p_5) != (((0i32) as u32))) as i32)) != 0) { break 'l3; }
                            'c4: {
                                F::Vu0Cur_LoadIdentity(c);
                                F::Vu0Cur_Translate(c, fp);
                                F::Vu0Cur_MulVec4(c, ((l_p_5).wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), ((l_p_5).wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
                                F::Vu0Cur_MulVec4(c, ((l_p_5).wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), ((l_p_5).wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
                                F::Vu0Cur_MulVec4(c, ((l_p_5).wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), ((l_p_5).wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
                                F::Vu0Cur_MulVec4(c, ((l_p_5).wrapping_add(96)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), ((l_p_5).wrapping_add(96)).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
                            }
                            l_p_5 = c.m.r32((l_p_5).wrapping_add(324));
                        }
                    }
                }
                l_g_4 = c.m.r32((l_g_4).wrapping_add(12));
            }
        }
        F::Vu0Cur_Pop(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_Warp", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_SetDir(c: &mut Ctx, mut p_task_1: u32, p_dir_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_SetDir", &PROBE_N, &[(p_task_1) as u32, (p_dir_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_em_3: u32 = 0;
    rt::memcpy(c, fp, p_dir_2, 16);
    let r_ = 'f: {
        if (((((c.m.r32(sym::gEftPart10Mgr)) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftPart10_Update)) as i32)) != 0) {
            break 'f;
        }
        l_em_3 = c.m.r32((p_task_1).wrapping_add(56));
        if (((((l_em_3) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if (((((c.m.r32((l_em_3).wrapping_add(960)) as i32)) & (1i32))) != 0) {
            F::Vec4_Copy(c, (l_em_3).wrapping_add(80), fp);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_SetDir", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_SetSize(c: &mut Ctx, mut p_task_1: u32, mut p_size_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_SetSize", &PROBE_N, &[(p_task_1) as u32, (p_size_2).0], false); }
    let mut l_em_3: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftPart10Mgr)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftPart10_Update)) as i32)) != 0) {
        break 'f;
    }
    l_em_3 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_em_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((c.m.r32((l_em_3).wrapping_add(960)) as i32)) & (1i32))) != 0) {
        { let v_ = p_size_2; let a_ = (l_em_3).wrapping_add(96); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_SetSize", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_SetRate(c: &mut Ctx, mut p_task_1: u32, mut p_rate_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_SetRate", &PROBE_N, &[(p_task_1) as u32, (p_rate_2).0], false); }
    let mut l_em_3: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftPart10Mgr)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftPart10_Update)) as i32)) != 0) {
        break 'f;
    }
    l_em_3 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_em_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((c.m.r32((l_em_3).wrapping_add(960)) as i32)) & (1i32))) != 0) {
        { let v_ = p_rate_2; let a_ = (l_em_3).wrapping_add(100); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_SetRate", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_SetTex(c: &mut Ctx, mut p_task_1: u32, mut p_tex_2: u32, mut p_a_3: i32, mut p_b_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_SetTex", &PROBE_N, &[(p_task_1) as u32, (p_tex_2) as u32, (p_a_3) as u32, (p_b_4) as u32], false); }
    let mut l_em_5: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftPart10Mgr)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftPart10_Update)) as i32)) != 0) {
        break 'f;
    }
    l_em_5 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_em_5) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((c.m.r32((l_em_5).wrapping_add(960)) as i32)) & (1i32))) != 0) {
        { let v_ = p_tex_2; let a_ = (l_em_5).wrapping_add(56); c.m.w32(a_, v_); };
        { let v_ = p_a_3; let a_ = (l_em_5).wrapping_add(104); c.m.w32(a_, (v_) as u32); };
        { let v_ = p_b_4; let a_ = (l_em_5).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
        F::EftPart10_SelectTex(c, l_em_5, p_tex_2, p_a_3, p_b_4);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_SetTex", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_SetDelay(c: &mut Ctx, mut p_task_1: u32, mut p_frames_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_SetDelay", &PROBE_N, &[(p_task_1) as u32, (p_frames_2) as u32], false); }
    let mut l_em_3: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftPart10Mgr)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftPart10_Update)) as i32)) != 0) {
        break 'f;
    }
    l_em_3 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_em_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((c.m.r32((l_em_3).wrapping_add(960)) as i32)) & (1i32))) != 0) {
        { let v_ = rt::i2f(p_frames_2); let a_ = (l_em_3).wrapping_add(932); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_SetDelay", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_SetHold(c: &mut Ctx, mut p_task_1: u32, mut p_frames_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_SetHold", &PROBE_N, &[(p_task_1) as u32, (p_frames_2) as u32], false); }
    let mut l_em_3: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftPart10Mgr)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftPart10_Update)) as i32)) != 0) {
        break 'f;
    }
    l_em_3 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_em_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((c.m.r32((l_em_3).wrapping_add(960)) as i32)) & (1i32))) != 0) {
        { let v_ = rt::i2f(p_frames_2); let a_ = (l_em_3).wrapping_add(936); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_SetHold", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_SetNoDepth(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_SetNoDepth", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_em_2: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftPart10Mgr)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftPart10_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_em_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_em_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32((l_em_2).wrapping_add(960)) as i32)) & (1i32))) != 0) {
        { let r_ = 1024i32; let a_ = (l_em_2).wrapping_add(960); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_SetNoDepth", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_SetKind(c: &mut Ctx, mut p_task_1: u32, mut p_kind_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_SetKind", &PROBE_N, &[(p_task_1) as u32, (p_kind_2) as u32], false); }
    let mut l_em_3: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftPart10Mgr)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftPart10_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_em_3 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_em_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32((l_em_3).wrapping_add(960)) as i32)) & (1i32))) != 0) {
        { let v_ = ((p_kind_2) as u8); let a_ = (l_em_3).wrapping_add(957); c.m.w8(a_, v_); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_SetKind", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_IsAlive(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_IsAlive", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_em_2: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftPart10Mgr)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftPart10_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_em_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_em_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32((l_em_2).wrapping_add(960)) as i32)) & (1i32))) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_IsAlive", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_IsCornerOffScreen(c: &mut Ctx, mut p_x_1: i32, mut p_y_2: i32, mut p_z_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_IsCornerOffScreen", &PROBE_N, &[(p_x_1) as u32, (p_y_2) as u32, (p_z_3) as u32], false); }
    let r_ = 'f: {
    if (((((p_z_3) <= (0i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    if (((((p_x_1) > (65519i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    if (((((p_x_1) <= (0i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    if (((((p_y_2) > (65519i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    if (((((p_y_2) <= (0i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_IsCornerOffScreen", &PROBE_N, &[], true); }
    r_
}
