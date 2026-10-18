#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EftSpdLine_SpawnStreakRing(c: &mut Ctx, mut p_objId_1: i32, mut p_dir_2: u32, mut p_node_3: i32, mut p_count_4: i32, mut p_unused_5: F32, mut p_lifeMin_6: F32, mut p_lifeRange_7: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_SpawnStreakRing", &PROBE_N, &[(p_objId_1) as u32, (p_dir_2) as u32, (p_node_3) as u32, (p_count_4) as u32, (p_unused_5).0, (p_lifeMin_6).0, (p_lifeRange_7).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let mut l_size_8: F32 = F32(0);
    let mut l_minSize_9: F32 = F32(0);
    let mut l_ang_10: F32 = F32(0);
    let mut l_radius_11: F32 = F32(0);
    let mut l_dist_12: F32 = F32(0);
    let mut l_speed_13: F32 = F32(0);
    let mut l_i_14: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp.wrapping_add(64), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(0), (v_).0); }
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(64).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(64).wrapping_add(12), (v_).0); }
        if (((((c.m.r32(sym::gEftSpdLine)) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        l_minSize_9 = F32(0x3f4ccccc);
        l_size_8 = rt::fmul(F::BtlCharApi_GetPartUnk5C(c, p_objId_1, p_node_3), F32(0x3ecccccc));
        if (((rt::flt(l_size_8, l_minSize_9) as i32)) != 0) {
            l_size_8 = rt::fmul(rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)), l_minSize_9);
        }
        F::Vec4_Scale(c, fp.wrapping_add(32), p_dir_2, F32(0xbf800000));
        F::Vec3_Normalize(c, fp.wrapping_add(32), fp.wrapping_add(32));
        F::Vec3_Cross(c, fp.wrapping_add(96), fp.wrapping_add(32), fp.wrapping_add(64));
        F::Vec3_Normalize(c, fp.wrapping_add(96), fp.wrapping_add(96));
        F::Vec3_Cross(c, fp.wrapping_add(80), fp.wrapping_add(32), fp.wrapping_add(96));
        F::Vec3_Normalize(c, fp.wrapping_add(80), fp.wrapping_add(80));
        {
            l_i_14 = 0i32;
            'l1: loop {
                if !(((((l_i_14) < (p_count_4)) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::BtlCharApi_GetNodePos(c, p_objId_1, p_node_3, fp);
                    l_ang_10 = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40c90fda));
                    l_radius_11 = rt::fmul(rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3fe66666)), F32(0x3dcccccc)), l_size_8);
                    l_dist_12 = rt::fmul(rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40000000)), F32(0x3fc00000)), l_size_8);
                    { let a2_ = rt::fmul(F::Mathf_Cos(c, l_ang_10), l_radius_11); F::Vec4_Scale(c, fp.wrapping_add(48), fp.wrapping_add(96), a2_) };
                    F::Vec4_Add(c, fp, fp, fp.wrapping_add(48));
                    { let a2_ = rt::fmul(F::Mathf_Sin(c, l_ang_10), l_radius_11); F::Vec4_Scale(c, fp.wrapping_add(48), fp.wrapping_add(80), a2_) };
                    F::Vec4_Add(c, fp, fp, fp.wrapping_add(48));
                    F::Vec4_Scale(c, fp.wrapping_add(48), fp.wrapping_add(32), l_dist_12);
                    F::Vec4_Sub(c, fp, fp, fp.wrapping_add(48));
                    { let a2_ = rt::fmul(rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40400000)), F32(0x40000000)), l_size_8); F::Vec4_Scale(c, fp.wrapping_add(48), fp.wrapping_add(32), a2_) };
                    F::Vec4_Add(c, fp.wrapping_add(16), fp, fp.wrapping_add(48));
                    l_speed_13 = rt::fmul(rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40000000)), F32(0x3f000000)), rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)));
                    { let a5_ = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), p_lifeRange_7), p_lifeMin_6); F::EftSpdLine_AddStreak(c, p_objId_1, fp, fp.wrapping_add(16), fp.wrapping_add(32), l_speed_13, a5_, F32(0x3d999999)) };
                }
                { let o_ = l_i_14; let n_ = (o_).wrapping_add(1); l_i_14 = n_; };
            }
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_SpawnStreakRing", &PROBE_N, &[], true); }
    r_
}

pub fn EftSpdLine_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftSpdLine)) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((6352u32) as i32)) }; let a_ = sym::gEftSpdLine; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gEftSpdLine); F::memset(c, a0_, 0i32, 6352u32) }; };
    }
    { let a0_ = (c.m.r32(sym::gEftSpdLine)).wrapping_add(6248); let a1_ = F::BtlScene_GetCommonEntry(c, 567i32); F::EftTexSet_Load4(c, a0_, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftSpdLine_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftSpdLine)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftSpdLine); F::BtlPool_Free(c, a0_, a1_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gEftSpdLine; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftSpdLine_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_Update", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::EftSpdLine_UpdateTrails(c);
    F::EftSpdLine_UpdateStreaks(c);
    F::EftSpdLine_UpdateTexture(c, 1i32, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftSpdLine_Stub(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_Stub", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_Stub", &PROBE_N, &[], true); }
    r_
}

pub fn EftSpdLine_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_Draw", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Vu0Cur_Push(c);
    { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
    F::EftSpdLine_DrawTrails(c);
    F::EftSpdLine_DrawStreaks(c);
    F::Vu0Cur_Pop(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftSpdLine_AllocTrail(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_AllocTrail", &PROBE_N, &[], false); }
    let mut l_p_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_p_1 = c.m.r32(sym::gEftSpdLine);
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (30i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32((l_p_1).wrapping_add(64)) as i32)) == (0i32)) as i32)) != 0) {
                    { let _ = F::memset(c, l_p_1, 0i32, 80u32); };
                    break 'f l_p_1;
                }
            }
            { { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; }; { let o_ = l_p_1; let n_ = (o_).wrapping_add(80); l_p_1 = n_; }; };
        }
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_AllocTrail", &PROBE_N, &[], true); }
    r_
}

pub fn EftSpdLine_AddTrail(c: &mut Ctx, mut p_objId_1: i32, p_pos_2: u32, p_dir_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_AddTrail", &PROBE_N, &[(p_objId_1) as u32, (p_pos_2) as u32, (p_dir_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_p_4: u32 = 0;
    let mut l_r_5: i32 = 0;
    let mut l_last_6: u32 = 0;
    rt::memcpy(c, fp, p_pos_2, 16);
    rt::memcpy(c, fp.wrapping_add(16), p_dir_3, 16);
    let r_ = 'f: {
        l_p_4 = F::EftSpdLine_AllocTrail(c);
        if (((((l_p_4) != (((0i32) as u32))) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gEftSpdLine)).wrapping_add(6324); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
                { let v_ = l_p_4; let a_ = (c.m.r32(sym::gEftSpdLine)).wrapping_add(6324); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_p_4).wrapping_add(72); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_p_4).wrapping_add(76); c.m.w32(a_, v_); };
            } else {
                l_last_6 = { let a_ = (c.m.r32(sym::gEftSpdLine)).wrapping_add(6328); c.m.r32(a_) };
                { let v_ = ((0i32) as u32); let a_ = (l_p_4).wrapping_add(76); c.m.w32(a_, v_); };
                { let v_ = l_last_6; let a_ = (l_p_4).wrapping_add(72); c.m.w32(a_, v_); };
            }
            if ((((({ let a_ = (c.m.r32(sym::gEftSpdLine)).wrapping_add(6328); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
                { let v_ = l_p_4; let a_ = ({ let a_ = (c.m.r32(sym::gEftSpdLine)).wrapping_add(6328); c.m.r32(a_) }).wrapping_add(76); c.m.w32(a_, v_); };
            }
            { let v_ = l_p_4; let a_ = (c.m.r32(sym::gEftSpdLine)).wrapping_add(6328); c.m.w32(a_, v_); };
            { let v_ = p_objId_1; let a_ = (l_p_4).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
            F::Vec4_Copy(c, l_p_4, fp);
            F::Vec4_Copy(c, (l_p_4).wrapping_add(16), fp);
            F::Vec4_Copy(c, (l_p_4).wrapping_add(32), fp.wrapping_add(16));
            l_r_5 = F::Port_Rand(c);
            { let v_ = rt::fmul(rt::fadd(rt::fmul(rt::fdiv(rt::i2f(l_r_5), F32(0x4effffff)), F32(0x3dcccccc)), F32(0x3d4ccccc)), rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc))); let a_ = (l_p_4).wrapping_add(52); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x42800000); let a_ = (l_p_4).wrapping_add(56); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x40000000); let a_ = (l_p_4).wrapping_add(60); c.m.w32(a_, (v_).0); };
            { let v_ = 1i32; let a_ = (l_p_4).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
            { let v_ = ((0i32) as u8); let a_ = (l_p_4).wrapping_add(68); c.m.w8(a_, v_); };
            { let v_ = ((0i32) as u8); let a_ = (l_p_4).wrapping_add(69); c.m.w8(a_, v_); };
            { let a_ = (c.m.r32(sym::gEftSpdLine)).wrapping_add(6240); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_AddTrail", &PROBE_N, &[], true); }
    r_
}

pub fn EftSpdLine_UpdateTrails(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_UpdateTrails", &PROBE_N, &[], false); }
    let mut l_p_1: u32 = 0;
    let mut l_cur_2: u32 = 0;
    let mut l_done_3: i32 = 0;
    let r_ = 'f: {
    if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
        l_p_1 = { let a_ = (c.m.r32(sym::gEftSpdLine)).wrapping_add(6324); c.m.r32(a_) };
        'l1: while (((((l_p_1) != (((0i32) as u32))) as i32)) != 0) {
            l_cur_2 = l_p_1;
            l_done_3 = F::EftSpdLine_StepTrail(c, l_p_1);
            l_p_1 = c.m.r32((l_p_1).wrapping_add(76));
            if ((l_done_3) != 0) {
                { let v_ = 0i32; let a_ = (l_cur_2).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
                if (((((c.m.r32((l_cur_2).wrapping_add(72))) == (((0i32) as u32))) as i32)) != 0) {
                    { let v_ = c.m.r32((l_cur_2).wrapping_add(76)); let a_ = (c.m.r32(sym::gEftSpdLine)).wrapping_add(6324); c.m.w32(a_, v_); };
                } else {
                    { let v_ = c.m.r32((l_cur_2).wrapping_add(76)); let a_ = (c.m.r32((l_cur_2).wrapping_add(72))).wrapping_add(76); c.m.w32(a_, v_); };
                }
                if (((((c.m.r32((l_cur_2).wrapping_add(76))) == (((0i32) as u32))) as i32)) != 0) {
                    { let v_ = c.m.r32((l_cur_2).wrapping_add(72)); let a_ = (c.m.r32(sym::gEftSpdLine)).wrapping_add(6328); c.m.w32(a_, v_); };
                } else {
                    { let v_ = c.m.r32((l_cur_2).wrapping_add(72)); let a_ = (c.m.r32((l_cur_2).wrapping_add(76))).wrapping_add(72); c.m.w32(a_, v_); };
                }
                { let v_ = ((0i32) as u32); let a_ = (l_cur_2).wrapping_add(72); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_cur_2).wrapping_add(76); c.m.w32(a_, v_); };
                { let a_ = (c.m.r32(sym::gEftSpdLine)).wrapping_add(6240); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_UpdateTrails", &PROBE_N, &[], true); }
    r_
}

pub fn EftSpdLine_StepTrail(c: &mut Ctx, mut p_p_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_StepTrail", &PROBE_N, &[(p_p_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_unit_2: F32 = F32(0);
    let mut l_r_3: i32 = 0;
    let r_ = 'f: {
        if (((((c.m.r32((p_p_1).wrapping_add(64)) as i32)) & (1i32))) != 0) {
            l_unit_2 = F32(0x3d4ccccc);
            { let a0_ = (c.m.r32((p_p_1).wrapping_add(48)) as i32); F::BtlCharApi_GetFrameMove(c, a0_, fp.wrapping_add(16)) };
            { let a2_ = rt::fmul(rt::fmul({ let a0_ = (c.m.r32((p_p_1).wrapping_add(48)) as i32); F::BtlCharApi_GetHeight(c, a0_) }, l_unit_2), F32(0x3e4ccccc)); F::Vec4_Scale(c, fp, fp.wrapping_add(16), a2_) };
            F::Vec4_Add(c, p_p_1, p_p_1, fp);
            l_r_3 = F::Port_Rand(c);
            { let a2_ = rt::fneg(rt::fmul(rt::fadd(rt::fmul(rt::fdiv(rt::i2f(l_r_3), F32(0x4effffff)), F32(0x3f4ccccc)), F32(0x3f4ccccc)), rt::fmul({ let a0_ = (c.m.r32((p_p_1).wrapping_add(48)) as i32); F::BtlCharApi_GetHeight(c, a0_) }, l_unit_2))); F::Vec4_Scale(c, fp, fp.wrapping_add(16), a2_) };
            F::Vec4_Add(c, (p_p_1).wrapping_add(16), p_p_1, fp);
            { let r_ = F32(0x3f800000); let a_ = (p_p_1).wrapping_add(60); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
        if (((rt::fle(F32(c.m.r32((p_p_1).wrapping_add(60))), F32(0x00000000)) as i32)) != 0) {
            { let v_ = F32(0x00000000); let a_ = (p_p_1).wrapping_add(60); c.m.w32(a_, (v_).0); };
            break 'f 1i32;
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_StepTrail", &PROBE_N, &[], true); }
    r_
}

pub fn EftSpdLine_DrawTrails(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_DrawTrails", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(128) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 128);
    let mut l_p_1: u32 = 0;
    let mut l_inView_2: i32 = 0;
    let r_ = 'f: {
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Mtx_Copy(c, fp, a1_) };
        {
            l_p_1 = { let a_ = (c.m.r32(sym::gEftSpdLine)).wrapping_add(6324); c.m.r32(a_) };
            'l1: loop {
                if !(((((l_p_1) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((c.m.r32((l_p_1).wrapping_add(64)) as i32)) & (1i32))) != 0) {
                        l_inView_2 = { let a0_ = (c.m.r32((l_p_1).wrapping_add(48)) as i32); F::BtlScene_IsCharInView(c, a0_) };
                        F::EftSpdLine_BuildTrailQuad(c, fp.wrapping_add(64), l_p_1);
                        { let a2_ = ((c.m.r32(sym::gEftSpdLine)).wrapping_add(6248)).wrapping_add((((c.m.r8((l_p_1).wrapping_add(68))) as u32)).wrapping_mul(16)); let a6_ = F32(c.m.r32((l_p_1).wrapping_add(56))); let a7_ = c.m.r8((l_p_1).wrapping_add(69)); F::EftSpdLine_DrawQuad(c, fp.wrapping_add(64), fp, a2_, F32(0x437f0000), F32(0x437f0000), F32(0x437f0000), a6_, a7_, l_inView_2) };
                    }
                }
                l_p_1 = c.m.r32((l_p_1).wrapping_add(76));
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_DrawTrails", &PROBE_N, &[], true); }
    r_
}

pub fn EftSpdLine_AllocStreak(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_AllocStreak", &PROBE_N, &[], false); }
    let mut l_p_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_p_1 = (c.m.r32(sym::gEftSpdLine)).wrapping_add(2400);
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (40i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32((l_p_1).wrapping_add(68)) as i32)) == (0i32)) as i32)) != 0) {
                    { let _ = F::memset(c, l_p_1, 0i32, 96u32); };
                    break 'f l_p_1;
                }
            }
            { { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; }; { let o_ = l_p_1; let n_ = (o_).wrapping_add(96); l_p_1 = n_; }; };
        }
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_AllocStreak", &PROBE_N, &[], true); }
    r_
}

pub fn EftSpdLine_AddStreak(c: &mut Ctx, mut p_objId_1: i32, p_pos_2: u32, p_tail_3: u32, p_dir_4: u32, mut p_speed_5: F32, mut p_life_6: F32, mut p_width_7: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_AddStreak", &PROBE_N, &[(p_objId_1) as u32, (p_pos_2) as u32, (p_tail_3) as u32, (p_dir_4) as u32, (p_speed_5).0, (p_life_6).0, (p_width_7).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_p_8: u32 = 0;
    let mut l_last_9: u32 = 0;
    rt::memcpy(c, fp, p_pos_2, 16);
    rt::memcpy(c, fp.wrapping_add(16), p_tail_3, 16);
    rt::memcpy(c, fp.wrapping_add(32), p_dir_4, 16);
    let r_ = 'f: {
        l_p_8 = F::EftSpdLine_AllocStreak(c);
        if (((((l_p_8) != (((0i32) as u32))) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gEftSpdLine)).wrapping_add(6332); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
                { let v_ = l_p_8; let a_ = (c.m.r32(sym::gEftSpdLine)).wrapping_add(6332); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_p_8).wrapping_add(76); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_p_8).wrapping_add(80); c.m.w32(a_, v_); };
            } else {
                l_last_9 = { let a_ = (c.m.r32(sym::gEftSpdLine)).wrapping_add(6336); c.m.r32(a_) };
                { let v_ = ((0i32) as u32); let a_ = (l_p_8).wrapping_add(80); c.m.w32(a_, v_); };
                { let v_ = l_last_9; let a_ = (l_p_8).wrapping_add(76); c.m.w32(a_, v_); };
            }
            if ((((({ let a_ = (c.m.r32(sym::gEftSpdLine)).wrapping_add(6336); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
                { let v_ = l_p_8; let a_ = ({ let a_ = (c.m.r32(sym::gEftSpdLine)).wrapping_add(6336); c.m.r32(a_) }).wrapping_add(80); c.m.w32(a_, v_); };
            }
            { let v_ = l_p_8; let a_ = (c.m.r32(sym::gEftSpdLine)).wrapping_add(6336); c.m.w32(a_, v_); };
            { let v_ = p_objId_1; let a_ = (l_p_8).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
            F::Vec4_Copy(c, l_p_8, fp);
            F::Vec4_Copy(c, (l_p_8).wrapping_add(16), fp.wrapping_add(16));
            F::Vec4_Copy(c, (l_p_8).wrapping_add(32), fp.wrapping_add(32));
            { let v_ = p_speed_5; let a_ = (l_p_8).wrapping_add(52); c.m.w32(a_, (v_).0); };
            { let v_ = p_width_7; let a_ = (l_p_8).wrapping_add(56); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x43200000); let a_ = (l_p_8).wrapping_add(60); c.m.w32(a_, (v_).0); };
            { let v_ = 1i32; let a_ = (l_p_8).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
            { let v_ = ((0i32) as u8); let a_ = (l_p_8).wrapping_add(72); c.m.w8(a_, v_); };
            { let v_ = rt::fdiv(F32(0x43200000), rt::fmul(p_life_6, F32(0x41f00000))); let a_ = (l_p_8).wrapping_add(64); c.m.w32(a_, (v_).0); };
            { let v_ = ((0i32) as u8); let a_ = (l_p_8).wrapping_add(73); c.m.w8(a_, v_); };
            { let a_ = (c.m.r32(sym::gEftSpdLine)).wrapping_add(6244); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_AddStreak", &PROBE_N, &[], true); }
    r_
}

pub fn EftSpdLine_UpdateStreaks(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_UpdateStreaks", &PROBE_N, &[], false); }
    let mut l_p_1: u32 = 0;
    let mut l_cur_2: u32 = 0;
    let mut l_done_3: i32 = 0;
    let r_ = 'f: {
    if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
        l_p_1 = { let a_ = (c.m.r32(sym::gEftSpdLine)).wrapping_add(6332); c.m.r32(a_) };
        'l1: while (((((l_p_1) != (((0i32) as u32))) as i32)) != 0) {
            l_cur_2 = l_p_1;
            l_done_3 = F::EftSpdLine_StepStreak(c, l_p_1);
            l_p_1 = c.m.r32((l_p_1).wrapping_add(80));
            if ((l_done_3) != 0) {
                { let v_ = 0i32; let a_ = (l_cur_2).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
                if (((((c.m.r32((l_cur_2).wrapping_add(76))) == (((0i32) as u32))) as i32)) != 0) {
                    { let v_ = c.m.r32((l_cur_2).wrapping_add(80)); let a_ = (c.m.r32(sym::gEftSpdLine)).wrapping_add(6332); c.m.w32(a_, v_); };
                } else {
                    { let v_ = c.m.r32((l_cur_2).wrapping_add(80)); let a_ = (c.m.r32((l_cur_2).wrapping_add(76))).wrapping_add(80); c.m.w32(a_, v_); };
                }
                if (((((c.m.r32((l_cur_2).wrapping_add(80))) == (((0i32) as u32))) as i32)) != 0) {
                    { let v_ = c.m.r32((l_cur_2).wrapping_add(76)); let a_ = (c.m.r32(sym::gEftSpdLine)).wrapping_add(6336); c.m.w32(a_, v_); };
                } else {
                    { let v_ = c.m.r32((l_cur_2).wrapping_add(76)); let a_ = (c.m.r32((l_cur_2).wrapping_add(80))).wrapping_add(76); c.m.w32(a_, v_); };
                }
                { let v_ = ((0i32) as u32); let a_ = (l_cur_2).wrapping_add(76); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_cur_2).wrapping_add(80); c.m.w32(a_, v_); };
                { let a_ = (c.m.r32(sym::gEftSpdLine)).wrapping_add(6244); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_UpdateStreaks", &PROBE_N, &[], true); }
    r_
}

pub fn EftSpdLine_StepStreak(c: &mut Ctx, mut p_p_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_StepStreak", &PROBE_N, &[(p_p_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        if (((((c.m.r32((p_p_1).wrapping_add(68)) as i32)) & (1i32))) != 0) {
            { let a2_ = F32(c.m.r32((p_p_1).wrapping_add(52))); F::Vec4_Scale(c, fp, (p_p_1).wrapping_add(32), a2_) };
            F::Vec4_Add(c, p_p_1, p_p_1, fp);
            F::Vec4_Add(c, (p_p_1).wrapping_add(16), (p_p_1).wrapping_add(16), fp);
            { let r_ = F32(c.m.r32((p_p_1).wrapping_add(64))); let a_ = (p_p_1).wrapping_add(60); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
        if (((rt::fle(F32(c.m.r32((p_p_1).wrapping_add(60))), F32(0x00000000)) as i32)) != 0) {
            { let v_ = F32(0x00000000); let a_ = (p_p_1).wrapping_add(60); c.m.w32(a_, (v_).0); };
            break 'f 1i32;
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_StepStreak", &PROBE_N, &[], true); }
    r_
}

pub fn EftSpdLine_DrawStreaks(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_DrawStreaks", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(128) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 128);
    let mut l_p_1: u32 = 0;
    let mut l_inView_2: i32 = 0;
    let r_ = 'f: {
        if ((((({ let a_ = (c.m.r32(sym::gEftSpdLine)).wrapping_add(6320); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            break 'f;
        }
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Mtx_Copy(c, fp, a1_) };
        {
            l_p_1 = { let a_ = (c.m.r32(sym::gEftSpdLine)).wrapping_add(6332); c.m.r32(a_) };
            'l1: loop {
                if !(((((l_p_1) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((c.m.r32((l_p_1).wrapping_add(68)) as i32)) & (1i32))) != 0) {
                        l_inView_2 = { let a0_ = (c.m.r32((l_p_1).wrapping_add(48)) as i32); F::BtlScene_IsCharInView(c, a0_) };
                        F::EftSpdLine_BuildStreakQuad(c, fp.wrapping_add(64), l_p_1);
                        { let a2_ = ((c.m.r32(sym::gEftSpdLine)).wrapping_add(6248)).wrapping_add((((c.m.r8((l_p_1).wrapping_add(72))) as u32)).wrapping_mul(16)); let a6_ = F32(c.m.r32((l_p_1).wrapping_add(60))); let a7_ = c.m.r8((l_p_1).wrapping_add(73)); F::EftSpdLine_DrawQuad(c, fp.wrapping_add(64), fp, a2_, F32(0x00000000), F32(0x00000000), F32(0x00000000), a6_, a7_, l_inView_2) };
                    }
                }
                l_p_1 = c.m.r32((l_p_1).wrapping_add(80));
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_DrawStreaks", &PROBE_N, &[], true); }
    r_
}

pub fn EftSpdLine_BuildTrailQuad(c: &mut Ctx, mut p_out_1: u32, mut p_p_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_BuildTrailQuad", &PROBE_N, &[(p_out_1) as u32, (p_p_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let mut l_first_3: u32 = 0;
    let mut l_out1_4: u32 = 0;
    let mut l_dir_5: u32 = 0;
    let mut l_out3_6: u32 = 0;
    let r_ = 'f: {
        l_first_3 = (p_p_2).wrapping_add(16);
        l_out1_4 = (p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16));
        l_dir_5 = (p_p_2).wrapping_add(32);
        l_out3_6 = (p_out_1).wrapping_add((((3i32) as u32)).wrapping_mul(16));
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(64); F::Mtx_InverseRT(c, fp.wrapping_add(48), a1_) };
        F::Vec4_Copy(c, fp, (fp.wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
        F::Vec4_Sub(c, fp.wrapping_add(16), fp, l_first_3);
        F::Vec3_Normalize(c, fp.wrapping_add(16), fp.wrapping_add(16));
        F::Vec3_Cross(c, fp.wrapping_add(32), l_dir_5, fp.wrapping_add(16));
        F::Vec3_Normalize(c, fp.wrapping_add(32), fp.wrapping_add(32));
        { let a2_ = F32(c.m.r32((p_p_2).wrapping_add(52))); F::Vec4_Scale(c, fp.wrapping_add(32), fp.wrapping_add(32), a2_) };
        F::Vec4_Add(c, p_out_1, l_first_3, fp.wrapping_add(32));
        F::Vec4_Sub(c, l_out1_4, l_first_3, fp.wrapping_add(32));
        { let v_ = F32(0x3f800000); let a_ = (l_out1_4).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec4_Sub(c, fp.wrapping_add(16), fp, p_p_2);
        F::Vec3_Normalize(c, fp.wrapping_add(16), fp.wrapping_add(16));
        F::Vec3_Cross(c, fp.wrapping_add(32), l_dir_5, fp.wrapping_add(16));
        F::Vec3_Normalize(c, fp.wrapping_add(32), fp.wrapping_add(32));
        { let a2_ = F32(c.m.r32((p_p_2).wrapping_add(52))); F::Vec4_Scale(c, fp.wrapping_add(32), fp.wrapping_add(32), a2_) };
        F::Vec4_Add(c, (p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(16)), p_p_2, fp.wrapping_add(32));
        F::Vec4_Sub(c, l_out3_6, p_p_2, fp.wrapping_add(32));
        { let v_ = F32(0x3f800000); let a_ = (l_out3_6).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_BuildTrailQuad", &PROBE_N, &[], true); }
    r_
}

pub fn EftSpdLine_BuildStreakQuad(c: &mut Ctx, mut p_out_1: u32, mut p_p_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_BuildStreakQuad", &PROBE_N, &[(p_out_1) as u32, (p_p_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let mut l_second_3: u32 = 0;
    let mut l_out1_4: u32 = 0;
    let mut l_out3_5: u32 = 0;
    let mut l_dir_6: u32 = 0;
    let r_ = 'f: {
        l_second_3 = (p_p_2).wrapping_add(16);
        l_out1_4 = (p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16));
        l_out3_5 = (p_out_1).wrapping_add((((3i32) as u32)).wrapping_mul(16));
        l_dir_6 = (p_p_2).wrapping_add(32);
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(64); F::Mtx_InverseRT(c, fp.wrapping_add(48), a1_) };
        F::Vec4_Copy(c, fp, (fp.wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
        F::Vec4_Sub(c, fp.wrapping_add(16), fp, p_p_2);
        F::Vec3_Normalize(c, fp.wrapping_add(16), fp.wrapping_add(16));
        F::Vec3_Cross(c, fp.wrapping_add(32), l_dir_6, fp.wrapping_add(16));
        F::Vec3_Normalize(c, fp.wrapping_add(32), fp.wrapping_add(32));
        { let a2_ = rt::fmul(F32(c.m.r32((p_p_2).wrapping_add(56))), F32(0x3f000000)); F::Vec4_Scale(c, fp.wrapping_add(32), fp.wrapping_add(32), a2_) };
        F::Vec4_Add(c, p_out_1, p_p_2, fp.wrapping_add(32));
        F::Vec4_Sub(c, l_out1_4, p_p_2, fp.wrapping_add(32));
        { let v_ = F32(0x3f800000); let a_ = (l_out1_4).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec4_Sub(c, fp.wrapping_add(16), fp, l_second_3);
        F::Vec3_Normalize(c, fp.wrapping_add(16), fp.wrapping_add(16));
        F::Vec3_Cross(c, fp.wrapping_add(32), l_dir_6, fp.wrapping_add(16));
        F::Vec3_Normalize(c, fp.wrapping_add(32), fp.wrapping_add(32));
        { let a2_ = rt::fmul(F32(c.m.r32((p_p_2).wrapping_add(56))), F32(0x3f000000)); F::Vec4_Scale(c, fp.wrapping_add(32), fp.wrapping_add(32), a2_) };
        F::Vec4_Add(c, (p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(16)), l_second_3, fp.wrapping_add(32));
        F::Vec4_Sub(c, l_out3_5, l_second_3, fp.wrapping_add(32));
        { let v_ = F32(0x3f800000); let a_ = (l_out3_5).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_BuildStreakQuad", &PROBE_N, &[], true); }
    r_
}

pub fn EftSpdLine_DrawQuad(c: &mut Ctx, mut p_quad_1: u32, mut p_mtx_2: u32, mut p_tex_3: u32, mut p_r_4: F32, mut p_g_5: F32, mut p_b_6: F32, mut p_a_7: F32, mut p_blend_8: u8, mut p_inView_9: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_DrawQuad", &PROBE_N, &[(p_quad_1) as u32, (p_mtx_2) as u32, (p_tex_3) as u32, (p_r_4).0, (p_g_5).0, (p_b_6).0, (p_a_7).0, (p_blend_8) as u32, (p_inView_9) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(624) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 624);
    let mut l_i_10: i32 = 0;
    let r_ = 'f: {
        F::Vec4_Copy(c, (fp.wrapping_add(432)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (p_quad_1).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
        F::Vec4_Copy(c, (fp.wrapping_add(432)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (p_quad_1).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
        F::Vec4_Copy(c, (fp.wrapping_add(432)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (p_quad_1).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
        F::Vec4_Copy(c, (fp.wrapping_add(432)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (p_quad_1).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
        F::Vec4_Set(c, (fp.wrapping_add(496)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), F32(0x00000000), F32(0x00000000), F32(0x3f800000), F32(0x00000000));
        F::Vec4_Set(c, (fp.wrapping_add(496)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), F32(0x3f800000), F32(0x00000000), F32(0x3f800000), F32(0x00000000));
        F::Vec4_Set(c, (fp.wrapping_add(496)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), F32(0x00000000), F32(0x3f800000), F32(0x3f800000), F32(0x00000000));
        F::Vec4_Set(c, (fp.wrapping_add(496)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), F32(0x3f800000), F32(0x3f800000), F32(0x3f800000), F32(0x00000000));
        F::Vec4_Set(c, (fp.wrapping_add(560)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), p_r_4, p_g_5, p_b_6, p_a_7);
        F::Vec4_Set(c, (fp.wrapping_add(560)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), p_r_4, p_g_5, p_b_6, p_a_7);
        F::Vec4_Set(c, (fp.wrapping_add(560)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), p_r_4, p_g_5, p_b_6, p_a_7);
        F::Vec4_Set(c, (fp.wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), p_r_4, p_g_5, p_b_6, p_a_7);
        { let _ = F::memset(c, fp, 0i32, 432u32); };
        {
            l_i_10 = 0i32;
            'l1: loop {
                if !(((((l_i_10) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::ClipVtx_Set(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(432)).wrapping_add((((l_i_10) as u32)).wrapping_mul(16)), (fp.wrapping_add(496)).wrapping_add((((l_i_10) as u32)).wrapping_mul(16)), (fp.wrapping_add(560)).wrapping_add((((l_i_10) as u32)).wrapping_mul(16)));
                    F::ClipVtx_Set(c, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(432)).wrapping_add(((((l_i_10).wrapping_add(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(496)).wrapping_add(((((l_i_10).wrapping_add(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(560)).wrapping_add(((((l_i_10).wrapping_add(1i32)) as u32)).wrapping_mul(16)));
                    F::ClipVtx_Set(c, (fp).wrapping_add((((2i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(432)).wrapping_add(((((l_i_10).wrapping_add(2i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(496)).wrapping_add(((((l_i_10).wrapping_add(2i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(560)).wrapping_add(((((l_i_10).wrapping_add(2i32)) as u32)).wrapping_mul(16)));
                    { let a6_ = c.m.r64(p_tex_3); F::EftGfx_DrawPolyAvgZFront(c, fp, ((p_blend_8) as i32), 0i32, 0i32, p_inView_9, 0i32, a6_, 0i32) };
                }
                { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_DrawQuad", &PROBE_N, &[], true); }
    r_
}

pub fn EftSpdLine_UpdateTexture(c: &mut Ctx, mut p_a_1: i32, mut p_b_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_UpdateTexture", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let r_ = 'f: {
    if (((((({ let a_ = (c.m.r32(sym::gEftSpdLine)).wrapping_add(6240); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gEftSpdLine)).wrapping_add(6244); (c.m.r32(a_) as i32) })) != (0i32)) as i32)) != 0) {
        { let a0_ = (c.m.r32(sym::gEftSpdLine)).wrapping_add(6248); F::EftTexSet_Keep4(c, a0_, p_a_1, p_b_2) };
        { let v_ = 1i32; let a_ = (c.m.r32(sym::gEftSpdLine)).wrapping_add(6320); c.m.w32(a_, (v_) as u32); };
    } else {
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gEftSpdLine)).wrapping_add(6320); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_UpdateTexture", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_GetSparkDir(c: &mut Ctx, mut p_out_1: u32, mut p_objId_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_GetSparkDir", &PROBE_N, &[(p_out_1) as u32, (p_objId_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        F::BtlCharApi_GetVelocity(c, p_objId_2, fp);
        { let a2_ = { let a_ = (c.m.r32(sym::gEftAuraPrm)).wrapping_add(380); F32(c.m.r32(a_)) }; F::Vec4_Scale(c, fp, fp, a2_) };
        { let a1_ = rt::fneg(F32(c.m.r32(fp))); let a2_ = rt::fmul({ let a_ = (c.m.r32(sym::gEftAuraPrm)).wrapping_add(12); F32(c.m.r32(a_)) }, rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff))); let a3_ = rt::fneg(F32(c.m.r32((fp).wrapping_add(8)))); F::Vec4_Set(c, p_out_1, a1_, a2_, a3_, F32(0x3f800000)) };
        F::Vec3_Normalize(c, p_out_1, p_out_1);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftAura_GetSparkDir", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_GetSparkDirAlt(c: &mut Ctx, mut p_out_1: u32, mut p_objId_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_GetSparkDirAlt", &PROBE_N, &[(p_out_1) as u32, (p_objId_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_kind_3: i32 = 0;
    let r_ = 'f: {
        F::BtlCharApi_GetVelocity(c, p_objId_2, fp);
        { let a2_ = { let a_ = (c.m.r32(sym::gEftAuraPrm)).wrapping_add(384); F32(c.m.r32(a_)) }; F::Vec4_Scale(c, fp, fp, a2_) };
        l_kind_3 = F::BtlCharApi_GetActionFxKind(c, p_objId_2);
        if (((((l_kind_3) == (3i32)) as i32)) != 0) {
            { let a2_ = rt::fsub(F32(0x3f800000), F::BtlCharApi_GetFallSpeed(c, p_objId_2)); F::Vec3_Set(c, p_out_1, F32(0x00000000), a2_, F32(0x00000000)) };
        } else {
            if (((((l_kind_3) == (4i32)) as i32)) != 0) {
                { let a2_ = rt::fsub(F32(0xbf800000), F::BtlCharApi_GetFallSpeed(c, p_objId_2)); F::Vec3_Set(c, p_out_1, F32(0x00000000), a2_, F32(0x00000000)) };
            } else {
                { let a1_ = rt::fneg(F32(c.m.r32(fp))); let a2_ = rt::fmul({ let a_ = (c.m.r32(sym::gEftAuraPrm)).wrapping_add(12); F32(c.m.r32(a_)) }, rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff))); let a3_ = rt::fneg(F32(c.m.r32((fp).wrapping_add(8)))); F::Vec3_Set(c, p_out_1, a1_, a2_, a3_) };
            }
        }
        { let v_ = F32(0x3f800000); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec3_Normalize(c, p_out_1, p_out_1);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftAura_GetSparkDirAlt", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_GetFlameSideDir(c: &mut Ctx, mut p_aura_1: u32, mut p_out_2: u32, mut p_objId_3: i32, mut p_power_4: F32, mut p_away_5: F32, mut p_side_6: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_GetFlameSideDir", &PROBE_N, &[(p_aura_1) as u32, (p_out_2) as u32, (p_objId_3) as u32, (p_power_4).0, (p_away_5).0, (p_side_6).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_one_7: F32 = F32(0);
    let r_ = 'f: {
        l_one_7 = F32(0x3f800000);
        { let r_ = rt::fmul(rt::fmul({ let a_ = (c.m.r32(sym::gEftAuraCfg)).wrapping_add(416); F32(c.m.r32(a_)) }, p_power_4), { let a_ = (c.m.r32(sym::gEftAuraPrm)).wrapping_add(388); F32(c.m.r32(a_)) }); let n_ = rt::fadd(p_side_6, r_); p_side_6 = n_; };
        F::BtlCharApi_GetDir(c, p_objId_3, fp);
        { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = l_one_7; let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec3_Normalize(c, fp, fp);
        if ((((F::Port_Rand(c)) & (1i32))) != 0) {
            p_side_6 = rt::fneg(p_side_6);
        }
        F::Vec4_Scale(c, fp, fp, p_side_6);
        { let a1_ = F32(c.m.r32((p_aura_1).wrapping_add(784))); let a3_ = F32(c.m.r32(((p_aura_1).wrapping_add(784)).wrapping_add(8))); F::Vec4_Set(c, fp.wrapping_add(16), a1_, F32(0x00000000), a3_, l_one_7) };
        F::Vec3_Sub(c, p_out_2, fp.wrapping_add(16), (p_aura_1).wrapping_add(112));
        { let v_ = l_one_7; let a_ = (p_out_2).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec3_Normalize(c, p_out_2, p_out_2);
        F::Vec3_Scale(c, p_out_2, p_out_2, p_away_5);
        { let r_ = F32(c.m.r32(fp)); let a_ = p_out_2; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32((fp).wrapping_add(8))); let a_ = (p_out_2).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftAura_GetFlameSideDir", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_GetFlameDragDir(c: &mut Ctx, mut p_aura_1: u32, mut p_pos_2: u32, mut p_out_3: u32, mut p_objId_4: i32, mut p_part_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_GetFlameDragDir", &PROBE_N, &[(p_aura_1) as u32, (p_pos_2) as u32, (p_out_3) as u32, (p_objId_4) as u32, (p_part_5) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_one_6: F32 = F32(0);
    let r_ = 'f: {
        F::Vec3_Sub(c, fp.wrapping_add(16), p_pos_2, (p_aura_1).wrapping_add(112));
        F::Vec3_Sub(c, fp.wrapping_add(32), ((p_aura_1).wrapping_add(304)).wrapping_add((((p_part_5) as u32)).wrapping_mul(16)), (p_aura_1).wrapping_add(128));
        F::Vec3_Sub(c, fp, fp.wrapping_add(16), fp.wrapping_add(32));
        l_one_6 = F32(0x3f800000);
        { let v_ = l_one_6; let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec3_Normalize(c, fp, fp);
        { let a2_ = rt::fmul({ let a0_ = F::Vec3_Dot(c, fp, fp); F::Port_sqrtf(c, a0_) }, { let a_ = (c.m.r32(sym::gEftAuraPrm)).wrapping_add(392); F32(c.m.r32(a_)) }); F::Vec4_Scale(c, fp, fp, a2_) };
        F::Vec3_Scale(c, p_out_3, fp, F32(0xbf800000));
        { let v_ = l_one_6; let a_ = (p_out_3).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftAura_GetFlameDragDir", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_GetFlameOutDir(c: &mut Ctx, mut p_aura_1: u32, mut p_out_2: u32, mut p_objId_3: i32, mut p_part_4: i32, mut p_power_5: F32, mut p_amount_6: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_GetFlameOutDir", &PROBE_N, &[(p_aura_1) as u32, (p_out_2) as u32, (p_objId_3) as u32, (p_part_4) as u32, (p_power_5).0, (p_amount_6).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_one_7: F32 = F32(0);
    let mut l_ratio_8: F32 = F32(0);
    let mut l_t_9: F32 = F32(0);
    let r_ = 'f: {
        l_ratio_8 = rt::fmul(F::BtlCharApi_GetSpeed(c, p_objId_3), { let a_ = (c.m.r32(sym::gEftAuraPrm)).wrapping_add(396); F32(c.m.r32(a_)) });
        l_one_7 = F32(0x3f800000);
        l_t_9 = rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff));
        { let r_ = rt::fmul({ let a_ = (c.m.r32(sym::gEftAuraCfg)).wrapping_add(416); F32(c.m.r32(a_)) }, p_power_5); let n_ = rt::fadd(p_amount_6, r_); p_amount_6 = n_; };
        if (((rt::flt(l_one_7, l_ratio_8) as i32)) != 0) {
            l_ratio_8 = l_one_7;
        }
        { let r_ = rt::fsub(l_one_7, l_ratio_8); let n_ = rt::fmul(p_amount_6, r_); p_amount_6 = n_; };
        { let v_ = rt::fsub(F32(c.m.r32((p_aura_1).wrapping_add(112))), F32(c.m.r32((p_aura_1).wrapping_add(800)))); let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((p_aura_1).wrapping_add(112)).wrapping_add(8))), F32(c.m.r32(((p_aura_1).wrapping_add(800)).wrapping_add(8)))); let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((p_aura_1).wrapping_add(112)).wrapping_add(4))); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((p_aura_1).wrapping_add(800))), rt::fmul(F32(c.m.r32(fp.wrapping_add(16))), l_t_9)); let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((p_aura_1).wrapping_add(800)).wrapping_add(8))), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), l_t_9)); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        F::Vec3_Copy(c, fp.wrapping_add(32), ((p_aura_1).wrapping_add(144)).wrapping_add((((p_part_4) as u32)).wrapping_mul(16)));
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec4_Sub(c, fp.wrapping_add(32), fp.wrapping_add(32), fp);
        F::Vec3_Normalize(c, fp.wrapping_add(32), fp.wrapping_add(32));
        { let a1_ = rt::fmul(F32(c.m.r32(fp.wrapping_add(32))), p_amount_6); let a2_ = rt::fmul(rt::fmul(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4))), p_amount_6), { let a_ = (c.m.r32(sym::gEftAuraPrm)).wrapping_add(400); F32(c.m.r32(a_)) }); let a3_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8))), p_amount_6); F::Vec4_Set(c, p_out_2, a1_, a2_, a3_, l_one_7) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftAura_GetFlameOutDir", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_TurnFlame(c: &mut Ctx, mut p_f_1: u32, mut p_pos_2: u32, mut p_t_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_TurnFlame", &PROBE_N, &[(p_f_1) as u32, (p_pos_2) as u32, (p_t_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_t3_4: F32 = F32(0);
    let mut l_one_5: F32 = F32(0);
    let mut l_dir_6: u32 = 0;
    let mut l_turn_7: F32 = F32(0);
    let r_ = 'f: {
        l_t3_4 = rt::fmul(rt::fmul(p_t_3, p_t_3), p_t_3);
        l_one_5 = F32(0x3f800000);
        l_dir_6 = (p_f_1).wrapping_add(128);
        l_turn_7 = { let a_ = (((c.m.r32(sym::gEftAuraPrm)).wrapping_add(260)).wrapping_add((((c.m.r8((p_f_1).wrapping_add(5))) as u32)).wrapping_mul(32))).wrapping_add(24); F32(c.m.r32(a_)) };
        F::Vec3_Scale(c, fp.wrapping_add(16), (p_f_1).wrapping_add(144), rt::fmul(rt::fmul(rt::fsub(l_one_5, l_t3_4), F32(0x3dcccccc)), l_turn_7));
        F::Vec3_Add(c, l_dir_6, l_dir_6, fp.wrapping_add(16));
        F::Vec3_Scale(c, fp.wrapping_add(16), (p_f_1).wrapping_add(160), rt::fmul(rt::fmul(rt::fsub(l_one_5, p_t_3), F32(0x3da3d70a)), l_turn_7));
        F::Vec3_Add(c, l_dir_6, l_dir_6, fp.wrapping_add(16));
        F::Vec3_Sub(c, fp, p_pos_2, (p_f_1).wrapping_add(80));
        { let v_ = l_one_5; let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec3_Normalize(c, fp, fp);
        F::Vec3_Scale(c, fp.wrapping_add(16), fp, rt::fmul(rt::fmul(l_t3_4, F32(0x3e199999)), l_turn_7));
        F::Vec3_Add(c, l_dir_6, l_dir_6, fp.wrapping_add(16));
        { let v_ = l_one_5; let a_ = ((p_f_1).wrapping_add(128)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec3_Normalize(c, l_dir_6, l_dir_6);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftAura_TurnFlame", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_AccelFlame(c: &mut Ctx, mut p_f_1: u32, mut p_t_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_AccelFlame", &PROBE_N, &[(p_f_1) as u32, (p_t_2).0], false); }
    let r_ = 'f: {
    { let v_ = rt::fmul(rt::fadd(F32(c.m.r32((p_f_1).wrapping_add(48))), rt::fmul(rt::fmul({ let a_ = ((c.m.r32(sym::gEftAuraPrm)).wrapping_add(52)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, rt::fsub(F32(0x3f800000), p_t_2)), { let a_ = (((c.m.r32(sym::gEftAuraPrm)).wrapping_add(260)).wrapping_add((((c.m.r8((p_f_1).wrapping_add(5))) as u32)).wrapping_mul(32))).wrapping_add(28); F32(c.m.r32(a_)) })), rt::fadd(rt::fmul(p_t_2, F32(0x3e4ccccc)), F32(0x3f4ccccc))); let a_ = (p_f_1).wrapping_add(48); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_AccelFlame", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_GetPartOffset(c: &mut Ctx, mut p_aura_1: u32, mut p_out_2: u32, mut p_objId_3: i32, mut p_part_4: i32, mut p_radial_5: F32, mut p_vertical_6: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_GetPartOffset", &PROBE_N, &[(p_aura_1) as u32, (p_out_2) as u32, (p_objId_3) as u32, (p_part_4) as u32, (p_radial_5).0, (p_vertical_6).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let r_ = 'f: {
        { let v_ = F32(c.m.r32((p_aura_1).wrapping_add(800))); let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((p_aura_1).wrapping_add(112)).wrapping_add(4))); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((p_aura_1).wrapping_add(800)).wrapping_add(8))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec3_Copy(c, fp.wrapping_add(16), ((p_aura_1).wrapping_add(144)).wrapping_add((((p_part_4) as u32)).wrapping_mul(16)));
        F::Vec3_Sub(c, fp.wrapping_add(16), fp.wrapping_add(16), fp);
        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec3_Normalize(c, fp.wrapping_add(16), fp.wrapping_add(16));
        { let v_ = F32(0x00000000); let a_ = (p_out_2).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(fp.wrapping_add(16))), p_radial_5); let a_ = p_out_2; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), p_vertical_6); let a_ = (p_out_2).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), p_radial_5); let a_ = (p_out_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftAura_GetPartOffset", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_AllocSpark(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_AllocSpark", &PROBE_N, &[], false); }
    let mut l_p_1: u32 = 0;
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(28); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        l_p_1 = { let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(28); c.m.r32(a_) };
        { let v_ = c.m.r32((l_p_1).wrapping_add(160)); let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(28); c.m.w32(a_, v_); };
    } else {
        if ((((({ let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(40); (c.m.r32(a_) as i32) }) <= (({ let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(8); (c.m.r32(a_) as i32) }).wrapping_sub(1i32))) as i32)) != 0) {
            l_p_1 = ({ let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(24); c.m.r32(a_) }).wrapping_add(((({ let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(40); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); o_ }) as u32)).wrapping_mul(176));
        } else {
            break 'f ((0i32) as u32);
        }
    }
    { let v_ = { let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(32); c.m.r32(a_) }; let a_ = (l_p_1).wrapping_add(160); c.m.w32(a_, v_); };
    { let v_ = l_p_1; let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(32); c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = l_p_1; c.m.w32(a_, (v_) as u32); };
    break 'f l_p_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_AllocSpark", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_FreeSparks(c: &mut Ctx, mut p_objId_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_FreeSparks", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_link_2: u32 = 0;
    let mut l_p_3: u32 = 0;
    let r_ = 'f: {
    l_link_2 = (c.m.r32(sym::gEftAuraPool)).wrapping_add(32);
    'l1: while (((((c.m.r32(l_link_2)) != (((0i32) as u32))) as i32)) != 0) {
        l_p_3 = c.m.r32(l_link_2);
        if (((((((c.m.r8((l_p_3).wrapping_add(4))) as i32)) != (p_objId_1)) as i32)) != 0) {
            l_link_2 = (l_p_3).wrapping_add(160);
        } else {
            { let v_ = c.m.r32((l_p_3).wrapping_add(160)); let a_ = l_link_2; c.m.w32(a_, v_); };
            { let v_ = { let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(28); c.m.r32(a_) }; let a_ = (l_p_3).wrapping_add(160); c.m.w32(a_, v_); };
            { let v_ = l_p_3; let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(28); c.m.w32(a_, v_); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_FreeSparks", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_SpawnSpark(c: &mut Ctx, mut p_aura_1: u32, mut p_objId_2: i32, mut p_emitter_3: i32, mut p_kind_4: i32, mut p_offset_5: u32, mut p_follow_6: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_SpawnSpark", &PROBE_N, &[(p_aura_1) as u32, (p_objId_2) as u32, (p_emitter_3) as u32, (p_kind_4) as u32, (p_offset_5) as u32, (p_follow_6).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_s_7: u32 = 0;
    let mut l_color_8: i32 = 0;
    let mut l_alpha_9: F32 = F32(0);
    let r_ = 'f: {
        l_s_7 = F::EftAura_AllocSpark(c);
        l_color_8 = rt::rem_i32(F::Port_Rand(c), (c.m.r32((p_aura_1).wrapping_add(24)) as i32));
        if (((((l_s_7) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        { let v_ = ((p_objId_2) as u8); let a_ = (l_s_7).wrapping_add(4); c.m.w8(a_, v_); };
        { let v_ = 0i32; let a_ = l_s_7; c.m.w32(a_, (v_) as u32); };
        if (((((c.m.r32(p_aura_1) as i32)) & (128i32))) != 0) {
            { let v_ = 8i32; let a_ = l_s_7; c.m.w32(a_, (v_) as u32); };
        }
        if (((((((((c.m.r32((p_aura_1).wrapping_add(16)) as i32)).wrapping_sub(9i32)) as u32)) < (((2i32) as u32))) as i32)) != 0) {
            { let r_ = 16i32; let a_ = l_s_7; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if ((((((c.m.r32((p_aura_1).wrapping_add(44)) as i32)) == (1i32)) as i32)) != 0) {
            { let r_ = 2i32; let a_ = l_s_7; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        { let v_ = ((p_emitter_3) as u8); let a_ = (l_s_7).wrapping_add(5); c.m.w8(a_, v_); };
        { let v_ = ((p_kind_4) as u8); let a_ = (l_s_7).wrapping_add(6); c.m.w8(a_, v_); };
        { let v_ = F32(0x00000000); let a_ = (l_s_7).wrapping_add(8); c.m.w32(a_, (v_).0); };
        if (((((c.m.r32(l_s_7) as i32)) & (16i32))) != 0) {
            { let v_ = rt::fadd({ let a_ = (c.m.r32(sym::gEftAuraCfg)).wrapping_add(456); F32(c.m.r32(a_)) }, rt::fmul({ let a_ = (c.m.r32(sym::gEftAuraCfg)).wrapping_add(460); F32(c.m.r32(a_)) }, rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = (l_s_7).wrapping_add(12); c.m.w32(a_, (v_).0); };
        } else {
            { let v_ = rt::fadd({ let a_ = (c.m.r32(sym::gEftAuraCfg)).wrapping_add(448); F32(c.m.r32(a_)) }, rt::fmul({ let a_ = (c.m.r32(sym::gEftAuraCfg)).wrapping_add(452); F32(c.m.r32(a_)) }, rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = (l_s_7).wrapping_add(12); c.m.w32(a_, (v_).0); };
        }
        { let v_ = p_follow_6; let a_ = (l_s_7).wrapping_add(32); c.m.w32(a_, (v_).0); };
        F::Vec4_Copy(c, (l_s_7).wrapping_add(64), p_offset_5);
        F::Vec4_Set(c, (l_s_7).wrapping_add(80), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x00000000));
        { let v_ = ((1i32) as u8); let a_ = (l_s_7).wrapping_add(7); c.m.w8(a_, v_); };
        if ((((!(((((c.m.r32(p_aura_1) as i32)) & (32i32))) != 0)) as i32)) != 0) {
            F::Vec3_Copy(c, (l_s_7).wrapping_add(112), ((p_aura_1).wrapping_add(1104)).wrapping_add((((l_color_8) as u32)).wrapping_mul(16)));
            { let v_ = F32(0x00000000); let a_ = ((l_s_7).wrapping_add(112)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            l_alpha_9 = rt::fmul(F32(c.m.r32((((p_aura_1).wrapping_add(1104)).wrapping_add((((l_color_8) as u32)).wrapping_mul(16))).wrapping_add(12))), F32(c.m.r32((p_aura_1).wrapping_add(72))));
            F::EftAura_GetSparkDir(c, fp, p_objId_2);
            if (((((c.m.r32(l_s_7) as i32)) & (8i32))) != 0) {
                { let v_ = { let a_ = (((c.m.r32(sym::gEftAuraCfg)).wrapping_add(480)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = (l_s_7).wrapping_add(24); c.m.w32(a_, (v_).0); };
                { let v_ = { let a_ = (((c.m.r32(sym::gEftAuraCfg)).wrapping_add(480)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = (l_s_7).wrapping_add(28); c.m.w32(a_, (v_).0); };
            } else {
                { let v_ = { let a_ = (((c.m.r32(sym::gEftAuraCfg)).wrapping_add(480)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = (l_s_7).wrapping_add(24); c.m.w32(a_, (v_).0); };
                { let v_ = { let a_ = (((c.m.r32(sym::gEftAuraCfg)).wrapping_add(480)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = (l_s_7).wrapping_add(28); c.m.w32(a_, (v_).0); };
            }
        } else {
            F::Vec3_Copy(c, (l_s_7).wrapping_add(112), ((p_aura_1).wrapping_add(1168)).wrapping_add((((l_color_8) as u32)).wrapping_mul(16)));
            { let v_ = F32(0x00000000); let a_ = ((l_s_7).wrapping_add(112)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            l_alpha_9 = F32(c.m.r32((((p_aura_1).wrapping_add(1168)).wrapping_add((((l_color_8) as u32)).wrapping_mul(16))).wrapping_add(12)));
            F::EftAura_GetSparkDirAlt(c, fp, p_objId_2);
            if (((((c.m.r32(l_s_7) as i32)) & (8i32))) != 0) {
                { let v_ = { let a_ = (((c.m.r32(sym::gEftAuraCfg)).wrapping_add(480)).wrapping_add((((3i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = (l_s_7).wrapping_add(24); c.m.w32(a_, (v_).0); };
                { let v_ = { let a_ = (((c.m.r32(sym::gEftAuraCfg)).wrapping_add(480)).wrapping_add((((3i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = (l_s_7).wrapping_add(28); c.m.w32(a_, (v_).0); };
            } else {
                { let v_ = { let a_ = (((c.m.r32(sym::gEftAuraCfg)).wrapping_add(480)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = (l_s_7).wrapping_add(24); c.m.w32(a_, (v_).0); };
                { let v_ = { let a_ = (((c.m.r32(sym::gEftAuraCfg)).wrapping_add(480)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = (l_s_7).wrapping_add(28); c.m.w32(a_, (v_).0); };
            }
        }
        F::Vec4_Copy(c, (l_s_7).wrapping_add(96), fp);
        { let v_ = F32(0x3f800000); let a_ = ((l_s_7).wrapping_add(96)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec3_Set(c, (l_s_7).wrapping_add(128), F32(0x00000000), F32(0x00000000), F32(0x00000000));
        { let v_ = rt::fdiv(l_alpha_9, { let a_ = (c.m.r32(sym::gEftAuraCfg)).wrapping_add(440); F32(c.m.r32(a_)) }); let a_ = ((l_s_7).wrapping_add(128)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec4_Set(c, (l_s_7).wrapping_add(144), F32(0x00000000), F32(0x00000000), F32(0x3f800000), F32(0x3f800000));
        { let v_ = F32(0x00000000); let a_ = (l_s_7).wrapping_add(16); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul({ let a_ = (c.m.r32(sym::gEftAuraCfg)).wrapping_add(468); F32(c.m.r32(a_)) }, F32(0x40490fda)); let a_ = (l_s_7).wrapping_add(20); c.m.w32(a_, (v_).0); };
        if ((((F::Port_Rand(c)) & (1i32))) != 0) {
            { let v_ = rt::fneg(F32(c.m.r32((l_s_7).wrapping_add(20)))); let a_ = (l_s_7).wrapping_add(20); c.m.w32(a_, (v_).0); };
        }
        if (((((c.m.r32(l_s_7) as i32)) & (4i32))) != 0) {
            { let r_ = { let a_ = (((c.m.r32(sym::gEftAuraCfg)).wrapping_add(512)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = (l_s_7).wrapping_add(20); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = { let a_ = (((c.m.r32(sym::gEftAuraCfg)).wrapping_add(512)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = (l_s_7).wrapping_add(28); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        } else {
            if (((((c.m.r32(l_s_7) as i32)) & (2i32))) != 0) {
                { let r_ = { let a_ = (((c.m.r32(sym::gEftAuraCfg)).wrapping_add(512)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = (l_s_7).wrapping_add(20); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = { let a_ = (((c.m.r32(sym::gEftAuraCfg)).wrapping_add(512)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = (l_s_7).wrapping_add(28); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            }
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftAura_SpawnSpark", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_SpawnSparks(c: &mut Ctx, mut p_aura_1: u32, mut p_objId_2: i32, mut p_emitter_3: i32, mut p_once_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_SpawnSparks", &PROBE_N, &[(p_aura_1) as u32, (p_objId_2) as u32, (p_emitter_3) as u32, (p_once_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_count_5: i32 = 0;
    let mut l_step_6: F32 = F32(0);
    let mut l_len_7: F32 = F32(0);
    let mut l_node_8: i32 = 0;
    let mut l_kind_9: i32 = 0;
    let mut l_i_10: i32 = 0;
    let r_ = 'f: {
        l_count_5 = 1i32;
        l_step_6 = rt::fmul({ let a_ = (c.m.r32(sym::gEftAuraCfg)).wrapping_add(464); F32(c.m.r32(a_)) }, F32(c.m.r32((p_aura_1).wrapping_add(96))));
        F::Vec4_Set(c, fp.wrapping_add(64), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x3f800000));
        l_kind_9 = { let a_ = ((c.m.r32(sym::gEftAuraCfg)).wrapping_add(1188)).wrapping_add((((p_emitter_3) as u32)).wrapping_mul(8)); (c.m.r32(a_) as i32) };
        l_node_8 = { let a_ = (((c.m.r32(sym::gEftAuraCfg)).wrapping_add(1188)).wrapping_add((((p_emitter_3) as u32)).wrapping_mul(8))).wrapping_add(4); (c.m.r32(a_) as i32) };
        F::Vec4_Copy(c, fp, ((p_aura_1).wrapping_add(816)).wrapping_add((((p_emitter_3) as u32)).wrapping_mul(16)));
        F::Vec4_Set(c, fp.wrapping_add(32), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x3f800000));
        if (((((l_node_8) >= (0i32)) as i32)) != 0) {
            F::BtlCharApi_GetNodePos(c, p_objId_2, l_node_8, fp.wrapping_add(16));
            F::Vec4_Sub(c, fp.wrapping_add(48), fp.wrapping_add(16), fp);
            l_len_7 = { let a0_ = F::Vec3_Dot(c, fp.wrapping_add(48), fp.wrapping_add(48)); F::Port_sqrtf(c, a0_) };
            F::Vec4_Copy(c, fp.wrapping_add(32), fp.wrapping_add(48));
            if (((rt::flt(l_step_6, l_len_7) as i32)) != 0) {
                l_count_5 = rt::f2i(rt::fadd(rt::fdiv(l_len_7, l_step_6), F32(0x3f800000)));
            }
        }
        {
            l_i_10 = 0i32;
            'l1: loop {
                if !(((((l_i_10) < (l_count_5)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let _ = { let a5_ = rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)); F::EftAura_SpawnSpark(c, p_aura_1, p_objId_2, p_emitter_3, l_kind_9, fp.wrapping_add(64), a5_) }; };
                    if ((p_once_4) != 0) {
                        break 'l1;
                    }
                }
                { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftAura_SpawnSparks", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_StepSparks(c: &mut Ctx, mut p_aura_1: u32, mut p_objId_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_StepSparks", &PROBE_N, &[(p_aura_1) as u32, (p_objId_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_link_3: u32 = 0;
    let mut l_s_4: u32 = 0;
    let mut l_pos_5: u32 = 0;
    let mut l_drift_6: u32 = 0;
    let mut l_one_7: F32 = F32(0);
    let mut l_t_8: F32 = F32(0);
    let mut l_fadeOut_9: F32 = F32(0);
    let mut l_node_10: i32 = 0;
    let r_ = 'f: {
        l_one_7 = F32(0x3f800000);
        F::Vec4_Set(c, fp, F32(0x00000000), F32(0x00000000), F32(0x00000000), l_one_7);
        l_link_3 = (c.m.r32(sym::gEftAuraPool)).wrapping_add(32);
        'l1: while (((((c.m.r32(l_link_3)) != (((0i32) as u32))) as i32)) != 0) {
            l_s_4 = c.m.r32(l_link_3);
            if (((((((c.m.r8((l_s_4).wrapping_add(4))) as i32)) == (p_objId_2)) as i32)) != 0) {
                if ((((!(((rt::flt(F32(c.m.r32((l_s_4).wrapping_add(8))), F32(c.m.r32((l_s_4).wrapping_add(12)))) as i32)) != 0)) as i32)) != 0) {
                    { let a1_ = ((c.m.r8((l_s_4).wrapping_add(4))) as i32); let a2_ = ((c.m.r8((l_s_4).wrapping_add(5))) as i32); F::EftAura_SpawnSparks(c, p_aura_1, a1_, a2_, 1i32) };
                    { let v_ = c.m.r32((l_s_4).wrapping_add(160)); let a_ = l_link_3; c.m.w32(a_, v_); };
                    { let v_ = { let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(28); c.m.r32(a_) }; let a_ = (l_s_4).wrapping_add(160); c.m.w32(a_, v_); };
                    { let v_ = l_s_4; let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(28); c.m.w32(a_, v_); };
                } else {
                    l_pos_5 = (l_s_4).wrapping_add(48);
                    { let a1_ = ((p_aura_1).wrapping_add(816)).wrapping_add((((c.m.r8((l_s_4).wrapping_add(5))) as u32)).wrapping_mul(16)); F::Vec4_Copy(c, l_pos_5, a1_) };
                    l_node_10 = { let a_ = (((c.m.r32(sym::gEftAuraCfg)).wrapping_add(1188)).wrapping_add((((c.m.r8((l_s_4).wrapping_add(5))) as u32)).wrapping_mul(8))).wrapping_add(4); (c.m.r32(a_) as i32) };
                    if (((((l_node_10) >= (0i32)) as i32)) != 0) {
                        { let a0_ = ((c.m.r8((l_s_4).wrapping_add(4))) as i32); F::BtlCharApi_GetNodePos(c, a0_, l_node_10, fp.wrapping_add(16)) };
                        F::Vec4_Sub(c, fp, fp.wrapping_add(16), l_pos_5);
                        { let a2_ = F32(c.m.r32((l_s_4).wrapping_add(32))); F::Vec3_Scale(c, fp.wrapping_add(32), fp, a2_) };
                        F::Vec4_Add(c, l_pos_5, l_pos_5, fp.wrapping_add(32));
                    }
                    { let v_ = l_one_7; let a_ = ((l_s_4).wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let a2_ = F32(c.m.r32((l_s_4).wrapping_add(28))); F::Vec3_Scale(c, fp.wrapping_add(32), (l_s_4).wrapping_add(96), a2_) };
                    l_drift_6 = (l_s_4).wrapping_add(80);
                    F::Vec4_Add(c, l_drift_6, l_drift_6, fp.wrapping_add(32));
                    l_t_8 = rt::fdiv(F32(c.m.r32((l_s_4).wrapping_add(8))), F32(c.m.r32((l_s_4).wrapping_add(12))));
                    { let r_ = rt::fmul({ let a_ = (c.m.r32(sym::gEftAuraCfg)).wrapping_add(472); F32(c.m.r32(a_)) }, l_t_8); let a_ = (l_s_4).wrapping_add(28); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let r_ = rt::fmul({ let a_ = (c.m.r32(sym::gEftAuraCfg)).wrapping_add(476); F32(c.m.r32(a_)) }, l_t_8); let a_ = (l_s_4).wrapping_add(28); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    F::Vec4_Add(c, l_pos_5, l_pos_5, l_drift_6);
                    { let v_ = l_one_7; let a_ = ((l_s_4).wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let r_ = F32(c.m.r32((l_s_4).wrapping_add(20))); let a_ = (l_s_4).wrapping_add(16); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if (((rt::fge(F32(c.m.r32((l_s_4).wrapping_add(16))), F32(0x40490fda)) as i32)) != 0) {
                        { let r_ = F32(0x40c90fda); let a_ = (l_s_4).wrapping_add(16); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    } else {
                        if (((rt::fle(F32(c.m.r32((l_s_4).wrapping_add(16))), F32(0xc0490fda)) as i32)) != 0) {
                            { let r_ = F32(0x40c90fda); let a_ = (l_s_4).wrapping_add(16); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        }
                    }
                    l_fadeOut_9 = { let a_ = (c.m.r32(sym::gEftAuraCfg)).wrapping_add(444); F32(c.m.r32(a_)) };
                    if (((rt::fle(rt::fsub(F32(c.m.r32((l_s_4).wrapping_add(12))), F32(c.m.r32((l_s_4).wrapping_add(8)))), l_fadeOut_9) as i32)) != 0) {
                        if ((((!(((((c.m.r32(l_s_4) as i32)) & (1i32))) != 0)) as i32)) != 0) {
                            { let r_ = 1i32; let a_ = l_s_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            { let v_ = rt::fdiv(rt::fneg(F32(c.m.r32(((l_s_4).wrapping_add(112)).wrapping_add(12)))), l_fadeOut_9); let a_ = ((l_s_4).wrapping_add(128)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        }
                    } else {
                        if (((rt::flt({ let a_ = (c.m.r32(sym::gEftAuraCfg)).wrapping_add(440); F32(c.m.r32(a_)) }, F32(c.m.r32((l_s_4).wrapping_add(8)))) as i32)) != 0) {
                            { let v_ = F32(0x00000000); let a_ = ((l_s_4).wrapping_add(128)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        }
                    }
                    { let r_ = F32(c.m.r32(((l_s_4).wrapping_add(128)).wrapping_add(12))); let a_ = ((l_s_4).wrapping_add(112)).wrapping_add(12); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if (((rt::flt(F32(c.m.r32(((l_s_4).wrapping_add(112)).wrapping_add(12))), F32(0x00000000)) as i32)) != 0) {
                        { let v_ = F32(0x00000000); let a_ = ((l_s_4).wrapping_add(112)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    }
                    { let r_ = l_one_7; let a_ = (l_s_4).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    l_link_3 = (l_s_4).wrapping_add(160);
                }
            } else {
                l_link_3 = (l_s_4).wrapping_add(160);
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftAura_StepSparks", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_UpdateSparks(c: &mut Ctx, mut p_aura_1: u32, mut p_objId_2: i32, mut p_state_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_UpdateSparks", &PROBE_N, &[(p_aura_1) as u32, (p_objId_2) as u32, (p_state_3) as u32], false); }
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    if ((((!(((((c.m.r32(p_state_3) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < (12i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::EftAura_SpawnSparks(c, p_aura_1, p_objId_2, l_i_4, 0i32);
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        { let r_ = 1i32; let a_ = p_state_3; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    F::EftAura_StepSparks(c, p_aura_1, p_objId_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_UpdateSparks", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_DrawSparks(c: &mut Ctx, mut p_aura_1: u32, mut p_objId_2: i32, mut p_alpha_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_DrawSparks", &PROBE_N, &[(p_aura_1) as u32, (p_objId_2) as u32, (p_alpha_3).0], false); }
    let mut l_link_4: u32 = 0;
    let mut l_tex_5: u32 = 0;
    let mut l_s_6: u32 = 0;
    let mut l_a_7: F32 = F32(0);
    let r_ = 'f: {
    l_link_4 = (c.m.r32(sym::gEftAuraPool)).wrapping_add(32);
    l_tex_5 = (c.m.r32(sym::gEftAuraPool)).wrapping_add(648);
    if (((((c.m.r32(p_aura_1) as i32)) & (32i32))) != 0) {
        l_a_7 = p_alpha_3;
    } else {
        l_a_7 = rt::fmul(rt::fmul(F32(c.m.r32((p_aura_1).wrapping_add(56))), F32(c.m.r32((p_aura_1).wrapping_add(80)))), p_alpha_3);
    }
    'l1: while (((((c.m.r32(l_link_4)) != (((0i32) as u32))) as i32)) != 0) {
        l_s_6 = c.m.r32(l_link_4);
        if (((((((c.m.r8((l_s_6).wrapping_add(4))) as i32)) == (p_objId_2)) as i32)) != 0) {
            { let a0_ = ((rt::f2u(rt::fmul(F32(c.m.r32((l_s_6).wrapping_add(112))), F32(0x437f0000)))) as u8); let a1_ = ((rt::f2u(rt::fmul(F32(c.m.r32(((l_s_6).wrapping_add(112)).wrapping_add(4))), F32(0x437f0000)))) as u8); let a2_ = ((rt::f2u(rt::fmul(F32(c.m.r32(((l_s_6).wrapping_add(112)).wrapping_add(8))), F32(0x437f0000)))) as u8); let a3_ = ((rt::f2u(rt::fmul(rt::fmul(F32(c.m.r32(((l_s_6).wrapping_add(112)).wrapping_add(12))), F32(0x437f0000)), l_a_7))) as u8); let a4_ = F32(c.m.r32((l_s_6).wrapping_add(48))); let a5_ = F32(c.m.r32(((l_s_6).wrapping_add(48)).wrapping_add(4))); let a6_ = F32(c.m.r32(((l_s_6).wrapping_add(48)).wrapping_add(8))); let a7_ = F32(c.m.r32((l_s_6).wrapping_add(144))); let a8_ = F32(c.m.r32(((l_s_6).wrapping_add(144)).wrapping_add(4))); let a9_ = F32(c.m.r32(((l_s_6).wrapping_add(144)).wrapping_add(8))); let a10_ = F32(c.m.r32(((l_s_6).wrapping_add(144)).wrapping_add(12))); let a11_ = F32(c.m.r32((l_s_6).wrapping_add(16))); let a17_ = rt::f2u(rt::fmul(F32(c.m.r32((l_s_6).wrapping_add(24))), F32(0x45800000))); let a19_ = ((c.m.r8((l_s_6).wrapping_add(7))) as i32); F::EftSpr_DrawRot(c, a0_, a1_, a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_, a10_, a11_, 0i32, 0i32, ((((64i32) as i64 as u64)) as u32), ((((64i32) as i64 as u64)) as u32), 0i32, a17_, 0i32, a19_, l_tex_5) };
        }
        l_link_4 = (l_s_6).wrapping_add(160);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_DrawSparks", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_AllocFlame(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_AllocFlame", &PROBE_N, &[], false); }
    let mut l_p_1: u32 = 0;
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(16); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        l_p_1 = { let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(16); c.m.r32(a_) };
        { let v_ = c.m.r32((l_p_1).wrapping_add(192)); let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(16); c.m.w32(a_, v_); };
    } else {
        if ((((({ let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(36); (c.m.r32(a_) as i32) }) <= (({ let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_sub(1i32))) as i32)) != 0) {
            l_p_1 = ({ let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(12); c.m.r32(a_) }).wrapping_add(((({ let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(36); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); o_ }) as u32)).wrapping_mul(208));
        } else {
            break 'f ((0i32) as u32);
        }
    }
    { let v_ = { let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(20); c.m.r32(a_) }; let a_ = (l_p_1).wrapping_add(192); c.m.w32(a_, v_); };
    { let v_ = l_p_1; let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(20); c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = l_p_1; c.m.w32(a_, (v_) as u32); };
    break 'f l_p_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_AllocFlame", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_GetFlameFlags(c: &mut Ctx, mut p_aura_1: u32, mut p_objId_2: i32, mut p_second_3: i32, mut p_alt_4: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_GetFlameFlags", &PROBE_N, &[(p_aura_1) as u32, (p_objId_2) as u32, (p_second_3) as u32, (p_alt_4) as u32], false); }
    let mut l_flags_5: i32 = 0;
    let r_ = 'f: {
    l_flags_5 = 2048i32;
    if (((((p_second_3) == (0i32)) as i32)) != 0) {
        if (((((p_alt_4) == (0i32)) as i32)) != 0) {
            l_flags_5 = 0i32;
        } else {
            l_flags_5 = 1024i32;
        }
    }
    if (((((c.m.r32(p_aura_1) as i32)) & (128i32))) != 0) {
        { let r_ = 256i32; let n_ = ((l_flags_5) | (r_)); l_flags_5 = n_; };
    }
    if (((((((((c.m.r32((p_aura_1).wrapping_add(16)) as i32)).wrapping_sub(9i32)) as u32)) < (((2i32) as u32))) as i32)) != 0) {
        { let r_ = 512i32; let n_ = ((l_flags_5) | (r_)); l_flags_5 = n_; };
    }
    if ((((((c.m.r32((p_aura_1).wrapping_add(44)) as i32)) == (1i32)) as i32)) != 0) {
        { let r_ = 32i32; let n_ = ((l_flags_5) | (r_)); l_flags_5 = n_; };
    } else {
        if ((((((c.m.r32((p_aura_1).wrapping_add(44)) as i32)) == (2i32)) as i32)) != 0) {
            { let r_ = 64i32; let n_ = ((l_flags_5) | (r_)); l_flags_5 = n_; };
        }
    }
    if (((((c.m.r32(p_aura_1) as i32)) & (32i32))) != 0) {
        { let r_ = 4096i32; let n_ = ((l_flags_5) | (r_)); l_flags_5 = n_; };
    }
    break 'f l_flags_5;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_GetFlameFlags", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_GetFlameLife(c: &mut Ctx, mut p_flags_1: i32, mut p_burst_2: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_GetFlameLife", &PROBE_N, &[(p_flags_1) as u32, (p_burst_2).0], false); }
    let mut l_p_3: u32 = 0;
    let mut l_life_4: F32 = F32(0);
    let r_ = 'f: {
    l_p_3 = (c.m.r32(sym::gEftAuraPrm)).wrapping_add(28);
    if ((((p_flags_1) & (512i32))) != 0) {
        l_life_4 = rt::fadd(F32(c.m.r32((l_p_3).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((l_p_3).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff))));
    } else {
        l_life_4 = rt::fadd(F32(c.m.r32((l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff))));
    }
    if ((((p_flags_1) & (32i32))) != 0) {
        { let r_ = rt::fsub(F32(0x3f800000), rt::fmul(p_burst_2, F32(0x3e199999))); let n_ = rt::fmul(l_life_4, r_); l_life_4 = n_; };
    } else {
        if ((((p_flags_1) & (128i32))) != 0) {
            { let r_ = F32(0x3f599999); let n_ = rt::fmul(l_life_4, r_); l_life_4 = n_; };
        }
    }
    break 'f l_life_4;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_GetFlameLife", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_GetFlameSpeed(c: &mut Ctx, mut p_flags_1: i32, mut p_objId_2: i32, mut p_level_3: F32, mut p_burst_4: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_GetFlameSpeed", &PROBE_N, &[(p_flags_1) as u32, (p_objId_2) as u32, (p_level_3).0, (p_burst_4).0], false); }
    let mut l_p_5: u32 = 0;
    let mut l_speed_6: F32 = F32(0);
    let r_ = 'f: {
    l_p_5 = (c.m.r32(sym::gEftAuraPrm)).wrapping_add(52);
    l_speed_6 = rt::fmul(rt::fadd(rt::fmul(F::BtlCharApi_GetSpeed(c, p_objId_2), F32(c.m.r32((l_p_5).wrapping_add((((2i32) as u32)).wrapping_mul(4))))), F32(c.m.r32((l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), p_level_3);
    if ((((p_flags_1) & (256i32))) != 0) {
        if ((((p_flags_1) & (2048i32))) != 0) {
            { let r_ = F32(c.m.r32((l_p_5).wrapping_add((((3i32) as u32)).wrapping_mul(4)))); let n_ = rt::fadd(l_speed_6, r_); l_speed_6 = n_; };
        }
    }
    if ((((p_flags_1) & (128i32))) != 0) {
        { let r_ = F32(c.m.r32((l_p_5).wrapping_add((((4i32) as u32)).wrapping_mul(4)))); let n_ = rt::fadd(l_speed_6, r_); l_speed_6 = n_; };
    } else {
        if ((((p_flags_1) & (32i32))) != 0) {
            { let r_ = rt::fmul(F32(c.m.r32((l_p_5).wrapping_add((((5i32) as u32)).wrapping_mul(4)))), p_burst_4); let n_ = rt::fadd(l_speed_6, r_); l_speed_6 = n_; };
        }
    }
    if ((((!((((p_flags_1) & (2048i32))) != 0)) as i32)) != 0) {
        if ((((p_flags_1) & (1024i32))) != 0) {
            l_p_5 = ((c.m.r32(sym::gEftAuraPrm)).wrapping_add(76)).wrapping_add((((1i32) as u32)).wrapping_mul(8));
        } else {
            l_p_5 = ((c.m.r32(sym::gEftAuraPrm)).wrapping_add(76)).wrapping_add((((0i32) as u32)).wrapping_mul(8));
        }
    } else {
        l_p_5 = ((c.m.r32(sym::gEftAuraPrm)).wrapping_add(76)).wrapping_add((((2i32) as u32)).wrapping_mul(8));
    }
    break 'f rt::fmul(l_speed_6, rt::fadd(F32(c.m.r32((l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_GetFlameSpeed", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_GetFlameAlpha(c: &mut Ctx, mut p_aura_1: u32, mut p_flags_2: i32, mut p_alpha_3: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_GetFlameAlpha", &PROBE_N, &[(p_aura_1) as u32, (p_flags_2) as u32, (p_alpha_3).0], false); }
    let mut l_base_4: u32 = 0;
    let mut l_mul_5: u32 = 0;
    let r_ = 'f: {
    l_base_4 = (c.m.r32(sym::gEftAuraPrm)).wrapping_add(356);
    l_mul_5 = (c.m.r32(sym::gEftAuraPrm)).wrapping_add(364);
    if ((((!((((p_flags_2) & (2048i32))) != 0)) as i32)) != 0) {
        { let r_ = F32(c.m.r32((l_base_4).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let n_ = rt::fmul(p_alpha_3, r_); p_alpha_3 = n_; };
    } else {
        { let r_ = F32(c.m.r32((l_base_4).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let n_ = rt::fmul(p_alpha_3, r_); p_alpha_3 = n_; };
    }
    if ((((p_flags_2) & (128i32))) != 0) {
        { let r_ = F32(c.m.r32((l_mul_5).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let n_ = rt::fmul(p_alpha_3, r_); p_alpha_3 = n_; };
    } else {
        if ((((p_flags_2) & (32i32))) != 0) {
            { let r_ = F32(c.m.r32((l_mul_5).wrapping_add((((3i32) as u32)).wrapping_mul(4)))); let n_ = rt::fmul(p_alpha_3, r_); p_alpha_3 = n_; };
        } else {
            if ((((!((((p_flags_2) & (2048i32))) != 0)) as i32)) != 0) {
                { let r_ = F32(c.m.r32((l_mul_5).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let n_ = rt::fmul(p_alpha_3, r_); p_alpha_3 = n_; };
            } else {
                { let r_ = F32(c.m.r32((l_mul_5).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let n_ = rt::fmul(p_alpha_3, r_); p_alpha_3 = n_; };
            }
        }
    }
    break 'f p_alpha_3;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_GetFlameAlpha", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_GetFlameStretch(c: &mut Ctx, mut p_f_1: u32, mut p_objId_2: i32, mut p_phase_3: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_GetFlameStretch", &PROBE_N, &[(p_f_1) as u32, (p_objId_2) as u32, (p_phase_3) as u32], false); }
    let mut l_mul_4: u32 = 0;
    let mut l_kind_5: u32 = 0;
    let mut l_rnd_6: u32 = 0;
    let mut l_st_7: u32 = 0;
    let mut l_v_8: F32 = F32(0);
    let mut l_k_9: F32 = F32(0);
    let r_ = 'f: {
    l_mul_4 = (c.m.r32(sym::gEftAuraPrm)).wrapping_add(132);
    l_kind_5 = (c.m.r32(sym::gEftAuraPrm)).wrapping_add(156);
    l_rnd_6 = ((c.m.r32(sym::gEftAuraPrm)).wrapping_add(188)).wrapping_add((((c.m.r8((p_f_1).wrapping_add(5))) as u32)).wrapping_mul(12));
    l_st_7 = ((c.m.r32(sym::gEftAuraPrm)).wrapping_add(100)).wrapping_add((((p_phase_3) as u32)).wrapping_mul(4));
    l_v_8 = F32(c.m.r32(l_st_7));
    if (((((c.m.r32(p_f_1) as i32)) & (4096i32))) != 0) {
        break 'f F32(0x00000000);
    }
    if (((((c.m.r32(p_f_1) as i32)) & (256i32))) != 0) {
        if ((((!(((((c.m.r32(p_f_1) as i32)) & (2048i32))) != 0)) as i32)) != 0) {
            { let r_ = F32(c.m.r32((l_kind_5).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let n_ = rt::fmul(l_v_8, r_); l_v_8 = n_; };
        } else {
            { let r_ = F32(c.m.r32((l_kind_5).wrapping_add((((3i32) as u32)).wrapping_mul(4)))); let n_ = rt::fmul(l_v_8, r_); l_v_8 = n_; };
        }
    } else {
        if ((((!(((((c.m.r32(p_f_1) as i32)) & (2048i32))) != 0)) as i32)) != 0) {
            { let r_ = F32(c.m.r32((l_kind_5).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let n_ = rt::fmul(l_v_8, r_); l_v_8 = n_; };
        } else {
            { let r_ = F32(c.m.r32((l_kind_5).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let n_ = rt::fmul(l_v_8, r_); l_v_8 = n_; };
        }
    }
    if (((((c.m.r32(p_f_1) as i32)) & (128i32))) != 0) {
        { let r_ = F32(c.m.r32((l_mul_4).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let n_ = rt::fmul(l_v_8, r_); l_v_8 = n_; };
    } else {
        if (((((c.m.r32(p_f_1) as i32)) & (32i32))) != 0) {
            { let r_ = F32(c.m.r32((l_mul_4).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let n_ = rt::fmul(l_v_8, r_); l_v_8 = n_; };
        } else {
            if (((((c.m.r32(p_f_1) as i32)) & (64i32))) != 0) {
                { let r_ = F32(c.m.r32((l_mul_4).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let n_ = rt::fmul(l_v_8, r_); l_v_8 = n_; };
            }
        }
    }
    l_k_9 = rt::fadd(F32(c.m.r32((l_rnd_6).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((l_rnd_6).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff))));
    { let r_ = rt::fmul(F::BtlCharApi_GetSpeed(c, p_objId_2), F32(c.m.r32((l_rnd_6).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let n_ = rt::fadd(l_k_9, r_); l_k_9 = n_; };
    if (((rt::flt(F32(0x3f800000), l_k_9) as i32)) != 0) {
        l_k_9 = F32(0x3f800000);
    }
    break 'f rt::fmul(l_v_8, l_k_9);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_GetFlameStretch", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_GetFlameSize(c: &mut Ctx, mut p_f_1: u32, mut p_objId_2: i32, mut p_phase_3: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_GetFlameSize", &PROBE_N, &[(p_f_1) as u32, (p_objId_2) as u32, (p_phase_3) as u32], false); }
    let mut l_v_4: F32 = F32(0);
    let r_ = 'f: {
    l_v_4 = { let a_ = ((c.m.r32(sym::gEftAuraPrm)).wrapping_add(116)).wrapping_add((((p_phase_3) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) };
    if (((((c.m.r32(p_f_1) as i32)) & (4096i32))) != 0) {
        break 'f F32(0x00000000);
    }
    if (((((c.m.r32(p_f_1) as i32)) & (256i32))) != 0) {
        if ((((!(((((c.m.r32(p_f_1) as i32)) & (2048i32))) != 0)) as i32)) != 0) {
            { let r_ = { let a_ = ((c.m.r32(sym::gEftAuraPrm)).wrapping_add(172)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let n_ = rt::fmul(l_v_4, r_); l_v_4 = n_; };
        } else {
            { let r_ = { let a_ = ((c.m.r32(sym::gEftAuraPrm)).wrapping_add(172)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let n_ = rt::fmul(l_v_4, r_); l_v_4 = n_; };
        }
    } else {
        { let r_ = { let a_ = ((c.m.r32(sym::gEftAuraPrm)).wrapping_add(172)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let n_ = rt::fmul(l_v_4, r_); l_v_4 = n_; };
    }
    if (((((c.m.r32(p_f_1) as i32)) & (128i32))) != 0) {
        { let r_ = { let a_ = ((c.m.r32(sym::gEftAuraPrm)).wrapping_add(144)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let n_ = rt::fmul(l_v_4, r_); l_v_4 = n_; };
    } else {
        if (((((c.m.r32(p_f_1) as i32)) & (32i32))) != 0) {
            { let r_ = { let a_ = ((c.m.r32(sym::gEftAuraPrm)).wrapping_add(144)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let n_ = rt::fmul(l_v_4, r_); l_v_4 = n_; };
        } else {
            if (((((c.m.r32(p_f_1) as i32)) & (64i32))) != 0) {
                { let r_ = { let a_ = ((c.m.r32(sym::gEftAuraPrm)).wrapping_add(144)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let n_ = rt::fmul(l_v_4, r_); l_v_4 = n_; };
            }
        }
    }
    break 'f l_v_4;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_GetFlameSize", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_GetFlameEnd(c: &mut Ctx, mut p_f_1: u32, mut p_objId_2: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_GetFlameEnd", &PROBE_N, &[(p_f_1) as u32, (p_objId_2) as u32], false); }
    let mut l_p_3: u32 = 0;
    let mut l_v_4: F32 = F32(0);
    let r_ = 'f: {
    l_p_3 = ((c.m.r32(sym::gEftAuraPrm)).wrapping_add(224)).wrapping_add((((c.m.r8((p_f_1).wrapping_add(5))) as u32)).wrapping_mul(12));
    l_v_4 = rt::fadd(F32(c.m.r32((l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff))));
    { let r_ = rt::fmul(F::BtlCharApi_GetSpeed(c, p_objId_2), F32(c.m.r32((l_p_3).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let n_ = rt::fsub(l_v_4, r_); l_v_4 = n_; };
    if ((((!(((((c.m.r32(p_f_1) as i32)) & (2048i32))) != 0)) as i32)) != 0) {
        if (((rt::flt(l_v_4, F32(0x3f266666)) as i32)) != 0) {
            l_v_4 = F32(0x3f266666);
        }
    } else {
        if (((rt::flt(l_v_4, F32(0x3f666666)) as i32)) != 0) {
            l_v_4 = F32(0x3f666666);
        }
    }
    break 'f l_v_4;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_GetFlameEnd", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_SpawnFlame(c: &mut Ctx, mut p_aura_1: u32, mut p_objId_2: i32, mut p_part_3: i32, mut p_pos_4: u32, mut p_offset_5: u32, mut p_follow_6: F32, mut p_alt_7: i32, mut p_second_8: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_SpawnFlame", &PROBE_N, &[(p_aura_1) as u32, (p_objId_2) as u32, (p_part_3) as u32, (p_pos_4) as u32, (p_offset_5) as u32, (p_follow_6).0, (p_alt_7) as u32, (p_second_8) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_f_9: u32 = 0;
    let mut l_color_10: i32 = 0;
    let mut l_lifeScale_11: F32 = F32(0);
    let mut l_level_12: F32 = F32(0);
    let mut l_burst_13: F32 = F32(0);
    let mut l_grow_14: F32 = F32(0);
    let mut l_r1_15: F32 = F32(0);
    let mut l_r2_16: F32 = F32(0);
    let mut l_r3_17: F32 = F32(0);
    let mut l_kind_18: u32 = 0;
    let r_ = 'f: {
        l_f_9 = F::EftAura_AllocFlame(c);
        l_color_10 = rt::rem_i32(F::Port_Rand(c), (c.m.r32((p_aura_1).wrapping_add(24)) as i32));
        F::Vec4_Set(c, fp.wrapping_add(16), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, fp.wrapping_add(32), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, fp.wrapping_add(48), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x3f800000));
        if (((((l_f_9) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        l_lifeScale_11 = F32(c.m.r32((p_aura_1).wrapping_add(68)));
        l_level_12 = F32(c.m.r32((p_aura_1).wrapping_add(72)));
        l_burst_13 = F32(c.m.r32((p_aura_1).wrapping_add(76)));
        { let v_ = ((p_objId_2) as u8); let a_ = (l_f_9).wrapping_add(7); c.m.w8(a_, v_); };
        { let v_ = ((p_part_3) as u8); let a_ = (l_f_9).wrapping_add(9); c.m.w8(a_, v_); };
        { let v_ = { let a_ = ((c.m.r32(sym::gEftAuraCfg)).wrapping_add(1040)).wrapping_add((((p_part_3) as u32)).wrapping_mul(12)); c.m.r8(a_) }; let a_ = (l_f_9).wrapping_add(8); c.m.w8(a_, v_); };
        { let v_ = F::EftAura_GetFlameFlags(c, p_aura_1, p_objId_2, p_second_8, p_alt_7); let a_ = l_f_9; c.m.w32(a_, (v_) as u32); };
        if (((((p_second_8) == (0i32)) as i32)) != 0) {
            { let v_ = ((0i32) as u8); let a_ = (l_f_9).wrapping_add(4); c.m.w8(a_, v_); };
            if (((((c.m.r32(l_f_9) as i32)) & (1024i32))) != 0) {
                { let v_ = ((1i32) as u8); let a_ = (l_f_9).wrapping_add(5); c.m.w8(a_, v_); };
            } else {
                { let v_ = ((0i32) as u8); let a_ = (l_f_9).wrapping_add(5); c.m.w8(a_, v_); };
            }
        } else {
            { let v_ = ((1i32) as u8); let a_ = (l_f_9).wrapping_add(4); c.m.w8(a_, v_); };
            { let v_ = ((2i32) as u8); let a_ = (l_f_9).wrapping_add(5); c.m.w8(a_, v_); };
        }
        { let v_ = { let v_ = { let a0_ = (c.m.r32(l_f_9) as i32); F::EftAura_GetFlameLife(c, a0_, l_burst_13) }; let a_ = (l_f_9).wrapping_add(16); c.m.w32(a_, (v_).0); v_ }; let a_ = (l_f_9).wrapping_add(20); c.m.w32(a_, (v_).0); };
        F::Vec4_Copy(c, (l_f_9).wrapping_add(64), p_offset_5);
        { let v_ = p_follow_6; let a_ = (l_f_9).wrapping_add(52); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = (c.m.r32(l_f_9) as i32); F::EftAura_GetFlameSpeed(c, a0_, p_objId_2, l_level_12, l_burst_13) }; let a_ = (l_f_9).wrapping_add(48); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(l_lifeScale_11, l_burst_13); let a_ = (l_f_9).wrapping_add(56); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32((p_aura_1).wrapping_add(96))), l_level_12); let a_ = (l_f_9).wrapping_add(24); c.m.w32(a_, (v_).0); };
        { let v_ = F::EftAura_GetFlameSize(c, l_f_9, p_objId_2, 0i32); let a_ = (l_f_9).wrapping_add(28); c.m.w32(a_, (v_).0); };
        { let v_ = F::EftAura_GetFlameStretch(c, l_f_9, p_objId_2, 0i32); let a_ = (l_f_9).wrapping_add(32); c.m.w32(a_, (v_).0); };
        l_grow_14 = rt::fmul(F32(c.m.r32((l_f_9).wrapping_add(20))), { let a_ = (c.m.r32(sym::gEftAuraPrm)).wrapping_add(4); F32(c.m.r32(a_)) });
        { let v_ = rt::fdiv(rt::fsub(F::EftAura_GetFlameSize(c, l_f_9, p_objId_2, 1i32), F32(c.m.r32((l_f_9).wrapping_add(28)))), l_grow_14); let a_ = (l_f_9).wrapping_add(36); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fsub(F::EftAura_GetFlameStretch(c, l_f_9, p_objId_2, 1i32), F32(c.m.r32((l_f_9).wrapping_add(32)))), l_grow_14); let a_ = (l_f_9).wrapping_add(40); c.m.w32(a_, (v_).0); };
        { let v_ = F::EftAura_GetFlameEnd(c, l_f_9, p_objId_2); let a_ = (l_f_9).wrapping_add(44); c.m.w32(a_, (v_).0); };
        F::EftAura_GetSparkDir(c, fp, p_objId_2);
        F::EftAura_GetFlameDragDir(c, p_aura_1, p_pos_4, fp.wrapping_add(32), p_objId_2, p_part_3);
        if (((((c.m.r32(l_f_9) as i32)) & (256i32))) != 0) {
            { let v_ = ((rt::rem_i32(F::Port_Rand(c), (c.m.r32((p_aura_1).wrapping_add(32)) as i32))) as u8); let a_ = (l_f_9).wrapping_add(6); c.m.w8(a_, v_); };
        }
        if ((((!(((((c.m.r32(l_f_9) as i32)) & (2048i32))) != 0)) as i32)) != 0) {
            { let v_ = ((0i32) as u8); let a_ = (l_f_9).wrapping_add(11); c.m.w8(a_, v_); };
            { let a2_ = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3e4ccccc)), F32(0x3f800000)); F::Vec3_Scale(c, (l_f_9).wrapping_add(96), (p_aura_1).wrapping_add(1008), a2_) };
            { let v_ = F32(0x00000000); let a_ = ((l_f_9).wrapping_add(96)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            F::Vec4_Sub(c, fp.wrapping_add(64), (p_aura_1).wrapping_add(1024), (l_f_9).wrapping_add(96));
            { let a2_ = rt::fdiv(F32(0x3f800000), F32(c.m.r32((l_f_9).wrapping_add(20)))); F::Vec3_Scale(c, (l_f_9).wrapping_add(112), fp.wrapping_add(64), a2_) };
            { let v_ = rt::fdiv(rt::fmul({ let a1_ = (c.m.r32(l_f_9) as i32); let a2_ = F32(c.m.r32(((p_aura_1).wrapping_add(1008)).wrapping_add(12))); F::EftAura_GetFlameAlpha(c, p_aura_1, a1_, a2_) }, l_level_12), { let a_ = (c.m.r32(sym::gEftAuraPrm)).wrapping_add(44); F32(c.m.r32(a_)) }); let a_ = ((l_f_9).wrapping_add(112)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        } else {
            { let v_ = ((1i32) as u8); let a_ = (l_f_9).wrapping_add(11); c.m.w8(a_, v_); };
            F::Vec3_Scale(c, (l_f_9).wrapping_add(96), ((p_aura_1).wrapping_add(1040)).wrapping_add((((l_color_10) as u32)).wrapping_mul(16)), F32(0x3f800000));
            { let v_ = F32(0x00000000); let a_ = ((l_f_9).wrapping_add(96)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            F::Vec4_Sub(c, fp.wrapping_add(64), ((p_aura_1).wrapping_add(1040)).wrapping_add((((l_color_10) as u32)).wrapping_mul(16)), (l_f_9).wrapping_add(96));
            { let a2_ = rt::fdiv(F32(0x3f800000), F32(c.m.r32((l_f_9).wrapping_add(20)))); F::Vec3_Scale(c, (l_f_9).wrapping_add(112), fp.wrapping_add(64), a2_) };
            { let v_ = rt::fdiv(rt::fmul({ let a1_ = (c.m.r32(l_f_9) as i32); let a2_ = F32(c.m.r32((((p_aura_1).wrapping_add(1040)).wrapping_add((((l_color_10) as u32)).wrapping_mul(16))).wrapping_add(12))); F::EftAura_GetFlameAlpha(c, p_aura_1, a1_, a2_) }, l_level_12), { let a_ = (c.m.r32(sym::gEftAuraPrm)).wrapping_add(44); F32(c.m.r32(a_)) }); let a_ = ((l_f_9).wrapping_add(112)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        }
        l_kind_18 = ((c.m.r32(sym::gEftAuraPrm)).wrapping_add(260)).wrapping_add((((c.m.r8((l_f_9).wrapping_add(5))) as u32)).wrapping_mul(32));
        l_r1_15 = rt::fadd(F32(c.m.r32((l_kind_18).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((l_kind_18).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff))));
        l_r2_16 = rt::fadd(F32(c.m.r32(((l_kind_18).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_kind_18).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff))));
        l_r3_17 = rt::fadd(F32(c.m.r32(((l_kind_18).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_kind_18).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff))));
        { let a3_ = F32(c.m.r32((l_f_9).wrapping_add(56))); F::EftAura_GetFlameSideDir(c, p_aura_1, fp.wrapping_add(16), p_objId_2, a3_, l_r1_15, l_r2_16) };
        { let a3_ = ((c.m.r8((l_f_9).wrapping_add(9))) as i32); let a4_ = F32(c.m.r32((l_f_9).wrapping_add(56))); F::EftAura_GetFlameOutDir(c, p_aura_1, fp.wrapping_add(48), p_objId_2, a3_, a4_, l_r3_17) };
        F::Vec4_Add(c, (l_f_9).wrapping_add(160), fp, fp.wrapping_add(32));
        F::Vec3_Normalize(c, (l_f_9).wrapping_add(160), (l_f_9).wrapping_add(160));
        F::Vec4_Add(c, fp, fp, fp.wrapping_add(16));
        F::Vec4_Add(c, fp, fp, fp.wrapping_add(32));
        F::Vec3_Normalize(c, fp, fp);
        F::Vec4_Add(c, (l_f_9).wrapping_add(128), fp.wrapping_add(48), fp);
        F::Vec3_Normalize(c, (l_f_9).wrapping_add(128), (l_f_9).wrapping_add(128));
        F::Vec4_Copy(c, (l_f_9).wrapping_add(144), (l_f_9).wrapping_add(128));
        F::Vec4_Sub(c, (l_f_9).wrapping_add(160), (l_f_9).wrapping_add(128), (l_f_9).wrapping_add(160));
        F::Vec3_Normalize(c, (l_f_9).wrapping_add(160), (l_f_9).wrapping_add(160));
        { let a2_ = { let a_ = (c.m.r32(sym::gEftAuraPrm)).wrapping_add(8); F32(c.m.r32(a_)) }; F::Vec3_Scale(c, (l_f_9).wrapping_add(176), (l_f_9).wrapping_add(128), a2_) };
        F::Vec4_Add(c, (l_f_9).wrapping_add(80), p_pos_4, p_offset_5);
        F::Vec4_Add(c, (l_f_9).wrapping_add(80), (l_f_9).wrapping_add(80), (l_f_9).wrapping_add(176));
        { let v_ = F32(0x3f800000); let a_ = ((l_f_9).wrapping_add(80)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32((l_f_9).wrapping_add(20))), { let a_ = c.m.r32(sym::gEftAuraPrm); F32(c.m.r32(a_)) }); let a_ = (l_f_9).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = ((rt::rem_i32(F::Port_Rand(c), 2i32)) as u8); let a_ = (l_f_9).wrapping_add(10); c.m.w8(a_, v_); };
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftAura_SpawnFlame", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_SpawnFlames(c: &mut Ctx, mut p_aura_1: u32, mut p_objId_2: i32, mut p_part_3: i32, mut p_once_4: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_SpawnFlames", &PROBE_N, &[(p_aura_1) as u32, (p_objId_2) as u32, (p_part_3) as u32, (p_once_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let mut l_ret_5: i32 = 0;
    let mut l_count_6: i32 = 0;
    let mut l_one_7: F32 = F32(0);
    let mut l_scale_8: F32 = F32(0);
    let mut l_step_9: F32 = F32(0);
    let mut l_len_10: F32 = F32(0);
    let mut l_t_11: F32 = F32(0);
    let mut l_r1_12: F32 = F32(0);
    let mut l_node_13: i32 = 0;
    let mut l_alt_14: i32 = 0;
    let mut l_i_15: i32 = 0;
    let r_ = 'f: {
        l_ret_5 = 0i32;
        l_count_6 = 1i32;
        l_one_7 = F32(0x3f800000);
        l_scale_8 = F32(c.m.r32((p_aura_1).wrapping_add(96)));
        l_step_9 = rt::fmul({ let a_ = (c.m.r32(sym::gEftAuraPrm)).wrapping_add(16); F32(c.m.r32(a_)) }, l_scale_8);
        F::Vec4_Copy(c, fp, ((p_aura_1).wrapping_add(144)).wrapping_add((((p_part_3) as u32)).wrapping_mul(16)));
        F::Vec4_Set(c, fp.wrapping_add(32), F32(0x00000000), F32(0x00000000), F32(0x00000000), l_one_7);
        l_node_13 = { let a_ = (((c.m.r32(sym::gEftAuraCfg)).wrapping_add(1040)).wrapping_add((((p_part_3) as u32)).wrapping_mul(12))).wrapping_add(4); (c.m.r32(a_) as i32) };
        if (((((l_node_13) >= (0i32)) as i32)) != 0) {
            F::BtlCharApi_GetNodePos(c, p_objId_2, l_node_13, fp.wrapping_add(16));
            F::Vec4_Sub(c, fp.wrapping_add(48), fp.wrapping_add(16), fp);
            l_len_10 = { let a0_ = F::Vec3_Dot(c, fp.wrapping_add(48), fp.wrapping_add(48)); F::Port_sqrtf(c, a0_) };
            F::Vec4_Copy(c, fp.wrapping_add(32), fp.wrapping_add(48));
            if (((rt::flt(l_step_9, l_len_10) as i32)) != 0) {
                l_count_6 = rt::f2i(rt::fadd(rt::fdiv(l_len_10, l_step_9), l_one_7));
            }
        }
        {
            l_i_15 = 0i32;
            'l1: loop {
                if !(((((l_i_15) < (l_count_6)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((p_part_3) < (2i32)) as i32)) != 0) {
                        l_t_11 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3ecccccc)), F32(0x3e999999));
                    } else {
                        l_t_11 = rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff));
                    }
                    F::Vec3_Scale(c, fp.wrapping_add(80), fp.wrapping_add(32), l_t_11);
                    l_r1_12 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3d4ccccc)), F32(0x3e999999));
                    { let a5_ = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3d4ccccc)), F32(0x3e999999)); F::EftAura_GetPartOffset(c, p_aura_1, fp.wrapping_add(96), p_objId_2, p_part_3, l_r1_12, a5_) };
                    F::Vec4_Scale(c, fp.wrapping_add(96), fp.wrapping_add(96), l_scale_8);
                    F::Vec3_Copy(c, fp.wrapping_add(64), fp.wrapping_add(96));
                    if ((((c.m.r32((p_aura_1).wrapping_add(4))) & ((1u32).wrapping_shl((p_part_3) as u32)))) != 0) {
                        { let r_ = (!((1u32).wrapping_shl((p_part_3) as u32))); let a_ = (p_aura_1).wrapping_add(4); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
                        l_alt_14 = 1i32;
                    } else {
                        { let r_ = (1u32).wrapping_shl((p_part_3) as u32); let a_ = (p_aura_1).wrapping_add(4); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                        l_alt_14 = 0i32;
                    }
                    if (((((((((c.m.r32((p_aura_1).wrapping_add(44)) as i32)).wrapping_sub(1i32)) as u32)) < (((2i32) as u32))) as i32)) != 0) {
                        if (((((p_part_3) < (6i32)) as i32)) != 0) {
                            l_alt_14 = 0i32;
                        } else {
                            l_alt_14 = 1i32;
                        }
                    }
                    l_ret_5 = F::EftAura_SpawnFlame(c, p_aura_1, p_objId_2, p_part_3, fp, fp.wrapping_add(64), l_t_11, l_alt_14, 0i32);
                    if (((((l_ret_5) != (0i32)) as i32)) != 0) {
                        { let a_ = (p_aura_1).wrapping_add(52); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        if ((((((c.m.r32((p_aura_1).wrapping_add(52)) as i32)) < (45i32)) as i32)) != 0) {
                            if (((((((((p_part_3) < (2i32)) as i32)) != 0) || ((((!((((F::Port_Rand(c)) & (1i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
                                if ((F::EftAura_SpawnFlame(c, p_aura_1, p_objId_2, p_part_3, fp, fp.wrapping_add(64), l_t_11, 0i32, 1i32)) != 0) {
                                    { let a_ = (p_aura_1).wrapping_add(52); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                }
                            }
                        }
                    }
                    if ((p_once_4) != 0) {
                        break 'l1;
                    }
                }
                { let o_ = l_i_15; let n_ = (o_).wrapping_add(1); l_i_15 = n_; };
            }
        }
        break 'f l_ret_5;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftAura_SpawnFlames", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_Start(c: &mut Ctx, mut p_aura_1: u32, mut p_arg_2: u32, mut p_reset_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_Start", &PROBE_N, &[(p_aura_1) as u32, (p_arg_2) as u32, (p_reset_3) as u32], false); }
    let r_ = 'f: {
    { let r_ = 1i32; let a_ = p_aura_1; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    if ((((((c.m.r32((p_arg_2).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) != (0i32)) as i32)) != 0) {
        { let r_ = 32i32; let a_ = p_aura_1; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        { let r_ = 16i32; let a_ = p_aura_1; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if ((p_reset_3) != 0) {
        { let v_ = F32(0x00000000); let a_ = (p_aura_1).wrapping_add(92); c.m.w32(a_, (v_).0); };
        { let v_ = 0i32; let a_ = (p_aura_1).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        { let v_ = F32(0x00000000); let a_ = (p_aura_1).wrapping_add(76); c.m.w32(a_, (v_).0); };
        { let v_ = 0i32; let a_ = (p_aura_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = F32(0x00000000); let a_ = (p_aura_1).wrapping_add(84); c.m.w32(a_, (v_).0); };
        { let v_ = 0i32; let a_ = (p_aura_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = F32(0x3f800000); let a_ = (p_aura_1).wrapping_add(68); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (p_aura_1).wrapping_add(80); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (p_aura_1).wrapping_add(88); c.m.w32(a_, (v_).0); };
        { let v_ = 1i32; let a_ = (p_aura_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_Start", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_FreeFlames(c: &mut Ctx, mut p_objId_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_FreeFlames", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_link_2: u32 = 0;
    let mut l_p_3: u32 = 0;
    let r_ = 'f: {
    l_link_2 = (c.m.r32(sym::gEftAuraPool)).wrapping_add(20);
    'l1: while (((((c.m.r32(l_link_2)) != (((0i32) as u32))) as i32)) != 0) {
        l_p_3 = c.m.r32(l_link_2);
        if (((((((c.m.r8((l_p_3).wrapping_add(7))) as i32)) != (p_objId_1)) as i32)) != 0) {
            l_link_2 = (l_p_3).wrapping_add(192);
        } else {
            { let v_ = c.m.r32((l_p_3).wrapping_add(192)); let a_ = l_link_2; c.m.w32(a_, v_); };
            { let v_ = { let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(16); c.m.r32(a_) }; let a_ = (l_p_3).wrapping_add(192); c.m.w32(a_, v_); };
            { let v_ = l_p_3; let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(16); c.m.w32(a_, v_); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_FreeFlames", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_StepFlames(c: &mut Ctx, mut p_aura_1: u32, mut p_objId_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_StepFlames", &PROBE_N, &[(p_aura_1) as u32, (p_objId_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_link_3: u32 = 0;
    let mut l_f_4: u32 = 0;
    let mut l_move_5: u32 = 0;
    let mut l_color_6: u32 = 0;
    let mut l_t_7: F32 = F32(0);
    let r_ = 'f: {
        l_link_3 = (c.m.r32(sym::gEftAuraPool)).wrapping_add(20);
        'l1: while (((((c.m.r32(l_link_3)) != (((0i32) as u32))) as i32)) != 0) {
            l_f_4 = c.m.r32(l_link_3);
            if (((((((c.m.r8((l_f_4).wrapping_add(7))) as i32)) == (p_objId_2)) as i32)) != 0) {
                if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_f_4).wrapping_add(16)))) as i32)) != 0) {
                    { let a1_ = ((p_aura_1).wrapping_add(144)).wrapping_add((((c.m.r8((l_f_4).wrapping_add(9))) as u32)).wrapping_mul(16)); F::Vec4_Copy(c, fp.wrapping_add(16), a1_) };
                    F::Vec4_Copy(c, fp, fp.wrapping_add(16));
                    if ((((({ let a_ = (((c.m.r32(sym::gEftAuraCfg)).wrapping_add(1040)).wrapping_add((((c.m.r8((l_f_4).wrapping_add(9))) as u32)).wrapping_mul(12))).wrapping_add(4); (c.m.r32(a_) as i32) }) >= (0i32)) as i32)) != 0) {
                        { let a1_ = ((p_aura_1).wrapping_add(464)).wrapping_add((((c.m.r8((l_f_4).wrapping_add(9))) as u32)).wrapping_mul(16)); F::Vec4_Copy(c, fp.wrapping_add(32), a1_) };
                        F::Vec4_Sub(c, fp.wrapping_add(48), fp.wrapping_add(32), fp.wrapping_add(16));
                        { let a2_ = F32(c.m.r32((l_f_4).wrapping_add(52))); F::Vec3_Scale(c, fp.wrapping_add(64), fp.wrapping_add(48), a2_) };
                        F::Vec4_Add(c, fp, fp, fp.wrapping_add(64));
                    }
                    F::Vec4_Add(c, fp, fp, (l_f_4).wrapping_add(64));
                    l_t_7 = rt::fdiv(F32(c.m.r32((l_f_4).wrapping_add(16))), F32(c.m.r32((l_f_4).wrapping_add(20))));
                    F::EftAura_TurnFlame(c, l_f_4, fp, l_t_7);
                    F::EftAura_AccelFlame(c, l_f_4, l_t_7);
                    { let a2_ = rt::fadd(F32(c.m.r32((l_f_4).wrapping_add(48))), F32(c.m.r32((p_aura_1).wrapping_add(60)))); F::Vec3_Scale(c, fp.wrapping_add(64), (l_f_4).wrapping_add(128), a2_) };
                    l_move_5 = (l_f_4).wrapping_add(176);
                    F::Vec3_Add(c, l_move_5, l_move_5, fp.wrapping_add(64));
                    F::Vec4_Add(c, (l_f_4).wrapping_add(80), fp, l_move_5);
                    { let v_ = F32(0x3f800000); let a_ = ((l_f_4).wrapping_add(80)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let r_ = F32(c.m.r32((l_f_4).wrapping_add(36))); let a_ = (l_f_4).wrapping_add(28); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let r_ = F32(c.m.r32((l_f_4).wrapping_add(40))); let a_ = (l_f_4).wrapping_add(32); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if (((rt::flt(F32(c.m.r32((l_f_4).wrapping_add(28))), F32(0x00000000)) as i32)) != 0) {
                        { let v_ = F32(0x00000000); let a_ = (l_f_4).wrapping_add(28); c.m.w32(a_, (v_).0); };
                    }
                    if (((rt::flt(F32(c.m.r32((l_f_4).wrapping_add(32))), F32(0x00000000)) as i32)) != 0) {
                        { let v_ = F32(0x00000000); let a_ = (l_f_4).wrapping_add(32); c.m.w32(a_, (v_).0); };
                    }
                    if ((((!(((((c.m.r32(l_f_4) as i32)) & (8i32))) != 0)) as i32)) != 0) {
                        if (((rt::flt({ let a_ = (c.m.r32(sym::gEftAuraPrm)).wrapping_add(4); F32(c.m.r32(a_)) }, rt::fsub(F32(0x3f800000), rt::fdiv(F32(c.m.r32((l_f_4).wrapping_add(16))), F32(c.m.r32((l_f_4).wrapping_add(20)))))) as i32)) != 0) {
                            { let v_ = rt::fdiv(rt::fsub({ let a1_ = ((c.m.r8((l_f_4).wrapping_add(7))) as i32); F::EftAura_GetFlameSize(c, l_f_4, a1_, 2i32) }, F32(c.m.r32((l_f_4).wrapping_add(28)))), F32(c.m.r32((l_f_4).wrapping_add(16)))); let a_ = (l_f_4).wrapping_add(36); c.m.w32(a_, (v_).0); };
                            { let v_ = rt::fdiv(rt::fsub({ let a1_ = ((c.m.r8((l_f_4).wrapping_add(7))) as i32); F::EftAura_GetFlameStretch(c, l_f_4, a1_, 2i32) }, F32(c.m.r32((l_f_4).wrapping_add(32)))), F32(c.m.r32((l_f_4).wrapping_add(16)))); let a_ = (l_f_4).wrapping_add(40); c.m.w32(a_, (v_).0); };
                            { let r_ = 8i32; let a_ = l_f_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                    }
                    if ((((!(((((c.m.r32(l_f_4) as i32)) & (1i32))) != 0)) as i32)) != 0) {
                        if (((rt::fle({ let a_ = (c.m.r32(sym::gEftAuraPrm)).wrapping_add(44); F32(c.m.r32(a_)) }, rt::fsub(F32(c.m.r32((l_f_4).wrapping_add(20))), F32(c.m.r32((l_f_4).wrapping_add(16))))) as i32)) != 0) {
                            { let v_ = F32(0x00000000); let a_ = ((l_f_4).wrapping_add(112)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                            { let r_ = 1i32; let a_ = l_f_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                    }
                    if ((((!(((((c.m.r32(l_f_4) as i32)) & (2i32))) != 0)) as i32)) != 0) {
                        if (((rt::fle(F32(c.m.r32((l_f_4).wrapping_add(16))), { let a_ = (c.m.r32(sym::gEftAuraPrm)).wrapping_add(48); F32(c.m.r32(a_)) }) as i32)) != 0) {
                            { let v_ = rt::fneg(rt::fmul(rt::fsub(F32(c.m.r32(((l_f_4).wrapping_add(96)).wrapping_add(12))), F32(c.m.r32(((p_aura_1).wrapping_add(1024)).wrapping_add(12)))), rt::fdiv(F32(0x3f800000), rt::fadd(F32(c.m.r32((l_f_4).wrapping_add(16))), F32(0x3f800000))))); let a_ = ((l_f_4).wrapping_add(112)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                            { let r_ = 2i32; let a_ = l_f_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                    }
                    l_color_6 = (l_f_4).wrapping_add(96);
                    F::Vec4_Add(c, l_color_6, l_color_6, (l_f_4).wrapping_add(112));
                    F::Vec4_Clamp(c, l_color_6, l_color_6, F32(0x00000000), F32(0x3f800000));
                    { let r_ = F32(0x3f800000); let a_ = (l_f_4).wrapping_add(16); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if (((rt::fle(F32(c.m.r32((l_f_4).wrapping_add(16))), F32(c.m.r32((l_f_4).wrapping_add(12)))) as i32)) != 0) {
                        if ((((!(((((c.m.r32(l_f_4) as i32)) & (16i32))) != 0)) as i32)) != 0) {
                            { let v_ = rt::fdiv(rt::fsub({ let a1_ = ((c.m.r8((l_f_4).wrapping_add(7))) as i32); F::EftAura_GetFlameSize(c, l_f_4, a1_, 3i32) }, F32(c.m.r32((l_f_4).wrapping_add(28)))), F32(c.m.r32((l_f_4).wrapping_add(16)))); let a_ = (l_f_4).wrapping_add(36); c.m.w32(a_, (v_).0); };
                            { let v_ = rt::fdiv(rt::fsub({ let a1_ = ((c.m.r8((l_f_4).wrapping_add(7))) as i32); F::EftAura_GetFlameStretch(c, l_f_4, a1_, 3i32) }, F32(c.m.r32((l_f_4).wrapping_add(32)))), F32(c.m.r32((l_f_4).wrapping_add(16)))); let a_ = (l_f_4).wrapping_add(40); c.m.w32(a_, (v_).0); };
                            { let r_ = 16i32; let a_ = l_f_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                        if ((((!(((((c.m.r32(l_f_4) as i32)) & (4i32))) != 0)) as i32)) != 0) {
                            if (((((c.m.r32(l_f_4) as i32)) & (2048i32))) != 0) {
                                { let r_ = 4i32; let a_ = l_f_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            } else {
                                if (({ let a1_ = ((c.m.r8((l_f_4).wrapping_add(7))) as i32); let a2_ = ((c.m.r8((l_f_4).wrapping_add(9))) as i32); F::EftAura_SpawnFlames(c, p_aura_1, a1_, a2_, 1i32) }) != 0) {
                                    { let r_ = 4i32; let a_ = l_f_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                }
                            }
                        }
                    }
                    l_link_3 = (l_f_4).wrapping_add(192);
                } else {
                    if ((((!(((((c.m.r32(l_f_4) as i32)) & (4i32))) != 0)) as i32)) != 0) {
                        if (({ let a1_ = ((c.m.r8((l_f_4).wrapping_add(7))) as i32); let a2_ = ((c.m.r8((l_f_4).wrapping_add(9))) as i32); F::EftAura_SpawnFlames(c, p_aura_1, a1_, a2_, 1i32) }) != 0) {
                            { let r_ = 4i32; let a_ = l_f_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                    } else {
                        { let v_ = c.m.r32((l_f_4).wrapping_add(192)); let a_ = l_link_3; c.m.w32(a_, v_); };
                        { let v_ = { let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(16); c.m.r32(a_) }; let a_ = (l_f_4).wrapping_add(192); c.m.w32(a_, v_); };
                        { let v_ = l_f_4; let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(16); c.m.w32(a_, v_); };
                        { let a_ = (p_aura_1).wrapping_add(52); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    }
                }
            } else {
                l_link_3 = (l_f_4).wrapping_add(192);
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftAura_StepFlames", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_UpdateState(c: &mut Ctx, mut p_aura_1: u32, mut p_objId_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_UpdateState", &PROBE_N, &[(p_aura_1) as u32, (p_objId_2) as u32], false); }
    let mut l_step_3: F32 = F32(0);
    let r_ = 'f: {
    'l1: while ((((((c.m.r32((p_aura_1).wrapping_add(36)) as i32)) < (10i32)) as i32)) != 0) {
        { let _ = { let a2_ = (c.m.r32((p_aura_1).wrapping_add(36)) as i32); F::EftAura_SpawnFlames(c, p_aura_1, p_objId_2, a2_, 0i32) }; };
        { let a_ = (p_aura_1).wrapping_add(36); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    }
    if ((((((c.m.r32((p_aura_1).wrapping_add(44)) as i32)) != (0i32)) as i32)) != 0) {
        if ((((((c.m.r32((p_aura_1).wrapping_add(44)) as i32)) == (1i32)) as i32)) != 0) {
            { let r_ = F32(0x3e199999); let a_ = (p_aura_1).wrapping_add(76); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let v_ = F32(0x3f999999); let a_ = (p_aura_1).wrapping_add(68); c.m.w32(a_, (v_).0); };
            if (((rt::fle(F32(0x3f800000), F32(c.m.r32((p_aura_1).wrapping_add(76)))) as i32)) != 0) {
                { let v_ = F32(0x3f800000); let a_ = (p_aura_1).wrapping_add(76); c.m.w32(a_, (v_).0); };
            }
        } else {
            if ((((((c.m.r32((p_aura_1).wrapping_add(44)) as i32)) == (2i32)) as i32)) != 0) {
                { let v_ = F32(0x3f800000); let a_ = (p_aura_1).wrapping_add(76); c.m.w32(a_, (v_).0); };
                { let r_ = F32(0x3f800000); let a_ = (p_aura_1).wrapping_add(64); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::fle({ let a_ = (c.m.r32(sym::gEftAuraCfg)).wrapping_add(412); F32(c.m.r32(a_)) }, F32(c.m.r32((p_aura_1).wrapping_add(64)))) as i32)) != 0) {
                    { let v_ = 3i32; let a_ = (p_aura_1).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                    { let v_ = F32(0x00000000); let a_ = (p_aura_1).wrapping_add(64); c.m.w32(a_, (v_).0); };
                    { let v_ = 1i32; let a_ = (p_aura_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 1i32; let a_ = (p_aura_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                }
            } else {
                { let r_ = F32(0x3dcccccc); let a_ = (p_aura_1).wrapping_add(76); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::fle(F32(c.m.r32((p_aura_1).wrapping_add(76))), F32(0x00000000)) as i32)) != 0) {
                    { let v_ = F32(0x00000000); let a_ = (p_aura_1).wrapping_add(76); c.m.w32(a_, (v_).0); };
                    { let r_ = F32(0x3e4ccccc); let a_ = (p_aura_1).wrapping_add(68); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if (((rt::fle(F32(c.m.r32((p_aura_1).wrapping_add(68))), F32(0x3f800000)) as i32)) != 0) {
                        { let v_ = F32(0x3f800000); let a_ = (p_aura_1).wrapping_add(68); c.m.w32(a_, (v_).0); };
                    }
                }
                if (((((((rt::fle(F32(c.m.r32((p_aura_1).wrapping_add(68))), F32(0x3f800000)) as i32)) != 0) && (((rt::fle(F32(c.m.r32((p_aura_1).wrapping_add(76))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                    { let v_ = 0i32; let a_ = (p_aura_1).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                }
            }
        }
    }
    { let v_ = rt::fmul({ let a_ = (c.m.r32(sym::gEftAuraCfg)).wrapping_add(420); F32(c.m.r32(a_)) }, F32(c.m.r32((p_aura_1).wrapping_add(76)))); let a_ = (p_aura_1).wrapping_add(60); c.m.w32(a_, (v_).0); };
    if ((((((c.m.r32((p_aura_1).wrapping_add(8)) as i32)) == (1i32)) as i32)) != 0) {
        if (((rt::flt(F32(0x00000000), F32(c.m.r32((p_aura_1).wrapping_add(80)))) as i32)) != 0) {
            { let r_ = rt::fdiv(F32(0x3f800000), { let a_ = (c.m.r32(sym::gEftAuraCfg)).wrapping_add(436); F32(c.m.r32(a_)) }); let a_ = (p_aura_1).wrapping_add(80); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::fle(F32(c.m.r32((p_aura_1).wrapping_add(80))), F32(0x00000000)) as i32)) != 0) {
                { let v_ = F32(0x00000000); let a_ = (p_aura_1).wrapping_add(80); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = (p_aura_1).wrapping_add(84); c.m.w32(a_, (v_).0); };
                { let v_ = 0i32; let a_ = (p_aura_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            }
        }
    } else {
        if (((rt::flt(F32(c.m.r32((p_aura_1).wrapping_add(80))), F32(0x3f800000)) as i32)) != 0) {
            { let r_ = rt::fdiv(F32(0x3f800000), { let a_ = (c.m.r32(sym::gEftAuraCfg)).wrapping_add(432); F32(c.m.r32(a_)) }); let a_ = (p_aura_1).wrapping_add(84); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = F32(c.m.r32((p_aura_1).wrapping_add(84))); let a_ = (p_aura_1).wrapping_add(80); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::fle(F32(0x3f800000), F32(c.m.r32((p_aura_1).wrapping_add(80)))) as i32)) != 0) {
                { let v_ = F32(0x3f800000); let a_ = (p_aura_1).wrapping_add(80); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = (p_aura_1).wrapping_add(84); c.m.w32(a_, (v_).0); };
            }
        }
    }
    if ((((((c.m.r32((p_aura_1).wrapping_add(12)) as i32)) == (1i32)) as i32)) != 0) {
        if (((rt::flt(F32(0x00000000), F32(c.m.r32((p_aura_1).wrapping_add(88)))) as i32)) != 0) {
            { let r_ = rt::fdiv(F32(0x3f800000), { let a_ = (c.m.r32(sym::gEftAuraCfg)).wrapping_add(428); F32(c.m.r32(a_)) }); let a_ = (p_aura_1).wrapping_add(88); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::fle(F32(c.m.r32((p_aura_1).wrapping_add(88))), F32(0x00000000)) as i32)) != 0) {
                { let v_ = F32(0x00000000); let a_ = (p_aura_1).wrapping_add(88); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = (p_aura_1).wrapping_add(92); c.m.w32(a_, (v_).0); };
                { let v_ = 0i32; let a_ = (p_aura_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
            }
        }
    } else {
        if (((rt::flt(F32(c.m.r32((p_aura_1).wrapping_add(88))), F32(0x3f800000)) as i32)) != 0) {
            { let r_ = rt::fdiv(F32(0x3f800000), { let a_ = (c.m.r32(sym::gEftAuraCfg)).wrapping_add(424); F32(c.m.r32(a_)) }); let a_ = (p_aura_1).wrapping_add(92); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = F32(c.m.r32((p_aura_1).wrapping_add(92))); let a_ = (p_aura_1).wrapping_add(88); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::fle(F32(0x3f800000), F32(c.m.r32((p_aura_1).wrapping_add(88)))) as i32)) != 0) {
                { let v_ = F32(0x3f800000); let a_ = (p_aura_1).wrapping_add(88); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = (p_aura_1).wrapping_add(92); c.m.w32(a_, (v_).0); };
            }
        }
    }
    if ((((((c.m.r32((p_aura_1).wrapping_add(40)) as i32)) != (0i32)) as i32)) != 0) {
        if ((((((c.m.r32((p_aura_1).wrapping_add(40)) as i32)) == (1i32)) as i32)) != 0) {
            l_step_3 = rt::fdiv(F32(0x3f800000), { let a_ = (c.m.r32(sym::gEftAuraCfg)).wrapping_add(404); F32(c.m.r32(a_)) });
            { let r_ = l_step_3; let a_ = (p_aura_1).wrapping_add(56); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::fle(F32(0x3f800000), F32(c.m.r32((p_aura_1).wrapping_add(56)))) as i32)) != 0) {
                { let v_ = F32(0x3f800000); let a_ = (p_aura_1).wrapping_add(56); c.m.w32(a_, (v_).0); };
                { let v_ = 0i32; let a_ = (p_aura_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            }
        } else {
            l_step_3 = rt::fdiv(F32(0x3f800000), { let a_ = (c.m.r32(sym::gEftAuraCfg)).wrapping_add(408); F32(c.m.r32(a_)) });
            { let r_ = l_step_3; let a_ = (p_aura_1).wrapping_add(56); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::fle(F32(c.m.r32((p_aura_1).wrapping_add(56))), F32(0x00000000)) as i32)) != 0) {
                if ((((((c.m.r32((p_aura_1).wrapping_add(40)) as i32)) == (3i32)) as i32)) != 0) {
                    { let r_ = 2i32; let a_ = p_aura_1; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
                { let v_ = F32(0x00000000); let a_ = (p_aura_1).wrapping_add(56); c.m.w32(a_, (v_).0); };
                { let v_ = 0i32; let a_ = (p_aura_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            }
        }
    }
    F::EftAura_StepFlames(c, p_aura_1, p_objId_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_UpdateState", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_BuildFlameMtx(c: &mut Ctx, mut p_out_1: u32, mut p_dir_2: u32, mut p_pos_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_BuildFlameMtx", &PROBE_N, &[(p_out_1) as u32, (p_dir_2) as u32, (p_pos_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_one_4: F32 = F32(0);
    let mut l_zero_5: F32 = F32(0);
    let mut l_d_6: F32 = F32(0);
    let mut l_lean_7: F32 = F32(0);
    let r_ = 'f: {
        l_one_4 = F32(0x3f800000);
        l_zero_5 = F32(0x00000000);
        { let v_ = { let a_ = (((c.m.r32(sym::gBtlCamView)).wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = fp.wrapping_add(64); c.m.w32(a_, (v_).0); };
        { let v_ = { let a_ = (((c.m.r32(sym::gBtlCamView)).wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = (fp.wrapping_add(64)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a_ = (((c.m.r32(sym::gBtlCamView)).wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = (fp.wrapping_add(64)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = l_one_4; let a_ = (fp.wrapping_add(64)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        l_d_6 = F::Vec3_Dot(c, fp.wrapping_add(64), p_dir_2);
        if (((rt::flt(l_d_6, l_zero_5) as i32)) != 0) {
            l_lean_7 = rt::fmul(rt::fneg(l_d_6), { let a_ = (c.m.r32(sym::gEftAuraPrm)).wrapping_add(20); F32(c.m.r32(a_)) });
            F::Vec3_Scale(c, fp, fp.wrapping_add(64), F32(0xbf800000));
            { let v_ = l_one_4; let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        } else {
            l_lean_7 = rt::fmul(l_d_6, { let a_ = (c.m.r32(sym::gEftAuraPrm)).wrapping_add(20); F32(c.m.r32(a_)) });
            F::Vec3_Copy(c, fp, fp.wrapping_add(64));
            { let v_ = l_one_4; let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        }
        F::Vec4_Sub(c, fp, p_dir_2, fp);
        F::Vec3_Normalize(c, fp, fp);
        F::Vec3_Scale(c, fp, fp, l_lean_7);
        { let v_ = rt::fadd(F32(c.m.r32(p_dir_2)), F32(c.m.r32(fp))); let a_ = fp.wrapping_add(48); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((p_dir_2).wrapping_add(4))), F32(c.m.r32((fp).wrapping_add(4)))); let a_ = (fp.wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((p_dir_2).wrapping_add(8))), F32(c.m.r32((fp).wrapping_add(8)))); let a_ = (fp.wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec3_Normalize(c, fp.wrapping_add(48), fp.wrapping_add(48));
        F::Vec3_Cross(c, fp.wrapping_add(16), fp.wrapping_add(64), fp.wrapping_add(48));
        F::Vec3_Normalize(c, fp.wrapping_add(16), fp.wrapping_add(16));
        F::Vec3_Cross(c, fp.wrapping_add(32), fp.wrapping_add(16), fp.wrapping_add(48));
        F::Vec3_Normalize(c, fp.wrapping_add(32), fp.wrapping_add(32));
        F::Mtx_StoreIdentity(c, p_out_1);
        { let v_ = F32(c.m.r32(fp.wrapping_add(16))); let a_ = ((p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(fp.wrapping_add(32))); let a_ = ((p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(fp.wrapping_add(48))); let a_ = ((p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); let a_ = ((p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4))); let a_ = ((p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))); let a_ = ((p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); let a_ = ((p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8))); let a_ = ((p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(8))); let a_ = ((p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(p_pos_3)); let a_ = ((p_out_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((p_pos_3).wrapping_add(4))); let a_ = ((p_out_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((p_pos_3).wrapping_add(8))); let a_ = ((p_out_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftAura_BuildFlameMtx", &PROBE_N, &[], true); }
    r_
}
