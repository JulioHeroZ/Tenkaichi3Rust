#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EftCharaFx_Stop(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharaFx_Stop", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_task_2: u32 = 0;
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    l_task_2 = F::EftCharSlot_Get3(c, p_objId_1);
    if (((((l_task_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((l_task_2).wrapping_add(56));
    if ((((!((((c.m.r32((l_w_3).wrapping_add(2096))) & (((1i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let r_ = ((2i32) as u32); let a_ = (l_w_3).wrapping_add(2096); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharaFx_Stop", &PROBE_N, &[], true); }
    r_
}

pub fn EftStruggle_Start(c: &mut Ctx, mut p_pos_1: u32, mut p_a_2: u32, mut p_b_3: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStruggle_Start", &PROBE_N, &[(p_pos_1) as u32, (p_a_2) as u32, (p_b_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(832) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 832);
    let mut l_d_4: F32 = F32(0);
    let r_ = 'f: {
        { let _ = F::memset(c, fp, 0i32, 832u32); };
        F::Vec4_Copy(c, fp, p_pos_1);
        if ((((((c.m.r32(p_a_2) as i32)) == (0i32)) as i32)) != 0) {
            { let s_ = p_a_2; let d_ = ((fp).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(400)); rt::memcpy(c, d_, s_, 400); };
            { let s_ = p_b_3; let d_ = ((fp).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(400)); rt::memcpy(c, d_, s_, 400); };
        } else {
            { let s_ = p_b_3; let d_ = ((fp).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(400)); rt::memcpy(c, d_, s_, 400); };
            { let s_ = p_a_2; let d_ = ((fp).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(400)); rt::memcpy(c, d_, s_, 400); };
        }
        { let v_ = F32(0x3f800000); let a_ = ((((fp).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(400))).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((((fp).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(400))).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        l_d_4 = F::Vec3_Dist(c, (((fp).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(400))).wrapping_add(16), (((fp).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(400))).wrapping_add(16));
        { let v_ = rt::fdiv(F::Vec3_Dist(c, (((fp).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(400))).wrapping_add(16), ((((fp).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(400))).wrapping_add(16)).wrapping_add(16)), l_d_4); let a_ = (fp).wrapping_add(16); c.m.w32(a_, (v_).0); };
        { let _ = { let a0_ = { let a_ = c.m.r32(sym::gEftStruggle); c.m.r32(a_) }; F::BtlTaskList_AddTail(c, a0_, sym::gEftStruggleClass, fp) }; };
        break 'f F32(c.m.r32((fp).wrapping_add(16)));
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftStruggle_Start", &PROBE_N, &[], true); }
    r_
}

pub fn EftStruggle_IsActive(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStruggle_IsActive", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftStruggle)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f ((({ let a_ = ({ let a_ = c.m.r32(sym::gEftStruggle); c.m.r32(a_) }).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStruggle_IsActive", &PROBE_N, &[], true); }
    r_
}

pub fn EftStruggle_End(c: &mut Ctx, mut p_winner_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStruggle_End", &PROBE_N, &[(p_winner_1) as u32], false); }
    let mut l_task_2: u32 = 0;
    let mut l_w_3: u32 = 0;
    let mut l_layer_4: i32 = 0;
    let mut l_loser_5: i32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftStruggle)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    l_task_2 = { let a_ = ({ let a_ = c.m.r32(sym::gEftStruggle); c.m.r32(a_) }).wrapping_add(4); c.m.r32(a_) };
    if (((((l_task_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if ((((((((c.m.r8(l_task_2)) as i32)) & (1i32))) as u8)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = c.m.r32((l_task_2).wrapping_add(40)); c.m.r32(a_) }) != (fid::EftStruggle_Update)) as i32)) != 0) {
        break 'f;
    }
    l_w_3 = c.m.r32((l_task_2).wrapping_add(56));
    if (((((c.m.r32((l_w_3).wrapping_add(40)) as i32)) & (1i32))) != 0) {
        break 'f;
    }
    { let r_ = 1i32; let a_ = (l_w_3).wrapping_add(40); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    if (((((p_winner_1) == ((1i32).wrapping_neg())) as i32)) != 0) {
        F::EftTechEvt_RequestExpire(c, 0i32);
        F::EftTechEvt_RequestExpire(c, 1i32);
        F::BtlScene_Reset(c, 2i32);
        F::BtlScene_Reset(c, 3i32);
    } else {
        if (((((p_winner_1) == (0i32)) as i32)) != 0) {
            l_loser_5 = 1i32;
            l_layer_4 = 3i32;
        } else {
            l_loser_5 = 0i32;
            l_layer_4 = 2i32;
        }
        F::EftTechEvt_RequestExpire(c, l_loser_5);
        F::BtlScene_Reset(c, l_layer_4);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStruggle_End", &PROBE_N, &[], true); }
    r_
}

pub fn EftStruggle_SetBias(c: &mut Ctx, mut p_bias_1: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStruggle_SetBias", &PROBE_N, &[(p_bias_1).0], false); }
    let mut l_task_2: u32 = 0;
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftStruggle)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    l_task_2 = { let a_ = ({ let a_ = c.m.r32(sym::gEftStruggle); c.m.r32(a_) }).wrapping_add(4); c.m.r32(a_) };
    if (((((l_task_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if ((((((((c.m.r8(l_task_2)) as i32)) & (1i32))) as u8)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = c.m.r32((l_task_2).wrapping_add(40)); c.m.r32(a_) }) != (fid::EftStruggle_Update)) as i32)) != 0) {
        break 'f;
    }
    l_w_3 = c.m.r32((l_task_2).wrapping_add(56));
    { let v_ = p_bias_1; let a_ = (l_w_3).wrapping_add(32); c.m.w32(a_, (v_).0); };
    if (((rt::fgt(p_bias_1, F32(0x3f800000)) as i32)) != 0) {
        { let v_ = F32(0x3f800000); let a_ = (l_w_3).wrapping_add(32); c.m.w32(a_, (v_).0); };
    }
    if (((rt::flt(F32(c.m.r32((l_w_3).wrapping_add(32))), F32(0x00000000)) as i32)) != 0) {
        { let v_ = F32(0x00000000); let a_ = (l_w_3).wrapping_add(32); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStruggle_SetBias", &PROBE_N, &[], true); }
    r_
}

pub fn EftStruggle_GetBias(c: &mut Ctx) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStruggle_GetBias", &PROBE_N, &[], false); }
    let mut l_task_1: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftStruggle)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f F32(0x00000000);
    }
    l_task_1 = { let a_ = ({ let a_ = c.m.r32(sym::gEftStruggle); c.m.r32(a_) }).wrapping_add(4); c.m.r32(a_) };
    if (((((l_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f F32(0x00000000);
    }
    if ((((((((c.m.r8(l_task_1)) as i32)) & (1i32))) as u8)) != 0) {
        break 'f F32(0x00000000);
    }
    if ((((({ let a_ = c.m.r32((l_task_1).wrapping_add(40)); c.m.r32(a_) }) != (fid::EftStruggle_Update)) as i32)) != 0) {
        break 'f F32(0x00000000);
    }
    break 'f { let a_ = (c.m.r32((l_task_1).wrapping_add(56))).wrapping_add(32); F32(c.m.r32(a_)) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStruggle_GetBias", &PROBE_N, &[], true); }
    r_
}

pub fn EftStruggle_GetPos(c: &mut Ctx, mut p_out_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStruggle_GetPos", &PROBE_N, &[(p_out_1) as u32], false); }
    let mut l_task_2: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftStruggle)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    l_task_2 = { let a_ = ({ let a_ = c.m.r32(sym::gEftStruggle); c.m.r32(a_) }).wrapping_add(4); c.m.r32(a_) };
    if (((((l_task_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if ((((((((c.m.r8(l_task_2)) as i32)) & (1i32))) as u8)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = c.m.r32((l_task_2).wrapping_add(40)); c.m.r32(a_) }) != (fid::EftStruggle_Update)) as i32)) != 0) {
        break 'f;
    }
    { let a1_ = c.m.r32((l_task_2).wrapping_add(56)); F::Vec4_Copy(c, p_out_1, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStruggle_GetPos", &PROBE_N, &[], true); }
    r_
}

pub fn EftStruggle_SetPos(c: &mut Ctx, mut p_pos_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStruggle_SetPos", &PROBE_N, &[(p_pos_1) as u32], false); }
    let mut l_task_2: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftStruggle)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    l_task_2 = { let a_ = ({ let a_ = c.m.r32(sym::gEftStruggle); c.m.r32(a_) }).wrapping_add(4); c.m.r32(a_) };
    if (((((l_task_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if ((((((((c.m.r8(l_task_2)) as i32)) & (1i32))) as u8)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = c.m.r32((l_task_2).wrapping_add(40)); c.m.r32(a_) }) != (fid::EftStruggle_Update)) as i32)) != 0) {
        break 'f;
    }
    { let a0_ = c.m.r32((l_task_2).wrapping_add(56)); F::Vec4_Copy(c, a0_, p_pos_1) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStruggle_SetPos", &PROBE_N, &[], true); }
    r_
}

pub fn EftStruggle_GetCamDist(c: &mut Ctx, mut p_objId_1: i32, mut p_mode_2: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStruggle_GetCamDist", &PROBE_N, &[(p_objId_1) as u32, (p_mode_2) as u32], false); }
    let mut l_ret_3: F32 = F32(0);
    let mut l_power_4: F32 = F32(0);
    let mut l_scale_5: F32 = F32(0);
    let r_ = 'f: {
    l_ret_3 = F32(0x00000000);
    l_power_4 = F32(0x3f800000);
    if (((((c.m.r32(sym::gEftStruggle)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f l_ret_3;
    }
    if ((((({ let a_ = ((c.m.r32(sym::gEftStruggle)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); (c.m.r32(a_) as i32) }) == (p_objId_1)) as i32)) != 0) {
        l_power_4 = { let a_ = (((c.m.r32(sym::gEftStruggle)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add(4); F32(c.m.r32(a_)) };
    }
    if ((((({ let a_ = ((c.m.r32(sym::gEftStruggle)).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); (c.m.r32(a_) as i32) }) == (p_objId_1)) as i32)) != 0) {
        l_power_4 = { let a_ = (((c.m.r32(sym::gEftStruggle)).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add(4); F32(c.m.r32(a_)) };
    }
    if (((rt::fle(l_power_4, F32(0x3f000000)) as i32)) != 0) {
        l_power_4 = F32(0x3f000000);
    }
    l_scale_5 = F::BtlScene_GetCharScale(c, p_objId_1);
    's1: {
        let sv6: i32 = p_mode_2;
        match sv6 {
            0i32 => {
                l_ret_3 = rt::fmul(rt::fadd({ let a_ = (c.m.r32(sym::gEftStruggle)).wrapping_add(20); F32(c.m.r32(a_)) }, rt::fmul(l_power_4, F32(0x40a00000))), l_scale_5);
                break 's1;
            }
            1i32 => {
                break 's1;
            }
            2i32 => {
                break 'f l_ret_3;
            }
            _ => {
                l_ret_3 = F32(0x00000000);
                break 's1;
            }
        }
    }
    break 'f l_ret_3;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStruggle_GetCamDist", &PROBE_N, &[], true); }
    r_
}

pub fn EftStruggle_GetMidDist(c: &mut Ctx) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStruggle_GetMidDist", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftStruggle)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f F32(0x00000000);
    }
    break 'f rt::fadd({ let a_ = (c.m.r32(sym::gEftStruggle)).wrapping_add(24); F32(c.m.r32(a_)) }, rt::fmul({ let a_ = (c.m.r32(sym::gEftStruggle)).wrapping_add(28); F32(c.m.r32(a_)) }, F32(0x41a00000)));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStruggle_GetMidDist", &PROBE_N, &[], true); }
    r_
}

pub fn EftStruggle_SetPower(c: &mut Ctx, mut p_side_1: i32, mut p_power_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStruggle_SetPower", &PROBE_N, &[(p_side_1) as u32, (p_power_2).0], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftStruggle)) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = p_power_2; let a_ = (((c.m.r32(sym::gEftStruggle)).wrapping_add(4)).wrapping_add((((p_side_1) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_side_1; let a_ = ((c.m.r32(sym::gEftStruggle)).wrapping_add(4)).wrapping_add((((p_side_1) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_) as u32); };
        { let v_ = (if (((rt::fgt({ let a_ = (((c.m.r32(sym::gEftStruggle)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add(4); F32(c.m.r32(a_)) }, { let a_ = (((c.m.r32(sym::gEftStruggle)).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add(4); F32(c.m.r32(a_)) }) as i32)) != 0) { { let a_ = (((c.m.r32(sym::gEftStruggle)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add(4); F32(c.m.r32(a_)) } } else { { let a_ = (((c.m.r32(sym::gEftStruggle)).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add(4); F32(c.m.r32(a_)) } }); let a_ = (c.m.r32(sym::gEftStruggle)).wrapping_add(28); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStruggle_SetPower", &PROBE_N, &[], true); }
    r_
}

pub fn EftStruggle_GetPower(c: &mut Ctx, mut p_side_1: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStruggle_GetPower", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftStruggle)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f F32(0x00000000);
    }
    break 'f { let a_ = (((c.m.r32(sym::gEftStruggle)).wrapping_add(4)).wrapping_add((((p_side_1) as u32)).wrapping_mul(8))).wrapping_add(4); F32(c.m.r32(a_)) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStruggle_GetPower", &PROBE_N, &[], true); }
    r_
}

pub fn EftStruggleMgr_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStruggleMgr_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((32u32) as i32)) }; let a_ = sym::gEftStruggle; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gEftStruggle); F::memset(c, a0_, 0i32, 32u32) }; };
    { let v_ = F32(0x41a00000); let a_ = (c.m.r32(sym::gEftStruggle)).wrapping_add(20); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x42200000); let a_ = (c.m.r32(sym::gEftStruggle)).wrapping_add(24); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = (c.m.r32(sym::gEftStruggle)).wrapping_add(28); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = (((c.m.r32(sym::gEftStruggle)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = 0i32; let a_ = ((c.m.r32(sym::gEftStruggle)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_) as u32); };
    { let v_ = F32(0x3f800000); let a_ = (((c.m.r32(sym::gEftStruggle)).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = 1i32; let a_ = ((c.m.r32(sym::gEftStruggle)).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_) as u32); };
    { let v_ = F::BtlTask_CreateChildList(c, p_task_1, ((((1i32) as i64 as u64)) as u32), ((((((896u32) as i32)) as i64 as u64)) as u32)); let a_ = c.m.r32(sym::gEftStruggle); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStruggleMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftStruggleMgr_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStruggleMgr_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftStruggle)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftStruggle); F::BtlPool_Free(c, a0_, a1_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gEftStruggle; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStruggleMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftStruggleMgr_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStruggleMgr_Update", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStruggleMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftStruggle_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStruggle_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_w_3: u32 = 0;
    let mut l_r0_4: u32 = 0;
    let mut l_r1_5: u32 = 0;
    let mut l_p0_6: F32 = F32(0);
    let mut l_p1_7: F32 = F32(0);
    let mut l_one_8: F32 = F32(0);
    let mut l_s0_9: u32 = 0;
    let mut l_s1_10: u32 = 0;
    let mut l_d0_11: u32 = 0;
    let mut l_d1_12: u32 = 0;
    let mut l_s0_13: u32 = 0;
    let mut l_s1_14: u32 = 0;
    let mut l_d0_15: u32 = 0;
    let mut l_d1_16: u32 = 0;
    let r_ = 'f: {
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        { let _ = F::memset(c, l_w_3, 0i32, 896u32); };
        { let s_ = p_arg_2; let d_ = (l_w_3).wrapping_add(48); rt::memcpy(c, d_, s_, 832); };
        { let v_ = F32(c.m.r32(((l_w_3).wrapping_add(48)).wrapping_add(16))); let a_ = (l_w_3).wrapping_add(32); c.m.w32(a_, (v_).0); };
        l_r0_4 = (((l_w_3).wrapping_add(48)).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(400));
        l_r1_5 = (((l_w_3).wrapping_add(48)).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(400));
        l_p0_6 = F::EftStruggle_GetRecRadius(c, l_r0_4);
        l_p1_7 = F::EftStruggle_GetRecRadius(c, l_r1_5);
        if (((rt::flt(l_p1_7, l_p0_6) as i32)) != 0) {
            l_s0_9 = c.m.r32((l_r0_4).wrapping_add(100));
            l_s1_10 = c.m.r32((l_r1_5).wrapping_add(100));
            l_d0_11 = c.m.r32((l_s0_9).wrapping_add(36));
            l_d1_12 = c.m.r32((l_s1_10).wrapping_add(36));
            { let v_ = F32(c.m.r32((l_d0_11).wrapping_add(136))); let a_ = (((c.m.r32(sym::gEftStruggle)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = (c.m.r32(l_r0_4) as i32); let a_ = ((c.m.r32(sym::gEftStruggle)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_) as u32); };
            { let v_ = F32(c.m.r32((l_d1_12).wrapping_add(136))); let a_ = (((c.m.r32(sym::gEftStruggle)).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = (c.m.r32(l_r1_5) as i32); let a_ = ((c.m.r32(sym::gEftStruggle)).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (if (((rt::fgt({ let a_ = (((c.m.r32(sym::gEftStruggle)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add(4); F32(c.m.r32(a_)) }, { let a_ = (((c.m.r32(sym::gEftStruggle)).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add(4); F32(c.m.r32(a_)) }) as i32)) != 0) { { let a_ = (((c.m.r32(sym::gEftStruggle)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add(4); F32(c.m.r32(a_)) } } else { { let a_ = (((c.m.r32(sym::gEftStruggle)).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add(4); F32(c.m.r32(a_)) } }); let a_ = (c.m.r32(sym::gEftStruggle)).wrapping_add(28); c.m.w32(a_, (v_).0); };
        } else {
            l_s0_13 = c.m.r32((l_r0_4).wrapping_add(100));
            l_s1_14 = c.m.r32((l_r1_5).wrapping_add(100));
            l_d0_15 = c.m.r32((l_s0_13).wrapping_add(36));
            l_d1_16 = c.m.r32((l_s1_14).wrapping_add(36));
            { let v_ = F32(c.m.r32((l_d0_15).wrapping_add(136))); let a_ = (((c.m.r32(sym::gEftStruggle)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = (c.m.r32(l_r0_4) as i32); let a_ = ((c.m.r32(sym::gEftStruggle)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_) as u32); };
            { let v_ = F32(c.m.r32((l_d1_16).wrapping_add(136))); let a_ = (((c.m.r32(sym::gEftStruggle)).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = (c.m.r32(l_r1_5) as i32); let a_ = ((c.m.r32(sym::gEftStruggle)).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (if (((rt::fgt({ let a_ = (((c.m.r32(sym::gEftStruggle)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add(4); F32(c.m.r32(a_)) }, { let a_ = (((c.m.r32(sym::gEftStruggle)).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add(4); F32(c.m.r32(a_)) }) as i32)) != 0) { { let a_ = (((c.m.r32(sym::gEftStruggle)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add(4); F32(c.m.r32(a_)) } } else { { let a_ = (((c.m.r32(sym::gEftStruggle)).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add(4); F32(c.m.r32(a_)) } }); let a_ = (c.m.r32(sym::gEftStruggle)).wrapping_add(28); c.m.w32(a_, (v_).0); };
        }
        l_one_8 = F32(0x3f800000);
        { let v_ = rt::fmul(rt::fadd(F::BtlCharApi_GetClashBias(c), l_one_8), F32(0x3f000000)); let a_ = (l_w_3).wrapping_add(32); c.m.w32(a_, (v_).0); };
        if (((rt::flt(F32(c.m.r32((l_w_3).wrapping_add(32))), F32(0x00000000)) as i32)) != 0) {
            { let v_ = F32(0x00000000); let a_ = (l_w_3).wrapping_add(32); c.m.w32(a_, (v_).0); };
        }
        if (((rt::fgt(F32(c.m.r32((l_w_3).wrapping_add(32))), l_one_8) as i32)) != 0) {
            { let v_ = l_one_8; let a_ = (l_w_3).wrapping_add(32); c.m.w32(a_, (v_).0); };
        }
        { let a3_ = F32(c.m.r32((l_w_3).wrapping_add(32))); F::Vec3_Lerp(c, l_w_3, ((((l_w_3).wrapping_add(48)).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(400))).wrapping_add(16), ((((l_w_3).wrapping_add(48)).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(400))).wrapping_add(16), a3_) };
        { let _ = F::memset(c, fp, 0i32, 48u32); };
        F::Vec4_Copy(c, fp, l_w_3);
        F::Vec3_Normalize(c, (fp).wrapping_add(16), (((((l_w_3).wrapping_add(48)).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(400))).wrapping_add(16)).wrapping_add(48));
        { let v_ = l_one_8; let a_ = (fp).wrapping_add(36); c.m.w32(a_, (v_).0); };
        { let v_ = 0i32; let a_ = (fp).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        { let v_ = F::EftClashSpark_Create(c, fp); let a_ = (l_w_3).wrapping_add(880); c.m.w32(a_, v_); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftStruggle_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftStruggle_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStruggle_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    { let _ = { let a0_ = c.m.r32((l_w_2).wrapping_add(880)); F::EftClashSpark_Stop(c, a0_) }; };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStruggle_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftStruggle_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStruggle_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add((((rt::div_i32(6640i32, 8i32)) as u32)).wrapping_mul(8)); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
        F::EftStruggle_Step(c, p_task_1);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStruggle_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftStruggle_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStruggle_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStruggle_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftStruggle_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStruggle_Draw", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStruggle_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftStruggle_GetRecRadius(c: &mut Ctx, mut p_rec_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStruggle_GetRecRadius", &PROBE_N, &[(p_rec_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (c.m.r32((p_rec_1).wrapping_add(112)) as i32);
        match sv2 {
            0i32 => {
                break 'f { let a_ = (c.m.r32((p_rec_1).wrapping_add(124))).wrapping_add(16); F32(c.m.r32(a_)) };
            }
            1i32 => {
                break 'f { let a_ = (c.m.r32((p_rec_1).wrapping_add(124))).wrapping_add(32); F32(c.m.r32(a_)) };
            }
            _ => {
            }
        }
    }
    break 'f F32(0x00000000);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStruggle_GetRecRadius", &PROBE_N, &[], true); }
    r_
}

pub fn EftStruggle_Step(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStruggle_Step", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_2: u32 = 0;
    let mut l_r0_3: u32 = 0;
    let mut l_r1_4: u32 = 0;
    let mut l_one_5: F32 = F32(0);
    let mut l_t_6: F32 = F32(0);
    let mut l_p0_7: F32 = F32(0);
    let mut l_p1_8: F32 = F32(0);
    let mut l_k_9: F32 = F32(0);
    let mut l_tmp_10: F32 = F32(0);
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        { let a_ = (l_w_2).wrapping_add(36); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        if ((((((((!((F::BtlCharApi_IsInClashA(c, 0i32)) != 0)) as i32)) != 0) || ((((!((F::BtlCharApi_IsInClashA(c, 1i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
            { let r_ = 1i32; let a_ = (l_w_2).wrapping_add(40); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if (((((c.m.r32((l_w_2).wrapping_add(40)) as i32)) & (1i32))) != 0) {
            F::BtlTask_SetDead(c, p_task_1);
            break 'f;
        }
        l_one_5 = F32(0x3f800000);
        { let _ = { let a0_ = c.m.r32((l_w_2).wrapping_add(880)); F::EftClashSpark_SetPos(c, a0_, l_w_2) }; };
        l_r0_3 = (((l_w_2).wrapping_add(48)).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(400));
        { let _ = { let a0_ = c.m.r32((l_w_2).wrapping_add(880)); F::EftClashSpark_SetScale(c, a0_, l_one_5) }; };
        l_r1_4 = (((l_w_2).wrapping_add(48)).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(400));
        l_t_6 = l_one_5;
        l_p0_7 = { let a_ = (((c.m.r32(sym::gEftStruggle)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add(4); F32(c.m.r32(a_)) };
        l_k_9 = F32(0x41500000);
        l_p1_8 = { let a_ = (((c.m.r32(sym::gEftStruggle)).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add(4); F32(c.m.r32(a_)) };
        l_tmp_10 = l_p0_7;
        if ((((({ let a_ = ((c.m.r32(sym::gEftStruggle)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); (c.m.r32(a_) as i32) }) != ((c.m.r32(l_r0_3) as i32))) as i32)) != 0) {
            l_p0_7 = l_p1_8;
            l_p1_8 = l_tmp_10;
        }
        { let v_ = rt::fmul(rt::fadd(F::BtlCharApi_GetClashBias(c), l_t_6), F32(0x3f000000)); let a_ = (l_w_2).wrapping_add(32); c.m.w32(a_, (v_).0); };
        if (((rt::flt(F32(c.m.r32((l_w_2).wrapping_add(32))), F32(0x00000000)) as i32)) != 0) {
            { let v_ = F32(0x00000000); let a_ = (l_w_2).wrapping_add(32); c.m.w32(a_, (v_).0); };
        }
        if (((rt::fgt(F32(c.m.r32((l_w_2).wrapping_add(32))), l_t_6) as i32)) != 0) {
            { let v_ = l_t_6; let a_ = (l_w_2).wrapping_add(32); c.m.w32(a_, (v_).0); };
        }
        { let a3_ = F32(c.m.r32((l_w_2).wrapping_add(32))); F::Vec3_Lerp(c, l_w_2, (l_r1_4).wrapping_add(16), (l_r0_3).wrapping_add(16), a3_) };
        { let v_ = l_one_5; let a_ = (l_w_2).wrapping_add(12); c.m.w32(a_, (v_).0); };
        l_t_6 = { let a_ = ({ let a_ = (c.m.r32((l_r0_3).wrapping_add(100))).wrapping_add(36); c.m.r32(a_) }).wrapping_add(48); F32(c.m.r32(a_)) };
        l_t_6 = rt::fmul(rt::fmul(l_p0_7, l_t_6), l_k_9);
        F::Vec4_Sub(c, fp, (l_r0_3).wrapping_add(16), l_w_2);
        F::Vec3_Normalize(c, fp, fp);
        F::Vec3_Scale(c, fp, fp, l_t_6);
        { let a0_ = (c.m.r32((l_r0_3).wrapping_add(96))).wrapping_add(16); F::Vec3_Add(c, a0_, l_w_2, fp) };
        l_t_6 = { let a_ = ({ let a_ = (c.m.r32((l_r1_4).wrapping_add(100))).wrapping_add(36); c.m.r32(a_) }).wrapping_add(48); F32(c.m.r32(a_)) };
        l_t_6 = rt::fmul(rt::fmul(l_p1_8, l_t_6), l_k_9);
        F::Vec4_Sub(c, fp, (l_r1_4).wrapping_add(16), l_w_2);
        F::Vec3_Normalize(c, fp, fp);
        F::Vec3_Scale(c, fp, fp, l_t_6);
        { let a0_ = (c.m.r32((l_r1_4).wrapping_add(96))).wrapping_add(16); F::Vec3_Add(c, a0_, l_w_2, fp) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftStruggle_Step", &PROBE_N, &[], true); }
    r_
}

pub fn EftClashSpark_SpawnParts(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32, mut p_set_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftClashSpark_SpawnParts", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32, (p_set_3) as u32], false); }
    let mut l_w_4: u32 = 0;
    let mut l_part_5: i32 = 0;
    let mut l_sub_6: i32 = 0;
    let mut l_def_7: u32 = 0;
    let mut l_f_8: i32 = 0;
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
                                l_f_8 = { let a5_ = ((((c.m.r32((l_w_4).wrapping_add(1344))) & (((2i32) as u32)))) as i32); let a6_ = ((((c.m.r32((l_w_4).wrapping_add(1344))) & (((8i32) as u32)))) as i32); let a7_ = ((c.m.r8((l_w_4).wrapping_add(1348))) as i32); F::EftEmit_GetFlagsFromMask(c, p_set_3, (l_w_4).wrapping_add(36), p_objId_1, l_part_5, l_sub_6, a5_, a6_, a7_) };
                                if ((((c.m.r32((l_w_4).wrapping_add(1344))) & (((16i32) as u32)))) != 0) {
                                    l_f_8 = 2i32;
                                }
                                if (((((l_f_8) != (0i32)) as i32)) != 0) {
                                    { let a11_ = F32(c.m.r32((l_w_4).wrapping_add(1352))); F::EftEmit_Spawn(c, p_set_3, (l_w_4).wrapping_add(36), (l_w_4).wrapping_add(752), (l_w_4).wrapping_add(16), l_w_4, p_objId_1, 0i32, 2i32, l_part_5, l_sub_6, l_f_8, a11_) };
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
    if rt::probing() { crate::host::harness::probe(c, "EftClashSpark_SpawnParts", &PROBE_N, &[], true); }
    r_
}

pub fn EftClashSpark_GetSet(c: &mut Ctx, mut p_kind_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftClashSpark_GetSet", &PROBE_N, &[(p_kind_1) as u32], false); }
    let r_ = 'f: {
    break 'f c.m.r32(sym::gEftClashSparkSet);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftClashSpark_GetSet", &PROBE_N, &[], true); }
    r_
}

pub fn EftClashSpark_GetNodeMask(c: &mut Ctx, mut p_kind_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftClashSpark_GetNodeMask", &PROBE_N, &[(p_kind_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = ((1i32) as u8); let a_ = (fp).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
        break 'f ((c.m.r8((fp).wrapping_add(((p_kind_1) as u32)))) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftClashSpark_GetNodeMask", &PROBE_N, &[], true); }
    r_
}

pub fn EftClashSpark_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftClashSpark_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let _ = F::memset(c, l_w_3, 0i32, 1376u32); };
    { let r_ = ((1i32) as u32); let a_ = (l_w_3).wrapping_add(1344); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    { let v_ = (((c.m.r32((p_arg_2).wrapping_add(32)) as i32)) as u8); let a_ = (l_w_3).wrapping_add(1349); c.m.w8(a_, v_); };
    { let v_ = (((c.m.r32((p_arg_2).wrapping_add(40)) as i32)) as u8); let a_ = (l_w_3).wrapping_add(1350); c.m.w8(a_, v_); };
    F::Vec4_Copy(c, (l_w_3).wrapping_add(16), p_arg_2);
    F::Vec4_Copy(c, l_w_3, (p_arg_2).wrapping_add(16));
    { let v_ = F32(0x3f800000); let a_ = (l_w_3).wrapping_add(12); c.m.w32(a_, (v_).0); };
    F::Vec3_Normalize(c, l_w_3, l_w_3);
    { let v_ = F32(c.m.r32((p_arg_2).wrapping_add(36))); let a_ = (l_w_3).wrapping_add(1352); c.m.w32(a_, (v_).0); };
    { let v_ = { let a0_ = ((c.m.r8((l_w_3).wrapping_add(1349))) as i32); F::EftClashSpark_GetSet(c, a0_) }; let a_ = (l_w_3).wrapping_add(32); c.m.w32(a_, v_); };
    { let a0_ = c.m.r32((l_w_3).wrapping_add(32)); F::EftEmit_InitState(c, a0_, (l_w_3).wrapping_add(36)) };
    { let v_ = rt::i2f({ let a0_ = c.m.r32((l_w_3).wrapping_add(32)); F::EftEmit_GetEndFrames(c, a0_) }); let a_ = (l_w_3).wrapping_add(1364); c.m.w32(a_, (v_).0); };
    { let v_ = (({ let a0_ = ((c.m.r8((l_w_3).wrapping_add(1349))) as i32); F::EftClashSpark_GetNodeMask(c, a0_) }) as u8); let a_ = (l_w_3).wrapping_add(1348); c.m.w8(a_, v_); };
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (6i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((((c.m.r8((l_w_3).wrapping_add(1348))) as i32)).wrapping_shr((l_i_4) as u32)) & (1i32))) != 0) {
                    F::EftEmit_SetNode(c, (l_w_3).wrapping_add(752), l_i_4, (1i32).wrapping_neg(), (l_w_3).wrapping_add(16));
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftClashSpark_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftClashSpark_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftClashSpark_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    { let a0_ = c.m.r32((l_w_2).wrapping_add(32)); F::EftEmit_TermState(c, a0_, (l_w_2).wrapping_add(36)) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftClashSpark_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftClashSpark_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftClashSpark_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_dead_3: i32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(({ let a0_ = ((c.m.r8((l_w_2).wrapping_add(1350))) as i32); F::BtlScene_IsEffectStopped(c, a0_, 2i32) }) != 0)) as i32)) != 0) {
        { let a0_ = ((c.m.r8((l_w_2).wrapping_add(1350))) as i32); F::EftEmit_RefreshFixedNodes(c, a0_, (l_w_2).wrapping_add(752)) };
        { let a0_ = ((c.m.r8((l_w_2).wrapping_add(1350))) as i32); let a2_ = c.m.r32((l_w_2).wrapping_add(32)); F::EftClashSpark_SpawnParts(c, a0_, p_task_1, a2_) };
        if ((((c.m.r32((l_w_2).wrapping_add(1344))) & (((2i32) as u32)))) != 0) {
            { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(1360); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
        if ((((c.m.r32((l_w_2).wrapping_add(1344))) & (((4i32) as u32)))) != 0) {
            F::BtlTask_SetDead(c, p_task_1);
            l_dead_3 = 1i32;
            break 'f;
        }
        if ((((c.m.r32((l_w_2).wrapping_add(1344))) & (((2i32) as u32)))) != 0) {
            if ((((c.m.r32((l_w_2).wrapping_add(1344))) & (((8i32) as u32)))) != 0) {
                { let r_ = ((4i32) as u32); let a_ = (l_w_2).wrapping_add(1344); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
            } else {
                if (((rt::fge(F32(c.m.r32((l_w_2).wrapping_add(1360))), F32(c.m.r32((l_w_2).wrapping_add(1364)))) as i32)) != 0) {
                    { let r_ = ((4i32) as u32); let a_ = (l_w_2).wrapping_add(1344); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftClashSpark_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftClashSpark_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftClashSpark_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(({ let a0_ = ((c.m.r8((l_w_2).wrapping_add(1350))) as i32); F::BtlScene_IsEffectStopped(c, a0_, 2i32) }) != 0)) as i32)) != 0) {
        if ((((c.m.r32((l_w_2).wrapping_add(1344))) & (((32i32) as u32)))) != 0) {
            { let _ = { let a0_ = c.m.r32((l_w_2).wrapping_add(32)); F::EftEmit_UpdateAlive(c, a0_, (l_w_2).wrapping_add(36)) }; };
            { let v_ = ((0i32) as u8); let a_ = (l_w_2).wrapping_add(1348); c.m.w8(a_, v_); };
        }
        { let r_ = ((32i32) as u32); let a_ = (l_w_2).wrapping_add(1344); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftClashSpark_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftClashSpark_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftClashSpark_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!((((c.m.r32((l_w_2).wrapping_add(1344))) & (((16i32) as u32)))) != 0)) as i32)) != 0) {
        { let r_ = ((16i32) as u32); let a_ = (l_w_2).wrapping_add(1344); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        { let a0_ = c.m.r32((l_w_2).wrapping_add(32)); F::EftEmit_KillAll(c, a0_, (l_w_2).wrapping_add(36)) };
    }
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftClashSpark_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftClashSpark_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftClashSpark_Draw", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftClashSpark_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftClashSparkMgr_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftClashSparkMgr_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((800u32) as i32)) }; let a_ = sym::gEftClashSparkSet; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gEftClashSparkSet); F::memset(c, a0_, 0i32, 800u32) }; };
        { let v_ = F::BtlTask_CreateChildList(c, p_task_1, ((((3i32) as i64 as u64)) as u32), ((((((1376u32) as i32)) as i64 as u64)) as u32)); let a_ = sym::gEftClashSparkList; c.m.w32(a_, v_); };
        {
            rt::memset(c, fp, 0, 4);
            { let v_ = 616i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
            {
                l_i_2 = 0i32;
                'l1: loop {
                    if !(((((l_i_2) < (1i32)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        { let a1_ = (c.m.r32(sym::gEftClashSparkSet)).wrapping_add((((l_i_2) as u32)).wrapping_mul(800)); let a5_ = (c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32); F::EftEmit_LoadSet(c, ((0i32) as u32), a1_, ((((0i32) as i64 as u64)) as u32), ((0i32) as u32), 1i32, a5_) };
                    }
                    { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftClashSparkMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftClashSparkMgr_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftClashSparkMgr_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32(sym::gEftClashSparkSet); F::EftEmit_FreeSet(c, a0_) };
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftClashSparkSet); F::BtlPool_Free(c, a0_, a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftClashSparkSet; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftClashSparkMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftClashSparkMgr_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftClashSparkMgr_Update", &PROBE_N, &[], false); }
    let mut l_set_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_set_1 = c.m.r32(sym::gEftClashSparkSet);
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                F::EftEmit_BeginFrame(c, (l_set_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(800)));
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftClashSparkMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftClashSparkMgr_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftClashSparkMgr_Reset", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftClashSparkMgr_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftClashSpark_Create(c: &mut Ctx, p_arg_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftClashSpark_Create", &PROBE_N, &[(p_arg_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    rt::memcpy(c, fp, p_arg_1, 48);
    let r_ = 'f: {
        break 'f { let a0_ = c.m.r32(sym::gEftClashSparkList); F::BtlTaskList_AddTail(c, a0_, sym::gEftClashSparkClass, fp) };
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftClashSpark_Create", &PROBE_N, &[], true); }
    r_
}

pub fn EftClashSpark_Stop(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftClashSpark_Stop", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = c.m.r32((p_task_1).wrapping_add(40)); c.m.r32(a_) }) != (fid::EftClashSpark_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!((((c.m.r32((l_w_2).wrapping_add(1344))) & (((1i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let r_ = ((2i32) as u32); let a_ = (l_w_2).wrapping_add(1344); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftClashSpark_Stop", &PROBE_N, &[], true); }
    r_
}

pub fn EftClashSpark_SetPos(c: &mut Ctx, mut p_task_1: u32, mut p_pos_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftClashSpark_SetPos", &PROBE_N, &[(p_task_1) as u32, (p_pos_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = c.m.r32((p_task_1).wrapping_add(40)); c.m.r32(a_) }) != (fid::EftClashSpark_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!((((c.m.r32((l_w_3).wrapping_add(1344))) & (((1i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    F::Vec4_Copy(c, (l_w_3).wrapping_add(16), p_pos_2);
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftClashSpark_SetPos", &PROBE_N, &[], true); }
    r_
}

pub fn EftClashSpark_SetScale(c: &mut Ctx, mut p_task_1: u32, mut p_scale_2: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftClashSpark_SetScale", &PROBE_N, &[(p_task_1) as u32, (p_scale_2).0], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = c.m.r32((p_task_1).wrapping_add(40)); c.m.r32(a_) }) != (fid::EftClashSpark_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!((((c.m.r32((l_w_3).wrapping_add(1344))) & (((1i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let v_ = p_scale_2; let a_ = (l_w_3).wrapping_add(1352); c.m.w32(a_, (v_).0); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftClashSpark_SetScale", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharge_SpawnParts(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32, mut p_set_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharge_SpawnParts", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32, (p_set_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_4: u32 = 0;
    let mut l_part_5: i32 = 0;
    let mut l_sub_6: i32 = 0;
    let mut l_def_7: u32 = 0;
    let mut l_f_8: i32 = 0;
    let r_ = 'f: {
        l_w_4 = c.m.r32((p_task_2).wrapping_add(56));
        F::Vec4_Set(c, fp, F32(0x00000000), F32(0xbf800000), F32(0x00000000), F32(0x3f800000));
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
                                    l_f_8 = { let a5_ = ((((c.m.r32((l_w_4).wrapping_add(1344))) & (((2i32) as u32)))) as i32); let a7_ = ((c.m.r32((l_w_4).wrapping_add(1352))) as i32); F::EftEmit_GetFlagsFromMask(c, p_set_3, (l_w_4).wrapping_add(24), p_objId_1, l_part_5, l_sub_6, a5_, 0i32, a7_) };
                                    if (((((l_f_8) != (0i32)) as i32)) != 0) {
                                        { let a11_ = F32(c.m.r32((l_w_4).wrapping_add(1348))); F::EftEmit_Spawn(c, p_set_3, (l_w_4).wrapping_add(24), (l_w_4).wrapping_add(752), l_w_4, fp, p_objId_1, 0i32, 3i32, l_part_5, l_sub_6, l_f_8, a11_) };
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
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftCharge_SpawnParts", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharge_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharge_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_mgr_3: u32 = 0;
    let mut l_w_4: u32 = 0;
    let mut l_set_5: u32 = 0;
    let mut l_objId_6: i32 = 0;
    let r_ = 'f: {
    l_mgr_3 = { let a_ = (F::BtlTask_GetParent(c, p_task_1)).wrapping_add(56); c.m.r32(a_) };
    l_w_4 = c.m.r32((p_task_1).wrapping_add(56));
    l_set_5 = c.m.r32(l_mgr_3);
    { let _ = F::memset(c, l_w_4, 0i32, 1376u32); };
    l_objId_6 = (c.m.r32((p_arg_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32);
    { let r_ = ((1i32) as u32); let a_ = (l_w_4).wrapping_add(1344); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    { let v_ = l_objId_6; let a_ = (l_w_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = rt::fdiv(F::BtlCharApi_GetHeight(c, l_objId_6), F32(0x419acccc)); let a_ = (l_w_4).wrapping_add(1348); c.m.w32(a_, (v_).0); };
    if (((rt::flt(F32(c.m.r32((l_w_4).wrapping_add(1348))), F32(0x3f000000)) as i32)) != 0) {
        { let v_ = F32(0x3f000000); let a_ = (l_w_4).wrapping_add(1348); c.m.w32(a_, (v_).0); };
    }
    { let a0_ = (c.m.r32((l_w_4).wrapping_add(16)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, l_w_4) };
    { let v_ = l_set_5; let a_ = (l_w_4).wrapping_add(20); c.m.w32(a_, v_); };
    F::EftEmit_InitState(c, l_set_5, (l_w_4).wrapping_add(24));
    { let r_ = ((1i32) as u32); let a_ = (l_w_4).wrapping_add(1352); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    F::EftEmit_SetNode(c, (l_w_4).wrapping_add(752), 0i32, 3i32, ((0i32) as u32));
    { let r_ = ((32i32) as u32); let a_ = (l_w_4).wrapping_add(1344); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    { let v_ = rt::i2f({ let a0_ = c.m.r32((l_w_4).wrapping_add(20)); F::EftEmit_GetEndFrames(c, a0_) }); let a_ = (l_w_4).wrapping_add(1360); c.m.w32(a_, (v_).0); };
    { let a1_ = (if ((((((c.m.r32((p_arg_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) == (0i32)) as i32)) != 0) { 2048i32 } else { 4096i32 }); F::BtlTask_SetOwnerTag(c, p_task_1, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharge_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharge_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharge_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_owner_3: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_owner_3 = (l_w_2).wrapping_add(16);
    { let a0_ = c.m.r32((l_w_2).wrapping_add(20)); F::EftEmit_TermState(c, a0_, (l_w_2).wrapping_add(24)) };
    { let v_ = ((0i32) as u32); let a_ = (l_w_2).wrapping_add(1344); c.m.w32(a_, v_); };
    { let a0_ = (c.m.r32(l_owner_3) as i32); F::EftCharSlot_Clear0(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharge_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharge_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharge_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_owner_3: u32 = 0;
    let mut l_nodes_4: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_owner_3 = (l_w_2).wrapping_add(16);
    if ((((!(({ let a0_ = (c.m.r32(l_owner_3) as i32); F::BtlScene_IsEffectStopped(c, a0_, 3i32) }) != 0)) as i32)) != 0) {
        { let a0_ = (c.m.r32(l_owner_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, l_w_2) };
        if ((((c.m.r32((l_w_2).wrapping_add(1344))) & (((8i32) as u32)))) != 0) {
            if ((((!((((c.m.r32((l_w_2).wrapping_add(1344))) & (((16i32) as u32)))) != 0)) as i32)) != 0) {
                { let r_ = ((2i32) as u32); let a_ = (l_w_2).wrapping_add(1352); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                l_nodes_4 = (l_w_2).wrapping_add(752);
                F::EftEmit_SetNode(c, l_nodes_4, 1i32, 3i32, ((0i32) as u32));
                { let _ = { let a0_ = (c.m.r32(l_owner_3) as i32); F::ScrWarp_Spawn(c, a0_, l_w_2, F32(0x3f4ccccc), F32(0x41200000), F32(0x42480000), F32(0x41200000), F32(0x3f000000)) }; };
                { let r_ = ((16i32) as u32); let a_ = (l_w_2).wrapping_add(1344); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
            } else {
                l_nodes_4 = (l_w_2).wrapping_add(752);
            }
        } else {
            l_nodes_4 = (l_w_2).wrapping_add(752);
        }
        { let a0_ = (c.m.r32((l_w_2).wrapping_add(16)) as i32); F::EftEmit_RefreshFixedNodes(c, a0_, l_nodes_4) };
        { let a0_ = (c.m.r32((l_w_2).wrapping_add(16)) as i32); let a2_ = c.m.r32((l_w_2).wrapping_add(20)); F::EftCharge_SpawnParts(c, a0_, p_task_1, a2_) };
        if ((((c.m.r32((l_w_2).wrapping_add(1344))) & (((2i32) as u32)))) != 0) {
            { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(1356); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::fge(F32(c.m.r32((l_w_2).wrapping_add(1356))), F32(c.m.r32((l_w_2).wrapping_add(1360)))) as i32)) != 0) {
                { let r_ = ((4i32) as u32); let a_ = (l_w_2).wrapping_add(1344); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
            }
        }
    }
    if ((((c.m.r32((l_w_2).wrapping_add(1344))) & (((4i32) as u32)))) != 0) {
        F::BtlTask_SetDead(c, p_task_1);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharge_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharge_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharge_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!((((c.m.r32((l_w_2).wrapping_add(1344))) & (((128i32) as u32)))) != 0)) as i32)) != 0) {
        { let r_ = ((128i32) as u32); let a_ = (l_w_2).wrapping_add(1344); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        { let a0_ = c.m.r32((l_w_2).wrapping_add(20)); F::EftEmit_KillAll(c, a0_, (l_w_2).wrapping_add(24)) };
    }
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharge_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharge_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharge_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((c.m.r32((l_w_2).wrapping_add(1344))) & (((64i32) as u32)))) != 0) {
        if ((((!(({ let a0_ = c.m.r32((l_w_2).wrapping_add(20)); F::EftEmit_UpdateAlive(c, a0_, (l_w_2).wrapping_add(24)) }) != 0)) as i32)) != 0) {
            { let r_ = (((!(32i32))) as u32); let a_ = (l_w_2).wrapping_add(1344); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
        } else {
            { let r_ = ((32i32) as u32); let a_ = (l_w_2).wrapping_add(1344); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        }
        { let v_ = ((0i32) as u32); let a_ = (l_w_2).wrapping_add(1352); c.m.w32(a_, v_); };
    }
    { let r_ = ((64i32) as u32); let a_ = (l_w_2).wrapping_add(1344); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharge_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharge_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharge_Draw", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharge_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftChargeMgr_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChargeMgr_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_mgr_3: u32 = 0;
    let mut l_set_4: u32 = 0;
    let mut l_pack_5: u32 = 0;
    let mut l_list_6: u32 = 0;
    let r_ = 'f: {
    l_mgr_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((804u32) as i32)) }; let a_ = l_mgr_3; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(l_mgr_3); F::memset(c, a0_, 0i32, 804u32) }; };
    l_set_4 = c.m.r32(l_mgr_3);
    l_pack_5 = { let a0_ = (c.m.r32((p_arg_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::BtlScene_GetCharPackEntry(c, a0_, 2i32) };
    { let v_ = l_pack_5; let a_ = (l_set_4).wrapping_add(800); c.m.w32(a_, v_); };
    if (((((l_pack_5) != (((0i32) as u32))) as i32)) != 0) {
        F::EftEmit_LoadSet(c, ((0i32) as u32), l_set_4, ((((0i32) as i64 as u64)) as u32), l_pack_5, 2i32, 1i32);
    }
    l_list_6 = F::BtlTask_CreateChildList(c, p_task_1, ((((4i32) as i64 as u64)) as u32), ((((((1376u32) as i32)) as i64 as u64)) as u32));
    { let a0_ = (c.m.r32((p_arg_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a1_ = (c.m.r32((p_arg_2).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); F::EftChar_SetList(c, a0_, a1_, l_list_6) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChargeMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftChargeMgr_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChargeMgr_Term", &PROBE_N, &[(p_task_1) as u32], false); }
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
    if rt::probing() { crate::host::harness::probe(c, "EftChargeMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftChargeMgr_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftChargeMgr_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_mgr_2: u32 = 0;
    let r_ = 'f: {
    l_mgr_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((({ let a_ = (c.m.r32(l_mgr_2)).wrapping_add(800); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(l_mgr_2); F::EftEmit_BeginFrame(c, a0_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftChargeMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharge_Start(c: &mut Ctx, mut p_arg_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharge_Start", &PROBE_N, &[(p_arg_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_list_2: u32 = 0;
    let mut l_task_3: u32 = 0;
    let r_ = 'f: {
        { let v_ = (c.m.r32((p_arg_1).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); c.m.w32(fp, (v_) as u32); }
        if ((((({ let a0_ = (c.m.r32(fp) as i32); F::EftCharSlot_Get0(c, a0_) }) != (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        l_list_2 = { let a0_ = (c.m.r32(fp) as i32); F::EftChar_GetList(c, a0_, 0i32) };
        if (((((l_list_2) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        l_task_3 = F::BtlTaskList_AddTail(c, l_list_2, sym::gEftChargeClass, fp);
        if (((((l_task_3) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        { let a0_ = (c.m.r32(fp) as i32); F::EftCharSlot_Set0(c, a0_, l_task_3) };
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftCharge_Start", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharge_Burst(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharge_Burst", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_task_2: u32 = 0;
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    l_task_2 = F::EftCharSlot_Get0(c, p_objId_1);
    if (((((l_task_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((l_task_2).wrapping_add(56));
    if ((((!((((c.m.r32((l_w_3).wrapping_add(1344))) & (((1i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let r_ = ((8i32) as u32); let a_ = (l_w_3).wrapping_add(1344); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharge_Burst", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharge_Stop(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharge_Stop", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_task_2: u32 = 0;
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    l_task_2 = F::EftCharSlot_Get0(c, p_objId_1);
    if (((((l_task_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((l_task_2).wrapping_add(56));
    if ((((!((((c.m.r32((l_w_3).wrapping_add(1344))) & (((1i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let r_ = ((2i32) as u32); let a_ = (l_w_3).wrapping_add(1344); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharge_Stop", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharge_IsActive(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharge_IsActive", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_task_2: u32 = 0;
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    l_task_2 = F::EftCharSlot_Get0(c, p_objId_1);
    if (((((l_task_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((l_task_2).wrapping_add(56));
    if ((((c.m.r32((l_w_3).wrapping_add(1344))) & (((1i32) as u32)))) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharge_IsActive", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiBlast_AddHitRecord(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiBlast_AddHitRecord", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_rec_3: u32 = 0;
    let mut l_a_4: u32 = 0;
    let mut l_b_5: u32 = 0;
    let mut l_radius_6: F32 = F32(0);
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_rec_3 = F::EftHit_GetNew(c);
    { let v_ = p_task_1; let a_ = (l_rec_3).wrapping_add(96); c.m.w32(a_, v_); };
    { let s_ = l_w_2; let d_ = (l_rec_3).wrapping_add(16); rt::memcpy(c, d_, s_, 64); };
    { let v_ = 0i32; let a_ = (l_rec_3).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((((((c.m.r32((l_w_2).wrapping_add(1456)) as i32)).wrapping_shr((9i32) as u32)) & (1i32))) as u8); let a_ = (l_rec_3).wrapping_add(92); c.m.w8(a_, v_); };
    { let v_ = (l_w_2).wrapping_add(64); let a_ = (l_rec_3).wrapping_add(104); c.m.w32(a_, v_); };
    l_radius_6 = rt::fmul(F32(c.m.r32((l_w_2).wrapping_add(1468))), F::EftEmit_GetTrailWidth(c, (l_w_2).wrapping_add(148)));
    l_a_4 = F::EftHitArena_AllocSphere(c);
    l_b_5 = F::EftHitArena_AllocSphere(c);
    F::ColSphere_Set(c, l_a_4, (l_w_2).wrapping_add(16), l_radius_6);
    F::ColSphere_Set(c, l_b_5, (l_w_2).wrapping_add(32), l_radius_6);
    F::EftHit_SetShapeSpheres(c, l_rec_3, l_a_4, l_b_5);
    F::EftHit_Add(c, l_rec_3);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiBlast_AddHitRecord", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiBlast_Turn(c: &mut Ctx, mut p_task_1: u32, mut p_w_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiBlast_Turn", &PROBE_N, &[(p_task_1) as u32, (p_w_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_id_3: u16 = 0;
    let mut l_life_4: i32 = 0;
    let r_ = 'f: {
        F::Vec3_Lerp(c, (p_w_2).wrapping_add(16), (p_task_1).wrapping_add(16), (p_w_2).wrapping_add(32), F32(0x3f333333));
        if (((((c.m.r32((p_task_1).wrapping_add(4)) as i32)) & (64i32))) != 0) {
            { let a0_ = (((((((((c.m.r16(((p_w_2).wrapping_add(64)).wrapping_add(18)) as i16)) as i32)) ^ (1i32))) as i16)) as i32); F::BtlCharApi_GetDeflectDir(c, a0_, fp) };
            F::Vec4_Copy(c, (p_w_2).wrapping_add(48), fp);
            { let r_ = 1i32; let a_ = ((p_w_2).wrapping_add(64)).wrapping_add(18); let n_ = (((((((c.m.r16(a_) as i16)) as i32)) ^ (r_))) as i16); c.m.w16(a_, (n_) as u16); };
            { let v_ = (((((c.m.r32((p_w_2).wrapping_add(1456)) as i32)) & ((!(4i32))))) | (8i32)); let a_ = (p_w_2).wrapping_add(1456); c.m.w32(a_, (v_) as u32); };
        } else {
            if (((((c.m.r32((p_task_1).wrapping_add(4)) as i32)) & (128i32))) != 0) {
                { let a0_ = (((((((((c.m.r16(((p_w_2).wrapping_add(64)).wrapping_add(18)) as i16)) as i32)) ^ (1i32))) as i16)) as i32); F::BtlCharApi_GetDeflectDir(c, a0_, fp) };
                F::Vec4_Copy(c, (p_w_2).wrapping_add(48), fp);
                { let r_ = 1i32; let a_ = ((p_w_2).wrapping_add(64)).wrapping_add(18); let n_ = (((((((c.m.r16(a_) as i16)) as i32)) ^ (r_))) as i16); c.m.w16(a_, (n_) as u16); };
                { let v_ = (((c.m.r16(((p_w_2).wrapping_add(64)).wrapping_add(28)) as i16)) as i32); let a_ = (p_w_2).wrapping_add(1460); c.m.w32(a_, (v_) as u32); };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftKiBlast_Turn", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiBlast_SpawnParts(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32, mut p_set_3: u32, mut p_reset_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiBlast_SpawnParts", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32, (p_set_3) as u32, (p_reset_4) as u32], false); }
    let mut l_w_5: u32 = 0;
    let mut l_part_6: i32 = 0;
    let mut l_sub_7: i32 = 0;
    let mut l_def_8: u32 = 0;
    let mut l_f_9: i32 = 0;
    let r_ = 'f: {
    l_w_5 = c.m.r32((p_task_2).wrapping_add(56));
    {
        l_part_6 = 0i32;
        'l1: loop {
            if !(((((l_part_6) < (19i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((({ let a_ = c.m.r32(p_set_3); c.m.r32(a_) }) & ((((1i32).wrapping_shl((l_part_6) as u32)) as u32)))) != 0) {
                    l_def_8 = c.m.r32(((p_set_3).wrapping_add(12)).wrapping_add((((l_part_6) as u32)).wrapping_mul(16)));
                    {
                        l_sub_7 = 0i32;
                        'l3: loop {
                            if !(((((l_sub_7) < (((c.m.r8((l_def_8).wrapping_add(1))) as i32))) as i32)) != 0) { break 'l3; }
                            'c4: {
                                if (((((p_reset_4) == (0i32)) as i32)) != 0) {
                                    l_f_9 = { let a5_ = (((c.m.r32((l_w_5).wrapping_add(1456)) as i32)) & (16i32)); let a7_ = ((c.m.r32((l_w_5).wrapping_add(1476))) as i32); F::EftEmit_GetFlagsFromMask(c, p_set_3, (l_w_5).wrapping_add(148), p_objId_1, l_part_6, l_sub_7, a5_, 0i32, a7_) };
                                } else {
                                    l_f_9 = F::EftEmit_GetResetFlags(c, p_set_3, (l_w_5).wrapping_add(148), l_part_6, l_sub_7);
                                }
                                if (((((l_f_9) != (0i32)) as i32)) != 0) {
                                    { let a11_ = F32(c.m.r32((l_w_5).wrapping_add(1468))); F::EftEmit_Spawn(c, p_set_3, (l_w_5).wrapping_add(148), (l_w_5).wrapping_add(864), (l_w_5).wrapping_add(16), (l_w_5).wrapping_add(48), p_objId_1, 0i32, 0i32, l_part_6, l_sub_7, l_f_9, a11_) };
                                }
                            }
                            { let o_ = l_sub_7; let n_ = (o_).wrapping_add(1); l_sub_7 = n_; };
                        }
                    }
                }
            }
            { let o_ = l_part_6; let n_ = (o_).wrapping_add(1); l_part_6 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiBlast_SpawnParts", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiBlast_EndTrail(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32, mut p_set_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiBlast_EndTrail", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32, (p_set_3) as u32], false); }
    let mut l_w_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_part_6: u32 = 0;
    let mut l_def_7: u32 = 0;
    let r_ = 'f: {
    l_w_4 = c.m.r32((p_task_2).wrapping_add(56));
    if (((({ let a_ = c.m.r32(p_set_3); c.m.r32(a_) }) & (((131072i32) as u32)))) != 0) {
        l_part_6 = ((p_set_3).wrapping_add(12)).wrapping_add((((17i32) as u32)).wrapping_mul(16));
        l_def_7 = c.m.r32(l_part_6);
        {
            l_i_5 = 0i32;
            'l1: loop {
                if !(((((l_i_5) < (((c.m.r8((l_def_7).wrapping_add(1))) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let r_ = 128i32; let a_ = (((l_w_4).wrapping_add(148)).wrapping_add(320)).wrapping_add((((((c.m.r8((l_part_6).wrapping_add(8))) as i32)).wrapping_add(l_i_5)) as u32)); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiBlast_EndTrail", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiBlast_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiBlast_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_mgr_3: u32 = 0;
    let mut l_w_4: u32 = 0;
    let mut l_set_5: u32 = 0;
    let r_ = 'f: {
    l_mgr_3 = { let a_ = (F::BtlTask_GetParent(c, p_task_1)).wrapping_add(56); c.m.r32(a_) };
    l_w_4 = c.m.r32((p_task_1).wrapping_add(56));
    l_set_5 = c.m.r32(l_mgr_3);
    { let _ = F::memset(c, l_w_4, 0i32, 1504u32); };
    { let s_ = p_arg_2; let d_ = (l_w_4).wrapping_add(64); rt::memcpy(c, d_, s_, 80); };
    { let v_ = (c.m.r16((p_arg_2).wrapping_add(18)) as i16); let a_ = ((l_w_4).wrapping_add(64)).wrapping_add(16); c.m.w16(a_, (v_) as u16); };
    if (((((((c.m.r8((p_arg_2).wrapping_add(26))) as i32)) != (0i32)) as i32)) != 0) {
        { let r_ = 128i32; let a_ = (l_w_4).wrapping_add(1456); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    { let a0_ = (((c.m.r16((p_arg_2).wrapping_add(18)) as i16)) as i32); let a1_ = (((c.m.r16((p_arg_2).wrapping_add(20)) as i16)) as i32); F::BtlCharApi_GetNodePos(c, a0_, a1_, l_w_4) };
    { let a0_ = (((c.m.r16((p_arg_2).wrapping_add(18)) as i16)) as i32); let a1_ = (((c.m.r16((p_arg_2).wrapping_add(20)) as i16)) as i32); F::BtlCharApi_GetNodePos(c, a0_, a1_, (l_w_4).wrapping_add(16)) };
    { let a0_ = (((c.m.r16((p_arg_2).wrapping_add(18)) as i16)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 17i32, (l_w_4).wrapping_add(32)) };
    F::Vec4_Copy(c, (l_w_4).wrapping_add(48), p_arg_2);
    { let v_ = F32(c.m.r32((p_arg_2).wrapping_add(40))); let a_ = (l_w_4).wrapping_add(1468); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_arg_2).wrapping_add(44))); let a_ = (l_w_4).wrapping_add(1472); c.m.w32(a_, (v_).0); };
    { let v_ = (((c.m.r16((p_arg_2).wrapping_add(28)) as i16)) as i32); let a_ = (l_w_4).wrapping_add(1460); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((c.m.r8((p_arg_2).wrapping_add(27))) as i32); let a_ = (l_w_4).wrapping_add(1464); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_set_5; let a_ = (l_w_4).wrapping_add(144); c.m.w32(a_, v_); };
    if ((F::EftEmit_HasWidth2(c, l_set_5)) != 0) {
        { let v_ = F::EftEmit_GetWidth2(c, (l_w_4).wrapping_add(148)); let a_ = ((l_w_4).wrapping_add(64)).wrapping_add(32); c.m.w32(a_, (v_).0); };
        { let r_ = 2048i32; let a_ = (l_w_4).wrapping_add(1456); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    { let a0_ = c.m.r32((l_w_4).wrapping_add(144)); F::EftEmit_InitState(c, a0_, (l_w_4).wrapping_add(148)) };
    { let r_ = ((2i32) as u32); let a_ = (l_w_4).wrapping_add(1476); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    { let a2_ = (((c.m.r16((p_arg_2).wrapping_add(20)) as i16)) as i32); F::EftEmit_SetNode(c, (l_w_4).wrapping_add(864), 1i32, a2_, ((0i32) as u32)) };
    { let a0_ = (((c.m.r16((p_arg_2).wrapping_add(18)) as i16)) as i32); F::EftEmit_RefreshFixedNodes(c, a0_, (l_w_4).wrapping_add(864)) };
    { let v_ = rt::i2f({ let a0_ = c.m.r32((l_w_4).wrapping_add(144)); F::EftEmit_GetEndFrames(c, a0_) }); let a_ = (l_w_4).wrapping_add(1484); c.m.w32(a_, (v_).0); };
    if (({ let a0_ = (((c.m.r16((p_arg_2).wrapping_add(18)) as i16)) as i32); F::BtlCharApi_IsLockedOn(c, a0_) }) != 0) {
        { let r_ = 4i32; let a_ = (l_w_4).wrapping_add(1456); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    { let a1_ = (if ((((((((c.m.r16((p_arg_2).wrapping_add(18)) as i16)) as i32)) == (0i32)) as i32)) != 0) { 2048i32 } else { 4096i32 }); F::BtlTask_SetOwnerTag(c, p_task_1, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiBlast_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiBlast_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiBlast_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    { let a0_ = c.m.r32((l_w_2).wrapping_add(144)); F::EftEmit_TermState(c, a0_, (l_w_2).wrapping_add(148)) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiBlast_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiBlast_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiBlast_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_w_2: u32 = 0;
    let mut l_arg_3: u32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_arg_3 = (l_w_2).wrapping_add(64);
        if ((((!(({ let a0_ = (((c.m.r16((l_arg_3).wrapping_add(18)) as i16)) as i32); F::BtlScene_IsEffectStopped(c, a0_, 0i32) }) != 0)) as i32)) != 0) {
            if ((((!(((((c.m.r32((l_w_2).wrapping_add(1456)) as i32)) & (256i32))) != 0)) as i32)) != 0) {
                if (((((c.m.r32((l_w_2).wrapping_add(1456)) as i32)) & (4i32))) != 0) {
                    { let a3_ = (((c.m.r16((l_arg_3).wrapping_add(18)) as i16)) as i32); let a4_ = F32(c.m.r32((l_arg_3).wrapping_add(32))); let a5_ = F32(c.m.r32((l_arg_3).wrapping_add(36))); F::EftAim_Home(c, fp, (l_w_2).wrapping_add(16), (l_w_2).wrapping_add(48), a3_, a4_, a5_) };
                    F::Vec4_Copy(c, (l_w_2).wrapping_add(48), fp);
                }
                if (((((c.m.r32((l_w_2).wrapping_add(1456)) as i32)) & (1i32))) != 0) {
                    if (((((c.m.r32((l_w_2).wrapping_add(1456)) as i32)) & (2i32))) != 0) {
                        F::Vec4_Copy(c, (l_w_2).wrapping_add(32), (l_w_2).wrapping_add(16));
                    }
                    if ((((!(((((c.m.r32((l_w_2).wrapping_add(1456)) as i32)) & (16i32))) != 0)) as i32)) != 0) {
                        { let a2_ = F32(c.m.r32((l_arg_3).wrapping_add(32))); F::Vec3_Scale(c, fp.wrapping_add(16), (l_w_2).wrapping_add(48), a2_) };
                        F::Vec3_Add(c, (l_w_2).wrapping_add(16), (l_w_2).wrapping_add(16), fp.wrapping_add(16));
                    }
                    { let r_ = 2i32; let a_ = (l_w_2).wrapping_add(1456); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
                { let r_ = 1i32; let a_ = (l_w_2).wrapping_add(1456); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
            { let a_ = (l_w_2).wrapping_add(1460); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
            if ((((((c.m.r32((l_w_2).wrapping_add(1460)) as i32)) <= (0i32)) as i32)) != 0) {
                { let r_ = 16i32; let a_ = (l_w_2).wrapping_add(1456); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
            { let a0_ = (((c.m.r16(((l_w_2).wrapping_add(64)).wrapping_add(16)) as i16)) as i32); let a2_ = c.m.r32((l_w_2).wrapping_add(144)); F::EftKiBlast_SpawnParts(c, a0_, p_task_1, a2_, 0i32) };
            if (((((c.m.r32((l_w_2).wrapping_add(1456)) as i32)) & (32i32))) != 0) {
                { let r_ = 64i32; let a_ = (l_w_2).wrapping_add(1456); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            } else {
                if (((((c.m.r32((l_w_2).wrapping_add(1456)) as i32)) & (16i32))) != 0) {
                    if (((rt::fge(F32(c.m.r32((l_w_2).wrapping_add(1480))), F32(c.m.r32((l_w_2).wrapping_add(1484)))) as i32)) != 0) {
                        { let r_ = 32i32; let a_ = (l_w_2).wrapping_add(1456); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(1480); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                } else {
                    if ((((((c.m.r32((l_arg_3).wrapping_add(56)) as i32)) != (0i32)) as i32)) != 0) {
                        F::EftKiBlast_AddHitRecord(c, p_task_1);
                    }
                }
            }
        }
        if (((((c.m.r32((l_w_2).wrapping_add(1456)) as i32)) & (64i32))) != 0) {
            F::BtlTask_SetDead(c, p_task_1);
        }
        if (((((c.m.r32((l_w_2).wrapping_add(1456)) as i32)) & (16i32))) != 0) {
            { let a_ = (l_w_2).wrapping_add(1488); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftKiBlast_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiBlast_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiBlast_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(1456)) as i32)) & (4096i32))) != 0)) as i32)) != 0) {
        { let r_ = 4096i32; let a_ = (l_w_2).wrapping_add(1456); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let a0_ = c.m.r32((l_w_2).wrapping_add(144)); F::EftEmit_KillAll(c, a0_, (l_w_2).wrapping_add(148)) };
    }
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiBlast_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiBlast_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiBlast_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_arg_3: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_arg_3 = (l_w_2).wrapping_add(64);
    { let _ = { let a0_ = c.m.r32((l_w_2).wrapping_add(144)); F::EftEmit_UpdateAlive(c, a0_, (l_w_2).wrapping_add(148)) }; };
    { let v_ = ((0i32) as u32); let a_ = (l_w_2).wrapping_add(1476); c.m.w32(a_, v_); };
    if (((((c.m.r32((p_task_1).wrapping_add(4)) as i32)) & (63i32))) != 0) {
        { let v_ = 0i32; let a_ = (l_w_2).wrapping_add(1460); c.m.w32(a_, (v_) as u32); };
        { let r_ = 16i32; let a_ = (l_w_2).wrapping_add(1456); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        if ((((!(((((c.m.r32((l_w_2).wrapping_add(1456)) as i32)) & (256i32))) != 0)) as i32)) != 0) {
            F::Vec3_Copy(c, (l_w_2).wrapping_add(16), (p_task_1).wrapping_add(16));
        }
        { let a0_ = (((c.m.r16((l_arg_3).wrapping_add(16)) as i16)) as i32); let a2_ = c.m.r32((l_w_2).wrapping_add(144)); F::EftKiBlast_SpawnParts(c, a0_, p_task_1, a2_, 1i32) };
        if ((((!(((((c.m.r32((l_w_2).wrapping_add(1456)) as i32)) & (256i32))) != 0)) as i32)) != 0) {
            { let a0_ = (((c.m.r16((l_arg_3).wrapping_add(16)) as i16)) as i32); let a2_ = c.m.r32((l_w_2).wrapping_add(144)); F::EftKiBlast_EndTrail(c, a0_, p_task_1, a2_) };
        }
        { let r_ = 256i32; let a_ = (l_w_2).wrapping_add(1456); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        if (((((c.m.r32((p_task_1).wrapping_add(4)) as i32)) & (192i32))) != 0) {
            if ((((!(((((c.m.r32((l_w_2).wrapping_add(1456)) as i32)) & (8i32))) != 0)) as i32)) != 0) {
                F::EftKiBlast_Turn(c, p_task_1, l_w_2);
                { let v_ = F32(c.m.r32((p_task_1).wrapping_add(16))); let a_ = (l_w_2).wrapping_add(16); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((p_task_1).wrapping_add(16)).wrapping_add(4))); let a_ = ((l_w_2).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((p_task_1).wrapping_add(16)).wrapping_add(8))); let a_ = ((l_w_2).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let a0_ = (((c.m.r16((l_arg_3).wrapping_add(16)) as i16)) as i32); let a2_ = c.m.r32((l_w_2).wrapping_add(144)); F::EftKiBlast_SpawnParts(c, a0_, p_task_1, a2_, 1i32) };
                { let a0_ = (((c.m.r16((l_arg_3).wrapping_add(16)) as i16)) as i32); let a2_ = c.m.r32((l_w_2).wrapping_add(144)); F::EftKiBlast_EndTrail(c, a0_, p_task_1, a2_) };
                { let r_ = 512i32; let a_ = (l_w_2).wrapping_add(1456); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = (!(192i32)); let a_ = (p_task_1).wrapping_add(4); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiBlast_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiBlast_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiBlast_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (({ let a0_ = (((c.m.r16(((l_w_2).wrapping_add(64)).wrapping_add(18)) as i16)) as i32); F::BtlScene_IsEffectHidden(c, a0_, 0i32) }) != 0) {
        break 'f;
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiBlast_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiBlastMgr_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiBlastMgr_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_mgr_3: u32 = 0;
    let mut l_set_4: u32 = 0;
    let mut l_pack_5: u32 = 0;
    let mut l_idx_6: i32 = 0;
    let mut l_count_7: i32 = 0;
    let mut l_list_8: u32 = 0;
    let r_ = 'f: {
    l_mgr_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((804u32) as i32)) }; let a_ = l_mgr_3; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(l_mgr_3); F::memset(c, a0_, 0i32, 804u32) }; };
    l_idx_6 = 4i32;
    l_set_4 = c.m.r32(l_mgr_3);
    if ((((((c.m.r32((p_arg_2).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) == (1i32)) as i32)) != 0) {
        l_idx_6 = 3i32;
        l_count_7 = 20i32;
    } else {
        l_count_7 = 10i32;
    }
    l_pack_5 = { let a0_ = (c.m.r32((p_arg_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::BtlScene_GetCharPackEntry(c, a0_, l_idx_6) };
    { let v_ = l_pack_5; let a_ = (l_set_4).wrapping_add(800); c.m.w32(a_, v_); };
    if (((((l_pack_5) != (((0i32) as u32))) as i32)) != 0) {
        F::EftEmit_LoadSet(c, ((0i32) as u32), l_set_4, ((((0i32) as i64 as u64)) as u32), l_pack_5, 2i32, 1i32);
    }
    l_list_8 = F::BtlTask_CreateChildList(c, p_task_1, ((((l_count_7) as i64 as u64)) as u32), ((((((1504u32) as i32)) as i64 as u64)) as u32));
    { let a0_ = (c.m.r32((p_arg_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a1_ = (c.m.r32((p_arg_2).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); F::EftChar_SetList(c, a0_, a1_, l_list_8) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiBlastMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiBlastMgr_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiBlastMgr_Term", &PROBE_N, &[(p_task_1) as u32], false); }
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
    if rt::probing() { crate::host::harness::probe(c, "EftKiBlastMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiBlastMgr_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiBlastMgr_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_mgr_2: u32 = 0;
    let r_ = 'f: {
    l_mgr_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((({ let a_ = (c.m.r32(l_mgr_2)).wrapping_add(800); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(l_mgr_2); F::EftEmit_BeginFrame(c, a0_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiBlastMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiBlast_Fire(c: &mut Ctx, p_arg_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiBlast_Fire", &PROBE_N, &[(p_arg_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_list_2: u32 = 0;
    rt::memcpy(c, fp, p_arg_1, 80);
    let r_ = 'f: {
        l_list_2 = { let a0_ = (((c.m.r16((fp).wrapping_add(18)) as i16)) as i32); let a1_ = (if (((((((c.m.r8((fp).wrapping_add(26))) as i32)) == (0i32)) as i32)) != 0) { 1i32 } else { 2i32 }); F::EftChar_GetList(c, a0_, a1_) };
        if (((((l_list_2) != (((0i32) as u32))) as i32)) != 0) {
            { let _ = F::BtlTaskList_AddTail(c, l_list_2, sym::gEftKiBlastClass, fp); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftKiBlast_Fire", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiBlast_Ret1A(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiBlast_Ret1A", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiBlast_Ret1A", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiBlast_Ret1B(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiBlast_Ret1B", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiBlast_Ret1B", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastCharge_SpawnParts(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32, mut p_set_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastCharge_SpawnParts", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32, (p_set_3) as u32], false); }
    let mut l_w_4: u32 = 0;
    let mut l_part_5: i32 = 0;
    let mut l_sub_6: i32 = 0;
    let mut l_def_7: u32 = 0;
    let mut l_f_8: i32 = 0;
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
                                l_f_8 = { let a5_ = ((((c.m.r32((l_w_4).wrapping_add(1376))) & (((2i32) as u32)))) as i32); let a7_ = ((c.m.r32((l_w_4).wrapping_add(1388))) as i32); F::EftEmit_GetFlagsFromMask(c, p_set_3, (l_w_4).wrapping_add(68), p_objId_1, l_part_5, l_sub_6, a5_, 0i32, a7_) };
                                if (((((l_f_8) != (0i32)) as i32)) != 0) {
                                    { let a11_ = F32(c.m.r32((l_w_4).wrapping_add(1384))); F::EftEmit_Spawn(c, p_set_3, (l_w_4).wrapping_add(68), (l_w_4).wrapping_add(784), l_w_4, (l_w_4).wrapping_add(16), p_objId_1, 0i32, 0i32, l_part_5, l_sub_6, l_f_8, a11_) };
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
    if rt::probing() { crate::host::harness::probe(c, "EftBlastCharge_SpawnParts", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastCharge_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastCharge_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_mgr_3: u32 = 0;
    let mut l_w_4: u32 = 0;
    let mut l_set_5: u32 = 0;
    let r_ = 'f: {
    l_mgr_3 = { let a_ = (F::BtlTask_GetParent(c, p_task_1)).wrapping_add(56); c.m.r32(a_) };
    l_w_4 = c.m.r32((p_task_1).wrapping_add(56));
    l_set_5 = c.m.r32(l_mgr_3);
    { let _ = F::memset(c, l_w_4, 0i32, 1408u32); };
    { let s_ = p_arg_2; let d_ = (l_w_4).wrapping_add(32); rt::memcpy(c, d_, s_, 32); };
    { let a0_ = (c.m.r32((p_arg_2).wrapping_add(16)) as i32); let a1_ = (c.m.r32((p_arg_2).wrapping_add(20)) as i32); F::BtlCharApi_GetNodePos(c, a0_, a1_, l_w_4) };
    F::Vec4_Copy(c, (l_w_4).wrapping_add(16), (l_w_4).wrapping_add(32));
    { let r_ = ((1i32) as u32); let a_ = (l_w_4).wrapping_add(1376); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    { let v_ = rt::fdiv({ let a0_ = (c.m.r32((p_arg_2).wrapping_add(16)) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x419acccc)); let a_ = (l_w_4).wrapping_add(1384); c.m.w32(a_, (v_).0); };
    if (((rt::flt(F32(c.m.r32((l_w_4).wrapping_add(1384))), F32(0x3f000000)) as i32)) != 0) {
        { let v_ = F32(0x3f000000); let a_ = (l_w_4).wrapping_add(1384); c.m.w32(a_, (v_).0); };
    }
    { let v_ = (c.m.r32(((l_w_4).wrapping_add(32)).wrapping_add(24)) as i32); let a_ = (l_w_4).wrapping_add(1380); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_set_5; let a_ = (l_w_4).wrapping_add(64); c.m.w32(a_, v_); };
    if (((((l_set_5) == (((0i32) as u32))) as i32)) != 0) {
        F::BtlTask_SetDead(c, p_task_1);
        break 'f;
    }
    F::EftEmit_InitState(c, l_set_5, (l_w_4).wrapping_add(68));
    { let r_ = ((1i32) as u32); let a_ = (l_w_4).wrapping_add(1388); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    { let a2_ = (c.m.r32((p_arg_2).wrapping_add(20)) as i32); F::EftEmit_SetNode(c, (l_w_4).wrapping_add(784), 0i32, a2_, ((0i32) as u32)) };
    { let v_ = rt::i2f({ let a0_ = c.m.r32((l_w_4).wrapping_add(64)); F::EftEmit_GetEndFrames(c, a0_) }); let a_ = (l_w_4).wrapping_add(1396); c.m.w32(a_, (v_).0); };
    { let a1_ = (if ((((((c.m.r32((p_arg_2).wrapping_add(16)) as i32)) == (0i32)) as i32)) != 0) { 2048i32 } else { 4096i32 }); F::BtlTask_SetOwnerTag(c, p_task_1, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastCharge_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastCharge_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastCharge_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_arg_3: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_arg_3 = (l_w_2).wrapping_add(32);
    if (((((c.m.r32((l_w_2).wrapping_add(64))) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32((l_w_2).wrapping_add(64)); F::EftEmit_TermState(c, a0_, (l_w_2).wrapping_add(68)) };
    }
    { let a0_ = (c.m.r32((l_arg_3).wrapping_add(16)) as i32); F::EftCharSlot_Clear1(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastCharge_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastCharge_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastCharge_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_arg_3: u32 = 0;
    let mut l_dead_4: i32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_arg_3 = (l_w_2).wrapping_add(32);
    if (((((c.m.r32((l_w_2).wrapping_add(64))) == (((0i32) as u32))) as i32)) != 0) {
        F::BtlTask_SetDead(c, p_task_1);
        break 'f;
    }
    if ((((!(({ let a0_ = (c.m.r32((l_arg_3).wrapping_add(16)) as i32); F::BtlScene_IsEffectStopped(c, a0_, 0i32) }) != 0)) as i32)) != 0) {
        { let a0_ = (c.m.r32((l_arg_3).wrapping_add(16)) as i32); let a1_ = (c.m.r32((l_arg_3).wrapping_add(20)) as i32); F::BtlCharApi_GetNodePos(c, a0_, a1_, l_w_2) };
        { let a0_ = (c.m.r32(((l_w_2).wrapping_add(32)).wrapping_add(16)) as i32); F::EftEmit_RefreshFixedNodes(c, a0_, (l_w_2).wrapping_add(784)) };
        { let a0_ = (c.m.r32(((l_w_2).wrapping_add(32)).wrapping_add(16)) as i32); let a2_ = c.m.r32((l_w_2).wrapping_add(64)); F::EftBlastCharge_SpawnParts(c, a0_, p_task_1, a2_) };
        if ((((!(({ let a0_ = (c.m.r32((l_arg_3).wrapping_add(16)) as i32); F::BtlCharApi_IsChargingKiBlast(c, a0_) }) != 0)) as i32)) != 0) {
            { let r_ = ((2i32) as u32); let a_ = (l_w_2).wrapping_add(1376); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        }
    }
    if ((((c.m.r32((l_w_2).wrapping_add(1376))) & (((4i32) as u32)))) != 0) {
        F::BtlTask_SetDead(c, p_task_1);
        l_dead_4 = 1i32;
    } else {
        if ((((c.m.r32((l_w_2).wrapping_add(1376))) & (((2i32) as u32)))) != 0) {
            { let a0_ = (c.m.r32(((l_w_2).wrapping_add(32)).wrapping_add(16)) as i32); let a2_ = c.m.r32((l_w_2).wrapping_add(64)); F::EftBlastCharge_SpawnParts(c, a0_, p_task_1, a2_) };
            { let r_ = ((4i32) as u32); let a_ = (l_w_2).wrapping_add(1376); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastCharge_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastCharge_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastCharge_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastCharge_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastCharge_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastCharge_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((c.m.r32((l_w_2).wrapping_add(1376))) & (((8i32) as u32)))) != 0) {
        { let _ = { let a0_ = c.m.r32((l_w_2).wrapping_add(64)); F::EftEmit_UpdateAlive(c, a0_, (l_w_2).wrapping_add(68)) }; };
        { let v_ = ((0i32) as u32); let a_ = (l_w_2).wrapping_add(1388); c.m.w32(a_, v_); };
    }
    { let r_ = ((8i32) as u32); let a_ = (l_w_2).wrapping_add(1376); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastCharge_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastCharge_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastCharge_Draw", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastCharge_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastChargeMgr_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastChargeMgr_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_mgr_3: u32 = 0;
    let mut l_set_4: u32 = 0;
    let mut l_pack_5: u32 = 0;
    let mut l_list_6: u32 = 0;
    let r_ = 'f: {
    l_mgr_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((804u32) as i32)) }; let a_ = l_mgr_3; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(l_mgr_3); F::memset(c, a0_, 0i32, 804u32) }; };
    l_set_4 = c.m.r32(l_mgr_3);
    l_pack_5 = { let a0_ = (c.m.r32((p_arg_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::BtlScene_GetCharPackEntry(c, a0_, 4i32) };
    { let v_ = l_pack_5; let a_ = (l_set_4).wrapping_add(800); c.m.w32(a_, v_); };
    if (((((l_pack_5) != (((0i32) as u32))) as i32)) != 0) {
        F::EftEmit_LoadSet(c, ((0i32) as u32), l_set_4, ((((0i32) as i64 as u64)) as u32), l_pack_5, 2i32, 1i32);
    }
    l_list_6 = F::BtlTask_CreateChildList(c, p_task_1, ((((10i32) as i64 as u64)) as u32), ((((((1408u32) as i32)) as i64 as u64)) as u32));
    { let a0_ = (c.m.r32((p_arg_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::EftChar_SetList(c, a0_, 3i32, l_list_6) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastChargeMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastChargeMgr_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastChargeMgr_Term", &PROBE_N, &[(p_task_1) as u32], false); }
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
    if rt::probing() { crate::host::harness::probe(c, "EftBlastChargeMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastChargeMgr_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastChargeMgr_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_mgr_2: u32 = 0;
    let r_ = 'f: {
    l_mgr_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((({ let a_ = (c.m.r32(l_mgr_2)).wrapping_add(800); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(l_mgr_2); F::EftEmit_BeginFrame(c, a0_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastChargeMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastCharge_Start(c: &mut Ctx, mut p_arg_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastCharge_Start", &PROBE_N, &[(p_arg_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_list_2: u32 = 0;
    let mut l_task_3: u32 = 0;
    let r_ = 'f: {
        if (((((p_arg_1) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        { let s_ = p_arg_1; let d_ = fp; rt::memcpy(c, d_, s_, 32); };
        l_list_2 = { let a0_ = (c.m.r32((fp).wrapping_add(16)) as i32); F::EftChar_GetList(c, a0_, 3i32) };
        if (((((l_list_2) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        l_task_3 = F::BtlTaskList_AddTail(c, l_list_2, sym::gEftBlastChargeClass, fp);
        if (((((l_task_3) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        { let a0_ = (c.m.r32((fp).wrapping_add(16)) as i32); F::EftCharSlot_Set1(c, a0_, l_task_3) };
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBlastCharge_Start", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastCharge_Stop(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastCharge_Stop", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_task_2: u32 = 0;
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    l_task_2 = F::EftCharSlot_Get1(c, p_objId_1);
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
    if rt::probing() { crate::host::harness::probe(c, "EftBlastCharge_Stop", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiBomb_AddHitRecord(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiBomb_AddHitRecord", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_rec_3: u32 = 0;
    let mut l_a_4: u32 = 0;
    let mut l_b_5: u32 = 0;
    let mut l_radius_6: F32 = F32(0);
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_rec_3 = F::EftHit_GetNew(c);
    { let v_ = p_task_1; let a_ = (l_rec_3).wrapping_add(96); c.m.w32(a_, v_); };
    { let s_ = (l_w_2).wrapping_add(80); let d_ = (l_rec_3).wrapping_add(16); rt::memcpy(c, d_, s_, 64); };
    { let v_ = 0i32; let a_ = (l_rec_3).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((((((c.m.r32((l_w_2).wrapping_add(208)) as i32)).wrapping_shr((5i32) as u32)) & (1i32))) as u8); let a_ = (l_rec_3).wrapping_add(92); c.m.w8(a_, v_); };
    { let r_ = 514i32; let a_ = (l_rec_3).wrapping_add(80); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    { let v_ = ((((c.m.r32((l_w_2).wrapping_add(208)) as i32)).wrapping_shr((9i32) as u32)) & (1i32)); let a_ = (l_rec_3).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_w_2; let a_ = (l_rec_3).wrapping_add(104); c.m.w32(a_, v_); };
    l_radius_6 = F32(c.m.r32((l_w_2).wrapping_add(228)));
    l_a_4 = F::EftHitArena_AllocSphere(c);
    l_b_5 = F::EftHitArena_AllocSphere(c);
    F::ColSphere_Set(c, l_a_4, ((l_w_2).wrapping_add(80)).wrapping_add(16), l_radius_6);
    F::ColSphere_Set(c, l_b_5, ((l_w_2).wrapping_add(80)).wrapping_add(32), l_radius_6);
    F::EftHit_SetShapeSpheres(c, l_rec_3, l_a_4, l_b_5);
    F::EftHit_Add(c, l_rec_3);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiBomb_AddHitRecord", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiBomb_Launch(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiBomb_Launch", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_w_2: u32 = 0;
    let mut l_res_3: u32 = 0;
    let mut l_spin_4: F32 = F32(0);
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        { let a0_ = (((c.m.r16((l_w_2).wrapping_add(18)) as i16)) as i32); let a1_ = (((c.m.r16((l_w_2).wrapping_add(20)) as i16)) as i32); F::BtlCharApi_GetNodePos(c, a0_, a1_, (l_w_2).wrapping_add(80)) };
        { let a0_ = (((c.m.r16((l_w_2).wrapping_add(18)) as i16)) as i32); let a1_ = (((c.m.r16((l_w_2).wrapping_add(20)) as i16)) as i32); F::BtlCharApi_GetNodePos(c, a0_, a1_, ((l_w_2).wrapping_add(80)).wrapping_add(16)) };
        { let a0_ = (((c.m.r16((l_w_2).wrapping_add(18)) as i16)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 17i32, ((l_w_2).wrapping_add(80)).wrapping_add(32)) };
        F::Vec4_Copy(c, ((l_w_2).wrapping_add(80)).wrapping_add(48), l_w_2);
        { let r_ = F::Rand_FloatRange(c, F32(0xbe4ccccc), F32(0x3e4ccccc)); let a_ = ((l_w_2).wrapping_add(80)).wrapping_add(48); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F::Rand_FloatRange(c, F32(0x00000000), F32(0x3e999999)); let a_ = (((l_w_2).wrapping_add(80)).wrapping_add(48)).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F::Rand_FloatRange(c, F32(0xbe4ccccc), F32(0x3e4ccccc)); let a_ = (((l_w_2).wrapping_add(80)).wrapping_add(48)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        F::Vec3_Normalize(c, ((l_w_2).wrapping_add(80)).wrapping_add(48), ((l_w_2).wrapping_add(80)).wrapping_add(48));
        { let v_ = F32(c.m.r32((l_w_2).wrapping_add(40))); let a_ = (l_w_2).wrapping_add(224); c.m.w32(a_, (v_).0); };
        { let v_ = (((c.m.r16((l_w_2).wrapping_add(28)) as i16)) as i32); let a_ = (l_w_2).wrapping_add(212); c.m.w32(a_, (v_) as u32); };
        { let v_ = F32(c.m.r32((l_w_2).wrapping_add(32))); let a_ = (l_w_2).wrapping_add(220); c.m.w32(a_, (v_).0); };
        { let v_ = ((15i32) as i16); let a_ = (l_w_2).wrapping_add(250); c.m.w16(a_, (v_) as u16); };
        { let v_ = ((300i32) as i16); let a_ = (l_w_2).wrapping_add(246); c.m.w16(a_, (v_) as u16); };
        { let v_ = 3i32; let a_ = (l_w_2).wrapping_add(216); c.m.w32(a_, (v_) as u32); };
        { let v_ = F32(0x3f000000); let a_ = (l_w_2).wrapping_add(240); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x40a00000); let a_ = (l_w_2).wrapping_add(228); c.m.w32(a_, (v_).0); };
        l_spin_4 = rt::fadd(rt::fsub(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x41200000)), F32(0x40a00000)), F32(0x42000000));
        { let v_ = l_spin_4; let a_ = (l_w_2).wrapping_add(236); c.m.w32(a_, (v_).0); };
        { let v_ = (if (((rt::flt(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3f000000)) as i32)) != 0) { l_spin_4 } else { rt::fneg(l_spin_4) }); let a_ = (l_w_2).wrapping_add(236); c.m.w32(a_, (v_).0); };
        { let a2_ = F32(c.m.r32((l_w_2).wrapping_add(220))); F::Vec3_Scale(c, ((l_w_2).wrapping_add(80)).wrapping_add(48), ((l_w_2).wrapping_add(80)).wrapping_add(48), a2_) };
        l_res_3 = { let a_ = { let a_ = (F::BtlTask_GetParent(c, p_task_1)).wrapping_add(56); c.m.r32(a_) }; c.m.r32(a_) };
        F::Mtx_StoreIdentity(c, fp);
        { let v_ = { let a1_ = c.m.r32((l_res_3).wrapping_add(80)); F::EftObj_Create(c, (l_w_2).wrapping_add(252), a1_) }; let a_ = (l_w_2).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
        if ((((((c.m.r32((l_w_2).wrapping_add(300)) as i32)) != ((1i32).wrapping_neg())) as i32)) != 0) {
            F::Mtx_SetTrans(c, fp, ((l_w_2).wrapping_add(80)).wrapping_add(16));
            { let a0_ = (c.m.r32((l_w_2).wrapping_add(300)) as i32); F::EftObj_SetMtx(c, a0_, fp) };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftKiBomb_Launch", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiBomb_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiBomb_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let _ = F::memset(c, l_w_3, 0i32, 304u32); };
    { let s_ = p_arg_2; let d_ = l_w_3; rt::memcpy(c, d_, s_, 80); };
    { let v_ = (c.m.r16((p_arg_2).wrapping_add(18)) as i16); let a_ = (l_w_3).wrapping_add(16); c.m.w16(a_, (v_) as u16); };
    F::EftKiBomb_Launch(c, p_task_1);
    { let r_ = ((64i32) | (128i32)); let a_ = (l_w_3).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    { let a1_ = (if ((((((((c.m.r16((p_arg_2).wrapping_add(18)) as i16)) as i32)) == (0i32)) as i32)) != 0) { 2048i32 } else { 4096i32 }); F::BtlTask_SetOwnerTag(c, p_task_1, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiBomb_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiBomb_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiBomb_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((((c.m.r32((l_w_2).wrapping_add(300)) as i32)) != ((1i32).wrapping_neg())) as i32)) != 0) {
        { let a0_ = (c.m.r32((l_w_2).wrapping_add(300)) as i32); { let _ = F::EftObj_Destroy(c, a0_); } };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiBomb_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiBomb_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiBomb_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_dead_3: i32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!((F::BtlScene_IsAnyCharStopped(c)) != 0)) as i32)) != 0) {
        { let _ = F::EftKiBomb_Move(c, p_task_1); };
        if (((((c.m.r32((l_w_2).wrapping_add(208)) as i32)) & (128i32))) != 0) {
            F::EftKiBomb_AddHitRecord(c, p_task_1);
        }
        if (((((c.m.r32((l_w_2).wrapping_add(208)) as i32)) & (8i32))) != 0) {
            F::BtlTask_SetDead(c, p_task_1);
            l_dead_3 = 1i32;
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiBomb_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiBomb_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiBomb_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiBomb_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiBomb_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiBomb_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::EftKiBomb_HandleHits(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiBomb_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiBomb_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiBomb_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::EftKiBomb_UpdateModel(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiBomb_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiBombMgr_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiBombMgr_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_mgr_3: u32 = 0;
    let mut l_res_4: u32 = 0;
    let mut l_list_5: u32 = 0;
    let r_ = 'f: {
    l_mgr_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let v_ = ((0i32) as u32); let a_ = l_mgr_3; c.m.w32(a_, v_); };
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((88u32) as i32)) }; let a_ = l_mgr_3; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(l_mgr_3); F::memset(c, a0_, 0i32, 88u32) }; };
    l_res_4 = c.m.r32(l_mgr_3);
    { let v_ = { let a0_ = (c.m.r32((p_arg_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::BtlScene_GetCharPackEntry(c, a0_, 4i32) }; let a_ = (l_res_4).wrapping_add(84); c.m.w32(a_, v_); };
    { let v_ = { let a0_ = c.m.r32((l_res_4).wrapping_add(84)); F::BtlScene_GetPackEntry(c, a0_, 1i32) }; let a_ = (l_res_4).wrapping_add(80); c.m.w32(a_, v_); };
    l_list_5 = F::BtlTask_CreateChildList(c, p_task_1, ((((10i32) as i64 as u64)) as u32), ((((((304u32) as i32)) as i64 as u64)) as u32));
    { let a0_ = (c.m.r32((p_arg_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a1_ = (c.m.r32((p_arg_2).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); F::EftChar_SetList(c, a0_, a1_, l_list_5) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiBombMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiBombMgr_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiBombMgr_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_mgr_2: u32 = 0;
    let r_ = 'f: {
    l_mgr_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((c.m.r32(l_mgr_2)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(l_mgr_2); F::BtlPool_Free(c, a0_, a1_) };
        { let v_ = ((0i32) as u32); let a_ = l_mgr_2; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiBombMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiBombMgr_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiBombMgr_Update", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiBombMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiBomb_Fire(c: &mut Ctx, p_arg_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiBomb_Fire", &PROBE_N, &[(p_arg_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_list_2: u32 = 0;
    rt::memcpy(c, fp, p_arg_1, 80);
    let r_ = 'f: {
        l_list_2 = { let a0_ = (((c.m.r16((fp).wrapping_add(18)) as i16)) as i32); F::EftChar_GetList(c, a0_, 2i32) };
        if (((((l_list_2) != (((0i32) as u32))) as i32)) != 0) {
            { let _ = F::BtlTaskList_AddTail(c, l_list_2, sym::gEftKiBombClass, fp); };
            { let _ = F::BtlTaskList_AddTail(c, l_list_2, sym::gEftKiBombClass, fp); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftKiBomb_Fire", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiBomb_SetContact(c: &mut Ctx, mut p_task_1: u32, mut p_mtx_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiBomb_SetContact", &PROBE_N, &[(p_task_1) as u32, (p_mtx_2) as u32], false); }
    let r_ = 'f: {
    if (((((((((((((p_task_1) != (((0i32) as u32))) as i32)) != 0) && ((((!((((((((c.m.r8(p_task_1)) as i32)) & (1i32))) as u8)) != 0)) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = c.m.r32((p_task_1).wrapping_add(40)); c.m.r32(a_) }) == (fid::EftKiBomb_Update)) as i32)) != 0)) as i32)) != 0) {
        { let s_ = p_mtx_2; let d_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(144); rt::memcpy(c, d_, s_, 64); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiBomb_SetContact", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiBomb_Reflect(c: &mut Ctx, mut p_out_1: u32, mut p_v_2: u32, mut p_normal_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiBomb_Reflect", &PROBE_N, &[(p_out_1) as u32, (p_v_2) as u32, (p_normal_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_d_4: F32 = F32(0);
    let r_ = 'f: {
        { let v_ = rt::fneg(F32(c.m.r32(p_normal_3))); let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fneg(F32(c.m.r32((p_normal_3).wrapping_add(4)))); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fneg(F32(c.m.r32((p_normal_3).wrapping_add(8)))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        l_d_4 = rt::fmul(rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(p_v_2)), F32(c.m.r32(fp))), rt::fmul(F32(c.m.r32((p_v_2).wrapping_add(4))), F32(c.m.r32((fp).wrapping_add(4))))), rt::fmul(F32(c.m.r32((p_v_2).wrapping_add(8))), F32(c.m.r32((fp).wrapping_add(8))))), F32(0xc0000000));
        { let v_ = rt::fadd(F32(c.m.r32(p_v_2)), rt::fmul(l_d_4, F32(c.m.r32(fp)))); let a_ = p_out_1; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((p_v_2).wrapping_add(4))), rt::fmul(l_d_4, F32(c.m.r32((fp).wrapping_add(4))))); let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((p_v_2).wrapping_add(8))), rt::fmul(l_d_4, F32(c.m.r32((fp).wrapping_add(8))))); let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec3_Normalize(c, p_out_1, p_out_1);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftKiBomb_Reflect", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiBomb_Move(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiBomb_Move", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(128) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 128);
    let mut l_w_2: u32 = 0;
    let mut l_pos_3: u32 = 0;
    let mut l_dist_4: F32 = F32(0);
    let mut l_r_5: F32 = F32(0);
    let mut l_snd_6: i32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        { let a_ = (l_w_2).wrapping_add(244); let o_ = (c.m.r16(a_) as i16); let n_ = (o_).wrapping_add(1); c.m.w16(a_, (n_) as u16); };
        if ((((((c.m.r32((l_w_2).wrapping_add(300)) as i32)) == ((1i32).wrapping_neg())) as i32)) != 0) {
            { let r_ = 8i32; let a_ = (l_w_2).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        l_pos_3 = ((l_w_2).wrapping_add(80)).wrapping_add(16);
        if (((((c.m.r32((l_w_2).wrapping_add(208)) as i32)) & (2i32))) != 0) {
            F::Vec4_Copy(c, ((l_w_2).wrapping_add(80)).wrapping_add(32), l_pos_3);
        }
        { let r_ = 2i32; let a_ = (l_w_2).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        F::Vec3_Add(c, l_pos_3, l_pos_3, ((l_w_2).wrapping_add(80)).wrapping_add(48));
        { let r_ = F32(c.m.r32((l_w_2).wrapping_add(240))); let a_ = (((l_w_2).wrapping_add(80)).wrapping_add(48)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let a_ = (l_w_2).wrapping_add(246); let o_ = (c.m.r16(a_) as i16); let n_ = (o_).wrapping_sub(1); c.m.w16(a_, (n_) as u16); };
        if ((((((((c.m.r16((l_w_2).wrapping_add(246)) as i16)) as i32)) <= (0i32)) as i32)) != 0) {
            { let r_ = 256i32; let a_ = (l_w_2).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if (((rt::flt(F::BtlStage_GetBottom(c), F32(c.m.r32((((l_w_2).wrapping_add(80)).wrapping_add(16)).wrapping_add(4)))) as i32)) != 0) {
            { let r_ = 256i32; let a_ = (l_w_2).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        { let a0_ = { let a0_ = (((c.m.r16((l_w_2).wrapping_add(18)) as i16)) as i32); F::BtlCharApi_GetOpponentObjId(c, a0_) }; F::BtlCharApi_GetPos(c, a0_, fp.wrapping_add(16)) };
        { let v_ = rt::fsub(F32(c.m.r32(fp.wrapping_add(16))), F32(c.m.r32(l_pos_3))); let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(c.m.r32((((l_w_2).wrapping_add(80)).wrapping_add(16)).wrapping_add(4)))); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(c.m.r32((((l_w_2).wrapping_add(80)).wrapping_add(16)).wrapping_add(8)))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        l_dist_4 = { let a0_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(fp)), F32(c.m.r32(fp))), rt::fmul(F32(c.m.r32((fp).wrapping_add(4))), F32(c.m.r32((fp).wrapping_add(4))))), rt::fmul(F32(c.m.r32((fp).wrapping_add(8))), F32(c.m.r32((fp).wrapping_add(8))))); F::Port_sqrtf(c, a0_) };
        if (((rt::flt(l_dist_4, F32(0x41e00000)) as i32)) != 0) {
            { let r_ = 256i32; let a_ = (l_w_2).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if ((((!(((((c.m.r32((l_w_2).wrapping_add(208)) as i32)) & (512i32))) != 0)) as i32)) != 0) {
            { let v_ = { let a0_ = rt::fdiv(rt::fmul(rt::i2f(rt::rem_i32(((((c.m.r16((l_w_2).wrapping_add(244)) as i16)) as i32)).wrapping_mul(rt::f2i(F32(c.m.r32((l_w_2).wrapping_add(236))))), 360i32)), F32(0x40490fda)), F32(0x43340000)); F::EftMath_WrapAngle(c, a0_) }; let a_ = (l_w_2).wrapping_add(232); c.m.w32(a_, (v_).0); };
        }
        if (((((c.m.r32((l_w_2).wrapping_add(208)) as i32)) & (512i32))) != 0) {
            { let v_ = rt::f2i(rt::fsub(rt::i2f((c.m.r32((l_w_2).wrapping_add(212)) as i32)), F32(0x3f800000))); let a_ = (l_w_2).wrapping_add(212); c.m.w32(a_, (v_) as u32); };
        }
        if ((((!(((((c.m.r32((l_w_2).wrapping_add(208)) as i32)) & (1024i32))) != 0)) as i32)) != 0) {
            if (((((((rt::fle(rt::i2f((c.m.r32((l_w_2).wrapping_add(212)) as i32)), F32(0x00000000)) as i32)) != 0) || (((((c.m.r32((l_w_2).wrapping_add(208)) as i32)) & (256i32))) != 0)) as i32)) != 0) {
                { let v_ = 0i32; let a_ = (l_w_2).wrapping_add(212); c.m.w32(a_, (v_) as u32); };
                { let s_ = { rt::memset(c, fp.wrapping_add(80), 0, 48);; { let s_ = ((l_w_2).wrapping_add(80)).wrapping_add(16); rt::memcpy(c, fp.wrapping_add(80).wrapping_add(0), s_, 16); }; rt::memset(c, fp.wrapping_add(80).wrapping_add(16), 0, 16);; rt::memset(c, fp.wrapping_add(80).wrapping_add(16).wrapping_add(0), 0, 16);; { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(80).wrapping_add(16).wrapping_add(0).wrapping_add(0), (v_).0); }; { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(80).wrapping_add(16).wrapping_add(0).wrapping_add(4), (v_).0); }; { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(80).wrapping_add(16).wrapping_add(0).wrapping_add(8), (v_).0); }; { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(80).wrapping_add(16).wrapping_add(0).wrapping_add(12), (v_).0); }; { let v_ = 1i32; c.m.w32(fp.wrapping_add(80).wrapping_add(32), (v_) as u32); }; { let v_ = (((c.m.r16((l_w_2).wrapping_add(18)) as i16)) as i32); c.m.w32(fp.wrapping_add(80).wrapping_add(36), (v_) as u32); }; { let v_ = 0i32; c.m.w32(fp.wrapping_add(80).wrapping_add(40), (v_) as u32); }; fp.wrapping_add(80) }; let d_ = fp.wrapping_add(32); rt::memcpy(c, d_, s_, 48); };
                { let _ = F::EftImpact_SpawnBlast(c, fp.wrapping_add(32), F32(0x3f800000), F32(0x3f800000)); };
                l_r_5 = F32(0x41e00000);
                { let a2_ = (if ((((F::Port_Rand(c)) & (2i32))) != 0) { 27i32 } else { 17i32 }); F::BtlCharApi_PlaySoundAt(c, l_pos_3, 0i32, a2_, F32(0x43480000), F32(0x44bb8000)) };
                { let v_ = l_r_5; let a_ = (l_w_2).wrapping_add(228); c.m.w32(a_, (v_).0); };
                { let v_ = (((((c.m.r32((l_w_2).wrapping_add(208)) as i32)) & ((!(64i32))))) | (1024i32)); let a_ = (l_w_2).wrapping_add(208); c.m.w32(a_, (v_) as u32); };
            }
        }
        if (((((c.m.r32((l_w_2).wrapping_add(208)) as i32)) & (1024i32))) != 0) {
            { let v_ = ((rt::f2i(rt::fsub(rt::i2f(((c.m.r16((l_w_2).wrapping_add(250)) as i16)) as i32), F32(0x3f800000)))) as i16); let a_ = (l_w_2).wrapping_add(250); c.m.w16(a_, (v_) as u16); };
            if (((rt::fle(rt::i2f(((c.m.r16((l_w_2).wrapping_add(250)) as i16)) as i32), F32(0x00000000)) as i32)) != 0) {
                { let v_ = ((0i32) as i16); let a_ = (l_w_2).wrapping_add(250); c.m.w16(a_, (v_) as u16); };
                { let r_ = 8i32; let a_ = (l_w_2).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                break 'f 1i32;
            }
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftKiBomb_Move", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiBomb_HandleHits(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiBomb_HandleHits", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_w_2: u32 = 0;
    let mut l_vel_3: u32 = 0;
    let mut l_r_4: F32 = F32(0);
    let mut l_speed_5: F32 = F32(0);
    let mut l_spin_6: F32 = F32(0);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 16);
        rt::memset(c, fp.wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(16), 0, 12);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x42b40000); c.m.w32(fp.wrapping_add(16).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x43870000); c.m.w32(fp.wrapping_add(16).wrapping_add(8), (v_).0); }
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        if ((((((c.m.r32((l_w_2).wrapping_add(300)) as i32)) == ((1i32).wrapping_neg())) as i32)) != 0) {
            break 'f;
        }
        if (((((c.m.r32((l_w_2).wrapping_add(208)) as i32)) & (64i32))) != 0) {
            if ((((!(({ let a0_ = (((c.m.r16((l_w_2).wrapping_add(18)) as i16)) as i32); F::BtlScene_IsEffectHidden(c, a0_, 0i32) }) != 0)) as i32)) != 0) {
                { let a0_ = (c.m.r32((l_w_2).wrapping_add(300)) as i32); F::EftObj_SetVisible(c, a0_, 1i32) };
            } else {
                { let a0_ = (c.m.r32((l_w_2).wrapping_add(300)) as i32); F::EftObj_SetVisible(c, a0_, 0i32) };
            }
        } else {
            { let a0_ = (c.m.r32((l_w_2).wrapping_add(300)) as i32); F::EftObj_SetVisible(c, a0_, 0i32) };
        }
        if (((((c.m.r32((p_task_1).wrapping_add(4)) as i32)) & (2i32))) != 0) {
            F::Vec4_Copy(c, ((l_w_2).wrapping_add(80)).wrapping_add(16), (p_task_1).wrapping_add(16));
            { let v_ = (((((c.m.r32((l_w_2).wrapping_add(208)) as i32)) | (256i32))) & ((!(128i32)))); let a_ = (l_w_2).wrapping_add(208); c.m.w32(a_, (v_) as u32); };
        } else {
            if (((((c.m.r32((p_task_1).wrapping_add(4)) as i32)) & (1i32))) != 0) {
                F::Vec4_Copy(c, ((l_w_2).wrapping_add(80)).wrapping_add(16), (p_task_1).wrapping_add(16));
                { let r_ = 256i32; let a_ = (l_w_2).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            } else {
                if (((((c.m.r32((p_task_1).wrapping_add(4)) as i32)) & (((4i32) | (16i32))))) != 0) {
                    l_vel_3 = ((l_w_2).wrapping_add(80)).wrapping_add(48);
                    F::Vec3_Normalize(c, l_vel_3, l_vel_3);
                    { let v_ = rt::fadd(F32(c.m.r32((p_task_1).wrapping_add(16))), rt::fmul(rt::fneg(F32(c.m.r32(l_vel_3))), F32(c.m.r32((l_w_2).wrapping_add(228))))); let a_ = ((l_w_2).wrapping_add(80)).wrapping_add(16); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32(((p_task_1).wrapping_add(16)).wrapping_add(4))), rt::fmul(rt::fneg(F32(c.m.r32((((l_w_2).wrapping_add(80)).wrapping_add(48)).wrapping_add(4)))), F32(c.m.r32((l_w_2).wrapping_add(228))))); let a_ = (((l_w_2).wrapping_add(80)).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32(((p_task_1).wrapping_add(16)).wrapping_add(8))), rt::fmul(rt::fneg(F32(c.m.r32((((l_w_2).wrapping_add(80)).wrapping_add(48)).wrapping_add(8)))), F32(c.m.r32((l_w_2).wrapping_add(228))))); let a_ = (((l_w_2).wrapping_add(80)).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    if (((((c.m.r32((p_task_1).wrapping_add(4)) as i32)) & (4i32))) != 0) {
                        { let a0_ = (((c.m.r16((l_w_2).wrapping_add(18)) as i16)) as i32); F::EftGndDust_SpawnImpact(c, a0_, ((l_w_2).wrapping_add(80)).wrapping_add(16), F32(0x3f800000)) };
                    }
                    if ((((((((c.m.r16((l_w_2).wrapping_add(248)) as i16)) as i32)) < (3i32)) as i32)) != 0) {
                        if (((((c.m.r32((p_task_1).wrapping_add(4)) as i32)) & (16i32))) != 0) {
                            F::Vec3_Scale(c, fp, ((l_w_2).wrapping_add(80)).wrapping_add(16), F32(0xbf800000));
                            F::Vec3_Normalize(c, fp, fp);
                        } else {
                            F::Vec4_Copy(c, fp, ((l_w_2).wrapping_add(144)).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
                        }
                        F::EftKiBomb_Reflect(c, fp.wrapping_add(32), l_vel_3, fp);
                        l_speed_5 = rt::fmul(F32(c.m.r32((l_w_2).wrapping_add(220))), F32(0x3f000000));
                        { let r_ = F32(0x3f000000); let a_ = (l_w_2).wrapping_add(236); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        l_r_4 = rt::fadd(F32(c.m.r32((l_w_2).wrapping_add(228))), F32(0x3f800000));
                        { let v_ = l_speed_5; let a_ = (l_w_2).wrapping_add(220); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(F32(c.m.r32((p_task_1).wrapping_add(16))), rt::fmul(F32(c.m.r32(fp)), l_r_4)); let a_ = ((l_w_2).wrapping_add(80)).wrapping_add(16); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(F32(c.m.r32(((p_task_1).wrapping_add(16)).wrapping_add(4))), rt::fmul(F32(c.m.r32((fp).wrapping_add(4))), l_r_4)); let a_ = (((l_w_2).wrapping_add(80)).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(F32(c.m.r32(((p_task_1).wrapping_add(16)).wrapping_add(8))), rt::fmul(F32(c.m.r32((fp).wrapping_add(8))), l_r_4)); let a_ = (((l_w_2).wrapping_add(80)).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fmul(F32(c.m.r32(fp.wrapping_add(32))), F32(c.m.r32((l_w_2).wrapping_add(220)))); let a_ = l_vel_3; c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4))), F32(c.m.r32((l_w_2).wrapping_add(220)))); let a_ = (((l_w_2).wrapping_add(80)).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8))), F32(c.m.r32((l_w_2).wrapping_add(220)))); let a_ = (((l_w_2).wrapping_add(80)).wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let a_ = (l_w_2).wrapping_add(248); let o_ = (c.m.r16(a_) as i16); let n_ = (o_).wrapping_add(1); c.m.w16(a_, (n_) as u16); };
                        if (((((c.m.r32((p_task_1).wrapping_add(4)) as i32)) & (4i32))) != 0) {
                            { let r_ = (!(4i32)); let a_ = (p_task_1).wrapping_add(4); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                        if (((((c.m.r32((p_task_1).wrapping_add(4)) as i32)) & (16i32))) != 0) {
                            { let r_ = (!(16i32)); let a_ = (p_task_1).wrapping_add(4); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                    } else {
                        { let v_ = F32(0x00000000); let a_ = l_vel_3; c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x00000000); let a_ = (((l_w_2).wrapping_add(80)).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x00000000); let a_ = (((l_w_2).wrapping_add(80)).wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x00000000); let a_ = (l_w_2).wrapping_add(240); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x00000000); let a_ = (l_w_2).wrapping_add(236); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(c.m.r32((p_task_1).wrapping_add(16))); let a_ = ((l_w_2).wrapping_add(80)).wrapping_add(16); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fsub(F32(c.m.r32(((p_task_1).wrapping_add(16)).wrapping_add(4))), F32(c.m.r32((l_w_2).wrapping_add(228)))); let a_ = (((l_w_2).wrapping_add(80)).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(c.m.r32(((p_task_1).wrapping_add(16)).wrapping_add(8))); let a_ = (((l_w_2).wrapping_add(80)).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        F::Vec4_Copy(c, fp, ((l_w_2).wrapping_add(144)).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
                        { let v_ = rt::fadd(F32(c.m.r32((p_task_1).wrapping_add(16))), rt::fmul(F32(c.m.r32(fp)), F32(c.m.r32((l_w_2).wrapping_add(228))))); let a_ = ((l_w_2).wrapping_add(80)).wrapping_add(16); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(F32(c.m.r32(((p_task_1).wrapping_add(16)).wrapping_add(4))), rt::fmul(F32(c.m.r32((fp).wrapping_add(4))), F32(c.m.r32((l_w_2).wrapping_add(228))))); let a_ = (((l_w_2).wrapping_add(80)).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(F32(c.m.r32(((p_task_1).wrapping_add(16)).wrapping_add(8))), rt::fmul(F32(c.m.r32((fp).wrapping_add(8))), F32(c.m.r32((l_w_2).wrapping_add(228))))); let a_ = (((l_w_2).wrapping_add(80)).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        if ((((!(((((c.m.r32((l_w_2).wrapping_add(208)) as i32)) & (512i32))) != 0)) as i32)) != 0) {
                            { let v_ = { let a0_ = rt::fdiv(rt::fmul({ let a_ = (fp.wrapping_add(16)).wrapping_add((((rt::rem_i32(F::Port_Rand(c), 3i32)) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, F32(0x40490fda)), F32(0x43340000)); F::EftMath_WrapAngle(c, a0_) }; let a_ = (l_w_2).wrapping_add(232); c.m.w32(a_, (v_).0); };
                            { let r_ = (!(4i32)); let a_ = (p_task_1).wrapping_add(4); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                        { let r_ = 512i32; let a_ = (l_w_2).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftKiBomb_HandleHits", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiBomb_UpdateModel(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiBomb_UpdateModel", &PROBE_N, &[(p_task_1) as u32], false); }
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
            { let v_ = rt::fmul(F32(c.m.r32((l_w_2).wrapping_add(224))), F32(0x40600000)); let a_ = (fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul(F32(c.m.r32((l_w_2).wrapping_add(224))), F32(0x40600000)); let a_ = ((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul(F32(c.m.r32((l_w_2).wrapping_add(224))), F32(0x40600000)); let a_ = ((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
            F::Mtx_Mul(c, fp, fp, fp.wrapping_add(64));
            { let a2_ = { let a0_ = rt::fneg(F32(c.m.r32((l_w_2).wrapping_add(232)))); F::EftMath_WrapAngle(c, a0_) }; F::Mtx_RotateX(c, fp, fp, a2_) };
            { let a1_ = F32(c.m.r32(((l_w_2).wrapping_add(80)).wrapping_add(16))); let a2_ = rt::fadd(F32(c.m.r32((((l_w_2).wrapping_add(80)).wrapping_add(16)).wrapping_add(4))), F32(0x3fc00000)); let a3_ = F32(c.m.r32((((l_w_2).wrapping_add(80)).wrapping_add(16)).wrapping_add(8))); F::Vec4_Set(c, fp.wrapping_add(128), a1_, a2_, a3_, F32(0x3f800000)) };
            F::Mtx_SetTrans(c, fp, fp.wrapping_add(128));
            { let a0_ = (c.m.r32((l_w_2).wrapping_add(300)) as i32); F::EftObj_SetMtx(c, a0_, fp) };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftKiBomb_UpdateModel", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiObj_AddHitRecord(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiObj_AddHitRecord", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_rec_3: u32 = 0;
    let mut l_a_4: u32 = 0;
    let mut l_b_5: u32 = 0;
    let mut l_radius_6: F32 = F32(0);
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_rec_3 = F::EftHit_GetNew(c);
    { let v_ = p_task_1; let a_ = (l_rec_3).wrapping_add(96); c.m.w32(a_, v_); };
    { let s_ = (l_w_2).wrapping_add(80); let d_ = (l_rec_3).wrapping_add(16); rt::memcpy(c, d_, s_, 64); };
    { let v_ = 0i32; let a_ = (l_rec_3).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((((((c.m.r32((l_w_2).wrapping_add(208)) as i32)).wrapping_shr((5i32) as u32)) & (1i32))) as u8); let a_ = (l_rec_3).wrapping_add(92); c.m.w8(a_, v_); };
    { let r_ = 1024i32; let a_ = (l_rec_3).wrapping_add(80); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    { let v_ = l_w_2; let a_ = (l_rec_3).wrapping_add(104); c.m.w32(a_, v_); };
    l_radius_6 = F32(c.m.r32((l_w_2).wrapping_add(228)));
    l_a_4 = F::EftHitArena_AllocSphere(c);
    l_b_5 = F::EftHitArena_AllocSphere(c);
    F::ColSphere_Set(c, l_a_4, ((l_w_2).wrapping_add(80)).wrapping_add(16), l_radius_6);
    F::ColSphere_Set(c, l_b_5, ((l_w_2).wrapping_add(80)).wrapping_add(32), l_radius_6);
    F::EftHit_SetShapeSpheres(c, l_rec_3, l_a_4, l_b_5);
    F::EftHit_Add(c, l_rec_3);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiObj_AddHitRecord", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiObj_Turn(c: &mut Ctx, mut p_task_1: u32, mut p_w_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiObj_Turn", &PROBE_N, &[(p_task_1) as u32, (p_w_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        F::Vec3_Lerp(c, ((p_w_2).wrapping_add(80)).wrapping_add(16), (p_task_1).wrapping_add(16), ((p_w_2).wrapping_add(80)).wrapping_add(32), F32(0x3f333333));
        if (((((c.m.r32((p_task_1).wrapping_add(4)) as i32)) & (64i32))) != 0) {
            { let a0_ = (((((((((c.m.r16((p_w_2).wrapping_add(18)) as i16)) as i32)) ^ (1i32))) as i16)) as i32); F::BtlCharApi_GetDeflectDir(c, a0_, fp) };
            F::Vec4_Copy(c, ((p_w_2).wrapping_add(80)).wrapping_add(48), fp);
            { let r_ = 1i32; let a_ = (p_w_2).wrapping_add(18); let n_ = (((((((c.m.r16(a_) as i16)) as i32)) ^ (r_))) as i16); c.m.w16(a_, (n_) as u16); };
            { let r_ = 4i32; let a_ = (p_w_2).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        } else {
            if (((((c.m.r32((p_task_1).wrapping_add(4)) as i32)) & (128i32))) != 0) {
                { let a0_ = (((((((((c.m.r16((p_w_2).wrapping_add(18)) as i16)) as i32)) ^ (1i32))) as i16)) as i32); F::BtlCharApi_GetDeflectDir(c, a0_, fp) };
                F::Vec4_Copy(c, ((p_w_2).wrapping_add(80)).wrapping_add(48), fp);
                { let r_ = 1i32; let a_ = (p_w_2).wrapping_add(18); let n_ = (((((((c.m.r16(a_) as i16)) as i32)) ^ (r_))) as i16); c.m.w16(a_, (n_) as u16); };
                { let v_ = (((c.m.r16((p_w_2).wrapping_add(28)) as i16)) as i32); let a_ = (p_w_2).wrapping_add(212); c.m.w32(a_, (v_) as u32); };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftKiObj_Turn", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiObj_Launch(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiObj_Launch", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_w_2: u32 = 0;
    let mut l_res_3: u32 = 0;
    let mut l_spin_4: F32 = F32(0);
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        { let a0_ = (((c.m.r16((l_w_2).wrapping_add(18)) as i16)) as i32); let a1_ = (((c.m.r16((l_w_2).wrapping_add(20)) as i16)) as i32); F::BtlCharApi_GetNodePos(c, a0_, a1_, (l_w_2).wrapping_add(80)) };
        { let a0_ = (((c.m.r16((l_w_2).wrapping_add(18)) as i16)) as i32); let a1_ = (((c.m.r16((l_w_2).wrapping_add(20)) as i16)) as i32); F::BtlCharApi_GetNodePos(c, a0_, a1_, ((l_w_2).wrapping_add(80)).wrapping_add(16)) };
        { let a0_ = (((c.m.r16((l_w_2).wrapping_add(18)) as i16)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 17i32, ((l_w_2).wrapping_add(80)).wrapping_add(32)) };
        F::Vec4_Copy(c, ((l_w_2).wrapping_add(80)).wrapping_add(48), l_w_2);
        { let v_ = F32(c.m.r32((l_w_2).wrapping_add(40))); let a_ = (l_w_2).wrapping_add(224); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((l_w_2).wrapping_add(32))); let a_ = (l_w_2).wrapping_add(220); c.m.w32(a_, (v_).0); };
        { let v_ = (((c.m.r16((l_w_2).wrapping_add(28)) as i16)) as i32); let a_ = (l_w_2).wrapping_add(212); c.m.w32(a_, (v_) as u32); };
        { let v_ = rt::fmul(F32(c.m.r32((l_w_2).wrapping_add(40))), F32(0x40a00000)); let a_ = (l_w_2).wrapping_add(228); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f000000); let a_ = (l_w_2).wrapping_add(240); c.m.w32(a_, (v_).0); };
        { let v_ = ((0i32) as i16); let a_ = (l_w_2).wrapping_add(250); c.m.w16(a_, (v_) as u16); };
        { let v_ = ((0i32) as i16); let a_ = (l_w_2).wrapping_add(246); c.m.w16(a_, (v_) as u16); };
        l_spin_4 = rt::fadd(rt::fsub(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x41200000)), F32(0x40a00000)), F32(0x42000000));
        { let v_ = l_spin_4; let a_ = (l_w_2).wrapping_add(236); c.m.w32(a_, (v_).0); };
        { let v_ = (if (((rt::flt(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3f000000)) as i32)) != 0) { l_spin_4 } else { rt::fneg(l_spin_4) }); let a_ = (l_w_2).wrapping_add(236); c.m.w32(a_, (v_).0); };
        { let a2_ = F32(c.m.r32((l_w_2).wrapping_add(220))); F::Vec3_Scale(c, ((l_w_2).wrapping_add(80)).wrapping_add(48), ((l_w_2).wrapping_add(80)).wrapping_add(48), a2_) };
        { let v_ = 0i32; let a_ = (l_w_2).wrapping_add(216); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((15i32) as i16); let a_ = (l_w_2).wrapping_add(468); c.m.w16(a_, (v_) as u16); };
        l_res_3 = { let a_ = { let a_ = (F::BtlTask_GetParent(c, p_task_1)).wrapping_add(56); c.m.r32(a_) }; c.m.r32(a_) };
        F::Mtx_StoreIdentity(c, fp);
        { let v_ = { let a1_ = c.m.r32((l_res_3).wrapping_add(80)); F::EftObj_Create(c, (l_w_2).wrapping_add(252), a1_) }; let a_ = (l_w_2).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
        if ((((((c.m.r32((l_w_2).wrapping_add(300)) as i32)) != ((1i32).wrapping_neg())) as i32)) != 0) {
            F::Mtx_SetTrans(c, fp, ((l_w_2).wrapping_add(80)).wrapping_add(16));
            { let a0_ = (c.m.r32((l_w_2).wrapping_add(300)) as i32); F::EftObj_SetMtx(c, a0_, fp) };
        }
        { let v_ = (l_res_3).wrapping_add(((8i32) as u32)); let a_ = (l_w_2).wrapping_add(464); c.m.w32(a_, v_); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftKiObj_Launch", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiObj_StepTex(c: &mut Ctx, mut p_tex_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiObj_StepTex", &PROBE_N, &[(p_tex_1) as u32], false); }
    let mut l_a_2: u64 = 0;
    let mut l_b_3: u64 = 0;
    let r_ = 'f: {
    if (((((((((p_tex_1) != (((0i32) as u32))) as i32)) != 0) && ((((!(((((c.m.r32((p_tex_1).wrapping_add(68)) as i32)) & (1i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
        l_a_2 = (((((F::EftVram_AddImage(c, p_tex_1, 1i32, 0i32))) as i64)) as u64);
        l_b_3 = (((((F::EftVram_AddClut(c, p_tex_1))) as i64)) as u64);
        { let v_ = ((l_a_2) | ((l_b_3).wrapping_shl((37i32) as u32))); let a_ = p_tex_1; c.m.w64(a_, v_); };
        { let r_ = 1i32; let a_ = (p_tex_1).wrapping_add(68); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiObj_StepTex", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiObj_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiObj_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let _ = F::memset(c, l_w_3, 0i32, 480u32); };
    { let s_ = p_arg_2; let d_ = l_w_3; rt::memcpy(c, d_, s_, 80); };
    { let v_ = (c.m.r16((p_arg_2).wrapping_add(18)) as i16); let a_ = (l_w_3).wrapping_add(16); c.m.w16(a_, (v_) as u16); };
    F::EftKiObj_Launch(c, p_task_1);
    { let r_ = ((64i32) | (128i32)); let a_ = (l_w_3).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    { let a1_ = (if ((((((((c.m.r16((p_arg_2).wrapping_add(18)) as i16)) as i32)) == (0i32)) as i32)) != 0) { 2048i32 } else { 4096i32 }); F::BtlTask_SetOwnerTag(c, p_task_1, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiObj_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiObj_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiObj_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((((c.m.r32((l_w_2).wrapping_add(300)) as i32)) != ((1i32).wrapping_neg())) as i32)) != 0) {
        { let a0_ = (c.m.r32((l_w_2).wrapping_add(300)) as i32); { let _ = F::EftObj_Destroy(c, a0_); } };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiObj_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftKiObj_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftKiObj_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_dead_3: i32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!((F::BtlScene_IsAnyCharStopped(c)) != 0)) as i32)) != 0) {
        { let _ = F::EftKiObj_Step(c, p_task_1); };
        if (((((c.m.r32((l_w_2).wrapping_add(208)) as i32)) & (128i32))) != 0) {
            F::EftKiObj_AddHitRecord(c, p_task_1);
        }
        if (((((c.m.r32((l_w_2).wrapping_add(208)) as i32)) & (8i32))) != 0) {
            F::BtlTask_SetDead(c, p_task_1);
            l_dead_3 = 1i32;
        } else {
            { let a0_ = c.m.r32((l_w_2).wrapping_add(464)); F::EftKiObj_StepTex(c, a0_) };
            l_dead_3 = 0i32;
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftKiObj_Update", &PROBE_N, &[], true); }
    r_
}
