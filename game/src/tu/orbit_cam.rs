#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn OrbitCam_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "OrbitCam_Reset", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let _ = { let a0_ = c.m.r32(sym::gOrbitCam); F::memset(c, a0_, 0i32, 688u32) }; };
    { let a0_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(80); F::View_InitLayout(c, a0_, 0i32) };
    F::OrbitCam_SetDist(c, F32(0x41a80000));
    F::OrbitCam_SetDistRange(c, F32(0x40a00000), F32(0x43fa0000));
    F::OrbitCam_SetPitchLimit(c, F32(0x3fc90fda));
    F::OrbitCam_SetTarget(c, F32(0x00000000), F32(0xc104a3d7), F32(0x00000000));
    F::OrbitCam_SetAngles(c, F32(0x00000000), F32(0x40490fda), F32(0x00000000));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "OrbitCam_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn OrbitCam_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "OrbitCam_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, ((688u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gOrbitCam; c.m.w32(a_, v_); };
    F::OrbitCam_Reset(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "OrbitCam_Init", &PROBE_N, &[], true); }
    r_
}

pub fn OrbitCam_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "OrbitCam_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32(sym::gOrbitCam); F::Heap_Free(c, a0_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gOrbitCam; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "OrbitCam_Term", &PROBE_N, &[], true); }
    r_
}

pub fn OrbitCam_Update(c: &mut Ctx, mut p_usePad_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "OrbitCam_Update", &PROBE_N, &[(p_usePad_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_p_2: u32 = 0;
    let mut l_view_3: u32 = 0;
    let mut l_m_4: u32 = 0;
    let mut l_max_5: F32 = F32(0);
    let mut l_cur_6: F32 = F32(0);
    let mut l_x_7: F32 = F32(0);
    let mut l_y_8: F32 = F32(0);
    let mut l_z_9: F32 = F32(0);
    let r_ = 'f: {
        l_p_2 = (sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448));
        l_view_3 = (c.m.r32(sym::gOrbitCam)).wrapping_add(80);
        l_x_7 = F32(0x00000000);
        l_y_8 = F32(0x00000000);
        l_z_9 = F32(0x00000000);
        if ((((({ let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(64); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
            p_usePad_1 = 0i32;
        }
        if (((((p_usePad_1) != (0i32)) as i32)) != 0) {
            if (((((((rt::fgt(F32(c.m.r32(((l_p_2).wrapping_add(420)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0x3dcccccc)) as i32)) != 0) || (((rt::flt(F32(c.m.r32(((l_p_2).wrapping_add(420)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0xbdcccccc)) as i32)) != 0)) as i32)) != 0) {
                l_x_7 = F32(c.m.r32(((l_p_2).wrapping_add(420)).wrapping_add((((0i32) as u32)).wrapping_mul(4))));
            }
            if (((((((rt::fgt(F32(c.m.r32(((l_p_2).wrapping_add(420)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(0x3dcccccc)) as i32)) != 0) || (((rt::flt(F32(c.m.r32(((l_p_2).wrapping_add(420)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(0xbdcccccc)) as i32)) != 0)) as i32)) != 0) {
                l_y_8 = F32(c.m.r32(((l_p_2).wrapping_add(420)).wrapping_add((((1i32) as u32)).wrapping_mul(4))));
            }
            if (((((((rt::fgt(F32(c.m.r32(((l_p_2).wrapping_add(428)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(0x3dcccccc)) as i32)) != 0) || (((rt::flt(F32(c.m.r32(((l_p_2).wrapping_add(428)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(0xbdcccccc)) as i32)) != 0)) as i32)) != 0) {
                l_z_9 = F32(c.m.r32(((l_p_2).wrapping_add(428)).wrapping_add((((1i32) as u32)).wrapping_mul(4))));
            }
        }
        { let r_ = rt::fmul(l_x_7, F32(0x3dcccccc)); let a_ = ((c.m.r32(sym::gOrbitCam)).wrapping_add(16)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if (((rt::flt({ let a_ = ((c.m.r32(sym::gOrbitCam)).wrapping_add(16)).wrapping_add(4); F32(c.m.r32(a_)) }, F32(0xc0490fda)) as i32)) != 0) {
            { let r_ = F32(0x40c90fda); let a_ = ((c.m.r32(sym::gOrbitCam)).wrapping_add(16)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
        if (((rt::fgt({ let a_ = ((c.m.r32(sym::gOrbitCam)).wrapping_add(16)).wrapping_add(4); F32(c.m.r32(a_)) }, F32(0x40490fda)) as i32)) != 0) {
            { let r_ = F32(0x40c90fda); let a_ = ((c.m.r32(sym::gOrbitCam)).wrapping_add(16)).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
        { let r_ = rt::fmul(l_y_8, F32(0x3dcccccc)); let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(16); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if ((((({ let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(68); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
            if (((rt::flt({ let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(16); F32(c.m.r32(a_)) }, rt::fneg({ let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(60); F32(c.m.r32(a_)) })) as i32)) != 0) {
                { let v_ = rt::fneg({ let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(60); F32(c.m.r32(a_)) }); let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(16); c.m.w32(a_, (v_).0); };
            }
            l_max_5 = F32(0x00000000);
            l_cur_6 = { let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(16); F32(c.m.r32(a_)) };
        } else {
            if (((rt::flt({ let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(16); F32(c.m.r32(a_)) }, rt::fneg({ let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(60); F32(c.m.r32(a_)) })) as i32)) != 0) {
                { let v_ = rt::fneg({ let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(60); F32(c.m.r32(a_)) }); let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(16); c.m.w32(a_, (v_).0); };
            }
            l_cur_6 = { let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(16); F32(c.m.r32(a_)) };
            l_max_5 = { let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(60); F32(c.m.r32(a_)) };
        }
        l_m_4 = (l_view_3).wrapping_add(64);
        if (((rt::fgt(l_cur_6, l_max_5) as i32)) != 0) {
            { let v_ = l_max_5; let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(16); c.m.w32(a_, (v_).0); };
        }
        { let r_ = rt::fmul(l_z_9, F32(0x3f000000)); let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(48); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if (((rt::flt({ let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(48); F32(c.m.r32(a_)) }, { let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(52); F32(c.m.r32(a_)) }) as i32)) != 0) {
            { let v_ = { let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(52); F32(c.m.r32(a_)) }; let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(48); c.m.w32(a_, (v_).0); };
        }
        if (((rt::fgt({ let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(48); F32(c.m.r32(a_)) }, { let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(56); F32(c.m.r32(a_)) }) as i32)) != 0) {
            { let v_ = { let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(56); F32(c.m.r32(a_)) }; let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(48); c.m.w32(a_, (v_).0); };
        }
        { let v_ = rt::fmul({ let a0_ = { let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(16); F32(c.m.r32(a_)) }; F::Port_cosf(c, a0_) }, { let a0_ = { let a_ = ((c.m.r32(sym::gOrbitCam)).wrapping_add(16)).wrapping_add(4); F32(c.m.r32(a_)) }; F::Port_sinf(c, a0_) }); let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fneg({ let a0_ = { let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(16); F32(c.m.r32(a_)) }; F::Port_sinf(c, a0_) }); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul({ let a0_ = { let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(16); F32(c.m.r32(a_)) }; F::Port_cosf(c, a0_) }, { let a0_ = { let a_ = ((c.m.r32(sym::gOrbitCam)).wrapping_add(16)).wrapping_add(4); F32(c.m.r32(a_)) }; F::Port_cosf(c, a0_) }); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let a2_ = { let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(48); F32(c.m.r32(a_)) }; F::Vec4_Scale(c, fp, fp, a2_) };
        { let a1_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(32); F::Vec4_Sub(c, fp, a1_, fp) };
        { let a3_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(16); F::View_SetTransform(c, l_view_3, l_m_4, fp, a3_) };
        { let _ = fp.wrapping_add(16); };
        { let _ = fp.wrapping_add(80); };
        F::View_UpdateMatrices(c, l_view_3, fp);
        F::View_Apply(c, l_view_3, 1i32);
        { let a0_ = c.m.r32(sym::gOrbitCam); F::Vec4_Copy(c, a0_, fp) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "OrbitCam_Update", &PROBE_N, &[], true); }
    r_
}

pub fn OrbitCam_SetTarget(c: &mut Ctx, mut p_x_1: F32, mut p_y_2: F32, mut p_z_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "OrbitCam_SetTarget", &PROBE_N, &[(p_x_1).0, (p_y_2).0, (p_z_3).0], false); }
    let r_ = 'f: {
    { let v_ = p_x_1; let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(32); c.m.w32(a_, (v_).0); };
    { let v_ = p_y_2; let a_ = ((c.m.r32(sym::gOrbitCam)).wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = p_z_3; let a_ = ((c.m.r32(sym::gOrbitCam)).wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = ((c.m.r32(sym::gOrbitCam)).wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "OrbitCam_SetTarget", &PROBE_N, &[], true); }
    r_
}

pub fn OrbitCam_SetAngles(c: &mut Ctx, mut p_pitch_1: F32, mut p_yaw_2: F32, mut p_roll_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "OrbitCam_SetAngles", &PROBE_N, &[(p_pitch_1).0, (p_yaw_2).0, (p_roll_3).0], false); }
    let r_ = 'f: {
    { let v_ = p_pitch_1; let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(16); c.m.w32(a_, (v_).0); };
    { let v_ = p_yaw_2; let a_ = ((c.m.r32(sym::gOrbitCam)).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = p_roll_3; let a_ = ((c.m.r32(sym::gOrbitCam)).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = ((c.m.r32(sym::gOrbitCam)).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "OrbitCam_SetAngles", &PROBE_N, &[], true); }
    r_
}

pub fn OrbitCam_SetDist(c: &mut Ctx, mut p_dist_1: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "OrbitCam_SetDist", &PROBE_N, &[(p_dist_1).0], false); }
    let r_ = 'f: {
    { let v_ = p_dist_1; let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(48); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "OrbitCam_SetDist", &PROBE_N, &[], true); }
    r_
}

pub fn OrbitCam_SetDistRange(c: &mut Ctx, mut p_min_1: F32, mut p_max_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "OrbitCam_SetDistRange", &PROBE_N, &[(p_min_1).0, (p_max_2).0], false); }
    let r_ = 'f: {
    { let v_ = p_min_1; let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(52); c.m.w32(a_, (v_).0); };
    { let v_ = p_max_2; let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(56); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "OrbitCam_SetDistRange", &PROBE_N, &[], true); }
    r_
}

pub fn OrbitCam_SetPitchLimit(c: &mut Ctx, mut p_limit_1: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "OrbitCam_SetPitchLimit", &PROBE_N, &[(p_limit_1).0], false); }
    let r_ = 'f: {
    { let v_ = p_limit_1; let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(60); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "OrbitCam_SetPitchLimit", &PROBE_N, &[], true); }
    r_
}

pub fn OrbitCam_GetTarget(c: &mut Ctx, mut p_out_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "OrbitCam_GetTarget", &PROBE_N, &[(p_out_1) as u32], false); }
    let r_ = 'f: {
    { let a1_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(32); F::Vec4_Copy(c, p_out_1, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "OrbitCam_GetTarget", &PROBE_N, &[], true); }
    r_
}

pub fn OrbitCam_GetAngles(c: &mut Ctx, mut p_out_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "OrbitCam_GetAngles", &PROBE_N, &[(p_out_1) as u32], false); }
    let r_ = 'f: {
    { let a1_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(16); F::Vec4_Copy(c, p_out_1, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "OrbitCam_GetAngles", &PROBE_N, &[], true); }
    r_
}

pub fn OrbitCam_GetDist(c: &mut Ctx) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "OrbitCam_GetDist", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(48); F32(c.m.r32(a_)) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "OrbitCam_GetDist", &PROBE_N, &[], true); }
    r_
}

pub fn OrbitCam_GetDistRange(c: &mut Ctx, mut p_min_1: u32, mut p_max_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "OrbitCam_GetDistRange", &PROBE_N, &[(p_min_1) as u32, (p_max_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = { let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(52); F32(c.m.r32(a_)) }; let a_ = p_min_1; c.m.w32(a_, (v_).0); };
    { let v_ = { let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(56); F32(c.m.r32(a_)) }; let a_ = p_max_2; c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "OrbitCam_GetDistRange", &PROBE_N, &[], true); }
    r_
}

pub fn OrbitCam_GetPitchLimit(c: &mut Ctx) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "OrbitCam_GetPitchLimit", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(60); F32(c.m.r32(a_)) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "OrbitCam_GetPitchLimit", &PROBE_N, &[], true); }
    r_
}

pub fn OrbitCam_ClearAngles(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "OrbitCam_ClearAngles", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = F32(0x00000000); let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(16); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = ((c.m.r32(sym::gOrbitCam)).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "OrbitCam_ClearAngles", &PROBE_N, &[], true); }
    r_
}

pub fn OrbitCam_SetPadLocked(c: &mut Ctx, mut p_locked_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "OrbitCam_SetPadLocked", &PROBE_N, &[(p_locked_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_locked_1; let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "OrbitCam_SetPadLocked", &PROBE_N, &[], true); }
    r_
}

pub fn OrbitCam_SetFloorClamp(c: &mut Ctx, mut p_clamp_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "OrbitCam_SetFloorClamp", &PROBE_N, &[(p_clamp_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_clamp_1; let a_ = (c.m.r32(sym::gOrbitCam)).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "OrbitCam_SetFloorClamp", &PROBE_N, &[], true); }
    r_
}
