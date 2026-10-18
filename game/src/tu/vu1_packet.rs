#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Vu1Pkt_CallProg7(c: &mut Ctx, mut p_chain_1: u32, mut p_w_2: F32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu1Pkt_CallProg7", &PROBE_N, &[(p_chain_1) as u32, (p_w_2).0], false); }
    let mut l_p_3: u32 = 0;
    let r_ = 'f: {
    l_p_3 = F::Dma_Alloc(c, 240i32);
    F::Vu0Cur_StoreMtx(c, (l_p_3).wrapping_add((((4i32) as u32)).wrapping_mul(4)));
    F::Vu0Screen_StoreMtx(c, (l_p_3).wrapping_add((((20i32) as u32)).wrapping_mul(4)));
    F::Vu0Clip_StoreMtx(c, (l_p_3).wrapping_add((((36i32) as u32)).wrapping_mul(4)));
    { let v_ = p_w_2; let a_ = (l_p_3).wrapping_add((((55i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = ((((1342177280i32) | (14i32))) as u32); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((p_chain_1) & (4294967295u32)); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((268435456i32) as u32); let a_ = (l_p_3).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((((1811939328i32) | ((13i32).wrapping_shl((16i32) as u32)))) | (0i32))) as u32); let a_ = (l_p_3).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((352321536i32) | (0i32))) as u32); let a_ = (l_p_3).wrapping_add((((56i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((50331648i32) | (13i32))) as u32); let a_ = (l_p_3).wrapping_add((((57i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((33554432i32) | (505i32))) as u32); let a_ = (l_p_3).wrapping_add((((58i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_3).wrapping_add((((59i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    break 'f l_p_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu1Pkt_CallProg7", &PROBE_N, &[], true); }
    r_
}

pub fn Vu1Pkt_LoadProg7(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu1Pkt_LoadProg7", &PROBE_N, &[], false); }
    let mut l_p_1: u32 = 0;
    let r_ = 'f: {
    F::Dma_AddRef(c, sym::D_002C2D30, ((sym::D_002C3080).wrapping_sub(sym::D_002C2D30) as i32));
    l_p_1 = F::Dma_Alloc(c, 240i32);
    F::Vu0Screen_StoreMtx(c, (l_p_1).wrapping_add((((20i32) as u32)).wrapping_mul(4)));
    F::Vu0Clip_StoreMtx(c, (l_p_1).wrapping_add((((36i32) as u32)).wrapping_mul(4)));
    { let v_ = ((((268435456i32) | (14i32))) as u32); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((268435456i32) as u32); let a_ = (l_p_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((((1811939328i32) | ((13i32).wrapping_shl((16i32) as u32)))) | (0i32))) as u32); let a_ = (l_p_1).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_1).wrapping_add((((56i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((50331648i32) | (13i32))) as u32); let a_ = (l_p_1).wrapping_add((((57i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((33554432i32) | (505i32))) as u32); let a_ = (l_p_1).wrapping_add((((58i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_1).wrapping_add((((59i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    break 'f l_p_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu1Pkt_LoadProg7", &PROBE_N, &[], true); }
    r_
}

pub fn Vu1Pkt_CallProg0(c: &mut Ctx, mut p_chain_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu1Pkt_CallProg0", &PROBE_N, &[(p_chain_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
    l_p_2 = F::Dma_Alloc(c, 576i32);
    { let v_ = ((((1342177280i32) | (35i32))) as u32); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((p_chain_1) & (4294967295u32)); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((268435456i32) as u32); let a_ = (l_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((((1811939328i32) | ((34i32).wrapping_shl((16i32) as u32)))) | (0i32))) as u32); let a_ = (l_p_2).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((352321536i32) | (0i32))) as u32); let a_ = (l_p_2).wrapping_add((((140i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((50331648i32) | (34i32))) as u32); let a_ = (l_p_2).wrapping_add((((141i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((33554432i32) | (495i32))) as u32); let a_ = (l_p_2).wrapping_add((((142i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_2).wrapping_add((((143i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    break 'f l_p_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu1Pkt_CallProg0", &PROBE_N, &[], true); }
    r_
}

pub fn Vu1Pkt_LoadProg0(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu1Pkt_LoadProg0", &PROBE_N, &[], false); }
    let mut l_p_1: u32 = 0;
    let r_ = 'f: {
    F::Dma_AddRef(c, sym::D_002BF6B0, ((sym::D_002BFAB0).wrapping_sub(sym::D_002BF6B0) as i32));
    l_p_1 = F::Dma_Alloc(c, 576i32);
    { let v_ = ((((268435456i32) | (35i32))) as u32); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((268435456i32) as u32); let a_ = (l_p_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((((1811939328i32) | ((34i32).wrapping_shl((16i32) as u32)))) | (0i32))) as u32); let a_ = (l_p_1).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_1).wrapping_add((((140i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((50331648i32) | (34i32))) as u32); let a_ = (l_p_1).wrapping_add((((141i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((33554432i32) | (495i32))) as u32); let a_ = (l_p_1).wrapping_add((((142i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_1).wrapping_add((((143i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    break 'f l_p_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu1Pkt_LoadProg0", &PROBE_N, &[], true); }
    r_
}

pub fn Vu1Pkt_CallProg1(c: &mut Ctx, mut p_obj_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu1Pkt_CallProg1", &PROBE_N, &[(p_obj_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
    l_p_2 = F::Dma_Alloc(c, 576i32);
    { let v_ = ((((1342177280i32) | (35i32))) as u32); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (((p_obj_1).wrapping_add(((96i32) as u32))) & (4294967295u32)); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((268435456i32) as u32); let a_ = (l_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((((1811939328i32) | ((34i32).wrapping_shl((16i32) as u32)))) | (0i32))) as u32); let a_ = (l_p_2).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((352321536i32) | (0i32))) as u32); let a_ = (l_p_2).wrapping_add((((140i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((50331648i32) | (34i32))) as u32); let a_ = (l_p_2).wrapping_add((((141i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((33554432i32) | (495i32))) as u32); let a_ = (l_p_2).wrapping_add((((142i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_2).wrapping_add((((143i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    break 'f l_p_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu1Pkt_CallProg1", &PROBE_N, &[], true); }
    r_
}

pub fn Vu1Pkt_LoadProg1(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu1Pkt_LoadProg1", &PROBE_N, &[], false); }
    let mut l_p_1: u32 = 0;
    let r_ = 'f: {
    F::Dma_AddRef(c, sym::D_002BFAB0, ((sym::D_002BFEA0).wrapping_sub(sym::D_002BFAB0) as i32));
    l_p_1 = F::Dma_Alloc(c, 576i32);
    { let v_ = ((((268435456i32) | (35i32))) as u32); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((268435456i32) as u32); let a_ = (l_p_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((((1811939328i32) | ((34i32).wrapping_shl((16i32) as u32)))) | (0i32))) as u32); let a_ = (l_p_1).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_1).wrapping_add((((140i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((50331648i32) | (34i32))) as u32); let a_ = (l_p_1).wrapping_add((((141i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((33554432i32) | (495i32))) as u32); let a_ = (l_p_1).wrapping_add((((142i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_1).wrapping_add((((143i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    break 'f l_p_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu1Pkt_LoadProg1", &PROBE_N, &[], true); }
    r_
}

pub fn Vu1Pkt_CallProg2(c: &mut Ctx, mut p_obj_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu1Pkt_CallProg2", &PROBE_N, &[(p_obj_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
    l_p_2 = F::Dma_Alloc(c, 208i32);
    { let v_ = ((((1342177280i32) | (12i32))) as u32); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (((p_obj_1).wrapping_add(((96i32) as u32))) & (4294967295u32)); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((268435456i32) as u32); let a_ = (l_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((((1811939328i32) | ((11i32).wrapping_shl((16i32) as u32)))) | (0i32))) as u32); let a_ = (l_p_2).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((352321536i32) | (0i32))) as u32); let a_ = (l_p_2).wrapping_add((((48i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((50331648i32) | (19i32))) as u32); let a_ = (l_p_2).wrapping_add((((49i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((33554432i32) | (502i32))) as u32); let a_ = (l_p_2).wrapping_add((((50i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_2).wrapping_add((((51i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    break 'f l_p_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu1Pkt_CallProg2", &PROBE_N, &[], true); }
    r_
}

pub fn Vu1Pkt_LoadProg2(c: &mut Ctx, mut p_alt_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu1Pkt_LoadProg2", &PROBE_N, &[(p_alt_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
    if ((p_alt_1) != 0) {
        F::Dma_AddRef(c, sym::D_002C01C0, ((sym::D_002C04F0).wrapping_sub(sym::D_002C01C0) as i32));
    } else {
        F::Dma_AddRef(c, sym::D_002BFEA0, ((sym::D_002C01C0).wrapping_sub(sym::D_002BFEA0) as i32));
    }
    l_p_2 = F::Dma_Alloc(c, 336i32);
    { let v_ = ((((268435456i32) | (20i32))) as u32); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((268435456i32) as u32); let a_ = (l_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((((1811939328i32) | ((19i32).wrapping_shl((16i32) as u32)))) | (0i32))) as u32); let a_ = (l_p_2).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_2).wrapping_add((((80i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((50331648i32) | (19i32))) as u32); let a_ = (l_p_2).wrapping_add((((81i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((33554432i32) | (502i32))) as u32); let a_ = (l_p_2).wrapping_add((((82i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_2).wrapping_add((((83i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    break 'f l_p_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu1Pkt_LoadProg2", &PROBE_N, &[], true); }
    r_
}

pub fn Vu1Pkt_CallProg4(c: &mut Ctx, mut p_chain_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu1Pkt_CallProg4", &PROBE_N, &[(p_chain_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
    l_p_2 = F::Dma_Alloc(c, 32i32);
    { let v_ = ((((1342177280i32) | (1i32))) as u32); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((p_chain_1) & (4294967295u32)); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((268435456i32) as u32); let a_ = (l_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_2).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((352321536i32) | (0i32))) as u32); let a_ = (l_p_2).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((50331648i32) | (141i32))) as u32); let a_ = (l_p_2).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((33554432i32) | (436i32))) as u32); let a_ = (l_p_2).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_2).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu1Pkt_CallProg4", &PROBE_N, &[], true); }
    r_
}

pub fn Vu1Pkt_LoadProg4(c: &mut Ctx, mut p_mtx_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu1Pkt_LoadProg4", &PROBE_N, &[(p_mtx_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
    F::Dma_AddRef(c, sym::D_002C04F0, ((sym::D_002C1180).wrapping_sub(sym::D_002C04F0) as i32));
    l_p_2 = F::Dma_Alloc(c, 224i32);
    F::Mtx_Copy(c, (l_p_2).wrapping_add((((36i32) as u32)).wrapping_mul(4)), p_mtx_1);
    F::Vu0Screen_StoreMtx(c, (l_p_2).wrapping_add((((4i32) as u32)).wrapping_mul(4)));
    F::Vu0Clip_StoreMtx(c, (l_p_2).wrapping_add((((20i32) as u32)).wrapping_mul(4)));
    { let v_ = ((((268435456i32) | (13i32))) as u32); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((268435456i32) as u32); let a_ = (l_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((((1811939328i32) | ((12i32).wrapping_shl((16i32) as u32)))) | (0i32))) as u32); let a_ = (l_p_2).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_2).wrapping_add((((52i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((50331648i32) | (141i32))) as u32); let a_ = (l_p_2).wrapping_add((((53i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((33554432i32) | (436i32))) as u32); let a_ = (l_p_2).wrapping_add((((54i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_2).wrapping_add((((55i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu1Pkt_LoadProg4", &PROBE_N, &[], true); }
    r_
}

pub fn Vu1Pkt_CallProg5(c: &mut Ctx, mut p_chain_1: u32, mut p_mtx_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu1Pkt_CallProg5", &PROBE_N, &[(p_chain_1) as u32, (p_mtx_2) as u32], false); }
    let mut l_p_3: u32 = 0;
    let r_ = 'f: {
    l_p_3 = F::Dma_Alloc(c, 96i32);
    F::Mtx_Copy(c, (l_p_3).wrapping_add((((4i32) as u32)).wrapping_mul(4)), p_mtx_2);
    { let v_ = ((((1342177280i32) | (5i32))) as u32); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((p_chain_1) & (4294967295u32)); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((268435456i32) as u32); let a_ = (l_p_3).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((((1811939328i32) | ((4i32).wrapping_shl((16i32) as u32)))) | (0i32))) as u32); let a_ = (l_p_3).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((352321536i32) | (0i32))) as u32); let a_ = (l_p_3).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((50331648i32) | (141i32))) as u32); let a_ = (l_p_3).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((33554432i32) | (436i32))) as u32); let a_ = (l_p_3).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_3).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu1Pkt_CallProg5", &PROBE_N, &[], true); }
    r_
}

pub fn Vu1Pkt_LoadProg5(c: &mut Ctx, mut p_mtx_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu1Pkt_LoadProg5", &PROBE_N, &[(p_mtx_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
    F::Dma_AddRef(c, sym::D_002C1180, ((sym::D_002C1F00).wrapping_sub(sym::D_002C1180) as i32));
    l_p_2 = F::Dma_Alloc(c, 288i32);
    F::Mtx_StoreIdentity(c, (l_p_2).wrapping_add((((4i32) as u32)).wrapping_mul(4)));
    F::Mtx_Copy(c, (l_p_2).wrapping_add((((52i32) as u32)).wrapping_mul(4)), p_mtx_1);
    F::Vu0Screen_StoreMtx(c, (l_p_2).wrapping_add((((20i32) as u32)).wrapping_mul(4)));
    F::Vu0Clip_StoreMtx(c, (l_p_2).wrapping_add((((36i32) as u32)).wrapping_mul(4)));
    { let v_ = ((((268435456i32) | (17i32))) as u32); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((268435456i32) as u32); let a_ = (l_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((((1811939328i32) | ((16i32).wrapping_shl((16i32) as u32)))) | (0i32))) as u32); let a_ = (l_p_2).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_2).wrapping_add((((68i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((50331648i32) | (141i32))) as u32); let a_ = (l_p_2).wrapping_add((((69i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((33554432i32) | (436i32))) as u32); let a_ = (l_p_2).wrapping_add((((70i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_2).wrapping_add((((71i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu1Pkt_LoadProg5", &PROBE_N, &[], true); }
    r_
}

pub fn Vu1Node_OrFlags(c: &mut Ctx, mut p_node_1: u32, mut p_bits0_2: u32, mut p_bits1_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu1Node_OrFlags", &PROBE_N, &[(p_node_1) as u32, (p_bits0_2) as u32, (p_bits1_3) as u32], false); }
    let mut l_p_4: u32 = 0;
    let r_ = 'f: {
    'l1: while ((1i32) != 0) {
        if (((c.m.r32((p_node_1).wrapping_add(12)) as i32)) != 0) {
            {
                l_p_4 = (p_node_1).wrapping_add(240);
                'l2: loop {
                    if !(((((c.m.r32(l_p_4)) != (((1879048192i32) as u32))) as i32)) != 0) { break 'l2; }
                    'c3: {
                        if (((((c.m.r32(l_p_4)) == (((1812168704i32) as u32))) as i32)) != 0) {
                            { let r_ = p_bits0_2; let a_ = (l_p_4).wrapping_add((((5i32) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                            { let r_ = (p_bits1_3).wrapping_shl((5i32) as u32); let a_ = (l_p_4).wrapping_add((((6i32) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                        }
                    }
                    { let o_ = l_p_4; let n_ = (o_).wrapping_add(4); l_p_4 = n_; };
                }
            }
        }
        if (((c.m.r32((p_node_1).wrapping_add(8)) as i32)) != 0) {
            break 'l1;
        }
        p_node_1 = (p_node_1).wrapping_add((((c.m.r32(p_node_1) as i32)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu1Node_OrFlags", &PROBE_N, &[], true); }
    r_
}

pub fn Vu1Node_Animate(c: &mut Ctx, mut p_node_1: u32, mut p_track_2: u32, mut p_mtx_3: u32, mut p_frame_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu1Node_Animate", &PROBE_N, &[(p_node_1) as u32, (p_track_2) as u32, (p_mtx_3) as u32, (p_frame_4).0], false); }
    let mut l_next_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_count_7: i32 = 0;
    let mut l_t_8: F32 = F32(0);
    let r_ = 'f: {
    F::Vu0Cur_ResetStack(c);
    F::Vu0Cur_LoadMtx(c, p_mtx_3);
    'l1: while ((1i32) != 0) {
        F::Vu0Cur_Push(c);
        if (((((p_track_2) != (((0i32) as u32))) as i32)) != 0) {
            l_count_7 = ((c.m.r16((p_track_2).wrapping_add(10))) as i32);
            l_next_5 = (p_track_2).wrapping_add(16);
            {
                l_i_6 = 1i32;
                'l2: loop {
                    if !(((((l_i_6) < (l_count_7)) as i32)) != 0) { break 'l2; }
                    'c3: {
                        { let o_ = l_next_5; let n_ = (o_).wrapping_add(32); l_next_5 = n_; };
                        if (((rt::feq(rt::u2f(c.m.r32(((l_next_5).wrapping_sub((((1i32) as u32)).wrapping_mul(32))).wrapping_add(12))), p_frame_4) as i32)) != 0) {
                            { let a1_ = F32(c.m.r32(((l_next_5).wrapping_sub((((1i32) as u32)).wrapping_mul(32))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a2_ = F32(c.m.r32(((l_next_5).wrapping_sub((((1i32) as u32)).wrapping_mul(32))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a3_ = F32(c.m.r32(((l_next_5).wrapping_sub((((1i32) as u32)).wrapping_mul(32))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a4_ = F32(c.m.r32(((l_next_5).wrapping_sub((((1i32) as u32)).wrapping_mul(32))).wrapping_add((((3i32) as u32)).wrapping_mul(4)))); F::Vec4_Set(c, (p_node_1).wrapping_add(16), a1_, a2_, a3_, a4_) };
                            { let a1_ = F32(c.m.r32((((l_next_5).wrapping_sub((((1i32) as u32)).wrapping_mul(32))).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a2_ = F32(c.m.r32((((l_next_5).wrapping_sub((((1i32) as u32)).wrapping_mul(32))).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a3_ = F32(c.m.r32((((l_next_5).wrapping_sub((((1i32) as u32)).wrapping_mul(32))).wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a4_ = F32(c.m.r32((((l_next_5).wrapping_sub((((1i32) as u32)).wrapping_mul(32))).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(4)))); F::Vec4_Set(c, (p_node_1).wrapping_add(32), a1_, a2_, a3_, a4_) };
                            break 'l2;
                        }
                        if (((((((rt::flt(rt::u2f(c.m.r32(((l_next_5).wrapping_sub((((1i32) as u32)).wrapping_mul(32))).wrapping_add(12))), p_frame_4) as i32)) != 0) && (((rt::flt(p_frame_4, rt::u2f(c.m.r32((l_next_5).wrapping_add(12)))) as i32)) != 0)) as i32)) != 0) {
                            l_t_8 = rt::fdiv(rt::fsub(p_frame_4, rt::u2f(c.m.r32(((l_next_5).wrapping_sub((((1i32) as u32)).wrapping_mul(32))).wrapping_add(12)))), rt::u2f((c.m.r32((l_next_5).wrapping_add(12))).wrapping_sub(c.m.r32(((l_next_5).wrapping_sub((((1i32) as u32)).wrapping_mul(32))).wrapping_add(12)))));
                            { let v_ = rt::fadd(rt::fmul(rt::fsub(F32(c.m.r32((l_next_5).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_next_5).wrapping_sub((((1i32) as u32)).wrapping_mul(32))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), l_t_8), F32(c.m.r32(((l_next_5).wrapping_sub((((1i32) as u32)).wrapping_mul(32))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((p_node_1).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                            { let v_ = rt::fadd(rt::fmul(rt::fsub(F32(c.m.r32((l_next_5).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_next_5).wrapping_sub((((1i32) as u32)).wrapping_mul(32))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), l_t_8), F32(c.m.r32(((l_next_5).wrapping_sub((((1i32) as u32)).wrapping_mul(32))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = ((p_node_1).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                            { let v_ = rt::fadd(rt::fmul(rt::fsub(F32(c.m.r32((l_next_5).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_next_5).wrapping_sub((((1i32) as u32)).wrapping_mul(32))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))), l_t_8), F32(c.m.r32(((l_next_5).wrapping_sub((((1i32) as u32)).wrapping_mul(32))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let a_ = ((p_node_1).wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                            { let v_ = rt::fadd(rt::fmul(rt::fsub(F32(c.m.r32(((l_next_5).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_next_5).wrapping_sub((((1i32) as u32)).wrapping_mul(32))).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), l_t_8), F32(c.m.r32((((l_next_5).wrapping_sub((((1i32) as u32)).wrapping_mul(32))).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((p_node_1).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                            { let v_ = rt::fadd(rt::fmul(rt::fsub(F32(c.m.r32(((l_next_5).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_next_5).wrapping_sub((((1i32) as u32)).wrapping_mul(32))).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), l_t_8), F32(c.m.r32((((l_next_5).wrapping_sub((((1i32) as u32)).wrapping_mul(32))).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = ((p_node_1).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                            { let v_ = rt::fadd(rt::fmul(rt::fsub(F32(c.m.r32(((l_next_5).wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_next_5).wrapping_sub((((1i32) as u32)).wrapping_mul(32))).wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(4))))), l_t_8), F32(c.m.r32((((l_next_5).wrapping_sub((((1i32) as u32)).wrapping_mul(32))).wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let a_ = ((p_node_1).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                            break 'l2;
                        }
                        if (((rt::feq(rt::u2f(c.m.r32((l_next_5).wrapping_add(12))), p_frame_4) as i32)) != 0) {
                            { let a1_ = F32(c.m.r32((l_next_5).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a2_ = F32(c.m.r32((l_next_5).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a3_ = F32(c.m.r32((l_next_5).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a4_ = F32(c.m.r32((l_next_5).wrapping_add((((3i32) as u32)).wrapping_mul(4)))); F::Vec4_Set(c, (p_node_1).wrapping_add(16), a1_, a2_, a3_, a4_) };
                            { let a1_ = F32(c.m.r32(((l_next_5).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a2_ = F32(c.m.r32(((l_next_5).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a3_ = F32(c.m.r32(((l_next_5).wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a4_ = F32(c.m.r32(((l_next_5).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(4)))); F::Vec4_Set(c, (p_node_1).wrapping_add(32), a1_, a2_, a3_, a4_) };
                            break 'l2;
                        }
                    }
                    { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                }
            }
        }
        { let v_ = F32(0x3f800000); let a_ = ((p_node_1).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((p_node_1).wrapping_add(32)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        F::Vu0Cur_StoreMtx(c, (p_node_1).wrapping_add(80));
        F::Vu0Cur_TranslateLocal(c, (p_node_1).wrapping_add(16));
        F::Vu0Cur_RotateYXZ(c, (p_node_1).wrapping_add(32));
        F::Vu0Cur_StoreMtx(c, (p_node_1).wrapping_add(144));
        F::Vec4_Copy(c, (p_node_1).wrapping_add(208), (p_node_1).wrapping_add(48));
        F::Vec4_Copy(c, (p_node_1).wrapping_add(224), (p_node_1).wrapping_add(64));
        if (((c.m.r32((p_node_1).wrapping_add(4)) as i32)) != 0) {
            { let a0_ = (c.m.r32((p_node_1).wrapping_add(4)) as i32); F::Vu0Cur_PopN(c, a0_) };
        }
        if (((c.m.r32((p_node_1).wrapping_add(8)) as i32)) != 0) {
            break 'l1;
        }
        p_node_1 = (p_node_1).wrapping_add((((c.m.r32(p_node_1) as i32)) as u32));
        if (((((p_track_2) != (((0i32) as u32))) as i32)) != 0) {
            p_track_2 = ((p_track_2).wrapping_add(16)).wrapping_add((((c.m.r16((p_track_2).wrapping_add(10))) as u32)).wrapping_mul(32));
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu1Node_Animate", &PROBE_N, &[], true); }
    r_
}

pub fn Vu1Node_Draw(c: &mut Ctx, mut p_node_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu1Node_Draw", &PROBE_N, &[(p_node_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
    'l1: while ((1i32) != 0) {
        if (((c.m.r32((p_node_1).wrapping_add(12)) as i32)) != 0) {
            l_p_2 = F::Vu1Pkt_CallProg8(c, p_node_1);
            F::Mtx_Copy(c, (l_p_2).wrapping_add((((20i32) as u32)).wrapping_mul(4)), (p_node_1).wrapping_add(80));
            F::Mtx_Copy(c, (l_p_2).wrapping_add((((4i32) as u32)).wrapping_mul(4)), (p_node_1).wrapping_add(144));
            F::Vec4_Copy(c, (l_p_2).wrapping_add((((36i32) as u32)).wrapping_mul(4)), (p_node_1).wrapping_add(208));
            F::Vec4_Copy(c, (l_p_2).wrapping_add((((40i32) as u32)).wrapping_mul(4)), (p_node_1).wrapping_add(224));
        }
        if (((c.m.r32((p_node_1).wrapping_add(8)) as i32)) != 0) {
            break 'l1;
        }
        p_node_1 = (p_node_1).wrapping_add((((c.m.r32(p_node_1) as i32)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu1Node_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn Vu1Pkt_CallProg8(c: &mut Ctx, mut p_node_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu1Pkt_CallProg8", &PROBE_N, &[(p_node_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
    l_p_2 = F::Dma_Alloc(c, 192i32);
    { let v_ = ((((1342177280i32) | (11i32))) as u32); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (((p_node_1).wrapping_add(240)) & (4294967295u32)); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((268435456i32) as u32); let a_ = (l_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((((1811939328i32) | ((10i32).wrapping_shl((16i32) as u32)))) | (0i32))) as u32); let a_ = (l_p_2).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((352321536i32) | (0i32))) as u32); let a_ = (l_p_2).wrapping_add((((44i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((50331648i32) | (18i32))) as u32); let a_ = (l_p_2).wrapping_add((((45i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((33554432i32) | (503i32))) as u32); let a_ = (l_p_2).wrapping_add((((46i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_2).wrapping_add((((47i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    break 'f l_p_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu1Pkt_CallProg8", &PROBE_N, &[], true); }
    r_
}

pub fn Vu1Pkt_LoadProg8(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu1Pkt_LoadProg8", &PROBE_N, &[], false); }
    let mut l_p_1: u32 = 0;
    let r_ = 'f: {
    F::Dma_AddRef(c, sym::D_002C3080, ((rt::div_u32(((768i32) as u32), 1u32)) as i32));
    l_p_1 = F::Dma_Alloc(c, 320i32);
    { let v_ = ((((268435456i32) | (19i32))) as u32); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((268435456i32) as u32); let a_ = (l_p_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((((1811939328i32) | ((18i32).wrapping_shl((16i32) as u32)))) | (0i32))) as u32); let a_ = (l_p_1).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_1).wrapping_add((((76i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((50331648i32) | (18i32))) as u32); let a_ = (l_p_1).wrapping_add((((77i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((33554432i32) | (503i32))) as u32); let a_ = (l_p_1).wrapping_add((((78i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_1).wrapping_add((((79i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    break 'f l_p_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu1Pkt_LoadProg8", &PROBE_N, &[], true); }
    r_
}

pub fn Vu1Pkt_CallProg6(c: &mut Ctx, mut p_chain_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu1Pkt_CallProg6", &PROBE_N, &[(p_chain_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
    l_p_2 = F::Dma_Alloc(c, 32i32);
    { let v_ = ((((1342177280i32) | (1i32))) as u32); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((p_chain_1) & (4294967295u32)); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((268435456i32) as u32); let a_ = (l_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_2).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((352321536i32) | (0i32))) as u32); let a_ = (l_p_2).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((50331648i32) | (146i32))) as u32); let a_ = (l_p_2).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((33554432i32) | (434i32))) as u32); let a_ = (l_p_2).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_2).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu1Pkt_CallProg6", &PROBE_N, &[], true); }
    r_
}

pub fn Vu1Pkt_LoadProg6(c: &mut Ctx, mut p_obj_1: u32, mut p_mtx_2: u32, mut p_vec_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu1Pkt_LoadProg6", &PROBE_N, &[(p_obj_1) as u32, (p_mtx_2) as u32, (p_vec_3) as u32], false); }
    let mut l_p_4: u32 = 0;
    let r_ = 'f: {
    F::Dma_AddRef(c, sym::D_002C1F00, ((sym::D_002C2D30).wrapping_sub(sym::D_002C1F00) as i32));
    l_p_4 = F::Dma_Alloc(c, 304i32);
    { let _ = F::memset(c, l_p_4, 0i32, ((304i32) as u32)); };
    F::Mtx_Copy(c, (l_p_4).wrapping_add((((36i32) as u32)).wrapping_mul(4)), (p_obj_1).wrapping_add(((256i32) as u32)));
    F::Mtx_Mul(c, (l_p_4).wrapping_add((((4i32) as u32)).wrapping_mul(4)), (p_obj_1).wrapping_add(((128i32) as u32)), (p_obj_1).wrapping_add(((64i32) as u32)));
    F::Mtx_Mul(c, (l_p_4).wrapping_add((((20i32) as u32)).wrapping_mul(4)), (p_obj_1).wrapping_add(((192i32) as u32)), (p_obj_1).wrapping_add(((64i32) as u32)));
    F::Mtx_Copy(c, (l_p_4).wrapping_add((((52i32) as u32)).wrapping_mul(4)), p_mtx_2);
    F::Vec4_Copy(c, (l_p_4).wrapping_add((((68i32) as u32)).wrapping_mul(4)), p_vec_3);
    { let v_ = ((((268435456i32) | (18i32))) as u32); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((268435456i32) as u32); let a_ = (l_p_4).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((((1811939328i32) | ((17i32).wrapping_shl((16i32) as u32)))) | (0i32))) as u32); let a_ = (l_p_4).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_4).wrapping_add((((72i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((50331648i32) | (146i32))) as u32); let a_ = (l_p_4).wrapping_add((((73i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((33554432i32) | (434i32))) as u32); let a_ = (l_p_4).wrapping_add((((74i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_4).wrapping_add((((75i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu1Pkt_LoadProg6", &PROBE_N, &[], true); }
    r_
}
