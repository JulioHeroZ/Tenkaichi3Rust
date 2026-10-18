#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn View_BuildProjection(c: &mut Ctx, mut p_view_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "View_BuildProjection", &PROBE_N, &[(p_view_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_zScale_2: F32 = F32(0);
    let mut l_zOfs_3: F32 = F32(0);
    let mut l_hw_4: F32 = F32(0);
    let mut l_hh_5: F32 = F32(0);
    let mut l_ox_6: F32 = F32(0);
    let mut l_oy_7: F32 = F32(0);
    let mut l_l_8: F32 = F32(0);
    let mut l_r_9: F32 = F32(0);
    let mut l_t_10: F32 = F32(0);
    let mut l_b_11: F32 = F32(0);
    let mut l_s_12: F32 = F32(0);
    let r_ = 'f: {
        l_zOfs_3 = rt::fdiv(rt::fadd(rt::fmul(rt::fneg(F32(c.m.r32((p_view_1).wrapping_add(588)))), F32(c.m.r32((p_view_1).wrapping_add(592)))), rt::fmul(F32(c.m.r32((p_view_1).wrapping_add(584))), F32(c.m.r32((p_view_1).wrapping_add(596))))), rt::fsub(F32(c.m.r32((p_view_1).wrapping_add(596))), F32(c.m.r32((p_view_1).wrapping_add(592)))));
        l_hw_4 = rt::fdiv(rt::fmul(F32(c.m.r32((p_view_1).wrapping_add(592))), F32(c.m.r32((p_view_1).wrapping_add(528)))), F32(c.m.r32((p_view_1).wrapping_add(564))));
        l_hh_5 = rt::fdiv(rt::fmul(F32(c.m.r32((p_view_1).wrapping_add(592))), F32(c.m.r32(((p_view_1).wrapping_add(528)).wrapping_add(4)))), F32(c.m.r32((p_view_1).wrapping_add(564))));
        l_zScale_2 = rt::fdiv(rt::fmul(rt::fmul(F32(c.m.r32((p_view_1).wrapping_add(596))), F32(c.m.r32((p_view_1).wrapping_add(592)))), rt::fsub(F32(c.m.r32((p_view_1).wrapping_add(588))), F32(c.m.r32((p_view_1).wrapping_add(584))))), rt::fsub(F32(c.m.r32((p_view_1).wrapping_add(596))), F32(c.m.r32((p_view_1).wrapping_add(592)))));
        l_ox_6 = rt::fdiv(rt::fsub(F32(c.m.r32((p_view_1).wrapping_add(576))), F32(0x45000000)), rt::fmul(F32(c.m.r32((p_view_1).wrapping_add(528))), F32(c.m.r32((p_view_1).wrapping_add(568)))));
        l_oy_7 = rt::fdiv(rt::fsub(F32(c.m.r32((p_view_1).wrapping_add(580))), F32(0x45000000)), rt::fmul(F32(c.m.r32(((p_view_1).wrapping_add(528)).wrapping_add(4))), F32(c.m.r32((p_view_1).wrapping_add(572)))));
        F::Mtx_StoreIdentity(c, (p_view_1).wrapping_add(128));
        { let v_ = rt::fmul(F32(c.m.r32((p_view_1).wrapping_add(564))), F32(c.m.r32((p_view_1).wrapping_add(600)))); let a_ = (((p_view_1).wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32((p_view_1).wrapping_add(564))), F32(c.m.r32((p_view_1).wrapping_add(600)))); let a_ = (((p_view_1).wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (((p_view_1).wrapping_add(128)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (((p_view_1).wrapping_add(128)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (((p_view_1).wrapping_add(128)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (((p_view_1).wrapping_add(128)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        F::Mtx_StoreIdentity(c, fp);
        { let v_ = F32(c.m.r32((p_view_1).wrapping_add(568))); let a_ = ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((p_view_1).wrapping_add(572))); let a_ = ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = l_zScale_2; let a_ = ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((p_view_1).wrapping_add(576))); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((p_view_1).wrapping_add(580))); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = l_zOfs_3; let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        F::Mtx_Mul(c, (p_view_1).wrapping_add(128), fp, (p_view_1).wrapping_add(128));
        l_s_12 = rt::fdiv(F32(c.m.r32((p_view_1).wrapping_add(592))), F32(c.m.r32((p_view_1).wrapping_add(564))));
        l_l_8 = rt::fmul(rt::fdiv(rt::fsub(F32(0xc3800000), rt::fsub(F32(c.m.r32((p_view_1).wrapping_add(576))), F32(0x45000000))), F32(c.m.r32((p_view_1).wrapping_add(568)))), l_s_12);
        l_r_9 = rt::fmul(rt::fdiv(rt::fsub(F32(0x43800000), rt::fsub(F32(c.m.r32((p_view_1).wrapping_add(576))), F32(0x45000000))), F32(c.m.r32((p_view_1).wrapping_add(568)))), l_s_12);
        l_b_11 = rt::fmul(rt::fdiv(rt::fsub(F32(0xc3600000), rt::fsub(F32(c.m.r32((p_view_1).wrapping_add(580))), F32(0x45000000))), F32(c.m.r32((p_view_1).wrapping_add(572)))), rt::fneg(l_s_12));
        l_t_10 = rt::fmul(rt::fdiv(rt::fsub(F32(0x43600000), rt::fsub(F32(c.m.r32((p_view_1).wrapping_add(580))), F32(0x45000000))), F32(c.m.r32((p_view_1).wrapping_add(572)))), rt::fneg(l_s_12));
        l_s_12 = rt::fdiv(F32(0x3f800000), rt::fsub(l_r_9, l_l_8));
        { let v_ = rt::fmul(rt::fmul(F32(0x40000000), F32(c.m.r32((p_view_1).wrapping_add(592)))), l_s_12); let a_ = (((p_view_1).wrapping_add(448)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (((p_view_1).wrapping_add(448)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(l_r_9, l_l_8), l_s_12); let a_ = (((p_view_1).wrapping_add(448)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (((p_view_1).wrapping_add(448)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        l_s_12 = rt::fdiv(F32(0x3f800000), rt::fsub(l_b_11, l_t_10));
        { let v_ = F32(0x00000000); let a_ = (((p_view_1).wrapping_add(448)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fmul(F32(0x40000000), F32(c.m.r32((p_view_1).wrapping_add(592)))), l_s_12); let a_ = (((p_view_1).wrapping_add(448)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(l_b_11, l_t_10), l_s_12); let a_ = (((p_view_1).wrapping_add(448)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (((p_view_1).wrapping_add(448)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        l_s_12 = rt::fdiv(F32(0x3f800000), rt::fsub(F32(c.m.r32((p_view_1).wrapping_add(596))), F32(c.m.r32((p_view_1).wrapping_add(592)))));
        { let v_ = F32(0x00000000); let a_ = (((p_view_1).wrapping_add(448)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (((p_view_1).wrapping_add(448)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fneg(F32(c.m.r32((p_view_1).wrapping_add(592)))), l_s_12); let a_ = (((p_view_1).wrapping_add(448)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fneg(rt::fmul(F32(c.m.r32((p_view_1).wrapping_add(596))), F32(c.m.r32((p_view_1).wrapping_add(592))))), l_s_12); let a_ = (((p_view_1).wrapping_add(448)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (((p_view_1).wrapping_add(448)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (((p_view_1).wrapping_add(448)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0xbf800000); let a_ = (((p_view_1).wrapping_add(448)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (((p_view_1).wrapping_add(448)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        F::Mtx_StoreIdentity(c, (p_view_1).wrapping_add(192));
        { let v_ = rt::fmul(rt::fdiv(rt::fmul(F32(0x40000000), F32(c.m.r32((p_view_1).wrapping_add(592)))), rt::fmul(F32(0x40000000), l_hw_4)), F32(c.m.r32((p_view_1).wrapping_add(600)))); let a_ = (((p_view_1).wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fdiv(rt::fmul(F32(0x40000000), F32(c.m.r32((p_view_1).wrapping_add(592)))), rt::fmul(F32(0x40000000), l_hh_5)), F32(c.m.r32((p_view_1).wrapping_add(600)))); let a_ = (((p_view_1).wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fadd(F32(c.m.r32((p_view_1).wrapping_add(596))), F32(c.m.r32((p_view_1).wrapping_add(592)))), rt::fsub(F32(c.m.r32((p_view_1).wrapping_add(596))), F32(c.m.r32((p_view_1).wrapping_add(592))))); let a_ = (((p_view_1).wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (((p_view_1).wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fmul(rt::fmul(F32(c.m.r32((p_view_1).wrapping_add(596))), F32(c.m.r32((p_view_1).wrapping_add(592)))), F32(0xc0000000)), rt::fsub(F32(c.m.r32((p_view_1).wrapping_add(596))), F32(c.m.r32((p_view_1).wrapping_add(592))))); let a_ = (((p_view_1).wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (((p_view_1).wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        F::Mtx_StoreIdentity(c, fp);
        { let v_ = l_ox_6; let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = l_oy_7; let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        F::Mtx_Mul(c, (p_view_1).wrapping_add(192), fp, (p_view_1).wrapping_add(192));
        F::Mtx_StoreIdentity(c, (p_view_1).wrapping_add(256));
        { let v_ = rt::fdiv(rt::fmul(rt::fmul(F32(c.m.r32((p_view_1).wrapping_add(564))), F32(c.m.r32((p_view_1).wrapping_add(568)))), l_hw_4), F32(c.m.r32((p_view_1).wrapping_add(592)))); let a_ = (((p_view_1).wrapping_add(256)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fmul(rt::fmul(F32(c.m.r32((p_view_1).wrapping_add(564))), F32(c.m.r32((p_view_1).wrapping_add(572)))), l_hh_5), F32(c.m.r32((p_view_1).wrapping_add(592)))); let a_ = (((p_view_1).wrapping_add(256)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fsub(F32(c.m.r32((p_view_1).wrapping_add(584))), F32(c.m.r32((p_view_1).wrapping_add(588)))), F32(0x3f000000)); let a_ = (((p_view_1).wrapping_add(256)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x45000000); let a_ = (((p_view_1).wrapping_add(256)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x45000000); let a_ = (((p_view_1).wrapping_add(256)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32((p_view_1).wrapping_add(588))), F32(c.m.r32((p_view_1).wrapping_add(584)))), F32(0x3f000000)); let a_ = (((p_view_1).wrapping_add(256)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (((p_view_1).wrapping_add(256)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "View_BuildProjection", &PROBE_N, &[], true); }
    r_
}

pub fn View_SetProjection(c: &mut Ctx, mut p_view_1: u32, mut p_screenSize_2: u32, mut p_screenDist_3: F32, mut p_aspectX_4: F32, mut p_aspectY_5: F32, mut p_centerX_6: F32, mut p_centerY_7: F32, mut p_zMin_8: F32, mut p_zMax_9: F32, mut p_nearZ_10: F32, mut p_farZ_11: F32, mut p_unk258_12: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "View_SetProjection", &PROBE_N, &[(p_view_1) as u32, (p_screenSize_2) as u32, (p_screenDist_3).0, (p_aspectX_4).0, (p_aspectY_5).0, (p_centerX_6).0, (p_centerY_7).0, (p_zMin_8).0, (p_zMax_9).0, (p_nearZ_10).0, (p_farZ_11).0, (p_unk258_12).0], false); }
    let mut l_y_13: F32 = F32(0);
    let mut l_x_14: F32 = F32(0);
    let r_ = 'f: {
    { let r_ = F::Port_WideFactor(c); let n_ = rt::fmul(p_aspectY_5, r_); p_aspectY_5 = n_; };
    F::Vec4_Copy(c, (p_view_1).wrapping_add(528), p_screenSize_2);
    { let v_ = p_screenDist_3; let a_ = (p_view_1).wrapping_add(564); c.m.w32(a_, (v_).0); };
    { let v_ = p_centerX_6; let a_ = (p_view_1).wrapping_add(576); c.m.w32(a_, (v_).0); };
    { let v_ = p_centerY_7; let a_ = (p_view_1).wrapping_add(580); c.m.w32(a_, (v_).0); };
    { let v_ = p_zMin_8; let a_ = (p_view_1).wrapping_add(584); c.m.w32(a_, (v_).0); };
    { let v_ = p_zMax_9; let a_ = (p_view_1).wrapping_add(588); c.m.w32(a_, (v_).0); };
    { let v_ = p_nearZ_10; let a_ = (p_view_1).wrapping_add(592); c.m.w32(a_, (v_).0); };
    { let v_ = p_farZ_11; let a_ = (p_view_1).wrapping_add(596); c.m.w32(a_, (v_).0); };
    { let v_ = p_unk258_12; let a_ = (p_view_1).wrapping_add(600); c.m.w32(a_, (v_).0); };
    { let v_ = p_aspectX_4; let a_ = (p_view_1).wrapping_add(568); c.m.w32(a_, (v_).0); };
    { let v_ = p_aspectY_5; let a_ = (p_view_1).wrapping_add(572); c.m.w32(a_, (v_).0); };
    if (((rt::fgt(p_aspectX_4, F32(0x3f800000)) as i32)) != 0) {
        { let v_ = F32(0x3f800000); let a_ = (p_view_1).wrapping_add(568); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(p_aspectY_5, p_aspectX_4); let a_ = (p_view_1).wrapping_add(572); c.m.w32(a_, (v_).0); };
    }
    if (((rt::fgt(F32(c.m.r32((p_view_1).wrapping_add(572))), F32(0x3f800000)) as i32)) != 0) {
        l_y_13 = F32(c.m.r32((p_view_1).wrapping_add(572)));
        l_x_14 = F32(c.m.r32((p_view_1).wrapping_add(568)));
        { let v_ = F32(0x3f800000); let a_ = (p_view_1).wrapping_add(572); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(l_x_14, l_y_13); let a_ = (p_view_1).wrapping_add(568); c.m.w32(a_, (v_).0); };
    }
    F::View_BuildProjection(c, p_view_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "View_SetProjection", &PROBE_N, &[], true); }
    r_
}

pub fn View_UpdateMatrices(c: &mut Ctx, mut p_view_1: u32, mut p_pos_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "View_UpdateMatrices", &PROBE_N, &[(p_view_1) as u32, (p_pos_2) as u32], false); }
    let r_ = 'f: {
    F::Vu0Screen_SetMulMtx(c, (p_view_1).wrapping_add(128), (p_view_1).wrapping_add(64));
    F::Vu0Screen_StoreMtx(c, (p_view_1).wrapping_add(320));
    F::Vu0Clip_SetMulMtx(c, (p_view_1).wrapping_add(192), (p_view_1).wrapping_add(64));
    F::Vu0Clip_StoreMtx(c, (p_view_1).wrapping_add(384));
    F::Vec4_Copy(c, (p_view_1).wrapping_add(544), p_pos_2);
    { let v_ = p_view_1; let a_ = sym::gBtlCamView; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "View_UpdateMatrices", &PROBE_N, &[], true); }
    r_
}

pub fn View_SetTransform(c: &mut Ctx, mut p_dst0_1: u32, mut p_dst1_2: u32, mut p_pos_3: u32, mut p_rot_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "View_SetTransform", &PROBE_N, &[(p_dst0_1) as u32, (p_dst1_2) as u32, (p_pos_3) as u32, (p_rot_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let r_ = 'f: {
        { let v_ = rt::fneg(F32(c.m.r32(p_pos_3))); let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fneg(F32(c.m.r32((p_pos_3).wrapping_add(4)))); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fneg(F32(c.m.r32((p_pos_3).wrapping_add(8)))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (p_rot_4).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vu0View_SetRotTrans(c, p_rot_4, fp);
        F::Vu0View_StoreMtx(c, fp.wrapping_add(16));
        F::Mtx_Copy(c, p_dst0_1, fp.wrapping_add(16));
        F::Mtx_Copy(c, p_dst1_2, fp.wrapping_add(16));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "View_SetTransform", &PROBE_N, &[], true); }
    r_
}

pub fn View_Apply(c: &mut Ctx, mut p_view_1: u32, mut p_scissor_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "View_Apply", &PROBE_N, &[(p_view_1) as u32, (p_scissor_2) as u32], false); }
    let r_ = 'f: {
    F::Vu0Screen_LoadMtx(c, (p_view_1).wrapping_add(320));
    F::Vu0Clip_LoadMtx(c, (p_view_1).wrapping_add(384));
    if (((((p_scissor_2) != (0i32)) as i32)) != 0) {
        F::View_ApplyScissor(c, p_view_1);
    }
    { let v_ = p_view_1; let a_ = sym::gBtlCamView; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "View_Apply", &PROBE_N, &[], true); }
    r_
}

pub fn View_GetDistXZ(c: &mut Ctx, mut p_pos_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "View_GetDistXZ", &PROBE_N, &[(p_pos_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let r_ = 'f: {
        { let a1_ = F32(c.m.r32(p_pos_1)); let a3_ = F32(c.m.r32((p_pos_1).wrapping_add(8))); F::Vec4_Set(c, fp, a1_, F32(0x00000000), a3_, F32(0x3f800000)) };
        { let a1_ = { let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(544); F32(c.m.r32(a_)) }; let a3_ = { let a_ = ((c.m.r32(sym::gBtlCamView)).wrapping_add(544)).wrapping_add(8); F32(c.m.r32(a_)) }; F::Vec4_Set(c, fp.wrapping_add(16), a1_, F32(0x00000000), a3_, F32(0x3f800000)) };
        break 'f F::Vec3_DistSq(c, fp, fp.wrapping_add(16));
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "View_GetDistXZ", &PROBE_N, &[], true); }
    r_
}

pub fn View_InitLayout(c: &mut Ctx, mut p_view_1: u32, mut p_layout_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "View_InitLayout", &PROBE_N, &[(p_view_1) as u32, (p_layout_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_split_3: i32 = 0;
    let mut l_left_4: i32 = 0;
    let r_ = 'f: {
        { let v_ = F32(0x3f955555); let a_ = (p_view_1).wrapping_add(560); c.m.w32(a_, (v_).0); };
        's1: {
            let sv5: i32 = p_layout_2;
            match sv5 {
                0i32 => {
                    l_split_3 = 0i32;
                    l_left_4 = 0i32;
                    break 's1;
                }
                1i32 => {
                    l_split_3 = 1i32;
                    l_left_4 = 1i32;
                    break 's1;
                }
                2i32 => {
                    l_split_3 = 1i32;
                    l_left_4 = 0i32;
                    break 's1;
                }
                _ => {
                    break 'f;
                }
            }
        }
        if (((((l_split_3) == (0i32)) as i32)) != 0) {
            F::Vec4_Set(c, fp, F32(0x44ffe000), F32(0x44ffe000), F32(0x3dcccccc), F32(0x4b7fffff));
            { let a4_ = F32(c.m.r32((p_view_1).wrapping_add(560))); F::View_SetProjection(c, p_view_1, fp, F32(0x43d88000), F32(0x3f800000), a4_, F32(0x45000000), F32(0x45000000), F32(0x3f800000), F32(0x4b25f848), F32(0x3e999999), F32(0x47800000), F32(0x3f800000)) };
            { let v_ = 0i32; let a_ = (p_view_1).wrapping_add(512); c.m.w32(a_, (v_) as u32); };
            { let v_ = 511i32; let a_ = (p_view_1).wrapping_add(516); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (p_view_1).wrapping_add(520); c.m.w32(a_, (v_) as u32); };
            { let v_ = 447i32; let a_ = (p_view_1).wrapping_add(524); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = F32(0x44efe000); let a_ = fp; c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x44ffe000); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3dcccccc); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x4b7fffff); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
            if (((((l_left_4) == (1i32)) as i32)) != 0) {
                { let a4_ = F32(c.m.r32((p_view_1).wrapping_add(560))); F::View_SetProjection(c, p_view_1, fp, F32(0x43d88000), F32(0x3f800000), a4_, F32(0x44f00000), F32(0x45000000), F32(0x3f800000), F32(0x4b25f848), F32(0x3e999999), F32(0x47800000), F32(0x3f800000)) };
                { let v_ = 0i32; let a_ = (p_view_1).wrapping_add(512); c.m.w32(a_, (v_) as u32); };
                { let v_ = 254i32; let a_ = (p_view_1).wrapping_add(516); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (p_view_1).wrapping_add(520); c.m.w32(a_, (v_) as u32); };
                { let v_ = 447i32; let a_ = (p_view_1).wrapping_add(524); c.m.w32(a_, (v_) as u32); };
            } else {
                { let a4_ = F32(c.m.r32((p_view_1).wrapping_add(560))); F::View_SetProjection(c, p_view_1, fp, F32(0x43d88000), F32(0x3f800000), a4_, F32(0x45080000), F32(0x45000000), F32(0x3f800000), F32(0x4b25f848), F32(0x3e999999), F32(0x47800000), F32(0x3f800000)) };
                { let v_ = 257i32; let a_ = (p_view_1).wrapping_add(512); c.m.w32(a_, (v_) as u32); };
                { let v_ = 511i32; let a_ = (p_view_1).wrapping_add(516); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (p_view_1).wrapping_add(520); c.m.w32(a_, (v_) as u32); };
                { let v_ = 447i32; let a_ = (p_view_1).wrapping_add(524); c.m.w32(a_, (v_) as u32); };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "View_InitLayout", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCam_SetViewLayout(c: &mut Ctx, mut p_view_1: u32, mut p_layout_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCam_SetViewLayout", &PROBE_N, &[(p_view_1) as u32, (p_layout_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_layout_2) != (0i32)) as i32)) != 0) {
        if (((((p_layout_2) < (0i32)) as i32)) != 0) {
            break 'f;
        }
        if (((((p_layout_2) >= (3i32)) as i32)) != 0) {
            break 'f;
        }
        { let v_ = 1i32; let a_ = (p_view_1).wrapping_add(640); c.m.w32(a_, (v_) as u32); };
    } else {
        { let v_ = 0i32; let a_ = (p_view_1).wrapping_add(640); c.m.w32(a_, (v_) as u32); };
    }
    { let s_ = (c.m.r32(sym::gBtlCam)).wrapping_add((((p_layout_2) as u32)).wrapping_mul(608)); let d_ = p_view_1; rt::memcpy(c, d_, s_, 608); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCam_SetViewLayout", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCam_InitViews(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCam_InitViews", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    { let a0_ = (c.m.r32(sym::gBtlCam)).wrapping_add((((0i32) as u32)).wrapping_mul(608)); F::View_InitLayout(c, a0_, 0i32) };
    { let a0_ = (c.m.r32(sym::gBtlCam)).wrapping_add((((1i32) as u32)).wrapping_mul(608)); F::View_InitLayout(c, a0_, 1i32) };
    { let a0_ = (c.m.r32(sym::gBtlCam)).wrapping_add((((2i32) as u32)).wrapping_mul(608)); F::View_InitLayout(c, a0_, 2i32) };
    if ((F::Battle_IsSplitScreen(c)) != 0) {
        {
            l_i_1 = 0i32;
            'l1: loop {
                if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = l_i_1; let a_ = (((c.m.r32(sym::gBtlCam)).wrapping_add(1824)).wrapping_add((((l_i_1) as u32)).wrapping_mul(656))).wrapping_add(644); c.m.w32(a_, (v_) as u32); };
                    { let a0_ = ((c.m.r32(sym::gBtlCam)).wrapping_add(1824)).wrapping_add((((l_i_1) as u32)).wrapping_mul(656)); F::BtlCam_SetViewLayout(c, a0_, (if (((((l_i_1) == (0i32)) as i32)) != 0) { 1i32 } else { 2i32 })) };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
    } else {
        {
            l_i_1 = 0i32;
            'l3: loop {
                if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let v_ = l_i_1; let a_ = (((c.m.r32(sym::gBtlCam)).wrapping_add(1824)).wrapping_add((((l_i_1) as u32)).wrapping_mul(656))).wrapping_add(644); c.m.w32(a_, (v_) as u32); };
                    { let a0_ = ((c.m.r32(sym::gBtlCam)).wrapping_add(1824)).wrapping_add((((l_i_1) as u32)).wrapping_mul(656)); F::BtlCam_SetViewLayout(c, a0_, 0i32) };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCam_InitViews", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCam_BuildView(c: &mut Ctx, mut p_view_1: u32, mut p_apply_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCam_BuildView", &PROBE_N, &[(p_view_1) as u32, (p_apply_2) as u32], false); }
    let r_ = 'f: {
    F::View_SetTransform(c, p_view_1, (p_view_1).wrapping_add(64), (p_view_1).wrapping_add(608), (p_view_1).wrapping_add(624));
    F::View_UpdateMatrices(c, p_view_1, (p_view_1).wrapping_add(608));
    if (((((p_apply_2) != (0i32)) as i32)) != 0) {
        F::View_Apply(c, p_view_1, 1i32);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCam_BuildView", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCam_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCam_Reset", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let _ = { let a0_ = c.m.r32(sym::gBtlCam); F::memset(c, a0_, 0i32, 3152u32) }; };
    F::BtlCam_InitViews(c);
    { let v_ = ((c.m.r32(sym::gBtlCam)).wrapping_add(1824)).wrapping_add((((0i32) as u32)).wrapping_mul(656)); let a_ = (c.m.r32(sym::gBtlCam)).wrapping_add(3136); c.m.w32(a_, v_); };
    F::DemoCam_Reset(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCam_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCam_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCam_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, ((3152u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gBtlCam; c.m.w32(a_, v_); };
    F::DemoCam_Init(c);
    F::BtlCam_Reset(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCam_Init", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCam_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCam_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32(sym::gBtlCam); F::Heap_Free(c, a0_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gBtlCam; c.m.w32(a_, v_); };
    F::DemoCam_Term(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCam_Term", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCam_UpdateView(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCam_UpdateView", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_view_2: u32 = 0;
    let mut l_objId_3: i32 = 0;
    let r_ = 'f: {
    l_view_2 = { let a_ = (c.m.r32(sym::gBtlCam)).wrapping_add(3136); c.m.r32(a_) };
    l_objId_3 = F::BattleSide_GetObjId(c, p_side_1);
    if ((F::Battle_IsSplitScreen(c)) != 0) {
        F::BtlCam_SetLayout(c, 1i32, 0i32);
    }
    { let _ = F::BtlCharApi_GetCamPose(c, l_objId_3, (l_view_2).wrapping_add(608), (l_view_2).wrapping_add(624)); };
    { let v_ = F::BtlCharApi_HasCamPriority(c, l_objId_3); let a_ = (l_view_2).wrapping_add(648); c.m.w32(a_, (v_) as u32); };
    F::BtlCam_BuildView(c, l_view_2, 1i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCam_UpdateView", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCam_TraceStage(c: &mut Ctx, mut p_out_1: u32, mut p_from_2: u32, mut p_to_3: u32, mut p_frac_4: u32, mut p_hitObj_5: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCam_TraceStage", &PROBE_N, &[(p_out_1) as u32, (p_from_2) as u32, (p_to_3) as u32, (p_frac_4) as u32, (p_hitObj_5) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(160) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 160);
    let mut l_objA_6: i32 = 0;
    let mut l_objB_7: i32 = 0;
    let mut l_obj_8: i32 = 0;
    let mut l_ret_9: i32 = 0;
    let mut l_f_10: F32 = F32(0);
    let r_ = 'f: {
        F::Vec4_Copy(c, p_out_1, p_from_2);
        if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((8192i32) as u64)))) != 0) {
            if (((((p_frac_4) != (((0i32) as u32))) as i32)) != 0) {
                { let v_ = F32(0x3f800000); let a_ = p_frac_4; c.m.w32(a_, (v_).0); };
            }
            break 'f 0i32;
        }
        F::Vec4_Copy(c, fp, p_to_3);
        F::Vec4_Copy(c, fp.wrapping_add(16), p_from_2);
        l_objA_6 = F::BtlStage_FindZoneNear(c, (1i32).wrapping_neg(), fp);
        l_objB_7 = F::BtlStage_FindZoneNear(c, (1i32).wrapping_neg(), fp.wrapping_add(16));
        F::Vec4_Copy(c, fp.wrapping_add(32), fp);
        F::Vec4_Copy(c, (fp.wrapping_add(32)).wrapping_add(16), fp.wrapping_add(16));
        { let v_ = F32(0x40000000); let a_ = (fp.wrapping_add(32)).wrapping_add(32); c.m.w32(a_, (v_).0); };
        if (((((l_objA_6) != (l_objB_7)) as i32)) != 0) {
            l_obj_8 = l_objA_6;
            if (((((F::StgCol_TraceSphere(c, l_obj_8, fp.wrapping_add(32), fp.wrapping_add(68), fp.wrapping_add(148), fp.wrapping_add(84))) != (0i32)) as i32)) != 0) {
                F::Vec4_Copy(c, p_out_1, fp.wrapping_add(68));
                l_ret_9 = 1i32;
                l_f_10 = F32(c.m.r32(fp.wrapping_add(148)));
            } else {
                l_obj_8 = l_objB_7;
                if (((((F::StgCol_TraceSphere(c, l_obj_8, fp.wrapping_add(32), fp.wrapping_add(68), fp.wrapping_add(148), fp.wrapping_add(84))) != (0i32)) as i32)) != 0) {
                    F::Vec4_Copy(c, p_out_1, fp.wrapping_add(68));
                    l_ret_9 = 1i32;
                    l_f_10 = F32(c.m.r32(fp.wrapping_add(148)));
                } else {
                    l_ret_9 = 0i32;
                    l_f_10 = F32(0x3f800000);
                }
            }
        } else {
            l_obj_8 = l_objA_6;
            if (((((F::StgCol_TraceSphere(c, l_obj_8, fp.wrapping_add(32), fp.wrapping_add(68), fp.wrapping_add(148), fp.wrapping_add(84))) != (0i32)) as i32)) != 0) {
                F::Vec4_Copy(c, p_out_1, fp.wrapping_add(68));
                l_ret_9 = 1i32;
                l_f_10 = F32(c.m.r32(fp.wrapping_add(148)));
            } else {
                l_f_10 = F32(0x3f800000);
                l_ret_9 = 0i32;
            }
        }
        if (((((p_frac_4) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = l_f_10; let a_ = p_frac_4; c.m.w32(a_, (v_).0); };
        }
        if (((((p_hitObj_5) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = l_obj_8; let a_ = p_hitObj_5; c.m.w32(a_, (v_) as u32); };
        }
        break 'f l_ret_9;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlCam_TraceStage", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCam_GetDefaultView(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCam_GetDefaultView", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((F::Battle_GetMode(c)) == (8i32)) as i32)) != 0) {
        if (((((F::BattleSide_GetControl(c, 0i32)) != (0i32)) as i32)) != 0) {
            break 'f (if (((((F::BattleSide_GetControl(c, 1i32)) == (0i32)) as i32)) != 0) { 1i32 } else { (c.m.r32((sym::D_002FF280).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32) });
        }
        break 'f 0i32;
    }
    if (((((F::BattleReplay_IsActive(c)) != (0i32)) as i32)) != 0) {
        break 'f F::BtlCharApi_GetMgrUnk134(c);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCam_GetDefaultView", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCam_SetLayout(c: &mut Ctx, mut p_split_1: i32, mut p_apply_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCam_SetLayout", &PROBE_N, &[(p_split_1) as u32, (p_apply_2) as u32], false); }
    let mut l_view_3: u32 = 0;
    let r_ = 'f: {
    if ((F::Battle_IsSplitScreen(c)) != 0) {
        l_view_3 = { let a_ = (c.m.r32(sym::gBtlCam)).wrapping_add(3136); c.m.r32(a_) };
        if (((((p_split_1) != ((c.m.r32((l_view_3).wrapping_add(640)) as i32))) as i32)) != 0) {
            if (((((p_split_1) == (1i32)) as i32)) != 0) {
                { let a1_ = (if ((((((c.m.r32((l_view_3).wrapping_add(644)) as i32)) == (0i32)) as i32)) != 0) { 1i32 } else { 2i32 }); F::BtlCam_SetViewLayout(c, l_view_3, a1_) };
            } else {
                F::BtlCam_SetViewLayout(c, l_view_3, 0i32);
            }
            { let a0_ = { let a_ = (c.m.r32(sym::gBtlCam)).wrapping_add(3136); c.m.r32(a_) }; F::BtlCam_BuildView(c, a0_, p_apply_2) };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCam_SetLayout", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCam_GetPriorityView(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCam_GetPriorityView", &PROBE_N, &[], false); }
    let mut l_best_1: i32 = 0;
    let mut l_bestPri_2: i32 = 0;
    let mut l_count_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_pri_5: i32 = 0;
    let r_ = 'f: {
    l_best_1 = (1i32).wrapping_neg();
    l_bestPri_2 = (9999i32).wrapping_neg();
    l_count_3 = 0i32;
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_pri_5 = { let a_ = (((c.m.r32(sym::gBtlCam)).wrapping_add(1824)).wrapping_add((((l_i_4) as u32)).wrapping_mul(656))).wrapping_add(648); (c.m.r32(a_) as i32) };
                if (((((l_bestPri_2) < (l_pri_5)) as i32)) != 0) {
                    l_bestPri_2 = l_pri_5;
                    l_best_1 = l_i_4;
                    l_count_3 = 1i32;
                } else {
                    if (((((l_pri_5) == (l_bestPri_2)) as i32)) != 0) {
                        { let o_ = l_count_3; let n_ = (o_).wrapping_add(1); l_count_3 = n_; };
                    }
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    if (((((l_count_3) >= (2i32)) as i32)) != 0) {
        if (((((l_bestPri_2) > (0i32)) as i32)) != 0) {
            break 'f F::BtlCam_GetDefaultView(c);
        }
        break 'f (1i32).wrapping_neg();
    }
    break 'f l_best_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCam_GetPriorityView", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCam_SelectView(c: &mut Ctx, mut p_idx_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCam_SelectView", &PROBE_N, &[(p_idx_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((c.m.r32(sym::gBtlCam)).wrapping_add(1824)).wrapping_add((((p_idx_1) as u32)).wrapping_mul(656)); let a_ = (c.m.r32(sym::gBtlCam)).wrapping_add(3136); c.m.w32(a_, v_); };
    { let v_ = { let a_ = (c.m.r32(sym::gBtlCam)).wrapping_add(3136); c.m.r32(a_) }; let a_ = sym::gBtlCamView; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCam_SelectView", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCam_ApplyView(c: &mut Ctx, mut p_scissor_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCam_ApplyView", &PROBE_N, &[(p_scissor_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = { let a_ = (c.m.r32(sym::gBtlCam)).wrapping_add(3136); c.m.r32(a_) }; F::View_Apply(c, a0_, p_scissor_1) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCam_ApplyView", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCam_UpdateOverride(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCam_UpdateOverride", &PROBE_N, &[], false); }
    let mut l_ret_1: i32 = 0;
    let mut l_idx_2: i32 = 0;
    let r_ = 'f: {
    l_ret_1 = 0i32;
    if ((((((((!((F::Battle_IsSplitScreen(c)) != 0)) as i32)) != 0) || (((((F::BattleReplay_IsActive(c)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
        l_idx_2 = F::BtlCam_GetPriorityView(c);
        if (((((l_idx_2) < (0i32)) as i32)) != 0) {
            l_idx_2 = F::BtlCam_GetDefaultView(c);
        }
        F::BtlCam_SelectView(c, l_idx_2);
        l_ret_1 = 1i32;
        F::BtlCam_ApplyView(c, 1i32);
        F::BtlCam_SetLayout(c, 0i32, 1i32);
    } else {
        l_idx_2 = F::BtlCam_GetPriorityView(c);
        if (((((l_idx_2) < (0i32)) as i32)) != 0) {
            l_idx_2 = F::Port_NetView(c);
        }
        if (((((l_idx_2) >= (0i32)) as i32)) != 0) {
            F::BtlCam_SelectView(c, l_idx_2);
            l_ret_1 = 1i32;
            F::BtlCam_SetLayout(c, 0i32, 1i32);
        }
    }
    if (((((F::DemoCam_IsActive(c)) != (0i32)) as i32)) != 0) {
        { let _ = F::DemoCam_Update(c); };
        l_ret_1 = 1i32;
    }
    break 'f l_ret_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCam_UpdateOverride", &PROBE_N, &[], true); }
    r_
}

pub fn DbgCam_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DbgCam_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let _ = F::memset(c, sym::gDbgCamView, 0i32, 608u32); };
    F::View_InitLayout(c, sym::gDbgCamView, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DbgCam_Init", &PROBE_N, &[], true); }
    r_
}

pub fn DbgCam_Update(c: &mut Ctx, mut p_view_1: u32, mut p_pos_2: u32, mut p_rot_3: u32, mut p_pad_4: i32, mut p_speed_5: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DbgCam_Update", &PROBE_N, &[(p_view_1) as u32, (p_pos_2) as u32, (p_rot_3) as u32, (p_pad_4) as u32, (p_speed_5).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_p_6: u32 = 0;
    let mut l_x_7: F32 = F32(0);
    let mut l_y_8: F32 = F32(0);
    let mut l_z_9: F32 = F32(0);
    let mut l_pan_10: i32 = 0;
    let r_ = 'f: {
        l_p_6 = (sym::gPad).wrapping_add((((p_pad_4) as u32)).wrapping_mul(448));
        l_x_7 = F32(0x00000000);
        l_y_8 = F32(0x00000000);
        l_z_9 = F32(0x00000000);
        if (((((p_view_1) == (((0i32) as u32))) as i32)) != 0) {
            p_view_1 = sym::gDbgCamView;
        }
        l_pan_10 = 0i32;
        if ((((c.m.r32((l_p_6).wrapping_add(392))) & (((16384i32) as u32)))) != 0) {
            l_pan_10 = 1i32;
        }
        if (((((((rt::fgt(F32(c.m.r32(((l_p_6).wrapping_add(420)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0x3dcccccc)) as i32)) != 0) || (((rt::flt(F32(c.m.r32(((l_p_6).wrapping_add(420)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0xbdcccccc)) as i32)) != 0)) as i32)) != 0) {
            l_x_7 = rt::fadd(rt::fmul(F32(c.m.r32(((l_p_6).wrapping_add(420)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0x3f000000)), l_x_7);
        }
        if (((((((rt::fgt(F32(c.m.r32(((l_p_6).wrapping_add(420)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(0x3dcccccc)) as i32)) != 0) || (((rt::flt(F32(c.m.r32(((l_p_6).wrapping_add(420)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(0xbdcccccc)) as i32)) != 0)) as i32)) != 0) {
            { let r_ = rt::fmul(F32(c.m.r32(((l_p_6).wrapping_add(420)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(0x3f000000)); let n_ = rt::fadd(l_y_8, r_); l_y_8 = n_; };
        }
        if (((((((rt::fgt(F32(c.m.r32(((l_p_6).wrapping_add(428)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(0x3dcccccc)) as i32)) != 0) || (((rt::flt(F32(c.m.r32(((l_p_6).wrapping_add(428)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(0xbdcccccc)) as i32)) != 0)) as i32)) != 0) {
            { let r_ = rt::fmul(F32(c.m.r32(((l_p_6).wrapping_add(428)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(0x3f000000)); let n_ = rt::fadd(l_z_9, r_); l_z_9 = n_; };
        }
        if ((((((c.m.r32(sym::gDbgCamLocked) as i32)) != (0i32)) as i32)) != 0) {
            l_x_7 = F32(0x00000000);
            l_pan_10 = 0i32;
            l_y_8 = F32(0x00000000);
            l_z_9 = F32(0x00000000);
        }
        { let _ = F::memset(c, fp.wrapping_add(64), 0i32, 16u32); };
        { let _ = F::memset(c, fp.wrapping_add(80), 0i32, 16u32); };
        F::Mtx_InverseRT(c, fp, (p_view_1).wrapping_add(64));
        if ((l_pan_10) != 0) {
            F::Vec4_Scale(c, fp.wrapping_add(80), (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), rt::fmul(l_x_7, p_speed_5));
            F::Vec4_Add(c, fp.wrapping_add(64), fp.wrapping_add(64), fp.wrapping_add(80));
            F::Vec4_Scale(c, fp.wrapping_add(80), (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), rt::fmul(l_y_8, p_speed_5));
            F::Vec4_Add(c, fp.wrapping_add(64), fp.wrapping_add(64), fp.wrapping_add(80));
        }
        F::Vec4_Scale(c, fp.wrapping_add(80), (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)), rt::fmul(rt::fneg(l_z_9), p_speed_5));
        F::Vec4_Add(c, fp.wrapping_add(64), fp.wrapping_add(64), fp.wrapping_add(80));
        F::Vec4_Add(c, p_pos_2, p_pos_2, fp.wrapping_add(64));
        if ((((!((l_pan_10) != 0)) as i32)) != 0) {
            { let r_ = rt::fmul(l_y_8, F32(0x3d23d70a)); let a_ = p_rot_3; let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = rt::fmul(l_x_7, F32(0x3d23d70a)); let a_ = (p_rot_3).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::flt(F32(c.m.r32(p_rot_3)), F32(0xbfc8f5c2)) as i32)) != 0) {
                { let v_ = F32(0xbfc8f5c2); let a_ = p_rot_3; c.m.w32(a_, (v_).0); };
            }
            if (((rt::fgt(F32(c.m.r32(p_rot_3)), F32(0x3fc8f5c2)) as i32)) != 0) {
                { let v_ = F32(0x3fc8f5c2); let a_ = p_rot_3; c.m.w32(a_, (v_).0); };
            }
            if (((rt::flt(F32(c.m.r32((p_rot_3).wrapping_add(4))), F32(0xc0490fda)) as i32)) != 0) {
                { let r_ = F32(0x40c90fda); let a_ = (p_rot_3).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            }
            if (((rt::fgt(F32(c.m.r32((p_rot_3).wrapping_add(4))), F32(0x40490fda)) as i32)) != 0) {
                { let r_ = F32(0x40c90fda); let a_ = (p_rot_3).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            }
        }
        F::View_SetTransform(c, p_view_1, (p_view_1).wrapping_add(64), p_pos_2, p_rot_3);
        F::View_UpdateMatrices(c, p_view_1, p_pos_2);
        F::View_Apply(c, p_view_1, 1i32);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DbgCam_Update", &PROBE_N, &[], true); }
    r_
}

pub fn DbgCam_SetLocked(c: &mut Ctx, mut p_locked_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DbgCam_SetLocked", &PROBE_N, &[(p_locked_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_locked_1; let a_ = sym::gDbgCamLocked; c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DbgCam_SetLocked", &PROBE_N, &[], true); }
    r_
}

pub fn CamShake_Add(c: &mut Ctx, mut p_shake_1: u32, mut p_strength_2: F32, mut p_time_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CamShake_Add", &PROBE_N, &[(p_shake_1) as u32, (p_strength_2).0, (p_time_3).0], false); }
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((((rt::fle(F32(c.m.r32((p_shake_1).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)))), p_time_3) as i32)) != 0) && (((rt::fle(F32(c.m.r32(((p_shake_1).wrapping_add(16)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)))), p_strength_2) as i32)) != 0)) as i32)) != 0) {
                    { let v_ = p_time_3; let a_ = (p_shake_1).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = p_strength_2; let a_ = ((p_shake_1).wrapping_add(16)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    break 'f;
                }
                if (((((((rt::fle(p_time_3, F32(c.m.r32((p_shake_1).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))))) as i32)) != 0) && (((rt::fle(p_strength_2, F32(c.m.r32(((p_shake_1).wrapping_add(16)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))))) as i32)) != 0)) as i32)) != 0) {
                    break 'l1;
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "CamShake_Add", &PROBE_N, &[], true); }
    r_
}

pub fn CamShake_Tick(c: &mut Ctx, mut p_shake_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CamShake_Tick", &PROBE_N, &[(p_shake_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let r_ = F32(0x3d088888); let a_ = (p_shake_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::flt(F32(c.m.r32((p_shake_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)))), F32(0x00000000)) as i32)) != 0) {
                    { let v_ = F32(0x00000000); let a_ = (p_shake_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x00000000); let a_ = ((p_shake_1).wrapping_add(16)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "CamShake_Tick", &PROBE_N, &[], true); }
    r_
}

pub fn CamShake_Calc(c: &mut Ctx, mut p_shake_1: u32, mut p_posOfs_2: u32, mut p_rotOfs_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CamShake_Calc", &PROBE_N, &[(p_shake_1) as u32, (p_posOfs_2) as u32, (p_rotOfs_3) as u32], false); }
    let mut l_strength_4: F32 = F32(0);
    let mut l_time_5: F32 = F32(0);
    let mut l_amp_6: F32 = F32(0);
    let mut l_i_7: i32 = 0;
    let r_ = 'f: {
    l_strength_4 = F32(0x00000000);
    l_time_5 = F32(0x00000000);
    {
        l_i_7 = 0i32;
        'l1: loop {
            if !(((((l_i_7) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((rt::flt(l_strength_4, F32(c.m.r32(((p_shake_1).wrapping_add(16)).wrapping_add((((l_i_7) as u32)).wrapping_mul(4))))) as i32)) != 0) {
                    l_time_5 = F32(c.m.r32((p_shake_1).wrapping_add((((l_i_7) as u32)).wrapping_mul(4))));
                    l_strength_4 = F32(c.m.r32(((p_shake_1).wrapping_add(16)).wrapping_add((((l_i_7) as u32)).wrapping_mul(4))));
                }
            }
            { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
        }
    }
    F::Vec4_SetZeroW1(c, p_posOfs_2);
    F::Vec4_SetZero(c, p_rotOfs_3);
    if (((rt::fgt(l_strength_4, F32(0x00000000)) as i32)) != 0) {
        l_amp_6 = rt::fmul(rt::fmul(l_time_5, F32(0x3d4ccccc)), l_strength_4);
        if (((rt::fgt(l_amp_6, F32(0x3dcccccc)) as i32)) != 0) {
            l_amp_6 = F32(0x3dcccccc);
        }
        { let v_ = rt::fmul(rt::fmul(rt::i2f((((F::Port_Rand(c)) & (255i32))).wrapping_sub(127i32)), F32(0x3c000000)), l_amp_6); let a_ = p_rotOfs_3; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fmul(rt::i2f((((F::Port_Rand(c)) & (255i32))).wrapping_sub(127i32)), F32(0x3c000000)), l_amp_6); let a_ = (p_rotOfs_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fmul(rt::i2f((((F::Port_Rand(c)) & (255i32))).wrapping_sub(127i32)), F32(0x3f000000)), l_amp_6); let a_ = p_posOfs_2; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fmul(rt::i2f((((F::Port_Rand(c)) & (255i32))).wrapping_sub(127i32)), F32(0x3f000000)), l_amp_6); let a_ = (p_posOfs_2).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fmul(rt::i2f((((F::Port_Rand(c)) & (255i32))).wrapping_sub(127i32)), F32(0x3f000000)), l_amp_6); let a_ = (p_posOfs_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "CamShake_Calc", &PROBE_N, &[], true); }
    r_
}

pub fn CamShake_GetTime(c: &mut Ctx, mut p_shake_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CamShake_GetTime", &PROBE_N, &[(p_shake_1) as u32], false); }
    let mut l_time_2: F32 = F32(0);
    let mut l_strength_3: F32 = F32(0);
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_time_2 = F32(0x00000000);
    l_strength_3 = F32(0x00000000);
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((rt::flt(l_strength_3, F32(c.m.r32(((p_shake_1).wrapping_add(16)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))))) as i32)) != 0) {
                    l_time_2 = F32(c.m.r32((p_shake_1).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))));
                    l_strength_3 = F32(c.m.r32(((p_shake_1).wrapping_add(16)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))));
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    break 'f l_time_2;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "CamShake_GetTime", &PROBE_N, &[], true); }
    r_
}
