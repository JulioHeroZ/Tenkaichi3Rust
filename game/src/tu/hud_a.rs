#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Hud_ClearTexMarks(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Hud_ClearTexMarks", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_j_2: i32 = 0;
    let mut l_n_3: i32 = 0;
    let mut l_t_4: u32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (5i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_n_3 = { let a_ = { let a_ = ((c.m.r32(sym::gHud)).wrapping_add(28)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }; (c.m.r32(a_) as i32) };
                {
                    l_j_2 = 0i32;
                    'l3: loop {
                        if !(((((l_j_2) < (l_n_3)) as i32)) != 0) { break 'l3; }
                        'c4: {
                            l_t_4 = ({ let a_ = ({ let a_ = ((c.m.r32(sym::gHud)).wrapping_add(28)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((l_j_2) as u32)).wrapping_mul(64));
                            { let v_ = 0i32; let a_ = (l_t_4).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                        }
                        { let o_ = l_j_2; let n_ = (o_).wrapping_add(1); l_j_2 = n_; };
                    }
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Hud_ClearTexMarks", &PROBE_N, &[], true); }
    r_
}

pub fn HudNode_Update(c: &mut Ctx, mut p_node_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNode_Update", &PROBE_N, &[(p_node_1) as u32], false); }
    let mut l_i_2: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32((p_node_1).wrapping_add(48))) != (((0i32) as u32))) as i32)) != 0) {
        { let fp_ = c.m.r32((p_node_1).wrapping_add(48)); { let _ = D::call(c, fp_, &[((p_node_1) as u64)]); } };
    }
    {
        l_i_2 = ((0i32) as u32);
        'l1: loop {
            if !(((((l_i_2) < (c.m.r32((p_node_1).wrapping_add(40)))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = { let a_ = (c.m.r32((p_node_1).wrapping_add(44))).wrapping_add((l_i_2).wrapping_mul(4)); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudNode_Update", &PROBE_N, &[], true); }
    r_
}

pub fn HudNode_Draw(c: &mut Ctx, mut p_node_1: u32, mut p_mirror_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNode_Draw", &PROBE_N, &[(p_node_1) as u32, (p_mirror_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(144) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 144);
    let mut l_i_3: u32 = 0;
    let r_ = 'f: {
        if ((((c.m.r32(p_node_1)) & (((1i32) as u32)))) != 0) {
            break 'f;
        }
        if (((((((((p_node_1) == ({ let a_ = (c.m.r32(sym::gHud)).wrapping_add(4); c.m.r32(a_) })) as i32)) != 0) || (((((p_node_1) == ({ let a_ = (c.m.r32(sym::gHud)).wrapping_add(12); c.m.r32(a_) })) as i32)) != 0)) as i32)) != 0) {
            F::Port_GsMarker(c, 18i32);
        }
        if (((((p_node_1) == ({ let a_ = (c.m.r32(sym::gHud)).wrapping_add(24); c.m.r32(a_) })) as i32)) != 0) {
            F::Port_GsMarker(c, 20i32);
        }
        F::Vu0Cur_Push(c);
        { let v_ = rt::i2f(((c.m.r32((p_node_1).wrapping_add(16)) as i32)).wrapping_add((c.m.r32((p_node_1).wrapping_add(24)) as i32))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f(((c.m.r32((p_node_1).wrapping_add(20)) as i32)).wrapping_add((c.m.r32((p_node_1).wrapping_add(28)) as i32))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        F::Mtx_StoreIdentity(c, fp.wrapping_add(16));
        F::Mtx_Translate(c, fp.wrapping_add(16), fp.wrapping_add(16), fp);
        F::Vu0Cur_MulMtx(c, fp.wrapping_add(16));
        F::Mtx_StoreIdentity(c, fp.wrapping_add(16));
        { let a2_ = F32(c.m.r32((p_node_1).wrapping_add(4))); F::Mtx_RotateZ(c, fp.wrapping_add(16), fp.wrapping_add(16), a2_) };
        F::Vu0Cur_MulMtx(c, fp.wrapping_add(16));
        if (((((c.m.r32((p_node_1).wrapping_add(52))) != (((0i32) as u32))) as i32)) != 0) {
            F::Vu0Cur_Push(c);
            if ((((((p_mirror_2) != 0) || ((((c.m.r32(p_node_1)) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
                F::Mtx_StoreIdentity(c, fp.wrapping_add(80));
                { let v_ = F32(0xbf800000); let a_ = ((fp.wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x43ff8000); let a_ = ((fp.wrapping_add(80)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                F::Vu0Cur_MulMtxRev(c, fp.wrapping_add(80));
            }
            { let fp_ = c.m.r32((p_node_1).wrapping_add(52)); { let _ = D::call(c, fp_, &[((p_node_1) as u64)]); } };
            F::Vu0Cur_Pop(c);
        }
        {
            l_i_3 = ((0i32) as u32);
            'l1: loop {
                if !(((((l_i_3) < (c.m.r32((p_node_1).wrapping_add(40)))) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a0_ = { let a_ = (c.m.r32((p_node_1).wrapping_add(44))).wrapping_add((l_i_3).wrapping_mul(4)); c.m.r32(a_) }; let a1_ = (if ((((((p_mirror_2) != 0) || ((((c.m.r32(p_node_1)) & (((2i32) as u32)))) != 0)) as i32)) != 0) { 1i32 } else { 0i32 }); F::HudNode_Draw(c, a0_, a1_) };
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        F::Vu0Cur_Pop(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HudNode_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn Hud_SetReplayMode(c: &mut Ctx, mut p_mode_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Hud_SetReplayMode", &PROBE_N, &[(p_mode_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_mode_1; let a_ = (c.m.r32(sym::gHud)).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Hud_SetReplayMode", &PROBE_N, &[], true); }
    r_
}

pub fn Hud_ShowAll(c: &mut Ctx, mut p_on_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Hud_ShowAll", &PROBE_N, &[(p_on_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((p_on_1) as u32); let a_ = (c.m.r32(sym::gHud)).wrapping_add(48); rt::bf_set(c, a_, 0, 1, (v_) as u64);  };
    { let v_ = ((p_on_1) as u32); let a_ = (c.m.r32(sym::gHud)).wrapping_add(48); rt::bf_set(c, a_, 1, 1, (v_) as u64);  };
    { let v_ = ((p_on_1) as u32); let a_ = (c.m.r32(sym::gHud)).wrapping_add(48); rt::bf_set(c, a_, 2, 1, (v_) as u64);  };
    { let v_ = ((p_on_1) as u32); let a_ = (c.m.r32(sym::gHud)).wrapping_add(48); rt::bf_set(c, a_, 3, 1, (v_) as u64);  };
    { let v_ = ((p_on_1) as u32); let a_ = (c.m.r32(sym::gHud)).wrapping_add(48); rt::bf_set(c, a_, 4, 1, (v_) as u64);  };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Hud_ShowAll", &PROBE_N, &[], true); }
    r_
}

pub fn Hud_ShowGauges(c: &mut Ctx, mut p_on_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Hud_ShowGauges", &PROBE_N, &[(p_on_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((p_on_1) as u32); let a_ = (c.m.r32(sym::gHud)).wrapping_add(48); rt::bf_set(c, a_, 0, 1, (v_) as u64);  };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Hud_ShowGauges", &PROBE_N, &[], true); }
    r_
}

pub fn Hud_ShowTimer(c: &mut Ctx, mut p_on_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Hud_ShowTimer", &PROBE_N, &[(p_on_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((p_on_1) as u32); let a_ = (c.m.r32(sym::gHud)).wrapping_add(48); rt::bf_set(c, a_, 1, 1, (v_) as u64);  };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Hud_ShowTimer", &PROBE_N, &[], true); }
    r_
}

pub fn Hud_ShowNotice(c: &mut Ctx, mut p_on_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Hud_ShowNotice", &PROBE_N, &[(p_on_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((p_on_1) as u32); let a_ = (c.m.r32(sym::gHud)).wrapping_add(48); rt::bf_set(c, a_, 2, 1, (v_) as u64);  };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Hud_ShowNotice", &PROBE_N, &[], true); }
    r_
}

pub fn Hud_ShowCombo(c: &mut Ctx, mut p_on_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Hud_ShowCombo", &PROBE_N, &[(p_on_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((p_on_1) as u32); let a_ = (c.m.r32(sym::gHud)).wrapping_add(48); rt::bf_set(c, a_, 3, 1, (v_) as u64);  };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Hud_ShowCombo", &PROBE_N, &[], true); }
    r_
}

pub fn Hud_ShowPrompt(c: &mut Ctx, mut p_on_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Hud_ShowPrompt", &PROBE_N, &[(p_on_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((p_on_1) as u32); let a_ = (c.m.r32(sym::gHud)).wrapping_add(48); rt::bf_set(c, a_, 4, 1, (v_) as u64);  };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Hud_ShowPrompt", &PROBE_N, &[], true); }
    r_
}

pub fn Hud_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Hud_Init", &PROBE_N, &[], false); }
    let mut l_file_1: u32 = 0;
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, ((56u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gHud; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gHud); F::memset(c, a0_, 0i32, 56u32) }; };
    l_file_1 = { let a_ = (c.m.r32(sym::gCommonRes)).wrapping_add(40); c.m.r32(a_) };
    { let v_ = (l_file_1).wrapping_add(((c.m.r32((l_file_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = ((c.m.r32(sym::gHud)).wrapping_add(28)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let a0_ = ((c.m.r32(sym::gHud)).wrapping_add(28)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let a1_ = { let a_ = ((c.m.r32(sym::gHud)).wrapping_add(28)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a2_ = { let a_ = ((c.m.r32(sym::gHud)).wrapping_add(28)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Res_RelocateOffsets(c, a0_, a1_, a2_) };
    { let v_ = (l_file_1).wrapping_add(((c.m.r32((l_file_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = ((c.m.r32(sym::gHud)).wrapping_add(28)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let a0_ = ((c.m.r32(sym::gHud)).wrapping_add(28)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); let a1_ = { let a_ = ((c.m.r32(sym::gHud)).wrapping_add(28)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a2_ = { let a_ = ((c.m.r32(sym::gHud)).wrapping_add(28)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Res_RelocateOffsets(c, a0_, a1_, a2_) };
    { let v_ = (l_file_1).wrapping_add(((c.m.r32((l_file_1).wrapping_add((((4i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = ((c.m.r32(sym::gHud)).wrapping_add(28)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let a0_ = ((c.m.r32(sym::gHud)).wrapping_add(28)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let a1_ = { let a_ = ((c.m.r32(sym::gHud)).wrapping_add(28)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a2_ = { let a_ = ((c.m.r32(sym::gHud)).wrapping_add(28)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Res_RelocateOffsets(c, a0_, a1_, a2_) };
    { let v_ = (l_file_1).wrapping_add(((c.m.r32((l_file_1).wrapping_add((((5i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = ((c.m.r32(sym::gHud)).wrapping_add(28)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let a0_ = ((c.m.r32(sym::gHud)).wrapping_add(28)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); let a1_ = { let a_ = ((c.m.r32(sym::gHud)).wrapping_add(28)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a2_ = { let a_ = ((c.m.r32(sym::gHud)).wrapping_add(28)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Res_RelocateOffsets(c, a0_, a1_, a2_) };
    { let v_ = (l_file_1).wrapping_add(((c.m.r32((l_file_1).wrapping_add((((3i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = ((c.m.r32(sym::gHud)).wrapping_add(28)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let a0_ = ((c.m.r32(sym::gHud)).wrapping_add(28)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); let a1_ = { let a_ = ((c.m.r32(sym::gHud)).wrapping_add(28)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a2_ = { let a_ = ((c.m.r32(sym::gHud)).wrapping_add(28)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Res_RelocateOffsets(c, a0_, a1_, a2_) };
    { let a0_ = c.m.r32(sym::gHud); let a1_ = { let a_ = ((c.m.r32(sym::gHud)).wrapping_add(28)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::HudGauge_Init(c, a0_, a1_) };
    { let a0_ = (c.m.r32(sym::gHud)).wrapping_add(4); let a1_ = { let a_ = ((c.m.r32(sym::gHud)).wrapping_add(28)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::HudTimer_Init(c, a0_, a1_) };
    { let a0_ = (c.m.r32(sym::gHud)).wrapping_add(8); let a1_ = { let a_ = ((c.m.r32(sym::gHud)).wrapping_add(28)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::HudTeam_Init(c, a0_, a1_) };
    { let a0_ = (c.m.r32(sym::gHud)).wrapping_add(12); let a1_ = { let a_ = ((c.m.r32(sym::gHud)).wrapping_add(28)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::HudNotice_Init(c, a0_, a1_) };
    { let a0_ = (c.m.r32(sym::gHud)).wrapping_add(16); let a1_ = { let a_ = ((c.m.r32(sym::gHud)).wrapping_add(28)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::HudCombo_Init(c, a0_, a1_) };
    { let a0_ = (c.m.r32(sym::gHud)).wrapping_add(20); let a1_ = { let a_ = ((c.m.r32(sym::gHud)).wrapping_add(28)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::HudPrompt_Init(c, a0_, a1_) };
    { let a0_ = (c.m.r32(sym::gHud)).wrapping_add(24); let a1_ = { let a_ = ((c.m.r32(sym::gHud)).wrapping_add(28)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::HudCaption_Init(c, a0_, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Hud_Init", &PROBE_N, &[], true); }
    r_
}

pub fn Hud_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Hud_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::HudGauge_Term(c);
    F::HudTimer_Term(c);
    F::HudTeam_Term(c);
    F::HudNotice_Term(c);
    F::HudCombo_Term(c);
    F::HudPrompt_Term(c);
    F::HudCaption_Term(c);
    { let a0_ = c.m.r32(sym::gHud); F::Heap_Free(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Hud_Term", &PROBE_N, &[], true); }
    r_
}

pub fn Hud_PreUpdate(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Hud_PreUpdate", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(160) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 160);
    let mut l_side_1: i32 = 0;
    let mut l_v_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_mask_4: i32 = 0;
    let mut l_up_5: i32 = 0;
    let mut l_isNew_6: i32 = 0;
    let mut l_n_7: i32 = 0;
    let r_ = 'f: {
        if (((((F::Battle_GetMode(c)) == (3i32)) as i32)) != 0) {
            { let a0_ = F::BtlCtrl_GetWork5A8Count(c); F::HudTimer_SetCount(c, a0_) };
        } else {
            { let a0_ = F::BtlSeq_GetTimeLeft(c); F::HudTimer_SetTime(c, a0_) };
        }
        if ((F::BtlGame_IsReplay(c)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gHud)).wrapping_add(52); (c.m.r32(a_) as i32) }) != (1i32)) as i32)) != 0) {
                F::HudNotice_ShowReplayMark(c, 0i32);
            } else {
                F::HudNotice_ShowReplayMark(c, 1i32);
            }
        } else {
            F::HudNotice_ShowReplayMark(c, 0i32);
        }
        {
            l_side_1 = 0i32;
            'l1: loop {
                if !(((((l_side_1) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a1_ = F::BtlSide_GetHp(c, l_side_1); F::HudGauge_SetHp(c, l_side_1, a1_) };
                    { let a1_ = F::BtlSide_GetKi(c, l_side_1); F::HudGauge_SetKi(c, l_side_1, a1_) };
                    l_v_2 = F::BtlSide_GetParamUnk2C(c, l_side_1);
                    if ((F::BtlSide_TestFlagBE(c, l_side_1)) != 0) {
                        F::HudGauge_SetKiReserve(c, l_side_1, l_v_2);
                    } else {
                        F::HudGauge_SetKiReserve(c, l_side_1, 0i32);
                    }
                    l_mask_4 = 0i32;
                    l_up_5 = 0i32;
                    {
                        l_i_3 = 0i32;
                        'l3: loop {
                            if !(((((l_i_3) < (4i32)) as i32)) != 0) { break 'l3; }
                            'c4: {
                                's5: {
                                    let sv8: i32 = l_i_3;
                                    match sv8 {
                                        0i32 => {
                                            l_v_2 = F::BtlCtrl_GetStatMod0(c, l_side_1);
                                            break 's5;
                                        }
                                        1i32 => {
                                            l_v_2 = F::BtlCtrl_GetStatMod3(c, l_side_1);
                                            break 's5;
                                        }
                                        2i32 => {
                                            l_v_2 = F::BtlCtrl_GetStatMod1(c, l_side_1);
                                            break 's5;
                                        }
                                        3i32 => {
                                            l_v_2 = F::BtlCtrl_GetStatMod2(c, l_side_1);
                                            break 's5;
                                        }
                                        _ => {
                                            break 'c4;
                                        }
                                    }
                                }
                                if (((((l_v_2) != (0i32)) as i32)) != 0) {
                                    { let r_ = (1i32).wrapping_shl((l_i_3) as u32); let n_ = ((l_mask_4) | (r_)); l_mask_4 = n_; };
                                    if (((((l_v_2) > (0i32)) as i32)) != 0) {
                                        { let r_ = (1i32).wrapping_shl((l_i_3) as u32); let n_ = ((l_up_5) | (r_)); l_up_5 = n_; };
                                    }
                                }
                            }
                            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                        }
                    }
                    F::HudGauge_SetStatIcons(c, l_side_1, l_mask_4, l_up_5);
                    { let a1_ = F::BtlSide_GetBlast(c, l_side_1); F::HudGauge_SetBlast(c, l_side_1, a1_) };
                    { let a1_ = F::BtlSide_GetMaxPower(c, l_side_1); F::HudGauge_SetMaxPower(c, l_side_1, a1_) };
                    { let a1_ = F::BtlCtrl_GetObj(c, l_side_1); F::HudGauge_SetFighter(c, l_side_1, a1_) };
                    if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
                        l_isNew_6 = F::BtlSide_IsComboHitNew(c, l_side_1);
                        if (((((l_isNew_6) == (1i32)) as i32)) != 0) {
                            l_v_2 = F::BtlCtrl_GetClashCount(c, l_side_1);
                            if (((((l_v_2) != (0i32)) as i32)) != 0) {
                                F::HudCombo_SetHits(c, l_side_1, ((((l_v_2) as i64 as u64)) as u32));
                            } else {
                                { let a1_ = F::BtlSide_GetComboHits(c, l_side_1); F::HudCombo_SetHits(c, l_side_1, ((((a1_) as i64 as u64)) as u32)) };
                            }
                        }
                        if ((F::BtlCtrl_GetOppUnkD50(c, l_side_1)) != 0) {
                            l_v_2 = F::BtlSide_GetComboDamage(c, l_side_1);
                            if (((((l_v_2) >= (0i32)) as i32)) != 0) {
                                F::HudCombo_SetDamage(c, l_side_1, l_v_2, l_isNew_6);
                            }
                            if (((((l_v_2) >= (9999i32)) as i32)) != 0) {
                                F::HudGauge_ShakeHp(c, (((l_side_1) == (0i32)) as i32), ((15i32) as u16), ((3i32) as u16));
                            } else {
                                if (((((l_v_2) >= (1000i32)) as i32)) != 0) {
                                    F::HudGauge_ShakeHp(c, (((l_side_1) == (0i32)) as i32), ((10i32) as u16), ((2i32) as u16));
                                } else {
                                    F::HudGauge_ShakeHp(c, (((l_side_1) == (0i32)) as i32), ((5i32) as u16), ((2i32) as u16));
                                }
                            }
                        }
                        if ((((!((F::BtlSide_IsComboShown(c, l_side_1)) != 0)) as i32)) != 0) {
                            F::HudCombo_SetHits(c, l_side_1, ((((0i32) as i64 as u64)) as u32));
                            F::HudCombo_SetDamage(c, l_side_1, (1i32).wrapping_neg(), l_isNew_6);
                        }
                    }
                    l_v_2 = F::BtlCtrl_GetFlagDEtoE2(c, l_side_1);
                    if (((((l_v_2) >= (0i32)) as i32)) != 0) {
                        F::HudCombo_SetText(c, l_side_1, l_v_2);
                    }
                    if ((F::BtlCtrl_IsFlag6Raised(c, l_side_1)) != 0) {
                        F::HudCombo_SetMessage(c, l_side_1, 1i32);
                        F::HudGauge_ShakeKi(c, l_side_1, ((8i32) as u16), ((2i32) as u16));
                    }
                    if ((F::BtlCtrl_TestFlag71(c, l_side_1)) != 0) {
                        F::HudCombo_SetMessage(c, l_side_1, 0i32);
                    }
                    if ((F::BtlCtrl_IsFlag5RaisedInFight(c, l_side_1)) != 0) {
                        F::HudCombo_SetMessage(c, l_side_1, 2i32);
                    }
                    if ((F::BtlCtrl_TestFlagE3(c, l_side_1)) != 0) {
                        F::HudCombo_SetMessage(c, l_side_1, 3i32);
                    }
                    if ((F::BtlCtrl_IsFlag9CRaised(c, l_side_1)) != 0) {
                        F::HudCombo_SetMessage(c, l_side_1, 4i32);
                    }
                    {
                        rt::memset(c, fp, 0, 64);
                        { let v_ = 0i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
                        { let v_ = 1i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
                        { let v_ = 2i32; c.m.w32(fp.wrapping_add(8), (v_) as u32); }
                        { let v_ = 3i32; c.m.w32(fp.wrapping_add(12), (v_) as u32); }
                        { let v_ = 0i32; c.m.w32(fp.wrapping_add(16), (v_) as u32); }
                        { let v_ = 1i32; c.m.w32(fp.wrapping_add(20), (v_) as u32); }
                        { let v_ = 2i32; c.m.w32(fp.wrapping_add(24), (v_) as u32); }
                        { let v_ = 3i32; c.m.w32(fp.wrapping_add(28), (v_) as u32); }
                        { let v_ = 0i32; c.m.w32(fp.wrapping_add(32), (v_) as u32); }
                        { let v_ = 1i32; c.m.w32(fp.wrapping_add(36), (v_) as u32); }
                        { let v_ = 2i32; c.m.w32(fp.wrapping_add(40), (v_) as u32); }
                        { let v_ = 3i32; c.m.w32(fp.wrapping_add(44), (v_) as u32); }
                        { let v_ = 0i32; c.m.w32(fp.wrapping_add(48), (v_) as u32); }
                        { let v_ = 1i32; c.m.w32(fp.wrapping_add(52), (v_) as u32); }
                        { let v_ = 2i32; c.m.w32(fp.wrapping_add(56), (v_) as u32); }
                        { let v_ = 3i32; c.m.w32(fp.wrapping_add(60), (v_) as u32); }
                        rt::memset(c, fp.wrapping_add(64), 0, 16);
                        { let v_ = 0i32; c.m.w32(fp.wrapping_add(64).wrapping_add(0), (v_) as u32); }
                        { let v_ = 0i32; c.m.w32(fp.wrapping_add(64).wrapping_add(4), (v_) as u32); }
                        { let v_ = 1i32; c.m.w32(fp.wrapping_add(64).wrapping_add(8), (v_) as u32); }
                        { let v_ = 2i32; c.m.w32(fp.wrapping_add(64).wrapping_add(12), (v_) as u32); }
                        if ((F::BtlCtrl_GetChangePrompt(c, l_side_1, fp.wrapping_add(80), fp.wrapping_add(84), fp.wrapping_add(88))) != 0) {
                            l_n_7 = (c.m.r32(fp.wrapping_add(80)) as i32);
                            { let v_ = { let a_ = (fp).wrapping_add(((((c.m.r32(fp.wrapping_add(80)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = fp.wrapping_add(80); c.m.w32(a_, (v_) as u32); };
                            { let r_ = ({ let a_ = (fp.wrapping_add(64)).wrapping_add(((((c.m.r32(fp.wrapping_add(84)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_mul(4i32); let a_ = fp.wrapping_add(80); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                            { let a1_ = (c.m.r32(fp.wrapping_add(80)) as i32); F::HudPrompt_SetButton(c, l_side_1, a1_, rt::div_i32(l_n_7, 4i32)) };
                        } else {
                            if ((F::BtlCtrl_TestFlagE6Pad1(c, l_side_1)) != 0) {
                                F::HudPrompt_SetButton(c, l_side_1, 12i32, 1i32);
                            } else {
                                if ((F::BtlCtrl_TestFlagE6Pad(c, l_side_1, fp.wrapping_add(88))) != 0) {
                                    { let a2_ = (c.m.r32(fp.wrapping_add(88)) as i32); F::HudPrompt_SetButton(c, l_side_1, 12i32, a2_) };
                                } else {
                                    F::HudPrompt_ClearButton(c, l_side_1);
                                }
                            }
                        }
                    }
                    if ((F::BtlCtrl_IsFlag9ERaised(c, l_side_1)) != 0) {
                        F::HudPrompt_AcceptCommand(c, l_side_1);
                    } else {
                        if ((F::BtlCtrl_GetTechPromptPad1(c, l_side_1, fp.wrapping_add(92), fp.wrapping_add(124), fp.wrapping_add(128))) != 0) {
                            if ((((((c.m.r32(fp.wrapping_add(124)) as i32)) == (3i32)) as i32)) != 0) {
                                { let v_ = (c.m.r32(fp.wrapping_add(124)) as i32); let a_ = (fp.wrapping_add(108)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 5i32; let a_ = (fp.wrapping_add(92)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            } else {
                                { let v_ = (c.m.r32(fp.wrapping_add(124)) as i32); let a_ = (fp.wrapping_add(108)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                { let v_ = ((c.m.r32((fp.wrapping_add(92)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)).wrapping_add(71i32); let a_ = (fp.wrapping_add(92)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            }
                            if ((((((c.m.r32(fp.wrapping_add(124)) as i32)) == (1i32)) as i32)) != 0) {
                                { let v_ = (1i32).wrapping_neg(); let a_ = fp.wrapping_add(128); c.m.w32(a_, (v_) as u32); };
                            }
                            if ((((((c.m.r32(fp.wrapping_add(124)) as i32)) == (3i32)) as i32)) != 0) {
                                { let v_ = (1i32).wrapping_neg(); let a_ = fp.wrapping_add(128); c.m.w32(a_, (v_) as u32); };
                            }
                            { let a4_ = (c.m.r32(fp.wrapping_add(128)) as i32); F::HudPrompt_SetCommand(c, l_side_1, 1i32, fp.wrapping_add(92), fp.wrapping_add(108), a4_) };
                        } else {
                            if ((F::BtlCtrl_GetSwitchPrompt(c, l_side_1, fp.wrapping_add(92), fp.wrapping_add(132), fp.wrapping_add(124), fp.wrapping_add(136), fp.wrapping_add(128))) != 0) {
                                rt::memset(c, fp.wrapping_add(140), 0, 16);
                                { let v_ = 3i32; c.m.w32(fp.wrapping_add(140).wrapping_add(0), (v_) as u32); }
                                { let v_ = 2i32; c.m.w32(fp.wrapping_add(140).wrapping_add(4), (v_) as u32); }
                                { let v_ = 1i32; c.m.w32(fp.wrapping_add(140).wrapping_add(8), (v_) as u32); }
                                { let v_ = 0i32; c.m.w32(fp.wrapping_add(140).wrapping_add(12), (v_) as u32); }
                                {
                                    l_i_3 = ((c.m.r32(fp.wrapping_add(132)) as i32)).wrapping_sub(1i32);
                                    'l6: loop {
                                        if !(((((l_i_3) >= (0i32)) as i32)) != 0) { break 'l6; }
                                        'c7: {
                                            { let v_ = (c.m.r32((fp.wrapping_add(92)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4))) as i32); let a_ = (fp.wrapping_add(92)).wrapping_add(((((l_i_3).wrapping_mul(2i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                        }
                                        { let o_ = l_i_3; let n_ = (o_).wrapping_sub(1); l_i_3 = n_; };
                                    }
                                }
                                {
                                    l_i_3 = 0i32;
                                    'l8: loop {
                                        if !(((((l_i_3) < ((((c.m.r32(fp.wrapping_add(132)) as i32)).wrapping_mul(2i32)).wrapping_sub(1i32))) as i32)) != 0) { break 'l8; }
                                        'c9: {
                                            if ((((l_i_3) & (1i32))) != 0) {
                                                { let v_ = 32i32; let a_ = (fp.wrapping_add(92)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                                { let v_ = 2i32; let a_ = (fp.wrapping_add(108)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                            } else {
                                                if (((((((((c.m.r32((fp.wrapping_add(92)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4))) as i32)).wrapping_sub(16i32)) as u32)) < (((4i32) as u32))) as i32)) != 0) {
                                                    { let v_ = ((c.m.r32((fp.wrapping_add(92)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4))) as i32)).wrapping_add(((c.m.r32(fp.wrapping_add(136)) as i32)).wrapping_mul(4i32)); let a_ = (fp.wrapping_add(92)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                                    { let v_ = 2i32; let a_ = (fp.wrapping_add(108)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                                } else {
                                                    if ((((((((c.m.r32((fp.wrapping_add(92)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4))) as i32)) as u32)) < (((16i32) as u32))) as i32)) != 0) {
                                                        { let v_ = { let a_ = (fp.wrapping_add(140)).wrapping_add(((((c.m.r32(fp.wrapping_add(124)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (fp.wrapping_add(108)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                                    } else {
                                                        { let v_ = 2i32; let a_ = (fp.wrapping_add(108)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                                    }
                                                }
                                            }
                                        }
                                        { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                                    }
                                }
                                { let a1_ = (((c.m.r32(fp.wrapping_add(132)) as i32)).wrapping_mul(2i32)).wrapping_sub(1i32); F::HudPrompt_SetCommand(c, l_side_1, a1_, fp.wrapping_add(92), fp.wrapping_add(108), (1i32).wrapping_neg()) };
                            } else {
                                if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
                                    F::HudPrompt_ClearCommand(c, l_side_1);
                                }
                            }
                        }
                    }
                    if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
                        if ((F::BtlCtrl_TestFlagE4(c, l_side_1)) != 0) {
                            F::HudTeam_StartFlipNext(c, l_side_1);
                        }
                        if ((F::BtlCtrl_TestFlagE5(c, l_side_1)) != 0) {
                            F::HudTeam_StartFlipPrev(c, l_side_1);
                        }
                        l_v_2 = F::BtlSide_CountAlive(c, l_side_1);
                        F::HudTeam_SetReserveCount(c, l_side_1, (l_v_2).wrapping_sub((((l_v_2) > (0i32)) as i32)));
                        { let a1_ = F::BtlSide_GetSwitchGauge(c, l_side_1); F::HudTeam_SetSwitchGauge(c, l_side_1, a1_) };
                        { let a1_ = F::BtlSide_GetSwitchTargetHp(c, l_side_1); F::HudTeam_SetTargetHp(c, l_side_1, a1_) };
                        { let a1_ = F::BtlSide_GetSwitchTargetHpMax(c, l_side_1); F::HudTeam_SetTargetHpMax(c, l_side_1, a1_) };
                        { let a1_ = F::BtlSide_GetSwitchTarget(c, l_side_1); F::HudTeam_SetTarget(c, l_side_1, a1_) };
                        l_v_2 = F::BtlCtrl_IsSwitching(c, l_side_1);
                        F::HudGauge_SetSwitching(c, l_side_1, l_v_2);
                        F::HudTeam_SetSwitching(c, l_side_1, l_v_2);
                    }
                }
                { let o_ = l_side_1; let n_ = (o_).wrapping_add(1); l_side_1 = n_; };
            }
        }
        if ((F::BtlCtrl_TestProgressFrameBit(c)) != 0) {
            F::HudNotice_Show(c, 8i32);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Hud_PreUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn Hud_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Hud_Reset", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::HudGauge_Reset(c);
    F::HudTeam_Reset(c);
    F::HudCombo_Reset(c);
    F::HudNotice_Reset(c);
    F::HudTimer_Reset(c);
    F::HudPrompt_Reset(c);
    F::Hud_ShowAll(c, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Hud_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn Hud_SlideOut(c: &mut Ctx, mut p_seconds_1: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Hud_SlideOut", &PROBE_N, &[(p_seconds_1).0], false); }
    let r_ = 'f: {
    F::HudGauge_SlideOut(c, p_seconds_1);
    F::HudTeam_SlideOut(c, p_seconds_1);
    F::HudTimer_SlideOut(c, p_seconds_1);
    F::HudPrompt_SlideOut(c, p_seconds_1);
    F::HudCombo_SlideOut(c, p_seconds_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Hud_SlideOut", &PROBE_N, &[], true); }
    r_
}

pub fn Hud_SlideIn(c: &mut Ctx, mut p_seconds_1: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Hud_SlideIn", &PROBE_N, &[(p_seconds_1).0], false); }
    let r_ = 'f: {
    F::HudGauge_SlideIn(c, p_seconds_1);
    F::HudTeam_SlideIn(c, p_seconds_1);
    F::HudTimer_SlideIn(c, p_seconds_1);
    F::HudPrompt_SlideIn(c, p_seconds_1);
    F::HudCombo_SlideIn(c, p_seconds_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Hud_SlideIn", &PROBE_N, &[], true); }
    r_
}

pub fn Hud_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Hud_Draw", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Vu0Cur_ResetStack(c);
    F::Vu0Cur_LoadIdentity(c);
    F::HudGfx_SetEnv(c);
    F::Port_GsMarker(c, 18i32);
    if (((((F::Battle_GetMode(c)) == (7i32)) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(24); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
        { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(24); c.m.r32(a_) }; F::HudNode_Draw(c, a0_, 0i32) };
    } else {
        if ((F::BtlGame_IsReplay(c)) != 0) {
            's1: {
                let sv1: i32 = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(52); (c.m.r32(a_) as i32) };
                match sv1 {
                    0i32 => {
                        if (((({ let a_ = (c.m.r32(sym::gHud)).wrapping_add(48); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
                            F::HudTeam_SelectSide(c, 0i32);
                            { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(8); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                            F::HudTeam_SelectSide(c, 1i32);
                            { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(8); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                        }
                        if (((({ let a_ = (c.m.r32(sym::gHud)).wrapping_add(48); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
                            F::HudGfx_ClearAlpha(c);
                            F::HudGauge_SelectSide(c, 0i32);
                            { let a0_ = { let a_ = c.m.r32(sym::gHud); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                            F::HudGfx_ClearAlpha(c);
                            F::HudGauge_SelectSide(c, 1i32);
                            { let a0_ = { let a_ = c.m.r32(sym::gHud); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                        } else {
                            F::HudNotice_ShowReplayMark(c, 0i32);
                        }
                        if (((({ let a_ = (c.m.r32(sym::gHud)).wrapping_add(48); c.m.r32(a_) }) & (((2i32) as u32)))) != 0) {
                            { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(4); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                        }
                        if (((({ let a_ = (c.m.r32(sym::gHud)).wrapping_add(48); c.m.r32(a_) }) & (((16i32) as u32)))) != 0) {
                            F::HudPrompt_SelectSide(c, 0i32);
                            { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(20); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                            if ((F::Battle_IsSplitScreen(c)) != 0) {
                                F::HudPrompt_SelectSide(c, 1i32);
                                { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(20); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                            }
                        }
                        if (((({ let a_ = (c.m.r32(sym::gHud)).wrapping_add(48); c.m.r32(a_) }) & (((8i32) as u32)))) != 0) {
                            F::HudCombo_SelectSide(c, 0i32);
                            { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(16); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                            F::HudCombo_SelectSide(c, 1i32);
                            { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(16); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                        }
                        if (((({ let a_ = (c.m.r32(sym::gHud)).wrapping_add(48); c.m.r32(a_) }) & (((4i32) as u32)))) != 0) {
                            { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(12); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                        }
                        { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(24); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                        { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(24); c.m.r32(a_) }; F::HudNode_Draw(c, a0_, 0i32) };
                        break 's1;
                    }
                    1i32 => {
                        if (((({ let a_ = (c.m.r32(sym::gHud)).wrapping_add(48); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
                            F::HudTeam_SelectSide(c, 0i32);
                            { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(8); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                            { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(8); c.m.r32(a_) }; F::HudNode_Draw(c, a0_, 0i32) };
                            F::HudTeam_SelectSide(c, 1i32);
                            { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(8); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                            { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(8); c.m.r32(a_) }; F::HudNode_Draw(c, a0_, 0i32) };
                        }
                        if (((({ let a_ = (c.m.r32(sym::gHud)).wrapping_add(48); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
                            F::HudGfx_ClearAlpha(c);
                            F::HudGauge_SelectSide(c, 0i32);
                            { let a0_ = { let a_ = c.m.r32(sym::gHud); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                            { let a0_ = { let a_ = c.m.r32(sym::gHud); c.m.r32(a_) }; F::HudNode_Draw(c, a0_, 0i32) };
                            F::HudGfx_ClearAlpha(c);
                            F::HudGauge_SelectSide(c, 1i32);
                            { let a0_ = { let a_ = c.m.r32(sym::gHud); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                            { let a0_ = { let a_ = c.m.r32(sym::gHud); c.m.r32(a_) }; F::HudNode_Draw(c, a0_, 0i32) };
                        } else {
                            F::HudNotice_ShowReplayMark(c, 0i32);
                        }
                        if (((({ let a_ = (c.m.r32(sym::gHud)).wrapping_add(48); c.m.r32(a_) }) & (((2i32) as u32)))) != 0) {
                            { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(4); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                            { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(4); c.m.r32(a_) }; F::HudNode_Draw(c, a0_, 0i32) };
                        }
                        if (((({ let a_ = (c.m.r32(sym::gHud)).wrapping_add(48); c.m.r32(a_) }) & (((16i32) as u32)))) != 0) {
                            F::HudPrompt_SelectSide(c, 0i32);
                            { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(20); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                            { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(20); c.m.r32(a_) }; F::HudNode_Draw(c, a0_, 0i32) };
                            if ((F::Battle_IsSplitScreen(c)) != 0) {
                                F::HudPrompt_SelectSide(c, 1i32);
                                { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(20); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                                { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(20); c.m.r32(a_) }; F::HudNode_Draw(c, a0_, 0i32) };
                            }
                            F::HudPrompt_DrawNames(c);
                        }
                        if (((({ let a_ = (c.m.r32(sym::gHud)).wrapping_add(48); c.m.r32(a_) }) & (((8i32) as u32)))) != 0) {
                            F::HudCombo_SelectSide(c, 0i32);
                            { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(16); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                            { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(16); c.m.r32(a_) }; F::HudNode_Draw(c, a0_, 0i32) };
                            F::HudCombo_SelectSide(c, 1i32);
                            { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(16); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                            { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(16); c.m.r32(a_) }; F::HudNode_Draw(c, a0_, 0i32) };
                            F::HudCombo_DrawText(c);
                        }
                        if (((({ let a_ = (c.m.r32(sym::gHud)).wrapping_add(48); c.m.r32(a_) }) & (((4i32) as u32)))) != 0) {
                            { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(12); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                            { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(12); c.m.r32(a_) }; F::HudNode_Draw(c, a0_, 0i32) };
                        }
                        { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(24); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                        break 's1;
                    }
                    2i32 => {
                        if (((({ let a_ = (c.m.r32(sym::gHud)).wrapping_add(48); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
                            F::HudTeam_SelectSide(c, 0i32);
                            { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(8); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                            F::HudTeam_SelectSide(c, 1i32);
                            { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(8); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                        }
                        if (((({ let a_ = (c.m.r32(sym::gHud)).wrapping_add(48); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
                            F::HudGfx_ClearAlpha(c);
                            F::HudGauge_SelectSide(c, 0i32);
                            { let a0_ = { let a_ = c.m.r32(sym::gHud); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                            F::HudGfx_ClearAlpha(c);
                            F::HudGauge_SelectSide(c, 1i32);
                            { let a0_ = { let a_ = c.m.r32(sym::gHud); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                        } else {
                            F::HudNotice_ShowReplayMark(c, 0i32);
                        }
                        if (((({ let a_ = (c.m.r32(sym::gHud)).wrapping_add(48); c.m.r32(a_) }) & (((2i32) as u32)))) != 0) {
                            { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(4); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                        }
                        if (((({ let a_ = (c.m.r32(sym::gHud)).wrapping_add(48); c.m.r32(a_) }) & (((16i32) as u32)))) != 0) {
                            F::HudPrompt_SelectSide(c, 0i32);
                            { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(20); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                            if ((F::Battle_IsSplitScreen(c)) != 0) {
                                F::HudPrompt_SelectSide(c, 1i32);
                                { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(20); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                            }
                        }
                        if (((({ let a_ = (c.m.r32(sym::gHud)).wrapping_add(48); c.m.r32(a_) }) & (((8i32) as u32)))) != 0) {
                            F::HudCombo_SelectSide(c, 0i32);
                            { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(16); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                            F::HudCombo_SelectSide(c, 1i32);
                            { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(16); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                        }
                        if (((({ let a_ = (c.m.r32(sym::gHud)).wrapping_add(48); c.m.r32(a_) }) & (((4i32) as u32)))) != 0) {
                            { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(12); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                        }
                        { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(24); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
        } else {
            if (((((F::Battle_GetMode(c)) != (1i32)) as i32)) != 0) {
                if (((({ let a_ = (c.m.r32(sym::gHud)).wrapping_add(48); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
                    F::HudTeam_SelectSide(c, 0i32);
                    { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(8); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                    { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(8); c.m.r32(a_) }; F::HudNode_Draw(c, a0_, 0i32) };
                    F::HudTeam_SelectSide(c, 1i32);
                    { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(8); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                    { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(8); c.m.r32(a_) }; F::HudNode_Draw(c, a0_, 0i32) };
                }
            }
            if (((({ let a_ = (c.m.r32(sym::gHud)).wrapping_add(48); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
                F::HudGfx_ClearAlpha(c);
                F::HudGauge_SelectSide(c, 0i32);
                { let a0_ = { let a_ = c.m.r32(sym::gHud); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                { let a0_ = { let a_ = c.m.r32(sym::gHud); c.m.r32(a_) }; F::HudNode_Draw(c, a0_, 0i32) };
                F::HudGfx_ClearAlpha(c);
                F::HudGauge_SelectSide(c, 1i32);
                { let a0_ = { let a_ = c.m.r32(sym::gHud); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                { let a0_ = { let a_ = c.m.r32(sym::gHud); c.m.r32(a_) }; F::HudNode_Draw(c, a0_, 0i32) };
            }
            if (((({ let a_ = (c.m.r32(sym::gHud)).wrapping_add(48); c.m.r32(a_) }) & (((2i32) as u32)))) != 0) {
                { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(4); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(4); c.m.r32(a_) }; F::HudNode_Draw(c, a0_, 0i32) };
            }
            if (((({ let a_ = (c.m.r32(sym::gHud)).wrapping_add(48); c.m.r32(a_) }) & (((16i32) as u32)))) != 0) {
                F::HudPrompt_SelectSide(c, 0i32);
                { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(20); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(20); c.m.r32(a_) }; F::HudNode_Draw(c, a0_, 0i32) };
                if ((F::Battle_IsSplitScreen(c)) != 0) {
                    F::HudPrompt_SelectSide(c, 1i32);
                    { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(20); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                    { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(20); c.m.r32(a_) }; F::HudNode_Draw(c, a0_, 0i32) };
                }
                F::HudPrompt_DrawNames(c);
            }
            if (((({ let a_ = (c.m.r32(sym::gHud)).wrapping_add(48); c.m.r32(a_) }) & (((8i32) as u32)))) != 0) {
                F::HudCombo_SelectSide(c, 0i32);
                { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(16); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(16); c.m.r32(a_) }; F::HudNode_Draw(c, a0_, 0i32) };
                F::HudCombo_SelectSide(c, 1i32);
                { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(16); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(16); c.m.r32(a_) }; F::HudNode_Draw(c, a0_, 0i32) };
                F::HudCombo_DrawText(c);
            }
            if (((({ let a_ = (c.m.r32(sym::gHud)).wrapping_add(48); c.m.r32(a_) }) & (((4i32) as u32)))) != 0) {
                { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(12); c.m.r32(a_) }; F::HudNode_Update(c, a0_) };
                { let a0_ = { let a_ = (c.m.r32(sym::gHud)).wrapping_add(12); c.m.r32(a_) }; F::HudNode_Draw(c, a0_, 0i32) };
            }
        }
    }
    F::Hud_ClearTexMarks(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Hud_Draw", &PROBE_N, &[], true); }
    r_
}
