#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn eft_c__EftTask_IsDead(c: &mut Ctx, mut p_task_1: u32) -> u8 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_c__EftTask_IsDead", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((((((c.m.r8(p_task_1)) as i32)) & (1i32))) as u8);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "eft_c__EftTask_IsDead", &PROBE_N, &[], true); }
    r_
}

pub fn EftGeyser_GetSmokePos(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGeyser_GetSmokePos", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_fx_2: i32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = c.m.r32((p_task_1).wrapping_add(40)); c.m.r32(a_) }) != (fid::EftGeyser_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((F::eft_c__EftTask_IsDead(c, p_task_1)) != 0) {
        break 'f 0i32;
    }
    l_fx_2 = { let a_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(236); (c.m.r32(a_) as i32) };
    if (((((l_fx_2) == (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f ((((F::EftSmoke_GetPos(c, ((((l_fx_2) as i64 as u64)) as u32))) as u64)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGeyser_GetSmokePos", &PROBE_N, &[], true); }
    r_
}

pub fn EftGeyser_GetSteamWork(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGeyser_GetSteamWork", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_fx_2: i32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = c.m.r32((p_task_1).wrapping_add(40)); c.m.r32(a_) }) != (fid::EftGeyser_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((F::eft_c__EftTask_IsDead(c, p_task_1)) != 0) {
        break 'f 0i32;
    }
    l_fx_2 = { let a_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(240); (c.m.r32(a_) as i32) };
    if (((((l_fx_2) == (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f ((((F::EftSteam_GetWork(c, ((((l_fx_2) as i64 as u64)) as u32))) as u64)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGeyser_GetSteamWork", &PROBE_N, &[], true); }
    r_
}

pub fn EftGeyser_StartSmoke(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGeyser_StartSmoke", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(160) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 160);
    let mut l_w_2: u32 = 0;
    let mut l_rec_3: u32 = 0;
    let mut l_r_4: F32 = F32(0);
    let mut l_pos_5: u32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_r_4 = F32(0x00000000);
        l_pos_5 = (l_w_2).wrapping_add(112);
        { let v_ = F32(c.m.r32(l_pos_5)); let a_ = fp.wrapping_add(80); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((l_pos_5).wrapping_add(4))); let a_ = (fp.wrapping_add(80)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((l_pos_5).wrapping_add(8))); let a_ = (fp.wrapping_add(80)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(80)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x42000000); let a_ = (fp.wrapping_add(80)).wrapping_add(16); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x42000000); let a_ = ((fp.wrapping_add(80)).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x42000000); let a_ = ((fp.wrapping_add(80)).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = l_r_4; let a_ = ((fp.wrapping_add(80)).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x43400000); let a_ = (fp.wrapping_add(80)).wrapping_add(32); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x43400000); let a_ = ((fp.wrapping_add(80)).wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x43400000); let a_ = ((fp.wrapping_add(80)).wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = l_r_4; let a_ = ((fp.wrapping_add(80)).wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x40000000); let a_ = (fp.wrapping_add(80)).wrapping_add(48); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x42800000); let a_ = (fp.wrapping_add(80)).wrapping_add(52); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(80)).wrapping_add(56); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f7c28f5); let a_ = (fp.wrapping_add(80)).wrapping_add(60); c.m.w32(a_, (v_).0); };
        { let v_ = 90i32; let a_ = (fp.wrapping_add(80)).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
        { let v_ = 30i32; let a_ = (fp.wrapping_add(80)).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
        { let v_ = 3i32; let a_ = (fp.wrapping_add(80)).wrapping_add(72); c.m.w32(a_, (v_) as u32); };
        { let s_ = fp.wrapping_add(80); let d_ = fp; rt::memcpy(c, d_, s_, 80); };
        l_rec_3 = F::BtlStage_GetFxResB2(c);
        if (((((l_rec_3) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = rt::i2f((c.m.r8((l_rec_3).wrapping_add(((28i32) as u32)))) as i32); let a_ = (fp).wrapping_add(16); c.m.w32(a_, (v_).0); };
            { let v_ = rt::i2f((c.m.r8((l_rec_3).wrapping_add(((29i32) as u32)))) as i32); let a_ = ((fp).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = rt::i2f((c.m.r8((l_rec_3).wrapping_add(((30i32) as u32)))) as i32); let a_ = ((fp).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = rt::i2f((c.m.r8((l_rec_3).wrapping_add(((31i32) as u32)))) as i32); let a_ = ((fp).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            { let v_ = rt::i2f((c.m.r8((l_rec_3).wrapping_add(((32i32) as u32)))) as i32); let a_ = (fp).wrapping_add(32); c.m.w32(a_, (v_).0); };
            { let v_ = rt::i2f((c.m.r8((l_rec_3).wrapping_add(((33i32) as u32)))) as i32); let a_ = ((fp).wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = rt::i2f((c.m.r8((l_rec_3).wrapping_add(((34i32) as u32)))) as i32); let a_ = ((fp).wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = rt::i2f((c.m.r8((l_rec_3).wrapping_add(((35i32) as u32)))) as i32); let a_ = ((fp).wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            l_r_4 = rt::fsub(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40a00000)), F32(0x40200000));
            { let v_ = (rt::rem_i32(F::Port_Rand(c), 2i32)).wrapping_add(2i32); let a_ = (fp).wrapping_add(72); c.m.w32(a_, (v_) as u32); };
            { let v_ = rt::fadd(rt::fdiv(F32(c.m.r32((l_w_2).wrapping_add(180))), F32(0x43160000)), l_r_4); let a_ = (fp).wrapping_add(56); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul(rt::fdiv(F32(c.m.r32((l_w_2).wrapping_add(176))), F32(0x41200000)), F32(0x40000000)); let a_ = (fp).wrapping_add(48); c.m.w32(a_, (v_).0); };
            { let v_ = rt::f2i(rt::fadd(rt::fdiv(F32(c.m.r32((l_w_2).wrapping_add(180))), F32(0x43160000)), F32(0x42b40000))); let a_ = (fp).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
            if (((rt::flt(F32(c.m.r32((fp).wrapping_add(56))), F32(0x00000000)) as i32)) != 0) {
                { let v_ = rt::fneg(F32(c.m.r32((fp).wrapping_add(56)))); let a_ = (fp).wrapping_add(56); c.m.w32(a_, (v_).0); };
            }
            if ((((((c.m.r32((l_w_2).wrapping_add(236)) as i32)) == (0i32)) as i32)) != 0) {
                { let v_ = ((((F::EftSmoke_Create(c, fp)) as u64)) as i32); let a_ = (l_w_2).wrapping_add(236); c.m.w32(a_, (v_) as u32); };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGeyser_StartSmoke", &PROBE_N, &[], true); }
    r_
}

pub fn EftGeyser_StartSteam(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGeyser_StartSteam", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(192) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 192);
    let mut l_w_2: u32 = 0;
    let mut l_rec_3: u32 = 0;
    let mut l_pos_4: u32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_pos_4 = (l_w_2).wrapping_add(112);
        { let v_ = F32(c.m.r32(l_pos_4)); let a_ = fp.wrapping_add(96); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((l_pos_4).wrapping_add(4))); let a_ = (fp.wrapping_add(96)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((l_pos_4).wrapping_add(8))); let a_ = (fp.wrapping_add(96)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(96)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let _ = F::memset(c, (fp.wrapping_add(96)).wrapping_add(16), 0i32, 16u32); };
        { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(96)).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x43000000); let a_ = (fp.wrapping_add(96)).wrapping_add(32); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x43000000); let a_ = ((fp.wrapping_add(96)).wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x43000000); let a_ = ((fp.wrapping_add(96)).wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x43000000); let a_ = ((fp.wrapping_add(96)).wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x41000000); let a_ = (fp.wrapping_add(96)).wrapping_add(48); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3ea740d9); let a_ = (fp.wrapping_add(96)).wrapping_add(52); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x40999999); let a_ = (fp.wrapping_add(96)).wrapping_add(56); c.m.w32(a_, (v_).0); };
        { let v_ = 25i32; let a_ = (fp.wrapping_add(96)).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
        { let v_ = F32(0x41700000); let a_ = (fp.wrapping_add(96)).wrapping_add(64); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f000000); let a_ = (fp.wrapping_add(96)).wrapping_add(68); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x40000000); let a_ = (fp.wrapping_add(96)).wrapping_add(72); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f000000); let a_ = (fp.wrapping_add(96)).wrapping_add(76); c.m.w32(a_, (v_).0); };
        { let v_ = 5i32; let a_ = (fp.wrapping_add(96)).wrapping_add(80); c.m.w32(a_, (v_) as u32); };
        { let s_ = fp.wrapping_add(96); let d_ = fp; rt::memcpy(c, d_, s_, 96); };
        l_rec_3 = F::BtlStage_GetFxResC2(c);
        if (((((l_rec_3) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = rt::i2f((c.m.r8((l_rec_3).wrapping_add(((26i32) as u32)))) as i32); let a_ = (fp).wrapping_add(32); c.m.w32(a_, (v_).0); };
            { let v_ = rt::i2f((c.m.r8((l_rec_3).wrapping_add(((27i32) as u32)))) as i32); let a_ = ((fp).wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = rt::i2f((c.m.r8((l_rec_3).wrapping_add(((28i32) as u32)))) as i32); let a_ = ((fp).wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = rt::i2f((c.m.r8((l_rec_3).wrapping_add(((29i32) as u32)))) as i32); let a_ = ((fp).wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fadd(rt::fdiv(F32(c.m.r32((l_w_2).wrapping_add(180))), F32(0x43160000)), F32(0x41000000)); let a_ = (fp).wrapping_add(48); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul(rt::fdiv(F32(c.m.r32((l_w_2).wrapping_add(176))), F32(0x41200000)), F32(0x40999999)); let a_ = (fp).wrapping_add(56); c.m.w32(a_, (v_).0); };
            { let v_ = rt::f2i(rt::fadd(rt::fdiv(F32(c.m.r32((l_w_2).wrapping_add(180))), F32(0x43160000)), F32(0x41c80000))); let a_ = (fp).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
            if ((((((c.m.r32((l_w_2).wrapping_add(240)) as i32)) == (0i32)) as i32)) != 0) {
                { let v_ = ((((F::EftSteam_Create(c, fp)) as u64)) as i32); let a_ = (l_w_2).wrapping_add(240); c.m.w32(a_, (v_) as u32); };
                { let a0_ = (c.m.r32((l_w_2).wrapping_add(240)) as i32); F::EftSteam_SetPaused(c, ((((a0_) as i64 as u64)) as u32), 1i32) };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGeyser_StartSteam", &PROBE_N, &[], true); }
    r_
}

pub fn EftWeather_ClipCode(c: &mut Ctx, mut p_v_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_ClipCode", &PROBE_N, &[(p_v_1) as u32], false); }
    let mut l_code_2: i32 = 0;
    let r_ = 'f: {
    l_code_2 = 0i32;
    if (((rt::fgt(F32(c.m.r32(p_v_1)), F32(c.m.r32((p_v_1).wrapping_add(12)))) as i32)) != 0) {
        l_code_2 = 1i32;
    }
    if (((rt::flt(F32(c.m.r32(p_v_1)), rt::fneg(F32(c.m.r32((p_v_1).wrapping_add(12))))) as i32)) != 0) {
        { let r_ = 2i32; let n_ = ((l_code_2) | (r_)); l_code_2 = n_; };
    }
    if (((rt::fgt(F32(c.m.r32((p_v_1).wrapping_add(4))), F32(c.m.r32((p_v_1).wrapping_add(12)))) as i32)) != 0) {
        { let r_ = 4i32; let n_ = ((l_code_2) | (r_)); l_code_2 = n_; };
    }
    if (((rt::flt(F32(c.m.r32((p_v_1).wrapping_add(4))), rt::fneg(F32(c.m.r32((p_v_1).wrapping_add(12))))) as i32)) != 0) {
        { let r_ = 8i32; let n_ = ((l_code_2) | (r_)); l_code_2 = n_; };
    }
    if (((rt::fgt(F32(c.m.r32((p_v_1).wrapping_add(8))), F32(c.m.r32((p_v_1).wrapping_add(12)))) as i32)) != 0) {
        { let r_ = 16i32; let n_ = ((l_code_2) | (r_)); l_code_2 = n_; };
    }
    if (((rt::flt(F32(c.m.r32((p_v_1).wrapping_add(8))), rt::fneg(F32(c.m.r32((p_v_1).wrapping_add(12))))) as i32)) != 0) {
        { let r_ = 32i32; let n_ = ((l_code_2) | (r_)); l_code_2 = n_; };
    }
    break 'f l_code_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_ClipCode", &PROBE_N, &[], true); }
    r_
}

pub fn EftWeather_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_pack_2: u32 = 0;
    let mut l_rec_3: u32 = 0;
    let r_ = 'f: {
    l_pack_2 = ((F::BtlScene_GetStageData(c)) as u32);
    l_rec_3 = F::BtlStage_GetList60(c);
    if (((((((((l_rec_3) != (((0i32) as u32))) as i32)) != 0) && (((((c.m.r32((l_rec_3).wrapping_add(16)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((704u32) as i32)) }; let a_ = sym::gEftWeather; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gEftWeather); F::memset(c, a0_, 0i32, 704u32) }; };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gEftWeather)).wrapping_add(672); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = ((c.m.r32(sym::gEftWeather)).wrapping_add(528)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = ((c.m.r32(sym::gEftWeather)).wrapping_add(528)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = ((c.m.r32(sym::gEftWeather)).wrapping_add(528)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = ((c.m.r32(sym::gEftWeather)).wrapping_add(528)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = F32(c.m.r32((l_rec_3).wrapping_add(12))); let a_ = (c.m.r32(sym::gEftWeather)).wrapping_add(684); c.m.w32(a_, (v_).0); };
        { let v_ = ((c.m.r16((l_rec_3).wrapping_add(22))) as i32); let a_ = (c.m.r32(sym::gEftWeather)).wrapping_add(676); c.m.w32(a_, (v_) as u32); };
        { let v_ = F32(c.m.r32((l_rec_3).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (c.m.r32(sym::gEftWeather)).wrapping_add(544); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((l_rec_3).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((c.m.r32(sym::gEftWeather)).wrapping_add(544)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((l_rec_3).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = ((c.m.r32(sym::gEftWeather)).wrapping_add(544)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = ((c.m.r16((l_rec_3).wrapping_add(20))) as i32); let a_ = (c.m.r32(sym::gEftWeather)).wrapping_add(680); c.m.w32(a_, (v_) as u32); };
        { let a0_ = (c.m.r32(sym::gEftWeather)).wrapping_add(8); let a1_ = F::BtlScene_GetPackEntry(c, l_pack_2, 11i32); F::EftTexSet_Load32(c, a0_, a1_) };
        { let v_ = F::BtlTask_CreateChildList(c, p_task_1, ((((1i32) as i64 as u64)) as u32), (((((((((30i32) as u32)).wrapping_mul(64u32)) as i32)) as i64 as u64)) as u32)); let a_ = c.m.r32(sym::gEftWeather); c.m.w32(a_, v_); };
        { let v_ = 1i32; let a_ = (c.m.r32(sym::gEftWeather)).wrapping_add(688); c.m.w32(a_, (v_) as u32); };
        { let _ = { let a0_ = { let a_ = c.m.r32(sym::gEftWeather); c.m.r32(a_) }; F::BtlTaskList_AddTail(c, a0_, sym::gEftWeatherPtclClass, ((0i32) as u32)) }; };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftWeather_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftWeather)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftWeather); F::BtlPool_Free(c, a0_, a1_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gEftWeather; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftWeather_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_Update", &PROBE_N, &[], false); }
    let mut l_angle_1: F32 = F32(0);
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftWeather)) != (((0i32) as u32))) as i32)) != 0) {
        l_angle_1 = rt::fdiv(rt::fmul(rt::i2f({ let a_ = (c.m.r32(sym::gEftWeather)).wrapping_add(672); (c.m.r32(a_) as i32) }), F32(0x40490fda)), F32(0x43340000));
        { let a0_ = (c.m.r32(sym::gEftWeather)).wrapping_add(560); let a1_ = (c.m.r32(sym::gEftWeather)).wrapping_add(544); F::Vec4_Copy(c, a0_, a1_) };
        { let r_ = rt::fmul(F::Port_sinf(c, l_angle_1), F32(0x3e999999)); let a_ = (c.m.r32(sym::gEftWeather)).wrapping_add(560); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = rt::fmul(F::Port_sinf(c, l_angle_1), F32(0x3e999999)); let a_ = ((c.m.r32(sym::gEftWeather)).wrapping_add(560)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let v_ = rt::rem_i32(({ let a_ = (c.m.r32(sym::gEftWeather)).wrapping_add(672); (c.m.r32(a_) as i32) }).wrapping_add(((F::Port_Rand(c)) & (3i32))), 360i32); let a_ = (c.m.r32(sym::gEftWeather)).wrapping_add(672); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftWeatherPtcl_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWeatherPtcl_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::EftWeatherPtcl_Reset(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWeatherPtcl_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftWeatherPtcl_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWeatherPtcl_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWeatherPtcl_Term", &PROBE_N, &[], true); }
    r_
}

pub fn eft_c__EftWeather_TrackCam(c: &mut Ctx, mut p_cam_1: u32, mut p_view_2: u32, mut p_second_3: i32, mut p_moving_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_c__EftWeather_TrackCam", &PROBE_N, &[(p_cam_1) as u32, (p_view_2) as u32, (p_second_3) as u32, (p_moving_4) as u32], false); }
    let mut l_dist_5: F32 = F32(0);
    let r_ = 'f: {
    l_dist_5 = F::Vec3_Dist(c, p_cam_1, (p_view_2).wrapping_add((((p_second_3) as u32)).wrapping_mul(16)));
    if (((rt::fgt(l_dist_5, F32(0x41700000)) as i32)) != 0) {
        { let v_ = 2i32; let a_ = p_moving_4; c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = ((p_view_2).wrapping_add(32)).wrapping_add((((p_second_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    } else {
        if (((rt::fgt(l_dist_5, F32(0x40800000)) as i32)) != 0) {
            { let v_ = 1i32; let a_ = p_moving_4; c.m.w32(a_, (v_) as u32); };
            if ((((((c.m.r32(((p_view_2).wrapping_add(32)).wrapping_add((((p_second_3) as u32)).wrapping_mul(4))) as i32)) < (15i32)) as i32)) != 0) {
                { let a_ = ((p_view_2).wrapping_add(32)).wrapping_add((((p_second_3) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
        } else {
            if ((((((c.m.r32(((p_view_2).wrapping_add(32)).wrapping_add((((p_second_3) as u32)).wrapping_mul(4))) as i32)) != (0i32)) as i32)) != 0) {
                { let a_ = ((p_view_2).wrapping_add(32)).wrapping_add((((p_second_3) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 1i32; let a_ = p_moving_4; c.m.w32(a_, (v_) as u32); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "eft_c__EftWeather_TrackCam", &PROBE_N, &[], true); }
    r_
}

pub fn EftWeather_Move(c: &mut Ctx, mut p_p_1: u32, mut p_view_2: u32, mut p_count_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_Move", &PROBE_N, &[(p_p_1) as u32, (p_view_2) as u32, (p_count_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(208) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 208);
    let mut l_second_4: i32 = 0;
    let mut l_flags_5: i32 = 0;
    let mut l_size_6: F32 = F32(0);
    let mut l_a_7: F32 = F32(0);
    let mut l_y_8: F32 = F32(0);
    let mut l_code_9: i32 = 0;
    let mut l_n_10: i32 = 0;
    let mut l_i_11: i32 = 0;
    let mut l_q_12: u32 = 0;
    let mut l_wrap_13: F32 = F32(0);
    let mut l_r_14: i32 = 0;
    let mut l_g_15: i32 = 0;
    let mut l_b_16: i32 = 0;
    let mut l_max_17: F32 = F32(0);
    let mut l_rec_18: u32 = 0;
    let mut l_k_19: F32 = F32(0);
    let r_ = 'f: {
        l_second_4 = F::BtlScene_IsSecondView(c);
        { let v_ = 0i32; c.m.w32(fp.wrapping_add(192), (v_) as u32); }
        l_flags_5 = { let a_ = (c.m.r32(sym::gEftWeather)).wrapping_add(680); (c.m.r32(a_) as i32) };
        l_size_6 = { let a_ = (c.m.r32(sym::gEftWeather)).wrapping_add(684); F32(c.m.r32(a_)) };
        F::Vec4_Set(c, fp, F32(0x00000000), F32(0x00000000), F32(0x425c0000), F32(0x00000000));
        F::Mtx_StoreIdentity(c, fp.wrapping_add(112));
        if (((((c.m.r32(sym::gBtlCamView)) != (((0i32) as u32))) as i32)) != 0) {
            { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(64); F::Mtx_Copy(c, fp.wrapping_add(48), a1_) };
            F::Mtx_InverseRT(c, fp.wrapping_add(112), fp.wrapping_add(48));
        }
        F::Vec4_Copy(c, fp.wrapping_add(32), (fp.wrapping_add(112)).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
        F::eft_c__EftWeather_TrackCam(c, fp.wrapping_add(32), p_view_2, l_second_4, fp.wrapping_add(192));
        F::Vec4_Copy(c, (p_view_2).wrapping_add((((l_second_4) as u32)).wrapping_mul(16)), fp.wrapping_add(32));
        F::Vec3_Sub(c, (fp.wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), fp);
        {
            l_i_11 = 0i32;
            'l1: loop {
                if !(((((l_i_11) < (p_count_3)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let r_ = rt::fmul(rt::fmul(rt::i2f((((F::Port_Rand(c)) & (511i32))).wrapping_sub(255i32)), F32(0x3b808080)), F32(0x3ca3d70a)); let a_ = (p_p_1).wrapping_add(16); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let r_ = rt::fmul(rt::fmul(rt::i2f((((F::Port_Rand(c)) & (511i32))).wrapping_sub(255i32)), F32(0x3b808080)), F32(0x3ca3d70a)); let a_ = ((p_p_1).wrapping_add(16)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let r_ = rt::fmul(rt::fmul(rt::i2f((((F::Port_Rand(c)) & (511i32))).wrapping_sub(255i32)), F32(0x3b808080)), F32(0x3ca3d70a)); let a_ = ((p_p_1).wrapping_add(16)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if (((rt::flt(F32(c.m.r32((p_p_1).wrapping_add(16))), F32(0xbecccccc)) as i32)) != 0) {
                        { let v_ = F32(0xbecccccc); let a_ = (p_p_1).wrapping_add(16); c.m.w32(a_, (v_).0); };
                    }
                    if (((rt::flt(F32(c.m.r32(((p_p_1).wrapping_add(16)).wrapping_add(4))), F32(0xbecccccc)) as i32)) != 0) {
                        { let v_ = F32(0xbecccccc); let a_ = ((p_p_1).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    }
                    if (((rt::flt(F32(c.m.r32(((p_p_1).wrapping_add(16)).wrapping_add(8))), F32(0xbecccccc)) as i32)) != 0) {
                        { let v_ = F32(0xbecccccc); let a_ = ((p_p_1).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    }
                    if (((rt::fgt(F32(c.m.r32((p_p_1).wrapping_add(16))), F32(0x3ecccccc)) as i32)) != 0) {
                        { let v_ = F32(0x3ecccccc); let a_ = (p_p_1).wrapping_add(16); c.m.w32(a_, (v_).0); };
                    }
                    if (((rt::fgt(F32(c.m.r32(((p_p_1).wrapping_add(16)).wrapping_add(4))), F32(0x3ecccccc)) as i32)) != 0) {
                        { let v_ = F32(0x3ecccccc); let a_ = ((p_p_1).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    }
                    if (((rt::fgt(F32(c.m.r32(((p_p_1).wrapping_add(16)).wrapping_add(8))), F32(0x3ecccccc)) as i32)) != 0) {
                        { let v_ = F32(0x3ecccccc); let a_ = ((p_p_1).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    }
                    F::Vec4_Copy(c, fp.wrapping_add(16), p_p_1);
                    { let a2_ = (c.m.r32(sym::gEftWeather)).wrapping_add(560); F::Vec4_Add(c, fp.wrapping_add(16), fp.wrapping_add(16), a2_) };
                    l_n_10 = 0i32;
                    F::Vec4_Add(c, fp.wrapping_add(16), fp.wrapping_add(16), (p_p_1).wrapping_add(16));
                    F::Mtx_MulVec4(c, fp.wrapping_add(16), fp.wrapping_add(48), fp.wrapping_add(16));
                    { let v_ = F32(0x42200000); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    l_q_12 = fp.wrapping_add(16);
                    l_code_9 = F::EftWeather_ClipCode(c, fp.wrapping_add(16));
                    if (((((l_code_9) != (0i32)) as i32)) != 0) {
                        l_wrap_13 = F32(0xc2a00000);
                        'l3: loop {
                            'c4: {
                                { let v_ = 3i32; let a_ = (p_p_1).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                                if ((((l_code_9) & (1i32))) != 0) {
                                    { let r_ = l_wrap_13; let a_ = fp.wrapping_add(16); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                }
                                if ((((l_code_9) & (2i32))) != 0) {
                                    { let r_ = l_wrap_13; let a_ = fp.wrapping_add(16); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                }
                                if ((((l_code_9) & (4i32))) != 0) {
                                    { let r_ = l_wrap_13; let a_ = (fp.wrapping_add(16)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                }
                                if ((((l_code_9) & (8i32))) != 0) {
                                    { let r_ = l_wrap_13; let a_ = (fp.wrapping_add(16)).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                }
                                if ((((l_code_9) & (16i32))) != 0) {
                                    { let r_ = l_wrap_13; let a_ = (fp.wrapping_add(16)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                }
                                if ((((l_code_9) & (32i32))) != 0) {
                                    { let r_ = l_wrap_13; let a_ = (fp.wrapping_add(16)).wrapping_add(8); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                }
                                l_code_9 = F::EftWeather_ClipCode(c, l_q_12);
                                if (((((l_n_10) > (64i32)) as i32)) != 0) {
                                    break 'l3;
                                }
                                { let o_ = l_n_10; let n_ = (o_).wrapping_add(1); l_n_10 = n_; };
                            }
                            if !(((((l_code_9) != (0i32)) as i32)) != 0) { break 'l3; }
                        }
                    }
                    if ((((((c.m.r32(fp.wrapping_add(192)) as i32)) == (2i32)) as i32)) != 0) {
                        { let v_ = 0i32; let a_ = (p_p_1).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                    }
                    F::Vec3_Add(c, fp.wrapping_add(16), fp.wrapping_add(16), fp);
                    { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    F::Mtx_MulVec4(c, fp.wrapping_add(16), fp.wrapping_add(112), fp.wrapping_add(16));
                    l_y_8 = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4)));
                    if ((((((((l_flags_5) & (2i32))) != 0) && (((rt::fge(l_y_8, F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                        l_y_8 = F32(0xc315ffff);
                    }
                    if (((rt::flt(l_y_8, F32(0xc3fa0000)) as i32)) != 0) {
                        l_y_8 = F32(0xc3fa0000);
                    }
                    l_y_8 = rt::fsub(F32(0x3f800000), rt::fdiv(l_y_8, F32(0xc3fa0000)));
                    l_a_7 = rt::fmul(F::Vec3_Dist(c, fp.wrapping_add(32), l_q_12), F32(0x3c14f209));
                    if (((rt::fgt(l_a_7, F32(0x3f800000)) as i32)) != 0) {
                        l_a_7 = F32(0x00000000);
                    } else {
                        if (((rt::flt(l_a_7, F32(0x3f000000)) as i32)) != 0) {
                            if (((rt::fle(l_a_7, F32(0x3dcccccc)) as i32)) != 0) {
                                l_a_7 = F32(0x00000000);
                            } else {
                                l_a_7 = rt::fmul(rt::fsub(l_a_7, F32(0x3dcccccc)), F32(0x40200000));
                            }
                        } else {
                            l_a_7 = rt::fsub(F32(0x3f800000), rt::fmul(rt::fsub(l_a_7, F32(0x3f000000)), F32(0x40000000)));
                        }
                    }
                    if ((((((c.m.r32((p_p_1).wrapping_add(48)) as i32)) != (0i32)) as i32)) != 0) {
                        { let r_ = rt::fdiv(rt::i2f((3i32).wrapping_sub((c.m.r32((p_p_1).wrapping_add(48)) as i32))), F32(0x40400000)); let n_ = rt::fmul(l_a_7, r_); l_a_7 = n_; };
                        { let a_ = (p_p_1).wrapping_add(48); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    }
                    if ((((((((((c.m.r32(fp.wrapping_add(192)) as i32)) == (1i32)) as i32)) != 0) && ((((((c.m.r32(((p_view_2).wrapping_add(32)).wrapping_add((((l_second_4) as u32)).wrapping_mul(4))) as i32)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
                        { let r_ = rt::fdiv(rt::i2f((30i32).wrapping_sub((c.m.r32(((p_view_2).wrapping_add(32)).wrapping_add((((l_second_4) as u32)).wrapping_mul(4))) as i32))), F32(0x41f00000)); let n_ = rt::fmul(l_a_7, r_); l_a_7 = n_; };
                    }
                    { let r_ = l_y_8; let n_ = rt::fmul(l_a_7, r_); l_a_7 = n_; };
                    if ((((l_flags_5) & (1i32))) != 0) {
                        if (((rt::fgt(rt::fadd(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), rt::fadd(l_size_6, F32(0x40800000))), F32(0x00000000)) as i32)) != 0) {
                            if (((rt::flt(rt::fadd(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), l_size_6), F32(0x00000000)) as i32)) != 0) {
                                { let r_ = rt::fneg(rt::fmul(rt::fadd(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), l_size_6), F32(0x3e800000))); let n_ = rt::fmul(l_a_7, r_); l_a_7 = n_; };
                            } else {
                                l_a_7 = F32(0x00000000);
                            }
                        }
                    }
                    if ((F::StgTint_IsOn(c, ((((0i32) as i64 as u64)) as u32))) != 0) {
                        l_r_14 = { let a_ = ((c.m.r32(sym::gEftWeather)).wrapping_add(528)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                        l_g_15 = { let a_ = ((c.m.r32(sym::gEftWeather)).wrapping_add(528)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                        l_b_16 = { let a_ = ((c.m.r32(sym::gEftWeather)).wrapping_add(528)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                        l_max_17 = F32(0x42c80000);
                        l_rec_18 = F::BtlStage_GetList80(c);
                        l_k_19 = F32(0x3f800000);
                        if (((((l_rec_18) != (((0i32) as u32))) as i32)) != 0) {
                            l_max_17 = rt::i2f((c.m.r8((l_rec_18).wrapping_add(((7i32) as u32)))) as i32);
                        }
                        F::StgTint_GetColor0(c, fp.wrapping_add(176));
                        if ((((((c.m.r32((fp.wrapping_add(176)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) != (0i32)) as i32)) != 0) {
                            { let r_ = (c.m.r32((fp.wrapping_add(176)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32); let n_ = (l_g_15).wrapping_sub(r_); l_g_15 = n_; };
                            { let r_ = (c.m.r32((fp.wrapping_add(176)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32); let n_ = (l_b_16).wrapping_sub(r_); l_b_16 = n_; };
                            { let r_ = (c.m.r32((fp.wrapping_add(176)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32); let n_ = (l_r_14).wrapping_sub(r_); l_r_14 = n_; };
                            if (((((l_r_14) < (0i32)) as i32)) != 0) {
                                l_r_14 = 0i32;
                            }
                            if (((((l_g_15) < (0i32)) as i32)) != 0) {
                                l_g_15 = 0i32;
                            }
                            if (((((l_b_16) < (0i32)) as i32)) != 0) {
                                l_b_16 = 0i32;
                            }
                            { let r_ = rt::fdiv(rt::i2f((c.m.r32((fp.wrapping_add(176)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)), l_max_17); let n_ = rt::fsub(l_k_19, r_); l_k_19 = n_; };
                        }
                        F::Vec4_Set(c, (p_p_1).wrapping_add(32), rt::i2f(l_r_14), rt::i2f(l_g_15), rt::i2f(l_b_16), rt::fmul(rt::fmul(l_a_7, F32(0x43000000)), l_k_19));
                    } else {
                        F::Vec4_Set(c, (p_p_1).wrapping_add(32), F32(0x43000000), F32(0x43000000), F32(0x43000000), rt::fmul(l_a_7, F32(0x43000000)));
                    }
                    F::Vec4_Copy(c, p_p_1, l_q_12);
                    { let o_ = p_p_1; let n_ = (o_).wrapping_add(64); p_p_1 = n_; };
                }
                { let o_ = l_i_11; let n_ = (o_).wrapping_add(1); l_i_11 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_Move", &PROBE_N, &[], true); }
    r_
}

pub fn EftWeatherPtcl_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWeatherPtcl_Update", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32(sym::gEftWeather)).wrapping_add(8); F::EftTexSet_Keep32(c, a0_, 1i32, 0i32) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWeatherPtcl_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftWeatherPtcl_PostUpdate(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWeatherPtcl_PostUpdate", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWeatherPtcl_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftWeatherPtcl_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWeatherPtcl_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    l_p_2 = c.m.r32((p_task_1).wrapping_add(56));
    {
        l_i_3 = (30i32).wrapping_sub(1i32);
        'l1: loop {
            if !(((((l_i_3) >= (0i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = rt::fmul(rt::i2f((((F::Port_Rand(c)) & (1023i32))).wrapping_sub(511i32)), F32(0x3dcccccc)); let a_ = l_p_2; c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul(rt::i2f((((F::Port_Rand(c)) & (1023i32))).wrapping_sub(511i32)), F32(0x3dcccccc)); let a_ = (l_p_2).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul(rt::i2f((((F::Port_Rand(c)) & (1023i32))).wrapping_sub(511i32)), F32(0x3dcccccc)); let a_ = (l_p_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = (l_p_2).wrapping_add(12); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = (l_p_2).wrapping_add(16); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = ((l_p_2).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = ((l_p_2).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = ((l_p_2).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                F::Vec4_Set(c, (l_p_2).wrapping_add(32), F32(0x43000000), F32(0x43000000), F32(0x43000000), F32(0x00000000));
                { let v_ = 0i32; let a_ = (l_p_2).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                { let v_ = rt::rem_i32(F::Port_Rand(c), { let a_ = ((c.m.r32(sym::gEftWeather)).wrapping_add(8)).wrapping_add(512); (c.m.r32(a_) as i32) }); let a_ = (l_p_2).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
                { let o_ = l_p_2; let n_ = (o_).wrapping_add(64); l_p_2 = n_; };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_sub(1); l_i_3 = n_; };
        }
    }
    { let v_ = 1i32; let a_ = (c.m.r32(sym::gEftWeather)).wrapping_add(688); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWeatherPtcl_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftWeather_DrawPtcls(c: &mut Ctx, mut p_p_1: u32, mut p_count_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_DrawPtcls", &PROBE_N, &[(p_p_1) as u32, (p_count_2) as u32], false); }
    let mut l_tex_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_tex_3 = (c.m.r32(sym::gEftWeather)).wrapping_add(8);
    F::Vu0Cur_Push(c);
    { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (p_count_2)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a2_ = rt::fmul({ let a_ = (c.m.r32(sym::gEftWeather)).wrapping_add(684); F32(c.m.r32(a_)) }, F32(0x41800000)); let a3_ = rt::fmul({ let a_ = (c.m.r32(sym::gEftWeather)).wrapping_add(684); F32(c.m.r32(a_)) }, F32(0x41800000)); let a11_ = { let a_ = (l_tex_3).wrapping_add(((((c.m.r32((p_p_1).wrapping_add(52)) as i32)) as u32)).wrapping_mul(16)); c.m.r64(a_) }; F::EftGfx_DrawSprite(c, p_p_1, (p_p_1).wrapping_add(32), a2_, a3_, F32(0x00000000), F32(0x00000000), F32(0x3f800000), F32(0x3f800000), F32(0x00000000), 0i32, 0i32, a11_) };
                { let o_ = p_p_1; let n_ = (o_).wrapping_add(64); p_p_1 = n_; };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    F::Vu0Cur_Pop(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_DrawPtcls", &PROBE_N, &[], true); }
    r_
}

pub fn EftWeatherPtcl_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWeatherPtcl_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_second_2: i32 = 0;
    let mut l_p_3: u32 = 0;
    let mut l_count_4: i32 = 0;
    let r_ = 'f: {
    l_second_2 = F::BtlScene_IsSecondView(c);
    l_p_3 = c.m.r32((p_task_1).wrapping_add(56));
    l_count_4 = { let a_ = (c.m.r32(sym::gEftWeather)).wrapping_add(676); (c.m.r32(a_) as i32) };
    if (({ let a_ = (c.m.r32(sym::gEftWeather)).wrapping_add(688); (c.m.r32(a_) as i32) }) != 0) {
        if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
            if ((F::Battle_IsSplitScreen(c)) != 0) {
                { let r_ = 2i32; let n_ = rt::div_i32(l_count_4, r_); l_count_4 = n_; };
                if ((((!((l_second_2) != 0)) as i32)) != 0) {
                    { let a1_ = (c.m.r32(sym::gEftWeather)).wrapping_add(576); F::EftWeather_Move(c, l_p_3, a1_, l_count_4) };
                } else {
                    { let r_ = rt::div_i32(30i32, 2i32); let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(64)); l_p_3 = n_; };
                    { let a1_ = (c.m.r32(sym::gEftWeather)).wrapping_add(576); F::EftWeather_Move(c, l_p_3, a1_, l_count_4) };
                }
            } else {
                { let a0_ = c.m.r32((p_task_1).wrapping_add(56)); let a1_ = (c.m.r32(sym::gEftWeather)).wrapping_add(576); F::EftWeather_Move(c, a0_, a1_, l_count_4) };
            }
        }
        if ((((!(((({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(8); (c.m.r32(a_) as i32) }) & (1i32))) != 0)) as i32)) != 0) {
            if ((F::EftStage_IsDrawOn(c)) != 0) {
                F::EftWeather_DrawPtcls(c, l_p_3, l_count_4);
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWeatherPtcl_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftWeather_SetCount(c: &mut Ctx, mut p_count_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_SetCount", &PROBE_N, &[(p_count_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftWeather)) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = p_count_1; let a_ = (c.m.r32(sym::gEftWeather)).wrapping_add(676); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_SetCount", &PROBE_N, &[], true); }
    r_
}

pub fn EftWeather_GetCount(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_GetCount", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftWeather)) != (((0i32) as u32))) as i32)) != 0) {
        break 'f { let a_ = (c.m.r32(sym::gEftWeather)).wrapping_add(676); (c.m.r32(a_) as i32) };
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_GetCount", &PROBE_N, &[], true); }
    r_
}

pub fn EftWeather_SetFlags(c: &mut Ctx, mut p_flags_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_SetFlags", &PROBE_N, &[(p_flags_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftWeather)) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = p_flags_1; let a_ = (c.m.r32(sym::gEftWeather)).wrapping_add(680); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_SetFlags", &PROBE_N, &[], true); }
    r_
}

pub fn EftWeather_GetFlags(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_GetFlags", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftWeather)) != (((0i32) as u32))) as i32)) != 0) {
        break 'f { let a_ = (c.m.r32(sym::gEftWeather)).wrapping_add(680); (c.m.r32(a_) as i32) };
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_GetFlags", &PROBE_N, &[], true); }
    r_
}

pub fn EftWeather_GetWindPtr(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_GetWindPtr", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftWeather)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    break 'f (c.m.r32(sym::gEftWeather)).wrapping_add(544);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_GetWindPtr", &PROBE_N, &[], true); }
    r_
}

pub fn EftWeather_SetSize(c: &mut Ctx, mut p_size_1: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_SetSize", &PROBE_N, &[(p_size_1).0], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftWeather)) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = p_size_1; let a_ = (c.m.r32(sym::gEftWeather)).wrapping_add(684); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_SetSize", &PROBE_N, &[], true); }
    r_
}

pub fn EftWeather_GetSize(c: &mut Ctx) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_GetSize", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftWeather)) != (((0i32) as u32))) as i32)) != 0) {
        break 'f { let a_ = (c.m.r32(sym::gEftWeather)).wrapping_add(684); F32(c.m.r32(a_)) };
    }
    break 'f F32(0x00000000);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_GetSize", &PROBE_N, &[], true); }
    r_
}

pub fn EftWeather_SetColor(c: &mut Ctx, mut p_r_1: u8, mut p_g_2: u8, mut p_b_3: u8) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_SetColor", &PROBE_N, &[(p_r_1) as u32, (p_g_2) as u32, (p_b_3) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftWeather)) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = ((p_r_1) as i32); let a_ = ((c.m.r32(sym::gEftWeather)).wrapping_add(528)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((p_g_2) as i32); let a_ = ((c.m.r32(sym::gEftWeather)).wrapping_add(528)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((p_b_3) as i32); let a_ = ((c.m.r32(sym::gEftWeather)).wrapping_add(528)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_SetColor", &PROBE_N, &[], true); }
    r_
}

pub fn EftWeather_GetColor(c: &mut Ctx, mut p_out_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_GetColor", &PROBE_N, &[(p_out_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftWeather)) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a_ = ((c.m.r32(sym::gEftWeather)).wrapping_add(528)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = ((c.m.r32(sym::gEftWeather)).wrapping_add(528)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = ((c.m.r32(sym::gEftWeather)).wrapping_add(528)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = (p_out_1).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_GetColor", &PROBE_N, &[], true); }
    r_
}

pub fn EftWeather_SetWind(c: &mut Ctx, mut p_x_1: F32, mut p_y_2: F32, mut p_z_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_SetWind", &PROBE_N, &[(p_x_1).0, (p_y_2).0, (p_z_3).0], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftWeather)) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = p_x_1; let a_ = (c.m.r32(sym::gEftWeather)).wrapping_add(544); c.m.w32(a_, (v_).0); };
        { let v_ = p_y_2; let a_ = ((c.m.r32(sym::gEftWeather)).wrapping_add(544)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_z_3; let a_ = ((c.m.r32(sym::gEftWeather)).wrapping_add(544)).wrapping_add(8); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_SetWind", &PROBE_N, &[], true); }
    r_
}

pub fn EftWeather_GetWind(c: &mut Ctx, mut p_out_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_GetWind", &PROBE_N, &[(p_out_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftWeather)) != (((0i32) as u32))) as i32)) != 0) {
        { let a1_ = { let a_ = (c.m.r32(sym::gEftWeather)).wrapping_add(544); F32(c.m.r32(a_)) }; let a2_ = { let a_ = ((c.m.r32(sym::gEftWeather)).wrapping_add(544)).wrapping_add(4); F32(c.m.r32(a_)) }; let a3_ = { let a_ = ((c.m.r32(sym::gEftWeather)).wrapping_add(544)).wrapping_add(8); F32(c.m.r32(a_)) }; F::Vec4_Set(c, p_out_1, a1_, a2_, a3_, F32(0x3f800000)) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_GetWind", &PROBE_N, &[], true); }
    r_
}

pub fn EftWeather_SetCount2(c: &mut Ctx, mut p_count_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_SetCount2", &PROBE_N, &[(p_count_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftWeather)) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = p_count_1; let a_ = (c.m.r32(sym::gEftWeather)).wrapping_add(676); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_SetCount2", &PROBE_N, &[], true); }
    r_
}

pub fn EftWeather_GetCount2(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_GetCount2", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftWeather)) != (((0i32) as u32))) as i32)) != 0) {
        break 'f { let a_ = (c.m.r32(sym::gEftWeather)).wrapping_add(676); (c.m.r32(a_) as i32) };
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_GetCount2", &PROBE_N, &[], true); }
    r_
}

pub fn EftWeather_SetFlags2(c: &mut Ctx, mut p_flags_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_SetFlags2", &PROBE_N, &[(p_flags_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftWeather)) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = p_flags_1; let a_ = (c.m.r32(sym::gEftWeather)).wrapping_add(680); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_SetFlags2", &PROBE_N, &[], true); }
    r_
}

pub fn EftWeather_GetFlags2(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_GetFlags2", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftWeather)) != (((0i32) as u32))) as i32)) != 0) {
        break 'f { let a_ = (c.m.r32(sym::gEftWeather)).wrapping_add(680); (c.m.r32(a_) as i32) };
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_GetFlags2", &PROBE_N, &[], true); }
    r_
}

pub fn EftWeather_SetEnabled(c: &mut Ctx, mut p_on_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_SetEnabled", &PROBE_N, &[(p_on_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftWeather)) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = p_on_1; let a_ = (c.m.r32(sym::gEftWeather)).wrapping_add(688); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftWeather_SetEnabled", &PROBE_N, &[], true); }
    r_
}

pub fn EftStage_Recreate(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStage_Recreate", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::BtlScene_FreeLayer0(c);
    F::BtlScene_CreateLayer0(c);
    F::EftGndDust_InitTemplate(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStage_Recreate", &PROBE_N, &[], true); }
    r_
}

pub fn EftStage_ResetAll(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStage_ResetAll", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::BtlScene_Reset(c, 0i32);
    F::EftStage_FreeKinds(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStage_ResetAll", &PROBE_N, &[], true); }
    r_
}

pub fn EftStage_IsDrawOn(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStage_IsDrawOn", &PROBE_N, &[], false); }
    let mut l_ret_1: i32 = 0;
    let r_ = 'f: {
    l_ret_1 = 0i32;
    if (((((c.m.r32(sym::gEftStage)) != (((0i32) as u32))) as i32)) != 0) {
        l_ret_1 = ((({ let a_ = (c.m.r32(sym::gEftStage)).wrapping_add(8); (c.m.r32(a_) as i32) }).wrapping_shr((1i32) as u32)) & (1i32));
    }
    break 'f l_ret_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStage_IsDrawOn", &PROBE_N, &[], true); }
    r_
}

pub fn EftStage_SetDrawOn(c: &mut Ctx, mut p_on_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStage_SetDrawOn", &PROBE_N, &[(p_on_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftStage)) != (((0i32) as u32))) as i32)) != 0) {
        if ((p_on_1) != 0) {
            { let r_ = 2i32; let a_ = (c.m.r32(sym::gEftStage)).wrapping_add(8); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        } else {
            { let r_ = (!(2i32)); let a_ = (c.m.r32(sym::gEftStage)).wrapping_add(8); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStage_SetDrawOn", &PROBE_N, &[], true); }
    r_
}

pub fn EftStage_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStage_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::BtlPool_SetCurrent(c, 2i32);
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((88u32) as i32)) }; let a_ = sym::gEftStage; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gEftStage); F::memset(c, a0_, 0i32, 88u32) }; };
    { let v_ = p_task_1; let a_ = (c.m.r32(sym::gEftStage)).wrapping_add(4); c.m.w32(a_, v_); };
    F::EftStage_CreateKinds(c);
    F::EftStage_SetDrawOn(c, 1i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStage_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftStage_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStage_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::BtlPool_SetCurrent(c, 2i32);
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftStage); F::BtlPool_Free(c, a0_, a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftStage; c.m.w32(a_, v_); };
    F::BtlPool_Reset(c, 2i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStage_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftStage_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStage_Update", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStage_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftStage_FreeKinds(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStage_FreeKinds", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = c.m.r32(sym::gEftStage); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        F::BtlPool_SetCurrent(c, 2i32);
        { let a0_ = { let a_ = (c.m.r32(sym::gEftStage)).wrapping_add(4); c.m.r32(a_) }; F::BtlTask_DestroyChildList(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = c.m.r32(sym::gEftStage); c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStage_FreeKinds", &PROBE_N, &[], true); }
    r_
}

pub fn EftStage_CreateKinds(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStage_CreateKinds", &PROBE_N, &[], false); }
    let mut l_kind_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    if ((((({ let a_ = c.m.r32(sym::gEftStage); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a0_ = { let a_ = (c.m.r32(sym::gEftStage)).wrapping_add(4); c.m.r32(a_) }; F::BtlTask_CreateChildList(c, a0_, ((((10i32) as i64 as u64)) as u32), ((((0i32) as i64 as u64)) as u32)) }; let a_ = c.m.r32(sym::gEftStage); c.m.w32(a_, v_); };
        l_kind_1 = sym::gEftStageKinds;
        {
            l_i_2 = 9i32;
            'l1: loop {
                if !(((((l_i_2) >= (0i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((c.m.r32(l_kind_1)) != (((0i32) as u32))) as i32)) != 0) {
                        if (({ let a0_ = (c.m.r32((l_kind_1).wrapping_add(4)) as i32); F::EftStage_HasKind(c, a0_) }) != 0) {
                            { let _ = { let a0_ = { let a_ = c.m.r32(sym::gEftStage); c.m.r32(a_) }; let a1_ = c.m.r32(l_kind_1); F::BtlTaskList_AddTail(c, a0_, a1_, ((0i32) as u32)) }; };
                        }
                    }
                    { let o_ = l_kind_1; let n_ = (o_).wrapping_add(8); l_kind_1 = n_; };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_sub(1); l_i_2 = n_; };
            }
        }
        { let _ = F::EftStage_LoadSprite(c); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStage_CreateKinds", &PROBE_N, &[], true); }
    r_
}

pub fn EftStage_HasKind(c: &mut Ctx, mut p_kind_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStage_HasKind", &PROBE_N, &[(p_kind_1) as u32], false); }
    let mut l_pack_2: u32 = 0;
    let mut l_ret_3: i32 = 0;
    let r_ = 'f: {
    l_pack_2 = { let a_ = (c.m.r32(sym::gCommonRes)).wrapping_add(36); c.m.r32(a_) };
    l_ret_3 = 0i32;
    if (((((p_kind_1) == ((1i32).wrapping_neg())) as i32)) != 0) {
        break 'f 1i32;
    }
    's1: {
        let sv4: i32 = p_kind_1;
        match sv4 {
            0i32 => {
                if (((((F::BtlScene_GetPackEntrySize(c, l_pack_2, 7i32)) > (0i32)) as i32)) != 0) {
                    l_ret_3 = 1i32;
                }
                break 's1;
            }
            2i32 => {
                if (((((F::BtlScene_GetPackEntrySize(c, l_pack_2, 11i32)) > (0i32)) as i32)) != 0) {
                    l_ret_3 = 1i32;
                }
                break 's1;
            }
            1i32 => {
                if (((((F::BtlScene_GetPackEntrySize(c, l_pack_2, 13i32)) > (0i32)) as i32)) != 0) {
                    l_ret_3 = 1i32;
                }
                break 's1;
            }
            3i32 => {
                if (((((F::BtlScene_GetPackEntrySize(c, l_pack_2, 14i32)) > (0i32)) as i32)) != 0) {
                    l_ret_3 = 1i32;
                }
                break 's1;
            }
            4i32 => {
                if (((((F::BtlStage_GetFxResB2(c)) != (((0i32) as u32))) as i32)) != 0) {
                    l_ret_3 = 1i32;
                }
                break 's1;
            }
            5i32 => {
                if (((((F::BtlStage_GetFxResC2(c)) != (((0i32) as u32))) as i32)) != 0) {
                    l_ret_3 = 1i32;
                }
                break 's1;
            }
            6i32 => {
                if (((((F::BtlStage_GetFxResA2(c)) != (((0i32) as u32))) as i32)) != 0) {
                    l_ret_3 = 1i32;
                }
                break 's1;
            }
            7i32 => {
                if (((((F::BtlStage_GetList90(c)) != (((0i32) as u32))) as i32)) != 0) {
                    if (((({ let a_ = F::BtlStage_GetList90(c); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
                        if (((((F::BtlScene_GetPackEntrySize(c, l_pack_2, 18i32)) > (0i32)) as i32)) != 0) {
                            l_ret_3 = 1i32;
                        }
                    }
                }
                break 's1;
            }
            8i32 => {
                if (((((F::BtlScene_GetPackEntrySize(c, l_pack_2, 21i32)) > (0i32)) as i32)) != 0) {
                    if (((((F::BtlScene_GetPackEntrySize(c, l_pack_2, 22i32)) > (0i32)) as i32)) != 0) {
                        l_ret_3 = (((F::BtlScene_GetPackEntrySize(c, l_pack_2, 23i32)) > (0i32)) as i32);
                    }
                }
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f l_ret_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStage_HasKind", &PROBE_N, &[], true); }
    r_
}

pub fn EftStage_Nop(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStage_Nop", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStage_Nop", &PROBE_N, &[], true); }
    r_
}

pub fn EftStage_GetTintScale(c: &mut Ctx) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStage_GetTintScale", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_scale_1: F32 = F32(0);
    let r_ = 'f: {
        l_scale_1 = F32(0x3f800000);
        F::StgTint_GetColor0(c, fp);
        { let r_ = rt::fdiv(rt::i2f((c.m.r32((fp).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)), F32(0x42c80000)); let n_ = rt::fsub(l_scale_1, r_); l_scale_1 = n_; };
        if (((rt::fgt(l_scale_1, F32(0x3f800000)) as i32)) != 0) {
            l_scale_1 = F32(0x3f800000);
        }
        if (((rt::flt(l_scale_1, F32(0x3f000000)) as i32)) != 0) {
            l_scale_1 = F32(0x3f000000);
        }
        break 'f l_scale_1;
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftStage_GetTintScale", &PROBE_N, &[], true); }
    r_
}

pub fn EftStage_LoadSprite(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStage_LoadSprite", &PROBE_N, &[], false); }
    let mut l_pack_1: u32 = 0;
    let r_ = 'f: {
    l_pack_1 = { let a_ = (c.m.r32(sym::gCommonRes)).wrapping_add(36); c.m.r32(a_) };
    if ((F::BtlStage_IsReady(c)) != 0) {
        if (((((F::BtlScene_GetPackEntrySize(c, l_pack_1, 20i32)) > (0i32)) as i32)) != 0) {
            { let r_ = 4i32; let a_ = (c.m.r32(sym::gEftStage)).wrapping_add(8); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        } else {
            { let r_ = (!(4i32)); let a_ = (c.m.r32(sym::gEftStage)).wrapping_add(8); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if (((({ let a_ = (c.m.r32(sym::gEftStage)).wrapping_add(8); (c.m.r32(a_) as i32) }) & (4i32))) != 0) {
            { let a0_ = (c.m.r32(sym::gEftStage)).wrapping_add(16); let a1_ = F::BtlScene_GetPackEntry(c, l_pack_1, 20i32); F::EftTexSet_Load4(c, a0_, a1_) };
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStage_LoadSprite", &PROBE_N, &[], true); }
    r_
}

pub fn EftStage_UpdateSprite(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStage_UpdateSprite", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((((F::BtlStage_IsReady(c)) != 0) && (((({ let a_ = (c.m.r32(sym::gEftStage)).wrapping_add(8); (c.m.r32(a_) as i32) }) & (4i32))) != 0)) as i32)) != 0) {
        { let a0_ = (c.m.r32(sym::gEftStage)).wrapping_add(16); F::EftTexSet_Keep4(c, a0_, 1i32, 0i32) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStage_UpdateSprite", &PROBE_N, &[], true); }
    r_
}

pub fn EftStage_DrawSprite(c: &mut Ctx, mut p_pos_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStage_DrawSprite", &PROBE_N, &[(p_pos_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_size_2: F32 = F32(0);
    let r_ = 'f: {
        if ((((((F::BtlStage_IsReady(c)) != 0) && (((({ let a_ = (c.m.r32(sym::gEftStage)).wrapping_add(8); (c.m.r32(a_) as i32) }) & (4i32))) != 0)) as i32)) != 0) {
            rt::memset(c, fp, 0, 16);
            { let v_ = F32(0x43000000); c.m.w32(fp.wrapping_add(0), (v_).0); }
            { let v_ = F32(0x43000000); c.m.w32(fp.wrapping_add(4), (v_).0); }
            { let v_ = F32(0x43000000); c.m.w32(fp.wrapping_add(8), (v_).0); }
            { let v_ = F32(0x43000000); c.m.w32(fp.wrapping_add(12), (v_).0); }
            l_size_2 = F32(0x41800000);
            F::Vu0Cur_Push(c);
            { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
            { let a4_ = { let a_ = (c.m.r32(sym::gEftStage)).wrapping_add(16); c.m.r64(a_) }; F::EftPrim_DrawQuadDepth(c, p_pos_1, fp, 0i32, 0i32, a4_, l_size_2, l_size_2, F32(0x00000000), F32(0x00000000), F32(0x3f800000), F32(0x3f800000), F32(0x00000000)) };
            F::Vu0Cur_Pop(c);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftStage_DrawSprite", &PROBE_N, &[], true); }
    r_
}

pub fn EftSurf_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_Init", &PROBE_N, &[], false); }
    let mut l_pack_1: u32 = 0;
    let mut l_param_2: u32 = 0;
    let mut l_size_3: i32 = 0;
    let mut l_rt_4: u32 = 0;
    let mut l_file_5: u32 = 0;
    let mut l_mesh_6: u32 = 0;
    let mut l_rec_7: u32 = 0;
    let mut l_count_8: i32 = 0;
    let mut l_i_9: i32 = 0;
    let r_ = 'f: {
    l_pack_1 = { let a_ = (c.m.r32(sym::gCommonRes)).wrapping_add(36); c.m.r32(a_) };
    l_param_2 = (l_pack_1).wrapping_add((((((c.m.r32((l_pack_1).wrapping_add((((13i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
    l_size_3 = ((c.m.r32((l_pack_1).wrapping_add((((14i32) as u32)).wrapping_mul(4))) as i32)).wrapping_sub((c.m.r32((l_pack_1).wrapping_add((((13i32) as u32)).wrapping_mul(4))) as i32));
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((4992u32) as i32)) }; let a_ = sym::gEftSurf; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gEftSurf); F::memset(c, a0_, 0i32, 4992u32) }; };
    { let v_ = l_param_2; let a_ = c.m.r32(sym::gEftSurf); c.m.w32(a_, v_); };
    { let v_ = (l_param_2).wrapping_add((((1i32) as u32)).wrapping_mul(64)); let a_ = (c.m.r32(sym::gEftSurf)).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = l_size_3; let a_ = (c.m.r32(sym::gEftSurf)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    l_rt_4 = (c.m.r32(sym::gEftSurf)).wrapping_add(576);
    { let a0_ = { let a_ = (c.m.r32(sym::gEftSurf)).wrapping_add(4); c.m.r32(a_) }; F::EftSurf_Relocate(c, a0_) };
    l_file_5 = { let a_ = (c.m.r32(sym::gEftSurf)).wrapping_add(4); c.m.r32(a_) };
    l_count_8 = (c.m.r32((l_file_5).wrapping_add(24)) as i32);
    l_mesh_6 = c.m.r32((l_file_5).wrapping_add(28));
    {
        l_i_9 = 0i32;
        'l1: loop {
            if !(((((l_i_9) < (l_count_8)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((c.m.r32((l_mesh_6).wrapping_add(32)) as i32)) & (2048i32))) != 0) {
                    if ((((!(((((c.m.r32((l_rt_4).wrapping_add(4372)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
                        { let a1_ = (l_pack_1).wrapping_add((((((c.m.r32((l_pack_1).wrapping_add((((10i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::EftTexSet_Load34(c, (l_rt_4).wrapping_add(3664), a1_) };
                        F::GfxClut_InitPacket(c, (l_rt_4).wrapping_add((((0i32) as u32)).wrapping_mul(1216)), ((((16000i32) as i64 as u64)) as u16));
                        F::GfxClut_InitPacket(c, (l_rt_4).wrapping_add((((1i32) as u32)).wrapping_mul(1216)), ((((16004i32) as i64 as u64)) as u16));
                        F::GfxClut_InitPacket(c, (l_rt_4).wrapping_add((((2i32) as u32)).wrapping_mul(1216)), ((((16008i32) as i64 as u64)) as u16));
                        { let v_ = ((c.m.r32(((l_rt_4).wrapping_add(3664)).wrapping_add(544)) as i32)).wrapping_sub(4i32); let a_ = (l_rt_4).wrapping_add(4356); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r32(((l_rt_4).wrapping_add(3664)).wrapping_add(544)) as i32)).wrapping_sub(4i32); let a_ = (l_rt_4).wrapping_add(4360); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r32(((l_rt_4).wrapping_add(3664)).wrapping_add(544)) as i32)).wrapping_sub(2i32); let a_ = (l_rt_4).wrapping_add(4364); c.m.w32(a_, (v_) as u32); };
                        { let s_ = { let a_ = c.m.r32(sym::gEftSurf); c.m.r32(a_) }; let d_ = (l_rt_4).wrapping_add(4224); rt::memcpy(c, d_, s_, 64); };
                        l_rec_7 = F::BtlStage_GetList58(c);
                        if (((((l_rec_7) != (((0i32) as u32))) as i32)) != 0) {
                            { let v_ = rt::i2f((c.m.r8(((l_rec_7).wrapping_add(23)).wrapping_add(((0i32) as u32)))) as i32); let a_ = ((l_rt_4).wrapping_add(4224)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                            { let v_ = rt::i2f((c.m.r8(((l_rec_7).wrapping_add(23)).wrapping_add(((1i32) as u32)))) as i32); let a_ = ((l_rt_4).wrapping_add(4224)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                            { let v_ = rt::i2f((c.m.r8(((l_rec_7).wrapping_add(23)).wrapping_add(((2i32) as u32)))) as i32); let a_ = ((l_rt_4).wrapping_add(4224)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                            { let v_ = rt::i2f((c.m.r8(((l_rec_7).wrapping_add(23)).wrapping_add(((3i32) as u32)))) as i32); let a_ = ((l_rt_4).wrapping_add(4224)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                            { let v_ = rt::i2f((c.m.r8(((l_rec_7).wrapping_add(27)).wrapping_add(((0i32) as u32)))) as i32); let a_ = (((l_rt_4).wrapping_add(4224)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                            { let v_ = rt::i2f((c.m.r8(((l_rec_7).wrapping_add(27)).wrapping_add(((1i32) as u32)))) as i32); let a_ = (((l_rt_4).wrapping_add(4224)).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                            { let v_ = rt::i2f((c.m.r8(((l_rec_7).wrapping_add(27)).wrapping_add(((2i32) as u32)))) as i32); let a_ = (((l_rt_4).wrapping_add(4224)).wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                            { let v_ = rt::i2f((c.m.r8(((l_rec_7).wrapping_add(27)).wrapping_add(((3i32) as u32)))) as i32); let a_ = (((l_rt_4).wrapping_add(4224)).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(c.m.r32((l_rec_7).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((l_rt_4).wrapping_add(4224)).wrapping_add(32); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(c.m.r32((l_rec_7).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (((l_rt_4).wrapping_add(4224)).wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(c.m.r32((l_rec_7).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (((l_rt_4).wrapping_add(4224)).wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(0x3f800000); let a_ = (((l_rt_4).wrapping_add(4224)).wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(c.m.r32((l_rec_7).wrapping_add(12))); let a_ = ((l_rt_4).wrapping_add(4224)).wrapping_add(48); c.m.w32(a_, (v_).0); };
                            { let v_ = ((c.m.r16((l_rec_7).wrapping_add(20))) as i32); let a_ = ((l_rt_4).wrapping_add(4224)).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
                            { let v_ = F32(c.m.r32((l_rec_7).wrapping_add(16))); let a_ = ((l_rt_4).wrapping_add(4224)).wrapping_add(56); c.m.w32(a_, (v_).0); };
                        }
                        if (((rt::feq(F32(c.m.r32(((l_rt_4).wrapping_add(4224)).wrapping_add(56))), F32(0x00000000)) as i32)) != 0) {
                            { let v_ = F32(0x447a0000); let a_ = ((l_rt_4).wrapping_add(4224)).wrapping_add(56); c.m.w32(a_, (v_).0); };
                        }
                        { let v_ = (l_rt_4).wrapping_add(3664); let a_ = (l_rt_4).wrapping_add(4216); c.m.w32(a_, v_); };
                        { let r_ = ((1i32) | (8i32)); let a_ = (l_rt_4).wrapping_add(4372); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                }
            }
            { { let o_ = l_i_9; let n_ = (o_).wrapping_add(1); l_i_9 = n_; }; { let o_ = l_mesh_6; let n_ = (o_).wrapping_add(96); l_mesh_6 = n_; }; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftSurf_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = { let a_ = (c.m.r32(sym::gEftSurf)).wrapping_add(4); c.m.r32(a_) }; F::EftSurf_Unrelocate(c, a0_) };
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftSurf); F::BtlPool_Free(c, a0_, a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftSurf; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftSurf_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_Update", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((F::BtlStage_IsReady(c)) != 0) {
        if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
            { let a0_ = { let a_ = (c.m.r32(sym::gEftSurf)).wrapping_add(4); c.m.r32(a_) }; F::EftSurf_Animate(c, a0_) };
        }
        { let a0_ = { let a_ = (c.m.r32(sym::gEftSurf)).wrapping_add(4); c.m.r32(a_) }; F::EftSurf_UpdateTextures(c, a0_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftSurf_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_Draw", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((((((((((!(((({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(8); (c.m.r32(a_) as i32) }) & (1i32))) != 0)) as i32)) != 0) && ((F::BtlStage_IsReady(c)) != 0)) as i32)) != 0) && ((F::EftStage_IsDrawOn(c)) != 0)) as i32)) != 0) {
        F::EftSurf_DrawMeshes(c);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftSurf_PostUpdate(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_PostUpdate", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftSurf_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_Reset", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftSurf_Relocate(c: &mut Ctx, mut p_file_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_Relocate", &PROBE_N, &[(p_file_1) as u32], false); }
    let mut l_tex_2: u32 = 0;
    let mut l_mesh_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    { let r_ = ((p_file_1) as i32); let a_ = (p_file_1).wrapping_add(12); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
    { let v_ = (((((c.m.r32((p_file_1).wrapping_add(20))) as i32)).wrapping_add(((p_file_1) as i32))) as u32); let a_ = (p_file_1).wrapping_add(20); c.m.w32(a_, v_); };
    { let v_ = (((((c.m.r32((p_file_1).wrapping_add(28))) as i32)).wrapping_add(((p_file_1) as i32))) as u32); let a_ = (p_file_1).wrapping_add(28); c.m.w32(a_, v_); };
    if ((((((c.m.r32((p_file_1).wrapping_add(4)) as i32)) != (4i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (p_file_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (p_file_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
        break 'f;
    }
    l_tex_2 = c.m.r32((p_file_1).wrapping_add(20));
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < ((c.m.r32((p_file_1).wrapping_add(16)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = ((c.m.r32(l_tex_2) as i32)).wrapping_add(((p_file_1) as i32)); let a_ = (l_tex_2).wrapping_add(56); c.m.w32(a_, (v_) as u32); };
                { let v_ = ((c.m.r32((l_tex_2).wrapping_add(4)) as i32)).wrapping_add(((p_file_1) as i32)); let a_ = (l_tex_2).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
                { let o_ = l_tex_2; let n_ = (o_).wrapping_add(64); l_tex_2 = n_; };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    l_mesh_3 = c.m.r32((p_file_1).wrapping_add(28));
    {
        l_i_4 = 0i32;
        'l3: loop {
            if !(((((l_i_4) < ((c.m.r32((p_file_1).wrapping_add(24)) as i32))) as i32)) != 0) { break 'l3; }
            'c4: {
                { let v_ = (((((c.m.r32((l_mesh_3).wrapping_add(8))) as i32)).wrapping_add(((p_file_1) as i32))) as u32); let a_ = (l_mesh_3).wrapping_add(8); c.m.w32(a_, v_); };
                { let o_ = l_mesh_3; let n_ = (o_).wrapping_add(96); l_mesh_3 = n_; };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_Relocate", &PROBE_N, &[], true); }
    r_
}

pub fn EftSurf_Unrelocate(c: &mut Ctx, mut p_file_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_Unrelocate", &PROBE_N, &[(p_file_1) as u32], false); }
    let mut l_tex_2: u32 = 0;
    let mut l_mesh_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_tex_2 = c.m.r32((p_file_1).wrapping_add(20));
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < ((c.m.r32((p_file_1).wrapping_add(16)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = (((p_file_1) as i32)).wrapping_sub((c.m.r32(l_tex_2) as i32)); let a_ = (l_tex_2).wrapping_add(56); c.m.w32(a_, (v_) as u32); };
                { let v_ = (((p_file_1) as i32)).wrapping_sub((c.m.r32((l_tex_2).wrapping_add(4)) as i32)); let a_ = (l_tex_2).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
                { let o_ = l_tex_2; let n_ = (o_).wrapping_add(64); l_tex_2 = n_; };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    l_mesh_3 = c.m.r32((p_file_1).wrapping_add(28));
    {
        l_i_4 = 0i32;
        'l3: loop {
            if !(((((l_i_4) < ((c.m.r32((p_file_1).wrapping_add(24)) as i32))) as i32)) != 0) { break 'l3; }
            'c4: {
                { let v_ = (((((c.m.r32((l_mesh_3).wrapping_add(8))) as i32)).wrapping_sub(((p_file_1) as i32))) as u32); let a_ = (l_mesh_3).wrapping_add(8); c.m.w32(a_, v_); };
                { let o_ = l_mesh_3; let n_ = (o_).wrapping_add(96); l_mesh_3 = n_; };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    { let r_ = ((p_file_1) as i32); let a_ = (p_file_1).wrapping_add(12); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
    { let v_ = (((((c.m.r32((p_file_1).wrapping_add(20))) as i32)).wrapping_sub(((p_file_1) as i32))) as u32); let a_ = (p_file_1).wrapping_add(20); c.m.w32(a_, v_); };
    { let v_ = (((((c.m.r32((p_file_1).wrapping_add(28))) as i32)).wrapping_sub(((p_file_1) as i32))) as u32); let a_ = (p_file_1).wrapping_add(28); c.m.w32(a_, v_); };
    if ((((((c.m.r32((p_file_1).wrapping_add(4)) as i32)) != (4i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (p_file_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (p_file_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_Unrelocate", &PROBE_N, &[], true); }
    r_
}

pub fn EftSurf_UpdateTextures(c: &mut Ctx, mut p_file_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_UpdateTextures", &PROBE_N, &[(p_file_1) as u32], false); }
    let mut l_rt_2: u32 = 0;
    let mut l_tex_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_rt_2 = (c.m.r32(sym::gEftSurf)).wrapping_add(576);
    l_tex_3 = c.m.r32((p_file_1).wrapping_add(20));
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < ((c.m.r32((p_file_1).wrapping_add(16)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = l_tex_3; let a_ = (((c.m.r32(sym::gEftSurf)).wrapping_add(8)).wrapping_add(((((l_i_4).wrapping_add(1i32)) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, v_); };
                { let v_ = c.m.r64((l_tex_3).wrapping_add(48)); let a_ = ((c.m.r32(sym::gEftSurf)).wrapping_add(8)).wrapping_add(((((l_i_4).wrapping_add(1i32)) as u32)).wrapping_mul(16)); c.m.w64(a_, v_); };
                { let v_ = { let a0_ = ((c.m.r32(sym::gEftSurf)).wrapping_add(8)).wrapping_add(((((l_i_4).wrapping_add(1i32)) as u32)).wrapping_mul(16)); F::EftVram_AddTex(c, a0_, 1i32, 0i32) }; let a_ = (l_tex_3).wrapping_add(40); c.m.w64(a_, v_); };
                { let o_ = l_tex_3; let n_ = (o_).wrapping_add(64); l_tex_3 = n_; };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    if (((((c.m.r32((l_rt_2).wrapping_add(4372)) as i32)) & (8i32))) != 0) {
        { let r_ = (!(4i32)); let a_ = (l_rt_2).wrapping_add(4372); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_UpdateTextures", &PROBE_N, &[], true); }
    r_
}

pub fn EftSurf_Animate(c: &mut Ctx, mut p_file_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_Animate", &PROBE_N, &[(p_file_1) as u32], false); }
    let mut l_rt_2: u32 = 0;
    let mut l_count_3: i32 = 0;
    let mut l_mesh_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
    l_rt_2 = (c.m.r32(sym::gEftSurf)).wrapping_add(576);
    l_count_3 = (c.m.r32((p_file_1).wrapping_add(24)) as i32);
    l_mesh_4 = c.m.r32((p_file_1).wrapping_add(28));
    { let r_ = (!(2i32)); let a_ = (l_rt_2).wrapping_add(4372); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((l_i_5) < (l_count_3)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((c.m.r32((l_mesh_4).wrapping_add(32)) as i32)) & (256i32))) != 0) {
                    { let r_ = F32(c.m.r32((l_mesh_4).wrapping_add(16))); let a_ = (l_mesh_4).wrapping_add(24); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if (((rt::feq(F32(c.m.r32((l_mesh_4).wrapping_add(16))), F32(0x00000000)) as i32)) != 0) {
                        { let v_ = F32(0xba83126e); let a_ = (l_mesh_4).wrapping_add(16); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x3f800000); let a_ = (l_mesh_4).wrapping_add(24); c.m.w32(a_, (v_).0); };
                    }
                    if (((rt::fgt(F32(c.m.r32((l_mesh_4).wrapping_add(24))), F32(0x3f800000)) as i32)) != 0) {
                        { let v_ = F32(0x3f800000); let a_ = (l_mesh_4).wrapping_add(24); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fneg(F32(c.m.r32((l_mesh_4).wrapping_add(16)))); let a_ = (l_mesh_4).wrapping_add(16); c.m.w32(a_, (v_).0); };
                    }
                    if (((rt::flt(F32(c.m.r32((l_mesh_4).wrapping_add(24))), F32(0x3f733333)) as i32)) != 0) {
                        { let v_ = F32(0x3f733333); let a_ = (l_mesh_4).wrapping_add(24); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fneg(F32(c.m.r32((l_mesh_4).wrapping_add(16)))); let a_ = (l_mesh_4).wrapping_add(16); c.m.w32(a_, (v_).0); };
                    }
                    { let r_ = F32(c.m.r32((l_mesh_4).wrapping_add(12))); let a_ = (l_mesh_4).wrapping_add(20); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if (((rt::flt(F32(c.m.r32((l_mesh_4).wrapping_add(20))), F32(0x00000000)) as i32)) != 0) {
                        { let v_ = F32(0x00000000); let a_ = (l_mesh_4).wrapping_add(20); c.m.w32(a_, (v_).0); };
                    }
                    if (((rt::fgt(F32(c.m.r32((l_mesh_4).wrapping_add(20))), F32(0x40c90fda)) as i32)) != 0) {
                        { let v_ = F32(0xc0c90fda); let a_ = (l_mesh_4).wrapping_add(20); c.m.w32(a_, (v_).0); };
                    }
                } else {
                    if (((((c.m.r32((l_mesh_4).wrapping_add(32)) as i32)) & (128i32))) != 0) {
                        { let r_ = F32(c.m.r32((l_mesh_4).wrapping_add(16))); let a_ = (l_mesh_4).wrapping_add(24); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        { let r_ = F32(c.m.r32((l_mesh_4).wrapping_add(12))); let a_ = (l_mesh_4).wrapping_add(20); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        if (((rt::fgt(F32(c.m.r32((l_mesh_4).wrapping_add(24))), F32(0x3f800000)) as i32)) != 0) {
                            { let r_ = F32(0x3f800000); let a_ = (l_mesh_4).wrapping_add(24); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        }
                        if (((rt::fgt(F32(c.m.r32((l_mesh_4).wrapping_add(20))), F32(0x40c90fda)) as i32)) != 0) {
                            { let v_ = F32(0xc0c90fda); let a_ = (l_mesh_4).wrapping_add(20); c.m.w32(a_, (v_).0); };
                        }
                    } else {
                        if (((((c.m.r32((l_mesh_4).wrapping_add(32)) as i32)) & (2048i32))) != 0) {
                            if ((((!(((((c.m.r32((l_rt_2).wrapping_add(4372)) as i32)) & (2i32))) != 0)) as i32)) != 0) {
                                F::EftSurf_StepAnim(c);
                                { let r_ = 2i32; let a_ = (l_rt_2).wrapping_add(4372); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            }
                        } else {
                            { let r_ = F32(c.m.r32((l_mesh_4).wrapping_add(12))); let a_ = (l_mesh_4).wrapping_add(20); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            { let r_ = F32(c.m.r32((l_mesh_4).wrapping_add(16))); let a_ = (l_mesh_4).wrapping_add(24); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            if (((rt::fgt(F32(c.m.r32((l_mesh_4).wrapping_add(20))), F32(0x3f800000)) as i32)) != 0) {
                                { let r_ = F32(0x3f800000); let a_ = (l_mesh_4).wrapping_add(20); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            }
                            if (((rt::fgt(F32(c.m.r32((l_mesh_4).wrapping_add(24))), F32(0x3f800000)) as i32)) != 0) {
                                { let r_ = F32(0x3f800000); let a_ = (l_mesh_4).wrapping_add(24); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            }
                        }
                    }
                }
            }
            { { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; }; { let o_ = l_mesh_4; let n_ = (o_).wrapping_add(96); l_mesh_4 = n_; }; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_Animate", &PROBE_N, &[], true); }
    r_
}

pub fn EftSurf_IsBoxVisible(c: &mut Ctx, mut p_min_1: u32, mut p_max_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_IsBoxVisible", &PROBE_N, &[(p_min_1) as u32, (p_max_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(176) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 176);
    let mut l_all_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_code_5: i16 = 0;
    let r_ = 'f: {
        l_all_3 = 0i32;
        { let _ = F::memset(c, fp.wrapping_add(144), 0i32, 20u32); };
        { let a1_ = F32(c.m.r32(p_min_1)); let a2_ = F32(c.m.r32((p_min_1).wrapping_add(4))); let a3_ = F32(c.m.r32((p_min_1).wrapping_add(8))); F::Vec4_Set(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, F32(0x3f800000)) };
        { let a1_ = F32(c.m.r32(p_max_2)); let a2_ = F32(c.m.r32((p_min_1).wrapping_add(4))); let a3_ = F32(c.m.r32((p_min_1).wrapping_add(8))); F::Vec4_Set(c, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, F32(0x3f800000)) };
        { let a1_ = F32(c.m.r32(p_min_1)); let a2_ = F32(c.m.r32((p_max_2).wrapping_add(4))); let a3_ = F32(c.m.r32((p_min_1).wrapping_add(8))); F::Vec4_Set(c, (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, F32(0x3f800000)) };
        { let a1_ = F32(c.m.r32(p_max_2)); let a2_ = F32(c.m.r32((p_max_2).wrapping_add(4))); let a3_ = F32(c.m.r32((p_min_1).wrapping_add(8))); F::Vec4_Set(c, (fp).wrapping_add((((3i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, F32(0x3f800000)) };
        { let a1_ = F32(c.m.r32(p_min_1)); let a2_ = F32(c.m.r32((p_min_1).wrapping_add(4))); let a3_ = F32(c.m.r32((p_max_2).wrapping_add(8))); F::Vec4_Set(c, (fp).wrapping_add((((4i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, F32(0x3f800000)) };
        { let a1_ = F32(c.m.r32(p_max_2)); let a2_ = F32(c.m.r32((p_min_1).wrapping_add(4))); let a3_ = F32(c.m.r32((p_max_2).wrapping_add(8))); F::Vec4_Set(c, (fp).wrapping_add((((5i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, F32(0x3f800000)) };
        { let a1_ = F32(c.m.r32(p_min_1)); let a2_ = F32(c.m.r32((p_max_2).wrapping_add(4))); let a3_ = F32(c.m.r32((p_max_2).wrapping_add(8))); F::Vec4_Set(c, (fp).wrapping_add((((6i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, F32(0x3f800000)) };
        { let a1_ = F32(c.m.r32(p_max_2)); let a2_ = F32(c.m.r32((p_max_2).wrapping_add(4))); let a3_ = F32(c.m.r32((p_max_2).wrapping_add(8))); F::Vec4_Set(c, (fp).wrapping_add((((7i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, F32(0x3f800000)) };
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < (8i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::Vu0Cur_ProjectInt(c, fp.wrapping_add(128), (fp).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)));
                    { let r_ = 1792i32; let a_ = (fp.wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                    { let r_ = 1824i32; let a_ = (fp.wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                    if ((((((c.m.r32((fp.wrapping_add(128)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) < (0i32)) as i32)) != 0) {
                        { let a_ = (fp.wrapping_add(144)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        l_code_5 = ((16i32) as i16);
                    } else {
                        l_code_5 = ((32i32) as i16);
                        if ((((((c.m.r32((fp.wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) < ({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(512); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                            { let a_ = (fp.wrapping_add(144)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            l_code_5 = ((33i32) as i16);
                        }
                        if (((((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(516); (c.m.r32(a_) as i32) }).wrapping_add(1i32)) < ((c.m.r32((fp.wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32))) as i32)) != 0) {
                            { let a_ = (fp.wrapping_add(144)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            { let r_ = 2i32; let n_ = ((((((l_code_5) as i32)) | (r_))) as i16); l_code_5 = n_; };
                        }
                        if ((((((c.m.r32((fp.wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) < ({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(520); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                            { let a_ = (fp.wrapping_add(144)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            { let r_ = 4i32; let n_ = ((((((l_code_5) as i32)) | (r_))) as i16); l_code_5 = n_; };
                        }
                        if (((((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(524); (c.m.r32(a_) as i32) }).wrapping_add(1i32)) < ((c.m.r32((fp.wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32))) as i32)) != 0) {
                            { let a_ = (fp.wrapping_add(144)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            { let r_ = 8i32; let n_ = ((((((l_code_5) as i32)) | (r_))) as i16); l_code_5 = n_; };
                        }
                    }
                    { let r_ = ((l_code_5) as i32); let n_ = ((l_all_3) | (r_)); l_all_3 = n_; };
                    if (((((((((l_code_5) as i32)) & ((!(32i32))))) == (0i32)) as i32)) != 0) {
                        break 'f 1i32;
                    }
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        if ((((((((((l_all_3) & (1i32))).wrapping_add((((l_all_3) & (2i32))).wrapping_shr((1i32) as u32))).wrapping_add((((l_all_3) & (4i32))).wrapping_shr((2i32) as u32))).wrapping_add((((l_all_3) & (8i32))).wrapping_shr((3i32) as u32))) >= (2i32)) as i32)) != 0) {
            break 'f 1i32;
        }
        break 'f (((((l_all_3) & (48i32))) == (48i32)) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_IsBoxVisible", &PROBE_N, &[], true); }
    r_
}

pub fn eft_c__EftSurfRt_GetTex(c: &mut Ctx, mut p_rt_1: u32, mut p_idx_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_c__EftSurfRt_GetTex", &PROBE_N, &[(p_rt_1) as u32, (p_idx_2) as u32], false); }
    let r_ = 'f: {
    break 'f (c.m.r32((p_rt_1).wrapping_add(4216))).wrapping_add((((p_idx_2) as u32)).wrapping_mul(16));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "eft_c__EftSurfRt_GetTex", &PROBE_N, &[], true); }
    r_
}

pub fn EftSurf_DrawMeshes(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_DrawMeshes", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(560) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 560);
    let mut l_w_1: u32 = 0;
    let mut l_rt_2: u32 = 0;
    let mut l_distSq_3: F32 = F32(0);
    let mut l_invDist_4: F32 = F32(0);
    let mut l_texs_5: u32 = 0;
    let mut l_mesh_6: u32 = 0;
    let mut l_meshCount_7: i32 = 0;
    let mut l_bright_8: F32 = F32(0);
    let mut l_i_9: i32 = 0;
    let mut l_j_10: i32 = 0;
    let mut l_tex0_11: u64 = 0;
    let mut l_zBias_12: i32 = 0;
    let mut l_near_13: i32 = 0;
    let mut l_layer_14: i32 = 0;
    let mut l_flags_15: i32 = 0;
    let mut l_u_16: F32 = F32(0);
    let mut l_fade_17: F32 = F32(0);
    let mut l_v_18: F32 = F32(0);
    let mut l_tri_19: u32 = 0;
    let mut l_triCount_20: i32 = 0;
    let mut l_d_21: F32 = F32(0);
    let mut l_k_22: i32 = 0;
    let r_ = 'f: {
        l_w_1 = c.m.r32(sym::gEftSurf);
        l_rt_2 = (l_w_1).wrapping_add(576);
        l_distSq_3 = rt::fmul(F32(c.m.r32(((l_rt_2).wrapping_add(4224)).wrapping_add(56))), F32(c.m.r32(((l_rt_2).wrapping_add(4224)).wrapping_add(56))));
        l_invDist_4 = rt::fdiv(F32(0x3f800000), l_distSq_3);
        l_texs_5 = { let a_ = (c.m.r32((l_w_1).wrapping_add(4))).wrapping_add(20); c.m.r32(a_) };
        l_mesh_6 = { let a_ = (c.m.r32((l_w_1).wrapping_add(4))).wrapping_add(28); c.m.r32(a_) };
        l_meshCount_7 = { let a_ = (c.m.r32((l_w_1).wrapping_add(4))).wrapping_add(24); (c.m.r32(a_) as i32) };
        { let _ = F::memset(c, fp.wrapping_add(464), 0i32, 16u32); };
        F::EftGfx_UpdateClipPlanes(c);
        { let v_ = F::EftGfx_GetClipPlanes(c); let a_ = ((c.m.r32(sym::gEftSurf)).wrapping_add(8)).wrapping_add(8); c.m.w32(a_, v_); };
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(544); F::Vec4_Copy(c, fp.wrapping_add(448), a1_) };
        { let _ = F::BtlStage_GetWaterLevel(c, fp.wrapping_add(556)); };
        { let v_ = F32(c.m.r32(fp.wrapping_add(556))); let a_ = (fp.wrapping_add(448)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        l_bright_8 = F::EftStage_GetTintScale(c);
        F::EftSurf_BeginDraw(c, 0i32, 0i32);
        F::Vec3_Scale(c, fp.wrapping_add(540), ((l_w_1).wrapping_add(576)).wrapping_add(4224), l_bright_8);
        { let v_ = rt::fmul(F32(c.m.r32(((l_rt_2).wrapping_add(4224)).wrapping_add((((3i32) as u32)).wrapping_mul(4)))), F32(0x3c000000)); let a_ = (fp.wrapping_add(540)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        if (((((c.m.r32((l_rt_2).wrapping_add(4216))) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = { let a_ = { let a1_ = ((c.m.r32((l_rt_2).wrapping_add(4360)) as i32)).wrapping_add((c.m.r32((l_rt_2).wrapping_add(4368)) as i32)); F::eft_c__EftSurfRt_GetTex(c, l_rt_2, a1_) }; c.m.r64(a_) }; let a_ = (fp.wrapping_add(464)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = { let a_ = { let a1_ = ((c.m.r32((l_rt_2).wrapping_add(4364)) as i32)).wrapping_add((c.m.r32((l_rt_2).wrapping_add(4368)) as i32)); F::eft_c__EftSurfRt_GetTex(c, l_rt_2, a1_) }; c.m.r64(a_) }; let a_ = (fp.wrapping_add(464)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        }
        F::Vu0Cur_Push(c);
        { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
        {
            l_i_9 = 0i32;
            'l1: loop {
                if !(((((l_i_9) < (l_meshCount_7)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_tex0_11 = { let a_ = ((l_texs_5).wrapping_add(((((c.m.r32(l_mesh_6) as i32)) as u32)).wrapping_mul(64))).wrapping_add(40); c.m.r64(a_) };
                    l_flags_15 = (c.m.r32((l_mesh_6).wrapping_add(32)) as i32);
                    l_u_16 = F32(c.m.r32((l_mesh_6).wrapping_add(20)));
                    l_fade_17 = F32(0x3f800000);
                    l_v_18 = F32(c.m.r32((l_mesh_6).wrapping_add(24)));
                    l_zBias_12 = (c.m.r32((l_mesh_6).wrapping_add(28)) as i32);
                    l_tri_19 = c.m.r32((l_mesh_6).wrapping_add(8));
                    l_triCount_20 = (c.m.r32((l_mesh_6).wrapping_add(4)) as i32);
                    if ((((l_flags_15) & (96i32))) != 0) {
                        break 'c2;
                    }
                    if ((((!((F::EftSurf_IsBoxVisible(c, (l_mesh_6).wrapping_add(48), (l_mesh_6).wrapping_add(64))) != 0)) as i32)) != 0) {
                        break 'c2;
                    }
                    if ((((l_flags_15) & (512i32))) != 0) {
                        F::Vec4_Sub(c, fp.wrapping_add(432), (l_mesh_6).wrapping_add(80), fp.wrapping_add(448));
                        l_d_21 = F::Vec3_Dot(c, fp.wrapping_add(432), fp.wrapping_add(432));
                        if (((rt::fgt(l_d_21, F32(0x481c4000)) as i32)) != 0) {
                            break 'c2;
                        }
                        if (((rt::fgt(l_d_21, F32(0x471c4000)) as i32)) != 0) {
                            { let r_ = rt::fdiv(rt::fsub(l_d_21, F32(0x471c4000)), F32(0x47ea6000)); let n_ = rt::fsub(l_fade_17, r_); l_fade_17 = n_; };
                        }
                    }
                    l_layer_14 = 1i32;
                    if ((((!((((l_flags_15) & (8i32))) != 0)) as i32)) != 0) {
                        l_layer_14 = (if ((((l_flags_15) & (16i32))) != 0) { 2i32 } else { 0i32 });
                    }
                    {
                        l_j_10 = 0i32;
                        'l3: loop {
                            if !(((((l_j_10) < (l_triCount_20)) as i32)) != 0) { break 'l3; }
                            'c4: {
                                { let v_ = F::Vec3_DistSq(c, (l_tri_19).wrapping_add((((0i32) as u32)).wrapping_mul(48)), fp.wrapping_add(448)); let a_ = (fp.wrapping_add(480)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let v_ = F::Vec3_DistSq(c, (l_tri_19).wrapping_add((((1i32) as u32)).wrapping_mul(48)), fp.wrapping_add(448)); let a_ = (fp.wrapping_add(480)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let v_ = F::Vec3_DistSq(c, (l_tri_19).wrapping_add((((2i32) as u32)).wrapping_mul(48)), fp.wrapping_add(448)); let a_ = (fp.wrapping_add(480)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                l_near_13 = 1i32;
                                {
                                    l_k_22 = 0i32;
                                    'l5: loop {
                                        if !(((((l_k_22) < (3i32)) as i32)) != 0) { break 'l5; }
                                        'c6: {
                                            F::ClipVtx_Set(c, (fp).wrapping_add((((l_k_22) as u32)).wrapping_mul(48)), (l_tri_19).wrapping_add((((l_k_22) as u32)).wrapping_mul(48)), ((l_tri_19).wrapping_add((((l_k_22) as u32)).wrapping_mul(48))).wrapping_add(16), ((l_tri_19).wrapping_add((((l_k_22) as u32)).wrapping_mul(48))).wrapping_add(32));
                                            { let v_ = F32(0x3f800000); let a_ = ((fp).wrapping_add((((l_k_22) as u32)).wrapping_mul(48))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                            { let v_ = F32(0x3f800000); let a_ = (((fp).wrapping_add((((l_k_22) as u32)).wrapping_mul(48))).wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                            { let v_ = F32(0x3f800000); let a_ = (((fp).wrapping_add((((l_k_22) as u32)).wrapping_mul(48))).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                            if ((((l_flags_15) & (128i32))) != 0) {
                                                if (((rt::flt(F32(c.m.r32((((fp).wrapping_add((((l_k_22) as u32)).wrapping_mul(48))).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(0x3f000000)) as i32)) != 0) {
                                                    { let r_ = rt::fmul(F::Mathf_Sin(c, l_u_16), F32(0x3d4ccccc)); let a_ = (((fp).wrapping_add((((l_k_22) as u32)).wrapping_mul(48))).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                                } else {
                                                    { let r_ = rt::fmul(F::Mathf_Sin(c, l_u_16), F32(0x3d4ccccc)); let a_ = (((fp).wrapping_add((((l_k_22) as u32)).wrapping_mul(48))).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                                }
                                                { let r_ = l_v_18; let a_ = (((fp).wrapping_add((((l_k_22) as u32)).wrapping_mul(48))).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                            } else {
                                                if ((((l_flags_15) & (2048i32))) != 0) {
                                                    F::Vec4_Copy(c, ((fp).wrapping_add((((l_k_22) as u32)).wrapping_mul(48))).wrapping_add(32), fp.wrapping_add(540));
                                                    { let r_ = F32(c.m.r32((((l_tri_19).wrapping_add((((l_k_22) as u32)).wrapping_mul(48))).wrapping_add(32)).wrapping_add((((3i32) as u32)).wrapping_mul(4)))); let a_ = (((fp).wrapping_add((((l_k_22) as u32)).wrapping_mul(48))).wrapping_add(32)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                                    if (((rt::fgt(F32(c.m.r32((fp.wrapping_add(480)).wrapping_add((((l_k_22) as u32)).wrapping_mul(4)))), l_distSq_3) as i32)) != 0) {
                                                        { let v_ = l_distSq_3; let a_ = (fp.wrapping_add(480)).wrapping_add((((l_k_22) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                                    }
                                                    if (((rt::fgt(F32(c.m.r32((fp.wrapping_add(480)).wrapping_add((((l_k_22) as u32)).wrapping_mul(4)))), F32(0x4845c100)) as i32)) != 0) {
                                                        l_near_13 = 0i32;
                                                    }
                                                    { let v_ = rt::fsub(F32(0x3f800000), rt::fmul(F32(c.m.r32((fp.wrapping_add(480)).wrapping_add((((l_k_22) as u32)).wrapping_mul(4)))), l_invDist_4)); let a_ = ((fp.wrapping_add(492)).wrapping_add((((l_k_22) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                                    if (((rt::fgt(F32(c.m.r32(((fp.wrapping_add(492)).wrapping_add((((l_k_22) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)))), F32(0x3f800000)) as i32)) != 0) {
                                                        { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(492)).wrapping_add((((l_k_22) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                                    } else {
                                                        if (((rt::flt(F32(c.m.r32(((fp.wrapping_add(492)).wrapping_add((((l_k_22) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)))), F32(0x00000000)) as i32)) != 0) {
                                                            { let v_ = F32(0x00000000); let a_ = ((fp.wrapping_add(492)).wrapping_add((((l_k_22) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                                        }
                                                    }
                                                } else {
                                                    { let r_ = l_u_16; let a_ = (((fp).wrapping_add((((l_k_22) as u32)).wrapping_mul(48))).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                                    { let r_ = l_v_18; let a_ = (((fp).wrapping_add((((l_k_22) as u32)).wrapping_mul(48))).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                                    { let r_ = l_fade_17; let a_ = (((fp).wrapping_add((((l_k_22) as u32)).wrapping_mul(48))).wrapping_add(32)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                                }
                                            }
                                            F::Vec3_Scale(c, ((fp).wrapping_add((((l_k_22) as u32)).wrapping_mul(48))).wrapping_add(32), ((fp).wrapping_add((((l_k_22) as u32)).wrapping_mul(48))).wrapping_add(32), l_bright_8);
                                        }
                                        { let o_ = l_k_22; let n_ = (o_).wrapping_add(1); l_k_22 = n_; };
                                    }
                                }
                                if (((((((l_flags_15) & (((1024i32) | (2048i32))))) == (((1024i32) | (2048i32)))) as i32)) != 0) {
                                    if ((l_near_13) != 0) {
                                        F::EftSurf_DrawReflectTriClipped(c, fp, fp.wrapping_add(464), fp.wrapping_add(492));
                                    } else {
                                        F::EftSurf_DrawReflectTri(c, fp, fp.wrapping_add(464), fp.wrapping_add(492));
                                    }
                                } else {
                                    if ((((l_flags_15) & (1024i32))) != 0) {
                                        if ((l_near_13) != 0) {
                                            F::EftSurf_DrawTriClipped(c, fp, l_tex0_11);
                                        } else {
                                            F::EftSurf_DrawTri(c, fp, l_tex0_11);
                                        }
                                    } else {
                                        if ((((l_flags_15) & (2048i32))) != 0) {
                                            F::EftSurf_DrawPolyOtClipped(c, fp, 0i32, fp.wrapping_add(464), fp.wrapping_add(492), l_zBias_12);
                                        } else {
                                            F::EftSurf_DrawTriOtClipped(c, fp, l_layer_14, 1i32, 0i32, l_tex0_11, l_zBias_12);
                                        }
                                    }
                                }
                            }
                            { { let o_ = l_j_10; let n_ = (o_).wrapping_add(1); l_j_10 = n_; }; { let o_ = l_tri_19; let n_ = (o_).wrapping_add(144); l_tri_19 = n_; }; };
                        }
                    }
                }
                { { let o_ = l_i_9; let n_ = (o_).wrapping_add(1); l_i_9 = n_; }; { let o_ = l_mesh_6; let n_ = (o_).wrapping_add(96); l_mesh_6 = n_; }; };
            }
        }
        F::EftSurf_EndDraw(c, 0i32);
        F::Vu0Cur_Pop(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_DrawMeshes", &PROBE_N, &[], true); }
    r_
}

pub fn EftSurf_PutTri(c: &mut Ctx, mut p_xyz0_1: u32, mut p_xyz1_2: u32, mut p_xyz2_3: u32, mut p_st0_4: u32, mut p_st1_5: u32, mut p_st2_6: u32, mut p_rgba0_7: u32, mut p_rgba1_8: u32, mut p_rgba2_9: u32, mut p_tex0_10: u64) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_PutTri", &PROBE_N, &[(p_xyz0_1) as u32, (p_xyz1_2) as u32, (p_xyz2_3) as u32, (p_st0_4) as u32, (p_st1_5) as u32, (p_st2_6) as u32, (p_rgba0_7) as u32, (p_rgba1_8) as u32, (p_rgba2_9) as u32, (p_tex0_10) as u32], false); }
    let mut l_p_11: u32 = 0;
    let r_ = 'f: {
    l_p_11 = F::Dma_BeginDirect(c);
    { let v_ = ((((((((1i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((12i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_11).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((269523459842400i64) as u64); let a_ = (l_p_11).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_11).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_11 = n_; };
    { let v_ = ((91i32) as u64); let a_ = (l_p_11).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = p_tex0_10; let a_ = (l_p_11).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_11).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_11 = n_; };
    { let v_ = ((((((((((rt::f2i(F32(c.m.r32(p_rgba0_7)))) as u64)) | ((((rt::f2i(F32(c.m.r32((p_rgba0_7).wrapping_add(4))))) as u64)).wrapping_shl((8i32) as u32)))) | ((((rt::f2i(F32(c.m.r32((p_rgba0_7).wrapping_add(8))))) as u64)).wrapping_shl((16i32) as u32)))) | ((((rt::f2i(F32(c.m.r32((p_rgba0_7).wrapping_add(12))))) as u64)).wrapping_shl((24i32) as u32)))) as u32); let a_ = (l_p_11).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = c.m.r32((p_st0_4).wrapping_add(8)); let a_ = (l_p_11).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = c.m.r32(p_st0_4); let a_ = (l_p_11).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = c.m.r32((p_st0_4).wrapping_add(4)); let a_ = (l_p_11).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_11).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_11 = n_; };
    { let v_ = (((((c.m.r32((p_xyz0_1).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) | (((c.m.r32((p_xyz0_1).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)).wrapping_shl((16i32) as u32)))) as u32); let a_ = (l_p_11).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (((c.m.r32((p_xyz0_1).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u32); let a_ = (l_p_11).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((((((((rt::f2i(F32(c.m.r32(p_rgba1_8)))) as u64)) | ((((rt::f2i(F32(c.m.r32((p_rgba1_8).wrapping_add(4))))) as u64)).wrapping_shl((8i32) as u32)))) | ((((rt::f2i(F32(c.m.r32((p_rgba1_8).wrapping_add(8))))) as u64)).wrapping_shl((16i32) as u32)))) | ((((rt::f2i(F32(c.m.r32((p_rgba1_8).wrapping_add(12))))) as u64)).wrapping_shl((24i32) as u32)))) as u32); let a_ = (l_p_11).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = c.m.r32((p_st1_5).wrapping_add(8)); let a_ = (l_p_11).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_11).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_11 = n_; };
    { let v_ = c.m.r32(p_st1_5); let a_ = (l_p_11).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = c.m.r32((p_st1_5).wrapping_add(4)); let a_ = (l_p_11).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (((((c.m.r32((p_xyz1_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) | (((c.m.r32((p_xyz1_2).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)).wrapping_shl((16i32) as u32)))) as u32); let a_ = (l_p_11).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (((c.m.r32((p_xyz1_2).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u32); let a_ = (l_p_11).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_11).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_11 = n_; };
    { let v_ = ((((((((((rt::f2i(F32(c.m.r32(p_rgba2_9)))) as u64)) | ((((rt::f2i(F32(c.m.r32((p_rgba2_9).wrapping_add(4))))) as u64)).wrapping_shl((8i32) as u32)))) | ((((rt::f2i(F32(c.m.r32((p_rgba2_9).wrapping_add(8))))) as u64)).wrapping_shl((16i32) as u32)))) | ((((rt::f2i(F32(c.m.r32((p_rgba2_9).wrapping_add(12))))) as u64)).wrapping_shl((24i32) as u32)))) as u32); let a_ = (l_p_11).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = c.m.r32((p_st2_6).wrapping_add(8)); let a_ = (l_p_11).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = c.m.r32(p_st2_6); let a_ = (l_p_11).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = c.m.r32((p_st2_6).wrapping_add(4)); let a_ = (l_p_11).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_11).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_11 = n_; };
    { let v_ = (((((c.m.r32((p_xyz2_3).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) | (((c.m.r32((p_xyz2_3).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)).wrapping_shl((16i32) as u32)))) as u32); let a_ = (l_p_11).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (((c.m.r32((p_xyz2_3).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u32); let a_ = (l_p_11).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u64); let a_ = (l_p_11).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_11).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_11 = n_; };
    F::Dma_EndDirect(c, l_p_11);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_PutTri", &PROBE_N, &[], true); }
    r_
}

pub fn EftSurf_BeginDraw(c: &mut Ctx, mut p_blend_1: i32, mut p_noDepthTest_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_BeginDraw", &PROBE_N, &[(p_blend_1) as u32, (p_noDepthTest_2) as u32], false); }
    let mut l_p_3: u32 = 0;
    let mut l_ztst_4: i32 = 0;
    let r_ = 'f: {
    l_ztst_4 = 3i32;
    if ((p_noDepthTest_2) != 0) {
        l_ztst_4 = 1i32;
    }
    l_p_3 = F::Dma_BeginDirect(c);
    { let v_ = ((((((((14i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    if (((((p_blend_1) == (0i32)) as i32)) != 0) {
        { let v_ = ((68i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((66i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    } else {
        if (((((p_blend_1) == (1i32)) as i32)) != 0) {
            { let v_ = ((72i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((66i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
        } else {
            if (((((p_blend_1) == (2i32)) as i32)) != 0) {
                { let v_ = ((66i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let v_ = ((66i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
            }
        }
    }
    { let v_ = ((96i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((20i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = (((((l_ztst_4) as u64)).wrapping_shl((17i32) as u32)) | (((65536i32) as u64))); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((71i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = ((0i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((8i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = ((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((78i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = ((1i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((70i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = ((72i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((67i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = ((96i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((21i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = (((((l_ztst_4) as u64)).wrapping_shl((17i32) as u32)) | (((65536i32) as u64))); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((72i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = ((0i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((9i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = ((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((79i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { 524400i32 } else { 524288i32 })) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((77i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = ((((28672i32) as u64)) | ((((29184i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((25i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = ((((((((0i32) as u64)) | ((((511i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) | ((((447i32) as u64)).wrapping_shl((48i32) as u32))); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((65i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    F::Dma_EndDirect(c, l_p_3);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_BeginDraw", &PROBE_N, &[], true); }
    r_
}

pub fn EftSurf_EndDraw(c: &mut Ctx, mut p_arg_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_EndDraw", &PROBE_N, &[(p_arg_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let mut l_clamp_3: i32 = 0;
    let r_ = 'f: {
    l_clamp_3 = (((p_arg_1) == (0i32)) as i32);
    l_p_2 = F::Dma_BeginDirect(c);
    { let v_ = ((((((((11i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
    { let v_ = ((68i32) as u64); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((66i32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
    { let v_ = ((96i32) as u64); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((20i32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
    { let v_ = ((327680i32) as u64); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((71i32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
    { let v_ = ((((((((((((l_clamp_3) as u64)) | ((((l_clamp_3) as u64)).wrapping_shl((2i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((4i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((14i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((34i32) as u32))); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((8i32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
    { let v_ = ((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((78i32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
    { let v_ = ((1i32) as u64); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((70i32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
    { let v_ = ((68i32) as u64); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((67i32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
    { let v_ = ((96i32) as u64); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((21i32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
    { let v_ = ((327680i32) as u64); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((72i32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
    { let v_ = ((((((((((((l_clamp_3) as u64)) | ((((l_clamp_3) as u64)).wrapping_shl((2i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((4i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((14i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((34i32) as u32))); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((9i32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
    { let v_ = ((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((79i32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
    F::Dma_EndDirect(c, l_p_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_EndDraw", &PROBE_N, &[], true); }
    r_
}

pub fn eft_c__EftSurf_IsOffScreen(c: &mut Ctx, mut p_xyz_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_c__EftSurf_IsOffScreen", &PROBE_N, &[(p_xyz_1) as u32], false); }
    let mut l_xMin_2: i32 = 0;
    let mut l_xMax_3: i32 = 0;
    let mut l_yMax_4: i32 = 0;
    let mut l_yMin_5: i32 = 0;
    let r_ = 'f: {
    l_xMin_2 = 1776i32;
    l_xMax_3 = 2320i32;
    l_yMax_4 = 2288i32;
    l_yMin_5 = 1808i32;
    if ((((!((F::BtlScene_IsSingleView(c)) != 0)) as i32)) != 0) {
        if ((((!((F::BtlScene_IsSecondView(c)) != 0)) as i32)) != 0) {
            l_xMax_3 = 2064i32;
        } else {
            l_xMin_2 = 2032i32;
        }
    }
    if ((((((c.m.r32((p_xyz_1).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) <= (0i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    if ((((((c.m.r32((p_xyz_1).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) >= ((l_xMax_3).wrapping_shl((4i32) as u32))) as i32)) != 0) {
        break 'f 1i32;
    }
    if ((((((c.m.r32((p_xyz_1).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) <= ((l_xMin_2).wrapping_shl((4i32) as u32))) as i32)) != 0) {
        break 'f 1i32;
    }
    if ((((((c.m.r32((p_xyz_1).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) >= ((l_yMax_4).wrapping_shl((4i32) as u32))) as i32)) != 0) {
        break 'f 1i32;
    }
    if ((((((c.m.r32((p_xyz_1).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) <= ((l_yMin_5).wrapping_shl((4i32) as u32))) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "eft_c__EftSurf_IsOffScreen", &PROBE_N, &[], true); }
    r_
}

pub fn EftSurf_DrawTri(c: &mut Ctx, mut p_tri_1: u32, mut p_tex0_2: u64) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_DrawTri", &PROBE_N, &[(p_tri_1) as u32, (p_tex0_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let r_ = 'f: {
        F::ClipPoly_ProjectCur(c, fp, fp.wrapping_add(48), p_tri_1, 3i32);
        if ((((((((((F::eft_c__EftSurf_IsOffScreen(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)))) != 0) && ((F::eft_c__EftSurf_IsOffScreen(c, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)))) != 0)) as i32)) != 0) && ((F::eft_c__EftSurf_IsOffScreen(c, (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)))) != 0)) as i32)) != 0) {
            break 'f;
        }
        F::EftSurf_PutTri(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), ((p_tri_1).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(32), ((p_tri_1).wrapping_add((((1i32) as u32)).wrapping_mul(48))).wrapping_add(32), ((p_tri_1).wrapping_add((((2i32) as u32)).wrapping_mul(48))).wrapping_add(32), p_tex0_2);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_DrawTri", &PROBE_N, &[], true); }
    r_
}

pub fn EftSurf_DrawTriClipped(c: &mut Ctx, mut p_tri_1: u32, mut p_tex0_2: u64) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_DrawTriClipped", &PROBE_N, &[(p_tri_1) as u32, (p_tex0_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(1056) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 1056);
    let mut l_plane_3: u32 = 0;
    let mut l_count_4: i32 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
        { let _ = { rt::memcpy(c, fp, p_tri_1, ((((3i32) as u32)).wrapping_mul(48u32)) as u32); fp }; };
        p_tri_1 = fp;
        l_plane_3 = { let a_ = ((c.m.r32(sym::gEftSurf)).wrapping_add(8)).wrapping_add(8); c.m.r32(a_) };
        l_count_4 = 3i32;
        {
            l_i_5 = 4i32;
            'l1: loop {
                if !(((((l_i_5) >= (0i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_count_4 = F::ClipPoly_ClipPlane(c, p_tri_1, l_plane_3, l_count_4);
                    { let o_ = l_plane_3; let n_ = (o_).wrapping_add(16); l_plane_3 = n_; };
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_sub(1); l_i_5 = n_; };
            }
        }
        if (((((l_count_4) != (0i32)) as i32)) != 0) {
            F::ClipPoly_ProjectCur(c, fp.wrapping_add(768), fp.wrapping_add(912), p_tri_1, l_count_4);
            {
                l_i_5 = 2i32;
                'l3: loop {
                    if !(((((l_i_5) < (l_count_4)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        if ((((((c.m.r32(((fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) > (16777215i32)) as i32)) != 0) {
                            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        }
                        if ((((((c.m.r32(((fp.wrapping_add(768)).wrapping_add(((((l_i_5).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) > (16777215i32)) as i32)) != 0) {
                            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add(((((l_i_5).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        }
                        if ((((((c.m.r32(((fp.wrapping_add(768)).wrapping_add((((l_i_5) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) > (16777215i32)) as i32)) != 0) {
                            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add((((l_i_5) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        }
                        F::EftSurf_PutTri(c, (fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(768)).wrapping_add(((((l_i_5).wrapping_sub(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(768)).wrapping_add((((l_i_5) as u32)).wrapping_mul(16)), (fp.wrapping_add(912)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(912)).wrapping_add(((((l_i_5).wrapping_sub(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(912)).wrapping_add((((l_i_5) as u32)).wrapping_mul(16)), ((p_tri_1).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(32), ((p_tri_1).wrapping_add(((((l_i_5).wrapping_sub(1i32)) as u32)).wrapping_mul(48))).wrapping_add(32), ((p_tri_1).wrapping_add((((l_i_5) as u32)).wrapping_mul(48))).wrapping_add(32), p_tex0_2);
                    }
                    { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_DrawTriClipped", &PROBE_N, &[], true); }
    r_
}

pub fn EftSurf_DrawReflectTri(c: &mut Ctx, mut p_tri_1: u32, mut p_tex_2: u32, mut p_fog_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_DrawReflectTri", &PROBE_N, &[(p_tri_1) as u32, (p_tex_2) as u32, (p_fog_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let r_ = 'f: {
        F::ClipPoly_ProjectCur(c, fp, fp.wrapping_add(48), p_tri_1, 3i32);
        if ((((((((((F::eft_c__EftSurf_IsOffScreen(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)))) != 0) && ((F::eft_c__EftSurf_IsOffScreen(c, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)))) != 0)) as i32)) != 0) && ((F::eft_c__EftSurf_IsOffScreen(c, (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)))) != 0)) as i32)) != 0) {
            break 'f;
        }
        F::EftSurf_DrawTriDirect(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)), ((p_tri_1).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(32), ((p_tri_1).wrapping_add((((1i32) as u32)).wrapping_mul(48))).wrapping_add(32), ((p_tri_1).wrapping_add((((2i32) as u32)).wrapping_mul(48))).wrapping_add(32), (fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), 0i32, p_tex_2, p_fog_3, 2i32);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_DrawReflectTri", &PROBE_N, &[], true); }
    r_
}

pub fn EftSurf_DrawReflectTriClipped(c: &mut Ctx, mut p_tri_1: u32, mut p_tex_2: u32, mut p_fog_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_DrawReflectTriClipped", &PROBE_N, &[(p_tri_1) as u32, (p_tex_2) as u32, (p_fog_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(1056) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 1056);
    let mut l_plane_4: u32 = 0;
    let mut l_count_5: i32 = 0;
    let mut l_i_6: i32 = 0;
    let r_ = 'f: {
        { let _ = { rt::memcpy(c, fp, p_tri_1, ((((3i32) as u32)).wrapping_mul(48u32)) as u32); fp }; };
        p_tri_1 = fp;
        l_plane_4 = { let a_ = ((c.m.r32(sym::gEftSurf)).wrapping_add(8)).wrapping_add(8); c.m.r32(a_) };
        l_count_5 = 3i32;
        {
            l_i_6 = 4i32;
            'l1: loop {
                if !(((((l_i_6) >= (0i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_count_5 = F::ClipPoly_ClipPlane(c, p_tri_1, l_plane_4, l_count_5);
                    { let o_ = l_plane_4; let n_ = (o_).wrapping_add(16); l_plane_4 = n_; };
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_sub(1); l_i_6 = n_; };
            }
        }
        if (((((l_count_5) != (0i32)) as i32)) != 0) {
            F::ClipPoly_ProjectCur(c, fp.wrapping_add(768), fp.wrapping_add(912), p_tri_1, l_count_5);
            {
                l_i_6 = 2i32;
                'l3: loop {
                    if !(((((l_i_6) < (l_count_5)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        F::EftSurf_DrawTriDirect(c, (fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(768)).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(768)).wrapping_add((((l_i_6) as u32)).wrapping_mul(16)), ((p_tri_1).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(32), ((p_tri_1).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(48))).wrapping_add(32), ((p_tri_1).wrapping_add((((l_i_6) as u32)).wrapping_mul(48))).wrapping_add(32), (fp.wrapping_add(912)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(912)).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(912)).wrapping_add((((l_i_6) as u32)).wrapping_mul(16)), 0i32, p_tex_2, p_fog_3, 2i32);
                    }
                    { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_DrawReflectTriClipped", &PROBE_N, &[], true); }
    r_
}

pub fn eft_c__EftSurf_QueueTri(c: &mut Ctx, mut p_v0_1: u32, mut p_v1_2: u32, mut p_v2_3: u32, mut p_c0_4: u32, mut p_c1_5: u32, mut p_c2_6: u32, mut p_uv0_7: u32, mut p_uv1_8: u32, mut p_uv2_9: u32, mut p_layer_10: i32, mut p_z_11: i32, mut p_tex0_12: u64) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_c__EftSurf_QueueTri", &PROBE_N, &[(p_v0_1) as u32, (p_v1_2) as u32, (p_v2_3) as u32, (p_c0_4) as u32, (p_c1_5) as u32, (p_c2_6) as u32, (p_uv0_7) as u32, (p_uv1_8) as u32, (p_uv2_9) as u32, (p_layer_10) as u32, (p_z_11) as u32, (p_tex0_12) as u32], false); }
    let mut l_abe_13: i32 = 0;
    let mut l_l_14: i32 = 0;
    let mut l_p_15: u32 = 0;
    let mut l_e_16: u32 = 0;
    let r_ = 'f: {
    l_abe_13 = 1i32;
    if (((((((((((rt::flt(F32(c.m.r32((p_c0_4).wrapping_add((((3i32) as u32)).wrapping_mul(4)))), F32(0x3dcccccc)) as i32)) != 0) && (((rt::flt(F32(c.m.r32((p_c1_5).wrapping_add((((3i32) as u32)).wrapping_mul(4)))), F32(0x3dcccccc)) as i32)) != 0)) as i32)) != 0) && (((rt::flt(F32(c.m.r32((p_c2_6).wrapping_add((((3i32) as u32)).wrapping_mul(4)))), F32(0x3dcccccc)) as i32)) != 0)) as i32)) != 0) {
        break 'f;
    }
    if (((((p_layer_10) < (0i32)) as i32)) != 0) {
        p_layer_10 = 0i32;
        l_abe_13 = 0i32;
    }
    l_p_15 = c.m.r32(sym::gOtCur);
    { let v_ = (l_p_15).wrapping_add((((1i32) as u32)).wrapping_mul(128)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
    { let v_ = (((((l_abe_13) as u64)).wrapping_shl((6i32) as u32)) | (((27i32) as u64))); let a_ = (l_p_15).wrapping_add(32); c.m.w64(a_, v_); };
    { let v_ = ((536870919i32) as u32); let a_ = l_p_15; c.m.w32(a_, v_); };
    { let v_ = ((268435456i32) as u32); let a_ = (l_p_15).wrapping_add(8); c.m.w32(a_, v_); };
    { let v_ = ((1342177287i32) as u32); let a_ = (l_p_15).wrapping_add(12); c.m.w32(a_, v_); };
    { let v_ = 14123288431433908225u64; let a_ = (l_p_15).wrapping_add(16); c.m.w64(a_, v_); };
    { let v_ = ((268423679713632i64) as u64); let a_ = (l_p_15).wrapping_add(24); c.m.w64(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_15).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u64); let a_ = (l_p_15).wrapping_add(120); c.m.w64(a_, v_); };
    { let v_ = ((rt::f2i(F32(c.m.r32((p_c0_4).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))) as u8); let a_ = (l_p_15).wrapping_add(48); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2i(F32(c.m.r32((p_c0_4).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))) as u8); let a_ = ((l_p_15).wrapping_add(48)).wrapping_add(1); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2i(F32(c.m.r32((p_c0_4).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))) as u8); let a_ = ((l_p_15).wrapping_add(48)).wrapping_add(2); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2i(F32(c.m.r32((p_c0_4).wrapping_add((((3i32) as u32)).wrapping_mul(4)))))) as u8); let a_ = ((l_p_15).wrapping_add(48)).wrapping_add(3); c.m.w8(a_, v_); };
    { let v_ = F32(c.m.r32((p_uv0_7).wrapping_add(8))); let a_ = ((l_p_15).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = ((rt::f2i(F32(c.m.r32((p_c1_5).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))) as u8); let a_ = (l_p_15).wrapping_add(72); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2i(F32(c.m.r32((p_c1_5).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))) as u8); let a_ = ((l_p_15).wrapping_add(72)).wrapping_add(1); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2i(F32(c.m.r32((p_c1_5).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))) as u8); let a_ = ((l_p_15).wrapping_add(72)).wrapping_add(2); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2i(F32(c.m.r32((p_c1_5).wrapping_add((((3i32) as u32)).wrapping_mul(4)))))) as u8); let a_ = ((l_p_15).wrapping_add(72)).wrapping_add(3); c.m.w8(a_, v_); };
    { let v_ = F32(c.m.r32((p_uv1_8).wrapping_add(8))); let a_ = ((l_p_15).wrapping_add(72)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = ((rt::f2i(F32(c.m.r32((p_c2_6).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))) as u8); let a_ = (l_p_15).wrapping_add(96); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2i(F32(c.m.r32((p_c2_6).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))) as u8); let a_ = ((l_p_15).wrapping_add(96)).wrapping_add(1); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2i(F32(c.m.r32((p_c2_6).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))) as u8); let a_ = ((l_p_15).wrapping_add(96)).wrapping_add(2); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2i(F32(c.m.r32((p_c2_6).wrapping_add((((3i32) as u32)).wrapping_mul(4)))))) as u8); let a_ = ((l_p_15).wrapping_add(96)).wrapping_add(3); c.m.w8(a_, v_); };
    l_l_14 = p_layer_10;
    if (((((l_l_14) >= (2i32)) as i32)) != 0) {
        { let r_ = 2i32; let n_ = (l_l_14).wrapping_sub(r_); l_l_14 = n_; };
    }
    { let v_ = F32(c.m.r32((p_uv2_9).wrapping_add(8))); let a_ = ((l_p_15).wrapping_add(96)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = p_tex0_12; let a_ = (l_p_15).wrapping_add(40); c.m.w64(a_, v_); };
    { let v_ = F32(c.m.r32(p_uv0_7)); let a_ = (l_p_15).wrapping_add(56); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_uv0_7).wrapping_add(4))); let a_ = ((l_p_15).wrapping_add(56)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(p_uv1_8)); let a_ = (l_p_15).wrapping_add(80); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_uv1_8).wrapping_add(4))); let a_ = ((l_p_15).wrapping_add(80)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(p_uv2_9)); let a_ = (l_p_15).wrapping_add(104); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_uv2_9).wrapping_add(4))); let a_ = ((l_p_15).wrapping_add(104)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = (((c.m.r32(p_v0_1) as i32)) as u64); let a_ = (l_p_15).wrapping_add(64); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
    { let v_ = (((c.m.r32((p_v0_1).wrapping_add(4)) as i32)) as u64); let a_ = ((l_p_15).wrapping_add(64)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
    { let v_ = (((c.m.r32((p_v0_1).wrapping_add(8)) as i32)) as u64); let a_ = ((l_p_15).wrapping_add(64)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
    { let v_ = ((255i32) as u64); let a_ = ((l_p_15).wrapping_add(64)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
    { let v_ = (((c.m.r32(p_v1_2) as i32)) as u64); let a_ = (l_p_15).wrapping_add(88); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
    { let v_ = (((c.m.r32((p_v1_2).wrapping_add(4)) as i32)) as u64); let a_ = ((l_p_15).wrapping_add(88)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
    { let v_ = (((c.m.r32((p_v1_2).wrapping_add(8)) as i32)) as u64); let a_ = ((l_p_15).wrapping_add(88)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
    { let v_ = ((255i32) as u64); let a_ = ((l_p_15).wrapping_add(88)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
    { let v_ = (((c.m.r32(p_v2_3) as i32)) as u64); let a_ = (l_p_15).wrapping_add(112); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
    { let v_ = (((c.m.r32((p_v2_3).wrapping_add(4)) as i32)) as u64); let a_ = ((l_p_15).wrapping_add(112)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
    { let v_ = (((c.m.r32((p_v2_3).wrapping_add(8)) as i32)) as u64); let a_ = ((l_p_15).wrapping_add(112)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
    { let v_ = ((255i32) as u64); let a_ = ((l_p_15).wrapping_add(112)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
    if (((((p_z_11) < (0i32)) as i32)) != 0) {
        l_e_16 = ((c.m.r32(sym::gOtZ)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_14) as u32)).wrapping_mul(8));
    } else {
        if (((((p_z_11) >= (4096i32)) as i32)) != 0) {
            l_e_16 = ((c.m.r32(sym::gOtZ)).wrapping_add((((4095i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_14) as u32)).wrapping_mul(8));
        } else {
            l_e_16 = ((c.m.r32(sym::gOtZ)).wrapping_add((((p_z_11) as u32)).wrapping_mul(16))).wrapping_add((((l_l_14) as u32)).wrapping_mul(8));
        }
    }
    { let v_ = l_p_15; let a_ = (c.m.r32((l_e_16).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = l_p_15; let a_ = (l_e_16).wrapping_add(4); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "eft_c__EftSurf_QueueTri", &PROBE_N, &[], true); }
    r_
}

pub fn EftSurf_DrawTriOtClipped(c: &mut Ctx, mut p_tri_1: u32, mut p_layer_2: i32, mut p_unused_3: i32, mut p_flipZ_4: i32, mut p_tex0_5: u64, mut p_zBias_6: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_DrawTriOtClipped", &PROBE_N, &[(p_tri_1) as u32, (p_layer_2) as u32, (p_unused_3) as u32, (p_flipZ_4) as u32, (p_tex0_5) as u32, (p_zBias_6) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(1056) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 1056);
    let mut l_plane_7: u32 = 0;
    let mut l_count_8: i32 = 0;
    let mut l_i_9: i32 = 0;
    let mut l_z_10: i32 = 0;
    let r_ = 'f: {
        { let _ = { rt::memcpy(c, fp, p_tri_1, ((((3i32) as u32)).wrapping_mul(48u32)) as u32); fp }; };
        p_tri_1 = fp;
        l_plane_7 = { let a_ = ((c.m.r32(sym::gEftSurf)).wrapping_add(8)).wrapping_add(8); c.m.r32(a_) };
        l_count_8 = 3i32;
        {
            l_i_9 = 4i32;
            'l1: loop {
                if !(((((l_i_9) >= (0i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_count_8 = F::ClipPoly_ClipPlane(c, p_tri_1, l_plane_7, l_count_8);
                    { let o_ = l_plane_7; let n_ = (o_).wrapping_add(16); l_plane_7 = n_; };
                }
                { let o_ = l_i_9; let n_ = (o_).wrapping_sub(1); l_i_9 = n_; };
            }
        }
        if (((((l_count_8) != (0i32)) as i32)) != 0) {
            F::ClipPoly_ProjectCur(c, fp.wrapping_add(768), fp.wrapping_add(912), p_tri_1, l_count_8);
            {
                l_i_9 = 2i32;
                'l3: loop {
                    if !(((((l_i_9) < (l_count_8)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        l_z_10 = ((c.m.r32(((fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)).wrapping_shr((8i32) as u32);
                        if ((p_flipZ_4) != 0) {
                            l_z_10 = (4096i32).wrapping_sub(l_z_10);
                        }
                        if ((((((c.m.r32(((fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) > (16777215i32)) as i32)) != 0) {
                            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        }
                        if ((((((c.m.r32(((fp.wrapping_add(768)).wrapping_add(((((l_i_9).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) > (16777215i32)) as i32)) != 0) {
                            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add(((((l_i_9).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        }
                        if ((((((c.m.r32(((fp.wrapping_add(768)).wrapping_add((((l_i_9) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) > (16777215i32)) as i32)) != 0) {
                            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add((((l_i_9) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        }
                        F::eft_c__EftSurf_QueueTri(c, (fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(768)).wrapping_add(((((l_i_9).wrapping_sub(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(768)).wrapping_add((((l_i_9) as u32)).wrapping_mul(16)), ((p_tri_1).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(32), ((p_tri_1).wrapping_add(((((l_i_9).wrapping_sub(1i32)) as u32)).wrapping_mul(48))).wrapping_add(32), ((p_tri_1).wrapping_add((((l_i_9) as u32)).wrapping_mul(48))).wrapping_add(32), (fp.wrapping_add(912)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(912)).wrapping_add(((((l_i_9).wrapping_sub(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(912)).wrapping_add((((l_i_9) as u32)).wrapping_mul(16)), p_layer_2, (l_z_10).wrapping_sub(p_zBias_6), p_tex0_5);
                    }
                    { let o_ = l_i_9; let n_ = (o_).wrapping_add(1); l_i_9 = n_; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_DrawTriOtClipped", &PROBE_N, &[], true); }
    r_
}

pub fn EftSurf_BuildPalettes(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_BuildPalettes", &PROBE_N, &[], false); }
    let mut l_rt_1: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftSurf)) != (((0i32) as u32))) as i32)) != 0) {
        l_rt_1 = (c.m.r32(sym::gEftSurf)).wrapping_add(576);
        if (((((c.m.r32((l_rt_1).wrapping_add(4372)) as i32)) & (8i32))) != 0) {
            if ((((!(((((c.m.r32((l_rt_1).wrapping_add(4372)) as i32)) & (4i32))) != 0)) as i32)) != 0) {
                { let v_ = ((((c.m.r32((l_rt_1).wrapping_add(4368)) as i32)).wrapping_add(1i32)) & (1i32)); let a_ = (l_rt_1).wrapping_add(4368); c.m.w32(a_, (v_) as u32); };
                F::EftSurf_RenderPalettes(c);
                { let r_ = 4i32; let a_ = (l_rt_1).wrapping_add(4372); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_BuildPalettes", &PROBE_N, &[], true); }
    r_
}

pub fn EftSurf_StepAnim(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_StepAnim", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(128) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 128);
    let mut l_w_1: u32 = 0;
    let mut l_rt_2: u32 = 0;
    let mut l_frame_3: u32 = 0;
    let mut l_step_4: F32 = F32(0);
    let r_ = 'f: {
        l_w_1 = c.m.r32(sym::gEftSurf);
        l_rt_2 = (l_w_1).wrapping_add(576);
        l_frame_3 = ((l_w_1).wrapping_add(576)).wrapping_add(4288);
        l_step_4 = rt::fdiv(rt::i2f((c.m.r32(((l_rt_2).wrapping_add(4224)).wrapping_add(52)) as i32)), rt::i2f((c.m.r32((l_rt_2).wrapping_add(4356)) as i32)));
        rt::memset(c, fp, 0, 48);
        rt::memset(c, fp.wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(16), 0, 16);
        { let v_ = F32(0x41200000); c.m.w32(fp.wrapping_add(16).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(32), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(32).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(32).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x41200000); c.m.w32(fp.wrapping_add(32).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(32).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(48), 0, 48);
        rt::memset(c, fp.wrapping_add(48).wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(48).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(48).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(48).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(48).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(48).wrapping_add(16), 0, 16);
        { let v_ = F32(0x41200000); c.m.w32(fp.wrapping_add(48).wrapping_add(16).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(48).wrapping_add(16).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(48).wrapping_add(16).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(48).wrapping_add(16).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(48).wrapping_add(32), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(48).wrapping_add(32).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x41200000); c.m.w32(fp.wrapping_add(48).wrapping_add(32).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(48).wrapping_add(32).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(48).wrapping_add(32).wrapping_add(12), (v_).0); }
        { let v_ = F32(c.m.r32(((l_rt_2).wrapping_add(4224)).wrapping_add(32))); let a_ = (fp.wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((((l_rt_2).wrapping_add(4224)).wrapping_add(32)).wrapping_add(4))); let a_ = (fp.wrapping_add(112)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((((l_rt_2).wrapping_add(4224)).wrapping_add(32)).wrapping_add(8))); let a_ = (fp.wrapping_add(112)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(112)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let s_ = fp.wrapping_add(112); let d_ = fp.wrapping_add(96); rt::memcpy(c, d_, s_, 16); };
        F::EftMath_CalcTangentFrame(c, l_frame_3, fp, fp.wrapping_add(48));
        { let v_ = F::Vec3_Dot(c, fp.wrapping_add(96), (l_frame_3).wrapping_add((((0i32) as u32)).wrapping_mul(16))); let a_ = (l_rt_2).wrapping_add(3648); c.m.w32(a_, (v_).0); };
        { let v_ = F::Vec3_Dot(c, fp.wrapping_add(96), (((l_w_1).wrapping_add(576)).wrapping_add(4288)).wrapping_add((((1i32) as u32)).wrapping_mul(16))); let a_ = ((l_rt_2).wrapping_add(3648)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F::Vec3_Dot(c, fp.wrapping_add(96), (((l_w_1).wrapping_add(576)).wrapping_add(4288)).wrapping_add((((2i32) as u32)).wrapping_mul(16))); let a_ = ((l_rt_2).wrapping_add(3648)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((l_rt_2).wrapping_add(3648)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let r_ = l_step_4; let a_ = (l_rt_2).wrapping_add(4376); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if (((rt::fge(F32(c.m.r32((l_rt_2).wrapping_add(4376))), F32(0x3f800000)) as i32)) != 0) {
            { let v_ = F32(0x00000000); let a_ = (l_rt_2).wrapping_add(4376); c.m.w32(a_, (v_).0); };
            { let a_ = (l_rt_2).wrapping_add(4352); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        if ((((((c.m.r32((l_rt_2).wrapping_add(4352)) as i32)) > (((c.m.r32((l_rt_2).wrapping_add(4356)) as i32)).wrapping_sub(1i32))) as i32)) != 0) {
            { let v_ = 0i32; let a_ = (l_rt_2).wrapping_add(4352); c.m.w32(a_, (v_) as u32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_StepAnim", &PROBE_N, &[], true); }
    r_
}

pub fn EftSurf_RenderPalettes(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_RenderPalettes", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_rt_1: u32 = 0;
    let mut l_src_2: u32 = 0;
    let mut l_dst_3: u32 = 0;
    let mut l_dst2_4: u32 = 0;
    let r_ = 'f: {
        if (((((c.m.r32(sym::gEftSurf)) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        l_rt_1 = (c.m.r32(sym::gEftSurf)).wrapping_add(576);
        if ((((!(((((c.m.r32((l_rt_1).wrapping_add(4372)) as i32)) & (8i32))) != 0)) as i32)) != 0) {
            break 'f;
        }
        l_src_2 = { let a1_ = (c.m.r32((l_rt_1).wrapping_add(4352)) as i32); F::eft_c__EftSurfRt_GetTex(c, l_rt_1, a1_) };
        { let s_ = ({ let a_ = (c.m.r32((l_src_2).wrapping_add(8))).wrapping_add(60); c.m.r32(a_) }).wrapping_add(96); let d_ = c.m.r32(((l_rt_1).wrapping_add((((0i32) as u32)).wrapping_mul(1216))).wrapping_add(1168)); rt::memcpy(c, d_, s_, 1024); };
        { let _ = { let a0_ = c.m.r32(((l_rt_1).wrapping_add((((1i32) as u32)).wrapping_mul(1216))).wrapping_add(1168)); F::memset(c, a0_, 0i32, 1024u32) }; };
        { let a0_ = c.m.r32(((l_rt_1).wrapping_add((((1i32) as u32)).wrapping_mul(1216))).wrapping_add(1168)); let a1_ = c.m.r32(((l_rt_1).wrapping_add((((0i32) as u32)).wrapping_mul(1216))).wrapping_add(1168)); let a3_ = ((rt::f2u(F32(c.m.r32((((l_rt_1).wrapping_add(4224)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))) as u8); let a4_ = ((rt::f2u(F32(c.m.r32((((l_rt_1).wrapping_add(4224)).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))) as u8); let a5_ = ((rt::f2u(F32(c.m.r32((((l_rt_1).wrapping_add(4224)).wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))) as u8); F::EftGfx_LightClutDiffuse(c, a0_, a1_, (l_rt_1).wrapping_add(3648), a3_, a4_, a5_) };
        { let _ = { let a0_ = c.m.r32(((l_rt_1).wrapping_add((((2i32) as u32)).wrapping_mul(1216))).wrapping_add(1168)); F::memset(c, a0_, 0i32, 1024u32) }; };
        F::Mtx_StoreIdentity(c, fp);
        if ((((!((F::Battle_IsSplitScreen(c)) != 0)) as i32)) != 0) {
            { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(64); F::Mtx_Copy(c, fp, a1_) };
        } else {
            { let v_ = F32(0x3f000000); let a_ = ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        }
        { let a0_ = c.m.r32(((l_rt_1).wrapping_add((((2i32) as u32)).wrapping_mul(1216))).wrapping_add(1168)); let a1_ = c.m.r32(((l_rt_1).wrapping_add((((0i32) as u32)).wrapping_mul(1216))).wrapping_add(1168)); let a5_ = F32(c.m.r32(((l_rt_1).wrapping_add(4224)).wrapping_add(48))); F::EftGfx_LightClutSpecular(c, a0_, a1_, fp, (l_rt_1).wrapping_add(4288), (l_rt_1).wrapping_add(3648), a5_) };
        { let v_ = F::EftVram_AddImage(c, l_src_2, 1i32, 0i32); let a_ = l_src_2; c.m.w64(a_, v_); };
        l_dst_3 = { let a1_ = ((c.m.r32((l_rt_1).wrapping_add(4360)) as i32)).wrapping_add((c.m.r32((l_rt_1).wrapping_add(4368)) as i32)); F::eft_c__EftSurfRt_GetTex(c, l_rt_1, a1_) };
        { let s_ = c.m.r32(((l_rt_1).wrapping_add((((1i32) as u32)).wrapping_mul(1216))).wrapping_add(1168)); let d_ = ({ let a_ = (c.m.r32((l_dst_3).wrapping_add(8))).wrapping_add(60); c.m.r32(a_) }).wrapping_add(96); rt::memcpy(c, d_, s_, 1024); };
        { let v_ = ((((c.m.r64(l_src_2)) & ((!((16383u64).wrapping_shl((37i32) as u32)))))) | ((F::EftVram_AddClut(c, l_dst_3)).wrapping_shl((37i32) as u32))); let a_ = l_dst_3; c.m.w64(a_, v_); };
        l_dst2_4 = { let a1_ = ((c.m.r32((l_rt_1).wrapping_add(4364)) as i32)).wrapping_add((c.m.r32((l_rt_1).wrapping_add(4368)) as i32)); F::eft_c__EftSurfRt_GetTex(c, l_rt_1, a1_) };
        { let s_ = c.m.r32(((l_rt_1).wrapping_add((((2i32) as u32)).wrapping_mul(1216))).wrapping_add(1168)); let d_ = ({ let a_ = (c.m.r32((l_dst2_4).wrapping_add(8))).wrapping_add(60); c.m.r32(a_) }).wrapping_add(96); rt::memcpy(c, d_, s_, 1024); };
        { let v_ = ((((c.m.r64(l_src_2)) & ((!((16383u64).wrapping_shl((37i32) as u32)))))) | ((F::EftVram_AddClut(c, l_dst2_4)).wrapping_shl((37i32) as u32))); let a_ = l_dst2_4; c.m.w64(a_, v_); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_RenderPalettes", &PROBE_N, &[], true); }
    r_
}
