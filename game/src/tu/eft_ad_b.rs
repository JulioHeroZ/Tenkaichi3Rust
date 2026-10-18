#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EftMesh_Init(c: &mut Ctx, mut p_mesh_1: u32, mut p_data_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_Init", &PROBE_N, &[(p_mesh_1) as u32, (p_data_2) as u32], false); }
    let r_ = 'f: {
    { let _ = F::memset(c, p_mesh_1, 0i32, 144u32); };
    { let v_ = p_data_2; let a_ = (p_mesh_1).wrapping_add(120); c.m.w32(a_, v_); };
    { let v_ = (p_data_2).wrapping_add(16); let a_ = (p_mesh_1).wrapping_add(124); c.m.w32(a_, v_); };
    { let v_ = (p_data_2).wrapping_add((((c.m.r32((p_data_2).wrapping_add(8)) as i32)) as u32)); let a_ = (p_mesh_1).wrapping_add(128); c.m.w32(a_, v_); };
    { let v_ = (p_data_2).wrapping_add((((c.m.r32((p_data_2).wrapping_add(12)) as i32)) as u32)); let a_ = (p_mesh_1).wrapping_add(132); c.m.w32(a_, v_); };
    { let v_ = ((128i32) as u8); let a_ = (p_mesh_1).wrapping_add(64); c.m.w8(a_, v_); };
    { let v_ = ((128i32) as u8); let a_ = (p_mesh_1).wrapping_add(65); c.m.w8(a_, v_); };
    { let v_ = ((128i32) as u8); let a_ = (p_mesh_1).wrapping_add(66); c.m.w8(a_, v_); };
    { let v_ = ((128i32) as u8); let a_ = (p_mesh_1).wrapping_add(67); c.m.w8(a_, v_); };
    { let v_ = F32(0x3f800000); let a_ = (p_mesh_1).wrapping_add(96); c.m.w32(a_, (v_).0); };
    { let v_ = (1i32).wrapping_neg(); let a_ = (p_mesh_1).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
    { let v_ = (1i32).wrapping_neg(); let a_ = (p_mesh_1).wrapping_add(116); c.m.w32(a_, (v_) as u32); };
    F::Mtx_StoreIdentity(c, p_mesh_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftMesh_SetTex(c: &mut Ctx, mut p_mesh_1: u32, mut p_tex_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_SetTex", &PROBE_N, &[(p_mesh_1) as u32, (p_tex_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_tex_2; let a_ = (p_mesh_1).wrapping_add(72); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_SetTex", &PROBE_N, &[], true); }
    r_
}

pub fn EftMesh_SetMtx(c: &mut Ctx, mut p_mesh_1: u32, mut p_m_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_SetMtx", &PROBE_N, &[(p_mesh_1) as u32, (p_m_2) as u32], false); }
    let r_ = 'f: {
    F::Mtx_Copy(c, p_mesh_1, p_m_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_SetMtx", &PROBE_N, &[], true); }
    r_
}

pub fn EftMesh_Copy(c: &mut Ctx, mut p_dst_1: u32, mut p_src_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_Copy", &PROBE_N, &[(p_dst_1) as u32, (p_src_2) as u32], false); }
    let r_ = 'f: {
    { let s_ = p_src_2; let d_ = p_dst_1; rt::memcpy(c, d_, s_, 144); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_Copy", &PROBE_N, &[], true); }
    r_
}

pub fn EftMesh_SetLayer(c: &mut Ctx, mut p_mesh_1: u32, mut p_blend_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_SetLayer", &PROBE_N, &[(p_mesh_1) as u32, (p_blend_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_blend_2; let a_ = (p_mesh_1).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_SetLayer", &PROBE_N, &[], true); }
    r_
}

pub fn EftMesh_SetTexBase(c: &mut Ctx, mut p_mesh_1: u32, mut p_base_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_SetTexBase", &PROBE_N, &[(p_mesh_1) as u32, (p_base_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_base_2; let a_ = (p_mesh_1).wrapping_add(100); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_SetTexBase", &PROBE_N, &[], true); }
    r_
}

pub fn EftMesh_SetClip(c: &mut Ctx, mut p_mesh_1: u32, mut p_on_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_SetClip", &PROBE_N, &[(p_mesh_1) as u32, (p_on_2) as u32], false); }
    let r_ = 'f: {
    { let r_ = (!(8i32)); let a_ = (p_mesh_1).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    if ((p_on_2) != 0) {
        { let r_ = 8i32; let a_ = (p_mesh_1).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_SetClip", &PROBE_N, &[], true); }
    r_
}

pub fn EftMesh_SetRepeat(c: &mut Ctx, mut p_mesh_1: u32, mut p_on_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_SetRepeat", &PROBE_N, &[(p_mesh_1) as u32, (p_on_2) as u32], false); }
    let r_ = 'f: {
    { let r_ = (!(1i32)); let a_ = (p_mesh_1).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    if ((p_on_2) != 0) {
        { let r_ = 1i32; let a_ = (p_mesh_1).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_SetRepeat", &PROBE_N, &[], true); }
    r_
}

pub fn EftMesh_SetFlag2(c: &mut Ctx, mut p_mesh_1: u32, mut p_on_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_SetFlag2", &PROBE_N, &[(p_mesh_1) as u32, (p_on_2) as u32], false); }
    let r_ = 'f: {
    { let r_ = (!(2i32)); let a_ = (p_mesh_1).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    if ((p_on_2) != 0) {
        { let r_ = 2i32; let a_ = (p_mesh_1).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_SetFlag2", &PROBE_N, &[], true); }
    r_
}

pub fn EftMesh_SetZFlip(c: &mut Ctx, mut p_mesh_1: u32, mut p_on_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_SetZFlip", &PROBE_N, &[(p_mesh_1) as u32, (p_on_2) as u32], false); }
    let r_ = 'f: {
    { let r_ = (!(4i32)); let a_ = (p_mesh_1).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    if ((p_on_2) != 0) {
        { let r_ = 4i32; let a_ = (p_mesh_1).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_SetZFlip", &PROBE_N, &[], true); }
    r_
}

pub fn EftMesh_SetZOfs(c: &mut Ctx, mut p_mesh_1: u32, mut p_ofs_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_SetZOfs", &PROBE_N, &[(p_mesh_1) as u32, (p_ofs_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_ofs_2; let a_ = (p_mesh_1).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_SetZOfs", &PROBE_N, &[], true); }
    r_
}

pub fn EftMesh_SetUvOfs(c: &mut Ctx, mut p_mesh_1: u32, mut p_u_2: F32, mut p_v_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_SetUvOfs", &PROBE_N, &[(p_mesh_1) as u32, (p_u_2).0, (p_v_3).0], false); }
    let r_ = 'f: {
    { let v_ = p_u_2; let a_ = (p_mesh_1).wrapping_add(80); c.m.w32(a_, (v_).0); };
    { let v_ = p_v_3; let a_ = ((p_mesh_1).wrapping_add(80)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = ((p_mesh_1).wrapping_add(80)).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = ((p_mesh_1).wrapping_add(80)).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_SetUvOfs", &PROBE_N, &[], true); }
    r_
}

pub fn EftMesh_SetUvScale(c: &mut Ctx, mut p_mesh_1: u32, mut p_scale_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_SetUvScale", &PROBE_N, &[(p_mesh_1) as u32, (p_scale_2).0], false); }
    let r_ = 'f: {
    { let v_ = p_scale_2; let a_ = (p_mesh_1).wrapping_add(96); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_SetUvScale", &PROBE_N, &[], true); }
    r_
}

pub fn EftMesh_SetColor(c: &mut Ctx, mut p_mesh_1: u32, mut p_r_2: u8, mut p_g_3: u8, mut p_b_4: u8, mut p_a_5: u8) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_SetColor", &PROBE_N, &[(p_mesh_1) as u32, (p_r_2) as u32, (p_g_3) as u32, (p_b_4) as u32, (p_a_5) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_r_2; let a_ = (p_mesh_1).wrapping_add(64); c.m.w8(a_, v_); };
    { let v_ = p_g_3; let a_ = (p_mesh_1).wrapping_add(65); c.m.w8(a_, v_); };
    { let v_ = p_b_4; let a_ = (p_mesh_1).wrapping_add(66); c.m.w8(a_, v_); };
    { let v_ = p_a_5; let a_ = (p_mesh_1).wrapping_add(67); c.m.w8(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_SetColor", &PROBE_N, &[], true); }
    r_
}

pub fn EftMesh_SetOwner(c: &mut Ctx, mut p_mesh_1: u32, mut p_objId_2: i32, mut p_type_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_SetOwner", &PROBE_N, &[(p_mesh_1) as u32, (p_objId_2) as u32, (p_type_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_objId_2; let a_ = (p_mesh_1).wrapping_add(116); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_type_3; let a_ = (p_mesh_1).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_SetOwner", &PROBE_N, &[], true); }
    r_
}

pub fn EftMesh_Draw(c: &mut Ctx, mut p_mesh_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_Draw", &PROBE_N, &[(p_mesh_1) as u32], false); }
    let r_ = 'f: {
    if ((((((((((c.m.r32((p_mesh_1).wrapping_add(112)) as i32)) >= (0i32)) as i32)) != 0) && (({ let a0_ = (c.m.r32((p_mesh_1).wrapping_add(116)) as i32); let a1_ = (c.m.r32((p_mesh_1).wrapping_add(112)) as i32); F::BtlScene_IsEffectHidden(c, a0_, a1_) }) != 0)) as i32)) != 0) {
        break 'f;
    }
    if (((((c.m.r32((p_mesh_1).wrapping_add(104)) as i32)) & (8i32))) != 0) {
        F::EftMesh_DrawClip(c, p_mesh_1);
    } else {
        F::EftMesh_DrawPlain(c, p_mesh_1);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftMesh_DrawNow(c: &mut Ctx, mut p_mesh_1: u32, mut p_abe_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_DrawNow", &PROBE_N, &[(p_mesh_1) as u32, (p_abe_2) as u32], false); }
    let r_ = 'f: {
    if ((((((((((c.m.r32((p_mesh_1).wrapping_add(112)) as i32)) >= (0i32)) as i32)) != 0) && (({ let a0_ = (c.m.r32((p_mesh_1).wrapping_add(116)) as i32); let a1_ = (c.m.r32((p_mesh_1).wrapping_add(112)) as i32); F::BtlScene_IsEffectHidden(c, a0_, a1_) }) != 0)) as i32)) != 0) {
        break 'f;
    }
    if (((((c.m.r32((p_mesh_1).wrapping_add(104)) as i32)) & (8i32))) != 0) {
        F::EftMesh_DrawNowClip(c, p_mesh_1, p_abe_2);
    } else {
        F::EftMesh_DrawNowPlain(c, p_mesh_1, p_abe_2);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_DrawNow", &PROBE_N, &[], true); }
    r_
}

pub fn EftMesh_DrawClip(c: &mut Ctx, mut p_mesh_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_DrawClip", &PROBE_N, &[(p_mesh_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(464) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 464);
    let mut l_tris_2: u32 = 0;
    let mut l_tex_3: u32 = 0;
    let mut l_blend_4: i32 = 0;
    let mut l_zOfs_5: i32 = 0;
    let mut l_flag1_6: i32 = 0;
    let mut l_flag2_7: i32 = 0;
    let mut l_zflip_8: i32 = 0;
    let mut l_tex0_9: u64 = 0;
    let mut l_n_10: i32 = 0;
    let mut l_g_11: i32 = 0;
    let mut l_i_12: i32 = 0;
    let mut l_group_13: u32 = 0;
    let mut l_verts_14: u32 = 0;
    let mut l_tri_15: u32 = 0;
    let mut l_a_16: u32 = 0;
    let mut l_b_17: u32 = 0;
    let mut l_c_18: u32 = 0;
    let mut l_hasTex_19: i32 = 0;
    let mut l_scale_20: F32 = F32(0);
    let r_ = 'f: {
        l_tex0_9 = ((0i32) as u64);
        { let v_ = rt::fmul(rt::i2f(((c.m.r8((p_mesh_1).wrapping_add(64))) as i32)), F32(0x3c000000)); let a_ = fp.wrapping_add(432); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::i2f(((c.m.r8((p_mesh_1).wrapping_add(65))) as i32)), F32(0x3c000000)); let a_ = (fp.wrapping_add(432)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::i2f(((c.m.r8((p_mesh_1).wrapping_add(66))) as i32)), F32(0x3c000000)); let a_ = (fp.wrapping_add(432)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::i2f(((c.m.r8((p_mesh_1).wrapping_add(67))) as i32)), F32(0x3c000000)); let a_ = (fp.wrapping_add(432)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((p_mesh_1).wrapping_add(80))); let a_ = fp.wrapping_add(448); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((p_mesh_1).wrapping_add(80)).wrapping_add(4))); let a_ = (fp.wrapping_add(448)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(448)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(448)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        l_flag1_6 = 0i32;
        if (((((c.m.r32((p_mesh_1).wrapping_add(104)) as i32)) & (1i32))) != 0) {
            l_flag1_6 = 1i32;
        }
        l_flag2_7 = ((((c.m.r32((p_mesh_1).wrapping_add(104)) as i32)).wrapping_shr((1i32) as u32)) & (1i32));
        l_zflip_8 = ((((c.m.r32((p_mesh_1).wrapping_add(104)) as i32)).wrapping_shr((2i32) as u32)) & (1i32));
        l_n_10 = { let a_ = (c.m.r32((p_mesh_1).wrapping_add(120))).wrapping_add(4); (c.m.r32(a_) as i32) };
        l_group_13 = c.m.r32((p_mesh_1).wrapping_add(124));
        l_tris_2 = c.m.r32((p_mesh_1).wrapping_add(128));
        l_verts_14 = c.m.r32((p_mesh_1).wrapping_add(132));
        l_tex_3 = c.m.r32((p_mesh_1).wrapping_add(72));
        l_blend_4 = (c.m.r32((p_mesh_1).wrapping_add(68)) as i32);
        l_zOfs_5 = (c.m.r32((p_mesh_1).wrapping_add(108)) as i32);
        l_scale_20 = F32(c.m.r32((p_mesh_1).wrapping_add(96)));
        F::Vu0Cur_Push(c);
        { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
        {
            l_g_11 = 0i32;
            'l1: loop {
                if !(((((l_g_11) < (l_n_10)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_tri_15 = (l_tris_2).wrapping_add(((((c.m.r32((l_group_13).wrapping_add(8)) as i32)) as u32)).wrapping_mul(16));
                    {
                        l_i_12 = 0i32;
                        'l3: loop {
                            if !(((((l_i_12) < ((c.m.r32(l_group_13) as i32))) as i32)) != 0) { break 'l3; }
                            'c4: {
                                l_a_16 = (l_verts_14).wrapping_add(((((c.m.r32((l_tri_15).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(48));
                                l_b_17 = (l_verts_14).wrapping_add(((((c.m.r32((l_tri_15).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(48));
                                l_c_18 = (l_verts_14).wrapping_add(((((c.m.r32((l_tri_15).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(48));
                                F::Mtx_MulVec4(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(48)), p_mesh_1, l_a_16);
                                F::Mtx_MulVec4(c, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(48)), p_mesh_1, l_b_17);
                                F::Mtx_MulVec4(c, (fp).wrapping_add((((2i32) as u32)).wrapping_mul(48)), p_mesh_1, l_c_18);
                                F::Vec4_Scale(c, ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(16), (l_a_16).wrapping_add(32), l_scale_20);
                                F::Vec4_Scale(c, ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(48))).wrapping_add(16), (l_b_17).wrapping_add(32), l_scale_20);
                                F::Vec4_Scale(c, ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(48))).wrapping_add(16), (l_c_18).wrapping_add(32), l_scale_20);
                                F::Vec4_Add(c, ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(16), ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(16), fp.wrapping_add(448));
                                F::Vec4_Add(c, ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(48))).wrapping_add(16), ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(48))).wrapping_add(16), fp.wrapping_add(448));
                                F::Vec4_Add(c, ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(48))).wrapping_add(16), ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(48))).wrapping_add(16), fp.wrapping_add(448));
                                F::Vec4_Mul(c, ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(32), (l_a_16).wrapping_add(16), fp.wrapping_add(432));
                                F::Vec4_Mul(c, ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(48))).wrapping_add(32), (l_b_17).wrapping_add(16), fp.wrapping_add(432));
                                F::Vec4_Mul(c, ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(48))).wrapping_add(32), (l_c_18).wrapping_add(16), fp.wrapping_add(432));
                                l_hasTex_19 = 0i32;
                                if ((((((((((c.m.r32((l_tri_15).wrapping_add(12)) as i32)) >= (0i32)) as i32)) != 0) && (((((l_tex_3) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
                                    l_tex0_9 = { let a_ = (l_tex_3).wrapping_add((((((c.m.r32((l_tri_15).wrapping_add(12)) as i32)).wrapping_add((c.m.r32((p_mesh_1).wrapping_add(100)) as i32))).wrapping_shl((4i32) as u32)) as u32)); c.m.r64(a_) };
                                    l_hasTex_19 = 1i32;
                                }
                                { let o_ = l_tri_15; let n_ = (o_).wrapping_add(16); l_tri_15 = n_; };
                                { let v_ = F32(0x3f800000); let a_ = (((fp).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x00000000); let a_ = (((fp).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = (((fp).wrapping_add((((1i32) as u32)).wrapping_mul(48))).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x00000000); let a_ = (((fp).wrapping_add((((1i32) as u32)).wrapping_mul(48))).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = (((fp).wrapping_add((((2i32) as u32)).wrapping_mul(48))).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x00000000); let a_ = (((fp).wrapping_add((((2i32) as u32)).wrapping_mul(48))).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                F::EftMesh_DrawTriClip(c, fp, l_hasTex_19, l_blend_4, l_flag1_6, l_flag2_7, l_zflip_8, l_tex0_9, l_zOfs_5);
                            }
                            { let o_ = l_i_12; let n_ = (o_).wrapping_add(1); l_i_12 = n_; };
                        }
                    }
                    { let o_ = l_group_13; let n_ = (o_).wrapping_add(16); l_group_13 = n_; };
                }
                { let o_ = l_g_11; let n_ = (o_).wrapping_add(1); l_g_11 = n_; };
            }
        }
        F::Vu0Cur_Pop(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_DrawClip", &PROBE_N, &[], true); }
    r_
}

pub fn EftMesh_DrawNowClip(c: &mut Ctx, mut p_mesh_1: u32, mut p_abe_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_DrawNowClip", &PROBE_N, &[(p_mesh_1) as u32, (p_abe_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(464) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 464);
    let mut l_tris_3: u32 = 0;
    let mut l_tex_4: u32 = 0;
    let mut l_n_5: i32 = 0;
    let mut l_g_6: i32 = 0;
    let mut l_i_7: i32 = 0;
    let mut l_group_8: u32 = 0;
    let mut l_verts_9: u32 = 0;
    let mut l_tri_10: u32 = 0;
    let mut l_a_11: u32 = 0;
    let mut l_b_12: u32 = 0;
    let mut l_c_13: u32 = 0;
    let mut l_tex0_14: u64 = 0;
    let mut l_scale_15: F32 = F32(0);
    let r_ = 'f: {
        { let v_ = rt::fmul(rt::i2f(((c.m.r8((p_mesh_1).wrapping_add(64))) as i32)), F32(0x3c000000)); let a_ = fp.wrapping_add(432); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::i2f(((c.m.r8((p_mesh_1).wrapping_add(65))) as i32)), F32(0x3c000000)); let a_ = (fp.wrapping_add(432)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::i2f(((c.m.r8((p_mesh_1).wrapping_add(66))) as i32)), F32(0x3c000000)); let a_ = (fp.wrapping_add(432)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::i2f(((c.m.r8((p_mesh_1).wrapping_add(67))) as i32)), F32(0x3c000000)); let a_ = (fp.wrapping_add(432)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((p_mesh_1).wrapping_add(80))); let a_ = fp.wrapping_add(448); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((p_mesh_1).wrapping_add(80)).wrapping_add(4))); let a_ = (fp.wrapping_add(448)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(448)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(448)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        l_n_5 = { let a_ = (c.m.r32((p_mesh_1).wrapping_add(120))).wrapping_add(4); (c.m.r32(a_) as i32) };
        l_group_8 = c.m.r32((p_mesh_1).wrapping_add(124));
        l_tris_3 = c.m.r32((p_mesh_1).wrapping_add(128));
        l_verts_9 = c.m.r32((p_mesh_1).wrapping_add(132));
        l_tex_4 = c.m.r32((p_mesh_1).wrapping_add(72));
        l_scale_15 = F32(c.m.r32((p_mesh_1).wrapping_add(96)));
        F::Vu0Cur_Push(c);
        { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
        {
            l_g_6 = 0i32;
            'l1: loop {
                if !(((((l_g_6) < (l_n_5)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_tri_10 = (l_tris_3).wrapping_add(((((c.m.r32((l_group_8).wrapping_add(8)) as i32)) as u32)).wrapping_mul(16));
                    {
                        l_i_7 = 0i32;
                        'l3: loop {
                            if !(((((l_i_7) < ((c.m.r32(l_group_8) as i32))) as i32)) != 0) { break 'l3; }
                            'c4: {
                                l_a_11 = (l_verts_9).wrapping_add(((((c.m.r32((l_tri_10).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(48));
                                l_b_12 = (l_verts_9).wrapping_add(((((c.m.r32((l_tri_10).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(48));
                                l_c_13 = (l_verts_9).wrapping_add(((((c.m.r32((l_tri_10).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(48));
                                F::Mtx_MulVec4(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(48)), p_mesh_1, l_a_11);
                                F::Mtx_MulVec4(c, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(48)), p_mesh_1, l_b_12);
                                F::Mtx_MulVec4(c, (fp).wrapping_add((((2i32) as u32)).wrapping_mul(48)), p_mesh_1, l_c_13);
                                F::Vec4_Scale(c, ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(16), (l_a_11).wrapping_add(32), l_scale_15);
                                F::Vec4_Scale(c, ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(48))).wrapping_add(16), (l_b_12).wrapping_add(32), l_scale_15);
                                F::Vec4_Scale(c, ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(48))).wrapping_add(16), (l_c_13).wrapping_add(32), l_scale_15);
                                F::Vec4_Add(c, ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(16), ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(16), fp.wrapping_add(448));
                                F::Vec4_Add(c, ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(48))).wrapping_add(16), ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(48))).wrapping_add(16), fp.wrapping_add(448));
                                F::Vec4_Add(c, ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(48))).wrapping_add(16), ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(48))).wrapping_add(16), fp.wrapping_add(448));
                                F::Vec4_Mul(c, ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(32), (l_a_11).wrapping_add(16), fp.wrapping_add(432));
                                F::Vec4_Mul(c, ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(48))).wrapping_add(32), (l_b_12).wrapping_add(16), fp.wrapping_add(432));
                                F::Vec4_Mul(c, ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(48))).wrapping_add(32), (l_c_13).wrapping_add(16), fp.wrapping_add(432));
                                l_tex0_14 = { let a_ = (l_tex_4).wrapping_add((((((c.m.r32((l_tri_10).wrapping_add(12)) as i32)).wrapping_add((c.m.r32((p_mesh_1).wrapping_add(100)) as i32))).wrapping_shl((4i32) as u32)) as u32)); c.m.r64(a_) };
                                { let o_ = l_tri_10; let n_ = (o_).wrapping_add(16); l_tri_10 = n_; };
                                { let v_ = F32(0x3f800000); let a_ = (((fp).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x00000000); let a_ = (((fp).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = (((fp).wrapping_add((((1i32) as u32)).wrapping_mul(48))).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x00000000); let a_ = (((fp).wrapping_add((((1i32) as u32)).wrapping_mul(48))).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = (((fp).wrapping_add((((2i32) as u32)).wrapping_mul(48))).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x00000000); let a_ = (((fp).wrapping_add((((2i32) as u32)).wrapping_mul(48))).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                F::EftMesh_DrawNowTriClip(c, fp, l_tex0_14, p_abe_2);
                            }
                            { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
                        }
                    }
                    { let o_ = l_group_8; let n_ = (o_).wrapping_add(16); l_group_8 = n_; };
                }
                { let o_ = l_g_6; let n_ = (o_).wrapping_add(1); l_g_6 = n_; };
            }
        }
        F::Vu0Cur_Pop(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_DrawNowClip", &PROBE_N, &[], true); }
    r_
}

pub fn EftMesh_DrawPlain(c: &mut Ctx, mut p_mesh_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_DrawPlain", &PROBE_N, &[(p_mesh_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(272) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 272);
    let mut l_tris_2: u32 = 0;
    let mut l_tex_3: u32 = 0;
    let mut l_blend_4: i32 = 0;
    let mut l_zOfs_5: i32 = 0;
    let mut l_flag1_6: i32 = 0;
    let mut l_flag2_7: i32 = 0;
    let mut l_zflip_8: i32 = 0;
    let mut l_tex0_9: u64 = 0;
    let mut l_n_10: i32 = 0;
    let mut l_g_11: i32 = 0;
    let mut l_i_12: i32 = 0;
    let mut l_group_13: u32 = 0;
    let mut l_verts_14: u32 = 0;
    let mut l_tri_15: u32 = 0;
    let mut l_a_16: u32 = 0;
    let mut l_b_17: u32 = 0;
    let mut l_c_18: u32 = 0;
    let mut l_hasTex_19: i32 = 0;
    let mut l_z_20: i32 = 0;
    let mut l_scale_21: F32 = F32(0);
    let r_ = 'f: {
        l_tex0_9 = ((0i32) as u64);
        { let v_ = rt::fmul(rt::i2f(((c.m.r8((p_mesh_1).wrapping_add(64))) as i32)), F32(0x3c000000)); let a_ = fp.wrapping_add(96); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::i2f(((c.m.r8((p_mesh_1).wrapping_add(65))) as i32)), F32(0x3c000000)); let a_ = (fp.wrapping_add(96)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::i2f(((c.m.r8((p_mesh_1).wrapping_add(66))) as i32)), F32(0x3c000000)); let a_ = (fp.wrapping_add(96)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::i2f(((c.m.r8((p_mesh_1).wrapping_add(67))) as i32)), F32(0x3c000000)); let a_ = (fp.wrapping_add(96)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((p_mesh_1).wrapping_add(80))); let a_ = fp.wrapping_add(256); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((p_mesh_1).wrapping_add(80)).wrapping_add(4))); let a_ = (fp.wrapping_add(256)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(256)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(256)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        l_flag1_6 = 0i32;
        if (((((c.m.r32((p_mesh_1).wrapping_add(104)) as i32)) & (1i32))) != 0) {
            l_flag1_6 = 1i32;
        }
        l_flag2_7 = ((((c.m.r32((p_mesh_1).wrapping_add(104)) as i32)).wrapping_shr((1i32) as u32)) & (1i32));
        l_zflip_8 = ((((c.m.r32((p_mesh_1).wrapping_add(104)) as i32)).wrapping_shr((2i32) as u32)) & (1i32));
        l_n_10 = { let a_ = (c.m.r32((p_mesh_1).wrapping_add(120))).wrapping_add(4); (c.m.r32(a_) as i32) };
        l_group_13 = c.m.r32((p_mesh_1).wrapping_add(124));
        l_tris_2 = c.m.r32((p_mesh_1).wrapping_add(128));
        l_verts_14 = c.m.r32((p_mesh_1).wrapping_add(132));
        l_tex_3 = c.m.r32((p_mesh_1).wrapping_add(72));
        l_blend_4 = (c.m.r32((p_mesh_1).wrapping_add(68)) as i32);
        l_zOfs_5 = (c.m.r32((p_mesh_1).wrapping_add(108)) as i32);
        l_scale_21 = F32(c.m.r32((p_mesh_1).wrapping_add(96)));
        F::Vu0Cur_Push(c);
        { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
        {
            l_g_11 = 0i32;
            'l1: loop {
                if !(((((l_g_11) < (l_n_10)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_tri_15 = (l_tris_2).wrapping_add(((((c.m.r32((l_group_13).wrapping_add(8)) as i32)) as u32)).wrapping_mul(16));
                    {
                        l_i_12 = 0i32;
                        'l3: loop {
                            if !(((((l_i_12) < ((c.m.r32(l_group_13) as i32))) as i32)) != 0) { break 'l3; }
                            'c4: {
                                l_a_16 = (l_verts_14).wrapping_add(((((c.m.r32((l_tri_15).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(48));
                                l_b_17 = (l_verts_14).wrapping_add(((((c.m.r32((l_tri_15).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(48));
                                l_c_18 = (l_verts_14).wrapping_add(((((c.m.r32((l_tri_15).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(48));
                                F::Mtx_MulVec4(c, (fp.wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), p_mesh_1, l_a_16);
                                F::Mtx_MulVec4(c, (fp.wrapping_add(112)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), p_mesh_1, l_b_17);
                                F::Mtx_MulVec4(c, (fp.wrapping_add(112)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), p_mesh_1, l_c_18);
                                F::Vec4_Scale(c, (fp.wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (l_a_16).wrapping_add(32), l_scale_21);
                                F::Vec4_Scale(c, (fp.wrapping_add(160)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (l_b_17).wrapping_add(32), l_scale_21);
                                F::Vec4_Scale(c, (fp.wrapping_add(160)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (l_c_18).wrapping_add(32), l_scale_21);
                                F::Vec4_Add(c, (fp.wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), fp.wrapping_add(256));
                                F::Vec4_Add(c, (fp.wrapping_add(160)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(160)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), fp.wrapping_add(256));
                                F::Vec4_Add(c, (fp.wrapping_add(160)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(160)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), fp.wrapping_add(256));
                                F::Vec4_Mul(c, (fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (l_a_16).wrapping_add(16), fp.wrapping_add(96));
                                F::Vec4_Mul(c, (fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (l_b_17).wrapping_add(16), fp.wrapping_add(96));
                                F::Vec4_Mul(c, (fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (l_c_18).wrapping_add(16), fp.wrapping_add(96));
                                { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x00000000); let a_ = ((fp.wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(160)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x00000000); let a_ = ((fp.wrapping_add(160)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(160)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x00000000); let a_ = ((fp.wrapping_add(160)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                { let _ = F::Vu0Cur_ProjectPointsStq(c, fp, fp.wrapping_add(208), fp.wrapping_add(112), fp.wrapping_add(160), 3i32); };
                                l_z_20 = ((c.m.r32(((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)).wrapping_shr((8i32) as u32);
                                if ((l_zflip_8) != 0) {
                                    l_z_20 = (4096i32).wrapping_sub(l_z_20);
                                }
                                if (((((((((((((((((((l_z_20) as u32)) < (((4096i32) as u32))) as i32)) != 0) && ((((!((F::EftMesh_IsOffScreen(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)))) != 0)) as i32)) != 0)) as i32)) != 0) && ((((!((F::EftMesh_IsOffScreen(c, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)))) != 0)) as i32)) != 0)) as i32)) != 0) && ((((!((F::EftMesh_IsOffScreen(c, (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)))) != 0)) as i32)) != 0)) as i32)) != 0) {
                                    l_hasTex_19 = 0i32;
                                    if ((((((((((c.m.r32((l_tri_15).wrapping_add(12)) as i32)) >= (0i32)) as i32)) != 0) && (((((l_tex_3) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
                                        l_tex0_9 = { let a_ = (l_tex_3).wrapping_add((((((c.m.r32((l_tri_15).wrapping_add(12)) as i32)).wrapping_add((c.m.r32((p_mesh_1).wrapping_add(100)) as i32))).wrapping_shl((4i32) as u32)) as u32)); c.m.r64(a_) };
                                        l_hasTex_19 = 1i32;
                                    }
                                    F::EftMesh_QueueTri(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(208)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(208)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(208)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), l_flag1_6, l_flag2_7, l_hasTex_19, l_blend_4, (l_z_20).wrapping_add(l_zOfs_5), l_tex0_9);
                                }
                                { let o_ = l_tri_15; let n_ = (o_).wrapping_add(16); l_tri_15 = n_; };
                            }
                            { let o_ = l_i_12; let n_ = (o_).wrapping_add(1); l_i_12 = n_; };
                        }
                    }
                    { let o_ = l_group_13; let n_ = (o_).wrapping_add(16); l_group_13 = n_; };
                }
                { let o_ = l_g_11; let n_ = (o_).wrapping_add(1); l_g_11 = n_; };
            }
        }
        F::Vu0Cur_Pop(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_DrawPlain", &PROBE_N, &[], true); }
    r_
}

pub fn EftMesh_DrawNowPlain(c: &mut Ctx, mut p_mesh_1: u32, mut p_abe_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_DrawNowPlain", &PROBE_N, &[(p_mesh_1) as u32, (p_abe_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(272) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 272);
    let mut l_tris_3: u32 = 0;
    let mut l_tex_4: u32 = 0;
    let mut l_n_5: i32 = 0;
    let mut l_g_6: i32 = 0;
    let mut l_i_7: i32 = 0;
    let mut l_group_8: u32 = 0;
    let mut l_verts_9: u32 = 0;
    let mut l_tri_10: u32 = 0;
    let mut l_a_11: u32 = 0;
    let mut l_b_12: u32 = 0;
    let mut l_c_13: u32 = 0;
    let mut l_tex0_14: u64 = 0;
    let mut l_scale_15: F32 = F32(0);
    let r_ = 'f: {
        { let v_ = rt::fmul(rt::i2f(((c.m.r8((p_mesh_1).wrapping_add(64))) as i32)), F32(0x3c000000)); let a_ = fp.wrapping_add(96); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::i2f(((c.m.r8((p_mesh_1).wrapping_add(65))) as i32)), F32(0x3c000000)); let a_ = (fp.wrapping_add(96)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::i2f(((c.m.r8((p_mesh_1).wrapping_add(66))) as i32)), F32(0x3c000000)); let a_ = (fp.wrapping_add(96)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::i2f(((c.m.r8((p_mesh_1).wrapping_add(67))) as i32)), F32(0x3c000000)); let a_ = (fp.wrapping_add(96)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((p_mesh_1).wrapping_add(80))); let a_ = fp.wrapping_add(256); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((p_mesh_1).wrapping_add(80)).wrapping_add(4))); let a_ = (fp.wrapping_add(256)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(256)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(256)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        l_n_5 = { let a_ = (c.m.r32((p_mesh_1).wrapping_add(120))).wrapping_add(4); (c.m.r32(a_) as i32) };
        l_group_8 = c.m.r32((p_mesh_1).wrapping_add(124));
        l_tris_3 = c.m.r32((p_mesh_1).wrapping_add(128));
        l_verts_9 = c.m.r32((p_mesh_1).wrapping_add(132));
        l_tex_4 = c.m.r32((p_mesh_1).wrapping_add(72));
        l_scale_15 = F32(c.m.r32((p_mesh_1).wrapping_add(96)));
        F::Vu0Cur_Push(c);
        { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
        {
            l_g_6 = 0i32;
            'l1: loop {
                if !(((((l_g_6) < (l_n_5)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_tri_10 = (l_tris_3).wrapping_add(((((c.m.r32((l_group_8).wrapping_add(8)) as i32)) as u32)).wrapping_mul(16));
                    {
                        l_i_7 = 0i32;
                        'l3: loop {
                            if !(((((l_i_7) < ((c.m.r32(l_group_8) as i32))) as i32)) != 0) { break 'l3; }
                            'c4: {
                                l_a_11 = (l_verts_9).wrapping_add(((((c.m.r32((l_tri_10).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(48));
                                l_b_12 = (l_verts_9).wrapping_add(((((c.m.r32((l_tri_10).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(48));
                                l_c_13 = (l_verts_9).wrapping_add(((((c.m.r32((l_tri_10).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(48));
                                F::Mtx_MulVec4(c, (fp.wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), p_mesh_1, l_a_11);
                                F::Mtx_MulVec4(c, (fp.wrapping_add(112)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), p_mesh_1, l_b_12);
                                F::Mtx_MulVec4(c, (fp.wrapping_add(112)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), p_mesh_1, l_c_13);
                                F::Vec4_Scale(c, (fp.wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (l_a_11).wrapping_add(32), l_scale_15);
                                F::Vec4_Scale(c, (fp.wrapping_add(160)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (l_b_12).wrapping_add(32), l_scale_15);
                                F::Vec4_Scale(c, (fp.wrapping_add(160)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (l_c_13).wrapping_add(32), l_scale_15);
                                F::Vec4_Add(c, (fp.wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), fp.wrapping_add(256));
                                F::Vec4_Add(c, (fp.wrapping_add(160)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(160)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), fp.wrapping_add(256));
                                F::Vec4_Add(c, (fp.wrapping_add(160)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(160)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), fp.wrapping_add(256));
                                F::Vec4_Mul(c, (fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (l_a_11).wrapping_add(16), fp.wrapping_add(96));
                                F::Vec4_Mul(c, (fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (l_b_12).wrapping_add(16), fp.wrapping_add(96));
                                F::Vec4_Mul(c, (fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (l_c_13).wrapping_add(16), fp.wrapping_add(96));
                                { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x00000000); let a_ = ((fp.wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(160)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x00000000); let a_ = ((fp.wrapping_add(160)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(160)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x00000000); let a_ = ((fp.wrapping_add(160)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                { let _ = F::Vu0Cur_ProjectPointsStq(c, fp, fp.wrapping_add(208), fp.wrapping_add(112), fp.wrapping_add(160), 3i32); };
                                if ((((((((((((!((F::EftMesh_IsOffScreen(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)))) != 0)) as i32)) != 0) && ((((!((F::EftMesh_IsOffScreen(c, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)))) != 0)) as i32)) != 0)) as i32)) != 0) && ((((!((F::EftMesh_IsOffScreen(c, (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)))) != 0)) as i32)) != 0)) as i32)) != 0) {
                                    l_tex0_14 = { let a_ = (l_tex_4).wrapping_add((((((c.m.r32((l_tri_10).wrapping_add(12)) as i32)).wrapping_add((c.m.r32((p_mesh_1).wrapping_add(100)) as i32))).wrapping_shl((4i32) as u32)) as u32)); c.m.r64(a_) };
                                    F::EftMesh_SendTri(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(208)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(208)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(208)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), l_tex0_14, p_abe_2);
                                }
                                { let o_ = l_tri_10; let n_ = (o_).wrapping_add(16); l_tri_10 = n_; };
                            }
                            { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
                        }
                    }
                    { let o_ = l_group_8; let n_ = (o_).wrapping_add(16); l_group_8 = n_; };
                }
                { let o_ = l_g_6; let n_ = (o_).wrapping_add(1); l_g_6 = n_; };
            }
        }
        F::Vu0Cur_Pop(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_DrawNowPlain", &PROBE_N, &[], true); }
    r_
}

pub fn EftMesh_IsOffScreen(c: &mut Ctx, p_p_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_IsOffScreen", &PROBE_N, &[(p_p_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    rt::memcpy(c, fp, p_p_1, 16);
    let r_ = 'f: {
        if ((((((c.m.r32((fp).wrapping_add(8)) as i32)) <= (0i32)) as i32)) != 0) {
            break 'f 1i32;
        }
        if ((((((c.m.r32(fp) as i32)) > (65519i32)) as i32)) != 0) {
            break 'f 1i32;
        }
        if ((((((c.m.r32(fp) as i32)) <= (0i32)) as i32)) != 0) {
            break 'f 1i32;
        }
        if ((((((c.m.r32((fp).wrapping_add(4)) as i32)) > (65519i32)) as i32)) != 0) {
            break 'f 1i32;
        }
        if ((((((c.m.r32((fp).wrapping_add(4)) as i32)) <= (0i32)) as i32)) != 0) {
            break 'f 1i32;
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_IsOffScreen", &PROBE_N, &[], true); }
    r_
}

pub fn EftMesh_DrawTriClip(c: &mut Ctx, mut p_v_1: u32, mut p_hasTex_2: i32, mut p_blend_3: i32, mut p_flag1_4: i32, mut p_flag2_5: i32, mut p_zflip_6: i32, mut p_tex0_7: u64, mut p_zOfs_8: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_DrawTriClip", &PROBE_N, &[(p_v_1) as u32, (p_hasTex_2) as u32, (p_blend_3) as u32, (p_flag1_4) as u32, (p_flag2_5) as u32, (p_zflip_6) as u32, (p_tex0_7) as u32, (p_zOfs_8) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(1056) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 1056);
    let mut l_plane_9: u32 = 0;
    let mut l_n_10: i32 = 0;
    let mut l_i_11: i32 = 0;
    let mut l_z_12: i32 = 0;
    let mut l_zp_13: u32 = 0;
    let r_ = 'f: {
        { let _ = { rt::memcpy(c, fp, p_v_1, ((((3i32) as u32)).wrapping_mul(48u32)) as u32); fp }; };
        p_v_1 = fp;
        l_plane_9 = F::EftGfx_GetClipPlanes(c);
        l_n_10 = 3i32;
        {
            l_i_11 = 0i32;
            'l1: loop {
                if !(((((l_i_11) < (5i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_n_10 = F::ClipPoly_ClipPlane(c, p_v_1, l_plane_9, l_n_10);
                    { let o_ = l_plane_9; let n_ = (o_).wrapping_add(16); l_plane_9 = n_; };
                }
                { let o_ = l_i_11; let n_ = (o_).wrapping_add(1); l_i_11 = n_; };
            }
        }
        if (((((l_n_10) == (0i32)) as i32)) != 0) {
            break 'f;
        }
        F::ClipPoly_ProjectCur(c, fp.wrapping_add(768), fp.wrapping_add(912), p_v_1, l_n_10);
        l_zp_13 = ((fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8);
        {
            l_i_11 = 2i32;
            'l3: loop {
                if !(((((l_i_11) < (l_n_10)) as i32)) != 0) { break 'l3; }
                'c4: {
                    l_z_12 = (rt::div_i32((((c.m.r32(((fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)).wrapping_add((c.m.r32((l_zp_13).wrapping_add(((((l_i_11).wrapping_sub(1i32)).wrapping_shl((4i32) as u32)) as u32))) as i32))).wrapping_add((c.m.r32((l_zp_13).wrapping_add((((l_i_11).wrapping_shl((4i32) as u32)) as u32))) as i32)), 3i32)).wrapping_shr((8i32) as u32);
                    if ((p_zflip_6) != 0) {
                        l_z_12 = (4096i32).wrapping_sub(l_z_12);
                    }
                    if ((((((c.m.r32(((fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) > (16777215i32)) as i32)) != 0) {
                        { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    }
                    if ((((((c.m.r32((l_zp_13).wrapping_add(((((l_i_11).wrapping_sub(1i32)).wrapping_shl((4i32) as u32)) as u32))) as i32)) > (16777215i32)) as i32)) != 0) {
                        { let v_ = 16777215i32; let a_ = (l_zp_13).wrapping_add(((((l_i_11).wrapping_sub(1i32)).wrapping_shl((4i32) as u32)) as u32)); c.m.w32(a_, (v_) as u32); };
                    }
                    if ((((((c.m.r32((l_zp_13).wrapping_add((((l_i_11).wrapping_shl((4i32) as u32)) as u32))) as i32)) > (16777215i32)) as i32)) != 0) {
                        { let v_ = 16777215i32; let a_ = (l_zp_13).wrapping_add((((l_i_11).wrapping_shl((4i32) as u32)) as u32)); c.m.w32(a_, (v_) as u32); };
                    }
                    if ((((((((((F::EftMesh_IsOffScreen(c, (fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16)))) != 0) && ((F::EftMesh_IsOffScreen(c, (fp.wrapping_add(768)).wrapping_add(((((l_i_11).wrapping_sub(1i32)) as u32)).wrapping_mul(16)))) != 0)) as i32)) != 0) && ((F::EftMesh_IsOffScreen(c, (fp.wrapping_add(768)).wrapping_add((((l_i_11) as u32)).wrapping_mul(16)))) != 0)) as i32)) != 0) {
                        break 'c4;
                    }
                    F::EftMesh_QueueTri(c, (fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(768)).wrapping_add(((((l_i_11).wrapping_sub(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(768)).wrapping_add((((l_i_11) as u32)).wrapping_mul(16)), ((p_v_1).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(32), ((p_v_1).wrapping_add(((((l_i_11).wrapping_sub(1i32)) as u32)).wrapping_mul(48))).wrapping_add(32), ((p_v_1).wrapping_add((((l_i_11) as u32)).wrapping_mul(48))).wrapping_add(32), (fp.wrapping_add(912)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(912)).wrapping_add(((((l_i_11).wrapping_sub(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(912)).wrapping_add((((l_i_11) as u32)).wrapping_mul(16)), p_flag1_4, p_flag2_5, p_hasTex_2, p_blend_3, (l_z_12).wrapping_add(p_zOfs_8), p_tex0_7);
                }
                { let o_ = l_i_11; let n_ = (o_).wrapping_add(1); l_i_11 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_DrawTriClip", &PROBE_N, &[], true); }
    r_
}

pub fn EftMesh_DrawNowTriClip(c: &mut Ctx, mut p_v_1: u32, mut p_tex0_2: u64, mut p_abe_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_DrawNowTriClip", &PROBE_N, &[(p_v_1) as u32, (p_tex0_2) as u32, (p_abe_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(1056) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 1056);
    let mut l_plane_4: u32 = 0;
    let mut l_n_5: i32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_zp_7: u32 = 0;
    let r_ = 'f: {
        { let _ = { rt::memcpy(c, fp, p_v_1, ((((3i32) as u32)).wrapping_mul(48u32)) as u32); fp }; };
        p_v_1 = fp;
        l_plane_4 = F::EftGfx_GetClipPlanes(c);
        l_n_5 = 3i32;
        {
            l_i_6 = 0i32;
            'l1: loop {
                if !(((((l_i_6) < (5i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_n_5 = F::ClipPoly_ClipPlane(c, p_v_1, l_plane_4, l_n_5);
                    { let o_ = l_plane_4; let n_ = (o_).wrapping_add(16); l_plane_4 = n_; };
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
        if (((((l_n_5) == (0i32)) as i32)) != 0) {
            break 'f;
        }
        F::ClipPoly_ProjectCur(c, fp.wrapping_add(768), fp.wrapping_add(912), p_v_1, l_n_5);
        l_zp_7 = ((fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8);
        {
            l_i_6 = 2i32;
            'l3: loop {
                if !(((((l_i_6) < (l_n_5)) as i32)) != 0) { break 'l3; }
                'c4: {
                    if ((((((c.m.r32(((fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) > (16777215i32)) as i32)) != 0) {
                        { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    }
                    if ((((((c.m.r32((((fp.wrapping_add(768)).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) > (16777215i32)) as i32)) != 0) {
                        { let v_ = 16777215i32; let a_ = (((fp.wrapping_add(768)).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    if ((((((c.m.r32((l_zp_7).wrapping_add((((l_i_6).wrapping_shl((4i32) as u32)) as u32))) as i32)) > (16777215i32)) as i32)) != 0) {
                        { let v_ = 16777215i32; let a_ = (l_zp_7).wrapping_add((((l_i_6).wrapping_shl((4i32) as u32)) as u32)); c.m.w32(a_, (v_) as u32); };
                    }
                    if ((((((((((F::EftMesh_IsOffScreen(c, (fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16)))) != 0) && ((F::EftMesh_IsOffScreen(c, (fp.wrapping_add(768)).wrapping_add((((1i32) as u32)).wrapping_mul(16)))) != 0)) as i32)) != 0) && ((F::EftMesh_IsOffScreen(c, (fp.wrapping_add(768)).wrapping_add((((2i32) as u32)).wrapping_mul(16)))) != 0)) as i32)) != 0) {
                        break 'c4;
                    }
                    F::EftMesh_SendTri(c, (fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(768)).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(768)).wrapping_add((((l_i_6) as u32)).wrapping_mul(16)), ((p_v_1).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(32), ((p_v_1).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(48))).wrapping_add(32), ((p_v_1).wrapping_add((((l_i_6) as u32)).wrapping_mul(48))).wrapping_add(32), (fp.wrapping_add(912)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(912)).wrapping_add(((((l_i_6).wrapping_sub(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(912)).wrapping_add((((l_i_6) as u32)).wrapping_mul(16)), p_tex0_2, p_abe_3);
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_DrawNowTriClip", &PROBE_N, &[], true); }
    r_
}

pub fn EftObj_Create(c: &mut Ctx, mut p_res_1: u32, mut p_model_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftObj_Create", &PROBE_N, &[(p_res_1) as u32, (p_model_2) as u32], false); }
    let mut l_id_3: i32 = 0;
    let r_ = 'f: {
    { let _ = F::memset(c, p_res_1, 0i32, 48u32); };
    { let v_ = p_model_2; let a_ = p_res_1; c.m.w32(a_, v_); };
    { let v_ = 32i32; let a_ = (p_res_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    l_id_3 = F::BtlObj_Create(c, 1i32, p_res_1, 1i32);
    if (((((l_id_3) < (0i32)) as i32)) != 0) {
        l_id_3 = (1i32).wrapping_neg();
    }
    break 'f l_id_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftObj_Create", &PROBE_N, &[], true); }
    r_
}

pub fn EftObj_Destroy(c: &mut Ctx, mut p_id_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftObj_Destroy", &PROBE_N, &[(p_id_1) as u32], false); }
    let r_ = 'f: {
    break 'f F::BtlObj_Destroy(c, p_id_1);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftObj_Destroy", &PROBE_N, &[], true); }
    r_
}

pub fn EftObj_SetPos(c: &mut Ctx, mut p_id_1: i32, mut p_pos_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftObj_SetPos", &PROBE_N, &[(p_id_1) as u32, (p_pos_2) as u32], false); }
    let r_ = 'f: {
    { let a0_ = (F::BtlObj_Get(c, p_id_1)).wrapping_add(2384); F::Vec4_Copy(c, a0_, p_pos_2) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftObj_SetPos", &PROBE_N, &[], true); }
    r_
}

pub fn EftObj_UpdateMtx(c: &mut Ctx, mut p_id_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftObj_UpdateMtx", &PROBE_N, &[(p_id_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = F::BtlObj_Get(c, p_id_1); F::BtlObjXf_Update(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftObj_UpdateMtx", &PROBE_N, &[], true); }
    r_
}

pub fn EftObj_PlayAnim(c: &mut Ctx, mut p_id_1: i32, mut p_anim_2: i32, mut p_mode_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftObj_PlayAnim", &PROBE_N, &[(p_id_1) as u32, (p_anim_2) as u32, (p_mode_3) as u32], false); }
    let r_ = 'f: {
    { let a0_ = F::BtlObj_Get(c, p_id_1); F::BtlObjAnim_PlayModel(c, a0_, p_anim_2, p_mode_3) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftObj_PlayAnim", &PROBE_N, &[], true); }
    r_
}

pub fn EftObj_IsAnimPlaying(c: &mut Ctx, mut p_id_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftObj_IsAnimPlaying", &PROBE_N, &[(p_id_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((({ let a0_ = F::BtlObj_Get(c, p_id_1); F::BtlObjAnim_GetMode(c, a0_) }) != (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftObj_IsAnimPlaying", &PROBE_N, &[], true); }
    r_
}

pub fn EftObj_SetRot(c: &mut Ctx, mut p_id_1: i32, mut p_rot_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftObj_SetRot", &PROBE_N, &[(p_id_1) as u32, (p_rot_2) as u32], false); }
    let r_ = 'f: {
    { let a0_ = (F::BtlObj_Get(c, p_id_1)).wrapping_add(2400); F::Vec4_Copy(c, a0_, p_rot_2) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftObj_SetRot", &PROBE_N, &[], true); }
    r_
}

pub fn EftObj_SetMtx(c: &mut Ctx, mut p_id_1: i32, mut p_m_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftObj_SetMtx", &PROBE_N, &[(p_id_1) as u32, (p_m_2) as u32], false); }
    let r_ = 'f: {
    { let a0_ = F::BtlObj_Get(c, p_id_1); F::BtlObjXf_SetMtx(c, a0_, p_m_2) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftObj_SetMtx", &PROBE_N, &[], true); }
    r_
}

pub fn EftObj_SetVisible(c: &mut Ctx, mut p_id_1: i32, mut p_on_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftObj_SetVisible", &PROBE_N, &[(p_id_1) as u32, (p_on_2) as u32], false); }
    let mut l_obj_3: u32 = 0;
    let r_ = 'f: {
    l_obj_3 = F::BtlObj_Get(c, p_id_1);
    if ((p_on_2) != 0) {
        { let r_ = ((2i32) as u32); let a_ = (l_obj_3).wrapping_add(2624); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    } else {
        { let r_ = (((!(2i32))) as u32); let a_ = (l_obj_3).wrapping_add(2624); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftObj_SetVisible", &PROBE_N, &[], true); }
    r_
}

pub fn EftObj_Nop(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftObj_Nop", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftObj_Nop", &PROBE_N, &[], true); }
    r_
}

pub fn EftMesh_QueueTri(c: &mut Ctx, mut p_p0_1: u32, mut p_p1_2: u32, mut p_p2_3: u32, mut p_c0_4: u32, mut p_c1_5: u32, mut p_c2_6: u32, mut p_t0_7: u32, mut p_t1_8: u32, mut p_t2_9: u32, mut p_repeat_10: i32, mut p_flag2_11: i32, mut p_hasTex_12: i32, mut p_layer_13: i32, mut p_z_14: i32, mut p_tex0_15: u64) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_QueueTri", &PROBE_N, &[(p_p0_1) as u32, (p_p1_2) as u32, (p_p2_3) as u32, (p_c0_4) as u32, (p_c1_5) as u32, (p_c2_6) as u32, (p_t0_7) as u32, (p_t1_8) as u32, (p_t2_9) as u32, (p_repeat_10) as u32, (p_flag2_11) as u32, (p_hasTex_12) as u32, (p_layer_13) as u32, (p_z_14) as u32, (p_tex0_15) as u32], false); }
    let mut l_abe_16: i64 = 0;
    let mut l_p_17: u32 = 0;
    let mut l_l_18: i32 = 0;
    let mut l_e_19: u32 = 0;
    let mut l_p_20: u32 = 0;
    let mut l_ctx_21: i32 = 0;
    let mut l_l_22: i32 = 0;
    let mut l_e_23: u32 = 0;
    let mut l_p_24: u32 = 0;
    let mut l_l_25: i32 = 0;
    let mut l_e_26: u32 = 0;
    let r_ = 'f: {
    l_abe_16 = ((1i32) as i64);
    if (((((((((((rt::flt(F32(c.m.r32((p_c0_4).wrapping_add(12))), F32(0x3dcccccc)) as i32)) != 0) && (((rt::flt(F32(c.m.r32((p_c1_5).wrapping_add(12))), F32(0x3dcccccc)) as i32)) != 0)) as i32)) != 0) && (((rt::flt(F32(c.m.r32((p_c2_6).wrapping_add(12))), F32(0x3dcccccc)) as i32)) != 0)) as i32)) != 0) {
        break 'f;
    }
    if (((((p_layer_13) < (0i32)) as i32)) != 0) {
        p_layer_13 = 0i32;
        l_abe_16 = ((0i32) as i64);
    }
    if ((p_repeat_10) != 0) {
        l_l_18 = p_layer_13;
        if (((((l_l_18) >= (2i32)) as i32)) != 0) {
            { let r_ = 2i32; let n_ = (l_l_18).wrapping_sub(r_); l_l_18 = n_; };
        }
        if (((((p_z_14) < (0i32)) as i32)) != 0) {
            l_e_19 = ((c.m.r32(sym::gOtZ)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_18) as u32)).wrapping_mul(8));
        } else {
            if (((((p_z_14) >= (4096i32)) as i32)) != 0) {
                l_e_19 = ((c.m.r32(sym::gOtZ)).wrapping_add((((4095i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_18) as u32)).wrapping_mul(8));
            } else {
                l_e_19 = ((c.m.r32(sym::gOtZ)).wrapping_add((((p_z_14) as u32)).wrapping_mul(16))).wrapping_add((((l_l_18) as u32)).wrapping_mul(8));
            }
        }
        l_p_17 = c.m.r32(sym::gOtCur);
        { let v_ = (l_p_17).wrapping_add((((1i32) as u32)).wrapping_mul(64)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
        { let v_ = ((536870915i32) as u32); let a_ = l_p_17; c.m.w32(a_, v_); };
        { let v_ = ((268435456i32) as u32); let a_ = (l_p_17).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = ((1342177283i32) as u32); let a_ = (l_p_17).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = ((1152921504606879746i64) as u64); let a_ = (l_p_17).wrapping_add(16); c.m.w64(a_, v_); };
        { let v_ = ((14i32) as u64); let a_ = (l_p_17).wrapping_add(24); c.m.w64(a_, v_); };
        { let v_ = ((8i32) as u64); let a_ = (l_p_17).wrapping_add(40); c.m.w64(a_, v_); };
        { let v_ = ((9i32) as u64); let a_ = (l_p_17).wrapping_add(56); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (l_p_17).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u64); let a_ = (l_p_17).wrapping_add(32); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u64); let a_ = (l_p_17).wrapping_add(48); c.m.w64(a_, v_); };
        { let v_ = l_p_17; let a_ = (c.m.r32((l_e_19).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = l_p_17; let a_ = (l_e_19).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if ((p_hasTex_12) != 0) {
        l_p_20 = c.m.r32(sym::gOtCur);
        { let v_ = (l_p_20).wrapping_add((((1i32) as u32)).wrapping_mul(128)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
        l_ctx_21 = (((p_layer_13) >= (2i32)) as i32);
        { let v_ = (((((((l_abe_16).wrapping_shl((6i32) as u32)) | ((((l_ctx_21) as i64)).wrapping_shl((9i32) as u32)))) | (((27i32) as i64)))) as u64); let a_ = (l_p_20).wrapping_add(32); c.m.w64(a_, v_); };
        { let v_ = ((536870919i32) as u32); let a_ = l_p_20; c.m.w32(a_, v_); };
        { let v_ = ((268435456i32) as u32); let a_ = (l_p_20).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = ((1342177287i32) as u32); let a_ = (l_p_20).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = 14123288431433908225u64; let a_ = (l_p_20).wrapping_add(16); c.m.w64(a_, v_); };
        { let v_ = ((((((l_ctx_21).wrapping_shl((4i32) as u32)) as i64)).wrapping_add(268423679713632i64)) as u64); let a_ = (l_p_20).wrapping_add(24); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (l_p_20).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u64); let a_ = (l_p_20).wrapping_add(120); c.m.w64(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32(p_c0_4)))) as u8); let a_ = ((l_p_20).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_c0_4).wrapping_add(4))))) as u8); let a_ = (((l_p_20).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_c0_4).wrapping_add(8))))) as u8); let a_ = (((l_p_20).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_c0_4).wrapping_add(12))))) as u8); let a_ = (((l_p_20).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = F32(c.m.r32((p_t0_7).wrapping_add(8))); let a_ = (((l_p_20).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = ((rt::f2i(F32(c.m.r32(p_c1_5)))) as u8); let a_ = ((l_p_20).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_c1_5).wrapping_add(4))))) as u8); let a_ = (((l_p_20).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_c1_5).wrapping_add(8))))) as u8); let a_ = (((l_p_20).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_c1_5).wrapping_add(12))))) as u8); let a_ = (((l_p_20).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = F32(c.m.r32((p_t1_8).wrapping_add(8))); let a_ = (((l_p_20).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = ((rt::f2i(F32(c.m.r32(p_c2_6)))) as u8); let a_ = ((l_p_20).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_c2_6).wrapping_add(4))))) as u8); let a_ = (((l_p_20).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_c2_6).wrapping_add(8))))) as u8); let a_ = (((l_p_20).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_c2_6).wrapping_add(12))))) as u8); let a_ = (((l_p_20).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = F32(c.m.r32((p_t2_9).wrapping_add(8))); let a_ = (((l_p_20).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_tex0_15; let a_ = (l_p_20).wrapping_add(40); c.m.w64(a_, v_); };
        { let v_ = F32(c.m.r32(p_t0_7)); let a_ = (((l_p_20).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((p_t0_7).wrapping_add(4))); let a_ = ((((l_p_20).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(p_t1_8)); let a_ = (((l_p_20).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((p_t1_8).wrapping_add(4))); let a_ = ((((l_p_20).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(p_t2_9)); let a_ = (((l_p_20).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((p_t2_9).wrapping_add(4))); let a_ = ((((l_p_20).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = (((c.m.r32(p_p0_1) as i32)) as u64); let a_ = (((l_p_20).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32((p_p0_1).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_20).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32((p_p0_1).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_20).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_20).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32(p_p1_2) as i32)) as u64); let a_ = (((l_p_20).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32((p_p1_2).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_20).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32((p_p1_2).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_20).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_20).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32(p_p2_3) as i32)) as u64); let a_ = (((l_p_20).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32((p_p2_3).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_20).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32((p_p2_3).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_20).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_20).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        l_l_22 = p_layer_13;
        if (((((l_l_22) >= (2i32)) as i32)) != 0) {
            { let r_ = 2i32; let n_ = (l_l_22).wrapping_sub(r_); l_l_22 = n_; };
        }
        if (((((p_z_14) < (0i32)) as i32)) != 0) {
            l_e_23 = ((c.m.r32(sym::gOtZ)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_22) as u32)).wrapping_mul(8));
        } else {
            if (((((p_z_14) >= (4096i32)) as i32)) != 0) {
                l_e_23 = ((c.m.r32(sym::gOtZ)).wrapping_add((((4095i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_22) as u32)).wrapping_mul(8));
            } else {
                l_e_23 = ((c.m.r32(sym::gOtZ)).wrapping_add((((p_z_14) as u32)).wrapping_mul(16))).wrapping_add((((l_l_22) as u32)).wrapping_mul(8));
            }
        }
        { let v_ = l_p_20; let a_ = (c.m.r32((l_e_23).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = l_p_20; let a_ = (l_e_23).wrapping_add(4); c.m.w32(a_, v_); };
    } else {
        l_p_24 = c.m.r32(sym::gOtCur);
        { let v_ = (l_p_24).wrapping_add((((1i32) as u32)).wrapping_mul(96)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
        { let v_ = (((((l_abe_16).wrapping_shl((6i32) as u32)) | (((11i32) as i64)))) as u64); let a_ = (l_p_24).wrapping_add(32); c.m.w64(a_, v_); };
        { let v_ = ((536870917i32) as u32); let a_ = l_p_24; c.m.w32(a_, v_); };
        { let v_ = ((268435456i32) as u32); let a_ = (l_p_24).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = ((1342177285i32) as u32); let a_ = (l_p_24).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = 9511602413006520321u64; let a_ = (l_p_24).wrapping_add(16); c.m.w64(a_, v_); };
        { let v_ = ((4094956560u32) as u64); let a_ = (l_p_24).wrapping_add(24); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (l_p_24).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32(p_c0_4)))) as u8); let a_ = ((l_p_24).wrapping_add(40)).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_c0_4).wrapping_add(4))))) as u8); let a_ = (((l_p_24).wrapping_add(40)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_c0_4).wrapping_add(8))))) as u8); let a_ = (((l_p_24).wrapping_add(40)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(2); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_c0_4).wrapping_add(12))))) as u8); let a_ = (((l_p_24).wrapping_add(40)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = F32(0x3f800000); let a_ = (((l_p_24).wrapping_add(40)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = ((rt::f2i(F32(c.m.r32(p_c1_5)))) as u8); let a_ = ((l_p_24).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_c1_5).wrapping_add(4))))) as u8); let a_ = (((l_p_24).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_c1_5).wrapping_add(8))))) as u8); let a_ = (((l_p_24).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(2); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_c1_5).wrapping_add(12))))) as u8); let a_ = (((l_p_24).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = F32(0x3f800000); let a_ = (((l_p_24).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = ((rt::f2i(F32(c.m.r32(p_c2_6)))) as u8); let a_ = ((l_p_24).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(16)); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_c2_6).wrapping_add(4))))) as u8); let a_ = (((l_p_24).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_c2_6).wrapping_add(8))))) as u8); let a_ = (((l_p_24).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(2); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_c2_6).wrapping_add(12))))) as u8); let a_ = (((l_p_24).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = F32(0x3f800000); let a_ = (((l_p_24).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = (((c.m.r32(p_p0_1) as i32)) as u64); let a_ = (((l_p_24).wrapping_add(40)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32((p_p0_1).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_24).wrapping_add(40)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32((p_p0_1).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_24).wrapping_add(40)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_24).wrapping_add(40)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32(p_p1_2) as i32)) as u64); let a_ = (((l_p_24).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32((p_p1_2).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_24).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32((p_p1_2).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_24).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_24).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32(p_p2_3) as i32)) as u64); let a_ = (((l_p_24).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32((p_p2_3).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_24).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32((p_p2_3).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_24).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_24).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        l_l_25 = p_layer_13;
        if (((((l_l_25) >= (2i32)) as i32)) != 0) {
            { let r_ = 2i32; let n_ = (l_l_25).wrapping_sub(r_); l_l_25 = n_; };
        }
        if (((((p_z_14) < (0i32)) as i32)) != 0) {
            l_e_26 = ((c.m.r32(sym::gOtZ)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_25) as u32)).wrapping_mul(8));
        } else {
            if (((((p_z_14) >= (4096i32)) as i32)) != 0) {
                l_e_26 = ((c.m.r32(sym::gOtZ)).wrapping_add((((4095i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_25) as u32)).wrapping_mul(8));
            } else {
                l_e_26 = ((c.m.r32(sym::gOtZ)).wrapping_add((((p_z_14) as u32)).wrapping_mul(16))).wrapping_add((((l_l_25) as u32)).wrapping_mul(8));
            }
        }
        { let v_ = l_p_24; let a_ = (c.m.r32((l_e_26).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = l_p_24; let a_ = (l_e_26).wrapping_add(4); c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_QueueTri", &PROBE_N, &[], true); }
    r_
}

pub fn EftMesh_SendTri(c: &mut Ctx, mut p_p0_1: u32, mut p_p1_2: u32, mut p_p2_3: u32, mut p_c0_4: u32, mut p_c1_5: u32, mut p_c2_6: u32, mut p_t0_7: u32, mut p_t1_8: u32, mut p_t2_9: u32, mut p_tex0_10: u64, mut p_abe_11: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_SendTri", &PROBE_N, &[(p_p0_1) as u32, (p_p1_2) as u32, (p_p2_3) as u32, (p_c0_4) as u32, (p_c1_5) as u32, (p_c2_6) as u32, (p_t0_7) as u32, (p_t1_8) as u32, (p_t2_9) as u32, (p_tex0_10) as u32, (p_abe_11) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(128) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 128);
    let mut l_dst_12: u32 = 0;
    let mut l_a_13: i8 = 0;
    let r_ = 'f: {
        if (((((((((((rt::flt(F32(c.m.r32((p_c0_4).wrapping_add(12))), F32(0x3dcccccc)) as i32)) != 0) && (((rt::flt(F32(c.m.r32((p_c1_5).wrapping_add(12))), F32(0x3dcccccc)) as i32)) != 0)) as i32)) != 0) && (((rt::flt(F32(c.m.r32((p_c2_6).wrapping_add(12))), F32(0x3dcccccc)) as i32)) != 0)) as i32)) != 0) {
            break 'f;
        }
        l_dst_12 = F::Dma_BeginDirect(c);
        l_a_13 = ((p_abe_11) as i8);
        { let v_ = (((((((l_a_13) as i64)).wrapping_shl((6i32) as u32)) | (((27i32) as i64)))) as u64); let a_ = (fp).wrapping_add(32); c.m.w64(a_, v_); };
        { let v_ = ((536870919i32) as u32); let a_ = fp; c.m.w32(a_, v_); };
        { let v_ = ((268435456i32) as u32); let a_ = (fp).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = ((1342177287i32) as u32); let a_ = (fp).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = 14123288431433908225u64; let a_ = (fp).wrapping_add(16); c.m.w64(a_, v_); };
        { let v_ = ((268423679713632i64) as u64); let a_ = (fp).wrapping_add(24); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (fp).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u64); let a_ = (fp).wrapping_add(120); c.m.w64(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32(p_c0_4)))) as u8); let a_ = ((fp).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_c0_4).wrapping_add(4))))) as u8); let a_ = (((fp).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_c0_4).wrapping_add(8))))) as u8); let a_ = (((fp).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_c0_4).wrapping_add(12))))) as u8); let a_ = (((fp).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = F32(c.m.r32((p_t0_7).wrapping_add(8))); let a_ = (((fp).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = ((rt::f2i(F32(c.m.r32(p_c1_5)))) as u8); let a_ = ((fp).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_c1_5).wrapping_add(4))))) as u8); let a_ = (((fp).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_c1_5).wrapping_add(8))))) as u8); let a_ = (((fp).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_c1_5).wrapping_add(12))))) as u8); let a_ = (((fp).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = F32(c.m.r32((p_t1_8).wrapping_add(8))); let a_ = (((fp).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = ((rt::f2i(F32(c.m.r32(p_c2_6)))) as u8); let a_ = ((fp).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_c2_6).wrapping_add(4))))) as u8); let a_ = (((fp).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_c2_6).wrapping_add(8))))) as u8); let a_ = (((fp).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_c2_6).wrapping_add(12))))) as u8); let a_ = (((fp).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = F32(c.m.r32((p_t2_9).wrapping_add(8))); let a_ = (((fp).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_tex0_10; let a_ = (fp).wrapping_add(40); c.m.w64(a_, v_); };
        { let v_ = F32(c.m.r32(p_t0_7)); let a_ = (((fp).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((p_t0_7).wrapping_add(4))); let a_ = ((((fp).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(p_t1_8)); let a_ = (((fp).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((p_t1_8).wrapping_add(4))); let a_ = ((((fp).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(p_t2_9)); let a_ = (((fp).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((p_t2_9).wrapping_add(4))); let a_ = ((((fp).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = (((c.m.r32(p_p0_1) as i32)) as u64); let a_ = (((fp).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32((p_p0_1).wrapping_add(4)) as i32)) as u64); let a_ = ((((fp).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32((p_p0_1).wrapping_add(8)) as i32)) as u64); let a_ = ((((fp).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((fp).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32(p_p1_2) as i32)) as u64); let a_ = (((fp).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32((p_p1_2).wrapping_add(4)) as i32)) as u64); let a_ = ((((fp).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32((p_p1_2).wrapping_add(8)) as i32)) as u64); let a_ = ((((fp).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((fp).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32(p_p2_3) as i32)) as u64); let a_ = (((fp).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32((p_p2_3).wrapping_add(4)) as i32)) as u64); let a_ = ((((fp).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32((p_p2_3).wrapping_add(8)) as i32)) as u64); let a_ = ((((fp).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((fp).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let _ = { let a2_ = (((((((c.m.r32(fp)) as u16)) as i32)).wrapping_shl((4i32) as u32)) as u32); F::memcpy(c, l_dst_12, (fp).wrapping_add(16), a2_) }; };
        { let a0_ = (l_dst_12).wrapping_add((((((((c.m.r32(fp)) as u16)) as i32)).wrapping_shl((4i32) as u32)) as u32)); F::Dma_EndDirect(c, a0_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftMesh_SendTri", &PROBE_N, &[], true); }
    r_
}
