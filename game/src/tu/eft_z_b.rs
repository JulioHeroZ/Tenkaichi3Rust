#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn eft_z_b__EftGndDust_FreeParts(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_z_b__EftGndDust_FreeParts", &PROBE_N, &[(p_w_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
    l_p_2 = F::List_GetHead(c, (p_w_1).wrapping_add(204));
    'l1: while (((((l_p_2) != (((0i32) as u32))) as i32)) != 0) {
        { let r_ = (!(2i32)); let a_ = (l_p_2).wrapping_add(188); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        l_p_2 = { let a1_ = c.m.r32((p_w_1).wrapping_add(200)); F::EftGndDust_MoveNode(c, (p_w_1).wrapping_add(204), a1_, l_p_2) };
        { let a_ = (c.m.r32(sym::gEftGndDust)).wrapping_add(25268); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "eft_z_b__EftGndDust_FreeParts", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDustLand_Init(c: &mut Ctx, mut p_task_1: u32, mut p_param_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustLand_Init", &PROBE_N, &[(p_task_1) as u32, (p_param_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_w_3: u32 = 0;
    let mut l_arg_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_life_6: i32 = 0;
    let mut l_fade_7: i32 = 0;
    let mut l_start_8: F32 = F32(0);
    let mut l_step_9: F32 = F32(0);
    let mut l_ang_10: F32 = F32(0);
    let mut l_s_11: F32 = F32(0);
    let mut l_s_12: F32 = F32(0);
    let r_ = 'f: {
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        l_arg_4 = p_param_2;
        l_step_9 = F32(0x40060a91);
        { let _ = F::memset(c, l_w_3, 0i32, 224u32); };
        F::List_Init(c, (l_w_3).wrapping_add(204));
        { let v_ = { let a0_ = (((c.m.r8((l_arg_4).wrapping_add(57)) as i8)) as i32); F::EftGndDust_GetFreeList(c, a0_) }; let a_ = (l_w_3).wrapping_add(200); c.m.w32(a_, v_); };
        F::EftGndDust_GetLightColors(c, (l_arg_4).wrapping_add(48), (l_arg_4).wrapping_add(52));
        F::Vec4_Set(c, (l_arg_4).wrapping_add(32), F32(0x00000000), F32(0xbf800000), F32(0x00000000), F32(0x3f800000));
        { let v_ = ((1i32) as i8); let a_ = (l_arg_4).wrapping_add(57); c.m.w8(a_, (v_) as u8); };
        { let v_ = ((2i32) as i8); let a_ = (l_arg_4).wrapping_add(59); c.m.w8(a_, (v_) as u8); };
        { let v_ = ((10i32) as i16); let a_ = (l_arg_4).wrapping_add(60); c.m.w16(a_, (v_) as u16); };
        { let v_ = ((30i32) as i16); let a_ = (l_arg_4).wrapping_add(62); c.m.w16(a_, (v_) as u16); };
        { let v_ = ((10i32) as i16); let a_ = (l_arg_4).wrapping_add(64); c.m.w16(a_, (v_) as u16); };
        { let v_ = F32(0x3dcccccc); let a_ = (l_arg_4).wrapping_add(72); c.m.w32(a_, (v_).0); };
        { let v_ = ((0i32) as i8); let a_ = (l_arg_4).wrapping_add(58); c.m.w8(a_, (v_) as u8); };
        { let v_ = ((0i32) as i16); let a_ = (l_arg_4).wrapping_add(68); c.m.w16(a_, (v_) as u16); };
        { let v_ = ((0i32) as i16); let a_ = (l_arg_4).wrapping_add(70); c.m.w16(a_, (v_) as u16); };
        { let v_ = ((0i32) as i16); let a_ = (l_arg_4).wrapping_add(66); c.m.w16(a_, (v_) as u16); };
        { let v_ = F32(0x00000000); let a_ = (l_arg_4).wrapping_add(80); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (l_arg_4).wrapping_add(84); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (l_arg_4).wrapping_add(88); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (l_arg_4).wrapping_add(92); c.m.w32(a_, (v_).0); };
        l_start_8 = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40c90fda));
        { let s_ = (l_arg_4).wrapping_add(48); let d_ = fp.wrapping_add(64); rt::memcpy(c, d_, s_, 4); };
        { let s_ = (l_arg_4).wrapping_add(52); let d_ = fp.wrapping_add(68); rt::memcpy(c, d_, s_, 4); };
        {
            l_i_5 = 0i32;
            'l1: loop {
                if !(((((l_i_5) < (3i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_ang_10 = F::EftMath_WrapAngle(c, rt::fadd(l_start_8, rt::fmul(l_step_9, rt::i2f(l_i_5))));
                    l_s_11 = F::Mathf_SinFast(c, l_ang_10);
                    { let a3_ = F::Mathf_CosFast(c, l_ang_10); F::Vec4_Set(c, fp.wrapping_add(16), l_s_11, F32(0x00000000), a3_, F32(0x3f800000)) };
                    F::Vec3_Normalize(c, fp.wrapping_add(16), fp.wrapping_add(16));
                    F::Vec4_Scale(c, fp.wrapping_add(48), fp.wrapping_add(16), F32(0x40000000));
                    F::Vec4_Add(c, fp, l_arg_4, fp.wrapping_add(48));
                    { let a3_ = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3e4ccccc)), F32(0x3f199999)); F::Vec3_Lerp(c, fp.wrapping_add(32), (l_arg_4).wrapping_add(16), fp.wrapping_add(16), a3_) };
                    { let _ = { let a13_ = ((rt::f2i(rt::i2f(((c.m.r16((l_arg_4).wrapping_add(62)) as i16)) as i32))) as i16); let a14_ = ((rt::f2i(rt::i2f(((c.m.r16((l_arg_4).wrapping_add(64)) as i16)) as i32))) as i16); F::EftGndDust_SpawnPieceEx(c, l_w_3, fp, fp.wrapping_add(32), fp.wrapping_add(64), fp.wrapping_add(68), F32(0x40a00000), F32(0x40c00000), F32(0x00000000), F32(0x3f800000), F32(0x00000000), F32(0x3f800000), F32(0x00000000), F32(0x00000000), a13_, a14_, ((10i32) as i16), F32(0x3f333333), F32(0x00000000), 64i32) }; };
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
        l_start_8 = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40c90fda));
        l_step_9 = F32(0x3f490fda);
        {
            l_i_5 = 0i32;
            'l3: loop {
                if !(((((l_i_5) < (8i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    l_ang_10 = F::EftMath_WrapAngle(c, rt::fadd(l_start_8, rt::fmul(l_step_9, rt::i2f(l_i_5))));
                    l_s_12 = F::Mathf_SinFast(c, l_ang_10);
                    { let a3_ = F::Mathf_CosFast(c, l_ang_10); F::Vec4_Set(c, fp.wrapping_add(16), l_s_12, F32(0x00000000), a3_, F32(0x3f800000)) };
                    F::Vec3_Normalize(c, fp.wrapping_add(16), fp.wrapping_add(16));
                    F::Vec4_Scale(c, fp.wrapping_add(48), fp.wrapping_add(16), F32(0x40000000));
                    F::Vec4_Add(c, fp, l_arg_4, fp.wrapping_add(48));
                    { let r_ = F32(0x40400000); let a_ = (fp).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    l_life_6 = ((((c.m.r16((l_arg_4).wrapping_add(62)) as i16)) as i32)).wrapping_add(rt::rem_i32(F::Port_Rand(c), 10i32));
                    l_fade_7 = ((((c.m.r16((l_arg_4).wrapping_add(64)) as i16)) as i32)).wrapping_add(rt::rem_i32(F::Port_Rand(c), 5i32));
                    { let _ = { let a9_ = rt::fmul(F32(c.m.r32((l_arg_4).wrapping_add(96))), F32(0x40f99999)); let a11_ = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40c90fda)); let a16_ = rt::fmul(F32(c.m.r32((l_arg_4).wrapping_add(96))), F32(0x3f333333)); let a17_ = rt::fmul(rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3c23d70a)), F32(0x3d8f5c28)), F32(c.m.r32((l_arg_4).wrapping_add(96)))); F::EftGndDust_SpawnPieceEx(c, l_w_3, fp, fp.wrapping_add(16), (l_arg_4).wrapping_add(48), (l_arg_4).wrapping_add(52), F32(0x3f800000), F32(0x3f800000), F32(0x00000000), F32(0x00000000), a9_, F32(0x3f4ccccc), a11_, F32(0x3e999999), ((rt::f2i(rt::i2f(l_life_6))) as i16), ((rt::f2i(rt::i2f(l_fade_7))) as i16), ((350i32) as i16), a16_, a17_, 0i32) }; };
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
        { let _ = { let a9_ = rt::fmul(F32(c.m.r32((l_arg_4).wrapping_add(96))), F32(0x3f4ccccc)); let a11_ = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40c90fda)); let a16_ = rt::fmul(F32(c.m.r32((l_arg_4).wrapping_add(96))), F32(0x3f333333)); let a17_ = rt::fmul(rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3c23d70a)), F32(0x3cf5c28f)), F32(c.m.r32((l_arg_4).wrapping_add(96)))); F::EftGndDust_SpawnPieceEx(c, l_w_3, l_arg_4, (l_arg_4).wrapping_add(16), (l_arg_4).wrapping_add(48), (l_arg_4).wrapping_add(52), F32(0x3f800000), F32(0x3f800000), F32(0x00000000), F32(0x00000000), a9_, F32(0x3f4ccccc), a11_, F32(0x3e999999), ((rt::f2i(rt::i2f(l_life_6))) as i16), ((rt::f2i(rt::i2f(l_fade_7))) as i16), ((350i32) as i16), a16_, a17_, 0i32) }; };
        { let s_ = l_arg_4; let d_ = l_w_3; rt::memcpy(c, d_, s_, 112); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustLand_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDustLand_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustLand_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    F::eft_z_b__EftGndDust_FreeParts(c, l_w_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustLand_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDustLand_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustLand_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustLand_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDustLand_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustLand_Update", &PROBE_N, &[(p_task_1) as u32], false); }
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
        { let v_ = { let a0_ = { let a_ = (c.m.r32(sym::gEftGndDust)).wrapping_add(25264); c.m.r32(a_) }; F::EftGndDust_GetTex(c, a0_, 2i32) }; let a_ = (l_w_2).wrapping_add(192); c.m.w64(a_, v_); };
    } else {
        F::BtlTask_SetDead(c, p_task_1);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustLand_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDustLand_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustLand_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_w_2: u32 = 0;
    let mut l_p_3: u32 = 0;
    let mut l_far_4: i32 = 0;
    let mut l_fade_5: F32 = F32(0);
    let mut l_dist_6: F32 = F32(0);
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_p_3 = F::List_GetHead(c, (l_w_2).wrapping_add(204));
        F::Vu0Cur_Push(c);
        { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
        'l1: while (((((l_p_3) != (((0i32) as u32))) as i32)) != 0) {
            l_far_4 = 0i32;
            if (((((c.m.r32((l_p_3).wrapping_add(188)) as i32)) & (64i32))) != 0) {
                l_far_4 = 1i32;
            }
            l_fade_5 = F32(0x3f800000);
            F::Vec3_Add(c, fp.wrapping_add(16), (l_p_3).wrapping_add(16), (l_p_3).wrapping_add(32));
            if ((l_far_4) != 0) {
                { let a2_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(544); F::Vec3_Sub(c, fp.wrapping_add(32), fp.wrapping_add(16), a2_) };
                l_dist_6 = F::Vec3_LengthSq(c, fp.wrapping_add(32));
                l_fade_5 = rt::fdiv(rt::fsub(l_dist_6, F32(0x45c80000)), F32(0x467b9000));
                if (((rt::flt(F32(0x46afc800), l_dist_6) as i32)) != 0) {
                    l_fade_5 = F32(0x3f800000);
                }
                if (((rt::flt(l_dist_6, F32(0x45c80000)) as i32)) != 0) {
                    l_fade_5 = F32(0x00000000);
                }
            }
            { let a1_ = rt::fmul(F32(c.m.r32((l_p_3).wrapping_add(64))), F32(c.m.r32((l_w_2).wrapping_add(76)))); let a2_ = rt::fmul(F32(c.m.r32(((l_p_3).wrapping_add(64)).wrapping_add(4))), F32(c.m.r32((l_w_2).wrapping_add(76)))); let a3_ = rt::fmul(F32(c.m.r32(((l_p_3).wrapping_add(64)).wrapping_add(8))), F32(c.m.r32((l_w_2).wrapping_add(76)))); let a4_ = rt::fmul(rt::fmul(F32(c.m.r32(((l_p_3).wrapping_add(64)).wrapping_add(12))), F32(c.m.r32((l_p_3).wrapping_add(180)))), l_fade_5); F::Vec4_Set(c, fp, a1_, a2_, a3_, a4_) };
            { let a3_ = (((c.m.r8((l_w_2).wrapping_add(58)) as i8)) as i32); let a4_ = (if ((l_far_4) != 0) { c.m.r64((l_w_2).wrapping_add(192)) } else { c.m.r64((l_w_2).wrapping_add(184)) }); let a6_ = rt::fmul(rt::fmul(rt::i2f(((c.m.r16((l_p_3).wrapping_add(160)) as i16)) as i32), F32(c.m.r32((l_w_2).wrapping_add(96)))), F32(c.m.r32((l_p_3).wrapping_add(96)))); let a7_ = rt::fmul(rt::fmul(rt::i2f(((c.m.r16((l_p_3).wrapping_add(160)) as i16)) as i32), F32(c.m.r32((l_w_2).wrapping_add(96)))), F32(c.m.r32(((l_p_3).wrapping_add(96)).wrapping_add(4)))); let a8_ = F32(c.m.r32((l_p_3).wrapping_add(164))); F::EftGndDust_DrawPiece(c, fp.wrapping_add(16), fp, (l_p_3).wrapping_add(48), a3_, a4_, l_far_4, a6_, a7_, a8_) };
            l_p_3 = F::List_GetNext(c, l_p_3);
        }
        F::Vu0Cur_Pop(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGndDustLand_Draw", &PROBE_N, &[], true); }
    r_
}
