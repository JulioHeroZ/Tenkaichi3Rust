#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlAiSeq_Reset(c: &mut Ctx, mut p_seq_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSeq_Reset", &PROBE_N, &[(p_seq_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = p_seq_1; c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (p_seq_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (p_seq_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (p_seq_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (p_seq_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (8i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = ((1i32) as u32); let a_ = ((p_seq_1).wrapping_add(20)).wrapping_add((((l_i_2) as u32)).wrapping_mul(8)); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as i8); let a_ = (((p_seq_1).wrapping_add(20)).wrapping_add((((l_i_2) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w8(a_, (v_) as u8); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    { let v_ = 0i32; let a_ = (p_seq_1).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSeq_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiSeq_PushRule(c: &mut Ctx, mut p_ai_1: u32, mut p_rule_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSeq_PushRule", &PROBE_N, &[(p_ai_1) as u32, (p_rule_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_seq_3: u32 = 0;
    let mut l_plan_4: u32 = 0;
    let mut l_entry_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let r_ = 'f: {
        l_seq_3 = (p_ai_1).wrapping_add(40);
        rt::memset(c, fp, 0, 20);
        { let v_ = 2i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
        { let v_ = 3i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
        { let v_ = 4i32; c.m.w32(fp.wrapping_add(8), (v_) as u32); }
        { let v_ = 5i32; c.m.w32(fp.wrapping_add(12), (v_) as u32); }
        { let v_ = 99999i32; c.m.w32(fp.wrapping_add(16), (v_) as u32); }
        l_plan_4 = (p_ai_1).wrapping_add(744);
        { let v_ = 0i32; let a_ = (l_seq_3).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (l_seq_3).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (p_ai_1).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
        { let a_ = (l_seq_3).wrapping_add(112); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
        if ((((((c.m.r32((l_seq_3).wrapping_add(112)) as i32)) < (0i32)) as i32)) != 0) {
            { let v_ = { let a_ = (fp).wrapping_add(((((if ((((((c.m.r32((p_ai_1).wrapping_add(8)) as i32)) >= (0i32)) as i32)) != 0) { rt::div_i32((c.m.r32((p_ai_1).wrapping_add(8)) as i32), 6i32) } else { 0i32 })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (l_seq_3).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
        }
        {
            l_i_6 = 0i32;
            'l1: loop {
                if !(((((l_i_6) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_entry_5 = ((l_seq_3).wrapping_add(20)).wrapping_add(((((c.m.r32((l_seq_3).wrapping_add(4)) as i32)) as u32)).wrapping_mul(8));
                    if (((((((c.m.r8(((p_rule_2).wrapping_add(4)).wrapping_add((((l_i_6).wrapping_add(16i32)) as u32)))) as i32)) == (255i32)) as i32)) != 0) {
                        break 'l1;
                    }
                    if (((((((((((((((c.m.r8(((p_rule_2).wrapping_add(4)).wrapping_add((((l_i_6).wrapping_add(16i32)) as u32)))) as i32)) == (2i32)) as i32)) != 0) && ((((((c.m.r32((l_seq_3).wrapping_add(112)) as i32)) > (0i32)) as i32)) != 0)) as i32)) != 0) && (((((((c.m.r8(((p_rule_2).wrapping_add(8)).wrapping_add((((l_i_6).wrapping_add(16i32)) as u32)))) as i32)) == (128i32)) as i32)) != 0)) as i32)) != 0) {
                        break 'c2;
                    }
                    if (((((((c.m.r8(((p_rule_2).wrapping_add(4)).wrapping_add((((l_i_6).wrapping_add(16i32)) as u32)))) as i32)) == (61i32)) as i32)) != 0) {
                        { let v_ = 90i32; let a_ = (l_plan_4).wrapping_add(164); c.m.w32(a_, (v_) as u32); };
                    }
                    { let v_ = ((c.m.r8(((p_rule_2).wrapping_add(4)).wrapping_add((((l_i_6).wrapping_add(16i32)) as u32)))) as u32); let a_ = l_entry_5; c.m.w32(a_, v_); };
                    { let v_ = (((((c.m.r8(((p_rule_2).wrapping_add(8)).wrapping_add((((l_i_6).wrapping_add(16i32)) as u32)))) as i32)).wrapping_add(129i32)) as i8); let a_ = (l_entry_5).wrapping_add(4); c.m.w8(a_, (v_) as u8); };
                    { let a_ = (l_seq_3).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAiSeq_PushRule", &PROBE_N, &[], true); }
    r_
}
