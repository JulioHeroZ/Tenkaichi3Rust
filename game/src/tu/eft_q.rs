#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn eft_q__EftGlow_ColorFromBytes(c: &mut Ctx, mut p_dst_1: u32, mut p_src_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_q__EftGlow_ColorFromBytes", &PROBE_N, &[(p_dst_1) as u32, (p_src_2) as u32], false); }
    let r_ = 'f: {
    F::Vec4_Scale(c, p_dst_1, p_src_2, F32(0x3b808080));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "eft_q__EftGlow_ColorFromBytes", &PROBE_N, &[], true); }
    r_
}

pub fn eft_q__EftGlow_SetBaseColors(c: &mut Ctx, mut p_w_1: u32, mut p_textured_2: u8, mut p_row_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_q__EftGlow_SetBaseColors", &PROBE_N, &[(p_w_1) as u32, (p_textured_2) as u32, (p_row_3) as u32], false); }
    let r_ = 'f: {
    if ((p_textured_2) != 0) {
        { let a1_ = ((c.m.r32(sym::gEftGlowCfg)).wrapping_add(2288)).wrapping_add(((((p_row_3).wrapping_mul(5i32)) as u32)).wrapping_mul(16)); F::Vec4_Scale(c, (p_w_1).wrapping_add(400), a1_, F32(0x3b808080)) };
        { let a1_ = ((c.m.r32(sym::gEftGlowCfg)).wrapping_add(2288)).wrapping_add(((((p_row_3).wrapping_mul(5i32)) as u32)).wrapping_mul(16)); F::Vec4_Scale(c, (p_w_1).wrapping_add(416), a1_, F32(0x3b808080)) };
        { let v_ = F32(0x00000000); let a_ = ((p_w_1).wrapping_add(416)).wrapping_add(12); c.m.w32(a_, (v_).0); };
    } else {
        { let a1_ = ((c.m.r32(sym::gEftGlowCfg)).wrapping_add(1568)).wrapping_add((((((c.m.r32((p_w_1).wrapping_add(28)) as i32)).wrapping_mul(5i32)) as u32)).wrapping_mul(16)); F::Vec4_Scale(c, (p_w_1).wrapping_add(400), a1_, F32(0x3b808080)) };
        { let a1_ = ((c.m.r32(sym::gEftGlowCfg)).wrapping_add(1568)).wrapping_add((((((c.m.r32((p_w_1).wrapping_add(28)) as i32)).wrapping_mul(5i32)) as u32)).wrapping_mul(16)); F::Vec4_Scale(c, (p_w_1).wrapping_add(416), a1_, F32(0x3b808080)) };
        { let v_ = F32(0x00000000); let a_ = ((p_w_1).wrapping_add(416)).wrapping_add(12); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "eft_q__EftGlow_SetBaseColors", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlow_UpdateTextures(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_UpdateTextures", &PROBE_N, &[(p_w_1) as u32], false); }
    let mut l_mgr_2: u32 = 0;
    let mut l_set_3: u32 = 0;
    let mut l_res_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_tex0_6: u64 = 0;
    let mut l_v_7: u32 = 0;
    let mut l_set2_8: u32 = 0;
    let r_ = 'f: {
    l_mgr_2 = c.m.r32(sym::gEftGlow);
    l_set_3 = (((l_mgr_2).wrapping_add(672)).wrapping_add((((0i32) as u32)).wrapping_mul(552))).wrapping_add(32);
    if (((((c.m.r32(p_w_1) as i32)) & (32i32))) != 0) {
        l_tex0_6 = F::EftVram_AddImage(c, ((((l_mgr_2).wrapping_add(672)).wrapping_add((((0i32) as u32)).wrapping_mul(552))).wrapping_add(32)).wrapping_add((((8i32) as u32)).wrapping_mul(16)), 1i32, 0i32);
        {
            l_i_5 = (c.m.r32((p_w_1).wrapping_add(40)) as i32);
            'l1: loop {
                if !(((((l_i_5) < (((c.m.r32((p_w_1).wrapping_add(40)) as i32)).wrapping_add((c.m.r32((p_w_1).wrapping_add(44)) as i32)))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_v_7 = (p_w_1).wrapping_add(8);
                    { let v_ = ((l_tex0_6) | ((F::EftVram_AddClut(c, (l_set_3).wrapping_add((((l_i_5) as u32)).wrapping_mul(16)))).wrapping_shl((37i32) as u32))); let a_ = ((l_v_7).wrapping_add(672)).wrapping_add(((((l_i_5).wrapping_sub((c.m.r32((p_w_1).wrapping_add(40)) as i32))) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
    } else {
        { let v_ = F::EftVram_AddImage(c, (l_set_3).wrapping_add((((0i32) as u32)).wrapping_mul(16)), 1i32, 0i32); let a_ = ((p_w_1).wrapping_add(672)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = ({ let a0_ = (l_set_3).wrapping_add(((((c.m.r32((p_w_1).wrapping_add(28)) as i32)) as u32)).wrapping_mul(16)); F::EftVram_AddClut(c, a0_) }).wrapping_shl((37i32) as u32); let a_ = ((p_w_1).wrapping_add(672)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    l_mgr_2 = c.m.r32(sym::gEftGlow);
    l_res_4 = ((l_mgr_2).wrapping_add(672)).wrapping_add((((1i32) as u32)).wrapping_mul(552));
    if ((((((c.m.r32(l_res_4) as i32)) == (0i32)) as i32)) != 0) {
        l_set2_8 = (((l_mgr_2).wrapping_add(672)).wrapping_add((((1i32) as u32)).wrapping_mul(552))).wrapping_add(32);
        {
            l_i_5 = 0i32;
            'l3: loop {
                if !(((((l_i_5) < ((c.m.r32((l_set2_8).wrapping_add(512)) as i32))) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let v_ = F::EftVram_AddTex(c, (l_set2_8).wrapping_add((((l_i_5) as u32)).wrapping_mul(16)), 1i32, 0i32); let a_ = (l_set2_8).wrapping_add((((l_i_5) as u32)).wrapping_mul(16)); c.m.w64(a_, v_); };
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
        { let v_ = 1i32; let a_ = l_res_4; c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_UpdateTextures", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlow_SetType(c: &mut Ctx, mut p_w_1: u32, mut p_objId_2: i32, mut p_type_3: i32, mut p_paramFlags_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_SetType", &PROBE_N, &[(p_w_1) as u32, (p_objId_2) as u32, (p_type_3) as u32, (p_paramFlags_4) as u32], false); }
    let mut l_row_5: i32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_textured_7: i32 = 0;
    let mut l_j_8: i32 = 0;
    let r_ = 'f: {
    l_row_5 = 0i32;
    { let v_ = p_paramFlags_4; let a_ = (p_w_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let r_ = (!(32i32)); let a_ = p_w_1; let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    if ((((((((p_type_3).wrapping_sub(8i32)) as u32)) < (((3i32) as u32))) as i32)) != 0) {
        { let r_ = 32i32; let a_ = p_w_1; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        p_type_3 = 8i32;
        { let v_ = 1i32; let a_ = (p_w_1).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        { let v_ = p_type_3; let a_ = (p_w_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
    }
    's1: {
        let sv9: i32 = p_type_3;
        match sv9 {
            8i32 => {
                { let v_ = p_type_3; let a_ = (p_w_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 2i32; let a_ = (p_w_1).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            9i32 => {
                { let v_ = 10i32; let a_ = (p_w_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 2i32; let a_ = (p_w_1).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                l_row_5 = 2i32;
                break 's1;
            }
            10i32 => {
                { let v_ = 12i32; let a_ = (p_w_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 2i32; let a_ = (p_w_1).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                l_row_5 = 3i32;
                break 's1;
            }
            _ => {
            }
        }
    }
    { let v_ = p_type_3; let a_ = (p_w_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_type_3; let a_ = (p_w_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
    { let v_ = 1i32; let a_ = (p_w_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
    l_textured_7 = (((c.m.r32(p_w_1) as i32)) & (32i32));
    if ((l_textured_7) != 0) {
        { let v_ = 2i32; let a_ = (p_w_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
    }
    F::eft_q__EftGlow_SetBaseColors(c, p_w_1, ((l_textured_7) as u8), l_row_5);
    {
        l_i_6 = 0i32;
        'l2: loop {
            if !(((((l_i_6) < (4i32)) as i32)) != 0) { break 'l2; }
            'c3: {
                if (((((c.m.r32(p_w_1) as i32)) & (32i32))) != 0) {
                    { let a1_ = ((c.m.r32(sym::gEftGlowCfg)).wrapping_add(2288)).wrapping_add(((((((l_row_5).wrapping_mul(5i32)).wrapping_add(l_i_6)).wrapping_add(1i32)) as u32)).wrapping_mul(16)); F::eft_q__EftGlow_ColorFromBytes(c, ((p_w_1).wrapping_add(608)).wrapping_add((((l_i_6) as u32)).wrapping_mul(16)), a1_) };
                } else {
                    if ((((((c.m.r32((p_w_1).wrapping_add(28)) as i32)) < (8i32)) as i32)) != 0) {
                        l_j_8 = 0i32;
                    } else {
                        l_j_8 = l_i_6;
                    }
                    { let a1_ = ((c.m.r32(sym::gEftGlowCfg)).wrapping_add(1568)).wrapping_add((((((((c.m.r32((p_w_1).wrapping_add(28)) as i32)).wrapping_mul(5i32)).wrapping_add(l_j_8)).wrapping_add(1i32)) as u32)).wrapping_mul(16)); F::eft_q__EftGlow_ColorFromBytes(c, ((p_w_1).wrapping_add(608)).wrapping_add((((l_i_6) as u32)).wrapping_mul(16)), a1_) };
                }
            }
            { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_SetType", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlow_Setup(c: &mut Ctx, mut p_w_1: u32, mut p_objId_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_Setup", &PROBE_N, &[(p_w_1) as u32, (p_objId_2) as u32], false); }
    let mut l_paramFlags_3: i32 = 0;
    let mut l_type_4: i32 = 0;
    let r_ = 'f: {
    l_paramFlags_3 = F::BtlCharApi_ObjGetParamFlags0(c, p_objId_2);
    l_type_4 = F::BtlCharApi_GetAuraType(c, p_objId_2);
    F::EftGlow_SetType(c, p_w_1, p_objId_2, l_type_4, l_paramFlags_3);
    { let v_ = rt::fdiv(F::BtlCharApi_GetHeight(c, p_objId_2), F32(0x419acccc)); let a_ = (p_w_1).wrapping_add(72); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_Setup", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlowTask_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlowTask_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let _ = F::memset(c, l_w_3, 0i32, 736u32); };
    { let a1_ = (c.m.r32((p_arg_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::EftGlow_Setup(c, l_w_3, a1_) };
    F::EftGlow_Begin(c, l_w_3, p_arg_2);
    { let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(24); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGlowTask_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlowTask_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlowTask_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_objId_3: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_objId_3 = (l_w_2).wrapping_add(76);
    { let a0_ = (c.m.r32(l_objId_3) as i32); F::EftGlow_FreeParts(c, a0_) };
    { let v_ = 0i32; let a_ = l_w_2; c.m.w32(a_, (v_) as u32); };
    { let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(24); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    { let v_ = ((0i32) as u32); let a_ = ({ let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(1776); c.m.r32(a_) }).wrapping_add(((((c.m.r32(l_objId_3) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGlowTask_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlowTask_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlowTask_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_objId_3: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_objId_3 = (l_w_2).wrapping_add(76);
    if (({ let a0_ = (c.m.r32(l_objId_3) as i32); F::BtlCharApi_IsModelNew(c, a0_) }) != 0) {
        { let r_ = 16i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    { let a1_ = (c.m.r32(l_objId_3) as i32); F::EftGlow_Setup(c, l_w_2, a1_) };
    { let r_ = (!(16i32)); let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
        { let a1_ = (c.m.r32(l_objId_3) as i32); F::EftGlow_Step(c, l_w_2, a1_) };
        's1: {
            let sv4: i32 = (c.m.r32((l_w_2).wrapping_add(16)) as i32);
            match sv4 {
                0i32 => {
                    { let v_ = 1i32; let a_ = (l_w_2).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                1i32 => {
                    { let v_ = 2i32; let a_ = (l_w_2).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                2i32 => {
                    break 's1;
                }
                _ => {
                }
            }
        }
        { let a1_ = (c.m.r32(l_objId_3) as i32); F::EftGlow_StepParts(c, l_w_2, a1_) };
    }
    if (((((c.m.r32(l_w_2) as i32)) & (4i32))) != 0) {
        F::BtlTask_SetDead(c, p_task_1);
    } else {
        F::EftGlow_UpdateTextures(c, l_w_2);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGlowTask_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlowTask_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlowTask_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGlowTask_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlowTask_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlowTask_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGlowTask_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlowTask_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlowTask_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_alpha_2: F32 = F32(0);
    let mut l_w_3: u32 = 0;
    let mut l_objId_4: u32 = 0;
    let r_ = 'f: {
    l_alpha_2 = F32(0x3f800000);
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    l_objId_4 = (l_w_3).wrapping_add(76);
    if (({ let a0_ = (c.m.r32(l_objId_4) as i32); F::BtlCharApi_IsHidden(c, a0_) }) != 0) {
        break 'f;
    }
    if (((((({ let a0_ = (c.m.r32(l_objId_4) as i32); F::BtlScene_IsCharInView(c, a0_) }) != 0) && (({ let a0_ = (c.m.r32(l_objId_4) as i32); F::BtlCharApi_ObjTestFlagBit21(c, a0_) }) != 0)) as i32)) != 0) {
        l_alpha_2 = F32(0x3e999999);
    }
    F::Vu0Cur_Push(c);
    { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
    { let a1_ = (c.m.r32(l_objId_4) as i32); F::EftGlow_DrawParts(c, l_w_3, a1_, l_alpha_2) };
    F::Vu0Cur_Pop(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGlowTask_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlowMgr_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlowMgr_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((1792u32) as i32)) }; let a_ = sym::gEftGlow; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gEftGlow); F::memset(c, a0_, 0i32, 1792u32) }; };
    { let v_ = F::BtlScene_GetCharCount(c); let a_ = c.m.r32(sym::gEftGlow); c.m.w32(a_, (v_) as u32); };
    { let v_ = ({ let a_ = c.m.r32(sym::gEftGlow); (c.m.r32(a_) as i32) }).wrapping_mul(70i32); let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = ({ let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(192i32); F::BtlPool_Alloc(c, a0_, a1_) }; let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(8); c.m.w32(a_, v_); };
    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(8); c.m.r32(a_) }; let a2_ = ((({ let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(192i32)) as u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = ({ let a_ = c.m.r32(sym::gEftGlow); (c.m.r32(a_) as i32) }).wrapping_mul(4i32); F::BtlPool_Alloc(c, a0_, a1_) }; let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(1776); c.m.w32(a_, v_); };
    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(1776); c.m.r32(a_) }; let a2_ = ((({ let a_ = c.m.r32(sym::gEftGlow); (c.m.r32(a_) as i32) }).wrapping_mul(4i32)) as u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = F::BtlScene_GetCommonEntry(c, 9i32); let a_ = (((c.m.r32(sym::gEftGlow)).wrapping_add(672)).wrapping_add((((0i32) as u32)).wrapping_mul(552))).wrapping_add(24); c.m.w32(a_, v_); };
    { let v_ = F::BtlScene_GetCommonEntry(c, 8i32); let a_ = (((c.m.r32(sym::gEftGlow)).wrapping_add(672)).wrapping_add((((1i32) as u32)).wrapping_mul(552))).wrapping_add(24); c.m.w32(a_, v_); };
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = (((c.m.r32(sym::gEftGlow)).wrapping_add(672)).wrapping_add((((l_i_2) as u32)).wrapping_mul(552))).wrapping_add(32); let a1_ = { let a_ = (((c.m.r32(sym::gEftGlow)).wrapping_add(672)).wrapping_add((((l_i_2) as u32)).wrapping_mul(552))).wrapping_add(24); c.m.r32(a_) }; F::EftTexSet_Load32(c, a0_, a1_) };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    { let v_ = F::BtlScene_GetCommonEntry(c, 7i32); let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(1780); c.m.w32(a_, v_); };
    { let v_ = { let v_ = { let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(1780); c.m.r32(a_) }; let a_ = sym::gEftGlowCfg; c.m.w32(a_, v_); v_ }; let a_ = sym::gEftGlowCfg2; c.m.w32(a_, v_); };
    {
        l_i_2 = 0i32;
        'l3: loop {
            if !(((((l_i_2) < ({ let a_ = c.m.r32(sym::gEftGlow); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l3; }
            'c4: {
                { let v_ = ((0i32) as u32); let a_ = ({ let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(1776); c.m.r32(a_) }).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    F::EftGlow_BuildTables(c);
    { let v_ = { let a1_ = { let a_ = c.m.r32(sym::gEftGlow); (c.m.r32(a_) as i32) }; F::BtlTask_CreateChildList(c, p_task_1, ((((a1_) as i64 as u64)) as u32), ((((((736u32) as i32)) as i64 as u64)) as u32)) }; let a_ = sym::gEftGlowList; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGlowMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlowMgr_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlowMgr_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = { let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(1776); c.m.r32(a_) }; F::BtlPool_Free(c, a0_, a1_) };
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = { let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(8); c.m.r32(a_) }; F::BtlPool_Free(c, a0_, a1_) };
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftGlow); F::BtlPool_Free(c, a0_, a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftGlow; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGlowMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlowMgr_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlowMgr_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gEftGlow)).wrapping_add(672)).wrapping_add((((l_i_2) as u32)).wrapping_mul(552)); c.m.w32(a_, (v_) as u32); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGlowMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlow_Start(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_Start", &PROBE_N, &[(p_objId_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_task_2: u32 = 0;
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
        if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(1776); c.m.r32(a_) }).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
            { let v_ = p_objId_1; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            l_task_2 = { let a0_ = c.m.r32(sym::gEftGlowList); F::BtlTaskList_AddTail(c, a0_, sym::gEftGlowClass, fp) };
            if (((((l_task_2) == (((0i32) as u32))) as i32)) != 0) {
                break 'f 0i32;
            }
            { let v_ = l_task_2; let a_ = ({ let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(1776); c.m.r32(a_) }).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        } else {
            l_w_3 = { let a_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(1776); c.m.r32(a_) }).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(56); c.m.r32(a_) };
            F::EftGlow_Begin(c, l_w_3, (l_w_3).wrapping_add(76));
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_Start", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlow_FadeOut(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_FadeOut", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_task_2: u32 = 0;
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    l_task_2 = { let a_ = ({ let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(1776); c.m.r32(a_) }).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    if (((((l_task_2) != (((0i32) as u32))) as i32)) != 0) {
        l_w_3 = c.m.r32((l_task_2).wrapping_add(56));
        if ((((!(((((c.m.r32(l_w_3) as i32)) & (1i32))) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        if (((((c.m.r32(l_w_3) as i32)) & (2i32))) != 0) {
            break 'f 0i32;
        }
        { let r_ = 2i32; let a_ = l_w_3; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let v_ = 2i32; let a_ = (l_w_3).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (c.m.r32(sym::gEftGlowCfg)).wrapping_add(428); F32(c.m.r32(a_)) }; let a_ = (l_w_3).wrapping_add(56); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (l_w_3).wrapping_add(60); c.m.w32(a_, (v_).0); };
        { let v_ = 9i32; let a_ = (l_w_3).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_FadeOut", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlow_Kill(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_Kill", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_task_2: u32 = 0;
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    l_task_2 = { let a_ = ({ let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(1776); c.m.r32(a_) }).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    if (((((l_task_2) != (((0i32) as u32))) as i32)) != 0) {
        l_w_3 = c.m.r32((l_task_2).wrapping_add(56));
        if ((((!(((((c.m.r32(l_w_3) as i32)) & (1i32))) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        { let r_ = 4i32; let a_ = l_w_3; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_Kill", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlow_Request(c: &mut Ctx, mut p_objId_1: i32, mut p_cmd_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_Request", &PROBE_N, &[(p_objId_1) as u32, (p_cmd_2) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftGlow)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    's1: {
        let sv3: i32 = p_cmd_2;
        match sv3 {
            0i32 => {
                { let _ = F::EftGlow_Start(c, p_objId_1); };
                break 's1;
            }
            1i32 => {
                { let _ = F::EftGlow_FadeOut(c, p_objId_1); };
                break 's1;
            }
            2i32 => {
                { let _ = F::EftGlow_Kill(c, p_objId_1); };
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_Request", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlow_IsActive(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_IsActive", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_ret_2: i32 = 0;
    let mut l_task_3: u32 = 0;
    let r_ = 'f: {
    l_ret_2 = 1i32;
    if (((((c.m.r32(sym::gEftGlow)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_task_3 = { let a_ = ({ let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(1776); c.m.r32(a_) }).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    if (((((l_task_3) != (((0i32) as u32))) as i32)) != 0) {
        if ((((!(((({ let a_ = c.m.r32((l_task_3).wrapping_add(56)); (c.m.r32(a_) as i32) }) & (1i32))) != 0)) as i32)) != 0) {
            l_ret_2 = 0i32;
        }
    } else {
        break 'f 0i32;
    }
    break 'f l_ret_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_IsActive", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlow_ChangeType(c: &mut Ctx, mut p_objId_1: i32, mut p_type_2: i32, mut p_paramFlags_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_ChangeType", &PROBE_N, &[(p_objId_1) as u32, (p_type_2) as u32, (p_paramFlags_3) as u32], false); }
    let r_ = 'f: {
    if (((((p_objId_1) < (2i32)) as i32)) != 0) {
        if (((((c.m.r32(sym::gEftGlow)) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if (((((p_type_2) < (0i32)) as i32)) != 0) {
            break 'f;
        }
        if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(1776); c.m.r32(a_) }).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        { let a0_ = { let a_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(1776); c.m.r32(a_) }).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(56); c.m.r32(a_) }; F::EftGlow_SetType(c, a0_, p_objId_1, p_type_2, p_paramFlags_3) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_ChangeType", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlow_GetColors(c: &mut Ctx, mut p_type_1: i32, mut p_paramFlags_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_GetColors", &PROBE_N, &[(p_type_1) as u32, (p_paramFlags_2) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftGlow)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    if (((((p_type_1) < (0i32)) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    if ((((!((((p_paramFlags_2) & (94i32))) != 0)) as i32)) != 0) {
        if (((((p_type_1) >= (9i32)) as i32)) != 0) {
            break 'f ((0i32) as u32);
        }
        break 'f ((c.m.r32(sym::gEftGlowCfg)).wrapping_add(1568)).wrapping_add(((((p_type_1).wrapping_mul(5i32)) as u32)).wrapping_mul(16));
    }
    if (((((p_type_1) >= (5i32)) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    break 'f ((c.m.r32(sym::gEftGlowCfg)).wrapping_add(2288)).wrapping_add(((((p_type_1).wrapping_mul(5i32)) as u32)).wrapping_mul(16));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_GetColors", &PROBE_N, &[], true); }
    r_
}

pub fn EftRushBurst_Emit(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32, mut p_set_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRushBurst_Emit", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32, (p_set_3) as u32], false); }
    let mut l_w_4: u32 = 0;
    let mut l_group_5: i32 = 0;
    let mut l_sub_6: i32 = 0;
    let mut l_def_7: u32 = 0;
    let mut l_spawn_8: i32 = 0;
    let r_ = 'f: {
    l_w_4 = c.m.r32((p_task_2).wrapping_add(56));
    {
        l_group_5 = 0i32;
        'l1: loop {
            if !(((((l_group_5) < (19i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((({ let a_ = c.m.r32(p_set_3); c.m.r32(a_) }) & ((((1i32).wrapping_shl((l_group_5) as u32)) as u32)))) != 0) {
                    l_def_7 = c.m.r32(((p_set_3).wrapping_add(12)).wrapping_add((((l_group_5) as u32)).wrapping_mul(16)));
                    {
                        l_sub_6 = 0i32;
                        'l3: loop {
                            if !(((((l_sub_6) < (((c.m.r8((l_def_7).wrapping_add(1))) as i32))) as i32)) != 0) { break 'l3; }
                            'c4: {
                                l_spawn_8 = { let a5_ = ((((c.m.r32((l_w_4).wrapping_add(1376))) & (((2i32) as u32)))) as i32); let a7_ = ((c.m.r32((l_w_4).wrapping_add(1384))) as i32); F::EftEmit_GetFlagsFromMask(c, p_set_3, (l_w_4).wrapping_add(68), p_objId_1, l_group_5, l_sub_6, a5_, 0i32, a7_) };
                                if (((((l_spawn_8) != (0i32)) as i32)) != 0) {
                                    { let a11_ = F32(c.m.r32((l_w_4).wrapping_add(1380))); F::EftEmit_Spawn(c, p_set_3, (l_w_4).wrapping_add(68), (l_w_4).wrapping_add(784), l_w_4, (l_w_4).wrapping_add(16), p_objId_1, 0i32, 3i32, l_group_5, l_sub_6, l_spawn_8, a11_) };
                                }
                            }
                            { let o_ = l_sub_6; let n_ = (o_).wrapping_add(1); l_sub_6 = n_; };
                        }
                    }
                }
            }
            { let o_ = l_group_5; let n_ = (o_).wrapping_add(1); l_group_5 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRushBurst_Emit", &PROBE_N, &[], true); }
    r_
}

pub fn EftRushBurst_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRushBurst_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_mgr_3: u32 = 0;
    let mut l_w_4: u32 = 0;
    let mut l_dir_5: u32 = 0;
    let mut l_set_6: u32 = 0;
    let r_ = 'f: {
    l_mgr_3 = { let a_ = (F::BtlTask_GetParent(c, p_task_1)).wrapping_add(56); c.m.r32(a_) };
    l_w_4 = c.m.r32((p_task_1).wrapping_add(56));
    l_dir_5 = (l_w_4).wrapping_add(16);
    l_set_6 = c.m.r32(l_mgr_3);
    { let _ = F::memset(c, l_w_4, 0i32, 1408u32); };
    { let s_ = p_arg_2; let d_ = (l_w_4).wrapping_add(32); rt::memcpy(c, d_, s_, 32); };
    { let r_ = ((1i32) as u32); let a_ = (l_w_4).wrapping_add(1376); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    { let v_ = F32(c.m.r32((p_arg_2).wrapping_add(24))); let a_ = (l_w_4).wrapping_add(1380); c.m.w32(a_, (v_).0); };
    { let a0_ = (c.m.r32((p_arg_2).wrapping_add(16)) as i32); let a1_ = (c.m.r32((p_arg_2).wrapping_add(20)) as i32); F::BtlCharApi_GetNodePos(c, a0_, a1_, l_w_4) };
    F::Vec4_Copy(c, l_dir_5, p_arg_2);
    { let v_ = F32(0x3f800000); let a_ = ((l_w_4).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
    F::Vec3_Normalize(c, l_dir_5, l_dir_5);
    { let v_ = l_set_6; let a_ = (l_w_4).wrapping_add(64); c.m.w32(a_, v_); };
    F::EftEmit_InitState(c, l_set_6, (l_w_4).wrapping_add(68));
    { let r_ = ((1i32) as u32); let a_ = (l_w_4).wrapping_add(1384); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    { let a2_ = (c.m.r32((p_arg_2).wrapping_add(20)) as i32); F::EftEmit_SetNode(c, (l_w_4).wrapping_add(784), 0i32, a2_, ((0i32) as u32)) };
    { let r_ = ((32i32) as u32); let a_ = (l_w_4).wrapping_add(1376); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    { let v_ = rt::i2f({ let a0_ = c.m.r32((l_w_4).wrapping_add(64)); F::EftEmit_GetEndFrames(c, a0_) }); let a_ = (l_w_4).wrapping_add(1392); c.m.w32(a_, (v_).0); };
    { let a1_ = (if ((((((c.m.r32((p_arg_2).wrapping_add(16)) as i32)) == (0i32)) as i32)) != 0) { 2048i32 } else { 4096i32 }); F::BtlTask_SetOwnerTag(c, p_task_1, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRushBurst_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftRushBurst_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRushBurst_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_arg_3: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_arg_3 = (l_w_2).wrapping_add(32);
    { let a0_ = c.m.r32((l_w_2).wrapping_add(64)); F::EftEmit_TermState(c, a0_, (l_w_2).wrapping_add(68)) };
    { let v_ = ((0i32) as u32); let a_ = (l_w_2).wrapping_add(1376); c.m.w32(a_, v_); };
    { let a0_ = (c.m.r32((l_arg_3).wrapping_add(16)) as i32); F::EftCharSlot_Clear2(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRushBurst_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftRushBurst_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRushBurst_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_arg_3: u32 = 0;
    let mut l_nodes_4: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_arg_3 = (l_w_2).wrapping_add(32);
    if ((((!(({ let a0_ = (c.m.r32((l_arg_3).wrapping_add(16)) as i32); F::BtlScene_IsEffectStopped(c, a0_, 3i32) }) != 0)) as i32)) != 0) {
        if ((((c.m.r32((l_w_2).wrapping_add(1376))) & (((4i32) as u32)))) != 0) {
            if ((((!((((c.m.r32((l_w_2).wrapping_add(1376))) & (((8i32) as u32)))) != 0)) as i32)) != 0) {
                { let r_ = ((2i32) as u32); let a_ = (l_w_2).wrapping_add(1384); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                l_nodes_4 = (l_w_2).wrapping_add(784);
                { let a2_ = (c.m.r32((l_arg_3).wrapping_add(20)) as i32); F::EftEmit_SetNode(c, l_nodes_4, 1i32, a2_, ((0i32) as u32)) };
                { let r_ = ((8i32) as u32); let a_ = (l_w_2).wrapping_add(1376); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
            } else {
                l_nodes_4 = (l_w_2).wrapping_add(784);
            }
        } else {
            l_nodes_4 = (l_w_2).wrapping_add(784);
        }
        { let a0_ = (c.m.r32((l_arg_3).wrapping_add(16)) as i32); let a1_ = (c.m.r32((l_arg_3).wrapping_add(20)) as i32); F::BtlCharApi_GetNodePos(c, a0_, a1_, l_w_2) };
        { let a0_ = (c.m.r32((l_arg_3).wrapping_add(16)) as i32); F::EftEmit_RefreshFixedNodes(c, a0_, l_nodes_4) };
        { let a0_ = (c.m.r32((l_arg_3).wrapping_add(16)) as i32); let a2_ = c.m.r32((l_w_2).wrapping_add(64)); F::EftRushBurst_Emit(c, a0_, p_task_1, a2_) };
        if ((((c.m.r32((l_w_2).wrapping_add(1376))) & (((2i32) as u32)))) != 0) {
            { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(1388); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::fge(F32(c.m.r32((l_w_2).wrapping_add(1388))), F32(c.m.r32((l_w_2).wrapping_add(1392)))) as i32)) != 0) {
                { let r_ = ((16i32) as u32); let a_ = (l_w_2).wrapping_add(1376); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
            }
        }
    }
    if ((((c.m.r32((l_w_2).wrapping_add(1376))) & (((16i32) as u32)))) != 0) {
        F::BtlTask_SetDead(c, p_task_1);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRushBurst_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftRushBurst_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRushBurst_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!((((c.m.r32((l_w_2).wrapping_add(1376))) & (((128i32) as u32)))) != 0)) as i32)) != 0) {
        { let r_ = ((128i32) as u32); let a_ = (l_w_2).wrapping_add(1376); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        { let a0_ = c.m.r32((l_w_2).wrapping_add(64)); F::EftEmit_KillAll(c, a0_, (l_w_2).wrapping_add(68)) };
    }
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRushBurst_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftRushBurst_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRushBurst_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((c.m.r32((l_w_2).wrapping_add(1376))) & (((64i32) as u32)))) != 0) {
        if ((((!(({ let a0_ = c.m.r32((l_w_2).wrapping_add(64)); F::EftEmit_UpdateAlive(c, a0_, (l_w_2).wrapping_add(68)) }) != 0)) as i32)) != 0) {
            { let r_ = (((!(32i32))) as u32); let a_ = (l_w_2).wrapping_add(1376); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
        } else {
            { let r_ = ((32i32) as u32); let a_ = (l_w_2).wrapping_add(1376); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        }
        { let v_ = ((0i32) as u32); let a_ = (l_w_2).wrapping_add(1384); c.m.w32(a_, v_); };
    }
    { let r_ = ((64i32) as u32); let a_ = (l_w_2).wrapping_add(1376); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRushBurst_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftRushBurst_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRushBurst_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRushBurst_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftRushBurstMgr_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRushBurstMgr_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_mgr_3: u32 = 0;
    let mut l_set_4: u32 = 0;
    let mut l_pack_5: u32 = 0;
    let mut l_list_6: u32 = 0;
    let r_ = 'f: {
    l_mgr_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((804u32) as i32)) }; let a_ = l_mgr_3; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(l_mgr_3); F::memset(c, a0_, 0i32, 804u32) }; };
    l_set_4 = c.m.r32(l_mgr_3);
    l_pack_5 = { let a0_ = (c.m.r32((p_arg_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::BtlScene_GetCharPackEntry(c, a0_, 7i32) };
    { let v_ = l_pack_5; let a_ = (l_set_4).wrapping_add(800); c.m.w32(a_, v_); };
    if (((((l_pack_5) != (((0i32) as u32))) as i32)) != 0) {
        F::EftEmit_LoadSet(c, ((0i32) as u32), l_set_4, ((((0i32) as i64 as u64)) as u32), l_pack_5, 2i32, 1i32);
    }
    l_list_6 = F::BtlTask_CreateChildList(c, p_task_1, ((((4i32) as i64 as u64)) as u32), ((((((1408u32) as i32)) as i64 as u64)) as u32));
    { let a0_ = (c.m.r32((p_arg_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a1_ = (c.m.r32((p_arg_2).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); F::EftChar_SetList(c, a0_, a1_, l_list_6) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRushBurstMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftRushBurstMgr_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRushBurstMgr_Term", &PROBE_N, &[(p_task_1) as u32], false); }
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
    if rt::probing() { crate::host::harness::probe(c, "EftRushBurstMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftRushBurstMgr_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRushBurstMgr_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_mgr_2: u32 = 0;
    let r_ = 'f: {
    l_mgr_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((({ let a_ = (c.m.r32(l_mgr_2)).wrapping_add(800); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(l_mgr_2); F::EftEmit_BeginFrame(c, a0_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRushBurstMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftRushBurst_Start(c: &mut Ctx, mut p_arg_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRushBurst_Start", &PROBE_N, &[(p_arg_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_list_2: u32 = 0;
    let mut l_task_3: u32 = 0;
    let r_ = 'f: {
        { let s_ = p_arg_1; let d_ = fp; rt::memcpy(c, d_, s_, 32); };
        l_list_2 = { let a0_ = (c.m.r32((fp).wrapping_add(16)) as i32); F::EftChar_GetList(c, a0_, 5i32) };
        if (((((l_list_2) != (((0i32) as u32))) as i32)) != 0) {
            l_task_3 = F::BtlTaskList_AddTail(c, l_list_2, sym::gEftRushBurstClass, fp);
            if (((((l_task_3) != (((0i32) as u32))) as i32)) != 0) {
                { let a0_ = (c.m.r32((fp).wrapping_add(16)) as i32); F::EftCharSlot_Set2(c, a0_, l_task_3) };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftRushBurst_Start", &PROBE_N, &[], true); }
    r_
}

pub fn EftRushBurst_Stage2(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRushBurst_Stage2", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_task_2: u32 = 0;
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    l_task_2 = F::EftCharSlot_Get2(c, p_objId_1);
    if (((((l_task_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((l_task_2).wrapping_add(56));
    if ((((!((((c.m.r32((l_w_3).wrapping_add(1376))) & (((1i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let r_ = ((4i32) as u32); let a_ = (l_w_3).wrapping_add(1376); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRushBurst_Stage2", &PROBE_N, &[], true); }
    r_
}

pub fn EftRushBurst_Stop(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRushBurst_Stop", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_task_2: u32 = 0;
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    l_task_2 = F::EftCharSlot_Get2(c, p_objId_1);
    if (((((l_task_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((l_task_2).wrapping_add(56));
    if ((((!((((c.m.r32((l_w_3).wrapping_add(1376))) & (((1i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let r_ = ((2i32) as u32); let a_ = (l_w_3).wrapping_add(1376); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRushBurst_Stop", &PROBE_N, &[], true); }
    r_
}

pub fn EftRushBurst_Nop(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRushBurst_Nop", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRushBurst_Nop", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharRoot_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharRoot_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let mut l_size_3: i32 = 0;
    let mut l_c_4: u32 = 0;
    let r_ = 'f: {
    F::BtlPool_SetCurrent(c, 6i32);
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((12u32) as i32)) }; let a_ = sym::gEftChar; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gEftChar); F::memset(c, a0_, 0i32, 12u32) }; };
    { let v_ = F::BtlScene_GetCharCount(c); let a_ = (c.m.r32(sym::gEftChar)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    l_size_3 = ((((({ let a_ = (c.m.r32(sym::gEftChar)).wrapping_add(8); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48u32)) as i32);
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, l_size_3) }; let a_ = (c.m.r32(sym::gEftChar)).wrapping_add(4); c.m.w32(a_, v_); };
    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gEftChar)).wrapping_add(4); c.m.r32(a_) }; F::memset(c, a0_, 0i32, ((l_size_3) as u32)) }; };
    { let v_ = { let a1_ = { let a_ = (c.m.r32(sym::gEftChar)).wrapping_add(8); (c.m.r32(a_) as i32) }; F::BtlTask_CreateChildList(c, p_task_1, ((((a1_) as i64 as u64)) as u32), ((((0i32) as i64 as u64)) as u32)) }; let a_ = c.m.r32(sym::gEftChar); c.m.w32(a_, v_); };
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < ({ let a_ = (c.m.r32(sym::gEftChar)).wrapping_add(8); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l1; }
            'c2: {
                l_c_4 = ({ let a_ = (c.m.r32(sym::gEftChar)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((l_i_2) as u32)).wrapping_mul(48));
                { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gEftChar); c.m.r32(a_) }; F::BtlTaskList_AddTail(c, a0_, sym::gEftCharClass, ((l_i_2) as u32)) }; let a_ = l_c_4; c.m.w32(a_, v_); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharRoot_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharRoot_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharRoot_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::BtlPool_SetCurrent(c, 6i32);
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = { let a_ = (c.m.r32(sym::gEftChar)).wrapping_add(4); c.m.r32(a_) }; F::BtlPool_Free(c, a0_, a1_) };
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftChar); F::BtlPool_Free(c, a0_, a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftChar; c.m.w32(a_, v_); };
    F::BtlPool_Reset(c, 6i32);
    F::BtlPool_Reset(c, 7i32);
    F::BtlPool_Reset(c, 8i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharRoot_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharRoot_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharRoot_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharRoot_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftChar_Init(c: &mut Ctx, mut p_task_1: u32, mut p_chr_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChar_Init", &PROBE_N, &[(p_task_1) as u32, (p_chr_2) as u32], false); }
    let mut l_c_3: u32 = 0;
    let mut l_kind_4: i32 = 0;
    let mut l_on_5: i32 = 0;
    let mut l_mode_6: i32 = 0;
    let r_ = 'f: {
    F::EftChar_SetPool(c, p_chr_2);
    { let v_ = ((p_chr_2) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
    l_c_3 = ({ let a_ = (c.m.r32(sym::gEftChar)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((p_chr_2) as u32)).wrapping_mul(48));
    { let v_ = F::BtlTask_CreateChildList(c, p_task_1, ((((10i32) as i64 as u64)) as u32), ((((4i32) as i64 as u64)) as u32)); let a_ = (l_c_3).wrapping_add(4); c.m.w32(a_, v_); };
    {
        l_kind_4 = 0i32;
        'l1: loop {
            if !(((((l_kind_4) < (10i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_mode_6 = 0i32;
                if (((((l_kind_4) == (3i32)) as i32)) != 0) {
                    l_on_5 = F::BtlScene_TestCharPackBit(c, p_chr_2, 3i32);
                } else {
                    if (((((l_kind_4) == (6i32)) as i32)) != 0) {
                        l_on_5 = 0i32;
                    } else {
                        l_on_5 = F::BtlScene_TestCharPackBit(c, p_chr_2, (l_kind_4).wrapping_add(1i32));
                    }
                }
                if ((((((F::BtlCharApi_HasKiBlastType2(c, p_chr_2)) != 0) && (((((l_kind_4) == (1i32)) as i32)) != 0)) as i32)) != 0) {
                    l_on_5 = 1i32;
                    l_mode_6 = 2i32;
                }
                if ((((((F::BtlCharApi_HasKiBlastType3(c, p_chr_2)) != 0) && (((((l_kind_4) == (2i32)) as i32)) != 0)) as i32)) != 0) {
                    l_on_5 = 1i32;
                    l_mode_6 = 4i32;
                }
                if ((((((F::BtlCharApi_HasKiBlastType3(c, p_chr_2)) != 0) && (((((l_kind_4) == (3i32)) as i32)) != 0)) as i32)) != 0) {
                    l_on_5 = 0i32;
                }
                if ((l_on_5) != 0) {
                    F::EftChar_CreateKind(c, p_chr_2, l_kind_4, l_mode_6);
                }
            }
            { let o_ = l_kind_4; let n_ = (o_).wrapping_add(1); l_kind_4 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChar_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftChar_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChar_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_chr_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_c_4: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = ((c.m.r8((p_task_1).wrapping_add(1))) as i32);
    if (((((c.m.r32(sym::gEftChar)) != (((0i32) as u32))) as i32)) != 0) {
        l_c_4 = ({ let a_ = (c.m.r32(sym::gEftChar)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((l_chr_2) as u32)).wrapping_mul(48));
        {
            l_i_3 = (10i32).wrapping_sub(1i32);
            'l1: loop {
                if !(((((l_i_3) >= (0i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = ((0i32) as u32); let a_ = ((l_c_4).wrapping_add(8)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_sub(1); l_i_3 = n_; };
            }
        }
    }
    F::EftChar_ResetPool(c, l_chr_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChar_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftChar_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChar_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChar_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftChar_SetPool(c: &mut Ctx, mut p_chr_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChar_SetPool", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    F::BtlPool_SetCurrent(c, (if (((((p_chr_1) == (0i32)) as i32)) != 0) { 7i32 } else { 8i32 }));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChar_SetPool", &PROBE_N, &[], true); }
    r_
}

pub fn EftChar_ResetPool(c: &mut Ctx, mut p_chr_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChar_ResetPool", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    F::BtlPool_Reset(c, (if (((((p_chr_1) == (0i32)) as i32)) != 0) { 7i32 } else { 8i32 }));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChar_ResetPool", &PROBE_N, &[], true); }
    r_
}

pub fn EftChar_CreateKind(c: &mut Ctx, mut p_chr_1: i32, mut p_kind_2: i32, mut p_mode_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChar_CreateKind", &PROBE_N, &[(p_chr_1) as u32, (p_kind_2) as u32, (p_mode_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_c_4: u32 = 0;
    let mut l_cls_5: u32 = 0;
    let r_ = 'f: {
        l_c_4 = ({ let a_ = (c.m.r32(sym::gEftChar)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((p_chr_1) as u32)).wrapping_mul(48));
        l_cls_5 = ((0i32) as u32);
        if (((((p_mode_3) != (0i32)) as i32)) != 0) {
            if ((((p_mode_3) & (2i32))) != 0) {
                l_cls_5 = sym::D_002C3E00;
            }
            if ((((p_mode_3) & (4i32))) != 0) {
                l_cls_5 = sym::gEftKiBombMgrClass;
            }
        } else {
            l_cls_5 = c.m.r32((sym::gEftCharKindClass).wrapping_add((((p_kind_2) as u32)).wrapping_mul(4)));
        }
        { let v_ = ((0i32) as u32); let a_ = ((l_c_4).wrapping_add(8)).wrapping_add((((p_kind_2) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = p_chr_1; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = p_kind_2; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let _ = { let a0_ = c.m.r32((l_c_4).wrapping_add(4)); F::BtlTaskList_AddTail(c, a0_, l_cls_5, fp) }; };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftChar_CreateKind", &PROBE_N, &[], true); }
    r_
}

pub fn EftChar_Kill(c: &mut Ctx, mut p_chr_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChar_Kill", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_c_2: u32 = 0;
    let r_ = 'f: {
    l_c_2 = ({ let a_ = (c.m.r32(sym::gEftChar)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((p_chr_1) as u32)).wrapping_mul(48));
    if (((((c.m.r32(l_c_2)) != (((0i32) as u32))) as i32)) != 0) {
        F::BtlScene_Reset(c, (if (((((p_chr_1) == (0i32)) as i32)) != 0) { 2i32 } else { 3i32 }));
        F::EftChar_SetPool(c, p_chr_1);
        { let a0_ = c.m.r32(l_c_2); F::BtlTask_Kill(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = l_c_2; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChar_Kill", &PROBE_N, &[], true); }
    r_
}

pub fn EftChar_Create(c: &mut Ctx, mut p_chr_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChar_Create", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_c_2: u32 = 0;
    let r_ = 'f: {
    l_c_2 = ({ let a_ = (c.m.r32(sym::gEftChar)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((p_chr_1) as u32)).wrapping_mul(48));
    if (((((c.m.r32(l_c_2)) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gEftChar); c.m.r32(a_) }; F::BtlTaskList_AddTail(c, a0_, sym::gEftCharClass, ((p_chr_1) as u32)) }; let a_ = l_c_2; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChar_Create", &PROBE_N, &[], true); }
    r_
}

pub fn EftChar_SetList(c: &mut Ctx, mut p_chr_1: i32, mut p_kind_2: i32, mut p_list_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChar_SetList", &PROBE_N, &[(p_chr_1) as u32, (p_kind_2) as u32, (p_list_3) as u32], false); }
    let mut l_c_4: u32 = 0;
    let r_ = 'f: {
    l_c_4 = ({ let a_ = (c.m.r32(sym::gEftChar)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((p_chr_1) as u32)).wrapping_mul(48));
    { let v_ = p_list_3; let a_ = ((l_c_4).wrapping_add(8)).wrapping_add((((p_kind_2) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChar_SetList", &PROBE_N, &[], true); }
    r_
}

pub fn EftChar_GetList(c: &mut Ctx, mut p_chr_1: i32, mut p_kind_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChar_GetList", &PROBE_N, &[(p_chr_1) as u32, (p_kind_2) as u32], false); }
    let mut l_c_3: u32 = 0;
    let r_ = 'f: {
    l_c_3 = ({ let a_ = (c.m.r32(sym::gEftChar)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((p_chr_1) as u32)).wrapping_mul(48));
    break 'f c.m.r32(((l_c_3).wrapping_add(8)).wrapping_add((((p_kind_2) as u32)).wrapping_mul(4)));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChar_GetList", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharNull_Start(c: &mut Ctx, mut p_chr_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharNull_Start", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_list_2: u32 = 0;
    let r_ = 'f: {
    l_list_2 = F::EftChar_GetList(c, p_chr_1, 0i32);
    if (((((l_list_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlTaskList_AddTail(c, l_list_2, sym::gEftCharNullClass, ((p_chr_1) as u32));
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharNull_Start", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharNullMgr_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharNullMgr_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_mgr_3: u32 = 0;
    let mut l_list_4: u32 = 0;
    let r_ = 'f: {
    l_mgr_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let v_ = ((0i32) as u32); let a_ = l_mgr_3; c.m.w32(a_, v_); };
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, 4i32) }; let a_ = l_mgr_3; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(l_mgr_3); F::memset(c, a0_, 0i32, ((4i32) as u32)) }; };
    l_list_4 = F::BtlTask_CreateChildList(c, p_task_1, ((((1i32) as i64 as u64)) as u32), ((((4i32) as i64 as u64)) as u32));
    { let a0_ = (c.m.r32((p_arg_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a1_ = (c.m.r32((p_arg_2).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); F::EftChar_SetList(c, a0_, a1_, l_list_4) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharNullMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharNullMgr_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharNullMgr_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_mgr_2: u32 = 0;
    let r_ = 'f: {
    l_mgr_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((c.m.r32(l_mgr_2)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(l_mgr_2); F::BtlPool_Free(c, a0_, a1_) };
        { let v_ = ((0i32) as u32); let a_ = l_mgr_2; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharNullMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharNullMgr_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharNullMgr_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharNullMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharNull_Init(c: &mut Ctx, mut p_task_1: u32, mut p_chr_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharNull_Init", &PROBE_N, &[(p_task_1) as u32, (p_chr_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let v_ = 0i32; let a_ = l_w_3; c.m.w32(a_, (v_) as u32); };
    F::BtlTask_SetOwnerTag(c, p_task_1, (if (((((p_chr_2) == (0i32)) as i32)) != 0) { 2048i32 } else { 4096i32 }));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharNull_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharNull_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharNull_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharNull_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharNull_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharNull_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharNull_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharNull_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharNull_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharNull_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharNull_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharNull_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharNull_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharNull_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharNull_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharNull_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftFlash_Start(c: &mut Ctx, mut p_arg_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftFlash_Start", &PROBE_N, &[(p_arg_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = (1i32).wrapping_neg(); let a_ = (p_arg_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
    break 'f { let a0_ = { let a_ = c.m.r32(sym::gEftFlash); c.m.r32(a_) }; F::BtlTaskList_AddTail(c, a0_, sym::gEftFlashClass, p_arg_1) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftFlash_Start", &PROBE_N, &[], true); }
    r_
}

pub fn EftFlash_StartOwned(c: &mut Ctx, mut p_arg_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftFlash_StartOwned", &PROBE_N, &[(p_arg_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a0_ = { let a_ = c.m.r32(sym::gEftFlash); c.m.r32(a_) }; F::BtlTaskList_AddTail(c, a0_, sym::gEftFlashClass, p_arg_1) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftFlash_StartOwned", &PROBE_N, &[], true); }
    r_
}

pub fn EftFlash_GetPhase(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftFlash_GetPhase", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftFlash_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((rt::bf_get_u(c, p_task_1, 0, 1) as u8)) != 0) {
        break 'f 0i32;
    }
    break 'f { let a_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(64); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftFlash_GetPhase", &PROBE_N, &[], true); }
    r_
}

pub fn EftFlash_SetPause(c: &mut Ctx, mut p_task_1: u32, mut p_pause_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftFlash_SetPause", &PROBE_N, &[(p_task_1) as u32, (p_pause_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftFlash_Update)) as i32)) != 0) {
        break 'f;
    }
    if (((rt::bf_get_u(c, p_task_1, 0, 1) as u8)) != 0) {
        break 'f;
    }
    { let v_ = p_pause_2; let a_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftFlash_SetPause", &PROBE_N, &[], true); }
    r_
}

pub fn EftFlashMgr_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftFlashMgr_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((16u32) as i32)) }; let a_ = sym::gEftFlash; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gEftFlash); F::memset(c, a0_, 0i32, 16u32) }; };
    { let v_ = F::BtlTask_CreateChildList(c, p_task_1, ((((1i32) as i64 as u64)) as u32), ((((((80u32) as i32)) as i64 as u64)) as u32)); let a_ = c.m.r32(sym::gEftFlash); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftFlashMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftFlashMgr_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftFlashMgr_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftFlash); F::BtlPool_Free(c, a0_, a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftFlash; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftFlashMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftFlashMgr_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftFlashMgr_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_count_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
        l_count_2 = F::BtlScene_GetCharCount(c);
        if (((((c.m.r32(sym::gEftFlash)) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < (l_count_2)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if ((F::BtlCharApi_IsFlag8Action104(c, l_i_3)) != 0) {
                        rt::memset(c, fp, 0, 48);
                        rt::memset(c, fp.wrapping_add(0), 0, 16);
                        { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(0).wrapping_add(0), (v_).0); }
                        { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(0).wrapping_add(4), (v_).0); }
                        { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(0).wrapping_add(8), (v_).0); }
                        { let v_ = F32(0x43000000); c.m.w32(fp.wrapping_add(0).wrapping_add(12), (v_).0); }
                        { let v_ = F32(0x3f000000); c.m.w32(fp.wrapping_add(16), (v_).0); }
                        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(20), (v_).0); }
                        { let v_ = F32(0x3f000000); c.m.w32(fp.wrapping_add(24), (v_).0); }
                        { let v_ = (1i32).wrapping_neg(); c.m.w32(fp.wrapping_add(28), (v_) as u32); }
                        { let v_ = 0i32; c.m.w32(fp.wrapping_add(32), (v_) as u32); }
                        if ((((!(((({ let a_ = (c.m.r32(sym::gEftFlash)).wrapping_add(4); (c.m.r32(a_) as i32) }) & (1i32))) != 0)) as i32)) != 0) {
                            { let _ = F::EftFlash_Start(c, fp); };
                            if (((((l_i_3) == (0i32)) as i32)) != 0) {
                                { let r_ = 2i32; let a_ = (c.m.r32(sym::gEftFlash)).wrapping_add(4); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            } else {
                                { let r_ = 4i32; let a_ = (c.m.r32(sym::gEftFlash)).wrapping_add(4); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            }
                        }
                    }
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        if (((({ let a_ = (c.m.r32(sym::gEftFlash)).wrapping_add(4); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
            { let v_ = F::EftFlash_GetRushState(c); let a_ = (c.m.r32(sym::gEftFlash)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftFlashMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftFlashMgr_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftFlashMgr_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftFlash)) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gEftFlash)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftFlashMgr_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftFlash_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftFlash_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let _ = F::memset(c, l_w_3, 0i32, 80u32); };
    { let s_ = p_arg_2; let d_ = l_w_3; rt::memcpy(c, d_, s_, 48); };
    { let r_ = F32(0x41f00000); let a_ = (l_w_3).wrapping_add(16); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    { let r_ = F32(0x41f00000); let a_ = (l_w_3).wrapping_add(20); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    { let r_ = F32(0x41f00000); let a_ = (l_w_3).wrapping_add(24); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    { let v_ = F32(c.m.r32((l_w_3).wrapping_add(16))); let a_ = (l_w_3).wrapping_add(52); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_w_3).wrapping_add(24))); let a_ = (l_w_3).wrapping_add(56); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = (l_w_3).wrapping_add(48); c.m.w32(a_, (v_).0); };
    { let v_ = 1i32; let a_ = (l_w_3).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
    { let v_ = 1i32; let a_ = (c.m.r32(sym::gEftFlash)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gEftFlash)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gEftFlash)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftFlash_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftFlash_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftFlash_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gEftFlash)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftFlash_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftFlash_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftFlash_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftFlash_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftFlash_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftFlash_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
        break 'f;
    }
    if ((((((c.m.r32((l_w_2).wrapping_add(60)) as i32)) != (0i32)) as i32)) != 0) {
        break 'f;
    }
    's1: {
        let sv3: i32 = ((c.m.r8((p_task_1).wrapping_add(1))) as i32);
        match sv3 {
            0i32 => {
                { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(16); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let v_ = 1i32; let a_ = (l_w_2).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
                { let v_ = rt::fdiv(F32(c.m.r32((l_w_2).wrapping_add(16))), F32(c.m.r32((l_w_2).wrapping_add(52)))); let a_ = (l_w_2).wrapping_add(48); c.m.w32(a_, (v_).0); };
                if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(16))), F32(0x00000000)) as i32)) != 0) {
                    { let v_ = F32(0x00000000); let a_ = (l_w_2).wrapping_add(16); c.m.w32(a_, (v_).0); };
                    { let v_ = ((1i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                }
                break 's1;
            }
            1i32 => {
                { let a_ = (l_w_2).wrapping_add(68); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                if ((((((c.m.r32((l_w_2).wrapping_add(68)) as i32)) >= (180i32)) as i32)) != 0) {
                    { let v_ = ((2i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                }
                if ((((((c.m.r32((l_w_2).wrapping_add(32)) as i32)) != (0i32)) as i32)) != 0) {
                    if ((((({ let a_ = (c.m.r32(sym::gEftFlash)).wrapping_add(8); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) {
                        { let v_ = ((2i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                        break 's1;
                    }
                } else {
                    if ((((((((({ let a_ = (c.m.r32(sym::gEftFlash)).wrapping_add(8); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gEftFlash)).wrapping_add(8); (c.m.r32(a_) as i32) }) != (2i32)) as i32)) != 0)) as i32)) != 0) {
                        break 's1;
                    }
                    if (((({ let a_ = (c.m.r32(sym::gEftFlash)).wrapping_add(4); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
                        if ((((!((F::BtlCharApi_GetMemberUnk60(c, 0i32)) != 0)) as i32)) != 0) {
                            break 's1;
                        }
                    }
                    if (((({ let a_ = (c.m.r32(sym::gEftFlash)).wrapping_add(4); (c.m.r32(a_) as i32) }) & (4i32))) != 0) {
                        if ((((!((F::BtlCharApi_GetMemberUnk60(c, 1i32)) != 0)) as i32)) != 0) {
                            break 's1;
                        }
                    }
                }
                { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(20); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let v_ = 2i32; let a_ = (l_w_2).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
                if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(20))), F32(0x00000000)) as i32)) != 0) {
                    { let v_ = F32(0x00000000); let a_ = (l_w_2).wrapping_add(20); c.m.w32(a_, (v_).0); };
                    { let v_ = ((2i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                }
                break 's1;
            }
            2i32 => {
                { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(24); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let v_ = 4i32; let a_ = (l_w_2).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
                { let v_ = rt::fsub(F32(0x3f800000), rt::fdiv(F32(c.m.r32((l_w_2).wrapping_add(24))), F32(c.m.r32((l_w_2).wrapping_add(56))))); let a_ = (l_w_2).wrapping_add(48); c.m.w32(a_, (v_).0); };
                if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(24))), F32(0x00000000)) as i32)) != 0) {
                    { let v_ = F32(0x00000000); let a_ = (l_w_2).wrapping_add(24); c.m.w32(a_, (v_).0); };
                    F::BtlTask_SetDead(c, p_task_1);
                }
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftFlash_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftFlash_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftFlash_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_w_2: u32 = 0;
    let mut l_p_3: u32 = 0;
    let mut l_e_4: u32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        rt::memset(c, fp.wrapping_add(16), 0, 16);
        { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(16).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(16).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(16).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(12), (v_).0); }
        { let a3_ = F32(c.m.r32((l_w_2).wrapping_add(48))); F::Vec4_Lerp(c, fp, fp.wrapping_add(16), l_w_2, a3_) };
        l_p_3 = c.m.r32(sym::gOtCur);
        { let v_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(64)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
        { let v_ = ((536870915i32) as u32); let a_ = l_p_3; c.m.w32(a_, v_); };
        { let v_ = ((1342177283i32) as u32); let a_ = (l_p_3).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = ((70i32) as u64); let a_ = (l_p_3).wrapping_add(32); c.m.w64(a_, v_); };
        { let v_ = ((268435456i32) as u32); let a_ = (l_p_3).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = ((4899916394579132417i64) as u64); let a_ = (l_p_3).wrapping_add(16); c.m.w64(a_, v_); };
        { let v_ = ((17424i32) as u64); let a_ = (l_p_3).wrapping_add(24); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (l_p_3).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32(fp)))) as u8); let a_ = (l_p_3).wrapping_add(40); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((fp).wrapping_add(4))))) as u8); let a_ = ((l_p_3).wrapping_add(40)).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((fp).wrapping_add(8))))) as u8); let a_ = ((l_p_3).wrapping_add(40)).wrapping_add(2); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((fp).wrapping_add(12))))) as u8); let a_ = ((l_p_3).wrapping_add(40)).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = ((28672i32) as u64); let a_ = (l_p_3).wrapping_add(48); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((29184i32) as u64); let a_ = ((l_p_3).wrapping_add(48)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((28672i32).wrapping_neg()) as u64); let a_ = (l_p_3).wrapping_add(56); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((29184i32).wrapping_neg()) as u64); let a_ = ((l_p_3).wrapping_add(56)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((16777215i32) as u64); let a_ = ((l_p_3).wrapping_add(48)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((16777215i32) as u64); let a_ = ((l_p_3).wrapping_add(56)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = F32(0x3f800000); let a_ = ((l_p_3).wrapping_add(40)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = ((255i32) as u64); let a_ = ((l_p_3).wrapping_add(48)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((l_p_3).wrapping_add(56)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        l_e_4 = (c.m.r32(sym::gOtLast)).wrapping_add((((1i32) as u32)).wrapping_mul(8));
        { let v_ = l_p_3; let a_ = (c.m.r32((l_e_4).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = l_p_3; let a_ = (l_e_4).wrapping_add(4); c.m.w32(a_, v_); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftFlash_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftFlash_GetRushState(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftFlash_GetRushState", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_ret_2: i32 = 0;
    let mut l_count_3: i32 = 0;
    let mut l_phase_4: i32 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
    'fl1: loop {
        match bb_ {
            0 => {
                l_i_1 = 0i32;
                l_ret_2 = 0i32;
                l_count_3 = F::BtlScene_GetCharCount(c);
                bb_ = 1; continue 'fl1;
            }
            1 => {
                if (((((l_i_1) < (l_count_3)) as i32)) != 0) { bb_ = 2; continue 'fl1; } else { bb_ = 4; continue 'fl1; }
            }
            2 => {
                if ((F::BtlCharApi_IsInRushSequence(c, l_i_1)) != 0) { bb_ = 5; continue 'fl1; } else { bb_ = 7; continue 'fl1; }
            }
            3 => {
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
                bb_ = 1; continue 'fl1;
            }
            4 => {
                if ((((({ let a_ = (c.m.r32(sym::gEftFlash)).wrapping_add(12); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) { bb_ = 32; continue 'fl1; } else { bb_ = 34; continue 'fl1; }
            }
            5 => {
                l_phase_4 = F::BtlCharApi_GetRushFinishPhase(c, l_i_1);
                if (((((l_phase_4) == (1i32)) as i32)) != 0) { bb_ = 8; continue 'fl1; } else { bb_ = 9; continue 'fl1; }
            }
            6 => {
                bb_ = 3; continue 'fl1;
            }
            7 => {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gEftFlash)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                bb_ = 6; continue 'fl1;
            }
            8 => {
                bb_ = 10; continue 'fl1;
            }
            9 => {
                if (((((l_phase_4) == (0i32)) as i32)) != 0) { bb_ = 12; continue 'fl1; } else { bb_ = 13; continue 'fl1; }
            }
            10 => {
                if (((((l_i_1) == (0i32)) as i32)) != 0) { bb_ = 24; continue 'fl1; } else { bb_ = 26; continue 'fl1; }
            }
            11 => {
                bb_ = 9; continue 'fl1;
            }
            12 => {
                bb_ = 3; continue 'fl1;
            }
            13 => {
                if (((((l_phase_4) == (2i32)) as i32)) != 0) { bb_ = 15; continue 'fl1; } else { bb_ = 16; continue 'fl1; }
            }
            14 => {
                bb_ = 13; continue 'fl1;
            }
            15 => {
                bb_ = 17; continue 'fl1;
            }
            16 => {
                if (((((l_phase_4) == ((1i32).wrapping_neg())) as i32)) != 0) { bb_ = 19; continue 'fl1; } else { bb_ = 20; continue 'fl1; }
            }
            17 => {
                if (((((l_i_1) == (0i32)) as i32)) != 0) { bb_ = 28; continue 'fl1; } else { bb_ = 30; continue 'fl1; }
            }
            18 => {
                bb_ = 16; continue 'fl1;
            }
            19 => {
                bb_ = 21; continue 'fl1;
            }
            20 => {
                bb_ = 3; continue 'fl1;
            }
            21 => {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gEftFlash)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                bb_ = 6; continue 'fl1;
            }
            22 => {
                bb_ = 20; continue 'fl1;
            }
            23 => {
                bb_ = 10; continue 'fl1;
            }
            24 => {
                { let r_ = 1i32; let a_ = (c.m.r32(sym::gEftFlash)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                bb_ = 25; continue 'fl1;
            }
            25 => {
                bb_ = 3; continue 'fl1;
            }
            26 => {
                { let r_ = 2i32; let a_ = (c.m.r32(sym::gEftFlash)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                bb_ = 25; continue 'fl1;
            }
            27 => {
                bb_ = 17; continue 'fl1;
            }
            28 => {
                { let r_ = 4i32; let a_ = (c.m.r32(sym::gEftFlash)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                bb_ = 29; continue 'fl1;
            }
            29 => {
                bb_ = 3; continue 'fl1;
            }
            30 => {
                { let r_ = 8i32; let a_ = (c.m.r32(sym::gEftFlash)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                bb_ = 29; continue 'fl1;
            }
            31 => {
                bb_ = 21; continue 'fl1;
            }
            32 => {
                l_ret_2 = 0i32;
                bb_ = 33; continue 'fl1;
            }
            33 => {
                break 'f l_ret_2;
            }
            34 => {
                if ((((((({ let a_ = (c.m.r32(sym::gEftFlash)).wrapping_add(12); (c.m.r32(a_) as i32) }) & (3i32))) == (1i32)) as i32)) != 0) { bb_ = 35; continue 'fl1; } else { bb_ = 37; continue 'fl1; }
            }
            35 => {
                l_ret_2 = 1i32;
                if ((F::BtlCharApi_TestFlag2B(c, 0i32)) != 0) { bb_ = 38; continue 'fl1; } else { bb_ = 39; continue 'fl1; }
            }
            36 => {
                bb_ = 33; continue 'fl1;
            }
            37 => {
                if ((((((({ let a_ = (c.m.r32(sym::gEftFlash)).wrapping_add(12); (c.m.r32(a_) as i32) }) & (3i32))) == (2i32)) as i32)) != 0) { bb_ = 40; continue 'fl1; } else { bb_ = 42; continue 'fl1; }
            }
            38 => {
                l_ret_2 = 2i32;
                bb_ = 39; continue 'fl1;
            }
            39 => {
                bb_ = 36; continue 'fl1;
            }
            40 => {
                l_ret_2 = 1i32;
                if ((F::BtlCharApi_TestFlag2B(c, 1i32)) != 0) { bb_ = 43; continue 'fl1; } else { bb_ = 44; continue 'fl1; }
            }
            41 => {
                bb_ = 36; continue 'fl1;
            }
            42 => {
                if ((((((({ let a_ = (c.m.r32(sym::gEftFlash)).wrapping_add(12); (c.m.r32(a_) as i32) }) & (3i32))) == (3i32)) as i32)) != 0) { bb_ = 45; continue 'fl1; } else { bb_ = 46; continue 'fl1; }
            }
            43 => {
                l_ret_2 = 2i32;
                bb_ = 44; continue 'fl1;
            }
            44 => {
                bb_ = 41; continue 'fl1;
            }
            45 => {
                l_ret_2 = 1i32;
                if ((F::BtlCharApi_TestFlag2B(c, 0i32)) != 0) { bb_ = 47; continue 'fl1; } else { bb_ = 48; continue 'fl1; }
            }
            46 => {
                bb_ = 41; continue 'fl1;
            }
            47 => {
                { let r_ = 4i32; let a_ = (c.m.r32(sym::gEftFlash)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                bb_ = 48; continue 'fl1;
            }
            48 => {
                if ((F::BtlCharApi_TestFlag2B(c, 1i32)) != 0) { bb_ = 49; continue 'fl1; } else { bb_ = 50; continue 'fl1; }
            }
            49 => {
                { let r_ = 8i32; let a_ = (c.m.r32(sym::gEftFlash)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                bb_ = 50; continue 'fl1;
            }
            50 => {
                if ((((((({ let a_ = (c.m.r32(sym::gEftFlash)).wrapping_add(12); (c.m.r32(a_) as i32) }) & (12i32))) == (12i32)) as i32)) != 0) { bb_ = 51; continue 'fl1; } else { bb_ = 52; continue 'fl1; }
            }
            51 => {
                l_ret_2 = 2i32;
                bb_ = 52; continue 'fl1;
            }
            52 => {
                bb_ = 46; continue 'fl1;
            }
            53 => {
                break 'fl1;
            }
            _ => unreachable!(),
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftFlash_GetRushState", &PROBE_N, &[], true); }
    r_
}

pub fn EftTrail_Start(c: &mut Ctx, mut p_pos_1: u32, mut p_dir_2: u32, mut p_life_3: i32, mut p_target_4: u32, mut p_speed_5: F32, mut p_accel_6: F32, mut p_turn_7: F32, mut p_turnAccel_8: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTrail_Start", &PROBE_N, &[(p_pos_1) as u32, (p_dir_2) as u32, (p_life_3) as u32, (p_target_4) as u32, (p_speed_5).0, (p_accel_6).0, (p_turn_7).0, (p_turnAccel_8).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(544) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 544);
    let mut l_i_9: i32 = 0;
    let r_ = 'f: {
        { let _ = F::memset(c, fp, 0i32, 544u32); };
        { let v_ = 20i32; let a_ = (fp).wrapping_add(432); c.m.w32(a_, (v_) as u32); };
        if ((F::Battle_IsSplitScreen(c)) != 0) {
            { let v_ = 15i32; let a_ = (fp).wrapping_add(432); c.m.w32(a_, (v_) as u32); };
        }
        F::Vec4_Copy(c, (fp).wrapping_add(64), p_pos_1);
        {
            l_i_9 = 0i32;
            'l1: loop {
                if !(((((l_i_9) < ((c.m.r32((fp).wrapping_add(432)) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::Vec4_Copy(c, ((fp).wrapping_add(112)).wrapping_add((((l_i_9) as u32)).wrapping_mul(16)), p_pos_1);
                }
                { let o_ = l_i_9; let n_ = (o_).wrapping_add(1); l_i_9 = n_; };
            }
        }
        F::Vec4_Copy(c, (fp).wrapping_add(80), p_dir_2);
        F::Vec3_Normalize(c, (fp).wrapping_add(80), (fp).wrapping_add(80));
        { let v_ = p_speed_5; let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = p_accel_6; let a_ = (fp).wrapping_add(16); c.m.w32(a_, (v_).0); };
        { let v_ = p_turn_7; let a_ = (fp).wrapping_add(20); c.m.w32(a_, (v_).0); };
        { let v_ = p_turnAccel_8; let a_ = (fp).wrapping_add(24); c.m.w32(a_, (v_).0); };
        { let v_ = p_life_3; let a_ = (fp).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        { let v_ = p_target_4; let a_ = (fp).wrapping_add(48); c.m.w32(a_, v_); };
        { let v_ = 4i32; let a_ = (fp).wrapping_add(440); c.m.w32(a_, (v_) as u32); };
        { let v_ = (c.m.r32(sym::gEftTrail)).wrapping_add(8); let a_ = fp; c.m.w32(a_, v_); };
        { let v_ = 0i32; let a_ = (fp).wrapping_add(436); c.m.w32(a_, (v_) as u32); };
        { let _ = { let a0_ = { let a_ = c.m.r32(sym::gEftTrail); c.m.r32(a_) }; F::BtlTaskList_AddTail(c, a0_, sym::gEftTrailClass, fp) }; };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftTrail_Start", &PROBE_N, &[], true); }
    r_
}

pub fn EftTrail_StartOwned(c: &mut Ctx, mut p_kind_1: u8, mut p_objId_2: i32, mut p_pos_3: u32, mut p_color_4: u32, mut p_tbl_5: u32, mut p_width_6: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTrail_StartOwned", &PROBE_N, &[(p_kind_1) as u32, (p_objId_2) as u32, (p_pos_3) as u32, (p_color_4) as u32, (p_tbl_5) as u32, (p_width_6).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(560) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 560);
    let mut l_i_7: i32 = 0;
    let r_ = 'f: {
        { let _ = F::memset(c, fp.wrapping_add(544), 0i32, 16u32); };
        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(544)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let _ = F::memset(c, fp, 0i32, 544u32); };
        { let v_ = p_objId_2; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = p_kind_1; let a_ = (fp).wrapping_add(8); c.m.w8(a_, v_); };
        { let v_ = 20i32; let a_ = (fp).wrapping_add(432); c.m.w32(a_, (v_) as u32); };
        if ((F::Battle_IsSplitScreen(c)) != 0) {
            { let v_ = 8i32; let a_ = (fp).wrapping_add(432); c.m.w32(a_, (v_) as u32); };
        }
        F::Vec4_Copy(c, (fp).wrapping_add(64), p_pos_3);
        {
            l_i_7 = 0i32;
            'l1: loop {
                if !(((((l_i_7) < ((c.m.r32((fp).wrapping_add(432)) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::Vec4_Copy(c, ((fp).wrapping_add(112)).wrapping_add((((l_i_7) as u32)).wrapping_mul(16)), p_pos_3);
                }
                { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
            }
        }
        F::Vec4_Copy(c, (fp).wrapping_add(80), fp.wrapping_add(544));
        F::Vec3_Normalize(c, (fp).wrapping_add(80), (fp).wrapping_add(80));
        F::Vec4_Copy(c, (fp).wrapping_add(96), p_color_4);
        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(16); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3e4ccccc); let a_ = (fp).wrapping_add(20); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3c23d70a); let a_ = (fp).wrapping_add(24); c.m.w32(a_, (v_).0); };
        { let v_ = p_width_6; let a_ = (fp).wrapping_add(28); c.m.w32(a_, (v_).0); };
        { let v_ = p_width_6; let a_ = (fp).wrapping_add(32); c.m.w32(a_, (v_).0); };
        { let v_ = (1i32).wrapping_neg(); let a_ = (fp).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        { let v_ = p_tbl_5; let a_ = fp; c.m.w32(a_, v_); };
        { let v_ = 0i32; let a_ = (fp).wrapping_add(436); c.m.w32(a_, (v_) as u32); };
        { let v_ = 4i32; let a_ = (fp).wrapping_add(440); c.m.w32(a_, (v_) as u32); };
        { let v_ = sym::gEftTrailOrigin; let a_ = (fp).wrapping_add(48); c.m.w32(a_, v_); };
        F::EftTrail_SetTextures(c, fp, p_tbl_5);
        { let _ = { let a0_ = { let a_ = c.m.r32(sym::gEftTrail); c.m.r32(a_) }; F::BtlTaskList_AddTail(c, a0_, sym::gEftTrailClass, fp) }; };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftTrail_StartOwned", &PROBE_N, &[], true); }
    r_
}

pub fn EftTrail_End(c: &mut Ctx, mut p_task_1: u32, mut p_frames_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTrail_End", &PROBE_N, &[(p_task_1) as u32, (p_frames_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftTrail_Update)) as i32)) != 0) {
        break 'f;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let v_ = p_frames_2; let a_ = (l_w_3).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
    { let r_ = 1i32; let a_ = (l_w_3).wrapping_add(40); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTrail_End", &PROBE_N, &[], true); }
    r_
}

pub fn EftTrail_SetColor(c: &mut Ctx, mut p_task_1: u32, mut p_color_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTrail_SetColor", &PROBE_N, &[(p_task_1) as u32, (p_color_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftTrail_Update)) as i32)) != 0) {
        break 'f;
    }
    { let a0_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(96); F::Vec4_Copy(c, a0_, p_color_2) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTrail_SetColor", &PROBE_N, &[], true); }
    r_
}

pub fn EftTrail_GetPos(c: &mut Ctx, mut p_task_1: u32, mut p_out_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTrail_GetPos", &PROBE_N, &[(p_task_1) as u32, (p_out_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftTrail_Update)) as i32)) != 0) {
        break 'f;
    }
    { let a1_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(64); F::Vec4_Copy(c, p_out_2, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTrail_GetPos", &PROBE_N, &[], true); }
    r_
}

pub fn EftTrail_SetPos(c: &mut Ctx, mut p_task_1: u32, mut p_pos_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTrail_SetPos", &PROBE_N, &[(p_task_1) as u32, (p_pos_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftTrail_Update)) as i32)) != 0) {
        break 'f;
    }
    { let a0_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(64); F::Vec4_Copy(c, a0_, p_pos_2) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTrail_SetPos", &PROBE_N, &[], true); }
    r_
}

pub fn EftTrail_SetWidth(c: &mut Ctx, mut p_task_1: u32, mut p_width_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTrail_SetWidth", &PROBE_N, &[(p_task_1) as u32, (p_width_2).0], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftTrail_Update)) as i32)) != 0) {
        break 'f;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let v_ = p_width_2; let a_ = (l_w_3).wrapping_add(28); c.m.w32(a_, (v_).0); };
    { let v_ = p_width_2; let a_ = (l_w_3).wrapping_add(32); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTrail_SetWidth", &PROBE_N, &[], true); }
    r_
}

pub fn EftTrailMgr_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTrailMgr_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((272u32) as i32)) }; let a_ = sym::gEftTrail; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gEftTrail); F::memset(c, a0_, 0i32, 272u32) }; };
    { let v_ = F::BtlTask_CreateChildList(c, p_task_1, ((((15i32) as i64 as u64)) as u32), ((((((544u32) as i32)) as i64 as u64)) as u32)); let a_ = c.m.r32(sym::gEftTrail); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTrailMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftTrailMgr_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTrailMgr_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftTrail); F::BtlPool_Free(c, a0_, a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftTrail; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTrailMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftTrailMgr_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTrailMgr_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gEftTrail)).wrapping_add(8)).wrapping_add(260); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTrailMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftTrail_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTrail_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let s_ = p_arg_2; let d_ = l_w_3; rt::memcpy(c, d_, s_, 544); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTrail_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftTrail_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTrail_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTrail_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftTrail_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTrail_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTrail_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftTrail_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTrail_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_pos_4: u32 = 0;
    let mut l_dir_5: u32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        if ((((!(({ let a0_ = (c.m.r32((l_w_2).wrapping_add(4)) as i32); let a1_ = ((c.m.r8((l_w_2).wrapping_add(8))) as i32); F::BtlScene_IsEffectStopped(c, a0_, a1_) }) != 0)) as i32)) != 0) {
            if ((((!(((((c.m.r32((l_w_2).wrapping_add(40)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
                {
                    l_i_3 = ((c.m.r32((l_w_2).wrapping_add(432)) as i32)).wrapping_sub(1i32);
                    'l1: loop {
                        if !(((((l_i_3) > (0i32)) as i32)) != 0) { break 'l1; }
                        'c2: {
                            F::Vec4_Copy(c, ((l_w_2).wrapping_add(112)).wrapping_add((((l_i_3) as u32)).wrapping_mul(16)), ((l_w_2).wrapping_add(112)).wrapping_add(((((l_i_3).wrapping_sub(1i32)) as u32)).wrapping_mul(16)));
                        }
                        { let o_ = l_i_3; let n_ = (o_).wrapping_sub(1); l_i_3 = n_; };
                    }
                }
                l_pos_4 = (l_w_2).wrapping_add(64);
                { let a1_ = c.m.r32((l_w_2).wrapping_add(48)); F::Vec3_Sub(c, fp, a1_, l_pos_4) };
                l_dir_5 = (l_w_2).wrapping_add(80);
                F::Vec3_Normalize(c, fp, fp);
                { let a2_ = F32(c.m.r32((l_w_2).wrapping_add(20))); F::Vec3_Scale(c, fp, fp, a2_) };
                F::Vec3_Add(c, l_dir_5, l_dir_5, fp);
                F::Vec3_Normalize(c, l_dir_5, l_dir_5);
                { let a2_ = F32(c.m.r32((l_w_2).wrapping_add(12))); F::Vec4_Scale(c, fp, l_dir_5, a2_) };
                F::Vec4_Copy(c, ((l_w_2).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), l_pos_4);
            }
            { let r_ = F32(c.m.r32((l_w_2).wrapping_add(16))); let a_ = (l_w_2).wrapping_add(12); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::fgt(F32(c.m.r32((l_w_2).wrapping_add(12))), F32(0x42c80000)) as i32)) != 0) {
                { let v_ = F32(0x42c80000); let a_ = (l_w_2).wrapping_add(12); c.m.w32(a_, (v_).0); };
            }
            { let r_ = F32(c.m.r32((l_w_2).wrapping_add(24))); let a_ = (l_w_2).wrapping_add(20); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((((c.m.r32((l_w_2).wrapping_add(40)) as i32)) & (1i32))) != 0) {
                { let r_ = rt::fdiv(F32(c.m.r32((l_w_2).wrapping_add(32))), rt::i2f((c.m.r32((l_w_2).wrapping_add(44)) as i32))); let a_ = (l_w_2).wrapping_add(28); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(28))), F32(0x3dcccccc)) as i32)) != 0) {
                    F::BtlTask_SetDead(c, p_task_1);
                }
            }
            if ((((((c.m.r32((l_w_2).wrapping_add(36)) as i32)) >= (0i32)) as i32)) != 0) {
                if ((((({ let a_ = (l_w_2).wrapping_add(36); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) <= (0i32)) as i32)) != 0) {
                    F::BtlTask_SetDead(c, p_task_1);
                }
            }
        }
        F::EftTrail_UpdateTextures(c, l_w_2);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftTrail_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftTrail_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTrail_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(784) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 784);
    let mut l_i_2: i32 = 0;
    let mut l_first_3: i32 = 0;
    let mut l_limit_4: F32 = F32(0);
    let mut l_ibias_5: i32 = 0;
    let mut l_w_6: u32 = 0;
    let mut l_width_7: F32 = F32(0);
    let mut l_p_8: u32 = 0;
    let mut l_j_9: i32 = 0;
    let mut l_seg_10: i32 = 0;
    let mut l_z_11: i32 = 0;
    let mut l_bias_12: F32 = F32(0);
    let mut l_sz_13: F32 = F32(0);
    let r_ = 'f: {
        l_first_3 = 1i32;
        l_limit_4 = F32(0x3f51eb85);
        l_ibias_5 = (7i32).wrapping_neg();
        l_w_6 = c.m.r32((p_task_1).wrapping_add(56));
        l_width_7 = F32(c.m.r32((l_w_6).wrapping_add(28)));
        rt::memset(c, fp.wrapping_add(720), 0, 64);
        rt::memset(c, fp.wrapping_add(720).wrapping_add(0), 0, 16);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(720).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(720).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(720).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(720).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(720).wrapping_add(16), 0, 16);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(720).wrapping_add(16).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(720).wrapping_add(16).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(720).wrapping_add(16).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(720).wrapping_add(16).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(720).wrapping_add(32), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(720).wrapping_add(32).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(720).wrapping_add(32).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(720).wrapping_add(32).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(720).wrapping_add(32).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(720).wrapping_add(48), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(720).wrapping_add(48).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(720).wrapping_add(48).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(720).wrapping_add(48).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(720).wrapping_add(48).wrapping_add(12), (v_).0); }
        if (({ let a0_ = (c.m.r32((l_w_6).wrapping_add(4)) as i32); let a1_ = ((c.m.r8((l_w_6).wrapping_add(8))) as i32); F::BtlScene_IsEffectHidden(c, a0_, a1_) }) != 0) {
            break 'f;
        }
        F::Vu0Cur_Push(c);
        l_p_8 = ((l_w_6).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(16));
        l_i_2 = 0i32;
        { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(544); F::Vec4_Copy(c, fp.wrapping_add(496), a1_) };
        {
            'l1: loop {
                if !(((((l_i_2) < (((c.m.r32((l_w_6).wrapping_add(432)) as i32)).wrapping_sub(1i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_seg_10 = 1i32;
                    if (((((l_i_2) == (0i32)) as i32)) != 0) {
                        l_seg_10 = 2i32;
                    }
                    if (((((l_i_2) == (((c.m.r32((l_w_6).wrapping_add(432)) as i32)).wrapping_sub(2i32))) as i32)) != 0) {
                        l_seg_10 = 0i32;
                    }
                    F::Vec4_Copy(c, fp.wrapping_add(480), (l_w_6).wrapping_add(96));
                    F::Vec4_Sub(c, fp.wrapping_add(432), (l_p_8).wrapping_add((((1i32) as u32)).wrapping_mul(16)), l_p_8);
                    F::Vec4_Sub(c, fp.wrapping_add(512), l_p_8, fp.wrapping_add(496));
                    F::Vec3_Cross(c, fp.wrapping_add(432), fp.wrapping_add(432), fp.wrapping_add(512));
                    F::Vec3_Normalize(c, fp.wrapping_add(432), fp.wrapping_add(432));
                    if ((l_first_3) != 0) {
                        F::Vec3_ScaleAdd(c, (fp.wrapping_add(528)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), fp.wrapping_add(432), l_p_8, l_width_7);
                        F::Vec3_ScaleAdd(c, (fp.wrapping_add(528)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), fp.wrapping_add(432), l_p_8, rt::fneg(l_width_7));
                        l_first_3 = 0i32;
                    } else {
                        if (((rt::flt(F::Vec3_Dot(c, fp.wrapping_add(432), fp.wrapping_add(448)), F32(0x00000000)) as i32)) != 0) {
                            F::Vec3_Scale(c, fp.wrapping_add(432), fp.wrapping_add(432), F32(0xbf800000));
                        }
                        if (((rt::flt({ let a0_ = F::Vec3_Dot(c, fp.wrapping_add(432), fp.wrapping_add(448)); rt::ffabs(a0_) }, l_limit_4) as i32)) != 0) {
                            { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(480)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                            if (((((c.m.r32((l_w_6).wrapping_add(40)) as i32)) & (1i32))) != 0) {
                                { let v_ = F32(c.m.r32(((l_w_6).wrapping_add(96)).wrapping_add(12))); let a_ = (fp.wrapping_add(480)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                            }
                            if (((rt::fgt(F::Vec3_Dist(c, l_p_8, (l_p_8).wrapping_add((((1i32) as u32)).wrapping_mul(16))), F32(0x3f4ccccc)) as i32)) != 0) {
                                l_bias_12 = rt::i2f(l_ibias_5);
                                { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(l_p_8)), F32(c.m.r32((l_p_8).wrapping_add((((1i32) as u32)).wrapping_mul(16))))), F32(0x3f000000)); let a_ = fp.wrapping_add(464); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fmul(rt::fadd(F32(c.m.r32((l_p_8).wrapping_add(4))), F32(c.m.r32(((l_p_8).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4)))), F32(0x3f000000)); let a_ = (fp.wrapping_add(464)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fmul(rt::fadd(F32(c.m.r32((l_p_8).wrapping_add(8))), F32(c.m.r32(((l_p_8).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)))), F32(0x3f000000)); let a_ = (fp.wrapping_add(464)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                l_sz_13 = F32(0x411759d3);
                                { let a0_ = ((rt::f2i(rt::fadd(F32(c.m.r32((l_w_6).wrapping_add(96))), l_bias_12))) as u8); let a1_ = ((rt::f2i(rt::fadd(F32(c.m.r32(((l_w_6).wrapping_add(96)).wrapping_add(4))), l_bias_12))) as u8); let a2_ = ((rt::f2i(rt::fadd(F32(c.m.r32(((l_w_6).wrapping_add(96)).wrapping_add(8))), l_bias_12))) as u8); let a3_ = ((rt::f2i(rt::fmul(F32(c.m.r32(((l_w_6).wrapping_add(96)).wrapping_add(12))), F32(0x3f4ccccc)))) as u8); let a4_ = F32(c.m.r32(l_p_8)); let a5_ = F32(c.m.r32((l_p_8).wrapping_add(4))); let a6_ = F32(c.m.r32((l_p_8).wrapping_add(8))); let a13_ = F32(c.m.r32((l_w_6).wrapping_add(28))); let a16_ = c.m.r64(((l_w_6).wrapping_add(512)).wrapping_add((((3i32) as u32)).wrapping_mul(8))); F::EftTrail_DrawSprite(c, a0_, a1_, a2_, a3_, a4_, a5_, a6_, l_sz_13, l_sz_13, F32(0x00000000), F32(0x00000000), F32(0x3f800000), F32(0x3f800000), a13_, F32(0x00000000), 1i32, a16_) };
                                l_sz_13 = F32(0x411759d3);
                                { let a0_ = ((rt::f2i(rt::fadd(F32(c.m.r32((l_w_6).wrapping_add(96))), l_bias_12))) as u8); let a1_ = ((rt::f2i(rt::fadd(F32(c.m.r32(((l_w_6).wrapping_add(96)).wrapping_add(4))), l_bias_12))) as u8); let a2_ = ((rt::f2i(rt::fadd(F32(c.m.r32(((l_w_6).wrapping_add(96)).wrapping_add(8))), l_bias_12))) as u8); let a3_ = ((rt::f2i(rt::fmul(F32(c.m.r32(((l_w_6).wrapping_add(96)).wrapping_add(12))), F32(0x3f4ccccc)))) as u8); let a4_ = F32(c.m.r32(fp.wrapping_add(464))); let a5_ = F32(c.m.r32((fp.wrapping_add(464)).wrapping_add(4))); let a6_ = F32(c.m.r32((fp.wrapping_add(464)).wrapping_add(8))); let a13_ = F32(c.m.r32((l_w_6).wrapping_add(28))); let a16_ = c.m.r64(((l_w_6).wrapping_add(512)).wrapping_add((((3i32) as u32)).wrapping_mul(8))); F::EftTrail_DrawSprite(c, a0_, a1_, a2_, a3_, a4_, a5_, a6_, l_sz_13, l_sz_13, F32(0x00000000), F32(0x00000000), F32(0x3f800000), F32(0x3f800000), a13_, F32(0x00000000), 1i32, a16_) };
                                l_sz_13 = F32(0x411759d3);
                                { let a0_ = ((rt::f2i(rt::fadd(F32(c.m.r32((l_w_6).wrapping_add(96))), l_bias_12))) as u8); let a1_ = ((rt::f2i(rt::fadd(F32(c.m.r32(((l_w_6).wrapping_add(96)).wrapping_add(4))), l_bias_12))) as u8); let a2_ = ((rt::f2i(rt::fadd(F32(c.m.r32(((l_w_6).wrapping_add(96)).wrapping_add(8))), l_bias_12))) as u8); let a3_ = ((rt::f2i(rt::fmul(F32(c.m.r32(((l_w_6).wrapping_add(96)).wrapping_add(12))), F32(0x3f4ccccc)))) as u8); let a4_ = F32(c.m.r32((l_p_8).wrapping_add((((1i32) as u32)).wrapping_mul(16)))); let a5_ = F32(c.m.r32(((l_p_8).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a6_ = F32(c.m.r32(((l_p_8).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a13_ = F32(c.m.r32((l_w_6).wrapping_add(28))); let a16_ = c.m.r64(((l_w_6).wrapping_add(512)).wrapping_add((((3i32) as u32)).wrapping_mul(8))); F::EftTrail_DrawSprite(c, a0_, a1_, a2_, a3_, a4_, a5_, a6_, l_sz_13, l_sz_13, F32(0x00000000), F32(0x00000000), F32(0x3f800000), F32(0x3f800000), a13_, F32(0x00000000), 1i32, a16_) };
                            }
                        }
                        F::Vec4_Copy(c, (fp.wrapping_add(528)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(592)).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
                        F::Vec4_Copy(c, (fp.wrapping_add(528)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(592)).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
                    }
                    F::Vec3_ScaleAdd(c, (fp.wrapping_add(528)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), fp.wrapping_add(432), (l_p_8).wrapping_add((((1i32) as u32)).wrapping_mul(16)), l_width_7);
                    F::Vec3_ScaleAdd(c, (fp.wrapping_add(528)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), fp.wrapping_add(432), (l_p_8).wrapping_add((((1i32) as u32)).wrapping_mul(16)), rt::fneg(l_width_7));
                    F::Vec4_Copy(c, fp.wrapping_add(448), fp.wrapping_add(432));
                    F::Vec4_Copy(c, (fp.wrapping_add(592)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(528)).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
                    F::Vec4_Copy(c, (fp.wrapping_add(592)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(528)).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
                    { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(528)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(528)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(528)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(528)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let _ = F::Vu0Cur_ProjectPoints(c, fp.wrapping_add(656), fp.wrapping_add(528), 4i32); };
                    l_z_11 = (((((c.m.r32(((fp.wrapping_add(656)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)).wrapping_add((c.m.r32(((fp.wrapping_add(656)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add((c.m.r32(((fp.wrapping_add(656)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add((c.m.r32(((fp.wrapping_add(656)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_shr((10i32) as u32);
                    {
                        l_j_9 = 0i32;
                        'l3: loop {
                            if !(((((l_j_9) < (2i32)) as i32)) != 0) { break 'l3; }
                            'c4: {
                                F::ClipVtx_Set(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(528)).wrapping_add((((l_j_9) as u32)).wrapping_mul(16)), (fp.wrapping_add(720)).wrapping_add((((l_j_9) as u32)).wrapping_mul(16)), fp.wrapping_add(480));
                                F::ClipVtx_Set(c, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(528)).wrapping_add(((((l_j_9).wrapping_add(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(720)).wrapping_add(((((l_j_9).wrapping_add(1i32)) as u32)).wrapping_mul(16)), fp.wrapping_add(480));
                                F::ClipVtx_Set(c, (fp).wrapping_add((((2i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(528)).wrapping_add(((((l_j_9).wrapping_add(2i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(720)).wrapping_add(((((l_j_9).wrapping_add(2i32)) as u32)).wrapping_mul(16)), fp.wrapping_add(480));
                                { let a5_ = c.m.r64(((l_w_6).wrapping_add(512)).wrapping_add((((l_seg_10) as u32)).wrapping_mul(8))); F::EftGfx_DrawPolyAvgZ(c, fp, 1i32, 0i32, 0i32, 0i32, a5_, l_z_11) };
                            }
                            { let o_ = l_j_9; let n_ = (o_).wrapping_add(1); l_j_9 = n_; };
                        }
                    }
                    { let o_ = l_p_8; let n_ = (o_).wrapping_add(16); l_p_8 = n_; };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        F::Vu0Cur_Pop(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftTrail_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftTrail_UpdateTextures(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTrail_UpdateTextures", &PROBE_N, &[(p_w_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    if ((((!(((({ let a_ = (c.m.r32(p_w_1)).wrapping_add(260); c.m.r32(a_) }) & ((((1i32).wrapping_shl(((c.m.r32((p_w_1).wrapping_add(436)) as i32)) as u32)) as u32)))) != 0)) as i32)) != 0) {
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < ((c.m.r32((p_w_1).wrapping_add(440)) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = F::EftVram_AddTex(c, ((p_w_1).wrapping_add(448)).wrapping_add((((l_i_2) as u32)).wrapping_mul(16)), 1i32, 0i32); let a_ = ((p_w_1).wrapping_add(512)).wrapping_add((((l_i_2) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = c.m.r64(((p_w_1).wrapping_add(512)).wrapping_add((((l_i_2) as u32)).wrapping_mul(8))); let a_ = (c.m.r32(p_w_1)).wrapping_add((((((c.m.r32((p_w_1).wrapping_add(436)) as i32)).wrapping_add(l_i_2)) as u32)).wrapping_mul(16)); c.m.w64(a_, v_); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let r_ = (((1i32).wrapping_shl(((c.m.r32((p_w_1).wrapping_add(436)) as i32)) as u32)) as u32); let a_ = (c.m.r32(p_w_1)).wrapping_add(260); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    } else {
        {
            l_i_2 = 0i32;
            'l3: loop {
                if !(((((l_i_2) < ((c.m.r32((p_w_1).wrapping_add(440)) as i32))) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let v_ = { let a_ = (c.m.r32(p_w_1)).wrapping_add((((((c.m.r32((p_w_1).wrapping_add(436)) as i32)).wrapping_add(l_i_2)) as u32)).wrapping_mul(16)); c.m.r64(a_) }; let a_ = ((p_w_1).wrapping_add(512)).wrapping_add((((l_i_2) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTrail_UpdateTextures", &PROBE_N, &[], true); }
    r_
}

pub fn EftTrail_SetTextures(c: &mut Ctx, mut p_w_1: u32, mut p_src_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTrail_SetTextures", &PROBE_N, &[(p_w_1) as u32, (p_src_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let s_ = (p_src_2).wrapping_add((((l_i_3) as u32)).wrapping_mul(16)); let d_ = ((p_w_1).wrapping_add(448)).wrapping_add((((l_i_3) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTrail_SetTextures", &PROBE_N, &[], true); }
    r_
}

pub fn EftTrail_DrawSprite(c: &mut Ctx, mut p_r_1: u8, mut p_g_2: u8, mut p_b_3: u8, mut p_a_4: u8, mut p_x_5: F32, mut p_y_6: F32, mut p_z_7: F32, mut p_w_8: F32, mut p_h_9: F32, mut p_u0_10: F32, mut p_v0_11: F32, mut p_u1_12: F32, mut p_v1_13: F32, mut p_scale_14: F32, mut p_rot_15: F32, mut p_layer_16: i32, mut p_tex0_17: u64) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTrail_DrawSprite", &PROBE_N, &[(p_r_1) as u32, (p_g_2) as u32, (p_b_3) as u32, (p_a_4) as u32, (p_x_5).0, (p_y_6).0, (p_z_7).0, (p_w_8).0, (p_h_9).0, (p_u0_10).0, (p_v0_11).0, (p_u1_12).0, (p_v1_13).0, (p_scale_14).0, (p_rot_15).0, (p_layer_16) as u32, (p_tex0_17) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(224) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 224);
    let mut l_p_18: u32 = 0;
    let mut l_e_19: u32 = 0;
    let mut l_i_20: i32 = 0;
    let mut l_l_21: i32 = 0;
    let mut l_oz_22: i32 = 0;
    let mut l_hh_23: F32 = F32(0);
    let mut l_ww_24: F32 = F32(0);
    let r_ = 'f: {
        if (((rt::fgt(p_rot_15, F32(0x40490fda)) as i32)) != 0) {
            { let r_ = F32(0x40c90fda); let n_ = rt::fsub(p_rot_15, r_); p_rot_15 = n_; };
        } else {
            if (((rt::flt(p_rot_15, F32(0xc0490fda)) as i32)) != 0) {
                { let r_ = F32(0x40c90fda); let n_ = rt::fadd(p_rot_15, r_); p_rot_15 = n_; };
            }
        }
        l_ww_24 = rt::fmul(p_w_8, p_scale_14);
        l_hh_23 = rt::fmul(p_h_9, p_scale_14);
        { let r_ = F32(0x3d800000); let n_ = rt::fmul(l_ww_24, r_); l_ww_24 = n_; };
        { let r_ = F32(0x3d800000); let n_ = rt::fmul(l_hh_23, r_); l_hh_23 = n_; };
        F::Vec4_Set(c, fp.wrapping_add(128), p_x_5, p_y_6, p_z_7, F32(0x3f800000));
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(544); F::Vec4_Sub(c, fp.wrapping_add(144), a1_, fp.wrapping_add(128)) };
        F::Vec3_Normalize(c, fp.wrapping_add(144), fp.wrapping_add(144));
        { let _ = F::EftMath_MtxFromDir(c, fp.wrapping_add(160), fp.wrapping_add(144), p_rot_15); };
        F::Vec4_Set(c, (fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), rt::fneg(l_ww_24), rt::fneg(l_hh_23), F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), l_ww_24, rt::fneg(l_hh_23), F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), rt::fneg(l_ww_24), l_hh_23, F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), l_ww_24, l_hh_23, F32(0x00000000), F32(0x3f800000));
        {
            l_i_20 = 0i32;
            'l1: loop {
                if !(((((l_i_20) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::Mtx_MulVec4(c, (fp.wrapping_add(64)).wrapping_add((((l_i_20) as u32)).wrapping_mul(16)), fp.wrapping_add(160), (fp.wrapping_add(64)).wrapping_add((((l_i_20) as u32)).wrapping_mul(16)));
                    F::Vec3_Add(c, (fp.wrapping_add(64)).wrapping_add((((l_i_20) as u32)).wrapping_mul(16)), (fp.wrapping_add(64)).wrapping_add((((l_i_20) as u32)).wrapping_mul(16)), fp.wrapping_add(128));
                    { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(64)).wrapping_add((((l_i_20) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_20; let n_ = (o_).wrapping_add(1); l_i_20 = n_; };
            }
        }
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); { let _ = F::Mtx_ProjectPoints(c, fp, a1_, fp.wrapping_add(64), 4i32); } };
        if ((F::IVec4_InGsRange4(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((3i32) as u32)).wrapping_mul(16)))) != 0) {
            break 'f;
        }
        l_p_18 = c.m.r32(sym::gOtCur);
        { let v_ = (l_p_18).wrapping_add((((1i32) as u32)).wrapping_mul(128)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
        { let v_ = ((84i32) as u64); let a_ = (l_p_18).wrapping_add(32); c.m.w64(a_, v_); };
        { let v_ = ((536870919i32) as u32); let a_ = l_p_18; c.m.w32(a_, v_); };
        { let v_ = ((268435456i32) as u32); let a_ = (l_p_18).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = ((1342177287i32) as u32); let a_ = (l_p_18).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = 14123288431433908225u64; let a_ = (l_p_18).wrapping_add(16); c.m.w64(a_, v_); };
        { let v_ = ((268436062347616i64) as u64); let a_ = (l_p_18).wrapping_add(24); c.m.w64(a_, v_); };
        l_l_21 = p_layer_16;
        if (((((l_l_21) >= (2i32)) as i32)) != 0) {
            { let r_ = 2i32; let n_ = (l_l_21).wrapping_sub(r_); l_l_21 = n_; };
        }
        { let v_ = p_r_1; let a_ = (l_p_18).wrapping_add(48); c.m.w8(a_, v_); };
        { let v_ = p_g_2; let a_ = ((l_p_18).wrapping_add(48)).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = p_b_3; let a_ = ((l_p_18).wrapping_add(48)).wrapping_add(2); c.m.w8(a_, v_); };
        { let v_ = p_a_4; let a_ = ((l_p_18).wrapping_add(48)).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = F32(0x3f800000); let a_ = ((l_p_18).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_tex0_17; let a_ = (l_p_18).wrapping_add(40); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (l_p_18).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = p_u0_10; let a_ = (l_p_18).wrapping_add(56); c.m.w32(a_, (v_).0); };
        { let v_ = p_v0_11; let a_ = ((l_p_18).wrapping_add(56)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_u1_12; let a_ = (l_p_18).wrapping_add(72); c.m.w32(a_, (v_).0); };
        { let v_ = p_v0_11; let a_ = ((l_p_18).wrapping_add(72)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_u0_10; let a_ = (l_p_18).wrapping_add(88); c.m.w32(a_, (v_).0); };
        { let v_ = p_v1_13; let a_ = ((l_p_18).wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_u1_12; let a_ = (l_p_18).wrapping_add(104); c.m.w32(a_, (v_).0); };
        { let v_ = p_v1_13; let a_ = ((l_p_18).wrapping_add(104)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = (((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (l_p_18).wrapping_add(64); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((l_p_18).wrapping_add(64)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((l_p_18).wrapping_add(64)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((l_p_18).wrapping_add(64)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (l_p_18).wrapping_add(80); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((l_p_18).wrapping_add(80)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((l_p_18).wrapping_add(80)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((l_p_18).wrapping_add(80)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (l_p_18).wrapping_add(96); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((l_p_18).wrapping_add(96)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((l_p_18).wrapping_add(96)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((l_p_18).wrapping_add(96)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (l_p_18).wrapping_add(112); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((l_p_18).wrapping_add(112)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((l_p_18).wrapping_add(112)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((l_p_18).wrapping_add(112)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        l_oz_22 = ((c.m.r32(((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)).wrapping_shr((8i32) as u32);
        if (((((l_oz_22) < (0i32)) as i32)) != 0) {
            l_e_19 = ((c.m.r32(sym::gOtZ)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_21) as u32)).wrapping_mul(8));
        } else {
            if (((((l_oz_22) >= (4096i32)) as i32)) != 0) {
                l_e_19 = ((c.m.r32(sym::gOtZ)).wrapping_add((((4095i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_21) as u32)).wrapping_mul(8));
            } else {
                l_e_19 = ((c.m.r32(sym::gOtZ)).wrapping_add((((l_oz_22) as u32)).wrapping_mul(16))).wrapping_add((((l_l_21) as u32)).wrapping_mul(8));
            }
        }
        { let v_ = l_p_18; let a_ = (c.m.r32((l_e_19).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = l_p_18; let a_ = (l_e_19).wrapping_add(4); c.m.w32(a_, v_); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftTrail_DrawSprite", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharaFx_Emit(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32, mut p_set_3: u32, mut p_state_4: u32, mut p_pos_5: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharaFx_Emit", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32, (p_set_3) as u32, (p_state_4) as u32, (p_pos_5) as u32], false); }
    let mut l_w_6: u32 = 0;
    let mut l_group_7: i32 = 0;
    let mut l_sub_8: i32 = 0;
    let mut l_def_9: u32 = 0;
    let mut l_spawn_10: i32 = 0;
    let r_ = 'f: {
    l_w_6 = c.m.r32((p_task_2).wrapping_add(56));
    {
        l_group_7 = 0i32;
        'l1: loop {
            if !(((((l_group_7) < (19i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((({ let a_ = c.m.r32(p_set_3); c.m.r32(a_) }) & ((((1i32).wrapping_shl((l_group_7) as u32)) as u32)))) != 0) {
                    l_def_9 = c.m.r32(((p_set_3).wrapping_add(12)).wrapping_add((((l_group_7) as u32)).wrapping_mul(16)));
                    {
                        l_sub_8 = 0i32;
                        'l3: loop {
                            if !(((((l_sub_8) < (((c.m.r8((l_def_9).wrapping_add(1))) as i32))) as i32)) != 0) { break 'l3; }
                            'c4: {
                                l_spawn_10 = { let a5_ = ((((c.m.r32((l_w_6).wrapping_add(2096))) & (((2i32) as u32)))) as i32); let a7_ = ((c.m.r32((l_w_6).wrapping_add(2104))) as i32); F::EftEmit_GetFlagsFromMask(c, p_set_3, p_state_4, p_objId_1, l_group_7, l_sub_8, a5_, 0i32, a7_) };
                                if (((((l_spawn_10) != (0i32)) as i32)) != 0) {
                                    { let a11_ = F32(c.m.r32((l_w_6).wrapping_add(2100))); F::EftEmit_Spawn(c, p_set_3, p_state_4, (l_w_6).wrapping_add(1504), p_pos_5, (l_w_6).wrapping_add(32), p_objId_1, 0i32, 6i32, l_group_7, l_sub_8, l_spawn_10, a11_) };
                                }
                            }
                            { let o_ = l_sub_8; let n_ = (o_).wrapping_add(1); l_sub_8 = n_; };
                        }
                    }
                }
            }
            { let o_ = l_group_7; let n_ = (o_).wrapping_add(1); l_group_7 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharaFx_Emit", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharaFx_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharaFx_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_mgr_3: u32 = 0;
    let mut l_w_4: u32 = 0;
    let mut l_dir_5: u32 = 0;
    let mut l_set_6: u32 = 0;
    let mut l_chara_7: i32 = 0;
    let r_ = 'f: {
        l_mgr_3 = { let a_ = (F::BtlTask_GetParent(c, p_task_1)).wrapping_add(56); c.m.r32(a_) };
        l_w_4 = c.m.r32((p_task_1).wrapping_add(56));
        l_dir_5 = (l_w_4).wrapping_add(32);
        l_set_6 = c.m.r32(l_mgr_3);
        { let _ = F::memset(c, l_w_4, 0i32, 2144u32); };
        { let s_ = p_arg_2; let d_ = (l_w_4).wrapping_add(48); rt::memcpy(c, d_, s_, 8); };
        { let r_ = ((1i32) as u32); let a_ = (l_w_4).wrapping_add(2096); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        { let v_ = F32(c.m.r32((p_arg_2).wrapping_add(4))); let a_ = (l_w_4).wrapping_add(2100); c.m.w32(a_, (v_).0); };
        l_chara_7 = { let a0_ = (c.m.r32(p_arg_2) as i32); F::BtlCharApi_GetChara(c, a0_) };
        if (((((l_chara_7) == (55i32)) as i32)) != 0) {
            { let v_ = 55i32; let a_ = (l_w_4).wrapping_add(2120); c.m.w32(a_, (v_) as u32); };
            { let v_ = F32(0x3fc00000); let a_ = (l_w_4).wrapping_add(2128); c.m.w32(a_, (v_).0); };
            { let v_ = 56i32; let a_ = (l_w_4).wrapping_add(2116); c.m.w32(a_, (v_) as u32); };
            { let v_ = 56i32; let a_ = (l_w_4).wrapping_add(2124); c.m.w32(a_, (v_) as u32); };
            { let r_ = ((64i32) as u32); let a_ = (l_w_4).wrapping_add(2096); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        } else {
            if (((((l_chara_7) == (152i32)) as i32)) != 0) {
                { let v_ = 116i32; let a_ = (l_w_4).wrapping_add(2116); c.m.w32(a_, (v_) as u32); };
                { let v_ = 116i32; let a_ = (l_w_4).wrapping_add(2120); c.m.w32(a_, (v_) as u32); };
                { let v_ = 117i32; let a_ = (l_w_4).wrapping_add(2124); c.m.w32(a_, (v_) as u32); };
            } else {
                if (((((l_chara_7) == (153i32)) as i32)) != 0) {
                    { let v_ = 61i32; let a_ = (l_w_4).wrapping_add(2116); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 61i32; let a_ = (l_w_4).wrapping_add(2120); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 62i32; let a_ = (l_w_4).wrapping_add(2124); c.m.w32(a_, (v_) as u32); };
                    { let r_ = F32(c.m.r32((l_w_4).wrapping_add(2100))); let a_ = (l_w_4).wrapping_add(2100); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                }
            }
        }
        { let a0_ = (c.m.r32(p_arg_2) as i32); let a1_ = (c.m.r32((l_w_4).wrapping_add(2116)) as i32); F::BtlCharApi_GetNodePos(c, a0_, a1_, (l_w_4).wrapping_add((((0i32) as u32)).wrapping_mul(16))) };
        { let a0_ = (c.m.r32(p_arg_2) as i32); let a1_ = (c.m.r32((l_w_4).wrapping_add(2116)) as i32); F::BtlCharApi_GetNodePos(c, a0_, a1_, (l_w_4).wrapping_add((((1i32) as u32)).wrapping_mul(16))) };
        { let a0_ = (c.m.r32(p_arg_2) as i32); let a1_ = (c.m.r32((l_w_4).wrapping_add(2124)) as i32); F::BtlCharApi_GetNodePos(c, a0_, a1_, fp) };
        { let a0_ = (c.m.r32(p_arg_2) as i32); let a1_ = (c.m.r32((l_w_4).wrapping_add(2120)) as i32); F::BtlCharApi_GetNodePos(c, a0_, a1_, l_dir_5) };
        F::Vec3_Sub(c, l_dir_5, fp, l_dir_5);
        { let v_ = F32(0x3f800000); let a_ = ((l_w_4).wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec3_Normalize(c, l_dir_5, l_dir_5);
        { let v_ = l_set_6; let a_ = (l_w_4).wrapping_add(56); c.m.w32(a_, v_); };
        F::EftEmit_InitState(c, l_set_6, ((l_w_4).wrapping_add(60)).wrapping_add((((0i32) as u32)).wrapping_mul(716)));
        { let a0_ = c.m.r32((l_w_4).wrapping_add(56)); F::EftEmit_InitState(c, a0_, ((l_w_4).wrapping_add(60)).wrapping_add((((1i32) as u32)).wrapping_mul(716))) };
        { let r_ = ((1i32) as u32); let a_ = (l_w_4).wrapping_add(2104); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        { let a2_ = (c.m.r32((l_w_4).wrapping_add(2116)) as i32); F::EftEmit_SetNode(c, (l_w_4).wrapping_add(1504), 0i32, a2_, ((0i32) as u32)) };
        { let r_ = ((8i32) as u32); let a_ = (l_w_4).wrapping_add(2096); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        { let v_ = rt::i2f({ let a0_ = c.m.r32((l_w_4).wrapping_add(56)); F::EftEmit_GetEndFrames(c, a0_) }); let a_ = (l_w_4).wrapping_add(2112); c.m.w32(a_, (v_).0); };
        { let a1_ = (if ((((((c.m.r32(p_arg_2) as i32)) == (0i32)) as i32)) != 0) { 2048i32 } else { 4096i32 }); F::BtlTask_SetOwnerTag(c, p_task_1, a1_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftCharaFx_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharaFx_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharaFx_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_arg_3: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_arg_3 = (l_w_2).wrapping_add(48);
    { let a0_ = c.m.r32((l_w_2).wrapping_add(56)); F::EftEmit_TermState(c, a0_, ((l_w_2).wrapping_add(60)).wrapping_add((((0i32) as u32)).wrapping_mul(716))) };
    { let a0_ = c.m.r32((l_w_2).wrapping_add(56)); F::EftEmit_TermState(c, a0_, ((l_w_2).wrapping_add(60)).wrapping_add((((1i32) as u32)).wrapping_mul(716))) };
    { let v_ = ((0i32) as u32); let a_ = (l_w_2).wrapping_add(2096); c.m.w32(a_, v_); };
    { let a0_ = (c.m.r32(l_arg_3) as i32); F::EftCharSlot_Clear3(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharaFx_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharaFx_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharaFx_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_w_2: u32 = 0;
    let mut l_arg_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_dir_5: u32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_arg_3 = (l_w_2).wrapping_add(48);
        if ((((!(({ let a0_ = (c.m.r32(l_arg_3) as i32); F::BtlScene_IsEffectStopped(c, a0_, 6i32) }) != 0)) as i32)) != 0) {
            { let a0_ = (c.m.r32(l_arg_3) as i32); let a1_ = (c.m.r32((l_w_2).wrapping_add(2116)) as i32); F::BtlCharApi_GetNodePos(c, a0_, a1_, (l_w_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))) };
            l_dir_5 = (l_w_2).wrapping_add(32);
            { let a0_ = (c.m.r32(l_arg_3) as i32); let a1_ = (c.m.r32((l_w_2).wrapping_add(2116)) as i32); F::BtlCharApi_GetNodePos(c, a0_, a1_, (l_w_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))) };
            { let a0_ = (c.m.r32(l_arg_3) as i32); let a1_ = (c.m.r32((l_w_2).wrapping_add(2124)) as i32); F::BtlCharApi_GetNodePos(c, a0_, a1_, fp) };
            { let a0_ = (c.m.r32(l_arg_3) as i32); let a1_ = (c.m.r32((l_w_2).wrapping_add(2120)) as i32); F::BtlCharApi_GetNodePos(c, a0_, a1_, l_dir_5) };
            F::Vec3_Sub(c, l_dir_5, fp, l_dir_5);
            { let v_ = F32(0x3f800000); let a_ = ((l_w_2).wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            F::Vec3_Normalize(c, l_dir_5, l_dir_5);
            { let a0_ = (c.m.r32(l_arg_3) as i32); F::EftEmit_RefreshFixedNodes(c, a0_, (l_w_2).wrapping_add(1504)) };
            if ((((c.m.r32((l_w_2).wrapping_add(2096))) & (((64i32) as u32)))) != 0) {
                { let a0_ = (c.m.r32(l_arg_3) as i32); let a1_ = (c.m.r32((l_w_2).wrapping_add(2116)) as i32); F::BtlCharApi_GetNodeMtx(c, a0_, a1_, fp.wrapping_add(16)) };
                { let a1_ = F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a2_ = F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a3_ = F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); F::Vec4_Set(c, fp.wrapping_add(80), a1_, a2_, a3_, F32(0x3f800000)) };
                F::Vec3_Normalize(c, fp.wrapping_add(80), fp.wrapping_add(80));
                { let a2_ = F32(c.m.r32((l_w_2).wrapping_add(2128))); F::Vec3_Scale(c, fp.wrapping_add(80), fp.wrapping_add(80), a2_) };
                {
                    l_i_4 = 0i32;
                    'l1: loop {
                        if !(((((l_i_4) < (2i32)) as i32)) != 0) { break 'l1; }
                        'c2: {
                            if (((((l_i_4) == (0i32)) as i32)) != 0) {
                                F::Vec3_Add(c, (l_w_2).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (l_w_2).wrapping_add((((0i32) as u32)).wrapping_mul(16)), fp.wrapping_add(80));
                            } else {
                                F::Vec3_Sub(c, (l_w_2).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)), (l_w_2).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)), fp.wrapping_add(80));
                            }
                        }
                        { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                    }
                }
            }
            {
                l_i_4 = 0i32;
                'l3: loop {
                    if !(((((l_i_4) < (2i32)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        if ((((((((c.m.r32((l_w_2).wrapping_add(2096))) & (((64i32) as u32)))) != 0) || (((((l_i_4) <= (0i32)) as i32)) != 0)) as i32)) != 0) {
                            { let a0_ = (c.m.r32(l_arg_3) as i32); let a2_ = c.m.r32((l_w_2).wrapping_add(56)); F::EftCharaFx_Emit(c, a0_, p_task_1, a2_, ((l_w_2).wrapping_add(60)).wrapping_add((((l_i_4) as u32)).wrapping_mul(716)), (l_w_2).wrapping_add((((l_i_4) as u32)).wrapping_mul(16))) };
                        }
                    }
                    { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                }
            }
            if ((((c.m.r32((l_w_2).wrapping_add(2096))) & (((2i32) as u32)))) != 0) {
                { let r_ = ((4i32) as u32); let a_ = (l_w_2).wrapping_add(2096); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                if ((((!((((c.m.r32((l_w_2).wrapping_add(2096))) & (((32i32) as u32)))) != 0)) as i32)) != 0) {
                    { let r_ = ((32i32) as u32); let a_ = (l_w_2).wrapping_add(2096); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                    { let a0_ = c.m.r32((l_w_2).wrapping_add(56)); F::EftEmit_KillAll(c, a0_, ((l_w_2).wrapping_add(60)).wrapping_add((((0i32) as u32)).wrapping_mul(716))) };
                    { let a0_ = c.m.r32((l_w_2).wrapping_add(56)); F::EftEmit_KillAll(c, a0_, ((l_w_2).wrapping_add(60)).wrapping_add((((1i32) as u32)).wrapping_mul(716))) };
                }
            }
        }
        if ((((c.m.r32((l_w_2).wrapping_add(2096))) & (((4i32) as u32)))) != 0) {
            F::BtlTask_SetDead(c, p_task_1);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftCharaFx_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharaFx_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharaFx_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!((((c.m.r32((l_w_2).wrapping_add(2096))) & (((32i32) as u32)))) != 0)) as i32)) != 0) {
        { let r_ = ((32i32) as u32); let a_ = (l_w_2).wrapping_add(2096); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        { let a0_ = c.m.r32((l_w_2).wrapping_add(56)); F::EftEmit_KillAll(c, a0_, ((l_w_2).wrapping_add(60)).wrapping_add((((0i32) as u32)).wrapping_mul(716))) };
        { let a0_ = c.m.r32((l_w_2).wrapping_add(56)); F::EftEmit_KillAll(c, a0_, ((l_w_2).wrapping_add(60)).wrapping_add((((1i32) as u32)).wrapping_mul(716))) };
    }
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharaFx_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharaFx_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharaFx_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((c.m.r32((l_w_2).wrapping_add(2096))) & (((16i32) as u32)))) != 0) {
        if ((((!(({ let a0_ = c.m.r32((l_w_2).wrapping_add(56)); F::EftEmit_UpdateAlive(c, a0_, ((l_w_2).wrapping_add(60)).wrapping_add((((0i32) as u32)).wrapping_mul(716))) }) != 0)) as i32)) != 0) {
            { let r_ = (((!(8i32))) as u32); let a_ = (l_w_2).wrapping_add(2096); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
        } else {
            { let r_ = ((8i32) as u32); let a_ = (l_w_2).wrapping_add(2096); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        }
        { let _ = { let a0_ = c.m.r32((l_w_2).wrapping_add(56)); F::EftEmit_UpdateAlive(c, a0_, ((l_w_2).wrapping_add(60)).wrapping_add((((1i32) as u32)).wrapping_mul(716))) }; };
        { let v_ = ((0i32) as u32); let a_ = (l_w_2).wrapping_add(2104); c.m.w32(a_, v_); };
    }
    { let r_ = ((16i32) as u32); let a_ = (l_w_2).wrapping_add(2096); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharaFx_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharaFx_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharaFx_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharaFx_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharaFxMgr_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharaFxMgr_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_mgr_3: u32 = 0;
    let mut l_set_4: u32 = 0;
    let mut l_pack_5: u32 = 0;
    let mut l_list_6: u32 = 0;
    let r_ = 'f: {
    l_mgr_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((804u32) as i32)) }; let a_ = l_mgr_3; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(l_mgr_3); F::memset(c, a0_, 0i32, 804u32) }; };
    l_set_4 = c.m.r32(l_mgr_3);
    l_pack_5 = { let a0_ = (c.m.r32((p_arg_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::BtlScene_GetCharPackEntry(c, a0_, 9i32) };
    { let v_ = l_pack_5; let a_ = (l_set_4).wrapping_add(800); c.m.w32(a_, v_); };
    if (((((l_pack_5) != (((0i32) as u32))) as i32)) != 0) {
        F::EftEmit_LoadSet(c, ((0i32) as u32), l_set_4, ((((0i32) as i64 as u64)) as u32), l_pack_5, 2i32, 1i32);
    }
    l_list_6 = F::BtlTask_CreateChildList(c, p_task_1, ((((4i32) as i64 as u64)) as u32), ((((((2144u32) as i32)) as i64 as u64)) as u32));
    { let a0_ = (c.m.r32((p_arg_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a1_ = (c.m.r32((p_arg_2).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); F::EftChar_SetList(c, a0_, a1_, l_list_6) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharaFxMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharaFxMgr_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharaFxMgr_Term", &PROBE_N, &[(p_task_1) as u32], false); }
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
    if rt::probing() { crate::host::harness::probe(c, "EftCharaFxMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharaFxMgr_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharaFxMgr_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_mgr_2: u32 = 0;
    let r_ = 'f: {
    l_mgr_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((({ let a_ = (c.m.r32(l_mgr_2)).wrapping_add(800); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(l_mgr_2); F::EftEmit_BeginFrame(c, a0_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharaFxMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharaFx_Start(c: &mut Ctx, mut p_arg_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharaFx_Start", &PROBE_N, &[(p_arg_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_list_2: u32 = 0;
    let mut l_task_3: u32 = 0;
    let r_ = 'f: {
        { let s_ = p_arg_1; let d_ = fp; rt::memcpy(c, d_, s_, 8); };
        if ((((({ let a0_ = (c.m.r32(fp) as i32); F::EftCharSlot_Get3(c, a0_) }) != (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        l_list_2 = { let a0_ = (c.m.r32(fp) as i32); F::EftChar_GetList(c, a0_, 7i32) };
        if (((((l_list_2) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        l_task_3 = F::BtlTaskList_AddTail(c, l_list_2, sym::gEftCharaFxClass, fp);
        if (((((l_task_3) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        { let a0_ = (c.m.r32(fp) as i32); F::EftCharSlot_Set3(c, a0_, l_task_3) };
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftCharaFx_Start", &PROBE_N, &[], true); }
    r_
}
