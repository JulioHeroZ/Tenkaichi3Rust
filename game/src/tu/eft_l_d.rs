#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EftSpdLine_SpawnBodyTrails(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_SpawnBodyTrails", &PROBE_N, &[(p_objId_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(128) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 128);
    let mut l_extra_2: F32 = F32(0);
    let mut l_i_3: u32 = 0;
    let mut l_j_4: i32 = 0;
    let mut l_angle_5: F32 = F32(0);
    let mut l_radius_6: F32 = F32(0);
    let mut l_len_7: F32 = F32(0);
    let r_ = 'f: {
        l_extra_2 = F32(0x00000000);
        { let s_ = sym::eft_l_d__EftSpdLine_SpawnBodyTrails__sUp; rt::memcpy(c, fp.wrapping_add(64), s_, 16); }
        if (((((c.m.r32(sym::gEftSpdLine)) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        F::BtlCharApi_GetDir(c, p_objId_1, fp.wrapping_add(48));
        F::Vec4_Scale(c, fp.wrapping_add(32), fp.wrapping_add(48), F32(0xbf800000));
        F::Vec3_Normalize(c, fp.wrapping_add(32), fp.wrapping_add(32));
        F::Vec3_Cross(c, fp.wrapping_add(96), fp.wrapping_add(32), fp.wrapping_add(64));
        F::Vec3_Normalize(c, fp.wrapping_add(96), fp.wrapping_add(96));
        F::Vec3_Cross(c, fp.wrapping_add(80), fp.wrapping_add(32), fp.wrapping_add(96));
        F::Vec3_Normalize(c, fp.wrapping_add(80), fp.wrapping_add(80));
        {
            l_i_3 = ((0i32) as u32);
            'l1: loop {
                if !(((((l_i_3) < (((11i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a1_ = (c.m.r32((sym::eft_l_d__EftSpdLine_SpawnBodyTrails__sNodes).wrapping_add((l_i_3).wrapping_mul(8))) as i32); F::BtlCharApi_GetNodePos(c, p_objId_1, a1_, fp.wrapping_add(16)) };
                    if ((((((c.m.r32(((sym::eft_l_d__EftSpdLine_SpawnBodyTrails__sNodes).wrapping_add((l_i_3).wrapping_mul(8))).wrapping_add(4)) as i32)) >= (0i32)) as i32)) != 0) {
                        { let a1_ = (c.m.r32(((sym::eft_l_d__EftSpdLine_SpawnBodyTrails__sNodes).wrapping_add((l_i_3).wrapping_mul(8))).wrapping_add(4)) as i32); F::BtlCharApi_GetNodePos(c, p_objId_1, a1_, fp.wrapping_add(112)) };
                        F::Vec4_Sub(c, fp.wrapping_add(112), fp.wrapping_add(112), fp.wrapping_add(16));
                        l_len_7 = F::Vec3_Length(c, fp.wrapping_add(112));
                        if (((rt::fgt(l_len_7, F32(0x40c00000)) as i32)) != 0) {
                            l_extra_2 = rt::fdiv(l_len_7, F32(0x40c00000));
                        }
                    }
                    l_angle_5 = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40c90fda));
                    l_radius_6 = rt::fadd(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x00000000));
                    { let a2_ = rt::fmul(rt::fmul(F::Mathf_Cos(c, l_angle_5), l_radius_6), rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc))); F::Vec4_Scale(c, fp.wrapping_add(48), fp.wrapping_add(96), a2_) };
                    F::Vec4_Add(c, fp, fp.wrapping_add(16), fp.wrapping_add(48));
                    { let a2_ = rt::fmul(rt::fmul(F::Mathf_Sin(c, l_angle_5), l_radius_6), rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc))); F::Vec4_Scale(c, fp.wrapping_add(48), fp.wrapping_add(80), a2_) };
                    F::Vec4_Add(c, fp, fp, fp.wrapping_add(48));
                    { let a2_ = rt::fmul(rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40a00000)), F32(0x3fc00000)), rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc))); F::Vec4_Scale(c, fp.wrapping_add(48), fp.wrapping_add(32), a2_) };
                    F::Vec4_Add(c, fp, fp, fp.wrapping_add(48));
                    F::EftSpdLine_AddTrail(c, p_objId_1, fp, fp.wrapping_add(32));
                    if (((rt::fgt(l_extra_2, F32(0x00000000)) as i32)) != 0) {
                        {
                            l_j_4 = 0i32;
                            'l3: loop {
                                if !(((((l_j_4) < (rt::f2i(l_extra_2))) as i32)) != 0) { break 'l3; }
                                'c4: {
                                    l_angle_5 = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40c90fda));
                                    l_radius_6 = rt::fadd(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x00000000));
                                    { let a2_ = rt::fmul(rt::fmul(F::Mathf_Cos(c, l_angle_5), l_radius_6), rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc))); F::Vec4_Scale(c, fp.wrapping_add(48), fp.wrapping_add(96), a2_) };
                                    F::Vec4_Add(c, fp, fp.wrapping_add(16), fp.wrapping_add(48));
                                    { let a2_ = rt::fmul(rt::fmul(F::Mathf_Sin(c, l_angle_5), l_radius_6), rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc))); F::Vec4_Scale(c, fp.wrapping_add(48), fp.wrapping_add(80), a2_) };
                                    F::Vec4_Add(c, fp, fp, fp.wrapping_add(48));
                                    { let a2_ = rt::fmul(rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40a00000)), F32(0x3fc00000)), rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc))); F::Vec4_Scale(c, fp.wrapping_add(48), fp.wrapping_add(32), a2_) };
                                    F::Vec4_Add(c, fp, fp, fp.wrapping_add(48));
                                    F::EftSpdLine_AddTrail(c, p_objId_1, fp, fp.wrapping_add(32));
                                }
                                { let o_ = l_j_4; let n_ = (o_).wrapping_add(1); l_j_4 = n_; };
                            }
                        }
                    }
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_SpawnBodyTrails", &PROBE_N, &[], true); }
    r_
}

pub fn EftSpdLine_SpawnPartStreaks(c: &mut Ctx, mut p_objId_1: i32, mut p_dir_2: u32, mut p_part_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_SpawnPartStreaks", &PROBE_N, &[(p_objId_1) as u32, (p_dir_2) as u32, (p_part_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let mut l_size_4: F32 = F32(0);
    let mut l_angle_5: F32 = F32(0);
    let mut l_radius_6: F32 = F32(0);
    let mut l_lead_7: F32 = F32(0);
    let mut l_width_8: F32 = F32(0);
    let mut l_i_9: i32 = 0;
    let r_ = 'f: {
        { let s_ = sym::eft_l_d__EftSpdLine_SpawnPartStreaks__sUp; rt::memcpy(c, fp.wrapping_add(64), s_, 16); }
        if (((((c.m.r32(sym::gEftSpdLine)) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        l_size_4 = rt::fmul(F::BtlCharApi_GetPartUnk5C(c, p_objId_1, p_part_3), F32(0x3ecccccc));
        if (((rt::flt(l_size_4, F32(0x3f4ccccc)) as i32)) != 0) {
            l_size_4 = rt::fmul(rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)), F32(0x3f4ccccc));
        }
        F::Vec4_Scale(c, fp.wrapping_add(32), p_dir_2, F32(0xbf800000));
        F::Vec3_Normalize(c, fp.wrapping_add(32), fp.wrapping_add(32));
        F::Vec3_Cross(c, fp.wrapping_add(96), fp.wrapping_add(32), fp.wrapping_add(64));
        F::Vec3_Normalize(c, fp.wrapping_add(96), fp.wrapping_add(96));
        F::Vec3_Cross(c, fp.wrapping_add(80), fp.wrapping_add(32), fp.wrapping_add(96));
        F::Vec3_Normalize(c, fp.wrapping_add(80), fp.wrapping_add(80));
        {
            l_i_9 = 0i32;
            'l1: loop {
                if !(((((l_i_9) < (15i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::BtlCharApi_GetNodePos(c, p_objId_1, p_part_3, fp);
                    l_angle_5 = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40c90fda));
                    l_radius_6 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3fe66666)), F32(0x3dcccccc));
                    l_lead_7 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40000000)), F32(0x3fc00000));
                    { let r_ = l_size_4; let n_ = rt::fmul(l_radius_6, r_); l_radius_6 = n_; };
                    { let r_ = l_size_4; let n_ = rt::fmul(l_lead_7, r_); l_lead_7 = n_; };
                    { let a2_ = rt::fmul(F::Mathf_Cos(c, l_angle_5), l_radius_6); F::Vec4_Scale(c, fp.wrapping_add(48), fp.wrapping_add(96), a2_) };
                    F::Vec4_Add(c, fp, fp, fp.wrapping_add(48));
                    { let a2_ = rt::fmul(F::Mathf_Sin(c, l_angle_5), l_radius_6); F::Vec4_Scale(c, fp.wrapping_add(48), fp.wrapping_add(80), a2_) };
                    F::Vec4_Add(c, fp, fp, fp.wrapping_add(48));
                    F::Vec4_Scale(c, fp.wrapping_add(48), fp.wrapping_add(32), l_lead_7);
                    F::Vec4_Sub(c, fp, fp, fp.wrapping_add(48));
                    { let a2_ = rt::fmul(rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40400000)), F32(0x40400000)), l_size_4); F::Vec4_Scale(c, fp.wrapping_add(48), fp.wrapping_add(32), a2_) };
                    F::Vec4_Add(c, fp.wrapping_add(16), fp, fp.wrapping_add(48));
                    l_width_8 = rt::fmul(rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40000000)), F32(0x3f800000)), rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)));
                    { let a5_ = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3db851eb)), F32(0x3d75c28f)); F::EftSpdLine_AddStreak(c, p_objId_1, fp, fp.wrapping_add(16), fp.wrapping_add(32), l_width_8, a5_, F32(0x3e4ccccc)) };
                }
                { let o_ = l_i_9; let n_ = (o_).wrapping_add(1); l_i_9 = n_; };
            }
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSpdLine_SpawnPartStreaks", &PROBE_N, &[], true); }
    r_
}
