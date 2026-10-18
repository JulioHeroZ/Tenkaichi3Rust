#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EftOrbTailMgr_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTailMgr_Update", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    { let v_ = ((0i32) as u32); let a_ = (({ let a_ = c.m.r32(sym::gEftOrbTail); c.m.r32(a_) }).wrapping_add(8)).wrapping_add(260); c.m.w32(a_, v_); };
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (3i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = ((0i32) as u32); let a_ = ((({ let a_ = c.m.r32(sym::gEftOrbTail); c.m.r32(a_) }).wrapping_add(272)).wrapping_add((((l_i_1) as u32)).wrapping_mul(264))).wrapping_add(260); c.m.w32(a_, v_); };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTailMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftOrbTailMgr_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTailMgr_Reset", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTailMgr_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftOrbTailMgr_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTailMgr_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = { let a_ = c.m.r32(sym::gEftOrbTail); c.m.r32(a_) }; F::BtlPool_Free(c, a0_, a1_) };
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftOrbTail); F::BtlPool_Free(c, a0_, a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftOrbTail; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTailMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftOrbTail_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let mut l_prm_4: u32 = 0;
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    l_prm_4 = c.m.r32(p_arg_2);
    { let _ = F::memset(c, l_w_3, 0i32, 480u32); };
    { let s_ = p_arg_2; let d_ = (l_w_3).wrapping_add(48); rt::memcpy(c, d_, s_, 48); };
    's1: {
        let sv5: i32 = ((c.m.r8((p_arg_2).wrapping_add(32))) as i32);
        match sv5 {
            7i32 => {
                { let v_ = ((1i32) as u8); let a_ = ((l_w_3).wrapping_add(48)).wrapping_add(32); c.m.w8(a_, v_); };
                break 's1;
            }
            8i32 | 9i32 | 10i32 => {
                { let v_ = ((2i32) as u8); let a_ = ((l_w_3).wrapping_add(48)).wrapping_add(32); c.m.w8(a_, v_); };
                break 's1;
            }
            _ => {
            }
        }
    }
    { let a1_ = ({ let a_ = c.m.r32(sym::gEftOrbTail); c.m.r32(a_) }).wrapping_add(8); let a3_ = ((c.m.r8(((l_w_3).wrapping_add(48)).wrapping_add(32))) as i32); F::EftOrbTail_SetTex(c, l_w_3, a1_, 0i32, a3_) };
    F::EftOrbTail_InitFrames(c, l_w_3);
    { let v_ = rt::fdiv({ let a0_ = ((c.m.r8((p_arg_2).wrapping_add(33))) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x419acccc)); let a_ = (l_w_3).wrapping_add(160); c.m.w32(a_, (v_).0); };
    { let v_ = (({ let a0_ = ((c.m.r8((l_prm_4).wrapping_add(116))) as i32); let a1_ = (((c.m.r8((l_prm_4).wrapping_add(116))) as i32)).wrapping_add(((c.m.r8((l_prm_4).wrapping_add(117))) as i32)); F::Rand_IntRange(c, a0_, a1_) }) as u8); let a_ = (l_w_3).wrapping_add(433); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2i(rt::fmul(rt::i2f(((c.m.r8((l_w_3).wrapping_add(433))) as i32)), F32(c.m.r32((l_w_3).wrapping_add(160)))))) as u8); let a_ = (l_w_3).wrapping_add(433); c.m.w8(a_, v_); };
    { let v_ = rt::fmul(F32(c.m.r32((l_prm_4).wrapping_add(84))), rt::i2f(((c.m.r8((l_w_3).wrapping_add(433))) as i32))); let a_ = (l_w_3).wrapping_add(144); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(rt::fadd(rt::fadd(rt::fadd(F32(c.m.r32((l_prm_4).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_prm_4).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), rt::fadd(F32(c.m.r32((l_prm_4).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_prm_4).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))), rt::fadd(F32(c.m.r32((l_prm_4).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_prm_4).wrapping_add(12)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))), F32(0x3f000000)); let a_ = (l_w_3).wrapping_add(152); c.m.w32(a_, (v_).0); };
    { let a0_ = ((c.m.r8((p_arg_2).wrapping_add(33))) as i32); F::BtlCharApi_GetDir(c, a0_, (l_w_3).wrapping_add(128)) };
    { let r_ = ((1i32) | (16i32)); let a_ = (l_w_3).wrapping_add(436); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftOrbTail_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_arg_3: u32 = 0;
    let mut l_prm_4: u32 = 0;
    let mut l_speed_5: F32 = F32(0);
    let mut l_i_6: i32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_arg_3 = (l_w_2).wrapping_add(48);
    l_prm_4 = c.m.r32(l_arg_3);
    l_speed_5 = F32(0x00000000);
    if ((((!(({ let a0_ = ((c.m.r8((l_arg_3).wrapping_add(33))) as i32); F::BtlScene_IsEffectStopped(c, a0_, 2i32) }) != 0)) as i32)) != 0) {
        F::Vec3_Sub(c, (l_w_2).wrapping_add(128), (l_w_2).wrapping_add(96), (l_w_2).wrapping_add(112));
        if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(148))), F32(c.m.r32((l_w_2).wrapping_add(144)))) as i32)) != 0) {
            l_speed_5 = (if (((rt::flt(rt::fdiv(F32(c.m.r32((l_w_2).wrapping_add(148))), F32(c.m.r32((l_w_2).wrapping_add(144)))), F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(0x3f800000), rt::fdiv(F32(c.m.r32((l_w_2).wrapping_add(148))), F32(c.m.r32((l_w_2).wrapping_add(144))))) as i32)) != 0) { F32(0x3f800000) } else { rt::fdiv(F32(c.m.r32((l_w_2).wrapping_add(148))), F32(c.m.r32((l_w_2).wrapping_add(144)))) }) });
            { let v_ = l_speed_5; let a_ = (l_w_2).wrapping_add(156); c.m.w32(a_, (v_).0); };
            { let r_ = F32(c.m.r32((l_w_2).wrapping_add(152))); let a_ = (l_w_2).wrapping_add(148); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
        if (({ let a0_ = ((c.m.r8((l_arg_3).wrapping_add(33))) as i32); F::BtlCharApi_IsInRushSequence(c, a0_) }) != 0) {
            l_speed_5 = F::Vec3_Length(c, (l_w_2).wrapping_add(128));
        } else {
            l_speed_5 = { let a0_ = ((c.m.r8((l_arg_3).wrapping_add(33))) as i32); F::BtlCharApi_GetSpeed(c, a0_) };
            { let r_ = { let a0_ = { let a0_ = ((c.m.r8((l_arg_3).wrapping_add(33))) as i32); F::BtlCharApi_GetFallSpeed(c, a0_) }; rt::ffabs(a0_) }; let n_ = rt::fadd(l_speed_5, r_); l_speed_5 = n_; };
        }
        if (((rt::fgt(l_speed_5, F32(0x00000000)) as i32)) != 0) {
            { let r_ = 64i32; let a_ = (l_w_2).wrapping_add(436); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        } else {
            { let r_ = (!(64i32)); let a_ = (l_w_2).wrapping_add(436); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        }
        F::Vec3_Normalize(c, (l_w_2).wrapping_add(128), (l_w_2).wrapping_add(128));
        if ((((!(((((c.m.r32((l_w_2).wrapping_add(436)) as i32)) & (((2i32) | (4i32))))) != 0)) as i32)) != 0) {
            if (((((c.m.r32((l_w_2).wrapping_add(436)) as i32)) & (64i32))) != 0) {
                if (((((((c.m.r8((l_w_2).wrapping_add(432))) as i32)) < (((c.m.r8((l_w_2).wrapping_add(433))) as i32))) as i32)) != 0) {
                    {
                        l_i_6 = 0i32;
                        'l1: loop {
                            if !(((((l_i_6) < (((c.m.r8((l_w_2).wrapping_add(433))) as i32))) as i32)) != 0) { break 'l1; }
                            'c2: {
                                F::EftOrbTail_AddNode(c, l_w_2);
                            }
                            { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                        }
                    }
                }
                if (((((c.m.r32((l_w_2).wrapping_add(436)) as i32)) & (32i32))) != 0) {
                    if (((((rt::rem_i32(rt::f2i(F32(c.m.r32((l_w_2).wrapping_add(168)))), ((c.m.r8((l_prm_4).wrapping_add(118))) as i32))) == (0i32)) as i32)) != 0) {
                        {
                            l_i_6 = 0i32;
                            'l3: loop {
                                if !(((((l_i_6) < (((c.m.r8((l_prm_4).wrapping_add(119))) as i32))) as i32)) != 0) { break 'l3; }
                                'c4: {
                                    F::EftOrbTail_AddStreak(c, l_w_2);
                                }
                                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                            }
                        }
                    }
                }
            }
            if (((((c.m.r32((l_w_2).wrapping_add(436)) as i32)) & (128i32))) != 0) {
                F::EftOrbTail_StartBurst(c, l_w_2);
                { let r_ = (!(128i32)); let a_ = (l_w_2).wrapping_add(436); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            }
        } else {
            F::EftOrbTail_StopBurst(c, l_w_2);
        }
        F::EftOrbTail_UpdateNodes(c, l_w_2);
        F::EftOrbTail_UpdateStreaks(c, l_w_2);
        F::EftOrbTail_PollBurst(c, l_w_2);
        { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(168); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
    if (((((((((c.m.r32((l_w_2).wrapping_add(436)) as i32)) & (2i32))) != 0) && (((((c.m.r32((l_w_2).wrapping_add(448))) == (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        { let r_ = 8i32; let a_ = (l_w_2).wrapping_add(436); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if (((((c.m.r32((l_w_2).wrapping_add(436)) as i32)) & (((4i32) | (8i32))))) != 0) {
        F::BtlTask_SetDead(c, p_task_1);
        if (((((c.m.r32((l_w_2).wrapping_add(436)) as i32)) & (((4i32) | (8i32))))) != 0) {
            break 'f;
        }
    }
    F::EftOrbTail_UpdateTex(c, l_w_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftOrbTail_PostUpdate(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_PostUpdate", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftOrbTail_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    F::Vu0Cur_Push(c);
    { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
    F::EftOrbTail_DrawStreaks(c, l_w_2);
    F::Vu0Cur_Pop(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftOrbTail_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftOrbTail_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_n_3: u32 = 0;
    let mut l_p_4: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    {
        l_n_3 = c.m.r32((l_w_2).wrapping_add(440));
        'l1: loop {
            if !(((((l_n_3) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((c.m.r32((l_w_2).wrapping_add(440))) != (((0i32) as u32))) as i32)) != 0) {
                    if (((((c.m.r32((l_n_3).wrapping_add(100))) == (((0i32) as u32))) as i32)) != 0) {
                        if (((((c.m.r32((l_n_3).wrapping_add(96))) == (((0i32) as u32))) as i32)) != 0) {
                            { let v_ = ((0i32) as u32); let a_ = (l_w_2).wrapping_add(440); c.m.w32(a_, v_); };
                            { let v_ = ((0i32) as u32); let a_ = (l_w_2).wrapping_add(444); c.m.w32(a_, v_); };
                        } else {
                            { let v_ = c.m.r32((l_n_3).wrapping_add(96)); let a_ = (l_w_2).wrapping_add(440); c.m.w32(a_, v_); };
                            { let v_ = ((0i32) as u32); let a_ = (c.m.r32((l_w_2).wrapping_add(440))).wrapping_add(100); c.m.w32(a_, v_); };
                        }
                    } else {
                        if (((((c.m.r32((l_n_3).wrapping_add(96))) == (((0i32) as u32))) as i32)) != 0) {
                            { let v_ = c.m.r32((l_n_3).wrapping_add(100)); let a_ = (l_w_2).wrapping_add(444); c.m.w32(a_, v_); };
                            { let v_ = ((0i32) as u32); let a_ = (c.m.r32((l_w_2).wrapping_add(444))).wrapping_add(96); c.m.w32(a_, v_); };
                        } else {
                            { let v_ = c.m.r32((l_n_3).wrapping_add(100)); let a_ = (c.m.r32((l_n_3).wrapping_add(96))).wrapping_add(100); c.m.w32(a_, v_); };
                            { let v_ = c.m.r32((l_n_3).wrapping_add(96)); let a_ = (c.m.r32((l_n_3).wrapping_add(100))).wrapping_add(96); c.m.w32(a_, v_); };
                        }
                    }
                }
                { let v_ = 0i32; let a_ = (l_n_3).wrapping_add(92); c.m.w32(a_, (v_) as u32); };
            }
            l_n_3 = c.m.r32((l_n_3).wrapping_add(96));
        }
    }
    {
        l_p_4 = c.m.r32((l_w_2).wrapping_add(448));
        'l3: loop {
            if !(((((l_p_4) != (((0i32) as u32))) as i32)) != 0) { break 'l3; }
            'c4: {
                if (((((c.m.r32((l_w_2).wrapping_add(448))) != (((0i32) as u32))) as i32)) != 0) {
                    if (((((c.m.r32((l_p_4).wrapping_add(144))) == (((0i32) as u32))) as i32)) != 0) {
                        if (((((c.m.r32((l_p_4).wrapping_add(140))) == (((0i32) as u32))) as i32)) != 0) {
                            { let v_ = ((0i32) as u32); let a_ = (l_w_2).wrapping_add(448); c.m.w32(a_, v_); };
                            { let v_ = ((0i32) as u32); let a_ = (l_w_2).wrapping_add(452); c.m.w32(a_, v_); };
                        } else {
                            { let v_ = c.m.r32((l_p_4).wrapping_add(140)); let a_ = (l_w_2).wrapping_add(448); c.m.w32(a_, v_); };
                            { let v_ = ((0i32) as u32); let a_ = (c.m.r32((l_w_2).wrapping_add(448))).wrapping_add(144); c.m.w32(a_, v_); };
                        }
                    } else {
                        if (((((c.m.r32((l_p_4).wrapping_add(140))) == (((0i32) as u32))) as i32)) != 0) {
                            { let v_ = c.m.r32((l_p_4).wrapping_add(144)); let a_ = (l_w_2).wrapping_add(452); c.m.w32(a_, v_); };
                            { let v_ = ((0i32) as u32); let a_ = (c.m.r32((l_w_2).wrapping_add(452))).wrapping_add(140); c.m.w32(a_, v_); };
                        } else {
                            { let v_ = c.m.r32((l_p_4).wrapping_add(144)); let a_ = (c.m.r32((l_p_4).wrapping_add(140))).wrapping_add(144); c.m.w32(a_, v_); };
                            { let v_ = c.m.r32((l_p_4).wrapping_add(140)); let a_ = (c.m.r32((l_p_4).wrapping_add(144))).wrapping_add(140); c.m.w32(a_, v_); };
                        }
                    }
                }
                { let v_ = 0i32; let a_ = (l_p_4).wrapping_add(136); c.m.w32(a_, (v_) as u32); };
            }
            l_p_4 = c.m.r32((l_p_4).wrapping_add(140));
        }
    }
    F::EftOrbTail_StopBurst(c, l_w_2);
    { let v_ = 0i32; let a_ = (l_w_2).wrapping_add(436); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftOrbTail_AddNode(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_AddNode", &PROBE_N, &[(p_w_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_prm_2: u32 = 0;
    let mut l_n_3: u32 = 0;
    let r_ = 'f: {
        l_prm_2 = c.m.r32((p_w_1).wrapping_add(48));
        rt::memset(c, fp, 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(12), (v_).0); }
        if (((((c.m.r32((p_w_1).wrapping_add(444))) != (((0i32) as u32))) as i32)) != 0) {
            { let a2_ = (c.m.r32((p_w_1).wrapping_add(444))).wrapping_add(64); let a3_ = rt::fneg(rt::fmul(F32(c.m.r32((l_prm_2).wrapping_add(84))), F32(c.m.r32((p_w_1).wrapping_add(160))))); F::Vec3_ScaleAdd(c, fp, (p_w_1).wrapping_add(128), a2_, a3_) };
        } else {
            { let a3_ = rt::fmul(F32(c.m.r32((l_prm_2).wrapping_add(60))), F32(c.m.r32((p_w_1).wrapping_add(160)))); F::Vec3_ScaleAdd(c, fp, (p_w_1).wrapping_add(128), (p_w_1).wrapping_add(96), a3_) };
        }
        l_n_3 = F::EftOrbTail_AllocNode(c);
        if (((((l_n_3) != (((0i32) as u32))) as i32)) != 0) {
            F::EftOrbTail_SetNodePos(c, l_n_3, p_w_1, fp);
            { let v_ = ((0i32) as u32); let a_ = (l_n_3).wrapping_add(96); c.m.w32(a_, v_); };
            { let v_ = ((0i32) as u32); let a_ = (l_n_3).wrapping_add(100); c.m.w32(a_, v_); };
            { let v_ = 1i32; let a_ = (l_n_3).wrapping_add(92); c.m.w32(a_, (v_) as u32); };
            if (((((c.m.r32((p_w_1).wrapping_add(440))) == (((0i32) as u32))) as i32)) != 0) {
                { let v_ = l_n_3; let a_ = (p_w_1).wrapping_add(440); c.m.w32(a_, v_); };
                { let v_ = l_n_3; let a_ = (p_w_1).wrapping_add(444); c.m.w32(a_, v_); };
            } else {
                { let v_ = c.m.r32((p_w_1).wrapping_add(444)); let a_ = (l_n_3).wrapping_add(100); c.m.w32(a_, v_); };
                { let v_ = l_n_3; let a_ = (c.m.r32((p_w_1).wrapping_add(444))).wrapping_add(96); c.m.w32(a_, v_); };
                { let v_ = l_n_3; let a_ = (p_w_1).wrapping_add(444); c.m.w32(a_, v_); };
            }
            { let a_ = (p_w_1).wrapping_add(432); let o_ = c.m.r8(a_); let n_ = (o_).wrapping_add(1); c.m.w8(a_, n_); };
        }
        if (((((((((c.m.r32((p_w_1).wrapping_add(440))) != (((0i32) as u32))) as i32)) != 0) && ((((({ let a_ = (c.m.r32((p_w_1).wrapping_add(440))).wrapping_add(96); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
            { let r_ = 32i32; let a_ = (p_w_1).wrapping_add(436); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_AddNode", &PROBE_N, &[], true); }
    r_
}

pub fn EftOrbTail_SetNodePos(c: &mut Ctx, mut p_node_1: u32, mut p_w_2: u32, p_pos_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_SetNodePos", &PROBE_N, &[(p_node_1) as u32, (p_w_2) as u32, (p_pos_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    rt::memcpy(c, fp, p_pos_3, 16);
    let r_ = 'f: {
        F::Vec4_Copy(c, (p_node_1).wrapping_add(64), fp);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_SetNodePos", &PROBE_N, &[], true); }
    r_
}

pub fn EftOrbTail_UpdateNodes(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_UpdateNodes", &PROBE_N, &[(p_w_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_prm_2: u32 = 0;
    let mut l_n_3: u32 = 0;
    let mut l_len_4: F32 = F32(0);
    let r_ = 'f: {
        l_prm_2 = c.m.r32((p_w_1).wrapping_add(48));
        rt::memset(c, fp, 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(16), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(12), (v_).0); }
        l_len_4 = F32(0x00000000);
        if (((((c.m.r32((p_w_1).wrapping_add(440))) != (((0i32) as u32))) as i32)) != 0) {
            { let a0_ = (c.m.r32((p_w_1).wrapping_add(440))).wrapping_add(64); let a3_ = rt::fmul(F32(c.m.r32((l_prm_2).wrapping_add(60))), F32(c.m.r32((p_w_1).wrapping_add(160)))); F::Vec3_ScaleAdd(c, a0_, (p_w_1).wrapping_add(128), (p_w_1).wrapping_add(96), a3_) };
        }
        {
            l_n_3 = c.m.r32((p_w_1).wrapping_add(440));
            'l1: loop {
                if !(((((l_n_3) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((c.m.r32((l_n_3).wrapping_add(96))) != (((0i32) as u32))) as i32)) != 0) {
                        { let a1_ = (c.m.r32((l_n_3).wrapping_add(96))).wrapping_add(64); F::Vec3_Sub(c, fp, a1_, (l_n_3).wrapping_add(64)) };
                        l_len_4 = F::Vec3_Length(c, fp);
                        if (((rt::fgt(l_len_4, F32(0x00000000)) as i32)) != 0) {
                            F::Vec3_Normalize(c, fp, fp);
                            if ((((!(((rt::flt(rt::fmul(F32(c.m.r32((l_prm_2).wrapping_add(84))), F32(c.m.r32((p_w_1).wrapping_add(160)))), l_len_4) as i32)) != 0)) as i32)) != 0) {
                                F::Vec3_ScaleAdd(c, fp.wrapping_add(16), fp, (l_n_3).wrapping_add(64), l_len_4);
                            } else {
                                { let a3_ = rt::fmul(F32(c.m.r32((l_prm_2).wrapping_add(84))), F32(c.m.r32((p_w_1).wrapping_add(160)))); F::Vec3_ScaleAdd(c, fp.wrapping_add(16), fp, (l_n_3).wrapping_add(64), a3_) };
                            }
                        } else {
                            F::Vec4_Copy(c, fp.wrapping_add(16), (l_n_3).wrapping_add(64));
                        }
                        { let a2_ = (c.m.r32((l_n_3).wrapping_add(96))).wrapping_add(64); F::Vec3_Sub(c, fp, fp.wrapping_add(16), a2_) };
                        { let a2_ = rt::fneg(rt::fmul(F32(c.m.r32((l_prm_2).wrapping_add(88))), F32(c.m.r32((p_w_1).wrapping_add(160))))); F::Vec3_Scale(c, fp, fp, a2_) };
                        { let a0_ = (c.m.r32((l_n_3).wrapping_add(96))).wrapping_add(64); F::Vec3_Add(c, a0_, fp.wrapping_add(16), fp) };
                    }
                }
                l_n_3 = c.m.r32((l_n_3).wrapping_add(96));
            }
        }
        { let v_ = F32(0x00000000); let a_ = (p_w_1).wrapping_add(164); c.m.w32(a_, (v_).0); };
        {
            l_n_3 = c.m.r32((p_w_1).wrapping_add(440));
            'l3: loop {
                if !(((((l_n_3) != (((0i32) as u32))) as i32)) != 0) { break 'l3; }
                'c4: {
                    if (((((c.m.r32((l_n_3).wrapping_add(96))) != (((0i32) as u32))) as i32)) != 0) {
                        { let a1_ = (c.m.r32((l_n_3).wrapping_add(96))).wrapping_add(64); F::Vec3_Sub(c, fp, a1_, (l_n_3).wrapping_add(64)) };
                    } else {
                        if (((((c.m.r32((l_n_3).wrapping_add(100))) != (((0i32) as u32))) as i32)) != 0) {
                            { let a2_ = (c.m.r32((l_n_3).wrapping_add(100))).wrapping_add(64); F::Vec3_Sub(c, fp, (l_n_3).wrapping_add(64), a2_) };
                        }
                    }
                    { let v_ = F::Vec3_Length(c, fp); let a_ = (l_n_3).wrapping_add(80); c.m.w32(a_, (v_).0); };
                    { let r_ = F32(c.m.r32((l_n_3).wrapping_add(80))); let a_ = (p_w_1).wrapping_add(164); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if (((rt::fgt(F32(c.m.r32((l_n_3).wrapping_add(80))), F32(0x00000000)) as i32)) != 0) {
                        F::Vec3_Normalize(c, fp, fp);
                        { let v_ = { let a0_ = { let a0_ = rt::fneg(F32(c.m.r32((fp).wrapping_add(4)))); F::Mathf_Asin(c, a0_) }; F::EftMath_WrapAngle(c, a0_) }; let a_ = (l_n_3).wrapping_add(84); c.m.w32(a_, (v_).0); };
                        { let v_ = { let a0_ = { let a0_ = F32(c.m.r32(fp)); let a1_ = F32(c.m.r32((fp).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) }; F::EftMath_WrapAngle(c, a0_) }; let a_ = (l_n_3).wrapping_add(88); c.m.w32(a_, (v_).0); };
                    } else {
                        if (((((c.m.r32((l_n_3).wrapping_add(100))) != (((0i32) as u32))) as i32)) != 0) {
                            { let v_ = { let a_ = (c.m.r32((l_n_3).wrapping_add(100))).wrapping_add(84); F32(c.m.r32(a_)) }; let a_ = (l_n_3).wrapping_add(84); c.m.w32(a_, (v_).0); };
                            { let v_ = { let a_ = (c.m.r32((l_n_3).wrapping_add(100))).wrapping_add(88); F32(c.m.r32(a_)) }; let a_ = (l_n_3).wrapping_add(88); c.m.w32(a_, (v_).0); };
                        }
                    }
                    F::Mtx_StoreIdentity(c, l_n_3);
                    { let a2_ = F32(c.m.r32((l_n_3).wrapping_add(84))); F::Mtx_RotateX(c, l_n_3, l_n_3, a2_) };
                    { let a2_ = F32(c.m.r32((l_n_3).wrapping_add(88))); F::Mtx_RotateY(c, l_n_3, l_n_3, a2_) };
                    F::Vec4_Copy(c, (l_n_3).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (l_n_3).wrapping_add(64));
                }
                l_n_3 = c.m.r32((l_n_3).wrapping_add(96));
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_UpdateNodes", &PROBE_N, &[], true); }
    r_
}

pub fn EftOrbTail_AllocNode(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_AllocNode", &PROBE_N, &[], false); }
    let mut l_mgr_1: u32 = 0;
    let mut l_next_2: u32 = 0;
    let mut l_i_3: u32 = 0;
    let mut l_start_4: u32 = 0;
    let mut l_n_5: u32 = 0;
    let r_ = 'f: {
    l_mgr_1 = { let a_ = c.m.r32(sym::gEftOrbTail); c.m.r32(a_) };
    l_next_2 = c.m.r32((l_mgr_1).wrapping_add(47440));
    if (((((l_next_2) >= (((128i32) as u32))) as i32)) != 0) {
        { let v_ = ((0i32) as u32); let a_ = (l_mgr_1).wrapping_add(47440); c.m.w32(a_, v_); };
        l_next_2 = ((0i32) as u32);
    }
    l_i_3 = l_next_2;
    l_start_4 = l_i_3;
    'l1: loop {
        'c2: {
            l_n_5 = ((l_mgr_1).wrapping_add(1104)).wrapping_add((l_i_3).wrapping_mul(112));
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            if (((((l_i_3) >= (((128i32) as u32))) as i32)) != 0) {
                l_i_3 = ((0i32) as u32);
            }
            if ((((((c.m.r32((l_n_5).wrapping_add(92)) as i32)) == (0i32)) as i32)) != 0) {
                { let v_ = l_i_3; let a_ = (l_mgr_1).wrapping_add(47440); c.m.w32(a_, v_); };
                break 'f l_n_5;
            }
        }
        if !(((((l_i_3) != (l_start_4)) as i32)) != 0) { break 'l1; }
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_AllocNode", &PROBE_N, &[], true); }
    r_
}

pub fn EftOrbTail_AddStreak(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_AddStreak", &PROBE_N, &[(p_w_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32((p_w_1).wrapping_add(440))) != (((0i32) as u32))) as i32)) != 0) {
        l_p_2 = F::EftOrbTail_AllocStreak(c);
        if (((((l_p_2) != (((0i32) as u32))) as i32)) != 0) {
            F::EftOrbTail_InitStreak(c, l_p_2, p_w_1);
            { let v_ = 1i32; let a_ = (l_p_2).wrapping_add(136); c.m.w32(a_, (v_) as u32); };
            { let v_ = ((0i32) as u32); let a_ = (l_p_2).wrapping_add(144); c.m.w32(a_, v_); };
            { let v_ = ((0i32) as u32); let a_ = (l_p_2).wrapping_add(140); c.m.w32(a_, v_); };
            if (((((c.m.r32((p_w_1).wrapping_add(448))) == (((0i32) as u32))) as i32)) != 0) {
                { let v_ = l_p_2; let a_ = (p_w_1).wrapping_add(448); c.m.w32(a_, v_); };
                { let v_ = l_p_2; let a_ = (p_w_1).wrapping_add(452); c.m.w32(a_, v_); };
            } else {
                { let v_ = c.m.r32((p_w_1).wrapping_add(452)); let a_ = (l_p_2).wrapping_add(144); c.m.w32(a_, v_); };
                { let v_ = l_p_2; let a_ = (c.m.r32((p_w_1).wrapping_add(452))).wrapping_add(140); c.m.w32(a_, v_); };
                { let v_ = l_p_2; let a_ = (p_w_1).wrapping_add(452); c.m.w32(a_, v_); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_AddStreak", &PROBE_N, &[], true); }
    r_
}

pub fn EftOrbTail_InitStreak(c: &mut Ctx, mut p_p_1: u32, mut p_w_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_InitStreak", &PROBE_N, &[(p_p_1) as u32, (p_w_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_prm_3: u32 = 0;
    let mut l_col_4: u32 = 0;
    let r_ = 'f: {
        l_prm_3 = c.m.r32((p_w_2).wrapping_add(48));
        l_col_4 = c.m.r32(((p_w_2).wrapping_add(48)).wrapping_add(4));
        rt::memset(c, fp, 0, 12);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(8), (v_).0); }
        { let a1_ = (l_col_4).wrapping_add((((c.m.r8(((p_w_2).wrapping_add(48)).wrapping_add(32))) as u32)).wrapping_mul(16)); F::Vec4_Copy(c, p_p_1, a1_) };
        F::Vec4_Set(c, (p_p_1).wrapping_add(16), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, (p_p_1).wrapping_add(32), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, (p_p_1).wrapping_add(48), F32(0x00000000), F32(0x00000000), F32(0x3f800000), F32(0x3f800000));
        { let v_ = rt::fmul(rt::fmul({ let a0_ = F32(c.m.r32((l_prm_3).wrapping_add(52))); let a1_ = rt::fadd(F32(c.m.r32((l_prm_3).wrapping_add(52))), F32(c.m.r32((l_prm_3).wrapping_add(56)))); F::Rand_FloatRange(c, a0_, a1_) }, F32(0x41f00000)), F32(c.m.r32((p_w_2).wrapping_add(160)))); let a_ = (p_p_1).wrapping_add(108); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_p_1).wrapping_add(104); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul({ let a0_ = F32(c.m.r32((l_prm_3).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32((l_prm_3).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_prm_3).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }, F32(c.m.r32((p_w_2).wrapping_add(160)))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul({ let a0_ = F32(c.m.r32((l_prm_3).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32((l_prm_3).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_prm_3).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }, F32(c.m.r32((p_w_2).wrapping_add(160)))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul({ let a0_ = F32(c.m.r32((l_prm_3).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32((l_prm_3).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_prm_3).wrapping_add(12)).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }, F32(c.m.r32((p_w_2).wrapping_add(160)))); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(108))), F32(c.m.r32((l_prm_3).wrapping_add(48))))); let a_ = (p_p_1).wrapping_add(68); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(108))), rt::fsub(F32(0x3f800000), F32(c.m.r32((l_prm_3).wrapping_add(48)))))); let a_ = (p_p_1).wrapping_add(72); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_p_1).wrapping_add(64); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul({ let a0_ = F32(c.m.r32(((l_prm_3).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((l_prm_3).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_prm_3).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }, F32(c.m.r32((p_w_2).wrapping_add(160)))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul({ let a0_ = F32(c.m.r32(((l_prm_3).wrapping_add(24)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((l_prm_3).wrapping_add(24)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_prm_3).wrapping_add(36)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }, F32(c.m.r32((p_w_2).wrapping_add(160)))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul({ let a0_ = F32(c.m.r32(((l_prm_3).wrapping_add(24)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((l_prm_3).wrapping_add(24)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_prm_3).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }, F32(c.m.r32((p_w_2).wrapping_add(160)))); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(108))), F32(c.m.r32((l_prm_3).wrapping_add(48))))); let a_ = (p_p_1).wrapping_add(84); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(108))), rt::fsub(F32(0x3f800000), F32(c.m.r32((l_prm_3).wrapping_add(48)))))); let a_ = (p_p_1).wrapping_add(88); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_p_1).wrapping_add(80); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((l_col_4).wrapping_add(112))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32((l_col_4).wrapping_add(116))), F32(c.m.r32((l_col_4).wrapping_add(112)))), F32(0x3f000000)); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((l_col_4).wrapping_add(116))); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(108))), F32(c.m.r32((l_col_4).wrapping_add(120))))); let a_ = (p_p_1).wrapping_add(96); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(108))), rt::fsub(F32(0x3f800000), F32(c.m.r32((l_col_4).wrapping_add(120)))))); let a_ = (p_p_1).wrapping_add(100); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_p_1).wrapping_add(92); c.m.w32(a_, (v_).0); };
        { let v_ = F::Rand_FloatRange(c, F32(0xc0490fda), F32(0x40490fda)); let a_ = (p_p_1).wrapping_add(76); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fmul({ let a0_ = F32(c.m.r32((l_prm_3).wrapping_add(64))); let a1_ = rt::fadd(F32(c.m.r32((l_prm_3).wrapping_add(64))), F32(c.m.r32((l_prm_3).wrapping_add(68)))); F::Rand_FloatRange(c, a0_, a1_) }, F32(0x41f00000)), F32(c.m.r32((p_w_2).wrapping_add(160)))); let a_ = (p_p_1).wrapping_add(124); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fmul({ let a0_ = F32(c.m.r32((l_prm_3).wrapping_add(72))); let a1_ = rt::fadd(F32(c.m.r32((l_prm_3).wrapping_add(72))), F32(c.m.r32((l_prm_3).wrapping_add(76)))); F::Rand_FloatRange(c, a0_, a1_) }, F32(0x41f00000)), F32(c.m.r32((p_w_2).wrapping_add(160)))); let a_ = (p_p_1).wrapping_add(128); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((p_p_1).wrapping_add(128))); let a_ = (p_p_1).wrapping_add(120); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_p_1).wrapping_add(116); c.m.w32(a_, (v_).0); };
        { let v_ = { let a1_ = rt::fsub(F32(c.m.r32((p_w_2).wrapping_add(172))), F32(0x3f800000)); F::Rand_FloatRange(c, F32(0x00000000), a1_) }; let a_ = (p_p_1).wrapping_add(132); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_p_1).wrapping_add(112); c.m.w32(a_, (v_).0); };
        { let v_ = c.m.r32((p_w_2).wrapping_add(440)); let a_ = (p_p_1).wrapping_add(148); c.m.w32(a_, v_); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_InitStreak", &PROBE_N, &[], true); }
    r_
}

pub fn EftOrbTail_UpdateStreaks(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_UpdateStreaks", &PROBE_N, &[(p_w_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_arg_2: u32 = 0;
    let mut l_gap_3: i32 = 0;
    let mut l_prm_4: u32 = 0;
    let mut l_col_5: u32 = 0;
    let mut l_p_6: u32 = 0;
    let mut l_fadeIn_7: F32 = F32(0);
    let mut l_fadeOut_8: F32 = F32(0);
    let mut l_tailFade_9: F32 = F32(0);
    let mut l_t_10: F32 = F32(0);
    let mut l_rest_11: F32 = F32(0);
    let r_ = 'f: {
        l_arg_2 = (p_w_1).wrapping_add(48);
        l_gap_3 = 0i32;
        l_prm_4 = c.m.r32(l_arg_2);
        l_col_5 = c.m.r32((l_arg_2).wrapping_add(4));
        rt::memset(c, fp, 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(12), (v_).0); }
        l_fadeIn_7 = F32(0x00000000);
        l_tailFade_9 = F32(0x3f800000);
        {
            l_p_6 = c.m.r32((p_w_1).wrapping_add(448));
            'l1: loop {
                if !(((((l_p_6) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_t_10 = rt::fdiv(F32(c.m.r32((l_p_6).wrapping_add(104))), F32(c.m.r32((l_p_6).wrapping_add(108))));
                    { let r_ = F32(0x3f800000); let a_ = (l_p_6).wrapping_add(104); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if (((rt::flt(l_t_10, F32(c.m.r32((l_prm_4).wrapping_add(48)))) as i32)) != 0) {
                        { let r_ = F32(c.m.r32((l_p_6).wrapping_add(84))); let a_ = (l_p_6).wrapping_add(80); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        { let r_ = F32(c.m.r32((l_p_6).wrapping_add(68))); let a_ = (l_p_6).wrapping_add(64); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    } else {
                        { let r_ = F32(c.m.r32((l_p_6).wrapping_add(88))); let a_ = (l_p_6).wrapping_add(80); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        { let r_ = F32(c.m.r32((l_p_6).wrapping_add(72))); let a_ = (l_p_6).wrapping_add(64); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    }
                    if (((rt::flt(l_t_10, F32(c.m.r32((l_col_5).wrapping_add(120)))) as i32)) != 0) {
                        { let r_ = F32(c.m.r32((l_p_6).wrapping_add(96))); let a_ = (l_p_6).wrapping_add(92); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    } else {
                        { let r_ = F32(c.m.r32((l_p_6).wrapping_add(100))); let a_ = (l_p_6).wrapping_add(92); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    }
                    { let a2_ = F32(c.m.r32((l_p_6).wrapping_add(64))); F::Vec3_Scale(c, fp, (l_p_6).wrapping_add(48), a2_) };
                    F::Vec3_Add(c, (l_p_6).wrapping_add(32), (l_p_6).wrapping_add(32), fp);
                    if (((rt::fle({ let a_ = (c.m.r32((l_p_6).wrapping_add(148))).wrapping_add(80); F32(c.m.r32(a_)) }, F32(c.m.r32(((l_p_6).wrapping_add(32)).wrapping_add(8)))) as i32)) != 0) {
                        l_rest_11 = rt::fsub(F32(c.m.r32(((l_p_6).wrapping_add(32)).wrapping_add(8))), { let a_ = (c.m.r32((l_p_6).wrapping_add(148))).wrapping_add(80); F32(c.m.r32(a_)) });
                        if ((((({ let a_ = (c.m.r32((l_p_6).wrapping_add(148))).wrapping_add(96); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
                            if (((rt::fle({ let a_ = ({ let a_ = (c.m.r32((l_p_6).wrapping_add(148))).wrapping_add(96); c.m.r32(a_) }).wrapping_add(80); F32(c.m.r32(a_)) }, l_rest_11) as i32)) != 0) {
                                {
                                    'l3: loop {
                                        'c4: {
                                            { let v_ = { let a_ = (c.m.r32((l_p_6).wrapping_add(148))).wrapping_add(96); c.m.r32(a_) }; let a_ = (l_p_6).wrapping_add(148); c.m.w32(a_, v_); };
                                            { let r_ = { let a_ = (c.m.r32((l_p_6).wrapping_add(148))).wrapping_add(80); F32(c.m.r32(a_)) }; let a_ = (l_p_6).wrapping_add(112); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                            if ((((((((({ let a_ = (c.m.r32((l_p_6).wrapping_add(148))).wrapping_add(96); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) && ((((({ let a_ = ({ let a_ = (c.m.r32((l_p_6).wrapping_add(148))).wrapping_add(96); c.m.r32(a_) }).wrapping_add(96); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
                                                { let r_ = 2i32; let a_ = (l_p_6).wrapping_add(136); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                            }
                                            if (((rt::fle({ let a_ = (c.m.r32((l_p_6).wrapping_add(148))).wrapping_add(80); F32(c.m.r32(a_)) }, l_rest_11) as i32)) != 0) {
                                                { let r_ = { let a_ = (c.m.r32((l_p_6).wrapping_add(148))).wrapping_add(80); F32(c.m.r32(a_)) }; let n_ = rt::fsub(l_rest_11, r_); l_rest_11 = n_; };
                                            } else {
                                                break 'l3;
                                            }
                                            if ((((({ let a_ = (c.m.r32((l_p_6).wrapping_add(148))).wrapping_add(96); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
                                                break 'l3;
                                            }
                                        }
                                    }
                                }
                            } else {
                                { let v_ = { let a_ = (c.m.r32((l_p_6).wrapping_add(148))).wrapping_add(96); c.m.r32(a_) }; let a_ = (l_p_6).wrapping_add(148); c.m.w32(a_, v_); };
                                { let r_ = { let a_ = (c.m.r32((l_p_6).wrapping_add(148))).wrapping_add(80); F32(c.m.r32(a_)) }; let a_ = (l_p_6).wrapping_add(112); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            }
                            { let r_ = l_rest_11; let a_ = (l_p_6).wrapping_add(112); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            F::Vec4_Set(c, (l_p_6).wrapping_add(32), F32(0x00000000), F32(0x00000000), l_rest_11, F32(0x3f800000));
                        }
                    }
                    { let a1_ = c.m.r32((l_p_6).wrapping_add(148)); F::Mtx_MulVec4(c, (l_p_6).wrapping_add(16), a1_, (l_p_6).wrapping_add(32)) };
                    if (((rt::fgt(rt::fmul(F32(c.m.r32((p_w_1).wrapping_add(164))), F32(c.m.r32((l_prm_4).wrapping_add(80)))), F32(0x00000000)) as i32)) != 0) {
                        l_tailFade_9 = rt::fsub(F32(0x3f800000), (if (((rt::flt(rt::fdiv(F32(c.m.r32((l_p_6).wrapping_add(112))), rt::fmul(F32(c.m.r32((p_w_1).wrapping_add(164))), rt::fmul(F32(c.m.r32((l_prm_4).wrapping_add(80))), F32(c.m.r32((p_w_1).wrapping_add(156)))))), F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(0x3f800000), rt::fdiv(F32(c.m.r32((l_p_6).wrapping_add(112))), rt::fmul(F32(c.m.r32((p_w_1).wrapping_add(164))), rt::fmul(F32(c.m.r32((l_prm_4).wrapping_add(80))), F32(c.m.r32((p_w_1).wrapping_add(156))))))) as i32)) != 0) { F32(0x3f800000) } else { rt::fdiv(F32(c.m.r32((l_p_6).wrapping_add(112))), rt::fmul(F32(c.m.r32((p_w_1).wrapping_add(164))), rt::fmul(F32(c.m.r32((l_prm_4).wrapping_add(80))), F32(c.m.r32((p_w_1).wrapping_add(156)))))) }) }));
                    } else {
                        l_tailFade_9 = F32(0x00000000);
                    }
                    if (((((c.m.r32((p_w_1).wrapping_add(436)) as i32)) & (256i32))) != 0) {
                        { let r_ = F32(0x3f800000); let a_ = (l_p_6).wrapping_add(132); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        if (((rt::fle(F32(c.m.r32((p_w_1).wrapping_add(172))), F32(c.m.r32((l_p_6).wrapping_add(132)))) as i32)) != 0) {
                            { let v_ = F32(0x00000000); let a_ = (l_p_6).wrapping_add(132); c.m.w32(a_, (v_).0); };
                        }
                    }
                    if ((((!(((((((((c.m.r32((p_w_1).wrapping_add(436)) as i32)) & (64i32))) != 0) && ((((!(((((c.m.r32((p_w_1).wrapping_add(436)) as i32)) & (2i32))) != 0)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
                        { let r_ = 2i32; let a_ = (l_p_6).wrapping_add(136); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    if ((((((((({ let a_ = (c.m.r32((l_p_6).wrapping_add(148))).wrapping_add(96); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) || ((((({ let a_ = ({ let a_ = (c.m.r32((l_p_6).wrapping_add(148))).wrapping_add(96); c.m.r32(a_) }).wrapping_add(96); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
                        { let r_ = 2i32; let a_ = (l_p_6).wrapping_add(136); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    if (((((c.m.r32((l_p_6).wrapping_add(136)) as i32)) & (2i32))) != 0) {
                        if (((rt::fgt(F32(c.m.r32((l_p_6).wrapping_add(128))), F32(0x00000000)) as i32)) != 0) {
                            l_fadeOut_8 = (if (((rt::flt(rt::fdiv(F32(c.m.r32((l_p_6).wrapping_add(120))), F32(c.m.r32((l_p_6).wrapping_add(128)))), F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(0x3f800000), rt::fdiv(F32(c.m.r32((l_p_6).wrapping_add(120))), F32(c.m.r32((l_p_6).wrapping_add(128))))) as i32)) != 0) { F32(0x3f800000) } else { rt::fdiv(F32(c.m.r32((l_p_6).wrapping_add(120))), F32(c.m.r32((l_p_6).wrapping_add(128)))) }) });
                        } else {
                            l_fadeOut_8 = F32(0x00000000);
                        }
                    } else {
                        l_fadeOut_8 = F32(0x3f800000);
                    }
                    if (((rt::fgt(F32(c.m.r32((l_p_6).wrapping_add(124))), F32(0x00000000)) as i32)) != 0) {
                        l_fadeIn_7 = (if (((rt::flt(rt::fdiv(F32(c.m.r32((l_p_6).wrapping_add(116))), F32(c.m.r32((l_p_6).wrapping_add(124)))), F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(0x3f800000), rt::fdiv(F32(c.m.r32((l_p_6).wrapping_add(116))), F32(c.m.r32((l_p_6).wrapping_add(124))))) as i32)) != 0) { F32(0x3f800000) } else { rt::fdiv(F32(c.m.r32((l_p_6).wrapping_add(116))), F32(c.m.r32((l_p_6).wrapping_add(124)))) }) });
                    } else {
                        l_fadeIn_7 = F32(0x3f800000);
                    }
                    if (((((c.m.r32((l_p_6).wrapping_add(136)) as i32)) & (2i32))) != 0) {
                        if (((rt::fgt(F32(c.m.r32((l_p_6).wrapping_add(120))), F32(0x00000000)) as i32)) != 0) {
                            { let r_ = F32(0x3f800000); let a_ = (l_p_6).wrapping_add(120); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        }
                    } else {
                        if (((rt::flt(F32(c.m.r32((l_p_6).wrapping_add(116))), F32(c.m.r32((l_p_6).wrapping_add(124)))) as i32)) != 0) {
                            { let r_ = F32(0x3f800000); let a_ = (l_p_6).wrapping_add(116); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        }
                    }
                    { let v_ = rt::fmul(rt::fmul(rt::fmul(rt::fmul(rt::fmul({ let a_ = ((l_col_5).wrapping_add((((((c.m.r8((l_arg_2).wrapping_add(32))) as i32)) as u32)).wrapping_mul(16))).wrapping_add(12); F32(c.m.r32(a_)) }, F32(c.m.r32((l_p_6).wrapping_add(92)))), rt::fsub(F32(0x3f800000), l_t_10)), l_fadeIn_7), l_fadeOut_8), l_tailFade_9); let a_ = (l_p_6).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    if (((((((rt::fle(F32(c.m.r32((l_p_6).wrapping_add(12))), F32(0x00000000)) as i32)) != 0) || (((rt::fle(F32(c.m.r32((l_p_6).wrapping_add(80))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                        { let r_ = (!(16i32)); let a_ = (l_p_6).wrapping_add(136); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                        if (((rt::fgt(l_fadeIn_7, F32(0x00000000)) as i32)) != 0) {
                            { let r_ = 2i32; let a_ = (l_p_6).wrapping_add(136); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                    } else {
                        { let r_ = 16i32; let a_ = (l_p_6).wrapping_add(136); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    if ((((!((((((((!(((((c.m.r32((l_p_6).wrapping_add(136)) as i32)) & (2i32))) != 0)) as i32)) != 0) || (((((c.m.r32((l_p_6).wrapping_add(136)) as i32)) & (16i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
                        { let r_ = 8i32; let a_ = (l_p_6).wrapping_add(136); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    if (((((((rt::fge(F32(c.m.r32((l_p_6).wrapping_add(104))), F32(c.m.r32((l_p_6).wrapping_add(108)))) as i32)) != 0) || (((((c.m.r32((l_p_6).wrapping_add(136)) as i32)) & (8i32))) != 0)) as i32)) != 0) {
                        if (((((c.m.r32((p_w_1).wrapping_add(448))) != (((0i32) as u32))) as i32)) != 0) {
                            if (((((c.m.r32((l_p_6).wrapping_add(144))) == (((0i32) as u32))) as i32)) != 0) {
                                if (((((c.m.r32((l_p_6).wrapping_add(140))) == (((0i32) as u32))) as i32)) != 0) {
                                    { let v_ = ((0i32) as u32); let a_ = (p_w_1).wrapping_add(448); c.m.w32(a_, v_); };
                                    { let v_ = ((0i32) as u32); let a_ = (p_w_1).wrapping_add(452); c.m.w32(a_, v_); };
                                } else {
                                    { let v_ = c.m.r32((l_p_6).wrapping_add(140)); let a_ = (p_w_1).wrapping_add(448); c.m.w32(a_, v_); };
                                    { let v_ = ((0i32) as u32); let a_ = (c.m.r32((p_w_1).wrapping_add(448))).wrapping_add(144); c.m.w32(a_, v_); };
                                }
                            } else {
                                if (((((c.m.r32((l_p_6).wrapping_add(140))) == (((0i32) as u32))) as i32)) != 0) {
                                    { let v_ = c.m.r32((l_p_6).wrapping_add(144)); let a_ = (p_w_1).wrapping_add(452); c.m.w32(a_, v_); };
                                    { let v_ = ((0i32) as u32); let a_ = (c.m.r32((p_w_1).wrapping_add(452))).wrapping_add(140); c.m.w32(a_, v_); };
                                } else {
                                    { let v_ = c.m.r32((l_p_6).wrapping_add(144)); let a_ = (c.m.r32((l_p_6).wrapping_add(140))).wrapping_add(144); c.m.w32(a_, v_); };
                                    { let v_ = c.m.r32((l_p_6).wrapping_add(140)); let a_ = (c.m.r32((l_p_6).wrapping_add(144))).wrapping_add(140); c.m.w32(a_, v_); };
                                }
                            }
                        }
                        { let v_ = 0i32; let a_ = (l_p_6).wrapping_add(136); c.m.w32(a_, (v_) as u32); };
                    }
                }
                l_p_6 = c.m.r32((l_p_6).wrapping_add(140));
            }
        }
        {
            l_p_6 = c.m.r32((p_w_1).wrapping_add(452));
            'l5: loop {
                if !(((((l_p_6) != (((0i32) as u32))) as i32)) != 0) { break 'l5; }
                'c6: {
                    if ((((!(((((c.m.r32((l_p_6).wrapping_add(136)) as i32)) & (2i32))) != 0)) as i32)) != 0) {
                        if (((((c.m.r32((l_p_6).wrapping_add(144))) != (((0i32) as u32))) as i32)) != 0) {
                            { let a2_ = (c.m.r32((l_p_6).wrapping_add(144))).wrapping_add(16); F::Vec3_Sub(c, fp, (l_p_6).wrapping_add(16), a2_) };
                            if (((rt::fgt(F::Vec3_Length(c, fp), rt::fmul(F32(c.m.r32((l_p_6).wrapping_add(80))), F32(0x3fc00000))) as i32)) != 0) {
                                l_gap_3 = 1i32;
                            }
                        }
                        if ((l_gap_3) != 0) {
                            { let r_ = 2i32; let a_ = (l_p_6).wrapping_add(136); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                    }
                }
                l_p_6 = c.m.r32((l_p_6).wrapping_add(144));
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_UpdateStreaks", &PROBE_N, &[], true); }
    r_
}

pub fn EftOrbTail_DrawStreaks(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_DrawStreaks", &PROBE_N, &[(p_w_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_tex_2: u32 = 0;
    let mut l_prm_3: u32 = 0;
    let mut l_p_4: u32 = 0;
    let mut l_f_5: i32 = 0;
    let r_ = 'f: {
        l_tex_2 = c.m.r32(p_w_1);
        l_prm_3 = c.m.r32((p_w_1).wrapping_add(48));
        rt::memset(c, fp, 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(12), (v_).0); }
        {
            l_p_4 = c.m.r32((p_w_1).wrapping_add(448));
            'l1: loop {
                if !(((((l_p_4) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((c.m.r32((l_p_4).wrapping_add(136)) as i32)) & (16i32))) != 0) {
                        if (((((c.m.r32((p_w_1).wrapping_add(436)) as i32)) & (256i32))) != 0) {
                            if (((rt::flt(F32(c.m.r32((l_p_4).wrapping_add(132))), F32(0x00000000)) as i32)) != 0) {
                                l_f_5 = 0i32;
                            } else {
                                if (((rt::flt(F32(c.m.r32((p_w_1).wrapping_add(172))), F32(c.m.r32((l_p_4).wrapping_add(132)))) as i32)) != 0) {
                                    l_f_5 = rt::f2i(F32(c.m.r32((p_w_1).wrapping_add(172))));
                                } else {
                                    l_f_5 = rt::f2i(F32(c.m.r32((l_p_4).wrapping_add(132))));
                                }
                            }
                            { let v_ = F32(c.m.r32((((p_w_1).wrapping_add(176)).wrapping_add((((l_f_5) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = fp; c.m.w32(a_, (v_).0); };
                            { let v_ = F32(c.m.r32((((p_w_1).wrapping_add(176)).wrapping_add((((l_f_5) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(c.m.r32((((p_w_1).wrapping_add(176)).wrapping_add((((l_f_5) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(c.m.r32((((p_w_1).wrapping_add(176)).wrapping_add((((l_f_5) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)))); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        } else {
                            { let v_ = F32(0x00000000); let a_ = fp; c.m.w32(a_, (v_).0); };
                            { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        }
                        { let a2_ = F32(c.m.r32((l_p_4).wrapping_add(80))); let a3_ = F32(c.m.r32((l_p_4).wrapping_add(80))); let a4_ = F32(c.m.r32(fp)); let a5_ = F32(c.m.r32((fp).wrapping_add(4))); let a6_ = F32(c.m.r32((fp).wrapping_add(8))); let a7_ = F32(c.m.r32((fp).wrapping_add(12))); let a8_ = F32(c.m.r32((l_p_4).wrapping_add(76))); let a9_ = ((c.m.r8((l_prm_3).wrapping_add(123))) as i32); let a11_ = { let a_ = (l_tex_2).wrapping_add((((c.m.r8((p_w_1).wrapping_add(434))) as u32)).wrapping_mul(16)); c.m.r64(a_) }; F::EftPrim_DrawQuadDepthScaled(c, (l_p_4).wrapping_add(16), l_p_4, a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_, 0i32, a11_, F32(0x40000000)) };
                    }
                }
                l_p_4 = c.m.r32((l_p_4).wrapping_add(140));
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_DrawStreaks", &PROBE_N, &[], true); }
    r_
}

pub fn EftOrbTail_AllocStreak(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_AllocStreak", &PROBE_N, &[], false); }
    let mut l_mgr_1: u32 = 0;
    let mut l_next_2: u32 = 0;
    let mut l_i_3: u32 = 0;
    let mut l_start_4: u32 = 0;
    let mut l_p_5: u32 = 0;
    let r_ = 'f: {
    l_mgr_1 = { let a_ = c.m.r32(sym::gEftOrbTail); c.m.r32(a_) };
    l_next_2 = c.m.r32((l_mgr_1).wrapping_add(47444));
    if (((((l_next_2) >= (((200i32) as u32))) as i32)) != 0) {
        { let v_ = ((0i32) as u32); let a_ = (l_mgr_1).wrapping_add(47444); c.m.w32(a_, v_); };
        l_next_2 = ((0i32) as u32);
    }
    l_i_3 = l_next_2;
    l_start_4 = l_i_3;
    'l1: loop {
        'c2: {
            l_p_5 = ((l_mgr_1).wrapping_add(15440)).wrapping_add((l_i_3).wrapping_mul(160));
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            if (((((l_i_3) >= (((200i32) as u32))) as i32)) != 0) {
                l_i_3 = ((0i32) as u32);
            }
            if ((((((c.m.r32((l_p_5).wrapping_add(136)) as i32)) == (0i32)) as i32)) != 0) {
                { let v_ = l_i_3; let a_ = (l_mgr_1).wrapping_add(47444); c.m.w32(a_, v_); };
                break 'f l_p_5;
            }
        }
        if !(((((l_i_3) != (l_start_4)) as i32)) != 0) { break 'l1; }
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_AllocStreak", &PROBE_N, &[], true); }
    r_
}

pub fn EftOrbTail_StartBurst(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_StartBurst", &PROBE_N, &[(p_w_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(144) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 144);
    let mut l_src_2: u32 = 0;
    let mut l_prm_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
        l_src_2 = (p_w_1).wrapping_add(48);
        l_prm_3 = c.m.r32(l_src_2);
        {
            rt::memset(c, fp.wrapping_add(64), 0, 64);
            rt::memset(c, fp.wrapping_add(64).wrapping_add(0), 0, 16);
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(0).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(0).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(0).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(0).wrapping_add(12), (v_).0); }
            rt::memset(c, fp.wrapping_add(64).wrapping_add(16), 0, 16);
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(16).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(16).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(16).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(16).wrapping_add(12), (v_).0); }
            { let v_ = 0i32; c.m.w32(fp.wrapping_add(64).wrapping_add(32), (v_) as u32); }
            { let v_ = 0i32; c.m.w32(fp.wrapping_add(64).wrapping_add(36), (v_) as u32); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(40), (v_).0); }
            { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(64).wrapping_add(44), (v_).0); }
            { let v_ = ((c.m.r8((l_src_2).wrapping_add(33))) as i32); let a_ = (fp.wrapping_add(64)).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
            { let s_ = fp.wrapping_add(64); let d_ = fp; rt::memcpy(c, d_, s_, 64); };
        }
        {
            rt::memset(c, fp.wrapping_add(128), 0, 16);
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(128).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(128).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(128).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(128).wrapping_add(12), (v_).0); }
            {
                l_i_4 = 0i32;
                'l1: loop {
                    if !(((((l_i_4) < (3i32)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        { let v_ = { let a_ = (({ let a_ = c.m.r32(sym::gEftOrbTail); c.m.r32(a_) }).wrapping_add(1064)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = (fp).wrapping_add(52); c.m.w32(a_, v_); };
                        { let v_ = { let a_ = (({ let a_ = c.m.r32(sym::gEftOrbTail); c.m.r32(a_) }).wrapping_add(1076)).wrapping_add((((c.m.r8((l_src_2).wrapping_add(32))) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = (fp).wrapping_add(56); c.m.w32(a_, v_); };
                        { let v_ = rt::fmul(F32(c.m.r32(((l_prm_3).wrapping_add(104)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)))), F32(c.m.r32((p_w_1).wrapping_add(160)))); let a_ = (fp).wrapping_add(40); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fmul(F32(c.m.r32(((l_prm_3).wrapping_add(92)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)))), F32(c.m.r32((p_w_1).wrapping_add(160)))); let a_ = (fp).wrapping_add(44); c.m.w32(a_, (v_).0); };
                        { let v_ = (({ let a_ = c.m.r32(sym::gEftOrbTail); c.m.r32(a_) }).wrapping_add(272)).wrapping_add((((l_i_4) as u32)).wrapping_mul(264)); let a_ = (fp).wrapping_add(48); c.m.w32(a_, v_); };
                        { let v_ = F::EftPtcl_Create(c, fp); let a_ = ((p_w_1).wrapping_add(456)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        { let a3_ = rt::fmul(F32(c.m.r32((l_prm_3).wrapping_add(60))), F32(c.m.r32((p_w_1).wrapping_add(160)))); F::Vec3_ScaleAdd(c, fp.wrapping_add(128), (p_w_1).wrapping_add(128), (p_w_1).wrapping_add(96), a3_) };
                        { let _ = { let a0_ = c.m.r32(((p_w_1).wrapping_add(456)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))); F::EftPtcl_SetPos(c, a0_, fp.wrapping_add(128)) }; };
                        { let _ = { let a0_ = c.m.r32(((p_w_1).wrapping_add(456)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))); F::EftPtcl_SetDir(c, a0_, (p_w_1).wrapping_add(128)) }; };
                        { let _ = { let a0_ = c.m.r32(((p_w_1).wrapping_add(456)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))); let a1_ = (({ let a_ = c.m.r32(sym::gEftOrbTail); c.m.r32(a_) }).wrapping_add(272)).wrapping_add((((l_i_4) as u32)).wrapping_mul(264)); F::EftPtcl_SetTexture(c, a0_, a1_, 0i32, 0i32) }; };
                    }
                    { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_StartBurst", &PROBE_N, &[], true); }
    r_
}

pub fn EftOrbTail_PollBurst(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_PollBurst", &PROBE_N, &[(p_w_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (3i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let _ = { let a0_ = c.m.r32(((p_w_1).wrapping_add(456)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))); F::EftPtcl_IsAlive(c, a0_) }; };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_PollBurst", &PROBE_N, &[], true); }
    r_
}

pub fn EftOrbTail_StopBurst(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_StopBurst", &PROBE_N, &[(p_w_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (3i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (({ let a0_ = c.m.r32(((p_w_1).wrapping_add(456)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))); F::EftPtcl_IsAlive(c, a0_) }) != 0) {
                    { let _ = { let a0_ = c.m.r32(((p_w_1).wrapping_add(456)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))); F::EftPtcl_Stop(c, a0_) }; };
                } else {
                    { let v_ = ((0i32) as u32); let a_ = ((p_w_1).wrapping_add(456)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_StopBurst", &PROBE_N, &[], true); }
    r_
}

pub fn EftOrbTail_InitFrames(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_InitFrames", &PROBE_N, &[(p_w_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_prm_2: u32 = 0;
    let mut l_n_3: u8 = 0;
    let mut l_row_4: i32 = 0;
    let mut l_col_5: i32 = 0;
    let r_ = 'f: {
        l_prm_2 = c.m.r32((p_w_1).wrapping_add(48));
        rt::memset(c, fp, 0, 8);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(4), (v_).0); }
        l_n_3 = ((0i32) as u8);
        l_row_4 = 0i32;
        { let v_ = rt::i2f((((c.m.r8((l_prm_2).wrapping_add(120))) as i32)).wrapping_mul(((c.m.r8((l_prm_2).wrapping_add(121))) as i32))); let a_ = (p_w_1).wrapping_add(172); c.m.w32(a_, (v_).0); };
        if (((((((rt::fgt(F32(c.m.r32((p_w_1).wrapping_add(172))), F32(0x3f800000)) as i32)) != 0) && (((rt::fle(F32(c.m.r32((p_w_1).wrapping_add(172))), F32(0x41800000)) as i32)) != 0)) as i32)) != 0) {
            { let v_ = rt::fdiv(F32(0x3f800000), rt::i2f(((c.m.r8((l_prm_2).wrapping_add(120))) as i32))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fdiv(F32(0x3f800000), rt::i2f(((c.m.r8((l_prm_2).wrapping_add(121))) as i32))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            {
                'l1: loop {
                    if !(((((l_row_4) < (((c.m.r8((l_prm_2).wrapping_add(121))) as i32))) as i32)) != 0) { break 'l1; }
                    'c2: {
                        {
                            l_col_5 = 0i32;
                            'l3: loop {
                                if !(((((l_col_5) < (((c.m.r8((l_prm_2).wrapping_add(120))) as i32))) as i32)) != 0) { break 'l3; }
                                'c4: {
                                    { let v_ = rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::i2f(l_col_5)); let a_ = (((p_w_1).wrapping_add(176)).wrapping_add((((l_n_3) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                    { let v_ = rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::i2f(l_row_4)); let a_ = (((p_w_1).wrapping_add(176)).wrapping_add((((l_n_3) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                    { let v_ = rt::fadd(rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::i2f(l_col_5)), F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(176)).wrapping_add((((l_n_3) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                    { let v_ = rt::fadd(rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::i2f(l_row_4)), F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(176)).wrapping_add((((l_n_3) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                    { let o_ = l_n_3; let n_ = (o_).wrapping_add(1); l_n_3 = n_; };
                                }
                                { let o_ = l_col_5; let n_ = (o_).wrapping_add(1); l_col_5 = n_; };
                            }
                        }
                    }
                    { let o_ = l_row_4; let n_ = (o_).wrapping_add(1); l_row_4 = n_; };
                }
            }
            { let r_ = 256i32; let a_ = (p_w_1).wrapping_add(436); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_InitFrames", &PROBE_N, &[], true); }
    r_
}

pub fn EftOrbTail_SetTex(c: &mut Ctx, mut p_w_1: u32, mut p_tex_2: u32, mut p_image_3: i32, mut p_palette_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_SetTex", &PROBE_N, &[(p_w_1) as u32, (p_tex_2) as u32, (p_image_3) as u32, (p_palette_4) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_tex_2; let a_ = p_w_1; c.m.w32(a_, v_); };
    if (((((p_image_3) == (p_palette_4)) as i32)) != 0) {
        { let v_ = ((p_palette_4) as u8); let a_ = (p_w_1).wrapping_add(434); c.m.w8(a_, v_); };
    } else {
        { let v_ = (((p_image_3).wrapping_add(p_palette_4)) as u8); let a_ = (p_w_1).wrapping_add(434); c.m.w8(a_, v_); };
    }
    { let s_ = (p_tex_2).wrapping_add((((p_image_3) as u32)).wrapping_mul(16)); let d_ = (p_w_1).wrapping_add(8); rt::memcpy(c, d_, s_, 16); };
    { let s_ = (p_tex_2).wrapping_add((((p_palette_4) as u32)).wrapping_mul(16)); let d_ = (p_w_1).wrapping_add(24); rt::memcpy(c, d_, s_, 16); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_SetTex", &PROBE_N, &[], true); }
    r_
}

pub fn EftOrbTail_UpdateTex(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_UpdateTex", &PROBE_N, &[(p_w_1) as u32], false); }
    let mut l_tex_2: u32 = 0;
    let mut l_tex0_3: u64 = 0;
    let r_ = 'f: {
    l_tex_2 = c.m.r32(p_w_1);
    if (((((l_tex_2) != (((0i32) as u32))) as i32)) != 0) {
        if ((((!((((c.m.r32((l_tex_2).wrapping_add(260))) & ((((1i32).wrapping_shl((((c.m.r8((p_w_1).wrapping_add(434))) as i32)) as u32)) as u32)))) != 0)) as i32)) != 0) {
            l_tex0_3 = F::EftVram_AddImage(c, (p_w_1).wrapping_add(8), 1i32, 0i32);
            { let r_ = (F::EftVram_AddClut(c, (p_w_1).wrapping_add(24))).wrapping_shl((37i32) as u32); let n_ = ((l_tex0_3) | (r_)); l_tex0_3 = n_; };
            { let v_ = l_tex0_3; let a_ = (l_tex_2).wrapping_add((((c.m.r8((p_w_1).wrapping_add(434))) as u32)).wrapping_mul(16)); c.m.w64(a_, v_); };
            { let r_ = (((1i32).wrapping_shl((((c.m.r8((p_w_1).wrapping_add(434))) as i32)) as u32)) as u32); let a_ = (l_tex_2).wrapping_add(260); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_UpdateTex", &PROBE_N, &[], true); }
    r_
}

pub fn EftOrbTail_Create(c: &mut Ctx, mut p_objId_1: i32, mut p_kind_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_Create", &PROBE_N, &[(p_objId_1) as u32, (p_kind_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_mgr_3: u32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 48);
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        rt::memset(c, fp.wrapping_add(16), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(12), (v_).0); }
        { let v_ = ((p_kind_2) as u8); c.m.w8(fp.wrapping_add(32), v_); }
        { let v_ = ((p_objId_1) as u8); c.m.w8(fp.wrapping_add(33), v_); }
        if (((((c.m.r32(sym::gEftOrbTail)) == (((0i32) as u32))) as i32)) != 0) {
            break 'f ((0i32) as u32);
        }
        l_mgr_3 = { let a_ = c.m.r32(sym::gEftOrbTail); c.m.r32(a_) };
        'l1: loop {
            'c2: {
                if (((((l_mgr_3) == (((0i32) as u32))) as i32)) != 0) {
                    break 'f ((0i32) as u32);
                }
                if (((((c.m.r32((l_mgr_3).wrapping_add(47448))) == (((0i32) as u32))) as i32)) != 0) {
                    break 'f ((0i32) as u32);
                }
            }
            if !((0i32) != 0) { break 'l1; }
        }
        { let v_ = { let a_ = ({ let a_ = c.m.r32(sym::gEftOrbTail); c.m.r32(a_) }).wrapping_add(4); c.m.r32(a_) }; let a_ = (fp).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = { let a_ = { let a_ = c.m.r32(sym::gEftOrbTail); c.m.r32(a_) }; c.m.r32(a_) }; let a_ = fp; c.m.w32(a_, v_); };
        break 'f { let a0_ = { let a_ = ({ let a_ = c.m.r32(sym::gEftOrbTail); c.m.r32(a_) }).wrapping_add(47448); c.m.r32(a_) }; F::BtlTaskList_AddTail(c, a0_, sym::gEftOrbTailClass, fp) };
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_Create", &PROBE_N, &[], true); }
    r_
}

pub fn EftOrbTail_Burst(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_Burst", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftOrbTail)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = c.m.r32(sym::gEftOrbTail); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftOrbTail_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32((l_w_2).wrapping_add(436)) as i32)) & (1i32))) != 0) {
        { let r_ = 128i32; let a_ = (l_w_2).wrapping_add(436); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_Burst", &PROBE_N, &[], true); }
    r_
}

pub fn EftOrbTail_End(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_End", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftOrbTail)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = c.m.r32(sym::gEftOrbTail); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftOrbTail_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32((l_w_2).wrapping_add(436)) as i32)) & (1i32))) != 0) {
        { let r_ = 2i32; let a_ = (l_w_2).wrapping_add(436); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_End", &PROBE_N, &[], true); }
    r_
}

pub fn EftOrbTail_Kill(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_Kill", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftOrbTail)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = c.m.r32(sym::gEftOrbTail); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftOrbTail_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32((l_w_2).wrapping_add(436)) as i32)) & (1i32))) != 0) {
        { let r_ = 4i32; let a_ = (l_w_2).wrapping_add(436); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_Kill", &PROBE_N, &[], true); }
    r_
}

pub fn EftOrbTail_SetPos(c: &mut Ctx, mut p_task_1: u32, p_pos_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_SetPos", &PROBE_N, &[(p_task_1) as u32, (p_pos_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_w_3: u32 = 0;
    rt::memcpy(c, fp, p_pos_2, 16);
    let r_ = 'f: {
        rt::memset(c, fp.wrapping_add(16), 0, 32);
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(16).wrapping_add(16), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(48), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(48).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(48).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(48).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(48).wrapping_add(12), (v_).0); }
        if (((((((((c.m.r32(sym::gEftOrbTail)) != (((0i32) as u32))) as i32)) != 0) && ((((({ let a_ = c.m.r32(sym::gEftOrbTail); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
            if (((((((((p_task_1) != (((0i32) as u32))) as i32)) != 0) && ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftOrbTail_Update)) as i32)) != 0)) as i32)) != 0) {
                l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
                if (((((l_w_3) != (((0i32) as u32))) as i32)) != 0) {
                    if (((((c.m.r32((l_w_3).wrapping_add(436)) as i32)) & (1i32))) != 0) {
                        if (((((c.m.r32((l_w_3).wrapping_add(436)) as i32)) & (512i32))) != 0) {
                            F::Vec4_Copy(c, (l_w_3).wrapping_add(112), (l_w_3).wrapping_add(96));
                        } else {
                            { let a0_ = ((c.m.r8(((l_w_3).wrapping_add(48)).wrapping_add(33))) as i32); F::BtlCharApi_GetNodePos(c, a0_, 46i32, (fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) };
                            { let a0_ = ((c.m.r8(((l_w_3).wrapping_add(48)).wrapping_add(33))) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, (fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) };
                            F::Vec3_Sub(c, fp.wrapping_add(48), (fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
                            F::Vec3_Normalize(c, fp.wrapping_add(48), fp.wrapping_add(48));
                            F::Vec3_Add(c, (l_w_3).wrapping_add(112), fp, fp.wrapping_add(48));
                            { let r_ = 512i32; let a_ = (l_w_3).wrapping_add(436); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                        F::Vec4_Copy(c, (l_w_3).wrapping_add(96), fp);
                    }
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_SetPos", &PROBE_N, &[], true); }
    r_
}

pub fn EftOrbTail_IsActive(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_IsActive", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftOrbTail)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = c.m.r32(sym::gEftOrbTail); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftOrbTail_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(436)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_IsActive", &PROBE_N, &[], true); }
    r_
}

pub fn EftOrbTail_Stub0(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_Stub0", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_Stub0", &PROBE_N, &[], true); }
    r_
}

pub fn EftOrbTail_Stub1(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_Stub1", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_Stub1", &PROBE_N, &[], true); }
    r_
}

pub fn EftOrbTail_Stub2(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_Stub2", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_Stub2", &PROBE_N, &[], true); }
    r_
}

pub fn EftOrbTail_Stub3(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_Stub3", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_Stub3", &PROBE_N, &[], true); }
    r_
}

pub fn EftOrbTail_Stub4(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_Stub4", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTail_Stub4", &PROBE_N, &[], true); }
    r_
}
