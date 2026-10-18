#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EftSpr_IsOffScreen(c: &mut Ctx, mut p_x_1: i32, mut p_y_2: i32, mut p_z_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSpr_IsOffScreen", &PROBE_N, &[(p_x_1) as u32, (p_y_2) as u32, (p_z_3) as u32], false); }
    let r_ = 'f: {
    if (((((p_z_3) <= (0i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    if (((((p_x_1) > (65519i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    if (((((p_x_1) <= (0i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    if (((((p_y_2) > (65519i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    if (((((p_y_2) <= (0i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSpr_IsOffScreen", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprAnim_InitPool(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_InitPool", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, (((((32i32) as u32)).wrapping_mul(1552u32)) as i32), ((((32i32) as i64 as u64)) as u32), 0i32, 2i32); let a_ = sym::gEftSprAnimPool; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gEftSprAnimPool); F::memset(c, a0_, 0i32, (((32i32) as u32)).wrapping_mul(1552u32)) }; };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_InitPool", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprAnim_TermPool(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_TermPool", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32(sym::gEftSprAnimPool); F::Heap_Free(c, a0_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftSprAnimPool; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_TermPool", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprAnim_AllocSlot(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_AllocSlot", &PROBE_N, &[], false); }
    let mut l_used_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_used_1 = (c.m.r32(sym::gEftSprAnimPool)).wrapping_add(1536);
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (32i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32(l_used_1) as i32)) == (0i32)) as i32)) != 0) {
                    { let v_ = 1i32; let a_ = l_used_1; c.m.w32(a_, (v_) as u32); };
                    break 'f (c.m.r32(sym::gEftSprAnimPool)).wrapping_add((((l_i_2) as u32)).wrapping_mul(1552));
                }
                l_used_1 = (l_used_1).wrapping_add(1552u32);
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_AllocSlot", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprAnim_FreeSlot(c: &mut Ctx, mut p_slot_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_FreeSlot", &PROBE_N, &[(p_slot_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = (p_slot_1).wrapping_add(1536); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_FreeSlot", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprAnim_Create(c: &mut Ctx, mut p_anim_1: u32, mut p_pack_2: u32, mut p_tex_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_Create", &PROBE_N, &[(p_anim_1) as u32, (p_pack_2) as u32, (p_tex_3) as u32], false); }
    let r_ = 'f: {
    { let _ = F::memset(c, p_anim_1, 0i32, 80u32); };
    { let v_ = p_pack_2; let a_ = p_anim_1; c.m.w32(a_, v_); };
    { let v_ = p_tex_3; let a_ = (p_anim_1).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = F::EftSprAnim_AllocSlot(c); let a_ = (p_anim_1).wrapping_add(8); c.m.w32(a_, v_); };
    { let v_ = 1i32; let a_ = (c.m.r32((p_anim_1).wrapping_add(8))).wrapping_add(1540); c.m.w32(a_, (v_) as u32); };
    F::Vec4_Set(c, (p_anim_1).wrapping_add(16), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x3f800000));
    F::Vec4_Set(c, (p_anim_1).wrapping_add(32), F32(0x00000000), F32(0x00000000), F32(0xbf800000), F32(0x3f800000));
    F::Vec4_Set(c, (p_anim_1).wrapping_add(48), F32(0x3f800000), F32(0x3f800000), F32(0x3f800000), F32(0x3f800000));
    F::Vec4_Set(c, (p_anim_1).wrapping_add(64), F32(0x3f800000), F32(0x3f800000), F32(0x3f800000), F32(0x3f800000));
    { let a0_ = c.m.r32(p_anim_1); F::EftSprPack_Bind(c, a0_) };
    F::EftSprAnim_Start(c, p_anim_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_Create", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprAnim_Destroy(c: &mut Ctx, mut p_anim_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_Destroy", &PROBE_N, &[(p_anim_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32((p_anim_1).wrapping_add(8)); F::EftSprAnim_FreeSlot(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_Destroy", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprAnim_Start(c: &mut Ctx, mut p_anim_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_Start", &PROBE_N, &[(p_anim_1) as u32], false); }
    let mut l_pack_2: u32 = 0;
    let mut l_def_3: u32 = 0;
    let mut l_n_4: i32 = 0;
    let mut l_st_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_key_7: u32 = 0;
    let r_ = 'f: {
    l_pack_2 = { let a0_ = c.m.r32(p_anim_1); F::EftSprPack_Get(c, a0_) };
    l_def_3 = F::EftSprPack_GetLayers(c, l_pack_2);
    l_n_4 = F::EftSprPack_GetLayerCount(c, l_pack_2);
    l_st_5 = c.m.r32((p_anim_1).wrapping_add(8));
    {
        l_i_6 = 0i32;
        'l1: loop {
            if !(((((l_i_6) < (l_n_4)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = l_def_3; let a_ = (l_st_5).wrapping_add(80); c.m.w32(a_, v_); };
                { let v_ = F32(0x00000000); let a_ = l_st_5; c.m.w32(a_, (v_).0); };
                { let v_ = rt::i2f(((((c.m.r16((l_def_3).wrapping_add(8)) as i16)) as i32)).wrapping_sub(1i32)); let a_ = (l_st_5).wrapping_add(12); c.m.w32(a_, (v_).0); };
                { let v_ = rt::i2f((c.m.r32((l_def_3).wrapping_add(24)) as i32)); let a_ = (l_st_5).wrapping_add(16); c.m.w32(a_, (v_).0); };
                { let v_ = rt::i2f((c.m.r32((l_def_3).wrapping_add(24)) as i32)); let a_ = (l_st_5).wrapping_add(20); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = (l_st_5).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = (l_st_5).wrapping_add(44); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = (l_st_5).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = rt::i2f((c.m.r8((l_def_3).wrapping_add(3))) as i32); let a_ = (l_st_5).wrapping_add(24); c.m.w32(a_, (v_).0); };
                { let v_ = 0i32; let a_ = (l_st_5).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_st_5).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                if ((((c.m.r32((l_def_3).wrapping_add(16))) & (((1i32) as u32)))) != 0) {
                    { let v_ = rt::i2f(F::EftSprPack_GetLoopStartFrame(c, l_def_3)); let a_ = l_st_5; c.m.w32(a_, (v_).0); };
                    { let v_ = rt::i2f({ let a0_ = c.m.r32((l_st_5).wrapping_add(80)); F::EftSprPack_GetLoopEndFrame(c, a0_) }); let a_ = (l_st_5).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = (({ let a_ = (c.m.r32((l_st_5).wrapping_add(80))).wrapping_add(7); (c.m.r8(a_) as i8) }) as i32); let a_ = (l_st_5).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                }
                if (((({ let a_ = (c.m.r32((l_st_5).wrapping_add(80))).wrapping_add(16); c.m.r32(a_) }) & (((64i32) as u32)))) != 0) {
                    l_key_7 = { let a_ = (c.m.r32((l_st_5).wrapping_add(80))).wrapping_add(20); c.m.r32(a_) };
                    F::Vec4_Set(c, (l_st_5).wrapping_add(48), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x3f800000));
                    { let a1_ = F32(c.m.r32((l_key_7).wrapping_add(16))); let a2_ = F32(c.m.r32(((l_key_7).wrapping_add(16)).wrapping_add(4))); let a3_ = F32(c.m.r32(((l_key_7).wrapping_add(16)).wrapping_add(8))); F::Vec4_Set(c, (l_st_5).wrapping_add(64), a1_, a2_, a3_, F32(0x3f800000)) };
                }
                { let o_ = l_st_5; let n_ = (o_).wrapping_add(96); l_st_5 = n_; };
                { let o_ = l_def_3; let n_ = (o_).wrapping_add(32); l_def_3 = n_; };
            }
            { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_Start", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprAnim_Step(c: &mut Ctx, mut p_anim_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_Step", &PROBE_N, &[(p_anim_1) as u32], false); }
    let mut l_quads_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_pack_4: u32 = 0;
    let mut l_st_5: u32 = 0;
    let r_ = 'f: {
    l_quads_2 = 0i32;
    l_i_3 = 0i32;
    l_pack_4 = { let a0_ = c.m.r32(p_anim_1); F::EftSprPack_Get(c, a0_) };
    l_st_5 = c.m.r32((p_anim_1).wrapping_add(8));
    {
        'l1: loop {
            if !(((((l_i_3) < (((c.m.r8((l_pack_4).wrapping_add(4))) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let r_ = rt::fadd(F32(c.m.r32((l_st_5).wrapping_add(24))), F32(c.m.r32((l_st_5).wrapping_add(24)))); let a_ = (l_st_5).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = rt::fadd(F32(c.m.r32((l_st_5).wrapping_add(24))), F32(c.m.r32((l_st_5).wrapping_add(24)))); let a_ = (l_st_5).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let v_ = F::Port_Rand(c); let a_ = (l_st_5).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                { let v_ = F::Port_Rand(c); let a_ = (l_st_5).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                F::Vec3_Add(c, (l_st_5).wrapping_add(48), (l_st_5).wrapping_add(48), (l_st_5).wrapping_add(64));
                F::EftSprAnim_WrapDegrees(c, (l_st_5).wrapping_add(48));
                if (((rt::fge(F32(c.m.r32((l_st_5).wrapping_add(4))), F32(c.m.r32((l_st_5).wrapping_add(12)))) as i32)) != 0) {
                    if ((((((((((c.m.r32((l_st_5).wrapping_add(28)) as i32)) > (0i32)) as i32)) != 0) || ((((((c.m.r32((l_st_5).wrapping_add(28)) as i32)) == ((1i32).wrapping_neg())) as i32)) != 0)) as i32)) != 0) {
                        { let v_ = F32(c.m.r32(l_st_5)); let a_ = (l_st_5).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        if ((((((c.m.r32((l_st_5).wrapping_add(28)) as i32)) != ((1i32).wrapping_neg())) as i32)) != 0) {
                            { let a_ = (l_st_5).wrapping_add(28); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                        }
                    } else {
                        { let v_ = F32(c.m.r32((l_st_5).wrapping_add(12))); let a_ = (l_st_5).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        if (((rt::fgt(F32(c.m.r32((l_st_5).wrapping_add(20))), F32(0x00000000)) as i32)) != 0) {
                            { let r_ = F32(0x3f800000); let a_ = (l_st_5).wrapping_add(20); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            { let v_ = rt::fdiv(F32(c.m.r32((l_st_5).wrapping_add(20))), F32(c.m.r32((l_st_5).wrapping_add(16)))); let a_ = (l_st_5).wrapping_add(44); c.m.w32(a_, (v_).0); };
                            if (((rt::fle(F32(c.m.r32((l_st_5).wrapping_add(20))), F32(0x00000000)) as i32)) != 0) {
                                { let v_ = F32(0x00000000); let a_ = (l_st_5).wrapping_add(44); c.m.w32(a_, (v_).0); };
                            }
                        }
                        if (((rt::fle(F32(c.m.r32((l_st_5).wrapping_add(20))), F32(0x00000000)) as i32)) != 0) {
                            { let r_ = 1i32; let a_ = (l_st_5).wrapping_add(40); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                    }
                }
                if (((({ let a_ = (c.m.r32((l_st_5).wrapping_add(80))).wrapping_add(16); c.m.r32(a_) }) & (((8i32) as u32)))) != 0) {
                    { let r_ = 4i32; let n_ = (l_quads_2).wrapping_add(r_); l_quads_2 = n_; };
                } else {
                    { let r_ = 1i32; let n_ = (l_quads_2).wrapping_add(r_); l_quads_2 = n_; };
                }
                { let o_ = l_st_5; let n_ = (o_).wrapping_add(96); l_st_5 = n_; };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    if ((((((((c.m.r8((l_pack_4).wrapping_add(5))) as i32)) != 0) && ((((!((F::EftSprAnim_IsPlaying(c, p_anim_1)) != 0)) as i32)) != 0)) as i32)) != 0) {
        F::EftSprAnim_Restart(c, p_anim_1);
    }
    break 'f l_quads_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_Step", &PROBE_N, &[], true); }
    r_
}

pub fn eft_ae__EftSprLayerDef_TestFlag(c: &mut Ctx, mut p_def_1: u32, mut p_flag_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_ae__EftSprLayerDef_TestFlag", &PROBE_N, &[(p_def_1) as u32, (p_flag_2) as u32], false); }
    let r_ = 'f: {
    break 'f (((((c.m.r32((p_def_1).wrapping_add(16))) & (((p_flag_2) as u32)))) != (((0i32) as u32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "eft_ae__EftSprLayerDef_TestFlag", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprAnim_Draw(c: &mut Ctx, mut p_anim_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_Draw", &PROBE_N, &[(p_anim_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(144) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 144);
    let mut l_ok_2: i32 = 0;
    let mut l_pack_3: u32 = 0;
    let mut l_st_4: u32 = 0;
    let mut l_m_5: u32 = 0;
    let mut l_mode_6: i32 = 0;
    let mut l_i_7: i32 = 0;
    let r_ = 'f: {
        l_i_7 = 0i32;
        F::Mtx_StoreIdentity(c, fp.wrapping_add(16));
        F::Mtx_StoreIdentity(c, fp.wrapping_add(80));
        l_ok_2 = F::EftSprAnim_BuildMatrices(c, p_anim_1, fp.wrapping_add(16), fp.wrapping_add(80));
        F::Vu0Cur_Push(c);
        { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
        l_pack_3 = { let a0_ = c.m.r32(p_anim_1); F::EftSprPack_Get(c, a0_) };
        l_st_4 = c.m.r32((p_anim_1).wrapping_add(8));
        {
            'l1: loop {
                if !(((((l_i_7) < (((c.m.r8((l_pack_3).wrapping_add(4))) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_m_5 = fp.wrapping_add(16);
                    l_mode_6 = 0i32;
                    if (({ let a0_ = c.m.r32((l_st_4).wrapping_add(80)); F::eft_ae__EftSprLayerDef_TestFlag(c, a0_, 4i32) }) != 0) {
                        l_m_5 = fp.wrapping_add(80);
                    }
                    if (({ let a0_ = c.m.r32((l_st_4).wrapping_add(80)); F::eft_ae__EftSprLayerDef_TestFlag(c, a0_, 2i32) }) != 0) {
                        l_mode_6 = ((({ let a_ = (c.m.r32((p_anim_1).wrapping_add(8))).wrapping_add(1540); (c.m.r32(a_) as i32) }) != (0i32)) as i32);
                    }
                    if (((((c.m.r32((p_anim_1).wrapping_add(12)) as i32)) & (1i32))) != 0) {
                        l_m_5 = fp.wrapping_add(80);
                    }
                    if (((((c.m.r32((p_anim_1).wrapping_add(12)) as i32)) & (2i32))) != 0) {
                        l_m_5 = fp.wrapping_add(16);
                    }
                    if (((((c.m.r32((p_anim_1).wrapping_add(12)) as i32)) & (4i32))) != 0) {
                        l_mode_6 = 1i32;
                    }
                    if (((((c.m.r32((p_anim_1).wrapping_add(12)) as i32)) & (8i32))) != 0) {
                        l_mode_6 = 0i32;
                    }
                    if (((((c.m.r32((p_anim_1).wrapping_add(12)) as i32)) & (16i32))) != 0) {
                        l_mode_6 = 2i32;
                    }
                    F::Vec4_Copy(c, fp, (p_anim_1).wrapping_add(64));
                    { let r_ = F32(c.m.r32((l_st_4).wrapping_add(44))); let a_ = (fp).wrapping_add(12); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if ((((((l_ok_2) != 0) || (((((l_m_5) != (fp.wrapping_add(80))) as i32)) != 0)) as i32)) != 0) {
                        { let a0_ = c.m.r32((l_st_4).wrapping_add(80)); let a1_ = rt::f2i(F32(c.m.r32((l_st_4).wrapping_add(4)))); let a2_ = rt::f2i(F32(c.m.r32((l_st_4).wrapping_add(8)))); let a3_ = c.m.r32((p_anim_1).wrapping_add(4)); let a10_ = (c.m.r32((l_st_4).wrapping_add(32)) as i32); let a11_ = (c.m.r32((l_st_4).wrapping_add(36)) as i32); F::EftSprAnim_DrawLayer(c, a0_, a1_, a2_, a3_, l_m_5, fp.wrapping_add(16), (p_anim_1).wrapping_add(16), (l_st_4).wrapping_add(48), fp, l_mode_6, a10_, a11_) };
                        { let o_ = l_st_4; let n_ = (o_).wrapping_add(96); l_st_4 = n_; };
                    }
                }
                { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
            }
        }
        F::Vu0Cur_Pop(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprAnim_IsPlaying(c: &mut Ctx, mut p_anim_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_IsPlaying", &PROBE_N, &[(p_anim_1) as u32], false); }
    let mut l_pack_2: u32 = 0;
    let mut l_st_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_pack_2 = { let a0_ = c.m.r32(p_anim_1); F::EftSprPack_Get(c, a0_) };
    l_st_3 = c.m.r32((p_anim_1).wrapping_add(8));
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (((c.m.r8((l_pack_2).wrapping_add(4))) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((!(((((c.m.r32((l_st_3).wrapping_add(40)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
                    break 'f 1i32;
                }
                { let o_ = l_st_3; let n_ = (o_).wrapping_add(96); l_st_3 = n_; };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_IsPlaying", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprAnim_SetPos(c: &mut Ctx, mut p_anim_1: u32, mut p_pos_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_SetPos", &PROBE_N, &[(p_anim_1) as u32, (p_pos_2) as u32], false); }
    let r_ = 'f: {
    { let s_ = p_pos_2; let d_ = (p_anim_1).wrapping_add(16); rt::memcpy(c, d_, s_, 16); };
    { let v_ = F32(0x3f800000); let a_ = ((p_anim_1).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_SetPos", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprAnim_SetDir(c: &mut Ctx, mut p_anim_1: u32, mut p_dir_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_SetDir", &PROBE_N, &[(p_anim_1) as u32, (p_dir_2) as u32], false); }
    let r_ = 'f: {
    { let s_ = p_dir_2; let d_ = (p_anim_1).wrapping_add(32); rt::memcpy(c, d_, s_, 16); };
    { let v_ = F32(0x3f800000); let a_ = ((p_anim_1).wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_SetDir", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprAnim_SetScale(c: &mut Ctx, mut p_anim_1: u32, mut p_scale_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_SetScale", &PROBE_N, &[(p_anim_1) as u32, (p_scale_2) as u32], false); }
    let r_ = 'f: {
    { let s_ = p_scale_2; let d_ = (p_anim_1).wrapping_add(48); rt::memcpy(c, d_, s_, 16); };
    { let v_ = F32(0x3f800000); let a_ = ((p_anim_1).wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_SetScale", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprAnim_SetColor(c: &mut Ctx, mut p_anim_1: u32, mut p_color_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_SetColor", &PROBE_N, &[(p_anim_1) as u32, (p_color_2) as u32], false); }
    let r_ = 'f: {
    { let s_ = p_color_2; let d_ = (p_anim_1).wrapping_add(64); rt::memcpy(c, d_, s_, 16); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_SetColor", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprAnim_SetSlotFlag(c: &mut Ctx, mut p_anim_1: u32, mut p_flag_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_SetSlotFlag", &PROBE_N, &[(p_anim_1) as u32, (p_flag_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_flag_2; let a_ = (c.m.r32((p_anim_1).wrapping_add(8))).wrapping_add(1540); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_SetSlotFlag", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprAnim_SetDrawFlags(c: &mut Ctx, mut p_anim_1: u32, mut p_flags_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_SetDrawFlags", &PROBE_N, &[(p_anim_1) as u32, (p_flags_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_flags_2; let a_ = (p_anim_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_SetDrawFlags", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprAnim_KeepTextures(c: &mut Ctx, mut p_anim_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_KeepTextures", &PROBE_N, &[(p_anim_1) as u32], false); }
    let mut l_pack_2: u32 = 0;
    let mut l_tex_3: u32 = 0;
    let mut l_st_4: u32 = 0;
    let mut l_def_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let r_ = 'f: {
    l_pack_2 = { let a0_ = c.m.r32(p_anim_1); F::EftSprPack_Get(c, a0_) };
    l_tex_3 = c.m.r32((p_anim_1).wrapping_add(4));
    l_st_4 = c.m.r32((p_anim_1).wrapping_add(8));
    {
        l_i_6 = 0i32;
        'l1: loop {
            if !(((((l_i_6) < (((c.m.r8((l_pack_2).wrapping_add(4))) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                l_def_5 = c.m.r32((l_st_4).wrapping_add(80));
                { let o_ = l_st_4; let n_ = (o_).wrapping_add(96); l_st_4 = n_; };
                if ((((!((((c.m.r32((l_tex_3).wrapping_add(516))) & ((((1i32).wrapping_shl((((c.m.r8(l_def_5)) as i32)) as u32)) as u32)))) != 0)) as i32)) != 0) {
                    { let v_ = { let a0_ = (l_tex_3).wrapping_add((((c.m.r8(l_def_5)) as u32)).wrapping_mul(16)); F::EftVram_AddTex(c, a0_, 1i32, 0i32) }; let a_ = (l_tex_3).wrapping_add((((c.m.r8(l_def_5)) as u32)).wrapping_mul(16)); c.m.w64(a_, v_); };
                    { let r_ = (((1i32).wrapping_shl((((c.m.r8(l_def_5)) as i32)) as u32)) as u32); let a_ = (l_tex_3).wrapping_add(516); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                }
            }
            { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_KeepTextures", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprAnim_Restart(c: &mut Ctx, mut p_anim_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_Restart", &PROBE_N, &[(p_anim_1) as u32], false); }
    let mut l_pack_2: u32 = 0;
    let mut l_def_3: u32 = 0;
    let mut l_n_4: i32 = 0;
    let mut l_st_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let r_ = 'f: {
    l_pack_2 = { let a0_ = c.m.r32(p_anim_1); F::EftSprPack_Get(c, a0_) };
    l_def_3 = F::EftSprPack_GetLayers(c, l_pack_2);
    l_n_4 = F::EftSprPack_GetLayerCount(c, l_pack_2);
    l_st_5 = c.m.r32((p_anim_1).wrapping_add(8));
    {
        l_i_6 = 0i32;
        'l1: loop {
            if !(((((l_i_6) < (l_n_4)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = l_def_3; let a_ = (l_st_5).wrapping_add(80); c.m.w32(a_, v_); };
                { let v_ = F32(0x00000000); let a_ = l_st_5; c.m.w32(a_, (v_).0); };
                { let v_ = rt::i2f(((((c.m.r16((l_def_3).wrapping_add(8)) as i16)) as i32)).wrapping_sub(1i32)); let a_ = (l_st_5).wrapping_add(12); c.m.w32(a_, (v_).0); };
                { let v_ = rt::i2f((c.m.r32((l_def_3).wrapping_add(24)) as i32)); let a_ = (l_st_5).wrapping_add(16); c.m.w32(a_, (v_).0); };
                { let v_ = rt::i2f((c.m.r32((l_def_3).wrapping_add(24)) as i32)); let a_ = (l_st_5).wrapping_add(20); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = (l_st_5).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = (l_st_5).wrapping_add(44); c.m.w32(a_, (v_).0); };
                { let v_ = rt::i2f((c.m.r8((l_def_3).wrapping_add(3))) as i32); let a_ = (l_st_5).wrapping_add(24); c.m.w32(a_, (v_).0); };
                { let v_ = 0i32; let a_ = (l_st_5).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_st_5).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                if ((((c.m.r32((l_def_3).wrapping_add(16))) & (((1i32) as u32)))) != 0) {
                    { let v_ = rt::i2f(F::EftSprPack_GetLoopStartFrame(c, l_def_3)); let a_ = l_st_5; c.m.w32(a_, (v_).0); };
                    { let v_ = rt::i2f(F::EftSprPack_GetLoopEndFrame(c, l_def_3)); let a_ = (l_st_5).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = (((c.m.r8((l_def_3).wrapping_add(7)) as i8)) as i32); let a_ = (l_st_5).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                }
                { let o_ = l_st_5; let n_ = (o_).wrapping_add(96); l_st_5 = n_; };
                { let o_ = l_def_3; let n_ = (o_).wrapping_add(32); l_def_3 = n_; };
            }
            { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_Restart", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprAnim_WrapDegrees(c: &mut Ctx, mut p_v_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_WrapDegrees", &PROBE_N, &[(p_v_1) as u32], false); }
    let r_ = 'f: {
    if (((rt::fgt(F32(c.m.r32(p_v_1)), F32(0x43b40000)) as i32)) != 0) {
        { let r_ = F32(0x43b40000); let a_ = p_v_1; let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
    if (((rt::flt(F32(c.m.r32(p_v_1)), F32(0x00000000)) as i32)) != 0) {
        { let r_ = F32(0x43b40000); let a_ = p_v_1; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
    if (((rt::fgt(F32(c.m.r32((p_v_1).wrapping_add(4))), F32(0x43b40000)) as i32)) != 0) {
        { let r_ = F32(0x43b40000); let a_ = (p_v_1).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
    if (((rt::flt(F32(c.m.r32((p_v_1).wrapping_add(4))), F32(0x00000000)) as i32)) != 0) {
        { let r_ = F32(0x43b40000); let a_ = (p_v_1).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
    if (((rt::fgt(F32(c.m.r32((p_v_1).wrapping_add(8))), F32(0x43b40000)) as i32)) != 0) {
        { let r_ = F32(0x43b40000); let a_ = (p_v_1).wrapping_add(8); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
    if (((rt::flt(F32(c.m.r32((p_v_1).wrapping_add(8))), F32(0x00000000)) as i32)) != 0) {
        { let r_ = F32(0x43b40000); let a_ = (p_v_1).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_WrapDegrees", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprAnim_BuildDirMtx(c: &mut Ctx, mut p_out_1: u32, mut p_dir_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_BuildDirMtx", &PROBE_N, &[(p_out_1) as u32, (p_dir_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let r_ = 'f: {
        F::Vec4_Set(c, fp, F32(0x00000000), F32(0xbf800000), F32(0x00000000), F32(0x00000000));
        if (((((((((((rt::feq(F32(c.m.r32(p_dir_2)), F32(0x00000000)) as i32)) != 0) && (((rt::feq(F32(c.m.r32((p_dir_2).wrapping_add(4))), F32(0xbf800000)) as i32)) != 0)) as i32)) != 0) && (((rt::feq(F32(c.m.r32((p_dir_2).wrapping_add(8))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
            F::Vec4_Set(c, fp, F32(0x00000000), F32(0x00000000), F32(0xbf800000), F32(0x00000000));
        }
        if (((((((((((rt::feq(F32(c.m.r32(p_dir_2)), F32(0x00000000)) as i32)) != 0) && (((rt::feq(F32(c.m.r32((p_dir_2).wrapping_add(4))), F32(0xbf800000)) as i32)) != 0)) as i32)) != 0) && (((rt::feq(F32(c.m.r32((p_dir_2).wrapping_add(8))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
            F::Vec4_Set(c, fp, F32(0x00000000), F32(0x00000000), F32(0x3f800000), F32(0x00000000));
        }
        F::Vec3_Normalize(c, fp.wrapping_add(32), p_dir_2);
        if (((((((((((rt::feq(F32(c.m.r32(fp.wrapping_add(32))), F32(0x00000000)) as i32)) != 0) && (((rt::feq(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) && (((rt::feq(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        F::Vec3_Cross(c, fp.wrapping_add(16), fp, fp.wrapping_add(32));
        F::Vec3_Normalize(c, fp.wrapping_add(16), fp.wrapping_add(16));
        F::Vec3_Cross(c, fp, fp.wrapping_add(16), fp.wrapping_add(32));
        F::Vec3_Normalize(c, fp, fp);
        F::Mtx_StoreIdentity(c, p_out_1);
        { let v_ = F32(c.m.r32(fp.wrapping_add(16))); let a_ = ((p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); let a_ = ((p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); let a_ = ((p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(fp)); let a_ = ((p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp).wrapping_add(4))); let a_ = ((p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp).wrapping_add(8))); let a_ = ((p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(fp.wrapping_add(32))); let a_ = ((p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4))); let a_ = ((p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8))); let a_ = ((p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_BuildDirMtx", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprAnim_BuildMatrices(c: &mut Ctx, mut p_anim_1: u32, mut p_oriented_2: u32, mut p_billboard_3: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_BuildMatrices", &PROBE_N, &[(p_anim_1) as u32, (p_oriented_2) as u32, (p_billboard_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(192) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 192);
    let r_ = 'f: {
        F::Mtx_StoreIdentity(c, p_oriented_2);
        F::Mtx_StoreIdentity(c, fp.wrapping_add(64));
        F::Mtx_StoreIdentity(c, fp);
        F::Mtx_StoreIdentity(c, fp.wrapping_add(128));
        { let _ = F::EftSprAnim_BuildDirMtx(c, fp, (p_anim_1).wrapping_add(32)); };
        F::Mtx_ScaleDiag(c, fp.wrapping_add(64), fp.wrapping_add(64), (p_anim_1).wrapping_add(48));
        F::Mtx_Mul(c, fp.wrapping_add(128), fp, fp.wrapping_add(64));
        { let s_ = fp.wrapping_add(128); let d_ = p_oriented_2; rt::memcpy(c, d_, s_, 64); };
        { let v_ = { let v_ = { let v_ = F32(0x00000000); let a_ = ((p_oriented_2).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); v_ }; let a_ = ((p_oriented_2).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); v_ }; let a_ = ((p_oriented_2).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(64); F::Mtx_InverseRT(c, p_billboard_3, a1_) };
        { let v_ = { let v_ = { let v_ = F32(0x00000000); let a_ = ((p_billboard_3).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); v_ }; let a_ = ((p_billboard_3).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); v_ }; let a_ = ((p_billboard_3).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        F::Mtx_Mul(c, p_billboard_3, p_billboard_3, fp.wrapping_add(64));
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_BuildMatrices", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprAnim_DrawLayer(c: &mut Ctx, mut p_def_1: u32, mut p_frame_2: i32, mut p_time_3: i32, mut p_tex_4: u32, mut p_m_5: u32, mut p_oriented_6: u32, mut p_pos_7: u32, mut p_spin_8: u32, mut p_color_9: u32, mut p_mode_10: i32, mut p_rand1_11: i32, mut p_rand2_12: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_DrawLayer", &PROBE_N, &[(p_def_1) as u32, (p_frame_2) as u32, (p_time_3) as u32, (p_tex_4) as u32, (p_m_5) as u32, (p_oriented_6) as u32, (p_pos_7) as u32, (p_spin_8) as u32, (p_color_9) as u32, (p_mode_10) as u32, (p_rand1_11) as u32, (p_rand2_12) as u32], false); }
    let r_ = 'f: {
    F::EftSprAnim_DrawLayerAt(c, p_m_5, p_oriented_6, p_pos_7, p_def_1, p_frame_2, p_time_3, p_tex_4, p_spin_8, p_color_9, p_mode_10, p_rand1_11, p_rand2_12);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_DrawLayer", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprAnim_DrawLayerAt(c: &mut Ctx, mut p_m_1: u32, mut p_oriented_2: u32, mut p_objPos_3: u32, mut p_def_4: u32, mut p_frame_5: i32, mut p_time_6: i32, mut p_tex_7: u32, mut p_spin_8: u32, mut p_color_9: u32, mut p_mode_10: i32, mut p_rand1_11: i32, mut p_rand2_12: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_DrawLayerAt", &PROBE_N, &[(p_m_1) as u32, (p_oriented_2) as u32, (p_objPos_3) as u32, (p_def_4) as u32, (p_frame_5) as u32, (p_time_6) as u32, (p_tex_7) as u32, (p_spin_8) as u32, (p_color_9) as u32, (p_mode_10) as u32, (p_rand1_11) as u32, (p_rand2_12) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(448) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 448);
    let mut l_k_13: u32 = 0;
    let mut l_kn_14: u32 = 0;
    let mut l_p_15: u32 = 0;
    let mut l_e_16: u32 = 0;
    let mut l_t_17: F32 = F32(0);
    let mut l_key_18: i32 = 0;
    let mut l_cell_19: i32 = 0;
    let mut l_z_20: i32 = 0;
    let mut l_otLayer_21: i32 = 0;
    let mut l_lay_22: i32 = 0;
    let mut l_i_23: i32 = 0;
    let mut l_on_24: i32 = 0;
    let mut l_off_25: i32 = 0;
    let mut l_step_26: i32 = 0;
    let mut l_n_27: i32 = 0;
    let mut l_step_28: i32 = 0;
    let mut l_on_29: i32 = 0;
    let mut l_off_30: i32 = 0;
    let r_ = 'f: {
        l_key_18 = F::EftSprPack_FindKey(c, p_def_4, p_frame_5);
        if (((((l_key_18) == ((1i32).wrapping_neg())) as i32)) != 0) {
            break 'f;
        }
        l_t_17 = F::EftSprPack_GetKeyT(c, p_def_4, p_frame_5, l_key_18);
        { let a1_ = (c.m.r32((p_def_4).wrapping_add(20))).wrapping_add(((((l_key_18).wrapping_add(1i32)) as u32)).wrapping_mul(64)); let a2_ = (c.m.r32((p_def_4).wrapping_add(20))).wrapping_add((((l_key_18) as u32)).wrapping_mul(64)); F::Vec3_Lerp(c, fp, a1_, a2_, l_t_17) };
        { let a1_ = ((c.m.r32((p_def_4).wrapping_add(20))).wrapping_add(((((l_key_18).wrapping_add(1i32)) as u32)).wrapping_mul(64))).wrapping_add(16); let a2_ = ((c.m.r32((p_def_4).wrapping_add(20))).wrapping_add((((l_key_18) as u32)).wrapping_mul(64))).wrapping_add(16); F::Vec3_Lerp(c, fp.wrapping_add(16), a1_, a2_, l_t_17) };
        { let a1_ = ((c.m.r32((p_def_4).wrapping_add(20))).wrapping_add(((((l_key_18).wrapping_add(1i32)) as u32)).wrapping_mul(64))).wrapping_add(32); let a2_ = ((c.m.r32((p_def_4).wrapping_add(20))).wrapping_add((((l_key_18) as u32)).wrapping_mul(64))).wrapping_add(32); F::Vec3_Lerp(c, fp.wrapping_add(32), a1_, a2_, l_t_17) };
        l_k_13 = ((((l_key_18).wrapping_shl((6i32) as u32)) as u32)).wrapping_add(c.m.r32((p_def_4).wrapping_add(20)));
        l_kn_14 = (l_k_13).wrapping_add((((1i32) as u32)).wrapping_mul(64));
        { let v_ = rt::i2f((c.m.r8(((l_kn_14).wrapping_add(48)).wrapping_add(((0i32) as u32)))) as i32); let a_ = fp.wrapping_add(64); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f((c.m.r8(((l_kn_14).wrapping_add(48)).wrapping_add(((1i32) as u32)))) as i32); let a_ = (fp.wrapping_add(64)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f((c.m.r8(((l_kn_14).wrapping_add(48)).wrapping_add(((2i32) as u32)))) as i32); let a_ = (fp.wrapping_add(64)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f((c.m.r8(((l_kn_14).wrapping_add(48)).wrapping_add(((3i32) as u32)))) as i32); let a_ = (fp.wrapping_add(64)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f((c.m.r8((((l_k_13).wrapping_add((((0i32) as u32)).wrapping_mul(64))).wrapping_add(48)).wrapping_add(((0i32) as u32)))) as i32); let a_ = fp.wrapping_add(80); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f((c.m.r8((((l_k_13).wrapping_add((((0i32) as u32)).wrapping_mul(64))).wrapping_add(48)).wrapping_add(((1i32) as u32)))) as i32); let a_ = (fp.wrapping_add(80)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f((c.m.r8((((l_k_13).wrapping_add((((0i32) as u32)).wrapping_mul(64))).wrapping_add(48)).wrapping_add(((2i32) as u32)))) as i32); let a_ = (fp.wrapping_add(80)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f((c.m.r8((((l_k_13).wrapping_add((((0i32) as u32)).wrapping_mul(64))).wrapping_add(48)).wrapping_add(((3i32) as u32)))) as i32); let a_ = (fp.wrapping_add(80)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec4_Lerp(c, fp.wrapping_add(48), fp.wrapping_add(64), fp.wrapping_add(80), l_t_17);
        F::Vec4_Mul(c, fp.wrapping_add(48), fp.wrapping_add(48), p_color_9);
        if ((((c.m.r32((p_def_4).wrapping_add(16))) & (((((((256i32) | (512i32))) | (1024i32))) as u32)))) != 0) {
            F::EftSprAnim_Jitter(c, fp.wrapping_add(32), p_def_4, p_frame_5, p_rand1_11, p_rand2_12);
        }
        if ((((c.m.r32((p_def_4).wrapping_add(16))) & (((64i32) as u32)))) != 0) {
            { let s_ = p_spin_8; let d_ = fp.wrapping_add(16); rt::memcpy(c, d_, s_, 16); };
        }
        { let v_ = { let a0_ = rt::fdiv(rt::fmul(F32(c.m.r32(fp.wrapping_add(16))), F32(0x40490fda)), F32(0x43340000)); F::EftMath_WrapAngle(c, a0_) }; let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = rt::fdiv(rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(0x40490fda)), F32(0x43340000)); F::EftMath_WrapAngle(c, a0_) }; let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = rt::fdiv(rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(0x40490fda)), F32(0x43340000)); F::EftMath_WrapAngle(c, a0_) }; let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        F::Mtx_StoreIdentity(c, fp.wrapping_add(288));
        F::Mtx_ScaleDiag(c, fp.wrapping_add(288), fp.wrapping_add(288), fp.wrapping_add(32));
        F::Mtx_RotateXYZ(c, fp.wrapping_add(288), fp.wrapping_add(288), fp.wrapping_add(16));
        F::Mtx_Mul(c, fp.wrapping_add(352), p_m_1, fp.wrapping_add(288));
        F::Mtx_MulVec4(c, fp.wrapping_add(416), p_oriented_2, fp);
        F::Vec3_Add(c, fp.wrapping_add(416), fp.wrapping_add(416), p_objPos_3);
        F::Vec4_Set(c, (fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), F32(0xc0a00000), F32(0xc0a00000), F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), F32(0x40a00000), F32(0xc0a00000), F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), F32(0xc0a00000), F32(0x40a00000), F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(96)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), F32(0x40a00000), F32(0x40a00000), F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), F32(0x00000000), F32(0x00000000), F32(0x3f800000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(160)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), F32(0x3f800000), F32(0x00000000), F32(0x3f800000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(160)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), F32(0x00000000), F32(0x3f800000), F32(0x3f800000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(160)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), F32(0x3f800000), F32(0x3f800000), F32(0x3f800000), F32(0x3f800000));
        { let a1_ = F32(c.m.r32(fp.wrapping_add(48))); let a2_ = F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))); let a3_ = F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(8))); let a4_ = F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(12))); F::Vec4_Set(c, (fp.wrapping_add(224)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, a4_) };
        { let a1_ = F32(c.m.r32(fp.wrapping_add(48))); let a2_ = F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))); let a3_ = F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(8))); let a4_ = F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(12))); F::Vec4_Set(c, (fp.wrapping_add(224)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, a4_) };
        { let a1_ = F32(c.m.r32(fp.wrapping_add(48))); let a2_ = F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))); let a3_ = F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(8))); let a4_ = F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(12))); F::Vec4_Set(c, (fp.wrapping_add(224)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, a4_) };
        { let a1_ = F32(c.m.r32(fp.wrapping_add(48))); let a2_ = F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))); let a3_ = F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(8))); let a4_ = F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(12))); F::Vec4_Set(c, (fp.wrapping_add(224)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, a4_) };
        {
            l_i_23 = 0i32;
            'l1: loop {
                if !(((((l_i_23) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::Mtx_MulVec4(c, (fp.wrapping_add(96)).wrapping_add((((l_i_23) as u32)).wrapping_mul(16)), fp.wrapping_add(352), (fp.wrapping_add(96)).wrapping_add((((l_i_23) as u32)).wrapping_mul(16)));
                    F::Vec3_Add(c, (fp.wrapping_add(96)).wrapping_add((((l_i_23) as u32)).wrapping_mul(16)), (fp.wrapping_add(96)).wrapping_add((((l_i_23) as u32)).wrapping_mul(16)), fp.wrapping_add(416));
                    { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(96)).wrapping_add((((l_i_23) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_23; let n_ = (o_).wrapping_add(1); l_i_23 = n_; };
            }
        }
        l_cell_19 = 0i32;
        if ((((c.m.r32((p_def_4).wrapping_add(16))) & (((16i32) as u32)))) != 0) {
            l_on_24 = (((c.m.r8((p_def_4).wrapping_add(13)) as i8)) as i32);
            l_off_25 = (((c.m.r8((p_def_4).wrapping_add(14)) as i8)) as i32);
            l_step_26 = (((c.m.r8((p_def_4).wrapping_add(15)) as i8)) as i32);
            if ((((((((((l_on_24).wrapping_add(l_off_25)) != (0i32)) as i32)) != 0) && (((((rt::rem_i32((p_time_6).wrapping_shr((1i32) as u32), (l_on_24).wrapping_add(l_off_25))) >= (l_on_24)) as i32)) != 0)) as i32)) != 0) {
                break 'f;
            }
            l_n_27 = { let a0_ = (((c.m.r8((p_def_4).wrapping_add(12)) as i8)) as i32); F::EftSprAnim_GetGridCells(c, a0_) };
            if ((((c.m.r32((p_def_4).wrapping_add(16))) & (((32i32) as u32)))) != 0) {
                l_cell_19 = rt::rem_i32(p_rand1_11, l_n_27);
            } else {
                if (((((l_step_26) != (0i32)) as i32)) != 0) {
                    l_cell_19 = rt::rem_i32(rt::div_i32((p_time_6).wrapping_shr((1i32) as u32), l_step_26), l_n_27);
                }
            }
            { let a1_ = (((c.m.r8((p_def_4).wrapping_add(12)) as i8)) as i32); F::EftSprAnim_SetGridUv(c, fp.wrapping_add(160), a1_, l_cell_19) };
        } else {
            if ((((c.m.r32((p_def_4).wrapping_add(16))) & (((128i32) as u32)))) != 0) {
                l_step_28 = (((c.m.r8((p_def_4).wrapping_add(15)) as i8)) as i32);
                if (((((l_step_28) > (0i32)) as i32)) != 0) {
                    l_on_29 = ((((c.m.r8((p_def_4).wrapping_add(13)) as i8)) as i32)).wrapping_add(rt::rem_i32(p_rand1_11, l_step_28));
                    l_off_30 = ((((c.m.r8((p_def_4).wrapping_add(14)) as i8)) as i32)).wrapping_add(rt::rem_i32(p_rand2_12, l_step_28));
                    if ((((((((((l_on_29).wrapping_add(l_off_30)) != (0i32)) as i32)) != 0) && (((((rt::rem_i32((p_frame_5).wrapping_shr((1i32) as u32), (l_on_29).wrapping_add(l_off_30))) >= (l_on_29)) as i32)) != 0)) as i32)) != 0) {
                        break 'f;
                    }
                }
            }
        }
        { let _ = F::Vu0Cur_ProjectPoint(c, fp.wrapping_add(432), fp.wrapping_add(416)); };
        l_z_20 = ((c.m.r32((fp.wrapping_add(432)).wrapping_add(8)) as i32)).wrapping_shr((8i32) as u32);
        if (((((p_mode_10) == (2i32)) as i32)) != 0) {
            l_z_20 = 4095i32;
        }
        l_otLayer_21 = ((c.m.r8((p_def_4).wrapping_add(2))) as i32);
        l_lay_22 = l_otLayer_21;
        if (((((l_otLayer_21) >= (2i32)) as i32)) != 0) {
            l_lay_22 = (l_otLayer_21).wrapping_sub(2i32);
        }
        if (((((l_z_20) < (0i32)) as i32)) != 0) {
            l_e_16 = ((c.m.r32(sym::gOtZ)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((l_lay_22) as u32)).wrapping_mul(8));
        } else {
            if (((((l_z_20) >= (4096i32)) as i32)) != 0) {
                l_e_16 = ((c.m.r32(sym::gOtZ)).wrapping_add((((4095i32) as u32)).wrapping_mul(16))).wrapping_add((((l_lay_22) as u32)).wrapping_mul(8));
            } else {
                l_e_16 = ((c.m.r32(sym::gOtZ)).wrapping_add((((l_z_20) as u32)).wrapping_mul(16))).wrapping_add((((l_lay_22) as u32)).wrapping_mul(8));
            }
        }
        l_p_15 = c.m.r32(sym::gOtCur);
        { let v_ = (l_p_15).wrapping_add((((1i32) as u32)).wrapping_mul(64)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
        { let v_ = ((536870915i32) as u32); let a_ = l_p_15; c.m.w32(a_, v_); };
        { let v_ = ((268435456i32) as u32); let a_ = (l_p_15).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = ((1342177283i32) as u32); let a_ = (l_p_15).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = ((1152921504606879746i64) as u64); let a_ = (l_p_15).wrapping_add(16); c.m.w64(a_, v_); };
        { let v_ = ((14i32) as u64); let a_ = (l_p_15).wrapping_add(24); c.m.w64(a_, v_); };
        { let v_ = ((8i32) as u64); let a_ = (l_p_15).wrapping_add(40); c.m.w64(a_, v_); };
        { let v_ = ((9i32) as u64); let a_ = (l_p_15).wrapping_add(56); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (l_p_15).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u64); let a_ = (l_p_15).wrapping_add(32); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u64); let a_ = (l_p_15).wrapping_add(48); c.m.w64(a_, v_); };
        { let v_ = l_p_15; let a_ = (c.m.r32((l_e_16).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = l_p_15; let a_ = (l_e_16).wrapping_add(4); c.m.w32(a_, v_); };
        if ((((c.m.r32((p_def_4).wrapping_add(16))) & (((8i32) as u32)))) != 0) {
            { let a0_ = { let a_ = (p_tex_7).wrapping_add((((c.m.r8(p_def_4)) as u32)).wrapping_mul(16)); c.m.r64(a_) }; F::EftSprAnim_DrawQuadSubdiv(c, a0_, fp.wrapping_add(96), fp.wrapping_add(160), fp.wrapping_add(224), l_otLayer_21, l_z_20, p_mode_10) };
        } else {
            { let a0_ = { let a_ = (p_tex_7).wrapping_add((((c.m.r8(p_def_4)) as u32)).wrapping_mul(16)); c.m.r64(a_) }; F::EftSprAnim_DrawQuad(c, a0_, fp.wrapping_add(96), fp.wrapping_add(160), fp.wrapping_add(224), l_otLayer_21, l_z_20, p_mode_10) };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_DrawLayerAt", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprAnim_Jitter(c: &mut Ctx, mut p_scale_1: u32, mut p_def_2: u32, mut p_frame_3: i32, mut p_rand1_4: i32, mut p_rand2_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_Jitter", &PROBE_N, &[(p_scale_1) as u32, (p_def_2) as u32, (p_frame_3) as u32, (p_rand1_4) as u32, (p_rand2_5) as u32], false); }
    let mut l_on_6: i32 = 0;
    let mut l_d_7: F32 = F32(0);
    let r_ = 'f: {
    l_on_6 = 0i32;
    if ((((((((c.m.r16((p_def_2).wrapping_add(10)) as i16)) as i32)) > (0i32)) as i32)) != 0) {
        l_on_6 = (((rt::rem_i32(p_frame_3, (((c.m.r16((p_def_2).wrapping_add(10)) as i16)) as i32))) == (0i32)) as i32);
    }
    if ((((((((((((c.m.r32((p_def_2).wrapping_add(16))) & (((256i32) as u32)))) != 0) && ((((((((c.m.r16((p_def_2).wrapping_add(28)) as i16)) as i32)) > (0i32)) as i32)) != 0)) as i32)) != 0) && ((l_on_6) != 0)) as i32)) != 0) {
        l_d_7 = rt::fmul(rt::i2f(rt::rem_i32(p_rand1_4, (((c.m.r16((p_def_2).wrapping_add(28)) as i16)) as i32))), F32(0x3a83126e));
        { let r_ = l_d_7; let a_ = p_scale_1; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = l_d_7; let a_ = (p_scale_1).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
    if ((((((((((((c.m.r32((p_def_2).wrapping_add(16))) & (((512i32) as u32)))) != 0) && ((((((((c.m.r16((p_def_2).wrapping_add(28)) as i16)) as i32)) > (0i32)) as i32)) != 0)) as i32)) != 0) && ((l_on_6) != 0)) as i32)) != 0) {
        { let r_ = rt::fmul(rt::i2f(rt::rem_i32(p_rand1_4, (((c.m.r16((p_def_2).wrapping_add(28)) as i16)) as i32))), F32(0x3a83126e)); let a_ = p_scale_1; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
    if ((((((((((((c.m.r32((p_def_2).wrapping_add(16))) & (((1024i32) as u32)))) != 0) && ((((((((c.m.r16((p_def_2).wrapping_add(30)) as i16)) as i32)) > (0i32)) as i32)) != 0)) as i32)) != 0) && ((l_on_6) != 0)) as i32)) != 0) {
        { let r_ = rt::fmul(rt::i2f(rt::rem_i32(p_rand2_5, (((c.m.r16((p_def_2).wrapping_add(30)) as i16)) as i32))), F32(0x3a83126e)); let a_ = (p_scale_1).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_Jitter", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprAnim_GetGridCells(c: &mut Ctx, mut p_grid_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_GetGridCells", &PROBE_N, &[(p_grid_1) as u32], false); }
    let mut l_n_2: i32 = 0;
    let r_ = 'f: {
    l_n_2 = 0i32;
    's1: {
        let sv3: i32 = p_grid_1;
        match sv3 {
            0i32 => {
                l_n_2 = 4i32;
                break 's1;
            }
            1i32 => {
                l_n_2 = 8i32;
                break 's1;
            }
            2i32 => {
                l_n_2 = 16i32;
                break 's1;
            }
            3i32 => {
                l_n_2 = 9i32;
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f l_n_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_GetGridCells", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprAnim_SetGridUv(c: &mut Ctx, mut p_uv_1: u32, mut p_grid_2: i32, mut p_cell_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_SetGridUv", &PROBE_N, &[(p_uv_1) as u32, (p_grid_2) as u32, (p_cell_3) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv4: i32 = p_grid_2;
        match sv4 {
            0i32 => {
                { let v_ = F32(c.m.r32(((sym::gEftSprGridUv2x2).wrapping_add((((p_cell_3) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_uv_1).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((sym::gEftSprGridUv2x2).wrapping_add((((p_cell_3) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((p_uv_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((sym::gEftSprGridUv2x2).wrapping_add((((p_cell_3) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (p_uv_1).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((sym::gEftSprGridUv2x2).wrapping_add((((p_cell_3) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((p_uv_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((sym::gEftSprGridUv2x2).wrapping_add((((p_cell_3) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_uv_1).wrapping_add((((2i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((sym::gEftSprGridUv2x2).wrapping_add((((p_cell_3) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)))); let a_ = ((p_uv_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((sym::gEftSprGridUv2x2).wrapping_add((((p_cell_3) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (p_uv_1).wrapping_add((((3i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((sym::gEftSprGridUv2x2).wrapping_add((((p_cell_3) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)))); let a_ = ((p_uv_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                break 's1;
            }
            1i32 => {
                { let v_ = F32(c.m.r32(((sym::gEftSprGridUv4x2).wrapping_add((((p_cell_3) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_uv_1).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((sym::gEftSprGridUv4x2).wrapping_add((((p_cell_3) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((p_uv_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((sym::gEftSprGridUv4x2).wrapping_add((((p_cell_3) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (p_uv_1).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((sym::gEftSprGridUv4x2).wrapping_add((((p_cell_3) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((p_uv_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((sym::gEftSprGridUv4x2).wrapping_add((((p_cell_3) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_uv_1).wrapping_add((((2i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((sym::gEftSprGridUv4x2).wrapping_add((((p_cell_3) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)))); let a_ = ((p_uv_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((sym::gEftSprGridUv4x2).wrapping_add((((p_cell_3) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (p_uv_1).wrapping_add((((3i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((sym::gEftSprGridUv4x2).wrapping_add((((p_cell_3) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)))); let a_ = ((p_uv_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                break 's1;
            }
            2i32 => {
                { let v_ = F32(c.m.r32(((sym::gEftSprGridUv4x4).wrapping_add((((p_cell_3) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_uv_1).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((sym::gEftSprGridUv4x4).wrapping_add((((p_cell_3) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((p_uv_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((sym::gEftSprGridUv4x4).wrapping_add((((p_cell_3) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (p_uv_1).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((sym::gEftSprGridUv4x4).wrapping_add((((p_cell_3) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((p_uv_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((sym::gEftSprGridUv4x4).wrapping_add((((p_cell_3) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_uv_1).wrapping_add((((2i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((sym::gEftSprGridUv4x4).wrapping_add((((p_cell_3) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)))); let a_ = ((p_uv_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((sym::gEftSprGridUv4x4).wrapping_add((((p_cell_3) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (p_uv_1).wrapping_add((((3i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((sym::gEftSprGridUv4x4).wrapping_add((((p_cell_3) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)))); let a_ = ((p_uv_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                break 's1;
            }
            3i32 => {
                { let v_ = F32(c.m.r32(((sym::gEftSprGridUv3x3).wrapping_add((((p_cell_3) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_uv_1).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((sym::gEftSprGridUv3x3).wrapping_add((((p_cell_3) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((p_uv_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((sym::gEftSprGridUv3x3).wrapping_add((((p_cell_3) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (p_uv_1).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((sym::gEftSprGridUv3x3).wrapping_add((((p_cell_3) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((p_uv_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((sym::gEftSprGridUv3x3).wrapping_add((((p_cell_3) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_uv_1).wrapping_add((((2i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((sym::gEftSprGridUv3x3).wrapping_add((((p_cell_3) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)))); let a_ = ((p_uv_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((sym::gEftSprGridUv3x3).wrapping_add((((p_cell_3) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (p_uv_1).wrapping_add((((3i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32(((sym::gEftSprGridUv3x3).wrapping_add((((p_cell_3) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)))); let a_ = ((p_uv_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_SetGridUv", &PROBE_N, &[], true); }
    r_
}

pub fn eft_ae__EftSprAnim_QueueTri(c: &mut Ctx, mut p_v0_1: u32, mut p_v1_2: u32, mut p_v2_3: u32, mut p_c0_4: u32, mut p_c1_5: u32, mut p_c2_6: u32, mut p_uv0_7: u32, mut p_uv1_8: u32, mut p_uv2_9: u32, mut p_otLayer_10: i32, mut p_z_11: i32, mut p_tex0_12: u64, mut p_mode_13: i32, mut p_ctx_14: i8) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_ae__EftSprAnim_QueueTri", &PROBE_N, &[(p_v0_1) as u32, (p_v1_2) as u32, (p_v2_3) as u32, (p_c0_4) as u32, (p_c1_5) as u32, (p_c2_6) as u32, (p_uv0_7) as u32, (p_uv1_8) as u32, (p_uv2_9) as u32, (p_otLayer_10) as u32, (p_z_11) as u32, (p_tex0_12) as u32, (p_mode_13) as u32, (p_ctx_14) as u32], false); }
    let mut l_abe_15: i32 = 0;
    let mut l_l_16: i32 = 0;
    let mut l_p_17: u32 = 0;
    let mut l_e_18: u32 = 0;
    let r_ = 'f: {
    l_abe_15 = 1i32;
    if (((((((((((rt::fle(F32(c.m.r32((p_c0_4).wrapping_add(12))), F32(0x00000000)) as i32)) != 0) && (((rt::fle(F32(c.m.r32((p_c1_5).wrapping_add(12))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) && (((rt::fle(F32(c.m.r32((p_c2_6).wrapping_add(12))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
        break 'f;
    }
    l_p_17 = c.m.r32(sym::gOtCur);
    { let v_ = (l_p_17).wrapping_add((((1i32) as u32)).wrapping_mul(144)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
    { let v_ = (((((((l_abe_15) as u64)).wrapping_shl((6i32) as u32)) | ((((p_ctx_14) as u64)).wrapping_shl((9i32) as u32)))) | (((27i32) as u64))); let a_ = (l_p_17).wrapping_add(32); c.m.w64(a_, v_); };
    { let v_ = ((((((p_ctx_14) as u64)).wrapping_shl((4i32) as u32)).wrapping_add((((p_ctx_14) as u64)).wrapping_shl((48i32) as u32))).wrapping_add((((p_ctx_14) as u64)).wrapping_shl((8i32) as u32))).wrapping_add(((69878448449001568i64) as u64)); let a_ = (l_p_17).wrapping_add(24); c.m.w64(a_, v_); };
    { let v_ = ((1342177288i32) as u32); let a_ = (l_p_17).wrapping_add(12); c.m.w32(a_, v_); };
    { let v_ = ((536870920i32) as u32); let a_ = l_p_17; c.m.w32(a_, v_); };
    { let v_ = ((268435456i32) as u32); let a_ = (l_p_17).wrapping_add(8); c.m.w32(a_, v_); };
    { let v_ = 16429131440647602177u64; let a_ = (l_p_17).wrapping_add(16); c.m.w64(a_, v_); };
    { let v_ = ((5i32) as u64); let a_ = (l_p_17).wrapping_add(48); c.m.w64(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_17).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u64); let a_ = (l_p_17).wrapping_add(136); c.m.w64(a_, v_); };
    { let v_ = ((0i32) as u64); let a_ = (l_p_17).wrapping_add(128); c.m.w64(a_, v_); };
    { let v_ = ((rt::f2i(F32(c.m.r32(p_c0_4)))) as u8); let a_ = (((l_p_17).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2i(F32(c.m.r32((p_c0_4).wrapping_add(4))))) as u8); let a_ = (((l_p_17).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2i(F32(c.m.r32((p_c0_4).wrapping_add(8))))) as u8); let a_ = (((l_p_17).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2i(F32(c.m.r32((p_c0_4).wrapping_add(12))))) as u8); let a_ = (((l_p_17).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = F32(c.m.r32((p_uv0_7).wrapping_add(8))); let a_ = (((l_p_17).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = ((rt::f2i(F32(c.m.r32(p_c1_5)))) as u8); let a_ = (((l_p_17).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2i(F32(c.m.r32((p_c1_5).wrapping_add(4))))) as u8); let a_ = (((l_p_17).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2i(F32(c.m.r32((p_c1_5).wrapping_add(8))))) as u8); let a_ = (((l_p_17).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2i(F32(c.m.r32((p_c1_5).wrapping_add(12))))) as u8); let a_ = (((l_p_17).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = F32(c.m.r32((p_uv1_8).wrapping_add(8))); let a_ = (((l_p_17).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = ((rt::f2i(F32(c.m.r32(p_c2_6)))) as u8); let a_ = (((l_p_17).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2i(F32(c.m.r32((p_c2_6).wrapping_add(4))))) as u8); let a_ = (((l_p_17).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2i(F32(c.m.r32((p_c2_6).wrapping_add(8))))) as u8); let a_ = (((l_p_17).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2i(F32(c.m.r32((p_c2_6).wrapping_add(12))))) as u8); let a_ = (((l_p_17).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = F32(c.m.r32((p_uv2_9).wrapping_add(8))); let a_ = (((l_p_17).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = p_tex0_12; let a_ = (l_p_17).wrapping_add(40); c.m.w64(a_, v_); };
    { let v_ = F32(c.m.r32(p_uv0_7)); let a_ = (((l_p_17).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_uv0_7).wrapping_add(4))); let a_ = (((l_p_17).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(p_uv1_8)); let a_ = (((l_p_17).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_uv1_8).wrapping_add(4))); let a_ = (((l_p_17).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(p_uv2_9)); let a_ = (((l_p_17).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_uv2_9).wrapping_add(4))); let a_ = (((l_p_17).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
    if (((((p_mode_13) != (0i32)) as i32)) != 0) {
        { let v_ = (((c.m.r32(p_v0_1) as i32)) as u64); let a_ = (((l_p_17).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32((p_v0_1).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_17).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((16777215i32) as u64); let a_ = ((((l_p_17).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_17).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32(p_v1_2) as i32)) as u64); let a_ = (((l_p_17).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32((p_v1_2).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_17).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((16777215i32) as u64); let a_ = ((((l_p_17).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_17).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32(p_v2_3) as i32)) as u64); let a_ = (((l_p_17).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32((p_v2_3).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_17).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((16777215i32) as u64); let a_ = ((((l_p_17).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_17).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
    } else {
        { let v_ = (((c.m.r32(p_v0_1) as i32)) as u64); let a_ = (((l_p_17).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32((p_v0_1).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_17).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32((p_v0_1).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_17).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_17).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32(p_v1_2) as i32)) as u64); let a_ = (((l_p_17).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32((p_v1_2).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_17).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32((p_v1_2).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_17).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_17).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32(p_v2_3) as i32)) as u64); let a_ = (((l_p_17).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32((p_v2_3).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_17).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32((p_v2_3).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_17).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_17).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
    }
    l_l_16 = p_otLayer_10;
    if (((((l_l_16) >= (2i32)) as i32)) != 0) {
        { let r_ = 2i32; let n_ = (l_l_16).wrapping_sub(r_); l_l_16 = n_; };
    }
    if (((((p_z_11) < (0i32)) as i32)) != 0) {
        l_e_18 = ((c.m.r32(sym::gOtZ)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_16) as u32)).wrapping_mul(8));
    } else {
        if (((((p_z_11) >= (4096i32)) as i32)) != 0) {
            l_e_18 = ((c.m.r32(sym::gOtZ)).wrapping_add((((4095i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_16) as u32)).wrapping_mul(8));
        } else {
            l_e_18 = ((c.m.r32(sym::gOtZ)).wrapping_add((((p_z_11) as u32)).wrapping_mul(16))).wrapping_add((((l_l_16) as u32)).wrapping_mul(8));
        }
    }
    { let v_ = l_p_17; let a_ = (c.m.r32((l_e_18).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = l_p_17; let a_ = (l_e_18).wrapping_add(4); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "eft_ae__EftSprAnim_QueueTri", &PROBE_N, &[], true); }
    r_
}

pub fn eft_ae__EftSprAnim_DrawTriClip(c: &mut Ctx, mut p_poly_1: u32, mut p_otLayer_2: i32, mut p_z_3: i32, mut p_tex0_4: u64, mut p_mode_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_ae__EftSprAnim_DrawTriClip", &PROBE_N, &[(p_poly_1) as u32, (p_otLayer_2) as u32, (p_z_3) as u32, (p_tex0_4) as u32, (p_mode_5) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(1056) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 1056);
    let mut l_plane_6: u32 = 0;
    let mut l_n_7: i32 = 0;
    let mut l_i_8: i32 = 0;
    let mut l_ctx_9: i8 = 0;
    let r_ = 'f: {
        { let _ = { rt::memcpy(c, fp, p_poly_1, ((((3i32) as u32)).wrapping_mul(48u32)) as u32); fp }; };
        p_poly_1 = fp;
        l_n_7 = 3i32;
        F::EftGfx_UpdateClipPlanes(c);
        l_plane_6 = F::EftGfx_GetClipPlanes(c);
        {
            l_i_8 = 0i32;
            'l1: loop {
                if !(((((l_i_8) < (5i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_n_7 = F::ClipPoly_ClipPlane(c, p_poly_1, l_plane_6, l_n_7);
                    { let o_ = l_plane_6; let n_ = (o_).wrapping_add(16); l_plane_6 = n_; };
                }
                { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
            }
        }
        if (((((l_n_7) != (0i32)) as i32)) != 0) {
            F::ClipPoly_ProjectCur(c, fp.wrapping_add(768), fp.wrapping_add(912), p_poly_1, l_n_7);
            l_ctx_9 = (((((p_otLayer_2) >= (2i32)) as i32)) as i8);
            {
                l_i_8 = 2i32;
                'l3: loop {
                    if !(((((l_i_8) < (l_n_7)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        if ((((((c.m.r32(((fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) > (16777215i32)) as i32)) != 0) {
                            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        }
                        if ((((((c.m.r32(((fp.wrapping_add(768)).wrapping_add(((((l_i_8).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) > (16777215i32)) as i32)) != 0) {
                            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add(((((l_i_8).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        }
                        if ((((((c.m.r32(((fp.wrapping_add(768)).wrapping_add((((l_i_8) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) > (16777215i32)) as i32)) != 0) {
                            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add((((l_i_8) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        }
                        F::eft_ae__EftSprAnim_QueueTri(c, (fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(768)).wrapping_add(((((l_i_8).wrapping_sub(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(768)).wrapping_add((((l_i_8) as u32)).wrapping_mul(16)), ((p_poly_1).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(32), ((p_poly_1).wrapping_add(((((l_i_8).wrapping_sub(1i32)) as u32)).wrapping_mul(48))).wrapping_add(32), ((p_poly_1).wrapping_add((((l_i_8) as u32)).wrapping_mul(48))).wrapping_add(32), (fp.wrapping_add(912)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(912)).wrapping_add(((((l_i_8).wrapping_sub(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(912)).wrapping_add((((l_i_8) as u32)).wrapping_mul(16)), p_otLayer_2, p_z_3, p_tex0_4, p_mode_5, l_ctx_9);
                    }
                    { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "eft_ae__EftSprAnim_DrawTriClip", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprAnim_DrawQuad(c: &mut Ctx, mut p_tex0_1: u64, mut p_corner_2: u32, mut p_uv_3: u32, mut p_color_4: u32, mut p_otLayer_5: i32, mut p_z_6: i32, mut p_mode_7: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_DrawQuad", &PROBE_N, &[(p_tex0_1) as u32, (p_corner_2) as u32, (p_uv_3) as u32, (p_color_4) as u32, (p_otLayer_5) as u32, (p_z_6) as u32, (p_mode_7) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(432) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 432);
    let mut l_j_8: i32 = 0;
    let r_ = 'f: {
        { let _ = F::memset(c, fp, 0i32, 432u32); };
        {
            l_j_8 = 0i32;
            'l1: loop {
                if !(((((l_j_8) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::ClipVtx_SetArray(c, fp, (p_corner_2).wrapping_add((((l_j_8) as u32)).wrapping_mul(16)), (p_uv_3).wrapping_add((((l_j_8) as u32)).wrapping_mul(16)), (p_color_4).wrapping_add((((l_j_8) as u32)).wrapping_mul(16)), 3i32);
                    F::eft_ae__EftSprAnim_DrawTriClip(c, fp, p_otLayer_5, p_z_6, p_tex0_1, p_mode_7);
                }
                { let o_ = l_j_8; let n_ = (o_).wrapping_add(1); l_j_8 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_DrawQuad", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprAnim_DrawQuadSubdiv(c: &mut Ctx, mut p_tex0_1: u64, mut p_corner_2: u32, mut p_uv_3: u32, mut p_color_4: u32, mut p_otLayer_5: i32, mut p_z_6: i32, mut p_mode_7: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_DrawQuadSubdiv", &PROBE_N, &[(p_tex0_1) as u32, (p_corner_2) as u32, (p_uv_3) as u32, (p_color_4) as u32, (p_otLayer_5) as u32, (p_z_6) as u32, (p_mode_7) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(976) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 976);
    let mut l_q_8: i32 = 0;
    let mut l_k_9: i32 = 0;
    let mut l_j_10: i32 = 0;
    let r_ = 'f: {
        { let v_ = F32(c.m.r32((p_corner_2).wrapping_add((((0i32) as u32)).wrapping_mul(16)))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((p_corner_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((p_corner_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32((p_corner_2).wrapping_add((((0i32) as u32)).wrapping_mul(16)))), F32(c.m.r32((p_corner_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))))), F32(0x3f000000)); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((p_corner_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))), F32(c.m.r32(((p_corner_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4)))), F32(0x3f000000)); let a_ = ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((p_corner_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8))), F32(c.m.r32(((p_corner_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)))), F32(0x3f000000)); let a_ = ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32((p_corner_2).wrapping_add((((0i32) as u32)).wrapping_mul(16)))), F32(c.m.r32((p_corner_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))))), F32(0x3f000000)); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((p_corner_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))), F32(c.m.r32(((p_corner_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4)))), F32(0x3f000000)); let a_ = ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((p_corner_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8))), F32(c.m.r32(((p_corner_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)))), F32(0x3f000000)); let a_ = ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32((p_corner_2).wrapping_add((((0i32) as u32)).wrapping_mul(16)))), F32(c.m.r32((p_corner_2).wrapping_add((((3i32) as u32)).wrapping_mul(16))))), F32(0x3f000000)); let a_ = (fp).wrapping_add((((3i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((p_corner_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))), F32(c.m.r32(((p_corner_2).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4)))), F32(0x3f000000)); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((p_corner_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8))), F32(c.m.r32(((p_corner_2).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)))), F32(0x3f000000)); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)))), F32(c.m.r32((p_corner_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))))); let a_ = (fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))), F32(c.m.r32(((p_corner_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)))); let a_ = ((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8))), F32(c.m.r32(((p_corner_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)))); let a_ = ((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)))), F32(c.m.r32((p_corner_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))))); let a_ = (fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))), F32(c.m.r32(((p_corner_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)))); let a_ = ((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8))), F32(c.m.r32(((p_corner_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)))); let a_ = ((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16)))), F32(c.m.r32((p_corner_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))))); let a_ = (fp.wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4))), F32(c.m.r32(((p_corner_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)))); let a_ = ((fp.wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8))), F32(c.m.r32(((p_corner_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)))); let a_ = ((fp.wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        {
            l_q_8 = 0i32;
            'l1: loop {
                if !(((((l_q_8) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    {
                        l_k_9 = 0i32;
                        'l3: loop {
                            if !(((((l_k_9) < (4i32)) as i32)) != 0) { break 'l3; }
                            'c4: {
                                F::Vec3_Add(c, (fp.wrapping_add(64)).wrapping_add((((l_k_9) as u32)).wrapping_mul(16)), (fp).wrapping_add((((l_k_9) as u32)).wrapping_mul(16)), (fp.wrapping_add(192)).wrapping_add((((l_q_8) as u32)).wrapping_mul(16)));
                                { let a1_ = (p_uv_3).wrapping_add(((((c.m.r32(((sym::gEftSprSubdivCorner).wrapping_add((((l_q_8) as u32)).wrapping_mul(16))).wrapping_add((((l_k_9) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(16)); F::Vec4_Copy(c, (fp.wrapping_add(128)).wrapping_add((((l_k_9) as u32)).wrapping_mul(16)), a1_) };
                                { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(64)).wrapping_add((((l_k_9) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(128)).wrapping_add((((l_k_9) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(128)).wrapping_add((((l_k_9) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                            }
                            { let o_ = l_k_9; let n_ = (o_).wrapping_add(1); l_k_9 = n_; };
                        }
                    }
                    {
                        l_j_10 = 0i32;
                        'l5: loop {
                            if !(((((l_j_10) < (2i32)) as i32)) != 0) { break 'l5; }
                            'c6: {
                                F::ClipVtx_SetArray(c, fp.wrapping_add(256), (fp.wrapping_add(64)).wrapping_add((((l_j_10) as u32)).wrapping_mul(16)), (fp.wrapping_add(128)).wrapping_add((((l_j_10) as u32)).wrapping_mul(16)), (p_color_4).wrapping_add((((l_j_10) as u32)).wrapping_mul(16)), 3i32);
                                F::eft_ae__EftSprAnim_DrawTriClip(c, fp.wrapping_add(256), p_otLayer_5, p_z_6, p_tex0_1, p_mode_7);
                            }
                            { let o_ = l_j_10; let n_ = (o_).wrapping_add(1); l_j_10 = n_; };
                        }
                    }
                }
                { let o_ = l_q_8; let n_ = (o_).wrapping_add(1); l_q_8 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSprAnim_DrawQuadSubdiv", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprPack_Get(c: &mut Ctx, mut p_pack_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprPack_Get", &PROBE_N, &[(p_pack_1) as u32], false); }
    let r_ = 'f: {
    { let _ = F::EftSprPack_IsValid(c, p_pack_1); };
    break 'f p_pack_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSprPack_Get", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprPack_Bind(c: &mut Ctx, mut p_pack_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprPack_Bind", &PROBE_N, &[(p_pack_1) as u32], false); }
    let mut l_def_2: u32 = 0;
    let mut l_key_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_def_2 = F::EftSprPack_GetLayers(c, p_pack_1);
    l_key_3 = (p_pack_1).wrapping_add((((c.m.r32((p_pack_1).wrapping_add(8)) as i32)) as u32));
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (((c.m.r8((p_pack_1).wrapping_add(4))) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = l_key_3; let a_ = (l_def_2).wrapping_add(20); c.m.w32(a_, v_); };
                { let r_ = ((c.m.r8((l_def_2).wrapping_add(4))) as i32); let n_ = (l_key_3).wrapping_add((((r_) as u32)).wrapping_mul(64)); l_key_3 = n_; };
                { let o_ = l_def_2; let n_ = (o_).wrapping_add(32); l_def_2 = n_; };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSprPack_Bind", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprPack_GetLayerCount(c: &mut Ctx, mut p_pack_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprPack_GetLayerCount", &PROBE_N, &[(p_pack_1) as u32], false); }
    let r_ = 'f: {
    { let _ = F::EftSprPack_IsValid(c, p_pack_1); };
    break 'f ((c.m.r8((p_pack_1).wrapping_add(4))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSprPack_GetLayerCount", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprPack_GetLayers(c: &mut Ctx, mut p_pack_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprPack_GetLayers", &PROBE_N, &[(p_pack_1) as u32], false); }
    let r_ = 'f: {
    { let _ = F::EftSprPack_IsValid(c, p_pack_1); };
    break 'f (p_pack_1).wrapping_add(32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSprPack_GetLayers", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprPack_GetLoopEndFrame(c: &mut Ctx, mut p_def_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprPack_GetLoopEndFrame", &PROBE_N, &[(p_def_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = ((c.m.r32((p_def_1).wrapping_add(20))).wrapping_add((((c.m.r8((p_def_1).wrapping_add(5))) as u32)).wrapping_mul(64))).wrapping_add(52); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSprPack_GetLoopEndFrame", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprPack_GetLoopStartFrame(c: &mut Ctx, mut p_def_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprPack_GetLoopStartFrame", &PROBE_N, &[(p_def_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = ((c.m.r32((p_def_1).wrapping_add(20))).wrapping_add((((c.m.r8((p_def_1).wrapping_add(6))) as u32)).wrapping_mul(64))).wrapping_add(52); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSprPack_GetLoopStartFrame", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprPack_GetKeyCount(c: &mut Ctx, mut p_def_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprPack_GetKeyCount", &PROBE_N, &[(p_def_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((c.m.r8((p_def_1).wrapping_add(4))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSprPack_GetKeyCount", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprPack_FindKey(c: &mut Ctx, mut p_def_1: u32, mut p_frame_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprPack_FindKey", &PROBE_N, &[(p_def_1) as u32, (p_frame_2) as u32], false); }
    let mut l_found_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_found_3 = (1i32).wrapping_neg();
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < ((((c.m.r8((p_def_1).wrapping_add(4))) as i32)).wrapping_sub(1i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((((((p_frame_2) >= ({ let a_ = ((c.m.r32((p_def_1).wrapping_add(20))).wrapping_add((((l_i_4) as u32)).wrapping_mul(64))).wrapping_add(52); (c.m.r32(a_) as i32) })) as i32)) != 0) && (((((p_frame_2) <= ({ let a_ = ((c.m.r32((p_def_1).wrapping_add(20))).wrapping_add(((((l_i_4).wrapping_add(1i32)) as u32)).wrapping_mul(64))).wrapping_add(52); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
                    l_found_3 = l_i_4;
                    break 'l1;
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    break 'f l_found_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSprPack_FindKey", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprPack_GetKeyT(c: &mut Ctx, mut p_def_1: u32, mut p_frame_2: i32, mut p_key_3: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprPack_GetKeyT", &PROBE_N, &[(p_def_1) as u32, (p_frame_2) as u32, (p_key_3) as u32], false); }
    let mut l_zero_4: F32 = F32(0);
    let mut l_t_5: F32 = F32(0);
    let mut l_first_6: i32 = 0;
    let mut l_span_7: F32 = F32(0);
    let mut l_at_8: F32 = F32(0);
    let r_ = 'f: {
    l_zero_4 = F32(0x00000000);
    if (((((p_key_3) < (0i32)) as i32)) != 0) {
        break 'f l_zero_4;
    }
    {
        l_first_6 = { let a_ = ((c.m.r32((p_def_1).wrapping_add(20))).wrapping_add((((p_key_3) as u32)).wrapping_mul(64))).wrapping_add(52); (c.m.r32(a_) as i32) };
        l_span_7 = rt::i2f(({ let a_ = ((c.m.r32((p_def_1).wrapping_add(20))).wrapping_add(((((p_key_3).wrapping_add(1i32)) as u32)).wrapping_mul(64))).wrapping_add(52); (c.m.r32(a_) as i32) }).wrapping_sub(l_first_6));
        l_at_8 = rt::i2f((p_frame_2).wrapping_sub(l_first_6));
        l_t_5 = F32(0x3f800000);
        if (((rt::fne(l_span_7, l_zero_4) as i32)) != 0) {
            l_t_5 = rt::fdiv(l_at_8, l_span_7);
        }
    }
    break 'f l_t_5;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSprPack_GetKeyT", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprPack_GetFirstFrame(c: &mut Ctx, mut p_def_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprPack_GetFirstFrame", &PROBE_N, &[(p_def_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = ((c.m.r32((p_def_1).wrapping_add(20))).wrapping_add((((0i32) as u32)).wrapping_mul(64))).wrapping_add(52); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSprPack_GetFirstFrame", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprPack_GetLastFrame(c: &mut Ctx, mut p_def_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprPack_GetLastFrame", &PROBE_N, &[(p_def_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = ((c.m.r32((p_def_1).wrapping_add(20))).wrapping_add(((((((c.m.r8((p_def_1).wrapping_add(4))) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(64))).wrapping_add(52); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSprPack_GetLastFrame", &PROBE_N, &[], true); }
    r_
}

pub fn EftSprPack_IsValid(c: &mut Ctx, mut p_pack_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSprPack_IsValid", &PROBE_N, &[(p_pack_1) as u32], false); }
    let r_ = 'f: {
    break 'f (((c.m.r32(p_pack_1)) == (((808464470i32) as u32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSprPack_IsValid", &PROBE_N, &[], true); }
    r_
}

pub fn BtlTaskList_UpdateRoot(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlTaskList_UpdateRoot", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32(sym::gBtlTaskRoot); F::BtlTaskList_Update(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlTaskList_UpdateRoot", &PROBE_N, &[], true); }
    r_
}

pub fn BtlTaskList_Update(c: &mut Ctx, mut p_list_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlTaskList_Update", &PROBE_N, &[(p_list_1) as u32], false); }
    let mut l_task_2: u32 = 0;
    let mut l_next_3: u32 = 0;
    let r_ = 'f: {
    l_task_2 = c.m.r32((p_list_1).wrapping_add(4));
    'l1: while (((((l_task_2) != (((0i32) as u32))) as i32)) != 0) {
        if ((((({ let a_ = c.m.r32((l_task_2).wrapping_add(40)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = l_task_2; let a_ = sym::gBtlTaskCur; c.m.w32(a_, v_); };
            { let fp_ = { let a_ = c.m.r32((l_task_2).wrapping_add(40)); c.m.r32(a_) }; { let _ = D::call(c, fp_, &[((l_task_2) as u64)]); } };
            if ((((({ let a_ = (c.m.r32((l_task_2).wrapping_add(40))).wrapping_add(20); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
                { let v_ = ((1i32) as u8); let a_ = l_task_2; rt::bf_set(c, a_, 1, 1, (v_) as u64);  };
            }
        }
        l_next_3 = c.m.r32((l_task_2).wrapping_add(48));
        if (((rt::bf_get_u(c, l_task_2, 0, 1) as u8)) != 0) {
            F::BtlTask_Kill(c, l_task_2);
        } else {
            if (((((c.m.r32((l_task_2).wrapping_add(36))) != (((0i32) as u32))) as i32)) != 0) {
                { let a0_ = c.m.r32((l_task_2).wrapping_add(36)); F::BtlTaskList_Update(c, a0_) };
            }
        }
        l_task_2 = l_next_3;
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlTaskList_Update", &PROBE_N, &[], true); }
    r_
}

pub fn BtlTaskList_PostUpdate(c: &mut Ctx, mut p_list_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlTaskList_PostUpdate", &PROBE_N, &[(p_list_1) as u32], false); }
    let mut l_task_2: u32 = 0;
    let mut l_next_3: u32 = 0;
    let r_ = 'f: {
    l_task_2 = c.m.r32((p_list_1).wrapping_add(4));
    'l1: while (((((l_task_2) != (((0i32) as u32))) as i32)) != 0) {
        if ((((({ let a_ = (c.m.r32((l_task_2).wrapping_add(40))).wrapping_add(12); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
            { let fp_ = { let a_ = (c.m.r32((l_task_2).wrapping_add(40))).wrapping_add(12); c.m.r32(a_) }; { let _ = D::call(c, fp_, &[((l_task_2) as u64)]); } };
        }
        l_next_3 = c.m.r32((l_task_2).wrapping_add(48));
        if ((((((((!(((rt::bf_get_u(c, l_task_2, 0, 1) as u8)) != 0)) as i32)) != 0) && (((((c.m.r32((l_task_2).wrapping_add(36))) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
            { let a0_ = c.m.r32((l_task_2).wrapping_add(36)); F::BtlTaskList_PostUpdate(c, a0_) };
        }
        l_task_2 = l_next_3;
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlTaskList_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn BtlTaskList_Reset(c: &mut Ctx, mut p_list_1: u32, mut p_mask_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlTaskList_Reset", &PROBE_N, &[(p_list_1) as u32, (p_mask_2) as u32], false); }
    let mut l_task_3: u32 = 0;
    let mut l_next_4: u32 = 0;
    let r_ = 'f: {
    l_task_3 = c.m.r32((p_list_1).wrapping_add(4));
    'l1: while (((((l_task_3) != (((0i32) as u32))) as i32)) != 0) {
        if (((((p_mask_2) == (0i32)) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32((l_task_3).wrapping_add(40))).wrapping_add(16); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
                { let fp_ = { let a_ = (c.m.r32((l_task_3).wrapping_add(40))).wrapping_add(16); c.m.r32(a_) }; { let _ = D::call(c, fp_, &[((l_task_3) as u64)]); } };
            }
        } else {
            if (((((((((p_mask_2) == (2048i32)) as i32)) != 0) && ((((c.m.r32((l_task_3).wrapping_add(4))) & (((2048i32) as u32)))) != 0)) as i32)) != 0) {
                if ((((({ let a_ = (c.m.r32((l_task_3).wrapping_add(40))).wrapping_add(16); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
                    { let fp_ = { let a_ = (c.m.r32((l_task_3).wrapping_add(40))).wrapping_add(16); c.m.r32(a_) }; { let _ = D::call(c, fp_, &[((l_task_3) as u64)]); } };
                }
            } else {
                if (((((((((p_mask_2) == (4096i32)) as i32)) != 0) && ((((c.m.r32((l_task_3).wrapping_add(4))) & (((4096i32) as u32)))) != 0)) as i32)) != 0) {
                    if ((((({ let a_ = (c.m.r32((l_task_3).wrapping_add(40))).wrapping_add(16); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
                        { let fp_ = { let a_ = (c.m.r32((l_task_3).wrapping_add(40))).wrapping_add(16); c.m.r32(a_) }; { let _ = D::call(c, fp_, &[((l_task_3) as u64)]); } };
                    }
                } else {
                    if (((((((((p_mask_2) == (8192i32)) as i32)) != 0) && ((((c.m.r32((l_task_3).wrapping_add(4))) & (((8192i32) as u32)))) != 0)) as i32)) != 0) {
                        if ((((({ let a_ = (c.m.r32((l_task_3).wrapping_add(40))).wrapping_add(16); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
                            { let fp_ = { let a_ = (c.m.r32((l_task_3).wrapping_add(40))).wrapping_add(16); c.m.r32(a_) }; { let _ = D::call(c, fp_, &[((l_task_3) as u64)]); } };
                        }
                    }
                }
            }
        }
        l_next_4 = c.m.r32((l_task_3).wrapping_add(48));
        if (((rt::bf_get_u(c, l_task_3, 0, 1) as u8)) != 0) {
            F::BtlTask_Kill(c, l_task_3);
        } else {
            if (((((c.m.r32((l_task_3).wrapping_add(36))) != (((0i32) as u32))) as i32)) != 0) {
                { let a0_ = c.m.r32((l_task_3).wrapping_add(36)); F::BtlTaskList_Reset(c, a0_, p_mask_2) };
            }
        }
        l_task_3 = l_next_4;
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlTaskList_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn BtlTaskList_Draw(c: &mut Ctx, mut p_list_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlTaskList_Draw", &PROBE_N, &[(p_list_1) as u32], false); }
    let mut l_task_2: u32 = 0;
    let mut l_next_3: u32 = 0;
    let r_ = 'f: {
    l_task_2 = c.m.r32((p_list_1).wrapping_add(4));
    'l1: while (((((l_task_2) != (((0i32) as u32))) as i32)) != 0) {
        if ((((((((({ let a_ = (c.m.r32((l_task_2).wrapping_add(40))).wrapping_add(20); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) && (((((rt::bf_get_u(c, l_task_2, 1, 1) as u8)) as i32)) != 0)) as i32)) != 0) {
            { let fp_ = { let a_ = (c.m.r32((l_task_2).wrapping_add(40))).wrapping_add(20); c.m.r32(a_) }; { let _ = D::call(c, fp_, &[((l_task_2) as u64)]); } };
        }
        l_next_3 = c.m.r32((l_task_2).wrapping_add(48));
        if ((((((((!(((rt::bf_get_u(c, l_task_2, 0, 1) as u8)) != 0)) as i32)) != 0) && (((((c.m.r32((l_task_2).wrapping_add(36))) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
            { let a0_ = c.m.r32((l_task_2).wrapping_add(36)); F::BtlTaskList_Draw(c, a0_) };
        }
        l_task_2 = l_next_3;
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlTaskList_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn BtlTaskList_CreateRoot(c: &mut Ctx, mut p_count_1: u32, mut p_workSize_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlTaskList_CreateRoot", &PROBE_N, &[(p_count_1) as u32, (p_workSize_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = F::BtlTaskList_Create(c, ((0i32) as u32), p_count_1, p_workSize_2); let a_ = sym::gBtlTaskRoot; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlTaskList_CreateRoot", &PROBE_N, &[], true); }
    r_
}

pub fn BtlTask_CreateChildList(c: &mut Ctx, mut p_task_1: u32, mut p_count_2: u32, mut p_workSize_3: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlTask_CreateChildList", &PROBE_N, &[(p_task_1) as u32, (p_count_2) as u32, (p_workSize_3) as u32], false); }
    let r_ = 'f: {
    break 'f { let v_ = F::BtlTaskList_Create(c, p_task_1, p_count_2, p_workSize_3); let a_ = (p_task_1).wrapping_add(36); c.m.w32(a_, v_); v_ };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlTask_CreateChildList", &PROBE_N, &[], true); }
    r_
}

pub fn BtlTask_CreateChildListCur(c: &mut Ctx, mut p_count_1: u32, mut p_workSize_2: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlTask_CreateChildListCur", &PROBE_N, &[(p_count_1) as u32, (p_workSize_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = { let a0_ = c.m.r32(sym::gBtlTaskCur); F::BtlTaskList_Create(c, a0_, p_count_1, p_workSize_2) }; let a_ = (c.m.r32(sym::gBtlTaskCur)).wrapping_add(36); c.m.w32(a_, v_); };
    break 'f { let a_ = (c.m.r32(sym::gBtlTaskCur)).wrapping_add(36); c.m.r32(a_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlTask_CreateChildListCur", &PROBE_N, &[], true); }
    r_
}

pub fn BtlTaskList_Create(c: &mut Ctx, mut p_parent_1: u32, mut p_count_2: u32, mut p_workSize_3: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlTaskList_Create", &PROBE_N, &[(p_parent_1) as u32, (p_count_2) as u32, (p_workSize_3) as u32], false); }
    let mut l_list_4: u32 = 0;
    let mut l_task_5: u32 = 0;
    let mut l_work_6: u32 = 0;
    let mut l_i_7: u16 = 0;
    let r_ = 'f: {
    if (((((((((p_parent_1) != (((0i32) as u32))) as i32)) != 0) && (((((c.m.r32((p_parent_1).wrapping_add(36))) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    l_list_4 = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((24u32) as i32)) };
    if (((((l_list_4) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    l_task_5 = ((0i32) as u32);
    if (((((p_count_2) != (((0i32) as u32))) as i32)) != 0) {
        l_task_5 = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, (((p_count_2).wrapping_mul(64u32)) as i32)) };
    }
    if (((((l_task_5) == (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Free(c, a0_, l_list_4) };
        break 'f ((0i32) as u32);
    }
    p_workSize_3 = (rt::div_u32((p_workSize_3).wrapping_add(((3i32) as u32)), ((4i32) as u32))).wrapping_mul(((4i32) as u32));
    l_work_6 = ((0i32) as u32);
    if (((((p_workSize_3) != (((0i32) as u32))) as i32)) != 0) {
        l_work_6 = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, (((p_workSize_3).wrapping_mul(p_count_2)) as i32)) };
        if (((((l_work_6) == (((0i32) as u32))) as i32)) != 0) {
            { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Free(c, a0_, l_task_5) };
            { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Free(c, a0_, l_list_4) };
            break 'f ((0i32) as u32);
        }
    }
    { let v_ = p_parent_1; let a_ = l_list_4; c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_list_4).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_list_4).wrapping_add(8); c.m.w32(a_, v_); };
    { let v_ = l_task_5; let a_ = (l_list_4).wrapping_add(12); c.m.w32(a_, v_); };
    { let v_ = l_task_5; let a_ = (l_list_4).wrapping_add(16); c.m.w32(a_, v_); };
    { let v_ = l_work_6; let a_ = (l_list_4).wrapping_add(20); c.m.w32(a_, v_); };
    {
        l_i_7 = ((0i32) as u16);
        'l1: loop {
            if !(((((((l_i_7) as u32)) < (p_count_2)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = l_i_7; let a_ = (l_task_5).wrapping_add(2); c.m.w16(a_, v_); };
                { let v_ = l_work_6; let a_ = (l_task_5).wrapping_add(56); c.m.w32(a_, v_); };
                if (((((l_work_6) != (((0i32) as u32))) as i32)) != 0) {
                    { let r_ = p_workSize_3; let n_ = (l_work_6).wrapping_add(r_); l_work_6 = n_; };
                }
                { let v_ = (l_task_5).wrapping_add((((1i32) as u32)).wrapping_mul(64)); let a_ = (l_task_5).wrapping_add(52); c.m.w32(a_, v_); };
                { let v_ = l_list_4; let a_ = (l_task_5).wrapping_add(32); c.m.w32(a_, v_); };
                { let o_ = l_task_5; let n_ = (o_).wrapping_add(64); l_task_5 = n_; };
            }
            { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
        }
    }
    { let v_ = ((0i32) as u32); let a_ = ((c.m.r32((l_list_4).wrapping_add(16))).wrapping_add(((p_count_2).wrapping_sub(((1i32) as u32))).wrapping_mul(64))).wrapping_add(52); c.m.w32(a_, v_); };
    break 'f l_list_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlTaskList_Create", &PROBE_N, &[], true); }
    r_
}

pub fn BtlTaskList_DestroyRoot(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlTaskList_DestroyRoot", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32(sym::gBtlTaskRoot); F::BtlTaskList_Destroy(c, a0_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gBtlTaskRoot; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlTaskList_DestroyRoot", &PROBE_N, &[], true); }
    r_
}

pub fn BtlTask_DestroyChildList(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlTask_DestroyChildList", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32((p_task_1).wrapping_add(36)); F::BtlTaskList_Destroy(c, a0_) };
    { let v_ = ((0i32) as u32); let a_ = (p_task_1).wrapping_add(36); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlTask_DestroyChildList", &PROBE_N, &[], true); }
    r_
}

pub fn BtlTaskList_Destroy(c: &mut Ctx, mut p_list_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlTaskList_Destroy", &PROBE_N, &[(p_list_1) as u32], false); }
    let r_ = 'f: {
    if (((((p_list_1) != (((0i32) as u32))) as i32)) != 0) {
        F::BtlTaskList_KillAll(c, p_list_1);
        if (((((c.m.r32((p_list_1).wrapping_add(20))) != (((0i32) as u32))) as i32)) != 0) {
            { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32((p_list_1).wrapping_add(20)); F::BtlPool_Free(c, a0_, a1_) };
        }
        { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32((p_list_1).wrapping_add(16)); F::BtlPool_Free(c, a0_, a1_) };
        { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Free(c, a0_, p_list_1) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlTaskList_Destroy", &PROBE_N, &[], true); }
    r_
}

pub fn BtlTaskList_AddFirst(c: &mut Ctx, mut p_list_1: u32, mut p_cls_2: u32, mut p_arg_3: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlTaskList_AddFirst", &PROBE_N, &[(p_list_1) as u32, (p_cls_2) as u32, (p_arg_3) as u32], false); }
    let mut l_task_4: u32 = 0;
    let r_ = 'f: {
    l_task_4 = c.m.r32((p_list_1).wrapping_add(12));
    { let v_ = l_task_4; let a_ = (p_list_1).wrapping_add(8); c.m.w32(a_, v_); };
    { let v_ = l_task_4; let a_ = (p_list_1).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_task_4).wrapping_add(44); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_task_4).wrapping_add(48); c.m.w32(a_, v_); };
    { let v_ = c.m.r32((l_task_4).wrapping_add(52)); let a_ = (p_list_1).wrapping_add(12); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_task_4).wrapping_add(52); c.m.w32(a_, v_); };
    F::BtlTask_Init(c, l_task_4, p_cls_2, p_arg_3);
    break 'f l_task_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlTaskList_AddFirst", &PROBE_N, &[], true); }
    r_
}

pub fn BtlTaskList_AddHead(c: &mut Ctx, mut p_list_1: u32, mut p_cls_2: u32, mut p_arg_3: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlTaskList_AddHead", &PROBE_N, &[(p_list_1) as u32, (p_cls_2) as u32, (p_arg_3) as u32], false); }
    let mut l_task_4: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32((p_list_1).wrapping_add(4))) == (((0i32) as u32))) as i32)) != 0) {
        l_task_4 = F::BtlTaskList_AddFirst(c, p_list_1, p_cls_2, p_arg_3);
    } else {
        l_task_4 = { let a0_ = c.m.r32((p_list_1).wrapping_add(4)); F::BtlTask_AddBefore(c, a0_, p_cls_2, p_arg_3) };
    }
    break 'f l_task_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlTaskList_AddHead", &PROBE_N, &[], true); }
    r_
}

pub fn BtlTaskList_AddTail(c: &mut Ctx, mut p_list_1: u32, mut p_cls_2: u32, mut p_arg_3: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlTaskList_AddTail", &PROBE_N, &[(p_list_1) as u32, (p_cls_2) as u32, (p_arg_3) as u32], false); }
    let mut l_task_4: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32((p_list_1).wrapping_add(8))) == (((0i32) as u32))) as i32)) != 0) {
        l_task_4 = F::BtlTaskList_AddFirst(c, p_list_1, p_cls_2, p_arg_3);
    } else {
        l_task_4 = { let a0_ = c.m.r32((p_list_1).wrapping_add(8)); F::BtlTask_AddAfter(c, a0_, p_cls_2, p_arg_3) };
    }
    break 'f l_task_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlTaskList_AddTail", &PROBE_N, &[], true); }
    r_
}

pub fn BtlTask_AddAfterCur(c: &mut Ctx, mut p_cls_1: u32, mut p_arg_2: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlTask_AddAfterCur", &PROBE_N, &[(p_cls_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    break 'f { let a0_ = c.m.r32(sym::gBtlTaskCur); F::BtlTask_AddAfter(c, a0_, p_cls_1, p_arg_2) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlTask_AddAfterCur", &PROBE_N, &[], true); }
    r_
}

pub fn BtlTask_AddAfter(c: &mut Ctx, mut p_task_1: u32, mut p_cls_2: u32, mut p_arg_3: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlTask_AddAfter", &PROBE_N, &[(p_task_1) as u32, (p_cls_2) as u32, (p_arg_3) as u32], false); }
    let mut l_list_4: u32 = 0;
    let mut l_add_5: u32 = 0;
    let mut l_next_6: u32 = 0;
    let r_ = 'f: {
    l_list_4 = c.m.r32((p_task_1).wrapping_add(32));
    l_add_5 = c.m.r32((l_list_4).wrapping_add(12));
    if (((((l_add_5) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    if (((((c.m.r32((l_list_4).wrapping_add(4))) == (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlTaskList_AddFirst(c, l_list_4, p_cls_2, p_arg_3);
    }
    l_next_6 = c.m.r32((p_task_1).wrapping_add(48));
    { let v_ = p_task_1; let a_ = (l_add_5).wrapping_add(44); c.m.w32(a_, v_); };
    { let v_ = l_next_6; let a_ = (l_add_5).wrapping_add(48); c.m.w32(a_, v_); };
    if (((((p_task_1) == (c.m.r32((l_list_4).wrapping_add(8)))) as i32)) != 0) {
        { let v_ = l_add_5; let a_ = (l_list_4).wrapping_add(8); c.m.w32(a_, v_); };
    } else {
        { let v_ = l_add_5; let a_ = (c.m.r32((p_task_1).wrapping_add(48))).wrapping_add(44); c.m.w32(a_, v_); };
    }
    { let v_ = l_add_5; let a_ = (p_task_1).wrapping_add(48); c.m.w32(a_, v_); };
    { let v_ = c.m.r32((l_add_5).wrapping_add(52)); let a_ = (l_list_4).wrapping_add(12); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_add_5).wrapping_add(52); c.m.w32(a_, v_); };
    F::BtlTask_Init(c, l_add_5, p_cls_2, p_arg_3);
    break 'f l_add_5;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlTask_AddAfter", &PROBE_N, &[], true); }
    r_
}

pub fn BtlTask_AddBeforeCur(c: &mut Ctx, mut p_cls_1: u32, mut p_arg_2: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlTask_AddBeforeCur", &PROBE_N, &[(p_cls_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    break 'f { let a0_ = c.m.r32(sym::gBtlTaskCur); F::BtlTask_AddBefore(c, a0_, p_cls_1, p_arg_2) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlTask_AddBeforeCur", &PROBE_N, &[], true); }
    r_
}

pub fn BtlTask_AddBefore(c: &mut Ctx, mut p_task_1: u32, mut p_cls_2: u32, mut p_arg_3: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlTask_AddBefore", &PROBE_N, &[(p_task_1) as u32, (p_cls_2) as u32, (p_arg_3) as u32], false); }
    let mut l_list_4: u32 = 0;
    let mut l_add_5: u32 = 0;
    let r_ = 'f: {
    l_list_4 = c.m.r32((p_task_1).wrapping_add(32));
    l_add_5 = c.m.r32((l_list_4).wrapping_add(12));
    if (((((l_add_5) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    if (((((c.m.r32((l_list_4).wrapping_add(4))) == (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlTaskList_AddFirst(c, l_list_4, p_cls_2, p_arg_3);
    }
    { let v_ = p_task_1; let a_ = (l_add_5).wrapping_add(48); c.m.w32(a_, v_); };
    { let v_ = c.m.r32((p_task_1).wrapping_add(44)); let a_ = (l_add_5).wrapping_add(44); c.m.w32(a_, v_); };
    if (((((p_task_1) == (c.m.r32((l_list_4).wrapping_add(4)))) as i32)) != 0) {
        { let v_ = l_add_5; let a_ = (l_list_4).wrapping_add(4); c.m.w32(a_, v_); };
    } else {
        { let v_ = l_add_5; let a_ = (c.m.r32((p_task_1).wrapping_add(44))).wrapping_add(48); c.m.w32(a_, v_); };
    }
    { let v_ = l_add_5; let a_ = (p_task_1).wrapping_add(44); c.m.w32(a_, v_); };
    { let v_ = c.m.r32((l_add_5).wrapping_add(52)); let a_ = (l_list_4).wrapping_add(12); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_add_5).wrapping_add(52); c.m.w32(a_, v_); };
    F::BtlTask_Init(c, l_add_5, p_cls_2, p_arg_3);
    break 'f l_add_5;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlTask_AddBefore", &PROBE_N, &[], true); }
    r_
}

pub fn BtlTask_Init(c: &mut Ctx, mut p_task_1: u32, mut p_cls_2: u32, mut p_arg_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlTask_Init", &PROBE_N, &[(p_task_1) as u32, (p_cls_2) as u32, (p_arg_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = (((!(3i32))) as u8); let a_ = p_task_1; c.m.w8(a_, v_); };
    { let v_ = F32(0x3f800000); let a_ = ((p_task_1).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = ((0i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (p_task_1).wrapping_add(36); c.m.w32(a_, v_); };
    { let v_ = p_cls_2; let a_ = (p_task_1).wrapping_add(40); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (p_task_1).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as i16); let a_ = (p_task_1).wrapping_add(8); c.m.w16(a_, (v_) as u16); };
    { let v_ = ((0i32) as u8); let a_ = (p_task_1).wrapping_add(10); c.m.w8(a_, v_); };
    { let v_ = ((0i32) as u8); let a_ = (p_task_1).wrapping_add(11); c.m.w8(a_, v_); };
    { let v_ = { let v_ = { let v_ = F32(0x00000000); let a_ = ((p_task_1).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); v_ }; let a_ = ((p_task_1).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); v_ }; let a_ = (p_task_1).wrapping_add(16); c.m.w32(a_, (v_).0); };
    if (((((c.m.r32((p_cls_2).wrapping_add(4))) != (((0i32) as u32))) as i32)) != 0) {
        { let fp_ = c.m.r32((p_cls_2).wrapping_add(4)); { let _ = D::call(c, fp_, &[((p_task_1) as u64), ((p_arg_3) as u64)]); } };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlTask_Init", &PROBE_N, &[], true); }
    r_
}

pub fn BtlTaskList_KillAll(c: &mut Ctx, mut p_list_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlTaskList_KillAll", &PROBE_N, &[(p_list_1) as u32], false); }
    let r_ = 'f: {
    'l1: while (((((c.m.r32((p_list_1).wrapping_add(4))) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32((p_list_1).wrapping_add(4)); F::BtlTask_Kill(c, a0_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlTaskList_KillAll", &PROBE_N, &[], true); }
    r_
}

pub fn BtlTask_SetDeadCur(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlTask_SetDeadCur", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32(sym::gBtlTaskCur); F::BtlTask_SetDead(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlTask_SetDeadCur", &PROBE_N, &[], true); }
    r_
}

pub fn BtlTask_SetDead(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlTask_SetDead", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((1i32) as u8); let a_ = p_task_1; rt::bf_set(c, a_, 0, 1, (v_) as u64);  };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlTask_SetDead", &PROBE_N, &[], true); }
    r_
}

pub fn BtlTask_KillCur(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlTask_KillCur", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32(sym::gBtlTaskCur); F::BtlTask_Kill(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlTask_KillCur", &PROBE_N, &[], true); }
    r_
}

pub fn BtlTask_Kill(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlTask_Kill", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_list_2: u32 = 0;
    let r_ = 'f: {
    l_list_2 = c.m.r32((p_task_1).wrapping_add(32));
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add(8); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let fp_ = { let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add(8); c.m.r32(a_) }; { let _ = D::call(c, fp_, &[((p_task_1) as u64)]); } };
    }
    if (((((c.m.r32((p_task_1).wrapping_add(36))) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32((p_task_1).wrapping_add(36)); F::BtlTaskList_KillAll(c, a0_) };
        { let a0_ = c.m.r32((p_task_1).wrapping_add(36)); F::BtlTaskList_Destroy(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (p_task_1).wrapping_add(36); c.m.w32(a_, v_); };
    }
    if (((((p_task_1) == (c.m.r32((l_list_2).wrapping_add(4)))) as i32)) != 0) {
        { let v_ = c.m.r32((p_task_1).wrapping_add(48)); let a_ = (l_list_2).wrapping_add(4); c.m.w32(a_, v_); };
    } else {
        { let v_ = c.m.r32((p_task_1).wrapping_add(48)); let a_ = (c.m.r32((p_task_1).wrapping_add(44))).wrapping_add(48); c.m.w32(a_, v_); };
    }
    if (((((p_task_1) == (c.m.r32((l_list_2).wrapping_add(8)))) as i32)) != 0) {
        { let v_ = c.m.r32((p_task_1).wrapping_add(44)); let a_ = (l_list_2).wrapping_add(8); c.m.w32(a_, v_); };
    } else {
        { let v_ = c.m.r32((p_task_1).wrapping_add(44)); let a_ = (c.m.r32((p_task_1).wrapping_add(48))).wrapping_add(44); c.m.w32(a_, v_); };
    }
    { let v_ = c.m.r32((l_list_2).wrapping_add(12)); let a_ = (p_task_1).wrapping_add(52); c.m.w32(a_, v_); };
    { let v_ = p_task_1; let a_ = (l_list_2).wrapping_add(12); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlTask_Kill", &PROBE_N, &[], true); }
    r_
}

pub fn BtlTask_SetTagBits(c: &mut Ctx, mut p_task_1: u32, mut p_bits_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlTask_SetTagBits", &PROBE_N, &[(p_task_1) as u32, (p_bits_2) as u32], false); }
    let r_ = 'f: {
    { let r_ = ((p_bits_2) as u32); let a_ = (p_task_1).wrapping_add(4); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlTask_SetTagBits", &PROBE_N, &[], true); }
    r_
}

pub fn BtlTask_SetPos(c: &mut Ctx, mut p_task_1: u32, p_pos_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlTask_SetPos", &PROBE_N, &[(p_task_1) as u32, (p_pos_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    rt::memcpy(c, fp, p_pos_2, 16);
    let r_ = 'f: {
        F::Vec4_Copy(c, (p_task_1).wrapping_add(16), fp);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlTask_SetPos", &PROBE_N, &[], true); }
    r_
}

pub fn BtlTask_SetOwnerTag(c: &mut Ctx, mut p_task_1: u32, mut p_bits_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlTask_SetOwnerTag", &PROBE_N, &[(p_task_1) as u32, (p_bits_2) as u32], false); }
    let r_ = 'f: {
    { let r_ = ((p_bits_2) as u32); let a_ = (p_task_1).wrapping_add(4); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlTask_SetOwnerTag", &PROBE_N, &[], true); }
    r_
}

pub fn BtlTaskList_GetRoot(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlTaskList_GetRoot", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f c.m.r32(sym::gBtlTaskRoot);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlTaskList_GetRoot", &PROBE_N, &[], true); }
    r_
}

pub fn BtlTask_GetCurrent(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlTask_GetCurrent", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f c.m.r32(sym::gBtlTaskCur);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlTask_GetCurrent", &PROBE_N, &[], true); }
    r_
}

pub fn BtlTask_GetParent(c: &mut Ctx, mut p_task_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlTask_GetParent", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = c.m.r32((p_task_1).wrapping_add(32)); c.m.r32(a_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlTask_GetParent", &PROBE_N, &[], true); }
    r_
}

pub fn EftVram_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVram_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, ((2060u32) as i32), ((((32i32) as i64 as u64)) as u32), 0i32, 2i32); let a_ = sym::gEftVram; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gEftVram); F::memset(c, a0_, 0i32, 2060u32) }; };
    { let v_ = 11008i32; let a_ = (c.m.r32(sym::gEftVram)).wrapping_add(2052); c.m.w32(a_, (v_) as u32); };
    { let v_ = 10752i32; let a_ = (c.m.r32(sym::gEftVram)).wrapping_add(2056); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftVram_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftVram_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVram_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftVram)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gEftVram); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gEftVram; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftVram_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftVram_SetCursor(c: &mut Ctx, mut p_imageBlock_1: i32, mut p_clutBlock_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVram_SetCursor", &PROBE_N, &[(p_imageBlock_1) as u32, (p_clutBlock_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_imageBlock_1; let a_ = (c.m.r32(sym::gEftVram)).wrapping_add(2052); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_clutBlock_2; let a_ = (c.m.r32(sym::gEftVram)).wrapping_add(2056); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftVram_SetCursor", &PROBE_N, &[], true); }
    r_
}

pub fn EftVram_GetCursor(c: &mut Ctx, mut p_imageBlock_1: u32, mut p_clutBlock_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVram_GetCursor", &PROBE_N, &[(p_imageBlock_1) as u32, (p_clutBlock_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = { let a_ = (c.m.r32(sym::gEftVram)).wrapping_add(2052); (c.m.r32(a_) as i32) }; let a_ = p_imageBlock_1; c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a_ = (c.m.r32(sym::gEftVram)).wrapping_add(2056); (c.m.r32(a_) as i32) }; let a_ = p_clutBlock_2; c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftVram_GetCursor", &PROBE_N, &[], true); }
    r_
}

pub fn EftVram_AddTex(c: &mut Ctx, mut p_tex_1: u32, mut p_tcc_2: i32, mut p_tfx_3: i32) -> u64 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVram_AddTex", &PROBE_N, &[(p_tex_1) as u32, (p_tcc_2) as u32, (p_tfx_3) as u32], false); }
    let mut l_e_4: u32 = 0;
    let mut l_tex0_5: u64 = 0;
    let r_ = 'f: {
    l_e_4 = (c.m.r32(sym::gEftVram)).wrapping_add(({ let a_ = (c.m.r32(sym::gEftVram)).wrapping_add(2048); c.m.r32(a_) }).wrapping_mul(16));
    { let v_ = p_tex_1; let a_ = l_e_4; c.m.w32(a_, v_); };
    { let v_ = { let a_ = (c.m.r32(sym::gEftVram)).wrapping_add(2052); (c.m.r32(a_) as i32) }; let a_ = (l_e_4).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a_ = (c.m.r32(sym::gEftVram)).wrapping_add(2056); (c.m.r32(a_) as i32) }; let a_ = (l_e_4).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = 3i32; let a_ = (l_e_4).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let a_ = (c.m.r32(sym::gEftVram)).wrapping_add(2048); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(1); c.m.w32(a_, n_); };
    { let r_ = 18444492291075719168u64; let a_ = p_tex_1; let n_ = ((c.m.r64(a_)) & (r_)); c.m.w64(a_, n_); };
    l_tex0_5 = ((((((c.m.r64(p_tex_1)) | ((((p_tcc_2) as u64)).wrapping_shl((34i32) as u32)))) | ((((((p_tfx_3) as u64)).wrapping_shl((35i32) as u32)) | ((((({ let a_ = (c.m.r32(sym::gEftVram)).wrapping_add(2052); (c.m.r32(a_) as i32) }) as u32)) as u64)))))) | (((((({ let a_ = (c.m.r32(sym::gEftVram)).wrapping_add(2056); (c.m.r32(a_) as i32) }) as u32)) as u64)).wrapping_shl((37i32) as u32)));
    { let r_ = { let a_ = (c.m.r32((p_tex_1).wrapping_add(8))).wrapping_add(16); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gEftVram)).wrapping_add(2052); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
    { let r_ = { let a_ = (c.m.r32((p_tex_1).wrapping_add(8))).wrapping_add(20); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gEftVram)).wrapping_add(2056); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
    break 'f l_tex0_5;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftVram_AddTex", &PROBE_N, &[], true); }
    r_
}

pub fn EftVram_AddImage(c: &mut Ctx, mut p_tex_1: u32, mut p_tcc_2: i32, mut p_tfx_3: i32) -> u64 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVram_AddImage", &PROBE_N, &[(p_tex_1) as u32, (p_tcc_2) as u32, (p_tfx_3) as u32], false); }
    let mut l_e_4: u32 = 0;
    let mut l_tex0_5: u64 = 0;
    let r_ = 'f: {
    l_e_4 = (c.m.r32(sym::gEftVram)).wrapping_add(({ let a_ = (c.m.r32(sym::gEftVram)).wrapping_add(2048); c.m.r32(a_) }).wrapping_mul(16));
    { let v_ = p_tex_1; let a_ = l_e_4; c.m.w32(a_, v_); };
    { let v_ = { let a_ = (c.m.r32(sym::gEftVram)).wrapping_add(2052); (c.m.r32(a_) as i32) }; let a_ = (l_e_4).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = 1i32; let a_ = (l_e_4).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (l_e_4).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let a_ = (c.m.r32(sym::gEftVram)).wrapping_add(2048); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(1); c.m.w32(a_, n_); };
    { let r_ = 18444492291075719168u64; let a_ = p_tex_1; let n_ = ((c.m.r64(a_)) & (r_)); c.m.w64(a_, n_); };
    l_tex0_5 = ((((c.m.r64(p_tex_1)) | ((((p_tcc_2) as u64)).wrapping_shl((34i32) as u32)))) | ((((((p_tfx_3) as u64)).wrapping_shl((35i32) as u32)) | ((((({ let a_ = (c.m.r32(sym::gEftVram)).wrapping_add(2052); (c.m.r32(a_) as i32) }) as u32)) as u64)))));
    { let r_ = { let a_ = (c.m.r32((p_tex_1).wrapping_add(8))).wrapping_add(16); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gEftVram)).wrapping_add(2052); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
    break 'f l_tex0_5;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftVram_AddImage", &PROBE_N, &[], true); }
    r_
}

pub fn EftVram_AddClut(c: &mut Ctx, mut p_tex_1: u32) -> u64 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVram_AddClut", &PROBE_N, &[(p_tex_1) as u32], false); }
    let mut l_e_2: u32 = 0;
    let mut l_block_3: u64 = 0;
    let r_ = 'f: {
    l_e_2 = (c.m.r32(sym::gEftVram)).wrapping_add(({ let a_ = (c.m.r32(sym::gEftVram)).wrapping_add(2048); c.m.r32(a_) }).wrapping_mul(16));
    { let v_ = p_tex_1; let a_ = l_e_2; c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = (l_e_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a_ = (c.m.r32(sym::gEftVram)).wrapping_add(2056); (c.m.r32(a_) as i32) }; let a_ = (l_e_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = 2i32; let a_ = (l_e_2).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let a_ = (c.m.r32(sym::gEftVram)).wrapping_add(2048); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(1); c.m.w32(a_, n_); };
    l_block_3 = (((({ let a_ = (c.m.r32(sym::gEftVram)).wrapping_add(2056); (c.m.r32(a_) as i32) }) as u32)) as u64);
    { let r_ = { let a_ = (c.m.r32((p_tex_1).wrapping_add(8))).wrapping_add(20); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gEftVram)).wrapping_add(2056); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
    break 'f l_block_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftVram_AddClut", &PROBE_N, &[], true); }
    r_
}

pub fn EftVram_AddTexAt(c: &mut Ctx, mut p_tex_1: u32, mut p_imageBlock_2: i32, mut p_clutBlock_3: i32, mut p_tcc_4: i32, mut p_tfx_5: i32) -> u64 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVram_AddTexAt", &PROBE_N, &[(p_tex_1) as u32, (p_imageBlock_2) as u32, (p_clutBlock_3) as u32, (p_tcc_4) as u32, (p_tfx_5) as u32], false); }
    let mut l_e_6: u32 = 0;
    let r_ = 'f: {
    l_e_6 = (c.m.r32(sym::gEftVram)).wrapping_add(({ let a_ = (c.m.r32(sym::gEftVram)).wrapping_add(2048); c.m.r32(a_) }).wrapping_mul(16));
    { let v_ = 3i32; let a_ = (l_e_6).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_imageBlock_2; let a_ = (l_e_6).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_clutBlock_3; let a_ = (l_e_6).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_tex_1; let a_ = l_e_6; c.m.w32(a_, v_); };
    { let a_ = (c.m.r32(sym::gEftVram)).wrapping_add(2048); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(1); c.m.w32(a_, n_); };
    { let r_ = 18444492291075719168u64; let a_ = p_tex_1; let n_ = ((c.m.r64(a_)) & (r_)); c.m.w64(a_, n_); };
    break 'f ((((((c.m.r64(p_tex_1)) | ((((p_tcc_4) as u64)).wrapping_shl((34i32) as u32)))) | ((((((p_tfx_5) as u64)).wrapping_shl((35i32) as u32)) | (((((p_imageBlock_2) as u32)) as u64)))))) | ((((p_clutBlock_3) as u64)).wrapping_shl((37i32) as u32)));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftVram_AddTexAt", &PROBE_N, &[], true); }
    r_
}

pub fn EftTexSet_Keep32(c: &mut Ctx, mut p_set_1: u32, mut p_tcc_2: i32, mut p_tfx_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTexSet_Keep32", &PROBE_N, &[(p_set_1) as u32, (p_tcc_2) as u32, (p_tfx_3) as u32], false); }
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < ((c.m.r32((p_set_1).wrapping_add(512)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = F::EftVram_AddTex(c, (p_set_1).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)), p_tcc_2, p_tfx_3); let a_ = (p_set_1).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)); c.m.w64(a_, v_); };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTexSet_Keep32", &PROBE_N, &[], true); }
    r_
}

pub fn EftTexSet_Keep4(c: &mut Ctx, mut p_set_1: u32, mut p_tcc_2: i32, mut p_tfx_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTexSet_Keep4", &PROBE_N, &[(p_set_1) as u32, (p_tcc_2) as u32, (p_tfx_3) as u32], false); }
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < ((c.m.r32((p_set_1).wrapping_add(64)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = F::EftVram_AddTex(c, (p_set_1).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)), p_tcc_2, p_tfx_3); let a_ = (p_set_1).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)); c.m.w64(a_, v_); };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTexSet_Keep4", &PROBE_N, &[], true); }
    r_
}

pub fn EftTexSet_Keep8(c: &mut Ctx, mut p_set_1: u32, mut p_tcc_2: i32, mut p_tfx_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTexSet_Keep8", &PROBE_N, &[(p_set_1) as u32, (p_tcc_2) as u32, (p_tfx_3) as u32], false); }
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < ((c.m.r32((p_set_1).wrapping_add(128)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = F::EftVram_AddTex(c, (p_set_1).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)), p_tcc_2, p_tfx_3); let a_ = (p_set_1).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)); c.m.w64(a_, v_); };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTexSet_Keep8", &PROBE_N, &[], true); }
    r_
}

pub fn EftTexSet_Keep16(c: &mut Ctx, mut p_set_1: u32, mut p_tcc_2: i32, mut p_tfx_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTexSet_Keep16", &PROBE_N, &[(p_set_1) as u32, (p_tcc_2) as u32, (p_tfx_3) as u32], false); }
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < ((c.m.r32((p_set_1).wrapping_add(256)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = F::EftVram_AddTex(c, (p_set_1).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)), p_tcc_2, p_tfx_3); let a_ = (p_set_1).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)); c.m.w64(a_, v_); };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTexSet_Keep16", &PROBE_N, &[], true); }
    r_
}

pub fn EftVram_UploadAll(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVram_UploadAll", &PROBE_N, &[], false); }
    let mut l_i_1: u32 = 0;
    let mut l_e_2: u32 = 0;
    let mut l_clutBlock_3: i32 = 0;
    let mut l_imageBlock_4: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = ((0i32) as u32);
        'l1: loop {
            if !(((((l_i_1) < ({ let a_ = (c.m.r32(sym::gEftVram)).wrapping_add(2048); c.m.r32(a_) })) as i32)) != 0) { break 'l1; }
            'c2: {
                l_e_2 = (c.m.r32(sym::gEftVram)).wrapping_add((l_i_1).wrapping_mul(16));
                l_clutBlock_3 = (1i32).wrapping_neg();
                l_imageBlock_4 = (1i32).wrapping_neg();
                if (((((c.m.r32((l_e_2).wrapping_add(12)) as i32)) & (1i32))) != 0) {
                    l_imageBlock_4 = (c.m.r32((l_e_2).wrapping_add(4)) as i32);
                }
                if (((((c.m.r32((l_e_2).wrapping_add(12)) as i32)) & (2i32))) != 0) {
                    l_clutBlock_3 = (c.m.r32((l_e_2).wrapping_add(8)) as i32);
                }
                { let a0_ = c.m.r32(l_e_2); F::EftVram_Upload(c, a0_, l_imageBlock_4, l_clutBlock_3) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftVram_UploadAll", &PROBE_N, &[], true); }
    r_
}

pub fn EftVram_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVram_Reset", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 11008i32; let a_ = (c.m.r32(sym::gEftVram)).wrapping_add(2052); c.m.w32(a_, (v_) as u32); };
    { let v_ = 10752i32; let a_ = (c.m.r32(sym::gEftVram)).wrapping_add(2056); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gEftVram)).wrapping_add(2048); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftVram_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftTexSet_CheckCount(c: &mut Ctx, mut p_count_1: i32, mut p_max_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTexSet_CheckCount", &PROBE_N, &[(p_count_1) as u32, (p_max_2) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftTexSet_CheckCount", &PROBE_N, &[], true); }
    r_
}

pub fn EftTexSet_Load32(c: &mut Ctx, mut p_set_1: u32, mut p_pack_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTexSet_Load32", &PROBE_N, &[(p_set_1) as u32, (p_pack_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_count_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
        { let _ = F::memset(c, p_set_1, 0i32, 520u32); };
        F::Res_RelocateOffsets(c, fp, p_pack_2, p_pack_2);
        l_count_3 = { let a_ = c.m.r32(fp); (c.m.r32(a_) as i32) };
        { let v_ = l_count_3; let a_ = (p_set_1).wrapping_add(512); c.m.w32(a_, (v_) as u32); };
        F::EftTexSet_CheckCount(c, l_count_3, 32i32);
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < ((c.m.r32((p_set_1).wrapping_add(512)) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((l_i_4) as u32)).wrapping_mul(64)); let a_ = ((p_set_1).wrapping_add((((l_i_4) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, v_); };
                    { let v_ = { let a_ = (({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((l_i_4) as u32)).wrapping_mul(64))).wrapping_add(48); c.m.r64(a_) }; let a_ = (p_set_1).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)); c.m.w64(a_, v_); };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftTexSet_Load32", &PROBE_N, &[], true); }
    r_
}

pub fn EftTexSet_Load4(c: &mut Ctx, mut p_set_1: u32, mut p_pack_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftTexSet_Load4", &PROBE_N, &[(p_set_1) as u32, (p_pack_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_count_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
        { let _ = F::memset(c, p_set_1, 0i32, 72u32); };
        F::Res_RelocateOffsets(c, fp, p_pack_2, p_pack_2);
        l_count_3 = { let a_ = c.m.r32(fp); (c.m.r32(a_) as i32) };
        { let v_ = l_count_3; let a_ = (p_set_1).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
        F::EftTexSet_CheckCount(c, l_count_3, 4i32);
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < ((c.m.r32((p_set_1).wrapping_add(64)) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((l_i_4) as u32)).wrapping_mul(64)); let a_ = ((p_set_1).wrapping_add((((l_i_4) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, v_); };
                    { let v_ = { let a_ = (({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((l_i_4) as u32)).wrapping_mul(64))).wrapping_add(48); c.m.r64(a_) }; let a_ = (p_set_1).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)); c.m.w64(a_, v_); };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftTexSet_Load4", &PROBE_N, &[], true); }
    r_
}
