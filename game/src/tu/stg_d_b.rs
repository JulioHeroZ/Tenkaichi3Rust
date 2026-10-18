#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn StgModel_InitStage(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgModel_InitStage", &PROBE_N, &[], false); }
    let mut l_file_1: u32 = 0;
    let mut l_stage_2: u32 = 0;
    let r_ = 'f: {
    l_file_1 = { let a_ = (c.m.r32(sym::gCommonRes)).wrapping_add(36); c.m.r32(a_) };
    l_stage_2 = (l_file_1).wrapping_add((rt::div_u32(c.m.r32((l_file_1).wrapping_add((((1i32) as u32)).wrapping_mul(4))), ((4i32) as u32))).wrapping_mul(((4i32) as u32)));
    F::BtlStage_Init(c, l_stage_2, l_stage_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgModel_InitStage", &PROBE_N, &[], true); }
    r_
}

pub fn StgModel_ResetAnims(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgModel_ResetAnims", &PROBE_N, &[], false); }
    let mut l_data_1: u32 = 0;
    let mut l_def_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_anim_4: i32 = 0;
    let mut l_track_5: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gStgAnimTable)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    l_data_1 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) };
    l_def_2 = c.m.r32((l_data_1).wrapping_add(12));
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < ((c.m.r32((l_data_1).wrapping_add(8)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                l_anim_4 = (c.m.r32((l_def_2).wrapping_add(8)) as i32);
                if (((((l_anim_4) <= (0i32)) as i32)) != 0) {
                    { let o_ = l_def_2; let n_ = (o_).wrapping_add(64); l_def_2 = n_; };
                    break 'c2;
                }
                if ((((((((!(((((c.m.r32((l_def_2).wrapping_add(4)) as i32)) & (1073741824i32))) != 0)) as i32)) != 0) && ((F::BtlStage_IsObjBroken(c, l_i_3)) != 0)) as i32)) != 0) {
                    { let o_ = l_def_2; let n_ = (o_).wrapping_add(64); l_def_2 = n_; };
                    break 'c2;
                }
                {
                    l_track_5 = { let a0_ = c.m.r32(sym::gStgAnimTable); ((((F::StgOfsTable_Get(c, a0_, ((l_anim_4).wrapping_mul(3i32)).wrapping_sub(1i32))) as i64 as u64)) as u32) };
                    { let v_ = rt::fmul(F::Rand_FloatRange(c, F32(0x00000000), F32(0x3f800000)), rt::i2f((c.m.r16((l_track_5).wrapping_add(12))) as i32)); let a_ = (l_def_2).wrapping_add(28); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_def_2; let n_ = (o_).wrapping_add(64); l_def_2 = n_; };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgModel_ResetAnims", &PROBE_N, &[], true); }
    r_
}

pub fn StgModel_BindAnims(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgModel_BindAnims", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_file_1: u32 = 0;
    let mut l_node_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_count_4: i32 = 0;
    let r_ = 'f: {
        l_file_1 = { let a_ = (c.m.r32(sym::gCommonRes)).wrapping_add(36); c.m.r32(a_) };
        if (((((l_file_1) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if (((((c.m.r32((l_file_1).wrapping_add((((10i32) as u32)).wrapping_mul(4)))) == (c.m.r32((l_file_1).wrapping_add((((9i32) as u32)).wrapping_mul(4))))) as i32)) != 0) {
            { let v_ = ((0i32) as u32); let a_ = sym::gStgAnimTable; c.m.w32(a_, v_); };
            break 'f;
        }
        { let v_ = (l_file_1).wrapping_add((rt::div_u32(c.m.r32((l_file_1).wrapping_add((((9i32) as u32)).wrapping_mul(4))), ((4i32) as u32))).wrapping_mul(((4i32) as u32))); let a_ = sym::gStgAnimTable; c.m.w32(a_, v_); };
        { let a0_ = c.m.r32(sym::gStgAnimTable); F::StgOfsTable_Relocate(c, a0_) };
        l_count_4 = { let a0_ = c.m.r32(sym::gStgAnimTable); F::StgOfsTable_GetCount(c, a0_) };
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < (l_count_4)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = { let a0_ = c.m.r32(sym::gStgAnimTable); ((((F::StgOfsTable_Get(c, a0_, l_i_3)) as i64 as u64)) as u32) }; let a_ = fp; c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
                    l_node_2 = { let a0_ = c.m.r32(sym::gStgAnimTable); ((((F::StgOfsTable_Get(c, a0_, (l_i_3).wrapping_add(1i32))) as i64 as u64)) as u32) };
                    { let a0_ = (l_node_2).wrapping_add((rt::div_u32(c.m.r32((l_node_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))), ((4i32) as u32))).wrapping_mul(((4i32) as u32))); F::Vu1Node_OrFlags(c, a0_, ((13440i32) as u32), ((14080i32) as u32)) };
                }
                { let r_ = 3i32; let n_ = (l_i_3).wrapping_add(r_); l_i_3 = n_; };
            }
        }
        F::StgModel_ResetAnims(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgModel_BindAnims", &PROBE_N, &[], true); }
    r_
}

pub fn StgModel_ClearAnims(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgModel_ClearAnims", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = ((0i32) as u32); let a_ = sym::gStgAnimTable; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgModel_ClearAnims", &PROBE_N, &[], true); }
    r_
}

pub fn StgModel_UpdateAnims(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgModel_UpdateAnims", &PROBE_N, &[], false); }
    let mut l_data_1: u32 = 0;
    let mut l_def_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_anim_4: i32 = 0;
    let mut l_track_5: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gStgAnimTable)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
        break 'f;
    }
    l_data_1 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) };
    l_def_2 = c.m.r32((l_data_1).wrapping_add(12));
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < ((c.m.r32((l_data_1).wrapping_add(8)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                l_anim_4 = (c.m.r32((l_def_2).wrapping_add(8)) as i32);
                if (((((l_anim_4) <= (0i32)) as i32)) != 0) {
                    { let o_ = l_def_2; let n_ = (o_).wrapping_add(64); l_def_2 = n_; };
                    break 'c2;
                }
                if ((((((((!(((((c.m.r32((l_def_2).wrapping_add(4)) as i32)) & (1073741824i32))) != 0)) as i32)) != 0) && ((F::BtlStage_IsObjBroken(c, l_i_3)) != 0)) as i32)) != 0) {
                    { let o_ = l_def_2; let n_ = (o_).wrapping_add(64); l_def_2 = n_; };
                    break 'c2;
                }
                { let r_ = F32(0x40000000); let a_ = (l_def_2).wrapping_add(28); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                l_track_5 = { let a0_ = c.m.r32(sym::gStgAnimTable); ((((F::StgOfsTable_Get(c, a0_, ((l_anim_4).wrapping_mul(3i32)).wrapping_sub(1i32))) as i64 as u64)) as u32) };
                if (((rt::flt(rt::i2f((c.m.r16((l_track_5).wrapping_add(12))) as i32), F32(c.m.r32((l_def_2).wrapping_add(28)))) as i32)) != 0) {
                    { let v_ = F32(0x00000000); let a_ = (l_def_2).wrapping_add(28); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_def_2; let n_ = (o_).wrapping_add(64); l_def_2 = n_; };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgModel_UpdateAnims", &PROBE_N, &[], true); }
    r_
}
