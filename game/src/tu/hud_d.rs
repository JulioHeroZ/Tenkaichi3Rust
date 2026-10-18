#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn HudSprite_Draw(c: &mut Ctx, mut p_spr_1: u32, mut p_res_2: u32, mut p_additive_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_Draw", &PROBE_N, &[(p_spr_1) as u32, (p_res_2) as u32, (p_additive_3) as u32], false); }
    let r_ = 'f: {
    F::HudSprite_DrawAt(c, p_spr_1, p_res_2, ((p_additive_3) as u8), 10752i32, 11392i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn HudNode_Show(c: &mut Ctx, mut p_node_1: u32, mut p_show_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNode_Show", &PROBE_N, &[(p_node_1) as u32, (p_show_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((((c.m.r32(p_node_1)) & ((((!(1i32))) as u32)))) | ((((((p_show_2) == (0i32)) as i32)) as u32))); let a_ = p_node_1; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudNode_Show", &PROBE_N, &[], true); }
    r_
}

pub fn HudNode_SetPos(c: &mut Ctx, mut p_node_1: u32, mut p_x_2: i32, mut p_y_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNode_SetPos", &PROBE_N, &[(p_node_1) as u32, (p_x_2) as u32, (p_y_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_x_2; let a_ = (p_node_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_y_3; let a_ = (p_node_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudNode_SetPos", &PROBE_N, &[], true); }
    r_
}

pub fn HudNode_SetOfs(c: &mut Ctx, mut p_node_1: u32, mut p_x_2: i32, mut p_y_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNode_SetOfs", &PROBE_N, &[(p_node_1) as u32, (p_x_2) as u32, (p_y_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_x_2; let a_ = (p_node_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_y_3; let a_ = (p_node_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudNode_SetOfs", &PROBE_N, &[], true); }
    r_
}

pub fn HudNode_SetRot(c: &mut Ctx, mut p_node_1: u32, mut p_rot_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNode_SetRot", &PROBE_N, &[(p_node_1) as u32, (p_rot_2).0], false); }
    let r_ = 'f: {
    { let v_ = p_rot_2; let a_ = (p_node_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudNode_SetRot", &PROBE_N, &[], true); }
    r_
}

pub fn HudNode_SetUnk8(c: &mut Ctx, mut p_node_1: u32, mut p_a_2: F32, mut p_b_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNode_SetUnk8", &PROBE_N, &[(p_node_1) as u32, (p_a_2).0, (p_b_3).0], false); }
    let r_ = 'f: {
    { let v_ = p_a_2; let a_ = (p_node_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = p_b_3; let a_ = (p_node_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudNode_SetUnk8", &PROBE_N, &[], true); }
    r_
}
