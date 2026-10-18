#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn StgPanBlur_Init(c: &mut Ctx, mut p_cbp_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgPanBlur_Init", &PROBE_N, &[(p_cbp_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, ((1792u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gStgPanBlur; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gStgPanBlur); F::memset(c, a0_, 0i32, 1792u32) }; };
    { let a0_ = c.m.r32(sym::gStgPanBlur); F::GfxClut_InitPacket(c, a0_, ((p_cbp_1) as u16)) };
    { let a0_ = { let a_ = (c.m.r32(sym::gStgPanBlur)).wrapping_add(1168); c.m.r32(a_) }; F::StgPanBlur_BuildClut(c, a0_) };
    { let v_ = F32(0x42ca0000); let a_ = (((c.m.r32(sym::gStgPanBlur)).wrapping_add(1216)).wrapping_add((((0i32) as u32)).wrapping_mul(272))).wrapping_add(128); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x42ca0000); let a_ = (((c.m.r32(sym::gStgPanBlur)).wrapping_add(1216)).wrapping_add((((1i32) as u32)).wrapping_mul(272))).wrapping_add(128); c.m.w32(a_, (v_).0); };
    { let v_ = 0i32; let a_ = (((c.m.r32(sym::gStgPanBlur)).wrapping_add(1216)).wrapping_add((((0i32) as u32)).wrapping_mul(272))).wrapping_add(144); c.m.w32(a_, (v_) as u32); };
    { let v_ = 1i32; let a_ = (((c.m.r32(sym::gStgPanBlur)).wrapping_add(1216)).wrapping_add((((1i32) as u32)).wrapping_mul(272))).wrapping_add(144); c.m.w32(a_, (v_) as u32); };
    if ((F::Battle_IsSplitScreen(c)) != 0) {
        { let v_ = 1i32; let a_ = (c.m.r32(sym::gStgPanBlur)).wrapping_add(1760); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (((c.m.r32(sym::gStgPanBlur)).wrapping_add(1216)).wrapping_add((((0i32) as u32)).wrapping_mul(272))).wrapping_add(148); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (((c.m.r32(sym::gStgPanBlur)).wrapping_add(1216)).wrapping_add((((0i32) as u32)).wrapping_mul(272))).wrapping_add(152); c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = (((c.m.r32(sym::gStgPanBlur)).wrapping_add(1216)).wrapping_add((((0i32) as u32)).wrapping_mul(272))).wrapping_add(156); c.m.w32(a_, (v_) as u32); };
        { let v_ = 448i32; let a_ = (((c.m.r32(sym::gStgPanBlur)).wrapping_add(1216)).wrapping_add((((0i32) as u32)).wrapping_mul(272))).wrapping_add(160); c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = (((c.m.r32(sym::gStgPanBlur)).wrapping_add(1216)).wrapping_add((((1i32) as u32)).wrapping_mul(272))).wrapping_add(148); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (((c.m.r32(sym::gStgPanBlur)).wrapping_add(1216)).wrapping_add((((1i32) as u32)).wrapping_mul(272))).wrapping_add(152); c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = (((c.m.r32(sym::gStgPanBlur)).wrapping_add(1216)).wrapping_add((((1i32) as u32)).wrapping_mul(272))).wrapping_add(156); c.m.w32(a_, (v_) as u32); };
        { let v_ = 448i32; let a_ = (((c.m.r32(sym::gStgPanBlur)).wrapping_add(1216)).wrapping_add((((1i32) as u32)).wrapping_mul(272))).wrapping_add(160); c.m.w32(a_, (v_) as u32); };
    } else {
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gStgPanBlur)).wrapping_add(1760); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (((c.m.r32(sym::gStgPanBlur)).wrapping_add(1216)).wrapping_add((((0i32) as u32)).wrapping_mul(272))).wrapping_add(148); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (((c.m.r32(sym::gStgPanBlur)).wrapping_add(1216)).wrapping_add((((0i32) as u32)).wrapping_mul(272))).wrapping_add(152); c.m.w32(a_, (v_) as u32); };
        { let v_ = 512i32; let a_ = (((c.m.r32(sym::gStgPanBlur)).wrapping_add(1216)).wrapping_add((((0i32) as u32)).wrapping_mul(272))).wrapping_add(156); c.m.w32(a_, (v_) as u32); };
        { let v_ = 448i32; let a_ = (((c.m.r32(sym::gStgPanBlur)).wrapping_add(1216)).wrapping_add((((0i32) as u32)).wrapping_mul(272))).wrapping_add(160); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (((c.m.r32(sym::gStgPanBlur)).wrapping_add(1216)).wrapping_add((((1i32) as u32)).wrapping_mul(272))).wrapping_add(148); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (((c.m.r32(sym::gStgPanBlur)).wrapping_add(1216)).wrapping_add((((1i32) as u32)).wrapping_mul(272))).wrapping_add(152); c.m.w32(a_, (v_) as u32); };
        { let v_ = 512i32; let a_ = (((c.m.r32(sym::gStgPanBlur)).wrapping_add(1216)).wrapping_add((((1i32) as u32)).wrapping_mul(272))).wrapping_add(156); c.m.w32(a_, (v_) as u32); };
        { let v_ = 448i32; let a_ = (((c.m.r32(sym::gStgPanBlur)).wrapping_add(1216)).wrapping_add((((1i32) as u32)).wrapping_mul(272))).wrapping_add(160); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgPanBlur_Init", &PROBE_N, &[], true); }
    r_
}

pub fn StgPanBlur_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgPanBlur_Draw", &PROBE_N, &[], false); }
    let mut l_on0_1: i32 = 0;
    let mut l_on1_2: i32 = 0;
    let r_ = 'f: {
    l_on0_1 = 1i32;
    l_on1_2 = 1i32;
    F::Gfx_AddDefaultEnv(c);
    F::Dma_AddZbuf(c, 224i32, 1i32);
    if ((F::Battle_IsSplitScreen(c)) != 0) {
        if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
            l_on0_1 = { let a0_ = ((c.m.r32(sym::gStgPanBlur)).wrapping_add(1216)).wrapping_add((((0i32) as u32)).wrapping_mul(272)); F::StgPanBlur_UpdateView(c, a0_) };
            l_on1_2 = { let a0_ = ((c.m.r32(sym::gStgPanBlur)).wrapping_add(1216)).wrapping_add((((1i32) as u32)).wrapping_mul(272)); F::StgPanBlur_UpdateView(c, a0_) };
        }
        if (((((((((l_on0_1) != (0i32)) as i32)) != 0) || (((((l_on1_2) != (0i32)) as i32)) != 0)) as i32)) != 0) {
            { let a0_ = (c.m.r32(sym::gStgPanBlur)).wrapping_add(1172); F::Dma_AddData(c, a0_, 16i32) };
            F::Dma_AddTexFlush(c);
            { let a0_ = (if (((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0) { 0i32 } else { 112i32 }); F::Dma_AddFrame(c, a0_, 8i32, 16777215i32) };
            { let a2_ = (({ let a_ = (c.m.r32(sym::gStgPanBlur)).wrapping_add(1204); c.m.r16(a_) }) as i32); F::GfxPost_DrawDepthClut(c, 0i32, 7168i32, a2_, ((68i32) as u64)) };
            F::StgPanBlur_CopyScreen(c);
            if (((((l_on0_1) != (0i32)) as i32)) != 0) {
                { let a0_ = ((c.m.r32(sym::gStgPanBlur)).wrapping_add(1216)).wrapping_add((((0i32) as u32)).wrapping_mul(272)); let a1_ = { let a_ = (c.m.r32(sym::gStgPanBlur)).wrapping_add(1760); (c.m.r32(a_) as i32) }; F::StgPanBlur_DrawView(c, a0_, a1_) };
            }
            if (((((l_on1_2) != (0i32)) as i32)) != 0) {
                { let a0_ = ((c.m.r32(sym::gStgPanBlur)).wrapping_add(1216)).wrapping_add((((1i32) as u32)).wrapping_mul(272)); let a1_ = { let a_ = (c.m.r32(sym::gStgPanBlur)).wrapping_add(1760); (c.m.r32(a_) as i32) }; F::StgPanBlur_DrawView(c, a0_, a1_) };
            }
            F::StgPanBlur_RestoreEnv(c);
        }
    } else {
        if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
            l_on0_1 = { let a0_ = ((c.m.r32(sym::gStgPanBlur)).wrapping_add(1216)).wrapping_add((((0i32) as u32)).wrapping_mul(272)); F::StgPanBlur_UpdateView(c, a0_) };
        }
        if (((((l_on0_1) != (0i32)) as i32)) != 0) {
            { let a0_ = (c.m.r32(sym::gStgPanBlur)).wrapping_add(1172); F::Dma_AddData(c, a0_, 16i32) };
            F::Dma_AddTexFlush(c);
            { let a0_ = (if (((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0) { 0i32 } else { 112i32 }); F::Dma_AddFrame(c, a0_, 8i32, 16777215i32) };
            { let a2_ = (({ let a_ = (c.m.r32(sym::gStgPanBlur)).wrapping_add(1204); c.m.r16(a_) }) as i32); F::GfxPost_DrawDepthClut(c, 0i32, 7168i32, a2_, ((68i32) as u64)) };
            F::StgPanBlur_CopyScreen(c);
            { let a0_ = ((c.m.r32(sym::gStgPanBlur)).wrapping_add(1216)).wrapping_add((((0i32) as u32)).wrapping_mul(272)); let a1_ = { let a_ = (c.m.r32(sym::gStgPanBlur)).wrapping_add(1760); (c.m.r32(a_) as i32) }; F::StgPanBlur_DrawView(c, a0_, a1_) };
            F::StgPanBlur_RestoreEnv(c);
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgPanBlur_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn StgPanBlur_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgPanBlur_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32(sym::gStgPanBlur); F::Heap_Free(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgPanBlur_Term", &PROBE_N, &[], true); }
    r_
}

pub fn StgPanBlur_BuildClut(c: &mut Ctx, mut p_clut_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgPanBlur_BuildClut", &PROBE_N, &[(p_clut_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let mut l_e_3: u32 = 0;
    let mut l_a_4: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (256i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_e_3 = (p_clut_1).wrapping_add((((((((((l_i_2) & (231i32))) | ((((l_i_2) & (8i32))).wrapping_shl((1i32) as u32)))) | ((((l_i_2) & (16i32))).wrapping_shr((1i32) as u32)))).wrapping_mul(4i32)) as u32));
                { let v_ = ((128i32) as u8); let a_ = (l_e_3).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
                { let v_ = ((128i32) as u8); let a_ = (l_e_3).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
                { let v_ = ((128i32) as u8); let a_ = (l_e_3).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
                l_a_4 = (128i32).wrapping_sub((l_i_2).wrapping_shr((1i32) as u32));
                if (((((l_a_4) < (0i32)) as i32)) != 0) {
                    { let v_ = ((0i32) as u8); let a_ = (l_e_3).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
                } else {
                    { let v_ = ((l_a_4) as u8); let a_ = (l_e_3).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgPanBlur_BuildClut", &PROBE_N, &[], true); }
    r_
}

pub fn StgPanBlur_UpdateView(c: &mut Ctx, mut p_view_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgPanBlur_UpdateView", &PROBE_N, &[(p_view_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_sp_2: u32 = 0;
    let mut l_cam_3: u32 = 0;
    let mut l_dist_4: F32 = F32(0);
    let mut l_i_5: i32 = 0;
    let mut l_a_6: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(12), (v_).0); }
        l_sp_2 = fp;
        rt::memset(c, fp.wrapping_add(16), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(32), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(32).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(32).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(32).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(32).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(48), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(48).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(48).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(48).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(48).wrapping_add(12), (v_).0); }
        l_cam_3 = ((c.m.r32(sym::gBtlCam)).wrapping_add(1824)).wrapping_add(((((c.m.r32((p_view_1).wrapping_add(144)) as i32)) as u32)).wrapping_mul(656));
        l_dist_4 = F32(0x00000000);
        F::Mtx_Copy(c, (p_view_1).wrapping_add(64), p_view_1);
        F::Mtx_InverseRT(c, p_view_1, (l_cam_3).wrapping_add(64));
        { let v_ = l_dist_4; let a_ = ((p_view_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = l_dist_4; let a_ = (((p_view_1).wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        F::Vec3_Normalize(c, fp.wrapping_add(32), (p_view_1).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
        F::Vec3_Normalize(c, fp.wrapping_add(48), ((p_view_1).wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
        F::Vec3_Cross(c, fp, fp.wrapping_add(48), fp.wrapping_add(32));
        F::Vec4_Sub(c, fp.wrapping_add(16), ((p_view_1).wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (p_view_1).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
        'l1: loop {
            'c2: {
                l_dist_4 = F::Vec3_Length(c, fp.wrapping_add(16));
            }
            if !((0i32) != 0) { break 'l1; }
        }
        if (((rt::fge(l_dist_4, F32(0x3f800000)) as i32)) != 0) {
            if (((rt::flt(l_dist_4, F32(0x40200000)) as i32)) != 0) {
                { let r_ = F32(0x3dcccccc); let a_ = (p_view_1).wrapping_add(132); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            }
            if (((((((rt::fge(l_dist_4, F32(0x40200000)) as i32)) != 0) && (((rt::flt(l_dist_4, F32(0x40800000)) as i32)) != 0)) as i32)) != 0) {
                { let r_ = F32(0x3e199999); let a_ = (p_view_1).wrapping_add(132); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            }
            if (((rt::fgt(l_dist_4, F32(0x40800000)) as i32)) != 0) {
                { let r_ = F32(0x3dcccccc); let a_ = (p_view_1).wrapping_add(132); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            }
        } else {
            { let r_ = F32(0x3e199999); let a_ = (p_view_1).wrapping_add(132); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
        if (((rt::fgt(F32(c.m.r32((p_view_1).wrapping_add(132))), F32(0x3f800000)) as i32)) != 0) {
            { let v_ = F32(0x3f800000); let a_ = (p_view_1).wrapping_add(132); c.m.w32(a_, (v_).0); };
        }
        if (((rt::flt(F32(c.m.r32((p_view_1).wrapping_add(132))), F32(0x00000000)) as i32)) != 0) {
            { let v_ = F32(0x00000000); let a_ = (p_view_1).wrapping_add(132); c.m.w32(a_, (v_).0); };
            break 'f 0i32;
        }
        { let v_ = rt::fmul(F32(c.m.r32((l_sp_2).wrapping_add(4))), F32(0x42c80000)); let a_ = (p_view_1).wrapping_add(136); c.m.w32(a_, (v_).0); };
        if (((rt::fgt(F32(c.m.r32((p_view_1).wrapping_add(136))), F32(0x3f800000)) as i32)) != 0) {
            { let v_ = F32(0x3f800000); let a_ = (p_view_1).wrapping_add(136); c.m.w32(a_, (v_).0); };
        }
        if (((rt::flt(F32(c.m.r32((p_view_1).wrapping_add(136))), F32(0xbf800000)) as i32)) != 0) {
            { let v_ = F32(0xbf800000); let a_ = (p_view_1).wrapping_add(136); c.m.w32(a_, (v_).0); };
        }
        {
            l_i_5 = 0i32;
            'l3: loop {
                if !(((((l_i_5) < (2i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    l_a_6 = rt::f2i(rt::fsub(rt::fmul(F32(c.m.r32((p_view_1).wrapping_add(128))), F32(c.m.r32((p_view_1).wrapping_add(132)))), rt::i2f((l_i_5).wrapping_mul(32i32))));
                    if (((((l_a_6) < (0i32)) as i32)) != 0) {
                        l_a_6 = 0i32;
                    }
                    { let v_ = rt::fmul(rt::i2f((l_i_5).wrapping_add(1i32)), F32(c.m.r32((p_view_1).wrapping_add(136)))); let a_ = ((p_view_1).wrapping_add(176)).wrapping_add((((((l_i_5).wrapping_mul(12i32)).wrapping_add(0i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x00000000); let a_ = ((p_view_1).wrapping_add(176)).wrapping_add((((((l_i_5).wrapping_mul(12i32)).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x00000000); let a_ = ((p_view_1).wrapping_add(176)).wrapping_add((((((l_i_5).wrapping_mul(12i32)).wrapping_add(2i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = ((p_view_1).wrapping_add(176)).wrapping_add((((((l_i_5).wrapping_mul(12i32)).wrapping_add(3i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x43000000); let a_ = ((p_view_1).wrapping_add(176)).wrapping_add((((((l_i_5).wrapping_mul(12i32)).wrapping_add(4i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x43000000); let a_ = ((p_view_1).wrapping_add(176)).wrapping_add((((((l_i_5).wrapping_mul(12i32)).wrapping_add(5i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x43000000); let a_ = ((p_view_1).wrapping_add(176)).wrapping_add((((((l_i_5).wrapping_mul(12i32)).wrapping_add(6i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::i2f(l_a_6); let a_ = ((p_view_1).wrapping_add(176)).wrapping_add((((((l_i_5).wrapping_mul(12i32)).wrapping_add(7i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgPanBlur_UpdateView", &PROBE_N, &[], true); }
    r_
}

pub fn StgPanBlur_CopyScreen(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgPanBlur_CopyScreen", &PROBE_N, &[], false); }
    let mut l_p_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_x0_3: i32 = 0;
    let r_ = 'f: {
    l_p_1 = F::Dma_BeginDirect(c);
    { let v_ = ((((((((9i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((1i32) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((70i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((0i32) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((74i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((262480i32) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((76i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((78i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((((30720i32) as u64)) | ((((30720i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((24i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((((((((0i32) as u64)) | ((((255i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) | ((((255i32) as u64)).wrapping_shl((48i32) as u32))); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((64i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((((((((((((2i32) as u64)) | ((((2i32) as u64)).wrapping_shl((2i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((4i32) as u32)))) | ((((511i32) as u64)).wrapping_shl((14i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((447i32) as u64)).wrapping_shl((34i32) as u32))); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((8i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((96i32) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((20i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((196608i32) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((71i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((((((((6i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((0i32) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((63i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { 2305843036661353984i64 } else { 2305843036661350400i64 })) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((6i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((68i32) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((66i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((196608i32) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((71i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((278i32) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((0i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((4575657223564329088i64) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((1i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((((((((16i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((4i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((21331i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    l_x0_3 = 0i32;
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (16i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = ((((l_i_2).wrapping_mul(32i32)).wrapping_shl((4i32) as u32)) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let v_ = ((((((l_x0_3).wrapping_add(1920i32)).wrapping_shl((4i32) as u32)) | ((30720i32).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
                { let v_ = (((((((l_i_2).wrapping_add(1i32)).wrapping_mul(32i32)).wrapping_shl((4i32) as u32)) | ((7168i32).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let v_ = ((((((((l_x0_3).wrapping_add(1936i32)).wrapping_shl((4i32) as u32)) as i64)) as u64)) | ((((34816i32) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
                { let r_ = 16i32; let n_ = (l_x0_3).wrapping_add(r_); l_x0_3 = n_; };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    F::Dma_EndDirect(c, l_p_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgPanBlur_CopyScreen", &PROBE_N, &[], true); }
    r_
}

pub fn StgPanBlur_DrawView(c: &mut Ctx, mut p_view_1: u32, mut p_split_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgPanBlur_DrawView", &PROBE_N, &[(p_view_1) as u32, (p_split_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_p_3: u32 = 0;
    let mut l_shift_4: i32 = 0;
    let mut l_srcX_5: i32 = 0;
    let mut l_n_6: i32 = 0;
    let mut l_step_7: i32 = 0;
    let mut l_i_8: i32 = 0;
    let mut l_size_9: i32 = 0;
    let mut l_n_10: i32 = 0;
    let mut l_step_11: i32 = 0;
    let mut l_layer_12: u32 = 0;
    let mut l_k_13: i32 = 0;
    let mut l_x_14: i32 = 0;
    let mut l_y_15: i32 = 0;
    let mut l_r_16: i32 = 0;
    let mut l_i_17: i32 = 0;
    let mut l_n_18: i32 = 0;
    let mut l_x_19: i32 = 0;
    let mut l_step_20: i32 = 0;
    let mut l_u_21: i32 = 0;
    let mut l_u1_22: i32 = 0;
    let mut l_i_23: i32 = 0;
    let r_ = 'f: {
        l_shift_4 = 0i32;
        l_srcX_5 = 0i32;
        if (((((p_split_2) != (0i32)) as i32)) != 0) {
            l_srcX_5 = 128i32;
            l_shift_4 = 1i32;
            if ((((((c.m.r32((p_view_1).wrapping_add(144)) as i32)) == (0i32)) as i32)) != 0) {
                l_srcX_5 = 0i32;
            }
        }
        l_p_3 = F::Dma_BeginDirect(c);
        { let v_ = ((((((((9i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((14i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
        { let v_ = ((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((78i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
        { let v_ = ((131440i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((76i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
        { let v_ = ((((31744i32) as u64)) | ((((31744i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((24i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
        { let v_ = ((((((((0i32) as u64)) | ((((127i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) | ((((127i32) as u64)).wrapping_shl((48i32) as u32))); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((64i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
        { let v_ = ((((((((((((2i32) as u64)) | ((((2i32) as u64)).wrapping_shl((2i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((4i32) as u32)))) | ((((255i32) as u64)).wrapping_shl((14i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((255i32) as u64)).wrapping_shl((34i32) as u32))); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((8i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
        { let v_ = ((0i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((63i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
        { let v_ = (((((((F::Tex_Log2Size(c, 256i32)) as u64)).wrapping_shl((26i32) as u32)) | ((((F::Tex_Log2Size(c, 256i32)) as u64)).wrapping_shl((30i32) as u32)))) | (((17179945472i64) as u64))); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((6i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
        { let v_ = ((278i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
        { let v_ = (((((rt::f2u(rt::fmul(F32(c.m.r32((p_view_1).wrapping_add(128))), F32(c.m.r32((p_view_1).wrapping_add(132)))))) as u64)).wrapping_shl((24i32) as u32)) | (((4575657221416845440i64) as u64))); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((1i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
        {
            l_n_6 = rt::div_i32((256i32).wrapping_shr((l_shift_4) as u32), 32i32);
            l_step_7 = rt::div_i32(128i32, l_n_6);
            { let v_ = ((((((((l_n_6) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((4i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((21331i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
            {
                l_i_8 = 0i32;
                'l1: loop {
                    if !(((((l_i_8) < (l_n_6)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        { let v_ = ((((l_srcX_5).wrapping_add((l_i_8).wrapping_mul(32i32))).wrapping_shl((4i32) as u32)) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let v_ = (((((31744i32).wrapping_add(((l_i_8).wrapping_mul(l_step_7)).wrapping_shl((4i32) as u32))) | ((31744i32).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
                        { let v_ = ((((((l_srcX_5).wrapping_add(((l_i_8).wrapping_add(1i32)).wrapping_mul(32i32))).wrapping_shl((4i32) as u32)) | ((4096i32).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let v_ = (((((((31744i32).wrapping_add((((l_i_8).wrapping_add(1i32)).wrapping_mul(l_step_7)).wrapping_shl((4i32) as u32))) as i64)) as u64)) | ((((33792i32) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
                    }
                    { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
                }
            }
        }
        {
            l_size_9 = (256i32).wrapping_shr((l_shift_4) as u32);
            { let v_ = ((((((((1i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((14i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
            { let v_ = 18374686479671755120u64; let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((76i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
            l_layer_12 = (p_view_1).wrapping_add(176);
            l_n_10 = rt::div_i32(l_size_9, l_size_9);
            l_step_11 = rt::div_i32(128i32, l_n_10);
            {
                l_k_13 = 0i32;
                'l3: loop {
                    if !(((((l_k_13) < (2i32)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        { let v_ = ((((((((1i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((2i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let v_ = ((16i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
                        { let v_ = ((342i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let v_ = (((((rt::f2u(F32(c.m.r32(((l_layer_12).wrapping_add(16)).wrapping_add(12))))) as u64)).wrapping_shl((24i32) as u32)) | (((4575657221416845440i64) as u64))); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
                        { let v_ = ((((((((l_n_10) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((4i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let v_ = ((21331i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
                        l_x_14 = rt::f2i(F32(c.m.r32(l_layer_12)));
                        l_y_15 = rt::f2i(F32(c.m.r32((l_layer_12).wrapping_add(4))));
                        l_r_16 = rt::f2i(F32(c.m.r32((l_layer_12).wrapping_add(32))));
                        {
                            l_i_17 = 0i32;
                            'l5: loop {
                                if !(((((l_i_17) < (l_n_10)) as i32)) != 0) { break 'l5; }
                                'c6: {
                                    { let v_ = (((((((l_srcX_5).wrapping_add((l_i_17).wrapping_mul(l_size_9))).wrapping_shl((4i32) as u32)).wrapping_add(8i32)) | ((8i32).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                    { let v_ = (((((((((((l_i_17).wrapping_mul(l_step_11)).wrapping_add(l_x_14)).wrapping_sub(l_r_16)).wrapping_shl((4i32) as u32)).wrapping_add(31744i32)) as i64)) | (((((((l_y_15).wrapping_sub(l_r_16)).wrapping_shl((4i32) as u32)).wrapping_add(31744i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
                                    { let v_ = (((((((l_srcX_5).wrapping_add(((l_i_17).wrapping_add(1i32)).wrapping_mul(l_size_9))).wrapping_shl((4i32) as u32)).wrapping_add(8i32)) | ((4104i32).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                    { let v_ = ((((((((((((l_i_17).wrapping_add(1i32)).wrapping_mul(l_step_11)).wrapping_add(l_r_16)).wrapping_add(l_x_14)).wrapping_shl((4i32) as u32)).wrapping_add(31744i32)) as i64)) | (((((((l_y_15).wrapping_add(l_r_16)).wrapping_add(2112i32)).wrapping_shl((4i32) as u32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
                                }
                                { let o_ = l_i_17; let n_ = (o_).wrapping_add(1); l_i_17 = n_; };
                            }
                        }
                    }
                    { { let o_ = l_k_13; let n_ = (o_).wrapping_add(1); l_k_13 = n_; }; { let o_ = l_layer_12; let n_ = (o_).wrapping_add(48); l_layer_12 = n_; }; };
                }
            }
        }
        { let v_ = ((((((((8i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((14i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
        { let v_ = (if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { 18374686479672148080u64 } else { 18374686479672147968u64 }); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((76i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
        { let v_ = ((((28672i32) as u64)) | ((((29184i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((24i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
        { let v_ = ((68i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((66i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
        { let v_ = ((4575657223564329088i64) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((1i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
        { let v_ = ((0i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((63i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
        { let v_ = ((25165868544i64) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((6i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
        { let v_ = (((((((((((c.m.r32((p_view_1).wrapping_add(148)) as i32)) as i64)) | (((((((c.m.r32((p_view_1).wrapping_add(148)) as i32)).wrapping_add((c.m.r32((p_view_1).wrapping_add(156)) as i32))).wrapping_sub(1i32)) as i64)).wrapping_shl((16i32) as u32)))) | (((((c.m.r32((p_view_1).wrapping_add(152)) as i32)) as i64)).wrapping_shl((32i32) as u32)))) | (((((((c.m.r32((p_view_1).wrapping_add(152)) as i32)).wrapping_add((c.m.r32((p_view_1).wrapping_add(160)) as i32))).wrapping_sub(1i32)) as i64)).wrapping_shl((48i32) as u32)))) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((64i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
        { let v_ = ((342i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
        {
            l_n_18 = rt::div_i32((c.m.r32((p_view_1).wrapping_add(156)) as i32), 32i32);
            l_x_19 = (c.m.r32((p_view_1).wrapping_add(148)) as i32);
            l_step_20 = rt::div_i32(128i32, l_n_18);
            l_u_21 = 0i32;
            { let v_ = ((342i32) as u64); let a_ = (fp).wrapping_add(32); c.m.w64(a_, v_); };
            { let v_ = ((536870917i32) as u32); let a_ = fp; c.m.w32(a_, v_); };
            { let v_ = ((0i32) as u32); let a_ = (fp).wrapping_add(4); c.m.w32(a_, v_); };
            { let v_ = ((268435456i32) as u32); let a_ = (fp).wrapping_add(8); c.m.w32(a_, v_); };
            { let v_ = ((1342177285i32) as u32); let a_ = (fp).wrapping_add(12); c.m.w32(a_, v_); };
            { let v_ = ((((((((1i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((8i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (fp).wrapping_add(16); c.m.w64(a_, v_); };
            { let v_ = ((4097061216u32) as u64); let a_ = (fp).wrapping_add(24); c.m.w64(a_, v_); };
            { let v_ = ((25165868544i64) as u64); let a_ = (fp).wrapping_add(40); c.m.w64(a_, v_); };
            { let v_ = ((128i32) as u8); let a_ = (fp).wrapping_add(48); c.m.w8(a_, v_); };
            { let v_ = ((128i32) as u8); let a_ = ((fp).wrapping_add(48)).wrapping_add(1); c.m.w8(a_, v_); };
            { let v_ = ((128i32) as u8); let a_ = ((fp).wrapping_add(48)).wrapping_add(2); c.m.w8(a_, v_); };
            { let v_ = ((128i32) as u8); let a_ = ((fp).wrapping_add(48)).wrapping_add(3); c.m.w8(a_, v_); };
            { let v_ = F32(0x3f800000); let a_ = ((fp).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            {
                l_i_23 = 0i32;
                'l7: loop {
                    if !(((((l_i_23) < (l_n_18)) as i32)) != 0) { break 'l7; }
                    'c8: {
                        l_u1_22 = (l_u_21).wrapping_add(l_step_20);
                        { let v_ = ((((l_u_21).wrapping_shl((4i32) as u32)).wrapping_add(8i32)) as u16); let a_ = (fp).wrapping_add(56); c.m.w16(a_, v_); };
                        { let v_ = ((8i32) as u16); let a_ = ((fp).wrapping_add(56)).wrapping_add(2); c.m.w16(a_, v_); };
                        { let v_ = ((((l_u1_22).wrapping_shl((4i32) as u32)).wrapping_add(8i32)) as u16); let a_ = (fp).wrapping_add(72); c.m.w16(a_, v_); };
                        { let v_ = ((2040i32) as u16); let a_ = ((fp).wrapping_add(72)).wrapping_add(2); c.m.w16(a_, v_); };
                        { let v_ = ((((l_x_19).wrapping_shl((4i32) as u32)).wrapping_add(28672i32)) as u64); let a_ = (fp).wrapping_add(64); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                        { let v_ = ((29184i32) as u64); let a_ = ((fp).wrapping_add(64)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                        { let v_ = ((0i32) as u64); let a_ = ((fp).wrapping_add(64)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                        { let v_ = ((255i32) as u64); let a_ = ((fp).wrapping_add(64)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                        { let v_ = (((((l_x_19).wrapping_add(32i32)).wrapping_shl((4i32) as u32)).wrapping_add(28672i32)) as u64); let a_ = (fp).wrapping_add(80); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                        { let v_ = ((36352i32) as u64); let a_ = ((fp).wrapping_add(80)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                        { let v_ = ((0i32) as u64); let a_ = ((fp).wrapping_add(80)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                        { let v_ = ((255i32) as u64); let a_ = ((fp).wrapping_add(80)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                        l_u_21 = l_u1_22;
                        { let r_ = 32i32; let n_ = (l_x_19).wrapping_add(r_); l_x_19 = n_; };
                        { let _ = { let a2_ = (((((c.m.r16(fp)) as i32)).wrapping_shl((4i32) as u32)) as u32); F::memcpy(c, l_p_3, (fp).wrapping_add(16), a2_) }; };
                        l_p_3 = (l_p_3).wrapping_add((((((c.m.r16(fp)) as i32)).wrapping_shl((4i32) as u32)) as u32));
                    }
                    { let o_ = l_i_23; let n_ = (o_).wrapping_add(1); l_i_23 = n_; };
                }
            }
        }
        F::Dma_EndDirect(c, l_p_3);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgPanBlur_DrawView", &PROBE_N, &[], true); }
    r_
}

pub fn StgPanBlur_RestoreEnv(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgPanBlur_RestoreEnv", &PROBE_N, &[], false); }
    let mut l_p_1: u32 = 0;
    let r_ = 'f: {
    l_p_1 = F::Dma_BeginDirect(c);
    { let v_ = ((((((((6i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { 524400i32 } else { 524288i32 })) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((76i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((((28672i32) as u64)) | ((((29184i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((24i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((((((((0i32) as u64)) | ((((511i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) | ((((447i32) as u64)).wrapping_shl((48i32) as u32))); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((64i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((458752i32) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((71i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((78i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((0i32) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((8i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    F::Dma_EndDirect(c, l_p_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgPanBlur_RestoreEnv", &PROBE_N, &[], true); }
    r_
}

pub fn GfxPost_DrawGlow(c: &mut Ctx, mut p_passes_1: i32, mut p_color_2: u32, mut p_glow_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxPost_DrawGlow", &PROBE_N, &[(p_passes_1) as u32, (p_color_2) as u32, (p_glow_3) as u32], false); }
    let mut l_p_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_x_6: i32 = 0;
    let mut l_u_7: i32 = 0;
    let mut l_v_8: i32 = 0;
    let mut l_u0_9: i64 = 0;
    let mut l_u1_10: i64 = 0;
    let mut l_u0_11: i64 = 0;
    let mut l_u1_12: i64 = 0;
    let r_ = 'f: {
    { let r_ = ((p_passes_1) & (1i32)); let n_ = (p_passes_1).wrapping_add(r_); p_passes_1 = n_; };
    F::Gfx_AddDefaultEnv(c);
    l_p_4 = F::Dma_BeginDirect(c);
    { let v_ = ((((((((13i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((1i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((70i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((0i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((74i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((0i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((75i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((262480i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((76i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((78i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((30720i32) as u64)) | ((((30720i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((24i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((((((0i32) as u64)) | ((((255i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) | ((((255i32) as u64)).wrapping_shl((48i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((64i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((((((((((2i32) as u64)) | ((((2i32) as u64)).wrapping_shl((2i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((4i32) as u32)))) | ((((511i32) as u64)).wrapping_shl((14i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((447i32) as u64)).wrapping_shl((34i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((8i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((549755813988i64) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((66i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((96i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((20i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((196608i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((71i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((70i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((0i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((4575657223555907584i64) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((1i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((((((1i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((2i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((85i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((2013296640i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((2281736192u32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((((((6i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((0i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((63i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { 2305843036661353984i64 } else { 2305843036661350400i64 })) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((6i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((68i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((66i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((196608i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((71i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((342i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((0i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((p_color_2) as u64)) | (((4575657221408423936i64) as u64))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((1i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((((((16i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((4i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((21331i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    l_x_6 = 0i32;
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((l_i_5) < (16i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = ((((l_i_5).wrapping_mul(32i32)).wrapping_shl((4i32) as u32)) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let v_ = ((((((l_x_6).wrapping_add(1920i32)).wrapping_shl((4i32) as u32)) | ((30720i32).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
                { let v_ = (((((((l_i_5).wrapping_add(1i32)).wrapping_mul(32i32)).wrapping_shl((4i32) as u32)) | ((7168i32).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let v_ = ((((((((l_x_6).wrapping_add(1936i32)).wrapping_shl((4i32) as u32)) as i64)) as u64)) | ((((34816i32) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
                { let r_ = 16i32; let n_ = (l_x_6).wrapping_add(r_); l_x_6 = n_; };
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
    { let v_ = ((((((((9i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((78i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((131440i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((76i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((31744i32) as u64)) | ((((31744i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((24i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((((((0i32) as u64)) | ((((127i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) | ((((127i32) as u64)).wrapping_shl((48i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((64i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((((((((((2i32) as u64)) | ((((2i32) as u64)).wrapping_shl((2i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((4i32) as u32)))) | ((((255i32) as u64)).wrapping_shl((14i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((255i32) as u64)).wrapping_shl((34i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((8i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((0i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((63i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = (((((((F::Tex_Log2Size(c, 256i32)) as u64)).wrapping_shl((26i32) as u32)) | ((((F::Tex_Log2Size(c, 256i32)) as u64)).wrapping_shl((30i32) as u32)))) | (((76288i32) as u64))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((6i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((342i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((0i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((4575657223564329088i64) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((1i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((((((8i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((4i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((21331i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    l_x_6 = 0i32;
    {
        l_i_5 = 0i32;
        'l3: loop {
            if !(((((l_i_5) < (8i32)) as i32)) != 0) { break 'l3; }
            'c4: {
                { let v_ = ((((l_i_5).wrapping_mul(32i32)).wrapping_shl((4i32) as u32)) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let v_ = ((((((l_x_6).wrapping_add(1984i32)).wrapping_shl((4i32) as u32)) | ((31744i32).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
                { let v_ = (((((((l_i_5).wrapping_add(1i32)).wrapping_mul(32i32)).wrapping_shl((4i32) as u32)) | ((4096i32).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let v_ = ((((((((l_x_6).wrapping_add(2000i32)).wrapping_shl((4i32) as u32)) as i64)) as u64)) | ((((33792i32) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
                { let r_ = 16i32; let n_ = (l_x_6).wrapping_add(r_); l_x_6 = n_; };
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
    { let v_ = ((((((((10i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((78i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((66038i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((76i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((32256i32) as u64)) | ((((32256i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((24i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((((((0i32) as u64)) | ((((63i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) | ((((63i32) as u64)).wrapping_shl((48i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((64i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((((((((((2i32) as u64)) | ((((2i32) as u64)).wrapping_shl((2i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((4i32) as u32)))) | ((((127i32) as u64)).wrapping_shl((14i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((127i32) as u64)).wrapping_shl((34i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((8i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((68i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((66i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((0i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((63i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = (((((((F::Tex_Log2Size(c, 128i32)) as u64)).wrapping_shl((26i32) as u32)) | ((((F::Tex_Log2Size(c, 128i32)) as u64)).wrapping_shl((30i32) as u32)))) | (((44544i32) as u64))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((6i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((342i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((0i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((4575657223564329088i64) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((1i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((((((2i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((4i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((21331i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((0i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((2113961472i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((134219776i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((2181071360u32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((1048592i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((2113961472i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((135268368i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((2181071360u32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((((((15i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((66040i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((77i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((79i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((((((0i32) as u64)) | ((((63i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) | ((((63i32) as u64)).wrapping_shl((48i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((65i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((32256i32) as u64)) | ((((32256i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((25i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((549755813988i64) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((67i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = (((((((F::Tex_Log2Size(c, 64i32)) as u64)).wrapping_shl((26i32) as u32)) | ((((F::Tex_Log2Size(c, 64i32)) as u64)).wrapping_shl((30i32) as u32)))) | (((32512i32) as u64))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((6i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = (((((((F::Tex_Log2Size(c, 64i32)) as u64)).wrapping_shl((26i32) as u32)) | ((((F::Tex_Log2Size(c, 64i32)) as u64)).wrapping_shl((30i32) as u32)))) | (((32448i32) as u64))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((7i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((((((((((2i32) as u64)) | ((((2i32) as u64)).wrapping_shl((2i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((4i32) as u32)))) | ((((63i32) as u64)).wrapping_shl((14i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((63i32) as u64)).wrapping_shl((34i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((8i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((((((((((2i32) as u64)) | ((((2i32) as u64)).wrapping_shl((2i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((4i32) as u32)))) | ((((63i32) as u64)).wrapping_shl((14i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((63i32) as u64)).wrapping_shl((34i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((9i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((96i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((21i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((96i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((20i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((549755813988i64) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((66i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((549755813988i64) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((67i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((196608i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((71i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((196608i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((72i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((((((p_passes_1) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((6i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((5460752i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    {
        l_i_5 = 0i32;
        'l5: loop {
            if !(((((l_i_5) < (p_passes_1)) as i32)) != 0) { break 'l5; }
            'c6: {
                if ((((l_i_5) & (1i32))) != 0) {
                    l_u_7 = 0i32;
                    l_v_8 = 0i32;
                } else {
                    l_u_7 = 16i32;
                    l_v_8 = 16i32;
                }
                { let v_ = (((if ((((!((((l_i_5) & (1i32))) != 0)) as i32)) != 0) { 854i32 } else { 342i32 })) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let v_ = ((4575657223564329088i64) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
                { let v_ = ((((((l_u_7) as i64)) | ((((l_v_8) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let v_ = ((2113961472i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
                { let v_ = (((((((l_u_7).wrapping_add(1024i32)) as i64)) | (((((l_v_8).wrapping_add(1024i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let v_ = ((2181071360u32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
    { let v_ = ((((((((10i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((131440i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((76i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((78i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((31744i32) as u64)) | ((((31744i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((24i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((((((0i32) as u64)) | ((((127i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) | ((((127i32) as u64)).wrapping_shl((48i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((64i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((((((((((2i32) as u64)) | ((((2i32) as u64)).wrapping_shl((2i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((4i32) as u32)))) | ((((63i32) as u64)).wrapping_shl((14i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((63i32) as u64)).wrapping_shl((34i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((8i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((196608i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((71i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((549755813988i64) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((66i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((p_glow_3) as u64)) | (((4575657221408423936i64) as u64))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((1i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((96i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((20i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((342i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((0i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((((((1i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((4i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((21331i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((262148i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((2080406528i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((67372036i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((2214626304u32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((((((12i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((262480i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((76i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((78i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((30720i32) as u64)) | ((((30720i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((24i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((((((0i32) as u64)) | ((((255i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) | ((((255i32) as u64)).wrapping_shl((48i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((64i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((((((((((2i32) as u64)) | ((((2i32) as u64)).wrapping_shl((2i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((4i32) as u32)))) | ((((127i32) as u64)).wrapping_shl((14i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((127i32) as u64)).wrapping_shl((34i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((8i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((196608i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((71i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((549755813988i64) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((66i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((4575657223564329088i64) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((1i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((96i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((20i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((342i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((0i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((0i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((63i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = (((((((F::Tex_Log2Size(c, 128i32)) as u64)).wrapping_shl((26i32) as u32)) | ((((F::Tex_Log2Size(c, 128i32)) as u64)).wrapping_shl((30i32) as u32)))) | (((44544i32) as u64))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((6i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((((((4i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((4i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((21331i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    l_x_6 = 0i32;
    {
        l_i_5 = 0i32;
        'l7: loop {
            if !(((((l_i_5) < (4i32)) as i32)) != 0) { break 'l7; }
            'c8: {
                l_u0_9 = ((((l_x_6).wrapping_shl((4i32) as u32)).wrapping_add(8i32)) as i64);
                l_u1_10 = (((((l_x_6).wrapping_add(32i32)).wrapping_shl((4i32) as u32)).wrapping_add(8i32)) as i64);
                { let v_ = ((((l_u0_9) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let v_ = (((((((l_i_5).wrapping_mul(64i32)).wrapping_add(1920i32)).wrapping_shl((4i32) as u32)) | ((30720i32).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
                { let v_ = ((((l_u1_10) as u64)) | ((((2056i32) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let v_ = (((((((((l_x_6).wrapping_mul(2i32)).wrapping_add(1984i32)).wrapping_shl((4i32) as u32)) as i64)) as u64)) | ((((34816i32) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
                { let r_ = 32i32; let n_ = (l_x_6).wrapping_add(r_); l_x_6 = n_; };
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
    { let v_ = ((((((((10i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((4575657223564329088i64) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((1i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { 524400i32 } else { 524288i32 })) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((76i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((78i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((28672i32) as u64)) | ((((29184i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((24i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((((((0i32) as u64)) | ((((511i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) | ((((447i32) as u64)).wrapping_shl((48i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((64i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((196608i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((71i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((((((((((2i32) as u64)) | ((((2i32) as u64)).wrapping_shl((2i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((4i32) as u32)))) | ((((255i32) as u64)).wrapping_shl((14i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((255i32) as u64)).wrapping_shl((34i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((8i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = (((((((F::Tex_Log2Size(c, 256i32)) as u64)).wrapping_shl((26i32) as u32)) | ((((F::Tex_Log2Size(c, 256i32)) as u64)).wrapping_shl((30i32) as u32)))) | (((76288i32) as u64))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((6i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((0i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((63i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((549755813992i64) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((66i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((((((8i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((4i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((21331i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    l_x_6 = 0i32;
    {
        l_i_5 = 0i32;
        'l9: loop {
            if !(((((l_i_5) < (8i32)) as i32)) != 0) { break 'l9; }
            'c10: {
                l_u0_11 = ((((l_x_6).wrapping_shl((4i32) as u32)).wrapping_add(8i32)) as i64);
                l_u1_12 = (((((l_x_6).wrapping_add(32i32)).wrapping_shl((4i32) as u32)).wrapping_add(8i32)) as i64);
                { let v_ = ((((l_u0_11) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let v_ = (((((((l_i_5).wrapping_mul(64i32)).wrapping_add(1792i32)).wrapping_shl((4i32) as u32)) | ((29184i32).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
                { let v_ = ((((l_u1_12) as u64)) | ((((4104i32) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let v_ = (((((((((l_x_6).wrapping_mul(2i32)).wrapping_add(1856i32)).wrapping_shl((4i32) as u32)) as i64)) as u64)) | ((((36352i32) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
                { let r_ = 32i32; let n_ = (l_x_6).wrapping_add(r_); l_x_6 = n_; };
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
    { let v_ = ((((((((6i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { 524400i32 } else { 524288i32 })) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((76i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((28672i32) as u64)) | ((((29184i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((24i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((((((0i32) as u64)) | ((((511i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) | ((((447i32) as u64)).wrapping_shl((48i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((64i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((458752i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((71i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((78i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((0i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((8i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    F::Dma_EndDirect(c, l_p_4);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "GfxPost_DrawGlow", &PROBE_N, &[], true); }
    r_
}

pub fn GfxPost_DrawTex16(c: &mut Ctx, mut p_tbp_1: i32, mut p_alpha_2: u64) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxPost_DrawTex16", &PROBE_N, &[(p_tbp_1) as u32, (p_alpha_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_p_3: u32 = 0;
    let mut l_tag_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_u1_6: i32 = 0;
    let mut l_step_7: i32 = 0;
    let mut l_u_8: i32 = 0;
    let mut l_x_9: i32 = 0;
    let mut l_half_10: i32 = 0;
    let mut l_w_11: i32 = 0;
    let mut l_h_12: i32 = 0;
    let mut l_n_13: i32 = 0;
    let r_ = 'f: {
        l_step_7 = 32i32;
        l_u_8 = 0i32;
        l_x_9 = 0i32;
        l_half_10 = 1i32;
        l_w_11 = 32i32;
        l_h_12 = 448i32;
        l_n_13 = rt::div_i32(512i32, l_step_7);
        l_p_3 = F::Dma_BeginDirect(c);
        { let v_ = ((((((((8i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((14i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
        { let v_ = ((196608i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((71i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
        { let v_ = p_alpha_2; let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((66i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
        { let v_ = ((0i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((63i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
        { let v_ = ((549755814016i64) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((59i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
        { let v_ = ((0i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((20i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
        { let v_ = ((((((((((p_tbp_1) | (606208000i32))) as u32)) as i64)) | (61203283968i64))) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((6i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
        { let v_ = ((342i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
        { let v_ = ((4575657223564329088i64) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((1i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
        l_tag_4 = l_p_3;
        { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
        {
            l_i_5 = 0i32;
            'l1: loop {
                if !(((((l_i_5) < (l_n_13)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_u1_6 = (l_u_8).wrapping_add(l_step_7);
                    F::GfxPostQuad_Set(c, fp, 1792i32, 1824i32, ((l_u_8) as i16), ((0i32) as i16), ((l_u1_6) as i16), ((l_h_12) as i16), l_x_9, 0i32, l_w_11, l_h_12, ((l_half_10) as i16));
                    { let v_ = (((((((c.m.r32((fp).wrapping_add(32)) as i32)) as i64)) | (((((c.m.r32((fp).wrapping_add(36)) as i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = (((((((c.m.r32((fp).wrapping_add(16)) as i32)) as i64)) | ((((((c.m.r32((fp).wrapping_add(20)) as i32)).wrapping_sub(16i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
                    { let v_ = (((((((c.m.r32((fp).wrapping_add(40)) as i32)) as i64)) | (((((c.m.r32((fp).wrapping_add(44)) as i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = (((((((c.m.r32((fp).wrapping_add(24)) as i32)) as i64)) | ((((((c.m.r32((fp).wrapping_add(28)) as i32)).wrapping_sub(16i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
                    l_u_8 = l_u1_6;
                    { let r_ = l_step_7; let n_ = (l_x_9).wrapping_add(r_); l_x_9 = n_; };
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
        { let v_ = ((((((((16i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((4i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_tag_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((21331i32) as u64); let a_ = (l_tag_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        F::Dma_EndDirect(c, l_p_3);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "GfxPost_DrawTex16", &PROBE_N, &[], true); }
    r_
}

pub fn GfxPost_AddFrame16(c: &mut Ctx, mut p_fbp_1: i32, mut p_fbmsk_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxPost_AddFrame16", &PROBE_N, &[(p_fbp_1) as u32, (p_fbmsk_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_lo_3: i32 = 0;
    let r_ = 'f: {
        l_lo_3 = ((p_fbp_1) | (34078720i32));
        rt::memset(c, fp, 0, 48);
        { let v_ = ((((268435456i32) | (2i32))) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((((1342177280i32) | (2i32))) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((((32768i32) | (1i32))) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = ((l_lo_3) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = (((((((l_lo_3) as i64)) | ((((p_fbmsk_2) as i64)).wrapping_shl((32i32) as u32)))).wrapping_shr((32i32) as u32)) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((76i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        F::Dma_AddData(c, fp, ((48u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "GfxPost_AddFrame16", &PROBE_N, &[], true); }
    r_
}

pub fn GfxPost_AddScreenFrame(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxPost_AddScreenFrame", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 64);
        rt::memset(c, fp.wrapping_add(0), 0, 64);
        { let v_ = ((((268435456i32) | (3i32))) as u32); c.m.w32(fp.wrapping_add(0).wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(0).wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(0).wrapping_add(8), v_); }
        { let v_ = ((((1342177280i32) | (3i32))) as u32); c.m.w32(fp.wrapping_add(0).wrapping_add(12), v_); }
        { let v_ = ((((32768i32) | (2i32))) as u32); c.m.w32(fp.wrapping_add(0).wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(0).wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(0).wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(0).wrapping_add(28), v_); }
        { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { 524400i32 } else { 524288i32 })) as u32); c.m.w32(fp.wrapping_add(0).wrapping_add(32), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(0).wrapping_add(36), v_); }
        { let v_ = ((76i32) as u32); c.m.w32(fp.wrapping_add(0).wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(0).wrapping_add(44), v_); }
        { let v_ = ((28672i32) as u32); c.m.w32(fp.wrapping_add(0).wrapping_add(48), v_); }
        { let v_ = ((29184i32) as u32); c.m.w32(fp.wrapping_add(0).wrapping_add(52), v_); }
        { let v_ = ((24i32) as u32); c.m.w32(fp.wrapping_add(0).wrapping_add(56), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(0).wrapping_add(60), v_); }
        F::Dma_AddData(c, fp, ((64u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "GfxPost_AddScreenFrame", &PROBE_N, &[], true); }
    r_
}

pub fn ObjOutline_BuildClut(c: &mut Ctx, mut p_clut_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjOutline_BuildClut", &PROBE_N, &[(p_clut_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let mut l_k_3: i32 = 0;
    let mut l_c_4: u8 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (256i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_k_3 = ((((((l_i_2) & (231i32))) | ((((l_i_2) & (8i32))).wrapping_shl((1i32) as u32)))) | ((((l_i_2) & (16i32))).wrapping_shr((1i32) as u32)));
                { let r_ = 4i32; let n_ = (l_k_3).wrapping_mul(r_); l_k_3 = n_; };
                if (((((l_i_2) == (255i32)) as i32)) != 0) {
                    { let v_ = ((0i32) as u8); let a_ = (p_clut_1).wrapping_add((((l_k_3).wrapping_add(0i32)) as u32)); c.m.w8(a_, v_); };
                    { let v_ = ((0i32) as u8); let a_ = (p_clut_1).wrapping_add((((l_k_3).wrapping_add(1i32)) as u32)); c.m.w8(a_, v_); };
                    { let v_ = ((0i32) as u8); let a_ = (p_clut_1).wrapping_add((((l_k_3).wrapping_add(2i32)) as u32)); c.m.w8(a_, v_); };
                    { let v_ = ((0i32) as u8); let a_ = (p_clut_1).wrapping_add((((l_k_3).wrapping_add(3i32)) as u32)); c.m.w8(a_, v_); };
                } else {
                    l_c_4 = (((l_i_2).wrapping_shl((3i32) as u32)) as u8);
                    if (((((((l_c_4) as i32)) == (0i32)) as i32)) != 0) {
                        l_c_4 = ((128i32) as u8);
                    }
                    { let v_ = l_c_4; let a_ = (p_clut_1).wrapping_add((((l_k_3).wrapping_add(0i32)) as u32)); c.m.w8(a_, v_); };
                    { let v_ = l_c_4; let a_ = (p_clut_1).wrapping_add((((l_k_3).wrapping_add(1i32)) as u32)); c.m.w8(a_, v_); };
                    { let v_ = ((0i32) as u8); let a_ = (p_clut_1).wrapping_add((((l_k_3).wrapping_add(2i32)) as u32)); c.m.w8(a_, v_); };
                    { let v_ = ((128i32) as u8); let a_ = (p_clut_1).wrapping_add((((l_k_3).wrapping_add(3i32)) as u32)); c.m.w8(a_, v_); };
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ObjOutline_BuildClut", &PROBE_N, &[], true); }
    r_
}

pub fn GfxPost_DrawDepthClutAt(c: &mut Ctx, mut p_half_1: i32, mut p_tbp_2: i32, mut p_cbp_3: i32, mut p_alpha_4: u64, mut p_dx_5: i32, mut p_dy_6: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxPost_DrawDepthClutAt", &PROBE_N, &[(p_half_1) as u32, (p_tbp_2) as u32, (p_cbp_3) as u32, (p_alpha_4) as u32, (p_dx_5) as u32, (p_dy_6) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_p_7: u32 = 0;
    let mut l_tag_8: u32 = 0;
    let mut l_i_9: i32 = 0;
    let mut l_u1_10: i32 = 0;
    let mut l_step_11: i32 = 0;
    let mut l_u_12: i32 = 0;
    let mut l_x_13: i32 = 0;
    let mut l_w_14: i32 = 0;
    let mut l_h_15: i32 = 0;
    let mut l_n_16: i32 = 0;
    let r_ = 'f: {
        l_step_11 = 32i32;
        l_u_12 = 0i32;
        l_x_13 = 0i32;
        l_w_14 = 32i32;
        l_h_15 = 448i32;
        l_n_16 = rt::div_i32(512i32, l_step_11);
        l_p_7 = F::Dma_BeginDirect(c);
        { let v_ = ((((((((10i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((14i32) as u64); let a_ = (l_p_7).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
        { let v_ = ((196608i32) as u64); let a_ = (l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((71i32) as u64); let a_ = (l_p_7).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
        { let v_ = ((0i32) as u64); let a_ = (l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((70i32) as u64); let a_ = (l_p_7).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
        { let v_ = p_alpha_4; let a_ = (l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((66i32) as u64); let a_ = (l_p_7).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
        { let v_ = ((0i32) as u64); let a_ = (l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((63i32) as u64); let a_ = (l_p_7).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
        { let v_ = ((32768i32) as u64); let a_ = (l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((59i32) as u64); let a_ = (l_p_7).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
        { let v_ = (((((p_dx_5).wrapping_add(28672i32)) as u64)) | (((((p_dy_6).wrapping_add(29184i32)) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((24i32) as u64); let a_ = (l_p_7).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
        if (((((p_half_1) != (0i32)) as i32)) != 0) {
            { let v_ = ((96i32) as u64); let a_ = (l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((20i32) as u64); let a_ = (l_p_7).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
        } else {
            { let v_ = ((0i32) as u64); let a_ = (l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((20i32) as u64); let a_ = (l_p_7).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
        }
        { let v_ = ((((((((((p_tbp_2) | (632422400i32))) as u32)) as u64)) | ((((p_cbp_3) as u64)).wrapping_shl((37i32) as u32)))) | (((2305843070416977920i64) as u64))); let a_ = (l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((6i32) as u64); let a_ = (l_p_7).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
        { let v_ = ((342i32) as u64); let a_ = (l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u64); let a_ = (l_p_7).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
        { let v_ = ((4575657223564329088i64) as u64); let a_ = (l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((1i32) as u64); let a_ = (l_p_7).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
        l_tag_8 = l_p_7;
        { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
        {
            l_i_9 = 0i32;
            'l1: loop {
                if !(((((l_i_9) < (l_n_16)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_u1_10 = (l_u_12).wrapping_add(l_step_11);
                    F::GfxPostQuad_Set(c, fp, 1792i32, 1824i32, ((l_u_12) as i16), ((0i32) as i16), ((l_u1_10) as i16), ((l_h_15) as i16), l_x_13, 0i32, l_w_14, l_h_15, ((p_half_1) as i16));
                    { let v_ = (((((((c.m.r32((fp).wrapping_add(32)) as i32)) as i64)) | (((((c.m.r32((fp).wrapping_add(36)) as i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = (((((((c.m.r32((fp).wrapping_add(16)) as i32)) as i64)) | (((((c.m.r32((fp).wrapping_add(20)) as i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_7).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
                    { let v_ = (((((((c.m.r32((fp).wrapping_add(40)) as i32)) as i64)) | (((((c.m.r32((fp).wrapping_add(44)) as i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = (((((((c.m.r32((fp).wrapping_add(24)) as i32)) as i64)) | (((((c.m.r32((fp).wrapping_add(28)) as i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_7).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
                    l_u_12 = l_u1_10;
                    { let r_ = l_step_11; let n_ = (l_x_13).wrapping_add(r_); l_x_13 = n_; };
                }
                { let o_ = l_i_9; let n_ = (o_).wrapping_add(1); l_i_9 = n_; };
            }
        }
        { let v_ = ((((((((16i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((4i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_tag_8).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((21331i32) as u64); let a_ = (l_tag_8).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        F::Dma_EndDirect(c, l_p_7);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "GfxPost_DrawDepthClutAt", &PROBE_N, &[], true); }
    r_
}

pub fn GfxPost_DrawTex16Texa(c: &mut Ctx, mut p_tbp_1: i32, mut p_alpha_2: u64, mut p_ta0_3: u8) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxPost_DrawTex16Texa", &PROBE_N, &[(p_tbp_1) as u32, (p_alpha_2) as u32, (p_ta0_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_p_4: u32 = 0;
    let mut l_tag_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_u1_7: i32 = 0;
    let mut l_step_8: i32 = 0;
    let mut l_u_9: i32 = 0;
    let mut l_x_10: i32 = 0;
    let mut l_half_11: i32 = 0;
    let mut l_w_12: i32 = 0;
    let mut l_h_13: i32 = 0;
    let mut l_n_14: i32 = 0;
    let r_ = 'f: {
        l_step_8 = 32i32;
        l_u_9 = 0i32;
        l_x_10 = 0i32;
        l_half_11 = 1i32;
        l_w_12 = 32i32;
        l_h_13 = 448i32;
        l_n_14 = rt::div_i32(512i32, l_step_8);
        l_p_4 = F::Dma_BeginDirect(c);
        { let v_ = ((((((((8i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((14i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
        { let v_ = ((196623i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((71i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
        { let v_ = p_alpha_2; let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((66i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
        { let v_ = ((0i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((63i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
        { let v_ = ((((((p_ta0_3) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((59i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
        { let v_ = ((96i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((20i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
        { let v_ = ((((((((((p_tbp_1) | (606208000i32))) as u32)) as i64)) | (61203283968i64))) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((6i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
        { let v_ = ((342i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
        { let v_ = ((4575657223564329088i64) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((1i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
        l_tag_5 = l_p_4;
        { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
        {
            l_i_6 = 0i32;
            'l1: loop {
                if !(((((l_i_6) < (l_n_14)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_u1_7 = (l_u_9).wrapping_add(l_step_8);
                    F::GfxPostQuad_Set(c, fp, 1792i32, 1824i32, ((l_u_9) as i16), ((0i32) as i16), ((l_u1_7) as i16), ((l_h_13) as i16), l_x_10, 0i32, l_w_12, l_h_13, ((l_half_11) as i16));
                    { let v_ = (((((((c.m.r32((fp).wrapping_add(32)) as i32)) as i64)) | (((((c.m.r32((fp).wrapping_add(36)) as i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = (((((((c.m.r32((fp).wrapping_add(16)) as i32)) as i64)) | ((((((c.m.r32((fp).wrapping_add(20)) as i32)).wrapping_sub(16i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
                    { let v_ = (((((((c.m.r32((fp).wrapping_add(40)) as i32)) as i64)) | (((((c.m.r32((fp).wrapping_add(44)) as i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = (((((((c.m.r32((fp).wrapping_add(24)) as i32)) as i64)) | ((((((c.m.r32((fp).wrapping_add(28)) as i32)).wrapping_sub(16i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
                    l_u_9 = l_u1_7;
                    { let r_ = l_step_8; let n_ = (l_x_10).wrapping_add(r_); l_x_10 = n_; };
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
        { let v_ = ((((((((16i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((4i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_tag_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((21331i32) as u64); let a_ = (l_tag_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        F::Dma_EndDirect(c, l_p_4);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "GfxPost_DrawTex16Texa", &PROBE_N, &[], true); }
    r_
}

pub fn GfxPost_FillBlend(c: &mut Ctx, mut p_alpha_1: u64, mut p_rgbaq_2: u64) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxPost_FillBlend", &PROBE_N, &[(p_alpha_1) as u32, (p_rgbaq_2) as u32], false); }
    let mut l_p_3: u32 = 0;
    let mut l_tag_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_x0_6: i32 = 0;
    let r_ = 'f: {
    l_p_3 = F::Dma_BeginDirect(c);
    { let v_ = ((((((((6i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = ((1i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((70i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = p_alpha_1; let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((66i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = ((196608i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((71i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = ((0i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((74i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = ((70i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((0i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = p_rgbaq_2; let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((1i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    l_tag_4 = l_p_3;
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    l_x0_6 = 0i32;
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((l_i_5) < (16i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = ((((((l_x0_6).wrapping_add(1792i32)).wrapping_shl((4i32) as u32)) | ((29184i32).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let v_ = ((((((((l_x0_6).wrapping_add(1824i32)).wrapping_shl((4i32) as u32)) as i64)) as u64)) | ((((36352i32) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
                { let r_ = 32i32; let n_ = (l_x0_6).wrapping_add(r_); l_x0_6 = n_; };
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
    { let v_ = ((((((((16i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((2i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_tag_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((85i32) as u64); let a_ = (l_tag_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    F::Dma_EndDirect(c, l_p_3);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "GfxPost_FillBlend", &PROBE_N, &[], true); }
    r_
}

pub fn GfxPost_ClearWork(c: &mut Ctx, mut p_fbp_1: i32, mut p_fbmsk_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxPost_ClearWork", &PROBE_N, &[(p_fbp_1) as u32, (p_fbmsk_2) as u32], false); }
    let mut l_p_3: u32 = 0;
    let mut l_tag_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_x0_6: i32 = 0;
    let r_ = 'f: {
    l_p_3 = F::Dma_BeginDirect(c);
    { let v_ = ((((((((8i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = ((((p_fbp_1) | (34078720i32))) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((76i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = ((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((78i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = ((((28672i32) as u64)) | ((((29184i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((24i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = ((549755813988i64) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((66i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = ((196608i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((71i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = ((0i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((74i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = ((70i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((0i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = ((4575657221408423936i64) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((1i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    l_tag_4 = l_p_3;
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    l_x0_6 = 0i32;
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((l_i_5) < (16i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = ((((((l_x0_6).wrapping_add(1792i32)).wrapping_shl((4i32) as u32)) | ((29184i32).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let v_ = ((((((((l_x0_6).wrapping_add(1824i32)).wrapping_shl((4i32) as u32)) as i64)) as u64)) | ((((36352i32) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
                { let r_ = 32i32; let n_ = (l_x0_6).wrapping_add(r_); l_x0_6 = n_; };
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
    { let v_ = ((((((((16i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((2i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_tag_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((85i32) as u64); let a_ = (l_tag_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((((((((1i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = ((((((((p_fbp_1) | (34078720i32))) as i64)) | ((((p_fbmsk_2) as i64)).wrapping_shl((32i32) as u32)))) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((76i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    F::Dma_EndDirect(c, l_p_3);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "GfxPost_ClearWork", &PROBE_N, &[], true); }
    r_
}

pub fn GfxPost_AddXyOffset(c: &mut Ctx, mut p_x_1: i32, mut p_y_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxPost_AddXyOffset", &PROBE_N, &[(p_x_1) as u32, (p_y_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 48);
        { let v_ = ((((268435456i32) | (2i32))) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((((1342177280i32) | (2i32))) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((((32768i32) | (1i32))) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = ((p_x_1) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = (((((((p_x_1) as i64)) | ((((p_y_2) as i64)).wrapping_shl((32i32) as u32)))).wrapping_shr((32i32) as u32)) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((24i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        F::Dma_AddData(c, fp, ((48u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "GfxPost_AddXyOffset", &PROBE_N, &[], true); }
    r_
}

pub fn ObjOutline_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjOutline_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, ((1280u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gObjOutline; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gObjOutline); F::memset(c, a0_, 0i32, 1280u32) }; };
    { let a0_ = (c.m.r32(sym::gObjOutline)).wrapping_add(64); F::GfxClut_InitPacket(c, a0_, ((16012i32) as u16)) };
    { let a0_ = { let a_ = ((c.m.r32(sym::gObjOutline)).wrapping_add(64)).wrapping_add(1168); c.m.r32(a_) }; F::ObjOutline_BuildClut(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ObjOutline_Init", &PROBE_N, &[], true); }
    r_
}

pub fn ObjOutline_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjOutline_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32(sym::gObjOutline); F::Heap_Free(c, a0_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gObjOutline; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ObjOutline_Term", &PROBE_N, &[], true); }
    r_
}

pub fn ObjOutline_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjOutline_Draw", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Port_GsMarker(c, 1i32);
    { let a0_ = ((c.m.r32(sym::gObjOutline)).wrapping_add(64)).wrapping_add(1172); F::Dma_AddData(c, a0_, 16i32) };
    F::Dma_AddTexFlush(c);
    F::GfxPost_ClearWork(c, 336i32, ((4278190080u32) as i32));
    { let a2_ = (({ let a_ = ((c.m.r32(sym::gObjOutline)).wrapping_add(64)).wrapping_add(1204); c.m.r16(a_) }) as i32); F::GfxPost_DrawDepthClutAt(c, 0i32, 7168i32, a2_, ((549755813956i64) as u64), 0i32, 0i32) };
    F::GfxPost_AddFrame16(c, 336i32, ((4294967040u32) as i32));
    { let a2_ = (({ let a_ = ((c.m.r32(sym::gObjOutline)).wrapping_add(64)).wrapping_add(1204); c.m.r16(a_) }) as i32); F::GfxPost_DrawDepthClutAt(c, 0i32, 7168i32, a2_, ((549755813986i64) as u64), 0i32, 8i32) };
    F::GfxPost_AddFrame16(c, 336i32, ((4294902015u32) as i32));
    { let a2_ = (({ let a_ = ((c.m.r32(sym::gObjOutline)).wrapping_add(64)).wrapping_add(1204); c.m.r16(a_) }) as i32); F::GfxPost_DrawDepthClutAt(c, 0i32, 7168i32, a2_, ((549755813986i64) as u64), 16i32, 0i32) };
    F::GfxPost_AddScreenFrame(c);
    F::Gfx_ClearScreen(c, 16777215i32, ((0i32) as u32));
    F::Dma_AddZbuf(c, 224i32, 1i32);
    { let a0_ = (if (((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0) { 0i32 } else { 112i32 }); F::Dma_AddFrame(c, a0_, 8i32, 16777215i32) };
    F::Dma_AddColClamp(c, ((1i32) as u64));
    F::GfxPost_AddXyOffset(c, 28672i32, 29200i32);
    F::GfxPost_DrawTex16Texa(c, 10752i32, ((549755813988i64) as u64), ((48i32) as u8));
    F::GfxPost_AddXyOffset(c, 28672i32, 29168i32);
    F::GfxPost_DrawTex16Texa(c, 10752i32, ((549755813988i64) as u64), ((48i32) as u8));
    F::GfxPost_AddXyOffset(c, 28672i32, 29184i32);
    F::GfxPost_DrawTex16Texa(c, 10752i32, ((68i32) as u64), ((128i32) as u8));
    { let a0_ = (if (((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0) { 0i32 } else { 112i32 }); F::Dma_AddFrame(c, a0_, 8i32, 0i32) };
    F::GfxPost_FillBlend(c, ((82i32) as u64), ((4575657223562486884i64) as u64));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ObjOutline_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn StgGlare_BuildClut(c: &mut Ctx, mut p_clut_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgGlare_BuildClut", &PROBE_N, &[(p_clut_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let mut l_k_3: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (256i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_k_3 = (((((((l_i_2) & (231i32))) | ((((l_i_2) & (8i32))).wrapping_shl((1i32) as u32)))) | ((((l_i_2) & (16i32))).wrapping_shr((1i32) as u32)))).wrapping_mul(4i32);
                if (((((l_i_2) == (255i32)) as i32)) != 0) {
                    { let v_ = ((255i32) as u8); let a_ = (p_clut_1).wrapping_add((((l_k_3).wrapping_add(0i32)) as u32)); c.m.w8(a_, v_); };
                    { let v_ = ((255i32) as u8); let a_ = (p_clut_1).wrapping_add((((l_k_3).wrapping_add(1i32)) as u32)); c.m.w8(a_, v_); };
                    { let v_ = ((255i32) as u8); let a_ = (p_clut_1).wrapping_add((((l_k_3).wrapping_add(2i32)) as u32)); c.m.w8(a_, v_); };
                    { let v_ = ((128i32) as u8); let a_ = (p_clut_1).wrapping_add((((l_k_3).wrapping_add(3i32)) as u32)); c.m.w8(a_, v_); };
                } else {
                    { let v_ = ((255i32) as u8); let a_ = (p_clut_1).wrapping_add((((l_k_3).wrapping_add(0i32)) as u32)); c.m.w8(a_, v_); };
                    { let v_ = ((255i32) as u8); let a_ = (p_clut_1).wrapping_add((((l_k_3).wrapping_add(1i32)) as u32)); c.m.w8(a_, v_); };
                    { let v_ = ((255i32) as u8); let a_ = (p_clut_1).wrapping_add((((l_k_3).wrapping_add(2i32)) as u32)); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((sym::gStgGlare).wrapping_add(1268)) as i32)) as u8); let a_ = (p_clut_1).wrapping_add((((l_k_3).wrapping_add(3i32)) as u32)); c.m.w8(a_, v_); };
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgGlare_BuildClut", &PROBE_N, &[], true); }
    r_
}

pub fn StgGlare_Nop(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgGlare_Nop", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgGlare_Nop", &PROBE_N, &[], true); }
    r_
}

pub fn StgGlare_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgGlare_Update", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let mut l_view_1: u32 = 0;
    let r_ = 'f: {
        l_view_1 = c.m.r32(sym::gBtlCamView);
        if ((((((c.m.r32((sym::gStgGlare).wrapping_add(1220)) as i32)) != (0i32)) as i32)) != 0) {
            if ((((((c.m.r32((sym::gStgGlare).wrapping_add(1224)) as i32)) != (0i32)) as i32)) != 0) {
                F::BtlStage_GetLightVecA(c, fp.wrapping_add(64));
                F::Vu0Screen_StoreMtx(c, fp);
                F::Mtx_ProjectInt(c, fp.wrapping_add(80), fp, fp.wrapping_add(64));
                { let v_ = rt::f2i(rt::fsub(rt::i2f((c.m.r32((fp.wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)), F32(0x44e00000))); let a_ = (fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = rt::f2i(rt::fsub(rt::i2f((c.m.r32((fp.wrapping_add(80)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)), F32(0x44e40000))); let a_ = (fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = (c.m.r32((fp.wrapping_add(80)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32); let a_ = (fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                if ((((((((((((((((((((((c.m.r32((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) > (0i32)) as i32)) != 0) && ((((((c.m.r32((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) >= ((c.m.r32((l_view_1).wrapping_add(512)) as i32))) as i32)) != 0)) as i32)) != 0) && ((((((c.m.r32((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) <= (((c.m.r32((l_view_1).wrapping_add(516)) as i32)).wrapping_add(1i32))) as i32)) != 0)) as i32)) != 0) && ((((((c.m.r32((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) >= ((c.m.r32((l_view_1).wrapping_add(520)) as i32))) as i32)) != 0)) as i32)) != 0) && ((((((c.m.r32((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) <= (((c.m.r32((l_view_1).wrapping_add(524)) as i32)).wrapping_add(1i32))) as i32)) != 0)) as i32)) != 0) {
                    if ((((((((c.m.r32((sym::gStgGlare).wrapping_add(1248)) as i32)) as u32)) < (((2i32) as u32))) as i32)) != 0) {
                        { let v_ = 1i32; let a_ = (sym::gStgGlare).wrapping_add(1248); c.m.w32(a_, (v_) as u32); };
                        { let r_ = (c.m.r32((sym::gStgGlare).wrapping_add(1264)) as i32); let a_ = (sym::gStgGlare).wrapping_add(1236); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                        if ((((((c.m.r32((sym::gStgGlare).wrapping_add(1236)) as i32)) > ((c.m.r32((sym::gStgGlare).wrapping_add(1252)) as i32))) as i32)) != 0) {
                            { let v_ = (c.m.r32((sym::gStgGlare).wrapping_add(1252)) as i32); let a_ = (sym::gStgGlare).wrapping_add(1236); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 2i32; let a_ = (sym::gStgGlare).wrapping_add(1248); c.m.w32(a_, (v_) as u32); };
                        }
                    } else {
                        { let r_ = (c.m.r32((sym::gStgGlare).wrapping_add(1264)) as i32); let a_ = (sym::gStgGlare).wrapping_add(1236); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                        if ((((((c.m.r32((sym::gStgGlare).wrapping_add(1236)) as i32)) < ((c.m.r32((sym::gStgGlare).wrapping_add(1256)) as i32))) as i32)) != 0) {
                            { let v_ = (c.m.r32((sym::gStgGlare).wrapping_add(1256)) as i32); let a_ = (sym::gStgGlare).wrapping_add(1236); c.m.w32(a_, (v_) as u32); };
                        }
                    }
                } else {
                    { let a_ = (sym::gStgGlare).wrapping_add(1236); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (sym::gStgGlare).wrapping_add(1248); c.m.w32(a_, (v_) as u32); };
                    if ((((((c.m.r32((sym::gStgGlare).wrapping_add(1236)) as i32)) < ((c.m.r32((sym::gStgGlare).wrapping_add(1260)) as i32))) as i32)) != 0) {
                        { let v_ = (c.m.r32((sym::gStgGlare).wrapping_add(1260)) as i32); let a_ = (sym::gStgGlare).wrapping_add(1236); c.m.w32(a_, (v_) as u32); };
                    }
                    { let v_ = (c.m.r32((sym::gStgGlare).wrapping_add(1264)) as i32); let a_ = (sym::gStgGlare).wrapping_add(1244); c.m.w32(a_, (v_) as u32); };
                }
            } else {
                { let v_ = (c.m.r32((sym::gStgGlare).wrapping_add(1252)) as i32); let a_ = (sym::gStgGlare).wrapping_add(1236); c.m.w32(a_, (v_) as u32); };
                { let v_ = (c.m.r32((sym::gStgGlare).wrapping_add(1264)) as i32); let a_ = (sym::gStgGlare).wrapping_add(1244); c.m.w32(a_, (v_) as u32); };
            }
            if ((((((c.m.r32((sym::gStgGlare).wrapping_add(1232)) as i32)) != (0i32)) as i32)) != 0) {
                { let v_ = (c.m.r32((sym::gStgGlare).wrapping_add(1252)) as i32); let a_ = (sym::gStgGlare).wrapping_add(1236); c.m.w32(a_, (v_) as u32); };
                { let v_ = (c.m.r32((sym::gStgGlare).wrapping_add(1264)) as i32); let a_ = (sym::gStgGlare).wrapping_add(1244); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (sym::gStgGlare).wrapping_add(1232); c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = 0i32; let a_ = (sym::gStgGlare).wrapping_add(1232); c.m.w32(a_, (v_) as u32); };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgGlare_Update", &PROBE_N, &[], true); }
    r_
}

pub fn StgGlare_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgGlare_Reset", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((((c.m.r32((sym::gStgGlare).wrapping_add(1220)) as i32)) == (0i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (sym::gStgGlare).wrapping_add(1232); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (sym::gStgGlare).wrapping_add(1248); c.m.w32(a_, (v_) as u32); };
        { let v_ = (c.m.r32((sym::gStgGlare).wrapping_add(1264)) as i32); let a_ = (sym::gStgGlare).wrapping_add(1236); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgGlare_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn StgGlare_Init(c: &mut Ctx, mut p_cbp_1: u16) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgGlare_Init", &PROBE_N, &[(p_cbp_1) as u32], false); }
    let r_ = 'f: {
    { let _ = F::memset(c, sym::gStgGlare, 0i32, 1280u32); };
    F::GfxClut_InitPacket(c, sym::gStgGlare, p_cbp_1);
    F::StgGlare_Reset(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgGlare_Init", &PROBE_N, &[], true); }
    r_
}

pub fn StgGlare_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgGlare_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgGlare_Term", &PROBE_N, &[], true); }
    r_
}

pub fn StgGlare_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgGlare_Draw", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((((c.m.r32((sym::gStgGlare).wrapping_add(1220)) as i32)) != (0i32)) as i32)) != 0) {
        F::Gfx_AddDefaultEnv(c);
        F::Dma_AddData(c, (sym::gStgGlare).wrapping_add(1172), 16i32);
        F::Dma_AddTexFlush(c);
        F::Dma_AddZbuf(c, 224i32, 1i32);
        { let a0_ = (if (((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0) { 0i32 } else { 112i32 }); F::Dma_AddFrame(c, a0_, 8i32, 16777215i32) };
        F::Dma_AddFillRect(c, 1792i32, 1824i32, 512i32, 448i32, ((0i32) as u32));
        { let a0_ = (if (((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0) { 0i32 } else { 112i32 }); F::Dma_AddFrame(c, a0_, 8i32, 16777215i32) };
        { let a2_ = ((c.m.r16((sym::gStgGlare).wrapping_add(1204))) as i32); F::GfxPost_DrawDepthClut(c, 0i32, 7168i32, a2_, ((68i32) as u64)) };
        F::Dma_AddTexFlush(c);
        { let a2_ = ((((((((c.m.r32((sym::gStgGlare).wrapping_add(1236)) as i32)).wrapping_shl((16i32) as u32)) | (((c.m.r32((sym::gStgGlare).wrapping_add(1236)) as i32)).wrapping_shl((8i32) as u32)))) | ((c.m.r32((sym::gStgGlare).wrapping_add(1236)) as i32)))) as u32); F::GfxPost_DrawGlow(c, 10i32, 2155905152u32, a2_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgGlare_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn StgGlare_LoadParams(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgGlare_LoadParams", &PROBE_N, &[], false); }
    let mut l_param_1: u32 = 0;
    let r_ = 'f: {
    l_param_1 = F::BtlStage_GetList48(c);
    if (((((l_param_1) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = 64i32; let a_ = (sym::gStgGlare).wrapping_add(1268); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (sym::gStgGlare).wrapping_add(1264); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = (sym::gStgGlare).wrapping_add(1252); c.m.w32(a_, (v_) as u32); };
        { let v_ = 80i32; let a_ = (sym::gStgGlare).wrapping_add(1256); c.m.w32(a_, (v_) as u32); };
        { let v_ = 4i32; let a_ = (sym::gStgGlare).wrapping_add(1260); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (sym::gStgGlare).wrapping_add(1224); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (sym::gStgGlare).wrapping_add(1220); c.m.w32(a_, (v_) as u32); };
    } else {
        { let v_ = (((c.m.r32(l_param_1) as i32)) & (1i32)); let a_ = (sym::gStgGlare).wrapping_add(1220); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((((c.m.r32(l_param_1) as i32)).wrapping_shr((1i32) as u32)) & (1i32)); let a_ = (sym::gStgGlare).wrapping_add(1224); c.m.w32(a_, (v_) as u32); };
        { let v_ = (c.m.r32((l_param_1).wrapping_add(4)) as i32); let a_ = (sym::gStgGlare).wrapping_add(1264); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r8((l_param_1).wrapping_add(8))) as i32); let a_ = (sym::gStgGlare).wrapping_add(1252); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r8((l_param_1).wrapping_add(9))) as i32); let a_ = (sym::gStgGlare).wrapping_add(1256); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r8((l_param_1).wrapping_add(10))) as i32); let a_ = (sym::gStgGlare).wrapping_add(1260); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r8((l_param_1).wrapping_add(11))) as i32); let a_ = (sym::gStgGlare).wrapping_add(1268); c.m.w32(a_, (v_) as u32); };
    }
    { let a0_ = c.m.r32((sym::gStgGlare).wrapping_add(1168)); F::StgGlare_BuildClut(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgGlare_LoadParams", &PROBE_N, &[], true); }
    r_
}

pub fn StgDepthTint_BuildClut(c: &mut Ctx, mut p_clut_1: u32, mut p_tint_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgDepthTint_BuildClut", &PROBE_N, &[(p_clut_1) as u32, (p_tint_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(1040) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 1040);
    let mut l_i_3: i32 = 0;
    let mut l_k_4: i32 = 0;
    let mut l_r_5: F32 = F32(0);
    let mut l_g_6: F32 = F32(0);
    let mut l_b_7: F32 = F32(0);
    let mut l_a_8: F32 = F32(0);
    let mut l_k_9: i32 = 0;
    let r_ = 'f: {
        { let _ = F::memset(c, fp, 0i32, 1024u32); };
        F::StgCurve_Build(c, (p_tint_2).wrapping_add(1232), fp);
        if ((((((c.m.r32((p_tint_2).wrapping_add(1448)) as i32)) != (0i32)) as i32)) != 0) {
            {
                l_i_3 = 0i32;
                'l1: loop {
                    if !(((((l_i_3) < (256i32)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        l_k_4 = ((((((l_i_3) & (231i32))) | ((((l_i_3) & (8i32))).wrapping_shl((1i32) as u32)))) | ((((l_i_3) & (16i32))).wrapping_shr((1i32) as u32)));
                        { let r_ = 4i32; let n_ = (l_k_4).wrapping_mul(r_); l_k_4 = n_; };
                        if ((((((((((c.m.r32((p_tint_2).wrapping_add(1444)) as i32)) != (0i32)) as i32)) != 0) && (((((l_i_3) == (255i32)) as i32)) != 0)) as i32)) != 0) {
                            { let v_ = ((0i32) as u8); let a_ = (p_clut_1).wrapping_add((((l_k_4).wrapping_add(0i32)) as u32)); c.m.w8(a_, v_); };
                            { let v_ = ((0i32) as u8); let a_ = (p_clut_1).wrapping_add((((l_k_4).wrapping_add(1i32)) as u32)); c.m.w8(a_, v_); };
                            { let v_ = ((0i32) as u8); let a_ = (p_clut_1).wrapping_add((((l_k_4).wrapping_add(2i32)) as u32)); c.m.w8(a_, v_); };
                            { let v_ = ((0i32) as u8); let a_ = (p_clut_1).wrapping_add((((l_k_4).wrapping_add(3i32)) as u32)); c.m.w8(a_, v_); };
                        } else {
                            l_r_5 = F32(c.m.r32((p_tint_2).wrapping_add(1216)));
                            l_g_6 = F32(c.m.r32((p_tint_2).wrapping_add(1220)));
                            l_b_7 = F32(c.m.r32((p_tint_2).wrapping_add(1224)));
                            l_a_8 = F32(c.m.r32((p_tint_2).wrapping_add(1228)));
                            { let v_ = rt::f2i(rt::fmul(rt::fmul(rt::fmul(F32(c.m.r32((fp).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)))), l_r_5), F32(0x43000000)), l_a_8)); let a_ = (fp.wrapping_add(1024)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            { let v_ = (((c.m.r32((fp.wrapping_add(1024)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (p_clut_1).wrapping_add((((l_k_4).wrapping_add(0i32)) as u32)); c.m.w8(a_, v_); };
                            { let v_ = rt::f2i(rt::fmul(rt::fmul(rt::fmul(F32(c.m.r32((fp).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)))), l_g_6), F32(0x43000000)), l_a_8)); let a_ = (fp.wrapping_add(1024)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            { let v_ = (((c.m.r32((fp.wrapping_add(1024)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (p_clut_1).wrapping_add((((l_k_4).wrapping_add(1i32)) as u32)); c.m.w8(a_, v_); };
                            { let v_ = rt::f2i(rt::fmul(rt::fmul(rt::fmul(F32(c.m.r32((fp).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)))), l_b_7), F32(0x43000000)), l_a_8)); let a_ = (fp.wrapping_add(1024)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            { let v_ = (((c.m.r32((fp.wrapping_add(1024)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (p_clut_1).wrapping_add((((l_k_4).wrapping_add(2i32)) as u32)); c.m.w8(a_, v_); };
                            { let v_ = ((128i32) as u8); let a_ = (p_clut_1).wrapping_add((((l_k_4).wrapping_add(3i32)) as u32)); c.m.w8(a_, v_); };
                        }
                    }
                    { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                }
            }
        } else {
            {
                l_i_3 = 0i32;
                'l3: loop {
                    if !(((((l_i_3) < (256i32)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        l_k_9 = ((((((l_i_3) & (231i32))) | ((((l_i_3) & (8i32))).wrapping_shl((1i32) as u32)))) | ((((l_i_3) & (16i32))).wrapping_shr((1i32) as u32)));
                        { let r_ = 4i32; let n_ = (l_k_9).wrapping_mul(r_); l_k_9 = n_; };
                        if ((((((((((c.m.r32((p_tint_2).wrapping_add(1444)) as i32)) != (0i32)) as i32)) != 0) && (((((l_i_3) == (255i32)) as i32)) != 0)) as i32)) != 0) {
                            { let v_ = ((0i32) as u8); let a_ = (p_clut_1).wrapping_add((((l_k_9).wrapping_add(0i32)) as u32)); c.m.w8(a_, v_); };
                            { let v_ = ((0i32) as u8); let a_ = (p_clut_1).wrapping_add((((l_k_9).wrapping_add(1i32)) as u32)); c.m.w8(a_, v_); };
                            { let v_ = ((0i32) as u8); let a_ = (p_clut_1).wrapping_add((((l_k_9).wrapping_add(2i32)) as u32)); c.m.w8(a_, v_); };
                            { let v_ = ((0i32) as u8); let a_ = (p_clut_1).wrapping_add((((l_k_9).wrapping_add(3i32)) as u32)); c.m.w8(a_, v_); };
                        } else {
                            { let v_ = rt::f2i(rt::fmul(F32(c.m.r32((p_tint_2).wrapping_add(1216))), F32(0x437f0000))); let a_ = (fp.wrapping_add(1024)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            { let v_ = rt::f2i(rt::fmul(F32(c.m.r32((p_tint_2).wrapping_add(1220))), F32(0x437f0000))); let a_ = (fp.wrapping_add(1024)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            { let v_ = rt::f2i(rt::fmul(F32(c.m.r32((p_tint_2).wrapping_add(1224))), F32(0x437f0000))); let a_ = (fp.wrapping_add(1024)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            { let v_ = (((c.m.r32((fp.wrapping_add(1024)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (p_clut_1).wrapping_add((((l_k_9).wrapping_add(0i32)) as u32)); c.m.w8(a_, v_); };
                            { let v_ = (((c.m.r32((fp.wrapping_add(1024)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (p_clut_1).wrapping_add((((l_k_9).wrapping_add(1i32)) as u32)); c.m.w8(a_, v_); };
                            { let v_ = (((c.m.r32((fp.wrapping_add(1024)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (p_clut_1).wrapping_add((((l_k_9).wrapping_add(2i32)) as u32)); c.m.w8(a_, v_); };
                            { let v_ = ((rt::f2u(rt::fmul(rt::fmul(F32(c.m.r32((fp).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)))), F32(0x43000000)), F32(c.m.r32((p_tint_2).wrapping_add(1228)))))) as u8); let a_ = (p_clut_1).wrapping_add((((l_k_9).wrapping_add(3i32)) as u32)); c.m.w8(a_, v_); };
                        }
                    }
                    { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgDepthTint_BuildClut", &PROBE_N, &[], true); }
    r_
}

pub fn StgDepthTint_Rebuild(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgDepthTint_Rebuild", &PROBE_N, &[], false); }
    let mut l_tint_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_tint_1 = sym::gStgDepthTint;
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = c.m.r32((l_tint_1).wrapping_add(1168)); F::StgDepthTint_BuildClut(c, a0_, l_tint_1) };
            }
            { { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; }; { let o_ = l_tint_1; let n_ = (o_).wrapping_add(1472); l_tint_1 = n_; }; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgDepthTint_Rebuild", &PROBE_N, &[], true); }
    r_
}

pub fn StgDepthTint_Init(c: &mut Ctx, mut p_cbp_1: u16) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgDepthTint_Init", &PROBE_N, &[(p_cbp_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let _ = F::memset(c, (sym::gStgDepthTint).wrapping_add((((l_i_2) as u32)).wrapping_mul(1472)), 0i32, 1472u32); };
                F::GfxClut_InitPacket(c, (sym::gStgDepthTint).wrapping_add((((l_i_2) as u32)).wrapping_mul(1472)), (((((p_cbp_1) as i32)).wrapping_add(4i32)) as u16));
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgDepthTint_Init", &PROBE_N, &[], true); }
    r_
}

pub fn StgDepthTint_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgDepthTint_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgDepthTint_Term", &PROBE_N, &[], true); }
    r_
}

pub fn StgDepthTint_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgDepthTint_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_view_1: u32 = 0;
    let mut l_tint_2: u32 = 0;
    let mut l_medium_3: i32 = 0;
    let mut l_idx_4: i32 = 0;
    let r_ = 'f: {
        l_view_1 = c.m.r32(sym::gBtlCamView);
        l_medium_3 = 0i32;
        if ((((!((F::Battle_IsSplitScreen(c)) != 0)) as i32)) != 0) {
            if ((F::BtlStage_GetWaterLevel(c, fp.wrapping_add(64))) != 0) {
                F::Mtx_InverseRT(c, fp, (l_view_1).wrapping_add(64));
                l_medium_3 = 0i32;
                if (((rt::flt(F32(c.m.r32(fp.wrapping_add(64))), F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))) as i32)) != 0) {
                    l_medium_3 = 1i32;
                }
            } else {
                l_medium_3 = 0i32;
            }
        }
        l_tint_2 = (sym::gStgDepthTint).wrapping_add((((l_medium_3) as u32)).wrapping_mul(1472));
        if ((((((c.m.r32((l_tint_2).wrapping_add(1440)) as i32)) != (0i32)) as i32)) != 0) {
            F::Gfx_AddDefaultEnv(c);
            F::Dma_AddData(c, (l_tint_2).wrapping_add(1172), 16i32);
            F::Dma_AddTexFlush(c);
            F::Dma_AddZbuf(c, 224i32, 1i32);
            if ((((((c.m.r32((l_tint_2).wrapping_add(1448)) as i32)) != (0i32)) as i32)) != 0) {
                { let a2_ = ((c.m.r16((l_tint_2).wrapping_add(1204))) as i32); F::GfxPost_DrawDepthClut(c, 0i32, 7168i32, a2_, ((72i32) as u64)) };
            } else {
                { let a2_ = ((c.m.r16((l_tint_2).wrapping_add(1204))) as i32); F::GfxPost_DrawDepthClut(c, 0i32, 7168i32, a2_, ((68i32) as u64)) };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgDepthTint_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn StgDepthTint_LoadParams(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgDepthTint_LoadParams", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_tint_2: u32 = 0;
    let mut l_param_3: u32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_tint_2 = (sym::gStgDepthTint).wrapping_add((((l_i_1) as u32)).wrapping_mul(1472));
                l_param_3 = F::BtlStage_GetList40(c, l_i_1);
                if (((((l_param_3) == (((0i32) as u32))) as i32)) != 0) {
                    { let v_ = 1i32; let a_ = (l_tint_2).wrapping_add(1448); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (l_tint_2).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (l_tint_2).wrapping_add(1444); c.m.w32(a_, (v_) as u32); };
                    { let v_ = F32(0x3f000000); let a_ = (l_tint_2).wrapping_add(1216); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f000000); let a_ = (l_tint_2).wrapping_add(1220); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f000000); let a_ = (l_tint_2).wrapping_add(1224); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = (l_tint_2).wrapping_add(1228); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x00000000); let a_ = ((l_tint_2).wrapping_add(1232)).wrapping_add((((0i32) as u32)).wrapping_mul(64)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x00000000); let a_ = (((l_tint_2).wrapping_add(1232)).wrapping_add((((0i32) as u32)).wrapping_mul(64))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x00000000); let a_ = (((l_tint_2).wrapping_add(1232)).wrapping_add((((0i32) as u32)).wrapping_mul(64))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = (((l_tint_2).wrapping_add(1232)).wrapping_add((((0i32) as u32)).wrapping_mul(64))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x43000000); let a_ = ((l_tint_2).wrapping_add(1232)).wrapping_add((((1i32) as u32)).wrapping_mul(64)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x00000000); let a_ = (((l_tint_2).wrapping_add(1232)).wrapping_add((((1i32) as u32)).wrapping_mul(64))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x43000000); let a_ = (((l_tint_2).wrapping_add(1232)).wrapping_add((((1i32) as u32)).wrapping_mul(64))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = (((l_tint_2).wrapping_add(1232)).wrapping_add((((1i32) as u32)).wrapping_mul(64))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x437f0000); let a_ = ((l_tint_2).wrapping_add(1232)).wrapping_add((((2i32) as u32)).wrapping_mul(64)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x00000000); let a_ = (((l_tint_2).wrapping_add(1232)).wrapping_add((((2i32) as u32)).wrapping_mul(64))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x437f0000); let a_ = (((l_tint_2).wrapping_add(1232)).wrapping_add((((2i32) as u32)).wrapping_mul(64))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = (((l_tint_2).wrapping_add(1232)).wrapping_add((((2i32) as u32)).wrapping_mul(64))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                } else {
                    { let v_ = (((c.m.r32(l_param_3) as i32)) & (1i32)); let a_ = (l_tint_2).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((((c.m.r32(l_param_3) as i32)).wrapping_shr((1i32) as u32)) & (1i32)); let a_ = (l_tint_2).wrapping_add(1448); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((((c.m.r32(l_param_3) as i32)).wrapping_shr((2i32) as u32)) & (1i32)); let a_ = (l_tint_2).wrapping_add(1444); c.m.w32(a_, (v_) as u32); };
                    { let v_ = rt::fdiv(rt::i2f(((c.m.r8((l_param_3).wrapping_add(4))) as i32)), F32(0x437f0000)); let a_ = (l_tint_2).wrapping_add(1216); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fdiv(rt::i2f(((c.m.r8((l_param_3).wrapping_add(5))) as i32)), F32(0x437f0000)); let a_ = (l_tint_2).wrapping_add(1220); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fdiv(rt::i2f(((c.m.r8((l_param_3).wrapping_add(6))) as i32)), F32(0x437f0000)); let a_ = (l_tint_2).wrapping_add(1224); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fdiv(rt::i2f(((c.m.r8((l_param_3).wrapping_add(7))) as i32)), F32(0x437f0000)); let a_ = (l_tint_2).wrapping_add(1228); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::i2f((c.m.r8((((l_param_3).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(2))).wrapping_add(((0i32) as u32)))) as i32); let a_ = ((l_tint_2).wrapping_add(1232)).wrapping_add((((0i32) as u32)).wrapping_mul(64)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x00000000); let a_ = (((l_tint_2).wrapping_add(1232)).wrapping_add((((0i32) as u32)).wrapping_mul(64))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::i2f((c.m.r8((((l_param_3).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(2))).wrapping_add(((1i32) as u32)))) as i32); let a_ = (((l_tint_2).wrapping_add(1232)).wrapping_add((((0i32) as u32)).wrapping_mul(64))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = (((l_tint_2).wrapping_add(1232)).wrapping_add((((0i32) as u32)).wrapping_mul(64))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::i2f((c.m.r8((((l_param_3).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(2))).wrapping_add(((0i32) as u32)))) as i32); let a_ = ((l_tint_2).wrapping_add(1232)).wrapping_add((((1i32) as u32)).wrapping_mul(64)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x00000000); let a_ = (((l_tint_2).wrapping_add(1232)).wrapping_add((((1i32) as u32)).wrapping_mul(64))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::i2f((c.m.r8((((l_param_3).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(2))).wrapping_add(((1i32) as u32)))) as i32); let a_ = (((l_tint_2).wrapping_add(1232)).wrapping_add((((1i32) as u32)).wrapping_mul(64))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = (((l_tint_2).wrapping_add(1232)).wrapping_add((((1i32) as u32)).wrapping_mul(64))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::i2f((c.m.r8((((l_param_3).wrapping_add(8)).wrapping_add((((2i32) as u32)).wrapping_mul(2))).wrapping_add(((0i32) as u32)))) as i32); let a_ = ((l_tint_2).wrapping_add(1232)).wrapping_add((((2i32) as u32)).wrapping_mul(64)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x00000000); let a_ = (((l_tint_2).wrapping_add(1232)).wrapping_add((((2i32) as u32)).wrapping_mul(64))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::i2f((c.m.r8((((l_param_3).wrapping_add(8)).wrapping_add((((2i32) as u32)).wrapping_mul(2))).wrapping_add(((1i32) as u32)))) as i32); let a_ = (((l_tint_2).wrapping_add(1232)).wrapping_add((((2i32) as u32)).wrapping_mul(64))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = (((l_tint_2).wrapping_add(1232)).wrapping_add((((2i32) as u32)).wrapping_mul(64))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                }
                { let a0_ = c.m.r32((l_tint_2).wrapping_add(1168)); F::StgDepthTint_BuildClut(c, a0_, l_tint_2) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgDepthTint_LoadParams", &PROBE_N, &[], true); }
    r_
}

pub fn ObjGlow_BuildClut(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjGlow_BuildClut", &PROBE_N, &[], false); }
    let mut l_clut_1: u32 = 0;
    let mut l_alpha_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_e_4: u32 = 0;
    let r_ = 'f: {
    l_clut_1 = { let a_ = ({ let a_ = (c.m.r32(sym::gObjGlow)).wrapping_add(2496); c.m.r32(a_) }).wrapping_add(1168); c.m.r32(a_) };
    l_alpha_2 = (c.m.r32(sym::gObjGlow)).wrapping_add(2504);
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (256i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_e_4 = (l_clut_1).wrapping_add((((((((((l_i_3) & (231i32))) | ((((l_i_3) & (8i32))).wrapping_shl((1i32) as u32)))) | ((((l_i_3) & (16i32))).wrapping_shr((1i32) as u32)))).wrapping_mul(4i32)) as u32));
                { let v_ = ((255i32) as u8); let a_ = (l_e_4).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
                { let v_ = ((255i32) as u8); let a_ = (l_e_4).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
                { let v_ = ((255i32) as u8); let a_ = (l_e_4).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
                { let v_ = c.m.r8((l_alpha_2).wrapping_add(((l_i_3) as u32))); let a_ = (l_e_4).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ObjGlow_BuildClut", &PROBE_N, &[], true); }
    r_
}

pub fn ObjGlow_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjGlow_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, ((2816u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gObjGlow; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gObjGlow); F::memset(c, a0_, 0i32, 2816u32) }; };
    { let a0_ = ((c.m.r32(sym::gObjGlow)).wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(1216)); F::GfxClut_InitPacket(c, a0_, ((16016i32) as u16)) };
    { let a0_ = ((c.m.r32(sym::gObjGlow)).wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(1216)); F::GfxClut_InitPacket(c, a0_, ((16016i32) as u16)) };
    { let v_ = ((c.m.r32(sym::gObjGlow)).wrapping_add(64)).wrapping_add(((({ let a_ = (c.m.r32(sym::gObjGlow)).wrapping_add(2500); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1216)); let a_ = (c.m.r32(sym::gObjGlow)).wrapping_add(2496); c.m.w32(a_, v_); };
    { let r_ = 1i32; let a_ = (c.m.r32(sym::gObjGlow)).wrapping_add(2500); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
    { let v_ = ((c.m.r32(sym::gObjGlow)).wrapping_add(64)).wrapping_add(((({ let a_ = (c.m.r32(sym::gObjGlow)).wrapping_add(2500); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1216)); let a_ = (c.m.r32(sym::gObjGlow)).wrapping_add(2496); c.m.w32(a_, v_); };
    F::ObjGlow_BuildClut(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ObjGlow_Init", &PROBE_N, &[], true); }
    r_
}

pub fn ObjGlow_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjGlow_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32(sym::gObjGlow); F::Heap_Free(c, a0_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gObjGlow; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ObjGlow_Term", &PROBE_N, &[], true); }
    r_
}

pub fn ObjGlow_SetAlphaTable(c: &mut Ctx, mut p_table_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjGlow_SetAlphaTable", &PROBE_N, &[(p_table_1) as u32], false); }
    let r_ = 'f: {
    { let s_ = p_table_1; let d_ = (c.m.r32(sym::gObjGlow)).wrapping_add(2504); rt::memcpy(c, d_, s_, 256); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ObjGlow_SetAlphaTable", &PROBE_N, &[], true); }
    r_
}

pub fn ObjGlow_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjGlow_Draw", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::ObjGlow_BuildClut(c);
    { let a0_ = ({ let a_ = (c.m.r32(sym::gObjGlow)).wrapping_add(2496); c.m.r32(a_) }).wrapping_add(1172); F::Dma_AddData(c, a0_, 16i32) };
    F::Dma_AddTexFlush(c);
    { let a0_ = (if (((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0) { 0i32 } else { 112i32 }); F::Dma_AddFrame(c, a0_, 8i32, 16777215i32) };
    { let a2_ = (({ let a_ = ({ let a_ = (c.m.r32(sym::gObjGlow)).wrapping_add(2496); c.m.r32(a_) }).wrapping_add(1204); c.m.r16(a_) }) as i32); F::GfxPost_DrawDepthClut(c, 0i32, 7168i32, a2_, ((68i32) as u64)) };
    { let a0_ = (if (((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0) { 0i32 } else { 112i32 }); F::Dma_AddFrame(c, a0_, 8i32, 0i32) };
    F::GfxPost_DrawGlow(c, 10i32, 2155905152u32, 2155905152u32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ObjGlow_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn GfxPost_InitNop(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxPost_InitNop", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "GfxPost_InitNop", &PROBE_N, &[], true); }
    r_
}

pub fn GfxPost_TermNop(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxPost_TermNop", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "GfxPost_TermNop", &PROBE_N, &[], true); }
    r_
}

pub fn GfxPost_Nop106C70(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxPost_Nop106C70", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "GfxPost_Nop106C70", &PROBE_N, &[], true); }
    r_
}

pub fn GfxPost_Stub106C78(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxPost_Stub106C78", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "GfxPost_Stub106C78", &PROBE_N, &[], true); }
    r_
}

pub fn GfxPostQuad_SetColor(c: &mut Ctx, mut p_q_1: u32, mut p_r_2: i32, mut p_g_3: i32, mut p_b_4: i32, mut p_a_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxPostQuad_SetColor", &PROBE_N, &[(p_q_1) as u32, (p_r_2) as u32, (p_g_3) as u32, (p_b_4) as u32, (p_a_5) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_r_2; let a_ = p_q_1; c.m.w32(a_, (v_) as u32); };
    { let v_ = p_g_3; let a_ = (p_q_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_b_4; let a_ = (p_q_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_a_5; let a_ = (p_q_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "GfxPostQuad_SetColor", &PROBE_N, &[], true); }
    r_
}

pub fn GfxPostQuad_Set(c: &mut Ctx, mut p_q_1: u32, mut p_baseX_2: i32, mut p_baseY_3: i32, mut p_u0_4: i16, mut p_v0_5: i16, mut p_u1_6: i16, mut p_v1_7: i16, mut p_dx_8: i32, mut p_dy_9: i32, mut p_w_10: i32, mut p_h_11: i32, mut p_half_12: i16) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxPostQuad_Set", &PROBE_N, &[(p_q_1) as u32, (p_baseX_2) as u32, (p_baseY_3) as u32, (p_u0_4) as u32, (p_v0_5) as u32, (p_u1_6) as u32, (p_v1_7) as u32, (p_dx_8) as u32, (p_dy_9) as u32, (p_w_10) as u32, (p_h_11) as u32, (p_half_12) as u32], false); }
    let r_ = 'f: {
    { let v_ = (((p_u0_4) as i32)).wrapping_shl((4i32) as u32); let a_ = (p_q_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
    { let v_ = (((p_v0_5) as i32)).wrapping_shl((4i32) as u32); let a_ = (p_q_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
    { let v_ = (((p_u1_6) as i32)).wrapping_shl((4i32) as u32); let a_ = (p_q_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
    { let v_ = (((p_v1_7) as i32)).wrapping_shl((4i32) as u32); let a_ = (p_q_1).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
    if (((((((p_half_12) as i32)) != (0i32)) as i32)) != 0) {
        { let v_ = ((((p_u0_4) as i32)).wrapping_shl((4i32) as u32)).wrapping_add(8i32); let a_ = (p_q_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((((p_u1_6) as i32)).wrapping_shl((4i32) as u32)).wrapping_add(8i32); let a_ = (p_q_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((((p_v0_5) as i32)).wrapping_shl((4i32) as u32)).wrapping_add(8i32); let a_ = (p_q_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((((p_v1_7) as i32)).wrapping_shl((4i32) as u32)).wrapping_add(8i32); let a_ = (p_q_1).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
    }
    { let v_ = (p_dx_8).wrapping_shl((4i32) as u32); let a_ = (p_q_1).wrapping_add(56); c.m.w32(a_, (v_) as u32); };
    { let v_ = (p_dy_9).wrapping_shl((4i32) as u32); let a_ = (p_q_1).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
    { let v_ = (p_w_10).wrapping_shl((4i32) as u32); let a_ = (p_q_1).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
    { let v_ = (p_h_11).wrapping_shl((4i32) as u32); let a_ = (p_q_1).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((p_baseX_2).wrapping_shl((4i32) as u32)).wrapping_add((p_dx_8).wrapping_shl((4i32) as u32)); let a_ = (p_q_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((p_baseY_3).wrapping_shl((4i32) as u32)).wrapping_add((p_dy_9).wrapping_shl((4i32) as u32)); let a_ = (p_q_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = (((p_baseX_2).wrapping_shl((4i32) as u32)).wrapping_add((p_dx_8).wrapping_shl((4i32) as u32))).wrapping_add((p_w_10).wrapping_shl((4i32) as u32)); let a_ = (p_q_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
    { let v_ = (((p_baseY_3).wrapping_shl((4i32) as u32)).wrapping_add((p_dy_9).wrapping_shl((4i32) as u32))).wrapping_add((p_h_11).wrapping_shl((4i32) as u32)); let a_ = (p_q_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
    { let v_ = 128i32; let a_ = p_q_1; c.m.w32(a_, (v_) as u32); };
    { let v_ = 128i32; let a_ = (p_q_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = 128i32; let a_ = (p_q_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = 128i32; let a_ = (p_q_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "GfxPostQuad_Set", &PROBE_N, &[], true); }
    r_
}
