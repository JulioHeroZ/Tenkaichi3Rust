#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EftTexSet_Load8(c: &mut Ctx, mut p_set_1: u32, mut p_pack_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTexSet_Load8", &PROBE_N, &[(p_set_1) as u32, (p_pack_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_count_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
        { let _ = F::memset(c, p_set_1, 0i32, 136u32); };
        F::Res_RelocateOffsets(c, fp, p_pack_2, p_pack_2);
        l_count_3 = { let a_ = c.m.r32(fp); (c.m.r32(a_) as i32) };
        { let v_ = l_count_3; let a_ = (p_set_1).wrapping_add(128); c.m.w32(a_, (v_) as u32); };
        F::EftTexSet_CheckCount(c, l_count_3, 8i32);
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < ((c.m.r32((p_set_1).wrapping_add(128)) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((l_i_4) as u32)).wrapping_mul(64)); let a_ = ((p_set_1).wrapping_add((((l_i_4) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, v_); };
                    { let v_ = { let a_ = (({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((l_i_4) as u32)).wrapping_mul(64))).wrapping_add(48); c.m.r64(a_) }; let a_ = (p_set_1).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)); c.m.w64(a_, v_); };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftTexSet_Load8", &PROBE_N, &[], true); }
    r_
}

pub fn EftTexSet_Load16(c: &mut Ctx, mut p_set_1: u32, mut p_pack_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTexSet_Load16", &PROBE_N, &[(p_set_1) as u32, (p_pack_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_count_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
        { let _ = F::memset(c, p_set_1, 0i32, 264u32); };
        F::Res_RelocateOffsets(c, fp, p_pack_2, p_pack_2);
        l_count_3 = { let a_ = c.m.r32(fp); (c.m.r32(a_) as i32) };
        { let v_ = l_count_3; let a_ = (p_set_1).wrapping_add(256); c.m.w32(a_, (v_) as u32); };
        F::EftTexSet_CheckCount(c, l_count_3, 16i32);
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < ((c.m.r32((p_set_1).wrapping_add(256)) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((l_i_4) as u32)).wrapping_mul(64)); let a_ = ((p_set_1).wrapping_add((((l_i_4) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, v_); };
                    { let v_ = { let a_ = (({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((l_i_4) as u32)).wrapping_mul(64))).wrapping_add(48); c.m.r64(a_) }; let a_ = (p_set_1).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)); c.m.w64(a_, v_); };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftTexSet_Load16", &PROBE_N, &[], true); }
    r_
}

pub fn EftTexSet_Load34(c: &mut Ctx, mut p_set_1: u32, mut p_pack_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTexSet_Load34", &PROBE_N, &[(p_set_1) as u32, (p_pack_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_count_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
        { let _ = F::memset(c, p_set_1, 0i32, 552u32); };
        F::Res_RelocateOffsets(c, fp, p_pack_2, p_pack_2);
        l_count_3 = { let a_ = c.m.r32(fp); (c.m.r32(a_) as i32) };
        { let v_ = l_count_3; let a_ = (p_set_1).wrapping_add(544); c.m.w32(a_, (v_) as u32); };
        F::EftTexSet_CheckCount(c, l_count_3, 34i32);
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < ((c.m.r32((p_set_1).wrapping_add(544)) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((l_i_4) as u32)).wrapping_mul(64)); let a_ = ((p_set_1).wrapping_add((((l_i_4) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, v_); };
                    { let v_ = { let a_ = (({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((l_i_4) as u32)).wrapping_mul(64))).wrapping_add(48); c.m.r64(a_) }; let a_ = (p_set_1).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)); c.m.w64(a_, v_); };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftTexSet_Load34", &PROBE_N, &[], true); }
    r_
}

pub fn EftVram_GetUsed(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVram_GetUsed", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftVram)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f ({ let a_ = (c.m.r32(sym::gEftVram)).wrapping_add(2052); (c.m.r32(a_) as i32) }).wrapping_sub(11008i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftVram_GetUsed", &PROBE_N, &[], true); }
    r_
}

pub fn EftVram_GetCapacity(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVram_GetCapacity", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f 5375i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftVram_GetCapacity", &PROBE_N, &[], true); }
    r_
}

pub fn EftVram_Upload(c: &mut Ctx, mut p_tex_1: u32, mut p_imageBlock_2: i32, mut p_clutBlock_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVram_Upload", &PROBE_N, &[(p_tex_1) as u32, (p_imageBlock_2) as u32, (p_clutBlock_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_img_4: u32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 48);
        { let v_ = ((268435458i32) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((1342177282i32) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((32769i32) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((80i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        l_img_4 = c.m.r32((p_tex_1).wrapping_add(8));
        if (((((((((p_imageBlock_2) >= (0i32)) as i32)) != 0) && ((((((c.m.r32((l_img_4).wrapping_add(8)) as i32)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
            { let v_ = (((((((((p_imageBlock_2) as i64)).wrapping_shl((32i32) as u32)) as u64)) | ((((c.m.r32((l_img_4).wrapping_add(24))) as u64)).wrapping_shl((48i32) as u32)))) as u32); let a_ = (fp).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            { let v_ = ((((((((((p_imageBlock_2) as i64)).wrapping_shl((32i32) as u32)) as u64)) | ((((c.m.r32((l_img_4).wrapping_add(24))) as u64)).wrapping_shl((48i32) as u32)))).wrapping_shr((32i32) as u32)) as u32); let a_ = (fp).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            F::Dma_AddData(c, fp, ((48u32) as i32));
            { let a0_ = c.m.r32((l_img_4).wrapping_add(56)); let a1_ = (c.m.r32((l_img_4).wrapping_add(8)) as i32); F::Dma_AddRef(c, a0_, a1_) };
        }
        if (((((((((p_clutBlock_3) >= (0i32)) as i32)) != 0) && ((((((c.m.r32((l_img_4).wrapping_add(12)) as i32)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
            { let v_ = (((((((((p_clutBlock_3) as i64)).wrapping_shl((32i32) as u32)) as u64)) | ((((c.m.r32((l_img_4).wrapping_add(28))) as u64)).wrapping_shl((48i32) as u32)))) as u32); let a_ = (fp).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            { let v_ = ((((((((((p_clutBlock_3) as i64)).wrapping_shl((32i32) as u32)) as u64)) | ((((c.m.r32((l_img_4).wrapping_add(28))) as u64)).wrapping_shl((48i32) as u32)))).wrapping_shr((32i32) as u32)) as u32); let a_ = (fp).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            F::Dma_AddData(c, fp, ((48u32) as i32));
            { let a0_ = c.m.r32((l_img_4).wrapping_add(60)); let a1_ = (c.m.r32((l_img_4).wrapping_add(12)) as i32); F::Dma_AddRef(c, a0_, a1_) };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftVram_Upload", &PROBE_N, &[], true); }
    r_
}

pub fn EftVolleyAim_Lerp(c: &mut Ctx, mut p_a_1: F32, mut p_b_2: F32, mut p_t_3: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVolleyAim_Lerp", &PROBE_N, &[(p_a_1).0, (p_b_2).0, (p_t_3).0], false); }
    let r_ = 'f: {
    break 'f rt::fadd(p_a_1, rt::fmul(rt::fsub(p_b_2, p_a_1), p_t_3));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "EftVolleyAim_Lerp", &PROBE_N, &[], true); }
    r_
}

pub fn EftVolleyAim_GetFighterDistSq(c: &mut Ctx) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVolleyAim_GetFighterDistSq", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let r_ = 'f: {
        F::BtlCharApi_GetBasePos(c, 0i32, fp);
        F::BtlCharApi_GetBasePos(c, 1i32, fp.wrapping_add(16));
        F::Vec4_Sub(c, fp.wrapping_add(32), fp, fp.wrapping_add(16));
        break 'f F::Vec3_Dot(c, fp.wrapping_add(32), fp.wrapping_add(32));
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftVolleyAim_GetFighterDistSq", &PROBE_N, &[], true); }
    r_
}

pub fn EftVolleyAim_GetToOpponent(c: &mut Ctx, mut p_objId_1: i32, mut p_out_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVolleyAim_GetToOpponent", &PROBE_N, &[(p_objId_1) as u32, (p_out_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let r_ = 'f: {
        { let a0_ = F::BtlCharApi_GetOpponentObjId(c, p_objId_1); F::BtlCharApi_GetBasePos(c, a0_, fp) };
        F::BtlCharApi_GetBasePos(c, p_objId_1, fp.wrapping_add(16));
        F::Vec4_Sub(c, p_out_2, fp, fp.wrapping_add(16));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftVolleyAim_GetToOpponent", &PROBE_N, &[], true); }
    r_
}

pub fn EftVolleyAim_GetNodeSide(c: &mut Ctx, mut p_node_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVolleyAim_GetNodeSide", &PROBE_N, &[(p_node_1) as u32], false); }
    let mut l_side_2: i32 = 0;
    let r_ = 'f: {
    l_side_2 = 0i32;
    's1: {
        let sv3: i32 = p_node_1;
        match sv3 {
            18i32 | 19i32 | 20i32 | 21i32 | 22i32 | 23i32 | 24i32 | 25i32 | 26i32 | 27i32 | 28i32 | 29i32 | 30i32 | 31i32 | 54i32 => {
                l_side_2 = 0i32;
                break 's1;
            }
            32i32 | 33i32 | 34i32 | 35i32 | 36i32 | 37i32 | 38i32 | 39i32 | 40i32 | 41i32 | 42i32 | 43i32 | 44i32 | 45i32 => {
                l_side_2 = 1i32;
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f l_side_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftVolleyAim_GetNodeSide", &PROBE_N, &[], true); }
    r_
}

pub fn EftVolleyAim_InitShot(c: &mut Ctx, mut p_objId_1: i32, mut p_shot_2: u32, mut p_kind_3: i32, mut p_side_4: i32, mut p_index_5: i32, mut p_count_6: i32, mut p_speed_7: F32, mut p_maxTurn_8: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVolleyAim_InitShot", &PROBE_N, &[(p_objId_1) as u32, (p_shot_2) as u32, (p_kind_3) as u32, (p_side_4) as u32, (p_index_5) as u32, (p_count_6) as u32, (p_speed_7).0, (p_maxTurn_8).0], false); }
    let mut l_script_9: i32 = 0;
    let r_ = 'f: {
    l_script_9 = { let a3_ = F::BtlCharApi_IsLockedOn(c, p_objId_1); F::EftVolleyAim_Spread(c, p_objId_1, (p_shot_2).wrapping_add(16), p_kind_3, a3_, p_side_4, p_index_5, p_count_6) };
    { let v_ = p_speed_7; let a_ = (p_shot_2).wrapping_add(36); c.m.w32(a_, (v_).0); };
    { let v_ = p_maxTurn_8; let a_ = (p_shot_2).wrapping_add(40); c.m.w32(a_, (v_).0); };
    { let v_ = l_script_9; let a_ = (p_shot_2).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftVolleyAim_InitShot", &PROBE_N, &[], true); }
    r_
}

pub fn EftVolleyAim_Spread(c: &mut Ctx, mut p_objId_1: i32, mut p_dir_2: u32, mut p_kind_3: i32, mut p_lockedOn_4: i32, mut p_side_5: i32, mut p_index_6: i32, mut p_count_7: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVolleyAim_Spread", &PROBE_N, &[(p_objId_1) as u32, (p_dir_2) as u32, (p_kind_3) as u32, (p_lockedOn_4) as u32, (p_side_5) as u32, (p_index_6) as u32, (p_count_7) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_jitV_8: F32 = F32(0);
    let mut l_jitH_9: F32 = F32(0);
    let mut l_script_10: i32 = 0;
    let mut l_distSq_11: F32 = F32(0);
    let mut l_sprH_12: F32 = F32(0);
    let mut l_sprV_13: F32 = F32(0);
    let mut l_yaw_14: F32 = F32(0);
    let mut l_pitch_15: F32 = F32(0);
    let mut l_a_16: F32 = F32(0);
    let mut l_t_17: F32 = F32(0);
    let mut l_half_18: i32 = 0;
    let mut l_rx_19: F32 = F32(0);
    let mut l_ry_20: F32 = F32(0);
    let r_ = 'f: {
        l_jitV_8 = F32(0x3c8efa34);
        l_jitH_9 = l_jitV_8;
        l_script_10 = (1i32).wrapping_neg();
        l_distSq_11 = F::EftVolleyAim_GetFighterDistSq(c);
        l_sprH_12 = F32(0x3e5f66f1);
        l_sprV_13 = F32(0x3e32b8c1);
        if (((((p_kind_3) == (9i32)) as i32)) != 0) {
            break 'f (1i32).wrapping_neg();
        }
        if (((((p_kind_3) == (7i32)) as i32)) != 0) {
            l_sprH_12 = F32(0x3ddf66f1);
            l_jitH_9 = F32(0x3c0efa34);
        } else {
            if (((((p_kind_3) == (8i32)) as i32)) != 0) {
                l_sprH_12 = F32(0x3ddf66f1);
                l_jitH_9 = F32(0x3c0efa34);
            } else {
                if (((((p_kind_3) == (10i32)) as i32)) != 0) {
                    l_sprH_12 = F32(0x3b8efa33);
                    l_jitH_9 = F32(0x39b702d1);
                    l_sprV_13 = F32(0x3c0efa33);
                    l_jitV_8 = F32(0x3a64c385);
                } else {
                    if (((((p_kind_3) == (12i32)) as i32)) != 0) {
                        l_jitH_9 = F32(0x3a64c385);
                        l_sprH_12 = F32(0x3c32b8c0);
                        l_sprV_13 = F32(0x3c0efa33);
                        l_jitV_8 = F32(0x3a64c385);
                    } else {
                        if (((((p_index_6) == (0i32)) as i32)) != 0) {
                            l_sprH_12 = F32(0x3cb2b8c0);
                            l_sprV_13 = F32(0x3c8efa33);
                        } else {
                            if (((rt::flt(l_distSq_11, F32(0x48742400)) as i32)) != 0) {
                                if (((rt::flt(F32(0x461c4000), l_distSq_11) as i32)) != 0) {
                                    l_t_17 = F::EftVolleyAim_Lerp(c, F32(0x3f000000), F32(0x3f800000), rt::fdiv(rt::fsub(l_distSq_11, F32(0x461c4000)), F32(0x486a6000)));
                                } else {
                                    l_t_17 = F32(0x3f000000);
                                }
                                { let r_ = l_t_17; let n_ = rt::fmul(l_sprH_12, r_); l_sprH_12 = n_; };
                                { let r_ = l_t_17; let n_ = rt::fmul(l_sprV_13, r_); l_sprV_13 = n_; };
                            }
                        }
                    }
                }
            }
        }
        's1: {
            let sv21: i32 = p_kind_3;
            match sv21 {
                0i32 => {
                    l_yaw_14 = F::BtlScene_RandRangeF(c, rt::fsub(rt::fneg(l_sprH_12), l_jitH_9), rt::fadd(l_sprH_12, l_jitH_9));
                    l_pitch_15 = F::BtlScene_RandRangeF(c, rt::fsub(rt::fneg(l_sprV_13), l_jitV_8), rt::fadd(l_sprV_13, l_jitV_8));
                    break 's1;
                }
                1i32 => {
                    l_yaw_14 = rt::fadd(rt::fsub(rt::fmul(rt::fdiv(rt::fmul(l_sprH_12, F32(0x40000000)), rt::i2f(p_count_7)), rt::i2f(p_index_6)), l_sprH_12), F::BtlScene_RandRangeF(c, rt::fneg(l_jitH_9), l_jitH_9));
                    l_pitch_15 = F::BtlScene_RandRangeF(c, rt::fmul(rt::fsub(rt::fneg(l_sprV_13), l_jitV_8), F32(0x3f000000)), rt::fmul(rt::fadd(l_sprV_13, l_jitV_8), F32(0x3f000000)));
                    break 's1;
                }
                2i32 => {
                    if (((((p_index_6) >= (4i32)) as i32)) != 0) {
                        l_t_17 = F::Port_powf(c, F32(0x3f000000), rt::i2f(rt::div_i32(p_index_6, 4i32)));
                        { let r_ = l_t_17; let n_ = rt::fmul(l_sprH_12, r_); l_sprH_12 = n_; };
                        { let r_ = l_t_17; let n_ = rt::fmul(l_sprV_13, r_); l_sprV_13 = n_; };
                    }
                    l_yaw_14 = rt::fadd(l_sprH_12, F::BtlScene_RandRangeF(c, rt::fneg(l_jitH_9), l_jitH_9));
                    if (((((rt::rem_i32(p_index_6, 2i32)) != (p_side_5)) as i32)) != 0) {
                        l_yaw_14 = rt::fneg(l_yaw_14);
                    }
                    l_pitch_15 = F::BtlScene_RandRangeF(c, rt::fsub(rt::fneg(l_sprV_13), l_jitV_8), rt::fadd(l_sprV_13, l_jitV_8));
                    break 's1;
                }
                3i32 => {
                    l_a_16 = F::BtlScene_RandRangeF(c, F32(0xc0490fda), F32(0x40490fda));
                    l_yaw_14 = rt::fadd(rt::fmul(l_sprH_12, F::Mathf_SinFast(c, l_a_16)), F::BtlScene_RandRangeF(c, rt::fneg(l_jitH_9), l_jitH_9));
                    l_pitch_15 = rt::fadd(rt::fmul(l_sprV_13, F::Mathf_CosFast(c, l_a_16)), F::BtlScene_RandRangeF(c, rt::fneg(l_jitV_8), l_jitV_8));
                    break 's1;
                }
                4i32 => {
                    l_a_16 = rt::fdiv(F32(0x40c90fda), rt::i2f((p_count_7).wrapping_add(2i32)));
                    l_half_18 = rt::div_i32(p_index_6, 2i32);
                    if (((((rt::rem_i32(p_index_6, 2i32)) == (p_side_5)) as i32)) != 0) {
                        l_a_16 = rt::fadd(rt::fmul(l_a_16, rt::i2f((l_half_18).wrapping_add(1i32))), F32(0xbfc90fda));
                    } else {
                        l_a_16 = F::EftMath_WrapAngle(c, rt::fsub(F32(0xbfc90fda), rt::fmul(l_a_16, rt::i2f((l_half_18).wrapping_add(1i32)))));
                    }
                    l_yaw_14 = rt::fadd(rt::fmul(l_sprH_12, F::Mathf_CosFast(c, l_a_16)), F::BtlScene_RandRangeF(c, rt::fneg(l_jitH_9), l_jitH_9));
                    l_pitch_15 = rt::fadd(rt::fmul(l_sprV_13, F::Mathf_SinFast(c, l_a_16)), F::BtlScene_RandRangeF(c, rt::fneg(l_jitV_8), l_jitV_8));
                    break 's1;
                }
                5i32 => {
                    l_a_16 = rt::fdiv(F32(0x40490fda), rt::i2f((p_count_7).wrapping_sub(1i32)));
                    l_half_18 = rt::div_i32(p_index_6, 2i32);
                    if (((((rt::rem_i32(p_index_6, 2i32)) == (p_side_5)) as i32)) != 0) {
                        l_a_16 = rt::fmul(l_a_16, rt::i2f(l_half_18));
                    } else {
                        l_a_16 = rt::fsub(F32(0x40490fda), rt::fmul(l_a_16, rt::i2f(l_half_18)));
                    }
                    l_yaw_14 = rt::fadd(rt::fmul(l_sprH_12, F::Mathf_CosFast(c, l_a_16)), F::BtlScene_RandRangeF(c, rt::fneg(l_jitH_9), l_jitH_9));
                    l_pitch_15 = rt::fadd(rt::fmul(l_sprV_13, F::Mathf_SinFast(c, l_a_16)), F::BtlScene_RandRangeF(c, rt::fneg(l_jitV_8), l_jitV_8));
                    break 's1;
                }
                6i32 => {
                    l_a_16 = rt::fdiv(F32(0x40490fda), rt::i2f((p_count_7).wrapping_sub(1i32)));
                    if (((((rt::rem_i32(p_index_6, 2i32)) == (p_side_5)) as i32)) != 0) {
                        l_a_16 = rt::fmul(l_a_16, rt::i2f(rt::div_i32(p_index_6, 2i32)));
                    } else {
                        l_a_16 = rt::fsub(F32(0x40490fda), rt::fmul(l_a_16, rt::i2f(rt::div_i32(p_index_6, 2i32))));
                    }
                    if (((((rt::rem_i32(rt::div_i32(p_index_6, 2i32), 2i32)) == (1i32)) as i32)) != 0) {
                        { let r_ = F32(0x3f000000); let n_ = rt::fmul(l_sprH_12, r_); l_sprH_12 = n_; };
                        { let r_ = F32(0x3f000000); let n_ = rt::fmul(l_sprV_13, r_); l_sprV_13 = n_; };
                    }
                    l_yaw_14 = rt::fadd(rt::fmul(l_sprH_12, F::Mathf_CosFast(c, l_a_16)), F::BtlScene_RandRangeF(c, rt::fneg(l_jitH_9), l_jitH_9));
                    l_pitch_15 = rt::fadd(rt::fmul(l_sprV_13, F::Mathf_SinFast(c, l_a_16)), F::BtlScene_RandRangeF(c, rt::fneg(l_jitV_8), l_jitV_8));
                    break 's1;
                }
                7i32 => {
                    l_pitch_15 = F32(0x3fc90fda);
                    l_script_10 = 0i32;
                    l_yaw_14 = F::BtlScene_RandRangeF(c, rt::fsub(rt::fneg(l_sprH_12), l_jitH_9), rt::fadd(l_sprH_12, l_jitH_9));
                    { let v_ = F32(0x00000000); let a_ = (p_dir_2).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    break 's1;
                }
                8i32 => {
                    l_pitch_15 = F32(0x3f490fda);
                    l_script_10 = 1i32;
                    l_yaw_14 = F::BtlScene_RandRangeF(c, rt::fsub(rt::fneg(l_sprH_12), l_jitH_9), rt::fadd(l_sprH_12, l_jitH_9));
                    { let v_ = F32(0x00000000); let a_ = (p_dir_2).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    break 's1;
                }
                10i32 => {
                    l_yaw_14 = rt::fadd(l_sprH_12, l_jitH_9);
                    l_pitch_15 = F::BtlScene_RandRangeF(c, rt::fsub(rt::fneg(l_sprV_13), l_jitV_8), rt::fadd(l_sprV_13, l_jitV_8));
                    if ((((p_index_6) & (1i32))) != 0) {
                        l_yaw_14 = rt::fneg(l_yaw_14);
                    }
                    break 's1;
                }
                11i32 => {
                    l_yaw_14 = rt::fadd(rt::fsub(l_sprH_12, rt::fmul(rt::fdiv(rt::fmul(l_sprH_12, F32(0x40000000)), rt::i2f(p_count_7)), rt::i2f(p_index_6))), F::BtlScene_RandRangeF(c, rt::fneg(l_jitH_9), l_jitH_9));
                    l_pitch_15 = F::BtlScene_RandRangeF(c, rt::fmul(rt::fsub(rt::fneg(l_sprV_13), l_jitV_8), F32(0x3f000000)), rt::fmul(rt::fadd(l_sprV_13, l_jitV_8), F32(0x3f000000)));
                    break 's1;
                }
                12i32 => {
                    if (((((p_index_6) == (0i32)) as i32)) != 0) {
                        l_yaw_14 = F32(0x00000000);
                    } else {
                        if ((((p_index_6) & (1i32))) != 0) {
                            l_yaw_14 = rt::fadd(rt::fadd(rt::fmul(rt::i2f(rt::div_i32(p_index_6, 2i32)), l_sprH_12), l_sprH_12), F::BtlScene_RandRangeF(c, F32(0x00000000), l_jitH_9));
                        } else {
                            l_yaw_14 = rt::fneg(rt::fadd(rt::fadd(rt::fmul(rt::i2f(rt::div_i32(p_index_6, 2i32)), l_sprH_12), l_sprH_12), F::BtlScene_RandRangeF(c, F32(0x00000000), l_jitH_9)));
                        }
                    }
                    l_pitch_15 = F::BtlScene_RandRangeF(c, rt::fsub(rt::fneg(l_sprV_13), l_jitV_8), rt::fadd(l_sprV_13, l_jitV_8));
                    break 's1;
                }
                _ => {
                    l_yaw_14 = F32(0x00000000);
                    l_pitch_15 = l_yaw_14;
                    break 's1;
                }
            }
        }
        if ((((!(((((((((p_kind_3) == (7i32)) as i32)) != 0) || (((((p_kind_3) == (8i32)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
            if (((rt::flt(F::BtlCharApi_GetAltitude(c, p_objId_1), F32(0x41f00000)) as i32)) != 0) {
                if (((rt::flt(l_pitch_15, F32(0x00000000)) as i32)) != 0) {
                    { let r_ = F32(0x3ecccccc); let n_ = rt::fmul(l_pitch_15, r_); l_pitch_15 = n_; };
                }
            }
        }
        {
            rt::memset(c, fp.wrapping_add(64), 0, 16);
            rt::memset(c, fp.wrapping_add(64).wrapping_add(0), 0, 16);
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(0).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(0).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(64).wrapping_add(0).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(64).wrapping_add(0).wrapping_add(12), (v_).0); }
            F::Vec3_Clamp(c, p_dir_2, p_dir_2, F32(0xbf800000), F32(0x3f800000));
            l_rx_19 = { let a0_ = rt::fneg(F32(c.m.r32((p_dir_2).wrapping_add(4)))); F::Mathf_Asin(c, a0_) };
            l_ry_20 = { let a0_ = F32(c.m.r32(p_dir_2)); let a1_ = F32(c.m.r32((p_dir_2).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) };
            l_rx_19 = F::EftMath_WrapAngle(c, rt::fadd(l_rx_19, l_pitch_15));
            l_ry_20 = F::EftMath_WrapAngle(c, rt::fadd(l_ry_20, l_yaw_14));
            F::Mtx_StoreIdentity(c, fp);
            F::Mtx_RotateX(c, fp, fp, l_rx_19);
            F::Mtx_RotateY(c, fp, fp, l_ry_20);
            F::Mtx_MulVec4(c, p_dir_2, fp, fp.wrapping_add(64));
        }
        break 'f l_script_10;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftVolleyAim_Spread", &PROBE_N, &[], true); }
    r_
}

pub fn EftVolleyAim_Steer(c: &mut Ctx, mut p_shot_1: u32, mut p_objId_2: i32, mut p_offset_3: u32, mut p_frontOnly_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVolleyAim_Steer", &PROBE_N, &[(p_shot_1) as u32, (p_objId_2) as u32, (p_offset_3) as u32, (p_frontOnly_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_opp_5: i32 = 0;
    let mut l_t_6: F32 = F32(0);
    let mut l_dot_7: F32 = F32(0);
    let mut l_c_8: F32 = F32(0);
    let r_ = 'f: {
        l_opp_5 = F::BtlCharApi_GetOpponentObjId(c, p_objId_2);
        if (((((l_opp_5) < (0i32)) as i32)) != 0) {
            break 'f;
        }
        F::BtlCharApi_GetNodePos(c, l_opp_5, 17i32, fp);
        if (((((p_offset_3) != (((0i32) as u32))) as i32)) != 0) {
            F::Vec3_Add(c, fp, fp, p_offset_3);
        }
        F::Vec4_Sub(c, fp.wrapping_add(32), fp, p_shot_1);
        l_t_6 = rt::fdiv({ let a0_ = F::Vec3_Dot(c, fp.wrapping_add(32), fp.wrapping_add(32)); F::Port_sqrtf(c, a0_) }, F32(c.m.r32((p_shot_1).wrapping_add(36))));
        if (((rt::flt(F32(0x41700000), l_t_6) as i32)) != 0) {
            l_t_6 = F32(0x41700000);
        }
        F::BtlCharApi_GetFrameMove(c, l_opp_5, fp.wrapping_add(16));
        F::Vec4_Scale(c, fp.wrapping_add(16), fp.wrapping_add(16), rt::fmul(l_t_6, F32(0x3f000000)));
        F::Vec4_Add(c, fp, fp, fp.wrapping_add(16));
        F::Vec4_Sub(c, fp.wrapping_add(32), fp, p_shot_1);
        F::Vec3_Normalize(c, fp.wrapping_add(32), fp.wrapping_add(32));
        F::Vec3_Normalize(c, (p_shot_1).wrapping_add(16), (p_shot_1).wrapping_add(16));
        l_dot_7 = F::Vec3_Dot(c, fp.wrapping_add(32), (p_shot_1).wrapping_add(16));
        if (((((((((p_frontOnly_4) == (0i32)) as i32)) != 0) || (((((((((p_frontOnly_4) == (1i32)) as i32)) != 0) && (((rt::flt(F32(0x00000000), l_dot_7) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
            if (((rt::flt(l_dot_7, F32(0x00000000)) as i32)) != 0) {
                l_c_8 = F32(0x00000000);
            } else {
                if (((rt::flt(F32(0x3f800000), l_dot_7) as i32)) != 0) {
                    l_c_8 = F32(0x3f800000);
                } else {
                    l_c_8 = l_dot_7;
                }
            }
            l_t_6 = F::Mathf_Acos(c, l_c_8);
            if (((rt::flt(F32(c.m.r32((p_shot_1).wrapping_add(40))), l_t_6) as i32)) != 0) {
                l_t_6 = F32(c.m.r32((p_shot_1).wrapping_add(40)));
            }
            F::Vec3_Cross(c, fp.wrapping_add(48), (p_shot_1).wrapping_add(16), fp.wrapping_add(32));
            F::Vec3_Normalize(c, fp.wrapping_add(48), fp.wrapping_add(48));
            F::Vec3_RotateAxis(c, (p_shot_1).wrapping_add(16), (p_shot_1).wrapping_add(16), fp.wrapping_add(48), l_t_6);
            F::Vec3_Normalize(c, (p_shot_1).wrapping_add(16), (p_shot_1).wrapping_add(16));
        }
        { let a2_ = F32(c.m.r32((p_shot_1).wrapping_add(36))); F::Vec4_Scale(c, fp.wrapping_add(64), (p_shot_1).wrapping_add(16), a2_) };
        F::Vec4_Add(c, p_shot_1, p_shot_1, fp.wrapping_add(64));
        { let v_ = F32(0x3f800000); let a_ = (p_shot_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftVolleyAim_Steer", &PROBE_N, &[], true); }
    r_
}

pub fn EftVolleyAim_Update(c: &mut Ctx, mut p_objId_1: i32, mut p_shot_2: u32, mut p_offset_3: u32, mut p_exact_4: i32, mut p_time_5: F32, mut p_speed_6: F32, mut p_maxTurn_7: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVolleyAim_Update", &PROBE_N, &[(p_objId_1) as u32, (p_shot_2) as u32, (p_offset_3) as u32, (p_exact_4) as u32, (p_time_5).0, (p_speed_6).0, (p_maxTurn_7).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_p_8: u32 = 0;
    let mut l_frontOnly_9: i32 = 0;
    let mut l_lenSq_10: F32 = F32(0);
    let mut l_scale_11: F32 = F32(0);
    let mut l_r_12: F32 = F32(0);
    let mut l_t_13: F32 = F32(0);
    let mut l_a_14: F32 = F32(0);
    let mut l_b_15: F32 = F32(0);
    let r_ = 'f: {
        l_p_8 = ((0i32) as u32);
        l_frontOnly_9 = 0i32;
        if ((((((((c.m.r32((p_shot_2).wrapping_add(32)) as i32)) as u32)) < (((2i32) as u32))) as i32)) != 0) {
            l_p_8 = { let a_ = (sym::gEftVolleyAimScripts).wrapping_add(((((c.m.r32((p_shot_2).wrapping_add(32)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) };
        }
        if (((((l_p_8) != (((0i32) as u32))) as i32)) != 0) {
            {
                'l1: loop {
                    if !((((((c.m.r32(l_p_8) as i32)) != (0i32)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        if (((((((rt::fle(rt::i2f((c.m.r8((l_p_8).wrapping_add(4))) as i32), p_time_5) as i32)) != 0) && (((rt::fle(p_time_5, rt::i2f((c.m.r8((l_p_8).wrapping_add(5))) as i32)) as i32)) != 0)) as i32)) != 0) {
                            's3: {
                                let sv16: i32 = (c.m.r32(l_p_8) as i32);
                                match sv16 {
                                    1i32 => {
                                        l_a_14 = rt::fmul(rt::fmul(p_maxTurn_7, rt::i2f((c.m.r8((l_p_8).wrapping_add(6))) as i32)), F32(0x3dcccccc));
                                        l_b_15 = rt::fmul(rt::fmul(p_maxTurn_7, rt::i2f((c.m.r8((l_p_8).wrapping_add(7))) as i32)), F32(0x3dcccccc));
                                        l_t_13 = F32(0x3f800000);
                                        if (((rt::fne(p_time_5, rt::i2f((c.m.r8((l_p_8).wrapping_add(4))) as i32)) as i32)) != 0) {
                                            l_t_13 = rt::fdiv(rt::fsub(p_time_5, rt::i2f((c.m.r8((l_p_8).wrapping_add(4))) as i32)), rt::i2f((((c.m.r8((l_p_8).wrapping_add(5))) as i32)).wrapping_sub(((c.m.r8((l_p_8).wrapping_add(4))) as i32))));
                                        }
                                        { let v_ = F::EftVolleyAim_Lerp(c, l_a_14, l_b_15, l_t_13); let a_ = (p_shot_2).wrapping_add(40); c.m.w32(a_, (v_).0); };
                                        break 's3;
                                    }
                                    2i32 => {
                                        l_a_14 = rt::fmul(rt::fmul(p_speed_6, rt::i2f((c.m.r8((l_p_8).wrapping_add(6))) as i32)), F32(0x3dcccccc));
                                        l_b_15 = rt::fmul(rt::fmul(p_speed_6, rt::i2f((c.m.r8((l_p_8).wrapping_add(7))) as i32)), F32(0x3dcccccc));
                                        l_t_13 = F32(0x3f800000);
                                        if (((rt::fne(p_time_5, rt::i2f((c.m.r8((l_p_8).wrapping_add(4))) as i32)) as i32)) != 0) {
                                            l_t_13 = rt::fdiv(rt::fsub(p_time_5, rt::i2f((c.m.r8((l_p_8).wrapping_add(4))) as i32)), rt::i2f((((c.m.r8((l_p_8).wrapping_add(5))) as i32)).wrapping_sub(((c.m.r8((l_p_8).wrapping_add(4))) as i32))));
                                        }
                                        { let v_ = F::EftVolleyAim_Lerp(c, l_a_14, l_b_15, l_t_13); let a_ = (p_shot_2).wrapping_add(36); c.m.w32(a_, (v_).0); };
                                        break 's3;
                                    }
                                    3i32 => {
                                        if ((((((((({ let a1_ = ((c.m.r8((l_p_8).wrapping_add(7))) as i32); F::BtlScene_RandRange(c, 0i32, a1_) }) == (0i32)) as i32)) != 0) || ((p_exact_4) != 0)) as i32)) != 0) {
                                            F::Vec4_Set(c, p_offset_3, F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x3f800000));
                                            l_r_12 = rt::fmul(rt::i2f((c.m.r8((l_p_8).wrapping_add(6))) as i32), F32(0x3d23d70a));
                                        } else {
                                            if ((((({ let a1_ = (((c.m.r8((l_p_8).wrapping_add(7))) as i32)).wrapping_shr((1i32) as u32); F::BtlScene_RandRange(c, 0i32, a1_) }) == (0i32)) as i32)) != 0) {
                                                F::Vec4_Set(c, p_offset_3, F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x3f800000));
                                                l_r_12 = rt::fmul(rt::i2f((c.m.r8((l_p_8).wrapping_add(6))) as i32), F32(0x3ecccccc));
                                            } else {
                                                F::EftVolleyAim_GetToOpponent(c, p_objId_1, p_offset_3);
                                                l_lenSq_10 = F::Vec3_LengthSq(c, p_offset_3);
                                                l_scale_11 = rt::fmul((if (((rt::flt(l_lenSq_10, rt::i2f((((c.m.r8((l_p_8).wrapping_add(6))) as i32)).wrapping_mul(((c.m.r8((l_p_8).wrapping_add(6))) as i32)))) as i32)) != 0) { F::Port_sqrtf(c, l_lenSq_10) } else { rt::i2f((c.m.r8((l_p_8).wrapping_add(6))) as i32) }), F32(0xbf000000));
                                                F::Vec3_Normalize(c, p_offset_3, p_offset_3);
                                                F::Vec3_Scale(c, p_offset_3, p_offset_3, l_scale_11);
                                                l_r_12 = rt::i2f((c.m.r8((l_p_8).wrapping_add(6))) as i32);
                                            }
                                        }
                                        { let r_ = F::BtlScene_RandRangeF(c, rt::fneg(l_r_12), l_r_12); let a_ = p_offset_3; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                        { let v_ = F32(0x00000000); let a_ = (p_offset_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                        { let r_ = F::BtlScene_RandRangeF(c, rt::fneg(l_r_12), l_r_12); let a_ = (p_offset_3).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                        break 's3;
                                    }
                                    4i32 => {
                                        F::EftVolleyAim_GetToOpponent(c, p_objId_1, p_offset_3);
                                        { let v_ = F32(0x00000000); let a_ = (p_offset_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                        F::Vec3_Normalize(c, p_offset_3, p_offset_3);
                                        F::Vec3_Scale(c, p_offset_3, p_offset_3, F32(0xc47a0000));
                                        break 's3;
                                    }
                                    5i32 => {
                                        F::EftVolleyAim_GetToOpponent(c, p_objId_1, p_offset_3);
                                        { let v_ = F32(0x00000000); let a_ = (p_offset_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                        F::Vec3_Normalize(c, p_offset_3, p_offset_3);
                                        F::Vec3_Scale(c, p_offset_3, p_offset_3, F32(0xc47a0000));
                                        F::Vec3_Normalize(c, fp, p_shot_2);
                                        { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                        F::Vec4_Scale(c, fp, fp, F32(0xc3c80000));
                                        F::Vec4_Add(c, p_offset_3, p_offset_3, fp);
                                        { let r_ = F32(0x447a0000); let a_ = (p_offset_3).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                        break 's3;
                                    }
                                    6i32 => {
                                        if (((((((c.m.r8((l_p_8).wrapping_add(6))) as i32)) != (0i32)) as i32)) != 0) {
                                            l_frontOnly_9 = 1i32;
                                        }
                                        break 's3;
                                    }
                                    _ => {
                                    }
                                }
                            }
                        }
                    }
                    { let o_ = l_p_8; let n_ = (o_).wrapping_add(8); l_p_8 = n_; };
                }
            }
        }
        F::EftVolleyAim_Steer(c, p_shot_2, p_objId_1, p_offset_3, l_frontOnly_9);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftVolleyAim_Update", &PROBE_N, &[], true); }
    r_
}

pub fn BtlBodyHit_TestVolumes(c: &mut Ctx, mut p_work_1: u32, mut p_body_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlBodyHit_TestVolumes", &PROBE_N, &[(p_work_1) as u32, (p_body_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_part_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < ((c.m.r32((p_work_1).wrapping_add(106672)) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_part_3 = c.m.r32(p_body_2);
                    'l3: loop {
                        'c4: {
                            if ((F::ColSphere_ContactObb(c, ((p_work_1).wrapping_add(98464)).wrapping_add((((l_i_4) as u32)).wrapping_mul(32)), (l_part_3).wrapping_add(64), fp, fp.wrapping_add(16), fp.wrapping_add(32))) != 0) {
                                break 'f 1i32;
                            }
                        }
                        if !((((!(((((c.m.r32({ let o_ = l_part_3; let n_ = (o_).wrapping_add(464); l_part_3 = n_; o_ }) as i32)) & (1i32))) != 0)) as i32)) != 0) { break 'l3; }
                    }
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        {
            l_i_4 = 0i32;
            'l5: loop {
                if !(((((l_i_4) < ((c.m.r32((p_work_1).wrapping_add(106676)) as i32))) as i32)) != 0) { break 'l5; }
                'c6: {
                    l_part_3 = c.m.r32(p_body_2);
                    'l7: loop {
                        'c8: {
                            if ((F::ColObb_Overlaps(c, ((p_work_1).wrapping_add(99072)).wrapping_add((((l_i_4) as u32)).wrapping_mul(400)), (l_part_3).wrapping_add(64))) != 0) {
                                break 'f 1i32;
                            }
                        }
                        if !((((!(((((c.m.r32({ let o_ = l_part_3; let n_ = (o_).wrapping_add(464); l_part_3 = n_; o_ }) as i32)) & (1i32))) != 0)) as i32)) != 0) { break 'l7; }
                    }
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlBodyHit_TestVolumes", &PROBE_N, &[], true); }
    r_
}

pub fn BtlBodyHit_Test(c: &mut Ctx, mut p_atk_1: u32, mut p_def_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlBodyHit_Test", &PROBE_N, &[(p_atk_1) as u32, (p_def_2) as u32], false); }
    let mut l_work_3: u32 = 0;
    let mut l_defWork_4: u32 = 0;
    let mut l_body_5: u32 = 0;
    let r_ = 'f: {
    if ((((((((c.m.r8((p_atk_1).wrapping_add(3245)) as i8)) as i32)) < (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_work_3 = c.m.r32((p_atk_1).wrapping_add(5728));
    l_defWork_4 = c.m.r32((p_def_2).wrapping_add(5728));
    if (((((l_work_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((l_defWork_4) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_body_5 = (p_def_2).wrapping_add(3920);
    if (((((l_body_5) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((F::ColBox_Overlaps(c, (l_work_3).wrapping_add(98432), (p_def_2).wrapping_add(4032))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((F::BtlBodyHit_TestVolumes(c, l_work_3, l_body_5)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let r_ = (1i32).wrapping_shl((((c.m.r32((p_def_2).wrapping_add(16)) as i32)).wrapping_add(24i32)) as u32); let a_ = (l_work_3).wrapping_add(98336); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    { let r_ = (1i32).wrapping_shl((((c.m.r32((p_atk_1).wrapping_add(16)) as i32)).wrapping_add(28i32)) as u32); let a_ = (l_defWork_4).wrapping_add(98336); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlBodyHit_Test", &PROBE_N, &[], true); }
    r_
}

pub fn BtlBodyHit_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlBodyHit_Update", &PROBE_N, &[], false); }
    let mut l_a_1: u32 = 0;
    let r_ = 'f: {
    l_a_1 = { let a0_ = F::BattleSide_GetObjId(c, 0i32); F::BtlObj_Get(c, a0_) };
    { let _ = { let a1_ = { let a0_ = F::BattleSide_GetObjId(c, 1i32); F::BtlObj_Get(c, a0_) }; F::BtlBodyHit_Test(c, l_a_1, a1_) }; };
    l_a_1 = { let a0_ = F::BattleSide_GetObjId(c, 1i32); F::BtlObj_Get(c, a0_) };
    { let _ = { let a1_ = { let a0_ = F::BattleSide_GetObjId(c, 0i32); F::BtlObj_Get(c, a0_) }; F::BtlBodyHit_Test(c, l_a_1, a1_) }; };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlBodyHit_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftDet_PrepareSpheres(c: &mut Ctx, mut p_shape_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDet_PrepareSpheres", &PROBE_N, &[(p_shape_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_a_2: u32 = 0;
    let mut l_b_3: u32 = 0;
    let r_ = 'f: {
        l_a_2 = c.m.r32((p_shape_1).wrapping_add(12));
        l_b_3 = c.m.r32((p_shape_1).wrapping_add(16));
        { let a3_ = F32(c.m.r32((l_a_2).wrapping_add(16))); F::ColCapsule_Set(c, fp, l_b_3, l_a_2, a3_) };
        F::ColBounds_OfCapsule(c, (p_shape_1).wrapping_add(132), fp);
        F::Vec3_Sub(c, (p_shape_1).wrapping_add(160), l_a_2, l_b_3);
        F::Vec3_Normalize(c, (p_shape_1).wrapping_add(176), (p_shape_1).wrapping_add(160));
        { let v_ = { let a0_ = (c.m.r32((p_shape_1).wrapping_add(128)) as i32); F::BtlStage_FindZoneNear(c, a0_, l_a_2) }; let a_ = (p_shape_1).wrapping_add(128); c.m.w32(a_, (v_) as u32); };
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftDet_PrepareSpheres", &PROBE_N, &[], true); }
    r_
}

pub fn EftDet_PrepareCapsule(c: &mut Ctx, mut p_shape_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDet_PrepareCapsule", &PROBE_N, &[(p_shape_1) as u32], false); }
    let mut l_a_2: u32 = 0;
    let mut l_end_3: u32 = 0;
    let r_ = 'f: {
    l_a_2 = c.m.r32((p_shape_1).wrapping_add(12));
    l_end_3 = (l_a_2).wrapping_add(16);
    F::ColBounds_OfCapsule(c, (p_shape_1).wrapping_add(132), l_a_2);
    F::Vec3_Sub(c, (p_shape_1).wrapping_add(160), l_end_3, l_a_2);
    F::Vec3_Normalize(c, (p_shape_1).wrapping_add(176), (p_shape_1).wrapping_add(160));
    { let v_ = { let a0_ = (c.m.r32((p_shape_1).wrapping_add(128)) as i32); F::BtlStage_FindZoneNear(c, a0_, l_end_3) }; let a_ = (p_shape_1).wrapping_add(128); c.m.w32(a_, (v_) as u32); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDet_PrepareCapsule", &PROBE_N, &[], true); }
    r_
}

pub fn EftDet_PrepareNone(c: &mut Ctx, mut p_shape_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDet_PrepareNone", &PROBE_N, &[(p_shape_1) as u32], false); }
    let r_ = 'f: {
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDet_PrepareNone", &PROBE_N, &[], true); }
    r_
}

pub fn EftDet_PrepareAll(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDet_PrepareAll", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_list_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 28);
        { let v_ = fid::EftDet_PrepareSpheres; c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = fid::EftDet_PrepareCapsule; c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = fid::EftDet_PrepareNone; c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = fid::EftDet_PrepareNone; c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = fid::EftDet_PrepareNone; c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = fid::EftDet_PrepareNone; c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = fid::EftDet_PrepareNone; c.m.w32(fp.wrapping_add(24), v_); }
        l_list_1 = F::EftHit_GetList(c);
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < ((c.m.r32((l_list_1).wrapping_add(25600)) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = 0i32; let a_ = (((l_list_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(400))).wrapping_add(112)).wrapping_add(200); c.m.w32(a_, (v_) as u32); };
                    { let v_ = F32(c.m.r32((sym::D_002FEAF8).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (((l_list_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(400))).wrapping_add(112)).wrapping_add(196); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((sym::D_002FEAF8).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (((l_list_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(400))).wrapping_add(112)).wrapping_add(192); c.m.w32(a_, (v_).0); };
                    { let _ = { let fp_ = { let a_ = (fp).wrapping_add(((((c.m.r32(((l_list_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(400))).wrapping_add(112)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; ((D::call(c, fp_, &[((((l_list_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(400))).wrapping_add(112)) as u64)])) as i32) }; };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftDet_PrepareAll", &PROBE_N, &[], true); }
    r_
}

pub fn EftDet_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDet_Update", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
        break 'f;
    }
    F::EftDet_PrepareAll(c);
    F::EftDet_ClashPass(c);
    F::EftDet_StagePass(c);
    F::EftDet_FighterPass(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDet_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftDet_SpheresVsFighter(c: &mut Ctx, mut p_obj_1: u32, mut p_shape_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDet_SpheresVsFighter", &PROBE_N, &[(p_obj_1) as u32, (p_shape_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_body_3: u32 = 0;
    let mut l_hits_4: i32 = 0;
    let mut l_best_5: F32 = F32(0);
    let mut l_b_6: u32 = 0;
    let mut l_part_7: u32 = 0;
    let r_ = 'f: {
        l_body_3 = (p_obj_1).wrapping_add(3920);
        l_hits_4 = 0i32;
        l_best_5 = F32(0x3f800000);
        l_b_6 = c.m.r32((p_shape_2).wrapping_add(16));
        if ((((!((F::ColBox_Overlaps(c, (p_shape_2).wrapping_add(132), (p_obj_1).wrapping_add(4056))) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        {
            l_part_7 = c.m.r32(l_body_3);
            'l1: loop {
                'c2: {
                    if ((F::ColSphere_SweepSphere(c, l_b_6, (l_part_7).wrapping_add(32), (p_shape_2).wrapping_add(160), sym::D_002EC2C0, fp)) != 0) {
                        if (((((((rt::fle(F32(c.m.r32(fp)), F32(0x358637bd)) as i32)) != 0) || (((rt::fle(F32(0x3f800000), F32(c.m.r32(fp))) as i32)) != 0)) as i32)) != 0) {
                            if ((F::ColSphere_TestSphere(c, l_b_6, (l_part_7).wrapping_add(32))) != 0) {
                                F::Vec4_Copy(c, (p_shape_2).wrapping_add(32), l_b_6);
                                F::Vec4_Copy(c, (p_shape_2).wrapping_add(48), l_b_6);
                                { let o_ = l_hits_4; let n_ = (o_).wrapping_add(1); l_hits_4 = n_; };
                                break 'l1;
                            }
                        } else {
                            if (((rt::flt(F32(c.m.r32(fp)), l_best_5) as i32)) != 0) {
                                { let o_ = l_hits_4; let n_ = (o_).wrapping_add(1); l_hits_4 = n_; };
                                { let v_ = rt::fadd(F32(c.m.r32(l_b_6)), rt::fmul(F32(c.m.r32(fp)), F32(c.m.r32((p_shape_2).wrapping_add(160))))); let a_ = (p_shape_2).wrapping_add(32); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fadd(F32(c.m.r32((l_b_6).wrapping_add(4))), rt::fmul(F32(c.m.r32(fp)), F32(c.m.r32(((p_shape_2).wrapping_add(160)).wrapping_add(4))))); let a_ = ((p_shape_2).wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fadd(F32(c.m.r32((l_b_6).wrapping_add(8))), rt::fmul(F32(c.m.r32(fp)), F32(c.m.r32(((p_shape_2).wrapping_add(160)).wrapping_add(8))))); let a_ = ((p_shape_2).wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = ((p_shape_2).wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fadd(F32(c.m.r32((p_shape_2).wrapping_add(32))), rt::fmul(F32(c.m.r32((l_b_6).wrapping_add(16))), F32(c.m.r32((p_shape_2).wrapping_add(176))))); let a_ = (p_shape_2).wrapping_add(48); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fadd(F32(c.m.r32(((p_shape_2).wrapping_add(32)).wrapping_add(4))), rt::fmul(F32(c.m.r32((l_b_6).wrapping_add(16))), F32(c.m.r32(((p_shape_2).wrapping_add(176)).wrapping_add(4))))); let a_ = ((p_shape_2).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fadd(F32(c.m.r32(((p_shape_2).wrapping_add(32)).wrapping_add(8))), rt::fmul(F32(c.m.r32((l_b_6).wrapping_add(16))), F32(c.m.r32(((p_shape_2).wrapping_add(176)).wrapping_add(8))))); let a_ = ((p_shape_2).wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                l_best_5 = F32(c.m.r32(fp));
                            }
                        }
                    }
                    if (((((c.m.r32(l_part_7) as i32)) & (1i32))) != 0) {
                        break 'l1;
                    }
                }
                { let o_ = l_part_7; let n_ = (o_).wrapping_add(464); l_part_7 = n_; };
            }
        }
        if (((((l_hits_4) != (0i32)) as i32)) != 0) {
            { let v_ = F::Vec3_Dist(c, l_b_6, (p_shape_2).wrapping_add(32)); let a_ = (p_shape_2).wrapping_add(192); c.m.w32(a_, (v_).0); };
            break 'f 1i32;
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftDet_SpheresVsFighter", &PROBE_N, &[], true); }
    r_
}

pub fn EftDet_CapsuleVsFighter(c: &mut Ctx, mut p_obj_1: u32, mut p_shape_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDet_CapsuleVsFighter", &PROBE_N, &[(p_obj_1) as u32, (p_shape_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_body_3: u32 = 0;
    let mut l_hits_4: i32 = 0;
    let mut l_best_5: F32 = F32(0);
    let mut l_a_6: u32 = 0;
    let mut l_part_7: u32 = 0;
    let r_ = 'f: {
        l_body_3 = (p_obj_1).wrapping_add(3920);
        l_hits_4 = 0i32;
        l_best_5 = F32(0x3f800000);
        l_a_6 = c.m.r32((p_shape_2).wrapping_add(12));
        if ((((!((F::ColBox_Overlaps(c, (p_shape_2).wrapping_add(132), (p_obj_1).wrapping_add(4056))) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        { let a2_ = F32(c.m.r32((l_a_6).wrapping_add(32))); F::ColSphere_Set(c, fp, l_a_6, a2_) };
        {
            l_part_7 = c.m.r32(l_body_3);
            'l1: loop {
                'c2: {
                    if ((F::ColSphere_SweepSphere(c, fp, (l_part_7).wrapping_add(32), (p_shape_2).wrapping_add(160), sym::D_002EC2C0, fp.wrapping_add(32))) != 0) {
                        if (((((((rt::fle(F32(c.m.r32(fp.wrapping_add(32))), F32(0x358637bd)) as i32)) != 0) || (((rt::fle(F32(0x3f800000), F32(c.m.r32(fp.wrapping_add(32)))) as i32)) != 0)) as i32)) != 0) {
                            if ((F::ColSphere_TestSphere(c, fp, (l_part_7).wrapping_add(32))) != 0) {
                                { let o_ = l_hits_4; let n_ = (o_).wrapping_add(1); l_hits_4 = n_; };
                                F::Vec4_Copy(c, (p_shape_2).wrapping_add(32), fp);
                                F::Vec4_Copy(c, (p_shape_2).wrapping_add(48), fp);
                                break 'l1;
                            }
                        } else {
                            if (((rt::flt(F32(c.m.r32(fp.wrapping_add(32))), l_best_5) as i32)) != 0) {
                                { let o_ = l_hits_4; let n_ = (o_).wrapping_add(1); l_hits_4 = n_; };
                                { let v_ = rt::fadd(F32(c.m.r32(fp)), rt::fmul(F32(c.m.r32(fp.wrapping_add(32))), F32(c.m.r32((p_shape_2).wrapping_add(160))))); let a_ = (p_shape_2).wrapping_add(32); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fadd(F32(c.m.r32((fp).wrapping_add(4))), rt::fmul(F32(c.m.r32(fp.wrapping_add(32))), F32(c.m.r32(((p_shape_2).wrapping_add(160)).wrapping_add(4))))); let a_ = ((p_shape_2).wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fadd(F32(c.m.r32((fp).wrapping_add(8))), rt::fmul(F32(c.m.r32(fp.wrapping_add(32))), F32(c.m.r32(((p_shape_2).wrapping_add(160)).wrapping_add(8))))); let a_ = ((p_shape_2).wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = ((p_shape_2).wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fadd(F32(c.m.r32((p_shape_2).wrapping_add(32))), rt::fmul(F32(c.m.r32((fp).wrapping_add(16))), F32(c.m.r32((p_shape_2).wrapping_add(176))))); let a_ = (p_shape_2).wrapping_add(48); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fadd(F32(c.m.r32(((p_shape_2).wrapping_add(32)).wrapping_add(4))), rt::fmul(F32(c.m.r32((fp).wrapping_add(16))), F32(c.m.r32(((p_shape_2).wrapping_add(176)).wrapping_add(4))))); let a_ = ((p_shape_2).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fadd(F32(c.m.r32(((p_shape_2).wrapping_add(32)).wrapping_add(8))), rt::fmul(F32(c.m.r32((fp).wrapping_add(16))), F32(c.m.r32(((p_shape_2).wrapping_add(176)).wrapping_add(8))))); let a_ = ((p_shape_2).wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = ((p_shape_2).wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                l_best_5 = F32(c.m.r32(fp.wrapping_add(32)));
                            }
                        }
                    }
                    if (((((c.m.r32(l_part_7) as i32)) & (1i32))) != 0) {
                        break 'l1;
                    }
                }
                { let o_ = l_part_7; let n_ = (o_).wrapping_add(464); l_part_7 = n_; };
            }
        }
        if (((((l_hits_4) != (0i32)) as i32)) != 0) {
            { let v_ = F::Vec3_Dist(c, l_a_6, (p_shape_2).wrapping_add(32)); let a_ = (p_shape_2).wrapping_add(192); c.m.w32(a_, (v_).0); };
            break 'f 1i32;
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftDet_CapsuleVsFighter", &PROBE_N, &[], true); }
    r_
}

pub fn EftDet_NoneVsFighter(c: &mut Ctx, mut p_obj_1: u32, mut p_shape_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDet_NoneVsFighter", &PROBE_N, &[(p_obj_1) as u32, (p_shape_2) as u32], false); }
    let r_ = 'f: {
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDet_NoneVsFighter", &PROBE_N, &[], true); }
    r_
}

pub fn EftDet_HitFighter(c: &mut Ctx, mut p_rec_1: u32, mut p_target_2: i32, mut p_idx_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDet_HitFighter", &PROBE_N, &[(p_rec_1) as u32, (p_target_2) as u32, (p_idx_3) as u32], false); }
    let r_ = 'f: {
    if ((F::BtlColl_TryGuard(c, p_target_2, p_rec_1)) != 0) {
        if ((F::EftHit_RearmImpact(c, p_rec_1)) != 0) {
            { F::EftHit_SetTaskFlag(c, ((p_idx_3) as u32), 2i32, ((p_rec_1).wrapping_add(112)).wrapping_add(48)); { let _ = 0i32; }; };
        }
    } else {
        if ((F::BtlColl_Hit(c, p_target_2, p_rec_1)) != 0) {
            if ((F::EftHit_RearmImpact(c, p_rec_1)) != 0) {
                { F::EftHit_SetTaskFlag(c, ((p_idx_3) as u32), 1i32, ((p_rec_1).wrapping_add(112)).wrapping_add(48)); { let _ = 0i32; }; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDet_HitFighter", &PROBE_N, &[], true); }
    r_
}

pub fn EftDet_HitFighterMulti(c: &mut Ctx, mut p_rec_1: u32, mut p_target_2: i32, mut p_idx_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDet_HitFighterMulti", &PROBE_N, &[(p_rec_1) as u32, (p_target_2) as u32, (p_idx_3) as u32], false); }
    let mut l_interval_4: i32 = 0;
    let mut l_maxHits_5: i32 = 0;
    let mut l_frames_6: i32 = 0;
    let mut l_hits_7: i32 = 0;
    let r_ = 'f: {
    F::EftHit_IncCountA(c, p_rec_1);
    l_interval_4 = F::EftHit_GetDefUnkB(c, p_rec_1);
    l_maxHits_5 = F::EftHit_GetMaxHits(c, p_rec_1);
    l_frames_6 = F::EftHit_GetCountA(c, p_rec_1);
    l_hits_7 = F::EftHit_GetHitCount(c, p_rec_1);
    if (((((l_maxHits_5) < (l_hits_7)) as i32)) != 0) {
        break 'f { F::EftHit_SetTaskFlag(c, ((p_idx_3) as u32), 1i32, ((p_rec_1).wrapping_add(112)).wrapping_add(48)); 0i32 };
    }
    if (((((((((l_hits_7) > (0i32)) as i32)) != 0) && (((((l_frames_6) < (l_interval_4)) as i32)) != 0)) as i32)) != 0) {
        if ((((!((F::EftHit_HasDefFlag20(c, p_rec_1)) != 0)) as i32)) != 0) {
            { F::EftHit_SetTaskFlag(c, ((p_idx_3) as u32), 32768i32, ((p_rec_1).wrapping_add(112)).wrapping_add(48)); { let _ = 0i32; }; };
        }
    } else {
        if ((F::BtlColl_TryGuard(c, p_target_2, p_rec_1)) != 0) {
            F::EftHit_IncHitCount(c, p_rec_1);
            F::EftHit_SetCountA(c, p_rec_1, 0i32);
            if ((((!((F::EftHit_HasDefFlag20(c, p_rec_1)) != 0)) as i32)) != 0) {
                { F::EftHit_SetTaskFlag(c, ((p_idx_3) as u32), 32768i32, ((p_rec_1).wrapping_add(112)).wrapping_add(48)); { let _ = 0i32; }; };
            }
            if ((F::EftHit_RearmImpact(c, p_rec_1)) != 0) {
                { F::EftHit_SetTaskFlag(c, ((p_idx_3) as u32), 2i32, ((p_rec_1).wrapping_add(112)).wrapping_add(48)); { let _ = 0i32; }; };
            }
        } else {
            if ((F::BtlColl_Hit(c, p_target_2, p_rec_1)) != 0) {
                F::EftHit_IncHitCount(c, p_rec_1);
                F::EftHit_SetCountA(c, p_rec_1, 0i32);
                if ((((!((F::EftHit_HasDefFlag20(c, p_rec_1)) != 0)) as i32)) != 0) {
                    { F::EftHit_SetTaskFlag(c, ((p_idx_3) as u32), 32768i32, ((p_rec_1).wrapping_add(112)).wrapping_add(48)); { let _ = 0i32; }; };
                }
                if ((F::EftHit_RearmImpact(c, p_rec_1)) != 0) {
                    { F::EftHit_SetTaskFlag(c, ((p_idx_3) as u32), 1i32, ((p_rec_1).wrapping_add(112)).wrapping_add(48)); { let _ = 0i32; }; };
                }
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDet_HitFighterMulti", &PROBE_N, &[], true); }
    r_
}

pub fn EftDet_FighterPass(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDet_FighterPass", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_list_1: u32 = 0;
    let mut l_rec_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_target_4: i32 = 0;
    let mut l_ok_5: i32 = 0;
    let mut l_obj_6: u32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 28);
        { let v_ = fid::EftDet_SpheresVsFighter; c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = fid::EftDet_CapsuleVsFighter; c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = fid::EftDet_NoneVsFighter; c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = fid::EftDet_NoneVsFighter; c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = fid::EftDet_NoneVsFighter; c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = fid::EftDet_NoneVsFighter; c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = fid::EftDet_NoneVsFighter; c.m.w32(fp.wrapping_add(24), v_); }
        l_list_1 = F::EftHit_GetList(c);
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < ((c.m.r32((l_list_1).wrapping_add(25600)) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_rec_2 = (l_list_1).wrapping_add((((l_i_3) as u32)).wrapping_mul(400));
                    if ((F::EftHit_CanHit(c, l_rec_2, 1i32)) != 0) {
                        l_target_4 = (((c.m.r32((l_list_1).wrapping_add((((l_i_3) as u32)).wrapping_mul(400))) as i32)) ^ (1i32));
                        if (((((l_target_4) == (0i32)) as i32)) != 0) {
                            l_obj_6 = { let a0_ = F::BattleSide_GetObjId(c, 0i32); F::BtlObj_Get(c, a0_) };
                        } else {
                            l_obj_6 = { let a0_ = F::BattleSide_GetObjId(c, 1i32); F::BtlObj_Get(c, a0_) };
                        }
                        if (({ let fp_ = { let a_ = (fp).wrapping_add(((((c.m.r32(((l_list_1).wrapping_add((((l_i_3) as u32)).wrapping_mul(400))).wrapping_add(112)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; ((D::call(c, fp_, &[((l_obj_6) as u64), (((l_rec_2).wrapping_add(112)) as u64)])) as i32) }) != 0) {
                            l_ok_5 = 0i32;
                            if ((((((((!(((((c.m.r32((((l_list_1).wrapping_add((((l_i_3) as u32)).wrapping_mul(400))).wrapping_add(112)).wrapping_add(200)) as i32)) & (1i32))) != 0)) as i32)) != 0) || (((rt::flt(F32(c.m.r32((((l_list_1).wrapping_add((((l_i_3) as u32)).wrapping_mul(400))).wrapping_add(112)).wrapping_add(192))), F32(c.m.r32((((l_list_1).wrapping_add((((l_i_3) as u32)).wrapping_mul(400))).wrapping_add(112)).wrapping_add(196)))) as i32)) != 0)) as i32)) != 0) {
                                l_ok_5 = 1i32;
                            }
                            if ((l_ok_5) != 0) {
                                { let r_ = 2i32; let a_ = (((l_list_1).wrapping_add((((l_i_3) as u32)).wrapping_mul(400))).wrapping_add(112)).wrapping_add(200); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                F::EftHit_MarkLastHit(c, l_rec_2);
                                if ((F::BtlColl_TryDodge(c, l_target_4, l_rec_2)) != 0) {
                                    break 'c2;
                                }
                                if ((F::BtlColl_TryDeflect(c, l_target_4, l_rec_2)) != 0) {
                                    { F::EftHit_SetTaskFlag(c, ((l_i_3) as u32), 64i32, ((l_rec_2).wrapping_add(112)).wrapping_add(48)); { let _ = 0i32; }; };
                                } else {
                                    if ((F::BtlColl_TryReflect(c, l_target_4, l_rec_2)) != 0) {
                                        { F::EftHit_SetTaskFlag(c, ((l_i_3) as u32), 128i32, ((l_rec_2).wrapping_add(112)).wrapping_add(48)); { let _ = 0i32; }; };
                                    } else {
                                        if ((F::BtlColl_TryAbsorb(c, l_target_4, l_rec_2)) != 0) {
                                            { F::EftHit_SetTaskFlag(c, ((l_i_3) as u32), 32i32, ((l_rec_2).wrapping_add(112)).wrapping_add(48)); { let _ = 0i32; }; };
                                        } else {
                                            if ((F::EftHit_IsMultiHit(c, l_rec_2)) != 0) {
                                                { let _ = F::EftDet_HitFighterMulti(c, l_rec_2, l_target_4, l_i_3); };
                                            } else {
                                                F::EftDet_HitFighter(c, l_rec_2, l_target_4, l_i_3);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftDet_FighterPass", &PROBE_N, &[], true); }
    r_
}

pub fn EftDet_SpheresVsSpheres(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDet_SpheresVsSpheres", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_sa_3: u32 = 0;
    let mut l_sb_4: u32 = 0;
    let r_ = 'f: {
        l_sa_3 = c.m.r32((p_a_1).wrapping_add(16));
        l_sb_4 = c.m.r32((p_b_2).wrapping_add(16));
        if ((((!((F::ColBox_Overlaps(c, (p_a_1).wrapping_add(132), (p_b_2).wrapping_add(132))) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((F::ColSphere_SweepSphere(c, l_sa_3, l_sb_4, (p_a_1).wrapping_add(160), (p_b_2).wrapping_add(160), fp.wrapping_add(16))) != 0) {
            if (((((((rt::fle(F32(c.m.r32(fp.wrapping_add(16))), F32(0x358637bd)) as i32)) != 0) || (((rt::fle(F32(0x3f800000), F32(c.m.r32(fp.wrapping_add(16)))) as i32)) != 0)) as i32)) != 0) {
                if ((F::ColSphere_SeparateSphere(c, l_sa_3, l_sb_4, (p_a_1).wrapping_add(32), (p_b_2).wrapping_add(32), fp)) != 0) {
                    { let v_ = rt::fadd(F32(c.m.r32((p_a_1).wrapping_add(32))), rt::fmul(F32(c.m.r32(fp)), F32(c.m.r32((l_sa_3).wrapping_add(16))))); let a_ = (p_a_1).wrapping_add(48); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32(((p_a_1).wrapping_add(32)).wrapping_add(4))), rt::fmul(F32(c.m.r32((fp).wrapping_add(4))), F32(c.m.r32((l_sa_3).wrapping_add(16))))); let a_ = ((p_a_1).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32(((p_a_1).wrapping_add(32)).wrapping_add(8))), rt::fmul(F32(c.m.r32((fp).wrapping_add(8))), F32(c.m.r32((l_sa_3).wrapping_add(16))))); let a_ = ((p_a_1).wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = ((p_a_1).wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    F::Vec4_Copy(c, (p_b_2).wrapping_add(48), (p_a_1).wrapping_add(48));
                    break 'f 1i32;
                }
            } else {
                { let v_ = rt::fadd(F32(c.m.r32(l_sa_3)), rt::fmul(F32(c.m.r32(fp.wrapping_add(16))), F32(c.m.r32((p_a_1).wrapping_add(160))))); let a_ = (p_a_1).wrapping_add(32); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((l_sa_3).wrapping_add(4))), rt::fmul(F32(c.m.r32(fp.wrapping_add(16))), F32(c.m.r32(((p_a_1).wrapping_add(160)).wrapping_add(4))))); let a_ = ((p_a_1).wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((l_sa_3).wrapping_add(8))), rt::fmul(F32(c.m.r32(fp.wrapping_add(16))), F32(c.m.r32(((p_a_1).wrapping_add(160)).wrapping_add(8))))); let a_ = ((p_a_1).wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = ((p_a_1).wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((p_a_1).wrapping_add(32))), rt::fmul(F32(c.m.r32((l_sa_3).wrapping_add(16))), F32(c.m.r32((p_a_1).wrapping_add(176))))); let a_ = (p_a_1).wrapping_add(48); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32(((p_a_1).wrapping_add(32)).wrapping_add(4))), rt::fmul(F32(c.m.r32((l_sa_3).wrapping_add(16))), F32(c.m.r32(((p_a_1).wrapping_add(176)).wrapping_add(4))))); let a_ = ((p_a_1).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32(((p_a_1).wrapping_add(32)).wrapping_add(8))), rt::fmul(F32(c.m.r32((l_sa_3).wrapping_add(16))), F32(c.m.r32(((p_a_1).wrapping_add(176)).wrapping_add(8))))); let a_ = ((p_a_1).wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = ((p_a_1).wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32(l_sb_4)), rt::fmul(F32(c.m.r32(fp.wrapping_add(16))), F32(c.m.r32((p_b_2).wrapping_add(160))))); let a_ = (p_b_2).wrapping_add(32); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((l_sb_4).wrapping_add(4))), rt::fmul(F32(c.m.r32(fp.wrapping_add(16))), F32(c.m.r32(((p_b_2).wrapping_add(160)).wrapping_add(4))))); let a_ = ((p_b_2).wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((l_sb_4).wrapping_add(8))), rt::fmul(F32(c.m.r32(fp.wrapping_add(16))), F32(c.m.r32(((p_b_2).wrapping_add(160)).wrapping_add(8))))); let a_ = ((p_b_2).wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = ((p_b_2).wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((p_b_2).wrapping_add(32))), rt::fmul(F32(c.m.r32((l_sb_4).wrapping_add(16))), F32(c.m.r32((p_b_2).wrapping_add(176))))); let a_ = (p_b_2).wrapping_add(48); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32(((p_b_2).wrapping_add(32)).wrapping_add(4))), rt::fmul(F32(c.m.r32((l_sb_4).wrapping_add(16))), F32(c.m.r32(((p_b_2).wrapping_add(176)).wrapping_add(4))))); let a_ = ((p_b_2).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32(((p_b_2).wrapping_add(32)).wrapping_add(8))), rt::fmul(F32(c.m.r32((l_sb_4).wrapping_add(16))), F32(c.m.r32(((p_b_2).wrapping_add(176)).wrapping_add(8))))); let a_ = ((p_b_2).wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = ((p_b_2).wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                break 'f 1i32;
            }
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftDet_SpheresVsSpheres", &PROBE_N, &[], true); }
    r_
}

pub fn EftDet_CapsuleVsCapsule(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDet_CapsuleVsCapsule", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_ca_3: u32 = 0;
    let mut l_cb_4: u32 = 0;
    let r_ = 'f: {
        l_ca_3 = c.m.r32((p_a_1).wrapping_add(12));
        l_cb_4 = c.m.r32((p_b_2).wrapping_add(12));
        { let a2_ = F32(c.m.r32((l_ca_3).wrapping_add(32))); F::ColSphere_Set(c, fp, l_ca_3, a2_) };
        { let a2_ = F32(c.m.r32((l_cb_4).wrapping_add(32))); F::ColSphere_Set(c, fp.wrapping_add(32), l_cb_4, a2_) };
        if ((((!((F::ColBox_Overlaps(c, (p_a_1).wrapping_add(132), (p_b_2).wrapping_add(132))) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((F::ColSphere_SweepSphere(c, fp, fp.wrapping_add(32), (p_a_1).wrapping_add(160), (p_b_2).wrapping_add(160), fp.wrapping_add(80))) != 0) {
            if (((((((rt::fle(F32(c.m.r32(fp.wrapping_add(80))), F32(0x358637bd)) as i32)) != 0) || (((rt::fle(F32(0x3f800000), F32(c.m.r32(fp.wrapping_add(80)))) as i32)) != 0)) as i32)) != 0) {
                if ((F::ColSphere_SeparateSphere(c, fp, fp.wrapping_add(32), (p_a_1).wrapping_add(32), (p_b_2).wrapping_add(32), fp.wrapping_add(64))) != 0) {
                    { let v_ = rt::fadd(F32(c.m.r32((p_a_1).wrapping_add(32))), rt::fmul(F32(c.m.r32(fp.wrapping_add(64))), F32(c.m.r32((fp).wrapping_add(16))))); let a_ = (p_a_1).wrapping_add(48); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32(((p_a_1).wrapping_add(32)).wrapping_add(4))), rt::fmul(F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(4))), F32(c.m.r32((fp).wrapping_add(16))))); let a_ = ((p_a_1).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32(((p_a_1).wrapping_add(32)).wrapping_add(8))), rt::fmul(F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(8))), F32(c.m.r32((fp).wrapping_add(16))))); let a_ = ((p_a_1).wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = ((p_a_1).wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    F::Vec4_Copy(c, (p_b_2).wrapping_add(48), (p_a_1).wrapping_add(48));
                    break 'f 1i32;
                }
            } else {
                { let v_ = rt::fadd(F32(c.m.r32(fp)), rt::fmul(F32(c.m.r32(fp.wrapping_add(80))), F32(c.m.r32((p_a_1).wrapping_add(160))))); let a_ = (p_a_1).wrapping_add(32); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((fp).wrapping_add(4))), rt::fmul(F32(c.m.r32(fp.wrapping_add(80))), F32(c.m.r32(((p_a_1).wrapping_add(160)).wrapping_add(4))))); let a_ = ((p_a_1).wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((fp).wrapping_add(8))), rt::fmul(F32(c.m.r32(fp.wrapping_add(80))), F32(c.m.r32(((p_a_1).wrapping_add(160)).wrapping_add(8))))); let a_ = ((p_a_1).wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = ((p_a_1).wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((p_a_1).wrapping_add(32))), rt::fmul(F32(c.m.r32((fp).wrapping_add(16))), F32(c.m.r32((p_a_1).wrapping_add(176))))); let a_ = (p_a_1).wrapping_add(48); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32(((p_a_1).wrapping_add(32)).wrapping_add(4))), rt::fmul(F32(c.m.r32((fp).wrapping_add(16))), F32(c.m.r32(((p_a_1).wrapping_add(176)).wrapping_add(4))))); let a_ = ((p_a_1).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32(((p_a_1).wrapping_add(32)).wrapping_add(8))), rt::fmul(F32(c.m.r32((fp).wrapping_add(16))), F32(c.m.r32(((p_a_1).wrapping_add(176)).wrapping_add(8))))); let a_ = ((p_a_1).wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = ((p_a_1).wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32(fp.wrapping_add(32))), rt::fmul(F32(c.m.r32(fp.wrapping_add(80))), F32(c.m.r32((p_b_2).wrapping_add(160))))); let a_ = (p_b_2).wrapping_add(32); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4))), rt::fmul(F32(c.m.r32(fp.wrapping_add(80))), F32(c.m.r32(((p_b_2).wrapping_add(160)).wrapping_add(4))))); let a_ = ((p_b_2).wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8))), rt::fmul(F32(c.m.r32(fp.wrapping_add(80))), F32(c.m.r32(((p_b_2).wrapping_add(160)).wrapping_add(8))))); let a_ = ((p_b_2).wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = ((p_b_2).wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((p_b_2).wrapping_add(32))), rt::fmul(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(16))), F32(c.m.r32((p_b_2).wrapping_add(176))))); let a_ = (p_b_2).wrapping_add(48); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32(((p_b_2).wrapping_add(32)).wrapping_add(4))), rt::fmul(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(16))), F32(c.m.r32(((p_b_2).wrapping_add(176)).wrapping_add(4))))); let a_ = ((p_b_2).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32(((p_b_2).wrapping_add(32)).wrapping_add(8))), rt::fmul(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(16))), F32(c.m.r32(((p_b_2).wrapping_add(176)).wrapping_add(8))))); let a_ = ((p_b_2).wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = ((p_b_2).wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                break 'f 1i32;
            }
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftDet_CapsuleVsCapsule", &PROBE_N, &[], true); }
    r_
}

pub fn EftDet_CapsuleVsSpheres(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDet_CapsuleVsSpheres", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_ca_3: u32 = 0;
    let mut l_sb_4: u32 = 0;
    let r_ = 'f: {
        l_ca_3 = c.m.r32((p_a_1).wrapping_add(12));
        l_sb_4 = c.m.r32((p_b_2).wrapping_add(16));
        { let a2_ = F32(c.m.r32((l_ca_3).wrapping_add(32))); F::ColSphere_Set(c, fp, l_ca_3, a2_) };
        if ((((!((F::ColBox_Overlaps(c, (p_a_1).wrapping_add(132), (p_b_2).wrapping_add(132))) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((F::ColSphere_SweepSphere(c, fp, l_sb_4, (p_a_1).wrapping_add(160), (p_b_2).wrapping_add(160), fp.wrapping_add(48))) != 0) {
            if (((((((rt::fle(F32(c.m.r32(fp.wrapping_add(48))), F32(0x358637bd)) as i32)) != 0) || (((rt::fle(F32(0x3f800000), F32(c.m.r32(fp.wrapping_add(48)))) as i32)) != 0)) as i32)) != 0) {
                if ((F::ColSphere_SeparateSphere(c, fp, l_sb_4, (p_a_1).wrapping_add(32), (p_b_2).wrapping_add(32), fp.wrapping_add(32))) != 0) {
                    { let v_ = rt::fadd(F32(c.m.r32((p_a_1).wrapping_add(32))), rt::fmul(F32(c.m.r32(fp.wrapping_add(32))), F32(c.m.r32((fp).wrapping_add(16))))); let a_ = (p_a_1).wrapping_add(48); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32(((p_a_1).wrapping_add(32)).wrapping_add(4))), rt::fmul(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4))), F32(c.m.r32((fp).wrapping_add(16))))); let a_ = ((p_a_1).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32(((p_a_1).wrapping_add(32)).wrapping_add(8))), rt::fmul(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8))), F32(c.m.r32((fp).wrapping_add(16))))); let a_ = ((p_a_1).wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = ((p_a_1).wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    F::Vec4_Copy(c, (p_b_2).wrapping_add(48), (p_a_1).wrapping_add(48));
                    break 'f 1i32;
                }
            } else {
                { let v_ = rt::fadd(F32(c.m.r32(fp)), rt::fmul(F32(c.m.r32(fp.wrapping_add(48))), F32(c.m.r32((p_a_1).wrapping_add(160))))); let a_ = (p_a_1).wrapping_add(32); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((fp).wrapping_add(4))), rt::fmul(F32(c.m.r32(fp.wrapping_add(48))), F32(c.m.r32(((p_a_1).wrapping_add(160)).wrapping_add(4))))); let a_ = ((p_a_1).wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((fp).wrapping_add(8))), rt::fmul(F32(c.m.r32(fp.wrapping_add(48))), F32(c.m.r32(((p_a_1).wrapping_add(160)).wrapping_add(8))))); let a_ = ((p_a_1).wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = ((p_a_1).wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((p_a_1).wrapping_add(32))), rt::fmul(F32(c.m.r32((fp).wrapping_add(16))), F32(c.m.r32((p_a_1).wrapping_add(176))))); let a_ = (p_a_1).wrapping_add(48); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32(((p_a_1).wrapping_add(32)).wrapping_add(4))), rt::fmul(F32(c.m.r32((fp).wrapping_add(16))), F32(c.m.r32(((p_a_1).wrapping_add(176)).wrapping_add(4))))); let a_ = ((p_a_1).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32(((p_a_1).wrapping_add(32)).wrapping_add(8))), rt::fmul(F32(c.m.r32((fp).wrapping_add(16))), F32(c.m.r32(((p_a_1).wrapping_add(176)).wrapping_add(8))))); let a_ = ((p_a_1).wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = ((p_a_1).wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32(l_sb_4)), rt::fmul(F32(c.m.r32(fp.wrapping_add(48))), F32(c.m.r32((p_b_2).wrapping_add(160))))); let a_ = (p_b_2).wrapping_add(32); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((l_sb_4).wrapping_add(4))), rt::fmul(F32(c.m.r32(fp.wrapping_add(48))), F32(c.m.r32(((p_b_2).wrapping_add(160)).wrapping_add(4))))); let a_ = ((p_b_2).wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((l_sb_4).wrapping_add(8))), rt::fmul(F32(c.m.r32(fp.wrapping_add(48))), F32(c.m.r32(((p_b_2).wrapping_add(160)).wrapping_add(8))))); let a_ = ((p_b_2).wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = ((p_b_2).wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((p_b_2).wrapping_add(32))), rt::fmul(F32(c.m.r32((l_sb_4).wrapping_add(16))), F32(c.m.r32((p_b_2).wrapping_add(176))))); let a_ = (p_b_2).wrapping_add(48); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32(((p_b_2).wrapping_add(32)).wrapping_add(4))), rt::fmul(F32(c.m.r32((l_sb_4).wrapping_add(16))), F32(c.m.r32(((p_b_2).wrapping_add(176)).wrapping_add(4))))); let a_ = ((p_b_2).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32(((p_b_2).wrapping_add(32)).wrapping_add(8))), rt::fmul(F32(c.m.r32((l_sb_4).wrapping_add(16))), F32(c.m.r32(((p_b_2).wrapping_add(176)).wrapping_add(8))))); let a_ = ((p_b_2).wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = ((p_b_2).wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                break 'f 1i32;
            }
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftDet_CapsuleVsSpheres", &PROBE_N, &[], true); }
    r_
}

pub fn EftDet_ClashPass(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDet_ClashPass", &PROBE_N, &[], false); }
    let mut l_list_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_j_3: i32 = 0;
    let mut l_hit_4: i32 = 0;
    let mut l_result_5: i32 = 0;
    let r_ = 'f: {
    l_list_1 = F::EftHit_GetList(c);
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (((c.m.r32((l_list_1).wrapping_add(25600)) as i32)).wrapping_sub(1i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((F::EftHit_CanHit(c, (l_list_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(400)), 2i32)) != 0) {
                    {
                        l_j_3 = l_i_2;
                        'l3: loop {
                            if !(((((l_j_3) < ((c.m.r32((l_list_1).wrapping_add(25600)) as i32))) as i32)) != 0) { break 'l3; }
                            'c4: {
                                l_hit_4 = 0i32;
                                if ((((((c.m.r32((l_list_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(400))) as i32)) != ((c.m.r32((l_list_1).wrapping_add((((l_j_3) as u32)).wrapping_mul(400))) as i32))) as i32)) != 0) {
                                    if ((F::EftHit_CanHit(c, (l_list_1).wrapping_add((((l_j_3) as u32)).wrapping_mul(400)), 2i32)) != 0) {
                                        if ((((((((((c.m.r32(((l_list_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(400))).wrapping_add(112)) as i32)) == (0i32)) as i32)) != 0) && ((((((c.m.r32(((l_list_1).wrapping_add((((l_j_3) as u32)).wrapping_mul(400))).wrapping_add(112)) as i32)) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                                            if ((F::EftDet_SpheresVsSpheres(c, ((l_list_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(400))).wrapping_add(112), ((l_list_1).wrapping_add((((l_j_3) as u32)).wrapping_mul(400))).wrapping_add(112))) != 0) {
                                                l_hit_4 = 1i32;
                                            }
                                        } else {
                                            if ((((((((((c.m.r32(((l_list_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(400))).wrapping_add(112)) as i32)) == (1i32)) as i32)) != 0) && ((((((c.m.r32(((l_list_1).wrapping_add((((l_j_3) as u32)).wrapping_mul(400))).wrapping_add(112)) as i32)) == (1i32)) as i32)) != 0)) as i32)) != 0) {
                                                if ((F::EftDet_CapsuleVsCapsule(c, ((l_list_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(400))).wrapping_add(112), ((l_list_1).wrapping_add((((l_j_3) as u32)).wrapping_mul(400))).wrapping_add(112))) != 0) {
                                                    l_hit_4 = 1i32;
                                                }
                                            } else {
                                                if ((((((((((((((c.m.r32(((l_list_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(400))).wrapping_add(112)) as i32)) == (1i32)) as i32)) != 0) && ((((((c.m.r32(((l_list_1).wrapping_add((((l_j_3) as u32)).wrapping_mul(400))).wrapping_add(112)) as i32)) == (0i32)) as i32)) != 0)) as i32)) != 0) || ((((((((((c.m.r32(((l_list_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(400))).wrapping_add(112)) as i32)) == (0i32)) as i32)) != 0) && ((((((c.m.r32(((l_list_1).wrapping_add((((l_j_3) as u32)).wrapping_mul(400))).wrapping_add(112)) as i32)) == (1i32)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
                                                    if ((((((c.m.r32(((l_list_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(400))).wrapping_add(112)) as i32)) == (1i32)) as i32)) != 0) {
                                                        if ((F::EftDet_CapsuleVsSpheres(c, ((l_list_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(400))).wrapping_add(112), ((l_list_1).wrapping_add((((l_j_3) as u32)).wrapping_mul(400))).wrapping_add(112))) != 0) {
                                                            l_hit_4 = 1i32;
                                                        }
                                                    } else {
                                                        if ((F::EftDet_CapsuleVsSpheres(c, ((l_list_1).wrapping_add((((l_j_3) as u32)).wrapping_mul(400))).wrapping_add(112), ((l_list_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(400))).wrapping_add(112))) != 0) {
                                                            l_hit_4 = 1i32;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        l_result_5 = F::EftHit_Clash(c, (l_list_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(400)), (l_list_1).wrapping_add((((l_j_3) as u32)).wrapping_mul(400)));
                                        if ((l_hit_4) != 0) {
                                            { let r_ = 4i32; let a_ = (((l_list_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(400))).wrapping_add(112)).wrapping_add(200); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                            if ((((l_result_5) & (1i32))) != 0) {
                                                { F::EftHit_SetTaskFlag(c, ((l_i_2) as u32), 8i32, (((l_list_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(400))).wrapping_add(112)).wrapping_add(32)); { let _ = 0i32; }; };
                                            }
                                            if ((((l_result_5) & (2i32))) != 0) {
                                                { F::EftHit_SetTaskFlag(c, ((l_j_3) as u32), 8i32, (((l_list_1).wrapping_add((((l_j_3) as u32)).wrapping_mul(400))).wrapping_add(112)).wrapping_add(32)); { let _ = 0i32; }; };
                                            }
                                            if ((((l_result_5) & (4i32))) != 0) {
                                                if (({ let a0_ = (c.m.r32((l_list_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(400))) as i32); F::BtlCharApi_IsSightBlocked(c, a0_) }) != 0) {
                                                    { F::EftHit_SetTaskFlag(c, ((l_i_2) as u32), 8i32, (((l_list_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(400))).wrapping_add(112)).wrapping_add(32)); { let _ = 0i32; }; };
                                                    { F::EftHit_SetTaskFlag(c, ((l_j_3) as u32), 8i32, (((l_list_1).wrapping_add((((l_j_3) as u32)).wrapping_mul(400))).wrapping_add(112)).wrapping_add(32)); { let _ = 0i32; }; };
                                                } else {
                                                    if (((((F::BtlSeq_GetState(c)) >= (4i32)) as i32)) != 0) {
                                                        { F::EftHit_SetTaskFlag(c, ((l_i_2) as u32), 8i32, (((l_list_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(400))).wrapping_add(112)).wrapping_add(32)); { let _ = 0i32; }; };
                                                        { F::EftHit_SetTaskFlag(c, ((l_j_3) as u32), 8i32, (((l_list_1).wrapping_add((((l_j_3) as u32)).wrapping_mul(400))).wrapping_add(112)).wrapping_add(32)); { let _ = 0i32; }; };
                                                    } else {
                                                        F::BtlCharApi_SetHeldFlagAA(c, 0i32);
                                                        F::BtlCharApi_SetHeldFlagAA(c, 1i32);
                                                        if ((((!((F::EftStruggle_IsActive(c)) != 0)) as i32)) != 0) {
                                                            { F::EftHit_SetTaskFlag(c, ((l_i_2) as u32), 256i32, (((l_list_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(400))).wrapping_add(112)).wrapping_add(32)); { let _ = 0i32; }; };
                                                            { F::EftHit_SetTaskFlag(c, ((l_j_3) as u32), 256i32, (((l_list_1).wrapping_add((((l_j_3) as u32)).wrapping_mul(400))).wrapping_add(112)).wrapping_add(32)); { let _ = 0i32; }; };
                                                            { let _ = F::EftStruggle_Start(c, (((l_list_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(400))).wrapping_add(112)).wrapping_add(32), (l_list_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(400)), (l_list_1).wrapping_add((((l_j_3) as u32)).wrapping_mul(400))); };
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            { let o_ = l_j_3; let n_ = (o_).wrapping_add(1); l_j_3 = n_; };
                        }
                    }
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDet_ClashPass", &PROBE_N, &[], true); }
    r_
}

pub fn EftDet_StageCb(c: &mut Ctx, mut p_node_1: u32, mut p_ctx_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDet_StageCb", &PROBE_N, &[(p_node_1) as u32, (p_ctx_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_poly_3: u32 = 0;
    let r_ = 'f: {
        { let _ = F::memset(c, fp.wrapping_add(64), 0i32, 16u32); };
        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(64)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        l_poly_3 = { let a0_ = c.m.r32(sym::gStgColMesh); let a1_ = (c.m.r32((p_node_1).wrapping_add(24)) as i32); F::ColMesh_GetPoly(c, a0_, a1_) };
        if (((((c.m.r32(l_poly_3) as i32)) & (134217728i32))) != 0) {
            break 'f 0i32;
        }
        { let a0_ = c.m.r32(sym::gStgColMesh); F::ColMesh_GetPolyVerts(c, a0_, l_poly_3, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))) };
        F::Vec4_Copy(c, (fp).wrapping_add(48), (l_poly_3).wrapping_add(16));
        if ((F::ColSweep_TestTri(c, p_ctx_2, fp, fp.wrapping_add(64), fp.wrapping_add(80))) != 0) {
            if (((rt::flt(F32(c.m.r32(fp.wrapping_add(80))), F32(c.m.r32((p_ctx_2).wrapping_add(132)))) as i32)) != 0) {
                { let v_ = F32(c.m.r32(fp.wrapping_add(80))); let a_ = (p_ctx_2).wrapping_add(132); c.m.w32(a_, (v_).0); };
                { let v_ = 1i32; let a_ = (p_ctx_2).wrapping_add(136); c.m.w32(a_, (v_) as u32); };
                { let a0_ = (c.m.r32((p_ctx_2).wrapping_add(128))).wrapping_add(32); F::Vec4_Copy(c, a0_, fp.wrapping_add(64)) };
                { let s_ = fp; let d_ = (c.m.r32((p_ctx_2).wrapping_add(128))).wrapping_add(64); rt::memcpy(c, d_, s_, 64); };
                if (((rt::flt(F32(c.m.r32((p_ctx_2).wrapping_add(132))), F32(0x3c23d70a)) as i32)) != 0) {
                    { let a0_ = (c.m.r32((p_ctx_2).wrapping_add(128))).wrapping_add(48); F::Vec4_Copy(c, a0_, fp.wrapping_add(64)) };
                } else {
                    { let v_ = rt::fsub(F32(c.m.r32(fp.wrapping_add(64))), rt::fmul(F32(c.m.r32((fp).wrapping_add(48))), F32(c.m.r32((p_ctx_2).wrapping_add(32))))); let a_ = (c.m.r32((p_ctx_2).wrapping_add(128))).wrapping_add(48); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(4))), rt::fmul(F32(c.m.r32(((fp).wrapping_add(48)).wrapping_add(4))), F32(c.m.r32((p_ctx_2).wrapping_add(32))))); let a_ = ((c.m.r32((p_ctx_2).wrapping_add(128))).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(8))), rt::fmul(F32(c.m.r32(((fp).wrapping_add(48)).wrapping_add(8))), F32(c.m.r32((p_ctx_2).wrapping_add(32))))); let a_ = ((c.m.r32((p_ctx_2).wrapping_add(128))).wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = ((c.m.r32((p_ctx_2).wrapping_add(128))).wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                }
                break 'f 1i32;
            }
            break 'f 0i32;
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftDet_StageCb", &PROBE_N, &[], true); }
    r_
}

pub fn EftDet_TestStage(c: &mut Ctx, mut p_shape_1: u32, mut p_isBlast_2: i32, mut p_objectsOnly_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDet_TestStage", &PROBE_N, &[(p_shape_1) as u32, (p_isBlast_2) as u32, (p_objectsOnly_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(256) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 256);
    let mut l_zone_4: u32 = 0;
    let mut l_idx_5: i32 = 0;
    let r_ = 'f: {
        if ((((((c.m.r32((p_shape_1).wrapping_add(128)) as i32)) < (0i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        { let _ = F::memset(c, fp, 0i32, 144u32); };
        { let v_ = p_shape_1; let a_ = (fp).wrapping_add(128); c.m.w32(a_, v_); };
        's1: {
            let sv6: i32 = (c.m.r32(p_shape_1) as i32);
            match sv6 {
                0i32 => {
                    { let a1_ = c.m.r32((p_shape_1).wrapping_add(16)); let a2_ = c.m.r32((p_shape_1).wrapping_add(12)); let a3_ = { let a_ = (c.m.r32((p_shape_1).wrapping_add(12))).wrapping_add(16); F32(c.m.r32(a_)) }; F::ColCapsule_Set(c, fp.wrapping_add(208), a1_, a2_, a3_) };
                    if ((((!((p_objectsOnly_3) != 0)) as i32)) != 0) {
                        { let r_ = F32(c.m.r32((p_shape_1).wrapping_add(4))); let a_ = (fp.wrapping_add(208)).wrapping_add(32); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    }
                    if (((rt::flt(F32(c.m.r32((fp.wrapping_add(208)).wrapping_add(32))), F32(0x3da3d70a)) as i32)) != 0) {
                        { let v_ = F32(0x3da3d70a); let a_ = (fp.wrapping_add(208)).wrapping_add(32); c.m.w32(a_, (v_).0); };
                    }
                    F::ColSweep_FromCapsule(c, fp, fp.wrapping_add(208));
                    F::ColBounds_OfCapsule(c, fp.wrapping_add(176), fp.wrapping_add(208));
                    if (((rt::fgt(F32(c.m.r32((fp).wrapping_add(112))), F32(c.m.r32((fp.wrapping_add(208)).wrapping_add(32)))) as i32)) != 0) {
                        { let v_ = rt::fadd(F32(c.m.r32((fp).wrapping_add(112))), F32(0x3a83126e)); let a_ = (fp).wrapping_add(132); c.m.w32(a_, (v_).0); };
                    } else {
                        { let v_ = rt::fadd(F32(c.m.r32((fp.wrapping_add(208)).wrapping_add(32))), F32(0x3a83126e)); let a_ = (fp).wrapping_add(132); c.m.w32(a_, (v_).0); };
                    }
                    break 's1;
                }
                1i32 => {
                    { let a1_ = (c.m.r32((p_shape_1).wrapping_add(16))).wrapping_add(16); let a2_ = (c.m.r32((p_shape_1).wrapping_add(12))).wrapping_add(16); let a3_ = { let a_ = (c.m.r32((p_shape_1).wrapping_add(12))).wrapping_add(32); F32(c.m.r32(a_)) }; F::ColCapsule_Set(c, fp.wrapping_add(208), a1_, a2_, a3_) };
                    { let r_ = F32(c.m.r32((p_shape_1).wrapping_add(4))); let a_ = (fp.wrapping_add(208)).wrapping_add(32); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if (((rt::flt(F32(c.m.r32((fp.wrapping_add(208)).wrapping_add(32))), F32(0x3da3d70a)) as i32)) != 0) {
                        { let v_ = F32(0x3da3d70a); let a_ = (fp.wrapping_add(208)).wrapping_add(32); c.m.w32(a_, (v_).0); };
                    }
                    F::ColSweep_FromCapsule(c, fp, fp.wrapping_add(208));
                    F::ColBounds_OfCapsule(c, fp.wrapping_add(176), fp.wrapping_add(208));
                    { let v_ = rt::fadd(F32(c.m.r32((fp).wrapping_add(112))), F32(0x3a83126e)); let a_ = (fp).wrapping_add(132); c.m.w32(a_, (v_).0); };
                    break 's1;
                }
                2i32 => {
                    break 's1;
                }
                _ => {
                }
            }
        }
        l_zone_4 = { let a0_ = (c.m.r32((p_shape_1).wrapping_add(128)) as i32); F::BtlStage_GetZone(c, a0_) };
        F::StgCol_SetModeAll(c);
        if ((p_objectsOnly_3) != 0) {
            F::StgCol_SetModeObjects(c);
        }
        if ((((!((F::StgCol_QueryZone(c, fp.wrapping_add(144), l_zone_4, fp.wrapping_add(176), fp, fid::EftDet_StageCb)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((((((c.m.r32((fp.wrapping_add(144)).wrapping_add(12)) as i32)) != (0i32)) as i32)) != 0) {
            l_idx_5 = { let a0_ = c.m.r64((fp.wrapping_add(144)).wrapping_add(24)); F::StgCol_FirstBit(c, a0_) };
            if (((((l_idx_5) >= (0i32)) as i32)) != 0) {
                if ((F::BtlStage_DamageObj(c, l_idx_5, (if (((((p_isBlast_2) == (1i32)) as i32)) != 0) { 1i32 } else { 999999i32 }))) != 0) {
                    { let _ = F::BtlStage_DestroyObj(c, (1i32).wrapping_neg(), l_idx_5, (fp).wrapping_add(80)); };
                    break 'f 0i32;
                }
            }
        }
        { let r_ = 1i32; let a_ = (p_shape_1).wrapping_add(200); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        's2: {
            let sv7: i32 = (c.m.r32(p_shape_1) as i32);
            match sv7 {
                0i32 => {
                    { let v_ = { let a0_ = c.m.r32((p_shape_1).wrapping_add(16)); F::Vec3_Dist(c, a0_, (p_shape_1).wrapping_add(32)) }; let a_ = (p_shape_1).wrapping_add(196); c.m.w32(a_, (v_).0); };
                    break 's2;
                }
                1i32 => {
                    { let v_ = { let a0_ = c.m.r32((p_shape_1).wrapping_add(12)); F::Vec3_Dist(c, a0_, (p_shape_1).wrapping_add(32)) }; let a_ = (p_shape_1).wrapping_add(196); c.m.w32(a_, (v_).0); };
                    break 's2;
                }
                2i32 => {
                    break 's2;
                }
                _ => {
                }
            }
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftDet_TestStage", &PROBE_N, &[], true); }
    r_
}

pub fn EftDet_StagePass(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftDet_StagePass", &PROBE_N, &[], false); }
    let mut l_list_1: u32 = 0;
    let mut l_rec_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_objectsOnly_4: i32 = 0;
    let mut l_def_5: u32 = 0;
    let r_ = 'f: {
    l_list_1 = F::EftHit_GetList(c);
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < ((c.m.r32((l_list_1).wrapping_add(25600)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                l_rec_2 = (l_list_1).wrapping_add((((l_i_3) as u32)).wrapping_mul(400));
                if ((F::EftHit_CanHit(c, l_rec_2, 0i32)) != 0) {
                    l_objectsOnly_4 = 0i32;
                    if (((((c.m.r32((l_rec_2).wrapping_add(100))) != (((0i32) as u32))) as i32)) != 0) {
                        l_def_5 = { let a_ = (c.m.r32((l_rec_2).wrapping_add(100))).wrapping_add(36); c.m.r32(a_) };
                        if ((((((((c.m.r8((l_def_5).wrapping_add(4)) as i8)) as i32)) == (0i32)) as i32)) != 0) {
                            if ((((((((c.m.r8((l_def_5).wrapping_add(5)) as i8)) as i32)) == (4i32)) as i32)) != 0) {
                                l_objectsOnly_4 = 1i32;
                            }
                        } else {
                            l_objectsOnly_4 = ((((((c.m.r8((l_def_5).wrapping_add(5)) as i8)) as i32)) == (9i32)) as i32);
                        }
                    }
                    if (({ let a1_ = ((((c.m.r32((l_rec_2).wrapping_add(12)) as i32)) == (0i32)) as i32); F::EftDet_TestStage(c, (l_rec_2).wrapping_add(112), a1_, l_objectsOnly_4) }) != 0) {
                        if ((F::EftHit_IsStoppedByHit(c, l_rec_2)) != 0) {
                            { F::EftHit_SetTaskFlag(c, ((l_i_3) as u32), 4i32, (((l_list_1).wrapping_add((((l_i_3) as u32)).wrapping_mul(400))).wrapping_add(112)).wrapping_add(48)); { let _ = 0i32; }; };
                            { let _ = F::EftHit_NotifyBlastTask(c, ((l_i_3) as u32), ((l_rec_2).wrapping_add(112)).wrapping_add(64)); };
                            F::BtlColl_PlayContactSound(c, l_rec_2);
                            if ((((((c.m.r32((l_rec_2).wrapping_add(12)) as i32)) == (1i32)) as i32)) != 0) {
                                F::BtlCharApi_ShakeCamsNear(c, (l_rec_2).wrapping_add(32), F32(0x42c80000), F32(0x44bb8000), F32(0x40400000), F32(0x3f000000));
                                F::BtlCharApi_RumbleNear(c, (l_rec_2).wrapping_add(32), F32(0x42c80000), F32(0x447a0000), F32(0x3f800000), F32(0x3e999999));
                            }
                        }
                    }
                }
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftDet_StagePass", &PROBE_N, &[], true); }
    r_
}

pub fn StgGround_ExtendBoxDown(c: &mut Ctx, mut p_box_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgGround_ExtendBoxDown", &PROBE_N, &[(p_box_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let r_ = 'f: {
        F::ColBox_GetCenter(c, p_box_1, fp);
        F::ColBox_GetHalf(c, p_box_1, fp.wrapping_add(16));
        { let v_ = rt::fmul(rt::fadd(rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(0x40000000)), F32(0x45fa0000)), F32(0x3f000000)); let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((p_box_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4)))); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        F::ColBox_SetCenterHalf(c, p_box_1, fp, fp.wrapping_add(16));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgGround_ExtendBoxDown", &PROBE_N, &[], true); }
    r_
}

pub fn StgGround_TestTri(c: &mut Ctx, mut p_out_1: u32, mut p_tri_2: u32, mut p_flags_3: i32, mut p_minY_4: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgGround_TestTri", &PROBE_N, &[(p_out_1) as u32, (p_tri_2) as u32, (p_flags_3) as u32, (p_minY_4).0], false); }
    let mut l_result_5: i32 = 0;
    let mut l_inside_6: i32 = 0;
    let mut l_x0_7: F32 = F32(0);
    let mut l_z0_8: F32 = F32(0);
    let mut l_px_9: F32 = F32(0);
    let mut l_pz_10: F32 = F32(0);
    let mut l_x2_11: F32 = F32(0);
    let mut l_z2_12: F32 = F32(0);
    let mut l_y_13: F32 = F32(0);
    let mut l_x1_14: F32 = F32(0);
    let mut l_z1_15: F32 = F32(0);
    let mut l_c_16: F32 = F32(0);
    let r_ = 'f: {
    l_result_5 = 0i32;
    l_inside_6 = 0i32;
    l_x0_7 = F32(c.m.r32((p_tri_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))));
    l_z0_8 = F32(c.m.r32(((p_tri_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)));
    l_px_9 = F32(c.m.r32(p_out_1));
    l_pz_10 = F32(c.m.r32((p_out_1).wrapping_add(8)));
    l_x2_11 = F32(c.m.r32((p_tri_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))));
    l_z2_12 = F32(c.m.r32(((p_tri_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)));
    l_y_13 = rt::fdiv(rt::fsub(rt::fsub(F32(c.m.r32(((p_tri_2).wrapping_add(48)).wrapping_add(12))), rt::fmul(F32(c.m.r32((p_tri_2).wrapping_add(48))), l_px_9)), rt::fmul(F32(c.m.r32(((p_tri_2).wrapping_add(48)).wrapping_add(8))), l_pz_10)), F32(c.m.r32(((p_tri_2).wrapping_add(48)).wrapping_add(4))));
    l_x1_14 = F32(c.m.r32((p_tri_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))));
    l_z1_15 = F32(c.m.r32(((p_tri_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)));
    l_c_16 = rt::fsub(rt::fmul(rt::fsub(l_x2_11, l_x0_7), rt::fsub(l_pz_10, l_z0_8)), rt::fmul(rt::fsub(l_z2_12, l_z0_8), rt::fsub(l_px_9, l_x0_7)));
    if (((rt::fle(l_c_16, F32(0x3a83126e)) as i32)) != 0) {
        l_c_16 = rt::fsub(rt::fmul(rt::fsub(l_x1_14, l_x2_11), rt::fsub(l_pz_10, l_z2_12)), rt::fmul(rt::fsub(l_z1_15, l_z2_12), rt::fsub(l_px_9, l_x2_11)));
        if (((rt::fle(l_c_16, F32(0x3a83126e)) as i32)) != 0) {
            l_c_16 = rt::fsub(rt::fmul(rt::fsub(l_x0_7, l_x1_14), rt::fsub(l_pz_10, l_z1_15)), rt::fmul(rt::fsub(l_z0_8, l_z1_15), rt::fsub(l_px_9, l_x1_14)));
            if (((rt::fle(l_c_16, F32(0x3a83126e)) as i32)) != 0) {
                l_inside_6 = 1i32;
            }
        }
    }
    if (((((((((((rt::flt(p_minY_4, l_y_13) as i32)) != 0) && (((rt::flt(l_y_13, F32(c.m.r32((p_out_1).wrapping_add(4)))) as i32)) != 0)) as i32)) != 0) && ((l_inside_6) != 0)) as i32)) != 0) {
        { let v_ = l_y_13; let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_flags_3; let a_ = (p_out_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
        F::Vec4_Copy(c, (p_out_1).wrapping_add(16), (p_tri_2).wrapping_add(48));
        l_result_5 = 1i32;
    }
    break 'f l_result_5;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "StgGround_TestTri", &PROBE_N, &[], true); }
    r_
}

pub fn StgGround_Cb(c: &mut Ctx, mut p_node_1: u32, mut p_ctx_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgGround_Cb", &PROBE_N, &[(p_node_1) as u32, (p_ctx_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_poly_3: u32 = 0;
    let r_ = 'f: {
        l_poly_3 = { let a0_ = c.m.r32(sym::gStgColMesh); let a1_ = (c.m.r32((p_node_1).wrapping_add(24)) as i32); F::ColMesh_GetPoly(c, a0_, a1_) };
        if (((((c.m.r32(l_poly_3) as i32)) & (134217728i32))) != 0) {
            break 'f 0i32;
        }
        if ((((!(((rt::flt(F::Vec3_Dot(c, sym::D_002EC2E0, (l_poly_3).wrapping_add(16)), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        { let a0_ = c.m.r32(sym::gStgColMesh); F::ColMesh_GetPolyVerts(c, a0_, l_poly_3, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))) };
        F::Vec4_Copy(c, (fp).wrapping_add(48), (l_poly_3).wrapping_add(16));
        break 'f ((({ let a0_ = c.m.r32(p_ctx_2); let a2_ = (c.m.r32(l_poly_3) as i32); let a3_ = F32(c.m.r32((p_ctx_2).wrapping_add(4))); F::StgGround_TestTri(c, a0_, fp, a2_, a3_) }) != (0i32)) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgGround_Cb", &PROBE_N, &[], true); }
    r_
}

pub fn StgGround_Probe(c: &mut Ctx, mut p_zone_1: i32, mut p_box_2: u32, mut p_out_3: u32, mut p_minY_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgGround_Probe", &PROBE_N, &[(p_zone_1) as u32, (p_box_2) as u32, (p_out_3) as u32, (p_minY_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_z_5: u32 = 0;
    let r_ = 'f: {
        if (((((p_out_3) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = F32(c.m.r32((sym::D_002FEAFC).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_out_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
        }
        F::ColBox_GetCenter(c, p_box_2, fp.wrapping_add(32));
        { let s_ = p_box_2; let d_ = fp.wrapping_add(8); rt::memcpy(c, d_, s_, 24); };
        F::StgGround_ExtendBoxDown(c, fp.wrapping_add(8));
        { let v_ = ((0i32) as u64); let a_ = fp; c.m.w64(a_, v_); };
        { let v_ = p_out_3; let a_ = fp; c.m.w32(a_, v_); };
        { let v_ = p_minY_4; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        l_z_5 = { let a0_ = F::BtlStage_FindZoneNear(c, p_zone_1, fp.wrapping_add(32)); F::BtlStage_GetZone(c, a0_) };
        F::StgCol_SetModeAll(c);
        { let _ = F::StgCol_QueryZone(c, sym::gStgGroundResult, l_z_5, fp.wrapping_add(8), fp, fid::StgGround_Cb); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgGround_Probe", &PROBE_N, &[], true); }
    r_
}

pub fn StgGround_UpdateFighter(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgGround_UpdateFighter", &PROBE_N, &[(p_obj_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_blk_2: u32 = 0;
    let mut l_body_3: u32 = 0;
    let mut l_work_4: u32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp.wrapping_add(48), 0, 16);
        rt::memset(c, fp.wrapping_add(48).wrapping_add(0), 0, 16);
        { let v_ = F32(0x3dcccccc); c.m.w32(fp.wrapping_add(48).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(48).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3dcccccc); c.m.w32(fp.wrapping_add(48).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(48).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(64), 0, 16);
        rt::memset(c, fp.wrapping_add(64).wrapping_add(0), 0, 16);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(64).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(64).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(64).wrapping_add(0).wrapping_add(12), (v_).0); }
        l_blk_2 = (p_obj_1).wrapping_add(2384);
        l_body_3 = c.m.r32((p_obj_1).wrapping_add(4000));
        l_work_4 = c.m.r32((p_obj_1).wrapping_add(5728));
        if (((((((((l_body_3) == (((0i32) as u32))) as i32)) != 0) || (((((l_work_4) == (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
            break 'f;
        }
        F::Vec4_Copy(c, fp.wrapping_add(32), (p_obj_1).wrapping_add(2416));
        { let v_ = F32(c.m.r32((l_body_3).wrapping_add(4))); let a_ = (fp.wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        F::ColBox_SetCenterHalf(c, fp, fp.wrapping_add(32), fp.wrapping_add(48));
        F::Vec3_Copy(c, (l_work_4).wrapping_add(98352), (p_obj_1).wrapping_add(2416));
        { let a0_ = (c.m.r32((l_blk_2).wrapping_add(((212i32) as u32))) as i32); let a3_ = F32(c.m.r32((l_body_3).wrapping_add(4))); F::StgGround_Probe(c, a0_, fp, (l_work_4).wrapping_add(98352), a3_) };
        if ((((((c.m.r32((sym::gStgGroundResult).wrapping_add(12)) as i32)) != (0i32)) as i32)) != 0) {
            if (((rt::flt(F32(c.m.r32(((l_work_4).wrapping_add(98352)).wrapping_add(4))), { let a_ = (((F::BtlObj_GetNode(c, p_obj_1, 0i32)).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }) as i32)) != 0) {
                { let a1_ = { let a0_ = c.m.r64((sym::gStgGroundResult).wrapping_add(24)); F::StgCol_FirstBit(c, a0_) }; F::StgCol_FighterBreakObj(c, p_obj_1, a1_, fp.wrapping_add(64)) };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgGround_UpdateFighter", &PROBE_N, &[], true); }
    r_
}
