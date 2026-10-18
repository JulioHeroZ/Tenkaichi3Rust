#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlAiSeq_IsInterrupted(c: &mut Ctx, mut p_ai_1: u32, mut p_id_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSeq_IsInterrupted", &PROBE_N, &[(p_ai_1) as u32, (p_id_2) as u32], false); }
    let mut l_body_3: u32 = 0;
    let mut l_flags_4: u16 = 0;
    let mut l_cls_5: i8 = 0;
    let mut l_kind_6: i32 = 0;
    let mut l_seq_7: u32 = 0;
    let r_ = 'f: {
    l_body_3 = ({ let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) }).wrapping_add(8);
    l_flags_4 = c.m.r16(((l_body_3).wrapping_add(640)).wrapping_add((((p_id_2) as u32)).wrapping_mul(2)));
    l_cls_5 = { let a_ = ((l_body_3).wrapping_add(896)).wrapping_add((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetUnk974(c, a0_) }) as u32)); (c.m.r8(a_) as i8) };
    l_kind_6 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetOppSkillKind(c, a0_) };
    l_seq_7 = (p_ai_1).wrapping_add(40);
    if ((((((((((((((((((l_flags_4) as i32)) & (1i32))) as u16)) as i32)) != 0) && ((((!(((((c.m.r32(l_seq_7) as i32)) & (1024i32))) != 0)) as i32)) != 0)) as i32)) != 0) && ((((((((((l_cls_5) as i32)).wrapping_sub(1i32)) as u32)) < (((7i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    if ((((((l_flags_4) as i32)) & (8i32))) != 0) {
        if (((((((((((l_cls_5) as i32)) == (32i32)) as i32)) != 0) || ((((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_IsInClashA(c, a0_) }) != (0i32)) as i32)) != 0)) as i32)) != 0) {
            break 'f 1i32;
        }
    }
    if ((((((((((l_flags_4) as i32)) & (16i32))) != 0) && (((((l_kind_6) != ((1i32).wrapping_neg())) as i32)) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSeq_IsInterrupted", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSeq_CheckHeight(c: &mut Ctx, mut p_ai_1: u32, mut p_mode_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSeq_CheckHeight", &PROBE_N, &[(p_ai_1) as u32, (p_mode_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_alt_3: F32 = F32(0);
    let mut l_height_4: F32 = F32(0);
    let mut l_limit_5: F32 = F32(0);
    let r_ = 'f: {
        l_alt_3 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetAltitude(c, a0_) };
        l_height_4 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetHeight(c, a0_) };
        l_limit_5 = rt::fadd(rt::fadd(F::BtlStage_GetTop(c), l_height_4), F32(0x41200000));
        { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetPos(c, a0_, fp) };
        's1: {
            let sv6: i32 = p_mode_2;
            match sv6 {
                0i32 => {
                    if (((rt::flt(F32(c.m.r32((fp).wrapping_add(4))), l_limit_5) as i32)) != 0) {
                        break 'f 1i32;
                    }
                    break 's1;
                }
                1i32 => {
                    if (((((((((F::BtlChar_IsStage4Or27(c)) != (0i32)) as i32)) != 0) && ((((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_TestPoseBit80(c, a0_, 1i32) }) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                        break 'f 1i32;
                    }
                    if (((rt::flt(l_alt_3, F32(0x41200000)) as i32)) != 0) {
                        break 'f 1i32;
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSeq_CheckHeight", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSeq_CheckStageAbort(c: &mut Ctx, mut p_ai_1: u32, mut p_id_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSeq_CheckStageAbort", &PROBE_N, &[(p_ai_1) as u32, (p_id_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_seq_3: u32 = 0;
    let mut l_out_4: u32 = 0;
    let mut l_cls_5: i8 = 0;
    let mut l_special_6: i32 = 0;
    let r_ = 'f: {
        l_seq_3 = (p_ai_1).wrapping_add(40);
        l_out_4 = (p_ai_1).wrapping_add(616);
        l_cls_5 = { let a_ = ((({ let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) }).wrapping_add(8)).wrapping_add(896)).wrapping_add((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetUnk974(c, a0_) }) as u32)); (c.m.r8(a_) as i8) };
        l_special_6 = F::BtlChar_IsStage4Or27(c);
        { let v_ = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_TestPoseBit80(c, a0_, 1i32) }; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = (((c.m.r32(p_ai_1) as i32)) ^ (1i32)); F::BtlCharApi_TestPoseBit80(c, a0_, 1i32) }; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetPos(c, a0_, fp.wrapping_add(8)) };
        's1: {
            let sv7: i32 = p_id_2;
            match sv7 {
                9i32 | 11i32 | 21i32 => {
                    if (((((l_special_6) != (0i32)) as i32)) != 0) {
                        if ((((((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) == (0i32)) as i32)) != 0) {
                            break 'f 1i32;
                        }
                        if (((rt::flt(F32(0xc2a00000), F32(c.m.r32((fp.wrapping_add(8)).wrapping_add(4)))) as i32)) != 0) {
                            break 'f 1i32;
                        }
                    }
                    break 's1;
                }
                13i32 => {
                    if (((((l_special_6) != (0i32)) as i32)) != 0) {
                        if ((((((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) == (0i32)) as i32)) != 0) {
                            break 'f 1i32;
                        }
                    }
                    break 's1;
                }
                15i32 => {
                    if ((((({ let a1_ = (c.m.r32((l_seq_3).wrapping_add(16)) as i32); F::BtlAiSeq_CheckHeight(c, p_ai_1, a1_) }) != (0i32)) as i32)) != 0) {
                        break 'f 1i32;
                    }
                    break 's1;
                }
                16i32 => {
                    if (((((F::BtlAiSeq_CheckHeight(c, p_ai_1, 0i32)) != (0i32)) as i32)) != 0) {
                        break 'f 1i32;
                    }
                    break 's1;
                }
                17i32 => {
                    if (((((F::BtlAiSeq_CheckHeight(c, p_ai_1, 1i32)) != (0i32)) as i32)) != 0) {
                        break 'f 1i32;
                    }
                    break 's1;
                }
                29i32 => {
                    if (((((l_special_6) != (0i32)) as i32)) != 0) {
                        if ((((((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) == (0i32)) as i32)) != 0) {
                            break 'f 1i32;
                        }
                        if ((((((c.m.r32((l_seq_3).wrapping_add(4)) as i32)) != (1i32)) as i32)) != 0) {
                            break 'f 0i32;
                        }
                        if (((((((l_cls_5) as i32)) != (12i32)) as i32)) != 0) {
                            break 'f 0i32;
                        }
                        if ((((((c.m.r32((l_out_4).wrapping_add(84)) as i32)) > (0i32)) as i32)) != 0) {
                            break 'f 0i32;
                        }
                        { let r_ = (!(4i32)); let a_ = (((p_ai_1).wrapping_add(616)).wrapping_add(12)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSeq_CheckStageAbort", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSeq_PhaseInit(c: &mut Ctx, mut p_ai_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSeq_PhaseInit", &PROBE_N, &[(p_ai_1) as u32], false); }
    let mut l_out_2: u32 = 0;
    let mut l_seq_3: u32 = 0;
    let mut l_top_4: u32 = 0;
    let mut l_body_5: u32 = 0;
    let mut l_flags_6: u16 = 0;
    let mut l_cls_7: i8 = 0;
    let r_ = 'f: {
    l_out_2 = (p_ai_1).wrapping_add(616);
    l_seq_3 = (p_ai_1).wrapping_add(40);
    l_top_4 = ((l_seq_3).wrapping_add(((((c.m.r32((l_seq_3).wrapping_add(4)) as i32)).wrapping_shl((3i32) as u32)) as u32))).wrapping_add(((12i32) as u32));
    l_body_5 = ({ let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) }).wrapping_add(8);
    l_flags_6 = { let a_ = ((l_body_5).wrapping_add(640)).wrapping_add((c.m.r32(l_top_4)).wrapping_mul(2)); c.m.r16(a_) };
    l_cls_7 = { let a_ = ((l_body_5).wrapping_add(896)).wrapping_add((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetUnk974(c, a0_) }) as u32)); (c.m.r8(a_) as i8) };
    { let v_ = 0i32; let a_ = (l_seq_3).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    F::BtlAiPad_Clear(c, l_out_2, 0i32);
    if ((((({ let a1_ = ((c.m.r32(l_top_4)) as i32); F::BtlAiSeq_IsInterrupted(c, p_ai_1, a1_) }) != (0i32)) as i32)) != 0) {
        { let v_ = 3i32; let a_ = (l_seq_3).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        break 'f;
    }
    if ((((((l_flags_6) as i32)) & (2i32))) != 0) {
        if (((((((l_cls_7) as i32)) != (0i32)) as i32)) != 0) {
            if (((((((l_cls_7) as i32)) == (10i32)) as i32)) != 0) {
                F::BtlAiPad_Clear(c, l_out_2, 1i32);
                F::BtlAiPad_Set(c, p_ai_1, 0i32, 2i32, 0i32, ((0i32) as i16), F32(0x00000000), F32(0x00000000));
            }
            break 'f;
        }
    }
    { let v_ = 1i32; let a_ = (l_seq_3).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    F::BtlAiSeq_PhaseStart(c, p_ai_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSeq_PhaseInit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSeq_PhaseStart(c: &mut Ctx, mut p_ai_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSeq_PhaseStart", &PROBE_N, &[(p_ai_1) as u32], false); }
    let mut l_seq_2: u32 = 0;
    let mut l_status_3: u32 = 0;
    let mut l_top_4: u32 = 0;
    let mut l_act_5: u32 = 0;
    let mut l_body_6: u32 = 0;
    let mut l_steps_7: u32 = 0;
    let mut l_count_8: u8 = 0;
    let mut l_pick_9: i8 = 0;
    let mut l_n_10: i32 = 0;
    let mut l_i_11: i32 = 0;
    let mut l_sel_12: i32 = 0;
    let r_ = 'f: {
    l_seq_2 = (p_ai_1).wrapping_add(40);
    l_status_3 = (p_ai_1).wrapping_add(704);
    l_top_4 = ((l_seq_2).wrapping_add(((((c.m.r32((l_seq_2).wrapping_add(4)) as i32)).wrapping_shl((3i32) as u32)) as u32))).wrapping_add(((12i32) as u32));
    l_act_5 = { let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) };
    l_body_6 = (l_act_5).wrapping_add(8);
    l_steps_7 = (c.m.r32((l_act_5).wrapping_add(4))).wrapping_add(((({ let a_ = (l_body_6).wrapping_add((c.m.r32(l_top_4)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(20));
    l_count_8 = { let a_ = ((l_body_6).wrapping_add(256)).wrapping_add(c.m.r32(l_top_4)); c.m.r8(a_) };
    l_pick_9 = { let a_ = ((l_body_6).wrapping_add(384)).wrapping_add(c.m.r32(l_top_4)); (c.m.r8(a_) as i8) };
    F::BtlAiPad_Clear(c, (p_ai_1).wrapping_add(616), 0i32);
    { let v_ = 0i32; let a_ = (l_seq_2).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (l_seq_2).wrapping_add(88); c.m.w32(a_, (v_) as u32); };
    { let r_ = (!(7744i32)); let a_ = l_seq_2; let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    { let v_ = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetUnk974(c, a0_) }; let a_ = (l_seq_2).wrapping_add(92); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetChargeRate(c, a0_) }; let a_ = (l_seq_2).wrapping_add(96); c.m.w32(a_, (v_).0); };
    { let v_ = { let a_ = (c.m.r32(sym::gBtlAi)).wrapping_add(4); F32(c.m.r32(a_)) }; let a_ = (l_seq_2).wrapping_add(100); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (l_seq_2).wrapping_add(116); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (l_seq_2).wrapping_add(120); c.m.w32(a_, (v_).0); };
    l_n_10 = 0i32;
    {
        l_i_11 = 0i32;
        'l1: loop {
            if !(((((l_i_11) < (((l_count_8) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32((l_seq_2).wrapping_add(12)) as i32)) == ((((c.m.r8(((l_steps_7).wrapping_add((((l_i_11) as u32)).wrapping_mul(20))).wrapping_add(1)) as i8)) as i32))) as i32)) != 0) {
                    { let o_ = l_n_10; let n_ = (o_).wrapping_add(1); l_n_10 = n_; };
                }
            }
            { let o_ = l_i_11; let n_ = (o_).wrapping_add(1); l_i_11 = n_; };
        }
    }
    if (((((l_n_10) == (0i32)) as i32)) != 0) {
        { let v_ = 3i32; let a_ = (l_seq_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        break 'f;
    }
    l_sel_12 = { let fp_ = c.m.r32((sym::gBtlAiPickFuncs).wrapping_add((((l_pick_9) as u32)).wrapping_mul(4))); let a2_ = (((c.m.r8((l_top_4).wrapping_add(4)) as i8)) as i32); ((D::call(c, fp_, &[((p_ai_1) as u64), ((l_n_10) as i64 as u64), ((a2_) as i64 as u64)])) as i32) };
    if (((((l_sel_12) < (0i32)) as i32)) != 0) {
        { let v_ = 3i32; let a_ = (l_seq_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        break 'f;
    }
    l_n_10 = 0i32;
    {
        l_i_11 = 0i32;
        'l3: loop {
            if !(((((l_i_11) < (((l_count_8) as i32))) as i32)) != 0) { break 'l3; }
            'c4: {
                if ((((((c.m.r32((l_seq_2).wrapping_add(12)) as i32)) == ((((c.m.r8(((l_steps_7).wrapping_add((((l_i_11) as u32)).wrapping_mul(20))).wrapping_add(1)) as i8)) as i32))) as i32)) != 0) {
                    if (((((l_sel_12) == (l_n_10)) as i32)) != 0) {
                        { let v_ = l_i_11; let a_ = (l_seq_2).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                        break 'l3;
                    }
                    { let o_ = l_n_10; let n_ = (o_).wrapping_add(1); l_n_10 = n_; };
                }
            }
            { let o_ = l_i_11; let n_ = (o_).wrapping_add(1); l_i_11 = n_; };
        }
    }
    { let v_ = 2i32; let a_ = (l_seq_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    if (((((c.m.r32(l_top_4)) == (((10i32) as u32))) as i32)) != 0) {
        if ((((((c.m.r32((l_seq_2).wrapping_add(16)) as i32)) == (0i32)) as i32)) != 0) {
            { let v_ = { let a_ = (c.m.r32(sym::gBtlAi)).wrapping_add(4); F32(c.m.r32(a_)) }; let a_ = (l_status_3).wrapping_add(36); c.m.w32(a_, (v_).0); };
        }
    } else {
        { let v_ = F32(0x00000000); let a_ = (l_status_3).wrapping_add(36); c.m.w32(a_, (v_).0); };
    }
    F::BtlAiSeq_PhaseRun(c, p_ai_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSeq_PhaseStart", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSeq_PhaseRun(c: &mut Ctx, mut p_ai_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSeq_PhaseRun", &PROBE_N, &[(p_ai_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_seq_2: u32 = 0;
    let mut l_top_3: u32 = 0;
    let mut l_act_4: u32 = 0;
    let mut l_body_5: u32 = 0;
    let mut l_timeout_6: i8 = 0;
    let mut l_step_7: u32 = 0;
    let r_ = 'f: {
        l_seq_2 = (p_ai_1).wrapping_add(40);
        l_top_3 = ((l_seq_2).wrapping_add(((((c.m.r32((l_seq_2).wrapping_add(4)) as i32)).wrapping_shl((3i32) as u32)) as u32))).wrapping_add(((12i32) as u32));
        l_act_4 = { let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) };
        l_body_5 = (l_act_4).wrapping_add(8);
        l_timeout_6 = { let a_ = ((l_body_5).wrapping_add(512)).wrapping_add(c.m.r32(l_top_3)); (c.m.r8(a_) as i8) };
        l_step_7 = (c.m.r32((l_act_4).wrapping_add(4))).wrapping_add(((({ let a_ = (l_body_5).wrapping_add((c.m.r32(l_top_3)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(20));
        { let v_ = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetUnk974(c, a0_) }; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = (((c.m.r32(p_ai_1) as i32)) ^ (1i32)); F::BtlCharApi_GetUnk974(c, a0_) }; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = ((l_body_5).wrapping_add(896)).wrapping_add((((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u32)); (c.m.r8(a_) as i8) }; let a_ = (fp.wrapping_add(8)).wrapping_add(((0i32) as u32)); c.m.w8(a_, (v_) as u8); };
        { let v_ = { let a_ = ((l_body_5).wrapping_add(896)).wrapping_add((((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)); (c.m.r8(a_) as i8) }; let a_ = (fp.wrapping_add(8)).wrapping_add(((1i32) as u32)); c.m.w8(a_, (v_) as u8); };
        l_step_7 = (l_step_7).wrapping_add(((((c.m.r32((l_seq_2).wrapping_add(16)) as i32)) as u32)).wrapping_mul(20));
        if ((((((((((((!(((((c.m.r32(l_seq_2) as i32)) & (64i32))) != 0)) as i32)) != 0) && ((((((((c.m.r8((fp.wrapping_add(8)).wrapping_add(((0i32) as u32))) as i8)) as i32)) == ((((c.m.r16((l_step_7).wrapping_add(6)) as i16)) as i32))) as i32)) != 0)) as i32)) != 0) && ((((((c.m.r32((l_seq_2).wrapping_add(92)) as i32)) != ((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32))) as i32)) != 0)) as i32)) != 0) {
            { let r_ = 64i32; let a_ = l_seq_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if ((((({ let fp_ = { let a_ = (sym::gBtlAiStepFuncs).wrapping_add(((((c.m.r8(l_step_7) as i8)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; ((D::call(c, fp_, &[((p_ai_1) as u64)])) as i32) }) != (0i32)) as i32)) != 0) {
            { let v_ = 1i32; let a_ = (l_seq_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let a_ = (l_seq_2).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            break 'f;
        }
        if ((((({ let a1_ = ((c.m.r32(l_top_3)) as i32); F::BtlAiSeq_IsInterrupted(c, p_ai_1, a1_) }) != (0i32)) as i32)) != 0) {
            { let v_ = 3i32; let a_ = (l_seq_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            break 'f;
        }
        if (((((((((((l_timeout_6) as i32)) > (0i32)) as i32)) != 0) && ((((!(((((c.m.r32(l_seq_2) as i32)) & (64i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
            if (((((F::BtlCharApi_AnyHasFlag128(c)) == (0i32)) as i32)) != 0) {
                { let a_ = (l_seq_2).wrapping_add(88); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            if ((((((((l_timeout_6) as i32)).wrapping_mul(30i32)) < ((c.m.r32((l_seq_2).wrapping_add(88)) as i32))) as i32)) != 0) {
                { let v_ = 3i32; let a_ = (l_seq_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                break 'f;
            }
        }
        if ((((({ let a1_ = ((c.m.r32(l_top_3)) as i32); F::BtlAiSeq_CheckStageAbort(c, p_ai_1, a1_) }) != (0i32)) as i32)) != 0) {
            { let v_ = 3i32; let a_ = (l_seq_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            break 'f;
        }
        if (((((c.m.r32(l_seq_2) as i32)) & (2048i32))) != 0) {
            F::BtlAiPad_Set(c, p_ai_1, 0i32, 0i32, 0i32, ((0i32) as i16), F32(0x00000000), F32(0x00000000));
        } else {
            { let a1_ = (c.m.r32((l_step_7).wrapping_add(8)) as i32); let a2_ = (c.m.r32((l_step_7).wrapping_add(12)) as i32); let a3_ = (c.m.r32((l_step_7).wrapping_add(16)) as i32); let a4_ = (c.m.r16((l_step_7).wrapping_add(4)) as i16); F::BtlAiPad_Set(c, p_ai_1, a1_, a2_, a3_, a4_, F32(0x00000000), F32(0x00000000)) };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSeq_PhaseRun", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSeq_RollPowerUpChain(c: &mut Ctx, mut p_ai_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSeq_RollPowerUpChain", &PROBE_N, &[(p_ai_1) as u32], false); }
    let mut l_first_2: u32 = 0;
    let mut l_diff_3: i32 = 0;
    let r_ = 'f: {
    l_first_2 = (((p_ai_1).wrapping_add(40)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(8));
    l_diff_3 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetUnkD74Diff(c, a0_) };
    if ((((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlSide_IsPoweredUp(c, a0_) }) == (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    's1: {
        let sv4: u32 = c.m.r32(l_first_2);
        match sv4 {
            45u32 | 46u32 => {
                if (((((l_diff_3) != (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                break 's1;
            }
            34u32 | 36u32 | 38u32 | 41u32 | 42u32 | 43u32 | 50u32 | 53u32 => {
                break 's1;
            }
            _ => {
                break 'f 0i32;
            }
        }
    }
    break 'f (((F::AiThink_RollActRate(c, p_ai_1, 76i32)) != (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSeq_RollPowerUpChain", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSeq_PhaseEnd(c: &mut Ctx, mut p_ai_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSeq_PhaseEnd", &PROBE_N, &[(p_ai_1) as u32], false); }
    let mut l_seq_2: u32 = 0;
    let mut l_cls_3: i8 = 0;
    let mut l_top_4: u32 = 0;
    let r_ = 'f: {
    l_seq_2 = (p_ai_1).wrapping_add(40);
    l_cls_3 = { let a_ = ((({ let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) }).wrapping_add(8)).wrapping_add(896)).wrapping_add((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetUnk974(c, a0_) }) as u32)); (c.m.r8(a_) as i8) };
    l_top_4 = ((l_seq_2).wrapping_add(((((c.m.r32((l_seq_2).wrapping_add(4)) as i32)).wrapping_shl((3i32) as u32)) as u32))).wrapping_add(((12i32) as u32));
    if (((((((((((l_cls_3) as i32)) == (10i32)) as i32)) != 0) && (((((c.m.r32(l_top_4)) != (((14i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        F::BtlAiPad_Clear(c, (p_ai_1).wrapping_add(616), 1i32);
        F::BtlAiPad_Set(c, p_ai_1, 0i32, 2i32, 0i32, ((0i32) as i16), F32(0x00000000), F32(0x00000000));
        break 'f;
    }
    { let a_ = (l_seq_2).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    if ((((((c.m.r32((l_seq_2).wrapping_add(4)) as i32)) <= (0i32)) as i32)) != 0) {
        if (((((F::BtlAiSeq_RollPowerUpChain(c, p_ai_1)) != (0i32)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = (l_seq_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            l_top_4 = ((l_seq_2).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(8));
            { let v_ = ((c.m.r32(l_top_4)) as i32); let a_ = (l_seq_2).wrapping_add(124); c.m.w32(a_, (v_) as u32); };
            { let v_ = ((67i32) as u32); let a_ = l_top_4; c.m.w32(a_, v_); };
            { let v_ = ((0i32) as i8); let a_ = (l_top_4).wrapping_add(4); c.m.w8(a_, (v_) as u8); };
            { let v_ = 1i32; let a_ = (l_seq_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            break 'f;
        }
        { let r_ = (!(128i32)); let a_ = l_seq_2; let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    }
    { let v_ = 0i32; let a_ = (l_seq_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSeq_PhaseEnd", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiPick_Random(c: &mut Ctx, mut p_ai_1: u32, mut p_n_2: i32, mut p_arg_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPick_Random", &PROBE_N, &[(p_ai_1) as u32, (p_n_2) as u32, (p_arg_3) as u32], false); }
    let r_ = 'f: {
    break 'f ((F::Rand_Range(c, ((p_n_2) as u32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPick_Random", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiPick_ByArg(c: &mut Ctx, mut p_ai_1: u32, mut p_n_2: i32, mut p_arg_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPick_ByArg", &PROBE_N, &[(p_ai_1) as u32, (p_n_2) as u32, (p_arg_3) as u32], false); }
    let r_ = 'f: {
    if (((((p_arg_3) < (p_n_2)) as i32)) != 0) {
        break 'f p_arg_3;
    }
    break 'f (1i32).wrapping_neg();
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPick_ByArg", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiPick_ArgSeconds(c: &mut Ctx, mut p_ai_1: u32, mut p_n_2: i32, mut p_arg_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPick_ArgSeconds", &PROBE_N, &[(p_ai_1) as u32, (p_n_2) as u32, (p_arg_3) as u32], false); }
    let mut l_seq_4: u32 = 0;
    let mut l_sel_5: i32 = 0;
    let r_ = 'f: {
    l_seq_4 = (p_ai_1).wrapping_add(40);
    if (((((p_arg_3) < (0i32)) as i32)) != 0) {
        l_sel_5 = rt::rem_i32((p_arg_3).wrapping_neg(), 10i32);
    } else {
        l_sel_5 = rt::rem_i32(p_arg_3, 10i32);
    }
    if (((((l_sel_5) == (9i32)) as i32)) != 0) {
        l_sel_5 = F::BtlAiPick_Random(c, p_ai_1, p_n_2, 0i32);
    }
    if (((((l_sel_5) >= (p_n_2)) as i32)) != 0) {
        break 'f (1i32).wrapping_neg();
    }
    { let v_ = (rt::div_i32(p_arg_3, 10i32)).wrapping_mul(30i32); let a_ = (l_seq_4).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
    break 'f l_sel_5;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPick_ArgSeconds", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiPick_ArgFrames(c: &mut Ctx, mut p_ai_1: u32, mut p_n_2: i32, mut p_arg_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPick_ArgFrames", &PROBE_N, &[(p_ai_1) as u32, (p_n_2) as u32, (p_arg_3) as u32], false); }
    let mut l_seq_4: u32 = 0;
    let mut l_sel_5: i32 = 0;
    let mut l_t_6: i32 = 0;
    let mut l_roll_7: i32 = 0;
    let r_ = 'f: {
    l_seq_4 = (p_ai_1).wrapping_add(40);
    if (((((p_arg_3) < (0i32)) as i32)) != 0) {
        l_sel_5 = rt::rem_i32((p_arg_3).wrapping_neg(), 10i32);
    } else {
        l_sel_5 = rt::rem_i32(p_arg_3, 10i32);
    }
    if (((((p_arg_3) < (0i32)) as i32)) != 0) {
        l_t_6 = (((F::Rand_Range(c, ((rt::div_i32((p_arg_3).wrapping_neg(), 10i32)) as u32))).wrapping_add(((1i32) as u32))) as i32);
    } else {
        l_t_6 = rt::div_i32(p_arg_3, 10i32);
    }
    l_roll_7 = ((F::Rand_Range(c, ((100i32) as u32))) as i32);
    { let v_ = l_t_6; let a_ = (l_seq_4).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
    if (((((p_n_2) != (5i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((l_sel_5) == (8i32)) as i32)) != 0) {
        if (((((l_roll_7) < (30i32)) as i32)) != 0) {
            { let v_ = (1i32).wrapping_neg(); let a_ = (l_seq_4).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
        } else {
            if (((((l_roll_7) < (60i32)) as i32)) != 0) {
                { let v_ = 10i32; let a_ = (l_seq_4).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = ((F::Rand_Range(c, ((10i32) as u32))) as i32); let a_ = (l_seq_4).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
            }
        }
    }
    if ((((((((((((l_sel_5).wrapping_sub(8i32)) as u32)) < (((2i32) as u32))) as i32)) != 0) || (((((l_sel_5) == (5i32)) as i32)) != 0)) as i32)) != 0) {
        l_sel_5 = F::BtlAiPick_Random(c, p_ai_1, p_n_2, 0i32);
    }
    break 'f l_sel_5;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPick_ArgFrames", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiPick_KiTarget(c: &mut Ctx, mut p_ai_1: u32, mut p_n_2: i32, mut p_arg_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPick_KiTarget", &PROBE_N, &[(p_ai_1) as u32, (p_n_2) as u32, (p_arg_3) as u32], false); }
    let mut l_seq_4: u32 = 0;
    let r_ = 'f: {
    l_seq_4 = (p_ai_1).wrapping_add(40);
    's1: {
        let sv5: i32 = p_arg_3;
        match sv5 {
            0i32 => {
                { let v_ = 100i32; let a_ = (l_seq_4).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            1i32 => {
                { let v_ = 101i32; let a_ = (l_seq_4).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = (c.m.r32(((p_ai_1).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); let a_ = (l_seq_4).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            3i32 => {
                { let v_ = (c.m.r32(((p_ai_1).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a_ = (l_seq_4).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPick_KiTarget", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiPick_SkillCharge(c: &mut Ctx, mut p_ai_1: u32, mut p_n_2: i32, mut p_arg_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPick_SkillCharge", &PROBE_N, &[(p_ai_1) as u32, (p_n_2) as u32, (p_arg_3) as u32], false); }
    let mut l_seq_4: u32 = 0;
    let mut l_plan_5: u32 = 0;
    let mut l_roll_6: i32 = 0;
    let r_ = 'f: {
    l_plan_5 = (p_ai_1).wrapping_add(744);
    l_seq_4 = (p_ai_1).wrapping_add(40);
    l_roll_6 = ((F::Rand_Range(c, ((100i32) as u32))) as i32);
    if ((((((c.m.r32((l_plan_5).wrapping_add(8)) as i32)) == ((1i32).wrapping_neg())) as i32)) != 0) {
        break 'f (1i32).wrapping_neg();
    }
    if ((((((c.m.r32((l_plan_5).wrapping_add(8)) as i32)) == (2i32)) as i32)) != 0) {
        break 'f (1i32).wrapping_neg();
    }
    if (((((l_roll_6) < (30i32)) as i32)) != 0) {
        { let v_ = 100i32; let a_ = (l_seq_4).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
    } else {
        if (((((l_roll_6) < (60i32)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = (l_seq_4).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = ((F::Rand_Range(c, ((100i32) as u32))) as i32); let a_ = (l_seq_4).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
        }
    }
    break 'f (c.m.r32((l_plan_5).wrapping_add(8)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPick_SkillCharge", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiPick_LevelDelay(c: &mut Ctx, mut p_ai_1: u32, mut p_n_2: i32, mut p_arg_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPick_LevelDelay", &PROBE_N, &[(p_ai_1) as u32, (p_n_2) as u32, (p_arg_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_seq_4: u32 = 0;
    let mut l_r_5: F32 = F32(0);
    let mut l_level_6: i32 = 0;
    let mut l_base_7: F32 = F32(0);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 20);
        { let v_ = F32(0x42340000); c.m.w32(fp.wrapping_add(0), (v_).0); }
        { let v_ = F32(0x42070000); c.m.w32(fp.wrapping_add(4), (v_).0); }
        { let v_ = F32(0x41b40000); c.m.w32(fp.wrapping_add(8), (v_).0); }
        { let v_ = F32(0x41340000); c.m.w32(fp.wrapping_add(12), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16), (v_).0); }
        l_seq_4 = (p_ai_1).wrapping_add(40);
        l_r_5 = rt::fmul(rt::i2f(((F::Rand_Range(c, ((100i32) as u32))) as i32)), F32(0x3c23d70a));
        l_level_6 = (c.m.r32((p_ai_1).wrapping_add(8)) as i32);
        l_base_7 = rt::fmul(F32(c.m.r32((fp).wrapping_add(((((if (((((l_level_6) < (0i32)) as i32)) != 0) { 0i32 } else { rt::div_i32(l_level_6, 6i32) })) as u32)).wrapping_mul(4)))), F32(0x3f000000));
        's1: {
            let sv8: i32 = p_arg_3;
            match sv8 {
                1i32 => {
                    { let v_ = rt::f2i(rt::fadd(l_base_7, rt::fmul(l_base_7, l_r_5))); let a_ = (l_seq_4).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                2i32 => {
                    if (((((l_level_6) < (0i32)) as i32)) != 0) {
                        { let v_ = 0i32; let a_ = (l_seq_4).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
                    } else {
                        { let v_ = rt::f2i(rt::fadd(rt::fmul(rt::i2f((29i32).wrapping_sub(l_level_6)), F32(0x3f308d3d)), F32(0x41200000))); let a_ = (l_seq_4).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPick_LevelDelay", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiPick_ComboBranch(c: &mut Ctx, mut p_ai_1: u32, mut p_n_2: i32, mut p_arg_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPick_ComboBranch", &PROBE_N, &[(p_ai_1) as u32, (p_n_2) as u32, (p_arg_3) as u32], false); }
    let mut l_roll_4: i32 = 0;
    let mut l_status_5: u32 = 0;
    let mut l_seq_6: u32 = 0;
    let mut l_sum_7: i32 = 0;
    let mut l_prof_8: u32 = 0;
    let mut l_kind_9: i32 = 0;
    let mut l_hi_10: u32 = 0;
    let mut l_lo_11: u32 = 0;
    let mut l_i_12: i32 = 0;
    let mut l_col_13: i32 = 0;
    let r_ = 'f: {
    l_seq_6 = (p_ai_1).wrapping_add(40);
    l_sum_7 = 0i32;
    l_status_5 = (p_ai_1).wrapping_add(704);
    l_prof_8 = { let a_ = (({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(200)).wrapping_add(((((c.m.r32((p_ai_1).wrapping_add(4)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    l_kind_9 = F::AiThink_GetBlastStep(c, p_ai_1);
    l_roll_4 = ((F::Rand_Range(c, ((100i32) as u32))) as i32);
    if (((((p_n_2) == (1i32)) as i32)) != 0) {
        { let r_ = (!(256i32)); let a_ = l_seq_6; let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        break 'f 0i32;
    }
    l_i_12 = 0i32;
    l_lo_11 = (l_prof_8).wrapping_add(((348i32) as u32));
    l_hi_10 = (l_prof_8).wrapping_add(((1052i32) as u32));
    l_col_13 = ((l_kind_9).wrapping_mul(3i32)).wrapping_add(16i32);
    {
        'l1: loop {
            if !(((((l_i_12) < (3i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let r_ = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = ((c.m.r8((l_lo_11).wrapping_add(((l_col_13) as u32)))) as i32); let a2_ = ((c.m.r8((l_hi_10).wrapping_add(((l_col_13) as u32)))) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_7).wrapping_add(r_); l_sum_7 = n_; };
                { let o_ = l_col_13; let n_ = (o_).wrapping_add(1); l_col_13 = n_; };
                if (((((((((((((l_kind_9) == (1i32)) as i32)) != 0) && (((((c.m.r32(l_seq_6) as i32)) & (256i32))) != 0)) as i32)) != 0) && (((((l_i_12) == (1i32)) as i32)) != 0)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if (((((l_roll_4) < (l_sum_7)) as i32)) != 0) {
                    if (((((((((l_kind_9) == (0i32)) as i32)) != 0) && (((((l_i_12) == (1i32)) as i32)) != 0)) as i32)) != 0) {
                        { let r_ = 256i32; let a_ = l_seq_6; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    if (((((((((l_i_12) == (2i32)) as i32)) != 0) && ((((c.m.r64(l_status_5)) & (((2305843009213693952i64) as u64)))) != 0)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    break 'f l_i_12;
                }
            }
            { let o_ = l_i_12; let n_ = (o_).wrapping_add(1); l_i_12 = n_; };
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPick_ComboBranch", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiPick_FusionSlot(c: &mut Ctx, mut p_ai_1: u32, mut p_n_2: i32, mut p_arg_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPick_FusionSlot", &PROBE_N, &[(p_ai_1) as u32, (p_n_2) as u32, (p_arg_3) as u32], false); }
    let mut l_slot_4: i32 = 0;
    let r_ = 'f: {
    l_slot_4 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_PickFusionSlot(c, a0_) };
    if (((((l_slot_4) == ((1i32).wrapping_neg())) as i32)) != 0) {
        l_slot_4 = 0i32;
    }
    break 'f l_slot_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPick_FusionSlot", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiPick_TransformSlot(c: &mut Ctx, mut p_ai_1: u32, mut p_n_2: i32, mut p_arg_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPick_TransformSlot", &PROBE_N, &[(p_ai_1) as u32, (p_n_2) as u32, (p_arg_3) as u32], false); }
    let mut l_slot_4: i32 = 0;
    let r_ = 'f: {
    l_slot_4 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_PickTransformSlot(c, a0_) };
    if (((((l_slot_4) == ((1i32).wrapping_neg())) as i32)) != 0) {
        l_slot_4 = 0i32;
    }
    break 'f l_slot_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPick_TransformSlot", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiPick_SwitchMember(c: &mut Ctx, mut p_ai_1: u32, mut p_n_2: i32, mut p_arg_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPick_SwitchMember", &PROBE_N, &[(p_ai_1) as u32, (p_n_2) as u32, (p_arg_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_seq_4: u32 = 0;
    let mut l_found_5: i32 = 0;
    let mut l_cur_6: i32 = 0;
    let mut l_roll_7: i32 = 0;
    let mut l_count_8: i32 = 0;
    let mut l_i_9: i32 = 0;
    let mut l_j_10: i32 = 0;
    let mut l_t_11: i32 = 0;
    let mut l_sel_12: i32 = 0;
    let mut l_hp_13: i32 = 0;
    let mut l_ki_14: i32 = 0;
    let r_ = 'f: {
        l_seq_4 = (p_ai_1).wrapping_add(40);
        l_found_5 = 0i32;
        l_cur_6 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetActiveMember(c, a0_) };
        l_roll_7 = ((F::Rand_Range(c, ((100i32) as u32))) as i32);
        l_count_8 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetMemberCount(c, a0_) };
        {
            l_i_9 = 0i32;
            'l1: loop {
                if !(((((l_i_9) < (l_count_8)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((((((l_cur_6) != (l_i_9)) as i32)) != 0) && ((((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetMemberHpPercent(c, a0_, l_i_9) }) != (0i32)) as i32)) != 0)) as i32)) != 0) {
                        l_hp_13 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetMemberHpPercent(c, a0_, l_i_9) };
                        l_ki_14 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetMemberKiPercent(c, a0_, l_i_9) };
                        { let v_ = (rt::f2i(rt::fmul(rt::i2f(l_hp_13), F32(0x3f333333)))).wrapping_add(rt::f2i(rt::fmul(rt::i2f(l_ki_14), F32(0x3e999999)))); let a_ = (fp).wrapping_add((((l_found_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        { let v_ = l_i_9; let a_ = (fp.wrapping_add(32)).wrapping_add((((l_found_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        { let o_ = l_found_5; let n_ = (o_).wrapping_add(1); l_found_5 = n_; };
                    }
                }
                { let o_ = l_i_9; let n_ = (o_).wrapping_add(1); l_i_9 = n_; };
            }
        }
        if (((((l_found_5) == (1i32)) as i32)) != 0) {
            { let v_ = (c.m.r32((fp.wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a_ = (l_seq_4).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
            break 'f 0i32;
        }
        {
            l_i_9 = 0i32;
            'l3: loop {
                if !(((((l_i_9) < ((l_found_5).wrapping_sub(1i32))) as i32)) != 0) { break 'l3; }
                'c4: {
                    {
                        l_j_10 = (l_found_5).wrapping_sub(1i32);
                        'l5: loop {
                            if !(((((l_j_10) > (l_i_9)) as i32)) != 0) { break 'l5; }
                            'c6: {
                                if ((((((c.m.r32((fp).wrapping_add((((l_j_10) as u32)).wrapping_mul(4))) as i32)) > ((c.m.r32((fp).wrapping_add(((((l_j_10).wrapping_sub(1i32)) as u32)).wrapping_mul(4))) as i32))) as i32)) != 0) {
                                    l_t_11 = (c.m.r32((fp).wrapping_add((((l_j_10) as u32)).wrapping_mul(4))) as i32);
                                    { let v_ = (c.m.r32((fp).wrapping_add(((((l_j_10).wrapping_sub(1i32)) as u32)).wrapping_mul(4))) as i32); let a_ = (fp).wrapping_add((((l_j_10) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = l_t_11; let a_ = (fp).wrapping_add(((((l_j_10).wrapping_sub(1i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                    l_t_11 = (c.m.r32((fp.wrapping_add(32)).wrapping_add((((l_j_10) as u32)).wrapping_mul(4))) as i32);
                                    { let v_ = (c.m.r32((fp.wrapping_add(32)).wrapping_add(((((l_j_10).wrapping_sub(1i32)) as u32)).wrapping_mul(4))) as i32); let a_ = (fp.wrapping_add(32)).wrapping_add((((l_j_10) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = l_t_11; let a_ = (fp.wrapping_add(32)).wrapping_add(((((l_j_10).wrapping_sub(1i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                }
                            }
                            { let o_ = l_j_10; let n_ = (o_).wrapping_sub(1); l_j_10 = n_; };
                        }
                    }
                }
                { let o_ = l_i_9; let n_ = (o_).wrapping_add(1); l_i_9 = n_; };
            }
        }
        's7: {
            let sv15: i32 = l_found_5;
            match sv15 {
                2i32 => {
                    if (((((l_roll_7) < (60i32)) as i32)) != 0) {
                        l_sel_12 = (c.m.r32((fp.wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32);
                    } else {
                        l_sel_12 = (c.m.r32((fp.wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32);
                    }
                    break 's7;
                }
                3i32 => {
                    if (((((l_roll_7) < (50i32)) as i32)) != 0) {
                        l_sel_12 = (c.m.r32((fp.wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32);
                    } else {
                        if (((((l_roll_7) < (85i32)) as i32)) != 0) {
                            l_sel_12 = (c.m.r32((fp.wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32);
                        } else {
                            l_sel_12 = (c.m.r32((fp.wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32);
                        }
                    }
                    break 's7;
                }
                4i32 => {
                    if (((((l_roll_7) < (40i32)) as i32)) != 0) {
                        l_sel_12 = (c.m.r32((fp.wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32);
                    } else {
                        if (((((l_roll_7) < (70i32)) as i32)) != 0) {
                            l_sel_12 = (c.m.r32((fp.wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32);
                        } else {
                            l_sel_12 = (c.m.r32((fp.wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32);
                        }
                    }
                    break 's7;
                }
                _ => {
                    l_sel_12 = 0i32;
                    break 's7;
                }
            }
        }
        { let v_ = l_sel_12; let a_ = (l_seq_4).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
        break 'f F::BtlAiPick_Random(c, p_ai_1, p_n_2, 0i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPick_SwitchMember", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiPick_DummyMode(c: &mut Ctx, mut p_ai_1: u32, mut p_n_2: i32, mut p_arg_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPick_DummyMode", &PROBE_N, &[(p_ai_1) as u32, (p_n_2) as u32, (p_arg_3) as u32], false); }
    let mut l_level_4: i32 = 0;
    let r_ = 'f: {
    l_level_4 = (c.m.r32((p_ai_1).wrapping_add(8)) as i32);
    if ((((!(((((l_level_4) < ((1i32).wrapping_neg())) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f ((l_level_4).wrapping_neg()).wrapping_sub(2i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPick_DummyMode", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiPick_PromptButton(c: &mut Ctx, mut p_ai_1: u32, mut p_n_2: i32, mut p_arg_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPick_PromptButton", &PROBE_N, &[(p_ai_1) as u32, (p_n_2) as u32, (p_arg_3) as u32], false); }
    let mut l_prompt_4: i32 = 0;
    let r_ = 'f: {
    l_prompt_4 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetPromptButtons(c, a0_) };
    if ((((l_prompt_4) & (16i32))) != 0) {
        break 'f 0i32;
    }
    if ((((l_prompt_4) & (32i32))) != 0) {
        break 'f 1i32;
    }
    if ((((l_prompt_4) & (64i32))) != 0) {
        break 'f 2i32;
    }
    if ((((l_prompt_4) & (128i32))) != 0) {
        break 'f 3i32;
    }
    if ((((l_prompt_4) & (4i32))) != 0) {
        break 'f 4i32;
    }
    break 'f (1i32).wrapping_neg();
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPick_PromptButton", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiPick_GuardMode(c: &mut Ctx, mut p_ai_1: u32, mut p_n_2: i32, mut p_arg_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPick_GuardMode", &PROBE_N, &[(p_ai_1) as u32, (p_n_2) as u32, (p_arg_3) as u32], false); }
    let mut l_seq_4: u32 = 0;
    let r_ = 'f: {
    l_seq_4 = (p_ai_1).wrapping_add(40);
    's1: {
        let sv5: i32 = p_arg_3;
        match sv5 {
            1i32 => {
                { let v_ = 0i32; let a_ = (l_seq_4).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
                break 'f 0i32;
            }
            2i32 => {
                { let v_ = 15i32; let a_ = (l_seq_4).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
                break 'f 0i32;
            }
            3i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (l_seq_4).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
                break 'f 0i32;
            }
            _ => {
            }
        }
    }
    break 'f (1i32).wrapping_neg();
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPick_GuardMode", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiPick_Direction(c: &mut Ctx, mut p_ai_1: u32, mut p_n_2: i32, mut p_arg_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPick_Direction", &PROBE_N, &[(p_ai_1) as u32, (p_n_2) as u32, (p_arg_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_seq_4: u32 = 0;
    let mut l_sel_5: i32 = 0;
    let mut l_sum_6: i32 = 0;
    let mut l_special_7: i32 = 0;
    let mut l_best_8: i32 = 0;
    let mut l_roll_9: i32 = 0;
    let mut l_i_10: i32 = 0;
    let mut l_range_11: i32 = 0;
    let mut l_radius_12: F32 = F32(0);
    let mut l_far_13: F32 = F32(0);
    let mut l_len_14: F32 = F32(0);
    let r_ = 'f: {
        l_seq_4 = (p_ai_1).wrapping_add(40);
        l_sel_5 = (if (((((p_arg_3) < (0i32)) as i32)) != 0) { rt::rem_i32((p_arg_3).wrapping_neg(), 10i32) } else { rt::rem_i32(p_arg_3, 10i32) });
        l_sum_6 = 0i32;
        l_special_7 = F::BtlChar_IsStage4Or27(c);
        if (((((p_n_2) == (1i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        if (((((((((p_n_2) == (8i32)) as i32)) != 0) || (((((p_n_2) == (4i32)) as i32)) != 0)) as i32)) != 0) {
            l_best_8 = (2i32).wrapping_neg();
            l_radius_12 = F::BtlStage_GetInnerRadius(c);
            l_range_11 = (p_n_2).wrapping_mul(10i32);
            { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetPos(c, a0_, fp) };
            l_len_14 = F::Vec3_Length(c, fp);
            l_far_13 = F32(0x00000000);
            if (((((((rt::flt(rt::fmul(l_radius_12, F32(0x3f000000)), l_len_14) as i32)) != 0) || (((((l_special_7) != (0i32)) as i32)) != 0)) as i32)) != 0) {
                { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetRot(c, a0_, fp.wrapping_add(16)) };
                { let v_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); F::Port_sinf(c, a0_) }; let a_ = fp.wrapping_add(32); c.m.w32(a_, (v_).0); };
                { let v_ = l_far_13; let a_ = (fp.wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); F::Port_cosf(c, a0_) }; let a_ = (fp.wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = l_far_13; let a_ = (fp.wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                F::Vec3_Normalize(c, fp.wrapping_add(32), fp.wrapping_add(32));
                {
                    l_i_10 = 0i32;
                    'l1: loop {
                        if !(((((l_i_10) < (p_n_2)) as i32)) != 0) { break 'l1; }
                        'c2: {
                            F::Vec3_Scale(c, fp.wrapping_add(48), fp.wrapping_add(32), F32(0x41700000));
                            F::Vec3_Add(c, fp.wrapping_add(48), fp, fp.wrapping_add(48));
                            l_len_14 = F::Vec3_Length(c, fp.wrapping_add(48));
                            if (((rt::flt(l_far_13, l_len_14) as i32)) != 0) {
                                l_far_13 = l_len_14;
                                l_best_8 = l_i_10;
                            }
                            F::Vec3_RotateY(c, fp.wrapping_add(32), fp.wrapping_add(32), F32(0x3f490fd9));
                        }
                        { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; };
                    }
                }
                l_range_11 = (if (((((p_n_2) == (8i32)) as i32)) != 0) { 59i32 } else { 33i32 });
            }
            l_roll_9 = ((F::Rand_Range(c, ((l_range_11) as u32))) as i32);
            {
                l_i_10 = 0i32;
                'l3: loop {
                    if !(((((l_i_10) < (p_n_2)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        if (((((p_n_2) == (8i32)) as i32)) != 0) {
                            if (((((((((l_best_8) != ((2i32).wrapping_neg())) as i32)) != 0) && (((((((((((((l_best_8) == (l_i_10)) as i32)) != 0) || (((((rt::rem_i32((l_best_8).wrapping_add(1i32), p_n_2)) == (l_i_10)) as i32)) != 0)) as i32)) != 0) || (((((rt::rem_i32((l_best_8).wrapping_add(7i32), p_n_2)) == (l_i_10)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
                                { let r_ = 3i32; let n_ = (l_sum_6).wrapping_add(r_); l_sum_6 = n_; };
                            } else {
                                { let r_ = 10i32; let n_ = (l_sum_6).wrapping_add(r_); l_sum_6 = n_; };
                            }
                        } else {
                            if (((((l_best_8) == (l_i_10)) as i32)) != 0) {
                                { let r_ = 3i32; let n_ = (l_sum_6).wrapping_add(r_); l_sum_6 = n_; };
                            } else {
                                { let r_ = 10i32; let n_ = (l_sum_6).wrapping_add(r_); l_sum_6 = n_; };
                            }
                        }
                        if (((((l_roll_9) < (l_sum_6)) as i32)) != 0) {
                            l_sel_5 = l_i_10;
                            break 'l3;
                        }
                    }
                    { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; };
                }
            }
        }
        if (((((l_sel_5) >= (p_n_2)) as i32)) != 0) {
            break 'f (1i32).wrapping_neg();
        }
        { let v_ = (rt::div_i32(p_arg_3, 10i32)).wrapping_mul(30i32); let a_ = (l_seq_4).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
        break 'f l_sel_5;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPick_Direction", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiPick_SetTimer(c: &mut Ctx, mut p_ai_1: u32, mut p_n_2: i32, mut p_arg_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPick_SetTimer", &PROBE_N, &[(p_ai_1) as u32, (p_n_2) as u32, (p_arg_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_arg_3; let a_ = ((p_ai_1).wrapping_add(40)).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiPick_SetTimer", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiStep_ReachClass(c: &mut Ctx, mut p_ai_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_ReachClass", &PROBE_N, &[(p_ai_1) as u32], false); }
    let mut l_seq_2: u32 = 0;
    let mut l_top_3: u32 = 0;
    let mut l_act_4: u32 = 0;
    let mut l_body_5: u32 = 0;
    let mut l_step_6: u32 = 0;
    let mut l_state_7: i32 = 0;
    let mut l_cls_8: i8 = 0;
    let mut l_flags_9: u8 = 0;
    let r_ = 'f: {
    l_seq_2 = (p_ai_1).wrapping_add(40);
    l_top_3 = ((l_seq_2).wrapping_add(((((c.m.r32((l_seq_2).wrapping_add(4)) as i32)).wrapping_shl((3i32) as u32)) as u32))).wrapping_add(((12i32) as u32));
    l_act_4 = { let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) };
    l_body_5 = (l_act_4).wrapping_add(8);
    l_step_6 = (c.m.r32((l_act_4).wrapping_add(4))).wrapping_add(((({ let a_ = (l_body_5).wrapping_add((c.m.r32(l_top_3)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(20));
    l_state_7 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetUnk974(c, a0_) };
    l_cls_8 = (c.m.r8(((l_body_5).wrapping_add(896)).wrapping_add(((l_state_7) as u32))) as i8);
    l_flags_9 = c.m.r8(((l_body_5).wrapping_add(1408)).wrapping_add(((l_state_7) as u32)));
    if ((((((((c.m.r16((l_step_6).wrapping_add(6)) as i16)) as i32)) == ((1i32).wrapping_neg())) as i32)) != 0) {
        break 'f 1i32;
    }
    if ((((((((((c.m.r32((l_seq_2).wrapping_add(92)) as i32)) == (l_state_7)) as i32)) != 0) && (((((((l_cls_8) as i32)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((((((c.m.r32(l_top_3)) == (((46i32) as u32))) as i32)) != 0) && (((((c.m.r32(l_seq_2) as i32)) & (64i32))) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    if ((((((l_flags_9) as i32)) & (2i32))) != 0) {
        break 'f 0i32;
    }
    l_step_6 = (l_step_6).wrapping_add(((((c.m.r32((l_seq_2).wrapping_add(16)) as i32)) as u32)).wrapping_mul(20));
    break 'f (((((l_cls_8) as i32)) == ((((c.m.r16((l_step_6).wrapping_add(6)) as i16)) as i32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_ReachClass", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiStep_ChargeKi(c: &mut Ctx, mut p_ai_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_ChargeKi", &PROBE_N, &[(p_ai_1) as u32], false); }
    let mut l_seq_2: u32 = 0;
    let mut l_ki_3: i32 = 0;
    let mut l_want_4: i32 = 0;
    let r_ = 'f: {
    l_seq_2 = (p_ai_1).wrapping_add(40);
    l_ki_3 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlSide_GetKi(c, a0_) };
    l_want_4 = rt::f2i(rt::fmul(rt::fdiv(rt::i2f((c.m.r32((l_seq_2).wrapping_add(84)) as i32)), F32(0x42c80000)), F32(0x47c35000)));
    if ((((({ let a0_ = ((c.m.r32(sym::gBtlAi)).wrapping_add(16)).wrapping_add(((((((c.m.r32(p_ai_1) as i32)) ^ (1i32))) as u32)).wrapping_mul(1312)); F::BtlAiSense_IsBehindOpponent(c, a0_) }) != (0i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    if ((((((c.m.r32((l_seq_2).wrapping_add(84)) as i32)) == (101i32)) as i32)) != 0) {
        if ((((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlSide_IsPoweredUp(c, a0_) }) != (0i32)) as i32)) != 0) {
            break 'f 1i32;
        }
    } else {
        if ((((!(((((l_ki_3) < (l_want_4)) as i32)) != 0)) as i32)) != 0) {
            break 'f 1i32;
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_ChargeKi", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiStep_UntilBlocked(c: &mut Ctx, mut p_ai_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_UntilBlocked", &PROBE_N, &[(p_ai_1) as u32], false); }
    let mut l_seq_2: u32 = 0;
    let mut l_act_3: u32 = 0;
    let mut l_blocked_4: i32 = 0;
    let mut l_cls_5: i8 = 0;
    let mut l_flag_6: u64 = 0;
    let r_ = 'f: {
    l_seq_2 = (p_ai_1).wrapping_add(40);
    l_act_3 = { let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) };
    l_blocked_4 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_IsBlockedByOpponent(c, a0_) };
    l_cls_5 = { let a_ = (((l_act_3).wrapping_add(8)).wrapping_add(896)).wrapping_add((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetUnk974(c, a0_) }) as u32)); (c.m.r8(a_) as i8) };
    l_flag_6 = (({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetUnk1288(c, a0_) }) & (((536870912i32) as u64)));
    if (((((((l_cls_5) as i32)) == (0i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    { let v_ = { let a_ = (c.m.r32(sym::gBtlAi)).wrapping_add(4); F32(c.m.r32(a_)) }; let a_ = (l_seq_2).wrapping_add(100); c.m.w32(a_, (v_).0); };
    if ((l_flag_6) != 0) {
        break 'f 1i32;
    }
    break 'f l_blocked_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_UntilBlocked", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiStep_Countdown(c: &mut Ctx, mut p_ai_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_Countdown", &PROBE_N, &[(p_ai_1) as u32], false); }
    let mut l_seq_2: u32 = 0;
    let r_ = 'f: {
    l_seq_2 = (p_ai_1).wrapping_add(40);
    if (((((F::BtlCharApi_AnyHasFlag128(c)) != (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let a_ = (l_seq_2).wrapping_add(84); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    break 'f ((((c.m.r32((l_seq_2).wrapping_add(84)) as i32)) < (1i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_Countdown", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiStep_BlockedOrCharged(c: &mut Ctx, mut p_ai_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_BlockedOrCharged", &PROBE_N, &[(p_ai_1) as u32], false); }
    let mut l_a_2: i32 = 0;
    let mut l_b_3: i32 = 0;
    let mut l_ret_4: i32 = 0;
    let r_ = 'f: {
    l_a_2 = F::BtlAiStep_UntilBlocked(c, p_ai_1);
    l_b_3 = F::BtlAiStep_Charged(c, p_ai_1);
    l_ret_4 = 0i32;
    if (((((((((l_a_2) != (0i32)) as i32)) != 0) || (((((l_b_3) != (0i32)) as i32)) != 0)) as i32)) != 0) {
        l_ret_4 = 1i32;
    }
    break 'f l_ret_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_BlockedOrCharged", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiStep_BlockedAndCharged(c: &mut Ctx, mut p_ai_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_BlockedAndCharged", &PROBE_N, &[(p_ai_1) as u32], false); }
    let mut l_a_2: i32 = 0;
    let mut l_b_3: i32 = 0;
    let r_ = 'f: {
    l_a_2 = F::BtlAiStep_UntilBlocked(c, p_ai_1);
    l_b_3 = F::BtlAiStep_Charged(c, p_ai_1);
    break 'f (((((((l_a_2) != (0i32)) as i32)) != 0) && (((((l_b_3) != (0i32)) as i32)) != 0)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_BlockedAndCharged", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiStep_NotHit(c: &mut Ctx, mut p_ai_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_NotHit", &PROBE_N, &[(p_ai_1) as u32], false); }
    let mut l_cls_2: i8 = 0;
    let r_ = 'f: {
    l_cls_2 = { let a_ = ((({ let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) }).wrapping_add(8)).wrapping_add(896)).wrapping_add((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetUnk974(c, a0_) }) as u32)); (c.m.r8(a_) as i8) };
    if ((((((((((l_cls_2) as i32)).wrapping_sub(1i32)) as u32)) < (((7i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_NotHit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiStep_Charged(c: &mut Ctx, mut p_ai_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_Charged", &PROBE_N, &[(p_ai_1) as u32], false); }
    let mut l_seq_2: u32 = 0;
    let mut l_rate_3: F32 = F32(0);
    let r_ = 'f: {
    l_rate_3 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetChargeRate(c, a0_) };
    l_seq_2 = (p_ai_1).wrapping_add(40);
    if (((((((((c.m.r32(l_seq_2) as i32)) & (64i32))) != 0) && (((rt::feq(l_rate_3, F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    if (((rt::flt(l_rate_3, F32(c.m.r32((l_seq_2).wrapping_add(96)))) as i32)) != 0) {
        break 'f 1i32;
    }
    { let v_ = l_rate_3; let a_ = (l_seq_2).wrapping_add(96); c.m.w32(a_, (v_).0); };
    if ((((((c.m.r32((l_seq_2).wrapping_add(84)) as i32)) == ((1i32).wrapping_neg())) as i32)) != 0) {
        if (((rt::feq(l_rate_3, F32(0x3f800000)) as i32)) != 0) {
            if ((((((((c.m.r32(l_seq_2) as i32)) & (512i32))) == (0i32)) as i32)) != 0) {
                { let r_ = 512i32; let a_ = l_seq_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            } else {
                break 'f 1i32;
            }
        }
        break 'f 0i32;
    }
    if (((rt::fle(rt::i2f((c.m.r32((l_seq_2).wrapping_add(84)) as i32)), rt::fmul(l_rate_3, F32(0x41200000))) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_Charged", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiStep_GuardUntilSafe(c: &mut Ctx, mut p_ai_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_GuardUntilSafe", &PROBE_N, &[(p_ai_1) as u32], false); }
    let mut l_seq_2: u32 = 0;
    let mut l_act_3: u32 = 0;
    let mut l_state_4: i32 = 0;
    let mut l_cls_5: i8 = 0;
    let mut l_attacking_6: i32 = 0;
    let mut l_passing_7: i32 = 0;
    let mut l_top_8: u32 = 0;
    let mut l_flags_9: u8 = 0;
    let mut l_kind_10: i32 = 0;
    let mut l_hitCls_11: i32 = 0;
    let mut l_ret_12: i32 = 0;
    let mut l_none_13: i32 = 0;
    let r_ = 'f: {
    l_seq_2 = (p_ai_1).wrapping_add(40);
    l_act_3 = { let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) };
    l_state_4 = { let a0_ = (((c.m.r32(p_ai_1) as i32)) ^ (1i32)); F::BtlCharApi_GetUnk974(c, a0_) };
    l_cls_5 = (c.m.r8((((l_act_3).wrapping_add(8)).wrapping_add(896)).wrapping_add(((l_state_4) as u32))) as i8);
    l_attacking_6 = { let a0_ = (((c.m.r32(p_ai_1) as i32)) ^ (1i32)); F::BtlCharApi_CheckUnkCAC(c, a0_) };
    l_passing_7 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_IsBlastPassing(c, a0_) };
    l_top_8 = ((l_seq_2).wrapping_add(((((c.m.r32((l_seq_2).wrapping_add(4)) as i32)).wrapping_shl((3i32) as u32)) as u32))).wrapping_add(((12i32) as u32));
    l_flags_9 = c.m.r8((((l_act_3).wrapping_add(8)).wrapping_add(1408)).wrapping_add(((l_state_4) as u32)));
    l_kind_10 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetOppSkillKind(c, a0_) };
    l_hitCls_11 = 16i32;
    l_ret_12 = 0i32;
    if ((((!((((((((l_kind_10).wrapping_sub(1i32)) as u32)) < (((2i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        l_ret_12 = 1i32;
    }
    if (((((c.m.r32(l_top_8)) == (((69i32) as u32))) as i32)) != 0) {
        if (((((((l_cls_5) as i32)) == (l_hitCls_11)) as i32)) != 0) {
            { let v_ = 1i32; let a_ = (l_seq_2).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
        } else {
            if (((((((l_cls_5) as i32)) == (17i32)) as i32)) != 0) {
                { let v_ = 2i32; let a_ = (l_seq_2).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = 0i32; let a_ = (l_seq_2).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
            }
        }
    }
    l_none_13 = 0i32;
    if ((((((c.m.r32((l_seq_2).wrapping_add(84)) as i32)) == ((1i32).wrapping_neg())) as i32)) != 0) {
        break 'f l_ret_12;
    }
    if ((((((c.m.r32((l_seq_2).wrapping_add(84)) as i32)) != (l_none_13)) as i32)) != 0) {
        break 'f (((l_passing_7) == (0i32)) as i32);
    }
    if ((((((l_flags_9) as i32)) & (2i32))) != 0) {
        break 'f 0i32;
    }
    break 'f (((l_attacking_6) == (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_GuardUntilSafe", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiStep_GuardCountdown(c: &mut Ctx, mut p_ai_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_GuardCountdown", &PROBE_N, &[(p_ai_1) as u32], false); }
    let mut l_seq_2: u32 = 0;
    let mut l_flags_3: u8 = 0;
    let mut l_attacking_4: i32 = 0;
    let r_ = 'f: {
    l_seq_2 = (p_ai_1).wrapping_add(40);
    l_flags_3 = { let a_ = ((({ let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) }).wrapping_add(8)).wrapping_add(1408)).wrapping_add((({ let a0_ = (((c.m.r32(p_ai_1) as i32)) ^ (1i32)); F::BtlCharApi_GetUnk974(c, a0_) }) as u32)); c.m.r8(a_) };
    l_attacking_4 = { let a0_ = (((c.m.r32(p_ai_1) as i32)) ^ (1i32)); F::BtlCharApi_CheckUnkCAC(c, a0_) };
    if ((((((l_flags_3) as i32)) & (2i32))) != 0) {
        break 'f 0i32;
    }
    if (((((l_attacking_4) != (0i32)) as i32)) != 0) {
        { let v_ = 30i32; let a_ = (l_seq_2).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
    }
    if (((((F::BtlCharApi_AnyHasFlag128(c)) != (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let a_ = (l_seq_2).wrapping_add(84); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    if ((((((c.m.r32((l_seq_2).wrapping_add(84)) as i32)) < (0i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_GuardCountdown", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiStep_ReachClassNoShots(c: &mut Ctx, mut p_ai_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_ReachClassNoShots", &PROBE_N, &[(p_ai_1) as u32], false); }
    let mut l_done_2: i32 = 0;
    let mut l_shots_3: i32 = 0;
    let r_ = 'f: {
    l_done_2 = F::BtlAiStep_ReachClass(c, p_ai_1);
    l_shots_3 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetBlastShots(c, a0_) };
    break 'f (((((((l_done_2) != (0i32)) as i32)) != 0) && (((((l_shots_3) == (0i32)) as i32)) != 0)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_ReachClassNoShots", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiStep_OneFrame(c: &mut Ctx, mut p_ai_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_OneFrame", &PROBE_N, &[(p_ai_1) as u32], false); }
    let mut l_seq_2: u32 = 0;
    let r_ = 'f: {
    l_seq_2 = (p_ai_1).wrapping_add(40);
    if ((((((c.m.r32((l_seq_2).wrapping_add(84)) as i32)) == ((1i32).wrapping_neg())) as i32)) != 0) {
        break 'f 1i32;
    }
    { let v_ = (1i32).wrapping_neg(); let a_ = (l_seq_2).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_OneFrame", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiStep_FacingOpponent(c: &mut Ctx, mut p_ai_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_FacingOpponent", &PROBE_N, &[(p_ai_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_seq_2: u32 = 0;
    let r_ = 'f: {
        l_seq_2 = (p_ai_1).wrapping_add(40);
        { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetRot(c, a0_, fp) };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); F::Port_sinf(c, a0_) }; let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); F::Port_cosf(c, a0_) }; let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec3_Normalize(c, fp.wrapping_add(16), fp.wrapping_add(16));
        if (((((((rt::feq(F32(c.m.r32((l_seq_2).wrapping_add(116))), F32(c.m.r32(fp.wrapping_add(16)))) as i32)) != 0) && (((rt::feq(F32(c.m.r32((l_seq_2).wrapping_add(120))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8)))) as i32)) != 0)) as i32)) != 0) {
            F::BtlAiPad_Clear(c, (p_ai_1).wrapping_add(616), 0i32);
        }
        { let v_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); let a_ = (l_seq_2).wrapping_add(120); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(fp.wrapping_add(16))); let a_ = (l_seq_2).wrapping_add(116); c.m.w32(a_, (v_).0); };
        { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetPos(c, a0_, fp.wrapping_add(32)) };
        { let a0_ = (((c.m.r32(p_ai_1) as i32)) ^ (1i32)); F::BtlCharApi_GetPos(c, a0_, fp.wrapping_add(48)) };
        F::Vec3_Sub(c, fp, fp.wrapping_add(48), fp.wrapping_add(32));
        { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        F::Vec3_Normalize(c, fp.wrapping_add(64), fp);
        if (((rt::flt(F32(0x00000000), F::Vec3_Dot(c, fp.wrapping_add(16), fp.wrapping_add(64))) as i32)) != 0) {
            break 'f 1i32;
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_FacingOpponent", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiStep_SwitchQueued(c: &mut Ctx, mut p_ai_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_SwitchQueued", &PROBE_N, &[(p_ai_1) as u32], false); }
    let mut l_seq_2: u32 = 0;
    let mut l_target_3: i32 = 0;
    let mut l_want_4: i32 = 0;
    let r_ = 'f: {
    l_seq_2 = (p_ai_1).wrapping_add(40);
    l_target_3 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetSwitchTarget(c, a0_) };
    l_want_4 = (c.m.r32((l_seq_2).wrapping_add(84)) as i32);
    break 'f (((l_want_4) == (l_target_3)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_SwitchQueued", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiStep_Unk14(c: &mut Ctx, mut p_ai_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_Unk14", &PROBE_N, &[(p_ai_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_seq_2: u32 = 0;
    let mut l_cls_3: i8 = 0;
    let mut l_roll_4: i32 = 0;
    let mut l_prof_5: u32 = 0;
    let mut l_lo_6: u32 = 0;
    let mut l_hi_7: u32 = 0;
    let mut l_a_8: i32 = 0;
    let mut l_b_9: i32 = 0;
    let r_ = 'f: {
        l_seq_2 = (p_ai_1).wrapping_add(40);
        l_cls_3 = { let a_ = (({ let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) }).wrapping_add(904)).wrapping_add((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetUnk974(c, a0_) }) as u32)); (c.m.r8(a_) as i8) };
        l_roll_4 = ((F::Rand_Range(c, ((100i32) as u32))) as i32);
        l_prof_5 = { let a_ = (({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(200)).wrapping_add(((((c.m.r32((p_ai_1).wrapping_add(4)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) };
        l_lo_6 = (l_prof_5).wrapping_add(((684i32) as u32));
        l_hi_7 = (l_prof_5).wrapping_add(((1388i32) as u32));
        's1: {
            let sv10: i32 = (c.m.r32((l_seq_2).wrapping_add(84)) as i32);
            match sv10 {
                1i32 => {
                    l_a_8 = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = ((c.m.r8((l_lo_6).wrapping_add(((4i32) as u32)))) as i32); let a2_ = ((c.m.r8((l_hi_7).wrapping_add(((4i32) as u32)))) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) };
                    l_b_9 = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = ((c.m.r8((l_lo_6).wrapping_add(((3i32) as u32)))) as i32); let a2_ = ((c.m.r8((l_hi_7).wrapping_add(((3i32) as u32)))) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) };
                    { let v_ = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetClashCountB(c, a0_) }; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a0_ = (((c.m.r32(p_ai_1) as i32)) ^ (1i32)); F::BtlCharApi_GetClashCountB(c, a0_) }; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                2i32 => {
                    l_a_8 = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = ((c.m.r8((l_lo_6).wrapping_add(((6i32) as u32)))) as i32); let a2_ = ((c.m.r8((l_hi_7).wrapping_add(((6i32) as u32)))) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) };
                    l_b_9 = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = ((c.m.r8((l_lo_6).wrapping_add(((5i32) as u32)))) as i32); let a2_ = ((c.m.r8((l_hi_7).wrapping_add(((5i32) as u32)))) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) };
                    { let v_ = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetClashCountA(c, a0_) }; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a0_ = (((c.m.r32(p_ai_1) as i32)) ^ (1i32)); F::BtlCharApi_GetClashCountA(c, a0_) }; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                _ => {
                    break 'f 1i32;
                }
            }
        }
        if ((((!((((((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) > ((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32))) as i32)) != 0)) as i32)) != 0) {
            l_a_8 = l_b_9;
        }
        if (((((l_roll_4) < (l_a_8)) as i32)) != 0) {
            { let v_ = rt::rem_i32(((c.m.r32((l_seq_2).wrapping_add(16)) as i32)).wrapping_add(1i32), 4i32); let a_ = (l_seq_2).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
        }
        's2: {
            let sv11: i32 = (c.m.r32((l_seq_2).wrapping_add(84)) as i32);
            match sv11 {
                1i32 => {
                    if (((((((l_cls_3) as i32)) == (32i32)) as i32)) != 0) {
                        break 's2;
                    }
                    break 'f 1i32;
                }
                2i32 => {
                    if (({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_IsInClashA(c, a0_) }) != 0) {
                        break 's2;
                    }
                    break 'f 1i32;
                }
                _ => {
                    break 'f 0i32;
                }
            }
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_Unk14", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiStep_Unk15(c: &mut Ctx, mut p_ai_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_Unk15", &PROBE_N, &[(p_ai_1) as u32], false); }
    let mut l_plan_2: u32 = 0;
    let mut l_busy_3: i32 = 0;
    let mut l_p_4: u32 = 0;
    let mut l_step_5: i32 = 0;
    let mut l_seq_6: u32 = 0;
    let r_ = 'f: {
    l_plan_2 = (p_ai_1).wrapping_add(744);
    l_busy_3 = F::BtlAiStep_ReachClass(c, p_ai_1);
    l_p_4 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetMoveTable(c, a0_) };
    l_seq_6 = (p_ai_1).wrapping_add(40);
    l_step_5 = (c.m.r32((l_seq_6).wrapping_add(16)) as i32);
    if (((((((l_step_5) as u32)) >= (((2i32) as u32))) as i32)) != 0) {
        break 'f 1i32;
    }
    if (((((l_busy_3) != (0i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    if ((((((((c.m.r8(((l_p_4).wrapping_add(158)).wrapping_add(((l_step_5) as u32))) as i8)) as i32)) == (4i32)) as i32)) != 0) {
        { let r_ = 1024i32; let a_ = l_seq_6; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    's1: {
        let sv7: i32 = (((c.m.r16(((l_p_4).wrapping_add(16)).wrapping_add((((l_step_5) as u32)).wrapping_mul(2))) as i16)) as i32);
        match sv7 {
            12i32 | 51i32 | 55i32 => {
                if ((((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_IsMoveSlotActive(c, a0_, ((l_step_5) as u32)) }) != (0i32)) as i32)) != 0) {
                    break 'f 1i32;
                }
                break 's1;
            }
            _ => {
            }
        }
    }
    { let v_ = 300i32; let a_ = (l_plan_2).wrapping_add(164); c.m.w32(a_, (v_) as u32); };
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_Unk15", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiStep_WaitNear2(c: &mut Ctx, mut p_ai_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_WaitNear2", &PROBE_N, &[(p_ai_1) as u32], false); }
    let mut l_seq_2: u32 = 0;
    let mut l_move_3: u32 = 0;
    let r_ = 'f: {
    l_seq_2 = (p_ai_1).wrapping_add(40);
    l_move_3 = (p_ai_1).wrapping_add(192);
    { let a_ = (l_seq_2).wrapping_add(84); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    if ((((((c.m.r32((l_seq_2).wrapping_add(84)) as i32)) > (60i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    if (((rt::flt(rt::fadd(F32(c.m.r32((l_move_3).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((l_move_3).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), { let a_ = (c.m.r32(sym::gBtlAi)).wrapping_add(4); F32(c.m.r32(a_)) }) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_WaitNear2", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiStep_FireSkill(c: &mut Ctx, mut p_ai_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_FireSkill", &PROBE_N, &[(p_ai_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_seq_2: u32 = 0;
    let mut l_plan_3: u32 = 0;
    let mut l_p_4: u32 = 0;
    let mut l_act_5: u32 = 0;
    let mut l_busy_6: i32 = 0;
    let mut l_rate_7: F32 = F32(0);
    let mut l_top_8: u32 = 0;
    let mut l_tbl_9: u32 = 0;
    let mut l_kind_10: i8 = 0;
    let mut l_uc_11: u8 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
        'fl1: loop {
            match bb_ {
                0 => {
                    l_seq_2 = (p_ai_1).wrapping_add(40);
                    l_plan_3 = (p_ai_1).wrapping_add(744);
                    l_p_4 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetSkillTable(c, a0_) };
                    l_act_5 = { let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) };
                    l_busy_6 = F::BtlAiStep_ReachClass(c, p_ai_1);
                    l_rate_7 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetUnkE44B(c, a0_) };
                    l_top_8 = ((l_seq_2).wrapping_add(((((c.m.r32((l_seq_2).wrapping_add(4)) as i32)).wrapping_shl((3i32) as u32)) as u32))).wrapping_add(((12i32) as u32));
                    if ((((((c.m.r32((l_plan_3).wrapping_add(8)) as i32)) == ((1i32).wrapping_neg())) as i32)) != 0) { bb_ = 1; continue 'fl1; } else { bb_ = 2; continue 'fl1; }
                }
                1 => {
                    break 'f 1i32;
                }
                2 => {
                    { let v_ = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetUnk974(c, a0_) }; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a0_ = (((c.m.r32(p_ai_1) as i32)) ^ (1i32)); F::BtlCharApi_GetUnk974(c, a0_) }; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_tbl_9 = (l_act_5).wrapping_add(((8i32) as u32));
                    { let v_ = (({ let a_ = ((l_tbl_9).wrapping_add(896)).wrapping_add((((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u32)); c.m.r8(a_) }) as i8); let a_ = (fp.wrapping_add(8)).wrapping_add(((0i32) as u32)); c.m.w8(a_, (v_) as u8); };
                    { let v_ = (({ let a_ = ((l_tbl_9).wrapping_add(896)).wrapping_add((((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)); c.m.r8(a_) }) as i8); let a_ = (fp.wrapping_add(8)).wrapping_add(((1i32) as u32)); c.m.w8(a_, (v_) as u8); };
                    if (((((((((l_busy_6) == (0i32)) as i32)) != 0) && ((((!(((((c.m.r32(l_seq_2) as i32)) & (4096i32))) != 0)) as i32)) != 0)) as i32)) != 0) { bb_ = 4; continue 'fl1; } else { bb_ = 6; continue 'fl1; }
                }
                3 => {
                    bb_ = 2; continue 'fl1;
                }
                4 => {
                    if (((((c.m.r32(l_seq_2) as i32)) & (64i32))) != 0) { bb_ = 7; continue 'fl1; } else { bb_ = 8; continue 'fl1; }
                }
                5 => {
                    break 'f 0i32;
                }
                6 => {
                    { let r_ = 4096i32; let a_ = l_seq_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    if (((((c.m.r32(l_top_8)) == (((63i32) as u32))) as i32)) != 0) { bb_ = 12; continue 'fl1; } else { bb_ = 13; continue 'fl1; }
                }
                7 => {
                    if ((((({ let a_ = ((l_p_4).wrapping_add(412)).wrapping_add(((((c.m.r32((l_plan_3).wrapping_add(8)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) > ({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlSide_GetKi(c, a0_) })) as i32)) != 0) { bb_ = 9; continue 'fl1; } else { bb_ = 10; continue 'fl1; }
                }
                8 => {
                    bb_ = 5; continue 'fl1;
                }
                9 => {
                    break 'f 1i32;
                }
                10 => {
                    { let v_ = (((((c.m.r32(l_seq_2) as i32)) ^ (2048i32))) & ((!(64i32)))); let a_ = l_seq_2; c.m.w32(a_, (v_) as u32); };
                    bb_ = 8; continue 'fl1;
                }
                11 => {
                    bb_ = 10; continue 'fl1;
                }
                12 => {
                    break 'f 1i32;
                }
                13 => {
                    { let v_ = 0i32; let a_ = (l_seq_2).wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                    if ((((!((((((((((({ let a_ = ((l_p_4).wrapping_add(357)).wrapping_add((((c.m.r32((l_plan_3).wrapping_add(8)) as i32)) as u32)); (c.m.r8(a_) as i8) }) as i32)) == (5i32)) as i32)) != 0) && (((((F::BtlChar_IsStage4Or27(c)) != (0i32)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) { bb_ = 15; continue 'fl1; } else { bb_ = 16; continue 'fl1; }
                }
                14 => {
                    bb_ = 13; continue 'fl1;
                }
                15 => {
                    bb_ = 17; continue 'fl1;
                }
                16 => {
                    if ((((((((c.m.r8((fp.wrapping_add(8)).wrapping_add(((0i32) as u32))) as i8)) as i32)) != (22i32)) as i32)) != 0) { bb_ = 19; continue 'fl1; } else { bb_ = 20; continue 'fl1; }
                }
                17 => {
                    l_kind_10 = { let a_ = ((l_p_4).wrapping_add(318)).wrapping_add((((c.m.r32((l_plan_3).wrapping_add(8)) as i32)) as u32)); (c.m.r8(a_) as i8) };
                    if (((((((l_kind_10) as i32)) == (2i32)) as i32)) != 0) { bb_ = 26; continue 'fl1; } else { bb_ = 27; continue 'fl1; }
                }
                18 => {
                    bb_ = 16; continue 'fl1;
                }
                19 => {
                    break 'f 1i32;
                }
                20 => {
                    { let v_ = 3i32; let a_ = (l_seq_2).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                    if ((((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_TestPoseBit80(c, a0_, 1i32) }) == (0i32)) as i32)) != 0) { bb_ = 22; continue 'fl1; } else { bb_ = 23; continue 'fl1; }
                }
                21 => {
                    bb_ = 20; continue 'fl1;
                }
                22 => {
                    { let v_ = 2i32; let a_ = (l_seq_2).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                    break 'f 0i32;
                }
                23 => {
                    bb_ = 5; continue 'fl1;
                }
                24 => {
                    bb_ = 23; continue 'fl1;
                }
                25 => {
                    bb_ = 17; continue 'fl1;
                }
                26 => {
                    if ((((((((c.m.r8((fp.wrapping_add(8)).wrapping_add(((0i32) as u32))) as i8)) as i32)) != (22i32)) as i32)) != 0) { bb_ = 28; continue 'fl1; } else { bb_ = 29; continue 'fl1; }
                }
                27 => {
                    break 'f 1i32;
                }
                28 => {
                    break 'f 1i32;
                }
                29 => {
                    l_uc_11 = (((c.m.r8((fp.wrapping_add(8)).wrapping_add(((1i32) as u32))) as i8)) as u8);
                    if ((((((((c.m.r8((fp.wrapping_add(8)).wrapping_add(((1i32) as u32))) as i8)) as i32)) == (((l_kind_10) as i32))) as i32)) != 0) { bb_ = 31; continue 'fl1; } else { bb_ = 32; continue 'fl1; }
                }
                30 => {
                    bb_ = 29; continue 'fl1;
                }
                31 => {
                    break 'f 0i32;
                }
                32 => {
                    if ((((((((c.m.r8((fp.wrapping_add(8)).wrapping_add(((1i32) as u32))) as i8)) as i32)) != (28i32)) as i32)) != 0) { bb_ = 34; continue 'fl1; } else { bb_ = 35; continue 'fl1; }
                }
                33 => {
                    bb_ = 32; continue 'fl1;
                }
                34 => {
                    if ((((((((((((c.m.r8((fp.wrapping_add(8)).wrapping_add(((1i32) as u32))) as i8)) as i32)) != ((((c.m.r8((fp.wrapping_add(8)).wrapping_add(((0i32) as u32))) as i8)) as i32))) as i32)) != 0) && ((((((((c.m.r8((fp.wrapping_add(8)).wrapping_add(((1i32) as u32))) as i8)) as i32)) != (23i32)) as i32)) != 0)) as i32)) != 0) { bb_ = 36; continue 'fl1; } else { bb_ = 37; continue 'fl1; }
                }
                35 => {
                    bb_ = 27; continue 'fl1;
                }
                36 => {
                    if ((((((((c.m.r8((fp.wrapping_add(8)).wrapping_add(((1i32) as u32))) as i8)) as i32)) == (25i32)) as i32)) != 0) { bb_ = 38; continue 'fl1; } else { bb_ = 39; continue 'fl1; }
                }
                37 => {
                    bb_ = 35; continue 'fl1;
                }
                38 => {
                    break 'f 1i32;
                }
                39 => {
                    if ((((((((c.m.r8((fp.wrapping_add(8)).wrapping_add(((1i32) as u32))) as i8)) as i32)) == (24i32)) as i32)) != 0) { bb_ = 41; continue 'fl1; } else { bb_ = 42; continue 'fl1; }
                }
                40 => {
                    bb_ = 39; continue 'fl1;
                }
                41 => {
                    break 'f 1i32;
                }
                42 => {
                    if ((((((((((l_uc_11) as i32)).wrapping_sub(15i32)) as u32)) < (((3i32) as u32))) as i32)) != 0) { bb_ = 44; continue 'fl1; } else { bb_ = 45; continue 'fl1; }
                }
                43 => {
                    bb_ = 42; continue 'fl1;
                }
                44 => {
                    break 'f 1i32;
                }
                45 => {
                    if (((((rt::f2i(rt::fmul(l_rate_7, F32(0x42c80000)))) < ((c.m.r32((l_seq_2).wrapping_add(84)) as i32))) as i32)) != 0) { bb_ = 47; continue 'fl1; } else { bb_ = 48; continue 'fl1; }
                }
                46 => {
                    bb_ = 45; continue 'fl1;
                }
                47 => {
                    break 'f 0i32;
                }
                48 => {
                    bb_ = 37; continue 'fl1;
                }
                49 => {
                    bb_ = 48; continue 'fl1;
                }
                50 => {
                    break 'fl1;
                }
                _ => unreachable!(),
            }
        }
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_FireSkill", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiStep_Unk18(c: &mut Ctx, mut p_ai_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_Unk18", &PROBE_N, &[(p_ai_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_seq_2: u32 = 0;
    let mut l_dist_3: i32 = 0;
    let mut l_t_4: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 20);
        { let v_ = 15i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
        { let v_ = 7i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
        { let v_ = 5i32; c.m.w32(fp.wrapping_add(8), (v_) as u32); }
        { let v_ = 3i32; c.m.w32(fp.wrapping_add(12), (v_) as u32); }
        { let v_ = 3i32; c.m.w32(fp.wrapping_add(16), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(20), 0, 20);
        { let v_ = 15i32; c.m.w32(fp.wrapping_add(20).wrapping_add(0), (v_) as u32); }
        { let v_ = 10i32; c.m.w32(fp.wrapping_add(20).wrapping_add(4), (v_) as u32); }
        { let v_ = 7i32; c.m.w32(fp.wrapping_add(20).wrapping_add(8), (v_) as u32); }
        { let v_ = 5i32; c.m.w32(fp.wrapping_add(20).wrapping_add(12), (v_) as u32); }
        { let v_ = 3i32; c.m.w32(fp.wrapping_add(20).wrapping_add(16), (v_) as u32); }
        l_seq_2 = (p_ai_1).wrapping_add(40);
        l_dist_3 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetHp(c, a0_) };
        l_t_4 = (c.m.r32((l_seq_2).wrapping_add(84)) as i32);
        { let v_ = 1i32; let a_ = (l_seq_2).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
        { let v_ = (l_t_4).wrapping_sub(1i32); let a_ = (l_seq_2).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
        if ((((((((l_t_4).wrapping_sub(2i32)) as u32)) >= (((15i32) as u32))) as i32)) != 0) {
            if (((((l_dist_3) < (10000i32)) as i32)) != 0) {
                { let v_ = { let a_ = (fp).wrapping_add(((((if ((((((c.m.r32((p_ai_1).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) { 0i32 } else { rt::div_i32((c.m.r32((p_ai_1).wrapping_add(8)) as i32), 6i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (l_seq_2).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = { let a_ = (fp.wrapping_add(20)).wrapping_add(((((if ((((((c.m.r32((p_ai_1).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) { 0i32 } else { rt::div_i32((c.m.r32((p_ai_1).wrapping_add(8)) as i32), 6i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (l_seq_2).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
            }
            { let v_ = 0i32; let a_ = (l_seq_2).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
        }
        break 'f F::BtlAiStep_NotHit(c, p_ai_1);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_Unk18", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiStep_Unk19(c: &mut Ctx, mut p_ai_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_Unk19", &PROBE_N, &[(p_ai_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_seq_2: u32 = 0;
    let mut l_dist_3: i32 = 0;
    let mut l_t_4: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 20);
        { let v_ = 15i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
        { let v_ = 7i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
        { let v_ = 5i32; c.m.w32(fp.wrapping_add(8), (v_) as u32); }
        { let v_ = 3i32; c.m.w32(fp.wrapping_add(12), (v_) as u32); }
        { let v_ = 3i32; c.m.w32(fp.wrapping_add(16), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(20), 0, 20);
        { let v_ = 15i32; c.m.w32(fp.wrapping_add(20).wrapping_add(0), (v_) as u32); }
        { let v_ = 10i32; c.m.w32(fp.wrapping_add(20).wrapping_add(4), (v_) as u32); }
        { let v_ = 7i32; c.m.w32(fp.wrapping_add(20).wrapping_add(8), (v_) as u32); }
        { let v_ = 5i32; c.m.w32(fp.wrapping_add(20).wrapping_add(12), (v_) as u32); }
        { let v_ = 3i32; c.m.w32(fp.wrapping_add(20).wrapping_add(16), (v_) as u32); }
        l_seq_2 = (p_ai_1).wrapping_add(40);
        l_dist_3 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetHp(c, a0_) };
        l_t_4 = (c.m.r32((l_seq_2).wrapping_add(84)) as i32);
        { let v_ = 1i32; let a_ = (l_seq_2).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
        { let v_ = (l_t_4).wrapping_sub(1i32); let a_ = (l_seq_2).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
        if ((((((((l_t_4).wrapping_sub(2i32)) as u32)) >= (((15i32) as u32))) as i32)) != 0) {
            if (((((l_dist_3) < (10000i32)) as i32)) != 0) {
                { let v_ = { let a_ = (fp).wrapping_add(((((if ((((((c.m.r32((p_ai_1).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) { 0i32 } else { rt::div_i32((c.m.r32((p_ai_1).wrapping_add(8)) as i32), 6i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (l_seq_2).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = { let a_ = (fp.wrapping_add(20)).wrapping_add(((((if ((((((c.m.r32((p_ai_1).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) { 0i32 } else { rt::div_i32((c.m.r32((p_ai_1).wrapping_add(8)) as i32), 6i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (l_seq_2).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
            }
            { let v_ = 0i32; let a_ = (l_seq_2).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
        }
        break 'f ((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetStunTimer(c, a0_) }) < (1i32)) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_Unk19", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiStep_Unk20(c: &mut Ctx, mut p_ai_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_Unk20", &PROBE_N, &[(p_ai_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_IsOppSkillFlag4(c, a0_) }) == (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_Unk20", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiStep_Unk21(c: &mut Ctx, mut p_ai_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_Unk21", &PROBE_N, &[(p_ai_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_seq_2: u32 = 0;
    let mut l_dist_3: i32 = 0;
    let mut l_t_4: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 20);
        { let v_ = 15i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
        { let v_ = 7i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
        { let v_ = 5i32; c.m.w32(fp.wrapping_add(8), (v_) as u32); }
        { let v_ = 3i32; c.m.w32(fp.wrapping_add(12), (v_) as u32); }
        { let v_ = 3i32; c.m.w32(fp.wrapping_add(16), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(20), 0, 20);
        { let v_ = 15i32; c.m.w32(fp.wrapping_add(20).wrapping_add(0), (v_) as u32); }
        { let v_ = 10i32; c.m.w32(fp.wrapping_add(20).wrapping_add(4), (v_) as u32); }
        { let v_ = 7i32; c.m.w32(fp.wrapping_add(20).wrapping_add(8), (v_) as u32); }
        { let v_ = 5i32; c.m.w32(fp.wrapping_add(20).wrapping_add(12), (v_) as u32); }
        { let v_ = 3i32; c.m.w32(fp.wrapping_add(20).wrapping_add(16), (v_) as u32); }
        l_seq_2 = (p_ai_1).wrapping_add(40);
        l_dist_3 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetHp(c, a0_) };
        l_t_4 = (c.m.r32((l_seq_2).wrapping_add(84)) as i32);
        { let v_ = 1i32; let a_ = (l_seq_2).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
        { let v_ = (l_t_4).wrapping_sub(1i32); let a_ = (l_seq_2).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
        if ((((((((l_t_4).wrapping_sub(2i32)) as u32)) >= (((15i32) as u32))) as i32)) != 0) {
            if (((((l_dist_3) < (10000i32)) as i32)) != 0) {
                { let v_ = { let a_ = (fp).wrapping_add(((((if ((((((c.m.r32((p_ai_1).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) { 0i32 } else { rt::div_i32((c.m.r32((p_ai_1).wrapping_add(8)) as i32), 6i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (l_seq_2).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = { let a_ = (fp.wrapping_add(20)).wrapping_add(((((if ((((((c.m.r32((p_ai_1).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) { 0i32 } else { rt::div_i32((c.m.r32((p_ai_1).wrapping_add(8)) as i32), 6i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (l_seq_2).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
            }
            { let v_ = 0i32; let a_ = (l_seq_2).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
        }
        break 'f ((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_TestFlagBE(c, a0_) }) == (0i32)) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_Unk21", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiStep_WaitNear3(c: &mut Ctx, mut p_ai_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_WaitNear3", &PROBE_N, &[(p_ai_1) as u32], false); }
    let mut l_seq_2: u32 = 0;
    let mut l_move_3: u32 = 0;
    let r_ = 'f: {
    l_seq_2 = (p_ai_1).wrapping_add(40);
    l_move_3 = (p_ai_1).wrapping_add(192);
    { let a_ = (l_seq_2).wrapping_add(84); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    if ((((((c.m.r32((l_seq_2).wrapping_add(84)) as i32)) > (60i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    if (((rt::flt(rt::fmul(F32(c.m.r32((l_move_3).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(0x40400000)), { let a_ = (c.m.r32(sym::gBtlAi)).wrapping_add(4); F32(c.m.r32(a_)) }) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f F::BtlAiStep_ReachClass(c, p_ai_1);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_WaitNear3", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiStep_Unk23(c: &mut Ctx, mut p_ai_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_Unk23", &PROBE_N, &[(p_ai_1) as u32], false); }
    let mut l_seq_2: u32 = 0;
    let mut l_busy_3: i32 = 0;
    let mut l_p_4: u32 = 0;
    let mut l_cls_5: i8 = 0;
    let mut l_n_6: i32 = 0;
    let r_ = 'f: {
    l_seq_2 = (p_ai_1).wrapping_add(40);
    l_busy_3 = F::BtlAiStep_ReachClass(c, p_ai_1);
    l_p_4 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetSkillTable(c, a0_) };
    l_cls_5 = { let a_ = (({ let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) }).wrapping_add(904)).wrapping_add((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetUnk974(c, a0_) }) as u32)); (c.m.r8(a_) as i8) };
    if (((((l_busy_3) == (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((F::BtlChar_IsStage4Or27(c)) == (0i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    's1: {
        let sv7: i32 = (c.m.r32((l_seq_2).wrapping_add(124)) as i32);
        match sv7 {
            50i32 => {
                l_n_6 = 7i32;
                break 's1;
            }
            42i32 => {
                l_n_6 = 4i32;
                break 's1;
            }
            36i32 => {
                l_n_6 = 2i32;
                break 's1;
            }
            53i32 => {
                l_n_6 = 8i32;
                break 's1;
            }
            38i32 => {
                l_n_6 = 6i32;
                break 's1;
            }
            34i32 => {
                l_n_6 = 1i32;
                break 's1;
            }
            43i32 => {
                l_n_6 = 5i32;
                break 's1;
            }
            41i32 => {
                l_n_6 = 3i32;
                break 's1;
            }
            45i32 | 46i32 => {
                l_n_6 = 0i32;
                break 's1;
            }
            _ => {
                break 'f 0i32;
            }
        }
    }
    if ((((((({ let a_ = ((l_p_4).wrapping_add(357)).wrapping_add((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetParamByte8F(c, a0_, l_n_6) }) as u32)); (c.m.r8(a_) as i8) }) as i32)) != (5i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    { let v_ = 0i32; let a_ = (l_seq_2).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    if ((((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_TestPoseBit80(c, a0_, 1i32) }) == (0i32)) as i32)) != 0) {
        { let v_ = 1i32; let a_ = (l_seq_2).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    }
    break 'f (((((l_cls_5) as i32)) == (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiStep_Unk23", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAi_RunInstant(c: &mut Ctx, mut p_ai_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAi_RunInstant", &PROBE_N, &[(p_ai_1) as u32], false); }
    let mut l_seq_2: u32 = 0;
    let r_ = 'f: {
    l_seq_2 = (p_ai_1).wrapping_add(40);
    's1: {
        let sv3: i32 = (({ let a_ = (((l_seq_2).wrapping_add(((((c.m.r32((l_seq_2).wrapping_add(4)) as i32)).wrapping_shl((3i32) as u32)) as u32))).wrapping_add(((12i32) as u32))).wrapping_add(4); (c.m.r8(a_) as i8) }) as i32);
        match sv3 {
            0i32 => {
                { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_MarkIncomingBlast(c, a0_) };
                break 's1;
            }
            1i32 => {
                { let r_ = 128i32; let a_ = l_seq_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                F::BtlAi_NoteOpponent(c, p_ai_1, 131072i32);
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAi_RunInstant", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAi_RunSeq(c: &mut Ctx, mut p_ai_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAi_RunSeq", &PROBE_N, &[(p_ai_1) as u32], false); }
    let mut l_out_2: u32 = 0;
    let mut l_seq_3: u32 = 0;
    let r_ = 'f: {
    l_out_2 = (p_ai_1).wrapping_add(616);
    l_seq_3 = (p_ai_1).wrapping_add(40);
    F::BtlAiPad_Clear(c, l_out_2, 1i32);
    if ((((((c.m.r32((l_seq_3).wrapping_add(4)) as i32)) > (0i32)) as i32)) != 0) {
        's1: {
            let sv4: u32 = { let a_ = ((l_seq_3).wrapping_add(((((c.m.r32((l_seq_3).wrapping_add(4)) as i32)).wrapping_shl((3i32) as u32)) as u32))).wrapping_add(((12i32) as u32)); c.m.r32(a_) };
            match sv4 {
                3u32 => {
                    F::BtlAi_RunInstant(c, p_ai_1);
                    { let a_ = (l_seq_3).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    break 's1;
                }
                7u32 => {
                    F::BtlAiMove_Dispatch(c, p_ai_1);
                    break 's1;
                }
                25u32 => {
                    F::BtlAiCombo_Dispatch(c, p_ai_1);
                    break 's1;
                }
                26u32 => {
                    F::BtlAiFollow_Dispatch(c, p_ai_1);
                    break 's1;
                }
                54u32 => {
                    F::BtlAiAct36_Dispatch(c, p_ai_1);
                    break 's1;
                }
                _ => {
                    { let fp_ = { let a_ = (sym::gBtlAiStateFuncs).wrapping_add(((((c.m.r32((l_seq_3).wrapping_add(8)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; { let _ = D::call(c, fp_, &[((p_ai_1) as u64)]); } };
                    break 's1;
                }
            }
        }
        F::BtlAiPad_EndFrame(c, l_out_2);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAi_RunSeq", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAi_SendInput(c: &mut Ctx, mut p_ai_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAi_SendInput", &PROBE_N, &[(p_ai_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32(p_ai_1) as i32); let a1_ = c.m.r32((p_ai_1).wrapping_add(616)); let a2_ = F32(c.m.r32(((p_ai_1).wrapping_add(616)).wrapping_add(4))); let a3_ = F32(c.m.r32(((p_ai_1).wrapping_add(616)).wrapping_add(8))); F::BtlCharApi_SetInjectedInput(c, a0_, a1_, a2_, a3_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAi_SendInput", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAi_ScaleByLevel(c: &mut Ctx, mut p_level_1: i32, mut p_lo_2: i32, mut p_hi_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAi_ScaleByLevel", &PROBE_N, &[(p_level_1) as u32, (p_lo_2) as u32, (p_hi_3) as u32], false); }
    let r_ = 'f: {
    if (((((p_level_1) < (0i32)) as i32)) != 0) {
        p_level_1 = 0i32;
    }
    break 'f (p_lo_2).wrapping_add(rt::f2i(rt::fadd(rt::fmul(rt::fdiv(rt::i2f((p_hi_3).wrapping_sub(p_lo_2)), F32(0x41e80000)), rt::i2f(p_level_1)), F32(0x3c23d70a))));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAi_ScaleByLevel", &PROBE_N, &[], true); }
    r_
}
