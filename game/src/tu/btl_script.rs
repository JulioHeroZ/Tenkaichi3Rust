#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlScript_InitWork(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_InitWork", &PROBE_N, &[], false); }
    let mut l_work_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_work_1 = sym::gBtlScript;
    { let _ = F::memset(c, l_work_1, 0i32, 3040u32); };
    { let v_ = (1i32).wrapping_neg(); let a_ = l_work_1; c.m.w32(a_, (v_) as u32); };
    { let v_ = (((1i32).wrapping_neg()) as i16); let a_ = (l_work_1).wrapping_add(394); c.m.w16(a_, (v_) as u16); };
    { let v_ = ((0i32) as i16); let a_ = (l_work_1).wrapping_add(392); c.m.w16(a_, (v_) as u16); };
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (300i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = ((0i32) as i16); let a_ = (((l_work_1).wrapping_add(852)).wrapping_add(160)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w16(a_, (v_) as u16); };
                { let v_ = ((32767i32) as i16); let a_ = ((((l_work_1).wrapping_add(852)).wrapping_add(160)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))).wrapping_add(2); c.m.w16(a_, (v_) as u16); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_InitWork", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::BtlScript_InitWork(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_Init", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_Nop(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_Nop", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_Nop", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_ScanEvents(c: &mut Ctx, mut p_script_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_ScanEvents", &PROBE_N, &[(p_script_1) as u32], false); }
    let mut l_work_2: u32 = 0;
    let mut l_file_3: u32 = 0;
    let mut l_action_4: u32 = 0;
    let mut l_values_5: u32 = 0;
    let mut l_cmd_6: u32 = 0;
    let mut l_ev_7: u32 = 0;
    let mut l_id_8: i32 = 0;
    let mut l_group_9: i32 = 0;
    let mut l_value_10: i32 = 0;
    let r_ = 'f: {
    l_work_2 = sym::gBtlScript;
    l_file_3 = F::Gsc_FindFile(c, p_script_1);
    l_action_4 = (c.m.r32((l_file_3).wrapping_add(28))).wrapping_add({ let a_ = (c.m.r32((l_file_3).wrapping_add(28))).wrapping_add(4); c.m.r32(a_) });
    l_values_5 = (c.m.r32((l_file_3).wrapping_add(32))).wrapping_add({ let a_ = (c.m.r32((l_file_3).wrapping_add(32))).wrapping_add(4); c.m.r32(a_) });
    'l1: while ((((((((((((((c.m.r32(l_action_4)) & (4278190080u32))).wrapping_shr((24i32) as u32)) | ((((c.m.r32(l_action_4)) & (((16711680i32) as u32)))).wrapping_shr((8i32) as u32)))) | ((((c.m.r32(l_action_4)) & (((65280i32) as u32)))).wrapping_shl((8i32) as u32)))) | ((((c.m.r32(l_action_4)) & (((255i32) as u32)))).wrapping_shl((24i32) as u32)))) == (((1196638531i32) as u32))) as i32)) != 0) {
        l_id_8 = (c.m.r32((l_action_4).wrapping_add(12)) as i32);
        l_cmd_6 = (l_action_4).wrapping_add(c.m.r32((l_action_4).wrapping_add(4)));
        if (((((l_id_8) >= (10000i32)) as i32)) != 0) {
            l_ev_7 = ((l_work_2).wrapping_add(332)).wrapping_add((((l_id_8).wrapping_sub(10000i32)) as u32));
            { let v_ = ((1i32) as u8); let a_ = l_ev_7; rt::bf_set(c, a_, 0, 1, (v_) as u64);  };
            l_group_9 = 4i32;
            if (((((l_id_8) < (10051i32)) as i32)) != 0) {
                l_group_9 = 3i32;
                if (((((l_id_8) < (10050i32)) as i32)) != 0) {
                    if (((((l_id_8) < (10040i32)) as i32)) != 0) {
                        l_group_9 = (((l_id_8) >= (10030i32)) as i32);
                    } else {
                        l_group_9 = 2i32;
                    }
                }
            }
            { let v_ = ((l_group_9) as u8); let a_ = l_ev_7; rt::bf_set(c, a_, 1, 3, (v_) as u64);  };
            { let v_ = ((0i32) as u8); let a_ = l_ev_7; rt::bf_set(c, a_, 4, 1, (v_) as u64);  };
            { let v_ = ((0i32) as u8); let a_ = l_ev_7; rt::bf_set(c, a_, 6, 2, (v_) as u64);  };
            'l2: while (((((((c.m.r32(l_cmd_6)) & (((15i32) as u32)))) != (((0i32) as u32))) as i32)) != 0) {
                if (((((((c.m.r32(l_cmd_6)) & (((15i32) as u32)))) == (((1i32) as u32))) as i32)) != 0) {
                    's3: {
                        let sv11: i32 = ((c.m.r16((l_cmd_6).wrapping_add((((1i32) as u32)).wrapping_mul(2)))) as i32);
                        match sv11 {
                            1201i32 => {
                                { let v_ = ((1i32) as u8); let a_ = l_ev_7; rt::bf_set(c, a_, 4, 1, (v_) as u64);  };
                                break 's3;
                            }
                            8i32 => {
                                l_value_10 = (({ let a_ = (l_values_5).wrapping_add(((c.m.r32((l_cmd_6).wrapping_add((((1i32) as u32)).wrapping_mul(4)))).wrapping_shr((8i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) as i32);
                                { let v_ = (((((l_value_10).wrapping_shr((15i32) as u32)) & (1i32))) as u8); let a_ = l_ev_7; rt::bf_set(c, a_, 5, 1, (v_) as u64);  };
                                's4: {
                                    let sv12: i32 = ((l_value_10) & (4095i32));
                                    match sv12 {
                                        16i32 => {
                                            { let v_ = ((1i32) as u8); let a_ = l_ev_7; rt::bf_set(c, a_, 6, 2, (v_) as u64);  };
                                            break 's4;
                                        }
                                        17i32 => {
                                            { let v_ = ((1i32) as u8); let a_ = l_ev_7; rt::bf_set(c, a_, 6, 2, (v_) as u64);  };
                                            break 's4;
                                        }
                                        18i32 => {
                                            { let v_ = ((1i32) as u8); let a_ = l_ev_7; rt::bf_set(c, a_, 6, 2, (v_) as u64);  };
                                            break 's4;
                                        }
                                        19i32 => {
                                            { let v_ = ((1i32) as u8); let a_ = l_ev_7; rt::bf_set(c, a_, 6, 2, (v_) as u64);  };
                                            break 's4;
                                        }
                                        20i32 => {
                                            { let v_ = ((1i32) as u8); let a_ = l_ev_7; rt::bf_set(c, a_, 6, 2, (v_) as u64);  };
                                            break 's4;
                                        }
                                        21i32 | 22i32 | 23i32 => {
                                            { let v_ = ((2i32) as u8); let a_ = l_ev_7; rt::bf_set(c, a_, 6, 2, (v_) as u64);  };
                                            break 's4;
                                        }
                                        _ => {
                                        }
                                    }
                                }
                                break 's3;
                            }
                            _ => {
                            }
                        }
                    }
                }
                l_cmd_6 = F::Gsc_NextCommand(c, l_cmd_6);
            }
        }
        l_action_4 = (((l_action_4).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(c.m.r32((l_action_4).wrapping_add(4)))).wrapping_add(c.m.r32((l_action_4).wrapping_add(8)));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_ScanEvents", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_Restart(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_Restart", &PROBE_N, &[], false); }
    let mut l_work_1: u32 = 0;
    let r_ = 'f: {
    l_work_1 = sym::gBtlScript;
    { let v_ = 0i32; let a_ = (l_work_1).wrapping_add(384); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((0i32) as u32); let a_ = ((l_work_1).wrapping_add(384)).wrapping_add(4); c.m.w32(a_, v_); };
    F::BtlScript_ResetView(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_Restart", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_Update", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((((((F::BtlSeq_GetState(c)) == (3i32)) as i32)) != 0) || (((((F::BtlSeq_GetState(c)) == (5i32)) as i32)) != 0)) as i32)) != 0) {
        F::BtlScript_UpdateEvents(c);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_Update", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_UpdateView(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_UpdateView", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_work_1: u32 = 0;
    let mut l_cam_2: u32 = 0;
    let mut l_key_3: u32 = 0;
    let mut l_next_4: u32 = 0;
    let mut l_total_5: i32 = 0;
    let mut l_t_6: F32 = F32(0);
    let mut l_state_7: i32 = 0;
    let mut l_window_8: u32 = 0;
    let mut l_text_9: u32 = 0;
    let r_ = 'f: {
        l_work_1 = sym::gBtlScript;
        l_cam_2 = (l_work_1).wrapping_add(400);
        if (((rt::fne(F32(c.m.r32((l_work_1).wrapping_add(396))), F32(0x00000000)) as i32)) != 0) {
            { let _ = { let a0_ = F32(c.m.r32((l_work_1).wrapping_add(396))); F::BtlFacade_ShakeCamera(c, a0_) }; };
        }
        if (((c.m.r32(l_cam_2) as i32)) != 0) {
            l_key_3 = ((l_cam_2).wrapping_add(16)).wrapping_add(((((c.m.r32((l_cam_2).wrapping_add(8)) as i32)) as u32)).wrapping_mul(48));
            l_next_4 = ((l_cam_2).wrapping_add(16)).wrapping_add((((((c.m.r32((l_cam_2).wrapping_add(8)) as i32)).wrapping_add(1i32)) as u32)).wrapping_mul(48));
            if ((((((((((c.m.r32((l_cam_2).wrapping_add(8)) as i32)) < (8i32)) as i32)) != 0) && (((((c.m.r32(l_next_4) as i32)) & (1i32))) != 0)) as i32)) != 0) {
                l_total_5 = rt::f2i(rt::fmul(F32(c.m.r32((l_next_4).wrapping_add(4))), F32(0x43960000)));
                { let r_ = 10i32; let a_ = (l_cam_2).wrapping_add(4); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                l_t_6 = rt::fdiv(rt::i2f((c.m.r32((l_cam_2).wrapping_add(4)) as i32)), rt::i2f(l_total_5));
                if (((rt::fgt(l_t_6, F32(0x3f800000)) as i32)) != 0) {
                    l_t_6 = F32(0x3f800000);
                }
                F::Vec4_Sub(c, fp, ((l_key_3).wrapping_add((((1i32) as u32)).wrapping_mul(48))).wrapping_add(16), (l_key_3).wrapping_add(16));
                F::Vec4_Sub(c, fp.wrapping_add(16), ((l_key_3).wrapping_add((((1i32) as u32)).wrapping_mul(48))).wrapping_add(32), (l_key_3).wrapping_add(32));
                F::Vec4_Scale(c, fp, fp, l_t_6);
                F::Vec4_Scale(c, fp.wrapping_add(16), fp.wrapping_add(16), l_t_6);
                F::Vec4_Add(c, fp, fp, (l_key_3).wrapping_add(16));
                F::Vec4_Add(c, fp.wrapping_add(16), fp.wrapping_add(16), (l_key_3).wrapping_add(32));
                F::BtlFacade_SetFixedCamera(c, fp, fp.wrapping_add(16));
                if (((((l_total_5) < ((c.m.r32((l_cam_2).wrapping_add(4)) as i32))) as i32)) != 0) {
                    { let v_ = 0i32; let a_ = (l_cam_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let a_ = (l_cam_2).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                }
            } else {
                { let v_ = 0i32; let a_ = l_cam_2; c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_cam_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_cam_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            }
        }
        if ((((((((c.m.r16((l_work_1).wrapping_add(394)) as i16)) as i32)) >= (0i32)) as i32)) != 0) {
            l_state_7 = F::Font_GetCmdCount(c);
            l_window_8 = { let a0_ = (((c.m.r16((l_work_1).wrapping_add(392)) as i16)) as i32); F::BtlScript_GetWindow(c, a0_) };
            F::Font_PushStyle(c);
            F::Font_SetStyle(c, l_window_8);
            l_text_9 = { let a0_ = (((c.m.r16((l_work_1).wrapping_add(394)) as i16)) as i32); F::BtlScript_GetText(c, a0_) };
            { let a0_ = (c.m.r32((l_window_8).wrapping_add(72)) as i32); let a1_ = ((c.m.r32((l_window_8).wrapping_add(76)) as i32)).wrapping_sub(F::Font_GetHeight(c, l_text_9)); F::Font_PrintAt(c, a0_, a1_, l_text_9) };
            F::Font_PopStyle(c);
            F::Font_Flush(c, l_state_7);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_UpdateView", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_Free(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_Free", &PROBE_N, &[], false); }
    let mut l_work_1: u32 = 0;
    let r_ = 'f: {
    l_work_1 = sym::gBtlScript;
    if (((((c.m.r32((l_work_1).wrapping_add(4))) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32((l_work_1).wrapping_add(4)); F::Heap_Free(c, a0_) };
    }
    if (((((c.m.r32((l_work_1).wrapping_add(8))) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32((l_work_1).wrapping_add(8)); F::Heap_Free(c, a0_) };
    }
    F::BtlScript_InitWork(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_Free", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_IsEventRunning(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_IsEventRunning", &PROBE_N, &[], false); }
    let mut l_task_1: u32 = 0;
    let mut l_found_2: i32 = 0;
    let r_ = 'f: {
    l_found_2 = 0i32;
    {
        l_task_1 = F::List_GetHead(c, sym::gGscTaskList);
        'l1: loop {
            if !(((((l_task_1) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((if ((((((c.m.r32((l_task_1).wrapping_add(16)) as i32)) < (10000i32)) as i32)) != 0) { 0i32 } else { ((((c.m.r32((l_task_1).wrapping_add(16)) as i32)) < (10052i32)) as i32) })) != 0) {
                    l_found_2 = 1i32;
                    break 'l1;
                }
            }
            l_task_1 = F::List_GetNext(c, l_task_1);
        }
    }
    break 'f l_found_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_IsEventRunning", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_AbortEvents(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_AbortEvents", &PROBE_N, &[], false); }
    let mut l_task_1: u32 = 0;
    let r_ = 'f: {
    {
        l_task_1 = F::List_GetHead(c, sym::gGscTaskList);
        'l1: loop {
            if !(((((l_task_1) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((if ((((((c.m.r32((l_task_1).wrapping_add(16)) as i32)) < (10000i32)) as i32)) != 0) { 0i32 } else { ((((c.m.r32((l_task_1).wrapping_add(16)) as i32)) < (10052i32)) as i32) })) != 0) {
                    { let a0_ = (c.m.r32((l_task_1).wrapping_add(12)) as i32); F::Gsc_AbortTask(c, a0_) };
                }
            }
            l_task_1 = F::List_GetNext(c, l_task_1);
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_AbortEvents", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_SetStory(c: &mut Ctx, mut p_story_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_SetStory", &PROBE_N, &[(p_story_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_story_1; let a_ = sym::gBtlScript; c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_SetStory", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_GetStory(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_GetStory", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (c.m.r32(sym::gBtlScript) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_GetStory", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_RequestTextFile(c: &mut Ctx, mut p_id_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_RequestTextFile", &PROBE_N, &[(p_id_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((D::call(c, fid::File_Request, &[((p_id_1) as i64 as u64), ((((0i32) as u32)) as u64), ((0i32) as i64 as u64)])) as u32); let a_ = (sym::gBtlScript).wrapping_add(4); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_RequestTextFile", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_GetText(c: &mut Ctx, mut p_n_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_GetText", &PROBE_N, &[(p_n_1) as u32], false); }
    let mut l_file_2: u32 = 0;
    let r_ = 'f: {
    l_file_2 = c.m.r32((sym::gBtlScript).wrapping_add(4));
    break 'f (l_file_2).wrapping_add((rt::div_u32(c.m.r32((l_file_2).wrapping_add(((((p_n_1).wrapping_add(1i32)) as u32)).wrapping_mul(4))), ((4i32) as u32))).wrapping_mul(((4i32) as u32)));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_GetText", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_RequestFile8(c: &mut Ctx, mut p_id_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_RequestFile8", &PROBE_N, &[(p_id_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((D::call(c, fid::File_Request, &[((p_id_1) as i64 as u64), ((((0i32) as u32)) as u64), ((0i32) as i64 as u64)])) as u32); let a_ = (sym::gBtlScript).wrapping_add(8); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_RequestFile8", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_GetFile8Entry(c: &mut Ctx, mut p_n_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_GetFile8Entry", &PROBE_N, &[(p_n_1) as u32], false); }
    let mut l_file_2: u32 = 0;
    let r_ = 'f: {
    l_file_2 = c.m.r32((sym::gBtlScript).wrapping_add(8));
    break 'f (l_file_2).wrapping_add((rt::div_u32(c.m.r32((l_file_2).wrapping_add(((((p_n_1).wrapping_add(1i32)) as u32)).wrapping_mul(4))), ((4i32) as u32))).wrapping_mul(((4i32) as u32)));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_GetFile8Entry", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_GetWindow(c: &mut Ctx, mut p_n_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_GetWindow", &PROBE_N, &[(p_n_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((sym::gBtlScript).wrapping_add(852)).wrapping_add((((p_n_1) as u32)).wrapping_mul(80));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_GetWindow", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_SetText(c: &mut Ctx, mut p_window_1: i32, mut p_text_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_SetText", &PROBE_N, &[(p_window_1) as u32, (p_text_2) as u32], false); }
    let mut l_work_3: u32 = 0;
    let r_ = 'f: {
    l_work_3 = sym::gBtlScript;
    { let v_ = ((p_window_1) as i16); let a_ = (l_work_3).wrapping_add(392); c.m.w16(a_, (v_) as u16); };
    { let v_ = ((p_text_2) as i16); let a_ = (l_work_3).wrapping_add(394); c.m.w16(a_, (v_) as u16); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_SetText", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_ClearText(c: &mut Ctx, mut p_text_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_ClearText", &PROBE_N, &[(p_text_1) as u32], false); }
    let mut l_work_2: u32 = 0;
    let r_ = 'f: {
    l_work_2 = sym::gBtlScript;
    if ((((((((c.m.r16((l_work_2).wrapping_add(394)) as i16)) as i32)) == (p_text_1)) as i32)) != 0) {
        { let v_ = (((1i32).wrapping_neg()) as i16); let a_ = (l_work_2).wrapping_add(394); c.m.w16(a_, (v_) as u16); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_ClearText", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_SetSpeaker(c: &mut Ctx, mut p_n_1: i32, mut p_who_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_SetSpeaker", &PROBE_N, &[(p_n_1) as u32, (p_who_2) as u32], false); }
    let mut l_speaker_3: u32 = 0;
    let mut l_side_4: i32 = 0;
    let r_ = 'f: {
    l_speaker_3 = (((sym::gBtlScript).wrapping_add(852)).wrapping_add(160)).wrapping_add((((p_n_1) as u32)).wrapping_mul(4));
    l_side_4 = (((p_who_2).wrapping_shr((15i32) as u32)) & (1i32));
    { let v_ = ((l_side_4) as i16); let a_ = l_speaker_3; c.m.w16(a_, (v_) as u16); };
    { let v_ = ((((p_who_2) & (4095i32))) as i16); let a_ = (l_speaker_3).wrapping_add(2); c.m.w16(a_, (v_) as u16); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_SetSpeaker", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_GetSpeaker(c: &mut Ctx, mut p_n_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_GetSpeaker", &PROBE_N, &[(p_n_1) as u32], false); }
    let r_ = 'f: {
    break 'f (((sym::gBtlScript).wrapping_add(852)).wrapping_add(160)).wrapping_add((((p_n_1) as u32)).wrapping_mul(4));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_GetSpeaker", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_GetCurrentEvent(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_GetCurrentEvent", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f c.m.r32(((sym::gBtlScript).wrapping_add(384)).wrapping_add(4));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_GetCurrentEvent", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_ResetView(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_ResetView", &PROBE_N, &[], false); }
    let mut l_work_1: u32 = 0;
    let r_ = 'f: {
    l_work_1 = sym::gBtlScript;
    { let v_ = 0i32; let a_ = ((l_work_1).wrapping_add(852)).wrapping_add(2160); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((0i32) as u32); let a_ = (l_work_1).wrapping_add(848); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as i16); let a_ = (l_work_1).wrapping_add(392); c.m.w16(a_, (v_) as u16); };
    { let v_ = F32(0x00000000); let a_ = (l_work_1).wrapping_add(396); c.m.w32(a_, (v_).0); };
    { let v_ = 0i32; let a_ = (l_work_1).wrapping_add(400); c.m.w32(a_, (v_) as u32); };
    { let v_ = (((1i32).wrapping_neg()) as i16); let a_ = (l_work_1).wrapping_add(394); c.m.w16(a_, (v_) as u16); };
    F::BtlScript_StopVoices(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_ResetView", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_IsTriggered(c: &mut Ctx, mut p_trigger_1: i32, mut p_range_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_IsTriggered", &PROBE_N, &[(p_trigger_1) as u32, (p_range_2) as u32], false); }
    let mut l_side_3: i32 = 0;
    let mut l_result_4: i32 = 0;
    let mut l_ev_5: i32 = 0;
    let mut sv_6: i32 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
    'fl1: loop {
        match bb_ {
            0 => {
                l_side_3 = (((p_trigger_1).wrapping_shr((15i32) as u32)) & (1i32));
                l_result_4 = 0i32;
                l_ev_5 = ((p_trigger_1) & (4095i32));
                if (((((p_trigger_1) == (65535i32)) as i32)) != 0) { bb_ = 1; continue 'fl1; } else { bb_ = 3; continue 'fl1; }
            }
            1 => {
                if ((F::BtlFacade_IsWaitOff(c)) != 0) { bb_ = 4; continue 'fl1; } else { bb_ = 5; continue 'fl1; }
            }
            2 => {
                break 'f l_result_4;
            }
            3 => {
                if ((((!((p_range_2) != 0)) as i32)) != 0) { bb_ = 6; continue 'fl1; } else { bb_ = 7; continue 'fl1; }
            }
            4 => {
                l_result_4 = 1i32;
                bb_ = 5; continue 'fl1;
            }
            5 => {
                bb_ = 2; continue 'fl1;
            }
            6 => {
                bb_ = 8; continue 'fl1;
            }
            7 => {
                sv_6 = l_ev_5;
                bb_ = match sv_6 { 22i32 => 12, 23i32 => 13, 24i32 => 14, 25i32 => 15, 26i32 => 16, 27i32 => 17, 28i32 => 18, _ => 20, }; continue 'fl1;
            }
            8 => {
                l_result_4 = F::BtlEvent_IsNew(c, l_side_3, l_ev_5);
                bb_ = 10; continue 'fl1;
            }
            9 => {
                bb_ = 7; continue 'fl1;
            }
            10 => {
                bb_ = 2; continue 'fl1;
            }
            11 => {
                bb_ = 12; continue 'fl1;
            }
            12 => {
                l_result_4 = F::BtlEvent_IsNew(c, l_side_3, 22i32);
                bb_ = 13; continue 'fl1;
            }
            13 => {
                { let r_ = F::BtlEvent_IsNew(c, l_side_3, 23i32); let n_ = ((l_result_4) | (r_)); l_result_4 = n_; };
                bb_ = 14; continue 'fl1;
            }
            14 => {
                { let r_ = F::BtlEvent_IsNew(c, l_side_3, 24i32); let n_ = ((l_result_4) | (r_)); l_result_4 = n_; };
                bb_ = 15; continue 'fl1;
            }
            15 => {
                { let r_ = F::BtlEvent_IsNew(c, l_side_3, 25i32); let n_ = ((l_result_4) | (r_)); l_result_4 = n_; };
                bb_ = 16; continue 'fl1;
            }
            16 => {
                { let r_ = F::BtlEvent_IsNew(c, l_side_3, 26i32); let n_ = ((l_result_4) | (r_)); l_result_4 = n_; };
                bb_ = 17; continue 'fl1;
            }
            17 => {
                { let r_ = F::BtlEvent_IsNew(c, l_side_3, 27i32); let n_ = ((l_result_4) | (r_)); l_result_4 = n_; };
                bb_ = 18; continue 'fl1;
            }
            18 => {
                { let r_ = F::BtlEvent_IsNew(c, l_side_3, 28i32); let n_ = ((l_result_4) | (r_)); l_result_4 = n_; };
                bb_ = 10; continue 'fl1;
            }
            19 => {
                bb_ = 20; continue 'fl1;
            }
            20 => {
                bb_ = 8; continue 'fl1;
            }
            21 => {
                bb_ = 10; continue 'fl1;
            }
            22 => {
                break 'fl1;
            }
            _ => unreachable!(),
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_IsTriggered", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_GetVoiceFile(c: &mut Ctx, mut p_line_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_GetVoiceFile", &PROBE_N, &[(p_line_1) as u32], false); }
    let mut l_base_2: i32 = 0;
    let r_ = 'f: {
    l_base_2 = 3400i32;
    if (((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
        l_base_2 = 18400i32;
    }
    break 'f ((l_base_2).wrapping_add((F::BtlScript_GetStory(c)).wrapping_mul(300i32))).wrapping_add(p_line_1);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_GetVoiceFile", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_AddLine(c: &mut Ctx, mut p_trigger_1: i32, mut p_line_2: i32, mut p_once_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_AddLine", &PROBE_N, &[(p_trigger_1) as u32, (p_line_2) as u32, (p_once_3) as u32], false); }
    let mut l_talk_4: u32 = 0;
    let mut l_req_5: u32 = 0;
    let mut l_on_6: i32 = 0;
    let r_ = 'f: {
    l_talk_4 = (sym::gBtlScript).wrapping_add(852);
    l_req_5 = ((l_talk_4).wrapping_add(1360)).wrapping_add(((((c.m.r32((l_talk_4).wrapping_add(2160)) as i32)) as u32)).wrapping_mul(16));
    l_on_6 = 1i32;
    { let v_ = p_trigger_1; let a_ = (l_req_5).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_on_6; let a_ = l_req_5; c.m.w32(a_, (v_) as u32); };
    { let v_ = ((l_on_6) as i8); let a_ = (l_req_5).wrapping_add(8); c.m.w8(a_, (v_) as u8); };
    { let v_ = p_line_2; let a_ = ((l_req_5).wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((p_once_3) as i8); let a_ = ((l_req_5).wrapping_add(8)).wrapping_add(1); c.m.w8(a_, (v_) as u8); };
    { let a_ = (l_talk_4).wrapping_add(2160); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_AddLine", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_AddEvent(c: &mut Ctx, mut p_trigger_1: i32, mut p_action_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_AddEvent", &PROBE_N, &[(p_trigger_1) as u32, (p_action_2) as u32], false); }
    let mut l_talk_3: u32 = 0;
    let mut l_req_4: u32 = 0;
    let mut l_work_5: u32 = 0;
    let r_ = 'f: {
    l_talk_3 = (sym::gBtlScript).wrapping_add(852);
    l_req_4 = ((l_talk_3).wrapping_add(1360)).wrapping_add(((((c.m.r32((l_talk_3).wrapping_add(2160)) as i32)) as u32)).wrapping_mul(16));
    l_work_5 = (l_talk_3).wrapping_sub(((852i32) as u32));
    { let v_ = 2i32; let a_ = l_req_4; c.m.w32(a_, (v_) as u32); };
    { let v_ = p_trigger_1; let a_ = (l_req_4).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_action_2; let a_ = (l_req_4).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((l_work_5).wrapping_add(332)).wrapping_add((((p_action_2).wrapping_sub(10000i32)) as u32)); let a_ = ((l_req_4).wrapping_add(8)).wrapping_add(4); c.m.w32(a_, v_); };
    { let a_ = (l_talk_3).wrapping_add(2160); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_AddEvent", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_IsLineVoiceStopped(c: &mut Ctx, mut p_line_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_IsLineVoiceStopped", &PROBE_N, &[(p_line_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let a2_ = { let a0_ = (c.m.r32((p_line_1).wrapping_add(4)) as i32); F::BtlScript_GetSpeaker(c, a0_) }; F::BtlScript_GetSpeakerSlot(c, fp, fp.wrapping_add(4), a2_) };
        break 'f ((({ let a0_ = (c.m.r32(fp) as i32); F::Voice_IsStopped(c, a0_) }) != (0i32)) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_IsLineVoiceStopped", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_PlayLine(c: &mut Ctx, mut p_line_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_PlayLine", &PROBE_N, &[(p_line_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_work_2: u32 = 0;
    let mut l_speaker_3: u32 = 0;
    let mut l_file_4: i32 = 0;
    let r_ = 'f: {
        l_work_2 = sym::gBtlScript;
        l_speaker_3 = { let a0_ = (c.m.r32((p_line_1).wrapping_add(4)) as i32); F::BtlScript_GetSpeaker(c, a0_) };
        l_file_4 = { let a0_ = (c.m.r32((p_line_1).wrapping_add(4)) as i32); F::BtlScript_GetVoiceFile(c, a0_) };
        F::BtlScript_GetSpeakerSlot(c, fp, fp.wrapping_add(4), l_speaker_3);
        if ((((({ let a_ = ((l_work_2).wrapping_add(3016)).wrapping_add(((((((c.m.r32(fp) as i32)).wrapping_mul(2i32)).wrapping_add((c.m.r32(fp.wrapping_add(4)) as i32))) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
            { let a0_ = (c.m.r32(fp) as i32); F::Voice_PlayDefault(c, a0_, l_file_4) };
            { let v_ = p_line_1; let a_ = ((l_work_2).wrapping_add(3016)).wrapping_add(((((((c.m.r32(fp) as i32)).wrapping_mul(2i32)).wrapping_add((c.m.r32(fp.wrapping_add(4)) as i32))) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            { let a0_ = (c.m.r32(fp.wrapping_add(4)) as i32); let a1_ = (c.m.r32((p_line_1).wrapping_add(4)) as i32); F::BtlScript_SetText(c, a0_, a1_) };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_PlayLine", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_GetSpeakerSlot(c: &mut Ctx, mut p_side_1: u32, mut p_window_2: u32, mut p_speaker_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_GetSpeakerSlot", &PROBE_N, &[(p_side_1) as u32, (p_window_2) as u32, (p_speaker_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = (((c.m.r16(p_speaker_3) as i16)) as i32); let a_ = p_side_1; c.m.w32(a_, (v_) as u32); };
    if ((((((((((((c.m.r16((p_speaker_3).wrapping_add(2)) as i16)) as i32)) != (32767i32)) as i32)) != 0) && ((((({ let a0_ = (((c.m.r16(p_speaker_3) as i16)) as i32); F::BtlCtrl_GetActiveMember(c, a0_) }) == ((((c.m.r16((p_speaker_3).wrapping_add(2)) as i16)) as i32))) as i32)) != 0)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = p_window_2; c.m.w32(a_, (v_) as u32); };
    } else {
        { let v_ = 1i32; let a_ = p_window_2; c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_GetSpeakerSlot", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_IsLinePlaying(c: &mut Ctx, mut p_line_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_IsLinePlaying", &PROBE_N, &[(p_line_1) as u32], false); }
    let mut l_work_2: u32 = 0;
    let mut l_result_3: i32 = 0;
    let r_ = 'f: {
    l_work_2 = sym::gBtlScript;
    l_result_3 = 0i32;
    if (((((((((((((((((c.m.r32(((l_work_2).wrapping_add(3016)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) || (((((c.m.r32(((l_work_2).wrapping_add(3016)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) || (((((c.m.r32(((l_work_2).wrapping_add(3016)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) || (((((c.m.r32(((l_work_2).wrapping_add(3016)).wrapping_add((((3i32) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        l_result_3 = 1i32;
    }
    break 'f l_result_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_IsLinePlaying", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_StartWaitEvents(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_StartWaitEvents", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_ev_2: u32 = 0;
    let mut l_work_3: u32 = 0;
    let mut l_talk_4: u32 = 0;
    let mut l_req_5: u32 = 0;
    let mut l_cur_6: u32 = 0;
    let r_ = 'f: {
    l_i_1 = 0i32;
    l_ev_2 = F::Battle_GetEventWork(c);
    l_work_3 = sym::gBtlScript;
    l_talk_4 = (l_work_3).wrapping_add(852);
    {
        l_req_5 = (l_talk_4).wrapping_add(1360);
        'l1: loop {
            if !(((((l_i_1) < ((c.m.r32((l_talk_4).wrapping_add(2160)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((((((c.m.r32(l_req_5) as i32)) == (2i32)) as i32)) != 0) && ((((((c.m.r32((l_req_5).wrapping_add(4)) as i32)) == (65535i32)) as i32)) != 0)) as i32)) != 0) {
                    l_cur_6 = (l_work_3).wrapping_add(384);
                    { let s_ = (l_req_5).wrapping_add(8); let d_ = l_cur_6; rt::memcpy(c, d_, s_, 8); };
                    { let a0_ = (c.m.r32((l_ev_2).wrapping_add(4)) as i32); F::Gsc_KillTask(c, a0_) };
                    F::BtlScript_ResetView(c);
                    { let v_ = { let a0_ = (c.m.r32(l_ev_2) as i32); let a1_ = (c.m.r32(l_cur_6) as i32); F::Gsc_StartAction(c, a0_, a1_) }; let a_ = (l_ev_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                }
            }
            { { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; }; { let o_ = l_req_5; let n_ = (o_).wrapping_add(16); l_req_5 = n_; }; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_StartWaitEvents", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_UpdateEvents(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_UpdateEvents", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::BtlScript_UpdateLines(c);
    F::BtlScript_UpdateRequests(c);
    F::BtlScript_StartPending(c);
    F::BtlScript_UpdateHud(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_UpdateEvents", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_UpdateLines(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_UpdateLines", &PROBE_N, &[], false); }
    let mut l_work_1: u32 = 0;
    let mut l_line_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    l_work_1 = sym::gBtlScript;
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_line_2 = c.m.r32(((l_work_1).wrapping_add(3016)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)));
                if (((((((((l_line_2) != (((0i32) as u32))) as i32)) != 0) && ((F::BtlScript_IsLineVoiceStopped(c, l_line_2)) != 0)) as i32)) != 0) {
                    { let r_ = 10i32; let a_ = (l_line_2).wrapping_add(2); let n_ = ((((((c.m.r16(a_) as i16)) as i32)).wrapping_add(r_)) as i16); c.m.w16(a_, (n_) as u16); };
                    if ((((((((c.m.r16((l_line_2).wrapping_add(2)) as i16)) as i32)) > (600i32)) as i32)) != 0) {
                        { let a0_ = (c.m.r32((l_line_2).wrapping_add(4)) as i32); F::BtlScript_ClearText(c, a0_) };
                        { let v_ = ((0i32) as i16); let a_ = (l_line_2).wrapping_add(2); c.m.w16(a_, (v_) as u16); };
                        if (((c.m.r8((l_line_2).wrapping_add(1)) as i8)) != 0) {
                            { let v_ = ((0i32) as i8); let a_ = l_line_2; c.m.w8(a_, (v_) as u8); };
                        }
                        { let v_ = ((0i32) as u32); let a_ = ((l_work_1).wrapping_add(3016)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    }
                }
            }
            l_i_3 = ((!(l_i_3))).wrapping_neg();
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_UpdateLines", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_UpdateRequests(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_UpdateRequests", &PROBE_N, &[], false); }
    let mut l_work_1: u32 = 0;
    let mut l_talk_2: u32 = 0;
    let mut l_req_3: u32 = 0;
    let mut l_line_4: u32 = 0;
    let mut l_start_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_kind_7: i32 = 0;
    let r_ = 'f: {
    l_work_1 = sym::gBtlScript;
    l_talk_2 = (l_work_1).wrapping_add(852);
    {
        { l_i_6 = 0i32; l_req_3 = (l_talk_2).wrapping_add(1360); };
        'l1: loop {
            if !(((((l_i_6) < ((c.m.r32((l_talk_2).wrapping_add(2160)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                l_kind_7 = (c.m.r32(l_req_3) as i32);
                's3: {
                    let sv8: i32 = l_kind_7;
                    match sv8 {
                        1i32 => {
                            l_line_4 = (l_req_3).wrapping_add(8);
                            if (((c.m.r8(l_line_4) as i8)) != 0) {
                                if ((((((((!((F::BtlScript_IsLinePlaying(c, l_line_4)) != 0)) as i32)) != 0) && (({ let a0_ = (c.m.r32((l_req_3).wrapping_add(4)) as i32); F::BtlScript_IsTriggered(c, a0_, 0i32) }) != 0)) as i32)) != 0) {
                                    F::BtlScript_PlayLine(c, l_line_4);
                                }
                            }
                            break 's3;
                        }
                        2i32 => {
                            l_start_5 = (l_req_3).wrapping_add(8);
                            if (({ let a0_ = (c.m.r32((l_req_3).wrapping_add(4)) as i32); F::BtlScript_IsTriggered(c, a0_, 1i32) }) != 0) {
                                if (((((c.m.r32((l_work_1).wrapping_add(848))) == (((0i32) as u32))) as i32)) != 0) {
                                    { let v_ = l_req_3; let a_ = (l_work_1).wrapping_add(848); c.m.w32(a_, v_); };
                                } else {
                                    if ((((((((((((((((((({ let a_ = c.m.r32((l_start_5).wrapping_add(4)); (rt::bf_get_u(c, a_, 1, 3) as u8) }) as i32)) == (1i32)) as i32)) != 0) || ((((((({ let a_ = c.m.r32((l_start_5).wrapping_add(4)); (rt::bf_get_u(c, a_, 1, 3) as u8) }) as i32)) == (2i32)) as i32)) != 0)) as i32)) != 0) || ((((((({ let a_ = c.m.r32((l_start_5).wrapping_add(4)); (rt::bf_get_u(c, a_, 1, 3) as u8) }) as i32)) == (3i32)) as i32)) != 0)) as i32)) != 0) || ((((((({ let a_ = c.m.r32((l_start_5).wrapping_add(4)); (rt::bf_get_u(c, a_, 1, 3) as u8) }) as i32)) == (4i32)) as i32)) != 0)) as i32)) != 0) {
                                        { let v_ = l_req_3; let a_ = (l_work_1).wrapping_add(848); c.m.w32(a_, v_); };
                                    }
                                    's4: {
                                        let sv9: i32 = (((c.m.r32((l_req_3).wrapping_add(4)) as i32)) & (4095i32));
                                        match sv9 {
                                            72i32 => {
                                                { let v_ = l_req_3; let a_ = (l_work_1).wrapping_add(848); c.m.w32(a_, v_); };
                                                break 's4;
                                            }
                                            74i32 => {
                                                { let v_ = l_req_3; let a_ = (l_work_1).wrapping_add(848); c.m.w32(a_, v_); };
                                                break 's4;
                                            }
                                            _ => {
                                            }
                                        }
                                    }
                                }
                            }
                            break 's3;
                        }
                        _ => {
                        }
                    }
                }
            }
            { { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; }; { let o_ = l_req_3; let n_ = (o_).wrapping_add(16); l_req_3 = n_; }; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_UpdateRequests", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_StartPending(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_StartPending", &PROBE_N, &[], false); }
    let mut l_ok_1: i32 = 0;
    let mut l_ev_2: u32 = 0;
    let mut l_work_3: u32 = 0;
    let mut l_info_4: u32 = 0;
    let mut l_cur_5: u32 = 0;
    let mut l_ready_6: i32 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
    'fl1: loop {
        match bb_ {
            0 => {
                l_ok_1 = 0i32;
                l_ev_2 = F::Battle_GetEventWork(c);
                l_work_3 = sym::gBtlScript;
                if (((((c.m.r32((l_work_3).wrapping_add(848))) != (((0i32) as u32))) as i32)) != 0) { bb_ = 1; continue 'fl1; } else { bb_ = 2; continue 'fl1; }
            }
            1 => {
                l_info_4 = { let a_ = ((c.m.r32((l_work_3).wrapping_add(848))).wrapping_add(8)).wrapping_add(4); c.m.r32(a_) };
                if (((rt::bf_get_u(c, l_info_4, 4, 1) as u8)) != 0) { bb_ = 3; continue 'fl1; } else { bb_ = 5; continue 'fl1; }
            }
            2 => {
                break 'fl1;
            }
            3 => {
                if ((((((((((((({ let a_ = (c.m.r32((l_work_3).wrapping_add(848))).wrapping_add(4); (c.m.r32(a_) as i32) }) == (65535i32)) as i32)) != 0) || ((((((({ let a_ = (c.m.r32((l_work_3).wrapping_add(848))).wrapping_add(4); (c.m.r32(a_) as i32) }) & (4095i32))) == (34i32)) as i32)) != 0)) as i32)) != 0) || ((((((({ let a_ = (c.m.r32((l_work_3).wrapping_add(848))).wrapping_add(4); (c.m.r32(a_) as i32) }) & (4095i32))) == (73i32)) as i32)) != 0)) as i32)) != 0) { bb_ = 6; continue 'fl1; } else { bb_ = 8; continue 'fl1; }
            }
            4 => {
                l_cur_5 = (l_work_3).wrapping_add(384);
                if ((l_ok_1) != 0) { bb_ = 32; continue 'fl1; } else { bb_ = 33; continue 'fl1; }
            }
            5 => {
                if (((rt::bf_get_u(c, l_info_4, 6, 2) as u8)) != 0) { bb_ = 23; continue 'fl1; } else { bb_ = 25; continue 'fl1; }
            }
            6 => {
                l_ok_1 = 1i32;
                bb_ = 7; continue 'fl1;
            }
            7 => {
                bb_ = 4; continue 'fl1;
            }
            8 => {
                l_ready_6 = F::BtlFacade_AreBothInterruptible(c);
                bb_ = 9; continue 'fl1;
            }
            9 => {
                if ((l_ready_6) != 0) { bb_ = 10; continue 'fl1; } else { bb_ = 11; continue 'fl1; }
            }
            10 => {
                if ((F::BtlCtrl_IsActiveDead(c, 0i32)) != 0) { bb_ = 12; continue 'fl1; } else { bb_ = 14; continue 'fl1; }
            }
            11 => {
                bb_ = 7; continue 'fl1;
            }
            12 => {
                l_ok_1 = 1i32;
                bb_ = 13; continue 'fl1;
            }
            13 => {
                bb_ = 11; continue 'fl1;
            }
            14 => {
                if ((F::BtlCtrl_IsActiveDead(c, 1i32)) != 0) { bb_ = 15; continue 'fl1; } else { bb_ = 17; continue 'fl1; }
            }
            15 => {
                l_ok_1 = 1i32;
                bb_ = 16; continue 'fl1;
            }
            16 => {
                bb_ = 13; continue 'fl1;
            }
            17 => {
                if (((rt::bf_get_u(c, l_info_4, 5, 1) as u8)) != 0) { bb_ = 18; continue 'fl1; } else { bb_ = 20; continue 'fl1; }
            }
            18 => {
                l_ok_1 = 1i32;
                bb_ = 19; continue 'fl1;
            }
            19 => {
                bb_ = 16; continue 'fl1;
            }
            20 => {
                if ((F::BtlEvent_IsNew(c, 0i32, 80i32)) != 0) { bb_ = 21; continue 'fl1; } else { bb_ = 22; continue 'fl1; }
            }
            21 => {
                l_ok_1 = 1i32;
                bb_ = 22; continue 'fl1;
            }
            22 => {
                bb_ = 19; continue 'fl1;
            }
            23 => {
                l_ready_6 = { let a0_ = (((rt::bf_get_u(c, l_info_4, 5, 1) as u8)) as i32); F::BtlFacade_CanCharAct(c, a0_) };
                bb_ = 9; continue 'fl1;
            }
            24 => {
                bb_ = 4; continue 'fl1;
            }
            25 => {
                if ((((((F::BtlCtrl_IsActiveDead(c, 0i32)) != 0) || ((F::BtlCtrl_IsActiveDead(c, 1i32)) != 0)) as i32)) != 0) { bb_ = 27; continue 'fl1; } else { bb_ = 29; continue 'fl1; }
            }
            26 => {
                bb_ = 24; continue 'fl1;
            }
            27 => {
                if ((F::BtlFacade_AreBothInterruptible(c)) != 0) { bb_ = 30; continue 'fl1; } else { bb_ = 31; continue 'fl1; }
            }
            28 => {
                bb_ = 24; continue 'fl1;
            }
            29 => {
                l_ok_1 = 1i32;
                bb_ = 28; continue 'fl1;
            }
            30 => {
                l_ok_1 = 1i32;
                bb_ = 31; continue 'fl1;
            }
            31 => {
                bb_ = 28; continue 'fl1;
            }
            32 => {
                { let s_ = (c.m.r32((l_work_3).wrapping_add(848))).wrapping_add(8); let d_ = l_cur_5; rt::memcpy(c, d_, s_, 8); };
                { let a0_ = (c.m.r32((l_ev_2).wrapping_add(4)) as i32); F::Gsc_KillTask(c, a0_) };
                F::BtlScript_ResetView(c);
                { let v_ = { let a0_ = (c.m.r32(l_ev_2) as i32); let a1_ = (c.m.r32(l_cur_5) as i32); F::Gsc_StartAction(c, a0_, a1_) }; let a_ = (l_ev_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                bb_ = 33; continue 'fl1;
            }
            33 => {
                bb_ = 2; continue 'fl1;
            }
            _ => unreachable!(),
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_StartPending", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_UpdateHud(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_UpdateHud", &PROBE_N, &[], false); }
    let mut l_work_1: u32 = 0;
    let r_ = 'f: {
    l_work_1 = sym::gBtlScript;
    if (((((c.m.r32((l_work_1).wrapping_add(848))) != (((0i32) as u32))) as i32)) != 0) {
        if ((((((((!((F::BtlCtrl_IsActiveDead(c, 0i32)) != 0)) as i32)) != 0) && ((((!((F::BtlCtrl_IsActiveDead(c, 1i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
            if ((((!(({ let a_ = { let a_ = ((c.m.r32((l_work_1).wrapping_add(848))).wrapping_add(8)).wrapping_add(4); c.m.r32(a_) }; (rt::bf_get_u(c, a_, 5, 1) as u8) }) != 0)) as i32)) != 0) {
                F::HudPrompt_ShowCue(c);
                { let a0_ = F::BtlFacade_CanCharAct(c, 0i32); F::HudPrompt_SetCueDim(c, a0_) };
                break 'f;
            }
        }
    }
    F::HudPrompt_HideCue(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_UpdateHud", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_IsSlotBusy(c: &mut Ctx, mut p_side_1: i32, mut p_window_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_IsSlotBusy", &PROBE_N, &[(p_side_1) as u32, (p_window_2) as u32], false); }
    let r_ = 'f: {
    break 'f (((c.m.r32(((sym::gBtlScript).wrapping_add(3016)).wrapping_add((((((p_side_1).wrapping_mul(2i32)).wrapping_add(p_window_2)) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_IsSlotBusy", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_StopVoice(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_StopVoice", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((0i32) as u32); let a_ = ((sym::gBtlScript).wrapping_add(3016)).wrapping_add(((((p_side_1).wrapping_mul(2i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    F::Voice_Stop(c, p_side_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_StopVoice", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_StopVoices(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_StopVoices", &PROBE_N, &[], false); }
    let mut l_work_1: u32 = 0;
    let r_ = 'f: {
    l_work_1 = sym::gBtlScript;
    { let v_ = ((0i32) as u32); let a_ = ((l_work_1).wrapping_add(3016)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = ((l_work_1).wrapping_add(3016)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = ((l_work_1).wrapping_add(3016)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = ((l_work_1).wrapping_add(3016)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    F::Voice_Stop(c, 0i32);
    F::Voice_Stop(c, 1i32);
    F::StreamSe_Stop(c, 0i32);
    F::StreamSe_Stop(c, 1i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_StopVoices", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScript_IsTextShown(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_IsTextShown", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f ((((((c.m.r16((sym::gBtlScript).wrapping_add(394)) as i16)) as i32)) >= (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScript_IsTextShown", &PROBE_N, &[], true); }
    r_
}
