#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn File_Stub264A50(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "File_Stub264A50", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "File_Stub264A50", &PROBE_N, &[], true); }
    r_
}

pub fn Vsync_Handler(c: &mut Ctx, mut p_cause_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vsync_Handler", &PROBE_N, &[(p_cause_1) as u32], false); }
    let r_ = 'f: {
    { let a_ = sym::gVsyncCount; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(1); c.m.w32(a_, n_); };
    F::ADXPS2_ExecVint(c, 0i32);
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Vsync_Handler", &PROBE_N, &[], true); }
    r_
}

pub fn Vsync_InstallHandler(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vsync_InstallHandler", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let _ = F::sceGsSyncVCallback(c, fid::Vsync_Handler); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vsync_InstallHandler", &PROBE_N, &[], true); }
    r_
}

pub fn File_AdxErrorCallback(c: &mut Ctx, mut p_obj_1: u32, mut p_msg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "File_AdxErrorCallback", &PROBE_N, &[(p_obj_1) as u32, (p_msg_2) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "File_AdxErrorCallback", &PROBE_N, &[], true); }
    r_
}

pub fn File_LoadPartitionNw(c: &mut Ctx, mut p_pt_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "File_LoadPartitionNw", &PROBE_N, &[(p_pt_1) as u32], false); }
    let mut l_part_2: u32 = 0;
    let r_ = 'f: {
    l_part_2 = (sym::gAfsPartitionTbl).wrapping_add((((p_pt_1) as u32)).wrapping_mul(12));
    break 'f { let a1_ = c.m.r32(l_part_2); let a3_ = c.m.r32((l_part_2).wrapping_add(8)); F::ADXF_LoadPartitionNw(c, p_pt_1, a1_, ((0i32) as u32), a3_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "File_LoadPartitionNw", &PROBE_N, &[], true); }
    r_
}

pub fn File_IsPartitionLoaded(c: &mut Ctx, mut p_pt_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "File_IsPartitionLoaded", &PROBE_N, &[(p_pt_1) as u32], false); }
    let r_ = 'f: {
    break 'f (((F::ADXF_GetPtStat(c, p_pt_1)) == (3i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "File_IsPartitionLoaded", &PROBE_N, &[], true); }
    r_
}

pub fn File_WaitPartition(c: &mut Ctx, mut p_pt_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "File_WaitPartition", &PROBE_N, &[(p_pt_1) as u32], false); }
    let r_ = 'f: {
    'l1: while (((((F::ADXF_GetPtStat(c, p_pt_1)) != (3i32)) as i32)) != 0) {
        F::ADXM_ExecMain(c);
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "File_WaitPartition", &PROBE_N, &[], true); }
    r_
}

pub fn File_WaitPartitionTimeout(c: &mut Ctx, mut p_pt_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "File_WaitPartitionTimeout", &PROBE_N, &[(p_pt_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (2400i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((F::ADXF_GetPtStat(c, p_pt_1)) == (3i32)) as i32)) != 0) {
                    break 'l1;
                }
                F::ADXM_ExecMain(c);
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "File_WaitPartitionTimeout", &PROBE_N, &[], true); }
    r_
}

pub fn File_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "File_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::File_InitAdx(c);
    F::File_LoadPartition1(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "File_Init", &PROBE_N, &[], true); }
    r_
}

pub fn File_InitAdx(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "File_InitAdx", &PROBE_N, &[], false); }
    let mut l_dvd_1: u32 = 0;
    let mut l_fc_2: u32 = 0;
    let mut l_part_3: u32 = 0;
    let mut l_player_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
    l_dvd_1 = sym::gAdxDvdFsParam;
    { let _ = F::memset(c, l_dvd_1, 0i32, 20u32); };
    { let v_ = sym::gFileRootDir; let a_ = l_dvd_1; c.m.w32(a_, v_); };
    F::ADXPS2_SetupDvdFs(c, l_dvd_1);
    l_fc_2 = sym::gAdxFcacheParam;
    { let _ = F::memset(c, l_fc_2, 0i32, 16u32); };
    { let v_ = sym::gFileDirName; let a_ = l_fc_2; c.m.w32(a_, v_); };
    { let v_ = 16i32; let a_ = (l_fc_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = 400i32; let a_ = (l_fc_2).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let v_ = F::Heap_Alloc(c, 400i32, ((32i32) as u32), 0i32, 2i32); let a_ = (l_fc_2).wrapping_add(8); c.m.w32(a_, v_); };
    F::ADXPS2_LoadFcacheDvd(c, l_fc_2);
    F::ADXPS2_SetupThrd(c, ((0i32) as u32), 0i32);
    F::ADXT_Init(c);
    F::Vsync_InstallHandler(c);
    F::ADXERR_EntryErrFunc(c, fid::File_AdxErrorCallback, ((0i32) as u32));
    l_part_3 = sym::gAfsPartitionTbl;
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((l_i_5) < (3i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = { let a0_ = (c.m.r32((l_part_3).wrapping_add(4)) as i32); F::Heap_Alloc(c, a0_, ((32i32) as u32), 0i32, 2i32) }; let a_ = (l_part_3).wrapping_add(8); c.m.w32(a_, v_); };
            }
            { { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; }; { let o_ = l_part_3; let n_ = (o_).wrapping_add(12); l_part_3 = n_; }; };
        }
    }
    l_player_4 = sym::gAdxPlayerTbl;
    {
        l_i_5 = 0i32;
        'l3: loop {
            if !(((((l_i_5) < (6i32)) as i32)) != 0) { break 'l3; }
            'c4: {
                { let v_ = { let a0_ = (c.m.r32((l_player_4).wrapping_add(4)) as i32); F::Heap_Alloc(c, a0_, ((32i32) as u32), 0i32, 2i32) }; let a_ = (l_player_4).wrapping_add(12); c.m.w32(a_, v_); };
                { let v_ = { let a1_ = c.m.r32((l_player_4).wrapping_add(12)); let a2_ = (c.m.r32((l_player_4).wrapping_add(4)) as i32); F::ADXT_Create(c, 2i32, a1_, a2_) }; let a_ = (l_player_4).wrapping_add(16); c.m.w32(a_, v_); };
                { let a0_ = c.m.r32((l_player_4).wrapping_add(16)); let a1_ = (c.m.r32((l_player_4).wrapping_add(8)) as i32); F::ADXT_SetReloadSct(c, a0_, a1_) };
                { let a0_ = c.m.r32((l_player_4).wrapping_add(16)); F::ADXT_SetSvrFreq(c, a0_, 30i32) };
            }
            { { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; }; { let o_ = l_player_4; let n_ = (o_).wrapping_add(20); l_player_4 = n_; }; };
        }
    }
    { let _ = F::File_LoadPartitionNw(c, 0i32); };
    { let _ = F::File_WaitPartition(c, 0i32); };
    F::File_InitRequests(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "File_InitAdx", &PROBE_N, &[], true); }
    r_
}

pub fn File_LoadPartition1(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "File_LoadPartition1", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let _ = F::File_LoadPartitionNw(c, 1i32); };
    { let _ = F::File_WaitPartition(c, 1i32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "File_LoadPartition1", &PROBE_N, &[], true); }
    r_
}

pub fn File_Stub264D90(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "File_Stub264D90", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "File_Stub264D90", &PROBE_N, &[], true); }
    r_
}

pub fn Vsync_Wait(c: &mut Ctx, mut p_count_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vsync_Wait", &PROBE_N, &[(p_count_1) as u32], false); }
    let mut l_waited_2: i32 = 0;
    let r_ = 'f: {
    l_waited_2 = 0i32;
    'l1: while (((((c.m.r32(sym::gVsyncCount)) < ((p_count_1).wrapping_sub(((1i32) as u32)))) as i32)) != 0) {
        F::ADXM_WaitVsync(c);
        l_waited_2 = 1i32;
    }
    if (((((((((c.m.r32(sym::gVsyncCount)) < (p_count_1)) as i32)) != 0) || ((((!((l_waited_2) != 0)) as i32)) != 0)) as i32)) != 0) {
        F::ADXM_WaitVsync(c);
    }
    { let v_ = ((0i32) as u32); let a_ = sym::gVsyncCount; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vsync_Wait", &PROBE_N, &[], true); }
    r_
}

pub fn Disc_Identify(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Disc_Identify", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_result_1: i32 = 0;
    let mut l_i_2: u32 = 0;
    let r_ = 'f: {
        's1: {
            let sv3: i32 = F::sceCdGetDiskTypeSafe(c);
            match sv3 {
                1i32 => {
                    l_result_1 = (2i32).wrapping_neg();
                    break 's1;
                }
                20i32 => {
                    l_result_1 = (1i32).wrapping_neg();
                    break 's1;
                }
                _ => {
                    l_result_1 = (3i32).wrapping_neg();
                    break 's1;
                }
            }
        }
        if (((((l_result_1) == ((1i32).wrapping_neg())) as i32)) != 0) {
            {
                l_i_2 = ((0i32) as u32);
                'l2: loop {
                    if !(((((l_i_2) < (((3i32) as u32))) as i32)) != 0) { break 'l2; }
                    'c3: {
                        if ((((({ let a1_ = c.m.r32((sym::gDiscExeNameTbl).wrapping_add((l_i_2).wrapping_mul(4))); F::sceCdSearchFile(c, fp, a1_) }) == (1i32)) as i32)) != 0) {
                            l_result_1 = ((l_i_2) as i32);
                            break 'l2;
                        }
                    }
                    { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                }
            }
        }
        break 'f l_result_1;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Disc_Identify", &PROBE_N, &[], true); }
    r_
}

pub fn Disc_GetDriveState(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Disc_GetDriveState", &PROBE_N, &[], false); }
    let mut l_state_1: i32 = 0;
    let r_ = 'f: {
    's1: {
        let sv2: i32 = F::sceCdStatus(c);
        match sv2 {
            0i32 => {
                l_state_1 = 0i32;
                break 's1;
            }
            1i32 => {
                l_state_1 = 1i32;
                break 's1;
            }
            2i32 => {
                l_state_1 = 2i32;
                break 's1;
            }
            6i32 => {
                l_state_1 = 3i32;
                break 's1;
            }
            10i32 => {
                l_state_1 = 4i32;
                break 's1;
            }
            18i32 => {
                l_state_1 = 5i32;
                break 's1;
            }
            _ => {
                l_state_1 = 6i32;
                break 's1;
            }
        }
    }
    if (((((((((l_state_1) != (1i32)) as i32)) != 0) && (((((l_state_1) != (6i32)) as i32)) != 0)) as i32)) != 0) {
        if (((((F::sceCdGetDiskTypeSafe(c)) == (0i32)) as i32)) != 0) {
            l_state_1 = 7i32;
        }
    }
    break 'f l_state_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Disc_GetDriveState", &PROBE_N, &[], true); }
    r_
}

pub fn File_Stub264F70(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "File_Stub264F70", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "File_Stub264F70", &PROBE_N, &[], true); }
    r_
}

pub fn File_InitRequests(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "File_InitRequests", &PROBE_N, &[], false); }
    let mut l_queue_1: u32 = 0;
    let mut l_free_2: u32 = 0;
    let mut l_req_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_queue_1 = sym::gFileReq;
    l_free_2 = (l_queue_1).wrapping_add(8);
    F::List_Init(c, l_free_2);
    F::List_Init(c, (l_queue_1).wrapping_add(20));
    l_req_3 = (l_queue_1).wrapping_add(32);
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (32i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = 0i32; let a_ = (l_req_3).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                F::List_PushBack(c, l_free_2, l_req_3);
            }
            { { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; }; { let o_ = l_req_3; let n_ = (o_).wrapping_add(24); l_req_3 = n_; }; };
        }
    }
    { let v_ = 0i32; let a_ = l_queue_1; c.m.w32(a_, (v_) as u32); };
    { let v_ = ((0i32) as u32); let a_ = (l_queue_1).wrapping_add(4); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "File_InitRequests", &PROBE_N, &[], true); }
    r_
}

pub fn File_LoadSyncEx(c: &mut Ctx, mut p_id_1: i32, mut p_name_2: u32, mut p_buf_3: u32, mut p_unused_4: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "File_LoadSyncEx", &PROBE_N, &[(p_id_1) as u32, (p_name_2) as u32, (p_buf_3) as u32, (p_unused_4) as u32], false); }
    let mut l_adxf_5: u32 = 0;
    let mut l_sectors_6: i32 = 0;
    let mut l_dst_7: u32 = 0;
    let r_ = 'f: {
    'l1: loop {
        'c2: {
            if (((((p_id_1) >= (0i32)) as i32)) != 0) {
                l_adxf_5 = F::File_OpenById(c, p_id_1);
            } else {
                l_adxf_5 = F::ADXF_Open(c, p_name_2, ((0i32) as u32));
            }
        }
        if !(((((l_adxf_5) == (((0i32) as u32))) as i32)) != 0) { break 'l1; }
    }
    l_sectors_6 = F::ADXF_GetFsizeSct(c, l_adxf_5);
    l_dst_7 = p_buf_3;
    if (((((p_buf_3) == (((0i32) as u32))) as i32)) != 0) {
        l_dst_7 = F::Heap_Alloc(c, (l_sectors_6).wrapping_mul(2048i32), ((64i32) as u32), 0i32, 2i32);
    }
    'l3: while (((((F::ADXF_ReadNw(c, l_adxf_5, l_sectors_6, l_dst_7)) != (l_sectors_6)) as i32)) != 0) {
    }
    'l4: loop {
        'c5: {
            F::ADXM_ExecMain(c);
        }
        if !(((((F::ADXF_GetStat(c, l_adxf_5)) != (3i32)) as i32)) != 0) { break 'l4; }
    }
    F::ADXF_Close(c, l_adxf_5);
    p_buf_3 = l_dst_7;
    break 'f p_buf_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "File_LoadSyncEx", &PROBE_N, &[], true); }
    r_
}

pub fn File_IsBusy(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "File_IsBusy", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f ((((c.m.r32(sym::gFileReq) as i32)) != (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "File_IsBusy", &PROBE_N, &[], true); }
    r_
}

pub fn File_GetPendingCount(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "File_GetPendingCount", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f F::List_GetCount(c, (sym::gFileReq).wrapping_add(20));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "File_GetPendingCount", &PROBE_N, &[], true); }
    r_
}

pub fn File_CancelRequests(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "File_CancelRequests", &PROBE_N, &[], false); }
    let mut l_queue_1: u32 = 0;
    let mut l_req_2: u32 = 0;
    let r_ = 'f: {
    l_queue_1 = sym::gFileReq;
    if ((F::File_IsBusy(c)) != 0) {
        if (((((c.m.r32((l_queue_1).wrapping_add(4))) != (((0i32) as u32))) as i32)) != 0) {
            { let _ = { let a0_ = c.m.r32((l_queue_1).wrapping_add(4)); F::ADXF_Stop(c, a0_) }; };
            { let a0_ = c.m.r32((l_queue_1).wrapping_add(4)); F::ADXF_Close(c, a0_) };
        }
        {
            l_req_2 = F::List_GetHead(c, (l_queue_1).wrapping_add(20));
            'l1: loop {
                if !(((((l_req_2) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((c.m.r32((l_req_2).wrapping_add(8)) as i32)) & (1i32))) != 0) {
                        { let a0_ = c.m.r32((l_req_2).wrapping_add(20)); F::Heap_Free(c, a0_) };
                    }
                    { let v_ = 0i32; let a_ = (l_req_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                }
                l_req_2 = F::List_GetNext(c, l_req_2);
            }
        }
    }
    { let v_ = 0i32; let a_ = l_queue_1; c.m.w32(a_, (v_) as u32); };
    { let v_ = ((0i32) as u32); let a_ = (l_queue_1).wrapping_add(4); c.m.w32(a_, v_); };
    F::List_AppendList(c, (l_queue_1).wrapping_add(8), (l_queue_1).wrapping_add(20));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "File_CancelRequests", &PROBE_N, &[], true); }
    r_
}

pub fn File_Request(c: &mut Ctx, mut p_id_1: i32, mut p_buf_2: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "File_Request", &PROBE_N, &[(p_id_1) as u32, (p_buf_2) as u32], false); }
    let mut l_queue_3: u32 = 0;
    let mut l_flags_4: i32 = 0;
    let mut l_adxf_5: u32 = 0;
    let mut l_sectors_6: i32 = 0;
    let mut l_dst_7: u32 = 0;
    let mut l_req_8: u32 = 0;
    let r_ = 'f: {
    l_queue_3 = sym::gFileReq;
    l_flags_4 = 0i32;
    'l1: loop {
        'c2: {
            l_adxf_5 = F::File_OpenById(c, p_id_1);
        }
        if !(((((l_adxf_5) == (((0i32) as u32))) as i32)) != 0) { break 'l1; }
    }
    l_sectors_6 = F::ADXF_GetFsizeSct(c, l_adxf_5);
    l_dst_7 = p_buf_2;
    if (((((p_buf_2) == (((0i32) as u32))) as i32)) != 0) {
        l_dst_7 = F::Heap_Alloc(c, (l_sectors_6).wrapping_mul(2048i32), ((64i32) as u32), 0i32, 2i32);
        l_flags_4 = 1i32;
    }
    F::ADXF_Close(c, l_adxf_5);
    l_req_8 = F::List_PopFront(c, (l_queue_3).wrapping_add(8));
    { let v_ = l_flags_4; let a_ = (l_req_8).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_id_1; let a_ = (l_req_8).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_sectors_6; let a_ = (l_req_8).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_dst_7; let a_ = (l_req_8).wrapping_add(20); c.m.w32(a_, v_); };
    F::List_PushBack(c, (l_queue_3).wrapping_add(20), l_req_8);
    p_buf_2 = l_dst_7;
    break 'f p_buf_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "File_Request", &PROBE_N, &[], true); }
    r_
}

pub fn File_UpdateRequests(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "File_UpdateRequests", &PROBE_N, &[], false); }
    let mut l_queue_1: u32 = 0;
    let mut l_done_2: i32 = 0;
    let mut l_pending_3: u32 = 0;
    let mut l_req_4: u32 = 0;
    let mut l_entry_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let r_ = 'f: {
    l_queue_1 = sym::gFileReq;
    l_done_2 = 0i32;
    if (((((F::File_GetPendingCount(c)) == (0i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    l_pending_3 = (l_queue_1).wrapping_add(20);
    l_req_4 = F::List_GetHead(c, l_pending_3);
    's1: {
        let sv7: i32 = (c.m.r32(l_queue_1) as i32);
        'k6: {
        'k5: {
        'k4: {
        'k3: {
        'k2: {
            match sv7 { 0i32 => break 'k2, 1i32 => break 'k3, 2i32 => break 'k4, 3i32 => break 'k5, 4i32 => break 'k6, _ => break 's1, }
        }
        { let v_ = 1i32; let a_ = l_queue_1; c.m.w32(a_, (v_) as u32); };
        l_entry_5 = (l_queue_1).wrapping_add(32);
        {
            l_i_6 = 0i32;
            'l7: loop {
                if !(((((l_i_6) < (32i32)) as i32)) != 0) { break 'l7; }
                'c8: {
                    { let r_ = (!(16i32)); let a_ = (l_entry_5).wrapping_add(8); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                }
                { { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; }; { let o_ = l_entry_5; let n_ = (o_).wrapping_add(24); l_entry_5 = n_; }; };
            }
        }
        }
        { let v_ = { let a0_ = (c.m.r32((l_req_4).wrapping_add(12)) as i32); F::File_OpenById(c, a0_) }; let a_ = (l_queue_1).wrapping_add(4); c.m.w32(a_, v_); };
        if (((((c.m.r32((l_queue_1).wrapping_add(4))) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = 2i32; let a_ = l_queue_1; c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
        }
        if ((((({ let a0_ = c.m.r32((l_queue_1).wrapping_add(4)); let a1_ = (c.m.r32((l_req_4).wrapping_add(16)) as i32); let a2_ = c.m.r32((l_req_4).wrapping_add(20)); F::ADXF_ReadNw(c, a0_, a1_, a2_) }) == ((c.m.r32((l_req_4).wrapping_add(16)) as i32))) as i32)) != 0) {
            { let v_ = 3i32; let a_ = l_queue_1; c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
        }
        if ((((({ let a0_ = c.m.r32((l_queue_1).wrapping_add(4)); F::ADXF_GetStat(c, a0_) }) == (3i32)) as i32)) != 0) {
            { let a0_ = c.m.r32((l_queue_1).wrapping_add(4)); F::ADXF_Close(c, a0_) };
            { let v_ = ((0i32) as u32); let a_ = (l_queue_1).wrapping_add(4); c.m.w32(a_, v_); };
            { let v_ = 4i32; let a_ = l_queue_1; c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
        }
        { let r_ = 16i32; let a_ = (l_req_4).wrapping_add(8); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let _ = F::List_Remove(c, l_pending_3, l_req_4); };
        F::List_PushBack(c, (l_queue_1).wrapping_add(8), l_req_4);
        if (((((F::File_GetPendingCount(c)) > (0i32)) as i32)) != 0) {
            { let v_ = 1i32; let a_ = l_queue_1; c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 0i32; let a_ = l_queue_1; c.m.w32(a_, (v_) as u32); };
            l_done_2 = 1i32;
        }
        break 's1;
    }
    break 'f l_done_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "File_UpdateRequests", &PROBE_N, &[], true); }
    r_
}

pub fn File_GetSize(c: &mut Ctx, mut p_id_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "File_GetSize", &PROBE_N, &[(p_id_1) as u32], false); }
    let mut l_adxf_2: u32 = 0;
    let mut l_sectors_3: i32 = 0;
    let r_ = 'f: {
    'l1: loop {
        'c2: {
            l_adxf_2 = F::File_OpenById(c, p_id_1);
        }
        if !(((((l_adxf_2) == (((0i32) as u32))) as i32)) != 0) { break 'l1; }
    }
    l_sectors_3 = F::ADXF_GetFsizeSct(c, l_adxf_2);
    F::ADXF_Close(c, l_adxf_2);
    break 'f (l_sectors_3).wrapping_mul(2048i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "File_GetSize", &PROBE_N, &[], true); }
    r_
}

pub fn File_LoadSyncByName(c: &mut Ctx, mut p_name_1: u32, mut p_buf_2: u32, mut p_unused_3: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "File_LoadSyncByName", &PROBE_N, &[(p_name_1) as u32, (p_buf_2) as u32, (p_unused_3) as u32], false); }
    let r_ = 'f: {
    break 'f F::File_LoadSyncEx(c, (1i32).wrapping_neg(), p_name_1, p_buf_2, p_unused_3);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "File_LoadSyncByName", &PROBE_N, &[], true); }
    r_
}

pub fn File_LoadSync(c: &mut Ctx, mut p_id_1: i32, mut p_buf_2: u32, mut p_unused_3: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "File_LoadSync", &PROBE_N, &[(p_id_1) as u32, (p_buf_2) as u32, (p_unused_3) as u32], false); }
    let r_ = 'f: {
    break 'f F::File_LoadSyncEx(c, p_id_1, ((0i32) as u32), p_buf_2, p_unused_3);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "File_LoadSync", &PROBE_N, &[], true); }
    r_
}

pub fn File_GetRequestQueue(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "File_GetRequestQueue", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f sym::gFileReq;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "File_GetRequestQueue", &PROBE_N, &[], true); }
    r_
}

pub fn File_OpenById(c: &mut Ctx, mut p_id_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "File_OpenById", &PROBE_N, &[(p_id_1) as u32], false); }
    let r_ = 'f: {
    break 'f F::ADXF_OpenAfs(c, (if (((((p_id_1) <= (0i32)) as i32)) != 0) { 0i32 } else { (if (((((p_id_1) < (3400i32)) as i32)) != 0) { 1i32 } else { 2i32 }) }), (if (((((p_id_1) <= (0i32)) as i32)) != 0) { p_id_1 } else { (if (((((p_id_1) < (3400i32)) as i32)) != 0) { (p_id_1).wrapping_sub(1i32) } else { (p_id_1).wrapping_sub(3400i32) }) }));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "File_OpenById", &PROBE_N, &[], true); }
    r_
}

pub fn File_StartAdxById(c: &mut Ctx, mut p_adxt_1: u32, mut p_id_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "File_StartAdxById", &PROBE_N, &[(p_adxt_1) as u32, (p_id_2) as u32], false); }
    let r_ = 'f: {
    F::ADXT_StartAfs(c, p_adxt_1, (if (((((p_id_2) <= (0i32)) as i32)) != 0) { 0i32 } else { (if (((((p_id_2) < (3400i32)) as i32)) != 0) { 1i32 } else { 2i32 }) }), (if (((((p_id_2) <= (0i32)) as i32)) != 0) { p_id_2 } else { (if (((((p_id_2) < (3400i32)) as i32)) != 0) { (p_id_2).wrapping_sub(1i32) } else { (p_id_2).wrapping_sub(3400i32) }) }));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "File_StartAdxById", &PROBE_N, &[], true); }
    r_
}
