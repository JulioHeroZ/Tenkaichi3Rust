#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn HudCaption_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudCaption_Draw", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((!((F::BtlGame_IsReplay(c)) != 0)) as i32)) != 0) {
        { let a0_ = ({ let a_ = (c.m.r32(sym::gHudCaption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudCaption); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
    }
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudCaption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudCaption); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudCaption_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn HudCaption_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudCaption_Update", &PROBE_N, &[], false); }
    let mut l_spr_1: u32 = 0;
    let r_ = 'f: {
    l_spr_1 = ({ let a_ = (c.m.r32(sym::gHudCaption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(28));
    if (({ let a0_ = (c.m.r32(sym::gHudCaption)).wrapping_add(12); F::Ramp_Step(c, a0_) }) != 0) {
        { let a0_ = (c.m.r32(sym::gHudCaption)).wrapping_add(12); F::Ramp_Start(c, a0_, F32(0x3f800000), F32(0x00000000), F32(0x3f800000)) };
    }
    { let a4_ = ((((rt::f2i(rt::fadd(rt::fmul(rt::fmul(rt::fadd({ let a0_ = rt::fmul(rt::fmul({ let a_ = ((c.m.r32(sym::gHudCaption)).wrapping_add(12)).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0x40490fda)), F32(0x40000000)); F::Mathf_Sin(c, a0_) }, F32(0x3f800000)), F32(0x3f000000)), F32(0x42800000)), F32(0x42800000)))) as u8)) as i32); F::HudSprite_SetColor(c, l_spr_1, 255i32, 255i32, 255i32, a4_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudCaption_Update", &PROBE_N, &[], true); }
    r_
}

pub fn HudCaption_Init(c: &mut Ctx, mut p_out_1: u32, mut p_res_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudCaption_Init", &PROBE_N, &[(p_out_1) as u32, (p_res_2) as u32], false); }
    let mut l_spr_3: u32 = 0;
    let mut l_node_4: u32 = 0;
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, ((36u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gHudCaption; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gHudCaption); F::memset(c, a0_, 0i32, 36u32) }; };
    { let v_ = F::Heap_Alloc(c, (((((3i32) as u32)).wrapping_mul(28u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gHudCaption)).wrapping_add(4); c.m.w32(a_, v_); };
    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gHudCaption)).wrapping_add(4); c.m.r32(a_) }; F::memset(c, a0_, 0i32, (((3i32) as u32)).wrapping_mul(28u32)) }; };
    { let v_ = F::Heap_Alloc(c, (((((2i32) as u32)).wrapping_mul(56u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gHudCaption)).wrapping_add(8); c.m.w32(a_, v_); };
    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gHudCaption)).wrapping_add(8); c.m.r32(a_) }; F::memset(c, a0_, 0i32, (((2i32) as u32)).wrapping_mul(56u32)) }; };
    { let v_ = p_res_2; let a_ = c.m.r32(sym::gHudCaption); c.m.w32(a_, v_); };
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudCaption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 0i32, 0i32);
    F::HudSprite_Move(c, l_spr_3, 240i32, 19i32);
    F::HudSprite_Show(c, l_spr_3, 1i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudCaption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 1i32, 0i32);
    F::HudSprite_Move(c, l_spr_3, 0i32, 386i32);
    F::HudSprite_Show(c, l_spr_3, 1i32);
    l_node_4 = ({ let a_ = (c.m.r32(sym::gHudCaption)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(56));
    { let v_ = ((2i32) as u32); let a_ = (l_node_4).wrapping_add(32); c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = (l_node_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (l_node_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = F::Heap_Alloc(c, (((((2i32) as u32)).wrapping_mul(4u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (l_node_4).wrapping_add(36); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_node_4).wrapping_add(36)); let a2_ = (c.m.r32((l_node_4).wrapping_add(32))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudCaption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudCaption)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = fid::HudCaption_Update; let a_ = (l_node_4).wrapping_add(48); c.m.w32(a_, v_); };
    { let v_ = fid::HudCaption_Draw; let a_ = (l_node_4).wrapping_add(52); c.m.w32(a_, v_); };
    l_node_4 = ({ let a_ = (c.m.r32(sym::gHudCaption)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(56));
    { let v_ = ((1i32) as u32); let a_ = (l_node_4).wrapping_add(40); c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = (l_node_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (l_node_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = F::Heap_Alloc(c, (((((1i32) as u32)).wrapping_mul(4u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (l_node_4).wrapping_add(44); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_node_4).wrapping_add(44)); let a2_ = (c.m.r32((l_node_4).wrapping_add(40))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudCaption)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(56)); let a_ = (c.m.r32((l_node_4).wrapping_add(44))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_node_4).wrapping_add(48); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_node_4).wrapping_add(52); c.m.w32(a_, v_); };
    { let v_ = { let a_ = (c.m.r32(sym::gHudCaption)).wrapping_add(8); c.m.r32(a_) }; let a_ = p_out_1; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudCaption_Init", &PROBE_N, &[], true); }
    r_
}

pub fn HudCaption_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudCaption_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gHudCaption)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gHudCaption)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
    }
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gHudCaption)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((l_i_1) as u32)).wrapping_mul(56))).wrapping_add(36); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
                    { let a0_ = { let a_ = (({ let a_ = (c.m.r32(sym::gHudCaption)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((l_i_1) as u32)).wrapping_mul(56))).wrapping_add(36); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
                }
                if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gHudCaption)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((l_i_1) as u32)).wrapping_mul(56))).wrapping_add(44); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
                    { let a0_ = { let a_ = (({ let a_ = (c.m.r32(sym::gHudCaption)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((l_i_1) as u32)).wrapping_mul(56))).wrapping_add(44); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gHudCaption)).wrapping_add(8); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gHudCaption)).wrapping_add(8); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
    }
    if (((((c.m.r32(sym::gHudCaption)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gHudCaption); F::Heap_Free(c, a0_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudCaption_Term", &PROBE_N, &[], true); }
    r_
}

pub fn HudCaption_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudCaption_Reset", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let _ = { let a0_ = (c.m.r32(sym::gHudCaption)).wrapping_add(12); F::memset(c, a0_, 0i32, 24u32) }; };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudCaption_Reset", &PROBE_N, &[], true); }
    r_
}
