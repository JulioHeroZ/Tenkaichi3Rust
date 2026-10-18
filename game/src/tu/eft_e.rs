#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EftSteam_Create(c: &mut Ctx, mut p_arg_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSteam_Create", &PROBE_N, &[(p_arg_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftSteam)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    break 'f { let a0_ = { let a_ = c.m.r32(sym::gEftSteam); c.m.r32(a_) }; F::BtlTaskList_AddTail(c, a0_, sym::gEftSteamClass, p_arg_1) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSteam_Create", &PROBE_N, &[], true); }
    r_
}

pub fn EftSteam_Stop(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSteam_Stop", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    if (((((((((((((p_task_1) != (((0i32) as u32))) as i32)) != 0) && ((((({ let a_ = c.m.r32((p_task_1).wrapping_add(40)); c.m.r32(a_) }) == (fid::EftSteam_Update)) as i32)) != 0)) as i32)) != 0) && ((((!((((((((c.m.r8(p_task_1)) as i32)) & (1i32))) as u8)) != 0)) as i32)) != 0)) as i32)) != 0) {
        { let r_ = 2i32; let a_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(3184); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSteam_Stop", &PROBE_N, &[], true); }
    r_
}

pub fn EftSteam_SetPaused(c: &mut Ctx, mut p_task_1: u32, mut p_paused_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSteam_SetPaused", &PROBE_N, &[(p_task_1) as u32, (p_paused_2) as u32], false); }
    let mut l_work_3: u32 = 0;
    let r_ = 'f: {
    if (((((((((((((p_task_1) != (((0i32) as u32))) as i32)) != 0) && ((((({ let a_ = c.m.r32((p_task_1).wrapping_add(40)); c.m.r32(a_) }) == (fid::EftSteam_Update)) as i32)) != 0)) as i32)) != 0) && ((((!((((((((c.m.r8(p_task_1)) as i32)) & (1i32))) as u8)) != 0)) as i32)) != 0)) as i32)) != 0) {
        l_work_3 = c.m.r32((p_task_1).wrapping_add(56));
        if ((p_paused_2) != 0) {
            { let r_ = 1i32; let a_ = (l_work_3).wrapping_add(3184); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        } else {
            { let r_ = (!(1i32)); let a_ = (l_work_3).wrapping_add(3184); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSteam_SetPaused", &PROBE_N, &[], true); }
    r_
}

pub fn EftSteam_GetWork(c: &mut Ctx, mut p_task_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSteam_GetWork", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    if ((((!((((({ let a_ = c.m.r32((p_task_1).wrapping_add(40)); c.m.r32(a_) }) == (fid::EftSteam_Update)) as i32)) != 0)) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    if ((((((((c.m.r8(p_task_1)) as i32)) & (1i32))) as u8)) != 0) {
        break 'f ((0i32) as u32);
    }
    break 'f c.m.r32((p_task_1).wrapping_add(56));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSteam_GetWork", &PROBE_N, &[], true); }
    r_
}

pub fn EftSteam_SetPos(c: &mut Ctx, mut p_task_1: u32, mut p_pos_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSteam_SetPos", &PROBE_N, &[(p_task_1) as u32, (p_pos_2) as u32], false); }
    let r_ = 'f: {
    if (((((((((((((p_task_1) != (((0i32) as u32))) as i32)) != 0) && ((((({ let a_ = c.m.r32((p_task_1).wrapping_add(40)); c.m.r32(a_) }) == (fid::EftSteam_Update)) as i32)) != 0)) as i32)) != 0) && ((((!((((((((c.m.r8(p_task_1)) as i32)) & (1i32))) as u8)) != 0)) as i32)) != 0)) as i32)) != 0) {
        { let a0_ = c.m.r32((p_task_1).wrapping_add(56)); F::Vec4_Copy(c, a0_, p_pos_2) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSteam_SetPos", &PROBE_N, &[], true); }
    r_
}

pub fn EftSteamMgr_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSteamMgr_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_stage_2: u32 = 0;
    let mut l_count_3: i32 = 0;
    let mut l_base_4: u32 = 0;
    let mut l_marker_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let r_ = 'f: {
        l_stage_2 = ((F::BtlScene_GetStageData(c)) as u32);
        l_count_3 = ((((F::BtlStage_GetFxResC(c)) as u64)) as i32);
        { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((144u32) as i32)) }; let a_ = sym::gEftSteam; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gEftSteam); F::memset(c, a0_, 0i32, 144u32) }; };
        { let v_ = { let a1_ = (l_count_3).wrapping_add(((((F::BtlStage_GetFxResA(c)) as u64)) as i32)); F::BtlTask_CreateChildList(c, p_task_1, ((((a1_) as i64 as u64)) as u32), ((((((3200u32) as i32)) as i64 as u64)) as u32)) }; let a_ = c.m.r32(sym::gEftSteam); c.m.w32(a_, v_); };
        { let a0_ = (c.m.r32(sym::gEftSteam)).wrapping_add(8); let a1_ = F::BtlScene_GetPackEntry(c, l_stage_2, 18i32); F::EftTexSet_Load8(c, a0_, a1_) };
        l_base_4 = F::BtlStage_GetFxResC2(c);
        if (((((l_base_4) != (((0i32) as u32))) as i32)) != 0) {
            {
                l_i_6 = 0i32;
                'l1: loop {
                    if !(((((l_i_6) < (l_count_3)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        l_marker_5 = (l_base_4).wrapping_add((((l_i_6) as u32)).wrapping_mul(48));
                        { let _ = F::memset(c, fp, 0i32, 96u32); };
                        { let v_ = F32(c.m.r32((l_marker_5).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = fp; c.m.w32(a_, (v_).0); };
                        { let v_ = F32(c.m.r32((l_marker_5).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(c.m.r32((l_marker_5).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(16); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0xbf800000); let a_ = ((fp).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x00000000); let a_ = ((fp).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x3f800000); let a_ = ((fp).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::i2f((c.m.r8(((l_marker_5).wrapping_add(26)).wrapping_add(((0i32) as u32)))) as i32); let a_ = (fp).wrapping_add(32); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::i2f((c.m.r8(((l_marker_5).wrapping_add(26)).wrapping_add(((1i32) as u32)))) as i32); let a_ = ((fp).wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::i2f((c.m.r8(((l_marker_5).wrapping_add(26)).wrapping_add(((2i32) as u32)))) as i32); let a_ = ((fp).wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::i2f((c.m.r8(((l_marker_5).wrapping_add(26)).wrapping_add(((3i32) as u32)))) as i32); let a_ = ((fp).wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(c.m.r32((l_marker_5).wrapping_add(12))); let a_ = (fp).wrapping_add(48); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(c.m.r32((l_marker_5).wrapping_add(16))); let a_ = (fp).wrapping_add(52); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(c.m.r32((l_marker_5).wrapping_add(20))); let a_ = (fp).wrapping_add(56); c.m.w32(a_, (v_).0); };
                        { let v_ = ((c.m.r16((l_marker_5).wrapping_add(24))) as i32); let a_ = (fp).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
                        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(64); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x3e999999); let a_ = (fp).wrapping_add(68); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(72); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x3f000000); let a_ = (fp).wrapping_add(76); c.m.w32(a_, (v_).0); };
                        { let v_ = 5i32; let a_ = (fp).wrapping_add(80); c.m.w32(a_, (v_) as u32); };
                        { let _ = F::EftSteam_Create(c, fp); };
                    }
                    { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSteamMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftSteamMgr_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSteamMgr_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftSteam); F::BtlPool_Free(c, a0_, a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftSteam; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSteamMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftSteamMgr_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSteamMgr_Update", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = ({ let a_ = c.m.r32(sym::gEftSteam); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a0_ = ((c.m.r32(sym::gEftSteam)).wrapping_add(8)).wrapping_add(48); F::EftVram_AddTex(c, a0_, 1i32, 0i32) }; let a_ = ((c.m.r32(sym::gEftSteam)).wrapping_add(8)).wrapping_add(48); c.m.w64(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSteamMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftSteam_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSteam_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_work_3: u32 = 0;
    let r_ = 'f: {
    l_work_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let _ = F::memset(c, l_work_3, 0i32, 3200u32); };
    { let s_ = p_arg_2; let d_ = l_work_3; rt::memcpy(c, d_, s_, 96); };
    { let v_ = (c.m.r32(sym::gEftSteam)).wrapping_add(8); let a_ = (l_work_3).wrapping_add(96); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSteam_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftSteam_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSteam_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSteam_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftSteam_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSteam_Reset", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSteam_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftSteam_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSteam_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_work_2: u32 = 0;
    let r_ = 'f: {
    l_work_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_work_2).wrapping_add(3184)) as i32)) & (((1i32) | (2i32))))) != 0)) as i32)) != 0) {
        if (((((c.m.r32((l_work_2).wrapping_add(3184)) as i32)) & (4i32))) != 0) {
            F::EftSteam_Emit(c, l_work_2);
        }
    }
    if (((((((((c.m.r32((l_work_2).wrapping_add(3184)) as i32)) & (2i32))) != 0) && ((((((c.m.r32((l_work_2).wrapping_add(3188)) as i32)) <= (0i32)) as i32)) != 0)) as i32)) != 0) {
        F::BtlTask_SetDead(c, p_task_1);
        break 'f;
    }
    F::EftSteam_MoveParts(c, l_work_2);
    { let r_ = 4i32; let a_ = (l_work_2).wrapping_add(3184); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSteam_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftSteam_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSteam_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_work_2: u32 = 0;
    let r_ = 'f: {
    l_work_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(8); (c.m.r32(a_) as i32) }) & (1i32))) != 0)) as i32)) != 0) {
        if ((F::EftStage_IsDrawOn(c)) != 0) {
            F::Vu0Cur_Push(c);
            { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
            { let a1_ = { let a_ = (c.m.r32((l_work_2).wrapping_add(96))).wrapping_add(48); c.m.r64(a_) }; F::EftSteam_DrawParts(c, l_work_2, a1_) };
            F::Vu0Cur_Pop(c);
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSteam_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftSteam_FindFreePart(c: &mut Ctx, mut p_part_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSteam_FindFreePart", &PROBE_N, &[(p_part_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (32i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32((p_part_1).wrapping_add(80)) as i32)) == (0i32)) as i32)) != 0) {
                    break 'f p_part_1;
                }
            }
            { { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; }; { let o_ = p_part_1; let n_ = (o_).wrapping_add(96); p_part_1 = n_; }; };
        }
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSteam_FindFreePart", &PROBE_N, &[], true); }
    r_
}

pub fn EftSteam_Emit(c: &mut Ctx, mut p_work_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSteam_Emit", &PROBE_N, &[(p_work_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let mut l_part_2: u32 = 0;
    let r_ = 'f: {
        l_part_2 = F::EftSteam_FindFreePart(c, (p_work_1).wrapping_add(112));
        rt::memset(c, fp.wrapping_add(96), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(96).wrapping_add(0), (v_).0); }
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(96).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(96).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(96).wrapping_add(12), (v_).0); }
        if (((((l_part_2) != (((0i32) as u32))) as i32)) != 0) {
            { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(64); F::Mtx_InverseRT(c, fp, a1_) };
            F::Vec4_Sub(c, fp.wrapping_add(64), (fp).wrapping_add((((3i32) as u32)).wrapping_mul(16)), p_work_1);
            F::Vec3_Normalize(c, fp.wrapping_add(64), fp.wrapping_add(64));
            F::Vec3_Cross(c, fp.wrapping_add(80), fp.wrapping_add(96), fp.wrapping_add(64));
            F::Vec4_Copy(c, l_part_2, p_work_1);
            { let r_ = rt::fmul(F32(c.m.r32(fp.wrapping_add(80))), { let a0_ = rt::fneg(F32(c.m.r32((p_work_1).wrapping_add(64)))); let a1_ = F32(c.m.r32((p_work_1).wrapping_add(64))); F::Rand_FloatRange(c, a0_, a1_) }); let a_ = l_part_2; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(80)).wrapping_add(8))), { let a0_ = rt::fneg(F32(c.m.r32((p_work_1).wrapping_add(64)))); let a1_ = F32(c.m.r32((p_work_1).wrapping_add(64))); F::Rand_FloatRange(c, a0_, a1_) }); let a_ = (l_part_2).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let v_ = F32(0x3f800000); let a_ = (l_part_2).wrapping_add(12); c.m.w32(a_, (v_).0); };
            { let r_ = { let a0_ = rt::fneg(F32(c.m.r32((p_work_1).wrapping_add(68)))); let a1_ = F32(c.m.r32((p_work_1).wrapping_add(68))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = fp.wrapping_add(64); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = { let a0_ = rt::fneg(F32(c.m.r32((p_work_1).wrapping_add(68)))); let a1_ = F32(c.m.r32((p_work_1).wrapping_add(68))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (fp.wrapping_add(64)).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let a1_ = F32(c.m.r32(fp.wrapping_add(64))); let a3_ = F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(8))); F::Vec4_Set(c, (l_part_2).wrapping_add(16), a1_, F32(0xbf800000), a3_, F32(0x3f800000)) };
            F::Vec3_Normalize(c, (l_part_2).wrapping_add(16), (l_part_2).wrapping_add(16));
            F::Vec4_Copy(c, (l_part_2).wrapping_add(32), (p_work_1).wrapping_add(32));
            { let v_ = F32(c.m.r32((p_work_1).wrapping_add(52))); let a_ = (l_part_2).wrapping_add(68); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fadd(F32(c.m.r32((p_work_1).wrapping_add(48))), { let a0_ = rt::fneg(F32(c.m.r32((p_work_1).wrapping_add(72)))); let a1_ = F32(c.m.r32((p_work_1).wrapping_add(72))); F::Rand_FloatRange(c, a0_, a1_) }); let a_ = (l_part_2).wrapping_add(64); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fadd(F32(c.m.r32((p_work_1).wrapping_add(56))), { let a0_ = rt::fneg(F32(c.m.r32((p_work_1).wrapping_add(76)))); let a1_ = F32(c.m.r32((p_work_1).wrapping_add(76))); F::Rand_FloatRange(c, a0_, a1_) }); let a_ = (l_part_2).wrapping_add(72); c.m.w32(a_, (v_).0); };
            { let v_ = ((c.m.r32((p_work_1).wrapping_add(60)) as i32)).wrapping_add(rt::f2i({ let a0_ = rt::i2f(((c.m.r32((p_work_1).wrapping_add(80)) as i32)).wrapping_neg()); let a1_ = rt::i2f((c.m.r32((p_work_1).wrapping_add(80)) as i32)); F::Rand_FloatRange(c, a0_, a1_) })); let a_ = (l_part_2).wrapping_add(80); c.m.w32(a_, (v_) as u32); };
            { let v_ = F::Rand_FloatRange(c, F32(0x00000000), F32(0x40c90fda)); let a_ = (l_part_2).wrapping_add(76); c.m.w32(a_, (v_).0); };
            { let a2_ = F32(c.m.r32((l_part_2).wrapping_add(64))); F::Vec3_Scale(c, (l_part_2).wrapping_add(48), (l_part_2).wrapping_add(16), a2_) };
            { let a_ = (p_work_1).wrapping_add(3188); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSteam_Emit", &PROBE_N, &[], true); }
    r_
}

pub fn EftSteam_MoveParts(c: &mut Ctx, mut p_work_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSteam_MoveParts", &PROBE_N, &[(p_work_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let mut l_part_3: u32 = 0;
    let r_ = 'f: {
    l_part_3 = (p_work_1).wrapping_add(112);
    if ((((!((F::BtlScene_IsTimeStopped(c)) != 0)) as i32)) != 0) {
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (32i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if ((((((c.m.r32((l_part_3).wrapping_add(80)) as i32)) != (0i32)) as i32)) != 0) {
                        F::Vec3_Add(c, l_part_3, l_part_3, (l_part_3).wrapping_add(48));
                        { let r_ = F32(c.m.r32((l_part_3).wrapping_add(68))); let a_ = ((l_part_3).wrapping_add(48)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        { let a_ = (l_part_3).wrapping_add(80); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                        if ((((((c.m.r32((l_part_3).wrapping_add(80)) as i32)) < (0i32)) as i32)) != 0) {
                            { let v_ = 0i32; let a_ = (l_part_3).wrapping_add(80); c.m.w32(a_, (v_) as u32); };
                            { let a_ = (p_work_1).wrapping_add(3188); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                        }
                    }
                }
                { { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; }; { let o_ = l_part_3; let n_ = (o_).wrapping_add(96); l_part_3 = n_; }; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSteam_MoveParts", &PROBE_N, &[], true); }
    r_
}

pub fn EftSteam_DrawParts(c: &mut Ctx, mut p_work_1: u32, mut p_tex0_2: u64) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSteam_DrawParts", &PROBE_N, &[(p_work_1) as u32, (p_tex0_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let mut l_part_4: u32 = 0;
    let r_ = 'f: {
    l_part_4 = (p_work_1).wrapping_add(112);
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (32i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32((l_part_4).wrapping_add(80)) as i32)) != (0i32)) as i32)) != 0) {
                    { let a10_ = F32(c.m.r32((l_part_4).wrapping_add(72))); let a11_ = F32(c.m.r32((l_part_4).wrapping_add(76))); F::EftSteam_DrawQuad(c, (l_part_4).wrapping_add(32), l_part_4, F32(0x42000000), F32(0x42000000), F32(0x00000000), F32(0x00000000), F32(0x3f800000), F32(0x3f800000), 1i32, p_tex0_2, a10_, a11_) };
                }
            }
            { { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; }; { let o_ = l_part_4; let n_ = (o_).wrapping_add(96); l_part_4 = n_; }; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSteam_DrawParts", &PROBE_N, &[], true); }
    r_
}

pub fn eft_e__EftSteam_OtAdd(c: &mut Ctx, mut p_p_1: u32, mut p_z_2: i32, mut p_layer_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_e__EftSteam_OtAdd", &PROBE_N, &[(p_p_1) as u32, (p_z_2) as u32, (p_layer_3) as u32], false); }
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
    if rt::probing() { crate::host::harness::probe(c, "eft_e__EftSteam_OtAdd", &PROBE_N, &[], true); }
    r_
}

pub fn eft_e__EftSteam_IsClipped(c: &mut Ctx, mut p_x_1: i32, mut p_y_2: i32, mut p_z_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_e__EftSteam_IsClipped", &PROBE_N, &[(p_x_1) as u32, (p_y_2) as u32, (p_z_3) as u32], false); }
    let mut l_r_4: i32 = 0;
    let r_ = 'f: {
    l_r_4 = 1i32;
    if (((((((((((((p_z_3) > (0i32)) as i32)) != 0) && (((((p_x_1) <= (65519i32)) as i32)) != 0)) as i32)) != 0) && (((((p_x_1) > (0i32)) as i32)) != 0)) as i32)) != 0) {
        if (((((p_y_2) <= (65519i32)) as i32)) != 0) {
            l_r_4 = (((p_y_2) < (1i32)) as i32);
        }
    }
    break 'f l_r_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "eft_e__EftSteam_IsClipped", &PROBE_N, &[], true); }
    r_
}

pub fn EftSteam_DrawQuad(c: &mut Ctx, mut p_color_1: u32, mut p_pos_2: u32, mut p_w_3: F32, mut p_h_4: F32, mut p_u0_5: F32, mut p_v0_6: F32, mut p_u1_7: F32, mut p_v1_8: F32, mut p_prio_9: i32, mut p_tex0_10: u64, mut p_size_11: F32, mut p_angle_12: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSteam_DrawQuad", &PROBE_N, &[(p_color_1) as u32, (p_pos_2) as u32, (p_w_3).0, (p_h_4).0, (p_u0_5).0, (p_v0_6).0, (p_u1_7).0, (p_v1_8).0, (p_prio_9) as u32, (p_tex0_10) as u32, (p_size_11).0, (p_angle_12).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(208) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 208);
    let mut l_tint_13: F32 = F32(0);
    let mut l_hh_14: F32 = F32(0);
    let mut l_hw_15: F32 = F32(0);
    let mut l_i_16: i32 = 0;
    let mut l_p_17: u32 = 0;
    let mut l_prim_18: u64 = 0;
    let r_ = 'f: {
        l_tint_13 = F::EftStage_GetTintScale(c);
        if (((rt::fgt(p_angle_12, F32(0x40490fda)) as i32)) != 0) {
            { let r_ = F32(0x40c90fda); let n_ = rt::fsub(p_angle_12, r_); p_angle_12 = n_; };
        } else {
            if (((rt::flt(p_angle_12, F32(0xc0490fda)) as i32)) != 0) {
                { let r_ = F32(0x40c90fda); let n_ = rt::fadd(p_angle_12, r_); p_angle_12 = n_; };
            }
        }
        l_hw_15 = rt::fmul(p_w_3, p_size_11);
        l_hh_14 = rt::fmul(p_h_4, p_size_11);
        { let r_ = F32(0x3d800000); let n_ = rt::fmul(l_hh_14, r_); l_hh_14 = n_; };
        { let r_ = F32(0x3d800000); let n_ = rt::fmul(l_hw_15, r_); l_hw_15 = n_; };
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(544); F::Vec4_Sub(c, fp.wrapping_add(128), a1_, p_pos_2) };
        F::Vec3_Normalize(c, fp.wrapping_add(128), fp.wrapping_add(128));
        { let _ = F::EftMath_MtxFromDir(c, fp.wrapping_add(144), fp.wrapping_add(128), p_angle_12); };
        F::Vec4_Set(c, (fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), rt::fneg(l_hw_15), rt::fneg(l_hh_14), F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), l_hw_15, rt::fneg(l_hh_14), F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), rt::fneg(l_hw_15), l_hh_14, F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), l_hw_15, l_hh_14, F32(0x00000000), F32(0x3f800000));
        {
            l_i_16 = 0i32;
            'l1: loop {
                if !(((((l_i_16) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::Mtx_MulVec4(c, (fp.wrapping_add(64)).wrapping_add((((l_i_16) as u32)).wrapping_mul(16)), fp.wrapping_add(144), (fp.wrapping_add(64)).wrapping_add((((l_i_16) as u32)).wrapping_mul(16)));
                    F::Vec3_Add(c, (fp.wrapping_add(64)).wrapping_add((((l_i_16) as u32)).wrapping_mul(16)), (fp.wrapping_add(64)).wrapping_add((((l_i_16) as u32)).wrapping_mul(16)), p_pos_2);
                    { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(64)).wrapping_add((((l_i_16) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_16; let n_ = (o_).wrapping_add(1); l_i_16 = n_; };
            }
        }
        { let _ = F::Vu0Cur_ProjectPoints(c, fp, fp.wrapping_add(64), 4i32); };
        if (({ let a0_ = (c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as i32); let a1_ = (c.m.r32(((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32); let a2_ = (c.m.r32(((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32); F::eft_e__EftSteam_IsClipped(c, a0_, a1_, a2_) }) != 0) {
            break 'f;
        }
        if (({ let a0_ = (c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as i32); let a1_ = (c.m.r32(((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32); let a2_ = (c.m.r32(((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32); F::eft_e__EftSteam_IsClipped(c, a0_, a1_, a2_) }) != 0) {
            break 'f;
        }
        if (({ let a0_ = (c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as i32); let a1_ = (c.m.r32(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32); let a2_ = (c.m.r32(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32); F::eft_e__EftSteam_IsClipped(c, a0_, a1_, a2_) }) != 0) {
            break 'f;
        }
        if (({ let a0_ = (c.m.r32((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as i32); let a1_ = (c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32); let a2_ = (c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32); F::eft_e__EftSteam_IsClipped(c, a0_, a1_, a2_) }) != 0) {
            break 'f;
        }
        l_prim_18 = ((84i32) as u64);
        l_p_17 = c.m.r32(sym::gOtCur);
        { let v_ = (l_p_17).wrapping_add((((1i32) as u32)).wrapping_mul(128)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
        { let v_ = ((536870919i32) as u32); let a_ = l_p_17; c.m.w32(a_, v_); };
        { let v_ = ((1342177287i32) as u32); let a_ = (l_p_17).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = p_tex0_10; let a_ = (l_p_17).wrapping_add(40); c.m.w64(a_, v_); };
        { let v_ = l_prim_18; let a_ = (l_p_17).wrapping_add(32); c.m.w64(a_, v_); };
        { let v_ = ((268435456i32) as u32); let a_ = (l_p_17).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = 14123288431433908225u64; let a_ = (l_p_17).wrapping_add(16); c.m.w64(a_, v_); };
        { let v_ = ((268436062347616i64) as u64); let a_ = (l_p_17).wrapping_add(24); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (l_p_17).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = ((rt::f2u(rt::fmul(F32(c.m.r32(p_color_1)), l_tint_13))) as u8); let a_ = ((l_p_17).wrapping_add(48)).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2u(rt::fmul(F32(c.m.r32((p_color_1).wrapping_add(4))), l_tint_13))) as u8); let a_ = ((l_p_17).wrapping_add(48)).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2u(rt::fmul(F32(c.m.r32((p_color_1).wrapping_add(8))), l_tint_13))) as u8); let a_ = ((l_p_17).wrapping_add(48)).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2u(F32(c.m.r32((p_color_1).wrapping_add(12))))) as u8); let a_ = ((l_p_17).wrapping_add(48)).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = F32(0x3f800000); let a_ = (l_p_17).wrapping_add(52); c.m.w32(a_, (v_).0); };
        { let v_ = p_u0_5; let a_ = ((l_p_17).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = p_v0_6; let a_ = (((l_p_17).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_u1_7; let a_ = ((l_p_17).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = p_v0_6; let a_ = (((l_p_17).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_u0_5; let a_ = ((l_p_17).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = p_v1_8; let a_ = (((l_p_17).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_u1_7; let a_ = ((l_p_17).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = p_v1_8; let a_ = (((l_p_17).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = (((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_17).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_17).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_17).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_17).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_17).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_17).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_17).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_17).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_17).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_17).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_17).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_17).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_17).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_17).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_17).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_17).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let a1_ = ((c.m.r32(((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)).wrapping_shr((8i32) as u32); F::eft_e__EftSteam_OtAdd(c, l_p_17, a1_, p_prio_9) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSteam_DrawQuad", &PROBE_N, &[], true); }
    r_
}

pub fn EftWater_GetSurfaceY(c: &mut Ctx, mut p_y_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWater_GetSurfaceY", &PROBE_N, &[(p_y_1) as u32], false); }
    let r_ = 'f: {
    { let _ = F::BtlStage_GetWaterLevel(c, p_y_1); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWater_GetSurfaceY", &PROBE_N, &[], true); }
    r_
}

pub fn EftWater_SetWake(c: &mut Ctx, mut p_objId_1: i32, mut p_off_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWater_SetWake", &PROBE_N, &[(p_objId_1) as u32, (p_off_2) as u32], false); }
    let mut l_wake_3: u32 = 0;
    let mut sv_4: i32 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
    'fl1: loop {
        match bb_ {
            0 => {
                if (((((c.m.r32(sym::gEftDust)) == (((0i32) as u32))) as i32)) != 0) { bb_ = 1; continue 'fl1; } else { bb_ = 2; continue 'fl1; }
            }
            1 => {
                break 'f 0i32;
            }
            2 => {
                if ((F::Battle_IsSplitScreen(c)) != 0) { bb_ = 4; continue 'fl1; } else { bb_ = 5; continue 'fl1; }
            }
            3 => {
                bb_ = 2; continue 'fl1;
            }
            4 => {
                break 'f 0i32;
            }
            5 => {
                l_wake_3 = ({ let a_ = c.m.r32(sym::gEftDust); c.m.r32(a_) }).wrapping_add((((p_objId_1) as u32)).wrapping_mul(12));
                sv_4 = ((p_off_2) as i32);
                bb_ = match sv_4 { 0i32 => 9, 1i32 => 15, _ => 7, }; continue 'fl1;
            }
            6 => {
                bb_ = 5; continue 'fl1;
            }
            7 => {
                break 'f 1i32;
            }
            8 => {
                bb_ = 9; continue 'fl1;
            }
            9 => {
                if ((((((c.m.r32((l_wake_3).wrapping_add(8)) as i32)) != (0i32)) as i32)) != 0) { bb_ = 10; continue 'fl1; } else { bb_ = 11; continue 'fl1; }
            }
            10 => {
                bb_ = 7; continue 'fl1;
            }
            11 => {
                { let v_ = p_objId_1; let a_ = l_wake_3; c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = (l_wake_3).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                bb_ = 13; continue 'fl1;
            }
            12 => {
                bb_ = 11; continue 'fl1;
            }
            13 => {
                { let v_ = ((0i32) as u32); let a_ = (l_wake_3).wrapping_add(4); c.m.w32(a_, v_); };
                bb_ = 7; continue 'fl1;
            }
            14 => {
                bb_ = 15; continue 'fl1;
            }
            15 => {
                if ((((((c.m.r32((l_wake_3).wrapping_add(8)) as i32)) == (0i32)) as i32)) != 0) { bb_ = 16; continue 'fl1; } else { bb_ = 17; continue 'fl1; }
            }
            16 => {
                bb_ = 7; continue 'fl1;
            }
            17 => {
                { let v_ = 0i32; let a_ = (l_wake_3).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                bb_ = 13; continue 'fl1;
            }
            18 => {
                bb_ = 17; continue 'fl1;
            }
            19 => {
                bb_ = 7; continue 'fl1;
            }
            20 => {
                break 'fl1;
            }
            _ => unreachable!(),
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWater_SetWake", &PROBE_N, &[], true); }
    r_
}

pub fn EftWater_AddBlastTrail(c: &mut Ctx, mut p_objId_1: i32, mut p_rec_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWater_AddBlastTrail", &PROBE_N, &[(p_objId_1) as u32, (p_rec_2) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftDust)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((F::Battle_IsSplitScreen(c)) != 0) {
        break 'f 0i32;
    }
    { let a0_ = ((c.m.r32(sym::gEftDust)).wrapping_add(68)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let a1_ = ((c.m.r32(sym::gEftDust)).wrapping_add(68)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); F::EftWaterTrail_Spawn(c, a0_, a1_, p_objId_1, p_rec_2) };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWater_AddBlastTrail", &PROBE_N, &[], true); }
    r_
}

pub fn EftWater_AddSplashFor(c: &mut Ctx, mut p_objId_1: i32, p_pos_2: u32, mut p_kind_3: u8, mut p_speed_4: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWater_AddSplashFor", &PROBE_N, &[(p_objId_1) as u32, (p_pos_2) as u32, (p_kind_3) as u32, (p_speed_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_raw_5: F32 = F32(0);
    let mut l_scale_6: F32 = F32(0);
    rt::memcpy(c, fp, p_pos_2, 16);
    let r_ = 'f: {
        l_raw_5 = p_speed_4;
        if (((((c.m.r32(sym::gEftDust)) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((F::Battle_IsSplitScreen(c)) != 0) {
            break 'f 0i32;
        }
        F::Vec4_Copy(c, fp.wrapping_add(16), fp);
        if ((F::EftWater_GetSurfaceY(c, fp.wrapping_add(32))) != 0) {
            { let v_ = rt::fadd(F32(c.m.r32(fp.wrapping_add(32))), F32(0xbf800000)); let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        }
        p_speed_4 = F::Port_sqrtf(c, l_raw_5);
        l_scale_6 = rt::fmul(p_speed_4, F32(0x3ea8f5c2));
        's1: {
            let sv7: i32 = ((p_kind_3) as i32);
            match sv7 {
                0i32 => {
                    { let a0_ = ((c.m.r32(sym::gEftDust)).wrapping_add(60)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let a1_ = ((c.m.r32(sym::gEftDust)).wrapping_add(60)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let a7_ = rt::fmul(rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)), l_scale_6); F::EftWater_AddSplash(c, a0_, a1_, fp.wrapping_add(16), 7i32, 4i32, 3i32, 2i32, a7_) };
                    break 's1;
                }
                1i32 => {
                    { let a0_ = ((c.m.r32(sym::gEftDust)).wrapping_add(60)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let a1_ = ((c.m.r32(sym::gEftDust)).wrapping_add(60)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); F::EftWater_AddSplash(c, a0_, a1_, fp.wrapping_add(16), 4i32, 3i32, 2i32, 2i32, rt::fmul(l_scale_6, F32(0x3f4ccccc))) };
                    break 's1;
                }
                2i32 => {
                    { let a0_ = ((c.m.r32(sym::gEftDust)).wrapping_add(60)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let a1_ = ((c.m.r32(sym::gEftDust)).wrapping_add(60)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let a7_ = rt::fmul(rt::fmul(rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)), l_scale_6), F32(0x3f999999)); F::EftWater_AddSplash(c, a0_, a1_, fp.wrapping_add(16), 7i32, 4i32, 3i32, 2i32, a7_) };
                    break 's1;
                }
                _ => {
                }
            }
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftWater_AddSplashFor", &PROBE_N, &[], true); }
    r_
}

pub fn EftWater_AddSplashAt(c: &mut Ctx, p_pos_1: u32, mut p_scale_2: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWater_AddSplashAt", &PROBE_N, &[(p_pos_1) as u32, (p_scale_2).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    rt::memcpy(c, fp, p_pos_1, 16);
    let r_ = 'f: {
        if (((((c.m.r32(sym::gEftDust)) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((F::Battle_IsSplitScreen(c)) != 0) {
            break 'f 0i32;
        }
        F::Vec4_Copy(c, fp.wrapping_add(16), fp);
        if ((F::EftWater_GetSurfaceY(c, fp.wrapping_add(32))) != 0) {
            { let v_ = rt::fadd(F32(c.m.r32(fp.wrapping_add(32))), F32(0xbf800000)); let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        }
        { let a0_ = ((c.m.r32(sym::gEftDust)).wrapping_add(60)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let a1_ = ((c.m.r32(sym::gEftDust)).wrapping_add(60)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); F::EftWater_AddSplash(c, a0_, a1_, fp.wrapping_add(16), 7i32, 4i32, 3i32, 2i32, p_scale_2) };
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftWater_AddSplashAt", &PROBE_N, &[], true); }
    r_
}

pub fn EftWater_AddBurst(c: &mut Ctx, mut p_objId_1: i32, mut p_count_2: i32, mut p_delay_3: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWater_AddBurst", &PROBE_N, &[(p_objId_1) as u32, (p_count_2) as u32, (p_delay_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_height_4: F32 = F32(0);
    let mut l_yaw_5: F32 = F32(0);
    let mut l_step_6: F32 = F32(0);
    let mut l_radius_7: F32 = F32(0);
    let mut l_speed_8: F32 = F32(0);
    let mut l_gravity_9: F32 = F32(0);
    let mut l_size_10: F32 = F32(0);
    let mut l_sizeVel_11: F32 = F32(0);
    let mut l_life_12: F32 = F32(0);
    let mut l_pitch_13: F32 = F32(0);
    let mut l_yawVel_14: F32 = F32(0);
    let mut l_dist_15: F32 = F32(0);
    let mut l_i_16: i32 = 0;
    let r_ = 'f: {
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16), (v_).0); }
        if (((((c.m.r32(sym::gEftDust)) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((F::Battle_IsSplitScreen(c)) != 0) {
            break 'f 0i32;
        }
        { let _ = F::EftWater_GetSurfaceY(c, fp.wrapping_add(16)); };
        if (((rt::flt(F::BtlCharApi_GetGroundY(c, p_objId_1), F32(c.m.r32(fp.wrapping_add(16)))) as i32)) != 0) {
            break 'f 0i32;
        }
        F::BtlCharApi_GetPos(c, p_objId_1, fp);
        if ((F::EftWater_GetSurfaceY(c, fp.wrapping_add(20))) != 0) {
            { let v_ = rt::fadd(F32(c.m.r32(fp.wrapping_add(20))), F32(0xbf800000)); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        }
        l_height_4 = rt::fmul(F32(0x40e00000), rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)));
        l_step_6 = rt::fdiv(F32(0x43b40000), rt::i2f(p_count_2));
        l_yaw_5 = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x43b40000));
        {
            l_i_16 = 0i32;
            'l1: loop {
                if !(((((l_i_16) < (p_count_2)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_radius_7 = F32(0x41200000);
                    l_speed_8 = F32(0x41c80000);
                    l_gravity_9 = F32(0x40199999);
                    l_size_10 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3fc00000)), F32(0x3f800000));
                    l_sizeVel_11 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3f999999)), F32(0x40333333));
                    l_life_12 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3e800000)), F32(0x3ecccccc));
                    l_pitch_13 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x41200000)), F32(0x42480000));
                    l_yawVel_14 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40a00000)), F32(0x41600000));
                    { let r_ = rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)); let n_ = rt::fmul(l_radius_7, r_); l_radius_7 = n_; };
                    { let r_ = rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)); let n_ = rt::fmul(l_speed_8, r_); l_speed_8 = n_; };
                    { let r_ = rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)); let n_ = rt::fmul(l_sizeVel_11, r_); l_sizeVel_11 = n_; };
                    { let r_ = rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)); let n_ = rt::fmul(l_gravity_9, r_); l_gravity_9 = n_; };
                    { let a0_ = ((c.m.r32(sym::gEftDust)).wrapping_add(100)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let a1_ = ((c.m.r32(sym::gEftDust)).wrapping_add(100)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let a9_ = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x43b40000)); F::EftWaterMist_Spawn(c, a0_, a1_, fp, l_radius_7, l_height_4, rt::fadd(l_yaw_5, rt::fmul(l_step_6, rt::i2f(l_i_16))), l_yawVel_14, F32(0xc0e00000), l_pitch_13, a9_, F32(0x3f666666), F32(0x3f400000), l_speed_8, l_size_10, l_sizeVel_11, F32(0xc0a00000), l_gravity_9, F32(0x3f800000), F32(0x3fc00000), l_life_12, p_delay_3, ((140i32) as u8), ((140i32) as u8), ((140i32) as u8), ((3i32) as u8), ((0i32) as u8)) };
                }
                { let o_ = l_i_16; let n_ = (o_).wrapping_add(1); l_i_16 = n_; };
            }
        }
        l_step_6 = F32(0x43340000);
        l_dist_15 = F32(0x00000000);
        l_size_10 = rt::fmul(rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)), l_dist_15);
        { let a0_ = ((c.m.r32(sym::gEftDust)).wrapping_add(76)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let a1_ = ((c.m.r32(sym::gEftDust)).wrapping_add(76)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let a5_ = rt::fmul(rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)), F32(0x41000000)); F::EftWaterRing_Spawn(c, a0_, a1_, fp, l_dist_15, l_size_10, a5_, F32(0xc1c00000), F32(0x42e00000), F32(0x3f999999), p_delay_3, ((255i32) as u8), ((255i32) as u8), ((255i32) as u8), ((2i32) as u8), ((0i32) as u8), 13i32) };
        l_yaw_5 = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x43b40000));
        {
            l_i_16 = 0i32;
            'l3: loop {
                if !(((((l_i_16) < (2i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    l_dist_15 = F32(0x00000000);
                    l_gravity_9 = F32(0x40999999);
                    l_speed_8 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x41200000)), F32(0x420c0000));
                    l_size_10 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40000000)), F32(0x3f800000));
                    l_sizeVel_11 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40200000)), F32(0x40900000));
                    l_life_12 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3e999999)), F32(0x3f199999));
                    l_pitch_13 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x41200000)), F32(0x41f00000));
                    { let r_ = rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)); let n_ = rt::fmul(l_dist_15, r_); l_dist_15 = n_; };
                    { let r_ = rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)); let n_ = rt::fmul(l_speed_8, r_); l_speed_8 = n_; };
                    { let r_ = rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)); let n_ = rt::fmul(l_sizeVel_11, r_); l_sizeVel_11 = n_; };
                    { let r_ = rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)); let n_ = rt::fmul(l_gravity_9, r_); l_gravity_9 = n_; };
                    { let a0_ = ((c.m.r32(sym::gEftDust)).wrapping_add(84)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let a1_ = ((c.m.r32(sym::gEftDust)).wrapping_add(84)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let a6_ = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x43b40000)); F::EftWaterDrop_Spawn(c, a0_, a1_, fp, rt::fadd(l_yaw_5, rt::fmul(l_step_6, rt::i2f(l_i_16))), l_pitch_13, l_speed_8, a6_, F32(0x00000000), l_size_10, l_sizeVel_11, F32(0xc0a00000), l_gravity_9, l_life_12, p_delay_3, ((128i32) as u8), ((128i32) as u8), ((128i32) as u8), ((1i32) as u8), ((0i32) as u8), 9i32) };
                }
                { let o_ = l_i_16; let n_ = (o_).wrapping_add(1); l_i_16 = n_; };
            }
        }
        l_step_6 = F32(0x43340000);
        l_yaw_5 = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x43b40000));
        {
            l_i_16 = 0i32;
            'l5: loop {
                if !(((((l_i_16) < (2i32)) as i32)) != 0) { break 'l5; }
                'c6: {
                    l_dist_15 = F32(0x00000000);
                    l_gravity_9 = F32(0x40999999);
                    l_speed_8 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x41200000)), F32(0x41f00000));
                    l_size_10 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40000000)), F32(0x40000000));
                    l_sizeVel_11 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40400000)), F32(0x40a00000));
                    l_life_12 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3e999999)), F32(0x3f333333));
                    l_pitch_13 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x41200000)), F32(0x42200000));
                    { let r_ = rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)); let n_ = rt::fmul(l_dist_15, r_); l_dist_15 = n_; };
                    { let r_ = rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)); let n_ = rt::fmul(l_speed_8, r_); l_speed_8 = n_; };
                    { let r_ = rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)); let n_ = rt::fmul(l_sizeVel_11, r_); l_sizeVel_11 = n_; };
                    { let r_ = rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)); let n_ = rt::fmul(l_gravity_9, r_); l_gravity_9 = n_; };
                    { let a0_ = ((c.m.r32(sym::gEftDust)).wrapping_add(84)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let a1_ = ((c.m.r32(sym::gEftDust)).wrapping_add(84)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let a6_ = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x43b40000)); F::EftWaterDrop_Spawn(c, a0_, a1_, fp, rt::fadd(l_yaw_5, rt::fmul(l_step_6, rt::i2f(l_i_16))), l_pitch_13, l_speed_8, a6_, F32(0x00000000), l_size_10, l_sizeVel_11, F32(0xc0a00000), l_gravity_9, l_life_12, p_delay_3, ((140i32) as u8), ((140i32) as u8), ((140i32) as u8), ((4i32) as u8), ((0i32) as u8), 261i32) };
                }
                { let o_ = l_i_16; let n_ = (o_).wrapping_add(1); l_i_16 = n_; };
            }
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftWater_AddBurst", &PROBE_N, &[], true); }
    r_
}

pub fn EftWater_UpdateBlast(c: &mut Ctx, mut p_rec_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWater_UpdateBlast", &PROBE_N, &[(p_rec_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_dy_2: F32 = F32(0);
    let mut l_scale_3: F32 = F32(0);
    let mut l_size_4: F32 = F32(0);
    let mut l_wait_5: u32 = 0;
    let r_ = 'f: {
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64), (v_).0); }
        l_dy_2 = F32(0x3f800000);
        if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
            break 'f;
        }
        if (((((c.m.r32(sym::gEftDust)) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if (((((c.m.r32((p_rec_1).wrapping_add(80)) as i32)) & (8i32))) != 0) {
            break 'f;
        }
        if (((({ let a_ = (c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(4); (c.m.r32(a_) as i32) }) & (4i32))) != 0) {
            break 'f;
        }
        if (((({ let a_ = (c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(4); (c.m.r32(a_) as i32) }) & (1024i32))) != 0) {
            break 'f;
        }
        { let _ = F::EftWater_GetSurfaceY(c, fp.wrapping_add(64)); };
        if (((((F::BtlStage_GetId(c)) == (12i32)) as i32)) != 0) {
            { let r_ = F32(0x40a00000); let a_ = fp.wrapping_add(64); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
        F::Vec4_Copy(c, fp, (p_rec_1).wrapping_add(32));
        F::Vec4_Copy(c, fp.wrapping_add(16), (p_rec_1).wrapping_add(48));
        if (((({ let a_ = (c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(4); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
            F::Vec4_Sub(c, fp.wrapping_add(32), fp.wrapping_add(16), fp);
            F::Vec3_Normalize(c, fp.wrapping_add(32), fp.wrapping_add(32));
            { let r_ = F32(c.m.r32((p_rec_1).wrapping_add(120))); let a_ = fp.wrapping_add(32); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = F32(c.m.r32((p_rec_1).wrapping_add(120))); let a_ = (fp.wrapping_add(32)).wrapping_add(4); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = F32(c.m.r32((p_rec_1).wrapping_add(120))); let a_ = (fp.wrapping_add(32)).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let v_ = l_dy_2; let a_ = (fp.wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            { let a1_ = (c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(16); F::Vec4_Add(c, fp.wrapping_add(16), a1_, fp.wrapping_add(32)) };
            { let a1_ = (c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(16); F::Vec4_Copy(c, fp, a1_) };
            { let r_ = 1024i32; let a_ = (c.m.r32((p_rec_1).wrapping_add(96))).wrapping_add(4); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        l_dy_2 = { let a0_ = rt::fsub(F32(c.m.r32((fp).wrapping_add(4))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4)))); rt::ffabs(a0_) };
        if (((rt::fgt(l_dy_2, F32(0x41a00000)) as i32)) != 0) {
            l_dy_2 = F32(0x41a00000);
        }
        l_scale_3 = rt::fdiv(F::EftHit_GetRadiusA(c, p_rec_1), F32(0x40a00000));
        l_size_4 = rt::fmul(rt::fadd(rt::fmul(l_dy_2, F32(0x3f4ccccc)), rt::fsub(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40400000)), F32(0x3fc00000))), l_scale_3);
        if (((((((rt::fge(F32(c.m.r32((fp).wrapping_add(4))), F32(c.m.r32(fp.wrapping_add(64)))) as i32)) != 0) && (((rt::fle(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(c.m.r32(fp.wrapping_add(64)))) as i32)) != 0)) as i32)) != 0) {
            F::EftUtil_ClipSegToWater(c, fp.wrapping_add(48), fp, fp.wrapping_add(16));
            if ((((((c.m.r32((p_rec_1).wrapping_add(12)) as i32)) == (0i32)) as i32)) != 0) {
                { let _ = { let a0_ = (c.m.r32(p_rec_1) as i32); F::EftWater_AddSplashFor(c, a0_, fp.wrapping_add(48), ((1i32) as u8), l_size_4) }; };
                F::BtlCharApi_PlaySoundAt(c, fp.wrapping_add(48), 0i32, 60i32, F32(0x43480000), F32(0x44bb8000));
            } else {
                if (((((c.m.r32((p_rec_1).wrapping_add(100))) != (((0i32) as u32))) as i32)) != 0) {
                    if (((((((((({ let a_ = ({ let a_ = (c.m.r32((p_rec_1).wrapping_add(100))).wrapping_add(36); c.m.r32(a_) }).wrapping_add(((5i32) as u32)); c.m.r8(a_) }) as i32)).wrapping_sub(3i32)) as u32)) < (((2i32) as u32))) as i32)) != 0) {
                        if ((((((c.m.r32(p_rec_1) as i32)) == (0i32)) as i32)) != 0) {
                            l_wait_5 = ((c.m.r32(sym::gEftDust)).wrapping_add(108)).wrapping_add(((0i32) as u32));
                        } else {
                            l_wait_5 = ((c.m.r32(sym::gEftDust)).wrapping_add(108)).wrapping_add(((1i32) as u32));
                        }
                        if (((((((c.m.r8(l_wait_5)) as i32)) != (0i32)) as i32)) != 0) {
                            { let a_ = l_wait_5; let o_ = c.m.r8(a_); let n_ = (o_).wrapping_sub(1); c.m.w8(a_, n_); };
                        } else {
                            { let _ = { let a0_ = (c.m.r32(p_rec_1) as i32); F::EftWater_AddSplashFor(c, a0_, fp.wrapping_add(48), ((2i32) as u8), l_dy_2) }; };
                            { let v_ = ((F::EftRec_GetDefClass(c, p_rec_1)) as u8); let a_ = l_wait_5; c.m.w8(a_, v_); };
                        }
                    } else {
                        { let _ = { let a0_ = (c.m.r32(p_rec_1) as i32); F::EftWater_AddSplashFor(c, a0_, fp.wrapping_add(48), ((2i32) as u8), l_dy_2) }; };
                    }
                }
            }
        } else {
            if (((((((rt::fle(F32(c.m.r32((fp).wrapping_add(4))), F32(c.m.r32(fp.wrapping_add(64)))) as i32)) != 0) && (((rt::fle(F32(c.m.r32(fp.wrapping_add(64))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4)))) as i32)) != 0)) as i32)) != 0) {
                F::EftUtil_ClipSegToWater(c, fp.wrapping_add(48), fp, fp.wrapping_add(16));
                if ((((((c.m.r32((p_rec_1).wrapping_add(12)) as i32)) == (0i32)) as i32)) != 0) {
                    { let _ = { let a0_ = (c.m.r32(p_rec_1) as i32); F::EftWater_AddSplashFor(c, a0_, fp.wrapping_add(48), ((1i32) as u8), l_size_4) }; };
                    F::BtlCharApi_PlaySoundAt(c, fp.wrapping_add(48), 0i32, 61i32, F32(0x43480000), F32(0x44bb8000));
                } else {
                    if (((((c.m.r32((p_rec_1).wrapping_add(100))) != (((0i32) as u32))) as i32)) != 0) {
                        if (((((((((({ let a_ = ({ let a_ = (c.m.r32((p_rec_1).wrapping_add(100))).wrapping_add(36); c.m.r32(a_) }).wrapping_add(((5i32) as u32)); c.m.r8(a_) }) as i32)).wrapping_sub(3i32)) as u32)) < (((2i32) as u32))) as i32)) != 0) {
                            if ((((((c.m.r32(p_rec_1) as i32)) == (0i32)) as i32)) != 0) {
                                l_wait_5 = ((c.m.r32(sym::gEftDust)).wrapping_add(108)).wrapping_add(((0i32) as u32));
                            } else {
                                l_wait_5 = ((c.m.r32(sym::gEftDust)).wrapping_add(108)).wrapping_add(((1i32) as u32));
                            }
                            if (((((((c.m.r8(l_wait_5)) as i32)) != (0i32)) as i32)) != 0) {
                                { let a_ = l_wait_5; let o_ = c.m.r8(a_); let n_ = (o_).wrapping_sub(1); c.m.w8(a_, n_); };
                            } else {
                                { let _ = { let a0_ = (c.m.r32(p_rec_1) as i32); F::EftWater_AddSplashFor(c, a0_, fp.wrapping_add(48), ((2i32) as u8), l_dy_2) }; };
                                { let v_ = ((F::EftRec_GetDefClass(c, p_rec_1)) as u8); let a_ = l_wait_5; c.m.w8(a_, v_); };
                            }
                        } else {
                            { let _ = { let a0_ = (c.m.r32(p_rec_1) as i32); F::EftWater_AddSplashFor(c, a0_, fp.wrapping_add(48), ((2i32) as u8), l_dy_2) }; };
                        }
                    }
                }
            } else {
                if ((((((c.m.r32((p_rec_1).wrapping_add(12)) as i32)) != (0i32)) as i32)) != 0) {
                    if (((((F::BtlStage_GetId(c)) == (10i32)) as i32)) != 0) {
                        if (((((((rt::fgt(F32(c.m.r32((fp).wrapping_add(4))), F32(c.m.r32(fp.wrapping_add(64)))) as i32)) != 0) && (((rt::flt(F32(c.m.r32((fp).wrapping_add(4))), rt::fadd(F32(c.m.r32(fp.wrapping_add(64))), F32(0x41400000))) as i32)) != 0)) as i32)) != 0) {
                            { let _ = { let a0_ = (c.m.r32(p_rec_1) as i32); F::EftWater_AddBlastTrail(c, a0_, p_rec_1) }; };
                        }
                    } else {
                        if (((((((rt::flt(rt::fsub(F32(c.m.r32(fp.wrapping_add(64))), F32(0x40c00000)), F32(c.m.r32((fp).wrapping_add(4)))) as i32)) != 0) && (((rt::flt(F32(c.m.r32((fp).wrapping_add(4))), rt::fadd(F32(c.m.r32(fp.wrapping_add(64))), F32(0x41400000))) as i32)) != 0)) as i32)) != 0) {
                            { let _ = { let a0_ = (c.m.r32(p_rec_1) as i32); F::EftWater_AddBlastTrail(c, a0_, p_rec_1) }; };
                        }
                    }
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftWater_UpdateBlast", &PROBE_N, &[], true); }
    r_
}

pub fn EftWater_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWater_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_stage_2: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftDust)) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((704u32) as i32)) }; let a_ = sym::gEftDust; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gEftDust); F::memset(c, a0_, 0i32, 704u32) }; };
    }
    if ((((({ let a_ = c.m.r32(sym::gEftDust); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, 24i32) }; let a_ = c.m.r32(sym::gEftDust); c.m.w32(a_, v_); };
        { let _ = { let a0_ = { let a_ = c.m.r32(sym::gEftDust); c.m.r32(a_) }; F::memset(c, a0_, 0i32, ((24i32) as u32)) }; };
    }
    if ((((({ let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(4); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, 640i32) }; let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(4); c.m.w32(a_, v_); };
        { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(4); c.m.r32(a_) }; F::memset(c, a0_, 0i32, ((640i32) as u32)) }; };
    }
    if ((((({ let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(8); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, 540i32) }; let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(8); c.m.w32(a_, v_); };
        { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(8); c.m.r32(a_) }; F::memset(c, a0_, 0i32, ((540i32) as u32)) }; };
    }
    if ((((({ let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(12); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, 6720i32) }; let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(12); c.m.w32(a_, v_); };
        { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(12); c.m.r32(a_) }; F::memset(c, a0_, 0i32, ((6720i32) as u32)) }; };
    }
    if ((((({ let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(16); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, 2400i32) }; let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(16); c.m.w32(a_, v_); };
        { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(16); c.m.r32(a_) }; F::memset(c, a0_, 0i32, ((2400i32) as u32)) }; };
    }
    if ((((({ let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(20); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, 4320i32) }; let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(20); c.m.w32(a_, v_); };
        { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(20); c.m.r32(a_) }; F::memset(c, a0_, 0i32, ((4320i32) as u32)) }; };
    }
    if ((((({ let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(24); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, 5280i32) }; let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(24); c.m.w32(a_, v_); };
        { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(24); c.m.r32(a_) }; F::memset(c, a0_, 0i32, ((5280i32) as u32)) }; };
    }
    l_stage_2 = ((F::BtlScene_GetStageData(c)) as u32);
    if (((((F::BtlScene_GetPackEntrySize(c, l_stage_2, 18i32)) > (0i32)) as i32)) != 0) {
        { let a0_ = (c.m.r32(sym::gEftDust)).wrapping_add(176); let a1_ = F::BtlScene_GetPackEntry(c, l_stage_2, 18i32); F::EftTexSet_Load32(c, a0_, a1_) };
    } else {
        F::BtlTask_SetDead(c, p_task_1);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWater_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftWater_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWater_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(24); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = { let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(24); c.m.r32(a_) }; F::BtlPool_Free(c, a0_, a1_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(24); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(20); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = { let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(20); c.m.r32(a_) }; F::BtlPool_Free(c, a0_, a1_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(20); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(16); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = { let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(16); c.m.r32(a_) }; F::BtlPool_Free(c, a0_, a1_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(16); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(12); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = { let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(12); c.m.r32(a_) }; F::BtlPool_Free(c, a0_, a1_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(12); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = { let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(4); c.m.r32(a_) }; F::BtlPool_Free(c, a0_, a1_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(8); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = { let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(8); c.m.r32(a_) }; F::BtlPool_Free(c, a0_, a1_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(8); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = c.m.r32(sym::gEftDust); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = { let a_ = c.m.r32(sym::gEftDust); c.m.r32(a_) }; F::BtlPool_Free(c, a0_, a1_) };
        { let v_ = ((0i32) as u32); let a_ = c.m.r32(sym::gEftDust); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gEftDust)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftDust); F::BtlPool_Free(c, a0_, a1_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gEftDust; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWater_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftWater_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWater_Update", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((!((F::BtlCharApi_IsInTechnique(c, 0i32)) != 0)) as i32)) != 0) {
        { let v_ = ((0i32) as u8); let a_ = ((c.m.r32(sym::gEftDust)).wrapping_add(108)).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
    }
    if ((((!((F::BtlCharApi_IsInTechnique(c, 1i32)) != 0)) as i32)) != 0) {
        { let v_ = ((0i32) as u8); let a_ = ((c.m.r32(sym::gEftDust)).wrapping_add(108)).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
    }
    F::EftWater_UpdateWakes(c);
    { let a0_ = ((c.m.r32(sym::gEftDust)).wrapping_add(60)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let a1_ = ((c.m.r32(sym::gEftDust)).wrapping_add(60)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); F::EftWaterSplash_UpdateList(c, a0_, a1_) };
    { let a0_ = ((c.m.r32(sym::gEftDust)).wrapping_add(68)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let a1_ = ((c.m.r32(sym::gEftDust)).wrapping_add(68)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); F::EftWaterTrail_UpdateList(c, a0_, a1_) };
    { let a0_ = ((c.m.r32(sym::gEftDust)).wrapping_add(84)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let a1_ = ((c.m.r32(sym::gEftDust)).wrapping_add(84)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); F::EftWaterDrop_UpdateList(c, a0_, a1_) };
    { let a0_ = ((c.m.r32(sym::gEftDust)).wrapping_add(76)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let a1_ = ((c.m.r32(sym::gEftDust)).wrapping_add(76)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); F::EftWaterRing_UpdateList(c, a0_, a1_) };
    { let a0_ = ((c.m.r32(sym::gEftDust)).wrapping_add(92)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let a1_ = ((c.m.r32(sym::gEftDust)).wrapping_add(92)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); F::EftWaterSpray_UpdateList(c, a0_, a1_) };
    { let a0_ = ((c.m.r32(sym::gEftDust)).wrapping_add(100)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let a1_ = ((c.m.r32(sym::gEftDust)).wrapping_add(100)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); F::EftWaterMist_UpdateList(c, a0_, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWater_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftWater_PostUpdate(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWater_PostUpdate", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::EftWater_UpdateTextures(c, 1i32, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWater_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftWater_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWater_Reset", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let _ = F::EftWater_SetWake(c, l_i_1, ((1i32) as u8)); };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWater_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftWater_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWater_Draw", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((!(((({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(8); (c.m.r32(a_) as i32) }) & (1i32))) != 0)) as i32)) != 0) {
        if ((F::EftStage_IsDrawOn(c)) != 0) {
            if ((F::BtlStage_IsReady(c)) != 0) {
                F::Vu0Cur_Push(c);
                { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
                { let a0_ = (c.m.r32(sym::gEftDust)).wrapping_add(112); let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(64); F::Mtx_InverseRT(c, a0_, a1_) };
                { let v_ = { let v_ = { let v_ = F32(0x00000000); let a_ = (((c.m.r32(sym::gEftDust)).wrapping_add(112)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); v_ }; let a_ = (((c.m.r32(sym::gEftDust)).wrapping_add(112)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); v_ }; let a_ = (((c.m.r32(sym::gEftDust)).wrapping_add(112)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let a0_ = { let a_ = ((c.m.r32(sym::gEftDust)).wrapping_add(60)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::EftWaterSplash_DrawList(c, a0_) };
                { let a0_ = { let a_ = ((c.m.r32(sym::gEftDust)).wrapping_add(68)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::EftWaterTrail_DrawList(c, a0_) };
                { let a0_ = { let a_ = ((c.m.r32(sym::gEftDust)).wrapping_add(76)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::EftWaterRing_DrawList(c, a0_) };
                { let a0_ = { let a_ = ((c.m.r32(sym::gEftDust)).wrapping_add(92)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::EftWaterSpray_DrawList(c, a0_, sym::D_002C3630) };
                { let a0_ = { let a_ = ((c.m.r32(sym::gEftDust)).wrapping_add(84)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::EftWaterDrop_DrawList(c, a0_, sym::D_002C3630) };
                { let a0_ = { let a_ = ((c.m.r32(sym::gEftDust)).wrapping_add(100)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::EftWaterMist_DrawList(c, a0_) };
                F::Vu0Cur_Pop(c);
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWater_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftWater_UpdateWakes(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWater_UpdateWakes", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_yaw_1: F32 = F32(0);
    let mut l_height_2: F32 = F32(0);
    let mut l_scale_3: F32 = F32(0);
    let mut l_size_4: F32 = F32(0);
    let mut l_sizeVel_5: F32 = F32(0);
    let mut l_life_6: F32 = F32(0);
    let mut l_pitch_7: F32 = F32(0);
    let mut l_speed_8: F32 = F32(0);
    let mut l_gravity_9: F32 = F32(0);
    let mut l_puffYaw_10: F32 = F32(0);
    let mut l_dist_11: F32 = F32(0);
    let mut l_wake_12: u32 = 0;
    let mut l_n_13: i32 = 0;
    let mut l_i_14: i32 = 0;
    let r_ = 'f: {
        if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
            l_wake_12 = { let a_ = c.m.r32(sym::gEftDust); c.m.r32(a_) };
            {
                l_n_13 = 0i32;
                'l1: loop {
                    if !(((((l_n_13) < (2i32)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        if ((((((c.m.r32((l_wake_12).wrapping_add(8)) as i32)) != (0i32)) as i32)) != 0) {
                            { let a0_ = (c.m.r32(l_wake_12) as i32); F::BtlCharApi_GetPos(c, a0_, fp) };
                            if ((F::EftWater_GetSurfaceY(c, fp.wrapping_add(48))) != 0) {
                                { let v_ = rt::fadd(F32(c.m.r32(fp.wrapping_add(48))), F32(0xbf800000)); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            }
                            { let a0_ = (c.m.r32(l_wake_12) as i32); F::BtlCharApi_GetDir(c, a0_, fp.wrapping_add(16)) };
                            { let a2_ = { let a0_ = (c.m.r32(l_wake_12) as i32); F::BtlCharApi_GetSpeed(c, a0_) }; F::Vec3_Scale(c, fp.wrapping_add(32), fp.wrapping_add(16), a2_) };
                            { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            l_scale_3 = rt::fmul({ let a0_ = F::Vec3_Length(c, fp.wrapping_add(32)); F::Port_sqrtf(c, a0_) }, F32(0x3e851eb8));
                            if (((rt::flt(l_scale_3, F32(0x3f666666)) as i32)) != 0) {
                                { let r_ = F32(0x3f266666); let n_ = rt::fmul(l_scale_3, r_); l_scale_3 = n_; };
                            }
                            l_yaw_1 = rt::fdiv(rt::fmul({ let a0_ = F32(c.m.r32(fp.wrapping_add(16))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) }, F32(0x43340000)), F32(0x40490fda));
                            l_height_2 = rt::fmul(F32(0x40e00000), rt::fmul(rt::fmul({ let a0_ = (c.m.r32(l_wake_12) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x3d4ccccc)), l_scale_3));
                            if ((((!((((c.m.r32((l_wake_12).wrapping_add(4))) & (((1i32) as u32)))) != 0)) as i32)) != 0) {
                                {
                                    l_i_14 = 0i32;
                                    'l3: loop {
                                        if !(((((l_i_14) < (2i32)) as i32)) != 0) { break 'l3; }
                                        'c4: {
                                            l_dist_11 = F32(0x00000000);
                                            l_speed_8 = F32(0x42200000);
                                            l_gravity_9 = F32(0x40cccccc);
                                            l_size_4 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40400000)), F32(0x3f800000));
                                            l_sizeVel_5 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40400000)), F32(0x40a00000));
                                            l_life_6 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3e4ccccc)), F32(0x3e999999));
                                            l_pitch_7 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x41200000)), F32(0x42480000));
                                            { let r_ = rt::fmul(rt::fmul({ let a0_ = (c.m.r32(l_wake_12) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x3d4ccccc)), l_scale_3); let n_ = rt::fmul(l_speed_8, r_); l_speed_8 = n_; };
                                            { let r_ = rt::fmul(rt::fmul({ let a0_ = (c.m.r32(l_wake_12) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x3d4ccccc)), l_scale_3); let n_ = rt::fmul(l_sizeVel_5, r_); l_sizeVel_5 = n_; };
                                            { let r_ = rt::fmul(rt::fmul({ let a0_ = (c.m.r32(l_wake_12) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x3d4ccccc)), l_scale_3); let n_ = rt::fmul(l_gravity_9, r_); l_gravity_9 = n_; };
                                            { let a0_ = ((c.m.r32(sym::gEftDust)).wrapping_add(92)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let a1_ = ((c.m.r32(sym::gEftDust)).wrapping_add(92)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let a7_ = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x43b40000)); F::EftWaterSpray_Spawn(c, a0_, a1_, fp, l_dist_11, l_height_2, rt::fadd(rt::fsub(l_yaw_1, F32(0x42700000)), rt::fmul(rt::i2f(l_i_14), F32(0x42f00000))), l_pitch_7, a7_, F32(0x3f4ccccc), F32(0x3f199999), l_speed_8, l_size_4, l_sizeVel_5, F32(0xc0a00000), l_gravity_9, F32(0x3f800000), F32(0x3fc00000), l_life_6, F32(0x00000000), ((140i32) as u8), ((140i32) as u8), ((140i32) as u8), ((3i32) as u8), ((0i32) as u8), 258i32) };
                                        }
                                        { let o_ = l_i_14; let n_ = (o_).wrapping_add(1); l_i_14 = n_; };
                                    }
                                }
                            } else {
                                l_dist_11 = F32(0x00000000);
                                l_speed_8 = F32(0x42200000);
                                l_gravity_9 = F32(0x40cccccc);
                                l_size_4 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40400000)), F32(0x3f800000));
                                l_sizeVel_5 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40400000)), F32(0x40a00000));
                                l_life_6 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3e4ccccc)), F32(0x3e999999));
                                l_pitch_7 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x41200000)), F32(0x42480000));
                                { let r_ = rt::fmul(rt::fmul({ let a0_ = (c.m.r32(l_wake_12) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x3d4ccccc)), l_scale_3); let n_ = rt::fmul(l_speed_8, r_); l_speed_8 = n_; };
                                { let r_ = rt::fmul(rt::fmul({ let a0_ = (c.m.r32(l_wake_12) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x3d4ccccc)), l_scale_3); let n_ = rt::fmul(l_sizeVel_5, r_); l_sizeVel_5 = n_; };
                                { let r_ = rt::fmul(rt::fmul({ let a0_ = (c.m.r32(l_wake_12) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x3d4ccccc)), l_scale_3); let n_ = rt::fmul(l_gravity_9, r_); l_gravity_9 = n_; };
                                { let a0_ = ((c.m.r32(sym::gEftDust)).wrapping_add(92)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let a1_ = ((c.m.r32(sym::gEftDust)).wrapping_add(92)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let a5_ = rt::fadd(rt::fsub(l_yaw_1, F32(0x42700000)), rt::fmul(rt::i2f(rt::rem_i32(F::Port_Rand(c), 2i32)), F32(0x42f00000))); let a7_ = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x43b40000)); F::EftWaterSpray_Spawn(c, a0_, a1_, fp, l_dist_11, l_height_2, a5_, l_pitch_7, a7_, F32(0x3f4ccccc), F32(0x3f000000), l_speed_8, l_size_4, l_sizeVel_5, F32(0xc0a00000), l_gravity_9, F32(0x3f800000), F32(0x3fc00000), l_life_6, F32(0x00000000), ((140i32) as u8), ((140i32) as u8), ((140i32) as u8), ((3i32) as u8), ((0i32) as u8), 258i32) };
                            }
                            if ((((!((((c.m.r32((l_wake_12).wrapping_add(4))) & (((1i32) as u32)))) != 0)) as i32)) != 0) {
                                l_speed_8 = F32(0x42b40000);
                                l_size_4 = F32(0x40000000);
                                l_gravity_9 = F32(0x41100000);
                                l_sizeVel_5 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40600000)), F32(0x40d00000));
                                l_life_6 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3e4ccccc)), F32(0x3ecccccc));
                                l_pitch_7 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x41200000)), F32(0x42700000));
                                { let r_ = rt::fmul(rt::fmul({ let a0_ = (c.m.r32(l_wake_12) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x3d4ccccc)), l_scale_3); let n_ = rt::fmul(l_speed_8, r_); l_speed_8 = n_; };
                                { let r_ = rt::fmul(rt::fmul({ let a0_ = (c.m.r32(l_wake_12) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x3d4ccccc)), l_scale_3); let n_ = rt::fmul(l_size_4, r_); l_size_4 = n_; };
                                { let r_ = rt::fmul(rt::fmul({ let a0_ = (c.m.r32(l_wake_12) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x3d4ccccc)), l_scale_3); let n_ = rt::fmul(l_sizeVel_5, r_); l_sizeVel_5 = n_; };
                                { let r_ = rt::fmul(rt::fmul({ let a0_ = (c.m.r32(l_wake_12) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x3d4ccccc)), l_scale_3); let n_ = rt::fmul(l_gravity_9, r_); l_gravity_9 = n_; };
                                l_puffYaw_10 = rt::fadd(rt::fsub(l_yaw_1, F32(0x41f00000)), rt::fmul(rt::i2f(rt::rem_i32(F::Port_Rand(c), 2i32)), F32(0x42700000)));
                                { let a0_ = ((c.m.r32(sym::gEftDust)).wrapping_add(84)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let a1_ = ((c.m.r32(sym::gEftDust)).wrapping_add(84)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let a6_ = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x43b40000)); F::EftWaterDrop_Spawn(c, a0_, a1_, fp, l_puffYaw_10, l_pitch_7, l_speed_8, a6_, F32(0x00000000), l_size_4, l_sizeVel_5, F32(0xc0a00000), l_gravity_9, l_life_6, F32(0x00000000), ((128i32) as u8), ((128i32) as u8), ((128i32) as u8), ((4i32) as u8), ((0i32) as u8), 262i32) };
                                l_speed_8 = F32(0x425c0000);
                                l_size_4 = F32(0x40000000);
                                l_gravity_9 = F32(0x40600000);
                                l_sizeVel_5 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40600000)), F32(0x40d00000));
                                l_life_6 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3e999999)), F32(0x3f199999));
                                l_pitch_7 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x41200000)), F32(0x428c0000));
                                { let r_ = rt::fmul(rt::fmul({ let a0_ = (c.m.r32(l_wake_12) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x3d4ccccc)), l_scale_3); let n_ = rt::fmul(l_speed_8, r_); l_speed_8 = n_; };
                                { let r_ = rt::fmul(rt::fmul({ let a0_ = (c.m.r32(l_wake_12) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x3d4ccccc)), l_scale_3); let n_ = rt::fmul(l_size_4, r_); l_size_4 = n_; };
                                { let r_ = rt::fmul(rt::fmul({ let a0_ = (c.m.r32(l_wake_12) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x3d4ccccc)), l_scale_3); let n_ = rt::fmul(l_sizeVel_5, r_); l_sizeVel_5 = n_; };
                                { let r_ = rt::fmul(rt::fmul({ let a0_ = (c.m.r32(l_wake_12) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x3d4ccccc)), l_scale_3); let n_ = rt::fmul(l_gravity_9, r_); l_gravity_9 = n_; };
                                { let a0_ = ((c.m.r32(sym::gEftDust)).wrapping_add(84)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let a1_ = ((c.m.r32(sym::gEftDust)).wrapping_add(84)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let a6_ = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x43b40000)); F::EftWaterDrop_Spawn(c, a0_, a1_, fp, l_yaw_1, l_pitch_7, l_speed_8, a6_, F32(0x00000000), l_size_4, l_sizeVel_5, F32(0xc0a00000), l_gravity_9, l_life_6, F32(0x00000000), ((128i32) as u8), ((128i32) as u8), ((128i32) as u8), ((1i32) as u8), ((0i32) as u8), 10i32) };
                            }
                            if (((((rt::rem_u32(c.m.r32((l_wake_12).wrapping_add(4)), ((3i32) as u32))) == (((0i32) as u32))) as i32)) != 0) {
                                l_size_4 = rt::fmul(rt::fmul(rt::fmul({ let a0_ = (c.m.r32(l_wake_12) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x3d4ccccc)), l_scale_3), F32(0x00000000));
                                { let a0_ = ((c.m.r32(sym::gEftDust)).wrapping_add(76)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let a1_ = ((c.m.r32(sym::gEftDust)).wrapping_add(76)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let a5_ = rt::fmul(rt::fmul(rt::fmul({ let a0_ = (c.m.r32(l_wake_12) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x3d4ccccc)), l_scale_3), F32(0x41200000)); F::EftWaterRing_Spawn(c, a0_, a1_, fp, F32(0x00000000), l_size_4, a5_, F32(0xc1c00000), F32(0x42e00000), F32(0x3f199999), F32(0x3dcccccc), ((255i32) as u8), ((255i32) as u8), ((255i32) as u8), ((2i32) as u8), ((0i32) as u8), 13i32) };
                            }
                            { let a_ = (l_wake_12).wrapping_add(4); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(1); c.m.w32(a_, n_); };
                        }
                    }
                    { { let o_ = l_n_13; let n_ = (o_).wrapping_add(1); l_n_13 = n_; }; { let o_ = l_wake_12; let n_ = (o_).wrapping_add(12); l_wake_12 = n_; }; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftWater_UpdateWakes", &PROBE_N, &[], true); }
    r_
}

pub fn EftWater_AllocSplash(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWater_AllocSplash", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_splash_2: u32 = 0;
    let r_ = 'f: {
    l_splash_2 = { let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(4); c.m.r32(a_) };
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (10i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32((l_splash_2).wrapping_add(16)) as i32)) == (0i32)) as i32)) != 0) {
                    { let _ = F::memset(c, l_splash_2, 0i32, 64u32); };
                    break 'f l_splash_2;
                }
            }
            { { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; }; { let o_ = l_splash_2; let n_ = (o_).wrapping_add(64); l_splash_2 = n_; }; };
        }
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWater_AllocSplash", &PROBE_N, &[], true); }
    r_
}

pub fn EftWaterSplash_IsEmpty(c: &mut Ctx, mut p_splash_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWaterSplash_IsEmpty", &PROBE_N, &[(p_splash_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(((p_splash_1).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32(((p_splash_1).wrapping_add(28)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f (((c.m.r32(((p_splash_1).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))) == (((0i32) as u32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWaterSplash_IsEmpty", &PROBE_N, &[], true); }
    r_
}

pub fn EftWater_AddSplash(c: &mut Ctx, mut p_head_1: u32, mut p_tail_2: u32, p_pos_3: u32, mut p_sprayA_4: i32, mut p_sprayB_5: i32, mut p_dropA_6: i32, mut p_dropB_7: i32, mut p_scale_8: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWater_AddSplash", &PROBE_N, &[(p_head_1) as u32, (p_tail_2) as u32, (p_pos_3) as u32, (p_sprayA_4) as u32, (p_sprayB_5) as u32, (p_dropA_6) as u32, (p_dropB_7) as u32, (p_scale_8).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_splash_9: u32 = 0;
    let mut l_height_10: F32 = F32(0);
    let mut l_yaw_11: F32 = F32(0);
    let mut l_step_12: F32 = F32(0);
    let mut l_spread_13: F32 = F32(0);
    let mut l_dist_14: F32 = F32(0);
    let mut l_speed_15: F32 = F32(0);
    let mut l_gravity_16: F32 = F32(0);
    let mut l_size_17: F32 = F32(0);
    let mut l_sizeVel_18: F32 = F32(0);
    let mut l_life_19: F32 = F32(0);
    let mut l_pitch_20: F32 = F32(0);
    let mut l_i_21: i32 = 0;
    rt::memcpy(c, fp, p_pos_3, 16);
    let r_ = 'f: {
        l_splash_9 = F::EftWater_AllocSplash(c);
        if (((((l_splash_9) != (((0i32) as u32))) as i32)) != 0) {
            if (((((c.m.r32(p_head_1)) == (((0i32) as u32))) as i32)) != 0) {
                { let v_ = l_splash_9; let a_ = p_head_1; c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_splash_9).wrapping_add(44); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_splash_9).wrapping_add(48); c.m.w32(a_, v_); };
            } else {
                { let v_ = c.m.r32(p_tail_2); let a_ = (l_splash_9).wrapping_add(44); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_splash_9).wrapping_add(48); c.m.w32(a_, v_); };
            }
            if (((((c.m.r32(p_tail_2)) != (((0i32) as u32))) as i32)) != 0) {
                { let v_ = l_splash_9; let a_ = (c.m.r32(p_tail_2)).wrapping_add(48); c.m.w32(a_, v_); };
            }
            { let v_ = l_splash_9; let a_ = p_tail_2; c.m.w32(a_, v_); };
            F::Vec4_Copy(c, l_splash_9, fp);
            { let v_ = 2i32; let a_ = (l_splash_9).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
            { let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(28); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            l_height_10 = rt::fmul(F32(0x40e00000), p_scale_8);
            l_yaw_11 = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x43b40000));
            l_step_12 = rt::fdiv(F32(0x43b40000), rt::i2f(p_sprayA_4));
            l_spread_13 = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3e4ccccc));
            {
                l_i_21 = 0i32;
                'l1: loop {
                    if !(((((l_i_21) < (p_sprayA_4)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        l_dist_14 = F32(0x00000000);
                        l_speed_15 = F32(0x42200000);
                        l_gravity_16 = F32(0x40cccccc);
                        l_sizeVel_18 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40199999)), F32(0x40b33333));
                        l_life_19 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3ecccccc)), F32(0x3f000000));
                        l_pitch_20 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x41480000)), F32(0x427a0000));
                        { let r_ = p_scale_8; let n_ = rt::fmul(l_speed_15, r_); l_speed_15 = n_; };
                        { let r_ = p_scale_8; let n_ = rt::fmul(l_sizeVel_18, r_); l_sizeVel_18 = n_; };
                        { let r_ = p_scale_8; let n_ = rt::fmul(l_gravity_16, r_); l_gravity_16 = n_; };
                        F::EftWaterSpray_Spawn(c, ((l_splash_9).wrapping_add(28)).wrapping_add((((0i32) as u32)).wrapping_mul(4)), ((l_splash_9).wrapping_add(28)).wrapping_add((((1i32) as u32)).wrapping_mul(4)), sym::D_002C3630, l_dist_14, l_height_10, rt::fadd(l_yaw_11, rt::fmul(l_step_12, rt::i2f(l_i_21))), l_pitch_20, F32(0x00000000), F32(0x3f4ccccc), F32(0x3f266666), l_speed_15, F32(0x00000000), l_sizeVel_18, F32(0xc0a00000), l_gravity_16, rt::fadd(l_spread_13, F32(0x3f800000)), F32(0x3f999999), l_life_19, F32(0x00000000), ((140i32) as u8), ((140i32) as u8), ((140i32) as u8), ((3i32) as u8), ((0i32) as u8), 258i32);
                    }
                    { let o_ = l_i_21; let n_ = (o_).wrapping_add(1); l_i_21 = n_; };
                }
            }
            l_yaw_11 = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x43b40000));
            l_step_12 = rt::fdiv(F32(0x43b40000), rt::i2f(p_sprayB_5));
            {
                l_i_21 = 0i32;
                'l3: loop {
                    if !(((((l_i_21) < (p_sprayB_5)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        l_dist_14 = F32(0x40000000);
                        l_speed_15 = F32(0x42200000);
                        l_gravity_16 = F32(0x40cccccc);
                        l_sizeVel_18 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40800000)), F32(0x40c00000));
                        l_life_19 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3ecccccc)), F32(0x3f4ccccc));
                        l_pitch_20 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x41480000)), F32(0x42910000));
                        { let r_ = p_scale_8; let n_ = rt::fmul(l_speed_15, r_); l_speed_15 = n_; };
                        { let r_ = p_scale_8; let n_ = rt::fmul(l_sizeVel_18, r_); l_sizeVel_18 = n_; };
                        { let r_ = p_scale_8; let n_ = rt::fmul(l_gravity_16, r_); l_gravity_16 = n_; };
                        F::EftWaterSpray_Spawn(c, ((l_splash_9).wrapping_add(28)).wrapping_add((((0i32) as u32)).wrapping_mul(4)), ((l_splash_9).wrapping_add(28)).wrapping_add((((1i32) as u32)).wrapping_mul(4)), sym::D_002C3630, l_dist_14, l_height_10, rt::fadd(l_yaw_11, rt::fmul(l_step_12, rt::i2f(l_i_21))), l_pitch_20, F32(0x00000000), F32(0x3f666666), F32(0x3f400000), l_speed_15, F32(0x00000000), l_sizeVel_18, F32(0xc0a00000), l_gravity_16, rt::fadd(l_spread_13, F32(0x3f999999)), F32(0x3f333333), l_life_19, F32(0x3dcccccc), ((140i32) as u8), ((140i32) as u8), ((140i32) as u8), ((3i32) as u8), ((0i32) as u8), 257i32);
                    }
                    { let o_ = l_i_21; let n_ = (o_).wrapping_add(1); l_i_21 = n_; };
                }
            }
            l_yaw_11 = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x43b40000));
            l_step_12 = rt::fdiv(F32(0x43b40000), rt::i2f(p_dropA_6));
            {
                l_i_21 = 0i32;
                'l5: loop {
                    if !(((((l_i_21) < (p_dropA_6)) as i32)) != 0) { break 'l5; }
                    'c6: {
                        l_size_17 = F32(0x40000000);
                        l_gravity_16 = F32(0x41300000);
                        l_speed_15 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x42200000)), F32(0x42dc0000));
                        l_sizeVel_18 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40800000)), F32(0x41000000));
                        l_life_19 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3f000000)), F32(0x3f666666));
                        l_pitch_20 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x41200000)), F32(0x42900000));
                        { let r_ = p_scale_8; let n_ = rt::fmul(l_speed_15, r_); l_speed_15 = n_; };
                        { let r_ = p_scale_8; let n_ = rt::fmul(l_size_17, r_); l_size_17 = n_; };
                        { let r_ = p_scale_8; let n_ = rt::fmul(l_sizeVel_18, r_); l_sizeVel_18 = n_; };
                        { let r_ = p_scale_8; let n_ = rt::fmul(l_gravity_16, r_); l_gravity_16 = n_; };
                        { let a6_ = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x43b40000)); F::EftWaterDrop_Spawn(c, ((l_splash_9).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(4)), ((l_splash_9).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(4)), sym::D_002C3630, rt::fadd(l_yaw_11, rt::fmul(l_step_12, rt::i2f(l_i_21))), l_pitch_20, l_speed_15, a6_, F32(0x00000000), l_size_17, l_sizeVel_18, F32(0xc0a00000), l_gravity_16, l_life_19, F32(0x00000000), ((128i32) as u8), ((128i32) as u8), ((128i32) as u8), ((4i32) as u8), ((0i32) as u8), 262i32) };
                    }
                    { let o_ = l_i_21; let n_ = (o_).wrapping_add(1); l_i_21 = n_; };
                }
            }
            l_yaw_11 = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x43b40000));
            l_step_12 = rt::fdiv(F32(0x43b40000), rt::i2f(p_dropB_7));
            {
                l_i_21 = 0i32;
                'l7: loop {
                    if !(((((l_i_21) < (p_dropB_7)) as i32)) != 0) { break 'l7; }
                    'c8: {
                        l_speed_15 = F32(0x425c0000);
                        l_size_17 = F32(0x40000000);
                        l_gravity_16 = F32(0x40600000);
                        l_sizeVel_18 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40800000)), F32(0x41000000));
                        l_life_19 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3f199999)), F32(0x3fb33333));
                        l_pitch_20 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x41200000)), F32(0x42960000));
                        { let r_ = p_scale_8; let n_ = rt::fmul(l_speed_15, r_); l_speed_15 = n_; };
                        { let r_ = p_scale_8; let n_ = rt::fmul(l_size_17, r_); l_size_17 = n_; };
                        { let r_ = p_scale_8; let n_ = rt::fmul(l_sizeVel_18, r_); l_sizeVel_18 = n_; };
                        { let r_ = p_scale_8; let n_ = rt::fmul(l_gravity_16, r_); l_gravity_16 = n_; };
                        { let a6_ = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x43b40000)); F::EftWaterDrop_Spawn(c, ((l_splash_9).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(4)), ((l_splash_9).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(4)), sym::D_002C3630, rt::fadd(l_yaw_11, rt::fmul(l_step_12, rt::i2f(l_i_21))), l_pitch_20, l_speed_15, a6_, F32(0x00000000), l_size_17, l_sizeVel_18, F32(0xc0a00000), l_gravity_16, l_life_19, F32(0x00000000), ((128i32) as u8), ((128i32) as u8), ((128i32) as u8), ((1i32) as u8), ((0i32) as u8), 10i32) };
                    }
                    { let o_ = l_i_21; let n_ = (o_).wrapping_add(1); l_i_21 = n_; };
                }
            }
            F::EftWaterRing_Spawn(c, ((l_splash_9).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(4)), ((l_splash_9).wrapping_add(36)).wrapping_add((((1i32) as u32)).wrapping_mul(4)), fp, F32(0x00000000), rt::fmul(p_scale_8, F32(0x40a00000)), rt::fmul(p_scale_8, F32(0x41400000)), F32(0xc0800000), F32(0x42c00000), F32(0x3fb33333), F32(0x00000000), ((128i32) as u8), ((128i32) as u8), ((128i32) as u8), ((0i32) as u8), ((0i32) as u8), 14i32);
            F::EftWaterRing_Spawn(c, ((l_splash_9).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(4)), ((l_splash_9).wrapping_add(36)).wrapping_add((((1i32) as u32)).wrapping_mul(4)), fp, F32(0x00000000), rt::fmul(p_scale_8, F32(0x41000000)), rt::fmul(p_scale_8, F32(0x40c00000)), F32(0xc1c00000), F32(0x42e00000), F32(0x3fc00000), F32(0x00000000), ((255i32) as u8), ((255i32) as u8), ((255i32) as u8), ((2i32) as u8), ((0i32) as u8), 14i32);
            l_size_17 = rt::fmul(p_scale_8, F32(0x41200000));
            l_sizeVel_18 = rt::fmul(p_scale_8, F32(0x40800000));
            F::EftWaterRing_Spawn(c, ((l_splash_9).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(4)), ((l_splash_9).wrapping_add(36)).wrapping_add((((1i32) as u32)).wrapping_mul(4)), fp, F32(0x00000000), l_size_17, l_sizeVel_18, F32(0xc1c00000), F32(0x42c00000), F32(0x3f999999), F32(0x3f666666), ((255i32) as u8), ((255i32) as u8), ((255i32) as u8), ((2i32) as u8), ((0i32) as u8), 13i32);
            F::EftWaterRing_Spawn(c, ((l_splash_9).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(4)), ((l_splash_9).wrapping_add(36)).wrapping_add((((1i32) as u32)).wrapping_mul(4)), fp, F32(0x00000000), l_size_17, l_sizeVel_18, F32(0xc1c00000), F32(0x42b00000), F32(0x3f999999), F32(0x3f000000), ((255i32) as u8), ((255i32) as u8), ((255i32) as u8), ((2i32) as u8), ((0i32) as u8), 13i32);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftWater_AddSplash", &PROBE_N, &[], true); }
    r_
}

pub fn eft_e__EftWaterOt_Add(c: &mut Ctx, mut p_p_1: u32, mut p_z_2: i32, mut p_layer_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_e__EftWaterOt_Add", &PROBE_N, &[(p_p_1) as u32, (p_z_2) as u32, (p_layer_3) as u32], false); }
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
    if rt::probing() { crate::host::harness::probe(c, "eft_e__EftWaterOt_Add", &PROBE_N, &[], true); }
    r_
}

pub fn EftWaterSplash_UpdateList(c: &mut Ctx, mut p_head_1: u32, mut p_tail_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWaterSplash_UpdateList", &PROBE_N, &[(p_head_1) as u32, (p_tail_2) as u32], false); }
    let mut l_next_3: u32 = 0;
    let mut l_src_4: u32 = 0;
    let mut l_done_5: i32 = 0;
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(28); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
        if ((((!(((({ let a_ = (((D::call(c, fid::Battle_GetWork, &[])) as u32)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
            l_next_3 = c.m.r32(p_head_1);
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            'l1: while (((((l_next_3) != (((0i32) as u32))) as i32)) != 0) {
                l_src_4 = l_next_3;
                l_done_5 = F::EftWaterSplash_Update(c, l_next_3);
                l_next_3 = c.m.r32((l_next_3).wrapping_add(48));
                if ((l_done_5) != 0) {
                    { let v_ = 0i32; let a_ = (l_src_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                    if (((((c.m.r32((l_src_4).wrapping_add(44))) == (((0i32) as u32))) as i32)) != 0) {
                        { let v_ = c.m.r32((l_src_4).wrapping_add(48)); let a_ = p_head_1; c.m.w32(a_, v_); };
                    } else {
                        { let v_ = c.m.r32((l_src_4).wrapping_add(48)); let a_ = (c.m.r32((l_src_4).wrapping_add(44))).wrapping_add(48); c.m.w32(a_, v_); };
                    }
                    if (((((c.m.r32((l_src_4).wrapping_add(48))) == (((0i32) as u32))) as i32)) != 0) {
                        { let v_ = c.m.r32((l_src_4).wrapping_add(44)); let a_ = p_tail_2; c.m.w32(a_, v_); };
                    } else {
                        { let v_ = c.m.r32((l_src_4).wrapping_add(44)); let a_ = (c.m.r32((l_src_4).wrapping_add(48))).wrapping_add(44); c.m.w32(a_, v_); };
                    }
                    { let v_ = ((0i32) as u32); let a_ = (l_src_4).wrapping_add(44); c.m.w32(a_, v_); };
                    { let v_ = ((0i32) as u32); let a_ = (l_src_4).wrapping_add(48); c.m.w32(a_, v_); };
                    { let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(28); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWaterSplash_UpdateList", &PROBE_N, &[], true); }
    r_
}

pub fn EftWaterSplash_Update(c: &mut Ctx, mut p_src_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWaterSplash_Update", &PROBE_N, &[(p_src_1) as u32], false); }
    let r_ = 'f: {
    F::EftWaterDrop_UpdateList(c, (p_src_1).wrapping_add(20), (p_src_1).wrapping_add(24));
    F::EftWaterSpray_UpdateList(c, (p_src_1).wrapping_add(28), (p_src_1).wrapping_add(32));
    F::EftWaterRing_UpdateList(c, (p_src_1).wrapping_add(36), (p_src_1).wrapping_add(40));
    break 'f (((((D::call(c, fid::EftWaterSplash_IsEmpty, &[((p_src_1) as u64)])) as i32)) != (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWaterSplash_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftWaterSplash_DrawList(c: &mut Ctx, mut p_src_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWaterSplash_DrawList", &PROBE_N, &[(p_src_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let r_ = 'f: {
        {
            'l1: loop {
                if !(((((p_src_1) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let _ = D::call(c, fid::Vec4_Copy, &[((fp) as u64), ((p_src_1) as u64)]); };
                    if ((F::EftWater_GetSurfaceY(c, fp.wrapping_add(16))) != 0) {
                        { let v_ = rt::fadd(F32(c.m.r32(fp.wrapping_add(16))), F32(0xbf800000)); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    }
                    { let a0_ = c.m.r32((p_src_1).wrapping_add(20)); F::EftWaterDrop_DrawList(c, a0_, p_src_1) };
                    { let a0_ = c.m.r32((p_src_1).wrapping_add(28)); F::EftWaterSpray_DrawList(c, a0_, p_src_1) };
                    { let a0_ = c.m.r32((p_src_1).wrapping_add(36)); F::EftWaterRing_DrawList(c, a0_) };
                }
                p_src_1 = c.m.r32((p_src_1).wrapping_add(48));
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftWaterSplash_DrawList", &PROBE_N, &[], true); }
    r_
}

pub fn EftWaterSplash_FadeOut(c: &mut Ctx, mut p_src_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWaterSplash_FadeOut", &PROBE_N, &[(p_src_1) as u32], false); }
    let r_ = 'f: {
    if ((((!(((((c.m.r32((p_src_1).wrapping_add(16)) as i32)) & (64i32))) != 0)) as i32)) != 0) {
        { let a0_ = c.m.r32((p_src_1).wrapping_add(20)); F::EftWaterDrop_FadeOutList(c, a0_) };
        { let a0_ = c.m.r32((p_src_1).wrapping_add(36)); F::EftWaterRing_FadeOutList(c, a0_) };
        { let a0_ = c.m.r32((p_src_1).wrapping_add(28)); F::EftWaterSpray_FadeOutList(c, a0_) };
        { let r_ = 64i32; let a_ = (p_src_1).wrapping_add(16); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWaterSplash_FadeOut", &PROBE_N, &[], true); }
    r_
}

pub fn EftWaterTrail_Alloc(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWaterTrail_Alloc", &PROBE_N, &[], false); }
    let mut l_trail_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_trail_1 = { let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(8); c.m.r32(a_) };
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (15i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32(l_trail_1) as i32)) == (0i32)) as i32)) != 0) {
                    { let _ = F::memset(c, l_trail_1, 0i32, 36u32); };
                    break 'f l_trail_1;
                }
            }
            { { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; }; { let o_ = l_trail_1; let n_ = (o_).wrapping_add(36); l_trail_1 = n_; }; };
        }
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWaterTrail_Alloc", &PROBE_N, &[], true); }
    r_
}

pub fn EftWaterTrail_IsEmpty(c: &mut Ctx, mut p_trail_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWaterTrail_IsEmpty", &PROBE_N, &[(p_trail_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32((p_trail_1).wrapping_add(4))) != (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32((p_trail_1).wrapping_add(12))) != (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f (((c.m.r32((p_trail_1).wrapping_add(20))) == (((0i32) as u32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWaterTrail_IsEmpty", &PROBE_N, &[], true); }
    r_
}

pub fn EftWaterTrail_Spawn(c: &mut Ctx, mut p_head_1: u32, mut p_tail_2: u32, mut p_objId_3: i32, mut p_hit_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWaterTrail_Spawn", &PROBE_N, &[(p_head_1) as u32, (p_tail_2) as u32, (p_objId_3) as u32, (p_hit_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_trail_5: u32 = 0;
    let mut l_yaw_6: F32 = F32(0);
    let mut l_height_7: F32 = F32(0);
    let mut l_scale_8: F32 = F32(0);
    let mut l_size_9: F32 = F32(0);
    let mut l_sizeVel_10: F32 = F32(0);
    let mut l_life_11: F32 = F32(0);
    let mut l_pitch_12: F32 = F32(0);
    let mut l_speed_13: F32 = F32(0);
    let mut l_gravity_14: F32 = F32(0);
    let mut l_dropYaw_15: F32 = F32(0);
    let mut l_dist_16: F32 = F32(0);
    let mut l_i_17: i32 = 0;
    let r_ = 'f: {
        l_trail_5 = F::EftWaterTrail_Alloc(c);
        if (((((l_trail_5) != (((0i32) as u32))) as i32)) != 0) {
            if (((((c.m.r32(p_head_1)) == (((0i32) as u32))) as i32)) != 0) {
                { let v_ = l_trail_5; let a_ = p_head_1; c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_trail_5).wrapping_add(28); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_trail_5).wrapping_add(32); c.m.w32(a_, v_); };
            } else {
                { let v_ = c.m.r32(p_tail_2); let a_ = (l_trail_5).wrapping_add(28); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_trail_5).wrapping_add(32); c.m.w32(a_, v_); };
            }
            if (((((c.m.r32(p_tail_2)) != (((0i32) as u32))) as i32)) != 0) {
                { let v_ = l_trail_5; let a_ = (c.m.r32(p_tail_2)).wrapping_add(32); c.m.w32(a_, v_); };
            }
            { let v_ = l_trail_5; let a_ = p_tail_2; c.m.w32(a_, v_); };
            { let v_ = 2i32; let a_ = l_trail_5; c.m.w32(a_, (v_) as u32); };
            { let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(32); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            { let _ = D::call(c, fid::Vec4_Copy, &[((fp) as u64), (((p_hit_4).wrapping_add(32)) as u64)]); };
            if ((F::EftWater_GetSurfaceY(c, fp.wrapping_add(48))) != 0) {
                { let v_ = rt::fadd(F32(c.m.r32(fp.wrapping_add(48))), F32(0xbf800000)); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
            }
            { let _ = D::call(c, fid::Vec4_Sub, &[((fp.wrapping_add(32)) as u64), (((p_hit_4).wrapping_add(32)) as u64), (((p_hit_4).wrapping_add(48)) as u64)]); };
            { let _ = D::call(c, fid::Vec3_Normalize, &[((fp.wrapping_add(16)) as u64), ((fp.wrapping_add(32)) as u64)]); };
            { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            l_scale_8 = rt::fmul({ let a0_ = F32((D::call(c, fid::Vec3_Length, &[((fp.wrapping_add(32)) as u64)])) as u32); F::Port_sqrtf(c, a0_) }, F32(0x3e851eb8));
            if (((rt::flt(l_scale_8, F32(0x3f666666)) as i32)) != 0) {
                { let r_ = F32(0x3f266666); let n_ = rt::fmul(l_scale_8, r_); l_scale_8 = n_; };
            }
            l_yaw_6 = rt::fdiv(rt::fmul({ let a0_ = F32(c.m.r32(fp.wrapping_add(16))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) }, F32(0x43340000)), F32(0x40490fda));
            l_height_7 = rt::fmul(F32(0x40e00000), rt::fmul(rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_3), F32(0x3d4ccccc)), l_scale_8));
            {
                l_i_17 = 0i32;
                'l1: loop {
                    if !(((((l_i_17) < (2i32)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        l_dist_16 = F32(0x00000000);
                        l_speed_13 = F32(0x42200000);
                        l_gravity_14 = F32(0x40cccccc);
                        l_size_9 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40400000)), F32(0x3f800000));
                        l_sizeVel_10 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40400000)), F32(0x40a00000));
                        l_life_11 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3e4ccccc)), F32(0x3e999999));
                        l_pitch_12 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x41200000)), F32(0x42480000));
                        { let r_ = rt::fmul(rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_3), F32(0x3d4ccccc)), l_scale_8); let n_ = rt::fmul(l_speed_13, r_); l_speed_13 = n_; };
                        { let r_ = rt::fmul(rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_3), F32(0x3d4ccccc)), l_scale_8); let n_ = rt::fmul(l_sizeVel_10, r_); l_sizeVel_10 = n_; };
                        { let r_ = rt::fmul(rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_3), F32(0x3d4ccccc)), l_scale_8); let n_ = rt::fmul(l_gravity_14, r_); l_gravity_14 = n_; };
                        { let a7_ = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x43b40000)); F::EftWaterSpray_Spawn(c, (l_trail_5).wrapping_add(12), (l_trail_5).wrapping_add(16), fp, l_dist_16, l_height_7, rt::fadd(rt::fsub(l_yaw_6, F32(0x42700000)), rt::fmul(rt::i2f(l_i_17), F32(0x42f00000))), l_pitch_12, a7_, F32(0x3f4ccccc), F32(0x3f199999), l_speed_13, l_size_9, l_sizeVel_10, F32(0xc0a00000), l_gravity_14, F32(0x3f800000), F32(0x3fc00000), l_life_11, F32(0x00000000), ((140i32) as u8), ((140i32) as u8), ((140i32) as u8), ((3i32) as u8), ((0i32) as u8), ((256i32) | (2i32))) };
                    }
                    { let o_ = l_i_17; let n_ = (o_).wrapping_add(1); l_i_17 = n_; };
                }
            }
            l_speed_13 = F32(0x42b40000);
            l_size_9 = F32(0x40000000);
            l_gravity_14 = F32(0x41100000);
            l_sizeVel_10 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40600000)), F32(0x40d00000));
            l_life_11 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3e4ccccc)), F32(0x3ecccccc));
            l_pitch_12 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x41200000)), F32(0x42700000));
            { let r_ = rt::fmul(rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_3), F32(0x3d4ccccc)), l_scale_8); let n_ = rt::fmul(l_speed_13, r_); l_speed_13 = n_; };
            { let r_ = rt::fmul(rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_3), F32(0x3d4ccccc)), l_scale_8); let n_ = rt::fmul(l_size_9, r_); l_size_9 = n_; };
            { let r_ = rt::fmul(rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_3), F32(0x3d4ccccc)), l_scale_8); let n_ = rt::fmul(l_sizeVel_10, r_); l_sizeVel_10 = n_; };
            { let r_ = rt::fmul(rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_3), F32(0x3d4ccccc)), l_scale_8); let n_ = rt::fmul(l_gravity_14, r_); l_gravity_14 = n_; };
            l_dropYaw_15 = rt::fadd(rt::fsub(l_yaw_6, F32(0x41f00000)), rt::fmul(rt::i2f(rt::rem_i32(F::Port_Rand(c), 2i32)), F32(0x42700000)));
            { let a6_ = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x43b40000)); F::EftWaterDrop_Spawn(c, (l_trail_5).wrapping_add(4), (l_trail_5).wrapping_add(8), fp, l_dropYaw_15, l_pitch_12, l_speed_13, a6_, F32(0x00000000), l_size_9, l_sizeVel_10, F32(0xc0a00000), l_gravity_14, l_life_11, F32(0x00000000), ((128i32) as u8), ((128i32) as u8), ((128i32) as u8), ((4i32) as u8), ((0i32) as u8), ((((256i32) | (4i32))) | (2i32))) };
            l_speed_13 = F32(0x425c0000);
            l_size_9 = F32(0x40000000);
            l_gravity_14 = F32(0x40600000);
            l_sizeVel_10 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40600000)), F32(0x40d00000));
            l_life_11 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3e999999)), F32(0x3f199999));
            l_pitch_12 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x41200000)), F32(0x428c0000));
            { let r_ = rt::fmul(rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_3), F32(0x3d4ccccc)), l_scale_8); let n_ = rt::fmul(l_speed_13, r_); l_speed_13 = n_; };
            { let r_ = rt::fmul(rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_3), F32(0x3d4ccccc)), l_scale_8); let n_ = rt::fmul(l_size_9, r_); l_size_9 = n_; };
            { let r_ = rt::fmul(rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_3), F32(0x3d4ccccc)), l_scale_8); let n_ = rt::fmul(l_sizeVel_10, r_); l_sizeVel_10 = n_; };
            { let r_ = rt::fmul(rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_3), F32(0x3d4ccccc)), l_scale_8); let n_ = rt::fmul(l_gravity_14, r_); l_gravity_14 = n_; };
            { let a6_ = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x43b40000)); F::EftWaterDrop_Spawn(c, (l_trail_5).wrapping_add(4), (l_trail_5).wrapping_add(8), fp, l_yaw_6, l_pitch_12, l_speed_13, a6_, F32(0x00000000), l_size_9, l_sizeVel_10, F32(0xc0a00000), l_gravity_14, l_life_11, F32(0x00000000), ((128i32) as u8), ((128i32) as u8), ((128i32) as u8), ((1i32) as u8), ((0i32) as u8), ((8i32) | (2i32))) };
            l_size_9 = F32(0x00000000);
            l_sizeVel_10 = F32(0x41200000);
            { let r_ = rt::fmul(rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_3), F32(0x3d4ccccc)), l_scale_8); let n_ = rt::fmul(l_size_9, r_); l_size_9 = n_; };
            { let r_ = rt::fmul(rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_3), F32(0x3d4ccccc)), l_scale_8); let n_ = rt::fmul(l_sizeVel_10, r_); l_sizeVel_10 = n_; };
            F::EftWaterRing_Spawn(c, (l_trail_5).wrapping_add(20), (l_trail_5).wrapping_add(24), fp, F32(0x00000000), l_size_9, l_sizeVel_10, F32(0xc1c00000), F32(0x42e00000), F32(0x3f199999), F32(0x3dcccccc), ((255i32) as u8), ((255i32) as u8), ((255i32) as u8), ((2i32) as u8), ((0i32) as u8), ((((8i32) | (4i32))) | (1i32)));
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftWaterTrail_Spawn", &PROBE_N, &[], true); }
    r_
}

pub fn EftWaterTrail_UpdateList(c: &mut Ctx, mut p_head_1: u32, mut p_tail_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWaterTrail_UpdateList", &PROBE_N, &[(p_head_1) as u32, (p_tail_2) as u32], false); }
    let mut l_next_3: u32 = 0;
    let mut l_trail_4: u32 = 0;
    let mut l_done_5: i32 = 0;
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(32); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
        if ((((!(((({ let a_ = (((D::call(c, fid::Battle_GetWork, &[])) as u32)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
            l_next_3 = c.m.r32(p_head_1);
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            'l1: while (((((l_next_3) != (((0i32) as u32))) as i32)) != 0) {
                l_trail_4 = l_next_3;
                l_done_5 = F::EftWaterTrail_Update(c, l_next_3);
                l_next_3 = c.m.r32((l_next_3).wrapping_add(32));
                if ((l_done_5) != 0) {
                    { let v_ = 0i32; let a_ = l_trail_4; c.m.w32(a_, (v_) as u32); };
                    if (((((c.m.r32((l_trail_4).wrapping_add(28))) == (((0i32) as u32))) as i32)) != 0) {
                        { let v_ = c.m.r32((l_trail_4).wrapping_add(32)); let a_ = p_head_1; c.m.w32(a_, v_); };
                    } else {
                        { let v_ = c.m.r32((l_trail_4).wrapping_add(32)); let a_ = (c.m.r32((l_trail_4).wrapping_add(28))).wrapping_add(32); c.m.w32(a_, v_); };
                    }
                    if (((((c.m.r32((l_trail_4).wrapping_add(32))) == (((0i32) as u32))) as i32)) != 0) {
                        { let v_ = c.m.r32((l_trail_4).wrapping_add(28)); let a_ = p_tail_2; c.m.w32(a_, v_); };
                    } else {
                        { let v_ = c.m.r32((l_trail_4).wrapping_add(28)); let a_ = (c.m.r32((l_trail_4).wrapping_add(32))).wrapping_add(28); c.m.w32(a_, v_); };
                    }
                    { let v_ = ((0i32) as u32); let a_ = (l_trail_4).wrapping_add(28); c.m.w32(a_, v_); };
                    { let v_ = ((0i32) as u32); let a_ = (l_trail_4).wrapping_add(32); c.m.w32(a_, v_); };
                    { let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(32); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                }
                if (((((((({ let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(32); (c.m.r32(a_) as i32) }).wrapping_sub({ let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(40); (c.m.r32(a_) as i32) })) as u32)) >= (((6i32) as u32))) as i32)) != 0) {
                    F::EftWaterTrail_FadeOut(c, l_trail_4);
                    { let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(40); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWaterTrail_UpdateList", &PROBE_N, &[], true); }
    r_
}

pub fn EftWaterTrail_Update(c: &mut Ctx, mut p_trail_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWaterTrail_Update", &PROBE_N, &[(p_trail_1) as u32], false); }
    let r_ = 'f: {
    F::EftWaterDrop_UpdateList(c, (p_trail_1).wrapping_add(4), (p_trail_1).wrapping_add(8));
    F::EftWaterSpray_UpdateList(c, (p_trail_1).wrapping_add(12), (p_trail_1).wrapping_add(16));
    F::EftWaterRing_UpdateList(c, (p_trail_1).wrapping_add(20), (p_trail_1).wrapping_add(24));
    break 'f (((F::EftWaterTrail_IsEmpty(c, p_trail_1)) != (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWaterTrail_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftWaterTrail_DrawList(c: &mut Ctx, mut p_trail_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWaterTrail_DrawList", &PROBE_N, &[(p_trail_1) as u32], false); }
    let r_ = 'f: {
    {
        'l1: loop {
            if !(((((p_trail_1) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = c.m.r32((p_trail_1).wrapping_add(4)); F::EftWaterDrop_DrawList(c, a0_, sym::D_002C3630) };
                { let a0_ = c.m.r32((p_trail_1).wrapping_add(12)); F::EftWaterSpray_DrawList(c, a0_, sym::D_002C3630) };
                { let a0_ = c.m.r32((p_trail_1).wrapping_add(20)); F::EftWaterRing_DrawList(c, a0_) };
            }
            p_trail_1 = c.m.r32((p_trail_1).wrapping_add(32));
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWaterTrail_DrawList", &PROBE_N, &[], true); }
    r_
}

pub fn EftWaterTrail_FadeOut(c: &mut Ctx, mut p_trail_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWaterTrail_FadeOut", &PROBE_N, &[(p_trail_1) as u32], false); }
    let r_ = 'f: {
    if ((((!(((((c.m.r32(p_trail_1) as i32)) & (64i32))) != 0)) as i32)) != 0) {
        { let a0_ = c.m.r32((p_trail_1).wrapping_add(4)); F::EftWaterDrop_FadeOutList(c, a0_) };
        { let a0_ = c.m.r32((p_trail_1).wrapping_add(20)); F::EftWaterRing_FadeOutList(c, a0_) };
        { let a0_ = c.m.r32((p_trail_1).wrapping_add(12)); F::EftWaterSpray_FadeOutList(c, a0_) };
        { let r_ = 64i32; let a_ = p_trail_1; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWaterTrail_FadeOut", &PROBE_N, &[], true); }
    r_
}

pub fn EftWaterDrop_Alloc(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWaterDrop_Alloc", &PROBE_N, &[], false); }
    let mut l_drop_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_drop_1 = { let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(12); c.m.r32(a_) };
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (60i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32((l_drop_1).wrapping_add(100)) as i32)) == (0i32)) as i32)) != 0) {
                    { let _ = F::memset(c, l_drop_1, 0i32, 112u32); };
                    break 'f l_drop_1;
                }
            }
            { { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; }; { let o_ = l_drop_1; let n_ = (o_).wrapping_add(112); l_drop_1 = n_; }; };
        }
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWaterDrop_Alloc", &PROBE_N, &[], true); }
    r_
}

pub fn EftWaterDrop_Spawn(c: &mut Ctx, mut p_head_1: u32, mut p_tail_2: u32, p_pos_3: u32, mut p_yaw_4: F32, mut p_pitch_5: F32, mut p_speed_6: F32, mut p_rot_7: F32, mut p_rotVel_8: F32, mut p_size_9: F32, mut p_sizeVel_10: F32, mut p_sizeDamp_11: F32, mut p_gravity_12: F32, mut p_life_13: F32, mut p_delay_14: F32, mut p_r_15: u8, mut p_g_16: u8, mut p_b_17: u8, mut p_tex_18: u8, mut p_layer_19: u8, mut p_flags_20: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWaterDrop_Spawn", &PROBE_N, &[(p_head_1) as u32, (p_tail_2) as u32, (p_pos_3) as u32, (p_yaw_4).0, (p_pitch_5).0, (p_speed_6).0, (p_rot_7).0, (p_rotVel_8).0, (p_size_9).0, (p_sizeVel_10).0, (p_sizeDamp_11).0, (p_gravity_12).0, (p_life_13).0, (p_delay_14).0, (p_r_15) as u32, (p_g_16) as u32, (p_b_17) as u32, (p_tex_18) as u32, (p_layer_19) as u32, (p_flags_20) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_drop_21: u32 = 0;
    let mut l_angle_22: F32 = F32(0);
    rt::memcpy(c, fp, p_pos_3, 16);
    let r_ = 'f: {
        l_drop_21 = F::EftWaterDrop_Alloc(c);
        if (((((l_drop_21) != (((0i32) as u32))) as i32)) != 0) {
            if (((((c.m.r32(p_head_1)) == (((0i32) as u32))) as i32)) != 0) {
                { let v_ = l_drop_21; let a_ = p_head_1; c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_drop_21).wrapping_add(104); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_drop_21).wrapping_add(108); c.m.w32(a_, v_); };
            } else {
                { let v_ = c.m.r32(p_tail_2); let a_ = (l_drop_21).wrapping_add(104); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_drop_21).wrapping_add(108); c.m.w32(a_, v_); };
            }
            if (((((c.m.r32(p_tail_2)) != (((0i32) as u32))) as i32)) != 0) {
                { let v_ = l_drop_21; let a_ = (c.m.r32(p_tail_2)).wrapping_add(108); c.m.w32(a_, v_); };
            }
            { let v_ = l_drop_21; let a_ = p_tail_2; c.m.w32(a_, v_); };
            { let _ = D::call(c, fid::Vec4_Copy, &[((l_drop_21) as u64), ((sym::D_002C3630) as u64)]); };
            { let _ = D::call(c, fid::Vec4_Copy, &[(((l_drop_21).wrapping_add(16)) as u64), ((fp) as u64)]); };
            { let v_ = p_rot_7; let a_ = (l_drop_21).wrapping_add(60); c.m.w32(a_, (v_).0); };
            { let v_ = p_rotVel_8; let a_ = (l_drop_21).wrapping_add(64); c.m.w32(a_, (v_).0); };
            { let v_ = p_size_9; let a_ = (l_drop_21).wrapping_add(48); c.m.w32(a_, (v_).0); };
            { let v_ = p_sizeVel_10; let a_ = (l_drop_21).wrapping_add(52); c.m.w32(a_, (v_).0); };
            { let v_ = (if (((rt::feq(p_sizeDamp_11, F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { rt::fdiv(F32(0x3f800000), p_sizeDamp_11) }); let a_ = (l_drop_21).wrapping_add(56); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul(p_gravity_12, F32(0x411ce809)); let a_ = (l_drop_21).wrapping_add(76); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3d088888); let a_ = (l_drop_21).wrapping_add(84); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul(p_delay_14, F32(0x41f00000)); let a_ = (l_drop_21).wrapping_add(88); c.m.w32(a_, (v_).0); };
            { let v_ = p_r_15; let a_ = (l_drop_21).wrapping_add(92); c.m.w8(a_, v_); };
            { let v_ = p_g_16; let a_ = (l_drop_21).wrapping_add(93); c.m.w8(a_, v_); };
            { let v_ = p_b_17; let a_ = (l_drop_21).wrapping_add(94); c.m.w8(a_, v_); };
            { let v_ = p_tex_18; let a_ = (l_drop_21).wrapping_add(95); c.m.w8(a_, v_); };
            { let v_ = p_layer_19; let a_ = (l_drop_21).wrapping_add(96); c.m.w8(a_, v_); };
            { let v_ = p_flags_20; let a_ = (l_drop_21).wrapping_add(100); c.m.w32(a_, (v_) as u32); };
            { let v_ = F32(0x00000000); let a_ = (l_drop_21).wrapping_add(80); c.m.w32(a_, (v_).0); };
            l_angle_22 = F::EftMath_WrapAngle(c, rt::fdiv(rt::fmul(p_yaw_4, F32(0x40490fda)), F32(0x43340000)));
            { let v_ = F::Mathf_SinFast(c, l_angle_22); let a_ = (l_drop_21).wrapping_add(32); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = ((l_drop_21).wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = F::Mathf_CosFast(c, l_angle_22); let a_ = ((l_drop_21).wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            l_angle_22 = rt::fdiv(rt::fmul(p_pitch_5, F32(0x40490fda)), F32(0x43340000));
            { let v_ = F32(0x3f800000); let a_ = ((l_drop_21).wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            { let _ = D::call(c, fid::Vec3_Normalize, &[(((l_drop_21).wrapping_add(32)) as u64), (((l_drop_21).wrapping_add(32)) as u64)]); };
            { let v_ = rt::fmul(rt::fmul(F32(c.m.r32((l_drop_21).wrapping_add(32))), p_speed_6), F::Mathf_CosFast(c, l_angle_22)); let a_ = (l_drop_21).wrapping_add(32); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul(rt::fneg(p_speed_6), F::Mathf_SinFast(c, l_angle_22)); let a_ = ((l_drop_21).wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul(rt::fmul(F32(c.m.r32(((l_drop_21).wrapping_add(32)).wrapping_add(8))), p_speed_6), F::Mathf_CosFast(c, l_angle_22)); let a_ = ((l_drop_21).wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            if ((((p_flags_20) & (256i32))) != 0) {
                { let v_ = F32(0x00000000); let a_ = (l_drop_21).wrapping_add(68); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fdiv(F32(0x43000000), rt::fmul(p_life_13, F32(0x41f00000))); let a_ = (l_drop_21).wrapping_add(72); c.m.w32(a_, (v_).0); };
            } else {
                { let v_ = F32(0x43000000); let a_ = (l_drop_21).wrapping_add(68); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fdiv(F32(0x43000000), rt::fmul(p_life_13, F32(0x41f00000))); let a_ = (l_drop_21).wrapping_add(72); c.m.w32(a_, (v_).0); };
            }
            { let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftWaterDrop_Spawn", &PROBE_N, &[], true); }
    r_
}

pub fn EftWaterDrop_UpdateList(c: &mut Ctx, mut p_head_1: u32, mut p_tail_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWaterDrop_UpdateList", &PROBE_N, &[(p_head_1) as u32, (p_tail_2) as u32], false); }
    let mut l_next_3: u32 = 0;
    let mut l_drop_4: u32 = 0;
    let mut l_done_5: i32 = 0;
    let r_ = 'f: {
    if ((((!(((({ let a_ = (((D::call(c, fid::Battle_GetWork, &[])) as u32)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
        l_next_3 = c.m.r32(p_head_1);
        'l1: while (((((l_next_3) != (((0i32) as u32))) as i32)) != 0) {
            l_drop_4 = l_next_3;
            l_done_5 = F::EftWaterDrop_Update(c, l_next_3);
            l_next_3 = c.m.r32((l_next_3).wrapping_add(108));
            if ((l_done_5) != 0) {
                { let v_ = 0i32; let a_ = (l_drop_4).wrapping_add(100); c.m.w32(a_, (v_) as u32); };
                if (((((c.m.r32((l_drop_4).wrapping_add(104))) == (((0i32) as u32))) as i32)) != 0) {
                    { let v_ = c.m.r32((l_drop_4).wrapping_add(108)); let a_ = p_head_1; c.m.w32(a_, v_); };
                } else {
                    { let v_ = c.m.r32((l_drop_4).wrapping_add(108)); let a_ = (c.m.r32((l_drop_4).wrapping_add(104))).wrapping_add(108); c.m.w32(a_, v_); };
                }
                if (((((c.m.r32((l_drop_4).wrapping_add(108))) == (((0i32) as u32))) as i32)) != 0) {
                    { let v_ = c.m.r32((l_drop_4).wrapping_add(104)); let a_ = p_tail_2; c.m.w32(a_, v_); };
                } else {
                    { let v_ = c.m.r32((l_drop_4).wrapping_add(104)); let a_ = (c.m.r32((l_drop_4).wrapping_add(108))).wrapping_add(104); c.m.w32(a_, v_); };
                }
                { let v_ = ((0i32) as u32); let a_ = (l_drop_4).wrapping_add(104); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_drop_4).wrapping_add(108); c.m.w32(a_, v_); };
                { let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWaterDrop_UpdateList", &PROBE_N, &[], true); }
    r_
}

pub fn EftWaterDrop_Update(c: &mut Ctx, mut p_drop_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWaterDrop_Update", &PROBE_N, &[(p_drop_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32((p_drop_1).wrapping_add(100)) as i32)) & (1i32))) != 0) {
        { let r_ = F32(0x3f800000); let a_ = (p_drop_1).wrapping_add(88); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if (((rt::fle(F32(c.m.r32((p_drop_1).wrapping_add(88))), F32(0x00000000)) as i32)) != 0) {
            { let v_ = F32(0x00000000); let a_ = (p_drop_1).wrapping_add(88); c.m.w32(a_, (v_).0); };
            { let v_ = (((((c.m.r32((p_drop_1).wrapping_add(100)) as i32)) & ((!(1i32))))) | (2i32)); let a_ = (p_drop_1).wrapping_add(100); c.m.w32(a_, (v_) as u32); };
        }
    }
    if (((((c.m.r32((p_drop_1).wrapping_add(100)) as i32)) & (2i32))) != 0) {
        { let v_ = rt::fadd(F32(c.m.r32((p_drop_1).wrapping_add(16))), rt::fmul(F32(c.m.r32((p_drop_1).wrapping_add(32))), F32(c.m.r32((p_drop_1).wrapping_add(80))))); let a_ = p_drop_1; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(rt::fadd(F32(c.m.r32(((p_drop_1).wrapping_add(16)).wrapping_add(4))), rt::fmul(F32(c.m.r32(((p_drop_1).wrapping_add(32)).wrapping_add(4))), F32(c.m.r32((p_drop_1).wrapping_add(80))))), rt::fmul(rt::fmul(F32(c.m.r32((p_drop_1).wrapping_add(76))), F32(c.m.r32((p_drop_1).wrapping_add(80)))), F32(c.m.r32((p_drop_1).wrapping_add(80))))); let a_ = (p_drop_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((p_drop_1).wrapping_add(16)).wrapping_add(8))), rt::fmul(F32(c.m.r32(((p_drop_1).wrapping_add(32)).wrapping_add(8))), F32(c.m.r32((p_drop_1).wrapping_add(80))))); let a_ = (p_drop_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let r_ = F32(c.m.r32((p_drop_1).wrapping_add(52))); let a_ = (p_drop_1).wrapping_add(48); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = rt::fmul(F32(c.m.r32((p_drop_1).wrapping_add(52))), F32(c.m.r32((p_drop_1).wrapping_add(56)))); let a_ = (p_drop_1).wrapping_add(52); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32((p_drop_1).wrapping_add(84))); let a_ = (p_drop_1).wrapping_add(80); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if ((((!(((((c.m.r32((p_drop_1).wrapping_add(100)) as i32)) & (64i32))) != 0)) as i32)) != 0) {
            if (((((c.m.r32((p_drop_1).wrapping_add(100)) as i32)) & (4i32))) != 0) {
                { let r_ = F32(c.m.r32((p_drop_1).wrapping_add(72))); let a_ = (p_drop_1).wrapping_add(68); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::fge(F32(c.m.r32((p_drop_1).wrapping_add(68))), F32(0x43000000)) as i32)) != 0) {
                    { let v_ = F32(0x43000000); let a_ = (p_drop_1).wrapping_add(68); c.m.w32(a_, (v_).0); };
                    if (((((c.m.r32((p_drop_1).wrapping_add(100)) as i32)) & (256i32))) != 0) {
                        { let r_ = 64i32; let a_ = (p_drop_1).wrapping_add(100); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                }
            } else {
                if (((((c.m.r32((p_drop_1).wrapping_add(100)) as i32)) & (8i32))) != 0) {
                    { let r_ = F32(c.m.r32((p_drop_1).wrapping_add(72))); let a_ = (p_drop_1).wrapping_add(68); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if (((rt::fle(F32(c.m.r32((p_drop_1).wrapping_add(68))), F32(0x00000000)) as i32)) != 0) {
                        { let v_ = F32(0x00000000); let a_ = (p_drop_1).wrapping_add(68); c.m.w32(a_, (v_).0); };
                        { let r_ = 64i32; let a_ = (p_drop_1).wrapping_add(100); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                }
            }
        }
    }
    if (((((c.m.r32((p_drop_1).wrapping_add(100)) as i32)) & (64i32))) != 0) {
        if (((((c.m.r32((p_drop_1).wrapping_add(100)) as i32)) & (256i32))) != 0) {
            { let r_ = F32(c.m.r32((p_drop_1).wrapping_add(72))); let a_ = (p_drop_1).wrapping_add(68); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::fge(F32(c.m.r32((p_drop_1).wrapping_add(68))), F32(0x43000000)) as i32)) != 0) {
                { let v_ = F32(0x43000000); let a_ = (p_drop_1).wrapping_add(68); c.m.w32(a_, (v_).0); };
                { let r_ = 128i32; let a_ = (p_drop_1).wrapping_add(100); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        } else {
            if (((((c.m.r32((p_drop_1).wrapping_add(100)) as i32)) & (8i32))) != 0) {
                { let r_ = F32(c.m.r32((p_drop_1).wrapping_add(72))); let a_ = (p_drop_1).wrapping_add(68); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::fle(F32(c.m.r32((p_drop_1).wrapping_add(68))), F32(0x00000000)) as i32)) != 0) {
                    { let v_ = F32(0x00000000); let a_ = (p_drop_1).wrapping_add(68); c.m.w32(a_, (v_).0); };
                    { let r_ = 128i32; let a_ = (p_drop_1).wrapping_add(100); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
        }
    }
    if (((((c.m.r32((p_drop_1).wrapping_add(100)) as i32)) & (128i32))) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWaterDrop_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftWaterDrop_DrawList(c: &mut Ctx, mut p_drop_1: u32, p_origin_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWaterDrop_DrawList", &PROBE_N, &[(p_drop_1) as u32, (p_origin_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    rt::memcpy(c, fp, p_origin_2, 16);
    let r_ = 'f: {
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Mtx_Copy(c, fp.wrapping_add(16), a1_) };
        {
            'l1: loop {
                if !(((((p_drop_1) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((c.m.r32((p_drop_1).wrapping_add(100)) as i32)) & (2i32))) != 0) {
                        { let _ = D::call(c, fid::Vec4_Add, &[((fp.wrapping_add(80)) as u64), ((p_drop_1) as u64), ((fp) as u64)]); };
                        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(80)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        if (((((c.m.r32((p_drop_1).wrapping_add(100)) as i32)) & (256i32))) != 0) {
                            { let a2_ = F32(c.m.r32((p_drop_1).wrapping_add(48))); let a3_ = F32(c.m.r32((p_drop_1).wrapping_add(60))); let a4_ = c.m.r8((p_drop_1).wrapping_add(92)); let a5_ = c.m.r8((p_drop_1).wrapping_add(93)); let a6_ = c.m.r8((p_drop_1).wrapping_add(94)); let a8_ = ((rt::f2u(F32(c.m.r32((p_drop_1).wrapping_add(68))))) as u8); let a9_ = ((c.m.r32(sym::gEftDust)).wrapping_add(176)).wrapping_add((((c.m.r8((p_drop_1).wrapping_add(95))) as u32)).wrapping_mul(16)); let a10_ = c.m.r8((p_drop_1).wrapping_add(96)); F::EftWater_DrawBillboard(c, fp.wrapping_add(80), fp.wrapping_add(16), a2_, a3_, a4_, a5_, a6_, ((96i32) as u8), a8_, a9_, a10_) };
                        } else {
                            { let a2_ = F32(c.m.r32((p_drop_1).wrapping_add(48))); let a3_ = F32(c.m.r32((p_drop_1).wrapping_add(60))); let a4_ = c.m.r8((p_drop_1).wrapping_add(92)); let a5_ = c.m.r8((p_drop_1).wrapping_add(93)); let a6_ = c.m.r8((p_drop_1).wrapping_add(94)); let a7_ = ((rt::f2u(F32(c.m.r32((p_drop_1).wrapping_add(68))))) as u8); let a9_ = ((c.m.r32(sym::gEftDust)).wrapping_add(176)).wrapping_add((((c.m.r8((p_drop_1).wrapping_add(95))) as u32)).wrapping_mul(16)); let a10_ = c.m.r8((p_drop_1).wrapping_add(96)); F::EftWater_DrawBillboard(c, fp.wrapping_add(80), fp.wrapping_add(16), a2_, a3_, a4_, a5_, a6_, a7_, ((0i32) as u8), a9_, a10_) };
                        }
                    }
                }
                p_drop_1 = c.m.r32((p_drop_1).wrapping_add(108));
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftWaterDrop_DrawList", &PROBE_N, &[], true); }
    r_
}

pub fn EftWaterDrop_FadeOutList(c: &mut Ctx, mut p_drop_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWaterDrop_FadeOutList", &PROBE_N, &[(p_drop_1) as u32], false); }
    let r_ = 'f: {
    {
        'l1: loop {
            if !(((((p_drop_1) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((!(((((c.m.r32((p_drop_1).wrapping_add(100)) as i32)) & (64i32))) != 0)) as i32)) != 0) {
                    if (((((c.m.r32((p_drop_1).wrapping_add(100)) as i32)) & (256i32))) != 0) {
                        { let v_ = rt::fdiv(rt::fsub(F32(0x43000000), F32(c.m.r32((p_drop_1).wrapping_add(68)))), F32(0x403fffff)); let a_ = (p_drop_1).wrapping_add(72); c.m.w32(a_, (v_).0); };
                    } else {
                        { let v_ = rt::fdiv(F32(c.m.r32((p_drop_1).wrapping_add(68))), F32(0x403fffff)); let a_ = (p_drop_1).wrapping_add(72); c.m.w32(a_, (v_).0); };
                    }
                    { let r_ = 64i32; let a_ = (p_drop_1).wrapping_add(100); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
            p_drop_1 = c.m.r32((p_drop_1).wrapping_add(108));
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWaterDrop_FadeOutList", &PROBE_N, &[], true); }
    r_
}

pub fn EftWaterRing_Alloc(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWaterRing_Alloc", &PROBE_N, &[], false); }
    let mut l_ring_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_ring_1 = { let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(16); c.m.r32(a_) };
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (30i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32((l_ring_1).wrapping_add(60)) as i32)) == (0i32)) as i32)) != 0) {
                    { let _ = F::memset(c, l_ring_1, 0i32, 80u32); };
                    break 'f l_ring_1;
                }
            }
            { { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; }; { let o_ = l_ring_1; let n_ = (o_).wrapping_add(80); l_ring_1 = n_; }; };
        }
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWaterRing_Alloc", &PROBE_N, &[], true); }
    r_
}

pub fn EftWaterRing_Spawn(c: &mut Ctx, mut p_head_1: u32, mut p_tail_2: u32, p_pos_3: u32, mut p_rot_4: F32, mut p_size_5: F32, mut p_sizeVel_6: F32, mut p_sizeDamp_7: F32, mut p_alphaMax_8: F32, mut p_time_9: F32, mut p_delay_10: F32, mut p_r_11: u8, mut p_g_12: u8, mut p_b_13: u8, mut p_tex_14: u8, mut p_layer_15: u8, mut p_flags_16: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWaterRing_Spawn", &PROBE_N, &[(p_head_1) as u32, (p_tail_2) as u32, (p_pos_3) as u32, (p_rot_4).0, (p_size_5).0, (p_sizeVel_6).0, (p_sizeDamp_7).0, (p_alphaMax_8).0, (p_time_9).0, (p_delay_10).0, (p_r_11) as u32, (p_g_12) as u32, (p_b_13) as u32, (p_tex_14) as u32, (p_layer_15) as u32, (p_flags_16) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_ring_17: u32 = 0;
    rt::memcpy(c, fp, p_pos_3, 16);
    let r_ = 'f: {
        l_ring_17 = F::EftWaterRing_Alloc(c);
        if (((((l_ring_17) != (((0i32) as u32))) as i32)) != 0) {
            if (((((c.m.r32(p_head_1)) == (((0i32) as u32))) as i32)) != 0) {
                { let v_ = l_ring_17; let a_ = p_head_1; c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_ring_17).wrapping_add(64); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_ring_17).wrapping_add(68); c.m.w32(a_, v_); };
            } else {
                { let v_ = c.m.r32(p_tail_2); let a_ = (l_ring_17).wrapping_add(64); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_ring_17).wrapping_add(68); c.m.w32(a_, v_); };
            }
            if (((((c.m.r32(p_tail_2)) != (((0i32) as u32))) as i32)) != 0) {
                { let v_ = l_ring_17; let a_ = (c.m.r32(p_tail_2)).wrapping_add(68); c.m.w32(a_, v_); };
            }
            { let v_ = l_ring_17; let a_ = p_tail_2; c.m.w32(a_, v_); };
            { let _ = D::call(c, fid::Vec4_Copy, &[((l_ring_17) as u64), ((fp) as u64)]); };
            { let v_ = p_size_5; let a_ = (l_ring_17).wrapping_add(16); c.m.w32(a_, (v_).0); };
            { let v_ = p_sizeVel_6; let a_ = (l_ring_17).wrapping_add(20); c.m.w32(a_, (v_).0); };
            { let v_ = (if (((rt::feq(p_sizeDamp_7, F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { rt::fdiv(F32(0x3f800000), p_sizeDamp_7) }); let a_ = (l_ring_17).wrapping_add(24); c.m.w32(a_, (v_).0); };
            { let v_ = p_rot_4; let a_ = (l_ring_17).wrapping_add(40); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul(p_delay_10, F32(0x41f00000)); let a_ = (l_ring_17).wrapping_add(48); c.m.w32(a_, (v_).0); };
            { let v_ = p_r_11; let a_ = (l_ring_17).wrapping_add(52); c.m.w8(a_, v_); };
            { let v_ = p_g_12; let a_ = (l_ring_17).wrapping_add(53); c.m.w8(a_, v_); };
            { let v_ = p_b_13; let a_ = (l_ring_17).wrapping_add(54); c.m.w8(a_, v_); };
            { let v_ = p_tex_14; let a_ = (l_ring_17).wrapping_add(55); c.m.w8(a_, v_); };
            { let v_ = p_layer_15; let a_ = (l_ring_17).wrapping_add(56); c.m.w8(a_, v_); };
            { let v_ = p_flags_16; let a_ = (l_ring_17).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
            if ((((p_flags_16) & (4i32))) != 0) {
                { let v_ = p_alphaMax_8; let a_ = (l_ring_17).wrapping_add(36); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = (l_ring_17).wrapping_add(28); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul(p_time_9, F32(0x3f333333)); let a_ = (l_ring_17).wrapping_add(44); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fdiv(p_alphaMax_8, rt::fmul(rt::fmul(p_time_9, F32(0x3e999999)), F32(0x41f00000))); let a_ = (l_ring_17).wrapping_add(32); c.m.w32(a_, (v_).0); };
            } else {
                if ((((p_flags_16) & (8i32))) != 0) {
                    { let v_ = p_time_9; let a_ = (l_ring_17).wrapping_add(44); c.m.w32(a_, (v_).0); };
                    { let v_ = p_alphaMax_8; let a_ = (l_ring_17).wrapping_add(28); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x00000000); let a_ = (l_ring_17).wrapping_add(36); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fdiv(p_alphaMax_8, rt::fmul(p_time_9, F32(0x41f00000))); let a_ = (l_ring_17).wrapping_add(32); c.m.w32(a_, (v_).0); };
                }
            }
            { let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(48); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftWaterRing_Spawn", &PROBE_N, &[], true); }
    r_
}

pub fn EftWaterRing_UpdateList(c: &mut Ctx, mut p_head_1: u32, mut p_tail_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWaterRing_UpdateList", &PROBE_N, &[(p_head_1) as u32, (p_tail_2) as u32], false); }
    let mut l_next_3: u32 = 0;
    let mut l_ring_4: u32 = 0;
    let mut l_done_5: i32 = 0;
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(48); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
        if ((((!(((({ let a_ = (((D::call(c, fid::Battle_GetWork, &[])) as u32)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
            l_next_3 = c.m.r32(p_head_1);
            'l1: while (((((l_next_3) != (((0i32) as u32))) as i32)) != 0) {
                l_ring_4 = l_next_3;
                l_done_5 = F::EftWaterRing_Update(c, l_next_3);
                l_next_3 = c.m.r32((l_next_3).wrapping_add(68));
                if ((l_done_5) != 0) {
                    { let v_ = 0i32; let a_ = (l_ring_4).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
                    if (((((c.m.r32((l_ring_4).wrapping_add(64))) == (((0i32) as u32))) as i32)) != 0) {
                        { let v_ = c.m.r32((l_ring_4).wrapping_add(68)); let a_ = p_head_1; c.m.w32(a_, v_); };
                    } else {
                        { let v_ = c.m.r32((l_ring_4).wrapping_add(68)); let a_ = (c.m.r32((l_ring_4).wrapping_add(64))).wrapping_add(68); c.m.w32(a_, v_); };
                    }
                    if (((((c.m.r32((l_ring_4).wrapping_add(68))) == (((0i32) as u32))) as i32)) != 0) {
                        { let v_ = c.m.r32((l_ring_4).wrapping_add(64)); let a_ = p_tail_2; c.m.w32(a_, v_); };
                    } else {
                        { let v_ = c.m.r32((l_ring_4).wrapping_add(64)); let a_ = (c.m.r32((l_ring_4).wrapping_add(68))).wrapping_add(64); c.m.w32(a_, v_); };
                    }
                    { let v_ = ((0i32) as u32); let a_ = (l_ring_4).wrapping_add(64); c.m.w32(a_, v_); };
                    { let v_ = ((0i32) as u32); let a_ = (l_ring_4).wrapping_add(68); c.m.w32(a_, v_); };
                    { let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(48); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWaterRing_UpdateList", &PROBE_N, &[], true); }
    r_
}

pub fn EftWaterRing_Update(c: &mut Ctx, mut p_ring_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWaterRing_Update", &PROBE_N, &[(p_ring_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        if (((((c.m.r32((p_ring_1).wrapping_add(60)) as i32)) & (1i32))) != 0) {
            { let r_ = F32(0x3f800000); let a_ = (p_ring_1).wrapping_add(48); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::fle(F32(c.m.r32((p_ring_1).wrapping_add(48))), F32(0x00000000)) as i32)) != 0) {
                { let v_ = F32(0x00000000); let a_ = (p_ring_1).wrapping_add(48); c.m.w32(a_, (v_).0); };
                { let v_ = (((((c.m.r32((p_ring_1).wrapping_add(60)) as i32)) | (2i32))) & ((!(1i32)))); let a_ = (p_ring_1).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
            }
        }
        if (((((c.m.r32((p_ring_1).wrapping_add(60)) as i32)) & (2i32))) != 0) {
            if ((F::EftWater_GetSurfaceY(c, fp)) != 0) {
                { let v_ = rt::fsub(F32(c.m.r32(fp)), F32(0x40800000)); let a_ = (p_ring_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
            }
            { let r_ = F32(c.m.r32((p_ring_1).wrapping_add(20))); let a_ = (p_ring_1).wrapping_add(16); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = rt::fmul(F32(c.m.r32((p_ring_1).wrapping_add(20))), F32(c.m.r32((p_ring_1).wrapping_add(24)))); let a_ = (p_ring_1).wrapping_add(20); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if ((((!(((((c.m.r32((p_ring_1).wrapping_add(60)) as i32)) & (64i32))) != 0)) as i32)) != 0) {
                if (((((c.m.r32((p_ring_1).wrapping_add(60)) as i32)) & (4i32))) != 0) {
                    { let r_ = F32(c.m.r32((p_ring_1).wrapping_add(32))); let a_ = (p_ring_1).wrapping_add(28); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if (((rt::fge(F32(c.m.r32((p_ring_1).wrapping_add(28))), F32(c.m.r32((p_ring_1).wrapping_add(36)))) as i32)) != 0) {
                        { let v_ = F32(c.m.r32((p_ring_1).wrapping_add(36))); let a_ = (p_ring_1).wrapping_add(28); c.m.w32(a_, (v_).0); };
                        { let r_ = (!(4i32)); let a_ = (p_ring_1).wrapping_add(60); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                        if (((((c.m.r32((p_ring_1).wrapping_add(60)) as i32)) & (8i32))) != 0) {
                            { let v_ = F32(0x00000000); let a_ = (p_ring_1).wrapping_add(36); c.m.w32(a_, (v_).0); };
                            { let v_ = rt::fdiv(F32(c.m.r32((p_ring_1).wrapping_add(28))), rt::fmul(F32(c.m.r32((p_ring_1).wrapping_add(44))), F32(0x41f00000))); let a_ = (p_ring_1).wrapping_add(32); c.m.w32(a_, (v_).0); };
                        }
                    }
                } else {
                    if (((((c.m.r32((p_ring_1).wrapping_add(60)) as i32)) & (8i32))) != 0) {
                        { let r_ = F32(c.m.r32((p_ring_1).wrapping_add(32))); let a_ = (p_ring_1).wrapping_add(28); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        if (((rt::fle(F32(c.m.r32((p_ring_1).wrapping_add(28))), F32(c.m.r32((p_ring_1).wrapping_add(36)))) as i32)) != 0) {
                            { let v_ = F32(c.m.r32((p_ring_1).wrapping_add(36))); let a_ = (p_ring_1).wrapping_add(28); c.m.w32(a_, (v_).0); };
                            { let v_ = (((((c.m.r32((p_ring_1).wrapping_add(60)) as i32)) & ((!(8i32))))) | (64i32)); let a_ = (p_ring_1).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
                        }
                    }
                }
            }
        }
        if (((((c.m.r32((p_ring_1).wrapping_add(60)) as i32)) & (64i32))) != 0) {
            { let r_ = F32(c.m.r32((p_ring_1).wrapping_add(32))); let a_ = (p_ring_1).wrapping_add(28); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::fle(F32(c.m.r32((p_ring_1).wrapping_add(28))), F32(0x00000000)) as i32)) != 0) {
                { let v_ = F32(0x00000000); let a_ = (p_ring_1).wrapping_add(28); c.m.w32(a_, (v_).0); };
                { let r_ = 128i32; let a_ = (p_ring_1).wrapping_add(60); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
        if (((((c.m.r32((p_ring_1).wrapping_add(60)) as i32)) & (128i32))) != 0) {
            break 'f 1i32;
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftWaterRing_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftWaterRing_DrawList(c: &mut Ctx, mut p_ring_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWaterRing_DrawList", &PROBE_N, &[(p_ring_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let r_ = 'f: {
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Mtx_Copy(c, fp, a1_) };
        {
            'l1: loop {
                if !(((((p_ring_1) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((c.m.r32((p_ring_1).wrapping_add(60)) as i32)) & (2i32))) != 0) {
                        { let a2_ = F32(c.m.r32((p_ring_1).wrapping_add(16))); let a3_ = F32(c.m.r32((p_ring_1).wrapping_add(40))); let a4_ = c.m.r8((p_ring_1).wrapping_add(52)); let a5_ = c.m.r8((p_ring_1).wrapping_add(53)); let a6_ = c.m.r8((p_ring_1).wrapping_add(54)); let a7_ = ((rt::f2u(F32(c.m.r32((p_ring_1).wrapping_add(28))))) as u8); let a8_ = ((c.m.r32(sym::gEftDust)).wrapping_add(176)).wrapping_add((((c.m.r8((p_ring_1).wrapping_add(55))) as u32)).wrapping_mul(16)); let a9_ = c.m.r8((p_ring_1).wrapping_add(56)); F::EftWater_DrawGroundQuad(c, p_ring_1, fp, a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_) };
                    }
                }
                p_ring_1 = c.m.r32((p_ring_1).wrapping_add(68));
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftWaterRing_DrawList", &PROBE_N, &[], true); }
    r_
}

pub fn EftWaterRing_FadeOutList(c: &mut Ctx, mut p_ring_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWaterRing_FadeOutList", &PROBE_N, &[(p_ring_1) as u32], false); }
    let r_ = 'f: {
    {
        'l1: loop {
            if !(((((p_ring_1) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((!(((((c.m.r32((p_ring_1).wrapping_add(60)) as i32)) & (64i32))) != 0)) as i32)) != 0) {
                    { let r_ = 64i32; let a_ = (p_ring_1).wrapping_add(60); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = rt::fdiv(F32(c.m.r32((p_ring_1).wrapping_add(28))), F32(0x403fffff)); let a_ = (p_ring_1).wrapping_add(32); c.m.w32(a_, (v_).0); };
                }
            }
            p_ring_1 = c.m.r32((p_ring_1).wrapping_add(68));
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWaterRing_FadeOutList", &PROBE_N, &[], true); }
    r_
}

pub fn EftWaterSpray_Alloc(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWaterSpray_Alloc", &PROBE_N, &[], false); }
    let mut l_spray_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_spray_1 = { let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(20); c.m.r32(a_) };
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (30i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32((l_spray_1).wrapping_add(132)) as i32)) == (0i32)) as i32)) != 0) {
                    { let _ = F::memset(c, l_spray_1, 0i32, 144u32); };
                    break 'f l_spray_1;
                }
            }
            { { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; }; { let o_ = l_spray_1; let n_ = (o_).wrapping_add(144); l_spray_1 = n_; }; };
        }
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWaterSpray_Alloc", &PROBE_N, &[], true); }
    r_
}

pub fn EftWaterSpray_Spawn(c: &mut Ctx, mut p_head_1: u32, mut p_tail_2: u32, p_pos_3: u32, mut p_dist_4: F32, mut p_height_5: F32, mut p_yaw_6: F32, mut p_pitch_7: F32, mut p_roll_8: F32, mut p_tilt_9: F32, mut p_tiltEnd_10: F32, mut p_speed_11: F32, mut p_size_12: F32, mut p_sizeVel_13: F32, mut p_sizeDamp_14: F32, mut p_gravity_15: F32, mut p_nearScale_16: F32, mut p_widthScale_17: F32, mut p_life_18: F32, mut p_delay_19: F32, mut p_r_20: u8, mut p_g_21: u8, mut p_b_22: u8, mut p_tex_23: u8, mut p_layer_24: u8, mut p_flags_25: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWaterSpray_Spawn", &PROBE_N, &[(p_head_1) as u32, (p_tail_2) as u32, (p_pos_3) as u32, (p_dist_4).0, (p_height_5).0, (p_yaw_6).0, (p_pitch_7).0, (p_roll_8).0, (p_tilt_9).0, (p_tiltEnd_10).0, (p_speed_11).0, (p_size_12).0, (p_sizeVel_13).0, (p_sizeDamp_14).0, (p_gravity_15).0, (p_nearScale_16).0, (p_widthScale_17).0, (p_life_18).0, (p_delay_19).0, (p_r_20) as u32, (p_g_21) as u32, (p_b_22) as u32, (p_tex_23) as u32, (p_layer_24) as u32, (p_flags_25) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_spray_26: u32 = 0;
    let mut l_angle_27: F32 = F32(0);
    rt::memcpy(c, fp, p_pos_3, 16);
    let r_ = 'f: {
        l_spray_26 = F::EftWaterSpray_Alloc(c);
        if (((((l_spray_26) != (((0i32) as u32))) as i32)) != 0) {
            if (((((c.m.r32(p_head_1)) == (((0i32) as u32))) as i32)) != 0) {
                { let v_ = l_spray_26; let a_ = p_head_1; c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_spray_26).wrapping_add(136); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_spray_26).wrapping_add(140); c.m.w32(a_, v_); };
            } else {
                { let v_ = c.m.r32(p_tail_2); let a_ = (l_spray_26).wrapping_add(136); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_spray_26).wrapping_add(140); c.m.w32(a_, v_); };
            }
            if (((((c.m.r32(p_tail_2)) != (((0i32) as u32))) as i32)) != 0) {
                { let v_ = l_spray_26; let a_ = (c.m.r32(p_tail_2)).wrapping_add(140); c.m.w32(a_, v_); };
            }
            { let v_ = l_spray_26; let a_ = p_tail_2; c.m.w32(a_, v_); };
            { let _ = D::call(c, fid::Vec4_Set, &[((l_spray_26) as u64), ((F32(0x00000000)).0 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
            { let _ = D::call(c, fid::Vec4_Copy, &[(((l_spray_26).wrapping_add(16)) as u64), ((fp) as u64)]); };
            { let r_ = p_height_5; let a_ = ((l_spray_26).wrapping_add(16)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let v_ = p_size_12; let a_ = (l_spray_26).wrapping_add(72); c.m.w32(a_, (v_).0); };
            { let v_ = p_sizeVel_13; let a_ = (l_spray_26).wrapping_add(76); c.m.w32(a_, (v_).0); };
            { let v_ = p_tilt_9; let a_ = (l_spray_26).wrapping_add(64); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fdiv(rt::fsub(p_tiltEnd_10, p_tilt_9), rt::fmul(p_life_18, F32(0x41f00000))); let a_ = (l_spray_26).wrapping_add(68); c.m.w32(a_, (v_).0); };
            if (((rt::fne(p_sizeDamp_14, F32(0x00000000)) as i32)) != 0) {
                { let v_ = rt::fdiv(F32(0x3f800000), p_sizeDamp_14); let a_ = (l_spray_26).wrapping_add(80); c.m.w32(a_, (v_).0); };
            } else {
                { let v_ = F32(0x00000000); let a_ = (l_spray_26).wrapping_add(80); c.m.w32(a_, (v_).0); };
            }
            { let v_ = rt::fmul(p_gravity_15, F32(0x411ce809)); let a_ = (l_spray_26).wrapping_add(100); c.m.w32(a_, (v_).0); };
            { let v_ = p_nearScale_16; let a_ = (l_spray_26).wrapping_add(104); c.m.w32(a_, (v_).0); };
            { let v_ = p_widthScale_17; let a_ = (l_spray_26).wrapping_add(108); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3d088888); let a_ = (l_spray_26).wrapping_add(116); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul(p_delay_19, F32(0x41f00000)); let a_ = (l_spray_26).wrapping_add(120); c.m.w32(a_, (v_).0); };
            { let v_ = p_roll_8; let a_ = (l_spray_26).wrapping_add(96); c.m.w32(a_, (v_).0); };
            { let v_ = p_r_20; let a_ = (l_spray_26).wrapping_add(124); c.m.w8(a_, v_); };
            { let v_ = p_g_21; let a_ = (l_spray_26).wrapping_add(125); c.m.w8(a_, v_); };
            { let v_ = p_b_22; let a_ = (l_spray_26).wrapping_add(126); c.m.w8(a_, v_); };
            { let v_ = p_tex_23; let a_ = (l_spray_26).wrapping_add(127); c.m.w8(a_, v_); };
            { let v_ = p_layer_24; let a_ = (l_spray_26).wrapping_add(128); c.m.w8(a_, v_); };
            { let v_ = p_flags_25; let a_ = (l_spray_26).wrapping_add(132); c.m.w32(a_, (v_) as u32); };
            { let v_ = F32(0x00000000); let a_ = (l_spray_26).wrapping_add(112); c.m.w32(a_, (v_).0); };
            l_angle_27 = F::EftMath_WrapAngle(c, rt::fdiv(rt::fmul(p_yaw_6, F32(0x40490fda)), F32(0x43340000)));
            { let v_ = F::Mathf_SinFast(c, l_angle_27); let a_ = (l_spray_26).wrapping_add(48); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = ((l_spray_26).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = F::Mathf_CosFast(c, l_angle_27); let a_ = ((l_spray_26).wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f800000); let a_ = ((l_spray_26).wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            l_angle_27 = rt::fdiv(rt::fmul(p_pitch_7, F32(0x40490fda)), F32(0x43340000));
            { let _ = D::call(c, fid::Vec3_Normalize, &[(((l_spray_26).wrapping_add(48)) as u64), (((l_spray_26).wrapping_add(48)) as u64)]); };
            { let v_ = rt::fmul(rt::fmul(F32(c.m.r32((l_spray_26).wrapping_add(48))), p_speed_11), F::Mathf_CosFast(c, l_angle_27)); let a_ = (l_spray_26).wrapping_add(32); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul(rt::fneg(p_speed_11), F::Mathf_SinFast(c, l_angle_27)); let a_ = ((l_spray_26).wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul(rt::fmul(F32(c.m.r32(((l_spray_26).wrapping_add(48)).wrapping_add(8))), p_speed_11), F::Mathf_CosFast(c, l_angle_27)); let a_ = ((l_spray_26).wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            F::Vec4_Scale(c, fp.wrapping_add(16), (l_spray_26).wrapping_add(48), p_dist_4);
            { let _ = D::call(c, fid::Vec4_Add, &[(((l_spray_26).wrapping_add(16)) as u64), (((l_spray_26).wrapping_add(16)) as u64), ((fp.wrapping_add(16)) as u64)]); };
            { let v_ = F32(0x3f800000); let a_ = ((l_spray_26).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            if ((((p_flags_25) & (256i32))) != 0) {
                { let v_ = F32(0x00000000); let a_ = (l_spray_26).wrapping_add(84); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x42c00000); let a_ = (l_spray_26).wrapping_add(92); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fdiv(F32(0x42c00000), rt::fmul(rt::fmul(p_life_18, F32(0x3fcccccc)), F32(0x41f00000))); let a_ = (l_spray_26).wrapping_add(88); c.m.w32(a_, (v_).0); };
            }
            { let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(52); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftWaterSpray_Spawn", &PROBE_N, &[], true); }
    r_
}

pub fn EftWaterSpray_UpdateList(c: &mut Ctx, mut p_head_1: u32, mut p_tail_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWaterSpray_UpdateList", &PROBE_N, &[(p_head_1) as u32, (p_tail_2) as u32], false); }
    let mut l_next_3: u32 = 0;
    let mut l_spray_4: u32 = 0;
    let mut l_done_5: i32 = 0;
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(52); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
        if ((((!(((({ let a_ = (((D::call(c, fid::Battle_GetWork, &[])) as u32)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
            l_next_3 = c.m.r32(p_head_1);
            'l1: while (((((l_next_3) != (((0i32) as u32))) as i32)) != 0) {
                l_spray_4 = l_next_3;
                l_done_5 = F::EftWaterSpray_Update(c, l_next_3);
                l_next_3 = c.m.r32((l_next_3).wrapping_add(140));
                if ((l_done_5) != 0) {
                    { let v_ = 0i32; let a_ = (l_spray_4).wrapping_add(132); c.m.w32(a_, (v_) as u32); };
                    if (((((c.m.r32((l_spray_4).wrapping_add(136))) == (((0i32) as u32))) as i32)) != 0) {
                        { let v_ = c.m.r32((l_spray_4).wrapping_add(140)); let a_ = p_head_1; c.m.w32(a_, v_); };
                    } else {
                        { let v_ = c.m.r32((l_spray_4).wrapping_add(140)); let a_ = (c.m.r32((l_spray_4).wrapping_add(136))).wrapping_add(140); c.m.w32(a_, v_); };
                    }
                    if (((((c.m.r32((l_spray_4).wrapping_add(140))) == (((0i32) as u32))) as i32)) != 0) {
                        { let v_ = c.m.r32((l_spray_4).wrapping_add(136)); let a_ = p_tail_2; c.m.w32(a_, v_); };
                    } else {
                        { let v_ = c.m.r32((l_spray_4).wrapping_add(136)); let a_ = (c.m.r32((l_spray_4).wrapping_add(140))).wrapping_add(136); c.m.w32(a_, v_); };
                    }
                    { let v_ = ((0i32) as u32); let a_ = (l_spray_4).wrapping_add(136); c.m.w32(a_, v_); };
                    { let v_ = ((0i32) as u32); let a_ = (l_spray_4).wrapping_add(140); c.m.w32(a_, v_); };
                    { let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(52); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWaterSpray_UpdateList", &PROBE_N, &[], true); }
    r_
}

pub fn EftWaterSpray_Update(c: &mut Ctx, mut p_spray_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWaterSpray_Update", &PROBE_N, &[(p_spray_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32((p_spray_1).wrapping_add(132)) as i32)) & (1i32))) != 0) {
        { let r_ = F32(0x3f800000); let a_ = (p_spray_1).wrapping_add(120); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if (((rt::fle(F32(c.m.r32((p_spray_1).wrapping_add(120))), F32(0x00000000)) as i32)) != 0) {
            { let v_ = F32(0x00000000); let a_ = (p_spray_1).wrapping_add(120); c.m.w32(a_, (v_).0); };
            { let v_ = (((((c.m.r32((p_spray_1).wrapping_add(132)) as i32)) | (2i32))) & ((!(1i32)))); let a_ = (p_spray_1).wrapping_add(132); c.m.w32(a_, (v_) as u32); };
        }
    }
    if (((((c.m.r32((p_spray_1).wrapping_add(132)) as i32)) & (2i32))) != 0) {
        { let v_ = rt::fadd(F32(c.m.r32((p_spray_1).wrapping_add(16))), rt::fmul(F32(c.m.r32((p_spray_1).wrapping_add(32))), F32(c.m.r32((p_spray_1).wrapping_add(112))))); let a_ = p_spray_1; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(rt::fadd(F32(c.m.r32(((p_spray_1).wrapping_add(16)).wrapping_add(4))), rt::fmul(F32(c.m.r32(((p_spray_1).wrapping_add(32)).wrapping_add(4))), F32(c.m.r32((p_spray_1).wrapping_add(112))))), rt::fmul(rt::fmul(F32(c.m.r32((p_spray_1).wrapping_add(100))), F32(c.m.r32((p_spray_1).wrapping_add(112)))), F32(c.m.r32((p_spray_1).wrapping_add(112))))); let a_ = (p_spray_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((p_spray_1).wrapping_add(16)).wrapping_add(8))), rt::fmul(F32(c.m.r32(((p_spray_1).wrapping_add(32)).wrapping_add(8))), F32(c.m.r32((p_spray_1).wrapping_add(112))))); let a_ = (p_spray_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let r_ = F32(c.m.r32((p_spray_1).wrapping_add(76))); let a_ = (p_spray_1).wrapping_add(72); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = rt::fmul(F32(c.m.r32((p_spray_1).wrapping_add(76))), F32(c.m.r32((p_spray_1).wrapping_add(80)))); let a_ = (p_spray_1).wrapping_add(76); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32((p_spray_1).wrapping_add(68))); let a_ = (p_spray_1).wrapping_add(64); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32((p_spray_1).wrapping_add(116))); let a_ = (p_spray_1).wrapping_add(112); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if ((((!(((((c.m.r32((p_spray_1).wrapping_add(132)) as i32)) & (64i32))) != 0)) as i32)) != 0) {
            if (((((c.m.r32((p_spray_1).wrapping_add(132)) as i32)) & (256i32))) != 0) {
                { let r_ = F32(c.m.r32((p_spray_1).wrapping_add(88))); let a_ = (p_spray_1).wrapping_add(84); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::fge(F32(c.m.r32((p_spray_1).wrapping_add(84))), F32(c.m.r32((p_spray_1).wrapping_add(92)))) as i32)) != 0) {
                    { let v_ = F32(c.m.r32((p_spray_1).wrapping_add(92))); let a_ = (p_spray_1).wrapping_add(84); c.m.w32(a_, (v_).0); };
                    { let r_ = 64i32; let a_ = (p_spray_1).wrapping_add(132); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
        }
    }
    if (((((c.m.r32((p_spray_1).wrapping_add(132)) as i32)) & (64i32))) != 0) {
        if (((((c.m.r32((p_spray_1).wrapping_add(132)) as i32)) & (256i32))) != 0) {
            { let r_ = F32(c.m.r32((p_spray_1).wrapping_add(88))); let a_ = (p_spray_1).wrapping_add(84); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::fge(F32(c.m.r32((p_spray_1).wrapping_add(84))), F32(c.m.r32((p_spray_1).wrapping_add(92)))) as i32)) != 0) {
                { let v_ = F32(c.m.r32((p_spray_1).wrapping_add(92))); let a_ = (p_spray_1).wrapping_add(84); c.m.w32(a_, (v_).0); };
                { let r_ = 128i32; let a_ = (p_spray_1).wrapping_add(132); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
    }
    if (((((c.m.r32((p_spray_1).wrapping_add(132)) as i32)) & (128i32))) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWaterSpray_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftWaterSpray_DrawList(c: &mut Ctx, mut p_spray_1: u32, p_origin_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWaterSpray_DrawList", &PROBE_N, &[(p_spray_1) as u32, (p_origin_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(192) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 192);
    rt::memcpy(c, fp, p_origin_2, 16);
    let r_ = 'f: {
        rt::memset(c, fp.wrapping_add(144), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(144).wrapping_add(0), (v_).0); }
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(144).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(144).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(144).wrapping_add(12), (v_).0); }
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Mtx_Copy(c, fp.wrapping_add(80), a1_) };
        {
            'l1: loop {
                if !(((((p_spray_1) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((c.m.r32((p_spray_1).wrapping_add(132)) as i32)) & (2i32))) != 0) {
                        { let a3_ = F32(c.m.r32((p_spray_1).wrapping_add(64))); F::Vec3_Lerp(c, fp.wrapping_add(176), fp.wrapping_add(144), (p_spray_1).wrapping_add(48), a3_) };
                        F::EftUtil_MakeFacingMtx(c, fp.wrapping_add(16), fp.wrapping_add(176), sym::D_002C3630);
                        { let _ = D::call(c, fid::Vec4_Add, &[((fp.wrapping_add(160)) as u64), ((p_spray_1) as u64), ((fp) as u64)]); };
                        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(160)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        { let a3_ = F32(c.m.r32((p_spray_1).wrapping_add(96))); let a4_ = F32(c.m.r32((p_spray_1).wrapping_add(72))); let a5_ = F32(c.m.r32((p_spray_1).wrapping_add(104))); let a7_ = F32(c.m.r32((p_spray_1).wrapping_add(108))); let a8_ = c.m.r8((p_spray_1).wrapping_add(124)); let a9_ = c.m.r8((p_spray_1).wrapping_add(125)); let a10_ = c.m.r8((p_spray_1).wrapping_add(126)); let a12_ = ((rt::f2u(F32(c.m.r32((p_spray_1).wrapping_add(84))))) as u8); let a13_ = ((c.m.r32(sym::gEftDust)).wrapping_add(176)).wrapping_add((((c.m.r8((p_spray_1).wrapping_add(127))) as u32)).wrapping_mul(16)); let a14_ = c.m.r8((p_spray_1).wrapping_add(128)); F::EftWater_DrawSprayQuad(c, fp.wrapping_add(160), fp.wrapping_add(16), fp.wrapping_add(80), a3_, a4_, a5_, F32(0x00000000), a7_, a8_, a9_, a10_, ((64i32) as u8), a12_, a13_, a14_) };
                    }
                }
                p_spray_1 = c.m.r32((p_spray_1).wrapping_add(140));
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftWaterSpray_DrawList", &PROBE_N, &[], true); }
    r_
}

pub fn EftWaterSpray_FadeOutList(c: &mut Ctx, mut p_spray_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWaterSpray_FadeOutList", &PROBE_N, &[(p_spray_1) as u32], false); }
    let r_ = 'f: {
    {
        'l1: loop {
            if !(((((p_spray_1) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((!(((((c.m.r32((p_spray_1).wrapping_add(132)) as i32)) & (64i32))) != 0)) as i32)) != 0) {
                    { let r_ = 64i32; let a_ = (p_spray_1).wrapping_add(132); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32((p_spray_1).wrapping_add(92))), F32(c.m.r32((p_spray_1).wrapping_add(84)))), F32(0x403fffff)); let a_ = (p_spray_1).wrapping_add(88); c.m.w32(a_, (v_).0); };
                }
            }
            p_spray_1 = c.m.r32((p_spray_1).wrapping_add(140));
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWaterSpray_FadeOutList", &PROBE_N, &[], true); }
    r_
}

pub fn EftWaterMist_Alloc(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWaterMist_Alloc", &PROBE_N, &[], false); }
    let mut l_mist_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_mist_1 = { let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(24); c.m.r32(a_) };
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (30i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32((l_mist_1).wrapping_add(148)) as i32)) == (0i32)) as i32)) != 0) {
                    { let _ = F::memset(c, l_mist_1, 0i32, 176u32); };
                    break 'f l_mist_1;
                }
            }
            { { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; }; { let o_ = l_mist_1; let n_ = (o_).wrapping_add(176); l_mist_1 = n_; }; };
        }
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWaterMist_Alloc", &PROBE_N, &[], true); }
    r_
}

pub fn EftWaterMist_Spawn(c: &mut Ctx, mut p_head_1: u32, mut p_tail_2: u32, p_pos_3: u32, mut p_radius_4: F32, mut p_height_5: F32, mut p_yaw_6: F32, mut p_yawVel_7: F32, mut p_yawDamp_8: F32, mut p_pitch_9: F32, mut p_roll_10: F32, mut p_tilt_11: F32, mut p_tiltEnd_12: F32, mut p_speed_13: F32, mut p_size_14: F32, mut p_sizeVel_15: F32, mut p_sizeDamp_16: F32, mut p_gravity_17: F32, mut p_nearScale_18: F32, mut p_widthScale_19: F32, mut p_life_20: F32, mut p_delay_21: F32, mut p_r_22: u8, mut p_g_23: u8, mut p_b_24: u8, mut p_tex_25: u8, mut p_layer_26: u8) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWaterMist_Spawn", &PROBE_N, &[(p_head_1) as u32, (p_tail_2) as u32, (p_pos_3) as u32, (p_radius_4).0, (p_height_5).0, (p_yaw_6).0, (p_yawVel_7).0, (p_yawDamp_8).0, (p_pitch_9).0, (p_roll_10).0, (p_tilt_11).0, (p_tiltEnd_12).0, (p_speed_13).0, (p_size_14).0, (p_sizeVel_15).0, (p_sizeDamp_16).0, (p_gravity_17).0, (p_nearScale_18).0, (p_widthScale_19).0, (p_life_20).0, (p_delay_21).0, (p_r_22) as u32, (p_g_23) as u32, (p_b_24) as u32, (p_tex_25) as u32, (p_layer_26) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_mist_27: u32 = 0;
    let mut l_angle_28: F32 = F32(0);
    let mut l_time_29: F32 = F32(0);
    rt::memcpy(c, fp, p_pos_3, 16);
    let r_ = 'f: {
        l_mist_27 = F::EftWaterMist_Alloc(c);
        if (((((l_mist_27) != (((0i32) as u32))) as i32)) != 0) {
            if (((((c.m.r32(p_head_1)) == (((0i32) as u32))) as i32)) != 0) {
                { let v_ = l_mist_27; let a_ = p_head_1; c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_mist_27).wrapping_add(156); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_mist_27).wrapping_add(160); c.m.w32(a_, v_); };
            } else {
                { let v_ = c.m.r32(p_tail_2); let a_ = (l_mist_27).wrapping_add(156); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_mist_27).wrapping_add(160); c.m.w32(a_, v_); };
            }
            if (((((c.m.r32(p_tail_2)) != (((0i32) as u32))) as i32)) != 0) {
                { let v_ = l_mist_27; let a_ = (c.m.r32(p_tail_2)).wrapping_add(160); c.m.w32(a_, v_); };
            }
            { let v_ = l_mist_27; let a_ = p_tail_2; c.m.w32(a_, v_); };
            { let _ = D::call(c, fid::Vec4_Set, &[((l_mist_27) as u64), ((F32(0x00000000)).0 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
            { let _ = D::call(c, fid::Vec4_Copy, &[(((l_mist_27).wrapping_add(16)) as u64), ((fp) as u64)]); };
            { let r_ = p_height_5; let a_ = ((l_mist_27).wrapping_add(16)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let v_ = p_radius_4; let a_ = (l_mist_27).wrapping_add(64); c.m.w32(a_, (v_).0); };
            { let v_ = p_yawVel_7; let a_ = (l_mist_27).wrapping_add(104); c.m.w32(a_, (v_).0); };
            { let v_ = p_yaw_6; let a_ = (l_mist_27).wrapping_add(100); c.m.w32(a_, (v_).0); };
            if (((rt::fne(p_yawDamp_8, F32(0x00000000)) as i32)) != 0) {
                { let v_ = rt::fdiv(F32(0x3f800000), p_yawDamp_8); let a_ = (l_mist_27).wrapping_add(108); c.m.w32(a_, (v_).0); };
            } else {
                { let v_ = F32(0x00000000); let a_ = (l_mist_27).wrapping_add(108); c.m.w32(a_, (v_).0); };
            }
            { let v_ = p_size_14; let a_ = (l_mist_27).wrapping_add(76); c.m.w32(a_, (v_).0); };
            { let v_ = p_tilt_11; let a_ = (l_mist_27).wrapping_add(68); c.m.w32(a_, (v_).0); };
            { let v_ = p_sizeVel_15; let a_ = (l_mist_27).wrapping_add(80); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fdiv(rt::fsub(p_tiltEnd_12, p_tilt_11), rt::fmul(p_life_20, F32(0x41f00000))); let a_ = (l_mist_27).wrapping_add(72); c.m.w32(a_, (v_).0); };
            if (((rt::fne(p_sizeDamp_16, F32(0x00000000)) as i32)) != 0) {
                { let v_ = rt::fdiv(F32(0x3f800000), p_sizeDamp_16); let a_ = (l_mist_27).wrapping_add(84); c.m.w32(a_, (v_).0); };
            } else {
                { let v_ = F32(0x00000000); let a_ = (l_mist_27).wrapping_add(84); c.m.w32(a_, (v_).0); };
            }
            { let v_ = rt::fmul(p_gravity_17, F32(0x411ce809)); let a_ = (l_mist_27).wrapping_add(116); c.m.w32(a_, (v_).0); };
            { let v_ = p_nearScale_18; let a_ = (l_mist_27).wrapping_add(120); c.m.w32(a_, (v_).0); };
            { let v_ = p_widthScale_19; let a_ = (l_mist_27).wrapping_add(124); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3d088888); let a_ = (l_mist_27).wrapping_add(132); c.m.w32(a_, (v_).0); };
            { let v_ = 1i32; let a_ = (l_mist_27).wrapping_add(148); c.m.w32(a_, (v_) as u32); };
            { let v_ = rt::fmul(p_delay_21, F32(0x41f00000)); let a_ = (l_mist_27).wrapping_add(136); c.m.w32(a_, (v_).0); };
            { let v_ = p_roll_10; let a_ = (l_mist_27).wrapping_add(112); c.m.w32(a_, (v_).0); };
            { let v_ = p_r_22; let a_ = (l_mist_27).wrapping_add(140); c.m.w8(a_, v_); };
            { let v_ = p_g_23; let a_ = (l_mist_27).wrapping_add(141); c.m.w8(a_, v_); };
            { let v_ = p_b_24; let a_ = (l_mist_27).wrapping_add(142); c.m.w8(a_, v_); };
            { let v_ = p_tex_25; let a_ = (l_mist_27).wrapping_add(145); c.m.w8(a_, v_); };
            { let v_ = p_layer_26; let a_ = (l_mist_27).wrapping_add(146); c.m.w8(a_, v_); };
            { let v_ = F32(0x00000000); let a_ = (l_mist_27).wrapping_add(128); c.m.w32(a_, (v_).0); };
            { let v_ = ((0i32) as u8); let a_ = (l_mist_27).wrapping_add(143); c.m.w8(a_, v_); };
            { let v_ = ((0i32) as u8); let a_ = (l_mist_27).wrapping_add(144); c.m.w8(a_, v_); };
            { let v_ = 0i32; let a_ = (l_mist_27).wrapping_add(152); c.m.w32(a_, (v_) as u32); };
            l_angle_28 = F::EftMath_WrapAngle(c, rt::fdiv(rt::fmul(p_yaw_6, F32(0x40490fda)), F32(0x43340000)));
            { let v_ = F::Mathf_SinFast(c, l_angle_28); let a_ = (l_mist_27).wrapping_add(32); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = ((l_mist_27).wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = F::Mathf_CosFast(c, l_angle_28); let a_ = ((l_mist_27).wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            l_angle_28 = rt::fdiv(rt::fmul(p_pitch_9, F32(0x40490fda)), F32(0x43340000));
            { let v_ = F32(0x3f800000); let a_ = ((l_mist_27).wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            { let _ = D::call(c, fid::Vec3_Normalize, &[(((l_mist_27).wrapping_add(32)) as u64), (((l_mist_27).wrapping_add(32)) as u64)]); };
            { let v_ = rt::fmul(p_speed_13, F::Mathf_CosFast(c, l_angle_28)); let a_ = (l_mist_27).wrapping_add(48); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul(rt::fneg(p_speed_13), F::Mathf_SinFast(c, l_angle_28)); let a_ = ((l_mist_27).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul(p_speed_13, F::Mathf_CosFast(c, l_angle_28)); let a_ = ((l_mist_27).wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (l_mist_27).wrapping_add(88); c.m.w32(a_, (v_).0); };
            l_time_29 = rt::fmul(p_life_20, F32(0x3fcccccc));
            { let v_ = F32(0x42c00000); let a_ = (l_mist_27).wrapping_add(92); c.m.w32(a_, (v_).0); };
            if (((rt::fne(l_time_29, F32(c.m.r32((l_mist_27).wrapping_add(88)))) as i32)) != 0) {
                { let v_ = rt::fdiv(F32(0x42c00000), rt::fmul(l_time_29, F32(0x41f00000))); let a_ = (l_mist_27).wrapping_add(92); c.m.w32(a_, (v_).0); };
            }
            { let v_ = F32(0x42c00000); let a_ = (l_mist_27).wrapping_add(96); c.m.w32(a_, (v_).0); };
            { let r_ = 1i32; let a_ = (l_mist_27).wrapping_add(144); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
            { let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(56); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftWaterMist_Spawn", &PROBE_N, &[], true); }
    r_
}

pub fn EftWaterMist_UpdateList(c: &mut Ctx, mut p_head_1: u32, mut p_tail_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWaterMist_UpdateList", &PROBE_N, &[(p_head_1) as u32, (p_tail_2) as u32], false); }
    let mut l_next_3: u32 = 0;
    let mut l_mist_4: u32 = 0;
    let mut l_done_5: i32 = 0;
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(56); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
        if ((((!(((({ let a_ = (((D::call(c, fid::Battle_GetWork, &[])) as u32)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
            l_next_3 = c.m.r32(p_head_1);
            'l1: while (((((l_next_3) != (((0i32) as u32))) as i32)) != 0) {
                l_mist_4 = l_next_3;
                l_done_5 = F::EftWaterMist_Update(c, l_next_3);
                l_next_3 = c.m.r32((l_next_3).wrapping_add(160));
                if ((l_done_5) != 0) {
                    { let v_ = 0i32; let a_ = (l_mist_4).wrapping_add(148); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (l_mist_4).wrapping_add(152); c.m.w32(a_, (v_) as u32); };
                    if (((((c.m.r32((l_mist_4).wrapping_add(156))) == (((0i32) as u32))) as i32)) != 0) {
                        { let v_ = c.m.r32((l_mist_4).wrapping_add(160)); let a_ = p_head_1; c.m.w32(a_, v_); };
                    } else {
                        { let v_ = c.m.r32((l_mist_4).wrapping_add(160)); let a_ = (c.m.r32((l_mist_4).wrapping_add(156))).wrapping_add(160); c.m.w32(a_, v_); };
                    }
                    if (((((c.m.r32((l_mist_4).wrapping_add(160))) == (((0i32) as u32))) as i32)) != 0) {
                        { let v_ = c.m.r32((l_mist_4).wrapping_add(156)); let a_ = p_tail_2; c.m.w32(a_, v_); };
                    } else {
                        { let v_ = c.m.r32((l_mist_4).wrapping_add(156)); let a_ = (c.m.r32((l_mist_4).wrapping_add(160))).wrapping_add(156); c.m.w32(a_, v_); };
                    }
                    { let v_ = ((0i32) as u32); let a_ = (l_mist_4).wrapping_add(156); c.m.w32(a_, v_); };
                    { let v_ = ((0i32) as u32); let a_ = (l_mist_4).wrapping_add(160); c.m.w32(a_, v_); };
                    { let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(56); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWaterMist_UpdateList", &PROBE_N, &[], true); }
    r_
}

pub fn EftWaterMist_Update(c: &mut Ctx, mut p_mist_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWaterMist_Update", &PROBE_N, &[(p_mist_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_angle_2: F32 = F32(0);
    let r_ = 'f: {
        's1: {
            let sv3: i32 = ((c.m.r8((p_mist_1).wrapping_add(143))) as i32);
            'k3: {
            'k2: {
                match sv3 { 0i32 => break 'k2, 1i32 => break 'k3, _ => break 's1, }
            }
            { let r_ = F32(0x3f800000); let a_ = (p_mist_1).wrapping_add(136); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::fle(F32(c.m.r32((p_mist_1).wrapping_add(136))), F32(0x00000000)) as i32)) != 0) {
                { let v_ = F32(0x00000000); let a_ = (p_mist_1).wrapping_add(136); c.m.w32(a_, (v_).0); };
                { let v_ = 1i32; let a_ = (p_mist_1).wrapping_add(152); c.m.w32(a_, (v_) as u32); };
                { let v_ = ((1i32) as u8); let a_ = (p_mist_1).wrapping_add(143); c.m.w8(a_, v_); };
            } else {
                break 's1;
            }
            }
            { let a_ = (p_mist_1).wrapping_add(143); let o_ = c.m.r8(a_); let n_ = (o_).wrapping_add(1); c.m.w8(a_, n_); };
            break 's1;
        }
        if (((c.m.r32((p_mist_1).wrapping_add(152)) as i32)) != 0) {
            l_angle_2 = { let a0_ = rt::fdiv(rt::fmul(F32(c.m.r32((p_mist_1).wrapping_add(100))), F32(0x40490fda)), F32(0x43340000)); F::EftMath_WrapAngle(c, a0_) };
            { let v_ = F::Mathf_SinFast(c, l_angle_2); let a_ = (p_mist_1).wrapping_add(32); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = ((p_mist_1).wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = F::Mathf_CosFast(c, l_angle_2); let a_ = ((p_mist_1).wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f800000); let a_ = ((p_mist_1).wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            { let r_ = F32(c.m.r32((p_mist_1).wrapping_add(104))); let a_ = (p_mist_1).wrapping_add(100); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = rt::fmul(F32(c.m.r32((p_mist_1).wrapping_add(104))), F32(c.m.r32((p_mist_1).wrapping_add(108)))); let a_ = (p_mist_1).wrapping_add(104); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let a2_ = F32(c.m.r32((p_mist_1).wrapping_add(64))); F::Vec4_Scale(c, fp, (p_mist_1).wrapping_add(32), a2_) };
            { let _ = D::call(c, fid::Vec4_Add, &[((fp.wrapping_add(16)) as u64), (((p_mist_1).wrapping_add(16)) as u64), ((fp) as u64)]); };
            { let v_ = rt::fadd(F32(c.m.r32(fp.wrapping_add(16))), rt::fmul(rt::fmul(F32(c.m.r32((p_mist_1).wrapping_add(32))), F32(c.m.r32((p_mist_1).wrapping_add(48)))), F32(c.m.r32((p_mist_1).wrapping_add(128))))); let a_ = p_mist_1; c.m.w32(a_, (v_).0); };
            { let v_ = rt::fadd(rt::fadd(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), rt::fmul(F32(c.m.r32(((p_mist_1).wrapping_add(48)).wrapping_add(4))), F32(c.m.r32((p_mist_1).wrapping_add(128))))), rt::fmul(rt::fmul(F32(c.m.r32((p_mist_1).wrapping_add(116))), F32(c.m.r32((p_mist_1).wrapping_add(128)))), F32(c.m.r32((p_mist_1).wrapping_add(128))))); let a_ = (p_mist_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fadd(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), rt::fmul(rt::fmul(F32(c.m.r32(((p_mist_1).wrapping_add(32)).wrapping_add(8))), F32(c.m.r32(((p_mist_1).wrapping_add(48)).wrapping_add(8)))), F32(c.m.r32((p_mist_1).wrapping_add(128))))); let a_ = (p_mist_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            { let r_ = F32(c.m.r32((p_mist_1).wrapping_add(80))); let a_ = (p_mist_1).wrapping_add(76); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = rt::fmul(F32(c.m.r32((p_mist_1).wrapping_add(80))), F32(c.m.r32((p_mist_1).wrapping_add(84)))); let a_ = (p_mist_1).wrapping_add(80); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = F32(c.m.r32((p_mist_1).wrapping_add(72))); let a_ = (p_mist_1).wrapping_add(68); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = F32(c.m.r32((p_mist_1).wrapping_add(132))); let a_ = (p_mist_1).wrapping_add(128); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if ((((((((c.m.r8((p_mist_1).wrapping_add(144))) as i32)) & (1i32))) as u8)) != 0) {
                { let r_ = F32(c.m.r32((p_mist_1).wrapping_add(92))); let a_ = (p_mist_1).wrapping_add(88); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::fgt(F32(c.m.r32((p_mist_1).wrapping_add(92))), F32(0x00000000)) as i32)) != 0) {
                    if (((rt::fgt(F32(c.m.r32((p_mist_1).wrapping_add(88))), F32(c.m.r32((p_mist_1).wrapping_add(96)))) as i32)) != 0) {
                        { let v_ = F32(c.m.r32((p_mist_1).wrapping_add(96))); let a_ = (p_mist_1).wrapping_add(88); c.m.w32(a_, (v_).0); };
                        { let r_ = (!(1i32)); let a_ = (p_mist_1).wrapping_add(144); let n_ = ((((((c.m.r8(a_)) as i32)) & (r_))) as u8); c.m.w8(a_, n_); };
                    }
                } else {
                    if (((rt::flt(F32(c.m.r32((p_mist_1).wrapping_add(88))), F32(c.m.r32((p_mist_1).wrapping_add(96)))) as i32)) != 0) {
                        { let v_ = F32(c.m.r32((p_mist_1).wrapping_add(96))); let a_ = (p_mist_1).wrapping_add(88); c.m.w32(a_, (v_).0); };
                        { let r_ = (!(1i32)); let a_ = (p_mist_1).wrapping_add(144); let n_ = ((((((c.m.r8(a_)) as i32)) & (r_))) as u8); c.m.w8(a_, n_); };
                    }
                }
            }
            if ((((!((((((((c.m.r8((p_mist_1).wrapping_add(144))) as i32)) & (1i32))) as u8)) != 0)) as i32)) != 0) {
                break 'f 1i32;
            }
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftWaterMist_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftWaterMist_DrawList(c: &mut Ctx, mut p_mist_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWaterMist_DrawList", &PROBE_N, &[(p_mist_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(160) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 160);
    let r_ = 'f: {
        rt::memset(c, fp.wrapping_add(128), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(128).wrapping_add(0), (v_).0); }
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(128).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(128).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(128).wrapping_add(12), (v_).0); }
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Mtx_Copy(c, fp.wrapping_add(64), a1_) };
        {
            'l1: loop {
                if !(((((p_mist_1) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((c.m.r32((p_mist_1).wrapping_add(152)) as i32)) != 0) {
                        { let a3_ = F32(c.m.r32((p_mist_1).wrapping_add(68))); F::Vec3_Lerp(c, fp.wrapping_add(144), fp.wrapping_add(128), (p_mist_1).wrapping_add(32), a3_) };
                        F::EftUtil_MakeFacingMtx(c, fp, fp.wrapping_add(144), sym::D_002C3630);
                        { let a3_ = F32(c.m.r32((p_mist_1).wrapping_add(112))); let a4_ = F32(c.m.r32((p_mist_1).wrapping_add(76))); let a5_ = F32(c.m.r32((p_mist_1).wrapping_add(120))); let a7_ = F32(c.m.r32((p_mist_1).wrapping_add(124))); let a8_ = c.m.r8((p_mist_1).wrapping_add(140)); let a9_ = c.m.r8((p_mist_1).wrapping_add(141)); let a10_ = c.m.r8((p_mist_1).wrapping_add(142)); let a12_ = ((rt::f2u(F32(c.m.r32((p_mist_1).wrapping_add(88))))) as u8); let a13_ = ((c.m.r32(sym::gEftDust)).wrapping_add(176)).wrapping_add((((c.m.r8((p_mist_1).wrapping_add(145))) as u32)).wrapping_mul(16)); let a14_ = c.m.r8((p_mist_1).wrapping_add(146)); F::EftWater_DrawSprayQuad(c, p_mist_1, fp, fp.wrapping_add(64), a3_, a4_, a5_, F32(0x00000000), a7_, a8_, a9_, a10_, ((64i32) as u8), a12_, a13_, a14_) };
                    }
                }
                p_mist_1 = c.m.r32((p_mist_1).wrapping_add(160));
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftWaterMist_DrawList", &PROBE_N, &[], true); }
    r_
}

pub fn EftWater_IsOnScreen(c: &mut Ctx, p_v_1: u32, mut p_margin_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWater_IsOnScreen", &PROBE_N, &[(p_v_1) as u32, (p_margin_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    rt::memcpy(c, fp, p_v_1, 16);
    let r_ = 'f: {
        's1: {
            let sv3: i32 = p_margin_2;
            match sv3 {
                0i32 => {
                    if ((((((c.m.r32((fp).wrapping_add(8)) as i32)) <= (0i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if ((((((c.m.r32(fp) as i32)) > (36863i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if ((((((c.m.r32(fp) as i32)) <= (28672i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if ((((((c.m.r32((fp).wrapping_add(4)) as i32)) > (36351i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if ((((((c.m.r32((fp).wrapping_add(4)) as i32)) <= (29184i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    break 's1;
                }
                1i32 => {
                    if ((((((c.m.r32((fp).wrapping_add(8)) as i32)) <= (0i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if ((((((c.m.r32(fp) as i32)) > (37887i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if ((((((c.m.r32(fp) as i32)) <= (27648i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if ((((((c.m.r32((fp).wrapping_add(4)) as i32)) > (37247i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if ((((((c.m.r32((fp).wrapping_add(4)) as i32)) <= (28288i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    break 's1;
                }
                2i32 => {
                    if ((((((c.m.r32((fp).wrapping_add(8)) as i32)) <= (0i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if ((((((c.m.r32(fp) as i32)) > (38911i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if ((((((c.m.r32(fp) as i32)) <= (26624i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if ((((((c.m.r32((fp).wrapping_add(4)) as i32)) > (38143i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if ((((((c.m.r32((fp).wrapping_add(4)) as i32)) <= (27392i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    break 's1;
                }
                3i32 => {
                    if ((((((c.m.r32((fp).wrapping_add(8)) as i32)) <= (0i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if ((((((c.m.r32(fp) as i32)) > (40959i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if ((((((c.m.r32(fp) as i32)) <= (24576i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if ((((((c.m.r32((fp).wrapping_add(4)) as i32)) > (39935i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if ((((((c.m.r32((fp).wrapping_add(4)) as i32)) <= (25600i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    break 's1;
                }
                _ => {
                    if ((((((c.m.r32((fp).wrapping_add(8)) as i32)) <= (0i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if ((((((c.m.r32(fp) as i32)) > (65519i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if ((((((c.m.r32(fp) as i32)) <= (0i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if ((((((c.m.r32((fp).wrapping_add(4)) as i32)) > (65519i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if ((((((c.m.r32((fp).wrapping_add(4)) as i32)) <= (0i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    break 's1;
                }
            }
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftWater_IsOnScreen", &PROBE_N, &[], true); }
    r_
}

pub fn EftWater_DrawBillboard(c: &mut Ctx, mut p_pos_1: u32, mut p_world2screen_2: u32, mut p_size_3: F32, mut p_rot_4: F32, mut p_r_5: u8, mut p_g_6: u8, mut p_b_7: u8, mut p_a_8: u8, mut p_alphaRef_9: u8, mut p_tex_10: u32, mut p_layer_11: u8) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWater_DrawBillboard", &PROBE_N, &[(p_pos_1) as u32, (p_world2screen_2) as u32, (p_size_3).0, (p_rot_4).0, (p_r_5) as u32, (p_g_6) as u32, (p_b_7) as u32, (p_a_8) as u32, (p_alphaRef_9) as u32, (p_tex_10) as u32, (p_layer_11) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(256) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 256);
    let mut l_p_12: u32 = 0;
    let mut l_half_13: F32 = F32(0);
    let mut l_z_14: i32 = 0;
    let mut l_i_15: i32 = 0;
    let mut l_q_16: u32 = 0;
    let mut l_d_17: u32 = 0;
    let mut l_q_18: u32 = 0;
    let mut l_d_19: u32 = 0;
    let r_ = 'f: {
        { let a1_ = (c.m.r32(sym::gEftDust)).wrapping_add(112); F::Mtx_Copy(c, fp, a1_) };
        if (((rt::fne(p_rot_4, F32(0x00000000)) as i32)) != 0) {
            F::Mtx_StoreIdentity(c, fp.wrapping_add(192));
            { let a2_ = F::EftMath_WrapAngle(c, rt::fdiv(rt::fmul(p_rot_4, F32(0x40490fda)), F32(0x43340000))); F::Mtx_RotateZ(c, fp.wrapping_add(192), fp.wrapping_add(192), a2_) };
            F::Mtx_Mul(c, fp, fp, fp.wrapping_add(192));
        }
        l_half_13 = rt::fmul(p_size_3, F32(0x3f000000));
        { let _ = D::call(c, fid::Vec4_Copy, &[(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as u64), ((p_pos_1) as u64)]); };
        { let _ = D::call(c, fid::Vec4_Set, &[(((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as u64), ((rt::fneg(l_half_13)).0 as u64), ((rt::fneg(l_half_13)).0 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
        { let _ = D::call(c, fid::Vec4_Set, &[(((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as u64), ((l_half_13).0 as u64), ((rt::fneg(l_half_13)).0 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
        { let _ = D::call(c, fid::Vec4_Set, &[(((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as u64), ((rt::fneg(l_half_13)).0 as u64), ((l_half_13).0 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
        { let _ = D::call(c, fid::Vec4_Set, &[(((fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as u64), ((l_half_13).0 as u64), ((l_half_13).0 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
        {
            l_i_15 = 0i32;
            'l1: loop {
                if !(((((l_i_15) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let _ = D::call(c, fid::Mtx_MulVec4, &[(((fp.wrapping_add(64)).wrapping_add((((l_i_15) as u32)).wrapping_mul(16))) as u64), ((fp) as u64), (((fp.wrapping_add(64)).wrapping_add((((l_i_15) as u32)).wrapping_mul(16))) as u64)]); };
                    { let _ = F::Mtx_ProjectPoint(c, (fp.wrapping_add(128)).wrapping_add((((l_i_15) as u32)).wrapping_mul(16)), p_world2screen_2, (fp.wrapping_add(64)).wrapping_add((((l_i_15) as u32)).wrapping_mul(16))); };
                    if ((((!((F::EftWater_IsOnScreen(c, (fp.wrapping_add(128)).wrapping_add((((l_i_15) as u32)).wrapping_mul(16)), 2i32)) != 0)) as i32)) != 0) {
                        break 'f;
                    }
                }
                { let o_ = l_i_15; let n_ = (o_).wrapping_add(1); l_i_15 = n_; };
            }
        }
        l_p_12 = c.m.r32(sym::gOtCur);
        { let v_ = (l_p_12).wrapping_add((((1i32) as u32)).wrapping_mul(128)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
        if (((((l_p_12) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        { let v_ = ((536870919i32) as u32); let a_ = l_p_12; c.m.w32(a_, v_); };
        { let v_ = ((84i32) as u64); let a_ = (l_p_12).wrapping_add(32); c.m.w64(a_, v_); };
        { let v_ = ((1342177287i32) as u32); let a_ = (l_p_12).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = 14123288431433908225u64; let a_ = (l_p_12).wrapping_add(16); c.m.w64(a_, v_); };
        { let v_ = ((268436062347616i64) as u64); let a_ = (l_p_12).wrapping_add(24); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (l_p_12).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = ((268435456i32) as u32); let a_ = (l_p_12).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = p_r_5; let a_ = ((l_p_12).wrapping_add(48)).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = p_g_6; let a_ = ((l_p_12).wrapping_add(48)).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = p_b_7; let a_ = ((l_p_12).wrapping_add(48)).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = p_a_8; let a_ = ((l_p_12).wrapping_add(48)).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = F32(0x3f800000); let a_ = (l_p_12).wrapping_add(52); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((l_p_12).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (((l_p_12).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((l_p_12).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (((l_p_12).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((l_p_12).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (((l_p_12).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((l_p_12).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (((l_p_12).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = (((c.m.r32((fp.wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_12).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_12).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_12).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_12).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32((fp.wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_12).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_12).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_12).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_12).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32((fp.wrapping_add(128)).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_12).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(128)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_12).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(128)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_12).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_12).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32((fp.wrapping_add(128)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_12).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(128)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_12).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(128)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_12).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_12).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        l_z_14 = (((((c.m.r32(((fp.wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)).wrapping_add((c.m.r32(((fp.wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add((c.m.r32(((fp.wrapping_add(128)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add((c.m.r32(((fp.wrapping_add(128)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_shr((10i32) as u32);
        if ((F::EftUtil_IsCamUnderWater(c)) != 0) {
            { let r_ = 500i32; let n_ = (l_z_14).wrapping_sub(r_); l_z_14 = n_; };
        } else {
            { let r_ = 500i32; let n_ = (l_z_14).wrapping_add(r_); l_z_14 = n_; };
        }
        { let v_ = c.m.r64(p_tex_10); let a_ = (l_p_12).wrapping_add(40); c.m.w64(a_, v_); };
        if (((((((p_alphaRef_9) as i32)) != (0i32)) as i32)) != 0) {
            {
                l_q_16 = c.m.r32(sym::gOtCur);
                { let v_ = (l_q_16).wrapping_add((((1i32) as u32)).wrapping_mul(48)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
                l_d_17 = (l_q_16).wrapping_add(16);
                if (((((l_q_16) == (((0i32) as u32))) as i32)) != 0) {
                    break 'f;
                }
                { let v_ = ((536870914i32) as u32); let a_ = l_q_16; c.m.w32(a_, v_); };
                { let v_ = ((268435456i32) as u32); let a_ = (l_q_16).wrapping_add(8); c.m.w32(a_, v_); };
                { let v_ = ((1342177282i32) as u32); let a_ = (l_q_16).wrapping_add(12); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_q_16).wrapping_add(4); c.m.w32(a_, v_); };
                { let v_ = ((1152921504606879745i64) as u64); let a_ = l_d_17; c.m.w64(a_, v_); };
                { let v_ = ((14i32) as u64); let a_ = (l_q_16).wrapping_add(24); c.m.w64(a_, v_); };
                { let v_ = (((((p_alphaRef_9) as u64)).wrapping_shl((4i32) as u32)) | (((327693i32) as u64))); let a_ = (l_q_16).wrapping_add(32); c.m.w64(a_, v_); };
                { let v_ = ((71i32) as u64); let a_ = (l_q_16).wrapping_add(40); c.m.w64(a_, v_); };
                F::eft_e__EftWaterOt_Add(c, l_q_16, l_z_14, ((p_layer_11) as i32));
            }
        }
        F::eft_e__EftWaterOt_Add(c, l_p_12, l_z_14, ((p_layer_11) as i32));
        if (((((((p_alphaRef_9) as i32)) != (0i32)) as i32)) != 0) {
            {
                l_q_18 = c.m.r32(sym::gOtCur);
                { let v_ = (l_q_18).wrapping_add((((1i32) as u32)).wrapping_mul(48)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
                l_d_19 = (l_q_18).wrapping_add(16);
                if (((((l_q_18) == (((0i32) as u32))) as i32)) != 0) {
                    break 'f;
                }
                { let v_ = ((536870914i32) as u32); let a_ = l_q_18; c.m.w32(a_, v_); };
                { let v_ = ((268435456i32) as u32); let a_ = (l_q_18).wrapping_add(8); c.m.w32(a_, v_); };
                { let v_ = ((1342177282i32) as u32); let a_ = (l_q_18).wrapping_add(12); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_q_18).wrapping_add(4); c.m.w32(a_, v_); };
                { let v_ = ((1152921504606879745i64) as u64); let a_ = l_d_19; c.m.w64(a_, v_); };
                { let v_ = ((14i32) as u64); let a_ = (l_q_18).wrapping_add(24); c.m.w64(a_, v_); };
                { let v_ = ((327680i32) as u64); let a_ = (l_q_18).wrapping_add(32); c.m.w64(a_, v_); };
                { let v_ = ((71i32) as u64); let a_ = (l_q_18).wrapping_add(40); c.m.w64(a_, v_); };
                F::eft_e__EftWaterOt_Add(c, l_q_18, l_z_14, ((p_layer_11) as i32));
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftWater_DrawBillboard", &PROBE_N, &[], true); }
    r_
}

pub fn EftWater_DrawGroundQuad(c: &mut Ctx, mut p_pos_1: u32, mut p_world2screen_2: u32, mut p_size_3: F32, mut p_rot_4: F32, mut p_r_5: u8, mut p_g_6: u8, mut p_b_7: u8, mut p_a_8: u8, mut p_tex_9: u32, mut p_layer_10: u8) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWater_DrawGroundQuad", &PROBE_N, &[(p_pos_1) as u32, (p_world2screen_2) as u32, (p_size_3).0, (p_rot_4).0, (p_r_5) as u32, (p_g_6) as u32, (p_b_7) as u32, (p_a_8) as u32, (p_tex_9) as u32, (p_layer_10) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(688) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 688);
    let mut l_half_11: F32 = F32(0);
    let mut l_i_12: i32 = 0;
    let r_ = 'f: {
        F::Mtx_StoreIdentity(c, fp.wrapping_add(624));
        if (((rt::fne(p_rot_4, F32(0x00000000)) as i32)) != 0) {
            { let a2_ = F::EftMath_WrapAngle(c, rt::fdiv(rt::fmul(p_rot_4, F32(0x40490fda)), F32(0x43340000))); F::Mtx_RotateY(c, fp.wrapping_add(624), fp.wrapping_add(624), a2_) };
        }
        l_half_11 = rt::fmul(p_size_3, F32(0x3f000000));
        { let _ = D::call(c, fid::Vec4_Set, &[(((fp.wrapping_add(432)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as u64), ((rt::fneg(l_half_11)).0 as u64), ((F32(0x00000000)).0 as u64), ((rt::fneg(l_half_11)).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
        { let _ = D::call(c, fid::Vec4_Set, &[(((fp.wrapping_add(432)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as u64), ((l_half_11).0 as u64), ((F32(0x00000000)).0 as u64), ((rt::fneg(l_half_11)).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
        { let _ = D::call(c, fid::Vec4_Set, &[(((fp.wrapping_add(432)).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as u64), ((rt::fneg(l_half_11)).0 as u64), ((F32(0x00000000)).0 as u64), ((l_half_11).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
        { let _ = D::call(c, fid::Vec4_Set, &[(((fp.wrapping_add(432)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as u64), ((l_half_11).0 as u64), ((F32(0x00000000)).0 as u64), ((l_half_11).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
        {
            l_i_12 = 0i32;
            'l1: loop {
                if !(((((l_i_12) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let _ = D::call(c, fid::Mtx_MulVec4, &[(((fp.wrapping_add(432)).wrapping_add((((l_i_12) as u32)).wrapping_mul(16))) as u64), ((fp.wrapping_add(624)) as u64), (((fp.wrapping_add(432)).wrapping_add((((l_i_12) as u32)).wrapping_mul(16))) as u64)]); };
                    { let _ = D::call(c, fid::Vec4_Add, &[(((fp.wrapping_add(432)).wrapping_add((((l_i_12) as u32)).wrapping_mul(16))) as u64), (((fp.wrapping_add(432)).wrapping_add((((l_i_12) as u32)).wrapping_mul(16))) as u64), ((p_pos_1) as u64)]); };
                    { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(432)).wrapping_add((((l_i_12) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_12; let n_ = (o_).wrapping_add(1); l_i_12 = n_; };
            }
        }
        { let _ = D::call(c, fid::Vec4_Set, &[(((fp.wrapping_add(496)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as u64), ((F32(0x00000000)).0 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f800000)).0 as u64), ((F32(0x00000000)).0 as u64)]); };
        { let _ = D::call(c, fid::Vec4_Set, &[(((fp.wrapping_add(496)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as u64), ((F32(0x3f800000)).0 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f800000)).0 as u64), ((F32(0x00000000)).0 as u64)]); };
        { let _ = D::call(c, fid::Vec4_Set, &[(((fp.wrapping_add(496)).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f800000)).0 as u64), ((F32(0x3f800000)).0 as u64), ((F32(0x00000000)).0 as u64)]); };
        { let _ = D::call(c, fid::Vec4_Set, &[(((fp.wrapping_add(496)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as u64), ((F32(0x3f800000)).0 as u64), ((F32(0x3f800000)).0 as u64), ((F32(0x3f800000)).0 as u64), ((F32(0x00000000)).0 as u64)]); };
        { let _ = D::call(c, fid::Vec4_Set, &[(((fp.wrapping_add(560)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as u64), ((rt::i2f((p_r_5) as i32)).0 as u64), ((rt::i2f((p_g_6) as i32)).0 as u64), ((rt::i2f((p_b_7) as i32)).0 as u64), ((rt::i2f((p_a_8) as i32)).0 as u64)]); };
        { let _ = D::call(c, fid::Vec4_Set, &[(((fp.wrapping_add(560)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as u64), ((rt::i2f((p_r_5) as i32)).0 as u64), ((rt::i2f((p_g_6) as i32)).0 as u64), ((rt::i2f((p_b_7) as i32)).0 as u64), ((rt::i2f((p_a_8) as i32)).0 as u64)]); };
        { let _ = D::call(c, fid::Vec4_Set, &[(((fp.wrapping_add(560)).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as u64), ((rt::i2f((p_r_5) as i32)).0 as u64), ((rt::i2f((p_g_6) as i32)).0 as u64), ((rt::i2f((p_b_7) as i32)).0 as u64), ((rt::i2f((p_a_8) as i32)).0 as u64)]); };
        { let _ = D::call(c, fid::Vec4_Set, &[(((fp.wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as u64), ((rt::i2f((p_r_5) as i32)).0 as u64), ((rt::i2f((p_g_6) as i32)).0 as u64), ((rt::i2f((p_b_7) as i32)).0 as u64), ((rt::i2f((p_a_8) as i32)).0 as u64)]); };
        F::ClipVtx_SetArray(c, fp, (fp.wrapping_add(432)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(496)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(560)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), 3i32);
        { let a2_ = c.m.r64(p_tex_9); F::EftWater_DrawClippedFan(c, fp, ((p_layer_10) as i32), a2_) };
        F::ClipVtx_SetArray(c, fp, (fp.wrapping_add(432)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(496)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(560)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), 3i32);
        { let a2_ = c.m.r64(p_tex_9); F::EftWater_DrawClippedFan(c, fp, ((p_layer_10) as i32), a2_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftWater_DrawGroundQuad", &PROBE_N, &[], true); }
    r_
}

pub fn EftWater_DrawSprayQuad(c: &mut Ctx, mut p_pos_1: u32, mut p_orient_2: u32, mut p_world2screen_3: u32, mut p_roll_4: F32, mut p_size_5: F32, mut p_near_6: F32, mut p_far_7: F32, mut p_width_8: F32, mut p_r_9: u8, mut p_g_10: u8, mut p_b_11: u8, mut p_a_12: u8, mut p_alphaRef_13: u8, mut p_tex_14: u32, mut p_layer_15: u8) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWater_DrawSprayQuad", &PROBE_N, &[(p_pos_1) as u32, (p_orient_2) as u32, (p_world2screen_3) as u32, (p_roll_4).0, (p_size_5).0, (p_near_6).0, (p_far_7).0, (p_width_8).0, (p_r_9) as u32, (p_g_10) as u32, (p_b_11) as u32, (p_a_12) as u32, (p_alphaRef_13) as u32, (p_tex_14) as u32, (p_layer_15) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(320) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 320);
    let mut l_p_16: u32 = 0;
    let mut l_c_17: u32 = 0;
    let mut l_zp_18: u32 = 0;
    let mut l_zNear_19: F32 = F32(0);
    let mut l_half_20: F32 = F32(0);
    let mut l_mid_21: F32 = F32(0);
    let mut l_z_22: i32 = 0;
    let mut l_i_23: i32 = 0;
    let mut l_q_24: u32 = 0;
    let mut l_d_25: u32 = 0;
    let mut l_q_26: u32 = 0;
    let mut l_d_27: u32 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
        'fl1: loop {
            match bb_ {
                0 => {
                    l_half_20 = rt::fmul(p_size_5, F32(0x3f000000));
                    l_zNear_19 = rt::fmul(p_size_5, p_near_6);
                    p_size_5 = rt::fmul(p_size_5, p_far_7);
                    { let r_ = p_width_8; let n_ = rt::fmul(l_half_20, r_); l_half_20 = n_; };
                    { let _ = D::call(c, fid::Vec4_Set, &[(((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as u64), ((rt::fneg(l_half_20)).0 as u64), ((F32(0x00000000)).0 as u64), ((l_zNear_19).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
                    { let _ = D::call(c, fid::Vec4_Set, &[(((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as u64), ((l_half_20).0 as u64), ((F32(0x00000000)).0 as u64), ((l_zNear_19).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
                    { let _ = D::call(c, fid::Vec4_Set, &[(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as u64), ((rt::fneg(l_half_20)).0 as u64), ((F32(0x00000000)).0 as u64), ((p_size_5).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
                    { let _ = D::call(c, fid::Vec4_Set, &[(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as u64), ((l_half_20).0 as u64), ((F32(0x00000000)).0 as u64), ((p_size_5).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
                    if (((rt::fne(p_roll_4, F32(0x00000000)) as i32)) != 0) { bb_ = 1; continue 'fl1; } else { bb_ = 2; continue 'fl1; }
                }
                1 => {
                    l_mid_21 = rt::fmul(rt::fsub(F32(c.m.r32(((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8))), F32(c.m.r32(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)))), F32(0x3f000000));
                    F::Mtx_StoreIdentity(c, fp.wrapping_add(256));
                    { let a2_ = F::EftMath_WrapAngle(c, rt::fdiv(rt::fmul(p_roll_4, F32(0x40490fda)), F32(0x43340000))); F::Mtx_RotateY(c, fp.wrapping_add(256), fp.wrapping_add(256), a2_) };
                    l_zp_18 = ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8);
                    l_c_17 = fp;
                    l_i_23 = 0i32;
                    bb_ = 3; continue 'fl1;
                }
                2 => {
                    { let _ = D::call(c, fid::Vec4_Set, &[(((fp.wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as u64), ((F32(0x00000000)).0 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f800000)).0 as u64), ((F32(0x00000000)).0 as u64)]); };
                    { let _ = D::call(c, fid::Vec4_Set, &[(((fp.wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as u64), ((F32(0x3f800000)).0 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f800000)).0 as u64), ((F32(0x00000000)).0 as u64)]); };
                    { let _ = D::call(c, fid::Vec4_Set, &[(((fp.wrapping_add(128)).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f800000)).0 as u64), ((F32(0x3f800000)).0 as u64), ((F32(0x00000000)).0 as u64)]); };
                    { let _ = D::call(c, fid::Vec4_Set, &[(((fp.wrapping_add(128)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as u64), ((F32(0x3f800000)).0 as u64), ((F32(0x3f800000)).0 as u64), ((F32(0x3f800000)).0 as u64), ((F32(0x00000000)).0 as u64)]); };
                    l_i_23 = 0i32;
                    bb_ = 7; continue 'fl1;
                }
                3 => {
                    { let r_ = l_mid_21; let a_ = l_zp_18; let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let _ = D::call(c, fid::Mtx_MulVec4, &[((l_c_17) as u64), ((fp.wrapping_add(256)) as u64), ((l_c_17) as u64)]); };
                    { let r_ = l_mid_21; let a_ = l_zp_18; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let o_ = l_c_17; let n_ = (o_).wrapping_add(16); l_c_17 = n_; };
                    { let r_ = 4i32; let n_ = (l_zp_18).wrapping_add((((r_) as u32)).wrapping_mul(4)); l_zp_18 = n_; };
                    { let o_ = l_i_23; let n_ = (o_).wrapping_add(1); l_i_23 = n_; };
                    if (((((l_i_23) < (4i32)) as i32)) != 0) { bb_ = 4; continue 'fl1; } else { bb_ = 5; continue 'fl1; }
                }
                4 => {
                    bb_ = 3; continue 'fl1;
                }
                5 => {
                    bb_ = 2; continue 'fl1;
                }
                6 => {
                    bb_ = 5; continue 'fl1;
                }
                7 => {
                    if (((((l_i_23) < (4i32)) as i32)) != 0) { bb_ = 8; continue 'fl1; } else { bb_ = 10; continue 'fl1; }
                }
                8 => {
                    { let _ = D::call(c, fid::Mtx_MulVec4, &[(((fp).wrapping_add((((l_i_23) as u32)).wrapping_mul(16))) as u64), ((p_orient_2) as u64), (((fp).wrapping_add((((l_i_23) as u32)).wrapping_mul(16))) as u64)]); };
                    { let _ = D::call(c, fid::Vec4_Add, &[(((fp).wrapping_add((((l_i_23) as u32)).wrapping_mul(16))) as u64), (((fp).wrapping_add((((l_i_23) as u32)).wrapping_mul(16))) as u64), ((p_pos_1) as u64)]); };
                    { let v_ = F32(0x3f800000); let a_ = ((fp).wrapping_add((((l_i_23) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    bb_ = 9; continue 'fl1;
                }
                9 => {
                    { let o_ = l_i_23; let n_ = (o_).wrapping_add(1); l_i_23 = n_; };
                    bb_ = 7; continue 'fl1;
                }
                10 => {
                    { let _ = F::Mtx_ProjectPointsStq(c, fp.wrapping_add(192), fp.wrapping_add(64), p_world2screen_3, fp, fp.wrapping_add(128), 4i32); };
                    l_i_23 = 0i32;
                    bb_ = 11; continue 'fl1;
                }
                11 => {
                    if (((((l_i_23) < (4i32)) as i32)) != 0) { bb_ = 12; continue 'fl1; } else { bb_ = 14; continue 'fl1; }
                }
                12 => {
                    if ((((!((F::EftWater_IsOnScreen(c, (fp.wrapping_add(192)).wrapping_add((((l_i_23) as u32)).wrapping_mul(16)), 4i32)) != 0)) as i32)) != 0) { bb_ = 15; continue 'fl1; } else { bb_ = 16; continue 'fl1; }
                }
                13 => {
                    { let o_ = l_i_23; let n_ = (o_).wrapping_add(1); l_i_23 = n_; };
                    bb_ = 11; continue 'fl1;
                }
                14 => {
                    l_z_22 = (((((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)).wrapping_add((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_shr((10i32) as u32);
                    if ((F::EftUtil_IsCamUnderWater(c)) != 0) { bb_ = 18; continue 'fl1; } else { bb_ = 20; continue 'fl1; }
                }
                15 => {
                    break 'f;
                }
                16 => {
                    bb_ = 13; continue 'fl1;
                }
                17 => {
                    bb_ = 16; continue 'fl1;
                }
                18 => {
                    { let r_ = 500i32; let n_ = (l_z_22).wrapping_sub(r_); l_z_22 = n_; };
                    bb_ = 19; continue 'fl1;
                }
                19 => {
                    l_q_24 = c.m.r32(sym::gOtCur);
                    { let v_ = (l_q_24).wrapping_add((((1i32) as u32)).wrapping_mul(48)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
                    l_d_25 = (l_q_24).wrapping_add(16);
                    if (((((l_q_24) == (((0i32) as u32))) as i32)) != 0) { bb_ = 21; continue 'fl1; } else { bb_ = 22; continue 'fl1; }
                }
                20 => {
                    { let r_ = 500i32; let n_ = (l_z_22).wrapping_add(r_); l_z_22 = n_; };
                    bb_ = 19; continue 'fl1;
                }
                21 => {
                    break 'f;
                }
                22 => {
                    { let v_ = ((536870914i32) as u32); let a_ = l_q_24; c.m.w32(a_, v_); };
                    { let v_ = ((268435456i32) as u32); let a_ = (l_q_24).wrapping_add(8); c.m.w32(a_, v_); };
                    { let v_ = ((1342177282i32) as u32); let a_ = (l_q_24).wrapping_add(12); c.m.w32(a_, v_); };
                    { let v_ = ((0i32) as u32); let a_ = (l_q_24).wrapping_add(4); c.m.w32(a_, v_); };
                    { let v_ = ((1152921504606879745i64) as u64); let a_ = l_d_25; c.m.w64(a_, v_); };
                    { let v_ = ((14i32) as u64); let a_ = (l_q_24).wrapping_add(24); c.m.w64(a_, v_); };
                    { let v_ = (((((p_alphaRef_13) as u64)).wrapping_shl((4i32) as u32)) | (((327693i32) as u64))); let a_ = (l_q_24).wrapping_add(32); c.m.w64(a_, v_); };
                    { let v_ = ((71i32) as u64); let a_ = (l_q_24).wrapping_add(40); c.m.w64(a_, v_); };
                    F::eft_e__EftWaterOt_Add(c, l_q_24, l_z_22, ((p_layer_15) as i32));
                    l_p_16 = c.m.r32(sym::gOtCur);
                    { let v_ = (l_p_16).wrapping_add((((1i32) as u32)).wrapping_mul(144)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
                    if (((((l_p_16) == (((0i32) as u32))) as i32)) != 0) { bb_ = 24; continue 'fl1; } else { bb_ = 25; continue 'fl1; }
                }
                23 => {
                    bb_ = 22; continue 'fl1;
                }
                24 => {
                    break 'f;
                }
                25 => {
                    { let v_ = ((92i32) as u64); let a_ = (l_p_16).wrapping_add(32); c.m.w64(a_, v_); };
                    { let v_ = ((536870920i32) as u32); let a_ = l_p_16; c.m.w32(a_, v_); };
                    { let v_ = ((268435456i32) as u32); let a_ = (l_p_16).wrapping_add(8); c.m.w32(a_, v_); };
                    { let v_ = 16429131440647602177u64; let a_ = (l_p_16).wrapping_add(16); c.m.w64(a_, v_); };
                    { let v_ = ((18599481537995104i64) as u64); let a_ = (l_p_16).wrapping_add(24); c.m.w64(a_, v_); };
                    { let v_ = ((0i32) as u32); let a_ = (l_p_16).wrapping_add(4); c.m.w32(a_, v_); };
                    { let v_ = ((1342177288i32) as u32); let a_ = (l_p_16).wrapping_add(12); c.m.w32(a_, v_); };
                    { let v_ = p_r_9; let a_ = (((l_p_16).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = p_g_10; let a_ = (((l_p_16).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = p_b_11; let a_ = (((l_p_16).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = p_a_12; let a_ = (((l_p_16).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = F32(c.m.r32(((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_p_16).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = p_r_9; let a_ = (((l_p_16).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = p_g_10; let a_ = (((l_p_16).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = p_b_11; let a_ = (((l_p_16).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = p_a_12; let a_ = (((l_p_16).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = F32(c.m.r32(((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_p_16).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = p_r_9; let a_ = (((l_p_16).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = p_g_10; let a_ = (((l_p_16).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = p_b_11; let a_ = (((l_p_16).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = p_a_12; let a_ = (((l_p_16).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = F32(c.m.r32(((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_p_16).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = p_r_9; let a_ = (((l_p_16).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = p_g_10; let a_ = (((l_p_16).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = p_b_11; let a_ = (((l_p_16).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = p_a_12; let a_ = (((l_p_16).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = F32(c.m.r32(((fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_p_16).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16)))); let a_ = (((l_p_16).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32(((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = (((l_p_16).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16)))); let a_ = (((l_p_16).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32(((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = (((l_p_16).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16)))); let a_ = (((l_p_16).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32(((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = (((l_p_16).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16)))); let a_ = (((l_p_16).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32(((fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = (((l_p_16).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_16).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_16).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_16).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                    { let v_ = ((255i32) as u64); let a_ = ((((l_p_16).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                    { let v_ = (((c.m.r32((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_16).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_16).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_16).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                    { let v_ = ((255i32) as u64); let a_ = ((((l_p_16).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                    { let v_ = (((c.m.r32((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_16).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_16).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_16).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                    { let v_ = ((255i32) as u64); let a_ = ((((l_p_16).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                    { let v_ = (((c.m.r32((fp.wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_16).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_16).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_16).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                    { let v_ = ((255i32) as u64); let a_ = ((((l_p_16).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                    { let v_ = c.m.r64(p_tex_14); let a_ = (l_p_16).wrapping_add(40); c.m.w64(a_, v_); };
                    F::eft_e__EftWaterOt_Add(c, l_p_16, l_z_22, ((p_layer_15) as i32));
                    l_q_26 = c.m.r32(sym::gOtCur);
                    { let v_ = (l_q_26).wrapping_add((((1i32) as u32)).wrapping_mul(48)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
                    l_d_27 = (l_q_26).wrapping_add(16);
                    if (((((l_q_26) == (((0i32) as u32))) as i32)) != 0) { bb_ = 27; continue 'fl1; } else { bb_ = 28; continue 'fl1; }
                }
                26 => {
                    bb_ = 25; continue 'fl1;
                }
                27 => {
                    break 'f;
                }
                28 => {
                    { let v_ = ((536870914i32) as u32); let a_ = l_q_26; c.m.w32(a_, v_); };
                    { let v_ = ((268435456i32) as u32); let a_ = (l_q_26).wrapping_add(8); c.m.w32(a_, v_); };
                    { let v_ = ((1342177282i32) as u32); let a_ = (l_q_26).wrapping_add(12); c.m.w32(a_, v_); };
                    { let v_ = ((0i32) as u32); let a_ = (l_q_26).wrapping_add(4); c.m.w32(a_, v_); };
                    { let v_ = ((1152921504606879745i64) as u64); let a_ = l_d_27; c.m.w64(a_, v_); };
                    { let v_ = ((14i32) as u64); let a_ = (l_q_26).wrapping_add(24); c.m.w64(a_, v_); };
                    { let v_ = ((327680i32) as u64); let a_ = (l_q_26).wrapping_add(32); c.m.w64(a_, v_); };
                    { let v_ = ((71i32) as u64); let a_ = (l_q_26).wrapping_add(40); c.m.w64(a_, v_); };
                    F::eft_e__EftWaterOt_Add(c, l_q_26, l_z_22, ((p_layer_15) as i32));
                    break 'fl1;
                }
                29 => {
                    bb_ = 28; continue 'fl1;
                }
                _ => unreachable!(),
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftWater_DrawSprayQuad", &PROBE_N, &[], true); }
    r_
}

pub fn EftWater_UpdateTextures(c: &mut Ctx, mut p_a0_1: i32, mut p_a1_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWater_UpdateTextures", &PROBE_N, &[(p_a0_1) as u32, (p_a1_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    if (((((((((c.m.r32(sym::gEftDust)) != (((0i32) as u32))) as i32)) != 0) && ((((((({ let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(44); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(48); (c.m.r32(a_) as i32) })).wrapping_add({ let a_ = (c.m.r32(sym::gEftDust)).wrapping_add(52); (c.m.r32(a_) as i32) })) != (0i32)) as i32)) != 0)) as i32)) != 0) {
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < (5i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = { let a0_ = ((c.m.r32(sym::gEftDust)).wrapping_add(176)).wrapping_add((((l_i_3) as u32)).wrapping_mul(16)); ((D::call(c, fid::EftVram_AddTex, &[((a0_) as u64), ((p_a0_1) as i64 as u64), ((p_a1_2) as i64 as u64)])) as u64) }; let a_ = ((c.m.r32(sym::gEftDust)).wrapping_add(176)).wrapping_add((((l_i_3) as u32)).wrapping_mul(16)); c.m.w64(a_, v_); };
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWater_UpdateTextures", &PROBE_N, &[], true); }
    r_
}

pub fn eft_e__EftWater_IsClipped(c: &mut Ctx, mut p_v_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_e__EftWater_IsClipped", &PROBE_N, &[(p_v_1) as u32], false); }
    let mut l_clipped_2: i32 = 0;
    let r_ = 'f: {
    l_clipped_2 = 1i32;
    if ((((((((((((((c.m.r32((p_v_1).wrapping_add(8)) as i32)) > (0i32)) as i32)) != 0) && ((((((c.m.r32(p_v_1) as i32)) <= (65519i32)) as i32)) != 0)) as i32)) != 0) && ((((((c.m.r32(p_v_1) as i32)) > (0i32)) as i32)) != 0)) as i32)) != 0) {
        if ((((((c.m.r32((p_v_1).wrapping_add(4)) as i32)) <= (65519i32)) as i32)) != 0) {
            l_clipped_2 = ((((c.m.r32((p_v_1).wrapping_add(4)) as i32)) <= (0i32)) as i32);
        }
    }
    break 'f l_clipped_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "eft_e__EftWater_IsClipped", &PROBE_N, &[], true); }
    r_
}

pub fn eft_e__EftWater_QueueTri(c: &mut Ctx, mut p_p0_1: u32, mut p_p1_2: u32, mut p_p2_3: u32, mut p_c0_4: u32, mut p_c1_5: u32, mut p_c2_6: u32, mut p_t0_7: u32, mut p_t1_8: u32, mut p_t2_9: u32, mut p_layer_10: i32, mut p_z_11: i32, mut p_tex0_12: u64) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_e__EftWater_QueueTri", &PROBE_N, &[(p_p0_1) as u32, (p_p1_2) as u32, (p_p2_3) as u32, (p_c0_4) as u32, (p_c1_5) as u32, (p_c2_6) as u32, (p_t0_7) as u32, (p_t1_8) as u32, (p_t2_9) as u32, (p_layer_10) as u32, (p_z_11) as u32, (p_tex0_12) as u32], false); }
    let mut l_p_13: u32 = 0;
    let r_ = 'f: {
    l_p_13 = c.m.r32(sym::gOtCur);
    { let v_ = (l_p_13).wrapping_add((((1i32) as u32)).wrapping_mul(128)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
    { let v_ = ((91i32) as u64); let a_ = (l_p_13).wrapping_add(32); c.m.w64(a_, v_); };
    { let v_ = ((536870919i32) as u32); let a_ = l_p_13; c.m.w32(a_, v_); };
    { let v_ = ((268435456i32) as u32); let a_ = (l_p_13).wrapping_add(8); c.m.w32(a_, v_); };
    { let v_ = ((1342177287i32) as u32); let a_ = (l_p_13).wrapping_add(12); c.m.w32(a_, v_); };
    { let v_ = 14123288431433908225u64; let a_ = (l_p_13).wrapping_add(16); c.m.w64(a_, v_); };
    { let v_ = ((268423679713632i64) as u64); let a_ = (l_p_13).wrapping_add(24); c.m.w64(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_13).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u64); let a_ = (l_p_13).wrapping_add(120); c.m.w64(a_, v_); };
    { let v_ = ((rt::f2u(F32(c.m.r32(p_c0_4)))) as u8); let a_ = (((l_p_13).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2u(F32(c.m.r32((p_c0_4).wrapping_add(4))))) as u8); let a_ = (((l_p_13).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2u(F32(c.m.r32((p_c0_4).wrapping_add(8))))) as u8); let a_ = (((l_p_13).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2u(F32(c.m.r32((p_c0_4).wrapping_add(12))))) as u8); let a_ = (((l_p_13).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = F32(c.m.r32((p_t0_7).wrapping_add(8))); let a_ = (((l_p_13).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = ((rt::f2u(F32(c.m.r32(p_c1_5)))) as u8); let a_ = (((l_p_13).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2u(F32(c.m.r32((p_c1_5).wrapping_add(4))))) as u8); let a_ = (((l_p_13).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2u(F32(c.m.r32((p_c1_5).wrapping_add(8))))) as u8); let a_ = (((l_p_13).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2u(F32(c.m.r32((p_c1_5).wrapping_add(12))))) as u8); let a_ = (((l_p_13).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = F32(c.m.r32((p_t1_8).wrapping_add(8))); let a_ = (((l_p_13).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = ((rt::f2u(F32(c.m.r32(p_c2_6)))) as u8); let a_ = (((l_p_13).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2u(F32(c.m.r32((p_c2_6).wrapping_add(4))))) as u8); let a_ = (((l_p_13).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2u(F32(c.m.r32((p_c2_6).wrapping_add(8))))) as u8); let a_ = (((l_p_13).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2u(F32(c.m.r32((p_c2_6).wrapping_add(12))))) as u8); let a_ = (((l_p_13).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = F32(c.m.r32((p_t2_9).wrapping_add(8))); let a_ = (((l_p_13).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = p_tex0_12; let a_ = (l_p_13).wrapping_add(40); c.m.w64(a_, v_); };
    { let v_ = F32(c.m.r32(p_t0_7)); let a_ = (((l_p_13).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_t0_7).wrapping_add(4))); let a_ = (((l_p_13).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(p_t1_8)); let a_ = (((l_p_13).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_t1_8).wrapping_add(4))); let a_ = (((l_p_13).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(p_t2_9)); let a_ = (((l_p_13).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_t2_9).wrapping_add(4))); let a_ = (((l_p_13).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = (((c.m.r32(p_p0_1) as i32)) as u64); let a_ = (((l_p_13).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
    { let v_ = (((c.m.r32((p_p0_1).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_13).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
    { let v_ = (((c.m.r32((p_p0_1).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_13).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
    { let v_ = ((255i32) as u64); let a_ = ((((l_p_13).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
    { let v_ = (((c.m.r32(p_p1_2) as i32)) as u64); let a_ = (((l_p_13).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
    { let v_ = (((c.m.r32((p_p1_2).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_13).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
    { let v_ = (((c.m.r32((p_p1_2).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_13).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
    { let v_ = ((255i32) as u64); let a_ = ((((l_p_13).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
    { let v_ = (((c.m.r32(p_p2_3) as i32)) as u64); let a_ = (((l_p_13).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
    { let v_ = (((c.m.r32((p_p2_3).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_13).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
    { let v_ = (((c.m.r32((p_p2_3).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_13).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
    { let v_ = ((255i32) as u64); let a_ = ((((l_p_13).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
    F::eft_e__EftWaterOt_Add(c, l_p_13, p_z_11, p_layer_10);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "eft_e__EftWater_QueueTri", &PROBE_N, &[], true); }
    r_
}

pub fn EftWater_DrawClippedFan(c: &mut Ctx, mut p_poly_1: u32, mut p_layer_2: i32, mut p_tex0_3: u64) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWater_DrawClippedFan", &PROBE_N, &[(p_poly_1) as u32, (p_layer_2) as u32, (p_tex0_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(1056) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 1056);
    let mut l_plane_4: u32 = 0;
    let mut l_n_5: i32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_z_7: i32 = 0;
    let r_ = 'f: {
        { let _ = { rt::memcpy(c, fp, p_poly_1, ((((3i32) as u32)).wrapping_mul(48u32)) as u32); fp }; };
        p_poly_1 = fp;
        l_plane_4 = F::EftGfx_GetClipPlanes(c);
        l_n_5 = 3i32;
        {
            l_i_6 = 0i32;
            'l1: loop {
                if !(((((l_i_6) < (5i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_n_5 = F::ClipPoly_ClipPlane(c, p_poly_1, l_plane_4, l_n_5);
                    { let o_ = l_plane_4; let n_ = (o_).wrapping_add(16); l_plane_4 = n_; };
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
        if (((((l_n_5) == (0i32)) as i32)) != 0) {
            break 'f;
        }
        F::ClipPoly_ProjectCur(c, fp.wrapping_add(768), fp.wrapping_add(912), p_poly_1, l_n_5);
        {
            l_i_6 = 2i32;
            'l3: loop {
                if !(((((l_i_6) < (l_n_5)) as i32)) != 0) { break 'l3; }
                'c4: {
                    l_z_7 = (rt::div_i32((((c.m.r32(((fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)).wrapping_add((c.m.r32(((fp.wrapping_add(768)).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add((c.m.r32(((fp.wrapping_add(768)).wrapping_add((((l_i_6) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)), 3i32)).wrapping_shr((8i32) as u32);
                    if ((F::EftUtil_IsCamUnderWater(c)) != 0) {
                        { let r_ = 500i32; let n_ = (l_z_7).wrapping_sub(r_); l_z_7 = n_; };
                    } else {
                        { let r_ = 500i32; let n_ = (l_z_7).wrapping_add(r_); l_z_7 = n_; };
                    }
                    if ((((((c.m.r32(((fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) > (16777215i32)) as i32)) != 0) {
                        { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    }
                    if ((((((c.m.r32(((fp.wrapping_add(768)).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) > (16777215i32)) as i32)) != 0) {
                        { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    }
                    if ((((((c.m.r32(((fp.wrapping_add(768)).wrapping_add((((l_i_6) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) > (16777215i32)) as i32)) != 0) {
                        { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add((((l_i_6) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    }
                    if ((((((((((F::eft_e__EftWater_IsClipped(c, (fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16)))) != 0) && ((F::eft_e__EftWater_IsClipped(c, (fp.wrapping_add(768)).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(16)))) != 0)) as i32)) != 0) && ((F::eft_e__EftWater_IsClipped(c, (fp.wrapping_add(768)).wrapping_add((((l_i_6) as u32)).wrapping_mul(16)))) != 0)) as i32)) != 0) {
                        break 'c4;
                    }
                    F::eft_e__EftWater_QueueTri(c, (fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(768)).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(768)).wrapping_add((((l_i_6) as u32)).wrapping_mul(16)), ((p_poly_1).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(32), ((p_poly_1).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(48))).wrapping_add(32), ((p_poly_1).wrapping_add((((l_i_6) as u32)).wrapping_mul(48))).wrapping_add(32), (fp.wrapping_add(912)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(912)).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(912)).wrapping_add((((l_i_6) as u32)).wrapping_mul(16)), p_layer_2, l_z_7, p_tex0_3);
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftWater_DrawClippedFan", &PROBE_N, &[], true); }
    r_
}
