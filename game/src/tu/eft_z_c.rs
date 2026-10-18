#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn eft_z_c__EftGndDust_FreeParts(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_z_c__EftGndDust_FreeParts", &PROBE_N, &[(p_w_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
    l_p_2 = F::List_GetHead(c, (p_w_1).wrapping_add(204));
    'l1: while (((((l_p_2) != (((0i32) as u32))) as i32)) != 0) {
        { let r_ = (!(2i32)); let a_ = (l_p_2).wrapping_add(188); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        l_p_2 = { let a1_ = c.m.r32((p_w_1).wrapping_add(200)); F::EftGndDust_MoveNode(c, (p_w_1).wrapping_add(204), a1_, l_p_2) };
        { let a_ = (c.m.r32(sym::gEftGndDust)).wrapping_add(25268); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "eft_z_c__EftGndDust_FreeParts", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDustImpact_Init(c: &mut Ctx, mut p_task_1: u32, mut p_param_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustImpact_Init", &PROBE_N, &[(p_task_1) as u32, (p_param_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_w_3: u32 = 0;
    let mut l_arg_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_life_6: i32 = 0;
    let mut l_fade_7: i32 = 0;
    let mut l_start_8: F32 = F32(0);
    let mut l_step_9: F32 = F32(0);
    let mut l_ang_10: F32 = F32(0);
    let mut l_s_11: F32 = F32(0);
    let mut l_spin_12: F32 = F32(0);
    let mut l_grow_13: F32 = F32(0);
    let r_ = 'f: {
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        l_arg_4 = p_param_2;
        { let _ = F::memset(c, l_w_3, 0i32, 224u32); };
        F::List_Init(c, (l_w_3).wrapping_add(204));
        { let v_ = { let a0_ = (((c.m.r8((l_arg_4).wrapping_add(57)) as i8)) as i32); F::EftGndDust_GetFreeList(c, a0_) }; let a_ = (l_w_3).wrapping_add(200); c.m.w32(a_, v_); };
        F::EftGndDust_GetLightColors(c, (l_arg_4).wrapping_add(48), (l_arg_4).wrapping_add(52));
        { let v_ = { let a0_ = (((c.m.r8((l_arg_4).wrapping_add(56)) as i8)) as i32); F::BtlScene_GetCharScale(c, a0_) }; let a_ = (l_arg_4).wrapping_add(96); c.m.w32(a_, (v_).0); };
        if (((rt::flt(F32(c.m.r32((l_arg_4).wrapping_add(96))), F32(0x3f4ccccc)) as i32)) != 0) {
            { let v_ = F32(0x3f4ccccc); let a_ = (l_arg_4).wrapping_add(96); c.m.w32(a_, (v_).0); };
        }
        F::Vec4_Set(c, (l_arg_4).wrapping_add(32), F32(0x00000000), F32(0xbf800000), F32(0x00000000), F32(0x3f800000));
        { let v_ = ((1i32) as i8); let a_ = (l_arg_4).wrapping_add(57); c.m.w8(a_, (v_) as u8); };
        { let v_ = ((2i32) as i8); let a_ = (l_arg_4).wrapping_add(59); c.m.w8(a_, (v_) as u8); };
        { let v_ = F32(0x3dcccccc); let a_ = (l_arg_4).wrapping_add(72); c.m.w32(a_, (v_).0); };
        { let v_ = ((10i32) as i16); let a_ = (l_arg_4).wrapping_add(60); c.m.w16(a_, (v_) as u16); };
        { let v_ = ((15i32) as i16); let a_ = (l_arg_4).wrapping_add(62); c.m.w16(a_, (v_) as u16); };
        { let v_ = ((5i32) as i16); let a_ = (l_arg_4).wrapping_add(64); c.m.w16(a_, (v_) as u16); };
        { let v_ = ((0i32) as i8); let a_ = (l_arg_4).wrapping_add(58); c.m.w8(a_, (v_) as u8); };
        { let v_ = ((0i32) as i16); let a_ = (l_arg_4).wrapping_add(68); c.m.w16(a_, (v_) as u16); };
        { let v_ = ((0i32) as i16); let a_ = (l_arg_4).wrapping_add(70); c.m.w16(a_, (v_) as u16); };
        { let v_ = ((0i32) as i16); let a_ = (l_arg_4).wrapping_add(66); c.m.w16(a_, (v_) as u16); };
        { let v_ = F32(0x00000000); let a_ = (l_arg_4).wrapping_add(80); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (l_arg_4).wrapping_add(84); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (l_arg_4).wrapping_add(88); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (l_arg_4).wrapping_add(92); c.m.w32(a_, (v_).0); };
        l_step_9 = F32(0x40060a91);
        l_start_8 = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40c90fda));
        {
            l_i_5 = 0i32;
            'l1: loop {
                if !(((((l_i_5) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_ang_10 = F::EftMath_WrapAngle(c, rt::fadd(l_start_8, rt::fmul(l_step_9, rt::i2f(l_i_5))));
                    l_s_11 = F::Mathf_SinFast(c, l_ang_10);
                    { let a3_ = F::Mathf_CosFast(c, l_ang_10); F::Vec4_Set(c, fp.wrapping_add(16), l_s_11, F32(0x00000000), a3_, F32(0x3f800000)) };
                    F::Vec3_Normalize(c, fp.wrapping_add(16), fp.wrapping_add(16));
                    F::Vec4_Scale(c, fp.wrapping_add(32), fp.wrapping_add(16), F32(0x40000000));
                    F::Vec4_Add(c, fp, l_arg_4, fp.wrapping_add(32));
                    { let r_ = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40400000)); let a_ = (fp).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    l_life_6 = ((((c.m.r16((l_arg_4).wrapping_add(62)) as i16)) as i32)).wrapping_add(rt::rem_i32(F::Port_Rand(c), 5i32));
                    l_fade_7 = ((((c.m.r16((l_arg_4).wrapping_add(64)) as i16)) as i32)).wrapping_add(rt::rem_i32(F::Port_Rand(c), 5i32));
                    l_spin_12 = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40c90fda));
                    l_grow_13 = rt::fmul(rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3ba3d70a)), F32(0x3ba3d70a)), F32(c.m.r32((l_arg_4).wrapping_add(96))));
                    { let _ = { let a9_ = rt::fmul(F32(c.m.r32((l_arg_4).wrapping_add(96))), F32(0x3e999999)); let a15_ = (((rt::rem_i32(F::Port_Rand(c), 10i32)).wrapping_add(25i32)) as i16); F::EftGndDust_SpawnPieceEx(c, l_w_3, fp, fp.wrapping_add(16), (l_arg_4).wrapping_add(48), (l_arg_4).wrapping_add(52), F32(0x3f800000), F32(0x3f800000), F32(0x3d4ccccc), F32(0x3d4ccccc), a9_, F32(0x3f4ccccc), l_spin_12, F32(0x3e999999), ((rt::f2i(rt::i2f(l_life_6))) as i16), ((rt::f2i(rt::i2f(l_fade_7))) as i16), a15_, F32(0x3f800000), l_grow_13, 0i32) }; };
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
        { let s_ = l_arg_4; let d_ = l_w_3; rt::memcpy(c, d_, s_, 112); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustImpact_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDustImpact_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustImpact_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    F::eft_z_c__EftGndDust_FreeParts(c, l_w_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustImpact_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDustImpact_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustImpact_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustImpact_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDustImpact_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustImpact_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!((F::BtlScene_IsTimeStopped(c)) != 0)) as i32)) != 0) {
        { let a0_ = (((c.m.r8((l_w_2).wrapping_add(56)) as i8)) as i32); F::BtlCharApi_GetPos(c, a0_, (l_w_2).wrapping_add(144)) };
        { let a0_ = (((c.m.r8((l_w_2).wrapping_add(56)) as i8)) as i32); F::BtlCharApi_GetDir(c, a0_, (l_w_2).wrapping_add(160)) };
        F::Vec3_Scale(c, (l_w_2).wrapping_add(160), (l_w_2).wrapping_add(160), F32(0xbf800000));
        F::Vec3_Normalize(c, (l_w_2).wrapping_add(160), (l_w_2).wrapping_add(160));
        F::EftGndDust_UpdateParts(c, l_w_2);
    }
    if (((((F::List_GetHead(c, (l_w_2).wrapping_add(204))) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a0_ = { let a_ = (c.m.r32(sym::gEftGndDust)).wrapping_add(25264); c.m.r32(a_) }; F::EftGndDust_GetTex(c, a0_, 0i32) }; let a_ = (l_w_2).wrapping_add(184); c.m.w64(a_, v_); };
    } else {
        F::BtlTask_SetDead(c, p_task_1);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustImpact_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDustImpact_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustImpact_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_2: u32 = 0;
    let mut l_p_3: u32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_p_3 = F::List_GetHead(c, (l_w_2).wrapping_add(204));
        F::Vu0Cur_Push(c);
        { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
        'l1: while (((((l_p_3) != (((0i32) as u32))) as i32)) != 0) {
            { let a1_ = rt::fmul(F32(c.m.r32((l_p_3).wrapping_add(64))), F32(c.m.r32((l_w_2).wrapping_add(76)))); let a2_ = rt::fmul(F32(c.m.r32(((l_p_3).wrapping_add(64)).wrapping_add(4))), F32(c.m.r32((l_w_2).wrapping_add(76)))); let a3_ = rt::fmul(F32(c.m.r32(((l_p_3).wrapping_add(64)).wrapping_add(8))), F32(c.m.r32((l_w_2).wrapping_add(76)))); let a4_ = rt::fmul(F32(c.m.r32(((l_p_3).wrapping_add(64)).wrapping_add(12))), F32(c.m.r32((l_p_3).wrapping_add(180)))); F::Vec4_Set(c, fp, a1_, a2_, a3_, a4_) };
            { let a3_ = (((c.m.r8((l_w_2).wrapping_add(58)) as i8)) as i32); let a4_ = c.m.r64((l_w_2).wrapping_add(184)); let a6_ = rt::fmul(rt::fmul(rt::i2f(((c.m.r16((l_p_3).wrapping_add(160)) as i16)) as i32), F32(c.m.r32((l_w_2).wrapping_add(96)))), F32(c.m.r32((l_p_3).wrapping_add(96)))); let a7_ = rt::fmul(rt::fmul(rt::i2f(((c.m.r16((l_p_3).wrapping_add(160)) as i16)) as i32), F32(c.m.r32((l_w_2).wrapping_add(96)))), F32(c.m.r32(((l_p_3).wrapping_add(96)).wrapping_add(4)))); let a8_ = F32(c.m.r32((l_p_3).wrapping_add(164))); F::EftGndDust_DrawPiece(c, (l_p_3).wrapping_add(16), fp, (l_p_3).wrapping_add(48), a3_, a4_, 0i32, a6_, a7_, a8_) };
            l_p_3 = F::List_GetNext(c, l_p_3);
        }
        F::Vu0Cur_Pop(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustImpact_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDust_UpdateParts(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_UpdateParts", &PROBE_N, &[(p_w_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
    l_p_2 = F::List_GetHead(c, (p_w_1).wrapping_add(204));
    'l1: while (((((l_p_2) != (((0i32) as u32))) as i32)) != 0) {
        { let a_ = (l_p_2).wrapping_add(152); let o_ = (c.m.r16(a_) as i16); let n_ = (o_).wrapping_sub(1); c.m.w16(a_, (n_) as u16); };
        if ((((((((c.m.r16((l_p_2).wrapping_add(152)) as i16)) as i32)) <= (0i32)) as i32)) != 0) {
            { let v_ = ((0i32) as i16); let a_ = (l_p_2).wrapping_add(152); c.m.w16(a_, (v_) as u16); };
            { let r_ = 4i32; let a_ = (l_p_2).wrapping_add(188); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if (((((c.m.r32((l_p_2).wrapping_add(188)) as i32)) & (16i32))) != 0) {
            F::Vec4_Copy(c, (l_p_2).wrapping_add(32), (p_w_1).wrapping_add(144));
        }
        if (((((c.m.r32((l_p_2).wrapping_add(188)) as i32)) & (32i32))) != 0) {
            F::Vec4_Copy(c, (l_p_2).wrapping_add(48), (p_w_1).wrapping_add(160));
            { let a2_ = F32(c.m.r32((l_p_2).wrapping_add(172))); F::Vec3_Scale(c, (l_p_2).wrapping_add(80), (l_p_2).wrapping_add(48), a2_) };
        }
        { let a3_ = rt::fdiv(rt::i2f(((c.m.r16((l_p_2).wrapping_add(152)) as i16)) as i32), rt::i2f(((c.m.r16((l_p_2).wrapping_add(154)) as i16)) as i32)); F::EftGndDust_LerpColor(c, (l_p_2).wrapping_add(64), (l_p_2).wrapping_add(144), (l_p_2).wrapping_add(148), a3_) };
        F::Vec3_Add(c, (l_p_2).wrapping_add(96), (l_p_2).wrapping_add(96), (l_p_2).wrapping_add(112));
        { let a2_ = F32(c.m.r32((l_p_2).wrapping_add(184))); F::Vec3_Scale(c, (l_p_2).wrapping_add(112), (l_p_2).wrapping_add(112), a2_) };
        { let r_ = rt::fdiv(rt::fmul(F32(c.m.r32((l_p_2).wrapping_add(168))), F32(0x40490fda)), F32(0x43340000)); let a_ = (l_p_2).wrapping_add(164); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((l_p_2).wrapping_add(164))); F::EftMath_WrapAngle(c, a0_) }; let a_ = (l_p_2).wrapping_add(164); c.m.w32(a_, (v_).0); };
        { let a2_ = F32(c.m.r32((l_p_2).wrapping_add(176))); F::Vec3_Scale(c, (l_p_2).wrapping_add(80), (l_p_2).wrapping_add(80), a2_) };
        F::Vec3_Add(c, (l_p_2).wrapping_add(80), (l_p_2).wrapping_add(80), (l_p_2).wrapping_add(128));
        F::Vec3_Add(c, (l_p_2).wrapping_add(16), (l_p_2).wrapping_add(16), (l_p_2).wrapping_add(80));
        if (((((c.m.r32((p_w_1).wrapping_add(180)) as i32)) & (8i32))) != 0) {
            if (((((c.m.r32((p_w_1).wrapping_add(176)) as i32)) & (16i32))) != 0) {
                { let r_ = (!(16i32)); let a_ = (l_p_2).wrapping_add(188); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                F::Vec3_Add(c, (l_p_2).wrapping_add(16), (l_p_2).wrapping_add(16), (l_p_2).wrapping_add(32));
                F::Vec4_Set(c, (l_p_2).wrapping_add(32), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x3f800000));
            }
        }
        if (((((c.m.r32((l_p_2).wrapping_add(188)) as i32)) & (4i32))) != 0) {
            'l2: loop {
                'c3: {
                    if ((((((((c.m.r16((l_p_2).wrapping_add(158)) as i16)) as i32)) > (0i32)) as i32)) != 0) {
                        { let a_ = (l_p_2).wrapping_add(158); let o_ = (c.m.r16(a_) as i16); let n_ = (o_).wrapping_sub(1); c.m.w16(a_, (n_) as u16); };
                        { let v_ = rt::fdiv(rt::i2f(((c.m.r16((l_p_2).wrapping_add(158)) as i16)) as i32), rt::i2f(((c.m.r16((l_p_2).wrapping_add(156)) as i16)) as i32)); let a_ = (l_p_2).wrapping_add(180); c.m.w32(a_, (v_).0); };
                        if ((((((((c.m.r16((l_p_2).wrapping_add(158)) as i16)) as i32)) > (0i32)) as i32)) != 0) {
                            break 'l2;
                        }
                        { let v_ = F32(0x00000000); let a_ = (l_p_2).wrapping_add(180); c.m.w32(a_, (v_).0); };
                    }
                    { let r_ = 8i32; let a_ = (l_p_2).wrapping_add(188); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
                if !((0i32) != 0) { break 'l2; }
            }
        }
        if (((((c.m.r32((l_p_2).wrapping_add(188)) as i32)) & (8i32))) != 0) {
            { let r_ = (!(2i32)); let a_ = (l_p_2).wrapping_add(188); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            l_p_2 = { let a1_ = c.m.r32((p_w_1).wrapping_add(200)); F::EftGndDust_MoveNode(c, (p_w_1).wrapping_add(204), a1_, l_p_2) };
            { let a_ = (c.m.r32(sym::gEftGndDust)).wrapping_add(25268); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
        } else {
            l_p_2 = F::List_GetNext(c, l_p_2);
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_UpdateParts", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDust_LerpColor(c: &mut Ctx, mut p_out_1: u32, mut p_a_2: u32, mut p_b_3: u32, mut p_t_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_LerpColor", &PROBE_N, &[(p_out_1) as u32, (p_a_2) as u32, (p_b_3) as u32, (p_t_4).0], false); }
    let mut l_u_5: F32 = F32(0);
    let r_ = 'f: {
    l_u_5 = rt::fsub(F32(0x3f800000), p_t_4);
    { let v_ = rt::fadd(rt::fmul(rt::i2f(((c.m.r8((p_a_2).wrapping_add(((0i32) as u32)))) as i32)), p_t_4), rt::fmul(rt::i2f(((c.m.r8((p_b_3).wrapping_add(((0i32) as u32)))) as i32)), l_u_5)); let a_ = p_out_1; c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(rt::fmul(rt::i2f(((c.m.r8((p_a_2).wrapping_add(((1i32) as u32)))) as i32)), p_t_4), rt::fmul(rt::i2f(((c.m.r8((p_b_3).wrapping_add(((1i32) as u32)))) as i32)), l_u_5)); let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(rt::fmul(rt::i2f(((c.m.r8((p_a_2).wrapping_add(((2i32) as u32)))) as i32)), p_t_4), rt::fmul(rt::i2f(((c.m.r8((p_b_3).wrapping_add(((2i32) as u32)))) as i32)), l_u_5)); let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(rt::fmul(rt::i2f(((c.m.r8((p_a_2).wrapping_add(((3i32) as u32)))) as i32)), p_t_4), rt::fmul(rt::i2f(((c.m.r8((p_b_3).wrapping_add(((3i32) as u32)))) as i32)), l_u_5)); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_LerpColor", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDust_GetFreeList(c: &mut Ctx, mut p_pool_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_GetFreeList", &PROBE_N, &[(p_pool_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((c.m.r32(sym::gEftGndDust)).wrapping_add(24576)).wrapping_add((((p_pool_1) as u32)).wrapping_mul(12));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_GetFreeList", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDust_AllocPart(c: &mut Ctx, mut p_list_1: u32, mut p_free_2: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_AllocPart", &PROBE_N, &[(p_list_1) as u32, (p_free_2) as u32], false); }
    let mut l_p_3: u32 = 0;
    let mut l_prev_4: u32 = 0;
    let mut l_next_5: u32 = 0;
    let r_ = 'f: {
    l_p_3 = F::List_PopFront(c, p_free_2);
    if (((((l_p_3) != (((0i32) as u32))) as i32)) != 0) {
        F::List_PushBack(c, p_list_1, l_p_3);
        l_prev_4 = c.m.r32(l_p_3);
        l_next_5 = c.m.r32((l_p_3).wrapping_add(4));
        { let _ = F::memset(c, l_p_3, 0i32, 192u32); };
        { let v_ = l_prev_4; let a_ = l_p_3; c.m.w32(a_, v_); };
        { let v_ = l_next_5; let a_ = (l_p_3).wrapping_add(4); c.m.w32(a_, v_); };
        break 'f l_p_3;
    }
    break 'f l_p_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_AllocPart", &PROBE_N, &[], true); }
    r_
}
