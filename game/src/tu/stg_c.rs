#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn StgFog_SetColor(c: &mut Ctx, mut p_r_1: i32, mut p_g_2: i32, mut p_b_3: i32, mut p_a_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgFog_SetColor", &PROBE_N, &[(p_r_1) as u32, (p_g_2) as u32, (p_b_3) as u32, (p_a_4) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((((((((p_r_1) | ((p_g_2).wrapping_shl((8i32) as u32)))) | ((p_b_3).wrapping_shl((16i32) as u32)))) | ((p_a_4).wrapping_shl((24i32) as u32)))) as u32); let a_ = (c.m.r32(sym::gStgFog)).wrapping_add(1488); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgFog_SetColor", &PROBE_N, &[], true); }
    r_
}

pub fn StgFog_GetCurve(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgFog_GetCurve", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (c.m.r32(sym::gStgFog)).wrapping_add(1280);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "StgFog_GetCurve", &PROBE_N, &[], true); }
    r_
}

pub fn StgFog_RebuildClut(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgFog_RebuildClut", &PROBE_N, &[], false); }
    let mut l_fog_1: u32 = 0;
    let r_ = 'f: {
    l_fog_1 = c.m.r32(sym::gStgFog);
    { let a2_ = (c.m.r32((l_fog_1).wrapping_add(1492)) as i32); F::StgFog_BuildClut(c, l_fog_1, (l_fog_1).wrapping_add(1280), a2_, F32(0x3f800000)) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgFog_RebuildClut", &PROBE_N, &[], true); }
    r_
}

pub fn StgCurve_AddSegment(c: &mut Ctx, mut p_curve_1: u32, mut p_table_2: u32, mut p_k0_3: u32, mut p_k1_4: u32, mut p_steps_5: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgCurve_AddSegment", &PROBE_N, &[(p_curve_1) as u32, (p_table_2) as u32, (p_k0_3) as u32, (p_k1_4) as u32, (p_steps_5) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_i_6: i32 = 0;
    let r_ = 'f: {
        F::Vec4_Copy(c, fp, p_k0_3);
        F::Vec4_Copy(c, fp.wrapping_add(16), p_k1_4);
        F::Vec4_Copy(c, fp.wrapping_add(32), (p_k0_3).wrapping_add(16));
        F::Vec4_Copy(c, fp.wrapping_add(48), (p_k1_4).wrapping_add(32));
        { let v_ = rt::fneg(F32(c.m.r32(fp.wrapping_add(32)))); let a_ = fp.wrapping_add(32); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4)))); let a_ = (fp.wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8)))); let a_ = (fp.wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        {
            l_i_6 = 0i32;
            'l1: loop {
                if !(((((l_i_6) <= (p_steps_5)) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::Vec3_Hermite(c, fp.wrapping_add(64), fp, fp.wrapping_add(16), fp.wrapping_add(32), fp.wrapping_add(48), rt::fdiv(rt::i2f(l_i_6), rt::i2f(p_steps_5)));
                    if (((rt::flt(F32(c.m.r32(fp.wrapping_add(64))), F32(0x00000000)) as i32)) != 0) {
                        { let v_ = F32(0x00000000); let a_ = fp.wrapping_add(64); c.m.w32(a_, (v_).0); };
                    }
                    if (((rt::fgt(F32(c.m.r32(fp.wrapping_add(64))), F32(0x437f0000)) as i32)) != 0) {
                        { let v_ = F32(0x437f0000); let a_ = fp.wrapping_add(64); c.m.w32(a_, (v_).0); };
                    }
                    if (((rt::flt(F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(8))), F32(0x00000000)) as i32)) != 0) {
                        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(64)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    }
                    if (((rt::fgt(F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(8))), F32(0x437f0000)) as i32)) != 0) {
                        { let v_ = F32(0x437f0000); let a_ = (fp.wrapping_add(64)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    }
                    { let v_ = rt::fdiv(F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(8))), F32(0x437f0000)); let a_ = (p_table_2).wrapping_add(((((c.m.r32((p_curve_1).wrapping_add(192)) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    if ((((((c.m.r32((p_curve_1).wrapping_add(192)) as i32)) < (255i32)) as i32)) != 0) {
                        { let a_ = (p_curve_1).wrapping_add(192); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    }
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
        break 'f F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(8)));
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgCurve_AddSegment", &PROBE_N, &[], true); }
    r_
}

pub fn StgCurve_Build(c: &mut Ctx, mut p_curve_1: u32, mut p_table_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgCurve_Build", &PROBE_N, &[(p_curve_1) as u32, (p_table_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_i_3: i32 = 0;
    let mut l_last_4: F32 = F32(0);
    let mut l_key_5: u32 = 0;
    let mut l_next_6: u32 = 0;
    let mut l_prev_7: u32 = 0;
    let mut l_steps_8: i32 = 0;
    let r_ = 'f: {
        l_key_5 = p_curve_1;
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < (3i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((l_i_3) != (0i32)) as i32)) != 0) {
                        if (((((l_i_3) != (2i32)) as i32)) != 0) {
                            l_next_6 = (p_curve_1).wrapping_add(((((l_i_3).wrapping_add(1i32)) as u32)).wrapping_mul(64));
                            l_prev_7 = (p_curve_1).wrapping_add(((((l_i_3).wrapping_sub(1i32)) as u32)).wrapping_mul(64));
                            F::Vec4_Sub(c, fp, l_next_6, (p_curve_1).wrapping_add((((l_i_3) as u32)).wrapping_mul(64)));
                            F::Vec4_Sub(c, fp.wrapping_add(16), l_next_6, l_prev_7);
                            F::Vec3_Normalize(c, fp.wrapping_add(16), fp.wrapping_add(16));
                            { let a2_ = F::Vec3_Length(c, fp); F::Vec3_Scale(c, fp.wrapping_add(16), fp.wrapping_add(16), a2_) };
                            F::Vec4_Copy(c, ((p_curve_1).wrapping_add((((l_i_3) as u32)).wrapping_mul(64))).wrapping_add(32), fp.wrapping_add(16));
                            F::Vec4_Sub(c, fp, l_prev_7, (p_curve_1).wrapping_add((((l_i_3) as u32)).wrapping_mul(64)));
                            F::Vec4_Sub(c, fp.wrapping_add(16), l_prev_7, l_next_6);
                            F::Vec3_Normalize(c, fp.wrapping_add(16), fp.wrapping_add(16));
                            { let a2_ = F::Vec3_Length(c, fp); F::Vec3_Scale(c, fp.wrapping_add(16), fp.wrapping_add(16), a2_) };
                            F::Vec4_Copy(c, ((p_curve_1).wrapping_add((((l_i_3) as u32)).wrapping_mul(64))).wrapping_add(16), fp.wrapping_add(16));
                        }
                    }
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        { let v_ = 0i32; let a_ = (p_curve_1).wrapping_add(192); c.m.w32(a_, (v_) as u32); };
        {
            l_i_3 = 0i32;
            'l3: loop {
                if !(((((l_i_3) < (rt::f2i(F32(c.m.r32((p_curve_1).wrapping_add((((0i32) as u32)).wrapping_mul(64))))))) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let v_ = F32(0x00000000); let a_ = (p_table_2).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let a_ = (p_curve_1).wrapping_add(192); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        {
            l_i_3 = 0i32;
            'l5: loop {
                if !(((((l_i_3) < (2i32)) as i32)) != 0) { break 'l5; }
                'c6: {
                    if (((((l_i_3) == (0i32)) as i32)) != 0) {
                        l_steps_8 = (rt::f2i(F32(c.m.r32((p_curve_1).wrapping_add((((1i32) as u32)).wrapping_mul(64)))))).wrapping_sub(rt::f2i(F32(c.m.r32((p_curve_1).wrapping_add((((0i32) as u32)).wrapping_mul(64))))));
                    } else {
                        l_steps_8 = (rt::f2i(F32(c.m.r32((p_curve_1).wrapping_add((((2i32) as u32)).wrapping_mul(64)))))).wrapping_sub(rt::f2i(F32(c.m.r32((p_curve_1).wrapping_add((((1i32) as u32)).wrapping_mul(64))))));
                    }
                    l_last_4 = F::StgCurve_AddSegment(c, p_curve_1, p_table_2, (p_curve_1).wrapping_add((((l_i_3) as u32)).wrapping_mul(64)), (p_curve_1).wrapping_add(((((l_i_3).wrapping_add(1i32)) as u32)).wrapping_mul(64)), l_steps_8);
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        {
            l_i_3 = (c.m.r32((p_curve_1).wrapping_add(192)) as i32);
            'l7: loop {
                if !(((((l_i_3) < (256i32)) as i32)) != 0) { break 'l7; }
                'c8: {
                    { let v_ = rt::fdiv(l_last_4, F32(0x437f0000)); let a_ = (p_table_2).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgCurve_Build", &PROBE_N, &[], true); }
    r_
}

pub fn StgHaze_BeginDraw(c: &mut Ctx, mut p_d_1: u32, mut p_haze_2: u32, mut p_x0_3: i32, mut p_x1_4: i32, mut p_width_5: i32, mut p_h_6: i32, mut p_tmpW_7: i32, mut p_tmpH_8: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_BeginDraw", &PROBE_N, &[(p_d_1) as u32, (p_haze_2) as u32, (p_x0_3) as u32, (p_x1_4) as u32, (p_width_5) as u32, (p_h_6) as u32, (p_tmpW_7) as u32, (p_tmpH_8) as u32], false); }
    let r_ = 'f: {
    F::Dma_AddData(c, (p_haze_2).wrapping_add(1172), 16i32);
    F::Dma_AddTexFlush(c);
    { let a0_ = (if (((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0) { 0i32 } else { 112i32 }); F::Dma_AddFrame(c, a0_, 8i32, 16777215i32) };
    F::Dma_AddZbuf(c, 224i32, 1i32);
    { let a2_ = ((c.m.r16((p_haze_2).wrapping_add(1204))) as i32); F::GfxPost_DrawDepthClut(c, 0i32, 7168i32, a2_, ((68i32) as u64)) };
    F::Dma_AddZbuf(c, 224i32, 0i32);
    { let v_ = F::Dma_BeginDirect(c); let a_ = p_d_1; c.m.w32(a_, v_); };
    F::Gfx_PutDefaultEnv(c, p_d_1);
    { let v_ = ((((((((6i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (c.m.r32(p_d_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_d_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = ((196608i32) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((71i32) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_d_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = ((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (c.m.r32(p_d_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((78i32) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_d_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { 10267790848i64 } else { 10267787264i64 })) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((6i32) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_d_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = ((524624i32) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((76i32) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_d_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = ((((((((0i32) as u64)) | (((((p_tmpW_7).wrapping_sub(1i32)) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) | (((((p_tmpH_8).wrapping_sub(1i32)) as u64)).wrapping_shl((48i32) as u32))); let a_ = (c.m.r32(p_d_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((64i32) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_d_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = ((((((((((((2i32) as u64)) | ((((2i32) as u64)).wrapping_shl((2i32) as u32)))) | ((((p_x0_3) as u64)).wrapping_shl((4i32) as u32)))) | (((((p_x1_4).wrapping_sub(1i32)) as u64)).wrapping_shl((14i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | (((((p_h_6).wrapping_sub(1i32)) as u64)).wrapping_shl((34i32) as u32))); let a_ = (c.m.r32(p_d_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((8i32) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_d_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    F::Dma_PutTexStrips(c, p_d_1, 0i32, 0i32, p_tmpW_7, p_tmpH_8, 0i32, 0i32, p_x0_3, 0i32, (p_width_5).wrapping_add(p_x0_3), p_h_6, 0i32, 0i32, 2155905152u32, 0i32);
    { let v_ = ((((((((5i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (c.m.r32(p_d_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_d_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = ((10267798016i64) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((6i32) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_d_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { 524400i32 } else { 524288i32 })) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((76i32) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_d_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = ((((((((p_x0_3) as u64)) | (((((p_x1_4).wrapping_sub(1i32)) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) | (((((p_h_6).wrapping_sub(1i32)) as u64)).wrapping_shl((48i32) as u32))); let a_ = (c.m.r32(p_d_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((64i32) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_d_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = ((((((((((((2i32) as u64)) | ((((2i32) as u64)).wrapping_shl((2i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((4i32) as u32)))) | (((((p_tmpW_7).wrapping_sub(1i32)) as u64)).wrapping_shl((14i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | (((((p_tmpH_8).wrapping_sub(1i32)) as u64)).wrapping_shl((34i32) as u32))); let a_ = (c.m.r32(p_d_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((8i32) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_d_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = ((84i32) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((66i32) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_d_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_BeginDraw", &PROBE_N, &[], true); }
    r_
}

pub fn StgHaze_EndDraw(c: &mut Ctx, mut p_d_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_EndDraw", &PROBE_N, &[(p_d_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32(p_d_1); F::Dma_EndDirect(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_EndDraw", &PROBE_N, &[], true); }
    r_
}

pub fn StgHaze_PutStripTag(c: &mut Ctx, mut p_d_1: u32, mut p_haze_2: u32, mut p_nVerts_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_PutStripTag", &PROBE_N, &[(p_d_1) as u32, (p_haze_2) as u32, (p_nVerts_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((2594073385365405697i64) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((16i32) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_d_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = ((340i32) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((((c.m.r32((p_haze_2).wrapping_add(5232))) as u64)) | ((((65024i32) as u64)).wrapping_shl((46i32) as u32))); let a_ = (c.m.r32(p_d_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_d_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = ((((p_nVerts_3) as u64)) | ((((36864i32) as u64)).wrapping_shl((46i32) as u32))); let a_ = (c.m.r32(p_d_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((83i32) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_d_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_PutStripTag", &PROBE_N, &[], true); }
    r_
}

pub fn StgHaze_EndStrip(c: &mut Ctx, mut p_d_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_EndStrip", &PROBE_N, &[(p_d_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_EndStrip", &PROBE_N, &[], true); }
    r_
}

pub fn StgHaze_PutVertex(c: &mut Ctx, mut p_d_1: u32, mut p_u_2: i32, mut p_v_3: i32, mut p_x_4: i32, mut p_y_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_PutVertex", &PROBE_N, &[(p_d_1) as u32, (p_u_2) as u32, (p_v_3) as u32, (p_x_4) as u32, (p_y_5) as u32], false); }
    let r_ = 'f: {
    { let v_ = (((((p_x_4).wrapping_add(8i32)) as u64)) | (((((p_y_5).wrapping_add(8i32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (c.m.r32(p_d_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((((((p_u_2).wrapping_shl((4i32) as u32)).wrapping_add(28672i32)) as u64)) | ((((((p_v_3).wrapping_shl((4i32) as u32)).wrapping_add(29184i32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (c.m.r32(p_d_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_d_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_PutVertex", &PROBE_N, &[], true); }
    r_
}

pub fn StgHaze_Create(c: &mut Ctx, mut p_haze_1: u32, mut p_clutBase_2: u16) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_Create", &PROBE_N, &[(p_haze_1) as u32, (p_clutBase_2) as u32], false); }
    let r_ = 'f: {
    F::GfxClut_InitPacket(c, p_haze_1, p_clutBase_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_Create", &PROBE_N, &[], true); }
    r_
}

pub fn StgHaze_BuildClut(c: &mut Ctx, mut p_haze_1: u32, mut p_curve_2: u32, mut p_scale_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_BuildClut", &PROBE_N, &[(p_haze_1) as u32, (p_curve_2) as u32, (p_scale_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(1024) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 1024);
    let mut l_clut_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_k_6: i32 = 0;
    let r_ = 'f: {
        l_clut_4 = c.m.r32((p_haze_1).wrapping_add(1168));
        { let _ = F::memset(c, fp, 0i32, 1024u32); };
        F::StgCurve_Build(c, p_curve_2, fp);
        {
            l_i_5 = 0i32;
            'l1: loop {
                if !(((((l_i_5) < (256i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_k_6 = ((((((l_i_5) & (231i32))) | ((((l_i_5) & (8i32))).wrapping_shl((1i32) as u32)))) | ((((l_i_5) & (16i32))).wrapping_shr((1i32) as u32)));
                    { let r_ = 4i32; let n_ = (l_k_6).wrapping_mul(r_); l_k_6 = n_; };
                    { let v_ = ((0i32) as u8); let a_ = (l_clut_4).wrapping_add((((l_k_6).wrapping_add(0i32)) as u32)); c.m.w8(a_, v_); };
                    { let v_ = ((0i32) as u8); let a_ = (l_clut_4).wrapping_add((((l_k_6).wrapping_add(1i32)) as u32)); c.m.w8(a_, v_); };
                    { let v_ = ((0i32) as u8); let a_ = (l_clut_4).wrapping_add((((l_k_6).wrapping_add(2i32)) as u32)); c.m.w8(a_, v_); };
                    { let v_ = ((rt::f2u(rt::fmul(rt::fmul(F32(c.m.r32((fp).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)))), F32(0x43000000)), p_scale_3))) as u8); let a_ = (l_clut_4).wrapping_add((((l_k_6).wrapping_add(3i32)) as u32)); c.m.w8(a_, v_); };
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_BuildClut", &PROBE_N, &[], true); }
    r_
}

pub fn StgHaze_Init(c: &mut Ctx, mut p_clutBase_1: u16) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_Init", &PROBE_N, &[(p_clutBase_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, (((5504u32).wrapping_mul(((2i32) as u32))) as i32), ((64i32) as u32), 0i32, 2i32); let a_ = sym::gStgHaze; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gStgHaze); F::memset(c, a0_, 0i32, (5504u32).wrapping_mul(((2i32) as u32))) }; };
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = (c.m.r32(sym::gStgHaze)).wrapping_add((((l_i_2) as u32)).wrapping_mul(5504)); F::StgHaze_Create(c, a0_, p_clutBase_1) };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    F::StgHaze_Reset(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_Init", &PROBE_N, &[], true); }
    r_
}

pub fn StgHaze_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gStgHaze)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gStgHaze); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gStgHaze; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_Term", &PROBE_N, &[], true); }
    r_
}

pub fn StgHaze_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_Reset", &PROBE_N, &[], false); }
    let mut l_haze_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_haze_1 = c.m.r32(sym::gStgHaze);
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = 20i32; let a_ = (l_haze_1).wrapping_add(5216); c.m.w32(a_, (v_) as u32); };
                { let v_ = 50i32; let a_ = (l_haze_1).wrapping_add(5220); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_haze_1).wrapping_add(5224); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_haze_1).wrapping_add(5228); c.m.w32(a_, (v_) as u32); };
                { let v_ = 2155905152u32; let a_ = (l_haze_1).wrapping_add(5232); c.m.w32(a_, v_); };
                { let v_ = 1i32; let a_ = (l_haze_1).wrapping_add(5236); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_haze_1).wrapping_add(5240); c.m.w32(a_, (v_) as u32); };
                { let v_ = F32(0x3dcccccc); let a_ = (l_haze_1).wrapping_add(5244); c.m.w32(a_, (v_).0); };
            }
            { { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; }; { let o_ = l_haze_1; let n_ = (o_).wrapping_add(5504); l_haze_1 = n_; }; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn StgHaze_UpdateClut(c: &mut Ctx, mut p_haze_1: u32, mut p_split_2: i32, mut p_view_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_UpdateClut", &PROBE_N, &[(p_haze_1) as u32, (p_split_2) as u32, (p_view_3) as u32], false); }
    let mut l_y_4: F32 = F32(0);
    let mut l_top_5: F32 = F32(0);
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlCam)) == (((0i32) as u32))) as i32)) != 0) {
        l_y_4 = { let a_ = ((c.m.r32(sym::gBtlCamView)).wrapping_add(544)).wrapping_add(4); F32(c.m.r32(a_)) };
    } else {
        l_y_4 = { let a_ = ((((c.m.r32(sym::gBtlCam)).wrapping_add(1824)).wrapping_add((((p_view_3) as u32)).wrapping_mul(656))).wrapping_add(608)).wrapping_add(4); F32(c.m.r32(a_)) };
    }
    l_top_5 = rt::fneg(F::BtlStage_GetTop(c));
    F::StgHaze_BuildClut(c, p_haze_1, (p_haze_1).wrapping_add(5248), rt::fsub(F32(0x3f800000), rt::fmul(rt::fdiv(rt::fneg(l_y_4), l_top_5), F32(0x3f000000))));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_UpdateClut", &PROBE_N, &[], true); }
    r_
}

pub fn StgHaze_Step(c: &mut Ctx, mut p_haze_1: u32, mut p_split_2: i32, mut p_view_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_Step", &PROBE_N, &[(p_haze_1) as u32, (p_split_2) as u32, (p_view_3) as u32], false); }
    let mut l_cols_4: i32 = 0;
    let mut l_rows_5: i32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_j_7: i32 = 0;
    let mut l_tx_8: u32 = 0;
    let mut l_ty_9: u32 = 0;
    let mut l_cx_10: u32 = 0;
    let mut l_cy_11: u32 = 0;
    let r_ = 'f: {
    l_cols_4 = (c.m.r32((p_haze_1).wrapping_add(5216)) as i32);
    l_rows_5 = (c.m.r32((p_haze_1).wrapping_add(5220)) as i32);
    if ((p_split_2) != 0) {
        { let r_ = 2i32; let n_ = rt::div_i32(l_cols_4, r_); l_cols_4 = n_; };
    }
    if (((((rt::rem_i32((c.m.r32((p_haze_1).wrapping_add(5240)) as i32), (c.m.r32((p_haze_1).wrapping_add(5236)) as i32))) == (0i32)) as i32)) != 0) {
        {
            l_i_6 = 0i32;
            'l1: loop {
                if !(((((l_i_6) < (l_rows_5)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_tx_8 = ((p_haze_1).wrapping_add(1216)).wrapping_add((((l_i_6) as u32)).wrapping_mul(20));
                    l_ty_9 = ((p_haze_1).wrapping_add(2216)).wrapping_add((((l_i_6) as u32)).wrapping_mul(20));
                    {
                        l_j_7 = 0i32;
                        'l3: loop {
                            if !(((((l_j_7) < (l_cols_4)) as i32)) != 0) { break 'l3; }
                            'c4: {
                                { let v_ = (({ let a0_ = ((c.m.r32((p_haze_1).wrapping_add(5224)) as i32)).wrapping_neg(); let a1_ = (c.m.r32((p_haze_1).wrapping_add(5224)) as i32); F::Rand_IntRange(c, a0_, a1_) }) as i8); let a_ = { let o_ = l_tx_8; let n_ = (o_).wrapping_add(1); l_tx_8 = n_; o_ }; c.m.w8(a_, (v_) as u8); };
                                { let v_ = (({ let a0_ = ((c.m.r32((p_haze_1).wrapping_add(5228)) as i32)).wrapping_neg(); let a1_ = (c.m.r32((p_haze_1).wrapping_add(5228)) as i32); F::Rand_IntRange(c, a0_, a1_) }) as i8); let a_ = { let o_ = l_ty_9; let n_ = (o_).wrapping_add(1); l_ty_9 = n_; o_ }; c.m.w8(a_, (v_) as u8); };
                            }
                            { let o_ = l_j_7; let n_ = (o_).wrapping_add(1); l_j_7 = n_; };
                        }
                    }
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
    }
    {
        l_i_6 = 0i32;
        'l5: loop {
            if !(((((l_i_6) < (l_rows_5)) as i32)) != 0) { break 'l5; }
            'c6: {
                l_tx_8 = ((p_haze_1).wrapping_add(1216)).wrapping_add((((l_i_6) as u32)).wrapping_mul(20));
                l_ty_9 = ((p_haze_1).wrapping_add(2216)).wrapping_add((((l_i_6) as u32)).wrapping_mul(20));
                l_cx_10 = ((p_haze_1).wrapping_add(3216)).wrapping_add((((l_i_6) as u32)).wrapping_mul(20));
                l_cy_11 = ((p_haze_1).wrapping_add(4216)).wrapping_add((((l_i_6) as u32)).wrapping_mul(20));
                {
                    l_j_7 = 0i32;
                    'l7: loop {
                        if !(((((l_j_7) < (l_cols_4)) as i32)) != 0) { break 'l7; }
                        'c8: {
                            { let r_ = rt::f2i(rt::fmul(rt::i2f(((((c.m.r8(l_tx_8) as i8)) as i32)).wrapping_sub((((c.m.r8(l_cx_10) as i8)) as i32))), F32(c.m.r32((p_haze_1).wrapping_add(5244))))); let a_ = l_cx_10; let n_ = ((((((c.m.r8(a_) as i8)) as i32)).wrapping_add(r_)) as i8); c.m.w8(a_, (n_) as u8); };
                            { let o_ = l_tx_8; let n_ = (o_).wrapping_add(1); l_tx_8 = n_; };
                            { let o_ = l_cx_10; let n_ = (o_).wrapping_add(1); l_cx_10 = n_; };
                            { let r_ = rt::f2i(rt::fmul(rt::i2f(((((c.m.r8(l_ty_9) as i8)) as i32)).wrapping_sub((((c.m.r8(l_cy_11) as i8)) as i32))), F32(c.m.r32((p_haze_1).wrapping_add(5244))))); let a_ = l_cy_11; let n_ = ((((((c.m.r8(a_) as i8)) as i32)).wrapping_add(r_)) as i8); c.m.w8(a_, (n_) as u8); };
                            { let o_ = l_ty_9; let n_ = (o_).wrapping_add(1); l_ty_9 = n_; };
                            { let o_ = l_cy_11; let n_ = (o_).wrapping_add(1); l_cy_11 = n_; };
                        }
                        { let o_ = l_j_7; let n_ = (o_).wrapping_add(1); l_j_7 = n_; };
                    }
                }
            }
            { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
        }
    }
    { let a_ = (p_haze_1).wrapping_add(5240); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_Step", &PROBE_N, &[], true); }
    r_
}

pub fn StgHaze_Draw(c: &mut Ctx, mut p_haze_1: u32, mut p_split_2: i32, mut p_view_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_Draw", &PROBE_N, &[(p_haze_1) as u32, (p_split_2) as u32, (p_view_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_width_4: i32 = 0;
    let mut l_widthPx_5: i32 = 0;
    let mut l_xOffset_6: i32 = 0;
    let mut l_cols_7: i32 = 0;
    let mut l_y0_8: i32 = 0;
    let mut l_y1_9: i32 = 0;
    let mut l_v0_10: i32 = 0;
    let mut l_v1_11: i32 = 0;
    let mut l_x1_12: u16 = 0;
    let mut l_rows_13: i32 = 0;
    let mut l_row_14: i32 = 0;
    let mut l_col_15: i32 = 0;
    let mut l_px_16: i32 = 0;
    let mut l_u_17: i32 = 0;
    let mut l_x_18: i32 = 0;
    let mut l_y_19: i32 = 0;
    let mut l_h_20: i32 = 0;
    let mut l_srcH_21: i32 = 0;
    let mut l_cx_22: u32 = 0;
    let mut l_cy_23: u32 = 0;
    let r_ = 'f: {
        l_h_20 = 224i32;
        l_srcH_21 = 448i32;
        l_width_4 = 512i32;
        l_widthPx_5 = 512i32;
        l_xOffset_6 = 0i32;
        l_cols_7 = (c.m.r32((p_haze_1).wrapping_add(5216)) as i32);
        l_rows_13 = (c.m.r32((p_haze_1).wrapping_add(5220)) as i32);
        l_x1_12 = ((512i32) as u16);
        if ((p_split_2) != 0) {
            l_width_4 = 256i32;
            l_xOffset_6 = (if (((((p_view_3) == (1i32)) as i32)) != 0) { 256i32 } else { 0i32 });
            l_x1_12 = (((l_xOffset_6).wrapping_add(256i32)) as u16);
            { let r_ = 2i32; let n_ = rt::div_i32(l_cols_7, r_); l_cols_7 = n_; };
            l_widthPx_5 = 256i32;
        }
        F::StgHaze_BeginDraw(c, fp, p_haze_1, l_xOffset_6, ((l_x1_12) as i32), l_width_4, l_srcH_21, l_widthPx_5, l_h_20);
        {
            l_row_14 = 0i32;
            'l1: loop {
                if !(((((l_row_14) < ((l_rows_13).wrapping_sub(1i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::StgHaze_PutStripTag(c, fp, p_haze_1, (l_cols_7).wrapping_mul(2i32));
                    l_cx_22 = ((p_haze_1).wrapping_add(3216)).wrapping_add((((l_row_14) as u32)).wrapping_mul(20));
                    l_cy_23 = ((p_haze_1).wrapping_add(4216)).wrapping_add((((l_row_14) as u32)).wrapping_mul(20));
                    l_y0_8 = (rt::div_i32((l_h_20).wrapping_mul(l_row_14), (l_rows_13).wrapping_sub(1i32))).wrapping_shl((4i32) as u32);
                    l_v0_10 = rt::div_i32((l_srcH_21).wrapping_mul(l_row_14), (l_rows_13).wrapping_sub(1i32));
                    l_y1_9 = (rt::div_i32((l_h_20).wrapping_mul((l_row_14).wrapping_add(1i32)), (l_rows_13).wrapping_sub(1i32))).wrapping_shl((4i32) as u32);
                    l_v1_11 = rt::div_i32((l_srcH_21).wrapping_mul((l_row_14).wrapping_add(1i32)), (l_rows_13).wrapping_sub(1i32));
                    {
                        l_col_15 = 0i32;
                        'l3: loop {
                            if !(((((l_col_15) < (l_cols_7)) as i32)) != 0) { break 'l3; }
                            'c4: {
                                l_u_17 = (rt::div_i32((l_col_15).wrapping_mul(l_width_4), (l_cols_7).wrapping_sub(1i32))).wrapping_add(l_xOffset_6);
                                l_px_16 = (rt::div_i32((l_col_15).wrapping_mul(l_widthPx_5), (l_cols_7).wrapping_sub(1i32))).wrapping_shl((4i32) as u32);
                                l_x_18 = (l_px_16).wrapping_add((((c.m.r8((l_cx_22).wrapping_add(((0i32) as u32))) as i8)) as i32));
                                l_y_19 = (l_y0_8).wrapping_add((((c.m.r8((l_cy_23).wrapping_add(((0i32) as u32))) as i8)) as i32));
                                l_x_18 = (if (((((l_x_18) < (0i32)) as i32)) != 0) { 0i32 } else { (if (((((l_x_18) > ((l_widthPx_5).wrapping_shl((4i32) as u32))) as i32)) != 0) { (l_widthPx_5).wrapping_shl((4i32) as u32) } else { l_x_18 }) });
                                l_y_19 = (if (((((l_y_19) < (0i32)) as i32)) != 0) { 0i32 } else { (if (((((l_y_19) > ((l_h_20).wrapping_shl((4i32) as u32))) as i32)) != 0) { (l_h_20).wrapping_shl((4i32) as u32) } else { l_y_19 }) });
                                F::StgHaze_PutVertex(c, fp, l_u_17, l_v0_10, l_x_18, l_y_19);
                                l_x_18 = (l_px_16).wrapping_add((((c.m.r8((l_cx_22).wrapping_add(((1i32) as u32))) as i8)) as i32));
                                l_y_19 = (l_y1_9).wrapping_add((((c.m.r8((l_cy_23).wrapping_add(((1i32) as u32))) as i8)) as i32));
                                l_x_18 = (if (((((l_x_18) < (0i32)) as i32)) != 0) { 0i32 } else { (if (((((l_x_18) > ((l_widthPx_5).wrapping_shl((4i32) as u32))) as i32)) != 0) { (l_widthPx_5).wrapping_shl((4i32) as u32) } else { l_x_18 }) });
                                l_y_19 = (if (((((l_y_19) < (0i32)) as i32)) != 0) { 0i32 } else { (if (((((l_y_19) > ((l_h_20).wrapping_shl((4i32) as u32))) as i32)) != 0) { (l_h_20).wrapping_shl((4i32) as u32) } else { l_y_19 }) });
                                F::StgHaze_PutVertex(c, fp, l_u_17, l_v1_11, l_x_18, l_y_19);
                                { let o_ = l_cx_22; let n_ = (o_).wrapping_add(1); l_cx_22 = n_; };
                                { let o_ = l_cy_23; let n_ = (o_).wrapping_add(1); l_cy_23 = n_; };
                            }
                            { let o_ = l_col_15; let n_ = (o_).wrapping_add(1); l_col_15 = n_; };
                        }
                    }
                    F::StgHaze_EndStrip(c, fp);
                }
                { let o_ = l_row_14; let n_ = (o_).wrapping_add(1); l_row_14 = n_; };
            }
        }
        F::StgHaze_EndDraw(c, fp);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn StgHaze_DrawView(c: &mut Ctx, mut p_split_1: i32, mut p_view_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_DrawView", &PROBE_N, &[(p_split_1) as u32, (p_view_2) as u32], false); }
    let mut l_haze_3: u32 = 0;
    let r_ = 'f: {
    l_haze_3 = (c.m.r32(sym::gStgHaze)).wrapping_add((((p_view_2) as u32)).wrapping_mul(5504));
    F::StgHaze_UpdateClut(c, l_haze_3, p_split_1, p_view_2);
    if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
        F::StgHaze_Step(c, l_haze_3, p_split_1, p_view_2);
    }
    F::StgHaze_Draw(c, l_haze_3, p_split_1, p_view_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_DrawView", &PROBE_N, &[], true); }
    r_
}

pub fn StgHaze_DrawAll(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_DrawAll", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlCam)) == (((0i32) as u32))) as i32)) != 0) {
        F::StgHaze_DrawView(c, 0i32, 0i32);
        break 'f;
    }
    if ((F::Battle_IsSplitScreen(c)) != 0) {
        if ((F::DemoCam_IsActive(c)) != 0) {
            F::StgHaze_DrawView(c, 0i32, 0i32);
        } else {
            if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gBtlCam)).wrapping_add(3136); c.m.r32(a_) }).wrapping_add(640); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                { let a1_ = { let a_ = ({ let a_ = (c.m.r32(sym::gBtlCam)).wrapping_add(3136); c.m.r32(a_) }).wrapping_add(644); (c.m.r32(a_) as i32) }; F::StgHaze_DrawView(c, 0i32, a1_) };
            } else {
                F::StgHaze_DrawView(c, 1i32, 0i32);
                F::StgHaze_DrawView(c, 1i32, 1i32);
            }
        }
    } else {
        { let a1_ = { let a_ = ({ let a_ = (c.m.r32(sym::gBtlCam)).wrapping_add(3136); c.m.r32(a_) }).wrapping_add(644); (c.m.r32(a_) as i32) }; F::StgHaze_DrawView(c, 0i32, a1_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_DrawAll", &PROBE_N, &[], true); }
    r_
}

pub fn StgHaze_SetParams(c: &mut Ctx, mut p_param_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_SetParams", &PROBE_N, &[(p_param_1) as u32], false); }
    let mut l_haze_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_key_4: u32 = 0;
    let r_ = 'f: {
    l_haze_2 = c.m.r32(sym::gStgHaze);
    if (((((p_param_1) == (((0i32) as u32))) as i32)) != 0) {
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::Vec4_Set(c, ((l_haze_2).wrapping_add(5248)).wrapping_add((((0i32) as u32)).wrapping_mul(64)), F32(0x00000000), F32(0x00000000), F32(0x43000000), F32(0x3f800000));
                    F::Vec4_Set(c, ((l_haze_2).wrapping_add(5248)).wrapping_add((((1i32) as u32)).wrapping_mul(64)), F32(0x43480000), F32(0x00000000), F32(0x43000000), F32(0x3f800000));
                    F::Vec4_Set(c, ((l_haze_2).wrapping_add(5248)).wrapping_add((((2i32) as u32)).wrapping_mul(64)), F32(0x437f0000), F32(0x00000000), F32(0x437f0000), F32(0x3f800000));
                }
                { { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; }; { let o_ = l_haze_2; let n_ = (o_).wrapping_add(5504); l_haze_2 = n_; }; };
            }
        }
    } else {
        l_key_4 = (p_param_1).wrapping_add(8);
        {
            l_i_3 = 0i32;
            'l3: loop {
                if !(((((l_i_3) < (2i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let a1_ = ((c.m.r8((p_param_1).wrapping_add(14))) as i32); let a2_ = ((c.m.r8((p_param_1).wrapping_add(15))) as i32); F::StgHaze_SetGrid(c, l_i_3, a1_, a2_) };
                    { let a1_ = ((c.m.r8((p_param_1).wrapping_add(16))) as i32); let a2_ = ((c.m.r8((p_param_1).wrapping_add(17))) as i32); F::StgHaze_SetAmplitude(c, l_i_3, a1_, a2_) };
                    { let a1_ = ((c.m.r8(((p_param_1).wrapping_add(4)).wrapping_add(((0i32) as u32)))) as i32); let a2_ = ((c.m.r8(((p_param_1).wrapping_add(4)).wrapping_add(((1i32) as u32)))) as i32); let a3_ = ((c.m.r8(((p_param_1).wrapping_add(4)).wrapping_add(((2i32) as u32)))) as i32); let a4_ = ((c.m.r8(((p_param_1).wrapping_add(4)).wrapping_add(((3i32) as u32)))) as i32); F::StgHaze_SetColor(c, l_i_3, a1_, a2_, a3_, a4_) };
                    { let a1_ = ((c.m.r8((p_param_1).wrapping_add(18))) as i32); let a2_ = F32(c.m.r32((p_param_1).wrapping_add(20))); F::StgHaze_SetSpeed(c, l_i_3, a1_, a2_) };
                    { let a0_ = ((c.m.r32(sym::gStgHaze)).wrapping_add(5248)).wrapping_add((((0i32) as u32)).wrapping_mul(64)); let a1_ = rt::i2f((c.m.r8((l_key_4).wrapping_add(((0i32) as u32)))) as i32); let a3_ = rt::i2f((c.m.r8((l_key_4).wrapping_add(((1i32) as u32)))) as i32); F::Vec4_Set(c, a0_, a1_, F32(0x00000000), a3_, F32(0x3f800000)) };
                    { let a0_ = ((c.m.r32(sym::gStgHaze)).wrapping_add(5248)).wrapping_add((((1i32) as u32)).wrapping_mul(64)); let a1_ = rt::i2f((c.m.r8((l_key_4).wrapping_add(((2i32) as u32)))) as i32); let a3_ = rt::i2f((c.m.r8((l_key_4).wrapping_add(((3i32) as u32)))) as i32); F::Vec4_Set(c, a0_, a1_, F32(0x00000000), a3_, F32(0x3f800000)) };
                    { let a0_ = ((c.m.r32(sym::gStgHaze)).wrapping_add(5248)).wrapping_add((((2i32) as u32)).wrapping_mul(64)); let a1_ = rt::i2f((c.m.r8((l_key_4).wrapping_add(((4i32) as u32)))) as i32); let a3_ = rt::i2f((c.m.r8((l_key_4).wrapping_add(((5i32) as u32)))) as i32); F::Vec4_Set(c, a0_, a1_, F32(0x00000000), a3_, F32(0x3f800000)) };
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_SetParams", &PROBE_N, &[], true); }
    r_
}

pub fn StgHaze_SetGrid(c: &mut Ctx, mut p_view_1: i32, mut p_cols_2: i32, mut p_rows_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_SetGrid", &PROBE_N, &[(p_view_1) as u32, (p_cols_2) as u32, (p_rows_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_cols_2; let a_ = ((c.m.r32(sym::gStgHaze)).wrapping_add((((p_view_1) as u32)).wrapping_mul(5504))).wrapping_add(5216); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_rows_3; let a_ = ((c.m.r32(sym::gStgHaze)).wrapping_add((((p_view_1) as u32)).wrapping_mul(5504))).wrapping_add(5220); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_SetGrid", &PROBE_N, &[], true); }
    r_
}

pub fn StgHaze_SetAmplitude(c: &mut Ctx, mut p_view_1: i32, mut p_ampX_2: i32, mut p_ampY_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_SetAmplitude", &PROBE_N, &[(p_view_1) as u32, (p_ampX_2) as u32, (p_ampY_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_ampX_2; let a_ = ((c.m.r32(sym::gStgHaze)).wrapping_add((((p_view_1) as u32)).wrapping_mul(5504))).wrapping_add(5224); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_ampY_3; let a_ = ((c.m.r32(sym::gStgHaze)).wrapping_add((((p_view_1) as u32)).wrapping_mul(5504))).wrapping_add(5228); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_SetAmplitude", &PROBE_N, &[], true); }
    r_
}

pub fn StgHaze_SetColor(c: &mut Ctx, mut p_view_1: i32, mut p_r_2: i32, mut p_g_3: i32, mut p_b_4: i32, mut p_a_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_SetColor", &PROBE_N, &[(p_view_1) as u32, (p_r_2) as u32, (p_g_3) as u32, (p_b_4) as u32, (p_a_5) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((((((((p_r_2) | ((p_g_3).wrapping_shl((8i32) as u32)))) | ((p_b_4).wrapping_shl((16i32) as u32)))) | ((p_a_5).wrapping_shl((24i32) as u32)))) as u32); let a_ = ((c.m.r32(sym::gStgHaze)).wrapping_add((((p_view_1) as u32)).wrapping_mul(5504))).wrapping_add(5232); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_SetColor", &PROBE_N, &[], true); }
    r_
}

pub fn StgHaze_SetSpeed(c: &mut Ctx, mut p_view_1: i32, mut p_period_2: i32, mut p_follow_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_SetSpeed", &PROBE_N, &[(p_view_1) as u32, (p_period_2) as u32, (p_follow_3).0], false); }
    let r_ = 'f: {
    { let v_ = p_period_2; let a_ = ((c.m.r32(sym::gStgHaze)).wrapping_add((((p_view_1) as u32)).wrapping_mul(5504))).wrapping_add(5236); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_follow_3; let a_ = ((c.m.r32(sym::gStgHaze)).wrapping_add((((p_view_1) as u32)).wrapping_mul(5504))).wrapping_add(5244); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_SetSpeed", &PROBE_N, &[], true); }
    r_
}

pub fn StgHaze_GetCurve(c: &mut Ctx, mut p_view_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_GetCurve", &PROBE_N, &[(p_view_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((c.m.r32(sym::gStgHaze)).wrapping_add((((p_view_1) as u32)).wrapping_mul(5504))).wrapping_add(5248);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_GetCurve", &PROBE_N, &[], true); }
    r_
}

pub fn StgHaze_RebuildClut(c: &mut Ctx, mut p_view_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_RebuildClut", &PROBE_N, &[(p_view_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32(sym::gStgHaze)).wrapping_add((((p_view_1) as u32)).wrapping_mul(5504)); F::StgHaze_UpdateClut(c, a0_, 0i32, 0i32) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgHaze_RebuildClut", &PROBE_N, &[], true); }
    r_
}

pub fn StgFx_SetDisabled(c: &mut Ctx, mut p_disabled_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgFx_SetDisabled", &PROBE_N, &[(p_disabled_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_disabled_1; let a_ = c.m.r32(sym::gStgFx); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgFx_SetDisabled", &PROBE_N, &[], true); }
    r_
}

pub fn StgFx_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgFx_Reset", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = c.m.r32(sym::gStgFx); c.m.w32(a_, (v_) as u32); };
    F::StgHaze_Reset(c);
    F::StgBlur_Reset(c);
    F::ScrXfade_Reset(c);
    F::StgFog_ResetColor(c);
    F::ScrWarp_Reset(c);
    F::StgDepthTint_Rebuild(c);
    F::StgGlare_Reset(c);
    F::GfxLens_Clear(c);
    F::StgTint_Reset(c);
    { let a0_ = F::BtlStage_GetListA0(c); F::StgHaze_SetParams(c, a0_) };
    { let a0_ = F::BtlStage_GetList50(c); F::StgFog_SetParams(c, a0_) };
    F::StgDepthTint_LoadParams(c);
    F::GfxWater_LoadStageColor(c);
    F::StgGlare_LoadParams(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgFx_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn StgFx_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgFx_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, 4i32, ((32i32) as u32), 0i32, 2i32); let a_ = sym::gStgFx; c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = c.m.r32(sym::gStgFx); c.m.w32(a_, (v_) as u32); };
    F::GfxPost_InitNop(c);
    F::StgHaze_Init(c, ((16028i32) as u16));
    F::StgBlur_Init(c);
    F::ScrXfade_Init(c);
    F::StgFog_Init(c, ((((16004i32) as i64 as u64)) as u16));
    F::ScrWarp_Init(c);
    F::GfxWater_Init(c);
    F::GfxDepthFog_Init(c, ((16000i32) as u16));
    F::StgDepthTint_Init(c, ((15968i32) as u16));
    F::StgGlare_Init(c, ((16008i32) as u16));
    F::GfxLens_Init(c);
    F::StgPanBlur_Init(c, 16024i32);
    F::StgTint_Init(c);
    F::StgFx_Reset(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgFx_Init", &PROBE_N, &[], true); }
    r_
}

pub fn StgFx_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgFx_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::StgHaze_Term(c);
    F::StgBlur_Term(c);
    F::ScrXfade_Term(c);
    F::StgFog_Term(c);
    F::ScrWarp_Term(c);
    F::GfxWater_Term(c);
    F::StgDepthTint_Term(c);
    F::StgGlare_Term(c);
    F::GfxLens_Stub107758(c);
    F::StgPanBlur_Term(c);
    F::GfxPost_TermNop(c);
    { let a0_ = c.m.r32(sym::gStgFx); F::Heap_Free(c, a0_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gStgFx; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgFx_Term", &PROBE_N, &[], true); }
    r_
}

pub fn StgFx_DrawPre(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgFx_DrawPre", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = c.m.r32(sym::gStgFx); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        F::StgTint_Update(c);
        F::GfxDepthFog_Draw(c);
        F::StgDepthTint_Draw(c);
        if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((288230376151711744i64) as u64)))) != 0)) as i32)) != 0) {
            F::StgGlare_Update(c);
            F::StgGlare_Draw(c);
        }
        if ((((((F::BtlStage_HasFeature(c, ((((3i32) as i64 as u64)) as u32))) != 0) && ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((288230376151711744i64) as u64)))) != 0)) as i32)) != 0)) as i32)) != 0) {
            F::StgFog_Draw(c);
        }
        if ((F::BtlStage_HasFeature(c, ((((13i32) as i64 as u64)) as u32))) != 0) {
            F::StgPanBlur_Draw(c);
        }
        if ((F::BtlStage_HasFeature(c, ((((9i32) as i64 as u64)) as u32))) != 0) {
            F::StgHaze_DrawAll(c);
        }
        F::StgTint_DrawBack(c);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgFx_DrawPre", &PROBE_N, &[], true); }
    r_
}

pub fn StgFx_DrawNop(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgFx_DrawNop", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = c.m.r32(sym::gStgFx); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        F::StgTint_Nop(c);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgFx_DrawNop", &PROBE_N, &[], true); }
    r_
}

pub fn StgFx_DrawPost(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgFx_DrawPost", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Fade_DrawSlot2(c);
    if ((((({ let a_ = c.m.r32(sym::gStgFx); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        F::GfxWater_Draw(c);
        F::StgBlur_DrawAll(c);
        F::GfxLens_DrawFull(c);
        F::ScrWarp_Update(c);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgFx_DrawPost", &PROBE_N, &[], true); }
    r_
}

pub fn StgFx_DrawOverlay(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgFx_DrawOverlay", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = c.m.r32(sym::gStgFx); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        F::ScrXfade_PostDraw(c);
        F::ScrXfade_Update(c);
        F::StgTint_DrawFront(c);
        F::StgTint_Release(c);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgFx_DrawOverlay", &PROBE_N, &[], true); }
    r_
}

pub fn StgFx_DrawPreNoCheck(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgFx_DrawPreNoCheck", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = c.m.r32(sym::gStgFx); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        F::StgTint_Update(c);
        F::GfxDepthFog_Draw(c);
        F::StgDepthTint_Draw(c);
        F::StgGlare_Update(c);
        F::StgGlare_Draw(c);
        F::StgFog_Draw(c);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgFx_DrawPreNoCheck", &PROBE_N, &[], true); }
    r_
}

pub fn StgTint_CalcColor(c: &mut Ctx, mut p_tint_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgTint_CalcColor", &PROBE_N, &[(p_tint_1) as u32], false); }
    let mut l_t_2: F32 = F32(0);
    let r_ = 'f: {
    l_t_2 = F32(c.m.r32(((p_tint_1).wrapping_add(64)).wrapping_add(16)));
    { let v_ = rt::fadd(F32(c.m.r32((p_tint_1).wrapping_add(16))), rt::fmul(l_t_2, F32(c.m.r32((p_tint_1).wrapping_add(48))))); let a_ = p_tint_1; c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(F32(c.m.r32(((p_tint_1).wrapping_add(16)).wrapping_add(4))), rt::fmul(l_t_2, F32(c.m.r32(((p_tint_1).wrapping_add(48)).wrapping_add(4))))); let a_ = (p_tint_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(F32(c.m.r32(((p_tint_1).wrapping_add(16)).wrapping_add(8))), rt::fmul(l_t_2, F32(c.m.r32(((p_tint_1).wrapping_add(48)).wrapping_add(8))))); let a_ = (p_tint_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(F32(c.m.r32(((p_tint_1).wrapping_add(16)).wrapping_add(12))), rt::fmul(l_t_2, F32(c.m.r32(((p_tint_1).wrapping_add(48)).wrapping_add(12))))); let a_ = (p_tint_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgTint_CalcColor", &PROBE_N, &[], true); }
    r_
}

pub fn StgTint_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgTint_Reset", &PROBE_N, &[], false); }
    let mut l_work_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_tint_3: u32 = 0;
    let r_ = 'f: {
    l_work_1 = sym::gStgTint;
    { let v_ = 0i32; let a_ = (l_work_1).wrapping_add(384); c.m.w32(a_, (v_) as u32); };
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_tint_3 = (l_work_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(96));
                { let v_ = ((0i32) as u8); let a_ = (l_tint_3).wrapping_add(88); c.m.w8(a_, v_); };
                F::Ramp_Stop(c, (l_tint_3).wrapping_add(64));
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgTint_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn StgTint_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgTint_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let _ = F::memset(c, sym::gStgTint, 0i32, 400u32); };
    F::StgTint_Reset(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgTint_Init", &PROBE_N, &[], true); }
    r_
}

pub fn StgTint_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgTint_Update", &PROBE_N, &[], false); }
    let mut l_tint_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_tint_1 = sym::gStgTint;
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((((((l_i_2) != (3i32)) as i32)) != 0) && (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
                    break 'c2;
                }
                if ((((((c.m.r8((l_tint_1).wrapping_add(88))) as i32)) & (1i32))) != 0) {
                    if (((((F::Ramp_Step(c, (l_tint_1).wrapping_add(64))) == (1i32)) as i32)) != 0) {
                        if ((((((c.m.r8((l_tint_1).wrapping_add(88))) as i32)) & (4i32))) != 0) {
                            { let r_ = 8i32; let a_ = (l_tint_1).wrapping_add(88); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
                            F::Ramp_Stop(c, (l_tint_1).wrapping_add(64));
                        }
                    }
                }
            }
            { { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; }; { let o_ = l_tint_1; let n_ = (o_).wrapping_add(96); l_tint_1 = n_; }; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgTint_Update", &PROBE_N, &[], true); }
    r_
}

pub fn StgTint_Release(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgTint_Release", &PROBE_N, &[], false); }
    let mut l_tint_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_tint_1 = sym::gStgTint;
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((((((c.m.r8((l_tint_1).wrapping_add(88))) as i32)) & (1i32))) != 0) && ((((((c.m.r8((l_tint_1).wrapping_add(88))) as i32)) & (8i32))) != 0)) as i32)) != 0) {
                    { let v_ = ((0i32) as u8); let a_ = (l_tint_1).wrapping_add(88); c.m.w8(a_, v_); };
                    F::Ramp_Stop(c, (l_tint_1).wrapping_add(64));
                }
            }
            { { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; }; { let o_ = l_tint_1; let n_ = (o_).wrapping_add(96); l_tint_1 = n_; }; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgTint_Release", &PROBE_N, &[], true); }
    r_
}

pub fn StgTint_DrawBack(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgTint_DrawBack", &PROBE_N, &[], false); }
    let mut l_tint_1: u32 = 0;
    let r_ = 'f: {
    l_tint_1 = (sym::gStgTint).wrapping_add((((0i32) as u32)).wrapping_mul(96));
    if ((((((c.m.r8((l_tint_1).wrapping_add(88))) as i32)) & (1i32))) != 0) {
        F::StgTint_CalcColor(c, l_tint_1);
        F::GfxPost_DrawTintRect(c, ((68i32) as u64), l_tint_1);
        { let v_ = rt::f2i(F32(c.m.r32(l_tint_1))); let a_ = (sym::gStgTintColor0).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = rt::f2i(F32(c.m.r32((l_tint_1).wrapping_add(4)))); let a_ = (sym::gStgTintColor0).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = rt::f2i(F32(c.m.r32((l_tint_1).wrapping_add(8)))); let a_ = (sym::gStgTintColor0).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = rt::f2i(F32(c.m.r32((l_tint_1).wrapping_add(12)))); let a_ = (sym::gStgTintColor0).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    } else {
        { let v_ = 0i32; let a_ = (sym::gStgTintColor0).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (sym::gStgTintColor0).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (sym::gStgTintColor0).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (sym::gStgTintColor0).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    }
    l_tint_1 = (sym::gStgTint).wrapping_add((((1i32) as u32)).wrapping_mul(96));
    if ((((((c.m.r8((l_tint_1).wrapping_add(88))) as i32)) & (1i32))) != 0) {
        F::StgTint_CalcColor(c, l_tint_1);
        F::GfxPost_DrawTintRect(c, ((68i32) as u64), l_tint_1);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgTint_DrawBack", &PROBE_N, &[], true); }
    r_
}

pub fn StgTint_Nop(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgTint_Nop", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgTint_Nop", &PROBE_N, &[], true); }
    r_
}

pub fn StgTint_DrawFront(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgTint_DrawFront", &PROBE_N, &[], false); }
    let mut l_tint_1: u32 = 0;
    let r_ = 'f: {
    l_tint_1 = (sym::gStgTint).wrapping_add((((3i32) as u32)).wrapping_mul(96));
    F::StgTint_CalcColor(c, l_tint_1);
    if (((rt::fgt(F32(c.m.r32((l_tint_1).wrapping_add(12))), F32(0x3a83126e)) as i32)) != 0) {
        F::GfxPost_DrawTintRect(c, ((68i32) as u64), l_tint_1);
    }
    { let o_ = l_tint_1; let n_ = (o_).wrapping_sub(96); l_tint_1 = n_; };
    F::StgTint_CalcColor(c, l_tint_1);
    if (((rt::fgt(F32(c.m.r32((l_tint_1).wrapping_add(12))), F32(0x3a83126e)) as i32)) != 0) {
        F::GfxPost_DrawTintRect(c, ((68i32) as u64), l_tint_1);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgTint_DrawFront", &PROBE_N, &[], true); }
    r_
}

pub fn StgTint_Start(c: &mut Ctx, mut p_slot_1: u32, mut p_dir_2: i32, mut p_seconds_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgTint_Start", &PROBE_N, &[(p_slot_1) as u32, (p_dir_2) as u32, (p_seconds_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(128) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 128);
    let mut l_stage_4: u32 = 0;
    let mut l_tint_5: u32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 128);
        rt::memset(c, fp.wrapping_add(0), 0, 32);
        rt::memset(c, fp.wrapping_add(0).wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(0).wrapping_add(16), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(16).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(16).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(16).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x42c80000); c.m.w32(fp.wrapping_add(0).wrapping_add(16).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(32), 0, 32);
        rt::memset(c, fp.wrapping_add(32).wrapping_add(0), 0, 16);
        { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(32).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(32).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(32).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(32).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(32).wrapping_add(16), 0, 16);
        { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(32).wrapping_add(16).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(32).wrapping_add(16).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(32).wrapping_add(16).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x43000000); c.m.w32(fp.wrapping_add(32).wrapping_add(16).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(64), 0, 32);
        rt::memset(c, fp.wrapping_add(64).wrapping_add(0), 0, 16);
        { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(64).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(64).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(64).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(64).wrapping_add(16), 0, 16);
        { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(64).wrapping_add(16).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(64).wrapping_add(16).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x437f0000); c.m.w32(fp.wrapping_add(64).wrapping_add(16).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x43000000); c.m.w32(fp.wrapping_add(64).wrapping_add(16).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(96), 0, 32);
        rt::memset(c, fp.wrapping_add(96).wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(96).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(96).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(96).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x43000000); c.m.w32(fp.wrapping_add(96).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(96).wrapping_add(16), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(96).wrapping_add(16).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(96).wrapping_add(16).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(96).wrapping_add(16).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(96).wrapping_add(16).wrapping_add(12), (v_).0); }
        l_stage_4 = F::BtlStage_GetList80(c);
        if (((((l_stage_4) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = rt::i2f((c.m.r8((l_stage_4).wrapping_add(((0i32) as u32)))) as i32); let a_ = ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(32))).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
            { let v_ = rt::i2f((c.m.r8((l_stage_4).wrapping_add(((1i32) as u32)))) as i32); let a_ = (((fp).wrapping_add((((0i32) as u32)).wrapping_mul(32))).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = rt::i2f((c.m.r8((l_stage_4).wrapping_add(((2i32) as u32)))) as i32); let a_ = (((fp).wrapping_add((((0i32) as u32)).wrapping_mul(32))).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = rt::i2f((c.m.r8((l_stage_4).wrapping_add(((3i32) as u32)))) as i32); let a_ = (((fp).wrapping_add((((0i32) as u32)).wrapping_mul(32))).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
            { let v_ = rt::i2f((c.m.r8((l_stage_4).wrapping_add(((4i32) as u32)))) as i32); let a_ = ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(32))).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
            { let v_ = rt::i2f((c.m.r8((l_stage_4).wrapping_add(((5i32) as u32)))) as i32); let a_ = (((fp).wrapping_add((((0i32) as u32)).wrapping_mul(32))).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = rt::i2f((c.m.r8((l_stage_4).wrapping_add(((6i32) as u32)))) as i32); let a_ = (((fp).wrapping_add((((0i32) as u32)).wrapping_mul(32))).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = rt::i2f((c.m.r8((l_stage_4).wrapping_add(((7i32) as u32)))) as i32); let a_ = (((fp).wrapping_add((((0i32) as u32)).wrapping_mul(32))).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
        }
        if (((((p_slot_1) >= (((4i32) as u32))) as i32)) != 0) {
            p_slot_1 = ((0i32) as u32);
        }
        l_tint_5 = (sym::gStgTint).wrapping_add((p_slot_1).wrapping_mul(96));
        if ((((!((((((c.m.r8((l_tint_5).wrapping_add(88))) as i32)) & (1i32))) != 0)) as i32)) != 0) {
            if (((((p_dir_2) == (1i32)) as i32)) != 0) {
                break 'f;
            }
            { let v_ = ((((1i32) | (2i32))) as u8); let a_ = (l_tint_5).wrapping_add(88); c.m.w8(a_, v_); };
            F::Vec4_Copy(c, (l_tint_5).wrapping_add(16), ((fp).wrapping_add((p_slot_1).wrapping_mul(32))).wrapping_add((((p_dir_2) as u32)).wrapping_mul(16)));
            F::Vec4_Copy(c, (l_tint_5).wrapping_add(32), ((fp).wrapping_add((p_slot_1).wrapping_mul(32))).wrapping_add((((((p_dir_2) ^ (1i32))) as u32)).wrapping_mul(16)));
            F::Vec4_Sub(c, (l_tint_5).wrapping_add(48), (l_tint_5).wrapping_add(32), (l_tint_5).wrapping_add(16));
        } else {
            if (((((p_dir_2) == (0i32)) as i32)) != 0) {
                if ((((((c.m.r8((l_tint_5).wrapping_add(88))) as i32)) & (2i32))) != 0) {
                    break 'f;
                }
                { let v_ = ((((1i32) | (2i32))) as u8); let a_ = (l_tint_5).wrapping_add(88); c.m.w8(a_, v_); };
            } else {
                if ((((((c.m.r8((l_tint_5).wrapping_add(88))) as i32)) & (4i32))) != 0) {
                    break 'f;
                }
                { let v_ = ((((1i32) | (4i32))) as u8); let a_ = (l_tint_5).wrapping_add(88); c.m.w8(a_, v_); };
            }
            F::Vec4_Copy(c, (l_tint_5).wrapping_add(16), l_tint_5);
            F::Vec4_Copy(c, (l_tint_5).wrapping_add(32), ((fp).wrapping_add((p_slot_1).wrapping_mul(32))).wrapping_add((((((p_dir_2) ^ (1i32))) as u32)).wrapping_mul(16)));
            F::Vec4_Sub(c, (l_tint_5).wrapping_add(48), (l_tint_5).wrapping_add(32), (l_tint_5).wrapping_add(16));
        }
        F::Ramp_Start(c, (l_tint_5).wrapping_add(64), p_seconds_3, F32(0x00000000), F32(0x3f800000));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgTint_Start", &PROBE_N, &[], true); }
    r_
}

pub fn StgTint_GetColor0(c: &mut Ctx, mut p_out_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgTint_GetColor0", &PROBE_N, &[(p_out_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = (c.m.r32((sym::gStgTintColor0).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a_ = (p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32((sym::gStgTintColor0).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); let a_ = (p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32((sym::gStgTintColor0).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32); let a_ = (p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32((sym::gStgTintColor0).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32); let a_ = (p_out_1).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgTint_GetColor0", &PROBE_N, &[], true); }
    r_
}

pub fn StgTint_GetWork(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgTint_GetWork", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f sym::gStgTint;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "StgTint_GetWork", &PROBE_N, &[], true); }
    r_
}

pub fn StgTint_IsOn(c: &mut Ctx, mut p_slot_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgTint_IsOn", &PROBE_N, &[(p_slot_1) as u32], false); }
    let r_ = 'f: {
    if (((((p_slot_1) >= (((4i32) as u32))) as i32)) != 0) {
        p_slot_1 = ((0i32) as u32);
    }
    break 'f ((((c.m.r8(((sym::gStgTint).wrapping_add((p_slot_1).wrapping_mul(96))).wrapping_add(88))) as i32)) & (1i32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "StgTint_IsOn", &PROBE_N, &[], true); }
    r_
}

pub fn StgBlur_BeginDraw(c: &mut Ctx, mut p_d_1: u32, mut p_x0_2: i32, mut p_srcW_3: i32, mut p_width_4: i32, mut p_srcH_5: i32, mut p_dstW_6: i32, mut p_dstH_7: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_BeginDraw", &PROBE_N, &[(p_d_1) as u32, (p_x0_2) as u32, (p_srcW_3) as u32, (p_width_4) as u32, (p_srcH_5) as u32, (p_dstW_6) as u32, (p_dstH_7) as u32], false); }
    let r_ = 'f: {
    { let v_ = F::Dma_BeginDirect(c); let a_ = p_d_1; c.m.w32(a_, v_); };
    F::Gfx_PutDefaultEnv(c, p_d_1);
    { let v_ = ((((((((6i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (c.m.r32(p_d_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_d_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = ((196608i32) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((71i32) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_d_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = ((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (c.m.r32(p_d_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((78i32) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_d_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { 10267790848i64 } else { 10267787264i64 })) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((6i32) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_d_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = ((262480i32) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((76i32) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_d_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = ((((((((0i32) as u64)) | (((((p_dstW_6).wrapping_sub(1i32)) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) | (((((p_dstH_7).wrapping_sub(1i32)) as u64)).wrapping_shl((48i32) as u32))); let a_ = (c.m.r32(p_d_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((64i32) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_d_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = ((((((((((((2i32) as u64)) | ((((2i32) as u64)).wrapping_shl((2i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((4i32) as u32)))) | (((((p_srcW_3).wrapping_sub(1i32)) as u64)).wrapping_shl((14i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | (((((p_srcH_5).wrapping_sub(1i32)) as u64)).wrapping_shl((34i32) as u32))); let a_ = (c.m.r32(p_d_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((8i32) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_d_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_BeginDraw", &PROBE_N, &[], true); }
    r_
}

pub fn StgBlur_EndDraw(c: &mut Ctx, mut p_d_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_EndDraw", &PROBE_N, &[(p_d_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32(p_d_1); F::Dma_EndDirect(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_EndDraw", &PROBE_N, &[], true); }
    r_
}

pub fn StgBlur_PutStripTag(c: &mut Ctx, mut p_d_1: u32, mut p_nVerts_2: i32, mut p_unused_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_PutStripTag", &PROBE_N, &[(p_d_1) as u32, (p_nVerts_2) as u32, (p_unused_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((1152921504606846977i64) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_d_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = ((348i32) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((0i32) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_d_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = ((((((p_nVerts_2) | (32768i32))) as u64)) | ((((34816i32) as u64)).wrapping_shl((47i32) as u32))); let a_ = (c.m.r32(p_d_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((21279i32) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_d_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_PutStripTag", &PROBE_N, &[], true); }
    r_
}

pub fn StgBlur_Nop(c: &mut Ctx, mut p_d_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_Nop", &PROBE_N, &[(p_d_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_Nop", &PROBE_N, &[], true); }
    r_
}

pub fn StgBlur_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_Init", &PROBE_N, &[], false); }
    let mut l_blur_1: u32 = 0;
    let r_ = 'f: {
    l_blur_1 = F::Heap_Alloc(c, (((80u32).wrapping_mul(((2i32) as u32))) as i32), ((32i32) as u32), 0i32, 2i32);
    { let v_ = l_blur_1; let a_ = sym::gStgBlur; c.m.w32(a_, v_); };
    { let _ = F::memset(c, l_blur_1, 0i32, (80u32).wrapping_mul(((2i32) as u32))); };
    F::StgBlur_Reset(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_Init", &PROBE_N, &[], true); }
    r_
}

pub fn StgBlur_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gStgBlur)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gStgBlur); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gStgBlur; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_Term", &PROBE_N, &[], true); }
    r_
}

pub fn StgBlur_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_Reset", &PROBE_N, &[], false); }
    let mut l_blur_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_blur_1 = c.m.r32(sym::gStgBlur);
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = 0i32; let a_ = l_blur_1; c.m.w32(a_, (v_) as u32); };
                F::Vec4_Set(c, (l_blur_1).wrapping_add(16), F32(0x00000000), F32(0x00000000), F32(0x3f800000), F32(0x00000000));
                { let v_ = 3i32; let a_ = (l_blur_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = F32(0x40000000); let a_ = (l_blur_1).wrapping_add(32); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x40000000); let a_ = (l_blur_1).wrapping_add(40); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x40000000); let a_ = (l_blur_1).wrapping_add(44); c.m.w32(a_, (v_).0); };
                { let v_ = 1i32; let a_ = (l_blur_1).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
                { let o_ = l_blur_1; let n_ = (o_).wrapping_add(80); l_blur_1 = n_; };
                F::StgBlur_SetColor0Rgba(c, l_i_2, ((128i32) as u8), ((128i32) as u8), ((128i32) as u8), ((0i32) as u8));
                F::StgBlur_SetColor1Rgba(c, l_i_2, ((128i32) as u8), ((128i32) as u8), ((128i32) as u8), ((0i32) as u8));
                F::StgBlur_SetColor2Rgba(c, l_i_2, ((128i32) as u8), ((128i32) as u8), ((128i32) as u8), ((0i32) as u8));
                F::StgBlur_SetColor3Rgba(c, l_i_2, ((128i32) as u8), ((128i32) as u8), ((128i32) as u8), ((0i32) as u8));
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn stg_c__StgBlur_PutVertex(c: &mut Ctx, mut p_d_1: u32, mut p_rgba_2: u32, mut p_x_3: i32, mut p_y_4: i32, mut p_u_5: i32, mut p_v_6: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "stg_c__StgBlur_PutVertex", &PROBE_N, &[(p_d_1) as u32, (p_rgba_2) as u32, (p_x_3) as u32, (p_y_4) as u32, (p_u_5) as u32, (p_v_6) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((0i32) as u64); let a_ = (c.m.r32(p_d_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((((p_rgba_2) as u64)) | ((((65024i32) as u64)).wrapping_shl((46i32) as u32))); let a_ = (c.m.r32(p_d_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_d_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = ((((((p_u_5).wrapping_shl((4i32) as u32)).wrapping_add(8i32)) as u64)) | ((((((p_v_6).wrapping_shl((4i32) as u32)).wrapping_add(8i32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (c.m.r32(p_d_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((((((p_x_3).wrapping_shl((4i32) as u32)).wrapping_add(28672i32)) as u64)) | ((((((p_y_4).wrapping_shl((4i32) as u32)).wrapping_add(29184i32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (c.m.r32(p_d_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_d_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "stg_c__StgBlur_PutVertex", &PROBE_N, &[], true); }
    r_
}

pub fn StgBlur_Draw(c: &mut Ctx, mut p_split_1: i32, mut p_view_2: i32, mut p_viewMtx_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_Draw", &PROBE_N, &[(p_split_1) as u32, (p_view_2) as u32, (p_viewMtx_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(240) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 240);
    let mut l_blur_4: u32 = 0;
    let mut l_width_5: i32 = 0;
    let mut l_half_6: i32 = 0;
    let mut l_xOffset_7: i32 = 0;
    let mut l_srcH_8: i32 = 0;
    let mut l_h_9: i32 = 0;
    let mut l_lastTbp_10: i32 = 0;
    let mut l_x1_11: i32 = 0;
    let mut l_u1_12: i32 = 0;
    let mut l_i_13: i32 = 0;
    let mut l_f_14: F32 = F32(0);
    let mut l_grow_15: i32 = 0;
    let mut l_dx_16: i32 = 0;
    let mut l_dy_17: i32 = 0;
    let mut l_nx_18: i32 = 0;
    let mut l_ny_19: i32 = 0;
    let mut l_ua_20: i32 = 0;
    let mut l_ub_21: i32 = 0;
    let mut l_vb_22: i32 = 0;
    let mut l_va_23: i32 = 0;
    let mut l_cx_24: i32 = 0;
    let mut l_cy_25: i32 = 0;
    let mut l_hx_26: i32 = 0;
    let mut l_hy_27: i32 = 0;
    let mut l_rx_28: i32 = 0;
    let mut l_ry_29: i32 = 0;
    let mut l_ru_30: i32 = 0;
    let mut l_rv_31: i32 = 0;
    let r_ = 'f: {
        l_blur_4 = (c.m.r32(sym::gStgBlur)).wrapping_add((((p_view_2) as u32)).wrapping_mul(80));
        l_width_5 = 512i32;
        l_half_6 = 256i32;
        l_srcH_8 = 448i32;
        l_h_9 = 224i32;
        l_lastTbp_10 = 0i32;
        l_xOffset_7 = 0i32;
        l_x1_11 = 512i32;
        if (((((((((((c.m.r64((l_blur_4).wrapping_add(48))) & (18374686483949813760u64))) == (((0i32) as u64))) as i32)) != 0) && (((((((c.m.r32((l_blur_4).wrapping_add(56))) & (4278190080u32))) == (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
            break 'f;
        }
        if ((p_split_1) != 0) {
            l_width_5 = 256i32;
            l_half_6 = 128i32;
            l_xOffset_7 = (if (((((p_view_2) == (1i32)) as i32)) != 0) { 256i32 } else { 0i32 });
            l_x1_11 = (l_xOffset_7).wrapping_add(l_width_5);
            l_u1_12 = l_x1_11;
        } else {
            l_u1_12 = 512i32;
        }
        if (((c.m.r32((l_blur_4).wrapping_add(64)) as i32)) != 0) {
            F::Vec4_Copy(c, fp, (l_blur_4).wrapping_add(16));
        } else {
            F::Mtx_Copy(c, fp.wrapping_add(16), p_viewMtx_3);
            F::Mtx_ClearTrans(c, fp.wrapping_add(16));
            F::Mtx_MulVec4(c, fp, fp.wrapping_add(16), (l_blur_4).wrapping_add(16));
        }
        F::StgBlur_BeginDraw(c, fp.wrapping_add(80), l_xOffset_7, l_x1_11, l_width_5, l_srcH_8, l_half_6, l_h_9);
        {
            rt::memset(c, fp.wrapping_add(84), 0, 8);
            { let v_ = ((10752i32) as u32); c.m.w32(fp.wrapping_add(84).wrapping_add(0), v_); }
            { let v_ = ((11648i32) as u32); c.m.w32(fp.wrapping_add(84).wrapping_add(4), v_); }
            rt::memset(c, fp.wrapping_add(92), 0, 8);
            { let v_ = ((336i32) as u32); c.m.w32(fp.wrapping_add(92).wrapping_add(0), v_); }
            { let v_ = ((364i32) as u32); c.m.w32(fp.wrapping_add(92).wrapping_add(4), v_); }
            F::Dma_PutTexStrips(c, fp.wrapping_add(80), 0i32, 0i32, l_half_6, l_h_9, 0i32, 0i32, l_xOffset_7, 0i32, l_u1_12, l_srcH_8, 0i32, 0i32, 2155905152u32, 0i32);
            l_f_14 = F32(0x3f800000);
            {
                l_i_13 = 0i32;
                'l1: loop {
                    if !(((((l_i_13) < ((c.m.r32((l_blur_4).wrapping_add(36)) as i32))) as i32)) != 0) { break 'l1; }
                    'c2: {
                        l_grow_15 = rt::f2i(rt::fmul(l_f_14, F32(c.m.r32((l_blur_4).wrapping_add(32)))));
                        l_dx_16 = rt::f2i(rt::fmul(rt::fmul(rt::i2f(rt::f2i(l_f_14)), F32(c.m.r32((l_blur_4).wrapping_add(40)))), F32(c.m.r32(fp))));
                        l_dy_17 = rt::f2i(rt::fmul(rt::fmul(l_f_14, F32(c.m.r32((l_blur_4).wrapping_add(44)))), F32(c.m.r32((fp).wrapping_add(4)))));
                        l_nx_18 = 0i32;
                        l_ny_19 = 0i32;
                        if (((((l_dx_16) < (0i32)) as i32)) != 0) {
                            l_nx_18 = (l_dx_16).wrapping_neg();
                            l_dx_16 = 0i32;
                        }
                        if (((((l_dy_17) < (0i32)) as i32)) != 0) {
                            l_ny_19 = (l_dy_17).wrapping_neg();
                            l_dy_17 = 0i32;
                        }
                        l_ua_20 = (l_grow_15).wrapping_add(l_dx_16);
                        l_ub_21 = (l_grow_15).wrapping_add(l_nx_18);
                        l_va_23 = (l_grow_15).wrapping_add(l_dy_17);
                        l_vb_22 = (l_grow_15).wrapping_add(l_ny_19);
                        { let v_ = ((((((((2i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (c.m.r32(fp.wrapping_add(80))).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let v_ = ((14i32) as u64); let a_ = (c.m.r32(fp.wrapping_add(80))).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let a_ = fp.wrapping_add(80); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
                        { let v_ = ((((((c.m.r32((fp.wrapping_add(84)).wrapping_add((((((l_i_13) & (1i32))) as u32)).wrapping_mul(4)))) | (((536936448i32) as u32)))) as u64)) | ((((8i32) as u64)).wrapping_shl((30i32) as u32))); let a_ = (c.m.r32(fp.wrapping_add(80))).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let v_ = ((6i32) as u64); let a_ = (c.m.r32(fp.wrapping_add(80))).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let a_ = fp.wrapping_add(80); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
                        { let v_ = ((((c.m.r32((fp.wrapping_add(92)).wrapping_add(((((if ((((l_i_13) & (1i32))) != 0) { 0i32 } else { 1i32 })) as u32)).wrapping_mul(4)))) | (((262144i32) as u32)))) as u64); let a_ = (c.m.r32(fp.wrapping_add(80))).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let v_ = ((76i32) as u64); let a_ = (c.m.r32(fp.wrapping_add(80))).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let a_ = fp.wrapping_add(80); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
                        l_lastTbp_10 = ((c.m.r32((fp.wrapping_add(84)).wrapping_add(((((if ((((l_i_13) & (1i32))) != 0) { 0i32 } else { 1i32 })) as u32)).wrapping_mul(4)))) as i32);
                        F::Dma_PutTexStrips(c, fp.wrapping_add(80), 0i32, 0i32, l_half_6, l_h_9, 0i32, 0i32, 0i32, 0i32, l_half_6, l_h_9, 8i32, 8i32, 2155905152u32, 0i32);
                        { let v_ = ((((((((2i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (c.m.r32(fp.wrapping_add(80))).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let v_ = ((14i32) as u64); let a_ = (c.m.r32(fp.wrapping_add(80))).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let a_ = fp.wrapping_add(80); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
                        { let v_ = ((342i32) as u64); let a_ = (c.m.r32(fp.wrapping_add(80))).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let v_ = ((0i32) as u64); let a_ = (c.m.r32(fp.wrapping_add(80))).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let a_ = fp.wrapping_add(80); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
                        { let v_ = ((((c.m.r32((l_blur_4).wrapping_add(60))) as u64)) | ((((65024i32) as u64)).wrapping_shl((46i32) as u32))); let a_ = (c.m.r32(fp.wrapping_add(80))).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let v_ = ((1i32) as u64); let a_ = (c.m.r32(fp.wrapping_add(80))).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let a_ = fp.wrapping_add(80); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
                        { let v_ = ((4899916394579132417i64) as u64); let a_ = (c.m.r32(fp.wrapping_add(80))).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let v_ = ((21331i32) as u64); let a_ = (c.m.r32(fp.wrapping_add(80))).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let a_ = fp.wrapping_add(80); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
                        { let v_ = ((((((l_ua_20).wrapping_shl((4i32) as u32)).wrapping_add(8i32)) as u64)) | ((((((l_va_23).wrapping_shl((4i32) as u32)).wrapping_add(8i32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (c.m.r32(fp.wrapping_add(80))).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let v_ = ((1912631296i32) as u64); let a_ = (c.m.r32(fp.wrapping_add(80))).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let a_ = fp.wrapping_add(80); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
                        { let v_ = (((((((l_half_6).wrapping_sub(l_ub_21)).wrapping_shl((4i32) as u32)).wrapping_add(8i32)) as u64)) | (((((((l_h_9).wrapping_sub(l_vb_22)).wrapping_shl((4i32) as u32)).wrapping_add(8i32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (c.m.r32(fp.wrapping_add(80))).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let v_ = ((((((l_half_6).wrapping_add(1792i32)).wrapping_shl((4i32) as u32)) as u64)) | ((((32768i32) as u64)).wrapping_shl((16i32) as u32))); let a_ = (c.m.r32(fp.wrapping_add(80))).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let a_ = fp.wrapping_add(80); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
                    }
                    { { let o_ = l_i_13; let n_ = (o_).wrapping_add(1); l_i_13 = n_; }; { let r_ = l_f_14; let n_ = rt::fadd(l_f_14, r_); l_f_14 = n_; }; };
                }
            }
            { let v_ = ((((((((4i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (c.m.r32(fp.wrapping_add(80))).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((14i32) as u64); let a_ = (c.m.r32(fp.wrapping_add(80))).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let a_ = fp.wrapping_add(80); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
            { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { 524400i32 } else { 524288i32 })) as u64); let a_ = (c.m.r32(fp.wrapping_add(80))).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((76i32) as u64); let a_ = (c.m.r32(fp.wrapping_add(80))).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let a_ = fp.wrapping_add(80); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
            { let v_ = ((((((((l_xOffset_7) as u64)) | (((((l_x1_11).wrapping_sub(1i32)) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) | (((((l_srcH_8).wrapping_sub(1i32)) as u64)).wrapping_shl((48i32) as u32))); let a_ = (c.m.r32(fp.wrapping_add(80))).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((64i32) as u64); let a_ = (c.m.r32(fp.wrapping_add(80))).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let a_ = fp.wrapping_add(80); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
            { let v_ = ((((((l_lastTbp_10) | (536936448i32))) as u64)) | ((((8i32) as u64)).wrapping_shl((30i32) as u32))); let a_ = (c.m.r32(fp.wrapping_add(80))).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((6i32) as u64); let a_ = (c.m.r32(fp.wrapping_add(80))).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let a_ = fp.wrapping_add(80); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
            { let v_ = ((((((((((((2i32) as u64)) | ((((2i32) as u64)).wrapping_shl((2i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((4i32) as u32)))) | (((((l_half_6).wrapping_sub(1i32)) as u64)).wrapping_shl((14i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | (((((l_h_9).wrapping_sub(1i32)) as u64)).wrapping_shl((34i32) as u32))); let a_ = (c.m.r32(fp.wrapping_add(80))).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((8i32) as u64); let a_ = (c.m.r32(fp.wrapping_add(80))).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let a_ = fp.wrapping_add(80); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
            {
                l_cx_24 = rt::f2i(rt::fmul(rt::fadd(rt::fmul(F32(c.m.r32(fp)), F32(0x3f000000)), F32(0x3f000000)), rt::i2f(l_width_5)));
                l_cy_25 = rt::f2i(rt::fmul(rt::fadd(rt::fmul(F32(c.m.r32((fp).wrapping_add(4))), F32(0x3f000000)), F32(0x3f000000)), rt::i2f(l_srcH_8)));
                l_hx_26 = rt::div_i32(l_cx_24, 2i32);
                l_hy_27 = rt::div_i32(l_cy_25, 2i32);
                l_rx_28 = (l_width_5).wrapping_sub(l_cx_24);
                l_ry_29 = (l_srcH_8).wrapping_sub(l_cy_25);
                l_ru_30 = (l_half_6).wrapping_sub(l_hx_26);
                l_rv_31 = (l_h_9).wrapping_sub(l_hy_27);
                { let v_ = 0i32; let a_ = ((fp.wrapping_add(164)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = ((fp.wrapping_add(164)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = ((fp.wrapping_add(164)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = ((fp.wrapping_add(164)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = l_width_5; let a_ = ((fp.wrapping_add(164)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = ((fp.wrapping_add(164)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = l_half_6; let a_ = ((fp.wrapping_add(164)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = ((fp.wrapping_add(164)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = l_width_5; let a_ = ((fp.wrapping_add(164)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = l_srcH_8; let a_ = ((fp.wrapping_add(164)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = l_half_6; let a_ = ((fp.wrapping_add(164)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = l_h_9; let a_ = ((fp.wrapping_add(164)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = ((fp.wrapping_add(164)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = l_srcH_8; let a_ = ((fp.wrapping_add(164)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = ((fp.wrapping_add(164)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = l_h_9; let a_ = ((fp.wrapping_add(164)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = l_hx_26; let a_ = ((fp.wrapping_add(100)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = l_hy_27; let a_ = ((fp.wrapping_add(100)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = rt::div_i32(l_hx_26, 2i32); let a_ = ((fp.wrapping_add(100)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = rt::div_i32(l_hy_27, 2i32); let a_ = ((fp.wrapping_add(100)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = (l_cx_24).wrapping_add(rt::div_i32(l_rx_28, 2i32)); let a_ = ((fp.wrapping_add(100)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = l_hy_27; let a_ = ((fp.wrapping_add(100)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = (l_hx_26).wrapping_add(rt::div_i32(l_ru_30, 2i32)); let a_ = ((fp.wrapping_add(100)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = rt::div_i32(l_hy_27, 2i32); let a_ = ((fp.wrapping_add(100)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = (l_cx_24).wrapping_add(rt::div_i32(l_rx_28, 2i32)); let a_ = ((fp.wrapping_add(100)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = (l_cy_25).wrapping_add(rt::div_i32(l_ry_29, 2i32)); let a_ = ((fp.wrapping_add(100)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = (l_hx_26).wrapping_add(rt::div_i32(l_ru_30, 2i32)); let a_ = ((fp.wrapping_add(100)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = (l_hy_27).wrapping_add(rt::div_i32(l_rv_31, 2i32)); let a_ = ((fp.wrapping_add(100)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = l_hx_26; let a_ = ((fp.wrapping_add(100)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = (l_cy_25).wrapping_add(rt::div_i32(l_ry_29, 2i32)); let a_ = ((fp.wrapping_add(100)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = rt::div_i32(l_hx_26, 2i32); let a_ = ((fp.wrapping_add(100)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = (l_hy_27).wrapping_add(rt::div_i32(l_rv_31, 2i32)); let a_ = ((fp.wrapping_add(100)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                F::StgBlur_PutStripTag(c, fp.wrapping_add(80), 10i32, 1i32);
                {
                    l_i_13 = 0i32;
                    'l3: loop {
                        if !(((((l_i_13) < (5i32)) as i32)) != 0) { break 'l3; }
                        'c4: {
                            { let a1_ = ((c.m.r64((l_blur_4).wrapping_add(48))) as u32); F::stg_c__StgBlur_PutVertex(c, fp.wrapping_add(80), a1_, (l_cx_24).wrapping_add(l_xOffset_7), l_cy_25, l_hx_26, l_hy_27) };
                            { let a1_ = c.m.r32(((l_blur_4).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(4))); let a2_ = ((c.m.r32(((fp.wrapping_add(100)).wrapping_add((((rt::rem_i32(l_i_13, 4i32)) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)).wrapping_add(l_xOffset_7); let a3_ = (c.m.r32(((fp.wrapping_add(100)).wrapping_add((((rt::rem_i32(l_i_13, 4i32)) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); let a4_ = (c.m.r32(((fp.wrapping_add(100)).wrapping_add((((rt::rem_i32(l_i_13, 4i32)) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32); let a5_ = (c.m.r32(((fp.wrapping_add(100)).wrapping_add((((rt::rem_i32(l_i_13, 4i32)) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32); F::stg_c__StgBlur_PutVertex(c, fp.wrapping_add(80), a1_, a2_, a3_, a4_, a5_) };
                        }
                        { let o_ = l_i_13; let n_ = (o_).wrapping_add(1); l_i_13 = n_; };
                    }
                }
                F::StgBlur_Nop(c, fp.wrapping_add(80));
                F::StgBlur_PutStripTag(c, fp.wrapping_add(80), 10i32, 1i32);
                {
                    l_i_13 = 0i32;
                    'l5: loop {
                        if !(((((l_i_13) < (5i32)) as i32)) != 0) { break 'l5; }
                        'c6: {
                            { let a1_ = c.m.r32(((l_blur_4).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(4))); let a2_ = ((c.m.r32(((fp.wrapping_add(100)).wrapping_add((((rt::rem_i32(l_i_13, 4i32)) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)).wrapping_add(l_xOffset_7); let a3_ = (c.m.r32(((fp.wrapping_add(100)).wrapping_add((((rt::rem_i32(l_i_13, 4i32)) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); let a4_ = (c.m.r32(((fp.wrapping_add(100)).wrapping_add((((rt::rem_i32(l_i_13, 4i32)) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32); let a5_ = (c.m.r32(((fp.wrapping_add(100)).wrapping_add((((rt::rem_i32(l_i_13, 4i32)) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32); F::stg_c__StgBlur_PutVertex(c, fp.wrapping_add(80), a1_, a2_, a3_, a4_, a5_) };
                            { let a1_ = c.m.r32((l_blur_4).wrapping_add(56)); let a2_ = ((c.m.r32(((fp.wrapping_add(164)).wrapping_add((((rt::rem_i32(l_i_13, 4i32)) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)).wrapping_add(l_xOffset_7); let a3_ = (c.m.r32(((fp.wrapping_add(164)).wrapping_add((((rt::rem_i32(l_i_13, 4i32)) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); let a4_ = (c.m.r32(((fp.wrapping_add(164)).wrapping_add((((rt::rem_i32(l_i_13, 4i32)) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32); let a5_ = (c.m.r32(((fp.wrapping_add(164)).wrapping_add((((rt::rem_i32(l_i_13, 4i32)) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32); F::stg_c__StgBlur_PutVertex(c, fp.wrapping_add(80), a1_, a2_, a3_, a4_, a5_) };
                        }
                        { let o_ = l_i_13; let n_ = (o_).wrapping_add(1); l_i_13 = n_; };
                    }
                }
                F::StgBlur_Nop(c, fp.wrapping_add(80));
            }
        }
        F::StgBlur_EndDraw(c, fp.wrapping_add(80));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn StgBlur_DrawAll(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_DrawAll", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlCam)) == (((0i32) as u32))) as i32)) != 0) {
        { let a2_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(64); F::StgBlur_Draw(c, 0i32, 0i32, a2_) };
        break 'f;
    }
    if ((F::Battle_IsSplitScreen(c)) != 0) {
        if ((F::DemoCam_IsActive(c)) != 0) {
            { let a2_ = (((c.m.r32(sym::gBtlCam)).wrapping_add(1824)).wrapping_add((((0i32) as u32)).wrapping_mul(656))).wrapping_add(64); F::StgBlur_Draw(c, 0i32, 0i32, a2_) };
            break 'f;
        }
        if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gBtlCam)).wrapping_add(3136); c.m.r32(a_) }).wrapping_add(640); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            { let a1_ = { let a_ = ({ let a_ = (c.m.r32(sym::gBtlCam)).wrapping_add(3136); c.m.r32(a_) }).wrapping_add(644); (c.m.r32(a_) as i32) }; let a2_ = (((c.m.r32(sym::gBtlCam)).wrapping_add(1824)).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gBtlCam)).wrapping_add(3136); c.m.r32(a_) }).wrapping_add(644); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(656))).wrapping_add(64); F::StgBlur_Draw(c, 0i32, a1_, a2_) };
            break 'f;
        }
        { let a2_ = (((c.m.r32(sym::gBtlCam)).wrapping_add(1824)).wrapping_add((((0i32) as u32)).wrapping_mul(656))).wrapping_add(64); F::StgBlur_Draw(c, 1i32, 0i32, a2_) };
        { let a2_ = (((c.m.r32(sym::gBtlCam)).wrapping_add(1824)).wrapping_add((((1i32) as u32)).wrapping_mul(656))).wrapping_add(64); F::StgBlur_Draw(c, 1i32, 1i32, a2_) };
        break 'f;
    }
    if ((F::DemoCam_IsActive(c)) != 0) {
        { let a2_ = (((c.m.r32(sym::gBtlCam)).wrapping_add(1824)).wrapping_add((((0i32) as u32)).wrapping_mul(656))).wrapping_add(64); F::StgBlur_Draw(c, 0i32, 0i32, a2_) };
        break 'f;
    }
    { let a1_ = { let a_ = ({ let a_ = (c.m.r32(sym::gBtlCam)).wrapping_add(3136); c.m.r32(a_) }).wrapping_add(644); (c.m.r32(a_) as i32) }; let a2_ = (((c.m.r32(sym::gBtlCam)).wrapping_add(1824)).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gBtlCam)).wrapping_add(3136); c.m.r32(a_) }).wrapping_add(644); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(656))).wrapping_add(64); F::StgBlur_Draw(c, 0i32, a1_, a2_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_DrawAll", &PROBE_N, &[], true); }
    r_
}

pub fn StgBlur_SetPasses(c: &mut Ctx, mut p_view_1: i32, mut p_passes_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_SetPasses", &PROBE_N, &[(p_view_1) as u32, (p_passes_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_passes_2; let a_ = ((c.m.r32(sym::gStgBlur)).wrapping_add((((p_view_1) as u32)).wrapping_mul(80))).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_SetPasses", &PROBE_N, &[], true); }
    r_
}

pub fn StgBlur_SetScale(c: &mut Ctx, mut p_view_1: i32, mut p_scale_2: F32, mut p_shiftX_3: F32, mut p_shiftY_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_SetScale", &PROBE_N, &[(p_view_1) as u32, (p_scale_2).0, (p_shiftX_3).0, (p_shiftY_4).0], false); }
    let r_ = 'f: {
    { let v_ = p_scale_2; let a_ = ((c.m.r32(sym::gStgBlur)).wrapping_add((((p_view_1) as u32)).wrapping_mul(80))).wrapping_add(32); c.m.w32(a_, (v_).0); };
    { let v_ = p_shiftX_3; let a_ = ((c.m.r32(sym::gStgBlur)).wrapping_add((((p_view_1) as u32)).wrapping_mul(80))).wrapping_add(40); c.m.w32(a_, (v_).0); };
    { let v_ = p_shiftY_4; let a_ = ((c.m.r32(sym::gStgBlur)).wrapping_add((((p_view_1) as u32)).wrapping_mul(80))).wrapping_add(44); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_SetScale", &PROBE_N, &[], true); }
    r_
}

pub fn StgBlur_SetCenter(c: &mut Ctx, mut p_view_1: i32, mut p_center_2: u32, mut p_screenSpace_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_SetCenter", &PROBE_N, &[(p_view_1) as u32, (p_center_2) as u32, (p_screenSpace_3) as u32], false); }
    let r_ = 'f: {
    { let a0_ = ((c.m.r32(sym::gStgBlur)).wrapping_add((((p_view_1) as u32)).wrapping_mul(80))).wrapping_add(16); F::Vec3_Normalize(c, a0_, p_center_2) };
    { let v_ = p_screenSpace_3; let a_ = ((c.m.r32(sym::gStgBlur)).wrapping_add((((p_view_1) as u32)).wrapping_mul(80))).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_SetCenter", &PROBE_N, &[], true); }
    r_
}

pub fn StgBlur_SetColor0Rgba(c: &mut Ctx, mut p_view_1: i32, mut p_r_2: u8, mut p_g_3: u8, mut p_b_4: u8, mut p_a_5: u8) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_SetColor0Rgba", &PROBE_N, &[(p_view_1) as u32, (p_r_2) as u32, (p_g_3) as u32, (p_b_4) as u32, (p_a_5) as u32], false); }
    let r_ = 'f: {
    F::StgBlur_SetColor0(c, p_view_1, ((((((((((p_r_2) as i32)) | ((((p_g_3) as i32)).wrapping_shl((8i32) as u32)))) | ((((p_b_4) as i32)).wrapping_shl((16i32) as u32)))) | ((((p_a_5) as i32)).wrapping_shl((24i32) as u32)))) as u32));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_SetColor0Rgba", &PROBE_N, &[], true); }
    r_
}

pub fn StgBlur_SetColor0(c: &mut Ctx, mut p_view_1: i32, mut p_rgba_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_SetColor0", &PROBE_N, &[(p_view_1) as u32, (p_rgba_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_rgba_2; let a_ = ((c.m.r32(sym::gStgBlur)).wrapping_add((((p_view_1) as u32)).wrapping_mul(80))).wrapping_add(48); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_SetColor0", &PROBE_N, &[], true); }
    r_
}

pub fn StgBlur_SetColor1Rgba(c: &mut Ctx, mut p_view_1: i32, mut p_r_2: u8, mut p_g_3: u8, mut p_b_4: u8, mut p_a_5: u8) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_SetColor1Rgba", &PROBE_N, &[(p_view_1) as u32, (p_r_2) as u32, (p_g_3) as u32, (p_b_4) as u32, (p_a_5) as u32], false); }
    let r_ = 'f: {
    F::StgBlur_SetColor1(c, p_view_1, ((((((((((p_r_2) as i32)) | ((((p_g_3) as i32)).wrapping_shl((8i32) as u32)))) | ((((p_b_4) as i32)).wrapping_shl((16i32) as u32)))) | ((((p_a_5) as i32)).wrapping_shl((24i32) as u32)))) as u32));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_SetColor1Rgba", &PROBE_N, &[], true); }
    r_
}

pub fn StgBlur_SetColor1(c: &mut Ctx, mut p_view_1: i32, mut p_rgba_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_SetColor1", &PROBE_N, &[(p_view_1) as u32, (p_rgba_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_rgba_2; let a_ = (((c.m.r32(sym::gStgBlur)).wrapping_add((((p_view_1) as u32)).wrapping_mul(80))).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_SetColor1", &PROBE_N, &[], true); }
    r_
}

pub fn StgBlur_SetColor2Rgba(c: &mut Ctx, mut p_view_1: i32, mut p_r_2: u8, mut p_g_3: u8, mut p_b_4: u8, mut p_a_5: u8) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_SetColor2Rgba", &PROBE_N, &[(p_view_1) as u32, (p_r_2) as u32, (p_g_3) as u32, (p_b_4) as u32, (p_a_5) as u32], false); }
    let r_ = 'f: {
    F::StgBlur_SetColor2(c, p_view_1, ((((((((((p_r_2) as i32)) | ((((p_g_3) as i32)).wrapping_shl((8i32) as u32)))) | ((((p_b_4) as i32)).wrapping_shl((16i32) as u32)))) | ((((p_a_5) as i32)).wrapping_shl((24i32) as u32)))) as u32));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_SetColor2Rgba", &PROBE_N, &[], true); }
    r_
}

pub fn StgBlur_SetColor2(c: &mut Ctx, mut p_view_1: i32, mut p_rgba_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_SetColor2", &PROBE_N, &[(p_view_1) as u32, (p_rgba_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_rgba_2; let a_ = ((c.m.r32(sym::gStgBlur)).wrapping_add((((p_view_1) as u32)).wrapping_mul(80))).wrapping_add(56); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_SetColor2", &PROBE_N, &[], true); }
    r_
}

pub fn StgBlur_SetColor3Rgba(c: &mut Ctx, mut p_view_1: i32, mut p_r_2: u8, mut p_g_3: u8, mut p_b_4: u8, mut p_a_5: u8) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_SetColor3Rgba", &PROBE_N, &[(p_view_1) as u32, (p_r_2) as u32, (p_g_3) as u32, (p_b_4) as u32, (p_a_5) as u32], false); }
    let r_ = 'f: {
    F::StgBlur_SetColor3(c, p_view_1, ((((((((((p_r_2) as i32)) | ((((p_g_3) as i32)).wrapping_shl((8i32) as u32)))) | ((((p_b_4) as i32)).wrapping_shl((16i32) as u32)))) | ((((p_a_5) as i32)).wrapping_shl((24i32) as u32)))) as u32));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_SetColor3Rgba", &PROBE_N, &[], true); }
    r_
}

pub fn StgBlur_SetColor3(c: &mut Ctx, mut p_view_1: i32, mut p_rgba_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_SetColor3", &PROBE_N, &[(p_view_1) as u32, (p_rgba_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_rgba_2; let a_ = ((c.m.r32(sym::gStgBlur)).wrapping_add((((p_view_1) as u32)).wrapping_mul(80))).wrapping_add(60); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgBlur_SetColor3", &PROBE_N, &[], true); }
    r_
}
