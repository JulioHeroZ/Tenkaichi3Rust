#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EftKiObj_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiObj_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiObj_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiObj_PostUpdateCb(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiObj_PostUpdateCb", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::EftKiObj_PostUpdate(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiObj_PostUpdateCb", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiObj_DrawCb(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiObj_DrawCb", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::EftKiObj_Draw(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiObj_DrawCb", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiObjMgr_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiObjMgr_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_mgr_3: u32 = 0;
    let mut l_res_4: u32 = 0;
    let mut l_list_5: u32 = 0;
    let r_ = 'f: {
    l_mgr_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let v_ = ((0i32) as u32); let a_ = l_mgr_3; c.m.w32(a_, v_); };
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((88u32) as i32)) }; let a_ = l_mgr_3; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(l_mgr_3); F::memset(c, a0_, 0i32, 88u32) }; };
    l_res_4 = c.m.r32(l_mgr_3);
    { let v_ = { let a0_ = (c.m.r32((p_arg_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::BtlScene_GetCharPackEntry(c, a0_, 3i32) }; let a_ = (l_res_4).wrapping_add(84); c.m.w32(a_, v_); };
    { let v_ = { let a0_ = c.m.r32((l_res_4).wrapping_add(84)); F::BtlScene_GetPackEntry(c, a0_, 1i32) }; let a_ = (l_res_4).wrapping_add(80); c.m.w32(a_, v_); };
    { let a1_ = { let a0_ = c.m.r32((l_res_4).wrapping_add(84)); F::BtlScene_GetPackEntry(c, a0_, 2i32) }; F::EftTexSet_Load4(c, (l_res_4).wrapping_add(8), a1_) };
    l_list_5 = F::BtlTask_CreateChildList(c, p_task_1, ((((10i32) as i64 as u64)) as u32), ((((((480u32) as i32)) as i64 as u64)) as u32));
    { let a0_ = (c.m.r32((p_arg_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a1_ = (c.m.r32((p_arg_2).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); F::EftChar_SetList(c, a0_, a1_, l_list_5) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiObjMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiObjMgr_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiObjMgr_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_mgr_2: u32 = 0;
    let r_ = 'f: {
    l_mgr_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((c.m.r32(l_mgr_2)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(l_mgr_2); F::BtlPool_Free(c, a0_, a1_) };
        { let v_ = ((0i32) as u32); let a_ = l_mgr_2; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiObjMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiObjMgr_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiObjMgr_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_mgr_2: u32 = 0;
    let r_ = 'f: {
    l_mgr_2 = c.m.r32((p_task_1).wrapping_add(56));
    { let v_ = 0i32; let a_ = (c.m.r32(l_mgr_2)).wrapping_add(76); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiObjMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiObj_Create(c: &mut Ctx, p_arg_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiObj_Create", &PROBE_N, &[(p_arg_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_list_2: u32 = 0;
    rt::memcpy(c, fp, p_arg_1, 80);
    let r_ = 'f: {
        l_list_2 = { let a0_ = (((c.m.r16((fp).wrapping_add(18)) as i16)) as i32); F::EftChar_GetList(c, a0_, 1i32) };
        if (((((l_list_2) != (((0i32) as u32))) as i32)) != 0) {
            { let _ = F::BtlTaskList_AddTail(c, l_list_2, sym::D_002C3E18, fp); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftKiObj_Create", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiObj_SetBlock(c: &mut Ctx, mut p_task_1: u32, mut p_src_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiObj_SetBlock", &PROBE_N, &[(p_task_1) as u32, (p_src_2) as u32], false); }
    let r_ = 'f: {
    if (((((((((((((p_task_1) != (((0i32) as u32))) as i32)) != 0) && ((((!((((((c.m.r8(p_task_1)) as i32)) & (1i32))) != 0)) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftKiObj_Update)) as i32)) != 0)) as i32)) != 0) {
        { let s_ = p_src_2; let d_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(144); rt::memcpy(c, d_, s_, 64); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiObj_SetBlock", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiObj_Step(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiObj_Step", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_period_3: i32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    { let a_ = (l_w_2).wrapping_add(244); let o_ = (c.m.r16(a_) as i16); let n_ = (o_).wrapping_add(1); c.m.w16(a_, (n_) as u16); };
    if ((((((c.m.r32((l_w_2).wrapping_add(300)) as i32)) == ((1i32).wrapping_neg())) as i32)) != 0) {
        { let r_ = 8i32; let a_ = (l_w_2).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(208)) as i32)) & (16i32))) != 0)) as i32)) != 0) {
        if (((((c.m.r32((l_w_2).wrapping_add(208)) as i32)) & (2i32))) != 0) {
            F::Vec4_Copy(c, (l_w_2).wrapping_add(112), (l_w_2).wrapping_add(96));
        }
        { let r_ = 2i32; let a_ = (l_w_2).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        F::Vec3_Add(c, (l_w_2).wrapping_add(96), (l_w_2).wrapping_add(96), (l_w_2).wrapping_add(128));
        { let r_ = F32(c.m.r32((l_w_2).wrapping_add(240))); let a_ = ((l_w_2).wrapping_add(128)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
    l_period_3 = 360i32;
    { let v_ = { let a0_ = rt::fdiv(rt::fmul(rt::i2f(rt::rem_i32(((((c.m.r16((l_w_2).wrapping_add(244)) as i16)) as i32)).wrapping_mul(rt::f2i(F32(c.m.r32((l_w_2).wrapping_add(236))))), l_period_3)), F32(0x40490fda)), F32(0x43340000)); F::EftMath_WrapAngle(c, a0_) }; let a_ = (l_w_2).wrapping_add(232); c.m.w32(a_, (v_).0); };
    if ((((((((c.m.r16((l_w_2).wrapping_add(468)) as i16)) as i32)) <= (0i32)) as i32)) != 0) {
        { let r_ = 8i32; let a_ = (l_w_2).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    F::EftKiObj_StepFrags(c, p_task_1);
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiObj_Step", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiObj_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiObj_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((c.m.r32((l_w_2).wrapping_add(208)) as i32)) & (64i32))) != 0) {
        if ((((!(({ let a0_ = (((c.m.r16((l_w_2).wrapping_add(18)) as i16)) as i32); F::BtlScene_IsEffectHidden(c, a0_, 0i32) }) != 0)) as i32)) != 0) {
            { let a0_ = (c.m.r32((l_w_2).wrapping_add(300)) as i32); F::EftObj_SetVisible(c, a0_, 1i32) };
        } else {
            { let a0_ = (c.m.r32((l_w_2).wrapping_add(300)) as i32); F::EftObj_SetVisible(c, a0_, 0i32) };
        }
    } else {
        { let a0_ = (c.m.r32((l_w_2).wrapping_add(300)) as i32); F::EftObj_SetVisible(c, a0_, 0i32) };
    }
    if ((((((c.m.r32((l_w_2).wrapping_add(300)) as i32)) != ((1i32).wrapping_neg())) as i32)) != 0) {
        if ((((c.m.r32((p_task_1).wrapping_add(4))) & (((63i32) as u32)))) != 0) {
            { let v_ = 0i32; let a_ = (l_w_2).wrapping_add(212); c.m.w32(a_, (v_) as u32); };
            if ((((!(((((c.m.r32((l_w_2).wrapping_add(208)) as i32)) & (16i32))) != 0)) as i32)) != 0) {
                F::Vec3_Copy(c, (l_w_2).wrapping_add(96), (p_task_1).wrapping_add(16));
                F::EftKiObj_Break(c, p_task_1, (l_w_2).wrapping_add(96));
                { let r_ = 16i32; let a_ = (l_w_2).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = (!(64i32)); let a_ = (l_w_2).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = (!(128i32)); let a_ = (l_w_2).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            }
        } else {
            if ((((c.m.r32((p_task_1).wrapping_add(4))) & (((192i32) as u32)))) != 0) {
                if ((((!(((((c.m.r32((l_w_2).wrapping_add(208)) as i32)) & (4i32))) != 0)) as i32)) != 0) {
                    F::EftKiObj_Turn(c, p_task_1, l_w_2);
                    { let v_ = F32(c.m.r32((p_task_1).wrapping_add(16))); let a_ = (l_w_2).wrapping_add(96); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32(((p_task_1).wrapping_add(16)).wrapping_add(4))); let a_ = ((l_w_2).wrapping_add(96)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32(((p_task_1).wrapping_add(16)).wrapping_add(8))); let a_ = ((l_w_2).wrapping_add(96)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let r_ = 32i32; let a_ = (l_w_2).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let r_ = (((!(192i32))) as u32); let a_ = (p_task_1).wrapping_add(4); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiObj_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiObj_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiObj_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(144) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 144);
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        if ((((((c.m.r32((l_w_2).wrapping_add(300)) as i32)) != ((1i32).wrapping_neg())) as i32)) != 0) {
            F::Mtx_StoreIdentity(c, fp);
            F::Mtx_StoreIdentity(c, fp.wrapping_add(64));
            { let v_ = F32(c.m.r32((l_w_2).wrapping_add(224))); let a_ = ((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32((l_w_2).wrapping_add(224))); let a_ = ((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32((l_w_2).wrapping_add(224))); let a_ = ((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            F::Mtx_Mul(c, fp, fp, fp.wrapping_add(64));
            { let a2_ = { let a0_ = rt::fneg(F32(c.m.r32((l_w_2).wrapping_add(232)))); F::EftMath_WrapAngle(c, a0_) }; F::Mtx_RotateX(c, fp, fp, a2_) };
            { let a1_ = F32(c.m.r32((l_w_2).wrapping_add(96))); let a2_ = F32(c.m.r32(((l_w_2).wrapping_add(96)).wrapping_add(4))); let a3_ = F32(c.m.r32(((l_w_2).wrapping_add(96)).wrapping_add(8))); F::Vec4_Set(c, fp.wrapping_add(128), a1_, a2_, a3_, F32(0x3f800000)) };
            F::Mtx_SetTrans(c, fp, fp.wrapping_add(128));
            { let a0_ = (c.m.r32((l_w_2).wrapping_add(300)) as i32); F::EftObj_SetMtx(c, a0_, fp) };
            if ((((!(({ let a0_ = (((c.m.r16((l_w_2).wrapping_add(16)) as i16)) as i32); F::BtlScene_IsEffectHidden(c, a0_, 0i32) }) != 0)) as i32)) != 0) {
                F::EftKiObj_DrawFrags(c, p_task_1);
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftKiObj_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiObj_Break(c: &mut Ctx, mut p_task_1: u32, mut p_pos_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiObj_Break", &PROBE_N, &[(p_task_1) as u32, (p_pos_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let mut l_f_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    l_f_4 = (l_w_3).wrapping_add(304);
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((l_i_5) < (5i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                F::Vec4_Copy(c, l_f_4, p_pos_2);
                { let v_ = rt::fsub(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40000000)), F32(0x3f800000)); let a_ = (l_f_4).wrapping_add(16); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fneg(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff))); let a_ = ((l_f_4).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fsub(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40000000)), F32(0x3f800000)); let a_ = ((l_f_4).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = ((l_f_4).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                F::Vec3_Normalize(c, (l_f_4).wrapping_add(16), (l_f_4).wrapping_add(16));
                F::Vec3_Scale(c, (l_f_4).wrapping_add(16), (l_f_4).wrapping_add(16), F32(0x40800000));
                { let r_ = F32(0x3fa66666); let a_ = ((l_f_4).wrapping_add(16)).wrapping_add(4); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let v_ = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3f000000)), F32(0x3f000000)); let a_ = ((l_f_4).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                { let o_ = l_f_4; let n_ = (o_).wrapping_add(32); l_f_4 = n_; };
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
    { let r_ = 256i32; let a_ = (l_w_3).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiObj_Break", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiObj_StepFrags(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiObj_StepFrags", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_f_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((c.m.r32((l_w_2).wrapping_add(208)) as i32)) & (256i32))) != 0) {
        l_f_3 = (l_w_2).wrapping_add(304);
        { let a_ = (l_w_2).wrapping_add(468); let o_ = (c.m.r16(a_) as i16); let n_ = (o_).wrapping_sub(1); c.m.w16(a_, (n_) as u16); };
        if ((((((((c.m.r16((l_w_2).wrapping_add(468)) as i16)) as i32)) <= (0i32)) as i32)) != 0) {
            { let v_ = ((0i32) as i16); let a_ = (l_w_2).wrapping_add(468); c.m.w16(a_, (v_) as u16); };
            { let r_ = (!(256i32)); let a_ = (l_w_2).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        } else {
            {
                l_i_4 = 0i32;
                'l1: loop {
                    if !(((((l_i_4) < (5i32)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        F::Vec3_Add(c, l_f_3, l_f_3, (l_f_3).wrapping_add(16));
                        { let r_ = F32(c.m.r32((l_w_2).wrapping_add(240))); let a_ = ((l_f_3).wrapping_add(16)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        { let o_ = l_f_3; let n_ = (o_).wrapping_add(32); l_f_3 = n_; };
                    }
                    { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiObj_StepFrags", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiObj_DrawFrags(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiObj_DrawFrags", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_f_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_f_3 = (l_w_2).wrapping_add(304);
    if (((((c.m.r32((l_w_2).wrapping_add(208)) as i32)) & (256i32))) != 0) {
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < (5i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a4_ = F32(c.m.r32(l_f_3)); let a5_ = F32(c.m.r32((l_f_3).wrapping_add(4))); let a6_ = F32(c.m.r32((l_f_3).wrapping_add(8))); let a11_ = F32(c.m.r32((l_w_2).wrapping_add(232))); let a17_ = rt::f2u(rt::fmul(rt::fmul(rt::fmul(F32(c.m.r32((l_w_2).wrapping_add(224))), F32(0x3ecccccc)), F32(0x45800000)), F32(c.m.r32(((l_f_3).wrapping_add(16)).wrapping_add(12))))); let a20_ = c.m.r32((l_w_2).wrapping_add(464)); F::EftSpr_DrawRot(c, ((128i32) as u8), ((128i32) as u8), ((128i32) as u8), ((128i32) as u8), a4_, a5_, a6_, F32(0x00000000), F32(0x00000000), F32(0x3f800000), F32(0x3f800000), a11_, 0i32, 0i32, ((((64i32) as i64 as u64)) as u32), ((((64i32) as i64 as u64)) as u32), 0i32, a17_, 0i32, 0i32, a20_) };
                }
                { { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; }; { let o_ = l_f_3; let n_ = (o_).wrapping_add(32); l_f_3 = n_; }; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiObj_DrawFrags", &PROBE_N, &[], true); }
    r_
}

pub fn EftChain_SetTex(c: &mut Ctx, mut p_w_1: u32, mut p_set_2: u32, mut p_a_3: i32, mut p_b_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChain_SetTex", &PROBE_N, &[(p_w_1) as u32, (p_set_2) as u32, (p_a_3) as u32, (p_b_4) as u32], false); }
    let r_ = 'f: {
    { let s_ = (p_set_2).wrapping_add((((p_a_3) as u32)).wrapping_mul(16)); let d_ = (p_w_1).wrapping_add(3808); rt::memcpy(c, d_, s_, 16); };
    { let s_ = (p_set_2).wrapping_add((((p_b_4) as u32)).wrapping_mul(16)); let d_ = (p_w_1).wrapping_add(3824); rt::memcpy(c, d_, s_, 16); };
    { let v_ = p_b_4; let a_ = (p_w_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChain_SetTex", &PROBE_N, &[], true); }
    r_
}

pub fn EftChain_BuildTex(c: &mut Ctx, mut p_dst_1: u32, mut p_src_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChain_BuildTex", &PROBE_N, &[(p_dst_1) as u32, (p_src_2) as u32], false); }
    let mut l_one_3: i32 = 0;
    let r_ = 'f: {
    l_one_3 = 1i32;
    if ((((!(((({ let a_ = (c.m.r32((p_src_2).wrapping_add(48))).wrapping_add(260); (c.m.r32(a_) as i32) }) & ((l_one_3).wrapping_shl(((c.m.r32((p_src_2).wrapping_add(36)) as i32)) as u32)))) != 0)) as i32)) != 0) {
        { let v_ = F::EftVram_AddImage(c, (p_dst_1).wrapping_add(3808), 1i32, 0i32); let a_ = (p_dst_1).wrapping_add(3840); c.m.w64(a_, v_); };
        { let r_ = (F::EftVram_AddClut(c, (p_dst_1).wrapping_add(3824))).wrapping_shl((37i32) as u32); let a_ = (p_dst_1).wrapping_add(3840); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
        { let v_ = c.m.r64((p_dst_1).wrapping_add(3840)); let a_ = (c.m.r32((p_src_2).wrapping_add(48))).wrapping_add(((((c.m.r32((p_src_2).wrapping_add(36)) as i32)) as u32)).wrapping_mul(16)); c.m.w64(a_, v_); };
        { let r_ = (l_one_3).wrapping_shl(((c.m.r32((p_src_2).wrapping_add(36)) as i32)) as u32); let a_ = (c.m.r32((p_src_2).wrapping_add(48))).wrapping_add(260); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        { let v_ = { let a_ = (c.m.r32((p_src_2).wrapping_add(48))).wrapping_add(((((c.m.r32((p_src_2).wrapping_add(36)) as i32)) as u32)).wrapping_mul(16)); c.m.r64(a_) }; let a_ = (p_dst_1).wrapping_add(3840); c.m.w64(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChain_BuildTex", &PROBE_N, &[], true); }
    r_
}

pub fn EftChain_SetKey(c: &mut Ctx, mut p_out_1: u32, mut p_w_2: u32, mut p_key_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChain_SetKey", &PROBE_N, &[(p_out_1) as u32, (p_w_2) as u32, (p_key_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_keys_4: u32 = 0;
    let mut l_p_5: u32 = 0;
    let mut l_j_6: i32 = 0;
    let r_ = 'f: {
        l_keys_4 = c.m.r32((p_w_2).wrapping_add(56));
        l_p_5 = c.m.r32((p_w_2).wrapping_add(52));
        F::Vec4_Copy(c, p_out_1, (l_keys_4).wrapping_add((((p_key_3) as u32)).wrapping_mul(16)));
        F::Vec4_Copy(c, (p_out_1).wrapping_add(16), ((l_keys_4).wrapping_add(48)).wrapping_add((((p_key_3) as u32)).wrapping_mul(16)));
        F::Vec4_Copy(c, (p_out_1).wrapping_add(32), ((l_keys_4).wrapping_add(96)).wrapping_add((((p_key_3) as u32)).wrapping_mul(16)));
        F::Vec4_Copy(c, (p_out_1).wrapping_add(48), ((l_keys_4).wrapping_add(144)).wrapping_add((((p_key_3) as u32)).wrapping_mul(16)));
        {
            l_j_6 = 0i32;
            'l1: loop {
                if !(((((l_j_6) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = F32(c.m.r32((((l_keys_4).wrapping_add(192)).wrapping_add((((p_key_3) as u32)).wrapping_mul(8))).wrapping_add((((l_j_6) as u32)).wrapping_mul(4)))); let a_ = (fp).wrapping_add((((l_j_6) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((((l_keys_4).wrapping_add(216)).wrapping_add((((p_key_3) as u32)).wrapping_mul(8))).wrapping_add((((l_j_6) as u32)).wrapping_mul(4)))); let a_ = (fp.wrapping_add(16)).wrapping_add((((l_j_6) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((((l_keys_4).wrapping_add(240)).wrapping_add((((p_key_3) as u32)).wrapping_mul(8))).wrapping_add((((l_j_6) as u32)).wrapping_mul(4)))); let a_ = (fp.wrapping_add(32)).wrapping_add((((l_j_6) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_j_6; let n_ = (o_).wrapping_add(1); l_j_6 = n_; };
            }
        }
        { let v_ = F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_out_1).wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_out_1).wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_out_1).wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((p_out_1).wrapping_add(76)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((p_out_1).wrapping_add(76)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((p_out_1).wrapping_add(76)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_keys_4).wrapping_add(264)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x41f00000)); let a_ = (p_out_1).wrapping_add(88); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_p_5).wrapping_add(20)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x41f00000)); let a_ = (p_out_1).wrapping_add(92); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_p_5).wrapping_add(32)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x40490fda)); let a_ = (p_out_1).wrapping_add(96); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_p_5).wrapping_add(44)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x40490fda)); let a_ = (p_out_1).wrapping_add(100); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_p_5).wrapping_add(56)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x40490fda)); let a_ = (p_out_1).wrapping_add(104); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_p_5).wrapping_add(68)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x40490fda)); let a_ = (p_out_1).wrapping_add(108); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_p_5).wrapping_add(80)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x40490fda)); let a_ = (p_out_1).wrapping_add(112); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_p_5).wrapping_add(92)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x40490fda)); let a_ = (p_out_1).wrapping_add(116); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_p_5).wrapping_add(104)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add(120); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_p_5).wrapping_add(116)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add(124); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_p_5).wrapping_add(128)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add(128); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_p_5).wrapping_add(140)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x41f00000)); let a_ = (p_out_1).wrapping_add(132); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_p_5).wrapping_add(152)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x41f00000)); let a_ = (p_out_1).wrapping_add(136); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_p_5).wrapping_add(164)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add(140); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_p_5).wrapping_add(176)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add(144); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_p_5).wrapping_add(188)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add(148); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_p_5).wrapping_add(200)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_p_5).wrapping_add(212)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add(152); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (p_out_1).wrapping_add(156); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_p_5).wrapping_add(224)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x41f00000)); let a_ = (p_out_1).wrapping_add(160); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_p_5).wrapping_add(236)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add(164); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_p_5).wrapping_add(248)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add(168); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_p_5).wrapping_add(260)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add(172); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_p_5).wrapping_add(272)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add(176); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_p_5).wrapping_add(284)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add(180); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_p_5).wrapping_add(296)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add(184); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_p_5).wrapping_add(308)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add(188); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_p_5).wrapping_add(320)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add(192); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_p_5).wrapping_add(332)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add(196); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_p_5).wrapping_add(344)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add(200); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_p_5).wrapping_add(356)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add(204); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_p_5).wrapping_add(368)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x40490fda)); let a_ = (p_out_1).wrapping_add(208); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_p_5).wrapping_add(380)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x40490fda)); let a_ = (p_out_1).wrapping_add(212); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_p_5).wrapping_add(392)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x40490fda)); let a_ = (p_out_1).wrapping_add(216); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_p_5).wrapping_add(404)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x40490fda)); let a_ = (p_out_1).wrapping_add(220); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_p_5).wrapping_add(416)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x40490fda)); let a_ = (p_out_1).wrapping_add(224); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_p_5).wrapping_add(428)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x40490fda)); let a_ = (p_out_1).wrapping_add(228); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_p_5).wrapping_add(440)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x40490fda)); let a_ = (p_out_1).wrapping_add(232); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_p_5).wrapping_add(452)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x40490fda)); let a_ = (p_out_1).wrapping_add(236); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_p_5).wrapping_add(464)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x41f00000)); let a_ = (p_out_1).wrapping_add(240); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_p_5).wrapping_add(476)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x41f00000)); let a_ = (p_out_1).wrapping_add(244); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftChain_SetKey", &PROBE_N, &[], true); }
    r_
}

pub fn EftChain_BlendKeys(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChain_BlendKeys", &PROBE_N, &[(p_w_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_out_2: u32 = 0;
    let mut l_p_3: u32 = 0;
    let mut l_keys_4: u32 = 0;
    let mut l_t_5: F32 = F32(0);
    let mut l_mid_6: F32 = F32(0);
    let mut l_k0_7: i32 = 0;
    let mut l_k1_8: i32 = 0;
    let mut l_j_9: i32 = 0;
    let r_ = 'f: {
        l_out_2 = (p_w_1).wrapping_add(64);
        l_p_3 = c.m.r32((p_w_1).wrapping_add(52));
        l_keys_4 = c.m.r32((p_w_1).wrapping_add(56));
        l_t_5 = F32(c.m.r32((p_w_1).wrapping_add(3752)));
        l_mid_6 = F32(c.m.r32((p_w_1).wrapping_add(3760)));
        if (((rt::flt(l_t_5, l_mid_6) as i32)) != 0) {
            l_t_5 = rt::fdiv(l_t_5, l_mid_6);
            l_k0_7 = 0i32;
            l_k1_8 = 1i32;
        } else {
            l_t_5 = rt::fdiv(rt::fsub(l_t_5, l_mid_6), rt::fsub(F32(c.m.r32((p_w_1).wrapping_add(3756))), l_mid_6));
            l_k0_7 = 1i32;
            l_k1_8 = 2i32;
        }
        F::Vec4_Sub(c, fp, (l_keys_4).wrapping_add((((l_k1_8) as u32)).wrapping_mul(16)), (l_keys_4).wrapping_add((((l_k0_7) as u32)).wrapping_mul(16)));
        F::Vec4_Scale(c, fp, fp, l_t_5);
        F::Vec4_Add(c, l_out_2, (l_keys_4).wrapping_add((((l_k0_7) as u32)).wrapping_mul(16)), fp);
        F::Vec4_Sub(c, fp, ((l_keys_4).wrapping_add(48)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(16)), ((l_keys_4).wrapping_add(48)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(16)));
        F::Vec4_Scale(c, fp, fp, l_t_5);
        F::Vec4_Add(c, (l_out_2).wrapping_add(16), ((l_keys_4).wrapping_add(48)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(16)), fp);
        F::Vec4_Sub(c, fp, ((l_keys_4).wrapping_add(96)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(16)), ((l_keys_4).wrapping_add(96)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(16)));
        F::Vec4_Scale(c, fp, fp, l_t_5);
        F::Vec4_Add(c, (l_out_2).wrapping_add(32), ((l_keys_4).wrapping_add(96)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(16)), fp);
        F::Vec4_Sub(c, fp, ((l_keys_4).wrapping_add(144)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(16)), ((l_keys_4).wrapping_add(144)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(16)));
        F::Vec4_Scale(c, fp, fp, l_t_5);
        F::Vec4_Add(c, (l_out_2).wrapping_add(48), ((l_keys_4).wrapping_add(144)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(16)), fp);
        {
            l_j_9 = 0i32;
            'l1: loop {
                if !(((((l_j_9) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = rt::fsub(F32(c.m.r32((((l_keys_4).wrapping_add(192)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(8))).wrapping_add((((l_j_9) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_keys_4).wrapping_add(192)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(8))).wrapping_add((((l_j_9) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(F32(c.m.r32((((l_keys_4).wrapping_add(216)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(8))).wrapping_add((((l_j_9) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_keys_4).wrapping_add(216)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(8))).wrapping_add((((l_j_9) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(F32(c.m.r32((((l_keys_4).wrapping_add(240)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(8))).wrapping_add((((l_j_9) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_keys_4).wrapping_add(240)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(8))).wrapping_add((((l_j_9) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32((((l_keys_4).wrapping_add(192)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(8))).wrapping_add((((l_j_9) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (fp.wrapping_add(16)).wrapping_add((((l_j_9) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32((((l_keys_4).wrapping_add(216)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(8))).wrapping_add((((l_j_9) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (fp.wrapping_add(32)).wrapping_add((((l_j_9) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32((((l_keys_4).wrapping_add(240)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(8))).wrapping_add((((l_j_9) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (fp.wrapping_add(48)).wrapping_add((((l_j_9) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_j_9; let n_ = (o_).wrapping_add(1); l_j_9 = n_; };
            }
        }
        { let v_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((l_out_2).wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((l_out_2).wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((l_out_2).wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((l_out_2).wrapping_add(76)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((l_out_2).wrapping_add(76)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((l_out_2).wrapping_add(76)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_keys_4).wrapping_add(264)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_keys_4).wrapping_add(264)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_keys_4).wrapping_add(264)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_5)), F32(0x41f00000)); let a_ = (l_out_2).wrapping_add(88); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(20)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(20)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(20)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_5)), F32(0x41f00000)); let a_ = (l_out_2).wrapping_add(92); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(32)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(32)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(44)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(44)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(32)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_5)), F32(0x40490fda)); let a_ = (l_out_2).wrapping_add(96); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(44)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_5)), F32(0x40490fda)); let a_ = (l_out_2).wrapping_add(100); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(56)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(56)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(68)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(68)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(56)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_5)), F32(0x40490fda)); let a_ = (l_out_2).wrapping_add(104); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(68)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_5)), F32(0x40490fda)); let a_ = (l_out_2).wrapping_add(108); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(80)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(80)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(92)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(92)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(80)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_5)), F32(0x40490fda)); let a_ = (l_out_2).wrapping_add(112); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(92)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_5)), F32(0x40490fda)); let a_ = (l_out_2).wrapping_add(116); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(104)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(104)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(116)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(116)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(128)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(128)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(104)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (l_out_2).wrapping_add(120); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(116)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (l_out_2).wrapping_add(124); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(128)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (l_out_2).wrapping_add(128); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(140)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(140)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(152)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(152)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(164)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(164)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(140)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_5)), F32(0x41f00000)); let a_ = (l_out_2).wrapping_add(132); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(152)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_5)), F32(0x41f00000)); let a_ = (l_out_2).wrapping_add(136); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(164)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (l_out_2).wrapping_add(140); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(176)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(176)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(188)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(188)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(176)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (l_out_2).wrapping_add(144); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(188)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (l_out_2).wrapping_add(148); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(200)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(200)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(212)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(212)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(224)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(224)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(200)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(212)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (l_out_2).wrapping_add(152); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (l_out_2).wrapping_add(156); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(224)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_t_5)), F32(0x41f00000)); let a_ = (l_out_2).wrapping_add(160); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(236)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(236)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(248)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(248)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(128)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(128)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(236)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (l_out_2).wrapping_add(164); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(248)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (l_out_2).wrapping_add(168); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(260)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (l_out_2).wrapping_add(172); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(272)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(272)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(284)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(284)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(296)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(296)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(308)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(308)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(272)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (l_out_2).wrapping_add(176); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(284)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (l_out_2).wrapping_add(180); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(296)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (l_out_2).wrapping_add(184); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(308)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((3i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (l_out_2).wrapping_add(188); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(320)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(320)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(332)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(332)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(320)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (l_out_2).wrapping_add(192); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(332)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (l_out_2).wrapping_add(196); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(344)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(344)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(356)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(356)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(344)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (l_out_2).wrapping_add(200); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(356)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (l_out_2).wrapping_add(204); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(368)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(368)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(380)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(380)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(392)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(392)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(404)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(404)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(368)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (l_out_2).wrapping_add(208); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(380)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (l_out_2).wrapping_add(212); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(392)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (l_out_2).wrapping_add(216); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(404)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((3i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (l_out_2).wrapping_add(220); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(416)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(416)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(428)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(428)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(440)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(440)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(452)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(452)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(416)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (l_out_2).wrapping_add(224); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(428)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (l_out_2).wrapping_add(228); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(440)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (l_out_2).wrapping_add(232); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(452)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((3i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = (l_out_2).wrapping_add(236); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(464)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(464)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_p_3).wrapping_add(476)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_3).wrapping_add(476)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(464)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_5)), F32(0x41f00000)); let a_ = (l_out_2).wrapping_add(240); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(476)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_5)), F32(0x41f00000)); let a_ = (l_out_2).wrapping_add(244); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftChain_BlendKeys", &PROBE_N, &[], true); }
    r_
}

pub fn EftChain_AllocNodes(c: &mut Ctx, mut p_ch_1: u32, mut p_count_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChain_AllocNodes", &PROBE_N, &[(p_ch_1) as u32, (p_count_2) as u32], false); }
    let mut l_n_3: i32 = 0;
    let mut l_prev_4: u32 = 0;
    let mut l_tries_5: i32 = 0;
    let mut l_idx_6: i32 = 0;
    let mut l_node_7: u32 = 0;
    let r_ = 'f: {
    l_n_3 = 0i32;
    l_prev_4 = ((0i32) as u32);
    l_tries_5 = 0i32;
    l_idx_6 = { let a_ = (c.m.r32(sym::gEftChain)).wrapping_add(4); (c.m.r32(a_) as i32) };
    {
        'l1: loop {
            if !(((((l_tries_5) < (500i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_node_7 = ((((l_idx_6) as u32)).wrapping_mul(96u32)).wrapping_add({ let a_ = c.m.r32(sym::gEftChain); c.m.r32(a_) });
                if ((((((c.m.r32((l_node_7).wrapping_add(64)) as i32)) == (0i32)) as i32)) != 0) {
                    { let _ = F::memset(c, l_node_7, 0i32, 96u32); };
                    if (((((l_prev_4) == (((0i32) as u32))) as i32)) != 0) {
                        { let v_ = l_node_7; let a_ = (p_ch_1).wrapping_add(96); c.m.w32(a_, v_); };
                    } else {
                        { let v_ = l_node_7; let a_ = (l_prev_4).wrapping_add(84); c.m.w32(a_, v_); };
                    }
                    { let v_ = (l_idx_6).wrapping_add(1i32); let a_ = (c.m.r32(sym::gEftChain)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    l_prev_4 = l_node_7;
                    if ((((({ let a_ = (c.m.r32(sym::gEftChain)).wrapping_add(4); (c.m.r32(a_) as i32) }) >= (500i32)) as i32)) != 0) {
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gEftChain)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    }
                    { let o_ = l_n_3; let n_ = (o_).wrapping_add(1); l_n_3 = n_; };
                    if (((((l_n_3) >= (p_count_2)) as i32)) != 0) {
                        break 'l1;
                    }
                }
                { let o_ = l_idx_6; let n_ = (o_).wrapping_add(1); l_idx_6 = n_; };
                if (((((l_idx_6) >= (500i32)) as i32)) != 0) {
                    l_idx_6 = 0i32;
                }
            }
            { let o_ = l_tries_5; let n_ = (o_).wrapping_add(1); l_tries_5 = n_; };
        }
    }
    { let v_ = l_n_3; let a_ = (p_ch_1).wrapping_add(100); c.m.w32(a_, (v_) as u32); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChain_AllocNodes", &PROBE_N, &[], true); }
    r_
}

pub fn EftChain_FreeNodes(c: &mut Ctx, mut p_ch_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChain_FreeNodes", &PROBE_N, &[(p_ch_1) as u32], false); }
    let mut l_link_2: u32 = 0;
    let mut l_node_3: u32 = 0;
    let r_ = 'f: {
    l_link_2 = (p_ch_1).wrapping_add(96);
    'l1: while (((((c.m.r32(l_link_2)) != (((0i32) as u32))) as i32)) != 0) {
        l_node_3 = c.m.r32(l_link_2);
        l_link_2 = (l_node_3).wrapping_add(84);
        { let v_ = 0i32; let a_ = (l_node_3).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
    }
    { let v_ = ((0i32) as u32); let a_ = (p_ch_1).wrapping_add(96); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChain_FreeNodes", &PROBE_N, &[], true); }
    r_
}

pub fn EftChain_InitStrand(c: &mut Ctx, mut p_task_1: u32, mut p_ch_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChain_InitStrand", &PROBE_N, &[(p_task_1) as u32, (p_ch_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(176) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 176);
    let mut l_w_3: u32 = 0;
    let mut l_key_4: u32 = 0;
    let mut l_n_5: u32 = 0;
    let mut l_link_6: u32 = 0;
    let mut l_i_7: i32 = 0;
    let mut l_r_8: F32 = F32(0);
    let mut l_step_9: F32 = F32(0);
    let r_ = 'f: {
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        l_key_4 = (l_w_3).wrapping_add(64);
        l_i_7 = 0i32;
        l_r_8 = rt::fadd(F32(c.m.r32((l_key_4).wrapping_add(176))), rt::fmul(F32(c.m.r32((l_key_4).wrapping_add(180))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff))));
        l_step_9 = rt::fadd(F32(c.m.r32((l_key_4).wrapping_add(184))), rt::fmul(F32(c.m.r32((l_key_4).wrapping_add(188))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff))));
        l_step_9 = rt::fdiv(rt::fsub(l_step_9, l_r_8), rt::i2f((c.m.r32((p_ch_2).wrapping_add(100)) as i32)));
        { let v_ = l_step_9; let a_ = (p_ch_2).wrapping_add(172); c.m.w32(a_, (v_).0); };
        { let r_ = F32(c.m.r32((l_w_3).wrapping_add(44))); let n_ = rt::fmul(l_r_8, r_); l_r_8 = n_; };
        { let r_ = F32(c.m.r32((l_w_3).wrapping_add(44))); let n_ = rt::fmul(l_step_9, r_); l_step_9 = n_; };
        F::Mtx_StoreIdentity(c, fp.wrapping_add(64));
        l_link_6 = (p_ch_2).wrapping_add(96);
        'l1: while (((((c.m.r32(l_link_6)) != (((0i32) as u32))) as i32)) != 0) {
            l_n_5 = c.m.r32(l_link_6);
            { let r_ = 1i32; let a_ = (l_n_5).wrapping_add(64); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            F::Vec4_Set(c, fp.wrapping_add(128), F32(0x00000000), F32(0x00000000), F32(0x3f800000), F32(0x3f800000));
            { let a2_ = F32(c.m.r32((p_ch_2).wrapping_add(136))); F::Mtx_RotateX(c, fp, fp.wrapping_add(64), a2_) };
            { let a2_ = F32(c.m.r32((p_ch_2).wrapping_add(140))); F::Mtx_RotateY(c, fp, fp, a2_) };
            { let a2_ = F32(c.m.r32((p_ch_2).wrapping_add(144))); F::Mtx_RotateZ(c, fp, fp, a2_) };
            F::Mtx_MulVec4(c, fp.wrapping_add(128), fp, fp.wrapping_add(128));
            if (((((c.m.r32((l_w_3).wrapping_add(3712)) as i32)) & (32i32))) != 0) {
                F::Mtx_MulVec4(c, fp.wrapping_add(128), (l_w_3).wrapping_add(3648), fp.wrapping_add(128));
            }
            { let v_ = l_r_8; let a_ = (l_n_5).wrapping_add(76); c.m.w32(a_, (v_).0); };
            F::Vec3_Scale(c, l_n_5, fp.wrapping_add(128), l_r_8);
            { let v_ = F32(0x3f800000); let a_ = (l_n_5).wrapping_add(12); c.m.w32(a_, (v_).0); };
            if (((((l_i_7) == (0i32)) as i32)) != 0) {
                F::Vec4_Set(c, (l_n_5).wrapping_add(16), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x3f800000));
                { let v_ = F32(0x00000000); let a_ = (l_n_5).wrapping_add(80); c.m.w32(a_, (v_).0); };
            } else {
                F::Vec3_Sub(c, fp.wrapping_add(144), l_n_5, fp.wrapping_add(160));
                { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(144)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul({ let a0_ = F::Vec3_Dot(c, fp.wrapping_add(144), fp.wrapping_add(144)); F::Port_sqrtf(c, a0_) }, F32(c.m.r32((l_key_4).wrapping_add(192)))); let a_ = (l_n_5).wrapping_add(80); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul(F32(c.m.r32((l_n_5).wrapping_add(80))), rt::fsub(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = (l_n_5).wrapping_add(16); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul(F32(c.m.r32((l_n_5).wrapping_add(80))), rt::fsub(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = ((l_n_5).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul(F32(c.m.r32((l_n_5).wrapping_add(80))), rt::fsub(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = ((l_n_5).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = ((l_n_5).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            }
            F::Vec4_Copy(c, fp.wrapping_add(160), l_n_5);
            { let v_ = rt::fadd(F32(c.m.r32((l_key_4).wrapping_add(144))), rt::fmul(F32(c.m.r32((l_key_4).wrapping_add(148))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = (l_n_5).wrapping_add(68); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fadd(F32(c.m.r32((l_key_4).wrapping_add(144))), rt::fmul(F32(c.m.r32((l_key_4).wrapping_add(148))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = (l_n_5).wrapping_add(72); c.m.w32(a_, (v_).0); };
            if (((((((((l_i_7) == (0i32)) as i32)) != 0) || (((((c.m.r32((l_n_5).wrapping_add(84))) == (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
                { let r_ = F32(0x3e4ccccc); let a_ = (l_n_5).wrapping_add(68); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = F32(0x3e4ccccc); let a_ = (l_n_5).wrapping_add(72); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            }
            F::Vec4_Copy(c, (l_n_5).wrapping_add(48), (p_ch_2).wrapping_add(48));
            { let r_ = l_step_9; let n_ = rt::fadd(l_r_8, r_); l_r_8 = n_; };
            { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
            { let r_ = F32(c.m.r32((p_ch_2).wrapping_add(148))); let a_ = (p_ch_2).wrapping_add(136); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let v_ = { let a0_ = F32(c.m.r32((p_ch_2).wrapping_add(136))); F::EftMath_WrapAngle(c, a0_) }; let a_ = (p_ch_2).wrapping_add(136); c.m.w32(a_, (v_).0); };
            { let r_ = F32(c.m.r32((p_ch_2).wrapping_add(152))); let a_ = (p_ch_2).wrapping_add(140); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let v_ = { let a0_ = F32(c.m.r32((p_ch_2).wrapping_add(140))); F::EftMath_WrapAngle(c, a0_) }; let a_ = (p_ch_2).wrapping_add(140); c.m.w32(a_, (v_).0); };
            l_link_6 = (l_n_5).wrapping_add(84);
        }
        F::Vec3_Copy(c, (p_ch_2).wrapping_add(16), fp.wrapping_add(144));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftChain_InitStrand", &PROBE_N, &[], true); }
    r_
}

pub fn EftChain_UpdateStrand(c: &mut Ctx, mut p_task_1: u32, mut p_ch_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChain_UpdateStrand", &PROBE_N, &[(p_task_1) as u32, (p_ch_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let mut l_w_3: u32 = 0;
    let mut l_p_4: u32 = 0;
    let mut l_key_5: u32 = 0;
    let mut l_n_6: u32 = 0;
    let mut l_next_7: u32 = 0;
    let mut l_link_8: u32 = 0;
    let mut l_i_9: i32 = 0;
    let mut l_fi_10: F32 = F32(0);
    let r_ = 'f: {
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        l_p_4 = c.m.r32((l_w_3).wrapping_add(52));
        l_key_5 = (l_w_3).wrapping_add(64);
        l_i_9 = 0i32;
        l_link_8 = (p_ch_2).wrapping_add(96);
        'l1: while (((((c.m.r32(l_link_8)) != (((0i32) as u32))) as i32)) != 0) {
            l_n_6 = c.m.r32(l_link_8);
            l_fi_10 = rt::i2f(l_i_9);
            l_next_7 = c.m.r32((l_n_6).wrapping_add(84));
            if (((rt::flt(l_fi_10, F32(c.m.r32((p_ch_2).wrapping_add(108)))) as i32)) != 0) {
                { let r_ = 2i32; let a_ = (l_n_6).wrapping_add(64); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            } else {
                { let r_ = (!(2i32)); let a_ = (l_n_6).wrapping_add(64); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            }
            if (((((c.m.r32((p_ch_2).wrapping_add(104)) as i32)) & (2i32))) != 0) {
                if (((((((c.m.r8((l_p_4).wrapping_add(4))) as i32)) == (1i32)) as i32)) != 0) {
                    if (((((l_next_7) != (((0i32) as u32))) as i32)) != 0) {
                        F::Vec4_Copy(c, l_n_6, l_next_7);
                        F::Vec4_Copy(c, (l_n_6).wrapping_add(16), (l_next_7).wrapping_add(16));
                    } else {
                        F::Vec4_Copy(c, fp.wrapping_add(64), l_n_6);
                        F::Vec4_Set(c, fp.wrapping_add(96), F32(0x00000000), F32(0x00000000), F32(0x3f800000), F32(0x3f800000));
                        F::Mtx_StoreIdentity(c, fp);
                        { let a2_ = F32(c.m.r32((p_ch_2).wrapping_add(136))); F::Mtx_RotateX(c, fp, fp, a2_) };
                        { let a2_ = F32(c.m.r32((p_ch_2).wrapping_add(140))); F::Mtx_RotateY(c, fp, fp, a2_) };
                        F::Mtx_MulVec4(c, fp.wrapping_add(96), fp, fp.wrapping_add(96));
                        { let a2_ = F32(c.m.r32((l_n_6).wrapping_add(76))); F::Vec3_Scale(c, l_n_6, fp.wrapping_add(96), a2_) };
                        { let v_ = F32(0x3f800000); let a_ = (l_n_6).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        F::Vec3_Sub(c, fp.wrapping_add(80), fp.wrapping_add(64), l_n_6);
                        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(80)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fmul({ let a0_ = F::Vec3_Dot(c, fp.wrapping_add(80), fp.wrapping_add(80)); F::Port_sqrtf(c, a0_) }, F32(c.m.r32((l_key_5).wrapping_add(192)))); let a_ = (l_n_6).wrapping_add(80); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fmul(F32(c.m.r32((l_n_6).wrapping_add(80))), rt::fsub(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = (l_n_6).wrapping_add(16); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fmul(F32(c.m.r32((l_n_6).wrapping_add(80))), rt::fsub(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = ((l_n_6).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fmul(F32(c.m.r32((l_n_6).wrapping_add(80))), rt::fsub(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = ((l_n_6).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x3f800000); let a_ = ((l_n_6).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        { let r_ = F32(c.m.r32((p_ch_2).wrapping_add(156))); let a_ = (p_ch_2).wrapping_add(136); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        { let v_ = rt::fadd({ let a0_ = F32(c.m.r32((p_ch_2).wrapping_add(136))); F::EftMath_WrapAngle(c, a0_) }, F32(c.m.r32((p_ch_2).wrapping_add(160)))); let a_ = (p_ch_2).wrapping_add(140); c.m.w32(a_, (v_).0); };
                        { let v_ = { let a0_ = F32(c.m.r32((p_ch_2).wrapping_add(140))); F::EftMath_WrapAngle(c, a0_) }; let a_ = (p_ch_2).wrapping_add(140); c.m.w32(a_, (v_).0); };
                    }
                } else {
                    F::Vec3_Add(c, l_n_6, l_n_6, (p_ch_2).wrapping_add(16));
                    { let v_ = rt::fmul(F32(c.m.r32((l_n_6).wrapping_add(80))), rt::fsub(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = (l_n_6).wrapping_add(16); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fmul(F32(c.m.r32((l_n_6).wrapping_add(80))), rt::fsub(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = ((l_n_6).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fmul(F32(c.m.r32((l_n_6).wrapping_add(80))), rt::fsub(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = ((l_n_6).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = ((l_n_6).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                }
            }
            F::Vec3_Add4(c, (l_n_6).wrapping_add(32), l_w_3, p_ch_2, l_n_6, (l_n_6).wrapping_add(16));
            { let v_ = F32(0x3f800000); let a_ = ((l_n_6).wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            F::Vec4_Copy(c, (l_n_6).wrapping_add(48), (p_ch_2).wrapping_add(48));
            { let r_ = F32(c.m.r32((p_ch_2).wrapping_add(168))); let a_ = ((l_n_6).wrapping_add(48)).wrapping_add(12); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::fgt(F32(c.m.r32(((l_n_6).wrapping_add(48)).wrapping_add(12))), F32(0x437f0000)) as i32)) != 0) {
                { let v_ = F32(0x437f0000); let a_ = ((l_n_6).wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            }
            if (((((((((((c.m.r8((l_p_4).wrapping_add(4))) as i32)) == (0i32)) as i32)) != 0) || (((((((c.m.r8((l_p_4).wrapping_add(4))) as i32)) == (2i32)) as i32)) != 0)) as i32)) != 0) {
                if (((rt::fle(rt::fsub(F32(c.m.r32((p_ch_2).wrapping_add(108))), F32(0x3f800000)), l_fi_10) as i32)) != 0) {
                    { let r_ = F32(0x00000000); let a_ = ((l_n_6).wrapping_add(48)).wrapping_add(12); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                } else {
                    if (((rt::fle(rt::fsub(F32(c.m.r32((p_ch_2).wrapping_add(108))), F32(0x40000000)), l_fi_10) as i32)) != 0) {
                        { let r_ = F32(0x3f000000); let a_ = ((l_n_6).wrapping_add(48)).wrapping_add(12); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    }
                }
            }
            { let o_ = l_i_9; let n_ = (o_).wrapping_add(1); l_i_9 = n_; };
            l_link_8 = (l_n_6).wrapping_add(84);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftChain_UpdateStrand", &PROBE_N, &[], true); }
    r_
}

pub fn eft_s__EftChain_OtAdd(c: &mut Ctx, mut p_p_1: u32, mut p_z_2: i32, mut p_layer_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_s__EftChain_OtAdd", &PROBE_N, &[(p_p_1) as u32, (p_z_2) as u32, (p_layer_3) as u32], false); }
    let mut l_e_4: u32 = 0;
    let r_ = 'f: {
    if (((((p_layer_3) >= (2i32)) as i32)) != 0) {
        { let r_ = 2i32; let n_ = (p_layer_3).wrapping_sub(r_); p_layer_3 = n_; };
    }
    if (((((p_z_2) < (0i32)) as i32)) != 0) {
        l_e_4 = ((c.m.r32(sym::gOtZ)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((p_layer_3) as u32)).wrapping_mul(8));
    } else {
        if (((((p_z_2) >= (4096i32)) as i32)) != 0) {
            l_e_4 = ((c.m.r32(sym::gOtZ)).wrapping_add((((4095i32) as u32)).wrapping_mul(16))).wrapping_add((((p_layer_3) as u32)).wrapping_mul(8));
        } else {
            l_e_4 = ((c.m.r32(sym::gOtZ)).wrapping_add((((p_z_2) as u32)).wrapping_mul(16))).wrapping_add((((p_layer_3) as u32)).wrapping_mul(8));
        }
    }
    { let v_ = p_p_1; let a_ = (c.m.r32((l_e_4).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = p_p_1; let a_ = (l_e_4).wrapping_add(4); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "eft_s__EftChain_OtAdd", &PROBE_N, &[], true); }
    r_
}

pub fn EftChain_DrawStrand(c: &mut Ctx, mut p_w_1: u32, mut p_w2_2: u32, mut p_ch_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChain_DrawStrand", &PROBE_N, &[(p_w_1) as u32, (p_w2_2) as u32, (p_ch_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(400) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 400);
    let mut l_p_4: u32 = 0;
    let mut l_first_5: i32 = 0;
    let mut l_link_6: u32 = 0;
    let mut l_n_7: u32 = 0;
    let mut l_next_8: u32 = 0;
    let mut l_pkt_9: u32 = 0;
    let mut l_z_10: i32 = 0;
    let mut l_j_11: i32 = 0;
    let mut l_q_12: F32 = F32(0);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 64);
        rt::memset(c, fp.wrapping_add(0), 0, 16);
        rt::memset(c, fp.wrapping_add(0).wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(0).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(16), 0, 16);
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(32), 0, 16);
        rt::memset(c, fp.wrapping_add(32).wrapping_add(0), 0, 16);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(32).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(32).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(32).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(32).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(48), 0, 16);
        rt::memset(c, fp.wrapping_add(48).wrapping_add(0), 0, 16);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(48).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(48).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(48).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(48).wrapping_add(0).wrapping_add(12), (v_).0); }
        l_first_5 = 1i32;
        l_p_4 = c.m.r32((p_w_1).wrapping_add(52));
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(544); F::Vec4_Copy(c, fp.wrapping_add(224), a1_) };
        {
            l_link_6 = (p_ch_3).wrapping_add(96);
            'l1: loop {
                if !(((((c.m.r32(l_link_6)) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_n_7 = c.m.r32(l_link_6);
                    l_next_8 = c.m.r32((l_n_7).wrapping_add(84));
                    if (((((((((l_next_8) != (((0i32) as u32))) as i32)) != 0) && (((((c.m.r32((l_n_7).wrapping_add(64)) as i32)) & (2i32))) != 0)) as i32)) != 0) {
                        F::Vec3_Sub(c, fp.wrapping_add(240), (l_next_8).wrapping_add(32), (l_n_7).wrapping_add(32));
                        F::Vec3_Sub(c, fp.wrapping_add(256), (l_n_7).wrapping_add(32), fp.wrapping_add(224));
                        F::Vec3_Cross(c, fp.wrapping_add(240), fp.wrapping_add(240), fp.wrapping_add(256));
                        F::Vec3_Normalize(c, fp.wrapping_add(240), fp.wrapping_add(240));
                        if ((l_first_5) != 0) {
                            { let a2_ = rt::fmul(F32(c.m.r32((l_n_7).wrapping_add(68))), F32(c.m.r32((p_ch_3).wrapping_add(164)))); F::Vec3_Scale(c, fp.wrapping_add(272), fp.wrapping_add(240), a2_) };
                            { let a2_ = rt::fmul(F32(c.m.r32((l_n_7).wrapping_add(72))), F32(c.m.r32((p_ch_3).wrapping_add(164)))); F::Vec3_Scale(c, fp.wrapping_add(288), fp.wrapping_add(240), a2_) };
                            F::Vec3_Add(c, (fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (l_n_7).wrapping_add(32), fp.wrapping_add(272));
                            F::Vec3_Sub(c, (fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (l_n_7).wrapping_add(32), fp.wrapping_add(288));
                        } else {
                            F::Vec3_Copy(c, (fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
                            F::Vec3_Copy(c, (fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
                        }
                        l_first_5 = 0i32;
                        { let a2_ = rt::fmul(F32(c.m.r32((l_next_8).wrapping_add(68))), F32(c.m.r32((p_ch_3).wrapping_add(164)))); F::Vec3_Scale(c, fp.wrapping_add(272), fp.wrapping_add(240), a2_) };
                        { let a2_ = rt::fmul(F32(c.m.r32((l_next_8).wrapping_add(72))), F32(c.m.r32((p_ch_3).wrapping_add(164)))); F::Vec3_Scale(c, fp.wrapping_add(288), fp.wrapping_add(240), a2_) };
                        F::Vec3_Add(c, (fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (l_next_8).wrapping_add(32), fp.wrapping_add(272));
                        F::Vec3_Sub(c, (fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (l_next_8).wrapping_add(32), fp.wrapping_add(288));
                        F::Vec3_Copy(c, (fp.wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
                        F::Vec3_Copy(c, (fp.wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
                        if (((((F::Vu0Cur_ProjectPoints(c, fp.wrapping_add(304), fp.wrapping_add(64), 4i32)) != (0i32)) as i32)) != 0) {
                            l_pkt_9 = c.m.r32(sym::gOtCur);
                            { let v_ = (l_pkt_9).wrapping_add((((1i32) as u32)).wrapping_mul(144)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
                            if (((((l_pkt_9) == (((0i32) as u32))) as i32)) != 0) {
                                break 'f;
                            }
                            if (((((((c.m.r8((l_p_4).wrapping_add(6))) as i32)) < (2i32)) as i32)) != 0) {
                                { let v_ = ((92i32) as u64); let a_ = (l_pkt_9).wrapping_add(32); c.m.w64(a_, v_); };
                                { let v_ = ((536870920i32) as u32); let a_ = l_pkt_9; c.m.w32(a_, v_); };
                                { let v_ = ((268435456i32) as u32); let a_ = (l_pkt_9).wrapping_add(8); c.m.w32(a_, v_); };
                                { let v_ = ((1342177288i32) as u32); let a_ = (l_pkt_9).wrapping_add(12); c.m.w32(a_, v_); };
                                { let v_ = 16429131440647602177u64; let a_ = (l_pkt_9).wrapping_add(16); c.m.w64(a_, v_); };
                                { let v_ = ((18599481537995104i64) as u64); let a_ = (l_pkt_9).wrapping_add(24); c.m.w64(a_, v_); };
                                { let v_ = ((0i32) as u32); let a_ = (l_pkt_9).wrapping_add(4); c.m.w32(a_, v_); };
                            } else {
                                { let v_ = ((604i32) as u64); let a_ = (l_pkt_9).wrapping_add(32); c.m.w64(a_, v_); };
                                { let v_ = ((536870920i32) as u32); let a_ = l_pkt_9; c.m.w32(a_, v_); };
                                { let v_ = ((268435456i32) as u32); let a_ = (l_pkt_9).wrapping_add(8); c.m.w32(a_, v_); };
                                { let v_ = ((1342177288i32) as u32); let a_ = (l_pkt_9).wrapping_add(12); c.m.w32(a_, v_); };
                                { let v_ = 16429131440647602177u64; let a_ = (l_pkt_9).wrapping_add(16); c.m.w64(a_, v_); };
                                { let v_ = ((18599481537995120i64) as u64); let a_ = (l_pkt_9).wrapping_add(24); c.m.w64(a_, v_); };
                                { let v_ = ((0i32) as u32); let a_ = (l_pkt_9).wrapping_add(4); c.m.w32(a_, v_); };
                            }
                            {
                                l_j_11 = 0i32;
                                'l3: loop {
                                    if !(((((l_j_11) < (4i32)) as i32)) != 0) { break 'l3; }
                                    'c4: {
                                        l_q_12 = rt::fdiv(F32(0x3f800000), rt::i2f((c.m.r32(((fp.wrapping_add(304)).wrapping_add((((l_j_11) as u32)).wrapping_mul(16))).wrapping_add(12)) as i32)));
                                        F::Vec3_Scale(c, (fp.wrapping_add(160)).wrapping_add((((l_j_11) as u32)).wrapping_mul(16)), (fp).wrapping_add((((l_j_11) as u32)).wrapping_mul(16)), l_q_12);
                                    }
                                    { let o_ = l_j_11; let n_ = (o_).wrapping_add(1); l_j_11 = n_; };
                                }
                            }
                            F::Vec4_ToInt(c, fp.wrapping_add(368), (l_n_7).wrapping_add(48));
                            F::Vec4_ToInt(c, fp.wrapping_add(384), (l_next_8).wrapping_add(48));
                            l_z_10 = ((((c.m.r32(((fp.wrapping_add(304)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)).wrapping_add((c.m.r32(((fp.wrapping_add(304)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add(((c.m.r32(((fp.wrapping_add(304)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)).wrapping_add((c.m.r32(((fp.wrapping_add(304)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)))).wrapping_shr((10i32) as u32);
                            if (((((c.m.r32((p_w2_2).wrapping_add(3712)) as i32)) & (16i32))) != 0) {
                                {
                                    l_j_11 = 3i32;
                                    'l5: loop {
                                        if !(((((l_j_11) >= (0i32)) as i32)) != 0) { break 'l5; }
                                        'c6: {
                                            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(304)).wrapping_add((((l_j_11) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                        }
                                        { let o_ = l_j_11; let n_ = (o_).wrapping_sub(1); l_j_11 = n_; };
                                    }
                                }
                            }
                            { let v_ = (((c.m.r32((fp.wrapping_add(368)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
                            { let v_ = (((c.m.r32((fp.wrapping_add(368)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
                            { let v_ = (((c.m.r32((fp.wrapping_add(368)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
                            { let v_ = (((c.m.r32((fp.wrapping_add(368)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
                            { let v_ = F32(c.m.r32(((fp.wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_pkt_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            { let v_ = (((c.m.r32((fp.wrapping_add(368)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
                            { let v_ = (((c.m.r32((fp.wrapping_add(368)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
                            { let v_ = (((c.m.r32((fp.wrapping_add(368)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
                            { let v_ = (((c.m.r32((fp.wrapping_add(368)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
                            { let v_ = F32(c.m.r32(((fp.wrapping_add(160)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_pkt_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            { let v_ = (((c.m.r32((fp.wrapping_add(384)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
                            { let v_ = (((c.m.r32((fp.wrapping_add(384)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
                            { let v_ = (((c.m.r32((fp.wrapping_add(384)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
                            { let v_ = (((c.m.r32((fp.wrapping_add(384)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
                            { let v_ = F32(c.m.r32(((fp.wrapping_add(160)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_pkt_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            { let v_ = (((c.m.r32((fp.wrapping_add(384)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
                            { let v_ = (((c.m.r32((fp.wrapping_add(384)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
                            { let v_ = (((c.m.r32((fp.wrapping_add(384)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
                            { let v_ = (((c.m.r32((fp.wrapping_add(384)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
                            { let v_ = F32(c.m.r32(((fp.wrapping_add(160)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_pkt_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(c.m.r32((fp.wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(16)))); let a_ = (((l_pkt_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(c.m.r32(((fp.wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = (((l_pkt_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(c.m.r32((fp.wrapping_add(160)).wrapping_add((((1i32) as u32)).wrapping_mul(16)))); let a_ = (((l_pkt_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(c.m.r32(((fp.wrapping_add(160)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = (((l_pkt_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(c.m.r32((fp.wrapping_add(160)).wrapping_add((((2i32) as u32)).wrapping_mul(16)))); let a_ = (((l_pkt_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(c.m.r32(((fp.wrapping_add(160)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = (((l_pkt_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(c.m.r32((fp.wrapping_add(160)).wrapping_add((((3i32) as u32)).wrapping_mul(16)))); let a_ = (((l_pkt_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(c.m.r32(((fp.wrapping_add(160)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = (((l_pkt_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                            { let v_ = (((c.m.r32((fp.wrapping_add(304)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_pkt_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                            { let v_ = (((c.m.r32(((fp.wrapping_add(304)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_pkt_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                            { let v_ = (((c.m.r32(((fp.wrapping_add(304)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_pkt_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                            { let v_ = ((255i32) as u64); let a_ = ((((l_pkt_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                            { let v_ = (((c.m.r32((fp.wrapping_add(304)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_pkt_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                            { let v_ = (((c.m.r32(((fp.wrapping_add(304)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_pkt_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                            { let v_ = (((c.m.r32(((fp.wrapping_add(304)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_pkt_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                            { let v_ = ((255i32) as u64); let a_ = ((((l_pkt_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                            { let v_ = (((c.m.r32((fp.wrapping_add(304)).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_pkt_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                            { let v_ = (((c.m.r32(((fp.wrapping_add(304)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_pkt_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                            { let v_ = (((c.m.r32(((fp.wrapping_add(304)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_pkt_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                            { let v_ = ((255i32) as u64); let a_ = ((((l_pkt_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                            { let v_ = (((c.m.r32((fp.wrapping_add(304)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_pkt_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                            { let v_ = (((c.m.r32(((fp.wrapping_add(304)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_pkt_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                            { let v_ = (((c.m.r32(((fp.wrapping_add(304)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_pkt_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                            { let v_ = ((255i32) as u64); let a_ = ((((l_pkt_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                            { let v_ = c.m.r64((p_w2_2).wrapping_add(3840)); let a_ = (l_pkt_9).wrapping_add(40); c.m.w64(a_, v_); };
                            { let a2_ = ((c.m.r8((l_p_4).wrapping_add(6))) as i32); F::eft_s__EftChain_OtAdd(c, l_pkt_9, l_z_10, a2_) };
                        }
                    }
                }
                l_link_6 = (l_n_7).wrapping_add(84);
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftChain_DrawStrand", &PROBE_N, &[], true); }
    r_
}

pub fn EftChain_DrawStrandClipped(c: &mut Ctx, mut p_w_1: u32, mut p_w2_2: u32, mut p_ch_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChain_DrawStrandClipped", &PROBE_N, &[(p_w_1) as u32, (p_w2_2) as u32, (p_ch_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(736) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 736);
    let mut l_p_4: u32 = 0;
    let mut l_first_5: i32 = 0;
    let mut l_link_6: u32 = 0;
    let mut l_n_7: u32 = 0;
    let mut l_next_8: u32 = 0;
    let mut l_j_9: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 64);
        rt::memset(c, fp.wrapping_add(0), 0, 16);
        rt::memset(c, fp.wrapping_add(0).wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(0).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(16), 0, 16);
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(32), 0, 16);
        rt::memset(c, fp.wrapping_add(32).wrapping_add(0), 0, 16);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(32).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(32).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(32).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(32).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(48), 0, 16);
        rt::memset(c, fp.wrapping_add(48).wrapping_add(0), 0, 16);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(48).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(48).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(48).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(48).wrapping_add(0).wrapping_add(12), (v_).0); }
        l_first_5 = 1i32;
        l_p_4 = c.m.r32((p_w_1).wrapping_add(52));
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(544); F::Vec4_Copy(c, fp.wrapping_add(160), a1_) };
        {
            l_link_6 = (p_ch_3).wrapping_add(96);
            'l1: loop {
                if !(((((c.m.r32(l_link_6)) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_n_7 = c.m.r32(l_link_6);
                    l_next_8 = c.m.r32((l_n_7).wrapping_add(84));
                    if (((((((((l_next_8) != (((0i32) as u32))) as i32)) != 0) && (((((c.m.r32((l_n_7).wrapping_add(64)) as i32)) & (2i32))) != 0)) as i32)) != 0) {
                        F::Vec3_Sub(c, fp.wrapping_add(176), (l_next_8).wrapping_add(32), (l_n_7).wrapping_add(32));
                        F::Vec3_Sub(c, fp.wrapping_add(192), (l_n_7).wrapping_add(32), fp.wrapping_add(160));
                        F::Vec3_Cross(c, fp.wrapping_add(176), fp.wrapping_add(176), fp.wrapping_add(192));
                        F::Vec3_Normalize(c, fp.wrapping_add(176), fp.wrapping_add(176));
                        if ((l_first_5) != 0) {
                            { let a2_ = rt::fmul(F32(c.m.r32((l_n_7).wrapping_add(68))), F32(c.m.r32((p_ch_3).wrapping_add(164)))); F::Vec3_Scale(c, fp.wrapping_add(208), fp.wrapping_add(176), a2_) };
                            { let a2_ = rt::fmul(F32(c.m.r32((l_n_7).wrapping_add(72))), F32(c.m.r32((p_ch_3).wrapping_add(164)))); F::Vec3_Scale(c, fp.wrapping_add(224), fp.wrapping_add(176), a2_) };
                            F::Vec3_Add(c, (fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (l_n_7).wrapping_add(32), fp.wrapping_add(208));
                            F::Vec3_Sub(c, (fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (l_n_7).wrapping_add(32), fp.wrapping_add(224));
                        } else {
                            F::Vec3_Copy(c, (fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
                            F::Vec3_Copy(c, (fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
                        }
                        l_first_5 = 0i32;
                        { let a2_ = rt::fmul(F32(c.m.r32((l_next_8).wrapping_add(68))), F32(c.m.r32((p_ch_3).wrapping_add(164)))); F::Vec3_Scale(c, fp.wrapping_add(208), fp.wrapping_add(176), a2_) };
                        { let a2_ = rt::fmul(F32(c.m.r32((l_next_8).wrapping_add(72))), F32(c.m.r32((p_ch_3).wrapping_add(164)))); F::Vec3_Scale(c, fp.wrapping_add(224), fp.wrapping_add(176), a2_) };
                        F::Vec3_Add(c, (fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (l_next_8).wrapping_add(32), fp.wrapping_add(208));
                        F::Vec3_Sub(c, (fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (l_next_8).wrapping_add(32), fp.wrapping_add(224));
                        F::Vec3_Copy(c, (fp.wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
                        F::Vec3_Copy(c, (fp.wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
                        F::Vec4_Copy(c, (fp.wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (l_n_7).wrapping_add(48));
                        F::Vec4_Copy(c, (fp.wrapping_add(240)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (l_n_7).wrapping_add(48));
                        F::Vec4_Copy(c, (fp.wrapping_add(240)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (l_next_8).wrapping_add(48));
                        F::Vec4_Copy(c, (fp.wrapping_add(240)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (l_next_8).wrapping_add(48));
                        {
                            l_j_9 = 0i32;
                            'l3: loop {
                                if !(((((l_j_9) < (2i32)) as i32)) != 0) { break 'l3; }
                                'c4: {
                                    F::ClipVtx_Set(c, (fp.wrapping_add(304)).wrapping_add((((0i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(64)).wrapping_add((((l_j_9) as u32)).wrapping_mul(16)), (fp).wrapping_add((((l_j_9) as u32)).wrapping_mul(16)), (fp.wrapping_add(240)).wrapping_add((((l_j_9) as u32)).wrapping_mul(16)));
                                    F::ClipVtx_Set(c, (fp.wrapping_add(304)).wrapping_add((((1i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(64)).wrapping_add(((((l_j_9).wrapping_add(1i32)) as u32)).wrapping_mul(16)), (fp).wrapping_add(((((l_j_9).wrapping_add(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(240)).wrapping_add(((((l_j_9).wrapping_add(1i32)) as u32)).wrapping_mul(16)));
                                    F::ClipVtx_Set(c, (fp.wrapping_add(304)).wrapping_add((((2i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(64)).wrapping_add(((((l_j_9).wrapping_add(2i32)) as u32)).wrapping_mul(16)), (fp).wrapping_add(((((l_j_9).wrapping_add(2i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(240)).wrapping_add(((((l_j_9).wrapping_add(2i32)) as u32)).wrapping_mul(16)));
                                    { let a1_ = ((c.m.r8((l_p_4).wrapping_add(6))) as i32); let a4_ = ((((c.m.r32((p_w2_2).wrapping_add(3712)) as i32)).wrapping_shr((4i32) as u32)) & (1i32)); let a6_ = c.m.r64((p_w2_2).wrapping_add(3840)); F::EftGfx_DrawPolyScaledZ(c, fp.wrapping_add(304), a1_, 0i32, 0i32, a4_, 0i32, a6_, F32(0x3f800000)) };
                                }
                                { let o_ = l_j_9; let n_ = (o_).wrapping_add(1); l_j_9 = n_; };
                            }
                        }
                    }
                }
                l_link_6 = (l_n_7).wrapping_add(84);
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftChain_DrawStrandClipped", &PROBE_N, &[], true); }
    r_
}

pub fn EftChain_StartStrand(c: &mut Ctx, mut p_task_1: u32, mut p_ch_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChain_StartStrand", &PROBE_N, &[(p_task_1) as u32, (p_ch_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let mut l_p_4: u32 = 0;
    let mut l_key_5: u32 = 0;
    let mut l_r_6: F32 = F32(0);
    let mut l_life_7: F32 = F32(0);
    let mut l_cx_8: F32 = F32(0);
    let mut l_cy_9: F32 = F32(0);
    let mut l_a_10: F32 = F32(0);
    let mut l_b_11: F32 = F32(0);
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    l_p_4 = c.m.r32((l_w_3).wrapping_add(52));
    l_key_5 = (l_w_3).wrapping_add(64);
    { let v_ = ((0i32) as u32); let a_ = (p_ch_2).wrapping_add(96); c.m.w32(a_, v_); };
    { let _ = { let a1_ = (c.m.r32((l_w_3).wrapping_add(3728)) as i32); F::EftChain_AllocNodes(c, p_ch_2, a1_) }; };
    if ((((((c.m.r32((p_ch_2).wrapping_add(100)) as i32)) < (3i32)) as i32)) != 0) {
        F::EftChain_FreeNodes(c, p_ch_2);
        break 'f 0i32;
    }
    { let r_ = 1i32; let a_ = (p_ch_2).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    l_life_7 = rt::fadd(F32(c.m.r32((l_key_5).wrapping_add(132))), rt::fmul(F32(c.m.r32((l_key_5).wrapping_add(136))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff))));
    { let v_ = F32(0x00000000); let a_ = (p_ch_2).wrapping_add(112); c.m.w32(a_, (v_).0); };
    { let v_ = l_life_7; let a_ = (p_ch_2).wrapping_add(116); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(l_life_7, F32(c.m.r32((l_key_5).wrapping_add(140)))); let a_ = (p_ch_2).wrapping_add(124); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(l_life_7, F32(c.m.r32((l_key_5).wrapping_add(164)))); let a_ = (p_ch_2).wrapping_add(128); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fsub(l_life_7, F32(c.m.r32((l_key_5).wrapping_add(244)))); let a_ = (p_ch_2).wrapping_add(132); c.m.w32(a_, (v_).0); };
    if (((rt::fle(F32(c.m.r32((l_key_5).wrapping_add(128))), F32(c.m.r32((p_ch_2).wrapping_add(112)))) as i32)) != 0) {
        { let r_ = 4i32; let a_ = (p_ch_2).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        l_r_6 = rt::fdiv(rt::i2f((c.m.r32((p_ch_2).wrapping_add(100)) as i32)), F32(c.m.r32((l_key_5).wrapping_add(128))));
        { let v_ = rt::fsub(l_life_7, l_r_6); let a_ = (p_ch_2).wrapping_add(120); c.m.w32(a_, (v_).0); };
    }
    { let v_ = F32(0x00000000); let a_ = (p_ch_2).wrapping_add(108); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = (p_ch_2).wrapping_add(164); c.m.w32(a_, (v_).0); };
    l_r_6 = rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff));
    { let v_ = rt::fadd(F32(c.m.r32(l_key_5)), rt::fmul(F32(c.m.r32((l_key_5).wrapping_add(16))), l_r_6)); let a_ = (p_ch_2).wrapping_add(64); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(F32(c.m.r32((l_key_5).wrapping_add(4))), rt::fmul(F32(c.m.r32(((l_key_5).wrapping_add(16)).wrapping_add(4))), l_r_6)); let a_ = ((p_ch_2).wrapping_add(64)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(F32(c.m.r32((l_key_5).wrapping_add(8))), rt::fmul(F32(c.m.r32(((l_key_5).wrapping_add(16)).wrapping_add(8))), l_r_6)); let a_ = ((p_ch_2).wrapping_add(64)).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(F32(c.m.r32((l_key_5).wrapping_add(12))), rt::fmul(F32(c.m.r32(((l_key_5).wrapping_add(16)).wrapping_add(12))), l_r_6)); let a_ = ((p_ch_2).wrapping_add(64)).wrapping_add(12); c.m.w32(a_, (v_).0); };
    l_r_6 = rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff));
    { let v_ = rt::fadd(F32(c.m.r32((l_key_5).wrapping_add(32))), rt::fmul(F32(c.m.r32((l_key_5).wrapping_add(48))), l_r_6)); let a_ = (p_ch_2).wrapping_add(80); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(F32(c.m.r32(((l_key_5).wrapping_add(32)).wrapping_add(4))), rt::fmul(F32(c.m.r32(((l_key_5).wrapping_add(48)).wrapping_add(4))), l_r_6)); let a_ = ((p_ch_2).wrapping_add(80)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(F32(c.m.r32(((l_key_5).wrapping_add(32)).wrapping_add(8))), rt::fmul(F32(c.m.r32(((l_key_5).wrapping_add(48)).wrapping_add(8))), l_r_6)); let a_ = ((p_ch_2).wrapping_add(80)).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(F32(c.m.r32(((l_key_5).wrapping_add(32)).wrapping_add(12))), rt::fmul(F32(c.m.r32(((l_key_5).wrapping_add(48)).wrapping_add(12))), l_r_6)); let a_ = ((p_ch_2).wrapping_add(80)).wrapping_add(12); c.m.w32(a_, (v_).0); };
    F::Vec4_Sub(c, (p_ch_2).wrapping_add(80), (p_ch_2).wrapping_add(80), (p_ch_2).wrapping_add(64));
    F::Vec4_Copy(c, (p_ch_2).wrapping_add(48), (p_ch_2).wrapping_add(64));
    if (((((((c.m.r8((l_p_4).wrapping_add(4))) as i32)) == (2i32)) as i32)) != 0) {
        l_a_10 = rt::fadd(F32(c.m.r32((l_key_5).wrapping_add(96))), rt::fmul(F32(c.m.r32((l_key_5).wrapping_add(100))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff))));
        l_b_11 = rt::fadd(F32(c.m.r32((l_key_5).wrapping_add(112))), rt::fmul(F32(c.m.r32((l_key_5).wrapping_add(116))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff))));
        { let v_ = l_a_10; let a_ = (p_ch_2).wrapping_add(136); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((p_ch_2).wrapping_add(136))); F::EftMath_WrapAngle(c, a0_) }; let a_ = (p_ch_2).wrapping_add(136); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_ch_2).wrapping_add(140); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((l_w_3).wrapping_add(3804))); let a_ = (p_ch_2).wrapping_add(144); c.m.w32(a_, (v_).0); };
        { let r_ = l_b_11; let a_ = (l_w_3).wrapping_add(3804); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((l_w_3).wrapping_add(3804))); F::EftMath_WrapAngle(c, a0_) }; let a_ = (l_w_3).wrapping_add(3804); c.m.w32(a_, (v_).0); };
    } else {
        { let v_ = { let a_ = ((l_w_3).wrapping_add(3772)).wrapping_add(((((c.m.r32((l_w_3).wrapping_add(3724)) as i32)) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = (p_ch_2).wrapping_add(136); c.m.w32(a_, (v_).0); };
        { let v_ = { let a_ = ((l_w_3).wrapping_add(3788)).wrapping_add(((((c.m.r32((l_w_3).wrapping_add(3724)) as i32)) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = (p_ch_2).wrapping_add(140); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((l_w_3).wrapping_add(3804))); let a_ = (p_ch_2).wrapping_add(144); c.m.w32(a_, (v_).0); };
    }
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(3724)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        { let v_ = rt::fadd(F32(c.m.r32((l_key_5).wrapping_add(208))), rt::fmul(F32(c.m.r32((l_key_5).wrapping_add(212))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = (p_ch_2).wrapping_add(148); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((l_key_5).wrapping_add(216))), rt::fmul(F32(c.m.r32((l_key_5).wrapping_add(220))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = (p_ch_2).wrapping_add(152); c.m.w32(a_, (v_).0); };
    } else {
        { let v_ = rt::fneg(rt::fadd(F32(c.m.r32((l_key_5).wrapping_add(208))), rt::fmul(F32(c.m.r32((l_key_5).wrapping_add(212))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff))))); let a_ = (p_ch_2).wrapping_add(148); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fneg(rt::fadd(F32(c.m.r32((l_key_5).wrapping_add(216))), rt::fmul(F32(c.m.r32((l_key_5).wrapping_add(220))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff))))); let a_ = (p_ch_2).wrapping_add(152); c.m.w32(a_, (v_).0); };
    }
    F::EftChain_InitStrand(c, p_task_1, p_ch_2);
    l_cx_8 = rt::fadd(F32(c.m.r32((l_key_5).wrapping_add(224))), rt::fmul(F32(c.m.r32((l_key_5).wrapping_add(228))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff))));
    l_cy_9 = rt::fadd(F32(c.m.r32((l_key_5).wrapping_add(232))), rt::fmul(F32(c.m.r32((l_key_5).wrapping_add(236))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff))));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(3724)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        { let v_ = l_cx_8; let a_ = (p_ch_2).wrapping_add(156); c.m.w32(a_, (v_).0); };
        { let v_ = l_cy_9; let a_ = (p_ch_2).wrapping_add(160); c.m.w32(a_, (v_).0); };
    } else {
        { let v_ = rt::fneg(l_cx_8); let a_ = (p_ch_2).wrapping_add(156); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fneg(l_cy_9); let a_ = (p_ch_2).wrapping_add(160); c.m.w32(a_, (v_).0); };
    }
    l_r_6 = rt::fadd(F32(c.m.r32((l_key_5).wrapping_add(200))), rt::fmul(F32(c.m.r32((l_key_5).wrapping_add(204))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff))));
    { let r_ = l_r_6; let a_ = (p_ch_2).wrapping_add(156); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    { let r_ = l_r_6; let a_ = (p_ch_2).wrapping_add(160); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    F::Vec3_Scale(c, (p_ch_2).wrapping_add(16), (p_ch_2).wrapping_add(16), l_r_6);
    's1: {
        let sv12: i32 = ((c.m.r8((l_p_4).wrapping_add(5))) as i32);
        match sv12 {
            0i32 => {
                { let r_ = 256i32; let a_ = (p_ch_2).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            1i32 => {
                if ((((!((((F::Port_Rand(c)) & (1i32))) != 0)) as i32)) != 0) {
                    { let r_ = 256i32; let a_ = (p_ch_2).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
                break 's1;
            }
            2i32 => {
                break 's1;
            }
            _ => {
            }
        }
    }
    { let v_ = F32(c.m.r32((l_key_5).wrapping_add(240))); let a_ = (p_ch_2).wrapping_add(184); c.m.w32(a_, (v_).0); };
    if (((rt::fle(F32(c.m.r32((p_ch_2).wrapping_add(184))), F32(0x00000000)) as i32)) != 0) {
        { let r_ = 16i32; let a_ = (p_ch_2).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    { let v_ = F32(c.m.r32((l_key_5).wrapping_add(244))); let a_ = (p_ch_2).wrapping_add(188); c.m.w32(a_, (v_).0); };
    if (((rt::fle(F32(c.m.r32((p_ch_2).wrapping_add(188))), F32(0x00000000)) as i32)) != 0) {
        { let r_ = 32i32; let a_ = (p_ch_2).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        { let v_ = F32(c.m.r32((p_ch_2).wrapping_add(188))); let a_ = (p_ch_2).wrapping_add(180); c.m.w32(a_, (v_).0); };
    }
    F::Vec3_Scale(c, (p_ch_2).wrapping_add(16), (p_ch_2).wrapping_add(16), F32(0x3f800000));
    { let a_ = (l_w_3).wrapping_add(3720); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChain_StartStrand", &PROBE_N, &[], true); }
    r_
}

pub fn EftChain_UpdateStrands(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChain_UpdateStrands", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_p_3: u32 = 0;
    let mut l_key_4: u32 = 0;
    let mut l_ch_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_t_7: F32 = F32(0);
    let mut l_u_8: F32 = F32(0);
    let mut l_m_9: F32 = F32(0);
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_p_3 = c.m.r32((l_w_2).wrapping_add(52));
    l_key_4 = (l_w_2).wrapping_add(64);
    {
        l_i_6 = 0i32;
        'l1: loop {
            if !(((((l_i_6) < (16i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_ch_5 = ((l_w_2).wrapping_add(320)).wrapping_add((((l_i_6) as u32)).wrapping_mul(208));
                if (((((c.m.r32((l_ch_5).wrapping_add(104)) as i32)) & (1i32))) != 0) {
                    if ((((!(((((c.m.r32((l_ch_5).wrapping_add(104)) as i32)) & (8i32))) != 0)) as i32)) != 0) {
                        l_t_7 = rt::fdiv(F32(c.m.r32((l_ch_5).wrapping_add(112))), F32(c.m.r32((l_ch_5).wrapping_add(116))));
                        if (((rt::fgt(l_t_7, F32(0x3f800000)) as i32)) != 0) {
                            l_t_7 = F32(0x3f800000);
                        }
                        if ((((((((!(((((c.m.r32((l_ch_5).wrapping_add(104)) as i32)) & (64i32))) != 0)) as i32)) != 0) && (((rt::fle(F32(c.m.r32((l_ch_5).wrapping_add(124))), F32(c.m.r32((l_ch_5).wrapping_add(112)))) as i32)) != 0)) as i32)) != 0) {
                            { let r_ = 64i32; let a_ = (l_ch_5).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                        if ((((!(((((c.m.r32((l_ch_5).wrapping_add(104)) as i32)) & (2i32))) != 0)) as i32)) != 0) {
                            { let r_ = F32(c.m.r32((l_key_4).wrapping_add(124))); let a_ = (l_ch_5).wrapping_add(108); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            if (((rt::fle(rt::i2f((c.m.r32((l_ch_5).wrapping_add(100)) as i32)), F32(c.m.r32((l_ch_5).wrapping_add(108)))) as i32)) != 0) {
                                { let v_ = rt::i2f((c.m.r32((l_ch_5).wrapping_add(100)) as i32)); let a_ = (l_ch_5).wrapping_add(108); c.m.w32(a_, (v_).0); };
                                { let r_ = 2i32; let a_ = (l_ch_5).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            }
                        }
                        if ((((((((!(((((c.m.r32((l_ch_5).wrapping_add(104)) as i32)) & (4i32))) != 0)) as i32)) != 0) && (((rt::fle(F32(c.m.r32((l_ch_5).wrapping_add(120))), F32(c.m.r32((l_ch_5).wrapping_add(112)))) as i32)) != 0)) as i32)) != 0) {
                            { let r_ = F32(c.m.r32((l_key_4).wrapping_add(128))); let a_ = (l_ch_5).wrapping_add(108); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            if (((rt::fle(F32(c.m.r32((l_ch_5).wrapping_add(108))), F32(0x00000000)) as i32)) != 0) {
                                { let v_ = F32(0x00000000); let a_ = (l_ch_5).wrapping_add(108); c.m.w32(a_, (v_).0); };
                                { let r_ = 4i32; let a_ = (l_ch_5).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            }
                        }
                        if ((((!(((((c.m.r32((l_ch_5).wrapping_add(104)) as i32)) & (128i32))) != 0)) as i32)) != 0) {
                            if (((rt::fle(F32(c.m.r32((l_ch_5).wrapping_add(128))), F32(c.m.r32((l_ch_5).wrapping_add(112)))) as i32)) != 0) {
                                { let v_ = F32(c.m.r32((l_key_4).wrapping_add(168))); let a_ = (l_ch_5).wrapping_add(164); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(c.m.r32((l_key_4).wrapping_add(172))); let a_ = (l_ch_5).wrapping_add(168); c.m.w32(a_, (v_).0); };
                                { let r_ = 128i32; let a_ = (l_ch_5).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            }
                        } else {
                            { let v_ = F32(0x3f800000); let a_ = (l_ch_5).wrapping_add(164); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(0x3f800000); let a_ = (l_ch_5).wrapping_add(168); c.m.w32(a_, (v_).0); };
                        }
                        if ((((((((c.m.r32(l_p_3)) & (((4i32) as u32)))) != 0) && (((rt::fgt(F32(c.m.r32((l_key_4).wrapping_add(160))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                            l_u_8 = rt::fdiv(F32(c.m.r32((l_ch_5).wrapping_add(192))), F32(c.m.r32((l_key_4).wrapping_add(160))));
                            l_m_9 = rt::fadd(F32(c.m.r32((l_key_4).wrapping_add(152))), rt::fmul(F32(c.m.r32((l_key_4).wrapping_add(156))), l_u_8));
                            { let r_ = l_m_9; let a_ = (l_ch_5).wrapping_add(164); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            if ((((!(((((c.m.r32((l_ch_5).wrapping_add(104)) as i32)) & (1024i32))) != 0)) as i32)) != 0) {
                                { let r_ = F32(0x3f800000); let a_ = (l_ch_5).wrapping_add(192); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                if (((rt::fle(F32(c.m.r32((l_key_4).wrapping_add(160))), F32(c.m.r32((l_ch_5).wrapping_add(192)))) as i32)) != 0) {
                                    { let v_ = F32(c.m.r32((l_key_4).wrapping_add(160))); let a_ = (l_ch_5).wrapping_add(192); c.m.w32(a_, (v_).0); };
                                    { let r_ = 1024i32; let a_ = (l_ch_5).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                }
                            } else {
                                { let r_ = F32(0x3f800000); let a_ = (l_ch_5).wrapping_add(192); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                if (((rt::fle(F32(c.m.r32((l_ch_5).wrapping_add(192))), F32(0x00000000)) as i32)) != 0) {
                                    { let v_ = F32(0x00000000); let a_ = (l_ch_5).wrapping_add(192); c.m.w32(a_, (v_).0); };
                                    { let r_ = (!(1024i32)); let a_ = (l_ch_5).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                                }
                            }
                        }
                        if (((((c.m.r32((l_ch_5).wrapping_add(104)) as i32)) & (256i32))) != 0) {
                            F::Vec4_Scale(c, (l_ch_5).wrapping_add(48), (l_ch_5).wrapping_add(80), l_t_7);
                            F::Vec4_Add(c, (l_ch_5).wrapping_add(48), (l_ch_5).wrapping_add(48), (l_ch_5).wrapping_add(64));
                        } else {
                            F::Vec4_Copy(c, (l_ch_5).wrapping_add(48), (l_ch_5).wrapping_add(64));
                        }
                        F::Vec4_Clamp(c, (l_ch_5).wrapping_add(48), (l_ch_5).wrapping_add(48), F32(0x00000000), F32(0x437f0000));
                        if ((((!(((((c.m.r32((l_ch_5).wrapping_add(104)) as i32)) & (16i32))) != 0)) as i32)) != 0) {
                            l_u_8 = rt::fdiv(F32(c.m.r32((l_ch_5).wrapping_add(176))), F32(c.m.r32((l_ch_5).wrapping_add(184))));
                            { let r_ = F32(0x3f800000); let a_ = (l_ch_5).wrapping_add(176); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            if (((rt::fle(F32(c.m.r32((l_ch_5).wrapping_add(184))), F32(c.m.r32((l_ch_5).wrapping_add(176)))) as i32)) != 0) {
                                { let r_ = 16i32; let a_ = (l_ch_5).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            } else {
                                { let r_ = l_u_8; let a_ = ((l_ch_5).wrapping_add(48)).wrapping_add(12); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            }
                        }
                        if (((rt::fge(F32(c.m.r32((l_ch_5).wrapping_add(112))), F32(c.m.r32((l_ch_5).wrapping_add(132)))) as i32)) != 0) {
                            if ((((!(((((c.m.r32((l_ch_5).wrapping_add(104)) as i32)) & (32i32))) != 0)) as i32)) != 0) {
                                l_u_8 = rt::fdiv(F32(c.m.r32((l_ch_5).wrapping_add(180))), F32(c.m.r32((l_ch_5).wrapping_add(188))));
                                { let r_ = F32(0x3f800000); let a_ = (l_ch_5).wrapping_add(180); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                if ((((!(((rt::fle(F32(c.m.r32((l_ch_5).wrapping_add(180))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                                    { let r_ = l_u_8; let a_ = ((l_ch_5).wrapping_add(48)).wrapping_add(12); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                } else {
                                    { let v_ = F32(0x00000000); let a_ = ((l_ch_5).wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                    { let r_ = 32i32; let a_ = (l_ch_5).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                }
                            } else {
                                { let v_ = F32(0x00000000); let a_ = ((l_ch_5).wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                            }
                        }
                        if ((((((((c.m.r32(l_p_3)) & (((8i32) as u32)))) != 0) && (((rt::fgt(F32(c.m.r32((l_key_4).wrapping_add(88))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                            l_u_8 = rt::fdiv(F32(c.m.r32((l_ch_5).wrapping_add(196))), F32(c.m.r32((l_key_4).wrapping_add(88))));
                            l_m_9 = rt::fadd(F32(c.m.r32(((l_key_4).wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_key_4).wrapping_add(76)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_u_8));
                            { let r_ = l_m_9; let a_ = (l_ch_5).wrapping_add(48); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            l_m_9 = rt::fadd(F32(c.m.r32(((l_key_4).wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_key_4).wrapping_add(76)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_u_8));
                            { let r_ = l_m_9; let a_ = ((l_ch_5).wrapping_add(48)).wrapping_add(4); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            l_m_9 = rt::fadd(F32(c.m.r32(((l_key_4).wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_key_4).wrapping_add(76)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_u_8));
                            { let r_ = l_m_9; let a_ = ((l_ch_5).wrapping_add(48)).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            if ((((!(((((c.m.r32((l_ch_5).wrapping_add(104)) as i32)) & (512i32))) != 0)) as i32)) != 0) {
                                { let r_ = F32(0x3f800000); let a_ = (l_ch_5).wrapping_add(196); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                if (((rt::fle(F32(c.m.r32((l_key_4).wrapping_add(88))), F32(c.m.r32((l_ch_5).wrapping_add(196)))) as i32)) != 0) {
                                    { let v_ = F32(c.m.r32((l_key_4).wrapping_add(88))); let a_ = (l_ch_5).wrapping_add(196); c.m.w32(a_, (v_).0); };
                                    { let r_ = 512i32; let a_ = (l_ch_5).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                }
                            } else {
                                { let r_ = F32(0x3f800000); let a_ = (l_ch_5).wrapping_add(196); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                if (((rt::fle(F32(c.m.r32((l_ch_5).wrapping_add(196))), F32(0x00000000)) as i32)) != 0) {
                                    { let v_ = F32(0x00000000); let a_ = (l_ch_5).wrapping_add(196); c.m.w32(a_, (v_).0); };
                                    { let r_ = (!(512i32)); let a_ = (l_ch_5).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                                }
                            }
                        }
                        if (((((((((((rt::fle(F32(c.m.r32(((l_ch_5).wrapping_add(48)).wrapping_add(12))), F32(0x00000000)) as i32)) != 0) || (((rt::fle(F32(c.m.r32((l_ch_5).wrapping_add(164))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) || (((rt::fle(F32(c.m.r32((l_ch_5).wrapping_add(168))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                            { let r_ = (!(2048i32)); let a_ = (l_ch_5).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                        } else {
                            { let r_ = 2048i32; let a_ = (l_ch_5).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                        F::EftChain_UpdateStrand(c, p_task_1, l_ch_5);
                        { let r_ = F32(0x3f800000); let a_ = (l_ch_5).wrapping_add(112); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        if (((rt::fle(F32(c.m.r32((l_ch_5).wrapping_add(116))), F32(c.m.r32((l_ch_5).wrapping_add(112)))) as i32)) != 0) {
                            if (((rt::fle(F32(c.m.r32(((l_ch_5).wrapping_add(48)).wrapping_add(12))), F32(0x00000000)) as i32)) != 0) {
                                { let r_ = 8i32; let a_ = (l_ch_5).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            }
                        }
                    } else {
                        F::EftChain_FreeNodes(c, l_ch_5);
                        { let v_ = 0i32; let a_ = (l_ch_5).wrapping_add(104); c.m.w32(a_, (v_) as u32); };
                        { let a_ = (l_w_2).wrapping_add(3720); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    }
                }
            }
            { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChain_UpdateStrands", &PROBE_N, &[], true); }
    r_
}

pub fn EftChain_DrawStrands(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChain_DrawStrands", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_p_3: u32 = 0;
    let mut l_ch_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_p_3 = c.m.r32((l_w_2).wrapping_add(52));
    l_ch_4 = (l_w_2).wrapping_add(320);
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((l_i_5) < (16i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((c.m.r32((l_ch_4).wrapping_add(104)) as i32)) & (((2048i32) | (1i32))))) != 0) {
                    if ((((c.m.r32(l_p_3)) & (((2i32) as u32)))) != 0) {
                        F::EftChain_DrawStrandClipped(c, l_w_2, l_w_2, l_ch_4);
                    } else {
                        F::EftChain_DrawStrand(c, l_w_2, l_w_2, l_ch_4);
                    }
                }
            }
            { { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; }; { let o_ = l_ch_4; let n_ = (o_).wrapping_add(208); l_ch_4 = n_; }; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChain_DrawStrands", &PROBE_N, &[], true); }
    r_
}

pub fn EftChain_Spawn(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChain_Spawn", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_key_3: u32 = 0;
    let mut l_p_4: u32 = 0;
    let mut l_full_5: i32 = 0;
    let mut l_ch_6: u32 = 0;
    let mut l_n_7: i32 = 0;
    let mut l_i_8: i32 = 0;
    let mut l_a_9: F32 = F32(0);
    let mut l_b_10: F32 = F32(0);
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_key_3 = (l_w_2).wrapping_add(64);
    l_p_4 = c.m.r32((l_w_2).wrapping_add(52));
    l_full_5 = 0i32;
    if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(3740))), F32(0x00000000)) as i32)) != 0) {
        'l1: loop {
            'c2: {
                l_n_7 = ((c.m.r8((l_p_4).wrapping_add(8))) as i32);
                if (((((((c.m.r8((l_p_4).wrapping_add(9))) as i32)) >= (2i32)) as i32)) != 0) {
                    { let r_ = rt::rem_i32(F::Port_Rand(c), ((c.m.r8((l_p_4).wrapping_add(9))) as i32)); let n_ = (l_n_7).wrapping_add(r_); l_n_7 = n_; };
                }
                {
                    'l3: loop {
                        if !(((((l_n_7) > (0i32)) as i32)) != 0) { break 'l3; }
                        'c4: {
                            l_full_5 = 1i32;
                            l_ch_6 = ((0i32) as u32);
                            {
                                l_i_8 = 0i32;
                                'l5: loop {
                                    if !(((((l_i_8) < (16i32)) as i32)) != 0) { break 'l5; }
                                    'c6: {
                                        if ((((((c.m.r32((((l_w_2).wrapping_add(320)).wrapping_add((((l_i_8) as u32)).wrapping_mul(208))).wrapping_add(104)) as i32)) == (0i32)) as i32)) != 0) {
                                            l_ch_6 = ((l_w_2).wrapping_add(320)).wrapping_add((((l_i_8) as u32)).wrapping_mul(208));
                                            { let _ = F::memset(c, l_ch_6, 0i32, 208u32); };
                                            l_full_5 = 0i32;
                                            break 'l5;
                                        }
                                    }
                                    { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
                                }
                            }
                            if ((l_full_5) != 0) {
                                break 'l3;
                            }
                            if ((((!((F::EftChain_StartStrand(c, p_task_1, l_ch_6)) != 0)) as i32)) != 0) {
                                break 'l3;
                            }
                            l_a_9 = rt::fadd(F32(c.m.r32((l_key_3).wrapping_add(96))), rt::fmul(F32(c.m.r32((l_key_3).wrapping_add(100))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff))));
                            l_b_10 = rt::fadd(F32(c.m.r32((l_key_3).wrapping_add(104))), rt::fmul(F32(c.m.r32((l_key_3).wrapping_add(108))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff))));
                            if ((((!(((((c.m.r32((l_w_2).wrapping_add(3724)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
                                { let r_ = l_a_9; let a_ = ((l_w_2).wrapping_add(3772)).wrapping_add(((((c.m.r32((l_w_2).wrapping_add(3724)) as i32)) as u32)).wrapping_mul(4)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                { let r_ = l_b_10; let a_ = ((l_w_2).wrapping_add(3788)).wrapping_add(((((c.m.r32((l_w_2).wrapping_add(3724)) as i32)) as u32)).wrapping_mul(4)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            } else {
                                { let r_ = l_a_9; let a_ = ((l_w_2).wrapping_add(3772)).wrapping_add(((((c.m.r32((l_w_2).wrapping_add(3724)) as i32)) as u32)).wrapping_mul(4)); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                { let r_ = l_b_10; let a_ = ((l_w_2).wrapping_add(3788)).wrapping_add(((((c.m.r32((l_w_2).wrapping_add(3724)) as i32)) as u32)).wrapping_mul(4)); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            }
                            { let v_ = { let a0_ = { let a_ = ((l_w_2).wrapping_add(3772)).wrapping_add(((((c.m.r32((l_w_2).wrapping_add(3724)) as i32)) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; F::EftMath_WrapAngle(c, a0_) }; let a_ = ((l_w_2).wrapping_add(3772)).wrapping_add(((((c.m.r32((l_w_2).wrapping_add(3724)) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                            { let v_ = { let a0_ = { let a_ = ((l_w_2).wrapping_add(3788)).wrapping_add(((((c.m.r32((l_w_2).wrapping_add(3724)) as i32)) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; F::EftMath_WrapAngle(c, a0_) }; let a_ = ((l_w_2).wrapping_add(3788)).wrapping_add(((((c.m.r32((l_w_2).wrapping_add(3724)) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                            { let a_ = (l_w_2).wrapping_add(3724); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            if ((((((c.m.r32((l_w_2).wrapping_add(3724)) as i32)) >= (4i32)) as i32)) != 0) {
                                { let v_ = 0i32; let a_ = (l_w_2).wrapping_add(3724); c.m.w32(a_, (v_) as u32); };
                            }
                        }
                        { let o_ = l_n_7; let n_ = (o_).wrapping_sub(1); l_n_7 = n_; };
                    }
                }
                { let r_ = F32(c.m.r32((l_key_3).wrapping_add(92))); let a_ = (l_w_2).wrapping_add(3740); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            }
            if !((((((((!((l_full_5) != 0)) as i32)) != 0) && (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(3740))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) { break 'l1; }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChain_Spawn", &PROBE_N, &[], true); }
    r_
}

pub fn EftChain_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChain_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let mut l_p_4: u32 = 0;
    let mut l_key_5: u32 = 0;
    let mut l_life_6: F32 = F32(0);
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    l_p_4 = c.m.r32((p_arg_2).wrapping_add(52));
    l_key_5 = (l_w_3).wrapping_add(64);
    { let _ = F::memset(c, l_w_3, 0i32, 3856u32); };
    { let s_ = p_arg_2; let d_ = l_w_3; rt::memcpy(c, d_, s_, 64); };
    { let r_ = 1i32; let a_ = (l_w_3).wrapping_add(3712); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    if (((((((((((c.m.r8((l_p_4).wrapping_add(4))) as i32)) == (2i32)) as i32)) != 0) || ((((c.m.r32(l_p_4)) & (((16i32) as u32)))) != 0)) as i32)) != 0) {
        { let r_ = 32i32; let a_ = (l_w_3).wrapping_add(3712); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    l_life_6 = F32(c.m.r32((p_arg_2).wrapping_add(40)));
    if (((rt::fle(l_life_6, F32(0x00000000)) as i32)) != 0) {
        { let r_ = 8i32; let a_ = (l_w_3).wrapping_add(3712); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        { let v_ = F32(0x00000000); let a_ = (l_w_3).wrapping_add(3732); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(l_life_6, F32(0x41f00000)); let a_ = (l_w_3).wrapping_add(3736); c.m.w32(a_, (v_).0); };
    }
    F::Vec3_Clamp(c, (p_arg_2).wrapping_add(16), (p_arg_2).wrapping_add(16), F32(0xbf800000), F32(0x3f800000));
    { let v_ = { let a0_ = rt::fneg(F32(c.m.r32(((p_arg_2).wrapping_add(16)).wrapping_add(4)))); F::Mathf_Asin(c, a0_) }; let a_ = (l_w_3).wrapping_add(3764); c.m.w32(a_, (v_).0); };
    { let v_ = { let a0_ = F32(c.m.r32((p_arg_2).wrapping_add(16))); let a1_ = F32(c.m.r32(((p_arg_2).wrapping_add(16)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) }; let a_ = (l_w_3).wrapping_add(3768); c.m.w32(a_, (v_).0); };
    F::Mtx_StoreIdentity(c, (l_w_3).wrapping_add(3648));
    { let a2_ = F32(c.m.r32((l_w_3).wrapping_add(3764))); F::Mtx_RotateX(c, (l_w_3).wrapping_add(3648), (l_w_3).wrapping_add(3648), a2_) };
    { let a2_ = F32(c.m.r32((l_w_3).wrapping_add(3768))); F::Mtx_RotateY(c, (l_w_3).wrapping_add(3648), (l_w_3).wrapping_add(3648), a2_) };
    { let v_ = F32(0x00000000); let a_ = ((l_w_3).wrapping_add(3772)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = ((l_w_3).wrapping_add(3788)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x40490fda); let a_ = ((l_w_3).wrapping_add(3772)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = ((l_w_3).wrapping_add(3788)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = ((l_w_3).wrapping_add(3772)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x40490fda); let a_ = ((l_w_3).wrapping_add(3788)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x40490fda); let a_ = ((l_w_3).wrapping_add(3772)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x40490fda); let a_ = ((l_w_3).wrapping_add(3788)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    if ((((c.m.r32(l_p_4)) & (((1i32) as u32)))) != 0) {
        { let v_ = rt::fmul(F32(c.m.r32((l_p_4).wrapping_add(12))), F32(0x41f00000)); let a_ = (l_w_3).wrapping_add(3756); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32((l_w_3).wrapping_add(3756))), F32(c.m.r32((l_p_4).wrapping_add(16)))); let a_ = (l_w_3).wrapping_add(3760); c.m.w32(a_, (v_).0); };
        F::EftChain_SetKey(c, l_key_5, l_w_3, 0i32);
    } else {
        F::EftChain_SetKey(c, l_key_5, l_w_3, 2i32);
    }
    { let v_ = rt::f2i(F32(c.m.r32((l_key_5).wrapping_add(120)))); let a_ = (l_w_3).wrapping_add(3728); c.m.w32(a_, (v_) as u32); };
    { let a1_ = c.m.r32((p_arg_2).wrapping_add(48)); let a2_ = (c.m.r32((p_arg_2).wrapping_add(36)) as i32); let a3_ = (c.m.r32((p_arg_2).wrapping_add(36)) as i32); F::EftChain_SetTex(c, l_w_3, a1_, a2_, a3_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChain_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftChain_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChain_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_ch_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    { let v_ = 0i32; let a_ = (l_w_2).wrapping_add(3712); c.m.w32(a_, (v_) as u32); };
    l_ch_3 = (l_w_2).wrapping_add(320);
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (16i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                F::EftChain_FreeNodes(c, l_ch_3);
                { let v_ = 0i32; let a_ = (l_ch_3).wrapping_add(104); c.m.w32(a_, (v_) as u32); };
                { let o_ = l_ch_3; let n_ = (o_).wrapping_add(208); l_ch_3 = n_; };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChain_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftChain_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChain_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_busy_3: i32 = 0;
    let mut l_p_4: u32 = 0;
    let mut l_key_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_busy_3 = 0i32;
    l_p_4 = c.m.r32((l_w_2).wrapping_add(52));
    if ((((!(({ let a0_ = (c.m.r32((l_w_2).wrapping_add(32)) as i32); let a1_ = ((c.m.r8((l_w_2).wrapping_add(3716))) as i32); F::BtlScene_IsEffectStopped(c, a0_, a1_) }) != 0)) as i32)) != 0) {
        if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(3744))), F32(0x00000000)) as i32)) != 0) {
            l_key_5 = (l_w_2).wrapping_add(64);
            if ((((((((!(((((c.m.r32((l_w_2).wrapping_add(3712)) as i32)) & (2i32))) != 0)) as i32)) != 0) || (((rt::fgt(F32(c.m.r32((l_w_2).wrapping_add(3748))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(3740))), F32(0x00000000)) as i32)) != 0) {
                    F::EftChain_Spawn(c, p_task_1);
                } else {
                    { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(3740); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                }
            }
            F::EftChain_UpdateStrands(c, p_task_1);
            if ((((((((c.m.r32(l_p_4)) & (((1i32) as u32)))) != 0) && (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(3752))), F32(c.m.r32((l_w_2).wrapping_add(3756)))) as i32)) != 0)) as i32)) != 0) {
                F::EftChain_BlendKeys(c, l_w_2);
                { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(3752); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(3756))), F32(c.m.r32((l_w_2).wrapping_add(3752)))) as i32)) != 0) {
                    F::EftChain_SetKey(c, l_key_5, l_w_2, 2i32);
                }
            }
            if ((((!(((((c.m.r32((l_w_2).wrapping_add(3712)) as i32)) & (8i32))) != 0)) as i32)) != 0) {
                { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(3732); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(3736))), F32(c.m.r32((l_w_2).wrapping_add(3732)))) as i32)) != 0) {
                    { let r_ = 2i32; let a_ = (l_w_2).wrapping_add(3712); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
        } else {
            { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(3744); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
        if (((((c.m.r32((l_w_2).wrapping_add(3712)) as i32)) & (2i32))) != 0) {
            if (((rt::fgt(F32(c.m.r32((l_w_2).wrapping_add(3748))), F32(0x00000000)) as i32)) != 0) {
                { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(3748); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            }
            if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(3748))), F32(0x00000000)) as i32)) != 0) {
                {
                    l_i_6 = 0i32;
                    'l1: loop {
                        if !(((((l_i_6) < (16i32)) as i32)) != 0) { break 'l1; }
                        'c2: {
                            if ((((((c.m.r32((((l_w_2).wrapping_add(320)).wrapping_add((((l_i_6) as u32)).wrapping_mul(208))).wrapping_add(104)) as i32)) != (0i32)) as i32)) != 0) {
                                l_busy_3 = 1i32;
                                break 'l1;
                            }
                        }
                        { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                    }
                }
                if ((((!((l_busy_3) != 0)) as i32)) != 0) {
                    { let r_ = 4i32; let a_ = (l_w_2).wrapping_add(3712); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
        }
    }
    if (((((c.m.r32((l_w_2).wrapping_add(3712)) as i32)) & (4i32))) != 0) {
        F::BtlTask_SetDead(c, p_task_1);
    } else {
        F::EftChain_BuildTex(c, l_w_2, l_w_2);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChain_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftChain_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChain_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChain_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftChain_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChain_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let _ = D::call(c, fid::BtlTask_SetDead, &[((p_task_1) as u64)]); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChain_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftChain_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChain_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (({ let a0_ = (c.m.r32((l_w_2).wrapping_add(32)) as i32); let a1_ = ((c.m.r8((l_w_2).wrapping_add(3716))) as i32); F::BtlScene_IsEffectHidden(c, a0_, a1_) }) != 0) {
        break 'f;
    }
    if (((((c.m.r32((l_w_2).wrapping_add(3712)) as i32)) & (16i32))) != 0) {
        if ((((!(({ let a0_ = (c.m.r32((l_w_2).wrapping_add(32)) as i32); F::BtlScene_IsCharInView(c, a0_) }) != 0)) as i32)) != 0) {
            break 'f;
        }
    }
    F::Vu0Cur_Push(c);
    { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
    { let _ = D::call(c, fid::EftChain_DrawStrands, &[((p_task_1) as u64)]); };
    F::Vu0Cur_Pop(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChain_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftChainMgr_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChainMgr_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((8u32) as i32)) }; let a_ = sym::gEftChain; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gEftChain); F::memset(c, a0_, 0i32, 8u32) }; };
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, 48000i32) }; let a_ = c.m.r32(sym::gEftChain); c.m.w32(a_, v_); };
    { let _ = { let a0_ = { let a_ = c.m.r32(sym::gEftChain); c.m.r32(a_) }; F::memset(c, a0_, 0i32, ((48000i32) as u32)) }; };
    { let v_ = ((D::call(c, fid::BtlTask_CreateChildList, &[((p_task_1) as u64), ((10i32) as i64 as u64), ((((3856u32) as i32)) as i64 as u64)])) as u32); let a_ = sym::gEftChainList; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChainMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftChainMgr_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChainMgr_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = { let a_ = c.m.r32(sym::gEftChain); c.m.r32(a_) }; F::BtlPool_Free(c, a0_, a1_) };
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftChain); F::BtlPool_Free(c, a0_, a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftChain; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChainMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftChainMgr_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChainMgr_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChainMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftChainMgr_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChainMgr_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChainMgr_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftChain_Create(c: &mut Ctx, p_arg_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChain_Create", &PROBE_N, &[(p_arg_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    rt::memcpy(c, fp, p_arg_1, 64);
    let r_ = 'f: {
        break 'f { let a0_ = c.m.r32(sym::gEftChainList); ((D::call(c, fid::BtlTaskList_AddTail, &[((a0_) as u64), ((sym::D_002C3E48) as u64), ((fp) as u64)])) as u32) };
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftChain_Create", &PROBE_N, &[], true); }
    r_
}

pub fn EftChain_Stop(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChain_Stop", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftChain_Update)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(3712)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let r_ = 2i32; let a_ = (l_w_2).wrapping_add(3712); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChain_Stop", &PROBE_N, &[], true); }
    r_
}

pub fn EftChain_Kill(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChain_Kill", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftChain_Update)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(3712)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let r_ = 4i32; let a_ = (l_w_2).wrapping_add(3712); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChain_Kill", &PROBE_N, &[], true); }
    r_
}

pub fn EftChain_IsAlive(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChain_IsAlive", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftChain_Update)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32((l_w_2).wrapping_add(3712)) as i32)) & (1i32))) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChain_IsAlive", &PROBE_N, &[], true); }
    r_
}

pub fn EftChain_SetPos(c: &mut Ctx, mut p_task_1: u32, mut p_pos_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChain_SetPos", &PROBE_N, &[(p_task_1) as u32, (p_pos_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftChain_Update)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(3712)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    F::Vec3_Copy(c, l_w_3, p_pos_2);
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChain_SetPos", &PROBE_N, &[], true); }
    r_
}

pub fn EftChain_Warp(c: &mut Ctx, mut p_task_1: u32, mut p_pos_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChain_Warp", &PROBE_N, &[(p_task_1) as u32, (p_pos_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let mut l_st_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_m_6: u32 = 0;
    let mut l_link_7: u32 = 0;
    let mut l_pp_8: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftChain_Update)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(3712)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    F::Vec3_Copy(c, l_w_3, p_pos_2);
    l_st_4 = l_w_3;
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((l_i_5) < (16i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_m_6 = (l_st_4).wrapping_add(((320i32) as u32));
                if (((((c.m.r32(((l_st_4).wrapping_add(((320i32) as u32))).wrapping_add(104)) as i32)) & (1i32))) != 0) {
                    l_pp_8 = ((l_st_4).wrapping_add(((320i32) as u32))).wrapping_add(96);
                    'l3: while (((((c.m.r32(l_pp_8)) != (((0i32) as u32))) as i32)) != 0) {
                        l_link_7 = c.m.r32(l_pp_8);
                        { let _ = D::call(c, fid::Vec3_Add4, &[(((l_link_7).wrapping_add(32)) as u64), ((l_w_3) as u64), ((l_m_6) as u64), ((l_link_7) as u64), (((l_link_7).wrapping_add(16)) as u64)]); };
                        { let v_ = F32(0x3f800000); let a_ = ((l_link_7).wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        l_pp_8 = (l_link_7).wrapping_add(84);
                    }
                }
            }
            { { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; }; { let o_ = l_st_4; let n_ = (o_).wrapping_add(208); l_st_4 = n_; }; };
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChain_Warp", &PROBE_N, &[], true); }
    r_
}

pub fn EftChain_SetDir(c: &mut Ctx, mut p_task_1: u32, mut p_dir_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChain_SetDir", &PROBE_N, &[(p_task_1) as u32, (p_dir_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftChain_Update)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(3712)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    F::Vec3_Copy(c, (l_w_3).wrapping_add(16), p_dir_2);
    { let v_ = { let a0_ = rt::fneg(F32(c.m.r32(((l_w_3).wrapping_add(16)).wrapping_add(4)))); F::Mathf_Asin(c, a0_) }; let a_ = (l_w_3).wrapping_add(3764); c.m.w32(a_, (v_).0); };
    { let v_ = { let a0_ = F32(c.m.r32((l_w_3).wrapping_add(16))); let a1_ = F32(c.m.r32(((l_w_3).wrapping_add(16)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) }; let a_ = (l_w_3).wrapping_add(3768); c.m.w32(a_, (v_).0); };
    F::Mtx_StoreIdentity(c, (l_w_3).wrapping_add(3648));
    { let a2_ = F32(c.m.r32((l_w_3).wrapping_add(3764))); F::Mtx_RotateX(c, (l_w_3).wrapping_add(3648), (l_w_3).wrapping_add(3648), a2_) };
    { let a2_ = F32(c.m.r32((l_w_3).wrapping_add(3768))); F::Mtx_RotateY(c, (l_w_3).wrapping_add(3648), (l_w_3).wrapping_add(3648), a2_) };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChain_SetDir", &PROBE_N, &[], true); }
    r_
}

pub fn EftChain_SetSize(c: &mut Ctx, mut p_task_1: u32, mut p_size_2: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChain_SetSize", &PROBE_N, &[(p_task_1) as u32, (p_size_2).0], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftChain_Update)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(3712)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let v_ = p_size_2; let a_ = (l_w_3).wrapping_add(44); c.m.w32(a_, (v_).0); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChain_SetSize", &PROBE_N, &[], true); }
    r_
}

pub fn EftChain_SetParam3(c: &mut Ctx, mut p_task_1: u32, mut p_v_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChain_SetParam3", &PROBE_N, &[(p_task_1) as u32, (p_v_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftChain_Update)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(3712)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let v_ = p_v_2; let a_ = (l_w_3).wrapping_add(3728); c.m.w32(a_, (v_) as u32); };
    if (((((p_v_2) < (3i32)) as i32)) != 0) {
        { let v_ = 3i32; let a_ = (l_w_3).wrapping_add(3728); c.m.w32(a_, (v_) as u32); };
    } else {
        if (((((p_v_2) >= (8i32)) as i32)) != 0) {
            { let v_ = 7i32; let a_ = (l_w_3).wrapping_add(3728); c.m.w32(a_, (v_) as u32); };
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChain_SetParam3", &PROBE_N, &[], true); }
    r_
}

pub fn EftChain_SetParam5(c: &mut Ctx, mut p_task_1: u32, mut p_v_2: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChain_SetParam5", &PROBE_N, &[(p_task_1) as u32, (p_v_2).0], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftChain_Update)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(3712)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let v_ = p_v_2; let a_ = (l_w_3).wrapping_add(3744); c.m.w32(a_, (v_).0); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChain_SetParam5", &PROBE_N, &[], true); }
    r_
}

pub fn EftChain_SetParam6(c: &mut Ctx, mut p_task_1: u32, mut p_v_2: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChain_SetParam6", &PROBE_N, &[(p_task_1) as u32, (p_v_2).0], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftChain_Update)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(3712)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let v_ = p_v_2; let a_ = (l_w_3).wrapping_add(3748); c.m.w32(a_, (v_).0); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChain_SetParam6", &PROBE_N, &[], true); }
    r_
}

pub fn EftChain_SetRes(c: &mut Ctx, mut p_task_1: u32, mut p_a1_2: i32, mut p_a2_3: i32, mut p_a3_4: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChain_SetRes", &PROBE_N, &[(p_task_1) as u32, (p_a1_2) as u32, (p_a2_3) as u32, (p_a3_4) as u32], false); }
    let mut l_w_5: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftChain_Update)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_5 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_5).wrapping_add(3712)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let _ = D::call(c, fid::EftChain_SetTex, &[((l_w_5) as u64), ((p_a1_2) as i64 as u64), ((p_a2_3) as i64 as u64), ((p_a3_4) as i64 as u64)]); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChain_SetRes", &PROBE_N, &[], true); }
    r_
}

pub fn EftChain_SetViewOnly(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChain_SetViewOnly", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftChain_Update)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(3712)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let r_ = 16i32; let a_ = (l_w_2).wrapping_add(3712); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChain_SetViewOnly", &PROBE_N, &[], true); }
    r_
}

pub fn EftChain_SetType(c: &mut Ctx, mut p_task_1: u32, mut p_type_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChain_SetType", &PROBE_N, &[(p_task_1) as u32, (p_type_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftChain_Update)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(3712)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let v_ = ((p_type_2) as u8); let a_ = (l_w_3).wrapping_add(3716); c.m.w8(a_, v_); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChain_SetType", &PROBE_N, &[], true); }
    r_
}

pub fn EftRay_Create(c: &mut Ctx, mut p_arg_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRay_Create", &PROBE_N, &[(p_arg_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let r_ = 'f: {
        if (((((c.m.r32(sym::gEftRay)) == (((0i32) as u32))) as i32)) != 0) {
            break 'f ((0i32) as u32);
        }
        { let s_ = p_arg_1; let d_ = fp; rt::memcpy(c, d_, s_, 96); };
        break 'f { let a0_ = { let a_ = (c.m.r32(sym::gEftRay)).wrapping_add(520); c.m.r32(a_) }; ((D::call(c, fid::BtlTaskList_AddTail, &[((a0_) as u64), ((sym::gEftRayClass) as u64), ((fp) as u64)])) as u32) };
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftRay_Create", &PROBE_N, &[], true); }
    r_
}

pub fn EftRay_SetPos(c: &mut Ctx, mut p_task_1: u32, p_pos_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRay_SetPos", &PROBE_N, &[(p_task_1) as u32, (p_pos_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_3: u32 = 0;
    rt::memcpy(c, fp, p_pos_2, 16);
    let r_ = 'f: {
        if (((((c.m.r32(sym::gEftRay)) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((((((c.m.r8(p_task_1)) as i32)) & (1i32))) != 0) {
            break 'f 0i32;
        }
        if ((((!((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftRay_Update)) as i32)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        if (((((((((c.m.r32((l_w_3).wrapping_add(1300)) as i32)) & (4i32))) != 0) || (((((c.m.r32((l_w_3).wrapping_add(1300)) as i32)) & (8i32))) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        F::Vec4_Copy(c, (l_w_3).wrapping_add(1152), fp);
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftRay_SetPos", &PROBE_N, &[], true); }
    r_
}

pub fn EftRay_Kill(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRay_Kill", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftRay)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((c.m.r8(p_task_1)) as i32)) & (1i32))) != 0) {
        break 'f 0i32;
    }
    if ((((!((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftRay_Update)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    { let r_ = ((2i32) | (16i32)); let a_ = (l_w_2).wrapping_add(1300); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRay_Kill", &PROBE_N, &[], true); }
    r_
}

pub fn EftRay_SetType(c: &mut Ctx, mut p_task_1: u32, mut p_type_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRay_SetType", &PROBE_N, &[(p_task_1) as u32, (p_type_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftRay)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((c.m.r8(p_task_1)) as i32)) & (1i32))) != 0) {
        break 'f 0i32;
    }
    if ((((!((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftRay_Update)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let v_ = ((p_type_2) as u8); let a_ = (l_w_3).wrapping_add(1304); c.m.w8(a_, v_); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRay_SetType", &PROBE_N, &[], true); }
    r_
}

pub fn EftRay_IsAlive(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRay_IsAlive", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftRay)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((c.m.r8(p_task_1)) as i32)) & (1i32))) != 0) {
        break 'f 0i32;
    }
    if ((((!((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftRay_Update)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((c.m.r32((l_w_2).wrapping_add(1300)) as i32)) & (4i32))) != 0) {
        { let r_ = 8i32; let a_ = (l_w_2).wrapping_add(1300); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        break 'f 0i32;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRay_IsAlive", &PROBE_N, &[], true); }
    r_
}

pub fn EftRay_StartHitDefault(c: &mut Ctx, mut p_objId_1: i32, p_pos_2: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRay_StartHitDefault", &PROBE_N, &[(p_objId_1) as u32, (p_pos_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    rt::memcpy(c, fp, p_pos_2, 16);
    let r_ = 'f: {
        break 'f F::EftRay_StartHit(c, p_objId_1, fp, 0i32, 10i32, 0i32, F32(0x3f000000));
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftRay_StartHitDefault", &PROBE_N, &[], true); }
    r_
}

pub fn EftRay_StartHit(c: &mut Ctx, mut p_objId_1: i32, p_pos_2: u32, mut p_delay_3: i32, mut p_fade_4: i32, mut p_unused_5: i32, mut p_life_6: F32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRay_StartHit", &PROBE_N, &[(p_objId_1) as u32, (p_pos_2) as u32, (p_delay_3) as u32, (p_fade_4) as u32, (p_unused_5) as u32, (p_life_6).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    rt::memcpy(c, fp, p_pos_2, 16);
    let r_ = 'f: {
        F::Vec4_Copy(c, fp.wrapping_add(16), fp);
        { let v_ = 192i32; let a_ = ((fp.wrapping_add(16)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = rt::f2i(rt::i2f(p_delay_3)); let a_ = (fp.wrapping_add(16)).wrapping_add(72); c.m.w32(a_, (v_) as u32); };
        { let v_ = rt::f2i(rt::i2f(p_fade_4)); let a_ = (fp.wrapping_add(16)).wrapping_add(76); c.m.w32(a_, (v_) as u32); };
        { let v_ = 192i32; let a_ = ((fp.wrapping_add(16)).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 192i32; let a_ = ((fp.wrapping_add(16)).wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 112i32; let a_ = ((fp.wrapping_add(16)).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = F32(0x42700000); let a_ = (fp.wrapping_add(16)).wrapping_add(36); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3e999999); let a_ = (fp.wrapping_add(16)).wrapping_add(40); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0xbf999999); let a_ = (fp.wrapping_add(16)).wrapping_add(44); c.m.w32(a_, (v_).0); };
        { let v_ = p_life_6; let a_ = (fp.wrapping_add(16)).wrapping_add(32); c.m.w32(a_, (v_).0); };
        { let v_ = p_objId_1; let a_ = (fp.wrapping_add(16)).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
        { let v_ = F32(0x3e999999); let a_ = (fp.wrapping_add(16)).wrapping_add(48); c.m.w32(a_, (v_).0); };
        { let v_ = 1i32; let a_ = (fp.wrapping_add(16)).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((36i32) as u32); let a_ = (fp.wrapping_add(16)).wrapping_add(56); c.m.w32(a_, v_); };
        { let v_ = 1i32; let a_ = (fp.wrapping_add(16)).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp.wrapping_add(16)).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
        { let v_ = 1i32; let a_ = (fp.wrapping_add(16)).wrapping_add(80); c.m.w32(a_, (v_) as u32); };
        { let v_ = F32(c.m.r32(fp)); let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp).wrapping_add(4))); let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp).wrapping_add(8))); let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp).wrapping_add(12))); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        break 'f F::EftRay_Create(c, fp.wrapping_add(16));
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftRay_StartHit", &PROBE_N, &[], true); }
    r_
}

pub fn EftRay_StartScreen(c: &mut Ctx, mut p_objId_1: i32, mut p_delay_2: i32, mut p_fade_3: i32, mut p_life_4: F32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRay_StartScreen", &PROBE_N, &[(p_objId_1) as u32, (p_delay_2) as u32, (p_fade_3) as u32, (p_life_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let r_ = 'f: {
        F::Vec4_Set(c, fp, F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x3f800000));
        { let v_ = 192i32; let a_ = ((fp).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 192i32; let a_ = ((fp).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 192i32; let a_ = ((fp).wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = ((fp).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = p_life_4; let a_ = (fp).wrapping_add(32); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x44480000); let a_ = (fp).wrapping_add(36); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3fc00000); let a_ = (fp).wrapping_add(40); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(44); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(48); c.m.w32(a_, (v_).0); };
        { let v_ = 3i32; let a_ = (fp).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((36i32) as u32); let a_ = (fp).wrapping_add(56); c.m.w32(a_, v_); };
        { let v_ = p_objId_1; let a_ = (fp).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
        { let v_ = 1i32; let a_ = (fp).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
        { let v_ = 1i32; let a_ = (fp).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
        { let v_ = rt::f2i(rt::i2f(p_delay_2)); let a_ = (fp).wrapping_add(72); c.m.w32(a_, (v_) as u32); };
        { let v_ = rt::f2i(rt::i2f(p_fade_3)); let a_ = (fp).wrapping_add(76); c.m.w32(a_, (v_) as u32); };
        { let v_ = 1i32; let a_ = (fp).wrapping_add(80); c.m.w32(a_, (v_) as u32); };
        break 'f F::EftRay_CreateByValue(c, fp);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftRay_StartScreen", &PROBE_N, &[], true); }
    r_
}

pub fn EftRay_CreateByValue(c: &mut Ctx, p_arg_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRay_CreateByValue", &PROBE_N, &[(p_arg_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    rt::memcpy(c, fp, p_arg_1, 96);
    let r_ = 'f: {
        break 'f F::EftRay_Create(c, fp);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftRay_CreateByValue", &PROBE_N, &[], true); }
    r_
}

pub fn EftRayMgr_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRayMgr_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((528u32) as i32)) }; let a_ = sym::gEftRay; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gEftRay); F::memset(c, a0_, 0i32, 528u32) }; };
    { let a0_ = c.m.r32(sym::gEftRay); let a1_ = F::BtlScene_GetCommonEntry(c, 10i32); F::EftTexSet_Load32(c, a0_, a1_) };
    { let v_ = ((D::call(c, fid::BtlTask_CreateChildList, &[((p_task_1) as u64), ((4i32) as i64 as u64), ((((1312u32) as i32)) as i64 as u64)])) as u32); let a_ = (c.m.r32(sym::gEftRay)).wrapping_add(520); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRayMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftRayMgr_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRayMgr_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftRay); F::BtlPool_Free(c, a0_, a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftRay; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRayMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftRayMgr_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRayMgr_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(36))).wrapping_add(4); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
        F::EftRay_StepTexture(c);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRayMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftRay_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRay_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let _ = F::memset(c, l_w_3, 0i32, 1312u32); };
    { let v_ = ((2i32) as u8); let a_ = (l_w_3).wrapping_add(1304); c.m.w8(a_, v_); };
    F::EftRay_Setup(c, l_w_3, p_arg_2);
    if ((((((((c.m.r32((p_arg_2).wrapping_add(60)) as i32)) as u32)) < (((2i32) as u32))) as i32)) != 0) {
        { let a1_ = (if ((((((c.m.r32((p_arg_2).wrapping_add(60)) as i32)) == (0i32)) as i32)) != 0) { 2048i32 } else { 4096i32 }); F::BtlTask_SetOwnerTag(c, p_task_1, a1_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRay_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftRay_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRay_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let _ = { let a0_ = c.m.r32((p_task_1).wrapping_add(56)); F::memset(c, a0_, 0i32, 1312u32) }; };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRay_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftRay_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRay_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_ray_3: u32 = 0;
    let mut l_turn_4: F32 = F32(0);
    let mut l_i_5: i32 = 0;
    let mut l_n_6: i32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (({ let a0_ = (c.m.r32((l_w_2).wrapping_add(1292)) as i32); let a1_ = ((c.m.r8((l_w_2).wrapping_add(1304))) as i32); F::BtlScene_IsEffectStopped(c, a0_, a1_) }) != 0) {
        break 'f;
    }
    if ((((((c.m.r32((l_w_2).wrapping_add(1296)) as i32)) > (0i32)) as i32)) != 0) {
        { let a_ = (l_w_2).wrapping_add(1296); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
        break 'f;
    }
    if (((((c.m.r32((l_w_2).wrapping_add(1300)) as i32)) & (1i32))) != 0) {
        { let r_ = F32(0x3d088888); let a_ = (l_w_2).wrapping_add(1252); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if (((rt::flt(F32(c.m.r32((l_w_2).wrapping_add(1252))), F32(0x00000000)) as i32)) != 0) {
            { let v_ = F32(0x00000000); let a_ = (l_w_2).wrapping_add(1252); c.m.w32(a_, (v_).0); };
            { let r_ = 2i32; let a_ = (l_w_2).wrapping_add(1300); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
    }
    if (((((c.m.r32((l_w_2).wrapping_add(1300)) as i32)) & (2i32))) != 0) {
        { let r_ = F32(c.m.r32((l_w_2).wrapping_add(1260))); let a_ = (l_w_2).wrapping_add(1256); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if (((rt::flt(F32(c.m.r32((l_w_2).wrapping_add(1256))), F32(0x00000000)) as i32)) != 0) {
            { let v_ = F32(0x00000000); let a_ = (l_w_2).wrapping_add(1256); c.m.w32(a_, (v_).0); };
            { let r_ = 4i32; let a_ = (l_w_2).wrapping_add(1300); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            if (((((c.m.r32((l_w_2).wrapping_add(1300)) as i32)) & (16i32))) != 0) {
                { let r_ = 8i32; let a_ = (l_w_2).wrapping_add(1300); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
    }
    if (((((c.m.r32((l_w_2).wrapping_add(1300)) as i32)) & (8i32))) != 0) {
        { let _ = D::call(c, fid::BtlTask_SetDead, &[((p_task_1) as u64)]); };
        break 'f;
    }
    's1: {
        let sv7: u32 = c.m.r32((l_w_2).wrapping_add(1288));
        match sv7 {
            0u32 => {
                { let r_ = F32(c.m.r32((l_w_2).wrapping_add(1248))); let a_ = (l_w_2).wrapping_add(1168); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::flt(F32(c.m.r32((l_w_2).wrapping_add(1168))), F32(0x00000000)) as i32)) != 0) {
                    { let v_ = F32(0x00000000); let a_ = (l_w_2).wrapping_add(1168); c.m.w32(a_, (v_).0); };
                }
                break 's1;
            }
            1u32 => {
                l_turn_4 = { let a0_ = rt::fdiv(rt::fmul(rt::i2f(rt::rem_i32(F::Port_Rand(c), 360i32)), F32(0x40490fda)), F32(0x43340000)); F::EftMath_WrapAngle(c, a0_) };
                l_ray_3 = l_w_2;
                {
                    l_i_5 = 0i32;
                    'l2: loop {
                        if !(((((l_i_5) < ((c.m.r32((l_w_2).wrapping_add(1276)) as i32))) as i32)) != 0) { break 'l2; }
                        'c3: {
                            { let v_ = ((rt::rem_i32(F::Port_Rand(c), (c.m.r32((l_w_2).wrapping_add(1272)) as i32))) as u8); let a_ = (l_ray_3).wrapping_add(3); c.m.w8(a_, v_); };
                            { let v_ = { let a0_ = rt::fadd(F32(c.m.r32((l_ray_3).wrapping_add(4))), l_turn_4); F::EftMath_WrapAngle(c, a0_) }; let a_ = (l_ray_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        }
                        { { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; }; { let o_ = l_ray_3; let n_ = (o_).wrapping_add(24); l_ray_3 = n_; }; };
                    }
                }
                break 's1;
            }
            2u32 => {
                {
                    l_n_6 = { let a0_ = (c.m.r32((l_w_2).wrapping_add(1292)) as i32); F::BtlCharApi_IsInTechnique(c, a0_) };
                    if ((((({ let a0_ = (c.m.r32((l_w_2).wrapping_add(1292)) as i32); F::BtlCharApi_GetOpponentObjId(c, a0_) }) >= (0i32)) as i32)) != 0) {
                        { let r_ = { let a0_ = { let a0_ = (c.m.r32((l_w_2).wrapping_add(1292)) as i32); F::BtlCharApi_GetOpponentObjId(c, a0_) }; F::BtlCharApi_IsInTechnique(c, a0_) }; let n_ = (l_n_6).wrapping_add(r_); l_n_6 = n_; };
                    }
                    if (((((l_n_6) != (0i32)) as i32)) != 0) {
                        { let v_ = F32(0xbf800000); let a_ = (l_w_2).wrapping_add(1252); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0xbf800000); let a_ = (l_w_2).wrapping_add(1256); c.m.w32(a_, (v_).0); };
                    }
                    l_turn_4 = { let a0_ = rt::fdiv(rt::fmul(rt::i2f(rt::rem_i32(F::Port_Rand(c), 360i32)), F32(0x40490fda)), F32(0x43340000)); F::EftMath_WrapAngle(c, a0_) };
                    l_ray_3 = l_w_2;
                    {
                        l_i_5 = 0i32;
                        'l4: loop {
                            if !(((((l_i_5) < ((c.m.r32((l_w_2).wrapping_add(1276)) as i32))) as i32)) != 0) { break 'l4; }
                            'c5: {
                                { let v_ = ((rt::rem_i32(F::Port_Rand(c), (c.m.r32((l_w_2).wrapping_add(1272)) as i32))) as u8); let a_ = (l_ray_3).wrapping_add(3); c.m.w8(a_, v_); };
                                { let v_ = { let a0_ = rt::fadd(F32(c.m.r32((l_ray_3).wrapping_add(4))), l_turn_4); F::EftMath_WrapAngle(c, a0_) }; let a_ = (l_ray_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            }
                            { { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; }; { let o_ = l_ray_3; let n_ = (o_).wrapping_add(24); l_ray_3 = n_; }; };
                        }
                    }
                    break 's1;
                }
            }
            3u32 => {
                if ((F::EftCam_IsActive(c)) != 0) {
                    { let v_ = F32(0xbf800000); let a_ = (l_w_2).wrapping_add(1252); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0xbf800000); let a_ = (l_w_2).wrapping_add(1256); c.m.w32(a_, (v_).0); };
                }
                l_turn_4 = { let a0_ = rt::fdiv(rt::fmul(rt::i2f(rt::rem_i32(F::Port_Rand(c), 360i32)), F32(0x40490fda)), F32(0x43340000)); F::EftMath_WrapAngle(c, a0_) };
                l_ray_3 = l_w_2;
                {
                    l_i_5 = 0i32;
                    'l6: loop {
                        if !(((((l_i_5) < ((c.m.r32((l_w_2).wrapping_add(1276)) as i32))) as i32)) != 0) { break 'l6; }
                        'c7: {
                            { let v_ = ((rt::rem_i32(F::Port_Rand(c), (c.m.r32((l_w_2).wrapping_add(1272)) as i32))) as u8); let a_ = (l_ray_3).wrapping_add(3); c.m.w8(a_, v_); };
                            { let v_ = { let a0_ = rt::fadd(F32(c.m.r32((l_ray_3).wrapping_add(4))), l_turn_4); F::EftMath_WrapAngle(c, a0_) }; let a_ = (l_ray_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        }
                        { { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; }; { let o_ = l_ray_3; let n_ = (o_).wrapping_add(24); l_ray_3 = n_; }; };
                    }
                }
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRay_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftRay_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRay_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let _ = D::call(c, fid::BtlTask_SetDead, &[((p_task_1) as u64)]); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRay_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftRay_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRay_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(({ let a0_ = (c.m.r32((l_w_2).wrapping_add(1292)) as i32); let a1_ = ((c.m.r8((l_w_2).wrapping_add(1304))) as i32); F::BtlScene_IsEffectHidden(c, a0_, a1_) }) != 0)) as i32)) != 0) {
        F::EftRay_DrawRays(c, l_w_2);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRay_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftRay_StepTexture(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRay_StepTexture", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32(sym::gEftRay); F::EftTexSet_Keep32(c, a0_, 1i32, 0i32) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRay_StepTexture", &PROBE_N, &[], true); }
    r_
}

pub fn EftRay_Setup(c: &mut Ctx, mut p_w_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRay_Setup", &PROBE_N, &[(p_w_1) as u32, (p_arg_2) as u32], false); }
    let mut l_ray_3: u32 = 0;
    let mut l_step_4: F32 = F32(0);
    let mut l_i_5: i32 = 0;
    let mut l_angle_6: F32 = F32(0);
    let r_ = 'f: {
    F::Vec4_Copy(c, (p_w_1).wrapping_add(1152), p_arg_2);
    { let v_ = F32(0x3f800000); let a_ = ((p_w_1).wrapping_add(1152)).wrapping_add(12); c.m.w32(a_, (v_).0); };
    F::Mtx_StoreIdentity(c, (p_w_1).wrapping_add(1184));
    F::Vec4_Set(c, (p_w_1).wrapping_add(1168), F32(0x3f800000), F32(0x3f800000), F32(0x3f800000), F32(0x3f800000));
    { let v_ = F32(c.m.r32((p_arg_2).wrapping_add(32))); let a_ = (p_w_1).wrapping_add(1252); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fdiv(F32(0x3f800000), rt::fmul(rt::fmul(F32(c.m.r32((p_arg_2).wrapping_add(32))), F32(0x3f666666)), F32(0x41f00000))); let a_ = (p_w_1).wrapping_add(1248); c.m.w32(a_, (v_).0); };
    { let v_ = (c.m.r32(((p_arg_2).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32); let a_ = (p_w_1).wrapping_add(1272); c.m.w32(a_, (v_) as u32); };
    { let v_ = F32(0x3f800000); let a_ = (p_w_1).wrapping_add(1256); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fdiv(F32(0x3f800000), rt::i2f((c.m.r32((p_arg_2).wrapping_add(76)) as i32))); let a_ = (p_w_1).wrapping_add(1260); c.m.w32(a_, (v_).0); };
    { let v_ = (((c.m.r32((p_arg_2).wrapping_add(52)) as i32)) as u32); let a_ = (p_w_1).wrapping_add(1288); c.m.w32(a_, v_); };
    { let v_ = F32(c.m.r32((p_arg_2).wrapping_add(44))); let a_ = (p_w_1).wrapping_add(1264); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_arg_2).wrapping_add(48))); let a_ = (p_w_1).wrapping_add(1268); c.m.w32(a_, (v_).0); };
    { let v_ = ((c.m.r32((p_arg_2).wrapping_add(56))) as i32); let a_ = (p_w_1).wrapping_add(1276); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32((p_arg_2).wrapping_add(64)) as i32); let a_ = (p_w_1).wrapping_add(1280); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32((p_arg_2).wrapping_add(68)) as i32); let a_ = (p_w_1).wrapping_add(1284); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32((p_arg_2).wrapping_add(60)) as i32); let a_ = (p_w_1).wrapping_add(1292); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32((p_arg_2).wrapping_add(72)) as i32); let a_ = (p_w_1).wrapping_add(1296); c.m.w32(a_, (v_) as u32); };
    if (((rt::flt(F32(0x00000000), F32(c.m.r32((p_w_1).wrapping_add(1252)))) as i32)) != 0) {
        { let r_ = 1i32; let a_ = (p_w_1).wrapping_add(1300); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if ((((((c.m.r32((p_arg_2).wrapping_add(80)) as i32)) != (0i32)) as i32)) != 0) {
        { let r_ = 16i32; let a_ = (p_w_1).wrapping_add(1300); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    l_ray_3 = p_w_1;
    l_step_4 = rt::fdiv(F32(0x43b40000), rt::u2f(c.m.r32((p_arg_2).wrapping_add(56))));
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((((l_i_5) as u32)) < (c.m.r32((p_arg_2).wrapping_add(56)))) as i32)) != 0) { break 'l1; }
            'c2: {
                l_angle_6 = { let a0_ = rt::fadd(rt::fdiv(rt::fmul(rt::fmul(l_step_4, rt::i2f(l_i_5)), F32(0x40490fda)), F32(0x43340000)), rt::fdiv(rt::fmul(rt::i2f(rt::rem_i32(F::Port_Rand(c), 5i32)), F32(0x40490fda)), F32(0x43340000))); F::EftMath_WrapAngle(c, a0_) };
                { let v_ = (((c.m.r32(((p_arg_2).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = l_ray_3; c.m.w8(a_, v_); };
                { let v_ = (((c.m.r32(((p_arg_2).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (l_ray_3).wrapping_add(1); c.m.w8(a_, v_); };
                { let v_ = (((c.m.r32(((p_arg_2).wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (l_ray_3).wrapping_add(2); c.m.w8(a_, v_); };
                { let v_ = ((((c.m.r32(((p_arg_2).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)).wrapping_sub(rt::rem_i32(F::Port_Rand(c), 16i32))) as u8); let a_ = (l_ray_3).wrapping_add(3); c.m.w8(a_, v_); };
                { let v_ = l_angle_6; let a_ = (l_ray_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((p_arg_2).wrapping_add(36))), rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x41a00000))); let a_ = (l_ray_3).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul(F32(c.m.r32((p_arg_2).wrapping_add(40))), rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40a00000))); let a_ = (l_ray_3).wrapping_add(16); c.m.w32(a_, (v_).0); };
                if (((rt::flt(F32(c.m.r32((l_ray_3).wrapping_add(16))), F32(0x3ecccccc)) as i32)) != 0) {
                    { let v_ = F32(0x3ecccccc); let a_ = (l_ray_3).wrapping_add(16); c.m.w32(a_, (v_).0); };
                }
                if ((((((c.m.r32((p_w_1).wrapping_add(1284)) as i32)) == (1i32)) as i32)) != 0) {
                    { let v_ = rt::fadd(F32(c.m.r32((p_arg_2).wrapping_add(40))), rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40800000))); let a_ = (l_ray_3).wrapping_add(16); c.m.w32(a_, (v_).0); };
                }
                { let v_ = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(c.m.r32((p_w_1).wrapping_add(1268)))); let a_ = (l_ray_3).wrapping_add(20); c.m.w32(a_, (v_).0); };
            }
            { { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; }; { let o_ = l_ray_3; let n_ = (o_).wrapping_add(24); l_ray_3 = n_; }; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRay_Setup", &PROBE_N, &[], true); }
    r_
}

pub fn EftRay_DrawRays(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRay_DrawRays", &PROBE_N, &[(p_w_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(272) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 272);
    let mut l_ray_2: u32 = 0;
    let mut l_c_3: u32 = 0;
    let mut l_bp_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_j_6: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp.wrapping_add(144), 0, 64);
        rt::memset(c, fp.wrapping_add(144).wrapping_add(0), 0, 16);
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(144).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(144).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(144).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(144).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(144).wrapping_add(16), 0, 16);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(144).wrapping_add(16).wrapping_add(0), (v_).0); }
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(144).wrapping_add(16).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(144).wrapping_add(16).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(144).wrapping_add(16).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(144).wrapping_add(32), 0, 16);
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(144).wrapping_add(32).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(144).wrapping_add(32).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(144).wrapping_add(32).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(144).wrapping_add(32).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(144).wrapping_add(48), 0, 16);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(144).wrapping_add(48).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(144).wrapping_add(48).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(144).wrapping_add(48).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(144).wrapping_add(48).wrapping_add(12), (v_).0); }
        if ((((((c.m.r32((p_w_1).wrapping_add(1292)) as i32)) >= (0i32)) as i32)) != 0) {
            if ((((!((F::EftCam_IsActive(c)) != 0)) as i32)) != 0) {
                if ((((!(({ let a0_ = (c.m.r32((p_w_1).wrapping_add(1292)) as i32); F::BtlScene_IsCharInView(c, a0_) }) != 0)) as i32)) != 0) {
                    break 'f;
                }
            }
            if (({ let a0_ = (c.m.r32((p_w_1).wrapping_add(1292)) as i32); F::BtlCharApi_IsInClashA(c, a0_) }) != 0) {
                break 'f;
            }
        }
        's1: {
            let sv7: i32 = (c.m.r32((p_w_1).wrapping_add(1284)) as i32);
            match sv7 {
                0i32 => {
                    F::Mtx_StoreIdentity(c, fp.wrapping_add(80));
                    F::Mtx_StoreIdentity(c, (p_w_1).wrapping_add(1184));
                    { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(64); F::Mtx_InverseRT(c, fp.wrapping_add(80), a1_) };
                    { let v_ = F32(0x00000000); let a_ = ((fp.wrapping_add(80)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x00000000); let a_ = ((fp.wrapping_add(80)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x00000000); let a_ = ((fp.wrapping_add(80)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(80)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    F::Mtx_Mul(c, (p_w_1).wrapping_add(1184), (p_w_1).wrapping_add(1184), fp.wrapping_add(80));
                    F::Vec4_Copy(c, ((p_w_1).wrapping_add(1184)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (p_w_1).wrapping_add(1152));
                    break 's1;
                }
                1i32 => {
                    if (((((((((c.m.r32((p_w_1).wrapping_add(1288))) == (((2i32) as u32))) as i32)) != 0) || (((((c.m.r32((p_w_1).wrapping_add(1288))) == (((3i32) as u32))) as i32)) != 0)) as i32)) != 0) {
                        { let a0_ = { let a0_ = (c.m.r32((p_w_1).wrapping_add(1292)) as i32); F::BtlCharApi_GetOpponentObjId(c, a0_) }; F::BtlCharApi_GetNodePos(c, a0_, 3i32, fp.wrapping_add(208)) };
                        { let _ = { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Mtx_ProjectPoint(c, fp.wrapping_add(224), a1_, fp.wrapping_add(208)) }; };
                        if ((((((c.m.r32((fp.wrapping_add(224)).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                            break 'f;
                        }
                        { let v_ = rt::i2f((((c.m.r32(fp.wrapping_add(224)) as i32)).wrapping_sub(28672i32)).wrapping_shr((4i32) as u32)); let a_ = (p_w_1).wrapping_add(1152); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::i2f((((c.m.r32((fp.wrapping_add(224)).wrapping_add(4)) as i32)).wrapping_sub(29184i32)).wrapping_shr((4i32) as u32)); let a_ = ((p_w_1).wrapping_add(1152)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x00000000); let a_ = ((p_w_1).wrapping_add(1152)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x3f800000); let a_ = ((p_w_1).wrapping_add(1152)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    }
                    F::Mtx_StoreIdentity(c, (p_w_1).wrapping_add(1184));
                    F::Vec4_Copy(c, ((p_w_1).wrapping_add(1184)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (p_w_1).wrapping_add(1152));
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::Vu0Cur_Push(c);
        { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
        l_ray_2 = p_w_1;
        {
            l_i_5 = 0i32;
            'l2: loop {
                if !(((((l_i_5) < ((c.m.r32((p_w_1).wrapping_add(1276)) as i32))) as i32)) != 0) { break 'l2; }
                'c3: {
                    {
                        { { l_j_6 = 0i32; l_bp_4 = fp.wrapping_add(144); }; l_c_3 = fp.wrapping_add(16); };
                        'l4: loop {
                            if !(((((l_j_6) < (4i32)) as i32)) != 0) { break 'l4; }
                            'c5: {
                                { let v_ = rt::fmul(rt::fmul(F32(c.m.r32(l_bp_4)), F32(c.m.r32((l_ray_2).wrapping_add(16)))), F32(c.m.r32((p_w_1).wrapping_add(1168)))); let a_ = fp.wrapping_add(240); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fmul(rt::fmul(F32(c.m.r32((l_bp_4).wrapping_add(4))), F32(c.m.r32((l_ray_2).wrapping_add(8)))), F32(c.m.r32(((p_w_1).wrapping_add(1168)).wrapping_add(4)))); let a_ = (fp.wrapping_add(240)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(c.m.r32((l_bp_4).wrapping_add(8))); let a_ = (fp.wrapping_add(240)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(c.m.r32((l_bp_4).wrapping_add(12))); let a_ = (fp.wrapping_add(240)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                { let a2_ = rt::fneg(rt::fadd(F32(c.m.r32((p_w_1).wrapping_add(1264))), F32(c.m.r32((l_ray_2).wrapping_add(20))))); F::Vec4_Set(c, fp, F32(0x00000000), a2_, F32(0x00000000), F32(0x00000000)) };
                                F::Vec4_Add(c, fp.wrapping_add(240), fp.wrapping_add(240), fp);
                                F::Mtx_StoreIdentity(c, fp.wrapping_add(80));
                                { let a2_ = F32(c.m.r32((l_ray_2).wrapping_add(4))); F::Mtx_RotateZ(c, fp.wrapping_add(80), fp.wrapping_add(80), a2_) };
                                F::Mtx_MulVec4(c, fp.wrapping_add(256), fp.wrapping_add(80), fp.wrapping_add(240));
                                F::Mtx_MulVec4(c, l_c_3, (p_w_1).wrapping_add(1184), fp.wrapping_add(256));
                                { let o_ = l_j_6; let n_ = (o_).wrapping_add(1); l_j_6 = n_; };
                                { let o_ = l_bp_4; let n_ = (o_).wrapping_add(16); l_bp_4 = n_; };
                                { let o_ = l_c_3; let n_ = (o_).wrapping_add(16); l_c_3 = n_; };
                                if (((((l_j_6) >= (4i32)) as i32)) != 0) {
                                    break 'l4;
                                }
                            }
                        }
                    }
                    if ((((((c.m.r32((p_w_1).wrapping_add(1284)) as i32)) == (0i32)) as i32)) != 0) {
                        { let a1_ = c.m.r8(l_ray_2); let a2_ = c.m.r8((l_ray_2).wrapping_add(1)); let a3_ = c.m.r8((l_ray_2).wrapping_add(2)); let a4_ = ((rt::f2u(rt::fmul(rt::i2f(((c.m.r8((l_ray_2).wrapping_add(3))) as i32)), F32(c.m.r32((p_w_1).wrapping_add(1256)))))) as u8); let a5_ = c.m.r8(l_ray_2); let a6_ = c.m.r8((l_ray_2).wrapping_add(1)); let a7_ = c.m.r8((l_ray_2).wrapping_add(2)); let a8_ = ((rt::f2u(rt::fmul(rt::i2f(((c.m.r8((l_ray_2).wrapping_add(3))) as i32)), F32(c.m.r32((p_w_1).wrapping_add(1256)))))) as u8); let a9_ = (c.m.r32((p_w_1).wrapping_add(1280)) as i32); let a10_ = c.m.r32(sym::gEftRay); F::EftRay_DrawQuad3D(c, fp.wrapping_add(16), a1_, a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_, a10_, F32(0x40800000)) };
                    } else {
                        { let a2_ = c.m.r8(l_ray_2); let a3_ = c.m.r8((l_ray_2).wrapping_add(1)); let a4_ = c.m.r8((l_ray_2).wrapping_add(2)); let a5_ = ((rt::f2u(rt::fmul(rt::i2f(((c.m.r8((l_ray_2).wrapping_add(3))) as i32)), F32(c.m.r32((p_w_1).wrapping_add(1256)))))) as u8); let a6_ = c.m.r8(l_ray_2); let a7_ = c.m.r8((l_ray_2).wrapping_add(1)); let a8_ = c.m.r8((l_ray_2).wrapping_add(2)); let a9_ = ((rt::f2u(rt::fmul(rt::i2f(((c.m.r8((l_ray_2).wrapping_add(3))) as i32)), F32(c.m.r32((p_w_1).wrapping_add(1256)))))) as u8); let a10_ = (c.m.r32((p_w_1).wrapping_add(1280)) as i32); let a11_ = c.m.r32(sym::gEftRay); F::EftRay_DrawQuad2D(c, fp.wrapping_add(16), 0i32, a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_, a10_, a11_, F32(0x40800000)) };
                    }
                }
                { { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; }; { let o_ = l_ray_2; let n_ = (o_).wrapping_add(24); l_ray_2 = n_; }; };
            }
        }
        F::Vu0Cur_Pop(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftRay_DrawRays", &PROBE_N, &[], true); }
    r_
}

pub fn EftRay_DrawQuad3D(c: &mut Ctx, mut p_corner_1: u32, mut p_r0_2: u8, mut p_g0_3: u8, mut p_b0_4: u8, mut p_a0_5: u8, mut p_r1_6: u8, mut p_g1_7: u8, mut p_b1_8: u8, mut p_a1_9: u8, mut p_blend_10: i32, mut p_tex_11: u32, mut p_segs_12: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRay_DrawQuad3D", &PROBE_N, &[(p_corner_1) as u32, (p_r0_2) as u32, (p_g0_3) as u32, (p_b0_4) as u32, (p_a0_5) as u32, (p_r1_6) as u32, (p_g1_7) as u32, (p_b1_8) as u32, (p_a1_9) as u32, (p_blend_10) as u32, (p_tex_11) as u32, (p_segs_12).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(560) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 560);
    let r_ = 'f: {
        F::Vec4_Set(c, (fp.wrapping_add(432)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), F32(0x00000000), F32(0x3f800000), F32(0x3f800000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(432)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), F32(0x3f800000), F32(0x3f800000), F32(0x3f800000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(432)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), F32(0x00000000), F32(0x00000000), F32(0x3f800000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(432)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), F32(0x3f800000), F32(0x00000000), F32(0x3f800000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(496)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), rt::i2f((p_r0_2) as i32), rt::i2f((p_g0_3) as i32), rt::i2f((p_b0_4) as i32), rt::i2f((p_a0_5) as i32));
        F::Vec4_Set(c, (fp.wrapping_add(496)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), rt::i2f((p_r0_2) as i32), rt::i2f((p_g0_3) as i32), rt::i2f((p_b0_4) as i32), rt::i2f((p_a0_5) as i32));
        F::Vec4_Set(c, (fp.wrapping_add(496)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), rt::i2f((p_r1_6) as i32), rt::i2f((p_g1_7) as i32), rt::i2f((p_b1_8) as i32), rt::i2f((p_a1_9) as i32));
        F::Vec4_Set(c, (fp.wrapping_add(496)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), rt::i2f((p_r1_6) as i32), rt::i2f((p_g1_7) as i32), rt::i2f((p_b1_8) as i32), rt::i2f((p_a1_9) as i32));
        F::ClipVtx_SetArray(c, fp, (p_corner_1).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(432)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(496)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), 3i32);
        { let a2_ = c.m.r64(p_tex_11); F::EftRay_DrawClipped(c, fp, p_blend_10, a2_) };
        F::ClipVtx_SetArray(c, fp, (p_corner_1).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(432)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(496)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), 3i32);
        { let a2_ = c.m.r64(p_tex_11); F::EftRay_DrawClipped(c, fp, p_blend_10, a2_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftRay_DrawQuad3D", &PROBE_N, &[], true); }
    r_
}

pub fn eft_s__EftTOt_Add(c: &mut Ctx, mut p_p_1: u32, mut p_z_2: i32, mut p_layer_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_s__EftTOt_Add", &PROBE_N, &[(p_p_1) as u32, (p_z_2) as u32, (p_layer_3) as u32], false); }
    let mut l_e_4: u32 = 0;
    let r_ = 'f: {
    if (((((p_layer_3) >= (2i32)) as i32)) != 0) {
        { let r_ = 2i32; let n_ = (p_layer_3).wrapping_sub(r_); p_layer_3 = n_; };
    }
    if (((((p_z_2) < (0i32)) as i32)) != 0) {
        l_e_4 = ((c.m.r32(sym::gOtZ)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((p_layer_3) as u32)).wrapping_mul(8));
    } else {
        if (((((p_z_2) >= (4096i32)) as i32)) != 0) {
            l_e_4 = ((c.m.r32(sym::gOtZ)).wrapping_add((((4095i32) as u32)).wrapping_mul(16))).wrapping_add((((p_layer_3) as u32)).wrapping_mul(8));
        } else {
            l_e_4 = ((c.m.r32(sym::gOtZ)).wrapping_add((((p_z_2) as u32)).wrapping_mul(16))).wrapping_add((((p_layer_3) as u32)).wrapping_mul(8));
        }
    }
    { let v_ = p_p_1; let a_ = (c.m.r32((l_e_4).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = p_p_1; let a_ = (l_e_4).wrapping_add(4); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "eft_s__EftTOt_Add", &PROBE_N, &[], true); }
    r_
}

pub fn EftRay_DrawQuad2D(c: &mut Ctx, mut p_corner_1: u32, mut p_z_2: i32, mut p_r0_3: u8, mut p_g0_4: u8, mut p_b0_5: u8, mut p_a0_6: u8, mut p_r1_7: u8, mut p_g1_8: u8, mut p_b1_9: u8, mut p_a1_10: u8, mut p_blend_11: i32, mut p_tex_12: u32, mut p_segs_13: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRay_DrawQuad2D", &PROBE_N, &[(p_corner_1) as u32, (p_z_2) as u32, (p_r0_3) as u32, (p_g0_4) as u32, (p_b0_5) as u32, (p_a0_6) as u32, (p_r1_7) as u32, (p_g1_8) as u32, (p_b1_9) as u32, (p_a1_10) as u32, (p_blend_11) as u32, (p_tex_12) as u32, (p_segs_13).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(288) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 288);
    let mut l_aStep_14: i32 = 0;
    let mut l_t_15: F32 = F32(0);
    let mut l_inv_16: F32 = F32(0);
    let mut l_vStep0_17: F32 = F32(0);
    let mut l_vStep1_18: F32 = F32(0);
    let mut l_aNext_19: i32 = 0;
    let mut l_aCur_20: i32 = 0;
    let mut l_q_21: u32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp.wrapping_add(224), 0, 64);
        rt::memset(c, fp.wrapping_add(224).wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(224).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(224).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(224).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(224).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(224).wrapping_add(16), 0, 16);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(224).wrapping_add(16).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(224).wrapping_add(16).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(224).wrapping_add(16).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(224).wrapping_add(16).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(224).wrapping_add(32), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(224).wrapping_add(32).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(224).wrapping_add(32).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(224).wrapping_add(32).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(224).wrapping_add(32).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(224).wrapping_add(48), 0, 16);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(224).wrapping_add(48).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(224).wrapping_add(48).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(224).wrapping_add(48).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(224).wrapping_add(48).wrapping_add(12), (v_).0); }
        l_t_15 = rt::fdiv(F32(0x3f800000), p_segs_13);
        l_inv_16 = l_t_15;
        F::Vec4_Sub(c, (fp.wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (p_corner_1).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (p_corner_1).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
        F::Vec4_Sub(c, (fp.wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (p_corner_1).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (p_corner_1).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
        F::Vec3_Div(c, (fp.wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), p_segs_13);
        F::Vec3_Div(c, (fp.wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), p_segs_13);
        F::Vec4_Copy(c, (fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (p_corner_1).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
        F::Vec4_Copy(c, (fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (p_corner_1).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
        { let v_ = F32(c.m.r32(((fp.wrapping_add(224)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(224)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        l_vStep0_17 = rt::fdiv(rt::fsub(F32(c.m.r32(((fp.wrapping_add(224)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))), F32(c.m.r32(((fp.wrapping_add(224)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)))), p_segs_13);
        l_vStep1_18 = rt::fdiv(rt::fsub(F32(c.m.r32(((fp.wrapping_add(224)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4))), F32(c.m.r32(((fp.wrapping_add(224)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4)))), p_segs_13);
        l_aStep_14 = rt::f2i(rt::fdiv(rt::i2f((((p_a1_10) as i32)).wrapping_sub(((p_a0_6) as i32))), p_segs_13));
        { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
        {
            { l_aNext_19 = ((p_a0_6) as i32); l_aCur_20 = l_aNext_19; };
            'l1: loop {
                if !((((!(((rt::flt(F32(0x3f800000), l_t_15) as i32)) != 0)) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::Vec3_Add(c, (fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
                    F::Vec3_Add(c, (fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
                    { let r_ = l_aStep_14; let n_ = (l_aNext_19).wrapping_add(r_); l_aNext_19 = n_; };
                    { let v_ = rt::fadd(F32(c.m.r32(((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))), l_vStep0_17); let a_ = ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32(((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))), l_vStep1_18); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let _ = D::call(c, fid::Vec4_ToInt, &[(((fp.wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as u64), (((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as u64)]); };
                    { let _ = D::call(c, fid::Vec4_ToInt, &[(((fp.wrapping_add(160)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as u64), (((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as u64)]); };
                    { let _ = D::call(c, fid::Vec4_ToInt, &[(((fp.wrapping_add(160)).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as u64), (((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as u64)]); };
                    { let _ = D::call(c, fid::Vec4_ToInt, &[(((fp.wrapping_add(160)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as u64), (((fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as u64)]); };
                    l_q_21 = c.m.r32(sym::gOtCur);
                    { let v_ = (l_q_21).wrapping_add((((1i32) as u32)).wrapping_mul(144)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
                    { let v_ = ((92i32) as u64); let a_ = (l_q_21).wrapping_add(32); c.m.w64(a_, v_); };
                    { let v_ = ((536870920i32) as u32); let a_ = l_q_21; c.m.w32(a_, v_); };
                    { let v_ = ((268435456i32) as u32); let a_ = (l_q_21).wrapping_add(8); c.m.w32(a_, v_); };
                    { let v_ = ((1342177288i32) as u32); let a_ = (l_q_21).wrapping_add(12); c.m.w32(a_, v_); };
                    { let v_ = 16429131440647602177u64; let a_ = (l_q_21).wrapping_add(16); c.m.w64(a_, v_); };
                    { let v_ = ((18599481537995104i64) as u64); let a_ = (l_q_21).wrapping_add(24); c.m.w64(a_, v_); };
                    { let v_ = ((0i32) as u32); let a_ = (l_q_21).wrapping_add(4); c.m.w32(a_, v_); };
                    { let v_ = p_r0_3; let a_ = (((l_q_21).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = p_g0_4; let a_ = (((l_q_21).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = p_b0_5; let a_ = (((l_q_21).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = ((l_aCur_20) as u8); let a_ = (((l_q_21).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = F32(0x3f800000); let a_ = (((l_q_21).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = p_r0_3; let a_ = (((l_q_21).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = p_g0_4; let a_ = (((l_q_21).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = p_b0_5; let a_ = (((l_q_21).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = ((l_aCur_20) as u8); let a_ = (((l_q_21).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = F32(0x3f800000); let a_ = (((l_q_21).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = p_r1_7; let a_ = (((l_q_21).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = p_g1_8; let a_ = (((l_q_21).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = p_b1_9; let a_ = (((l_q_21).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = ((l_aNext_19) as u8); let a_ = (((l_q_21).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = F32(0x3f800000); let a_ = (((l_q_21).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = p_r1_7; let a_ = (((l_q_21).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = p_g1_8; let a_ = (((l_q_21).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = p_b1_9; let a_ = (((l_q_21).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = ((l_aNext_19) as u8); let a_ = (((l_q_21).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = F32(0x3f800000); let a_ = (((l_q_21).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((fp.wrapping_add(224)).wrapping_add((((0i32) as u32)).wrapping_mul(16)))); let a_ = (((l_q_21).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32(((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = (((l_q_21).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((fp.wrapping_add(224)).wrapping_add((((1i32) as u32)).wrapping_mul(16)))); let a_ = (((l_q_21).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32(((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = (((l_q_21).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((fp.wrapping_add(224)).wrapping_add((((2i32) as u32)).wrapping_mul(16)))); let a_ = (((l_q_21).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = (((l_q_21).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((fp.wrapping_add(224)).wrapping_add((((3i32) as u32)).wrapping_mul(16)))); let a_ = (((l_q_21).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = (((l_q_21).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = (((((((((c.m.r32((fp.wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as i32)) as u16)) as i32)).wrapping_mul(16i32)).wrapping_add(28672i32)) as u64); let a_ = (((l_q_21).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((((((((c.m.r32(((fp.wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u16)) as i32)).wrapping_mul(16i32)).wrapping_add(29184i32)) as u64); let a_ = ((((l_q_21).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = ((16777215i32) as u64); let a_ = ((((l_q_21).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                    { let v_ = ((255i32) as u64); let a_ = ((((l_q_21).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                    { let v_ = (((((((((c.m.r32((fp.wrapping_add(160)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as i32)) as u16)) as i32)).wrapping_mul(16i32)).wrapping_add(28672i32)) as u64); let a_ = (((l_q_21).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((((((((c.m.r32(((fp.wrapping_add(160)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u16)) as i32)).wrapping_mul(16i32)).wrapping_add(29184i32)) as u64); let a_ = ((((l_q_21).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = ((16777215i32) as u64); let a_ = ((((l_q_21).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                    { let v_ = ((255i32) as u64); let a_ = ((((l_q_21).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                    { let v_ = (((((((((c.m.r32((fp.wrapping_add(160)).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as i32)) as u16)) as i32)).wrapping_mul(16i32)).wrapping_add(28672i32)) as u64); let a_ = (((l_q_21).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((((((((c.m.r32(((fp.wrapping_add(160)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u16)) as i32)).wrapping_mul(16i32)).wrapping_add(29184i32)) as u64); let a_ = ((((l_q_21).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = ((16777215i32) as u64); let a_ = ((((l_q_21).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                    { let v_ = ((255i32) as u64); let a_ = ((((l_q_21).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                    { let v_ = (((((((((c.m.r32((fp.wrapping_add(160)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as i32)) as u16)) as i32)).wrapping_mul(16i32)).wrapping_add(28672i32)) as u64); let a_ = (((l_q_21).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((((((((c.m.r32(((fp.wrapping_add(160)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u16)) as i32)).wrapping_mul(16i32)).wrapping_add(29184i32)) as u64); let a_ = ((((l_q_21).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = ((16777215i32) as u64); let a_ = ((((l_q_21).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                    { let v_ = ((255i32) as u64); let a_ = ((((l_q_21).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                    { let v_ = c.m.r64(p_tex_12); let a_ = (l_q_21).wrapping_add(40); c.m.w64(a_, v_); };
                    F::eft_s__EftTOt_Add(c, l_q_21, p_z_2, p_blend_11);
                    F::Vec4_Copy(c, (fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
                    l_aCur_20 = l_aNext_19;
                    F::Vec4_Copy(c, (fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
                    { let v_ = F32(c.m.r32(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                }
                { let r_ = l_inv_16; let n_ = rt::fadd(l_t_15, r_); l_t_15 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftRay_DrawQuad2D", &PROBE_N, &[], true); }
    r_
}

pub fn EftRay_DrawClipped(c: &mut Ctx, mut p_poly_1: u32, mut p_blend_2: i32, mut p_tex0_3: u64) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRay_DrawClipped", &PROBE_N, &[(p_poly_1) as u32, (p_blend_2) as u32, (p_tex0_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(1056) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 1056);
    let mut l_plane_4: u32 = 0;
    let mut l_n_5: i32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_z_7: i32 = 0;
    let r_ = 'f: {
        { let _ = { rt::memcpy(c, fp, p_poly_1, ((((3i32) as u32)).wrapping_mul(48u32)) as u32); fp }; };
        p_poly_1 = fp;
        l_n_5 = 3i32;
        l_plane_4 = F::EftGfx_GetClipPlanes(c);
        {
            l_i_6 = 0i32;
            'l1: loop {
                if !(((((l_i_6) < (5i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_n_5 = F::ClipPoly_ClipPlane(c, p_poly_1, l_plane_4, l_n_5);
                    { let o_ = l_plane_4; let n_ = (o_).wrapping_add(16); l_plane_4 = n_; };
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
        if (((((l_n_5) != (0i32)) as i32)) != 0) {
            F::ClipPoly_ProjectCur(c, fp.wrapping_add(768), fp.wrapping_add(912), p_poly_1, l_n_5);
            {
                l_i_6 = 2i32;
                'l3: loop {
                    if !(((((l_i_6) < (l_n_5)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        l_z_7 = rt::div_i32((((c.m.r32(((fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)).wrapping_add((c.m.r32(((fp.wrapping_add(768)).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add((c.m.r32(((fp.wrapping_add(768)).wrapping_add((((l_i_6) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)), 3i32);
                        { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add((((l_i_6) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        F::EftPrim_DrawTriangle(c, (fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(768)).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(768)).wrapping_add((((l_i_6) as u32)).wrapping_mul(16)), ((p_poly_1).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(32), ((p_poly_1).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(48))).wrapping_add(32), ((p_poly_1).wrapping_add((((l_i_6) as u32)).wrapping_mul(48))).wrapping_add(32), (fp.wrapping_add(912)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(912)).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(912)).wrapping_add((((l_i_6) as u32)).wrapping_mul(16)), 0i32, 0i32, p_blend_2, (l_z_7).wrapping_shr((8i32) as u32), p_tex0_3);
                    }
                    { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftRay_DrawClipped", &PROBE_N, &[], true); }
    r_
}
