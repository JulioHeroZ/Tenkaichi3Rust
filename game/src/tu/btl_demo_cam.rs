#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn DemoCam_FixupAnim(c: &mut Ctx, mut p_anim_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_FixupAnim", &PROBE_N, &[(p_anim_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let mut l_ch_3: u32 = 0;
    let r_ = 'f: {
    if ((((((c.m.r32((p_anim_1).wrapping_add(4)) as i32)) != (0i32)) as i32)) != 0) {
        { let v_ = (c.m.r32((p_anim_1).wrapping_add(8))).wrapping_add(((((p_anim_1) as i32)) as u32)); let a_ = (p_anim_1).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_anim_1).wrapping_add(24))).wrapping_add(((((p_anim_1) as i32)) as u32)); let a_ = (p_anim_1).wrapping_add(24); c.m.w32(a_, v_); };
        if (((((c.m.r32((p_anim_1).wrapping_add(28))) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = (c.m.r32((p_anim_1).wrapping_add(28))).wrapping_add(((((p_anim_1) as i32)) as u32)); let a_ = (p_anim_1).wrapping_add(28); c.m.w32(a_, v_); };
        }
        if ((((((c.m.r32((p_anim_1).wrapping_add(32)) as i32)) < (2i32)) as i32)) != 0) {
            { let v_ = ((0i32) as u32); let a_ = (p_anim_1).wrapping_add(28); c.m.w32(a_, v_); };
        }
        l_ch_3 = c.m.r32((p_anim_1).wrapping_add(8));
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < ((c.m.r32((p_anim_1).wrapping_add(12)) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = (c.m.r32((l_ch_3).wrapping_add(4))).wrapping_add(((((p_anim_1) as i32)) as u32)); let a_ = (l_ch_3).wrapping_add(4); c.m.w32(a_, v_); };
                    { let o_ = l_ch_3; let n_ = (o_).wrapping_add(16); l_ch_3 = n_; };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = 0i32; let a_ = (p_anim_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_FixupAnim", &PROBE_N, &[], true); }
    r_
}

pub fn DemoCam_EvalAnim(c: &mut Ctx, mut p_out_1: u32, mut p_time_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_EvalAnim", &PROBE_N, &[(p_out_1) as u32, (p_time_2).0], false); }
    let mut l_anim_3: u32 = 0;
    let mut l_ch_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_key_6: u32 = 0;
    let mut l_prev_7: u32 = 0;
    let mut l_next_8: u32 = 0;
    let mut l_j_9: i32 = 0;
    let r_ = 'f: {
    l_anim_3 = { let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(704); c.m.r32(a_) };
    l_ch_4 = c.m.r32((l_anim_3).wrapping_add(8));
    { let s_ = c.m.r32((l_anim_3).wrapping_add(24)); let d_ = p_out_1; rt::memcpy(c, d_, s_, 32); };
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((l_i_5) < ((c.m.r32((l_anim_3).wrapping_add(12)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                l_key_6 = c.m.r32((l_ch_4).wrapping_add(4));
                l_prev_7 = ((0i32) as u32);
                l_next_8 = ((0i32) as u32);
                {
                    l_j_9 = 0i32;
                    'l3: loop {
                        if !(((((l_j_9) < ((c.m.r32((l_ch_4).wrapping_add(8)) as i32))) as i32)) != 0) { break 'l3; }
                        'c4: {
                            if (((rt::feq(F32(c.m.r32((l_key_6).wrapping_add(4))), p_time_2) as i32)) != 0) {
                                { let v_ = rt::f2i(F32(c.m.r32((l_key_6).wrapping_add(4)))); let a_ = (p_out_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                                l_prev_7 = l_key_6;
                                break 'l3;
                            }
                            if (((rt::flt(F32(c.m.r32((l_key_6).wrapping_add(4))), p_time_2) as i32)) != 0) {
                                l_prev_7 = l_key_6;
                            }
                            if (((rt::flt(p_time_2, F32(c.m.r32((l_key_6).wrapping_add(4)))) as i32)) != 0) {
                                l_next_8 = l_key_6;
                                break 'l3;
                            }
                            { let o_ = l_key_6; let n_ = (o_).wrapping_add(32); l_key_6 = n_; };
                        }
                        { let o_ = l_j_9; let n_ = (o_).wrapping_add(1); l_j_9 = n_; };
                    }
                }
                if (((((((((l_next_8) != (((0i32) as u32))) as i32)) != 0) && (((((l_prev_7) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
                    if ((((c.m.r32(l_prev_7)) & (((16i32) as u32)))) != 0) {
                        { let v_ = F32(c.m.r32((l_prev_7).wrapping_add(8))); let a_ = (p_out_1).wrapping_add(((((c.m.r32(l_ch_4) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    } else {
                        { let v_ = rt::fadd(rt::fmul(rt::fsub(F32(c.m.r32((l_next_8).wrapping_add(8))), F32(c.m.r32((l_prev_7).wrapping_add(8)))), rt::fdiv(rt::fsub(p_time_2, F32(c.m.r32((l_prev_7).wrapping_add(4)))), rt::fsub(F32(c.m.r32((l_next_8).wrapping_add(4))), F32(c.m.r32((l_prev_7).wrapping_add(4)))))), F32(c.m.r32((l_prev_7).wrapping_add(8)))); let a_ = (p_out_1).wrapping_add(((((c.m.r32(l_ch_4) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    }
                } else {
                    if (((((l_prev_7) != (((0i32) as u32))) as i32)) != 0) {
                        { let v_ = F32(c.m.r32((l_prev_7).wrapping_add(8))); let a_ = (p_out_1).wrapping_add(((((c.m.r32(l_ch_4) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    } else {
                        if (((((l_next_8) != (((0i32) as u32))) as i32)) != 0) {
                            { let v_ = F32(c.m.r32((l_next_8).wrapping_add(8))); let a_ = (p_out_1).wrapping_add(((((c.m.r32(l_ch_4) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                        }
                    }
                }
                { let o_ = l_ch_4; let n_ = (o_).wrapping_add(16); l_ch_4 = n_; };
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_EvalAnim", &PROBE_N, &[], true); }
    r_
}

pub fn DemoCam_Get(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_Get", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f c.m.r32(sym::gDemoCam);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_Get", &PROBE_N, &[], true); }
    r_
}

pub fn DemoCam_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_Reset", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let _ = { let a0_ = (c.m.r32(sym::gDemoCam)).wrapping_add(608); F::memset(c, a0_, 0i32, ((224i32) as u32)) }; };
    { let a0_ = c.m.r32(sym::gDemoCam); F::View_InitLayout(c, a0_, 0i32) };
    { let a0_ = (c.m.r32(sym::gDemoCam)).wrapping_add(608); F::Mtx_StoreIdentity(c, a0_) };
    { let a0_ = (c.m.r32(sym::gDemoCam)).wrapping_add(720); F::Vec4_SetZeroW1(c, a0_) };
    { let a0_ = (c.m.r32(sym::gDemoCam)).wrapping_add(736); F::Vec4_SetZeroW1(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn DemoCam_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, ((832u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gDemoCam; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gDemoCam); F::memset(c, a0_, 0i32, 832u32) }; };
    { let a0_ = c.m.r32(sym::gDemoCam); F::View_InitLayout(c, a0_, 0i32) };
    F::DemoCam_Reset(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_Init", &PROBE_N, &[], true); }
    r_
}

pub fn DemoCam_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32(sym::gDemoCam); F::Heap_Free(c, a0_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gDemoCam; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_Term", &PROBE_N, &[], true); }
    r_
}

pub fn DemoCam_SetAnim(c: &mut Ctx, mut p_anim_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_SetAnim", &PROBE_N, &[(p_anim_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_anim_1; let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(704); c.m.w32(a_, v_); };
    { let a0_ = { let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(704); c.m.r32(a_) }; F::DemoCam_FixupAnim(c, a0_) };
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(760); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(764); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(768); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(772); c.m.w32(a_, v_); };
    { let a0_ = (c.m.r32(sym::gDemoCam)).wrapping_add(608); F::Mtx_StoreIdentity(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_SetAnim", &PROBE_N, &[], true); }
    r_
}

pub fn DemoCam_Update(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_Update", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(304) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 304);
    let mut l_pose_1: u32 = 0;
    let mut l_owner_2: u32 = 0;
    let mut l_t_3: F32 = F32(0);
    let mut l_i_4: i32 = 0;
    let mut l_a_5: u32 = 0;
    let mut l_scale_6: F32 = F32(0);
    let r_ = 'f: {
        if ((((({ let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(812); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
            { let _ = F::memset(c, fp, 0i32, 16u32); };
            { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
            { let _ = F::memset(c, fp.wrapping_add(16), 0i32, 16u32); };
            { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            { let _ = F::memset(c, fp.wrapping_add(32), 0i32, 16u32); };
            { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            { let _ = F::memset(c, fp.wrapping_add(48), 0i32, 16u32); };
            { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            { let a1_ = (c.m.r32(sym::gDemoCam)).wrapping_add(720); F::Vec4_Copy(c, fp.wrapping_add(32), a1_) };
            { let a1_ = (c.m.r32(sym::gDemoCam)).wrapping_add(736); F::Vec4_Copy(c, fp.wrapping_add(48), a1_) };
            { let a0_ = (c.m.r32(sym::gDemoCam)).wrapping_add(780); F::CamShake_Calc(c, a0_, fp, fp.wrapping_add(16)) };
            { let r_ = F32(c.m.r32(fp)); let a_ = fp.wrapping_add(32); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = F32(c.m.r32((fp).wrapping_add(4))); let a_ = (fp.wrapping_add(32)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = F32(c.m.r32((fp).wrapping_add(8))); let a_ = (fp.wrapping_add(32)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = F32(c.m.r32(fp.wrapping_add(16))); let a_ = fp.wrapping_add(48); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); let a_ = (fp.wrapping_add(48)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); let a_ = (fp.wrapping_add(48)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let a0_ = (c.m.r32(sym::gDemoCam)).wrapping_add(780); F::CamShake_Tick(c, a0_) };
            { let a0_ = c.m.r32(sym::gDemoCam); let a1_ = (c.m.r32(sym::gDemoCam)).wrapping_add(64); F::View_SetTransform(c, a0_, a1_, fp.wrapping_add(32), fp.wrapping_add(48)) };
            { let a0_ = c.m.r32(sym::gDemoCam); F::View_UpdateMatrices(c, a0_, fp.wrapping_add(32)) };
            { let a0_ = c.m.r32(sym::gDemoCam); F::View_Apply(c, a0_, 1i32) };
            break 'f 0i32;
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(704); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
                break 'f 1i32;
            }
            if (((({ let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(776); (c.m.r32(a_) as i32) }) & (4i32))) != 0) {
                if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
                    l_t_3 = rt::fadd(F::DemoCam_GetTime(c), F32(0x40000000));
                    if (((rt::flt(F::DemoCam_GetLength(c), l_t_3) as i32)) != 0) {
                        if (((({ let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(776); (c.m.r32(a_) as i32) }) & (8i32))) != 0) {
                            F::DemoCam_Stop(c);
                        }
                        { let a0_ = F::DemoCam_GetLength(c); F::DemoCam_SetTime(c, a0_) };
                    } else {
                        F::DemoCam_SetTime(c, l_t_3);
                    }
                }
            }
            if ((((({ let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(768); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
                { let a0_ = ({ let a0_ = { let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(768); c.m.r32(a_) }; F::BtlObj_GetNode(c, a0_, 0i32) }).wrapping_add(((16i32) as u32)); F::DemoCam_SetBase(c, a0_) };
            } else {
                if ((((({ let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(772); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
                    { let a0_ = ({ let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(772); c.m.r32(a_) }).wrapping_add(((2464i32) as u32)); F::DemoCam_SetBase(c, a0_) };
                }
            }
            l_pose_1 = (c.m.r32(sym::gDemoCam)).wrapping_add(672);
            { let a1_ = { let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(752); F32(c.m.r32(a_)) }; F::DemoCam_EvalAnim(c, l_pose_1, a1_) };
            { let _ = F::memset(c, fp.wrapping_add(208), 0i32, 16u32); };
            { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(208)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            { let a1_ = F32(c.m.r32(((l_pose_1).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a2_ = F32(c.m.r32(((l_pose_1).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a3_ = F32(c.m.r32(((l_pose_1).wrapping_add(12)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); F::Vec4_Set(c, fp.wrapping_add(192), a1_, a2_, a3_, F32(0x00000000)) };
            { let a0_ = (c.m.r32(sym::gDemoCam)).wrapping_add(780); F::CamShake_Calc(c, a0_, fp.wrapping_add(208), fp.wrapping_add(224)) };
            { let r_ = F32(c.m.r32(fp.wrapping_add(224))); let a_ = fp.wrapping_add(192); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = F32(c.m.r32((fp.wrapping_add(224)).wrapping_add(4))); let a_ = (fp.wrapping_add(192)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = F32(c.m.r32((fp.wrapping_add(224)).wrapping_add(8))); let a_ = (fp.wrapping_add(192)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let a0_ = (c.m.r32(sym::gDemoCam)).wrapping_add(780); F::CamShake_Tick(c, a0_) };
            {
                l_i_4 = 0i32;
                'l1: loop {
                    if !(((((l_i_4) < (3i32)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        l_a_5 = (fp.wrapping_add(192)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4));
                        if (((rt::fgt(F32(c.m.r32(l_a_5)), F32(0x40490fda)) as i32)) != 0) {
                            { let r_ = F32(0x40c90fda); let a_ = l_a_5; let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        }
                        if (((rt::flt(F32(c.m.r32(l_a_5)), F32(0xc0490fda)) as i32)) != 0) {
                            { let r_ = F32(0x40c90fda); let a_ = l_a_5; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        }
                        if (((rt::fgt(F32(c.m.r32(l_a_5)), F32(0x40490fda)) as i32)) != 0) {
                            { let r_ = F32(0x40c90fda); let a_ = l_a_5; let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        }
                        if (((rt::flt(F32(c.m.r32(l_a_5)), F32(0xc0490fda)) as i32)) != 0) {
                            { let r_ = F32(0x40c90fda); let a_ = l_a_5; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        }
                    }
                    { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                }
            }
            { let a0_ = (c.m.r32(sym::gDemoCam)).wrapping_add(64); F::Mtx_StoreIdentity(c, a0_) };
            F::Mtx_StoreIdentity(c, fp.wrapping_add(128));
            { let a2_ = F32(c.m.r32(fp.wrapping_add(192))); F::Mtx_RotateX(c, fp.wrapping_add(128), fp.wrapping_add(128), a2_) };
            { let a0_ = (c.m.r32(sym::gDemoCam)).wrapping_add(64); let a2_ = (c.m.r32(sym::gDemoCam)).wrapping_add(64); F::Mtx_Mul(c, a0_, fp.wrapping_add(128), a2_) };
            F::Mtx_StoreIdentity(c, fp.wrapping_add(128));
            { let a2_ = F32(c.m.r32((fp.wrapping_add(192)).wrapping_add(4))); F::Mtx_RotateY(c, fp.wrapping_add(128), fp.wrapping_add(128), a2_) };
            { let a0_ = (c.m.r32(sym::gDemoCam)).wrapping_add(64); let a2_ = (c.m.r32(sym::gDemoCam)).wrapping_add(64); F::Mtx_Mul(c, a0_, fp.wrapping_add(128), a2_) };
            F::Mtx_StoreIdentity(c, fp.wrapping_add(128));
            { let a2_ = F32(c.m.r32((fp.wrapping_add(192)).wrapping_add(8))); F::Mtx_RotateZ(c, fp.wrapping_add(128), fp.wrapping_add(128), a2_) };
            { let a0_ = (c.m.r32(sym::gDemoCam)).wrapping_add(64); let a2_ = (c.m.r32(sym::gDemoCam)).wrapping_add(64); F::Mtx_Mul(c, a0_, fp.wrapping_add(128), a2_) };
            if ((((({ let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(768); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
                l_owner_2 = { let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(768); c.m.r32(a_) };
            } else {
                l_owner_2 = { let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(772); c.m.r32(a_) };
            }
            if (((((((((((((l_owner_2) != (((0i32) as u32))) as i32)) != 0) && ((((({ let a0_ = { let a0_ = c.m.r32((l_owner_2).wrapping_add(((16i32) as u32))); ((((F::BtlCharApi_GetPlayer(c, ((((a0_) as u64)) as i32))) as i64 as u64)) as u32) }; F::BtlCharApi_HasMemberUnk70(c, ((((a0_) as u64)) as i32)) }) != (0i32)) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(816); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0)) as i32)) != 0) {
                l_scale_6 = rt::fdiv({ let a_ = (c.m.r32((l_owner_2).wrapping_add(((2332i32) as u32)))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, F32(0x3f8ccccc));
                { let v_ = rt::fadd(F32(c.m.r32((l_pose_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(fp.wrapping_add(208)))); let a_ = (((c.m.r32(sym::gDemoCam)).wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(rt::fmul(F32(c.m.r32((l_pose_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_scale_6), F32(c.m.r32((fp.wrapping_add(208)).wrapping_add(4)))); let a_ = (((c.m.r32(sym::gDemoCam)).wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((l_pose_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(208)).wrapping_add(8)))); let a_ = (((c.m.r32(sym::gDemoCam)).wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            } else {
                { let v_ = rt::fadd(F32(c.m.r32((l_pose_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(fp.wrapping_add(208)))); let a_ = (((c.m.r32(sym::gDemoCam)).wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((l_pose_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(208)).wrapping_add(4)))); let a_ = (((c.m.r32(sym::gDemoCam)).wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((l_pose_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(208)).wrapping_add(8)))); let a_ = (((c.m.r32(sym::gDemoCam)).wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            }
            { let a0_ = (c.m.r32(sym::gDemoCam)).wrapping_add(64); let a1_ = (c.m.r32(sym::gDemoCam)).wrapping_add(608); let a2_ = (c.m.r32(sym::gDemoCam)).wrapping_add(64); F::Mtx_Mul(c, a0_, a1_, a2_) };
            { let a0_ = (c.m.r32(sym::gDemoCam)).wrapping_add(64); let a1_ = (c.m.r32(sym::gDemoCam)).wrapping_add(64); F::Mtx_InverseRT(c, a0_, a1_) };
            { let a0_ = (c.m.r32(sym::gDemoCam)).wrapping_add(64); F::Vu0View_LoadMtx(c, a0_) };
            { let a1_ = (c.m.r32(sym::gDemoCam)).wrapping_add(64); F::Mtx_InverseRT(c, fp.wrapping_add(64), a1_) };
            if ((((((((({ let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(768); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) || ((((({ let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(772); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
                if (((((F::BtlStage_IsReady(c)) != (0i32)) as i32)) != 0) {
                    F::Vec4_Copy(c, fp.wrapping_add(240), (fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
                    if ((((({ let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(768); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
                        { let a0_ = { let a_ = ({ let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(768); c.m.r32(a_) }).wrapping_add(((16i32) as u32)); c.m.r32(a_) }; F::BtlCharApi_GetCamUnk460(c, ((((a0_) as u64)) as i32), fp.wrapping_add(256)) };
                    } else {
                        { let a0_ = { let a_ = ({ let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(772); c.m.r32(a_) }).wrapping_add(((16i32) as u32)); c.m.r32(a_) }; F::BtlCharApi_GetCamUnk460(c, ((((a0_) as u64)) as i32), fp.wrapping_add(256)) };
                    }
                    if (((((F::BtlCam_TraceStage(c, fp.wrapping_add(272), fp.wrapping_add(240), fp.wrapping_add(256), fp.wrapping_add(288), ((0i32) as u32))) != (0i32)) as i32)) != 0) {
                        F::Vec4_Copy(c, (fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), fp.wrapping_add(272));
                        { let a0_ = (c.m.r32(sym::gDemoCam)).wrapping_add(64); F::Mtx_InverseRT(c, a0_, fp.wrapping_add(64)) };
                        { let a0_ = (c.m.r32(sym::gDemoCam)).wrapping_add(64); F::Vu0View_LoadMtx(c, a0_) };
                    }
                }
            }
            { let a0_ = c.m.r32(sym::gDemoCam); F::View_UpdateMatrices(c, a0_, (fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) };
            { let a0_ = c.m.r32(sym::gDemoCam); F::View_Apply(c, a0_, 1i32) };
            break 'f 0i32;
        }
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_Update", &PROBE_N, &[], true); }
    r_
}

pub fn DemoCam_SetBase(c: &mut Ctx, mut p_base_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_SetBase", &PROBE_N, &[(p_base_1) as u32], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(704); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = (c.m.r32(sym::gDemoCam)).wrapping_add(608); F::Mtx_Copy(c, a0_, p_base_1) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_SetBase", &PROBE_N, &[], true); }
    r_
}

pub fn DemoCam_GetLength(c: &mut Ctx) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_GetLength", &PROBE_N, &[], false); }
    let mut l_anim_1: u32 = 0;
    let mut l_ret_2: F32 = F32(0);
    let r_ = 'f: {
    l_anim_1 = { let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(704); c.m.r32(a_) };
    l_ret_2 = F32(0x00000000);
    if (((((l_anim_1) != (((0i32) as u32))) as i32)) != 0) {
        l_ret_2 = F32(c.m.r32((l_anim_1).wrapping_add(20)));
    }
    break 'f l_ret_2;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_GetLength", &PROBE_N, &[], true); }
    r_
}

pub fn DemoCam_GetTime(c: &mut Ctx) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_GetTime", &PROBE_N, &[], false); }
    let mut l_ret_1: F32 = F32(0);
    let r_ = 'f: {
    l_ret_1 = F32(0x00000000);
    if ((((({ let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(704); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        l_ret_1 = { let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(752); F32(c.m.r32(a_)) };
    }
    break 'f l_ret_1;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_GetTime", &PROBE_N, &[], true); }
    r_
}

pub fn DemoCam_SetTime(c: &mut Ctx, mut p_time_1: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_SetTime", &PROBE_N, &[(p_time_1).0], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(704); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        if ((((!(((rt::fle({ let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(752); F32(c.m.r32(a_)) }, p_time_1) as i32)) != 0)) as i32)) != 0) {
            { let v_ = p_time_1; let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(756); c.m.w32(a_, (v_).0); };
        } else {
            { let v_ = { let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(752); F32(c.m.r32(a_)) }; let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(756); c.m.w32(a_, (v_).0); };
        }
        { let v_ = p_time_1; let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(752); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_SetTime", &PROBE_N, &[], true); }
    r_
}

pub fn DemoCam_IsActive(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_IsActive", &PROBE_N, &[], false); }
    let mut l_playing_1: i32 = 0;
    let mut l_flags_2: i32 = 0;
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(812); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    if ((((({ let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(704); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        l_flags_2 = { let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(776); (c.m.r32(a_) as i32) };
        l_playing_1 = ((l_flags_2) & (1i32));
        { let r_ = 2i32; let n_ = ((l_flags_2) & (r_)); l_flags_2 = n_; };
        if ((l_flags_2) != 0) {
            F::DemoCam_Reset(c);
        } else {
            if ((l_playing_1) != 0) {
                break 'f 1i32;
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_IsActive", &PROBE_N, &[], true); }
    r_
}

pub fn DemoCam_IsInUse(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_IsInUse", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(704); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f ((({ let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(812); (c.m.r32(a_) as i32) }) != (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_IsInUse", &PROBE_N, &[], true); }
    r_
}

pub fn DemoCam_Start(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_Start", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(704); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(812); c.m.w32(a_, (v_) as u32); };
        { let v_ = 1i32; let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(776); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_Start", &PROBE_N, &[], true); }
    r_
}

pub fn DemoCam_StartAuto(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_StartAuto", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(704); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(812); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((1i32) | (4i32)); let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(776); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_StartAuto", &PROBE_N, &[], true); }
    r_
}

pub fn DemoCam_StartAutoOnce(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_StartAutoOnce", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(704); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(812); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((((1i32) | (4i32))) | (8i32)); let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(776); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_StartAutoOnce", &PROBE_N, &[], true); }
    r_
}

pub fn DemoCam_Stop(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_Stop", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(704); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let r_ = 2i32; let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(776); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_Stop", &PROBE_N, &[], true); }
    r_
}

pub fn DemoCam_SetObj(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_SetObj", &PROBE_N, &[(p_obj_1) as u32], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(704); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = p_obj_1; let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(768); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(772); c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_SetObj", &PROBE_N, &[], true); }
    r_
}

pub fn DemoCam_SetChr(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_SetChr", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(704); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = p_chr_1; let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(772); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(768); c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_SetChr", &PROBE_N, &[], true); }
    r_
}

pub fn DemoCam_AddShake(c: &mut Ctx, mut p_time_1: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_AddShake", &PROBE_N, &[(p_time_1).0], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32(sym::gDemoCam)).wrapping_add(780); F::CamShake_Add(c, a0_, F32(0x3f800000), p_time_1) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_AddShake", &PROBE_N, &[], true); }
    r_
}

pub fn DemoCam_GetShakeTime(c: &mut Ctx) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_GetShakeTime", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a0_ = (c.m.r32(sym::gDemoCam)).wrapping_add(780); F::CamShake_GetTime(c, a0_) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_GetShakeTime", &PROBE_N, &[], true); }
    r_
}

pub fn DemoCam_SetFixedPose(c: &mut Ctx, mut p_pos_1: u32, mut p_rot_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_SetFixedPose", &PROBE_N, &[(p_pos_1) as u32, (p_rot_2) as u32], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(704); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let r_ = 2i32; let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(776); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    { let v_ = 1i32; let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(812); c.m.w32(a_, (v_) as u32); };
    { let a0_ = (c.m.r32(sym::gDemoCam)).wrapping_add(720); F::Vec4_Copy(c, a0_, p_pos_1) };
    { let a0_ = (c.m.r32(sym::gDemoCam)).wrapping_add(736); F::Vec4_Copy(c, a0_, p_rot_2) };
    { let v_ = F32(0x3f800000); let a_ = ((c.m.r32(sym::gDemoCam)).wrapping_add(720)).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = ((c.m.r32(sym::gDemoCam)).wrapping_add(736)).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_SetFixedPose", &PROBE_N, &[], true); }
    r_
}

pub fn DemoCam_GetFixedPose(c: &mut Ctx, mut p_pos_1: u32, mut p_rot_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_GetFixedPose", &PROBE_N, &[(p_pos_1) as u32, (p_rot_2) as u32], false); }
    let r_ = 'f: {
    { let a1_ = (c.m.r32(sym::gDemoCam)).wrapping_add(720); F::Vec4_Copy(c, p_pos_1, a1_) };
    { let a1_ = (c.m.r32(sym::gDemoCam)).wrapping_add(736); F::Vec4_Copy(c, p_rot_2, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_GetFixedPose", &PROBE_N, &[], true); }
    r_
}

pub fn DemoCam_SetScaleHeight(c: &mut Ctx, mut p_on_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_SetScaleHeight", &PROBE_N, &[(p_on_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_on_1; let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(816); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_SetScaleHeight", &PROBE_N, &[], true); }
    r_
}

pub fn DemoCam_ClearFixed(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_ClearFixed", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(812); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_ClearFixed", &PROBE_N, &[], true); }
    r_
}

pub fn DemoCam_IsFixed(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_IsFixed", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = (c.m.r32(sym::gDemoCam)).wrapping_add(812); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_IsFixed", &PROBE_N, &[], true); }
    r_
}

pub fn DemoCam_PlayObjAnim(c: &mut Ctx, mut p_objId_1: i32, mut p_idx_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_PlayObjAnim", &PROBE_N, &[(p_objId_1) as u32, (p_idx_2) as u32], false); }
    let mut l_obj_3: u32 = 0;
    let mut l_anim_4: u32 = 0;
    let r_ = 'f: {
    l_obj_3 = F::BtlObj_Get(c, p_objId_1);
    l_anim_4 = c.m.r32(((l_obj_3).wrapping_add(((176i32) as u32))).wrapping_add((((p_idx_2) as u32)).wrapping_mul(4)));
    if (((((l_anim_4) != (((0i32) as u32))) as i32)) != 0) {
        F::DemoCam_SetAnim(c, l_anim_4);
        F::DemoCam_SetObj(c, l_obj_3);
        F::DemoCam_SetTime(c, F32(0x00000000));
        F::DemoCam_StartAuto(c);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_PlayObjAnim", &PROBE_N, &[], true); }
    r_
}

pub fn DemoCam_PlayCharAnim0(c: &mut Ctx, mut p_objId_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_PlayCharAnim0", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let mut l_anim_3: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlObj_Get(c, p_objId_1);
    l_anim_3 = c.m.r32((l_obj_2).wrapping_add(((2312i32) as u32)));
    if (((((l_anim_3) != (((0i32) as u32))) as i32)) != 0) {
        F::DemoCam_SetAnim(c, l_anim_3);
        F::DemoCam_SetChr(c, l_obj_2);
        F::DemoCam_SetTime(c, F32(0x00000000));
        F::DemoCam_StartAuto(c);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_PlayCharAnim0", &PROBE_N, &[], true); }
    r_
}

pub fn DemoCam_PlayCharAnim1(c: &mut Ctx, mut p_objId_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_PlayCharAnim1", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let mut l_anim_3: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlObj_Get(c, p_objId_1);
    l_anim_3 = c.m.r32((l_obj_2).wrapping_add(((2316i32) as u32)));
    if (((((l_anim_3) != (((0i32) as u32))) as i32)) != 0) {
        F::DemoCam_SetAnim(c, l_anim_3);
        F::DemoCam_SetChr(c, l_obj_2);
        F::DemoCam_SetTime(c, F32(0x00000000));
        F::DemoCam_StartAuto(c);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_PlayCharAnim1", &PROBE_N, &[], true); }
    r_
}

pub fn DemoCam_PlayCharAnim2(c: &mut Ctx, mut p_objId_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_PlayCharAnim2", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let mut l_anim_3: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlObj_Get(c, p_objId_1);
    l_anim_3 = c.m.r32((l_obj_2).wrapping_add(((2320i32) as u32)));
    if (((((l_anim_3) != (((0i32) as u32))) as i32)) != 0) {
        F::DemoCam_SetAnim(c, l_anim_3);
        F::DemoCam_SetObj(c, l_obj_2);
        F::DemoCam_SetTime(c, F32(0x00000000));
        F::DemoCam_StartAuto(c);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_PlayCharAnim2", &PROBE_N, &[], true); }
    r_
}

pub fn DemoCam_PlayStageAnim(c: &mut Ctx, mut p_idx_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_PlayStageAnim", &PROBE_N, &[(p_idx_1) as u32], false); }
    let mut l_anim_2: u32 = 0;
    let r_ = 'f: {
    l_anim_2 = F::BtlStage_GetFileMember(c, p_idx_1);
    if (((((l_anim_2) != (((0i32) as u32))) as i32)) != 0) {
        F::DemoCam_SetAnim(c, l_anim_2);
        F::DemoCam_SetTime(c, F32(0x00000000));
        F::DemoCam_StartAutoOnce(c);
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "DemoCam_PlayStageAnim", &PROBE_N, &[], true); }
    r_
}
