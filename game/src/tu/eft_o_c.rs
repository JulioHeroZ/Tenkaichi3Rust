#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EftBodyFx_UpdateParts(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32, mut p_set_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBodyFx_UpdateParts", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32, (p_set_3) as u32], false); }
    let mut l_w_4: u32 = 0;
    let mut l_g_5: i32 = 0;
    let mut l_p_6: i32 = 0;
    let mut l_gd_7: u32 = 0;
    let mut l_f_8: i32 = 0;
    let r_ = 'f: {
    l_w_4 = c.m.r32((p_task_2).wrapping_add(56));
    {
        l_g_5 = 0i32;
        'l1: loop {
            if !(((((l_g_5) < (19i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((({ let a_ = c.m.r32(p_set_3); c.m.r32(a_) }) & ((((1i32).wrapping_shl((l_g_5) as u32)) as u32)))) != 0) {
                    l_gd_7 = c.m.r32(((p_set_3).wrapping_add(12)).wrapping_add((((l_g_5) as u32)).wrapping_mul(16)));
                    {
                        l_p_6 = 0i32;
                        'l3: loop {
                            if !(((((l_p_6) < (((c.m.r8((l_gd_7).wrapping_add(1))) as i32))) as i32)) != 0) { break 'l3; }
                            'c4: {
                                l_f_8 = { let a5_ = (((c.m.r32((l_w_4).wrapping_add(1360)) as i32)) & (2i32)); let a7_ = (c.m.r32((l_w_4).wrapping_add(1368)) as i32); F::EftEmit_GetFlagsFromMask(c, p_set_3, (l_w_4).wrapping_add(48), p_objId_1, l_g_5, l_p_6, a5_, 0i32, a7_) };
                                if (((((l_f_8) != (0i32)) as i32)) != 0) {
                                    { let a11_ = F32(c.m.r32((l_w_4).wrapping_add(1364))); F::EftEmit_Spawn(c, p_set_3, (l_w_4).wrapping_add(48), (l_w_4).wrapping_add(768), l_w_4, (l_w_4).wrapping_add(16), p_objId_1, 0i32, 4i32, l_g_5, l_p_6, l_f_8, a11_) };
                                }
                            }
                            { let o_ = l_p_6; let n_ = (o_).wrapping_add(1); l_p_6 = n_; };
                        }
                    }
                }
            }
            { let o_ = l_g_5; let n_ = (o_).wrapping_add(1); l_g_5 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBodyFx_UpdateParts", &PROBE_N, &[], true); }
    r_
}

pub fn EftBodyFx_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBodyFx_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_3: u32 = 0;
    let mut l_dir_4: u32 = 0;
    let r_ = 'f: {
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        l_dir_4 = (l_w_3).wrapping_add(16);
        { let _ = F::memset(c, l_w_3, 0i32, 1392u32); };
        { let s_ = p_arg_2; let d_ = (l_w_3).wrapping_add(32); rt::memcpy(c, d_, s_, 12); };
        { let r_ = 1i32; let a_ = (l_w_3).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let v_ = { let a0_ = (c.m.r32(p_arg_2) as i32); F::BtlScene_GetCharScale(c, a0_) }; let a_ = (l_w_3).wrapping_add(1364); c.m.w32(a_, (v_).0); };
        if (((rt::flt(F32(c.m.r32((l_w_3).wrapping_add(1364))), F32(0x3f000000)) as i32)) != 0) {
            { let v_ = F32(0x3f000000); let a_ = (l_w_3).wrapping_add(1364); c.m.w32(a_, (v_).0); };
        }
        { let a0_ = (c.m.r32(p_arg_2) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, l_w_3) };
        { let a0_ = (c.m.r32(p_arg_2) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, l_dir_4) };
        { let a0_ = (c.m.r32(p_arg_2) as i32); F::BtlCharApi_GetNodePos(c, a0_, 46i32, fp) };
        F::Vec3_Sub(c, l_dir_4, fp, l_dir_4);
        { let v_ = F32(0x3f800000); let a_ = ((l_w_3).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec3_Normalize(c, l_dir_4, l_dir_4);
        { let v_ = (c.m.r32(sym::gEftBodyFx)).wrapping_add(((((c.m.r32((p_arg_2).wrapping_add(4)) as i32)) as u32)).wrapping_mul(800)); let a_ = (l_w_3).wrapping_add(44); c.m.w32(a_, v_); };
        { let a0_ = c.m.r32((l_w_3).wrapping_add(44)); F::EftEmit_InitState(c, a0_, (l_w_3).wrapping_add(48)) };
        { let r_ = 1i32; let a_ = (l_w_3).wrapping_add(1368); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        F::EftEmit_SetNode(c, (l_w_3).wrapping_add(768), 0i32, 3i32, ((0i32) as u32));
        { let r_ = 8i32; let a_ = (l_w_3).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let v_ = rt::i2f({ let a0_ = c.m.r32((l_w_3).wrapping_add(44)); F::EftEmit_GetEndFrames(c, a0_) }); let a_ = (l_w_3).wrapping_add(1376); c.m.w32(a_, (v_).0); };
        { let a1_ = (if ((((((c.m.r32(p_arg_2) as i32)) == (0i32)) as i32)) != 0) { 2048i32 } else { 4096i32 }); F::BtlTask_SetOwnerTag(c, p_task_1, a1_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBodyFx_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftBodyFx_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBodyFx_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_arg_3: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_arg_3 = (l_w_2).wrapping_add(32);
    { let a0_ = c.m.r32((l_w_2).wrapping_add(44)); F::EftEmit_TermState(c, a0_, (l_w_2).wrapping_add(48)) };
    { let v_ = 0i32; let a_ = (l_w_2).wrapping_add(1360); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gEftBodyFx)).wrapping_add(800)).wrapping_add(((((c.m.r32(l_arg_3) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBodyFx_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftBodyFx_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBodyFx_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_2: u32 = 0;
    let mut l_arg_3: u32 = 0;
    let mut l_dir_4: u32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_arg_3 = (l_w_2).wrapping_add(32);
        l_dir_4 = (l_w_2).wrapping_add(16);
        if ((((!(({ let a0_ = (c.m.r32(l_arg_3) as i32); F::BtlScene_IsEffectStopped(c, a0_, 4i32) }) != 0)) as i32)) != 0) {
            { let a0_ = (c.m.r32(l_arg_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, l_w_2) };
            { let a0_ = (c.m.r32(l_arg_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, l_dir_4) };
            { let a0_ = (c.m.r32(l_arg_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 46i32, fp) };
            F::Vec3_Sub(c, l_dir_4, fp, l_dir_4);
            { let v_ = F32(0x3f800000); let a_ = ((l_w_2).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            F::Vec3_Normalize(c, l_dir_4, l_dir_4);
            { let a0_ = (c.m.r32(l_arg_3) as i32); F::EftEmit_RefreshFixedNodes(c, a0_, (l_w_2).wrapping_add(768)) };
            { let a0_ = (c.m.r32(l_arg_3) as i32); let a2_ = c.m.r32((l_w_2).wrapping_add(44)); F::EftBodyFx_UpdateParts(c, a0_, p_task_1, a2_) };
            if (((((c.m.r32((l_w_2).wrapping_add(1360)) as i32)) & (2i32))) != 0) {
                { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(1372); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(1376))), F32(c.m.r32((l_w_2).wrapping_add(1372)))) as i32)) != 0) {
                    { let r_ = 4i32; let a_ = (l_w_2).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
        }
        if (((((c.m.r32((l_w_2).wrapping_add(1360)) as i32)) & (4i32))) != 0) {
            F::BtlTask_SetDead(c, p_task_1);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBodyFx_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftBodyFx_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBodyFx_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(1360)) as i32)) & (32i32))) != 0)) as i32)) != 0) {
        { let r_ = 32i32; let a_ = (l_w_2).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let a0_ = c.m.r32((l_w_2).wrapping_add(44)); F::EftEmit_KillAll(c, a0_, (l_w_2).wrapping_add(48)) };
    }
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBodyFx_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftBodyFx_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBodyFx_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((c.m.r32((l_w_2).wrapping_add(1360)) as i32)) & (16i32))) != 0) {
        if ((((!(({ let a0_ = c.m.r32((l_w_2).wrapping_add(44)); F::EftEmit_UpdateAlive(c, a0_, (l_w_2).wrapping_add(48)) }) != 0)) as i32)) != 0) {
            { let r_ = (!(8i32)); let a_ = (l_w_2).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        } else {
            { let r_ = 8i32; let a_ = (l_w_2).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        { let v_ = 0i32; let a_ = (l_w_2).wrapping_add(1368); c.m.w32(a_, (v_) as u32); };
    }
    { let r_ = 16i32; let a_ = (l_w_2).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBodyFx_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftBodyFx_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBodyFx_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBodyFx_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftBodyFxMgr_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBodyFxMgr_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        { let v_ = { let a0_ = ((((F::BtlPool_GetCurrent(c)) as i64 as u64)) as u32); F::BtlPool_Alloc(c, ((((a0_) as u64)) as i32), ((808u32) as i32)) }; let a_ = sym::gEftBodyFx; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gEftBodyFx); F::memset(c, a0_, 0i32, 808u32) }; };
        { let v_ = F::BtlTask_CreateChildList(c, p_task_1, ((((3i32) as i64 as u64)) as u32), ((((((1392u32) as i32)) as i64 as u64)) as u32)); let a_ = sym::gEftBodyFxList; c.m.w32(a_, v_); };
        { let v_ = 575i32; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (1i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a1_ = (c.m.r32(sym::gEftBodyFx)).wrapping_add((((l_i_2) as u32)).wrapping_mul(800)); let a5_ = (c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32); F::EftEmit_LoadSet(c, ((0i32) as u32), a1_, ((0i32) as u32), ((0i32) as u32), 1i32, a5_) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBodyFxMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftBodyFxMgr_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBodyFxMgr_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = (c.m.r32(sym::gEftBodyFx)).wrapping_add((((l_i_1) as u32)).wrapping_mul(800)); F::EftEmit_FreeSet(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    { let a0_ = ((((F::BtlPool_GetCurrent(c)) as i64 as u64)) as u32); let a1_ = c.m.r32(sym::gEftBodyFx); F::BtlPool_Free(c, ((((a0_) as u64)) as i32), a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftBodyFx; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBodyFxMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftBodyFxMgr_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBodyFxMgr_Update", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = (c.m.r32(sym::gEftBodyFx)).wrapping_add((((l_i_1) as u32)).wrapping_mul(800)); F::EftEmit_BeginFrame(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBodyFxMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftBodyFx_Start(c: &mut Ctx, p_arg_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBodyFx_Start", &PROBE_N, &[(p_arg_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    rt::memcpy(c, fp, p_arg_1, 12);
    let r_ = 'f: {
        if ((((({ let a_ = ((c.m.r32(sym::gEftBodyFx)).wrapping_add(800)).wrapping_add(((((c.m.r32(fp) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        { let v_ = { let a0_ = c.m.r32(sym::gEftBodyFxList); F::BtlTaskList_AddTail(c, a0_, sym::gEftBodyFxClass, fp) }; let a_ = ((c.m.r32(sym::gEftBodyFx)).wrapping_add(800)).wrapping_add(((((c.m.r32(fp) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        break 'f ((({ let a_ = ((c.m.r32(sym::gEftBodyFx)).wrapping_add(800)).wrapping_add(((((c.m.r32(fp) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBodyFx_Start", &PROBE_N, &[], true); }
    r_
}

pub fn EftBodyFx_Stop(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBodyFx_Stop", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_t_2: u32 = 0;
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    l_t_2 = { let a_ = ((c.m.r32(sym::gEftBodyFx)).wrapping_add(800)).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    if (((((l_t_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((l_t_2).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(1360)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let r_ = 2i32; let a_ = (l_w_3).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBodyFx_Stop", &PROBE_N, &[], true); }
    r_
}

pub fn EftDiscMgr_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDiscMgr_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = { let a0_ = ((((F::BtlPool_GetCurrent(c)) as i64 as u64)) as u32); F::BtlPool_Alloc(c, ((((a0_) as u64)) as i32), ((4u32) as i32)) }; let a_ = sym::gEftDisc; c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = c.m.r32(sym::gEftDisc); c.m.w32(a_, v_); };
    { let v_ = { let a0_ = ((((F::BtlPool_GetCurrent(c)) as i64 as u64)) as u32); F::BtlPool_Alloc(c, ((((a0_) as u64)) as i32), 1632i32) }; let a_ = c.m.r32(sym::gEftDisc); c.m.w32(a_, v_); };
    { let v_ = F::BtlScene_GetCommonEntry(c, 568i32); let a_ = ({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }).wrapping_add(1344); c.m.w32(a_, v_); };
    { let v_ = F::BtlScene_GetCommonEntry(c, 570i32); let a_ = ({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }).wrapping_add(1348); c.m.w32(a_, v_); };
    { let v_ = F::BtlScene_GetCommonEntry(c, 569i32); let a_ = ({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }).wrapping_add(1352); c.m.w32(a_, v_); };
    { let v_ = F::BtlScene_GetCommonEntry(c, 571i32); let a_ = ({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }).wrapping_add(1356); c.m.w32(a_, v_); };
    { let a0_ = { let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }; let a1_ = { let a_ = ({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }).wrapping_add(1344); c.m.r32(a_) }; F::EftTexSet_Load32(c, a0_, a1_) };
    { let a0_ = ({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }).wrapping_add(520); let a1_ = { let a_ = ({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }).wrapping_add(1348); c.m.r32(a_) }; F::EftTexSet_Load32(c, a0_, a1_) };
    { let a0_ = ({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }).wrapping_add(1056); let a1_ = { let a_ = ({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }).wrapping_add(1352); c.m.r32(a_) }; F::EftMesh_Init(c, a0_, a1_) };
    { let a0_ = ({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }).wrapping_add(1200); let a1_ = { let a_ = ({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }).wrapping_add(1356); c.m.r32(a_) }; F::EftMesh_Init(c, a0_, a1_) };
    { let a0_ = ({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }).wrapping_add(1056); let a1_ = { let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }; F::EftMesh_SetTex(c, a0_, a1_) };
    { let a0_ = ({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }).wrapping_add(1200); let a1_ = ({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }).wrapping_add(520); F::EftMesh_SetTex(c, a0_, a1_) };
    { let a0_ = ({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }).wrapping_add(1056); F::EftMesh_SetTexBase(c, a0_, 0i32) };
    { let a0_ = ({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }).wrapping_add(1200); F::EftMesh_SetTexBase(c, a0_, 0i32) };
    { let v_ = 0i32; let a_ = ({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }).wrapping_add(1040); c.m.w32(a_, (v_) as u32); };
    { let v_ = F::BtlTask_CreateChildList(c, p_task_1, ((((20i32) as i64 as u64)) as u32), ((((((1216u32) as i32)) as i64 as u64)) as u32)); let a_ = ({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }).wrapping_add(1360); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDiscMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftDiscMgr_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDiscMgr_Update", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = ({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }).wrapping_add(516); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }).wrapping_add(520)).wrapping_add(516); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDiscMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftDiscMgr_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDiscMgr_Reset", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDiscMgr_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftDiscMgr_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDiscMgr_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = ((((F::BtlPool_GetCurrent(c)) as i64 as u64)) as u32); let a1_ = { let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }; F::BtlPool_Free(c, ((((a0_) as u64)) as i32), a1_) };
    { let a0_ = ((((F::BtlPool_GetCurrent(c)) as i64 as u64)) as u32); let a1_ = c.m.r32(sym::gEftDisc); F::BtlPool_Free(c, ((((a0_) as u64)) as i32), a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftDisc; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDiscMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftDisc_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let mut l_model_4: u32 = 0;
    let mut l_life_5: F32 = F32(0);
    let mut l_roll_6: F32 = F32(0);
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let _ = F::memset(c, l_w_3, 0i32, 1216u32); };
    { let s_ = p_arg_2; let d_ = (l_w_3).wrapping_add(48); rt::memcpy(c, d_, s_, 96); };
    { let a0_ = ((c.m.r8((p_arg_2).wrapping_add(86))) as i32); let a1_ = (if (((c.m.r32((p_arg_2).wrapping_add(92)) as i32)) != 0) { 21i32 } else { 35i32 }); F::BtlCharApi_GetNodePos(c, a0_, a1_, (p_arg_2).wrapping_add(32)) };
    { let a0_ = ((c.m.r8((p_arg_2).wrapping_add(86))) as i32); F::BtlCharApi_GetNodePos(c, a0_, 17i32, (l_w_3).wrapping_add(1120)) };
    F::Vec4_Copy(c, (l_w_3).wrapping_add(1136), (l_w_3).wrapping_add(1120));
    F::Vec4_Copy(c, (l_w_3).wrapping_add(1104), (p_arg_2).wrapping_add(16));
    F::Vec4_Set(c, (l_w_3).wrapping_add(1088), F32(0x42fe0000), F32(0x42fe0000), F32(0x42fe0000), F32(0x00000000));
    { let v_ = { let a0_ = rt::fmul(F32(c.m.r32((p_arg_2).wrapping_add(76))), F32(0x40c90fda)); F::EftMath_WrapAngle(c, a0_) }; let a_ = (l_w_3).wrapping_add(1168); c.m.w32(a_, (v_).0); };
    { let v_ = { let v_ = rt::fmul(F32(c.m.r32((p_arg_2).wrapping_add(80))), F32(0x41f00000)); let a_ = (l_w_3).wrapping_add(1176); c.m.w32(a_, (v_).0); v_ }; let a_ = (l_w_3).wrapping_add(1172); c.m.w32(a_, (v_).0); };
    if (((rt::flt(F32(c.m.r32((p_arg_2).wrapping_add(80))), F32(0x00000000)) as i32)) != 0) {
        { let v_ = F32(0x3f800000); let a_ = (l_w_3).wrapping_add(1176); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0xbf800000); let a_ = (l_w_3).wrapping_add(1172); c.m.w32(a_, (v_).0); };
    }
    { let v_ = rt::fdiv({ let a0_ = ((c.m.r8((p_arg_2).wrapping_add(86))) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x419acccc)); let a_ = (l_w_3).wrapping_add(1204); c.m.w32(a_, (v_).0); };
    l_life_5 = rt::fmul(F32(c.m.r32((p_arg_2).wrapping_add(56))), F32(0x41f00000));
    { let v_ = F32(0x00000000); let a_ = (l_w_3).wrapping_add(1200); c.m.w32(a_, (v_).0); };
    { let v_ = l_life_5; let a_ = (l_w_3).wrapping_add(1196); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(F32(c.m.r32((p_arg_2).wrapping_add(60))), F32(0x41f00000)); let a_ = (l_w_3).wrapping_add(1184); c.m.w32(a_, (v_).0); };
    { let v_ = { let v_ = rt::fmul(l_life_5, rt::fsub(F32(0x3f800000), F32(c.m.r32((p_arg_2).wrapping_add(64))))); let a_ = (l_w_3).wrapping_add(1188); c.m.w32(a_, (v_).0); v_ }; let a_ = (l_w_3).wrapping_add(1192); c.m.w32(a_, (v_).0); };
    's1: {
        let sv7: i32 = ((c.m.r8((p_arg_2).wrapping_add(87))) as i32);
        match sv7 {
            0i32 | 1i32 => {
                l_roll_6 = F32(c.m.r32((l_w_3).wrapping_add(1168)));
                if ((((((c.m.r32((p_arg_2).wrapping_add(92)) as i32)) == (0i32)) as i32)) != 0) {
                    l_roll_6 = rt::fneg(l_roll_6);
                }
                { let v_ = l_roll_6; let a_ = (l_w_3).wrapping_add(1168); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul(rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3c23d70a)), F32(0x3dcccccc)), F32(0x40490fda)); let a_ = (l_w_3).wrapping_add(1152); c.m.w32(a_, (v_).0); };
                l_model_4 = (l_w_3).wrapping_add(944);
                { let a1_ = ({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }).wrapping_add(1056); F::EftMesh_Copy(c, l_model_4, a1_) };
                if ((((!(({ let a1_ = ((c.m.r8((p_arg_2).wrapping_add(86))) as i32); let a2_ = ((c.m.r8((p_arg_2).wrapping_add(87))) as i32); F::EftDiscMgr_PushHeld(c, p_task_1, a1_, ((((a2_) as i64 as u64)) as u8)) }) != 0)) as i32)) != 0) {
                    { let r_ = 4i32; let a_ = (l_w_3).wrapping_add(1212); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
                if (((((((c.m.r8((p_arg_2).wrapping_add(87))) as i32)) == (1i32)) as i32)) != 0) {
                    { let r_ = 4096i32; let a_ = (l_w_3).wrapping_add(1212); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
                { let r_ = 8i32; let a_ = (l_w_3).wrapping_add(1212); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = F32(0x3fc00000); let a_ = (l_w_3).wrapping_add(1204); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                break 's1;
            }
            2i32 => {
                l_roll_6 = F32(c.m.r32((l_w_3).wrapping_add(1168)));
                if ((((((c.m.r32((p_arg_2).wrapping_add(92)) as i32)) == (0i32)) as i32)) != 0) {
                    l_roll_6 = rt::fneg(l_roll_6);
                }
                { let v_ = l_roll_6; let a_ = (l_w_3).wrapping_add(1168); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul(rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3c23d70a)), F32(0x3dcccccc)), F32(0x40490fda)); let a_ = (l_w_3).wrapping_add(1152); c.m.w32(a_, (v_).0); };
                l_model_4 = (l_w_3).wrapping_add(944);
                { let a0_ = c.m.r32((p_arg_2).wrapping_add(4)); F::EftEmit_InitState(c, a0_, (l_w_3).wrapping_add(224)) };
                { let a1_ = ({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }).wrapping_add(1056); F::EftMesh_Copy(c, l_model_4, a1_) };
                { let r_ = ((128i32) | (8i32)); let a_ = (l_w_3).wrapping_add(1212); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            3i32 => {
                { let v_ = F32(0x40490fda); let a_ = (l_w_3).wrapping_add(1160); c.m.w32(a_, (v_).0); };
                l_model_4 = (l_w_3).wrapping_add(944);
                { let v_ = F32(0x3f800000); let a_ = (l_w_3).wrapping_add(1180); c.m.w32(a_, (v_).0); };
                { let a1_ = ({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }).wrapping_add(1200); F::EftMesh_Copy(c, l_model_4, a1_) };
                { let r_ = 32i32; let a_ = (l_w_3).wrapping_add(1212); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            4i32 => {
                { let v_ = F32(0x40490fda); let a_ = (l_w_3).wrapping_add(1160); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = (l_w_3).wrapping_add(1180); c.m.w32(a_, (v_).0); };
                l_model_4 = (l_w_3).wrapping_add(944);
                { let a0_ = c.m.r32((p_arg_2).wrapping_add(4)); F::EftEmit_InitState(c, a0_, (l_w_3).wrapping_add(224)) };
                { let a1_ = ({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }).wrapping_add(1200); F::EftMesh_Copy(c, l_model_4, a1_) };
                { let r_ = ((128i32) | (32i32)); let a_ = (l_w_3).wrapping_add(1212); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            _ => {
                l_model_4 = (l_w_3).wrapping_add(944);
                break 's1;
            }
        }
    }
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(1212)) as i32)) & (128i32))) != 0)) as i32)) != 0) {
        { let a1_ = ((c.m.r8((p_arg_2).wrapping_add(86))) as i32); F::EftMesh_SetOwner(c, l_model_4, a1_, 0i32) };
    } else {
        { let a1_ = ((c.m.r8((p_arg_2).wrapping_add(86))) as i32); F::EftMesh_SetOwner(c, l_model_4, a1_, 1i32) };
    }
    { let a1_ = c.m.r32((l_w_3).wrapping_add(1016)); let a2_ = ((c.m.r8((p_arg_2).wrapping_add(84))) as i32); let a3_ = ((c.m.r8((p_arg_2).wrapping_add(85))) as i32); F::EftDisc_SetTex(c, l_w_3, a1_, a2_, a3_) };
    { let r_ = 1i32; let a_ = (l_w_3).wrapping_add(1212); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    { let a1_ = (if (((((((c.m.r8((p_arg_2).wrapping_add(86))) as i32)) == (0i32)) as i32)) != 0) { 2048i32 } else { 4096i32 }); F::BtlTask_SetOwnerTag(c, p_task_1, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftDisc_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let mut l_hold_2: i32 = 0;
    let mut l_pitch_3: F32 = F32(0);
    let mut l_w_4: u32 = 0;
    let mut l_arg_5: u32 = 0;
    let mut l_stopped_6: i32 = 0;
    let mut l_yaw_7: F32 = F32(0);
    let mut l_r_8: F32 = F32(0);
    let mut l_t_9: F32 = F32(0);
    let mut l_k_10: F32 = F32(0);
    let mut l_side_11: F32 = F32(0);
    let mut l_bank_12: F32 = F32(0);
    let r_ = 'f: {
        l_hold_2 = 0i32;
        l_pitch_3 = F32(0x00000000);
        l_w_4 = c.m.r32((p_task_1).wrapping_add(56));
        l_arg_5 = (l_w_4).wrapping_add(48);
        if ((((!(((((c.m.r32((l_w_4).wrapping_add(1212)) as i32)) & (128i32))) != 0)) as i32)) != 0) {
            l_stopped_6 = { let a0_ = ((c.m.r8((l_arg_5).wrapping_add(86))) as i32); F::BtlScene_IsEffectStopped(c, a0_, 0i32) };
            if (({ let a0_ = ((c.m.r8((l_arg_5).wrapping_add(86))) as i32); F::BtlCharApi_IsChanging(c, a0_) }) != 0) {
                { let r_ = 4i32; let a_ = (l_w_4).wrapping_add(1212); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
            if (((((((((((((((c.m.r8((l_arg_5).wrapping_add(87))) as i32)) == (0i32)) as i32)) != 0) && (((((c.m.r32((l_w_4).wrapping_add(1212)) as i32)) & (8i32))) != 0)) as i32)) != 0) && ((((!(({ let a0_ = ((c.m.r8((l_arg_5).wrapping_add(86))) as i32); F::BtlCharApi_IsChargingKiBlast(c, a0_) }) != 0)) as i32)) != 0)) as i32)) != 0) {
                { let r_ = ((1024i32) | (4i32)); let a_ = (l_w_4).wrapping_add(1212); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        } else {
            l_stopped_6 = { let a0_ = ((c.m.r8((l_arg_5).wrapping_add(86))) as i32); F::BtlScene_IsEffectStopped(c, a0_, 1i32) };
            l_hold_2 = ((({ let a_ = ({ let a_ = (c.m.r32(l_arg_5)).wrapping_add(36); c.m.r32(a_) }).wrapping_add(60); (c.m.r32(a_) as i32) }).wrapping_shr((1i32) as u32)) & (1i32));
        }
        if ((((((((!((l_stopped_6) != 0)) as i32)) != 0) && ((((!((l_hold_2) != 0)) as i32)) != 0)) as i32)) != 0) {
            { let _ = F::memset(c, fp, 0i32, 32u32); };
            { let _ = F::memset(c, fp.wrapping_add(32), 0i32, 16u32); };
            if (((((c.m.r32((l_w_4).wrapping_add(1212)) as i32)) & (8i32))) != 0) {
                if (((((((c.m.r8((l_arg_5).wrapping_add(87))) as i32)) < (3i32)) as i32)) != 0) {
                    if (((rt::flt(F32(c.m.r32((l_w_4).wrapping_add(1208))), F32(c.m.r32((l_w_4).wrapping_add(1184)))) as i32)) != 0) {
                        l_r_8 = rt::fdiv(F32(c.m.r32((l_w_4).wrapping_add(1208))), F32(c.m.r32((l_w_4).wrapping_add(1184))));
                        l_t_9 = (if (((rt::flt(l_r_8, F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(0x3f800000), l_r_8) as i32)) != 0) { F32(0x3f800000) } else { l_r_8 }) });
                        { let v_ = l_t_9; let a_ = (l_w_4).wrapping_add(1180); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fmul(l_t_9, F32(0x42fe0000)); let a_ = (l_w_4).wrapping_add(1100); c.m.w32(a_, (v_).0); };
                    } else {
                        { let v_ = F32(0x42fe0000); let a_ = (l_w_4).wrapping_add(1100); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x3f800000); let a_ = (l_w_4).wrapping_add(1180); c.m.w32(a_, (v_).0); };
                        { let r_ = 2048i32; let a_ = (l_w_4).wrapping_add(1212); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    if ((((((c.m.r32((l_arg_5).wrapping_add(92)) as i32)) != (0i32)) as i32)) != 0) {
                        { let a0_ = ((c.m.r8((l_arg_5).wrapping_add(86))) as i32); F::BtlCharApi_GetNodePos(c, a0_, 20i32, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))) };
                        { let a0_ = ((c.m.r8((l_arg_5).wrapping_add(86))) as i32); F::BtlCharApi_GetNodePos(c, a0_, 21i32, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))) };
                    } else {
                        { let a0_ = ((c.m.r8((l_arg_5).wrapping_add(86))) as i32); F::BtlCharApi_GetNodePos(c, a0_, 34i32, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))) };
                        { let a0_ = ((c.m.r8((l_arg_5).wrapping_add(86))) as i32); F::BtlCharApi_GetNodePos(c, a0_, 35i32, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))) };
                    }
                    F::Vec3_Sub(c, fp.wrapping_add(32), (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
                    F::Vec3_Normalize(c, fp.wrapping_add(32), fp.wrapping_add(32));
                    { let a2_ = rt::fmul(F32(c.m.r32((l_w_4).wrapping_add(1204))), F32(0x40400000)); F::Vec3_Scale(c, fp.wrapping_add(32), fp.wrapping_add(32), a2_) };
                    F::Vec3_Add(c, (l_arg_5).wrapping_add(32), (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), fp.wrapping_add(32));
                    { let r_ = { let a0_ = { let a0_ = rt::fmul(rt::fmul(F32(c.m.r32((l_w_4).wrapping_add(1208))), F32(0x3d23d70a)), F32(0x40490fda)); F::EftMath_WrapAngle(c, a0_) }; F::Port_cosf(c, a0_) }; let a_ = ((l_arg_5).wrapping_add(32)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                }
                if (((((c.m.r32((l_w_4).wrapping_add(1212)) as i32)) & (4096i32))) != 0) {
                    if (((((c.m.r32((l_w_4).wrapping_add(1212)) as i32)) & (2048i32))) != 0) {
                        { let r_ = (!(8i32)); let a_ = (l_w_4).wrapping_add(1212); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                        { let r_ = 32i32; let a_ = (l_w_4).wrapping_add(1212); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                }
            }
            { let _ = F::memset(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), 0i32, 16u32); };
            { let _ = F::memset(c, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), 0i32, 16u32); };
            { let _ = F::memset(c, fp.wrapping_add(32), 0i32, 16u32); };
            if (((((c.m.r32((l_w_4).wrapping_add(1212)) as i32)) & (32i32))) != 0) {
                { let a3_ = ((c.m.r8((l_arg_5).wrapping_add(86))) as i32); let a4_ = F32(c.m.r32((l_arg_5).wrapping_add(52))); let a5_ = F32(c.m.r32((l_arg_5).wrapping_add(68))); let a6_ = F32(c.m.r32((l_arg_5).wrapping_add(72))); F::EftDisc_Home(c, (l_arg_5).wrapping_add(16), (l_arg_5).wrapping_add(32), (l_arg_5).wrapping_add(16), a3_, a4_, a5_, a6_) };
                F::Vec4_Copy(c, (l_w_4).wrapping_add(1104), (l_arg_5).wrapping_add(16));
                { let a2_ = F32(c.m.r32((l_arg_5).wrapping_add(52))); F::Vec3_Scale(c, fp.wrapping_add(32), (l_arg_5).wrapping_add(16), a2_) };
                if (((((c.m.r32((l_w_4).wrapping_add(1212)) as i32)) & (8192i32))) != 0) {
                    F::Vec4_Copy(c, (l_w_4).wrapping_add(1136), (l_w_4).wrapping_add(1120));
                    F::Vec4_Copy(c, (l_w_4).wrapping_add(1120), (l_arg_5).wrapping_add(32));
                } else {
                    { let a0_ = ((c.m.r8((l_arg_5).wrapping_add(86))) as i32); F::BtlCharApi_GetNodePos(c, a0_, 17i32, (l_w_4).wrapping_add(1120)) };
                    F::Vec4_Copy(c, (l_w_4).wrapping_add(1136), (l_w_4).wrapping_add(1120));
                }
                F::Vec3_Add(c, (l_arg_5).wrapping_add(32), (l_arg_5).wrapping_add(32), fp.wrapping_add(32));
                l_pitch_3 = { let a0_ = { let a0_ = rt::fneg(F32(c.m.r32(((l_arg_5).wrapping_add(16)).wrapping_add(4)))); F::Mathf_Asin(c, a0_) }; F::EftMath_WrapAngle(c, a0_) };
                if ((((!(((((((c.m.r8((l_arg_5).wrapping_add(87))) as i32)) < (3i32)) as i32)) != 0)) as i32)) != 0) {
                    if (((rt::flt(F32(c.m.r32((l_w_4).wrapping_add(1208))), F32(c.m.r32((l_w_4).wrapping_add(1184)))) as i32)) != 0) {
                        l_r_8 = rt::fdiv(F32(c.m.r32((l_w_4).wrapping_add(1208))), F32(c.m.r32((l_w_4).wrapping_add(1184))));
                        l_t_9 = (if (((rt::flt(l_r_8, F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(0x3f800000), l_r_8) as i32)) != 0) { F32(0x3f800000) } else { l_r_8 }) });
                        { let v_ = rt::fmul(l_t_9, F32(0x42fe0000)); let a_ = (l_w_4).wrapping_add(1100); c.m.w32(a_, (v_).0); };
                    } else {
                        { let v_ = F32(0x42fe0000); let a_ = (l_w_4).wrapping_add(1100); c.m.w32(a_, (v_).0); };
                    }
                }
                if (((((((rt::fle(rt::fsub(F32(c.m.r32((l_w_4).wrapping_add(1196))), F32(c.m.r32((l_w_4).wrapping_add(1188)))), F32(c.m.r32((l_w_4).wrapping_add(1200)))) as i32)) != 0) || (((((c.m.r32((l_w_4).wrapping_add(1212)) as i32)) & (2i32))) != 0)) as i32)) != 0) {
                    l_r_8 = rt::fdiv(F32(c.m.r32((l_w_4).wrapping_add(1192))), F32(c.m.r32((l_w_4).wrapping_add(1188))));
                    l_t_9 = (if (((rt::flt(l_r_8, F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(0x3f800000), l_r_8) as i32)) != 0) { F32(0x3f800000) } else { l_r_8 }) });
                    { let v_ = rt::fmul(l_t_9, F32(0x42fe0000)); let a_ = (l_w_4).wrapping_add(1100); c.m.w32(a_, (v_).0); };
                    { let r_ = F32(0x3f800000); let a_ = (l_w_4).wrapping_add(1192); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                }
                if (((((((c.m.r8((l_arg_5).wrapping_add(87))) as i32)) < (3i32)) as i32)) != 0) {
                    {
                        l_k_10 = F32(0x00000000);
                        if (((rt::flt(l_k_10, F32(c.m.r32((l_w_4).wrapping_add(1172)))) as i32)) != 0) {
                            l_r_8 = rt::fdiv(F32(c.m.r32((l_w_4).wrapping_add(1172))), F32(c.m.r32((l_w_4).wrapping_add(1176))));
                            if ((((!(((rt::flt(l_r_8, l_k_10) as i32)) != 0)) as i32)) != 0) {
                                l_k_10 = (if (((rt::flt(F32(0x3f800000), l_r_8) as i32)) != 0) { F32(0x3f800000) } else { l_r_8 });
                            }
                            { let v_ = rt::fmul(F32(c.m.r32((l_w_4).wrapping_add(1168))), l_k_10); let a_ = (l_w_4).wrapping_add(1168); c.m.w32(a_, (v_).0); };
                            { let r_ = F32(0x3f800000); let a_ = (l_w_4).wrapping_add(1172); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        }
                    }
                    if (({ let a0_ = ((c.m.r8((l_arg_5).wrapping_add(86))) as i32); F::BtlCharApi_IsLockedOn(c, a0_) }) != 0) {
                        { let a0_ = { let a0_ = ((c.m.r8((l_arg_5).wrapping_add(86))) as i32); F::BtlCharApi_GetOpponentObjId(c, a0_) }; F::BtlCharApi_GetNodePos(c, a0_, 3i32, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))) };
                        F::Vec3_Sub(c, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (l_arg_5).wrapping_add(32));
                        F::Vec3_Normalize(c, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
                        l_side_11 = rt::fsub(rt::fmul(F32(c.m.r32(((l_w_4).wrapping_add(1104)).wrapping_add(8))), F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))))), rt::fmul(F32(c.m.r32((l_w_4).wrapping_add(1104))), F32(c.m.r32(((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)))));
                        if (((rt::fle(F32(0x00000000), F::Vec3_Dot(c, (l_w_4).wrapping_add(1104), (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)))) as i32)) != 0) {
                            l_bank_12 = rt::fmul(rt::fmul(l_side_11, F32(0x40490fda)), F32(0x3f999999));
                            if (((rt::flt(l_bank_12, F32(0xbf490fda)) as i32)) != 0) {
                                { let v_ = F32(0xbf490fda); let a_ = (l_w_4).wrapping_add(1164); c.m.w32(a_, (v_).0); };
                            } else {
                                if (((rt::flt(F32(0x3f490fda), l_bank_12) as i32)) != 0) {
                                    { let v_ = F32(0x3f490fda); let a_ = (l_w_4).wrapping_add(1164); c.m.w32(a_, (v_).0); };
                                } else {
                                    { let v_ = l_bank_12; let a_ = (l_w_4).wrapping_add(1164); c.m.w32(a_, (v_).0); };
                                }
                            }
                        }
                    }
                }
                { let r_ = F32(0x3f800000); let a_ = (l_w_4).wrapping_add(1200); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::fle(F32(c.m.r32((l_w_4).wrapping_add(1196))), F32(c.m.r32((l_w_4).wrapping_add(1200)))) as i32)) != 0) {
                    { let r_ = 4i32; let a_ = (l_w_4).wrapping_add(1212); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
                { let r_ = 8192i32; let a_ = (l_w_4).wrapping_add(1212); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
            if (((((((rt::fle(F32(c.m.r32((l_w_4).wrapping_add(1180))), F32(0x00000000)) as i32)) != 0) || (((rt::fle(F32(c.m.r32((l_w_4).wrapping_add(1100))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                { let r_ = (!(64i32)); let a_ = (l_w_4).wrapping_add(1212); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            } else {
                { let r_ = 64i32; let a_ = (l_w_4).wrapping_add(1212); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
            if (((((((c.m.r8((l_arg_5).wrapping_add(87))) as i32)) < (3i32)) as i32)) != 0) {
                { let r_ = F32(c.m.r32((l_w_4).wrapping_add(1152))); let a_ = (l_w_4).wrapping_add(1160); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let v_ = { let a0_ = rt::fneg(F32(c.m.r32((l_w_4).wrapping_add(1160)))); F::EftMath_WrapAngle(c, a0_) }; let a_ = (l_w_4).wrapping_add(1160); c.m.w32(a_, (v_).0); };
            }
            if ((((!(((((((c.m.r8((l_arg_5).wrapping_add(87))) as i32)) < (3i32)) as i32)) != 0)) as i32)) != 0) {
                { let r_ = F32(c.m.r32((l_w_4).wrapping_add(1152))); let a_ = (l_w_4).wrapping_add(1156); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let v_ = { let a0_ = F32(c.m.r32((l_w_4).wrapping_add(1156))); F::EftMath_WrapAngle(c, a0_) }; let a_ = (l_w_4).wrapping_add(1156); c.m.w32(a_, (v_).0); };
            }
            l_yaw_7 = { let a0_ = { let a0_ = F32(c.m.r32((l_arg_5).wrapping_add(16))); let a1_ = F32(c.m.r32(((l_arg_5).wrapping_add(16)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) }; F::EftMath_WrapAngle(c, a0_) };
            F::Vu0Cur_Push(c);
            F::Vu0Cur_LoadIdentity(c);
            { let a0_ = rt::fmul(rt::fmul(F32(c.m.r32((l_arg_5).wrapping_add(48))), F32(c.m.r32((l_w_4).wrapping_add(1204)))), F32(c.m.r32((l_w_4).wrapping_add(1180)))); F::Vu0Cur_ScaleDiagUniform(c, a0_) };
            { let a0_ = F32(c.m.r32((l_w_4).wrapping_add(1160))); F::Vu0Cur_RotateY(c, a0_) };
            { let a0_ = F32(c.m.r32((l_w_4).wrapping_add(1156))); F::Vu0Cur_RotateX(c, a0_) };
            { let a0_ = { let a0_ = rt::fadd(F32(c.m.r32((l_w_4).wrapping_add(1164))), F32(c.m.r32((l_w_4).wrapping_add(1168)))); F::EftMath_WrapAngle(c, a0_) }; F::Vu0Cur_RotateZ(c, a0_) };
            F::Vu0Cur_RotateX(c, l_pitch_3);
            F::Vu0Cur_RotateY(c, l_yaw_7);
            F::Vu0Cur_Translate(c, (l_arg_5).wrapping_add(32));
            F::Vu0Cur_StoreMtx(c, fp.wrapping_add(48));
            F::EftMesh_SetMtx(c, (l_w_4).wrapping_add(944), fp.wrapping_add(48));
            F::Vu0Cur_Pop(c);
            if (((((c.m.r32((l_w_4).wrapping_add(1212)) as i32)) & (2i32))) != 0) {
                if ((((!(((((c.m.r32((l_w_4).wrapping_add(1212)) as i32)) & (64i32))) != 0)) as i32)) != 0) {
                    { let r_ = 4i32; let a_ = (l_w_4).wrapping_add(1212); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
            if (((((c.m.r32((l_w_4).wrapping_add(1212)) as i32)) & (128i32))) != 0) {
                F::EftDisc_SpawnParts(c, p_task_1, 0i32);
                { let a0_ = c.m.r32((l_arg_5).wrapping_add(4)); F::EftEmit_UpdateTrailWidth(c, a0_, (l_w_4).wrapping_add(224)) };
            }
            if (((((c.m.r32((l_w_4).wrapping_add(1212)) as i32)) & (32i32))) != 0) {
                if ((((!(((((c.m.r32((l_w_4).wrapping_add(1212)) as i32)) & (((2i32) | (4i32))))) != 0)) as i32)) != 0) {
                    F::EftDisc_AddHit(c, p_task_1);
                }
            }
            if (((((c.m.r32((l_w_4).wrapping_add(1212)) as i32)) & (4i32))) != 0) {
                { let r_ = (!(1i32)); let a_ = (l_w_4).wrapping_add(1212); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                F::BtlTask_SetDead(c, p_task_1);
            }
            { let r_ = F32(0x3f800000); let a_ = (l_w_4).wrapping_add(1208); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
        if ((((!(((((c.m.r32((l_w_4).wrapping_add(1212)) as i32)) & (4i32))) != 0)) as i32)) != 0) {
            F::EftDisc_StepTex(c, l_w_4);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_Update", &PROBE_N, &[], true); }
    r_
}
