#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlAct_QueueComboFinish(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_QueueComboFinish", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_n_2: i32 = 0;
    let mut l_col_3: i32 = 0;
    let mut l_tech2_4: i32 = 0;
    let mut l_tech3_5: i32 = 0;
    let r_ = 'f: {
    l_n_2 = 0i32;
    if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((6i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((F::BtlInput_TestAction(c, p_chr_1, 80i32, 1i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    's1: {
        let sv6: i32 = (c.m.r32((p_chr_1).wrapping_add(3472)) as i32);
        match sv6 {
            166i32 => {
                l_col_3 = 3i32;
                break 's1;
            }
            167i32 | 168i32 => {
                l_col_3 = 6i32;
                break 's1;
            }
            169i32 => {
                l_col_3 = 5i32;
                break 's1;
            }
            170i32 => {
                l_col_3 = 4i32;
                break 's1;
            }
            171i32 => {
                l_col_3 = 7i32;
                break 's1;
            }
            172i32 => {
                l_col_3 = 8i32;
                break 's1;
            }
            173i32 => {
                l_col_3 = 2i32;
                break 's1;
            }
            90i32 | 91i32 | 92i32 | 93i32 => {
                l_col_3 = 0i32;
                break 's1;
            }
            _ => {
                l_col_3 = 1i32;
                break 's1;
            }
        }
    }
    's2: {
        let sv7: i32 = F::BtlSuper_GetType(c, p_chr_1, 2i32);
        match sv7 {
            1i32 => {
                l_tech2_4 = 280i32;
                break 's2;
            }
            8i32 => {
                l_tech2_4 = 295i32;
                break 's2;
            }
            _ => {
                l_tech2_4 = 277i32;
                break 's2;
            }
        }
    }
    's3: {
        let sv8: i32 = F::BtlSuper_GetType(c, p_chr_1, 3i32);
        match sv8 {
            1i32 => {
                l_tech3_5 = 281i32;
                break 's3;
            }
            8i32 => {
                l_tech3_5 = 296i32;
                break 's3;
            }
            _ => {
                l_tech3_5 = 278i32;
                break 's3;
            }
        }
    }
    's4: {
        let sv9: i32 = F::BtlParam_GetUnk8F(c, p_chr_1, l_col_3);
        match sv9 {
            0i32 => {
                F::BtlAct_SetQueue(c, p_chr_1, (({ let o_ = l_n_2; let n_ = (o_).wrapping_add(1); l_n_2 = n_; o_ }) as u32), l_tech2_4);
                break 's4;
            }
            1i32 => {
                F::BtlAct_SetQueue(c, p_chr_1, (({ let o_ = l_n_2; let n_ = (o_).wrapping_add(1); l_n_2 = n_; o_ }) as u32), l_tech3_5);
                break 's4;
            }
            2i32 => {
                F::BtlAct_SetQueue(c, p_chr_1, (({ let o_ = l_n_2; let n_ = (o_).wrapping_add(1); l_n_2 = n_; o_ }) as u32), 277i32);
                break 's4;
            }
            3i32 => {
                F::BtlAct_SetQueue(c, p_chr_1, (({ let o_ = l_n_2; let n_ = (o_).wrapping_add(1); l_n_2 = n_; o_ }) as u32), 43i32);
                F::BtlAct_SetQueue(c, p_chr_1, (({ let o_ = l_n_2; let n_ = (o_).wrapping_add(1); l_n_2 = n_; o_ }) as u32), 46i32);
                F::BtlAct_SetQueue(c, p_chr_1, (({ let o_ = l_n_2; let n_ = (o_).wrapping_add(1); l_n_2 = n_; o_ }) as u32), l_tech2_4);
                break 's4;
            }
            4i32 => {
                F::BtlAct_SetQueue(c, p_chr_1, (({ let o_ = l_n_2; let n_ = (o_).wrapping_add(1); l_n_2 = n_; o_ }) as u32), 43i32);
                F::BtlAct_SetQueue(c, p_chr_1, (({ let o_ = l_n_2; let n_ = (o_).wrapping_add(1); l_n_2 = n_; o_ }) as u32), 46i32);
                F::BtlAct_SetQueue(c, p_chr_1, (({ let o_ = l_n_2; let n_ = (o_).wrapping_add(1); l_n_2 = n_; o_ }) as u32), l_tech3_5);
                break 's4;
            }
            5i32 => {
                F::BtlAct_SetQueue(c, p_chr_1, (({ let o_ = l_n_2; let n_ = (o_).wrapping_add(1); l_n_2 = n_; o_ }) as u32), 43i32);
                F::BtlAct_SetQueue(c, p_chr_1, (({ let o_ = l_n_2; let n_ = (o_).wrapping_add(1); l_n_2 = n_; o_ }) as u32), 46i32);
                F::BtlAct_SetQueue(c, p_chr_1, (({ let o_ = l_n_2; let n_ = (o_).wrapping_add(1); l_n_2 = n_; o_ }) as u32), 277i32);
                break 's4;
            }
            _ => {
            }
        }
    }
    F::BtlAct_ResetAttack(c, p_chr_1);
    break 'f l_n_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_QueueComboFinish", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_CanTransform(c: &mut Ctx, mut p_chr_1: u32, mut p_index_2: u32, mut p_needBlast_3: i32, mut p_needAllowed_4: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_CanTransform", &PROBE_N, &[(p_chr_1) as u32, (p_index_2) as u32, (p_needBlast_3) as u32, (p_needAllowed_4) as u32], false); }
    let mut l_chara_5: i32 = 0;
    let mut l_cost_6: i32 = 0;
    let mut l_kind_7: i32 = 0;
    let r_ = 'f: {
    if (((((p_index_2) >= (((4i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((p_needAllowed_4) != 0) && ((((((c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(52)) as i32)) == (0i32)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlMember_HasAbility(c, p_chr_1, 94i32)) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((175i32) as i64 as u64)) as u32))) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlChar_TestMemberUnk70(c, p_chr_1)) != 0) {
        break 'f 0i32;
    }
    l_chara_5 = F::BtlParam_GetSlotId(c, p_chr_1, ((p_index_2) as i32));
    l_cost_6 = F::BtlParam_GetSlotCost(c, p_chr_1, ((p_index_2) as i32));
    l_kind_7 = F::BtlParam_GetSlotA4(c, p_chr_1, ((p_index_2) as i32));
    if (((((l_chara_5) == (255i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!(({ let a0_ = (c.m.r32(p_chr_1) as i32); F::BattleSide_IsCharaUsable(c, a0_, l_chara_5) }) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((((((l_kind_7) == (3i32)) as i32)) != 0) && ((((!((F::BtlStage_HasMoon(c)) != 0)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((p_needBlast_3) != 0) && ((((!((F::BtlMember_HasBlast(c, p_chr_1, l_cost_6)) != 0)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_CanTransform", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_CheckTransformInput(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_CheckTransformInput", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_index_2: i32 = 0;
    let mut l_def_3: i32 = 0;
    let r_ = 'f: {
    l_index_2 = (1i32).wrapping_neg();
    l_def_3 = F::BtlParam_GetUnkAC(c, p_chr_1);
    if ((F::BtlInput_TestAction(c, p_chr_1, 99i32, 1i32)) != 0) {
        l_index_2 = l_def_3;
    }
    if ((F::BtlInput_TestAction(c, p_chr_1, 100i32, 1i32)) != 0) {
        l_index_2 = 0i32;
    }
    if ((F::BtlInput_TestAction(c, p_chr_1, 101i32, 1i32)) != 0) {
        l_index_2 = 1i32;
    }
    if ((F::BtlInput_TestAction(c, p_chr_1, 102i32, 1i32)) != 0) {
        l_index_2 = 2i32;
    }
    if ((F::BtlInput_TestAction(c, p_chr_1, 103i32, 1i32)) != 0) {
        l_index_2 = 3i32;
    }
    if (((((l_index_2) < (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((((((F::BtlParam_CountSlots(c, p_chr_1)) == (1i32)) as i32)) != 0) && (((((l_def_3) >= (0i32)) as i32)) != 0)) as i32)) != 0) {
        l_index_2 = l_def_3;
    }
    if (({ let a3_ = ((!((F::BtlChar_TestFlag(c, p_chr_1, ((((271i32) as i64 as u64)) as u32))) != 0)) as i32); F::BtlAct_CanTransform(c, p_chr_1, ((l_index_2) as u32), 1i32, a3_) }) != 0) {
        break 'f F::BtlAct_QueueTransform(c, p_chr_1, l_index_2);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_CheckTransformInput", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_QueueTransform(c: &mut Ctx, mut p_chr_1: u32, mut p_index_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_QueueTransform", &PROBE_N, &[(p_chr_1) as u32, (p_index_2) as u32], false); }
    let mut l_chara_3: i32 = 0;
    let mut l_cost_4: i32 = 0;
    let mut l_seq_5: i32 = 0;
    let mut l_kind_6: i32 = 0;
    let mut l_unk24_7: i32 = 0;
    let mut l_action_8: i32 = 0;
    let r_ = 'f: {
    l_chara_3 = F::BtlParam_GetSlotId(c, p_chr_1, p_index_2);
    l_cost_4 = F::BtlParam_GetSlotCost(c, p_chr_1, p_index_2);
    l_seq_5 = F::BtlParam_GetSlotA0(c, p_chr_1, p_index_2);
    l_kind_6 = F::BtlParam_GetSlotA4(c, p_chr_1, p_index_2);
    l_unk24_7 = F::BtlParam_GetSlotA8(c, p_chr_1, p_index_2);
    { let v_ = p_index_2; let a_ = (p_chr_1).wrapping_add(4812); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_chara_3; let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a_ = (F::BtlMember_GetActive(c, p_chr_1)).wrapping_add(4); (c.m.r32(a_) as i32) }; let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_cost_4; let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let v_ = (1i32).wrapping_neg(); let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_kind_6; let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_unk24_7; let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(32); (c.m.r32(a_) as i32) }; let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_chara_3; let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_chara_3; let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_chara_3; let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
    if ((((({ let a_ = F::BtlMember_GetActive(c, p_chr_1); (c.m.r32(a_) as i32) }) == (106i32)) as i32)) != 0) {
        { let v_ = 2i32; let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
    }
    if ((F::BtlParam_TestUnkADBit(c, p_chr_1, p_index_2)) != 0) {
        { let v_ = 0i32; let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    }
    's1: {
        let sv9: i32 = l_seq_5;
        match sv9 {
            1i32 => {
                l_action_8 = 237i32;
                break 's1;
            }
            2i32 => {
                l_action_8 = 238i32;
                break 's1;
            }
            3i32 => {
                l_action_8 = 239i32;
                break 's1;
            }
            4i32 => {
                l_action_8 = 240i32;
                break 's1;
            }
            0i32 => {
                l_action_8 = 236i32;
                break 's1;
            }
            _ => {
                l_action_8 = 236i32;
                break 's1;
            }
        }
    }
    F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), l_action_8);
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_QueueTransform", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_CanFuse(c: &mut Ctx, mut p_chr_1: u32, mut p_index_2: i32, mut p_needBlast_3: i32, mut p_needAllowed_4: i32, mut p_partner_5: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_CanFuse", &PROBE_N, &[(p_chr_1) as u32, (p_index_2) as u32, (p_needBlast_3) as u32, (p_needAllowed_4) as u32, (p_partner_5) as u32], false); }
    let mut l_chara_6: i32 = 0;
    let mut l_cost_7: i32 = 0;
    let mut l_member_8: i32 = 0;
    let mut l_i_9: i32 = 0;
    let r_ = 'f: {
    if ((((((p_needAllowed_4) != 0) && ((((((c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(52)) as i32)) == (0i32)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlMember_HasAbility(c, p_chr_1, 94i32)) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((175i32) as i64 as u64)) as u32))) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlChar_TestMemberUnk70(c, p_chr_1)) != 0) {
        break 'f 0i32;
    }
    l_chara_6 = F::BtlParam_GetUnkB4(c, p_chr_1, p_index_2);
    l_cost_7 = F::BtlParam_GetCostAE(c, p_chr_1, p_index_2);
    if (((((l_chara_6) == (255i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!(({ let a0_ = (c.m.r32(p_chr_1) as i32); F::BattleSide_IsCharaUsable(c, a0_, l_chara_6) }) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((p_needBlast_3) != 0) && ((((!((F::BtlMember_HasBlast(c, p_chr_1, l_cost_7)) != 0)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_member_8 = (1i32).wrapping_neg();
    l_i_9 = 0i32;
    'l1: while (((((l_i_9) < (4i32)) as i32)) != 0) {
        l_member_8 = { let a1_ = F::BtlParam_GetUnkBA(c, p_chr_1, p_index_2, { let o_ = l_i_9; let n_ = (o_).wrapping_add(1); l_i_9 = n_; o_ }); F::BtlMember_FindPresentIndex(c, p_chr_1, a1_) };
        if (((((l_member_8) >= (0i32)) as i32)) != 0) {
            break 'l1;
        }
    }
    if (((((l_member_8) < (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (F::BtlMember_GetGauge(c, p_chr_1, l_member_8)).wrapping_add(48); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = F::BtlMember_GetGauge(c, p_chr_1, l_member_8); (c.m.r32(a_) as i32) }) <= (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((p_partner_5) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = l_member_8; let a_ = p_partner_5; c.m.w32(a_, (v_) as u32); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_CanFuse", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_CheckFusionInput(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_CheckFusionInput", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_index_2: i32 = 0;
    let r_ = 'f: {
        l_index_2 = (1i32).wrapping_neg();
        if ((F::BtlInput_TestAction(c, p_chr_1, 104i32, 1i32)) != 0) {
            l_index_2 = 0i32;
        }
        if ((F::BtlInput_TestAction(c, p_chr_1, 105i32, 1i32)) != 0) {
            l_index_2 = 1i32;
        }
        if ((F::BtlInput_TestAction(c, p_chr_1, 106i32, 1i32)) != 0) {
            l_index_2 = 2i32;
        }
        if (((((l_index_2) < (0i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        if (({ let a3_ = ((!((F::BtlChar_TestFlag(c, p_chr_1, ((((271i32) as i64 as u64)) as u32))) != 0)) as i32); F::BtlAct_CanFuse(c, p_chr_1, l_index_2, 1i32, a3_, fp) }) != 0) {
            break 'f { let a2_ = (c.m.r32(fp) as i32); F::BtlAct_QueueFusion(c, p_chr_1, l_index_2, a2_) };
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_CheckFusionInput", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_QueueFusion(c: &mut Ctx, mut p_chr_1: u32, mut p_index_2: i32, mut p_partner_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_QueueFusion", &PROBE_N, &[(p_chr_1) as u32, (p_index_2) as u32, (p_partner_3) as u32], false); }
    let mut l_chara_4: i32 = 0;
    let mut l_cost_5: i32 = 0;
    let mut l_seq_6: i32 = 0;
    let mut l_unk24_7: i32 = 0;
    let mut l_i_8: i32 = 0;
    let mut l_action_9: i32 = 0;
    let r_ = 'f: {
    l_chara_4 = F::BtlParam_GetUnkB4(c, p_chr_1, p_index_2);
    l_cost_5 = F::BtlParam_GetCostAE(c, p_chr_1, p_index_2);
    l_seq_6 = F::BtlParam_GetUnkB1(c, p_chr_1, p_index_2);
    l_unk24_7 = F::BtlParam_GetUnkB7(c, p_chr_1, p_index_2);
    if (((((p_partner_3) < (0i32)) as i32)) != 0) {
        l_i_8 = 0i32;
        'l1: while (((((l_i_8) < (4i32)) as i32)) != 0) {
            p_partner_3 = { let a1_ = F::BtlParam_GetUnkBA(c, p_chr_1, p_index_2, { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; o_ }); F::BtlMember_FindPresentIndex(c, p_chr_1, a1_) };
            if (((((p_partner_3) >= (0i32)) as i32)) != 0) {
                break 'l1;
            }
        }
        if (((((p_partner_3) < (0i32)) as i32)) != 0) {
            break 'f 0i32;
        }
    }
    { let v_ = p_index_2; let a_ = (p_chr_1).wrapping_add(4812); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_cost_5; let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let v_ = 5i32; let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_unk24_7; let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_chara_4; let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_partner_3; let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a_ = (F::BtlMember_Get(c, p_chr_1, p_partner_3)).wrapping_add(4); (c.m.r32(a_) as i32) }; let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(32); (c.m.r32(a_) as i32) }; let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_chara_4; let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_chara_4; let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_chara_4; let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
    's2: {
        let sv10: i32 = l_seq_6;
        match sv10 {
            1i32 => {
                l_action_9 = 241i32;
                break 's2;
            }
            2i32 => {
                l_action_9 = 242i32;
                break 's2;
            }
            _ => {
                break 'f 0i32;
            }
        }
    }
    F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), l_action_9);
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_QueueFusion", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_CanSwitch(c: &mut Ctx, mut p_chr_1: u32, mut p_needGauge_2: i32, mut p_needAllowed_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_CanSwitch", &PROBE_N, &[(p_chr_1) as u32, (p_needGauge_2) as u32, (p_needAllowed_3) as u32], false); }
    let r_ = 'f: {
    if ((((((p_needAllowed_3) != 0) && ((((((c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(52)) as i32)) == (0i32)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((175i32) as i64 as u64)) as u32))) != 0) {
        break 'f 0i32;
    } else {
        if ((((((p_needGauge_2) != 0) && ((((((c.m.r32((p_chr_1).wrapping_add(2460)) as i32)) < (100000i32)) as i32)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        if (((((F::BtlMember_CountAlive(c, p_chr_1)) < (2i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        break 'f 1i32;
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_CanSwitch", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_CheckSwitchInput(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_CheckSwitchInput", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if ((((!((F::BtlInput_TestAction(c, p_chr_1, 107i32, 1i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!(({ let a2_ = ((!((F::BtlChar_TestFlag(c, p_chr_1, ((((271i32) as i64 as u64)) as u32))) != 0)) as i32); F::BtlAct_CanSwitch(c, p_chr_1, 1i32, a2_) }) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f F::BtlAct_QueueSwitch(c, p_chr_1);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_CheckSwitchInput", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_QueueSwitch(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_QueueSwitch", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    F::BtlAct_PrepareSwitch(c, p_chr_1);
    F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 243i32);
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_QueueSwitch", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_GetTechniqueAction(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetTechniqueAction", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let mut l_n_3: i32 = 0;
    let r_ = 'f: {
    l_n_3 = (p_slot_2).wrapping_sub(2i32);
    's1: {
        let sv4: i32 = F::BtlSuper_GetType(c, p_chr_1, p_slot_2);
        match sv4 {
            0i32 | 7i32 => {
                break 'f (l_n_3).wrapping_add(262i32);
            }
            1i32 => {
                break 'f (l_n_3).wrapping_add(268i32);
            }
            2i32 => {
                break 'f (l_n_3).wrapping_add(271i32);
            }
            3i32 => {
                break 'f (l_n_3).wrapping_add(274i32);
            }
            6i32 => {
                break 'f (l_n_3).wrapping_add(286i32);
            }
            4i32 | 5i32 => {
                break 'f (l_n_3).wrapping_add(283i32);
            }
            9i32 => {
                break 'f (l_n_3).wrapping_add(265i32);
            }
            8i32 => {
                break 'f (l_n_3).wrapping_add(292i32);
            }
            10i32 => {
                break 'f (l_n_3).wrapping_add(289i32);
            }
            _ => {
                break 'f (p_slot_2).wrapping_add(260i32);
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetTechniqueAction", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_CanUseTechnique(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_CanUseTechnique", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let mut l_check_3: i32 = 0;
    let r_ = 'f: {
    if ((((((((p_slot_2).wrapping_sub(2i32)) as u32)) >= (((3i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlChar_TestMemberUnk70(c, p_chr_1)) != 0) {
        break 'f 0i32;
    }
    if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((((((p_slot_2) == (4i32)) as i32)) != 0) && ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((6i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!(({ let a1_ = F::BtlSuper_GetKiCost(c, p_chr_1, p_slot_2); F::BtlMember_HasKi(c, p_chr_1, a1_) }) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((((F::BtlSuper_GetFlags(c, p_chr_1, p_slot_2)) as u64)) as i32)) & (8192i32))) != 0) {
        if (((((F::Battle_GetMode(c)) == (5i32)) as i32)) != 0) {
            break 'f 1i32;
        }
        if (((((F::Battle_GetMode(c)) == (6i32)) as i32)) != 0) {
            break 'f 1i32;
        }
        l_check_3 = 1i32;
        if (((((F::Battle_GetMode(c)) == (1i32)) as i32)) != 0) {
            l_check_3 = ((!((F::BtlChar_TestFlag(c, p_chr_1, ((((287i32) as i64 as u64)) as u32))) != 0)) as i32);
        }
        if ((l_check_3) != 0) {
            if ((((({ let a_ = ((F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(76)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                break 'f 0i32;
            }
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_CanUseTechnique", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_CheckTechniqueInput(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_CheckTechniqueInput", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_slot_2: i32 = 0;
    let mut l_n_3: i32 = 0;
    let mut l_pre_4: i32 = 0;
    let mut l_flags_5: i32 = 0;
    let mut l_kind_6: i32 = 0;
    let r_ = 'f: {
    l_slot_2 = (1i32).wrapping_neg();
    l_n_3 = 0i32;
    if ((((((c.m.r32((p_chr_1).wrapping_add(3648)) as i32)) > (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlInput_TestAction(c, p_chr_1, 108i32, 1i32)) != 0) {
        l_slot_2 = 2i32;
    }
    if ((F::BtlInput_TestAction(c, p_chr_1, 109i32, 1i32)) != 0) {
        l_slot_2 = 3i32;
    }
    if ((F::BtlInput_TestAction(c, p_chr_1, 110i32, 1i32)) != 0) {
        l_slot_2 = 4i32;
    }
    if (((((l_slot_2) < (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((F::BtlAct_CanUseTechnique(c, p_chr_1, l_slot_2)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((F::BtlChar_IsFree(c, p_chr_1)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((19i32) as i64 as u64)) as u32))) != 0) {
        l_flags_5 = ((((((F::BtlSuper_GetFlags(c, p_chr_1, l_slot_2)) as u64)) as i32)) & (512i32));
        l_kind_6 = F::BtlSuper_GetType(c, p_chr_1, l_slot_2);
        l_pre_4 = (((l_flags_5) == (0i32)) as i32);
        if (((((l_kind_6) == (10i32)) as i32)) != 0) {
            l_pre_4 = 0i32;
        }
        if ((l_pre_4) != 0) {
            F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 27i32);
            l_n_3 = 1i32;
        }
    }
    if (((((l_slot_2) == (4i32)) as i32)) != 0) {
        F::BtlAct_SetQueue(c, p_chr_1, (({ let o_ = l_n_3; let n_ = (o_).wrapping_add(1); l_n_3 = n_; o_ }) as u32), 261i32);
    }
    { let a2_ = F::BtlAct_GetTechniqueAction(c, p_chr_1, l_slot_2); F::BtlAct_SetQueue(c, p_chr_1, (({ let o_ = l_n_3; let n_ = (o_).wrapping_add(1); l_n_3 = n_; o_ }) as u32), a2_) };
    break 'f l_n_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_CheckTechniqueInput", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_GetSkillAction(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetSkillAction", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv3: i32 = F::BtlSkill_GetSequence(c, p_chr_1, p_slot_2);
        match sv3 {
            1i32 => {
                break 'f (p_slot_2).wrapping_add(255i32);
            }
            0i32 => {
                break 'f (p_slot_2).wrapping_add(253i32);
            }
            2i32 => {
                break 'f (p_slot_2).wrapping_add(257i32);
            }
            _ => {
                break 'f (p_slot_2).wrapping_add(253i32);
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetSkillAction", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_CanUseSkill(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_CanUseSkill", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let mut l_flags_3: i32 = 0;
    let mut l_stack_4: i32 = 0;
    let r_ = 'f: {
    if (((((p_slot_2) >= (((2i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_flags_3 = ((((F::BtlSkill_GetFlags(c, p_chr_1, ((p_slot_2) as i32))) as u64)) as i32);
    l_stack_4 = (((l_flags_3).wrapping_shr((8i32) as u32)) & (1i32));
    if ((F::BtlChar_TestMemberUnk70(c, p_chr_1)) != 0) {
        break 'f 0i32;
    }
    if ((((((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) && ((((l_flags_3) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!(({ let a1_ = F::BtlSkill_GetBlastCost(c, p_chr_1, ((p_slot_2) as i32)); F::BtlMember_HasBlast(c, p_chr_1, a1_) }) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((((l_flags_3) & (16i32))) != 0) && ((((({ let a_ = ((F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(56)).wrapping_add((p_slot_2).wrapping_mul(4)); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    's1: {
        let sv5: i32 = F::BtlSkill_GetId(c, p_chr_1, ((p_slot_2) as i32));
        match sv5 {
            12i32 | 50i32 => {
                if ((l_stack_4) != 0) {
                    if ((((((c.m.r32((p_chr_1).wrapping_add(3584)) as i32)) >= (3i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                } else {
                    if ((((((c.m.r32((p_chr_1).wrapping_add(3584)) as i32)) > (0i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                }
                break 's1;
            }
            55i32 => {
                if ((l_stack_4) != 0) {
                    if ((((((c.m.r32((p_chr_1).wrapping_add(3588)) as i32)) >= (3i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                } else {
                    if ((((((c.m.r32((p_chr_1).wrapping_add(3588)) as i32)) > (0i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                }
                break 's1;
            }
            65i32 => {
                if ((((((c.m.r32((p_chr_1).wrapping_add(3676)) as i32)) >= (3i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_CanUseSkill", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_CheckSkillInput(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_CheckSkillInput", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_slot_2: i32 = 0;
    let r_ = 'f: {
    l_slot_2 = (1i32).wrapping_neg();
    if ((F::BtlInput_TestAction(c, p_chr_1, 113i32, 1i32)) != 0) {
        l_slot_2 = 0i32;
    }
    if ((F::BtlInput_TestAction(c, p_chr_1, 114i32, 1i32)) != 0) {
        l_slot_2 = 1i32;
    }
    if (((((l_slot_2) < (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((F::BtlAct_CanUseSkill(c, p_chr_1, ((l_slot_2) as u32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((F::BtlChar_IsFree(c, p_chr_1)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((((({ let a0_ = F::BtlAnim_GetId(c, p_chr_1); ((((F::BtlAnim_GetFlags(c, ((((a0_) as i64 as u64)) as u32))) as u64)) as i32) }) & (16i32))) != 0) && ((((!((((((((F::BtlSkill_GetFlags(c, p_chr_1, l_slot_2)) as u64)) as i32)) & (8i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let a2_ = F::BtlAct_GetSkillAction(c, p_chr_1, l_slot_2); F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), a2_) };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_CheckSkillInput", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_QueueReaction(c: &mut Ctx, mut p_chr_1: u32, mut p_reaction_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_QueueReaction", &PROBE_N, &[(p_chr_1) as u32, (p_reaction_2) as u32], false); }
    let mut l_action_3: i32 = 0;
    let r_ = 'f: {
    if (((((p_reaction_2) < (((2i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((((((p_reaction_2) == (((2i32) as u32))) as i32)) != 0) && ((((!((F::BtlChar_IsDead(c, p_chr_1)) != 0)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((((c.m.r8(((p_chr_1).wrapping_add(4100)).wrapping_add(p_reaction_2)) as i8)) as i32)) < (100i32)) as i32)) != 0) {
        { let a_ = ((p_chr_1).wrapping_add(4100)).wrapping_add(p_reaction_2); let o_ = (c.m.r8(a_) as i8); let n_ = (o_).wrapping_add(1); c.m.w8(a_, (n_) as u8); };
    }
    's1: {
        let sv4: u32 = p_reaction_2;
        match sv4 {
            35u32 => {
                { let a_ = ((p_chr_1).wrapping_add(4100)).wrapping_add(((36i32) as u32)); let o_ = (c.m.r8(a_) as i8); let n_ = (o_).wrapping_add(1); c.m.w8(a_, (n_) as u8); };
                break 's1;
            }
            36u32 => {
                { let a_ = ((p_chr_1).wrapping_add(4100)).wrapping_add(((35i32) as u32)); let o_ = (c.m.r8(a_) as i8); let n_ = (o_).wrapping_add(1); c.m.w8(a_, (n_) as u8); };
                break 's1;
            }
            _ => {
            }
        }
    }
    's2: {
        let sv5: u32 = p_reaction_2;
        match sv5 {
            5u32 | 6u32 | 7u32 | 8u32 | 9u32 | 10u32 | 11u32 | 12u32 | 13u32 | 35u32 | 36u32 | 42u32 | 45u32 | 46u32 | 47u32 => {
                if ((((((((c.m.r8(((p_chr_1).wrapping_add(4100)).wrapping_add(p_reaction_2)) as i8)) as i32)) >= (2i32)) as i32)) != 0) {
                    F::BtlChar_SetFlag(c, p_chr_1, ((((148i32) as i64 as u64)) as u32));
                    p_reaction_2 = ((16i32) as u32);
                    if ((((((c.m.r32((p_chr_1).wrapping_add(4028)) as i32)) != (0i32)) as i32)) != 0) {
                        { let v_ = F32(c.m.r32((p_chr_1).wrapping_add(4044))); let a_ = (p_chr_1).wrapping_add(4048); c.m.w32(a_, (v_).0); };
                        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(4052); c.m.w32(a_, (v_) as u32); };
                    } else {
                        { let v_ = { let a0_ = rt::fadd(F32(c.m.r32((p_chr_1).wrapping_add(4044))), F32(0x40490fda)); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = (p_chr_1).wrapping_add(4048); c.m.w32(a_, (v_).0); };
                        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(4052); c.m.w32(a_, (v_) as u32); };
                    }
                }
                break 's2;
            }
            _ => {
            }
        }
    }
    if ((F::BtlChar_IsDead(c, p_chr_1)) != 0) {
        's3: {
            let sv6: u32 = p_reaction_2;
            match sv6 {
                2u32 | 3u32 | 4u32 | 5u32 | 6u32 | 7u32 | 8u32 | 9u32 | 23u32 | 24u32 | 25u32 | 42u32 | 43u32 | 44u32 | 45u32 | 46u32 | 47u32 | 48u32 => {
                    F::BtlChar_SetFlag(c, p_chr_1, ((((148i32) as i64 as u64)) as u32));
                    p_reaction_2 = ((15i32) as u32);
                    if ((((((c.m.r32((p_chr_1).wrapping_add(4028)) as i32)) != (0i32)) as i32)) != 0) {
                        { let v_ = F32(c.m.r32((p_chr_1).wrapping_add(4044))); let a_ = (p_chr_1).wrapping_add(4048); c.m.w32(a_, (v_).0); };
                        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(4052); c.m.w32(a_, (v_) as u32); };
                    } else {
                        { let v_ = { let a0_ = rt::fadd(F32(c.m.r32((p_chr_1).wrapping_add(4044))), F32(0x40490fda)); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = (p_chr_1).wrapping_add(4048); c.m.w32(a_, (v_).0); };
                        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(4052); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's3;
                }
                _ => {
                }
            }
        }
    }
    { let v_ = ((0i32) as i8); let a_ = ((p_chr_1).wrapping_add(4150)).wrapping_add(p_reaction_2); c.m.w8(a_, (v_) as u8); };
    's4: {
        let sv7: u32 = p_reaction_2;
        match sv7 {
            4u32 => {
                l_action_3 = 193i32;
                break 's4;
            }
            5u32 => {
                l_action_3 = 194i32;
                break 's4;
            }
            6u32 => {
                l_action_3 = 195i32;
                break 's4;
            }
            7u32 | 43u32 | 46u32 => {
                l_action_3 = 196i32;
                break 's4;
            }
            8u32 | 44u32 => {
                l_action_3 = 197i32;
                break 's4;
            }
            9u32 | 42u32 | 45u32 | 47u32 | 48u32 => {
                l_action_3 = 198i32;
                break 's4;
            }
            10u32 => {
                l_action_3 = 206i32;
                break 's4;
            }
            11u32 => {
                l_action_3 = 205i32;
                break 's4;
            }
            14u32 => {
                l_action_3 = 218i32;
                break 's4;
            }
            15u32 => {
                l_action_3 = 207i32;
                break 's4;
            }
            16u32 => {
                l_action_3 = 208i32;
                break 's4;
            }
            38u32 => {
                l_action_3 = 209i32;
                break 's4;
            }
            17u32 => {
                l_action_3 = 213i32;
                break 's4;
            }
            18u32 => {
                l_action_3 = 214i32;
                break 's4;
            }
            19u32 => {
                l_action_3 = 212i32;
                break 's4;
            }
            20u32 => {
                l_action_3 = 184i32;
                break 's4;
            }
            37u32 => {
                l_action_3 = 188i32;
                break 's4;
            }
            22u32 => {
                l_action_3 = 186i32;
                break 's4;
            }
            49u32 => {
                l_action_3 = 191i32;
                break 's4;
            }
            12u32 | 39u32 => {
                l_action_3 = 203i32;
                break 's4;
            }
            13u32 => {
                l_action_3 = 204i32;
                break 's4;
            }
            31u32 => {
                l_action_3 = 219i32;
                break 's4;
            }
            32u32 => {
                l_action_3 = 220i32;
                break 's4;
            }
            33u32 => {
                l_action_3 = 221i32;
                break 's4;
            }
            35u32 => {
                l_action_3 = 199i32;
                break 's4;
            }
            36u32 => {
                l_action_3 = 200i32;
                break 's4;
            }
            40u32 => {
                l_action_3 = 223i32;
                break 's4;
            }
            23u32 => {
                l_action_3 = 211i32;
                if ((((((c.m.r32((p_chr_1).wrapping_add(4064)) as i32)) <= (0i32)) as i32)) != 0) {
                    { let v_ = 15i32; let a_ = (p_chr_1).wrapping_add(4064); c.m.w32(a_, (v_) as u32); };
                }
                break 's4;
            }
            24u32 => {
                l_action_3 = 211i32;
                if ((((((c.m.r32((p_chr_1).wrapping_add(4064)) as i32)) <= (0i32)) as i32)) != 0) {
                    { let v_ = 30i32; let a_ = (p_chr_1).wrapping_add(4064); c.m.w32(a_, (v_) as u32); };
                }
                break 's4;
            }
            25u32 => {
                l_action_3 = 211i32;
                if ((((((c.m.r32((p_chr_1).wrapping_add(4064)) as i32)) <= (0i32)) as i32)) != 0) {
                    { let v_ = 45i32; let a_ = (p_chr_1).wrapping_add(4064); c.m.w32(a_, (v_) as u32); };
                }
                break 's4;
            }
            26u32 => {
                l_action_3 = 224i32;
                if ((((((c.m.r32((p_chr_1).wrapping_add(4064)) as i32)) <= (0i32)) as i32)) != 0) {
                    { let v_ = 15i32; let a_ = (p_chr_1).wrapping_add(4064); c.m.w32(a_, (v_) as u32); };
                }
                break 's4;
            }
            27u32 => {
                l_action_3 = 224i32;
                if ((((((c.m.r32((p_chr_1).wrapping_add(4064)) as i32)) <= (0i32)) as i32)) != 0) {
                    { let v_ = 30i32; let a_ = (p_chr_1).wrapping_add(4064); c.m.w32(a_, (v_) as u32); };
                }
                break 's4;
            }
            28u32 => {
                l_action_3 = 224i32;
                if ((((((c.m.r32((p_chr_1).wrapping_add(4064)) as i32)) <= (0i32)) as i32)) != 0) {
                    { let v_ = 45i32; let a_ = (p_chr_1).wrapping_add(4064); c.m.w32(a_, (v_) as u32); };
                }
                break 's4;
            }
            34u32 => {
                l_action_3 = ((c.m.r32((p_chr_1).wrapping_add(3740)) as i32)).wrapping_add(311i32);
                break 's4;
            }
            29u32 | 30u32 => {
                l_action_3 = ((c.m.r32((p_chr_1).wrapping_add(3740)) as i32)).wrapping_add(305i32);
                break 's4;
            }
            41u32 => {
                l_action_3 = ((c.m.r32((p_chr_1).wrapping_add(4084)) as i32)).wrapping_add(308i32);
                break 's4;
            }
            _ => {
                l_action_3 = 192i32;
                break 's4;
            }
        }
    }
    if (({ let a0_ = F::BtlAnim_GetId(c, p_chr_1); ((((F::BtlAnim_GetFlags(c, ((((a0_) as i64 as u64)) as u32))) as u64)) as i32) }) != 0) {
        { let a0_ = F::BtlOpp_GetPlayer(c, p_chr_1); F::BtlEvent_Raise(c, a0_, 85i32) };
    }
    F::BtlChar_SetFlag(c, p_chr_1, ((((297i32) as i64 as u64)) as u32));
    F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), l_action_3);
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_QueueReaction", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_GetLandingAction(c: &mut Ctx, mut p_chr_1: u32, mut p_arg1_2: i32, mut p_kind_3: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetLandingAction", &PROBE_N, &[(p_chr_1) as u32, (p_arg1_2) as u32, (p_kind_3) as u32], false); }
    let mut l_action_4: i32 = 0;
    let r_ = 'f: {
    l_action_4 = 207i32;
    's1: {
        let sv5: u32 = p_kind_3;
        match sv5 {
            0u32 => {
                l_action_4 = 216i32;
                break 's1;
            }
            1u32 => {
                l_action_4 = 217i32;
                break 's1;
            }
            2u32 => {
                l_action_4 = 207i32;
                break 's1;
            }
            3u32 => {
                l_action_4 = 208i32;
                break 's1;
            }
            4u32 => {
                l_action_4 = 213i32;
                break 's1;
            }
            6u32 => {
                l_action_4 = 212i32;
                break 's1;
            }
            7u32 => {
                l_action_4 = 219i32;
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f l_action_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetLandingAction", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_QueueFlag21Action(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_QueueFlag21Action", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((33i32) as i64 as u64)) as u32))) != 0) {
        if ((((!((F::BtlParam_CanFly(c, p_chr_1)) != 0)) as i32)) != 0) {
            F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 37i32);
            break 'f 1i32;
        } else {
            F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 39i32);
            break 'f 1i32;
        }
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((34i32) as i64 as u64)) as u32))) != 0) {
        if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
            if ((((!((F::BtlParam_CanFly(c, p_chr_1)) != 0)) as i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 37i32);
                break 'f 1i32;
            } else {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 39i32);
                break 'f 1i32;
            }
        } else {
            F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 40i32);
            break 'f 1i32;
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_QueueFlag21Action", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_QueueFlag21ActionB(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_QueueFlag21ActionB", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((33i32) as i64 as u64)) as u32))) != 0) {
        F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 41i32);
        break 'f 1i32;
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((34i32) as i64 as u64)) as u32))) != 0) {
        if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
            F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 41i32);
            break 'f 1i32;
        } else {
            F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 42i32);
            break 'f 1i32;
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_QueueFlag21ActionB", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_CheckRecoveryInput(c: &mut Ctx, mut p_chr_1: u32, mut p_held_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_CheckRecoveryInput", &PROBE_N, &[(p_chr_1) as u32, (p_held_2) as u32], false); }
    let mut l_pose_3: u32 = 0;
    let mut l_test_4: u32 = 0;
    let mut l_button_5: i32 = 0;
    let mut l_front_6: i32 = 0;
    let r_ = 'f: {
    l_pose_3 = F::BtlChar_GetPos(c, p_chr_1);
    if ((p_held_2) != 0) {
        l_test_4 = fid::BtlInput_IsHeld;
    } else {
        l_test_4 = fid::BtlInput_IsPressed;
    }
    if ((((!((F::BtlChar_IsFree(c, p_chr_1)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0) {
        if ((((D::call(c, l_test_4, &[((p_chr_1) as u64), ((((240i32) as u32)) as u64)])) as i32)) != 0) {
            F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 230i32);
            break 'f 1i32;
        }
        if ((((D::call(c, l_test_4, &[((p_chr_1) as u64), ((((1048576i32) as u32)) as u64)])) as i32)) != 0) {
            F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 230i32);
            break 'f 1i32;
        }
    } else {
        l_button_5 = F::BtlInput_IsPressed(c, p_chr_1, ((1048576i32) as u32));
        l_front_6 = 0i32;
        if ((l_button_5) != 0) {
            l_test_4 = fid::BtlInput_IsHeld;
        }
        if (((rt::flt({ let a0_ = { let a0_ = rt::fsub(F32(c.m.r32(((l_pose_3).wrapping_add(16)).wrapping_add(4))), F32(c.m.r32((p_chr_1).wrapping_add(1184)))); F::BtlUtil_WrapAngle(c, a0_) }; rt::ffabs(a0_) }, F32(0x3fc90fda)) as i32)) != 0) {
            l_front_6 = 1i32;
        }
        if ((((D::call(c, l_test_4, &[((p_chr_1) as u64), ((((64i32) as u32)) as u64)])) as i32)) != 0) {
            F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), (if ((l_front_6) != 0) { 226i32 } else { 227i32 }));
            break 'f 1i32;
        }
        if ((((D::call(c, l_test_4, &[((p_chr_1) as u64), ((((128i32) as u32)) as u64)])) as i32)) != 0) {
            F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), (if ((l_front_6) != 0) { 227i32 } else { 226i32 }));
            break 'f 1i32;
        }
        if ((((D::call(c, l_test_4, &[((p_chr_1) as u64), ((((16i32) as u32)) as u64)])) as i32)) != 0) {
            F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), (if ((l_front_6) != 0) { 229i32 } else { 228i32 }));
            break 'f 1i32;
        }
        if ((((D::call(c, l_test_4, &[((p_chr_1) as u64), ((((32i32) as u32)) as u64)])) as i32)) != 0) {
            F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), (if ((l_front_6) != 0) { 228i32 } else { 229i32 }));
            break 'f 1i32;
        }
        if ((l_button_5) != 0) {
            F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 225i32);
            break 'f 1i32;
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_CheckRecoveryInput", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_CheckStoryForced(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_CheckStoryForced", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if (((((F::Battle_GetMode(c)) != (1i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((250i32) as i64 as u64)) as u32))) != 0) {
        F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 4i32);
        break 'f 1i32;
    }
    if ((((!(({ let a0_ = F::BtlAct_GetCurrent(c, p_chr_1); F::BtlAct_IsTechniqueId(c, a0_) }) != 0)) as i32)) != 0) {
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((278i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_ClearFlag(c, p_chr_1, ((((278i32) as i64 as u64)) as u32));
            { let _ = F::BtlAct_QueueTransform(c, p_chr_1, 0i32); };
            break 'f 1i32;
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((279i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_ClearFlag(c, p_chr_1, ((((279i32) as i64 as u64)) as u32));
            { let _ = F::BtlAct_QueueTransform(c, p_chr_1, 1i32); };
            break 'f 1i32;
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((280i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_ClearFlag(c, p_chr_1, ((((280i32) as i64 as u64)) as u32));
            { let _ = F::BtlAct_QueueTransform(c, p_chr_1, 2i32); };
            break 'f 1i32;
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((281i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_ClearFlag(c, p_chr_1, ((((281i32) as i64 as u64)) as u32));
            { let _ = F::BtlAct_QueueTransform(c, p_chr_1, 3i32); };
            break 'f 1i32;
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((282i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_ClearFlag(c, p_chr_1, ((((282i32) as i64 as u64)) as u32));
            { let _ = F::BtlAct_QueueFusion(c, p_chr_1, 0i32, (1i32).wrapping_neg()); };
            break 'f 1i32;
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((283i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_ClearFlag(c, p_chr_1, ((((283i32) as i64 as u64)) as u32));
            { let _ = F::BtlAct_QueueFusion(c, p_chr_1, 1i32, (1i32).wrapping_neg()); };
            break 'f 1i32;
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((284i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_ClearFlag(c, p_chr_1, ((((284i32) as i64 as u64)) as u32));
            { let _ = F::BtlAct_QueueFusion(c, p_chr_1, 2i32, (1i32).wrapping_neg()); };
            break 'f 1i32;
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((285i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_ClearFlag(c, p_chr_1, ((((285i32) as i64 as u64)) as u32));
            { let _ = F::BtlAct_QueueSwitch(c, p_chr_1); };
            break 'f 1i32;
        }
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((272i32) as i64 as u64)) as u32))) != 0) {
        F::BtlChar_ClearFlag(c, p_chr_1, ((((272i32) as i64 as u64)) as u32));
        F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 5i32);
        break 'f 1i32;
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((273i32) as i64 as u64)) as u32))) != 0) {
        F::BtlChar_ClearFlag(c, p_chr_1, ((((273i32) as i64 as u64)) as u32));
        F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 6i32);
        break 'f 1i32;
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((274i32) as i64 as u64)) as u32))) != 0) {
        F::BtlChar_ClearFlag(c, p_chr_1, ((((274i32) as i64 as u64)) as u32));
        F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 7i32);
        break 'f 1i32;
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((275i32) as i64 as u64)) as u32))) != 0) {
        F::BtlChar_ClearFlag(c, p_chr_1, ((((275i32) as i64 as u64)) as u32));
        F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 8i32);
        break 'f 1i32;
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((276i32) as i64 as u64)) as u32))) != 0) {
        F::BtlChar_ClearFlag(c, p_chr_1, ((((276i32) as i64 as u64)) as u32));
        F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 9i32);
        break 'f 1i32;
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((277i32) as i64 as u64)) as u32))) != 0) {
        F::BtlChar_ClearFlag(c, p_chr_1, ((((277i32) as i64 as u64)) as u32));
        F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 10i32);
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_CheckStoryForced", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_GetDownAction(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetDownAction", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (if ((((((c.m.r32((p_chr_1).wrapping_add(4064)) as i32)) > (0i32)) as i32)) != 0) { 224i32 } else { 216i32 });
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetDownAction", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_GetEvasionAttack(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetEvasionAttack", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_n_2: i32 = 0;
    let r_ = 'f: {
    l_n_2 = (c.m.r32((p_chr_1).wrapping_add(3464)) as i32);
    { let r_ = F::BtlOpp_GetUnkD88(c, p_chr_1); let n_ = (l_n_2).wrapping_add(r_); l_n_2 = n_; };
    's1: {
        let sv3: i32 = rt::rem_i32(l_n_2, 3i32);
        match sv3 {
            1i32 => {
                break 'f 137i32;
            }
            0i32 => {
                break 'f (if (((rt::fgt(F::BtlAct_GetHeightRatio(c, p_chr_1), F32(0x3f000000)) as i32)) != 0) { 140i32 } else { 139i32 });
            }
            2i32 => {
                break 'f 138i32;
            }
            _ => {
            }
        }
    }
    break 'f 140i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetEvasionAttack", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_GetChainAction(c: &mut Ctx, mut p_chr_1: u32, mut p_kind_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetChainAction", &PROBE_N, &[(p_chr_1) as u32, (p_kind_2) as u32], false); }
    let mut l_id_3: i32 = 0;
    let r_ = 'f: {
    's1: {
        let sv4: u32 = p_kind_2;
        match sv4 {
            0u32 => {
                l_id_3 = 106i32;
                break 's1;
            }
            1u32 => {
                l_id_3 = 158i32;
                break 's1;
            }
            2u32 => {
                l_id_3 = 159i32;
                break 's1;
            }
            3u32 => {
                l_id_3 = 160i32;
                break 's1;
            }
            4u32 => {
                l_id_3 = 161i32;
                break 's1;
            }
            _ => {
                break 'f 106i32;
            }
        }
    }
    if ((((!((F::BtlAct_IsAttackId(c, l_id_3)) != 0)) as i32)) != 0) {
        break 'f l_id_3;
    }
    if ((F::BtlDecide_QueueAttack(c, p_chr_1, l_id_3)) != 0) {
        break 'f F::BtlAct_GetQueued(c, p_chr_1);
    }
    break 'f 106i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetChainAction", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_GetIdleFollowUp(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetIdleFollowUp", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 11i32);
    { let _ = F::BtlDecide_Main(c, p_chr_1, ((((((524288i32) as u32)) as u64)) as i32)); };
    { let _ = F::BtlDecide_Common(c, p_chr_1, 15i32); };
    break 'f F::BtlAct_GetQueued(c, p_chr_1);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetIdleFollowUp", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_GetAction14or16(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetAction14or16", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (if ((((!((F::BtlParam_CanFly(c, p_chr_1)) != 0)) as i32)) != 0) { 22i32 } else { 20i32 });
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetAction14or16", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_GetAction11or15(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetAction11or15", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if ((((!((F::BtlParam_CanFly(c, p_chr_1)) != 0)) as i32)) != 0) {
        if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
            break 'f 17i32;
        }
    }
    break 'f 21i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetAction11or15", &PROBE_N, &[], true); }
    r_
}
