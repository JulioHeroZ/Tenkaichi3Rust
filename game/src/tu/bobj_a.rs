#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlObjAnim_Retarget(c: &mut Ctx, mut p_anim_1: u32, mut p_obj_2: u32, mut p_other_3: u32, mut p_useTable_4: i32, mut p_useOwn_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_Retarget", &PROBE_N, &[(p_anim_1) as u32, (p_obj_2) as u32, (p_other_3) as u32, (p_useTable_4) as u32, (p_useOwn_5) as u32], false); }
    let mut l_bone_6: u32 = 0;
    let mut l_scale_7: F32 = F32(0);
    let mut l_id_8: u32 = 0;
    let mut l_trk_9: u32 = 0;
    let mut l_keys_10: u32 = 0;
    let mut l_n_11: i32 = 0;
    let mut l_i_12: i32 = 0;
    let mut l_src_13: u32 = 0;
    let r_ = 'f: {
    l_bone_6 = c.m.r32(((p_obj_2).wrapping_add(24)).wrapping_add(44));
    if (((((((((p_useOwn_5) != (0i32)) as i32)) != 0) && (((((c.m.r32(((p_obj_2).wrapping_add(24)).wrapping_add(2304))) == (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        p_useOwn_5 = 0i32;
    }
    if (((((p_useTable_4) != (0i32)) as i32)) != 0) {
        l_scale_7 = { let a_ = (c.m.r32(((p_obj_2).wrapping_add(24)).wrapping_add(2308))).wrapping_add(4); F32(c.m.r32(a_)) };
    } else {
        if (((((p_useOwn_5) != (0i32)) as i32)) != 0) {
            l_scale_7 = rt::fdiv({ let a_ = (c.m.r32(((p_obj_2).wrapping_add(24)).wrapping_add(2308))).wrapping_add(4); F32(c.m.r32(a_)) }, F32(0x3f8ccccc));
        } else {
            l_scale_7 = rt::fdiv({ let a_ = (c.m.r32(((p_obj_2).wrapping_add(24)).wrapping_add(2308))).wrapping_add(4); F32(c.m.r32(a_)) }, { let a_ = (c.m.r32(((p_other_3).wrapping_add(24)).wrapping_add(2308))).wrapping_add(4); F32(c.m.r32(a_)) });
        }
    }
    'l1: while ((1i32) != 0) {
        l_id_8 = ((c.m.r16((l_bone_6).wrapping_add(10))) as u32);
        if (((((((((l_id_8) < (((71i32) as u32))) as i32)) != 0) && (((((((c.m.r16(((p_anim_1).wrapping_add(6)).wrapping_add((l_id_8).wrapping_mul(2)))) as i32)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
            if (((((p_useTable_4) != (0i32)) as i32)) != 0) {
                l_trk_9 = (p_anim_1).wrapping_add(((((((c.m.r16(((p_anim_1).wrapping_add(6)).wrapping_add((l_id_8).wrapping_mul(2)))) as i32)).wrapping_mul(2i32)) as u32)).wrapping_mul(2));
                if ((((!((((((((c.m.r16(l_trk_9)) as i32)) & (1i32))) as u16)) != 0)) as i32)) != 0) {
                    l_n_11 = ((c.m.r16((l_trk_9).wrapping_add(2))) as i32);
                    l_keys_10 = (l_trk_9).wrapping_add(4);
                    {
                        l_i_12 = 0i32;
                        'l2: loop {
                            if !(((((l_i_12) < (l_n_11)) as i32)) != 0) { break 'l2; }
                            'c3: {
                                { let r_ = F32(c.m.r32((sym::D_002F2420).wrapping_add(((l_id_8).wrapping_mul(((3i32) as u32))).wrapping_mul(4)))); let a_ = (l_keys_10).wrapping_add((((l_i_12) as u32)).wrapping_mul(24)); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                { let r_ = F32(c.m.r32((sym::D_002F2420).wrapping_add((((l_id_8).wrapping_mul(((3i32) as u32))).wrapping_add(((1i32) as u32))).wrapping_mul(4)))); let a_ = ((l_keys_10).wrapping_add((((l_i_12) as u32)).wrapping_mul(24))).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                { let r_ = F32(c.m.r32((sym::D_002F2420).wrapping_add((((l_id_8).wrapping_mul(((3i32) as u32))).wrapping_add(((2i32) as u32))).wrapping_mul(4)))); let a_ = ((l_keys_10).wrapping_add((((l_i_12) as u32)).wrapping_mul(24))).wrapping_add(8); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                { let r_ = l_scale_7; let a_ = ((l_keys_10).wrapping_add((((l_i_12) as u32)).wrapping_mul(24))).wrapping_add(4); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                { let r_ = F32(c.m.r32((l_bone_6).wrapping_add(48))); let a_ = (l_keys_10).wrapping_add((((l_i_12) as u32)).wrapping_mul(24)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                { let r_ = F32(c.m.r32(((l_bone_6).wrapping_add(48)).wrapping_add(4))); let a_ = ((l_keys_10).wrapping_add((((l_i_12) as u32)).wrapping_mul(24))).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                { let r_ = F32(c.m.r32(((l_bone_6).wrapping_add(48)).wrapping_add(8))); let a_ = ((l_keys_10).wrapping_add((((l_i_12) as u32)).wrapping_mul(24))).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            }
                            { let o_ = l_i_12; let n_ = (o_).wrapping_add(1); l_i_12 = n_; };
                        }
                    }
                }
            } else {
                if (((((p_useOwn_5) != (0i32)) as i32)) != 0) {
                    l_trk_9 = (p_anim_1).wrapping_add(((((((c.m.r16(((p_anim_1).wrapping_add(6)).wrapping_add((l_id_8).wrapping_mul(2)))) as i32)).wrapping_mul(2i32)) as u32)).wrapping_mul(2));
                    if ((((!((((((((c.m.r16(l_trk_9)) as i32)) & (1i32))) as u16)) != 0)) as i32)) != 0) {
                        l_n_11 = ((c.m.r16((l_trk_9).wrapping_add(2))) as i32);
                        l_keys_10 = (l_trk_9).wrapping_add(4);
                        {
                            l_i_12 = 0i32;
                            'l4: loop {
                                if !(((((l_i_12) < (l_n_11)) as i32)) != 0) { break 'l4; }
                                'c5: {
                                    { let r_ = { let a_ = (c.m.r32(((p_obj_2).wrapping_add(24)).wrapping_add(2304))).wrapping_add((l_id_8).wrapping_mul(12)); F32(c.m.r32(a_)) }; let a_ = (l_keys_10).wrapping_add((((l_i_12) as u32)).wrapping_mul(24)); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                    { let r_ = { let a_ = ((c.m.r32(((p_obj_2).wrapping_add(24)).wrapping_add(2304))).wrapping_add((l_id_8).wrapping_mul(12))).wrapping_add(4); F32(c.m.r32(a_)) }; let a_ = ((l_keys_10).wrapping_add((((l_i_12) as u32)).wrapping_mul(24))).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                    { let r_ = { let a_ = ((c.m.r32(((p_obj_2).wrapping_add(24)).wrapping_add(2304))).wrapping_add((l_id_8).wrapping_mul(12))).wrapping_add(8); F32(c.m.r32(a_)) }; let a_ = ((l_keys_10).wrapping_add((((l_i_12) as u32)).wrapping_mul(24))).wrapping_add(8); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                    { let r_ = l_scale_7; let a_ = ((l_keys_10).wrapping_add((((l_i_12) as u32)).wrapping_mul(24))).wrapping_add(4); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                    { let r_ = F32(c.m.r32((l_bone_6).wrapping_add(48))); let a_ = (l_keys_10).wrapping_add((((l_i_12) as u32)).wrapping_mul(24)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                    { let r_ = F32(c.m.r32(((l_bone_6).wrapping_add(48)).wrapping_add(4))); let a_ = ((l_keys_10).wrapping_add((((l_i_12) as u32)).wrapping_mul(24))).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                    { let r_ = F32(c.m.r32(((l_bone_6).wrapping_add(48)).wrapping_add(8))); let a_ = ((l_keys_10).wrapping_add((((l_i_12) as u32)).wrapping_mul(24))).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                }
                                { let o_ = l_i_12; let n_ = (o_).wrapping_add(1); l_i_12 = n_; };
                            }
                        }
                    }
                } else {
                    l_src_13 = F::BtlObj_FindBound(c, p_other_3, ((l_id_8) as i32));
                    if (((((l_src_13) != (((0i32) as u32))) as i32)) != 0) {
                        l_trk_9 = (p_anim_1).wrapping_add(((((((c.m.r16(((p_anim_1).wrapping_add(6)).wrapping_add((l_id_8).wrapping_mul(2)))) as i32)).wrapping_mul(2i32)) as u32)).wrapping_mul(2));
                        if ((((!((((((((c.m.r16(l_trk_9)) as i32)) & (1i32))) as u16)) != 0)) as i32)) != 0) {
                            l_n_11 = ((c.m.r16((l_trk_9).wrapping_add(2))) as i32);
                            l_keys_10 = (l_trk_9).wrapping_add(4);
                            {
                                l_i_12 = 0i32;
                                'l6: loop {
                                    if !(((((l_i_12) < (l_n_11)) as i32)) != 0) { break 'l6; }
                                    'c7: {
                                        { let r_ = F32(c.m.r32((l_src_13).wrapping_add(48))); let a_ = (l_keys_10).wrapping_add((((l_i_12) as u32)).wrapping_mul(24)); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                        { let r_ = F32(c.m.r32(((l_src_13).wrapping_add(48)).wrapping_add(4))); let a_ = ((l_keys_10).wrapping_add((((l_i_12) as u32)).wrapping_mul(24))).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                        { let r_ = F32(c.m.r32(((l_src_13).wrapping_add(48)).wrapping_add(8))); let a_ = ((l_keys_10).wrapping_add((((l_i_12) as u32)).wrapping_mul(24))).wrapping_add(8); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                        { let r_ = l_scale_7; let a_ = ((l_keys_10).wrapping_add((((l_i_12) as u32)).wrapping_mul(24))).wrapping_add(4); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                        { let r_ = F32(c.m.r32((l_bone_6).wrapping_add(48))); let a_ = (l_keys_10).wrapping_add((((l_i_12) as u32)).wrapping_mul(24)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                        { let r_ = F32(c.m.r32(((l_bone_6).wrapping_add(48)).wrapping_add(4))); let a_ = ((l_keys_10).wrapping_add((((l_i_12) as u32)).wrapping_mul(24))).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                        { let r_ = F32(c.m.r32(((l_bone_6).wrapping_add(48)).wrapping_add(8))); let a_ = ((l_keys_10).wrapping_add((((l_i_12) as u32)).wrapping_mul(24))).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                    }
                                    { let o_ = l_i_12; let n_ = (o_).wrapping_add(1); l_i_12 = n_; };
                                }
                            }
                        }
                    }
                }
            }
        }
        if (((((((c.m.r16((l_bone_6).wrapping_add(6))) as i32)) != (0i32)) as i32)) != 0) {
            break 'l1;
        } else {
            l_bone_6 = (l_bone_6).wrapping_add((((c.m.r32(l_bone_6) as i32)) as u32));
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_Retarget", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjAnim_SampleRot(c: &mut Ctx, mut p_trk_1: u32, mut p_out_2: u32, mut p_cursor_3: u32, mut p_frame_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_SampleRot", &PROBE_N, &[(p_trk_1) as u32, (p_out_2) as u32, (p_cursor_3) as u32, (p_frame_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_n_5: i32 = 0;
    let mut l_times_6: u32 = 0;
    let mut l_keys_7: u32 = 0;
    let mut l_key_8: u32 = 0;
    let mut l_i_9: i32 = 0;
    let mut l_j_10: i32 = 0;
    let mut l_found_11: i32 = 0;
    let mut l_t_12: F32 = F32(0);
    let mut l_qa_13: u64 = 0;
    let mut l_qb_14: u64 = 0;
    let r_ = 'f: {
        l_n_5 = ((c.m.r16((p_trk_1).wrapping_add(2))) as i32);
        if ((((((((c.m.r16(p_trk_1)) as i32)) & (1i32))) as u16)) != 0) {
            l_times_6 = ((p_trk_1).wrapping_add((((l_n_5).wrapping_mul(8i32)) as u32))).wrapping_add(((4i32) as u32));
        } else {
            l_times_6 = ((p_trk_1).wrapping_add((((l_n_5).wrapping_mul(24i32)) as u32))).wrapping_add(((4i32) as u32));
        }
        if (((((l_n_5) == (1i32)) as i32)) != 0) {
            l_key_8 = (p_trk_1).wrapping_add(4);
            { let a1_ = (((c.m.r32((l_key_8).wrapping_add((((0i32) as u32)).wrapping_mul(8)))) as u64)).wrapping_add((((c.m.r32(((l_key_8).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add(4))) as u64)).wrapping_shl((32i32) as u32)); F::Quat_Unpack(c, p_out_2, a1_) };
            break 'f;
        }
        l_i_9 = ((c.m.r16(p_cursor_3)) as i32);
        l_keys_7 = (p_trk_1).wrapping_add(4);
        if (((((l_i_9) >= ((l_n_5).wrapping_sub(1i32))) as i32)) != 0) {
            { let v_ = ((0i32) as u16); let a_ = p_cursor_3; c.m.w16(a_, v_); };
            l_i_9 = 0i32;
        }
        l_found_11 = 0i32;
        'l1: while ((1i32) != 0) {
            l_key_8 = (l_keys_7).wrapping_add((((l_i_9) as u32)).wrapping_mul(8));
            {
                l_j_10 = (l_i_9).wrapping_add(1i32);
                'l2: loop {
                    if !(((((l_j_10) < (l_n_5)) as i32)) != 0) { break 'l2; }
                    'c3: {
                        { let o_ = l_key_8; let n_ = (o_).wrapping_add(8); l_key_8 = n_; };
                        if (((rt::feq(rt::i2f((c.m.r16((l_times_6).wrapping_add(((((l_j_10).wrapping_sub(1i32)) as u32)).wrapping_mul(2)))) as i32), p_frame_4) as i32)) != 0) {
                            { let a1_ = (((c.m.r32((l_key_8).wrapping_add(((((1i32).wrapping_neg()) as u32)).wrapping_mul(8)))) as u64)).wrapping_add((((c.m.r32(((l_key_8).wrapping_add(((((1i32).wrapping_neg()) as u32)).wrapping_mul(8))).wrapping_add(4))) as u64)).wrapping_shl((32i32) as u32)); F::Quat_Unpack(c, p_out_2, a1_) };
                            { let v_ = (((l_j_10).wrapping_sub(1i32)) as u16); let a_ = p_cursor_3; c.m.w16(a_, v_); };
                            l_found_11 = 1i32;
                            break 'l2;
                        }
                        if (((((((rt::flt(rt::i2f((c.m.r16((l_times_6).wrapping_add(((((l_j_10).wrapping_sub(1i32)) as u32)).wrapping_mul(2)))) as i32), p_frame_4) as i32)) != 0) && (((rt::flt(p_frame_4, rt::i2f((c.m.r16((l_times_6).wrapping_add((((l_j_10) as u32)).wrapping_mul(2)))) as i32)) as i32)) != 0)) as i32)) != 0) {
                            l_t_12 = rt::fdiv(rt::fsub(p_frame_4, rt::i2f((c.m.r16((l_times_6).wrapping_add(((((l_j_10).wrapping_sub(1i32)) as u32)).wrapping_mul(2)))) as i32)), rt::i2f((((c.m.r16((l_times_6).wrapping_add((((l_j_10) as u32)).wrapping_mul(2)))) as i32)).wrapping_sub(((c.m.r16((l_times_6).wrapping_add(((((l_j_10).wrapping_sub(1i32)) as u32)).wrapping_mul(2)))) as i32))));
                            l_qa_13 = (((c.m.r32((l_key_8).wrapping_add((((0i32) as u32)).wrapping_mul(8)))) as u64)).wrapping_add((((c.m.r32(((l_key_8).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add(4))) as u64)).wrapping_shl((32i32) as u32));
                            l_qb_14 = (((c.m.r32((l_key_8).wrapping_add(((((1i32).wrapping_neg()) as u32)).wrapping_mul(8)))) as u64)).wrapping_add((((c.m.r32(((l_key_8).wrapping_add(((((1i32).wrapping_neg()) as u32)).wrapping_mul(8))).wrapping_add(4))) as u64)).wrapping_shl((32i32) as u32));
                            F::Quat_Unpack(c, fp, l_qa_13);
                            F::Quat_Unpack(c, fp.wrapping_add(16), l_qb_14);
                            F::Quat_Slerp(c, p_out_2, fp.wrapping_add(16), fp, l_t_12);
                            { let v_ = (((l_j_10).wrapping_sub(1i32)) as u16); let a_ = p_cursor_3; c.m.w16(a_, v_); };
                            l_found_11 = 1i32;
                            break 'l2;
                        }
                        if (((rt::feq(rt::i2f((c.m.r16((l_times_6).wrapping_add((((l_j_10) as u32)).wrapping_mul(2)))) as i32), p_frame_4) as i32)) != 0) {
                            { let a1_ = (((c.m.r32((l_key_8).wrapping_add((((0i32) as u32)).wrapping_mul(8)))) as u64)).wrapping_add((((c.m.r32(((l_key_8).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add(4))) as u64)).wrapping_shl((32i32) as u32)); F::Quat_Unpack(c, p_out_2, a1_) };
                            { let v_ = (((l_j_10).wrapping_sub(1i32)) as u16); let a_ = p_cursor_3; c.m.w16(a_, v_); };
                            l_found_11 = 1i32;
                            break 'l2;
                        }
                    }
                    { let o_ = l_j_10; let n_ = (o_).wrapping_add(1); l_j_10 = n_; };
                }
            }
            if ((l_found_11) != 0) {
                break 'f;
            }
            if (((((((c.m.r16(p_cursor_3)) as i32)) == (0i32)) as i32)) != 0) {
                F::Vec4_Copy(c, p_out_2, sym::D_002EC2C0);
                break 'f;
            } else {
                { let v_ = ((0i32) as u16); let a_ = p_cursor_3; c.m.w16(a_, v_); };
                l_i_9 = 0i32;
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_SampleRot", &PROBE_N, &[], true); }
    r_
}

pub fn bobj_a__BobjKey_GetFrame(c: &mut Ctx, mut p_key_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "bobj_a__BobjKey_GetFrame", &PROBE_N, &[(p_key_1) as u32], false); }
    let r_ = 'f: {
    break 'f rt::u2f(c.m.r32((p_key_1).wrapping_add(12)));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "bobj_a__BobjKey_GetFrame", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjAnim_SamplePosRot(c: &mut Ctx, mut p_trk_1: u32, mut p_pos_2: u32, mut p_rot_3: u32, mut p_cursor_4: u32, mut p_frame_5: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_SamplePosRot", &PROBE_N, &[(p_trk_1) as u32, (p_pos_2) as u32, (p_rot_3) as u32, (p_cursor_4) as u32, (p_frame_5).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_n_6: i32 = 0;
    let mut l_keys_7: u32 = 0;
    let mut l_a_8: u32 = 0;
    let mut l_b_9: u32 = 0;
    let mut l_i_10: i32 = 0;
    let mut l_j_11: i32 = 0;
    let mut l_found_12: i32 = 0;
    let mut l_t_13: F32 = F32(0);
    let mut l_qa_14: u64 = 0;
    let mut l_qb_15: u64 = 0;
    let r_ = 'f: {
        l_n_6 = ((c.m.r16((p_trk_1).wrapping_add(2))) as i32);
        if (((((l_n_6) == (1i32)) as i32)) != 0) {
            l_b_9 = (p_trk_1).wrapping_add(4);
            { let v_ = F32(c.m.r32(l_b_9)); let a_ = p_pos_2; c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32((l_b_9).wrapping_add(4))); let a_ = (p_pos_2).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32((l_b_9).wrapping_add(8))); let a_ = (p_pos_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f800000); let a_ = (p_pos_2).wrapping_add(12); c.m.w32(a_, (v_).0); };
            { let a1_ = (((c.m.r32((l_b_9).wrapping_add(16))) as u64)).wrapping_add((((c.m.r32(((l_b_9).wrapping_add(16)).wrapping_add(4))) as u64)).wrapping_shl((32i32) as u32)); F::Quat_Unpack(c, p_rot_3, a1_) };
            break 'f;
        }
        l_i_10 = ((c.m.r16(p_cursor_4)) as i32);
        if (((((l_i_10) >= ((l_n_6).wrapping_sub(1i32))) as i32)) != 0) {
            { let v_ = ((0i32) as u16); let a_ = p_cursor_4; c.m.w16(a_, v_); };
            l_i_10 = 0i32;
        }
        l_keys_7 = (p_trk_1).wrapping_add(4);
        l_found_12 = 0i32;
        'l1: while ((1i32) != 0) {
            l_b_9 = (l_keys_7).wrapping_add((((l_i_10) as u32)).wrapping_mul(24));
            {
                l_j_11 = (l_i_10).wrapping_add(1i32);
                'l2: loop {
                    if !(((((l_j_11) < (l_n_6)) as i32)) != 0) { break 'l2; }
                    'c3: {
                        { let o_ = l_b_9; let n_ = (o_).wrapping_add(24); l_b_9 = n_; };
                        l_a_8 = (l_b_9).wrapping_sub((((1i32) as u32)).wrapping_mul(24));
                        if (((rt::feq(F::bobj_a__BobjKey_GetFrame(c, (l_b_9).wrapping_sub((((1i32) as u32)).wrapping_mul(24))), p_frame_5) as i32)) != 0) {
                            { let v_ = F32(c.m.r32(l_a_8)); let a_ = p_pos_2; c.m.w32(a_, (v_).0); };
                            { let v_ = F32(c.m.r32((l_a_8).wrapping_add(4))); let a_ = (p_pos_2).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(c.m.r32((l_a_8).wrapping_add(8))); let a_ = (p_pos_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(0x3f800000); let a_ = (p_pos_2).wrapping_add(12); c.m.w32(a_, (v_).0); };
                            { let a1_ = (((c.m.r32(((l_b_9).wrapping_add(((((1i32).wrapping_neg()) as u32)).wrapping_mul(24))).wrapping_add(16))) as u64)).wrapping_add((((c.m.r32((((l_b_9).wrapping_add(((((1i32).wrapping_neg()) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4))) as u64)).wrapping_shl((32i32) as u32)); F::Quat_Unpack(c, p_rot_3, a1_) };
                            { let v_ = (((l_j_11).wrapping_sub(1i32)) as u16); let a_ = p_cursor_4; c.m.w16(a_, v_); };
                            l_found_12 = 1i32;
                            break 'l2;
                        }
                        if (((((((rt::flt(F::bobj_a__BobjKey_GetFrame(c, (l_b_9).wrapping_sub((((1i32) as u32)).wrapping_mul(24))), p_frame_5) as i32)) != 0) && (((rt::flt(p_frame_5, F::bobj_a__BobjKey_GetFrame(c, l_b_9)) as i32)) != 0)) as i32)) != 0) {
                            l_t_13 = rt::fdiv(rt::fsub(p_frame_5, F::bobj_a__BobjKey_GetFrame(c, (l_b_9).wrapping_sub((((1i32) as u32)).wrapping_mul(24)))), rt::u2f((c.m.r32((l_b_9).wrapping_add(12))).wrapping_sub(c.m.r32(((l_b_9).wrapping_sub((((1i32) as u32)).wrapping_mul(24))).wrapping_add(12)))));
                            { let v_ = rt::fadd(rt::fmul(rt::fsub(F32(c.m.r32(l_b_9)), F32(c.m.r32(l_a_8))), l_t_13), F32(c.m.r32(l_a_8))); let a_ = p_pos_2; c.m.w32(a_, (v_).0); };
                            { let v_ = rt::fadd(rt::fmul(rt::fsub(F32(c.m.r32((l_b_9).wrapping_add(4))), F32(c.m.r32((l_a_8).wrapping_add(4)))), l_t_13), F32(c.m.r32((l_a_8).wrapping_add(4)))); let a_ = (p_pos_2).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            { let v_ = rt::fadd(rt::fmul(rt::fsub(F32(c.m.r32((l_b_9).wrapping_add(8))), F32(c.m.r32((l_a_8).wrapping_add(8)))), l_t_13), F32(c.m.r32((l_a_8).wrapping_add(8)))); let a_ = (p_pos_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(0x3f800000); let a_ = (p_pos_2).wrapping_add(12); c.m.w32(a_, (v_).0); };
                            l_qa_14 = (((c.m.r32((l_b_9).wrapping_add(16))) as u64)).wrapping_add((((c.m.r32(((l_b_9).wrapping_add(16)).wrapping_add(4))) as u64)).wrapping_shl((32i32) as u32));
                            l_qb_15 = (((c.m.r32(((l_b_9).wrapping_add(((((1i32).wrapping_neg()) as u32)).wrapping_mul(24))).wrapping_add(16))) as u64)).wrapping_add((((c.m.r32((((l_b_9).wrapping_add(((((1i32).wrapping_neg()) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4))) as u64)).wrapping_shl((32i32) as u32));
                            F::Quat_Unpack(c, fp, l_qa_14);
                            F::Quat_Unpack(c, fp.wrapping_add(16), l_qb_15);
                            F::Quat_Slerp(c, p_rot_3, fp.wrapping_add(16), fp, l_t_13);
                            { let v_ = (((l_j_11).wrapping_sub(1i32)) as u16); let a_ = p_cursor_4; c.m.w16(a_, v_); };
                            l_found_12 = 1i32;
                            break 'l2;
                        }
                        if (((rt::feq(F::bobj_a__BobjKey_GetFrame(c, l_b_9), p_frame_5) as i32)) != 0) {
                            { let v_ = F32(c.m.r32(l_b_9)); let a_ = p_pos_2; c.m.w32(a_, (v_).0); };
                            { let v_ = F32(c.m.r32((l_b_9).wrapping_add(4))); let a_ = (p_pos_2).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(c.m.r32((l_b_9).wrapping_add(8))); let a_ = (p_pos_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(0x3f800000); let a_ = (p_pos_2).wrapping_add(12); c.m.w32(a_, (v_).0); };
                            { let a1_ = (((c.m.r32((l_b_9).wrapping_add(16))) as u64)).wrapping_add((((c.m.r32(((l_b_9).wrapping_add(16)).wrapping_add(4))) as u64)).wrapping_shl((32i32) as u32)); F::Quat_Unpack(c, p_rot_3, a1_) };
                            { let v_ = (((l_j_11).wrapping_sub(1i32)) as u16); let a_ = p_cursor_4; c.m.w16(a_, v_); };
                            l_found_12 = 1i32;
                            break 'l2;
                        }
                    }
                    { let o_ = l_j_11; let n_ = (o_).wrapping_add(1); l_j_11 = n_; };
                }
            }
            if ((l_found_12) != 0) {
                break 'f;
            }
            if (((((((c.m.r16(p_cursor_4)) as i32)) == (0i32)) as i32)) != 0) {
                F::Vec4_Copy(c, p_rot_3, sym::D_002EC2C0);
                F::Vec4_Copy(c, p_pos_2, sym::D_002EC2C0);
                break 'f;
            } else {
                { let v_ = ((0i32) as u16); let a_ = p_cursor_4; c.m.w16(a_, v_); };
                l_i_10 = 0i32;
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_SamplePosRot", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjAnim_Load(c: &mut Ctx, mut p_obj_1: u32, mut p_buf_2: u32, mut p_anim_3: i32, mut p_size_4: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_Load", &PROBE_N, &[(p_obj_1) as u32, (p_buf_2) as u32, (p_anim_3) as u32, (p_size_4) as u32], false); }
    let mut l_tbl_5: u32 = 0;
    let mut l_ret_6: u32 = 0;
    let r_ = 'f: {
    l_tbl_5 = (p_obj_1).wrapping_add(24);
    if (((((p_anim_3) >= (414i32)) as i32)) != 0) {
        l_ret_6 = c.m.r32(((l_tbl_5).wrapping_add((((p_anim_3) as u32)).wrapping_mul(4))).wrapping_add((((42i32) as u32)).wrapping_mul(4)));
        { let v_ = 1i32; let a_ = p_size_4; c.m.w32(a_, (v_) as u32); };
    } else {
        if (((((c.m.r32((((p_obj_1).wrapping_add(24)).wrapping_add(168)).wrapping_add((((p_anim_3) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
            { let a0_ = c.m.r32((((p_obj_1).wrapping_add(24)).wrapping_add(168)).wrapping_add((((p_anim_3) as u32)).wrapping_mul(4))); { let _ = F::Bpe_Decode(c, a0_, p_buf_2, p_size_4); } };
            l_ret_6 = p_buf_2;
        } else {
            l_ret_6 = ((0i32) as u32);
        }
    }
    break 'f l_ret_6;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_Load", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjAnim_Init(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_Init", &PROBE_N, &[(p_obj_1) as u32], false); }
    let mut l_hit_2: u32 = 0;
    let mut l_ap_3: u32 = 0;
    let r_ = 'f: {
    l_hit_2 = (p_obj_1).wrapping_add(3216);
    l_ap_3 = (p_obj_1).wrapping_add(2880);
    { let v_ = F::BtlObj_GetUnk44FC4(c); let a_ = (l_ap_3).wrapping_add(320); c.m.w32(a_, (v_).0); };
    F::Quat_SetIdentity(c, (p_obj_1).wrapping_add(3360));
    F::Quat_SetIdentity(c, ((p_obj_1).wrapping_add(3360)).wrapping_add(16));
    F::Quat_SetIdentity(c, ((p_obj_1).wrapping_add(3360)).wrapping_add(32));
    { let v_ = F32(0x3f800000); let a_ = (l_hit_2).wrapping_add(40); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_Init", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjAnim_StartBlend(c: &mut Ctx, mut p_obj_1: u32, mut p_seconds_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_StartBlend", &PROBE_N, &[(p_obj_1) as u32, (p_seconds_2).0], false); }
    let mut l_ap_3: u32 = 0;
    let mut l_bone_4: u32 = 0;
    let mut l_part_5: u32 = 0;
    let r_ = 'f: {
    l_ap_3 = (p_obj_1).wrapping_add(2880);
    if ((((!(((rt::flt(F32(0x3d088888), p_seconds_2) as i32)) != 0)) as i32)) != 0) {
        { let v_ = F32(0x00000000); let a_ = (l_ap_3).wrapping_add(324); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (l_ap_3).wrapping_add(328); c.m.w32(a_, (v_).0); };
        break 'f;
    }
    { let v_ = F32(0x3f800000); let a_ = (l_ap_3).wrapping_add(324); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fdiv(F32(0x3f800000), rt::fmul(p_seconds_2, F32(0x41f00000))); let a_ = (l_ap_3).wrapping_add(328); c.m.w32(a_, (v_).0); };
    l_bone_4 = c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(44));
    {
        'l1: loop {
            'c2: {
                l_part_5 = { let a1_ = ((c.m.r16((l_bone_4).wrapping_add(10))) as i32); F::BtlObj_GetNode(c, p_obj_1, a1_) };
                F::Vec4_Copy(c, (l_part_5).wrapping_add(176), (l_part_5).wrapping_add(144));
                F::Vec4_Copy(c, (l_part_5).wrapping_add(192), (l_part_5).wrapping_add(160));
                if (((((((c.m.r16((l_bone_4).wrapping_add(6))) as i32)) != (0i32)) as i32)) != 0) {
                    break 'l1;
                }
                l_bone_4 = (l_bone_4).wrapping_add((((c.m.r32(l_bone_4) as i32)) as u32));
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_StartBlend", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjAnim_ZeroRootAxes(c: &mut Ctx, mut p_anim_1: u32, mut p_x_2: i32, mut p_y_3: i32, mut p_z_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_ZeroRootAxes", &PROBE_N, &[(p_anim_1) as u32, (p_x_2) as u32, (p_y_3) as u32, (p_z_4) as u32], false); }
    let mut l_trk_5: u32 = 0;
    let mut l_n_6: i32 = 0;
    let mut l_keys_7: u32 = 0;
    let mut l_i_8: i32 = 0;
    let r_ = 'f: {
    if (((((((c.m.r16(((p_anim_1).wrapping_add(6)).wrapping_add((((0i32) as u32)).wrapping_mul(2)))) as i32)) == (0i32)) as i32)) != 0) {
        break 'f;
    }
    l_trk_5 = (p_anim_1).wrapping_add(((((((c.m.r16(((p_anim_1).wrapping_add(6)).wrapping_add((((0i32) as u32)).wrapping_mul(2)))) as i32)).wrapping_mul(2i32)) as u32)).wrapping_mul(2));
    if ((((((((c.m.r16(l_trk_5)) as i32)) & (1i32))) as u16)) != 0) {
        break 'f;
    }
    l_n_6 = ((c.m.r16((l_trk_5).wrapping_add(2))) as i32);
    l_keys_7 = (l_trk_5).wrapping_add(4);
    if (((((p_x_2) != (0i32)) as i32)) != 0) {
        {
            l_i_8 = 0i32;
            'l1: loop {
                if !(((((l_i_8) < (l_n_6)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = F32(0x00000000); let a_ = (l_keys_7).wrapping_add((((l_i_8) as u32)).wrapping_mul(24)); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
            }
        }
    }
    if (((((p_y_3) != (0i32)) as i32)) != 0) {
        {
            l_i_8 = 0i32;
            'l3: loop {
                if !(((((l_i_8) < (l_n_6)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let v_ = F32(0x00000000); let a_ = ((l_keys_7).wrapping_add((((l_i_8) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
            }
        }
    }
    if (((((p_z_4) != (0i32)) as i32)) != 0) {
        {
            l_i_8 = 0i32;
            'l5: loop {
                if !(((((l_i_8) < (l_n_6)) as i32)) != 0) { break 'l5; }
                'c6: {
                    { let v_ = F32(0x00000000); let a_ = ((l_keys_7).wrapping_add((((l_i_8) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_ZeroRootAxes", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjAnim_RebaseRoot(c: &mut Ctx, mut p_anim_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_RebaseRoot", &PROBE_N, &[(p_anim_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_trk_2: u32 = 0;
    let mut l_keys_3: u32 = 0;
    let mut l_n_4: i32 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
        if (((((((c.m.r16(((p_anim_1).wrapping_add(6)).wrapping_add((((0i32) as u32)).wrapping_mul(2)))) as i32)) == (0i32)) as i32)) != 0) {
            break 'f;
        }
        l_trk_2 = (p_anim_1).wrapping_add(((((((c.m.r16(((p_anim_1).wrapping_add(6)).wrapping_add((((0i32) as u32)).wrapping_mul(2)))) as i32)).wrapping_mul(2i32)) as u32)).wrapping_mul(2));
        if ((((((((c.m.r16(l_trk_2)) as i32)) & (1i32))) as u16)) != 0) {
            break 'f;
        }
        l_keys_3 = (l_trk_2).wrapping_add(4);
        l_n_4 = ((c.m.r16((l_trk_2).wrapping_add(2))) as i32);
        { let v_ = F32(c.m.r32((l_keys_3).wrapping_add((((0i32) as u32)).wrapping_mul(24)))); let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_keys_3).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(4))); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_keys_3).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(8))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        {
            l_i_5 = 0i32;
            'l1: loop {
                if !(((((l_i_5) < (l_n_4)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let r_ = F32(c.m.r32(fp)); let a_ = (l_keys_3).wrapping_add((((l_i_5) as u32)).wrapping_mul(24)); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let r_ = F32(c.m.r32((fp).wrapping_add(4))); let a_ = ((l_keys_3).wrapping_add((((l_i_5) as u32)).wrapping_mul(24))).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let r_ = F32(c.m.r32((fp).wrapping_add(8))); let a_ = ((l_keys_3).wrapping_add((((l_i_5) as u32)).wrapping_mul(24))).wrapping_add(8); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_RebaseRoot", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjAnim_ClearNodeRot(c: &mut Ctx, mut p_anim_1: u32, mut p_node_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_ClearNodeRot", &PROBE_N, &[(p_anim_1) as u32, (p_node_2) as u32], false); }
    let mut l_trk_3: u32 = 0;
    let mut l_n_4: i32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_q_6: u32 = 0;
    let mut l_keys_7: u32 = 0;
    let r_ = 'f: {
    if (((((((c.m.r16(((p_anim_1).wrapping_add(6)).wrapping_add((((p_node_2) as u32)).wrapping_mul(2)))) as i32)) == (0i32)) as i32)) != 0) {
        break 'f;
    }
    l_trk_3 = (p_anim_1).wrapping_add(((((((c.m.r16(((p_anim_1).wrapping_add(6)).wrapping_add((((p_node_2) as u32)).wrapping_mul(2)))) as i32)).wrapping_mul(2i32)) as u32)).wrapping_mul(2));
    if ((((((((c.m.r16(l_trk_3)) as i32)) & (1i32))) as u16)) != 0) {
        l_n_4 = ((c.m.r16((l_trk_3).wrapping_add(2))) as i32);
        l_q_6 = (l_trk_3).wrapping_add(4);
        {
            l_i_5 = 0i32;
            'l1: loop {
                if !(((((l_i_5) < (l_n_4)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = 4294443007u32; let a_ = (l_q_6).wrapping_add((((l_i_5) as u32)).wrapping_mul(8)); c.m.w32(a_, v_); };
                    { let v_ = ((939523967i32) as u32); let a_ = ((l_q_6).wrapping_add((((l_i_5) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
    } else {
        l_n_4 = ((c.m.r16((l_trk_3).wrapping_add(2))) as i32);
        l_keys_7 = (l_trk_3).wrapping_add(4);
        {
            l_i_5 = 0i32;
            'l3: loop {
                if !(((((l_i_5) < (l_n_4)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let v_ = 4294443007u32; let a_ = ((l_keys_7).wrapping_add((((l_i_5) as u32)).wrapping_mul(24))).wrapping_add(16); c.m.w32(a_, v_); };
                    { let v_ = ((939523967i32) as u32); let a_ = (((l_keys_7).wrapping_add((((l_i_5) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_ClearNodeRot", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjAnim_SamplePose(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_SamplePose", &PROBE_N, &[(p_obj_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_ap_2: u32 = 0;
    let mut l_bone_3: u32 = 0;
    let mut l_part_4: u32 = 0;
    let mut l_trk_5: u32 = 0;
    let mut l_id_6: u32 = 0;
    let mut l_w_7: F32 = F32(0);
    let mut l_frame_8: F32 = F32(0);
    let mut l_dst_9: u32 = 0;
    let r_ = 'f: {
        l_ap_2 = (p_obj_1).wrapping_add(2880);
        { let v_ = ((p_obj_1).wrapping_add(2880)).wrapping_add((((1i32) as u32)).wrapping_mul(152)); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (l_ap_2).wrapping_add((((0i32) as u32)).wrapping_mul(152)); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        if (((((c.m.r32((l_ap_2).wrapping_add((((0i32) as u32)).wrapping_mul(152)))) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        l_w_7 = F32(0x00000000);
        l_bone_3 = c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(44));
        if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_ap_2).wrapping_add(324)))) as i32)) != 0) {
            l_w_7 = rt::fadd(rt::fmul(rt::fmul(rt::fmul(F32(c.m.r32((l_ap_2).wrapping_add(324))), F32(c.m.r32((l_ap_2).wrapping_add(324)))), F32(c.m.r32((l_ap_2).wrapping_add(324)))), F32(0xc0000000)), rt::fmul(rt::fmul(F32(c.m.r32((l_ap_2).wrapping_add(324))), F32(c.m.r32((l_ap_2).wrapping_add(324)))), F32(0x40400000)));
        }
        'l1: while ((1i32) != 0) {
            l_id_6 = ((c.m.r16((l_bone_3).wrapping_add(10))) as u32);
            l_part_4 = F::BtlObj_GetNode(c, p_obj_1, ((l_id_6) as i32));
            { let r_ = (((!(1i32))) as u32); let a_ = (l_part_4).wrapping_add(4); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
            if (((((l_id_6) < (((71i32) as u32))) as i32)) != 0) {
                if ((((((({ let a_ = (({ let a_ = c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))); c.m.r32(a_) }).wrapping_add(6)).wrapping_add((l_id_6).wrapping_mul(2)); c.m.r16(a_) }) as i32)) != (0i32)) as i32)) != 0) {
                    l_trk_5 = ({ let a_ = c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))); c.m.r32(a_) }).wrapping_add((((((({ let a_ = (({ let a_ = c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))); c.m.r32(a_) }).wrapping_add(6)).wrapping_add((l_id_6).wrapping_mul(2)); c.m.r16(a_) }) as i32)).wrapping_mul(2i32)) as u32)).wrapping_mul(2));
                    if ((((((((c.m.r16(l_trk_5)) as i32)) & (1i32))) as u16)) != 0) {
                        { let a2_ = ((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))).wrapping_add(10)).wrapping_add((l_id_6).wrapping_mul(2)); let a3_ = F32(c.m.r32((l_ap_2).wrapping_add(312))); F::BtlObjAnim_SampleRot(c, l_trk_5, (l_part_4).wrapping_add(160), a2_, a3_) };
                        l_dst_9 = (l_part_4).wrapping_add(144);
                        F::Vec4_Copy(c, l_dst_9, (l_bone_3).wrapping_add(48));
                    } else {
                        l_dst_9 = (l_part_4).wrapping_add(144);
                        { let a3_ = ((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))).wrapping_add(10)).wrapping_add((l_id_6).wrapping_mul(2)); let a4_ = F32(c.m.r32((l_ap_2).wrapping_add(312))); F::BtlObjAnim_SamplePosRot(c, l_trk_5, l_dst_9, (l_part_4).wrapping_add(160), a3_, a4_) };
                    }
                } else {
                    l_dst_9 = (l_part_4).wrapping_add(144);
                    F::Vec4_Copy(c, l_dst_9, (l_bone_3).wrapping_add(48));
                    { let v_ = F32(0x00000000); let a_ = (l_part_4).wrapping_add(160); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x00000000); let a_ = ((l_part_4).wrapping_add(160)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x00000000); let a_ = ((l_part_4).wrapping_add(160)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = ((l_part_4).wrapping_add(160)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                }
                if ((((((((({ let a_ = c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) && (((rt::flt(F32(0x00000000), F32(c.m.r32((l_ap_2).wrapping_add(332)))) as i32)) != 0)) as i32)) != 0) {
                    l_frame_8 = F32(c.m.r32((l_ap_2).wrapping_add(312)));
                    if (((rt::fne({ let a_ = (c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))).wrapping_add(4); F32(c.m.r32(a_)) }, { let a_ = (c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))).wrapping_add(4); F32(c.m.r32(a_)) }) as i32)) != 0) {
                        l_frame_8 = rt::fmul(l_frame_8, rt::fdiv({ let a_ = (c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))).wrapping_add(4); F32(c.m.r32(a_)) }, { let a_ = (c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))).wrapping_add(4); F32(c.m.r32(a_)) }));
                        if (((rt::flt({ let a_ = (c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))).wrapping_add(4); F32(c.m.r32(a_)) }, l_frame_8) as i32)) != 0) {
                            l_frame_8 = { let a_ = (c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))).wrapping_add(4); F32(c.m.r32(a_)) };
                        }
                    }
                    if ((((((({ let a_ = (({ let a_ = c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))); c.m.r32(a_) }).wrapping_add(6)).wrapping_add((l_id_6).wrapping_mul(2)); c.m.r16(a_) }) as i32)) != (0i32)) as i32)) != 0) {
                        l_trk_5 = ({ let a_ = c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))); c.m.r32(a_) }).wrapping_add((((((({ let a_ = (({ let a_ = c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))); c.m.r32(a_) }).wrapping_add(6)).wrapping_add((l_id_6).wrapping_mul(2)); c.m.r16(a_) }) as i32)).wrapping_mul(2i32)) as u32)).wrapping_mul(2));
                        if ((((((((c.m.r16(l_trk_5)) as i32)) & (1i32))) as u16)) != 0) {
                            { let a2_ = ((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))).wrapping_add(10)).wrapping_add((l_id_6).wrapping_mul(2)); F::BtlObjAnim_SampleRot(c, l_trk_5, fp.wrapping_add(24), a2_, l_frame_8) };
                            F::Vec4_Copy(c, fp.wrapping_add(8), (l_bone_3).wrapping_add(48));
                        } else {
                            { let a3_ = ((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))).wrapping_add(10)).wrapping_add((l_id_6).wrapping_mul(2)); F::BtlObjAnim_SamplePosRot(c, l_trk_5, fp.wrapping_add(8), fp.wrapping_add(24), a3_, l_frame_8) };
                        }
                        { let a3_ = F32(c.m.r32((l_ap_2).wrapping_add(332))); F::Vec4_Lerp(c, l_dst_9, fp.wrapping_add(8), l_dst_9, a3_) };
                        { let v_ = F32(0x3f800000); let a_ = ((l_part_4).wrapping_add(144)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        { let a3_ = F32(c.m.r32((l_ap_2).wrapping_add(332))); F::Quat_Slerp(c, (l_part_4).wrapping_add(160), (l_part_4).wrapping_add(160), fp.wrapping_add(24), a3_) };
                    }
                }
                if (((((((((l_id_6) != (((0i32) as u32))) as i32)) != 0) && (((rt::flt(F32(0x00000000), l_w_7) as i32)) != 0)) as i32)) != 0) {
                    F::Vec4_Lerp(c, l_dst_9, (l_part_4).wrapping_add(176), l_dst_9, l_w_7);
                    { let v_ = F32(0x3f800000); let a_ = ((l_part_4).wrapping_add(144)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    F::Quat_Slerp(c, (l_part_4).wrapping_add(160), (l_part_4).wrapping_add(160), (l_part_4).wrapping_add(192), l_w_7);
                }
            } else {
                F::Vec4_Copy(c, (l_part_4).wrapping_add(144), (l_bone_3).wrapping_add(48));
                { let v_ = F32(0x00000000); let a_ = (l_part_4).wrapping_add(160); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = ((l_part_4).wrapping_add(160)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = ((l_part_4).wrapping_add(160)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = ((l_part_4).wrapping_add(160)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            }
            if (((((((c.m.r16((l_bone_3).wrapping_add(6))) as i32)) != (0i32)) as i32)) != 0) {
                break 'l1;
            } else {
                l_bone_3 = (l_bone_3).wrapping_add((((c.m.r32(l_bone_3) as i32)) as u32));
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_SamplePose", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjAnim_UpdateEvents(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_UpdateEvents", &PROBE_N, &[(p_obj_1) as u32], false); }
    let mut l_hit_2: u32 = 0;
    let mut l_begin_3: i32 = 0;
    let mut l_ap_4: u32 = 0;
    let mut l_arg_5: u32 = 0;
    let mut l_shape_6: i32 = 0;
    let mut l_begins_7: i32 = 0;
    let mut l_ends_8: i32 = 0;
    let mut l_data_9: u32 = 0;
    let mut l_sphere_10: i32 = 0;
    let mut l_events_11: u32 = 0;
    let mut l_i_12: i32 = 0;
    let mut l_prev_13: i32 = 0;
    let mut l_cur_14: i32 = 0;
    let mut l_attr_15: i64 = 0;
    let mut l_end_16: i32 = 0;
    let mut l_n_17: i32 = 0;
    let mut l_k_18: i32 = 0;
    let mut l_s_19: u32 = 0;
    let r_ = 'f: {
    l_hit_2 = (p_obj_1).wrapping_add(3216);
    l_begin_3 = 0i32;
    l_ap_4 = (p_obj_1).wrapping_add(2880);
    l_shape_6 = 0i32;
    l_arg_5 = ((0i32) as u32);
    { let v_ = (((1i32).wrapping_neg()) as i8); let a_ = (l_hit_2).wrapping_add(29); c.m.w8(a_, (v_) as u8); };
    l_begins_7 = 0i32;
    { let v_ = ((0i32) as u32); let a_ = (l_hit_2).wrapping_add(12); c.m.w32(a_, v_); };
    l_ends_8 = 0i32;
    { let v_ = ((0i32) as i8); let a_ = (l_hit_2).wrapping_add(28); c.m.w8(a_, (v_) as u8); };
    l_data_9 = c.m.r32((l_ap_4).wrapping_add((((0i32) as u32)).wrapping_mul(152)));
    l_sphere_10 = 0i32;
    if (((((l_data_9) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    l_events_11 = (l_data_9).wrapping_add(148);
    l_cur_14 = rt::f2i(rt::fadd(F32(c.m.r32((l_ap_4).wrapping_add(312))), F32(0x3c888888)));
    l_prev_13 = rt::f2i(rt::fadd(F32(c.m.r32((l_ap_4).wrapping_add(316))), F32(0x3c888888)));
    if (((((l_cur_14) == (l_prev_13)) as i32)) != 0) {
        l_prev_13 = (l_prev_13).wrapping_sub(1i32);
    }
    {
        l_i_12 = 0i32;
        'l1: loop {
            if !(((((l_i_12) < (((c.m.r8((l_data_9).wrapping_add(1))) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                l_attr_15 = (((((c.m.r32((l_events_11).wrapping_add((((l_i_12) as u32)).wrapping_mul(16)))) as u64)).wrapping_add((((c.m.r32(((l_events_11).wrapping_add((((l_i_12) as u32)).wrapping_mul(16))).wrapping_add(4))) as u64)).wrapping_shl((32i32) as u32))) as i64);
                if ((((((l_attr_15) & (((1i32) as i64)))) as i32)) != 0) {
                    l_begin_3 = ((c.m.r16(((l_events_11).wrapping_add((((l_i_12) as u32)).wrapping_mul(16))).wrapping_add(8))) as i32);
                    l_arg_5 = c.m.r32(((l_events_11).wrapping_add((((l_i_12) as u32)).wrapping_mul(16))).wrapping_add(12));
                    l_sphere_10 = (((((l_attr_15).wrapping_shr((32i32) as u32)) as i32)) & (1i32));
                    l_shape_6 = ((c.m.r16(((l_events_11).wrapping_add((((l_i_12) as u32)).wrapping_mul(16))).wrapping_add(10))) as i32);
                    { let o_ = l_begins_7; let n_ = (o_).wrapping_add(1); l_begins_7 = n_; };
                }
                if ((((l_attr_15) & (((2i32) as i64)))) != 0) {
                    { let o_ = l_ends_8; let n_ = (o_).wrapping_add(1); l_ends_8 = n_; };
                    l_end_16 = ((c.m.r16(((l_events_11).wrapping_add((((l_i_12) as u32)).wrapping_mul(16))).wrapping_add(8))) as i32);
                    if (((((l_begins_7) == (l_ends_8)) as i32)) != 0) {
                        { let a_ = (l_hit_2).wrapping_add(28); let o_ = (c.m.r8(a_) as i8); let n_ = (o_).wrapping_add(1); c.m.w8(a_, (n_) as u8); };
                        if ((((((((c.m.r8((l_hit_2).wrapping_add(29)) as i8)) as i32)) < (0i32)) as i32)) != 0) {
                            if ((((((((!(((((l_cur_14) < (l_begin_3)) as i32)) != 0)) as i32)) != 0) && (((((l_prev_13) < (l_end_16)) as i32)) != 0)) as i32)) != 0) {
                                l_s_19 = (l_data_9).wrapping_add((((l_shape_6) as u32)).wrapping_mul(4));
                                { let v_ = F32(c.m.r32(l_s_19)); let a_ = l_hit_2; c.m.w32(a_, (v_).0); };
                                { let v_ = F32(c.m.r32((l_s_19).wrapping_add(4))); let a_ = (l_hit_2).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(c.m.r32((l_s_19).wrapping_add(8))); let a_ = (l_hit_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fmul(F32(c.m.r32((l_s_19).wrapping_add(16))), F32(c.m.r32((l_hit_2).wrapping_add(40)))); let a_ = (l_hit_2).wrapping_add(16); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fmul(F32(c.m.r32((l_s_19).wrapping_add(20))), F32(c.m.r32((l_hit_2).wrapping_add(40)))); let a_ = (l_hit_2).wrapping_add(20); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fmul(F32(c.m.r32((l_s_19).wrapping_add(24))), F32(c.m.r32((l_hit_2).wrapping_add(40)))); let a_ = (l_hit_2).wrapping_add(24); c.m.w32(a_, (v_).0); };
                                { let v_ = (((l_ends_8).wrapping_sub(1i32)) as i8); let a_ = (l_hit_2).wrapping_add(29); c.m.w8(a_, (v_) as u8); };
                                { let v_ = l_arg_5; let a_ = (l_hit_2).wrapping_add(12); c.m.w32(a_, v_); };
                                { let v_ = l_sphere_10; let a_ = (l_hit_2).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                            }
                        }
                    }
                }
            }
            { let o_ = l_i_12; let n_ = (o_).wrapping_add(1); l_i_12 = n_; };
        }
    }
    if ((((((((((((c.m.r8((l_hit_2).wrapping_add(28)) as i8)) as i32)) > (0i32)) as i32)) != 0) && ((((((((c.m.r8((l_hit_2).wrapping_add(29)) as i8)) as i32)) < (0i32)) as i32)) != 0)) as i32)) != 0) {
        { let v_ = (((1i32).wrapping_neg()) as i8); let a_ = (l_hit_2).wrapping_add(29); c.m.w8(a_, (v_) as u8); };
        {
            l_i_12 = 0i32;
            'l3: loop {
                if !(((((l_i_12) < (((c.m.r8((l_data_9).wrapping_add(1))) as i32))) as i32)) != 0) { break 'l3; }
                'c4: {
                    l_attr_15 = (((((c.m.r32((l_events_11).wrapping_add((((l_i_12) as u32)).wrapping_mul(16)))) as u64)).wrapping_add((((c.m.r32(((l_events_11).wrapping_add((((l_i_12) as u32)).wrapping_mul(16))).wrapping_add(4))) as u64)).wrapping_shl((32i32) as u32))) as i64);
                    if ((((((l_attr_15) & (((1i32) as i64)))) as i32)) != 0) {
                        if (((((l_cur_14) < (((c.m.r16(((l_events_11).wrapping_add((((l_i_12) as u32)).wrapping_mul(16))).wrapping_add(8))) as i32))) as i32)) != 0) {
                            break 'l3;
                        }
                        { let a_ = (l_hit_2).wrapping_add(29); let o_ = (c.m.r8(a_) as i8); let n_ = (o_).wrapping_sub(1); c.m.w8(a_, (n_) as u8); };
                    }
                }
                { let o_ = l_i_12; let n_ = (o_).wrapping_add(1); l_i_12 = n_; };
            }
        }
    }
    l_n_17 = 0i32;
    {
        l_i_12 = 0i32;
        'l5: loop {
            if !(((((l_i_12) < (((c.m.r8((l_data_9).wrapping_add(1))) as i32))) as i32)) != 0) { break 'l5; }
            'c6: {
                if (((((l_prev_13) > (0i32)) as i32)) != 0) {
                    if (((((l_cur_14) < (((c.m.r16(((l_events_11).wrapping_add((((l_i_12) as u32)).wrapping_mul(16))).wrapping_add(8))) as i32))) as i32)) != 0) {
                        break 'c6;
                    }
                    if ((((!(((((l_prev_13) < (((c.m.r16(((l_events_11).wrapping_add((((l_i_12) as u32)).wrapping_mul(16))).wrapping_add(8))) as i32))) as i32)) != 0)) as i32)) != 0) {
                        break 'c6;
                    }
                } else {
                    if (((((l_cur_14) < (((c.m.r16(((l_events_11).wrapping_add((((l_i_12) as u32)).wrapping_mul(16))).wrapping_add(8))) as i32))) as i32)) != 0) {
                        break 'c6;
                    }
                    if (((((((c.m.r16(((l_events_11).wrapping_add((((l_i_12) as u32)).wrapping_mul(16))).wrapping_add(8))) as i32)) < (l_prev_13)) as i32)) != 0) {
                        break 'c6;
                    }
                }
                if (((((l_n_17) >= (8i32)) as i32)) != 0) {
                    break 'c6;
                }
                l_attr_15 = (((((c.m.r32((l_events_11).wrapping_add((((l_i_12) as u32)).wrapping_mul(16)))) as u64)).wrapping_add((((c.m.r32(((l_events_11).wrapping_add((((l_i_12) as u32)).wrapping_mul(16))).wrapping_add(4))) as u64)).wrapping_shl((32i32) as u32))) as i64);
                {
                    l_k_18 = 0i32;
                    'l7: loop {
                        if !(((((l_k_18) < (l_n_17)) as i32)) != 0) { break 'l7; }
                        'c8: {
                            if (((((c.m.r64(((l_hit_2).wrapping_add(48)).wrapping_add((((l_k_18) as u32)).wrapping_mul(8)))) == (((l_attr_15) as u64))) as i32)) != 0) {
                                { let r_ = c.m.r32(((l_events_11).wrapping_add((((l_i_12) as u32)).wrapping_mul(16))).wrapping_add(12)); let a_ = ((l_hit_2).wrapping_add(112)).wrapping_add((((l_k_18) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                                break 'l7;
                            }
                        }
                        { let o_ = l_k_18; let n_ = (o_).wrapping_add(1); l_k_18 = n_; };
                    }
                }
                if (((((l_k_18) == (l_n_17)) as i32)) != 0) {
                    { let v_ = ((l_attr_15) as u64); let a_ = ((l_hit_2).wrapping_add(48)).wrapping_add((((l_n_17) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = c.m.r32(((l_events_11).wrapping_add((((l_i_12) as u32)).wrapping_mul(16))).wrapping_add(12)); let a_ = ((l_hit_2).wrapping_add(112)).wrapping_add((((l_n_17) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let o_ = l_n_17; let n_ = (o_).wrapping_add(1); l_n_17 = n_; };
                }
            }
            { let o_ = l_i_12; let n_ = (o_).wrapping_add(1); l_i_12 = n_; };
        }
    }
    { let v_ = ((l_n_17) as i8); let a_ = (l_hit_2).wrapping_add(30); c.m.w8(a_, (v_) as u8); };
    if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((17179869184i64) as u64))) != 0) {
        { let v_ = { let a0_ = F::BtlObjAnim_GetEventArg(c, p_obj_1, ((17179869184i64) as u64)); F::BtlObjAnim_MaskToNode(c, a0_) }; let a_ = (l_hit_2).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_UpdateEvents", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjAnim_SetStep(c: &mut Ctx, mut p_obj_1: u32, mut p_step_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_SetStep", &PROBE_N, &[(p_obj_1) as u32, (p_step_2).0], false); }
    let r_ = 'f: {
    { let v_ = p_step_2; let a_ = ((p_obj_1).wrapping_add(2880)).wrapping_add(320); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_SetStep", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjAnim_Play(c: &mut Ctx, mut p_obj_1: u32, mut p_layer_2: i32, mut p_anim_3: i32, mut p_reset_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_Play", &PROBE_N, &[(p_obj_1) as u32, (p_layer_2) as u32, (p_anim_3) as u32, (p_reset_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_ap_5: u32 = 0;
    let mut l_l_6: u32 = 0;
    let mut l_data_7: u32 = 0;
    let mut l_work_8: u32 = 0;
    let r_ = 'f: {
        l_ap_5 = (p_obj_1).wrapping_add(2880);
        l_work_8 = c.m.r32((p_obj_1).wrapping_add(5728));
        if (((((p_layer_2) == (0i32)) as i32)) != 0) {
            l_l_6 = (l_ap_5).wrapping_add((((0i32) as u32)).wrapping_mul(152));
            { let v_ = F::BtlObjAnim_Load(c, p_obj_1, (l_work_8).wrapping_add((((0i32) as u32)).wrapping_mul(49152)), p_anim_3, fp); let a_ = l_l_6; c.m.w32(a_, v_); };
        } else {
            l_data_7 = F::BtlObjAnim_Load(c, p_obj_1, (l_work_8).wrapping_add((((1i32) as u32)).wrapping_mul(49152)), p_anim_3, fp);
            l_l_6 = ((p_obj_1).wrapping_add(2880)).wrapping_add((((1i32) as u32)).wrapping_mul(152));
            { let v_ = l_data_7; let a_ = l_l_6; c.m.w32(a_, v_); };
        }
        if (((((c.m.r32(l_l_6)) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        { let v_ = 1i32; let a_ = (l_ap_5).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((p_anim_3) as u16); let a_ = (l_l_6).wrapping_add(8); c.m.w16(a_, v_); };
        { let v_ = rt::i2f(({ let a_ = (c.m.r32(l_l_6)).wrapping_add(2); c.m.r16(a_) }) as i32); let a_ = (l_l_6).wrapping_add(4); c.m.w32(a_, (v_).0); };
        if (((((p_layer_2) == (0i32)) as i32)) != 0) {
            { let v_ = 3i32; let a_ = ((p_obj_1).wrapping_add(3216)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            if (((((p_reset_4) != (0i32)) as i32)) != 0) {
                { let v_ = F32(0x00000000); let a_ = (l_ap_5).wrapping_add(312); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = (l_ap_5).wrapping_add(316); c.m.w32(a_, (v_).0); };
            } else {
                if (((rt::fgt(F32(c.m.r32((l_ap_5).wrapping_add(312))), F32(c.m.r32((l_l_6).wrapping_add(4)))) as i32)) != 0) {
                    { let v_ = F32(c.m.r32((l_l_6).wrapping_add(4))); let a_ = (l_ap_5).wrapping_add(312); c.m.w32(a_, (v_).0); };
                }
                if (((rt::fgt(F32(c.m.r32((l_ap_5).wrapping_add(316))), F32(c.m.r32((l_l_6).wrapping_add(4)))) as i32)) != 0) {
                    { let v_ = F32(c.m.r32((l_l_6).wrapping_add(4))); let a_ = (l_ap_5).wrapping_add(316); c.m.w32(a_, (v_).0); };
                }
            }
        }
        { let _ = F::memset(c, (l_l_6).wrapping_add(10), 0i32, 142u32); };
        if ((((({ let a_ = (c.m.r32((p_obj_1).wrapping_add(5728))).wrapping_add(98344); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
            { let a0_ = c.m.r32(l_l_6); F::BtlObjAnim_Retarget(c, a0_, p_obj_1, p_obj_1, 0i32, 1i32) };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_Play", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjAnim_PlayFrom(c: &mut Ctx, mut p_obj_1: u32, mut p_other_2: u32, mut p_anim_3: i32, mut p_useTable_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_PlayFrom", &PROBE_N, &[(p_obj_1) as u32, (p_other_2) as u32, (p_anim_3) as u32, (p_useTable_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_ap_5: u32 = 0;
    let mut l_data_6: u32 = 0;
    let r_ = 'f: {
        l_ap_5 = (p_obj_1).wrapping_add(2880);
        { let v_ = { let a1_ = (c.m.r32((p_obj_1).wrapping_add(5728))).wrapping_add((((0i32) as u32)).wrapping_mul(49152)); F::BtlObjAnim_Load(c, p_other_2, a1_, p_anim_3, fp) }; let a_ = (l_ap_5).wrapping_add((((0i32) as u32)).wrapping_mul(152)); c.m.w32(a_, v_); };
        { let v_ = 3i32; let a_ = ((p_obj_1).wrapping_add(3216)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        l_data_6 = c.m.r32((l_ap_5).wrapping_add((((0i32) as u32)).wrapping_mul(152)));
        if (((((l_data_6) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = 1i32; let a_ = (l_ap_5).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
            { let v_ = ((p_anim_3) as u16); let a_ = ((l_ap_5).wrapping_add((((0i32) as u32)).wrapping_mul(152))).wrapping_add(8); c.m.w16(a_, v_); };
            { let v_ = rt::i2f((c.m.r16((l_data_6).wrapping_add(2))) as i32); let a_ = ((l_ap_5).wrapping_add((((0i32) as u32)).wrapping_mul(152))).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (l_ap_5).wrapping_add(312); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (l_ap_5).wrapping_add(316); c.m.w32(a_, (v_).0); };
            { let _ = F::memset(c, ((l_ap_5).wrapping_add((((0i32) as u32)).wrapping_mul(152))).wrapping_add(10), 0i32, 142u32); };
            { let a0_ = c.m.r32((l_ap_5).wrapping_add((((0i32) as u32)).wrapping_mul(152))); F::BtlObjAnim_Retarget(c, a0_, p_obj_1, p_other_2, p_useTable_4, 0i32) };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_PlayFrom", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjAnim_PromoteLayer(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_PromoteLayer", &PROBE_N, &[(p_obj_1) as u32], false); }
    let mut l_ap_2: u32 = 0;
    let r_ = 'f: {
    l_ap_2 = (p_obj_1).wrapping_add(2880);
    { let s_ = c.m.r32((l_ap_2).wrapping_add((((1i32) as u32)).wrapping_mul(152))); let d_ = c.m.r32((l_ap_2).wrapping_add((((0i32) as u32)).wrapping_mul(152))); rt::memcpy(c, d_, s_, 49152); };
    { let v_ = c.m.r16(((l_ap_2).wrapping_add((((1i32) as u32)).wrapping_mul(152))).wrapping_add(8)); let a_ = ((l_ap_2).wrapping_add((((0i32) as u32)).wrapping_mul(152))).wrapping_add(8); c.m.w16(a_, v_); };
    { let v_ = F32(c.m.r32(((l_ap_2).wrapping_add((((1i32) as u32)).wrapping_mul(152))).wrapping_add(4))); let a_ = ((l_ap_2).wrapping_add((((0i32) as u32)).wrapping_mul(152))).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (l_ap_2).wrapping_add(312); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (l_ap_2).wrapping_add(316); c.m.w32(a_, (v_).0); };
    { let _ = F::memset(c, ((l_ap_2).wrapping_add((((0i32) as u32)).wrapping_mul(152))).wrapping_add(10), 0i32, 142u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_PromoteLayer", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjAnim_PlayAuto(c: &mut Ctx, mut p_obj_1: u32, mut p_anim_2: i32, mut p_mode_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_PlayAuto", &PROBE_N, &[(p_obj_1) as u32, (p_anim_2) as u32, (p_mode_3) as u32], false); }
    let mut l_ap_4: u32 = 0;
    let mut l_m_5: i32 = 0;
    let r_ = 'f: {
    l_ap_4 = (p_obj_1).wrapping_add(2880);
    if (((((p_anim_2) >= (414i32)) as i32)) != 0) {
        break 'f;
    }
    if (((((p_anim_2) < (0i32)) as i32)) != 0) {
        break 'f;
    }
    F::BtlObjAnim_Play(c, p_obj_1, 0i32, p_anim_2, 1i32);
    l_m_5 = (if (((((c.m.r32((l_ap_4).wrapping_add((((0i32) as u32)).wrapping_mul(152)))) != (((0i32) as u32))) as i32)) != 0) { p_mode_3 } else { 0i32 });
    { let v_ = 0i32; let a_ = (l_ap_4).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((l_m_5) as u32); let a_ = (l_ap_4).wrapping_add(308); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_PlayAuto", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjAnim_PlayModel(c: &mut Ctx, mut p_obj_1: u32, mut p_anim_2: i32, mut p_mode_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_PlayModel", &PROBE_N, &[(p_obj_1) as u32, (p_anim_2) as u32, (p_mode_3) as u32], false); }
    let mut l_mdl_4: u32 = 0;
    let mut l_ap_5: u32 = 0;
    let r_ = 'f: {
    l_mdl_4 = (p_obj_1).wrapping_add(24);
    l_ap_5 = (p_obj_1).wrapping_add(2880);
    if (((((p_anim_2) >= (8i32)) as i32)) != 0) {
        break 'f;
    }
    if (((((p_anim_2) >= (0i32)) as i32)) != 0) {
        { let v_ = c.m.r32(((l_mdl_4).wrapping_add(1824)).wrapping_add((((p_anim_2) as u32)).wrapping_mul(4))); let a_ = (l_ap_5).wrapping_add((((0i32) as u32)).wrapping_mul(152)); c.m.w32(a_, v_); };
        if (((((c.m.r32((l_ap_5).wrapping_add((((0i32) as u32)).wrapping_mul(152)))) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = ((p_anim_2) as u16); let a_ = ((l_ap_5).wrapping_add((((0i32) as u32)).wrapping_mul(152))).wrapping_add(8); c.m.w16(a_, v_); };
            { let v_ = F32(0x00000000); let a_ = (l_ap_5).wrapping_add(312); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (l_ap_5).wrapping_add(316); c.m.w32(a_, (v_).0); };
            { let v_ = rt::i2f(({ let a_ = (c.m.r32((l_ap_5).wrapping_add((((0i32) as u32)).wrapping_mul(152)))).wrapping_add(2); c.m.r16(a_) }) as i32); let a_ = ((l_ap_5).wrapping_add((((0i32) as u32)).wrapping_mul(152))).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = ((p_mode_3) as u32); let a_ = (l_ap_5).wrapping_add(308); c.m.w32(a_, v_); };
            { let _ = F::memset(c, ((l_ap_5).wrapping_add((((0i32) as u32)).wrapping_mul(152))).wrapping_add(10), 0i32, 142u32); };
        } else {
            { let v_ = ((0i32) as u32); let a_ = (l_ap_5).wrapping_add(308); c.m.w32(a_, v_); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_PlayModel", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjAnim_Step(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_Step", &PROBE_N, &[(p_obj_1) as u32], false); }
    let mut l_ap_2: u32 = 0;
    let r_ = 'f: {
    l_ap_2 = (p_obj_1).wrapping_add(2880);
    's1: {
        let sv3: u32 = c.m.r32((l_ap_2).wrapping_add(308));
        match sv3 {
            1u32 => {
                { let v_ = F32(c.m.r32((l_ap_2).wrapping_add(312))); let a_ = (l_ap_2).wrapping_add(316); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((l_ap_2).wrapping_add(312))), F32(c.m.r32((l_ap_2).wrapping_add(320)))); let a_ = (l_ap_2).wrapping_add(312); c.m.w32(a_, (v_).0); };
                if (((rt::fle(F32(c.m.r32(((l_ap_2).wrapping_add((((0i32) as u32)).wrapping_mul(152))).wrapping_add(4))), F32(c.m.r32((l_ap_2).wrapping_add(312)))) as i32)) != 0) {
                    { let v_ = ((0i32) as u32); let a_ = (l_ap_2).wrapping_add(308); c.m.w32(a_, v_); };
                    { let v_ = F32(c.m.r32(((l_ap_2).wrapping_add((((0i32) as u32)).wrapping_mul(152))).wrapping_add(4))); let a_ = (l_ap_2).wrapping_add(312); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32(((l_ap_2).wrapping_add((((0i32) as u32)).wrapping_mul(152))).wrapping_add(4))); let a_ = (l_ap_2).wrapping_add(316); c.m.w32(a_, (v_).0); };
                }
                break 's1;
            }
            0u32 => {
                break 's1;
            }
            2u32 => {
                { let v_ = F32(c.m.r32((l_ap_2).wrapping_add(312))); let a_ = (l_ap_2).wrapping_add(316); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((l_ap_2).wrapping_add(312))), F32(c.m.r32((l_ap_2).wrapping_add(320)))); let a_ = (l_ap_2).wrapping_add(312); c.m.w32(a_, (v_).0); };
                if (((rt::fle(F32(c.m.r32(((l_ap_2).wrapping_add((((0i32) as u32)).wrapping_mul(152))).wrapping_add(4))), F32(c.m.r32((l_ap_2).wrapping_add(312)))) as i32)) != 0) {
                    { let v_ = F32(0x00000000); let a_ = (l_ap_2).wrapping_add(312); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x00000000); let a_ = (l_ap_2).wrapping_add(316); c.m.w32(a_, (v_).0); };
                }
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_Step", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjAnim_TestEvent(c: &mut Ctx, mut p_obj_1: u32, mut p_mask_2: u64) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_TestEvent", &PROBE_N, &[(p_obj_1) as u32, (p_mask_2) as u32], false); }
    let mut l_bits_3: u64 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_bits_3 = ((0i32) as u64);
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < ((((c.m.r8(((p_obj_1).wrapping_add(3216)).wrapping_add(30)) as i8)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let r_ = ((c.m.r64((((p_obj_1).wrapping_add(3216)).wrapping_add(48)).wrapping_add((((l_i_4) as u32)).wrapping_mul(8)))) & (p_mask_2)); let n_ = ((l_bits_3) | (r_)); l_bits_3 = n_; };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    break 'f (((l_bits_3) != (((0i32) as u64))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_TestEvent", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjAnim_GetEventArg(c: &mut Ctx, mut p_obj_1: u32, mut p_mask_2: u64) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_GetEventArg", &PROBE_N, &[(p_obj_1) as u32, (p_mask_2) as u32], false); }
    let mut l_arg_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_arg_3 = ((0i32) as u32);
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < ((((c.m.r8(((p_obj_1).wrapping_add(3216)).wrapping_add(30)) as i8)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((c.m.r64((((p_obj_1).wrapping_add(3216)).wrapping_add(48)).wrapping_add((((l_i_4) as u32)).wrapping_mul(8)))) & (p_mask_2))) != 0) {
                    { let r_ = c.m.r32((((p_obj_1).wrapping_add(3216)).wrapping_add(112)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))); let n_ = ((l_arg_3) | (r_)); l_arg_3 = n_; };
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    break 'f l_arg_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_GetEventArg", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjAnim_MaskToNode(c: &mut Ctx, mut p_mask_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_MaskToNode", &PROBE_N, &[(p_mask_1) as u32], false); }
    let r_ = 'f: {
    if ((((p_mask_1) & (((1i32) as u32)))) != 0) {
        break 'f 3i32;
    }
    if ((((p_mask_1) & (((2i32) as u32)))) != 0) {
        break 'f 4i32;
    }
    if ((((p_mask_1) & (((4i32) as u32)))) != 0) {
        break 'f 10i32;
    }
    if ((((p_mask_1) & (((8i32) as u32)))) != 0) {
        break 'f 11i32;
    }
    if ((((p_mask_1) & (((64i32) as u32)))) != 0) {
        break 'f 19i32;
    }
    if ((((p_mask_1) & (((128i32) as u32)))) != 0) {
        break 'f 20i32;
    }
    if ((((p_mask_1) & (((256i32) as u32)))) != 0) {
        break 'f 21i32;
    }
    if ((((p_mask_1) & (((16i32) as u32)))) != 0) {
        break 'f 14i32;
    }
    if ((((p_mask_1) & (((32i32) as u32)))) != 0) {
        break 'f 15i32;
    }
    if ((((p_mask_1) & (((512i32) as u32)))) != 0) {
        break 'f 33i32;
    }
    if ((((p_mask_1) & (((1024i32) as u32)))) != 0) {
        break 'f 34i32;
    }
    if ((((p_mask_1) & (((2048i32) as u32)))) != 0) {
        break 'f 35i32;
    }
    if ((((p_mask_1) & (((4096i32) as u32)))) != 0) {
        break 'f 46i32;
    }
    if ((((p_mask_1) & (((8192i32) as u32)))) != 0) {
        break 'f 64i32;
    }
    if ((((p_mask_1) & (((16384i32) as u32)))) != 0) {
        break 'f 65i32;
    }
    if ((((p_mask_1) & (((32768i32) as u32)))) != 0) {
        break 'f 0i32;
    }
    if ((((p_mask_1) & (((65536i32) as u32)))) != 0) {
        break 'f 31i32;
    }
    if ((((p_mask_1) & (((131072i32) as u32)))) != 0) {
        break 'f 45i32;
    }
    if ((((p_mask_1) & (((262144i32) as u32)))) != 0) {
        break 'f 54i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_MaskToNode", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjAnim_QueryEvent(c: &mut Ctx, mut p_obj_1: u32, mut p_mask_2: u64, mut p_layer_3: i32, mut p_what_4: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_QueryEvent", &PROBE_N, &[(p_obj_1) as u32, (p_mask_2) as u32, (p_layer_3) as u32, (p_what_4) as u32], false); }
    let mut l_ap_5: u32 = 0;
    let mut l_first_6: i32 = 0;
    let mut l_next_7: i32 = 0;
    let mut l_last_8: i32 = 0;
    let mut l_firstArg_9: u32 = 0;
    let mut l_nextArg_10: u32 = 0;
    let mut l_data_11: u32 = 0;
    let mut l_lastArg_12: u32 = 0;
    let mut l_count_13: i32 = 0;
    let mut l_ahead_14: i32 = 0;
    let mut l_reached_15: i32 = 0;
    let mut l_ev_16: u32 = 0;
    let mut l_i_17: i32 = 0;
    let mut l_events_18: u32 = 0;
    let r_ = 'f: {
    l_ap_5 = (p_obj_1).wrapping_add(2880);
    l_first_6 = (1i32).wrapping_neg();
    l_next_7 = (1i32).wrapping_neg();
    l_last_8 = (1i32).wrapping_neg();
    l_firstArg_9 = ((0i32) as u32);
    l_nextArg_10 = ((0i32) as u32);
    l_data_11 = c.m.r32((l_ap_5).wrapping_add((((p_layer_3) as u32)).wrapping_mul(152)));
    l_lastArg_12 = ((0i32) as u32);
    l_count_13 = 0i32;
    l_ahead_14 = 0i32;
    l_reached_15 = 0i32;
    if (((((l_data_11) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    {
        l_events_18 = (l_data_11).wrapping_add(148);
        {
            l_i_17 = 0i32;
            'l1: loop {
                if !(((((l_i_17) < (((c.m.r8((l_data_11).wrapping_add(1))) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((((c.m.r32((l_events_18).wrapping_add((((l_i_17) as u32)).wrapping_mul(16)))) as u64)).wrapping_add((((c.m.r32(((l_events_18).wrapping_add((((l_i_17) as u32)).wrapping_mul(16))).wrapping_add(4))) as u64)).wrapping_shl((32i32) as u32))) & (p_mask_2))) != 0) {
                        if (((((l_first_6) < (0i32)) as i32)) != 0) {
                            l_first_6 = ((c.m.r16(((l_events_18).wrapping_add((((l_i_17) as u32)).wrapping_mul(16))).wrapping_add(8))) as i32);
                            l_firstArg_9 = c.m.r32(((l_events_18).wrapping_add((((l_i_17) as u32)).wrapping_mul(16))).wrapping_add(12));
                        }
                        if (((((l_next_7) < (0i32)) as i32)) != 0) {
                            if (((rt::flt(F32(c.m.r32((l_ap_5).wrapping_add(312))), rt::i2f((c.m.r16(((l_events_18).wrapping_add((((l_i_17) as u32)).wrapping_mul(16))).wrapping_add(8))) as i32)) as i32)) != 0) {
                                l_next_7 = ((c.m.r16(((l_events_18).wrapping_add((((l_i_17) as u32)).wrapping_mul(16))).wrapping_add(8))) as i32);
                                l_nextArg_10 = c.m.r32(((l_events_18).wrapping_add((((l_i_17) as u32)).wrapping_mul(16))).wrapping_add(12));
                            }
                        }
                        l_last_8 = ((c.m.r16(((l_events_18).wrapping_add((((l_i_17) as u32)).wrapping_mul(16))).wrapping_add(8))) as i32);
                        { let o_ = l_count_13; let n_ = (o_).wrapping_add(1); l_count_13 = n_; };
                        l_lastArg_12 = c.m.r32(((l_events_18).wrapping_add((((l_i_17) as u32)).wrapping_mul(16))).wrapping_add(12));
                        if (((rt::flt(F32(c.m.r32((l_ap_5).wrapping_add(312))), rt::i2f(l_last_8)) as i32)) != 0) {
                            { let o_ = l_ahead_14; let n_ = (o_).wrapping_add(1); l_ahead_14 = n_; };
                        }
                        if (((rt::fle(rt::i2f(l_last_8), F32(c.m.r32((l_ap_5).wrapping_add(312)))) as i32)) != 0) {
                            { let o_ = l_reached_15; let n_ = (o_).wrapping_add(1); l_reached_15 = n_; };
                        }
                    }
                }
                { let o_ = l_i_17; let n_ = (o_).wrapping_add(1); l_i_17 = n_; };
            }
        }
    }
    's3: {
        let sv19: u32 = p_what_4;
        match sv19 {
            0u32 => {
                break 'f l_first_6;
            }
            1u32 => {
                break 'f l_next_7;
            }
            2u32 => {
                break 'f l_last_8;
            }
            3u32 => {
                break 'f l_count_13;
            }
            4u32 => {
                break 'f l_ahead_14;
            }
            5u32 => {
                break 'f l_reached_15;
            }
            6u32 => {
                break 'f ((l_firstArg_9) as i32);
            }
            7u32 => {
                break 'f ((l_nextArg_10) as i32);
            }
            8u32 => {
                break 'f ((l_lastArg_12) as i32);
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_QueryEvent", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjAnim_GetMode(c: &mut Ctx, mut p_obj_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_GetMode", &PROBE_N, &[(p_obj_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((c.m.r32(((p_obj_1).wrapping_add(2880)).wrapping_add(308))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjAnim_GetMode", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjBody_Exists(c: &mut Ctx, mut p_obj_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjBody_Exists", &PROBE_N, &[(p_obj_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(60))) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f ((((c.m.r32(p_obj_1) as i32)) == (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjBody_Exists", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjBody_Swap(c: &mut Ctx, mut p_body_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjBody_Swap", &PROBE_N, &[(p_body_1) as u32], false); }
    let mut l_old_2: i32 = 0;
    let r_ = 'f: {
    if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
        l_old_2 = (c.m.r32((p_body_1).wrapping_add(88)) as i32);
        { let v_ = ((p_body_1).wrapping_add(16)).wrapping_add((((l_old_2) as u32)).wrapping_mul(32)); let a_ = (p_body_1).wrapping_add(80); c.m.w32(a_, v_); };
        { let v_ = ((p_body_1).wrapping_add(16)).wrapping_add((((((l_old_2) ^ (1i32))) as u32)).wrapping_mul(32)); let a_ = (p_body_1).wrapping_add(84); c.m.w32(a_, v_); };
        { let v_ = ((l_old_2) ^ (1i32)); let a_ = (p_body_1).wrapping_add(88); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjBody_Swap", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjObb_SetMtx(c: &mut Ctx, mut p_box_1: u32, mut p_m_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjObb_SetMtx", &PROBE_N, &[(p_box_1) as u32, (p_m_2) as u32], false); }
    let r_ = 'f: {
    F::Mtx_Copy(c, (p_box_1).wrapping_add(80), p_m_2);
    { let v_ = F32(c.m.r32((((p_box_1).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_box_1).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((p_box_1).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (((p_box_1).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((p_box_1).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (((p_box_1).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((p_box_1).wrapping_add(80)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_box_1).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((p_box_1).wrapping_add(80)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (((p_box_1).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((p_box_1).wrapping_add(80)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (((p_box_1).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((p_box_1).wrapping_add(80)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_box_1).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((p_box_1).wrapping_add(80)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (((p_box_1).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((p_box_1).wrapping_add(80)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (((p_box_1).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((p_box_1).wrapping_add(80)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = p_box_1; c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((p_box_1).wrapping_add(80)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (p_box_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((p_box_1).wrapping_add(80)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (p_box_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
    F::Mtx_MulVec4(c, ((p_box_1).wrapping_add(272)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (p_box_1).wrapping_add(80), ((p_box_1).wrapping_add(144)).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
    F::Mtx_MulVec4(c, ((p_box_1).wrapping_add(272)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (p_box_1).wrapping_add(80), ((p_box_1).wrapping_add(144)).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
    F::Mtx_MulVec4(c, ((p_box_1).wrapping_add(272)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (p_box_1).wrapping_add(80), ((p_box_1).wrapping_add(144)).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
    F::Mtx_MulVec4(c, ((p_box_1).wrapping_add(272)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (p_box_1).wrapping_add(80), ((p_box_1).wrapping_add(144)).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
    F::Mtx_MulVec4(c, ((p_box_1).wrapping_add(272)).wrapping_add((((4i32) as u32)).wrapping_mul(16)), (p_box_1).wrapping_add(80), ((p_box_1).wrapping_add(144)).wrapping_add((((4i32) as u32)).wrapping_mul(16)));
    F::Mtx_MulVec4(c, ((p_box_1).wrapping_add(272)).wrapping_add((((5i32) as u32)).wrapping_mul(16)), (p_box_1).wrapping_add(80), ((p_box_1).wrapping_add(144)).wrapping_add((((5i32) as u32)).wrapping_mul(16)));
    F::Mtx_MulVec4(c, ((p_box_1).wrapping_add(272)).wrapping_add((((6i32) as u32)).wrapping_mul(16)), (p_box_1).wrapping_add(80), ((p_box_1).wrapping_add(144)).wrapping_add((((6i32) as u32)).wrapping_mul(16)));
    F::Mtx_MulVec4(c, ((p_box_1).wrapping_add(272)).wrapping_add((((7i32) as u32)).wrapping_mul(16)), (p_box_1).wrapping_add(80), ((p_box_1).wrapping_add(144)).wrapping_add((((7i32) as u32)).wrapping_mul(16)));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjObb_SetMtx", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjBody_SetCenter(c: &mut Ctx, mut p_dst_1: u32, mut p_src_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjBody_SetCenter", &PROBE_N, &[(p_dst_1) as u32, (p_src_2) as u32], false); }
    let r_ = 'f: {
    F::Vec4_Copy(c, p_dst_1, p_src_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjBody_SetCenter", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjBody_UpdateSphere(c: &mut Ctx, mut p_obj_1: u32, mut p_body_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjBody_UpdateSphere", &PROBE_N, &[(p_obj_1) as u32, (p_body_2) as u32], false); }
    let mut l_s_3: u32 = 0;
    let mut l_part_4: u32 = 0;
    let mut l_y_5: F32 = F32(0);
    let r_ = 'f: {
    l_s_3 = c.m.r32((p_body_2).wrapping_add(80));
    l_part_4 = F::BtlObj_GetNode(c, p_obj_1, 0i32);
    if (((((l_part_4) != (((0i32) as u32))) as i32)) != 0) {
        l_y_5 = F32(c.m.r32((((l_part_4).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))));
        F::Vec4_Copy(c, l_s_3, ((l_part_4).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
        l_part_4 = F::BtlObj_GetNode(c, p_obj_1, 17i32);
        if (((((l_part_4) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = F32(c.m.r32((p_body_2).wrapping_add(180))); let a_ = (l_s_3).wrapping_add(16); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul(F32(c.m.r32((p_body_2).wrapping_add(180))), F32(c.m.r32((p_body_2).wrapping_add(180)))); let a_ = (l_s_3).wrapping_add(20); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fsub(F32(c.m.r32((((l_part_4).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(0x3f999999)); let a_ = (l_s_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
            if (((rt::flt(rt::fsub(l_y_5, F32(0x3f999999)), rt::fadd(F32(c.m.r32((l_s_3).wrapping_add(4))), F32(c.m.r32((l_s_3).wrapping_add(16))))) as i32)) != 0) {
                { let v_ = rt::fsub(rt::fsub(l_y_5, F32(c.m.r32((l_s_3).wrapping_add(16)))), F32(0x3f999999)); let a_ = (l_s_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjBody_UpdateSphere", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjBody_UpdateParts(c: &mut Ctx, mut p_body_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjBody_UpdateParts", &PROBE_N, &[(p_body_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_part_2: u32 = 0;
    let r_ = 'f: {
        l_part_2 = c.m.r32(p_body_1);
        F::StgAabb_SetEmpty(c, (p_body_1).wrapping_add(112));
        F::StgAabb_SetEmpty(c, (p_body_1).wrapping_add(136));
        {
            'l1: loop {
                'c2: {
                    { let a2_ = (c.m.r32((l_part_2).wrapping_add(8))).wrapping_add(16); F::Vec3_Sub(c, fp, (l_part_2).wrapping_add(16), a2_) };
                    { let a1_ = (c.m.r32((l_part_2).wrapping_add(12))).wrapping_add(16); F::Mtx_MulVec4(c, fp.wrapping_add(16), a1_, fp) };
                    { let a1_ = (c.m.r32((l_part_2).wrapping_add(12))).wrapping_add(16); F::Mtx_Copy(c, fp.wrapping_add(32), a1_) };
                    F::Vec4_Copy(c, (fp.wrapping_add(32)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), fp.wrapping_add(16));
                    F::BtlObjObb_SetMtx(c, (l_part_2).wrapping_add(64), fp.wrapping_add(32));
                    F::BtlObjBody_SetCenter(c, (l_part_2).wrapping_add(32), (fp.wrapping_add(32)).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
                    F::ColBox_AddPoint(c, (p_body_1).wrapping_add(112), (((l_part_2).wrapping_add(64)).wrapping_add(272)).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
                    F::ColBox_AddPoint(c, (p_body_1).wrapping_add(112), (((l_part_2).wrapping_add(64)).wrapping_add(272)).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
                    F::ColBox_AddPoint(c, (p_body_1).wrapping_add(112), (((l_part_2).wrapping_add(64)).wrapping_add(272)).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
                    F::ColBox_AddPoint(c, (p_body_1).wrapping_add(112), (((l_part_2).wrapping_add(64)).wrapping_add(272)).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
                    F::ColBox_AddPoint(c, (p_body_1).wrapping_add(112), (((l_part_2).wrapping_add(64)).wrapping_add(272)).wrapping_add((((4i32) as u32)).wrapping_mul(16)));
                    F::ColBox_AddPoint(c, (p_body_1).wrapping_add(112), (((l_part_2).wrapping_add(64)).wrapping_add(272)).wrapping_add((((5i32) as u32)).wrapping_mul(16)));
                    F::ColBox_AddPoint(c, (p_body_1).wrapping_add(112), (((l_part_2).wrapping_add(64)).wrapping_add(272)).wrapping_add((((6i32) as u32)).wrapping_mul(16)));
                    F::ColBox_AddPoint(c, (p_body_1).wrapping_add(112), (((l_part_2).wrapping_add(64)).wrapping_add(272)).wrapping_add((((7i32) as u32)).wrapping_mul(16)));
                    F::ColBounds_AddSphere(c, (p_body_1).wrapping_add(136), (l_part_2).wrapping_add(32));
                    if ((((c.m.r32(l_part_2)) & (((1i32) as u32)))) != 0) {
                        break 'l1;
                    }
                    { let o_ = l_part_2; let n_ = (o_).wrapping_add(464); l_part_2 = n_; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlObjBody_UpdateParts", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjBody_Init(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjBody_Init", &PROBE_N, &[(p_obj_1) as u32], false); }
    let mut l_body_2: u32 = 0;
    let mut l_mdl_3: u32 = 0;
    let mut l_param_4: u32 = 0;
    let r_ = 'f: {
    l_body_2 = (p_obj_1).wrapping_add(3920);
    l_mdl_3 = (p_obj_1).wrapping_add(24);
    l_param_4 = c.m.r32((l_mdl_3).wrapping_add(2308));
    { let _ = F::memset(c, l_body_2, 0i32, 192u32); };
    { let v_ = { let a_ = ((c.m.r32((l_mdl_3).wrapping_add(40))).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = ((l_body_2).wrapping_add(164)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = { let a_ = ((c.m.r32((l_mdl_3).wrapping_add(40))).wrapping_add(24)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = ((l_body_2).wrapping_add(164)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = { let a_ = ((c.m.r32((l_mdl_3).wrapping_add(40))).wrapping_add(24)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = ((l_body_2).wrapping_add(164)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = { let a_ = ((c.m.r32((l_mdl_3).wrapping_add(40))).wrapping_add(24)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = ((l_body_2).wrapping_add(164)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    if (((((l_param_4) != (((0i32) as u32))) as i32)) != 0) {
        if (((rt::fle(F32(c.m.r32((l_param_4).wrapping_add(8))), F32(0x00000000)) as i32)) != 0) {
            { let v_ = rt::fadd(F32(c.m.r32(((l_body_2).wrapping_add(164)).wrapping_add((((3i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_body_2).wrapping_add(164)).wrapping_add((((3i32) as u32)).wrapping_mul(4))))); let a_ = (l_body_2).wrapping_add(180); c.m.w32(a_, (v_).0); };
        } else {
            { let v_ = F32(c.m.r32((l_param_4).wrapping_add(8))); let a_ = (l_body_2).wrapping_add(180); c.m.w32(a_, (v_).0); };
        }
    } else {
        { let v_ = F32(0x40833333); let a_ = (l_body_2).wrapping_add(180); c.m.w32(a_, (v_).0); };
    }
    if ((F::BtlObjBody_Exists(c, p_obj_1)) != 0) {
        { let v_ = { let a_ = (c.m.r32((l_mdl_3).wrapping_add(60))).wrapping_add(4); F32(c.m.r32(a_)) }; let a_ = (l_body_2).wrapping_add(160); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x40833333); let a_ = (l_body_2).wrapping_add(184); c.m.w32(a_, (v_).0); };
        { let v_ = (c.m.r32((l_mdl_3).wrapping_add(60))).wrapping_add(16); let a_ = l_body_2; c.m.w32(a_, v_); };
        l_body_2 = c.m.r32(l_body_2);
        {
            'l1: loop {
                'c2: {
                    { let v_ = { let a1_ = (c.m.r32((l_body_2).wrapping_add(4)) as i32); F::BtlObj_GetNode(c, p_obj_1, a1_) }; let a_ = (l_body_2).wrapping_add(12); c.m.w32(a_, v_); };
                    { let v_ = { let a1_ = (c.m.r32((l_body_2).wrapping_add(4)) as i32); F::BtlObj_FindBound(c, p_obj_1, a1_) }; let a_ = (l_body_2).wrapping_add(8); c.m.w32(a_, v_); };
                    if ((((c.m.r32(l_body_2)) & (((1i32) as u32)))) != 0) {
                        break 'l1;
                    }
                    l_body_2 = (l_body_2).wrapping_add((((1i32) as u32)).wrapping_mul(464));
                }
            }
        }
        F::BtlObjBody_Reset(c, p_obj_1);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjBody_Init", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjBody_Update(c: &mut Ctx, mut p_obj_1: u32, mut p_parts_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjBody_Update", &PROBE_N, &[(p_obj_1) as u32, (p_parts_2) as u32], false); }
    let r_ = 'f: {
    if ((F::BtlObjBody_Exists(c, p_obj_1)) != 0) {
        if (((((p_parts_2) == (0i32)) as i32)) != 0) {
            F::BtlObjBody_UpdateSphere(c, p_obj_1, (p_obj_1).wrapping_add(3920));
        } else {
            F::BtlObjBody_UpdateParts(c, (p_obj_1).wrapping_add(3920));
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjBody_Update", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjBody_BeginFrame(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjBody_BeginFrame", &PROBE_N, &[(p_obj_1) as u32], false); }
    let mut l_work_2: u32 = 0;
    let mut l_contact_3: u32 = 0;
    let r_ = 'f: {
    if ((F::BtlObjBody_Exists(c, p_obj_1)) != 0) {
        l_work_2 = c.m.r32((p_obj_1).wrapping_add(5728));
        if (((((l_work_2) != (((0i32) as u32))) as i32)) != 0) {
            l_contact_3 = c.m.r32((l_work_2).wrapping_add(98336));
            { let v_ = ((0i32) as u32); let a_ = (l_work_2).wrapping_add(98336); c.m.w32(a_, v_); };
            { let v_ = l_contact_3; let a_ = (l_work_2).wrapping_add(98340); c.m.w32(a_, v_); };
        }
        F::BtlObjBody_Swap(c, (p_obj_1).wrapping_add(3920));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjBody_BeginFrame", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjBody_Reset(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjBody_Reset", &PROBE_N, &[(p_obj_1) as u32], false); }
    let mut l_body_2: u32 = 0;
    let mut l_old_3: i32 = 0;
    let r_ = 'f: {
    if ((F::BtlObjBody_Exists(c, p_obj_1)) != 0) {
        l_body_2 = (p_obj_1).wrapping_add(3920);
        l_old_3 = (c.m.r32((l_body_2).wrapping_add(88)) as i32);
        { let v_ = 1i32; let a_ = (l_body_2).wrapping_add(188); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((l_old_3) ^ (1i32)); let a_ = (l_body_2).wrapping_add(88); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((l_body_2).wrapping_add(16)).wrapping_add((((l_old_3) as u32)).wrapping_mul(32)); let a_ = (l_body_2).wrapping_add(84); c.m.w32(a_, v_); };
        { let v_ = ((l_body_2).wrapping_add(16)).wrapping_add(((((c.m.r32((l_body_2).wrapping_add(88)) as i32)) as u32)).wrapping_mul(32)); let a_ = (l_body_2).wrapping_add(80); c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjBody_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjBody_Warp(c: &mut Ctx, mut p_obj_1: u32, mut p_pos_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjBody_Warp", &PROBE_N, &[(p_obj_1) as u32, (p_pos_2) as u32], false); }
    let r_ = 'f: {
    if ((F::BtlObjBody_Exists(c, p_obj_1)) != 0) {
        F::Vec4_Copy(c, (((p_obj_1).wrapping_add(3920)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(32)), p_pos_2);
        F::Vec4_Copy(c, (((p_obj_1).wrapping_add(3920)).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(32)), p_pos_2);
        F::BtlObjBody_Reset(c, p_obj_1);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjBody_Warp", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjHit_BuildVolumes(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjHit_BuildVolumes", &PROBE_N, &[(p_obj_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(128) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 128);
    let mut l_hit_2: u32 = 0;
    let mut l_work_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_part_5: u32 = 0;
    let r_ = 'f: {
        l_hit_2 = (p_obj_1).wrapping_add(3216);
        l_work_3 = c.m.r32((p_obj_1).wrapping_add(5728));
        { let _ = F::BtlObjBody_Update(c, p_obj_1, 1i32); };
        if (((((l_work_3) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        { let v_ = 0i32; let a_ = (l_work_3).wrapping_add(106672); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (l_work_3).wrapping_add(106676); c.m.w32(a_, (v_) as u32); };
        { let r_ = ((16777215i32) as u32); let a_ = (l_work_3).wrapping_add(98336); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
        F::StgAabb_SetEmpty(c, (l_work_3).wrapping_add(98432));
        if ((((((((c.m.r8(((p_obj_1).wrapping_add(3216)).wrapping_add(29)) as i8)) as i32)) < (0i32)) as i32)) != 0) {
            break 'f;
        }
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < (19i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((((1i32).wrapping_shl((l_i_4) as u32)) as u32)) & (c.m.r32((l_hit_2).wrapping_add(12))))) != 0) {
                        l_part_5 = F::BtlObj_GetNodeByCode(c, p_obj_1, ((((l_i_4) as i64 as u64)) as u32));
                        if ((((((c.m.r32((l_hit_2).wrapping_add(32)) as i32)) == (1i32)) as i32)) != 0) {
                            { let v_ = F32(c.m.r32(l_hit_2)); let a_ = fp.wrapping_add(24); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(c.m.r32((l_hit_2).wrapping_add(4))); let a_ = (fp.wrapping_add(24)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(c.m.r32((l_hit_2).wrapping_add(8))); let a_ = (fp.wrapping_add(24)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(24)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                            F::Mtx_MulVec4(c, fp.wrapping_add(40), (l_part_5).wrapping_add(16), fp.wrapping_add(24));
                            { let a0_ = ((l_work_3).wrapping_add(98464)).wrapping_add(((((c.m.r32((l_work_3).wrapping_add(106672)) as i32)) as u32)).wrapping_mul(32)); let a2_ = F32(c.m.r32((l_hit_2).wrapping_add(16))); F::ColSphere_Set(c, a0_, fp.wrapping_add(40), a2_) };
                            { let a1_ = ((l_work_3).wrapping_add(98464)).wrapping_add(((((c.m.r32((l_work_3).wrapping_add(106672)) as i32)) as u32)).wrapping_mul(32)); F::ColBounds_OfSphere(c, fp, a1_) };
                            { let a_ = (l_work_3).wrapping_add(106672); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            F::ColBox_AddPointKeepW(c, (l_work_3).wrapping_add(98432), fp);
                            F::ColBox_AddPointKeepW(c, (l_work_3).wrapping_add(98432), (fp).wrapping_add(12));
                        } else {
                            { let v_ = F32(c.m.r32(l_hit_2)); let a_ = fp.wrapping_add(24); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(c.m.r32((l_hit_2).wrapping_add(4))); let a_ = (fp.wrapping_add(24)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(c.m.r32((l_hit_2).wrapping_add(8))); let a_ = (fp.wrapping_add(24)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(24)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                            F::Mtx_MulVec4(c, fp.wrapping_add(40), (l_part_5).wrapping_add(16), fp.wrapping_add(24));
                            F::Mtx_Copy(c, fp.wrapping_add(56), (l_part_5).wrapping_add(16));
                            F::Vec4_Copy(c, (fp.wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), fp.wrapping_add(40));
                            { let a0_ = ((l_work_3).wrapping_add(99072)).wrapping_add(((((c.m.r32((l_work_3).wrapping_add(106676)) as i32)) as u32)).wrapping_mul(400)); let a2_ = F32(c.m.r32((l_hit_2).wrapping_add(16))); let a3_ = F32(c.m.r32((l_hit_2).wrapping_add(20))); let a4_ = F32(c.m.r32((l_hit_2).wrapping_add(24))); F::ColObb_Init(c, a0_, fp.wrapping_add(40), a2_, a3_, a4_) };
                            { let a0_ = ((l_work_3).wrapping_add(99072)).wrapping_add(((((c.m.r32((l_work_3).wrapping_add(106676)) as i32)) as u32)).wrapping_mul(400)); F::ColObb_Update(c, a0_, fp.wrapping_add(56)) };
                            { let a1_ = ((l_work_3).wrapping_add(99072)).wrapping_add(((((c.m.r32((l_work_3).wrapping_add(106676)) as i32)) as u32)).wrapping_mul(400)); F::ColBounds_OfObb(c, fp, a1_) };
                            { let a_ = (l_work_3).wrapping_add(106676); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            F::ColBox_AddPointKeepW(c, (l_work_3).wrapping_add(98432), fp);
                            F::ColBox_AddPointKeepW(c, (l_work_3).wrapping_add(98432), (fp).wrapping_add(12));
                        }
                    }
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlObjHit_BuildVolumes", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjPose_GetNodeMtx(c: &mut Ctx, mut p_obj_1: u32, mut p_out_2: u32, mut p_part_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjPose_GetNodeMtx", &PROBE_N, &[(p_obj_1) as u32, (p_out_2) as u32, (p_part_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_look_4: u32 = 0;
    let r_ = 'f: {
        l_look_4 = (p_obj_1).wrapping_add(3360);
        if ((((((c.m.r32(p_obj_1) as i32)) == (0i32)) as i32)) != 0) {
            's1: {
                let sv5: u32 = c.m.r32((p_part_3).wrapping_add(8));
                match sv5 {
                    2u32 => {
                        F::Quat_Mul(c, fp, l_look_4, (p_part_3).wrapping_add(160));
                        break 's1;
                    }
                    46u32 => {
                        { let a3_ = F32(c.m.r32((l_look_4).wrapping_add(48))); F::Quat_Slerp(c, fp, (p_part_3).wrapping_add(160), ((p_obj_1).wrapping_add(3360)).wrapping_add(16), a3_) };
                        break 's1;
                    }
                    47u32 => {
                        { let a3_ = F32(c.m.r32((l_look_4).wrapping_add(52))); F::Quat_Slerp(c, fp, (p_part_3).wrapping_add(160), ((p_obj_1).wrapping_add(3360)).wrapping_add(32), a3_) };
                        break 's1;
                    }
                    49u32 => {
                        if ((F::BtlObj_IsJawActive(c, p_obj_1)) != 0) {
                            { let _ = F::BtlObj_GetJawRot(c, p_obj_1, fp); };
                        } else {
                            F::Vec4_Copy(c, fp, (p_part_3).wrapping_add(160));
                        }
                        break 's1;
                    }
                    _ => {
                        F::Vec4_Copy(c, fp, (p_part_3).wrapping_add(160));
                        break 's1;
                    }
                }
            }
            F::Quat_ToMtx(c, p_out_2, fp);
        } else {
            F::Vec4_Copy(c, fp, (p_part_3).wrapping_add(160));
            F::Quat_ToMtx(c, p_out_2, fp);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlObjPose_GetNodeMtx", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjXf_Reset(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjXf_Reset", &PROBE_N, &[(p_obj_1) as u32], false); }
    let mut l_xf_2: u32 = 0;
    let r_ = 'f: {
    l_xf_2 = (p_obj_1).wrapping_add(2384);
    { let v_ = 0i32; let a_ = ((p_obj_1).wrapping_add(2384)).wrapping_add(224); c.m.w32(a_, (v_) as u32); };
    F::Vec4_Copy(c, l_xf_2, sym::D_002EC2C0);
    F::Vec4_Copy(c, ((p_obj_1).wrapping_add(2384)).wrapping_add(16), sym::D_002EC2C0);
    { let v_ = F32(0x3f800000); let a_ = (l_xf_2).wrapping_add(208); c.m.w32(a_, (v_).0); };
    F::BtlObjXf_Update(c, p_obj_1);
    F::Vec4_Copy(c, ((p_obj_1).wrapping_add(2384)).wrapping_add(32), l_xf_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjXf_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjXf_Update(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjXf_Update", &PROBE_N, &[(p_obj_1) as u32], false); }
    let mut l_rot_2: u32 = 0;
    let mut l_xf_3: u32 = 0;
    let mut l_mtxRot_4: u32 = 0;
    let mut l_mtxPos_5: u32 = 0;
    let mut l_scale_6: F32 = F32(0);
    let mut l_mtx_7: u32 = 0;
    let mut l_changed_8: i32 = 0;
    let r_ = 'f: {
    l_rot_2 = ((p_obj_1).wrapping_add(2384)).wrapping_add(16);
    l_xf_3 = (p_obj_1).wrapping_add(2384);
    l_mtxRot_4 = ((p_obj_1).wrapping_add(2384)).wrapping_add(64);
    l_mtxPos_5 = ((p_obj_1).wrapping_add(2384)).wrapping_add(48);
    l_scale_6 = F32(c.m.r32((l_xf_3).wrapping_add(208)));
    l_changed_8 = (((F::memcmp(c, l_rot_2, l_mtxRot_4, 16u32)) != (0i32)) as i32);
    { let v_ = F32(0x3f800000); let a_ = ((p_obj_1).wrapping_add(2384)).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (((p_obj_1).wrapping_add(2384)).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
    F::Vec4_Copy(c, l_mtxPos_5, l_xf_3);
    F::Vec4_Copy(c, l_mtxRot_4, l_rot_2);
    l_mtx_7 = ((p_obj_1).wrapping_add(2384)).wrapping_add(80);
    { let r_ = F32(0x40490fda); let a_ = ((l_xf_3).wrapping_add(64)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    if (((rt::flt(F32(0x40490fda), F32(c.m.r32(((l_xf_3).wrapping_add(64)).wrapping_add(4)))) as i32)) != 0) {
        { let r_ = F32(0x40c90fda); let a_ = ((l_xf_3).wrapping_add(64)).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
    F::Vu0Cur_LoadIdentity(c);
    F::Vu0Cur_ScaleDiagUniform(c, l_scale_6);
    F::Vu0Cur_RotateZXY(c, l_mtxRot_4);
    F::Vu0Cur_Translate(c, l_mtxPos_5);
    F::Vu0Cur_StoreMtx(c, l_mtx_7);
    F::Mtx_InverseRT(c, ((p_obj_1).wrapping_add(2384)).wrapping_add(144), l_mtx_7);
    { let v_ = (!(l_changed_8)); let a_ = ((p_obj_1).wrapping_add(2384)).wrapping_add(224); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjXf_Update", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjXf_SetMtx(c: &mut Ctx, mut p_obj_1: u32, mut p_m_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjXf_SetMtx", &PROBE_N, &[(p_obj_1) as u32, (p_m_2) as u32], false); }
    let mut l_dst_3: u32 = 0;
    let r_ = 'f: {
    l_dst_3 = ((p_obj_1).wrapping_add(2384)).wrapping_add(80);
    F::Mtx_Copy(c, l_dst_3, p_m_2);
    F::Mtx_InverseRT(c, ((p_obj_1).wrapping_add(2384)).wrapping_add(144), l_dst_3);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjXf_SetMtx", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjPose_CalcMatrices(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjPose_CalcMatrices", &PROBE_N, &[(p_obj_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_xf_2: u32 = 0;
    let mut l_bone_3: u32 = 0;
    let mut l_part_4: u32 = 0;
    let mut l_done_5: i32 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
        'fl1: loop {
            match bb_ {
                0 => {
                    l_xf_2 = (p_obj_1).wrapping_add(2384);
                    l_bone_3 = c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(44));
                    F::Vu0Cur_ResetStack(c);
                    F::Vu0Cur_LoadMtx(c, ((p_obj_1).wrapping_add(2384)).wrapping_add(80));
                    bb_ = 1; continue 'fl1;
                }
                1 => {
                    if ((1i32) != 0) { bb_ = 2; continue 'fl1; } else { bb_ = 3; continue 'fl1; }
                }
                2 => {
                    l_part_4 = { let a1_ = ((c.m.r16((l_bone_3).wrapping_add(10))) as i32); F::BtlObj_GetNode(c, p_obj_1, a1_) };
                    F::Vu0Cur_Push(c);
                    F::Vu0Cur_StoreMtx(c, (l_part_4).wrapping_add(80));
                    F::BtlObjPose_GetNodeMtx(c, p_obj_1, fp, l_part_4);
                    F::Vec3_Copy(c, (fp).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (l_part_4).wrapping_add(144));
                    F::Mtx_Mul(c, (l_part_4).wrapping_add(16), (l_part_4).wrapping_add(80), fp);
                    F::Vu0Cur_LoadMtx(c, (l_part_4).wrapping_add(16));
                    if (((((((c.m.r16((l_bone_3).wrapping_add(4))) as i32)) != (0i32)) as i32)) != 0) { bb_ = 4; continue 'fl1; } else { bb_ = 5; continue 'fl1; }
                }
                3 => {
                    bb_ = 10; continue 'fl1;
                }
                4 => {
                    { let a0_ = ((c.m.r16((l_bone_3).wrapping_add(4))) as i32); F::Vu0Cur_PopN(c, a0_) };
                    bb_ = 5; continue 'fl1;
                }
                5 => {
                    if (((((((c.m.r16((l_bone_3).wrapping_add(6))) as i32)) != (0i32)) as i32)) != 0) { bb_ = 6; continue 'fl1; } else { bb_ = 8; continue 'fl1; }
                }
                6 => {
                    bb_ = 3; continue 'fl1;
                }
                7 => {
                    bb_ = 1; continue 'fl1;
                }
                8 => {
                    l_bone_3 = (l_bone_3).wrapping_add((((c.m.r32(l_bone_3) as i32)) as u32));
                    bb_ = 7; continue 'fl1;
                }
                9 => {
                    bb_ = 7; continue 'fl1;
                }
                10 => {
                    { let a1_ = ((F::BtlObj_GetNode(c, p_obj_1, 0i32)).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(16)); F::Vec4_Copy(c, (l_xf_2).wrapping_add(32), a1_) };
                    break 'fl1;
                }
                _ => unreachable!(),
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlObjPose_CalcMatrices", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_HasWork(c: &mut Ctx, mut p_obj_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_HasWork", &PROBE_N, &[(p_obj_1) as u32], false); }
    let r_ = 'f: {
    break 'f (((c.m.r32((p_obj_1).wrapping_add(5728))) != (((0i32) as u32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_HasWork", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjFlash_Get(c: &mut Ctx, mut p_obj_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjFlash_Get", &PROBE_N, &[(p_obj_1) as u32], false); }
    let r_ = 'f: {
    if ((F::BtlObj_HasWork(c, p_obj_1)) != 0) {
        break 'f (c.m.r32((p_obj_1).wrapping_add(5728))).wrapping_add(98304);
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjFlash_Get", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjFlash_Clear(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjFlash_Clear", &PROBE_N, &[(p_obj_1) as u32], false); }
    let r_ = 'f: {
    if ((F::BtlObj_HasWork(c, p_obj_1)) != 0) {
        { let _ = { let a0_ = F::BtlObjFlash_Get(c, p_obj_1); F::memset(c, a0_, 0i32, 32u32) }; };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjFlash_Clear", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjFlash_Reset(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjFlash_Reset", &PROBE_N, &[(p_obj_1) as u32], false); }
    let r_ = 'f: {
    F::BtlObjFlash_Clear(c, p_obj_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjFlash_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjFlash_Start(c: &mut Ctx, mut p_obj_1: u32, mut p_kind_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjFlash_Start", &PROBE_N, &[(p_obj_1) as u32, (p_kind_2) as u32], false); }
    let mut l_flash_3: u32 = 0;
    let r_ = 'f: {
    l_flash_3 = F::BtlObjFlash_Get(c, p_obj_1);
    if (((((l_flash_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    { let v_ = F32(0x3e4ccccc); let a_ = (l_flash_3).wrapping_add(16); c.m.w32(a_, (v_).0); };
    's1: {
        let sv4: i32 = p_kind_2;
        match sv4 {
            0i32 => {
                F::Vec4_Set(c, l_flash_3, F32(0x437f0000), F32(0x437f0000), F32(0x00000000), F32(0x42000000));
                break 's1;
            }
            1i32 => {
                F::Vec4_Set(c, l_flash_3, F32(0x437f0000), F32(0x437f0000), F32(0x437f0000), F32(0x42000000));
                break 's1;
            }
            2i32 => {
                F::Vec4_Set(c, l_flash_3, F32(0x437f0000), F32(0x437f0000), F32(0x437f0000), F32(0x42c00000));
                break 's1;
            }
            _ => {
                F::Vec4_Set(c, l_flash_3, F32(0x437f0000), F32(0x437f0000), F32(0x00000000), F32(0x42000000));
                break 's1;
            }
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjFlash_Start", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjFlash_Step(c: &mut Ctx, mut p_obj_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjFlash_Step", &PROBE_N, &[(p_obj_1) as u32], false); }
    let mut l_flash_2: u32 = 0;
    let r_ = 'f: {
    l_flash_2 = F::BtlObjFlash_Get(c, p_obj_1);
    if (((((l_flash_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    { let r_ = F32(0x3d088888); let a_ = (l_flash_2).wrapping_add(16); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    if (((rt::flt(F32(c.m.r32((l_flash_2).wrapping_add(16))), F32(0x00000000)) as i32)) != 0) {
        { let v_ = F32(0x00000000); let a_ = (l_flash_2).wrapping_add(16); c.m.w32(a_, (v_).0); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjFlash_Step", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjFlash_GetColor(c: &mut Ctx, mut p_obj_1: u32, mut p_color_2: u32, mut p_alpha_3: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjFlash_GetColor", &PROBE_N, &[(p_obj_1) as u32, (p_color_2) as u32, (p_alpha_3) as u32], false); }
    let mut l_flash_4: u32 = 0;
    let mut l_result_5: i32 = 0;
    let r_ = 'f: {
    l_flash_4 = F::BtlObjFlash_Get(c, p_obj_1);
    if (((((l_flash_4) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    { let v_ = rt::fmul(rt::fmul(rt::fdiv(F32(c.m.r32((l_flash_4).wrapping_add(16))), F32(0x3e4ccccc)), F32(c.m.r32((l_flash_4).wrapping_add(12)))), F32(0x3c000000)); let a_ = p_alpha_3; c.m.w32(a_, (v_).0); };
    F::Vec4_Copy(c, p_color_2, l_flash_4);
    l_result_5 = 1i32;
    if (((rt::fle(F32(c.m.r32((l_flash_4).wrapping_add(16))), F32(0x00000000)) as i32)) != 0) {
        l_result_5 = 0i32;
    }
    break 'f l_result_5;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjFlash_GetColor", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjFade_Step(c: &mut Ctx, mut p_obj_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjFade_Step", &PROBE_N, &[(p_obj_1) as u32], false); }
    let mut l_flash_2: u32 = 0;
    let r_ = 'f: {
    l_flash_2 = F::BtlObjFlash_Get(c, p_obj_1);
    if (((((l_flash_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((c.m.r32((p_obj_1).wrapping_add(2624))) & (((262144i32) as u32)))) != 0) {
        { let r_ = F32(0x3dcccccc); let a_ = (l_flash_2).wrapping_add(20); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if (((rt::flt(F32(0x3f800000), F32(c.m.r32((l_flash_2).wrapping_add(20)))) as i32)) != 0) {
            { let v_ = F32(0x3f800000); let a_ = (l_flash_2).wrapping_add(20); c.m.w32(a_, (v_).0); };
        }
    } else {
        { let r_ = F32(0x3dcccccc); let a_ = (l_flash_2).wrapping_add(20); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if (((rt::flt(F32(c.m.r32((l_flash_2).wrapping_add(20))), F32(0x00000000)) as i32)) != 0) {
            { let v_ = F32(0x00000000); let a_ = (l_flash_2).wrapping_add(20); c.m.w32(a_, (v_).0); };
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjFade_Step", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjFade_Get(c: &mut Ctx, mut p_obj_1: u32, mut p_out_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjFade_Get", &PROBE_N, &[(p_obj_1) as u32, (p_out_2) as u32], false); }
    let mut l_flash_3: u32 = 0;
    let r_ = 'f: {
    l_flash_3 = F::BtlObjFlash_Get(c, p_obj_1);
    if (((((l_flash_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    { let v_ = F32(c.m.r32((l_flash_3).wrapping_add(20))); let a_ = p_out_2; c.m.w32(a_, (v_).0); };
    if (((rt::fne(F32(c.m.r32((l_flash_3).wrapping_add(20))), F32(0x00000000)) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjFade_Get", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_SetUnkB30(c: &mut Ctx, mut p_obj_1: u32, mut p_value_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_SetUnkB30", &PROBE_N, &[(p_obj_1) as u32, (p_value_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((p_value_2) as u8); let a_ = (p_obj_1).wrapping_add(2864); c.m.w8(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_SetUnkB30", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjFace_Reset(c: &mut Ctx, mut p_face_1: u32, mut p_keep_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjFace_Reset", &PROBE_N, &[(p_face_1) as u32, (p_keep_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = F32(0x00000000); let a_ = (p_face_1).wrapping_add(48); c.m.w32(a_, (v_).0); };
    { let v_ = 0i32; let a_ = (p_face_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
    { let v_ = F32(0x00000000); let a_ = (p_face_1).wrapping_add(52); c.m.w32(a_, (v_).0); };
    { let v_ = 0i32; let a_ = (p_face_1).wrapping_add(56); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((0i32) as u32); let a_ = (p_face_1).wrapping_add(36); c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = (p_face_1).wrapping_add(96); c.m.w32(a_, (v_) as u32); };
    if (((((p_keep_2) == (0i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (p_face_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (p_face_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        { let v_ = F32(0x00000000); let a_ = (p_face_1).wrapping_add(68); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_face_1).wrapping_add(72); c.m.w32(a_, (v_).0); };
        { let v_ = ((0i32) as i16); let a_ = (p_face_1).wrapping_add(66); c.m.w16(a_, (v_) as u16); };
        { let v_ = ((0i32) as i16); let a_ = (p_face_1).wrapping_add(64); c.m.w16(a_, (v_) as u16); };
        { let v_ = 0i32; let a_ = (p_face_1).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
        F::Vec4_Set(c, (p_face_1).wrapping_add(80), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x3f800000));
    }
    { let v_ = F32(0x40000000); let a_ = (p_face_1).wrapping_add(44); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjFace_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjMdl_HasLipTrack(c: &mut Ctx, mut p_mdl_1: u32, mut p_index_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjMdl_HasLipTrack", &PROBE_N, &[(p_mdl_1) as u32, (p_index_2) as u32], false); }
    let r_ = 'f: {
    if (((((((((p_index_2) < (((100i32) as u32))) as i32)) != 0) && (((((c.m.r32(((p_mdl_1).wrapping_add(1856)).wrapping_add((p_index_2).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjMdl_HasLipTrack", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjMdl_CopyTexAlpha(c: &mut Ctx, mut p_mdl_1: u32, mut p_src_2: u32, mut p_dst_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjMdl_CopyTexAlpha", &PROBE_N, &[(p_mdl_1) as u32, (p_src_2) as u32, (p_dst_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_from_4: u32 = 0;
    let mut l_to_5: u32 = 0;
    let mut l_i_6: u32 = 0;
    let mut l_j_7: i32 = 0;
    let r_ = 'f: {
        { let v_ = p_dst_3; c.m.w32(fp, v_); }
        F::Res_RelocateOffsets(c, fp, p_dst_3, p_dst_3);
        l_from_4 = ({ let a_ = ((c.m.r32((p_src_2).wrapping_add(16))).wrapping_add(((((({ let a_ = (c.m.r32((p_mdl_1).wrapping_add(40))).wrapping_add(76); c.m.r8(a_) }) as i32)).wrapping_mul(64i32)) as u32))).wrapping_add(((60i32) as u32)); c.m.r32(a_) }).wrapping_add(((96i32) as u32));
        {
            l_i_6 = ((0i32) as u32);
            'l1: loop {
                if !(((((l_i_6) < ({ let a_ = c.m.r32(fp); c.m.r32(a_) })) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_to_5 = ({ let a_ = (({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((l_i_6).wrapping_mul(((64i32) as u32)))).wrapping_add(((60i32) as u32)); c.m.r32(a_) }).wrapping_add(((96i32) as u32));
                    {
                        l_j_7 = 0i32;
                        'l3: loop {
                            if !(((((l_j_7) < (16i32)) as i32)) != 0) { break 'l3; }
                            'c4: {
                                { let v_ = c.m.r8((l_from_4).wrapping_add(((((l_j_7).wrapping_mul(4i32)).wrapping_add(3i32)) as u32))); let a_ = (l_to_5).wrapping_add(((((l_j_7).wrapping_mul(4i32)).wrapping_add(3i32)) as u32)); c.m.w8(a_, v_); };
                            }
                            { let o_ = l_j_7; let n_ = (o_).wrapping_add(1); l_j_7 = n_; };
                        }
                    }
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlObjMdl_CopyTexAlpha", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjMdl_HasJaw(c: &mut Ctx, mut p_obj_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjMdl_HasJaw", &PROBE_N, &[(p_obj_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((((({ let a_ = (c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(40))).wrapping_add(12); c.m.r32(a_) }) & (((12582912i32) as u32)))) != (((0i32) as u32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjMdl_HasJaw", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjFace_PickJawKeys(c: &mut Ctx, mut p_obj_1: u32, mut p_face_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjFace_PickJawKeys", &PROBE_N, &[(p_obj_1) as u32, (p_face_2) as u32], false); }
    let r_ = 'f: {
    if ((((!((F::BtlObjMdl_HasJaw(c, p_obj_1)) != 0)) as i32)) != 0) {
        break 'f;
    }
    if (((({ let a_ = (c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(40))).wrapping_add(12); c.m.r32(a_) }) & (((4194304i32) as u32)))) != 0) {
        's1: {
            let sv3: i32 = rt::rem_i32(F::Port_Rand(c), 4i32);
            match sv3 {
                0i32 => {
                    { let v_ = sym::D_002C6D58; let a_ = (p_face_2).wrapping_add(32); c.m.w32(a_, v_); };
                    break 's1;
                }
                1i32 => {
                    { let v_ = sym::D_002C6D98; let a_ = (p_face_2).wrapping_add(32); c.m.w32(a_, v_); };
                    break 's1;
                }
                2i32 => {
                    { let v_ = sym::D_002C6DD8; let a_ = (p_face_2).wrapping_add(32); c.m.w32(a_, v_); };
                    break 's1;
                }
                _ => {
                    { let v_ = sym::D_002C6E18; let a_ = (p_face_2).wrapping_add(32); c.m.w32(a_, v_); };
                    break 's1;
                }
            }
        }
    } else {
        's2: {
            let sv4: i32 = rt::rem_i32(F::Port_Rand(c), 4i32);
            match sv4 {
                0i32 => {
                    { let v_ = sym::D_002C6C58; let a_ = (p_face_2).wrapping_add(32); c.m.w32(a_, v_); };
                    break 's2;
                }
                1i32 => {
                    { let v_ = sym::D_002C6C98; let a_ = (p_face_2).wrapping_add(32); c.m.w32(a_, v_); };
                    break 's2;
                }
                2i32 => {
                    { let v_ = sym::D_002C6CD8; let a_ = (p_face_2).wrapping_add(32); c.m.w32(a_, v_); };
                    break 's2;
                }
                _ => {
                    { let v_ = sym::D_002C6D18; let a_ = (p_face_2).wrapping_add(32); c.m.w32(a_, v_); };
                    break 's2;
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjFace_PickJawKeys", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjFace_Nop(c: &mut Ctx, mut p_face_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjFace_Nop", &PROBE_N, &[(p_face_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjFace_Nop", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjFace_Init(c: &mut Ctx, mut p_obj_1: u32, mut p_face_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjFace_Init", &PROBE_N, &[(p_obj_1) as u32, (p_face_2) as u32], false); }
    let mut l_mdl_3: u32 = 0;
    let r_ = 'f: {
    l_mdl_3 = (p_obj_1).wrapping_add(24);
    { let a1_ = c.m.r32((l_mdl_3).wrapping_add(48)); let a2_ = c.m.r32((l_mdl_3).wrapping_add(52)); F::BtlObjMdl_CopyTexAlpha(c, l_mdl_3, a1_, a2_) };
    F::BtlObjFace_Nop(c, p_face_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjFace_Init", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjFace_StepBlink(c: &mut Ctx, mut p_obj_1: u32, mut p_face_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjFace_StepBlink", &PROBE_N, &[(p_obj_1) as u32, (p_face_2) as u32], false); }
    let mut l_mdl_3: u32 = 0;
    let mut l_n_4: u32 = 0;
    let r_ = 'f: {
    l_mdl_3 = (p_obj_1).wrapping_add(24);
    if ((((c.m.r32((p_obj_1).wrapping_add(2624))) & (((67108864i32) as u32)))) != 0) {
        l_n_4 = ((((c.m.r32((p_face_2).wrapping_add(20)) as i32)).wrapping_sub(1i32)) as u32);
        if (((((((((l_n_4) < ({ let a_ = c.m.r32((l_mdl_3).wrapping_add(52)); c.m.r32(a_) })) as i32)) != 0) && (((((((l_n_4) as i32)) >= (0i32)) as i32)) != 0)) as i32)) != 0) {
            { let v_ = (c.m.r32((p_face_2).wrapping_add(20)) as i32); let a_ = (p_face_2).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 0i32; let a_ = (p_face_2).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
        }
    } else {
        if ((((c.m.r32((p_obj_1).wrapping_add(2624))) & (((134217728i32) as u32)))) != 0) {
            { let v_ = 1i32; let a_ = (p_face_2).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
        } else {
            if ((((({ let a_ = (p_face_2).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); o_ }) < (0i32)) as i32)) != 0) {
                { let v_ = (rt::rem_i32(F::Port_Rand(c), 3i32)).wrapping_add(3i32); let a_ = (p_face_2).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                { let v_ = (rt::rem_i32(F::Port_Rand(c), 90i32)).wrapping_add(90i32); let a_ = (p_face_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            }
            if ((((((c.m.r32((p_face_2).wrapping_add(12)) as i32)) != (0i32)) as i32)) != 0) {
                if ((((({ let a_ = (p_face_2).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); o_ }) < (0i32)) as i32)) != 0) {
                    { let v_ = 0i32; let a_ = (p_face_2).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (p_face_2).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let v_ = 1i32; let a_ = (p_face_2).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                }
            } else {
                { let v_ = 0i32; let a_ = (p_face_2).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjFace_StepBlink", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjFace_CanStart(c: &mut Ctx, mut p_face_1: u32, mut p_state_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjFace_CanStart", &PROBE_N, &[(p_face_1) as u32, (p_state_2) as u32], false); }
    let r_ = 'f: {
    if (((((((((c.m.r32((p_face_1).wrapping_add(24)) as i32)).wrapping_sub(9i32)) as u32)) < (((4i32) as u32))) as i32)) != 0) {
        if ((((((((((((p_state_2).wrapping_sub(9i32)) as u32)) < (((4i32) as u32))) as i32)) != 0) || (((((p_state_2) == (0i32)) as i32)) != 0)) as i32)) != 0) {
            break 'f 1i32;
        }
        break 'f 0i32;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjFace_CanStart", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjFace_Step(c: &mut Ctx, mut p_obj_1: u32, mut p_face_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjFace_Step", &PROBE_N, &[(p_obj_1) as u32, (p_face_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let mut l_t_4: F32 = F32(0);
    let r_ = 'f: {
    's1: {
        let sv5: i32 = (c.m.r32((p_face_2).wrapping_add(24)) as i32);
        match sv5 {
            0i32 => {
                F::BtlObjFace_Reset(c, p_face_2, 0i32);
                break 'f;
            }
            3i32 => {
                { let r_ = F32(c.m.r32((p_face_2).wrapping_add(44))); let a_ = (p_face_2).wrapping_add(52); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::fle(F32(c.m.r32((p_face_2).wrapping_add(48))), F32(c.m.r32((p_face_2).wrapping_add(52)))) as i32)) != 0) {
                    if ((((F::Port_Rand(c)) & (1i32))) != 0) {
                        { let v_ = 7i32; let a_ = (p_face_2).wrapping_add(56); c.m.w32(a_, (v_) as u32); };
                        { let v_ = sym::D_002C6C20; let a_ = (p_face_2).wrapping_add(36); c.m.w32(a_, v_); };
                    } else {
                        { let v_ = 5i32; let a_ = (p_face_2).wrapping_add(56); c.m.w32(a_, (v_) as u32); };
                        { let v_ = sym::D_002C6C40; let a_ = (p_face_2).wrapping_add(36); c.m.w32(a_, v_); };
                    }
                    { let v_ = F32(0x00000000); let a_ = (p_face_2).wrapping_add(52); c.m.w32(a_, (v_).0); };
                    { let v_ = (({ let a_ = (c.m.r32((p_face_2).wrapping_add(36))).wrapping_add(2); (c.m.r16(a_) as i16) }) as i32); let a_ = (p_face_2).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            4i32 => {
                { let r_ = F32(c.m.r32((p_face_2).wrapping_add(44))); let a_ = (p_face_2).wrapping_add(52); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::fle(F32(c.m.r32((p_face_2).wrapping_add(48))), F32(c.m.r32((p_face_2).wrapping_add(52)))) as i32)) != 0) {
                    F::BtlObjFace_Reset(c, p_face_2, 0i32);
                }
                break 's1;
            }
            13i32 => {
                { let r_ = F32(c.m.r32((p_face_2).wrapping_add(44))); let a_ = (p_face_2).wrapping_add(52); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::fle(F32(c.m.r32((p_face_2).wrapping_add(48))), F32(c.m.r32((p_face_2).wrapping_add(52)))) as i32)) != 0) {
                    if ((((({ let a_ = (p_face_2).wrapping_add(96); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); o_ }) < (3i32)) as i32)) != 0) {
                        if ((((F::Port_Rand(c)) & (1i32))) != 0) {
                            { let v_ = 7i32; let a_ = (p_face_2).wrapping_add(56); c.m.w32(a_, (v_) as u32); };
                            { let v_ = sym::D_002C6C20; let a_ = (p_face_2).wrapping_add(36); c.m.w32(a_, v_); };
                        } else {
                            { let v_ = 5i32; let a_ = (p_face_2).wrapping_add(56); c.m.w32(a_, (v_) as u32); };
                            { let v_ = sym::D_002C6C40; let a_ = (p_face_2).wrapping_add(36); c.m.w32(a_, v_); };
                        }
                        { let v_ = F32(0x00000000); let a_ = (p_face_2).wrapping_add(52); c.m.w32(a_, (v_).0); };
                        { let v_ = (({ let a_ = (c.m.r32((p_face_2).wrapping_add(36))).wrapping_add(2); (c.m.r16(a_) as i16) }) as i32); let a_ = (p_face_2).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                    } else {
                        if (((rt::fge(F32(c.m.r32((p_face_2).wrapping_add(52))), F32(c.m.r32((p_face_2).wrapping_add(48)))) as i32)) != 0) {
                            F::BtlObjFace_Reset(c, p_face_2, 0i32);
                        }
                    }
                }
                break 's1;
            }
            10i32 => {
                { let v_ = F32(0x3ebba864); let a_ = (p_face_2).wrapping_add(80); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = ((p_face_2).wrapping_add(80)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = ((p_face_2).wrapping_add(80)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = ((p_face_2).wrapping_add(80)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                break 'f;
            }
            1i32 | 2i32 | 14i32 => {
                break 's1;
            }
            _ => {
                break 'f;
            }
        }
    }
    if ((F::BtlObjMdl_HasJaw(c, p_obj_1)) != 0) {
        {
            l_i_3 = 1i32;
            'l2: loop {
                if !(((((l_i_3) < ((c.m.r32((p_face_2).wrapping_add(56)) as i32))) as i32)) != 0) { break 'l2; }
                'c3: {
                    if (((rt::feq(rt::u2f({ let a_ = ((c.m.r32((p_face_2).wrapping_add(32))).wrapping_add(((((l_i_3).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.r32(a_) }), F32(c.m.r32((p_face_2).wrapping_add(52)))) as i32)) != 0) {
                        { let a1_ = { let a_ = (c.m.r32((p_face_2).wrapping_add(32))).wrapping_add(((((l_i_3).wrapping_sub(1i32)) as u32)).wrapping_mul(16)); F32(c.m.r32(a_)) }; let a2_ = { let a_ = ((c.m.r32((p_face_2).wrapping_add(32))).wrapping_add(((((l_i_3).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(4); F32(c.m.r32(a_)) }; let a3_ = { let a_ = ((c.m.r32((p_face_2).wrapping_add(32))).wrapping_add(((((l_i_3).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(8); F32(c.m.r32(a_)) }; F::Vec4_Set(c, (p_face_2).wrapping_add(80), a1_, a2_, a3_, F32(0x3f800000)) };
                        break 'l2;
                    }
                    if (((((((rt::flt(rt::u2f({ let a_ = ((c.m.r32((p_face_2).wrapping_add(32))).wrapping_add(((((l_i_3).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.r32(a_) }), F32(c.m.r32((p_face_2).wrapping_add(52)))) as i32)) != 0) && (((rt::flt(F32(c.m.r32((p_face_2).wrapping_add(52))), rt::u2f({ let a_ = ((c.m.r32((p_face_2).wrapping_add(32))).wrapping_add((((l_i_3) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.r32(a_) })) as i32)) != 0)) as i32)) != 0) {
                        l_t_4 = rt::fdiv(rt::fsub(F32(c.m.r32((p_face_2).wrapping_add(52))), rt::u2f({ let a_ = ((c.m.r32((p_face_2).wrapping_add(32))).wrapping_add(((((l_i_3).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.r32(a_) })), rt::fsub(rt::u2f({ let a_ = ((c.m.r32((p_face_2).wrapping_add(32))).wrapping_add((((l_i_3) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.r32(a_) }), rt::u2f({ let a_ = ((c.m.r32((p_face_2).wrapping_add(32))).wrapping_add(((((l_i_3).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.r32(a_) })));
                        { let v_ = rt::fadd(rt::fmul(rt::fsub({ let a_ = (c.m.r32((p_face_2).wrapping_add(32))).wrapping_add((((l_i_3) as u32)).wrapping_mul(16)); F32(c.m.r32(a_)) }, { let a_ = (c.m.r32((p_face_2).wrapping_add(32))).wrapping_add(((((l_i_3).wrapping_sub(1i32)) as u32)).wrapping_mul(16)); F32(c.m.r32(a_)) }), l_t_4), { let a_ = (c.m.r32((p_face_2).wrapping_add(32))).wrapping_add(((((l_i_3).wrapping_sub(1i32)) as u32)).wrapping_mul(16)); F32(c.m.r32(a_)) }); let a_ = (p_face_2).wrapping_add(80); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(rt::fmul(rt::fsub({ let a_ = ((c.m.r32((p_face_2).wrapping_add(32))).wrapping_add((((l_i_3) as u32)).wrapping_mul(16))).wrapping_add(4); F32(c.m.r32(a_)) }, { let a_ = ((c.m.r32((p_face_2).wrapping_add(32))).wrapping_add(((((l_i_3).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(4); F32(c.m.r32(a_)) }), l_t_4), { let a_ = ((c.m.r32((p_face_2).wrapping_add(32))).wrapping_add(((((l_i_3).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(4); F32(c.m.r32(a_)) }); let a_ = ((p_face_2).wrapping_add(80)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(rt::fmul(rt::fsub({ let a_ = ((c.m.r32((p_face_2).wrapping_add(32))).wrapping_add((((l_i_3) as u32)).wrapping_mul(16))).wrapping_add(8); F32(c.m.r32(a_)) }, { let a_ = ((c.m.r32((p_face_2).wrapping_add(32))).wrapping_add(((((l_i_3).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(8); F32(c.m.r32(a_)) }), l_t_4), { let a_ = ((c.m.r32((p_face_2).wrapping_add(32))).wrapping_add(((((l_i_3).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(8); F32(c.m.r32(a_)) }); let a_ = ((p_face_2).wrapping_add(80)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x3f800000); let a_ = ((p_face_2).wrapping_add(80)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        break 'l2;
                    }
                    if (((rt::feq(rt::u2f({ let a_ = ((c.m.r32((p_face_2).wrapping_add(32))).wrapping_add((((l_i_3) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.r32(a_) }), F32(c.m.r32((p_face_2).wrapping_add(52)))) as i32)) != 0) {
                        { let a1_ = { let a_ = (c.m.r32((p_face_2).wrapping_add(32))).wrapping_add((((l_i_3) as u32)).wrapping_mul(16)); F32(c.m.r32(a_)) }; let a2_ = { let a_ = ((c.m.r32((p_face_2).wrapping_add(32))).wrapping_add((((l_i_3) as u32)).wrapping_mul(16))).wrapping_add(4); F32(c.m.r32(a_)) }; let a3_ = { let a_ = ((c.m.r32((p_face_2).wrapping_add(32))).wrapping_add((((l_i_3) as u32)).wrapping_mul(16))).wrapping_add(8); F32(c.m.r32(a_)) }; F::Vec4_Set(c, (p_face_2).wrapping_add(80), a1_, a2_, a3_, F32(0x3f800000)) };
                        break 'l2;
                    }
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        { let v_ = 0i32; let a_ = (p_face_2).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
    } else {
        {
            l_i_3 = 1i32;
            'l4: loop {
                if !(((((l_i_3) < ((c.m.r32((p_face_2).wrapping_add(56)) as i32))) as i32)) != 0) { break 'l4; }
                'c5: {
                    if (((rt::feq(rt::i2f(({ let a_ = (c.m.r32((p_face_2).wrapping_add(36))).wrapping_add(((((l_i_3).wrapping_sub(1i32)) as u32)).wrapping_mul(4)); (c.m.r16(a_) as i16) }) as i32), F32(c.m.r32((p_face_2).wrapping_add(52)))) as i32)) != 0) {
                        { let v_ = (({ let a_ = ((c.m.r32((p_face_2).wrapping_add(36))).wrapping_add(((((l_i_3).wrapping_sub(1i32)) as u32)).wrapping_mul(4))).wrapping_add(2); (c.m.r16(a_) as i16) }) as i32); let a_ = (p_face_2).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                        break 'f;
                    }
                    if (((((((rt::fle(rt::i2f(({ let a_ = (c.m.r32((p_face_2).wrapping_add(36))).wrapping_add(((((l_i_3).wrapping_sub(1i32)) as u32)).wrapping_mul(4)); (c.m.r16(a_) as i16) }) as i32), F32(c.m.r32((p_face_2).wrapping_add(52)))) as i32)) != 0) && (((rt::fle(F32(c.m.r32((p_face_2).wrapping_add(52))), rt::i2f(({ let a_ = (c.m.r32((p_face_2).wrapping_add(36))).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); (c.m.r16(a_) as i16) }) as i32)) as i32)) != 0)) as i32)) != 0) {
                        { let v_ = (({ let a_ = ((c.m.r32((p_face_2).wrapping_add(36))).wrapping_add(((((l_i_3).wrapping_sub(1i32)) as u32)).wrapping_mul(4))).wrapping_add(2); (c.m.r16(a_) as i16) }) as i32); let a_ = (p_face_2).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                        break 'f;
                    }
                    if (((rt::feq(rt::i2f(({ let a_ = (c.m.r32((p_face_2).wrapping_add(36))).wrapping_add(((((l_i_3).wrapping_sub(1i32)) as u32)).wrapping_mul(4)); (c.m.r16(a_) as i16) }) as i32), F32(c.m.r32((p_face_2).wrapping_add(52)))) as i32)) != 0) {
                        { let v_ = (({ let a_ = ((c.m.r32((p_face_2).wrapping_add(36))).wrapping_add((((l_i_3) as u32)).wrapping_mul(4))).wrapping_add(2); (c.m.r16(a_) as i16) }) as i32); let a_ = (p_face_2).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                        break 'f;
                    }
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjFace_Step", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjMdl_HasMouthSet(c: &mut Ctx, mut p_mdl_1: u32, mut p_n_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjMdl_HasMouthSet", &PROBE_N, &[(p_mdl_1) as u32, (p_n_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    l_i_3 = (p_n_2).wrapping_sub(1i32);
    if (((((l_i_3) < (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f (((c.m.r32(((p_mdl_1).wrapping_add(68)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjMdl_HasMouthSet", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjMdl_HasMouth(c: &mut Ctx, mut p_mdl_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjMdl_HasMouth", &PROBE_N, &[(p_mdl_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (8i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((c.m.r32(((p_mdl_1).wrapping_add(68)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
                    break 'f 1i32;
                }
                if (((((c.m.r32(((p_mdl_1).wrapping_add(100)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
                    break 'f 1i32;
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    break 'f ((((({ let a_ = (c.m.r32((p_mdl_1).wrapping_add(40))).wrapping_add(12); c.m.r32(a_) }) & (((12582912i32) as u32)))) != (((0i32) as u32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjMdl_HasMouth", &PROBE_N, &[], true); }
    r_
}

pub fn BObjFace_InitMouth(c: &mut Ctx, mut p_obj_1: u32, mut p_face_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BObjFace_InitMouth", &PROBE_N, &[(p_obj_1) as u32, (p_face_2) as u32], false); }
    let mut l_mdl_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    { let _ = D::call(c, fid::BtlObjFace_Reset, &[((p_face_2) as u64), ((0i32) as i64 as u64)]); };
    l_mdl_3 = (p_obj_1).wrapping_add(24);
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (8i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((c.m.r32((((p_obj_1).wrapping_add(24)).wrapping_add(100)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
                    { let a1_ = c.m.r32((l_mdl_3).wrapping_add(48)); let a2_ = c.m.r32((((p_obj_1).wrapping_add(24)).wrapping_add(100)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))); { let _ = D::call(c, fid::BtlObjMdl_CopyTexAlpha, &[((l_mdl_3) as u64), ((a1_) as u64), ((a2_) as u64)]); } };
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BObjFace_InitMouth", &PROBE_N, &[], true); }
    r_
}

pub fn BObjFace_StepLipData(c: &mut Ctx, mut p_obj_1: u32, mut p_face_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BObjFace_StepLipData", &PROBE_N, &[(p_obj_1) as u32, (p_face_2) as u32], false); }
    let mut l_keys_3: u32 = 0;
    let mut l_time_4: F32 = F32(0);
    let mut l_found_5: i32 = 0;
    let mut l_i_6: u16 = 0;
    let r_ = 'f: {
    l_keys_3 = c.m.r32((p_face_2).wrapping_add(40));
    if (((((l_keys_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    l_time_4 = F32(c.m.r32((p_face_2).wrapping_add(68)));
    if (((rt::fle(F32(0x00000000), l_time_4) as i32)) != 0) {
        l_found_5 = (1i32).wrapping_neg();
        {
            l_i_6 = c.m.r16((p_face_2).wrapping_add(64));
            'l1: loop {
                if !(((((((l_i_6) as i32)) < ((((c.m.r16((p_face_2).wrapping_add(66))) as i32)).wrapping_sub(1i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((((rt::fle(rt::i2f(((c.m.r16((l_keys_3).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)))) as i32)), l_time_4) as i32)) != 0) && (((rt::flt(l_time_4, rt::i2f(((c.m.r16((l_keys_3).wrapping_add(((((((l_i_6) as i32)).wrapping_add(1i32)) as u32)).wrapping_mul(4)))) as i32))) as i32)) != 0)) as i32)) != 0) {
                        l_found_5 = ((l_i_6) as i32);
                        { let v_ = l_i_6; let a_ = (p_face_2).wrapping_add(64); c.m.w16(a_, v_); };
                        break 'l1;
                    }
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
        if (((((((rt::flt(F32(c.m.r32((p_face_2).wrapping_add(72))), l_time_4) as i32)) != 0) || (((((l_found_5) == ((1i32).wrapping_neg())) as i32)) != 0)) as i32)) != 0) {
            F::BtlObj_SetSubState(c, p_obj_1, 0i32, 0i32);
            break 'f;
        }
        's3: {
            let sv7: i32 = ((c.m.r16(((l_keys_3).wrapping_add((((l_found_5) as u32)).wrapping_mul(4))).wrapping_add(2))) as i32);
            match sv7 {
                1i32 => {
                    if ((((((c.m.r32((p_face_2).wrapping_add(24)) as i32)) != (3i32)) as i32)) != 0) {
                        F::BtlObj_SetSubState(c, p_obj_1, 3i32, 0i32);
                    }
                    break 's3;
                }
                0i32 => {
                    F::BtlObj_SetSubState(c, p_obj_1, 1i32, 0i32);
                    break 's3;
                }
                _ => {
                    if ((((((c.m.r32((p_face_2).wrapping_add(24)) as i32)) != (3i32)) as i32)) != 0) {
                        F::BtlObj_SetSubState(c, p_obj_1, 3i32, 0i32);
                    }
                    break 's3;
                }
            }
        }
    }
    { let v_ = rt::fadd(F32(c.m.r32((p_face_2).wrapping_add(68))), F32(c.m.r32((p_face_2).wrapping_add(44)))); let a_ = (p_face_2).wrapping_add(68); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BObjFace_StepLipData", &PROBE_N, &[], true); }
    r_
}

pub fn BObjFace_UpdateMouth(c: &mut Ctx, mut p_obj_1: u32, mut p_face_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BObjFace_UpdateMouth", &PROBE_N, &[(p_obj_1) as u32, (p_face_2) as u32], false); }
    let r_ = 'f: {
    F::BObjFace_StepLipData(c, p_obj_1, p_face_2);
    { let _ = D::call(c, fid::BtlObjFace_Step, &[((p_obj_1) as u64), ((p_face_2) as u64)]); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BObjFace_UpdateMouth", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_UpdateFace(c: &mut Ctx, mut p_obj_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_UpdateFace", &PROBE_N, &[(p_obj_1) as u32], false); }
    let mut l_face_2: u32 = 0;
    let r_ = 'f: {
    l_face_2 = c.m.r32((p_obj_1).wrapping_add(5732));
    if (((((l_face_2) != (((0i32) as u32))) as i32)) != 0) {
        if (((((c.m.r32((l_face_2).wrapping_add(4)) as i32)) & (1i32))) != 0) {
            { let _ = D::call(c, fid::BtlObjFace_StepBlink, &[((p_obj_1) as u64), ((l_face_2) as u64)]); };
        }
        if (((((c.m.r32((l_face_2).wrapping_add(4)) as i32)) & (2i32))) != 0) {
            F::BObjFace_UpdateMouth(c, p_obj_1, l_face_2);
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_UpdateFace", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_CreateFace(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_CreateFace", &PROBE_N, &[(p_obj_1) as u32], false); }
    let mut l_flags_2: i32 = 0;
    let mut l_face_3: u32 = 0;
    let mut l_mdl_4: u32 = 0;
    let r_ = 'f: {
    l_mdl_4 = (p_obj_1).wrapping_add(24);
    l_flags_2 = 0i32;
    if (((((c.m.r32((l_mdl_4).wrapping_add(52))) != (((0i32) as u32))) as i32)) != 0) {
        { let r_ = 1i32; let n_ = ((l_flags_2) | (r_)); l_flags_2 = n_; };
    }
    if ((((D::call(c, fid::BtlObjMdl_HasMouth, &[((l_mdl_4) as u64)])) as i32)) != 0) {
        { let r_ = 2i32; let n_ = ((l_flags_2) | (r_)); l_flags_2 = n_; };
    }
    if (((((l_flags_2) != (0i32)) as i32)) != 0) {
        l_face_3 = F::BtlObjPool70_Alloc(c);
        { let _ = F::memset(c, l_face_3, 0i32, 112u32); };
        { let v_ = l_flags_2; let a_ = (l_face_3).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        if ((((l_flags_2) & (1i32))) != 0) {
            { let _ = D::call(c, fid::BtlObjFace_Init, &[((p_obj_1) as u64), ((l_face_3) as u64)]); };
        }
        if (((((c.m.r32((l_face_3).wrapping_add(4)) as i32)) & (2i32))) != 0) {
            F::BObjFace_InitMouth(c, p_obj_1, l_face_3);
        }
        { let v_ = l_face_3; let a_ = (p_obj_1).wrapping_add(5732); c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_CreateFace", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_FreeFace(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_FreeFace", &PROBE_N, &[(p_obj_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32((p_obj_1).wrapping_add(5732))) != (((0i32) as u32))) as i32)) != 0) {
        { let _ = { let a0_ = c.m.r32((p_obj_1).wrapping_add(5732)); F::BtlObjPool70_Free(c, a0_) }; };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_FreeFace", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_SetEyeFrame(c: &mut Ctx, mut p_obj_1: u32, mut p_frame_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_SetEyeFrame", &PROBE_N, &[(p_obj_1) as u32, (p_frame_2) as u32], false); }
    let mut l_face_3: u32 = 0;
    let mut l_mdl_4: u32 = 0;
    let r_ = 'f: {
    l_face_3 = c.m.r32((p_obj_1).wrapping_add(5732));
    l_mdl_4 = (p_obj_1).wrapping_add(24);
    if (((((((((l_face_3) != (((0i32) as u32))) as i32)) != 0) && (((((c.m.r32((l_face_3).wrapping_add(4)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        if (((((p_frame_2) != (((9i32) as u32))) as i32)) != 0) {
            { let r_ = ((67108864i32) as u32); let a_ = (p_obj_1).wrapping_add(2624); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
            if ((((!((((({ let a_ = c.m.r32((l_mdl_4).wrapping_add(52)); c.m.r32(a_) }) < (p_frame_2)) as i32)) != 0)) as i32)) != 0) {
                { let v_ = p_frame_2; let a_ = (l_face_3).wrapping_add(20); c.m.w32(a_, v_); };
            }
        } else {
            { let r_ = (((!(67108864i32))) as u32); let a_ = (p_obj_1).wrapping_add(2624); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_SetEyeFrame", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_SetSubState(c: &mut Ctx, mut p_obj_1: u32, mut p_mode_2: i32, mut p_arg_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_SetSubState", &PROBE_N, &[(p_obj_1) as u32, (p_mode_2) as u32, (p_arg_3) as u32], false); }
    let mut l_face_4: u32 = 0;
    let mut l_mdl_5: u32 = 0;
    let mut l_data_6: u32 = 0;
    let mut l_end_7: i16 = 0;
    let r_ = 'f: {
    l_face_4 = c.m.r32((p_obj_1).wrapping_add(5732));
    l_mdl_5 = (p_obj_1).wrapping_add(24);
    if (((((l_face_4) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if ((((!(((((c.m.r32((l_face_4).wrapping_add(4)) as i32)) & (2i32))) != 0)) as i32)) != 0) {
        break 'f;
    }
    if (((((((((p_arg_3) >= (0i32)) as i32)) != 0) && (((((p_mode_2) == (2i32)) as i32)) != 0)) as i32)) != 0) {
        if ((((!((((D::call(c, fid::BtlObjMdl_HasLipTrack, &[((l_mdl_5) as u64), ((p_arg_3) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) {
            break 'f;
        }
    }
    if ((((!((((D::call(c, fid::BtlObjFace_CanStart, &[((l_face_4) as u64), ((p_mode_2) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) {
        break 'f;
    }
    { let v_ = p_mode_2; let a_ = (l_face_4).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
    's1: {
        let sv8: i32 = p_mode_2;
        match sv8 {
            0i32 => {
                { let _ = D::call(c, fid::BtlObjFace_Reset, &[((l_face_4) as u64), ((0i32) as i64 as u64)]); };
                break 'f;
            }
            1i32 => {
                { let _ = D::call(c, fid::BtlObjFace_Reset, &[((l_face_4) as u64), ((1i32) as i64 as u64)]); };
                break 's1;
            }
            3i32 | 4i32 | 13i32 => {
                if ((((F::Port_Rand(c)) & (1i32))) != 0) {
                    { let v_ = 7i32; let a_ = (l_face_4).wrapping_add(56); c.m.w32(a_, (v_) as u32); };
                    { let v_ = sym::D_002C6C20; let a_ = (l_face_4).wrapping_add(36); c.m.w32(a_, v_); };
                } else {
                    { let v_ = 5i32; let a_ = (l_face_4).wrapping_add(56); c.m.w32(a_, (v_) as u32); };
                    { let v_ = sym::D_002C6C40; let a_ = (l_face_4).wrapping_add(36); c.m.w32(a_, v_); };
                }
                l_end_7 = { let a_ = (c.m.r32((l_face_4).wrapping_add(36))).wrapping_add(((((((c.m.r32((l_face_4).wrapping_add(56)) as i32)).wrapping_mul(2i32)).wrapping_sub(2i32)) as u32)).wrapping_mul(2)); (c.m.r16(a_) as i16) };
                { let v_ = F32(0x00000000); let a_ = (l_face_4).wrapping_add(52); c.m.w32(a_, (v_).0); };
                { let v_ = rt::i2f((l_end_7) as i32); let a_ = (l_face_4).wrapping_add(48); c.m.w32(a_, (v_).0); };
                { let v_ = 0i32; let a_ = (l_face_4).wrapping_add(96); c.m.w32(a_, (v_) as u32); };
                { let _ = D::call(c, fid::BtlObjFace_PickJawKeys, &[((p_obj_1) as u64), ((l_face_4) as u64)]); };
                break 's1;
            }
            2i32 => {
                l_data_6 = c.m.r32(((l_mdl_5).wrapping_add(1856)).wrapping_add((((p_arg_3) as u32)).wrapping_mul(4)));
                if (((((l_data_6) == (((0i32) as u32))) as i32)) != 0) {
                    break 's1;
                }
                { let v_ = ((0i32) as u16); let a_ = (l_face_4).wrapping_add(64); c.m.w16(a_, v_); };
                { let v_ = c.m.r16((l_data_6).wrapping_add(8)); let a_ = (l_face_4).wrapping_add(66); c.m.w16(a_, v_); };
                { let v_ = (l_data_6).wrapping_add(16); let a_ = (l_face_4).wrapping_add(40); c.m.w32(a_, v_); };
                { let v_ = p_arg_3; let a_ = (l_face_4).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
                { let v_ = F32(0x00000000); let a_ = (l_face_4).wrapping_add(68); c.m.w32(a_, (v_).0); };
                { let v_ = rt::i2f(({ let a_ = (c.m.r32((l_face_4).wrapping_add(40))).wrapping_add(((((((c.m.r16((l_face_4).wrapping_add(66))) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(4)); c.m.r16(a_) }) as i32); let a_ = (l_face_4).wrapping_add(72); c.m.w32(a_, (v_).0); };
                { let _ = D::call(c, fid::BtlObjFace_PickJawKeys, &[((p_obj_1) as u64), ((l_face_4) as u64)]); };
                break 's1;
            }
            14i32 => {
                { let v_ = ((0i32) as u16); let a_ = (l_face_4).wrapping_add(64); c.m.w16(a_, v_); };
                { let v_ = c.m.r16((((p_arg_3) as u32)).wrapping_add(8)); let a_ = (l_face_4).wrapping_add(66); c.m.w16(a_, v_); };
                { let v_ = (((p_arg_3) as u32)).wrapping_add(16); let a_ = (l_face_4).wrapping_add(40); c.m.w32(a_, v_); };
                { let v_ = p_arg_3; let a_ = (l_face_4).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
                { let v_ = F32(0x00000000); let a_ = (l_face_4).wrapping_add(68); c.m.w32(a_, (v_).0); };
                { let v_ = rt::i2f(({ let a_ = (c.m.r32((l_face_4).wrapping_add(40))).wrapping_add(((((((c.m.r16((l_face_4).wrapping_add(66))) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(4)); c.m.r16(a_) }) as i32); let a_ = (l_face_4).wrapping_add(72); c.m.w32(a_, (v_).0); };
                { let _ = D::call(c, fid::BtlObjFace_PickJawKeys, &[((p_obj_1) as u64), ((l_face_4) as u64)]); };
                break 's1;
            }
            5i32 => {
                { let v_ = 1i32; let a_ = (l_face_4).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            6i32 => {
                { let v_ = 2i32; let a_ = (l_face_4).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            7i32 => {
                { let v_ = 3i32; let a_ = (l_face_4).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            8i32 => {
                { let v_ = 4i32; let a_ = (l_face_4).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            9i32 => {
                { let _ = D::call(c, fid::BtlObjFace_Reset, &[((l_face_4) as u64), ((0i32) as i64 as u64)]); };
                { let v_ = p_mode_2; let a_ = (l_face_4).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let v_ = 5i32; let a_ = (l_face_4).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            10i32 => {
                { let _ = D::call(c, fid::BtlObjFace_Reset, &[((l_face_4) as u64), ((0i32) as i64 as u64)]); };
                { let v_ = p_mode_2; let a_ = (l_face_4).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let v_ = 6i32; let a_ = (l_face_4).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                { let v_ = F32(0x3ebba864); let a_ = (l_face_4).wrapping_add(80); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = ((l_face_4).wrapping_add(80)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = ((l_face_4).wrapping_add(80)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = ((l_face_4).wrapping_add(80)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                break 's1;
            }
            11i32 => {
                { let _ = D::call(c, fid::BtlObjFace_Reset, &[((l_face_4) as u64), ((0i32) as i64 as u64)]); };
                { let v_ = p_mode_2; let a_ = (l_face_4).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let v_ = 7i32; let a_ = (l_face_4).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            12i32 => {
                { let _ = D::call(c, fid::BtlObjFace_Reset, &[((l_face_4) as u64), ((0i32) as i64 as u64)]); };
                { let v_ = p_mode_2; let a_ = (l_face_4).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let v_ = 8i32; let a_ = (l_face_4).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_SetSubState", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_GetSubState(c: &mut Ctx, mut p_obj_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_GetSubState", &PROBE_N, &[(p_obj_1) as u32], false); }
    let mut l_face_2: u32 = 0;
    let r_ = 'f: {
    l_face_2 = c.m.r32((p_obj_1).wrapping_add(5732));
    if (((((l_face_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f (1i32).wrapping_neg();
    }
    if (({ let a1_ = (c.m.r32((l_face_2).wrapping_add(28)) as i32); ((D::call(c, fid::BtlObjMdl_HasMouthSet, &[(((p_obj_1).wrapping_add(24)) as u64), ((a1_) as i64 as u64)])) as i32) }) != 0) {
        break 'f ((c.m.r32((l_face_2).wrapping_add(28)) as i32)).wrapping_sub(1i32);
    }
    break 'f (1i32).wrapping_neg();
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_GetSubState", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_GetFaceTexture(c: &mut Ctx, mut p_obj_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_GetFaceTexture", &PROBE_N, &[(p_obj_1) as u32], false); }
    let mut l_face_2: u32 = 0;
    let mut l_set_3: u32 = 0;
    let mut l_shape_4: i32 = 0;
    let r_ = 'f: {
    l_face_2 = c.m.r32((p_obj_1).wrapping_add(5732));
    if (((((l_face_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    l_shape_4 = F::BtlObj_GetSubState(c, p_obj_1);
    if (((((l_shape_4) >= (4i32)) as i32)) != 0) {
        l_set_3 = c.m.r32((((p_obj_1).wrapping_add(24)).wrapping_add(100)).wrapping_add((((l_shape_4) as u32)).wrapping_mul(4)));
        if (((((l_set_3) != (((0i32) as u32))) as i32)) != 0) {
            break 'f c.m.r32((l_set_3).wrapping_add(16));
        }
    } else {
        if ((((((c.m.r32((l_face_2).wrapping_add(16)) as i32)) != (0i32)) as i32)) != 0) {
            break 'f (({ let a_ = (c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(52))).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((((c.m.r32((l_face_2).wrapping_add(16)) as i32)) as u32)).wrapping_mul(64))).wrapping_sub((((1i32) as u32)).wrapping_mul(64));
        }
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_GetFaceTexture", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_HasEyes(c: &mut Ctx, mut p_obj_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_HasEyes", &PROBE_N, &[(p_obj_1) as u32], false); }
    let mut l_face_2: u32 = 0;
    let r_ = 'f: {
    l_face_2 = c.m.r32((p_obj_1).wrapping_add(5732));
    if (((((l_face_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32((l_face_2).wrapping_add(4)) as i32)) & (1i32))) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_HasEyes", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_HasMouth(c: &mut Ctx, mut p_obj_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_HasMouth", &PROBE_N, &[(p_obj_1) as u32], false); }
    let mut l_face_2: u32 = 0;
    let r_ = 'f: {
    l_face_2 = c.m.r32((p_obj_1).wrapping_add(5732));
    if (((((l_face_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32((l_face_2).wrapping_add(4)) as i32)) & (2i32))) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_HasMouth", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_IsJawActive(c: &mut Ctx, mut p_obj_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_IsJawActive", &PROBE_N, &[(p_obj_1) as u32], false); }
    let mut l_face_2: u32 = 0;
    let r_ = 'f: {
    l_face_2 = c.m.r32((p_obj_1).wrapping_add(5732));
    if (((((l_face_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((c.m.r32((l_face_2).wrapping_add(24)) as i32)) != (0i32)) as i32)) != 0) {
        if ((((D::call(c, fid::BtlObjMdl_HasJaw, &[((p_obj_1) as u64)])) as i32)) != 0) {
            break 'f 1i32;
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_IsJawActive", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_GetJawRot(c: &mut Ctx, mut p_obj_1: u32, mut p_out_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_GetJawRot", &PROBE_N, &[(p_obj_1) as u32, (p_out_2) as u32], false); }
    let mut l_face_3: u32 = 0;
    let r_ = 'f: {
    l_face_3 = c.m.r32((p_obj_1).wrapping_add(5732));
    if (((((l_face_3) == (((0i32) as u32))) as i32)) != 0) {
        F::Vec4_Copy(c, p_out_2, sym::D_002EC2C0);
        break 'f 0i32;
    }
    F::Quat_FromEuler(c, p_out_2, (l_face_3).wrapping_add(80));
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_GetJawRot", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_GetMouthMode(c: &mut Ctx, mut p_obj_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_GetMouthMode", &PROBE_N, &[(p_obj_1) as u32], false); }
    let mut l_face_2: u32 = 0;
    let r_ = 'f: {
    l_face_2 = c.m.r32((p_obj_1).wrapping_add(5732));
    if (((((l_face_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f (1i32).wrapping_neg();
    }
    break 'f (c.m.r32((l_face_2).wrapping_add(24)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_GetMouthMode", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_GetEyeFrame(c: &mut Ctx, mut p_obj_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_GetEyeFrame", &PROBE_N, &[(p_obj_1) as u32], false); }
    let mut l_face_2: u32 = 0;
    let r_ = 'f: {
    l_face_2 = c.m.r32((p_obj_1).wrapping_add(5732));
    if (((((l_face_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f (1i32).wrapping_neg();
    }
    break 'f ((c.m.r32((l_face_2).wrapping_add(20))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_GetEyeFrame", &PROBE_N, &[], true); }
    r_
}

pub fn BObjFile_GetEntry(c: &mut Ctx, mut p_file_1: u32, mut p_n_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BObjFile_GetEntry", &PROBE_N, &[(p_file_1) as u32, (p_n_2) as u32], false); }
    let mut l_buf_3: u32 = 0;
    let r_ = 'f: {
    l_buf_3 = c.m.r32(p_file_1);
    if (((((c.m.r32((l_buf_3).wrapping_add(((((p_n_2).wrapping_add(1i32)) as u32)).wrapping_mul(4)))) != (c.m.r32((l_buf_3).wrapping_add((((p_n_2) as u32)).wrapping_mul(4))))) as i32)) != 0) {
        break 'f (l_buf_3).wrapping_add(((c.m.r32((l_buf_3).wrapping_add((((p_n_2) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BObjFile_GetEntry", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_BindTables(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_BindTables", &PROBE_N, &[(p_obj_1) as u32], false); }
    let mut l_mdl_2: u32 = 0;
    let mut l_slot_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_j_5: i32 = 0;
    let mut l_base_6: i32 = 0;
    let r_ = 'f: {
    l_mdl_2 = (p_obj_1).wrapping_add(24);
    l_slot_3 = c.m.r32((p_obj_1).wrapping_add(20));
    { let v_ = F::BObjFile_GetEntry(c, (l_slot_3).wrapping_add((((0i32) as u32)).wrapping_mul(16)), 3i32); let a_ = l_mdl_2; c.m.w32(a_, v_); };
    { let v_ = F::BObjFile_GetEntry(c, (l_slot_3).wrapping_add((((0i32) as u32)).wrapping_mul(16)), 12i32); let a_ = (l_mdl_2).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = F::BObjFile_GetEntry(c, (l_slot_3).wrapping_add((((0i32) as u32)).wrapping_mul(16)), 2i32); let a_ = (l_mdl_2).wrapping_add(60); c.m.w32(a_, v_); };
    if (((((c.m.r32((l_slot_3).wrapping_add((((1i32) as u32)).wrapping_mul(16)))) != (((0i32) as u32))) as i32)) != 0) {
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < (414i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = F::BObjFile_GetEntry(c, (l_slot_3).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (l_i_4).wrapping_add(1i32)); let a_ = ((((p_obj_1).wrapping_add(12)).wrapping_add(12)).wrapping_add(168)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
    }
    {
        l_i_4 = 0i32;
        'l3: loop {
            if !(((((l_i_4) < (8i32)) as i32)) != 0) { break 'l3; }
            'c4: {
                { let v_ = F::BObjFile_GetEntry(c, (l_slot_3).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (l_i_4).wrapping_add(29i32)); let a_ = ((l_mdl_2).wrapping_add(1824)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    {
        l_i_4 = 0i32;
        'l5: loop {
            if !(((((l_i_4) < (8i32)) as i32)) != 0) { break 'l5; }
            'c6: {
                { let v_ = F::BObjFile_GetEntry(c, (l_slot_3).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (l_i_4).wrapping_add(4i32)); let a_ = ((l_mdl_2).wrapping_add(68)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    {
        { l_i_4 = 0i32; l_j_5 = 0i32; };
        'l7: loop {
            if !(((((l_i_4) < (8i32)) as i32)) != 0) { break 'l7; }
            'c8: {
                if (((((l_i_4) >= (4i32)) as i32)) != 0) {
                    { let v_ = F::BObjFile_GetEntry(c, (l_slot_3).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (l_j_5).wrapping_add(14i32)); let a_ = ((l_mdl_2).wrapping_add(100)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let o_ = l_j_5; let n_ = (o_).wrapping_add(1); l_j_5 = n_; };
                } else {
                    { let v_ = ((0i32) as u32); let a_ = ((l_mdl_2).wrapping_add(100)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    { let v_ = F::BObjFile_GetEntry(c, (l_slot_3).wrapping_add((((0i32) as u32)).wrapping_mul(16)), 13i32); let a_ = (l_mdl_2).wrapping_add(52); c.m.w32(a_, v_); };
    {
        l_i_4 = 0i32;
        'l9: loop {
            if !(((((l_i_4) < (3i32)) as i32)) != 0) { break 'l9; }
            'c10: {
                { let v_ = F::BObjFile_GetEntry(c, (l_slot_3).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (l_i_4).wrapping_add(41i32)); let a_ = ((((p_obj_1).wrapping_add(12)).wrapping_add(12)).wrapping_add(152)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    {
        l_i_4 = 0i32;
        'l11: loop {
            if !(((((l_i_4) < (8i32)) as i32)) != 0) { break 'l11; }
            'c12: {
                { let v_ = F::BObjFile_GetEntry(c, (l_slot_3).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (l_i_4).wrapping_add(29i32)); let a_ = ((l_mdl_2).wrapping_add(2256)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    {
        l_i_4 = 0i32;
        'l13: loop {
            if !(((((l_i_4) < (4i32)) as i32)) != 0) { break 'l13; }
            'c14: {
                { let v_ = F::BObjFile_GetEntry(c, (l_slot_3).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (l_i_4).wrapping_add(37i32)); let a_ = ((l_mdl_2).wrapping_add(2288)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    { let v_ = { let a1_ = ({ let a_ = c.m.r32(sym::gProgress); (c.m.r32(a_) as i32) }).wrapping_add(45i32); F::BObjFile_GetEntry(c, (l_slot_3).wrapping_add((((0i32) as u32)).wrapping_mul(16)), a1_) }; let a_ = (l_mdl_2).wrapping_add(164); c.m.w32(a_, v_); };
    l_base_6 = 100i32;
    if ((((!(((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); c.m.r32(a_) }) & (((1i32) as u32)))) != 0)) as i32)) != 0) {
        l_base_6 = 0i32;
    }
    {
        l_i_4 = 0i32;
        'l15: loop {
            if !(((((l_i_4) < (100i32)) as i32)) != 0) { break 'l15; }
            'c16: {
                { let v_ = F::BObjFile_GetEntry(c, (l_slot_3).wrapping_add((((0i32) as u32)).wrapping_mul(16)), ((l_base_6).wrapping_add(53i32)).wrapping_add(l_i_4)); let a_ = ((l_mdl_2).wrapping_add(1856)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    if (((((c.m.r32((l_slot_3).wrapping_add((((2i32) as u32)).wrapping_mul(16)))) != (((0i32) as u32))) as i32)) != 0) {
        {
            l_i_4 = 0i32;
            'l17: loop {
                if !(((((l_i_4) < (5i32)) as i32)) != 0) { break 'l17; }
                'c18: {
                    { let v_ = F::BObjFile_GetEntry(c, (l_slot_3).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (l_i_4).wrapping_add(1i32)); let a_ = ((l_mdl_2).wrapping_add(132)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        { let v_ = F::BObjFile_GetEntry(c, (l_slot_3).wrapping_add((((2i32) as u32)).wrapping_mul(16)), 6i32); let a_ = (l_mdl_2).wrapping_add(64); c.m.w32(a_, v_); };
    }
    { let v_ = F::BObjFile_GetEntry(c, (l_slot_3).wrapping_add((((0i32) as u32)).wrapping_mul(16)), 1i32); let a_ = (l_mdl_2).wrapping_add(56); c.m.w32(a_, v_); };
    if (((((c.m.r32((l_mdl_2).wrapping_add(56))) != (((0i32) as u32))) as i32)) != 0) {
        { let a1_ = c.m.r32((l_mdl_2).wrapping_add(56)); let a2_ = c.m.r32((l_mdl_2).wrapping_add(56)); { let _ = D::call(c, fid::Res_RelocateOffsets, &[(((l_mdl_2).wrapping_add(56)) as u64), ((a1_) as u64), ((a2_) as u64)]); } };
    }
    { let v_ = F::BObjFile_GetEntry(c, (l_slot_3).wrapping_add((((0i32) as u32)).wrapping_mul(16)), 22i32); let a_ = (l_mdl_2).wrapping_add(2340); c.m.w32(a_, v_); };
    { let v_ = F::BObjFile_GetEntry(c, (l_slot_3).wrapping_add((((0i32) as u32)).wrapping_mul(16)), 18i32); let a_ = (l_mdl_2).wrapping_add(2308); c.m.w32(a_, v_); };
    { let v_ = F::BObjFile_GetEntry(c, (l_slot_3).wrapping_add((((0i32) as u32)).wrapping_mul(16)), 19i32); let a_ = (l_mdl_2).wrapping_add(2312); c.m.w32(a_, v_); };
    { let v_ = F::BObjFile_GetEntry(c, (l_slot_3).wrapping_add((((0i32) as u32)).wrapping_mul(16)), 20i32); let a_ = (l_mdl_2).wrapping_add(2316); c.m.w32(a_, v_); };
    { let v_ = F::BObjFile_GetEntry(c, (l_slot_3).wrapping_add((((0i32) as u32)).wrapping_mul(16)), 21i32); let a_ = (l_mdl_2).wrapping_add(2320); c.m.w32(a_, v_); };
    { let v_ = F::BObjFile_GetEntry(c, (l_slot_3).wrapping_add((((0i32) as u32)).wrapping_mul(16)), 24i32); let a_ = (l_mdl_2).wrapping_add(2324); c.m.w32(a_, v_); };
    { let v_ = F::BObjFile_GetEntry(c, (l_slot_3).wrapping_add((((0i32) as u32)).wrapping_mul(16)), 25i32); let a_ = (l_mdl_2).wrapping_add(2328); c.m.w32(a_, v_); };
    { let v_ = F::BObjFile_GetEntry(c, (l_slot_3).wrapping_add((((0i32) as u32)).wrapping_mul(16)), 27i32); let a_ = (l_mdl_2).wrapping_add(2332); c.m.w32(a_, v_); };
    { let v_ = F::BObjFile_GetEntry(c, (l_slot_3).wrapping_add((((0i32) as u32)).wrapping_mul(16)), 28i32); let a_ = (l_mdl_2).wrapping_add(2336); c.m.w32(a_, v_); };
    { let v_ = F::BObjFile_GetEntry(c, (l_slot_3).wrapping_add((((0i32) as u32)).wrapping_mul(16)), 23i32); let a_ = (l_mdl_2).wrapping_add(2344); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_BindTables", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_InitFlags(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_InitFlags", &PROBE_N, &[(p_obj_1) as u32], false); }
    let mut l_state_2: u32 = 0;
    let mut l_mdl_3: u32 = 0;
    let mut l_top_4: i32 = 0;
    let r_ = 'f: {
    l_state_2 = (p_obj_1).wrapping_add(2624);
    l_mdl_3 = (p_obj_1).wrapping_add(24);
    { let v_ = ((33554438i32) as u32); let a_ = l_state_2; c.m.w32(a_, v_); };
    if ((((((((((c.m.r32(p_obj_1) as i32)) == (0i32)) as i32)) != 0) || ((((((c.m.r32(p_obj_1) as i32)) == (2i32)) as i32)) != 0)) as i32)) != 0) {
        { let v_ = ((301989910i32) as u32); let a_ = l_state_2; c.m.w32(a_, v_); };
    } else {
        if ((((((c.m.r32(p_obj_1) as i32)) != (1i32)) as i32)) != 0) {
            { let v_ = ((570425366i32) as u32); let a_ = l_state_2; c.m.w32(a_, v_); };
        }
    }
    if (((({ let a_ = (c.m.r32((l_mdl_3).wrapping_add(40))).wrapping_add(12); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
        { let r_ = ((8i32) as u32); let a_ = l_state_2; let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    }
    if (((({ let a_ = (c.m.r32((l_mdl_3).wrapping_add(40))).wrapping_add(12); c.m.r32(a_) }) & (((16777216i32) as u32)))) != 0) {
        { let r_ = ((131072i32) as u32); let a_ = l_state_2; let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    } else {
        { let r_ = ((65536i32) as u32); let a_ = l_state_2; let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    }
    if (((((c.m.r32((l_mdl_3).wrapping_add(2308))) == (((0i32) as u32))) as i32)) != 0) {
        F::BtlObj_SetColorPreset(c, p_obj_1, 0i32, 0i32);
    } else {
        l_top_4 = (({ let a_ = (c.m.r32((l_mdl_3).wrapping_add(40))).wrapping_add(15); (c.m.r8(a_) as i8) }) as i32);
        { let a1_ = (({ let a_ = (c.m.r32((l_mdl_3).wrapping_add(2308))).wrapping_add(((3i32) as u32)); (c.m.r8(a_) as i8) }) as i32); F::BtlObj_SetColorPreset(c, p_obj_1, a1_, ((l_top_4) & (1i32))) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_InitFlags", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_SetColorPreset(c: &mut Ctx, mut p_obj_1: u32, mut p_row_2: i32, mut p_set_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_SetColorPreset", &PROBE_N, &[(p_obj_1) as u32, (p_row_2) as u32, (p_set_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(448) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 448);
    let mut l_state_4: u32 = 0;
    let mut l_mdl_5: u32 = 0;
    let mut l_top_6: i32 = 0;
    let r_ = 'f: {
        l_state_4 = (p_obj_1).wrapping_add(2624);
        l_mdl_5 = (p_obj_1).wrapping_add(24);
        if (((((p_set_3) < (0i32)) as i32)) != 0) {
            l_top_6 = (({ let a_ = (c.m.r32((l_mdl_5).wrapping_add(40))).wrapping_add(15); (c.m.r8(a_) as i8) }) as i32);
            p_set_3 = ((l_top_6) & (1i32));
        } else {
            if (((((p_set_3) >= (2i32)) as i32)) != 0) {
                p_set_3 = 0i32;
            }
        }
        {
            rt::memset(c, fp, 0, 440);
            rt::memset(c, fp.wrapping_add(0), 0, 220);
            rt::memset(c, fp.wrapping_add(0).wrapping_add(0), 0, 20);
            { let v_ = F32(0x427c0000); c.m.w32(fp.wrapping_add(0).wrapping_add(0).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x427c0000); c.m.w32(fp.wrapping_add(0).wrapping_add(0).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(0).wrapping_add(0).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x42200000); c.m.w32(fp.wrapping_add(0).wrapping_add(0).wrapping_add(12), (v_).0); }
            { let v_ = F32(0x42820000); c.m.w32(fp.wrapping_add(0).wrapping_add(0).wrapping_add(16), (v_).0); }
            rt::memset(c, fp.wrapping_add(0).wrapping_add(20), 0, 20);
            { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(0).wrapping_add(20).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x427c0000); c.m.w32(fp.wrapping_add(0).wrapping_add(20).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(0).wrapping_add(20).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x42200000); c.m.w32(fp.wrapping_add(0).wrapping_add(20).wrapping_add(12), (v_).0); }
            { let v_ = F32(0x42820000); c.m.w32(fp.wrapping_add(0).wrapping_add(20).wrapping_add(16), (v_).0); }
            rt::memset(c, fp.wrapping_add(0).wrapping_add(40), 0, 20);
            { let v_ = F32(0x430e0000); c.m.w32(fp.wrapping_add(0).wrapping_add(40).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x430e0000); c.m.w32(fp.wrapping_add(0).wrapping_add(40).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x41c80000); c.m.w32(fp.wrapping_add(0).wrapping_add(40).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x42200000); c.m.w32(fp.wrapping_add(0).wrapping_add(40).wrapping_add(12), (v_).0); }
            { let v_ = F32(0x42820000); c.m.w32(fp.wrapping_add(0).wrapping_add(40).wrapping_add(16), (v_).0); }
            rt::memset(c, fp.wrapping_add(0).wrapping_add(60), 0, 20);
            { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(0).wrapping_add(60).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x427c0000); c.m.w32(fp.wrapping_add(0).wrapping_add(60).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x427c0000); c.m.w32(fp.wrapping_add(0).wrapping_add(60).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x42200000); c.m.w32(fp.wrapping_add(0).wrapping_add(60).wrapping_add(12), (v_).0); }
            { let v_ = F32(0x42820000); c.m.w32(fp.wrapping_add(0).wrapping_add(60).wrapping_add(16), (v_).0); }
            rt::memset(c, fp.wrapping_add(0).wrapping_add(80), 0, 20);
            { let v_ = F32(0x427c0000); c.m.w32(fp.wrapping_add(0).wrapping_add(80).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(0).wrapping_add(80).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x427c0000); c.m.w32(fp.wrapping_add(0).wrapping_add(80).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x42200000); c.m.w32(fp.wrapping_add(0).wrapping_add(80).wrapping_add(12), (v_).0); }
            { let v_ = F32(0x42820000); c.m.w32(fp.wrapping_add(0).wrapping_add(80).wrapping_add(16), (v_).0); }
            rt::memset(c, fp.wrapping_add(0).wrapping_add(100), 0, 20);
            { let v_ = F32(0x427c0000); c.m.w32(fp.wrapping_add(0).wrapping_add(100).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x42c80000); c.m.w32(fp.wrapping_add(0).wrapping_add(100).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(0).wrapping_add(100).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x42200000); c.m.w32(fp.wrapping_add(0).wrapping_add(100).wrapping_add(12), (v_).0); }
            { let v_ = F32(0x42820000); c.m.w32(fp.wrapping_add(0).wrapping_add(100).wrapping_add(16), (v_).0); }
            rt::memset(c, fp.wrapping_add(0).wrapping_add(120), 0, 20);
            { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(0).wrapping_add(120).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x427c0000); c.m.w32(fp.wrapping_add(0).wrapping_add(120).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x427c0000); c.m.w32(fp.wrapping_add(0).wrapping_add(120).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x42200000); c.m.w32(fp.wrapping_add(0).wrapping_add(120).wrapping_add(12), (v_).0); }
            { let v_ = F32(0x42820000); c.m.w32(fp.wrapping_add(0).wrapping_add(120).wrapping_add(16), (v_).0); }
            rt::memset(c, fp.wrapping_add(0).wrapping_add(140), 0, 20);
            { let v_ = F32(0x43000000); c.m.w32(fp.wrapping_add(0).wrapping_add(140).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x427c0000); c.m.w32(fp.wrapping_add(0).wrapping_add(140).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(0).wrapping_add(140).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x42200000); c.m.w32(fp.wrapping_add(0).wrapping_add(140).wrapping_add(12), (v_).0); }
            { let v_ = F32(0x42820000); c.m.w32(fp.wrapping_add(0).wrapping_add(140).wrapping_add(16), (v_).0); }
            rt::memset(c, fp.wrapping_add(0).wrapping_add(160), 0, 20);
            { let v_ = F32(0x430e0000); c.m.w32(fp.wrapping_add(0).wrapping_add(160).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(0).wrapping_add(160).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x41c80000); c.m.w32(fp.wrapping_add(0).wrapping_add(160).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x42200000); c.m.w32(fp.wrapping_add(0).wrapping_add(160).wrapping_add(12), (v_).0); }
            { let v_ = F32(0x42820000); c.m.w32(fp.wrapping_add(0).wrapping_add(160).wrapping_add(16), (v_).0); }
            rt::memset(c, fp.wrapping_add(0).wrapping_add(180), 0, 20);
            { let v_ = F32(0x430e0000); c.m.w32(fp.wrapping_add(0).wrapping_add(180).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(0).wrapping_add(180).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x41c80000); c.m.w32(fp.wrapping_add(0).wrapping_add(180).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x42200000); c.m.w32(fp.wrapping_add(0).wrapping_add(180).wrapping_add(12), (v_).0); }
            { let v_ = F32(0x42820000); c.m.w32(fp.wrapping_add(0).wrapping_add(180).wrapping_add(16), (v_).0); }
            rt::memset(c, fp.wrapping_add(0).wrapping_add(200), 0, 20);
            { let v_ = F32(0x430e0000); c.m.w32(fp.wrapping_add(0).wrapping_add(200).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(0).wrapping_add(200).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x41c80000); c.m.w32(fp.wrapping_add(0).wrapping_add(200).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x42200000); c.m.w32(fp.wrapping_add(0).wrapping_add(200).wrapping_add(12), (v_).0); }
            { let v_ = F32(0x42820000); c.m.w32(fp.wrapping_add(0).wrapping_add(200).wrapping_add(16), (v_).0); }
            rt::memset(c, fp.wrapping_add(220), 0, 220);
            rt::memset(c, fp.wrapping_add(220).wrapping_add(0), 0, 20);
            { let v_ = F32(0x427c0000); c.m.w32(fp.wrapping_add(220).wrapping_add(0).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x427c0000); c.m.w32(fp.wrapping_add(220).wrapping_add(0).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(220).wrapping_add(0).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x42200000); c.m.w32(fp.wrapping_add(220).wrapping_add(0).wrapping_add(12), (v_).0); }
            { let v_ = F32(0x42820000); c.m.w32(fp.wrapping_add(220).wrapping_add(0).wrapping_add(16), (v_).0); }
            rt::memset(c, fp.wrapping_add(220).wrapping_add(20), 0, 20);
            { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(220).wrapping_add(20).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x427c0000); c.m.w32(fp.wrapping_add(220).wrapping_add(20).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(220).wrapping_add(20).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x42200000); c.m.w32(fp.wrapping_add(220).wrapping_add(20).wrapping_add(12), (v_).0); }
            { let v_ = F32(0x42820000); c.m.w32(fp.wrapping_add(220).wrapping_add(20).wrapping_add(16), (v_).0); }
            rt::memset(c, fp.wrapping_add(220).wrapping_add(40), 0, 20);
            { let v_ = F32(0x430e0000); c.m.w32(fp.wrapping_add(220).wrapping_add(40).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x430e0000); c.m.w32(fp.wrapping_add(220).wrapping_add(40).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x41c80000); c.m.w32(fp.wrapping_add(220).wrapping_add(40).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x42200000); c.m.w32(fp.wrapping_add(220).wrapping_add(40).wrapping_add(12), (v_).0); }
            { let v_ = F32(0x42820000); c.m.w32(fp.wrapping_add(220).wrapping_add(40).wrapping_add(16), (v_).0); }
            rt::memset(c, fp.wrapping_add(220).wrapping_add(60), 0, 20);
            { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(220).wrapping_add(60).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x427c0000); c.m.w32(fp.wrapping_add(220).wrapping_add(60).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x427c0000); c.m.w32(fp.wrapping_add(220).wrapping_add(60).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x42200000); c.m.w32(fp.wrapping_add(220).wrapping_add(60).wrapping_add(12), (v_).0); }
            { let v_ = F32(0x42820000); c.m.w32(fp.wrapping_add(220).wrapping_add(60).wrapping_add(16), (v_).0); }
            rt::memset(c, fp.wrapping_add(220).wrapping_add(80), 0, 20);
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(220).wrapping_add(80).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x432b0000); c.m.w32(fp.wrapping_add(220).wrapping_add(80).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(220).wrapping_add(80).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x42200000); c.m.w32(fp.wrapping_add(220).wrapping_add(80).wrapping_add(12), (v_).0); }
            { let v_ = F32(0x42820000); c.m.w32(fp.wrapping_add(220).wrapping_add(80).wrapping_add(16), (v_).0); }
            rt::memset(c, fp.wrapping_add(220).wrapping_add(100), 0, 20);
            { let v_ = F32(0x427c0000); c.m.w32(fp.wrapping_add(220).wrapping_add(100).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x42c80000); c.m.w32(fp.wrapping_add(220).wrapping_add(100).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(220).wrapping_add(100).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x42200000); c.m.w32(fp.wrapping_add(220).wrapping_add(100).wrapping_add(12), (v_).0); }
            { let v_ = F32(0x42820000); c.m.w32(fp.wrapping_add(220).wrapping_add(100).wrapping_add(16), (v_).0); }
            rt::memset(c, fp.wrapping_add(220).wrapping_add(120), 0, 20);
            { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(220).wrapping_add(120).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x427c0000); c.m.w32(fp.wrapping_add(220).wrapping_add(120).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x427c0000); c.m.w32(fp.wrapping_add(220).wrapping_add(120).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x42200000); c.m.w32(fp.wrapping_add(220).wrapping_add(120).wrapping_add(12), (v_).0); }
            { let v_ = F32(0x42820000); c.m.w32(fp.wrapping_add(220).wrapping_add(120).wrapping_add(16), (v_).0); }
            rt::memset(c, fp.wrapping_add(220).wrapping_add(140), 0, 20);
            { let v_ = F32(0x43000000); c.m.w32(fp.wrapping_add(220).wrapping_add(140).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x427c0000); c.m.w32(fp.wrapping_add(220).wrapping_add(140).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(220).wrapping_add(140).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x42200000); c.m.w32(fp.wrapping_add(220).wrapping_add(140).wrapping_add(12), (v_).0); }
            { let v_ = F32(0x42820000); c.m.w32(fp.wrapping_add(220).wrapping_add(140).wrapping_add(16), (v_).0); }
            rt::memset(c, fp.wrapping_add(220).wrapping_add(160), 0, 20);
            { let v_ = F32(0x430e0000); c.m.w32(fp.wrapping_add(220).wrapping_add(160).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x430e0000); c.m.w32(fp.wrapping_add(220).wrapping_add(160).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x42000000); c.m.w32(fp.wrapping_add(220).wrapping_add(160).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x42200000); c.m.w32(fp.wrapping_add(220).wrapping_add(160).wrapping_add(12), (v_).0); }
            { let v_ = F32(0x42820000); c.m.w32(fp.wrapping_add(220).wrapping_add(160).wrapping_add(16), (v_).0); }
            rt::memset(c, fp.wrapping_add(220).wrapping_add(180), 0, 20);
            { let v_ = F32(0x430e0000); c.m.w32(fp.wrapping_add(220).wrapping_add(180).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x430e0000); c.m.w32(fp.wrapping_add(220).wrapping_add(180).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x42000000); c.m.w32(fp.wrapping_add(220).wrapping_add(180).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x42200000); c.m.w32(fp.wrapping_add(220).wrapping_add(180).wrapping_add(12), (v_).0); }
            { let v_ = F32(0x42820000); c.m.w32(fp.wrapping_add(220).wrapping_add(180).wrapping_add(16), (v_).0); }
            rt::memset(c, fp.wrapping_add(220).wrapping_add(200), 0, 20);
            { let v_ = F32(0x430e0000); c.m.w32(fp.wrapping_add(220).wrapping_add(200).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x430e0000); c.m.w32(fp.wrapping_add(220).wrapping_add(200).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x42000000); c.m.w32(fp.wrapping_add(220).wrapping_add(200).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x42200000); c.m.w32(fp.wrapping_add(220).wrapping_add(200).wrapping_add(12), (v_).0); }
            { let v_ = F32(0x42820000); c.m.w32(fp.wrapping_add(220).wrapping_add(200).wrapping_add(16), (v_).0); }
            { let v_ = F32(c.m.r32((((fp).wrapping_add((((p_set_3) as u32)).wrapping_mul(220))).wrapping_add((((p_row_2) as u32)).wrapping_mul(20))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((l_state_4).wrapping_add(204)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32((((fp).wrapping_add((((p_set_3) as u32)).wrapping_mul(220))).wrapping_add((((p_row_2) as u32)).wrapping_mul(20))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((l_state_4).wrapping_add(204)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32((((fp).wrapping_add((((p_set_3) as u32)).wrapping_mul(220))).wrapping_add((((p_row_2) as u32)).wrapping_mul(20))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = ((l_state_4).wrapping_add(204)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32((((fp).wrapping_add((((p_set_3) as u32)).wrapping_mul(220))).wrapping_add((((p_row_2) as u32)).wrapping_mul(20))).wrapping_add((((3i32) as u32)).wrapping_mul(4)))); let a_ = ((l_state_4).wrapping_add(204)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32((((fp).wrapping_add((((p_set_3) as u32)).wrapping_mul(220))).wrapping_add((((p_row_2) as u32)).wrapping_mul(20))).wrapping_add((((4i32) as u32)).wrapping_mul(4)))); let a_ = ((l_state_4).wrapping_add(204)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_SetColorPreset", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_BindCommonTables(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_BindCommonTables", &PROBE_N, &[(p_obj_1) as u32], false); }
    let mut l_res_2: u32 = 0;
    let mut l_mdl_3: u32 = 0;
    let mut l_first_4: i32 = 0;
    let mut l_ofs_5: i32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_p_7: u32 = 0;
    let r_ = 'f: {
    l_first_4 = 100i32;
    if ((((!(((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); c.m.r32(a_) }) & (((1i32) as u32)))) != 0)) as i32)) != 0) {
        l_first_4 = 0i32;
    }
    l_res_2 = c.m.r32(sym::gCommonRes);
    l_mdl_3 = (p_obj_1).wrapping_add(24);
    l_ofs_5 = ((l_first_4).wrapping_mul(4i32)).wrapping_add(56i32);
    {
        l_i_6 = 0i32;
        'l1: loop {
            if !(((((l_i_6) < (100i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_p_7 = (((l_ofs_5) as u32)).wrapping_add(c.m.r32((l_res_2).wrapping_add(4)));
                { let r_ = 4i32; let n_ = (l_ofs_5).wrapping_add(r_); l_ofs_5 = n_; };
                if (((((c.m.r32((l_p_7).wrapping_add((((1i32) as u32)).wrapping_mul(4)))) != (c.m.r32((l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(4))))) as i32)) != 0) {
                    { let v_ = (c.m.r32((l_res_2).wrapping_add(4))).wrapping_add(((c.m.r32((l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (((p_obj_1).wrapping_add(24)).wrapping_add(1856)).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                } else {
                    { let v_ = ((0i32) as u32); let a_ = (((p_obj_1).wrapping_add(24)).wrapping_add(1856)).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
            }
            { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
        }
    }
    if ((((({ let a_ = (c.m.r32((l_res_2).wrapping_add(4))).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != ({ let a_ = (c.m.r32((l_res_2).wrapping_add(4))).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.r32(a_) })) as i32)) != 0) {
        { let v_ = (c.m.r32((l_res_2).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((l_res_2).wrapping_add(4))).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (l_mdl_3).wrapping_add(156); c.m.w32(a_, v_); };
    } else {
        { let v_ = ((0i32) as u32); let a_ = (l_mdl_3).wrapping_add(156); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32((l_res_2).wrapping_add(4))).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != ({ let a_ = (c.m.r32((l_res_2).wrapping_add(4))).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.r32(a_) })) as i32)) != 0) {
        { let v_ = (c.m.r32((l_res_2).wrapping_add(4))).wrapping_add((({ let a_ = (c.m.r32((l_res_2).wrapping_add(4))).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (l_mdl_3).wrapping_add(2304); c.m.w32(a_, v_); };
    } else {
        { let v_ = ((0i32) as u32); let a_ = (l_mdl_3).wrapping_add(2304); c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_BindCommonTables", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_CopyLipTables(c: &mut Ctx, mut p_obj_1: u32, mut p_src_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_CopyLipTables", &PROBE_N, &[(p_obj_1) as u32, (p_src_2) as u32], false); }
    let mut l_dst_3: u32 = 0;
    let mut l_from_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
    l_dst_3 = (p_obj_1).wrapping_add(24);
    l_from_4 = (p_src_2).wrapping_add(24);
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((l_i_5) < (100i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = c.m.r32(((l_from_4).wrapping_add(1856)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))); let a_ = ((l_dst_3).wrapping_add(1856)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_CopyLipTables", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_SetColorMode(c: &mut Ctx, mut p_obj_1: u32, mut p_bit_2: i32, mut p_on_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_SetColorMode", &PROBE_N, &[(p_obj_1) as u32, (p_bit_2) as u32, (p_on_3) as u32], false); }
    let mut l_state_4: u32 = 0;
    let mut l_mask_5: u32 = 0;
    let r_ = 'f: {
    l_state_4 = (p_obj_1).wrapping_add(2624);
    l_mask_5 = (((1i32).wrapping_shl((p_bit_2) as u32)) as u32);
    if ((p_on_3) != 0) {
        { let r_ = l_mask_5; let a_ = (l_state_4).wrapping_add(244); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    } else {
        { let r_ = (!(l_mask_5)); let a_ = (l_state_4).wrapping_add(244); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
    }
    if (((((((((p_bit_2) == (7i32)) as i32)) != 0) || (((((c.m.r32((l_state_4).wrapping_add(244))) == (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        if (((({ let a_ = (c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(40))).wrapping_add(12); c.m.r32(a_) }) & (((16777216i32) as u32)))) != 0) {
            { let v_ = ((((c.m.r32((p_obj_1).wrapping_add(2624))) & ((((!(16711680i32))) as u32)))) | (((131072i32) as u32))); let a_ = (p_obj_1).wrapping_add(2624); c.m.w32(a_, v_); };
        } else {
            { let v_ = ((((c.m.r32((p_obj_1).wrapping_add(2624))) & ((((!(16711680i32))) as u32)))) | (((65536i32) as u32))); let a_ = (p_obj_1).wrapping_add(2624); c.m.w32(a_, v_); };
        }
        { let v_ = ((0i32) as u32); let a_ = (l_state_4).wrapping_add(244); c.m.w32(a_, v_); };
    } else {
        if ((((c.m.r32((l_state_4).wrapping_add(244))) & (((8i32) as u32)))) != 0) {
            { let v_ = ((((c.m.r32((p_obj_1).wrapping_add(2624))) & ((((!(16711680i32))) as u32)))) | (((524288i32) as u32))); let a_ = (p_obj_1).wrapping_add(2624); c.m.w32(a_, v_); };
        } else {
            if ((((c.m.r32((l_state_4).wrapping_add(244))) & (((4i32) as u32)))) != 0) {
                { let v_ = ((((c.m.r32((p_obj_1).wrapping_add(2624))) & ((((!(16711680i32))) as u32)))) | (((262144i32) as u32))); let a_ = (p_obj_1).wrapping_add(2624); c.m.w32(a_, v_); };
            } else {
                if ((((c.m.r32((l_state_4).wrapping_add(244))) & (((64i32) as u32)))) != 0) {
                    { let v_ = ((((c.m.r32((p_obj_1).wrapping_add(2624))) & ((((!(16711680i32))) as u32)))) | (((4194304i32) as u32))); let a_ = (p_obj_1).wrapping_add(2624); c.m.w32(a_, v_); };
                } else {
                    if ((((c.m.r32((l_state_4).wrapping_add(244))) & (((1i32) as u32)))) != 0) {
                        { let v_ = ((((c.m.r32((p_obj_1).wrapping_add(2624))) & ((((!(16711680i32))) as u32)))) | (((65536i32) as u32))); let a_ = (p_obj_1).wrapping_add(2624); c.m.w32(a_, v_); };
                    } else {
                        if ((((c.m.r32((l_state_4).wrapping_add(244))) & (((2i32) as u32)))) != 0) {
                            { let v_ = ((((c.m.r32((p_obj_1).wrapping_add(2624))) & ((((!(16711680i32))) as u32)))) | (((131072i32) as u32))); let a_ = (p_obj_1).wrapping_add(2624); c.m.w32(a_, v_); };
                        } else {
                            if ((((c.m.r32((l_state_4).wrapping_add(244))) & (((16i32) as u32)))) != 0) {
                                { let v_ = ((((c.m.r32((p_obj_1).wrapping_add(2624))) & ((((!(16711680i32))) as u32)))) | (((1048576i32) as u32))); let a_ = (p_obj_1).wrapping_add(2624); c.m.w32(a_, v_); };
                            } else {
                                if ((((c.m.r32((l_state_4).wrapping_add(244))) & (((32i32) as u32)))) != 0) {
                                    { let v_ = ((((c.m.r32((p_obj_1).wrapping_add(2624))) & ((((!(16711680i32))) as u32)))) | (((2097152i32) as u32))); let a_ = (p_obj_1).wrapping_add(2624); c.m.w32(a_, v_); };
                                } else {
                                    if (((({ let a_ = (c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(40))).wrapping_add(12); c.m.r32(a_) }) & (((16777216i32) as u32)))) != 0) {
                                        { let v_ = ((((c.m.r32((p_obj_1).wrapping_add(2624))) & ((((!(16711680i32))) as u32)))) | (((131072i32) as u32))); let a_ = (p_obj_1).wrapping_add(2624); c.m.w32(a_, v_); };
                                    } else {
                                        { let v_ = ((((c.m.r32((p_obj_1).wrapping_add(2624))) & ((((!(16711680i32))) as u32)))) | (((65536i32) as u32))); let a_ = (p_obj_1).wrapping_add(2624); c.m.w32(a_, v_); };
                                    }
                                    { let v_ = ((0i32) as u32); let a_ = (l_state_4).wrapping_add(244); c.m.w32(a_, v_); };
                                }
                            }
                        }
                    }
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_SetColorMode", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_GetColorMode(c: &mut Ctx, mut p_obj_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_GetColorMode", &PROBE_N, &[(p_obj_1) as u32], false); }
    let mut l_flags_2: u32 = 0;
    let r_ = 'f: {
    l_flags_2 = c.m.r32((p_obj_1).wrapping_add(2624));
    if ((((l_flags_2) & (((65536i32) as u32)))) != 0) {
        break 'f 0i32;
    }
    if ((((l_flags_2) & (((131072i32) as u32)))) != 0) {
        break 'f 1i32;
    }
    if ((((l_flags_2) & (((262144i32) as u32)))) != 0) {
        break 'f 2i32;
    }
    if ((((l_flags_2) & (((524288i32) as u32)))) != 0) {
        break 'f 3i32;
    }
    if ((((l_flags_2) & (((1048576i32) as u32)))) != 0) {
        break 'f 4i32;
    }
    if ((((l_flags_2) & (((4194304i32) as u32)))) != 0) {
        break 'f 6i32;
    }
    if ((((l_flags_2) & (((2097152i32) as u32)))) != 0) {
        break 'f 5i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_GetColorMode", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_SetPartVisible(c: &mut Ctx, mut p_obj_1: u32, mut p_part_2: u32, mut p_on_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_SetPartVisible", &PROBE_N, &[(p_obj_1) as u32, (p_part_2) as u32, (p_on_3) as u32], false); }
    let mut l_mdl_4: u32 = 0;
    let mut l_high_5: u32 = 0;
    let r_ = 'f: {
    l_mdl_4 = (p_obj_1).wrapping_add(24);
    if ((p_on_3) != 0) {
        's1: {
            let sv6: u32 = p_part_2;
            match sv6 {
                0u32 => {
                    if ((((!((((c.m.r32((p_obj_1).wrapping_add(2624))) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
                        { let r_ = ((2i32) as u32); let a_ = (p_obj_1).wrapping_add(2624); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                    }
                    break 's1;
                }
                1u32 => {
                    if ((((!(((((((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 1, 1) as u32)) as i32)) != 0) && (((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 4, 1) as u32)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
                        { let r_ = ((16i32) as u32); let a_ = (p_obj_1).wrapping_add(2624); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                    }
                    break 's1;
                }
                4u32 => {
                    if ((((!(((((((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 1, 1) as u32)) as i32)) != 0) && (((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 7, 1) as u32)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
                        { let r_ = ((128i32) as u32); let a_ = (p_obj_1).wrapping_add(2624); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                    }
                    break 's1;
                }
                2u32 => {
                    if ((((!(((((((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 1, 1) as u32)) as i32)) != 0) && (((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 5, 1) as u32)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
                        { let r_ = ((32i32) as u32); let a_ = (p_obj_1).wrapping_add(2624); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                    }
                    break 's1;
                }
                3u32 => {
                    if ((((!(((((((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 1, 1) as u32)) as i32)) != 0) && (((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 6, 1) as u32)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
                        { let r_ = ((64i32) as u32); let a_ = (p_obj_1).wrapping_add(2624); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
    } else {
        's2: {
            let sv7: u32 = p_part_2;
            match sv7 {
                0u32 => {
                    if ((((c.m.r32((p_obj_1).wrapping_add(2624))) & (((2i32) as u32)))) != 0) {
                        { let r_ = (((!(2i32))) as u32); let a_ = (p_obj_1).wrapping_add(2624); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
                    }
                    break 's2;
                }
                1u32 => {
                    if (((((((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 1, 1) as u32)) as i32)) != 0) && (((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 4, 1) as u32)) as i32)) != 0)) as i32)) != 0) {
                        { let r_ = (((!(16i32))) as u32); let a_ = (p_obj_1).wrapping_add(2624); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
                    }
                    break 's2;
                }
                4u32 => {
                    if (((((((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 1, 1) as u32)) as i32)) != 0) && (((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 7, 1) as u32)) as i32)) != 0)) as i32)) != 0) {
                        { let r_ = (((!(128i32))) as u32); let a_ = (p_obj_1).wrapping_add(2624); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
                    }
                    break 's2;
                }
                2u32 => {
                    if (((((((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 1, 1) as u32)) as i32)) != 0) && (((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 5, 1) as u32)) as i32)) != 0)) as i32)) != 0) {
                        { let r_ = (((!(32i32))) as u32); let a_ = (p_obj_1).wrapping_add(2624); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
                    }
                    break 's2;
                }
                3u32 => {
                    if (((((((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 1, 1) as u32)) as i32)) != 0) && (((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 6, 1) as u32)) as i32)) != 0)) as i32)) != 0) {
                        { let r_ = (((!(64i32))) as u32); let a_ = (p_obj_1).wrapping_add(2624); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
                    }
                    break 's2;
                }
                _ => {
                }
            }
        }
    }
    if (((((p_part_2) == (((5i32) as u32))) as i32)) != 0) {
        l_high_5 = ((c.m.r32((p_obj_1).wrapping_add(2624))) & (4294901760u32));
        { let v_ = ((l_high_5) | (((22i32) as u32))); let a_ = (p_obj_1).wrapping_add(2624); c.m.w32(a_, v_); };
        if (((({ let a_ = (c.m.r32((l_mdl_4).wrapping_add(40))).wrapping_add(12); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
            { let v_ = ((l_high_5) | (((30i32) as u32))); let a_ = (p_obj_1).wrapping_add(2624); c.m.w32(a_, v_); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_SetPartVisible", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_IsPartVisible(c: &mut Ctx, mut p_obj_1: u32, mut p_part_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_IsPartVisible", &PROBE_N, &[(p_obj_1) as u32, (p_part_2) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv3: u32 = p_part_2;
        match sv3 {
            0u32 => {
                break 'f (((((c.m.r32((p_obj_1).wrapping_add(2624))) as i32)).wrapping_shr((1i32) as u32)) & (1i32));
            }
            1u32 => {
                break 'f (((((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 1, 1) as u32)) as i32)) != 0) && (((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 4, 1) as u32)) as i32)) != 0)) as i32);
            }
            4u32 => {
                break 'f (((((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 1, 1) as u32)) as i32)) != 0) && (((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 7, 1) as u32)) as i32)) != 0)) as i32);
            }
            2u32 => {
                break 'f (((((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 1, 1) as u32)) as i32)) != 0) && (((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 5, 1) as u32)) as i32)) != 0)) as i32);
            }
            3u32 => {
                break 'f (((((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 1, 1) as u32)) as i32)) != 0) && (((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 6, 1) as u32)) as i32)) != 0)) as i32);
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_IsPartVisible", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_FindBound(c: &mut Ctx, mut p_obj_1: u32, mut p_node_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_FindBound", &PROBE_N, &[(p_obj_1) as u32, (p_node_2) as u32], false); }
    let mut l_bound_3: u32 = 0;
    let r_ = 'f: {
    l_bound_3 = c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(44));
    'l1: while ((1i32) != 0) {
        if (((((((c.m.r16((l_bound_3).wrapping_add(10))) as i32)) == (p_node_2)) as i32)) != 0) {
            break 'f l_bound_3;
        }
        if (((((((c.m.r16((l_bound_3).wrapping_add(6))) as i32)) != (0i32)) as i32)) != 0) {
            break 'l1;
        }
        l_bound_3 = (l_bound_3).wrapping_add((((c.m.r32(l_bound_3) as i32)) as u32));
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_FindBound", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_GetNode(c: &mut Ctx, mut p_obj_1: u32, mut p_node_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_GetNode", &PROBE_N, &[(p_obj_1) as u32, (p_node_2) as u32], false); }
    let mut l_p_3: u32 = 0;
    let r_ = 'f: {
    l_p_3 = (p_obj_1).wrapping_add((((p_node_2).wrapping_mul(4i32)) as u32));
    break 'f c.m.r32((l_p_3).wrapping_add((((rt::div_i32(3436i32, 4i32)) as u32)).wrapping_mul(4)));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_GetNode", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_HasNode(c: &mut Ctx, mut p_obj_1: u32, mut p_node_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_HasNode", &PROBE_N, &[(p_obj_1) as u32, (p_node_2) as u32], false); }
    let mut l_p_3: u32 = 0;
    let r_ = 'f: {
    l_p_3 = (p_obj_1).wrapping_add((((p_node_2).wrapping_mul(4i32)) as u32));
    break 'f (((c.m.r32((l_p_3).wrapping_add((((rt::div_i32(3436i32, 4i32)) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_HasNode", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_IsNodeShown(c: &mut Ctx, mut p_obj_1: u32, mut p_node_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_IsNodeShown", &PROBE_N, &[(p_obj_1) as u32, (p_node_2) as u32], false); }
    let mut l_shown_3: i32 = 0;
    let r_ = 'f: {
    l_shown_3 = 1i32;
    if (((({ let a_ = (c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(40))).wrapping_add(12); c.m.r32(a_) }) & (((65536i32) as u32)))) != 0) {
        if (((((((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 1, 1) as u32)) as i32)) != 0) && (((((rt::bf_get_u(c, ((p_obj_1).wrapping_add(2624)).wrapping_add(1), 0, 1) as u32)) as i32)) != 0)) as i32)) != 0) {
            if (((((((((p_node_2) >= (35i32)) as i32)) != 0) && (((((p_node_2) <= (44i32)) as i32)) != 0)) as i32)) != 0) {
                l_shown_3 = 0i32;
            } else {
                if (((((p_node_2) == (64i32)) as i32)) != 0) {
                    l_shown_3 = 0i32;
                } else {
                    if (((((p_node_2) == (65i32)) as i32)) != 0) {
                        l_shown_3 = 0i32;
                    } else {
                        if (((((p_node_2) == (66i32)) as i32)) != 0) {
                            l_shown_3 = 0i32;
                        } else {
                            if (((((p_node_2) == (67i32)) as i32)) != 0) {
                                l_shown_3 = 0i32;
                            }
                        }
                    }
                }
            }
        } else {
            if (((((((((p_node_2) >= (21i32)) as i32)) != 0) && (((((p_node_2) <= (30i32)) as i32)) != 0)) as i32)) != 0) {
                if (((((((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 1, 1) as u32)) as i32)) != 0) && (((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 6, 1) as u32)) as i32)) != 0)) as i32)) != 0) {
                    l_shown_3 = 0i32;
                }
            } else {
                if (((((((((p_node_2) >= (35i32)) as i32)) != 0) && (((((p_node_2) <= (44i32)) as i32)) != 0)) as i32)) != 0) {
                    if (((((((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 1, 1) as u32)) as i32)) != 0) && (((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 5, 1) as u32)) as i32)) != 0)) as i32)) != 0) {
                        l_shown_3 = 0i32;
                    }
                } else {
                    if (((((p_node_2) == (66i32)) as i32)) != 0) {
                        if ((((!(((((((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 1, 1) as u32)) as i32)) != 0) && (((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 5, 1) as u32)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
                            l_shown_3 = 0i32;
                        }
                    } else {
                        if (((((p_node_2) == (67i32)) as i32)) != 0) {
                            if ((((!(((((((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 1, 1) as u32)) as i32)) != 0) && (((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 6, 1) as u32)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
                                l_shown_3 = 0i32;
                            }
                        }
                    }
                }
            }
        }
    } else {
        if (((({ let a_ = (c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(40))).wrapping_add(12); c.m.r32(a_) }) & (((131072i32) as u32)))) != 0) {
            if (((((((((p_node_2) >= (35i32)) as i32)) != 0) && (((((p_node_2) <= (44i32)) as i32)) != 0)) as i32)) != 0) {
                if (((((((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 1, 1) as u32)) as i32)) != 0) && (((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 5, 1) as u32)) as i32)) != 0)) as i32)) != 0) {
                    l_shown_3 = 0i32;
                }
            } else {
                if (((((p_node_2) == (64i32)) as i32)) != 0) {
                    if ((((!(((((((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 1, 1) as u32)) as i32)) != 0) && (((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 5, 1) as u32)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
                        l_shown_3 = 0i32;
                    }
                }
            }
        } else {
            if (((({ let a_ = (c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(40))).wrapping_add(12); c.m.r32(a_) }) & (((262144i32) as u32)))) != 0) {
                if (((((((((p_node_2) >= (35i32)) as i32)) != 0) && (((((p_node_2) <= (44i32)) as i32)) != 0)) as i32)) != 0) {
                    if (((((((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 1, 1) as u32)) as i32)) != 0) && (((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 6, 1) as u32)) as i32)) != 0)) as i32)) != 0) {
                        l_shown_3 = 0i32;
                    }
                } else {
                    if (((((((((p_node_2) >= (21i32)) as i32)) != 0) && (((((p_node_2) <= (30i32)) as i32)) != 0)) as i32)) != 0) {
                        if (((((((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 1, 1) as u32)) as i32)) != 0) && (((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 5, 1) as u32)) as i32)) != 0)) as i32)) != 0) {
                            l_shown_3 = 0i32;
                        }
                    } else {
                        if (((((p_node_2) == (64i32)) as i32)) != 0) {
                            if ((((!(((((((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 1, 1) as u32)) as i32)) != 0) && (((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 5, 1) as u32)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
                                l_shown_3 = 0i32;
                            }
                        } else {
                            if (((((p_node_2) == (65i32)) as i32)) != 0) {
                                if ((((!(((((((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 1, 1) as u32)) as i32)) != 0) && (((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 6, 1) as u32)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
                                    l_shown_3 = 0i32;
                                }
                            } else {
                                if (((((p_node_2) == (66i32)) as i32)) != 0) {
                                    if (((((((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 1, 1) as u32)) as i32)) != 0) && (((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 5, 1) as u32)) as i32)) != 0)) as i32)) != 0) {
                                        l_shown_3 = 0i32;
                                    }
                                } else {
                                    if (((((p_node_2) == (67i32)) as i32)) != 0) {
                                        if (((((((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 1, 1) as u32)) as i32)) != 0) && (((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 6, 1) as u32)) as i32)) != 0)) as i32)) != 0) {
                                            l_shown_3 = 0i32;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                if (((((p_node_2) == (64i32)) as i32)) != 0) {
                    if ((((!(((((((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 1, 1) as u32)) as i32)) != 0) && (((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 5, 1) as u32)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
                        l_shown_3 = 0i32;
                    }
                } else {
                    if (((((p_node_2) == (65i32)) as i32)) != 0) {
                        if ((((!(((((((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 1, 1) as u32)) as i32)) != 0) && (((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 6, 1) as u32)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
                            l_shown_3 = 0i32;
                        }
                    } else {
                        if (((((p_node_2) == (66i32)) as i32)) != 0) {
                            if (((((((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 1, 1) as u32)) as i32)) != 0) && (((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 5, 1) as u32)) as i32)) != 0)) as i32)) != 0) {
                                l_shown_3 = 0i32;
                            }
                        } else {
                            if (((((p_node_2) == (67i32)) as i32)) != 0) {
                                if (((((((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 1, 1) as u32)) as i32)) != 0) && (((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 6, 1) as u32)) as i32)) != 0)) as i32)) != 0) {
                                    l_shown_3 = 0i32;
                                }
                            } else {
                                if (((((p_node_2) == (111i32)) as i32)) != 0) {
                                    if ((((!(((((((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 1, 1) as u32)) as i32)) != 0) && (((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 7, 1) as u32)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
                                        l_shown_3 = 0i32;
                                    }
                                } else {
                                    if (((((p_node_2) == (118i32)) as i32)) != 0) {
                                        if (((((((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 1, 1) as u32)) as i32)) != 0) && (((((rt::bf_get_u(c, ((p_obj_1).wrapping_add(2624)).wrapping_add(3), 6, 1) as u32)) as i32)) != 0)) as i32)) != 0) {
                                            l_shown_3 = 0i32;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    break 'f l_shown_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_IsNodeShown", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_GetNodeSide(c: &mut Ctx, mut p_node_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_GetNodeSide", &PROBE_N, &[(p_node_1) as u32], false); }
    let r_ = 'f: {
    if (((((p_node_1) < (8i32)) as i32)) != 0) {
        break 'f 2i32;
    }
    if (((((p_node_1) < (12i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    if (((((p_node_1) < (16i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((p_node_1) < (18i32)) as i32)) != 0) {
        break 'f 2i32;
    }
    if (((((p_node_1) < (32i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    if (((((p_node_1) < (46i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((p_node_1) < (53i32)) as i32)) != 0) {
        break 'f 2i32;
    }
    break 'f 3i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_GetNodeSide", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_SaveNodePositions(c: &mut Ctx, mut p_obj_1: u32, mut p_relative_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_SaveNodePositions", &PROBE_N, &[(p_obj_1) as u32, (p_relative_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_bound_3: u32 = 0;
    let mut l_pose_4: u32 = 0;
    let mut l_node_5: u32 = 0;
    let r_ = 'f: {
        l_bound_3 = c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(44));
        l_pose_4 = (p_obj_1).wrapping_add(2384);
        if ((p_relative_2) != 0) {
            { let a1_ = (F::BtlObj_GetNode(c, p_obj_1, 0i32)).wrapping_add(144); F::Vec4_Copy(c, fp, a1_) };
        } else {
            F::Vec4_SetZero(c, fp);
        }
        'l1: while ((1i32) != 0) {
            l_node_5 = { let a1_ = ((c.m.r16((l_bound_3).wrapping_add(10))) as i32); F::BtlObj_GetNode(c, p_obj_1, a1_) };
            F::Mtx_MulVec4(c, (l_node_5).wrapping_add(208), (l_pose_4).wrapping_add(144), ((l_node_5).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
            F::Vec4_Sub(c, (l_node_5).wrapping_add(208), (l_node_5).wrapping_add(208), fp);
            if (((((((c.m.r16((l_bound_3).wrapping_add(6))) as i32)) != (0i32)) as i32)) != 0) {
                break 'l1;
            }
            l_bound_3 = (l_bound_3).wrapping_add((((c.m.r32(l_bound_3) as i32)) as u32));
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_SaveNodePositions", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_GetNodeVelocity(c: &mut Ctx, mut p_obj_1: u32, mut p_node_2: i32, mut p_out_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_GetNodeVelocity", &PROBE_N, &[(p_obj_1) as u32, (p_node_2) as u32, (p_out_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_n_4: u32 = 0;
    let r_ = 'f: {
        l_n_4 = F::BtlObj_GetNode(c, p_obj_1, p_node_2);
        if (((((l_n_4) == (((0i32) as u32))) as i32)) != 0) {
            F::Vec4_SetZero(c, p_out_3);
            break 'f;
        }
        F::Mtx_MulVec4(c, fp, ((p_obj_1).wrapping_add(2384)).wrapping_add(144), ((l_n_4).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
        F::Vec4_Sub(c, p_out_3, fp, (l_n_4).wrapping_add(208));
        { let v_ = F32(0x00000000); let a_ = (p_out_3).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_GetNodeVelocity", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_GetNodeByCode(c: &mut Ctx, mut p_obj_1: u32, mut p_code_2: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_GetNodeByCode", &PROBE_N, &[(p_obj_1) as u32, (p_code_2) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv3: u32 = p_code_2;
        match sv3 {
            0u32 => {
                break 'f F::BtlObj_GetNode(c, p_obj_1, 3i32);
            }
            1u32 => {
                break 'f F::BtlObj_GetNode(c, p_obj_1, 7i32);
            }
            2u32 => {
                break 'f F::BtlObj_GetNode(c, p_obj_1, 10i32);
            }
            3u32 => {
                break 'f F::BtlObj_GetNode(c, p_obj_1, 11i32);
            }
            4u32 => {
                break 'f F::BtlObj_GetNode(c, p_obj_1, 14i32);
            }
            5u32 => {
                break 'f F::BtlObj_GetNode(c, p_obj_1, 15i32);
            }
            6u32 => {
                break 'f F::BtlObj_GetNode(c, p_obj_1, 19i32);
            }
            7u32 => {
                break 'f F::BtlObj_GetNode(c, p_obj_1, 20i32);
            }
            8u32 => {
                break 'f F::BtlObj_GetNode(c, p_obj_1, 21i32);
            }
            9u32 => {
                break 'f F::BtlObj_GetNode(c, p_obj_1, 33i32);
            }
            10u32 => {
                break 'f F::BtlObj_GetNode(c, p_obj_1, 34i32);
            }
            11u32 => {
                break 'f F::BtlObj_GetNode(c, p_obj_1, 35i32);
            }
            12u32 => {
                break 'f F::BtlObj_GetNode(c, p_obj_1, 46i32);
            }
            13u32 => {
                break 'f F::BtlObj_GetNode(c, p_obj_1, 64i32);
            }
            14u32 => {
                break 'f F::BtlObj_GetNode(c, p_obj_1, 65i32);
            }
            15u32 => {
                break 'f F::BtlObj_GetNode(c, p_obj_1, 0i32);
            }
            16u32 => {
                break 'f F::BtlObj_GetNode(c, p_obj_1, 31i32);
            }
            17u32 => {
                break 'f F::BtlObj_GetNode(c, p_obj_1, 45i32);
            }
            18u32 => {
                break 'f F::BtlObj_GetNode(c, p_obj_1, 54i32);
            }
            _ => {
            }
        }
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_GetNodeByCode", &PROBE_N, &[], true); }
    r_
}
