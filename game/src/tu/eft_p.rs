#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn eft_p__EftDisc_Clamp(c: &mut Ctx, mut p_x_1: F32, mut p_lo_2: F32, mut p_hi_3: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_p__EftDisc_Clamp", &PROBE_N, &[(p_x_1).0, (p_lo_2).0, (p_hi_3).0], false); }
    let r_ = 'f: {
    if (((rt::flt(p_x_1, p_lo_2) as i32)) != 0) {
        break 'f p_lo_2;
    }
    if (((rt::flt(p_hi_3, p_x_1) as i32)) != 0) {
        break 'f p_hi_3;
    }
    break 'f p_x_1;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "eft_p__EftDisc_Clamp", &PROBE_N, &[], true); }
    r_
}

pub fn EftDisc_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_arg_3: u32 = 0;
    let mut l_stopped_4: i32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_arg_3 = (l_w_2).wrapping_add(48);
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(1212)) as i32)) & (128i32))) != 0)) as i32)) != 0) {
        l_stopped_4 = { let a0_ = ((c.m.r8((l_arg_3).wrapping_add(86))) as i32); F::BtlScene_IsEffectStopped(c, a0_, 0i32) };
    } else {
        l_stopped_4 = { let a0_ = ((c.m.r8((l_arg_3).wrapping_add(86))) as i32); F::BtlScene_IsEffectStopped(c, a0_, 1i32) };
    }
    if ((((!((l_stopped_4) != 0)) as i32)) != 0) {
        if (((((c.m.r32((p_task_1).wrapping_add(4)) as i32)) & (63i32))) != 0) {
            { let r_ = 4i32; let a_ = (l_w_2).wrapping_add(1212); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if (((((c.m.r32((l_w_2).wrapping_add(1212)) as i32)) & (128i32))) != 0) {
            { let a0_ = c.m.r32((l_arg_3).wrapping_add(4)); { let _ = F::EftEmit_UpdateAlive(c, a0_, (l_w_2).wrapping_add(224)); } };
            if ((((((c.m.r16((p_task_1).wrapping_add(8))) as i32)) & (1i32))) != 0) {
                F::EftDisc_SpawnParts(c, p_task_1, 1i32);
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftDisc_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((c.m.r32((l_w_2).wrapping_add(1212)) as i32)) & (4i32))) != 0) {
        break 'f;
    }
    F::EftMesh_SetLayer(c, (l_w_2).wrapping_add(944), 1i32);
    { let a1_ = ((rt::f2i(F32(c.m.r32((l_w_2).wrapping_add(1088))))) as u8); let a2_ = ((rt::f2i(F32(c.m.r32(((l_w_2).wrapping_add(1088)).wrapping_add(4))))) as u8); let a3_ = ((rt::f2i(F32(c.m.r32(((l_w_2).wrapping_add(1088)).wrapping_add(8))))) as u8); let a4_ = ((rt::f2i(F32(c.m.r32(((l_w_2).wrapping_add(1088)).wrapping_add(12))))) as u8); F::EftMesh_SetColor(c, (l_w_2).wrapping_add(944), a1_, a2_, a3_, a4_) };
    if (((((c.m.r32((l_w_2).wrapping_add(1212)) as i32)) & (64i32))) != 0) {
        F::EftMesh_Draw(c, (l_w_2).wrapping_add(944));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftDisc_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_arg_3: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_arg_3 = (l_w_2).wrapping_add(48);
    if (((((c.m.r32((l_w_2).wrapping_add(1212)) as i32)) & (128i32))) != 0) {
        { let a0_ = c.m.r32((l_arg_3).wrapping_add(4)); F::EftEmit_KillAll(c, a0_, (l_w_2).wrapping_add(224)) };
    }
    { let r_ = 4i32; let a_ = (l_w_2).wrapping_add(1212); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftDisc_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_arg_3: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_arg_3 = (l_w_2).wrapping_add(48);
    if (((((c.m.r32((l_w_2).wrapping_add(1212)) as i32)) & (128i32))) != 0) {
        { let a0_ = c.m.r32((l_arg_3).wrapping_add(4)); F::EftEmit_TermState(c, a0_, (l_w_2).wrapping_add(224)) };
    } else {
        if (((((((c.m.r8((l_arg_3).wrapping_add(87))) as i32)) < (2i32)) as i32)) != 0) {
            { let _ = { let a0_ = ((c.m.r8((l_arg_3).wrapping_add(86))) as i32); F::EftDiscMgr_PopHeld(c, a0_) }; };
        }
    }
    { let _ = F::memset(c, l_w_2, 0i32, 1216u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftDisc_SpawnParts(c: &mut Ctx, mut p_task_1: u32, mut p_mode_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_SpawnParts", &PROBE_N, &[(p_task_1) as u32, (p_mode_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(592) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 592);
    let mut l_w_3: u32 = 0;
    let mut l_arg_4: u32 = 0;
    let mut l_g_5: i32 = 0;
    let mut l_info_6: u32 = 0;
    let mut l_j_7: i32 = 0;
    let mut l_part_8: i32 = 0;
    let mut l_ok_9: i32 = 0;
    let mut l_res_10: i32 = 0;
    let r_ = 'f: {
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        l_arg_4 = (l_w_3).wrapping_add(48);
        { let _ = F::memset(c, fp, 0i32, 592u32); };
        {
            l_g_5 = 0i32;
            'l1: loop {
                if !(((((l_g_5) < (19i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((({ let a_ = { let a_ = c.m.r32((l_arg_4).wrapping_add(4)); c.m.r32(a_) }; c.m.r32(a_) }) & ((((1i32).wrapping_shl((l_g_5) as u32)) as u32)))) != 0) {
                        l_info_6 = { let a_ = ((c.m.r32((l_arg_4).wrapping_add(4))).wrapping_add(12)).wrapping_add((((l_g_5) as u32)).wrapping_mul(16)); c.m.r32(a_) };
                        {
                            l_j_7 = 0i32;
                            'l3: loop {
                                if !(((((l_j_7) < (((c.m.r8((l_info_6).wrapping_add(((1i32) as u32)))) as i32))) as i32)) != 0) { break 'l3; }
                                'c4: {
                                    l_part_8 = ((({ let a_ = (((c.m.r32((l_arg_4).wrapping_add(4))).wrapping_add(12)).wrapping_add((((l_g_5) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.r8(a_) }) as i32)).wrapping_add(l_j_7);
                                    l_ok_9 = 0i32;
                                    if (((((c.m.r32((l_w_3).wrapping_add(1212)) as i32)) & (8i32))) != 0) {
                                        if ((((((({ let a_ = (({ let a_ = (c.m.r32((l_arg_4).wrapping_add(4))).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((l_part_8) as u32)).wrapping_mul(64))).wrapping_add(8); c.m.r8(a_) }) as i32)) != (1i32)) as i32)) != 0) {
                                            l_ok_9 = ((((({ let a_ = (({ let a_ = (c.m.r32((l_arg_4).wrapping_add(4))).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((l_part_8) as u32)).wrapping_mul(64))).wrapping_add(10); c.m.r8(a_) }) as i32)) == (5i32)) as i32);
                                        }
                                    }
                                    if (((((c.m.r32((l_w_3).wrapping_add(1212)) as i32)) & (32i32))) != 0) {
                                        if ((((((({ let a_ = (({ let a_ = (c.m.r32((l_arg_4).wrapping_add(4))).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((l_part_8) as u32)).wrapping_mul(64))).wrapping_add(8); c.m.r8(a_) }) as i32)) != (0i32)) as i32)) != 0) {
                                            if ((((((({ let a_ = (({ let a_ = (c.m.r32((l_arg_4).wrapping_add(4))).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((l_part_8) as u32)).wrapping_mul(64))).wrapping_add(10); c.m.r8(a_) }) as i32)) == (5i32)) as i32)) != 0) {
                                                l_ok_9 = 1i32;
                                            }
                                        }
                                    }
                                    if ((l_ok_9) != 0) {
                                        if (((((p_mode_2) == (0i32)) as i32)) != 0) {
                                            l_res_10 = { let a0_ = c.m.r32((l_arg_4).wrapping_add(4)); let a2_ = ((c.m.r8((l_arg_4).wrapping_add(86))) as i32); let a5_ = (((c.m.r32((l_w_3).wrapping_add(1212)) as i32)) & (6i32)); let a6_ = (((c.m.r32((l_w_3).wrapping_add(1212)) as i32)) & (512i32)); F::EftEmit_GetFlagsFromReq(c, a0_, (l_w_3).wrapping_add(224), a2_, l_g_5, l_j_7, a5_, a6_) };
                                        } else {
                                            l_res_10 = { let a0_ = c.m.r32((l_arg_4).wrapping_add(4)); F::EftEmit_GetResetFlags(c, a0_, (l_w_3).wrapping_add(224), l_g_5, l_j_7) };
                                        }
                                        if (((((l_res_10) != (0i32)) as i32)) != 0) {
                                            { let a0_ = c.m.r32((l_arg_4).wrapping_add(4)); let a8_ = F32(c.m.r32((l_arg_4).wrapping_add(48))); F::EftEmit_SpawnOwn(c, a0_, (l_w_3).wrapping_add(224), fp, (l_arg_4).wrapping_add(32), (l_arg_4).wrapping_add(16), l_g_5, l_j_7, l_res_10, a8_) };
                                        }
                                    }
                                }
                                { let o_ = l_j_7; let n_ = (o_).wrapping_add(1); l_j_7 = n_; };
                            }
                        }
                    }
                }
                { let o_ = l_g_5; let n_ = (o_).wrapping_add(1); l_g_5 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_SpawnParts", &PROBE_N, &[], true); }
    r_
}

pub fn EftDisc_AddHit(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_AddHit", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_rec_2: u32 = 0;
    let mut l_w_3: u32 = 0;
    let mut l_r_4: F32 = F32(0);
    let mut l_a_5: u32 = 0;
    let mut l_b_6: u32 = 0;
    let mut l_src_7: u32 = 0;
    let r_ = 'f: {
    l_rec_2 = F::EftHit_GetNew(c);
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    F::Vec4_Copy(c, (l_w_3).wrapping_add(144), ((l_w_3).wrapping_add(48)).wrapping_add(16));
    F::Vec4_Copy(c, (l_rec_2).wrapping_add(16), ((l_w_3).wrapping_add(48)).wrapping_add(32));
    F::Vec4_Copy(c, (l_rec_2).wrapping_add(64), ((l_w_3).wrapping_add(48)).wrapping_add(16));
    F::Vec4_Copy(c, (l_rec_2).wrapping_add(32), ((l_w_3).wrapping_add(48)).wrapping_add(32));
    F::Vec4_Copy(c, (l_rec_2).wrapping_add(48), (l_w_3).wrapping_add(1120));
    { let v_ = p_task_1; let a_ = (l_rec_2).wrapping_add(96); c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = (l_rec_2).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((c.m.r8(((l_w_3).wrapping_add(48)).wrapping_add(86))) as i32); let a_ = l_rec_2; c.m.w32(a_, (v_) as u32); };
    's1: {
        let sv8: i32 = ((c.m.r8(((l_w_3).wrapping_add(48)).wrapping_add(85))) as i32);
        match sv8 {
            0i32 => {
                { let v_ = 1i32; let a_ = (l_rec_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            1i32 => {
                { let v_ = 2i32; let a_ = (l_rec_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = 4i32; let a_ = (l_rec_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            3i32 => {
                { let v_ = 6i32; let a_ = (l_rec_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
                { let v_ = 1i32; let a_ = (l_rec_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
        }
    }
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(1212)) as i32)) & (128i32))) != 0)) as i32)) != 0) {
        { let v_ = (l_w_3).wrapping_add(144); let a_ = (l_rec_2).wrapping_add(104); c.m.w32(a_, v_); };
        { let v_ = 0i32; let a_ = (l_rec_2).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        l_r_4 = rt::fmul(F32(c.m.r32(((l_w_3).wrapping_add(48)).wrapping_add(48))), F32(0x40900000));
    } else {
        l_src_7 = c.m.r32((l_w_3).wrapping_add(48));
        if ((((!(((({ let a_ = (c.m.r32((l_src_7).wrapping_add(36))).wrapping_add(60); (c.m.r32(a_) as i32) }) & (1i32))) != 0)) as i32)) != 0) {
            break 'f;
        }
        { let v_ = l_src_7; let a_ = (l_rec_2).wrapping_add(100); c.m.w32(a_, v_); };
        { let v_ = 1i32; let a_ = (l_rec_2).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        l_r_4 = rt::fmul(F::EftEmit_GetTrailWidth(c, (l_w_3).wrapping_add(224)), F32(0x40400000));
        if ((((((c.m.r32(((l_w_3).wrapping_add(48)).wrapping_add(88)) as i32)) != (0i32)) as i32)) != 0) {
            { let r_ = 16i32; let a_ = (l_rec_2).wrapping_add(80); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
    }
    l_a_5 = F::EftHitArena_AllocSphere(c);
    l_b_6 = F::EftHitArena_AllocSphere(c);
    F::ColSphere_Set(c, l_a_5, ((l_w_3).wrapping_add(48)).wrapping_add(32), l_r_4);
    F::ColSphere_Set(c, l_b_6, (l_w_3).wrapping_add(1120), l_r_4);
    F::EftHit_SetShapeSpheres(c, l_rec_2, l_a_5, l_b_6);
    F::EftHit_Add(c, l_rec_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_AddHit", &PROBE_N, &[], true); }
    r_
}

pub fn EftDisc_SetHeldAtk(c: &mut Ctx, mut p_task_1: u32, mut p_atk_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_SetHeldAtk", &PROBE_N, &[(p_task_1) as u32, (p_atk_2) as u32], false); }
    let mut l_n_3: u32 = 0;
    let mut l_t_4: u32 = 0;
    let mut l_w_5: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftDisc)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    l_n_3 = { let a_ = (({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }).wrapping_add(1364)).wrapping_add(((((c.m.r16((p_atk_2).wrapping_add(18)) as i16)) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    if (((((l_n_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    l_t_4 = c.m.r32(l_n_3);
    if (((((l_t_4) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    l_w_5 = c.m.r32((l_t_4).wrapping_add(56));
    if (((((l_w_5) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((c.m.r32((l_w_5).wrapping_add(1212)) as i32)) & (1i32))) != 0) {
        { let s_ = p_atk_2; let d_ = (l_w_5).wrapping_add(144); rt::memcpy(c, d_, s_, 80); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_SetHeldAtk", &PROBE_N, &[], true); }
    r_
}

pub fn EftDisc_SetTex(c: &mut Ctx, mut p_w_1: u32, mut p_tex_2: u32, mut p_a_3: i32, mut p_b_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_SetTex", &PROBE_N, &[(p_w_1) as u32, (p_tex_2) as u32, (p_a_3) as u32, (p_b_4) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((p_a_3) as u8); let a_ = ((p_w_1).wrapping_add(48)).wrapping_add(84); c.m.w8(a_, v_); };
    { let v_ = p_tex_2; let a_ = p_w_1; c.m.w32(a_, v_); };
    { let s_ = (p_tex_2).wrapping_add((((p_a_3) as u32)).wrapping_mul(16)); let d_ = (p_w_1).wrapping_add(8); rt::memcpy(c, d_, s_, 16); };
    { let s_ = (p_tex_2).wrapping_add((((p_b_4) as u32)).wrapping_mul(16)); let d_ = (p_w_1).wrapping_add(24); rt::memcpy(c, d_, s_, 16); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_SetTex", &PROBE_N, &[], true); }
    r_
}

pub fn EftDisc_Home(c: &mut Ctx, mut p_out_1: u32, mut p_pos_2: u32, mut p_dir_3: u32, mut p_objId_4: i32, mut p_speed_5: F32, mut p_maxTurn_6: F32, mut p_bank_7: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_Home", &PROBE_N, &[(p_out_1) as u32, (p_pos_2) as u32, (p_dir_3) as u32, (p_objId_4) as u32, (p_speed_5).0, (p_maxTurn_6).0, (p_bank_7).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let mut l_opp_8: i32 = 0;
    let mut l_frames_9: F32 = F32(0);
    let mut l_t_10: F32 = F32(0);
    let mut l_d_11: F32 = F32(0);
    let mut l_angle_12: F32 = F32(0);
    let mut l_k_13: F32 = F32(0);
    let mut l_c_14: F32 = F32(0);
    let r_ = 'f: {
        l_opp_8 = F::BtlCharApi_GetOpponentObjId(c, p_objId_4);
        F::Vec3_Normalize(c, p_dir_3, p_dir_3);
        if ((((!((F::BtlCharApi_IsLockedOn(c, p_objId_4)) != 0)) as i32)) != 0) {
            F::Vec4_Copy(c, p_out_1, p_dir_3);
            break 'f;
        }
        F::BtlCharApi_GetNodePos(c, l_opp_8, 17i32, fp);
        F::Vec4_Sub(c, fp.wrapping_add(80), fp, p_pos_2);
        l_frames_9 = rt::fdiv({ let a0_ = F::Vec3_Dot(c, fp.wrapping_add(80), fp.wrapping_add(80)); F::Port_sqrtf(c, a0_) }, p_speed_5);
        p_speed_5 = l_frames_9;
        if (((rt::fgt(p_speed_5, F32(0x41700000)) as i32)) != 0) {
            p_speed_5 = F32(0x41700000);
        }
        F::BtlCharApi_GetFrameMove(c, l_opp_8, fp.wrapping_add(16));
        F::Vec4_Scale(c, fp.wrapping_add(32), fp.wrapping_add(16), rt::fmul(p_speed_5, F32(0x3f000000)));
        F::Vec4_Add(c, fp, fp, fp.wrapping_add(32));
        F::Vec4_Sub(c, fp.wrapping_add(80), fp, p_pos_2);
        F::Vec3_Normalize(c, fp.wrapping_add(80), fp.wrapping_add(80));
        F::Vec3_Normalize(c, fp.wrapping_add(64), p_dir_3);
        l_d_11 = F::Vec3_Dot(c, fp.wrapping_add(80), fp.wrapping_add(64));
        l_c_14 = F32(0x00000000);
        if (((rt::fgt(l_d_11, F32(0x00000000)) as i32)) != 0) {
            if (((rt::fgt(l_d_11, F32(0x3f800000)) as i32)) != 0) {
                l_d_11 = F32(0x3f800000);
            }
            l_angle_12 = F::Port_acosf(c, l_d_11);
            if (((rt::fgt(l_frames_9, F32(0x40000000)) as i32)) != 0) {
                { let r_ = rt::fmul(l_frames_9, F32(0x3f000000)); let n_ = rt::fdiv(l_angle_12, r_); l_angle_12 = n_; };
            }
            l_t_10 = (if (((rt::flt(p_maxTurn_6, l_angle_12) as i32)) != 0) { p_maxTurn_6 } else { l_angle_12 });
            F::Vec3_Cross(c, fp.wrapping_add(96), fp.wrapping_add(64), fp.wrapping_add(80));
            F::Vec3_Normalize(c, fp.wrapping_add(96), fp.wrapping_add(96));
            l_k_13 = rt::fmul(rt::fsub(rt::fmul(F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(4))), F32(c.m.r32((fp.wrapping_add(80)).wrapping_add(8)))), rt::fmul(F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(8))), F32(c.m.r32((fp.wrapping_add(80)).wrapping_add(4))))), p_bank_7);
            l_c_14 = (if (((rt::flt(l_k_13, F32(0xbf800000)) as i32)) != 0) { F32(0xbf800000) } else { (if (((rt::flt(F32(0x3f800000), l_k_13) as i32)) != 0) { F32(0x3f800000) } else { l_k_13 }) });
            l_k_13 = rt::ffabs(l_c_14);
            l_c_14 = rt::fsub(F32(0x3f800000), l_k_13);
            F::EftDisc_RotateAboutAxis(c, fp.wrapping_add(48), fp.wrapping_add(64), fp.wrapping_add(96), rt::fmul(l_t_10, l_c_14));
            F::Vec3_Normalize(c, fp.wrapping_add(48), fp.wrapping_add(48));
        } else {
            F::Vec4_Copy(c, fp.wrapping_add(48), fp.wrapping_add(64));
        }
        F::Vec4_Copy(c, p_out_1, fp.wrapping_add(48));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_Home", &PROBE_N, &[], true); }
    r_
}

pub fn EftDisc_RotateAboutAxis(c: &mut Ctx, mut p_out_1: u32, mut p_v_2: u32, mut p_axis_3: u32, mut p_angle_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_RotateAboutAxis", &PROBE_N, &[(p_out_1) as u32, (p_v_2) as u32, (p_axis_3) as u32, (p_angle_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_cs_5: F32 = F32(0);
    let mut l_sn_6: F32 = F32(0);
    let r_ = 'f: {
        l_cs_5 = F::Port_cosf(c, p_angle_4);
        l_sn_6 = F::Port_sinf(c, p_angle_4);
        F::Vec3_Scale(c, fp, p_v_2, l_cs_5);
        F::Vec3_Cross(c, fp.wrapping_add(16), p_axis_3, p_v_2);
        F::Vec3_Scale(c, fp.wrapping_add(16), fp.wrapping_add(16), l_sn_6);
        { let a2_ = rt::fmul(F::Vec3_Dot(c, p_axis_3, p_v_2), rt::fsub(F32(0x3f800000), l_cs_5)); F::Vec3_Scale(c, fp.wrapping_add(32), p_axis_3, a2_) };
        F::Vec3_Add(c, p_out_1, fp, fp.wrapping_add(16));
        F::Vec3_Add(c, p_out_1, p_out_1, fp.wrapping_add(32));
        { let v_ = F32(c.m.r32((p_v_2).wrapping_add(12))); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_RotateAboutAxis", &PROBE_N, &[], true); }
    r_
}

pub fn EftDisc_StepTex(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_StepTex", &PROBE_N, &[(p_w_1) as u32], false); }
    let mut l_tex_2: u32 = 0;
    let mut l_arg_3: u32 = 0;
    let mut l_bit_4: i32 = 0;
    let mut l_lo_5: u64 = 0;
    let mut l_hi_6: u64 = 0;
    let r_ = 'f: {
    l_tex_2 = c.m.r32(p_w_1);
    l_arg_3 = (p_w_1).wrapping_add(48);
    if (((((l_tex_2) != (((0i32) as u32))) as i32)) != 0) {
        if (((((((c.m.r8((l_arg_3).wrapping_add(87))) as i32)) < (3i32)) as i32)) != 0) {
            l_bit_4 = (1i32).wrapping_shl((((c.m.r8(((p_w_1).wrapping_add(48)).wrapping_add(84))) as i32)) as u32);
            if ((((!(((((c.m.r32((l_tex_2).wrapping_add(516)) as i32)) & (l_bit_4))) != 0)) as i32)) != 0) {
                l_lo_5 = F::EftVram_AddImage(c, (p_w_1).wrapping_add(8), 1i32, 0i32);
                l_hi_6 = F::EftVram_AddClut(c, (p_w_1).wrapping_add(24));
                { let v_ = ((l_lo_5) | ((l_hi_6).wrapping_shl((37i32) as u32))); let a_ = (l_tex_2).wrapping_add((((c.m.r8(((p_w_1).wrapping_add(48)).wrapping_add(84))) as u32)).wrapping_mul(16)); c.m.w64(a_, v_); };
                { let r_ = (1i32).wrapping_shl((((c.m.r8(((p_w_1).wrapping_add(48)).wrapping_add(84))) as i32)) as u32); let a_ = (l_tex_2).wrapping_add(516); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        } else {
            if ((((!(((((c.m.r32((l_tex_2).wrapping_add(516)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
                F::EftTexSet_Keep32(c, l_tex_2, 1i32, 0i32);
                { let r_ = 1i32; let a_ = (l_tex_2).wrapping_add(516); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_StepTex", &PROBE_N, &[], true); }
    r_
}

pub fn EftDiscMgr_PushHeld(c: &mut Ctx, mut p_task_1: u32, mut p_objId_2: i32, mut p_kind_3: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDiscMgr_PushHeld", &PROBE_N, &[(p_task_1) as u32, (p_objId_2) as u32, (p_kind_3) as u32], false); }
    let mut l_mgr_4: u32 = 0;
    let mut l_n_5: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32(sym::gEftDisc)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_mgr_4 = { let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) };
    if (((((l_mgr_4) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_n_5 = F::EftDiscMgr_AllocNode(c);
    if (((((l_n_5) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    { let v_ = p_task_1; let a_ = l_n_5; c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_n_5).wrapping_add(8); c.m.w32(a_, v_); };
    { let r_ = 1i32; let a_ = (l_n_5).wrapping_add(4); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    if (((((((p_kind_3) as i32)) == (1i32)) as i32)) != 0) {
        { let r_ = 16i32; let a_ = (l_n_5).wrapping_add(4); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if (((((c.m.r32(((l_mgr_4).wrapping_add(1364)).wrapping_add((((p_objId_2) as u32)).wrapping_mul(4)))) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = l_n_5; let a_ = ((l_mgr_4).wrapping_add(1364)).wrapping_add((((p_objId_2) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = l_n_5; let a_ = ((l_mgr_4).wrapping_add(1372)).wrapping_add((((p_objId_2) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    } else {
        { let v_ = l_n_5; let a_ = (c.m.r32(((l_mgr_4).wrapping_add(1372)).wrapping_add((((p_objId_2) as u32)).wrapping_mul(4)))).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = l_n_5; let a_ = ((l_mgr_4).wrapping_add(1372)).wrapping_add((((p_objId_2) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDiscMgr_PushHeld", &PROBE_N, &[], true); }
    r_
}

pub fn EftDiscMgr_PopHeld(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDiscMgr_PopHeld", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_mgr_2: u32 = 0;
    let mut l_n_3: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftDisc)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_mgr_2 = { let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) };
    if (((((l_mgr_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_n_3 = c.m.r32(((l_mgr_2).wrapping_add(1364)).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)));
    if (((((l_n_3) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = c.m.r32((l_n_3).wrapping_add(8)); let a_ = ((l_mgr_2).wrapping_add(1364)).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = 0i32; let a_ = (l_n_3).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((0i32) as u32); let a_ = (l_n_3).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = l_n_3; c.m.w32(a_, v_); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDiscMgr_PopHeld", &PROBE_N, &[], true); }
    r_
}

pub fn EftDiscMgr_AllocNode(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDiscMgr_AllocNode", &PROBE_N, &[], false); }
    let mut l_mgr_1: u32 = 0;
    let mut l_i_2: u8 = 0;
    let mut l_n_3: u32 = 0;
    let r_ = 'f: {
    l_mgr_1 = { let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) };
    if (((((((c.m.r8((l_mgr_1).wrapping_add(1620))) as i32)) >= (20i32)) as i32)) != 0) {
        { let v_ = ((0i32) as u8); let a_ = (l_mgr_1).wrapping_add(1620); c.m.w8(a_, v_); };
    }
    l_i_2 = c.m.r8((l_mgr_1).wrapping_add(1620));
    'l1: loop {
        'c2: {
            l_n_3 = ((l_mgr_1).wrapping_add(1380)).wrapping_add((((l_i_2) as u32)).wrapping_mul(12));
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            if (((((((l_i_2) as i32)) >= (20i32)) as i32)) != 0) {
                l_i_2 = ((0i32) as u8);
            }
            if ((((((c.m.r32((l_n_3).wrapping_add(4)) as i32)) == (0i32)) as i32)) != 0) {
                { let v_ = l_i_2; let a_ = (l_mgr_1).wrapping_add(1620); c.m.w8(a_, v_); };
                break 'f l_n_3;
            }
        }
        if !(((((((l_i_2) as i32)) != (((c.m.r8((l_mgr_1).wrapping_add(1620))) as i32))) as i32)) != 0) { break 'l1; }
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDiscMgr_AllocNode", &PROBE_N, &[], true); }
    r_
}

pub fn EftDisc_SpawnHeld(c: &mut Ctx, mut p_a_1: u32, mut p_hand_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_SpawnHeld", &PROBE_N, &[(p_a_1) as u32, (p_hand_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(192) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 192);
    let r_ = 'f: {
        if (((((((((c.m.r32(sym::gEftDisc)) == (((0i32) as u32))) as i32)) != 0) || (((((p_a_1) == (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((((({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((((({ let a_ = ({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }).wrapping_add(1360); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        { let s_ = { rt::memset(c, fp.wrapping_add(96), 0, 96);; { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(96).wrapping_add(0), v_); }; { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(96).wrapping_add(4), v_); }; { let s_ = p_a_1; rt::memcpy(c, fp.wrapping_add(96).wrapping_add(16), s_, 16); }; rt::memset(c, fp.wrapping_add(96).wrapping_add(32), 0, 16);; { let v_ = F32(c.m.r32((p_a_1).wrapping_add(28))); c.m.w32(fp.wrapping_add(96).wrapping_add(48), (v_).0); }; { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(96).wrapping_add(52), (v_).0); }; { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(96).wrapping_add(56), (v_).0); }; { let v_ = F32(0x3e999999); c.m.w32(fp.wrapping_add(96).wrapping_add(60), (v_).0); }; { let v_ = F32(0x3f333333); c.m.w32(fp.wrapping_add(96).wrapping_add(64), (v_).0); }; { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(96).wrapping_add(68), (v_).0); }; { let v_ = F32(0x3fc00000); c.m.w32(fp.wrapping_add(96).wrapping_add(72), (v_).0); }; { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(96).wrapping_add(76), (v_).0); }; { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(96).wrapping_add(80), (v_).0); }; { let v_ = ((0i32) as u8); c.m.w8(fp.wrapping_add(96).wrapping_add(84), v_); }; { let v_ = ((0i32) as u8); c.m.w8(fp.wrapping_add(96).wrapping_add(85), v_); }; { let v_ = (((c.m.r32((p_a_1).wrapping_add(16)) as i32)) as u8); c.m.w8(fp.wrapping_add(96).wrapping_add(86), v_); }; { let v_ = ((0i32) as u8); c.m.w8(fp.wrapping_add(96).wrapping_add(87), v_); }; { let v_ = 0i32; c.m.w32(fp.wrapping_add(96).wrapping_add(88), (v_) as u32); }; { let v_ = p_hand_2; c.m.w32(fp.wrapping_add(96).wrapping_add(92), (v_) as u32); }; fp.wrapping_add(96) }; let d_ = fp; rt::memcpy(c, d_, s_, 96); };
        's1: {
            let sv3: i32 = (c.m.r32((p_a_1).wrapping_add(24)) as i32);
            match sv3 {
                1i32 => {
                    { let v_ = ((0i32) as u8); let a_ = (fp).wrapping_add(85); c.m.w8(a_, v_); };
                    break 's1;
                }
                2i32 => {
                    { let v_ = ((1i32) as u8); let a_ = (fp).wrapping_add(85); c.m.w8(a_, v_); };
                    break 's1;
                }
                4i32 => {
                    { let v_ = ((2i32) as u8); let a_ = (fp).wrapping_add(85); c.m.w8(a_, v_); };
                    break 's1;
                }
                6i32 => {
                    { let v_ = ((3i32) as u8); let a_ = (fp).wrapping_add(85); c.m.w8(a_, v_); };
                    break 's1;
                }
                _ => {
                    { let v_ = ((0i32) as u8); let a_ = (fp).wrapping_add(85); c.m.w8(a_, v_); };
                    break 's1;
                }
            }
        }
        if ((((({ let a0_ = (c.m.r32((p_a_1).wrapping_add(20)) as i32); F::BtlObj_GetNodeSide(c, a0_) }) == (0i32)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = (fp).wrapping_add(92); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 1i32; let a_ = (fp).wrapping_add(92); c.m.w32(a_, (v_) as u32); };
        }
        break 'f ((({ let a0_ = { let a_ = ({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }).wrapping_add(1360); c.m.r32(a_) }; F::BtlTaskList_AddTail(c, a0_, sym::D_002C3B38, fp) }) != (((0i32) as u32))) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_SpawnHeld", &PROBE_N, &[], true); }
    r_
}

pub fn EftDisc_Throw(c: &mut Ctx, mut p_a_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_Throw", &PROBE_N, &[(p_a_1) as u32], false); }
    let mut l_n_2: u32 = 0;
    let mut l_task_3: u32 = 0;
    let mut l_w_4: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftDisc)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_n_2 = { let a_ = (({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }).wrapping_add(1364)).wrapping_add(((((c.m.r16((p_a_1).wrapping_add(18)) as i16)) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    'l1: while (((((((((l_n_2) != (((0i32) as u32))) as i32)) != 0) && (((((c.m.r32((l_n_2).wrapping_add(4)) as i32)) & (48i32))) != 0)) as i32)) != 0) {
        l_n_2 = c.m.r32((l_n_2).wrapping_add(8));
    }
    if (((((l_n_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f F::EftDisc_SpawnThrown(c, p_a_1);
    }
    l_task_3 = c.m.r32(l_n_2);
    if (((((l_task_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f F::EftDisc_SpawnThrown(c, p_a_1);
    }
    l_w_4 = c.m.r32((l_task_3).wrapping_add(56));
    if (((((l_w_4) == (((0i32) as u32))) as i32)) != 0) {
        break 'f F::EftDisc_SpawnThrown(c, p_a_1);
    }
    if (((((c.m.r32((l_w_4).wrapping_add(1212)) as i32)) & (1024i32))) != 0) {
        break 'f F::EftDisc_SpawnThrown(c, p_a_1);
    }
    if (((((c.m.r32((l_w_4).wrapping_add(1212)) as i32)) & (1i32))) != 0) {
        F::EftDisc_SetHeldAtk(c, l_task_3, p_a_1);
        F::Vec4_Copy(c, ((l_w_4).wrapping_add(48)).wrapping_add(16), p_a_1);
        F::Vec4_Copy(c, (l_w_4).wrapping_add(1104), p_a_1);
        { let v_ = rt::fdiv(rt::i2f(((c.m.r16((p_a_1).wrapping_add(28)) as i16)) as i32), F32(0x41f00000)); let a_ = ((l_w_4).wrapping_add(48)).wrapping_add(56); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f(((c.m.r16((p_a_1).wrapping_add(28)) as i16)) as i32); let a_ = (l_w_4).wrapping_add(1196); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (l_w_4).wrapping_add(1200); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((p_a_1).wrapping_add(32))); let a_ = ((l_w_4).wrapping_add(48)).wrapping_add(52); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((p_a_1).wrapping_add(36))); let a_ = ((l_w_4).wrapping_add(48)).wrapping_add(68); c.m.w32(a_, (v_).0); };
        { let v_ = { let v_ = rt::fmul(rt::i2f(((c.m.r16((p_a_1).wrapping_add(28)) as i16)) as i32), rt::fsub(F32(0x3f800000), F32(c.m.r32(((l_w_4).wrapping_add(48)).wrapping_add(64))))); let a_ = (l_w_4).wrapping_add(1188); c.m.w32(a_, (v_).0); v_ }; let a_ = (l_w_4).wrapping_add(1192); c.m.w32(a_, (v_).0); };
        { let r_ = 32i32; let a_ = (l_w_4).wrapping_add(1212); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let r_ = 32i32; let a_ = (l_n_2).wrapping_add(4); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let r_ = (!(8i32)); let a_ = (l_w_4).wrapping_add(1212); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_Throw", &PROBE_N, &[], true); }
    r_
}

pub fn EftDisc_SpawnThrown(c: &mut Ctx, mut p_a_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_SpawnThrown", &PROBE_N, &[(p_a_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(192) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 192);
    let mut l_task_2: u32 = 0;
    let r_ = 'f: {
        if (((((((((c.m.r32(sym::gEftDisc)) == (((0i32) as u32))) as i32)) != 0) || (((((p_a_1) == (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((((({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((((({ let a_ = ({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }).wrapping_add(1360); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        { let s_ = { rt::memset(c, fp.wrapping_add(96), 0, 96);; { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(96).wrapping_add(0), v_); }; { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(96).wrapping_add(4), v_); }; { let s_ = p_a_1; rt::memcpy(c, fp.wrapping_add(96).wrapping_add(16), s_, 16); }; rt::memset(c, fp.wrapping_add(96).wrapping_add(32), 0, 16);; { let v_ = F32(c.m.r32((p_a_1).wrapping_add(40))); c.m.w32(fp.wrapping_add(96).wrapping_add(48), (v_).0); }; { let v_ = F32(c.m.r32((p_a_1).wrapping_add(32))); c.m.w32(fp.wrapping_add(96).wrapping_add(52), (v_).0); }; { let v_ = rt::fdiv(rt::i2f(((c.m.r16((p_a_1).wrapping_add(28)) as i16)) as i32), F32(0x41f00000)); c.m.w32(fp.wrapping_add(96).wrapping_add(56), (v_).0); }; { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(96).wrapping_add(60), (v_).0); }; { let v_ = F32(0x3f333333); c.m.w32(fp.wrapping_add(96).wrapping_add(64), (v_).0); }; { let v_ = F32(c.m.r32((p_a_1).wrapping_add(36))); c.m.w32(fp.wrapping_add(96).wrapping_add(68), (v_).0); }; { let v_ = F32(0x3fc00000); c.m.w32(fp.wrapping_add(96).wrapping_add(72), (v_).0); }; { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(96).wrapping_add(76), (v_).0); }; { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(96).wrapping_add(80), (v_).0); }; { let v_ = ((0i32) as u8); c.m.w8(fp.wrapping_add(96).wrapping_add(84), v_); }; { let v_ = ((0i32) as u8); c.m.w8(fp.wrapping_add(96).wrapping_add(85), v_); }; { let v_ = (((c.m.r16((p_a_1).wrapping_add(18)) as i16)) as u8); c.m.w8(fp.wrapping_add(96).wrapping_add(86), v_); }; { let v_ = ((1i32) as u8); c.m.w8(fp.wrapping_add(96).wrapping_add(87), v_); }; { let v_ = 0i32; c.m.w32(fp.wrapping_add(96).wrapping_add(88), (v_) as u32); }; { let v_ = 0i32; c.m.w32(fp.wrapping_add(96).wrapping_add(92), (v_) as u32); }; fp.wrapping_add(96) }; let d_ = fp; rt::memcpy(c, d_, s_, 96); };
        if ((((({ let a0_ = (((c.m.r16((p_a_1).wrapping_add(20)) as i16)) as i32); F::BtlObj_GetNodeSide(c, a0_) }) == (0i32)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = (fp).wrapping_add(92); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 1i32; let a_ = (fp).wrapping_add(92); c.m.w32(a_, (v_) as u32); };
        }
        's1: {
            let sv3: i32 = ((c.m.r8((p_a_1).wrapping_add(27))) as i32);
            match sv3 {
                1i32 => {
                    { let v_ = ((0i32) as u8); let a_ = (fp).wrapping_add(85); c.m.w8(a_, v_); };
                    break 's1;
                }
                2i32 => {
                    { let v_ = ((1i32) as u8); let a_ = (fp).wrapping_add(85); c.m.w8(a_, v_); };
                    break 's1;
                }
                4i32 => {
                    { let v_ = ((2i32) as u8); let a_ = (fp).wrapping_add(85); c.m.w8(a_, v_); };
                    break 's1;
                }
                6i32 => {
                    { let v_ = ((3i32) as u8); let a_ = (fp).wrapping_add(85); c.m.w8(a_, v_); };
                    break 's1;
                }
                _ => {
                    { let v_ = ((0i32) as u8); let a_ = (fp).wrapping_add(85); c.m.w8(a_, v_); };
                    break 's1;
                }
            }
        }
        l_task_2 = { let a0_ = { let a_ = ({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }).wrapping_add(1360); c.m.r32(a_) }; F::BtlTaskList_AddTail(c, a0_, sym::D_002C3B38, fp) };
        if (((((l_task_2) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        F::EftDisc_SetHeldAtk(c, l_task_2, p_a_1);
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_SpawnThrown", &PROBE_N, &[], true); }
    r_
}

pub fn EftDisc_DropHeld(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_DropHeld", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_mgr_2: u32 = 0;
    let mut l_n_3: u32 = 0;
    let mut l_t_4: u32 = 0;
    let mut l_w_5: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftDisc)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_mgr_2 = { let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) };
    l_n_3 = c.m.r32(((l_mgr_2).wrapping_add(1364)).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)));
    if (((((l_n_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_t_4 = c.m.r32(l_n_3);
    if (((((l_t_4) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_5 = c.m.r32((l_t_4).wrapping_add(56));
    if (((((l_w_5) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32((l_w_5).wrapping_add(1212)) as i32)) & (1i32))) != 0) {
        { let r_ = 4i32; let a_ = (l_w_5).wrapping_add(1212); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let v_ = c.m.r32((l_n_3).wrapping_add(8)); let a_ = ((l_mgr_2).wrapping_add(1364)).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = 0i32; let a_ = (l_n_3).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((0i32) as u32); let a_ = (l_n_3).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = l_n_3; c.m.w32(a_, v_); };
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_DropHeld", &PROBE_N, &[], true); }
    r_
}

pub fn EftDisc_SpawnFromNode(c: &mut Ctx, mut p_a_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_SpawnFromNode", &PROBE_N, &[(p_a_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(192) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 192);
    let mut l_task_2: u32 = 0;
    let r_ = 'f: {
        if (((((((((c.m.r32(sym::gEftDisc)) == (((0i32) as u32))) as i32)) != 0) || (((((p_a_1) == (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((((({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((((({ let a_ = ({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }).wrapping_add(1360); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        { let s_ = { rt::memset(c, fp.wrapping_add(96), 0, 96);; { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(96).wrapping_add(0), v_); }; { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(96).wrapping_add(4), v_); }; rt::memset(c, fp.wrapping_add(96).wrapping_add(16), 0, 16);; rt::memset(c, fp.wrapping_add(96).wrapping_add(32), 0, 16);; { let v_ = F32(c.m.r32((p_a_1).wrapping_add(40))); c.m.w32(fp.wrapping_add(96).wrapping_add(48), (v_).0); }; { let v_ = F32(c.m.r32((p_a_1).wrapping_add(32))); c.m.w32(fp.wrapping_add(96).wrapping_add(52), (v_).0); }; { let v_ = rt::i2f(((c.m.r16((p_a_1).wrapping_add(28)) as i16)) as i32); c.m.w32(fp.wrapping_add(96).wrapping_add(56), (v_).0); }; { let v_ = F32(0x3d4ccccc); c.m.w32(fp.wrapping_add(96).wrapping_add(60), (v_).0); }; { let v_ = F32(0x3f333333); c.m.w32(fp.wrapping_add(96).wrapping_add(64), (v_).0); }; { let v_ = F32(c.m.r32((p_a_1).wrapping_add(36))); c.m.w32(fp.wrapping_add(96).wrapping_add(68), (v_).0); }; { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(96).wrapping_add(72), (v_).0); }; { let v_ = F32(0xbed70a3d); c.m.w32(fp.wrapping_add(96).wrapping_add(76), (v_).0); }; { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(96).wrapping_add(80), (v_).0); }; { let v_ = ((0i32) as u8); c.m.w8(fp.wrapping_add(96).wrapping_add(84), v_); }; { let v_ = ((0i32) as u8); c.m.w8(fp.wrapping_add(96).wrapping_add(85), v_); }; { let v_ = (((c.m.r16((p_a_1).wrapping_add(18)) as i16)) as u8); c.m.w8(fp.wrapping_add(96).wrapping_add(86), v_); }; { let v_ = ((3i32) as u8); c.m.w8(fp.wrapping_add(96).wrapping_add(87), v_); }; { let v_ = 0i32; c.m.w32(fp.wrapping_add(96).wrapping_add(88), (v_) as u32); }; { let v_ = 0i32; c.m.w32(fp.wrapping_add(96).wrapping_add(92), (v_) as u32); }; fp.wrapping_add(96) }; let d_ = fp; rt::memcpy(c, d_, s_, 96); };
        F::Vec4_Copy(c, (fp).wrapping_add(16), p_a_1);
        { let a0_ = (((c.m.r16((p_a_1).wrapping_add(18)) as i16)) as i32); let a1_ = (((c.m.r16((p_a_1).wrapping_add(20)) as i16)) as i32); F::BtlCharApi_GetNodePos(c, a0_, a1_, (fp).wrapping_add(32)) };
        l_task_2 = { let a0_ = { let a_ = ({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }).wrapping_add(1360); c.m.r32(a_) }; F::BtlTaskList_AddTail(c, a0_, sym::D_002C3B38, fp) };
        if (((((l_task_2) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        { let s_ = p_a_1; let d_ = (c.m.r32((l_task_2).wrapping_add(56))).wrapping_add(144); rt::memcpy(c, d_, s_, 80); };
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_SpawnFromNode", &PROBE_N, &[], true); }
    r_
}

pub fn EftDisc_Create(c: &mut Ctx, mut p_arg_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_Create", &PROBE_N, &[(p_arg_1) as u32], false); }
    let r_ = 'f: {
    if (((((((((c.m.r32(sym::gEftDisc)) == (((0i32) as u32))) as i32)) != 0) || (((((p_arg_1) == (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    if ((((({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    if ((((({ let a_ = ({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }).wrapping_add(1360); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    break 'f { let a0_ = { let a_ = ({ let a_ = c.m.r32(sym::gEftDisc); c.m.r32(a_) }).wrapping_add(1360); c.m.r32(a_) }; F::BtlTaskList_AddTail(c, a0_, sym::D_002C3B38, p_arg_1) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_Create", &PROBE_N, &[], true); }
    r_
}

pub fn EftDisc_Release(c: &mut Ctx, mut p_task_1: u32, p_dir_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_Release", &PROBE_N, &[(p_task_1) as u32, (p_dir_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_3: u32 = 0;
    rt::memcpy(c, fp, p_dir_2, 16);
    let r_ = 'f: {
        if (((((((((c.m.r32(sym::gEftDisc)) == (((0i32) as u32))) as i32)) != 0) || (((((p_task_1) == (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
            break 'f;
        }
        if ((((({ let a_ = c.m.r32((p_task_1).wrapping_add(40)); c.m.r32(a_) }) != (fid::EftDisc_Update)) as i32)) != 0) {
            break 'f;
        }
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        if (((((l_w_3) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if ((((!(((((c.m.r32((l_w_3).wrapping_add(1212)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
            break 'f;
        }
        if ((((!(((((c.m.r32((l_w_3).wrapping_add(1212)) as i32)) & (128i32))) != 0)) as i32)) != 0) {
            F::Vec4_Copy(c, ((l_w_3).wrapping_add(48)).wrapping_add(16), fp);
        } else {
            if (((((((rt::fle(F32(c.m.r32((l_w_3).wrapping_add(1180))), F32(0x00000000)) as i32)) != 0) || (((rt::fle(F32(c.m.r32(((l_w_3).wrapping_add(1088)).wrapping_add(12))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                { let v_ = F32(0x3f800000); let a_ = (l_w_3).wrapping_add(1180); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x42fe0000); let a_ = ((l_w_3).wrapping_add(1088)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                { let r_ = 64i32; let a_ = (l_w_3).wrapping_add(1212); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
            { let _ = { let a0_ = c.m.r32((l_w_3).wrapping_add(48)); let a3_ = ((c.m.r8(((l_w_3).wrapping_add(48)).wrapping_add(86))) as i32); F::EftAim_GetDirKeep(c, a0_, ((l_w_3).wrapping_add(48)).wrapping_add(16), ((((((l_w_3).wrapping_add(48)).wrapping_add(32)) as u64)) as i32), a3_) }; };
        }
        { let r_ = 32i32; let a_ = (l_w_3).wrapping_add(1212); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let r_ = (!(8i32)); let a_ = (l_w_3).wrapping_add(1212); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_Release", &PROBE_N, &[], true); }
    r_
}

pub fn EftDisc_RequestEnd(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_RequestEnd", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((((((c.m.r32(sym::gEftDisc)) != (((0i32) as u32))) as i32)) != 0) && (((((p_task_1) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        if ((((({ let a_ = c.m.r32((p_task_1).wrapping_add(40)); c.m.r32(a_) }) == (fid::EftDisc_Update)) as i32)) != 0) {
            l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
            if (((((l_w_2) != (((0i32) as u32))) as i32)) != 0) {
                if (((((c.m.r32((l_w_2).wrapping_add(1212)) as i32)) & (1i32))) != 0) {
                    { let r_ = 2i32; let a_ = (l_w_2).wrapping_add(1212); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_RequestEnd", &PROBE_N, &[], true); }
    r_
}

pub fn EftDisc_Kill(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_Kill", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((((((c.m.r32(sym::gEftDisc)) != (((0i32) as u32))) as i32)) != 0) && (((((p_task_1) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        if ((((({ let a_ = c.m.r32((p_task_1).wrapping_add(40)); c.m.r32(a_) }) == (fid::EftDisc_Update)) as i32)) != 0) {
            l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
            if (((((l_w_2) != (((0i32) as u32))) as i32)) != 0) {
                if (((((c.m.r32((l_w_2).wrapping_add(1212)) as i32)) & (1i32))) != 0) {
                    { let r_ = 4i32; let a_ = (l_w_2).wrapping_add(1212); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_Kill", &PROBE_N, &[], true); }
    r_
}

pub fn EftDisc_SetSpin(c: &mut Ctx, mut p_task_1: u32, mut p_spin_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_SetSpin", &PROBE_N, &[(p_task_1) as u32, (p_spin_2).0], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((((((c.m.r32(sym::gEftDisc)) != (((0i32) as u32))) as i32)) != 0) && (((((p_task_1) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        if ((((({ let a_ = c.m.r32((p_task_1).wrapping_add(40)); c.m.r32(a_) }) == (fid::EftDisc_Update)) as i32)) != 0) {
            l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
            if (((((l_w_3) != (((0i32) as u32))) as i32)) != 0) {
                if (((((c.m.r32((l_w_3).wrapping_add(1212)) as i32)) & (1i32))) != 0) {
                    { let v_ = p_spin_2; let a_ = (l_w_3).wrapping_add(1152); c.m.w32(a_, (v_).0); };
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_SetSpin", &PROBE_N, &[], true); }
    r_
}

pub fn EftDisc_SetScale(c: &mut Ctx, mut p_task_1: u32, mut p_scale_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_SetScale", &PROBE_N, &[(p_task_1) as u32, (p_scale_2).0], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((((((c.m.r32(sym::gEftDisc)) != (((0i32) as u32))) as i32)) != 0) && (((((p_task_1) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        if ((((({ let a_ = c.m.r32((p_task_1).wrapping_add(40)); c.m.r32(a_) }) == (fid::EftDisc_Update)) as i32)) != 0) {
            l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
            if (((((l_w_3) != (((0i32) as u32))) as i32)) != 0) {
                if (((((c.m.r32((l_w_3).wrapping_add(1212)) as i32)) & (1i32))) != 0) {
                    { let v_ = p_scale_2; let a_ = ((l_w_3).wrapping_add(48)).wrapping_add(48); c.m.w32(a_, (v_).0); };
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_SetScale", &PROBE_N, &[], true); }
    r_
}

pub fn EftDisc_SetModelTex(c: &mut Ctx, mut p_task_1: u32, mut p_tex_2: u32, mut p_a_3: i32, mut p_b_4: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_SetModelTex", &PROBE_N, &[(p_task_1) as u32, (p_tex_2) as u32, (p_a_3) as u32, (p_b_4) as u32], false); }
    let mut l_w_5: u32 = 0;
    let r_ = 'f: {
    if (((((((((c.m.r32(sym::gEftDisc)) != (((0i32) as u32))) as i32)) != 0) && (((((p_task_1) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        if ((((({ let a_ = c.m.r32((p_task_1).wrapping_add(40)); c.m.r32(a_) }) == (fid::EftDisc_Update)) as i32)) != 0) {
            l_w_5 = c.m.r32((p_task_1).wrapping_add(56));
            if (((((l_w_5) != (((0i32) as u32))) as i32)) != 0) {
                if (((((c.m.r32((l_w_5).wrapping_add(1212)) as i32)) & (1i32))) != 0) {
                    F::EftMesh_SetTex(c, (l_w_5).wrapping_add(944), p_tex_2);
                    F::EftMesh_SetTexBase(c, (l_w_5).wrapping_add(944), 0i32);
                    { let a1_ = c.m.r32(((l_w_5).wrapping_add(944)).wrapping_add(((72i32) as u32))); F::EftDisc_SetTex(c, l_w_5, a1_, p_a_3, p_b_4) };
                }
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_SetModelTex", &PROBE_N, &[], true); }
    r_
}

pub fn EftDisc_SetLastHit(c: &mut Ctx, mut p_task_1: u32, mut p_on_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_SetLastHit", &PROBE_N, &[(p_task_1) as u32, (p_on_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((((((c.m.r32(sym::gEftDisc)) != (((0i32) as u32))) as i32)) != 0) && (((((p_task_1) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        if ((((({ let a_ = c.m.r32((p_task_1).wrapping_add(40)); c.m.r32(a_) }) == (fid::EftDisc_Update)) as i32)) != 0) {
            l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
            if (((((l_w_3) != (((0i32) as u32))) as i32)) != 0) {
                if (((((c.m.r32((l_w_3).wrapping_add(1212)) as i32)) & (1i32))) != 0) {
                    { let v_ = p_on_2; let a_ = ((l_w_3).wrapping_add(48)).wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_SetLastHit", &PROBE_N, &[], true); }
    r_
}

pub fn EftDisc_SetFlag200(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_SetFlag200", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((((((c.m.r32(sym::gEftDisc)) != (((0i32) as u32))) as i32)) != 0) && (((((p_task_1) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        if ((((({ let a_ = c.m.r32((p_task_1).wrapping_add(40)); c.m.r32(a_) }) == (fid::EftDisc_Update)) as i32)) != 0) {
            l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
            if (((((l_w_2) != (((0i32) as u32))) as i32)) != 0) {
                if (((((c.m.r32((l_w_2).wrapping_add(1212)) as i32)) & (1i32))) != 0) {
                    { let r_ = 512i32; let a_ = (l_w_2).wrapping_add(1212); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_SetFlag200", &PROBE_N, &[], true); }
    r_
}

pub fn EftDisc_IsAlive(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_IsAlive", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftDisc)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = c.m.r32((p_task_1).wrapping_add(40)); c.m.r32(a_) }) != (fid::EftDisc_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32((l_w_2).wrapping_add(1212)) as i32)) & (1i32))) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDisc_IsAlive", &PROBE_N, &[], true); }
    r_
}
