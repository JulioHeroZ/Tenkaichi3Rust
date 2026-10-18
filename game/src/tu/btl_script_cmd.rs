#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlScriptCmd_SetCamera(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetCamera", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_cam_3: u32 = 0;
    let mut l_ret_4: i32 = 0;
    let r_ = 'f: {
        l_cam_3 = (sym::gBtlScript).wrapping_add(400);
        l_ret_4 = 0i32;
        's1: {
            let sv5: u32 = p_phase_1;
            match sv5 {
                0u32 | 4u32 => {
                    { let v_ = 0i32; let a_ = l_cam_3; c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (l_cam_3).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (l_cam_3).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = F::Gsc_GetFloat(c); let a_ = fp; c.m.w32(a_, (v_).0); };
                    { let v_ = F::Gsc_GetFloat(c); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = F::Gsc_GetFloat(c); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fdiv(rt::fmul(F::Gsc_GetFloat(c), F32(0x40490fda)), F32(0x43340000)); let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fdiv(rt::fmul(F::Gsc_GetFloat(c), F32(0x40490fda)), F32(0x43340000)); let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fdiv(rt::fmul(F::Gsc_GetFloat(c), F32(0x40490fda)), F32(0x43340000)); let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    F::BtlFacade_SetFixedCamera(c, fp, fp.wrapping_add(16));
                    l_ret_4 = 1i32;
                    break 's1;
                }
                1u32 | 2u32 | 3u32 => {
                    break 's1;
                }
                _ => {
                }
            }
        }
        break 'f l_ret_4;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetCamera", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_ClearCamera(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_ClearCamera", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let r_ = 'f: {
    if (((((((((p_phase_1) == (((0i32) as u32))) as i32)) != 0) || (((((p_phase_1) == (((4i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        F::BtlFacade_ClearFixedCamera(c);
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_ClearCamera", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_ShakeCamera(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_ShakeCamera", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let mut l_ret_3: i32 = 0;
    let mut l_time_4: F32 = F32(0);
    let r_ = 'f: {
    l_ret_3 = 0i32;
    's1: {
        let sv5: u32 = p_phase_1;
        match sv5 {
            0u32 => {
                l_time_4 = F::Gsc_GetFloat(c);
                { let _ = F::BtlFacade_ShakeCamera(c, l_time_4); };
                if ((F::Gsc_FindOption(c, ((108i32) as i8))) != 0) {
                    { let v_ = l_time_4; let a_ = (sym::gBtlScript).wrapping_add(396); c.m.w32(a_, (v_).0); };
                }
                l_ret_3 = 1i32;
                break 's1;
            }
            4u32 => {
                l_ret_3 = 1i32;
                break 's1;
            }
            1u32 | 2u32 | 3u32 => {
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f l_ret_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_ShakeCamera", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_StopShake(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_StopShake", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let mut l_work_3: u32 = 0;
    let r_ = 'f: {
    if (((((((((p_phase_1) == (((0i32) as u32))) as i32)) != 0) || (((((p_phase_1) == (((4i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        l_work_3 = sym::gBtlScript;
        { let v_ = F32(0x00000000); let a_ = (l_work_3).wrapping_add(396); c.m.w32(a_, (v_).0); };
        { let _ = { let a0_ = F32(c.m.r32((l_work_3).wrapping_add(396))); F::BtlFacade_ShakeCamera(c, a0_) }; };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_StopShake", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_MoveCamera(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_MoveCamera", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_cam_3: u32 = 0;
    let mut l_ret_4: i32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_p_6: u32 = 0;
    let mut l_last_7: u32 = 0;
    let mut l_time_8: F32 = F32(0);
    let r_ = 'f: {
        l_cam_3 = (sym::gBtlScript).wrapping_add(400);
        l_ret_4 = 0i32;
        's1: {
            let sv9: u32 = p_phase_1;
            match sv9 {
                0u32 => {
                    { let v_ = 0i32; let a_ = l_cam_3; c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (l_cam_3).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (l_cam_3).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    {
                        l_i_5 = 8i32;
                        'l2: loop {
                            if !(((((l_i_5) >= (0i32)) as i32)) != 0) { break 'l2; }
                            'c3: {
                                { let v_ = 0i32; let a_ = ((l_cam_3).wrapping_add(16)).wrapping_add((((l_i_5) as u32)).wrapping_mul(48)); c.m.w32(a_, (v_) as u32); };
                            }
                            { let o_ = l_i_5; let n_ = (o_).wrapping_sub(1); l_i_5 = n_; };
                        }
                    }
                    if ((F::Gsc_FindOption(c, ((108i32) as i8))) != 0) {
                        F::BtlFacade_GetFixedCamera(c, fp.wrapping_add(32), fp.wrapping_add(48));
                        F::Vec4_Copy(c, (((l_cam_3).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(16), fp.wrapping_add(32));
                        F::Vec4_Copy(c, (((l_cam_3).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(32), fp.wrapping_add(48));
                        { let v_ = F32(0x00000000); let a_ = (((l_cam_3).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = 1i32; let a_ = ((l_cam_3).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(48)); c.m.w32(a_, (v_) as u32); };
                        l_p_6 = l_cam_3;
                        {
                            l_i_5 = 0i32;
                            'l4: loop {
                                if !((((((F::Gsc_FindOptionN(c, ((108i32) as i8), l_i_5)) != 0) && (((((l_i_5) < (8i32)) as i32)) != 0)) as i32)) != 0) { break 'l4; }
                                'c5: {
                                    { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
                                    l_time_8 = F::Gsc_GetFloat(c);
                                    { let v_ = F::Gsc_GetFloat(c); let a_ = fp; c.m.w32(a_, (v_).0); };
                                    { let v_ = F::Gsc_GetFloat(c); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                    { let v_ = F::Gsc_GetFloat(c); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                    { let v_ = rt::fdiv(rt::fmul(F::Gsc_GetFloat(c), F32(0x40490fda)), F32(0x43340000)); let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
                                    { let v_ = rt::fdiv(rt::fmul(F::Gsc_GetFloat(c), F32(0x40490fda)), F32(0x43340000)); let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                    { let v_ = rt::fdiv(rt::fmul(F::Gsc_GetFloat(c), F32(0x40490fda)), F32(0x43340000)); let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                    F::Vec4_Copy(c, (((l_p_6).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(48))).wrapping_add(16), fp);
                                    F::Vec4_Copy(c, (((l_p_6).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(48))).wrapping_add(32), fp.wrapping_add(16));
                                    { let v_ = 1i32; let a_ = ((l_p_6).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(48)); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = l_time_8; let a_ = (((l_p_6).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(48))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                    l_p_6 = (l_p_6).wrapping_add(48u32);
                                }
                            }
                        }
                        { let v_ = 1i32; let a_ = l_cam_3; c.m.w32(a_, (v_) as u32); };
                    }
                    if (((((F::Gsc_FindOption(c, ((119i32) as i8))) == (0i32)) as i32)) != 0) {
                        l_ret_4 = 1i32;
                    }
                    break 's1;
                }
                2u32 => {
                    if ((((((c.m.r32(l_cam_3) as i32)) == (0i32)) as i32)) != 0) {
                        l_ret_4 = 1i32;
                    }
                    break 's1;
                }
                4u32 => {
                    { let v_ = 0i32; let a_ = l_cam_3; c.m.w32(a_, (v_) as u32); };
                    {
                        l_last_7 = l_cam_3;
                        'l6: loop {
                            'c7: {
                                if ((((((c.m.r32(((l_last_7).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(48))) as i32)) == (0i32)) as i32)) != 0) {
                                    F::BtlFacade_GetFixedCamera(c, (((l_last_7).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(16), (((l_last_7).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(32));
                                    break 'l6;
                                }
                            }
                            l_last_7 = (l_last_7).wrapping_add(48u32);
                        }
                    }
                    break 's1;
                }
                1u32 | 3u32 => {
                    break 's1;
                }
                _ => {
                }
            }
        }
        break 'f l_ret_4;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_MoveCamera", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_PlaceChar(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_PlaceChar", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_side_3: i32 = 0;
    let r_ = 'f: {
        if (((((((((p_phase_1) == (((0i32) as u32))) as i32)) != 0) || (((((p_phase_1) == (((4i32) as u32))) as i32)) != 0)) as i32)) != 0) {
            l_side_3 = (((F::Gsc_GetInt(c)).wrapping_shr((15i32) as u32)) & (1i32));
            { let v_ = F::Gsc_GetFloat(c); let a_ = fp; c.m.w32(a_, (v_).0); };
            { let v_ = F::Gsc_GetFloat(c); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = F::Gsc_GetFloat(c); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fdiv(rt::fmul(F::Gsc_GetFloat(c), F32(0x40490fda)), F32(0x43340000)); let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fdiv(rt::fmul(F::Gsc_GetFloat(c), F32(0x40490fda)), F32(0x43340000)); let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fdiv(rt::fmul(F::Gsc_GetFloat(c), F32(0x40490fda)), F32(0x43340000)); let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            F::BtlFacade_SetCharPos(c, l_side_3, fp);
            F::BtlFacade_SetCharRot(c, l_side_3, fp.wrapping_add(16));
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_PlaceChar", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_SetCharPos(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetCharPos", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_side_3: i32 = 0;
    let r_ = 'f: {
        if (((((((((p_phase_1) == (((0i32) as u32))) as i32)) != 0) || (((((p_phase_1) == (((4i32) as u32))) as i32)) != 0)) as i32)) != 0) {
            l_side_3 = (((F::Gsc_GetInt(c)).wrapping_shr((15i32) as u32)) & (1i32));
            { let v_ = F::Gsc_GetFloat(c); let a_ = fp; c.m.w32(a_, (v_).0); };
            { let v_ = F::Gsc_GetFloat(c); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = F::Gsc_GetFloat(c); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
            F::BtlFacade_SetCharPos(c, l_side_3, fp);
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetCharPos", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_SetCharDir(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetCharDir", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_side_3: i32 = 0;
    let r_ = 'f: {
        if (((((((((p_phase_1) == (((0i32) as u32))) as i32)) != 0) || (((((p_phase_1) == (((4i32) as u32))) as i32)) != 0)) as i32)) != 0) {
            l_side_3 = (((F::Gsc_GetInt(c)).wrapping_shr((15i32) as u32)) & (1i32));
            { let v_ = rt::fdiv(rt::fmul(F::Gsc_GetFloat(c), F32(0x40490fda)), F32(0x43340000)); let a_ = fp; c.m.w32(a_, (v_).0); };
            { let v_ = rt::fdiv(rt::fmul(F::Gsc_GetFloat(c), F32(0x40490fda)), F32(0x43340000)); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fdiv(rt::fmul(F::Gsc_GetFloat(c), F32(0x40490fda)), F32(0x43340000)); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
            F::BtlFacade_SetCharRot(c, l_side_3, fp);
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetCharDir", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_ClearCtrl100(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_ClearCtrl100", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let r_ = 'f: {
    if (((((((((p_phase_1) == (((0i32) as u32))) as i32)) != 0) || (((((p_phase_1) == (((4i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        { let a0_ = (((F::Gsc_GetInt(c)).wrapping_shr((15i32) as u32)) & (1i32)); F::BtlFacade_ClearCtrl100(c, a0_) };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_ClearCtrl100", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_SetCtrl100(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetCtrl100", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let r_ = 'f: {
    if (((((((((p_phase_1) == (((0i32) as u32))) as i32)) != 0) || (((((p_phase_1) == (((4i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        { let a0_ = (((F::Gsc_GetInt(c)).wrapping_shr((15i32) as u32)) & (1i32)); F::BtlFacade_SetCtrl100(c, a0_) };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetCtrl100", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_SetCtrlFE(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetCtrlFE", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let r_ = 'f: {
    if (((((((((p_phase_1) == (((0i32) as u32))) as i32)) != 0) || (((((p_phase_1) == (((4i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        { let a0_ = (((F::Gsc_GetInt(c)).wrapping_shr((15i32) as u32)) & (1i32)); F::BtlFacade_SetCtrlFE(c, a0_) };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetCtrlFE", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_SetCtrlFF(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetCtrlFF", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let r_ = 'f: {
    if (((((((((p_phase_1) == (((0i32) as u32))) as i32)) != 0) || (((((p_phase_1) == (((4i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        { let a0_ = (((F::Gsc_GetInt(c)).wrapping_shr((15i32) as u32)) & (1i32)); F::BtlFacade_SetCtrlFF(c, a0_) };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetCtrlFF", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_SetCtrl101(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetCtrl101", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_phase_1) == (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = (((F::Gsc_GetInt(c)).wrapping_shr((15i32) as u32)) & (1i32)); F::BtlFacade_SetCtrl101(c, a0_) };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetCtrl101", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_ClearCtrl101(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_ClearCtrl101", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let r_ = 'f: {
    if (((((((((p_phase_1) == (((0i32) as u32))) as i32)) != 0) || (((((p_phase_1) == (((4i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        { let a0_ = (((F::Gsc_GetInt(c)).wrapping_shr((15i32) as u32)) & (1i32)); F::BtlFacade_ClearCtrl101(c, a0_) };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_ClearCtrl101", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_SetCtrl102(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetCtrl102", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let r_ = 'f: {
    if (((((((((p_phase_1) == (((0i32) as u32))) as i32)) != 0) || (((((p_phase_1) == (((4i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        { let a0_ = (((F::Gsc_GetInt(c)).wrapping_shr((15i32) as u32)) & (1i32)); F::BtlFacade_SetCtrl102(c, a0_) };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetCtrl102", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_MoveChar(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_MoveChar", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let mut l_work_3: u32 = 0;
    let mut l_ret_4: i32 = 0;
    let mut l_side_5: i32 = 0;
    let mut l_type_6: i32 = 0;
    let mut l_loop_7: i32 = 0;
    let mut l_done_8: i32 = 0;
    let mut l_value_9: F32 = F32(0);
    let r_ = 'f: {
    l_work_3 = p_taskWork_2;
    l_ret_4 = 0i32;
    's1: {
        let sv10: u32 = p_phase_1;
        match sv10 {
            0u32 | 4u32 => {
                l_side_5 = (((F::Gsc_GetInt(c)).wrapping_shr((15i32) as u32)) & (1i32));
                l_value_9 = F32(0x00000000);
                l_type_6 = F::Gsc_GetInt(c);
                if ((F::Gsc_FindOption(c, ((116i32) as i8))) != 0) {
                    l_value_9 = F::Gsc_GetFloat(c);
                }
                l_done_8 = 1i32;
                l_loop_7 = (((F::Gsc_FindOption(c, ((108i32) as i8))) != (0i32)) as i32);
                if ((F::Gsc_FindOption(c, ((119i32) as i8))) != 0) {
                    l_ret_4 = l_loop_7;
                } else {
                    l_ret_4 = l_done_8;
                }
                F::BtlFacade_ClearCtrl100(c, l_side_5);
                { let v_ = l_side_5; let a_ = (l_work_3).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                F::BtlFacade_PlayCharMotion(c, l_side_5, l_type_6, l_loop_7, l_value_9);
                if (((((p_phase_1) == (((4i32) as u32))) as i32)) != 0) {
                    l_ret_4 = 1i32;
                }
                break 's1;
            }
            2u32 => {
                l_ret_4 = ((({ let a0_ = (c.m.r32((l_work_3).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::BtlFacade_IsCharMotionPlaying(c, a0_) }) == (0i32)) as i32);
                break 's1;
            }
            1u32 | 3u32 => {
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f l_ret_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_MoveChar", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_StopCharMoves(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_StopCharMoves", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let r_ = 'f: {
    if (((((((((p_phase_1) == (((0i32) as u32))) as i32)) != 0) || (((((p_phase_1) == (((4i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        F::BtlFacade_StopCharMotion(c, 0i32);
        F::BtlFacade_StopCharMotion(c, 1i32);
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_StopCharMoves", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_FadeIn(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_FadeIn", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let mut l_work_3: u32 = 0;
    let mut l_ret_4: i32 = 0;
    let mut l_time_5: F32 = F32(0);
    let r_ = 'f: {
    l_work_3 = p_taskWork_2;
    l_ret_4 = 0i32;
    's1: {
        let sv6: u32 = p_phase_1;
        match sv6 {
            0u32 | 4u32 => {
                l_time_5 = F::Gsc_GetFloat(c);
                if (((((F::Gsc_FindOption(c, ((87i32) as i8))) == (0i32)) as i32)) != 0) {
                    { let v_ = 0i32; let a_ = l_work_3; c.m.w32(a_, (v_) as u32); };
                } else {
                    { let v_ = 1i32; let a_ = l_work_3; c.m.w32(a_, (v_) as u32); };
                }
                { let a0_ = (c.m.r32(l_work_3) as i32); F::Fade_Start(c, a0_, 1i32, l_time_5) };
                { let v_ = 0i32; let a_ = (l_work_3).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = F::Gsc_FindOption(c, ((119i32) as i8)); let a_ = (l_work_3).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32((l_work_3).wrapping_add(8)) as i32)) == (0i32)) as i32)) != 0) {
                    l_ret_4 = 1i32;
                }
                break 's1;
            }
            2u32 => {
                if (({ let a0_ = (c.m.r32(l_work_3) as i32); F::Fade_IsDone(c, a0_) }) != 0) {
                    l_ret_4 = ((((c.m.r32((l_work_3).wrapping_add(4)) as i32)) > (0i32)) as i32);
                    { let a_ = (l_work_3).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                }
                break 's1;
            }
            1u32 => {
                if ((((((c.m.r32((l_work_3).wrapping_add(8)) as i32)) != (0i32)) as i32)) != 0) {
                    { let a0_ = (c.m.r32(l_work_3) as i32); F::Fade_Reset(c, a0_) };
                }
                break 's1;
            }
            3u32 => {
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f l_ret_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_FadeIn", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_FadeOut(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_FadeOut", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let mut l_work_3: u32 = 0;
    let mut l_ret_4: i32 = 0;
    let mut l_time_5: F32 = F32(0);
    let r_ = 'f: {
    l_work_3 = p_taskWork_2;
    l_ret_4 = 0i32;
    's1: {
        let sv6: u32 = p_phase_1;
        match sv6 {
            0u32 | 4u32 => {
                l_time_5 = F::Gsc_GetFloat(c);
                if (((((F::Gsc_FindOption(c, ((87i32) as i8))) == (0i32)) as i32)) != 0) {
                    { let v_ = 0i32; let a_ = l_work_3; c.m.w32(a_, (v_) as u32); };
                } else {
                    { let v_ = 1i32; let a_ = l_work_3; c.m.w32(a_, (v_) as u32); };
                }
                { let a0_ = (c.m.r32(l_work_3) as i32); F::Fade_Start(c, a0_, 0i32, l_time_5) };
                { let v_ = 0i32; let a_ = (l_work_3).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = F::Gsc_FindOption(c, ((119i32) as i8)); let a_ = (l_work_3).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32((l_work_3).wrapping_add(8)) as i32)) == (0i32)) as i32)) != 0) {
                    l_ret_4 = 1i32;
                }
                break 's1;
            }
            2u32 => {
                if (({ let a0_ = (c.m.r32(l_work_3) as i32); F::Fade_IsDone(c, a0_) }) != 0) {
                    l_ret_4 = ((((c.m.r32((l_work_3).wrapping_add(4)) as i32)) > (0i32)) as i32);
                    { let a_ = (l_work_3).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                }
                break 's1;
            }
            1u32 => {
                if ((((((c.m.r32((l_work_3).wrapping_add(8)) as i32)) != (0i32)) as i32)) != 0) {
                    { let a0_ = (c.m.r32(l_work_3) as i32); F::Fade_Reset(c, a0_) };
                }
                break 's1;
            }
            3u32 => {
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f l_ret_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_FadeOut", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_ResetFades(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_ResetFades", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let r_ = 'f: {
    if (((((((((p_phase_1) == (((0i32) as u32))) as i32)) != 0) || (((((p_phase_1) == (((4i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        F::Fade_Reset(c, 0i32);
        F::Fade_Reset(c, 1i32);
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_ResetFades", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_Nop1301(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_Nop1301", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let r_ = 'f: {
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_Nop1301", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_LoadText(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_LoadText", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let mut l_base_3: i32 = 0;
    let r_ = 'f: {
    if (((((((((p_phase_1) == (((0i32) as u32))) as i32)) != 0) || (((((p_phase_1) == (((4i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        l_base_3 = (1i32).wrapping_neg();
        's1: {
            let sv4: i32 = { let a_ = c.m.r32(sym::gProgress); (c.m.r32(a_) as i32) };
            match sv4 {
                0i32 => {
                    l_base_3 = 561i32;
                    break 's1;
                }
                1i32 => {
                    l_base_3 = 611i32;
                    break 's1;
                }
                _ => {
                }
            }
        }
        { let a0_ = (l_base_3).wrapping_add(F::BtlScript_GetStory(c)); F::BtlScript_RequestTextFile(c, a0_) };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_LoadText", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_LoadFile8(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_LoadFile8", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let mut l_base_3: i32 = 0;
    let r_ = 'f: {
    if (((((((((p_phase_1) == (((0i32) as u32))) as i32)) != 0) || (((((p_phase_1) == (((4i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        if (((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
            l_base_3 = 711i32;
        } else {
            l_base_3 = 661i32;
        }
        { let a0_ = (l_base_3).wrapping_add(F::BtlScript_GetStory(c)); F::BtlScript_RequestFile8(c, a0_) };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_LoadFile8", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_SetWindow(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetWindow", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let mut l_win_3: u32 = 0;
    let mut l_r_4: u8 = 0;
    let mut l_g_5: u8 = 0;
    let mut l_b_6: u8 = 0;
    let mut l_a_7: i32 = 0;
    let r_ = 'f: {
    if (((((((((p_phase_1) == (((0i32) as u32))) as i32)) != 0) || (((((p_phase_1) == (((4i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        { let _ = F::Gsc_FindOption(c, ((108i32) as i8)); };
        if (((((F::Gsc_GetInt(c)) == (1i32)) as i32)) != 0) {
            { let _ = F::Gsc_FindOption(c, ((110i32) as i8)); };
            l_win_3 = { let a0_ = F::Gsc_GetIntOr(c, 0i32); F::BtlScript_GetWindow(c, a0_) };
            { let s_ = F::Font_GetStyle(c); let d_ = l_win_3; rt::memcpy(c, d_, s_, 80); };
            { let _ = F::Gsc_FindOption(c, ((100i32) as i8)); };
            { let v_ = F::Gsc_GetIntOr(c, 0i32); let a_ = (l_win_3).wrapping_add(72); c.m.w32(a_, (v_) as u32); };
            { let v_ = F::Gsc_GetIntOr(c, 0i32); let a_ = (l_win_3).wrapping_add(76); c.m.w32(a_, (v_) as u32); };
            { let _ = F::Gsc_FindOption(c, ((97i32) as i8)); };
            { let v_ = F::Gsc_GetFloatOr(c, F32(0x3f800000)); let a_ = l_win_3; c.m.w32(a_, (v_).0); };
            { let v_ = F::Gsc_GetFloatOr(c, F32(0x3f800000)); let a_ = (l_win_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let _ = F::Gsc_FindOption(c, ((115i32) as i8)); };
            { let v_ = F::Gsc_GetFloatOr(c, F32(0x3f800000)); let a_ = (l_win_3).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let _ = F::Gsc_FindOption(c, ((99i32) as i8)); };
            l_r_4 = ((F::Gsc_GetIntOr(c, 255i32)) as u8);
            l_g_5 = ((F::Gsc_GetIntOr(c, 255i32)) as u8);
            l_b_6 = ((F::Gsc_GetIntOr(c, 255i32)) as u8);
            l_a_7 = F::Gsc_GetIntOr(c, 128i32);
            { let v_ = ((((((((l_r_4) as i32)) | ((l_a_7).wrapping_shl((24i32) as u32)))) | ((((((l_b_6) as i32)).wrapping_shl((16i32) as u32)) | ((((l_g_5) as i32)).wrapping_shl((8i32) as u32)))))) as u32); let a_ = (l_win_3).wrapping_add(12); c.m.w32(a_, v_); };
            { let _ = F::Gsc_FindOption(c, ((112i32) as i8)); };
            { let v_ = F::Gsc_GetIntOr(c, 2i32); let a_ = (l_win_3).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
            { let v_ = F::Gsc_GetIntOr(c, 5i32); let a_ = (l_win_3).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
            { let _ = F::Gsc_FindOption(c, ((119i32) as i8)); };
            { let v_ = F::Gsc_GetIntOr(c, 0i32); let a_ = (l_win_3).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
            { let _ = F::Gsc_FindOption(c, ((84i32) as i8)); };
            { let v_ = F::Gsc_GetIntOr(c, 2i32); let a_ = (l_win_3).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let _ = F::Gsc_FindOption(c, ((79i32) as i8)); };
            { let v_ = ((F::Gsc_GetIntOr(c, 1i32)) as i16); let a_ = (l_win_3).wrapping_add(44); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((F::Gsc_GetIntOr(c, 2i32)) as i16); let a_ = (l_win_3).wrapping_add(46); c.m.w16(a_, (v_) as u16); };
            { let _ = F::Gsc_FindOption(c, ((67i32) as i8)); };
            l_r_4 = ((F::Gsc_GetIntOr(c, 255i32)) as u8);
            l_g_5 = ((F::Gsc_GetIntOr(c, 255i32)) as u8);
            l_b_6 = ((F::Gsc_GetIntOr(c, 255i32)) as u8);
            l_a_7 = F::Gsc_GetIntOr(c, 128i32);
            { let v_ = ((((((((l_r_4) as i32)) | ((l_a_7).wrapping_shl((24i32) as u32)))) | ((((((l_b_6) as i32)).wrapping_shl((16i32) as u32)) | ((((l_g_5) as i32)).wrapping_shl((8i32) as u32)))))) as u32); let a_ = (l_win_3).wrapping_add(40); c.m.w32(a_, v_); };
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetWindow", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_PlayBgm(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_PlayBgm", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let r_ = 'f: {
    if (((((((((p_phase_1) == (((0i32) as u32))) as i32)) != 0) || (((((p_phase_1) == (((4i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        { let a0_ = (F::Gsc_GetInt(c)).wrapping_add(68374i32); F::Bgm_Play(c, a0_) };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_PlayBgm", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_StopBgm(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_StopBgm", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let mut l_ret_3: i32 = 0;
    let r_ = 'f: {
    l_ret_3 = 0i32;
    's1: {
        let sv4: u32 = p_phase_1;
        match sv4 {
            0u32 | 4u32 => {
                F::Bgm_FadeOutStep(c);
                if (((((F::Gsc_FindOption(c, ((119i32) as i8))) == (0i32)) as i32)) != 0) {
                    l_ret_3 = 1i32;
                }
                break 's1;
            }
            2u32 => {
                l_ret_3 = (((F::Bgm_IsStopped(c)) != (0i32)) as i32);
                break 's1;
            }
            1u32 | 3u32 => {
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f l_ret_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_StopBgm", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_PlaySe(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_PlaySe", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_phase_1) == (((0i32) as u32))) as i32)) != 0) {
        { let _ = { let a1_ = F::Gsc_GetInt(c); F::Snd_PlaySe(c, ((4i32) as u32), a1_) }; };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_PlaySe", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_PlayVoice(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_PlayVoice", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let mut l_work_3: u32 = 0;
    let mut l_ret_4: i32 = 0;
    let mut l_line_5: i32 = 0;
    let r_ = 'f: {
    l_work_3 = p_taskWork_2;
    l_ret_4 = 0i32;
    's1: {
        let sv6: u32 = p_phase_1;
        match sv6 {
            0u32 => {
                l_line_5 = F::Gsc_GetInt(c);
                if ((F::Gsc_FindOption(c, ((104i32) as i8))) != 0) {
                    { let v_ = F::Gsc_GetInt(c); let a_ = (l_work_3).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                }
                l_ret_4 = (((F::Gsc_FindOption(c, ((119i32) as i8))) == (0i32)) as i32);
                { let v_ = 0i32; let a_ = (l_work_3).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let a1_ = F::BtlScript_GetVoiceFile(c, l_line_5); F::Voice_PlayDefault(c, 0i32, a1_) };
                break 's1;
            }
            2u32 => {
                if (({ let a0_ = (c.m.r32((l_work_3).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::Voice_IsStopped(c, a0_) }) != 0) {
                    l_ret_4 = 1i32;
                }
                break 's1;
            }
            4u32 => {
                if ((F::Gsc_IsCurrentBegun(c)) != 0) {
                    { let a0_ = (c.m.r32((l_work_3).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::Voice_Stop(c, a0_) };
                }
                l_ret_4 = 1i32;
                break 's1;
            }
            1u32 | 3u32 => {
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f l_ret_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_PlayVoice", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_Talk(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_Talk", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let mut l_work_3: u32 = 0;
    let mut l_who_4: i32 = 0;
    let mut l_side_5: i32 = 0;
    let mut l_member_6: i32 = 0;
    let mut l_line_7: i32 = 0;
    let mut l_channel_8: i32 = 0;
    let mut l_ret_9: i32 = 0;
    let r_ = 'f: {
    l_work_3 = p_taskWork_2;
    l_ret_9 = 0i32;
    's1: {
        let sv10: u32 = p_phase_1;
        match sv10 {
            0u32 => {
                l_channel_8 = 0i32;
                l_who_4 = F::Gsc_GetInt(c);
                l_member_6 = ((l_who_4) & (4095i32));
                l_side_5 = (((l_who_4).wrapping_shr((15i32) as u32)) & (1i32));
                l_line_7 = F::Gsc_GetInt(c);
                if ((F::Gsc_FindOption(c, ((104i32) as i8))) != 0) {
                    l_channel_8 = F::Gsc_GetInt(c);
                }
                l_ret_9 = (((F::Gsc_FindOption(c, ((119i32) as i8))) == (0i32)) as i32);
                if (((((F::BtlFacade_GetActiveMember(c, l_side_5)) == (l_member_6)) as i32)) != 0) {
                    F::BtlScript_SetText(c, 0i32, l_line_7);
                } else {
                    F::BtlScript_SetText(c, 1i32, l_line_7);
                }
                { let v_ = l_channel_8; let a_ = l_work_3; c.m.w32(a_, (v_) as u32); };
                { let v_ = 600i32; let a_ = (l_work_3).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = l_line_7; let a_ = (l_work_3).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = l_side_5; let a_ = (l_work_3).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                { let a1_ = F::BtlScript_GetVoiceFile(c, l_line_7); F::Voice_PlayDefault(c, l_channel_8, a1_) };
                { let v_ = ((0i32) as u32); let a_ = (l_work_3).wrapping_add(16); c.m.w32(a_, v_); };
                { let v_ = 0i32; let a_ = (l_work_3).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                if (((((F::BtlFacade_GetActiveMember(c, l_side_5)) == (l_member_6)) as i32)) != 0) {
                    { let v_ = F::BtlScript_GetFile8Entry(c, l_line_7); let a_ = (l_work_3).wrapping_add(16); c.m.w32(a_, v_); };
                }
                break 's1;
            }
            2u32 => {
                if (((((((((((((c.m.r32((l_work_3).wrapping_add(16))) != (((0i32) as u32))) as i32)) != 0) && ((((((c.m.r32((l_work_3).wrapping_add(20)) as i32)) == (0i32)) as i32)) != 0)) as i32)) != 0) && ((((({ let a0_ = ((c.m.r32(l_work_3) as i32)).wrapping_add(4i32); F::Adx_GetStat(c, a0_) }) == (3i32)) as i32)) != 0)) as i32)) != 0) {
                    { let a0_ = (c.m.r32((l_work_3).wrapping_add(12)) as i32); let a1_ = ((c.m.r32((l_work_3).wrapping_add(16))) as i32); F::BtlFacade_SetObjSubState14(c, a0_, a1_) };
                    { let v_ = 1i32; let a_ = (l_work_3).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                }
                if (({ let a0_ = (c.m.r32(l_work_3) as i32); F::Voice_IsStopped(c, a0_) }) != 0) {
                    { let a0_ = (c.m.r32((l_work_3).wrapping_add(12)) as i32); F::BtlFacade_EndObjSubState3(c, a0_) };
                    { let r_ = 10i32; let a_ = (l_work_3).wrapping_add(8); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                    if ((((((c.m.r32((l_work_3).wrapping_add(8)) as i32)) <= (0i32)) as i32)) != 0) {
                        l_ret_9 = 1i32;
                    }
                }
                break 's1;
            }
            1u32 => {
                { let a0_ = (c.m.r32((l_work_3).wrapping_add(4)) as i32); F::BtlScript_ClearText(c, a0_) };
                break 's1;
            }
            4u32 => {
                if ((F::Gsc_IsCurrentBegun(c)) != 0) {
                    { let a0_ = (c.m.r32(l_work_3) as i32); F::Voice_Stop(c, a0_) };
                    { let a0_ = (c.m.r32((l_work_3).wrapping_add(12)) as i32); F::BtlFacade_EndObjSubState3(c, a0_) };
                }
                l_ret_9 = 1i32;
                break 's1;
            }
            3u32 => {
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f l_ret_9;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_Talk", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_Wait(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_Wait", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let mut l_ret_3: i32 = 0;
    let r_ = 'f: {
    l_ret_3 = 0i32;
    's1: {
        let sv4: u32 = p_phase_1;
        'k4: {
        'k3: {
        'k2: {
            match sv4 { 2u32 => break 'k2, 4u32 => break 'k3, 0u32 => break 'k4, 1u32 => break 'k4, 3u32 => break 'k4, _ => break 's1, }
        }
        if (((((F::Gsc_GetCurrentTime(c)) < (rt::f2i(rt::fmul(F::Gsc_GetFloat(c), F32(0x43960000))))) as i32)) != 0) {
            break 's1;
        }
        }
        l_ret_3 = 1i32;
        break 's1;
        }
        break 's1;
    }
    break 'f l_ret_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_Wait", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_WaitButton(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_WaitButton", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let mut l_work_3: u32 = 0;
    let mut l_ret_4: i32 = 0;
    let r_ = 'f: {
    l_work_3 = p_taskWork_2;
    l_ret_4 = 0i32;
    's1: {
        let sv5: u32 = p_phase_1;
        match sv5 {
            0u32 => {
                { let v_ = F::Gsc_GetInt(c); let a_ = (l_work_3).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = l_work_3; c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            2u32 => {
                if ((((((c.m.r32(l_work_3) as i32)) != (0i32)) as i32)) != 0) {
                    l_ret_4 = 1i32;
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & ((((1i32).wrapping_shl(((c.m.r32((l_work_3).wrapping_add(4)) as i32)) as u32)) as u32)))) != 0) {
                        { let v_ = 1i32; let a_ = l_work_3; c.m.w32(a_, (v_) as u32); };
                    }
                }
                break 's1;
            }
            4u32 => {
                l_ret_4 = 1i32;
                break 's1;
            }
            1u32 | 3u32 => {
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f l_ret_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_WaitButton", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_SetStory(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetStory", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let r_ = 'f: {
    if (((((((((p_phase_1) == (((0i32) as u32))) as i32)) != 0) || (((((p_phase_1) == (((4i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        { let a0_ = F::Gsc_GetInt(c); F::BtlScript_SetStory(c, a0_) };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetStory", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_Nop3(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_Nop3", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let r_ = 'f: {
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_Nop3", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_EndEvent(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_EndEvent", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let mut l_event_3: u32 = 0;
    let r_ = 'f: {
    l_event_3 = c.m.r32(((sym::gBtlScript).wrapping_add(384)).wrapping_add(4));
    if (((((((((p_phase_1) == (((0i32) as u32))) as i32)) != 0) || (((((p_phase_1) == (((4i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        if (((((((((((((l_event_3) == (((0i32) as u32))) as i32)) != 0) || ((((((((rt::bf_get_u(c, l_event_3, 1, 3) as u8)) as i32)) == (0i32)) as i32)) != 0)) as i32)) != 0) || ((((!(((rt::bf_get_u(c, l_event_3, 4, 1) as u8)) != 0)) as i32)) != 0)) as i32)) != 0) {
            F::BtlFacade_StopCharMotion(c, 0i32);
            F::BtlFacade_StopCharMotion(c, 1i32);
            F::BtlFacade_ClearFixedCamera(c);
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_EndEvent", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_BeginScene(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_BeginScene", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_work_3: u32 = 0;
    let r_ = 'f: {
        l_work_3 = sym::gBtlScript;
        if (((((((((p_phase_1) == (((0i32) as u32))) as i32)) != 0) || (((((p_phase_1) == (((4i32) as u32))) as i32)) != 0)) as i32)) != 0) {
            if (((((((((c.m.r32(((l_work_3).wrapping_add(384)).wrapping_add(4))) != (((0i32) as u32))) as i32)) != 0) && (((({ let a_ = c.m.r32(((l_work_3).wrapping_add(384)).wrapping_add(4)); (rt::bf_get_u(c, a_, 4, 1) as u8) }) as i32)) != 0)) as i32)) != 0) {
                F::BtlFacade_BeginInterrupt(c);
                F::BtlScene_Reset(c, 2i32);
                F::BtlScene_Reset(c, 3i32);
                { let _ = F::BtlStage_GetStartPlace(c, 0i32, fp, fp.wrapping_add(32), 0i32); };
                { let _ = F::BtlStage_GetStartPlace(c, 1i32, fp.wrapping_add(16), fp.wrapping_add(48), 0i32); };
                F::BtlFacade_SetCharPos(c, 0i32, fp);
                F::BtlFacade_SetCharRot(c, 0i32, fp.wrapping_add(32));
                F::BtlFacade_SetCharPos(c, 1i32, fp.wrapping_add(16));
                F::BtlFacade_SetCharRot(c, 1i32, fp.wrapping_add(48));
                F::BtlFacade_PlayCharMotion(c, 0i32, 0i32, 1i32, F32(0x00000000));
                F::BtlFacade_PlayCharMotion(c, 1i32, 0i32, 1i32, F32(0x00000000));
            }
            if (((((c.m.r32(((l_work_3).wrapping_add(384)).wrapping_add(4))) == (((0i32) as u32))) as i32)) != 0) {
                { let _ = F::BtlStage_GetStartPlace(c, 0i32, fp, fp.wrapping_add(32), 0i32); };
                { let _ = F::BtlStage_GetStartPlace(c, 1i32, fp.wrapping_add(16), fp.wrapping_add(48), 0i32); };
                F::BtlFacade_SetCharPos(c, 0i32, fp);
                F::BtlFacade_SetCharRot(c, 0i32, fp.wrapping_add(32));
                F::BtlFacade_SetCharPos(c, 1i32, fp.wrapping_add(16));
                F::BtlFacade_SetCharRot(c, 1i32, fp.wrapping_add(48));
                F::BtlFacade_PlayCharMotion(c, 0i32, 0i32, 1i32, F32(0x00000000));
                F::BtlFacade_PlayCharMotion(c, 1i32, 0i32, 1i32, F32(0x00000000));
            }
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_BeginScene", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_EndScene(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_EndScene", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let mut l_work_3: u32 = 0;
    let r_ = 'f: {
    l_work_3 = sym::gBtlScript;
    if (((((((((p_phase_1) == (((0i32) as u32))) as i32)) != 0) || (((((p_phase_1) == (((4i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        if (((((((((c.m.r32(((l_work_3).wrapping_add(384)).wrapping_add(4))) != (((0i32) as u32))) as i32)) != 0) && (((({ let a_ = c.m.r32(((l_work_3).wrapping_add(384)).wrapping_add(4)); (rt::bf_get_u(c, a_, 4, 1) as u8) }) as i32)) != 0)) as i32)) != 0) {
            F::BtlFacade_SetCtrlFF(c, 0i32);
            F::BtlFacade_SetCtrlFF(c, 1i32);
            F::BtlFacade_ClearCtrl101(c, 0i32);
            F::BtlFacade_ClearCtrl101(c, 1i32);
            { let _ = F::BtlFacade_ShakeCamera(c, F32(0x00000000)); };
            F::BtlFacade_EndInterrupt(c);
        }
        if (((((c.m.r32(((l_work_3).wrapping_add(384)).wrapping_add(4))) == (((0i32) as u32))) as i32)) != 0) {
            F::BtlFacade_SetCtrlFF(c, 0i32);
            F::BtlFacade_SetCtrlFF(c, 1i32);
            F::BtlFacade_ClearCtrl101(c, 0i32);
            F::BtlFacade_ClearCtrl101(c, 1i32);
            { let _ = F::BtlFacade_ShakeCamera(c, F32(0x00000000)); };
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_EndScene", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_Battle(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_Battle", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let mut l_what_3: i32 = 0;
    let mut l_arg_4: i32 = 0;
    let mut l_side_5: i32 = 0;
    let r_ = 'f: {
    if (((((((((p_phase_1) == (((0i32) as u32))) as i32)) != 0) || (((((p_phase_1) == (((4i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        F::BtlFacade_BeginWait(c);
        l_what_3 = F::Gsc_GetInt(c);
        l_arg_4 = F::Gsc_GetInt(c);
        l_side_5 = (((l_what_3).wrapping_shr((15i32) as u32)) & (1i32));
        's1: {
            let sv6: i32 = ((l_what_3) & (4095i32));
            match sv6 {
                0i32 => {
                    break 's1;
                }
                1i32 => {
                    { let _ = F::BtlFacade_CallSeqExtra(c); };
                    break 's1;
                }
                2i32 => {
                    { let _ = F::BtlFacade_CallSeqExtra2(c); };
                    break 's1;
                }
                3i32 => {
                    F::BtlFacade_RequestStageChange(c);
                    break 's1;
                }
                4i32 => {
                    F::BtlFacade_ForceBoth110(c);
                    break 's1;
                }
                5i32 | 6i32 | 7i32 => {
                    F::BtlFacade_ForceBoth111(c);
                    break 's1;
                }
                8i32 => {
                    F::BtlFacade_ForceActions(c, 2i32, 2i32);
                    break 's1;
                }
                9i32 => {
                    F::BtlFacade_ForceActions(c, 2i32, 3i32);
                    break 's1;
                }
                10i32 => {
                    F::BtlFacade_ForceActions(c, 3i32, 2i32);
                    break 's1;
                }
                11i32 => {
                    F::BtlFacade_ForceActions(c, 3i32, 3i32);
                    break 's1;
                }
                12i32 => {
                    F::BtlFacade_ForceActions(c, 4i32, 4i32);
                    break 's1;
                }
                13i32 | 14i32 | 15i32 => {
                    break 's1;
                }
                16i32 => {
                    F::BtlFacade_ChangeMember(c, l_side_5, 0i32);
                    break 's1;
                }
                17i32 => {
                    F::BtlFacade_ChangeMember(c, l_side_5, 1i32);
                    break 's1;
                }
                18i32 => {
                    F::BtlFacade_ChangeMember(c, l_side_5, 2i32);
                    break 's1;
                }
                19i32 => {
                    F::BtlFacade_ChangeMember(c, l_side_5, 3i32);
                    break 's1;
                }
                20i32 => {
                    F::BtlFacade_ChangeMember(c, l_side_5, 4i32);
                    break 's1;
                }
                21i32 | 22i32 => {
                    F::BtlFacade_Transform(c, l_side_5, l_arg_4);
                    break 's1;
                }
                23i32 => {
                    F::BtlFacade_Fuse(c, l_side_5, l_arg_4);
                    break 's1;
                }
                24i32 => {
                    F::BtlFacade_SetMaxPower(c, l_side_5, 1i32);
                    break 's1;
                }
                25i32 => {
                    F::BtlFacade_ForceAction01(c, l_side_5, 0i32);
                    break 's1;
                }
                26i32 => {
                    F::BtlFacade_ForceAction01(c, l_side_5, 1i32);
                    break 's1;
                }
                27i32 => {
                    F::BtlFacade_ForceAction23(c, l_side_5, 0i32);
                    break 's1;
                }
                28i32 => {
                    F::BtlFacade_ForceAction23(c, l_side_5, 1i32);
                    break 's1;
                }
                29i32 => {
                    F::BtlFacade_ForceAction4(c, l_side_5);
                    break 's1;
                }
                _ => {
                }
            }
        }
        { let _ = F::BtlFacade_ResetSubClock(c); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_Battle", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_SetResult(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetResult", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let mut l_work_3: u32 = 0;
    let mut l_ret_4: i32 = 0;
    let mut l_result_5: i32 = 0;
    let r_ = 'f: {
    l_work_3 = p_taskWork_2;
    l_ret_4 = 0i32;
    's1: {
        let sv6: u32 = p_phase_1;
        match sv6 {
            0u32 | 4u32 => {
                { let v_ = 0i32; let a_ = (l_work_3).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                l_result_5 = F::Gsc_GetInt(c);
                if (((((l_result_5) >= (0i32)) as i32)) != 0) {
                    F::BtlFacade_SetResult(c, l_result_5);
                } else {
                    l_ret_4 = 1i32;
                }
                break 's1;
            }
            2u32 => {
                if ((F::BtlFacade_IsNotFinished(c)) != 0) {
                    l_ret_4 = ((((c.m.r32((l_work_3).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) > (0i32)) as i32);
                    { let a_ = (l_work_3).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                }
                break 's1;
            }
            1u32 | 3u32 => {
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f l_ret_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetResult", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_SetTriggers(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetTriggers", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let mut l_trigger_4: i32 = 0;
    let r_ = 'f: {
    if (((((((((p_phase_1) == (((0i32) as u32))) as i32)) != 0) || (((((p_phase_1) == (((4i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        F::BtlScript_ResetView(c);
        l_i_3 = 0i32;
        'l1: while ((F::Gsc_FindOptionN(c, ((97i32) as i8), { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; o_ })) != 0) {
            l_trigger_4 = F::Gsc_GetInt(c);
            { let a1_ = F::Gsc_GetInt(c); F::BtlScript_AddEvent(c, l_trigger_4, a1_) };
        }
        l_i_3 = 0i32;
        'l2: while ((F::Gsc_FindOptionN(c, ((118i32) as i8), { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; o_ })) != 0) {
            l_trigger_4 = F::Gsc_GetInt(c);
            { let a1_ = F::Gsc_GetInt(c); F::BtlScript_AddLine(c, l_trigger_4, a1_, 1i32) };
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetTriggers", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_SetRule(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetRule", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let mut l_stage_3: i32 = 0;
    let mut l_bgm_4: i32 = 0;
    let mut l_timeLimit_5: i32 = 0;
    let mut l_announcer_6: i32 = 0;
    let mut l_unk10_7: i32 = 0;
    let r_ = 'f: {
    if (((((((((p_phase_1) == (((0i32) as u32))) as i32)) != 0) || (((((p_phase_1) == (((4i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        l_stage_3 = F::Gsc_GetInt(c);
        l_bgm_4 = F::Gsc_GetInt(c);
        l_timeLimit_5 = F::Gsc_GetInt(c);
        l_announcer_6 = F::Gsc_GetInt(c);
        l_unk10_7 = F::Gsc_GetInt(c);
        F::Battle_ClearWork(c);
        F::BattleSetup_SetRule(c, 0i32, 1i32, l_bgm_4, l_timeLimit_5, l_announcer_6, l_stage_3, l_unk10_7);
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetRule", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_SetSide(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetSide", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let mut l_side_3: i32 = 0;
    let mut l_count_4: i32 = 0;
    let r_ = 'f: {
    if (((((((((p_phase_1) == (((0i32) as u32))) as i32)) != 0) || (((((p_phase_1) == (((4i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        l_side_3 = F::Gsc_GetInt(c);
        l_count_4 = F::Gsc_GetInt(c);
        F::BattleSetup_SetSide(c, l_side_3, (if (((((l_side_3) != (0i32)) as i32)) != 0) { 2i32 } else { 0i32 }), 0i32, l_count_4, 0i32, 0i32, 0i32, ((0i32) as u32));
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetSide", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_SetMember(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetMember", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_work_3: u32 = 0;
    let mut l_who_4: i32 = 0;
    let mut l_member_5: i32 = 0;
    let mut l_side_6: i32 = 0;
    let mut l_chara_7: i32 = 0;
    let mut l_costume_8: i32 = 0;
    let mut l_variant_9: i32 = 0;
    let mut l_cpuLevel_10: i32 = 0;
    let mut l_item7_11: i32 = 0;
    let mut l_health_12: i32 = 0;
    let r_ = 'f: {
        l_work_3 = sym::gBtlScript;
        if (((((((((p_phase_1) == (((0i32) as u32))) as i32)) != 0) || (((((p_phase_1) == (((4i32) as u32))) as i32)) != 0)) as i32)) != 0) {
            l_who_4 = F::Gsc_GetInt(c);
            l_member_5 = ((l_who_4) & (4095i32));
            l_side_6 = (((l_who_4).wrapping_shr((15i32) as u32)) & (1i32));
            l_chara_7 = F::Gsc_GetInt(c);
            l_costume_8 = F::Gsc_GetInt(c);
            l_variant_9 = F::Gsc_GetInt(c);
            l_cpuLevel_10 = F::Gsc_GetInt(c);
            l_item7_11 = F::Gsc_GetInt(c);
            l_health_12 = F::Gsc_GetInt(c);
            { let v_ = F::Gsc_GetInt(c); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = F::Gsc_GetInt(c); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = F::Gsc_GetInt(c); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = F::Gsc_GetInt(c); let a_ = (fp).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = F::Gsc_GetInt(c); let a_ = (fp).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = F::Gsc_GetInt(c); let a_ = (fp).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = F::Gsc_GetInt(c); let a_ = (fp).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = l_item7_11; let a_ = (fp).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            if (((((l_chara_7) >= (0i32)) as i32)) != 0) {
                's1: {
                    let sv13: i32 = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(60); (c.m.r32(a_) as i32) };
                    match sv13 {
                        1i32 => {
                            { let r_ = 6i32; let n_ = (l_cpuLevel_10).wrapping_add(r_); l_cpuLevel_10 = n_; };
                            break 's1;
                        }
                        2i32 => {
                            { let r_ = 9i32; let n_ = (l_cpuLevel_10).wrapping_add(r_); l_cpuLevel_10 = n_; };
                            break 's1;
                        }
                        _ => {
                        }
                    }
                }
                F::BattleSetup_SetMemberByItemIds(c, l_side_6, l_member_5, l_chara_7, l_costume_8, l_variant_9, l_cpuLevel_10, rt::i2f(l_health_12), fp);
                { let s_ = fp; let d_ = (((l_work_3).wrapping_add(12)).wrapping_add((((l_side_6) as u32)).wrapping_mul(160))).wrapping_add((((l_member_5) as u32)).wrapping_mul(32)); rt::memcpy(c, d_, s_, 32); };
            }
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetMember", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_SetStatus0(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetStatus0", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let mut l_work_3: u32 = 0;
    let mut l_member_4: i32 = 0;
    let mut l_value_5: i32 = 0;
    let mut l_items_6: u32 = 0;
    let r_ = 'f: {
    l_work_3 = sym::gBtlScript;
    if (((((((((p_phase_1) == (((0i32) as u32))) as i32)) != 0) || (((((p_phase_1) == (((4i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        l_member_4 = F::BtlFacade_GetActiveMember(c, 0i32);
        if ((F::Gsc_FindOption(c, ((67i32) as i8))) != 0) {
            l_member_4 = F::Gsc_GetInt(c);
        }
        if ((F::Gsc_FindOption(c, ((104i32) as i8))) != 0) {
            l_value_5 = F::Gsc_GetInt(c);
            if (((((l_value_5) > (0i32)) as i32)) != 0) {
                F::BtlFacade_RaiseHp(c, 0i32, l_member_4, rt::i2f(l_value_5));
            } else {
                F::BtlFacade_LowerHp(c, 0i32, l_member_4, rt::i2f((l_value_5).wrapping_neg()));
            }
        }
        if ((F::Gsc_FindOption(c, ((72i32) as i8))) != 0) {
            { let a2_ = rt::i2f(F::Gsc_GetInt(c)); F::BtlFacade_AddHp(c, 0i32, l_member_4, a2_) };
        }
        if ((F::Gsc_FindOption(c, ((102i32) as i8))) != 0) {
            l_value_5 = F::Gsc_GetInt(c);
            if (((((l_value_5) > (0i32)) as i32)) != 0) {
                F::BtlFacade_RaiseGaugeC(c, 0i32, l_member_4, l_value_5);
            } else {
                F::BtlFacade_LowerGaugeC(c, 0i32, l_member_4, (l_value_5).wrapping_neg());
            }
        }
        if ((F::Gsc_FindOption(c, ((70i32) as i8))) != 0) {
            { let a2_ = F::Gsc_GetInt(c); F::BtlFacade_AddGaugeC(c, 0i32, l_member_4, a2_) };
        }
        if ((F::Gsc_FindOption(c, ((98i32) as i8))) != 0) {
            l_value_5 = F::Gsc_GetInt(c);
            if (((((l_value_5) > (0i32)) as i32)) != 0) {
                F::BtlFacade_RaiseGauge14(c, 0i32, l_member_4, l_value_5);
            } else {
                F::BtlFacade_LowerGauge14(c, 0i32, l_member_4, (l_value_5).wrapping_neg());
            }
        }
        if ((F::Gsc_FindOption(c, ((66i32) as i8))) != 0) {
            { let a2_ = F::Gsc_GetInt(c); F::BtlFacade_AddGaugeC(c, 0i32, l_member_4, a2_) };
        }
        l_items_6 = (((l_work_3).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(160))).wrapping_add((((l_member_4) as u32)).wrapping_mul(32));
        if ((F::Gsc_FindOption(c, ((48i32) as i8))) != 0) {
            { let v_ = F::Gsc_GetInt(c); let a_ = (l_items_6).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
        if ((F::Gsc_FindOption(c, ((49i32) as i8))) != 0) {
            { let v_ = F::Gsc_GetInt(c); let a_ = (l_items_6).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
        if ((F::Gsc_FindOption(c, ((50i32) as i8))) != 0) {
            { let v_ = F::Gsc_GetInt(c); let a_ = (l_items_6).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
        if ((F::Gsc_FindOption(c, ((51i32) as i8))) != 0) {
            { let v_ = F::Gsc_GetInt(c); let a_ = (l_items_6).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
        if ((F::Gsc_FindOption(c, ((52i32) as i8))) != 0) {
            { let v_ = F::Gsc_GetInt(c); let a_ = (l_items_6).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
        if ((F::Gsc_FindOption(c, ((53i32) as i8))) != 0) {
            { let v_ = F::Gsc_GetInt(c); let a_ = (l_items_6).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
        if ((F::Gsc_FindOption(c, ((54i32) as i8))) != 0) {
            { let v_ = F::Gsc_GetInt(c); let a_ = (l_items_6).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
        F::BtlFacade_SetMemberItems(c, 0i32, l_member_4, l_items_6);
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetStatus0", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_SetStatus1(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetStatus1", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let mut l_side_3: i32 = 0;
    let mut l_work_4: u32 = 0;
    let mut l_member_5: i32 = 0;
    let mut l_value_6: i32 = 0;
    let mut l_items_7: u32 = 0;
    let r_ = 'f: {
    l_side_3 = 1i32;
    l_work_4 = sym::gBtlScript;
    if (((((((((p_phase_1) == (((0i32) as u32))) as i32)) != 0) || (((((p_phase_1) == (((4i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        l_member_5 = F::BtlFacade_GetActiveMember(c, l_side_3);
        if ((F::Gsc_FindOption(c, ((67i32) as i8))) != 0) {
            l_member_5 = F::Gsc_GetInt(c);
        }
        if ((F::Gsc_FindOption(c, ((108i32) as i8))) != 0) {
            l_value_6 = F::Gsc_GetInt(c);
            's1: {
                let sv8: i32 = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(60); (c.m.r32(a_) as i32) };
                match sv8 {
                    1i32 => {
                        { let r_ = 6i32; let n_ = (l_value_6).wrapping_add(r_); l_value_6 = n_; };
                        break 's1;
                    }
                    2i32 => {
                        { let r_ = 9i32; let n_ = (l_value_6).wrapping_add(r_); l_value_6 = n_; };
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
            { let _ = 0i32; F::BtlFacade_SetCpuParam8(c, l_value_6) };
        }
        if ((F::Gsc_FindOption(c, ((104i32) as i8))) != 0) {
            l_value_6 = F::Gsc_GetInt(c);
            if (((((l_value_6) > (0i32)) as i32)) != 0) {
                F::BtlFacade_RaiseHp(c, l_side_3, l_member_5, rt::i2f(l_value_6));
            } else {
                F::BtlFacade_LowerHp(c, l_side_3, l_member_5, rt::i2f((l_value_6).wrapping_neg()));
            }
        }
        if ((F::Gsc_FindOption(c, ((72i32) as i8))) != 0) {
            { let a2_ = rt::i2f(F::Gsc_GetInt(c)); F::BtlFacade_AddHp(c, l_side_3, l_member_5, a2_) };
        }
        if ((F::Gsc_FindOption(c, ((102i32) as i8))) != 0) {
            l_value_6 = F::Gsc_GetInt(c);
            if (((((l_value_6) > (0i32)) as i32)) != 0) {
                F::BtlFacade_RaiseGaugeC(c, l_side_3, l_member_5, l_value_6);
            } else {
                F::BtlFacade_LowerGaugeC(c, l_side_3, l_member_5, (l_value_6).wrapping_neg());
            }
        }
        if ((F::Gsc_FindOption(c, ((70i32) as i8))) != 0) {
            { let a2_ = F::Gsc_GetInt(c); F::BtlFacade_AddGaugeC(c, l_side_3, l_member_5, a2_) };
        }
        if ((F::Gsc_FindOption(c, ((98i32) as i8))) != 0) {
            l_value_6 = F::Gsc_GetInt(c);
            if (((((l_value_6) > (0i32)) as i32)) != 0) {
                F::BtlFacade_RaiseGauge14(c, l_side_3, l_member_5, l_value_6);
            } else {
                F::BtlFacade_LowerGauge14(c, l_side_3, l_member_5, (l_value_6).wrapping_neg());
            }
        }
        if ((F::Gsc_FindOption(c, ((66i32) as i8))) != 0) {
            { let a2_ = F::Gsc_GetInt(c); F::BtlFacade_AddGaugeC(c, l_side_3, l_member_5, a2_) };
        }
        l_items_7 = (((l_work_4).wrapping_add(12)).wrapping_add((((l_side_3) as u32)).wrapping_mul(160))).wrapping_add((((l_member_5) as u32)).wrapping_mul(32));
        if ((F::Gsc_FindOption(c, ((48i32) as i8))) != 0) {
            { let v_ = F::Gsc_GetInt(c); let a_ = (l_items_7).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
        if ((F::Gsc_FindOption(c, ((49i32) as i8))) != 0) {
            { let v_ = F::Gsc_GetInt(c); let a_ = (l_items_7).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
        if ((F::Gsc_FindOption(c, ((50i32) as i8))) != 0) {
            { let v_ = F::Gsc_GetInt(c); let a_ = (l_items_7).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
        if ((F::Gsc_FindOption(c, ((51i32) as i8))) != 0) {
            { let v_ = F::Gsc_GetInt(c); let a_ = (l_items_7).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
        if ((F::Gsc_FindOption(c, ((52i32) as i8))) != 0) {
            { let v_ = F::Gsc_GetInt(c); let a_ = (l_items_7).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
        if ((F::Gsc_FindOption(c, ((53i32) as i8))) != 0) {
            { let v_ = F::Gsc_GetInt(c); let a_ = (l_items_7).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
        if ((F::Gsc_FindOption(c, ((54i32) as i8))) != 0) {
            { let v_ = F::Gsc_GetInt(c); let a_ = (l_items_7).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
        if ((F::Gsc_FindOption(c, ((97i32) as i8))) != 0) {
            { let v_ = F::Gsc_GetInt(c); let a_ = (l_items_7).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
        F::BtlFacade_SetMemberItems(c, l_side_3, l_member_5, l_items_7);
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetStatus1", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_SetSpeakers(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetSpeakers", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let mut l_line_4: i32 = 0;
    let r_ = 'f: {
    if (((((((((p_phase_1) == (((0i32) as u32))) as i32)) != 0) || (((((p_phase_1) == (((4i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        l_i_3 = 0i32;
        'l1: while ((F::Gsc_FindOptionN(c, ((118i32) as i8), { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; o_ })) != 0) {
            l_line_4 = F::Gsc_GetInt(c);
            { let a1_ = F::Gsc_GetInt(c); F::BtlScript_SetSpeaker(c, l_line_4, a1_) };
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetSpeakers", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScriptCmd_SetRewards(c: &mut Ctx, mut p_phase_1: u32, mut p_taskWork_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetRewards", &PROBE_N, &[(p_phase_1) as u32, (p_taskWork_2) as u32], false); }
    let mut l_reward_3: u32 = 0;
    let r_ = 'f: {
    l_reward_3 = (c.m.r32(sym::gProgress)).wrapping_add(64);
    if (((((((((p_phase_1) == (((0i32) as u32))) as i32)) != 0) || (((((p_phase_1) == (((4i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        { let v_ = F::Gsc_GetInt(c); let a_ = (l_reward_3).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = F::Gsc_GetInt(c); let a_ = (l_reward_3).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = F::Gsc_GetInt(c); let a_ = (l_reward_3).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = F::Gsc_GetInt(c); let a_ = ((l_reward_3).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = F::Gsc_GetInt(c); let a_ = ((l_reward_3).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = F::Gsc_GetInt(c); let a_ = ((l_reward_3).wrapping_add(12)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = F::Gsc_GetInt(c); let a_ = ((l_reward_3).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = F::Gsc_GetInt(c); let a_ = ((l_reward_3).wrapping_add(36)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = F::Gsc_GetInt(c); let a_ = ((l_reward_3).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = F::Gsc_GetInt(c); let a_ = ((l_reward_3).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = F::Gsc_GetInt(c); let a_ = ((l_reward_3).wrapping_add(24)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = F::Gsc_GetInt(c); let a_ = ((l_reward_3).wrapping_add(24)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = F::Gsc_GetInt(c); let a_ = ((l_reward_3).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = F::Gsc_GetInt(c); let a_ = ((l_reward_3).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = F::Gsc_GetInt(c); let a_ = ((l_reward_3).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScriptCmd_SetRewards", &PROBE_N, &[], true); }
    r_
}
