#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EftStreakMgr_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStreakMgr_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        l_i_2 = 0i32;
        { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((4u32) as i32)) }; let a_ = sym::gEftStreak; c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = c.m.r32(sym::gEftStreak); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((23248u32) as i32)) }; let a_ = c.m.r32(sym::gEftStreak); c.m.w32(a_, v_); };
        { let _ = { let a0_ = { let a_ = c.m.r32(sym::gEftStreak); c.m.r32(a_) }; F::memset(c, a0_, 0i32, 23248u32) }; };
        {
            rt::memset(c, fp, 0, 40);
            { let v_ = 591i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
            { let v_ = 592i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
            { let v_ = 593i32; c.m.w32(fp.wrapping_add(8), (v_) as u32); }
            { let v_ = 594i32; c.m.w32(fp.wrapping_add(12), (v_) as u32); }
            { let v_ = 595i32; c.m.w32(fp.wrapping_add(16), (v_) as u32); }
            { let v_ = 596i32; c.m.w32(fp.wrapping_add(20), (v_) as u32); }
            { let v_ = 597i32; c.m.w32(fp.wrapping_add(24), (v_) as u32); }
            { let v_ = 598i32; c.m.w32(fp.wrapping_add(28), (v_) as u32); }
            { let v_ = 599i32; c.m.w32(fp.wrapping_add(32), (v_) as u32); }
            { let v_ = 600i32; c.m.w32(fp.wrapping_add(36), (v_) as u32); }
            'l1: loop {
                'c2: {
                    { let v_ = { let a0_ = (c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32); F::BtlScene_GetCommonEntry(c, a0_) }; let a_ = (({ let a_ = c.m.r32(sym::gEftStreak); c.m.r32(a_) }).wrapping_add(140)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                }
                if !(((((l_i_2) < (10i32)) as i32)) != 0) { break 'l1; }
            }
        }
        { let v_ = F::BtlScene_GetCommonEntry(c, 590i32); let a_ = ({ let a_ = c.m.r32(sym::gEftStreak); c.m.r32(a_) }).wrapping_add(180); c.m.w32(a_, v_); };
        { let a0_ = { let a_ = c.m.r32(sym::gEftStreak); c.m.r32(a_) }; let a1_ = F::BtlScene_GetCommonEntry(c, 589i32); F::EftTexSet_Load8(c, a0_, a1_) };
        { let v_ = F::BtlTask_CreateChildList(c, p_task_1, ((((3i32) as i64 as u64)) as u32), ((((((128u32) as i32)) as i64 as u64)) as u32)); let a_ = ({ let a_ = c.m.r32(sym::gEftStreak); c.m.r32(a_) }).wrapping_add(136); c.m.w32(a_, v_); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftStreakMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftStreakMgr_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStreakMgr_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = ({ let a_ = c.m.r32(sym::gEftStreak); c.m.r32(a_) }).wrapping_add(132); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStreakMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftStreakMgr_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStreakMgr_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStreakMgr_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftStreakMgr_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStreakMgr_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = { let a_ = c.m.r32(sym::gEftStreak); c.m.r32(a_) }; F::BtlPool_Free(c, a0_, a1_) };
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftStreak); F::BtlPool_Free(c, a0_, a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftStreak; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStreakMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftStreak_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let mut l_def_4: u32 = 0;
    let mut l_mgr_5: u32 = 0;
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    l_def_4 = c.m.r32(p_arg_2);
    { let _ = F::memset(c, l_w_3, 0i32, 128u32); };
    { let v_ = c.m.r32(p_arg_2); let a_ = l_w_3; c.m.w32(a_, v_); };
    { let v_ = c.m.r32((p_arg_2).wrapping_add(4)); let a_ = (l_w_3).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = (((c.m.r32((p_arg_2).wrapping_add(16)) as i32)) as u8); let a_ = (l_w_3).wrapping_add(106); c.m.w8(a_, v_); };
    { let v_ = (((c.m.r32((p_arg_2).wrapping_add(20)) as i32)) as u8); let a_ = (l_w_3).wrapping_add(107); c.m.w8(a_, v_); };
    { let v_ = ((2i32) as u8); let a_ = (l_w_3).wrapping_add(105); c.m.w8(a_, v_); };
    { let v_ = F32(c.m.r32((l_def_4).wrapping_add(32))); let a_ = (l_w_3).wrapping_add(92); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_def_4).wrapping_add(36))); let a_ = (l_w_3).wrapping_add(96); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = (l_w_3).wrapping_add(100); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_arg_2).wrapping_add(8))); let a_ = (l_w_3).wrapping_add(80); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = (l_w_3).wrapping_add(84); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(F32(c.m.r32((p_arg_2).wrapping_add(24))), F32(0x41f00000)); let a_ = (l_w_3).wrapping_add(88); c.m.w32(a_, (v_).0); };
    { let v_ = ((0i32) as u32); let a_ = (l_w_3).wrapping_add(112); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_w_3).wrapping_add(116); c.m.w32(a_, v_); };
    l_mgr_5 = { let a_ = c.m.r32(sym::gEftStreak); c.m.r32(a_) };
    { let s_ = (l_mgr_5).wrapping_add((((0i32) as u32)).wrapping_mul(16)); let d_ = (l_w_3).wrapping_add(8); rt::memcpy(c, d_, s_, 16); };
    { let s_ = (l_mgr_5).wrapping_add((((c.m.r8((l_w_3).wrapping_add(106))) as u32)).wrapping_mul(16)); let d_ = (l_w_3).wrapping_add(24); rt::memcpy(c, d_, s_, 16); };
    's1: {
        let sv6: i32 = (c.m.r32((l_def_4).wrapping_add(68)) as i32);
        match sv6 {
            2i32 | 3i32 => {
                { let r_ = 256i32; let a_ = (l_w_3).wrapping_add(108); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            0i32 | 1i32 => {
                { let r_ = 512i32; let a_ = (l_w_3).wrapping_add(108); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
    's2: {
        let sv7: i32 = (c.m.r32((l_def_4).wrapping_add(68)) as i32);
        match sv7 {
            1i32 | 3i32 => {
                { let r_ = 1024i32; let a_ = (l_w_3).wrapping_add(108); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                break 's2;
            }
            _ => {
            }
        }
    }
    { let r_ = 2i32; let a_ = (l_w_3).wrapping_add(108); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftStreak_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_def_3: u32 = 0;
    let mut l_hit_4: i32 = 0;
    let mut l_t_5: F32 = F32(0);
    let mut l_a_6: F32 = F32(0);
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_def_3 = c.m.r32(l_w_2);
    if ((((!(({ let a0_ = ((c.m.r8((l_w_2).wrapping_add(107))) as i32); let a1_ = ((c.m.r8((l_w_2).wrapping_add(105))) as i32); F::BtlScene_IsEffectStopped(c, a0_, a1_) }) != 0)) as i32)) != 0) {
        if (((rt::fle(F32(0x00000000), F32(c.m.r32((l_w_2).wrapping_add(92)))) as i32)) != 0) {
            { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(92); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        } else {
            's1: {
                let sv7: i32 = (c.m.r32((l_def_3).wrapping_add(64)) as i32);
                match sv7 {
                    1i32 => {
                        {
                            l_hit_4 = ((({ let a0_ = ((c.m.r8((l_w_2).wrapping_add(107))) as i32); F::BtlCharApi_IsInTechnique(c, a0_) }) != (0i32)) as i32);
                            if ((((({ let a0_ = ((c.m.r8((l_w_2).wrapping_add(107))) as i32); F::BtlCharApi_GetOpponentObjId(c, a0_) }) >= (0i32)) as i32)) != 0) {
                                if (({ let a0_ = { let a0_ = ((c.m.r8((l_w_2).wrapping_add(107))) as i32); F::BtlCharApi_GetOpponentObjId(c, a0_) }; F::BtlCharApi_IsInTechnique(c, a0_) }) != 0) {
                                    l_hit_4 = 1i32;
                                }
                            }
                            if ((l_hit_4) != 0) {
                                { let r_ = 8i32; let a_ = (l_w_2).wrapping_add(108); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            }
                            break 's1;
                        }
                    }
                    2i32 => {
                        if ((F::EftCam_IsActive(c)) != 0) {
                            { let r_ = 8i32; let a_ = (l_w_2).wrapping_add(108); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
            if ((((!(((((c.m.r32((l_w_2).wrapping_add(108)) as i32)) & (4i32))) != 0)) as i32)) != 0) {
                F::EftStreak_SpawnAll(c, l_w_2);
            }
            if (((((c.m.r32((l_w_2).wrapping_add(108)) as i32)) & (4i32))) != 0) {
                l_t_5 = rt::fdiv(F32(c.m.r32((l_w_2).wrapping_add(96))), F32(c.m.r32((l_def_3).wrapping_add(36))));
                if (((rt::flt(l_t_5, F32(0x00000000)) as i32)) != 0) {
                    l_a_6 = F32(0x00000000);
                } else {
                    if (((rt::flt(F32(0x3f800000), l_t_5) as i32)) != 0) {
                        l_a_6 = F32(0x3f800000);
                    } else {
                        l_a_6 = l_t_5;
                    }
                }
                { let v_ = l_a_6; let a_ = (l_w_2).wrapping_add(100); c.m.w32(a_, (v_).0); };
                { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(96); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::flt(F32(c.m.r32((l_w_2).wrapping_add(96))), F32(0x00000000)) as i32)) != 0) {
                    { let r_ = 8i32; let a_ = (l_w_2).wrapping_add(108); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
            F::EftStreak_Step(c, l_w_2);
            if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_w_2).wrapping_add(88)))) as i32)) != 0) {
                { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(88); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::flt(F32(c.m.r32((l_w_2).wrapping_add(88))), F32(0x00000000)) as i32)) != 0) {
                    { let r_ = 4i32; let a_ = (l_w_2).wrapping_add(108); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
        }
        if (((((c.m.r32((l_w_2).wrapping_add(108)) as i32)) & (8i32))) != 0) {
            { let r_ = (!(2i32)); let a_ = (l_w_2).wrapping_add(108); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            F::BtlTask_SetDead(c, p_task_1);
        }
    }
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(108)) as i32)) & (8i32))) != 0)) as i32)) != 0) {
        F::EftStreak_StepTexture(c, l_w_2);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftStreak_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftStreak_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(464) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 464);
    let mut l_w_2: u32 = 0;
    let mut l_def_3: u32 = 0;
    let mut l_mgr_4: u32 = 0;
    let mut l_dist_5: F32 = F32(0);
    let mut l_s_6: u32 = 0;
    let mut l_j_7: i32 = 0;
    let mut l_f_8: i32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_def_3 = c.m.r32(l_w_2);
        l_mgr_4 = { let a_ = c.m.r32(sym::gEftStreak); c.m.r32(a_) };
        rt::memset(c, fp, 0, 64);
        rt::memset(c, fp.wrapping_add(0), 0, 64);
        rt::memset(c, fp.wrapping_add(0).wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(0).wrapping_add(0), (v_).0); }
        rt::memset(c, fp.wrapping_add(64), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(0), (v_).0); }
        rt::memset(c, fp.wrapping_add(80), 0, 64);
        rt::memset(c, fp.wrapping_add(80).wrapping_add(0), 0, 64);
        rt::memset(c, fp.wrapping_add(80).wrapping_add(0).wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(80).wrapping_add(0).wrapping_add(0).wrapping_add(0), (v_).0); }
        rt::memset(c, fp.wrapping_add(144), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(144).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(144).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(144).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(144).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(160), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(160).wrapping_add(0), (v_).0); }
        rt::memset(c, fp.wrapping_add(176), 0, 16);
        { let v_ = 0i32; c.m.w32(fp.wrapping_add(176).wrapping_add(0), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(192), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(192).wrapping_add(0), (v_).0); }
        l_dist_5 = F32(0x00000000);
        F::Mtx_StoreIdentity(c, fp);
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(64); F::Mtx_InverseRT(c, fp.wrapping_add(80), a1_) };
        F::Vec4_Set(c, (fp.wrapping_add(80)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x3f800000));
        F::Mtx_MulVec4(c, fp.wrapping_add(144), fp.wrapping_add(80), fp.wrapping_add(144));
        F::Vec3_Normalize(c, fp.wrapping_add(144), fp.wrapping_add(144));
        if ((((!(((((c.m.r32((l_w_2).wrapping_add(108)) as i32)) & (2048i32))) != 0)) as i32)) != 0) {
            if (((((c.m.r32((l_w_2).wrapping_add(108)) as i32)) & (1024i32))) != 0) {
                { let a0_ = ((c.m.r8((l_w_2).wrapping_add(107))) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, fp.wrapping_add(64)) };
                { let a2_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(544); F::Vec3_Sub(c, fp.wrapping_add(192), fp.wrapping_add(64), a2_) };
                l_dist_5 = F::Vec3_Length(c, fp.wrapping_add(192));
                { let r_ = rt::fmul({ let a0_ = ((c.m.r8((l_w_2).wrapping_add(107))) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x3f000000)); let n_ = rt::fadd(l_dist_5, r_); l_dist_5 = n_; };
            } else {
                l_dist_5 = F32(0x41f00000);
            }
            { let a2_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(544); F::Vec3_ScaleAdd(c, fp.wrapping_add(160), fp.wrapping_add(144), a2_, l_dist_5) };
        } else {
            F::Vec4_Copy(c, fp.wrapping_add(160), (l_w_2).wrapping_add(48));
        }
        { let _ = { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Mtx_ProjectPoint(c, fp.wrapping_add(176), a1_, fp.wrapping_add(160)) }; };
        if (((((c.m.r32((l_w_2).wrapping_add(108)) as i32)) & (512i32))) != 0) {
            F::Mtx_Mul(c, fp, fp, fp.wrapping_add(80));
            F::Vec4_Copy(c, (fp).wrapping_add((((3i32) as u32)).wrapping_mul(16)), fp.wrapping_add(160));
        } else {
            if ((((((c.m.r32((fp.wrapping_add(176)).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                break 'f;
            }
            { let v_ = rt::i2f((((c.m.r32(fp.wrapping_add(176)) as i32)).wrapping_sub(28672i32)).wrapping_shr((4i32) as u32)); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = rt::i2f((((c.m.r32((fp.wrapping_add(176)).wrapping_add(4)) as i32)).wrapping_sub(29184i32)).wrapping_shr((4i32) as u32)); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f800000); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        }
        F::Vu0Cur_Push(c);
        { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
        {
            rt::memset(c, fp.wrapping_add(208), 0, 64);
            rt::memset(c, fp.wrapping_add(208).wrapping_add(0), 0, 16);
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(208).wrapping_add(0).wrapping_add(0), (v_).0); }
            rt::memset(c, fp.wrapping_add(272), 0, 64);
            rt::memset(c, fp.wrapping_add(272).wrapping_add(0), 0, 16);
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(272).wrapping_add(0).wrapping_add(0), (v_).0); }
            rt::memset(c, fp.wrapping_add(336), 0, 16);
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(336).wrapping_add(0), (v_).0); }
            rt::memset(c, fp.wrapping_add(352), 0, 16);
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(352).wrapping_add(0), (v_).0); }
            rt::memset(c, fp.wrapping_add(368), 0, 32);
            rt::memset(c, fp.wrapping_add(368).wrapping_add(0), 0, 16);
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(368).wrapping_add(0).wrapping_add(0), (v_).0); }
            rt::memset(c, fp.wrapping_add(400), 0, 64);
            rt::memset(c, fp.wrapping_add(400).wrapping_add(0), 0, 64);
            rt::memset(c, fp.wrapping_add(400).wrapping_add(0).wrapping_add(0), 0, 16);
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(400).wrapping_add(0).wrapping_add(0).wrapping_add(0), (v_).0); }
            F::Vec4_Set(c, (fp.wrapping_add(208)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), F32(0xbf000000), F32(0xbf000000), F32(0x00000000), F32(0x3f800000));
            F::Vec4_Set(c, (fp.wrapping_add(208)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), F32(0xbf000000), F32(0x3f000000), F32(0x00000000), F32(0x3f800000));
            F::Vec4_Set(c, (fp.wrapping_add(208)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), F32(0x3f000000), F32(0xbf000000), F32(0x00000000), F32(0x3f800000));
            F::Vec4_Set(c, (fp.wrapping_add(208)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), F32(0x3f000000), F32(0x3f000000), F32(0x00000000), F32(0x3f800000));
            F::Vec4_Set(c, (fp.wrapping_add(368)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x3f800000));
            F::Vec4_Set(c, (fp.wrapping_add(368)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), F32(0x3f800000), F32(0x00000000), F32(0x3f800000), F32(0x3f800000));
            {
                l_s_6 = c.m.r32((l_w_2).wrapping_add(112));
                'l1: loop {
                    if !(((((l_s_6) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                    'c2: {
                        if (((((c.m.r32((l_s_6).wrapping_add(48)) as i32)) & (16i32))) != 0) {
                            F::Mtx_StoreIdentity(c, fp.wrapping_add(400));
                            F::Vec4_Set(c, (l_w_2).wrapping_add(64), F32(0x3f800000), F32(0x00000000), F32(0x00000000), F32(0x3f800000));
                            { let a2_ = F32(c.m.r32((l_s_6).wrapping_add(24))); F::Vec3_Scale(c, fp.wrapping_add(336), (l_w_2).wrapping_add(64), a2_) };
                            { let a2_ = F32(c.m.r32((l_s_6).wrapping_add(32))); F::Vec4_Set(c, fp.wrapping_add(352), F32(0x00000000), a2_, F32(0x00000000), F32(0x3f800000)) };
                            F::Mtx_Translate(c, fp.wrapping_add(400), fp.wrapping_add(400), fp.wrapping_add(352));
                            F::Mtx_Translate(c, fp.wrapping_add(400), fp.wrapping_add(400), fp.wrapping_add(336));
                            { let a2_ = F32(c.m.r32((l_w_2).wrapping_add(80))); F::Mtx_RotateZ(c, fp.wrapping_add(400), fp.wrapping_add(400), a2_) };
                            {
                                l_j_7 = 0i32;
                                'l3: loop {
                                    if !(((((l_j_7) < (4i32)) as i32)) != 0) { break 'l3; }
                                    'c4: {
                                        { let v_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(208)).wrapping_add((((l_j_7) as u32)).wrapping_mul(16)))), F32(c.m.r32((l_s_6).wrapping_add(16)))); let a_ = (fp.wrapping_add(272)).wrapping_add((((l_j_7) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                                        { let v_ = rt::fmul(F32(c.m.r32(((fp.wrapping_add(208)).wrapping_add((((l_j_7) as u32)).wrapping_mul(16))).wrapping_add(4))), F32(c.m.r32((l_s_6).wrapping_add(20)))); let a_ = ((fp.wrapping_add(272)).wrapping_add((((l_j_7) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                        { let v_ = F32(c.m.r32(((fp.wrapping_add(208)).wrapping_add((((l_j_7) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = ((fp.wrapping_add(272)).wrapping_add((((l_j_7) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                        { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(272)).wrapping_add((((l_j_7) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                        F::Mtx_MulVec4(c, (fp.wrapping_add(272)).wrapping_add((((l_j_7) as u32)).wrapping_mul(16)), fp.wrapping_add(400), (fp.wrapping_add(272)).wrapping_add((((l_j_7) as u32)).wrapping_mul(16)));
                                        F::Mtx_MulVec4(c, (fp.wrapping_add(272)).wrapping_add((((l_j_7) as u32)).wrapping_mul(16)), fp, (fp.wrapping_add(272)).wrapping_add((((l_j_7) as u32)).wrapping_mul(16)));
                                    }
                                    { let o_ = l_j_7; let n_ = (o_).wrapping_add(1); l_j_7 = n_; };
                                }
                            }
                            if (((((c.m.r32((l_w_2).wrapping_add(108)) as i32)) & (512i32))) != 0) {
                                l_f_8 = (((c.m.r32((l_w_2).wrapping_add(108)) as i32)) & (1024i32));
                                l_f_8 = (((l_f_8) == (0i32)) as i32);
                                if (((((c.m.r32((l_w_2).wrapping_add(108)) as i32)) & (2048i32))) != 0) {
                                    l_f_8 = 0i32;
                                }
                                { let a4_ = ((c.m.r8((l_def_3).wrapping_add(72))) as i32); F::EftStreak_DrawWorld(c, fp.wrapping_add(272), (fp.wrapping_add(368)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(368)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), l_s_6, a4_, 0i32, l_f_8, l_mgr_4) };
                            } else {
                                { let a2_ = ((c.m.r8((l_def_3).wrapping_add(72))) as i32); let a4_ = (if (((((c.m.r32((l_w_2).wrapping_add(108)) as i32)) & (1024i32))) != 0) { (c.m.r32((fp.wrapping_add(176)).wrapping_add(8)) as i32) } else { 16777215i32 }); let a5_ = c.m.r64((l_mgr_4).wrapping_add((((0i32) as u32)).wrapping_mul(16))); F::EftStreak_DrawScreen(c, fp.wrapping_add(272), l_s_6, a2_, 4i32, a4_, a5_) };
                            }
                        }
                    }
                    l_s_6 = c.m.r32((l_s_6).wrapping_add(52));
                }
            }
        }
        F::Vu0Cur_Pop(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftStreak_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    { let r_ = (!(2i32)); let a_ = (l_w_2).wrapping_add(108); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftStreak_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_s_3: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    {
        l_s_3 = c.m.r32((l_w_2).wrapping_add(112));
        'l1: loop {
            if !(((((l_s_3) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = 0i32; let a_ = (l_s_3).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
            }
            l_s_3 = c.m.r32((l_s_3).wrapping_add(52));
        }
    }
    { let _ = F::memset(c, l_w_2, 0i32, 128u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftStreak_SpawnAll(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_SpawnAll", &PROBE_N, &[(p_w_1) as u32], false); }
    let mut l_def_2: u32 = 0;
    let mut l_gap_3: F32 = F32(0);
    let mut l_s_4: u32 = 0;
    let r_ = 'f: {
    l_def_2 = c.m.r32(p_w_1);
    l_gap_3 = rt::fdiv(F32(c.m.r32((l_def_2).wrapping_add(56))), rt::i2f((c.m.r8((l_def_2).wrapping_add(73))) as i32));
    if (((((((c.m.r8((p_w_1).wrapping_add(104))) as i32)) < (((c.m.r8((l_def_2).wrapping_add(73))) as i32))) as i32)) != 0) {
        'l1: loop {
            'c2: {
                l_s_4 = F::EftStreak_Alloc(c);
                if (((((l_s_4) == (((0i32) as u32))) as i32)) != 0) {
                    break 'l1;
                }
                { let a2_ = rt::fsub(rt::fmul(F32(c.m.r32((l_def_2).wrapping_add(56))), F32(0x3f000000)), rt::fmul(l_gap_3, rt::i2f((c.m.r8((p_w_1).wrapping_add(104))) as i32))); F::EftStreak_Roll(c, l_s_4, p_w_1, a2_) };
                { let v_ = ((0i32) as u32); let a_ = (l_s_4).wrapping_add(52); c.m.w32(a_, v_); };
                { let v_ = 2i32; let a_ = (l_s_4).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                if (((((c.m.r32((p_w_1).wrapping_add(112))) == (((0i32) as u32))) as i32)) != 0) {
                    { let v_ = l_s_4; let a_ = (p_w_1).wrapping_add(112); c.m.w32(a_, v_); };
                    { let v_ = l_s_4; let a_ = (p_w_1).wrapping_add(116); c.m.w32(a_, v_); };
                } else {
                    { let v_ = l_s_4; let a_ = (c.m.r32((p_w_1).wrapping_add(116))).wrapping_add(52); c.m.w32(a_, v_); };
                    { let v_ = l_s_4; let a_ = (p_w_1).wrapping_add(116); c.m.w32(a_, v_); };
                }
                { let a_ = (p_w_1).wrapping_add(104); let o_ = c.m.r8(a_); let n_ = (o_).wrapping_add(1); c.m.w8(a_, n_); };
            }
            if !(((((((c.m.r8((p_w_1).wrapping_add(104))) as i32)) < (((c.m.r8((l_def_2).wrapping_add(73))) as i32))) as i32)) != 0) { break 'l1; }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_SpawnAll", &PROBE_N, &[], true); }
    r_
}

pub fn EftStreak_Alloc(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_Alloc", &PROBE_N, &[], false); }
    let mut l_i_1: u32 = 0;
    let mut l_s_2: u32 = 0;
    let r_ = 'f: {
    if ((((({ let a_ = ({ let a_ = c.m.r32(sym::gEftStreak); c.m.r32(a_) }).wrapping_add(23232); c.m.r32(a_) }) >= (((360i32) as u32))) as i32)) != 0) {
        { let v_ = ((0i32) as u32); let a_ = ({ let a_ = c.m.r32(sym::gEftStreak); c.m.r32(a_) }).wrapping_add(23232); c.m.w32(a_, v_); };
    }
    l_i_1 = { let a_ = ({ let a_ = c.m.r32(sym::gEftStreak); c.m.r32(a_) }).wrapping_add(23232); c.m.r32(a_) };
    'l1: loop {
        'c2: {
            l_s_2 = (({ let a_ = c.m.r32(sym::gEftStreak); c.m.r32(a_) }).wrapping_add(192)).wrapping_add((l_i_1).wrapping_mul(64));
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            if (((((l_i_1) >= (((360i32) as u32))) as i32)) != 0) {
                l_i_1 = ((0i32) as u32);
            }
            if ((((((c.m.r32((l_s_2).wrapping_add(48)) as i32)) == (0i32)) as i32)) != 0) {
                { let v_ = l_i_1; let a_ = ({ let a_ = c.m.r32(sym::gEftStreak); c.m.r32(a_) }).wrapping_add(23232); c.m.w32(a_, v_); };
                break 'f l_s_2;
            }
        }
        if !(((((l_i_1) != ({ let a_ = ({ let a_ = c.m.r32(sym::gEftStreak); c.m.r32(a_) }).wrapping_add(23232); c.m.r32(a_) })) as i32)) != 0) { break 'l1; }
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_Alloc", &PROBE_N, &[], true); }
    r_
}

pub fn EftStreak_Roll(c: &mut Ctx, mut p_s_1: u32, mut p_w_2: u32, mut p_offset_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_Roll", &PROBE_N, &[(p_s_1) as u32, (p_w_2) as u32, (p_offset_3).0], false); }
    let mut l_def_4: u32 = 0;
    let r_ = 'f: {
    l_def_4 = c.m.r32(p_w_2);
    { let a1_ = (c.m.r32((p_w_2).wrapping_add(4))).wrapping_add((((c.m.r8((p_w_2).wrapping_add(106))) as u32)).wrapping_mul(16)); F::Vec4_Copy(c, p_s_1, a1_) };
    { let v_ = { let a0_ = F32(c.m.r32(l_def_4)); let a1_ = rt::fadd(F32(c.m.r32(l_def_4)), F32(c.m.r32((l_def_4).wrapping_add(4)))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (p_s_1).wrapping_add(16); c.m.w32(a_, (v_).0); };
    { let v_ = { let a0_ = F32(c.m.r32((l_def_4).wrapping_add(8))); let a1_ = rt::fadd(F32(c.m.r32((l_def_4).wrapping_add(8))), F32(c.m.r32((l_def_4).wrapping_add(12)))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (p_s_1).wrapping_add(20); c.m.w32(a_, (v_).0); };
    { let v_ = { let a0_ = F32(c.m.r32((l_def_4).wrapping_add(40))); let a1_ = rt::fadd(F32(c.m.r32((l_def_4).wrapping_add(40))), F32(c.m.r32((l_def_4).wrapping_add(44)))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (p_s_1).wrapping_add(24); c.m.w32(a_, (v_).0); };
    { let v_ = { let a0_ = F32(c.m.r32((l_def_4).wrapping_add(48))); let a1_ = rt::fadd(F32(c.m.r32((l_def_4).wrapping_add(48))), F32(c.m.r32((l_def_4).wrapping_add(52)))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (p_s_1).wrapping_add(28); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul({ let a0_ = F32(c.m.r32((l_def_4).wrapping_add(16))); let a1_ = rt::fadd(F32(c.m.r32((l_def_4).wrapping_add(16))), F32(c.m.r32((l_def_4).wrapping_add(20)))); F::Rand_FloatRange(c, a0_, a1_) }, F32(0x41f00000)); let a_ = (p_s_1).wrapping_add(40); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (p_s_1).wrapping_add(36); c.m.w32(a_, (v_).0); };
    { let v_ = { let a0_ = F32(c.m.r32((p_s_1).wrapping_add(36))); let a1_ = F32(c.m.r32((l_def_4).wrapping_add(24))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (p_s_1).wrapping_add(44); c.m.w32(a_, (v_).0); };
    { let v_ = p_offset_3; let a_ = (p_s_1).wrapping_add(32); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_Roll", &PROBE_N, &[], true); }
    r_
}

pub fn EftStreak_Step(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_Step", &PROBE_N, &[(p_w_1) as u32], false); }
    let mut l_def_2: u32 = 0;
    let mut l_colors_3: u32 = 0;
    let mut l_s_4: u32 = 0;
    let mut l_half_5: F32 = F32(0);
    let mut l_a_6: F32 = F32(0);
    let mut l_in_7: F32 = F32(0);
    let mut l_hole_8: F32 = F32(0);
    let mut l_r_9: F32 = F32(0);
    let mut l_t_10: F32 = F32(0);
    let mut l_x_11: F32 = F32(0);
    let mut l_x_12: F32 = F32(0);
    let r_ = 'f: {
    l_def_2 = c.m.r32(p_w_1);
    l_half_5 = F32(0x00000000);
    l_colors_3 = c.m.r32((p_w_1).wrapping_add(4));
    l_hole_8 = rt::fmul(F32(c.m.r32((l_def_2).wrapping_add(60))), F32(c.m.r32((p_w_1).wrapping_add(84))));
    l_a_6 = F32(0x3f800000);
    l_in_7 = F32(0x3ecccccc);
    if (((rt::flt(l_hole_8, l_half_5) as i32)) != 0) {
        l_r_9 = l_half_5;
    } else {
        if (((rt::flt(l_a_6, l_hole_8) as i32)) != 0) {
            l_r_9 = l_a_6;
        } else {
            l_r_9 = l_hole_8;
        }
    }
    l_half_5 = rt::fmul(rt::fmul(F32(c.m.r32((l_def_2).wrapping_add(56))), l_r_9), F32(0x3f000000));
    {
        l_s_4 = c.m.r32((p_w_1).wrapping_add(112));
        'l1: loop {
            if !(((((l_s_4) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_s_4).wrapping_add(44)))) as i32)) != 0) {
                    { let r_ = F32(0x3f800000); let a_ = (l_s_4).wrapping_add(44); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    break 'c2;
                }
                { let r_ = F32(c.m.r32((l_s_4).wrapping_add(28))); let a_ = (l_s_4).wrapping_add(24); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_s_4).wrapping_add(40)))) as i32)) != 0) {
                    l_t_10 = rt::fdiv(F32(c.m.r32((l_s_4).wrapping_add(36))), F32(c.m.r32((l_s_4).wrapping_add(40))));
                    if (((rt::flt(l_t_10, l_in_7) as i32)) != 0) {
                        l_x_11 = rt::fdiv(l_t_10, l_in_7);
                        if (((rt::flt(l_x_11, F32(0x00000000)) as i32)) != 0) {
                            l_a_6 = F32(0x00000000);
                        } else {
                            l_a_6 = l_x_11;
                            if (((rt::flt(F32(0x3f800000), l_a_6) as i32)) != 0) {
                                l_a_6 = F32(0x3f800000);
                            }
                        }
                    } else {
                        l_x_12 = rt::fsub(F32(0x3f800000), rt::fdiv(rt::fsub(l_t_10, l_in_7), rt::fsub(F32(0x3f800000), l_in_7)));
                        if (((rt::flt(l_x_12, F32(0x00000000)) as i32)) != 0) {
                            l_a_6 = F32(0x00000000);
                        } else {
                            l_a_6 = F32(0x3f800000);
                            if ((((!(((rt::flt(l_a_6, l_x_12) as i32)) != 0)) as i32)) != 0) {
                                l_a_6 = l_x_12;
                            }
                        }
                    }
                    { let r_ = F32(0x3f800000); let a_ = (l_s_4).wrapping_add(36); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if (((rt::fle(F32(c.m.r32((l_s_4).wrapping_add(40))), F32(c.m.r32((l_s_4).wrapping_add(36)))) as i32)) != 0) {
                        { let r_ = 128i32; let a_ = (l_s_4).wrapping_add(48); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                }
                { let v_ = rt::fmul(rt::fmul({ let a_ = ((l_colors_3).wrapping_add((((c.m.r8((p_w_1).wrapping_add(106))) as u32)).wrapping_mul(16))).wrapping_add(12); F32(c.m.r32(a_)) }, F32(c.m.r32((p_w_1).wrapping_add(100)))), l_a_6); let a_ = (l_s_4).wrapping_add(12); c.m.w32(a_, (v_).0); };
                if (((((((((((rt::flt(F32(c.m.r32((l_s_4).wrapping_add(32))), l_half_5) as i32)) != 0) && (((rt::flt(rt::fneg(l_half_5), F32(c.m.r32((l_s_4).wrapping_add(32)))) as i32)) != 0)) as i32)) != 0) || (((rt::fle(F32(c.m.r32((l_s_4).wrapping_add(12))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                    { let r_ = (!(16i32)); let a_ = (l_s_4).wrapping_add(48); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                } else {
                    { let r_ = 16i32; let a_ = (l_s_4).wrapping_add(48); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
                if (((((c.m.r32((l_s_4).wrapping_add(48)) as i32)) & (128i32))) != 0) {
                    { let a2_ = F32(c.m.r32((l_s_4).wrapping_add(32))); F::EftStreak_Roll(c, l_s_4, p_w_1, a2_) };
                    { let v_ = 2i32; let a_ = (l_s_4).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                }
            }
            l_s_4 = c.m.r32((l_s_4).wrapping_add(52));
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_Step", &PROBE_N, &[], true); }
    r_
}

pub fn EftStreak_DrawWorld(c: &mut Ctx, mut p_corner_1: u32, p_uv0_2: u32, p_uv1_3: u32, p_color_4: u32, mut p_blend_5: i32, mut p_tex_6: i32, mut p_flag_7: i32, mut p_mgr_8: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_DrawWorld", &PROBE_N, &[(p_corner_1) as u32, (p_uv0_2) as u32, (p_uv1_3) as u32, (p_color_4) as u32, (p_blend_5) as u32, (p_tex_6) as u32, (p_flag_7) as u32, (p_mgr_8) as u32], false); }
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
        { let a6_ = c.m.r64((p_mgr_8).wrapping_add((((p_tex_6) as u32)).wrapping_mul(16))); F::EftGfx_DrawPolyScaledZ(c, fp.wrapping_add(176), p_blend_5, 1i32, 0i32, p_flag_7, 0i32, a6_, F32(0x40000000)) };
        F::ClipVtx_Set(c, (fp.wrapping_add(176)).wrapping_add((((0i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(112)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), fp.wrapping_add(32));
        F::ClipVtx_Set(c, (fp.wrapping_add(176)).wrapping_add((((1i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(112)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), fp.wrapping_add(32));
        F::ClipVtx_Set(c, (fp.wrapping_add(176)).wrapping_add((((2i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(112)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), fp.wrapping_add(32));
        { let a6_ = c.m.r64((p_mgr_8).wrapping_add((((p_tex_6) as u32)).wrapping_mul(16))); F::EftGfx_DrawPolyScaledZ(c, fp.wrapping_add(176), p_blend_5, 1i32, 0i32, p_flag_7, 0i32, a6_, F32(0x40000000)) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_DrawWorld", &PROBE_N, &[], true); }
    r_
}

pub fn eft_t_b__EftTOt_Add(c: &mut Ctx, mut p_p_1: u32, mut p_z_2: i32, mut p_layer_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_t_b__EftTOt_Add", &PROBE_N, &[(p_p_1) as u32, (p_z_2) as u32, (p_layer_3) as u32], false); }
    let mut l_e_4: u32 = 0;
    let r_ = 'f: {
    if (((((p_layer_3) >= (2i32)) as i32)) != 0) {
        { let r_ = 2i32; let n_ = (p_layer_3).wrapping_sub(r_); p_layer_3 = n_; };
    }
    if (((((p_z_2) < (0i32)) as i32)) != 0) {
        l_e_4 = ((c.m.r32(sym::gOtZ)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((p_layer_3) as u32)).wrapping_mul(8));
    } else {
        if (((((p_z_2) >= (4096i32)) as i32)) != 0) {
            l_e_4 = ((c.m.r32(sym::gOtZ)).wrapping_add((((4095i32) as u32)).wrapping_mul(16))).wrapping_add((((p_layer_3) as u32)).wrapping_mul(8));
        } else {
            l_e_4 = ((c.m.r32(sym::gOtZ)).wrapping_add((((p_z_2) as u32)).wrapping_mul(16))).wrapping_add((((p_layer_3) as u32)).wrapping_mul(8));
        }
    }
    { let v_ = p_p_1; let a_ = (c.m.r32((l_e_4).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = p_p_1; let a_ = (l_e_4).wrapping_add(4); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "eft_t_b__EftTOt_Add", &PROBE_N, &[], true); }
    r_
}

pub fn EftStreak_DrawScreen(c: &mut Ctx, mut p_corner_1: u32, p_color_2: u32, mut p_blend_3: i32, mut p_segs_4: i32, mut p_z_5: i32, mut p_tex0_6: u64) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_DrawScreen", &PROBE_N, &[(p_corner_1) as u32, (p_color_2) as u32, (p_blend_3) as u32, (p_segs_4) as u32, (p_z_5) as u32, (p_tex0_6) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(256) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 256);
    let mut l_fsegs_7: F32 = F32(0);
    let mut l_slot_8: i32 = 0;
    let mut l_q_9: u32 = 0;
    let mut l_zz_10: i32 = 0;
    let mut l_c3_11: u32 = 0;
    let mut l_c1_12: u32 = 0;
    rt::memcpy(c, fp, p_color_2, 16);
    let r_ = 'f: {
        rt::memset(c, fp.wrapping_add(16), 0, 32);
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(0), (v_).0); }
        rt::memset(c, fp.wrapping_add(48), 0, 64);
        rt::memset(c, fp.wrapping_add(48).wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(48).wrapping_add(0).wrapping_add(0), (v_).0); }
        rt::memset(c, fp.wrapping_add(112), 0, 64);
        rt::memset(c, fp.wrapping_add(112).wrapping_add(0), 0, 16);
        { let v_ = 0i32; c.m.w32(fp.wrapping_add(112).wrapping_add(0).wrapping_add(0), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(176), 0, 16);
        { let v_ = 0i32; c.m.w32(fp.wrapping_add(176).wrapping_add(0), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(192), 0, 32);
        rt::memset(c, fp.wrapping_add(192).wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(192).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(192).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(192).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(192).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(192).wrapping_add(16), 0, 16);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(192).wrapping_add(16).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(192).wrapping_add(16).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(192).wrapping_add(16).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(192).wrapping_add(16).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(224), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(224).wrapping_add(0), (v_).0); }
        rt::memset(c, fp.wrapping_add(240), 0, 8);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(240).wrapping_add(0), (v_).0); }
        l_fsegs_7 = rt::i2f(p_segs_4);
        l_slot_8 = 0i32;
        l_c1_12 = (p_corner_1).wrapping_add((((1i32) as u32)).wrapping_mul(16));
        F::Vec3_Sub(c, (fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (p_corner_1).wrapping_add((((0i32) as u32)).wrapping_mul(16)), l_c1_12);
        l_c3_11 = (p_corner_1).wrapping_add((((2i32) as u32)).wrapping_mul(16));
        { let r_ = 3i32; let n_ = (p_corner_1).wrapping_add((((r_) as u32)).wrapping_mul(16)); p_corner_1 = n_; };
        F::Vec3_Sub(c, (fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), l_c3_11, p_corner_1);
        F::Vec3_Div(c, (fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), l_fsegs_7);
        F::Vec3_Div(c, (fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), l_fsegs_7);
        F::Vec4_Copy(c, (fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), l_c1_12);
        F::Vec4_Copy(c, (fp.wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), p_corner_1);
        { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec4_ToInt(c, fp.wrapping_add(176), fp);
        { let v_ = F32(c.m.r32(((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); let a_ = (fp.wrapping_add(224)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12))); let a_ = (fp.wrapping_add(224)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32(((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))), F32(c.m.r32(((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12)))), l_fsegs_7); let a_ = (fp.wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32(((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))), F32(c.m.r32(((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12)))), l_fsegs_7); let a_ = (fp.wrapping_add(240)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        if (((((p_z_5) != (16777215i32)) as i32)) != 0) {
            l_slot_8 = (p_z_5).wrapping_shr((8i32) as u32);
            l_zz_10 = (l_slot_8).wrapping_shl((8i32) as u32);
        } else {
            l_zz_10 = 16777215i32;
        }
        if (((((p_segs_4) > (0i32)) as i32)) != 0) {
            'l1: loop {
                'c2: {
                    F::Vec3_Add(c, (fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
                    F::Vec3_Add(c, (fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
                    { let v_ = rt::fadd(F32(c.m.r32((fp.wrapping_add(224)).wrapping_add(4))), F32(c.m.r32((fp.wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = fp.wrapping_add(224); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32((fp.wrapping_add(224)).wrapping_add(12))), F32(c.m.r32((fp.wrapping_add(240)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = (fp.wrapping_add(224)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    F::Vec4_ToInt(c, (fp.wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
                    F::Vec4_ToInt(c, (fp.wrapping_add(112)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
                    F::Vec4_ToInt(c, (fp.wrapping_add(112)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
                    F::Vec4_ToInt(c, (fp.wrapping_add(112)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
                    l_q_9 = c.m.r32(sym::gOtCur);
                    { let v_ = (l_q_9).wrapping_add((((1i32) as u32)).wrapping_mul(144)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
                    { let v_ = ((92i32) as u64); let a_ = (l_q_9).wrapping_add(32); c.m.w64(a_, v_); };
                    { let v_ = ((536870920i32) as u32); let a_ = l_q_9; c.m.w32(a_, v_); };
                    { let v_ = ((268435456i32) as u32); let a_ = (l_q_9).wrapping_add(8); c.m.w32(a_, v_); };
                    { let v_ = ((1342177288i32) as u32); let a_ = (l_q_9).wrapping_add(12); c.m.w32(a_, v_); };
                    { let v_ = 16429131440647602177u64; let a_ = (l_q_9).wrapping_add(16); c.m.w64(a_, v_); };
                    { let v_ = ((18599481537995104i64) as u64); let a_ = (l_q_9).wrapping_add(24); c.m.w64(a_, v_); };
                    { let v_ = ((0i32) as u32); let a_ = (l_q_9).wrapping_add(4); c.m.w32(a_, v_); };
                    { let v_ = (((c.m.r32(fp.wrapping_add(176)) as i32)) as u8); let a_ = (((l_q_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(176)).wrapping_add(4)) as i32)) as u8); let a_ = (((l_q_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(176)).wrapping_add(8)) as i32)) as u8); let a_ = (((l_q_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(176)).wrapping_add(12)) as i32)) as u8); let a_ = (((l_q_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = F32(0x3f800000); let a_ = (((l_q_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = (((c.m.r32(fp.wrapping_add(176)) as i32)) as u8); let a_ = (((l_q_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(176)).wrapping_add(4)) as i32)) as u8); let a_ = (((l_q_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(176)).wrapping_add(8)) as i32)) as u8); let a_ = (((l_q_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(176)).wrapping_add(12)) as i32)) as u8); let a_ = (((l_q_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = F32(0x3f800000); let a_ = (((l_q_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = (((c.m.r32(fp.wrapping_add(176)) as i32)) as u8); let a_ = (((l_q_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(176)).wrapping_add(4)) as i32)) as u8); let a_ = (((l_q_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(176)).wrapping_add(8)) as i32)) as u8); let a_ = (((l_q_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(176)).wrapping_add(12)) as i32)) as u8); let a_ = (((l_q_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = F32(0x3f800000); let a_ = (((l_q_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = (((c.m.r32(fp.wrapping_add(176)) as i32)) as u8); let a_ = (((l_q_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(176)).wrapping_add(4)) as i32)) as u8); let a_ = (((l_q_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(176)).wrapping_add(8)) as i32)) as u8); let a_ = (((l_q_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(176)).wrapping_add(12)) as i32)) as u8); let a_ = (((l_q_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = F32(0x3f800000); let a_ = (((l_q_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16)))); let a_ = (((l_q_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32(fp.wrapping_add(224))); let a_ = (((l_q_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32(((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_q_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((fp.wrapping_add(224)).wrapping_add(4))); let a_ = (((l_q_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(16)))); let a_ = (((l_q_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((fp.wrapping_add(224)).wrapping_add(8))); let a_ = (((l_q_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32(((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_q_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((fp.wrapping_add(224)).wrapping_add(12))); let a_ = (((l_q_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = (((((((((c.m.r32((fp.wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as i32)) as u16)) as i32)).wrapping_mul(16i32)).wrapping_add(28672i32)) as u64); let a_ = (((l_q_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((((((((c.m.r32(((fp.wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u16)) as i32)).wrapping_mul(16i32)).wrapping_add(29184i32)) as u64); let a_ = ((((l_q_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = ((l_zz_10) as u64); let a_ = ((((l_q_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                    { let v_ = ((255i32) as u64); let a_ = ((((l_q_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                    { let v_ = (((((((((c.m.r32((fp.wrapping_add(112)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as i32)) as u16)) as i32)).wrapping_mul(16i32)).wrapping_add(28672i32)) as u64); let a_ = (((l_q_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((((((((c.m.r32(((fp.wrapping_add(112)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u16)) as i32)).wrapping_mul(16i32)).wrapping_add(29184i32)) as u64); let a_ = ((((l_q_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = ((l_zz_10) as u64); let a_ = ((((l_q_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                    { let v_ = ((255i32) as u64); let a_ = ((((l_q_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                    { let v_ = (((((((((c.m.r32((fp.wrapping_add(112)).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as i32)) as u16)) as i32)).wrapping_mul(16i32)).wrapping_add(28672i32)) as u64); let a_ = (((l_q_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((((((((c.m.r32(((fp.wrapping_add(112)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u16)) as i32)).wrapping_mul(16i32)).wrapping_add(29184i32)) as u64); let a_ = ((((l_q_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = ((l_zz_10) as u64); let a_ = ((((l_q_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                    { let v_ = ((255i32) as u64); let a_ = ((((l_q_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                    { let v_ = (((((((((c.m.r32((fp.wrapping_add(112)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as i32)) as u16)) as i32)).wrapping_mul(16i32)).wrapping_add(28672i32)) as u64); let a_ = (((l_q_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((((((((c.m.r32(((fp.wrapping_add(112)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u16)) as i32)).wrapping_mul(16i32)).wrapping_add(29184i32)) as u64); let a_ = ((((l_q_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = p_tex0_6; let a_ = (l_q_9).wrapping_add(40); c.m.w64(a_, v_); };
                    { let v_ = ((l_zz_10) as u64); let a_ = ((((l_q_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                    { let v_ = ((255i32) as u64); let a_ = ((((l_q_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                    F::eft_t_b__EftTOt_Add(c, l_q_9, l_slot_8, p_blend_3);
                    F::Vec4_Copy(c, (fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
                    F::Vec4_Copy(c, (fp.wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
                    { let v_ = F32(c.m.r32(fp.wrapping_add(224))); let a_ = (fp.wrapping_add(224)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((fp.wrapping_add(224)).wrapping_add(8))); let a_ = (fp.wrapping_add(224)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                }
                if !((((({ let o_ = p_segs_4; let n_ = (o_).wrapping_sub(1); p_segs_4 = n_; n_ }) != (0i32)) as i32)) != 0) { break 'l1; }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_DrawScreen", &PROBE_N, &[], true); }
    r_
}

pub fn EftStreak_StepTexture(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_StepTexture", &PROBE_N, &[(p_w_1) as u32], false); }
    let mut l_mgr_2: u32 = 0;
    let mut l_tex0_3: u64 = 0;
    let r_ = 'f: {
    l_mgr_2 = { let a_ = c.m.r32(sym::gEftStreak); c.m.r32(a_) };
    if ((((!(((((c.m.r32((l_mgr_2).wrapping_add(132)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        l_tex0_3 = F::EftVram_AddImage(c, (p_w_1).wrapping_add(8), 1i32, 0i32);
        { let r_ = (F::EftVram_AddClut(c, (p_w_1).wrapping_add(24))).wrapping_shl((37i32) as u32); let n_ = ((l_tex0_3) | (r_)); l_tex0_3 = n_; };
        { let v_ = l_tex0_3; let a_ = (l_mgr_2).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w64(a_, v_); };
        { let r_ = 1i32; let a_ = (l_mgr_2).wrapping_add(132); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_StepTexture", &PROBE_N, &[], true); }
    r_
}

pub fn EftStreak_Start(c: &mut Ctx, mut p_objId_1: i32, mut p_kind_2: i32, mut p_color_3: i32, mut p_angle_4: F32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_Start", &PROBE_N, &[(p_objId_1) as u32, (p_kind_2) as u32, (p_color_3) as u32, (p_angle_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let r_ = 'f: {
        if (((((c.m.r32(sym::gEftStreak)) == (((0i32) as u32))) as i32)) != 0) {
            break 'f ((0i32) as u32);
        }
        if ((((({ let a_ = c.m.r32(sym::gEftStreak); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
            break 'f ((0i32) as u32);
        }
        if ((((({ let a_ = ({ let a_ = c.m.r32(sym::gEftStreak); c.m.r32(a_) }).wrapping_add(136); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
            break 'f ((0i32) as u32);
        }
        if (((((p_kind_2) >= (10i32)) as i32)) != 0) {
            break 'f ((0i32) as u32);
        }
        { let v_ = { let a_ = (({ let a_ = c.m.r32(sym::gEftStreak); c.m.r32(a_) }).wrapping_add(140)).wrapping_add((((p_kind_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = fp; c.m.w32(a_, v_); };
        { let v_ = { let a_ = ({ let a_ = c.m.r32(sym::gEftStreak); c.m.r32(a_) }).wrapping_add(180); c.m.r32(a_) }; let a_ = (fp).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = p_objId_1; let a_ = (fp).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
        { let v_ = p_kind_2; let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = p_angle_4; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(24); c.m.w32(a_, (v_).0); };
        if (((((p_color_3) < (7i32)) as i32)) != 0) {
            { let v_ = ((p_color_3) & (255i32)); let a_ = (fp).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
        } else {
            if (((((p_color_3) == (7i32)) as i32)) != 0) {
                { let v_ = 1i32; let a_ = (fp).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = 2i32; let a_ = (fp).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 'f { let a0_ = { let a_ = ({ let a_ = c.m.r32(sym::gEftStreak); c.m.r32(a_) }).wrapping_add(136); c.m.r32(a_) }; F::BtlTaskList_AddTail(c, a0_, sym::gEftStreakClass, fp) };
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_Start", &PROBE_N, &[], true); }
    r_
}

pub fn EftStreak_Stop(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_Stop", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((((((((((c.m.r32(sym::gEftStreak)) != (((0i32) as u32))) as i32)) != 0) && ((((({ let a_ = c.m.r32(sym::gEftStreak); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) && (((((p_task_1) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftStreak_Update)) as i32)) != 0) {
            l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
            if (((((l_w_2) != (((0i32) as u32))) as i32)) != 0) {
                if (((((c.m.r32((l_w_2).wrapping_add(108)) as i32)) & (2i32))) != 0) {
                    { let r_ = 4i32; let a_ = (l_w_2).wrapping_add(108); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_Stop", &PROBE_N, &[], true); }
    r_
}

pub fn EftStreak_SetPos(c: &mut Ctx, mut p_task_1: u32, p_pos_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_SetPos", &PROBE_N, &[(p_task_1) as u32, (p_pos_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_3: u32 = 0;
    rt::memcpy(c, fp, p_pos_2, 16);
    let r_ = 'f: {
        if (((((((((((((c.m.r32(sym::gEftStreak)) != (((0i32) as u32))) as i32)) != 0) && ((((({ let a_ = c.m.r32(sym::gEftStreak); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) && (((((p_task_1) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftStreak_Update)) as i32)) != 0) {
                l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
                if (((((l_w_3) != (((0i32) as u32))) as i32)) != 0) {
                    if (((((c.m.r32((l_w_3).wrapping_add(108)) as i32)) & (2i32))) != 0) {
                        F::Vec4_Copy(c, (l_w_3).wrapping_add(48), fp);
                        { let r_ = 2048i32; let a_ = (l_w_3).wrapping_add(108); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_SetPos", &PROBE_N, &[], true); }
    r_
}

pub fn EftStreak_SetCover(c: &mut Ctx, mut p_task_1: u32, mut p_cover_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_SetCover", &PROBE_N, &[(p_task_1) as u32, (p_cover_2).0], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((((((((((c.m.r32(sym::gEftStreak)) != (((0i32) as u32))) as i32)) != 0) && ((((({ let a_ = c.m.r32(sym::gEftStreak); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) && (((((p_task_1) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftStreak_Update)) as i32)) != 0) {
            l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
            if (((((l_w_3) != (((0i32) as u32))) as i32)) != 0) {
                if (((((c.m.r32((l_w_3).wrapping_add(108)) as i32)) & (2i32))) != 0) {
                    { let v_ = p_cover_2; let a_ = (l_w_3).wrapping_add(84); c.m.w32(a_, (v_).0); };
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_SetCover", &PROBE_N, &[], true); }
    r_
}

pub fn EftStreak_IsAlive(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_IsAlive", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftStreak)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = c.m.r32(sym::gEftStreak); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftStreak_Update)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32((l_w_2).wrapping_add(108)) as i32)) & (2i32))) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_IsAlive", &PROBE_N, &[], true); }
    r_
}

pub fn EftStreak_NopA(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_NopA", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_NopA", &PROBE_N, &[], true); }
    r_
}

pub fn EftStreak_NopB(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_NopB", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStreak_NopB", &PROBE_N, &[], true); }
    r_
}
