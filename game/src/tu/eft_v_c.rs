#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EftLinkMgr_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLinkMgr_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((67296u32) as i32)) }; let a_ = sym::gEftLink; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gEftLink); F::memset(c, a0_, 0i32, 67296u32) }; };
    { let v_ = F::BtlTask_CreateChildList(c, p_task_1, ((((6i32) as i64 as u64)) as u32), ((((((944u32) as i32)) as i64 as u64)) as u32)); let a_ = c.m.r32(sym::gEftLink); c.m.w32(a_, v_); };
    { let a0_ = (c.m.r32(sym::gEftLink)).wrapping_add(67232); F::Mtx_StoreIdentity(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftLinkMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftLinkMgr_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLinkMgr_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftLinkMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftLinkMgr_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLinkMgr_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftLinkMgr_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftLinkMgr_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLinkMgr_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftLink); F::BtlPool_Free(c, a0_, a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftLink; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftLinkMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_3: u32 = 0;
    let mut l_def_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_j_6: i32 = 0;
    let mut l_n_7: i32 = 0;
    let r_ = 'f: {
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        l_def_4 = c.m.r32(p_arg_2);
        { let _ = F::memset(c, l_w_3, 0i32, 944u32); };
        { let s_ = p_arg_2; let d_ = (l_w_3).wrapping_add(48); rt::memcpy(c, d_, s_, 80); };
        { let v_ = c.m.r32((p_arg_2).wrapping_add(8)); let a_ = ((l_w_3).wrapping_add(48)).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = c.m.r32(p_arg_2); let a_ = (l_w_3).wrapping_add(48); c.m.w32(a_, v_); };
        { let v_ = c.m.r32((p_arg_2).wrapping_add(4)); let a_ = ((l_w_3).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, v_); };
        { let a1_ = c.m.r32((p_arg_2).wrapping_add(8)); let a2_ = (c.m.r32((p_arg_2).wrapping_add(56)) as i32); let a3_ = (c.m.r32((p_arg_2).wrapping_add(60)) as i32); F::EftLink_SelectTex(c, l_w_3, a1_, a2_, a3_) };
        if (((((c.m.r32((l_def_4).wrapping_add(416)) as i32)) & (512i32))) != 0) {
            F::EftLink_InitGrow(c, l_w_3);
            { let r_ = 8192i32; let a_ = (l_w_3).wrapping_add(920); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        } else {
            { let v_ = F32(c.m.r32(((l_def_4).wrapping_add(376)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (l_w_3).wrapping_add(892); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f800000); let a_ = (l_w_3).wrapping_add(880); c.m.w32(a_, (v_).0); };
        }
        if (((((c.m.r32((l_def_4).wrapping_add(416)) as i32)) & (1024i32))) != 0) {
            F::EftLink_InitKeys(c, l_w_3);
            { let r_ = 16384i32; let a_ = (l_w_3).wrapping_add(920); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        } else {
            F::EftLink_SetKey(c, l_w_3, 2i32);
        }
        {
            rt::memset(c, fp, 0, 8);
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(4), (v_).0); }
            if (((((((((((c.m.r8((l_def_4).wrapping_add(412))) as i32)) >= (2i32)) as i32)) != 0) || (((((((c.m.r8((l_def_4).wrapping_add(413))) as i32)) >= (2i32)) as i32)) != 0)) as i32)) != 0) {
                { let v_ = rt::i2f((((c.m.r8((l_def_4).wrapping_add(412))) as i32)).wrapping_mul(((c.m.r8((l_def_4).wrapping_add(413))) as i32))); let a_ = (l_w_3).wrapping_add(596); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fdiv(F32(0x3f800000), rt::i2f(((c.m.r8((l_def_4).wrapping_add(412))) as i32))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fdiv(F32(0x3f800000), rt::i2f(((c.m.r8((l_def_4).wrapping_add(413))) as i32))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                l_n_7 = 0i32;
                {
                    l_j_6 = 0i32;
                    'l1: loop {
                        if !(((((l_j_6) < (((c.m.r8((l_def_4).wrapping_add(413))) as i32))) as i32)) != 0) { break 'l1; }
                        'c2: {
                            {
                                l_i_5 = 0i32;
                                'l3: loop {
                                    if !(((((l_i_5) < (((c.m.r8((l_def_4).wrapping_add(412))) as i32))) as i32)) != 0) { break 'l3; }
                                    'c4: {
                                        { let v_ = rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::i2f(l_i_5)); let a_ = (((l_w_3).wrapping_add(616)).wrapping_add((((l_n_7) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                        { let v_ = rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::i2f(l_j_6)); let a_ = (((l_w_3).wrapping_add(616)).wrapping_add((((l_n_7) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                        { let v_ = rt::fadd(rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::i2f(l_i_5)), F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (((l_w_3).wrapping_add(616)).wrapping_add((((l_n_7) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                        { let v_ = rt::fadd(rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::i2f(l_j_6)), F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = (((l_w_3).wrapping_add(616)).wrapping_add((((l_n_7) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                        { let o_ = l_n_7; let n_ = (o_).wrapping_add(1); l_n_7 = n_; };
                                    }
                                    { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
                                }
                            }
                        }
                        { let o_ = l_j_6; let n_ = (o_).wrapping_add(1); l_j_6 = n_; };
                    }
                }
                { let r_ = 256i32; let a_ = (l_w_3).wrapping_add(920); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
        if (((((c.m.r32((l_def_4).wrapping_add(416)) as i32)) & (2i32))) != 0) {
            { let r_ = 512i32; let a_ = (l_w_3).wrapping_add(920); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        { let r_ = 1i32; let a_ = (l_w_3).wrapping_add(920); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftLink_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_arg_3: u32 = 0;
    let mut l_fade_4: F32 = F32(0);
    let mut l_r_5: F32 = F32(0);
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_arg_3 = (l_w_2).wrapping_add(48);
    l_fade_4 = F32(0x3f800000);
    if ((((!(({ let a0_ = (c.m.r32((l_arg_3).wrapping_add(64)) as i32); let a1_ = (c.m.r32((l_w_2).wrapping_add(912)) as i32); F::BtlScene_IsEffectStopped(c, a0_, a1_) }) != 0)) as i32)) != 0) {
        if (((rt::fgt(F32(c.m.r32((l_w_2).wrapping_add(600))), F32(0x00000000)) as i32)) != 0) {
            { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(600); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        } else {
            if (((((((((c.m.r32((l_w_2).wrapping_add(920)) as i32)) & (32i32))) != 0) && ((((!(((((c.m.r32((l_w_2).wrapping_add(920)) as i32)) & (8i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
                l_r_5 = rt::fdiv(F32(c.m.r32((l_w_2).wrapping_add(608))), F32(c.m.r32((l_w_2).wrapping_add(612))));
                if (((rt::flt(l_r_5, F32(0x00000000)) as i32)) != 0) {
                    l_fade_4 = F32(0x00000000);
                } else {
                    if ((((!(((rt::fgt(l_r_5, F32(0x3f800000)) as i32)) != 0)) as i32)) != 0) {
                        l_fade_4 = l_r_5;
                    }
                }
                { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(608); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::flt(F32(c.m.r32((l_w_2).wrapping_add(608))), F32(0x00000000)) as i32)) != 0) {
                    { let r_ = (!(1i32)); let a_ = (l_w_2).wrapping_add(920); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                    { let r_ = 16i32; let a_ = (l_w_2).wrapping_add(920); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
            if (((((c.m.r32((l_w_2).wrapping_add(920)) as i32)) & (8192i32))) != 0) {
                F::EftLink_UpdateGrow(c, l_w_2);
            }
            if (((((c.m.r32((l_w_2).wrapping_add(920)) as i32)) & (16384i32))) != 0) {
                F::EftLink_UpdateKeys(c, l_w_2);
            }
            if ((((!(((((c.m.r32((l_w_2).wrapping_add(920)) as i32)) & (4i32))) != 0)) as i32)) != 0) {
                F::EftLink_Place(c, l_w_2);
            }
            F::EftLink_StepNodes(c, l_w_2, l_fade_4);
            { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(592); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((((c.m.r32((l_w_2).wrapping_add(920)) as i32)) & (16384i32))) != 0) {
                F::EftLink_InitKeys(c, l_w_2);
                if (((rt::fge(F32(c.m.r32((l_w_2).wrapping_add(592))), F32(c.m.r32((l_w_2).wrapping_add(560)))) as i32)) != 0) {
                    F::EftLink_SetKey(c, l_w_2, 2i32);
                    { let r_ = (!(16384i32)); let a_ = (l_w_2).wrapping_add(920); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
            if (((((((rt::fle(rt::fmul(F32(c.m.r32((l_arg_3).wrapping_add(52))), F32(0x41f00000)), F32(c.m.r32((l_w_2).wrapping_add(592)))) as i32)) != 0) && (((rt::fgt(F32(c.m.r32((l_arg_3).wrapping_add(52))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                if (((rt::fgt(F32(c.m.r32((l_w_2).wrapping_add(612))), F32(0x00000000)) as i32)) != 0) {
                    { let r_ = ((4096i32) | (32i32)); let a_ = (l_w_2).wrapping_add(920); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                } else {
                    { let r_ = 4i32; let a_ = (l_w_2).wrapping_add(920); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
            if (((((c.m.r32((l_w_2).wrapping_add(920)) as i32)) & (8i32))) != 0) {
                { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(604); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(604))), F32(0x00000000)) as i32)) != 0) {
                    { let r_ = (!(8i32)); let a_ = (l_w_2).wrapping_add(920); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                    if ((((!(((((c.m.r32((l_w_2).wrapping_add(920)) as i32)) & (32i32))) != 0)) as i32)) != 0) {
                        { let r_ = 4i32; let a_ = (l_w_2).wrapping_add(920); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                }
            }
            if (((((((((c.m.r32((l_w_2).wrapping_add(920)) as i32)) & (4i32))) != 0) && (((((c.m.r32((l_w_2).wrapping_add(924))) == (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
                { let r_ = (!(1i32)); let a_ = (l_w_2).wrapping_add(920); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = 2i32; let a_ = (l_w_2).wrapping_add(920); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
            if (((((((((c.m.r32((l_w_2).wrapping_add(920)) as i32)) & (16i32))) != 0) || (((((c.m.r32((l_w_2).wrapping_add(920)) as i32)) & (2i32))) != 0)) as i32)) != 0) {
                F::BtlTask_SetDead(c, p_task_1);
            }
        }
    }
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(920)) as i32)) & (16i32))) != 0)) as i32)) != 0) {
        if ((((!(((((c.m.r32((l_w_2).wrapping_add(920)) as i32)) & (2i32))) != 0)) as i32)) != 0) {
            F::EftLink_BuildTex(c, l_w_2);
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftLink_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftLink_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_w_2: u32 = 0;
    let mut l_def_3: u32 = 0;
    let mut l_n_4: u32 = 0;
    let mut l_size_5: F32 = F32(0);
    let mut l_rot_6: F32 = F32(0);
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_def_3 = c.m.r32((l_w_2).wrapping_add(48));
        rt::memset(c, fp, 0, 64);
        rt::memset(c, fp.wrapping_add(0), 0, 64);
        rt::memset(c, fp.wrapping_add(0).wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(0).wrapping_add(0), (v_).0); }
        rt::memset(c, fp.wrapping_add(64), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(64).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(80), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(80).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(80).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(80).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(80).wrapping_add(12), (v_).0); }
        { let a1_ = (c.m.r32(sym::gEftLink)).wrapping_add(67232); F::Mtx_Copy(c, fp, a1_) };
        { let a2_ = F32(c.m.r32((l_w_2).wrapping_add(904))); F::Mtx_RotateX(c, fp, fp, a2_) };
        { let a2_ = F32(c.m.r32((l_w_2).wrapping_add(908))); F::Mtx_RotateY(c, fp, fp, a2_) };
        F::Vu0Cur_Push(c);
        { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
        {
            l_n_4 = c.m.r32((l_w_2).wrapping_add(924));
            'l1: loop {
                if !(((((l_n_4) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((c.m.r32((l_n_4).wrapping_add(320)) as i32)) & (64i32))) != 0) {
                        if (((((c.m.r32((l_def_3).wrapping_add(416)) as i32)) & (16i32))) != 0) {
                            F::Vec4_Copy(c, fp.wrapping_add(64), (l_n_4).wrapping_add(80));
                            if ((((!(((((c.m.r32((l_def_3).wrapping_add(416)) as i32)) & (8192i32))) != 0)) as i32)) != 0) {
                                { let a2_ = rt::fmul(F32(c.m.r32(((l_w_2).wrapping_add(48)).wrapping_add(48))), F32(c.m.r32((l_w_2).wrapping_add(880)))); F::Vec3_Scale(c, fp.wrapping_add(64), fp.wrapping_add(64), a2_) };
                            }
                            F::Mtx_MulVec4(c, fp.wrapping_add(64), fp, fp.wrapping_add(64));
                            F::Vec3_Add(c, fp.wrapping_add(80), fp.wrapping_add(64), (l_n_4).wrapping_add(96));
                            l_size_5 = rt::fmul(rt::fmul(rt::fmul(rt::fmul(F32(c.m.r32((l_n_4).wrapping_add(224))), F32(c.m.r32(((l_w_2).wrapping_add(48)).wrapping_add(48)))), F32(c.m.r32((l_w_2).wrapping_add(880)))), F32(c.m.r32((l_n_4).wrapping_add(252)))), F32(0x3f000000));
                            l_rot_6 = { let a0_ = rt::fadd(F32(c.m.r32((l_n_4).wrapping_add(212))), F32(c.m.r32((l_n_4).wrapping_add(216)))); F::EftMath_WrapAngle(c, a0_) };
                            if (((((c.m.r32((l_def_3).wrapping_add(416)) as i32)) & (4i32))) != 0) {
                                { let a5_ = F32(c.m.r32((l_n_4).wrapping_add(176))); let a6_ = F32(c.m.r32(((l_n_4).wrapping_add(176)).wrapping_add(4))); let a7_ = F32(c.m.r32(((l_n_4).wrapping_add(192)).wrapping_add(8))); let a8_ = F32(c.m.r32(((l_n_4).wrapping_add(192)).wrapping_add(12))); let a10_ = ((c.m.r8((l_def_3).wrapping_add(415))) as i32); let a11_ = ((((c.m.r32((l_w_2).wrapping_add(920)) as i32)).wrapping_shr((9i32) as u32)) & (1i32)); let a12_ = { let a_ = (c.m.r32(((l_w_2).wrapping_add(48)).wrapping_add(8))).wrapping_add(((((c.m.r32(((l_w_2).wrapping_add(48)).wrapping_add(56)) as i32)).wrapping_shl((4i32) as u32)) as u32)); c.m.r64(a_) }; F::EftLink_DrawBillboardClipped(c, fp.wrapping_add(80), l_size_5, l_size_5, (l_n_4).wrapping_add(128), (l_n_4).wrapping_add(64), a5_, a6_, a7_, a8_, l_rot_6, a10_, a11_, a12_, F32(0x40000000)) };
                            } else {
                                { let a4_ = (rt::f2i(F32(c.m.r32((l_n_4).wrapping_add(64))))).wrapping_shl((4i32) as u32); let a5_ = (rt::f2i(F32(c.m.r32(((l_n_4).wrapping_add(64)).wrapping_add(4))))).wrapping_shl((4i32) as u32); let a6_ = F32(c.m.r32((l_n_4).wrapping_add(176))); let a7_ = F32(c.m.r32(((l_n_4).wrapping_add(176)).wrapping_add(4))); let a8_ = F32(c.m.r32(((l_n_4).wrapping_add(192)).wrapping_add(8))); let a9_ = F32(c.m.r32(((l_n_4).wrapping_add(192)).wrapping_add(12))); let a11_ = ((c.m.r8((l_def_3).wrapping_add(415))) as i32); let a12_ = ((((c.m.r32((l_w_2).wrapping_add(920)) as i32)).wrapping_shr((9i32) as u32)) & (1i32)); let a13_ = { let a_ = (c.m.r32(((l_w_2).wrapping_add(48)).wrapping_add(8))).wrapping_add(((((c.m.r32(((l_w_2).wrapping_add(48)).wrapping_add(56)) as i32)).wrapping_shl((4i32) as u32)) as u32)); c.m.r64(a_) }; F::EftLink_DrawBillboard(c, fp.wrapping_add(80), rt::fmul(l_size_5, F32(0x41800000)), rt::fmul(l_size_5, F32(0x41800000)), (l_n_4).wrapping_add(128), a4_, a5_, a6_, a7_, a8_, a9_, l_rot_6, a11_, a12_, a13_) };
                            }
                        } else {
                            if (((((c.m.r32((l_def_3).wrapping_add(416)) as i32)) & (4i32))) != 0) {
                                { let a4_ = ((c.m.r8((l_def_3).wrapping_add(415))) as i32); let a5_ = (c.m.r32(((l_w_2).wrapping_add(48)).wrapping_add(56)) as i32); let a6_ = ((((c.m.r32((l_w_2).wrapping_add(920)) as i32)).wrapping_shr((9i32) as u32)) & (1i32)); let a7_ = c.m.r32(((l_w_2).wrapping_add(48)).wrapping_add(8)); F::EftLink_DrawQuadClipped(c, l_n_4, (l_n_4).wrapping_add(176), (l_n_4).wrapping_add(192), (l_n_4).wrapping_add(128), a4_, a5_, a6_, a7_) };
                            } else {
                                { let a4_ = ((c.m.r8((l_def_3).wrapping_add(415))) as i32); let a5_ = (c.m.r32(((l_w_2).wrapping_add(48)).wrapping_add(56)) as i32); let a6_ = ((((c.m.r32((l_w_2).wrapping_add(920)) as i32)).wrapping_shr((9i32) as u32)) & (1i32)); let a7_ = c.m.r32(((l_w_2).wrapping_add(48)).wrapping_add(8)); F::EftLink_DrawQuad(c, l_n_4, (l_n_4).wrapping_add(176), (l_n_4).wrapping_add(192), (l_n_4).wrapping_add(128), a4_, a5_, a6_, a7_) };
                            }
                        }
                    }
                }
                l_n_4 = c.m.r32((l_n_4).wrapping_add(324));
            }
        }
        F::Vu0Cur_Pop(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftLink_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftLink_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_n_3: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    {
        l_n_3 = c.m.r32((l_w_2).wrapping_add(924));
        'l1: loop {
            if !(((((l_n_3) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = 0i32; let a_ = (l_n_3).wrapping_add(320); c.m.w32(a_, (v_) as u32); };
                F::EftLink_UnlinkNode(c, (l_w_2).wrapping_add(924), (l_w_2).wrapping_add(928), l_n_3);
            }
            l_n_3 = c.m.r32((l_n_3).wrapping_add(324));
        }
    }
    { let v_ = 0i32; let a_ = (l_w_2).wrapping_add(920); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftLink_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_Place(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_Place", &PROBE_N, &[(p_w_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_dir_2: u32 = 0;
    let mut l_arg_3: u32 = 0;
    let mut l_n_4: u32 = 0;
    let mut l_len_5: F32 = F32(0);
    let mut l_count_6: F32 = F32(0);
    let mut l_a_7: F32 = F32(0);
    let mut l_fi_8: F32 = F32(0);
    let mut l_i_9: i32 = 0;
    let r_ = 'f: {
        l_dir_2 = (p_w_1).wrapping_add(576);
        l_arg_3 = (p_w_1).wrapping_add(48);
        rt::memset(c, fp, 0, 16);
        rt::memset(c, fp.wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(16), 0, 16);
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(12), (v_).0); }
        l_count_6 = F32(0x00000000);
        F::Vec3_Sub(c, l_dir_2, ((p_w_1).wrapping_add(48)).wrapping_add(32), ((p_w_1).wrapping_add(48)).wrapping_add(16));
        l_len_5 = F::Vec3_Length(c, l_dir_2);
        if (((rt::fgt(l_len_5, l_count_6) as i32)) != 0) {
            l_count_6 = rt::fdiv(l_len_5, F32(c.m.r32((p_w_1).wrapping_add(892))));
            F::Vec3_Normalize(c, l_dir_2, l_dir_2);
            l_a_7 = (if (((rt::fgt(F32(c.m.r32(((p_w_1).wrapping_add(576)).wrapping_add(4))), F32(0x3f800000)) as i32)) != 0) { F32(0xbf800000) } else { (if (((rt::flt(F32(c.m.r32(((p_w_1).wrapping_add(576)).wrapping_add(4))), F32(0xbf800000)) as i32)) != 0) { F32(0x3f800000) } else { rt::fneg(F32(c.m.r32(((p_w_1).wrapping_add(576)).wrapping_add(4)))) }) });
            { let v_ = { let a0_ = F::Port_asinf(c, l_a_7); F::EftMath_WrapAngle(c, a0_) }; let a_ = (p_w_1).wrapping_add(904); c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = { let a0_ = F32(c.m.r32((p_w_1).wrapping_add(576))); let a1_ = F32(c.m.r32(((p_w_1).wrapping_add(576)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) }; F::EftMath_WrapAngle(c, a0_) }; let a_ = (p_w_1).wrapping_add(908); c.m.w32(a_, (v_).0); };
            l_n_4 = c.m.r32((p_w_1).wrapping_add(924));
            {
                l_i_9 = 0i32;
                'l1: loop {
                    if !(((((l_i_9) < (rt::f2i(l_count_6))) as i32)) != 0) { break 'l1; }
                    'c2: {
                        l_fi_8 = rt::i2f(l_i_9);
                        F::Vec3_ScaleAdd(c, fp, (p_w_1).wrapping_add(576), (l_arg_3).wrapping_add(16), rt::fmul(l_len_5, rt::fdiv(l_fi_8, l_count_6)));
                        { let a2_ = { let a0_ = F32(c.m.r32(((p_w_1).wrapping_add(328)).wrapping_add((((11i32) as u32)).wrapping_mul(8)))); let a1_ = F32(c.m.r32(((p_w_1).wrapping_add(328)).wrapping_add((((12i32) as u32)).wrapping_mul(8)))); F::Rand_FloatRange(c, a0_, a1_) }; F::Vec3_Scale(c, fp.wrapping_add(16), (p_w_1).wrapping_add(576), a2_) };
                        F::Vec3_Add(c, fp, fp, fp.wrapping_add(16));
                        if (((((l_n_4) == (((0i32) as u32))) as i32)) != 0) {
                            l_n_4 = F::EftLink_NewNode(c, p_w_1, fp, l_fi_8);
                            if (((((l_n_4) == (((0i32) as u32))) as i32)) != 0) {
                                break 'c2;
                            }
                        }
                        if ((((!(((((c.m.r32((l_n_4).wrapping_add(320)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
                            F::EftLink_InitNode(c, l_n_4, p_w_1);
                            F::Vec4_Copy(c, (l_n_4).wrapping_add(96), fp);
                            { let v_ = { let a0_ = rt::fmul(rt::fmul(F32(c.m.r32(((p_w_1).wrapping_add(328)).wrapping_add((((8i32) as u32)).wrapping_mul(8)))), F32(0x40c90fda)), l_fi_8); F::EftMath_WrapAngle(c, a0_) }; let a_ = (l_n_4).wrapping_add(216); c.m.w32(a_, (v_).0); };
                            { let r_ = 1i32; let a_ = (l_n_4).wrapping_add(320); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                        { let r_ = 1024i32; let a_ = (l_n_4).wrapping_add(320); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        l_n_4 = c.m.r32((l_n_4).wrapping_add(324));
                    }
                    { let o_ = l_i_9; let n_ = (o_).wrapping_add(1); l_i_9 = n_; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftLink_Place", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_InitGrow(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_InitGrow", &PROBE_N, &[(p_w_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_def_2: u32 = 0;
    let mut l_dur_3: F32 = F32(0);
    let r_ = 'f: {
        l_def_2 = c.m.r32((p_w_1).wrapping_add(48));
        l_dur_3 = F32(c.m.r32((l_def_2).wrapping_add(372)));
        { let v_ = F32(0x00000000); let a_ = (p_w_1).wrapping_add(872); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(l_dur_3, F32(0x41f00000)); let a_ = (p_w_1).wrapping_add(876); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((l_def_2).wrapping_add(388)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((l_def_2).wrapping_add(388)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(400)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((l_def_2).wrapping_add(388)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((l_def_2).wrapping_add(388)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(400)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((l_def_2).wrapping_add(388)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((l_def_2).wrapping_add(388)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(400)).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((p_w_1).wrapping_add(876))), F32(c.m.r32((l_def_2).wrapping_add(368))))); let a_ = ((p_w_1).wrapping_add(884)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((p_w_1).wrapping_add(876))), rt::fsub(F32(0x3f800000), F32(c.m.r32((l_def_2).wrapping_add(368)))))); let a_ = ((p_w_1).wrapping_add(884)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_w_1).wrapping_add(880); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32(((l_def_2).wrapping_add(376)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(376)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((p_w_1).wrapping_add(876))), F32(c.m.r32((l_def_2).wrapping_add(368))))); let a_ = ((p_w_1).wrapping_add(896)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32(((l_def_2).wrapping_add(376)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(376)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((p_w_1).wrapping_add(876))), rt::fsub(F32(0x3f800000), F32(c.m.r32((l_def_2).wrapping_add(368)))))); let a_ = ((p_w_1).wrapping_add(896)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_def_2).wrapping_add(376)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_w_1).wrapping_add(892); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftLink_InitGrow", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_UpdateGrow(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_UpdateGrow", &PROBE_N, &[(p_w_1) as u32], false); }
    let mut l_def_2: u32 = 0;
    let r_ = 'f: {
    l_def_2 = c.m.r32((p_w_1).wrapping_add(48));
    if (((rt::flt(rt::fdiv(F32(c.m.r32((p_w_1).wrapping_add(872))), F32(c.m.r32((p_w_1).wrapping_add(876)))), F32(c.m.r32((l_def_2).wrapping_add(368)))) as i32)) != 0) {
        { let r_ = F32(c.m.r32(((p_w_1).wrapping_add(884)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_w_1).wrapping_add(880); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32(((p_w_1).wrapping_add(896)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_w_1).wrapping_add(892); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    } else {
        { let r_ = F32(c.m.r32(((p_w_1).wrapping_add(884)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (p_w_1).wrapping_add(880); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32(((p_w_1).wrapping_add(896)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (p_w_1).wrapping_add(892); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
    { let r_ = F32(0x3f800000); let a_ = (p_w_1).wrapping_add(872); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    if (((rt::fle(F32(c.m.r32((p_w_1).wrapping_add(876))), F32(c.m.r32((p_w_1).wrapping_add(872)))) as i32)) != 0) {
        { let r_ = (!(8192i32)); let a_ = (p_w_1).wrapping_add(920); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftLink_UpdateGrow", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_InitKeys(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_InitKeys", &PROBE_N, &[(p_w_1) as u32], false); }
    let mut l_def_2: u32 = 0;
    let mut l_def2_3: u32 = 0;
    let mut l_k_4: i32 = 0;
    let r_ = 'f: {
    l_def_2 = c.m.r32((p_w_1).wrapping_add(48));
    l_def2_3 = c.m.r32(((p_w_1).wrapping_add(48)).wrapping_add(4));
    l_k_4 = 0i32;
    if (((rt::fle(F32(c.m.r32((p_w_1).wrapping_add(592))), F32(0x00000000)) as i32)) != 0) {
        l_k_4 = 1i32;
        { let v_ = rt::fmul(F32(c.m.r32((l_def_2).wrapping_add(348))), F32(0x41f00000)); let a_ = (p_w_1).wrapping_add(560); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32((p_w_1).wrapping_add(560))), F32(c.m.r32((l_def_2).wrapping_add(352)))); let a_ = (p_w_1).wrapping_add(564); c.m.w32(a_, (v_).0); };
    }
    if (((((((rt::fle(F32(c.m.r32((p_w_1).wrapping_add(564))), F32(c.m.r32((p_w_1).wrapping_add(592)))) as i32)) != 0) && ((((!(((((c.m.r32((p_w_1).wrapping_add(920)) as i32)) & (32768i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
        l_k_4 = 2i32;
        { let r_ = 32768i32; let a_ = (p_w_1).wrapping_add(920); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if (((((l_k_4) != (0i32)) as i32)) != 0) {
        { let v_ = rt::fsub(F32(c.m.r32((l_def_2).wrapping_add((((l_k_4) as u32)).wrapping_mul(4)))), F32(c.m.r32((l_def_2).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(328)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_2).wrapping_add(12)).wrapping_add((((l_k_4) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(12)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(328)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_2).wrapping_add(24)).wrapping_add((((l_k_4) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(24)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(328)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_2).wrapping_add(36)).wrapping_add((((l_k_4) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(36)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(328)).wrapping_add((((3i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_2).wrapping_add(48)).wrapping_add((((l_k_4) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(48)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(328)).wrapping_add((((4i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_2).wrapping_add(60)).wrapping_add((((l_k_4) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(60)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(328)).wrapping_add((((5i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_2).wrapping_add(72)).wrapping_add((((l_k_4) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(72)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(328)).wrapping_add((((6i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_2).wrapping_add(84)).wrapping_add((((l_k_4) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(84)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(328)).wrapping_add((((7i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_2).wrapping_add(96)).wrapping_add((((l_k_4) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(96)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(328)).wrapping_add((((8i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_2).wrapping_add(108)).wrapping_add((((l_k_4) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(108)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(328)).wrapping_add((((9i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_2).wrapping_add(120)).wrapping_add((((l_k_4) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(120)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(328)).wrapping_add((((10i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_2).wrapping_add(132)).wrapping_add((((l_k_4) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(132)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(328)).wrapping_add((((11i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_2).wrapping_add(144)).wrapping_add((((l_k_4) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(144)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(328)).wrapping_add((((12i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_2).wrapping_add(156)).wrapping_add((((l_k_4) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(156)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(328)).wrapping_add((((13i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_2).wrapping_add(168)).wrapping_add((((l_k_4) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(168)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(328)).wrapping_add((((14i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_2).wrapping_add(180)).wrapping_add((((l_k_4) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(180)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(328)).wrapping_add((((15i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_2).wrapping_add(192)).wrapping_add((((l_k_4) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_2).wrapping_add(192)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(456)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_2).wrapping_add(192)).wrapping_add((((l_k_4) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_2).wrapping_add(192)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(456)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_2).wrapping_add(192)).wrapping_add((((l_k_4) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_2).wrapping_add(192)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(456)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_2).wrapping_add(228)).wrapping_add((((l_k_4) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_2).wrapping_add(228)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(480)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_2).wrapping_add(228)).wrapping_add((((l_k_4) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_2).wrapping_add(228)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(480)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_2).wrapping_add(228)).wrapping_add((((l_k_4) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_2).wrapping_add(228)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(480)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_2).wrapping_add(264)).wrapping_add((((l_k_4) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(264)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(504)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_2).wrapping_add(276)).wrapping_add((((l_k_4) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(276)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(504)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_2).wrapping_add(288)).wrapping_add((((l_k_4) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(288)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(504)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_2).wrapping_add(300)).wrapping_add((((l_k_4) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(300)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(504)).wrapping_add((((3i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_2).wrapping_add(312)).wrapping_add((((l_k_4) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_2).wrapping_add(312)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((p_w_1).wrapping_add(536)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_2).wrapping_add(312)).wrapping_add((((l_k_4) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_2).wrapping_add(312)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(536)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_2).wrapping_add(336)).wrapping_add((((l_k_4) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_2).wrapping_add(336)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((p_w_1).wrapping_add(552)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def2_3).wrapping_add(192)).wrapping_add((((l_k_4) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def2_3).wrapping_add(192)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(256)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def2_3).wrapping_add(192)).wrapping_add((((l_k_4) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def2_3).wrapping_add(192)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = ((((p_w_1).wrapping_add(256)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def2_3).wrapping_add(216)).wrapping_add((((l_k_4) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def2_3).wrapping_add(216)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(256)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def2_3).wrapping_add(216)).wrapping_add((((l_k_4) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def2_3).wrapping_add(216)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = ((((p_w_1).wrapping_add(256)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def2_3).wrapping_add(240)).wrapping_add((((l_k_4) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def2_3).wrapping_add(240)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(256)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def2_3).wrapping_add(240)).wrapping_add((((l_k_4) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def2_3).wrapping_add(240)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = ((((p_w_1).wrapping_add(256)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def2_3).wrapping_add(264)).wrapping_add((((l_k_4) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def2_3).wrapping_add(264)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(304)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def2_3).wrapping_add(276)).wrapping_add((((l_k_4) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def2_3).wrapping_add(276)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(304)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def2_3).wrapping_add(288)).wrapping_add((((l_k_4) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def2_3).wrapping_add(288)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(304)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        F::Vec4_Sub(c, (((p_w_1).wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(32))).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (l_def2_3).wrapping_add((((l_k_4) as u32)).wrapping_mul(16)), (l_def2_3).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(16)));
        F::Vec4_Sub(c, (((p_w_1).wrapping_add(128)).wrapping_add((((2i32) as u32)).wrapping_mul(32))).wrapping_add((((1i32) as u32)).wrapping_mul(16)), ((l_def2_3).wrapping_add(96)).wrapping_add((((l_k_4) as u32)).wrapping_mul(16)), ((l_def2_3).wrapping_add(96)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(16)));
        F::Vec4_Sub(c, (((p_w_1).wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(32))).wrapping_add((((1i32) as u32)).wrapping_mul(16)), ((l_def2_3).wrapping_add(48)).wrapping_add((((l_k_4) as u32)).wrapping_mul(16)), ((l_def2_3).wrapping_add(48)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(16)));
        F::Vec4_Sub(c, (((p_w_1).wrapping_add(128)).wrapping_add((((3i32) as u32)).wrapping_mul(32))).wrapping_add((((1i32) as u32)).wrapping_mul(16)), ((l_def2_3).wrapping_add(144)).wrapping_add((((l_k_4) as u32)).wrapping_mul(16)), ((l_def2_3).wrapping_add(144)).wrapping_add(((((l_k_4).wrapping_sub(1i32)) as u32)).wrapping_mul(16)));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftLink_InitKeys", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_UpdateKeys(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_UpdateKeys", &PROBE_N, &[(p_w_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_k_2: i32 = 0;
    let mut l_def_3: u32 = 0;
    let mut l_def2_4: u32 = 0;
    let mut l_t_5: F32 = F32(0);
    let mut l_p_6: u32 = 0;
    let r_ = 'f: {
        l_def_3 = c.m.r32((p_w_1).wrapping_add(48));
        l_def2_4 = c.m.r32(((p_w_1).wrapping_add(48)).wrapping_add(4));
        l_p_6 = fp;
        { let _ = F::memset(c, l_p_6, 0i32, 16u32); };
        l_k_2 = 0i32;
        if ((((!(((((c.m.r32((p_w_1).wrapping_add(920)) as i32)) & (32768i32))) != 0)) as i32)) != 0) {
            l_t_5 = rt::fdiv(F32(c.m.r32((p_w_1).wrapping_add(592))), F32(c.m.r32((p_w_1).wrapping_add(564))));
        } else {
            l_k_2 = 1i32;
            l_t_5 = rt::fdiv(rt::fsub(F32(c.m.r32((p_w_1).wrapping_add(592))), F32(c.m.r32((p_w_1).wrapping_add(564)))), rt::fsub(F32(c.m.r32((p_w_1).wrapping_add(560))), F32(c.m.r32((p_w_1).wrapping_add(564)))));
        }
        { let v_ = rt::fadd(F32(c.m.r32((l_def_3).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_w_1).wrapping_add(328)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add(4))), l_t_5)); let a_ = ((p_w_1).wrapping_add(328)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(12)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_w_1).wrapping_add(328)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add(4))), l_t_5)); let a_ = ((p_w_1).wrapping_add(328)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(24)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_w_1).wrapping_add(328)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add(4))), l_t_5)); let a_ = ((p_w_1).wrapping_add(328)).wrapping_add((((2i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(36)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_w_1).wrapping_add(328)).wrapping_add((((3i32) as u32)).wrapping_mul(8))).wrapping_add(4))), l_t_5)); let a_ = ((p_w_1).wrapping_add(328)).wrapping_add((((3i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(48)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_w_1).wrapping_add(328)).wrapping_add((((4i32) as u32)).wrapping_mul(8))).wrapping_add(4))), l_t_5)); let a_ = ((p_w_1).wrapping_add(328)).wrapping_add((((4i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(60)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_w_1).wrapping_add(328)).wrapping_add((((5i32) as u32)).wrapping_mul(8))).wrapping_add(4))), l_t_5)); let a_ = ((p_w_1).wrapping_add(328)).wrapping_add((((5i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(72)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_w_1).wrapping_add(328)).wrapping_add((((6i32) as u32)).wrapping_mul(8))).wrapping_add(4))), l_t_5)); let a_ = ((p_w_1).wrapping_add(328)).wrapping_add((((6i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(84)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_w_1).wrapping_add(328)).wrapping_add((((7i32) as u32)).wrapping_mul(8))).wrapping_add(4))), l_t_5)); let a_ = ((p_w_1).wrapping_add(328)).wrapping_add((((7i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(96)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_w_1).wrapping_add(328)).wrapping_add((((8i32) as u32)).wrapping_mul(8))).wrapping_add(4))), l_t_5)); let a_ = ((p_w_1).wrapping_add(328)).wrapping_add((((8i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(108)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_w_1).wrapping_add(328)).wrapping_add((((9i32) as u32)).wrapping_mul(8))).wrapping_add(4))), l_t_5)); let a_ = ((p_w_1).wrapping_add(328)).wrapping_add((((9i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(120)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_w_1).wrapping_add(328)).wrapping_add((((10i32) as u32)).wrapping_mul(8))).wrapping_add(4))), l_t_5)); let a_ = ((p_w_1).wrapping_add(328)).wrapping_add((((10i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(132)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_w_1).wrapping_add(328)).wrapping_add((((11i32) as u32)).wrapping_mul(8))).wrapping_add(4))), l_t_5)); let a_ = ((p_w_1).wrapping_add(328)).wrapping_add((((11i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(144)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_w_1).wrapping_add(328)).wrapping_add((((12i32) as u32)).wrapping_mul(8))).wrapping_add(4))), l_t_5)); let a_ = ((p_w_1).wrapping_add(328)).wrapping_add((((12i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(156)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_w_1).wrapping_add(328)).wrapping_add((((13i32) as u32)).wrapping_mul(8))).wrapping_add(4))), l_t_5)); let a_ = ((p_w_1).wrapping_add(328)).wrapping_add((((13i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(168)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_w_1).wrapping_add(328)).wrapping_add((((14i32) as u32)).wrapping_mul(8))).wrapping_add(4))), l_t_5)); let a_ = ((p_w_1).wrapping_add(328)).wrapping_add((((14i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(180)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_w_1).wrapping_add(328)).wrapping_add((((15i32) as u32)).wrapping_mul(8))).wrapping_add(4))), l_t_5)); let a_ = ((p_w_1).wrapping_add(328)).wrapping_add((((15i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_3).wrapping_add(192)).wrapping_add((((l_k_2) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_w_1).wrapping_add(456)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (((p_w_1).wrapping_add(456)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_3).wrapping_add(192)).wrapping_add((((l_k_2) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_w_1).wrapping_add(456)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (((p_w_1).wrapping_add(456)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_3).wrapping_add(192)).wrapping_add((((l_k_2) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_w_1).wrapping_add(456)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (((p_w_1).wrapping_add(456)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_3).wrapping_add(228)).wrapping_add((((l_k_2) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_w_1).wrapping_add(480)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (((p_w_1).wrapping_add(480)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_3).wrapping_add(228)).wrapping_add((((l_k_2) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_w_1).wrapping_add(480)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (((p_w_1).wrapping_add(480)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_3).wrapping_add(228)).wrapping_add((((l_k_2) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_w_1).wrapping_add(480)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (((p_w_1).wrapping_add(480)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(264)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_w_1).wrapping_add(504)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add(4))), l_t_5)); let a_ = ((p_w_1).wrapping_add(504)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(276)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_w_1).wrapping_add(504)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add(4))), l_t_5)); let a_ = ((p_w_1).wrapping_add(504)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(288)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_w_1).wrapping_add(504)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add(4))), l_t_5)); let a_ = ((p_w_1).wrapping_add(504)).wrapping_add((((2i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(300)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_w_1).wrapping_add(504)).wrapping_add((((3i32) as u32)).wrapping_mul(8))).wrapping_add(4))), l_t_5)); let a_ = ((p_w_1).wrapping_add(504)).wrapping_add((((3i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_3).wrapping_add(312)).wrapping_add((((l_k_2) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_w_1).wrapping_add(536)).wrapping_add((((1i32) as u32)).wrapping_mul(8)))), l_t_5)); let a_ = ((p_w_1).wrapping_add(536)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_3).wrapping_add(312)).wrapping_add((((l_k_2) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_w_1).wrapping_add(536)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add(4))), l_t_5)); let a_ = (((p_w_1).wrapping_add(536)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(336)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_w_1).wrapping_add(552)).wrapping_add(4))), l_t_5)); let a_ = (p_w_1).wrapping_add(552); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def2_4).wrapping_add(192)).wrapping_add((((l_k_2) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_w_1).wrapping_add(256)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(8)))), l_t_5)); let a_ = (((p_w_1).wrapping_add(256)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def2_4).wrapping_add(192)).wrapping_add((((l_k_2) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((((p_w_1).wrapping_add(256)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add(4))), l_t_5)); let a_ = ((((p_w_1).wrapping_add(256)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def2_4).wrapping_add(216)).wrapping_add((((l_k_2) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_w_1).wrapping_add(256)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(8)))), l_t_5)); let a_ = (((p_w_1).wrapping_add(256)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def2_4).wrapping_add(216)).wrapping_add((((l_k_2) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((((p_w_1).wrapping_add(256)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add(4))), l_t_5)); let a_ = ((((p_w_1).wrapping_add(256)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def2_4).wrapping_add(240)).wrapping_add((((l_k_2) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_w_1).wrapping_add(256)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(8)))), l_t_5)); let a_ = (((p_w_1).wrapping_add(256)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def2_4).wrapping_add(240)).wrapping_add((((l_k_2) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((((p_w_1).wrapping_add(256)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add(4))), l_t_5)); let a_ = ((((p_w_1).wrapping_add(256)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def2_4).wrapping_add(264)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_w_1).wrapping_add(304)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add(4))), l_t_5)); let a_ = ((p_w_1).wrapping_add(304)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def2_4).wrapping_add(276)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_w_1).wrapping_add(304)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add(4))), l_t_5)); let a_ = ((p_w_1).wrapping_add(304)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def2_4).wrapping_add(288)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_w_1).wrapping_add(304)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add(4))), l_t_5)); let a_ = ((p_w_1).wrapping_add(304)).wrapping_add((((2i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
        F::Vec4_Scale(c, l_p_6, (((p_w_1).wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(32))).wrapping_add((((1i32) as u32)).wrapping_mul(16)), l_t_5);
        F::Vec4_Add(c, (((p_w_1).wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(32))).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (l_def2_4).wrapping_add((((l_k_2) as u32)).wrapping_mul(16)), l_p_6);
        F::Vec4_Scale(c, l_p_6, (((p_w_1).wrapping_add(128)).wrapping_add((((2i32) as u32)).wrapping_mul(32))).wrapping_add((((1i32) as u32)).wrapping_mul(16)), l_t_5);
        F::Vec4_Add(c, (((p_w_1).wrapping_add(128)).wrapping_add((((2i32) as u32)).wrapping_mul(32))).wrapping_add((((0i32) as u32)).wrapping_mul(16)), ((l_def2_4).wrapping_add(96)).wrapping_add((((l_k_2) as u32)).wrapping_mul(16)), l_p_6);
        F::Vec4_Scale(c, l_p_6, (((p_w_1).wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(32))).wrapping_add((((1i32) as u32)).wrapping_mul(16)), l_t_5);
        F::Vec4_Add(c, (((p_w_1).wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(32))).wrapping_add((((0i32) as u32)).wrapping_mul(16)), ((l_def2_4).wrapping_add(48)).wrapping_add((((l_k_2) as u32)).wrapping_mul(16)), l_p_6);
        F::Vec4_Scale(c, l_p_6, (((p_w_1).wrapping_add(128)).wrapping_add((((3i32) as u32)).wrapping_mul(32))).wrapping_add((((1i32) as u32)).wrapping_mul(16)), l_t_5);
        F::Vec4_Add(c, (((p_w_1).wrapping_add(128)).wrapping_add((((3i32) as u32)).wrapping_mul(32))).wrapping_add((((0i32) as u32)).wrapping_mul(16)), ((l_def2_4).wrapping_add(144)).wrapping_add((((l_k_2) as u32)).wrapping_mul(16)), l_p_6);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftLink_UpdateKeys", &PROBE_N, &[], true); }
    r_
}
