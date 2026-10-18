#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlObjDraw_ApplyView(c: &mut Ctx, mut p_view_1: i32, mut p_count_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjDraw_ApplyView", &PROBE_N, &[(p_view_1) as u32, (p_count_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_count_2) >= (2i32)) as i32)) != 0) {
        { let a0_ = F::BtlObjVis_GetView(c, p_view_1); F::View_Apply(c, a0_, 1i32) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjDraw_ApplyView", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjDraw_DrawLists(c: &mut Ctx, mut p_count_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjDraw_DrawLists", &PROBE_N, &[(p_count_1) as u32], false); }
    let mut l_v_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_list_4: u32 = 0;
    let mut l_obj_5: u32 = 0;
    let mut l_state_6: u32 = 0;
    let r_ = 'f: {
    {
        l_v_2 = 0i32;
        'l1: loop {
            if !(((((l_v_2) < (p_count_1)) as i32)) != 0) { break 'l1; }
            'c2: {
                F::BtlObjDraw_ApplyView(c, l_v_2, p_count_1);
                l_list_4 = F::BtlObjVis_GetList(c, l_v_2);
                {
                    l_i_3 = 0i32;
                    'l3: loop {
                        if !(((((l_i_3) < ((c.m.r32((l_list_4).wrapping_add(48)) as i32))) as i32)) != 0) { break 'l3; }
                        'c4: {
                            l_obj_5 = c.m.r32((l_list_4).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)));
                            l_state_6 = (l_obj_5).wrapping_add(2624);
                            { let v_ = ((l_state_6).wrapping_add(16)).wrapping_add((((l_v_2) as u32)).wrapping_mul(80)); let a_ = (l_state_6).wrapping_add(176); c.m.w32(a_, v_); };
                            F::BtlObjDraw_DrawModel(c, l_obj_5);
                        }
                        { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                    }
                }
            }
            { let o_ = l_v_2; let n_ = (o_).wrapping_add(1); l_v_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjDraw_DrawLists", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjDraw_DrawFadeLists(c: &mut Ctx, mut p_count_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjDraw_DrawFadeLists", &PROBE_N, &[(p_count_1) as u32], false); }
    let mut l_v_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_list_4: u32 = 0;
    let mut l_obj_5: u32 = 0;
    let mut l_state_6: u32 = 0;
    let r_ = 'f: {
    {
        l_v_2 = 0i32;
        'l1: loop {
            if !(((((l_v_2) < (p_count_1)) as i32)) != 0) { break 'l1; }
            'c2: {
                F::BtlObjDraw_ApplyView(c, l_v_2, p_count_1);
                l_list_4 = F::BtlObjVis_GetList(c, l_v_2);
                {
                    l_i_3 = 0i32;
                    'l3: loop {
                        if !(((((l_i_3) < ((c.m.r32((l_list_4).wrapping_add(48)) as i32))) as i32)) != 0) { break 'l3; }
                        'c4: {
                            l_obj_5 = c.m.r32((l_list_4).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)));
                            l_state_6 = (l_obj_5).wrapping_add(2624);
                            { let v_ = ((l_state_6).wrapping_add(16)).wrapping_add((((l_v_2) as u32)).wrapping_mul(80)); let a_ = (l_state_6).wrapping_add(176); c.m.w32(a_, v_); };
                            if (((({ let a_ = c.m.r32((l_state_6).wrapping_add(176)); c.m.r32(a_) }) & (((8i32) as u32)))) != 0) {
                                F::BtlObjDraw_DrawModelFade(c, l_obj_5);
                            }
                        }
                        { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                    }
                }
            }
            { let o_ = l_v_2; let n_ = (o_).wrapping_add(1); l_v_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjDraw_DrawFadeLists", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjDraw_IsInList(c: &mut Ctx, mut p_view_1: i32, mut p_obj_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjDraw_IsInList", &PROBE_N, &[(p_view_1) as u32, (p_obj_2) as u32], false); }
    let mut l_list_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_list_3 = F::BtlObjVis_GetList(c, p_view_1);
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < ((c.m.r32((l_list_3).wrapping_add(48)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((c.m.r32((l_list_3).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)))) == (p_obj_2)) as i32)) != 0) {
                    break 'f 1i32;
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjDraw_IsInList", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjDraw_DrawShadows(c: &mut Ctx, mut p_count_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjDraw_DrawShadows", &PROBE_N, &[(p_count_1) as u32], false); }
    let mut l_node_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_mask_4: i32 = 0;
    let mut l_obj_5: u32 = 0;
    let mut l_state_6: u32 = 0;
    let r_ = 'f: {
    F::BtlObjDraw_SetEnv(c);
    F::Gfx_ClearScreen(c, 16777215i32, ((0i32) as u32));
    {
        l_node_2 = { let a0_ = F::BtlObj_GetUsedList(c); F::List_GetHead(c, a0_) };
        'l1: loop {
            if !(((((l_node_2) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
            'c2: {
                l_mask_4 = 0i32;
                l_obj_5 = (l_node_2).wrapping_add(((16i32) as u32));
                l_state_6 = (l_obj_5).wrapping_add(2624);
                {
                    l_i_3 = 0i32;
                    'l3: loop {
                        if !(((((l_i_3) < (p_count_1)) as i32)) != 0) { break 'l3; }
                        'c4: {
                            if ((F::BtlObjDraw_IsInList(c, l_i_3, l_obj_5)) != 0) {
                                { let r_ = (1i32).wrapping_shl((l_i_3) as u32); let n_ = ((l_mask_4) | (r_)); l_mask_4 = n_; };
                            }
                        }
                        { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                    }
                }
                if (((((((((((((l_mask_4) != (0i32)) as i32)) != 0) && (((((rt::bf_get_u(c, (l_obj_5).wrapping_add(2624), 1, 1) as u32)) as i32)) != 0)) as i32)) != 0) && (((((((rt::bf_get_u(c, (l_obj_5).wrapping_add(2624), 4, 12) as u32)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
                    if ((((c.m.r32((l_obj_5).wrapping_add(2624))) & (((536870912i32) as u32)))) != 0) {
                        F::ObjGs_AddFlatShadowEnv(c);
                        {
                            l_i_3 = 0i32;
                            'l5: loop {
                                if !(((((l_i_3) < (p_count_1)) as i32)) != 0) { break 'l5; }
                                'c6: {
                                    if (((((l_mask_4).wrapping_shr((l_i_3) as u32)) & (1i32))) != 0) {
                                        { let v_ = ((l_state_6).wrapping_add(16)).wrapping_add((((l_i_3) as u32)).wrapping_mul(80)); let a_ = (l_state_6).wrapping_add(176); c.m.w32(a_, v_); };
                                        F::ObjShadow_BeginFlat(c, l_obj_5);
                                    }
                                }
                                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                            }
                        }
                    } else {
                        if ((((c.m.r32((l_obj_5).wrapping_add(2624))) & (((268435456i32) as u32)))) != 0) {
                            F::ObjShadow_BeginRender(c, l_obj_5);
                            {
                                l_i_3 = 0i32;
                                'l7: loop {
                                    if !(((((l_i_3) < (p_count_1)) as i32)) != 0) { break 'l7; }
                                    'c8: {
                                        if (((((l_mask_4).wrapping_shr((l_i_3) as u32)) & (1i32))) != 0) {
                                            { let v_ = ((l_state_6).wrapping_add(16)).wrapping_add((((l_i_3) as u32)).wrapping_mul(80)); let a_ = (l_state_6).wrapping_add(176); c.m.w32(a_, v_); };
                                            { let a1_ = F::BtlObjVis_GetView(c, l_i_3); F::ObjShadow_Draw(c, l_obj_5, a1_) };
                                        }
                                    }
                                    { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                                }
                            }
                        }
                    }
                }
            }
            l_node_2 = F::List_GetNext(c, l_node_2);
        }
    }
    F::ObjShadow_ResetEnv(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjDraw_DrawShadows", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjDraw_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjDraw_Draw", &PROBE_N, &[], false); }
    let mut l_count_1: i32 = 0;
    let r_ = 'f: {
    l_count_1 = { let a_ = (F::BtlObjVis_Get(c)).wrapping_add(8); (c.m.r32(a_) as i32) };
    if ((F::BtlObjVis_TestFlags(c, ((2i32) as u32))) != 0) {
        F::Gfx_ClearScreen(c, 16777215i32, 4278190080u32);
        F::BtlObjLight_FindRes0(c);
        F::BtlObjDraw_DrawLists(c, l_count_1);
        F::BtlObj_BeginDraw(c);
        if ((F::BtlObjVis_TestFlags(c, ((8i32) as u32))) != 0) {
            F::BtlObjDraw_DrawFadeLists(c, l_count_1);
        }
        F::BtlObjDraw_DrawShadows(c, l_count_1);
        F::BtlObjDraw_End(c);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjDraw_Draw", &PROBE_N, &[], true); }
    r_
}
