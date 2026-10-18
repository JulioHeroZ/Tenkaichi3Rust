#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlObjMdl_InitAlphaSlots(c: &mut Ctx, mut p_res_1: u32, mut p_mdl_2: u32, mut p_row_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjMdl_InitAlphaSlots", &PROBE_N, &[(p_res_1) as u32, (p_mdl_2) as u32, (p_row_3) as u32], false); }
    let mut l_model_4: u32 = 0;
    let mut l_tex_5: u32 = 0;
    let mut l_first_6: i32 = 0;
    let mut l_count_7: i32 = 0;
    let mut l_skip_8: i32 = 0;
    let mut l_i_9: i32 = 0;
    let mut l_j_10: i32 = 0;
    let mut l_e_11: u32 = 0;
    let mut l_alpha_12: i32 = 0;
    let mut l_slot_13: i32 = 0;
    let mut l_p_14: u32 = 0;
    let r_ = 'f: {
    l_model_4 = c.m.r32((p_mdl_2).wrapping_add(40));
    l_tex_5 = c.m.r32((p_mdl_2).wrapping_add(48));
    { let r_ = 15i32; let n_ = (p_row_3).wrapping_mul(r_); p_row_3 = n_; };
    l_skip_8 = ((c.m.r8((l_model_4).wrapping_add(84))) as i32);
    l_first_6 = ((c.m.r8((l_model_4).wrapping_add(78))) as i32);
    l_count_7 = ((c.m.r8((l_model_4).wrapping_add(79))) as i32);
    if (((((l_skip_8) == (0i32)) as i32)) != 0) {
        l_skip_8 = (1i32).wrapping_neg();
    }
    { let _ = F::memset(c, (l_model_4).wrapping_add(44), 0i32, ((16i32) as u32)); };
    {
        l_i_9 = 0i32;
        'l1: loop {
            if !(((((((l_i_9) as u32)) < (c.m.r32(l_tex_5))) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((((((l_i_9) >= (l_first_6)) as i32)) != 0) && (((((l_i_9) < ((l_first_6).wrapping_add(l_count_7))) as i32)) != 0)) as i32)) != 0) {
                    l_e_11 = (c.m.r32((l_tex_5).wrapping_add(16))).wrapping_add((((l_i_9) as u32)).wrapping_mul(64));
                    l_alpha_12 = ((c.m.r8(((l_e_11).wrapping_add(40)).wrapping_add(((5i32) as u32)))) as i32);
                    l_slot_13 = ((c.m.r8(((l_e_11).wrapping_add(40)).wrapping_add(((4i32) as u32)))) as i32);
                    if (((((l_alpha_12) > (128i32)) as i32)) != 0) {
                        l_alpha_12 = 127i32;
                    }
                    if (((((l_skip_8) == (l_i_9)) as i32)) != 0) {
                        l_alpha_12 = 0i32;
                    } else {
                        l_p_14 = c.m.r32((l_e_11).wrapping_add(60));
                        {
                            l_j_10 = 15i32;
                            'l3: loop {
                                if !(((((l_j_10) >= (0i32)) as i32)) != 0) { break 'l3; }
                                'c4: {
                                    { let v_ = (((l_slot_13).wrapping_add(p_row_3)) as u8); let a_ = (l_p_14).wrapping_add(((((96i32).wrapping_add((l_j_10).wrapping_mul(4i32))).wrapping_add(3i32)) as u32)); c.m.w8(a_, v_); };
                                }
                                { let o_ = l_j_10; let n_ = (o_).wrapping_sub(1); l_j_10 = n_; };
                            }
                        }
                    }
                    { let v_ = ((l_alpha_12) as u8); let a_ = ((c.m.r32((p_mdl_2).wrapping_add(40))).wrapping_add(44)).wrapping_add(((l_slot_13) as u32)); c.m.w8(a_, v_); };
                }
            }
            { let o_ = l_i_9; let n_ = (o_).wrapping_add(1); l_i_9 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjMdl_InitAlphaSlots", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjMdl_BindFiles(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjMdl_BindFiles", &PROBE_N, &[(p_obj_1) as u32], false); }
    let mut l_mdl_2: u32 = 0;
    let mut l_res_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_mdl_2 = (p_obj_1).wrapping_add(24);
    l_res_3 = c.m.r32((p_obj_1).wrapping_add(20));
    { let v_ = c.m.r32(l_mdl_2); let a_ = (l_mdl_2).wrapping_add(40); c.m.w32(a_, v_); };
    { let v_ = (c.m.r32(l_mdl_2)).wrapping_add((({ let a_ = (c.m.r32(l_mdl_2)).wrapping_add(108); (c.m.r32(a_) as i32) }) as u32)); let a_ = (l_mdl_2).wrapping_add(44); c.m.w32(a_, v_); };
    if ((((!(({ let a_ = (c.m.r32(l_mdl_2)).wrapping_add(15); (rt::bf_get_u(c, a_, 7, 1) as u32) }) != 0)) as i32)) != 0) {
        { let v_ = 13440i32; let a_ = (c.m.r32(l_mdl_2)).wrapping_add(88); c.m.w32(a_, (v_) as u32); };
        { let v_ = 15360i32; let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(92); c.m.w32(a_, (v_) as u32); };
        { let v_ = 15616i32; let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(96); c.m.w32(a_, (v_) as u32); };
        { let v_ = 15680i32; let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(100); c.m.w32(a_, (v_) as u32); };
        { let v_ = 15936i32; let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(104); c.m.w32(a_, (v_) as u32); };
        { let v_ = F::BtlObjVis_AllocFreeBit(c); let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        { let a1_ = c.m.r32((l_mdl_2).wrapping_add(44)); let a2_ = { let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(88); (c.m.r32(a_) as i32) }; let a3_ = { let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(92); (c.m.r32(a_) as i32) }; let a5_ = { let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(96); (c.m.r32(a_) as i32) }; F::MdlTex_RebaseChain(c, 0i32, a1_, a2_, a3_, 15680i32, a5_, ((2147483647i32) as u32), ((2147483647i32) as u32)) };
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < (8i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((c.m.r32((((p_obj_1).wrapping_add(24)).wrapping_add(68)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
                        { let a1_ = c.m.r32((((p_obj_1).wrapping_add(24)).wrapping_add(68)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))); let a2_ = { let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(88); (c.m.r32(a_) as i32) }; let a3_ = { let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(92); (c.m.r32(a_) as i32) }; let a5_ = { let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(96); (c.m.r32(a_) as i32) }; F::MdlTex_RebaseChain(c, 1i32, a1_, a2_, a3_, 15680i32, a5_, ((0i32) as u32), ((0i32) as u32)) };
                    }
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        { let a1_ = c.m.r32((l_mdl_2).wrapping_add(4)); let a2_ = c.m.r32((l_mdl_2).wrapping_add(4)); F::Res_RelocateOffsets(c, (l_mdl_2).wrapping_add(48), a1_, a2_) };
        { let a2_ = { let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(40); (c.m.r32(a_) as i32) }; F::BtlObjMdl_InitAlphaSlots(c, l_res_3, l_mdl_2, a2_) };
        { let v_ = ((1i32) as u32); let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(15); rt::bf_set(c, a_, 7, 1, (v_) as u64);  };
    } else {
        { let a1_ = c.m.r32((l_mdl_2).wrapping_add(4)); let a2_ = c.m.r32((l_mdl_2).wrapping_add(4)); F::Res_RelocateOffsets(c, ((p_obj_1).wrapping_add(24)).wrapping_add(48), a1_, a2_) };
        { let _ = { let a0_ = { let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(40); (c.m.r32(a_) as i32) }; F::BtlObjVis_AllocBit(c, a0_) }; };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjMdl_BindFiles", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjMdl_AllocParts(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjMdl_AllocParts", &PROBE_N, &[(p_obj_1) as u32], false); }
    let mut l_list_2: u32 = 0;
    let mut l_mesh_3: u32 = 0;
    let mut l_part_4: u32 = 0;
    let mut l_slot_5: u32 = 0;
    let r_ = 'f: {
    l_list_2 = (p_obj_1).wrapping_add(3424);
    l_mesh_3 = c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(44));
    {
        'l1: loop {
            'c2: {
                l_part_4 = F::BtlObjPoolE0_Alloc(c);
                { let v_ = ((c.m.r16((l_mesh_3).wrapping_add(10))) as i32); let a_ = (l_part_4).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_part_4).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = ((0i32) as u8); let a_ = (l_part_4).wrapping_add(12); c.m.w8(a_, v_); };
                F::SList_PushBack(c, l_list_2, l_part_4);
                l_slot_5 = ((p_obj_1).wrapping_add(3424)).wrapping_add(12);
                { let r_ = (c.m.r32((l_part_4).wrapping_add(8)) as i32); let n_ = (l_slot_5).wrapping_add((((r_) as u32)).wrapping_mul(4)); l_slot_5 = n_; };
                if (((((c.m.r32(l_slot_5)) == (((0i32) as u32))) as i32)) != 0) {
                    { let v_ = l_part_4; let a_ = l_slot_5; c.m.w32(a_, v_); };
                }
                if (((((((c.m.r16((l_mesh_3).wrapping_add(6))) as i32)) != (0i32)) as i32)) != 0) {
                    break 'l1;
                }
                l_mesh_3 = (l_mesh_3).wrapping_add((((c.m.r32(l_mesh_3) as i32)) as u32));
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjMdl_AllocParts", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjMdl_FreeParts(c: &mut Ctx, mut p_obj_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjMdl_FreeParts", &PROBE_N, &[(p_obj_1) as u32], false); }
    let r_ = 'f: {
    break 'f F::BtlObjPoolE0_FreeList(c, (p_obj_1).wrapping_add(3424));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjMdl_FreeParts", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjMdl_Init(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjMdl_Init", &PROBE_N, &[(p_obj_1) as u32], false); }
    let r_ = 'f: {
    F::BtlObj_BindTables(c, p_obj_1);
    F::BtlObjMdl_BindFiles(c, p_obj_1);
    F::BtlObjMdl_AllocParts(c, p_obj_1);
    F::BtlObj_InitFlags(c, p_obj_1);
    F::BtlObjXf_Reset(c, p_obj_1);
    F::BtlObjBody_Init(c, p_obj_1);
    F::BtlObj_CreateFace(c, p_obj_1);
    F::ObjShadow_Alloc(c, p_obj_1);
    F::BtlObjAnim_Init(c, p_obj_1);
    { let v_ = 1i32; let a_ = (p_obj_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjMdl_Init", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjMdl_Create(c: &mut Ctx, mut p_type_1: i32, mut p_obj_2: u32, mut p_res_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjMdl_Create", &PROBE_N, &[(p_type_1) as u32, (p_obj_2) as u32, (p_res_3) as u32], false); }
    let r_ = 'f: {
    { let _ = F::memset(c, p_obj_2, 0i32, 5744u32); };
    { let v_ = p_type_1; let a_ = p_obj_2; c.m.w32(a_, (v_) as u32); };
    { let v_ = p_res_3; let a_ = (p_obj_2).wrapping_add(20); c.m.w32(a_, v_); };
    F::BtlObjMdl_Init(c, p_obj_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjMdl_Create", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjMdl_Destroy(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjMdl_Destroy", &PROBE_N, &[(p_obj_1) as u32], false); }
    let r_ = 'f: {
    if ((((((c.m.r32((p_obj_1).wrapping_add(4)) as i32)) != (0i32)) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(40))).wrapping_add(40); (c.m.r32(a_) as i32) }; F::BtlObjVis_FreeBit(c, a0_) };
        { let _ = F::BtlObjMdl_FreeParts(c, p_obj_1); };
        F::BtlObj_FreeFace(c, p_obj_1);
        F::ObjShadow_Free(c, p_obj_1);
        { let _ = F::memset(c, p_obj_1, 0i32, 5744u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjMdl_Destroy", &PROBE_N, &[], true); }
    r_
}

pub fn ObjShadow_CalcPacketWords(c: &mut Ctx, mut p_count_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_CalcPacketWords", &PROBE_N, &[(p_count_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let mut l_words_3: i32 = 0;
    let mut l_j_4: i32 = 0;
    let mut l_pad_5: i32 = 0;
    let mut l_k_6: i32 = 0;
    let r_ = 'f: {
    l_i_2 = 0i32;
    l_words_3 = 8i32;
    'l1: loop {
        'c2: {
            { let r_ = 14i32; let n_ = (l_words_3).wrapping_add(r_); l_words_3 = n_; };
            l_j_4 = 0i32;
            'l3: loop {
                'c4: {
                    { let r_ = 48i32; let n_ = (l_words_3).wrapping_add(r_); l_words_3 = n_; };
                    { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                    { let o_ = l_j_4; let n_ = (o_).wrapping_add(1); l_j_4 = n_; };
                }
                if !(((((((((l_i_2) < (p_count_1)) as i32)) != 0) && (((((l_j_4) < (18i32)) as i32)) != 0)) as i32)) != 0) { break 'l3; }
            }
            { let r_ = 1i32; let n_ = (l_words_3).wrapping_add(r_); l_words_3 = n_; };
        }
        if !(((((l_i_2) < (p_count_1)) as i32)) != 0) { break 'l1; }
    }
    l_pad_5 = rt::rem_i32(l_words_3, 4i32);
    if (((((l_pad_5) != (0i32)) as i32)) != 0) {
        {
            l_k_6 = 0i32;
            'l5: loop {
                if !(((((l_k_6) < ((4i32).wrapping_sub(l_pad_5))) as i32)) != 0) { break 'l5; }
                'c6: {
                    { let o_ = l_words_3; let n_ = (o_).wrapping_add(1); l_words_3 = n_; };
                }
                { let o_ = l_k_6; let n_ = (o_).wrapping_add(1); l_k_6 = n_; };
            }
        }
    }
    break 'f (l_words_3).wrapping_add(4i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_CalcPacketWords", &PROBE_N, &[], true); }
    r_
}

pub fn ObjShadow_BuildPacket(c: &mut Ctx, mut p_pkt_1: u32, mut p_count_2: i32, mut p_tris_3: u32, mut p_color_4: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_BuildPacket", &PROBE_N, &[(p_pkt_1) as u32, (p_count_2) as u32, (p_tris_3) as u32, (p_color_4) as u32], false); }
    let mut l_p_5: u32 = 0;
    let mut l_f_6: u32 = 0;
    let mut l_tag_7: u32 = 0;
    let mut l_i_8: i32 = 0;
    let mut l_j_9: i32 = 0;
    let mut l_k_10: i32 = 0;
    let mut l_pad_11: i32 = 0;
    let mut l_n_12: i32 = 0;
    let mut l_ret_13: i32 = 0;
    let r_ = 'f: {
    { let v_ = ((1610612736i32) as u32); let a_ = (p_pkt_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((1812037632i32) as u32); let a_ = (p_pkt_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((808370176i32) as u32); let a_ = (p_pkt_1).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((32768i32) as u32); let a_ = (p_pkt_1).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((1042i32) as u32); let a_ = (p_pkt_1).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((385875968i32) as u32); let a_ = (p_pkt_1).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (p_pkt_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (p_pkt_1).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    l_p_5 = (p_pkt_1).wrapping_add((((8i32) as u32)).wrapping_mul(4));
    l_i_8 = 0i32;
    'l1: loop {
        'c2: {
            { let v_ = ((1812168704i32) as u32); let a_ = { let o_ = l_p_5; let n_ = (o_).wrapping_add(4); l_p_5 = n_; o_ }; c.m.w32(a_, v_); };
            { let v_ = ((32769i32) as u32); let a_ = { let o_ = l_p_5; let n_ = (o_).wrapping_add(4); l_p_5 = n_; o_ }; c.m.w32(a_, v_); };
            { let v_ = ((268435456i32) as u32); let a_ = { let o_ = l_p_5; let n_ = (o_).wrapping_add(4); l_p_5 = n_; o_ }; c.m.w32(a_, v_); };
            { let v_ = ((6i32) as u32); let a_ = { let o_ = l_p_5; let n_ = (o_).wrapping_add(4); l_p_5 = n_; o_ }; c.m.w32(a_, v_); };
            { let v_ = ((0i32) as u32); let a_ = { let o_ = l_p_5; let n_ = (o_).wrapping_add(4); l_p_5 = n_; o_ }; c.m.w32(a_, v_); };
            { let v_ = ((0i32) as u32); let a_ = { let o_ = l_p_5; let n_ = (o_).wrapping_add(4); l_p_5 = n_; o_ }; c.m.w32(a_, v_); };
            { let v_ = ((0i32) as u32); let a_ = { let o_ = l_p_5; let n_ = (o_).wrapping_add(4); l_p_5 = n_; o_ }; c.m.w32(a_, v_); };
            { let v_ = ((0i32) as u32); let a_ = { let o_ = l_p_5; let n_ = (o_).wrapping_add(4); l_p_5 = n_; o_ }; c.m.w32(a_, v_); };
            { let v_ = ((0i32) as u32); let a_ = { let o_ = l_p_5; let n_ = (o_).wrapping_add(4); l_p_5 = n_; o_ }; c.m.w32(a_, v_); };
            l_tag_7 = l_p_5;
            { let v_ = ((0i32) as u32); let a_ = { let o_ = l_p_5; let n_ = (o_).wrapping_add(4); l_p_5 = n_; o_ }; c.m.w32(a_, v_); };
            { let v_ = ((808337408i32) as u32); let a_ = { let o_ = l_p_5; let n_ = (o_).wrapping_add(4); l_p_5 = n_; o_ }; c.m.w32(a_, v_); };
            { let v_ = ((1042i32) as u32); let a_ = { let o_ = l_p_5; let n_ = (o_).wrapping_add(4); l_p_5 = n_; o_ }; c.m.w32(a_, v_); };
            { let v_ = ((0i32) as u32); let a_ = { let o_ = l_p_5; let n_ = (o_).wrapping_add(4); l_p_5 = n_; o_ }; c.m.w32(a_, v_); };
            { let v_ = ((0i32) as u32); let a_ = { let o_ = l_p_5; let n_ = (o_).wrapping_add(4); l_p_5 = n_; o_ }; c.m.w32(a_, v_); };
            l_f_6 = l_p_5;
            l_j_9 = 0i32;
            'l3: loop {
                'c4: {
                    {
                        l_k_10 = 0i32;
                        'l5: loop {
                            if !(((((l_k_10) < (3i32)) as i32)) != 0) { break 'l5; }
                            'c6: {
                                { let v_ = rt::fadd(F32(c.m.r32(((p_tris_3).wrapping_add((((l_i_8) as u32)).wrapping_mul(64))).wrapping_add((((l_k_10) as u32)).wrapping_mul(16)))), rt::fmul(F32(c.m.r32(((p_tris_3).wrapping_add((((l_i_8) as u32)).wrapping_mul(64))).wrapping_add(48))), F32(0x3ca3d70a))); let a_ = { let o_ = l_f_6; let n_ = (o_).wrapping_add(4); l_f_6 = n_; o_ }; c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fadd(F32(c.m.r32((((p_tris_3).wrapping_add((((l_i_8) as u32)).wrapping_mul(64))).wrapping_add((((l_k_10) as u32)).wrapping_mul(16))).wrapping_add(4))), rt::fmul(F32(c.m.r32((((p_tris_3).wrapping_add((((l_i_8) as u32)).wrapping_mul(64))).wrapping_add(48)).wrapping_add(4))), F32(0x3ca3d70a))); let a_ = { let o_ = l_f_6; let n_ = (o_).wrapping_add(4); l_f_6 = n_; o_ }; c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fadd(F32(c.m.r32((((p_tris_3).wrapping_add((((l_i_8) as u32)).wrapping_mul(64))).wrapping_add((((l_k_10) as u32)).wrapping_mul(16))).wrapping_add(8))), rt::fmul(F32(c.m.r32((((p_tris_3).wrapping_add((((l_i_8) as u32)).wrapping_mul(64))).wrapping_add(48)).wrapping_add(8))), F32(0x3ca3d70a))); let a_ = { let o_ = l_f_6; let n_ = (o_).wrapping_add(4); l_f_6 = n_; o_ }; c.m.w32(a_, (v_).0); };
                                l_p_5 = l_f_6;
                                if (((((l_k_10) < (2i32)) as i32)) != 0) {
                                    if (((((l_j_9) == (0i32)) as i32)) != 0) {
                                        { let v_ = ((0i32) as u32); let a_ = { let o_ = l_p_5; let n_ = (o_).wrapping_add(4); l_p_5 = n_; o_ }; c.m.w32(a_, v_); };
                                        l_f_6 = l_p_5;
                                        { let v_ = F32(c.m.r32(((p_tris_3).wrapping_add((((l_i_8) as u32)).wrapping_mul(64))).wrapping_add(48))); let a_ = (l_f_6).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                        { let v_ = F32(c.m.r32((((p_tris_3).wrapping_add((((l_i_8) as u32)).wrapping_mul(64))).wrapping_add(48)).wrapping_add(4))); let a_ = (l_f_6).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                    } else {
                                        { let v_ = ((1i32) as u32); let a_ = { let o_ = l_p_5; let n_ = (o_).wrapping_add(4); l_p_5 = n_; o_ }; c.m.w32(a_, v_); };
                                        l_f_6 = l_p_5;
                                        { let v_ = F32(c.m.r32(((p_tris_3).wrapping_add((((l_i_8) as u32)).wrapping_mul(64))).wrapping_add(48))); let a_ = (l_f_6).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                        { let v_ = F32(c.m.r32((((p_tris_3).wrapping_add((((l_i_8) as u32)).wrapping_mul(64))).wrapping_add(48)).wrapping_add(4))); let a_ = (l_f_6).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                    }
                                } else {
                                    { let v_ = ((0i32) as u32); let a_ = { let o_ = l_p_5; let n_ = (o_).wrapping_add(4); l_p_5 = n_; o_ }; c.m.w32(a_, v_); };
                                    l_f_6 = l_p_5;
                                    { let v_ = F32(c.m.r32(((p_tris_3).wrapping_add((((l_i_8) as u32)).wrapping_mul(64))).wrapping_add(48))); let a_ = (l_f_6).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                    { let v_ = F32(c.m.r32((((p_tris_3).wrapping_add((((l_i_8) as u32)).wrapping_mul(64))).wrapping_add(48)).wrapping_add(4))); let a_ = (l_f_6).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                }
                                { let v_ = F32(c.m.r32((((p_tris_3).wrapping_add((((l_i_8) as u32)).wrapping_mul(64))).wrapping_add(48)).wrapping_add(8))); let a_ = (l_f_6).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = (l_f_6).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(c.m.r32((p_color_4).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (l_f_6).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(c.m.r32((p_color_4).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (l_f_6).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(c.m.r32((p_color_4).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (l_f_6).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(c.m.r32((p_color_4).wrapping_add((((3i32) as u32)).wrapping_mul(4)))); let a_ = (l_f_6).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x00000000); let a_ = (l_f_6).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x00000000); let a_ = (l_f_6).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = (l_f_6).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x00000000); let a_ = (l_f_6).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let r_ = 12i32; let n_ = (l_f_6).wrapping_add((((r_) as u32)).wrapping_mul(4)); l_f_6 = n_; };
                            }
                            { let o_ = l_k_10; let n_ = (o_).wrapping_add(1); l_k_10 = n_; };
                        }
                    }
                    { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
                    { let o_ = l_j_9; let n_ = (o_).wrapping_add(1); l_j_9 = n_; };
                }
                if !(((((((((l_i_8) < (p_count_2)) as i32)) != 0) && (((((l_j_9) < (18i32)) as i32)) != 0)) as i32)) != 0) { break 'l3; }
            }
            l_p_5 = l_f_6;
            l_n_12 = (l_j_9).wrapping_mul(3i32);
            { let v_ = ((((l_n_12) | (32768i32))) as u32); let a_ = (l_tag_7).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            { let v_ = (((((l_n_12).wrapping_shl((18i32) as u32)) | (1811972099i32))) as u32); let a_ = (l_tag_7).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            { let v_ = ((385875968i32) as u32); let a_ = { let o_ = l_p_5; let n_ = (o_).wrapping_add(4); l_p_5 = n_; o_ }; c.m.w32(a_, v_); };
        }
        if !(((((l_i_8) < (p_count_2)) as i32)) != 0) { break 'l1; }
    }
    l_n_12 = (((l_p_5).wrapping_sub(p_pkt_1) as i32)).wrapping_div(4);
    l_pad_11 = rt::rem_i32(l_n_12, 4i32);
    if (((((l_pad_11) != (0i32)) as i32)) != 0) {
        l_pad_11 = (4i32).wrapping_sub(l_pad_11);
        {
            l_k_10 = 0i32;
            'l7: loop {
                if !(((((l_k_10) < (l_pad_11)) as i32)) != 0) { break 'l7; }
                'c8: {
                    { let v_ = ((0i32) as u32); let a_ = { let o_ = l_p_5; let n_ = (o_).wrapping_add(4); l_p_5 = n_; o_ }; c.m.w32(a_, v_); };
                }
                { let o_ = l_k_10; let n_ = (o_).wrapping_add(1); l_k_10 = n_; };
            }
        }
    }
    l_ret_13 = 1i32;
    { let v_ = ((1879048192i32) as u32); let a_ = { let o_ = l_p_5; let n_ = (o_).wrapping_add(4); l_p_5 = n_; o_ }; c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = { let o_ = l_p_5; let n_ = (o_).wrapping_add(4); l_p_5 = n_; o_ }; c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = { let o_ = l_p_5; let n_ = (o_).wrapping_add(4); l_p_5 = n_; o_ }; c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = { let o_ = l_p_5; let n_ = (o_).wrapping_add(4); l_p_5 = n_; o_ }; c.m.w32(a_, v_); };
    { let v_ = (((rt::div_i32((((l_p_5).wrapping_sub(p_pkt_1) as i32)).wrapping_div(4), 4i32)).wrapping_add(1610612734i32)) as u32); let a_ = (p_pkt_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    break 'f l_ret_13;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_BuildPacket", &PROBE_N, &[], true); }
    r_
}

pub fn ObjShadow_MakePlaneMtx(c: &mut Ctx, mut p_out_1: u32, mut p_plane_2: u32, mut p_light_3: u32, mut p_bias_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_MakePlaneMtx", &PROBE_N, &[(p_out_1) as u32, (p_plane_2) as u32, (p_light_3) as u32, (p_bias_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_lx_5: F32 = F32(0);
    let mut l_ly_6: F32 = F32(0);
    let mut l_lz_7: F32 = F32(0);
    let mut l_a_8: F32 = F32(0);
    let mut l_b_9: F32 = F32(0);
    let mut l_c_10: F32 = F32(0);
    let mut l_d_11: F32 = F32(0);
    let r_ = 'f: {
        F::Vec4_Copy(c, fp, p_light_3);
        l_lx_5 = rt::fneg(F32(c.m.r32(fp)));
        l_ly_6 = rt::fneg(F32(c.m.r32((fp).wrapping_add(4))));
        l_lz_7 = rt::fneg(F32(c.m.r32((fp).wrapping_add(8))));
        l_a_8 = F32(c.m.r32(p_plane_2));
        l_b_9 = F32(c.m.r32((p_plane_2).wrapping_add(4)));
        l_c_10 = F32(c.m.r32((p_plane_2).wrapping_add(8)));
        l_d_11 = rt::fneg(rt::fadd(F32(c.m.r32((p_plane_2).wrapping_add(12))), p_bias_4));
        { let v_ = rt::fadd(rt::fmul(l_b_9, l_ly_6), rt::fmul(l_c_10, l_lz_7)); let a_ = ((p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fneg(l_a_8), l_ly_6); let a_ = ((p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fneg(l_a_8), l_lz_7); let a_ = ((p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fneg(l_b_9), l_lx_5); let a_ = ((p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(rt::fmul(l_a_8, l_lx_5), rt::fmul(l_c_10, l_lz_7)); let a_ = ((p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fneg(l_b_9), l_lz_7); let a_ = ((p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fneg(l_c_10), l_lx_5); let a_ = ((p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fneg(l_c_10), l_ly_6); let a_ = ((p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(rt::fmul(l_a_8, l_lx_5), rt::fmul(l_b_9, l_ly_6)); let a_ = ((p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(l_d_11, l_lx_5); let a_ = ((p_out_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(l_d_11, l_ly_6); let a_ = ((p_out_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(l_d_11, l_lz_7); let a_ = ((p_out_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(rt::fadd(rt::fmul(l_a_8, l_lx_5), rt::fmul(l_b_9, l_ly_6)), rt::fmul(l_c_10, l_lz_7)); let a_ = ((p_out_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_MakePlaneMtx", &PROBE_N, &[], true); }
    r_
}

pub fn ObjShadow_CalcScreenDist(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32, mut p_size_3: F32, mut p_half_4: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_CalcScreenDist", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32, (p_size_3).0, (p_half_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_dist_5: F32 = F32(0);
    let mut l_s_6: F32 = F32(0);
    let r_ = 'f: {
        F::Vec4_Sub(c, fp, p_a_1, p_b_2);
        l_dist_5 = { let a0_ = F::Vec3_Dot(c, fp, fp); F::Port_sqrtf(c, a0_) };
        l_s_6 = rt::fdiv(p_size_3, l_dist_5);
        break 'f rt::fdiv(rt::fmul(rt::fmul(l_dist_5, F::Port_sqrtf(c, rt::fsub(F32(0x3f800000), rt::fmul(l_s_6, l_s_6)))), p_half_4), rt::fadd(p_size_3, p_size_3));
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_CalcScreenDist", &PROBE_N, &[], true); }
    r_
}

pub fn ObjShadow_IsEnabled(c: &mut Ctx, mut p_obj_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_IsEnabled", &PROBE_N, &[(p_obj_1) as u32], false); }
    let r_ = 'f: {
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((8192i32) as u64)))) != 0) {
        break 'f 0i32;
    }
    if ((((c.m.r32((p_obj_1).wrapping_add(2624))) & (((268435456i32) as u32)))) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_IsEnabled", &PROBE_N, &[], true); }
    r_
}

pub fn ObjShadow_IsFlat(c: &mut Ctx, mut p_obj_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_IsFlat", &PROBE_N, &[(p_obj_1) as u32], false); }
    let r_ = 'f: {
    if ((((c.m.r32((p_obj_1).wrapping_add(2624))) & (((536870912i32) as u32)))) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_IsFlat", &PROBE_N, &[], true); }
    r_
}

pub fn ObjShadow_AddTargetEnv(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_AddTargetEnv", &PROBE_N, &[], false); }
    let mut l_p_1: u32 = 0;
    let mut l_tag_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_x_4: i32 = 0;
    let r_ = 'f: {
    l_p_1 = F::Dma_BeginDirect(c);
    { let v_ = ((((((((12i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((((((((0i32) as u64)) | ((((255i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) | ((((255i32) as u64)).wrapping_shl((48i32) as u32))); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((64i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((((((((336i32) as u64)) | ((((4i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((76i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((78i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((((30720i32) as u64)) | ((((30720i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((24i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((5i32) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((8i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((1i32) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((70i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((0i32) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((74i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((96i32) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((20i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((196608i32) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((71i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = (((((128i32) as u64)).wrapping_shl((32i32) as u32)) | (((100i32) as u64))); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((66i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((6i32) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((0i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = (((1065353216i32) as u64)).wrapping_shl((32i32) as u32); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((1i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    l_tag_2 = l_p_1;
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    l_x_4 = 0i32;
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (8i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = ((((((l_x_4).wrapping_add(1920i32)).wrapping_shl((4i32) as u32)) | ((30720i32).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let v_ = ((((((((l_x_4).wrapping_add(1952i32)).wrapping_shl((4i32) as u32)) as i64)) as u64)) | ((((34816i32) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
                { let r_ = 32i32; let n_ = (l_x_4).wrapping_add(r_); l_x_4 = n_; };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    { let v_ = ((((((((8i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((2i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_tag_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((85i32) as u64); let a_ = (l_tag_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((((((((4i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((((((((1i32) as u64)) | ((((254i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((32i32) as u32)))) | ((((254i32) as u64)).wrapping_shl((48i32) as u32))); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((64i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((4575657223564329088i64) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((1i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((0i32) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((26i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((72i32) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((27i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    F::Dma_EndDirect(c, l_p_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_AddTargetEnv", &PROBE_N, &[], true); }
    r_
}

pub fn ObjShadow_GetBox(c: &mut Ctx, mut p_out_1: u32, mut p_obj_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_GetBox", &PROBE_N, &[(p_out_1) as u32, (p_obj_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_down_3: i32 = 0;
    let r_ = 'f: {
        l_down_3 = 0i32;
        { let s_ = (p_obj_2).wrapping_add(2804); let d_ = p_out_1; rt::memcpy(c, d_, s_, 24); };
        F::ColBox_GetCenter(c, p_out_1, fp);
        F::ColBox_GetHalf(c, p_out_1, fp.wrapping_add(16));
        if ((F::Battle_IsSplitScreen(c)) != 0) {
            { let r_ = F32(0x3f8ccccc); let a_ = fp.wrapping_add(16); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = F32(0x3f8ccccc); let a_ = (fp.wrapping_add(16)).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            l_down_3 = 1i32;
        } else {
            if (((rt::fgt(F32(c.m.r32((p_obj_2).wrapping_add(4084))), F32(0x42480000)) as i32)) != 0) {
                { let r_ = F32(0x3f8ccccc); let a_ = fp.wrapping_add(16); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = F32(0x3f8ccccc); let a_ = (fp.wrapping_add(16)).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                l_down_3 = 1i32;
            } else {
                { let r_ = F32(0x40133333); let a_ = fp.wrapping_add(16); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = F32(0x40133333); let a_ = (fp.wrapping_add(16)).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            }
        }
        F::ColBox_SetCenterHalf(c, p_out_1, fp, fp.wrapping_add(16));
        break 'f l_down_3;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_GetBox", &PROBE_N, &[], true); }
    r_
}

pub fn ObjShadow_SetTarget(c: &mut Ctx, mut p_obj_1: u32, mut p_work_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_SetTarget", &PROBE_N, &[(p_obj_1) as u32, (p_work_2) as u32], false); }
    let mut l_scale_3: F32 = F32(0);
    let mut l_xf_4: u32 = 0;
    let r_ = 'f: {
    l_scale_3 = F32(c.m.r32((p_obj_1).wrapping_add(4084)));
    l_xf_4 = (p_obj_1).wrapping_add(2384);
    if (((rt::fle(l_scale_3, F32(0x41b80000)) as i32)) != 0) {
        { let v_ = F32(0x41900000); let a_ = (p_work_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3d6147ae); let a_ = (p_work_2).wrapping_add(48); c.m.w32(a_, (v_).0); };
    } else {
        if (((rt::fle(l_scale_3, F32(0x42480000)) as i32)) != 0) {
            { let v_ = F32(0x41c332ff); let a_ = (p_work_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3d2c0831); let a_ = (p_work_2).wrapping_add(48); c.m.w32(a_, (v_).0); };
        } else {
            if (((rt::fle(l_scale_3, F32(0x428c0000)) as i32)) != 0) {
                { let v_ = F32(0x42480000); let a_ = (p_work_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3cac0831); let a_ = (p_work_2).wrapping_add(48); c.m.w32(a_, (v_).0); };
            } else {
                { let v_ = F32(0x42aa0000); let a_ = (p_work_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3c449ba5); let a_ = (p_work_2).wrapping_add(48); c.m.w32(a_, (v_).0); };
            }
        }
    }
    F::Vec4_Copy(c, (p_work_2).wrapping_add(16), (l_xf_4).wrapping_add(32));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_SetTarget", &PROBE_N, &[], true); }
    r_
}

pub fn ObjShadow_MakeLightMtx(c: &mut Ctx, mut p_obj_1: u32, mut p_work_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_MakeLightMtx", &PROBE_N, &[(p_obj_1) as u32, (p_work_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let r_ = 'f: {
        rt::memset(c, fp.wrapping_add(64), 0, 16);
        { let v_ = F32(0x3fc90fda); c.m.w32(fp.wrapping_add(64).wrapping_add(0), (v_).0); }
        { let v_ = F32(0xbfc90fda); c.m.w32(fp.wrapping_add(64).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(64).wrapping_add(12), (v_).0); }
        { let v_ = F32(c.m.r32((p_work_2).wrapping_add(16))); let a_ = (p_work_2).wrapping_add(32); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((p_work_2).wrapping_add(16)).wrapping_add(4))), F32(0x447a0000)); let a_ = ((p_work_2).wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((p_work_2).wrapping_add(16)).wrapping_add(8))); let a_ = ((p_work_2).wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((p_work_2).wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Mtx_StoreIdentity(c, fp);
        { let v_ = F32(0xbf800000); let a_ = ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        F::Vu0Cur_LoadIdentity(c);
        F::Vu0Cur_RotateZXY(c, fp.wrapping_add(64));
        F::Vu0Cur_StoreMtx(c, fp);
        { let v_ = F32(c.m.r32((p_work_2).wrapping_add(32))); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((p_work_2).wrapping_add(32)).wrapping_add(4))); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((p_work_2).wrapping_add(32)).wrapping_add(8))); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        F::Mtx_InverseRT(c, (p_work_2).wrapping_add(64), fp);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_MakeLightMtx", &PROBE_N, &[], true); }
    r_
}

pub fn ObjShadow_MakeProjMtx(c: &mut Ctx, mut p_obj_1: u32, mut p_work_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_MakeProjMtx", &PROBE_N, &[(p_obj_1) as u32, (p_work_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(176) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 176);
    let r_ = 'f: {
        { let _ = F::memset(c, fp.wrapping_add(160), 0i32, 16u32); };
        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(160)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        F::Vec3_Sub(c, fp.wrapping_add(16), (p_work_2).wrapping_add(16), (p_work_2).wrapping_add(32));
        F::Vec3_Normalize(c, fp.wrapping_add(16), fp.wrapping_add(16));
        F::Vec3_Cross(c, fp, fp.wrapping_add(16), fp.wrapping_add(160));
        F::Vec3_Normalize(c, fp, fp);
        { let a1_ = { let a2_ = F32(c.m.r32((p_work_2).wrapping_add(8))); F::ObjShadow_CalcScreenDist(c, (p_work_2).wrapping_add(16), (p_work_2).wrapping_add(32), a2_, F32(0x437e0000)) }; F::Mtx_MakeViewScreen(c, fp.wrapping_add(32), a1_, F32(0x3f800000), F32(0x3f800000), F32(0x45000000), F32(0x45000000), F32(0x3f800000), F32(0x4b25f848), F32(0x3f800000), F32(0x4b25f848)) };
        F::Mtx_MakeCamera(c, fp.wrapping_add(96), (p_work_2).wrapping_add(32), fp.wrapping_add(16), fp);
        F::Mtx_Mul(c, (p_work_2).wrapping_add(128), fp.wrapping_add(32), fp.wrapping_add(96));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_MakeProjMtx", &PROBE_N, &[], true); }
    r_
}

pub fn ObjShadow_LoadProg(c: &mut Ctx, mut p_obj_1: u32, mut p_work_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_LoadProg", &PROBE_N, &[(p_obj_1) as u32, (p_work_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(144) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 144);
    let mut l_proj_3: u32 = 0;
    let mut l_sh_4: u32 = 0;
    let mut l_prog_5: u32 = 0;
    let mut l_screen_6: u32 = 0;
    let mut l_view_7: u32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp.wrapping_add(64), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(0), (v_).0); }
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(64).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(64).wrapping_add(12), (v_).0); }
        l_proj_3 = (p_work_2).wrapping_add(128);
        l_sh_4 = c.m.r32((p_obj_1).wrapping_add(5736));
        l_prog_5 = F::Vu1Pkt_LoadProg2(c, 1i32);
        { let a1_ = (F::BtlObj_GetNode(c, p_obj_1, 0i32)).wrapping_add(16); F::Mtx_InverseRT(c, fp.wrapping_add(80), a1_) };
        l_screen_6 = (l_prog_5).wrapping_add(192);
        F::Mtx_Mul(c, l_screen_6, fp.wrapping_add(80), l_proj_3);
        { let v_ = F32(c.m.r32(((p_work_2).wrapping_add(16)).wrapping_add(4))); let a_ = (fp.wrapping_add(64)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::ObjShadow_MakePlaneMtx(c, fp, fp.wrapping_add(64), (l_sh_4).wrapping_add(8224), F32(0xbc23d70a));
        F::Mtx_Copy(c, l_screen_6, l_proj_3);
        F::Mtx_Mul(c, l_screen_6, l_screen_6, fp);
        F::Mtx_Copy(c, (l_prog_5).wrapping_add(256), l_screen_6);
        l_view_7 = c.m.r32((p_obj_1).wrapping_add(2800));
        { let v_ = F32(0x43000000); let a_ = (l_view_7).wrapping_add(48); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x43000000); let a_ = ((l_view_7).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x43000000); let a_ = ((l_view_7).wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x43000000); let a_ = ((l_view_7).wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::ObjDraw_DrawPartsFlat(c, p_obj_1);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_LoadProg", &PROBE_N, &[], true); }
    r_
}

pub fn ObjShadow_Alloc(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_Alloc", &PROBE_N, &[(p_obj_1) as u32], false); }
    let r_ = 'f: {
    if ((((c.m.r32((p_obj_1).wrapping_add(2624))) & (((268435456i32) as u32)))) != 0) {
        { let v_ = F::BtlObjPool3_Alloc(c); let a_ = (p_obj_1).wrapping_add(5736); c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_Alloc", &PROBE_N, &[], true); }
    r_
}

pub fn ObjShadow_Free(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_Free", &PROBE_N, &[(p_obj_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32((p_obj_1).wrapping_add(5736))) != (((0i32) as u32))) as i32)) != 0) {
        { let _ = { let a0_ = c.m.r32((p_obj_1).wrapping_add(5736)); F::BtlObjPool3_Free(c, a0_) }; };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_Free", &PROBE_N, &[], true); }
    r_
}

pub fn ObjShadow_ResetEnv(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_ResetEnv", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Dma_AddScissor(c, 0i32, 511i32, 0i32, 447i32);
    F::ObjGs_AddDefaultEnv(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_ResetEnv", &PROBE_N, &[], true); }
    r_
}

pub fn ObjShadow_Update(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_Update", &PROBE_N, &[(p_obj_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_sh_2: u32 = 0;
    let mut l_xf_3: u32 = 0;
    let mut l_work_4: u32 = 0;
    let r_ = 'f: {
        if ((F::ObjShadow_IsEnabled(c, p_obj_1)) != 0) {
            if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
                l_sh_2 = c.m.r32((p_obj_1).wrapping_add(5736));
                l_xf_3 = (p_obj_1).wrapping_add(2384);
                { let v_ = ((0i32) as u32); let a_ = (l_sh_2).wrapping_add(8208); c.m.w32(a_, v_); };
                { let v_ = 0i32; let a_ = (l_sh_2).wrapping_add(8212); c.m.w32(a_, (v_) as u32); };
                { let v_ = F32(0x00000000); let a_ = (l_sh_2).wrapping_add(8216); c.m.w32(a_, (v_).0); };
                l_work_4 = c.m.r32((l_sh_2).wrapping_add(8240));
                if ((F::ObjShadow_GetBox(c, fp, p_obj_1)) != 0) {
                    rt::memset(c, fp.wrapping_add(24), 0, 16);
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(24).wrapping_add(0), (v_).0); }
                    { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(24).wrapping_add(4), (v_).0); }
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(24).wrapping_add(8), (v_).0); }
                    { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(24).wrapping_add(12), (v_).0); }
                    F::Vec4_Copy(c, (l_sh_2).wrapping_add(8224), fp.wrapping_add(24));
                } else {
                    F::BtlObjLight_GetHalf(c, (l_sh_2).wrapping_add(8224));
                }
                { let r_ = 1i32; let a_ = (l_work_4).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = (({ let a0_ = (c.m.r32((l_xf_3).wrapping_add(212)) as i32); F::StgShadow_Collect(c, a0_, fp, 128i32, (l_sh_2).wrapping_add(16)) }) as u32); let a_ = (l_sh_2).wrapping_add(8208); c.m.w32(a_, v_); };
                { let v_ = { let a_ = F::StgShadow_GetOut(c); F32(c.m.r32(a_)) }; let a_ = (l_sh_2).wrapping_add(8216); c.m.w32(a_, (v_).0); };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_Update", &PROBE_N, &[], true); }
    r_
}

pub fn ObjShadow_BeginRender(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_BeginRender", &PROBE_N, &[(p_obj_1) as u32], false); }
    let mut l_work_2: u32 = 0;
    let r_ = 'f: {
    if ((F::ObjShadow_IsEnabled(c, p_obj_1)) != 0) {
        l_work_2 = { let a_ = (c.m.r32((p_obj_1).wrapping_add(5736))).wrapping_add(8240); c.m.r32(a_) };
        F::ObjGs_AddDefaultEnv(c);
        F::ObjShadow_SetTarget(c, p_obj_1, l_work_2);
        F::ObjShadow_MakeLightMtx(c, p_obj_1, l_work_2);
        F::ObjShadow_MakeProjMtx(c, p_obj_1, l_work_2);
        F::ObjShadow_AddTargetEnv(c);
        F::ObjShadow_LoadProg(c, p_obj_1, l_work_2);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_BeginRender", &PROBE_N, &[], true); }
    r_
}

pub fn ObjShadow_Draw(c: &mut Ctx, mut p_obj_1: u32, mut p_view_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_Draw", &PROBE_N, &[(p_obj_1) as u32, (p_view_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_sh_3: u32 = 0;
    let mut l_work_4: u32 = 0;
    let r_ = 'f: {
        if ((F::ObjShadow_IsEnabled(c, p_obj_1)) != 0) {
            l_sh_3 = c.m.r32((p_obj_1).wrapping_add(5736));
            l_work_4 = c.m.r32((l_sh_3).wrapping_add(8240));
            if (((((c.m.r32((l_sh_3).wrapping_add(8208))) != (((0i32) as u32))) as i32)) != 0) {
                if (((((c.m.r32((l_sh_3).wrapping_add(8208))) >= (((144i32) as u32))) as i32)) != 0) {
                    { let v_ = (((144i32).wrapping_sub(1i32)) as u32); let a_ = (l_sh_3).wrapping_add(8208); c.m.w32(a_, v_); };
                }
                { let a0_ = (c.m.r32((p_view_2).wrapping_add(512)) as i32); let a1_ = (c.m.r32((p_view_2).wrapping_add(516)) as i32); let a2_ = (c.m.r32((p_view_2).wrapping_add(520)) as i32); let a3_ = (c.m.r32((p_view_2).wrapping_add(524)) as i32); F::Dma_AddScissor(c, a0_, a1_, a2_, a3_) };
                F::ObjGs_AddDefaultEnv(c);
                F::BtlObjLight_GetColor(c, fp);
                { let r_ = F32(c.m.r32((l_sh_3).wrapping_add(8216))); let a_ = (fp).wrapping_add(12); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let _ = { let a0_ = { let a_ = (l_work_4).wrapping_add(((((c.m.r32((l_work_4).wrapping_add(12)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a1_ = ((c.m.r32((l_sh_3).wrapping_add(8208))) as i32); F::ObjShadow_BuildPacket(c, a0_, a1_, (l_sh_3).wrapping_add(16), fp) }; };
                F::Dma_AddTexFlush(c);
                F::ObjGs_AddShadowTexEnv(c, 10752i32, 4i32);
                F::Vu1Pkt_LoadProg6(c, p_view_2, (l_work_4).wrapping_add(64), (l_work_4).wrapping_add(48));
                { let a0_ = { let a_ = (l_work_4).wrapping_add(((((c.m.r32((l_work_4).wrapping_add(12)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Vu1Pkt_CallProg6(c, a0_) };
                F::Dma_AddTexFlush(c);
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn ObjShadow_BeginFlat(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_BeginFlat", &PROBE_N, &[(p_obj_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_prog_2: u32 = 0;
    let r_ = 'f: {
        { let _ = F::memset(c, fp.wrapping_add(80), 0i32, 16u32); };
        { let v_ = F32(0xbf800000); let a_ = (fp.wrapping_add(80)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        if ((F::ObjShadow_IsFlat(c, p_obj_1)) != 0) {
            { let a0_ = (c.m.r32((p_obj_1).wrapping_add(2800))).wrapping_add(48); F::BtlObjLight_GetColor(c, a0_) };
            { let _ = F::Vu1Pkt_LoadProg2(c, 0i32); };
            F::BtlObjLight_GetHalf(c, fp.wrapping_add(64));
            F::ObjShadow_MakePlaneMtx(c, fp, fp.wrapping_add(80), fp.wrapping_add(64), F32(0xbc23d70a));
            l_prog_2 = F::Vu1Pkt_LoadProg2(c, 0i32);
            F::Vu0Screen_StoreMtx(c, (l_prog_2).wrapping_add(192));
            F::Vu0Clip_StoreMtx(c, (l_prog_2).wrapping_add(256));
            F::Mtx_Mul(c, (l_prog_2).wrapping_add(192), (l_prog_2).wrapping_add(192), fp);
            F::Mtx_Mul(c, (l_prog_2).wrapping_add(256), (l_prog_2).wrapping_add(256), fp);
            F::ObjDraw_DrawPartsFlat(c, p_obj_1);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_BeginFlat", &PROBE_N, &[], true); }
    r_
}

pub fn ObjShadow_InitPool(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_InitPool", &PROBE_N, &[], false); }
    let mut l_pool_1: u32 = 0;
    let mut l_size_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_work_4: u32 = 0;
    let r_ = 'f: {
    l_pool_1 = F::BtlObjPool3_Get(c);
    { let _ = F::memset(c, l_pool_1, 0i32, 41296u32); };
    l_size_2 = (F::ObjShadow_CalcPacketWords(c, 144i32)).wrapping_mul(4i32);
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (5i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = F::Heap_Alloc(c, ((192u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = ((l_pool_1).wrapping_add((((l_i_3) as u32)).wrapping_mul(8256))).wrapping_add(8240); c.m.w32(a_, v_); };
                { let _ = { let a0_ = c.m.r32(((l_pool_1).wrapping_add((((l_i_3) as u32)).wrapping_mul(8256))).wrapping_add(8240)); F::memset(c, a0_, 0i32, 192u32) }; };
                l_work_4 = c.m.r32(((l_pool_1).wrapping_add((((l_i_3) as u32)).wrapping_mul(8256))).wrapping_add(8240));
                { let v_ = F::Heap_Alloc(c, l_size_2, ((64i32) as u32), 0i32, 2i32); let a_ = (l_work_4).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = F::Heap_Alloc(c, l_size_2, ((64i32) as u32), 0i32, 2i32); let a_ = (l_work_4).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let _ = { let a0_ = c.m.r32((l_work_4).wrapping_add((((0i32) as u32)).wrapping_mul(4))); F::memset(c, a0_, 0i32, ((l_size_2) as u32)) }; };
                { let _ = { let a0_ = c.m.r32((l_work_4).wrapping_add((((1i32) as u32)).wrapping_mul(4))); F::memset(c, a0_, 0i32, ((l_size_2) as u32)) }; };
                F::SList_PushFront(c, (l_pool_1).wrapping_add(41280), (l_pool_1).wrapping_add((((l_i_3) as u32)).wrapping_mul(8256)));
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_InitPool", &PROBE_N, &[], true); }
    r_
}

pub fn ObjShadow_TermPool(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_TermPool", &PROBE_N, &[], false); }
    let mut l_pool_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_work_3: u32 = 0;
    let r_ = 'f: {
    l_pool_1 = F::BtlObjPool3_Get(c);
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (5i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_work_3 = c.m.r32(((l_pool_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(8256))).wrapping_add(8240));
                { let a0_ = c.m.r32((l_work_3).wrapping_add((((0i32) as u32)).wrapping_mul(4))); F::Heap_Free(c, a0_) };
                { let a0_ = c.m.r32((l_work_3).wrapping_add((((1i32) as u32)).wrapping_mul(4))); F::Heap_Free(c, a0_) };
                { let a0_ = c.m.r32(((l_pool_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(8256))).wrapping_add(8240)); F::Heap_Free(c, a0_) };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ObjShadow_TermPool", &PROBE_N, &[], true); }
    r_
}
