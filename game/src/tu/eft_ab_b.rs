#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EftTransform_SpawnParts(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32, mut p_set_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTransform_SpawnParts", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32, (p_set_3) as u32], false); }
    let mut l_w_4: u32 = 0;
    let mut l_part_5: i32 = 0;
    let mut l_sub_6: i32 = 0;
    let mut l_def_7: u32 = 0;
    let mut l_spawn_8: i32 = 0;
    let r_ = 'f: {
    l_w_4 = c.m.r32((p_task_2).wrapping_add(56));
    {
        l_part_5 = 0i32;
        'l1: loop {
            if !(((((l_part_5) < (19i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((({ let a_ = c.m.r32(p_set_3); c.m.r32(a_) }) & ((((1i32).wrapping_shl((l_part_5) as u32)) as u32)))) != 0) {
                    l_def_7 = c.m.r32(((p_set_3).wrapping_add(12)).wrapping_add((((l_part_5) as u32)).wrapping_mul(16)));
                    {
                        l_sub_6 = 0i32;
                        'l3: loop {
                            if !(((((l_sub_6) < (((c.m.r8((l_def_7).wrapping_add(1))) as i32))) as i32)) != 0) { break 'l3; }
                            'c4: {
                                l_spawn_8 = { let a5_ = (((c.m.r32((l_w_4).wrapping_add(1360)) as i32)) & (1i32)); let a7_ = (c.m.r32((l_w_4).wrapping_add(1368)) as i32); F::EftEmit_GetFlagsFromMask(c, p_set_3, (l_w_4).wrapping_add(44), p_objId_1, l_part_5, l_sub_6, a5_, 0i32, a7_) };
                                if (((((l_spawn_8) != (0i32)) as i32)) != 0) {
                                    { let a11_ = F32(c.m.r32((l_w_4).wrapping_add(1364))); F::EftEmit_Spawn(c, p_set_3, (l_w_4).wrapping_add(44), (l_w_4).wrapping_add(768), l_w_4, (l_w_4).wrapping_add(16), p_objId_1, 0i32, 5i32, l_part_5, l_sub_6, l_spawn_8, a11_) };
                                }
                            }
                            { let o_ = l_sub_6; let n_ = (o_).wrapping_add(1); l_sub_6 = n_; };
                        }
                    }
                }
            }
            { let o_ = l_part_5; let n_ = (o_).wrapping_add(1); l_part_5 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTransform_SpawnParts", &PROBE_N, &[], true); }
    r_
}

pub fn EftTransform_GetSet(c: &mut Ctx, mut p_kind_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTransform_GetSet", &PROBE_N, &[(p_kind_1) as u32], false); }
    let mut l_set_2: u32 = 0;
    let r_ = 'f: {
    l_set_2 = ((0i32) as u32);
    's1: {
        let sv3: i32 = p_kind_1;
        match sv3 {
            0i32 => {
                l_set_2 = (c.m.r32(sym::gEftTransform)).wrapping_add((((0i32) as u32)).wrapping_mul(800));
                break 's1;
            }
            1i32 | 2i32 | 3i32 | 4i32 => {
                l_set_2 = (c.m.r32(sym::gEftTransform)).wrapping_add((((1i32) as u32)).wrapping_mul(800));
                break 's1;
            }
            5i32 => {
                l_set_2 = (c.m.r32(sym::gEftTransform)).wrapping_add((((2i32) as u32)).wrapping_mul(800));
                break 's1;
            }
            6i32 => {
                l_set_2 = (c.m.r32(sym::gEftTransform)).wrapping_add((((3i32) as u32)).wrapping_mul(800));
                break 's1;
            }
            7i32 => {
                l_set_2 = (c.m.r32(sym::gEftTransform)).wrapping_add((((4i32) as u32)).wrapping_mul(800));
                break 's1;
            }
            8i32 => {
                l_set_2 = (c.m.r32(sym::gEftTransform)).wrapping_add((((5i32) as u32)).wrapping_mul(800));
                break 's1;
            }
            9i32 => {
                l_set_2 = (c.m.r32(sym::gEftTransform)).wrapping_add((((6i32) as u32)).wrapping_mul(800));
                break 's1;
            }
            10i32 => {
                l_set_2 = (c.m.r32(sym::gEftTransform)).wrapping_add((((7i32) as u32)).wrapping_mul(800));
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f l_set_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTransform_GetSet", &PROBE_N, &[], true); }
    r_
}

pub fn EftTransform_HasStartGroup(c: &mut Ctx, mut p_kind_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTransform_HasStartGroup", &PROBE_N, &[(p_kind_1) as u32], false); }
    let r_ = 'f: {
    if (((((p_kind_1) < (7i32)) as i32)) != 0) {
        break 'f (((p_kind_1) < (2i32)) as i32);
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTransform_HasStartGroup", &PROBE_N, &[], true); }
    r_
}

pub fn EftTransform_UpdateNodes(c: &mut Ctx, mut p_task_1: u32, mut p_kind_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTransform_UpdateNodes", &PROBE_N, &[(p_task_1) as u32, (p_kind_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_w_3: u32 = 0;
    let mut l_arg_4: u32 = 0;
    let mut l_node_5: i32 = 0;
    let mut l_objId_6: i32 = 0;
    let mut l_partner_7: i32 = 0;
    let mut l_objId_8: i32 = 0;
    let mut l_partner_9: i32 = 0;
    let r_ = 'f: {
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        l_arg_4 = (l_w_3).wrapping_add(32);
        's1: {
            let sv10: i32 = p_kind_2;
            match sv10 {
                5i32 => {
                    if (({ let a0_ = (c.m.r32(l_arg_4) as i32); F::BtlCharApi_ObjTestAttr(c, a0_, ((16384i32) as u64)) }) != 0) {
                        if ((((!(((((c.m.r32((l_w_3).wrapping_add(1360)) as i32)) & (512i32))) != 0)) as i32)) != 0) {
                            { let r_ = 512i32; let a_ = (l_w_3).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        } else {
                            { let r_ = (!(512i32)); let a_ = (l_w_3).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                    }
                    { let a0_ = (c.m.r32(l_arg_4) as i32); F::BtlCharApi_GetNodePos(c, a0_, 54i32, l_w_3) };
                    break 's1;
                }
                6i32 => {
                    if (({ let a0_ = (c.m.r32(l_arg_4) as i32); F::BtlCharApi_ObjTestAttr(c, a0_, ((512i32) as u64)) }) != 0) {
                        { let a2_ = { let a0_ = (c.m.r32(l_arg_4) as i32); F::BtlCharApi_ObjGetAttrKind(c, a0_, ((512i32) as u64)) }; F::EftEmit_SetNode(c, (l_w_3).wrapping_add(768), 0i32, a2_, ((0i32) as u32)) };
                        { let r_ = 1i32; let a_ = (l_w_3).wrapping_add(1368); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    if (({ let a0_ = (c.m.r32(l_arg_4) as i32); F::BtlCharApi_ObjTestAttr(c, a0_, ((1024i32) as u64)) }) != 0) {
                        l_node_5 = { let a0_ = (c.m.r32(l_arg_4) as i32); F::BtlCharApi_ObjGetAttrKind(c, a0_, ((1024i32) as u64)) };
                        F::EftEmit_SetNode(c, (l_w_3).wrapping_add(768), 1i32, l_node_5, ((0i32) as u32));
                        { let r_ = 2i32; let a_ = (l_w_3).wrapping_add(1368); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        { let r_ = 128i32; let a_ = (l_w_3).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        { let a0_ = (c.m.r32(l_arg_4) as i32); F::BtlCharApi_GetNodePos(c, a0_, l_node_5, l_w_3) };
                        { let a0_ = (c.m.r32(l_arg_4) as i32); F::BtlCharApi_GetNodeMtx(c, a0_, l_node_5, fp) };
                        { let a1_ = rt::fneg(F32(c.m.r32(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a2_ = rt::fneg(F32(c.m.r32(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a3_ = rt::fneg(F32(c.m.r32(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); F::Vec4_Set(c, (l_w_3).wrapping_add(16), a1_, a2_, a3_, F32(0x3f800000)) };
                        F::Vec3_Normalize(c, (l_w_3).wrapping_add(16), (l_w_3).wrapping_add(16));
                    }
                    if (({ let a0_ = (c.m.r32(l_arg_4) as i32); F::BtlCharApi_ObjTestAttr(c, a0_, ((4096i32) as u64)) }) != 0) {
                        F::EftEmit_SetNode(c, (l_w_3).wrapping_add(768), 3i32, 46i32, ((0i32) as u32));
                        { let r_ = 8i32; let a_ = (l_w_3).wrapping_add(1368); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    if (({ let a0_ = (c.m.r32(l_arg_4) as i32); F::BtlCharApi_ObjTestAttr(c, a0_, ((8192i32) as u64)) }) != 0) {
                        { let a2_ = { let a0_ = (c.m.r32(l_arg_4) as i32); F::BtlCharApi_ObjGetAttrKind(c, a0_, ((8192i32) as u64)) }; F::EftEmit_SetNode(c, (l_w_3).wrapping_add(768), 4i32, a2_, ((0i32) as u32)) };
                        { let r_ = 16i32; let a_ = (l_w_3).wrapping_add(1368); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    if (((((c.m.r32((l_w_3).wrapping_add(1360)) as i32)) & (128i32))) != 0) {
                        { let a0_ = (c.m.r32(l_arg_4) as i32); F::BtlCharApi_GetNodePos(c, a0_, 54i32, l_w_3) };
                    }
                    break 's1;
                }
                2i32 => {
                    if ((((!(((((c.m.r32((l_w_3).wrapping_add(1360)) as i32)) & (2048i32))) != 0)) as i32)) != 0) {
                        l_objId_6 = (c.m.r32(l_arg_4) as i32);
                        l_partner_7 = F::BtlCharApi_GetPartnerObjId(c, l_objId_6);
                        if (({ let a0_ = (c.m.r32(l_arg_4) as i32); F::BtlCharApi_ObjTestAttr(c, a0_, ((4096i32) as u64)) }) != 0) {
                            { let r_ = 2048i32; let a_ = (l_w_3).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        } else {
                            if (((((((((l_partner_7) >= (0i32)) as i32)) != 0) && ((F::BtlCharApi_ObjTestAttr(c, l_partner_7, ((4096i32) as u64))) != 0)) as i32)) != 0) {
                                l_objId_6 = l_partner_7;
                                { let r_ = 2048i32; let a_ = (l_w_3).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            }
                        }
                        if (((((c.m.r32((l_w_3).wrapping_add(1360)) as i32)) & (2048i32))) != 0) {
                            F::BtlCharApi_GetNodePos(c, l_objId_6, 54i32, l_w_3);
                            F::EftEmit_SetNode(c, (l_w_3).wrapping_add(768), 3i32, (1i32).wrapping_neg(), l_w_3);
                            { let r_ = 8i32; let a_ = (l_w_3).wrapping_add(1368); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                    }
                    break 's1;
                }
                3i32 => {
                    if ((((!(((((c.m.r32((l_w_3).wrapping_add(1360)) as i32)) & (2048i32))) != 0)) as i32)) != 0) {
                        l_objId_8 = (c.m.r32(l_arg_4) as i32);
                        l_partner_9 = F::BtlCharApi_GetPartnerObjId(c, l_objId_8);
                        if (({ let a0_ = (c.m.r32(l_arg_4) as i32); F::BtlCharApi_ObjTestAttr(c, a0_, ((4096i32) as u64)) }) != 0) {
                            { let r_ = 2048i32; let a_ = (l_w_3).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        } else {
                            if (((((((((l_partner_9) >= (0i32)) as i32)) != 0) && ((F::BtlCharApi_ObjTestAttr(c, l_partner_9, ((4096i32) as u64))) != 0)) as i32)) != 0) {
                                l_objId_8 = l_partner_9;
                                { let r_ = 2048i32; let a_ = (l_w_3).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            }
                        }
                        if (((((c.m.r32((l_w_3).wrapping_add(1360)) as i32)) & (2048i32))) != 0) {
                            F::BtlCharApi_GetNodePos(c, l_objId_8, 54i32, l_w_3);
                            F::EftEmit_SetNode(c, (l_w_3).wrapping_add(768), 4i32, (1i32).wrapping_neg(), l_w_3);
                            { let r_ = 16i32; let a_ = (l_w_3).wrapping_add(1368); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                    }
                    break 's1;
                }
                _ => {
                    { let a0_ = (c.m.r32(l_arg_4) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, l_w_3) };
                    break 's1;
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftTransform_UpdateNodes", &PROBE_N, &[], true); }
    r_
}

pub fn EftTransform_OnFlash(c: &mut Ctx, mut p_task_1: u32, mut p_kind_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTransform_OnFlash", &PROBE_N, &[(p_task_1) as u32, (p_kind_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    's1: {
        let sv4: i32 = p_kind_2;
        match sv4 {
            6i32 => {
                { let a0_ = c.m.r32((l_w_3).wrapping_add(40)); F::EftEmit_KillAll(c, a0_, (l_w_3).wrapping_add(44)) };
                break 's1;
            }
            4i32 => {
                F::EftEmit_SetNode(c, (l_w_3).wrapping_add(768), 3i32, 3i32, ((0i32) as u32));
                { let r_ = 8i32; let a_ = (l_w_3).wrapping_add(1368); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTransform_OnFlash", &PROBE_N, &[], true); }
    r_
}

pub fn EftTransform_Init(c: &mut Ctx, mut p_task_1: u32, mut p_req_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTransform_Init", &PROBE_N, &[(p_task_1) as u32, (p_req_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let mut l_arg_4: u32 = 0;
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let _ = F::memset(c, l_w_3, 0i32, 1392u32); };
    l_arg_4 = (l_w_3).wrapping_add(32);
    { let s_ = p_req_2; let d_ = l_arg_4; rt::memcpy(c, d_, s_, 8); };
    { let v_ = rt::fdiv({ let a0_ = (c.m.r32(l_arg_4) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x419acccc)); let a_ = (l_w_3).wrapping_add(1364); c.m.w32(a_, (v_).0); };
    if (((rt::flt(F32(c.m.r32((l_w_3).wrapping_add(1364))), F32(0x3f000000)) as i32)) != 0) {
        { let v_ = F32(0x3f000000); let a_ = (l_w_3).wrapping_add(1364); c.m.w32(a_, (v_).0); };
    }
    { let a0_ = (c.m.r32(l_arg_4) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, l_w_3) };
    F::Vec4_Set(c, (l_w_3).wrapping_add(16), F32(0x00000000), F32(0xbf800000), F32(0x00000000), F32(0x3f800000));
    { let v_ = { let a0_ = (c.m.r32((p_req_2).wrapping_add(4)) as i32); F::EftTransform_GetSet(c, a0_) }; let a_ = (l_w_3).wrapping_add(40); c.m.w32(a_, v_); };
    { let a0_ = c.m.r32((l_w_3).wrapping_add(40)); F::EftEmit_InitState(c, a0_, (l_w_3).wrapping_add(44)) };
    { let v_ = F32(0x41200000); let a_ = (l_w_3).wrapping_add(1384); c.m.w32(a_, (v_).0); };
    if (({ let a0_ = (c.m.r32((p_req_2).wrapping_add(4)) as i32); F::EftTransform_HasStartGroup(c, a0_) }) != 0) {
        { let r_ = 1i32; let a_ = (l_w_3).wrapping_add(1368); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        F::EftEmit_SetNode(c, (l_w_3).wrapping_add(768), 0i32, 3i32, ((0i32) as u32));
    }
    { let a0_ = (c.m.r32(l_arg_4) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, l_w_3) };
    { let r_ = 8i32; let a_ = (l_w_3).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    { let v_ = rt::i2f({ let a0_ = c.m.r32((l_w_3).wrapping_add(40)); F::EftEmit_GetEndFrames(c, a0_) }); let a_ = (l_w_3).wrapping_add(1376); c.m.w32(a_, (v_).0); };
    { let v_ = 1i32; let a_ = (c.m.r32(sym::gEftTransform)).wrapping_add(6404); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTransform_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftTransform_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTransform_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_2: u32 = 0;
    let mut l_arg_3: u32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_arg_3 = (l_w_2).wrapping_add(32);
        if (((((c.m.r32((l_w_2).wrapping_add(1360)) as i32)) & (256i32))) != 0) {
            F::StgTint_Start(c, ((1i32) as u32), 1i32, F32(0x00000000));
            { let a0_ = (c.m.r32(l_arg_3) as i32); F::BtlCharApi_ObjClearMaskBit3(c, a0_) };
            { let a0_ = (c.m.r32(l_arg_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, fp) };
            { let _ = { let a0_ = (c.m.r32(l_arg_3) as i32); F::ScrWarp_Spawn(c, a0_, fp, F32(0x3f199999), F32(0x41200000), F32(0x42480000), F32(0x41200000), F32(0x3ecccccc)) }; };
            { let _ = { let a0_ = (c.m.r32(l_arg_3) as i32); F::ScrWarp_Spawn(c, a0_, fp, F32(0x3f199999), F32(0x41200000), F32(0x42480000), F32(0x41200000), F32(0x3ecccccc)) }; };
        }
        F::EftStage_SetDrawOn(c, 1i32);
        { let a0_ = c.m.r32((l_w_2).wrapping_add(40)); F::EftEmit_TermState(c, a0_, (l_w_2).wrapping_add(44)) };
        if ((((!(((((c.m.r32((l_w_2).wrapping_add(1360)) as i32)) & (2i32))) != 0)) as i32)) != 0) {
            { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gEftTransform)).wrapping_add(6400); c.m.w32(a_, v_); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gEftTransform)).wrapping_add(6404); c.m.w32(a_, (v_) as u32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftTransform_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftTransform_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTransform_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_2: u32 = 0;
    let mut l_arg_3: u32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_arg_3 = (l_w_2).wrapping_add(32);
        if ((((!(({ let a0_ = (c.m.r32(l_arg_3) as i32); F::BtlScene_IsEffectStopped(c, a0_, 5i32) }) != 0)) as i32)) != 0) {
            { let a1_ = (c.m.r32((l_arg_3).wrapping_add(4)) as i32); F::EftTransform_UpdateNodes(c, p_task_1, a1_) };
            's1: {
                let sv4: i32 = ((c.m.r8((p_task_1).wrapping_add(1))) as i32);
                match sv4 {
                    0i32 => {
                        { let v_ = ((1i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                        break 's1;
                    }
                    1i32 => {
                        if (((((c.m.r32((l_w_2).wrapping_add(1360)) as i32)) & (16i32))) != 0) {
                            { let a1_ = (c.m.r32((l_arg_3).wrapping_add(4)) as i32); F::EftTransform_OnFlash(c, p_task_1, a1_) };
                            F::EftStage_SetDrawOn(c, 0i32);
                            F::StgTint_Start(c, ((1i32) as u32), 0i32, F32(0x00000000));
                            { let r_ = 256i32; let a_ = (l_w_2).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            { let a0_ = (c.m.r32(l_arg_3) as i32); F::BtlCharApi_ObjSetMaskBit3(c, a0_) };
                            { let a_ = (p_task_1).wrapping_add(1); let o_ = c.m.r8(a_); let n_ = (o_).wrapping_add(1); c.m.w8(a_, n_); };
                        }
                        break 's1;
                    }
                    2i32 => {
                        { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(1380); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        if (((rt::flt(F32(c.m.r32((l_w_2).wrapping_add(1384))), F32(c.m.r32((l_w_2).wrapping_add(1380)))) as i32)) != 0) {
                            if (((((c.m.r32((l_w_2).wrapping_add(1360)) as i32)) & (256i32))) != 0) {
                                F::StgTint_Start(c, ((1i32) as u32), 1i32, F32(0x00000000));
                                { let a0_ = (c.m.r32(l_arg_3) as i32); F::BtlCharApi_ObjClearMaskBit3(c, a0_) };
                                { let a0_ = (c.m.r32(l_arg_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, fp) };
                                { let _ = { let a0_ = (c.m.r32(l_arg_3) as i32); F::ScrWarp_Spawn(c, a0_, fp, F32(0x3f199999), F32(0x41200000), F32(0x42480000), F32(0x41200000), F32(0x3ecccccc)) }; };
                                { let _ = { let a0_ = (c.m.r32(l_arg_3) as i32); F::ScrWarp_Spawn(c, a0_, fp, F32(0x3f199999), F32(0x41200000), F32(0x42480000), F32(0x41200000), F32(0x3ecccccc)) }; };
                                { let r_ = (!(256i32)); let a_ = (l_w_2).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                            }
                            { let r_ = 32i32; let a_ = (l_w_2).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            { let a_ = (p_task_1).wrapping_add(1); let o_ = c.m.r8(a_); let n_ = (o_).wrapping_add(1); c.m.w8(a_, n_); };
                        }
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
            if (((((c.m.r32((l_w_2).wrapping_add(1360)) as i32)) & (32i32))) != 0) {
                if ((((!(((((c.m.r32((l_w_2).wrapping_add(1360)) as i32)) & (64i32))) != 0)) as i32)) != 0) {
                    { let r_ = 2i32; let a_ = (l_w_2).wrapping_add(1368); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    F::EftEmit_SetNode(c, (l_w_2).wrapping_add(768), 1i32, 3i32, ((0i32) as u32));
                    { let r_ = 64i32; let a_ = (l_w_2).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
            { let a0_ = (c.m.r32(l_arg_3) as i32); F::EftEmit_RefreshFixedNodes(c, a0_, (l_w_2).wrapping_add(768)) };
            { let a0_ = (c.m.r32(l_arg_3) as i32); let a2_ = c.m.r32((l_w_2).wrapping_add(40)); F::EftTransform_SpawnParts(c, a0_, p_task_1, a2_) };
            if (((((c.m.r32((l_w_2).wrapping_add(1360)) as i32)) & (1i32))) != 0) {
                { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(1372); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(1376))), F32(c.m.r32((l_w_2).wrapping_add(1372)))) as i32)) != 0) {
                    { let r_ = 4i32; let a_ = (l_w_2).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
        }
        if ((((((c.m.r32((l_arg_3).wrapping_add(4)) as i32)) == (5i32)) as i32)) != 0) {
            F::EftStage_UpdateSprite(c);
        }
        if (((((c.m.r32((l_w_2).wrapping_add(1360)) as i32)) & (4i32))) != 0) {
            F::BtlTask_SetDead(c, p_task_1);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftTransform_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftTransform_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTransform_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(1360)) as i32)) & (1024i32))) != 0)) as i32)) != 0) {
        { let r_ = 1024i32; let a_ = (l_w_2).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let a0_ = c.m.r32((l_w_2).wrapping_add(40)); F::EftEmit_KillAll(c, a0_, (l_w_2).wrapping_add(44)) };
    }
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTransform_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftTransform_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTransform_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(({ let a0_ = c.m.r32((l_w_2).wrapping_add(40)); F::EftEmit_UpdateAlive(c, a0_, (l_w_2).wrapping_add(44)) }) != 0)) as i32)) != 0) {
        { let r_ = (!(8i32)); let a_ = (l_w_2).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        { let r_ = 8i32; let a_ = (l_w_2).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    { let v_ = 0i32; let a_ = (l_w_2).wrapping_add(1368); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTransform_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftTransform_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTransform_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((((((((c.m.r32(((l_w_2).wrapping_add(32)).wrapping_add(4)) as i32)) == (5i32)) as i32)) != 0) && (((((c.m.r32((l_w_2).wrapping_add(1360)) as i32)) & (512i32))) != 0)) as i32)) != 0) {
        F::EftStage_DrawSprite(c, l_w_2);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTransform_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftTransformMgr_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTransformMgr_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((6408u32) as i32)) }; let a_ = sym::gEftTransform; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gEftTransform); F::memset(c, a0_, 0i32, 6408u32) }; };
        { let v_ = F::BtlTask_CreateChildList(c, p_task_1, ((((2i32) as i64 as u64)) as u32), ((((((1392u32) as i32)) as i64 as u64)) as u32)); let a_ = sym::gEftTransformList; c.m.w32(a_, v_); };
        {
            rt::memset(c, fp, 0, 32);
            { let v_ = 402i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
            { let v_ = 426i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
            { let v_ = 448i32; c.m.w32(fp.wrapping_add(8), (v_) as u32); }
            { let v_ = 449i32; c.m.w32(fp.wrapping_add(12), (v_) as u32); }
            { let v_ = 459i32; c.m.w32(fp.wrapping_add(16), (v_) as u32); }
            { let v_ = 486i32; c.m.w32(fp.wrapping_add(20), (v_) as u32); }
            { let v_ = 508i32; c.m.w32(fp.wrapping_add(24), (v_) as u32); }
            { let v_ = 535i32; c.m.w32(fp.wrapping_add(28), (v_) as u32); }
            {
                l_i_2 = 0i32;
                'l1: loop {
                    if !(((((l_i_2) < (8i32)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        { let a1_ = (c.m.r32(sym::gEftTransform)).wrapping_add((((l_i_2) as u32)).wrapping_mul(800)); let a5_ = (c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32); F::EftEmit_LoadSet(c, ((0i32) as u32), a1_, ((((0i32) as i64 as u64)) as u32), ((0i32) as u32), 1i32, a5_) };
                    }
                    { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftTransformMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftTransformMgr_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTransformMgr_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (8i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = (c.m.r32(sym::gEftTransform)).wrapping_add((((l_i_1) as u32)).wrapping_mul(800)); F::EftEmit_FreeSet(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftTransform); F::BtlPool_Free(c, a0_, a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftTransform; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTransformMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftTransformMgr_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTransformMgr_Update", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (8i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = (c.m.r32(sym::gEftTransform)).wrapping_add((((l_i_1) as u32)).wrapping_mul(800)); F::EftEmit_BeginFrame(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTransformMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftTransform_Start(c: &mut Ctx, mut p_req_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTransform_Start", &PROBE_N, &[(p_req_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_task_2: u32 = 0;
    let r_ = 'f: {
        { let s_ = p_req_1; rt::memcpy(c, fp, s_, 8); }
        if ((((({ let a_ = (c.m.r32(sym::gEftTransform)).wrapping_add(6404); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        l_task_2 = { let a0_ = c.m.r32(sym::gEftTransformList); F::BtlTaskList_AddTail(c, a0_, sym::gEftTransformClass, fp) };
        if (((((l_task_2) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        { let v_ = l_task_2; let a_ = (c.m.r32(sym::gEftTransform)).wrapping_add(6400); c.m.w32(a_, v_); };
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftTransform_Start", &PROBE_N, &[], true); }
    r_
}

pub fn EftTransform_Flash(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTransform_Flash", &PROBE_N, &[], false); }
    let mut l_task_1: u32 = 0;
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_task_1 = { let a_ = (c.m.r32(sym::gEftTransform)).wrapping_add(6400); c.m.r32(a_) };
    if (((((l_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((l_task_1).wrapping_add(56));
    if ((((({ let a_ = (c.m.r32(sym::gEftTransform)).wrapping_add(6404); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let r_ = 16i32; let a_ = (l_w_2).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTransform_Flash", &PROBE_N, &[], true); }
    r_
}

pub fn EftTransform_End(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTransform_End", &PROBE_N, &[], false); }
    let mut l_task_1: u32 = 0;
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_task_1 = { let a_ = (c.m.r32(sym::gEftTransform)).wrapping_add(6400); c.m.r32(a_) };
    if (((((l_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((l_task_1).wrapping_add(56));
    if ((((({ let a_ = (c.m.r32(sym::gEftTransform)).wrapping_add(6404); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let r_ = ((1i32) | (2i32)); let a_ = (l_w_2).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gEftTransform)).wrapping_add(6400); c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gEftTransform)).wrapping_add(6404); c.m.w32(a_, (v_) as u32); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTransform_End", &PROBE_N, &[], true); }
    r_
}
