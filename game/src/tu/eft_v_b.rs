#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EftImpact_SpawnParts(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32, mut p_set_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftImpact_SpawnParts", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32, (p_set_3) as u32], false); }
    let mut l_w_4: u32 = 0;
    let mut l_type_5: i32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_g_7: u32 = 0;
    let mut l_flags_8: i32 = 0;
    let r_ = 'f: {
    l_w_4 = c.m.r32((p_task_2).wrapping_add(56));
    {
        l_type_5 = 0i32;
        'l1: loop {
            if !(((((l_type_5) < (19i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((({ let a_ = c.m.r32(p_set_3); c.m.r32(a_) }) & ((((1i32).wrapping_shl((l_type_5) as u32)) as u32)))) != 0) {
                    l_g_7 = c.m.r32(((p_set_3).wrapping_add(12)).wrapping_add((((l_type_5) as u32)).wrapping_mul(16)));
                    {
                        l_i_6 = 0i32;
                        'l3: loop {
                            if !(((((l_i_6) < (((c.m.r8((l_g_7).wrapping_add(1))) as i32))) as i32)) != 0) { break 'l3; }
                            'c4: {
                                l_flags_8 = { let a5_ = (((c.m.r32((l_w_4).wrapping_add(1344)) as i32)) & (1i32)); let a6_ = (((c.m.r32((l_w_4).wrapping_add(1344)) as i32)) & (4i32)); let a7_ = ((c.m.r8((l_w_4).wrapping_add(1348))) as i32); F::EftEmit_GetFlagsFromMask(c, p_set_3, (l_w_4).wrapping_add(36), p_objId_1, l_type_5, l_i_6, a5_, a6_, a7_) };
                                if (((((c.m.r32((l_w_4).wrapping_add(1344)) as i32)) & (8i32))) != 0) {
                                    l_flags_8 = 2i32;
                                }
                                if ((l_flags_8) != 0) {
                                    { let a11_ = F32(c.m.r32((l_w_4).wrapping_add(1352))); F::EftEmit_Spawn(c, p_set_3, (l_w_4).wrapping_add(36), (l_w_4).wrapping_add(752), (l_w_4).wrapping_add(16), l_w_4, p_objId_1, 0i32, 2i32, l_type_5, l_i_6, l_flags_8, a11_) };
                                }
                            }
                            { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                        }
                    }
                }
            }
            { let o_ = l_type_5; let n_ = (o_).wrapping_add(1); l_type_5 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftImpact_SpawnParts", &PROBE_N, &[], true); }
    r_
}

pub fn EftImpact_GetSet(c: &mut Ctx, mut p_group_1: i32, mut p_id_2: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftImpact_GetSet", &PROBE_N, &[(p_group_1) as u32, (p_id_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_group_1) == (0i32)) as i32)) != 0) {
        's1: {
            let sv3: u32 = p_id_2;
            match sv3 {
                4u32 | 5u32 | 6u32 => {
                    break 'f ((c.m.r32(sym::gEftImpact)).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(800));
                }
                7u32 | 8u32 | 9u32 | 10u32 => {
                    break 'f ((c.m.r32(sym::gEftImpact)).wrapping_add(8)).wrapping_add((((2i32) as u32)).wrapping_mul(800));
                }
                11u32 | 12u32 | 13u32 => {
                    break 'f ((c.m.r32(sym::gEftImpact)).wrapping_add(8)).wrapping_add((((3i32) as u32)).wrapping_mul(800));
                }
                14u32 => {
                    break 'f ((c.m.r32(sym::gEftImpact)).wrapping_add(8)).wrapping_add((((4i32) as u32)).wrapping_mul(800));
                }
                15u32 | 16u32 => {
                    break 'f ((c.m.r32(sym::gEftImpact)).wrapping_add(8)).wrapping_add((((5i32) as u32)).wrapping_mul(800));
                }
                _ => {
                    break 'f ((c.m.r32(sym::gEftImpact)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(800));
                }
            }
        }
    } else {
        's2: {
            let sv4: u32 = p_id_2;
            match sv4 {
                0u32 => {
                    break 'f ((c.m.r32(sym::gEftImpact)).wrapping_add(4808)).wrapping_add((((0i32) as u32)).wrapping_mul(800));
                }
                1u32 => {
                    break 'f ((c.m.r32(sym::gEftImpact)).wrapping_add(4808)).wrapping_add((((1i32) as u32)).wrapping_mul(800));
                }
                2u32 => {
                    break 'f ((c.m.r32(sym::gEftImpact)).wrapping_add(4808)).wrapping_add((((2i32) as u32)).wrapping_mul(800));
                }
                3u32 | 4u32 => {
                    break 'f ((c.m.r32(sym::gEftImpact)).wrapping_add(4808)).wrapping_add((((3i32) as u32)).wrapping_mul(800));
                }
                5u32 | 6u32 => {
                    break 'f ((c.m.r32(sym::gEftImpact)).wrapping_add(4808)).wrapping_add((((4i32) as u32)).wrapping_mul(800));
                }
                7u32 | 8u32 => {
                    break 'f ((c.m.r32(sym::gEftImpact)).wrapping_add(4808)).wrapping_add((((5i32) as u32)).wrapping_mul(800));
                }
                9u32 | 10u32 => {
                    break 'f ((c.m.r32(sym::gEftImpact)).wrapping_add(4808)).wrapping_add((((6i32) as u32)).wrapping_mul(800));
                }
                11u32 | 12u32 => {
                    break 'f ((c.m.r32(sym::gEftImpact)).wrapping_add(4808)).wrapping_add((((7i32) as u32)).wrapping_mul(800));
                }
                _ => {
                    break 'f ((c.m.r32(sym::gEftImpact)).wrapping_add(4808)).wrapping_add((((0i32) as u32)).wrapping_mul(800));
                }
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftImpact_GetSet", &PROBE_N, &[], true); }
    r_
}

pub fn EftImpact_GetNodeMask(c: &mut Ctx, mut p_group_1: i32, mut p_id_2: i32) -> u8 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftImpact_GetNodeMask", &PROBE_N, &[(p_group_1) as u32, (p_id_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let r_ = 'f: {
        if (((((p_group_1) == (0i32)) as i32)) != 0) {
            rt::memset(c, fp, 0, 17);
            { let v_ = ((1i32) as u8); c.m.w8(fp.wrapping_add(0), v_); }
            { let v_ = ((1i32) as u8); c.m.w8(fp.wrapping_add(1), v_); }
            { let v_ = ((8i32) as u8); c.m.w8(fp.wrapping_add(2), v_); }
            { let v_ = ((16i32) as u8); c.m.w8(fp.wrapping_add(3), v_); }
            { let v_ = ((1i32) as u8); c.m.w8(fp.wrapping_add(4), v_); }
            { let v_ = ((2i32) as u8); c.m.w8(fp.wrapping_add(5), v_); }
            { let v_ = ((8i32) as u8); c.m.w8(fp.wrapping_add(6), v_); }
            { let v_ = ((1i32) as u8); c.m.w8(fp.wrapping_add(7), v_); }
            { let v_ = ((2i32) as u8); c.m.w8(fp.wrapping_add(8), v_); }
            { let v_ = ((8i32) as u8); c.m.w8(fp.wrapping_add(9), v_); }
            { let v_ = ((16i32) as u8); c.m.w8(fp.wrapping_add(10), v_); }
            { let v_ = ((1i32) as u8); c.m.w8(fp.wrapping_add(11), v_); }
            { let v_ = ((2i32) as u8); c.m.w8(fp.wrapping_add(12), v_); }
            { let v_ = ((8i32) as u8); c.m.w8(fp.wrapping_add(13), v_); }
            { let v_ = ((1i32) as u8); c.m.w8(fp.wrapping_add(14), v_); }
            { let v_ = ((1i32) as u8); c.m.w8(fp.wrapping_add(15), v_); }
            { let v_ = ((2i32) as u8); c.m.w8(fp.wrapping_add(16), v_); }
            break 'f c.m.r8((fp).wrapping_add(((p_id_2) as u32)));
        } else {
            rt::memset(c, fp.wrapping_add(20), 0, 13);
            { let v_ = ((1i32) as u8); c.m.w8(fp.wrapping_add(20).wrapping_add(0), v_); }
            { let v_ = ((1i32) as u8); c.m.w8(fp.wrapping_add(20).wrapping_add(1), v_); }
            { let v_ = ((1i32) as u8); c.m.w8(fp.wrapping_add(20).wrapping_add(2), v_); }
            { let v_ = ((1i32) as u8); c.m.w8(fp.wrapping_add(20).wrapping_add(3), v_); }
            { let v_ = ((2i32) as u8); c.m.w8(fp.wrapping_add(20).wrapping_add(4), v_); }
            { let v_ = ((1i32) as u8); c.m.w8(fp.wrapping_add(20).wrapping_add(5), v_); }
            { let v_ = ((2i32) as u8); c.m.w8(fp.wrapping_add(20).wrapping_add(6), v_); }
            { let v_ = ((1i32) as u8); c.m.w8(fp.wrapping_add(20).wrapping_add(7), v_); }
            { let v_ = ((2i32) as u8); c.m.w8(fp.wrapping_add(20).wrapping_add(8), v_); }
            { let v_ = ((1i32) as u8); c.m.w8(fp.wrapping_add(20).wrapping_add(9), v_); }
            { let v_ = ((2i32) as u8); c.m.w8(fp.wrapping_add(20).wrapping_add(10), v_); }
            { let v_ = ((1i32) as u8); c.m.w8(fp.wrapping_add(20).wrapping_add(11), v_); }
            { let v_ = ((2i32) as u8); c.m.w8(fp.wrapping_add(20).wrapping_add(12), v_); }
            break 'f c.m.r8((fp.wrapping_add(20)).wrapping_add(((p_id_2) as u32)));
        }
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftImpact_GetNodeMask", &PROBE_N, &[], true); }
    r_
}

pub fn EftImpact_Init(c: &mut Ctx, mut p_task_1: u32, mut p_init_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftImpact_Init", &PROBE_N, &[(p_task_1) as u32, (p_init_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let mut l_arg_4: u32 = 0;
    let mut l_pos_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_chr_7: u32 = 0;
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    l_arg_4 = (p_init_2).wrapping_add(16);
    l_pos_5 = (l_w_3).wrapping_add(16);
    { let _ = F::memset(c, l_w_3, 0i32, 1376u32); };
    { let v_ = (((c.m.r32(p_init_2) as i32)) as u8); let a_ = (l_w_3).wrapping_add(1349); c.m.w8(a_, v_); };
    { let v_ = (((c.m.r32((l_arg_4).wrapping_add(32)) as i32)) as u8); let a_ = (l_w_3).wrapping_add(1350); c.m.w8(a_, v_); };
    { let v_ = (((c.m.r32((l_arg_4).wrapping_add(36)) as i32)) as u8); let a_ = (l_w_3).wrapping_add(1351); c.m.w8(a_, v_); };
    F::Vec4_Copy(c, l_pos_5, l_arg_4);
    F::Vec4_Copy(c, l_w_3, ((p_init_2).wrapping_add(16)).wrapping_add(16));
    { let v_ = F32(0x3f800000); let a_ = (l_w_3).wrapping_add(12); c.m.w32(a_, (v_).0); };
    F::Vec3_Normalize(c, l_w_3, l_w_3);
    if (((((((c.m.r8((l_w_3).wrapping_add(1349))) as i32)) == (0i32)) as i32)) != 0) {
        if (((rt::fle(F32(c.m.r32((p_init_2).wrapping_add(4))), F32(0x00000000)) as i32)) != 0) {
            l_chr_7 = ((c.m.r8((l_w_3).wrapping_add(1351))) as u32);
            if (((((l_chr_7) >= (((2i32) as u32))) as i32)) != 0) {
                l_chr_7 = ((0i32) as u32);
            }
            { let v_ = F::BtlScene_GetCharScale(c, ((l_chr_7) as i32)); let a_ = (l_w_3).wrapping_add(1352); c.m.w32(a_, (v_).0); };
            if (((rt::flt(F32(c.m.r32((l_w_3).wrapping_add(1352))), F32(0x3f333333)) as i32)) != 0) {
                { let v_ = F32(0x3f333333); let a_ = (l_w_3).wrapping_add(1352); c.m.w32(a_, (v_).0); };
            }
            { let r_ = F32(0x3f400000); let a_ = (l_w_3).wrapping_add(1352); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((((((c.m.r8((l_w_3).wrapping_add(1350))) as i32)) == (1i32)) as i32)) != 0) {
                { let r_ = F32(0x3f999999); let a_ = (l_w_3).wrapping_add(1352); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            }
        } else {
            { let v_ = F32(c.m.r32((p_init_2).wrapping_add(4))); let a_ = (l_w_3).wrapping_add(1352); c.m.w32(a_, (v_).0); };
        }
    } else {
        { let v_ = F32(c.m.r32((p_init_2).wrapping_add(4))); let a_ = (l_w_3).wrapping_add(1352); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((p_init_2).wrapping_add(8))); let a_ = (l_w_3).wrapping_add(1356); c.m.w32(a_, (v_).0); };
    }
    { let v_ = { let a0_ = (c.m.r32(p_init_2) as i32); let a1_ = ((c.m.r8((l_w_3).wrapping_add(1350))) as u32); F::EftImpact_GetSet(c, a0_, a1_) }; let a_ = (l_w_3).wrapping_add(32); c.m.w32(a_, v_); };
    { let a0_ = c.m.r32((l_w_3).wrapping_add(32)); F::EftEmit_InitState(c, a0_, (l_w_3).wrapping_add(36)) };
    { let v_ = rt::i2f({ let a0_ = c.m.r32((l_w_3).wrapping_add(32)); F::EftEmit_GetEndFrames(c, a0_) }); let a_ = (l_w_3).wrapping_add(1364); c.m.w32(a_, (v_).0); };
    { let v_ = { let a0_ = ((c.m.r8((l_w_3).wrapping_add(1349))) as i32); let a1_ = ((c.m.r8((l_w_3).wrapping_add(1350))) as i32); F::EftImpact_GetNodeMask(c, a0_, a1_) }; let a_ = (l_w_3).wrapping_add(1348); c.m.w8(a_, v_); };
    {
        l_i_6 = 0i32;
        'l1: loop {
            if !(((((l_i_6) < (6i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((((c.m.r8((l_w_3).wrapping_add(1348))) as i32)).wrapping_shr((l_i_6) as u32)) & (1i32))) != 0) {
                    F::EftEmit_SetNode(c, (l_w_3).wrapping_add(752), l_i_6, (1i32).wrapping_neg(), l_pos_5);
                }
            }
            { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
        }
    }
    { let a_ = (c.m.r32(sym::gEftImpact)).wrapping_add((((c.m.r8((l_w_3).wrapping_add(1349))) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftImpact_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftImpact_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftImpact_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    { let a0_ = c.m.r32((l_w_2).wrapping_add(32)); F::EftEmit_TermState(c, a0_, (l_w_2).wrapping_add(36)) };
    { let a_ = (c.m.r32(sym::gEftImpact)).wrapping_add((((c.m.r8((l_w_2).wrapping_add(1349))) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftImpact_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftImpact_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftImpact_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (({ let a0_ = ((c.m.r8((l_w_2).wrapping_add(1351))) as i32); F::BtlScene_IsEffectStopped(c, a0_, 2i32) }) != 0) {
        break 'f;
    }
    if (((((((c.m.r8((l_w_2).wrapping_add(1349))) as i32)) != (0i32)) as i32)) != 0) {
        { let r_ = F32(c.m.r32((l_w_2).wrapping_add(1356))); let a_ = ((l_w_2).wrapping_add(16)).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (6i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((((c.m.r8((l_w_2).wrapping_add(1348))) as i32)).wrapping_shr((l_i_3) as u32)) & (1i32))) != 0) {
                    F::EftEmit_SetNodePos(c, (l_w_2).wrapping_add(752), l_i_3, (l_w_2).wrapping_add(16));
                }
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    { let a0_ = ((c.m.r8((l_w_2).wrapping_add(1351))) as i32); let a2_ = c.m.r32((l_w_2).wrapping_add(32)); F::EftImpact_SpawnParts(c, a0_, p_task_1, a2_) };
    if (((((c.m.r32((l_w_2).wrapping_add(1344)) as i32)) & (1i32))) != 0) {
        { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(1360); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
    if (((((c.m.r32((l_w_2).wrapping_add(1344)) as i32)) & (2i32))) != 0) {
        F::BtlTask_SetDead(c, p_task_1);
    } else {
        if (((((c.m.r32((l_w_2).wrapping_add(1344)) as i32)) & (1i32))) != 0) {
            if (((((((((c.m.r32((l_w_2).wrapping_add(1344)) as i32)) & (4i32))) != 0) || (((rt::fge(F32(c.m.r32((l_w_2).wrapping_add(1360))), F32(c.m.r32((l_w_2).wrapping_add(1364)))) as i32)) != 0)) as i32)) != 0) {
                { let r_ = 2i32; let a_ = (l_w_2).wrapping_add(1344); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftImpact_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftImpact_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftImpact_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((F::BtlScene_IsTimeStopped(c)) != 0) {
        break 'f;
    }
    if (((((c.m.r32((l_w_2).wrapping_add(1344)) as i32)) & (16i32))) != 0) {
        if ((((!(({ let a0_ = c.m.r32((l_w_2).wrapping_add(32)); F::EftEmit_UpdateAlive(c, a0_, (l_w_2).wrapping_add(36)) }) != 0)) as i32)) != 0) {
            { let r_ = 1i32; let a_ = (l_w_2).wrapping_add(1344); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        { let v_ = ((0i32) as u8); let a_ = (l_w_2).wrapping_add(1348); c.m.w8(a_, v_); };
    }
    { let r_ = 16i32; let a_ = (l_w_2).wrapping_add(1344); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftImpact_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftImpact_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftImpact_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(1344)) as i32)) & (8i32))) != 0)) as i32)) != 0) {
        { let r_ = 8i32; let a_ = (l_w_2).wrapping_add(1344); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let a0_ = c.m.r32((l_w_2).wrapping_add(32)); F::EftEmit_KillAll(c, a0_, (l_w_2).wrapping_add(36)) };
    }
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftImpact_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftImpact_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftImpact_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftImpact_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftImpactMgr_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftImpactMgr_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((11208u32) as i32)) }; let a_ = sym::gEftImpact; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gEftImpact); F::memset(c, a0_, 0i32, 11208u32) }; };
        { let v_ = F::BtlTask_CreateChildList(c, p_task_1, ((((30i32) as i64 as u64)) as u32), ((((((1376u32) as i32)) as i64 as u64)) as u32)); let a_ = sym::gEftImpactList; c.m.w32(a_, v_); };
        {
            rt::memset(c, fp, 0, 24);
            { let v_ = 12i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
            { let v_ = 32i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
            { let v_ = 37i32; c.m.w32(fp.wrapping_add(8), (v_) as u32); }
            { let v_ = 76i32; c.m.w32(fp.wrapping_add(12), (v_) as u32); }
            { let v_ = 102i32; c.m.w32(fp.wrapping_add(16), (v_) as u32); }
            { let v_ = 115i32; c.m.w32(fp.wrapping_add(20), (v_) as u32); }
            {
                l_i_2 = 0i32;
                'l1: loop {
                    if !(((((l_i_2) < (6i32)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        { let a1_ = ((c.m.r32(sym::gEftImpact)).wrapping_add(8)).wrapping_add((((l_i_2) as u32)).wrapping_mul(800)); let a5_ = (c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32); F::EftEmit_LoadSet(c, ((0i32) as u32), a1_, ((0i32) as u32), ((0i32) as u32), 1i32, a5_) };
                    }
                    { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                }
            }
        }
        {
            rt::memset(c, fp.wrapping_add(24), 0, 32);
            { let v_ = 136i32; c.m.w32(fp.wrapping_add(24).wrapping_add(0), (v_) as u32); }
            { let v_ = 145i32; c.m.w32(fp.wrapping_add(24).wrapping_add(4), (v_) as u32); }
            { let v_ = 164i32; c.m.w32(fp.wrapping_add(24).wrapping_add(8), (v_) as u32); }
            { let v_ = 185i32; c.m.w32(fp.wrapping_add(24).wrapping_add(12), (v_) as u32); }
            { let v_ = 242i32; c.m.w32(fp.wrapping_add(24).wrapping_add(16), (v_) as u32); }
            { let v_ = 282i32; c.m.w32(fp.wrapping_add(24).wrapping_add(20), (v_) as u32); }
            { let v_ = 311i32; c.m.w32(fp.wrapping_add(24).wrapping_add(24), (v_) as u32); }
            { let v_ = 344i32; c.m.w32(fp.wrapping_add(24).wrapping_add(28), (v_) as u32); }
            {
                l_i_2 = 0i32;
                'l3: loop {
                    if !(((((l_i_2) < (8i32)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        { let a1_ = ((c.m.r32(sym::gEftImpact)).wrapping_add(4808)).wrapping_add((((l_i_2) as u32)).wrapping_mul(800)); let a5_ = (c.m.r32((fp.wrapping_add(24)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32); F::EftEmit_LoadSet(c, ((0i32) as u32), a1_, ((0i32) as u32), ((0i32) as u32), 1i32, a5_) };
                    }
                    { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftImpactMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftImpactMgr_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftImpactMgr_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (6i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gEftImpact)).wrapping_add(8)).wrapping_add((((l_i_2) as u32)).wrapping_mul(800)); F::EftEmit_FreeSet(c, a0_) };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    {
        l_i_2 = 0i32;
        'l3: loop {
            if !(((((l_i_2) < (8i32)) as i32)) != 0) { break 'l3; }
            'c4: {
                { let a0_ = ((c.m.r32(sym::gEftImpact)).wrapping_add(4808)).wrapping_add((((l_i_2) as u32)).wrapping_mul(800)); F::EftEmit_FreeSet(c, a0_) };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftImpact); F::BtlPool_Free(c, a0_, a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftImpact; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftImpactMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftImpactMgr_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftImpactMgr_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (6i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gEftImpact)).wrapping_add(8)).wrapping_add((((l_i_2) as u32)).wrapping_mul(800)); F::EftEmit_BeginFrame(c, a0_) };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    {
        l_i_2 = 0i32;
        'l3: loop {
            if !(((((l_i_2) < (8i32)) as i32)) != 0) { break 'l3; }
            'c4: {
                { let a0_ = ((c.m.r32(sym::gEftImpact)).wrapping_add(4808)).wrapping_add((((l_i_2) as u32)).wrapping_mul(800)); F::EftEmit_BeginFrame(c, a0_) };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftImpactMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftImpactMgr_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftImpactMgr_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftImpactMgr_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftImpact_SpawnHit(c: &mut Ctx, mut p_arg_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftImpact_SpawnHit", &PROBE_N, &[(p_arg_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let r_ = 'f: {
        { let s_ = p_arg_1; let d_ = (fp).wrapping_add(16); rt::memcpy(c, d_, s_, 48); };
        { let v_ = 0i32; let a_ = fp; c.m.w32(a_, (v_) as u32); };
        { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        break 'f ((({ let a0_ = c.m.r32(sym::gEftImpactList); F::BtlTaskList_AddTail(c, a0_, sym::gEftImpactClass, fp) }) != (((0i32) as u32))) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftImpact_SpawnHit", &PROBE_N, &[], true); }
    r_
}

pub fn EftImpact_SpawnHitScaled(c: &mut Ctx, mut p_arg_1: u32, mut p_scale_2: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftImpact_SpawnHitScaled", &PROBE_N, &[(p_arg_1) as u32, (p_scale_2).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let r_ = 'f: {
        { let v_ = p_scale_2; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let s_ = p_arg_1; let d_ = (fp).wrapping_add(16); rt::memcpy(c, d_, s_, 48); };
        { let v_ = 0i32; let a_ = fp; c.m.w32(a_, (v_) as u32); };
        { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        break 'f ((({ let a0_ = c.m.r32(sym::gEftImpactList); F::BtlTaskList_AddTail(c, a0_, sym::gEftImpactClass, fp) }) != (((0i32) as u32))) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftImpact_SpawnHitScaled", &PROBE_N, &[], true); }
    r_
}

pub fn EftImpact_SpawnBlast(c: &mut Ctx, mut p_arg_1: u32, mut p_scale_2: F32, mut p_rise_3: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftImpact_SpawnBlast", &PROBE_N, &[(p_arg_1) as u32, (p_scale_2).0, (p_rise_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let r_ = 'f: {
        { let v_ = 1i32; let a_ = fp; c.m.w32(a_, (v_) as u32); };
        { let v_ = p_scale_2; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_rise_3; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let s_ = p_arg_1; let d_ = (fp).wrapping_add(16); rt::memcpy(c, d_, s_, 48); };
        break 'f ((({ let a0_ = c.m.r32(sym::gEftImpactList); F::BtlTaskList_AddTail(c, a0_, sym::gEftImpactClass, fp) }) != (((0i32) as u32))) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftImpact_SpawnBlast", &PROBE_N, &[], true); }
    r_
}
