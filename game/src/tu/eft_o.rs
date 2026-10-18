#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EftBolt_UpdateTex(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBolt_UpdateTex", &PROBE_N, &[(p_w_1) as u32], false); }
    let mut l_set_2: u32 = 0;
    let mut l_t_3: u32 = 0;
    let mut l_frame_4: u64 = 0;
    let mut l_base_5: u64 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_one_7: i32 = 0;
    let r_ = 'f: {
    l_set_2 = (((c.m.r32(sym::gEftBoltPool)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(560))).wrapping_add(40);
    l_t_3 = ((c.m.r32(sym::gEftBoltPool)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(560));
    if ((((!(((c.m.r32(l_t_3) as i32)) != 0)) as i32)) != 0) {
        { let v_ = F::EftVram_AddImage(c, l_set_2, 1i32, 0i32); let a_ = (l_t_3).wrapping_add(16); c.m.w64(a_, v_); };
        l_frame_4 = F::EftVram_AddClut(c, l_set_2);
        l_one_7 = 1i32;
        { let v_ = ((c.m.r64((l_t_3).wrapping_add(16))) | ((l_frame_4).wrapping_shl((37i32) as u32))); let a_ = (p_w_1).wrapping_add(3736); c.m.w64(a_, v_); };
        { let v_ = l_one_7; let a_ = (l_t_3).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = l_one_7; let a_ = l_t_3; c.m.w32(a_, (v_) as u32); };
        { let v_ = l_frame_4; let a_ = (l_t_3).wrapping_add(24); c.m.w64(a_, v_); };
        l_set_2 = (((c.m.r32(sym::gEftBoltPool)).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(560))).wrapping_add(40);
        l_base_5 = F::EftVram_AddImage(c, l_set_2, 1i32, 0i32);
        {
            l_i_6 = 0i32;
            'l1: loop {
                if !(((((l_i_6) < ((c.m.r32((l_set_2).wrapping_add(512)) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = ((l_base_5) | ((F::EftVram_AddClut(c, (l_set_2).wrapping_add((((l_i_6) as u32)).wrapping_mul(16)))).wrapping_shl((37i32) as u32))); let a_ = (l_set_2).wrapping_add((((l_i_6) as u32)).wrapping_mul(16)); c.m.w64(a_, v_); };
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
    } else {
        if ((((!(((((c.m.r32((l_t_3).wrapping_add(8)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
            l_frame_4 = F::EftVram_AddClut(c, l_set_2);
        } else {
            l_frame_4 = c.m.r64((l_t_3).wrapping_add(24));
        }
        { let v_ = ((c.m.r64((l_t_3).wrapping_add(16))) | ((l_frame_4).wrapping_shl((37i32) as u32))); let a_ = (p_w_1).wrapping_add(3736); c.m.w64(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBolt_UpdateTex", &PROBE_N, &[], true); }
    r_
}

pub fn EftBolt_ReadChar(c: &mut Ctx, mut p_w_1: u32, mut p_objId_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBolt_ReadChar", &PROBE_N, &[(p_w_1) as u32, (p_objId_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = F::BtlCharApi_ObjGetParamFlags0(c, p_objId_2); let a_ = (p_w_1).wrapping_add(3696); c.m.w32(a_, (v_) as u32); };
    { let v_ = rt::fdiv(F::BtlCharApi_GetHeight(c, p_objId_2), F32(0x419acccc)); let a_ = (p_w_1).wrapping_add(3744); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBolt_ReadChar", &PROBE_N, &[], true); }
    r_
}

pub fn EftBoltTask_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBoltTask_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let _ = F::memset(c, l_w_3, 0i32, 3760u32); };
    { let v_ = (c.m.r32(p_arg_2) as i32); let a_ = (l_w_3).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let r_ = 1i32; let a_ = l_w_3; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    { let a1_ = (c.m.r32(p_arg_2) as i32); F::EftBolt_ReadChar(c, l_w_3, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBoltTask_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftBoltTask_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBoltTask_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_bolt_3: u32 = 0;
    let mut l_objId_4: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_bolt_3 = (l_w_2).wrapping_add(16);
    l_objId_4 = (l_w_2).wrapping_add(4);
    F::EftBolt_FreeSegs(c, l_bolt_3);
    { let v_ = 0i32; let a_ = (l_bolt_3).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((0i32) as u32); let a_ = ({ let a_ = (c.m.r32(sym::gEftBoltPool)).wrapping_add(1136); c.m.r32(a_) }).wrapping_add(((((c.m.r32(l_objId_4) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBoltTask_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftBoltTask_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBoltTask_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_busy_2: i32 = 0;
    let mut l_w_3: u32 = 0;
    let mut l_objId_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
    l_busy_2 = 0i32;
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    l_objId_4 = (l_w_3).wrapping_add(4);
    if (({ let a0_ = (c.m.r32(l_objId_4) as i32); F::BtlCharApi_IsModelNew(c, a0_) }) != 0) {
        { let r_ = 8i32; let a_ = l_w_3; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if (((((c.m.r32(l_w_3) as i32)) & (8i32))) != 0) {
        { let a1_ = (c.m.r32(l_objId_4) as i32); F::EftBolt_ReadChar(c, l_w_3, a1_) };
        { let r_ = (!(8i32)); let a_ = l_w_3; let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
        if ((((!(((((c.m.r32(l_w_3) as i32)) & (2i32))) != 0)) as i32)) != 0) {
            { let a1_ = (c.m.r32(l_objId_4) as i32); F::EftBolt_Spawn(c, l_w_3, a1_) };
        }
        { let a1_ = (c.m.r32(l_objId_4) as i32); F::EftBolt_UpdateAll(c, l_w_3, a1_) };
        { let a1_ = (c.m.r32(l_objId_4) as i32); F::EftBolt_UpdateFlashes(c, l_w_3, a1_) };
        if (((((c.m.r32(l_w_3) as i32)) & (2i32))) != 0) {
            {
                l_i_5 = 0i32;
                'l1: loop {
                    if !(((((l_i_5) < (10i32)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        if (((c.m.r32((((l_w_3).wrapping_add(16)).wrapping_add((((l_i_5) as u32)).wrapping_mul(144))).wrapping_add(12)) as i32)) != 0) {
                            l_busy_2 = 1i32;
                            break 'l1;
                        }
                    }
                    { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
                }
            }
            {
                l_i_5 = 0i32;
                'l3: loop {
                    if !(((((l_i_5) < (20i32)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        if (((c.m.r32(((l_w_3).wrapping_add(1456)).wrapping_add((((l_i_5) as u32)).wrapping_mul(112))) as i32)) != 0) {
                            l_busy_2 = 1i32;
                            break 'l3;
                        }
                    }
                    { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
                }
            }
            if ((((!((l_busy_2) != 0)) as i32)) != 0) {
                { let r_ = 4i32; let a_ = l_w_3; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
    }
    if (((((c.m.r32(l_w_3) as i32)) & (4i32))) != 0) {
        F::BtlTask_SetDead(c, p_task_1);
    } else {
        F::EftBolt_UpdateTex(c, l_w_3);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBoltTask_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftBoltTask_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBoltTask_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBoltTask_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftBoltTask_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBoltTask_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    { let v_ = ((0i32) as u32); let a_ = ({ let a_ = (c.m.r32(sym::gEftBoltPool)).wrapping_add(1136); c.m.r32(a_) }).wrapping_add(((((c.m.r32((l_w_2).wrapping_add(4)) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBoltTask_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftBoltTask_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBoltTask_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_alpha_2: F32 = F32(0);
    let mut l_w_3: u32 = 0;
    let mut l_objId_4: u32 = 0;
    let r_ = 'f: {
    l_alpha_2 = F32(0x3f800000);
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    l_objId_4 = (l_w_3).wrapping_add(4);
    if ((((!(({ let a0_ = (c.m.r32(l_objId_4) as i32); F::BtlCharApi_IsHidden(c, a0_) }) != 0)) as i32)) != 0) {
        if (((((({ let a0_ = (c.m.r32(l_objId_4) as i32); F::BtlScene_IsCharInView(c, a0_) }) != 0) && (({ let a0_ = (c.m.r32(l_objId_4) as i32); F::BtlCharApi_ObjTestFlagBit21(c, a0_) }) != 0)) as i32)) != 0) {
            l_alpha_2 = F32(0x3e999999);
        }
        F::EftBolt_DrawAll(c, l_w_3, l_alpha_2);
        F::EftBolt_DrawFlashes(c, l_w_3, l_alpha_2);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBoltTask_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftBoltMgr_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBoltMgr_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_t_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    { let v_ = { let a0_ = ((((F::BtlPool_GetCurrent(c)) as i64 as u64)) as u32); F::BtlPool_Alloc(c, ((((a0_) as u64)) as i32), ((1144u32) as i32)) }; let a_ = sym::gEftBoltPool; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gEftBoltPool); F::memset(c, a0_, 0i32, 1144u32) }; };
    { let v_ = F::BtlScene_GetCharCount(c); let a_ = c.m.r32(sym::gEftBoltPool); c.m.w32(a_, (v_) as u32); };
    { let v_ = ({ let a_ = c.m.r32(sym::gEftBoltPool); (c.m.r32(a_) as i32) }).wrapping_mul(100i32); let a_ = (c.m.r32(sym::gEftBoltPool)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a0_ = ((((F::BtlPool_GetCurrent(c)) as i64 as u64)) as u32); let a1_ = ({ let a_ = (c.m.r32(sym::gEftBoltPool)).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_shl((6i32) as u32); F::BtlPool_Alloc(c, ((((a0_) as u64)) as i32), a1_) }; let a_ = (c.m.r32(sym::gEftBoltPool)).wrapping_add(8); c.m.w32(a_, v_); };
    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gEftBoltPool)).wrapping_add(8); c.m.r32(a_) }; let a2_ = ((({ let a_ = (c.m.r32(sym::gEftBoltPool)).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_shl((6i32) as u32)) as u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = { let a0_ = ((((F::BtlPool_GetCurrent(c)) as i64 as u64)) as u32); let a1_ = ({ let a_ = c.m.r32(sym::gEftBoltPool); (c.m.r32(a_) as i32) }).wrapping_mul(4i32); F::BtlPool_Alloc(c, ((((a0_) as u64)) as i32), a1_) }; let a_ = (c.m.r32(sym::gEftBoltPool)).wrapping_add(1136); c.m.w32(a_, v_); };
    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gEftBoltPool)).wrapping_add(1136); c.m.r32(a_) }; let a2_ = ((({ let a_ = c.m.r32(sym::gEftBoltPool); (c.m.r32(a_) as i32) }).wrapping_mul(4i32)) as u32); F::memset(c, a0_, 0i32, a2_) }; };
    l_t_2 = ((c.m.r32(sym::gEftBoltPool)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(560));
    { let v_ = F::BtlScene_GetCommonEntry(c, 5i32); let a_ = (l_t_2).wrapping_add(32); c.m.w32(a_, v_); };
    l_t_2 = ((c.m.r32(sym::gEftBoltPool)).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(560));
    { let v_ = F::BtlScene_GetCommonEntry(c, 6i32); let a_ = (l_t_2).wrapping_add(32); c.m.w32(a_, v_); };
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = (((c.m.r32(sym::gEftBoltPool)).wrapping_add(16)).wrapping_add((((l_i_3) as u32)).wrapping_mul(560))).wrapping_add(40); let a1_ = { let a_ = (((c.m.r32(sym::gEftBoltPool)).wrapping_add(16)).wrapping_add((((l_i_3) as u32)).wrapping_mul(560))).wrapping_add(32); c.m.r32(a_) }; F::EftTexSet_Load32(c, a0_, a1_) };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    { let v_ = F::BtlScene_GetCommonEntry(c, 4i32); let a_ = (c.m.r32(sym::gEftBoltPool)).wrapping_add(1140); c.m.w32(a_, v_); };
    {
        l_i_3 = 0i32;
        'l3: loop {
            if !(((((l_i_3) < ({ let a_ = c.m.r32(sym::gEftBoltPool); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l3; }
            'c4: {
                { let v_ = ((0i32) as u32); let a_ = ({ let a_ = (c.m.r32(sym::gEftBoltPool)).wrapping_add(1136); c.m.r32(a_) }).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    { let v_ = { let a1_ = { let a_ = c.m.r32(sym::gEftBoltPool); (c.m.r32(a_) as i32) }; F::BtlTask_CreateChildList(c, p_task_1, ((((a1_) as i64 as u64)) as u32), ((((((3760u32) as i32)) as i64 as u64)) as u32)) }; let a_ = sym::gEftBoltList; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBoltMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftBoltMgr_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBoltMgr_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = ((((F::BtlPool_GetCurrent(c)) as i64 as u64)) as u32); let a1_ = { let a_ = (c.m.r32(sym::gEftBoltPool)).wrapping_add(1136); c.m.r32(a_) }; F::BtlPool_Free(c, ((((a0_) as u64)) as i32), a1_) };
    { let a0_ = ((((F::BtlPool_GetCurrent(c)) as i64 as u64)) as u32); let a1_ = { let a_ = (c.m.r32(sym::gEftBoltPool)).wrapping_add(8); c.m.r32(a_) }; F::BtlPool_Free(c, ((((a0_) as u64)) as i32), a1_) };
    { let a0_ = ((((F::BtlPool_GetCurrent(c)) as i64 as u64)) as u32); let a1_ = c.m.r32(sym::gEftBoltPool); F::BtlPool_Free(c, ((((a0_) as u64)) as i32), a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftBoltPool; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBoltMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftBoltMgr_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBoltMgr_Update", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gEftBoltPool)).wrapping_add(16)).wrapping_add((((l_i_1) as u32)).wrapping_mul(560)); c.m.w32(a_, (v_) as u32); };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBoltMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftBoltMgr_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBoltMgr_Reset", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBoltMgr_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftBolt_Enable(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBolt_Enable", &PROBE_N, &[(p_objId_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_t_2: u32 = 0;
    let r_ = 'f: {
        if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gEftBoltPool)).wrapping_add(1136); c.m.r32(a_) }).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
            { let v_ = p_objId_1; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            l_t_2 = { let a0_ = c.m.r32(sym::gEftBoltList); F::BtlTaskList_AddTail(c, a0_, sym::gEftBoltTaskClass, fp) };
            if (((((l_t_2) == (((0i32) as u32))) as i32)) != 0) {
                break 'f 0i32;
            }
            { let v_ = l_t_2; let a_ = ({ let a_ = (c.m.r32(sym::gEftBoltPool)).wrapping_add(1136); c.m.r32(a_) }).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBolt_Enable", &PROBE_N, &[], true); }
    r_
}

pub fn EftBolt_Disable(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBolt_Disable", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_t_2: u32 = 0;
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    l_t_2 = { let a_ = ({ let a_ = (c.m.r32(sym::gEftBoltPool)).wrapping_add(1136); c.m.r32(a_) }).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    if (((((l_t_2) != (((0i32) as u32))) as i32)) != 0) {
        l_w_3 = c.m.r32((l_t_2).wrapping_add(56));
        if ((((!(((((c.m.r32(l_w_3) as i32)) & (1i32))) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        { let r_ = 2i32; let a_ = l_w_3; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBolt_Disable", &PROBE_N, &[], true); }
    r_
}

pub fn EftRays_Create(c: &mut Ctx, mut p_arg_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRays_Create", &PROBE_N, &[(p_arg_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a0_ = { let a_ = (c.m.r32(sym::gEftRays)).wrapping_add(2572); c.m.r32(a_) }; F::BtlTaskList_AddTail(c, a0_, sym::gEftRaysClass, p_arg_1) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRays_Create", &PROBE_N, &[], true); }
    r_
}

pub fn EftRays_Stop(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRays_Stop", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    if ((F::EftRays_IsTask(c, p_task_1)) != 0) {
        { let r_ = 8i32; let a_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(84); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRays_Stop", &PROBE_N, &[], true); }
    r_
}

pub fn EftRays_Kill(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRays_Kill", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    if ((F::EftRays_IsTask(c, p_task_1)) != 0) {
        { let r_ = 16i32; let a_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(84); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRays_Kill", &PROBE_N, &[], true); }
    r_
}

pub fn EftRays_SetPos(c: &mut Ctx, mut p_task_1: u32, mut p_pos_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRays_SetPos", &PROBE_N, &[(p_task_1) as u32, (p_pos_2) as u32], false); }
    let r_ = 'f: {
    if ((F::EftRays_IsTask(c, p_task_1)) != 0) {
        { let a0_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(32); F::Vec4_Copy(c, a0_, p_pos_2) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRays_SetPos", &PROBE_N, &[], true); }
    r_
}

pub fn EftRays_SetDelay(c: &mut Ctx, mut p_task_1: u32, mut p_frames_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRays_SetDelay", &PROBE_N, &[(p_task_1) as u32, (p_frames_2).0], false); }
    let r_ = 'f: {
    if ((F::EftRays_IsTask(c, p_task_1)) != 0) {
        { let v_ = p_frames_2; let a_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(56); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRays_SetDelay", &PROBE_N, &[], true); }
    r_
}

pub fn EftRays_SetHold(c: &mut Ctx, mut p_task_1: u32, mut p_frames_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRays_SetHold", &PROBE_N, &[(p_task_1) as u32, (p_frames_2).0], false); }
    let r_ = 'f: {
    if ((F::EftRays_IsTask(c, p_task_1)) != 0) {
        { let v_ = p_frames_2; let a_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(60); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRays_SetHold", &PROBE_N, &[], true); }
    r_
}

pub fn EftRays_SetFade(c: &mut Ctx, mut p_task_1: u32, mut p_frames_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRays_SetFade", &PROBE_N, &[(p_task_1) as u32, (p_frames_2).0], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if ((F::EftRays_IsTask(c, p_task_1)) != 0) {
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        { let v_ = p_frames_2; let a_ = (l_w_3).wrapping_add(68); c.m.w32(a_, (v_).0); };
        { let v_ = p_frames_2; let a_ = (l_w_3).wrapping_add(64); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRays_SetFade", &PROBE_N, &[], true); }
    r_
}

pub fn EftRays_IsAlive(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRays_IsAlive", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_ret_2: i32 = 0;
    let r_ = 'f: {
    l_ret_2 = F::EftRays_IsTask(c, p_task_1);
    if ((l_ret_2) != 0) {
        if (((({ let a_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(84); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
            break 'f 1i32;
        }
        break 'f 0i32;
    }
    break 'f l_ret_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRays_IsAlive", &PROBE_N, &[], true); }
    r_
}

pub fn EftRays_SetSize(c: &mut Ctx, mut p_task_1: u32, mut p_size_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRays_SetSize", &PROBE_N, &[(p_task_1) as u32, (p_size_2).0], false); }
    let r_ = 'f: {
    if ((F::EftRays_IsTask(c, p_task_1)) != 0) {
        { let v_ = p_size_2; let a_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(80); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRays_SetSize", &PROBE_N, &[], true); }
    r_
}

pub fn EftRaysMgr_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRaysMgr_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    { let v_ = { let a0_ = ((((F::BtlPool_GetCurrent(c)) as i64 as u64)) as u32); F::BtlPool_Alloc(c, ((((a0_) as u64)) as i32), ((2576u32) as i32)) }; let a_ = sym::gEftRays; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gEftRays); F::memset(c, a0_, 0i32, 2576u32) }; };
    { let a0_ = (c.m.r32(sym::gEftRays)).wrapping_add(2560); F::List_Init(c, a0_) };
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (20i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = (c.m.r32(sym::gEftRays)).wrapping_add(2560); let a1_ = (c.m.r32(sym::gEftRays)).wrapping_add((((l_i_2) as u32)).wrapping_mul(128)); F::List_PushBack(c, a0_, a1_) };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    { let v_ = F::BtlTask_CreateChildList(c, p_task_1, ((((3i32) as i64 as u64)) as u32), ((((((96u32) as i32)) as i64 as u64)) as u32)); let a_ = (c.m.r32(sym::gEftRays)).wrapping_add(2572); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRaysMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftRaysMgr_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRaysMgr_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftRays)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = ((((F::BtlPool_GetCurrent(c)) as i64 as u64)) as u32); let a1_ = c.m.r32(sym::gEftRays); F::BtlPool_Free(c, ((((a0_) as u64)) as i32), a1_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gEftRays; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRaysMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftRaysMgr_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRaysMgr_Update", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRaysMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftRays_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRays_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_one_3: F32 = F32(0);
    let mut l_w_4: u32 = 0;
    let mut l_def_5: u32 = 0;
    let mut l_list_6: u32 = 0;
    let mut l_i_7: i32 = 0;
    let mut l_ray_8: u32 = 0;
    let r_ = 'f: {
    l_one_3 = F32(0x3f800000);
    l_w_4 = c.m.r32((p_task_1).wrapping_add(56));
    { let _ = F::memset(c, l_w_4, 0i32, 96u32); };
    { let s_ = p_arg_2; let d_ = (l_w_4).wrapping_add(16); rt::memcpy(c, d_, s_, 16); };
    { let v_ = l_one_3; let a_ = (l_w_4).wrapping_add(76); c.m.w32(a_, (v_).0); };
    { let v_ = l_one_3; let a_ = (l_w_4).wrapping_add(80); c.m.w32(a_, (v_).0); };
    { let r_ = 1i32; let a_ = (l_w_4).wrapping_add(84); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    if (((rt::fgt(F32(c.m.r32((p_arg_2).wrapping_add(12))), F32(0x00000000)) as i32)) != 0) {
        { let v_ = rt::fmul(F32(c.m.r32((p_arg_2).wrapping_add(12))), F32(0x41f00000)); let a_ = (l_w_4).wrapping_add(72); c.m.w32(a_, (v_).0); };
        { let r_ = 4i32; let a_ = (l_w_4).wrapping_add(84); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    l_def_5 = c.m.r32(((l_w_4).wrapping_add(16)).wrapping_add(4));
    l_list_6 = (l_w_4).wrapping_add(4);
    l_i_7 = 0i32;
    F::List_Init(c, l_list_6);
    {
        'l1: loop {
            if !(((((l_i_7) < ((c.m.r32((l_def_5).wrapping_add(1200)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                l_ray_8 = F::EftRays_AllocRay(c, l_list_6);
                if (((((l_ray_8) == (((0i32) as u32))) as i32)) != 0) {
                    break 'f;
                }
                { let _ = F::memset(c, l_ray_8, 0i32, 128u32); };
                { let v_ = ((l_i_7) as i16); let a_ = (l_ray_8).wrapping_add(108); c.m.w16(a_, (v_) as u16); };
                { let v_ = rt::fdiv(rt::fmul({ let a_ = ((l_def_5).wrapping_add(((((c.m.r16((l_ray_8).wrapping_add(108)) as i16)) as u32)).wrapping_mul(120))).wrapping_add(96); F32(c.m.r32(a_)) }, F32(0x40490fda)), F32(0x43340000)); let a_ = (l_ray_8).wrapping_add(40); c.m.w32(a_, (v_).0); };
                { let v_ = (({ let a_ = ((l_def_5).wrapping_add(((((c.m.r16((l_ray_8).wrapping_add(108)) as i16)) as u32)).wrapping_mul(120))).wrapping_add(112); c.m.r16(a_) }) as i16); let a_ = (l_ray_8).wrapping_add(110); c.m.w16(a_, (v_) as u16); };
                { let v_ = F32(0x00000000); let a_ = (l_ray_8).wrapping_add(48); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul({ let a_ = ((l_def_5).wrapping_add(((((c.m.r16((l_ray_8).wrapping_add(108)) as i16)) as u32)).wrapping_mul(120))).wrapping_add(64); F32(c.m.r32(a_)) }, F32(0x41f00000)); let a_ = (l_ray_8).wrapping_add(52); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd({ let a_ = ((l_def_5).wrapping_add(((((c.m.r16((l_ray_8).wrapping_add(108)) as i16)) as u32)).wrapping_mul(120))).wrapping_add(48); F32(c.m.r32(a_)) }, rt::fmul({ let a_ = (((l_def_5).wrapping_add(((((c.m.r16((l_ray_8).wrapping_add(108)) as i16)) as u32)).wrapping_mul(120))).wrapping_add(48)).wrapping_add(4); F32(c.m.r32(a_)) }, rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = (l_ray_8).wrapping_add(56); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd({ let a_ = (((l_def_5).wrapping_add(((((c.m.r16((l_ray_8).wrapping_add(108)) as i16)) as u32)).wrapping_mul(120))).wrapping_add(48)).wrapping_add(8); F32(c.m.r32(a_)) }, rt::fmul({ let a_ = (((l_def_5).wrapping_add(((((c.m.r16((l_ray_8).wrapping_add(108)) as i16)) as u32)).wrapping_mul(120))).wrapping_add(48)).wrapping_add(12); F32(c.m.r32(a_)) }, rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = (l_ray_8).wrapping_add(60); c.m.w32(a_, (v_).0); };
                { let v_ = 0i32; let a_ = (l_ray_8).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
                { let v_ = l_one_3; let a_ = (l_ray_8).wrapping_add(44); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = (l_ray_8).wrapping_add(96); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul({ let a_ = (((l_def_5).wrapping_add(((((c.m.r16((l_ray_8).wrapping_add(108)) as i16)) as u32)).wrapping_mul(120))).wrapping_add(80)).wrapping_add(12); F32(c.m.r32(a_)) }, F32(0x41f00000)); let a_ = (l_ray_8).wrapping_add(100); c.m.w32(a_, (v_).0); };
                { let v_ = 0i32; let a_ = (l_ray_8).wrapping_add(104); c.m.w32(a_, (v_) as u32); };
                { let v_ = l_one_3; let a_ = ((l_ray_8).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let v_ = l_one_3; let a_ = ((l_ray_8).wrapping_add(80)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let v_ = l_one_3; let a_ = ((l_ray_8).wrapping_add(80)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let v_ = l_one_3; let a_ = ((l_ray_8).wrapping_add(80)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                if (((((c.m.r32((l_def_5).wrapping_add(1204)) as i32)) & (1i32))) != 0) {
                    F::EftRays_SetKey(c, l_ray_8, l_def_5, 0i32);
                } else {
                    F::EftRays_SetKey(c, l_ray_8, l_def_5, 2i32);
                }
            }
            { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRays_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftRays_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRays_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_list_2: u32 = 0;
    let mut l_ray_3: u32 = 0;
    let r_ = 'f: {
    l_list_2 = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(4);
    l_ray_3 = F::List_GetHead(c, l_list_2);
    'l1: while (((((l_ray_3) != (((0i32) as u32))) as i32)) != 0) {
        l_ray_3 = F::EftRays_FreeRay(c, l_list_2, l_ray_3);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRays_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftRays_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRays_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_alpha_2: F32 = F32(0);
    let mut l_w_3: u32 = 0;
    let mut l_arg_4: u32 = 0;
    let r_ = 'f: {
    l_alpha_2 = F32(0x00000000);
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    l_arg_4 = (l_w_3).wrapping_add(16);
    if ((((!(({ let a0_ = ((c.m.r8(l_arg_4)) as i32); let a1_ = ((c.m.r8((l_arg_4).wrapping_add(1))) as i32); F::BtlScene_IsEffectStopped(c, a0_, a1_) }) != 0)) as i32)) != 0) {
        if (((rt::fle(F32(c.m.r32((l_w_3).wrapping_add(56))), F32(0x00000000)) as i32)) != 0) {
            F::EftRays_Step(c, l_w_3);
            if (((((c.m.r32((l_w_3).wrapping_add(84)) as i32)) & (4i32))) != 0) {
                { let r_ = F32(0x3f800000); let a_ = (l_w_3).wrapping_add(72); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::fle(F32(c.m.r32((l_w_3).wrapping_add(72))), F32(0x00000000)) as i32)) != 0) {
                    { let r_ = 8i32; let a_ = (l_w_3).wrapping_add(84); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
        } else {
            { let r_ = F32(0x3f800000); let a_ = (l_w_3).wrapping_add(56); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
        if (((((c.m.r32((l_w_3).wrapping_add(84)) as i32)) & (8i32))) != 0) {
            if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_w_3).wrapping_add(60)))) as i32)) != 0) {
                { let r_ = F32(0x3f800000); let a_ = (l_w_3).wrapping_add(60); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            } else {
                if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_w_3).wrapping_add(64)))) as i32)) != 0) {
                    l_alpha_2 = rt::fdiv(F32(c.m.r32((l_w_3).wrapping_add(68))), F32(c.m.r32((l_w_3).wrapping_add(64))));
                    { let r_ = F32(0x3f800000); let a_ = (l_w_3).wrapping_add(68); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                }
                if (((rt::fle(F32(c.m.r32((l_w_3).wrapping_add(68))), F32(0x00000000)) as i32)) != 0) {
                    { let r_ = 16i32; let a_ = (l_w_3).wrapping_add(84); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let r_ = (!(1i32)); let a_ = (l_w_3).wrapping_add(84); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                    l_alpha_2 = F32(0x00000000);
                }
                { let v_ = l_alpha_2; let a_ = (l_w_3).wrapping_add(76); c.m.w32(a_, (v_).0); };
            }
        }
    }
    if (((((c.m.r32((l_w_3).wrapping_add(84)) as i32)) & (16i32))) != 0) {
        F::BtlTask_SetDead(c, p_task_1);
    } else {
        { let v_ = { let a0_ = c.m.r32((l_arg_4).wrapping_add(8)); let a1_ = ((c.m.r8((l_arg_4).wrapping_add(2))) as i32); F::EftRays_GetTex(c, a0_, a1_) }; let a_ = (l_w_3).wrapping_add(48); c.m.w64(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRays_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftRays_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRays_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRays_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftRays_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRays_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_arg_3: u32 = 0;
    let mut l_ray_4: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_arg_3 = (l_w_2).wrapping_add(16);
    if ((((!(({ let a0_ = ((c.m.r8(l_arg_3)) as i32); let a1_ = ((c.m.r8((l_arg_3).wrapping_add(1))) as i32); F::BtlScene_IsEffectHidden(c, a0_, a1_) }) != 0)) as i32)) != 0) {
        l_ray_4 = F::List_GetHead(c, (l_w_2).wrapping_add(4));
        F::Vu0Cur_Push(c);
        { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
        {
            'l1: loop {
                if !(((((l_ray_4) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((c.m.r32((l_ray_4).wrapping_add(112)) as i32)) & (1i32))) != 0) {
                        F::EftRays_DrawRay(c, l_w_2, l_ray_4);
                    }
                }
                l_ray_4 = F::List_GetNext(c, l_ray_4);
            }
        }
        F::Vu0Cur_Pop(c);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRays_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftRays_DrawRay(c: &mut Ctx, mut p_w_1: u32, mut p_ray_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRays_DrawRay", &PROBE_N, &[(p_w_1) as u32, (p_ray_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(880) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 880);
    let mut l_def_3: u32 = 0;
    let mut l_pos_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_rect_6: u32 = 0;
    let mut l_grow_7: F32 = F32(0);
    let mut l_ratio_8: F32 = F32(0);
    let r_ = 'f: {
        l_i_5 = 0i32;
        l_rect_6 = (p_ray_2).wrapping_add(8);
        l_def_3 = c.m.r32(((p_w_1).wrapping_add(16)).wrapping_add(4));
        F::Vec4_Set(c, (fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), F32(0xbf800000), F32(0xbf800000), F32(0x3f800000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), F32(0x3f800000), F32(0xbf800000), F32(0x3f800000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), F32(0xbf800000), F32(0x3c23d70a), F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), F32(0x3f800000), F32(0x3c23d70a), F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(256)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), F32(0x00000000), F32(0x3d4ccccc), F32(0x3f800000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(256)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), F32(0x3f800000), F32(0x3d4ccccc), F32(0x3f800000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(256)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), F32(0x00000000), F32(0x3f800000), F32(0x3f800000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(256)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), F32(0x3f800000), F32(0x3f800000), F32(0x3f800000), F32(0x3f800000));
        F::Mtx_StoreIdentity(c, fp);
        F::Mtx_StoreIdentity(c, fp.wrapping_add(64));
        l_pos_4 = (p_w_1).wrapping_add(32);
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(64); F::Mtx_InverseRT(c, fp, a1_) };
        { let v_ = F32(0x00000000); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        F::Mtx_Mul(c, fp.wrapping_add(64), fp.wrapping_add(64), fp);
        l_ratio_8 = rt::fdiv(F32(c.m.r32((l_rect_6).wrapping_add(8))), F32(c.m.r32((l_rect_6).wrapping_add(12))));
        { let v_ = F32(0x00000000); let a_ = ((fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        l_grow_7 = F32(0x00000000);
        {
            'l1: loop {
                if !(((((l_i_5) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::Vec4_Copy(c, fp.wrapping_add(752), (fp.wrapping_add(192)).wrapping_add((((l_i_5) as u32)).wrapping_mul(16)));
                    F::Mtx_StoreIdentity(c, fp);
                    { let v_ = rt::fmul(rt::fadd(rt::fmul(F32(c.m.r32((l_rect_6).wrapping_add(4))), F32(c.m.r32((p_ray_2).wrapping_add(44)))), l_grow_7), F32(c.m.r32((p_w_1).wrapping_add(80)))); let a_ = fp.wrapping_add(784); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fmul(rt::fadd(F32(c.m.r32((l_rect_6).wrapping_add(8))), l_grow_7), F32(c.m.r32((p_w_1).wrapping_add(80)))); let a_ = (fp.wrapping_add(784)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fmul(rt::fmul(l_ratio_8, F32(0xc1200000)), F32(c.m.r32((p_w_1).wrapping_add(80)))); let a_ = (fp.wrapping_add(784)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(784)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    F::Mtx_ScaleDiag(c, fp, fp, fp.wrapping_add(784));
                    { let a2_ = F32(c.m.r32((p_ray_2).wrapping_add(40))); F::Mtx_RotateZ(c, fp, fp, a2_) };
                    F::Mtx_MulVec4(c, fp.wrapping_add(768), fp, fp.wrapping_add(752));
                    F::Mtx_MulVec4(c, (fp.wrapping_add(128)).wrapping_add((((l_i_5) as u32)).wrapping_mul(16)), fp.wrapping_add(64), fp.wrapping_add(768));
                    F::Vec4_Add(c, (fp.wrapping_add(128)).wrapping_add((((l_i_5) as u32)).wrapping_mul(16)), (fp.wrapping_add(128)).wrapping_add((((l_i_5) as u32)).wrapping_mul(16)), l_pos_4);
                    { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(128)).wrapping_add((((l_i_5) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
        { let v_ = rt::fmul(rt::i2f(((c.m.r8(l_rect_6)) as i32)), F32(c.m.r32(((p_ray_2).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = fp.wrapping_add(800); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::i2f(((c.m.r8((l_rect_6).wrapping_add(1))) as i32)), F32(c.m.r32(((p_ray_2).wrapping_add(80)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = (fp.wrapping_add(800)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::i2f(((c.m.r8((l_rect_6).wrapping_add(2))) as i32)), F32(c.m.r32(((p_ray_2).wrapping_add(80)).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let a_ = (fp.wrapping_add(800)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::i2f(((c.m.r8((l_rect_6).wrapping_add(3))) as i32)), F32(c.m.r32((p_w_1).wrapping_add(76)))); let a_ = (fp.wrapping_add(800)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = (if (((rt::flt(F32(c.m.r32(fp.wrapping_add(800))), F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(0x437f0000), F32(c.m.r32(fp.wrapping_add(800)))) as i32)) != 0) { F32(0x437f0000) } else { F32(c.m.r32(fp.wrapping_add(800))) }) }); let a_ = fp.wrapping_add(800); c.m.w32(a_, (v_).0); };
        { let v_ = (if (((rt::flt(F32(c.m.r32((fp.wrapping_add(800)).wrapping_add(4))), F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(0x437f0000), F32(c.m.r32((fp.wrapping_add(800)).wrapping_add(4)))) as i32)) != 0) { F32(0x437f0000) } else { F32(c.m.r32((fp.wrapping_add(800)).wrapping_add(4))) }) }); let a_ = (fp.wrapping_add(800)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = (if (((rt::flt(F32(c.m.r32((fp.wrapping_add(800)).wrapping_add(8))), F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(0x437f0000), F32(c.m.r32((fp.wrapping_add(800)).wrapping_add(8)))) as i32)) != 0) { F32(0x437f0000) } else { F32(c.m.r32((fp.wrapping_add(800)).wrapping_add(8))) }) }); let a_ = (fp.wrapping_add(800)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = (if (((rt::flt(F32(c.m.r32((fp.wrapping_add(800)).wrapping_add(12))), F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(0x437f0000), F32(c.m.r32((fp.wrapping_add(800)).wrapping_add(12)))) as i32)) != 0) { F32(0x437f0000) } else { F32(c.m.r32((fp.wrapping_add(800)).wrapping_add(12))) }) }); let a_ = (fp.wrapping_add(800)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let _ = F::Vu0Cur_ProjectPoints(c, fp.wrapping_add(816), fp.wrapping_add(128), 4i32); };
        {
            l_i_5 = 0i32;
            'l3: loop {
                if !(((((l_i_5) < (2i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    F::ClipVtx_Set(c, (fp.wrapping_add(320)).wrapping_add((((0i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(128)).wrapping_add((((l_i_5) as u32)).wrapping_mul(16)), (fp.wrapping_add(256)).wrapping_add((((l_i_5) as u32)).wrapping_mul(16)), fp.wrapping_add(800));
                    F::ClipVtx_Set(c, (fp.wrapping_add(320)).wrapping_add((((1i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(128)).wrapping_add(((((l_i_5).wrapping_add(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(256)).wrapping_add(((((l_i_5).wrapping_add(1i32)) as u32)).wrapping_mul(16)), fp.wrapping_add(800));
                    F::ClipVtx_Set(c, (fp.wrapping_add(320)).wrapping_add((((2i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(128)).wrapping_add(((((l_i_5).wrapping_add(2i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(256)).wrapping_add(((((l_i_5).wrapping_add(2i32)) as u32)).wrapping_mul(16)), fp.wrapping_add(800));
                    { let a1_ = (({ let a_ = ((l_def_3).wrapping_add(((((c.m.r16((p_ray_2).wrapping_add(108)) as i16)) as u32)).wrapping_mul(120))).wrapping_add(114); (c.m.r8(a_) as i8) }) as i32); let a6_ = c.m.r64((p_w_1).wrapping_add(48)); F::EftGfx_DrawPolyAvgZFront(c, fp.wrapping_add(320), a1_, 0i32, 0i32, 0i32, 0i32, a6_, 0i32) };
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftRays_DrawRay", &PROBE_N, &[], true); }
    r_
}

pub fn EftRays_Step(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRays_Step", &PROBE_N, &[(p_w_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_t_2: F32 = F32(0);
    let mut l_def_3: u32 = 0;
    let mut l_ray_4: u32 = 0;
    let mut l_rd_5: u32 = 0;
    let r_ = 'f: {
        l_t_2 = F32(0x00000000);
        l_def_3 = c.m.r32(((p_w_1).wrapping_add(16)).wrapping_add(4));
        l_ray_4 = F::List_GetHead(c, (p_w_1).wrapping_add(4));
        'l1: while (((((l_ray_4) != (((0i32) as u32))) as i32)) != 0) {
            l_rd_5 = (l_def_3).wrapping_add(((((c.m.r16((l_ray_4).wrapping_add(108)) as i16)) as u32)).wrapping_mul(120));
            { let a_ = (l_ray_4).wrapping_add(110); let o_ = (c.m.r16(a_) as i16); let n_ = (o_).wrapping_sub(1); c.m.w16(a_, (n_) as u16); };
            if ((((((((c.m.r16((l_ray_4).wrapping_add(110)) as i16)) as i32)) <= (0i32)) as i32)) != 0) {
                { let v_ = ((0i32) as i16); let a_ = (l_ray_4).wrapping_add(110); c.m.w16(a_, (v_) as u16); };
                { let r_ = 1i32; let a_ = (l_ray_4).wrapping_add(112); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
            if ((((!(((((c.m.r32((l_ray_4).wrapping_add(112)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
                l_ray_4 = F::List_GetNext(c, l_ray_4);
                continue 'l1;
            }
            if (((((c.m.r32((l_def_3).wrapping_add(1204)) as i32)) & (1i32))) != 0) {
                F::EftRays_LerpKey(c, l_ray_4, l_def_3);
            }
            if (((((c.m.r32((l_def_3).wrapping_add(1204)) as i32)) & (2i32))) != 0) {
                if (((c.m.r32((l_ray_4).wrapping_add(64)) as i32)) != 0) {
                    { let r_ = F32(0x3f800000); let a_ = (l_ray_4).wrapping_add(48); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                } else {
                    { let r_ = F32(0x3f800000); let a_ = (l_ray_4).wrapping_add(48); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                }
                if (((rt::fge(F32(c.m.r32((l_ray_4).wrapping_add(48))), F32(c.m.r32((l_ray_4).wrapping_add(52)))) as i32)) != 0) {
                    { let v_ = F32(c.m.r32((l_ray_4).wrapping_add(52))); let a_ = (l_ray_4).wrapping_add(48); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32((l_rd_5).wrapping_add(48))), rt::fmul(F32(c.m.r32(((l_rd_5).wrapping_add(48)).wrapping_add(4))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = (l_ray_4).wrapping_add(56); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32(((l_rd_5).wrapping_add(48)).wrapping_add(8))), rt::fmul(F32(c.m.r32(((l_rd_5).wrapping_add(48)).wrapping_add(12))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = (l_ray_4).wrapping_add(60); c.m.w32(a_, (v_).0); };
                    { let r_ = 1i32; let a_ = (l_ray_4).wrapping_add(64); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                } else {
                    if (((rt::fle(F32(c.m.r32((l_ray_4).wrapping_add(48))), F32(0x00000000)) as i32)) != 0) {
                        { let v_ = F32(0x00000000); let a_ = (l_ray_4).wrapping_add(48); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(F32(c.m.r32((l_rd_5).wrapping_add(48))), rt::fmul(F32(c.m.r32(((l_rd_5).wrapping_add(48)).wrapping_add(4))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = (l_ray_4).wrapping_add(56); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(F32(c.m.r32(((l_rd_5).wrapping_add(48)).wrapping_add(8))), rt::fmul(F32(c.m.r32(((l_rd_5).wrapping_add(48)).wrapping_add(12))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = (l_ray_4).wrapping_add(60); c.m.w32(a_, (v_).0); };
                        { let r_ = 1i32; let a_ = (l_ray_4).wrapping_add(64); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                }
                l_t_2 = rt::fdiv(F32(c.m.r32((l_ray_4).wrapping_add(48))), F32(c.m.r32((l_ray_4).wrapping_add(52))));
                { let v_ = rt::fadd(F32(c.m.r32((l_ray_4).wrapping_add(56))), rt::fmul(rt::fsub(F32(c.m.r32((l_ray_4).wrapping_add(60))), F32(c.m.r32((l_ray_4).wrapping_add(56)))), l_t_2)); let a_ = (l_ray_4).wrapping_add(44); c.m.w32(a_, (v_).0); };
            }
            if (((((c.m.r32((l_def_3).wrapping_add(1204)) as i32)) & (4i32))) != 0) {
                if (((c.m.r32((l_ray_4).wrapping_add(104)) as i32)) != 0) {
                    { let r_ = F32(0x3f800000); let a_ = (l_ray_4).wrapping_add(96); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                } else {
                    { let r_ = F32(0x3f800000); let a_ = (l_ray_4).wrapping_add(96); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                }
                if (((rt::fge(F32(c.m.r32((l_ray_4).wrapping_add(96))), F32(c.m.r32((l_ray_4).wrapping_add(100)))) as i32)) != 0) {
                    { let v_ = F32(c.m.r32((l_ray_4).wrapping_add(100))); let a_ = (l_ray_4).wrapping_add(96); c.m.w32(a_, (v_).0); };
                    { let r_ = 1i32; let a_ = (l_ray_4).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                } else {
                    if (((rt::fle(F32(c.m.r32((l_ray_4).wrapping_add(96))), F32(0x00000000)) as i32)) != 0) {
                        { let v_ = F32(0x00000000); let a_ = (l_ray_4).wrapping_add(96); c.m.w32(a_, (v_).0); };
                        { let r_ = 1i32; let a_ = (l_ray_4).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                }
                { let v_ = rt::fsub(F32(c.m.r32((l_rd_5).wrapping_add(80))), F32(c.m.r32(((l_rd_5).wrapping_add(68)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fsub(F32(c.m.r32(((l_rd_5).wrapping_add(80)).wrapping_add(4))), F32(c.m.r32(((l_rd_5).wrapping_add(68)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fsub(F32(c.m.r32(((l_rd_5).wrapping_add(80)).wrapping_add(8))), F32(c.m.r32(((l_rd_5).wrapping_add(68)).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                l_t_2 = rt::fdiv(F32(c.m.r32((l_ray_4).wrapping_add(96))), F32(c.m.r32((l_ray_4).wrapping_add(100))));
                { let v_ = rt::fadd(F32(c.m.r32(((l_rd_5).wrapping_add(68)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_2)); let a_ = ((l_ray_4).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32(((l_rd_5).wrapping_add(68)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_2)); let a_ = ((l_ray_4).wrapping_add(80)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32(((l_rd_5).wrapping_add(68)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_t_2)); let a_ = ((l_ray_4).wrapping_add(80)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            }
            { let r_ = F32(c.m.r32((l_rd_5).wrapping_add(100))); let a_ = (l_ray_4).wrapping_add(40); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let v_ = { let a0_ = F32(c.m.r32((l_ray_4).wrapping_add(40))); F::EftMath_WrapAngle(c, a0_) }; let a_ = (l_ray_4).wrapping_add(40); c.m.w32(a_, (v_).0); };
            l_ray_4 = F::List_GetNext(c, l_ray_4);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftRays_Step", &PROBE_N, &[], true); }
    r_
}

pub fn EftRays_AllocRay(c: &mut Ctx, mut p_list_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRays_AllocRay", &PROBE_N, &[(p_list_1) as u32], false); }
    let mut l_ray_2: u32 = 0;
    let r_ = 'f: {
    l_ray_2 = { let a0_ = (c.m.r32(sym::gEftRays)).wrapping_add(2560); F::List_PopFront(c, a0_) };
    if (((((l_ray_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    F::List_PushBack(c, p_list_1, l_ray_2);
    break 'f l_ray_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRays_AllocRay", &PROBE_N, &[], true); }
    r_
}

pub fn EftRays_FreeRay(c: &mut Ctx, mut p_list_1: u32, mut p_ray_2: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRays_FreeRay", &PROBE_N, &[(p_list_1) as u32, (p_ray_2) as u32], false); }
    let mut l_next_3: u32 = 0;
    let r_ = 'f: {
    l_next_3 = ((0i32) as u32);
    if (((((p_ray_2) != (((0i32) as u32))) as i32)) != 0) {
        l_next_3 = F::List_GetNext(c, p_ray_2);
        { let _ = F::List_Remove(c, p_list_1, p_ray_2); };
        { let a0_ = (c.m.r32(sym::gEftRays)).wrapping_add(2560); F::List_PushBack(c, a0_, p_ray_2) };
    }
    break 'f l_next_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRays_FreeRay", &PROBE_N, &[], true); }
    r_
}

pub fn EftRays_GetTex(c: &mut Ctx, mut p_res_1: u32, mut p_n_2: i32) -> u64 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRays_GetTex", &PROBE_N, &[(p_res_1) as u32, (p_n_2) as u32], false); }
    let r_ = 'f: {
    if ((((!((((c.m.r32((p_res_1).wrapping_add(68))) & ((1u32).wrapping_shl((p_n_2) as u32)))) != 0)) as i32)) != 0) {
        { let v_ = F::EftVram_AddTex(c, (p_res_1).wrapping_add((((p_n_2) as u32)).wrapping_mul(16)), 1i32, 0i32); let a_ = (p_res_1).wrapping_add((((p_n_2) as u32)).wrapping_mul(16)); c.m.w64(a_, v_); };
        { let r_ = (1u32).wrapping_shl((p_n_2) as u32); let a_ = (p_res_1).wrapping_add(68); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    }
    break 'f c.m.r64((p_res_1).wrapping_add((((p_n_2) as u32)).wrapping_mul(16)));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRays_GetTex", &PROBE_N, &[], true); }
    r_
}

pub fn EftRays_SetKey(c: &mut Ctx, mut p_ray_1: u32, mut p_def_2: u32, mut p_key_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRays_SetKey", &PROBE_N, &[(p_ray_1) as u32, (p_def_2) as u32, (p_key_3) as u32], false); }
    let mut l_anim_4: u32 = 0;
    let mut l_rect_5: u32 = 0;
    let mut l_k_6: u32 = 0;
    let mut l_rd_7: u32 = 0;
    let r_ = 'f: {
    l_anim_4 = (p_ray_1).wrapping_add(24);
    l_rect_5 = (p_ray_1).wrapping_add(8);
    { let v_ = F32(0x00000000); let a_ = l_anim_4; c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul({ let a_ = ((p_def_2).wrapping_add(((((c.m.r16((p_ray_1).wrapping_add(108)) as i16)) as u32)).wrapping_mul(120))).wrapping_add(104); F32(c.m.r32(a_)) }, F32(0x41f00000)); let a_ = (l_anim_4).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(F32(c.m.r32((l_anim_4).wrapping_add(4))), { let a_ = ((p_def_2).wrapping_add(((((c.m.r16((p_ray_1).wrapping_add(108)) as i16)) as u32)).wrapping_mul(120))).wrapping_add(108); F32(c.m.r32(a_)) }); let a_ = (l_anim_4).wrapping_add(8); c.m.w32(a_, (v_).0); };
    l_k_6 = ((p_def_2).wrapping_add(((((c.m.r16((p_ray_1).wrapping_add(108)) as i16)) as u32)).wrapping_mul(120))).wrapping_add((((p_key_3) as u32)).wrapping_mul(16));
    { let s_ = (l_k_6).wrapping_add(12); let d_ = l_rect_5; rt::memcpy(c, d_, s_, 4); };
    { let v_ = F32(c.m.r32((l_k_6).wrapping_add(4))); let a_ = (l_rect_5).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(l_k_6)); let a_ = (l_rect_5).wrapping_add(8); c.m.w32(a_, (v_).0); };
    l_rd_7 = (p_def_2).wrapping_add(((((c.m.r16((p_ray_1).wrapping_add(108)) as i16)) as u32)).wrapping_mul(120));
    { let v_ = F32(c.m.r32((l_rd_7).wrapping_add((((2i32) as u32)).wrapping_mul(16)))); let a_ = (l_rect_5).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRays_SetKey", &PROBE_N, &[], true); }
    r_
}

pub fn EftRays_LerpKey(c: &mut Ctx, mut p_ray_1: u32, mut p_def_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRays_LerpKey", &PROBE_N, &[(p_ray_1) as u32, (p_def_2) as u32], false); }
    let mut l_anim_3: u32 = 0;
    let mut l_rays_4: u32 = 0;
    let mut l_rect_5: u32 = 0;
    let mut l_d_6: F32 = F32(0);
    let mut l_di_7: i32 = 0;
    let mut l_a_8: u32 = 0;
    let mut l_b_9: u32 = 0;
    let mut l_ka_10: i32 = 0;
    let mut l_kb_11: i32 = 0;
    let mut l_t_12: F32 = F32(0);
    let r_ = 'f: {
    l_anim_3 = (p_ray_1).wrapping_add(24);
    l_rays_4 = p_def_2;
    l_rect_5 = (p_ray_1).wrapping_add(8);
    { let r_ = F32(0x3f800000); let a_ = l_anim_3; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    if (((rt::flt(F32(c.m.r32((l_anim_3).wrapping_add(4))), F32(c.m.r32(l_anim_3))) as i32)) != 0) {
        { let v_ = F32(c.m.r32((l_anim_3).wrapping_add(4))); let a_ = l_anim_3; c.m.w32(a_, (v_).0); };
    }
    if (((rt::flt(F32(c.m.r32(l_anim_3)), F32(c.m.r32((l_anim_3).wrapping_add(8)))) as i32)) != 0) {
        l_t_12 = rt::fdiv(F32(c.m.r32(l_anim_3)), F32(c.m.r32((l_anim_3).wrapping_add(8))));
        l_ka_10 = 0i32;
        l_kb_11 = 1i32;
    } else {
        l_ka_10 = 1i32;
        l_t_12 = rt::fsub(F32(c.m.r32(l_anim_3)), F32(c.m.r32((l_anim_3).wrapping_add(8))));
        l_kb_11 = 2i32;
        { let r_ = rt::fsub(F32(c.m.r32((l_anim_3).wrapping_add(4))), F32(c.m.r32((l_anim_3).wrapping_add(8)))); let n_ = rt::fdiv(l_t_12, r_); l_t_12 = n_; };
    }
    l_a_8 = ((l_rays_4).wrapping_add(((((c.m.r16((p_ray_1).wrapping_add(108)) as i16)) as u32)).wrapping_mul(120))).wrapping_add((((l_ka_10) as u32)).wrapping_mul(16));
    l_b_9 = ((l_rays_4).wrapping_add(((((c.m.r16((p_ray_1).wrapping_add(108)) as i16)) as u32)).wrapping_mul(120))).wrapping_add((((l_kb_11) as u32)).wrapping_mul(16));
    l_di_7 = (((c.m.r8((l_b_9).wrapping_add(12))) as i32)).wrapping_sub(((c.m.r8((l_a_8).wrapping_add(12))) as i32));
    { let v_ = (((((c.m.r8((l_a_8).wrapping_add(12))) as u32)).wrapping_add(rt::f2u(rt::fmul(rt::i2f(l_di_7), l_t_12)))) as u8); let a_ = l_rect_5; c.m.w8(a_, v_); };
    l_di_7 = (((c.m.r8(((l_b_9).wrapping_add(12)).wrapping_add(1))) as i32)).wrapping_sub(((c.m.r8(((l_a_8).wrapping_add(12)).wrapping_add(1))) as i32));
    { let v_ = (((((c.m.r8(((l_a_8).wrapping_add(12)).wrapping_add(1))) as u32)).wrapping_add(rt::f2u(rt::fmul(rt::i2f(l_di_7), l_t_12)))) as u8); let a_ = (l_rect_5).wrapping_add(1); c.m.w8(a_, v_); };
    l_di_7 = (((c.m.r8(((l_b_9).wrapping_add(12)).wrapping_add(2))) as i32)).wrapping_sub(((c.m.r8(((l_a_8).wrapping_add(12)).wrapping_add(2))) as i32));
    { let v_ = (((((c.m.r8(((l_a_8).wrapping_add(12)).wrapping_add(2))) as u32)).wrapping_add(rt::f2u(rt::fmul(rt::i2f(l_di_7), l_t_12)))) as u8); let a_ = (l_rect_5).wrapping_add(2); c.m.w8(a_, v_); };
    l_d_6 = rt::fsub(F32(c.m.r32((l_b_9).wrapping_add(4))), F32(c.m.r32((l_a_8).wrapping_add(4))));
    { let v_ = rt::fadd(F32(c.m.r32((l_a_8).wrapping_add(4))), rt::fmul(l_d_6, l_t_12)); let a_ = (l_rect_5).wrapping_add(4); c.m.w32(a_, (v_).0); };
    l_d_6 = rt::fsub(F32(c.m.r32(l_b_9)), F32(c.m.r32(l_a_8)));
    { let v_ = rt::fadd(F32(c.m.r32(l_a_8)), rt::fmul(l_d_6, l_t_12)); let a_ = (l_rect_5).wrapping_add(8); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRays_LerpKey", &PROBE_N, &[], true); }
    r_
}

pub fn EftRays_IsTask(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRays_IsTask", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((((c.m.r8(p_task_1)) as i32)) & (1i32))) as u8)) != 0) {
        break 'f 0i32;
    }
    break 'f ((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftRays_Update)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRays_IsTask", &PROBE_N, &[], true); }
    r_
}
