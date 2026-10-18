#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn FlashClipList_GotoLabel(c: &mut Ctx, mut p_list_1: u32, mut p_ref_2: u32, mut p_label_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_GotoLabel", &PROBE_N, &[(p_list_1) as u32, (p_ref_2) as u32, (p_label_3) as u32], false); }
    let mut l_clip_4: u32 = 0;
    let mut l_p_5: u32 = 0;
    let mut l_n_6: i32 = 0;
    let mut l_i_7: u32 = 0;
    let r_ = 'f: {
    l_clip_4 = (c.m.r32(p_list_1)).wrapping_add(((((c.m.r32(p_ref_2) as i32)) as u32)).wrapping_mul(228));
    F::FlashTl_GotoLabel(c, (l_clip_4).wrapping_add(204), p_label_3, ((((1i32) as i64 as u64)) as u8));
    if ((((((c.m.r8(((l_clip_4).wrapping_add(204)).wrapping_add(21))) as i32)) & (4i32))) != 0) {
        if (((((c.m.r32((l_clip_4).wrapping_add(68)) as i32)) & (8i32))) != 0) {
            { let a1_ = { let a_ = (c.m.r32((l_clip_4).wrapping_add(204))).wrapping_add(20); (c.m.r32(a_) as i32) }; F::FlashTl_Advance(c, (l_clip_4).wrapping_add(204), ((((a1_) as i64 as u64)) as u32), ((((1i32) as i64 as u64)) as u8), ((((1i32) as i64 as u64)) as u8)) };
        } else {
            { let a1_ = { let a_ = (c.m.r32((l_clip_4).wrapping_add(204))).wrapping_add(20); (c.m.r32(a_) as i32) }; F::FlashTl_Advance(c, (l_clip_4).wrapping_add(204), ((((a1_) as i64 as u64)) as u32), ((((1i32) as i64 as u64)) as u8), ((((0i32) as i64 as u64)) as u8)) };
        }
    }
    l_n_6 = (c.m.r32((p_ref_2).wrapping_add(4)) as i32);
    if (((((l_n_6) != (0i32)) as i32)) != 0) {
        {
            l_i_7 = ((1i32) as u32);
            'l1: loop {
                if !(((((l_i_7) < ((c.m.r32((p_list_1).wrapping_add(4))).wrapping_sub((((c.m.r32(p_ref_2) as i32)) as u32)))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_p_5 = (l_clip_4).wrapping_add((l_i_7).wrapping_mul(228));
                    if (((((F::strcmp(c, l_p_5, l_clip_4)) == (0i32)) as i32)) != 0) {
                        F::FlashTl_GotoLabel(c, (l_p_5).wrapping_add(204), p_label_3, ((((1i32) as i64 as u64)) as u8));
                        if ((((((c.m.r8(((l_p_5).wrapping_add(204)).wrapping_add(21))) as i32)) & (4i32))) != 0) {
                            if (((((c.m.r32((l_p_5).wrapping_add(68)) as i32)) & (8i32))) != 0) {
                                { let a1_ = { let a_ = (c.m.r32((l_p_5).wrapping_add(204))).wrapping_add(20); (c.m.r32(a_) as i32) }; F::FlashTl_Advance(c, (l_p_5).wrapping_add(204), ((((a1_) as i64 as u64)) as u32), ((((1i32) as i64 as u64)) as u8), ((((1i32) as i64 as u64)) as u8)) };
                            } else {
                                { let a1_ = { let a_ = (c.m.r32((l_p_5).wrapping_add(204))).wrapping_add(20); (c.m.r32(a_) as i32) }; F::FlashTl_Advance(c, (l_p_5).wrapping_add(204), ((((a1_) as i64 as u64)) as u32), ((((1i32) as i64 as u64)) as u8), ((((0i32) as i64 as u64)) as u8)) };
                            }
                        }
                        { let o_ = l_n_6; let n_ = (o_).wrapping_sub(1); l_n_6 = n_; };
                        if (((((l_n_6) == (0i32)) as i32)) != 0) {
                            break 'l1;
                        }
                    }
                }
                { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_GotoLabel", &PROBE_N, &[], true); }
    r_
}

pub fn FlashClipList_SetPreDraw(c: &mut Ctx, mut p_list_1: u32, mut p_ref_2: u32, mut p_fn_3: i32, mut p_arg_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_SetPreDraw", &PROBE_N, &[(p_list_1) as u32, (p_ref_2) as u32, (p_fn_3) as u32, (p_arg_4) as u32], false); }
    let mut l_clip_5: u32 = 0;
    let mut l_n_6: i32 = 0;
    let mut l_i_7: u32 = 0;
    let r_ = 'f: {
    l_clip_5 = (c.m.r32(p_list_1)).wrapping_add(((((c.m.r32(p_ref_2) as i32)) as u32)).wrapping_mul(228));
    { let v_ = p_fn_3; let a_ = (l_clip_5).wrapping_add(180); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_arg_4; let a_ = (l_clip_5).wrapping_add(188); c.m.w32(a_, (v_) as u32); };
    l_n_6 = (c.m.r32((p_ref_2).wrapping_add(4)) as i32);
    if (((((l_n_6) != (0i32)) as i32)) != 0) {
        {
            l_i_7 = ((1i32) as u32);
            'l1: loop {
                if !(((((l_i_7) < ((c.m.r32((p_list_1).wrapping_add(4))).wrapping_sub((((c.m.r32(p_ref_2) as i32)) as u32)))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((F::strcmp(c, (l_clip_5).wrapping_add((l_i_7).wrapping_mul(228)), l_clip_5)) == (0i32)) as i32)) != 0) {
                        { let v_ = p_fn_3; let a_ = ((l_clip_5).wrapping_add((l_i_7).wrapping_mul(228))).wrapping_add(180); c.m.w32(a_, (v_) as u32); };
                        { let v_ = p_arg_4; let a_ = ((l_clip_5).wrapping_add((l_i_7).wrapping_mul(228))).wrapping_add(188); c.m.w32(a_, (v_) as u32); };
                        { let o_ = l_n_6; let n_ = (o_).wrapping_sub(1); l_n_6 = n_; };
                        if (((((l_n_6) == (0i32)) as i32)) != 0) {
                            break 'l1;
                        }
                    }
                }
                { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_SetPreDraw", &PROBE_N, &[], true); }
    r_
}

pub fn FlashClipList_SetPostDraw(c: &mut Ctx, mut p_list_1: u32, mut p_ref_2: u32, mut p_fn_3: i32, mut p_arg_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_SetPostDraw", &PROBE_N, &[(p_list_1) as u32, (p_ref_2) as u32, (p_fn_3) as u32, (p_arg_4) as u32], false); }
    let mut l_clip_5: u32 = 0;
    let mut l_n_6: i32 = 0;
    let mut l_i_7: u32 = 0;
    let r_ = 'f: {
    l_clip_5 = (c.m.r32(p_list_1)).wrapping_add(((((c.m.r32(p_ref_2) as i32)) as u32)).wrapping_mul(228));
    { let v_ = p_fn_3; let a_ = (l_clip_5).wrapping_add(184); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_arg_4; let a_ = (l_clip_5).wrapping_add(192); c.m.w32(a_, (v_) as u32); };
    l_n_6 = (c.m.r32((p_ref_2).wrapping_add(4)) as i32);
    if (((((l_n_6) != (0i32)) as i32)) != 0) {
        {
            l_i_7 = ((1i32) as u32);
            'l1: loop {
                if !(((((l_i_7) < ((c.m.r32((p_list_1).wrapping_add(4))).wrapping_sub((((c.m.r32(p_ref_2) as i32)) as u32)))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((F::strcmp(c, (l_clip_5).wrapping_add((l_i_7).wrapping_mul(228)), l_clip_5)) == (0i32)) as i32)) != 0) {
                        { let v_ = p_fn_3; let a_ = ((l_clip_5).wrapping_add((l_i_7).wrapping_mul(228))).wrapping_add(184); c.m.w32(a_, (v_) as u32); };
                        { let v_ = p_arg_4; let a_ = ((l_clip_5).wrapping_add((l_i_7).wrapping_mul(228))).wrapping_add(192); c.m.w32(a_, (v_) as u32); };
                        { let o_ = l_n_6; let n_ = (o_).wrapping_sub(1); l_n_6 = n_; };
                        if (((((l_n_6) == (0i32)) as i32)) != 0) {
                            break 'l1;
                        }
                    }
                }
                { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_SetPostDraw", &PROBE_N, &[], true); }
    r_
}

pub fn FlashClipList_SetDrawOver(c: &mut Ctx, mut p_list_1: u32, mut p_ref_2: u32, mut p_fn_3: i32, mut p_arg_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_SetDrawOver", &PROBE_N, &[(p_list_1) as u32, (p_ref_2) as u32, (p_fn_3) as u32, (p_arg_4) as u32], false); }
    let mut l_clip_5: u32 = 0;
    let mut l_n_6: i32 = 0;
    let mut l_i_7: u32 = 0;
    let r_ = 'f: {
    l_clip_5 = (c.m.r32(p_list_1)).wrapping_add(((((c.m.r32(p_ref_2) as i32)) as u32)).wrapping_mul(228));
    { let v_ = p_fn_3; let a_ = (l_clip_5).wrapping_add(196); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_arg_4; let a_ = (l_clip_5).wrapping_add(200); c.m.w32(a_, (v_) as u32); };
    l_n_6 = (c.m.r32((p_ref_2).wrapping_add(4)) as i32);
    if (((((l_n_6) != (0i32)) as i32)) != 0) {
        {
            l_i_7 = ((1i32) as u32);
            'l1: loop {
                if !(((((l_i_7) < ((c.m.r32((p_list_1).wrapping_add(4))).wrapping_sub((((c.m.r32(p_ref_2) as i32)) as u32)))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((F::strcmp(c, (l_clip_5).wrapping_add((l_i_7).wrapping_mul(228)), l_clip_5)) == (0i32)) as i32)) != 0) {
                        { let v_ = p_fn_3; let a_ = ((l_clip_5).wrapping_add((l_i_7).wrapping_mul(228))).wrapping_add(196); c.m.w32(a_, (v_) as u32); };
                        { let v_ = p_arg_4; let a_ = ((l_clip_5).wrapping_add((l_i_7).wrapping_mul(228))).wrapping_add(200); c.m.w32(a_, (v_) as u32); };
                        { let o_ = l_n_6; let n_ = (o_).wrapping_sub(1); l_n_6 = n_; };
                        if (((((l_n_6) == (0i32)) as i32)) != 0) {
                            break 'l1;
                        }
                    }
                }
                { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_SetDrawOver", &PROBE_N, &[], true); }
    r_
}

pub fn FlashClipList_SetFlags(c: &mut Ctx, mut p_list_1: u32, mut p_ref_2: u32, mut p_mask_3: i32, mut p_on_4: u8) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_SetFlags", &PROBE_N, &[(p_list_1) as u32, (p_ref_2) as u32, (p_mask_3) as u32, (p_on_4) as u32], false); }
    let mut l_clip_5: u32 = 0;
    let mut l_n_6: i32 = 0;
    let mut l_i_7: u32 = 0;
    let r_ = 'f: {
    l_clip_5 = (c.m.r32(p_list_1)).wrapping_add(((((c.m.r32(p_ref_2) as i32)) as u32)).wrapping_mul(228));
    if ((p_on_4) != 0) {
        { let r_ = p_mask_3; let a_ = (l_clip_5).wrapping_add(68); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        { let r_ = (!(p_mask_3)); let a_ = (l_clip_5).wrapping_add(68); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    }
    l_n_6 = (c.m.r32((p_ref_2).wrapping_add(4)) as i32);
    if (((((l_n_6) != (0i32)) as i32)) != 0) {
        {
            l_i_7 = ((1i32) as u32);
            'l1: loop {
                if !(((((l_i_7) < ((c.m.r32((p_list_1).wrapping_add(4))).wrapping_sub((((c.m.r32(p_ref_2) as i32)) as u32)))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((F::strcmp(c, (l_clip_5).wrapping_add((l_i_7).wrapping_mul(228)), l_clip_5)) == (0i32)) as i32)) != 0) {
                        if ((p_on_4) != 0) {
                            { let r_ = p_mask_3; let a_ = ((l_clip_5).wrapping_add((l_i_7).wrapping_mul(228))).wrapping_add(68); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        } else {
                            { let r_ = (!(p_mask_3)); let a_ = ((l_clip_5).wrapping_add((l_i_7).wrapping_mul(228))).wrapping_add(68); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                        { let o_ = l_n_6; let n_ = (o_).wrapping_sub(1); l_n_6 = n_; };
                        if (((((l_n_6) == (0i32)) as i32)) != 0) {
                            break 'l1;
                        }
                    }
                }
                { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_SetFlags", &PROBE_N, &[], true); }
    r_
}

pub fn FlashClipList_SetOverride(c: &mut Ctx, mut p_list_1: u32, mut p_ref_2: u32, mut p_mask_3: i32, mut p_on_4: u8) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_SetOverride", &PROBE_N, &[(p_list_1) as u32, (p_ref_2) as u32, (p_mask_3) as u32, (p_on_4) as u32], false); }
    let mut l_clip_5: u32 = 0;
    let mut l_n_6: i32 = 0;
    let mut l_i_7: u32 = 0;
    let r_ = 'f: {
    l_clip_5 = (c.m.r32(p_list_1)).wrapping_add(((((c.m.r32(p_ref_2) as i32)) as u32)).wrapping_mul(228));
    if ((p_on_4) != 0) {
        { let r_ = p_mask_3; let a_ = (l_clip_5).wrapping_add(72); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        { let r_ = (!(p_mask_3)); let a_ = (l_clip_5).wrapping_add(72); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    }
    l_n_6 = (c.m.r32((p_ref_2).wrapping_add(4)) as i32);
    if (((((l_n_6) != (0i32)) as i32)) != 0) {
        {
            l_i_7 = ((1i32) as u32);
            'l1: loop {
                if !(((((l_i_7) < ((c.m.r32((p_list_1).wrapping_add(4))).wrapping_sub((((c.m.r32(p_ref_2) as i32)) as u32)))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((F::strcmp(c, (l_clip_5).wrapping_add((l_i_7).wrapping_mul(228)), l_clip_5)) == (0i32)) as i32)) != 0) {
                        if ((p_on_4) != 0) {
                            { let r_ = p_mask_3; let a_ = ((l_clip_5).wrapping_add((l_i_7).wrapping_mul(228))).wrapping_add(72); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        } else {
                            { let r_ = (!(p_mask_3)); let a_ = ((l_clip_5).wrapping_add((l_i_7).wrapping_mul(228))).wrapping_add(72); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                        { let o_ = l_n_6; let n_ = (o_).wrapping_sub(1); l_n_6 = n_; };
                        if (((((l_n_6) == (0i32)) as i32)) != 0) {
                            break 'l1;
                        }
                    }
                }
                { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_SetOverride", &PROBE_N, &[], true); }
    r_
}

pub fn FlashClipList_SetOffset(c: &mut Ctx, mut p_list_1: u32, mut p_ref_2: u32, mut p_x_3: i32, mut p_y_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_SetOffset", &PROBE_N, &[(p_list_1) as u32, (p_ref_2) as u32, (p_x_3) as u32, (p_y_4) as u32], false); }
    let mut l_clip_5: u32 = 0;
    let mut l_n_6: i32 = 0;
    let mut l_i_7: u32 = 0;
    let mut l_fx_8: F32 = F32(0);
    let mut l_fy_9: F32 = F32(0);
    let r_ = 'f: {
    l_clip_5 = (c.m.r32(p_list_1)).wrapping_add(((((c.m.r32(p_ref_2) as i32)) as u32)).wrapping_mul(228));
    l_fx_8 = rt::i2f(p_x_3);
    l_fy_9 = rt::i2f(p_y_4);
    { let v_ = l_fx_8; let a_ = (l_clip_5).wrapping_add(76); c.m.w32(a_, (v_).0); };
    { let v_ = l_fy_9; let a_ = (l_clip_5).wrapping_add(80); c.m.w32(a_, (v_).0); };
    { let r_ = 2i32; let a_ = (l_clip_5).wrapping_add(72); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    l_n_6 = (c.m.r32((p_ref_2).wrapping_add(4)) as i32);
    if (((((l_n_6) != (0i32)) as i32)) != 0) {
        {
            l_i_7 = ((1i32) as u32);
            'l1: loop {
                if !(((((l_i_7) < ((c.m.r32((p_list_1).wrapping_add(4))).wrapping_sub((((c.m.r32(p_ref_2) as i32)) as u32)))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((F::strcmp(c, (l_clip_5).wrapping_add((l_i_7).wrapping_mul(228)), l_clip_5)) == (0i32)) as i32)) != 0) {
                        { let v_ = l_fx_8; let a_ = ((l_clip_5).wrapping_add((l_i_7).wrapping_mul(228))).wrapping_add(76); c.m.w32(a_, (v_).0); };
                        { let v_ = l_fy_9; let a_ = ((l_clip_5).wrapping_add((l_i_7).wrapping_mul(228))).wrapping_add(80); c.m.w32(a_, (v_).0); };
                        { let r_ = 2i32; let a_ = ((l_clip_5).wrapping_add((l_i_7).wrapping_mul(228))).wrapping_add(72); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        { let o_ = l_n_6; let n_ = (o_).wrapping_sub(1); l_n_6 = n_; };
                        if (((((l_n_6) == (0i32)) as i32)) != 0) {
                            break 'l1;
                        }
                    }
                }
                { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_SetOffset", &PROBE_N, &[], true); }
    r_
}

pub fn FlashClipList_SetScale(c: &mut Ctx, mut p_list_1: u32, mut p_ref_2: u32, mut p_x_3: F32, mut p_y_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_SetScale", &PROBE_N, &[(p_list_1) as u32, (p_ref_2) as u32, (p_x_3).0, (p_y_4).0], false); }
    let mut l_clip_5: u32 = 0;
    let mut l_n_6: i32 = 0;
    let mut l_i_7: u32 = 0;
    let r_ = 'f: {
    l_clip_5 = (c.m.r32(p_list_1)).wrapping_add(((((c.m.r32(p_ref_2) as i32)) as u32)).wrapping_mul(228));
    { let v_ = p_x_3; let a_ = (l_clip_5).wrapping_add(88); c.m.w32(a_, (v_).0); };
    { let v_ = p_y_4; let a_ = (l_clip_5).wrapping_add(92); c.m.w32(a_, (v_).0); };
    { let r_ = 16i32; let a_ = (l_clip_5).wrapping_add(72); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    l_n_6 = (c.m.r32((p_ref_2).wrapping_add(4)) as i32);
    if (((((l_n_6) != (0i32)) as i32)) != 0) {
        {
            l_i_7 = ((1i32) as u32);
            'l1: loop {
                if !(((((l_i_7) < ((c.m.r32((p_list_1).wrapping_add(4))).wrapping_sub((((c.m.r32(p_ref_2) as i32)) as u32)))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((F::strcmp(c, (l_clip_5).wrapping_add((l_i_7).wrapping_mul(228)), l_clip_5)) == (0i32)) as i32)) != 0) {
                        { let v_ = p_x_3; let a_ = ((l_clip_5).wrapping_add((l_i_7).wrapping_mul(228))).wrapping_add(88); c.m.w32(a_, (v_).0); };
                        { let v_ = p_y_4; let a_ = ((l_clip_5).wrapping_add((l_i_7).wrapping_mul(228))).wrapping_add(92); c.m.w32(a_, (v_).0); };
                        { let r_ = 16i32; let a_ = ((l_clip_5).wrapping_add((l_i_7).wrapping_mul(228))).wrapping_add(72); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        { let o_ = l_n_6; let n_ = (o_).wrapping_sub(1); l_n_6 = n_; };
                        if (((((l_n_6) == (0i32)) as i32)) != 0) {
                            break 'l1;
                        }
                    }
                }
                { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_SetScale", &PROBE_N, &[], true); }
    r_
}

pub fn FlashClipList_SetAlpha(c: &mut Ctx, mut p_list_1: u32, mut p_ref_2: u32, mut p_alpha_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_SetAlpha", &PROBE_N, &[(p_list_1) as u32, (p_ref_2) as u32, (p_alpha_3).0], false); }
    let mut l_clip_4: u32 = 0;
    let mut l_p_5: u32 = 0;
    let mut l_n_6: i32 = 0;
    let mut l_i_7: u32 = 0;
    let r_ = 'f: {
    l_clip_4 = (c.m.r32(p_list_1)).wrapping_add(((((c.m.r32(p_ref_2) as i32)) as u32)).wrapping_mul(228));
    if (((rt::fgt(p_alpha_3, F32(0x3f800000)) as i32)) != 0) {
        { let v_ = F32(0x3f800000); let a_ = (l_clip_4).wrapping_add(84); c.m.w32(a_, (v_).0); };
    } else {
        if (((rt::flt(p_alpha_3, F32(0x00000000)) as i32)) != 0) {
            { let v_ = F32(0x00000000); let a_ = (l_clip_4).wrapping_add(84); c.m.w32(a_, (v_).0); };
        } else {
            { let v_ = p_alpha_3; let a_ = (l_clip_4).wrapping_add(84); c.m.w32(a_, (v_).0); };
        }
    }
    { let r_ = 8i32; let a_ = (l_clip_4).wrapping_add(72); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    l_n_6 = (c.m.r32((p_ref_2).wrapping_add(4)) as i32);
    if (((((l_n_6) != (0i32)) as i32)) != 0) {
        {
            l_i_7 = ((1i32) as u32);
            'l1: loop {
                if !(((((l_i_7) < ((c.m.r32((p_list_1).wrapping_add(4))).wrapping_sub((((c.m.r32(p_ref_2) as i32)) as u32)))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_p_5 = (l_clip_4).wrapping_add((l_i_7).wrapping_mul(228));
                    if (((((F::strcmp(c, l_p_5, l_clip_4)) == (0i32)) as i32)) != 0) {
                        if (((rt::fgt(p_alpha_3, F32(0x3f800000)) as i32)) != 0) {
                            { let v_ = F32(0x3f800000); let a_ = (l_p_5).wrapping_add(84); c.m.w32(a_, (v_).0); };
                        } else {
                            if (((rt::flt(p_alpha_3, F32(0x00000000)) as i32)) != 0) {
                                { let v_ = F32(0x00000000); let a_ = (l_p_5).wrapping_add(84); c.m.w32(a_, (v_).0); };
                            } else {
                                { let v_ = p_alpha_3; let a_ = (l_p_5).wrapping_add(84); c.m.w32(a_, (v_).0); };
                            }
                        }
                        { let r_ = 8i32; let a_ = (l_p_5).wrapping_add(72); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        { let o_ = l_n_6; let n_ = (o_).wrapping_sub(1); l_n_6 = n_; };
                        if (((((l_n_6) == (0i32)) as i32)) != 0) {
                            break 'l1;
                        }
                    }
                }
                { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_SetAlpha", &PROBE_N, &[], true); }
    r_
}

pub fn FlashClipList_SetColor(c: &mut Ctx, mut p_list_1: u32, mut p_ref_2: u32, mut p_color_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_SetColor", &PROBE_N, &[(p_list_1) as u32, (p_ref_2) as u32, (p_color_3).0], false); }
    let mut l_clip_4: u32 = 0;
    let mut l_p_5: u32 = 0;
    let mut l_n_6: i32 = 0;
    let mut l_i_7: u32 = 0;
    let r_ = 'f: {
    l_clip_4 = (c.m.r32(p_list_1)).wrapping_add(((((c.m.r32(p_ref_2) as i32)) as u32)).wrapping_mul(228));
    if (((rt::fgt(p_color_3, F32(0x3f800000)) as i32)) != 0) {
        { let v_ = F32(0x3f800000); let a_ = (l_clip_4).wrapping_add(96); c.m.w32(a_, (v_).0); };
    } else {
        if (((rt::flt(p_color_3, F32(0x00000000)) as i32)) != 0) {
            { let v_ = F32(0x00000000); let a_ = (l_clip_4).wrapping_add(96); c.m.w32(a_, (v_).0); };
        } else {
            { let v_ = p_color_3; let a_ = (l_clip_4).wrapping_add(96); c.m.w32(a_, (v_).0); };
        }
    }
    { let r_ = 32i32; let a_ = (l_clip_4).wrapping_add(72); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    l_n_6 = (c.m.r32((p_ref_2).wrapping_add(4)) as i32);
    if (((((l_n_6) != (0i32)) as i32)) != 0) {
        {
            l_i_7 = ((1i32) as u32);
            'l1: loop {
                if !(((((l_i_7) < ((c.m.r32((p_list_1).wrapping_add(4))).wrapping_sub((((c.m.r32(p_ref_2) as i32)) as u32)))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_p_5 = (l_clip_4).wrapping_add((l_i_7).wrapping_mul(228));
                    if (((((F::strcmp(c, l_p_5, l_clip_4)) == (0i32)) as i32)) != 0) {
                        if (((rt::fgt(p_color_3, F32(0x3f800000)) as i32)) != 0) {
                            { let v_ = F32(0x3f800000); let a_ = (l_p_5).wrapping_add(96); c.m.w32(a_, (v_).0); };
                        } else {
                            if (((rt::flt(p_color_3, F32(0x00000000)) as i32)) != 0) {
                                { let v_ = F32(0x00000000); let a_ = (l_p_5).wrapping_add(96); c.m.w32(a_, (v_).0); };
                            } else {
                                { let v_ = p_color_3; let a_ = (l_p_5).wrapping_add(96); c.m.w32(a_, (v_).0); };
                            }
                        }
                        { let r_ = 32i32; let a_ = (l_p_5).wrapping_add(72); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        { let o_ = l_n_6; let n_ = (o_).wrapping_sub(1); l_n_6 = n_; };
                        if (((((l_n_6) == (0i32)) as i32)) != 0) {
                            break 'l1;
                        }
                    }
                }
                { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_SetColor", &PROBE_N, &[], true); }
    r_
}

pub fn FlashClipList_SetTex(c: &mut Ctx, mut p_list_1: u32, mut p_ref_2: u32, mut p_user_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_SetTex", &PROBE_N, &[(p_list_1) as u32, (p_ref_2) as u32, (p_user_3) as u32], false); }
    let mut l_clip_4: u32 = 0;
    let mut l_n_5: i32 = 0;
    let mut l_i_6: u32 = 0;
    let r_ = 'f: {
    l_clip_4 = (c.m.r32(p_list_1)).wrapping_add(((((c.m.r32(p_ref_2) as i32)) as u32)).wrapping_mul(228));
    { let v_ = p_user_3; let a_ = (l_clip_4).wrapping_add(116); c.m.w32(a_, (v_) as u32); };
    { let r_ = 1i32; let a_ = (l_clip_4).wrapping_add(72); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    l_n_5 = (c.m.r32((p_ref_2).wrapping_add(4)) as i32);
    if (((((l_n_5) != (0i32)) as i32)) != 0) {
        {
            l_i_6 = ((1i32) as u32);
            'l1: loop {
                if !(((((l_i_6) < ((c.m.r32((p_list_1).wrapping_add(4))).wrapping_sub((((c.m.r32(p_ref_2) as i32)) as u32)))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((F::strcmp(c, (l_clip_4).wrapping_add((l_i_6).wrapping_mul(228)), l_clip_4)) == (0i32)) as i32)) != 0) {
                        { let v_ = p_user_3; let a_ = ((l_clip_4).wrapping_add((l_i_6).wrapping_mul(228))).wrapping_add(116); c.m.w32(a_, (v_) as u32); };
                        { let r_ = 1i32; let a_ = ((l_clip_4).wrapping_add((l_i_6).wrapping_mul(228))).wrapping_add(72); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        { let o_ = l_n_5; let n_ = (o_).wrapping_sub(1); l_n_5 = n_; };
                        if (((((l_n_5) == (0i32)) as i32)) != 0) {
                            break 'l1;
                        }
                    }
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_SetTex", &PROBE_N, &[], true); }
    r_
}

pub fn FlashClipList_SetUv(c: &mut Ctx, mut p_list_1: u32, mut p_ref_2: u32, mut p_uv_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_SetUv", &PROBE_N, &[(p_list_1) as u32, (p_ref_2) as u32, (p_uv_3) as u32], false); }
    let mut l_clip_4: u32 = 0;
    let mut l_p_5: u32 = 0;
    let mut l_n_6: i32 = 0;
    let mut l_i_7: u32 = 0;
    let r_ = 'f: {
    l_clip_4 = (c.m.r32(p_list_1)).wrapping_add(((((c.m.r32(p_ref_2) as i32)) as u32)).wrapping_mul(228));
    { let v_ = (c.m.r32(p_uv_3) as i32); let a_ = (l_clip_4).wrapping_add(100); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32((p_uv_3).wrapping_add(4)) as i32); let a_ = ((l_clip_4).wrapping_add(100)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32((p_uv_3).wrapping_add(8)) as i32); let a_ = ((l_clip_4).wrapping_add(100)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32((p_uv_3).wrapping_add(12)) as i32); let a_ = ((l_clip_4).wrapping_add(100)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let r_ = 4i32; let a_ = (l_clip_4).wrapping_add(72); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    l_n_6 = (c.m.r32((p_ref_2).wrapping_add(4)) as i32);
    if (((((l_n_6) != (0i32)) as i32)) != 0) {
        {
            l_i_7 = ((1i32) as u32);
            'l1: loop {
                if !(((((l_i_7) < ((c.m.r32((p_list_1).wrapping_add(4))).wrapping_sub((((c.m.r32(p_ref_2) as i32)) as u32)))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_p_5 = (l_clip_4).wrapping_add((l_i_7).wrapping_mul(228));
                    if (((((F::strcmp(c, l_p_5, l_clip_4)) == (0i32)) as i32)) != 0) {
                        { let v_ = (c.m.r32(p_uv_3) as i32); let a_ = (l_p_5).wrapping_add(100); c.m.w32(a_, (v_) as u32); };
                        { let v_ = (c.m.r32((p_uv_3).wrapping_add(4)) as i32); let a_ = ((l_p_5).wrapping_add(100)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = (c.m.r32((p_uv_3).wrapping_add(8)) as i32); let a_ = ((l_p_5).wrapping_add(100)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = (c.m.r32((p_uv_3).wrapping_add(12)) as i32); let a_ = ((l_p_5).wrapping_add(100)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        { let r_ = 4i32; let a_ = (l_p_5).wrapping_add(72); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        { let o_ = l_n_6; let n_ = (o_).wrapping_sub(1); l_n_6 = n_; };
                        if (((((l_n_6) == (0i32)) as i32)) != 0) {
                            break 'l1;
                        }
                    }
                }
                { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_SetUv", &PROBE_N, &[], true); }
    r_
}

pub fn FlashClipList_GetPos(c: &mut Ctx, mut p_list_1: u32, mut p_ref_2: u32, mut p_x_3: u32, mut p_y_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_GetPos", &PROBE_N, &[(p_list_1) as u32, (p_ref_2) as u32, (p_x_3) as u32, (p_y_4) as u32], false); }
    let mut l_clip_5: u32 = 0;
    let r_ = 'f: {
    l_clip_5 = (c.m.r32(p_list_1)).wrapping_add(((((c.m.r32(p_ref_2) as i32)) as u32)).wrapping_mul(228));
    { let v_ = rt::f2i(F32(c.m.r32((l_clip_5).wrapping_add(128)))); let a_ = p_x_3; c.m.w32(a_, (v_) as u32); };
    { let v_ = rt::f2i(F32(c.m.r32((l_clip_5).wrapping_add(140)))); let a_ = p_y_4; c.m.w32(a_, (v_) as u32); };
    if (((((c.m.r32((l_clip_5).wrapping_add(72)) as i32)) & (2i32))) != 0) {
        { let r_ = rt::f2i(F32(c.m.r32((l_clip_5).wrapping_add(76)))); let a_ = p_x_3; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
        { let r_ = rt::f2i(F32(c.m.r32((l_clip_5).wrapping_add(80)))); let a_ = p_y_4; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_GetPos", &PROBE_N, &[], true); }
    r_
}

pub fn FlashClipList_GetAlpha(c: &mut Ctx, mut p_list_1: u32, mut p_ref_2: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_GetAlpha", &PROBE_N, &[(p_list_1) as u32, (p_ref_2) as u32], false); }
    let mut l_clip_3: u32 = 0;
    let mut l_a_4: i32 = 0;
    let mut l_f_5: F32 = F32(0);
    let r_ = 'f: {
    l_clip_3 = (c.m.r32(p_list_1)).wrapping_add(((((c.m.r32(p_ref_2) as i32)) as u32)).wrapping_mul(228));
    l_a_4 = (rt::f2i(rt::fmul(F32(c.m.r32((l_clip_3).wrapping_add(168))), F32(0x43000000)))).wrapping_add((((c.m.r16((l_clip_3).wrapping_add(178)) as i16)) as i32));
    l_f_5 = rt::i2f((if (((((l_a_4) < (0i32)) as i32)) != 0) { 0i32 } else { (if (((((l_a_4) > (255i32)) as i32)) != 0) { 255i32 } else { l_a_4 }) }));
    if (((((c.m.r32((l_clip_3).wrapping_add(72)) as i32)) & (8i32))) != 0) {
        l_f_5 = rt::fmul(rt::fmul(l_f_5, F32(c.m.r32((l_clip_3).wrapping_add(84)))), F32(0x3c000000));
    } else {
        l_f_5 = rt::fmul(l_f_5, F32(0x3c000000));
    }
    break 'f l_f_5;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_GetAlpha", &PROBE_N, &[], true); }
    r_
}
