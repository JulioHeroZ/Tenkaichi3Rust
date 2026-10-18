#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EftAbsorb_DrawOne(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32, mut p_model_3: u32, mut p_inst_4: u32, mut p_pos_5: u32, mut p_dir_6: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAbsorb_DrawOne", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32, (p_model_3) as u32, (p_inst_4) as u32, (p_pos_5) as u32, (p_dir_6) as u32], false); }
    let mut l_w_7: u32 = 0;
    let mut l_part_8: i32 = 0;
    let mut l_sub_9: i32 = 0;
    let mut l_def_10: u32 = 0;
    let mut l_spawn_11: i32 = 0;
    let r_ = 'f: {
    l_w_7 = c.m.r32((p_task_2).wrapping_add(56));
    {
        l_part_8 = 0i32;
        'l1: loop {
            if !(((((l_part_8) < (19i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((({ let a_ = c.m.r32(p_model_3); c.m.r32(a_) }) & ((((1i32).wrapping_shl((l_part_8) as u32)) as u32)))) != 0) {
                    l_def_10 = c.m.r32(((p_model_3).wrapping_add(12)).wrapping_add((((l_part_8) as u32)).wrapping_mul(16)));
                    {
                        l_sub_9 = 0i32;
                        'l3: loop {
                            if !(((((l_sub_9) < (((c.m.r8((l_def_10).wrapping_add(1))) as i32))) as i32)) != 0) { break 'l3; }
                            'c4: {
                                l_spawn_11 = { let a5_ = ((((c.m.r32((l_w_7).wrapping_add(2116))) & (((2i32) as u32)))) as i32); let a7_ = ((c.m.r32((l_w_7).wrapping_add(2120))) as i32); F::EftEmit_GetFlagsFromMask(c, p_model_3, p_inst_4, p_objId_1, l_part_8, l_sub_9, a5_, 0i32, a7_) };
                                if (((((l_spawn_11) != (0i32)) as i32)) != 0) {
                                    { let a11_ = F32(c.m.r32((l_w_7).wrapping_add(2124))); F::EftEmit_Spawn(c, p_model_3, p_inst_4, (l_w_7).wrapping_add(1520), p_pos_5, p_dir_6, p_objId_1, 0i32, 3i32, l_part_8, l_sub_9, l_spawn_11, a11_) };
                                }
                            }
                            { let o_ = l_sub_9; let n_ = (o_).wrapping_add(1); l_sub_9 = n_; };
                        }
                    }
                }
            }
            { let o_ = l_part_8; let n_ = (o_).wrapping_add(1); l_part_8 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAbsorb_DrawOne", &PROBE_N, &[], true); }
    r_
}

pub fn EftAbsorb_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAbsorb_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_mgr_3: u32 = 0;
    let mut l_w_4: u32 = 0;
    let mut l_owner_5: u32 = 0;
    let mut l_model_6: u32 = 0;
    let r_ = 'f: {
        l_mgr_3 = { let a_ = (F::BtlTask_GetParent(c, p_task_1)).wrapping_add(56); c.m.r32(a_) };
        l_w_4 = c.m.r32((p_task_1).wrapping_add(56));
        l_owner_5 = (l_w_4).wrapping_add(64);
        l_model_6 = c.m.r32(l_mgr_3);
        { let _ = F::memset(c, l_w_4, 0i32, 2144u32); };
        { let s_ = p_arg_2; let d_ = l_owner_5; rt::memcpy(c, d_, s_, 8); };
        { let v_ = (c.m.r32((p_arg_2).wrapping_add(8)) as i32); let a_ = (l_w_4).wrapping_add(2112); c.m.w32(a_, (v_) as u32); };
        { let v_ = F32(c.m.r32(((l_w_4).wrapping_add(64)).wrapping_add(4))); let a_ = (l_w_4).wrapping_add(2124); c.m.w32(a_, (v_).0); };
        { let r_ = ((1i32) as u32); let a_ = (l_w_4).wrapping_add(2116); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        { let v_ = l_model_6; let a_ = (l_w_4).wrapping_add(72); c.m.w32(a_, v_); };
        F::EftEmit_InitState(c, l_model_6, ((l_w_4).wrapping_add(76)).wrapping_add((((0i32) as u32)).wrapping_mul(716)));
        { let a0_ = c.m.r32((l_w_4).wrapping_add(72)); F::EftEmit_InitState(c, a0_, ((l_w_4).wrapping_add(76)).wrapping_add((((1i32) as u32)).wrapping_mul(716))) };
        { let r_ = ((8i32) as u32); let a_ = (l_w_4).wrapping_add(2116); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        { let v_ = rt::i2f({ let a0_ = c.m.r32((l_w_4).wrapping_add(72)); F::EftEmit_GetEndFrames(c, a0_) }); let a_ = (l_w_4).wrapping_add(2132); c.m.w32(a_, (v_).0); };
        { let a1_ = (if ((((((c.m.r32(l_owner_5) as i32)) == (0i32)) as i32)) != 0) { 2048i32 } else { 4096i32 }); F::BtlTask_SetOwnerTag(c, p_task_1, a1_) };
        if ((((((c.m.r32((l_w_4).wrapping_add(2112)) as i32)) == (0i32)) as i32)) != 0) {
            { let a0_ = (c.m.r32(l_owner_5) as i32); F::BtlCharApi_GetNodePos(c, a0_, 54i32, (l_w_4).wrapping_add((((0i32) as u32)).wrapping_mul(16))) };
            F::Vec4_Set(c, ((l_w_4).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), F32(0x00000000), F32(0xbf800000), F32(0x00000000), F32(0x3f800000));
            { let r_ = ((1i32) as u32); let a_ = (l_w_4).wrapping_add(2120); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
            F::EftEmit_SetNode(c, (l_w_4).wrapping_add(1520), 0i32, 54i32, ((0i32) as u32));
        } else {
            { let a0_ = (c.m.r32(l_owner_5) as i32); F::BtlCharApi_GetNodePos(c, a0_, 21i32, (l_w_4).wrapping_add((((0i32) as u32)).wrapping_mul(16))) };
            { let a0_ = (c.m.r32(l_owner_5) as i32); F::BtlCharApi_GetNodePos(c, a0_, 35i32, (l_w_4).wrapping_add((((1i32) as u32)).wrapping_mul(16))) };
            { let r_ = ((2i32) as u32); let a_ = (l_w_4).wrapping_add(2120); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
            F::EftEmit_SetNode(c, (l_w_4).wrapping_add(1520), 1i32, 54i32, ((0i32) as u32));
            { let a0_ = (c.m.r32(l_owner_5) as i32); F::BtlCharApi_GetNodePos(c, a0_, 20i32, fp) };
            { let a0_ = (c.m.r32(l_owner_5) as i32); F::BtlCharApi_GetNodePos(c, a0_, 34i32, fp.wrapping_add(16)) };
            F::Vec3_Sub(c, ((l_w_4).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (l_w_4).wrapping_add((((0i32) as u32)).wrapping_mul(16)), fp);
            F::Vec3_Sub(c, ((l_w_4).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (l_w_4).wrapping_add((((0i32) as u32)).wrapping_mul(16)), fp.wrapping_add(16));
            F::Vec3_Normalize(c, ((l_w_4).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), ((l_w_4).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
            F::Vec3_Normalize(c, ((l_w_4).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), ((l_w_4).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
            { let v_ = F32(0x3f800000); let a_ = (((l_w_4).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f800000); let a_ = (((l_w_4).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftAbsorb_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftAbsorb_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAbsorb_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_owner_3: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_owner_3 = (l_w_2).wrapping_add(64);
    { let a0_ = c.m.r32((l_w_2).wrapping_add(72)); F::EftEmit_TermState(c, a0_, ((l_w_2).wrapping_add(76)).wrapping_add((((0i32) as u32)).wrapping_mul(716))) };
    { let a0_ = c.m.r32((l_w_2).wrapping_add(72)); F::EftEmit_TermState(c, a0_, ((l_w_2).wrapping_add(76)).wrapping_add((((1i32) as u32)).wrapping_mul(716))) };
    { let v_ = ((0i32) as u32); let a_ = (l_w_2).wrapping_add(2116); c.m.w32(a_, v_); };
    if ((((((c.m.r32((l_w_2).wrapping_add(2112)) as i32)) == (0i32)) as i32)) != 0) {
        { let a0_ = (c.m.r32(l_owner_3) as i32); F::EftCharSlot_ClearAbsorb(c, a0_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAbsorb_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftAbsorb_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAbsorb_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_w_2: u32 = 0;
    let mut l_owner_3: u32 = 0;
    let mut l_nodes_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_owner_3 = (l_w_2).wrapping_add(64);
        if ((((!(({ let a0_ = (c.m.r32(l_owner_3) as i32); F::BtlScene_IsEffectStopped(c, a0_, 3i32) }) != 0)) as i32)) != 0) {
            if ((((((c.m.r32((l_w_2).wrapping_add(2112)) as i32)) == (0i32)) as i32)) != 0) {
                { let a0_ = (c.m.r32(l_owner_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 54i32, (l_w_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))) };
                if (({ let a0_ = (c.m.r32(l_owner_3) as i32); F::BtlCharApi_ObjTestAttr(c, a0_, ((1024i32) as u64)) }) != 0) {
                    { let r_ = ((1i32) as u32); let a_ = (l_w_2).wrapping_add(2120); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                    l_nodes_4 = (l_w_2).wrapping_add(1520);
                    F::EftEmit_SetNode(c, l_nodes_4, 0i32, 54i32, ((0i32) as u32));
                } else {
                    l_nodes_4 = (l_w_2).wrapping_add(1520);
                    { let _ = { let a0_ = (c.m.r32(l_owner_3) as i32); F::BtlCharApi_ObjTestAttr(c, a0_, ((4096i32) as u64)) }; };
                }
            } else {
                { let a0_ = (c.m.r32(l_owner_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 21i32, (l_w_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))) };
                { let a0_ = (c.m.r32(l_owner_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 35i32, (l_w_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))) };
                { let r_ = ((2i32) as u32); let a_ = (l_w_2).wrapping_add(2120); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                l_nodes_4 = (l_w_2).wrapping_add(1520);
                F::EftEmit_SetNode(c, l_nodes_4, 1i32, 54i32, ((0i32) as u32));
                { let a0_ = (c.m.r32(l_owner_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 20i32, fp) };
                { let a0_ = (c.m.r32(l_owner_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 34i32, fp.wrapping_add(16)) };
                F::Vec3_Sub(c, ((l_w_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (l_w_2).wrapping_add((((0i32) as u32)).wrapping_mul(16)), fp);
                F::Vec3_Sub(c, ((l_w_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (l_w_2).wrapping_add((((1i32) as u32)).wrapping_mul(16)), fp.wrapping_add(16));
                F::Vec3_Normalize(c, ((l_w_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), ((l_w_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
                F::Vec3_Normalize(c, ((l_w_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), ((l_w_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
                { let v_ = F32(0x3f800000); let a_ = (((l_w_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = (((l_w_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
            }
            { let a0_ = (c.m.r32(l_owner_3) as i32); F::EftEmit_RefreshFixedNodes(c, a0_, l_nodes_4) };
            if ((((((c.m.r32((l_w_2).wrapping_add(2112)) as i32)) == (0i32)) as i32)) != 0) {
                { let a0_ = (c.m.r32(l_owner_3) as i32); let a2_ = c.m.r32((l_w_2).wrapping_add(72)); F::EftAbsorb_DrawOne(c, a0_, p_task_1, a2_, ((l_w_2).wrapping_add(76)).wrapping_add((((0i32) as u32)).wrapping_mul(716)), (l_w_2).wrapping_add((((0i32) as u32)).wrapping_mul(16)), ((l_w_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) };
            } else {
                {
                    l_i_5 = 0i32;
                    'l1: loop {
                        if !(((((l_i_5) < (2i32)) as i32)) != 0) { break 'l1; }
                        'c2: {
                            { let a0_ = (c.m.r32(l_owner_3) as i32); let a2_ = c.m.r32((l_w_2).wrapping_add(72)); F::EftAbsorb_DrawOne(c, a0_, p_task_1, a2_, ((l_w_2).wrapping_add(76)).wrapping_add((((l_i_5) as u32)).wrapping_mul(716)), (l_w_2).wrapping_add((((l_i_5) as u32)).wrapping_mul(16)), ((l_w_2).wrapping_add(32)).wrapping_add((((l_i_5) as u32)).wrapping_mul(16))) };
                        }
                        { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
                    }
                }
            }
            if ((((c.m.r32((l_w_2).wrapping_add(2116))) & (((2i32) as u32)))) != 0) {
                { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(2128); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::fge(F32(c.m.r32((l_w_2).wrapping_add(2128))), F32(c.m.r32((l_w_2).wrapping_add(2132)))) as i32)) != 0) {
                    { let r_ = ((4i32) as u32); let a_ = (l_w_2).wrapping_add(2116); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                }
            }
        }
        if ((((c.m.r32((l_w_2).wrapping_add(2116))) & (((4i32) as u32)))) != 0) {
            F::BtlTask_SetDead(c, p_task_1);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftAbsorb_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftAbsorb_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAbsorb_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!((((c.m.r32((l_w_2).wrapping_add(2116))) & (((32i32) as u32)))) != 0)) as i32)) != 0) {
        { let r_ = ((32i32) as u32); let a_ = (l_w_2).wrapping_add(2116); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        { let a0_ = c.m.r32((l_w_2).wrapping_add(72)); F::EftEmit_KillAll(c, a0_, ((l_w_2).wrapping_add(76)).wrapping_add((((0i32) as u32)).wrapping_mul(716))) };
        { let a0_ = c.m.r32((l_w_2).wrapping_add(72)); F::EftEmit_KillAll(c, a0_, ((l_w_2).wrapping_add(76)).wrapping_add((((1i32) as u32)).wrapping_mul(716))) };
    }
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAbsorb_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftAbsorb_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAbsorb_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((c.m.r32((l_w_2).wrapping_add(2116))) & (((16i32) as u32)))) != 0) {
        if ((((!(({ let a0_ = c.m.r32((l_w_2).wrapping_add(72)); F::EftEmit_UpdateAlive(c, a0_, ((l_w_2).wrapping_add(76)).wrapping_add((((0i32) as u32)).wrapping_mul(716))) }) != 0)) as i32)) != 0) {
            if ((((((c.m.r32((l_w_2).wrapping_add(2112)) as i32)) == (0i32)) as i32)) != 0) {
                { let r_ = (((!(8i32))) as u32); let a_ = (l_w_2).wrapping_add(2116); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
            } else {
                { let r_ = ((2i32) as u32); let a_ = (l_w_2).wrapping_add(2116); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
            }
        } else {
            { let r_ = ((8i32) as u32); let a_ = (l_w_2).wrapping_add(2116); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        }
        { let v_ = ((0i32) as u32); let a_ = (l_w_2).wrapping_add(2120); c.m.w32(a_, v_); };
    }
    { let r_ = ((16i32) as u32); let a_ = (l_w_2).wrapping_add(2116); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAbsorb_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftAbsorb_Nop(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAbsorb_Nop", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAbsorb_Nop", &PROBE_N, &[], true); }
    r_
}

pub fn EftAbsorbMgr_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAbsorbMgr_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_mgr_3: u32 = 0;
    let mut l_model_4: u32 = 0;
    let mut l_pack_5: u32 = 0;
    let mut l_list_6: u32 = 0;
    let r_ = 'f: {
    l_mgr_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((804u32) as i32)) }; let a_ = l_mgr_3; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(l_mgr_3); F::memset(c, a0_, 0i32, 804u32) }; };
    l_model_4 = c.m.r32(l_mgr_3);
    l_pack_5 = { let a0_ = (c.m.r32((p_arg_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::BtlScene_GetCharPackEntry(c, a0_, 11i32) };
    { let v_ = l_pack_5; let a_ = (l_model_4).wrapping_add(800); c.m.w32(a_, v_); };
    if (((((l_pack_5) != (((0i32) as u32))) as i32)) != 0) {
        F::EftEmit_LoadSet(c, ((0i32) as u32), l_model_4, ((((0i32) as i64 as u64)) as u32), l_pack_5, 2i32, 1i32);
    }
    l_list_6 = F::BtlTask_CreateChildList(c, p_task_1, ((((6i32) as i64 as u64)) as u32), ((((((2144u32) as i32)) as i64 as u64)) as u32));
    { let a0_ = (c.m.r32((p_arg_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a1_ = (c.m.r32((p_arg_2).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); F::EftChar_SetList(c, a0_, a1_, l_list_6) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAbsorbMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftAbsorbMgr_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAbsorbMgr_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_mgr_2: u32 = 0;
    let r_ = 'f: {
    l_mgr_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((({ let a_ = (c.m.r32(l_mgr_2)).wrapping_add(800); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(l_mgr_2); F::EftEmit_FreeSet(c, a0_) };
    }
    if (((((c.m.r32(l_mgr_2)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(l_mgr_2); F::BtlPool_Free(c, a0_, a1_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAbsorbMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftAbsorbMgr_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAbsorbMgr_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_mgr_2: u32 = 0;
    let r_ = 'f: {
    l_mgr_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((({ let a_ = (c.m.r32(l_mgr_2)).wrapping_add(800); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(l_mgr_2); F::EftEmit_BeginFrame(c, a0_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAbsorbMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftAbsorb_Start(c: &mut Ctx, mut p_arg_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAbsorb_Start", &PROBE_N, &[(p_arg_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_list_2: u32 = 0;
    let mut l_task_3: u32 = 0;
    let r_ = 'f: {
        { let s_ = p_arg_1; let d_ = fp; rt::memcpy(c, d_, s_, 8); };
        { let v_ = 0i32; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a0_ = (c.m.r32(fp) as i32); F::EftCharSlot_GetAbsorb(c, a0_) }) != (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        l_list_2 = { let a0_ = (c.m.r32(fp) as i32); F::EftChar_GetList(c, a0_, 9i32) };
        if (((((l_list_2) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        l_task_3 = F::BtlTaskList_AddTail(c, l_list_2, sym::gEftAbsorbClass, fp);
        if (((((l_task_3) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        { let a0_ = (c.m.r32(fp) as i32); F::EftCharSlot_SetAbsorb(c, a0_, l_task_3) };
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftAbsorb_Start", &PROBE_N, &[], true); }
    r_
}

pub fn EftAbsorb_Stop(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAbsorb_Stop", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_task_2: u32 = 0;
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    l_task_2 = F::EftCharSlot_GetAbsorb(c, p_objId_1);
    if (((((l_task_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((l_task_2).wrapping_add(56));
    if ((((!((((c.m.r32((l_w_3).wrapping_add(2116))) & (((1i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let r_ = ((2i32) as u32); let a_ = (l_w_3).wrapping_add(2116); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAbsorb_Stop", &PROBE_N, &[], true); }
    r_
}

pub fn EftAbsorb_StartHands(c: &mut Ctx, mut p_arg_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAbsorb_StartHands", &PROBE_N, &[(p_arg_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_list_2: u32 = 0;
    let r_ = 'f: {
        { let s_ = p_arg_1; let d_ = fp; rt::memcpy(c, d_, s_, 8); };
        { let v_ = 1i32; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        l_list_2 = { let a0_ = (c.m.r32(fp) as i32); F::EftChar_GetList(c, a0_, 9i32) };
        if (((((l_list_2) != (((0i32) as u32))) as i32)) != 0) {
            { let _ = F::BtlTaskList_AddTail(c, l_list_2, sym::gEftAbsorbClass, fp); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftAbsorb_StartHands", &PROBE_N, &[], true); }
    r_
}
