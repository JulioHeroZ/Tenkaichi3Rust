#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EftGndDust_MoveNode(c: &mut Ctx, mut p_from_1: u32, mut p_to_2: u32, mut p_node_3: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_MoveNode", &PROBE_N, &[(p_from_1) as u32, (p_to_2) as u32, (p_node_3) as u32], false); }
    let mut l_next_4: u32 = 0;
    let r_ = 'f: {
    l_next_4 = ((0i32) as u32);
    if (((((p_node_3) != (((0i32) as u32))) as i32)) != 0) {
        l_next_4 = F::List_GetNext(c, p_node_3);
        { let _ = F::List_Remove(c, p_from_1, p_node_3); };
        F::List_PushBack(c, p_to_2, p_node_3);
    }
    break 'f l_next_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_MoveNode", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDust_GetTex(c: &mut Ctx, mut p_tex_1: u32, mut p_idx_2: i32) -> u64 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_GetTex", &PROBE_N, &[(p_tex_1) as u32, (p_idx_2) as u32], false); }
    let mut l_bit_3: i32 = 0;
    let r_ = 'f: {
    l_bit_3 = (1i32).wrapping_shl((p_idx_2) as u32);
    if ((((!(((((c.m.r32((p_tex_1).wrapping_add(132)) as i32)) & (l_bit_3))) != 0)) as i32)) != 0) {
        { let v_ = F::EftVram_AddTex(c, (p_tex_1).wrapping_add((((p_idx_2) as u32)).wrapping_mul(16)), 1i32, 0i32); let a_ = (p_tex_1).wrapping_add((((p_idx_2) as u32)).wrapping_mul(16)); c.m.w64(a_, v_); };
        { let r_ = l_bit_3; let a_ = (p_tex_1).wrapping_add(132); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    break 'f c.m.r64((p_tex_1).wrapping_add((((p_idx_2) as u32)).wrapping_mul(16)));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_GetTex", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDust_GetLightColors(c: &mut Ctx, mut p_colA_1: u32, mut p_colB_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_GetLightColors", &PROBE_N, &[(p_colA_1) as u32, (p_colB_2) as u32], false); }
    let mut l_src_3: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftGndDust)) != (((0i32) as u32))) as i32)) != 0) {
        l_src_3 = F::BtlStage_GetLightColors(c);
        if (((((l_src_3) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = c.m.r8((l_src_3).wrapping_add(((0i32) as u32))); let a_ = (p_colA_1).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = c.m.r8((l_src_3).wrapping_add(((1i32) as u32))); let a_ = (p_colA_1).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = c.m.r8((l_src_3).wrapping_add(((2i32) as u32))); let a_ = (p_colA_1).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = c.m.r8((l_src_3).wrapping_add(((3i32) as u32))); let a_ = (p_colA_1).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = c.m.r8((l_src_3).wrapping_add(((4i32) as u32))); let a_ = (p_colB_2).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = c.m.r8((l_src_3).wrapping_add(((5i32) as u32))); let a_ = (p_colB_2).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = c.m.r8((l_src_3).wrapping_add(((6i32) as u32))); let a_ = (p_colB_2).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = c.m.r8((l_src_3).wrapping_add(((7i32) as u32))); let a_ = (p_colB_2).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
        } else {
            { let v_ = ((128i32) as u8); let a_ = (p_colA_1).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = ((128i32) as u8); let a_ = (p_colA_1).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = ((128i32) as u8); let a_ = (p_colA_1).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = ((128i32) as u8); let a_ = (p_colA_1).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = ((128i32) as u8); let a_ = (p_colB_2).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = ((128i32) as u8); let a_ = (p_colB_2).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = ((128i32) as u8); let a_ = (p_colB_2).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = ((0i32) as u8); let a_ = (p_colB_2).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
        }
        { let r_ = 8i32; let a_ = (p_colA_1).wrapping_add(((3i32) as u32)); let n_ = (((((c.m.r8(a_)) as i32)).wrapping_add(r_)) as u8); c.m.w8(a_, n_); };
        { let r_ = 8i32; let a_ = (p_colB_2).wrapping_add(((3i32) as u32)); let n_ = (((((c.m.r8(a_)) as i32)).wrapping_add(r_)) as u8); c.m.w8(a_, n_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_GetLightColors", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDust_SpawnPiece(c: &mut Ctx, mut p_w_1: u32, mut p_def_2: u32, mut p_flags_3: i32, mut p_size_4: F32, mut p_rot_5: F32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_SpawnPiece", &PROBE_N, &[(p_w_1) as u32, (p_def_2) as u32, (p_flags_3) as u32, (p_size_4).0, (p_rot_5).0], false); }
    let mut l_p_6: u32 = 0;
    let mut l_spin_7: F32 = F32(0);
    let r_ = 'f: {
    l_p_6 = { let a1_ = c.m.r32((p_w_1).wrapping_add(200)); F::EftGndDust_AllocPart(c, (p_w_1).wrapping_add(204), a1_) };
    if (((((l_p_6) != (((0i32) as u32))) as i32)) != 0) {
        F::Vec4_Copy(c, (l_p_6).wrapping_add(16), p_def_2);
        F::Vec4_Copy(c, (l_p_6).wrapping_add(48), (p_def_2).wrapping_add(16));
        F::Vec4_Set(c, (l_p_6).wrapping_add(64), F32(0x43000000), F32(0x43000000), F32(0x43000000), F32(0x43000000));
        { let a2_ = rt::fadd(rt::fmul(F32(c.m.r32((p_def_2).wrapping_add(92))), F32(c.m.r32((p_def_2).wrapping_add(96)))), F32(c.m.r32((p_def_2).wrapping_add(84)))); F::Vec3_Scale(c, (l_p_6).wrapping_add(128), (p_def_2).wrapping_add(32), a2_) };
        F::Vec3_Normalize(c, (l_p_6).wrapping_add(48), (l_p_6).wrapping_add(48));
        F::Vec4_Set(c, (l_p_6).wrapping_add(96), p_size_4, p_size_4, F32(0x3f800000), F32(0x3f800000));
        { let a1_ = F32(c.m.r32((p_def_2).wrapping_add(72))); let a2_ = F32(c.m.r32((p_def_2).wrapping_add(72))); F::Vec4_Set(c, (l_p_6).wrapping_add(112), a1_, a2_, F32(0x3f800000), F32(0x3f800000)) };
        { let s_ = (p_def_2).wrapping_add(48); let d_ = (l_p_6).wrapping_add(144); rt::memcpy(c, d_, s_, 4); };
        { let s_ = (p_def_2).wrapping_add(52); let d_ = (l_p_6).wrapping_add(148); rt::memcpy(c, d_, s_, 4); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32((p_def_2).wrapping_add(80))), rt::fmul(F32(c.m.r32((p_def_2).wrapping_add(84))), F::Rand_Float01(c))), F32(c.m.r32((p_def_2).wrapping_add(96)))); let a_ = (l_p_6).wrapping_add(172); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((p_def_2).wrapping_add(88))); let a_ = (l_p_6).wrapping_add(176); c.m.w32(a_, (v_).0); };
        { let v_ = { let v_ = ((rt::f2i(rt::i2f(((c.m.r16((p_def_2).wrapping_add(62)) as i16)) as i32))) as i16); let a_ = (l_p_6).wrapping_add(152); c.m.w16(a_, (v_) as u16); v_ }; let a_ = (l_p_6).wrapping_add(154); c.m.w16(a_, (v_) as u16); };
        { let v_ = { let v_ = ((rt::f2i(rt::i2f(((c.m.r16((p_def_2).wrapping_add(64)) as i16)) as i32))) as i16); let a_ = (l_p_6).wrapping_add(158); c.m.w16(a_, (v_) as u16); v_ }; let a_ = (l_p_6).wrapping_add(156); c.m.w16(a_, (v_) as u16); };
        { let v_ = F32(0x3f800000); let a_ = (l_p_6).wrapping_add(180); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (l_p_6).wrapping_add(184); c.m.w32(a_, (v_).0); };
        { let v_ = p_rot_5; let a_ = (l_p_6).wrapping_add(164); c.m.w32(a_, (v_).0); };
        { let v_ = c.m.r16((p_def_2).wrapping_add(60)); let a_ = (l_p_6).wrapping_add(160); c.m.w16(a_, v_); };
        l_spin_7 = rt::fmul(rt::i2f(((c.m.r16((p_def_2).wrapping_add(66)) as i16)) as i32), F32(0x3dcccccc));
        { let v_ = l_spin_7; let a_ = (l_p_6).wrapping_add(168); c.m.w32(a_, (v_).0); };
        { let v_ = (if (((rt::fgt(F::Rand_Float01(c), F32(0x3f000000)) as i32)) != 0) { rt::fneg(l_spin_7) } else { l_spin_7 }); let a_ = (l_p_6).wrapping_add(168); c.m.w32(a_, (v_).0); };
        { let a2_ = F32(c.m.r32((l_p_6).wrapping_add(172))); F::Vec3_Scale(c, (l_p_6).wrapping_add(80), (l_p_6).wrapping_add(48), a2_) };
        { let v_ = 2i32; let a_ = (l_p_6).wrapping_add(188); c.m.w32(a_, (v_) as u32); };
        if (((((p_flags_3) != (0i32)) as i32)) != 0) {
            { let v_ = ((p_flags_3) | (2i32)); let a_ = (l_p_6).wrapping_add(188); c.m.w32(a_, (v_) as u32); };
        }
        { let a_ = (c.m.r32(sym::gEftGndDust)).wrapping_add(25268); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 'f l_p_6;
    }
    break 'f l_p_6;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_SpawnPiece", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDust_SpawnPieceEx(c: &mut Ctx, mut p_w_1: u32, mut p_pos_2: u32, mut p_dir_3: u32, mut p_colA_4: u32, mut p_colB_5: u32, mut p_sizeX_6: F32, mut p_sizeY_7: F32, mut p_growX_8: F32, mut p_growY_9: F32, mut p_speed_10: F32, mut p_drag_11: F32, mut p_rot_12: F32, mut p_spin_13: F32, mut p_life_14: i16, mut p_fade_15: i16, mut p_tex_16: i16, mut p_unkB8_17: F32, mut p_gravity_18: F32, mut p_flags_19: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_SpawnPieceEx", &PROBE_N, &[(p_w_1) as u32, (p_pos_2) as u32, (p_dir_3) as u32, (p_colA_4) as u32, (p_colB_5) as u32, (p_sizeX_6).0, (p_sizeY_7).0, (p_growX_8).0, (p_growY_9).0, (p_speed_10).0, (p_drag_11).0, (p_rot_12).0, (p_spin_13).0, (p_life_14) as u32, (p_fade_15) as u32, (p_tex_16) as u32, (p_unkB8_17).0, (p_gravity_18).0, (p_flags_19) as u32], false); }
    let mut l_p_20: u32 = 0;
    let r_ = 'f: {
    l_p_20 = { let a1_ = c.m.r32((p_w_1).wrapping_add(200)); F::EftGndDust_AllocPart(c, (p_w_1).wrapping_add(204), a1_) };
    if (((((l_p_20) != (((0i32) as u32))) as i32)) != 0) {
        F::Vec4_Copy(c, (l_p_20).wrapping_add(16), p_pos_2);
        F::Vec4_Copy(c, (l_p_20).wrapping_add(48), p_dir_3);
        F::Vec4_Set(c, (l_p_20).wrapping_add(64), F32(0x43000000), F32(0x43000000), F32(0x43000000), F32(0x43000000));
        F::Vec4_Set(c, (l_p_20).wrapping_add(128), F32(0x00000000), rt::fneg(p_gravity_18), F32(0x00000000), F32(0x3f800000));
        F::Vec3_Normalize(c, (l_p_20).wrapping_add(48), (l_p_20).wrapping_add(48));
        F::Vec4_Set(c, (l_p_20).wrapping_add(96), p_sizeX_6, p_sizeY_7, F32(0x3f800000), F32(0x3f800000));
        F::Vec4_Set(c, (l_p_20).wrapping_add(112), p_growX_8, p_growY_9, F32(0x3f800000), F32(0x3f800000));
        { let s_ = p_colA_4; let d_ = (l_p_20).wrapping_add(144); rt::memcpy(c, d_, s_, 4); };
        { let s_ = p_colB_5; let d_ = (l_p_20).wrapping_add(148); rt::memcpy(c, d_, s_, 4); };
        { let v_ = p_speed_10; let a_ = (l_p_20).wrapping_add(172); c.m.w32(a_, (v_).0); };
        { let v_ = p_drag_11; let a_ = (l_p_20).wrapping_add(176); c.m.w32(a_, (v_).0); };
        { let v_ = p_life_14; let a_ = (l_p_20).wrapping_add(152); c.m.w16(a_, (v_) as u16); };
        { let v_ = p_fade_15; let a_ = (l_p_20).wrapping_add(158); c.m.w16(a_, (v_) as u16); };
        { let v_ = p_life_14; let a_ = (l_p_20).wrapping_add(154); c.m.w16(a_, (v_) as u16); };
        { let v_ = p_fade_15; let a_ = (l_p_20).wrapping_add(156); c.m.w16(a_, (v_) as u16); };
        { let v_ = F32(0x3f800000); let a_ = (l_p_20).wrapping_add(180); c.m.w32(a_, (v_).0); };
        { let v_ = p_unkB8_17; let a_ = (l_p_20).wrapping_add(184); c.m.w32(a_, (v_).0); };
        { let v_ = p_rot_12; let a_ = (l_p_20).wrapping_add(164); c.m.w32(a_, (v_).0); };
        { let v_ = ((p_tex_16) as u16); let a_ = (l_p_20).wrapping_add(160); c.m.w16(a_, v_); };
        { let v_ = p_spin_13; let a_ = (l_p_20).wrapping_add(168); c.m.w32(a_, (v_).0); };
        { let v_ = (if (((rt::fgt(F::Rand_Float01(c), F32(0x3f000000)) as i32)) != 0) { rt::fneg(p_spin_13) } else { p_spin_13 }); let a_ = (l_p_20).wrapping_add(168); c.m.w32(a_, (v_).0); };
        { let a2_ = F32(c.m.r32((l_p_20).wrapping_add(172))); F::Vec3_Scale(c, (l_p_20).wrapping_add(80), (l_p_20).wrapping_add(48), a2_) };
        { let v_ = 2i32; let a_ = (l_p_20).wrapping_add(188); c.m.w32(a_, (v_) as u32); };
        if (((((p_flags_19) != (0i32)) as i32)) != 0) {
            { let v_ = ((p_flags_19) | (2i32)); let a_ = (l_p_20).wrapping_add(188); c.m.w32(a_, (v_) as u32); };
        }
        { let a_ = (c.m.r32(sym::gEftGndDust)).wrapping_add(25268); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 'f l_p_20;
    }
    break 'f l_p_20;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_SpawnPieceEx", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDust_SpawnBodyDust(c: &mut Ctx, mut p_w_1: u32, mut p_def_2: u32, mut p_scale_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_SpawnBodyDust", &PROBE_N, &[(p_w_1) as u32, (p_def_2) as u32, (p_scale_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(256) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 256);
    let mut l_h_4: F32 = F32(0);
    let mut l_i_5: i32 = 0;
    let mut l_j_6: i32 = 0;
    let mut l_n_7: i32 = 0;
    let mut l_len_8: F32 = F32(0);
    let mut l_extra_9: F32 = F32(0);
    let mut l_life_10: F32 = F32(0);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 120);
        rt::memset(c, fp.wrapping_add(0), 0, 12);
        { let v_ = 3i32; c.m.w32(fp.wrapping_add(0).wrapping_add(0), (v_) as u32); }
        { let v_ = (1i32).wrapping_neg(); c.m.w32(fp.wrapping_add(0).wrapping_add(4), (v_) as u32); }
        { let v_ = 0i32; c.m.w32(fp.wrapping_add(0).wrapping_add(8), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(12), 0, 12);
        { let v_ = 11i32; c.m.w32(fp.wrapping_add(12).wrapping_add(0), (v_) as u32); }
        { let v_ = 8i32; c.m.w32(fp.wrapping_add(12).wrapping_add(4), (v_) as u32); }
        { let v_ = 0i32; c.m.w32(fp.wrapping_add(12).wrapping_add(8), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(24), 0, 12);
        { let v_ = 15i32; c.m.w32(fp.wrapping_add(24).wrapping_add(0), (v_) as u32); }
        { let v_ = 12i32; c.m.w32(fp.wrapping_add(24).wrapping_add(4), (v_) as u32); }
        { let v_ = 0i32; c.m.w32(fp.wrapping_add(24).wrapping_add(8), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(36), 0, 12);
        { let v_ = 21i32; c.m.w32(fp.wrapping_add(36).wrapping_add(0), (v_) as u32); }
        { let v_ = (1i32).wrapping_neg(); c.m.w32(fp.wrapping_add(36).wrapping_add(4), (v_) as u32); }
        { let v_ = 0i32; c.m.w32(fp.wrapping_add(36).wrapping_add(8), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(48), 0, 12);
        { let v_ = 35i32; c.m.w32(fp.wrapping_add(48).wrapping_add(0), (v_) as u32); }
        { let v_ = (1i32).wrapping_neg(); c.m.w32(fp.wrapping_add(48).wrapping_add(4), (v_) as u32); }
        { let v_ = 0i32; c.m.w32(fp.wrapping_add(48).wrapping_add(8), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(60), 0, 12);
        { let v_ = 18i32; c.m.w32(fp.wrapping_add(60).wrapping_add(0), (v_) as u32); }
        { let v_ = 21i32; c.m.w32(fp.wrapping_add(60).wrapping_add(4), (v_) as u32); }
        { let v_ = 1i32; c.m.w32(fp.wrapping_add(60).wrapping_add(8), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(72), 0, 12);
        { let v_ = 32i32; c.m.w32(fp.wrapping_add(72).wrapping_add(0), (v_) as u32); }
        { let v_ = 35i32; c.m.w32(fp.wrapping_add(72).wrapping_add(4), (v_) as u32); }
        { let v_ = 1i32; c.m.w32(fp.wrapping_add(72).wrapping_add(8), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(84), 0, 12);
        { let v_ = 8i32; c.m.w32(fp.wrapping_add(84).wrapping_add(0), (v_) as u32); }
        { let v_ = 18i32; c.m.w32(fp.wrapping_add(84).wrapping_add(4), (v_) as u32); }
        { let v_ = 1i32; c.m.w32(fp.wrapping_add(84).wrapping_add(8), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(96), 0, 12);
        { let v_ = 12i32; c.m.w32(fp.wrapping_add(96).wrapping_add(0), (v_) as u32); }
        { let v_ = 32i32; c.m.w32(fp.wrapping_add(96).wrapping_add(4), (v_) as u32); }
        { let v_ = 1i32; c.m.w32(fp.wrapping_add(96).wrapping_add(8), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(108), 0, 12);
        { let v_ = 48i32; c.m.w32(fp.wrapping_add(108).wrapping_add(0), (v_) as u32); }
        { let v_ = (1i32).wrapping_neg(); c.m.w32(fp.wrapping_add(108).wrapping_add(4), (v_) as u32); }
        { let v_ = 0i32; c.m.w32(fp.wrapping_add(108).wrapping_add(8), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(192), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(192).wrapping_add(0), (v_).0); }
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(192).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(192).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(192).wrapping_add(12), (v_).0); }
        l_h_4 = rt::fmul({ let a0_ = (((c.m.r8((p_def_2).wrapping_add(56)) as i8)) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x3e800000));
        if (((rt::flt(l_h_4, F32(0x40c00000)) as i32)) != 0) {
            l_h_4 = F32(0x40c00000);
        }
        { let a0_ = (((c.m.r8((p_def_2).wrapping_add(56)) as i8)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, fp.wrapping_add(176)) };
        {
            l_i_5 = 0i32;
            'l1: loop {
                if !(((((l_i_5) < (10i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a0_ = (((c.m.r8((p_def_2).wrapping_add(56)) as i8)) as i32); let a1_ = (c.m.r32((fp).wrapping_add((((l_i_5) as u32)).wrapping_mul(12))) as i32); F::BtlCharApi_GetNodePos(c, a0_, a1_, fp.wrapping_add(128)) };
                    F::Vec4_Copy(c, fp.wrapping_add(144), fp.wrapping_add(128));
                    if ((((((c.m.r32(((fp).wrapping_add((((l_i_5) as u32)).wrapping_mul(12))).wrapping_add(4)) as i32)) >= (0i32)) as i32)) != 0) {
                        { let a0_ = (((c.m.r8((p_def_2).wrapping_add(56)) as i8)) as i32); let a1_ = (c.m.r32(((fp).wrapping_add((((l_i_5) as u32)).wrapping_mul(12))).wrapping_add(4)) as i32); F::BtlCharApi_GetNodePos(c, a0_, a1_, fp.wrapping_add(208)) };
                        F::Vec4_Sub(c, fp.wrapping_add(208), fp.wrapping_add(208), fp.wrapping_add(128));
                        l_len_8 = F::Vec3_Length(c, fp.wrapping_add(208));
                        if (((rt::flt(l_h_4, l_len_8) as i32)) != 0) {
                            l_n_7 = rt::f2i(rt::fadd(rt::fdiv(l_len_8, l_h_4), F32(0x3f800000)));
                            F::Vec3_Scale(c, fp.wrapping_add(160), fp.wrapping_add(208), rt::fdiv(F32(0x3f800000), rt::i2f(l_n_7)));
                        } else {
                            F::Vec4_Set(c, fp.wrapping_add(160), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x3f800000));
                            l_n_7 = 1i32;
                            if ((((((c.m.r32(((fp).wrapping_add((((l_i_5) as u32)).wrapping_mul(12))).wrapping_add(8)) as i32)) != (0i32)) as i32)) != 0) {
                                l_n_7 = 0i32;
                            }
                        }
                    } else {
                        F::Vec4_Set(c, fp.wrapping_add(160), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x3f800000));
                        l_n_7 = 1i32;
                    }
                    {
                        l_j_6 = 0i32;
                        'l3: loop {
                            if !(((((l_j_6) < (l_n_7)) as i32)) != 0) { break 'l3; }
                            'c4: {
                                { let _ = F::Port_Rand(c); };
                                l_extra_9 = rt::fmul(rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3fc00000)), l_h_4), p_scale_3);
                                { let _ = F::Port_Rand(c); };
                                l_life_10 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3f000000)), F32(0x3f333333));
                                { let v_ = rt::fadd(F32(c.m.r32(fp.wrapping_add(144))), rt::fmul(rt::i2f(l_j_6), F32(c.m.r32(fp.wrapping_add(160))))); let a_ = fp.wrapping_add(128); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fadd(F32(c.m.r32((fp.wrapping_add(144)).wrapping_add(4))), rt::fmul(rt::i2f(l_j_6), F32(c.m.r32((fp.wrapping_add(160)).wrapping_add(4))))); let a_ = (fp.wrapping_add(128)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fadd(F32(c.m.r32((fp.wrapping_add(144)).wrapping_add(8))), rt::fmul(rt::i2f(l_j_6), F32(c.m.r32((fp.wrapping_add(160)).wrapping_add(8))))); let a_ = (fp.wrapping_add(128)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                F::Vec3_Sub(c, fp.wrapping_add(192), fp.wrapping_add(128), fp.wrapping_add(176));
                                F::Vec3_Normalize(c, fp.wrapping_add(192), fp.wrapping_add(192));
                                F::Vec4_Set(c, fp.wrapping_add(224), F32(0x43000000), F32(0x43000000), F32(0x43000000), F32(0x00000000));
                                { let a4_ = rt::i2f((128i32).wrapping_sub(rt::rem_i32(F::Port_Rand(c), 32i32))); F::Vec4_Set(c, fp.wrapping_add(240), F32(0x43000000), F32(0x43000000), F32(0x43000000), a4_) };
                                { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(192)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(128)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                F::Vec4_Copy(c, p_def_2, fp.wrapping_add(128));
                                F::Vec4_Copy(c, (p_def_2).wrapping_add(16), fp.wrapping_add(192));
                                F::Vec4_Set(c, (p_def_2).wrapping_add(32), F32(0x00000000), F32(0xbf800000), F32(0x00000000), F32(0x3f800000));
                                { let v_ = ((rt::f2i(rt::fmul(l_life_10, F32(0x41f00000)))) as i16); let a_ = (p_def_2).wrapping_add(62); c.m.w16(a_, (v_) as u16); };
                                { let v_ = ((5i32) as i16); let a_ = (p_def_2).wrapping_add(64); c.m.w16(a_, (v_) as u16); };
                                { let v_ = F32(0x3dcccccc); let a_ = (p_def_2).wrapping_add(72); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f4ccccc); let a_ = (p_def_2).wrapping_add(88); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3c23d70a); let a_ = (p_def_2).wrapping_add(92); c.m.w32(a_, (v_).0); };
                                { let v_ = ((0i32) as i16); let a_ = (p_def_2).wrapping_add(66); c.m.w16(a_, (v_) as u16); };
                                { let v_ = F32(0x3fc00000); let a_ = (p_def_2).wrapping_add(80); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x00000000); let a_ = (p_def_2).wrapping_add(84); c.m.w32(a_, (v_).0); };
                                { let _ = { let a4_ = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40c90fda)); F::EftGndDust_SpawnPiece(c, p_w_1, p_def_2, 0i32, l_extra_9, a4_) }; };
                            }
                            { let o_ = l_j_6; let n_ = (o_).wrapping_add(1); l_j_6 = n_; };
                        }
                    }
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_SpawnBodyDust", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDust_SpawnChip(c: &mut Ctx, mut p_w_1: u32, mut p_pos_2: u32, mut p_dir_3: u32, mut p_colA_4: u32, mut p_colB_5: u32, mut p_life_6: i32, mut p_fade_7: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_SpawnChip", &PROBE_N, &[(p_w_1) as u32, (p_pos_2) as u32, (p_dir_3) as u32, (p_colA_4) as u32, (p_colB_5) as u32, (p_life_6) as u32, (p_fade_7) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let r_ = 'f: {
        rt::memset(c, fp.wrapping_add(48), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(48).wrapping_add(0), (v_).0); }
        rt::memset(c, fp.wrapping_add(64), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(0), (v_).0); }
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(64).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(64).wrapping_add(12), (v_).0); }
        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec3_Normalize(c, fp.wrapping_add(32), p_dir_3);
        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let a2_ = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40a00000)), F32(0x40800000)); F::Vec4_Scale(c, fp, fp.wrapping_add(32), a2_) };
        F::Vec4_Add(c, fp.wrapping_add(16), p_pos_2, fp);
        { let r_ = F32(0x40e00000); let a_ = (fp.wrapping_add(16)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        F::Vec3_Lerp(c, fp.wrapping_add(48), fp.wrapping_add(64), fp.wrapping_add(32), F32(0x3f000000));
        { let _ = F::EftGndDust_SpawnPieceEx(c, p_w_1, fp.wrapping_add(16), fp.wrapping_add(48), p_colA_4, p_colB_5, F32(0x3f4ccccc), F32(0x3f800000), F32(0x00000000), F32(0x3e4ccccc), F32(0x00000000), F32(0x3f800000), F32(0x00000000), F32(0x00000000), ((rt::f2i(rt::i2f(p_life_6))) as i16), ((rt::f2i(rt::i2f(p_fade_7))) as i16), ((10i32) as i16), F32(0x3f800000), F32(0x00000000), 80i32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_SpawnChip", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDust_MakeFacingMtx(c: &mut Ctx, mut p_out_1: u32, mut p_dir_2: u32, mut p_pos_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_MakeFacingMtx", &PROBE_N, &[(p_out_1) as u32, (p_dir_2) as u32, (p_pos_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_d_4: F32 = F32(0);
    let mut l_half_5: F32 = F32(0);
    let r_ = 'f: {
        l_half_5 = F32(0x00000000);
        { let v_ = { let a_ = (((c.m.r32(sym::gBtlCamView)).wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = fp.wrapping_add(64); c.m.w32(a_, (v_).0); };
        { let v_ = { let a_ = (((c.m.r32(sym::gBtlCamView)).wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = (fp.wrapping_add(64)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a_ = (((c.m.r32(sym::gBtlCamView)).wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = (fp.wrapping_add(64)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(64)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        l_d_4 = F::Vec3_Dot(c, fp.wrapping_add(64), p_dir_2);
        if (((rt::flt(l_d_4, F32(0x00000000)) as i32)) != 0) {
            l_d_4 = rt::fneg(l_d_4);
            { let v_ = rt::fneg(F32(c.m.r32(fp.wrapping_add(64)))); let a_ = fp; c.m.w32(a_, (v_).0); };
            { let v_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(4)))); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(8)))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
            l_half_5 = rt::fmul(l_d_4, F32(0x3f000000));
        } else {
            l_half_5 = rt::fmul(l_d_4, F32(0x3f000000));
            { let v_ = F32(c.m.r32(fp.wrapping_add(64))); let a_ = fp; c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(4))); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(8))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        }
        F::Vec4_Sub(c, fp, p_dir_2, fp);
        F::Vec3_Normalize(c, fp, fp);
        F::Vec4_Scale(c, fp, fp, l_half_5);
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
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_MakeFacingMtx", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDust_IsOnScreen(c: &mut Ctx, mut p_p_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_IsOnScreen", &PROBE_N, &[(p_p_1) as u32], false); }
    let mut l_r_2: i32 = 0;
    let r_ = 'f: {
    if ((((((c.m.r32((p_p_1).wrapping_add(8)) as i32)) <= (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((c.m.r32(p_p_1) as i32)) > (65519i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((c.m.r32(p_p_1) as i32)) <= (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_r_2 = ((((c.m.r32((p_p_1).wrapping_add(4)) as i32)) > (0i32)) as i32);
    if ((((((c.m.r32((p_p_1).wrapping_add(4)) as i32)) > (65519i32)) as i32)) != 0) {
        l_r_2 = 0i32;
    }
    break 'f l_r_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_IsOnScreen", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDust_DrawQuad(c: &mut Ctx, mut p_pos_1: u32, mut p_color_2: u32, mut p_mtx_3: u32, mut p_tex0_4: u64, mut p_scale_5: F32, mut p_z0_6: F32, mut p_z1_7: F32, mut p_width_8: F32, mut p_u0_9: F32, mut p_v0_10: F32, mut p_u1_11: F32, mut p_v1_12: F32, mut p_layer_13: u8) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_DrawQuad", &PROBE_N, &[(p_pos_1) as u32, (p_color_2) as u32, (p_mtx_3) as u32, (p_tex0_4) as u32, (p_scale_5).0, (p_z0_6).0, (p_z1_7).0, (p_width_8).0, (p_u0_9).0, (p_v0_10).0, (p_u1_11).0, (p_v1_12).0, (p_layer_13) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(256) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 256);
    let mut l_p_14: u32 = 0;
    let mut l_e_15: u32 = 0;
    let mut l_hw_16: F32 = F32(0);
    let mut l_n_17: F32 = F32(0);
    let mut l_f_18: F32 = F32(0);
    let mut l_i_19: i32 = 0;
    let mut l_l_20: i32 = 0;
    let mut l_z_21: i32 = 0;
    let r_ = 'f: {
        if ((F::EftBurst_IsBusy(c)) != 0) {
            break 'f;
        }
        if (((rt::fle(F32(c.m.r32((p_color_2).wrapping_add(12))), F32(0x00000000)) as i32)) != 0) {
            break 'f;
        }
        l_hw_16 = rt::fmul(rt::fmul(p_scale_5, p_width_8), F32(0x3f000000));
        l_n_17 = rt::fmul(p_scale_5, p_z0_6);
        l_f_18 = rt::fmul(p_scale_5, p_z1_7);
        F::Vec4_Set(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), rt::fneg(l_hw_16), F32(0x00000000), l_n_17, F32(0x3f800000));
        F::Vec4_Set(c, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), l_hw_16, F32(0x00000000), l_n_17, F32(0x3f800000));
        F::Vec4_Set(c, (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)), rt::fneg(l_hw_16), F32(0x00000000), l_f_18, F32(0x3f800000));
        F::Vec4_Set(c, (fp).wrapping_add((((3i32) as u32)).wrapping_mul(16)), l_hw_16, F32(0x00000000), l_f_18, F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), p_u0_9, p_v0_10, F32(0x3f800000), F32(0x00000000));
        F::Vec4_Set(c, (fp.wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), p_u1_11, p_v0_10, F32(0x3f800000), F32(0x00000000));
        F::Vec4_Set(c, (fp.wrapping_add(128)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), p_u0_9, p_v1_12, F32(0x3f800000), F32(0x00000000));
        F::Vec4_Set(c, (fp.wrapping_add(128)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), p_u1_11, p_v1_12, F32(0x3f800000), F32(0x00000000));
        {
            l_i_19 = 0i32;
            'l1: loop {
                if !(((((l_i_19) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::Mtx_MulVec4(c, (fp).wrapping_add((((l_i_19) as u32)).wrapping_mul(16)), p_mtx_3, (fp).wrapping_add((((l_i_19) as u32)).wrapping_mul(16)));
                    F::Vec4_Add(c, (fp).wrapping_add((((l_i_19) as u32)).wrapping_mul(16)), (fp).wrapping_add((((l_i_19) as u32)).wrapping_mul(16)), p_pos_1);
                    { let v_ = F32(0x3f800000); let a_ = ((fp).wrapping_add((((l_i_19) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_19; let n_ = (o_).wrapping_add(1); l_i_19 = n_; };
            }
        }
        { let _ = F::Vu0Cur_ProjectPointsStq(c, fp.wrapping_add(192), fp.wrapping_add(64), fp, fp.wrapping_add(128), 4i32); };
        {
            l_i_19 = 0i32;
            'l3: loop {
                if !(((((l_i_19) < (4i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    if ((((!((F::EftGndDust_IsOnScreen(c, (fp.wrapping_add(192)).wrapping_add((((l_i_19) as u32)).wrapping_mul(16)))) != 0)) as i32)) != 0) {
                        break 'f;
                    }
                }
                { let o_ = l_i_19; let n_ = (o_).wrapping_add(1); l_i_19 = n_; };
            }
        }
        l_p_14 = c.m.r32(sym::gOtCur);
        { let v_ = (l_p_14).wrapping_add((((1i32) as u32)).wrapping_mul(144)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
        if (((((l_p_14) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        { let v_ = ((536870920i32) as u32); let a_ = l_p_14; c.m.w32(a_, v_); };
        { let v_ = ((1342177288i32) as u32); let a_ = (l_p_14).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = ((92i32) as u64); let a_ = (l_p_14).wrapping_add(32); c.m.w64(a_, v_); };
        { let v_ = ((268435456i32) as u32); let a_ = (l_p_14).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = 16429131440647602177u64; let a_ = (l_p_14).wrapping_add(16); c.m.w64(a_, v_); };
        { let v_ = ((18599481537995104i64) as u64); let a_ = (l_p_14).wrapping_add(24); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (l_p_14).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32(p_color_2)))) as u8); let a_ = ((l_p_14).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_color_2).wrapping_add(4))))) as u8); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_color_2).wrapping_add(8))))) as u8); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_color_2).wrapping_add(12))))) as u8); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = ((rt::f2i(F32(c.m.r32(p_color_2)))) as u8); let a_ = ((l_p_14).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_color_2).wrapping_add(4))))) as u8); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_color_2).wrapping_add(8))))) as u8); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_color_2).wrapping_add(12))))) as u8); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = ((rt::f2i(F32(c.m.r32(p_color_2)))) as u8); let a_ = ((l_p_14).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_color_2).wrapping_add(4))))) as u8); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_color_2).wrapping_add(8))))) as u8); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_color_2).wrapping_add(12))))) as u8); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = ((rt::f2i(F32(c.m.r32(p_color_2)))) as u8); let a_ = ((l_p_14).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_color_2).wrapping_add(4))))) as u8); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_color_2).wrapping_add(8))))) as u8); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_color_2).wrapping_add(12))))) as u8); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
        l_l_20 = ((p_layer_13) as i32);
        if (((((l_l_20) >= (2i32)) as i32)) != 0) {
            { let r_ = 2i32; let n_ = (l_l_20).wrapping_sub(r_); l_l_20 = n_; };
        }
        { let v_ = F32(c.m.r32(((fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16)))); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = ((((l_p_14).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16)))); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = ((((l_p_14).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16)))); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = ((((l_p_14).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16)))); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = ((((l_p_14).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = (((c.m.r32((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_14).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_14).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_14).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_14).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_14).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_14).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_14).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_14).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_14).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32((fp.wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_14).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_14).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_14).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_14).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        l_z_21 = (((((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)).wrapping_add((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_shr((10i32) as u32);
        { let v_ = p_tex0_4; let a_ = (l_p_14).wrapping_add(40); c.m.w64(a_, v_); };
        if (((((l_z_21) < (0i32)) as i32)) != 0) {
            l_e_15 = ((c.m.r32(sym::gOtZ)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_20) as u32)).wrapping_mul(8));
        } else {
            if (((((l_z_21) >= (4096i32)) as i32)) != 0) {
                l_e_15 = ((c.m.r32(sym::gOtZ)).wrapping_add((((4095i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_20) as u32)).wrapping_mul(8));
            } else {
                l_e_15 = ((c.m.r32(sym::gOtZ)).wrapping_add((((l_z_21) as u32)).wrapping_mul(16))).wrapping_add((((l_l_20) as u32)).wrapping_mul(8));
            }
        }
        { let v_ = l_p_14; let a_ = (c.m.r32((l_e_15).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = l_p_14; let a_ = (l_e_15).wrapping_add(4); c.m.w32(a_, v_); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_DrawQuad", &PROBE_N, &[], true); }
    r_
}

pub fn EftGndDust_DrawPiece(c: &mut Ctx, mut p_pos_1: u32, mut p_color_2: u32, mut p_dir_3: u32, mut p_layer_4: i32, mut p_tex0_5: u64, mut p_along_6: i32, mut p_w_7: F32, mut p_h_8: F32, mut p_rot_9: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_DrawPiece", &PROBE_N, &[(p_pos_1) as u32, (p_color_2) as u32, (p_dir_3) as u32, (p_layer_4) as u32, (p_tex0_5) as u32, (p_along_6) as u32, (p_w_7).0, (p_h_8).0, (p_rot_9).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let r_ = 'f: {
        if (((((p_along_6) != (0i32)) as i32)) != 0) {
            F::EftGndDust_MakeFacingMtx(c, fp, p_dir_3, sym::D_002EC2C0);
            F::EftGndDust_DrawQuad(c, p_pos_1, p_color_2, fp, p_tex0_5, F32(0x3f800000), p_h_8, F32(0x00000000), p_w_7, F32(0x00000000), F32(0x00000000), F32(0x3f800000), F32(0x3f800000), ((p_layer_4) as u8));
        } else {
            F::EftGfx_DrawSprite(c, p_pos_1, p_color_2, p_w_7, p_h_8, F32(0x00000000), F32(0x00000000), F32(0x3f800000), F32(0x3f800000), p_rot_9, p_layer_4, 0i32, p_tex0_5);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGndDust_DrawPiece", &PROBE_N, &[], true); }
    r_
}

pub fn EftDelaySe_Start(c: &mut Ctx, mut p_objId_1: i32, mut p_def_2: u32, mut p_count_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDelaySe_Start", &PROBE_N, &[(p_objId_1) as u32, (p_def_2) as u32, (p_count_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < (p_count_3)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let _ = F::memset(c, (fp).wrapping_add(4), 0i32, 12u32); };
                    { let v_ = p_objId_1; let a_ = fp; c.m.w32(a_, (v_) as u32); };
                    { let s_ = (p_def_2).wrapping_add((((l_i_4) as u32)).wrapping_mul(8)); let d_ = (fp).wrapping_add(4); rt::memcpy(c, d_, s_, 8); };
                    { let _ = { let a0_ = { let a_ = c.m.r32(sym::gEftDelaySe); c.m.r32(a_) }; F::BtlTaskList_AddTail(c, a0_, sym::gEftDelaySeClass, fp) }; };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftDelaySe_Start", &PROBE_N, &[], true); }
    r_
}

pub fn EftDelaySeMgr_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDelaySeMgr_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((4u32) as i32)) }; let a_ = sym::gEftDelaySe; c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = c.m.r32(sym::gEftDelaySe); c.m.w32(a_, v_); };
    { let v_ = F::BtlTask_CreateChildList(c, p_task_1, ((((5i32) as i64 as u64)) as u32), ((((((12u32) as i32)) as i64 as u64)) as u32)); let a_ = c.m.r32(sym::gEftDelaySe); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDelaySeMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftDelaySeMgr_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDelaySeMgr_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftDelaySe)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftDelaySe); F::BtlPool_Free(c, a0_, a1_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gEftDelaySe; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDelaySeMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftDelaySeMgr_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDelaySeMgr_Update", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDelaySeMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftDelaySe_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDelaySe_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let _ = F::memset(c, l_w_3, 0i32, 12u32); };
    { let s_ = p_arg_2; let d_ = l_w_3; rt::memcpy(c, d_, s_, 12); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDelaySe_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftDelaySe_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDelaySe_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDelaySe_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftDelaySe_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDelaySe_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(({ let a0_ = (c.m.r32(l_w_2) as i32); F::BtlScene_IsCharStopped(c, a0_) }) != 0)) as i32)) != 0) {
        if ((((({ let a_ = (l_w_2).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) <= (0i32)) as i32)) != 0) {
            { let _ = { let a1_ = (c.m.r32((l_w_2).wrapping_add(4)) as i32); F::Snd_PlaySeEx(c, ((4i32) as u32), a1_, 127i32, 64i32, 0i32) }; };
            F::BtlTask_SetDead(c, p_task_1);
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDelaySe_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftDelaySe_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDelaySe_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDelaySe_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharSlotMgr_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlotMgr_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((80u32) as i32)) }; let a_ = sym::gEftCharSlot; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gEftCharSlot); F::memset(c, a0_, 0i32, 80u32) }; };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlotMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharSlotMgr_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlotMgr_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftCharSlot); F::BtlPool_Free(c, a0_, a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftCharSlot; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlotMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharSlotMgr_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlotMgr_Update", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlotMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharSlotMgr_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlotMgr_Reset", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlotMgr_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharSlot_Set0(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlot_Set0", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32], false); }
    let mut l_s_3: u32 = 0;
    let r_ = 'f: {
    l_s_3 = (c.m.r32(sym::gEftCharSlot)).wrapping_add((((p_objId_1) as u32)).wrapping_mul(8));
    if (((((p_task_2) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = p_task_2; let a_ = l_s_3; c.m.w32(a_, v_); };
        { let v_ = 1i32; let a_ = (l_s_3).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlot_Set0", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharSlot_Clear0(c: &mut Ctx, mut p_objId_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlot_Clear0", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_s_2: u32 = 0;
    let r_ = 'f: {
    l_s_2 = (c.m.r32(sym::gEftCharSlot)).wrapping_add((((p_objId_1) as u32)).wrapping_mul(8));
    { let v_ = ((0i32) as u32); let a_ = l_s_2; c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = (l_s_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlot_Clear0", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharSlot_Get0(c: &mut Ctx, mut p_objId_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlot_Get0", &PROBE_N, &[(p_objId_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = (c.m.r32(sym::gEftCharSlot)).wrapping_add((((p_objId_1) as u32)).wrapping_mul(8)); c.m.r32(a_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlot_Get0", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharSlot_Set1(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlot_Set1", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32], false); }
    let mut l_s_3: u32 = 0;
    let r_ = 'f: {
    l_s_3 = ((c.m.r32(sym::gEftCharSlot)).wrapping_add(16)).wrapping_add((((p_objId_1) as u32)).wrapping_mul(8));
    if (((((p_task_2) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = p_task_2; let a_ = l_s_3; c.m.w32(a_, v_); };
        { let v_ = 1i32; let a_ = (l_s_3).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlot_Set1", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharSlot_Clear1(c: &mut Ctx, mut p_objId_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlot_Clear1", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_s_2: u32 = 0;
    let r_ = 'f: {
    l_s_2 = ((c.m.r32(sym::gEftCharSlot)).wrapping_add(16)).wrapping_add((((p_objId_1) as u32)).wrapping_mul(8));
    { let v_ = ((0i32) as u32); let a_ = l_s_2; c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = (l_s_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlot_Clear1", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharSlot_Get1(c: &mut Ctx, mut p_objId_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlot_Get1", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_s_2: u32 = 0;
    let r_ = 'f: {
    l_s_2 = ((c.m.r32(sym::gEftCharSlot)).wrapping_add(16)).wrapping_add((((p_objId_1) as u32)).wrapping_mul(8));
    break 'f c.m.r32(l_s_2);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlot_Get1", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharSlot_Set2(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlot_Set2", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32], false); }
    let mut l_s_3: u32 = 0;
    let r_ = 'f: {
    l_s_3 = ((c.m.r32(sym::gEftCharSlot)).wrapping_add(32)).wrapping_add((((p_objId_1) as u32)).wrapping_mul(8));
    if (((((p_task_2) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = p_task_2; let a_ = l_s_3; c.m.w32(a_, v_); };
        { let v_ = 1i32; let a_ = (l_s_3).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlot_Set2", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharSlot_Clear2(c: &mut Ctx, mut p_objId_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlot_Clear2", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_s_2: u32 = 0;
    let r_ = 'f: {
    l_s_2 = ((c.m.r32(sym::gEftCharSlot)).wrapping_add(32)).wrapping_add((((p_objId_1) as u32)).wrapping_mul(8));
    { let v_ = ((0i32) as u32); let a_ = l_s_2; c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = (l_s_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlot_Clear2", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharSlot_Get2(c: &mut Ctx, mut p_objId_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlot_Get2", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_s_2: u32 = 0;
    let r_ = 'f: {
    l_s_2 = ((c.m.r32(sym::gEftCharSlot)).wrapping_add(32)).wrapping_add((((p_objId_1) as u32)).wrapping_mul(8));
    break 'f c.m.r32(l_s_2);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlot_Get2", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharSlot_Set3(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlot_Set3", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32], false); }
    let mut l_s_3: u32 = 0;
    let r_ = 'f: {
    l_s_3 = ((c.m.r32(sym::gEftCharSlot)).wrapping_add(48)).wrapping_add((((p_objId_1) as u32)).wrapping_mul(8));
    if (((((p_task_2) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = p_task_2; let a_ = l_s_3; c.m.w32(a_, v_); };
        { let v_ = 1i32; let a_ = (l_s_3).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlot_Set3", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharSlot_Clear3(c: &mut Ctx, mut p_objId_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlot_Clear3", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_s_2: u32 = 0;
    let r_ = 'f: {
    l_s_2 = ((c.m.r32(sym::gEftCharSlot)).wrapping_add(48)).wrapping_add((((p_objId_1) as u32)).wrapping_mul(8));
    { let v_ = ((0i32) as u32); let a_ = l_s_2; c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = (l_s_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlot_Clear3", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharSlot_Get3(c: &mut Ctx, mut p_objId_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlot_Get3", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_s_2: u32 = 0;
    let r_ = 'f: {
    l_s_2 = ((c.m.r32(sym::gEftCharSlot)).wrapping_add(48)).wrapping_add((((p_objId_1) as u32)).wrapping_mul(8));
    break 'f c.m.r32(l_s_2);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlot_Get3", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharSlot_SetAbsorb(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlot_SetAbsorb", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32], false); }
    let mut l_s_3: u32 = 0;
    let r_ = 'f: {
    l_s_3 = ((c.m.r32(sym::gEftCharSlot)).wrapping_add(64)).wrapping_add((((p_objId_1) as u32)).wrapping_mul(8));
    if (((((p_task_2) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = p_task_2; let a_ = l_s_3; c.m.w32(a_, v_); };
        { let v_ = 1i32; let a_ = (l_s_3).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlot_SetAbsorb", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharSlot_ClearAbsorb(c: &mut Ctx, mut p_objId_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlot_ClearAbsorb", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_s_2: u32 = 0;
    let r_ = 'f: {
    l_s_2 = ((c.m.r32(sym::gEftCharSlot)).wrapping_add(64)).wrapping_add((((p_objId_1) as u32)).wrapping_mul(8));
    { let v_ = ((0i32) as u32); let a_ = l_s_2; c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = (l_s_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlot_ClearAbsorb", &PROBE_N, &[], true); }
    r_
}

pub fn EftCharSlot_GetAbsorb(c: &mut Ctx, mut p_objId_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlot_GetAbsorb", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_s_2: u32 = 0;
    let r_ = 'f: {
    l_s_2 = ((c.m.r32(sym::gEftCharSlot)).wrapping_add(64)).wrapping_add((((p_objId_1) as u32)).wrapping_mul(8));
    break 'f c.m.r32(l_s_2);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftCharSlot_GetAbsorb", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlade_Create(c: &mut Ctx, mut p_objId_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlade_Create", &PROBE_N, &[(p_objId_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(4768) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 4768);
    let r_ = 'f: {
        { let _ = F::memset(c, fp, 0i32, 4768u32); };
        { let v_ = p_objId_1; let a_ = (fp).wrapping_add(4736); c.m.w32(a_, (v_) as u32); };
        { let v_ = (rt::rem_i32(F::Port_Rand(c), 30i32)).wrapping_add(60i32); let a_ = (fp).wrapping_add(4740); c.m.w32(a_, (v_) as u32); };
        { let _ = { let a0_ = { let a_ = c.m.r32(sym::gEftBlade); c.m.r32(a_) }; F::BtlTaskList_AddTail(c, a0_, sym::gEftBladeClass, fp) }; };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBlade_Create", &PROBE_N, &[], true); }
    r_
}

pub fn EftBladeMgr_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBladeMgr_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_d_2: u32 = 0;
    let r_ = 'f: {
    l_d_2 = sym::gEftBladeDef;
    F::Vec4_Set(c, (l_d_2).wrapping_add((((0i32) as u32)).wrapping_mul(64)), F32(0x00000000), F32(0x00000000), F32(0xbfc00000), F32(0x3f800000));
    F::Vec4_Set(c, ((l_d_2).wrapping_add((((0i32) as u32)).wrapping_mul(64))).wrapping_add(16), F32(0x00000000), F32(0x00000000), F32(0xc1280000), F32(0x3f800000));
    F::Vec4_Set(c, ((l_d_2).wrapping_add((((0i32) as u32)).wrapping_mul(64))).wrapping_add(32), F32(0x42540000), F32(0x43130000), F32(0x43720000), F32(0x435b0000));
    { let v_ = 62i32; let a_ = ((l_d_2).wrapping_add((((0i32) as u32)).wrapping_mul(64))).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
    { let v_ = 1i32; let a_ = ((l_d_2).wrapping_add((((0i32) as u32)).wrapping_mul(64))).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
    F::Vec4_Set(c, (l_d_2).wrapping_add((((1i32) as u32)).wrapping_mul(64)), F32(0xbfc00000), F32(0x00000000), F32(0xbfc00000), F32(0x3f800000));
    F::Vec4_Set(c, ((l_d_2).wrapping_add((((1i32) as u32)).wrapping_mul(64))).wrapping_add(16), F32(0xbfc00000), F32(0x00000000), F32(0xc1680000), F32(0x3f800000));
    F::Vec4_Set(c, ((l_d_2).wrapping_add((((1i32) as u32)).wrapping_mul(64))).wrapping_add(32), F32(0x435c0000), F32(0x00000000), F32(0x43160000), F32(0x43570000));
    { let v_ = 0i32; let a_ = ((l_d_2).wrapping_add((((1i32) as u32)).wrapping_mul(64))).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
    { let v_ = 138i32; let a_ = ((l_d_2).wrapping_add((((1i32) as u32)).wrapping_mul(64))).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
    F::Vec4_Set(c, (l_d_2).wrapping_add((((2i32) as u32)).wrapping_mul(64)), F32(0x00000000), F32(0x00000000), F32(0xbfc00000), F32(0x3f800000));
    F::Vec4_Set(c, ((l_d_2).wrapping_add((((2i32) as u32)).wrapping_mul(64))).wrapping_add(16), F32(0x00000000), F32(0x00000000), F32(0xc1019999), F32(0x3f800000));
    F::Vec4_Set(c, ((l_d_2).wrapping_add((((2i32) as u32)).wrapping_mul(64))).wrapping_add(32), F32(0x42540000), F32(0x43130000), F32(0x43720000), F32(0x435b0000));
    { let v_ = 1i32; let a_ = ((l_d_2).wrapping_add((((2i32) as u32)).wrapping_mul(64))).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
    { let v_ = 39i32; let a_ = ((l_d_2).wrapping_add((((2i32) as u32)).wrapping_mul(64))).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
    F::Vec4_Set(c, (l_d_2).wrapping_add((((3i32) as u32)).wrapping_mul(64)), F32(0x00000000), F32(0x00000000), F32(0xbfc00000), F32(0x3f800000));
    F::Vec4_Set(c, ((l_d_2).wrapping_add((((3i32) as u32)).wrapping_mul(64))).wrapping_add(16), F32(0x00000000), F32(0x00000000), F32(0xc1019999), F32(0x3f800000));
    F::Vec4_Set(c, ((l_d_2).wrapping_add((((3i32) as u32)).wrapping_mul(64))).wrapping_add(32), F32(0x42540000), F32(0x43130000), F32(0x43720000), F32(0x435b0000));
    { let v_ = 1i32; let a_ = ((l_d_2).wrapping_add((((3i32) as u32)).wrapping_mul(64))).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
    { let v_ = 40i32; let a_ = ((l_d_2).wrapping_add((((3i32) as u32)).wrapping_mul(64))).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
    F::Vec4_Set(c, (l_d_2).wrapping_add((((4i32) as u32)).wrapping_mul(64)), F32(0x00000000), F32(0x00000000), F32(0xbfc00000), F32(0x3f800000));
    F::Vec4_Set(c, ((l_d_2).wrapping_add((((4i32) as u32)).wrapping_mul(64))).wrapping_add(16), F32(0x00000000), F32(0x00000000), F32(0xc1080000), F32(0x3f800000));
    F::Vec4_Set(c, ((l_d_2).wrapping_add((((4i32) as u32)).wrapping_mul(64))).wrapping_add(32), F32(0x435c0000), F32(0x435c0000), F32(0x43700000), F32(0x437f0000));
    { let v_ = 0i32; let a_ = ((l_d_2).wrapping_add((((4i32) as u32)).wrapping_mul(64))).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
    { let v_ = 60i32; let a_ = ((l_d_2).wrapping_add((((4i32) as u32)).wrapping_mul(64))).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
    F::Vec4_Set(c, (l_d_2).wrapping_add((((5i32) as u32)).wrapping_mul(64)), F32(0x00000000), F32(0x00000000), F32(0xbfc00000), F32(0x3f800000));
    F::Vec4_Set(c, ((l_d_2).wrapping_add((((5i32) as u32)).wrapping_mul(64))).wrapping_add(16), F32(0x00000000), F32(0x00000000), F32(0xc1280000), F32(0x3f800000));
    F::Vec4_Set(c, ((l_d_2).wrapping_add((((5i32) as u32)).wrapping_mul(64))).wrapping_add(32), F32(0x435c0000), F32(0x435c0000), F32(0x43700000), F32(0x437f0000));
    { let v_ = 0i32; let a_ = ((l_d_2).wrapping_add((((5i32) as u32)).wrapping_mul(64))).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
    { let v_ = 148i32; let a_ = ((l_d_2).wrapping_add((((5i32) as u32)).wrapping_mul(64))).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
    F::Vec4_Set(c, (l_d_2).wrapping_add((((6i32) as u32)).wrapping_mul(64)), F32(0x00000000), F32(0x00000000), F32(0xbfc00000), F32(0x3f800000));
    F::Vec4_Set(c, ((l_d_2).wrapping_add((((6i32) as u32)).wrapping_mul(64))).wrapping_add(16), F32(0x00000000), F32(0x00000000), F32(0xc1800000), F32(0x3f800000));
    F::Vec4_Set(c, ((l_d_2).wrapping_add((((6i32) as u32)).wrapping_mul(64))).wrapping_add(32), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x43660000));
    { let v_ = 0i32; let a_ = ((l_d_2).wrapping_add((((6i32) as u32)).wrapping_mul(64))).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
    { let v_ = 111i32; let a_ = ((l_d_2).wrapping_add((((6i32) as u32)).wrapping_mul(64))).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
    F::Vec4_Set(c, (l_d_2).wrapping_add((((7i32) as u32)).wrapping_mul(64)), F32(0x00000000), F32(0x00000000), F32(0xbfc00000), F32(0x3f800000));
    F::Vec4_Set(c, ((l_d_2).wrapping_add((((7i32) as u32)).wrapping_mul(64))).wrapping_add(16), F32(0x00000000), F32(0x00000000), F32(0xc1800000), F32(0x3f800000));
    F::Vec4_Set(c, ((l_d_2).wrapping_add((((7i32) as u32)).wrapping_mul(64))).wrapping_add(32), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x43660000));
    { let v_ = 0i32; let a_ = ((l_d_2).wrapping_add((((7i32) as u32)).wrapping_mul(64))).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
    { let v_ = 97i32; let a_ = ((l_d_2).wrapping_add((((7i32) as u32)).wrapping_mul(64))).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((16u32) as i32)) }; let a_ = sym::gEftBlade; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gEftBlade); F::memset(c, a0_, 0i32, 16u32) }; };
    { let v_ = sym::gEftBladeColor; let a_ = (c.m.r32(sym::gEftBlade)).wrapping_add(12); c.m.w32(a_, v_); };
    { let v_ = F::BtlTask_CreateChildList(c, p_task_1, ((((2i32) as i64 as u64)) as u32), ((((((4784u32) as i32)) as i64 as u64)) as u32)); let a_ = c.m.r32(sym::gEftBlade); c.m.w32(a_, v_); };
    { let v_ = F::BtlCharApi_GetChara(c, 0i32); let a_ = ((c.m.r32(sym::gEftBlade)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = F::BtlCharApi_GetChara(c, 1i32); let a_ = ((c.m.r32(sym::gEftBlade)).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    F::EftBlade_Create(c, 0i32);
    F::EftBlade_Create(c, 1i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBladeMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftBladeMgr_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBladeMgr_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftBlade)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftBlade); F::BtlPool_Free(c, a0_, a1_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gEftBlade; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBladeMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftBladeMgr_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBladeMgr_Update", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBladeMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlade_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlade_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let _ = F::memset(c, l_w_3, 0i32, 4784u32); };
    { let s_ = p_arg_2; let d_ = l_w_3; rt::memcpy(c, d_, s_, 4768); };
    { let v_ = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x41a00000)); let a_ = (l_w_3).wrapping_add(4768); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0xc1200000); let a_ = ((l_w_3).wrapping_add(4768)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x41a00000)); let a_ = ((l_w_3).wrapping_add(4768)).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = ((l_w_3).wrapping_add(4768)).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlade_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlade_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlade_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlade_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlade_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlade_Reset", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlade_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlade_AddPoint(c: &mut Ctx, mut p_t_1: u32, p_a_2: u32, p_b_3: u32, mut p_objId_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlade_AddPoint", &PROBE_N, &[(p_t_1) as u32, (p_a_2) as u32, (p_b_3) as u32, (p_objId_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_head_5: i32 = 0;
    let mut l_p4_6: i32 = 0;
    let mut l_p3_7: i32 = 0;
    let mut l_p1_8: i32 = 0;
    let mut l_p2_9: i32 = 0;
    let mut l_tail_10: i32 = 0;
    let mut l_i_11: i32 = 0;
    rt::memcpy(c, fp, p_a_2, 16);
    rt::memcpy(c, fp.wrapping_add(16), p_b_3, 16);
    let r_ = 'f: {
        l_head_5 = (c.m.r32((p_t_1).wrapping_add(4752)) as i32);
        l_p4_6 = (((l_head_5).wrapping_add(124i32)) & (127i32));
        l_p3_7 = (((l_head_5).wrapping_add(125i32)) & (127i32));
        l_p1_8 = (((l_head_5).wrapping_add(127i32)) & (127i32));
        l_p2_9 = (((l_head_5).wrapping_add(126i32)) & (127i32));
        l_tail_10 = (c.m.r32((p_t_1).wrapping_add(4748)) as i32);
        if ((((((((((c.m.r32((p_t_1).wrapping_add(4756)) as i32)) != (F::BtlCharApi_HasWeaponOut(c, p_objId_4))) as i32)) != 0) && ((((((c.m.r32((p_t_1).wrapping_add(4756)) as i32)) == (0i32)) as i32)) != 0)) as i32)) != 0) {
            {
                l_i_11 = 0i32;
                'l1: loop {
                    if !(((((l_i_11) < (4i32)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        F::Vec4_Copy(c, ((p_t_1).wrapping_add(4096)).wrapping_add((((l_i_11) as u32)).wrapping_mul(16)), fp);
                        F::Vec4_Copy(c, ((p_t_1).wrapping_add(4160)).wrapping_add((((l_i_11) as u32)).wrapping_mul(16)), fp.wrapping_add(16));
                    }
                    { let o_ = l_i_11; let n_ = (o_).wrapping_add(1); l_i_11 = n_; };
                }
            }
        }
        { let v_ = F::BtlCharApi_HasWeaponOut(c, p_objId_4); let a_ = (p_t_1).wrapping_add(4756); c.m.w32(a_, (v_) as u32); };
        if ((F::BtlCharApi_HasWeaponOut(c, p_objId_4)) != 0) {
            {
                l_i_11 = 0i32;
                'l3: loop {
                    if !(((((l_i_11) < (3i32)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        F::Vec4_Copy(c, ((p_t_1).wrapping_add(4096)).wrapping_add((((l_i_11) as u32)).wrapping_mul(16)), ((p_t_1).wrapping_add(4096)).wrapping_add(((((l_i_11).wrapping_add(1i32)) as u32)).wrapping_mul(16)));
                        F::Vec4_Copy(c, ((p_t_1).wrapping_add(4160)).wrapping_add((((l_i_11) as u32)).wrapping_mul(16)), ((p_t_1).wrapping_add(4160)).wrapping_add(((((l_i_11).wrapping_add(1i32)) as u32)).wrapping_mul(16)));
                    }
                    { let o_ = l_i_11; let n_ = (o_).wrapping_add(1); l_i_11 = n_; };
                }
            }
            F::Vec4_Copy(c, ((p_t_1).wrapping_add(4096)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), fp);
            F::Vec4_Copy(c, ((p_t_1).wrapping_add(4160)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), fp.wrapping_add(16));
        }
        if (((((((((((((((((((((l_head_5) == (l_tail_10)) as i32)) != 0) || (((((l_p1_8) == (l_tail_10)) as i32)) != 0)) as i32)) != 0) || (((((l_p2_9) == (l_tail_10)) as i32)) != 0)) as i32)) != 0) || (((((l_p3_7) == (l_tail_10)) as i32)) != 0)) as i32)) != 0) || (((((l_p4_6) == (l_tail_10)) as i32)) != 0)) as i32)) != 0) {
            if ((F::BtlCharApi_HasWeaponOut(c, p_objId_4)) != 0) {
                F::Vec4_Copy(c, (p_t_1).wrapping_add((((l_head_5) as u32)).wrapping_mul(16)), fp);
                F::Vec4_Copy(c, ((p_t_1).wrapping_add(2048)).wrapping_add((((l_head_5) as u32)).wrapping_mul(16)), fp.wrapping_add(16));
                { let v_ = 81i32; let a_ = ((p_t_1).wrapping_add(4224)).wrapping_add((((l_head_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                l_head_5 = (((l_head_5).wrapping_add(1i32)) & (127i32));
                if (((((l_head_5) == (l_tail_10)) as i32)) != 0) {
                    l_tail_10 = (((l_tail_10).wrapping_add(1i32)) & (127i32));
                }
            }
        } else {
            if ((F::BtlCharApi_HasWeaponOut(c, p_objId_4)) != 0) {
                F::Vec4_Sub(c, fp.wrapping_add(64), ((p_t_1).wrapping_add(4096)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), ((p_t_1).wrapping_add(4096)).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
                if ((((!(((rt::fle(F::Vec3_Length(c, fp.wrapping_add(64)), F32(0x3f800000)) as i32)) != 0)) as i32)) != 0) {
                    F::Vec4_Copy(c, fp.wrapping_add(32), (p_t_1).wrapping_add((((l_p1_8) as u32)).wrapping_mul(16)));
                    F::Vec4_Copy(c, fp.wrapping_add(48), ((p_t_1).wrapping_add(2048)).wrapping_add((((l_p1_8) as u32)).wrapping_mul(16)));
                    F::EftBlade_Spline(c, (p_t_1).wrapping_add((((l_p1_8) as u32)).wrapping_mul(16)), (p_t_1).wrapping_add(4096), F32(0x3dcccccc));
                    F::EftBlade_Spline(c, ((p_t_1).wrapping_add(2048)).wrapping_add((((l_p1_8) as u32)).wrapping_mul(16)), (p_t_1).wrapping_add(4160), F32(0x3dcccccc));
                    { let v_ = 76i32; let a_ = ((p_t_1).wrapping_add(4224)).wrapping_add((((l_p1_8) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    F::EftBlade_Spline(c, (p_t_1).wrapping_add((((l_head_5) as u32)).wrapping_mul(16)), (p_t_1).wrapping_add(4096), F32(0x3e4ccccc));
                    F::EftBlade_Spline(c, ((p_t_1).wrapping_add(2048)).wrapping_add((((l_head_5) as u32)).wrapping_mul(16)), (p_t_1).wrapping_add(4160), F32(0x3e4ccccc));
                    { let v_ = 78i32; let a_ = ((p_t_1).wrapping_add(4224)).wrapping_add((((l_head_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_head_5 = (((l_head_5).wrapping_add(1i32)) & (127i32));
                    if (((((l_head_5) == (l_tail_10)) as i32)) != 0) {
                        l_tail_10 = (((l_tail_10).wrapping_add(1i32)) & (127i32));
                    }
                    F::EftBlade_Spline(c, (p_t_1).wrapping_add((((l_head_5) as u32)).wrapping_mul(16)), (p_t_1).wrapping_add(4096), F32(0x3e999999));
                    F::EftBlade_Spline(c, ((p_t_1).wrapping_add(2048)).wrapping_add((((l_head_5) as u32)).wrapping_mul(16)), (p_t_1).wrapping_add(4160), F32(0x3e999999));
                    { let v_ = 80i32; let a_ = ((p_t_1).wrapping_add(4224)).wrapping_add((((l_head_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_head_5 = (((l_head_5).wrapping_add(1i32)) & (127i32));
                    if (((((l_head_5) == (l_tail_10)) as i32)) != 0) {
                        l_tail_10 = (((l_tail_10).wrapping_add(1i32)) & (127i32));
                    }
                    F::EftBlade_Spline(c, (p_t_1).wrapping_add((((l_head_5) as u32)).wrapping_mul(16)), (p_t_1).wrapping_add(4096), F32(0x3ecccccc));
                    F::EftBlade_Spline(c, ((p_t_1).wrapping_add(2048)).wrapping_add((((l_head_5) as u32)).wrapping_mul(16)), (p_t_1).wrapping_add(4160), F32(0x3ecccccc));
                    { let v_ = 82i32; let a_ = ((p_t_1).wrapping_add(4224)).wrapping_add((((l_head_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_head_5 = (((l_head_5).wrapping_add(1i32)) & (127i32));
                    if (((((l_head_5) == (l_tail_10)) as i32)) != 0) {
                        l_tail_10 = (((l_tail_10).wrapping_add(1i32)) & (127i32));
                    }
                    F::EftBlade_Spline(c, (p_t_1).wrapping_add((((l_head_5) as u32)).wrapping_mul(16)), (p_t_1).wrapping_add(4096), F32(0x3f000000));
                    F::EftBlade_Spline(c, ((p_t_1).wrapping_add(2048)).wrapping_add((((l_head_5) as u32)).wrapping_mul(16)), (p_t_1).wrapping_add(4160), F32(0x3f000000));
                    { let v_ = 84i32; let a_ = ((p_t_1).wrapping_add(4224)).wrapping_add((((l_head_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_head_5 = (((l_head_5).wrapping_add(1i32)) & (127i32));
                    if (((((l_head_5) == (l_tail_10)) as i32)) != 0) {
                        l_tail_10 = (((l_tail_10).wrapping_add(1i32)) & (127i32));
                    }
                    F::EftBlade_Spline(c, (p_t_1).wrapping_add((((l_head_5) as u32)).wrapping_mul(16)), (p_t_1).wrapping_add(4096), F32(0x3f199999));
                    F::EftBlade_Spline(c, ((p_t_1).wrapping_add(2048)).wrapping_add((((l_head_5) as u32)).wrapping_mul(16)), (p_t_1).wrapping_add(4160), F32(0x3f199999));
                    { let v_ = 86i32; let a_ = ((p_t_1).wrapping_add(4224)).wrapping_add((((l_head_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_head_5 = (((l_head_5).wrapping_add(1i32)) & (127i32));
                    if (((((l_head_5) == (l_tail_10)) as i32)) != 0) {
                        l_tail_10 = (((l_tail_10).wrapping_add(1i32)) & (127i32));
                    }
                    F::EftBlade_Spline(c, (p_t_1).wrapping_add((((l_head_5) as u32)).wrapping_mul(16)), (p_t_1).wrapping_add(4096), F32(0x3f333333));
                    F::EftBlade_Spline(c, ((p_t_1).wrapping_add(2048)).wrapping_add((((l_head_5) as u32)).wrapping_mul(16)), (p_t_1).wrapping_add(4160), F32(0x3f333333));
                    { let v_ = 88i32; let a_ = ((p_t_1).wrapping_add(4224)).wrapping_add((((l_head_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_head_5 = (((l_head_5).wrapping_add(1i32)) & (127i32));
                    if (((((l_head_5) == (l_tail_10)) as i32)) != 0) {
                        l_tail_10 = (((l_tail_10).wrapping_add(1i32)) & (127i32));
                    }
                    F::EftBlade_Spline(c, (p_t_1).wrapping_add((((l_head_5) as u32)).wrapping_mul(16)), (p_t_1).wrapping_add(4096), F32(0x3f4ccccc));
                    F::EftBlade_Spline(c, ((p_t_1).wrapping_add(2048)).wrapping_add((((l_head_5) as u32)).wrapping_mul(16)), (p_t_1).wrapping_add(4160), F32(0x3f4ccccc));
                    { let v_ = 90i32; let a_ = ((p_t_1).wrapping_add(4224)).wrapping_add((((l_head_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_head_5 = (((l_head_5).wrapping_add(1i32)) & (127i32));
                    if (((((l_head_5) == (l_tail_10)) as i32)) != 0) {
                        l_tail_10 = (((l_tail_10).wrapping_add(1i32)) & (127i32));
                    }
                    F::EftBlade_Spline(c, (p_t_1).wrapping_add((((l_head_5) as u32)).wrapping_mul(16)), (p_t_1).wrapping_add(4096), F32(0x3f666666));
                    F::EftBlade_Spline(c, ((p_t_1).wrapping_add(2048)).wrapping_add((((l_head_5) as u32)).wrapping_mul(16)), (p_t_1).wrapping_add(4160), F32(0x3f666666));
                    { let v_ = 92i32; let a_ = ((p_t_1).wrapping_add(4224)).wrapping_add((((l_head_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_head_5 = (((l_head_5).wrapping_add(1i32)) & (127i32));
                    if (((((l_head_5) == (l_tail_10)) as i32)) != 0) {
                        l_tail_10 = (((l_tail_10).wrapping_add(1i32)) & (127i32));
                    }
                    F::Vec4_Copy(c, (p_t_1).wrapping_add((((l_head_5) as u32)).wrapping_mul(16)), fp.wrapping_add(32));
                    F::Vec4_Copy(c, ((p_t_1).wrapping_add(2048)).wrapping_add((((l_head_5) as u32)).wrapping_mul(16)), fp.wrapping_add(48));
                    { let v_ = 94i32; let a_ = ((p_t_1).wrapping_add(4224)).wrapping_add((((l_head_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_head_5 = (((l_head_5).wrapping_add(1i32)) & (127i32));
                    if (((((l_head_5) == (l_tail_10)) as i32)) != 0) {
                        l_tail_10 = (((l_tail_10).wrapping_add(1i32)) & (127i32));
                    }
                }
                F::Vec4_Copy(c, (p_t_1).wrapping_add((((l_head_5) as u32)).wrapping_mul(16)), fp);
                F::Vec4_Copy(c, ((p_t_1).wrapping_add(2048)).wrapping_add((((l_head_5) as u32)).wrapping_mul(16)), fp.wrapping_add(16));
                { let v_ = 96i32; let a_ = ((p_t_1).wrapping_add(4224)).wrapping_add((((l_head_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                l_head_5 = (((l_head_5).wrapping_add(1i32)) & (127i32));
                if (((((l_head_5) == (l_tail_10)) as i32)) != 0) {
                    l_tail_10 = (((l_tail_10).wrapping_add(1i32)) & (127i32));
                }
            }
        }
        { let v_ = l_tail_10; let a_ = (p_t_1).wrapping_add(4748); c.m.w32(a_, (v_) as u32); };
        { let v_ = l_head_5; let a_ = (p_t_1).wrapping_add(4752); c.m.w32(a_, (v_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBlade_AddPoint", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlade_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlade_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(128) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 128);
    let mut l_t_2: u32 = 0;
    let mut l_head_3: i32 = 0;
    let mut l_tail_4: i32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_j_6: i32 = 0;
    let r_ = 'f: {
        l_t_2 = c.m.r32((p_task_1).wrapping_add(56));
        rt::memset(c, fp.wrapping_add(96), 0, 32);
        rt::memset(c, fp.wrapping_add(96).wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(96).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(96).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0xbfc00000); c.m.w32(fp.wrapping_add(96).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(96).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(96).wrapping_add(16), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(96).wrapping_add(16).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(96).wrapping_add(16).wrapping_add(4), (v_).0); }
        { let v_ = F32(0xc1280000); c.m.w32(fp.wrapping_add(96).wrapping_add(16).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(96).wrapping_add(16).wrapping_add(12), (v_).0); }
        l_head_3 = (c.m.r32((l_t_2).wrapping_add(4752)) as i32);
        l_tail_4 = (c.m.r32((l_t_2).wrapping_add(4748)) as i32);
        if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
            break 'f;
        }
        {
            l_i_5 = l_tail_4;
            'l1: loop {
                if !(((((l_i_5) != (l_head_3)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if ((((((c.m.r32(((l_t_2).wrapping_add(4224)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))) as i32)) > (0i32)) as i32)) != 0) {
                        { let r_ = 20i32; let a_ = ((l_t_2).wrapping_add(4224)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                        if ((((((c.m.r32(((l_t_2).wrapping_add(4224)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))) as i32)) <= (0i32)) as i32)) != 0) {
                            { let v_ = 0i32; let a_ = ((l_t_2).wrapping_add(4224)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            { let v_ = l_i_5; let a_ = (l_t_2).wrapping_add(4748); c.m.w32(a_, (v_) as u32); };
                        }
                    }
                }
                l_i_5 = (((l_i_5).wrapping_add(1i32)) & (127i32));
            }
        }
        if (({ let a0_ = (c.m.r32((l_t_2).wrapping_add(4736)) as i32); F::BtlCharApi_HasWeaponOut(c, a0_) }) != 0) {
            {
                l_j_6 = 0i32;
                'l3: loop {
                    if !(((((l_j_6) < (8i32)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        if ((((((c.m.r32(((sym::gEftBladeDef).wrapping_add((((l_j_6) as u32)).wrapping_mul(64))).wrapping_add(52)) as i32)) == ({ let a0_ = (c.m.r32((l_t_2).wrapping_add(4736)) as i32); F::BtlCharApi_GetChara(c, a0_) })) as i32)) != 0) {
                            F::Vec4_Copy(c, (fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (sym::gEftBladeDef).wrapping_add((((l_j_6) as u32)).wrapping_mul(64)));
                            F::Vec4_Copy(c, (fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), ((sym::gEftBladeDef).wrapping_add((((l_j_6) as u32)).wrapping_mul(64))).wrapping_add(16));
                        }
                    }
                    { let o_ = l_j_6; let n_ = (o_).wrapping_add(1); l_j_6 = n_; };
                }
            }
            { let a0_ = (c.m.r32((l_t_2).wrapping_add(4736)) as i32); F::BtlCharApi_GetNodeMtx(c, a0_, 64i32, fp) };
            F::Mtx_MulVec4(c, fp.wrapping_add(64), fp, (fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
            F::Mtx_MulVec4(c, fp.wrapping_add(80), fp, (fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
            { let a3_ = (c.m.r32((l_t_2).wrapping_add(4736)) as i32); F::EftBlade_AddPoint(c, l_t_2, fp.wrapping_add(64), fp.wrapping_add(80), a3_) };
        } else {
            { let v_ = 0i32; let a_ = (l_t_2).wrapping_add(4756); c.m.w32(a_, (v_) as u32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBlade_Update", &PROBE_N, &[], true); }
    r_
}

pub fn eft_aa__EftBlade_MakePrim(c: &mut Ctx, mut p_type_1: i32, mut p_abe_2: i32, mut p_ctx_3: i32) -> u64 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_aa__EftBlade_MakePrim", &PROBE_N, &[(p_type_1) as u32, (p_abe_2) as u32, (p_ctx_3) as u32], false); }
    let r_ = 'f: {
    break 'f (((((((p_ctx_3) as u64)).wrapping_shl((9i32) as u32)) | ((((p_abe_2) as u64)).wrapping_shl((6i32) as u32)))) | (((p_type_1) as u64)));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "eft_aa__EftBlade_MakePrim", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlade_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlade_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(224) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 224);
    let mut l_t_2: u32 = 0;
    let mut l_head_3: i32 = 0;
    let mut l_p3_4: i32 = 0;
    let mut l_p2_5: i32 = 0;
    let mut l_p1_6: i32 = 0;
    let mut l_p4_7: i32 = 0;
    let mut l_i_8: i32 = 0;
    let mut l_def_9: u32 = 0;
    let mut l_aScale_10: F32 = F32(0);
    let mut l_k_11: i32 = 0;
    let mut l_p_12: u32 = 0;
    let mut l_e_13: u32 = 0;
    let mut l_abe_14: i32 = 0;
    let mut l_layer_15: i32 = 0;
    let mut l_l_16: i32 = 0;
    let mut l_z_17: i32 = 0;
    let r_ = 'f: {
        l_t_2 = c.m.r32((p_task_1).wrapping_add(56));
        rt::memset(c, fp, 0, 48);
        rt::memset(c, fp.wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(16), 0, 16);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(32), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(32).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(32).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(32).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(32).wrapping_add(12), (v_).0); }
        l_head_3 = (c.m.r32((l_t_2).wrapping_add(4752)) as i32);
        l_p3_4 = (((l_head_3).wrapping_add(125i32)) & (127i32));
        l_p2_5 = (((l_head_3).wrapping_add(126i32)) & (127i32));
        l_p1_6 = (((l_head_3).wrapping_add(127i32)) & (127i32));
        l_p4_7 = (((l_head_3).wrapping_add(124i32)) & (127i32));
        l_i_8 = (c.m.r32((l_t_2).wrapping_add(4748)) as i32);
        if (({ let a0_ = (c.m.r32((l_t_2).wrapping_add(4736)) as i32); F::BtlCharApi_IsHidden(c, a0_) }) != 0) {
            break 'f;
        }
        if (((((((((((((((((((((l_head_3) == (l_i_8)) as i32)) != 0) || (((((l_p1_6) == (l_i_8)) as i32)) != 0)) as i32)) != 0) || (((((l_p2_5) == (l_i_8)) as i32)) != 0)) as i32)) != 0) || (((((l_p3_4) == (l_i_8)) as i32)) != 0)) as i32)) != 0) || (((((l_p4_7) == (l_i_8)) as i32)) != 0)) as i32)) != 0) {
            break 'f;
        }
        { let _ = F::EftBlade_GetColor(c, fp.wrapping_add(192)); };
        l_def_9 = { let a0_ = (c.m.r32((l_t_2).wrapping_add(4736)) as i32); F::EftBlade_FindDef(c, a0_) };
        if (((((l_def_9) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        { let v_ = rt::f2i(F32(c.m.r32((l_def_9).wrapping_add(32)))); let a_ = fp.wrapping_add(192); c.m.w32(a_, (v_) as u32); };
        { let v_ = rt::f2i(F32(c.m.r32(((l_def_9).wrapping_add(32)).wrapping_add(4)))); let a_ = (fp.wrapping_add(192)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = rt::f2i(F32(c.m.r32(((l_def_9).wrapping_add(32)).wrapping_add(8)))); let a_ = (fp.wrapping_add(192)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = rt::f2i(F32(c.m.r32(((l_def_9).wrapping_add(32)).wrapping_add(12)))); let a_ = (fp.wrapping_add(192)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        l_aScale_10 = rt::fdiv(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add(12)) as i32)), F32(0x437f0000));
        'l1: while (((((l_i_8) != (l_head_3)) as i32)) != 0) {
            l_i_8 = (((l_i_8).wrapping_add(1i32)) & (127i32));
            if (((((l_i_8) == (l_p1_6)) as i32)) != 0) {
                break 'l1;
            }
            if ((((((c.m.r32(((l_t_2).wrapping_add(4224)).wrapping_add((((l_i_8) as u32)).wrapping_mul(4))) as i32)) == (0i32)) as i32)) != 0) {
                continue 'l1;
            }
            {
                l_k_11 = 0i32;
                'l2: loop {
                    if !(((((l_k_11) < (2i32)) as i32)) != 0) { break 'l2; }
                    'c3: {
                        l_abe_14 = 1i32;
                        if ((((l_k_11) & (1i32))) != 0) {
                            F::Vec4_Copy(c, (fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (l_t_2).wrapping_add((((l_i_8) as u32)).wrapping_mul(16)));
                            F::Vec4_Copy(c, (fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), ((l_t_2).wrapping_add(2048)).wrapping_add((((l_i_8) as u32)).wrapping_mul(16)));
                            F::Vec4_Copy(c, (fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (l_t_2).wrapping_add(((((((l_i_8).wrapping_add(1i32)) & (127i32))) as u32)).wrapping_mul(16)));
                            { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(0x00000000); let a_ = ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(0x00000000); let a_ = ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(0x00000000); let a_ = ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            { let v_ = (c.m.r32(((l_t_2).wrapping_add(4224)).wrapping_add((((l_i_8) as u32)).wrapping_mul(4))) as i32); let a_ = (fp.wrapping_add(208)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 0i32; let a_ = (fp.wrapping_add(208)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 0i32; let a_ = (fp.wrapping_add(208)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        } else {
                            F::Vec4_Copy(c, (fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), ((l_t_2).wrapping_add(2048)).wrapping_add((((l_i_8) as u32)).wrapping_mul(16)));
                            F::Vec4_Copy(c, (fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (l_t_2).wrapping_add(((((((l_i_8).wrapping_add(1i32)) & (127i32))) as u32)).wrapping_mul(16)));
                            F::Vec4_Copy(c, (fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), ((l_t_2).wrapping_add(2048)).wrapping_add(((((((l_i_8).wrapping_add(1i32)) & (127i32))) as u32)).wrapping_mul(16)));
                            { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(0x00000000); let a_ = ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(0x00000000); let a_ = ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(0x00000000); let a_ = ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            { let v_ = (c.m.r32(((l_t_2).wrapping_add(4224)).wrapping_add((((l_i_8) as u32)).wrapping_mul(4))) as i32); let a_ = (fp.wrapping_add(208)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            { let v_ = (c.m.r32(((l_t_2).wrapping_add(4224)).wrapping_add(((((((l_i_8).wrapping_add(1i32)) & (127i32))) as u32)).wrapping_mul(4))) as i32); let a_ = (fp.wrapping_add(208)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 0i32; let a_ = (fp.wrapping_add(208)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        { let a2_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); { let _ = F::Mtx_ProjectPointsStq(c, fp.wrapping_add(96), fp.wrapping_add(144), a2_, fp.wrapping_add(48), fp, 3i32); } };
                        if ((F::EftBlade_IsClipped(c, (fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16)))) != 0) {
                            break 'c3;
                        }
                        if ((F::EftBlade_IsClipped(c, (fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(16)))) != 0) {
                            break 'c3;
                        }
                        if ((F::EftBlade_IsClipped(c, (fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(16)))) != 0) {
                            break 'c3;
                        }
                        l_p_12 = c.m.r32(sym::gOtCur);
                        { let v_ = (l_p_12).wrapping_add((((1i32) as u32)).wrapping_mul(96)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
                        l_layer_15 = (c.m.r32((l_def_9).wrapping_add(48)) as i32);
                        { let v_ = F::eft_aa__EftBlade_MakePrim(c, 11i32, l_abe_14, (((l_layer_15) >= (2i32)) as i32)); let a_ = (l_p_12).wrapping_add(32); c.m.w64(a_, v_); };
                        { let v_ = ((536870917i32) as u32); let a_ = l_p_12; c.m.w32(a_, v_); };
                        { let v_ = ((1342177285i32) as u32); let a_ = (l_p_12).wrapping_add(12); c.m.w32(a_, v_); };
                        { let v_ = ((268435456i32) as u32); let a_ = (l_p_12).wrapping_add(8); c.m.w32(a_, v_); };
                        { let v_ = 9511602413006520321u64; let a_ = (l_p_12).wrapping_add(16); c.m.w64(a_, v_); };
                        { let v_ = ((4094956560u32) as u64); let a_ = (l_p_12).wrapping_add(24); c.m.w64(a_, v_); };
                        { let v_ = ((0i32) as u32); let a_ = (l_p_12).wrapping_add(4); c.m.w32(a_, v_); };
                        { let v_ = (((c.m.r32(fp.wrapping_add(192)) as i32)) as u8); let a_ = ((l_p_12).wrapping_add(40)).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w8(a_, v_); };
                        { let v_ = (((c.m.r32((fp.wrapping_add(192)).wrapping_add(4)) as i32)) as u8); let a_ = (((l_p_12).wrapping_add(40)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(1); c.m.w8(a_, v_); };
                        { let v_ = (((c.m.r32((fp.wrapping_add(192)).wrapping_add(8)) as i32)) as u8); let a_ = (((l_p_12).wrapping_add(40)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(2); c.m.w8(a_, v_); };
                        { let v_ = ((rt::f2i(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(208)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)), l_aScale_10))) as u8); let a_ = (((l_p_12).wrapping_add(40)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(3); c.m.w8(a_, v_); };
                        { let v_ = F32(0x3f800000); let a_ = (((l_p_12).wrapping_add(40)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = (((c.m.r32(fp.wrapping_add(192)) as i32)) as u8); let a_ = ((l_p_12).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w8(a_, v_); };
                        { let v_ = (((c.m.r32((fp.wrapping_add(192)).wrapping_add(4)) as i32)) as u8); let a_ = (((l_p_12).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(1); c.m.w8(a_, v_); };
                        { let v_ = (((c.m.r32((fp.wrapping_add(192)).wrapping_add(8)) as i32)) as u8); let a_ = (((l_p_12).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(2); c.m.w8(a_, v_); };
                        { let v_ = ((rt::f2i(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(208)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)), l_aScale_10))) as u8); let a_ = (((l_p_12).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(3); c.m.w8(a_, v_); };
                        { let v_ = F32(0x3f800000); let a_ = (((l_p_12).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = (((c.m.r32(fp.wrapping_add(192)) as i32)) as u8); let a_ = ((l_p_12).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(16)); c.m.w8(a_, v_); };
                        { let v_ = (((c.m.r32((fp.wrapping_add(192)).wrapping_add(4)) as i32)) as u8); let a_ = (((l_p_12).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(1); c.m.w8(a_, v_); };
                        { let v_ = (((c.m.r32((fp.wrapping_add(192)).wrapping_add(8)) as i32)) as u8); let a_ = (((l_p_12).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(2); c.m.w8(a_, v_); };
                        { let v_ = ((rt::f2i(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(208)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)), l_aScale_10))) as u8); let a_ = (((l_p_12).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(3); c.m.w8(a_, v_); };
                        l_l_16 = l_layer_15;
                        { let v_ = F32(0x3f800000); let a_ = (((l_p_12).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        if (((((l_l_16) >= (2i32)) as i32)) != 0) {
                            { let r_ = 2i32; let n_ = (l_l_16).wrapping_sub(r_); l_l_16 = n_; };
                        }
                        { let v_ = (((c.m.r32((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_12).wrapping_add(40)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                        { let v_ = (((c.m.r32(((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_12).wrapping_add(40)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                        { let v_ = (((c.m.r32(((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_12).wrapping_add(40)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                        { let v_ = ((255i32) as u64); let a_ = ((((l_p_12).wrapping_add(40)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                        { let v_ = (((c.m.r32((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_12).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                        { let v_ = (((c.m.r32(((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_12).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                        { let v_ = (((c.m.r32(((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_12).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                        { let v_ = ((255i32) as u64); let a_ = ((((l_p_12).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                        { let v_ = (((c.m.r32((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_12).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                        { let v_ = (((c.m.r32(((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_12).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                        { let v_ = (((c.m.r32(((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_12).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                        { let v_ = ((255i32) as u64); let a_ = ((((l_p_12).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                        l_z_17 = ((c.m.r32(((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)).wrapping_shr((8i32) as u32);
                        if (((((l_z_17) < (0i32)) as i32)) != 0) {
                            l_e_13 = ((c.m.r32(sym::gOtZ)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_16) as u32)).wrapping_mul(8));
                        } else {
                            if (((((l_z_17) >= (4096i32)) as i32)) != 0) {
                                l_e_13 = ((c.m.r32(sym::gOtZ)).wrapping_add((((4095i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_16) as u32)).wrapping_mul(8));
                            } else {
                                l_e_13 = ((c.m.r32(sym::gOtZ)).wrapping_add((((l_z_17) as u32)).wrapping_mul(16))).wrapping_add((((l_l_16) as u32)).wrapping_mul(8));
                            }
                        }
                        { let v_ = l_p_12; let a_ = (c.m.r32((l_e_13).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
                        { let v_ = l_p_12; let a_ = (l_e_13).wrapping_add(4); c.m.w32(a_, v_); };
                    }
                    { let o_ = l_k_11; let n_ = (o_).wrapping_add(1); l_k_11 = n_; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBlade_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlade_Spline(c: &mut Ctx, mut p_out_1: u32, mut p_p_2: u32, mut p_t_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlade_Spline", &PROBE_N, &[(p_out_1) as u32, (p_p_2) as u32, (p_t_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_t2_4: F32 = F32(0);
    let mut l_t3_5: F32 = F32(0);
    let r_ = 'f: {
        l_t2_4 = rt::fmul(p_t_3, p_t_3);
        F::Vec4_Sub(c, fp.wrapping_add(32), (p_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (p_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
        F::Vec4_Sub(c, fp.wrapping_add(48), (p_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (p_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
        F::Vec4_Add(c, fp, fp.wrapping_add(32), fp.wrapping_add(48));
        F::Vec4_Sub(c, fp.wrapping_add(16), (p_p_2).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (p_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
        F::Vec4_Add(c, fp.wrapping_add(16), fp.wrapping_add(48), fp.wrapping_add(16));
        l_t3_5 = rt::fmul(l_t2_4, p_t_3);
        F::Vec4_Scale(c, fp, fp, F32(0x3f000000));
        F::Vec4_Scale(c, fp.wrapping_add(16), fp.wrapping_add(16), F32(0x3f000000));
        F::Vec4_Scale(c, fp.wrapping_add(32), (p_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(16)), rt::fadd(rt::fsub(rt::fmul(F32(0x40000000), l_t3_5), rt::fmul(F32(0x40400000), l_t2_4)), F32(0x3f800000)));
        F::Vec4_Scale(c, fp, fp, rt::fadd(rt::fsub(l_t3_5, rt::fmul(F32(0x40000000), l_t2_4)), p_t_3));
        F::Vec4_Scale(c, fp.wrapping_add(16), fp.wrapping_add(16), rt::fsub(l_t3_5, l_t2_4));
        F::Vec4_Add(c, fp, fp.wrapping_add(32), fp);
        F::Vec4_Add(c, fp, fp, fp.wrapping_add(16));
        F::Vec4_Scale(c, fp.wrapping_add(32), (p_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(16)), rt::fadd(rt::fmul(F32(0xc0000000), l_t3_5), rt::fmul(F32(0x40400000), l_t2_4)));
        F::Vec4_Add(c, p_out_1, fp.wrapping_add(32), fp);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBlade_Spline", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlade_GetColorPtr(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlade_GetColorPtr", &PROBE_N, &[], false); }
    let mut l_r_1: u32 = 0;
    let r_ = 'f: {
    l_r_1 = ((0i32) as u32);
    if (((((c.m.r32(sym::gEftBlade)) != (((0i32) as u32))) as i32)) != 0) {
        l_r_1 = { let a_ = (c.m.r32(sym::gEftBlade)).wrapping_add(12); c.m.r32(a_) };
    }
    break 'f l_r_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlade_GetColorPtr", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlade_GetColor(c: &mut Ctx, mut p_out_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlade_GetColor", &PROBE_N, &[(p_out_1) as u32], false); }
    let mut l_c_2: u32 = 0;
    let r_ = 'f: {
    l_c_2 = ((0i32) as u32);
    if (((((((((c.m.r32(sym::gEftBlade)) == (((0i32) as u32))) as i32)) != 0) || ((((({ let a_ = (c.m.r32(sym::gEftBlade)).wrapping_add(12); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        { let v_ = 240i32; let a_ = p_out_1; c.m.w32(a_, (v_) as u32); };
        { let v_ = 240i32; let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 240i32; let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        break 'f F32(0x3f800000);
    }
    l_c_2 = { let a_ = (c.m.r32(sym::gEftBlade)).wrapping_add(12); c.m.r32(a_) };
    { let v_ = ((c.m.r8((l_c_2).wrapping_add(((0i32) as u32)))) as i32); let a_ = p_out_1; c.m.w32(a_, (v_) as u32); };
    { let v_ = ((c.m.r8((l_c_2).wrapping_add(((1i32) as u32)))) as i32); let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((c.m.r8((l_c_2).wrapping_add(((2i32) as u32)))) as i32); let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((c.m.r8((l_c_2).wrapping_add(((3i32) as u32)))) as i32); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    break 'f rt::fdiv(rt::i2f((c.m.r32((p_out_1).wrapping_add(12)) as i32)), F32(0x437f0000));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlade_GetColor", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlade_FindDef(c: &mut Ctx, mut p_objId_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlade_FindDef", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (8i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32(((sym::gEftBladeDef).wrapping_add((((l_i_2) as u32)).wrapping_mul(64))).wrapping_add(52)) as i32)) == (F::BtlCharApi_GetChara(c, p_objId_1))) as i32)) != 0) {
                    break 'f (sym::gEftBladeDef).wrapping_add((((l_i_2) as u32)).wrapping_mul(64));
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlade_FindDef", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlade_IsClipped(c: &mut Ctx, mut p_p_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlade_IsClipped", &PROBE_N, &[(p_p_1) as u32], false); }
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
    if rt::probing() { crate::host::harness::probe(c, "EftBlade_IsClipped", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlind_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlind_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((12u32) as i32)) }; let a_ = sym::gEftBlind; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gEftBlind); F::memset(c, a0_, 0i32, 12u32) }; };
    { let r_ = 1i32; let a_ = (c.m.r32(sym::gEftBlind)).wrapping_add(8); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlind_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlind_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlind_Update", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((F::BtlScene_IsTimeStopped(c)) != 0) {
        break 'f;
    }
    { let v_ = (if (((rt::flt(rt::fmul(F::BtlCharApi_GetBlindRatio(c, 0i32), F32(0x437f0000)), F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::fgt(rt::fmul(F::BtlCharApi_GetBlindRatio(c, 0i32), F32(0x437f0000)), F32(0x437f0000)) as i32)) != 0) { F32(0x437f0000) } else { rt::fmul(F::BtlCharApi_GetBlindRatio(c, 0i32), F32(0x437f0000)) }) }); let a_ = (c.m.r32(sym::gEftBlind)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    if (((rt::fle({ let a_ = (c.m.r32(sym::gEftBlind)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, F32(0x00000000)) as i32)) != 0) {
        { let r_ = (!(8i32)); let a_ = (c.m.r32(sym::gEftBlind)).wrapping_add(8); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        { let r_ = 8i32; let a_ = (c.m.r32(sym::gEftBlind)).wrapping_add(8); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if ((F::Battle_IsSplitScreen(c)) != 0) {
        { let v_ = (if (((rt::flt(rt::fmul(F::BtlCharApi_GetBlindRatio(c, 1i32), F32(0x437f0000)), F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::fgt(rt::fmul(F::BtlCharApi_GetBlindRatio(c, 1i32), F32(0x437f0000)), F32(0x437f0000)) as i32)) != 0) { F32(0x437f0000) } else { rt::fmul(F::BtlCharApi_GetBlindRatio(c, 1i32), F32(0x437f0000)) }) }); let a_ = (c.m.r32(sym::gEftBlind)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        if (((rt::fle({ let a_ = (c.m.r32(sym::gEftBlind)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, F32(0x00000000)) as i32)) != 0) {
            { let r_ = (!(16i32)); let a_ = (c.m.r32(sym::gEftBlind)).wrapping_add(8); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        } else {
            { let r_ = 16i32; let a_ = (c.m.r32(sym::gEftBlind)).wrapping_add(8); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlind_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlind_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlind_Draw", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((F::BattleReplay_IsActive(c)) != 0) {
        break 'f;
    }
    if ((F::Battle_IsSplitScreen(c)) != 0) {
        if ((F::BtlScene_IsSingleView(c)) != 0) {
            break 'f;
        }
        if (((({ let a_ = (c.m.r32(sym::gEftBlind)).wrapping_add(8); (c.m.r32(a_) as i32) }) & (8i32))) != 0) {
            { let a2_ = { let a_ = (c.m.r32(sym::gEftBlind)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; F::EftBlind_DrawOverlay(c, 0i32, 0i32, a2_) };
        }
        if (((({ let a_ = (c.m.r32(sym::gEftBlind)).wrapping_add(8); (c.m.r32(a_) as i32) }) & (16i32))) != 0) {
            { let a2_ = { let a_ = (c.m.r32(sym::gEftBlind)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; F::EftBlind_DrawOverlay(c, 1i32, 0i32, a2_) };
        }
    } else {
        if (((({ let a_ = (c.m.r32(sym::gEftBlind)).wrapping_add(8); (c.m.r32(a_) as i32) }) & (8i32))) != 0) {
            { let a2_ = { let a_ = (c.m.r32(sym::gEftBlind)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; F::EftBlind_DrawOverlay(c, 0i32, 1i32, a2_) };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlind_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlind_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlind_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftBlind); F::BtlPool_Free(c, a0_, a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftBlind; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlind_Term", &PROBE_N, &[], true); }
    r_
}

pub fn eft_aa__EftBlind_SetRect(c: &mut Ctx, mut p_p_1: u32, mut p_x0_2: i32, mut p_y0_3: i32, mut p_x1_4: i32, mut p_y1_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_aa__EftBlind_SetRect", &PROBE_N, &[(p_p_1) as u32, (p_x0_2) as u32, (p_y0_3) as u32, (p_x1_4) as u32, (p_y1_5) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((p_x0_2) as u64); let a_ = (p_p_1).wrapping_add(48); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
    { let v_ = ((p_y0_3) as u64); let a_ = ((p_p_1).wrapping_add(48)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
    { let v_ = ((p_x1_4) as u64); let a_ = (p_p_1).wrapping_add(56); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
    { let v_ = ((p_y1_5) as u64); let a_ = ((p_p_1).wrapping_add(56)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
    { let v_ = ((16777215i32) as u64); let a_ = ((p_p_1).wrapping_add(48)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
    { let v_ = ((16777215i32) as u64); let a_ = ((p_p_1).wrapping_add(56)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
    { let v_ = ((255i32) as u64); let a_ = ((p_p_1).wrapping_add(48)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
    { let v_ = ((255i32) as u64); let a_ = ((p_p_1).wrapping_add(56)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "eft_aa__EftBlind_SetRect", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlind_DrawOverlay(c: &mut Ctx, mut p_view_1: i32, mut p_full_2: i32, mut p_alpha_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlind_DrawOverlay", &PROBE_N, &[(p_view_1) as u32, (p_full_2) as u32, (p_alpha_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_p_4: u32 = 0;
    let mut l_e_5: u32 = 0;
    let r_ = 'f: {
        F::Vec4_Set(c, fp, F32(0x43000000), F32(0x43000000), F32(0x43000000), p_alpha_3);
        l_p_4 = c.m.r32(sym::gOtCur);
        { let v_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(64)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
        { let v_ = ((70i32) as u64); let a_ = (l_p_4).wrapping_add(32); c.m.w64(a_, v_); };
        { let v_ = ((536870915i32) as u32); let a_ = l_p_4; c.m.w32(a_, v_); };
        { let v_ = ((1342177283i32) as u32); let a_ = (l_p_4).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = ((268435456i32) as u32); let a_ = (l_p_4).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = ((4899916394579132417i64) as u64); let a_ = (l_p_4).wrapping_add(16); c.m.w64(a_, v_); };
        { let v_ = ((17424i32) as u64); let a_ = (l_p_4).wrapping_add(24); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (l_p_4).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32(fp)))) as u8); let a_ = (l_p_4).wrapping_add(40); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((fp).wrapping_add(4))))) as u8); let a_ = ((l_p_4).wrapping_add(40)).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((fp).wrapping_add(8))))) as u8); let a_ = ((l_p_4).wrapping_add(40)).wrapping_add(2); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((fp).wrapping_add(12))))) as u8); let a_ = ((l_p_4).wrapping_add(40)).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = F32(0x3f800000); let a_ = ((l_p_4).wrapping_add(40)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        if (((((p_full_2) == (0i32)) as i32)) != 0) {
            if (((((p_view_1) == (0i32)) as i32)) != 0) {
                F::eft_aa__EftBlind_SetRect(c, l_p_4, 28672i32, 29184i32, (32768i32).wrapping_neg(), (29184i32).wrapping_neg());
            } else {
                F::eft_aa__EftBlind_SetRect(c, l_p_4, (32768i32).wrapping_neg(), 29184i32, (28672i32).wrapping_neg(), (29184i32).wrapping_neg());
            }
        } else {
            F::eft_aa__EftBlind_SetRect(c, l_p_4, 28672i32, 29184i32, (28672i32).wrapping_neg(), (29184i32).wrapping_neg());
        }
        l_e_5 = ((c.m.r32(sym::gOtLast)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(8));
        { let v_ = l_p_4; let a_ = (c.m.r32((l_e_5).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = l_p_4; let a_ = (l_e_5).wrapping_add(4); c.m.w32(a_, v_); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBlind_DrawOverlay", &PROBE_N, &[], true); }
    r_
}

pub fn EftAnimPart_Create(c: &mut Ctx, mut p_arg_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPart_Create", &PROBE_N, &[(p_arg_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a0_ = c.m.r32(sym::gEftAnimPartList); F::BtlTaskList_AddTail(c, a0_, sym::gEftAnimPartClass, p_arg_1) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPart_Create", &PROBE_N, &[], true); }
    r_
}

pub fn EftAnimPart_Stop(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPart_Stop", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    if ((F::EftAnimPart_IsValid(c, p_task_1)) != 0) {
        { let r_ = 4i32; let a_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(152); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPart_Stop", &PROBE_N, &[], true); }
    r_
}

pub fn EftAnimPart_Kill(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPart_Kill", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    if ((F::EftAnimPart_IsValid(c, p_task_1)) != 0) {
        { let r_ = 8i32; let a_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(152); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPart_Kill", &PROBE_N, &[], true); }
    r_
}

pub fn EftAnimPart_SetType(c: &mut Ctx, mut p_task_1: u32, mut p_type_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPart_SetType", &PROBE_N, &[(p_task_1) as u32, (p_type_2) as u32], false); }
    let r_ = 'f: {
    if ((F::EftAnimPart_IsValid(c, p_task_1)) != 0) {
        { let v_ = p_type_2; let a_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(144); c.m.w32(a_, (v_) as u32); };
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPart_SetType", &PROBE_N, &[], true); }
    r_
}

pub fn EftAnimPart_SetPos(c: &mut Ctx, mut p_task_1: u32, mut p_pos_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPart_SetPos", &PROBE_N, &[(p_task_1) as u32, (p_pos_2) as u32], false); }
    let r_ = 'f: {
    if ((F::EftAnimPart_IsValid(c, p_task_1)) != 0) {
        { let a0_ = c.m.r32((p_task_1).wrapping_add(56)); F::EftSprAnim_SetPos(c, a0_, p_pos_2) };
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPart_SetPos", &PROBE_N, &[], true); }
    r_
}

pub fn EftAnimPart_SetSize(c: &mut Ctx, mut p_task_1: u32, mut p_size_2: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPart_SetSize", &PROBE_N, &[(p_task_1) as u32, (p_size_2).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
        if ((F::EftAnimPart_IsValid(c, p_task_1)) != 0) {
            l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
            F::Vec4_Set(c, fp, p_size_2, p_size_2, p_size_2, F32(0x3f800000));
            F::EftSprAnim_SetScale(c, l_w_3, fp);
            break 'f 1i32;
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPart_SetSize", &PROBE_N, &[], true); }
    r_
}

pub fn EftAnimPart_SetDir(c: &mut Ctx, mut p_task_1: u32, mut p_dir_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPart_SetDir", &PROBE_N, &[(p_task_1) as u32, (p_dir_2) as u32], false); }
    let r_ = 'f: {
    if ((F::EftAnimPart_IsValid(c, p_task_1)) != 0) {
        { let a0_ = c.m.r32((p_task_1).wrapping_add(56)); F::EftSprAnim_SetDir(c, a0_, p_dir_2) };
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPart_SetDir", &PROBE_N, &[], true); }
    r_
}

pub fn EftAnimPart_SetDelay(c: &mut Ctx, mut p_task_1: u32, mut p_frames_2: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPart_SetDelay", &PROBE_N, &[(p_task_1) as u32, (p_frames_2).0], false); }
    let r_ = 'f: {
    if ((F::EftAnimPart_IsValid(c, p_task_1)) != 0) {
        { let v_ = p_frames_2; let a_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(156); c.m.w32(a_, (v_).0); };
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPart_SetDelay", &PROBE_N, &[], true); }
    r_
}

pub fn EftAnimPart_SetHold(c: &mut Ctx, mut p_task_1: u32, mut p_frames_2: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPart_SetHold", &PROBE_N, &[(p_task_1) as u32, (p_frames_2).0], false); }
    let r_ = 'f: {
    if ((F::EftAnimPart_IsValid(c, p_task_1)) != 0) {
        { let v_ = p_frames_2; let a_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(160); c.m.w32(a_, (v_).0); };
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPart_SetHold", &PROBE_N, &[], true); }
    r_
}

pub fn EftAnimPart_SetFade(c: &mut Ctx, mut p_task_1: u32, mut p_frames_2: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPart_SetFade", &PROBE_N, &[(p_task_1) as u32, (p_frames_2).0], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if ((F::EftAnimPart_IsValid(c, p_task_1)) != 0) {
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        { let v_ = p_frames_2; let a_ = (l_w_3).wrapping_add(168); c.m.w32(a_, (v_).0); };
        { let v_ = p_frames_2; let a_ = (l_w_3).wrapping_add(164); c.m.w32(a_, (v_).0); };
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPart_SetFade", &PROBE_N, &[], true); }
    r_
}

pub fn EftAnimPart_IsAlive(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPart_IsAlive", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    break 'f (((F::EftAnimPart_IsValid(c, p_task_1)) != (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPart_IsAlive", &PROBE_N, &[], true); }
    r_
}

pub fn EftAnimPartMgr_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPartMgr_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = F::BtlTask_CreateChildList(c, p_task_1, ((((32i32) as i64 as u64)) as u32), ((((((176u32) as i32)) as i64 as u64)) as u32)); let a_ = sym::gEftAnimPartList; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPartMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftAnimPartMgr_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPartMgr_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPartMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftAnimPartMgr_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPartMgr_Update", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPartMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftAnimPart_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPart_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        { let _ = F::memset(c, l_w_3, 0i32, 176u32); };
        { let s_ = p_arg_2; let d_ = (l_w_3).wrapping_add(80); rt::memcpy(c, d_, s_, 64); };
        { let a1_ = c.m.r32(((l_w_3).wrapping_add(80)).wrapping_add(52)); let a2_ = c.m.r32(((l_w_3).wrapping_add(80)).wrapping_add(56)); F::EftSprAnim_Create(c, l_w_3, a1_, a2_) };
        { let v_ = ((c.m.r16((p_task_1).wrapping_add(2))) as i32); let a_ = (l_w_3).wrapping_add(148); c.m.w32(a_, (v_) as u32); };
        { let r_ = 1i32; let a_ = (l_w_3).wrapping_add(152); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        if (((rt::fgt(F32(c.m.r32((p_arg_2).wrapping_add(8))), F32(0x00000000)) as i32)) != 0) {
            { let v_ = rt::fmul(F32(c.m.r32((p_arg_2).wrapping_add(8))), F32(0x41f00000)); let a_ = (l_w_3).wrapping_add(172); c.m.w32(a_, (v_).0); };
            { let r_ = 16i32; let a_ = (l_w_3).wrapping_add(152); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        { let a1_ = F32(c.m.r32(((l_w_3).wrapping_add(80)).wrapping_add(48))); let a2_ = F32(c.m.r32(((l_w_3).wrapping_add(80)).wrapping_add(48))); let a3_ = F32(c.m.r32(((l_w_3).wrapping_add(80)).wrapping_add(48))); F::Vec4_Set(c, fp, a1_, a2_, a3_, F32(0x3f800000)) };
        F::EftSprAnim_SetPos(c, l_w_3, ((l_w_3).wrapping_add(80)).wrapping_add(16));
        F::EftSprAnim_SetDir(c, l_w_3, ((l_w_3).wrapping_add(80)).wrapping_add(32));
        F::EftSprAnim_SetScale(c, l_w_3, fp);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPart_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftAnimPart_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPart_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32((p_task_1).wrapping_add(56)); F::EftSprAnim_Destroy(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPart_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftAnimPart_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPart_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPart_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftAnimPart_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPart_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_2: u32 = 0;
    let mut l_a_3: F32 = F32(0);
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_a_3 = F32(0x00000000);
        if ((((!(({ let a0_ = (c.m.r32((l_w_2).wrapping_add(80)) as i32); let a1_ = (c.m.r32((l_w_2).wrapping_add(144)) as i32); F::BtlScene_IsEffectStopped(c, a0_, a1_) }) != 0)) as i32)) != 0) {
            if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(156))), F32(0x00000000)) as i32)) != 0) {
                { let _ = F::EftSprAnim_Step(c, l_w_2); };
                { let r_ = 2i32; let a_ = (l_w_2).wrapping_add(152); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                if (((((c.m.r32((l_w_2).wrapping_add(152)) as i32)) & (16i32))) != 0) {
                    { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(172); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(172))), F32(0x00000000)) as i32)) != 0) {
                        { let r_ = 4i32; let a_ = (l_w_2).wrapping_add(152); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                }
                if ((((!((F::EftSprAnim_IsPlaying(c, l_w_2)) != 0)) as i32)) != 0) {
                    { let r_ = (!(2i32)); let a_ = (l_w_2).wrapping_add(152); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                    { let r_ = 4i32; let a_ = (l_w_2).wrapping_add(152); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            } else {
                { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(156); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            }
            if (((((c.m.r32((l_w_2).wrapping_add(152)) as i32)) & (4i32))) != 0) {
                if (((rt::fgt(F32(c.m.r32((l_w_2).wrapping_add(160))), F32(0x00000000)) as i32)) != 0) {
                    { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(160); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                } else {
                    if (((rt::fgt(F32(c.m.r32((l_w_2).wrapping_add(164))), F32(0x00000000)) as i32)) != 0) {
                        l_a_3 = rt::fdiv(F32(c.m.r32((l_w_2).wrapping_add(168))), F32(c.m.r32((l_w_2).wrapping_add(164))));
                        { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(168); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    }
                    if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(168))), F32(0x00000000)) as i32)) != 0) {
                        { let r_ = 8i32; let a_ = (l_w_2).wrapping_add(152); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        { let r_ = (!(1i32)); let a_ = (l_w_2).wrapping_add(152); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                        l_a_3 = F32(0x00000000);
                    }
                    F::Vec4_Set(c, fp, F32(0x3f800000), F32(0x3f800000), F32(0x3f800000), l_a_3);
                    F::EftSprAnim_SetColor(c, l_w_2, fp);
                }
            }
        }
        if (((((c.m.r32((l_w_2).wrapping_add(152)) as i32)) & (8i32))) != 0) {
            F::BtlTask_SetDead(c, p_task_1);
        } else {
            F::EftSprAnim_KeepTextures(c, l_w_2);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPart_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftAnimPart_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPart_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_layer_2: i32 = 0;
    let mut l_w_3: u32 = 0;
    let mut l_chr_4: i32 = 0;
    let mut l_other_5: i32 = 0;
    let r_ = 'f: {
    l_layer_2 = 0i32;
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    l_chr_4 = (c.m.r32((l_w_3).wrapping_add(80)) as i32);
    l_other_5 = 0i32;
    if (({ let a1_ = (c.m.r32((l_w_3).wrapping_add(144)) as i32); F::BtlScene_IsEffectHidden(c, l_chr_4, a1_) }) != 0) {
        break 'f;
    }
    if ((((!((F::EftCam_IsActive(c)) != 0)) as i32)) != 0) {
        l_other_5 = (((F::BtlScene_IsCharInView(c, l_chr_4)) == (0i32)) as i32);
    }
    { let o_ = l_other_5; let n_ = (o_).wrapping_add(1); l_other_5 = n_; };
    { let o_ = l_other_5; let n_ = (o_).wrapping_sub(1); l_other_5 = n_; };
    's1: {
        let sv6: i32 = (c.m.r32(((l_w_3).wrapping_add(80)).wrapping_add(4)) as i32);
        'k9: {
        'k8: {
        'k7: {
        'k6: {
        'k5: {
        'k4: {
        'k3: {
        'k2: {
            match sv6 { 1i32 => break 'k2, 3i32 => break 'k2, 4i32 => break 'k3, 5i32 => break 'k4, 6i32 => break 'k5, 0i32 => break 'k6, 2i32 => break 'k6, 7i32 => break 'k7, 8i32 => break 'k8, 9i32 => break 'k9, _ => break 's1, }
        }
        if ((l_other_5) != 0) {
            break 'f;
        }
        l_layer_2 = 5i32;
        break 's1;
        }
        l_layer_2 = 10i32;
        break 's1;
        }
        l_layer_2 = 6i32;
        break 's1;
        }
        if ((((!((l_other_5) != 0)) as i32)) != 0) {
            l_layer_2 = 5i32;
            break 's1;
        }
        }
        l_layer_2 = 9i32;
        break 's1;
        }
        l_layer_2 = 0i32;
        break 's1;
        }
        if ((l_other_5) != 0) {
            break 'f;
        }
        l_layer_2 = 17i32;
        break 's1;
        }
        l_layer_2 = 18i32;
        break 's1;
    }
    if (((((c.m.r32((l_w_3).wrapping_add(152)) as i32)) & (2i32))) != 0) {
        F::EftSprAnim_SetDrawFlags(c, l_w_3, l_layer_2);
        F::EftSprAnim_Draw(c, l_w_3);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPart_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftAnimPart_IsValid(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPart_IsValid", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((((c.m.r8(p_task_1)) as i32)) & (1i32))) as u8)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftAnimPart_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((((c.m.r32((l_w_2).wrapping_add(148)) as i32)) != (((c.m.r16((p_task_1).wrapping_add(2))) as i32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32((l_w_2).wrapping_add(152)) as i32)) & (1i32))) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAnimPart_IsValid", &PROBE_N, &[], true); }
    r_
}

pub fn EftOrbTailMgr_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTailMgr_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((4u32) as i32)) }; let a_ = sym::gEftOrbTail; c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = c.m.r32(sym::gEftOrbTail); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((47456u32) as i32)) }; let a_ = c.m.r32(sym::gEftOrbTail); c.m.w32(a_, v_); };
        { let _ = { let a0_ = { let a_ = c.m.r32(sym::gEftOrbTail); c.m.r32(a_) }; F::memset(c, a0_, 0i32, 47456u32) }; };
        { let a0_ = ({ let a_ = c.m.r32(sym::gEftOrbTail); c.m.r32(a_) }).wrapping_add(8); let a1_ = F::BtlScene_GetCommonEntry(c, 601i32); F::EftTexSet_Load16(c, a0_, a1_) };
        { let v_ = F::BtlScene_GetCommonEntry(c, 603i32); let a_ = { let a_ = c.m.r32(sym::gEftOrbTail); c.m.r32(a_) }; c.m.w32(a_, v_); };
        { let v_ = F::BtlScene_GetCommonEntry(c, 602i32); let a_ = ({ let a_ = c.m.r32(sym::gEftOrbTail); c.m.r32(a_) }).wrapping_add(4); c.m.w32(a_, v_); };
        {
            rt::memset(c, fp, 0, 28);
            { let v_ = 609i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
            { let v_ = 610i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
            { let v_ = 611i32; c.m.w32(fp.wrapping_add(8), (v_) as u32); }
            { let v_ = 612i32; c.m.w32(fp.wrapping_add(12), (v_) as u32); }
            { let v_ = 613i32; c.m.w32(fp.wrapping_add(16), (v_) as u32); }
            { let v_ = 614i32; c.m.w32(fp.wrapping_add(20), (v_) as u32); }
            { let v_ = 615i32; c.m.w32(fp.wrapping_add(24), (v_) as u32); }
            rt::memset(c, fp.wrapping_add(28), 0, 12);
            { let v_ = 605i32; c.m.w32(fp.wrapping_add(28).wrapping_add(0), (v_) as u32); }
            { let v_ = 607i32; c.m.w32(fp.wrapping_add(28).wrapping_add(4), (v_) as u32); }
            { let v_ = 608i32; c.m.w32(fp.wrapping_add(28).wrapping_add(8), (v_) as u32); }
            rt::memset(c, fp.wrapping_add(40), 0, 12);
            { let v_ = 604i32; c.m.w32(fp.wrapping_add(40).wrapping_add(0), (v_) as u32); }
            { let v_ = 606i32; c.m.w32(fp.wrapping_add(40).wrapping_add(4), (v_) as u32); }
            { let v_ = 606i32; c.m.w32(fp.wrapping_add(40).wrapping_add(8), (v_) as u32); }
            {
                l_i_2 = 0i32;
                'l1: loop {
                    if !(((((l_i_2) < (3i32)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        { let a0_ = (({ let a_ = c.m.r32(sym::gEftOrbTail); c.m.r32(a_) }).wrapping_add(272)).wrapping_add((((l_i_2) as u32)).wrapping_mul(264)); let a1_ = { let a0_ = (c.m.r32((fp.wrapping_add(40)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32); F::BtlScene_GetCommonEntry(c, a0_) }; F::EftTexSet_Load16(c, a0_, a1_) };
                    }
                    { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                }
            }
            {
                l_i_2 = 0i32;
                'l3: loop {
                    if !(((((l_i_2) < (3i32)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        { let v_ = { let a0_ = (c.m.r32((fp.wrapping_add(28)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32); F::BtlScene_GetCommonEntry(c, a0_) }; let a_ = (({ let a_ = c.m.r32(sym::gEftOrbTail); c.m.r32(a_) }).wrapping_add(1064)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    }
                    { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                }
            }
            {
                l_i_2 = 0i32;
                'l5: loop {
                    if !(((((l_i_2) < (7i32)) as i32)) != 0) { break 'l5; }
                    'c6: {
                        { let v_ = { let a0_ = (c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32); F::BtlScene_GetCommonEntry(c, a0_) }; let a_ = (({ let a_ = c.m.r32(sym::gEftOrbTail); c.m.r32(a_) }).wrapping_add(1076)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    }
                    { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                }
            }
        }
        { let v_ = F::BtlTask_CreateChildList(c, p_task_1, ((((2i32) as i64 as u64)) as u32), ((((480i32) as i64 as u64)) as u32)); let a_ = ({ let a_ = c.m.r32(sym::gEftOrbTail); c.m.r32(a_) }).wrapping_add(47448); c.m.w32(a_, v_); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftOrbTailMgr_Init", &PROBE_N, &[], true); }
    r_
}
