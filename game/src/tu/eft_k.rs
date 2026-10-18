#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EftShotTech_UpdateAuraBall(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_UpdateAuraBall", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_w_2: u32 = 0;
    let mut l_src_3: u32 = 0;
    let mut l_fx_4: u32 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
        'fl1: loop {
            match bb_ {
                0 => {
                    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
                    l_src_3 = c.m.r32((l_w_2).wrapping_add(1504));
                    if (((((({ let a0_ = (c.m.r32(l_src_3) as i32); F::EftShot_TestBits(c, a0_, 256i32) }) != 0) && (((((c.m.r32((l_w_2).wrapping_add(2236))) == (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) { bb_ = 1; continue 'fl1; } else { bb_ = 2; continue 'fl1; }
                }
                1 => {
                    l_fx_4 = { let a0_ = (c.m.r32(l_src_3) as i32); let a1_ = { let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_GetAuraType(c, a0_) }; F::EftOrbTail_Create(c, a0_, a1_) };
                    { let v_ = l_fx_4; let a_ = (l_w_2).wrapping_add(2236); c.m.w32(a_, v_); };
                    if (((((l_fx_4) == (((0i32) as u32))) as i32)) != 0) { bb_ = 3; continue 'fl1; } else { bb_ = 4; continue 'fl1; }
                }
                2 => {
                    if (((((c.m.r32((l_w_2).wrapping_add(2236))) != (((0i32) as u32))) as i32)) != 0) { bb_ = 7; continue 'fl1; } else { bb_ = 8; continue 'fl1; }
                }
                3 => {
                    bb_ = 5; continue 'fl1;
                }
                4 => {
                    { let _ = F::EftOrbTail_Burst(c, l_fx_4); };
                    { let a0_ = (c.m.r32(l_src_3) as i32); { let _ = F::EftGlow_Request(c, a0_, 0i32); } };
                    bb_ = 2; continue 'fl1;
                }
                5 => {
                    if (((((({ let a0_ = (c.m.r32(l_src_3) as i32); F::EftShot_TestBits(c, a0_, 512i32) }) != 0) && (((((c.m.r32((l_w_2).wrapping_add(2236))) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) { bb_ = 9; continue 'fl1; } else { bb_ = 10; continue 'fl1; }
                }
                6 => {
                    bb_ = 4; continue 'fl1;
                }
                7 => {
                    { let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 10i32, fp) };
                    { let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 14i32, fp.wrapping_add(16)) };
                    F::Vec3_Add(c, fp, fp, fp.wrapping_add(16));
                    F::Vec3_Scale(c, fp, fp, F32(0x3f000000));
                    { let a0_ = c.m.r32((l_w_2).wrapping_add(2236)); F::EftOrbTail_SetPos(c, a0_, fp) };
                    bb_ = 8; continue 'fl1;
                }
                8 => {
                    bb_ = 5; continue 'fl1;
                }
                9 => {
                    { let a0_ = c.m.r32((l_w_2).wrapping_add(2236)); { let _ = F::EftOrbTail_Burst(c, a0_); } };
                    { let a0_ = c.m.r32((l_w_2).wrapping_add(2236)); { let _ = F::EftOrbTail_Kill(c, a0_); } };
                    { let v_ = ((0i32) as u32); let a_ = (l_w_2).wrapping_add(2236); c.m.w32(a_, v_); };
                    { let a0_ = (c.m.r32(l_src_3) as i32); { let _ = F::EftGlow_Request(c, a0_, 1i32); } };
                    bb_ = 10; continue 'fl1;
                }
                10 => {
                    break 'fl1;
                }
                _ => unreachable!(),
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_UpdateAuraBall", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotTech_UpdateTargetBurst(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_UpdateTargetBurst", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_w_2: u32 = 0;
    let mut l_src_3: u32 = 0;
    let mut l_opp_4: i32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_src_3 = c.m.r32((l_w_2).wrapping_add(1504));
        l_opp_4 = { let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_GetOpponentObjId(c, a0_) };
        if (((((((({ let a_ = (c.m.r32((l_src_3).wrapping_add(36))).wrapping_add(60); (c.m.r32(a_) as i32) }) & (1024i32))) != 0) && ((F::EftShot_TestBits(c, l_opp_4, 4i32)) != 0)) as i32)) != 0) {
            { let s_ = sym::D_002ECB60; let d_ = fp.wrapping_add(16); rt::memcpy(c, d_, s_, 16); };
            F::BtlCharApi_GetNodePos(c, l_opp_4, 3i32, fp);
            { let a0_ = (c.m.r32(l_src_3) as i32); let a4_ = rt::fmul(F::BtlCharApi_GetHeight(c, l_opp_4), F32(0x3cf5c28f)); F::EftGndDust_SpawnLandingScaled(c, a0_, fp, fp.wrapping_add(16), F32(0x3f800000), a4_) };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_UpdateTargetBurst", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotTech_UpdateMuzzle(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_UpdateMuzzle", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_w_2: u32 = 0;
    let mut l_src_3: u32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_src_3 = c.m.r32((l_w_2).wrapping_add(1504));
        if (((((((((({ let a0_ = c.m.r32(l_w_2); ((((F::EftEmit_GetHead26(c, a0_)) as i64 as u64)) as u8) }) as i32)) & (1i32))) != 0) && (({ let a0_ = (c.m.r32(l_src_3) as i32); F::EftShot_TestBits(c, a0_, 32i32) }) != 0)) as i32)) != 0) {
            { let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_GetNodeMtx(c, a0_, 54i32, fp) };
            { let a1_ = rt::fneg(F32(c.m.r32(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a2_ = rt::fneg(F32(c.m.r32(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a3_ = rt::fneg(F32(c.m.r32(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); F::Vec4_Set(c, fp.wrapping_add(80), a1_, a2_, a3_, F32(0x3f800000)) };
            F::Vec3_Normalize(c, fp.wrapping_add(80), fp.wrapping_add(80));
            { let a1_ = F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a2_ = F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a3_ = F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); F::Vec4_Set(c, fp.wrapping_add(64), a1_, a2_, a3_, F32(0x3f800000)) };
            F::Vec4_Copy(c, (l_w_2).wrapping_add(1376), fp.wrapping_add(80));
            F::Vec4_Copy(c, (l_w_2).wrapping_add(1392), fp.wrapping_add(64));
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_UpdateMuzzle", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotTech_StartCam(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_StartCam", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_w_2: u32 = 0;
    let mut l_src_3: u32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_src_3 = c.m.r32((l_w_2).wrapping_add(1504));
        if (((((c.m.r32((l_w_2).wrapping_add(1408)) as i32)) & (2048i32))) != 0) {
            { let v_ = c.m.r32((l_w_2).wrapping_add(2196)); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
            { let v_ = (c.m.r32(l_src_3) as i32); let a_ = (fp.wrapping_add(12)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            { let v_ = (({ let a_ = c.m.r32((l_src_3).wrapping_add(36)); (c.m.r16(a_) as i16) }) as i32); let a_ = (fp.wrapping_add(12)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let s_ = fp.wrapping_add(12); let d_ = fp; rt::memcpy(c, d_, s_, 12); };
            F::EftCam_Start(c, fp);
            { let r_ = 32i32; let a_ = (l_w_2).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_StartCam", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotTech_MoveStraight(c: &mut Ctx, mut p_task_1: u32, mut p_toTarget_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_MoveStraight", &PROBE_N, &[(p_task_1) as u32, (p_toTarget_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let mut l_src_4: u32 = 0;
    let mut l_pos_5: u32 = 0;
    let mut l_opp_6: i32 = 0;
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    l_src_4 = c.m.r32((l_w_3).wrapping_add(1504));
    l_pos_5 = (l_w_3).wrapping_add(1456);
    l_opp_6 = { let a0_ = (c.m.r32(l_src_4) as i32); F::BtlCharApi_GetOpponentObjId(c, a0_) };
    if ((((((c.m.r32((l_w_3).wrapping_add(1416)) as i32)) == (0i32)) as i32)) != 0) {
        { let a0_ = (c.m.r32(l_src_4) as i32); let a1_ = (c.m.r32((l_w_3).wrapping_add(1412)) as i32); F::BtlCharApi_GetNodePos(c, a0_, a1_, (l_w_3).wrapping_add(1360)) };
        F::Vec4_Copy(c, (l_w_3).wrapping_add(1472), l_pos_5);
        if (((((p_toTarget_2) == (0i32)) as i32)) != 0) {
            F::Vec3_Add(c, l_pos_5, l_pos_5, (l_w_3).wrapping_add(1488));
        } else {
            F::BtlCharApi_GetNodePos(c, l_opp_6, 3i32, l_pos_5);
        }
        if ((F::EftShot_TestBits(c, l_opp_6, 64i32)) != 0) {
            { let v_ = 1i32; let a_ = (l_w_3).wrapping_add(1416); c.m.w32(a_, (v_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_MoveStraight", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotTech_MoveFromTarget(c: &mut Ctx, mut p_task_1: u32, mut p_wait_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_MoveFromTarget", &PROBE_N, &[(p_task_1) as u32, (p_wait_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_w_3: u32 = 0;
    let mut l_src_4: u32 = 0;
    let mut l_opp_5: i32 = 0;
    let mut l_dist_6: F32 = F32(0);
    let mut l_pos_7: u32 = 0;
    let mut l_go_8: i32 = 0;
    let r_ = 'f: {
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        l_src_4 = c.m.r32((l_w_3).wrapping_add(1504));
        l_opp_5 = { let a0_ = (c.m.r32(l_src_4) as i32); F::BtlCharApi_GetOpponentObjId(c, a0_) };
        l_dist_6 = { let a0_ = c.m.r32(l_w_3); F::EftEmit_GetHead28(c, a0_) };
        's1: {
            let sv9: i32 = (c.m.r32((l_w_3).wrapping_add(1416)) as i32);
            match sv9 {
                0i32 => {
                    {
                        { let a0_ = (c.m.r32(l_src_4) as i32); let a1_ = (c.m.r32((l_w_3).wrapping_add(1412)) as i32); F::BtlCharApi_GetNodePos(c, a0_, a1_, (l_w_3).wrapping_add(1360)) };
                        l_pos_7 = (l_w_3).wrapping_add(1456);
                        l_go_8 = 0i32;
                        F::Vec4_Copy(c, (l_w_3).wrapping_add(1472), l_pos_7);
                        F::Vec3_Add(c, l_pos_7, l_pos_7, (l_w_3).wrapping_add(1488));
                        if (((((((((p_wait_2) == (0i32)) as i32)) != 0) || (({ let a0_ = (c.m.r32(l_src_4) as i32); F::EftShot_TestBits(c, a0_, 16i32) }) != 0)) as i32)) != 0) {
                            l_go_8 = 1i32;
                        }
                        if ((l_go_8) != 0) {
                            F::BtlCharApi_GetNodePos(c, l_opp_5, 17i32, fp.wrapping_add(16));
                            { let a0_ = (c.m.r32(l_src_4) as i32); F::BtlCharApi_GetNodePos(c, a0_, 17i32, fp.wrapping_add(32)) };
                            F::Vec3_Sub(c, fp, fp.wrapping_add(32), fp.wrapping_add(16));
                            F::Vec3_Normalize(c, fp, fp);
                            F::Vec3_Scale(c, fp, fp, l_dist_6);
                            F::Vec3_Add(c, fp.wrapping_add(16), fp.wrapping_add(16), fp);
                            F::Vec4_Copy(c, l_pos_7, fp.wrapping_add(16));
                            { let v_ = 1i32; let a_ = (l_w_3).wrapping_add(1416); c.m.w32(a_, (v_) as u32); };
                        }
                        break 's1;
                    }
                }
                1i32 => {
                    F::BtlCharApi_GetNodePos(c, l_opp_5, 17i32, fp.wrapping_add(16));
                    { let a0_ = (c.m.r32(l_src_4) as i32); F::BtlCharApi_GetNodePos(c, a0_, 17i32, fp.wrapping_add(32)) };
                    F::Vec3_Sub(c, fp, fp.wrapping_add(32), fp.wrapping_add(16));
                    F::Vec3_Normalize(c, fp, fp);
                    F::Vec3_Scale(c, fp, fp, l_dist_6);
                    F::Vec3_Add(c, fp.wrapping_add(16), fp.wrapping_add(16), fp);
                    F::Vec4_Copy(c, (l_w_3).wrapping_add(1456), fp.wrapping_add(16));
                    if (({ let a0_ = (c.m.r32(l_src_4) as i32); F::EftShot_TestBits(c, a0_, 32i32) }) != 0) {
                        { let r_ = 256i32; let a_ = (l_w_3).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    if (((((c.m.r32((l_w_3).wrapping_add(1408)) as i32)) & (256i32))) != 0) {
                        { let r_ = F32(0x3d4ccccc); let a_ = (l_w_3).wrapping_add(1432); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        if (((rt::fgt(F32(c.m.r32((l_w_3).wrapping_add(1432))), F32(0x40000000)) as i32)) != 0) {
                            { let v_ = F32(0x40000000); let a_ = (l_w_3).wrapping_add(1432); c.m.w32(a_, (v_).0); };
                        }
                    }
                    if ((F::EftShot_TestBits(c, l_opp_5, 64i32)) != 0) {
                        { let v_ = 2i32; let a_ = (l_w_3).wrapping_add(1416); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                2i32 => {
                    break 's1;
                }
                _ => {
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_MoveFromTarget", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotTech_MoveAimed(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_MoveAimed", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_2: u32 = 0;
    let mut l_src_3: u32 = 0;
    let mut l_opp_4: i32 = 0;
    let mut l_dist_5: F32 = F32(0);
    let mut l_start_6: u32 = 0;
    let mut l_pos_7: u32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_src_3 = c.m.r32((l_w_2).wrapping_add(1504));
        l_opp_4 = { let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_GetOpponentObjId(c, a0_) };
        if ((((((c.m.r32((l_w_2).wrapping_add(1416)) as i32)) == (0i32)) as i32)) != 0) {
            if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::EftShot_TestBits(c, a0_, 16i32) }) != 0) {
                { let r_ = 16384i32; let a_ = (l_w_2).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
            l_start_6 = (l_w_2).wrapping_add(1360);
            l_pos_7 = (l_w_2).wrapping_add(1456);
            { let a0_ = (c.m.r32(l_src_3) as i32); let a1_ = (c.m.r32((l_w_2).wrapping_add(1412)) as i32); F::BtlCharApi_GetNodePos(c, a0_, a1_, l_start_6) };
            F::Vec4_Copy(c, (l_w_2).wrapping_add(1472), l_pos_7);
            if ((((!(((((c.m.r32((l_w_2).wrapping_add(1408)) as i32)) & (16384i32))) != 0)) as i32)) != 0) {
                F::BtlCharApi_GetNodePos(c, l_opp_4, 3i32, l_pos_7);
                F::Vec3_Sub(c, fp, l_pos_7, l_start_6);
                l_dist_5 = rt::fadd({ let a0_ = F::Vec3_Dot(c, fp, fp); F::Port_sqrtf(c, a0_) }, F32(0x41f00000));
                F::Vec3_Normalize(c, fp, fp);
                F::Vec3_ScaleAdd(c, l_pos_7, fp, l_start_6, l_dist_5);
            } else {
                l_dist_5 = F32(0x41f00000);
                { let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 31i32, l_pos_7) };
                F::Vec3_Sub(c, fp, l_pos_7, l_start_6);
                if ((((((({ let a_ = c.m.r32((l_src_3).wrapping_add(36)); (c.m.r16(a_) as i16) }) as i32)) == (692i32)) as i32)) != 0) {
                    l_dist_5 = F32(0x418fffff);
                }
                F::Vec3_Normalize(c, fp, fp);
                F::Vec3_ScaleAdd(c, l_pos_7, fp, l_start_6, l_dist_5);
            }
            F::Vec4_Copy(c, (l_w_2).wrapping_add(1312), fp);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_MoveAimed", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotTech_UpdateConnected(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_UpdateConnected", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_owner_2: u32 = 0;
    let mut l_w_3: u32 = 0;
    let mut l_mgr_4: u32 = 0;
    let mut l_src_5: u32 = 0;
    let r_ = 'f: {
        l_owner_2 = F::BtlTask_GetParent(c, p_task_1);
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        l_mgr_4 = c.m.r32((l_owner_2).wrapping_add(56));
        l_src_5 = c.m.r32((l_w_3).wrapping_add(1504));
        F::EftShotTech_UpdateFlash(c, p_task_1);
        F::EftShotTech_UpdateNodeFx(c, p_task_1);
        F::EftShotTech_UpdateBlur(c, p_task_1);
        F::EftShotTech_UpdateSubs(c, p_task_1);
        F::EftShotTech_UpdateAuraBall(c, p_task_1);
        F::EftShotTech_UpdateTargetBurst(c, p_task_1);
        F::EftShotTech_UpdateMuzzle(c, p_task_1);
        if (((((c.m.r32((l_mgr_4).wrapping_add(812))) != (((0i32) as u32))) as i32)) != 0) {
            if (({ let a0_ = (c.m.r32(l_src_5) as i32); F::EftShot_TestBits(c, a0_, 16i32) }) != 0) {
                { let r_ = 1048576i32; let a_ = (l_w_3).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
            if (({ let a0_ = (c.m.r32(l_src_5) as i32); F::EftShot_TestBits(c, a0_, 32i32) }) != 0) {
                { let r_ = (!(1048576i32)); let a_ = (l_w_3).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            }
            if (((((c.m.r32((l_w_3).wrapping_add(1408)) as i32)) & (1048576i32))) != 0) {
                { let a0_ = (c.m.r32(l_src_5) as i32); F::BtlCharApi_GetNodePos(c, a0_, 54i32, fp) };
                F::Vec3_Clamp(c, fp.wrapping_add(32), (l_w_3).wrapping_add(1312), F32(0xbf800000), F32(0x3f800000));
                { let v_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4))); F::Mathf_Asin(c, a0_) }; let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
                { let v_ = { let a0_ = F32(c.m.r32(fp.wrapping_add(32))); let a1_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) }; let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = { let a0_ = F32(c.m.r32(fp.wrapping_add(16))); F::EftMath_WrapAngle(c, a0_) }; let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
                { let v_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); F::EftMath_WrapAngle(c, a0_) }; let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                F::EftShotTech_SetModelPose(c, p_task_1, fp, fp.wrapping_add(16));
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_UpdateConnected", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotTech_Init(c: &mut Ctx, mut p_task_1: u32, mut p_src_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_Init", &PROBE_N, &[(p_task_1) as u32, (p_src_2) as u32], false); }
    let mut l_owner_3: u32 = 0;
    let mut l_w_4: u32 = 0;
    let mut l_mgr_5: u32 = 0;
    let mut l_def_6: u32 = 0;
    let mut l_head_7: i32 = 0;
    let mut l_pack_8: u32 = 0;
    let r_ = 'f: {
    l_owner_3 = F::BtlTask_GetParent(c, p_task_1);
    l_w_4 = c.m.r32((p_task_1).wrapping_add(56));
    l_mgr_5 = c.m.r32((l_owner_3).wrapping_add(56));
    { let _ = F::memset(c, l_w_4, 0i32, 2304u32); };
    { let v_ = p_src_2; let a_ = (l_w_4).wrapping_add(1504); c.m.w32(a_, v_); };
    l_def_6 = c.m.r32((p_src_2).wrapping_add(36));
    { let v_ = F32(c.m.r32((l_def_6).wrapping_add(76))); let a_ = (l_w_4).wrapping_add(1428); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_def_6).wrapping_add(48))); let a_ = (l_w_4).wrapping_add(1432); c.m.w32(a_, (v_).0); };
    { let v_ = l_mgr_5; let a_ = l_w_4; c.m.w32(a_, v_); };
    F::EftEmit_InitState(c, l_mgr_5, (l_w_4).wrapping_add(4));
    l_head_7 = (({ let a0_ = c.m.r32(l_w_4); ((((F::EftEmit_GetHead26(c, a0_)) as i64 as u64)) as u8) }) as i32);
    if (((((((((({ let a0_ = c.m.r32(l_w_4); ((((F::EftEmit_GetHead21(c, a0_)) as i64 as u64)) as u8) }) as i32)) & (1i32))) != 0) && ((((((({ let a0_ = c.m.r32(l_w_4); ((((F::EftEmit_GetHead22(c, a0_)) as i64 as u64)) as u8) }) as i32)) == (2i32)) as i32)) != 0)) as i32)) != 0) {
        { let v_ = 21i32; let a_ = (l_w_4).wrapping_add(1412); c.m.w32(a_, (v_) as u32); };
        { let r_ = 8192i32; let a_ = (l_w_4).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if ((((l_head_7) & (32i32))) != 0) {
        { let r_ = 131072i32; let a_ = (l_w_4).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if ((((((c.m.r32((p_src_2).wrapping_add(4)) as i32)) >= (2i32)) as i32)) != 0) {
        l_pack_8 = { let a_ = ((l_mgr_5).wrapping_add(792)).wrapping_add(((((c.m.r32((p_src_2).wrapping_add(4)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) };
        if (((((l_pack_8) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = l_pack_8; let a_ = (l_w_4).wrapping_add(2196); c.m.w32(a_, v_); };
        }
    }
    if (((((c.m.r32((l_mgr_5).wrapping_add(812))) != (((0i32) as u32))) as i32)) != 0) {
        F::EftShotTech_InitModel(c, p_task_1);
    }
    { let a1_ = (if ((((((c.m.r32(p_src_2) as i32)) == (0i32)) as i32)) != 0) { 2048i32 } else { 4096i32 }); F::BtlTask_SetOwnerTag(c, p_task_1, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotTech_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_owner_2: u32 = 0;
    let mut l_w_3: u32 = 0;
    let mut l_src_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
    l_owner_2 = F::BtlTask_GetParent(c, p_task_1);
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    l_src_4 = c.m.r32((l_w_3).wrapping_add(1504));
    if ((((({ let a_ = (c.m.r32((l_owner_2).wrapping_add(56))).wrapping_add(812); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        F::EftShotTech_FreeModel(c, p_task_1);
    }
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((l_i_5) < (8i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((c.m.r32(((l_w_3).wrapping_add(2200)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
                    { let a0_ = c.m.r32(((l_w_3).wrapping_add(2200)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))); F::EftStreak_Stop(c, a0_) };
                }
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
    if (((((c.m.r32((l_w_3).wrapping_add(2236))) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32((l_w_3).wrapping_add(2236)); { let _ = F::EftOrbTail_Kill(c, a0_); } };
        { let v_ = ((0i32) as u32); let a_ = (l_w_3).wrapping_add(2236); c.m.w32(a_, v_); };
        { let a0_ = (c.m.r32(l_src_4) as i32); { let _ = F::EftGlow_Request(c, a0_, 2i32); } };
    }
    { let a0_ = c.m.r32(l_w_3); F::EftEmit_TermState(c, a0_, (l_w_3).wrapping_add(4)) };
    { let a0_ = (c.m.r32(l_src_4) as i32); F::EftShot_SetHeldFlagA8(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotTech_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_advance_2: i32 = 0;
    let mut l_busy_3: i32 = 0;
    let mut l_w_4: u32 = 0;
    let mut l_src_5: u32 = 0;
    let mut l_head_6: i32 = 0;
    let mut l_kind_7: i32 = 0;
    let mut l_toTarget_8: u8 = 0;
    let mut l_wait_9: u8 = 0;
    let r_ = 'f: {
        l_advance_2 = 0i32;
        l_busy_3 = 0i32;
        l_w_4 = c.m.r32((p_task_1).wrapping_add(56));
        l_src_5 = c.m.r32((l_w_4).wrapping_add(1504));
        if (({ let a0_ = (c.m.r32(l_src_5) as i32); F::BtlScene_IsCharStopped(c, a0_) }) != 0) {
            break 'f;
        }
        if (({ let a0_ = (c.m.r32(l_src_5) as i32); F::BtlCharApi_IsRushConnected(c, a0_) }) != 0) {
            { let r_ = 2048i32; let a_ = (l_w_4).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if ((((!(((((c.m.r32((l_w_4).wrapping_add(1408)) as i32)) & (32i32))) != 0)) as i32)) != 0) {
            F::EftShotTech_StartCam(c, p_task_1);
        }
        l_head_6 = (({ let a0_ = c.m.r32(l_w_4); ((((F::EftEmit_GetHead26(c, a0_)) as i64 as u64)) as u8) }) as i32);
        F::EftEmit_UpdateNodesReq(c, l_src_5, (l_w_4).wrapping_add(720));
        if ((((!(((((c.m.r32((l_w_4).wrapping_add(1408)) as i32)) & (4096i32))) != 0)) as i32)) != 0) {
            if (({ let a0_ = (c.m.r32(l_src_5) as i32); F::EftShot_TestBits(c, a0_, 2i32) }) != 0) {
                F::EftShotTech_Start(c, p_task_1, 2i32);
                { let v_ = ((0i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                { let r_ = 4096i32; let a_ = (l_w_4).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            } else {
                if (({ let a0_ = (c.m.r32(l_src_5) as i32); F::EftShot_TestBits(c, a0_, 4i32) }) != 0) {
                    F::EftShotTech_Start(c, p_task_1, 4i32);
                    { let v_ = ((2i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                    { let r_ = 4096i32; let a_ = (l_w_4).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((l_head_6) & (4i32))) != 0) {
                        if (({ let a0_ = (c.m.r32(l_src_5) as i32); F::EftShot_TestBits(c, a0_, 16i32) }) != 0) {
                            F::EftShotTech_Start(c, p_task_1, 16i32);
                            { let v_ = ((2i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                            { let r_ = 4096i32; let a_ = (l_w_4).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                    }
                }
            }
        }
        's1: {
            let sv10: i32 = ((c.m.r8((p_task_1).wrapping_add(1))) as i32);
            match sv10 {
                0i32 => {
                    if (({ let a0_ = (c.m.r32(l_src_5) as i32); F::EftShot_TestBits(c, a0_, 4i32) }) != 0) {
                        F::EftShotTech_Start(c, p_task_1, 4i32);
                        { let v_ = ((1i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                    }
                    break 's1;
                }
                1i32 => {
                    l_advance_2 = 1i32;
                    break 's1;
                }
                2i32 => {
                    if (((({ let a_ = (c.m.r32((l_src_5).wrapping_add(36))).wrapping_add(60); (c.m.r32(a_) as i32) }) & (16i32))) != 0) {
                        F::Vec4_Scale(c, fp, (l_w_4).wrapping_add(1312), F32(0xbf800000));
                    }
                    if (((((({ let a0_ = c.m.r32(l_w_4); ((((F::EftEmit_GetHead21(c, a0_)) as i64 as u64)) as u8) }) as i32)) & (1i32))) != 0) {
                        l_kind_7 = (({ let a0_ = c.m.r32(l_w_4); ((((F::EftEmit_GetHead22(c, a0_)) as i64 as u64)) as u8) }) as i32);
                        l_toTarget_8 = (((((((l_head_6) & (2i32))) > (0i32)) as i32)) as u8);
                        l_wait_9 = (((((((l_head_6) & (8i32))) > (0i32)) as i32)) as u8);
                        's2: {
                            let sv11: i32 = l_kind_7;
                            match sv11 {
                                0i32 => {
                                    F::EftShotTech_MoveStraight(c, p_task_1, ((l_toTarget_8) as i32));
                                    break 's2;
                                }
                                1i32 => {
                                    F::EftShotTech_MoveFromTarget(c, p_task_1, ((l_wait_9) as i32));
                                    break 's2;
                                }
                                2i32 => {
                                    F::EftShotTech_MoveAimed(c, p_task_1);
                                    break 's2;
                                }
                                _ => {
                                }
                            }
                        }
                    }
                    if (({ let a0_ = (c.m.r32(l_src_5) as i32); F::EftShot_TestBits(c, a0_, 8i32) }) != 0) {
                        { let r_ = 9i32; let a_ = (l_w_4).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        { let v_ = ((3i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                    }
                    break 's1;
                }
                3i32 => {
                    l_advance_2 = 1i32;
                    break 's1;
                }
                _ => {
                }
            }
        }
        if (((((c.m.r32((l_w_4).wrapping_add(1408)) as i32)) & (2048i32))) != 0) {
            if ((((l_head_6) & (16i32))) != 0) {
                { let a0_ = (c.m.r32(l_src_5) as i32); F::BtlCharApi_GetNodePos(c, a0_, 54i32, (l_w_4).wrapping_add(1456)) };
            }
            if ((((l_head_6) & (64i32))) != 0) {
                F::EftShotTech_CalcDir(c, p_task_1, (l_w_4).wrapping_add(1312));
            }
            if (((((c.m.r32((l_w_4).wrapping_add(1408)) as i32)) & (2048i32))) != 0) {
                F::EftShotTech_UpdateConnected(c, p_task_1);
            }
        }
        if (((((c.m.r32((l_w_4).wrapping_add(1408)) as i32)) & (131072i32))) != 0) {
            l_busy_3 = { let a0_ = (c.m.r32(l_src_5) as i32); F::EftShotTech_UpdateShots(c, a0_, p_task_1) };
        }
        if (({ let a0_ = (c.m.r32(l_src_5) as i32); F::EftShot_TestBits(c, a0_, 1024i32) }) != 0) {
            if ((((!(((((c.m.r32((l_w_4).wrapping_add(1408)) as i32)) & (8i32))) != 0)) as i32)) != 0) {
                { let r_ = 4i32; let a_ = (l_w_4).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
            { let r_ = 1i32; let a_ = (l_w_4).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        { let a0_ = (c.m.r32(l_src_5) as i32); let a2_ = c.m.r32(l_w_4); F::EftShotTech_UpdateParts(c, a0_, p_task_1, a2_) };
        if ((l_advance_2) != 0) {
            { let a_ = (p_task_1).wrapping_add(1); let o_ = c.m.r8(a_); let n_ = (o_).wrapping_add(1); c.m.w8(a_, n_); };
        }
        if ((((((((!((l_busy_3) != 0)) as i32)) != 0) && (((((c.m.r32((l_w_4).wrapping_add(1408)) as i32)) & (2i32))) != 0)) as i32)) != 0) {
            F::BtlTask_SetDead(c, p_task_1);
            break 'f;
        }
        if (((((c.m.r32((l_w_4).wrapping_add(1408)) as i32)) & (1i32))) != 0) {
            if (((((c.m.r32((l_w_4).wrapping_add(1408)) as i32)) & (32i32))) != 0) {
                if ((((!(((((c.m.r32((l_w_4).wrapping_add(1408)) as i32)) & (64i32))) != 0)) as i32)) != 0) {
                    F::EftCam_Stop(c);
                    { let r_ = 64i32; let a_ = (l_w_4).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
            { let r_ = 2i32; let a_ = (l_w_4).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotTech_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(({ let a0_ = { let a_ = c.m.r32((l_w_2).wrapping_add(1504)); (c.m.r32(a_) as i32) }; F::BtlScene_IsCharStopped(c, a0_) }) != 0)) as i32)) != 0) {
        { let _ = { let a0_ = c.m.r32(l_w_2); F::EftEmit_UpdateAlive(c, a0_, (l_w_2).wrapping_add(4)) }; };
        { let _ = F::EftShotTech_ReapShots(c, p_task_1); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotTech_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotTech_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotTech_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotTechMgr_Init(c: &mut Ctx, mut p_task_1: u32, mut p_src_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotTechMgr_Init", &PROBE_N, &[(p_task_1) as u32, (p_src_2) as u32], false); }
    let mut l_mgr_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_pack_5: u32 = 0;
    let r_ = 'f: {
    l_mgr_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let _ = ((816u32) as i32); F::EftShot_Nop(c) };
    { let _ = F::memset(c, l_mgr_3, 0i32, 816u32); };
    { let a3_ = c.m.r32((p_src_2).wrapping_add(28)); F::EftEmit_LoadSet(c, p_src_2, l_mgr_3, ((((0i32) as i64 as u64)) as u32), a3_, 0i32, 4i32) };
    { let _ = F::BtlTask_CreateChildList(c, p_task_1, ((((2i32) as i64 as u64)) as u32), ((((((2304u32) as i32)) as i64 as u64)) as u32)); };
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (3i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_pack_5 = { let a0_ = (c.m.r32((p_src_2).wrapping_add(32)) as i32); F::BtlScene_GetCharPackEntry(c, a0_, 8i32) };
                if ((((((c.m.r32((l_pack_5).wrapping_add(((((l_i_4).wrapping_add(2i32)) as u32)).wrapping_mul(4))) as i32)) != ((c.m.r32((l_pack_5).wrapping_add(((((l_i_4).wrapping_add(1i32)) as u32)).wrapping_mul(4))) as i32))) as i32)) != 0) {
                    { let v_ = F::BtlScene_GetPackEntry(c, l_pack_5, (l_i_4).wrapping_add(1i32)); let a_ = ((l_mgr_3).wrapping_add(800)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    if (((({ let a_ = (c.m.r32((p_src_2).wrapping_add(36))).wrapping_add(60); (c.m.r32(a_) as i32) }) & (131072i32))) != 0) {
        { let v_ = { let a0_ = c.m.r32((p_src_2).wrapping_add(28)); F::BtlScene_GetPackEntry(c, a0_, 1i32) }; let a_ = (l_mgr_3).wrapping_add(812); c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotTechMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotTechMgr_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotTechMgr_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32((p_task_1).wrapping_add(56)); F::EftEmit_FreeSet(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotTechMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotTechMgr_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotTechMgr_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32((p_task_1).wrapping_add(56)); F::EftEmit_BeginFrame(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotTechMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotTechMgr_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotTechMgr_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotTechMgr_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftTechEvt_GetEvents(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvt_GetEvents", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_evt_2: u32 = 0;
    let mut l_e_3: u32 = 0;
    let r_ = 'f: {
    l_evt_2 = c.m.r32(sym::gEftTechEvt);
    if (((((l_evt_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_e_3 = (c.m.r32((l_evt_2).wrapping_add(4))).wrapping_add((((p_objId_1) as u32)).wrapping_mul(16));
    break 'f (c.m.r32(l_e_3) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvt_GetEvents", &PROBE_N, &[], true); }
    r_
}

pub fn EftTechEvt_Start(c: &mut Ctx, mut p_src_1: u32, mut p_flagTime_2: i32, mut p_endTime_3: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvt_Start", &PROBE_N, &[(p_src_1) as u32, (p_flagTime_2) as u32, (p_endTime_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        if (((((c.m.r32(sym::gEftTechEvt)) == (((0i32) as u32))) as i32)) != 0) {
            break 'f ((0i32) as u32);
        }
        { let _ = F::memset(c, fp, 0i32, 16u32); };
        { let v_ = p_src_1; let a_ = fp; c.m.w32(a_, v_); };
        { let v_ = p_flagTime_2; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = (p_endTime_3).wrapping_sub(1i32); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = (p_endTime_3).wrapping_sub(1i32); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        break 'f { let a0_ = { let a_ = c.m.r32(sym::gEftTechEvt); c.m.r32(a_) }; F::BtlTaskList_AddTail(c, a0_, sym::gEftTechEvtTaskClass, fp) };
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvt_Start", &PROBE_N, &[], true); }
    r_
}

pub fn EftTechEvt_RequestStop(c: &mut Ctx, mut p_objId_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvt_RequestStop", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_e_2: u32 = 0;
    let r_ = 'f: {
    l_e_2 = ({ let a_ = (c.m.r32(sym::gEftTechEvt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((p_objId_1) as u32)).wrapping_mul(16));
    { let r_ = 2i32; let a_ = (l_e_2).wrapping_add(4); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    if (((((c.m.r32(l_e_2) as i32)) & (8i32))) != 0) {
        { let r_ = (!(8i32)); let a_ = l_e_2; let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvt_RequestStop", &PROBE_N, &[], true); }
    r_
}

pub fn EftTechEvt_RequestRestart(c: &mut Ctx, mut p_objId_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvt_RequestRestart", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_e_2: u32 = 0;
    let r_ = 'f: {
    l_e_2 = ({ let a_ = (c.m.r32(sym::gEftTechEvt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((p_objId_1) as u32)).wrapping_mul(16));
    if ((((!(((((c.m.r32(l_e_2) as i32)) & (8i32))) != 0)) as i32)) != 0) {
        { let r_ = 4i32; let a_ = (l_e_2).wrapping_add(4); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvt_RequestRestart", &PROBE_N, &[], true); }
    r_
}

pub fn EftTechEvt_RequestExpire(c: &mut Ctx, mut p_objId_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvt_RequestExpire", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_e_2: u32 = 0;
    let r_ = 'f: {
    l_e_2 = ({ let a_ = (c.m.r32(sym::gEftTechEvt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((p_objId_1) as u32)).wrapping_mul(16));
    if ((((!(((((c.m.r32(l_e_2) as i32)) & (8i32))) != 0)) as i32)) != 0) {
        { let r_ = 8i32; let a_ = (l_e_2).wrapping_add(4); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvt_RequestExpire", &PROBE_N, &[], true); }
    r_
}

pub fn EftTechEvt_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvt_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_size_2: i32 = 0;
    let r_ = 'f: {
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((12u32) as i32)) }; let a_ = sym::gEftTechEvt; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gEftTechEvt); F::memset(c, a0_, 0i32, 12u32) }; };
    { let v_ = F::BtlScene_GetCharCount(c); let a_ = (c.m.r32(sym::gEftTechEvt)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    l_size_2 = ((((({ let a_ = (c.m.r32(sym::gEftTechEvt)).wrapping_add(8); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16u32)) as i32);
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, l_size_2) }; let a_ = (c.m.r32(sym::gEftTechEvt)).wrapping_add(4); c.m.w32(a_, v_); };
    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gEftTechEvt)).wrapping_add(4); c.m.r32(a_) }; F::memset(c, a0_, 0i32, ((l_size_2) as u32)) }; };
    { let v_ = { let a1_ = { let a_ = (c.m.r32(sym::gEftTechEvt)).wrapping_add(8); (c.m.r32(a_) as i32) }; F::BtlTask_CreateChildList(c, p_task_1, ((((a1_) as i64 as u64)) as u32), ((((((28u32) as i32)) as i64 as u64)) as u32)) }; let a_ = c.m.r32(sym::gEftTechEvt); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvt_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftTechEvt_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvt_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = { let a_ = (c.m.r32(sym::gEftTechEvt)).wrapping_add(4); c.m.r32(a_) }; F::BtlPool_Free(c, a0_, a1_) };
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftTechEvt); F::BtlPool_Free(c, a0_, a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftTechEvt; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvt_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftTechEvt_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvt_Update", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_e_2: u32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < ({ let a_ = (c.m.r32(sym::gEftTechEvt)).wrapping_add(8); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l1; }
            'c2: {
                l_e_2 = ({ let a_ = (c.m.r32(sym::gEftTechEvt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((l_i_1) as u32)).wrapping_mul(16));
                { let v_ = 0i32; let a_ = l_e_2; c.m.w32(a_, (v_) as u32); };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvt_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftTechEvtTask_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvtTask_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let mut l_src_4: u32 = 0;
    let mut l_def_5: u32 = 0;
    let mut l_e_6: u32 = 0;
    let mut l_i_7: i32 = 0;
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    l_src_4 = c.m.r32(p_arg_2);
    l_def_5 = c.m.r32((l_src_4).wrapping_add(36));
    { let _ = F::memset(c, l_w_3, 0i32, 28u32); };
    { let s_ = p_arg_2; let d_ = l_w_3; rt::memcpy(c, d_, s_, 16); };
    { let v_ = 0i32; let a_ = (l_w_3).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
    if ((((((((c.m.r8((l_def_5).wrapping_add(4)) as i8)) as i32)) == (0i32)) as i32)) != 0) {
        {
            l_i_7 = 0i32;
            'l1: loop {
                if !(((((l_i_7) < (6i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if ((((((((c.m.r16(((l_def_5).wrapping_add(22)).wrapping_add((((l_i_7) as u32)).wrapping_mul(2))) as i16)) as i32)) != ((1i32).wrapping_neg())) as i32)) != 0) {
                        { let v_ = 1i32; let a_ = (l_w_3).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                    }
                }
                { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
            }
        }
    }
    l_e_6 = ({ let a_ = (c.m.r32(sym::gEftTechEvt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(((((c.m.r32(l_src_4) as i32)) as u32)).wrapping_mul(16));
    { let v_ = 0i32; let a_ = (l_e_6).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvtTask_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftTechEvtTask_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvtTask_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvtTask_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftTechEvtTask_Update(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvtTask_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_src_3: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_src_3 = c.m.r32(l_w_2);
    { let r_ = (!(16i32)); let a_ = (l_w_2).wrapping_add(16); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    if ((((!(({ let a0_ = (c.m.r32(l_src_3) as i32); F::BtlScene_IsCharStopped(c, a0_) }) != 0)) as i32)) != 0) {
        if ((F::EftTechEvt_IsRushTech(c, l_src_3)) != 0) {
            F::EftTechEvtTask_UpdateRush(c, p_task_1);
        } else {
            F::EftTechEvtTask_UpdateNormal(c, p_task_1);
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvtTask_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftTechEvtTask_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvtTask_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvtTask_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftTechEvtTask_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvtTask_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_src_3: u32 = 0;
    let mut l_e_4: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_src_3 = c.m.r32(l_w_2);
    l_e_4 = ({ let a_ = (c.m.r32(sym::gEftTechEvt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(((((c.m.r32(l_src_3) as i32)) as u32)).wrapping_mul(16));
    if (((((c.m.r32((l_w_2).wrapping_add(16)) as i32)) & (16i32))) != 0) {
        break 'f;
    }
    { let r_ = 16i32; let a_ = (l_w_2).wrapping_add(16); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    if ((F::EftTechEvt_IsRushTech(c, l_src_3)) != 0) {
        break 'f;
    }
    if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::BtlScene_IsCharStopped(c, a0_) }) != 0) {
        break 'f;
    }
    if (((((c.m.r32((l_e_4).wrapping_add(4)) as i32)) & (2i32))) != 0) {
        break 'f;
    }
    if (((((c.m.r32((l_e_4).wrapping_add(4)) as i32)) & (4i32))) != 0) {
        { let v_ = (c.m.r32((l_w_2).wrapping_add(12)) as i32); let a_ = (l_w_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let r_ = (!(4i32)); let a_ = (l_e_4).wrapping_add(4); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if (((((c.m.r32((l_e_4).wrapping_add(4)) as i32)) & (8i32))) != 0) {
        { let v_ = 0i32; let a_ = (l_w_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let r_ = (!(8i32)); let a_ = (l_e_4).wrapping_add(4); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if (((((c.m.r32((l_w_2).wrapping_add(16)) as i32)) & (2i32))) != 0) {
        if ((((({ let a_ = (l_w_2).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) <= (0i32)) as i32)) != 0) {
            { let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_SetHeldFlagA7(c, a0_) };
            { let r_ = (!(2i32)); let a_ = (l_w_2).wrapping_add(16); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        }
    }
    if (((((c.m.r32((l_w_2).wrapping_add(16)) as i32)) & (4i32))) != 0) {
        if ((((({ let a_ = (l_w_2).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) <= (0i32)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = (l_w_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let r_ = (!(4i32)); let a_ = (l_w_2).wrapping_add(16); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvtTask_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftTechEvtTask_UpdateNormal(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvtTask_UpdateNormal", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_src_3: u32 = 0;
    let mut l_e_4: u32 = 0;
    let mut l_ev_5: u64 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_src_3 = c.m.r32(l_w_2);
    l_e_4 = ({ let a_ = (c.m.r32(sym::gEftTechEvt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(((((c.m.r32(l_src_3) as i32)) as u32)).wrapping_mul(16));
    l_ev_5 = F::EftTechEvtTask_CollectEvents(c, l_w_2);
    { let a_ = (l_w_2).wrapping_add(20); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    if ((((l_ev_5) & (((512i32) as u64)))) != 0) {
        { let r_ = 2i32; let a_ = l_e_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let r_ = 2i32; let a_ = (l_w_2).wrapping_add(16); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if ((((l_ev_5) & (((1024i32) as u64)))) != 0) {
        { let r_ = 4i32; let a_ = l_e_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        if ((((((({ let a_ = (c.m.r32((l_src_3).wrapping_add(36))).wrapping_add(4); (c.m.r8(a_) as i8) }) as i32)) != (0i32)) as i32)) != 0) {
            { let r_ = 4i32; let a_ = (l_w_2).wrapping_add(16); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        } else {
            { let r_ = 32i32; let a_ = (l_w_2).wrapping_add(16); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
    }
    if ((((l_ev_5) & (((2048i32) as u64)))) != 0) {
        { let r_ = 8i32; let a_ = l_e_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        F::BtlTask_SetDead(c, p_task_1);
        break 'f;
    }
    if ((((((({ let a_ = (c.m.r32((l_src_3).wrapping_add(36))).wrapping_add(4); (c.m.r8(a_) as i8) }) as i32)) == (0i32)) as i32)) != 0) {
        if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::EftTechEvt_IsInterrupted(c, a0_) }) != 0) {
            { let r_ = 1024i32; let a_ = l_e_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            F::BtlTask_SetDead(c, p_task_1);
            break 'f;
        }
        { let _ = { let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_IsSkillApplied(c, a0_) }; };
        if ((((!(({ let a0_ = (c.m.r32(l_src_3) as i32); F::EftTechEvt_IsInSkill(c, a0_) }) != 0)) as i32)) != 0) {
            if (((((c.m.r32((l_w_2).wrapping_add(16)) as i32)) & (32i32))) != 0) {
                { let r_ = 8i32; let a_ = l_e_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            } else {
                { let r_ = 1024i32; let a_ = l_e_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
            F::BtlTask_SetDead(c, p_task_1);
            break 'f;
        }
    } else {
        if ((((!(({ let a0_ = (c.m.r32(l_src_3) as i32); F::EftTechEvt_IsInTechnique(c, a0_) }) != 0)) as i32)) != 0) {
            { let r_ = 1024i32; let a_ = l_e_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            F::BtlTask_SetDead(c, p_task_1);
            break 'f;
        }
        if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::EftTechEvt_IsInterrupted(c, a0_) }) != 0) {
            { let r_ = 1024i32; let a_ = l_e_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            F::BtlTask_SetDead(c, p_task_1);
            break 'f;
        }
    }
    if ((((l_ev_5) & (((4096i32) as u64)))) != 0) {
        { let r_ = 16i32; let a_ = l_e_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if ((((l_ev_5) & (((8192i32) as u64)))) != 0) {
        { let r_ = 32i32; let a_ = l_e_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if ((((l_ev_5) & (((16384i32) as u64)))) != 0) {
        { let r_ = 64i32; let a_ = l_e_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if ((((l_ev_5) & (((2097152i32) as u64)))) != 0) {
        { let r_ = 128i32; let a_ = l_e_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if ((((l_ev_5) & (((4194304i32) as u64)))) != 0) {
        { let r_ = 256i32; let a_ = l_e_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if ((((l_ev_5) & (((8388608i32) as u64)))) != 0) {
        { let r_ = 512i32; let a_ = l_e_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if ((((((c.m.r32((l_w_2).wrapping_add(8)) as i32)) <= (0i32)) as i32)) != 0) {
        { let r_ = 8i32; let a_ = l_e_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvtTask_UpdateNormal", &PROBE_N, &[], true); }
    r_
}

pub fn EftTechEvtTask_UpdateRush(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvtTask_UpdateRush", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_src_3: u32 = 0;
    let mut l_e_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_src_3 = c.m.r32(l_w_2);
    if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_IsRushConnected(c, a0_) }) != 0) {
        { let r_ = 8i32; let a_ = (l_w_2).wrapping_add(16); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if (((((c.m.r32((l_w_2).wrapping_add(16)) as i32)) & (8i32))) != 0) {
        { let a_ = (l_w_2).wrapping_add(20); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    }
    if ((((!(({ let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_IsInTechnique(c, a0_) }) != 0)) as i32)) != 0) {
        l_e_4 = ({ let a_ = (c.m.r32(sym::gEftTechEvt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(((((c.m.r32(l_src_3) as i32)) as u32)).wrapping_mul(16));
        { let r_ = 1024i32; let a_ = l_e_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        F::BtlTask_SetDead(c, p_task_1);
        break 'f;
    }
    if (((((((((c.m.r32((l_w_2).wrapping_add(16)) as i32)) & (8i32))) != 0) && ((((!(({ let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_IsInRushSequence(c, a0_) }) != 0)) as i32)) != 0)) as i32)) != 0) {
        l_e_4 = ({ let a_ = (c.m.r32(sym::gEftTechEvt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(((((c.m.r32(l_src_3) as i32)) as u32)).wrapping_mul(16));
        { let r_ = 1024i32; let a_ = l_e_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        F::BtlTask_SetDead(c, p_task_1);
        break 'f;
    }
    if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_ObjTestAttr(c, a0_, ((2048i32) as u64)) }) != 0) {
        l_e_4 = ({ let a_ = (c.m.r32(sym::gEftTechEvt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(((((c.m.r32(l_src_3) as i32)) as u32)).wrapping_mul(16));
        { let r_ = 8i32; let a_ = l_e_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        F::BtlTask_SetDead(c, p_task_1);
        break 'f;
    }
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((l_i_5) < ({ let a_ = (c.m.r32(sym::gEftTechEvt)).wrapping_add(8); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l1; }
            'c2: {
                l_e_4 = ({ let a_ = (c.m.r32(sym::gEftTechEvt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((l_i_5) as u32)).wrapping_mul(16));
                if ((F::BtlCharApi_ObjTestAttr(c, l_i_5, ((512i32) as u64))) != 0) {
                    { let r_ = 2i32; let a_ = l_e_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let r_ = 2i32; let a_ = (l_w_2).wrapping_add(16); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
                if ((F::BtlCharApi_ObjTestAttr(c, l_i_5, ((1024i32) as u64))) != 0) {
                    { let r_ = 4i32; let a_ = l_e_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let r_ = 4i32; let a_ = (l_w_2).wrapping_add(16); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
                if ((F::BtlCharApi_ObjTestAttr(c, l_i_5, ((4096i32) as u64))) != 0) {
                    { let r_ = 16i32; let a_ = l_e_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
                if ((F::BtlCharApi_ObjTestAttr(c, l_i_5, ((8192i32) as u64))) != 0) {
                    { let r_ = 32i32; let a_ = l_e_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
                if ((F::BtlCharApi_ObjTestAttr(c, l_i_5, ((16384i32) as u64))) != 0) {
                    { let r_ = 64i32; let a_ = l_e_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
                if ((F::BtlCharApi_ObjTestAttr(c, l_i_5, ((2097152i32) as u64))) != 0) {
                    { let r_ = 128i32; let a_ = l_e_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
                if ((F::BtlCharApi_ObjTestAttr(c, l_i_5, ((4194304i32) as u64))) != 0) {
                    { let r_ = 256i32; let a_ = l_e_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
                if ((F::BtlCharApi_ObjTestAttr(c, l_i_5, ((8388608i32) as u64))) != 0) {
                    { let r_ = 512i32; let a_ = l_e_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvtTask_UpdateRush", &PROBE_N, &[], true); }
    r_
}

pub fn EftTechEvt_IsRushTech(c: &mut Ctx, mut p_src_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvt_IsRushTech", &PROBE_N, &[(p_src_1) as u32], false); }
    let mut l_def_2: u32 = 0;
    let r_ = 'f: {
    l_def_2 = c.m.r32((p_src_1).wrapping_add(36));
    if (((((l_def_2) != (((0i32) as u32))) as i32)) != 0) {
        if ((((((((c.m.r8((l_def_2).wrapping_add(4)) as i8)) as i32)) != (0i32)) as i32)) != 0) {
            if ((((((((c.m.r8((l_def_2).wrapping_add(5)) as i8)) as i32)) == (5i32)) as i32)) != 0) {
                break 'f 1i32;
            }
            if (({ let a0_ = (c.m.r32((p_src_1).wrapping_add(32)) as i32); F::BtlCharApi_IsInRushSequence(c, a0_) }) != 0) {
                break 'f 1i32;
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvt_IsRushTech", &PROBE_N, &[], true); }
    r_
}

pub fn EftTechEvtTask_CollectEvents(c: &mut Ctx, mut p_w_1: u32) -> u64 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvtTask_CollectEvents", &PROBE_N, &[(p_w_1) as u32], false); }
    let mut l_ev_2: u64 = 0;
    let mut l_src_3: u32 = 0;
    let mut l_def_4: u32 = 0;
    let mut l_frame_5: i32 = 0;
    let r_ = 'f: {
    l_ev_2 = ((0i32) as u64);
    l_src_3 = c.m.r32(p_w_1);
    l_def_4 = c.m.r32((l_src_3).wrapping_add(36));
    if ((((((c.m.r32((p_w_1).wrapping_add(24)) as i32)) == (0i32)) as i32)) != 0) {
        if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_ObjTestAttr(c, a0_, ((512i32) as u64)) }) != 0) {
            { let r_ = ((512i32) as u64); let n_ = ((l_ev_2) | (r_)); l_ev_2 = n_; };
        }
        if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_ObjTestAttr(c, a0_, ((1024i32) as u64)) }) != 0) {
            { let r_ = ((1024i32) as u64); let n_ = ((l_ev_2) | (r_)); l_ev_2 = n_; };
        }
        if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_ObjTestAttr(c, a0_, ((2048i32) as u64)) }) != 0) {
            { let r_ = ((2048i32) as u64); let n_ = ((l_ev_2) | (r_)); l_ev_2 = n_; };
        }
        if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_ObjTestAttr(c, a0_, ((4096i32) as u64)) }) != 0) {
            { let r_ = ((4096i32) as u64); let n_ = ((l_ev_2) | (r_)); l_ev_2 = n_; };
        }
        if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_ObjTestAttr(c, a0_, ((8192i32) as u64)) }) != 0) {
            { let r_ = ((8192i32) as u64); let n_ = ((l_ev_2) | (r_)); l_ev_2 = n_; };
        }
        if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_ObjTestAttr(c, a0_, ((16384i32) as u64)) }) != 0) {
            { let r_ = ((16384i32) as u64); let n_ = ((l_ev_2) | (r_)); l_ev_2 = n_; };
        }
    } else {
        l_frame_5 = (c.m.r32((p_w_1).wrapping_add(20)) as i32);
        if (((((l_frame_5) == ((((c.m.r16(((l_def_4).wrapping_add(22)).wrapping_add((((0i32) as u32)).wrapping_mul(2))) as i16)) as i32))) as i32)) != 0) {
            { let r_ = ((512i32) as u64); let n_ = ((l_ev_2) | (r_)); l_ev_2 = n_; };
        }
        if (((((l_frame_5) == ((((c.m.r16(((l_def_4).wrapping_add(22)).wrapping_add((((1i32) as u32)).wrapping_mul(2))) as i16)) as i32))) as i32)) != 0) {
            { let r_ = ((1024i32) as u64); let n_ = ((l_ev_2) | (r_)); l_ev_2 = n_; };
        }
        if (((((l_frame_5) == ((((c.m.r16(((l_def_4).wrapping_add(22)).wrapping_add((((2i32) as u32)).wrapping_mul(2))) as i16)) as i32))) as i32)) != 0) {
            { let r_ = ((2048i32) as u64); let n_ = ((l_ev_2) | (r_)); l_ev_2 = n_; };
        }
        if (((((l_frame_5) == ((((c.m.r16(((l_def_4).wrapping_add(22)).wrapping_add((((3i32) as u32)).wrapping_mul(2))) as i16)) as i32))) as i32)) != 0) {
            { let r_ = ((4096i32) as u64); let n_ = ((l_ev_2) | (r_)); l_ev_2 = n_; };
        }
        if (((((l_frame_5) == ((((c.m.r16(((l_def_4).wrapping_add(22)).wrapping_add((((4i32) as u32)).wrapping_mul(2))) as i16)) as i32))) as i32)) != 0) {
            { let r_ = ((8192i32) as u64); let n_ = ((l_ev_2) | (r_)); l_ev_2 = n_; };
        }
        if (((((l_frame_5) == ((((c.m.r16(((l_def_4).wrapping_add(22)).wrapping_add((((5i32) as u32)).wrapping_mul(2))) as i16)) as i32))) as i32)) != 0) {
            { let r_ = ((16384i32) as u64); let n_ = ((l_ev_2) | (r_)); l_ev_2 = n_; };
        }
    }
    if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_ObjTestAttr(c, a0_, ((2097152i32) as u64)) }) != 0) {
        { let r_ = ((2097152i32) as u64); let n_ = ((l_ev_2) | (r_)); l_ev_2 = n_; };
    }
    if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_ObjTestAttr(c, a0_, ((4194304i32) as u64)) }) != 0) {
        { let r_ = ((4194304i32) as u64); let n_ = ((l_ev_2) | (r_)); l_ev_2 = n_; };
    }
    if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::BtlCharApi_ObjTestAttr(c, a0_, ((8388608i32) as u64)) }) != 0) {
        { let r_ = ((8388608i32) as u64); let n_ = ((l_ev_2) | (r_)); l_ev_2 = n_; };
    }
    break 'f l_ev_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvtTask_CollectEvents", &PROBE_N, &[], true); }
    r_
}

pub fn EftTechEvt_IsInterrupted(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvt_IsInterrupted", &PROBE_N, &[(p_objId_1) as u32], false); }
    let r_ = 'f: {
    break 'f F::BtlCharApi_TestAnimFlag10(c, p_objId_1);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvt_IsInterrupted", &PROBE_N, &[], true); }
    r_
}

pub fn EftTechEvt_IsInSkill(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvt_IsInSkill", &PROBE_N, &[(p_objId_1) as u32], false); }
    let r_ = 'f: {
    break 'f F::BtlCharApi_IsInSkill(c, p_objId_1);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvt_IsInSkill", &PROBE_N, &[], true); }
    r_
}

pub fn EftTechEvt_IsInTechnique(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvt_IsInTechnique", &PROBE_N, &[(p_objId_1) as u32], false); }
    let r_ = 'f: {
    break 'f F::BtlCharApi_IsInTechnique(c, p_objId_1);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTechEvt_IsInTechnique", &PROBE_N, &[], true); }
    r_
}

pub fn EftObjTech_AddHitRecord(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftObjTech_AddHitRecord", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_rec_3: u32 = 0;
    let mut l_size_4: F32 = F32(0);
    let mut l_shape_5: u32 = 0;
    let mut l_a_6: u32 = 0;
    let mut l_b_7: u32 = 0;
    let mut l_a_8: u32 = 0;
    let mut l_b_9: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_rec_3 = F::EftHit_GetNew(c);
    l_size_4 = rt::fmul(F32(c.m.r32((l_w_2).wrapping_add(1480))), F::EftEmit_GetTrailWidth(c, (l_w_2).wrapping_add(148)));
    l_shape_5 = (l_w_2).wrapping_add(16);
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(1460)) as i32)) & (2048i32))) != 0)) as i32)) != 0) {
        if ((((((c.m.r32((l_w_2).wrapping_add(1464)) as i32)) == (0i32)) as i32)) != 0) {
            { let r_ = 2048i32; let a_ = (l_rec_3).wrapping_add(80); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
    } else {
        { let r_ = 16i32; let a_ = (l_rec_3).wrapping_add(80); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        l_shape_5 = (l_w_2).wrapping_add(80);
    }
    { let s_ = l_shape_5; let d_ = (l_rec_3).wrapping_add(16); rt::memcpy(c, d_, s_, 64); };
    { let v_ = p_task_1; let a_ = (l_rec_3).wrapping_add(96); c.m.w32(a_, v_); };
    { let v_ = c.m.r32((l_w_2).wrapping_add(144)); let a_ = (l_rec_3).wrapping_add(100); c.m.w32(a_, v_); };
    's1: {
        let sv10: i32 = (({ let a_ = ({ let a_ = (c.m.r32((l_w_2).wrapping_add(144))).wrapping_add(36); c.m.r32(a_) }).wrapping_add(8); (c.m.r8(a_) as i8) }) as i32);
        match sv10 {
            1i32 => {
                {
                    l_a_6 = F::EftHitArena_AllocBox(c);
                    l_b_7 = F::EftHitArena_AllocBox(c);
                    F::ColCapsule_Set(c, l_a_6, (l_w_2).wrapping_add(960), (l_shape_5).wrapping_add(16), l_size_4);
                    F::ColCapsule_Set(c, l_b_7, (l_w_2).wrapping_add(960), (l_shape_5).wrapping_add(32), l_size_4);
                    F::EftHit_SetShapeBoxes(c, l_rec_3, l_a_6, l_b_7);
                    break 's1;
                }
            }
            0i32 => {
                {
                    l_a_8 = F::EftHitArena_AllocSphere(c);
                    l_b_9 = F::EftHitArena_AllocSphere(c);
                    F::ColSphere_Set(c, l_a_8, (l_shape_5).wrapping_add(16), l_size_4);
                    F::ColSphere_Set(c, l_b_9, (l_shape_5).wrapping_add(32), l_size_4);
                    F::EftHit_SetShapeSpheres(c, l_rec_3, l_a_8, l_b_9);
                    break 's1;
                }
            }
            _ => {
                break 'f;
            }
        }
    }
    F::EftHit_Add(c, l_rec_3);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftObjTech_AddHitRecord", &PROBE_N, &[], true); }
    r_
}

pub fn EftObjTech_CreateModel(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftObjTech_CreateModel", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_owner_2: u32 = 0;
    let mut l_w_3: u32 = 0;
    let mut l_model_4: u32 = 0;
    let r_ = 'f: {
    l_owner_2 = F::BtlTask_GetParent(c, p_task_1);
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    l_model_4 = (l_w_3).wrapping_add(1500);
    { let v_ = { let a_ = (c.m.r32((l_owner_2).wrapping_add(56))).wrapping_add(800); c.m.r32(a_) }; let a_ = l_model_4; c.m.w32(a_, v_); };
    { let v_ = (({ let a1_ = c.m.r32(l_model_4); F::EftObj_Create(c, (l_model_4).wrapping_add(4), a1_) }) as i8); let a_ = (l_model_4).wrapping_add(53); c.m.w8(a_, (v_) as u8); };
    { let v_ = (((1i32).wrapping_neg()) as i8); let a_ = (l_model_4).wrapping_add(54); c.m.w8(a_, (v_) as u8); };
    { let a0_ = (((c.m.r8((l_model_4).wrapping_add(53)) as i8)) as i32); F::EftObj_SetVisible(c, a0_, 0i32) };
    { let a0_ = (((c.m.r8((l_model_4).wrapping_add(53)) as i8)) as i32); let _ = a0_; let _ = 0i32; F::EftObj_Nop(c) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftObjTech_CreateModel", &PROBE_N, &[], true); }
    r_
}

pub fn EftObjTech_SetModelPose(c: &mut Ctx, mut p_task_1: u32, mut p_pos_2: u32, mut p_angles_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftObjTech_SetModelPose", &PROBE_N, &[(p_task_1) as u32, (p_pos_2) as u32, (p_angles_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_w_4: u32 = 0;
    let mut l_model_5: u32 = 0;
    let r_ = 'f: {
        l_w_4 = c.m.r32((p_task_1).wrapping_add(56));
        F::Mtx_StoreIdentity(c, fp);
        { let a2_ = F32(c.m.r32((l_w_4).wrapping_add(1476))); F::Mtx_ScaleDiagUniform(c, fp, fp, a2_) };
        l_model_5 = (l_w_4).wrapping_add(1500);
        F::Mtx_RotateZXY(c, fp, fp, p_angles_3);
        F::Mtx_Translate(c, fp, fp, p_pos_2);
        { let a0_ = (((c.m.r8((l_model_5).wrapping_add(53)) as i8)) as i32); F::EftObj_SetMtx(c, a0_, fp) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftObjTech_SetModelPose", &PROBE_N, &[], true); }
    r_
}

pub fn EftObjTech_DestroyModel(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftObjTech_DestroyModel", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = (({ let a_ = ((c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(1500)).wrapping_add(53); (c.m.r8(a_) as i8) }) as i32); { let _ = F::EftObj_Destroy(c, a0_); } };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftObjTech_DestroyModel", &PROBE_N, &[], true); }
    r_
}

pub fn EftObjTech_InitRings(c: &mut Ctx, mut p_task_1: u32, mut p_roll_2: F32, mut p_step_3: F32, mut p_grow_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftObjTech_InitRings", &PROBE_N, &[(p_task_1) as u32, (p_roll_2).0, (p_step_3).0, (p_grow_4).0], false); }
    let mut l_i_5: i32 = 0;
    let mut l_owner_6: u32 = 0;
    let mut l_w_7: u32 = 0;
    let mut l_mgr_8: u32 = 0;
    let mut l_rings_9: u32 = 0;
    let mut l_src_10: u32 = 0;
    let mut l_offset_11: F32 = F32(0);
    let mut l_scale_12: F32 = F32(0);
    let r_ = 'f: {
    l_owner_6 = F::BtlTask_GetParent(c, p_task_1);
    l_w_7 = c.m.r32((p_task_1).wrapping_add(56));
    l_mgr_8 = c.m.r32((l_owner_6).wrapping_add(56));
    l_rings_9 = (l_w_7).wrapping_add(1568);
    l_src_10 = c.m.r32((l_w_7).wrapping_add(144));
    l_offset_11 = F32(0x00000000);
    l_scale_12 = F32(0x3f800000);
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((l_i_5) < ((c.m.r32((l_rings_9).wrapping_add(468)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                F::EftMesh_Copy(c, (l_rings_9).wrapping_add((((l_i_5) as u32)).wrapping_mul(144)), (l_mgr_8).wrapping_add(816));
                F::EftMesh_SetTex(c, (l_rings_9).wrapping_add((((l_i_5) as u32)).wrapping_mul(144)), (l_mgr_8).wrapping_add(960));
                F::EftMesh_SetLayer(c, (l_rings_9).wrapping_add((((l_i_5) as u32)).wrapping_mul(144)), 1i32);
                F::EftMesh_SetTexBase(c, (l_rings_9).wrapping_add((((l_i_5) as u32)).wrapping_mul(144)), 0i32);
                { let a1_ = (c.m.r32(l_src_10) as i32); F::EftMesh_SetOwner(c, (l_rings_9).wrapping_add((((l_i_5) as u32)).wrapping_mul(144)), a1_, 1i32) };
                { let v_ = p_roll_2; let a_ = ((l_rings_9).wrapping_add(432)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let v_ = l_offset_11; let a_ = ((l_rings_9).wrapping_add(444)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let r_ = p_step_3; let n_ = rt::fadd(l_offset_11, r_); l_offset_11 = n_; };
                { let v_ = l_scale_12; let a_ = ((l_rings_9).wrapping_add(456)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let r_ = p_grow_4; let n_ = rt::fadd(l_scale_12, r_); l_scale_12 = n_; };
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftObjTech_InitRings", &PROBE_N, &[], true); }
    r_
}

pub fn EftObjTech_SetRingParams(c: &mut Ctx, mut p_task_1: u32, mut p_roll_2: F32, mut p_step_3: F32, mut p_grow_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftObjTech_SetRingParams", &PROBE_N, &[(p_task_1) as u32, (p_roll_2).0, (p_step_3).0, (p_grow_4).0], false); }
    let mut l_i_5: i32 = 0;
    let mut l_w_6: u32 = 0;
    let mut l_rings_7: u32 = 0;
    let mut l_offset_8: F32 = F32(0);
    let mut l_scale_9: F32 = F32(0);
    let r_ = 'f: {
    l_w_6 = c.m.r32((p_task_1).wrapping_add(56));
    l_rings_7 = (l_w_6).wrapping_add(1568);
    l_offset_8 = F32(0x00000000);
    l_scale_9 = F32(0x3f800000);
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((l_i_5) < ((c.m.r32((l_rings_7).wrapping_add(468)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = p_roll_2; let a_ = ((l_rings_7).wrapping_add(432)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let v_ = l_offset_8; let a_ = ((l_rings_7).wrapping_add(444)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let r_ = p_step_3; let n_ = rt::fadd(l_offset_8, r_); l_offset_8 = n_; };
                { let v_ = l_scale_9; let a_ = ((l_rings_7).wrapping_add(456)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let r_ = p_grow_4; let n_ = rt::fadd(l_scale_9, r_); l_scale_9 = n_; };
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftObjTech_SetRingParams", &PROBE_N, &[], true); }
    r_
}

pub fn EftObjTech_PlaceRings(c: &mut Ctx, mut p_task_1: u32, mut p_pos_2: u32, mut p_angles_3: u32, mut p_width_4: F32, mut p_length_5: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftObjTech_PlaceRings", &PROBE_N, &[(p_task_1) as u32, (p_pos_2) as u32, (p_angles_3) as u32, (p_width_4).0, (p_length_5).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_i_6: i32 = 0;
    let mut l_w_7: u32 = 0;
    let mut l_rings_8: u32 = 0;
    let r_ = 'f: {
        l_w_7 = c.m.r32((p_task_1).wrapping_add(56));
        l_rings_8 = (l_w_7).wrapping_add(1568);
        {
            l_i_6 = 0i32;
            'l1: loop {
                if !(((((l_i_6) < ((c.m.r32((l_rings_8).wrapping_add(468)) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::Mtx_StoreIdentity(c, fp);
                    { let r_ = rt::fmul(p_width_4, F32(c.m.r32(((l_rings_8).wrapping_add(456)).wrapping_add((((l_i_6) as u32)).wrapping_mul(4))))); let a_ = ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let r_ = p_length_5; let a_ = ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let r_ = rt::fmul(p_width_4, F32(c.m.r32(((l_rings_8).wrapping_add(456)).wrapping_add((((l_i_6) as u32)).wrapping_mul(4))))); let a_ = ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let v_ = F32(c.m.r32(((l_rings_8).wrapping_add(432)).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)))); let a_ = p_angles_3; c.m.w32(a_, (v_).0); };
                    { let v_ = { let a0_ = F32(c.m.r32(p_angles_3)); F::EftMath_WrapAngle(c, a0_) }; let a_ = p_angles_3; c.m.w32(a_, (v_).0); };
                    F::Mtx_RotateZXY(c, fp, fp, p_angles_3);
                    { let a2_ = F32(c.m.r32(((l_rings_8).wrapping_add(444)).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)))); F::Vec3_Scale(c, fp.wrapping_add(64), l_w_7, a2_) };
                    F::Vec3_Add(c, fp.wrapping_add(64), fp.wrapping_add(64), p_pos_2);
                    { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(64)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    F::Mtx_Translate(c, fp, fp, fp.wrapping_add(64));
                    F::EftMesh_SetMtx(c, (l_rings_8).wrapping_add((((l_i_6) as u32)).wrapping_mul(144)), fp);
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftObjTech_PlaceRings", &PROBE_N, &[], true); }
    r_
}

pub fn EftObjTech_DrawRings(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftObjTech_DrawRings", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let mut l_w_3: u32 = 0;
    let mut l_rings_4: u32 = 0;
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    l_rings_4 = (l_w_3).wrapping_add(1568);
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < ((c.m.r32((l_rings_4).wrapping_add(468)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                F::EftMesh_Draw(c, (l_rings_4).wrapping_add((((l_i_2) as u32)).wrapping_mul(144)));
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftObjTech_DrawRings", &PROBE_N, &[], true); }
    r_
}

pub fn EftObjTech_TermRings(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftObjTech_TermRings", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftObjTech_TermRings", &PROBE_N, &[], true); }
    r_
}

pub fn EftObjTech_UpdateEmitters(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32, mut p_set_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftObjTech_UpdateEmitters", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32, (p_set_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_4: u32 = 0;
    let mut l_endOnTarget_5: i32 = 0;
    let mut l_onTarget_6: i32 = 0;
    let mut l_end_7: i32 = 0;
    let mut l_fast_8: i32 = 0;
    let mut l_g_9: i32 = 0;
    let mut l_i_10: i32 = 0;
    let mut l_info_11: u32 = 0;
    let mut l_idx_12: i32 = 0;
    let mut l_res_13: i32 = 0;
    let r_ = 'f: {
        l_w_4 = c.m.r32((p_task_2).wrapping_add(56));
        l_endOnTarget_5 = 0i32;
        l_onTarget_6 = 0i32;
        F::Vec4_Copy(c, fp, ((l_w_4).wrapping_add(16)).wrapping_add(16));
        l_fast_8 = (((c.m.r32((l_w_4).wrapping_add(1460)) as i32)) & (8i32));
        if ((((!(((((c.m.r32((l_w_4).wrapping_add(1460)) as i32)) & (1024i32))) != 0)) as i32)) != 0) {
            l_end_7 = (((c.m.r32((l_w_4).wrapping_add(1460)) as i32)) & (1i32));
        } else {
            if ((((!(((((c.m.r32((l_w_4).wrapping_add(1460)) as i32)) & (2048i32))) != 0)) as i32)) != 0) {
                l_end_7 = (((c.m.r32((l_w_4).wrapping_add(1460)) as i32)) & (1i32));
                { let r_ = 2048i32; let a_ = (l_w_4).wrapping_add(1460); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            } else {
                F::Vec4_Copy(c, fp, ((l_w_4).wrapping_add(80)).wrapping_add(16));
                l_end_7 = (((c.m.r32((l_w_4).wrapping_add(1460)) as i32)) & (2i32));
                l_onTarget_6 = 1i32;
                l_endOnTarget_5 = l_end_7;
            }
        }
        {
            l_g_9 = 0i32;
            'l1: loop {
                if !(((((l_g_9) < (19i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((({ let a_ = c.m.r32(p_set_3); c.m.r32(a_) }) & ((((1i32).wrapping_shl((l_g_9) as u32)) as u32)))) != 0) {
                        l_info_11 = c.m.r32(((p_set_3).wrapping_add(12)).wrapping_add((((l_g_9) as u32)).wrapping_mul(16)));
                        {
                            l_i_10 = 0i32;
                            'l3: loop {
                                if !(((((l_i_10) < (((c.m.r8((l_info_11).wrapping_add(((1i32) as u32)))) as i32))) as i32)) != 0) { break 'l3; }
                                'c4: {
                                    l_idx_12 = (((c.m.r8((((p_set_3).wrapping_add(12)).wrapping_add((((l_g_9) as u32)).wrapping_mul(16))).wrapping_add(8))) as i32)).wrapping_add(l_i_10);
                                    if ((l_onTarget_6) != 0) {
                                        l_end_7 = l_endOnTarget_5;
                                        if ((((((((((({ let a_ = ((c.m.r32((p_set_3).wrapping_add(8))).wrapping_add((((l_idx_12) as u32)).wrapping_mul(64))).wrapping_add(9); c.m.r8(a_) }) as i32)) == (0i32)) as i32)) != 0) && (((rt::fgt({ let a_ = ((c.m.r32((p_set_3).wrapping_add(8))).wrapping_add((((l_idx_12) as u32)).wrapping_mul(64))).wrapping_add(24); F32(c.m.r32(a_)) }, F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                                            l_end_7 = 0i32;
                                            l_fast_8 = 0i32;
                                        }
                                    }
                                    l_res_13 = F::EftEmit_GetFlagsFromReq(c, p_set_3, (l_w_4).wrapping_add(148), p_objId_1, l_g_9, l_i_10, l_end_7, l_fast_8);
                                    if (((((l_res_13) != (0i32)) as i32)) != 0) {
                                        { let a8_ = F32(c.m.r32((l_w_4).wrapping_add(1476))); F::EftEmit_SpawnOwn(c, p_set_3, (l_w_4).wrapping_add(148), (l_w_4).wrapping_add(864), fp, l_w_4, l_g_9, l_i_10, l_res_13, a8_) };
                                    }
                                }
                                { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; };
                            }
                        }
                    }
                }
                { let o_ = l_g_9; let n_ = (o_).wrapping_add(1); l_g_9 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftObjTech_UpdateEmitters", &PROBE_N, &[], true); }
    r_
}

pub fn EftObjTech_UpdateMotion(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftObjTech_UpdateMotion", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(240) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 240);
    let mut l_owner_2: u32 = 0;
    let mut l_w_3: u32 = 0;
    let mut l_mgr_4: u32 = 0;
    let mut l_src_5: u32 = 0;
    let mut l_pos_6: u32 = 0;
    let mut l_taskFlags_7: i32 = 0;
    let mut l_start_8: u32 = 0;
    let mut l_id_9: i32 = 0;
    let mut l_step_10: F32 = F32(0);
    let mut l_width_11: F32 = F32(0);
    let mut l_length_12: F32 = F32(0);
    let mut l_opp_13: i32 = 0;
    let r_ = 'f: {
        l_owner_2 = F::BtlTask_GetParent(c, p_task_1);
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        l_mgr_4 = c.m.r32((l_owner_2).wrapping_add(56));
        l_src_5 = c.m.r32((l_w_3).wrapping_add(144));
        's1: {
            let sv14: i32 = (c.m.r32((l_w_3).wrapping_add(1464)) as i32);
            match sv14 {
                0i32 => {
                    {
                        if (({ let a0_ = (c.m.r32(l_src_5) as i32); F::EftShot_TestBits(c, a0_, 16i32) }) != 0) {
                            l_start_8 = (l_w_3).wrapping_add(80);
                            { let a0_ = (c.m.r32(l_src_5) as i32); let a1_ = (c.m.r32((l_w_3).wrapping_add(1072)) as i32); F::BtlCharApi_GetNodePos(c, a0_, a1_, l_start_8) };
                            l_pos_6 = ((l_w_3).wrapping_add(80)).wrapping_add(16);
                            F::Vec4_Copy(c, l_pos_6, l_start_8);
                            { let a2_ = (c.m.r32(l_src_5) as i32); { let _ = F::EftAim_GetDir(c, l_w_3, ((((l_pos_6) as u64)) as i32), a2_); } };
                            { let a2_ = F32(c.m.r32((l_w_3).wrapping_add(1468))); F::Vec3_Scale(c, ((l_w_3).wrapping_add(80)).wrapping_add(48), l_w_3, a2_) };
                            { let r_ = 4096i32; let a_ = (l_w_3).wrapping_add(1460); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        } else {
                            l_pos_6 = ((l_w_3).wrapping_add(80)).wrapping_add(16);
                        }
                        if (({ let a0_ = (c.m.r32(l_src_5) as i32); F::EftShot_TestBits(c, a0_, 32i32) }) != 0) {
                            { let v_ = { let a_ = (c.m.r32((l_src_5).wrapping_add(36))).wrapping_add(52); F32(c.m.r32(a_)) }; let a_ = (l_w_3).wrapping_add(1468); c.m.w32(a_, (v_).0); };
                            { let a2_ = (c.m.r32(l_src_5) as i32); { let _ = F::EftAim_GetDir(c, l_w_3, ((((l_pos_6) as u64)) as i32), a2_); } };
                            { let a2_ = F32(c.m.r32((l_w_3).wrapping_add(1468))); F::Vec3_Scale(c, ((l_w_3).wrapping_add(80)).wrapping_add(48), l_w_3, a2_) };
                            { let r_ = 32i32; let a_ = (l_w_3).wrapping_add(1460); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            l_taskFlags_7 = ((c.m.r32((p_task_1).wrapping_add(4))) as i32);
                            { let r_ = (!(1i32)); let n_ = ((l_taskFlags_7) & (r_)); l_taskFlags_7 = n_; };
                            { let r_ = (!(4i32)); let n_ = ((l_taskFlags_7) & (r_)); l_taskFlags_7 = n_; };
                            { let r_ = (!(512i32)); let n_ = ((l_taskFlags_7) & (r_)); l_taskFlags_7 = n_; };
                            { let v_ = ((l_taskFlags_7) as u32); let a_ = (p_task_1).wrapping_add(4); c.m.w32(a_, v_); };
                            { let r_ = 8192i32; let a_ = (l_w_3).wrapping_add(1460); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                        F::Vec4_Copy(c, ((l_w_3).wrapping_add(80)).wrapping_add(32), l_pos_6);
                        if (((((c.m.r32((l_w_3).wrapping_add(1460)) as i32)) & (8192i32))) != 0) {
                            F::Vec3_Add(c, l_pos_6, l_pos_6, ((l_w_3).wrapping_add(80)).wrapping_add(48));
                        } else {
                            if (((((c.m.r32((l_w_3).wrapping_add(1460)) as i32)) & (4096i32))) != 0) {
                                { let a0_ = (c.m.r32(l_src_5) as i32); let a1_ = (c.m.r32((l_w_3).wrapping_add(1072)) as i32); F::BtlCharApi_GetNodePos(c, a0_, a1_, l_pos_6) };
                            }
                        }
                        if (((((((((c.m.r32((l_mgr_4).wrapping_add(800))) != (((0i32) as u32))) as i32)) != 0) && (((((c.m.r32((l_w_3).wrapping_add(1460)) as i32)) & (4096i32))) != 0)) as i32)) != 0) {
                            F::Vec3_Clamp(c, fp.wrapping_add(16), l_w_3, F32(0xbf800000), F32(0x3f800000));
                            { let v_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); F::Mathf_Asin(c, a0_) }; let a_ = fp; c.m.w32(a_, (v_).0); };
                            { let v_ = rt::fadd({ let a0_ = F32(c.m.r32(fp.wrapping_add(16))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) }, F32(0x40490fda)); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            { let v_ = { let a0_ = F32(c.m.r32(fp)); F::EftMath_WrapAngle(c, a0_) }; let a_ = fp; c.m.w32(a_, (v_).0); };
                            { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); F::EftMath_WrapAngle(c, a0_) }; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
                            F::EftObjTech_SetModelPose(c, p_task_1, l_pos_6, fp);
                        }
                        break 's1;
                    }
                }
                1i32 => {
                    if (((((c.m.r32((l_w_3).wrapping_add(1460)) as i32)) & (1024i32))) != 0) {
                        { let a0_ = { let a0_ = (c.m.r32(l_src_5) as i32); F::BtlCharApi_GetOpponentObjId(c, a0_) }; F::BtlCharApi_GetNodePos(c, a0_, 3i32, ((l_w_3).wrapping_add(80)).wrapping_add(16)) };
                    }
                    break 's1;
                }
                2i32 => {
                    if (({ let a0_ = (c.m.r32(l_src_5) as i32); F::EftShot_TestBits(c, a0_, 4i32) }) != 0) {
                        { let r_ = 16384i32; let a_ = (l_w_3).wrapping_add(1460); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    if (((((c.m.r32((l_w_3).wrapping_add(1460)) as i32)) & (16384i32))) != 0) {
                        l_id_9 = (c.m.r32(l_src_5) as i32);
                        if ((((!(((((c.m.r32((l_w_3).wrapping_add(1460)) as i32)) & (1024i32))) != 0)) as i32)) != 0) {
                            F::BtlCharApi_GetNodePos(c, l_id_9, 31i32, ((l_w_3).wrapping_add(16)).wrapping_add(16));
                        } else {
                            l_id_9 = F::BtlCharApi_GetOpponentObjId(c, l_id_9);
                            F::BtlCharApi_GetNodePos(c, l_id_9, 31i32, ((l_w_3).wrapping_add(80)).wrapping_add(16));
                        }
                        if (((((c.m.r32((l_mgr_4).wrapping_add(800))) != (((0i32) as u32))) as i32)) != 0) {
                            F::BtlCharApi_GetNodePos(c, l_id_9, 21i32, fp.wrapping_add(128));
                            F::BtlCharApi_GetNodePos(c, l_id_9, 25i32, fp.wrapping_add(144));
                            F::Vec3_Sub(c, fp.wrapping_add(176), fp.wrapping_add(144), fp.wrapping_add(128));
                            F::Vec3_Normalize(c, fp.wrapping_add(176), fp.wrapping_add(176));
                            F::BtlCharApi_GetNodePos(c, l_id_9, 31i32, fp.wrapping_add(144));
                            F::Vec3_Sub(c, fp.wrapping_add(48), fp.wrapping_add(144), fp.wrapping_add(128));
                            F::Vec3_Normalize(c, fp.wrapping_add(48), fp.wrapping_add(48));
                            F::Vec3_Cross(c, fp.wrapping_add(160), fp.wrapping_add(176), fp.wrapping_add(48));
                            F::Vec3_Cross(c, fp.wrapping_add(160), fp.wrapping_add(176), fp.wrapping_add(160));
                            F::Vec3_Normalize(c, fp.wrapping_add(160), fp.wrapping_add(160));
                            F::Vec3_Copy(c, (fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), fp.wrapping_add(48));
                            F::Vec3_Copy(c, (fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), fp.wrapping_add(160));
                            F::Vec3_Copy(c, (fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), fp.wrapping_add(176));
                            F::Mtx_AxisZToEuler(c, fp.wrapping_add(32), fp.wrapping_add(64));
                            F::BtlCharApi_GetNodePos(c, l_id_9, 31i32, fp.wrapping_add(128));
                            F::EftObjTech_SetModelPose(c, p_task_1, fp.wrapping_add(128), fp.wrapping_add(32));
                        }
                    }
                    break 's1;
                }
                3i32 => {
                    if (((((c.m.r32((l_w_3).wrapping_add(1460)) as i32)) & (1024i32))) != 0) {
                        { let a0_ = { let a0_ = (c.m.r32(l_src_5) as i32); F::BtlCharApi_GetOpponentObjId(c, a0_) }; F::BtlCharApi_GetNodePos(c, a0_, 17i32, ((l_w_3).wrapping_add(80)).wrapping_add(16)) };
                        F::Vec4_Set(c, l_w_3, F32(0x00000000), F32(0xbf800000), F32(0x00000000), F32(0x3f800000));
                        if ((((!(((((c.m.r32((l_w_3).wrapping_add(1460)) as i32)) & (2048i32))) != 0)) as i32)) != 0) {
                            if (((({ let a_ = (c.m.r32((l_src_5).wrapping_add(36))).wrapping_add(60); (c.m.r32(a_) as i32) }) & (262144i32))) != 0) {
                                l_step_10 = F32(0x00000000);
                                if ((((((({ let a_ = c.m.r32((l_src_5).wrapping_add(36)); (c.m.r16(a_) as i16) }) as i32)) == (360i32)) as i32)) != 0) {
                                    l_step_10 = rt::fmul({ let a0_ = { let a0_ = (c.m.r32(l_src_5) as i32); F::BtlCharApi_GetOpponentObjId(c, a0_) }; F::BtlScene_GetCharScale(c, a0_) }, F32(0xc0800000));
                                }
                                F::EftObjTech_SetRingParams(c, p_task_1, F32(0x00000000), l_step_10, F32(0x00000000));
                            }
                        }
                    }
                    if (({ let a0_ = (c.m.r32(l_src_5) as i32); F::EftShot_TestBits(c, a0_, 2i32) }) != 0) {
                        if (((((c.m.r32((l_w_3).wrapping_add(1460)) as i32)) & (32768i32))) != 0) {
                            { let r_ = 16384i32; let a_ = (l_w_3).wrapping_add(1460); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                    }
                    if (({ let a0_ = (c.m.r32(l_src_5) as i32); F::EftShot_TestBits(c, a0_, 4i32) }) != 0) {
                        { let r_ = 81920i32; let a_ = (l_w_3).wrapping_add(1460); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    if (({ let a0_ = (c.m.r32(l_src_5) as i32); F::EftShot_TestBits(c, a0_, 16i32) }) != 0) {
                        { let r_ = 131072i32; let a_ = (l_w_3).wrapping_add(1460); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    { let _ = { let a0_ = (c.m.r32(l_src_5) as i32); F::EftShot_TestBits(c, a0_, 32i32) }; };
                    if (((((((({ let a_ = (c.m.r32((l_src_5).wrapping_add(36))).wrapping_add(60); (c.m.r32(a_) as i32) }) & (262144i32))) != 0) && (((((c.m.r32((l_w_3).wrapping_add(1460)) as i32)) & (16384i32))) != 0)) as i32)) != 0) {
                        { let _ = F::memset(c, fp.wrapping_add(192), 0i32, 16u32); };
                        { let _ = F::memset(c, fp.wrapping_add(208), 0i32, 16u32); };
                        { let _ = F::memset(c, fp.wrapping_add(224), 0i32, 16u32); };
                        l_length_12 = rt::fadd(F32(c.m.r32((l_w_3).wrapping_add(1476))), F32(c.m.r32((l_w_3).wrapping_add(1476))));
                        l_width_11 = rt::fmul(F32(c.m.r32((l_w_3).wrapping_add(1476))), F32(0x3fc00000));
                        if (((((c.m.r32((l_w_3).wrapping_add(1460)) as i32)) & (1024i32))) != 0) {
                            l_opp_13 = { let a0_ = (c.m.r32(l_src_5) as i32); F::BtlCharApi_GetOpponentObjId(c, a0_) };
                            if ((((!(((((c.m.r32((l_w_3).wrapping_add(1460)) as i32)) & (131072i32))) != 0)) as i32)) != 0) {
                                { let r_ = rt::fmul(F::BtlScene_GetCharScale(c, l_opp_13), F32(0x3f4ccccc)); let n_ = rt::fmul(l_width_11, r_); l_width_11 = n_; };
                                { let r_ = rt::fmul(F::BtlScene_GetCharScale(c, l_opp_13), F32(0x3f4ccccc)); let n_ = rt::fmul(l_length_12, r_); l_length_12 = n_; };
                            } else {
                                { let r_ = rt::fmul(F::BtlScene_GetCharScale(c, l_opp_13), F32(0x3f000000)); let n_ = rt::fmul(l_width_11, r_); l_width_11 = n_; };
                                { let r_ = rt::fmul(F::BtlScene_GetCharScale(c, l_opp_13), F32(0x3f000000)); let n_ = rt::fmul(l_length_12, r_); l_length_12 = n_; };
                            }
                            F::Vec4_Copy(c, fp.wrapping_add(192), ((l_w_3).wrapping_add(80)).wrapping_add(16));
                        } else {
                            F::Vec3_Clamp(c, fp.wrapping_add(224), l_w_3, F32(0xbf800000), F32(0x3f800000));
                            { let v_ = { let a0_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(224)).wrapping_add(4)))); F::Mathf_Asin(c, a0_) }; let a_ = fp.wrapping_add(208); c.m.w32(a_, (v_).0); };
                            { let v_ = { let a0_ = F32(c.m.r32(fp.wrapping_add(224))); let a1_ = F32(c.m.r32((fp.wrapping_add(224)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) }; let a_ = (fp.wrapping_add(208)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            { let v_ = { let a0_ = F32(c.m.r32(fp.wrapping_add(208))); F::EftMath_WrapAngle(c, a0_) }; let a_ = fp.wrapping_add(208); c.m.w32(a_, (v_).0); };
                            { let v_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(208)).wrapping_add(4))); F::EftMath_WrapAngle(c, a0_) }; let a_ = (fp.wrapping_add(208)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(208)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                            if ((((!(((((c.m.r32((l_w_3).wrapping_add(1460)) as i32)) & (65536i32))) != 0)) as i32)) != 0) {
                                { let v_ = F32(0x00000000); let a_ = fp.wrapping_add(208); c.m.w32(a_, (v_).0); };
                                F::Vec4_Copy(c, fp.wrapping_add(192), (l_w_3).wrapping_add(896));
                            } else {
                                { let r_ = l_width_11; let n_ = rt::fadd(l_width_11, r_); l_width_11 = n_; };
                                { let r_ = l_length_12; let n_ = rt::fadd(l_length_12, r_); l_length_12 = n_; };
                                F::Vec4_Copy(c, fp.wrapping_add(192), ((l_w_3).wrapping_add(16)).wrapping_add(16));
                            }
                        }
                        F::EftObjTech_PlaceRings(c, p_task_1, fp.wrapping_add(192), fp.wrapping_add(208), l_width_11, l_length_12);
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftObjTech_UpdateMotion", &PROBE_N, &[], true); }
    r_
}

pub fn EftObjTech_UpdateModelVisible(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftObjTech_UpdateModelVisible", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_owner_2: u32 = 0;
    let mut l_w_3: u32 = 0;
    let mut l_mgr_4: u32 = 0;
    let mut l_show_5: i32 = 0;
    let mut l_model_6: u32 = 0;
    let mut l_src_7: u32 = 0;
    let r_ = 'f: {
    l_owner_2 = F::BtlTask_GetParent(c, p_task_1);
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    l_mgr_4 = c.m.r32((l_owner_2).wrapping_add(56));
    l_show_5 = 0i32;
    l_model_6 = (l_w_3).wrapping_add(1500);
    l_src_7 = c.m.r32((l_w_3).wrapping_add(144));
    's1: {
        let sv8: i32 = (c.m.r32((l_w_3).wrapping_add(1464)) as i32);
        match sv8 {
            0i32 => {
                if (((((c.m.r32((l_mgr_4).wrapping_add(800))) != (((0i32) as u32))) as i32)) != 0) {
                    if (((((c.m.r32((l_w_3).wrapping_add(1460)) as i32)) & (4096i32))) != 0) {
                        l_show_5 = 1i32;
                    }
                }
                break 's1;
            }
            1i32 => {
                break 's1;
            }
            2i32 => {
                if (((((c.m.r32((l_w_3).wrapping_add(1460)) as i32)) & (16384i32))) != 0) {
                    l_show_5 = (((c.m.r32((l_mgr_4).wrapping_add(800))) != (((0i32) as u32))) as i32);
                }
                break 's1;
            }
            _ => {
            }
        }
    }
    if ((l_show_5) != 0) {
        if ((((!(({ let a0_ = (c.m.r32(l_src_7) as i32); F::BtlScene_IsEffectHidden(c, a0_, 1i32) }) != 0)) as i32)) != 0) {
            { let a0_ = (((c.m.r8((l_model_6).wrapping_add(53)) as i8)) as i32); F::EftObj_SetVisible(c, a0_, 1i32) };
        } else {
            { let a0_ = (((c.m.r8((l_model_6).wrapping_add(53)) as i8)) as i32); F::EftObj_SetVisible(c, a0_, 0i32) };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftObjTech_UpdateModelVisible", &PROBE_N, &[], true); }
    r_
}

pub fn EftObjTech_Init(c: &mut Ctx, mut p_task_1: u32, mut p_src_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftObjTech_Init", &PROBE_N, &[(p_task_1) as u32, (p_src_2) as u32], false); }
    let mut l_owner_3: u32 = 0;
    let mut l_w_4: u32 = 0;
    let mut l_mgr_5: u32 = 0;
    let mut l_emit_6: u32 = 0;
    let mut l_def_7: u32 = 0;
    let mut l_roll_8: F32 = F32(0);
    let mut l_step_9: F32 = F32(0);
    let mut l_grow_10: F32 = F32(0);
    let r_ = 'f: {
    l_owner_3 = F::BtlTask_GetParent(c, p_task_1);
    l_w_4 = c.m.r32((p_task_1).wrapping_add(56));
    l_mgr_5 = c.m.r32((l_owner_3).wrapping_add(56));
    { let _ = F::memset(c, l_w_4, 0i32, 2048u32); };
    l_emit_6 = (l_w_4).wrapping_add(148);
    { let v_ = p_src_2; let a_ = (l_w_4).wrapping_add(144); c.m.w32(a_, v_); };
    l_def_7 = c.m.r32((p_src_2).wrapping_add(36));
    { let v_ = F32(c.m.r32((l_def_7).wrapping_add(52))); let a_ = (l_w_4).wrapping_add(1468); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_def_7).wrapping_add(48))); let a_ = (l_w_4).wrapping_add(1472); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_def_7).wrapping_add(48))); let a_ = (l_w_4).wrapping_add(1476); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_w_4).wrapping_add(1472))); let a_ = (l_w_4).wrapping_add(1480); c.m.w32(a_, (v_).0); };
    { let v_ = l_mgr_5; let a_ = (l_w_4).wrapping_add(1456); c.m.w32(a_, v_); };
    F::EftEmit_InitState(c, l_mgr_5, l_emit_6);
    if (({ let a0_ = c.m.r32((l_w_4).wrapping_add(1456)); F::EftEmit_HasWidth2(c, a0_) }) != 0) {
        { let v_ = rt::fmul(F::EftEmit_GetWidth2(c, l_emit_6), F32(c.m.r32((l_def_7).wrapping_add(48)))); let a_ = (l_w_4).wrapping_add(1468); c.m.w32(a_, (v_).0); };
        { let r_ = 512i32; let a_ = (l_w_4).wrapping_add(1460); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    { let v_ = rt::i2f(({ let a0_ = c.m.r32((l_w_4).wrapping_add(1456)); ((((F::EftEmit_GetEndFrames(c, a0_)) as i64 as u64)) as u8) }) as i32); let a_ = (l_w_4).wrapping_add(1492); c.m.w32(a_, (v_).0); };
    if ((((((((c.m.r16(l_def_7) as i16)) as i32)) == (642i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (l_w_4).wrapping_add(1464); c.m.w32(a_, (v_) as u32); };
    } else {
        if ((((((((c.m.r16(l_def_7) as i16)) as i32)) == (680i32)) as i32)) != 0) {
            { let v_ = 1i32; let a_ = (l_w_4).wrapping_add(1464); c.m.w32(a_, (v_) as u32); };
        } else {
            if ((((((((c.m.r16(l_def_7) as i16)) as i32)) == (354i32)) as i32)) != 0) {
                { let v_ = 2i32; let a_ = (l_w_4).wrapping_add(1464); c.m.w32(a_, (v_) as u32); };
            } else {
                if ((((((((c.m.r16(l_def_7) as i16)) as i32)) == (499i32)) as i32)) != 0) {
                    { let v_ = 0i32; let a_ = (l_w_4).wrapping_add(1464); c.m.w32(a_, (v_) as u32); };
                } else {
                    if ((((((((c.m.r16(l_def_7) as i16)) as i32)) == (359i32)) as i32)) != 0) {
                        { let v_ = 3i32; let a_ = (l_w_4).wrapping_add(1464); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 1i32; let a_ = ((l_w_4).wrapping_add(1568)).wrapping_add(468); c.m.w32(a_, (v_) as u32); };
                        { let r_ = 32768i32; let a_ = (l_w_4).wrapping_add(1460); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    } else {
                        if ((((((((c.m.r16(l_def_7) as i16)) as i32)) == (360i32)) as i32)) != 0) {
                            { let v_ = 3i32; let a_ = (l_w_4).wrapping_add(1464); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 3i32; let a_ = ((l_w_4).wrapping_add(1568)).wrapping_add(468); c.m.w32(a_, (v_) as u32); };
                        }
                    }
                }
            }
        }
    }
    if (((((c.m.r32((l_mgr_5).wrapping_add(800))) != (((0i32) as u32))) as i32)) != 0) {
        F::EftObjTech_CreateModel(c, p_task_1);
    }
    if (((({ let a_ = (c.m.r32((p_src_2).wrapping_add(36))).wrapping_add(60); (c.m.r32(a_) as i32) }) & (262144i32))) != 0) {
        l_roll_8 = F32(0x00000000);
        l_step_9 = F32(0x00000000);
        l_grow_10 = F32(0x00000000);
        if ((((((((c.m.r16(l_def_7) as i16)) as i32)) == (360i32)) as i32)) != 0) {
            l_roll_8 = F32(0xbfc90fda);
            l_step_9 = F32(0xc1a00000);
            l_grow_10 = F32(0x3e999999);
        }
        F::EftObjTech_InitRings(c, p_task_1, l_roll_8, l_step_9, l_grow_10);
    }
    { let a1_ = (if ((((((c.m.r32(p_src_2) as i32)) == (0i32)) as i32)) != 0) { 2048i32 } else { 4096i32 }); F::BtlTask_SetOwnerTag(c, p_task_1, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftObjTech_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftObjTech_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftObjTech_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_owner_2: u32 = 0;
    let mut l_w_3: u32 = 0;
    let mut l_src_4: u32 = 0;
    let r_ = 'f: {
    l_owner_2 = F::BtlTask_GetParent(c, p_task_1);
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    l_src_4 = c.m.r32((l_w_3).wrapping_add(144));
    if ((((({ let a_ = (c.m.r32((l_owner_2).wrapping_add(56))).wrapping_add(800); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        F::EftObjTech_DestroyModel(c, p_task_1);
    }
    if (((({ let a_ = (c.m.r32((l_src_4).wrapping_add(36))).wrapping_add(60); (c.m.r32(a_) as i32) }) & (262144i32))) != 0) {
        F::EftObjTech_TermRings(c, p_task_1);
    }
    { let a0_ = c.m.r32((l_w_3).wrapping_add(1456)); F::EftEmit_TermState(c, a0_, (l_w_3).wrapping_add(148)) };
    if ((((((({ let a_ = (c.m.r32((l_src_4).wrapping_add(36))).wrapping_add(4); (c.m.r8(a_) as i8) }) as i32)) != (0i32)) as i32)) != 0) {
        { let a0_ = (c.m.r32(l_src_4) as i32); F::EftShot_SetHeldFlagA8(c, a0_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftObjTech_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftObjTech_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftObjTech_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_advance_2: i32 = 0;
    let mut l_w_3: u32 = 0;
    let mut l_src_4: u32 = 0;
    let mut sv_5: i32 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
    'fl1: loop {
        match bb_ {
            0 => {
                l_advance_2 = 0i32;
                l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
                l_src_4 = c.m.r32((l_w_3).wrapping_add(144));
                if (({ let a0_ = (c.m.r32(l_src_4) as i32); F::BtlScene_IsCharStopped(c, a0_) }) != 0) { bb_ = 1; continue 'fl1; } else { bb_ = 2; continue 'fl1; }
            }
            1 => {
                break 'f;
            }
            2 => {
                F::EftEmit_UpdateNodesReq(c, l_src_4, (l_w_3).wrapping_add(864));
                if ((((!(((((c.m.r32((l_w_3).wrapping_add(1460)) as i32)) & (128i32))) != 0)) as i32)) != 0) { bb_ = 4; continue 'fl1; } else { bb_ = 5; continue 'fl1; }
            }
            3 => {
                bb_ = 2; continue 'fl1;
            }
            4 => {
                if (({ let a0_ = (c.m.r32(l_src_4) as i32); F::EftShot_TestBits(c, a0_, 2i32) }) != 0) { bb_ = 6; continue 'fl1; } else { bb_ = 8; continue 'fl1; }
            }
            5 => {
                sv_5 = ((c.m.r8((p_task_1).wrapping_add(1))) as i32);
                bb_ = match sv_5 { 0i32 => 17, 1i32 => 21, 2i32 => 23, 3i32 => 38, _ => 15, }; continue 'fl1;
            }
            6 => {
                { let a2_ = (c.m.r32(l_src_4) as i32); { let _ = F::EftAim_GetDir(c, l_w_3, (((((l_w_3).wrapping_add(896)) as u64)) as i32), a2_); } };
                { let v_ = ((0i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                { let r_ = 128i32; let a_ = (l_w_3).wrapping_add(1460); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                bb_ = 7; continue 'fl1;
            }
            7 => {
                if ((((!(((((c.m.r32((l_w_3).wrapping_add(1460)) as i32)) & (128i32))) != 0)) as i32)) != 0) { bb_ = 11; continue 'fl1; } else { bb_ = 12; continue 'fl1; }
            }
            8 => {
                if (({ let a0_ = (c.m.r32(l_src_4) as i32); F::EftShot_TestBits(c, a0_, 4i32) }) != 0) { bb_ = 9; continue 'fl1; } else { bb_ = 10; continue 'fl1; }
            }
            9 => {
                { let v_ = ((0i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                { let r_ = 128i32; let a_ = (l_w_3).wrapping_add(1460); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                bb_ = 10; continue 'fl1;
            }
            10 => {
                bb_ = 7; continue 'fl1;
            }
            11 => {
                bb_ = 13; continue 'fl1;
            }
            12 => {
                bb_ = 5; continue 'fl1;
            }
            13 => {
                F::EftObjTech_UpdateMotion(c, p_task_1);
                if (({ let a0_ = (c.m.r32(l_src_4) as i32); F::EftShot_TestBits(c, a0_, 1024i32) }) != 0) { bb_ = 40; continue 'fl1; } else { bb_ = 41; continue 'fl1; }
            }
            14 => {
                bb_ = 12; continue 'fl1;
            }
            15 => {
                bb_ = 13; continue 'fl1;
            }
            16 => {
                bb_ = 17; continue 'fl1;
            }
            17 => {
                if (({ let a0_ = (c.m.r32(l_src_4) as i32); F::EftShot_TestBits(c, a0_, 4i32) }) != 0) { bb_ = 18; continue 'fl1; } else { bb_ = 19; continue 'fl1; }
            }
            18 => {
                { let r_ = 32i32; let a_ = (l_w_3).wrapping_add(1460); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                F::Vec4_Copy(c, (l_w_3).wrapping_add(16), (l_w_3).wrapping_add(960));
                F::Vec4_Copy(c, ((l_w_3).wrapping_add(16)).wrapping_add(16), (l_w_3).wrapping_add(16));
                { let a0_ = (c.m.r32(l_src_4) as i32); F::BtlCharApi_GetNodePos(c, a0_, 17i32, ((l_w_3).wrapping_add(16)).wrapping_add(32)) };
                { let a3_ = (c.m.r32(l_src_4) as i32); { let _ = F::EftAim_GetDirKeep(c, l_src_4, l_w_3, ((((((l_w_3).wrapping_add(16)).wrapping_add(16)) as u64)) as i32), a3_); } };
                { let a2_ = F32(c.m.r32((l_w_3).wrapping_add(1468))); F::Vec3_Scale(c, ((l_w_3).wrapping_add(16)).wrapping_add(48), l_w_3, a2_) };
                { let v_ = ((1i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                bb_ = 19; continue 'fl1;
            }
            19 => {
                bb_ = 15; continue 'fl1;
            }
            20 => {
                bb_ = 21; continue 'fl1;
            }
            21 => {
                l_advance_2 = 1i32;
                bb_ = 15; continue 'fl1;
            }
            22 => {
                bb_ = 23; continue 'fl1;
            }
            23 => {
                if ((((!(((((c.m.r32((l_w_3).wrapping_add(1460)) as i32)) & (1024i32))) != 0)) as i32)) != 0) { bb_ = 24; continue 'fl1; } else { bb_ = 25; continue 'fl1; }
            }
            24 => {
                if ((((!(((((c.m.r32((l_w_3).wrapping_add(1460)) as i32)) & (64i32))) != 0)) as i32)) != 0) { bb_ = 26; continue 'fl1; } else { bb_ = 27; continue 'fl1; }
            }
            25 => {
                if (({ let a0_ = (c.m.r32(l_src_4) as i32); F::EftShot_TestBits(c, a0_, 8i32) }) != 0) { bb_ = 35; continue 'fl1; } else { bb_ = 36; continue 'fl1; }
            }
            26 => {
                F::Vec4_Copy(c, ((l_w_3).wrapping_add(16)).wrapping_add(32), ((l_w_3).wrapping_add(16)).wrapping_add(16));
                bb_ = 27; continue 'fl1;
            }
            27 => {
                if ((((!(((({ let a_ = ({ let a_ = (c.m.r32((l_w_3).wrapping_add(144))).wrapping_add(36); c.m.r32(a_) }).wrapping_add(60); (c.m.r32(a_) as i32) }) & (2i32))) != 0)) as i32)) != 0) { bb_ = 28; continue 'fl1; } else { bb_ = 29; continue 'fl1; }
            }
            28 => {
                if ((((!(((((c.m.r32((l_w_3).wrapping_add(1460)) as i32)) & (64i32))) != 0)) as i32)) != 0) { bb_ = 30; continue 'fl1; } else { bb_ = 32; continue 'fl1; }
            }
            29 => {
                bb_ = 25; continue 'fl1;
            }
            30 => {
                if (((rt::flt(F32(0x00000000), { let a_ = (c.m.r32((l_src_4).wrapping_add(36))).wrapping_add(56); F32(c.m.r32(a_)) }) as i32)) != 0) { bb_ = 33; continue 'fl1; } else { bb_ = 34; continue 'fl1; }
            }
            31 => {
                bb_ = 29; continue 'fl1;
            }
            32 => {
                F::Vec3_Copy(c, ((l_w_3).wrapping_add(16)).wrapping_add(16), (p_task_1).wrapping_add(16));
                bb_ = 31; continue 'fl1;
            }
            33 => {
                { let a3_ = (c.m.r32(l_src_4) as i32); let a4_ = F32(c.m.r32((l_w_3).wrapping_add(1468))); let a5_ = { let a_ = (c.m.r32((l_src_4).wrapping_add(36))).wrapping_add(56); F32(c.m.r32(a_)) }; F::EftAim_Home(c, l_w_3, ((l_w_3).wrapping_add(16)).wrapping_add(16), l_w_3, a3_, a4_, a5_) };
                { let a2_ = F32(c.m.r32((l_w_3).wrapping_add(1468))); F::Vec3_Scale(c, ((l_w_3).wrapping_add(16)).wrapping_add(48), l_w_3, a2_) };
                bb_ = 34; continue 'fl1;
            }
            34 => {
                F::Vec3_Add(c, ((l_w_3).wrapping_add(16)).wrapping_add(16), ((l_w_3).wrapping_add(16)).wrapping_add(16), ((l_w_3).wrapping_add(16)).wrapping_add(48));
                bb_ = 31; continue 'fl1;
            }
            35 => {
                { let r_ = 19i32; let a_ = (l_w_3).wrapping_add(1460); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = ((3i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                bb_ = 36; continue 'fl1;
            }
            36 => {
                bb_ = 15; continue 'fl1;
            }
            37 => {
                bb_ = 38; continue 'fl1;
            }
            38 => {
                l_advance_2 = 1i32;
                bb_ = 15; continue 'fl1;
            }
            39 => {
                bb_ = 15; continue 'fl1;
            }
            40 => {
                if ((((!(((((c.m.r32((l_w_3).wrapping_add(1460)) as i32)) & (16i32))) != 0)) as i32)) != 0) { bb_ = 42; continue 'fl1; } else { bb_ = 43; continue 'fl1; }
            }
            41 => {
                { let a0_ = (c.m.r32(l_src_4) as i32); let a2_ = c.m.r32((l_w_3).wrapping_add(1456)); F::EftObjTech_UpdateEmitters(c, a0_, p_task_1, a2_) };
                if (((((c.m.r32((l_w_3).wrapping_add(1460)) as i32)) & (32i32))) != 0) { bb_ = 44; continue 'fl1; } else { bb_ = 45; continue 'fl1; }
            }
            42 => {
                { let r_ = 8i32; let a_ = (l_w_3).wrapping_add(1460); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                bb_ = 43; continue 'fl1;
            }
            43 => {
                { let r_ = 2i32; let a_ = (l_w_3).wrapping_add(1460); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                bb_ = 41; continue 'fl1;
            }
            44 => {
                if (((((c.m.r32((l_w_3).wrapping_add(1460)) as i32)) & (512i32))) != 0) { bb_ = 46; continue 'fl1; } else { bb_ = 47; continue 'fl1; }
            }
            45 => {
                if ((l_advance_2) != 0) { bb_ = 48; continue 'fl1; } else { bb_ = 49; continue 'fl1; }
            }
            46 => {
                { let a0_ = c.m.r32((l_w_3).wrapping_add(1456)); F::EftEmit_UpdateWidth2(c, a0_, (l_w_3).wrapping_add(148)) };
                { let v_ = rt::fmul(F::EftEmit_GetWidth2(c, (l_w_3).wrapping_add(148)), { let a_ = (c.m.r32((l_src_4).wrapping_add(36))).wrapping_add(48); F32(c.m.r32(a_)) }); let a_ = (l_w_3).wrapping_add(1468); c.m.w32(a_, (v_).0); };
                { let a2_ = F32(c.m.r32((l_w_3).wrapping_add(1468))); F::Vec3_Scale(c, ((l_w_3).wrapping_add(16)).wrapping_add(48), l_w_3, a2_) };
                bb_ = 47; continue 'fl1;
            }
            47 => {
                bb_ = 45; continue 'fl1;
            }
            48 => {
                { let a_ = (p_task_1).wrapping_add(1); let o_ = c.m.r8(a_); let n_ = (o_).wrapping_add(1); c.m.w8(a_, n_); };
                bb_ = 49; continue 'fl1;
            }
            49 => {
                if (((((c.m.r32((l_w_3).wrapping_add(1460)) as i32)) & (1i32))) != 0) { bb_ = 50; continue 'fl1; } else { bb_ = 51; continue 'fl1; }
            }
            50 => {
                { let r_ = F32(0x3f800000); let a_ = (l_w_3).wrapping_add(1488); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                bb_ = 51; continue 'fl1;
            }
            51 => {
                if (((((c.m.r32((l_w_3).wrapping_add(1460)) as i32)) & (4i32))) != 0) { bb_ = 52; continue 'fl1; } else { bb_ = 54; continue 'fl1; }
            }
            52 => {
                F::BtlTask_SetDead(c, p_task_1);
                bb_ = 53; continue 'fl1;
            }
            53 => {
                break 'fl1;
            }
            54 => {
                if (((((c.m.r32((l_w_3).wrapping_add(1460)) as i32)) & (2i32))) != 0) { bb_ = 55; continue 'fl1; } else { bb_ = 57; continue 'fl1; }
            }
            55 => {
                if (((((c.m.r32((l_w_3).wrapping_add(1460)) as i32)) & (8i32))) != 0) { bb_ = 58; continue 'fl1; } else { bb_ = 60; continue 'fl1; }
            }
            56 => {
                bb_ = 53; continue 'fl1;
            }
            57 => {
                if (((({ let a_ = (c.m.r32((l_src_4).wrapping_add(36))).wrapping_add(60); (c.m.r32(a_) as i32) }) & (1i32))) != 0) { bb_ = 63; continue 'fl1; } else { bb_ = 64; continue 'fl1; }
            }
            58 => {
                { let r_ = 4i32; let a_ = (l_w_3).wrapping_add(1460); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                bb_ = 59; continue 'fl1;
            }
            59 => {
                bb_ = 56; continue 'fl1;
            }
            60 => {
                if (((rt::fge(F32(c.m.r32((l_w_3).wrapping_add(1488))), F32(c.m.r32((l_w_3).wrapping_add(1492)))) as i32)) != 0) { bb_ = 61; continue 'fl1; } else { bb_ = 62; continue 'fl1; }
            }
            61 => {
                { let r_ = 4i32; let a_ = (l_w_3).wrapping_add(1460); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                bb_ = 62; continue 'fl1;
            }
            62 => {
                bb_ = 59; continue 'fl1;
            }
            63 => {
                if ((((!(((((c.m.r32((l_w_3).wrapping_add(1460)) as i32)) & (32i32))) != 0)) as i32)) != 0) { bb_ = 65; continue 'fl1; } else { bb_ = 66; continue 'fl1; }
            }
            64 => {
                bb_ = 56; continue 'fl1;
            }
            65 => {
                break 'f;
            }
            66 => {
                { let a0_ = c.m.r32((l_w_3).wrapping_add(1456)); F::EftEmit_UpdateTrailWidth(c, a0_, (l_w_3).wrapping_add(148)) };
                F::EftObjTech_AddHitRecord(c, p_task_1);
                bb_ = 64; continue 'fl1;
            }
            67 => {
                bb_ = 66; continue 'fl1;
            }
            _ => unreachable!(),
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftObjTech_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftObjTech_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftObjTech_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_2: u32 = 0;
    let mut l_src_3: u32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_src_3 = c.m.r32((l_w_2).wrapping_add(144));
        F::EftObjTech_UpdateModelVisible(c, p_task_1);
        if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::BtlScene_IsCharStopped(c, a0_) }) != 0) {
            break 'f;
        }
        { let _ = { let a0_ = c.m.r32((l_w_2).wrapping_add(1456)); F::EftEmit_UpdateAlive(c, a0_, (l_w_2).wrapping_add(148)) }; };
        if ((((((c.m.r16((p_task_1).wrapping_add(8))) as i32)) & (1i32))) != 0) {
            F::Vec3_Sub(c, fp, (p_task_1).wrapping_add(16), ((l_w_2).wrapping_add(16)).wrapping_add(16));
            F::Vec3_Copy(c, ((l_w_2).wrapping_add(16)).wrapping_add(16), (p_task_1).wrapping_add(16));
            F::Vec3_Add(c, ((l_w_2).wrapping_add(16)).wrapping_add(32), ((l_w_2).wrapping_add(16)).wrapping_add(32), fp);
            { let r_ = 64i32; let a_ = (l_w_2).wrapping_add(1460); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if ((((c.m.r32((p_task_1).wrapping_add(4))) & (((1i32) as u32)))) != 0) {
            if ((((!((((c.m.r32((p_task_1).wrapping_add(4))) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
                { let a0_ = (c.m.r32(l_src_3) as i32); F::EftTechEvt_RequestStop(c, a0_) };
                { let v_ = (((((c.m.r32((l_w_2).wrapping_add(1460)) as i32)) & ((!(32i32))))) | (1024i32)); let a_ = (l_w_2).wrapping_add(1460); c.m.w32(a_, (v_) as u32); };
                { let a0_ = c.m.r32((l_w_2).wrapping_add(1456)); F::EftEmit_MarkKind6(c, a0_, (l_w_2).wrapping_add(148)) };
            } else {
                { let v_ = (((((c.m.r32((l_w_2).wrapping_add(1460)) as i32)) | (2i32))) & ((!(32i32)))); let a_ = (l_w_2).wrapping_add(1460); c.m.w32(a_, (v_) as u32); };
            }
        } else {
            if ((((c.m.r32((p_task_1).wrapping_add(4))) & (((2i32) as u32)))) != 0) {
                { let v_ = (((((c.m.r32((l_w_2).wrapping_add(1460)) as i32)) | (2i32))) & ((!(32i32)))); let a_ = (l_w_2).wrapping_add(1460); c.m.w32(a_, (v_) as u32); };
            } else {
                if ((((c.m.r32((p_task_1).wrapping_add(4))) & (((4i32) as u32)))) != 0) {
                    { let v_ = (((((c.m.r32((l_w_2).wrapping_add(1460)) as i32)) | (2i32))) & ((!(32i32)))); let a_ = (l_w_2).wrapping_add(1460); c.m.w32(a_, (v_) as u32); };
                }
            }
        }
        if ((((((c.m.r16((p_task_1).wrapping_add(8))) as i32)) & (4i32))) != 0) {
            { let v_ = (((((c.m.r32((l_w_2).wrapping_add(1460)) as i32)) | (1i32))) & ((!(32i32)))); let a_ = (l_w_2).wrapping_add(1460); c.m.w32(a_, (v_) as u32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftObjTech_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftObjTech_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftObjTech_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(1460)) as i32)) & (256i32))) != 0)) as i32)) != 0) {
        { let r_ = 256i32; let a_ = (l_w_2).wrapping_add(1460); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let a0_ = c.m.r32((l_w_2).wrapping_add(1456)); F::EftEmit_KillAll(c, a0_, (l_w_2).wrapping_add(148)) };
    }
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftObjTech_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftObjTech_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftObjTech_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((((({ let a_ = ({ let a_ = (c.m.r32((l_w_2).wrapping_add(144))).wrapping_add(36); c.m.r32(a_) }).wrapping_add(60); (c.m.r32(a_) as i32) }) & (262144i32))) != 0) && (((((c.m.r32((l_w_2).wrapping_add(1460)) as i32)) & (16384i32))) != 0)) as i32)) != 0) {
        F::EftObjTech_DrawRings(c, p_task_1);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftObjTech_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftObjTechMgr_Init(c: &mut Ctx, mut p_task_1: u32, mut p_src_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftObjTechMgr_Init", &PROBE_N, &[(p_task_1) as u32, (p_src_2) as u32], false); }
    let mut l_mgr_3: u32 = 0;
    let mut l_pack_4: u32 = 0;
    let r_ = 'f: {
    l_mgr_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let _ = ((1488u32) as i32); F::EftShot_Nop(c) };
    { let _ = F::memset(c, l_mgr_3, 0i32, 1488u32); };
    { let a3_ = c.m.r32((p_src_2).wrapping_add(28)); F::EftEmit_LoadSet(c, p_src_2, l_mgr_3, ((((0i32) as i64 as u64)) as u32), a3_, 0i32, 4i32) };
    { let _ = F::BtlTask_CreateChildList(c, p_task_1, ((((2i32) as i64 as u64)) as u32), ((((((2048u32) as i32)) as i64 as u64)) as u32)); };
    if (((({ let a_ = (c.m.r32((p_src_2).wrapping_add(36))).wrapping_add(60); (c.m.r32(a_) as i32) }) & (131072i32))) != 0) {
        { let v_ = { let a0_ = c.m.r32((p_src_2).wrapping_add(28)); F::BtlScene_GetPackEntry(c, a0_, 1i32) }; let a_ = (l_mgr_3).wrapping_add(800); c.m.w32(a_, v_); };
    }
    if (((({ let a_ = (c.m.r32((p_src_2).wrapping_add(36))).wrapping_add(60); (c.m.r32(a_) as i32) }) & (262144i32))) != 0) {
        l_pack_4 = { let a0_ = c.m.r32((p_src_2).wrapping_add(28)); F::BtlScene_GetPackEntry(c, a0_, 1i32) };
        { let v_ = F::BtlScene_GetPackEntry(c, l_pack_4, 1i32); let a_ = (l_mgr_3).wrapping_add(804); c.m.w32(a_, v_); };
        { let v_ = F::BtlScene_GetPackEntry(c, l_pack_4, 2i32); let a_ = (l_mgr_3).wrapping_add(808); c.m.w32(a_, v_); };
        { let a1_ = c.m.r32((l_mgr_3).wrapping_add(804)); F::EftMesh_Init(c, (l_mgr_3).wrapping_add(816), a1_) };
        { let a1_ = c.m.r32((l_mgr_3).wrapping_add(808)); F::EftTexSet_Load32(c, (l_mgr_3).wrapping_add(960), a1_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftObjTechMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftObjTechMgr_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftObjTechMgr_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32((p_task_1).wrapping_add(56)); F::EftEmit_FreeSet(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftObjTechMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftObjTechMgr_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftObjTechMgr_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_mgr_2: u32 = 0;
    let r_ = 'f: {
    l_mgr_2 = c.m.r32((p_task_1).wrapping_add(56));
    F::EftEmit_BeginFrame(c, l_mgr_2);
    if ((((((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(36))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) && (((((c.m.r32((l_mgr_2).wrapping_add(808))) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        F::EftTexSet_Keep32(c, (l_mgr_2).wrapping_add(960), 1i32, 0i32);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftObjTechMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftObjTechMgr_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftObjTechMgr_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftObjTechMgr_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftRushShot_AddHitRecord(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRushShot_AddHitRecord", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_rec_3: u32 = 0;
    let mut l_size_4: F32 = F32(0);
    let mut l_shape_5: u32 = 0;
    let mut l_a_6: u32 = 0;
    let mut l_b_7: u32 = 0;
    let mut l_a_8: u32 = 0;
    let mut l_b_9: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_rec_3 = F::EftHit_GetNew(c);
    l_size_4 = rt::fmul(F32(c.m.r32((l_w_2).wrapping_add(1432))), F::EftEmit_GetTrailWidth(c, (l_w_2).wrapping_add(100)));
    l_shape_5 = (l_w_2).wrapping_add(32);
    { let s_ = l_shape_5; let d_ = (l_rec_3).wrapping_add(16); rt::memcpy(c, d_, s_, 64); };
    { let v_ = p_task_1; let a_ = (l_rec_3).wrapping_add(96); c.m.w32(a_, v_); };
    { let v_ = c.m.r32((l_w_2).wrapping_add(96)); let a_ = (l_rec_3).wrapping_add(100); c.m.w32(a_, v_); };
    's1: {
        let sv10: i32 = (({ let a_ = ({ let a_ = (c.m.r32((l_w_2).wrapping_add(96))).wrapping_add(36); c.m.r32(a_) }).wrapping_add(8); (c.m.r8(a_) as i8) }) as i32);
        match sv10 {
            1i32 => {
                {
                    l_a_6 = F::EftHitArena_AllocBox(c);
                    l_b_7 = F::EftHitArena_AllocBox(c);
                    F::ColCapsule_Set(c, l_a_6, (l_w_2).wrapping_add(912), ((l_w_2).wrapping_add(32)).wrapping_add(16), l_size_4);
                    F::ColCapsule_Set(c, l_b_7, (l_w_2).wrapping_add(912), ((l_w_2).wrapping_add(32)).wrapping_add(32), l_size_4);
                    F::EftHit_SetShapeBoxes(c, l_rec_3, l_a_6, l_b_7);
                    break 's1;
                }
            }
            0i32 => {
                {
                    l_a_8 = F::EftHitArena_AllocSphere(c);
                    l_b_9 = F::EftHitArena_AllocSphere(c);
                    F::ColSphere_Set(c, l_a_8, ((l_w_2).wrapping_add(32)).wrapping_add(16), l_size_4);
                    F::ColSphere_Set(c, l_b_9, ((l_w_2).wrapping_add(32)).wrapping_add(32), l_size_4);
                    F::EftHit_SetShapeSpheres(c, l_rec_3, l_a_8, l_b_9);
                    break 's1;
                }
            }
            _ => {
                break 'f;
            }
        }
    }
    F::EftHit_Add(c, l_rec_3);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRushShot_AddHitRecord", &PROBE_N, &[], true); }
    r_
}

pub fn EftRushShot_CreateModels(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRushShot_CreateModels", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_owner_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_w_4: u32 = 0;
    let mut l_mgr_5: u32 = 0;
    let mut l_model_6: u32 = 0;
    let r_ = 'f: {
    l_owner_2 = F::BtlTask_GetParent(c, p_task_1);
    l_w_4 = c.m.r32((p_task_1).wrapping_add(56));
    l_mgr_5 = c.m.r32((l_owner_2).wrapping_add(56));
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < ((c.m.r32((l_w_4).wrapping_add(1488)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                l_model_6 = ((l_w_4).wrapping_add(1504)).wrapping_add((((l_i_3) as u32)).wrapping_mul(176));
                { let v_ = c.m.r32((l_mgr_5).wrapping_add(800)); let a_ = l_model_6; c.m.w32(a_, v_); };
                { let v_ = (({ let a1_ = c.m.r32(l_model_6); F::EftObj_Create(c, (l_model_6).wrapping_add(4), a1_) }) as i8); let a_ = (l_model_6).wrapping_add(53); c.m.w8(a_, (v_) as u8); };
                { let v_ = (((1i32).wrapping_neg()) as i8); let a_ = (l_model_6).wrapping_add(54); c.m.w8(a_, (v_) as u8); };
                { let a0_ = (((c.m.r8((l_model_6).wrapping_add(53)) as i8)) as i32); F::EftObj_SetVisible(c, a0_, 0i32) };
                { let a0_ = (((c.m.r8((l_model_6).wrapping_add(53)) as i8)) as i32); let _ = a0_; let _ = 0i32; F::EftObj_Nop(c) };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRushShot_CreateModels", &PROBE_N, &[], true); }
    r_
}

pub fn EftRushShot_UpdateModels(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRushShot_UpdateModels", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(240) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 240);
    let mut l_i_2: i32 = 0;
    let mut l_w_3: u32 = 0;
    let mut l_o_4: u32 = 0;
    let mut l_ang_5: F32 = F32(0);
    let mut l_pitch_6: F32 = F32(0);
    let mut l_yaw_7: F32 = F32(0);
    let r_ = 'f: {
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        F::Mtx_StoreIdentity(c, fp);
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < ((c.m.r32((l_w_3).wrapping_add(1488)) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_o_4 = ((l_w_3).wrapping_add(1504)).wrapping_add((((l_i_2) as u32)).wrapping_mul(176));
                    if ((((((c.m.r32((l_w_3).wrapping_add(1416)) as i32)) == (1i32)) as i32)) != 0) {
                        if ((((!(((((c.m.r32((l_w_3).wrapping_add(1412)) as i32)) & (262144i32))) != 0)) as i32)) != 0) {
                            if ((((!((((((c.m.r8((l_o_4).wrapping_add(52))) as i32)) & (1i32))) != 0)) as i32)) != 0) {
                                l_ang_5 = rt::fmul(rt::fdiv(rt::i2f(l_i_2), rt::i2f((c.m.r32((l_w_3).wrapping_add(1488)) as i32))), F32(0x40c90fda));
                                if (((rt::fgt(l_ang_5, F32(0x40490fda)) as i32)) != 0) {
                                    { let r_ = F32(0x40c90fda); let n_ = rt::fsub(l_ang_5, r_); l_ang_5 = n_; };
                                }
                                F::Mtx_StoreIdentity(c, fp.wrapping_add(176));
                                F::Mtx_RotateY(c, fp.wrapping_add(176), fp.wrapping_add(176), l_ang_5);
                                F::Vec4_Copy(c, (l_o_4).wrapping_add(96), (l_w_3).wrapping_add(848));
                                { let a2_ = rt::fmul(rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x41000000)), F32(0x40400000)), F32(c.m.r32((l_w_3).wrapping_add(1428)))); let a3_ = rt::fmul(rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40800000)), F32(0x40c00000)), F32(c.m.r32((l_w_3).wrapping_add(1428)))); F::Vec4_Set(c, (l_o_4).wrapping_add(112), F32(0x00000000), a2_, a3_, F32(0x3f800000)) };
                                F::Mtx_MulVec4(c, (l_o_4).wrapping_add(112), fp.wrapping_add(176), (l_o_4).wrapping_add(112));
                                F::Vec4_Set(c, (l_o_4).wrapping_add(128), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x00000000));
                                { let v_ = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40490fda)); let a_ = (l_o_4).wrapping_add(144); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40490fda)); let a_ = ((l_o_4).wrapping_add(144)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x00000000); let a_ = ((l_o_4).wrapping_add(144)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fmul(rt::fmul(rt::fsub(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff))), F32(0x3da3d70a)), F32(0x40490fda)); let a_ = (l_o_4).wrapping_add(160); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fmul(rt::fmul(rt::fsub(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff))), F32(0x3da3d70a)), F32(0x40490fda)); let a_ = ((l_o_4).wrapping_add(160)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x00000000); let a_ = ((l_o_4).wrapping_add(160)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                if (((((((l_i_2) & (3i32))) == (0i32)) as i32)) != 0) {
                                    { let v_ = F32(0x00000000); let a_ = (l_o_4).wrapping_add(60); c.m.w32(a_, (v_).0); };
                                } else {
                                    { let v_ = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40800000)); let a_ = (l_o_4).wrapping_add(60); c.m.w32(a_, (v_).0); };
                                }
                                if (((((((l_i_2) & (3i32))) == (0i32)) as i32)) != 0) {
                                    { let v_ = F32(0x00000000); let a_ = (l_o_4).wrapping_add(64); c.m.w32(a_, (v_).0); };
                                } else {
                                    { let v_ = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40400000)); let a_ = (l_o_4).wrapping_add(64); c.m.w32(a_, (v_).0); };
                                }
                                if ((((!((((l_i_2) & (1i32))) != 0)) as i32)) != 0) {
                                    { let v_ = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3ecccccc)), F32(0x3f000000)); let a_ = (l_o_4).wrapping_add(68); c.m.w32(a_, (v_).0); };
                                } else {
                                    { let v_ = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3e800000)), F32(0x3e800000)); let a_ = (l_o_4).wrapping_add(68); c.m.w32(a_, (v_).0); };
                                }
                                { let v_ = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3e4ccccc)), F32(0x3e999999)); let a_ = (l_o_4).wrapping_add(72); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40490fda)); let a_ = (l_o_4).wrapping_add(76); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fmul(rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3d4ccccc)), F32(0x3d4ccccc)), F32(0x40490fda)); let a_ = (l_o_4).wrapping_add(80); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fmul(rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3f000000)), F32(0x3f800000)), F32(0x3e4ccccc)); let a_ = (l_o_4).wrapping_add(84); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = (l_o_4).wrapping_add(88); c.m.w32(a_, (v_).0); };
                                { let r_ = 1i32; let a_ = (l_o_4).wrapping_add(52); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
                            }
                            { let r_ = F32(c.m.r32((l_o_4).wrapping_add(84))); let a_ = ((l_o_4).wrapping_add(112)).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        } else {
                            if (((rt::fle(F32(c.m.r32((l_o_4).wrapping_add(64))), F32(0x00000000)) as i32)) != 0) {
                                if ((((!((((((c.m.r8((l_o_4).wrapping_add(52))) as i32)) & (2i32))) != 0)) as i32)) != 0) {
                                    F::Vec3_Scale(c, (l_o_4).wrapping_add(112), (l_o_4).wrapping_add(112), F32(0x40400000));
                                    { let r_ = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3dcccccc)), F32(0x3f8ccccc)); let a_ = (l_o_4).wrapping_add(160); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                    { let r_ = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3dcccccc)), F32(0x3f8ccccc)); let a_ = ((l_o_4).wrapping_add(160)).wrapping_add(4); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                    { let v_ = F32(0x00000000); let a_ = ((l_o_4).wrapping_add(160)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                    { let v_ = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3ecccccc)), F32(0x3ecccccc)); let a_ = (l_o_4).wrapping_add(72); c.m.w32(a_, (v_).0); };
                                    { let v_ = rt::fmul(rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3dcccccc)), F32(0x3d4ccccc)), F32(0x40490fda)); let a_ = (l_o_4).wrapping_add(80); c.m.w32(a_, (v_).0); };
                                    { let r_ = 2i32; let a_ = (l_o_4).wrapping_add(52); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
                                }
                                F::Vec3_Add(c, (l_o_4).wrapping_add(96), (l_o_4).wrapping_add(96), ((l_w_3).wrapping_add(32)).wrapping_add(48));
                            } else {
                                { let r_ = F32(0x3f800000); let a_ = (l_o_4).wrapping_add(64); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            }
                        }
                        if (((((c.m.r32((l_w_3).wrapping_add(1412)) as i32)) & (2048i32))) != 0) {
                            { let a0_ = (((c.m.r8((l_o_4).wrapping_add(53)) as i8)) as i32); F::EftObj_SetVisible(c, a0_, 0i32) };
                        } else {
                            F::Vec4_Copy(c, fp.wrapping_add(160), (l_o_4).wrapping_add(112));
                            { let r_ = rt::fmul({ let a0_ = F32(c.m.r32((l_o_4).wrapping_add(76))); F::Port_sinf(c, a0_) }, F32(c.m.r32((l_o_4).wrapping_add(72)))); let a_ = (fp.wrapping_add(160)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            { let r_ = F32(c.m.r32((l_o_4).wrapping_add(80))); let a_ = (l_o_4).wrapping_add(76); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            { let v_ = { let a0_ = F32(c.m.r32((l_o_4).wrapping_add(76))); F::EftMath_WrapAngle(c, a0_) }; let a_ = (l_o_4).wrapping_add(76); c.m.w32(a_, (v_).0); };
                            F::Vec3_Add(c, (l_o_4).wrapping_add(112), (l_o_4).wrapping_add(112), (l_o_4).wrapping_add(128));
                            F::Vec3_Add(c, (l_o_4).wrapping_add(144), (l_o_4).wrapping_add(144), (l_o_4).wrapping_add(160));
                            { let v_ = { let a0_ = F32(c.m.r32((l_o_4).wrapping_add(144))); F::EftMath_WrapAngle(c, a0_) }; let a_ = (l_o_4).wrapping_add(144); c.m.w32(a_, (v_).0); };
                            { let v_ = { let a0_ = F32(c.m.r32(((l_o_4).wrapping_add(144)).wrapping_add(4))); F::EftMath_WrapAngle(c, a0_) }; let a_ = ((l_o_4).wrapping_add(144)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            { let v_ = { let a0_ = F32(c.m.r32(((l_o_4).wrapping_add(144)).wrapping_add(8))); F::EftMath_WrapAngle(c, a0_) }; let a_ = ((l_o_4).wrapping_add(144)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                            { let a2_ = F32(c.m.r32((l_o_4).wrapping_add(68))); F::Mtx_ScaleDiagUniform(c, fp.wrapping_add(64), fp, a2_) };
                            F::Mtx_RotateXYZ(c, fp.wrapping_add(64), fp.wrapping_add(64), (l_o_4).wrapping_add(144));
                            F::Vec3_Add(c, fp.wrapping_add(144), (l_o_4).wrapping_add(96), fp.wrapping_add(160));
                            { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(144)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                            F::Mtx_Translate(c, fp.wrapping_add(64), fp.wrapping_add(64), fp.wrapping_add(144));
                            { let a0_ = (((c.m.r8((l_o_4).wrapping_add(53)) as i8)) as i32); F::EftObj_SetMtx(c, a0_, fp.wrapping_add(64)) };
                            { let a0_ = (((c.m.r8((l_o_4).wrapping_add(53)) as i8)) as i32); F::EftObj_SetVisible(c, a0_, 1i32) };
                        }
                    } else {
                        { let a2_ = F32(c.m.r32((l_w_3).wrapping_add(1428))); F::Mtx_ScaleDiagUniform(c, fp, fp, a2_) };
                        F::Vec3_Clamp(c, fp.wrapping_add(128), l_w_3, F32(0xbf800000), F32(0x3f800000));
                        l_pitch_6 = { let a0_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(128)).wrapping_add(4)))); F::Mathf_Asin(c, a0_) };
                        l_yaw_7 = { let a0_ = F32(c.m.r32(fp.wrapping_add(128))); let a1_ = F32(c.m.r32((fp.wrapping_add(128)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) };
                        l_pitch_6 = F::EftMath_WrapAngle(c, l_pitch_6);
                        l_yaw_7 = F::EftMath_WrapAngle(c, l_yaw_7);
                        F::Mtx_RotateX(c, fp.wrapping_add(64), fp, l_pitch_6);
                        F::Mtx_RotateY(c, fp.wrapping_add(64), fp.wrapping_add(64), l_yaw_7);
                        F::Mtx_Translate(c, fp.wrapping_add(64), fp.wrapping_add(64), ((l_w_3).wrapping_add(32)).wrapping_add(16));
                        { let a0_ = (((c.m.r8((l_o_4).wrapping_add(53)) as i8)) as i32); F::EftObj_SetMtx(c, a0_, fp.wrapping_add(64)) };
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftRushShot_UpdateModels", &PROBE_N, &[], true); }
    r_
}

pub fn EftRushShot_FreeModels(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRushShot_FreeModels", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_sub_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < ((c.m.r32((l_w_2).wrapping_add(1488)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                l_sub_3 = ((l_w_2).wrapping_add(1504)).wrapping_add((((l_i_4) as u32)).wrapping_mul(176));
                { let a0_ = (((c.m.r8((l_sub_3).wrapping_add(53)) as i8)) as i32); { let _ = F::EftObj_Destroy(c, a0_); } };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRushShot_FreeModels", &PROBE_N, &[], true); }
    r_
}

pub fn EftRushShot_StartCam(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRushShot_StartCam", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_w_2: u32 = 0;
    let mut l_arg_3: u32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_arg_3 = c.m.r32((l_w_2).wrapping_add(96));
        if ((((c.m.r32((l_w_2).wrapping_add(1412))) & (((65536i32) as u32)))) != 0) {
            { let s_ = { rt::memset(c, fp.wrapping_add(12), 0, 12);; { let v_ = ((c.m.r32((l_w_2).wrapping_add(2736))) as i32); c.m.w32(fp.wrapping_add(12).wrapping_add(0), (v_) as u32); }; { let v_ = (c.m.r32(l_arg_3) as i32); c.m.w32(fp.wrapping_add(12).wrapping_add(4), (v_) as u32); }; { let v_ = (({ let a_ = c.m.r32((l_arg_3).wrapping_add(36)); (c.m.r16(a_) as i16) }) as i32); c.m.w32(fp.wrapping_add(12).wrapping_add(8), (v_) as u32); }; fp.wrapping_add(12) }; let d_ = fp; rt::memcpy(c, d_, s_, 12); };
            { let _ = D::call(c, fid::EftCam_Start, &[((fp) as u64)]); };
            { let r_ = ((524288i32) as u32); let a_ = (l_w_2).wrapping_add(1412); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftRushShot_StartCam", &PROBE_N, &[], true); }
    r_
}

pub fn EftRushShot_UpdateFade(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRushShot_UpdateFade", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_2: u32 = 0;
    let mut l_arg_3: u32 = 0;
    let mut l_opp_4: i32 = 0;
    let mut l_center_5: u32 = 0;
    let mut l_fade_6: F32 = F32(0);
    let mut l_a1_7: i32 = 0;
    let mut l_a2_8: i32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_arg_3 = c.m.r32((l_w_2).wrapping_add(96));
        l_opp_4 = { let a0_ = (c.m.r32(l_arg_3) as i32); F::BtlCharApi_GetOpponentObjId(c, a0_) };
        if (((({ let a_ = (c.m.r32((l_arg_3).wrapping_add(36))).wrapping_add(60); c.m.r32(a_) }) & (((512i32) as u32)))) != 0) {
            if ((F::EftShot_TestBits(c, l_opp_4, 16i32)) != 0) {
                { let r_ = ((1024i32) as u32); let a_ = (l_w_2).wrapping_add(1412); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                l_center_5 = (l_w_2).wrapping_add(16);
                { let a0_ = (c.m.r32(l_arg_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 17i32, fp) };
                { let a2_ = (c.m.r32(l_arg_3) as i32); { let _ = D::call(c, fid::EftAim_GetDir, &[((l_center_5) as u64), ((fp) as u64), ((a2_) as i64 as u64)]); } };
            } else {
                l_center_5 = (l_w_2).wrapping_add(16);
            }
            if ((F::EftShot_TestBits(c, l_opp_4, 32i32)) != 0) {
                { let v_ = F32(0x00000000); let a_ = (l_w_2).wrapping_add(1448); c.m.w32(a_, (v_).0); };
                { let r_ = (((!(1024i32))) as u32); let a_ = (l_w_2).wrapping_add(1412); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
            }
            if ((((c.m.r32((l_w_2).wrapping_add(1412))) & (((1024i32) as u32)))) != 0) {
                { let r_ = F32(0x3dcccccc); let a_ = (l_w_2).wrapping_add(1448); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::fgt(F32(c.m.r32((l_w_2).wrapping_add(1448))), F32(0x3f800000)) as i32)) != 0) {
                    { let v_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(1448); c.m.w32(a_, (v_).0); };
                }
            }
            l_fade_6 = F32(c.m.r32((l_w_2).wrapping_add(1448)));
            l_a1_7 = rt::f2i(rt::fmul(l_fade_6, F32(0x42800000)));
            l_a2_8 = rt::f2i(rt::fmul(l_fade_6, F32(0x43000000)));
            F::StgBlur_SetColor0Rgba(c, 0i32, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((((((rt::f2i(rt::fmul(l_fade_6, F32(0x42000000)))) as u8)) as i32)) as i64 as u64)) as u8));
            F::StgBlur_SetColor1Rgba(c, 0i32, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((((((l_a1_7) as u8)) as i32)) as i64 as u64)) as u8));
            F::StgBlur_SetColor2Rgba(c, 0i32, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((((((l_a2_8) as u8)) as i32)) as i64 as u64)) as u8));
            F::StgBlur_SetColor3Rgba(c, 0i32, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((64i32) as i64 as u64)) as u8));
            F::StgBlur_SetCenter(c, 0i32, l_center_5, 0i32);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftRushShot_UpdateFade", &PROBE_N, &[], true); }
    r_
}

pub fn EftRushShot_UpdateStageFx(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRushShot_UpdateStageFx", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_arg_3: u32 = 0;
    let mut l_opp_4: i32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_angle_6: F32 = F32(0);
    let mut l_kind_7: i32 = 0;
    let mut l_unk80_8: i32 = 0;
    let mut l_i_9: i32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_arg_3 = c.m.r32((l_w_2).wrapping_add(96));
    l_opp_4 = { let a0_ = (c.m.r32(l_arg_3) as i32); F::BtlCharApi_GetOpponentObjId(c, a0_) };
    if ((F::EftShot_TestBits(c, l_opp_4, 128i32)) != 0) {
        l_i_5 = (c.m.r32((l_w_2).wrapping_add(1484)) as i32);
        l_angle_6 = rt::fdiv(rt::fmul({ let a_ = ((c.m.r32((l_arg_3).wrapping_add(36))).wrapping_add(96)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, F32(0x40490fda)), F32(0x43340000));
        l_kind_7 = (({ let a_ = ((c.m.r32((l_arg_3).wrapping_add(36))).wrapping_add(88)).wrapping_add(((l_i_5) as u32)); (c.m.r8(a_) as i8) }) as i32);
        l_unk80_8 = (({ let a_ = (c.m.r32((l_arg_3).wrapping_add(36))).wrapping_add(128); (c.m.r16(a_) as i16) }) as i32);
        l_angle_6 = F::EftMath_WrapAngle(c, l_angle_6);
        if (((((l_kind_7) >= (0i32)) as i32)) != 0) {
            { let v_ = { let a0_ = (c.m.r32(l_arg_3) as i32); ((((F::EftStreak_Start(c, a0_, l_kind_7, l_unk80_8, l_angle_6)) as u64)) as i32) }; let a_ = ((l_w_2).wrapping_add(1452)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let r_ = ((2097152i32) as u32); let a_ = (l_w_2).wrapping_add(1412); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        }
        { let a_ = (l_w_2).wrapping_add(1484); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    }
    if ((F::EftShot_TestBits(c, l_opp_4, 256i32)) != 0) {
        l_i_9 = ((c.m.r32((l_w_2).wrapping_add(1484)) as i32)).wrapping_sub(1i32);
        if (((((l_i_9) < (0i32)) as i32)) != 0) {
            l_i_9 = 0i32;
        }
        if ((((c.m.r32((l_w_2).wrapping_add(1412))) & (((2097152i32) as u32)))) != 0) {
            { let a0_ = (c.m.r32(((l_w_2).wrapping_add(1452)).wrapping_add((((l_i_9) as u32)).wrapping_mul(4))) as i32); { let _ = D::call(c, fid::EftStreak_Stop, &[((a0_) as i64 as u64)]); } };
            { let v_ = 0i32; let a_ = ((l_w_2).wrapping_add(1452)).wrapping_add((((l_i_9) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let r_ = (((!(2097152i32))) as u32); let a_ = (l_w_2).wrapping_add(1412); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRushShot_UpdateStageFx", &PROBE_N, &[], true); }
    r_
}

pub fn EftRushShot_UpdateFlash(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRushShot_UpdateFlash", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_w_2: u32 = 0;
    let mut l_arg_3: u32 = 0;
    let mut l_opp_4: i32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_arg_3 = c.m.r32((l_w_2).wrapping_add(96));
        l_opp_4 = { let a0_ = (c.m.r32(l_arg_3) as i32); F::BtlCharApi_GetOpponentObjId(c, a0_) };
        if ((((((((!(((({ let a_ = (c.m.r32((l_arg_3).wrapping_add(36))).wrapping_add(60); c.m.r32(a_) }) & (((4194304i32) as u32)))) != 0)) as i32)) != 0) && ((F::EftShot_TestBits(c, l_opp_4, 64i32)) != 0)) as i32)) != 0) {
            { let s_ = { rt::memset(c, fp.wrapping_add(48), 0, 48);; rt::memset(c, fp.wrapping_add(48).wrapping_add(0), 0, 16);; { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(48).wrapping_add(0).wrapping_add(0), (v_).0); }; { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(48).wrapping_add(0).wrapping_add(4), (v_).0); }; { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(48).wrapping_add(0).wrapping_add(8), (v_).0); }; { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(48).wrapping_add(0).wrapping_add(12), (v_).0); }; { let v_ = F32(0x3f000000); c.m.w32(fp.wrapping_add(48).wrapping_add(16), (v_).0); }; { let v_ = 0i32; c.m.w32(fp.wrapping_add(48).wrapping_add(20), (v_) as u32); }; { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(48).wrapping_add(24), (v_).0); }; { let v_ = (c.m.r32(l_arg_3) as i32); c.m.w32(fp.wrapping_add(48).wrapping_add(28), (v_) as u32); }; { let v_ = 0i32; c.m.w32(fp.wrapping_add(48).wrapping_add(32), (v_) as u32); }; fp.wrapping_add(48) }; let d_ = fp; rt::memcpy(c, d_, s_, 48); };
            { let _ = F::EftFlash_Start(c, fp); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftRushShot_UpdateFlash", &PROBE_N, &[], true); }
    r_
}

pub fn EftRushShot_UpdateSparks(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRushShot_UpdateSparks", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_w_2: u32 = 0;
    let mut l_arg_3: u32 = 0;
    let mut l_opp_4: i32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_arg_3 = c.m.r32((l_w_2).wrapping_add(96));
        l_opp_4 = { let a0_ = (c.m.r32(l_arg_3) as i32); F::BtlCharApi_GetOpponentObjId(c, a0_) };
        if ((F::EftShot_TestBits(c, l_opp_4, 64i32)) != 0) {
            { let s_ = sym::gEftRushShotSparkTbl; rt::memcpy(c, fp, s_, 40); }
            { let a0_ = (c.m.r32(l_arg_3) as i32); F::EftDelaySe_Start(c, a0_, fp, 5i32) };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftRushShot_UpdateSparks", &PROBE_N, &[], true); }
    r_
}

pub fn EftRushShot_Draw(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32, mut p_model_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRushShot_Draw", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32, (p_model_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_4: u32 = 0;
    let mut l_endAlt_5: i32 = 0;
    let mut l_alt_6: i32 = 0;
    let mut l_end_7: i32 = 0;
    let mut l_fast_8: i32 = 0;
    let mut l_part_9: i32 = 0;
    let mut l_sub_10: i32 = 0;
    let mut l_def_11: u32 = 0;
    let mut l_idx_12: i32 = 0;
    let mut l_spawn_13: i32 = 0;
    let r_ = 'f: {
        l_w_4 = c.m.r32((p_task_2).wrapping_add(56));
        l_endAlt_5 = 0i32;
        l_alt_6 = 0i32;
        F::Vec4_Copy(c, fp, (l_w_4).wrapping_add(48));
        l_fast_8 = ((((c.m.r32((l_w_4).wrapping_add(1412))) & (((8i32) as u32)))) as i32);
        l_end_7 = ((((c.m.r32((l_w_4).wrapping_add(1412))) & (((2i32) as u32)))) as i32);
        if ((((((c.m.r32((l_w_4).wrapping_add(1416)) as i32)) == (1i32)) as i32)) != 0) {
            if ((((!((((c.m.r32((l_w_4).wrapping_add(1412))) & (((2048i32) as u32)))) != 0)) as i32)) != 0) {
                l_end_7 = ((((c.m.r32((l_w_4).wrapping_add(1412))) & (((1i32) as u32)))) as i32);
            } else {
                if ((((!((((c.m.r32((l_w_4).wrapping_add(1412))) & (((4096i32) as u32)))) != 0)) as i32)) != 0) {
                    l_end_7 = ((((c.m.r32((l_w_4).wrapping_add(1412))) & (((1i32) as u32)))) as i32);
                    { let r_ = ((4096i32) as u32); let a_ = (l_w_4).wrapping_add(1412); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                } else {
                    F::Vec4_Copy(c, fp, (l_w_4).wrapping_add(48));
                    l_end_7 = ((((c.m.r32((l_w_4).wrapping_add(1412))) & (((2i32) as u32)))) as i32);
                    l_alt_6 = 1i32;
                    l_endAlt_5 = l_end_7;
                }
            }
        }
        {
            l_part_9 = 0i32;
            'l1: loop {
                if !(((((l_part_9) < (19i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((({ let a_ = c.m.r32(p_model_3); c.m.r32(a_) }) & ((((1i32).wrapping_shl((l_part_9) as u32)) as u32)))) != 0) {
                        l_def_11 = c.m.r32(((p_model_3).wrapping_add(12)).wrapping_add((((l_part_9) as u32)).wrapping_mul(16)));
                        {
                            l_sub_10 = 0i32;
                            'l3: loop {
                                if !(((((l_sub_10) < (((c.m.r8((l_def_11).wrapping_add(1))) as i32))) as i32)) != 0) { break 'l3; }
                                'c4: {
                                    l_idx_12 = (((c.m.r8((((p_model_3).wrapping_add(12)).wrapping_add((((l_part_9) as u32)).wrapping_mul(16))).wrapping_add(8))) as i32)).wrapping_add(l_sub_10);
                                    if ((l_alt_6) != 0) {
                                        l_end_7 = l_endAlt_5;
                                        if ((((((((((({ let a_ = ((c.m.r32((p_model_3).wrapping_add(8))).wrapping_add((((l_idx_12) as u32)).wrapping_mul(64))).wrapping_add(9); c.m.r8(a_) }) as i32)) == (0i32)) as i32)) != 0) && (((rt::fgt({ let a_ = ((c.m.r32((p_model_3).wrapping_add(8))).wrapping_add((((l_idx_12) as u32)).wrapping_mul(64))).wrapping_add(24); F32(c.m.r32(a_)) }, F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                                            l_end_7 = 0i32;
                                            l_fast_8 = 0i32;
                                        }
                                    }
                                    l_spawn_13 = ((D::call(c, fid::EftEmit_GetFlagsFromReq, &[((p_model_3) as u64), (((l_w_4).wrapping_add(100)) as u64), ((p_objId_1) as i64 as u64), ((l_part_9) as i64 as u64), ((l_sub_10) as i64 as u64), ((l_end_7) as i64 as u64), ((l_fast_8) as i64 as u64)])) as i32);
                                    if (((((l_spawn_13) != (0i32)) as i32)) != 0) {
                                        { let a8_ = F32(c.m.r32((l_w_4).wrapping_add(1428))); { let _ = D::call(c, fid::EftEmit_SpawnOwn, &[((p_model_3) as u64), (((l_w_4).wrapping_add(100)) as u64), (((l_w_4).wrapping_add(816)) as u64), ((fp) as u64), ((l_w_4) as u64), ((l_part_9) as i64 as u64), ((l_sub_10) as i64 as u64), ((l_spawn_13) as i64 as u64), ((a8_).0 as u64)]); } };
                                    }
                                }
                                { let o_ = l_sub_10; let n_ = (o_).wrapping_add(1); l_sub_10 = n_; };
                            }
                        }
                    }
                }
                { let o_ = l_part_9; let n_ = (o_).wrapping_add(1); l_part_9 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftRushShot_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftRushShot_UpdateAttached(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRushShot_UpdateAttached", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_mgr_2: u32 = 0;
    let mut l_w_3: u32 = 0;
    let mut l_arg_4: u32 = 0;
    let r_ = 'f: {
    l_mgr_2 = { let a_ = (((D::call(c, fid::BtlTask_GetParent, &[((p_task_1) as u64)])) as u32)).wrapping_add(56); c.m.r32(a_) };
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    l_arg_4 = c.m.r32((l_w_3).wrapping_add(96));
    if ((((((c.m.r32((l_w_3).wrapping_add(1416)) as i32)) == (1i32)) as i32)) != 0) {
        if (((((c.m.r32(((l_mgr_2).wrapping_add(800)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
            if ((((c.m.r32((l_w_3).wrapping_add(1412))) & (((131072i32) as u32)))) != 0) {
                { let _ = D::call(c, fid::EftRushShot_UpdateModels, &[((p_task_1) as u64)]); };
            }
        }
        if ((((c.m.r32((l_w_3).wrapping_add(1412))) & (((2048i32) as u32)))) != 0) {
            { let a0_ = { let a0_ = (c.m.r32(l_arg_4) as i32); F::BtlCharApi_GetOpponentObjId(c, a0_) }; F::BtlCharApi_GetNodePos(c, a0_, 3i32, (l_w_3).wrapping_add(48)) };
            { let a2_ = (c.m.r32(l_arg_4) as i32); { let _ = D::call(c, fid::EftAim_GetDir, &[((l_w_3) as u64), (((l_w_3).wrapping_add(912)) as u64), ((a2_) as i64 as u64)]); } };
        }
    } else {
        if ((((c.m.r32((l_w_3).wrapping_add(1412))) & (((2048i32) as u32)))) != 0) {
            { let a0_ = (c.m.r32(l_arg_4) as i32); F::BtlCharApi_GetNodePos(c, a0_, 54i32, (l_w_3).wrapping_add(48)) };
            { let a2_ = (c.m.r32(l_arg_4) as i32); { let _ = D::call(c, fid::EftAim_GetDir, &[((l_w_3) as u64), (((l_w_3).wrapping_add(912)) as u64), ((a2_) as i64 as u64)]); } };
        }
    }
    if ((((c.m.r32((l_w_3).wrapping_add(1412))) & (((65536i32) as u32)))) != 0) {
        if (((({ let a_ = (c.m.r32((l_arg_4).wrapping_add(36))).wrapping_add(60); c.m.r32(a_) }) & (((512i32) as u32)))) != 0) {
            F::EftRushShot_UpdateFade(c, p_task_1);
        }
        F::EftRushShot_UpdateStageFx(c, p_task_1);
        F::EftRushShot_UpdateFlash(c, p_task_1);
        F::EftRushShot_UpdateSparks(c, p_task_1);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRushShot_UpdateAttached", &PROBE_N, &[], true); }
    r_
}

pub fn EftRushShot_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRushShot_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_mgr_3: u32 = 0;
    let mut l_w_4: u32 = 0;
    let mut l_def_5: u32 = 0;
    let mut l_v_6: u32 = 0;
    let r_ = 'f: {
    l_mgr_3 = { let a_ = (((D::call(c, fid::BtlTask_GetParent, &[((p_task_1) as u64)])) as u32)).wrapping_add(56); c.m.r32(a_) };
    l_w_4 = c.m.r32((p_task_1).wrapping_add(56));
    { let _ = F::memset(c, l_w_4, 0i32, 2752u32); };
    { let v_ = p_arg_2; let a_ = (l_w_4).wrapping_add(96); c.m.w32(a_, v_); };
    l_def_5 = c.m.r32((p_arg_2).wrapping_add(36));
    { let v_ = F32(c.m.r32((l_def_5).wrapping_add(52))); let a_ = (l_w_4).wrapping_add(1420); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_def_5).wrapping_add(48))); let a_ = (l_w_4).wrapping_add(1424); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_def_5).wrapping_add(48))); let a_ = (l_w_4).wrapping_add(1428); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_w_4).wrapping_add(1424))); let a_ = (l_w_4).wrapping_add(1432); c.m.w32(a_, (v_).0); };
    { let v_ = l_mgr_3; let a_ = (l_w_4).wrapping_add(1408); c.m.w32(a_, v_); };
    { let _ = D::call(c, fid::EftEmit_InitState, &[((l_mgr_3) as u64), (((l_w_4).wrapping_add(100)) as u64)]); };
    if (({ let a0_ = c.m.r32((l_w_4).wrapping_add(1408)); ((D::call(c, fid::EftEmit_HasWidth2, &[((a0_) as u64)])) as i32) }) != 0) {
        { let v_ = rt::fmul(F32((D::call(c, fid::EftEmit_GetWidth2, &[(((l_w_4).wrapping_add(100)) as u64)])) as u32), F32(c.m.r32((l_def_5).wrapping_add(48)))); let a_ = (l_w_4).wrapping_add(1420); c.m.w32(a_, (v_).0); };
        { let r_ = ((512i32) as u32); let a_ = (l_w_4).wrapping_add(1412); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    }
    { let v_ = rt::i2f({ let a0_ = c.m.r32((l_w_4).wrapping_add(1408)); ((D::call(c, fid::EftEmit_GetEndFrames, &[((a0_) as u64)])) as i32) }); let a_ = (l_w_4).wrapping_add(1444); c.m.w32(a_, (v_).0); };
    if ((((((((c.m.r16(l_def_5) as i16)) as i32)) == (413i32)) as i32)) != 0) {
        { let v_ = 1i32; let a_ = (l_w_4).wrapping_add(1416); c.m.w32(a_, (v_) as u32); };
    } else {
        { let v_ = 0i32; let a_ = (l_w_4).wrapping_add(1416); c.m.w32(a_, (v_) as u32); };
    }
    if (((((c.m.r32(((l_mgr_3).wrapping_add(800)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = 7i32; let a_ = (l_w_4).wrapping_add(1488); c.m.w32(a_, (v_) as u32); };
        { let _ = D::call(c, fid::EftRushShot_CreateModels, &[((p_task_1) as u64)]); };
    }
    if ((((((c.m.r32((p_arg_2).wrapping_add(4)) as i32)) >= (2i32)) as i32)) != 0) {
        l_v_6 = { let a_ = ((l_mgr_3).wrapping_add(800)).wrapping_add((((((c.m.r32((p_arg_2).wrapping_add(4)) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) };
        if (((((l_v_6) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = l_v_6; let a_ = (l_w_4).wrapping_add(2736); c.m.w32(a_, v_); };
        }
    }
    { let a1_ = (if ((((((c.m.r32(p_arg_2) as i32)) == (0i32)) as i32)) != 0) { 2048i32 } else { 4096i32 }); { let _ = D::call(c, fid::BtlTask_SetOwnerTag, &[((p_task_1) as u64), ((a1_) as i64 as u64)]); } };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRushShot_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftRushShot_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRushShot_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_mgr_2: u32 = 0;
    let mut l_w_3: u32 = 0;
    let mut l_arg_4: u32 = 0;
    let r_ = 'f: {
    l_mgr_2 = { let a_ = (((D::call(c, fid::BtlTask_GetParent, &[((p_task_1) as u64)])) as u32)).wrapping_add(56); c.m.r32(a_) };
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    l_arg_4 = c.m.r32((l_w_3).wrapping_add(96));
    if ((((c.m.r32((l_w_3).wrapping_add(1412))) & (((2048i32) as u32)))) != 0) {
        F::StgTint_Start(c, ((((0i32) as i64 as u64)) as u32), 1i32, F32(0x3e4ccccc));
    }
    if (((((c.m.r32(((l_mgr_2).wrapping_add(800)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
        F::EftRushShot_FreeModels(c, p_task_1);
    }
    { let a0_ = c.m.r32((l_w_3).wrapping_add(1408)); { let _ = D::call(c, fid::EftEmit_TermState, &[((a0_) as u64), (((l_w_3).wrapping_add(100)) as u64)]); } };
    if ((((((({ let a_ = (c.m.r32((l_arg_4).wrapping_add(36))).wrapping_add(4); (c.m.r8(a_) as i8) }) as i32)) != (0i32)) as i32)) != 0) {
        { let a0_ = (c.m.r32(l_arg_4) as i32); F::EftShot_SetHeldFlagA8(c, a0_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRushShot_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftRushShot_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRushShot_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_advance_2: i32 = 0;
    let mut l_mgr_3: u32 = 0;
    let mut l_w_4: u32 = 0;
    let mut l_arg_5: u32 = 0;
    let r_ = 'f: {
    l_advance_2 = 0i32;
    l_mgr_3 = { let a_ = (((D::call(c, fid::BtlTask_GetParent, &[((p_task_1) as u64)])) as u32)).wrapping_add(56); c.m.r32(a_) };
    l_w_4 = c.m.r32((p_task_1).wrapping_add(56));
    l_arg_5 = c.m.r32((l_w_4).wrapping_add(96));
    if (({ let a0_ = (c.m.r32(l_arg_5) as i32); F::BtlScene_IsCharStopped(c, a0_) }) != 0) {
        break 'f;
    }
    if (((((({ let a0_ = (c.m.r32(l_arg_5) as i32); F::BtlCharApi_IsRushConnected(c, a0_) }) != 0) && ((((!((((c.m.r32((l_w_4).wrapping_add(1412))) & (((65536i32) as u32)))) != 0)) as i32)) != 0)) as i32)) != 0) {
        { let a0_ = c.m.r32((l_w_4).wrapping_add(1408)); { let _ = D::call(c, fid::EftEmit_MarkKind6, &[((a0_) as u64), (((l_w_4).wrapping_add(100)) as u64)]); } };
        { let a0_ = (c.m.r32(l_arg_5) as i32); F::EftTechEvt_RequestStop(c, a0_) };
        { let v_ = ((((c.m.r32((l_w_4).wrapping_add(1412))) & ((((!(32i32))) as u32)))) | (((2048i32) as u32))); let a_ = (l_w_4).wrapping_add(1412); c.m.w32(a_, v_); };
        F::StgTint_Start(c, ((((0i32) as i64 as u64)) as u32), 0i32, F32(0x3e4ccccc));
        { let r_ = ((65536i32) as u32); let a_ = (l_w_4).wrapping_add(1412); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    }
    if ((((!((((c.m.r32((l_w_4).wrapping_add(1412))) & (((524288i32) as u32)))) != 0)) as i32)) != 0) {
        F::EftRushShot_StartCam(c, p_task_1);
    }
    { let _ = D::call(c, fid::EftEmit_UpdateNodesReq, &[((l_arg_5) as u64), (((l_w_4).wrapping_add(816)) as u64)]); };
    if ((((!((((c.m.r32((l_w_4).wrapping_add(1412))) & (((128i32) as u32)))) != 0)) as i32)) != 0) {
        if (({ let a0_ = (c.m.r32(l_arg_5) as i32); F::EftShot_TestBits(c, a0_, 2i32) }) != 0) {
            { let a2_ = (c.m.r32(l_arg_5) as i32); { let _ = D::call(c, fid::EftAim_GetDir, &[((l_w_4) as u64), (((l_w_4).wrapping_add(848)) as u64), ((a2_) as i64 as u64)]); } };
            { let v_ = ((0i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
            { let r_ = ((128i32) as u32); let a_ = (l_w_4).wrapping_add(1412); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
            if (((((c.m.r32(((l_mgr_3).wrapping_add(800)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
                { let r_ = ((131072i32) as u32); let a_ = (l_w_4).wrapping_add(1412); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
            }
        } else {
            if (({ let a0_ = (c.m.r32(l_arg_5) as i32); F::EftShot_TestBits(c, a0_, 4i32) }) != 0) {
                { let v_ = ((0i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                { let r_ = ((128i32) as u32); let a_ = (l_w_4).wrapping_add(1412); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
            }
        }
    }
    if ((((c.m.r32((l_w_4).wrapping_add(1412))) & (((128i32) as u32)))) != 0) {
        's1: {
            let sv6: i32 = ((c.m.r8((p_task_1).wrapping_add(1))) as i32);
            match sv6 {
                0i32 => {
                    if (({ let a0_ = (c.m.r32(l_arg_5) as i32); F::EftShot_TestBits(c, a0_, 4i32) }) != 0) {
                        { let r_ = ((32i32) as u32); let a_ = (l_w_4).wrapping_add(1412); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                        F::Vec4_Copy(c, (l_w_4).wrapping_add(32), (l_w_4).wrapping_add(912));
                        F::Vec4_Copy(c, (l_w_4).wrapping_add(48), (l_w_4).wrapping_add(32));
                        { let a0_ = (c.m.r32(l_arg_5) as i32); F::BtlCharApi_GetNodePos(c, a0_, 17i32, (l_w_4).wrapping_add(64)) };
                        { let a3_ = (c.m.r32(l_arg_5) as i32); { let _ = D::call(c, fid::EftAim_GetDirKeep, &[((l_arg_5) as u64), ((l_w_4) as u64), (((l_w_4).wrapping_add(48)) as u64), ((a3_) as i64 as u64)]); } };
                        { let a2_ = F32(c.m.r32((l_w_4).wrapping_add(1420))); F::Vec3_Scale(c, (l_w_4).wrapping_add(80), l_w_4, a2_) };
                        { let v_ = ((1i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                        { let r_ = ((262144i32) as u32); let a_ = (l_w_4).wrapping_add(1412); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                    }
                    break 's1;
                }
                2i32 => {
                    if ((((!((((c.m.r32((l_w_4).wrapping_add(1412))) & (((64i32) as u32)))) != 0)) as i32)) != 0) {
                        F::Vec4_Copy(c, (l_w_4).wrapping_add(64), (l_w_4).wrapping_add(48));
                    }
                    if ((((!(((({ let a_ = ({ let a_ = (c.m.r32((l_w_4).wrapping_add(96))).wrapping_add(36); c.m.r32(a_) }).wrapping_add(60); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
                        if ((((!((((c.m.r32((l_w_4).wrapping_add(1412))) & (((64i32) as u32)))) != 0)) as i32)) != 0) {
                            if (((rt::fgt({ let a_ = (c.m.r32((l_arg_5).wrapping_add(36))).wrapping_add(56); F32(c.m.r32(a_)) }, F32(0x00000000)) as i32)) != 0) {
                                { let a3_ = (c.m.r32(l_arg_5) as i32); let a4_ = F32(c.m.r32((l_w_4).wrapping_add(1420))); let a5_ = { let a_ = (c.m.r32((l_arg_5).wrapping_add(36))).wrapping_add(56); F32(c.m.r32(a_)) }; F::EftAim_Home(c, l_w_4, (l_w_4).wrapping_add(48), l_w_4, a3_, a4_, a5_) };
                                { let a2_ = F32(c.m.r32((l_w_4).wrapping_add(1420))); F::Vec3_Scale(c, (l_w_4).wrapping_add(80), l_w_4, a2_) };
                            }
                            F::Vec3_Add(c, (l_w_4).wrapping_add(48), (l_w_4).wrapping_add(48), (l_w_4).wrapping_add(80));
                        } else {
                            F::Vec3_Copy(c, (l_w_4).wrapping_add(48), (p_task_1).wrapping_add(16));
                        }
                    }
                    if (({ let a0_ = (c.m.r32(l_arg_5) as i32); F::EftShot_TestBits(c, a0_, 8i32) }) != 0) {
                        { let r_ = ((((((16i32) | (2i32))) | (1i32))) as u32); let a_ = (l_w_4).wrapping_add(1412); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                        { let v_ = ((3i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                    }
                    break 's1;
                }
                1i32 => {
                    l_advance_2 = 1i32;
                    break 's1;
                }
                3i32 => {
                    l_advance_2 = 1i32;
                    break 's1;
                }
                _ => {
                }
            }
        }
    }
    F::EftRushShot_UpdateAttached(c, p_task_1);
    if (({ let a0_ = (c.m.r32(l_arg_5) as i32); F::EftShot_TestBits(c, a0_, 1024i32) }) != 0) {
        if ((((!((((c.m.r32((l_w_4).wrapping_add(1412))) & (((16i32) as u32)))) != 0)) as i32)) != 0) {
            { let r_ = ((8i32) as u32); let a_ = (l_w_4).wrapping_add(1412); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        }
        { let r_ = ((2i32) as u32); let a_ = (l_w_4).wrapping_add(1412); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    }
    { let a0_ = (c.m.r32(l_arg_5) as i32); let a2_ = c.m.r32((l_w_4).wrapping_add(1408)); F::EftRushShot_Draw(c, a0_, p_task_1, a2_) };
    if ((((c.m.r32((l_w_4).wrapping_add(1412))) & (((32i32) as u32)))) != 0) {
        if ((((c.m.r32((l_w_4).wrapping_add(1412))) & (((512i32) as u32)))) != 0) {
            { let a0_ = c.m.r32((l_w_4).wrapping_add(1408)); { let _ = D::call(c, fid::EftEmit_UpdateWidth2, &[((a0_) as u64), (((l_w_4).wrapping_add(100)) as u64)]); } };
            { let v_ = rt::fmul(F32((D::call(c, fid::EftEmit_GetWidth2, &[(((l_w_4).wrapping_add(100)) as u64)])) as u32), { let a_ = (c.m.r32((l_arg_5).wrapping_add(36))).wrapping_add(48); F32(c.m.r32(a_)) }); let a_ = (l_w_4).wrapping_add(1420); c.m.w32(a_, (v_).0); };
            { let a2_ = F32(c.m.r32((l_w_4).wrapping_add(1420))); F::Vec3_Scale(c, (l_w_4).wrapping_add(80), l_w_4, a2_) };
        }
    }
    if ((l_advance_2) != 0) {
        { let a_ = (p_task_1).wrapping_add(1); let o_ = c.m.r8(a_); let n_ = (o_).wrapping_add(1); c.m.w8(a_, n_); };
    }
    if ((((c.m.r32((l_w_4).wrapping_add(1412))) & (((1i32) as u32)))) != 0) {
        { let r_ = F32(0x3f800000); let a_ = (l_w_4).wrapping_add(1440); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
    if ((((c.m.r32((l_w_4).wrapping_add(1412))) & (((4i32) as u32)))) != 0) {
        { let _ = D::call(c, fid::BtlTask_SetDead, &[((p_task_1) as u64)]); };
    } else {
        if ((((c.m.r32((l_w_4).wrapping_add(1412))) & (((2i32) as u32)))) != 0) {
            if ((((((((c.m.r32((l_w_4).wrapping_add(1412))) & (((8i32) as u32)))) != 0) || (((rt::fge(F32(c.m.r32((l_w_4).wrapping_add(1440))), F32(c.m.r32((l_w_4).wrapping_add(1444)))) as i32)) != 0)) as i32)) != 0) {
                { let r_ = ((4i32) as u32); let a_ = (l_w_4).wrapping_add(1412); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
            }
            if ((((c.m.r32((l_w_4).wrapping_add(1412))) & (((524288i32) as u32)))) != 0) {
                if ((((!((((c.m.r32((l_w_4).wrapping_add(1412))) & (((1048576i32) as u32)))) != 0)) as i32)) != 0) {
                    F::EftCam_Stop(c);
                    { let r_ = ((1048576i32) as u32); let a_ = (l_w_4).wrapping_add(1412); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                }
            }
        } else {
            if (((({ let a_ = (c.m.r32((l_arg_5).wrapping_add(36))).wrapping_add(60); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
                if ((((c.m.r32((l_w_4).wrapping_add(1412))) & (((32i32) as u32)))) != 0) {
                    { let a0_ = c.m.r32((l_w_4).wrapping_add(1408)); { let _ = D::call(c, fid::EftEmit_UpdateTrailWidth, &[((a0_) as u64), (((l_w_4).wrapping_add(100)) as u64)]); } };
                    { let _ = D::call(c, fid::EftRushShot_AddHitRecord, &[((p_task_1) as u64)]); };
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRushShot_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftRushShot_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRushShot_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        if (({ let a0_ = { let a_ = c.m.r32((l_w_2).wrapping_add(96)); (c.m.r32(a_) as i32) }; F::BtlScene_IsCharStopped(c, a0_) }) != 0) {
            break 'f;
        }
        { let _ = { let a0_ = c.m.r32((l_w_2).wrapping_add(1408)); ((D::call(c, fid::EftEmit_UpdateAlive, &[((a0_) as u64), (((l_w_2).wrapping_add(100)) as u64)])) as i32) }; };
        if ((((((c.m.r16((p_task_1).wrapping_add(8))) as i32)) & (1i32))) != 0) {
            F::Vec3_Sub(c, fp, (p_task_1).wrapping_add(16), (l_w_2).wrapping_add(48));
            F::Vec3_Copy(c, (l_w_2).wrapping_add(48), (p_task_1).wrapping_add(16));
            F::Vec3_Add(c, (l_w_2).wrapping_add(64), (l_w_2).wrapping_add(64), fp);
            { let r_ = ((64i32) as u32); let a_ = (l_w_2).wrapping_add(1412); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        }
        if ((((c.m.r32((p_task_1).wrapping_add(4))) & (((1i32) as u32)))) != 0) {
            if ((((c.m.r32((p_task_1).wrapping_add(4))) & (((2i32) as u32)))) != 0) {
                { let v_ = ((((c.m.r32((l_w_2).wrapping_add(1412))) | (((2i32) as u32)))) & ((((!(32i32))) as u32))); let a_ = (l_w_2).wrapping_add(1412); c.m.w32(a_, v_); };
            } else {
                { let r_ = (((!(32i32))) as u32); let a_ = (l_w_2).wrapping_add(1412); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
            }
        } else {
            if ((((c.m.r32((p_task_1).wrapping_add(4))) & (((2i32) as u32)))) != 0) {
                { let v_ = ((((c.m.r32((l_w_2).wrapping_add(1412))) | (((2i32) as u32)))) & ((((!(32i32))) as u32))); let a_ = (l_w_2).wrapping_add(1412); c.m.w32(a_, v_); };
            } else {
                if ((((c.m.r32((p_task_1).wrapping_add(4))) & (((4i32) as u32)))) != 0) {
                    { let v_ = ((((c.m.r32((l_w_2).wrapping_add(1412))) | (((2i32) as u32)))) & ((((!(32i32))) as u32))); let a_ = (l_w_2).wrapping_add(1412); c.m.w32(a_, v_); };
                }
            }
        }
        if ((((((c.m.r16((p_task_1).wrapping_add(8))) as i32)) & (4i32))) != 0) {
            { let v_ = ((((c.m.r32((l_w_2).wrapping_add(1412))) | (((1i32) as u32)))) & ((((!(32i32))) as u32))); let a_ = (l_w_2).wrapping_add(1412); c.m.w32(a_, v_); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftRushShot_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftRushShot_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRushShot_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!((((c.m.r32((l_w_2).wrapping_add(1412))) & (((256i32) as u32)))) != 0)) as i32)) != 0) {
        { let r_ = ((256i32) as u32); let a_ = (l_w_2).wrapping_add(1412); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        { let a0_ = c.m.r32((l_w_2).wrapping_add(1408)); { let _ = D::call(c, fid::EftEmit_KillAll, &[((a0_) as u64), (((l_w_2).wrapping_add(100)) as u64)]); } };
    }
    { let _ = D::call(c, fid::BtlTask_SetDead, &[((p_task_1) as u64)]); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRushShot_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftRushShot_Nop(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRushShot_Nop", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRushShot_Nop", &PROBE_N, &[], true); }
    r_
}

pub fn EftRushShotMgr_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRushShotMgr_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_mgr_3: u32 = 0;
    let mut l_next_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_pack_6: u32 = 0;
    let r_ = 'f: {
    l_mgr_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let _ = ((816u32) as i32); F::EftShot_Nop(c) };
    { let _ = F::memset(c, l_mgr_3, 0i32, 816u32); };
    { let a3_ = c.m.r32((p_arg_2).wrapping_add(28)); { let _ = D::call(c, fid::EftEmit_LoadSet, &[((p_arg_2) as u64), ((l_mgr_3) as u64), ((0i32) as i64 as u64), ((a3_) as u64), ((0i32) as i64 as u64), ((4i32) as i64 as u64)]); } };
    { let _ = D::call(c, fid::BtlTask_CreateChildList, &[((p_task_1) as u64), ((2i32) as i64 as u64), ((((2752u32) as i32)) as i64 as u64)]); };
    if (((({ let a_ = (c.m.r32((p_arg_2).wrapping_add(36))).wrapping_add(60); c.m.r32(a_) }) & (((131072i32) as u32)))) != 0) {
        { let v_ = { let a0_ = c.m.r32((p_arg_2).wrapping_add(28)); F::BtlScene_GetPackEntry(c, a0_, 1i32) }; let a_ = ((l_mgr_3).wrapping_add(800)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    l_next_4 = (l_mgr_3).wrapping_add(((4i32) as u32));
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((l_i_5) < (3i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_pack_6 = { let a0_ = (c.m.r32((p_arg_2).wrapping_add(32)) as i32); F::BtlScene_GetCharPackEntry(c, a0_, 8i32) };
                if ((((((c.m.r32((l_pack_6).wrapping_add(((((l_i_5).wrapping_add(2i32)) as u32)).wrapping_mul(4))) as i32)) != ((c.m.r32((l_pack_6).wrapping_add(((((l_i_5).wrapping_add(1i32)) as u32)).wrapping_mul(4))) as i32))) as i32)) != 0) {
                    { let v_ = F::BtlScene_GetPackEntry(c, l_pack_6, (l_i_5).wrapping_add(1i32)); let a_ = ((l_next_4).wrapping_add(800)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRushShotMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftRushShotMgr_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRushShotMgr_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32((p_task_1).wrapping_add(56)); F::EftEmit_FreeSet(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRushShotMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftRushShotMgr_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRushShotMgr_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32((p_task_1).wrapping_add(56)); F::EftEmit_BeginFrame(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRushShotMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftRushShotMgr_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRushShotMgr_Reset", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRushShotMgr_Reset", &PROBE_N, &[], true); }
    r_
}
