#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlAi_NoteOpponent(c: &mut Ctx, mut p_ai_1: u32, mut p_react_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAi_NoteOpponent", &PROBE_N, &[(p_ai_1) as u32, (p_react_2) as u32], false); }
    let mut l_act_3: u32 = 0;
    let mut l_action_4: i32 = 0;
    let mut l_st_5: u32 = 0;
    let mut l_cls_6: i32 = 0;
    let r_ = 'f: {
    l_act_3 = { let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) };
    l_action_4 = { let a0_ = (((c.m.r32(p_ai_1) as i32)) ^ (1i32)); F::BtlCharApi_GetUnk974(c, a0_) };
    l_st_5 = (p_ai_1).wrapping_add(704);
    l_cls_6 = (((c.m.r8(((l_act_3).wrapping_add(904)).wrapping_add(((l_action_4) as u32))) as i8)) as i32);
    { let v_ = l_action_4; let a_ = (l_st_5).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_cls_6; let a_ = (l_st_5).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let r_ = p_react_2; let a_ = (l_st_5).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAi_NoteOpponent", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCond_False(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_False", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_False", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCond_True(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_True", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_True", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCond_Percent(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_Percent", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let mut l_seq_3: u32 = 0;
    let mut l_roll_4: i32 = 0;
    let r_ = 'f: {
    l_seq_3 = (p_ai_1).wrapping_add(40);
    l_roll_4 = ((F::Rand_Range(c, ((100i32) as u32))) as i32);
    { let v_ = l_roll_4; let a_ = (l_seq_3).wrapping_add(132); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((p_arg_2) as i32); let a_ = (l_seq_3).wrapping_add(136); c.m.w32(a_, (v_) as u32); };
    break 'f (((l_roll_4) < (((p_arg_2) as i32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_Percent", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCond_Idle(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_Idle", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    break 'f ((((c.m.r32(((p_ai_1).wrapping_add(40)).wrapping_add(4)) as i32)) == (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_Idle", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCond_LevelAtLeast(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_LevelAtLeast", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    if ((((((c.m.r32((p_ai_1).wrapping_add(8)) as i32)) < (((p_arg_2) as i32))) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_LevelAtLeast", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCond_LevelBelow(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_LevelBelow", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    break 'f ((((c.m.r32((p_ai_1).wrapping_add(8)) as i32)) < (((p_arg_2) as i32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_LevelBelow", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCond_Status4(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_Status4", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    break 'f ((((((((c.m.r64((p_ai_1).wrapping_add(704))).wrapping_shr((4i32) as u32)) as i32)) & (1i32))) == (((p_arg_2) as i32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_Status4", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCond_GaugeAPercent(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_GaugeAPercent", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    if (((rt::fge(rt::fmul(rt::fdiv(rt::i2f({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlSide_GetKi(c, a0_) }), F32(0x47c35000)), F32(0x42c80000)), rt::i2f((p_arg_2) as i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_GaugeAPercent", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCond_GaugeBStock(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_GaugeBStock", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    if (((((rt::div_i32({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlSide_GetBlast(c, a0_) }, 100000i32)) < (((p_arg_2) as i32))) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_GaugeBStock", &PROBE_N, &[], true); }
    r_
}

pub fn btl_ai_cond__BtlAiCond_TestFlag6(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "btl_ai_cond__BtlAiCond_TestFlag6", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    break 'f ((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlSide_IsPoweredUp(c, a0_) }) == (((p_arg_2) as i32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "btl_ai_cond__BtlAiCond_TestFlag6", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCond_Flag6(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_Flag6", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    break 'f F::btl_ai_cond__BtlAiCond_TestFlag6(c, p_ai_1, p_arg_2);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_Flag6", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCond_Status6(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_Status6", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    break 'f ((((((((c.m.r64((p_ai_1).wrapping_add(704))).wrapping_shr((6i32) as u32)) as i32)) & (1i32))) == (((p_arg_2) as i32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_Status6", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCond_Unk11(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_Unk11", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let mut l_r_3: i32 = 0;
    let r_ = 'f: {
    if (((((F::BtlChar_IsStage4Or27(c)) == (0i32)) as i32)) != 0) {
        l_r_3 = 0i32;
    } else {
        l_r_3 = ((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_TestPoseBit80(c, a0_, 1i32) }) == (0i32)) as i32);
    }
    break 'f (((l_r_3) == (((p_arg_2) as i32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_Unk11", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCond_Unk12(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_Unk12", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    break 'f ((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_TestOppSkillFlags(c, a0_) }) == (((p_arg_2) as i32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_Unk12", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCond_Plan8Is2(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_Plan8Is2", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    break 'f (((((((c.m.r32(((p_ai_1).wrapping_add(744)).wrapping_add(8)) as i32)) == (2i32)) as i32)) == (((p_arg_2) as i32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_Plan8Is2", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCond_WorkBit0(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_WorkBit0", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    break 'f ((((((c.m.r32((p_ai_1).wrapping_add(12)) as i32)) & (1i32))) == (((p_arg_2) as i32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_WorkBit0", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCond_WorkBit1(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_WorkBit1", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    break 'f (((((((c.m.r32((p_ai_1).wrapping_add(12)) as i32)).wrapping_shr((1i32) as u32)) & (1i32))) == (((p_arg_2) as i32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_WorkBit1", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCond_WorkBit2(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_WorkBit2", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    break 'f (((((((c.m.r32((p_ai_1).wrapping_add(12)) as i32)).wrapping_shr((2i32) as u32)) & (1i32))) == (((p_arg_2) as i32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_WorkBit2", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCond_SetPlan54(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_SetPlan54", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((p_arg_2) as i32); let a_ = ((p_ai_1).wrapping_add(744)).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_SetPlan54", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCond_SeqFlag2(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_SeqFlag2", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let mut l_mgr_3: u32 = 0;
    let mut l_seq_4: u32 = 0;
    let mut l_act_5: u32 = 0;
    let r_ = 'f: {
    l_mgr_3 = c.m.r32(sym::gBtlAi);
    l_seq_4 = (p_ai_1).wrapping_add(40);
    l_act_5 = { let a_ = (c.m.r32(l_mgr_3)).wrapping_add(164); c.m.r32(a_) };
    if ((((((c.m.r32((l_seq_4).wrapping_add(4)) as i32)) == (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f (((((((({ let a_ = ((l_act_5).wrapping_add(648)).wrapping_add(({ let a_ = ((l_seq_4).wrapping_add(((((c.m.r32((l_seq_4).wrapping_add(4)) as i32)).wrapping_shl((3i32) as u32)) as u32))).wrapping_add(((12i32) as u32)); c.m.r32(a_) }).wrapping_mul(2)); c.m.r16(a_) }) as i32)).wrapping_shr((2i32) as u32)) & (1i32))) == (((p_arg_2) as i32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_SeqFlag2", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCond_LevelBelowDummy(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_LevelBelowDummy", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    break 'f (((((((c.m.r32((p_ai_1).wrapping_add(8)) as i32)) < ((1i32).wrapping_neg())) as i32)) == (((p_arg_2) as i32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_LevelBelowDummy", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCond_GuardRoll(c: &mut Ctx, mut p_ai_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_GuardRoll", &PROBE_N, &[(p_ai_1) as u32], false); }
    let mut l_st_2: u32 = 0;
    let mut l_cls_3: i8 = 0;
    let mut l_roll_4: i32 = 0;
    let r_ = 'f: {
    l_st_2 = (p_ai_1).wrapping_add(704);
    l_cls_3 = { let a_ = (({ let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) }).wrapping_add(904)).wrapping_add((({ let a0_ = (((c.m.r32(p_ai_1) as i32)) ^ (1i32)); F::BtlCharApi_GetUnk974(c, a0_) }) as u32)); (c.m.r8(a_) as i8) };
    l_roll_4 = ((F::Rand_Range(c, ((100i32) as u32))) as i32);
    if ((((((((((((!((((((((((l_cls_3) as i32)).wrapping_sub(1i32)) as u32)) < (((7i32) as u32))) as i32)) != 0)) as i32)) != 0) && (((((((l_cls_3) as i32)) != (22i32)) as i32)) != 0)) as i32)) != 0) && (((((((l_cls_3) as i32)) != (23i32)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((c.m.r64(l_st_2)) & (((16777216i32) as u64)))) != 0) {
        break 'f 0i32;
    }
    if (((((l_roll_4) < (25i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let r_ = (!(65536i32)); let a_ = (l_st_2).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_GuardRoll", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCond_React(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_React", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let mut l_step_3: i32 = 0;
    let mut l_roll_4: i32 = 0;
    let mut l_action_5: i32 = 0;
    let mut l_chance_6: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 20);
        { let v_ = 20i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
        { let v_ = 40i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
        { let v_ = 60i32; c.m.w32(fp.wrapping_add(8), (v_) as u32); }
        { let v_ = 80i32; c.m.w32(fp.wrapping_add(12), (v_) as u32); }
        { let v_ = 95i32; c.m.w32(fp.wrapping_add(16), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(20), 0, 20);
        { let v_ = 10i32; c.m.w32(fp.wrapping_add(20).wrapping_add(0), (v_) as u32); }
        { let v_ = 20i32; c.m.w32(fp.wrapping_add(20).wrapping_add(4), (v_) as u32); }
        { let v_ = 30i32; c.m.w32(fp.wrapping_add(20).wrapping_add(8), (v_) as u32); }
        { let v_ = 40i32; c.m.w32(fp.wrapping_add(20).wrapping_add(12), (v_) as u32); }
        { let v_ = 50i32; c.m.w32(fp.wrapping_add(20).wrapping_add(16), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(40), 0, 20);
        { let v_ = 0i32; c.m.w32(fp.wrapping_add(40).wrapping_add(0), (v_) as u32); }
        { let v_ = 4i32; c.m.w32(fp.wrapping_add(40).wrapping_add(4), (v_) as u32); }
        { let v_ = 8i32; c.m.w32(fp.wrapping_add(40).wrapping_add(8), (v_) as u32); }
        { let v_ = 12i32; c.m.w32(fp.wrapping_add(40).wrapping_add(12), (v_) as u32); }
        { let v_ = 15i32; c.m.w32(fp.wrapping_add(40).wrapping_add(16), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(60), 0, 20);
        { let v_ = 0i32; c.m.w32(fp.wrapping_add(60).wrapping_add(0), (v_) as u32); }
        { let v_ = 3i32; c.m.w32(fp.wrapping_add(60).wrapping_add(4), (v_) as u32); }
        { let v_ = 5i32; c.m.w32(fp.wrapping_add(60).wrapping_add(8), (v_) as u32); }
        { let v_ = 8i32; c.m.w32(fp.wrapping_add(60).wrapping_add(12), (v_) as u32); }
        { let v_ = 10i32; c.m.w32(fp.wrapping_add(60).wrapping_add(16), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(80), 0, 20);
        { let v_ = 0i32; c.m.w32(fp.wrapping_add(80).wrapping_add(0), (v_) as u32); }
        { let v_ = 2i32; c.m.w32(fp.wrapping_add(80).wrapping_add(4), (v_) as u32); }
        { let v_ = 3i32; c.m.w32(fp.wrapping_add(80).wrapping_add(8), (v_) as u32); }
        { let v_ = 4i32; c.m.w32(fp.wrapping_add(80).wrapping_add(12), (v_) as u32); }
        { let v_ = 5i32; c.m.w32(fp.wrapping_add(80).wrapping_add(16), (v_) as u32); }
        l_step_3 = (if ((((((c.m.r32((p_ai_1).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) { 0i32 } else { rt::div_i32((c.m.r32((p_ai_1).wrapping_add(8)) as i32), 6i32) });
        l_roll_4 = ((F::Rand_Range(c, ((100i32) as u32))) as i32);
        l_action_5 = { let a0_ = (((c.m.r32(p_ai_1) as i32)) ^ (1i32)); F::BtlCharApi_GetUnk974(c, a0_) };
        's1: {
            let sv7: i32 = ((p_arg_2) as i32);
            'k6: {
            'k5: {
            'k4: {
            'k3: {
            'k2: {
                match sv7 { 0i32 => break 'k2, 1i32 => break 'k3, 2i32 => break 'k4, 3i32 => break 'k5, _ => break 'k6, }
            }
            F::BtlAi_NoteOpponent(c, p_ai_1, 65536i32);
            if (((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetPromptButtons(c, a0_) }) & (4i32))) != 0) {
                break 'f F::BtlAiCond_GuardRoll(c, p_ai_1);
            }
            }
            l_chance_6 = (if ((((((c.m.r32((p_ai_1).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) { 0i32 } else { { let a_ = (fp).wrapping_add((((rt::div_i32((c.m.r32((p_ai_1).wrapping_add(8)) as i32), 6i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) } });
            break 's1;
            }
            F::BtlAi_NoteOpponent(c, p_ai_1, 524288i32);
            l_chance_6 = (if ((((((c.m.r32((p_ai_1).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) { 0i32 } else { { let a_ = (fp.wrapping_add(20)).wrapping_add((((rt::div_i32((c.m.r32((p_ai_1).wrapping_add(8)) as i32), 6i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) } });
            break 's1;
            }
            F::BtlAi_NoteOpponent(c, p_ai_1, 1048576i32);
            if (((((((((l_action_5) == (55i32)) as i32)) != 0) || (((((l_action_5) == (60i32)) as i32)) != 0)) as i32)) != 0) {
                l_chance_6 = (if ((((((c.m.r32((p_ai_1).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) { 0i32 } else { { let a_ = (fp.wrapping_add(40)).wrapping_add((((rt::div_i32((c.m.r32((p_ai_1).wrapping_add(8)) as i32), 6i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) } });
            } else {
                if (((((((((l_action_5) == (56i32)) as i32)) != 0) || (((((l_action_5) == (61i32)) as i32)) != 0)) as i32)) != 0) {
                    l_chance_6 = (if ((((((c.m.r32((p_ai_1).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) { 0i32 } else { { let a_ = (fp.wrapping_add(60)).wrapping_add((((rt::div_i32((c.m.r32((p_ai_1).wrapping_add(8)) as i32), 6i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) } });
                } else {
                    if (((((((((l_action_5) == (57i32)) as i32)) != 0) || (((((l_action_5) == (62i32)) as i32)) != 0)) as i32)) != 0) {
                        l_chance_6 = (if ((((((c.m.r32((p_ai_1).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) { 0i32 } else { { let a_ = (fp.wrapping_add(80)).wrapping_add((((rt::div_i32((c.m.r32((p_ai_1).wrapping_add(8)) as i32), 6i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) } });
                    } else {
                        F::BtlAi_NoteOpponent(c, p_ai_1, 16i32);
                        break 'f 0i32;
                    }
                }
            }
            break 's1;
            }
            break 'f 0i32;
        }
        break 'f (((l_roll_4) < (l_chance_6)) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_React", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCond_SeqReady(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_SeqReady", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    break 'f ((((((F::BtlSeq_GetState(c)) == (2i32)) as i32)) == (((p_arg_2) as i32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_SeqReady", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCond_Stage4Or27(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_Stage4Or27", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    break 'f (((F::BtlChar_IsStage4Or27(c)) == (((p_arg_2) as i32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_Stage4Or27", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCond_React40000(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_React40000", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_chance_3: i32 = 0;
    let mut l_roll_4: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 20);
        { let v_ = 40i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
        { let v_ = 55i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
        { let v_ = 70i32; c.m.w32(fp.wrapping_add(8), (v_) as u32); }
        { let v_ = 85i32; c.m.w32(fp.wrapping_add(12), (v_) as u32); }
        { let v_ = 95i32; c.m.w32(fp.wrapping_add(16), (v_) as u32); }
        l_chance_3 = (if ((((((c.m.r32((p_ai_1).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) { 0i32 } else { { let a_ = (fp).wrapping_add((((rt::div_i32((c.m.r32((p_ai_1).wrapping_add(8)) as i32), 6i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) } });
        l_roll_4 = ((F::Rand_Range(c, ((100i32) as u32))) as i32);
        if (((((c.m.r32(((p_ai_1).wrapping_add(704)).wrapping_add(20)) as i32)) & (262144i32))) != 0) {
            break 'f 0i32;
        }
        F::BtlAi_NoteOpponent(c, p_ai_1, 262144i32);
        break 'f (((l_roll_4) < (l_chance_3)) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_React40000", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCond_Unk24(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_Unk24", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    break 'f ((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_HasBlastLimit(c, a0_) }) == (((p_arg_2) as i32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_Unk24", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCond_Rate0(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_Rate0", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let mut l_lo_3: u32 = 0;
    let mut l_hi_4: u32 = 0;
    let mut l_roll_5: i32 = 0;
    let mut l_chance_6: i32 = 0;
    let mut l_seq_7: u32 = 0;
    let r_ = 'f: {
    l_lo_3 = (c.m.r32((p_ai_1).wrapping_add(24))).wrapping_add(((8i32) as u32));
    l_hi_4 = (c.m.r32((p_ai_1).wrapping_add(24))).wrapping_add(((256i32) as u32));
    l_roll_5 = ((F::Rand_Range(c, ((100i32) as u32))) as i32);
    l_chance_6 = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((l_lo_3).wrapping_add(((0i32) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((l_hi_4).wrapping_add(((0i32) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) };
    l_seq_7 = (p_ai_1).wrapping_add(40);
    { let v_ = l_roll_5; let a_ = (l_seq_7).wrapping_add(132); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_chance_6; let a_ = (l_seq_7).wrapping_add(136); c.m.w32(a_, (v_) as u32); };
    if (((((c.m.r32(((p_ai_1).wrapping_add(704)).wrapping_add(20)) as i32)) & (2i32))) != 0) {
        F::BtlAi_NoteOpponent(c, p_ai_1, 32i32);
    }
    break 'f (((l_roll_5) < (l_chance_6)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_Rate0", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCond_Rate5(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_Rate5", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let mut l_lo_3: u32 = 0;
    let mut l_hi_4: u32 = 0;
    let mut l_roll_5: i32 = 0;
    let mut l_chance_6: i32 = 0;
    let mut l_a_7: i32 = 0;
    let mut l_b_8: i32 = 0;
    let r_ = 'f: {
    l_lo_3 = (c.m.r32((p_ai_1).wrapping_add(24))).wrapping_add(((8i32) as u32));
    l_hi_4 = (c.m.r32((p_ai_1).wrapping_add(24))).wrapping_add(((256i32) as u32));
    l_roll_5 = ((F::Rand_Range(c, ((100i32) as u32))) as i32);
    l_chance_6 = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((l_lo_3).wrapping_add(((5i32) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((l_hi_4).wrapping_add(((5i32) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) };
    l_a_7 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetParamUnk14(c, a0_) };
    l_b_8 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetParamFlags(c, a0_) };
    if ((((((((!((((l_a_7) & (32i32))) != 0)) as i32)) != 0) && ((((!((((l_b_8) & (2i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f (((l_roll_5) < (l_chance_6)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_Rate5", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCond_Unk27(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_Unk27", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    if ((((((((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_FindIncomingBlast(c, a0_, 1i32) }) == (2i32)) as i32)) != 0) && (((((((p_arg_2) as i32)) == (1i32)) as i32)) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_Unk27", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCond_Rate6(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_Rate6", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let mut l_lo_3: u32 = 0;
    let mut l_hi_4: u32 = 0;
    let mut l_roll_5: i32 = 0;
    let mut l_chance_6: i32 = 0;
    let r_ = 'f: {
    l_lo_3 = (c.m.r32((p_ai_1).wrapping_add(24))).wrapping_add(((8i32) as u32));
    l_hi_4 = (c.m.r32((p_ai_1).wrapping_add(24))).wrapping_add(((256i32) as u32));
    l_roll_5 = ((F::Rand_Range(c, ((100i32) as u32))) as i32);
    l_chance_6 = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((l_lo_3).wrapping_add(((6i32) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((l_hi_4).wrapping_add(((6i32) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) };
    F::BtlAi_NoteOpponent(c, p_ai_1, 128i32);
    break 'f (((l_roll_5) < (l_chance_6)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_Rate6", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCond_Rate1(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_Rate1", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let mut l_seq_3: u32 = 0;
    let mut l_lo_4: u32 = 0;
    let mut l_hi_5: u32 = 0;
    let mut l_roll_6: i32 = 0;
    let mut l_chance_7: i32 = 0;
    let r_ = 'f: {
    l_seq_3 = (p_ai_1).wrapping_add(40);
    l_lo_4 = (c.m.r32((p_ai_1).wrapping_add(24))).wrapping_add(((8i32) as u32));
    l_hi_5 = (c.m.r32((p_ai_1).wrapping_add(24))).wrapping_add(((256i32) as u32));
    l_roll_6 = ((F::Rand_Range(c, ((100i32) as u32))) as i32);
    l_chance_7 = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((l_lo_4).wrapping_add(((1i32) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((l_hi_5).wrapping_add(((1i32) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) };
    if ((((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetOppSkillKind(c, a0_) }) == ((1i32).wrapping_neg())) as i32)) != 0) {
        F::BtlAi_NoteOpponent(c, p_ai_1, 64i32);
    }
    { let v_ = l_roll_6; let a_ = (l_seq_3).wrapping_add(132); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_chance_7; let a_ = (l_seq_3).wrapping_add(136); c.m.w32(a_, (v_) as u32); };
    break 'f (((l_roll_6) < (l_chance_7)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_Rate1", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCond_Rate4(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_Rate4", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let mut l_lo_3: u32 = 0;
    let mut l_hi_4: u32 = 0;
    let mut l_roll_5: i32 = 0;
    let mut l_chance_6: i32 = 0;
    let r_ = 'f: {
    l_lo_3 = (c.m.r32((p_ai_1).wrapping_add(24))).wrapping_add(((8i32) as u32));
    l_hi_4 = (c.m.r32((p_ai_1).wrapping_add(24))).wrapping_add(((256i32) as u32));
    l_roll_5 = ((F::Rand_Range(c, ((100i32) as u32))) as i32);
    l_chance_6 = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((l_lo_3).wrapping_add(((4i32) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((l_hi_4).wrapping_add(((4i32) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) };
    F::BtlAi_NoteOpponent(c, p_ai_1, 256i32);
    break 'f (((l_roll_5) < (l_chance_6)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_Rate4", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCond_TypeRateByOppAction(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_TypeRateByOppAction", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let mut l_seq_3: u32 = 0;
    let mut l_action_4: i32 = 0;
    let mut l_prof_5: u32 = 0;
    let mut l_lo_6: u32 = 0;
    let mut l_hi_7: u32 = 0;
    let mut l_idx_8: i32 = 0;
    let mut l_roll_9: i32 = 0;
    let mut l_chance_10: i32 = 0;
    let r_ = 'f: {
    l_seq_3 = (p_ai_1).wrapping_add(40);
    l_action_4 = { let a0_ = (((c.m.r32(p_ai_1) as i32)) ^ (1i32)); F::BtlCharApi_GetUnk974(c, a0_) };
    l_prof_5 = { let a_ = (({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(200)).wrapping_add(((((c.m.r32((p_ai_1).wrapping_add(4)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    l_lo_6 = (l_prof_5).wrapping_add(((680i32) as u32));
    l_hi_7 = (l_prof_5).wrapping_add(((1384i32) as u32));
    l_idx_8 = (if (((((l_action_4) < (60i32)) as i32)) != 0) { (l_action_4).wrapping_sub(55i32) } else { (l_action_4).wrapping_sub(60i32) });
    l_roll_9 = ((F::Rand_Range(c, ((100i32) as u32))) as i32);
    if (((((((l_idx_8) as u32)) >= (((4i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_chance_10 = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((l_lo_6).wrapping_add(((l_idx_8) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((l_hi_7).wrapping_add(((l_idx_8) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) };
    { let v_ = l_roll_9; let a_ = (l_seq_3).wrapping_add(132); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_chance_10; let a_ = (l_seq_3).wrapping_add(136); c.m.w32(a_, (v_) as u32); };
    break 'f (((l_roll_9) < (l_chance_10)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_TypeRateByOppAction", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCond_Rate2(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_Rate2", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let mut l_lo_3: u32 = 0;
    let mut l_hi_4: u32 = 0;
    let mut l_roll_5: i32 = 0;
    let mut l_chance_6: i32 = 0;
    let mut l_seq_7: u32 = 0;
    let mut l_st_8: u32 = 0;
    let r_ = 'f: {
    l_lo_3 = (c.m.r32((p_ai_1).wrapping_add(24))).wrapping_add(((8i32) as u32));
    l_hi_4 = (c.m.r32((p_ai_1).wrapping_add(24))).wrapping_add(((256i32) as u32));
    l_roll_5 = ((F::Rand_Range(c, ((100i32) as u32))) as i32);
    l_chance_6 = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((l_lo_3).wrapping_add(((2i32) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((l_hi_4).wrapping_add(((2i32) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) };
    l_seq_7 = (p_ai_1).wrapping_add(40);
    { let v_ = l_roll_5; let a_ = (l_seq_7).wrapping_add(132); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_chance_6; let a_ = (l_seq_7).wrapping_add(136); c.m.w32(a_, (v_) as u32); };
    l_st_8 = (p_ai_1).wrapping_add(704);
    { let r_ = 1024i32; let a_ = (l_st_8).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    break 'f (((l_roll_5) < (l_chance_6)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_Rate2", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCond_Rate8(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_Rate8", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let mut l_st_3: u32 = 0;
    let mut l_seq_4: u32 = 0;
    let mut l_lo_5: u32 = 0;
    let mut l_hi_6: u32 = 0;
    let mut l_roll_7: i32 = 0;
    let mut l_chance_8: i32 = 0;
    let mut l_force_9: i32 = 0;
    let r_ = 'f: {
    l_st_3 = (p_ai_1).wrapping_add(704);
    l_seq_4 = (p_ai_1).wrapping_add(40);
    l_lo_5 = (c.m.r32((p_ai_1).wrapping_add(24))).wrapping_add(((8i32) as u32));
    l_hi_6 = (c.m.r32((p_ai_1).wrapping_add(24))).wrapping_add(((256i32) as u32));
    l_roll_7 = ((F::Rand_Range(c, ((100i32) as u32))) as i32);
    l_chance_8 = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((l_lo_5).wrapping_add(((8i32) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((l_hi_6).wrapping_add(((8i32) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) };
    l_force_9 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetUnk1290(c, a0_) };
    if (((((((p_arg_2) as i32)) == (0i32)) as i32)) != 0) {
        { let v_ = 90i32; let a_ = (l_st_3).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
    } else {
        if (((((((p_arg_2) as i32)) == (1i32)) as i32)) != 0) {
            { let v_ = 90i32; let a_ = (l_st_3).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
            { let r_ = 20i32; let n_ = (l_chance_8).wrapping_add(r_); l_chance_8 = n_; };
        }
    }
    { let v_ = l_roll_7; let a_ = (l_seq_4).wrapping_add(132); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_chance_8; let a_ = (l_seq_4).wrapping_add(136); c.m.w32(a_, (v_) as u32); };
    if (((((((((l_roll_7) < (l_chance_8)) as i32)) != 0) || ((l_force_9) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_Rate8", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiCond_TenPercent(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_TenPercent", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let mut l_roll_3: i32 = 0;
    let r_ = 'f: {
    l_roll_3 = ((F::Rand_Range(c, ((100i32) as u32))) as i32);
    { let v_ = 90i32; let a_ = ((p_ai_1).wrapping_add(704)).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
    if ((F::btl_ai_cond__BtlAiCond_TestFlag6(c, p_ai_1, ((0i32) as u8))) != 0) {
        break 'f (((l_roll_3) < (10i32)) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiCond_TenPercent", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAi_ScaleByGauge(c: &mut Ctx, mut p_ai_1: u32, mut p_lo_2: u32, mut p_hi_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAi_ScaleByGauge", &PROBE_N, &[(p_ai_1) as u32, (p_lo_2) as u32, (p_hi_3) as u32], false); }
    let mut l_prof_4: u32 = 0;
    let mut l_limit_5: F32 = F32(0);
    let mut l_pct_6: F32 = F32(0);
    let mut l_a_7: i32 = 0;
    let mut l_b_8: i32 = 0;
    let mut l_k_9: F32 = F32(0);
    let r_ = 'f: {
    l_prof_4 = { let a_ = (({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(200)).wrapping_add(((((c.m.r32((p_ai_1).wrapping_add(4)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    l_limit_5 = rt::i2f({ let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = ((c.m.r8((l_prof_4).wrapping_add(((684i32) as u32)))) as i32); let a2_ = ((c.m.r8((l_prof_4).wrapping_add(((1388i32) as u32)))) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) });
    l_pct_6 = rt::fmul(rt::fdiv(rt::i2f({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetMaxPower(c, a0_) }), F32(0x46ea6000)), F32(0x42c80000));
    l_a_7 = (c.m.r32(p_lo_2) as i32);
    l_b_8 = (c.m.r32(p_hi_3) as i32);
    if (((rt::flt(l_limit_5, l_pct_6) as i32)) != 0) {
        l_k_9 = rt::fsub(F32(0x3f800000), rt::fdiv(rt::fsub(l_pct_6, l_limit_5), rt::fsub(F32(0x42c80000), l_limit_5)));
        { let v_ = rt::f2i(rt::fmul(rt::i2f(l_a_7), l_k_9)); let a_ = p_lo_2; c.m.w32(a_, (v_) as u32); };
        { let v_ = rt::f2i(rt::fmul(rt::i2f(l_b_8), l_k_9)); let a_ = p_hi_3; c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAi_ScaleByGauge", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAi_GetLowGaugeFactor(c: &mut Ctx, mut p_ai_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAi_GetLowGaugeFactor", &PROBE_N, &[(p_ai_1) as u32], false); }
    let mut l_pct_2: F32 = F32(0);
    let mut l_prof_3: u32 = 0;
    let mut l_r_4: F32 = F32(0);
    let mut l_lo_5: i32 = 0;
    let mut l_hi_6: i32 = 0;
    let r_ = 'f: {
    l_pct_2 = rt::fmul(rt::fdiv(rt::i2f({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlSide_GetKi(c, a0_) }), F32(0x47c35000)), F32(0x42c80000));
    l_prof_3 = { let a_ = (({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(200)).wrapping_add(((((c.m.r32((p_ai_1).wrapping_add(4)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    l_r_4 = F32(0x3f800000);
    if (((rt::flt(l_pct_2, F32(0x41a00000)) as i32)) != 0) {
        l_lo_5 = ((c.m.r8((l_prof_3).wrapping_add(((695i32) as u32)))) as i32);
        l_hi_6 = ((c.m.r8((l_prof_3).wrapping_add(((1399i32) as u32)))) as i32);
    } else {
        if (((rt::flt(l_pct_2, F32(0x42200000)) as i32)) != 0) {
            l_lo_5 = ((c.m.r8((l_prof_3).wrapping_add(((694i32) as u32)))) as i32);
            l_hi_6 = ((c.m.r8((l_prof_3).wrapping_add(((1398i32) as u32)))) as i32);
        } else {
            break 'f l_r_4;
        }
    }
    break 'f rt::fmul(rt::i2f({ let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, l_lo_5, l_hi_6) }), F32(0x3dcccccc));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAi_GetLowGaugeFactor", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAi_GetPairRate(c: &mut Ctx, mut p_ai_1: u32, mut p_kind_2: u32, mut p_off_3: i32, mut p_base_lo_4: u32, mut p_base_hi_5: u32, mut p_byGauge_6: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAi_GetPairRate", &PROBE_N, &[(p_ai_1) as u32, (p_kind_2) as u32, (p_off_3) as u32, (p_base_lo_4) as u32, (p_base_hi_5) as u32, (p_byGauge_6) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_plan_7: u32 = 0;
    let mut l_i_8: i32 = 0;
    let mut l_i_9: i32 = 0;
    let mut l_i_10: i32 = 0;
    let mut l_i_11: i32 = 0;
    let mut l_i_12: i32 = 0;
    let mut l_i_13: i32 = 0;
    let mut l_i_14: i32 = 0;
    let mut l_i_15: i32 = 0;
    let mut l_i_16: i32 = 0;
    let mut l_i_17: i32 = 0;
    let mut l_i_18: i32 = 0;
    let mut l_i_19: i32 = 0;
    let mut l_k_20: F32 = F32(0);
    let r_ = 'f: {
        l_plan_7 = (p_ai_1).wrapping_add(744);
        { let v_ = 0i32; c.m.w32(fp, (v_) as u32); }
        { let v_ = 0i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
        's1: {
            let sv21: u32 = p_kind_2;
            match sv21 {
                0u32 => {
                    { let v_ = (((c.m.r8((p_base_lo_4).wrapping_add(((p_off_3) as u32))) as i8)) as i32); let a_ = fp; c.m.w32(a_, (v_) as u32); };
                    { let v_ = (((c.m.r8((p_base_hi_5).wrapping_add(((p_off_3) as u32))) as i8)) as i32); let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                1u32 => {
                    { let v_ = (((c.m.r8(((p_base_lo_4).wrapping_add(((p_off_3) as u32))).wrapping_add(8)) as i8)) as i32); let a_ = fp; c.m.w32(a_, (v_) as u32); };
                    { let v_ = (((c.m.r8(((p_base_hi_5).wrapping_add(((p_off_3) as u32))).wrapping_add(8)) as i8)) as i32); let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                2u32 => {
                    {
                        l_i_8 = (p_off_3).wrapping_add(16i32);
                        { let v_ = (((c.m.r8((p_base_lo_4).wrapping_add(((l_i_8) as u32))) as i8)) as i32); let a_ = fp; c.m.w32(a_, (v_) as u32); };
                        { let v_ = (((c.m.r8((p_base_hi_5).wrapping_add(((l_i_8) as u32))) as i8)) as i32); let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                }
                3u32 => {
                    {
                        l_i_9 = (p_off_3).wrapping_add(16i32);
                        { let v_ = (((c.m.r8(((p_base_lo_4).wrapping_add(((l_i_9) as u32))).wrapping_add(8)) as i8)) as i32); let a_ = fp; c.m.w32(a_, (v_) as u32); };
                        { let v_ = (((c.m.r8(((p_base_hi_5).wrapping_add(((l_i_9) as u32))).wrapping_add(8)) as i8)) as i32); let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                }
                4u32 => {
                    {
                        l_i_10 = (p_off_3).wrapping_add(32i32);
                        { let v_ = (((c.m.r8((p_base_lo_4).wrapping_add(((l_i_10) as u32))) as i8)) as i32); let a_ = fp; c.m.w32(a_, (v_) as u32); };
                        { let v_ = (((c.m.r8((p_base_hi_5).wrapping_add(((l_i_10) as u32))) as i8)) as i32); let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                }
                5u32 => {
                    {
                        l_i_11 = (p_off_3).wrapping_add(32i32);
                        { let v_ = (((c.m.r8(((p_base_lo_4).wrapping_add(((l_i_11) as u32))).wrapping_add(8)) as i8)) as i32); let a_ = fp; c.m.w32(a_, (v_) as u32); };
                        { let v_ = (((c.m.r8(((p_base_hi_5).wrapping_add(((l_i_11) as u32))).wrapping_add(8)) as i8)) as i32); let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                }
                6u32 => {
                    {
                        l_i_12 = (p_off_3).wrapping_add(48i32);
                        { let v_ = (((c.m.r8((p_base_lo_4).wrapping_add(((l_i_12) as u32))) as i8)) as i32); let a_ = fp; c.m.w32(a_, (v_) as u32); };
                        { let v_ = (((c.m.r8((p_base_hi_5).wrapping_add(((l_i_12) as u32))) as i8)) as i32); let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                }
                7u32 => {
                    {
                        l_i_13 = (p_off_3).wrapping_add(48i32);
                        { let v_ = (((c.m.r8(((p_base_lo_4).wrapping_add(((l_i_13) as u32))).wrapping_add(8)) as i8)) as i32); let a_ = fp; c.m.w32(a_, (v_) as u32); };
                        { let v_ = (((c.m.r8(((p_base_hi_5).wrapping_add(((l_i_13) as u32))).wrapping_add(8)) as i8)) as i32); let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                }
                8u32 => {
                    {
                        l_i_14 = (p_off_3).wrapping_add(64i32);
                        { let v_ = (((c.m.r8((p_base_lo_4).wrapping_add(((l_i_14) as u32))) as i8)) as i32); let a_ = fp; c.m.w32(a_, (v_) as u32); };
                        { let v_ = (((c.m.r8((p_base_hi_5).wrapping_add(((l_i_14) as u32))) as i8)) as i32); let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                }
                9u32 => {
                    {
                        l_i_15 = (p_off_3).wrapping_add(64i32);
                        { let v_ = (((c.m.r8(((p_base_lo_4).wrapping_add(((l_i_15) as u32))).wrapping_add(8)) as i8)) as i32); let a_ = fp; c.m.w32(a_, (v_) as u32); };
                        { let v_ = (((c.m.r8(((p_base_hi_5).wrapping_add(((l_i_15) as u32))).wrapping_add(8)) as i8)) as i32); let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                }
                10u32 => {
                    {
                        l_i_16 = (p_off_3).wrapping_add(80i32);
                        { let v_ = (((c.m.r8((p_base_lo_4).wrapping_add(((l_i_16) as u32))) as i8)) as i32); let a_ = fp; c.m.w32(a_, (v_) as u32); };
                        { let v_ = (((c.m.r8((p_base_hi_5).wrapping_add(((l_i_16) as u32))) as i8)) as i32); let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                }
                11u32 => {
                    {
                        l_i_17 = (p_off_3).wrapping_add(80i32);
                        { let v_ = (((c.m.r8(((p_base_lo_4).wrapping_add(((l_i_17) as u32))).wrapping_add(8)) as i8)) as i32); let a_ = fp; c.m.w32(a_, (v_) as u32); };
                        { let v_ = (((c.m.r8(((p_base_hi_5).wrapping_add(((l_i_17) as u32))).wrapping_add(8)) as i8)) as i32); let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                }
                12u32 => {
                    {
                        l_i_18 = (p_off_3).wrapping_add(96i32);
                        { let v_ = (((c.m.r8((p_base_lo_4).wrapping_add(((l_i_18) as u32))) as i8)) as i32); let a_ = fp; c.m.w32(a_, (v_) as u32); };
                        { let v_ = (((c.m.r8((p_base_hi_5).wrapping_add(((l_i_18) as u32))) as i8)) as i32); let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                }
                13u32 => {
                    {
                        l_i_19 = (p_off_3).wrapping_add(96i32);
                        { let v_ = (((c.m.r8(((p_base_lo_4).wrapping_add(((l_i_19) as u32))).wrapping_add(8)) as i8)) as i32); let a_ = fp; c.m.w32(a_, (v_) as u32); };
                        { let v_ = (((c.m.r8(((p_base_hi_5).wrapping_add(((l_i_19) as u32))).wrapping_add(8)) as i8)) as i32); let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                }
                _ => {
                    break 'f 0i32;
                }
            }
        }
        if (((((p_byGauge_6) == (1i32)) as i32)) != 0) {
            F::BtlAi_ScaleByGauge(c, p_ai_1, fp, fp.wrapping_add(4));
        }
        if ((((({ let a_ = (sym::D_002EDA70).wrapping_add(((((c.m.r32((l_plan_7).wrapping_add(12)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (38i32)) as i32)) != 0) {
            l_k_20 = F::BtlAi_GetLowGaugeFactor(c, p_ai_1);
            { let v_ = rt::f2i(rt::fmul(rt::i2f((c.m.r32(fp) as i32)), l_k_20)); let a_ = fp; c.m.w32(a_, (v_) as u32); };
            { let v_ = rt::f2i(rt::fmul(rt::i2f((c.m.r32(fp.wrapping_add(4)) as i32)), l_k_20)); let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        }
        break 'f { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (c.m.r32(fp) as i32); let a2_ = (c.m.r32(fp.wrapping_add(4)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) };
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAi_GetPairRate", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAi_GetQuadRate(c: &mut Ctx, mut p_ai_1: u32, mut p_kind_2: u32, mut p_off_3: i32, mut p_base_lo_4: u32, mut p_base_hi_5: u32, mut p_byGauge_6: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAi_GetQuadRate", &PROBE_N, &[(p_ai_1) as u32, (p_kind_2) as u32, (p_off_3) as u32, (p_base_lo_4) as u32, (p_base_hi_5) as u32, (p_byGauge_6) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_7: i32 = 0;
    let mut l_i_8: i32 = 0;
    let mut l_i_9: i32 = 0;
    let mut l_i_10: i32 = 0;
    let mut l_i_11: i32 = 0;
    let mut l_i_12: i32 = 0;
    let mut l_i_13: i32 = 0;
    let mut l_i_14: i32 = 0;
    let mut l_i_15: i32 = 0;
    let mut l_i_16: i32 = 0;
    let r_ = 'f: {
        { let v_ = 0i32; c.m.w32(fp, (v_) as u32); }
        { let v_ = 0i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
        's1: {
            let sv17: u32 = p_kind_2;
            match sv17 {
                0u32 => {
                    { let v_ = (((c.m.r8((p_base_lo_4).wrapping_add(((p_off_3) as u32))) as i8)) as i32); let a_ = fp; c.m.w32(a_, (v_) as u32); };
                    { let v_ = (((c.m.r8((p_base_hi_5).wrapping_add(((p_off_3) as u32))) as i8)) as i32); let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                1u32 => {
                    { let v_ = (((c.m.r8(((p_base_lo_4).wrapping_add(((p_off_3) as u32))).wrapping_add(4)) as i8)) as i32); let a_ = fp; c.m.w32(a_, (v_) as u32); };
                    { let v_ = (((c.m.r8(((p_base_hi_5).wrapping_add(((p_off_3) as u32))).wrapping_add(4)) as i8)) as i32); let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                2u32 => {
                    { let v_ = (((c.m.r8(((p_base_lo_4).wrapping_add(((p_off_3) as u32))).wrapping_add(8)) as i8)) as i32); let a_ = fp; c.m.w32(a_, (v_) as u32); };
                    { let v_ = (((c.m.r8(((p_base_hi_5).wrapping_add(((p_off_3) as u32))).wrapping_add(8)) as i8)) as i32); let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                3u32 => {
                    { let v_ = (((c.m.r8(((p_base_lo_4).wrapping_add(((p_off_3) as u32))).wrapping_add(12)) as i8)) as i32); let a_ = fp; c.m.w32(a_, (v_) as u32); };
                    { let v_ = (((c.m.r8(((p_base_hi_5).wrapping_add(((p_off_3) as u32))).wrapping_add(12)) as i8)) as i32); let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                4u32 => {
                    {
                        l_i_7 = (p_off_3).wrapping_add(16i32);
                        { let v_ = (((c.m.r8((p_base_lo_4).wrapping_add(((l_i_7) as u32))) as i8)) as i32); let a_ = fp; c.m.w32(a_, (v_) as u32); };
                        { let v_ = (((c.m.r8((p_base_hi_5).wrapping_add(((l_i_7) as u32))) as i8)) as i32); let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                }
                5u32 => {
                    {
                        l_i_8 = (p_off_3).wrapping_add(16i32);
                        { let v_ = (((c.m.r8(((p_base_lo_4).wrapping_add(((l_i_8) as u32))).wrapping_add(4)) as i8)) as i32); let a_ = fp; c.m.w32(a_, (v_) as u32); };
                        { let v_ = (((c.m.r8(((p_base_hi_5).wrapping_add(((l_i_8) as u32))).wrapping_add(4)) as i8)) as i32); let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                }
                6u32 => {
                    {
                        l_i_9 = (p_off_3).wrapping_add(16i32);
                        { let v_ = (((c.m.r8(((p_base_lo_4).wrapping_add(((l_i_9) as u32))).wrapping_add(8)) as i8)) as i32); let a_ = fp; c.m.w32(a_, (v_) as u32); };
                        { let v_ = (((c.m.r8(((p_base_hi_5).wrapping_add(((l_i_9) as u32))).wrapping_add(8)) as i8)) as i32); let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                }
                7u32 => {
                    {
                        l_i_10 = (p_off_3).wrapping_add(16i32);
                        { let v_ = (((c.m.r8(((p_base_lo_4).wrapping_add(((l_i_10) as u32))).wrapping_add(12)) as i8)) as i32); let a_ = fp; c.m.w32(a_, (v_) as u32); };
                        { let v_ = (((c.m.r8(((p_base_hi_5).wrapping_add(((l_i_10) as u32))).wrapping_add(12)) as i8)) as i32); let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                }
                8u32 => {
                    {
                        l_i_11 = (p_off_3).wrapping_add(32i32);
                        { let v_ = (((c.m.r8((p_base_lo_4).wrapping_add(((l_i_11) as u32))) as i8)) as i32); let a_ = fp; c.m.w32(a_, (v_) as u32); };
                        { let v_ = (((c.m.r8((p_base_hi_5).wrapping_add(((l_i_11) as u32))) as i8)) as i32); let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                }
                9u32 => {
                    {
                        l_i_12 = (p_off_3).wrapping_add(32i32);
                        { let v_ = (((c.m.r8(((p_base_lo_4).wrapping_add(((l_i_12) as u32))).wrapping_add(4)) as i8)) as i32); let a_ = fp; c.m.w32(a_, (v_) as u32); };
                        { let v_ = (((c.m.r8(((p_base_hi_5).wrapping_add(((l_i_12) as u32))).wrapping_add(4)) as i8)) as i32); let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                }
                10u32 => {
                    {
                        l_i_13 = (p_off_3).wrapping_add(32i32);
                        { let v_ = (((c.m.r8(((p_base_lo_4).wrapping_add(((l_i_13) as u32))).wrapping_add(8)) as i8)) as i32); let a_ = fp; c.m.w32(a_, (v_) as u32); };
                        { let v_ = (((c.m.r8(((p_base_hi_5).wrapping_add(((l_i_13) as u32))).wrapping_add(8)) as i8)) as i32); let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                }
                11u32 => {
                    {
                        l_i_14 = (p_off_3).wrapping_add(32i32);
                        { let v_ = (((c.m.r8(((p_base_lo_4).wrapping_add(((l_i_14) as u32))).wrapping_add(12)) as i8)) as i32); let a_ = fp; c.m.w32(a_, (v_) as u32); };
                        { let v_ = (((c.m.r8(((p_base_hi_5).wrapping_add(((l_i_14) as u32))).wrapping_add(12)) as i8)) as i32); let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                }
                12u32 => {
                    {
                        l_i_15 = (p_off_3).wrapping_add(48i32);
                        { let v_ = (((c.m.r8((p_base_lo_4).wrapping_add(((l_i_15) as u32))) as i8)) as i32); let a_ = fp; c.m.w32(a_, (v_) as u32); };
                        { let v_ = (((c.m.r8((p_base_hi_5).wrapping_add(((l_i_15) as u32))) as i8)) as i32); let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                }
                13u32 => {
                    {
                        l_i_16 = (p_off_3).wrapping_add(48i32);
                        { let v_ = (((c.m.r8(((p_base_lo_4).wrapping_add(((l_i_16) as u32))).wrapping_add(4)) as i8)) as i32); let a_ = fp; c.m.w32(a_, (v_) as u32); };
                        { let v_ = (((c.m.r8(((p_base_hi_5).wrapping_add(((l_i_16) as u32))).wrapping_add(4)) as i8)) as i32); let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                }
                _ => {
                    break 'f 0i32;
                }
            }
        }
        if (((((p_byGauge_6) == (1i32)) as i32)) != 0) {
            F::BtlAi_ScaleByGauge(c, p_ai_1, fp, fp.wrapping_add(4));
        }
        break 'f { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (c.m.r32(fp) as i32); let a2_ = (c.m.r32(fp.wrapping_add(4)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) };
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAi_GetQuadRate", &PROBE_N, &[], true); }
    r_
}

pub fn AiThink_GetSubRate(c: &mut Ctx, mut p_ai_1: u32, mut p_col_2: i32, mut p_base_lo_3: u32, mut p_base_hi_4: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThink_GetSubRate", &PROBE_N, &[(p_ai_1) as u32, (p_col_2) as u32, (p_base_lo_3) as u32, (p_base_hi_4) as u32], false); }
    let mut l_lo_5: i32 = 0;
    let mut l_hi_6: i32 = 0;
    let mut l_i_7: i32 = 0;
    let r_ = 'f: {
    's1: {
        let sv8: i32 = (c.m.r32(((p_ai_1).wrapping_add(744)).wrapping_add(84)) as i32);
        match sv8 {
            0i32 => {
                l_lo_5 = (((c.m.r8((p_base_lo_3).wrapping_add(((p_col_2) as u32))) as i8)) as i32);
                l_hi_6 = (((c.m.r8((p_base_hi_4).wrapping_add(((p_col_2) as u32))) as i8)) as i32);
                break 's1;
            }
            1i32 => {
                l_lo_5 = (((c.m.r8((p_base_lo_3).wrapping_add((((p_col_2).wrapping_add(8i32)) as u32))) as i8)) as i32);
                l_hi_6 = (((c.m.r8((p_base_hi_4).wrapping_add((((p_col_2).wrapping_add(8i32)) as u32))) as i8)) as i32);
                break 's1;
            }
            2i32 => {
                l_i_7 = (p_col_2).wrapping_add(16i32);
                l_lo_5 = (((c.m.r8((p_base_lo_3).wrapping_add(((l_i_7) as u32))) as i8)) as i32);
                l_hi_6 = (((c.m.r8((p_base_hi_4).wrapping_add(((l_i_7) as u32))) as i8)) as i32);
                break 's1;
            }
            3i32 => {
                l_i_7 = (p_col_2).wrapping_add(16i32);
                l_lo_5 = (((c.m.r8((p_base_lo_3).wrapping_add((((l_i_7).wrapping_add(8i32)) as u32))) as i8)) as i32);
                l_hi_6 = (((c.m.r8((p_base_hi_4).wrapping_add((((l_i_7).wrapping_add(8i32)) as u32))) as i8)) as i32);
                break 's1;
            }
            4i32 => {
                l_i_7 = (p_col_2).wrapping_add(32i32);
                l_lo_5 = (((c.m.r8((p_base_lo_3).wrapping_add(((l_i_7) as u32))) as i8)) as i32);
                l_hi_6 = (((c.m.r8((p_base_hi_4).wrapping_add(((l_i_7) as u32))) as i8)) as i32);
                break 's1;
            }
            5i32 => {
                l_i_7 = (p_col_2).wrapping_add(32i32);
                l_lo_5 = (((c.m.r8((p_base_lo_3).wrapping_add((((l_i_7).wrapping_add(8i32)) as u32))) as i8)) as i32);
                l_hi_6 = (((c.m.r8((p_base_hi_4).wrapping_add((((l_i_7).wrapping_add(8i32)) as u32))) as i8)) as i32);
                break 's1;
            }
            6i32 => {
                l_i_7 = (p_col_2).wrapping_add(48i32);
                l_lo_5 = (((c.m.r8((p_base_lo_3).wrapping_add(((l_i_7) as u32))) as i8)) as i32);
                l_hi_6 = (((c.m.r8((p_base_hi_4).wrapping_add(((l_i_7) as u32))) as i8)) as i32);
                break 's1;
            }
            7i32 => {
                l_i_7 = (p_col_2).wrapping_add(48i32);
                l_lo_5 = (((c.m.r8((p_base_lo_3).wrapping_add((((l_i_7).wrapping_add(8i32)) as u32))) as i8)) as i32);
                l_hi_6 = (((c.m.r8((p_base_hi_4).wrapping_add((((l_i_7).wrapping_add(8i32)) as u32))) as i8)) as i32);
                break 's1;
            }
            8i32 => {
                l_i_7 = (p_col_2).wrapping_add(64i32);
                l_lo_5 = (((c.m.r8((p_base_lo_3).wrapping_add(((l_i_7) as u32))) as i8)) as i32);
                l_hi_6 = (((c.m.r8((p_base_hi_4).wrapping_add(((l_i_7) as u32))) as i8)) as i32);
                break 's1;
            }
            9i32 => {
                l_i_7 = (p_col_2).wrapping_add(64i32);
                l_lo_5 = (((c.m.r8((p_base_lo_3).wrapping_add((((l_i_7).wrapping_add(8i32)) as u32))) as i8)) as i32);
                l_hi_6 = (((c.m.r8((p_base_hi_4).wrapping_add((((l_i_7).wrapping_add(8i32)) as u32))) as i8)) as i32);
                break 's1;
            }
            10i32 => {
                l_i_7 = (p_col_2).wrapping_add(80i32);
                l_lo_5 = (((c.m.r8((p_base_lo_3).wrapping_add(((l_i_7) as u32))) as i8)) as i32);
                l_hi_6 = (((c.m.r8((p_base_hi_4).wrapping_add(((l_i_7) as u32))) as i8)) as i32);
                break 's1;
            }
            11i32 => {
                l_i_7 = (p_col_2).wrapping_add(80i32);
                l_lo_5 = (((c.m.r8((p_base_lo_3).wrapping_add((((l_i_7).wrapping_add(8i32)) as u32))) as i8)) as i32);
                l_hi_6 = (((c.m.r8((p_base_hi_4).wrapping_add((((l_i_7).wrapping_add(8i32)) as u32))) as i8)) as i32);
                break 's1;
            }
            12i32 => {
                l_i_7 = (p_col_2).wrapping_add(96i32);
                l_lo_5 = (((c.m.r8((p_base_lo_3).wrapping_add(((l_i_7) as u32))) as i8)) as i32);
                l_hi_6 = (((c.m.r8((p_base_hi_4).wrapping_add(((l_i_7) as u32))) as i8)) as i32);
                break 's1;
            }
            13i32 => {
                l_i_7 = (p_col_2).wrapping_add(96i32);
                l_lo_5 = (((c.m.r8((p_base_lo_3).wrapping_add((((l_i_7).wrapping_add(8i32)) as u32))) as i8)) as i32);
                l_hi_6 = (((c.m.r8((p_base_hi_4).wrapping_add((((l_i_7).wrapping_add(8i32)) as u32))) as i8)) as i32);
                break 's1;
            }
            14i32 => {
                l_i_7 = (p_col_2).wrapping_add(112i32);
                l_lo_5 = (((c.m.r8((p_base_lo_3).wrapping_add(((l_i_7) as u32))) as i8)) as i32);
                l_hi_6 = (((c.m.r8((p_base_hi_4).wrapping_add(((l_i_7) as u32))) as i8)) as i32);
                break 's1;
            }
            15i32 => {
                l_i_7 = (p_col_2).wrapping_add(112i32);
                l_lo_5 = (((c.m.r8((p_base_lo_3).wrapping_add((((l_i_7).wrapping_add(8i32)) as u32))) as i8)) as i32);
                l_hi_6 = (((c.m.r8((p_base_hi_4).wrapping_add((((l_i_7).wrapping_add(8i32)) as u32))) as i8)) as i32);
                break 's1;
            }
            16i32 => {
                l_i_7 = (p_col_2).wrapping_add(128i32);
                l_lo_5 = (((c.m.r8((p_base_lo_3).wrapping_add(((l_i_7) as u32))) as i8)) as i32);
                l_hi_6 = (((c.m.r8((p_base_hi_4).wrapping_add(((l_i_7) as u32))) as i8)) as i32);
                break 's1;
            }
            17i32 => {
                l_i_7 = (p_col_2).wrapping_add(128i32);
                l_lo_5 = (((c.m.r8((p_base_lo_3).wrapping_add((((l_i_7).wrapping_add(8i32)) as u32))) as i8)) as i32);
                l_hi_6 = (((c.m.r8((p_base_hi_4).wrapping_add((((l_i_7).wrapping_add(8i32)) as u32))) as i8)) as i32);
                break 's1;
            }
            18i32 => {
                l_i_7 = (p_col_2).wrapping_add(144i32);
                l_lo_5 = (((c.m.r8((p_base_lo_3).wrapping_add(((l_i_7) as u32))) as i8)) as i32);
                l_hi_6 = (((c.m.r8((p_base_hi_4).wrapping_add(((l_i_7) as u32))) as i8)) as i32);
                break 's1;
            }
            19i32 => {
                l_i_7 = (p_col_2).wrapping_add(144i32);
                l_lo_5 = (((c.m.r8((p_base_lo_3).wrapping_add((((l_i_7).wrapping_add(8i32)) as u32))) as i8)) as i32);
                l_hi_6 = (((c.m.r8((p_base_hi_4).wrapping_add((((l_i_7).wrapping_add(8i32)) as u32))) as i8)) as i32);
                break 's1;
            }
            _ => {
                break 'f 0i32;
            }
        }
    }
    break 'f { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, l_lo_5, l_hi_6) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "AiThink_GetSubRate", &PROBE_N, &[], true); }
    r_
}

pub fn AiThink_GetSubRate4(c: &mut Ctx, mut p_ai_1: u32, mut p_col_2: i32, mut p_base_lo_3: u32, mut p_base_hi_4: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThink_GetSubRate4", &PROBE_N, &[(p_ai_1) as u32, (p_col_2) as u32, (p_base_lo_3) as u32, (p_base_hi_4) as u32], false); }
    let mut l_lo_5: i32 = 0;
    let mut l_hi_6: i32 = 0;
    let mut l_i_7: i32 = 0;
    let r_ = 'f: {
    's1: {
        let sv8: i32 = (c.m.r32(((p_ai_1).wrapping_add(744)).wrapping_add(84)) as i32);
        match sv8 {
            0i32 => {
                l_lo_5 = (((c.m.r8((p_base_lo_3).wrapping_add(((p_col_2) as u32))) as i8)) as i32);
                l_hi_6 = (((c.m.r8((p_base_hi_4).wrapping_add(((p_col_2) as u32))) as i8)) as i32);
                break 's1;
            }
            1i32 => {
                l_lo_5 = (((c.m.r8((p_base_lo_3).wrapping_add((((p_col_2).wrapping_add(8i32)) as u32))) as i8)) as i32);
                l_hi_6 = (((c.m.r8((p_base_hi_4).wrapping_add((((p_col_2).wrapping_add(8i32)) as u32))) as i8)) as i32);
                break 's1;
            }
            2i32 => {
                l_i_7 = (p_col_2).wrapping_add(16i32);
                l_lo_5 = (((c.m.r8((p_base_lo_3).wrapping_add(((l_i_7) as u32))) as i8)) as i32);
                l_hi_6 = (((c.m.r8((p_base_hi_4).wrapping_add(((l_i_7) as u32))) as i8)) as i32);
                break 's1;
            }
            3i32 => {
                l_i_7 = (p_col_2).wrapping_add(16i32);
                l_lo_5 = (((c.m.r8((p_base_lo_3).wrapping_add((((l_i_7).wrapping_add(8i32)) as u32))) as i8)) as i32);
                l_hi_6 = (((c.m.r8((p_base_hi_4).wrapping_add((((l_i_7).wrapping_add(8i32)) as u32))) as i8)) as i32);
                break 's1;
            }
            _ => {
                break 'f 0i32;
            }
        }
    }
    break 'f { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, l_lo_5, l_hi_6) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "AiThink_GetSubRate4", &PROBE_N, &[], true); }
    r_
}

pub fn AiThink_FindBasicColumn(c: &mut Ctx, mut p_cls_1: i32, mut p_code_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThink_FindBasicColumn", &PROBE_N, &[(p_cls_1) as u32, (p_code_2) as u32], false); }
    let mut l_p_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    if (((((p_cls_1) == (14i32)) as i32)) != 0) {
        break 'f (2i32).wrapping_neg();
    }
    l_p_3 = (sym::D_002EDC70).wrapping_add((((p_cls_1).wrapping_mul(8i32)) as u32));
    if (((((p_code_2) < (0i32)) as i32)) != 0) {
        break 'f (1i32).wrapping_neg();
    }
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (8i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((p_code_2) == (((c.m.r8({ let o_ = l_p_3; let n_ = (o_).wrapping_add(1); l_p_3 = n_; o_ })) as i32))) as i32)) != 0) {
                    break 'f l_i_4;
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    break 'f (1i32).wrapping_neg();
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "AiThink_FindBasicColumn", &PROBE_N, &[], true); }
    r_
}

pub fn AiThink_TestBasic(c: &mut Ctx, mut p_ai_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThink_TestBasic", &PROBE_N, &[(p_ai_1) as u32], false); }
    let mut l_plan_2: u32 = 0;
    let mut l_seq_3: u32 = 0;
    let mut l_prof_4: u32 = 0;
    let mut l_r_5: u32 = 0;
    let mut l_col_6: i32 = 0;
    let r_ = 'f: {
    l_plan_2 = (p_ai_1).wrapping_add(744);
    l_seq_3 = (p_ai_1).wrapping_add(40);
    l_prof_4 = { let a_ = (({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(200)).wrapping_add(((((c.m.r32((p_ai_1).wrapping_add(4)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    l_r_5 = ((l_plan_2).wrapping_add(20)).wrapping_add(((((c.m.r32((l_plan_2).wrapping_add(16)) as i32)) as u32)).wrapping_mul(8));
    l_col_6 = { let a0_ = (c.m.r32(l_plan_2) as i32); let a1_ = ((c.m.r32((l_plan_2).wrapping_add(12)) as i32)).wrapping_sub(35i32); F::AiThink_FindBasicColumn(c, a0_, a1_) };
    if (((((l_col_6) == ((2i32).wrapping_neg())) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((l_col_6) == ((1i32).wrapping_neg())) as i32)) != 0) {
        break 'f 0i32;
    }
    { let r_ = { let a1_ = (((c.m.r32(l_plan_2) as i32)) as u32); F::BtlAi_GetPairRate(c, p_ai_1, a1_, l_col_6, (l_prof_4).wrapping_add(((4i32) as u32)), (l_prof_4).wrapping_add(((708i32) as u32)), 0i32) }; let a_ = (l_r_5).wrapping_add(4); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
    { let v_ = (c.m.r32(l_r_5) as i32); let a_ = (l_seq_3).wrapping_add(132); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32((l_r_5).wrapping_add(4)) as i32); let a_ = (l_seq_3).wrapping_add(136); c.m.w32(a_, (v_) as u32); };
    if ((((((c.m.r32(l_r_5) as i32)) < ((c.m.r32((l_r_5).wrapping_add(4)) as i32))) as i32)) != 0) {
        { let v_ = (((c.m.r32(l_r_5) as i32)) as u8); let a_ = (l_plan_2).wrapping_add(169); c.m.w8(a_, v_); };
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "AiThink_TestBasic", &PROBE_N, &[], true); }
    r_
}

pub fn AiThCond_BasicNoFlag6(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThCond_BasicNoFlag6", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    if ((((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlSide_IsPoweredUp(c, a0_) }) != (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_TestFlag98(c, a0_) }) != (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f F::AiThink_TestBasic(c, p_ai_1);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "AiThCond_BasicNoFlag6", &PROBE_N, &[], true); }
    r_
}

pub fn AiThink_FindWeightColumn(c: &mut Ctx, mut p_plan_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThink_FindWeightColumn", &PROBE_N, &[(p_plan_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_code_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_v_4: u8 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 24);
        { let v_ = 8i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
        { let v_ = 8i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
        { let v_ = 4i32; c.m.w32(fp.wrapping_add(8), (v_) as u32); }
        { let v_ = 8i32; c.m.w32(fp.wrapping_add(12), (v_) as u32); }
        { let v_ = 4i32; c.m.w32(fp.wrapping_add(16), (v_) as u32); }
        { let v_ = 8i32; c.m.w32(fp.wrapping_add(20), (v_) as u32); }
        l_code_2 = ((c.m.r32((p_plan_1).wrapping_add(12)) as i32)).wrapping_sub(40i32);
        if (((((l_code_2) < (0i32)) as i32)) != 0) {
            break 'f (1i32).wrapping_neg();
        }
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < ({ let a_ = (fp).wrapping_add(((((c.m.r32((p_plan_1).wrapping_add(4)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l1; }
                'c2: {
                    's3: {
                        let sv5: i32 = (c.m.r32((p_plan_1).wrapping_add(4)) as i32);
                        match sv5 {
                            0i32 => {
                                l_v_4 = { let a_ = (sym::D_002EDCE0).wrapping_add((((l_i_3).wrapping_add(((c.m.r32(p_plan_1) as i32)).wrapping_mul(8i32))) as u32)); c.m.r8(a_) };
                                break 's3;
                            }
                            1i32 => {
                                l_v_4 = { let a_ = (sym::D_002EDD50).wrapping_add((((l_i_3).wrapping_add(((c.m.r32(p_plan_1) as i32)).wrapping_mul(8i32))) as u32)); c.m.r8(a_) };
                                break 's3;
                            }
                            2i32 => {
                                l_v_4 = { let a_ = (sym::D_002EDDD8).wrapping_add((((l_i_3).wrapping_add(((c.m.r32(p_plan_1) as i32)).wrapping_mul(4i32))) as u32)); c.m.r8(a_) };
                                break 's3;
                            }
                            3i32 => {
                                l_v_4 = { let a_ = (sym::D_002EDE10).wrapping_add((((l_i_3).wrapping_add(((c.m.r32((p_plan_1).wrapping_add(84)) as i32)).wrapping_mul(8i32))) as u32)); c.m.r8(a_) };
                                break 's3;
                            }
                            4i32 => {
                                l_v_4 = { let a_ = (sym::D_002EDEB0).wrapping_add((((l_i_3).wrapping_add(((c.m.r32(p_plan_1) as i32)).wrapping_mul(4i32))) as u32)); c.m.r8(a_) };
                                break 's3;
                            }
                            5i32 => {
                                l_v_4 = { let a_ = (sym::D_002EDEE8).wrapping_add((((l_i_3).wrapping_add(((c.m.r32((p_plan_1).wrapping_add(84)) as i32)).wrapping_mul(8i32))) as u32)); c.m.r8(a_) };
                                break 's3;
                            }
                            _ => {
                                break 'f (1i32).wrapping_neg();
                            }
                        }
                    }
                    if (((((l_code_2) == (((l_v_4) as i32))) as i32)) != 0) {
                        break 'f l_i_3;
                    }
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        break 'f (1i32).wrapping_neg();
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "AiThink_FindWeightColumn", &PROBE_N, &[], true); }
    r_
}

pub fn AiThink_TestWeighted(c: &mut Ctx, mut p_ai_1: u32, mut p_byGauge_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThink_TestWeighted", &PROBE_N, &[(p_ai_1) as u32, (p_byGauge_2) as u32], false); }
    let mut l_plan_3: u32 = 0;
    let mut l_st_4: u32 = 0;
    let mut l_seq_5: u32 = 0;
    let mut l_prof_6: u32 = 0;
    let mut l_r_7: u32 = 0;
    let mut l_fn_8: i32 = 0;
    let mut l_skills_9: u32 = 0;
    let mut l_dist_10: i32 = 0;
    let mut l_near_11: i32 = 0;
    let mut l_col_12: i32 = 0;
    let mut l_rate_13: i32 = 0;
    let r_ = 'f: {
    l_plan_3 = (p_ai_1).wrapping_add(744);
    l_st_4 = (p_ai_1).wrapping_add(704);
    l_seq_5 = (p_ai_1).wrapping_add(40);
    l_prof_6 = { let a_ = (({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(200)).wrapping_add(((((c.m.r32((p_ai_1).wrapping_add(4)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    l_r_7 = ((l_plan_3).wrapping_add(20)).wrapping_add(((((c.m.r32((l_plan_3).wrapping_add(16)) as i32)) as u32)).wrapping_mul(8));
    l_fn_8 = { let a_ = (sym::D_002EDA70).wrapping_add(((((c.m.r32((l_plan_3).wrapping_add(12)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
    l_skills_9 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetSkillTable(c, a0_) };
    l_dist_10 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetHp(c, a0_) };
    l_near_11 = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = ((c.m.r8((l_prof_6).wrapping_add(((686i32) as u32)))) as i32); let a2_ = ((c.m.r8((l_prof_6).wrapping_add(((1390i32) as u32)))) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) };
    l_col_12 = F::AiThink_FindWeightColumn(c, l_plan_3);
    if (((((l_col_12) == ((1i32).wrapping_neg())) as i32)) != 0) {
        break 'f 0i32;
    }
    's1: {
        let sv14: i32 = (c.m.r32((l_plan_3).wrapping_add(4)) as i32);
        match sv14 {
            0i32 => {
                l_rate_13 = { let a1_ = (((c.m.r32(l_plan_3) as i32)) as u32); F::BtlAi_GetPairRate(c, p_ai_1, a1_, l_col_12, (l_prof_6).wrapping_add(((116i32) as u32)), (l_prof_6).wrapping_add(((820i32) as u32)), p_byGauge_2) };
                break 's1;
            }
            1i32 => {
                l_rate_13 = { let a1_ = (((c.m.r32(l_plan_3) as i32)) as u32); F::BtlAi_GetPairRate(c, p_ai_1, a1_, l_col_12, (l_prof_6).wrapping_add(((228i32) as u32)), (l_prof_6).wrapping_add(((932i32) as u32)), p_byGauge_2) };
                break 's1;
            }
            2i32 => {
                l_rate_13 = { let a1_ = (((c.m.r32(l_plan_3) as i32)) as u32); F::BtlAi_GetQuadRate(c, p_ai_1, a1_, l_col_12, (l_prof_6).wrapping_add(((376i32) as u32)), (l_prof_6).wrapping_add(((1080i32) as u32)), p_byGauge_2) };
                break 's1;
            }
            3i32 => {
                l_rate_13 = F::AiThink_GetSubRate(c, p_ai_1, l_col_12, (l_prof_6).wrapping_add(((432i32) as u32)), (l_prof_6).wrapping_add(((1136i32) as u32)));
                break 's1;
            }
            4i32 => {
                l_rate_13 = { let a1_ = (((c.m.r32(l_plan_3) as i32)) as u32); F::BtlAi_GetQuadRate(c, p_ai_1, a1_, l_col_12, (l_prof_6).wrapping_add(((592i32) as u32)), (l_prof_6).wrapping_add(((1296i32) as u32)), p_byGauge_2) };
                break 's1;
            }
            5i32 => {
                l_rate_13 = F::AiThink_GetSubRate4(c, p_ai_1, l_col_12, (l_prof_6).wrapping_add(((648i32) as u32)), (l_prof_6).wrapping_add(((1352i32) as u32)));
                break 's1;
            }
            _ => {
                break 'f 0i32;
            }
        }
    }
    if ((((c.m.r64(l_st_4)) & (((3458764513820540928i64) as u64)))) != 0) {
        if (((((((((l_fn_8) == (49i32)) as i32)) != 0) || (((((l_fn_8) == (81i32)) as i32)) != 0)) as i32)) != 0) {
            l_rate_13 = rt::f2i(rt::fmul(rt::i2f(l_rate_13), F32(0x3f000000)));
        }
        if ((((c.m.r64(l_st_4)) & (((2305843009213693952i64) as u64)))) != 0) {
            if (((((((((l_fn_8) == (51i32)) as i32)) != 0) || (((((l_fn_8) == (94i32)) as i32)) != 0)) as i32)) != 0) {
                l_rate_13 = 0i32;
            }
            if ((((((((l_fn_8).wrapping_sub(95i32)) as u32)) < (((2i32) as u32))) as i32)) != 0) {
                if ((((((c.m.r32((l_plan_3).wrapping_add(8)) as i32)) == ((1i32).wrapping_neg())) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((!(((({ let a_ = ((l_skills_9).wrapping_add(12)).wrapping_add(((((c.m.r32((l_plan_3).wrapping_add(8)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }) & (((134217728i32) as u32)))) != 0)) as i32)) != 0) {
                    l_rate_13 = rt::f2i(rt::fmul(rt::i2f(l_rate_13), F32(0x3f000000)));
                }
            }
        }
    }
    if (((((l_dist_10) < (10000i32)) as i32)) != 0) {
        if (((((((((l_near_11) != (0i32)) as i32)) != 0) && (((((l_fn_8) == (93i32)) as i32)) != 0)) as i32)) != 0) {
            l_rate_13 = l_near_11;
        }
    }
    { let r_ = l_rate_13; let a_ = (l_r_7).wrapping_add(4); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
    { let v_ = (c.m.r32(l_r_7) as i32); let a_ = (l_seq_5).wrapping_add(132); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32((l_r_7).wrapping_add(4)) as i32); let a_ = (l_seq_5).wrapping_add(136); c.m.w32(a_, (v_) as u32); };
    break 'f ((((c.m.r32(l_r_7) as i32)) < ((c.m.r32((l_r_7).wrapping_add(4)) as i32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "AiThink_TestWeighted", &PROBE_N, &[], true); }
    r_
}

pub fn AiThink_TestMove(c: &mut Ctx, mut p_ai_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThink_TestMove", &PROBE_N, &[(p_ai_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_plan_2: u32 = 0;
    let mut l_count_3: i32 = 0;
    let mut l_moves_4: u32 = 0;
    let mut l_stock_5: i32 = 0;
    let mut l_kind_6: i32 = 0;
    let mut l_dist_7: i32 = 0;
    let mut l_oppVal_8: i32 = 0;
    let mut l_i_9: i32 = 0;
    let mut l_blocked_10: i32 = 0;
    let r_ = 'f: {
        l_plan_2 = (p_ai_1).wrapping_add(744);
        l_count_3 = 0i32;
        l_moves_4 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetMoveTable(c, a0_) };
        l_stock_5 = rt::div_i32({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlSide_GetBlast(c, a0_) }, 100000i32);
        l_kind_6 = ((c.m.r32((l_plan_2).wrapping_add(12)) as i32)).wrapping_sub(92i32);
        l_dist_7 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetHp(c, a0_) };
        l_oppVal_8 = { let a0_ = (((c.m.r32(p_ai_1) as i32)) ^ (1i32)); F::BtlCharApi_GetStunTimer(c, a0_) };
        if (((((((l_kind_6) as u32)) >= (((9i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        { let v_ = (1i32).wrapping_neg(); let a_ = (l_plan_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        {
            l_i_9 = 0i32;
            'l1: loop {
                if !(((((l_i_9) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((l_kind_6) != ((((c.m.r8(((l_moves_4).wrapping_add(158)).wrapping_add(((l_i_9) as u32))) as i8)) as i32))) as i32)) != 0) {
                        break 'c2;
                    }
                    if ((((((c.m.r32((l_plan_2).wrapping_add(164)) as i32)) > (0i32)) as i32)) != 0) {
                        break 'c2;
                    }
                    l_blocked_10 = 0i32;
                    's3: {
                        let sv11: i32 = l_kind_6;
                        match sv11 {
                            0i32 => {
                                if ((((((((c.m.r16(((l_moves_4).wrapping_add(16)).wrapping_add((((l_i_9) as u32)).wrapping_mul(2))) as i16)) as i32)) == (1i32)) as i32)) != 0) {
                                    if ((((({ let a0_ = (((c.m.r32(p_ai_1) as i32)) ^ (1i32)); F::BtlCharApi_TestFlag05(c, a0_) }) == (0i32)) as i32)) != 0) {
                                        l_blocked_10 = 1i32;
                                    }
                                } else {
                                    if ((((!(((((l_oppVal_8) < (31i32)) as i32)) != 0)) as i32)) != 0) {
                                        l_blocked_10 = 1i32;
                                    }
                                }
                                break 's3;
                            }
                            1i32 | 2i32 | 3i32 | 4i32 => {
                                break 's3;
                            }
                            5i32 => {
                                if ((((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_IsMoveFlag100(c, a0_, ((((l_i_9) as i64 as u64)) as u32)) }) == (0i32)) as i32)) != 0) {
                                    if ((((({ let a0_ = (c.m.r32(p_ai_1) as i32); ((D::call(c, fid::BtlCharApi_IsMoveSlotActive, &[((a0_) as i64 as u64), ((l_i_9) as i64 as u64)])) as i32) }) != (0i32)) as i32)) != 0) {
                                        l_blocked_10 = 1i32;
                                    }
                                }
                                break 's3;
                            }
                            6i32 => {
                                l_blocked_10 = ((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlSide_IsPoweredUp(c, a0_) }) != (0i32)) as i32);
                                break 's3;
                            }
                            7i32 | 8i32 => {
                                break 's3;
                            }
                            _ => {
                            }
                        }
                    }
                    if ((l_blocked_10) != 0) {
                        break 'c2;
                    }
                    if ((((((((((((c.m.r16(((l_moves_4).wrapping_add(16)).wrapping_add((((l_i_9) as u32)).wrapping_mul(2))) as i16)) as i32)) == (23i32)) as i32)) != 0) && (((((l_dist_7) < (10000i32)) as i32)) != 0)) as i32)) != 0) {
                        break 'c2;
                    }
                    if (((((l_stock_5) < ((((c.m.r8(((l_moves_4).wrapping_add(148)).wrapping_add(((l_i_9) as u32))) as i8)) as i32))) as i32)) != 0) {
                        break 'c2;
                    }
                    { let v_ = l_i_9; let a_ = (fp).wrapping_add(((({ let o_ = l_count_3; let n_ = (o_).wrapping_add(1); l_count_3 = n_; o_ }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = l_i_9; let a_ = (l_plan_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                }
                { let o_ = l_i_9; let n_ = (o_).wrapping_add(1); l_i_9 = n_; };
            }
        }
        if (((((l_count_3) == (0i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        if (((((F::AiThink_TestWeighted(c, p_ai_1, 0i32)) == (0i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        if (((((l_count_3) == (1i32)) as i32)) != 0) {
            { let v_ = (c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a_ = (l_plan_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = { let a_ = (fp).wrapping_add((F::Rand_Range(c, ((l_count_3) as u32))).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (l_plan_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "AiThink_TestMove", &PROBE_N, &[], true); }
    r_
}

pub fn AiThink_TestSkill(c: &mut Ctx, mut p_ai_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThink_TestSkill", &PROBE_N, &[(p_ai_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_gauge_2: i32 = 0;
    let mut l_plan_3: u32 = 0;
    let mut l_kind_4: i32 = 0;
    let mut l_count_5: i32 = 0;
    let mut l_anyBasic_6: i32 = 0;
    let mut l_beatOpp_7: i32 = 0;
    let mut l_dist_8: i32 = 0;
    let mut l_strict_9: i32 = 0;
    let mut l_skills_10: u32 = 0;
    let mut l_opp_11: u32 = 0;
    let mut l_oppSlot_12: i32 = 0;
    let mut l_n_13: i32 = 0;
    let mut l_i_14: i32 = 0;
    let r_ = 'f: {
        l_n_13 = 2i32;
        l_plan_3 = (p_ai_1).wrapping_add(744);
        l_count_5 = 0i32;
        l_anyBasic_6 = 0i32;
        l_skills_10 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetSkillTable(c, a0_) };
        l_beatOpp_7 = 0i32;
        l_strict_9 = 1i32;
        l_gauge_2 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlSide_GetKi(c, a0_) };
        l_kind_4 = ((c.m.r32((l_plan_3).wrapping_add(12)) as i32)).wrapping_sub(101i32);
        l_dist_8 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetHp(c, a0_) };
        l_oppSlot_12 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetOppSkillClass(c, a0_) };
        l_opp_11 = { let a0_ = (((c.m.r32(p_ai_1) as i32)) ^ (1i32)); F::BtlCharApi_GetSkillTable(c, a0_) };
        if ((((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetUnkE40(c, a0_) }) != (0i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((((({ let a_ = (sym::D_002EDA70).wrapping_add(((((c.m.r32((l_plan_3).wrapping_add(12)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (107i32)) as i32)) != 0) {
            l_anyBasic_6 = 1i32;
        } else {
            if ((((({ let a_ = (sym::D_002EDA70).wrapping_add(((((c.m.r32((l_plan_3).wrapping_add(12)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (106i32)) as i32)) != 0) {
                l_beatOpp_7 = 1i32;
                if (((((l_oppSlot_12) != ((1i32).wrapping_neg())) as i32)) != 0) {
                    if ((((c.m.r32((l_opp_11).wrapping_add((((l_oppSlot_12) as u32)).wrapping_mul(4)))) & (((256i32) as u32)))) != 0) {
                        l_strict_9 = 0i32;
                    } else {
                        l_strict_9 = 1i32;
                    }
                }
            }
        }
        if (((((((((((((((l_kind_4) as u32)) >= (((7i32) as u32))) as i32)) != 0) && (((((l_anyBasic_6) == (0i32)) as i32)) != 0)) as i32)) != 0) && (((((l_beatOpp_7) == (0i32)) as i32)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        { let v_ = (1i32).wrapping_neg(); let a_ = (l_plan_3).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (sym::D_002EDA70).wrapping_add(((((c.m.r32((l_plan_3).wrapping_add(12)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (94i32)) as i32)) != 0) {
            if (((rt::flt(rt::fmul(F32(c.m.r32(((p_ai_1).wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0x3fc00000)), { let a_ = (c.m.r32(sym::gBtlAi)).wrapping_add(4); F32(c.m.r32(a_)) }) as i32)) != 0) {
                break 'f 0i32;
            }
        }
        if ((((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlSide_IsPoweredUp(c, a0_) }) != (0i32)) as i32)) != 0) {
            l_n_13 = 3i32;
        }
        {
            l_i_14 = (l_n_13).wrapping_sub(1i32);
            'l1: loop {
                if !(((((l_i_14) >= (0i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((l_anyBasic_6) != (0i32)) as i32)) != 0) {
                        if ((((((((((((c.m.r8(((l_skills_10).wrapping_add(357)).wrapping_add(((l_i_14) as u32))) as i8)) as i32)) != (1i32)) as i32)) != 0) && ((((((((c.m.r8(((l_skills_10).wrapping_add(357)).wrapping_add(((l_i_14) as u32))) as i8)) as i32)) != (2i32)) as i32)) != 0)) as i32)) != 0) {
                            break 'c2;
                        }
                    } else {
                        if (((((l_beatOpp_7) != (0i32)) as i32)) != 0) {
                            if (((((l_strict_9) == (1i32)) as i32)) != 0) {
                                if ((((c.m.r32((l_skills_10).wrapping_add((((l_i_14) as u32)).wrapping_mul(4)))) & (((256i32) as u32)))) != 0) {
                                    break 'c2;
                                }
                                if (((((l_oppSlot_12) != ((1i32).wrapping_neg())) as i32)) != 0) {
                                    if ((((((((c.m.r16(((l_opp_11).wrapping_add(30)).wrapping_add((((l_oppSlot_12) as u32)).wrapping_mul(2))) as i16)) as i32)) > ((((c.m.r16(((l_skills_10).wrapping_add(30)).wrapping_add((((l_i_14) as u32)).wrapping_mul(2))) as i16)) as i32))) as i32)) != 0) {
                                        break 'c2;
                                    }
                                    if ((((((((((((c.m.r16(((l_opp_11).wrapping_add(30)).wrapping_add((((l_oppSlot_12) as u32)).wrapping_mul(2))) as i16)) as i32)) == ((((c.m.r16(((l_skills_10).wrapping_add(30)).wrapping_add((((l_i_14) as u32)).wrapping_mul(2))) as i16)) as i32))) as i32)) != 0) && ((((((((c.m.r8(((l_skills_10).wrapping_add(147)).wrapping_add(((l_i_14) as u32))) as i8)) as i32)) < ((((c.m.r8(((l_opp_11).wrapping_add(147)).wrapping_add(((l_oppSlot_12) as u32))) as i8)) as i32))) as i32)) != 0)) as i32)) != 0) {
                                        break 'c2;
                                    }
                                }
                            } else {
                                if (((((l_oppSlot_12) != ((1i32).wrapping_neg())) as i32)) != 0) {
                                    if ((((((((c.m.r16(((l_opp_11).wrapping_add(30)).wrapping_add((((l_oppSlot_12) as u32)).wrapping_mul(2))) as i16)) as i32)) > ((((c.m.r16(((l_skills_10).wrapping_add(30)).wrapping_add((((l_i_14) as u32)).wrapping_mul(2))) as i16)) as i32))) as i32)) != 0) {
                                        break 'c2;
                                    }
                                } else {
                                    if ((((((((c.m.r8(((l_skills_10).wrapping_add(357)).wrapping_add(((l_i_14) as u32))) as i8)) as i32)) != (3i32)) as i32)) != 0) {
                                        break 'c2;
                                    }
                                }
                            }
                        } else {
                            if (((((l_kind_4) != ((((c.m.r8(((l_skills_10).wrapping_add(357)).wrapping_add(((l_i_14) as u32))) as i8)) as i32))) as i32)) != 0) {
                                break 'c2;
                            }
                        }
                    }
                    if ((((((((((((c.m.r8(((l_skills_10).wrapping_add(357)).wrapping_add(((l_i_14) as u32))) as i8)) as i32)) == (5i32)) as i32)) != 0) && (((((F::BtlChar_IsStage4Or27(c)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
                        if ((((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_TestPoseBit80(c, a0_, 1i32) }) == (0i32)) as i32)) != 0) {
                            break 'c2;
                        }
                        if ((((({ let a0_ = (((c.m.r32(p_ai_1) as i32)) ^ (1i32)); F::BtlCharApi_TestPoseBit80(c, a0_, 1i32) }) == (0i32)) as i32)) != 0) {
                            break 'c2;
                        }
                    }
                    if ((((((((c.m.r32(((l_skills_10).wrapping_add(12)).wrapping_add((((l_i_14) as u32)).wrapping_mul(4)))) & (((8192i32) as u32)))) != 0) || ((((((((c.m.r16(((l_skills_10).wrapping_add(24)).wrapping_add((((l_i_14) as u32)).wrapping_mul(2))) as i16)) as i32)) == (640i32)) as i32)) != 0)) as i32)) != 0) {
                        if (((((l_dist_8) > (10000i32)) as i32)) != 0) {
                            break 'c2;
                        }
                    }
                    if (((((l_gauge_2) < ((c.m.r32(((l_skills_10).wrapping_add(412)).wrapping_add((((l_i_14) as u32)).wrapping_mul(4))) as i32))) as i32)) != 0) {
                        break 'c2;
                    }
                    { let v_ = l_i_14; let a_ = (fp).wrapping_add(((({ let o_ = l_count_5; let n_ = (o_).wrapping_add(1); l_count_5 = n_; o_ }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                }
                { let o_ = l_i_14; let n_ = (o_).wrapping_sub(1); l_i_14 = n_; };
            }
        }
        if (((((l_count_5) == (0i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        if (((((l_count_5) == (1i32)) as i32)) != 0) {
            { let v_ = (c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a_ = (l_plan_3).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        }
        if (((((l_count_5) >= (2i32)) as i32)) != 0) {
            if ((((((((((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) == (2i32)) as i32)) != 0) && (((((((F::Rand_Range(c, ((100i32) as u32))) as i32)) < (50i32)) as i32)) != 0)) as i32)) != 0) {
                { let v_ = (c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a_ = (l_plan_3).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = { let a_ = (fp).wrapping_add((F::Rand_Range(c, ((l_count_5) as u32))).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (l_plan_3).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            }
        }
        if ((((({ let a1_ = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlSide_IsPoweredUp(c, a0_) }; F::AiThink_TestWeighted(c, p_ai_1, a1_) }) == (0i32)) as i32)) != 0) {
            { let v_ = (1i32).wrapping_neg(); let a_ = (l_plan_3).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            break 'f 0i32;
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "AiThink_TestSkill", &PROBE_N, &[], true); }
    r_
}

pub fn AiThCond_WeightedIfBit0(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThCond_WeightedIfBit0", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    if (((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetParamUnk14(c, a0_) }) & (1i32))) != 0) {
        break 'f F::AiThink_TestWeighted(c, p_ai_1, 0i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "AiThCond_WeightedIfBit0", &PROBE_N, &[], true); }
    r_
}

pub fn AiThCond_MiscRate0Or3(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThCond_MiscRate0Or3", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let mut l_seq_3: u32 = 0;
    let mut l_prof_4: u32 = 0;
    let mut l_lo_5: u32 = 0;
    let mut l_hi_6: u32 = 0;
    let mut l_roll_7: i32 = 0;
    let mut l_stage_8: i32 = 0;
    let mut l_chance_9: i32 = 0;
    let r_ = 'f: {
    l_seq_3 = (p_ai_1).wrapping_add(40);
    l_prof_4 = { let a_ = (({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(200)).wrapping_add(((((c.m.r32((p_ai_1).wrapping_add(4)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    l_lo_5 = (l_prof_4).wrapping_add(((696i32) as u32));
    l_hi_6 = (l_prof_4).wrapping_add(((1400i32) as u32));
    l_roll_7 = ((F::Rand_Range(c, ((100i32) as u32))) as i32);
    l_stage_8 = F::BtlChar_IsStage4Or27(c);
    's1: {
        let sv10: i32 = ((p_arg_2) as i32);
        match sv10 {
            0i32 => {
                l_chance_9 = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = ((c.m.r8((l_lo_5).wrapping_add(((0i32) as u32)))) as i32); let a2_ = ((c.m.r8((l_hi_6).wrapping_add(((0i32) as u32)))) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) };
                break 's1;
            }
            1i32 => {
                l_chance_9 = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = ((c.m.r8((l_lo_5).wrapping_add(((3i32) as u32)))) as i32); let a2_ = ((c.m.r8((l_hi_6).wrapping_add(((3i32) as u32)))) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) };
                break 's1;
            }
            _ => {
                l_chance_9 = 0i32;
                break 's1;
            }
        }
    }
    if (((((((((l_stage_8) != (0i32)) as i32)) != 0) && ((((({ let a0_ = (((c.m.r32(p_ai_1) as i32)) ^ (1i32)); F::BtlCharApi_TestPoseBit80(c, a0_, 1i32) }) == (0i32)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let v_ = l_roll_7; let a_ = (l_seq_3).wrapping_add(132); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_chance_9; let a_ = (l_seq_3).wrapping_add(136); c.m.w32(a_, (v_) as u32); };
    break 'f (((l_roll_7) < (l_chance_9)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "AiThCond_MiscRate0Or3", &PROBE_N, &[], true); }
    r_
}

pub fn AiThCond_MiscRate1(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThCond_MiscRate1", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let mut l_prof_3: u32 = 0;
    let mut l_chance_4: i32 = 0;
    let mut l_roll_5: i32 = 0;
    let mut l_seq_6: u32 = 0;
    let r_ = 'f: {
    l_prof_3 = { let a_ = (({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(200)).wrapping_add(((((c.m.r32((p_ai_1).wrapping_add(4)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    l_chance_4 = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = ((c.m.r8((l_prof_3).wrapping_add(((697i32) as u32)))) as i32); let a2_ = ((c.m.r8((l_prof_3).wrapping_add(((1401i32) as u32)))) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) };
    l_roll_5 = ((F::Rand_Range(c, ((100i32) as u32))) as i32);
    l_seq_6 = (p_ai_1).wrapping_add(40);
    { let v_ = l_roll_5; let a_ = (l_seq_6).wrapping_add(132); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_chance_4; let a_ = (l_seq_6).wrapping_add(136); c.m.w32(a_, (v_) as u32); };
    break 'f (((l_roll_5) < (l_chance_4)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "AiThCond_MiscRate1", &PROBE_N, &[], true); }
    r_
}

pub fn AiThCond_MiscRate2(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThCond_MiscRate2", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let mut l_prof_3: u32 = 0;
    let mut l_chance_4: i32 = 0;
    let mut l_roll_5: i32 = 0;
    let mut l_seq_6: u32 = 0;
    let r_ = 'f: {
    l_prof_3 = { let a_ = (({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(200)).wrapping_add(((((c.m.r32((p_ai_1).wrapping_add(4)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    l_chance_4 = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = ((c.m.r8((l_prof_3).wrapping_add(((698i32) as u32)))) as i32); let a2_ = ((c.m.r8((l_prof_3).wrapping_add(((1402i32) as u32)))) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) };
    l_roll_5 = ((F::Rand_Range(c, ((100i32) as u32))) as i32);
    l_seq_6 = (p_ai_1).wrapping_add(40);
    { let v_ = l_roll_5; let a_ = (l_seq_6).wrapping_add(132); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_chance_4; let a_ = (l_seq_6).wrapping_add(136); c.m.w32(a_, (v_) as u32); };
    break 'f (((l_roll_5) < (l_chance_4)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "AiThCond_MiscRate2", &PROBE_N, &[], true); }
    r_
}

pub fn AiThCond_MiscRate4(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThCond_MiscRate4", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let mut l_prof_3: u32 = 0;
    let mut l_chance_4: i32 = 0;
    let mut l_roll_5: i32 = 0;
    let mut l_seq_6: u32 = 0;
    let r_ = 'f: {
    l_prof_3 = { let a_ = (({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(200)).wrapping_add(((((c.m.r32((p_ai_1).wrapping_add(4)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    l_chance_4 = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = ((c.m.r8((l_prof_3).wrapping_add(((700i32) as u32)))) as i32); let a2_ = ((c.m.r8((l_prof_3).wrapping_add(((1404i32) as u32)))) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) };
    l_roll_5 = ((F::Rand_Range(c, ((100i32) as u32))) as i32);
    l_seq_6 = (p_ai_1).wrapping_add(40);
    { let v_ = l_roll_5; let a_ = (l_seq_6).wrapping_add(132); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_chance_4; let a_ = (l_seq_6).wrapping_add(136); c.m.w32(a_, (v_) as u32); };
    break 'f (((l_roll_5) < (l_chance_4)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "AiThCond_MiscRate4", &PROBE_N, &[], true); }
    r_
}

pub fn AiThCond_React4000(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThCond_React4000", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let mut l_seq_3: u32 = 0;
    let mut l_prof_4: u32 = 0;
    let mut l_chance_5: i32 = 0;
    let mut l_roll_6: i32 = 0;
    let r_ = 'f: {
    l_seq_3 = (p_ai_1).wrapping_add(40);
    l_prof_4 = { let a_ = (({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(200)).wrapping_add(((((c.m.r32((p_ai_1).wrapping_add(4)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    l_chance_5 = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = ((c.m.r8((l_prof_4).wrapping_add(((692i32) as u32)))) as i32); let a2_ = ((c.m.r8((l_prof_4).wrapping_add(((1396i32) as u32)))) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) };
    l_roll_6 = ((F::Rand_Range(c, ((100i32) as u32))) as i32);
    F::BtlAi_NoteOpponent(c, p_ai_1, 16384i32);
    { let v_ = l_roll_6; let a_ = (l_seq_3).wrapping_add(132); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_chance_5; let a_ = (l_seq_3).wrapping_add(136); c.m.w32(a_, (v_) as u32); };
    break 'f (((l_roll_6) < (l_chance_5)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "AiThCond_React4000", &PROBE_N, &[], true); }
    r_
}

pub fn AiThCond_React8000(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThCond_React8000", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let mut l_seq_3: u32 = 0;
    let mut l_prof_4: u32 = 0;
    let mut l_chance_5: i32 = 0;
    let mut l_roll_6: i32 = 0;
    let r_ = 'f: {
    l_seq_3 = (p_ai_1).wrapping_add(40);
    l_prof_4 = { let a_ = (({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(200)).wrapping_add(((((c.m.r32((p_ai_1).wrapping_add(4)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    l_chance_5 = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = ((c.m.r8((l_prof_4).wrapping_add(((693i32) as u32)))) as i32); let a2_ = ((c.m.r8((l_prof_4).wrapping_add(((1397i32) as u32)))) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) };
    l_roll_6 = ((F::Rand_Range(c, ((100i32) as u32))) as i32);
    F::BtlAi_NoteOpponent(c, p_ai_1, 32768i32);
    { let v_ = l_roll_6; let a_ = (l_seq_3).wrapping_add(132); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_chance_5; let a_ = (l_seq_3).wrapping_add(136); c.m.w32(a_, (v_) as u32); };
    break 'f (((l_roll_6) < (l_chance_5)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "AiThCond_React8000", &PROBE_N, &[], true); }
    r_
}

pub fn AiThCond_Basic(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThCond_Basic", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    break 'f F::AiThink_TestBasic(c, p_ai_1);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "AiThCond_Basic", &PROBE_N, &[], true); }
    r_
}

pub fn AiThCond_Weighted(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThCond_Weighted", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    break 'f F::AiThink_TestWeighted(c, p_ai_1, 0i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "AiThCond_Weighted", &PROBE_N, &[], true); }
    r_
}

pub fn AiThCond_WeightedByHeight(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThCond_WeightedByHeight", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_st_3: u32 = 0;
    let mut l_plan_4: u32 = 0;
    let mut l_height_5: F32 = F32(0);
    let mut l_dy_6: F32 = F32(0);
    let r_ = 'f: {
        l_st_3 = (p_ai_1).wrapping_add(704);
        l_plan_4 = (p_ai_1).wrapping_add(744);
        l_height_5 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetHeight(c, a0_) };
        { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetPos(c, a0_, fp) };
        { let a0_ = (((c.m.r32(p_ai_1) as i32)) ^ (1i32)); F::BtlCharApi_GetPos(c, a0_, fp.wrapping_add(16)) };
        l_dy_6 = rt::fsub(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(c.m.r32((fp).wrapping_add(4))));
        's1: {
            let sv7: i32 = { let a_ = (sym::D_002EDA70).wrapping_add(((((c.m.r32((l_plan_4).wrapping_add(12)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
            match sv7 {
                54i32 | 83i32 => {
                    if (((rt::flt(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), rt::fsub(F32(c.m.r32((fp).wrapping_add(4))), l_height_5)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if ((((!((((c.m.r64(l_st_3)) & (((48i32) as u64)))) != 0)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    break 's1;
                }
                84i32 => {
                    if (((rt::flt(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), rt::fsub(F32(c.m.r32((fp).wrapping_add(4))), l_height_5)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if ((((!((((c.m.r64(l_st_3)) & (((32i32) as u64)))) != 0)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    break 's1;
                }
                55i32 => {
                    if ((((({ let a0_ = (((c.m.r32(p_ai_1) as i32)) ^ (1i32)); F::BtlCharApi_IsAnimFlag2800(c, a0_) }) == (0i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if (((rt::fgt({ let a_ = (c.m.r32(sym::gBtlAi)).wrapping_add(4); F32(c.m.r32(a_)) }, rt::fadd({ let a_ = (c.m.r32(sym::gBtlAi)).wrapping_add(8); F32(c.m.r32(a_)) }, { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetRadius(c, a0_) })) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if (((rt::fgt((if (((rt::flt(l_dy_6, F32(0x00000000)) as i32)) != 0) { rt::fneg(l_dy_6) } else { l_dy_6 }), l_height_5) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if (((((F::BtlAiSense_IsBehindOpponent(c, p_ai_1)) != (0i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
        break 'f F::AiThink_TestWeighted(c, p_ai_1, 0i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "AiThCond_WeightedByHeight", &PROBE_N, &[], true); }
    r_
}

pub fn AiThCond_ActRateByFlags(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThCond_ActRateByFlags", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_idx_3: i32 = 0;
    let mut l_seq_4: u32 = 0;
    let mut l_plan_5: u32 = 0;
    let mut l_roll_6: i32 = 0;
    let mut l_prof_7: u32 = 0;
    let mut l_code_8: i32 = 0;
    let mut l_f_9: u64 = 0;
    let mut l_act_10: u32 = 0;
    let mut l_chance_11: i32 = 0;
    let mut l_i_12: i32 = 0;
    let r_ = 'f: {
        l_idx_3 = (1i32).wrapping_neg();
        l_seq_4 = (p_ai_1).wrapping_add(40);
        l_plan_5 = (p_ai_1).wrapping_add(744);
        l_roll_6 = ((F::Rand_Range(c, ((100i32) as u32))) as i32);
        l_prof_7 = { let a_ = (({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(200)).wrapping_add(((((c.m.r32((p_ai_1).wrapping_add(4)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) };
        l_code_8 = ((c.m.r32((l_plan_5).wrapping_add(12)) as i32)).wrapping_sub(40i32);
        l_f_9 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetUnk1288(c, a0_) };
        l_act_10 = { let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) };
        if (((((l_code_8) < (0i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        {
            l_i_12 = 0i32;
            'l1: loop {
                if !(((((l_i_12) < (24i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((l_code_8) == (((c.m.r8((sym::D_002EDDC0).wrapping_add(((l_i_12) as u32)))) as i32))) as i32)) != 0) {
                        l_idx_3 = l_i_12;
                    }
                }
                { let o_ = l_i_12; let n_ = (o_).wrapping_add(1); l_i_12 = n_; };
            }
        }
        if (((((l_idx_3) == ((1i32).wrapping_neg())) as i32)) != 0) {
            break 'f 0i32;
        }
        l_chance_11 = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = ((c.m.r8((((l_prof_7).wrapping_add(4)).wrapping_add(336)).wrapping_add(((l_idx_3) as u32)))) as i32); let a2_ = ((c.m.r8((((l_prof_7).wrapping_add(708)).wrapping_add(336)).wrapping_add(((l_idx_3) as u32)))) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) };
        { let v_ = l_roll_6; let a_ = (l_seq_4).wrapping_add(132); c.m.w32(a_, (v_) as u32); };
        { let v_ = l_chance_11; let a_ = (l_seq_4).wrapping_add(136); c.m.w32(a_, (v_) as u32); };
        if (((((l_chance_11) < (l_roll_6)) as i32)) != 0) {
            break 'f 0i32;
        }
        { let v_ = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetUnk974(c, a0_) }; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = (((c.m.r32(p_ai_1) as i32)) ^ (1i32)); F::BtlCharApi_GetUnk974(c, a0_) }; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = (({ let a_ = (((l_act_10).wrapping_add(((8i32) as u32))).wrapping_add(896)).wrapping_add((((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u32)); c.m.r8(a_) }) as i8); let a_ = (fp.wrapping_add(8)).wrapping_add(((0i32) as u32)); c.m.w8(a_, (v_) as u8); };
        { let v_ = (({ let a_ = (((l_act_10).wrapping_add(((8i32) as u32))).wrapping_add(896)).wrapping_add((((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)); c.m.r8(a_) }) as i8); let a_ = (fp.wrapping_add(8)).wrapping_add(((1i32) as u32)); c.m.w8(a_, (v_) as u8); };
        's3: {
            let sv13: i32 = { let a_ = (sym::D_002EDA70).wrapping_add(((((c.m.r32((l_plan_5).wrapping_add(12)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
            match sv13 {
                56i32 => {
                    if ((((!((((l_f_9) & (((128i32) as u64)))) != 0)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    break 's3;
                }
                57i32 => {
                    if ((((!((((l_f_9) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    break 's3;
                }
                58i32 => {
                    if ((((!((((l_f_9) & (((268435456i32) as u64)))) != 0)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    break 's3;
                }
                59i32 => {
                    if ((((!((((l_f_9) & (((4i32) as u64)))) != 0)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    break 's3;
                }
                60i32 => {
                    if ((((!((((l_f_9) & (((16i32) as u64)))) != 0)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    break 's3;
                }
                61i32 => {
                    if ((((!((((l_f_9) & (((536870912i32) as u64)))) != 0)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    break 's3;
                }
                63i32 => {
                    if ((((!((((l_f_9) & (((2147483648u32) as u64)))) != 0)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    break 's3;
                }
                73i32 => {
                    if ((((!((((l_f_9) & (((4294967296i64) as u64)))) != 0)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    break 's3;
                }
                64i32 | 65i32 => {
                    if ((((!((((l_f_9) & (((8i32) as u64)))) != 0)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    break 's3;
                }
                66i32 => {
                    if ((((!((((l_f_9) & (((8589934592i64) as u64)))) != 0)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    break 's3;
                }
                67i32 => {
                    if ((((!((((l_f_9) & (((17179869184i64) as u64)))) != 0)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    break 's3;
                }
                68i32 => {
                    if ((((!((((l_f_9) & (((2i32) as u64)))) != 0)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    break 's3;
                }
                69i32 => {
                    if ((((!((((l_f_9) & (((1i32) as u64)))) != 0)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    break 's3;
                }
                70i32 => {
                    if ((((!((((l_f_9) & (((32i32) as u64)))) != 0)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    break 's3;
                }
                71i32 => {
                    if ((((((((l_f_9) & (((524288i32) as u64)))) != 0) && ((((((((c.m.r8((fp.wrapping_add(8)).wrapping_add(((0i32) as u32))) as i8)) as i32)) == (16i32)) as i32)) != 0)) as i32)) != 0) {
                        break 's3;
                    }
                    break 'f 0i32;
                }
                72i32 => {
                    if ((((!((((l_f_9) & (((64i32) as u64)))) != 0)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    break 's3;
                }
                74i32 => {
                    if ((((!((((l_f_9) & (((68719476736i64) as u64)))) != 0)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    break 's3;
                }
                75i32 => {
                    if ((((!((((l_f_9) & (((34359738368i64) as u64)))) != 0)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    break 's3;
                }
                62i32 => {
                    if ((((((((l_f_9) & (((128i32) as u64)))) != 0) && ((((((((c.m.r8((fp.wrapping_add(8)).wrapping_add(((1i32) as u32))) as i8)) as i32)) == (3i32)) as i32)) != 0)) as i32)) != 0) {
                        break 's3;
                    }
                    break 'f 0i32;
                }
                _ => {
                    break 'f 0i32;
                }
            }
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "AiThCond_ActRateByFlags", &PROBE_N, &[], true); }
    r_
}

pub fn AiThink_RollActRate(c: &mut Ctx, mut p_ai_1: u32, mut p_cond_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThink_RollActRate", &PROBE_N, &[(p_ai_1) as u32, (p_cond_2) as u32], false); }
    let mut l_code_3: i32 = 0;
    let mut l_roll_4: i32 = 0;
    let mut l_prof_5: u32 = 0;
    let mut l_idx_6: i32 = 0;
    let mut l_i_7: i32 = 0;
    let r_ = 'f: {
    l_code_3 = (p_cond_2).wrapping_sub(40i32);
    l_roll_4 = ((F::Rand_Range(c, ((100i32) as u32))) as i32);
    l_prof_5 = { let a_ = (({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(200)).wrapping_add(((((c.m.r32((p_ai_1).wrapping_add(4)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    l_idx_6 = (1i32).wrapping_neg();
    {
        l_i_7 = 0i32;
        'l1: loop {
            if !(((((l_i_7) < (24i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((l_code_3) == (((c.m.r8((sym::D_002EDDC0).wrapping_add(((l_i_7) as u32)))) as i32))) as i32)) != 0) {
                    l_idx_6 = l_i_7;
                }
            }
            { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
        }
    }
    if (((((l_idx_6) == ((1i32).wrapping_neg())) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f (((l_roll_4) < ({ let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = ((c.m.r8((((l_prof_5).wrapping_add(4)).wrapping_add(336)).wrapping_add(((l_idx_6) as u32)))) as i32); let a2_ = ((c.m.r8((((l_prof_5).wrapping_add(708)).wrapping_add(336)).wrapping_add(((l_idx_6) as u32)))) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) })) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "AiThink_RollActRate", &PROBE_N, &[], true); }
    r_
}

pub fn AiThCond_WeightedIfAbility(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThCond_WeightedIfAbility", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let mut l_ab_3: i32 = 0;
    let mut l_fn_4: i32 = 0;
    let r_ = 'f: {
    l_ab_3 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetParamUnk18(c, a0_) };
    l_fn_4 = { let a_ = (sym::D_002EDA70).wrapping_add(((((c.m.r32(((p_ai_1).wrapping_add(744)).wrapping_add(12)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
    if ((((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlSide_IsPoweredUp(c, a0_) }) == (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    's1: {
        let sv5: i32 = l_fn_4;
        match sv5 {
            115i32 => {
                if ((((l_ab_3) & (512i32))) != 0) {
                    break 's1;
                }
                break 'f 0i32;
            }
            114i32 => {
                if ((((l_ab_3) & (16i32))) != 0) {
                    break 's1;
                }
                break 'f 0i32;
            }
            117i32 => {
                if ((((!((((l_ab_3) & (8i32))) != 0)) as i32)) != 0) {
                    break 'f 0i32;
                }
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f F::AiThink_TestWeighted(c, p_ai_1, 0i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "AiThCond_WeightedIfAbility", &PROBE_N, &[], true); }
    r_
}

pub fn AiThCond_Move(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThCond_Move", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    break 'f F::AiThink_TestMove(c, p_ai_1);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "AiThCond_Move", &PROBE_N, &[], true); }
    r_
}

pub fn AiThCond_Skill(c: &mut Ctx, mut p_ai_1: u32, mut p_arg_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThCond_Skill", &PROBE_N, &[(p_ai_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    break 'f F::AiThink_TestSkill(c, p_ai_1);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "AiThCond_Skill", &PROBE_N, &[], true); }
    r_
}

pub fn AiThink_GetGroupClass(c: &mut Ctx, mut p_group_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThink_GetGroupClass", &PROBE_N, &[(p_group_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = p_group_1;
        match sv2 {
            7i32 => {
                break 'f 0i32;
            }
            8i32 => {
                break 'f 1i32;
            }
            9i32 => {
                break 'f 2i32;
            }
            10i32 => {
                break 'f 3i32;
            }
            11i32 => {
                break 'f 4i32;
            }
            12i32 => {
                break 'f 5i32;
            }
            14i32 => {
                break 'f 6i32;
            }
            16i32 => {
                break 'f 7i32;
            }
            17i32 => {
                break 'f 8i32;
            }
            18i32 => {
                break 'f 9i32;
            }
            19i32 => {
                break 'f 10i32;
            }
            20i32 => {
                break 'f 11i32;
            }
            23i32 => {
                break 'f 12i32;
            }
            24i32 => {
                break 'f 13i32;
            }
            _ => {
                break 'f 14i32;
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "AiThink_GetGroupClass", &PROBE_N, &[], true); }
    r_
}

pub fn AiThink_HasMoveKind(c: &mut Ctx, mut p_ai_1: u32, mut p_kind_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThink_HasMoveKind", &PROBE_N, &[(p_ai_1) as u32, (p_kind_2) as u32], false); }
    let mut l_moves_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_p_5: u32 = 0;
    let r_ = 'f: {
    l_moves_3 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetMoveTable(c, a0_) };
    l_p_5 = (l_moves_3).wrapping_add(158);
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((p_kind_2) == ((((c.m.r8({ let o_ = l_p_5; let n_ = (o_).wrapping_add(1); l_p_5 = n_; o_ }) as i8)) as i32))) as i32)) != 0) {
                    break 'f 1i32;
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "AiThink_HasMoveKind", &PROBE_N, &[], true); }
    r_
}

pub fn AiThink_HasSkillKind(c: &mut Ctx, mut p_ai_1: u32, mut p_kind_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThink_HasSkillKind", &PROBE_N, &[(p_ai_1) as u32, (p_kind_2) as u32], false); }
    let mut l_skills_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_p_5: u32 = 0;
    let r_ = 'f: {
    l_skills_3 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetSkillTable(c, a0_) };
    l_p_5 = (l_skills_3).wrapping_add(357);
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (3i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((p_kind_2) == ((((c.m.r8({ let o_ = l_p_5; let n_ = (o_).wrapping_add(1); l_p_5 = n_; o_ }) as i8)) as i32))) as i32)) != 0) {
                    break 'f 1i32;
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "AiThink_HasSkillKind", &PROBE_N, &[], true); }
    r_
}

pub fn AiThink_HasAbility(c: &mut Ctx, mut p_ai_1: u32, mut p_mask_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThink_HasAbility", &PROBE_N, &[(p_ai_1) as u32, (p_mask_2) as u32], false); }
    let r_ = 'f: {
    break 'f ((((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetParamUnk18(c, a0_) }) & (p_mask_2))) != (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "AiThink_HasAbility", &PROBE_N, &[], true); }
    r_
}

pub fn AiThink_IsCodeUsable(c: &mut Ctx, mut p_ai_1: u32, mut p_code_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThink_IsCodeUsable", &PROBE_N, &[(p_ai_1) as u32, (p_code_2) as u32], false); }
    let mut l_r_3: i32 = 0;
    let r_ = 'f: {
    l_r_3 = (({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetParamUnk14(c, a0_) }) & (1i32));
    if (((((p_code_2) == (0i32)) as i32)) != 0) {
        break 'f l_r_3;
    }
    if ((((((((p_code_2).wrapping_sub(52i32)) as u32)) < (((9i32) as u32))) as i32)) != 0) {
        break 'f F::AiThink_HasMoveKind(c, p_ai_1, (p_code_2).wrapping_sub(52i32));
    }
    if ((((((((p_code_2).wrapping_sub(61i32)) as u32)) < (((7i32) as u32))) as i32)) != 0) {
        break 'f F::AiThink_HasSkillKind(c, p_ai_1, (p_code_2).wrapping_sub(61i32));
    }
    if (((((p_code_2) == (73i32)) as i32)) != 0) {
        break 'f F::AiThink_HasSkillKind(c, p_ai_1, 3i32);
    }
    if (((((p_code_2) == (74i32)) as i32)) != 0) {
        break 'f ((F::AiThink_HasSkillKind(c, p_ai_1, 1i32)) | (F::AiThink_HasSkillKind(c, p_ai_1, 2i32)));
    }
    's1: {
        let sv4: i32 = p_code_2;
        match sv4 {
            82i32 => {
                break 'f F::AiThink_HasAbility(c, p_ai_1, 16i32);
            }
            83i32 => {
                break 'f F::AiThink_HasAbility(c, p_ai_1, 512i32);
            }
            84i32 => {
                break 'f F::AiThink_HasAbility(c, p_ai_1, 32i32);
            }
            85i32 => {
                break 'f F::AiThink_HasAbility(c, p_ai_1, 8i32);
            }
            _ => {
            }
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "AiThink_IsCodeUsable", &PROBE_N, &[], true); }
    r_
}

pub fn AiThink_SumPairRates(c: &mut Ctx, mut p_ai_1: u32, mut p_cls_2: u32, mut p_lo_3: u32, mut p_hi_4: u32, mut p_codes_5: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThink_SumPairRates", &PROBE_N, &[(p_ai_1) as u32, (p_cls_2) as u32, (p_lo_3) as u32, (p_hi_4) as u32, (p_codes_5) as u32], false); }
    let mut l_sum_6: i32 = 0;
    let mut l_c_7: i32 = 0;
    let mut l_i_8: i32 = 0;
    let r_ = 'f: {
    l_sum_6 = 0i32;
    {
        l_c_7 = 0i32;
        'l1: loop {
            if !(((((l_c_7) < (8i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((({ let a1_ = ((c.m.r8(((p_codes_5).wrapping_add((p_cls_2).wrapping_mul(((8i32) as u32)))).wrapping_add(((l_c_7) as u32)))) as i32); F::AiThink_IsCodeUsable(c, p_ai_1, a1_) }) != (0i32)) as i32)) != 0) {
                    's3: {
                        let sv9: u32 = p_cls_2;
                        match sv9 {
                            0u32 => {
                                { let r_ = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_3).wrapping_add(((l_c_7) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_4).wrapping_add(((l_c_7) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_6).wrapping_add(r_); l_sum_6 = n_; };
                                break 's3;
                            }
                            1u32 => {
                                { let r_ = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_3).wrapping_add((((l_c_7).wrapping_add(8i32)) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_4).wrapping_add((((l_c_7).wrapping_add(8i32)) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_6).wrapping_add(r_); l_sum_6 = n_; };
                                break 's3;
                            }
                            2u32 => {
                                l_i_8 = (l_c_7).wrapping_add(16i32);
                                { let r_ = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_3).wrapping_add(((l_i_8) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_4).wrapping_add(((l_i_8) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_6).wrapping_add(r_); l_sum_6 = n_; };
                                break 's3;
                            }
                            3u32 => {
                                l_i_8 = (l_c_7).wrapping_add(16i32);
                                { let r_ = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_3).wrapping_add((((l_i_8).wrapping_add(8i32)) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_4).wrapping_add((((l_i_8).wrapping_add(8i32)) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_6).wrapping_add(r_); l_sum_6 = n_; };
                                break 's3;
                            }
                            4u32 => {
                                l_i_8 = (l_c_7).wrapping_add(32i32);
                                { let r_ = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_3).wrapping_add(((l_i_8) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_4).wrapping_add(((l_i_8) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_6).wrapping_add(r_); l_sum_6 = n_; };
                                break 's3;
                            }
                            5u32 => {
                                l_i_8 = (l_c_7).wrapping_add(32i32);
                                { let r_ = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_3).wrapping_add((((l_i_8).wrapping_add(8i32)) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_4).wrapping_add((((l_i_8).wrapping_add(8i32)) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_6).wrapping_add(r_); l_sum_6 = n_; };
                                break 's3;
                            }
                            6u32 => {
                                l_i_8 = (l_c_7).wrapping_add(48i32);
                                { let r_ = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_3).wrapping_add(((l_i_8) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_4).wrapping_add(((l_i_8) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_6).wrapping_add(r_); l_sum_6 = n_; };
                                break 's3;
                            }
                            7u32 => {
                                l_i_8 = (l_c_7).wrapping_add(48i32);
                                { let r_ = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_3).wrapping_add((((l_i_8).wrapping_add(8i32)) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_4).wrapping_add((((l_i_8).wrapping_add(8i32)) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_6).wrapping_add(r_); l_sum_6 = n_; };
                                break 's3;
                            }
                            8u32 => {
                                l_i_8 = (l_c_7).wrapping_add(64i32);
                                { let r_ = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_3).wrapping_add(((l_i_8) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_4).wrapping_add(((l_i_8) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_6).wrapping_add(r_); l_sum_6 = n_; };
                                break 's3;
                            }
                            9u32 => {
                                l_i_8 = (l_c_7).wrapping_add(64i32);
                                { let r_ = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_3).wrapping_add((((l_i_8).wrapping_add(8i32)) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_4).wrapping_add((((l_i_8).wrapping_add(8i32)) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_6).wrapping_add(r_); l_sum_6 = n_; };
                                break 's3;
                            }
                            10u32 => {
                                l_i_8 = (l_c_7).wrapping_add(80i32);
                                { let r_ = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_3).wrapping_add(((l_i_8) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_4).wrapping_add(((l_i_8) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_6).wrapping_add(r_); l_sum_6 = n_; };
                                break 's3;
                            }
                            11u32 => {
                                l_i_8 = (l_c_7).wrapping_add(80i32);
                                { let r_ = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_3).wrapping_add((((l_i_8).wrapping_add(8i32)) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_4).wrapping_add((((l_i_8).wrapping_add(8i32)) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_6).wrapping_add(r_); l_sum_6 = n_; };
                                break 's3;
                            }
                            12u32 => {
                                l_i_8 = (l_c_7).wrapping_add(96i32);
                                { let r_ = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_3).wrapping_add(((l_i_8) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_4).wrapping_add(((l_i_8) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_6).wrapping_add(r_); l_sum_6 = n_; };
                                break 's3;
                            }
                            13u32 => {
                                l_i_8 = (l_c_7).wrapping_add(96i32);
                                { let r_ = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_3).wrapping_add((((l_i_8).wrapping_add(8i32)) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_4).wrapping_add((((l_i_8).wrapping_add(8i32)) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_6).wrapping_add(r_); l_sum_6 = n_; };
                                break 's3;
                            }
                            _ => {
                            }
                        }
                    }
                }
            }
            { let o_ = l_c_7; let n_ = (o_).wrapping_add(1); l_c_7 = n_; };
        }
    }
    break 'f l_sum_6;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "AiThink_SumPairRates", &PROBE_N, &[], true); }
    r_
}

pub fn AiThink_SumQuadRates(c: &mut Ctx, mut p_ai_1: u32, mut p_cls_2: u32, mut p_lo_3: u32, mut p_hi_4: u32, mut p_codes_5: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThink_SumQuadRates", &PROBE_N, &[(p_ai_1) as u32, (p_cls_2) as u32, (p_lo_3) as u32, (p_hi_4) as u32, (p_codes_5) as u32], false); }
    let mut l_sum_6: i32 = 0;
    let mut l_c_7: i32 = 0;
    let mut l_i_8: i32 = 0;
    let r_ = 'f: {
    l_sum_6 = 0i32;
    {
        l_c_7 = 0i32;
        'l1: loop {
            if !(((((l_c_7) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((({ let a1_ = ((c.m.r8(((p_codes_5).wrapping_add((p_cls_2).wrapping_mul(((4i32) as u32)))).wrapping_add(((l_c_7) as u32)))) as i32); F::AiThink_IsCodeUsable(c, p_ai_1, a1_) }) != (0i32)) as i32)) != 0) {
                    's3: {
                        let sv9: u32 = p_cls_2;
                        match sv9 {
                            0u32 => {
                                { let r_ = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_3).wrapping_add(((l_c_7) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_4).wrapping_add(((l_c_7) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_6).wrapping_add(r_); l_sum_6 = n_; };
                                break 's3;
                            }
                            1u32 => {
                                { let r_ = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_3).wrapping_add((((l_c_7).wrapping_add(4i32)) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_4).wrapping_add((((l_c_7).wrapping_add(4i32)) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_6).wrapping_add(r_); l_sum_6 = n_; };
                                break 's3;
                            }
                            2u32 => {
                                { let r_ = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_3).wrapping_add((((l_c_7).wrapping_add(8i32)) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_4).wrapping_add((((l_c_7).wrapping_add(8i32)) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_6).wrapping_add(r_); l_sum_6 = n_; };
                                break 's3;
                            }
                            3u32 => {
                                { let r_ = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_3).wrapping_add((((l_c_7).wrapping_add(12i32)) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_4).wrapping_add((((l_c_7).wrapping_add(12i32)) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_6).wrapping_add(r_); l_sum_6 = n_; };
                                break 's3;
                            }
                            4u32 => {
                                l_i_8 = (l_c_7).wrapping_add(16i32);
                                { let r_ = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_3).wrapping_add(((l_i_8) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_4).wrapping_add(((l_i_8) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_6).wrapping_add(r_); l_sum_6 = n_; };
                                break 's3;
                            }
                            5u32 => {
                                l_i_8 = (l_c_7).wrapping_add(16i32);
                                { let r_ = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_3).wrapping_add((((l_i_8).wrapping_add(4i32)) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_4).wrapping_add((((l_i_8).wrapping_add(4i32)) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_6).wrapping_add(r_); l_sum_6 = n_; };
                                break 's3;
                            }
                            6u32 => {
                                l_i_8 = (l_c_7).wrapping_add(16i32);
                                { let r_ = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_3).wrapping_add((((l_i_8).wrapping_add(8i32)) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_4).wrapping_add((((l_i_8).wrapping_add(8i32)) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_6).wrapping_add(r_); l_sum_6 = n_; };
                                break 's3;
                            }
                            7u32 => {
                                l_i_8 = (l_c_7).wrapping_add(16i32);
                                { let r_ = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_3).wrapping_add((((l_i_8).wrapping_add(12i32)) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_4).wrapping_add((((l_i_8).wrapping_add(12i32)) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_6).wrapping_add(r_); l_sum_6 = n_; };
                                break 's3;
                            }
                            8u32 => {
                                l_i_8 = (l_c_7).wrapping_add(32i32);
                                { let r_ = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_3).wrapping_add(((l_i_8) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_4).wrapping_add(((l_i_8) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_6).wrapping_add(r_); l_sum_6 = n_; };
                                break 's3;
                            }
                            9u32 => {
                                l_i_8 = (l_c_7).wrapping_add(32i32);
                                { let r_ = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_3).wrapping_add((((l_i_8).wrapping_add(4i32)) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_4).wrapping_add((((l_i_8).wrapping_add(4i32)) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_6).wrapping_add(r_); l_sum_6 = n_; };
                                break 's3;
                            }
                            10u32 => {
                                l_i_8 = (l_c_7).wrapping_add(32i32);
                                { let r_ = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_3).wrapping_add((((l_i_8).wrapping_add(8i32)) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_4).wrapping_add((((l_i_8).wrapping_add(8i32)) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_6).wrapping_add(r_); l_sum_6 = n_; };
                                break 's3;
                            }
                            11u32 => {
                                l_i_8 = (l_c_7).wrapping_add(32i32);
                                { let r_ = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_3).wrapping_add((((l_i_8).wrapping_add(12i32)) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_4).wrapping_add((((l_i_8).wrapping_add(12i32)) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_6).wrapping_add(r_); l_sum_6 = n_; };
                                break 's3;
                            }
                            12u32 => {
                                l_i_8 = (l_c_7).wrapping_add(48i32);
                                { let r_ = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_3).wrapping_add(((l_i_8) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_4).wrapping_add(((l_i_8) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_6).wrapping_add(r_); l_sum_6 = n_; };
                                break 's3;
                            }
                            13u32 => {
                                l_i_8 = (l_c_7).wrapping_add(48i32);
                                { let r_ = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((p_lo_3).wrapping_add((((l_i_8).wrapping_add(4i32)) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((p_hi_4).wrapping_add((((l_i_8).wrapping_add(4i32)) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_6).wrapping_add(r_); l_sum_6 = n_; };
                                break 's3;
                            }
                            _ => {
                            }
                        }
                    }
                }
            }
            { let o_ = l_c_7; let n_ = (o_).wrapping_add(1); l_c_7 = n_; };
        }
    }
    break 'f l_sum_6;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "AiThink_SumQuadRates", &PROBE_N, &[], true); }
    r_
}

pub fn AiThink_SumSubRates(c: &mut Ctx, mut p_ai_1: u32, mut p_sub_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThink_SumSubRates", &PROBE_N, &[(p_ai_1) as u32, (p_sub_2) as u32], false); }
    let mut l_sum_3: i32 = 0;
    let mut l_c_4: i32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_codes_6: u32 = 0;
    let mut l_prof_7: u32 = 0;
    let mut l_lo0_8: u32 = 0;
    let mut l_hi0_9: u32 = 0;
    let mut l_lo1_10: u32 = 0;
    let mut l_hi1_11: u32 = 0;
    let r_ = 'f: {
    l_sum_3 = 0i32;
    l_codes_6 = (sym::D_002EDEE8).wrapping_add((((p_sub_2).wrapping_mul(8i32)) as u32));
    l_prof_7 = { let a_ = (({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(200)).wrapping_add(((((c.m.r32((p_ai_1).wrapping_add(4)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    l_lo0_8 = (l_prof_7).wrapping_add(((648i32) as u32));
    l_hi0_9 = (l_prof_7).wrapping_add(((1352i32) as u32));
    l_lo1_10 = (l_prof_7).wrapping_add(((656i32) as u32));
    l_hi1_11 = (l_prof_7).wrapping_add(((1360i32) as u32));
    l_i_5 = 16i32;
    {
        l_c_4 = 0i32;
        'l1: loop {
            if !(((((l_c_4) < (8i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((({ let a1_ = ((c.m.r8(l_codes_6)) as i32); F::AiThink_IsCodeUsable(c, p_ai_1, a1_) }) != (0i32)) as i32)) != 0) {
                    's3: {
                        let sv12: i32 = p_sub_2;
                        match sv12 {
                            0i32 => {
                                { let r_ = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((l_lo0_8).wrapping_add(((l_c_4) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((l_hi0_9).wrapping_add(((l_c_4) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_3).wrapping_add(r_); l_sum_3 = n_; };
                                break 's3;
                            }
                            1i32 => {
                                { let r_ = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((l_lo1_10).wrapping_add(((l_c_4) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((l_hi1_11).wrapping_add(((l_c_4) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_3).wrapping_add(r_); l_sum_3 = n_; };
                                break 's3;
                            }
                            2i32 => {
                                { let r_ = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((l_lo0_8).wrapping_add(((l_i_5) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((l_hi0_9).wrapping_add(((l_i_5) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_3).wrapping_add(r_); l_sum_3 = n_; };
                                break 's3;
                            }
                            3i32 => {
                                { let r_ = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((l_lo1_10).wrapping_add(((l_i_5) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((l_hi1_11).wrapping_add(((l_i_5) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_3).wrapping_add(r_); l_sum_3 = n_; };
                                break 's3;
                            }
                            _ => {
                            }
                        }
                    }
                }
            }
            { { { let o_ = l_c_4; let n_ = (o_).wrapping_add(1); l_c_4 = n_; }; { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; }; }; { let o_ = l_codes_6; let n_ = (o_).wrapping_add(1); l_codes_6 = n_; }; };
        }
    }
    break 'f l_sum_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "AiThink_SumSubRates", &PROBE_N, &[], true); }
    r_
}

pub fn AiThink_BuildTotals(c: &mut Ctx, mut p_ai_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThink_BuildTotals", &PROBE_N, &[(p_ai_1) as u32], false); }
    let mut l_plan_2: u32 = 0;
    let mut l_prof_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_j_5: i32 = 0;
    let r_ = 'f: {
    l_plan_2 = (p_ai_1).wrapping_add(744);
    l_prof_3 = { let a_ = (({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(200)).wrapping_add(((((c.m.r32((p_ai_1).wrapping_add(4)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (14i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = ((F::AiThink_SumPairRates(c, p_ai_1, ((l_i_4) as u32), (l_prof_3).wrapping_add(((116i32) as u32)), (l_prof_3).wrapping_add(((820i32) as u32)), sym::D_002EDCE0)) as i8); let a_ = (((l_plan_2).wrapping_add(105)).wrapping_add((((0i32) as u32)).wrapping_mul(14))).wrapping_add(((l_i_4) as u32)); c.m.w8(a_, (v_) as u8); };
                { let v_ = ((F::AiThink_SumPairRates(c, p_ai_1, ((l_i_4) as u32), (l_prof_3).wrapping_add(((228i32) as u32)), (l_prof_3).wrapping_add(((932i32) as u32)), sym::D_002EDD50)) as i8); let a_ = (((l_plan_2).wrapping_add(105)).wrapping_add((((1i32) as u32)).wrapping_mul(14))).wrapping_add(((l_i_4) as u32)); c.m.w8(a_, (v_) as u8); };
                { let v_ = ((F::AiThink_SumQuadRates(c, p_ai_1, ((l_i_4) as u32), (l_prof_3).wrapping_add(((376i32) as u32)), (l_prof_3).wrapping_add(((1080i32) as u32)), sym::D_002EDDD8)) as i8); let a_ = (((l_plan_2).wrapping_add(105)).wrapping_add((((2i32) as u32)).wrapping_mul(14))).wrapping_add(((l_i_4) as u32)); c.m.w8(a_, (v_) as u8); };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    {
        l_j_5 = 0i32;
        'l3: loop {
            if !(((((l_j_5) < (4i32)) as i32)) != 0) { break 'l3; }
            'c4: {
                { let v_ = ((F::AiThink_SumSubRates(c, p_ai_1, l_j_5)) as i8); let a_ = (((l_plan_2).wrapping_add(105)).wrapping_add((((3i32) as u32)).wrapping_mul(14))).wrapping_add(((l_j_5) as u32)); c.m.w8(a_, (v_) as u8); };
            }
            { let o_ = l_j_5; let n_ = (o_).wrapping_add(1); l_j_5 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "AiThink_BuildTotals", &PROBE_N, &[], true); }
    r_
}

pub fn AiThink_GetRollRange(c: &mut Ctx, mut p_ai_1: u32, mut p_tbl_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThink_GetRollRange", &PROBE_N, &[(p_ai_1) as u32, (p_tbl_2) as u32], false); }
    let mut l_plan_3: u32 = 0;
    let mut l_idx_4: i32 = 0;
    let mut l_total_5: i8 = 0;
    let mut l_dist_6: i32 = 0;
    let mut l_near_7: i32 = 0;
    let r_ = 'f: {
    l_plan_3 = (p_ai_1).wrapping_add(744);
    if (((((p_tbl_2) != (3i32)) as i32)) != 0) {
        l_idx_4 = (c.m.r32(l_plan_3) as i32);
    } else {
        l_idx_4 = (c.m.r32((l_plan_3).wrapping_add(84)) as i32);
    }
    l_total_5 = (c.m.r8((((l_plan_3).wrapping_add(105)).wrapping_add((((p_tbl_2) as u32)).wrapping_mul(14))).wrapping_add(((l_idx_4) as u32))) as i8);
    l_dist_6 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetHp(c, a0_) };
    l_near_7 = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (({ let a_ = ({ let a_ = (({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(200)).wrapping_add(((((c.m.r32((p_ai_1).wrapping_add(4)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((686i32) as u32)); c.m.r8(a_) }) as i32); let a2_ = (({ let a_ = ({ let a_ = (({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(200)).wrapping_add(((((c.m.r32((p_ai_1).wrapping_add(4)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((1390i32) as u32)); c.m.r8(a_) }) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) };
    if (((((((l_total_5) as i32)) == (0i32)) as i32)) != 0) {
        break 'f 100i32;
    }
    if (((((l_near_7) == (0i32)) as i32)) != 0) {
        break 'f ((l_total_5) as i32);
    }
    if (((((p_tbl_2) != (1i32)) as i32)) != 0) {
        break 'f ((l_total_5) as i32);
    }
    's1: {
        let sv8: u32 = (((c.m.r32(l_plan_3) as i32)) as u32);
        match sv8 {
            8u32 | 9u32 | 13u32 => {
                break 's1;
            }
            _ => {
                break 'f ((l_total_5) as i32);
            }
        }
    }
    if (((((l_dist_6) < (10001i32)) as i32)) != 0) {
        break 'f (((l_total_5) as i32)).wrapping_add(l_near_7);
    }
    break 'f ((l_total_5) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "AiThink_GetRollRange", &PROBE_N, &[], true); }
    r_
}

pub fn AiThink_GetCond38Range(c: &mut Ctx, mut p_ai_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThink_GetCond38Range", &PROBE_N, &[(p_ai_1) as u32], false); }
    let mut l_k_2: F32 = F32(0);
    let mut l_plan_3: u32 = 0;
    let mut l_rate_4: F32 = F32(0);
    let r_ = 'f: {
    l_k_2 = F::BtlAi_GetLowGaugeFactor(c, p_ai_1);
    l_plan_3 = (p_ai_1).wrapping_add(744);
    if (((rt::feq(l_k_2, F32(0x3f800000)) as i32)) != 0) {
        break 'f 100i32;
    }
    l_rate_4 = rt::i2f(({ let a_ = (((p_ai_1).wrapping_add(744)).wrapping_add(88)).wrapping_add((((c.m.r32(l_plan_3) as i32)) as u32)); (c.m.r8(a_) as i8) }) as i32);
    break 'f (rt::f2i(rt::fsub(rt::fmul(l_rate_4, l_k_2), l_rate_4))).wrapping_add(100i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "AiThink_GetCond38Range", &PROBE_N, &[], true); }
    r_
}

pub fn AiThink_RollGroupGate(c: &mut Ctx, mut p_ai_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThink_RollGroupGate", &PROBE_N, &[(p_ai_1) as u32], false); }
    let mut l_plan_2: u32 = 0;
    let mut l_sum_3: i32 = 0;
    let mut l_c_4: i32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_sub_6: i32 = 0;
    let mut l_roll_7: i32 = 0;
    let mut l_prof_8: u32 = 0;
    let mut l_lo0_9: u32 = 0;
    let mut l_hi0_10: u32 = 0;
    let mut l_lo1_11: u32 = 0;
    let mut l_hi1_12: u32 = 0;
    let r_ = 'f: {
    l_plan_2 = (p_ai_1).wrapping_add(744);
    l_sum_3 = 0i32;
    l_c_4 = 0i32;
    l_roll_7 = ((F::Rand_Range(c, ((100i32) as u32))) as i32);
    l_prof_8 = { let a_ = (({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(200)).wrapping_add(((((c.m.r32((p_ai_1).wrapping_add(4)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    l_lo0_9 = (l_prof_8).wrapping_add(((648i32) as u32));
    l_hi0_10 = (l_prof_8).wrapping_add(((1352i32) as u32));
    l_lo1_11 = (l_prof_8).wrapping_add(((656i32) as u32));
    l_hi1_12 = (l_prof_8).wrapping_add(((1360i32) as u32));
    {
        'l1: loop {
            if !(((((l_c_4) < (8i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_sub_6 = (c.m.r32((l_plan_2).wrapping_add(84)) as i32);
                's3: {
                    let sv13: i32 = l_sub_6;
                    match sv13 {
                        0i32 => {
                            { let r_ = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((l_lo0_9).wrapping_add(((l_c_4) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((l_hi0_10).wrapping_add(((l_c_4) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_3).wrapping_add(r_); l_sum_3 = n_; };
                            break 's3;
                        }
                        1i32 => {
                            { let r_ = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((l_lo1_11).wrapping_add(((l_c_4) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((l_hi1_12).wrapping_add(((l_c_4) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_3).wrapping_add(r_); l_sum_3 = n_; };
                            break 's3;
                        }
                        3i32 => {
                            l_i_5 = (l_c_4).wrapping_add(16i32);
                            { let r_ = { let a0_ = (c.m.r32((p_ai_1).wrapping_add(8)) as i32); let a1_ = (((c.m.r8((l_lo1_11).wrapping_add(((l_i_5) as u32))) as i8)) as i32); let a2_ = (((c.m.r8((l_hi1_12).wrapping_add(((l_i_5) as u32))) as i8)) as i32); F::BtlAi_ScaleByLevel(c, a0_, a1_, a2_) }; let n_ = (l_sum_3).wrapping_add(r_); l_sum_3 = n_; };
                            break 's3;
                        }
                        _ => {
                            break 'f 0i32;
                        }
                    }
                }
            }
            { let o_ = l_c_4; let n_ = (o_).wrapping_add(1); l_c_4 = n_; };
        }
    }
    if (((((l_roll_7) < (l_sum_3)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_sub_6 = (c.m.r32((l_plan_2).wrapping_add(84)) as i32);
    's4: {
        let sv14: i32 = l_sub_6;
        match sv14 {
            0i32 => {
                F::BtlAi_NoteOpponent(c, p_ai_1, 512i32);
                break 's4;
            }
            1i32 => {
                F::BtlAi_NoteOpponent(c, p_ai_1, 2048i32);
                break 's4;
            }
            3i32 => {
                F::BtlAi_NoteOpponent(c, p_ai_1, 4096i32);
                break 's4;
            }
            _ => {
            }
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "AiThink_RollGroupGate", &PROBE_N, &[], true); }
    r_
}

pub fn AiThink_EvalRules(c: &mut Ctx, mut p_ai_1: u32, mut p_list_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThink_EvalRules", &PROBE_N, &[(p_ai_1) as u32, (p_list_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_seq_3: u32 = 0;
    let mut l_n_4: i32 = 0;
    let mut l_lastGroup_5: i32 = 0;
    let mut l_st_6: u32 = 0;
    let mut l_plan_7: u32 = 0;
    let mut l_skipGroup_8: i32 = 0;
    let mut l_rule_9: u32 = 0;
    let mut l_r_10: u32 = 0;
    let mut l_i_11: i32 = 0;
    let mut l_pass_12: i32 = 0;
    let mut l_range_13: i32 = 0;
    let mut l_cls_14: i32 = 0;
    let mut l_next_15: i32 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
        'fl1: loop {
            match bb_ {
                0 => {
                    l_seq_3 = (p_ai_1).wrapping_add(40);
                    l_lastGroup_5 = (1i32).wrapping_neg();
                    l_st_6 = (p_ai_1).wrapping_add(704);
                    l_plan_7 = (p_ai_1).wrapping_add(744);
                    l_skipGroup_8 = (1i32).wrapping_neg();
                    l_n_4 = 0i32;
                    bb_ = 1; continue 'fl1;
                }
                1 => {
                    if (((((l_n_4) < ((((c.m.r16(p_list_2) as i16)) as i32))) as i32)) != 0) { bb_ = 2; continue 'fl1; } else { bb_ = 4; continue 'fl1; }
                }
                2 => {
                    { let v_ = ((l_n_4) as u8); let a_ = (l_plan_7).wrapping_add(168); c.m.w8(a_, v_); };
                    l_rule_9 = (c.m.r32((p_list_2).wrapping_add(4))).wrapping_add((((l_n_4) as u32)).wrapping_mul(28));
                    if (((((l_skipGroup_8) == ((((c.m.r16(l_rule_9) as i16)) as i32))) as i32)) != 0) { bb_ = 5; continue 'fl1; } else { bb_ = 6; continue 'fl1; }
                }
                3 => {
                    { let o_ = l_n_4; let n_ = (o_).wrapping_add(1); l_n_4 = n_; };
                    bb_ = 1; continue 'fl1;
                }
                4 => {
                    break 'fl1;
                }
                5 => {
                    bb_ = 3; continue 'fl1;
                }
                6 => {
                    if ((((!(((((c.m.r64(l_st_6)).wrapping_shr(((((c.m.r16(l_rule_9) as i16)) as i32)) as u32)) & (((1i32) as u64)))) != 0)) as i32)) != 0) { bb_ = 8; continue 'fl1; } else { bb_ = 9; continue 'fl1; }
                }
                7 => {
                    bb_ = 6; continue 'fl1;
                }
                8 => {
                    bb_ = 3; continue 'fl1;
                }
                9 => {
                    { let v_ = { let a0_ = (((c.m.r16(l_rule_9) as i16)) as i32); F::AiThink_GetGroupClass(c, a0_) }; let a_ = l_plan_7; c.m.w32(a_, (v_) as u32); };
                    if ((((((((c.m.r16(l_rule_9) as i16)) as i32)) == (29i32)) as i32)) != 0) { bb_ = 11; continue 'fl1; } else { bb_ = 13; continue 'fl1; }
                }
                10 => {
                    bb_ = 9; continue 'fl1;
                }
                11 => {
                    { let v_ = 0i32; let a_ = (l_plan_7).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
                    bb_ = 12; continue 'fl1;
                }
                12 => {
                    if (((((l_lastGroup_5) != ((((c.m.r16(l_rule_9) as i16)) as i32))) as i32)) != 0) { bb_ = 22; continue 'fl1; } else { bb_ = 23; continue 'fl1; }
                }
                13 => {
                    if ((((((((c.m.r16(l_rule_9) as i16)) as i32)) == (36i32)) as i32)) != 0) { bb_ = 14; continue 'fl1; } else { bb_ = 16; continue 'fl1; }
                }
                14 => {
                    { let v_ = 1i32; let a_ = (l_plan_7).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
                    bb_ = 15; continue 'fl1;
                }
                15 => {
                    bb_ = 12; continue 'fl1;
                }
                16 => {
                    if ((((((((c.m.r16(l_rule_9) as i16)) as i32)) == (38i32)) as i32)) != 0) { bb_ = 17; continue 'fl1; } else { bb_ = 19; continue 'fl1; }
                }
                17 => {
                    { let v_ = 2i32; let a_ = (l_plan_7).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
                    bb_ = 18; continue 'fl1;
                }
                18 => {
                    bb_ = 15; continue 'fl1;
                }
                19 => {
                    if ((((((((c.m.r16(l_rule_9) as i16)) as i32)) == (37i32)) as i32)) != 0) { bb_ = 20; continue 'fl1; } else { bb_ = 21; continue 'fl1; }
                }
                20 => {
                    { let v_ = 3i32; let a_ = (l_plan_7).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
                    bb_ = 21; continue 'fl1;
                }
                21 => {
                    bb_ = 18; continue 'fl1;
                }
                22 => {
                    if ((((((((((((((((c.m.r16(l_rule_9) as i16)) as i32)) == (29i32)) as i32)) != 0) || ((((((((c.m.r16(l_rule_9) as i16)) as i32)) == (36i32)) as i32)) != 0)) as i32)) != 0) || ((((((((c.m.r16(l_rule_9) as i16)) as i32)) == (37i32)) as i32)) != 0)) as i32)) != 0) { bb_ = 24; continue 'fl1; } else { bb_ = 25; continue 'fl1; }
                }
                23 => {
                    l_lastGroup_5 = (((c.m.r16(l_rule_9) as i16)) as i32);
                    l_pass_12 = 0i32;
                    l_i_11 = 0i32;
                    bb_ = 44; continue 'fl1;
                }
                24 => {
                    if (((((F::AiThink_RollGroupGate(c, p_ai_1)) != (0i32)) as i32)) != 0) { bb_ = 26; continue 'fl1; } else { bb_ = 27; continue 'fl1; }
                }
                25 => {
                    l_i_11 = 0i32;
                    bb_ = 29; continue 'fl1;
                }
                26 => {
                    l_skipGroup_8 = (((c.m.r16(l_rule_9) as i16)) as i32);
                    bb_ = 3; continue 'fl1;
                }
                27 => {
                    bb_ = 25; continue 'fl1;
                }
                28 => {
                    bb_ = 27; continue 'fl1;
                }
                29 => {
                    if (((((l_i_11) < (8i32)) as i32)) != 0) { bb_ = 30; continue 'fl1; } else { bb_ = 32; continue 'fl1; }
                }
                30 => {
                    l_range_13 = 100i32;
                    { let v_ = 0i32; let a_ = (fp).wrapping_add((((l_i_11) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    if (((((((((c.m.r32((l_seq_3).wrapping_add(104)) as i32)).wrapping_sub(1i32)) as u32)) < (((3i32) as u32))) as i32)) != 0) { bb_ = 33; continue 'fl1; } else { bb_ = 35; continue 'fl1; }
                }
                31 => {
                    { let o_ = l_i_11; let n_ = (o_).wrapping_add(1); l_i_11 = n_; };
                    bb_ = 29; continue 'fl1;
                }
                32 => {
                    bb_ = 23; continue 'fl1;
                }
                33 => {
                    l_range_13 = { let a1_ = ((c.m.r32((l_seq_3).wrapping_add(104)) as i32)).wrapping_sub(1i32); F::AiThink_GetRollRange(c, p_ai_1, a1_) };
                    bb_ = 34; continue 'fl1;
                }
                34 => {
                    if ((((({ let a_ = (sym::D_002EDA70).wrapping_add((((c.m.r8(((l_rule_9).wrapping_add(4)).wrapping_add(((0i32) as u32)))) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (38i32)) as i32)) != 0) { bb_ = 42; continue 'fl1; } else { bb_ = 43; continue 'fl1; }
                }
                35 => {
                    if ((((((c.m.r32((l_seq_3).wrapping_add(104)) as i32)) == (7i32)) as i32)) != 0) { bb_ = 36; continue 'fl1; } else { bb_ = 37; continue 'fl1; }
                }
                36 => {
                    if ((((((c.m.r32((l_plan_7).wrapping_add(84)) as i32)) >= (0i32)) as i32)) != 0) { bb_ = 38; continue 'fl1; } else { bb_ = 39; continue 'fl1; }
                }
                37 => {
                    bb_ = 34; continue 'fl1;
                }
                38 => {
                    if ((((((c.m.r32((l_plan_7).wrapping_add(84)) as i32)) < (4i32)) as i32)) != 0) { bb_ = 40; continue 'fl1; } else { bb_ = 41; continue 'fl1; }
                }
                39 => {
                    bb_ = 37; continue 'fl1;
                }
                40 => {
                    l_range_13 = F::AiThink_GetRollRange(c, p_ai_1, 3i32);
                    bb_ = 41; continue 'fl1;
                }
                41 => {
                    bb_ = 39; continue 'fl1;
                }
                42 => {
                    l_range_13 = F::AiThink_GetCond38Range(c, p_ai_1);
                    bb_ = 43; continue 'fl1;
                }
                43 => {
                    { let v_ = ((l_range_13) as u8); let a_ = (l_plan_7).wrapping_add(171); c.m.w8(a_, v_); };
                    l_r_10 = ((l_plan_7).wrapping_add(20)).wrapping_add((((l_i_11) as u32)).wrapping_mul(8));
                    { let v_ = ((F::Rand_Range(c, ((l_range_13) as u32))) as i32); let a_ = l_r_10; c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (l_r_10).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    bb_ = 31; continue 'fl1;
                }
                44 => {
                    if (((((l_i_11) < (8i32)) as i32)) != 0) { bb_ = 45; continue 'fl1; } else { bb_ = 47; continue 'fl1; }
                }
                45 => {
                    if (((((((c.m.r8(((l_rule_9).wrapping_add(4)).wrapping_add(((l_i_11) as u32)))) as i32)) == (254i32)) as i32)) != 0) { bb_ = 48; continue 'fl1; } else { bb_ = 49; continue 'fl1; }
                }
                46 => {
                    { let o_ = l_i_11; let n_ = (o_).wrapping_add(1); l_i_11 = n_; };
                    bb_ = 44; continue 'fl1;
                }
                47 => {
                    if (((((l_pass_12) == (0i32)) as i32)) != 0) { bb_ = 61; continue 'fl1; } else { bb_ = 62; continue 'fl1; }
                }
                48 => {
                    l_pass_12 = (c.m.r32((fp).wrapping_add((((l_i_11) as u32)).wrapping_mul(4))) as i32);
                    if (((((l_pass_12) == (0i32)) as i32)) != 0) { bb_ = 50; continue 'fl1; } else { bb_ = 51; continue 'fl1; }
                }
                49 => {
                    if (((((((c.m.r8(((l_rule_9).wrapping_add(4)).wrapping_add(((l_i_11) as u32)))) as i32)) == (255i32)) as i32)) != 0) { bb_ = 54; continue 'fl1; } else { bb_ = 55; continue 'fl1; }
                }
                50 => {
                    bb_ = 47; continue 'fl1;
                }
                51 => {
                    bb_ = 46; continue 'fl1;
                }
                52 => {
                    bb_ = 51; continue 'fl1;
                }
                53 => {
                    bb_ = 49; continue 'fl1;
                }
                54 => {
                    bb_ = 47; continue 'fl1;
                }
                55 => {
                    l_pass_12 = ((c.m.r8(((l_rule_9).wrapping_add(4)).wrapping_add(((l_i_11) as u32)))) as i32);
                    { let v_ = l_i_11; let a_ = (l_plan_7).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                    { let v_ = l_pass_12; let a_ = (l_plan_7).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    l_pass_12 = { let fp_ = { let a_ = (sym::D_002C4768).wrapping_add(((((c.m.r32((sym::D_002EDA70).wrapping_add((((l_pass_12) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a1_ = c.m.r8(((l_rule_9).wrapping_add(12)).wrapping_add(((l_i_11) as u32))); ((D::call(c, fp_, &[((p_ai_1) as u64), ((a1_) as u64)])) as i32) };
                    { let v_ = l_pass_12; let a_ = (fp).wrapping_add((((l_i_11) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    if (((((l_pass_12) == (0i32)) as i32)) != 0) { bb_ = 57; continue 'fl1; } else { bb_ = 58; continue 'fl1; }
                }
                56 => {
                    bb_ = 55; continue 'fl1;
                }
                57 => {
                    bb_ = 59; continue 'fl1;
                }
                58 => {
                    bb_ = 46; continue 'fl1;
                }
                59 => {
                    bb_ = 3; continue 'fl1;
                }
                60 => {
                    bb_ = 58; continue 'fl1;
                }
                61 => {
                    bb_ = 3; continue 'fl1;
                }
                62 => {
                    if (((((((c.m.r8((l_rule_9).wrapping_add(20))) as i32)) == (0i32)) as i32)) != 0) { bb_ = 64; continue 'fl1; } else { bb_ = 65; continue 'fl1; }
                }
                63 => {
                    bb_ = 62; continue 'fl1;
                }
                64 => {
                    break 'f;
                }
                65 => {
                    if (((((((c.m.r8((l_rule_9).wrapping_add(20))) as i32)) == (4i32)) as i32)) != 0) { bb_ = 67; continue 'fl1; } else { bb_ = 68; continue 'fl1; }
                }
                66 => {
                    bb_ = 65; continue 'fl1;
                }
                67 => {
                    l_cls_14 = (c.m.r32(l_plan_7) as i32);
                    l_next_15 = (((c.m.r8((l_rule_9).wrapping_add(24))) as i32)).wrapping_sub(127i32);
                    { let v_ = l_next_15; let a_ = (l_plan_7).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (((c.m.r16(l_rule_9) as i16)) as u8); let a_ = (l_plan_7).wrapping_add(170); c.m.w8(a_, v_); };
                    if (((((l_cls_14) == (14i32)) as i32)) != 0) { bb_ = 69; continue 'fl1; } else { bb_ = 70; continue 'fl1; }
                }
                68 => {
                    if ((((((((c.m.r16(l_rule_9) as i16)) as i32)) == (36i32)) as i32)) != 0) { bb_ = 73; continue 'fl1; } else { bb_ = 75; continue 'fl1; }
                }
                69 => {
                    break 'f;
                }
                70 => {
                    { let a_ = (((l_plan_7).wrapping_add(228)).wrapping_add((((l_cls_14) as u32)).wrapping_mul(24))).wrapping_add((((l_next_15) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let a_ = ((l_plan_7).wrapping_add(172)).wrapping_add(((((c.m.r32(l_plan_7) as i32)) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    break 'f;
                }
                71 => {
                    bb_ = 70; continue 'fl1;
                }
                72 => {
                    bb_ = 68; continue 'fl1;
                }
                73 => {
                    F::BtlAi_NoteOpponent(c, p_ai_1, 2048i32);
                    bb_ = 74; continue 'fl1;
                }
                74 => {
                    { let v_ = l_n_4; let a_ = (l_seq_3).wrapping_add(128); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (((c.m.r16(l_rule_9) as i16)) as i32); let a_ = (l_seq_3).wrapping_add(140); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (c.m.r32((l_seq_3).wrapping_add(104)) as i32); let a_ = (l_seq_3).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
                    F::BtlAiSeq_PushRule(c, p_ai_1, l_rule_9);
                    break 'f;
                }
                75 => {
                    if ((((((((c.m.r16(l_rule_9) as i16)) as i32)) == (37i32)) as i32)) != 0) { bb_ = 76; continue 'fl1; } else { bb_ = 78; continue 'fl1; }
                }
                76 => {
                    F::BtlAi_NoteOpponent(c, p_ai_1, 4096i32);
                    bb_ = 77; continue 'fl1;
                }
                77 => {
                    bb_ = 74; continue 'fl1;
                }
                78 => {
                    if ((((((((c.m.r16(l_rule_9) as i16)) as i32)) == (38i32)) as i32)) != 0) { bb_ = 79; continue 'fl1; } else { bb_ = 80; continue 'fl1; }
                }
                79 => {
                    F::BtlAi_NoteOpponent(c, p_ai_1, 8192i32);
                    bb_ = 80; continue 'fl1;
                }
                80 => {
                    bb_ = 77; continue 'fl1;
                }
                81 => {
                    bb_ = 59; continue 'fl1;
                }
                _ => unreachable!(),
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "AiThink_EvalRules", &PROBE_N, &[], true); }
    r_
}

pub fn AiThink_GetBlastStep(c: &mut Ctx, mut p_ai_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThink_GetBlastStep", &PROBE_N, &[(p_ai_1) as u32], false); }
    let mut l_act_2: u32 = 0;
    let mut l_action_3: i32 = 0;
    let mut l_seq_4: u32 = 0;
    let mut l_k_5: i32 = 0;
    let r_ = 'f: {
    l_act_2 = { let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) };
    l_action_3 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetUnk974(c, a0_) };
    l_seq_4 = (p_ai_1).wrapping_add(40);
    l_k_5 = (l_action_3).wrapping_sub(60i32);
    if ((((((((c.m.r8(((l_act_2).wrapping_add(904)).wrapping_add(((l_action_3) as u32))) as i8)) as i32)) == (15i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    if (((((((l_k_5) as u32)) >= (((4i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32(l_seq_4) as i32)) & (256i32))) != 0) {
        break 'f (l_action_3).wrapping_sub(58i32);
    }
    break 'f (l_action_3).wrapping_sub(59i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "AiThink_GetBlastStep", &PROBE_N, &[], true); }
    r_
}

pub fn AiThink_TickCooldown(c: &mut Ctx, mut p_ai_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThink_TickCooldown", &PROBE_N, &[(p_ai_1) as u32], false); }
    let mut l_plan_2: u32 = 0;
    let r_ = 'f: {
    l_plan_2 = (p_ai_1).wrapping_add(744);
    if ((((((c.m.r32((l_plan_2).wrapping_add(164)) as i32)) > (0i32)) as i32)) != 0) {
        { let a_ = (l_plan_2).wrapping_add(164); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "AiThink_TickCooldown", &PROBE_N, &[], true); }
    r_
}

pub fn AiThink_Think(c: &mut Ctx, mut p_ai_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThink_Think", &PROBE_N, &[(p_ai_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_seq_2: u32 = 0;
    let mut l_data_3: u32 = 0;
    let mut l_plan_4: u32 = 0;
    let r_ = 'f: {
        l_seq_2 = (p_ai_1).wrapping_add(40);
        l_data_3 = { let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) };
        rt::memset(c, fp, 0, 24);
        { let v_ = 1i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
        { let v_ = 2i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
        { let v_ = 3i32; c.m.w32(fp.wrapping_add(8), (v_) as u32); }
        { let v_ = 5i32; c.m.w32(fp.wrapping_add(12), (v_) as u32); }
        { let v_ = 6i32; c.m.w32(fp.wrapping_add(16), (v_) as u32); }
        { let v_ = 7i32; c.m.w32(fp.wrapping_add(20), (v_) as u32); }
        F::AiThink_TickCooldown(c, p_ai_1);
        l_plan_4 = (p_ai_1).wrapping_add(744);
        { let v_ = 7i32; let a_ = (l_seq_2).wrapping_add(104); c.m.w32(a_, (v_) as u32); };
        { let v_ = 5i32; let a_ = (l_plan_4).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let a1_ = { let a_ = ((l_data_3).wrapping_add(168)).wrapping_add(((((c.m.r32((l_seq_2).wrapping_add(104)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::AiThink_EvalRules(c, p_ai_1, a1_) };
        if ((((((c.m.r32((l_seq_2).wrapping_add(4)) as i32)) != (0i32)) as i32)) != 0) {
            break 'f;
        }
        if ((((((c.m.r32((p_ai_1).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
            break 'f;
        }
        { let v_ = 4i32; let a_ = (l_seq_2).wrapping_add(104); c.m.w32(a_, (v_) as u32); };
        { let a1_ = c.m.r32(((l_data_3).wrapping_add(168)).wrapping_add((((4i32) as u32)).wrapping_mul(4))); F::AiThink_EvalRules(c, p_ai_1, a1_) };
        if ((((((c.m.r32((l_seq_2).wrapping_add(4)) as i32)) != (0i32)) as i32)) != 0) {
            break 'f;
        }
        { let v_ = 0i32; let a_ = (l_seq_2).wrapping_add(104); c.m.w32(a_, (v_) as u32); };
        { let a1_ = c.m.r32(((l_data_3).wrapping_add(168)).wrapping_add((((0i32) as u32)).wrapping_mul(4))); F::AiThink_EvalRules(c, p_ai_1, a1_) };
        { let v_ = { let a_ = (fp).wrapping_add(((((c.m.r32((l_plan_4).wrapping_add(4)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (l_seq_2).wrapping_add(104); c.m.w32(a_, (v_) as u32); };
        { let a1_ = { let a_ = ((l_data_3).wrapping_add(168)).wrapping_add(((((c.m.r32((l_seq_2).wrapping_add(104)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::AiThink_EvalRules(c, p_ai_1, a1_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "AiThink_Think", &PROBE_N, &[], true); }
    r_
}

pub fn AiThink_BindData(c: &mut Ctx, mut p_owned_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThink_BindData", &PROBE_N, &[(p_owned_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let mut l_data_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_file_5: u32 = 0;
    let r_ = 'f: {
    l_p_2 = ((0i32) as u32);
    if (((((p_owned_1) != (1i32)) as i32)) != 0) {
        l_file_5 = { let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) };
        l_p_2 = (l_file_5).wrapping_add(((c.m.r32((l_file_5).wrapping_add((((4i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
    }
    if (((((l_p_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    l_data_3 = l_p_2;
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gBtlAi)).wrapping_add(2640); c.m.w32(a_, (v_) as u32); };
    { let r_ = (!(1i32)); let a_ = (c.m.r32(sym::gBtlAi)).wrapping_add(2644); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    l_p_2 = (l_data_3).wrapping_add(((328i32) as u32));
    { let v_ = l_p_2; let a_ = (l_data_3).wrapping_add(164); c.m.w32(a_, v_); };
    { let v_ = (l_data_3).wrapping_add(((2256i32) as u32)); let a_ = (l_p_2).wrapping_add(4); c.m.w32(a_, v_); };
    { let r_ = (c.m.r32((l_data_3).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let n_ = (l_p_2).wrapping_add(((r_) as u32)); l_p_2 = n_; };
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (8i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = l_p_2; let a_ = ((l_data_3).wrapping_add(168)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = (l_p_2).wrapping_add(((8i32) as u32)); let a_ = (l_p_2).wrapping_add(4); c.m.w32(a_, v_); };
                { let r_ = (c.m.r32((l_data_3).wrapping_add(((((1i32).wrapping_add(l_i_4)) as u32)).wrapping_mul(4))) as i32); let n_ = (l_p_2).wrapping_add(((r_) as u32)); l_p_2 = n_; };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    {
        l_i_4 = 0i32;
        'l3: loop {
            if !(((((l_i_4) < (32i32)) as i32)) != 0) { break 'l3; }
            'c4: {
                { let v_ = l_p_2; let a_ = ((l_data_3).wrapping_add(200)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                if ((((((c.m.r32((l_data_3).wrapping_add(((((9i32).wrapping_add(l_i_4)) as u32)).wrapping_mul(4))) as i32)) == (0i32)) as i32)) != 0) {
                    { let v_ = ((0i32) as u32); let a_ = ((l_data_3).wrapping_add(200)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let r_ = (c.m.r32((l_data_3).wrapping_add(((((9i32).wrapping_add(l_i_4)) as u32)).wrapping_mul(4))) as i32); let n_ = (l_p_2).wrapping_add(((r_) as u32)); l_p_2 = n_; };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    { let v_ = l_data_3; let a_ = c.m.r32(sym::gBtlAi); c.m.w32(a_, v_); };
    if (((((p_owned_1) == (1i32)) as i32)) != 0) {
        { let r_ = 1i32; let a_ = (c.m.r32(sym::gBtlAi)).wrapping_add(2644); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "AiThink_BindData", &PROBE_N, &[], true); }
    r_
}

pub fn AiThink_ReadKit(c: &mut Ctx, mut p_ai_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThink_ReadKit", &PROBE_N, &[(p_ai_1) as u32], false); }
    let mut l_moves_2: u32 = 0;
    let mut l_skills_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_moves_2 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetMoveTable(c, a0_) };
    l_skills_3 = { let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_GetSkillTable(c, a0_) };
    { let v_ = 0i32; let a_ = (p_ai_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((((c.m.r8(((l_moves_2).wrapping_add(158)).wrapping_add(((l_i_4) as u32))) as i8)) as i32)) == (6i32)) as i32)) != 0) {
                    { let r_ = 1i32; let a_ = (p_ai_1).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
                if ((((((((c.m.r8(((l_moves_2).wrapping_add(158)).wrapping_add(((l_i_4) as u32))) as i8)) as i32)) == (7i32)) as i32)) != 0) {
                    { let r_ = 2i32; let a_ = (p_ai_1).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    if ((((((((c.m.r32((p_ai_1).wrapping_add(12)) as i32)) & (3i32))) == (3i32)) as i32)) != 0) {
        { let r_ = (!(2i32)); let a_ = (p_ai_1).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if ((((({ let a0_ = (c.m.r32(p_ai_1) as i32); F::BtlCharApi_HasParamBit80(c, a0_) }) != (0i32)) as i32)) != 0) {
        { let r_ = 4i32; let a_ = (p_ai_1).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    { let v_ = rt::div_i32((if ((((((c.m.r32(((l_skills_3).wrapping_add(412)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) < ((c.m.r32(((l_skills_3).wrapping_add(412)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32))) as i32)) != 0) { (c.m.r32(((l_skills_3).wrapping_add(412)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32) } else { (c.m.r32(((l_skills_3).wrapping_add(412)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32) }), 1000i32); let a_ = (p_ai_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = rt::div_i32((if ((((((c.m.r32(((l_skills_3).wrapping_add(412)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) < ((c.m.r32(((l_skills_3).wrapping_add(412)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32))) as i32)) != 0) { (c.m.r32(((l_skills_3).wrapping_add(412)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32) } else { (c.m.r32(((l_skills_3).wrapping_add(412)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32) }), 1000i32); let a_ = (p_ai_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "AiThink_ReadKit", &PROBE_N, &[], true); }
    r_
}

pub fn AiThink_ResetSide(c: &mut Ctx, mut p_side_1: i32, mut p_owned_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "AiThink_ResetSide", &PROBE_N, &[(p_side_1) as u32, (p_owned_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(256) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 256);
    let mut l_ai_3: u32 = 0;
    let mut l_range_4: u32 = 0;
    let mut l_param_5: u32 = 0;
    let mut l_d_6: F32 = F32(0);
    let r_ = 'f: {
        l_ai_3 = ((c.m.r32(sym::gBtlAi)).wrapping_add(16)).wrapping_add((((p_side_1) as u32)).wrapping_mul(1312));
        l_range_4 = (l_ai_3).wrapping_add(192);
        l_param_5 = ((0i32) as u32);
        if (((((c.m.r32((l_ai_3).wrapping_add(36)) as i32)) & (1i32))) != 0) {
            if (((((c.m.r32((l_ai_3).wrapping_add(24))) != (((0i32) as u32))) as i32)) != 0) {
                { let a0_ = c.m.r32((l_ai_3).wrapping_add(24)); F::Heap_Free(c, a0_) };
                { let v_ = ((0i32) as u32); let a_ = (l_ai_3).wrapping_add(24); c.m.w32(a_, v_); };
            }
        }
        { let _ = F::memset(c, l_ai_3, 0i32, 1312u32); };
        if (((((p_owned_2) == (0i32)) as i32)) != 0) {
            l_param_5 = { let a_ = (F::BtlObj_Get(c, p_side_1)).wrapping_add(2356); c.m.r32(a_) };
            { let _ = F::strcpy(c, fp, sym::btl_ai_cond__str0); };
        }
        if (((((l_param_5) == (((0i32) as u32))) as i32)) != 0) {
            { let v_ = ((0i32) as u32); let a_ = (l_ai_3).wrapping_add(24); c.m.w32(a_, v_); };
            break 'f;
        }
        { let v_ = l_param_5; let a_ = (l_ai_3).wrapping_add(24); c.m.w32(a_, v_); };
        { let v_ = p_side_1; let a_ = l_ai_3; c.m.w32(a_, (v_) as u32); };
        { let r_ = (!(1i32)); let a_ = (l_ai_3).wrapping_add(36); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        { let a0_ = (c.m.r32(l_ai_3) as i32); let a1_ = F::BtlCharApi_GetCpuLevel(c, p_side_1); F::BtlAiMgr_SetLevel(c, a0_, a1_) };
        { let a0_ = (c.m.r32(l_ai_3) as i32); let a1_ = { let a0_ = (c.m.r32(l_ai_3) as i32); F::BtlCharApi_GetAiType(c, a0_) }; F::BtlAiMgr_SetType(c, a0_, a1_) };
        F::BtlAiSeq_Reset(c, (l_ai_3).wrapping_add(40));
        { let v_ = rt::fadd({ let a0_ = (c.m.r32(l_ai_3) as i32); F::BtlCharApi_GetRadius(c, a0_) }, { let a0_ = (((c.m.r32(l_ai_3) as i32)) ^ (1i32)); F::BtlCharApi_GetRadius(c, a0_) }); let a_ = (c.m.r32(sym::gBtlAi)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = { let a_ = (c.m.r32(sym::gBtlAi)).wrapping_add(8); F32(c.m.r32(a_)) }; let a_ = (l_range_4).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        l_d_6 = rt::fsub(F::BtlCharApi_GetCloseRange(c, p_side_1), F::BtlCharApi_GetRadius(c, p_side_1));
        { let v_ = l_d_6; let a_ = (l_range_4).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(l_d_6, F32(0x40a00000)); let a_ = (l_range_4).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = { let v_ = rt::fmul(l_d_6, F32(0x41200000)); let a_ = (l_range_4).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); v_ }; let a_ = (l_range_4).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        F::AiThink_ReadKit(c, l_ai_3);
        if (((((p_owned_2) != (0i32)) as i32)) != 0) {
            { let r_ = 1i32; let a_ = (l_ai_3).wrapping_add(36); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "AiThink_ResetSide", &PROBE_N, &[], true); }
    r_
}
