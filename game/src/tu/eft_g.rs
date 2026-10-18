#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn eft_g__EftScr_IsValid(c: &mut Ctx, mut p_p_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_g__EftScr_IsValid", &PROBE_N, &[(p_p_1) as u32], false); }
    let mut l_ok_2: i32 = 0;
    let r_ = 'f: {
    l_ok_2 = 0i32;
    if ((((((((((((((c.m.r32((p_p_1).wrapping_add(8)) as i32)) > (0i32)) as i32)) != 0) && ((((((c.m.r32(p_p_1) as i32)) <= (65535i32)) as i32)) != 0)) as i32)) != 0) && ((((((c.m.r32(p_p_1) as i32)) > (0i32)) as i32)) != 0)) as i32)) != 0) {
        if ((((((c.m.r32((p_p_1).wrapping_add(4)) as i32)) <= (65535i32)) as i32)) != 0) {
            if ((((((c.m.r32((p_p_1).wrapping_add(4)) as i32)) > (0i32)) as i32)) != 0) {
                l_ok_2 = 1i32;
            } else {
                l_ok_2 = 0i32;
            }
        }
    }
    break 'f l_ok_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "eft_g__EftScr_IsValid", &PROBE_N, &[], true); }
    r_
}

pub fn eft_g__EftOt_Add(c: &mut Ctx, mut p_p_1: u32, mut p_z_2: i32, mut p_layer_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_g__EftOt_Add", &PROBE_N, &[(p_p_1) as u32, (p_z_2) as u32, (p_layer_3) as u32], false); }
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
    if rt::probing() { crate::host::harness::probe(c, "eft_g__EftOt_Add", &PROBE_N, &[], true); }
    r_
}

pub fn eft_g__EftOt_Alloc(c: &mut Ctx, mut p_size_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_g__EftOt_Alloc", &PROBE_N, &[(p_size_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
    l_p_2 = c.m.r32(sym::gOtCur);
    { let v_ = (l_p_2).wrapping_add(((p_size_1) as u32)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
    break 'f l_p_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "eft_g__EftOt_Alloc", &PROBE_N, &[], true); }
    r_
}

pub fn EftRec_GetDefClass(c: &mut Ctx, mut p_rec_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRec_GetDefClass", &PROBE_N, &[(p_rec_1) as u32], false); }
    let mut l_cls_2: i32 = 0;
    let mut l_def_3: u32 = 0;
    let mut l_kind_4: i32 = 0;
    let mut l_id_5: i32 = 0;
    let r_ = 'f: {
    l_cls_2 = 0i32;
    if (((((c.m.r32((p_rec_1).wrapping_add(100))) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_def_3 = { let a_ = (c.m.r32((p_rec_1).wrapping_add(100))).wrapping_add(36); c.m.r32(a_) };
    l_kind_4 = (((c.m.r8((l_def_3).wrapping_add(5)) as i8)) as i32);
    l_id_5 = (((c.m.r16(l_def_3) as i16)) as i32);
    if (((((l_kind_4) != (3i32)) as i32)) != 0) {
        if (((((l_kind_4) == (4i32)) as i32)) != 0) {
            l_cls_2 = 2i32;
        }
    } else {
        l_cls_2 = 3i32;
    }
    if (((((((((((((l_id_5) == (456i32)) as i32)) != 0) || (((((l_id_5) == (345i32)) as i32)) != 0)) as i32)) != 0) || (((((l_id_5) == (624i32)) as i32)) != 0)) as i32)) != 0) {
        l_cls_2 = 3i32;
    }
    if (((((l_id_5) == (367i32)) as i32)) != 0) {
        l_cls_2 = 4i32;
    }
    break 'f l_cls_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRec_GetDefClass", &PROBE_N, &[], true); }
    r_
}

pub fn EftUtil_MakeFacingMtx(c: &mut Ctx, mut p_out_1: u32, mut p_dir_2: u32, mut p_pos_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftUtil_MakeFacingMtx", &PROBE_N, &[(p_out_1) as u32, (p_dir_2) as u32, (p_pos_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_d_4: F32 = F32(0);
    let mut l_s_5: F32 = F32(0);
    let r_ = 'f: {
        l_s_5 = F32(0x00000000);
        { let v_ = { let a_ = (((c.m.r32(sym::gBtlCamView)).wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = fp.wrapping_add(64); c.m.w32(a_, (v_).0); };
        { let v_ = { let a_ = (((c.m.r32(sym::gBtlCamView)).wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = (fp.wrapping_add(64)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a_ = (((c.m.r32(sym::gBtlCamView)).wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = (fp.wrapping_add(64)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(64)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        l_d_4 = F::Vec3_Dot(c, fp.wrapping_add(64), p_dir_2);
        if (((rt::flt(l_d_4, l_s_5) as i32)) != 0) {
            l_d_4 = rt::fneg(l_d_4);
            { let v_ = rt::fneg(F32(c.m.r32(fp.wrapping_add(64)))); let a_ = fp; c.m.w32(a_, (v_).0); };
            { let v_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(4)))); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(8)))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
            l_s_5 = rt::fmul(l_d_4, F32(0x3f000000));
        } else {
            l_s_5 = rt::fmul(l_d_4, F32(0x3f000000));
            { let v_ = F32(c.m.r32(fp.wrapping_add(64))); let a_ = fp; c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(4))); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(8))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        }
        F::Vec4_Sub(c, fp, p_dir_2, fp);
        F::Vec3_Normalize(c, fp, fp);
        F::Vec4_Scale(c, fp, fp, l_s_5);
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
    if rt::probing() { crate::host::harness::probe(c, "EftUtil_MakeFacingMtx", &PROBE_N, &[], true); }
    r_
}

pub fn EftUtil_ClipSegToWater(c: &mut Ctx, mut p_out_1: u32, mut p_a_2: u32, mut p_b_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftUtil_ClipSegToWater", &PROBE_N, &[(p_out_1) as u32, (p_a_2) as u32, (p_b_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_da_4: F32 = F32(0);
    let mut l_dd_5: F32 = F32(0);
    let r_ = 'f: {
        { let _ = F::memset(c, fp.wrapping_add(32), 0i32, 16u32); };
        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let _ = F::EftWater_GetSurfaceY(c, (fp.wrapping_add(32)).wrapping_add(4)); };
        { let v_ = F32(0x00000000); let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = F32(0xbf800000); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fneg(F::Vec3_Dot(c, fp, fp.wrapping_add(32))); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec4_Sub(c, fp.wrapping_add(16), p_a_2, p_b_3);
        l_da_4 = F::Vec3_Dot(c, fp, p_a_2);
        l_dd_5 = F::Vec3_Dot(c, fp, fp.wrapping_add(16));
        { let a3_ = rt::fsub(F32(0x3f800000), rt::fdiv(rt::fadd(l_da_4, F32(c.m.r32((fp).wrapping_add(12)))), l_dd_5)); F::Vec4_Lerp(c, p_out_1, p_a_2, p_b_3, a3_) };
        { let v_ = F32(0x3f800000); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftUtil_ClipSegToWater", &PROBE_N, &[], true); }
    r_
}

pub fn EftUtil_IsCamUnderWater(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftUtil_IsCamUnderWater", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let r_ = 'f: {
        { let v_ = F32(0x00000000); let a_ = fp.wrapping_add(64); c.m.w32(a_, (v_).0); };
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(64); F::Mtx_InverseRT(c, fp, a1_) };
        { let _ = F::BtlStage_GetWaterLevel(c, fp.wrapping_add(64)); };
        break 'f (rt::fge(F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(fp.wrapping_add(64)))) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftUtil_IsCamUnderWater", &PROBE_N, &[], true); }
    r_
}

pub fn EftUtil_IsScreenPosClipped(c: &mut Ctx, mut p_p_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftUtil_IsScreenPosClipped", &PROBE_N, &[(p_p_1) as u32], false); }
    let mut l_r_2: i32 = 0;
    let r_ = 'f: {
    if ((((((c.m.r32((p_p_1).wrapping_add(8)) as i32)) <= (0i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    if ((((((c.m.r32(p_p_1) as i32)) > (65519i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    if ((((((c.m.r32(p_p_1) as i32)) <= (0i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    l_r_2 = ((((c.m.r32((p_p_1).wrapping_add(4)) as i32)) < (1i32)) as i32);
    if ((((((c.m.r32((p_p_1).wrapping_add(4)) as i32)) > (65519i32)) as i32)) != 0) {
        l_r_2 = 1i32;
    }
    break 'f l_r_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftUtil_IsScreenPosClipped", &PROBE_N, &[], true); }
    r_
}

pub fn EftUtil_DrawTri(c: &mut Ctx, mut p_xyz0_1: u32, mut p_xyz1_2: u32, mut p_xyz2_3: u32, mut p_col0_4: u32, mut p_col1_5: u32, mut p_col2_6: u32, mut p_st0_7: u32, mut p_st1_8: u32, mut p_st2_9: u32, mut p_layer_10: i32, mut p_z_11: i32, mut p_tex0_12: u64) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftUtil_DrawTri", &PROBE_N, &[(p_xyz0_1) as u32, (p_xyz1_2) as u32, (p_xyz2_3) as u32, (p_col0_4) as u32, (p_col1_5) as u32, (p_col2_6) as u32, (p_st0_7) as u32, (p_st1_8) as u32, (p_st2_9) as u32, (p_layer_10) as u32, (p_z_11) as u32, (p_tex0_12) as u32], false); }
    let mut l_gif1_13: u64 = 0;
    let mut l_p_14: u32 = 0;
    let r_ = 'f: {
    l_gif1_13 = ((268423679713632i64) as u64);
    l_p_14 = c.m.r32(sym::gOtCur);
    { let v_ = (l_p_14).wrapping_add((((1i32) as u32)).wrapping_mul(128)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
    { let v_ = ((536870919i32) as u32); let a_ = l_p_14; c.m.w32(a_, v_); };
    { let v_ = ((1342177287i32) as u32); let a_ = (l_p_14).wrapping_add(12); c.m.w32(a_, v_); };
    { let v_ = ((91i32) as u64); let a_ = (l_p_14).wrapping_add(32); c.m.w64(a_, v_); };
    { let v_ = ((268435456i32) as u32); let a_ = (l_p_14).wrapping_add(8); c.m.w32(a_, v_); };
    { let v_ = 14123288431433908225u64; let a_ = (l_p_14).wrapping_add(16); c.m.w64(a_, v_); };
    { let v_ = l_gif1_13; let a_ = (l_p_14).wrapping_add(24); c.m.w64(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_14).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u64); let a_ = (l_p_14).wrapping_add(120); c.m.w64(a_, v_); };
    { let v_ = ((rt::f2u(F32(c.m.r32((p_col0_4).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))) as u8); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2u(F32(c.m.r32((p_col0_4).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))) as u8); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2u(F32(c.m.r32((p_col0_4).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))) as u8); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2u(F32(c.m.r32((p_col0_4).wrapping_add((((3i32) as u32)).wrapping_mul(4)))))) as u8); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = F32(c.m.r32((p_st0_7).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = ((rt::f2u(F32(c.m.r32((p_col1_5).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))) as u8); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2u(F32(c.m.r32((p_col1_5).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))) as u8); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2u(F32(c.m.r32((p_col1_5).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))) as u8); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2u(F32(c.m.r32((p_col1_5).wrapping_add((((3i32) as u32)).wrapping_mul(4)))))) as u8); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = F32(c.m.r32((p_st1_8).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = ((rt::f2u(F32(c.m.r32((p_col2_6).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))) as u8); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2u(F32(c.m.r32((p_col2_6).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))) as u8); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2u(F32(c.m.r32((p_col2_6).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))) as u8); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2u(F32(c.m.r32((p_col2_6).wrapping_add((((3i32) as u32)).wrapping_mul(4)))))) as u8); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = F32(c.m.r32((p_st2_9).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = p_tex0_12; let a_ = (l_p_14).wrapping_add(40); c.m.w64(a_, v_); };
    { let v_ = F32(c.m.r32((p_st0_7).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_st0_7).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_st1_8).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_st1_8).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_st2_9).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_st2_9).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = (((c.m.r32(p_xyz0_1) as i32)) as u64); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
    { let v_ = (((c.m.r32((p_xyz0_1).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_14).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
    { let v_ = (((c.m.r32((p_xyz0_1).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_14).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
    { let v_ = ((255i32) as u64); let a_ = ((((l_p_14).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
    { let v_ = (((c.m.r32(p_xyz1_2) as i32)) as u64); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
    { let v_ = (((c.m.r32((p_xyz1_2).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_14).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
    { let v_ = (((c.m.r32((p_xyz1_2).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_14).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
    { let v_ = ((255i32) as u64); let a_ = ((((l_p_14).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
    { let v_ = (((c.m.r32(p_xyz2_3) as i32)) as u64); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
    { let v_ = (((c.m.r32((p_xyz2_3).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_14).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
    { let v_ = (((c.m.r32((p_xyz2_3).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_14).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
    { let v_ = ((255i32) as u64); let a_ = ((((l_p_14).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
    F::eft_g__EftOt_Add(c, l_p_14, p_z_11, p_layer_10);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftUtil_DrawTri", &PROBE_N, &[], true); }
    r_
}

pub fn EftStorm_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStorm_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_pack_2: i32 = 0;
    let r_ = 'f: {
    l_pack_2 = F::BtlScene_GetStageData(c);
    if (((((c.m.r32(sym::gEftStorm)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftStorm); F::BtlPool_Free(c, a0_, a1_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gEftStorm; c.m.w32(a_, v_); };
    }
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((36592u32) as i32)) }; let a_ = sym::gEftStorm; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gEftStorm); F::memset(c, a0_, 0i32, 36592u32) }; };
    { let a0_ = c.m.r32(sym::gEftStorm); let a1_ = F::BtlScene_GetPackEntry(c, ((l_pack_2) as u32), 14i32); F::EftTexSet_Load32(c, a0_, a1_) };
    F::EftStorm_Reset(c, p_task_1);
    F::EftStorm_InitRain(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStorm_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftStorm_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStorm_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftStorm); F::BtlPool_Free(c, a0_, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStorm_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftStorm_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStorm_Update", &PROBE_N, &[], false); }
    let mut l_bolt_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_bolt_1 = (c.m.r32(sym::gEftStorm)).wrapping_add(528);
    { let a0_ = c.m.r32(sym::gEftStorm); F::EftTexSet_Keep32(c, a0_, 1i32, 0i32) };
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(((6640i32) as u32)); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
        break 'f;
    }
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32((l_bolt_1).wrapping_add(32)) as i32)) != (0i32)) as i32)) != 0) {
                    { let a_ = (l_bolt_1).wrapping_add(32); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((((c.m.r32((l_bolt_1).wrapping_add(16)) as i32)) != (0i32)) as i32)) != 0) {
                        { let a_ = (l_bolt_1).wrapping_add(16); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    } else {
                        F::EftStorm_SpawnBolt(c, l_bolt_1);
                    }
                }
            }
            { { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; }; { let o_ = l_bolt_1; let n_ = (o_).wrapping_add(48); l_bolt_1 = n_; }; };
        }
    }
    F::EftStorm_MoveRain(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStorm_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftStorm_DrawBolts(c: &mut Ctx, mut p_bolt_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStorm_DrawBolts", &PROBE_N, &[(p_bolt_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(256) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 256);
    let mut l_work_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_tag_4: u32 = 0;
    let mut l_vif_5: u32 = 0;
    let mut l_p_6: u32 = 0;
    let mut l_col_7: F32 = F32(0);
    let mut l_j_8: i32 = 0;
    let r_ = 'f: {
        l_work_2 = c.m.r32(sym::gEftStorm);
        F::Vec4_Set(c, (fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), F32(0xc0800000), F32(0xbfc00000), F32(0x00000000), F32(0x00000000));
        F::Vec4_Set(c, (fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), F32(0x40800000), F32(0x41000000), F32(0x00000000), F32(0x00000000));
        F::Vec4_Set(c, (fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), F32(0xc1000000), F32(0xc0400000), F32(0x00000000), F32(0x00000000));
        { let a2_ = F32(c.m.r32((sym::D_002FE9E4).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); F::Vec4_Set(c, (fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), F32(0x41000000), a2_, F32(0x00000000), F32(0x00000000)) };
        l_col_7 = rt::fmul(F::EftStage_GetTintScale(c), F32(0x43000000));
        F::Vec4_Scale(c, (fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), F32(0x43160000));
        F::Vec4_Scale(c, (fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), F32(0x43160000));
        F::Vec4_Scale(c, (fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), F32(0x43160000));
        F::Vec4_Scale(c, (fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), F32(0x43160000));
        F::Mtx_StoreIdentity(c, fp);
        if (((((c.m.r32(sym::gBtlCamView)) != (((0i32) as u32))) as i32)) != 0) {
            { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(64); F::Mtx_InverseRT(c, fp, a1_) };
        }
        l_tag_4 = ((536870912i32) as u32);
        F::Mtx_MulVec4(c, (fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), fp, (fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
        l_vif_5 = ((1342177280i32) as u32);
        F::Mtx_MulVec4(c, (fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), fp, (fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
        F::Mtx_MulVec4(c, (fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), fp, (fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
        F::Mtx_MulVec4(c, (fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), fp, (fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
        F::Vu0Cur_Push(c);
        { let r_ = ((7i32) as u32); let n_ = ((l_tag_4) | (r_)); l_tag_4 = n_; };
        { let r_ = ((7i32) as u32); let n_ = ((l_vif_5) | (r_)); l_vif_5 = n_; };
        { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
        {
            l_i_3 = 3i32;
            'l1: loop {
                if !(((((l_i_3) >= (0i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if ((((((((((c.m.r32((p_bolt_1).wrapping_add(16)) as i32)) == (0i32)) as i32)) != 0) || ((((((c.m.r32((p_bolt_1).wrapping_add(32)) as i32)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
                        break 'c2;
                    }
                    {
                        l_j_8 = 0i32;
                        'l3: loop {
                            if !(((((l_j_8) < (4i32)) as i32)) != 0) { break 'l3; }
                            'c4: {
                                { let v_ = F32(0x3f800000); let a_ = (p_bolt_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                F::Vec4_Add(c, (fp.wrapping_add(128)).wrapping_add((((l_j_8) as u32)).wrapping_mul(16)), (fp.wrapping_add(64)).wrapping_add((((l_j_8) as u32)).wrapping_mul(16)), p_bolt_1);
                                { let _ = F::Vu0Cur_ProjectPoint(c, (fp.wrapping_add(192)).wrapping_add((((l_j_8) as u32)).wrapping_mul(16)), (fp.wrapping_add(128)).wrapping_add((((l_j_8) as u32)).wrapping_mul(16))); };
                                if ((((!((F::eft_g__EftScr_IsValid(c, (fp.wrapping_add(192)).wrapping_add((((l_j_8) as u32)).wrapping_mul(16)))) != 0)) as i32)) != 0) {
                                    break 'l3;
                                }
                            }
                            { let o_ = l_j_8; let n_ = (o_).wrapping_add(1); l_j_8 = n_; };
                        }
                    }
                    if (((((l_j_8) != (4i32)) as i32)) != 0) {
                        break 'c2;
                    }
                    if ((((((c.m.r32((p_bolt_1).wrapping_add(36)) as i32)) != (0i32)) as i32)) != 0) {
                        'l5: loop {
                            'c6: {
                                l_p_6 = F::eft_g__EftOt_Alloc(c, ((128u32) as i32));
                                { let v_ = ((268436062347616i64) as u64); let a_ = (l_p_6).wrapping_add(24); c.m.w64(a_, v_); };
                                { let v_ = ((268435456i32) as u32); let a_ = (l_p_6).wrapping_add(8); c.m.w32(a_, v_); };
                                { let v_ = l_tag_4; let a_ = l_p_6; c.m.w32(a_, v_); };
                                { let v_ = ((0i32) as u32); let a_ = (l_p_6).wrapping_add(4); c.m.w32(a_, v_); };
                                { let v_ = 14123288431433908225u64; let a_ = (l_p_6).wrapping_add(16); c.m.w64(a_, v_); };
                                { let v_ = l_vif_5; let a_ = (l_p_6).wrapping_add(12); c.m.w32(a_, v_); };
                                { let v_ = ((84i32) as u64); let a_ = (l_p_6).wrapping_add(32); c.m.w64(a_, v_); };
                                { let v_ = ((rt::f2u(l_col_7)) as u8); let a_ = ((l_p_6).wrapping_add(48)).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
                                { let v_ = ((rt::f2u(l_col_7)) as u8); let a_ = ((l_p_6).wrapping_add(48)).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
                                { let v_ = ((rt::f2u(l_col_7)) as u8); let a_ = ((l_p_6).wrapping_add(48)).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
                                { let v_ = ((rt::div_i32(((c.m.r32((p_bolt_1).wrapping_add(16)) as i32)).wrapping_shl((7i32) as u32), (c.m.r32((p_bolt_1).wrapping_add(20)) as i32))) as u8); let a_ = ((l_p_6).wrapping_add(48)).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
                                { let v_ = F32(0x3f800000); let a_ = (l_p_6).wrapping_add(52); c.m.w32(a_, (v_).0); };
                                { let v_ = (((c.m.r32((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_6).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                                { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_6).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                                { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_6).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                                { let v_ = ((255i32) as u64); let a_ = ((((l_p_6).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                                { let v_ = (((c.m.r32((fp.wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_6).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                                { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_6).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                                { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_6).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                                { let v_ = ((255i32) as u64); let a_ = ((((l_p_6).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                                { let v_ = (((c.m.r32((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_6).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                                { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_6).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                                { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_6).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                                { let v_ = ((255i32) as u64); let a_ = ((((l_p_6).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                                { let v_ = (((c.m.r32((fp.wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_6).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                                { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_6).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                                { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_6).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                                { let v_ = ((255i32) as u64); let a_ = ((((l_p_6).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                                { let v_ = F32(0x00000000); let a_ = ((l_p_6).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x00000000); let a_ = (((l_p_6).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = ((l_p_6).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x00000000); let a_ = (((l_p_6).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x00000000); let a_ = ((l_p_6).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = (((l_p_6).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = ((l_p_6).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = (((l_p_6).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = { let a_ = (l_work_2).wrapping_add(((((c.m.r32((p_bolt_1).wrapping_add(28)) as i32)) as u32)).wrapping_mul(16)); c.m.r64(a_) }; let a_ = (l_p_6).wrapping_add(40); c.m.w64(a_, v_); };
                                { let a1_ = ((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)).wrapping_shr((8i32) as u32); F::eft_g__EftOt_Add(c, l_p_6, a1_, 1i32) };
                            }
                            if !((0i32) != 0) { break 'l5; }
                        }
                    } else {
                        'l7: loop {
                            'c8: {
                                l_p_6 = F::eft_g__EftOt_Alloc(c, ((128u32) as i32));
                                { let v_ = ((268436062347616i64) as u64); let a_ = (l_p_6).wrapping_add(24); c.m.w64(a_, v_); };
                                { let v_ = ((268435456i32) as u32); let a_ = (l_p_6).wrapping_add(8); c.m.w32(a_, v_); };
                                { let v_ = l_tag_4; let a_ = l_p_6; c.m.w32(a_, v_); };
                                { let v_ = ((0i32) as u32); let a_ = (l_p_6).wrapping_add(4); c.m.w32(a_, v_); };
                                { let v_ = 14123288431433908225u64; let a_ = (l_p_6).wrapping_add(16); c.m.w64(a_, v_); };
                                { let v_ = l_vif_5; let a_ = (l_p_6).wrapping_add(12); c.m.w32(a_, v_); };
                                { let v_ = ((84i32) as u64); let a_ = (l_p_6).wrapping_add(32); c.m.w64(a_, v_); };
                                { let v_ = ((rt::f2u(l_col_7)) as u8); let a_ = ((l_p_6).wrapping_add(48)).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
                                { let v_ = ((rt::f2u(l_col_7)) as u8); let a_ = ((l_p_6).wrapping_add(48)).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
                                { let v_ = ((rt::f2u(l_col_7)) as u8); let a_ = ((l_p_6).wrapping_add(48)).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
                                { let v_ = ((rt::div_i32(((c.m.r32((p_bolt_1).wrapping_add(16)) as i32)).wrapping_shl((7i32) as u32), (c.m.r32((p_bolt_1).wrapping_add(20)) as i32))) as u8); let a_ = ((l_p_6).wrapping_add(48)).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
                                { let v_ = F32(0x3f800000); let a_ = (l_p_6).wrapping_add(52); c.m.w32(a_, (v_).0); };
                                { let v_ = (((c.m.r32((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_6).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                                { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_6).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                                { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_6).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                                { let v_ = ((255i32) as u64); let a_ = ((((l_p_6).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                                { let v_ = (((c.m.r32((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_6).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                                { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_6).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                                { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_6).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                                { let v_ = ((255i32) as u64); let a_ = ((((l_p_6).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                                { let v_ = (((c.m.r32((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_6).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                                { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_6).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                                { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_6).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                                { let v_ = ((255i32) as u64); let a_ = ((((l_p_6).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                                { let v_ = (((c.m.r32((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_6).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                                { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_6).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                                { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_6).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                                { let v_ = ((255i32) as u64); let a_ = ((((l_p_6).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                                { let v_ = F32(0x00000000); let a_ = ((l_p_6).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x00000000); let a_ = (((l_p_6).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = ((l_p_6).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x00000000); let a_ = (((l_p_6).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x00000000); let a_ = ((l_p_6).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = (((l_p_6).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = ((l_p_6).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = (((l_p_6).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = { let a_ = (l_work_2).wrapping_add(((((c.m.r32((p_bolt_1).wrapping_add(24)) as i32)) as u32)).wrapping_mul(16)); c.m.r64(a_) }; let a_ = (l_p_6).wrapping_add(40); c.m.w64(a_, v_); };
                                { let a1_ = ((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)).wrapping_shr((8i32) as u32); F::eft_g__EftOt_Add(c, l_p_6, a1_, 1i32) };
                            }
                            if !((0i32) != 0) { break 'l7; }
                        }
                        'l9: loop {
                            'c10: {
                                l_p_6 = F::eft_g__EftOt_Alloc(c, ((128u32) as i32));
                                { let v_ = ((268436062347616i64) as u64); let a_ = (l_p_6).wrapping_add(24); c.m.w64(a_, v_); };
                                { let v_ = ((268435456i32) as u32); let a_ = (l_p_6).wrapping_add(8); c.m.w32(a_, v_); };
                                { let v_ = l_tag_4; let a_ = l_p_6; c.m.w32(a_, v_); };
                                { let v_ = ((0i32) as u32); let a_ = (l_p_6).wrapping_add(4); c.m.w32(a_, v_); };
                                { let v_ = 14123288431433908225u64; let a_ = (l_p_6).wrapping_add(16); c.m.w64(a_, v_); };
                                { let v_ = l_vif_5; let a_ = (l_p_6).wrapping_add(12); c.m.w32(a_, v_); };
                                { let v_ = ((84i32) as u64); let a_ = (l_p_6).wrapping_add(32); c.m.w64(a_, v_); };
                                { let v_ = ((rt::f2u(l_col_7)) as u8); let a_ = ((l_p_6).wrapping_add(48)).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
                                { let v_ = ((rt::f2u(l_col_7)) as u8); let a_ = ((l_p_6).wrapping_add(48)).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
                                { let v_ = ((rt::f2u(l_col_7)) as u8); let a_ = ((l_p_6).wrapping_add(48)).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
                                { let v_ = ((rt::div_i32(((c.m.r32((p_bolt_1).wrapping_add(16)) as i32)).wrapping_shl((7i32) as u32), (c.m.r32((p_bolt_1).wrapping_add(20)) as i32))) as u8); let a_ = ((l_p_6).wrapping_add(48)).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
                                { let v_ = F32(0x3f800000); let a_ = (l_p_6).wrapping_add(52); c.m.w32(a_, (v_).0); };
                                { let v_ = (((c.m.r32((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_6).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                                { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_6).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                                { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_6).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                                { let v_ = ((255i32) as u64); let a_ = ((((l_p_6).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                                { let v_ = (((c.m.r32((fp.wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_6).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                                { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_6).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                                { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_6).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                                { let v_ = ((255i32) as u64); let a_ = ((((l_p_6).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                                { let v_ = (((c.m.r32((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_6).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                                { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_6).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                                { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_6).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                                { let v_ = ((255i32) as u64); let a_ = ((((l_p_6).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                                { let v_ = (((c.m.r32((fp.wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_6).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                                { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_6).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                                { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_6).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                                { let v_ = ((255i32) as u64); let a_ = ((((l_p_6).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                                { let v_ = F32(0x00000000); let a_ = ((l_p_6).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x00000000); let a_ = (((l_p_6).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = ((l_p_6).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x00000000); let a_ = (((l_p_6).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x00000000); let a_ = ((l_p_6).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = (((l_p_6).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = ((l_p_6).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = (((l_p_6).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = { let a_ = (l_work_2).wrapping_add(((((c.m.r32((p_bolt_1).wrapping_add(28)) as i32)) as u32)).wrapping_mul(16)); c.m.r64(a_) }; let a_ = (l_p_6).wrapping_add(40); c.m.w64(a_, v_); };
                                { let a1_ = ((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)).wrapping_shr((8i32) as u32); F::eft_g__EftOt_Add(c, l_p_6, a1_, 1i32) };
                            }
                            if !((0i32) != 0) { break 'l9; }
                        }
                    }
                }
                { { let o_ = l_i_3; let n_ = (o_).wrapping_sub(1); l_i_3 = n_; }; { let o_ = p_bolt_1; let n_ = (o_).wrapping_add(48); p_bolt_1 = n_; }; };
            }
        }
        F::Vu0Cur_Pop(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftStorm_DrawBolts", &PROBE_N, &[], true); }
    r_
}

pub fn EftStorm_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStorm_Draw", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32(sym::gEftStorm)).wrapping_add(528); F::EftStorm_DrawBolts(c, a0_) };
    F::EftStorm_DrawRain(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStorm_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftStorm_PostUpdate(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStorm_PostUpdate", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStorm_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftStorm_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStorm_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_bolt_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    l_bolt_2 = (c.m.r32(sym::gEftStorm)).wrapping_add(528);
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                F::EftStorm_SpawnBolt(c, l_bolt_2);
            }
            { { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; }; { let o_ = l_bolt_2; let n_ = (o_).wrapping_add(48); l_bolt_2 = n_; }; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStorm_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftStorm_SpawnBolt(c: &mut Ctx, mut p_bolt_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStorm_SpawnBolt", &PROBE_N, &[(p_bolt_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_dist_2: F32 = F32(0);
    let mut l_other_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_r_5: i32 = 0;
    let mut l_r2_6: i32 = 0;
    let r_ = 'f: {
        l_dist_2 = rt::fadd(rt::i2f((((F::Port_Rand(c)) & (1023i32))).wrapping_sub(511i32)), F32(0x459c4000));
        { let v_ = rt::i2f((((F::Port_Rand(c)) & (511i32))).wrapping_sub(255i32)); let a_ = p_bolt_1; c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_bolt_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f((((F::Port_Rand(c)) & (255i32))).wrapping_sub(31i32)); let a_ = (p_bolt_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (p_bolt_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
        if (((((((rt::feq(F32(c.m.r32(p_bolt_1)), F32(0x00000000)) as i32)) != 0) && (((rt::feq(F32(c.m.r32((p_bolt_1).wrapping_add(8))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
            { let v_ = F32(0x3f800000); let a_ = p_bolt_1; c.m.w32(a_, (v_).0); };
        }
        F::Vec3_Normalize(c, p_bolt_1, p_bolt_1);
        F::Vec4_Scale(c, p_bolt_1, p_bolt_1, l_dist_2);
        { let v_ = { let v_ = ((((((((F::Port_Rand(c)) & (31i32))).wrapping_add(32i32)) as u32)).wrapping_shr((1i32) as u32)) as i32); let a_ = (p_bolt_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); v_ }; let a_ = (p_bolt_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((rt::rem_u32(((F::Port_Rand(c)) as u32), ((3i32) as u32))) as i32); let a_ = (p_bolt_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
        l_r_5 = F::Port_Rand(c);
        l_r2_6 = ((F::Port_Rand(c)) & (1i32));
        { let v_ = F32(0xc42f0000); let a_ = (p_bolt_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = ((((l_r_5) & (1i32))).wrapping_add(l_r2_6)).wrapping_add(3i32); let a_ = (p_bolt_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
        { let _ = F::Port_Rand(c); };
        { let v_ = 1i32; let a_ = (p_bolt_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
        if (((rt::flt(l_dist_2, F32(0x459c4000)) as i32)) != 0) {
            l_r2_6 = 1i32;
            { let v_ = l_r2_6; let a_ = (p_bolt_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        } else {
            l_r2_6 = ((F::Port_Rand(c)) & (1i32));
            { let v_ = l_r2_6; let a_ = (p_bolt_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        }
        {
            { l_other_3 = (c.m.r32(sym::gEftStorm)).wrapping_add(528); l_i_4 = 0i32; };
            'l1: loop {
                if !(((((l_i_4) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                    if (((((l_other_3) == (p_bolt_1)) as i32)) != 0) {
                        break 'c2;
                    }
                    if ((((((c.m.r32((l_other_3).wrapping_add(16)) as i32)) == (0i32)) as i32)) != 0) {
                        break 'c2;
                    }
                    F::Vec4_Sub(c, fp, l_other_3, p_bolt_1);
                    if (((rt::flt(F::Vec3_Dot(c, fp, fp), F32(0x48ef4200)) as i32)) != 0) {
                        { let v_ = 0i32; let a_ = (p_bolt_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                        break 'l1;
                    }
                }
                { let o_ = l_other_3; let n_ = (o_).wrapping_add(48); l_other_3 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftStorm_SpawnBolt", &PROBE_N, &[], true); }
    r_
}

pub fn EftStorm_InitRain(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStorm_InitRain", &PROBE_N, &[], false); }
    let mut l_drop_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_drop_1 = (c.m.r32(sym::gEftStorm)).wrapping_add(34512);
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (64i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = rt::fadd(rt::i2f((((F::Port_Rand(c)) & (1023i32))).wrapping_sub(511i32)), F32(0x00000000)); let a_ = l_drop_1; c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(rt::i2f((((F::Port_Rand(c)) & (1023i32))).wrapping_sub(511i32)), F32(0x00000000)); let a_ = (l_drop_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(rt::i2f((((F::Port_Rand(c)) & (1023i32))).wrapping_sub(511i32)), F32(0x00000000)); let a_ = (l_drop_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = (l_drop_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = (l_drop_1).wrapping_add(16); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x419ccccc); let a_ = ((l_drop_1).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = ((l_drop_1).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = ((l_drop_1).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            }
            { { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; }; { let o_ = l_drop_1; let n_ = (o_).wrapping_add(32); l_drop_1 = n_; }; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStorm_InitRain", &PROBE_N, &[], true); }
    r_
}

pub fn EftStorm_MoveRain(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStorm_MoveRain", &PROBE_N, &[], false); }
    let mut l_drop_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_drop_1 = (c.m.r32(sym::gEftStorm)).wrapping_add(34512);
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (64i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                F::Vec4_Add(c, l_drop_1, l_drop_1, (l_drop_1).wrapping_add(16));
            }
            { { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; }; { let o_ = l_drop_1; let n_ = (o_).wrapping_add(32); l_drop_1 = n_; }; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStorm_MoveRain", &PROBE_N, &[], true); }
    r_
}

pub fn EftStorm_DrawRainLines(c: &mut Ctx, mut p_drop_1: u32, mut p_count_2: i32, mut p_streak_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStorm_DrawRainLines", &PROBE_N, &[(p_drop_1) as u32, (p_count_2) as u32, (p_streak_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_p_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_cur_6: u32 = 0;
    let r_ = 'f: {
        {
            l_i_5 = 0i32;
            'l1: loop {
                if !(((((l_i_5) < (p_count_2)) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::Vec4_Copy(c, (fp.wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), p_drop_1);
                    F::Vec4_Add(c, (fp.wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), p_drop_1, p_streak_3);
                    if (((((((((((((F::Vu0Cur_ProjectPoints(c, fp, fp.wrapping_add(32), 2i32)) != (0i32)) as i32)) != 0) && ((F::eft_g__EftScr_IsValid(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)))) != 0)) as i32)) != 0) && ((F::eft_g__EftScr_IsValid(c, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)))) != 0)) as i32)) != 0) {
                        {
                            l_cur_6 = c.m.r32(sym::gOtCur);
                            l_p_4 = l_cur_6;
                            { let r_ = 16i32; let n_ = (l_cur_6).wrapping_add((((r_) as u32)).wrapping_mul(4)); l_cur_6 = n_; };
                            { let v_ = l_cur_6; let a_ = sym::gOtCur; c.m.w32(a_, v_); };
                        }
                        { let v_ = ((65i32) as u64); let a_ = (l_p_4).wrapping_add(32); c.m.w64(a_, v_); };
                        { let v_ = ((536870915i32) as u32); let a_ = l_p_4; c.m.w32(a_, v_); };
                        { let v_ = ((268435456i32) as u32); let a_ = (l_p_4).wrapping_add(8); c.m.w32(a_, v_); };
                        { let v_ = ((1342177283i32) as u32); let a_ = (l_p_4).wrapping_add(12); c.m.w32(a_, v_); };
                        { let v_ = ((4899916394579132417i64) as u64); let a_ = (l_p_4).wrapping_add(16); c.m.w64(a_, v_); };
                        { let v_ = ((17424i32) as u64); let a_ = (l_p_4).wrapping_add(24); c.m.w64(a_, v_); };
                        { let v_ = ((32i32) as u8); let a_ = ((l_p_4).wrapping_add(40)).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
                        { let v_ = F32(0x3f800000); let a_ = (l_p_4).wrapping_add(44); c.m.w32(a_, (v_).0); };
                        { let v_ = ((0i32) as u32); let a_ = (l_p_4).wrapping_add(4); c.m.w32(a_, v_); };
                        { let v_ = ((96i32) as u8); let a_ = ((l_p_4).wrapping_add(40)).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
                        { let v_ = ((96i32) as u8); let a_ = ((l_p_4).wrapping_add(40)).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
                        { let v_ = ((96i32) as u8); let a_ = ((l_p_4).wrapping_add(40)).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
                        { let v_ = (((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = ((l_p_4).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                        { let v_ = (((c.m.r32(((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = (((l_p_4).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                        { let v_ = (((c.m.r32(((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = (((l_p_4).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                        { let v_ = ((255i32) as u64); let a_ = (((l_p_4).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                        { let v_ = (((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = ((l_p_4).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                        { let v_ = (((c.m.r32(((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = (((l_p_4).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                        { let v_ = (((c.m.r32(((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = (((l_p_4).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                        { let v_ = ((255i32) as u64); let a_ = (((l_p_4).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                        F::eft_g__EftOt_Add(c, l_p_4, 0i32, 0i32);
                    }
                }
                { { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; }; { let o_ = p_drop_1; let n_ = (o_).wrapping_add(32); p_drop_1 = n_; }; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftStorm_DrawRainLines", &PROBE_N, &[], true); }
    r_
}

pub fn EftStorm_WrapRain(c: &mut Ctx, mut p_center_1: u32, mut p_drop_2: u32, mut p_count_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStorm_WrapRain", &PROBE_N, &[(p_center_1) as u32, (p_drop_2) as u32, (p_count_3) as u32], false); }
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (p_count_3)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((rt::flt(rt::fadd(F32(c.m.r32(p_center_1)), F32(0x42c80000)), F32(c.m.r32(p_drop_2))) as i32)) != 0) {
                    { let r_ = F32(0x43480000); let a_ = p_drop_2; let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                }
                if (((rt::flt(F32(c.m.r32(p_drop_2)), rt::fsub(F32(c.m.r32(p_center_1)), F32(0x42c80000))) as i32)) != 0) {
                    { let r_ = F32(0x43480000); let a_ = p_drop_2; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                }
                if (((rt::flt(rt::fadd(F32(c.m.r32((p_center_1).wrapping_add(4))), F32(0x42c80000)), F32(c.m.r32((p_drop_2).wrapping_add(4)))) as i32)) != 0) {
                    { let v_ = rt::fadd(F32(c.m.r32(p_center_1)), rt::fmul(rt::i2f((((F::Port_Rand(c)) & (1023i32))).wrapping_sub(511i32)), F32(0x3e800000))); let a_ = p_drop_2; c.m.w32(a_, (v_).0); };
                    { let r_ = F32(0x43480000); let a_ = (p_drop_2).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32((p_center_1).wrapping_add(8))), rt::fmul(rt::i2f((((F::Port_Rand(c)) & (1023i32))).wrapping_sub(511i32)), F32(0x3e800000))); let a_ = (p_drop_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
                }
                if (((rt::flt(F32(c.m.r32((p_drop_2).wrapping_add(4))), rt::fsub(F32(c.m.r32((p_center_1).wrapping_add(4))), F32(0x42c80000))) as i32)) != 0) {
                    { let v_ = rt::fadd(F32(c.m.r32(p_center_1)), rt::fmul(rt::i2f((((F::Port_Rand(c)) & (1023i32))).wrapping_sub(511i32)), F32(0x3e800000))); let a_ = p_drop_2; c.m.w32(a_, (v_).0); };
                    { let r_ = F32(0x43480000); let a_ = (p_drop_2).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32((p_center_1).wrapping_add(8))), rt::fmul(rt::i2f((((F::Port_Rand(c)) & (1023i32))).wrapping_sub(511i32)), F32(0x3e800000))); let a_ = (p_drop_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
                }
                if (((rt::flt(rt::fadd(F32(c.m.r32((p_center_1).wrapping_add(8))), F32(0x42c80000)), F32(c.m.r32((p_drop_2).wrapping_add(8)))) as i32)) != 0) {
                    { let r_ = F32(0x43480000); let a_ = (p_drop_2).wrapping_add(8); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                }
                if (((rt::flt(F32(c.m.r32((p_drop_2).wrapping_add(8))), rt::fsub(F32(c.m.r32((p_center_1).wrapping_add(8))), F32(0x42c80000))) as i32)) != 0) {
                    { let r_ = F32(0x43480000); let a_ = (p_drop_2).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                }
            }
            { { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; }; { let o_ = p_drop_2; let n_ = (o_).wrapping_add(32); p_drop_2 = n_; }; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStorm_WrapRain", &PROBE_N, &[], true); }
    r_
}

pub fn EftStorm_DrawRain(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStorm_DrawRain", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let mut l_view_1: i32 = 0;
    let r_ = 'f: {
        l_view_1 = { let a_ = ({ let a_ = (c.m.r32(sym::gBtlCam)).wrapping_add(3136); c.m.r32(a_) }).wrapping_add(644); (c.m.r32(a_) as i32) };
        F::Vec4_Set(c, fp, F32(0x00000000), F32(0x00000000), F32(0x43160000), F32(0x3f800000));
        F::Vec4_Set(c, fp.wrapping_add(16), F32(0x00000000), F32(0x42c80000), F32(0x00000000), F32(0x00000000));
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(64); F::Mtx_InverseRT(c, fp.wrapping_add(48), a1_) };
        F::Mtx_MulVec4(c, fp, fp.wrapping_add(48), fp);
        { let a1_ = ((c.m.r32(sym::gEftStorm)).wrapping_add(36560)).wrapping_add((((l_view_1) as u32)).wrapping_mul(16)); F::Vec4_Sub(c, fp.wrapping_add(32), a1_, (fp.wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) };
        F::Vec4_Add(c, fp.wrapping_add(16), fp.wrapping_add(16), fp.wrapping_add(32));
        F::Vec3_Normalize(c, fp.wrapping_add(16), fp.wrapping_add(16));
        F::Vec4_Scale(c, fp.wrapping_add(16), fp.wrapping_add(16), F32(0x41a00000));
        { let a0_ = ((c.m.r32(sym::gEftStorm)).wrapping_add(36560)).wrapping_add((((l_view_1) as u32)).wrapping_mul(16)); F::Vec4_Copy(c, a0_, (fp.wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) };
        F::Vu0Cur_Push(c);
        { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
        if ((((!((F::Battle_IsSplitScreen(c)) != 0)) as i32)) != 0) {
            { let a1_ = (c.m.r32(sym::gEftStorm)).wrapping_add(34512); F::EftStorm_WrapRain(c, fp, a1_, 64i32) };
            { let a0_ = (c.m.r32(sym::gEftStorm)).wrapping_add(34512); F::EftStorm_DrawRainLines(c, a0_, 64i32, fp.wrapping_add(16)) };
        } else {
            if (((((l_view_1) != (0i32)) as i32)) != 0) {
                { let a1_ = (c.m.r32(sym::gEftStorm)).wrapping_add(34512); F::EftStorm_WrapRain(c, fp, a1_, rt::div_i32(64i32, 2i32)) };
                { let a0_ = (c.m.r32(sym::gEftStorm)).wrapping_add(34512); F::EftStorm_DrawRainLines(c, a0_, rt::div_i32(64i32, 2i32), fp.wrapping_add(16)) };
            } else {
                { let a1_ = ((c.m.r32(sym::gEftStorm)).wrapping_add(34512)).wrapping_add((((rt::div_i32(64i32, 2i32)) as u32)).wrapping_mul(32)); F::EftStorm_WrapRain(c, fp, a1_, rt::div_i32(64i32, 2i32)) };
                { let a0_ = ((c.m.r32(sym::gEftStorm)).wrapping_add(34512)).wrapping_add((((rt::div_i32(64i32, 2i32)) as u32)).wrapping_mul(32)); F::EftStorm_DrawRainLines(c, a0_, rt::div_i32(64i32, 2i32), fp.wrapping_add(16)) };
            }
        }
        F::Vu0Cur_Pop(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftStorm_DrawRain", &PROBE_N, &[], true); }
    r_
}

pub fn EftSmoke_Create(c: &mut Ctx, mut p_arg_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSmoke_Create", &PROBE_N, &[(p_arg_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftSmokeMgr)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    break 'f { let a0_ = { let a_ = c.m.r32(sym::gEftSmokeMgr); c.m.r32(a_) }; F::BtlTaskList_AddTail(c, a0_, sym::gEftSmokeClass, p_arg_1) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSmoke_Create", &PROBE_N, &[], true); }
    r_
}

pub fn EftSmoke_Stop(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSmoke_Stop", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    if (((((((((((((p_task_1) != (((0i32) as u32))) as i32)) != 0) && ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftSmoke_Update)) as i32)) != 0)) as i32)) != 0) && ((((!((((((c.m.r8(p_task_1)) as i32)) & (1i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
        { let r_ = 1i32; let a_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(96); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSmoke_Stop", &PROBE_N, &[], true); }
    r_
}

pub fn EftSmoke_GetPos(c: &mut Ctx, mut p_task_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSmoke_GetPos", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftSmoke_Update)) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    if ((((((c.m.r8(p_task_1)) as i32)) & (1i32))) != 0) {
        break 'f ((0i32) as u32);
    }
    break 'f (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(16);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSmoke_GetPos", &PROBE_N, &[], true); }
    r_
}

pub fn EftSmoke_SetPos(c: &mut Ctx, mut p_task_1: u32, mut p_pos_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSmoke_SetPos", &PROBE_N, &[(p_task_1) as u32, (p_pos_2) as u32], false); }
    let r_ = 'f: {
    if (((((((((((((p_task_1) != (((0i32) as u32))) as i32)) != 0) && ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftSmoke_Update)) as i32)) != 0)) as i32)) != 0) && ((((!((((((c.m.r8(p_task_1)) as i32)) & (1i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
        { let a0_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(16); F::Vec4_Copy(c, a0_, p_pos_2) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSmoke_SetPos", &PROBE_N, &[], true); }
    r_
}

pub fn EftSmokeMgr_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSmokeMgr_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_pack_2: i32 = 0;
    let mut l_count_3: i32 = 0;
    let mut l_src_4: u32 = 0;
    let mut l_list_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let r_ = 'f: {
        l_pack_2 = F::BtlScene_GetStageData(c);
        l_count_3 = ((((F::BtlStage_GetFxResB(c)) as u64)) as i32);
        { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((144u32) as i32)) }; let a_ = sym::gEftSmokeMgr; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gEftSmokeMgr); F::memset(c, a0_, 0i32, 144u32) }; };
        { let a0_ = (c.m.r32(sym::gEftSmokeMgr)).wrapping_add(8); let a1_ = F::BtlScene_GetPackEntry(c, ((l_pack_2) as u32), 18i32); F::EftTexSet_Load8(c, a0_, a1_) };
        { let v_ = { let a1_ = (l_count_3).wrapping_add(((((F::BtlStage_GetFxResA(c)) as u64)) as i32)); F::BtlTask_CreateChildList(c, p_task_1, ((((a1_) as i64 as u64)) as u32), ((((((2032u32) as i32)) as i64 as u64)) as u32)) }; let a_ = c.m.r32(sym::gEftSmokeMgr); c.m.w32(a_, v_); };
        l_list_5 = F::BtlStage_GetFxResB2(c);
        if (((((l_list_5) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        {
            l_i_6 = 0i32;
            'l1: loop {
                if !(((((l_i_6) < (l_count_3)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_src_4 = (l_list_5).wrapping_add((((l_i_6) as u32)).wrapping_mul(48));
                    { let _ = F::memset(c, fp, 0i32, 80u32); };
                    { let v_ = F32(c.m.r32((l_src_4).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = fp; c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((l_src_4).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((l_src_4).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::i2f((c.m.r8(((l_src_4).wrapping_add(28)).wrapping_add(((0i32) as u32)))) as i32); let a_ = (fp).wrapping_add(16); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::i2f((c.m.r8(((l_src_4).wrapping_add(28)).wrapping_add(((1i32) as u32)))) as i32); let a_ = ((fp).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::i2f((c.m.r8(((l_src_4).wrapping_add(28)).wrapping_add(((2i32) as u32)))) as i32); let a_ = ((fp).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::i2f((c.m.r8(((l_src_4).wrapping_add(28)).wrapping_add(((3i32) as u32)))) as i32); let a_ = ((fp).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::i2f((c.m.r8(((l_src_4).wrapping_add(32)).wrapping_add(((0i32) as u32)))) as i32); let a_ = (fp).wrapping_add(32); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::i2f((c.m.r8(((l_src_4).wrapping_add(32)).wrapping_add(((1i32) as u32)))) as i32); let a_ = ((fp).wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::i2f((c.m.r8(((l_src_4).wrapping_add(32)).wrapping_add(((2i32) as u32)))) as i32); let a_ = ((fp).wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::i2f((c.m.r8(((l_src_4).wrapping_add(32)).wrapping_add(((3i32) as u32)))) as i32); let a_ = ((fp).wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::i2f((c.m.r8(((l_src_4).wrapping_add(28)).wrapping_add(((3i32) as u32)))) as i32); let a_ = (fp).wrapping_add(52); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((l_src_4).wrapping_add(16))); let a_ = (fp).wrapping_add(48); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((l_src_4).wrapping_add(12))); let a_ = (fp).wrapping_add(60); c.m.w32(a_, (v_).0); };
                    { let v_ = ((c.m.r16((l_src_4).wrapping_add(24))) as i32); let a_ = (fp).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 30i32; let a_ = (fp).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r16((l_src_4).wrapping_add(26))) as i32); let a_ = (fp).wrapping_add(72); c.m.w32(a_, (v_) as u32); };
                    { let v_ = F32(c.m.r32((l_src_4).wrapping_add(20))); let a_ = (fp).wrapping_add(56); c.m.w32(a_, (v_).0); };
                    { let _ = F::EftSmoke_Create(c, fp); };
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSmokeMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftSmokeMgr_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSmokeMgr_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftSmokeMgr); F::BtlPool_Free(c, a0_, a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftSmokeMgr; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSmokeMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftSmokeMgr_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSmokeMgr_Update", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = ({ let a_ = c.m.r32(sym::gEftSmokeMgr); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a0_ = (c.m.r32(sym::gEftSmokeMgr)).wrapping_add(40); F::EftVram_AddTex(c, a0_, 1i32, 0i32) }; let a_ = (c.m.r32(sym::gEftSmokeMgr)).wrapping_add(40); c.m.w64(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSmokeMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftSmoke_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSmoke_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_work_3: u32 = 0;
    let mut l_n_4: i32 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
    l_work_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let _ = F::memset(c, l_work_3, 0i32, 2032u32); };
    { let s_ = p_arg_2; let d_ = (l_work_3).wrapping_add(16); rt::memcpy(c, d_, s_, 80); };
    { let v_ = (c.m.r32(sym::gEftSmokeMgr)).wrapping_add(8); let a_ = l_work_3; c.m.w32(a_, v_); };
    l_n_4 = (rt::rem_i32(F::Port_Rand(c), 30i32)).wrapping_add(60i32);
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((l_i_5) < (l_n_4)) as i32)) != 0) { break 'l1; }
            'c2: {
                F::EftSmoke_Update(c, p_task_1);
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSmoke_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftSmoke_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSmoke_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSmoke_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftSmoke_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSmoke_Reset", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSmoke_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftSmoke_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSmoke_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_work_2: u32 = 0;
    let mut l_part_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_n_5: i32 = 0;
    let mut l_x_6: F32 = F32(0);
    let mut l_z_7: F32 = F32(0);
    let mut l_flags_8: u32 = 0;
    let r_ = 'f: {
    l_work_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((F::BtlScene_IsTimeStopped(c)) != 0) {
        break 'f;
    }
    l_flags_8 = (((c.m.r32((l_work_2).wrapping_add(96)) as i32)) as u32);
    if ((((!((((l_flags_8) & (((1i32) as u32)))) != 0)) as i32)) != 0) {
        if (((((rt::rem_i32(F::Port_Rand(c), (c.m.r32(((l_work_2).wrapping_add(16)).wrapping_add(72)) as i32))) == (0i32)) as i32)) != 0) {
            l_part_3 = (l_work_2).wrapping_add(112);
            {
                l_i_4 = 0i32;
                'l1: loop {
                    if !(((((l_i_4) < (40i32)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        if (((((((c.m.r16((l_part_3).wrapping_add(32))) as i32)) == (0i32)) as i32)) != 0) {
                            F::Vec4_Copy(c, l_part_3, (l_work_2).wrapping_add(16));
                            l_x_6 = F::Rand_FloatRange(c, F32(0xc2480000), F32(0x42480000));
                            l_z_7 = F::Rand_FloatRange(c, F32(0xc2480000), F32(0x42480000));
                            F::Vec4_Set(c, (l_part_3).wrapping_add(16), l_x_6, F32(0xc2c80000), l_z_7, F32(0x3f800000));
                            F::Vec3_Normalize(c, (l_part_3).wrapping_add(16), (l_part_3).wrapping_add(16));
                            { let a2_ = F32(c.m.r32(((l_work_2).wrapping_add(16)).wrapping_add(56))); F::Vec4_Scale(c, (l_part_3).wrapping_add(16), (l_part_3).wrapping_add(16), a2_) };
                            { let v_ = ((((c.m.r32(((l_work_2).wrapping_add(16)).wrapping_add(64)) as i32)).wrapping_add(rt::rem_i32(F::Port_Rand(c), (c.m.r32(((l_work_2).wrapping_add(16)).wrapping_add(68)) as i32)))) as u16); let a_ = (l_part_3).wrapping_add(32); c.m.w16(a_, v_); };
                            break 'l1;
                        }
                    }
                    { { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; }; { let o_ = l_part_3; let n_ = (o_).wrapping_add(48); l_part_3 = n_; }; };
                }
            }
        }
    }
    l_part_3 = (l_work_2).wrapping_add(112);
    if (((((c.m.r32((l_work_2).wrapping_add(96)) as i32)) & (1i32))) != 0) {
        l_n_5 = 0i32;
        {
            l_i_4 = 0i32;
            'l3: loop {
                if !(((((l_i_4) < (40i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    if (((((((c.m.r16((l_part_3).wrapping_add(32))) as i32)) != (0i32)) as i32)) != 0) {
                        { let o_ = l_n_5; let n_ = (o_).wrapping_add(1); l_n_5 = n_; };
                    }
                }
                { { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; }; { let o_ = l_part_3; let n_ = (o_).wrapping_add(48); l_part_3 = n_; }; };
            }
        }
        if (((((l_n_5) == (0i32)) as i32)) != 0) {
            F::BtlTask_SetDead(c, p_task_1);
            break 'f;
        }
    }
    l_part_3 = (l_work_2).wrapping_add(112);
    {
        l_i_4 = 0i32;
        'l5: loop {
            if !(((((l_i_4) < (40i32)) as i32)) != 0) { break 'l5; }
            'c6: {
                if (((((((c.m.r16((l_part_3).wrapping_add(32))) as i32)) != (0i32)) as i32)) != 0) {
                    F::Vec3_Add(c, l_part_3, l_part_3, (l_part_3).wrapping_add(16));
                    { let r_ = F32(c.m.r32(((l_work_2).wrapping_add(16)).wrapping_add(60))); let a_ = (l_part_3).wrapping_add(16); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let r_ = F32(c.m.r32(((l_work_2).wrapping_add(16)).wrapping_add(60))); let a_ = ((l_part_3).wrapping_add(16)).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let a_ = (l_part_3).wrapping_add(32); let o_ = c.m.r16(a_); let n_ = (o_).wrapping_sub(1); c.m.w16(a_, n_); };
                }
            }
            { { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; }; { let o_ = l_part_3; let n_ = (o_).wrapping_add(48); l_part_3 = n_; }; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSmoke_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftSmoke_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSmoke_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_tex_2: u32 = 0;
    let mut l_work_3: u32 = 0;
    let mut l_part_4: u32 = 0;
    let mut l_bright_5: F32 = F32(0);
    let mut l_alpha_6: F32 = F32(0);
    let mut l_one_7: F32 = F32(0);
    let mut l_zero_8: F32 = F32(0);
    let mut l_hi_9: F32 = F32(0);
    let mut l_lo_10: F32 = F32(0);
    let mut l_life_11: i32 = 0;
    let mut l_fade_12: i32 = 0;
    let mut l_i_13: i32 = 0;
    let r_ = 'f: {
        l_work_3 = c.m.r32((p_task_1).wrapping_add(56));
        l_fade_12 = 50i32;
        l_tex_2 = (c.m.r32(l_work_3)).wrapping_add(((32i32) as u32));
        if (((({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(8); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
            break 'f;
        }
        if ((((!((F::EftStage_IsDrawOn(c)) != 0)) as i32)) != 0) {
            break 'f;
        }
        l_part_4 = (l_work_3).wrapping_add(112);
        l_bright_5 = F::EftStage_GetTintScale(c);
        l_one_7 = F32(0x3f800000);
        l_zero_8 = F32(0x00000000);
        F::Vu0Cur_Push(c);
        { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
        F::BtlStage_GetLightVecB(c, fp.wrapping_add(16));
        F::Vec3_Normalize(c, fp.wrapping_add(16), fp.wrapping_add(16));
        { let a1_ = F32(c.m.r32((l_work_3).wrapping_add(16))); let a2_ = rt::fsub(F32(c.m.r32(((l_work_3).wrapping_add(16)).wrapping_add(4))), F32(0x42480000)); let a3_ = F32(c.m.r32(((l_work_3).wrapping_add(16)).wrapping_add(8))); F::Vec4_Set(c, fp, a1_, a2_, a3_, l_one_7) };
        { let a1_ = F32(c.m.r32(((l_work_3).wrapping_add(16)).wrapping_add(16))); let a2_ = F32(c.m.r32((((l_work_3).wrapping_add(16)).wrapping_add(16)).wrapping_add(4))); let a3_ = F32(c.m.r32((((l_work_3).wrapping_add(16)).wrapping_add(16)).wrapping_add(8))); F::Vec4_Set(c, fp.wrapping_add(64), a1_, a2_, a3_, l_zero_8) };
        { let a1_ = F32(c.m.r32(((l_work_3).wrapping_add(16)).wrapping_add(32))); let a2_ = F32(c.m.r32((((l_work_3).wrapping_add(16)).wrapping_add(32)).wrapping_add(4))); let a3_ = F32(c.m.r32((((l_work_3).wrapping_add(16)).wrapping_add(32)).wrapping_add(8))); F::Vec4_Set(c, fp.wrapping_add(48), a1_, a2_, a3_, l_zero_8) };
        l_hi_9 = l_one_7;
        l_lo_10 = l_zero_8;
        {
            l_i_13 = 0i32;
            'l1: loop {
                if !(((((l_i_13) < (40i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((((c.m.r16((l_part_4).wrapping_add(32))) as i32)) != (0i32)) as i32)) != 0) {
                        l_life_11 = ((c.m.r16((l_part_4).wrapping_add(32))) as i32);
                        l_alpha_6 = F32(c.m.r32(((l_work_3).wrapping_add(16)).wrapping_add(52)));
                        if (((((l_life_11) < (l_fade_12)) as i32)) != 0) {
                            l_alpha_6 = rt::fdiv(rt::fmul(l_alpha_6, rt::i2f(l_life_11)), rt::i2f(l_fade_12));
                        }
                        F::Vec3_Sub(c, fp.wrapping_add(80), l_part_4, fp);
                        { let r_ = F32(0x3f4ccccc); let a_ = (fp.wrapping_add(80)).wrapping_add(4); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        F::Vec3_Normalize(c, fp.wrapping_add(80), fp.wrapping_add(80));
                        { let a2_ = F::Vec3_Dot(c, fp.wrapping_add(80), fp.wrapping_add(16)); F::Vec3_Scale(c, fp.wrapping_add(32), fp.wrapping_add(48), a2_) };
                        if (((rt::flt(F32(c.m.r32(fp.wrapping_add(32))), F32(0x00000000)) as i32)) != 0) {
                            { let v_ = F32(0x00000000); let a_ = fp.wrapping_add(32); c.m.w32(a_, (v_).0); };
                        }
                        if (((rt::flt(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4))), F32(0x00000000)) as i32)) != 0) {
                            { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        }
                        if (((rt::flt(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8))), F32(0x00000000)) as i32)) != 0) {
                            { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        }
                        F::Vec3_Add(c, fp.wrapping_add(32), fp.wrapping_add(32), fp.wrapping_add(64));
                        if (((rt::fgt(F32(c.m.r32(fp.wrapping_add(32))), F32(0x437f0000)) as i32)) != 0) {
                            { let v_ = F32(0x437f0000); let a_ = fp.wrapping_add(32); c.m.w32(a_, (v_).0); };
                        }
                        if (((rt::fgt(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4))), F32(0x437f0000)) as i32)) != 0) {
                            { let v_ = F32(0x437f0000); let a_ = (fp.wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        }
                        if (((rt::fgt(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8))), F32(0x437f0000)) as i32)) != 0) {
                            { let v_ = F32(0x437f0000); let a_ = (fp.wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        }
                        { let v_ = l_hi_9; let a_ = (l_part_4).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        l_one_7 = l_alpha_6;
                        l_zero_8 = l_alpha_6;
                        { let a0_ = rt::f2i(rt::fmul(F32(c.m.r32(fp.wrapping_add(32))), l_bright_5)); let a1_ = rt::f2i(rt::fmul(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4))), l_bright_5)); let a2_ = rt::f2i(rt::fmul(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8))), l_bright_5)); let a9_ = rt::f2u(rt::fmul(F32(c.m.r32(((l_work_3).wrapping_add(16)).wrapping_add(48))), F32(0x43000000))); let a10_ = rt::f2u(rt::fmul(F32(c.m.r32(((l_work_3).wrapping_add(16)).wrapping_add(48))), F32(0x43000000))); F::EftSpr_DrawFlat(c, a0_, a1_, a2_, rt::f2i(l_alpha_6), l_lo_10, l_lo_10, l_hi_9, l_hi_9, l_part_4, a9_, a10_, 0i32, 0i32, l_tex_2) };
                    }
                }
                { { let o_ = l_i_13; let n_ = (o_).wrapping_add(1); l_i_13 = n_; }; { let o_ = l_part_4; let n_ = (o_).wrapping_add(48); l_part_4 = n_; }; };
            }
        }
        F::Vu0Cur_Pop(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSmoke_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn eft_g__EftVec3_SubInl(c: &mut Ctx, mut p_out_1: u32, mut p_a_2: u32, mut p_b_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_g__EftVec3_SubInl", &PROBE_N, &[(p_out_1) as u32, (p_a_2) as u32, (p_b_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = rt::fsub(F32(c.m.r32(p_a_2)), F32(c.m.r32(p_b_3))); let a_ = p_out_1; c.m.w32(a_, (v_).0); };
    { let v_ = rt::fsub(F32(c.m.r32((p_a_2).wrapping_add(4))), F32(c.m.r32((p_b_3).wrapping_add(4)))); let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fsub(F32(c.m.r32((p_a_2).wrapping_add(8))), F32(c.m.r32((p_b_3).wrapping_add(8)))); let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "eft_g__EftVec3_SubInl", &PROBE_N, &[], true); }
    r_
}

pub fn EftBound_UpdateAngle(c: &mut Ctx, mut p_work_1: u32, mut p_src_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBound_UpdateAngle", &PROBE_N, &[(p_work_1) as u32, (p_src_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let a0_ = (c.m.r32(p_src_2) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, (p_work_1).wrapping_add(32)) };
        F::Vec3_Normalize(c, fp, (p_work_1).wrapping_add(32));
        { let v_ = { let a0_ = F32(c.m.r32(fp)); let a1_ = F32(c.m.r32((fp).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) }; let a_ = (p_work_1).wrapping_add(16); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBound_UpdateAngle", &PROBE_N, &[], true); }
    r_
}

pub fn EftBound_BuildWall(c: &mut Ctx, mut p_work_1: u32, mut p_src_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBound_BuildWall", &PROBE_N, &[(p_work_1) as u32, (p_src_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_param_3: u32 = 0;
    let mut l_color_4: u32 = 0;
    let mut l_mesh_5: u32 = 0;
    let mut l_vtx_6: u32 = 0;
    let mut l_alpha_7: F32 = F32(0);
    let mut l_m_8: u32 = 0;
    let mut l_m2_9: u32 = 0;
    let mut l_q_10: u32 = 0;
    let mut l_one_11: F32 = F32(0);
    let mut l_fade_12: F32 = F32(0);
    let mut l_y0_13: F32 = F32(0);
    let mut l_pulse_14: F32 = F32(0);
    let mut l_a_15: F32 = F32(0);
    let mut l_t_16: F32 = F32(0);
    let mut l_u_17: F32 = F32(0);
    let mut l_k_18: F32 = F32(0);
    let mut l_len_19: F32 = F32(0);
    let mut l_i_20: i32 = 0;
    let mut l_j_21: i32 = 0;
    let mut l_n_22: i32 = 0;
    let mut l_col_23: i32 = 0;
    let mut l_pi_24: u32 = 0;
    let r_ = 'f: {
        l_param_3 = { let a_ = (c.m.r32(sym::gEftBound)).wrapping_add(536); c.m.r32(a_) };
        l_color_4 = { let a_ = (c.m.r32(sym::gEftBound)).wrapping_add(540); c.m.r32(a_) };
        l_mesh_5 = (p_work_1).wrapping_add(48);
        l_q_10 = ((0i32) as u32);
        l_one_11 = F32(0x3f800000);
        { let v_ = rt::fmul({ let a0_ = F32(c.m.r32((p_work_1).wrapping_add(16))); F::Port_sinf(c, a0_) }, F32(c.m.r32((p_work_1).wrapping_add(4)))); let a_ = (l_mesh_5).wrapping_add(48); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((p_work_1).wrapping_add(32)).wrapping_add(4))); let a_ = ((l_mesh_5).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul({ let a0_ = F32(c.m.r32((p_work_1).wrapping_add(16))); F::Port_cosf(c, a0_) }, F32(c.m.r32((p_work_1).wrapping_add(4)))); let a_ = ((l_mesh_5).wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = l_one_11; let a_ = ((l_mesh_5).wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec3_Sub(c, fp, ((p_work_1).wrapping_add(48)).wrapping_add(48), (p_work_1).wrapping_add(32));
        { let v_ = { let a0_ = rt::fadd(rt::fmul(F32(c.m.r32(fp)), F32(c.m.r32(fp))), rt::fmul(F32(c.m.r32((fp).wrapping_add(8))), F32(c.m.r32((fp).wrapping_add(8))))); F::Port_sqrtf(c, a0_) }; let a_ = (l_mesh_5).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let r_ = F32(c.m.r32((p_work_1).wrapping_add(24))); let a_ = (l_mesh_5).wrapping_add(8); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if (((rt::flt(F32(c.m.r32((l_mesh_5).wrapping_add(8))), F32(0x00000000)) as i32)) != 0) {
            { let v_ = F32(0x00000000); let a_ = (l_mesh_5).wrapping_add(8); c.m.w32(a_, (v_).0); };
        }
        l_fade_12 = rt::fdiv(rt::fsub(F32(c.m.r32(l_param_3)), F32(c.m.r32((l_mesh_5).wrapping_add(8)))), F32(c.m.r32(l_param_3)));
        if (((rt::flt(l_fade_12, F32(0x00000000)) as i32)) != 0) {
            l_fade_12 = F32(0x00000000);
        }
        if ((((!((F::BtlScene_IsTimeStopped(c)) != 0)) as i32)) != 0) {
            { let r_ = F32(c.m.r32((l_param_3).wrapping_add(44))); let a_ = (l_mesh_5).wrapping_add(28); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::fgt(F32(c.m.r32((l_mesh_5).wrapping_add(28))), l_one_11) as i32)) != 0) {
                { let r_ = l_one_11; let a_ = (l_mesh_5).wrapping_add(28); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            }
            { let r_ = F32(c.m.r32((l_param_3).wrapping_add(48))); let a_ = (l_mesh_5).wrapping_add(32); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::fgt(F32(c.m.r32((l_mesh_5).wrapping_add(32))), l_one_11) as i32)) != 0) {
                { let r_ = l_one_11; let a_ = (l_mesh_5).wrapping_add(32); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            }
            { let r_ = rt::fmul(F32(c.m.r32((l_param_3).wrapping_add(52))), F32(0x40c90fda)); let a_ = (l_mesh_5).wrapping_add(24); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let v_ = { let a0_ = F32(c.m.r32((l_mesh_5).wrapping_add(24))); F::EftMath_WrapAngle(c, a0_) }; let a_ = (l_mesh_5).wrapping_add(24); c.m.w32(a_, (v_).0); };
        }
        if (((((({ let a0_ = (c.m.r32(p_src_2) as i32); F::BtlCharApi_IsInTechnique(c, a0_) }) != 0) || (({ let a0_ = (c.m.r32(p_src_2) as i32); F::BtlCharApi_IsRushConnected(c, a0_) }) != 0)) as i32)) != 0) {
            l_fade_12 = F32(0x00000000);
        }
        if ((((!(((rt::fgt(l_fade_12, F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = l_mesh_5; c.m.w32(a_, (v_) as u32); };
            break 'f;
        }
        { let v_ = 1i32; let a_ = l_mesh_5; c.m.w32(a_, (v_) as u32); };
        { let v_ = F32(c.m.r32((l_param_3).wrapping_add(28))); let a_ = (l_mesh_5).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x40c00000); let a_ = (l_mesh_5).wrapping_add(16); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x40800000); let a_ = (l_mesh_5).wrapping_add(20); c.m.w32(a_, (v_).0); };
        l_y0_13 = rt::fsub(F32(c.m.r32(((p_work_1).wrapping_add(32)).wrapping_add(4))), rt::fmul(F32(c.m.r32((l_param_3).wrapping_add(4))), F32(0x3f000000)));
        l_pulse_14 = { let a0_ = { let a0_ = F32(c.m.r32((l_mesh_5).wrapping_add(24))); F::Port_cosf(c, a0_) }; F::Port_fabsf(c, a0_) };
        {
            l_j_21 = 0i32;
            'l1: loop {
                if !(((((l_j_21) < (rt::f2i(F32(c.m.r32((l_mesh_5).wrapping_add(20)))))) as i32)) != 0) { break 'l1; }
                'c2: {
                    {
                        l_i_20 = 0i32;
                        'l3: loop {
                            if !(((((l_i_20) < (rt::f2i(F32(c.m.r32((l_mesh_5).wrapping_add(16)))))) as i32)) != 0) { break 'l3; }
                            'c4: {
                                l_vtx_6 = ((l_mesh_5).wrapping_add(64)).wrapping_add((((((l_j_21).wrapping_mul(rt::f2i(F32(c.m.r32((l_mesh_5).wrapping_add(16)))))).wrapping_add(l_i_20)) as u32)).wrapping_mul(64));
                                l_a_15 = rt::fadd(rt::fsub(F32(c.m.r32((p_work_1).wrapping_add(16))), rt::fmul(F32(c.m.r32((l_mesh_5).wrapping_add(12))), F32(0x3f000000))), rt::fmul(F32(c.m.r32((l_mesh_5).wrapping_add(12))), rt::fdiv(rt::i2f(l_i_20), rt::fsub(F32(c.m.r32((l_mesh_5).wrapping_add(16))), F32(0x3f800000)))));
                                { let v_ = rt::fmul(F::Port_sinf(c, l_a_15), F32(c.m.r32((p_work_1).wrapping_add(4)))); let a_ = (l_vtx_6).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fmul(F::Port_cosf(c, l_a_15), F32(c.m.r32((p_work_1).wrapping_add(4)))); let a_ = (l_vtx_6).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fadd(l_y0_13, rt::fmul(rt::fdiv(rt::i2f(l_j_21), rt::fsub(F32(c.m.r32((l_mesh_5).wrapping_add(20))), F32(0x3f800000))), F32(c.m.r32((l_param_3).wrapping_add(4))))); let a_ = (l_vtx_6).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = (l_vtx_6).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                l_u_17 = rt::fadd(F32(c.m.r32((l_mesh_5).wrapping_add(28))), rt::fdiv(l_a_15, F32(c.m.r32((l_param_3).wrapping_add(32)))));
                                l_t_16 = rt::fadd(F32(c.m.r32((l_mesh_5).wrapping_add(32))), rt::fdiv(F32(c.m.r32((l_vtx_6).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((l_param_3).wrapping_add(36)))));
                                { let v_ = l_u_17; let a_ = ((l_vtx_6).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let v_ = l_t_16; let a_ = ((l_vtx_6).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = ((l_vtx_6).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x00000000); let a_ = ((l_vtx_6).wrapping_add(32)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fsub(F32(c.m.r32((l_vtx_6).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((l_mesh_5).wrapping_add(48)))); let a_ = fp; c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fsub(F32(c.m.r32((l_vtx_6).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_mesh_5).wrapping_add(48)).wrapping_add(4)))); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fsub(F32(c.m.r32((l_vtx_6).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_mesh_5).wrapping_add(48)).wrapping_add(8)))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                l_len_19 = { let a0_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(fp)), F32(c.m.r32(fp))), rt::fmul(F32(c.m.r32((fp).wrapping_add(4))), F32(c.m.r32((fp).wrapping_add(4))))), rt::fmul(F32(c.m.r32((fp).wrapping_add(8))), F32(c.m.r32((fp).wrapping_add(8))))); F::Port_sqrtf(c, a0_) };
                                l_k_18 = rt::fdiv(rt::fsub(F32(c.m.r32((l_param_3).wrapping_add(24))), l_len_19), F32(c.m.r32((l_param_3).wrapping_add(24))));
                                if (((rt::flt(l_k_18, F32(0x00000000)) as i32)) != 0) {
                                    l_k_18 = F32(0x00000000);
                                } else {
                                    if (((rt::fgt(l_k_18, F32(0x3f800000)) as i32)) != 0) {
                                        l_k_18 = F32(0x3f800000);
                                    }
                                }
                                { let v_ = F32(c.m.r32((l_color_4).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((l_vtx_6).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(c.m.r32((l_color_4).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((l_vtx_6).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(c.m.r32((l_color_4).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = ((l_vtx_6).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                l_alpha_7 = rt::fmul(rt::fmul(F32(c.m.r32((l_param_3).wrapping_add(20))), l_fade_12), l_k_18);
                                l_alpha_7 = rt::fadd(l_alpha_7, rt::fmul(l_alpha_7, l_pulse_14));
                                { let v_ = l_alpha_7; let a_ = ((l_vtx_6).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                if (((rt::fgt(l_alpha_7, F32(0x437f0000)) as i32)) != 0) {
                                    { let v_ = F32(0x437f0000); let a_ = ((l_vtx_6).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                }
                            }
                            { let o_ = l_i_20; let n_ = (o_).wrapping_add(1); l_i_20 = n_; };
                        }
                    }
                }
                { let o_ = l_j_21; let n_ = (o_).wrapping_add(1); l_j_21 = n_; };
            }
        }
        { let v_ = ((((rt::f2i(F32(c.m.r32((l_mesh_5).wrapping_add(16))))).wrapping_sub(1i32)).wrapping_mul((rt::f2i(F32(c.m.r32((l_mesh_5).wrapping_add(20))))).wrapping_sub(1i32))) as i16); let a_ = (l_mesh_5).wrapping_add(4); c.m.w16(a_, (v_) as u16); };
        l_pi_24 = ((fp).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4));
        { let v_ = 0i32; let a_ = (l_pi_24).wrapping_add(((((1i32).wrapping_neg()) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 1i32; let a_ = (l_pi_24).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = rt::f2i(F32(c.m.r32((l_mesh_5).wrapping_add(16)))); let a_ = (l_pi_24).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = (rt::f2i(F32(c.m.r32((l_mesh_5).wrapping_add(16))))).wrapping_add(1i32); let a_ = (l_pi_24).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        l_col_23 = 0i32;
        {
            l_n_22 = 0i32;
            'l5: loop {
                if !(((((l_n_22) < ((((c.m.r16((l_mesh_5).wrapping_add(4)) as i16)) as i32))) as i32)) != 0) { break 'l5; }
                'c6: {
                    l_m2_9 = (l_mesh_5).wrapping_add((((l_n_22) as u32)).wrapping_mul(32u32));
                    l_q_10 = ((l_m2_9).wrapping_add(1600)).wrapping_add((((0i32) as u32)).wrapping_mul(32));
                    { let s_ = (fp).wrapping_add(16); let d_ = l_q_10; rt::memcpy(c, d_, s_, 16); };
                    l_col_23 = rt::rem_i32((l_col_23).wrapping_add(1i32), (rt::f2i(F32(c.m.r32((l_mesh_5).wrapping_add(16))))).wrapping_sub(1i32));
                    l_pi_24 = (fp).wrapping_add(16);
                    {
                        l_j_21 = 0i32;
                        'l7: loop {
                            if !(((((l_j_21) < (4i32)) as i32)) != 0) { break 'l7; }
                            'c8: {
                                if (((((l_col_23) == (0i32)) as i32)) != 0) {
                                    { let r_ = 2i32; let a_ = l_pi_24; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                                } else {
                                    { let r_ = 1i32; let a_ = l_pi_24; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                                }
                            }
                            { { let o_ = l_j_21; let n_ = (o_).wrapping_add(1); l_j_21 = n_; }; { let o_ = l_pi_24; let n_ = (o_).wrapping_add(4); l_pi_24 = n_; }; };
                        }
                    }
                }
                { let o_ = l_n_22; let n_ = (o_).wrapping_add(1); l_n_22 = n_; };
            }
        }
        l_vtx_6 = (l_mesh_5).wrapping_add(64);
        {
            l_i_20 = 0i32;
            'l9: loop {
                if !(((((l_i_20) < ((((c.m.r16((l_mesh_5).wrapping_add(4)) as i16)) as i32))) as i32)) != 0) { break 'l9; }
                'c10: {
                    l_m_8 = (l_mesh_5).wrapping_add((((l_i_20) as u32)).wrapping_mul(32u32));
                    { let _ = F::memcpy(c, fp.wrapping_add(64), ((l_m_8).wrapping_add(1600)).wrapping_add((((0i32) as u32)).wrapping_mul(32)), ((12i32) as u32)); };
                    { let a1_ = (l_vtx_6).wrapping_add(((((c.m.r32((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64)); let a2_ = (l_vtx_6).wrapping_add(((((c.m.r32((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64)); F::Vec3_Sub(c, fp.wrapping_add(32), a1_, a2_) };
                    { let a1_ = (l_vtx_6).wrapping_add(((((c.m.r32((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64)); let a2_ = (l_vtx_6).wrapping_add(((((c.m.r32((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64)); F::Vec3_Sub(c, fp.wrapping_add(48), a1_, a2_) };
                    F::Vec3_Cross(c, (((l_m_8).wrapping_add(1600)).wrapping_add((((0i32) as u32)).wrapping_mul(32))).wrapping_add(16), fp.wrapping_add(48), fp.wrapping_add(32));
                    F::Vec3_Normalize(c, (((l_m_8).wrapping_add(1600)).wrapping_add((((0i32) as u32)).wrapping_mul(32))).wrapping_add(16), (((l_m_8).wrapping_add(1600)).wrapping_add((((0i32) as u32)).wrapping_mul(32))).wrapping_add(16));
                }
                { let o_ = l_i_20; let n_ = (o_).wrapping_add(1); l_i_20 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBound_BuildWall", &PROBE_N, &[], true); }
    r_
}

pub fn EftBound_DrawTri(c: &mut Ctx, p_m_1: u32, p_p0_2: u32, p_p1_3: u32, p_p2_4: u32, p_st0_5: u32, p_st1_6: u32, p_st2_7: u32, p_c0_8: u32, p_c1_9: u32, p_c2_10: u32, mut p_otZ_11: i32, mut p_tex0_12: u64) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBound_DrawTri", &PROBE_N, &[(p_m_1) as u32, (p_p0_2) as u32, (p_p1_3) as u32, (p_p2_4) as u32, (p_st0_5) as u32, (p_st1_6) as u32, (p_st2_7) as u32, (p_c0_8) as u32, (p_c1_9) as u32, (p_c2_10) as u32, (p_otZ_11) as u32, (p_tex0_12) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(640) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 640);
    let mut l_fs_13: F32 = F32(0);
    let mut l_ft_14: F32 = F32(0);
    rt::memcpy(c, fp, p_m_1, 64);
    rt::memcpy(c, fp.wrapping_add(64), p_p0_2, 16);
    rt::memcpy(c, fp.wrapping_add(80), p_p1_3, 16);
    rt::memcpy(c, fp.wrapping_add(96), p_p2_4, 16);
    rt::memcpy(c, fp.wrapping_add(112), p_st0_5, 16);
    rt::memcpy(c, fp.wrapping_add(128), p_st1_6, 16);
    rt::memcpy(c, fp.wrapping_add(144), p_st2_7, 16);
    rt::memcpy(c, fp.wrapping_add(160), p_c0_8, 16);
    rt::memcpy(c, fp.wrapping_add(176), p_c1_9, 16);
    rt::memcpy(c, fp.wrapping_add(192), p_c2_10, 16);
    let r_ = 'f: {
        { let _ = F::memset(c, fp.wrapping_add(208), 0i32, 432u32); };
        F::ClipVtx_Set(c, (fp.wrapping_add(208)).wrapping_add((((0i32) as u32)).wrapping_mul(48)), fp.wrapping_add(64), fp.wrapping_add(112), fp.wrapping_add(160));
        F::ClipVtx_Set(c, (fp.wrapping_add(208)).wrapping_add((((1i32) as u32)).wrapping_mul(48)), fp.wrapping_add(80), fp.wrapping_add(128), fp.wrapping_add(176));
        F::ClipVtx_Set(c, (fp.wrapping_add(208)).wrapping_add((((2i32) as u32)).wrapping_mul(48)), fp.wrapping_add(96), fp.wrapping_add(144), fp.wrapping_add(192));
        l_fs_13 = rt::i2f(rt::f2i(F32(c.m.r32((fp.wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))));
        l_ft_14 = rt::i2f(rt::f2i(F32(c.m.r32((fp.wrapping_add(112)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))));
        { let r_ = l_fs_13; let a_ = (((fp.wrapping_add(208)).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = l_fs_13; let a_ = (((fp.wrapping_add(208)).wrapping_add((((1i32) as u32)).wrapping_mul(48))).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = l_fs_13; let a_ = (((fp.wrapping_add(208)).wrapping_add((((2i32) as u32)).wrapping_mul(48))).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = l_ft_14; let a_ = (((fp.wrapping_add(208)).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = l_ft_14; let a_ = (((fp.wrapping_add(208)).wrapping_add((((1i32) as u32)).wrapping_mul(48))).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = l_ft_14; let a_ = (((fp.wrapping_add(208)).wrapping_add((((2i32) as u32)).wrapping_mul(48))).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        F::EftGfx_DrawPolyScaledZ(c, fp.wrapping_add(208), p_otZ_11, 1i32, 0i32, 0i32, 0i32, p_tex0_12, F32(0x40000000));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBound_DrawTri", &PROBE_N, &[], true); }
    r_
}

pub fn EftBound_DrawMeshCulled(c: &mut Ctx, mut p_mesh_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBound_DrawMeshCulled", &PROBE_N, &[(p_mesh_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_param_2: u32 = 0;
    let mut l_tex_3: u32 = 0;
    let mut l_vtx_4: u32 = 0;
    let mut l_m_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let r_ = 'f: {
        l_param_2 = { let a_ = (c.m.r32(sym::gEftBound)).wrapping_add(536); c.m.r32(a_) };
        l_tex_3 = (c.m.r32(sym::gEftBound)).wrapping_add(16);
        l_vtx_4 = (p_mesh_1).wrapping_add(64);
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(544); F::Vec4_Copy(c, fp, a1_) };
        F::Vu0Cur_Push(c);
        { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
        {
            l_i_6 = 0i32;
            'l1: loop {
                if !(((((l_i_6) < ((((c.m.r16((p_mesh_1).wrapping_add(4)) as i16)) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_m_5 = (p_mesh_1).wrapping_add((((l_i_6) as u32)).wrapping_mul(32u32));
                    { let s_ = ((l_m_5).wrapping_add(1600)).wrapping_add((((0i32) as u32)).wrapping_mul(32)); let d_ = fp.wrapping_add(32); rt::memcpy(c, d_, s_, 16); };
                    { let a1_ = (l_vtx_4).wrapping_add(((((c.m.r32((fp.wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64)); F::Vec3_Sub(c, fp.wrapping_add(16), a1_, fp) };
                    F::Vec3_Normalize(c, fp.wrapping_add(16), fp.wrapping_add(16));
                    if (((rt::flt(F::Vec3_Dot(c, (((l_m_5).wrapping_add(1600)).wrapping_add((((0i32) as u32)).wrapping_mul(32))).wrapping_add(16), fp.wrapping_add(16)), F32(0x00000000)) as i32)) != 0) {
                        { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); let a1_ = (l_vtx_4).wrapping_add(((((c.m.r32((fp.wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64)); let a2_ = (l_vtx_4).wrapping_add(((((c.m.r32((fp.wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64)); let a3_ = (l_vtx_4).wrapping_add(((((c.m.r32((fp.wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64)); let a4_ = ((l_vtx_4).wrapping_add(((((c.m.r32((fp.wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64))).wrapping_add(32); let a5_ = ((l_vtx_4).wrapping_add(((((c.m.r32((fp.wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64))).wrapping_add(32); let a6_ = ((l_vtx_4).wrapping_add(((((c.m.r32((fp.wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64))).wrapping_add(32); let a7_ = ((l_vtx_4).wrapping_add(((((c.m.r32((fp.wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64))).wrapping_add(48); let a8_ = ((l_vtx_4).wrapping_add(((((c.m.r32((fp.wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64))).wrapping_add(48); let a9_ = ((l_vtx_4).wrapping_add(((((c.m.r32((fp.wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64))).wrapping_add(48); let a10_ = (c.m.r32((l_param_2).wrapping_add(56)) as i32); let a11_ = c.m.r64(l_tex_3); F::EftBound_DrawTri(c, a0_, a1_, a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_, a10_, a11_) };
                        { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); let a1_ = (l_vtx_4).wrapping_add(((((c.m.r32((fp.wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64)); let a2_ = (l_vtx_4).wrapping_add(((((c.m.r32((fp.wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64)); let a3_ = (l_vtx_4).wrapping_add(((((c.m.r32((fp.wrapping_add(32)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64)); let a4_ = ((l_vtx_4).wrapping_add(((((c.m.r32((fp.wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64))).wrapping_add(32); let a5_ = ((l_vtx_4).wrapping_add(((((c.m.r32((fp.wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64))).wrapping_add(32); let a6_ = ((l_vtx_4).wrapping_add(((((c.m.r32((fp.wrapping_add(32)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64))).wrapping_add(32); let a7_ = ((l_vtx_4).wrapping_add(((((c.m.r32((fp.wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64))).wrapping_add(48); let a8_ = ((l_vtx_4).wrapping_add(((((c.m.r32((fp.wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64))).wrapping_add(48); let a9_ = ((l_vtx_4).wrapping_add(((((c.m.r32((fp.wrapping_add(32)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64))).wrapping_add(48); let a10_ = (c.m.r32((l_param_2).wrapping_add(56)) as i32); let a11_ = c.m.r64(l_tex_3); F::EftBound_DrawTri(c, a0_, a1_, a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_, a10_, a11_) };
                    }
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
        F::Vu0Cur_Pop(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBound_DrawMeshCulled", &PROBE_N, &[], true); }
    r_
}

pub fn EftBound_DrawMesh(c: &mut Ctx, mut p_mesh_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBound_DrawMesh", &PROBE_N, &[(p_mesh_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_param_2: u32 = 0;
    let mut l_tex_3: u32 = 0;
    let mut l_vtx_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
        l_param_2 = { let a_ = (c.m.r32(sym::gEftBound)).wrapping_add(536); c.m.r32(a_) };
        l_tex_3 = (c.m.r32(sym::gEftBound)).wrapping_add(16);
        l_vtx_4 = (p_mesh_1).wrapping_add(64);
        F::Vu0Cur_Push(c);
        { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
        {
            l_i_5 = 0i32;
            'l1: loop {
                if !(((((l_i_5) < ((((c.m.r16((p_mesh_1).wrapping_add(4)) as i16)) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let s_ = ((p_mesh_1).wrapping_add(1600)).wrapping_add((((l_i_5) as u32)).wrapping_mul(32)); let d_ = fp; rt::memcpy(c, d_, s_, 16); };
                    { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); let a1_ = (l_vtx_4).wrapping_add(((((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64)); let a2_ = (l_vtx_4).wrapping_add(((((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64)); let a3_ = (l_vtx_4).wrapping_add(((((c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64)); let a4_ = ((l_vtx_4).wrapping_add(((((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64))).wrapping_add(32); let a5_ = ((l_vtx_4).wrapping_add(((((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64))).wrapping_add(32); let a6_ = ((l_vtx_4).wrapping_add(((((c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64))).wrapping_add(32); let a7_ = ((l_vtx_4).wrapping_add(((((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64))).wrapping_add(48); let a8_ = ((l_vtx_4).wrapping_add(((((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64))).wrapping_add(48); let a9_ = ((l_vtx_4).wrapping_add(((((c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64))).wrapping_add(48); let a10_ = (c.m.r32((l_param_2).wrapping_add(56)) as i32); let a11_ = c.m.r64(l_tex_3); F::EftBound_DrawTri(c, a0_, a1_, a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_, a10_, a11_) };
                    { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); let a1_ = (l_vtx_4).wrapping_add(((((c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64)); let a2_ = (l_vtx_4).wrapping_add(((((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64)); let a3_ = (l_vtx_4).wrapping_add(((((c.m.r32((fp).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64)); let a4_ = ((l_vtx_4).wrapping_add(((((c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64))).wrapping_add(32); let a5_ = ((l_vtx_4).wrapping_add(((((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64))).wrapping_add(32); let a6_ = ((l_vtx_4).wrapping_add(((((c.m.r32((fp).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64))).wrapping_add(32); let a7_ = ((l_vtx_4).wrapping_add(((((c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64))).wrapping_add(48); let a8_ = ((l_vtx_4).wrapping_add(((((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64))).wrapping_add(48); let a9_ = ((l_vtx_4).wrapping_add(((((c.m.r32((fp).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64))).wrapping_add(48); let a10_ = (c.m.r32((l_param_2).wrapping_add(56)) as i32); let a11_ = c.m.r64(l_tex_3); F::EftBound_DrawTri(c, a0_, a1_, a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_, a10_, a11_) };
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
        F::Vu0Cur_Pop(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBound_DrawMesh", &PROBE_N, &[], true); }
    r_
}

pub fn EftBound_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBound_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_work_3: u32 = 0;
    let mut l_param_4: u32 = 0;
    let mut l_h_5: F32 = F32(0);
    let r_ = 'f: {
    l_work_3 = c.m.r32((p_task_1).wrapping_add(56));
    l_param_4 = { let a_ = (c.m.r32(sym::gEftBound)).wrapping_add(536); c.m.r32(a_) };
    { let _ = F::memset(c, l_work_3, 0i32, 4208u32); };
    { let v_ = (c.m.r32(p_arg_2) as i32); let a_ = l_work_3; c.m.w32(a_, (v_) as u32); };
    { let v_ = rt::fdiv({ let a0_ = (c.m.r32(p_arg_2) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x419acccc)); let a_ = (l_work_3).wrapping_add(20); c.m.w32(a_, (v_).0); };
    if (((rt::flt(F32(c.m.r32((l_work_3).wrapping_add(20))), F32(0x3f333333)) as i32)) != 0) {
        { let v_ = F32(0x3f333333); let a_ = (l_work_3).wrapping_add(20); c.m.w32(a_, (v_).0); };
    }
    { let v_ = { let a0_ = (c.m.r32(p_arg_2) as i32); F::BtlCharApi_GetRadius(c, a0_) }; let a_ = (l_work_3).wrapping_add(24); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(F::BtlStage_GetInnerRadius(c), F32(c.m.r32((l_param_4).wrapping_add(12)))); let a_ = (l_work_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fsub(F32(c.m.r32((l_param_4).wrapping_add(16))), F::BtlStage_GetTop(c)); let a_ = (l_work_3).wrapping_add(8); c.m.w32(a_, (v_).0); };
    l_h_5 = F::BtlStage_GetBottom(c);
    { let v_ = rt::fsub(l_h_5, F32(c.m.r32((l_work_3).wrapping_add(8)))); let a_ = (l_work_3).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = ((l_work_3).wrapping_add(48)).wrapping_add(28); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = ((l_work_3).wrapping_add(48)).wrapping_add(32); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = ((l_work_3).wrapping_add(2128)).wrapping_add(28); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = ((l_work_3).wrapping_add(2128)).wrapping_add(32); c.m.w32(a_, (v_).0); };
    { let v_ = 0i32; let a_ = (l_work_3).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (l_work_3).wrapping_add(2128); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBound_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftBound_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBound_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBound_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftBound_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBound_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_work_2: u32 = 0;
    let mut l_tex_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_work_2 = c.m.r32((p_task_1).wrapping_add(56));
    F::EftBound_UpdateAngle(c, l_work_2, l_work_2);
    F::EftBound_BuildWall(c, l_work_2, l_work_2);
    if ((((({ let a_ = c.m.r32(sym::gEftBound); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        l_tex_3 = (c.m.r32(sym::gEftBound)).wrapping_add(16);
        if ((((((((((c.m.r32((l_work_2).wrapping_add(48)) as i32)) != (0i32)) as i32)) != 0) || ((((((c.m.r32((l_work_2).wrapping_add(2128)) as i32)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
            {
                l_i_4 = 0i32;
                'l1: loop {
                    if !(((((l_i_4) < ((c.m.r32((l_tex_3).wrapping_add(512)) as i32))) as i32)) != 0) { break 'l1; }
                    'c2: {
                        { let v_ = F::EftVram_AddTex(c, (l_tex_3).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)), 1i32, 0i32); let a_ = (l_tex_3).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)); c.m.w64(a_, v_); };
                    }
                    { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                }
            }
            { let v_ = 1i32; let a_ = c.m.r32(sym::gEftBound); c.m.w32(a_, (v_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBound_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftBound_PostUpdate(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBound_PostUpdate", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((F::BtlScene_IsTimeStopped(c)) != 0) {
        break 'f;
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBound_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftBound_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBound_Reset", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBound_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftBound_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBound_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_work_2: u32 = 0;
    let r_ = 'f: {
    l_work_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!((F::EftStage_IsDrawOn(c)) != 0)) as i32)) != 0) {
        break 'f;
    }
    if (({ let a0_ = (c.m.r32(l_work_2) as i32); F::BtlCharApi_IsHidden(c, a0_) }) != 0) {
        break 'f;
    }
    if (({ let a0_ = (c.m.r32(l_work_2) as i32); F::BtlCharApi_IsLockedOn(c, a0_) }) != 0) {
        if (({ let a0_ = (c.m.r32(l_work_2) as i32); F::BtlCharApi_IsRushConnected(c, a0_) }) != 0) {
            break 'f;
        }
        if (({ let a0_ = { let a0_ = (c.m.r32(l_work_2) as i32); F::BtlCharApi_GetOpponentObjId(c, a0_) }; F::BtlCharApi_IsRushConnected(c, a0_) }) != 0) {
            break 'f;
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gEftBound)).wrapping_add(4); (c.m.r32(a_) as i32) }) != (1i32)) as i32)) != 0) {
        break 'f;
    }
    if ((((((c.m.r32((l_work_2).wrapping_add(48)) as i32)) != (0i32)) as i32)) != 0) {
        F::EftBound_DrawMeshCulled(c, (l_work_2).wrapping_add(48));
    }
    if ((((((c.m.r32((l_work_2).wrapping_add(2128)) as i32)) != (0i32)) as i32)) != 0) {
        F::EftBound_DrawMesh(c, (l_work_2).wrapping_add(2128));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBound_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftBoundMgr_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBoundMgr_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_pack_2: i32 = 0;
    let r_ = 'f: {
    l_pack_2 = F::BtlScene_GetStageData(c);
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((544u32) as i32)) }; let a_ = sym::gEftBound; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gEftBound); F::memset(c, a0_, 0i32, 544u32) }; };
    { let v_ = F::BtlScene_GetPackEntry(c, ((l_pack_2) as u32), 21i32); let a_ = (c.m.r32(sym::gEftBound)).wrapping_add(12); c.m.w32(a_, v_); };
    { let v_ = F::BtlScene_GetPackEntry(c, ((l_pack_2) as u32), 23i32); let a_ = (c.m.r32(sym::gEftBound)).wrapping_add(536); c.m.w32(a_, v_); };
    { let v_ = F::BtlScene_GetPackEntry(c, ((l_pack_2) as u32), 22i32); let a_ = (c.m.r32(sym::gEftBound)).wrapping_add(540); c.m.w32(a_, v_); };
    { let a0_ = (c.m.r32(sym::gEftBound)).wrapping_add(16); let a1_ = { let a_ = (c.m.r32(sym::gEftBound)).wrapping_add(12); c.m.r32(a_) }; F::EftTexSet_Load32(c, a0_, a1_) };
    F::EftBoundMgr_Enable(c);
    { let v_ = F::BtlTask_CreateChildList(c, p_task_1, ((((2i32) as i64 as u64)) as u32), ((((((4208u32) as i32)) as i64 as u64)) as u32)); let a_ = sym::gEftBoundList; c.m.w32(a_, v_); };
    { let _ = F::EftBoundMgr_AddChar(c, 0i32); };
    { let _ = F::EftBoundMgr_AddChar(c, 1i32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBoundMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftBoundMgr_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBoundMgr_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32(sym::gEftBoundList); F::BtlTaskList_KillAll(c, a0_) };
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftBound); F::BtlPool_Free(c, a0_, a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftBound; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBoundMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftBoundMgr_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBoundMgr_Update", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = c.m.r32(sym::gEftBound); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBoundMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftBoundMgr_AddChar(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBoundMgr_AddChar", &PROBE_N, &[(p_objId_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = p_objId_1; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        break 'f ((({ let a0_ = c.m.r32(sym::gEftBoundList); F::BtlTaskList_AddTail(c, a0_, sym::gEftBoundClass, fp) }) != (((0i32) as u32))) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBoundMgr_AddChar", &PROBE_N, &[], true); }
    r_
}

pub fn EftBoundMgr_Enable(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBoundMgr_Enable", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftBound)) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = 1i32; let a_ = (c.m.r32(sym::gEftBound)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBoundMgr_Enable", &PROBE_N, &[], true); }
    r_
}

pub fn EftBoundMgr_Disable(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBoundMgr_Disable", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftBound)) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gEftBound)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBoundMgr_Disable", &PROBE_N, &[], true); }
    r_
}

pub fn EftShot_AttrMaskFromBits(c: &mut Ctx, mut p_bits_1: u64) -> u64 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShot_AttrMaskFromBits", &PROBE_N, &[(p_bits_1) as u32], false); }
    let mut l_mask_2: u64 = 0;
    let r_ = 'f: {
    l_mask_2 = (((p_bits_1).wrapping_shl((8i32) as u32)) & (((512i32) as u64)));
    if ((((p_bits_1) & (((4i32) as u64)))) != 0) {
        { let r_ = ((1024i32) as u64); let n_ = ((l_mask_2) | (r_)); l_mask_2 = n_; };
    }
    if ((((p_bits_1) & (((8i32) as u64)))) != 0) {
        { let r_ = ((2048i32) as u64); let n_ = ((l_mask_2) | (r_)); l_mask_2 = n_; };
    }
    if ((((p_bits_1) & (((16i32) as u64)))) != 0) {
        { let r_ = ((4096i32) as u64); let n_ = ((l_mask_2) | (r_)); l_mask_2 = n_; };
    }
    if ((((p_bits_1) & (((32i32) as u64)))) != 0) {
        { let r_ = ((8192i32) as u64); let n_ = ((l_mask_2) | (r_)); l_mask_2 = n_; };
    }
    if ((((p_bits_1) & (((64i32) as u64)))) != 0) {
        { let r_ = ((16384i32) as u64); let n_ = ((l_mask_2) | (r_)); l_mask_2 = n_; };
    }
    if ((((p_bits_1) & (((128i32) as u64)))) != 0) {
        { let r_ = ((2097152i32) as u64); let n_ = ((l_mask_2) | (r_)); l_mask_2 = n_; };
    }
    if ((((p_bits_1) & (((256i32) as u64)))) != 0) {
        { let r_ = ((4194304i32) as u64); let n_ = ((l_mask_2) | (r_)); l_mask_2 = n_; };
    }
    if ((((p_bits_1) & (((512i32) as u64)))) != 0) {
        { let r_ = ((8388608i32) as u64); let n_ = ((l_mask_2) | (r_)); l_mask_2 = n_; };
    }
    break 'f l_mask_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShot_AttrMaskFromBits", &PROBE_N, &[], true); }
    r_
}

pub fn EftShot_TestBits(c: &mut Ctx, mut p_objId_1: i32, mut p_mask_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShot_TestBits", &PROBE_N, &[(p_objId_1) as u32, (p_mask_2) as u32], false); }
    let r_ = 'f: {
    break 'f (((((F::EftTechEvt_GetEvents(c, p_objId_1)) & (p_mask_2))) != (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShot_TestBits", &PROBE_N, &[], true); }
    r_
}

pub fn EftShot_GetAttrValue(c: &mut Ctx, mut p_objId_1: i32, mut p_bits_2: u64) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShot_GetAttrValue", &PROBE_N, &[(p_objId_1) as u32, (p_bits_2) as u32], false); }
    let mut l_mask_3: u64 = 0;
    let mut l_r_4: i32 = 0;
    let r_ = 'f: {
    l_mask_3 = F::EftShot_AttrMaskFromBits(c, p_bits_2);
    l_r_4 = F::BtlCharApi_ObjTestAttr(c, p_objId_1, l_mask_3);
    if (((((l_r_4) != (0i32)) as i32)) != 0) {
        break 'f F::BtlCharApi_ObjGetAttrValue(c, p_objId_1, l_mask_3);
    }
    break 'f l_r_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShot_GetAttrValue", &PROBE_N, &[], true); }
    r_
}

pub fn EftShot_GetAttrKind(c: &mut Ctx, mut p_objId_1: i32, mut p_bits_2: u64) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShot_GetAttrKind", &PROBE_N, &[(p_objId_1) as u32, (p_bits_2) as u32], false); }
    let mut l_slot_3: i32 = 0;
    let mut l_mask_4: u64 = 0;
    let mut l_r_5: i32 = 0;
    let r_ = 'f: {
    l_slot_3 = F::EftShot_GetCurSlot(c, p_objId_1);
    if (((((l_slot_3) < (0i32)) as i32)) != 0) {
        break 'f 17i32;
    }
    if ((F::EftShot_HasTwoAttrs(c, p_objId_1, p_bits_2)) != 0) {
        break 'f 17i32;
    }
    if ((F::EftShot_IsSlotFlag2000(c, p_objId_1, l_slot_3)) != 0) {
        break 'f 54i32;
    }
    l_mask_4 = F::EftShot_AttrMaskFromBits(c, p_bits_2);
    l_r_5 = F::BtlCharApi_ObjTestAttr(c, p_objId_1, l_mask_4);
    if (((((l_r_5) != (0i32)) as i32)) != 0) {
        break 'f F::BtlCharApi_ObjGetAttrKind(c, p_objId_1, l_mask_4);
    }
    break 'f l_r_5;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShot_GetAttrKind", &PROBE_N, &[], true); }
    r_
}

pub fn EftShot_IsSlotFlag2000(c: &mut Ctx, mut p_chr_1: i32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShot_IsSlotFlag2000", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let mut l_c_3: u32 = 0;
    let mut l_s_4: u32 = 0;
    let r_ = 'f: {
    l_c_3 = ({ let a_ = (c.m.r32(sym::gEftShot)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((p_chr_1) as u32)).wrapping_mul(1344));
    l_s_4 = (l_c_3).wrapping_add((((p_slot_2) as u32)).wrapping_mul(80));
    if (((({ let a_ = (c.m.r32((l_s_4).wrapping_add(36))).wrapping_add(60); (c.m.r32(a_) as i32) }) & (8192i32))) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShot_IsSlotFlag2000", &PROBE_N, &[], true); }
    r_
}

pub fn EftShot_HasTwoAttrs(c: &mut Ctx, mut p_objId_1: i32, mut p_bits_2: u64) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShot_HasTwoAttrs", &PROBE_N, &[(p_objId_1) as u32, (p_bits_2) as u32], false); }
    let mut l_v_3: i32 = 0;
    let mut l_n_4: i32 = 0;
    let r_ = 'f: {
    l_v_3 = F::EftShot_GetAttrValue(c, p_objId_1, p_bits_2);
    l_n_4 = 0i32;
    if ((((l_v_3) & (131072i32))) != 0) {
        l_n_4 = 1i32;
    }
    if ((((l_v_3) & (65536i32))) != 0) {
        { let o_ = l_n_4; let n_ = (o_).wrapping_add(1); l_n_4 = n_; };
    }
    if ((((l_v_3) & (262144i32))) != 0) {
        { let o_ = l_n_4; let n_ = (o_).wrapping_add(1); l_n_4 = n_; };
    }
    if (((((l_n_4) < (2i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShot_HasTwoAttrs", &PROBE_N, &[], true); }
    r_
}

pub fn EftShot_GetAttrPair(c: &mut Ctx, mut p_objId_1: i32, mut p_bits_2: u64, mut p_outA_3: u32, mut p_outB_4: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShot_GetAttrPair", &PROBE_N, &[(p_objId_1) as u32, (p_bits_2) as u32, (p_outA_3) as u32, (p_outB_4) as u32], false); }
    let mut l_v_5: i32 = 0;
    let mut l_a_6: i32 = 0;
    let mut l_b_7: i32 = 0;
    let r_ = 'f: {
    l_v_5 = F::EftShot_GetAttrValue(c, p_objId_1, p_bits_2);
    { let v_ = 17i32; let a_ = p_outA_3; c.m.w32(a_, (v_) as u32); };
    { let v_ = 17i32; let a_ = p_outB_4; c.m.w32(a_, (v_) as u32); };
    if ((((!((((l_v_5) & (458752i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_a_6 = ((l_v_5) & (131072i32));
    if ((l_a_6) != 0) {
        { let v_ = 45i32; let a_ = p_outA_3; c.m.w32(a_, (v_) as u32); };
    }
    l_b_7 = ((l_v_5) & (65536i32));
    if ((l_b_7) != 0) {
        { let v_ = 31i32; let a_ = p_outB_4; c.m.w32(a_, (v_) as u32); };
    }
    if ((((l_v_5) & (262144i32))) != 0) {
        if ((l_a_6) != 0) {
            { let v_ = 54i32; let a_ = p_outB_4; c.m.w32(a_, (v_) as u32); };
        }
        if ((l_b_7) != 0) {
            { let v_ = 54i32; let a_ = p_outA_3; c.m.w32(a_, (v_) as u32); };
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShot_GetAttrPair", &PROBE_N, &[], true); }
    r_
}

pub fn EftShot_SetHeldFlagA8(c: &mut Ctx, mut p_objId_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShot_SetHeldFlagA8", &PROBE_N, &[(p_objId_1) as u32], false); }
    let r_ = 'f: {
    F::BtlCharApi_SetHeldFlagA8(c, p_objId_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShot_SetHeldFlagA8", &PROBE_N, &[], true); }
    r_
}

pub fn EftShot_SetHeldFlagA9(c: &mut Ctx, mut p_objId_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShot_SetHeldFlagA9", &PROBE_N, &[(p_objId_1) as u32], false); }
    let r_ = 'f: {
    F::BtlCharApi_SetHeldFlagA9(c, p_objId_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShot_SetHeldFlagA9", &PROBE_N, &[], true); }
    r_
}

pub fn EftShot_Request(c: &mut Ctx, mut p_req_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShot_Request", &PROBE_N, &[(p_req_1) as u32], false); }
    let r_ = 'f: {
    F::EftShot_Start(c, p_req_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShot_Request", &PROBE_N, &[], true); }
    r_
}

pub fn EftShot_Nop(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShot_Nop", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShot_Nop", &PROBE_N, &[], true); }
    r_
}

pub fn EftShot_SetCurSlot(c: &mut Ctx, mut p_chr_1: i32, mut p_slot_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShot_SetCurSlot", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let mut l_c_3: u32 = 0;
    let r_ = 'f: {
    l_c_3 = ({ let a_ = (c.m.r32(sym::gEftShot)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((p_chr_1) as u32)).wrapping_mul(1344));
    { let v_ = p_slot_2; let a_ = (l_c_3).wrapping_add(1328); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShot_SetCurSlot", &PROBE_N, &[], true); }
    r_
}

pub fn EftShot_GetCurSlot(c: &mut Ctx, mut p_chr_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShot_GetCurSlot", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_c_2: u32 = 0;
    let r_ = 'f: {
    l_c_2 = ({ let a_ = (c.m.r32(sym::gEftShot)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((p_chr_1) as u32)).wrapping_mul(1344));
    break 'f (c.m.r32((l_c_2).wrapping_add(1328)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShot_GetCurSlot", &PROBE_N, &[], true); }
    r_
}

pub fn EftShot_PlayFireSound(c: &mut Ctx, mut p_objId_1: i32, mut p_slot_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShot_PlayFireSound", &PROBE_N, &[(p_objId_1) as u32, (p_slot_2) as u32], false); }
    let mut l_snd_3: i32 = 0;
    let r_ = 'f: {
    l_snd_3 = (1i32).wrapping_neg();
    if ((((((({ let a_ = (c.m.r32((p_slot_2).wrapping_add(36))).wrapping_add(4); (c.m.r8(a_) as i8) }) as i32)) == (0i32)) as i32)) != 0) {
        if ((F::EftShot_TestBits(c, p_objId_1, 2i32)) != 0) {
            if ((((((({ let a_ = c.m.r32((p_slot_2).wrapping_add(36)); (c.m.r16(a_) as i16) }) as i32)) != (39i32)) as i32)) != 0) {
                l_snd_3 = 0i32;
            }
        }
        if ((F::EftShot_TestBits(c, p_objId_1, 4i32)) != 0) {
            l_snd_3 = 1i32;
        }
        if ((F::BtlCharApi_ObjTestAttr(c, p_objId_1, ((262144i32) as u64))) != 0) {
            l_snd_3 = 2i32;
        }
    }
    if (((((l_snd_3) >= (0i32)) as i32)) != 0) {
        { let _ = F::BtlCharApi_PlayTechniqueSound(c, p_objId_1, ((l_snd_3) as u32)); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShot_PlayFireSound", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotMgr_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotMgr_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_size_2: i32 = 0;
    let mut l_c_3: u32 = 0;
    let r_ = 'f: {
        F::BtlPool_SetCurrent(c, 3i32);
        { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((12u32) as i32)) }; let a_ = sym::gEftShot; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gEftShot); F::memset(c, a0_, 0i32, 12u32) }; };
        { let v_ = F::BtlScene_GetCharCount(c); let a_ = (c.m.r32(sym::gEftShot)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        l_size_2 = ((((({ let a_ = (c.m.r32(sym::gEftShot)).wrapping_add(8); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1344u32)) as i32);
        { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, l_size_2) }; let a_ = (c.m.r32(sym::gEftShot)).wrapping_add(4); c.m.w32(a_, v_); };
        { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gEftShot)).wrapping_add(4); c.m.r32(a_) }; F::memset(c, a0_, 0i32, ((l_size_2) as u32)) }; };
        { let v_ = { let a1_ = ({ let a_ = (c.m.r32(sym::gEftShot)).wrapping_add(8); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::BtlTask_CreateChildList(c, p_task_1, ((((a1_) as i64 as u64)) as u32), ((((0i32) as i64 as u64)) as u32)) }; let a_ = c.m.r32(sym::gEftShot); c.m.w32(a_, v_); };
        { let _ = { let a0_ = { let a_ = c.m.r32(sym::gEftShot); c.m.r32(a_) }; F::BtlTaskList_AddTail(c, a0_, sym::gEftTechEvtClass, ((0i32) as u32)) }; };
        { let v_ = 0i32; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 1i32; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        l_c_3 = { let a_ = (c.m.r32(sym::gEftShot)).wrapping_add(4); c.m.r32(a_) };
        { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gEftShot); c.m.r32(a_) }; F::BtlTaskList_AddTail(c, a0_, sym::gEftShotCharClass, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) }; let a_ = (l_c_3).wrapping_add(1324); c.m.w32(a_, v_); };
        l_c_3 = ({ let a_ = (c.m.r32(sym::gEftShot)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(1344));
        { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gEftShot); c.m.r32(a_) }; F::BtlTaskList_AddTail(c, a0_, sym::gEftShotCharClass, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) }; let a_ = (l_c_3).wrapping_add(1324); c.m.w32(a_, v_); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftShotMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotMgr_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotMgr_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::BtlPool_SetCurrent(c, 3i32);
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = { let a_ = (c.m.r32(sym::gEftShot)).wrapping_add(4); c.m.r32(a_) }; F::BtlPool_Free(c, a0_, a1_) };
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftShot); F::BtlPool_Free(c, a0_, a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftShot; c.m.w32(a_, v_); };
    F::BtlPool_Reset(c, 3i32);
    F::BtlPool_Reset(c, 4i32);
    F::BtlPool_Reset(c, 5i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotMgr_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotMgr_Update", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotChar_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotChar_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_chr_3 = ({ let a_ = (c.m.r32(sym::gEftShot)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(((((c.m.r32(p_arg_2) as i32)) as u32)).wrapping_mul(1344));
    { let v_ = (((c.m.r32(p_arg_2) as i32)) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
    { let a0_ = (c.m.r32(p_arg_2) as i32); F::EftShot_SetCharPool(c, a0_) };
    { let v_ = F::BtlTask_CreateChildList(c, p_task_1, ((((6i32) as i64 as u64)) as u32), ((((8192i32) as i64 as u64)) as u32)); let a_ = (l_chr_3).wrapping_add(1320); c.m.w32(a_, v_); };
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (5i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (({ let a0_ = (c.m.r32(p_arg_2) as i32); let a2_ = { let a0_ = (c.m.r32(p_arg_2) as i32); ((((F::EftShot_GetCharPack(c, a0_, l_i_4)) as i64 as u64)) as u32) }; F::EftShot_InitSlot(c, a0_, l_i_4, a2_) }) != 0) {
                    { let a0_ = (c.m.r32(p_arg_2) as i32); F::EftShot_CreateSlotTask(c, a0_, l_i_4) };
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotChar_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotChar_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotChar_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = ((c.m.r8((p_task_1).wrapping_add(1))) as i32); F::EftShot_ResetCharPool(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotChar_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotChar_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotChar_Update", &PROBE_N, &[], false); }
    let mut l_chr_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < ({ let a_ = (c.m.r32(sym::gEftShot)).wrapping_add(8); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l1; }
            'c2: {
                l_chr_1 = ({ let a_ = (c.m.r32(sym::gEftShot)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((l_i_2) as u32)).wrapping_mul(1344));
                if ((((((((!((F::BtlCharApi_IsInTechnique(c, l_i_2)) != 0)) as i32)) != 0) && ((((!((F::BtlCharApi_IsInSkill(c, l_i_2)) != 0)) as i32)) != 0)) as i32)) != 0) {
                    { let v_ = (1i32).wrapping_neg(); let a_ = (l_chr_1).wrapping_add(1328); c.m.w32(a_, (v_) as u32); };
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotChar_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftShot_SetCharPool(c: &mut Ctx, mut p_chr_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShot_SetCharPool", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_pool_2: i32 = 0;
    let r_ = 'f: {
    if (((((p_chr_1) == (0i32)) as i32)) != 0) {
        l_pool_2 = 4i32;
    } else {
        l_pool_2 = 5i32;
    }
    F::BtlPool_SetCurrent(c, l_pool_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShot_SetCharPool", &PROBE_N, &[], true); }
    r_
}

pub fn EftShot_ResetCharPool(c: &mut Ctx, mut p_chr_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShot_ResetCharPool", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_pool_2: i32 = 0;
    let r_ = 'f: {
    if (((((p_chr_1) == (0i32)) as i32)) != 0) {
        l_pool_2 = 4i32;
    } else {
        l_pool_2 = 5i32;
    }
    F::BtlPool_Reset(c, l_pool_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShot_ResetCharPool", &PROBE_N, &[], true); }
    r_
}

pub fn EftShot_InitSlot(c: &mut Ctx, mut p_chr_1: i32, mut p_slot_2: i32, mut p_pack_3: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShot_InitSlot", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32, (p_pack_3) as u32], false); }
    let mut l_c_4: u32 = 0;
    let mut l_s_5: u32 = 0;
    let mut l_def_6: u32 = 0;
    let r_ = 'f: {
    l_c_4 = ({ let a_ = (c.m.r32(sym::gEftShot)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((p_chr_1) as u32)).wrapping_mul(1344));
    l_s_5 = (l_c_4).wrapping_add((((p_slot_2) as u32)).wrapping_mul(80));
    { let _ = F::memset(c, l_s_5, 0i32, 80u32); };
    if (((((p_pack_3) == (((0i32) as u32))) as i32)) != 0) {
        l_def_6 = ((l_c_4).wrapping_add(480)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(140));
        { let v_ = l_def_6; let a_ = (l_s_5).wrapping_add(36); c.m.w32(a_, v_); };
        F::EftShot_BuildParam(c, p_chr_1, p_slot_2, l_def_6, 1i32);
        break 'f 0i32;
    }
    { let v_ = p_pack_3; let a_ = (l_s_5).wrapping_add(28); c.m.w32(a_, v_); };
    { let v_ = ((l_c_4).wrapping_add(480)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(140)); let a_ = (l_s_5).wrapping_add(36); c.m.w32(a_, v_); };
    F::EftShot_BuildParam(c, p_chr_1, p_slot_2, ((l_c_4).wrapping_add(480)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(140)), 0i32);
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShot_InitSlot", &PROBE_N, &[], true); }
    r_
}
