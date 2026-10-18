#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Gsc_Init(c: &mut Ctx, mut p_commands_1: u32, mut p_fileMax_2: i32, mut p_taskMax_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_Init", &PROBE_N, &[(p_commands_1) as u32, (p_fileMax_2) as u32, (p_taskMax_3) as u32], false); }
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    { let v_ = 1i32; let a_ = sym::gGscActive; c.m.w32(a_, (v_) as u32); };
    { let v_ = F::Heap_Alloc(c, (((((p_fileMax_2) as u32)).wrapping_mul(36u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gGscFilePool; c.m.w32(a_, v_); };
    { let v_ = F::Heap_Alloc(c, (((((p_taskMax_3) as u32)).wrapping_mul(80u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gGscTaskPool; c.m.w32(a_, v_); };
    { let v_ = p_commands_1; let a_ = sym::gGscCommands; c.m.w32(a_, v_); };
    { let v_ = p_fileMax_2; let a_ = sym::gGscFileMax; c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = sym::gGscFileSerial; c.m.w32(a_, (v_) as u32); };
    F::List_Init(c, sym::gGscFileList);
    F::List_Init(c, sym::gGscFileFree);
    { let v_ = p_taskMax_3; let a_ = sym::gGscTaskMax; c.m.w32(a_, (v_) as u32); };
    { let v_ = ((0i32) as u32); let a_ = sym::gGscCurTask; c.m.w32(a_, v_); };
    F::List_Init(c, sym::gGscTaskList);
    F::List_Init(c, sym::gGscTaskFree);
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < ((c.m.r32(sym::gGscFileMax) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a1_ = (c.m.r32(sym::gGscFilePool)).wrapping_add((((l_i_4) as u32)).wrapping_mul(36)); F::List_PushBack(c, sym::gGscFileFree, a1_) };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    {
        l_i_4 = 0i32;
        'l3: loop {
            if !(((((l_i_4) < ((c.m.r32(sym::gGscTaskMax) as i32))) as i32)) != 0) { break 'l3; }
            'c4: {
                { let a1_ = (c.m.r32(sym::gGscTaskPool)).wrapping_add((((l_i_4) as u32)).wrapping_mul(80)); F::List_PushBack(c, sym::gGscTaskFree, a1_) };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_Init", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_InitDefault(c: &mut Ctx, mut p_commands_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_InitDefault", &PROBE_N, &[(p_commands_1) as u32], false); }
    let r_ = 'f: {
    F::Gsc_Init(c, p_commands_1, 10i32, 50i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_InitDefault", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_Exit(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_Exit", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::List_Init(c, sym::gGscFileList);
    F::List_Init(c, sym::gGscFileFree);
    F::List_Init(c, sym::gGscTaskList);
    F::List_Init(c, sym::gGscTaskFree);
    { let a0_ = c.m.r32(sym::gGscFilePool); F::Heap_Free(c, a0_) };
    { let a0_ = c.m.r32(sym::gGscTaskPool); F::Heap_Free(c, a0_) };
    { let v_ = 0i32; let a_ = sym::gGscActive; c.m.w32(a_, (v_) as u32); };
    { let v_ = ((0i32) as u32); let a_ = sym::gGscCommands; c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = sym::gGscCurTask; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_Exit", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_Nop(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_Nop", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_Nop", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_SetCommands(c: &mut Ctx, mut p_commands_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_SetCommands", &PROBE_N, &[(p_commands_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_commands_1; let a_ = sym::gGscCommands; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_SetCommands", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_Update", &PROBE_N, &[], false); }
    let mut l_task_1: u32 = 0;
    let mut l_next_2: u32 = 0;
    let r_ = 'f: {
    if (((c.m.r32(sym::gGscActive) as i32)) != 0) {
        {
            l_task_1 = F::List_GetHead(c, sym::gGscTaskList);
            'l1: loop {
                if !(((((l_task_1) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_next_2 = F::List_GetNext(c, l_task_1);
                    F::Gsc_StepTask(c, l_task_1);
                }
                l_task_1 = l_next_2;
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_Update", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_StepTaskById(c: &mut Ctx, mut p_id_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_StepTaskById", &PROBE_N, &[(p_id_1) as u32], false); }
    let r_ = 'f: {
    if (((c.m.r32(sym::gGscActive) as i32)) != 0) {
        { let a0_ = F::Gsc_FindTask(c, p_id_1); F::Gsc_StepTask(c, a0_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_StepTaskById", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_NotifyAll(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_NotifyAll", &PROBE_N, &[], false); }
    let mut l_task_1: u32 = 0;
    let mut l_handler_2: u32 = 0;
    let r_ = 'f: {
    if (((c.m.r32(sym::gGscActive) as i32)) != 0) {
        l_task_1 = F::List_GetHead(c, sym::gGscTaskList);
        if (((((l_task_1) != (((0i32) as u32))) as i32)) != 0) {
            'l1: loop {
                'c2: {
                    { let v_ = l_task_1; let a_ = sym::gGscCurTask; c.m.w32(a_, v_); };
                    if ((((((({ let a_ = c.m.r32((l_task_1).wrapping_add(24)); c.m.r32(a_) }) & (((15i32) as u32)))) == (((1i32) as u32))) as i32)) != 0) {
                        l_handler_2 = { let a0_ = (({ let a_ = (c.m.r32((l_task_1).wrapping_add(24))).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as i32); F::Gsc_FindHandler(c, a0_) };
                        F::Gsc_ResetOperands(c);
                        { let _ = ((D::call(c, l_handler_2, &[((3i32) as i64 as u64), (((l_task_1).wrapping_add(48)) as u64)])) as i32); };
                    }
                    l_task_1 = F::List_GetNext(c, l_task_1);
                }
                if !(((((l_task_1) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
            }
            { let v_ = ((0i32) as u32); let a_ = sym::gGscCurTask; c.m.w32(a_, v_); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_NotifyAll", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_StepTask(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_StepTask", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_run_2: i32 = 0;
    let mut l_handler_3: u32 = 0;
    let mut l_done_4: i32 = 0;
    let mut l_cmd_5: u32 = 0;
    let mut l_type_6: i32 = 0;
    let mut l_next_7: u32 = 0;
    let mut l_next2_8: u32 = 0;
    let mut l_action_9: i32 = 0;
    let r_ = 'f: {
    { let v_ = p_task_1; let a_ = sym::gGscCurTask; c.m.w32(a_, v_); };
    l_run_2 = 1i32;
    'l1: loop {
        'c2: {
            l_cmd_5 = c.m.r32((p_task_1).wrapping_add(24));
            l_type_6 = ((((c.m.r32(l_cmd_5)) & (((15i32) as u32)))) as i32);
            's3: {
                let sv10: i32 = l_type_6;
                match sv10 {
                    0i32 => {
                        F::Gsc_FreeTask(c, p_task_1);
                        l_run_2 = 0i32;
                        break 's3;
                    }
                    2i32 => {
                        { let _ = { let a0_ = (((c.m.r16((l_cmd_5).wrapping_add((((1i32) as u32)).wrapping_mul(2))) as i16)) as i32); F::Gsc_CallAction(c, a0_) }; };
                        l_next_7 = c.m.r32((p_task_1).wrapping_add(24));
                        { let v_ = 0i32; let a_ = (p_task_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = (p_task_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                        { let v_ = (l_next_7).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let a_ = (p_task_1).wrapping_add(24); c.m.w32(a_, v_); };
                        break 's3;
                    }
                    3i32 => {
                        l_action_9 = (((c.m.r16((l_cmd_5).wrapping_add((((1i32) as u32)).wrapping_mul(2))) as i16)) as i32);
                        if ((((!(((((c.m.r32((p_task_1).wrapping_add(32)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
                            { let v_ = F::Gsc_CallAction(c, l_action_9); let a_ = (p_task_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                            { let r_ = 1i32; let a_ = (p_task_1).wrapping_add(32); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                        if (((c.m.r32((p_task_1).wrapping_add(44)) as i32)) != 0) {
                            { let a0_ = (c.m.r32((p_task_1).wrapping_add(40)) as i32); F::Gsc_AbortTask(c, a0_) };
                        }
                        if ((((!(({ let a0_ = (c.m.r32((p_task_1).wrapping_add(40)) as i32); F::Gsc_IsTaskAlive(c, a0_) }) != 0)) as i32)) != 0) {
                            l_next2_8 = c.m.r32((p_task_1).wrapping_add(24));
                            { let v_ = 0i32; let a_ = (p_task_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 0i32; let a_ = (p_task_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                            { let v_ = (l_next2_8).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let a_ = (p_task_1).wrapping_add(24); c.m.w32(a_, v_); };
                        } else {
                            l_run_2 = 0i32;
                        }
                        break 's3;
                    }
                    1i32 => {
                        l_done_4 = 0i32;
                        l_handler_3 = { let a0_ = ((c.m.r16((l_cmd_5).wrapping_add((((1i32) as u32)).wrapping_mul(2)))) as i32); F::Gsc_FindHandler(c, a0_) };
                        if (((c.m.r32((p_task_1).wrapping_add(44)) as i32)) != 0) {
                            if ((((!(((((c.m.r32((p_task_1).wrapping_add(32)) as i32)) & (2i32))) != 0)) as i32)) != 0) {
                                F::Gsc_ResetOperands(c);
                                l_done_4 = ((D::call(c, l_handler_3, &[((4i32) as i64 as u64), (((p_task_1).wrapping_add(48)) as u64)])) as i32);
                                { let r_ = 2i32; let a_ = (p_task_1).wrapping_add(32); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            }
                        } else {
                            if ((((!(((((c.m.r32((p_task_1).wrapping_add(32)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
                                F::Gsc_ResetOperands(c);
                                l_done_4 = ((D::call(c, l_handler_3, &[((0i32) as i64 as u64), (((p_task_1).wrapping_add(48)) as u64)])) as i32);
                                { let r_ = 1i32; let a_ = (p_task_1).wrapping_add(32); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            }
                        }
                        if (((((l_done_4) == (0i32)) as i32)) != 0) {
                            F::Gsc_ResetOperands(c);
                            l_done_4 = ((D::call(c, l_handler_3, &[((2i32) as i64 as u64), (((p_task_1).wrapping_add(48)) as u64)])) as i32);
                        }
                        if (((((l_done_4) != (0i32)) as i32)) != 0) {
                            F::Gsc_ResetOperands(c);
                            { let _ = ((D::call(c, l_handler_3, &[((1i32) as i64 as u64), (((p_task_1).wrapping_add(48)) as u64)])) as i32); };
                            l_cmd_5 = { let a0_ = c.m.r32((p_task_1).wrapping_add(24)); F::Gsc_NextCommand(c, a0_) };
                            { let v_ = 0i32; let a_ = (p_task_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                            { let v_ = l_cmd_5; let a_ = (p_task_1).wrapping_add(24); c.m.w32(a_, v_); };
                            { let v_ = 0i32; let a_ = (p_task_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                        } else {
                            l_run_2 = 0i32;
                            { let r_ = 10i32; let a_ = (p_task_1).wrapping_add(36); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                        }
                        break 's3;
                    }
                    _ => {
                        break 's3;
                    }
                }
            }
        }
        if !((l_run_2) != 0) { break 'l1; }
    }
    { let v_ = ((0i32) as u32); let a_ = sym::gGscCurTask; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_StepTask", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_LoadFile(c: &mut Ctx, mut p_file_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_LoadFile", &PROBE_N, &[(p_file_1) as u32], false); }
    let mut l_node_2: u32 = 0;
    let mut l_chunk_3: u32 = 0;
    let mut l_i_4: u32 = 0;
    let mut l_count_5: u32 = 0;
    let r_ = 'f: {
    l_chunk_3 = p_file_1;
    l_count_5 = (((16u32).wrapping_add(c.m.r32((l_chunk_3).wrapping_add(4)))).wrapping_add(c.m.r32((l_chunk_3).wrapping_add(8)))).wrapping_shr((2i32) as u32);
    {
        l_i_4 = ((0i32) as u32);
        'l1: loop {
            if !(((((l_i_4) < (l_count_5)) as i32)) != 0) { break 'l1; }
            'c2: {
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    l_node_2 = F::Gsc_GetFreeFile(c);
    { let v_ = { let a0_ = (c.m.r32(sym::gGscFileSerial) as i32); F::Gsc_MakeFileHandle(c, a0_) }; let a_ = (l_node_2).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (l_node_2).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let a_ = sym::gGscFileSerial; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    { let v_ = l_chunk_3; let a_ = (l_node_2).wrapping_add(20); c.m.w32(a_, v_); };
    l_chunk_3 = (l_chunk_3).wrapping_add(c.m.r32((l_chunk_3).wrapping_add(4)));
    { let v_ = l_chunk_3; let a_ = (l_node_2).wrapping_add(24); c.m.w32(a_, v_); };
    l_chunk_3 = (((l_chunk_3).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(c.m.r32((l_chunk_3).wrapping_add(4)))).wrapping_add(c.m.r32((l_chunk_3).wrapping_add(8)));
    { let v_ = l_chunk_3; let a_ = (l_node_2).wrapping_add(28); c.m.w32(a_, v_); };
    l_chunk_3 = (((l_chunk_3).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(c.m.r32((l_chunk_3).wrapping_add(4)))).wrapping_add(c.m.r32((l_chunk_3).wrapping_add(8)));
    { let v_ = l_chunk_3; let a_ = (l_node_2).wrapping_add(32); c.m.w32(a_, v_); };
    F::Gsc_LinkFile(c, l_node_2);
    break 'f (c.m.r32((l_node_2).wrapping_add(12)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_LoadFile", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_UnloadFile(c: &mut Ctx, mut p_handle_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_UnloadFile", &PROBE_N, &[(p_handle_1) as u32], false); }
    let mut l_file_2: u32 = 0;
    let r_ = 'f: {
    l_file_2 = F::Gsc_FindFile(c, p_handle_1);
    if (((((l_file_2) != (((0i32) as u32))) as i32)) != 0) {
        F::Gsc_FreeFile(c, l_file_2);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_UnloadFile", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_FindFile(c: &mut Ctx, mut p_handle_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_FindFile", &PROBE_N, &[(p_handle_1) as u32], false); }
    let mut l_file_2: u32 = 0;
    let mut l_found_3: u32 = 0;
    let r_ = 'f: {
    l_file_2 = F::List_GetHead(c, sym::gGscFileList);
    l_found_3 = ((0i32) as u32);
    'l1: while (((((l_file_2) != (((0i32) as u32))) as i32)) != 0) {
        if ((((((c.m.r32((l_file_2).wrapping_add(12)) as i32)) == (p_handle_1)) as i32)) != 0) {
            l_found_3 = l_file_2;
            break 'l1;
        }
        l_file_2 = F::List_GetNext(c, l_file_2);
    }
    break 'f l_found_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_FindFile", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_IsFileLoaded(c: &mut Ctx, mut p_handle_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_IsFileLoaded", &PROBE_N, &[(p_handle_1) as u32], false); }
    let r_ = 'f: {
    break 'f (((F::Gsc_FindFile(c, p_handle_1)) != (((0i32) as u32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_IsFileLoaded", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_GetFileCount(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_GetFileCount", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f F::List_GetCount(c, sym::gGscFileList);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_GetFileCount", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_FindTask(c: &mut Ctx, mut p_id_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_FindTask", &PROBE_N, &[(p_id_1) as u32], false); }
    let mut l_task_2: u32 = 0;
    let mut l_found_3: u32 = 0;
    let r_ = 'f: {
    l_task_2 = F::List_GetHead(c, sym::gGscTaskList);
    l_found_3 = ((0i32) as u32);
    'l1: while (((((l_task_2) != (((0i32) as u32))) as i32)) != 0) {
        if ((((((c.m.r32((l_task_2).wrapping_add(12)) as i32)) == (p_id_1)) as i32)) != 0) {
            l_found_3 = l_task_2;
            break 'l1;
        }
        l_task_2 = F::List_GetNext(c, l_task_2);
    }
    break 'f l_found_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_FindTask", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_IsTaskAlive(c: &mut Ctx, mut p_id_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_IsTaskAlive", &PROBE_N, &[(p_id_1) as u32], false); }
    let r_ = 'f: {
    break 'f (((F::Gsc_FindTask(c, p_id_1)) != (((0i32) as u32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_IsTaskAlive", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_GetTaskCount(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_GetTaskCount", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f F::List_GetCount(c, sym::gGscTaskList);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_GetTaskCount", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_GetTaskTime(c: &mut Ctx, mut p_id_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_GetTaskTime", &PROBE_N, &[(p_id_1) as u32], false); }
    let mut l_task_2: u32 = 0;
    let mut l_time_3: i32 = 0;
    let r_ = 'f: {
    l_task_2 = F::Gsc_FindTask(c, p_id_1);
    l_time_3 = 0i32;
    if (((((l_task_2) != (((0i32) as u32))) as i32)) != 0) {
        l_time_3 = (c.m.r32((l_task_2).wrapping_add(36)) as i32);
    }
    break 'f l_time_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_GetTaskTime", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_StartMain(c: &mut Ctx, mut p_handle_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_StartMain", &PROBE_N, &[(p_handle_1) as u32], false); }
    let r_ = 'f: {
    break 'f F::Gsc_StartAction(c, p_handle_1, (1i32).wrapping_neg());
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_StartMain", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_RunAction(c: &mut Ctx, mut p_handle_1: i32, mut p_action_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_RunAction", &PROBE_N, &[(p_handle_1) as u32, (p_action_2) as u32], false); }
    let mut l_id_3: i32 = 0;
    let r_ = 'f: {
    l_id_3 = F::Gsc_StartAction(c, p_handle_1, p_action_2);
    'l1: while ((F::Gsc_IsTaskAlive(c, l_id_3)) != 0) {
        F::Gsc_StepTaskById(c, l_id_3);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_RunAction", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_StartAction(c: &mut Ctx, mut p_handle_1: i32, mut p_action_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_StartAction", &PROBE_N, &[(p_handle_1) as u32, (p_action_2) as u32], false); }
    let mut l_file_3: u32 = 0;
    let mut l_task_4: u32 = 0;
    let r_ = 'f: {
    l_file_3 = F::Gsc_FindFile(c, p_handle_1);
    l_task_4 = F::Gsc_GetFreeTask(c);
    { let v_ = p_action_2; let a_ = (l_task_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a1_ = (c.m.r32((l_file_3).wrapping_add(16)) as i32); F::Gsc_MakeTaskId(c, p_handle_1, a1_) }; let a_ = (l_task_4).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let a_ = (l_file_3).wrapping_add(16); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    { let v_ = c.m.r32((l_file_3).wrapping_add(32)); let a_ = (l_task_4).wrapping_add(20); c.m.w32(a_, v_); };
    { let v_ = { let a0_ = c.m.r32((l_file_3).wrapping_add(28)); F::Gsc_FindAction(c, a0_, p_action_2, 1i32) }; let a_ = (l_task_4).wrapping_add(24); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_task_4).wrapping_add(28); c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = (l_task_4).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (l_task_4).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (l_task_4).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (l_task_4).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
    F::Gsc_LinkTask(c, l_task_4);
    break 'f (c.m.r32((l_task_4).wrapping_add(12)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_StartAction", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_KillTask(c: &mut Ctx, mut p_id_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_KillTask", &PROBE_N, &[(p_id_1) as u32], false); }
    let mut l_task_2: u32 = 0;
    let r_ = 'f: {
    l_task_2 = F::Gsc_FindTask(c, p_id_1);
    if (((((l_task_2) != (((0i32) as u32))) as i32)) != 0) {
        F::Gsc_FreeTask(c, l_task_2);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_KillTask", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_AbortTask(c: &mut Ctx, mut p_id_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_AbortTask", &PROBE_N, &[(p_id_1) as u32], false); }
    let mut l_task_2: u32 = 0;
    let r_ = 'f: {
    l_task_2 = F::Gsc_FindTask(c, p_id_1);
    if (((((l_task_2) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = 1i32; let a_ = (l_task_2).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_AbortTask", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_HasAction(c: &mut Ctx, mut p_handle_1: i32, mut p_action_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_HasAction", &PROBE_N, &[(p_handle_1) as u32, (p_action_2) as u32], false); }
    let r_ = 'f: {
    break 'f ((({ let a0_ = { let a_ = (F::Gsc_FindFile(c, p_handle_1)).wrapping_add(28); c.m.r32(a_) }; F::Gsc_FindAction(c, a0_, p_action_2, 0i32) }) != (((0i32) as u32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_HasAction", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_GetCurrentTask(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_GetCurrentTask", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f c.m.r32(sym::gGscCurTask);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_GetCurrentTask", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_CallAction(c: &mut Ctx, mut p_action_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_CallAction", &PROBE_N, &[(p_action_1) as u32], false); }
    let mut l_cur_2: u32 = 0;
    let mut l_id_3: i32 = 0;
    let r_ = 'f: {
    l_cur_2 = F::Gsc_GetCurrentTask(c);
    l_id_3 = { let a0_ = { let a0_ = (c.m.r32((l_cur_2).wrapping_add(12)) as i32); F::Gsc_GetFileHandle(c, a0_) }; F::Gsc_StartAction(c, a0_, p_action_1) };
    { let a0_ = F::Gsc_FindTask(c, l_id_3); F::Gsc_StepTask(c, a0_) };
    { let v_ = l_cur_2; let a_ = sym::gGscCurTask; c.m.w32(a_, v_); };
    break 'f l_id_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_CallAction", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_GetCurrentTime(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_GetCurrentTime", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a0_ = { let a_ = (F::Gsc_GetCurrentTask(c)).wrapping_add(12); (c.m.r32(a_) as i32) }; F::Gsc_GetTaskTime(c, a0_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_GetCurrentTime", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_IsCurrentBegun(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_IsCurrentBegun", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (F::Gsc_GetCurrentTask(c)).wrapping_add(32); (c.m.r32(a_) as i32) }) & (1i32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_IsCurrentBegun", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_DumpFile(c: &mut Ctx, mut p_handle_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_DumpFile", &PROBE_N, &[(p_handle_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = F::Gsc_FindFile(c, p_handle_1); F::Gsc_DumpFileNode(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_DumpFile", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_DumpCurrent(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_DumpCurrent", &PROBE_N, &[], false); }
    let mut l_data_1: u32 = 0;
    let r_ = 'f: {
    l_data_1 = { let a_ = (c.m.r32(sym::gGscCurTask)).wrapping_add(20); c.m.r32(a_) };
    { let a0_ = { let a_ = (c.m.r32(sym::gGscCurTask)).wrapping_add(24); c.m.r32(a_) }; let a1_ = (l_data_1).wrapping_add(c.m.r32((l_data_1).wrapping_add(4))); F::Gsc_DumpCommand(c, a0_, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_DumpCurrent", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_DumpCommand(c: &mut Ctx, mut p_cmd_1: u32, mut p_data_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_DumpCommand", &PROBE_N, &[(p_cmd_1) as u32, (p_data_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_kind_3: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 20);
        { let v_ = sym::gsc__str0; c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = sym::gsc__str1; c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = sym::gsc__str2; c.m.w32(fp.wrapping_add(8), v_); }
        'l1: loop {
            'c2: {
                l_kind_3 = ((((c.m.r32(p_cmd_1)) & (((15i32) as u32)))) as i32);
                's3: {
                    let sv4: i32 = l_kind_3;
                    match sv4 {
                        1i32 => {
                            l_kind_3 = ((c.m.r8((p_cmd_1).wrapping_add(((1i32) as u32)))) as i32);
                            break 's3;
                        }
                        2i32 => {
                            l_kind_3 = (((c.m.r16((p_cmd_1).wrapping_add((((1i32) as u32)).wrapping_mul(2))) as i16)) as i32);
                            break 's3;
                        }
                        3i32 => {
                            l_kind_3 = (((c.m.r16((p_cmd_1).wrapping_add((((1i32) as u32)).wrapping_mul(2))) as i16)) as i32);
                            break 's3;
                        }
                        8i32 => {
                            l_kind_3 = ((c.m.r8((p_cmd_1).wrapping_add(((2i32) as u32)))) as i32);
                            break 's3;
                        }
                        10i32 => {
                            l_kind_3 = (((((c.m.r32(p_cmd_1)).wrapping_shr((4i32) as u32)) & (((15i32) as u32)))) as i32);
                            break 's3;
                        }
                        _ => {
                        }
                    }
                }
                { let o_ = p_cmd_1; let n_ = (o_).wrapping_add(4); p_cmd_1 = n_; };
            }
            if !((((c.m.r32(p_cmd_1)) & (((8i32) as u32)))) != 0) { break 'l1; }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Gsc_DumpCommand", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_DumpFileNode(c: &mut Ctx, mut p_file_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_DumpFileNode", &PROBE_N, &[(p_file_1) as u32], false); }
    let mut l_action_2: u32 = 0;
    let mut l_data_3: u32 = 0;
    let mut l_cmd_4: u32 = 0;
    let r_ = 'f: {
    l_action_2 = (c.m.r32((p_file_1).wrapping_add(28))).wrapping_add({ let a_ = (c.m.r32((p_file_1).wrapping_add(28))).wrapping_add(4); c.m.r32(a_) });
    l_data_3 = (c.m.r32((p_file_1).wrapping_add(32))).wrapping_add({ let a_ = (c.m.r32((p_file_1).wrapping_add(32))).wrapping_add(4); c.m.r32(a_) });
    'l1: while ((((((((((((((c.m.r32(l_action_2)) & (4278190080u32))).wrapping_shr((24i32) as u32)) | ((((c.m.r32(l_action_2)) & (((16711680i32) as u32)))).wrapping_shr((8i32) as u32)))) | ((((c.m.r32(l_action_2)) & (((65280i32) as u32)))).wrapping_shl((8i32) as u32)))) | ((((c.m.r32(l_action_2)) & (((255i32) as u32)))).wrapping_shl((24i32) as u32)))) == (((1196638531i32) as u32))) as i32)) != 0) {
        l_cmd_4 = (l_action_2).wrapping_add(c.m.r32((l_action_2).wrapping_add(4)));
        'l2: while (((((((c.m.r32(l_cmd_4)) & (((15i32) as u32)))) != (((0i32) as u32))) as i32)) != 0) {
            F::Gsc_DumpCommand(c, l_cmd_4, l_data_3);
            l_cmd_4 = F::Gsc_NextCommand(c, l_cmd_4);
        }
        l_action_2 = (((l_action_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(c.m.r32((l_action_2).wrapping_add(4)))).wrapping_add(c.m.r32((l_action_2).wrapping_add(8)));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_DumpFileNode", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_DumpCommandsById(c: &mut Ctx, mut p_file_1: u32, mut p_id_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_DumpCommandsById", &PROBE_N, &[(p_file_1) as u32, (p_id_2) as u32], false); }
    let mut l_action_3: u32 = 0;
    let mut l_data_4: u32 = 0;
    let mut l_cmd_5: u32 = 0;
    let r_ = 'f: {
    l_action_3 = (c.m.r32((p_file_1).wrapping_add(28))).wrapping_add({ let a_ = (c.m.r32((p_file_1).wrapping_add(28))).wrapping_add(4); c.m.r32(a_) });
    l_data_4 = (c.m.r32((p_file_1).wrapping_add(32))).wrapping_add({ let a_ = (c.m.r32((p_file_1).wrapping_add(32))).wrapping_add(4); c.m.r32(a_) });
    'l1: while ((((((((((((((c.m.r32(l_action_3)) & (4278190080u32))).wrapping_shr((24i32) as u32)) | ((((c.m.r32(l_action_3)) & (((16711680i32) as u32)))).wrapping_shr((8i32) as u32)))) | ((((c.m.r32(l_action_3)) & (((65280i32) as u32)))).wrapping_shl((8i32) as u32)))) | ((((c.m.r32(l_action_3)) & (((255i32) as u32)))).wrapping_shl((24i32) as u32)))) == (((1196638531i32) as u32))) as i32)) != 0) {
        l_cmd_5 = (l_action_3).wrapping_add(c.m.r32((l_action_3).wrapping_add(4)));
        'l2: while (((((((c.m.r32(l_cmd_5)) & (((15i32) as u32)))) != (((0i32) as u32))) as i32)) != 0) {
            if (((((((((((c.m.r32(l_cmd_5)) & (((15i32) as u32)))) == (((1i32) as u32))) as i32)) != 0) && (((((((c.m.r16((l_cmd_5).wrapping_add((((1i32) as u32)).wrapping_mul(2)))) as i32)) == (p_id_2)) as i32)) != 0)) as i32)) != 0) {
                F::Gsc_DumpCommand(c, l_cmd_5, l_data_4);
            }
            l_cmd_5 = F::Gsc_NextCommand(c, l_cmd_5);
        }
        l_action_3 = (((l_action_3).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(c.m.r32((l_action_3).wrapping_add(4)))).wrapping_add(c.m.r32((l_action_3).wrapping_add(8)));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_DumpCommandsById", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_GetInt(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_GetInt", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = { let a0_ = c.m.r32(sym::gGscCurTask); F::GscTask_NextOperand(c, a0_, 0i32) }; (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_GetInt", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_GetFloat(c: &mut Ctx) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_GetFloat", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = { let a0_ = c.m.r32(sym::gGscCurTask); F::GscTask_NextOperand(c, a0_, 1i32) }; F32(c.m.r32(a_)) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_GetFloat", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_GetString(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_GetString", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a0_ = c.m.r32(sym::gGscCurTask); F::GscTask_NextOperand(c, a0_, 2i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_GetString", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_GetIntOr(c: &mut Ctx, mut p_def_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_GetIntOr", &PROBE_N, &[(p_def_1) as u32], false); }
    let mut l_value_2: u32 = 0;
    let r_ = 'f: {
    l_value_2 = { let a0_ = c.m.r32(sym::gGscCurTask); F::GscTask_NextOperand(c, a0_, 0i32) };
    if (((((l_value_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f p_def_1;
    }
    break 'f (c.m.r32(l_value_2) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_GetIntOr", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_GetFloatOr(c: &mut Ctx, mut p_def_1: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_GetFloatOr", &PROBE_N, &[(p_def_1).0], false); }
    let mut l_value_2: u32 = 0;
    let r_ = 'f: {
    l_value_2 = { let a0_ = c.m.r32(sym::gGscCurTask); F::GscTask_NextOperand(c, a0_, 1i32) };
    if (((((l_value_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f p_def_1;
    }
    break 'f F32(c.m.r32(l_value_2));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_GetFloatOr", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_GetStringOr(c: &mut Ctx, mut p_def_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_GetStringOr", &PROBE_N, &[(p_def_1) as u32], false); }
    let mut l_value_2: u32 = 0;
    let r_ = 'f: {
    l_value_2 = { let a0_ = c.m.r32(sym::gGscCurTask); F::GscTask_NextOperand(c, a0_, 2i32) };
    if (((((l_value_2) != (((0i32) as u32))) as i32)) != 0) {
        p_def_1 = l_value_2;
    }
    break 'f p_def_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_GetStringOr", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_SkipOperand(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_SkipOperand", &PROBE_N, &[], false); }
    let mut l_has_1: i32 = 0;
    let r_ = 'f: {
    l_has_1 = F::Gsc_HasOperand(c);
    if ((l_has_1) != 0) {
        { let a_ = (c.m.r32(sym::gGscCurTask)).wrapping_add(28); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(4); c.m.w32(a_, n_); };
    }
    break 'f l_has_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_SkipOperand", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_ResetOperands(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_ResetOperands", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = ({ let a_ = (c.m.r32(sym::gGscCurTask)).wrapping_add(24); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let a_ = (c.m.r32(sym::gGscCurTask)).wrapping_add(28); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_ResetOperands", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_FindOption(c: &mut Ctx, mut p_letter_1: i8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_FindOption", &PROBE_N, &[(p_letter_1) as u32], false); }
    let r_ = 'f: {
    break 'f F::Gsc_FindOptionN(c, p_letter_1, 0i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_FindOption", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_FindOptionN(c: &mut Ctx, mut p_letter_1: i8, mut p_n_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_FindOptionN", &PROBE_N, &[(p_letter_1) as u32, (p_n_2) as u32], false); }
    let mut l_word_3: u32 = 0;
    let mut l_type_4: i32 = 0;
    let mut l_task_5: u32 = 0;
    let mut l_found_6: i32 = 0;
    let mut l_count_7: i32 = 0;
    let r_ = 'f: {
    l_task_5 = c.m.r32(sym::gGscCurTask);
    l_found_6 = 0i32;
    l_count_7 = 0i32;
    l_word_3 = ((c.m.r32((l_task_5).wrapping_add(24))).wrapping_add(((((({ let a_ = (c.m.r32((l_task_5).wrapping_add(24))).wrapping_add(((1i32) as u32)); c.m.r8(a_) }) as i32)) as u32)).wrapping_mul(4))).wrapping_add((((1i32) as u32)).wrapping_mul(4));
    'l1: while (({ l_type_4 = ((((c.m.r32(l_word_3)) & (((15i32) as u32)))) as i32); ((c.m.r32(l_word_3)) & (((8i32) as u32))) }) != 0) {
        if (((((((((l_type_4) == (8i32)) as i32)) != 0) && (((((((c.m.r8((l_word_3).wrapping_add(((1i32) as u32)))) as i32)) == (((p_letter_1) as i32))) as i32)) != 0)) as i32)) != 0) {
            if (((((l_count_7) >= (p_n_2)) as i32)) != 0) {
                l_found_6 = 1i32;
                { let v_ = (l_word_3).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let a_ = (l_task_5).wrapping_add(28); c.m.w32(a_, v_); };
                break 'l1;
            }
            { let o_ = l_count_7; let n_ = (o_).wrapping_add(1); l_count_7 = n_; };
            l_word_3 = ((l_word_3).wrapping_add((((((c.m.r8((l_word_3).wrapping_add(((2i32) as u32)))) as i32)).wrapping_mul(4i32)) as u32))).wrapping_add(((4i32) as u32));
        } else {
            { let r_ = 4i32; let n_ = (l_word_3).wrapping_add(((r_) as u32)); l_word_3 = n_; };
        }
    }
    break 'f l_found_6;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_FindOptionN", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_SeekOption(c: &mut Ctx, mut p_n_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_SeekOption", &PROBE_N, &[(p_n_1) as u32], false); }
    let mut l_word_2: u32 = 0;
    let mut l_type_3: i32 = 0;
    let mut l_task_4: u32 = 0;
    let mut l_letter_5: i32 = 0;
    let mut l_count_6: i32 = 0;
    let r_ = 'f: {
    l_task_4 = c.m.r32(sym::gGscCurTask);
    l_letter_5 = 0i32;
    l_count_6 = 0i32;
    l_word_2 = ((c.m.r32((l_task_4).wrapping_add(24))).wrapping_add(((((({ let a_ = (c.m.r32((l_task_4).wrapping_add(24))).wrapping_add(((1i32) as u32)); c.m.r8(a_) }) as i32)) as u32)).wrapping_mul(4))).wrapping_add((((1i32) as u32)).wrapping_mul(4));
    'l1: while (({ l_type_3 = ((((c.m.r32(l_word_2)) & (((15i32) as u32)))) as i32); ((c.m.r32(l_word_2)) & (((8i32) as u32))) }) != 0) {
        if (((((l_type_3) == (8i32)) as i32)) != 0) {
            if (((((l_count_6) >= (p_n_1)) as i32)) != 0) {
                { let v_ = (l_word_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let a_ = (l_task_4).wrapping_add(28); c.m.w32(a_, v_); };
                l_letter_5 = ((((c.m.r8((l_word_2).wrapping_add(((1i32) as u32)))) as i8)) as i32);
                break 'l1;
            }
            { let o_ = l_count_6; let n_ = (o_).wrapping_add(1); l_count_6 = n_; };
            l_word_2 = ((l_word_2).wrapping_add((((((c.m.r8((l_word_2).wrapping_add(((2i32) as u32)))) as i32)).wrapping_mul(4i32)) as u32))).wrapping_add(((4i32) as u32));
        } else {
            { let r_ = 4i32; let n_ = (l_word_2).wrapping_add(((r_) as u32)); l_word_2 = n_; };
        }
    }
    break 'f l_letter_5;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_SeekOption", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_CountOptions(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_CountOptions", &PROBE_N, &[], false); }
    let mut l_count_1: i8 = 0;
    let mut l_word_2: u32 = 0;
    let r_ = 'f: {
    l_count_1 = ((0i32) as i8);
    l_word_2 = (({ let a_ = (c.m.r32(sym::gGscCurTask)).wrapping_add(24); c.m.r32(a_) }).wrapping_add(((((({ let a_ = ({ let a_ = (c.m.r32(sym::gGscCurTask)).wrapping_add(24); c.m.r32(a_) }).wrapping_add(((1i32) as u32)); c.m.r8(a_) }) as i32)) as u32)).wrapping_mul(4))).wrapping_add((((1i32) as u32)).wrapping_mul(4));
    'l1: while ((((c.m.r32(l_word_2)) & (((8i32) as u32)))) != 0) {
        if (((((((c.m.r32(l_word_2)) & (((15i32) as u32)))) == (((8i32) as u32))) as i32)) != 0) {
            { let o_ = l_count_1; let n_ = (o_).wrapping_add(1); l_count_1 = n_; };
            l_word_2 = ((l_word_2).wrapping_add((((((c.m.r8((l_word_2).wrapping_add(((2i32) as u32)))) as i32)).wrapping_mul(4i32)) as u32))).wrapping_add(((4i32) as u32));
        } else {
            { let r_ = 4i32; let n_ = (l_word_2).wrapping_add(((r_) as u32)); l_word_2 = n_; };
        }
    }
    break 'f ((l_count_1) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_CountOptions", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_CountOption(c: &mut Ctx, mut p_letter_1: i8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_CountOption", &PROBE_N, &[(p_letter_1) as u32], false); }
    let mut l_count_2: i8 = 0;
    let mut l_word_3: u32 = 0;
    let r_ = 'f: {
    l_count_2 = ((0i32) as i8);
    l_word_3 = (({ let a_ = (c.m.r32(sym::gGscCurTask)).wrapping_add(24); c.m.r32(a_) }).wrapping_add(((((({ let a_ = ({ let a_ = (c.m.r32(sym::gGscCurTask)).wrapping_add(24); c.m.r32(a_) }).wrapping_add(((1i32) as u32)); c.m.r8(a_) }) as i32)) as u32)).wrapping_mul(4))).wrapping_add((((1i32) as u32)).wrapping_mul(4));
    'l1: while ((((c.m.r32(l_word_3)) & (((8i32) as u32)))) != 0) {
        if (((((((c.m.r32(l_word_3)) & (((15i32) as u32)))) == (((8i32) as u32))) as i32)) != 0) {
            if (((((((c.m.r8((l_word_3).wrapping_add(((1i32) as u32)))) as i32)) == (((p_letter_1) as i32))) as i32)) != 0) {
                { let o_ = l_count_2; let n_ = (o_).wrapping_add(1); l_count_2 = n_; };
            }
            l_word_3 = ((l_word_3).wrapping_add((((((c.m.r8((l_word_3).wrapping_add(((2i32) as u32)))) as i32)).wrapping_mul(4i32)) as u32))).wrapping_add(((4i32) as u32));
        } else {
            { let r_ = 4i32; let n_ = (l_word_3).wrapping_add(((r_) as u32)); l_word_3 = n_; };
        }
    }
    break 'f ((l_count_2) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_CountOption", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_HasOperand(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_HasOperand", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f ((((({ let a_ = { let a_ = (c.m.r32(sym::gGscCurTask)).wrapping_add(28); c.m.r32(a_) }; c.m.r32(a_) }) & (((15i32) as u32)))) == (((10i32) as u32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_HasOperand", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_MakeFileHandle(c: &mut Ctx, mut p_serial_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_MakeFileHandle", &PROBE_N, &[(p_serial_1) as u32], false); }
    let r_ = 'f: {
    break 'f (p_serial_1).wrapping_shl((16i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_MakeFileHandle", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_MakeTaskId(c: &mut Ctx, mut p_handle_1: i32, mut p_serial_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_MakeTaskId", &PROBE_N, &[(p_handle_1) as u32, (p_serial_2) as u32], false); }
    let r_ = 'f: {
    break 'f ((p_handle_1) | (((p_serial_2) & (65535i32))));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_MakeTaskId", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_GetFileHandle(c: &mut Ctx, mut p_id_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_GetFileHandle", &PROBE_N, &[(p_id_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((((((p_id_1) as u32)) & (4294901760u32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_GetFileHandle", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_NextCommand(c: &mut Ctx, mut p_cmd_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_NextCommand", &PROBE_N, &[(p_cmd_1) as u32], false); }
    let mut l_count_2: i32 = 0;
    let mut l_word_3: u32 = 0;
    let mut l_n_4: i32 = 0;
    let r_ = 'f: {
    l_count_2 = ((c.m.r8((p_cmd_1).wrapping_add(((1i32) as u32)))) as i32);
    l_word_3 = ((p_cmd_1).wrapping_add((((l_count_2) as u32)).wrapping_mul(4))).wrapping_add((((1i32) as u32)).wrapping_mul(4));
    'l1: while ((((c.m.r32(l_word_3)) & (((8i32) as u32)))) != 0) {
        if (((((((c.m.r32(l_word_3)) & (((15i32) as u32)))) == (((8i32) as u32))) as i32)) != 0) {
            l_n_4 = (((c.m.r8((l_word_3).wrapping_add(((2i32) as u32)))) as i32)).wrapping_add(1i32);
            l_word_3 = ((l_word_3).wrapping_add((((((c.m.r8((l_word_3).wrapping_add(((2i32) as u32)))) as i32)).wrapping_mul(4i32)) as u32))).wrapping_add(((4i32) as u32));
            { let r_ = l_n_4; let n_ = (l_count_2).wrapping_add(r_); l_count_2 = n_; };
        } else {
            { let r_ = 4i32; let n_ = (l_word_3).wrapping_add(((r_) as u32)); l_word_3 = n_; };
        }
    }
    break 'f ((p_cmd_1).wrapping_add((((l_count_2) as u32)).wrapping_mul(4))).wrapping_add((((1i32) as u32)).wrapping_mul(4));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_NextCommand", &PROBE_N, &[], true); }
    r_
}

pub fn GscTask_PeekOperand(c: &mut Ctx, mut p_task_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GscTask_PeekOperand", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_word_2: u32 = 0;
    let mut l_values_3: u32 = 0;
    let mut l_value_4: u32 = 0;
    let r_ = 'f: {
    l_word_2 = { let a_ = c.m.r32((p_task_1).wrapping_add(28)); c.m.r32(a_) };
    l_values_3 = (c.m.r32((p_task_1).wrapping_add(20))).wrapping_add({ let a_ = (c.m.r32((p_task_1).wrapping_add(20))).wrapping_add(4); c.m.r32(a_) });
    l_value_4 = ((0i32) as u32);
    if (((((((l_word_2) & (((15i32) as u32)))) == (((10i32) as u32))) as i32)) != 0) {
        l_value_4 = (l_values_3).wrapping_add(((l_word_2).wrapping_shr((8i32) as u32)).wrapping_mul(4));
    }
    break 'f l_value_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "GscTask_PeekOperand", &PROBE_N, &[], true); }
    r_
}

pub fn GscTask_NextOperand(c: &mut Ctx, mut p_task_1: u32, mut p_type_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GscTask_NextOperand", &PROBE_N, &[(p_task_1) as u32, (p_type_2) as u32], false); }
    let mut l_value_3: u32 = 0;
    let r_ = 'f: {
    l_value_3 = F::GscTask_PeekOperand(c, p_task_1);
    if (((((l_value_3) != (((0i32) as u32))) as i32)) != 0) {
        { let a_ = (p_task_1).wrapping_add(28); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(4); c.m.w32(a_, n_); };
    }
    break 'f l_value_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "GscTask_NextOperand", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_FindHandler(c: &mut Ctx, mut p_id_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_FindHandler", &PROBE_N, &[(p_id_1) as u32], false); }
    let mut l_entry_2: u32 = 0;
    let mut l_found_3: u32 = 0;
    let r_ = 'f: {
    l_entry_2 = c.m.r32(sym::gGscCommands);
    l_found_3 = ((0i32) as u32);
    'l1: while (((((c.m.r32((l_entry_2).wrapping_add(4))) != (((0i32) as u32))) as i32)) != 0) {
        if ((((((c.m.r32(l_entry_2) as i32)) == (p_id_1)) as i32)) != 0) {
            l_found_3 = c.m.r32((l_entry_2).wrapping_add(4));
            break 'l1;
        }
        { let o_ = l_entry_2; let n_ = (o_).wrapping_add(8); l_entry_2 = n_; };
    }
    break 'f l_found_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_FindHandler", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_GetFreeFile(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_GetFreeFile", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f F::List_GetHead(c, sym::gGscFileFree);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_GetFreeFile", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_LinkFile(c: &mut Ctx, mut p_file_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_LinkFile", &PROBE_N, &[(p_file_1) as u32], false); }
    let r_ = 'f: {
    { let _ = F::List_Remove(c, sym::gGscFileFree, p_file_1); };
    F::List_PushBack(c, sym::gGscFileList, p_file_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_LinkFile", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_FreeFile(c: &mut Ctx, mut p_file_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_FreeFile", &PROBE_N, &[(p_file_1) as u32], false); }
    let mut l_task_2: u32 = 0;
    let mut l_next_3: u32 = 0;
    let r_ = 'f: {
    {
        l_task_2 = F::List_GetHead(c, sym::gGscTaskList);
        'l1: loop {
            if !(((((l_task_2) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
            'c2: {
                l_next_3 = F::List_GetNext(c, l_task_2);
                if ((((({ let a0_ = (c.m.r32((l_task_2).wrapping_add(12)) as i32); F::Gsc_GetFileHandle(c, a0_) }) == ((c.m.r32((p_file_1).wrapping_add(12)) as i32))) as i32)) != 0) {
                    F::Gsc_FreeTask(c, l_task_2);
                }
            }
            l_task_2 = l_next_3;
        }
    }
    { let _ = F::List_Remove(c, sym::gGscFileList, p_file_1); };
    F::List_PushBack(c, sym::gGscFileFree, p_file_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_FreeFile", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_GetFreeTask(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_GetFreeTask", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f F::List_GetHead(c, sym::gGscTaskFree);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_GetFreeTask", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_LinkTask(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_LinkTask", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let _ = F::List_Remove(c, sym::gGscTaskFree, p_task_1); };
    F::List_PushFront(c, sym::gGscTaskList, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_LinkTask", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_FreeTask(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_FreeTask", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let _ = F::List_Remove(c, sym::gGscTaskList, p_task_1); };
    F::List_PushBack(c, sym::gGscTaskFree, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_FreeTask", &PROBE_N, &[], true); }
    r_
}

pub fn Gsc_FindAction(c: &mut Ctx, mut p_code_1: u32, mut p_action_2: i32, mut p_unused_3: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Gsc_FindAction", &PROBE_N, &[(p_code_1) as u32, (p_action_2) as u32, (p_unused_3) as u32], false); }
    let mut l_chunk_4: u32 = 0;
    let mut l_found_5: u32 = 0;
    let r_ = 'f: {
    l_chunk_4 = (p_code_1).wrapping_add(c.m.r32((p_code_1).wrapping_add(4)));
    l_found_5 = ((0i32) as u32);
    'l1: while ((((((((((((((c.m.r32(l_chunk_4)) & (4278190080u32))).wrapping_shr((24i32) as u32)) | ((((c.m.r32(l_chunk_4)) & (((16711680i32) as u32)))).wrapping_shr((8i32) as u32)))) | ((((c.m.r32(l_chunk_4)) & (((65280i32) as u32)))).wrapping_shl((8i32) as u32)))) | ((((c.m.r32(l_chunk_4)) & (((255i32) as u32)))).wrapping_shl((24i32) as u32)))) == (((1196638531i32) as u32))) as i32)) != 0) {
        if ((((((c.m.r32((l_chunk_4).wrapping_add(12)) as i32)) == (p_action_2)) as i32)) != 0) {
            l_found_5 = (l_chunk_4).wrapping_add(c.m.r32((l_chunk_4).wrapping_add(4)));
            break 'l1;
        }
        l_chunk_4 = (((l_chunk_4).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(c.m.r32((l_chunk_4).wrapping_add(4)))).wrapping_add(c.m.r32((l_chunk_4).wrapping_add(8)));
    }
    break 'f l_found_5;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Gsc_FindAction", &PROBE_N, &[], true); }
    r_
}
