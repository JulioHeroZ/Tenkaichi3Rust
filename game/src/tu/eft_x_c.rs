#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn eft_x_c__EftQuad_Clamp(c: &mut Ctx, mut p_x_1: F32, mut p_lo_2: F32, mut p_hi_3: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_x_c__EftQuad_Clamp", &PROBE_N, &[(p_x_1).0, (p_lo_2).0, (p_hi_3).0], false); }
    let r_ = 'f: {
    if (((rt::flt(p_x_1, p_lo_2) as i32)) != 0) {
        break 'f p_lo_2;
    }
    if (((rt::flt(p_hi_3, p_x_1) as i32)) != 0) {
        break 'f p_hi_3;
    }
    break 'f p_x_1;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "eft_x_c__EftQuad_Clamp", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuadMgr_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuadMgr_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((76816u32) as i32)) }; let a_ = sym::gEftQuadMgr; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gEftQuadMgr); F::memset(c, a0_, 0i32, 76816u32) }; };
    { let v_ = F::BtlTask_CreateChildList(c, p_task_1, ((((30i32) as i64 as u64)) as u32), ((((((976u32) as i32)) as i64 as u64)) as u32)); let a_ = (c.m.r32(sym::gEftQuadMgr)).wrapping_add(76804); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftQuadMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuadMgr_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuadMgr_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftQuadMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuadMgr_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuadMgr_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let _ = { let a0_ = c.m.r32(sym::gEftQuadMgr); F::memset(c, a0_, 0i32, 76800u32) }; };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftQuadMgr_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuadMgr_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuadMgr_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftQuadMgr); F::BtlPool_Free(c, a0_, a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftQuadMgr; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftQuadMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_em_3: u32 = 0;
    let mut l_def_4: u32 = 0;
    let r_ = 'f: {
    l_em_3 = c.m.r32((p_task_1).wrapping_add(56));
    l_def_4 = c.m.r32(p_arg_2);
    { let _ = F::memset(c, l_em_3, 0i32, 976u32); };
    { let v_ = c.m.r32(p_arg_2); let a_ = l_em_3; c.m.w32(a_, v_); };
    { let v_ = c.m.r32((p_arg_2).wrapping_add(4)); let a_ = (l_em_3).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = c.m.r8((p_arg_2).wrapping_add(56)); let a_ = (l_em_3).wrapping_add(957); c.m.w8(a_, v_); };
    { let s_ = (p_arg_2).wrapping_add(16); let d_ = (l_em_3).wrapping_add(48); rt::memcpy(c, d_, s_, 16); };
    { let s_ = (p_arg_2).wrapping_add(32); let d_ = (l_em_3).wrapping_add(64); rt::memcpy(c, d_, s_, 16); };
    F::Vec3_Normalize(c, (l_em_3).wrapping_add(64), (l_em_3).wrapping_add(64));
    { let v_ = F32(c.m.r32((p_arg_2).wrapping_add(48))); let a_ = (l_em_3).wrapping_add(92); c.m.w32(a_, (v_).0); };
    { let v_ = rt::i2f((c.m.r8((l_def_4).wrapping_add(570))) as i32); let a_ = (l_em_3).wrapping_add(948); c.m.w32(a_, (v_).0); };
    { let v_ = rt::i2f((c.m.r8((l_def_4).wrapping_add(573))) as i32); let a_ = (l_em_3).wrapping_add(952); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(F32(c.m.r32((p_arg_2).wrapping_add(52))), F32(0x41f00000)); let a_ = (l_em_3).wrapping_add(96); c.m.w32(a_, (v_).0); };
    if (((((((((((c.m.r8((l_def_4).wrapping_add(575))) as i32)) >= (2i32)) as i32)) != 0) || (((((((c.m.r8((l_def_4).wrapping_add(576))) as i32)) >= (2i32)) as i32)) != 0)) as i32)) != 0) {
        { let a1_ = ((c.m.r8((l_def_4).wrapping_add(575))) as i32); let a2_ = ((c.m.r8((l_def_4).wrapping_add(576))) as i32); F::EftQuad_SetSheet(c, l_em_3, ((((a1_) as i64 as u64)) as u8), ((((a2_) as i64 as u64)) as u8)) };
        { let r_ = 1024i32; let a_ = (l_em_3).wrapping_add(964); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    { let a1_ = c.m.r32((p_arg_2).wrapping_add(8)); F::EftQuad_SetTex(c, l_em_3, a1_, 0i32, 0i32) };
    { let v_ = ((0i32) as u32); let a_ = (l_em_3).wrapping_add(968); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_em_3).wrapping_add(972); c.m.w32(a_, v_); };
    { let r_ = 1i32; let a_ = (l_em_3).wrapping_add(964); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    if (((((c.m.r32((l_def_4).wrapping_add(564)) as i32)) & (16i32))) != 0) {
        F::EftQuad_CalcKeyDeltas(c, p_task_1);
        { let r_ = 8192i32; let a_ = (l_em_3).wrapping_add(964); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        F::EftQuad_SetLastKey(c, l_em_3);
        { let v_ = F32(0x00000000); let a_ = ((l_em_3).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((l_em_3).wrapping_add(80)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((l_em_3).wrapping_add(80)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_em_2: u32 = 0;
    let mut l_def_3: u32 = 0;
    let mut l_t_4: F32 = F32(0);
    let mut l_i_5: i32 = 0;
    let mut l_r_6: F32 = F32(0);
    let r_ = 'f: {
    l_em_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_def_3 = c.m.r32(l_em_2);
    l_t_4 = F32(0x3f800000);
    if ((((!(({ let a0_ = ((c.m.r8((l_em_2).wrapping_add(957))) as i32); let a1_ = ((c.m.r8((l_em_2).wrapping_add(960))) as i32); F::BtlScene_IsEffectStopped(c, a0_, a1_) }) != 0)) as i32)) != 0) {
        if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_em_2).wrapping_add(104)))) as i32)) != 0) {
            { let r_ = l_t_4; let a_ = (l_em_2).wrapping_add(104); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            break 'f;
        }
        if (((((((((c.m.r32((l_em_2).wrapping_add(964)) as i32)) & (8i32))) != 0) && ((((!(((((c.m.r32((l_em_2).wrapping_add(964)) as i32)) & (4i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
            l_r_6 = rt::fdiv(F32(c.m.r32((l_em_2).wrapping_add(116))), F32(c.m.r32((l_em_2).wrapping_add(112))));
            l_r_6 = F::eft_x_c__EftQuad_Clamp(c, l_r_6, F32(0x00000000), F32(0x3f800000));
            { let r_ = F32(0x3f800000); let a_ = (l_em_2).wrapping_add(116); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::flt(F32(c.m.r32((l_em_2).wrapping_add(116))), F32(0x00000000)) as i32)) != 0) {
                { let r_ = 16i32; let a_ = (l_em_2).wrapping_add(964); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
        if (((((c.m.r32((l_em_2).wrapping_add(964)) as i32)) & (8192i32))) != 0) {
            F::EftQuad_Animate(c, p_task_1);
        }
        if ((((!(((((c.m.r32((l_em_2).wrapping_add(964)) as i32)) & (2i32))) != 0)) as i32)) != 0) {
            if (((((rt::rem_i32(rt::f2i(F32(c.m.r32((l_em_2).wrapping_add(100)))), rt::f2i(F32(c.m.r32((l_em_2).wrapping_add(948)))))) == (0i32)) as i32)) != 0) {
                {
                    l_i_5 = 0i32;
                    'l1: loop {
                        if !(((((l_i_5) < (rt::f2i(F32(c.m.r32((l_em_2).wrapping_add(952)))))) as i32)) != 0) { break 'l1; }
                        'c2: {
                            { let r_ = rt::fmul(F32(c.m.r32(((l_em_2).wrapping_add(784)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0x40490fda)); let a_ = ((l_em_2).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            { let r_ = rt::fmul(F32(c.m.r32(((l_em_2).wrapping_add(784)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(0x40490fda)); let a_ = ((l_em_2).wrapping_add(80)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            { let r_ = rt::fmul(F32(c.m.r32(((l_em_2).wrapping_add(784)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(0x40490fda)); let a_ = ((l_em_2).wrapping_add(80)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            { let v_ = { let a0_ = F32(c.m.r32(((l_em_2).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); F::EftMath_WrapAngle(c, a0_) }; let a_ = ((l_em_2).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                            { let v_ = { let a0_ = F32(c.m.r32(((l_em_2).wrapping_add(80)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); F::EftMath_WrapAngle(c, a0_) }; let a_ = ((l_em_2).wrapping_add(80)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                            { let v_ = { let a0_ = F32(c.m.r32(((l_em_2).wrapping_add(80)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); F::EftMath_WrapAngle(c, a0_) }; let a_ = ((l_em_2).wrapping_add(80)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                            { let _ = F::EftQuad_Emit(c, l_em_2); };
                        }
                        { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
                    }
                }
            }
        }
        F::EftQuad_StepAll(c, l_em_2);
        if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_em_2).wrapping_add(96)))) as i32)) != 0) {
            { let r_ = F32(0x3f800000); let a_ = (l_em_2).wrapping_add(96); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::fle(F32(c.m.r32((l_em_2).wrapping_add(96))), F32(0x00000000)) as i32)) != 0) {
                { let v_ = F32(0x00000000); let a_ = (l_em_2).wrapping_add(96); c.m.w32(a_, (v_).0); };
                if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_em_2).wrapping_add(112)))) as i32)) != 0) {
                    { let r_ = 8i32; let a_ = (l_em_2).wrapping_add(964); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                } else {
                    { let r_ = 2i32; let a_ = (l_em_2).wrapping_add(964); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
        }
        if (((((c.m.r32((l_em_2).wrapping_add(964)) as i32)) & (4i32))) != 0) {
            { let r_ = F32(0x3f800000); let a_ = (l_em_2).wrapping_add(108); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::flt(F32(c.m.r32((l_em_2).wrapping_add(108))), F32(0x00000000)) as i32)) != 0) {
                { let r_ = (!(4i32)); let a_ = (l_em_2).wrapping_add(964); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                if ((((!(((((c.m.r32((l_em_2).wrapping_add(964)) as i32)) & (8i32))) != 0)) as i32)) != 0) {
                    { let r_ = 2i32; let a_ = (l_em_2).wrapping_add(964); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
        }
        { let r_ = F32(0x3f800000); let a_ = (l_em_2).wrapping_add(100); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if (((((((((c.m.r32((l_em_2).wrapping_add(968))) == (((0i32) as u32))) as i32)) != 0) && (((((c.m.r32((l_em_2).wrapping_add(964)) as i32)) & (2i32))) != 0)) as i32)) != 0) {
            { let r_ = 16i32; let a_ = (l_em_2).wrapping_add(964); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if (((((c.m.r32((l_em_2).wrapping_add(964)) as i32)) & (16i32))) != 0) {
            F::BtlTask_SetDead(c, p_task_1);
        }
        if (((((c.m.r32((l_em_2).wrapping_add(964)) as i32)) & (8192i32))) != 0) {
            F::EftQuad_CalcKeyDeltas(c, p_task_1);
            if (((rt::fge(F32(c.m.r32((l_em_2).wrapping_add(100))), F32(c.m.r32((l_em_2).wrapping_add(660)))) as i32)) != 0) {
                F::EftQuad_SetLastKey(c, l_em_2);
                { let v_ = rt::i2f((c.m.r8((l_def_3).wrapping_add(573))) as i32); let a_ = (l_em_2).wrapping_add(952); c.m.w32(a_, (v_).0); };
                { let v_ = rt::i2f((c.m.r8((l_def_3).wrapping_add(570))) as i32); let a_ = (l_em_2).wrapping_add(948); c.m.w32(a_, (v_).0); };
                { let r_ = (!(8192i32)); let a_ = (l_em_2).wrapping_add(964); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
    }
    if ((((!(((((c.m.r32((l_em_2).wrapping_add(964)) as i32)) & (32i32))) != 0)) as i32)) != 0) {
        if ((((!(((((c.m.r32((l_em_2).wrapping_add(964)) as i32)) & (16i32))) != 0)) as i32)) != 0) {
            { let a1_ = c.m.r32((l_em_2).wrapping_add(8)); F::EftQuad_LoadTex(c, l_em_2, a1_) };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_em_2: u32 = 0;
    let mut l_def_3: u32 = 0;
    let r_ = 'f: {
    l_em_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_def_3 = c.m.r32(l_em_2);
    if (((((c.m.r32((l_em_2).wrapping_add(8))) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (({ let a0_ = ((c.m.r8((l_em_2).wrapping_add(957))) as i32); let a1_ = ((c.m.r8((l_em_2).wrapping_add(960))) as i32); F::BtlScene_IsEffectHidden(c, a0_, a1_) }) != 0) {
        break 'f;
    }
    if (((((((((c.m.r32((l_em_2).wrapping_add(964)) as i32)) & (128i32))) != 0) && ((((!(({ let a0_ = ((c.m.r8((l_em_2).wrapping_add(957))) as i32); F::BtlScene_IsCharInView(c, a0_) }) != 0)) as i32)) != 0)) as i32)) != 0) {
        break 'f;
    }
    F::Vu0Cur_Push(c);
    { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
    if (((({ let a_ = (c.m.r32(l_em_2)).wrapping_add(564); (c.m.r32(a_) as i32) }) & (32i32))) != 0) {
        { let a1_ = ((c.m.r8((l_def_3).wrapping_add(574))) as i32); F::EftQuad_DrawAllFacing(c, l_em_2, a1_) };
    } else {
        { let a1_ = ((c.m.r8((l_def_3).wrapping_add(574))) as i32); F::EftQuad_DrawAllSprite(c, l_em_2, a1_) };
    }
    F::Vu0Cur_Pop(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_em_2: u32 = 0;
    let mut l_p_3: u32 = 0;
    let r_ = 'f: {
    l_em_2 = c.m.r32((p_task_1).wrapping_add(56));
    {
        l_p_3 = c.m.r32((l_em_2).wrapping_add(968));
        'l1: loop {
            if !(((((l_p_3) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = 0i32; let a_ = (l_p_3).wrapping_add(356); c.m.w32(a_, (v_) as u32); };
                F::EftQuad_ListRemove(c, (l_em_2).wrapping_add(968), (l_em_2).wrapping_add(972), l_p_3);
            }
            l_p_3 = c.m.r32((l_p_3).wrapping_add(364));
        }
    }
    { let v_ = 0i32; let a_ = (l_em_2).wrapping_add(964); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_InitQuad(c: &mut Ctx, mut p_p_1: u32, mut p_em_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_InitQuad", &PROBE_N, &[(p_p_1) as u32, (p_em_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let mut l_def_3: u32 = 0;
    let mut l_x_4: F32 = F32(0);
    let mut l_y_5: F32 = F32(0);
    let mut l_a_6: F32 = F32(0);
    let r_ = 'f: {
        l_def_3 = c.m.r32(p_em_2);
        l_x_4 = { let a0_ = F32(c.m.r32((((p_em_2).wrapping_add(724)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32((((p_em_2).wrapping_add(724)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((p_em_2).wrapping_add(724)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) };
        l_y_5 = { let a0_ = F32(c.m.r32((((p_em_2).wrapping_add(724)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32((((p_em_2).wrapping_add(724)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((p_em_2).wrapping_add(724)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) };
        { let a3_ = { let a0_ = F32(c.m.r32((((p_em_2).wrapping_add(724)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32((((p_em_2).wrapping_add(724)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((p_em_2).wrapping_add(724)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }; F::Vec4_Set(c, p_p_1, l_x_4, l_y_5, a3_, F32(0x3f800000)) };
        F::Vec4_Copy(c, (p_p_1).wrapping_add(16), (p_em_2).wrapping_add(48));
        F::Mtx_StoreIdentity(c, fp);
        F::Vec4_Set(c, fp.wrapping_add(64), F32(0x00000000), F32(0x00000000), F32(0x3f800000), F32(0x3f800000));
        l_a_6 = rt::fmul({ let a0_ = F32(c.m.r32((p_em_2).wrapping_add(716))); let a1_ = rt::fadd(F32(c.m.r32((p_em_2).wrapping_add(716))), F32(c.m.r32((p_em_2).wrapping_add(720)))); F::Rand_FloatRange(c, a0_, a1_) }, F32(0x40490fda));
        { let v_ = F::Port_sinf(c, l_a_6); let a_ = (p_p_1).wrapping_add(32); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((p_p_1).wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F::Port_cosf(c, l_a_6); let a_ = ((p_p_1).wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((p_p_1).wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let a2_ = { let a0_ = rt::fmul(F::Rand_FloatRange(c, F32(0x00000000), F32(0x3f800000)), F32(0x40c90fda)); F::EftMath_WrapAngle(c, a0_) }; F::Mtx_RotateZ(c, fp, fp, a2_) };
        F::Mtx_MulVec4(c, (p_p_1).wrapping_add(32), fp, (p_p_1).wrapping_add(32));
        F::Vec3_Sub(c, (p_p_1).wrapping_add(48), fp.wrapping_add(64), (p_p_1).wrapping_add(32));
        F::Vec3_Normalize(c, (p_p_1).wrapping_add(48), (p_p_1).wrapping_add(48));
        { let a2_ = F32(c.m.r32((p_em_2).wrapping_add(756))); F::Vec3_Scale(c, (p_p_1).wrapping_add(48), (p_p_1).wrapping_add(48), a2_) };
        { let v_ = rt::fmul({ let a0_ = F32(c.m.r32(((p_em_2).wrapping_add(808)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((p_em_2).wrapping_add(808)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_em_2).wrapping_add(820)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }, F32(0x40490fda)); let a_ = ((p_p_1).wrapping_add(224)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul({ let a0_ = F32(c.m.r32(((p_em_2).wrapping_add(808)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((p_em_2).wrapping_add(808)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_em_2).wrapping_add(820)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }, F32(0x40490fda)); let a_ = ((p_p_1).wrapping_add(224)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul({ let a0_ = F32(c.m.r32(((p_em_2).wrapping_add(808)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((p_em_2).wrapping_add(808)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_em_2).wrapping_add(820)).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }, F32(0x40490fda)); let a_ = ((p_p_1).wrapping_add(224)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul({ let a0_ = F32(c.m.r32(((p_em_2).wrapping_add(760)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((p_em_2).wrapping_add(760)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_em_2).wrapping_add(772)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }, F32(0x40c90fda)); let a_ = (p_p_1).wrapping_add(112); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul({ let a0_ = F32(c.m.r32(((p_em_2).wrapping_add(760)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((p_em_2).wrapping_add(760)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_em_2).wrapping_add(772)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }, F32(0x40c90fda)); let a_ = ((p_p_1).wrapping_add(112)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul({ let a0_ = F32(c.m.r32(((p_em_2).wrapping_add(760)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((p_em_2).wrapping_add(760)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_em_2).wrapping_add(772)).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }, F32(0x40c90fda)); let a_ = ((p_p_1).wrapping_add(112)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((p_p_1).wrapping_add(112)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let r_ = F32(c.m.r32(((p_em_2).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_p_1).wrapping_add(112); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32(((p_em_2).wrapping_add(80)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((p_p_1).wrapping_add(112)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32(((p_em_2).wrapping_add(80)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = ((p_p_1).wrapping_add(112)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if (((((c.m.r32((l_def_3).wrapping_add(564)) as i32)) & (256i32))) != 0) {
            if ((((!((((F::Port_Rand(c)) & (1i32))) != 0)) as i32)) != 0) {
                { let v_ = rt::fneg(F32(c.m.r32((p_p_1).wrapping_add(112)))); let a_ = (p_p_1).wrapping_add(112); c.m.w32(a_, (v_).0); };
            }
        }
        if (((((c.m.r32((l_def_3).wrapping_add(564)) as i32)) & (512i32))) != 0) {
            if ((((!((((F::Port_Rand(c)) & (1i32))) != 0)) as i32)) != 0) {
                { let v_ = rt::fneg(F32(c.m.r32(((p_p_1).wrapping_add(112)).wrapping_add(4)))); let a_ = ((p_p_1).wrapping_add(112)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            }
        }
        if (((((c.m.r32((l_def_3).wrapping_add(564)) as i32)) & (1024i32))) != 0) {
            if ((((!((((F::Port_Rand(c)) & (1i32))) != 0)) as i32)) != 0) {
                { let v_ = rt::fneg(F32(c.m.r32(((p_p_1).wrapping_add(112)).wrapping_add(8)))); let a_ = ((p_p_1).wrapping_add(112)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            }
        }
        if (((((c.m.r32((l_def_3).wrapping_add(564)) as i32)) & (2i32))) != 0) {
            if ((((!((((F::Port_Rand(c)) & (1i32))) != 0)) as i32)) != 0) {
                { let v_ = rt::fneg(F32(c.m.r32(((p_p_1).wrapping_add(224)).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let a_ = ((p_p_1).wrapping_add(224)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            }
        }
        if (((((c.m.r32((l_def_3).wrapping_add(564)) as i32)) & (1i32))) != 0) {
            { let v_ = rt::fneg(F32(c.m.r32(((p_p_1).wrapping_add(224)).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let a_ = ((p_p_1).wrapping_add(224)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        }
        { let v_ = rt::fmul({ let a0_ = F32(c.m.r32((p_em_2).wrapping_add(748))); let a1_ = rt::fadd(F32(c.m.r32((p_em_2).wrapping_add(748))), F32(c.m.r32((p_em_2).wrapping_add(752)))); F::Rand_FloatRange(c, a0_, a1_) }, F32(0x41f00000)); let a_ = (p_p_1).wrapping_add(336); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_p_1).wrapping_add(340); c.m.w32(a_, (v_).0); };
        if (((rt::fle(F32(c.m.r32((p_p_1).wrapping_add(336))), F32(c.m.r32((p_p_1).wrapping_add(340)))) as i32)) != 0) {
            break 'f 0i32;
        }
        { let v_ = ((0i32) as u32); let a_ = (p_p_1).wrapping_add(364); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (p_p_1).wrapping_add(368); c.m.w32(a_, v_); };
        if (((((((c.m.r8((p_em_2).wrapping_add(958))) as i32)) == (((c.m.r8((p_em_2).wrapping_add(959))) as i32))) as i32)) != 0) {
            { let v_ = c.m.r8((p_em_2).wrapping_add(958)); let a_ = (p_p_1).wrapping_add(360); c.m.w8(a_, v_); };
        } else {
            { let v_ = (((((c.m.r8((p_em_2).wrapping_add(958))) as i32)).wrapping_add(((c.m.r8((p_em_2).wrapping_add(959))) as i32))) as u8); let a_ = (p_p_1).wrapping_add(360); c.m.w8(a_, v_); };
        }
        if (((((((((c.m.r32((l_def_3).wrapping_add(564)) as i32)) & (4i32))) != 0) && (((rt::flt(F32(c.m.r32(((p_p_1).wrapping_add(224)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
            { let v_ = F32(0x3f7c0000); let a_ = ((p_p_1).wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3c800000); let a_ = ((p_p_1).wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3c800000); let a_ = ((p_p_1).wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3c800000); let a_ = ((p_p_1).wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f7c0000); let a_ = ((p_p_1).wrapping_add(192)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f7c0000); let a_ = ((p_p_1).wrapping_add(192)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3c800000); let a_ = ((p_p_1).wrapping_add(192)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f7c0000); let a_ = ((p_p_1).wrapping_add(192)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        } else {
            { let v_ = F32(0x3c800000); let a_ = ((p_p_1).wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3c800000); let a_ = ((p_p_1).wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3c800000); let a_ = ((p_p_1).wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f7c0000); let a_ = ((p_p_1).wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f7c0000); let a_ = ((p_p_1).wrapping_add(192)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3c800000); let a_ = ((p_p_1).wrapping_add(192)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f7c0000); let a_ = ((p_p_1).wrapping_add(192)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f7c0000); let a_ = ((p_p_1).wrapping_add(192)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        }
        { let v_ = { let a1_ = rt::i2f((((c.m.r8((p_em_2).wrapping_add(956))) as i32)).wrapping_sub(1i32)); F::Rand_FloatRange(c, F32(0x00000000), a1_) }; let a_ = (p_p_1).wrapping_add(344); c.m.w32(a_, (v_).0); };
        if (((((c.m.r32((l_def_3).wrapping_add(564)) as i32)) & (8i32))) != 0) {
            { let v_ = rt::fmul(F32(c.m.r32((p_em_2).wrapping_add(840))), F32(0x41f00000)); let a_ = (p_p_1).wrapping_add(284); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (p_p_1).wrapping_add(280); c.m.w32(a_, (v_).0); };
            if (((rt::flt(F32(0x00000000), F32(c.m.r32((p_em_2).wrapping_add(840)))) as i32)) != 0) {
                { let v_ = rt::fsub(F32(c.m.r32((p_em_2).wrapping_add(836))), F32(c.m.r32((p_em_2).wrapping_add(832)))); let a_ = (p_p_1).wrapping_add(244); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32((p_em_2).wrapping_add(832))); let a_ = (p_p_1).wrapping_add(276); c.m.w32(a_, (v_).0); };
                { let r_ = 2048i32; let a_ = (p_p_1).wrapping_add(356); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
        if (((((c.m.r32((l_def_3).wrapping_add(564)) as i32)) & (128i32))) != 0) {
            { let v_ = rt::fmul(F32(c.m.r32((p_em_2).wrapping_add(944))), F32(0x41f00000)); let a_ = (p_p_1).wrapping_add(304); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (p_p_1).wrapping_add(300); c.m.w32(a_, (v_).0); };
            if (((rt::flt(F32(c.m.r32((p_p_1).wrapping_add(300))), F32(c.m.r32((p_p_1).wrapping_add(304)))) as i32)) != 0) {
                { let v_ = rt::fsub(F32(c.m.r32(((p_em_2).wrapping_add(932)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_em_2).wrapping_add(920)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((p_p_1).wrapping_add(248)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fsub(F32(c.m.r32(((p_em_2).wrapping_add(932)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_em_2).wrapping_add(920)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = ((p_p_1).wrapping_add(248)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fsub(F32(c.m.r32(((p_em_2).wrapping_add(932)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_em_2).wrapping_add(920)).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let a_ = ((p_p_1).wrapping_add(248)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((p_em_2).wrapping_add(920)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_p_1).wrapping_add(288)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((p_em_2).wrapping_add(920)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((p_p_1).wrapping_add(288)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((p_em_2).wrapping_add(920)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = ((p_p_1).wrapping_add(288)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                if (((rt::fne(F32(c.m.r32(((p_p_1).wrapping_add(248)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0x00000000)) as i32)) != 0) {
                    { let r_ = 65536i32; let a_ = (p_p_1).wrapping_add(356); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
                if (((rt::fne(F32(c.m.r32(((p_p_1).wrapping_add(248)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(0x00000000)) as i32)) != 0) {
                    { let r_ = 131072i32; let a_ = (p_p_1).wrapping_add(356); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
                if (((rt::fne(F32(c.m.r32(((p_p_1).wrapping_add(248)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(0x00000000)) as i32)) != 0) {
                    { let r_ = 262144i32; let a_ = (p_p_1).wrapping_add(356); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
        } else {
            { let v_ = F32(0x3f800000); let a_ = ((p_p_1).wrapping_add(288)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f800000); let a_ = ((p_p_1).wrapping_add(288)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f800000); let a_ = ((p_p_1).wrapping_add(288)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        }
        { let v_ = { let a0_ = F32(c.m.r32(((p_em_2).wrapping_add(668)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((p_em_2).wrapping_add(668)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_em_2).wrapping_add(680)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (fp.wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((p_em_2).wrapping_add(668)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((p_em_2).wrapping_add(668)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_em_2).wrapping_add(680)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (fp.wrapping_add(80)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((p_em_2).wrapping_add(668)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((p_em_2).wrapping_add(668)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_em_2).wrapping_add(680)).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (fp.wrapping_add(80)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_p_1).wrapping_add(316); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32((fp.wrapping_add(80)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(336))), F32(c.m.r32((l_def_3).wrapping_add(544))))); let a_ = (p_p_1).wrapping_add(320); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32((fp.wrapping_add(80)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(80)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(336))), rt::fsub(F32(0x3f800000), F32(c.m.r32((l_def_3).wrapping_add(544)))))); let a_ = (p_p_1).wrapping_add(324); c.m.w32(a_, (v_).0); };
        { let v_ = { let v_ = { let a0_ = F32(c.m.r32((l_def_3).wrapping_add(548))); let a1_ = rt::fadd(F32(c.m.r32((l_def_3).wrapping_add(548))), F32(c.m.r32((l_def_3).wrapping_add(552)))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (p_p_1).wrapping_add(308); c.m.w32(a_, (v_).0); v_ }; let a_ = (p_p_1).wrapping_add(312); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((p_em_2).wrapping_add(692)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((p_em_2).wrapping_add(692)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_em_2).wrapping_add(704)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (fp.wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((p_em_2).wrapping_add(692)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((p_em_2).wrapping_add(692)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_em_2).wrapping_add(704)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (fp.wrapping_add(80)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((p_em_2).wrapping_add(692)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((p_em_2).wrapping_add(692)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_em_2).wrapping_add(704)).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (fp.wrapping_add(80)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_p_1).wrapping_add(264); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32((fp.wrapping_add(80)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(336))), F32(c.m.r32((l_def_3).wrapping_add(544))))); let a_ = (p_p_1).wrapping_add(268); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32((fp.wrapping_add(80)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(80)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(336))), rt::fsub(F32(0x3f800000), F32(c.m.r32((l_def_3).wrapping_add(544)))))); let a_ = (p_p_1).wrapping_add(272); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((l_def_3).wrapping_add(556))); let a1_ = rt::fadd(F32(c.m.r32((l_def_3).wrapping_add(556))), F32(c.m.r32((l_def_3).wrapping_add(560)))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (p_p_1).wrapping_add(260); c.m.w32(a_, (v_).0); };
        { let _ = F::memset(c, fp.wrapping_add(96), 0i32, 16u32); };
        's1: {
            let sv7: i32 = ((c.m.r8((l_def_3).wrapping_add(578))) as i32);
            'k3: {
            'k2: {
                match sv7 { 2i32 => break 'k2, 1i32 => break 'k3, _ => break 's1, }
            }
            if ((((F::Port_Rand(c)) & (1i32))) != 0) {
                break 's1;
            }
            }
            { let r_ = 64i32; let a_ = (p_p_1).wrapping_add(356); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            break 's1;
        }
        { let v_ = F32(c.m.r32((p_em_2).wrapping_add(912))); let a_ = (p_p_1).wrapping_add(348); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((p_em_2).wrapping_add(916))); let a_ = (p_p_1).wrapping_add(352); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_p_1).wrapping_add(332); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(336))), rt::fsub(F32(c.m.r32((p_p_1).wrapping_add(352))), F32(c.m.r32((p_p_1).wrapping_add(348))))); let a_ = (p_p_1).wrapping_add(328); c.m.w32(a_, (v_).0); };
        if (((((((rt::fne(F32(c.m.r32((p_p_1).wrapping_add(328))), F32(c.m.r32((p_p_1).wrapping_add(336)))) as i32)) != 0) && (((rt::flt(F32(c.m.r32((p_p_1).wrapping_add(332))), F32(c.m.r32((p_p_1).wrapping_add(328)))) as i32)) != 0)) as i32)) != 0) {
            { let r_ = 32768i32; let a_ = (p_p_1).wrapping_add(356); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        { let v_ = { let a0_ = F32(c.m.r32(((p_em_2).wrapping_add(848)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((p_em_2).wrapping_add(848)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_em_2).wrapping_add(864)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (p_p_1).wrapping_add(64); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((p_em_2).wrapping_add(848)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((p_em_2).wrapping_add(848)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_em_2).wrapping_add(864)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = ((p_p_1).wrapping_add(64)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((p_em_2).wrapping_add(848)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((p_em_2).wrapping_add(848)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_em_2).wrapping_add(864)).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = ((p_p_1).wrapping_add(64)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((p_p_1).wrapping_add(64)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((p_em_2).wrapping_add(880)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((p_em_2).wrapping_add(880)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_em_2).wrapping_add(896)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = fp.wrapping_add(96); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((p_em_2).wrapping_add(880)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((p_em_2).wrapping_add(880)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_em_2).wrapping_add(896)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (fp.wrapping_add(96)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((p_em_2).wrapping_add(880)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((p_em_2).wrapping_add(880)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_em_2).wrapping_add(896)).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (fp.wrapping_add(96)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(96)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let a2_ = F32(c.m.r32((fp.wrapping_add(96)).wrapping_add(12))); F::Vec4_Clamp(c, (p_p_1).wrapping_add(64), (p_p_1).wrapping_add(64), a2_, F32(0x437f0000)) };
        F::Vec4_Clamp(c, fp.wrapping_add(96), fp.wrapping_add(96), F32(0x00000000), F32(0x437f0000));
        F::Vec4_Copy(c, (p_p_1).wrapping_add(96), (p_p_1).wrapping_add(64));
        F::Vec3_Sub(c, (p_p_1).wrapping_add(80), fp.wrapping_add(96), (p_p_1).wrapping_add(64));
        { let v_ = { let a0_ = F32(c.m.r32(((p_em_2).wrapping_add(848)).wrapping_add((((3i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((p_em_2).wrapping_add(848)).wrapping_add((((3i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_em_2).wrapping_add(864)).wrapping_add((((3i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = ((p_p_1).wrapping_add(96)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((p_p_1).wrapping_add(96)).wrapping_add(12))); F::eft_x_c__EftQuad_Clamp(c, a0_, F32(0x00000000), F32(0x437f0000)) }; let a_ = ((p_p_1).wrapping_add(96)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(336))), F32(c.m.r32((l_def_3).wrapping_add(536)))); let a_ = (p_p_1).wrapping_add(236); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(336))), rt::fsub(F32(0x3f800000), F32(c.m.r32((l_def_3).wrapping_add(540))))); let a_ = (p_p_1).wrapping_add(240); c.m.w32(a_, (v_).0); };
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_InitQuad", &PROBE_N, &[], true); }
    r_
}
