#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn late_a__Shen_DebugFile(c: &mut Ctx, mut p_name_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "late_a__Shen_DebugFile", &PROBE_N, &[(p_name_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "late_a__Shen_DebugFile", &PROBE_N, &[], true); }
    r_
}

pub fn Shen_Main(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Shen_Main", &PROBE_N, &[], false); }
    let mut l_done_1: i32 = 0;
    let mut l_ret_2: i32 = 0;
    let r_ = 'f: {
    l_done_1 = 0i32;
    l_ret_2 = 1i32;
    if (((((c.m.r32(sym::gMenuArc12)) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a0_ = ({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_add(24i32); F::File_LoadSync(c, a0_, ((0i32) as u32), 0i32) }; let a_ = sym::gMenuArc12; c.m.w32(a_, v_); };
    }
    'l1: loop {
        'c2: {
            's3: {
                let sv3: i32 = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); (c.m.r32(a_) as i32) };
                match sv3 {
                    70i32 => {
                        F::Bgm_Play(c, 68376i32);
                        F::Pad_SetRepeat(c, 20i32, 1i32);
                        { let _ = F::Shen_Run(c, 2i32); };
                        F::Adx_StopAll(c);
                        F::Pad_SetRepeat(c, 40i32, 3i32);
                        { let v_ = 71i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        break 's3;
                    }
                    71i32 => {
                        { let _ = F::ShenSave_Run(c, 2i32); };
                        l_ret_2 = 0i32;
                        F::Adx_StopAll(c);
                        l_done_1 = 1i32;
                        { let v_ = 4i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        break 's3;
                    }
                    _ => {
                    }
                }
            }
            { let _ = F::sceGsSyncPath(c, 0i32, ((0i32) as u16)); };
        }
        if !((((!((l_done_1) != 0)) as i32)) != 0) { break 'l1; }
    }
    if (((((c.m.r32(sym::gMenuArc12)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gMenuArc12); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gMenuArc12; c.m.w32(a_, v_); };
    }
    break 'f l_ret_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Shen_Main", &PROBE_N, &[], true); }
    r_
}

pub fn ShenList_Free(c: &mut Ctx, mut p_head_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ShenList_Free", &PROBE_N, &[(p_head_1) as u32], false); }
    let mut l_node_2: u32 = 0;
    let mut l_next_3: u32 = 0;
    let r_ = 'f: {
    l_node_2 = c.m.r32((p_head_1).wrapping_add(16));
    'l1: while (((((c.m.r32(l_node_2)) != (((0i32) as u32))) as i32)) != 0) {
        l_next_3 = c.m.r32((l_node_2).wrapping_add(16));
        F::Heap_Free(c, l_node_2);
        l_node_2 = l_next_3;
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ShenList_Free", &PROBE_N, &[], true); }
    r_
}

pub fn ShenList_Build(c: &mut Ctx, mut p_head_1: u32, mut p_count_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ShenList_Build", &PROBE_N, &[(p_head_1) as u32, (p_count_2) as u32], false); }
    let mut l_i_3: u32 = 0;
    let mut l_cur_4: u32 = 0;
    let mut l_n_5: i32 = 0;
    let mut l_prev_6: u32 = 0;
    let mut l_first_7: u32 = 0;
    let mut l_node_8: u32 = 0;
    let r_ = 'f: {
    l_i_3 = ((1i32) as u32);
    l_cur_4 = p_head_1;
    { let v_ = ((0i32) as u32); let a_ = l_cur_4; c.m.w32(a_, v_); };
    l_n_5 = 0i32;
    l_prev_6 = ((0i32) as u32);
    l_first_7 = p_head_1;
    l_node_8 = ((0i32) as u32);
    {
        'l1: loop {
            if !(((((l_i_3) < (p_count_2)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_node_8 = F::Heap_Alloc(c, ((20u32) as i32), ((32i32) as u32), 0i32, 2i32);
                { let _ = F::memset(c, l_node_8, 0i32, 20u32); };
                { let o_ = l_n_5; let n_ = (o_).wrapping_add(1); l_n_5 = n_; };
                { let v_ = l_i_3; let a_ = l_node_8; c.m.w32(a_, v_); };
                { let v_ = l_prev_6; let a_ = (l_cur_4).wrapping_add(12); c.m.w32(a_, v_); };
                { let v_ = l_node_8; let a_ = (l_cur_4).wrapping_add(16); c.m.w32(a_, v_); };
                l_prev_6 = l_cur_4;
                if (((((((l_n_5) as u32)) != ((p_count_2).wrapping_sub(((1i32) as u32)))) as i32)) != 0) {
                    l_cur_4 = l_node_8;
                }
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    { let v_ = l_node_8; let a_ = (l_first_7).wrapping_add(12); c.m.w32(a_, v_); };
    { let v_ = l_first_7; let a_ = (l_node_8).wrapping_add(16); c.m.w32(a_, v_); };
    { let v_ = l_cur_4; let a_ = (l_node_8).wrapping_add(12); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ShenList_Build", &PROBE_N, &[], true); }
    r_
}

pub fn ShenList_Find(c: &mut Ctx, mut p_head_1: u32, mut p_no_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ShenList_Find", &PROBE_N, &[(p_head_1) as u32, (p_no_2) as u32], false); }
    let mut l_node_3: u32 = 0;
    let r_ = 'f: {
    l_node_3 = p_head_1;
    if (((((((((p_no_2) < (0i32)) as i32)) != 0) && ((((({ let a_ = c.m.r32((l_node_3).wrapping_add(12)); c.m.r32(a_) }) < (((p_no_2) as u32))) as i32)) != 0)) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    if (((((c.m.r32(p_head_1)) != (((p_no_2) as u32))) as i32)) != 0) {
        'l1: loop {
            'c2: {
                l_node_3 = c.m.r32((l_node_3).wrapping_add(16));
            }
            if !(((((c.m.r32(l_node_3)) != (((p_no_2) as u32))) as i32)) != 0) { break 'l1; }
        }
    }
    break 'f l_node_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ShenList_Find", &PROBE_N, &[], true); }
    r_
}

pub fn ShenList_StepWish(c: &mut Ctx, mut p_node_1: u32, mut p_steps_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ShenList_StepWish", &PROBE_N, &[(p_node_1) as u32, (p_steps_2) as u32], false); }
    let mut l_dir_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_dir_3 = p_steps_2;
    l_i_4 = 0i32;
    if (((((p_steps_2) == (0i32)) as i32)) != 0) {
        break 'f (p_node_1).wrapping_add(4);
    }
    if (((((p_steps_2) < (0i32)) as i32)) != 0) {
        p_steps_2 = (p_steps_2).wrapping_neg();
    }
    {
        'l1: loop {
            if !(((((l_i_4) != (p_steps_2)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((l_dir_3) < (0i32)) as i32)) != 0) {
                    p_node_1 = c.m.r32((p_node_1).wrapping_add(12));
                } else {
                    if (((((l_dir_3) > (0i32)) as i32)) != 0) {
                        p_node_1 = c.m.r32((p_node_1).wrapping_add(16));
                    }
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    break 'f (p_node_1).wrapping_add(4);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ShenList_StepWish", &PROBE_N, &[], true); }
    r_
}

pub fn ShenList_Step(c: &mut Ctx, mut p_node_1: u32, mut p_steps_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ShenList_Step", &PROBE_N, &[(p_node_1) as u32, (p_steps_2) as u32], false); }
    let mut l_dir_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_dir_3 = p_steps_2;
    l_i_4 = 0i32;
    if (((((p_steps_2) == (0i32)) as i32)) != 0) {
        break 'f p_node_1;
    }
    if (((((p_steps_2) < (0i32)) as i32)) != 0) {
        p_steps_2 = (p_steps_2).wrapping_neg();
    }
    {
        'l1: loop {
            if !(((((l_i_4) != (p_steps_2)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((l_dir_3) < (0i32)) as i32)) != 0) {
                    p_node_1 = c.m.r32((p_node_1).wrapping_add(12));
                } else {
                    if (((((l_dir_3) > (0i32)) as i32)) != 0) {
                        p_node_1 = c.m.r32((p_node_1).wrapping_add(16));
                    }
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    break 'f p_node_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ShenList_Step", &PROBE_N, &[], true); }
    r_
}

pub fn Shen_Say(c: &mut Ctx, mut p_work_1: u32, mut p_line_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Shen_Say", &PROBE_N, &[(p_work_1) as u32, (p_line_2) as u32], false); }
    let r_ = 'f: {
    F::Voice_PlayWithSubtitle(c, ((0i32) as u32), 34231i32, p_line_2);
    { let v_ = p_line_2; let a_ = (p_work_1).wrapping_add(864); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Shen_Say", &PROBE_N, &[], true); }
    r_
}

pub fn Shen_ClampCursor(c: &mut Ctx, mut p_cursor_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Shen_ClampCursor", &PROBE_N, &[(p_cursor_1) as u32], false); }
    let mut l_max_2: i32 = 0;
    let r_ = 'f: {
    l_max_2 = 3i32;
    if ((((((c.m.r32(p_cursor_1) as i32)) < (0i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = p_cursor_1; c.m.w32(a_, (v_) as u32); };
        break 'f 1i32;
    }
    if ((((((c.m.r32(p_cursor_1) as i32)) > (l_max_2)) as i32)) != 0) {
        { let v_ = l_max_2; let a_ = p_cursor_1; c.m.w32(a_, (v_) as u32); };
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Shen_ClampCursor", &PROBE_N, &[], true); }
    r_
}

pub fn Shen_PlayPlateOk(c: &mut Ctx, mut p_work_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Shen_PlayPlateOk", &PROBE_N, &[(p_work_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(272) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 272);
    let r_ = 'f: {
        { let _ = { let a2_ = ((c.m.r32((p_work_1).wrapping_add(916)) as i32)).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::gShenPlateFmt, &[((a2_) as i64 as u64)]) }; };
        F::Flash_FindLabel(c, (p_work_1).wrapping_add(8), ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, (p_work_1).wrapping_add(8), fp, sym::late_a__str0);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Shen_PlayPlateOk", &PROBE_N, &[], true); }
    r_
}

pub fn Shen_SetPlate(c: &mut Ctx, mut p_flash_1: u32, mut p_row_2: i32, mut p_on_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Shen_SetPlate", &PROBE_N, &[(p_flash_1) as u32, (p_row_2) as u32, (p_on_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(272) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 272);
    let r_ = 'f: {
        { let _ = F::sprintf(c, fp.wrapping_add(8), sym::gShenPlateFmt, &[(((p_row_2).wrapping_add(1i32)) as i64 as u64)]); };
        F::Flash_FindLabel(c, p_flash_1, ((0i32) as u32), fp.wrapping_add(8), fp);
        if ((p_on_3) != 0) {
            F::Flash_ClipGotoLabel(c, p_flash_1, fp, sym::late_a__str1);
        } else {
            F::Flash_ClipGotoLabel(c, p_flash_1, fp, sym::late_a__str2);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Shen_SetPlate", &PROBE_N, &[], true); }
    r_
}

pub fn Shen_MoveCursor(c: &mut Ctx, mut p_work_1: u32, mut p_dir_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Shen_MoveCursor", &PROBE_N, &[(p_work_1) as u32, (p_dir_2) as u32], false); }
    let r_ = 'f: {
    { let a1_ = (c.m.r32((p_work_1).wrapping_add(916)) as i32); F::Shen_SetPlate(c, (p_work_1).wrapping_add(8), a1_, 0i32) };
    if (((((p_dir_2) == (0i32)) as i32)) != 0) {
        { let a_ = (p_work_1).wrapping_add(916); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    } else {
        if (((((p_dir_2) == (1i32)) as i32)) != 0) {
            { let a_ = (p_work_1).wrapping_add(916); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
    }
    if ((F::Shen_ClampCursor(c, (p_work_1).wrapping_add(916))) != 0) {
        if (((((p_dir_2) == (0i32)) as i32)) != 0) {
            { let v_ = { let a_ = (c.m.r32((p_work_1).wrapping_add(908))).wrapping_add(12); c.m.r32(a_) }; let a_ = (p_work_1).wrapping_add(908); c.m.w32(a_, v_); };
            F::Flash_GotoLabel(c, (p_work_1).wrapping_add(8), sym::late_a__str3, ((((1i32) as i64 as u64)) as u8));
            { let v_ = { let a0_ = c.m.r32((p_work_1).wrapping_add(908)); F::ShenList_Step(c, a0_, 4i32) }; let a_ = (p_work_1).wrapping_add(912); c.m.w32(a_, v_); };
        } else {
            if (((((p_dir_2) == (1i32)) as i32)) != 0) {
                { let v_ = { let a_ = (c.m.r32((p_work_1).wrapping_add(908))).wrapping_add(16); c.m.r32(a_) }; let a_ = (p_work_1).wrapping_add(908); c.m.w32(a_, v_); };
                F::Flash_GotoLabel(c, (p_work_1).wrapping_add(8), sym::late_a__str4, ((((1i32) as i64 as u64)) as u8));
                { let v_ = { let a0_ = c.m.r32((p_work_1).wrapping_add(908)); F::ShenList_Step(c, a0_, (1i32).wrapping_neg()) }; let a_ = (p_work_1).wrapping_add(912); c.m.w32(a_, v_); };
            }
        }
    }
    { let a1_ = (c.m.r32((p_work_1).wrapping_add(916)) as i32); F::Shen_SetPlate(c, (p_work_1).wrapping_add(8), a1_, 1i32) };
    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Shen_MoveCursor", &PROBE_N, &[], true); }
    r_
}

pub fn late_a__Shen_SetClipUv(c: &mut Ctx, mut p_flash_1: u32, mut p_parent_2: u32, mut p_name_3: u32, mut p_uv_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "late_a__Shen_SetClipUv", &PROBE_N, &[(p_flash_1) as u32, (p_parent_2) as u32, (p_name_3) as u32, (p_uv_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        F::Flash_FindLabel(c, p_flash_1, p_parent_2, p_name_3, fp);
        F::Flash_ClipSetUv(c, p_flash_1, fp, p_uv_4);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "late_a__Shen_SetClipUv", &PROBE_N, &[], true); }
    r_
}

pub fn Shen_SetArrowUvs(c: &mut Ctx, mut p_flash_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Shen_SetArrowUvs", &PROBE_N, &[(p_flash_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let r_ = 'f: {
        { let v_ = 32i32; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = fp; c.m.w32(a_, (v_) as u32); };
        F::late_a__Shen_SetClipUv(c, p_flash_1, sym::late_a__str5, sym::late_a__str6, fp);
        { let v_ = 32i32; let a_ = fp; c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::late_a__Shen_SetClipUv(c, p_flash_1, sym::late_a__str7, sym::late_a__str8, fp);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Shen_SetArrowUvs", &PROBE_N, &[], true); }
    r_
}

pub fn Shen_ScissorList(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Shen_ScissorList", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Sprite_SetScissor(c, 0i32, 512i32, 106i32, 286i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Shen_ScissorList", &PROBE_N, &[], true); }
    r_
}

pub fn Shen_ScissorFull(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Shen_ScissorFull", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Sprite_SetScissor(c, 0i32, 511i32, 0i32, 447i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Shen_ScissorFull", &PROBE_N, &[], true); }
    r_
}

pub fn late_a__Shen_IsOwned(c: &mut Ctx, mut p_kind_1: i32, mut p_n_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "late_a__Shen_IsOwned", &PROBE_N, &[(p_kind_1) as u32, (p_n_2) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv3: i32 = p_kind_1;
        match sv3 {
            0i32 => {
                break 'f (((((((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11976)).wrapping_add(((p_n_2) as u32)); c.m.r8(a_) }) as i32)) & (1i32))) as u8)) as i32);
            }
            1i32 => {
                break 'f (((((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(3112); c.m.r64(a_) }).wrapping_shr((p_n_2) as u32)) & (((1i32) as u64)))) as i32)) != (0i32)) as i32);
            }
            2i32 => {
                break 'f (((((((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3088)).wrapping_add((((rt::div_i32(p_n_2, 64i32)) as u32)).wrapping_mul(8)); c.m.r64(a_) }).wrapping_shr((rt::rem_i32(p_n_2, 64i32)) as u32)) & (((1i32) as u64)))) as i32)) != (0i32)) as i32);
            }
            3i32 => {
                break 'f 0i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "late_a__Shen_IsOwned", &PROBE_N, &[], true); }
    r_
}

pub fn late_a__Shen_FindClip(c: &mut Ctx, mut p_flash_1: u32, mut p_parent_2: u32, mut p_name_3: u32, mut p_ref_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "late_a__Shen_FindClip", &PROBE_N, &[(p_flash_1) as u32, (p_parent_2) as u32, (p_name_3) as u32, (p_ref_4) as u32], false); }
    let r_ = 'f: {
    F::Flash_FindLabel(c, p_flash_1, p_parent_2, p_name_3, p_ref_4);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "late_a__Shen_FindClip", &PROBE_N, &[], true); }
    r_
}

pub fn Shen_DrawList(c: &mut Ctx, mut p_work_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Shen_DrawList", &PROBE_N, &[(p_work_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(288) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 288);
    let mut l_flash_2: u32 = 0;
    let mut l_node_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_idx_5: i32 = 0;
    let mut l_cell_6: i32 = 0;
    let mut l_no_7: i32 = 0;
    let mut l_line_8: i32 = 0;
    let r_ = 'f: {
        l_flash_2 = (p_work_1).wrapping_add(8);
        F::Shen_SetArrowUvs(c, l_flash_2);
        l_node_3 = c.m.r32((p_work_1).wrapping_add(908));
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < (5i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let _ = F::sprintf(c, fp, sym::gShenPlateFmt, &[(((l_i_4).wrapping_add(1i32)) as i64 as u64)]); };
                    's3: {
                        let sv9: u32 = c.m.r32(((l_node_3).wrapping_add(4)).wrapping_add(4));
                        match sv9 {
                            0u32 => {
                                {
                                    l_idx_5 = ((c.m.r32((l_node_3).wrapping_add(4)) as i32)).wrapping_sub(1i32);
                                    if ((((((({ let a_ = (c.m.r32((p_work_1).wrapping_add(856))).wrapping_add((((l_idx_5) as u32)).wrapping_mul(40)); c.m.r8(a_) }) as i32)) != (0i32)) as i32)) != 0) {
                                        l_cell_6 = (if ((((((({ let a_ = (c.m.r32((p_work_1).wrapping_add(856))).wrapping_add((((l_idx_5) as u32)).wrapping_mul(40)); c.m.r8(a_) }) as i32)) != (2i32)) as i32)) != 0) { 2i32 } else { 1i32 });
                                    } else {
                                        l_cell_6 = 3i32;
                                    }
                                    { let v_ = (l_cell_6).wrapping_shl((6i32) as u32); let a_ = fp.wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 0i32; let a_ = (fp.wrapping_add(264)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = ((l_cell_6).wrapping_shl((6i32) as u32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(264)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 64i32; let a_ = (fp.wrapping_add(264)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                    break 's3;
                                }
                            }
                            1u32 => {
                                { let v_ = 64i32; let a_ = fp.wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 128i32; let a_ = (fp.wrapping_add(264)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 64i32; let a_ = (fp.wrapping_add(264)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 128i32; let a_ = (fp.wrapping_add(264)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                break 's3;
                            }
                            2u32 => {
                                { let v_ = 0i32; let a_ = fp.wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 64i32; let a_ = (fp.wrapping_add(264)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 64i32; let a_ = (fp.wrapping_add(264)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 128i32; let a_ = (fp.wrapping_add(264)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                break 's3;
                            }
                            3u32 => {
                                { let v_ = 0i32; let a_ = fp.wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 64i32; let a_ = (fp.wrapping_add(264)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 0i32; let a_ = (fp.wrapping_add(264)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 64i32; let a_ = (fp.wrapping_add(264)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                break 's3;
                            }
                            _ => {
                            }
                        }
                    }
                    F::late_a__Shen_SetClipUv(c, l_flash_2, fp, sym::gShenIconClip, fp.wrapping_add(264));
                    F::Flash_FindLabel(c, l_flash_2, fp, sym::gShenIconClip, fp.wrapping_add(256));
                    F::Flash_ClipSetCallbackA(c, l_flash_2, fp.wrapping_add(256), fid::Shen_ScissorList, ((((0i32) as i64 as u64)) as u32));
                    F::Flash_ClipSetCallbackB(c, l_flash_2, fp.wrapping_add(256), fid::Shen_ScissorFull, ((((0i32) as i64 as u64)) as u32));
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp, fp.wrapping_add(256));
                    F::Flash_ClipSetCallbackA(c, l_flash_2, fp.wrapping_add(256), fid::Shen_ScissorList, ((((0i32) as i64 as u64)) as u32));
                    F::Flash_ClipSetCallbackB(c, l_flash_2, fp.wrapping_add(256), fid::Shen_ScissorFull, ((((0i32) as i64 as u64)) as u32));
                    if (({ let a0_ = ((c.m.r32(((l_node_3).wrapping_add(4)).wrapping_add(4))) as i32); let a1_ = ((c.m.r32((l_node_3).wrapping_add(4)) as i32)).wrapping_sub(1i32); F::late_a__Shen_IsOwned(c, a0_, a1_) }) != 0) {
                        F::Flash_ClipSetColor(c, l_flash_2, fp.wrapping_add(256), F32(0x3f000000));
                    } else {
                        F::Flash_ClipSetColor(c, l_flash_2, fp.wrapping_add(256), F32(0x3f800000));
                    }
                    F::late_a__Shen_FindClip(c, l_flash_2, fp, sym::gShenTextClip, fp.wrapping_add(256));
                    {
                        l_no_7 = ((c.m.r32(l_node_3)) as i32);
                        if ((((((c.m.r32((p_work_1).wrapping_add(876)) as i32)) == (1i32)) as i32)) != 0) {
                            l_line_8 = (l_no_7).wrapping_add(4i32);
                        } else {
                            l_line_8 = l_no_7;
                            if ((((((c.m.r32((p_work_1).wrapping_add(876)) as i32)) == (2i32)) as i32)) != 0) {
                                { let r_ = 11i32; let n_ = (l_line_8).wrapping_add(r_); l_line_8 = n_; };
                            }
                        }
                        F::TextBox_AttachLine(c, l_flash_2, fp.wrapping_add(256), 0i32, 0i32, l_line_8, ((p_work_1).wrapping_add(156)).wrapping_add((((l_i_4) as u32)).wrapping_mul(140)));
                    }
                    if (((((l_i_4) == (3i32)) as i32)) != 0) {
                        l_node_3 = c.m.r32((p_work_1).wrapping_add(912));
                    } else {
                        l_node_3 = c.m.r32((l_node_3).wrapping_add(16));
                    }
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Shen_DrawList", &PROBE_N, &[], true); }
    r_
}

pub fn Shen_SetupGetWin(c: &mut Ctx, mut p_work_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Shen_SetupGetWin", &PROBE_N, &[(p_work_1) as u32], false); }
    let mut l_wish_2: u32 = 0;
    let r_ = 'f: {
    l_wish_2 = ({ let a0_ = c.m.r32((p_work_1).wrapping_add(908)); let a1_ = (c.m.r32((p_work_1).wrapping_add(916)) as i32); F::ShenList_Step(c, a0_, a1_) }).wrapping_add(4);
    's1: {
        let sv3: u32 = c.m.r32((l_wish_2).wrapping_add(4));
        match sv3 {
            0u32 => {
                { let a1_ = ((c.m.r32(l_wish_2) as i32)).wrapping_sub(1i32); F::GetWin_Setup(c, 2i32, a1_) };
                break 's1;
            }
            1u32 => {
                { let a1_ = ((c.m.r32(l_wish_2) as i32)).wrapping_sub(1i32); F::GetWin_Setup(c, 1i32, a1_) };
                break 's1;
            }
            2u32 => {
                { let a1_ = ((c.m.r32(l_wish_2) as i32)).wrapping_sub(1i32); F::GetWin_Setup(c, 0i32, a1_) };
                break 's1;
            }
            3u32 => {
                { let a1_ = (c.m.r32(l_wish_2) as i32); F::GetWin_Setup(c, 6i32, a1_) };
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Shen_SetupGetWin", &PROBE_N, &[], true); }
    r_
}

pub fn late_a__ShenList_Fill(c: &mut Ctx, mut p_head_1: u32, mut p_src_2: u32, mut p_count_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "late_a__ShenList_Fill", &PROBE_N, &[(p_head_1) as u32, (p_src_2) as u32, (p_count_3) as u32], false); }
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (p_count_3)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let s_ = { let o_ = p_src_2; let n_ = (o_).wrapping_add(8); p_src_2 = n_; o_ }; let d_ = (F::ShenList_Find(c, p_head_1, l_i_4)).wrapping_add(4); rt::memcpy(c, d_, s_, 8); };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "late_a__ShenList_Fill", &PROBE_N, &[], true); }
    r_
}

pub fn Shen_BuildList(c: &mut Ctx, mut p_work_1: u32, mut p_file_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Shen_BuildList", &PROBE_N, &[(p_work_1) as u32, (p_file_2) as u32], false); }
    let mut l_a_3: i32 = 0;
    let mut l_b_4: i32 = 0;
    let mut l_c_5: i32 = 0;
    let mut l_r_6: i32 = 0;
    let mut l_list_7: u32 = 0;
    let r_ = 'f: {
    if (((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(16)).wrapping_add((((8i32) as u32)).wrapping_mul(16)); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
        l_a_3 = 40i32;
        l_b_4 = 60i32;
        l_c_5 = 100i32;
    } else {
        l_a_3 = 50i32;
        l_b_4 = 100i32;
        l_c_5 = 200i32;
    }
    l_r_6 = ((F::Rand_Range(c, ((100i32) as u32))) as i32);
    if (((((((((l_r_6) >= (l_b_4)) as i32)) != 0) && (((((l_r_6) < (l_c_5)) as i32)) != 0)) as i32)) != 0) {
        { let v_ = 2i32; let a_ = (p_work_1).wrapping_add(876); c.m.w32(a_, (v_) as u32); };
        { let v_ = 1i32; let a_ = (p_work_1).wrapping_add(884); c.m.w32(a_, (v_) as u32); };
        F::ShenScene_Init(c, 2i32);
        F::ShenList_Build(c, (p_work_1).wrapping_add(888), ((4i32) as u32));
        l_list_7 = (p_work_1).wrapping_add(888);
        F::late_a__ShenList_Fill(c, l_list_7, (p_file_2).wrapping_add(88), 4i32);
        { let v_ = l_list_7; let a_ = (p_work_1).wrapping_add(912); c.m.w32(a_, v_); };
    } else {
        if (((((((((l_r_6) >= (l_a_3)) as i32)) != 0) && (((((l_r_6) < (l_b_4)) as i32)) != 0)) as i32)) != 0) {
            { let v_ = 1i32; let a_ = (p_work_1).wrapping_add(876); c.m.w32(a_, (v_) as u32); };
            { let v_ = 3i32; let a_ = (p_work_1).wrapping_add(884); c.m.w32(a_, (v_) as u32); };
            F::ShenScene_Init(c, 1i32);
            F::ShenList_Build(c, (p_work_1).wrapping_add(888), ((7i32) as u32));
            l_list_7 = (p_work_1).wrapping_add(888);
            F::late_a__ShenList_Fill(c, l_list_7, (p_file_2).wrapping_add(32), 7i32);
            { let v_ = l_list_7; let a_ = (p_work_1).wrapping_add(912); c.m.w32(a_, v_); };
        } else {
            { let v_ = 0i32; let a_ = (p_work_1).wrapping_add(876); c.m.w32(a_, (v_) as u32); };
            { let v_ = 1i32; let a_ = (p_work_1).wrapping_add(884); c.m.w32(a_, (v_) as u32); };
            F::ShenScene_Init(c, 0i32);
            F::ShenList_Build(c, (p_work_1).wrapping_add(888), ((4i32) as u32));
            l_list_7 = (p_work_1).wrapping_add(888);
            F::late_a__ShenList_Fill(c, l_list_7, p_file_2, 4i32);
            { let v_ = l_list_7; let a_ = (p_work_1).wrapping_add(912); c.m.w32(a_, v_); };
        }
    }
    { let v_ = l_list_7; let a_ = (p_work_1).wrapping_add(908); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Shen_BuildList", &PROBE_N, &[], true); }
    r_
}

pub fn Shen_Init(c: &mut Ctx, mut p_work_1: u32, mut p_section_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Shen_Init", &PROBE_N, &[(p_work_1) as u32, (p_section_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = (c.m.r32(sym::gMenuArc12)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc12)).wrapping_add((((p_section_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = p_work_1; c.m.w32(a_, v_); };
        { let v_ = { let a0_ = c.m.r32(p_work_1); F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (p_work_1).wrapping_add(4); c.m.w32(a_, v_); };
        F::late_a__Shen_DebugFile(c, sym::late_a__str9);
        { let v_ = (c.m.r32((p_work_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_work_1).wrapping_add(4))).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = { let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }; let a_ = ((p_work_1).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((64i32) as u32)); let a_ = ((p_work_1).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((64i32).wrapping_mul(2i32)) as u32)); let a_ = ((p_work_1).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((64i32).wrapping_mul(3i32)) as u32)); let a_ = ((p_work_1).wrapping_add(56)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((64i32).wrapping_mul(4i32)) as u32)); let a_ = ((p_work_1).wrapping_add(56)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((64i32).wrapping_mul(5i32)) as u32)); let a_ = ((p_work_1).wrapping_add(56)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((64i32).wrapping_mul(6i32)) as u32)); let a_ = ((p_work_1).wrapping_add(56)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((p_work_1).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        F::late_a__Shen_DebugFile(c, sym::late_a__str10);
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < (1i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a1_ = (c.m.r32((p_work_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_work_1).wrapping_add(4))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::Flash_Create(c, ((p_work_1).wrapping_add(8)).wrapping_add((((l_i_3) as u32)).wrapping_mul(44)), a1_, (p_work_1).wrapping_add(56)) };
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        F::late_a__Shen_DebugFile(c, sym::late_a__str11);
        { let v_ = (c.m.r32((p_work_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_work_1).wrapping_add(4))).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        F::late_a__Shen_DebugFile(c, sym::late_a__str12);
        { let a0_ = (c.m.r32((p_work_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_work_1).wrapping_add(4))).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = c.m.r32(fp); F::IconWin_Init(c, a0_, a1_) };
        F::late_a__Shen_DebugFile(c, sym::late_a__str13);
        { let v_ = (c.m.r32((p_work_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_work_1).wrapping_add(4))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (p_work_1).wrapping_add(88); c.m.w32(a_, v_); };
        F::late_a__Shen_DebugFile(c, sym::late_a__str14);
        { let a0_ = (c.m.r32((p_work_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_work_1).wrapping_add(4))).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = c.m.r32((p_work_1).wrapping_add(88)); let _ = (p_work_1).wrapping_add(92); F::MsgWin_Init(c, a0_, a1_, 0i32) };
        F::late_a__Shen_DebugFile(c, sym::late_a__str15);
        { let a0_ = (c.m.r32((p_work_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_work_1).wrapping_add(4))).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = c.m.r32((p_work_1).wrapping_add(88)); F::Dialog_Init(c, a0_, a1_, 0i32) };
        F::late_a__Shen_DebugFile(c, sym::late_a__str16);
        { let a0_ = (c.m.r32((p_work_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_work_1).wrapping_add(4))).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::ShenCfm_Init(c, a0_) };
        F::late_a__Shen_DebugFile(c, sym::late_a__str17);
        { let v_ = (c.m.r32((p_work_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_work_1).wrapping_add(4))).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (p_work_1).wrapping_add(856); c.m.w32(a_, v_); };
        F::late_a__Shen_DebugFile(c, sym::late_a__str18);
        { let a0_ = (c.m.r32((p_work_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_work_1).wrapping_add(4))).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::GetWin_Init(c, a0_, 0i32) };
        F::late_a__Shen_DebugFile(c, sym::late_a__str19);
        { let v_ = (c.m.r32((p_work_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_work_1).wrapping_add(4))).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (p_work_1).wrapping_add(152); c.m.w32(a_, v_); };
        {
            l_i_3 = 0i32;
            'l3: loop {
                if !(((((l_i_3) < (5i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let a1_ = c.m.r32((p_work_1).wrapping_add(152)); F::TextBox_Init(c, ((p_work_1).wrapping_add(156)).wrapping_add((((l_i_3) as u32)).wrapping_mul(140)), a1_, ((2i32) as u32)) };
                    F::TextBox_SetRect(c, ((p_work_1).wrapping_add(156)).wrapping_add((((l_i_3) as u32)).wrapping_mul(140)), 0i32, 512i32, 106i32, 286i32);
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        F::late_a__Shen_DebugFile(c, sym::late_a__str20);
        { let a1_ = (c.m.r32((p_work_1).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((p_work_1).wrapping_add(4))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::Shen_BuildList(c, p_work_1, a1_) };
        { let r_ = (!(127i32)); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(8); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Shen_Init", &PROBE_N, &[], true); }
    r_
}

pub fn Shen_Advance(c: &mut Ctx, mut p_work_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Shen_Advance", &PROBE_N, &[(p_work_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                F::Flash_Advance(c, ((p_work_1).wrapping_add(8)).wrapping_add((((l_i_2) as u32)).wrapping_mul(44)));
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Shen_Advance", &PROBE_N, &[], true); }
    r_
}

pub fn Shen_Draw(c: &mut Ctx, mut p_work_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Shen_Draw", &PROBE_N, &[(p_work_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    if ((((!(((((c.m.r32((p_work_1).wrapping_add(924)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        if (((((F::ShenScene_GetState(c)) == (1i32)) as i32)) != 0) {
            { let r_ = ((1i32) | (8i32)); let a_ = (p_work_1).wrapping_add(924); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
    } else {
        if (((((c.m.r32((p_work_1).wrapping_add(924)) as i32)) & (8i32))) != 0) {
            F::IconWin_Open(c);
            F::MsgWin_Open(c);
            { let r_ = 8i32; let a_ = (p_work_1).wrapping_add(924); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
        }
    }
    F::Shen_DrawList(c, p_work_1);
    { let _ = F::ShenScene_Update(c); };
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                F::Flash_Draw(c, ((p_work_1).wrapping_add(8)).wrapping_add((((l_i_2) as u32)).wrapping_mul(44)));
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    F::IconWin_Draw(c);
    F::GetWin_Draw(c);
    { let a2_ = (c.m.r32((p_work_1).wrapping_add(864)) as i32); F::MsgWin_Draw(c, 0i32, 0i32, a2_) };
    F::ShenCfm_Draw(c);
    F::Dialog_Draw(c, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Shen_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn Shen_Term(c: &mut Ctx, mut p_work_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Shen_Term", &PROBE_N, &[(p_work_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                F::Flash_Destroy(c, ((p_work_1).wrapping_add(8)).wrapping_add((((l_i_2) as u32)).wrapping_mul(44)));
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    F::ShenList_Free(c, (p_work_1).wrapping_add(888));
    F::ShenCfm_Term(c);
    F::IconWin_Term(c);
    F::Dialog_Term(c);
    F::MsgWin_Term(c);
    F::ShenScene_Term(c);
    F::GetWin_Term(c);
    if (((((c.m.r32((p_work_1).wrapping_add(4))) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32((p_work_1).wrapping_add(4)); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (p_work_1).wrapping_add(4); c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Shen_Term", &PROBE_N, &[], true); }
    r_
}

pub fn late_a__Shen_PressedOk(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "late_a__Shen_PressedOk", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "late_a__Shen_PressedOk", &PROBE_N, &[], true); }
    r_
}

pub fn Shen_UpdateTalk(c: &mut Ctx, mut p_work_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Shen_UpdateTalk", &PROBE_N, &[(p_work_1) as u32], false); }
    let mut l_line_2: i32 = 0;
    let r_ = 'f: {
    if ((((!(((((c.m.r32((p_work_1).wrapping_add(924)) as i32)) & (256i32))) != 0)) as i32)) != 0) {
        l_line_2 = ((F::Rand_Range(c, ((2i32) as u32))) as i32);
        if ((((((c.m.r32((p_work_1).wrapping_add(876)) as i32)) != (1i32)) as i32)) != 0) {
            if (((((l_line_2) == (1i32)) as i32)) != 0) {
                l_line_2 = 2i32;
            }
        }
        { let a1_ = (if ((((((c.m.r32((p_work_1).wrapping_add(876)) as i32)) == (1i32)) as i32)) != 0) { (l_line_2).wrapping_add(8i32) } else { l_line_2 }); F::Shen_Say(c, p_work_1, a1_) };
        { let r_ = 256i32; let a_ = (p_work_1).wrapping_add(924); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        if ((((((((((((((c.m.r32((p_work_1).wrapping_add(864)) as i32)) == ((if ((((((c.m.r32((p_work_1).wrapping_add(876)) as i32)) == (1i32)) as i32)) != 0) { 8i32 } else { 0i32 }))) as i32)) != 0) || ((((((c.m.r32((p_work_1).wrapping_add(864)) as i32)) == ((if ((((((c.m.r32((p_work_1).wrapping_add(876)) as i32)) == (1i32)) as i32)) != 0) { 9i32 } else { 1i32 }))) as i32)) != 0)) as i32)) != 0) || ((((((c.m.r32((p_work_1).wrapping_add(864)) as i32)) == ((if ((((((c.m.r32((p_work_1).wrapping_add(876)) as i32)) == (1i32)) as i32)) != 0) { 10i32 } else { 2i32 }))) as i32)) != 0)) as i32)) != 0) {
            if (((((((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) || ((F::late_a__Shen_PressedOk(c)) != 0)) as i32)) != 0) {
                F::Flash_Play(c, (p_work_1).wrapping_add(8), 1i32);
            }
            if ((((!(((((c.m.r32((p_work_1).wrapping_add(924)) as i32)) & (512i32))) != 0)) as i32)) != 0) {
                if (((((c.m.r32((((p_work_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) & (2i32))) != 0) {
                    { let a1_ = (c.m.r32((p_work_1).wrapping_add(916)) as i32); F::Shen_SetPlate(c, (p_work_1).wrapping_add(8), a1_, 1i32) };
                    { let r_ = 512i32; let a_ = (p_work_1).wrapping_add(924); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
            break 'f 0i32;
        } else {
            if ((((((((((c.m.r32((p_work_1).wrapping_add(864)) as i32)) == ((if ((((((c.m.r32((p_work_1).wrapping_add(876)) as i32)) == (1i32)) as i32)) != 0) { 15i32 } else { 7i32 }))) as i32)) != 0) && ((((!(((((c.m.r32((p_work_1).wrapping_add(924)) as i32)) & (1024i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
                if (((((((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) || ((F::late_a__Shen_PressedOk(c)) != 0)) as i32)) != 0) {
                    F::MsgWin_Close(c);
                    F::IconWin_Close(c);
                    { let v_ = 40i32; let a_ = (p_work_1).wrapping_add(928); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (1i32).wrapping_neg(); let a_ = (p_work_1).wrapping_add(864); c.m.w32(a_, (v_) as u32); };
                    { let r_ = 1024i32; let a_ = (p_work_1).wrapping_add(924); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            } else {
                if (((((c.m.r32((p_work_1).wrapping_add(924)) as i32)) & (1024i32))) != 0) {
                    { let a_ = (p_work_1).wrapping_add(928); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    if ((((((c.m.r32((p_work_1).wrapping_add(928)) as i32)) == ((1i32).wrapping_neg())) as i32)) != 0) {
                        F::ShenScene_SetState(c, 3i32);
                        { let r_ = 1024i32; let a_ = (p_work_1).wrapping_add(924); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                } else {
                    break 'f 0i32;
                }
            }
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Shen_UpdateTalk", &PROBE_N, &[], true); }
    r_
}

pub fn Shen_Update(c: &mut Ctx, mut p_work_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Shen_Update", &PROBE_N, &[(p_work_1) as u32], false); }
    let mut l_ready_2: i32 = 0;
    let mut l_wish_3: u32 = 0;
    let mut l_kind_4: i32 = 0;
    let mut l_id_5: i32 = 0;
    let mut l_line_6: i32 = 0;
    let mut l_got_7: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32((p_work_1).wrapping_add(924)) as i32)) & (2i32))) != 0) {
        break 'f;
    }
    l_ready_2 = F::ShenScene_GetState(c);
    if (((((l_ready_2) != (1i32)) as i32)) != 0) {
        break 'f;
    }
    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
        break 'f;
    }
    if ((F::GetWin_IsAnimating(c)) != 0) {
        break 'f;
    }
    if ((F::Shen_UpdateTalk(c, p_work_1)) != 0) {
        break 'f;
    }
    if ((((!(((((c.m.r32((((p_work_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) & (2i32))) != 0)) as i32)) != 0) {
        break 'f;
    }
    's1: {
        let sv8: i32 = (c.m.r32((p_work_1).wrapping_add(868)) as i32);
        match sv8 {
            0i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                    F::Shen_MoveCursor(c, p_work_1, 0i32);
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                        F::Shen_MoveCursor(c, p_work_1, 1i32);
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                            l_kind_4 = (({ let a_ = ({ let a0_ = c.m.r32((p_work_1).wrapping_add(908)); let a1_ = (c.m.r32((p_work_1).wrapping_add(916)) as i32); F::ShenList_StepWish(c, a0_, a1_) }).wrapping_add(4); c.m.r32(a_) }) as i32);
                            l_id_5 = ({ let a_ = { let a0_ = c.m.r32((p_work_1).wrapping_add(908)); let a1_ = (c.m.r32((p_work_1).wrapping_add(916)) as i32); F::ShenList_StepWish(c, a0_, a1_) }; (c.m.r32(a_) as i32) }).wrapping_sub(1i32);
                            { let v_ = 1i32; let a_ = (p_work_1).wrapping_add(868); c.m.w32(a_, (v_) as u32); };
                            F::Shen_PlayPlateOk(c, p_work_1);
                            F::ShenCfm_Open(c, l_kind_4, l_id_5);
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                        }
                    }
                }
                break 's1;
            }
            1i32 => {
                if ((F::ShenCfm_Update(c, (p_work_1).wrapping_add(872))) != 0) {
                    's2: {
                        let sv9: i32 = (c.m.r32((p_work_1).wrapping_add(872)) as i32);
                        match sv9 {
                            0i32 => {
                                {
                                    { let a_ = (p_work_1).wrapping_add(880); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                    { let v_ = 2i32; let a_ = (p_work_1).wrapping_add(868); c.m.w32(a_, (v_) as u32); };
                                    { let r_ = 64i32; let a_ = (p_work_1).wrapping_add(924); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                    F::ShenCfm_Close(c);
                                    F::Shen_SetupGetWin(c, p_work_1);
                                    F::GetWin_Open(c);
                                    l_wish_3 = { let a0_ = c.m.r32((p_work_1).wrapping_add(908)); let a1_ = (c.m.r32((p_work_1).wrapping_add(916)) as i32); F::ShenList_StepWish(c, a0_, a1_) };
                                    l_line_6 = ((F::Rand_Range(c, ((2i32) as u32))) as i32);
                                    if (((((l_line_6) == (0i32)) as i32)) != 0) {
                                        l_line_6 = 2i32;
                                    }
                                    { let a1_ = (if ((((((c.m.r32((p_work_1).wrapping_add(876)) as i32)) == (1i32)) as i32)) != 0) { (l_line_6).wrapping_add(11i32) } else { (l_line_6).wrapping_add(3i32) }); F::Shen_Say(c, p_work_1, a1_) };
                                    's3: {
                                        let sv10: u32 = c.m.r32((l_wish_3).wrapping_add(4));
                                        match sv10 {
                                            0u32 => {
                                                { let a0_ = ((c.m.r32(l_wish_3) as i32)).wrapping_sub(1i32); F::Save_AddItem(c, a0_) };
                                                break 's3;
                                            }
                                            1u32 => {
                                                { let r_ = (((1i64).wrapping_shl((((c.m.r32(l_wish_3) as i32)).wrapping_sub(1i32)) as u32)) as u64); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(3112); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
                                                break 's3;
                                            }
                                            2u32 => {
                                                { let r_ = (((1i64).wrapping_shl((rt::rem_i32(((c.m.r32(l_wish_3) as i32)).wrapping_sub(1i32), 64i32)) as u32)) as u64); let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3088)).wrapping_add((((rt::div_i32(((c.m.r32(l_wish_3) as i32)).wrapping_sub(1i32), 64i32)) as u32)).wrapping_mul(8)); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
                                                break 's3;
                                            }
                                            3u32 => {
                                                { let a0_ = (c.m.r32(l_wish_3) as i32); F::Save_AddMoney(c, a0_) };
                                                break 's3;
                                            }
                                            _ => {
                                            }
                                        }
                                    }
                                    break 's2;
                                }
                            }
                            1i32 => {
                                { let v_ = 0i32; let a_ = (p_work_1).wrapping_add(868); c.m.w32(a_, (v_) as u32); };
                                F::ShenCfm_Close(c);
                                break 's2;
                            }
                            _ => {
                            }
                        }
                    }
                }
                break 's1;
            }
            2i32 => {
                if ((((c.m.r64(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(392))) & ((((((512i32) | (1024i32))) as u64)).wrapping_shl((32i32) as u32)))) != 0) {
                    l_got_7 = { let a0_ = c.m.r32((p_work_1).wrapping_add(908)); let a1_ = (c.m.r32((p_work_1).wrapping_add(916)) as i32); F::ShenList_StepWish(c, a0_, a1_) };
                    if (((((((((((((c.m.r32((l_got_7).wrapping_add(4))) == (((2i32) as u32))) as i32)) != 0) && ((((((c.m.r32(l_got_7) as i32)) == (160i32)) as i32)) != 0)) as i32)) != 0) && ((((!(({ let a0_ = ((c.m.r32((l_got_7).wrapping_add(4))) as i32); let a1_ = (c.m.r32(l_got_7) as i32); F::late_a__Shen_IsOwned(c, a0_, a1_) }) != 0)) as i32)) != 0)) as i32)) != 0) {
                        { let r_ = (((1i64).wrapping_shl((rt::rem_i32((c.m.r32(l_got_7) as i32), 64i32)) as u32)) as u64); let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3088)).wrapping_add((((rt::div_i32((c.m.r32(l_got_7) as i32), 64i32)) as u32)).wrapping_mul(8)); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
                        { let a1_ = (c.m.r32(l_got_7) as i32); F::GetWin_Setup(c, 0i32, a1_) };
                        F::GetWin_Next(c);
                    } else {
                        F::GetWin_Close(c);
                        if ((((((c.m.r32((p_work_1).wrapping_add(884)) as i32)) == ((c.m.r32((p_work_1).wrapping_add(880)) as i32))) as i32)) != 0) {
                            { let a1_ = (if ((((((c.m.r32((p_work_1).wrapping_add(876)) as i32)) == (1i32)) as i32)) != 0) { 15i32 } else { 7i32 }); F::Shen_Say(c, p_work_1, a1_) };
                            F::Flash_GotoLabel(c, (p_work_1).wrapping_add(8), sym::late_a__str21, ((((1i32) as i64 as u64)) as u8));
                            { let v_ = 3i32; let a_ = (p_work_1).wrapping_add(868); c.m.w32(a_, (v_) as u32); };
                        } else {
                            { let v_ = 0i32; let a_ = (p_work_1).wrapping_add(868); c.m.w32(a_, (v_) as u32); };
                        }
                    }
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                }
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Shen_Update", &PROBE_N, &[], true); }
    r_
}

pub fn Shen_UpdateExit(c: &mut Ctx, mut p_work_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Shen_UpdateExit", &PROBE_N, &[(p_work_1) as u32], false); }
    let r_ = 'f: {
    if (((((((((F::ShenScene_GetState(c)) == (2i32)) as i32)) != 0) && ((((!(((((c.m.r32((p_work_1).wrapping_add(924)) as i32)) & (2i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
        { let r_ = 2i32; let a_ = (p_work_1).wrapping_add(924); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if (((((c.m.r32((p_work_1).wrapping_add(924)) as i32)) & (2i32))) != 0) {
        if ((((!(((((c.m.r32((p_work_1).wrapping_add(924)) as i32)) & (4i32))) != 0)) as i32)) != 0) {
            { let r_ = 4i32; let a_ = (p_work_1).wrapping_add(924); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            { let v_ = 30i32; let a_ = (p_work_1).wrapping_add(920); c.m.w32(a_, (v_) as u32); };
            F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
        }
        if ((F::ColorFade_IsFadingOut(c)) != 0) {
            F::Voice_FadeOutStep(c, 0i32);
            if ((((((c.m.r32((p_work_1).wrapping_add(860)) as i32)) == (0i32)) as i32)) != 0) {
                F::Bgm_FadeOutStep(c);
            }
        } else {
            break 'f 1i32;
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Shen_UpdateExit", &PROBE_N, &[], true); }
    r_
}

pub fn Shen_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Shen_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(944) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 944);
    let r_ = 'f: {
        { let _ = F::memset(c, fp, 0i32, 932u32); };
        F::Shen_Init(c, fp, p_section_1);
        F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
        {
            'l1: loop {
                'c2: {
                    F::Gfx_BeginFrame(c);
                    F::Pad_Update(c);
                    F::ColorFade_Update(c);
                    F::Snd_Update(c);
                    F::Shen_Advance(c, fp);
                    F::Shen_Draw(c, fp);
                    F::ColorFade_Draw(c);
                    F::Gfx_EndFrame(c, 2i32);
                    F::Dma_Flush(c);
                    F::File_Stub264D90(c);
                    if ((F::Shen_UpdateExit(c, fp)) != 0) {
                        break 'l1;
                    }
                    F::Shen_Update(c, fp);
                }
            }
        }
        F::Shen_Term(c, fp);
        F::Dma_ResetBuffers(c);
        break 'f (c.m.r32((fp).wrapping_add(860)) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Shen_Run", &PROBE_N, &[], true); }
    r_
}
