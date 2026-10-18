#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EftBurst_DrawQuad(c: &mut Ctx, mut p_p_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_DrawQuad", &PROBE_N, &[(p_p_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(208) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 208);
    let mut l_pkt_2: u32 = 0;
    let mut l_vis_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_scale_5: F32 = F32(0);
    let mut l_life_6: i32 = 0;
    let mut l_lifeMax_7: i32 = 0;
    let mut l_pc_8: u32 = 0;
    let mut l_ps_9: u32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp.wrapping_add(16), 0, 64);
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0), 0, 16);
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(16).wrapping_add(16), 0, 16);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(0), (v_).0); }
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(16).wrapping_add(32), 0, 16);
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(16).wrapping_add(32).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(32).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(32).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(32).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(16).wrapping_add(48), 0, 16);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(48).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(48).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(48).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(48).wrapping_add(12), (v_).0); }
        l_vis_3 = 0i32;
        { let _ = F::memset(c, fp.wrapping_add(80), 0i32, 64u32); };
        { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(80)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(80)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(80)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(80)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        l_pc_8 = fp.wrapping_add(16);
        l_scale_5 = F32(c.m.r32((p_p_1).wrapping_add(76)));
        l_ps_9 = fp.wrapping_add(144);
        l_life_6 = (c.m.r32((p_p_1).wrapping_add(64)) as i32);
        l_lifeMax_7 = (c.m.r32((p_p_1).wrapping_add(68)) as i32);
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::Vec4_Scale(c, fp, (l_pc_8).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)), l_scale_5);
                    F::Vec4_Add(c, fp, fp, p_p_1);
                    { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let _ = { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Mtx_ProjectPoint(c, (l_ps_9).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)), a1_, fp) }; };
                    { let r_ = ((((c.m.r32(((fp.wrapping_add(144)).wrapping_add((((l_i_4) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) > (0i32)) as i32); let n_ = (l_vis_3).wrapping_add(r_); l_vis_3 = n_; };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        if (((((l_vis_3) != (0i32)) as i32)) != 0) {
            { let a0_ = c.m.r32((p_p_1).wrapping_add(72)); F::EftBurst_SetTexture(c, a0_, 11520i32, 10752i32) };
            l_pkt_2 = F::Dma_BeginDirect(c);
            { let v_ = ((1152921504606879748i64) as u64); let a_ = (l_pkt_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((14i32) as u64); let a_ = (l_pkt_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_pkt_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_2 = n_; };
            { let v_ = ((5117051104i64) as u64); let a_ = (l_pkt_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((78i32) as u64); let a_ = (l_pkt_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_pkt_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_2 = n_; };
            { let v_ = ((72i32) as u64); let a_ = (l_pkt_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((66i32) as u64); let a_ = (l_pkt_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_pkt_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_2 = n_; };
            { let v_ = (({ let a_ = (c.m.r32((p_p_1).wrapping_add(72))).wrapping_add(48); c.m.r64(a_) }) | (((1477760807611648i64) as u64))); let a_ = (l_pkt_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((6i32) as u64); let a_ = (l_pkt_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_pkt_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_2 = n_; };
            { let v_ = ((471043i32) as u64); let a_ = (l_pkt_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((71i32) as u64); let a_ = (l_pkt_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_pkt_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_2 = n_; };
            { let v_ = ((2594073385365438465i64) as u64); let a_ = (l_pkt_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((16i32) as u64); let a_ = (l_pkt_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_pkt_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_2 = n_; };
            { let v_ = ((84i32) as u64); let a_ = (l_pkt_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            if (((((l_lifeMax_7) < ((l_life_6).wrapping_mul(2i32))) as i32)) != 0) {
                { let v_ = (((((((rt::div_i32(((l_lifeMax_7).wrapping_sub(l_life_6)).wrapping_mul(16i32), l_lifeMax_7)) as i64)).wrapping_shl((24i32) as u32)) | (4575657221416845440i64))) as u64); let a_ = (l_pkt_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            } else {
                { let v_ = (((((((rt::div_i32((l_life_6).wrapping_mul(16i32), l_lifeMax_7)) as i64)).wrapping_shl((24i32) as u32)) | (4575657221416845440i64))) as u64); let a_ = (l_pkt_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            }
            { let r_ = 2i32; let n_ = (l_pkt_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_2 = n_; };
            { let v_ = 9511602413006520321u64; let a_ = (l_pkt_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((1381126738i32) as u64); let a_ = (l_pkt_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_pkt_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_2 = n_; };
            {
                l_i_4 = 0i32;
                'l3: loop {
                    if !(((((l_i_4) < (4i32)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        { let v_ = ((((c.m.r32(((fp.wrapping_add(80)).wrapping_add((((l_i_4) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))) as u64)) | ((((c.m.r32(((fp.wrapping_add(80)).wrapping_add((((l_i_4) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_pkt_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let v_ = (((((((((c.m.r32(((l_ps_9).wrapping_add((((l_i_4) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as i64)) | (((((c.m.r32(((l_ps_9).wrapping_add((((l_i_4) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as i64)).wrapping_shl((16i32) as u32)))) | (((((c.m.r32(((l_ps_9).wrapping_add((((l_i_4) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as i64)).wrapping_shl((32i32) as u32)))) as u64); let a_ = (l_pkt_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let r_ = 2i32; let n_ = (l_pkt_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_2 = n_; };
                    }
                    { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                }
            }
            F::Dma_EndDirect(c, l_pkt_2);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_DrawQuad", &PROBE_N, &[], true); }
    r_
}

pub fn EftBurst_DrawQuadRot(c: &mut Ctx, mut p_p_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_DrawQuadRot", &PROBE_N, &[(p_p_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(336) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 336);
    let mut l_pkt_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_flags_4: i32 = 0;
    let mut l_scale_5: F32 = F32(0);
    let mut l_life_6: i32 = 0;
    let mut l_vis_7: i32 = 0;
    let mut l_lifeMax_8: i32 = 0;
    let mut l_alpha_9: i32 = 0;
    let mut l_scaleY_10: i32 = 0;
    let mut l_rotate_11: i32 = 0;
    let mut l_scaleX_12: i32 = 0;
    let mut l_ps_13: u32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp.wrapping_add(16), 0, 64);
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0), 0, 16);
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(16).wrapping_add(16), 0, 16);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(0), (v_).0); }
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(16).wrapping_add(32), 0, 16);
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(16).wrapping_add(32).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(32).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(32).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(32).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(16).wrapping_add(48), 0, 16);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(48).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(48).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(48).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(48).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(80), 0, 64);
        rt::memset(c, fp.wrapping_add(80).wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(80).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(80).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(80).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(80).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(80).wrapping_add(16), 0, 16);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(80).wrapping_add(16).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(80).wrapping_add(16).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(80).wrapping_add(16).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(80).wrapping_add(16).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(80).wrapping_add(32), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(80).wrapping_add(32).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(80).wrapping_add(32).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(80).wrapping_add(32).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(80).wrapping_add(32).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(80).wrapping_add(48), 0, 16);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(80).wrapping_add(48).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(80).wrapping_add(48).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(80).wrapping_add(48).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(80).wrapping_add(48).wrapping_add(12), (v_).0); }
        l_vis_7 = 0i32;
        l_ps_13 = fp.wrapping_add(144);
        l_flags_4 = (c.m.r32((p_p_1).wrapping_add(80)) as i32);
        l_scaleX_12 = ((l_flags_4) & (16i32));
        l_scaleY_10 = ((l_flags_4) & (32i32));
        l_rotate_11 = ((l_flags_4) & (128i32));
        l_scale_5 = F32(c.m.r32((p_p_1).wrapping_add(76)));
        l_life_6 = (c.m.r32((p_p_1).wrapping_add(64)) as i32);
        l_lifeMax_8 = (c.m.r32((p_p_1).wrapping_add(68)) as i32);
        l_alpha_9 = (c.m.r32((p_p_1).wrapping_add(96)) as i32);
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::Vec4_Copy(c, fp, (fp.wrapping_add(16)).wrapping_add((((l_i_3) as u32)).wrapping_mul(16)));
                    if ((l_scaleX_12) != 0) {
                        { let r_ = l_scale_5; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    }
                    if ((l_scaleY_10) != 0) {
                        { let r_ = l_scale_5; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    }
                    if ((l_rotate_11) != 0) {
                        F::Mtx_StoreIdentity(c, fp.wrapping_add(208));
                        { let a2_ = F32(c.m.r32(((p_p_1).wrapping_add(32)).wrapping_add(4))); F::Mtx_RotateY(c, fp.wrapping_add(208), fp.wrapping_add(208), a2_) };
                        F::Mtx_StoreIdentity(c, fp.wrapping_add(272));
                        { let a2_ = F32(c.m.r32(((p_p_1).wrapping_add(32)).wrapping_add(8))); F::Mtx_RotateZ(c, fp.wrapping_add(272), fp.wrapping_add(272), a2_) };
                        F::Mtx_Mul(c, fp.wrapping_add(208), fp.wrapping_add(272), fp.wrapping_add(208));
                        F::Mtx_StoreIdentity(c, fp.wrapping_add(272));
                        { let a2_ = F32(c.m.r32((p_p_1).wrapping_add(32))); F::Mtx_RotateY(c, fp.wrapping_add(272), fp.wrapping_add(272), a2_) };
                        F::Mtx_Mul(c, fp.wrapping_add(208), fp.wrapping_add(208), fp.wrapping_add(272));
                        F::Mtx_MulVec4(c, fp, fp.wrapping_add(208), fp);
                    }
                    F::Vec4_Add(c, fp, fp, p_p_1);
                    { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let _ = { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Mtx_ProjectPoint(c, (l_ps_13).wrapping_add((((l_i_3) as u32)).wrapping_mul(16)), a1_, fp) }; };
                    { let r_ = ((((c.m.r32(((fp.wrapping_add(144)).wrapping_add((((l_i_3) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) > (0i32)) as i32); let n_ = (l_vis_7).wrapping_add(r_); l_vis_7 = n_; };
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        if (((((l_vis_7) != (0i32)) as i32)) != 0) {
            { let a0_ = c.m.r32((p_p_1).wrapping_add(72)); F::EftBurst_SetTexture(c, a0_, 11520i32, 10752i32) };
            l_pkt_2 = F::Dma_BeginDirect(c);
            { let v_ = ((1152921504606879748i64) as u64); let a_ = (l_pkt_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((14i32) as u64); let a_ = (l_pkt_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_pkt_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_2 = n_; };
            { let v_ = ((5117051104i64) as u64); let a_ = (l_pkt_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((78i32) as u64); let a_ = (l_pkt_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_pkt_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_2 = n_; };
            if (((((c.m.r32((p_p_1).wrapping_add(80)) as i32)) & (8i32))) != 0) {
                { let v_ = ((68i32) as u64); let a_ = (l_pkt_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let v_ = ((66i32) as u64); let a_ = (l_pkt_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let r_ = 2i32; let n_ = (l_pkt_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_2 = n_; };
            } else {
                { let v_ = ((72i32) as u64); let a_ = (l_pkt_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let v_ = ((66i32) as u64); let a_ = (l_pkt_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let r_ = 2i32; let n_ = (l_pkt_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_2 = n_; };
            }
            { let v_ = (({ let a_ = (c.m.r32((p_p_1).wrapping_add(72))).wrapping_add(48); c.m.r64(a_) }) | (((1477760807611648i64) as u64))); let a_ = (l_pkt_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((6i32) as u64); let a_ = (l_pkt_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_pkt_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_2 = n_; };
            { let v_ = ((471043i32) as u64); let a_ = (l_pkt_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((71i32) as u64); let a_ = (l_pkt_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_pkt_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_2 = n_; };
            { let v_ = ((2594073385365438465i64) as u64); let a_ = (l_pkt_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((16i32) as u64); let a_ = (l_pkt_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_pkt_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_2 = n_; };
            { let v_ = ((84i32) as u64); let a_ = (l_pkt_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            if ((((l_flags_4) & (64i32))) != 0) {
                { let v_ = ((((((c.m.r32((p_p_1).wrapping_add(92))) as u64)) | ((((((rt::div_i32((l_alpha_9).wrapping_mul(l_life_6), l_lifeMax_8)) as i64)).wrapping_shl((24i32) as u32)) as u64)))) | (((4575657221408423936i64) as u64))); let a_ = (l_pkt_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            } else {
                { let v_ = (((((((rt::div_i32((l_alpha_9).wrapping_mul(l_life_6), l_lifeMax_8)) as i64)).wrapping_shl((24i32) as u32)) | (4575657221416833104i64))) as u64); let a_ = (l_pkt_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            }
            { let r_ = 2i32; let n_ = (l_pkt_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_2 = n_; };
            { let v_ = ((2594073385365438468i64) as u64); let a_ = (l_pkt_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((82i32) as u64); let a_ = (l_pkt_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_pkt_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_2 = n_; };
            {
                l_i_3 = 0i32;
                'l3: loop {
                    if !(((((l_i_3) < (4i32)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        { let v_ = ((((c.m.r32(((fp.wrapping_add(80)).wrapping_add((((l_i_3) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))) as u64)) | ((((c.m.r32(((fp.wrapping_add(80)).wrapping_add((((l_i_3) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_pkt_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let v_ = (((((((((c.m.r32(((fp.wrapping_add(144)).wrapping_add((((l_i_3) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as i64)) | (((((c.m.r32(((fp.wrapping_add(144)).wrapping_add((((l_i_3) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as i64)).wrapping_shl((16i32) as u32)))) | (((((c.m.r32(((fp.wrapping_add(144)).wrapping_add((((l_i_3) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as i64)).wrapping_shl((32i32) as u32)))) as u64); let a_ = (l_pkt_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let r_ = 2i32; let n_ = (l_pkt_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_2 = n_; };
                    }
                    { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                }
            }
            F::Dma_EndDirect(c, l_pkt_2);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_DrawQuadRot", &PROBE_N, &[], true); }
    r_
}

pub fn EftBurst_DrawSprite(c: &mut Ctx, mut p_p_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_DrawSprite", &PROBE_N, &[(p_p_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(176) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 176);
    let mut l_pkt_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_vis_4: i32 = 0;
    let mut l_scale_5: F32 = F32(0);
    let mut l_life_6: i32 = 0;
    let mut l_alpha_7: i32 = 0;
    let mut l_lifeMax_8: i32 = 0;
    let mut l_flags_9: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp.wrapping_add(16), 0, 32);
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0), 0, 16);
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(16).wrapping_add(16), 0, 16);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(48), 0, 32);
        rt::memset(c, fp.wrapping_add(48).wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(48).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(48).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(48).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(48).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(48).wrapping_add(16), 0, 16);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(48).wrapping_add(16).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(48).wrapping_add(16).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(48).wrapping_add(16).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(48).wrapping_add(16).wrapping_add(12), (v_).0); }
        l_vis_4 = 0i32;
        l_scale_5 = F32(c.m.r32((p_p_1).wrapping_add(76)));
        l_life_6 = (c.m.r32((p_p_1).wrapping_add(64)) as i32);
        l_lifeMax_8 = (c.m.r32((p_p_1).wrapping_add(68)) as i32);
        l_alpha_7 = (c.m.r32((p_p_1).wrapping_add(96)) as i32);
        l_flags_9 = (c.m.r32((p_p_1).wrapping_add(80)) as i32);
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(64); F::Mtx_Transpose(c, fp.wrapping_add(112), a1_) };
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::Mtx_MulVec4(c, fp, fp.wrapping_add(112), (fp.wrapping_add(16)).wrapping_add((((l_i_3) as u32)).wrapping_mul(16)));
                    F::Vec4_Scale(c, fp, fp, l_scale_5);
                    F::Vec4_Add(c, fp, fp, p_p_1);
                    { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let _ = { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Mtx_ProjectPoint(c, (fp.wrapping_add(80)).wrapping_add((((l_i_3) as u32)).wrapping_mul(16)), a1_, fp) }; };
                    { let r_ = ((((c.m.r32(((fp.wrapping_add(80)).wrapping_add((((l_i_3) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) > (0i32)) as i32); let n_ = (l_vis_4).wrapping_add(r_); l_vis_4 = n_; };
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        if (((((l_vis_4) != (0i32)) as i32)) != 0) {
            { let a0_ = c.m.r32((p_p_1).wrapping_add(72)); F::EftBurst_SetTexture(c, a0_, 11520i32, 10752i32) };
            l_pkt_2 = F::Dma_BeginDirect(c);
            { let v_ = ((1152921504606879748i64) as u64); let a_ = (l_pkt_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((14i32) as u64); let a_ = (l_pkt_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_pkt_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_2 = n_; };
            { let v_ = ((5117051104i64) as u64); let a_ = (l_pkt_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((78i32) as u64); let a_ = (l_pkt_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_pkt_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_2 = n_; };
            if ((((l_flags_9) & (8i32))) != 0) {
                { let v_ = ((68i32) as u64); let a_ = (l_pkt_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let v_ = ((66i32) as u64); let a_ = (l_pkt_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let r_ = 2i32; let n_ = (l_pkt_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_2 = n_; };
            } else {
                if ((((l_flags_9) & (256i32))) != 0) {
                    { let v_ = ((66i32) as u64); let a_ = (l_pkt_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = ((66i32) as u64); let a_ = (l_pkt_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = 2i32; let n_ = (l_pkt_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_2 = n_; };
                } else {
                    { let v_ = ((72i32) as u64); let a_ = (l_pkt_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = ((66i32) as u64); let a_ = (l_pkt_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = 2i32; let n_ = (l_pkt_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_2 = n_; };
                }
            }
            { let v_ = (({ let a_ = (c.m.r32((p_p_1).wrapping_add(72))).wrapping_add(48); c.m.r64(a_) }) | (((1477760807611648i64) as u64))); let a_ = (l_pkt_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((6i32) as u64); let a_ = (l_pkt_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_pkt_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_2 = n_; };
            { let v_ = ((208899i32) as u64); let a_ = (l_pkt_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((71i32) as u64); let a_ = (l_pkt_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_pkt_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_2 = n_; };
            { let v_ = ((7205759403792826369i64) as u64); let a_ = (l_pkt_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((1381126672i32) as u64); let a_ = (l_pkt_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_pkt_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_2 = n_; };
            { let v_ = ((86i32) as u64); let a_ = (l_pkt_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            if ((((l_flags_9) & (64i32))) != 0) {
                { let v_ = ((((((c.m.r32((p_p_1).wrapping_add(92))) as u64)) | ((((((l_alpha_7) as i64)).wrapping_shl((24i32) as u32)) as u64)))) | (((4575657221408423936i64) as u64))); let a_ = (l_pkt_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            } else {
                { let v_ = (((((((rt::div_i32((l_life_6).wrapping_mul(80i32), l_lifeMax_8)) as i64)).wrapping_shl((24i32) as u32)) | (4575657221413687376i64))) as u64); let a_ = (l_pkt_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            }
            { let r_ = 2i32; let n_ = (l_pkt_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_2 = n_; };
            { let v_ = ((((c.m.r32(((fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))) as u64)) | ((((c.m.r32(((fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_pkt_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = (((((((((c.m.r32(((fp.wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as i64)) | (((((c.m.r32(((fp.wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as i64)).wrapping_shl((16i32) as u32)))) | (((((c.m.r32(((fp.wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as i64)).wrapping_shl((32i32) as u32)))) as u64); let a_ = (l_pkt_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_pkt_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_2 = n_; };
            { let v_ = ((((c.m.r32(((fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))) as u64)) | ((((c.m.r32(((fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_pkt_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = (((((((((c.m.r32(((fp.wrapping_add(80)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as i64)) | (((((c.m.r32(((fp.wrapping_add(80)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as i64)).wrapping_shl((16i32) as u32)))) | (((((c.m.r32(((fp.wrapping_add(80)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as i64)).wrapping_shl((32i32) as u32)))) as u64); let a_ = (l_pkt_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_pkt_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_2 = n_; };
            F::Dma_EndDirect(c, l_pkt_2);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_DrawSprite", &PROBE_N, &[], true); }
    r_
}

pub fn EftBurst_RelocateModel(c: &mut Ctx, mut p_model_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_RelocateModel", &PROBE_N, &[(p_model_1) as u32], false); }
    let mut l_grp_2: u32 = 0;
    let mut l_tex_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_n_5: i32 = 0;
    let r_ = 'f: {
    l_n_5 = (c.m.r32((p_model_1).wrapping_add(4)) as i32);
    if ((((((c.m.r32((p_model_1).wrapping_add(20)) as i32)) != (1i32)) as i32)) != 0) {
        { let v_ = 1i32; let a_ = (p_model_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
        { let v_ = (c.m.r32((p_model_1).wrapping_add(8))).wrapping_add(((((p_model_1) as i32)) as u32)); let a_ = (p_model_1).wrapping_add(8); c.m.w32(a_, v_); };
        l_grp_2 = c.m.r32((p_model_1).wrapping_add(8));
        if (((((l_n_5) > (0i32)) as i32)) != 0) {
            l_i_4 = l_n_5;
            'l1: loop {
                'c2: {
                    { let v_ = (c.m.r32((l_grp_2).wrapping_add(8))).wrapping_add(((((p_model_1) as i32)) as u32)); let a_ = (l_grp_2).wrapping_add(8); c.m.w32(a_, v_); };
                    { let v_ = (c.m.r32((l_grp_2).wrapping_add(12))).wrapping_add(((((p_model_1) as i32)) as u32)); let a_ = (l_grp_2).wrapping_add(12); c.m.w32(a_, v_); };
                    { let o_ = l_grp_2; let n_ = (o_).wrapping_add(24); l_grp_2 = n_; };
                }
                if !((((({ let o_ = l_i_4; let n_ = (o_).wrapping_sub(1); l_i_4 = n_; n_ }) != (0i32)) as i32)) != 0) { break 'l1; }
            }
        }
        { let v_ = (c.m.r32((p_model_1).wrapping_add(16))).wrapping_add(((((p_model_1) as i32)) as u32)); let a_ = (p_model_1).wrapping_add(16); c.m.w32(a_, v_); };
        l_tex_3 = c.m.r32((p_model_1).wrapping_add(16));
        {
            l_i_4 = 0i32;
            'l3: loop {
                if !(((((l_i_4) < ((c.m.r32((p_model_1).wrapping_add(12)) as i32))) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let v_ = ((c.m.r32(l_tex_3) as i32)).wrapping_add(((p_model_1) as i32)); let a_ = (l_tex_3).wrapping_add(56); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32((l_tex_3).wrapping_add(4)) as i32)).wrapping_add(((p_model_1) as i32)); let a_ = (l_tex_3).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
                    { let o_ = l_tex_3; let n_ = (o_).wrapping_add(64); l_tex_3 = n_; };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_RelocateModel", &PROBE_N, &[], true); }
    r_
}

pub fn EftBurst_PutModelEnv(c: &mut Ctx, mut p_p_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_PutModelEnv", &PROBE_N, &[(p_p_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((1152921504606879750i64) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((96i32) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((20i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((327693i32) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((71i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((822083808i32) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((78i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((68i32) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((66i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((1i32) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((70i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = (if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { 18374686479672148080u64 } else { 18374686479672147968u64 }); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((76i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((1152921504606879750i64) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((96i32) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((21i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((327693i32) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((72i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((822083808i32) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((79i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((68i32) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((67i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((1i32) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((70i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { 524400i32 } else { 524288i32 })) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((77i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    break 'f p_p_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_PutModelEnv", &PROBE_N, &[], true); }
    r_
}

pub fn EftBurst_PutModelEnvEnd(c: &mut Ctx, mut p_p_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_PutModelEnvEnd", &PROBE_N, &[(p_p_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((1152921504606879749i64) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((0i32) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((20i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((196608i32) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((71i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((822083808i32) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((78i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((1i32) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((70i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { 524400i32 } else { 524288i32 })) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((76i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    break 'f p_p_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_PutModelEnvEnd", &PROBE_N, &[], true); }
    r_
}

pub fn EftBurst_ClearAlphaPlane(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_ClearAlphaPlane", &PROBE_N, &[], false); }
    let mut l_p_1: u32 = 0;
    let r_ = 'f: {
    l_p_1 = F::Dma_BeginDirect(c);
    { let v_ = ((1152921504606879753i64) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((68i32) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((67i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((96i32) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((21i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((0i32) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((75i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((196608i32) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((72i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((5i32) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((9i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((5117051104i64) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((79i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { 72057589743485040i64 } else { 72057589743484928i64 })) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((77i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((125344325595136i64) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((25i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((125819314623152128i64) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((65i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((4899916394579132417i64) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((21776i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((518i32) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((4575657221408423936i64) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((1912631296i32) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((2382401536u32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    F::Dma_EndDirect(c, l_p_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_ClearAlphaPlane", &PROBE_N, &[], true); }
    r_
}

pub fn eft_d_b__EftBurst_IsVisible(c: &mut Ctx, mut p_pos_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_d_b__EftBurst_IsVisible", &PROBE_N, &[(p_pos_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
    l_p_2 = p_pos_1;
    if ((((((c.m.r32((l_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) <= (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((c.m.r32((l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) > (38911i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((c.m.r32((l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) < (26625i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((c.m.r32((l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) > (38143i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((c.m.r32((l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) < (27393i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "eft_d_b__EftBurst_IsVisible", &PROBE_N, &[], true); }
    r_
}

pub fn EftBurst_DrawModel(c: &mut Ctx, mut p_model_1: u32, mut p_unused_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_DrawModel", &PROBE_N, &[(p_model_1) as u32, (p_unused_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(384) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 384);
    let mut l_grp_3: u32 = 0;
    let mut l_texs_4: u32 = 0;
    let mut l_envTex_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_verts_7: u32 = 0;
    let mut l_tri_8: u32 = 0;
    let mut l_tex_9: u32 = 0;
    let mut l_t_10: i32 = 0;
    let mut l_vis_11: i32 = 0;
    let mut l_env_12: i32 = 0;
    let mut l_pkt_13: u32 = 0;
    let mut l_flags_14: i32 = 0;
    let mut l_k_15: i32 = 0;
    let mut l_n_16: i32 = 0;
    let mut l_x_17: i32 = 0;
    let mut l_y_18: i32 = 0;
    let mut l_fade_19: F32 = F32(0);
    let mut l_dist_20: F32 = F32(0);
    let mut l_scale_21: F32 = F32(0);
    let mut l_v_22: u32 = 0;
    let mut l_len_23: F32 = F32(0);
    let mut l_lit_24: F32 = F32(0);
    let r_ = 'f: {
        l_grp_3 = c.m.r32((p_model_1).wrapping_add(8));
        l_envTex_5 = ({ let a_ = (c.m.r32((sym::gEftBurstRes).wrapping_add(4))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(64));
        l_texs_4 = c.m.r32((p_model_1).wrapping_add(16));
        { let _ = F::memset(c, fp, 0i32, 16u32); };
        { let v_ = F32(0xc2480000); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        l_n_16 = (c.m.r32((p_model_1).wrapping_add(4)) as i32);
        l_fade_19 = rt::fmul(rt::i2f({ let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(4); (c.m.r32(a_) as i32) }), F32(0x3c888888));
        l_dist_20 = { let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(8); F32(c.m.r32(a_)) };
        if (((rt::fgt(l_fade_19, F32(0x3f800000)) as i32)) != 0) {
            l_fade_19 = F32(0x3f800000);
        }
        F::EftBurst_SetTexture(c, ((0i32) as u32), 0i32, 0i32);
        F::EftBurst_ClearAlphaPlane(c);
        { let a0_ = { let a0_ = F::Dma_BeginDirect(c); F::EftBurst_PutModelEnv(c, a0_) }; F::Dma_EndDirect(c, a0_) };
        {
            l_i_6 = 0i32;
            'l1: loop {
                if !(((((l_i_6) < (l_n_16)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_verts_7 = c.m.r32((l_grp_3).wrapping_add(8));
                    l_tri_8 = c.m.r32((l_grp_3).wrapping_add(12));
                    l_tex_9 = (l_texs_4).wrapping_add(((((c.m.r32((l_grp_3).wrapping_add(4)) as i32)) as u32)).wrapping_mul(64));
                    l_flags_14 = (c.m.r32(l_grp_3) as i32);
                    l_scale_21 = F32(c.m.r32((l_grp_3).wrapping_add(16)));
                    l_x_17 = 11520i32;
                    l_y_18 = 10752i32;
                    F::EftBurst_SetTexture(c, l_tex_9, 11520i32, 10752i32);
                    { let v_ = l_x_17; let a_ = (l_tex_9).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                    { let v_ = l_y_18; let a_ = (l_tex_9).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                    { let r_ = (c.m.r32((l_tex_9).wrapping_add(16)) as i32); let n_ = (l_x_17).wrapping_add(r_); l_x_17 = n_; };
                    { let r_ = (c.m.r32((l_tex_9).wrapping_add(20)) as i32); let n_ = (l_y_18).wrapping_add(r_); l_y_18 = n_; };
                    F::EftBurst_SetTexture(c, l_envTex_5, l_x_17, l_y_18);
                    { let v_ = l_x_17; let a_ = (l_envTex_5).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                    { let v_ = l_y_18; let a_ = (l_envTex_5).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                    l_pkt_13 = F::Dma_BeginDirect(c);
                    { let v_ = ((1152921504606879748i64) as u64); let a_ = (l_pkt_13).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = ((14i32) as u64); let a_ = (l_pkt_13).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = 2i32; let n_ = (l_pkt_13).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_13 = n_; };
                    { let v_ = ((0i32) as u64); let a_ = (l_pkt_13).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = ((8i32) as u64); let a_ = (l_pkt_13).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = 2i32; let n_ = (l_pkt_13).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_13 = n_; };
                    { let v_ = ((0i32) as u64); let a_ = (l_pkt_13).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = ((9i32) as u64); let a_ = (l_pkt_13).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = 2i32; let n_ = (l_pkt_13).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_13 = n_; };
                    { let v_ = ((68i32) as u64); let a_ = (l_pkt_13).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = ((66i32) as u64); let a_ = (l_pkt_13).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = 2i32; let n_ = (l_pkt_13).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_13 = n_; };
                    { let v_ = ((68i32) as u64); let a_ = (l_pkt_13).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = ((67i32) as u64); let a_ = (l_pkt_13).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = 2i32; let n_ = (l_pkt_13).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_13 = n_; };
                    {
                        l_t_10 = 0i32;
                        'l3: loop {
                            if !(((((l_t_10) < ((((c.m.r16((l_grp_3).wrapping_add(20)) as i16)) as i32))) as i32)) != 0) { break 'l3; }
                            'c4: {
                                l_vis_11 = 0i32;
                                l_env_12 = ((l_flags_14) & (2048i32));
                                {
                                    l_k_15 = 0i32;
                                    'l5: loop {
                                        if !(((((l_k_15) < (3i32)) as i32)) != 0) { break 'l5; }
                                        'c6: {
                                            l_v_22 = (l_verts_7).wrapping_add((((((((c.m.r16((l_tri_8).wrapping_add((((l_k_15) as u32)).wrapping_mul(2))) as i16)) as i32)).wrapping_mul(3i32)) as u32)).wrapping_mul(2));
                                            { let a1_ = (((c.m.r16((l_v_22).wrapping_add((((0i32) as u32)).wrapping_mul(2))) as i16)) as i32); let a2_ = (((c.m.r16((l_v_22).wrapping_add((((1i32) as u32)).wrapping_mul(2))) as i16)) as i32); let a3_ = (((c.m.r16((l_v_22).wrapping_add((((2i32) as u32)).wrapping_mul(2))) as i16)) as i32); F::IVec4_Set(c, fp.wrapping_add(32), a1_, a2_, a3_, 0i32) };
                                            F::IVec4_ToFloat12(c, fp.wrapping_add(48), fp.wrapping_add(32));
                                            { let a1_ = (((c.m.r16((((l_tri_8).wrapping_add(6)).wrapping_add((((l_k_15) as u32)).wrapping_mul(4))).wrapping_add((((0i32) as u32)).wrapping_mul(2))) as i16)) as i32); let a2_ = (((c.m.r16((((l_tri_8).wrapping_add(6)).wrapping_add((((l_k_15) as u32)).wrapping_mul(4))).wrapping_add((((1i32) as u32)).wrapping_mul(2))) as i16)) as i32); F::IVec4_Set(c, fp.wrapping_add(32), a1_, a2_, 0i32, 0i32) };
                                            F::IVec4_ToFloat12(c, (fp.wrapping_add(112)).wrapping_add((((l_k_15) as u32)).wrapping_mul(16)), fp.wrapping_add(32));
                                            F::Vec4_Scale(c, fp.wrapping_add(48), fp.wrapping_add(48), l_scale_21);
                                            F::Vec4_Sub(c, fp.wrapping_add(16), fp, fp.wrapping_add(48));
                                            { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                            { let a2_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); { let _ = F::Mtx_ProjectPointStq(c, (fp.wrapping_add(64)).wrapping_add((((l_k_15) as u32)).wrapping_mul(16)), (fp.wrapping_add(160)).wrapping_add((((l_k_15) as u32)).wrapping_mul(16)), a2_, fp.wrapping_add(48), (fp.wrapping_add(112)).wrapping_add((((l_k_15) as u32)).wrapping_mul(16))); } };
                                            { let v_ = ((((c.m.r16((((l_tri_8).wrapping_add(18)).wrapping_add((((l_k_15) as u32)).wrapping_mul(4))).wrapping_add((((0i32) as u32)).wrapping_mul(2)))) as i32)) & (255i32)); let a_ = (fp.wrapping_add(304)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = (((((c.m.r16((((l_tri_8).wrapping_add(18)).wrapping_add((((l_k_15) as u32)).wrapping_mul(4))).wrapping_add((((0i32) as u32)).wrapping_mul(2)))) as i32)).wrapping_shr((8i32) as u32)) & (255i32)); let a_ = (fp.wrapping_add(304)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = ((((c.m.r16((((l_tri_8).wrapping_add(18)).wrapping_add((((l_k_15) as u32)).wrapping_mul(4))).wrapping_add((((1i32) as u32)).wrapping_mul(2)))) as i32)) & (255i32)); let a_ = (fp.wrapping_add(304)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = (((((c.m.r16((((l_tri_8).wrapping_add(18)).wrapping_add((((l_k_15) as u32)).wrapping_mul(4))).wrapping_add((((1i32) as u32)).wrapping_mul(2)))) as i32)).wrapping_shr((8i32) as u32)) & (255i32)); let a_ = (fp.wrapping_add(304)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = (((((((((((c.m.r32((fp.wrapping_add(304)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u64)) | (((((c.m.r32((fp.wrapping_add(304)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u64)).wrapping_shl((8i32) as u32)))) | (((((c.m.r32((fp.wrapping_add(304)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u64)).wrapping_shl((16i32) as u32)))) | (((((c.m.r32((fp.wrapping_add(304)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) as u64)).wrapping_shl((24i32) as u32)))) | ((((c.m.r32(((fp.wrapping_add(160)).wrapping_add((((l_k_15) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))) as u64)).wrapping_shl((32i32) as u32))); let a_ = ((fp.wrapping_add(256)).wrapping_add((((l_k_15) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                            if ((l_env_12) != 0) {
                                                F::Vec4_Copy(c, (fp.wrapping_add(208)).wrapping_add((((l_k_15) as u32)).wrapping_mul(16)), (fp.wrapping_add(160)).wrapping_add((((l_k_15) as u32)).wrapping_mul(16)));
                                                { let r_ = F32(0x41000000); let a_ = ((fp.wrapping_add(208)).wrapping_add((((l_k_15) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                                { let r_ = F32(0x41000000); let a_ = ((fp.wrapping_add(208)).wrapping_add((((l_k_15) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                                if ((((({ let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(4); (c.m.r32(a_) as i32) }) >= (11i32)) as i32)) != 0) {
                                                    rt::memset(c, fp.wrapping_add(320), 0, 16);
                                                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(320).wrapping_add(0), (v_).0); }
                                                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(320).wrapping_add(4), (v_).0); }
                                                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(320).wrapping_add(8), (v_).0); }
                                                    { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(320).wrapping_add(12), (v_).0); }
                                                    rt::memset(c, fp.wrapping_add(336), 0, 16);
                                                    { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(336).wrapping_add(0), (v_).0); }
                                                    { let v_ = F32(0xbe4ccccc); c.m.w32(fp.wrapping_add(336).wrapping_add(4), (v_).0); }
                                                    { let v_ = F32(0xbf4ccccc); c.m.w32(fp.wrapping_add(336).wrapping_add(8), (v_).0); }
                                                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(336).wrapping_add(12), (v_).0); }
                                                    rt::memset(c, fp.wrapping_add(352), 0, 16);
                                                    { let v_ = F32(0x42fe0000); c.m.w32(fp.wrapping_add(352).wrapping_add(0), (v_).0); }
                                                    { let v_ = F32(0x42fe0000); c.m.w32(fp.wrapping_add(352).wrapping_add(4), (v_).0); }
                                                    { let v_ = F32(0x42fe0000); c.m.w32(fp.wrapping_add(352).wrapping_add(8), (v_).0); }
                                                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(352).wrapping_add(12), (v_).0); }
                                                    l_len_23 = { let a0_ = F::Vec3_Dot(c, fp.wrapping_add(16), fp.wrapping_add(16)); F::Port_sqrtf(c, a0_) };
                                                    l_lit_24 = rt::fsub(F32(0x437f0000), rt::fmul(rt::fdiv(l_len_23, l_dist_20), F32(0x437f0000)));
                                                    if (((rt::fgt(l_lit_24, F32(0x437f0000)) as i32)) != 0) {
                                                        l_lit_24 = F32(0x437f0000);
                                                    }
                                                    F::IVec4_ToFloat(c, fp.wrapping_add(368), fp.wrapping_add(304));
                                                    F::Vec4_Scale(c, fp.wrapping_add(320), fp.wrapping_add(320), rt::fmul(rt::fmul(l_fade_19, F32(0x3f199999)), l_lit_24));
                                                    F::Vec4_Add(c, fp.wrapping_add(352), fp.wrapping_add(320), fp.wrapping_add(352));
                                                    F::Vec4_Clamp(c, fp.wrapping_add(352), fp.wrapping_add(352), F32(0x00000000), F32(0x437f0000));
                                                    F::Vec4_ToInt(c, fp.wrapping_add(304), fp.wrapping_add(352));
                                                    { let v_ = (((((((((((c.m.r32((fp.wrapping_add(304)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u64)) | (((((c.m.r32((fp.wrapping_add(304)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u64)).wrapping_shl((8i32) as u32)))) | (((((c.m.r32((fp.wrapping_add(304)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u64)).wrapping_shl((16i32) as u32)))) | (((((c.m.r32((fp.wrapping_add(304)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) as u64)).wrapping_shl((24i32) as u32)))) | ((((c.m.r32(((fp.wrapping_add(160)).wrapping_add((((l_k_15) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))) as u64)).wrapping_shl((32i32) as u32))); let a_ = ((fp.wrapping_add(256)).wrapping_add((((l_k_15) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                                    if (((rt::flt(l_len_23, l_dist_20) as i32)) != 0) {
                                                        F::Vec4_Scale(c, fp.wrapping_add(336), fp.wrapping_add(336), l_lit_24);
                                                        F::Vec4_Add(c, fp.wrapping_add(352), fp.wrapping_add(336), fp.wrapping_add(368));
                                                        F::Vec4_Clamp(c, fp.wrapping_add(352), fp.wrapping_add(352), F32(0x00000000), F32(0x437f0000));
                                                        F::Vec4_ToInt(c, fp.wrapping_add(304), fp.wrapping_add(352));
                                                        { let v_ = (((((((((((c.m.r32((fp.wrapping_add(304)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u64)) | (((((c.m.r32((fp.wrapping_add(304)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u64)).wrapping_shl((8i32) as u32)))) | (((((c.m.r32((fp.wrapping_add(304)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u64)).wrapping_shl((16i32) as u32)))) | (((((c.m.r32((fp.wrapping_add(304)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) as u64)).wrapping_shl((24i32) as u32)))) | ((((c.m.r32(((fp.wrapping_add(160)).wrapping_add((((l_k_15) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))) as u64)).wrapping_shl((32i32) as u32))); let a_ = ((fp.wrapping_add(256)).wrapping_add((((l_k_15) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                                    }
                                                } else {
                                                    { let v_ = (((c.m.r32(((fp.wrapping_add(160)).wrapping_add((((l_k_15) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))) as u64)).wrapping_shl((32i32) as u32); let a_ = ((fp.wrapping_add(256)).wrapping_add((((l_k_15) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                                }
                                            }
                                            { let r_ = F::eft_d_b__EftBurst_IsVisible(c, (fp.wrapping_add(64)).wrapping_add((((l_k_15) as u32)).wrapping_mul(16))); let n_ = (l_vis_11).wrapping_add(r_); l_vis_11 = n_; };
                                        }
                                        { let o_ = l_k_15; let n_ = (o_).wrapping_add(1); l_k_15 = n_; };
                                    }
                                }
                                if (((((l_vis_11) != (0i32)) as i32)) != 0) {
                                    { let v_ = ((1152921504606879746i64) as u64); let a_ = (l_pkt_13).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                    { let v_ = ((14i32) as u64); let a_ = (l_pkt_13).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                    { let r_ = 2i32; let n_ = (l_pkt_13).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_13 = n_; };
                                    { let v_ = ((c.m.r64((l_tex_9).wrapping_add(48))) | (((1477760807611648i64) as u64))); let a_ = (l_pkt_13).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                    { let v_ = ((6i32) as u64); let a_ = (l_pkt_13).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                    { let r_ = 2i32; let n_ = (l_pkt_13).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_13 = n_; };
                                    { let v_ = ((91i32) as u64); let a_ = (l_pkt_13).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                    { let v_ = ((0i32) as u64); let a_ = (l_pkt_13).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                    { let r_ = 2i32; let n_ = (l_pkt_13).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_13 = n_; };
                                    { let v_ = ((3746994889972285443i64) as u64); let a_ = (l_pkt_13).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                    { let v_ = ((1313i32) as u64); let a_ = (l_pkt_13).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                    { let r_ = 2i32; let n_ = (l_pkt_13).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_13 = n_; };
                                    { let v_ = c.m.r64(((fp.wrapping_add(256)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(8))); let a_ = (l_pkt_13).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                    { let v_ = ((((c.m.r32(((fp.wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))) as u64)) | ((((c.m.r32(((fp.wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_pkt_13).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                    { let r_ = 2i32; let n_ = (l_pkt_13).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_13 = n_; };
                                    { let v_ = (((((((((c.m.r32(((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as i64)) | (((((c.m.r32(((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as i64)).wrapping_shl((16i32) as u32)))) | (((((c.m.r32(((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as i64)).wrapping_shl((32i32) as u32)))) as u64); let a_ = (l_pkt_13).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                    { let v_ = c.m.r64(((fp.wrapping_add(256)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(8))); let a_ = (l_pkt_13).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                    { let r_ = 2i32; let n_ = (l_pkt_13).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_13 = n_; };
                                    { let v_ = ((((c.m.r32(((fp.wrapping_add(160)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))) as u64)) | ((((c.m.r32(((fp.wrapping_add(160)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_pkt_13).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                    { let v_ = (((((((((c.m.r32(((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as i64)) | (((((c.m.r32(((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as i64)).wrapping_shl((16i32) as u32)))) | (((((c.m.r32(((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as i64)).wrapping_shl((32i32) as u32)))) as u64); let a_ = (l_pkt_13).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                    { let r_ = 2i32; let n_ = (l_pkt_13).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_13 = n_; };
                                    { let v_ = c.m.r64(((fp.wrapping_add(256)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(8))); let a_ = (l_pkt_13).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                    { let v_ = ((((c.m.r32(((fp.wrapping_add(160)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))) as u64)) | ((((c.m.r32(((fp.wrapping_add(160)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_pkt_13).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                    { let r_ = 2i32; let n_ = (l_pkt_13).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_13 = n_; };
                                    { let v_ = (((((((((c.m.r32(((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as i64)) | (((((c.m.r32(((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as i64)).wrapping_shl((16i32) as u32)))) | (((((c.m.r32(((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as i64)).wrapping_shl((32i32) as u32)))) as u64); let a_ = (l_pkt_13).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                    { let v_ = ((0i32) as u64); let a_ = (l_pkt_13).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                    { let r_ = 2i32; let n_ = (l_pkt_13).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_13 = n_; };
                                    if ((l_env_12) != 0) {
                                        { let v_ = ((1152921504606879746i64) as u64); let a_ = (l_pkt_13).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                        { let v_ = ((14i32) as u64); let a_ = (l_pkt_13).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                        { let r_ = 2i32; let n_ = (l_pkt_13).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_13 = n_; };
                                        { let v_ = ((c.m.r64((l_envTex_5).wrapping_add(48))) | (((((((((((c.m.r32((l_envTex_5).wrapping_add(36)) as i32)) as u32)) as u64)).wrapping_shl((37i32) as u32)) | ((((((c.m.r32((l_envTex_5).wrapping_add(32)) as i32)) as u32)) as u64)))) | (((17179869184i64) as u64))))); let a_ = (l_pkt_13).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                        { let v_ = ((7i32) as u64); let a_ = (l_pkt_13).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                        { let r_ = 2i32; let n_ = (l_pkt_13).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_13 = n_; };
                                        { let v_ = ((603i32) as u64); let a_ = (l_pkt_13).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                        { let v_ = ((0i32) as u64); let a_ = (l_pkt_13).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                        { let r_ = 2i32; let n_ = (l_pkt_13).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_13 = n_; };
                                        { let v_ = ((3746994889972285443i64) as u64); let a_ = (l_pkt_13).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                        { let v_ = ((1313i32) as u64); let a_ = (l_pkt_13).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                        { let r_ = 2i32; let n_ = (l_pkt_13).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_13 = n_; };
                                        { let v_ = c.m.r64(((fp.wrapping_add(256)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(8))); let a_ = (l_pkt_13).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                        { let v_ = ((((c.m.r32(((fp.wrapping_add(208)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))) as u64)) | ((((c.m.r32(((fp.wrapping_add(208)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_pkt_13).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                        { let r_ = 2i32; let n_ = (l_pkt_13).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_13 = n_; };
                                        { let v_ = (((((((((c.m.r32(((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as i64)) | (((((c.m.r32(((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as i64)).wrapping_shl((16i32) as u32)))) | (((((c.m.r32(((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as i64)).wrapping_shl((32i32) as u32)))) as u64); let a_ = (l_pkt_13).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                        { let v_ = c.m.r64(((fp.wrapping_add(256)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(8))); let a_ = (l_pkt_13).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                        { let r_ = 2i32; let n_ = (l_pkt_13).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_13 = n_; };
                                        { let v_ = ((((c.m.r32(((fp.wrapping_add(208)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))) as u64)) | ((((c.m.r32(((fp.wrapping_add(208)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_pkt_13).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                        { let v_ = (((((((((c.m.r32(((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as i64)) | (((((c.m.r32(((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as i64)).wrapping_shl((16i32) as u32)))) | (((((c.m.r32(((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as i64)).wrapping_shl((32i32) as u32)))) as u64); let a_ = (l_pkt_13).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                        { let r_ = 2i32; let n_ = (l_pkt_13).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_13 = n_; };
                                        { let v_ = c.m.r64(((fp.wrapping_add(256)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(8))); let a_ = (l_pkt_13).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                        { let v_ = ((((c.m.r32(((fp.wrapping_add(208)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))) as u64)) | ((((c.m.r32(((fp.wrapping_add(208)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_pkt_13).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                        { let r_ = 2i32; let n_ = (l_pkt_13).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_13 = n_; };
                                        { let v_ = (((((((((c.m.r32(((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as i64)) | (((((c.m.r32(((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as i64)).wrapping_shl((16i32) as u32)))) | (((((c.m.r32(((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as i64)).wrapping_shl((32i32) as u32)))) as u64); let a_ = (l_pkt_13).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                        { let v_ = ((0i32) as u64); let a_ = (l_pkt_13).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                        { let r_ = 2i32; let n_ = (l_pkt_13).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_pkt_13 = n_; };
                                    }
                                }
                            }
                            { { let o_ = l_t_10; let n_ = (o_).wrapping_add(1); l_t_10 = n_; }; { let o_ = l_tri_8; let n_ = (o_).wrapping_add(30); l_tri_8 = n_; }; };
                        }
                    }
                    F::Dma_EndDirect(c, l_pkt_13);
                }
                { { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; }; { let o_ = l_grp_3; let n_ = (o_).wrapping_add(24); l_grp_3 = n_; }; };
            }
        }
        { let a0_ = { let a0_ = F::Dma_BeginDirect(c); F::EftBurst_PutModelEnvEnd(c, a0_) }; F::Dma_EndDirect(c, a0_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_DrawModel", &PROBE_N, &[], true); }
    r_
}

pub fn EftBurst_SetTexture(c: &mut Ctx, mut p_tex_1: u32, mut p_x_2: i32, mut p_y_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_SetTexture", &PROBE_N, &[(p_tex_1) as u32, (p_x_2) as u32, (p_y_3) as u32], false); }
    let mut l_w_4: u32 = 0;
    let r_ = 'f: {
    l_w_4 = c.m.r32(sym::gEftBurst);
    if (((((c.m.r32((l_w_4).wrapping_add(560))) != (p_tex_1)) as i32)) != 0) {
        { let v_ = p_tex_1; let a_ = (l_w_4).wrapping_add(560); c.m.w32(a_, v_); };
        if (((((p_tex_1) != (((0i32) as u32))) as i32)) != 0) {
            F::Tex_Upload(c, p_tex_1, p_x_2, p_y_3);
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_SetTexture", &PROBE_N, &[], true); }
    r_
}

pub fn EftBurst_AllocPtcl(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_AllocPtcl", &PROBE_N, &[], false); }
    let mut l_p_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_p_1 = (c.m.r32(sym::gEftBurst)).wrapping_add(576);
    l_i_2 = 350i32;
    'l1: loop {
        'c2: {
            if ((((((c.m.r32((l_p_1).wrapping_add(64)) as i32)) == (0i32)) as i32)) != 0) {
                break 'f l_p_1;
            }
            { let o_ = l_p_1; let n_ = (o_).wrapping_add(112); l_p_1 = n_; };
        }
        if !((((({ let o_ = l_i_2; let n_ = (o_).wrapping_sub(1); l_i_2 = n_; n_ }) != (0i32)) as i32)) != 0) { break 'l1; }
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_AllocPtcl", &PROBE_N, &[], true); }
    r_
}

pub fn EftBurst_InitSpark(c: &mut Ctx, mut p_p_1: u32, mut p_idx_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_InitSpark", &PROBE_N, &[(p_p_1) as u32, (p_idx_2) as u32], false); }
    let mut l_fi_3: F32 = F32(0);
    let r_ = 'f: {
    l_fi_3 = rt::i2f(p_idx_2);
    F::Vec4_Set(c, p_p_1, F32(0x00000000), F32(0x00000000), rt::fsub(F32(0xc2200000), l_fi_3), F32(0x00000000));
    { let v_ = rt::fmul(rt::fmul(rt::i2f((((F::Port_Rand(c)) & (255i32))).wrapping_sub(127i32)), F32(0x3dcccccc)), F32(0x3c000000)); let a_ = (p_p_1).wrapping_add(16); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(rt::fmul(rt::i2f((((F::Port_Rand(c)) & (255i32))).wrapping_sub(127i32)), F32(0x3dcccccc)), F32(0x3c000000)); let a_ = ((p_p_1).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fsub(rt::fmul(rt::fneg(rt::i2f(((F::Port_Rand(c)) & (127i32)))), F32(0x3c000000)), F32(0x3f800000)); let a_ = ((p_p_1).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = ((p_p_1).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
    F::Vec3_Normalize(c, (p_p_1).wrapping_add(16), (p_p_1).wrapping_add(16));
    F::Vec4_Scale(c, (p_p_1).wrapping_add(16), (p_p_1).wrapping_add(16), F32(0x3f800000));
    { let r_ = F32(0x3f800000); let a_ = ((p_p_1).wrapping_add(16)).wrapping_add(8); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    { let v_ = ({ let a_ = (c.m.r32((sym::gEftBurstRes).wrapping_add(4))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(64)); let a_ = (p_p_1).wrapping_add(72); c.m.w32(a_, v_); };
    { let v_ = rt::fadd(rt::fsub(F32(0x41880000), rt::fmul(l_fi_3, F32(0x3ecccccc))), { let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(16); F32(c.m.r32(a_)) }); let a_ = (p_p_1).wrapping_add(76); c.m.w32(a_, (v_).0); };
    { let v_ = ((64i32) | (2i32)); let a_ = (p_p_1).wrapping_add(80); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let v_ = (25i32).wrapping_sub(p_idx_2); let a_ = (p_p_1).wrapping_add(68); c.m.w32(a_, (v_) as u32); v_ }; let a_ = (p_p_1).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((8421504i32) as u32); let a_ = (p_p_1).wrapping_add(92); c.m.w32(a_, v_); };
    { let v_ = 16i32; let a_ = (p_p_1).wrapping_add(96); c.m.w32(a_, (v_) as u32); };
    if ((((((c.m.r32((p_p_1).wrapping_add(64)) as i32)) <= (0i32)) as i32)) != 0) {
        { let v_ = 1i32; let a_ = (p_p_1).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
    }
    { let v_ = fid::EftBurst_UpdateSpark; let a_ = (p_p_1).wrapping_add(84); c.m.w32(a_, v_); };
    { let v_ = fid::EftBurst_DrawSprite; let a_ = (p_p_1).wrapping_add(88); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_InitSpark", &PROBE_N, &[], true); }
    r_
}

pub fn EftBurst_InitStreak(c: &mut Ctx, mut p_p_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_InitStreak", &PROBE_N, &[(p_p_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_t_2: F32 = F32(0);
    let mut l_ang_3: F32 = F32(0);
    let mut l_r_4: F32 = F32(0);
    let r_ = 'f: {
        F::EftBurst_InitDebris(c, p_p_1);
        { let v_ = 242i32; let a_ = (p_p_1).wrapping_add(80); c.m.w32(a_, (v_) as u32); };
        { let v_ = ({ let a_ = (c.m.r32((sym::gEftBurstRes).wrapping_add(4))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(64)); let a_ = (p_p_1).wrapping_add(72); c.m.w32(a_, v_); };
        { let v_ = { let v_ = (((F::Port_Rand(c)) & (15i32))).wrapping_add(7i32); let a_ = (p_p_1).wrapping_add(64); c.m.w32(a_, (v_) as u32); v_ }; let a_ = (p_p_1).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
        { let v_ = rt::fadd(rt::fmul(rt::fmul(rt::i2f(((F::Port_Rand(c)) & (31i32))), F32(0x3d000000)), F32(0x3fc00000)), F32(0x40200000)); let a_ = (p_p_1).wrapping_add(76); c.m.w32(a_, (v_).0); };
        F::Vec4_Set(c, (p_p_1).wrapping_add(48), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x00000000));
        F::Vec4_Set(c, (p_p_1).wrapping_add(32), F32(0x00000000), F32(0x3fc8f5c2), F32(0x00000000), F32(0x00000000));
        l_t_2 = rt::fmul(rt::i2f({ let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(4); (c.m.r32(a_) as i32) }), F32(0x3ada740d));
        l_ang_3 = rt::fdiv(rt::fmul(F32(c.m.r32(sym::gEftBurstStreakAngle)), F32(0x4048f5c2)), F32(0x43340000));
        if (((rt::fgt(l_t_2, F32(0x3f800000)) as i32)) != 0) {
            l_t_2 = F32(0x3f800000);
        }
        l_r_4 = rt::fadd(rt::fmul(rt::fmul(rt::i2f(((F::Port_Rand(c)) & (31i32))), F32(0x3d000000)), rt::fadd(l_t_2, F32(0x40000000))), F32(0x41200000));
        {
            rt::memset(c, fp, 0, 16);
            rt::memset(c, fp.wrapping_add(0), 0, 16);
            { let v_ = rt::fmul(F::Port_cosf(c, l_ang_3), l_r_4); c.m.w32(fp.wrapping_add(0).wrapping_add(0), (v_).0); }
            { let v_ = rt::fmul(F::Port_sinf(c, l_ang_3), l_r_4); c.m.w32(fp.wrapping_add(0).wrapping_add(4), (v_).0); }
            { let v_ = F32(0xc2480000); c.m.w32(fp.wrapping_add(0).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(12), (v_).0); }
            F::Vec4_Copy(c, p_p_1, fp);
            F::Vec4_Scale(c, fp, fp, F32(0x3d23d70a));
            { let v_ = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(((F::Port_Rand(c)) & (31i32))), F32(0x41700000)), F32(0xbf199999)), rt::fmul(l_t_2, F32(0x3e4ccccc))); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            F::Vec4_Copy(c, (p_p_1).wrapping_add(16), fp);
        }
        { let a3_ = rt::fdiv(rt::fmul(F32(c.m.r32(sym::gEftBurstStreakAngle)), F32(0x4048f5c2)), F32(0x43340000)); F::Vec4_Set(c, (p_p_1).wrapping_add(32), F32(0xbdb2a18f), F32(0x3fc8f5c2), a3_, F32(0x00000000)) };
        if ((((F::Port_Rand(c)) & (1i32))) != 0) {
            { let v_ = ((48i32) as u32); let a_ = (p_p_1).wrapping_add(92); c.m.w32(a_, v_); };
            { let v_ = 80i32; let a_ = (p_p_1).wrapping_add(96); c.m.w32(a_, (v_) as u32); };
            { let r_ = 8i32; let a_ = (p_p_1).wrapping_add(80); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        } else {
            { let v_ = ((64i32) as u32); let a_ = (p_p_1).wrapping_add(92); c.m.w32(a_, v_); };
            { let v_ = 80i32; let a_ = (p_p_1).wrapping_add(96); c.m.w32(a_, (v_) as u32); };
        }
        { let r_ = rt::i2f(((F::Port_Rand(c)) & (63i32))); let a_ = sym::gEftBurstStreakAngle; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if (((rt::fgt(F32(c.m.r32(sym::gEftBurstStreakAngle)), F32(0x43340000)) as i32)) != 0) {
            { let r_ = F32(0x43b40000); let a_ = sym::gEftBurstStreakAngle; let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
        { let v_ = fid::EftBurst_UpdateStreak; let a_ = (p_p_1).wrapping_add(84); c.m.w32(a_, v_); };
        { let v_ = fid::EftBurst_DrawQuadRot; let a_ = (p_p_1).wrapping_add(88); c.m.w32(a_, v_); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_InitStreak", &PROBE_N, &[], true); }
    r_
}

pub fn EftBurst_InitFlash(c: &mut Ctx, mut p_p_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_InitFlash", &PROBE_N, &[(p_p_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = 122i32; let a_ = (p_p_1).wrapping_add(80); c.m.w32(a_, (v_) as u32); };
    { let v_ = ({ let a_ = (c.m.r32((sym::gEftBurstRes).wrapping_add(4))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(64)); let a_ = (p_p_1).wrapping_add(72); c.m.w32(a_, v_); };
    { let v_ = 10i32; let a_ = (p_p_1).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
    { let v_ = 10i32; let a_ = (p_p_1).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
    { let v_ = F32(0x41200000); let a_ = (p_p_1).wrapping_add(76); c.m.w32(a_, (v_).0); };
    F::Vec4_Set(c, p_p_1, F32(0x00000000), F32(0x00000000), F32(0xc25c0000), F32(0x00000000));
    F::Vec4_Set(c, (p_p_1).wrapping_add(16), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x00000000));
    { let v_ = 128i32; let a_ = (p_p_1).wrapping_add(96); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((8421504i32) as u32); let a_ = (p_p_1).wrapping_add(92); c.m.w32(a_, v_); };
    { let v_ = fid::EftBurst_UpdateFlash; let a_ = (p_p_1).wrapping_add(84); c.m.w32(a_, v_); };
    { let v_ = fid::EftBurst_DrawQuadRot; let a_ = (p_p_1).wrapping_add(88); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_InitFlash", &PROBE_N, &[], true); }
    r_
}

pub fn EftBurst_InitRing(c: &mut Ctx, mut p_p_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_InitRing", &PROBE_N, &[(p_p_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = 74i32; let a_ = (p_p_1).wrapping_add(80); c.m.w32(a_, (v_) as u32); };
    { let v_ = ({ let a_ = (c.m.r32((sym::gEftBurstRes).wrapping_add(4))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(64)); let a_ = (p_p_1).wrapping_add(72); c.m.w32(a_, v_); };
    { let v_ = 8i32; let a_ = (p_p_1).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
    { let v_ = 8i32; let a_ = (p_p_1).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
    { let v_ = F32(0x3dcccccc); let a_ = (p_p_1).wrapping_add(76); c.m.w32(a_, (v_).0); };
    { let v_ = ((7372928i32) as u32); let a_ = (p_p_1).wrapping_add(92); c.m.w32(a_, v_); };
    { let v_ = 64i32; let a_ = (p_p_1).wrapping_add(96); c.m.w32(a_, (v_) as u32); };
    { let v_ = F32(0x428c0000); let a_ = (p_p_1).wrapping_add(104); c.m.w32(a_, (v_).0); };
    F::Vec4_Set(c, p_p_1, F32(0xc0a00000), F32(0x00000000), F32(0xc24c0000), F32(0x00000000));
    F::Vec4_Set(c, (p_p_1).wrapping_add(16), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x00000000));
    { let v_ = fid::EftBurst_UpdateRing; let a_ = (p_p_1).wrapping_add(84); c.m.w32(a_, v_); };
    { let v_ = fid::EftBurst_DrawQuadRot; let a_ = (p_p_1).wrapping_add(88); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_InitRing", &PROBE_N, &[], true); }
    r_
}

pub fn EftBurst_InitGlow(c: &mut Ctx, mut p_p_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_InitGlow", &PROBE_N, &[(p_p_1) as u32], false); }
    let r_ = 'f: {
    F::Vec4_Set(c, p_p_1, F32(0x00000000), F32(0x00000000), F32(0xc24c0000), F32(0x00000000));
    F::Vec4_Set(c, (p_p_1).wrapping_add(16), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x00000000));
    { let v_ = ({ let a_ = (c.m.r32((sym::gEftBurstRes).wrapping_add(4))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(64)); let a_ = (p_p_1).wrapping_add(72); c.m.w32(a_, v_); };
    { let v_ = F32(0x42200000); let a_ = (p_p_1).wrapping_add(76); c.m.w32(a_, (v_).0); };
    { let v_ = ((1i32) | (2i32)); let a_ = (p_p_1).wrapping_add(80); c.m.w32(a_, (v_) as u32); };
    { let v_ = (((F::Port_Rand(c)) & (15i32))).wrapping_add(47i32); let a_ = (p_p_1).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32((p_p_1).wrapping_add(64)) as i32); let a_ = (p_p_1).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
    { let v_ = F32(0x41200000); let a_ = (p_p_1).wrapping_add(76); c.m.w32(a_, (v_).0); };
    { let v_ = fid::EftBurst_DrawQuad; let a_ = (p_p_1).wrapping_add(88); c.m.w32(a_, v_); };
    { let v_ = fid::EftBurst_UpdateGlow; let a_ = (p_p_1).wrapping_add(84); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_InitGlow", &PROBE_N, &[], true); }
    r_
}

pub fn EftBurst_InitDebris(c: &mut Ctx, mut p_p_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_InitDebris", &PROBE_N, &[(p_p_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = 75i32; let a_ = (p_p_1).wrapping_add(80); c.m.w32(a_, (v_) as u32); };
        { let v_ = rt::fadd(rt::fmul(rt::fmul(rt::i2f(((F::Port_Rand(c)) & (127i32))), F32(0x40000000)), F32(0x3c000000)), F32(0x40800000)); let a_ = (p_p_1).wrapping_add(76); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fmul(rt::i2f((((F::Port_Rand(c)) & (255i32))).wrapping_sub(127i32)), F32(0x41000000)), F32(0x3c000000)); let a_ = p_p_1; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fmul(rt::i2f((((F::Port_Rand(c)) & (255i32))).wrapping_sub(127i32)), F32(0x41000000)), F32(0x3c000000)); let a_ = (p_p_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(rt::fmul(rt::fmul(rt::fneg(rt::i2f(((F::Port_Rand(c)) & (127i32)))), F32(0x40000000)), F32(0x3c000000)), F32(0x424c0000)); let a_ = (p_p_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_p_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fmul(rt::i2f((((F::Port_Rand(c)) & (255i32))).wrapping_sub(127i32)), F32(0x3dcccccc)), F32(0x3c000000)); let a_ = (p_p_1).wrapping_add(16); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fmul(rt::i2f((((F::Port_Rand(c)) & (255i32))).wrapping_sub(127i32)), F32(0x3dcccccc)), F32(0x3c000000)); let a_ = ((p_p_1).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fmul(rt::fneg(rt::i2f(((F::Port_Rand(c)) & (127i32)))), F32(0x3e4ccccc)), F32(0x3c000000)); let a_ = ((p_p_1).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((p_p_1).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let _ = F::memset(c, fp, 0i32, 16u32); };
        { let v_ = F32(0x42340000); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        F::Vec4_Sub(c, fp, fp, p_p_1);
        { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        F::Vec3_Normalize(c, fp, fp);
        F::Vec4_Scale(c, fp, fp, F32(0xbdcccccc));
        F::Vec4_Add(c, (p_p_1).wrapping_add(16), (p_p_1).wrapping_add(16), fp);
        { let v_ = { let v_ = (((F::Port_Rand(c)) & (3i32))).wrapping_add(31i32); let a_ = (p_p_1).wrapping_add(68); c.m.w32(a_, (v_) as u32); v_ }; let a_ = (p_p_1).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
        { let v_ = ({ let a_ = (c.m.r32((sym::gEftBurstRes).wrapping_add(4))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(64)); let a_ = (p_p_1).wrapping_add(72); c.m.w32(a_, v_); };
        { let v_ = fid::EftBurst_UpdateDebris; let a_ = (p_p_1).wrapping_add(84); c.m.w32(a_, v_); };
        { let v_ = fid::EftBurst_DrawSprite; let a_ = (p_p_1).wrapping_add(88); c.m.w32(a_, v_); };
        { let v_ = ((6316128i32) as u32); let a_ = (p_p_1).wrapping_add(92); c.m.w32(a_, v_); };
        { let v_ = 0i32; let a_ = (p_p_1).wrapping_add(96); c.m.w32(a_, (v_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_InitDebris", &PROBE_N, &[], true); }
    r_
}

pub fn EftBurst_UpdateFlash(c: &mut Ctx, mut p_p_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_UpdateFlash", &PROBE_N, &[(p_p_1) as u32], false); }
    let r_ = 'f: {
    F::Vec4_Add(c, p_p_1, p_p_1, (p_p_1).wrapping_add(16));
    F::Vec4_Scale(c, (p_p_1).wrapping_add(16), (p_p_1).wrapping_add(16), F32(0x3f77bdef));
    { let v_ = rt::fmul(rt::fadd(F32(c.m.r32((p_p_1).wrapping_add(76))), F32(0x3dcccccc)), F32(0x3f999999)); let a_ = (p_p_1).wrapping_add(76); c.m.w32(a_, (v_).0); };
    if (((((c.m.r32((p_p_1).wrapping_add(80)) as i32)) & (2i32))) != 0) {
        if ((((((c.m.r32((p_p_1).wrapping_add(64)) as i32)) != (0i32)) as i32)) != 0) {
            { let a_ = (p_p_1).wrapping_add(64); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
        } else {
            break 'f 0i32;
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_UpdateFlash", &PROBE_N, &[], true); }
    r_
}

pub fn EftBurst_UpdateStreak(c: &mut Ctx, mut p_p_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_UpdateStreak", &PROBE_N, &[(p_p_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_i_2: i32 = 0;
    let mut l_pos_3: u32 = 0;
    let mut l_vel_4: u32 = 0;
    let mut l_rot_5: u32 = 0;
    let mut l_rotVel_6: u32 = 0;
    let r_ = 'f: {
        l_pos_3 = p_p_1;
        l_vel_4 = (p_p_1).wrapping_add(16);
        l_rot_5 = (p_p_1).wrapping_add(32);
        l_rotVel_6 = (p_p_1).wrapping_add(48);
        { let _ = F::memset(c, fp, 0i32, 16u32); };
        F::Vec4_Add(c, l_pos_3, l_pos_3, l_vel_4);
        F::Vec4_Scale(c, l_vel_4, l_vel_4, F32(0x3f77bdef));
        F::Vec4_Sub(c, fp.wrapping_add(16), fp, l_pos_3);
        F::Vec3_Normalize(c, fp.wrapping_add(16), fp.wrapping_add(16));
        F::Vec4_Scale(c, fp.wrapping_add(16), fp.wrapping_add(16), F32(0x3d85cd7b));
        F::Vec4_Add(c, (p_p_1).wrapping_add(16), (p_p_1).wrapping_add(16), fp.wrapping_add(16));
        { let v_ = F32(0x00000000); let a_ = ((p_p_1).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec4_Add(c, l_rot_5, l_rot_5, l_rotVel_6);
        { let r_ = F32(0xbc0ee7a6); let a_ = l_rotVel_6; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (3i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((rt::fgt(F32(c.m.r32((l_rot_5).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)))), F32(0x4048f5c2)) as i32)) != 0) {
                        { let v_ = F32(0xc048f5c2); let a_ = (l_rot_5).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    }
                    if (((rt::flt(F32(c.m.r32((l_rot_5).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)))), F32(0xc048f5c2)) as i32)) != 0) {
                        { let v_ = F32(0x4048f5c2); let a_ = (l_rot_5).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        if (((((c.m.r32((p_p_1).wrapping_add(80)) as i32)) & (2i32))) != 0) {
            if ((((((c.m.r32((p_p_1).wrapping_add(64)) as i32)) != (0i32)) as i32)) != 0) {
                if ((((({ let a_ = (p_p_1).wrapping_add(64); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) == (0i32)) as i32)) != 0) {
                    F::EftBurst_InitStreak(c, p_p_1);
                }
            } else {
                break 'f 0i32;
            }
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_UpdateStreak", &PROBE_N, &[], true); }
    r_
}

pub fn EftBurst_UpdateSpark(c: &mut Ctx, mut p_p_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_UpdateSpark", &PROBE_N, &[(p_p_1) as u32], false); }
    let r_ = 'f: {
    F::Vec4_Add(c, p_p_1, p_p_1, (p_p_1).wrapping_add(16));
    F::Vec4_Scale(c, (p_p_1).wrapping_add(16), (p_p_1).wrapping_add(16), F32(0x3f7ae147));
    { let r_ = F32(0x3e999999); let a_ = (p_p_1).wrapping_add(76); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    if ((((((c.m.r32((p_p_1).wrapping_add(64)) as i32)) < (10i32)) as i32)) != 0) {
        { let v_ = (c.m.r32((p_p_1).wrapping_add(64)) as i32); let a_ = (p_p_1).wrapping_add(96); c.m.w32(a_, (v_) as u32); };
    }
    if (((((c.m.r32((p_p_1).wrapping_add(80)) as i32)) & (2i32))) != 0) {
        if ((((((c.m.r32((p_p_1).wrapping_add(64)) as i32)) != (0i32)) as i32)) != 0) {
            { let a_ = (p_p_1).wrapping_add(64); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
        } else {
            break 'f 0i32;
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_UpdateSpark", &PROBE_N, &[], true); }
    r_
}

pub fn EftBurst_UpdateMove(c: &mut Ctx, mut p_p_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_UpdateMove", &PROBE_N, &[(p_p_1) as u32], false); }
    let r_ = 'f: {
    F::Vec4_Add(c, p_p_1, p_p_1, (p_p_1).wrapping_add(16));
    F::Vec4_Scale(c, (p_p_1).wrapping_add(16), (p_p_1).wrapping_add(16), F32(0x3f77bdef));
    if (((((c.m.r32((p_p_1).wrapping_add(80)) as i32)) & (2i32))) != 0) {
        if ((((((c.m.r32((p_p_1).wrapping_add(64)) as i32)) != (0i32)) as i32)) != 0) {
            { let a_ = (p_p_1).wrapping_add(64); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
        } else {
            break 'f 0i32;
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_UpdateMove", &PROBE_N, &[], true); }
    r_
}

pub fn EftBurst_UpdateRing(c: &mut Ctx, mut p_p_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_UpdateRing", &PROBE_N, &[(p_p_1) as u32], false); }
    let r_ = 'f: {
    F::Vec4_Add(c, p_p_1, p_p_1, (p_p_1).wrapping_add(16));
    F::Vec4_Scale(c, (p_p_1).wrapping_add(16), (p_p_1).wrapping_add(16), F32(0x3f77bdef));
    { let v_ = rt::fdiv(rt::i2f(((c.m.r32((p_p_1).wrapping_add(68)) as i32)).wrapping_sub((c.m.r32((p_p_1).wrapping_add(64)) as i32))), rt::i2f((c.m.r32((p_p_1).wrapping_add(68)) as i32))); let a_ = (p_p_1).wrapping_add(76); c.m.w32(a_, (v_).0); };
    if (((rt::fgt(F32(c.m.r32((p_p_1).wrapping_add(76))), F32(0x3f000000)) as i32)) != 0) {
        { let v_ = rt::fsub(F32(0x3f800000), F32(c.m.r32((p_p_1).wrapping_add(76)))); let a_ = (p_p_1).wrapping_add(76); c.m.w32(a_, (v_).0); };
    }
    { let v_ = rt::fadd(rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(76))), F32(c.m.r32((p_p_1).wrapping_add(104)))), F32(0x41200000)); let a_ = (p_p_1).wrapping_add(76); c.m.w32(a_, (v_).0); };
    if (((((c.m.r32((p_p_1).wrapping_add(80)) as i32)) & (2i32))) != 0) {
        if ((((((c.m.r32((p_p_1).wrapping_add(64)) as i32)) != (0i32)) as i32)) != 0) {
            { let a_ = (p_p_1).wrapping_add(64); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
        } else {
            break 'f 0i32;
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_UpdateRing", &PROBE_N, &[], true); }
    r_
}

pub fn EftBurst_UpdateGlow(c: &mut Ctx, mut p_p_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_UpdateGlow", &PROBE_N, &[(p_p_1) as u32], false); }
    let r_ = 'f: {
    F::Vec4_Add(c, p_p_1, p_p_1, (p_p_1).wrapping_add(16));
    F::Vec4_Scale(c, (p_p_1).wrapping_add(16), (p_p_1).wrapping_add(16), F32(0x3f77bdef));
    { let r_ = F32(0x3eb33333); let a_ = (p_p_1).wrapping_add(76); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    if (((((c.m.r32((p_p_1).wrapping_add(80)) as i32)) & (2i32))) != 0) {
        if ((((((c.m.r32((p_p_1).wrapping_add(64)) as i32)) != (0i32)) as i32)) != 0) {
            { let a_ = (p_p_1).wrapping_add(64); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
        } else {
            break 'f 0i32;
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_UpdateGlow", &PROBE_N, &[], true); }
    r_
}

pub fn EftBurst_UpdateDebris(c: &mut Ctx, mut p_p_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_UpdateDebris", &PROBE_N, &[(p_p_1) as u32], false); }
    let r_ = 'f: {
    F::Vec4_Add(c, p_p_1, p_p_1, (p_p_1).wrapping_add(16));
    F::Vec4_Scale(c, (p_p_1).wrapping_add(16), (p_p_1).wrapping_add(16), F32(0x3f7ae147));
    { let v_ = rt::div_i32(((c.m.r32((p_p_1).wrapping_add(64)) as i32)).wrapping_mul(48i32), (c.m.r32((p_p_1).wrapping_add(68)) as i32)); let a_ = (p_p_1).wrapping_add(96); c.m.w32(a_, (v_) as u32); };
    if ((((((c.m.r32((p_p_1).wrapping_add(96)) as i32)) >= (25i32)) as i32)) != 0) {
        { let v_ = (48i32).wrapping_sub((c.m.r32((p_p_1).wrapping_add(96)) as i32)); let a_ = (p_p_1).wrapping_add(96); c.m.w32(a_, (v_) as u32); };
    }
    if (((((c.m.r32((p_p_1).wrapping_add(80)) as i32)) & (2i32))) != 0) {
        if ((((((c.m.r32((p_p_1).wrapping_add(64)) as i32)) != (0i32)) as i32)) != 0) {
            { let a_ = (p_p_1).wrapping_add(64); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
        } else {
            break 'f 0i32;
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_UpdateDebris", &PROBE_N, &[], true); }
    r_
}

pub fn EftBurstLayer_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBurstLayer_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = F::BtlTask_CreateChildList(c, p_task_1, ((((1i32) as i64 as u64)) as u32), ((((4i32) as i64 as u64)) as u32)); let a_ = sym::gEftBurstRes; c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = (sym::gEftBurstRes).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (sym::gEftBurstRes).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBurstLayer_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftBurstLayer_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBurstLayer_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBurstLayer_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftBurstLayer_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBurstLayer_Update", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        if ((((((c.m.r32((sym::gEftBurstRes).wrapping_add(16)) as i32)) != (0i32)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = fp; c.m.w32(a_, (v_) as u32); };
            { let _ = { let a0_ = c.m.r32(sym::gEftBurstRes); F::BtlTaskList_AddTail(c, a0_, sym::gEftBurstClass, fp) }; };
            { let v_ = 0i32; let a_ = (sym::gEftBurstRes).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBurstLayer_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftBurst_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_Init", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_off_2: i32 = 0;
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, ((39792u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gEftBurst; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gEftBurst); F::memset(c, a0_, 0i32, 39792u32) }; };
    { let v_ = 0i32; let a_ = (sym::gEftBurstRes).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    {
        { l_i_1 = 0i32; l_off_2 = 0i32; };
        'l1: loop {
            if !(((((l_i_1) < (350i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let _ = { let a0_ = ((c.m.r32(sym::gEftBurst)).wrapping_add(((l_off_2) as u32))).wrapping_add(((576i32) as u32)); F::memset(c, a0_, 0i32, 112u32) }; };
            }
            { { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; }; { let r_ = 112u32; let n_ = (((((l_off_2) as u32)).wrapping_add(r_)) as i32); l_off_2 = n_; }; };
        }
    }
    { let v_ = F32(0x41000000); let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x41a00000); let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = 60i32; let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(39776); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((c.m.r32((sym::gEftBurstRes).wrapping_add(8))) as i32); let a_ = ((c.m.r32(sym::gEftBurst)).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = (1i32).wrapping_neg(); let a_ = ((c.m.r32(sym::gEftBurst)).wrapping_add(24)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = (1i32).wrapping_neg(); let a_ = ((c.m.r32(sym::gEftBurst)).wrapping_add(24)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let a0_ = (c.m.r32(sym::gEftBurst)).wrapping_add(24); F::EftCam_Start(c, a0_) };
    F::EftCam_SetHold(c, 1i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftBurst_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftBurst)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gEftBurst); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gEftBurst; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftBurst_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let mut l_j_3: i32 = 0;
    let mut l_part_4: u32 = 0;
    let mut l_part_5: u32 = 0;
    let mut l_part_6: u32 = 0;
    let mut l_part_7: u32 = 0;
    let mut l_part_8: u32 = 0;
    let mut l_part_9: u32 = 0;
    let mut l_part_10: u32 = 0;
    let mut l_part_11: u32 = 0;
    let mut l_part_12: u32 = 0;
    let mut l_part_13: u32 = 0;
    let mut l_part_14: u32 = 0;
    let r_ = 'f: {
    if ((((((c.m.r32((sym::gEftBurstRes).wrapping_add(20)) as i32)) != (0i32)) as i32)) != 0) {
        F::BtlTask_SetDead(c, p_task_1);
        F::EftCam_Stop(c);
        break 'f;
    }
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(4); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        {
            l_part_4 = ((D::call(c, fid::EftBurst_AllocPtcl, &[])) as u32);
            if (((((l_part_4) != (((0i32) as u32))) as i32)) != 0) {
                { let _ = D::call(c, fid::EftBurst_InitRing, &[((l_part_4) as u64)]); };
                { let r_ = 16i32; let a_ = (l_part_4).wrapping_add(80); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = F32(0x428c0000); let a_ = (l_part_4).wrapping_add(104); c.m.w32(a_, (v_).0); };
                { let v_ = 32i32; let a_ = (l_part_4).wrapping_add(96); c.m.w32(a_, (v_) as u32); };
            }
        }
        {
            l_part_5 = ((D::call(c, fid::EftBurst_AllocPtcl, &[])) as u32);
            if (((((l_part_5) != (((0i32) as u32))) as i32)) != 0) {
                { let _ = D::call(c, fid::EftBurst_InitRing, &[((l_part_5) as u64)]); };
                { let r_ = 16i32; let a_ = (l_part_5).wrapping_add(80); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = F32(0x42480000); let a_ = (l_part_5).wrapping_add(104); c.m.w32(a_, (v_).0); };
                { let v_ = 64i32; let a_ = (l_part_5).wrapping_add(96); c.m.w32(a_, (v_) as u32); };
            }
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(4); (c.m.r32(a_) as i32) }) == (7i32)) as i32)) != 0) {
        {
            l_part_6 = ((D::call(c, fid::EftBurst_AllocPtcl, &[])) as u32);
            if (((((l_part_6) != (((0i32) as u32))) as i32)) != 0) {
                { let _ = D::call(c, fid::EftBurst_InitFlash, &[((l_part_6) as u64)]); };
                { let v_ = 69i32; let a_ = (l_part_6).wrapping_add(96); c.m.w32(a_, (v_) as u32); };
            }
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(4); (c.m.r32(a_) as i32) }) == (10i32)) as i32)) != 0) {
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (240i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    {
                        l_part_7 = ((D::call(c, fid::EftBurst_AllocPtcl, &[])) as u32);
                        if (((((l_part_7) != (((0i32) as u32))) as i32)) != 0) {
                            { let _ = D::call(c, fid::EftBurst_InitStreak, &[((l_part_7) as u64)]); };
                        }
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(4); (c.m.r32(a_) as i32) }) == (30i32)) as i32)) != 0) {
        {
            l_part_8 = ((D::call(c, fid::EftBurst_AllocPtcl, &[])) as u32);
            if (((((l_part_8) != (((0i32) as u32))) as i32)) != 0) {
                { let _ = D::call(c, fid::EftBurst_InitFlash, &[((l_part_8) as u64)]); };
                { let v_ = 32i32; let a_ = (l_part_8).wrapping_add(96); c.m.w32(a_, (v_) as u32); };
            }
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(4); (c.m.r32(a_) as i32) }) >= (9i32)) as i32)) != 0) {
        if ((((({ let a_ = c.m.r32(sym::gEftBurst); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
            { let a_ = c.m.r32(sym::gEftBurst); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
        } else {
            {
                l_part_9 = ((D::call(c, fid::EftBurst_AllocPtcl, &[])) as u32);
                if (((((l_part_9) != (((0i32) as u32))) as i32)) != 0) {
                    { let _ = D::call(c, fid::EftBurst_InitGlow, &[((l_part_9) as u64)]); };
                }
            }
            { let v_ = (((F::Port_Rand(c)) & (3i32))).wrapping_add(5i32); let a_ = c.m.r32(sym::gEftBurst); c.m.w32(a_, (v_) as u32); };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(4); (c.m.r32(a_) as i32) }) == (10i32)) as i32)) != 0) {
        {
            l_i_2 = 0i32;
            'l3: loop {
                if !(((((l_i_2) < (32i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    {
                        l_part_10 = ((D::call(c, fid::EftBurst_AllocPtcl, &[])) as u32);
                        if (((((l_part_10) != (((0i32) as u32))) as i32)) != 0) {
                            { let _ = D::call(c, fid::EftBurst_InitSpark, &[((l_part_10) as u64), ((l_i_2) as i64 as u64)]); };
                        }
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        {
            l_i_2 = 0i32;
            'l5: loop {
                if !(((((l_i_2) < (16i32)) as i32)) != 0) { break 'l5; }
                'c6: {
                    {
                        l_part_11 = ((D::call(c, fid::EftBurst_AllocPtcl, &[])) as u32);
                        if (((((l_part_11) != (((0i32) as u32))) as i32)) != 0) {
                            { let _ = D::call(c, fid::EftBurst_InitDebris, &[((l_part_11) as u64)]); };
                        }
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(4); (c.m.r32(a_) as i32) }) >= (11i32)) as i32)) != 0) {
        {
            l_part_12 = ((D::call(c, fid::EftBurst_AllocPtcl, &[])) as u32);
            if (((((l_part_12) != (((0i32) as u32))) as i32)) != 0) {
                { let _ = D::call(c, fid::EftBurst_InitSpark, &[((l_part_12) as u64), ((0i32) as i64 as u64)]); };
            }
        }
        {
            l_part_13 = ((D::call(c, fid::EftBurst_AllocPtcl, &[])) as u32);
            if (((((l_part_13) != (((0i32) as u32))) as i32)) != 0) {
                { let _ = D::call(c, fid::EftBurst_InitDebris, &[((l_part_13) as u64)]); };
            }
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(4); (c.m.r32(a_) as i32) }) < (151i32)) as i32)) != 0) {
        { let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    }
    if (((rt::flt({ let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(8); F32(c.m.r32(a_)) }, F32(0x42480000)) as i32)) != 0) {
        { let r_ = F32(0x3ecccccc); let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
    { let r_ = F32(0x3f7eb851); let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(12); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    { let r_ = (((F::Port_Rand(c)) & (7i32))).wrapping_add(3i32); let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(20); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
    if ((((({ let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(20); (c.m.r32(a_) as i32) }) > (360i32)) as i32)) != 0) {
        { let r_ = 360i32; let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(20); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
    }
    { let v_ = rt::fdiv(rt::fadd(rt::fadd(rt::fadd(rt::fadd(rt::fadd({ let a0_ = rt::fdiv(rt::fmul(rt::i2f({ let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(20); (c.m.r32(a_) as i32) }), F32(0x40490fda)), F32(0x43340000)); F::Port_sinf(c, a0_) }, { let a0_ = rt::fdiv(rt::fmul(rt::i2f(rt::rem_i32(({ let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(20); (c.m.r32(a_) as i32) }).wrapping_mul(2i32), 360i32)), F32(0x40490fda)), F32(0x43340000)); F::Port_sinf(c, a0_) }), { let a0_ = rt::fdiv(rt::fmul(rt::i2f(rt::rem_i32(({ let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(20); (c.m.r32(a_) as i32) }).wrapping_mul(3i32), 360i32)), F32(0x40490fda)), F32(0x43340000)); F::Port_sinf(c, a0_) }), { let a0_ = rt::fdiv(rt::fmul(rt::i2f(rt::rem_i32(({ let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(20); (c.m.r32(a_) as i32) }).wrapping_mul(4i32), 360i32)), F32(0x40490fda)), F32(0x43340000)); F::Port_sinf(c, a0_) }), { let a0_ = rt::fdiv(rt::fmul(rt::i2f(rt::rem_i32(({ let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(20); (c.m.r32(a_) as i32) }).wrapping_mul(5i32), 360i32)), F32(0x40490fda)), F32(0x43340000)); F::Port_sinf(c, a0_) }), { let a0_ = rt::fdiv(rt::fmul(rt::i2f(rt::rem_i32(({ let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(20); (c.m.r32(a_) as i32) }).wrapping_mul(6i32), 360i32)), F32(0x40490fda)), F32(0x43340000)); F::Port_sinf(c, a0_) }), F32(0x40c00000)); let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(16); c.m.w32(a_, (v_).0); };
    {
        l_part_14 = (c.m.r32(sym::gEftBurst)).wrapping_add(576);
        {
            l_j_3 = 0i32;
            'l7: loop {
                if !(((((l_j_3) < (350i32)) as i32)) != 0) { break 'l7; }
                'c8: {
                    { let v_ = 0i32; let a_ = (l_part_14).wrapping_add(100); c.m.w32(a_, (v_) as u32); };
                    if (((((c.m.r32((l_part_14).wrapping_add(84))) != (((0i32) as u32))) as i32)) != 0) {
                        if (({ let fp_ = c.m.r32((l_part_14).wrapping_add(84)); ((D::call(c, fp_, &[((l_part_14) as u64)])) as i32) }) != 0) {
                            { let v_ = 1i32; let a_ = (l_part_14).wrapping_add(100); c.m.w32(a_, (v_) as u32); };
                        }
                    }
                }
                { { let o_ = l_j_3; let n_ = (o_).wrapping_add(1); l_j_3 = n_; }; { let o_ = l_part_14; let n_ = (o_).wrapping_add(112); l_part_14 = n_; }; };
            }
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(4); (c.m.r32(a_) as i32) }) == (7i32)) as i32)) != 0) {
        { let _ = F::Snd_PlaySeEx(c, ((4i32) as u32), 74i32, 127i32, 64i32, (850i32).wrapping_neg()); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(4); (c.m.r32(a_) as i32) }) == (30i32)) as i32)) != 0) {
        { let _ = F::Snd_PlaySeEx(c, ((4i32) as u32), 74i32, 127i32, 64i32, (850i32).wrapping_neg()); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(4); (c.m.r32(a_) as i32) }) >= (31i32)) as i32)) != 0) {
        if (((((rt::rem_i32({ let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(4); (c.m.r32(a_) as i32) }, 60i32)) == (0i32)) as i32)) != 0) {
            if (((((rt::rem_i32(F::Port_Rand(c), 100i32)) < (20i32)) as i32)) != 0) {
                { let _ = F::Snd_PlaySeEx(c, ((4i32) as u32), 71i32, 127i32, 64i32, (500i32).wrapping_neg()); };
            }
        }
        { let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(39776); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(39776); (c.m.r32(a_) as i32) }) <= (0i32)) as i32)) != 0) {
            { let _ = F::Snd_PlaySeEx(c, ((4i32) as u32), 74i32, 127i32, 64i32, (850i32).wrapping_neg()); };
            { let v_ = (rt::rem_i32(F::Port_Rand(c), 80i32)).wrapping_add(60i32); let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(39776); c.m.w32(a_, (v_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftBurst_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_Draw", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_part_2: u32 = 0;
    let r_ = 'f: {
    if ((((((c.m.r32((sym::gEftBurstRes).wrapping_add(20)) as i32)) == (0i32)) as i32)) != 0) {
        F::Gfx_AddDefaultEnv(c);
        { let a0_ = c.m.r32((sym::gEftBurstRes).wrapping_add(12)); { let _ = D::call(c, fid::EftBurst_DrawModel, &[((a0_) as u64), ((0i32) as i64 as u64)]); } };
        { let _ = D::call(c, fid::EftBurst_SetTexture, &[((((0i32) as u32)) as u64), ((0i32) as i64 as u64), ((0i32) as i64 as u64)]); };
        l_part_2 = (c.m.r32(sym::gEftBurst)).wrapping_add(576);
        {
            l_i_1 = 0i32;
            'l1: loop {
                if !(((((l_i_1) < (350i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if ((((((c.m.r32((l_part_2).wrapping_add(100)) as i32)) != (0i32)) as i32)) != 0) {
                        { let fp_ = c.m.r32((l_part_2).wrapping_add(88)); { let _ = D::call(c, fp_, &[((l_part_2) as u64)]); } };
                    }
                }
                { { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; }; { let o_ = l_part_2; let n_ = (o_).wrapping_add(112); l_part_2 = n_; }; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftBurst_PostUpdate(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_PostUpdate", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftBurst_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::EftCam_Stop(c);
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftBurst_Start(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_Start", &PROBE_N, &[], false); }
    let mut l_pack_1: u32 = 0;
    let r_ = 'f: {
    l_pack_1 = { let a_ = (c.m.r32(sym::gCommonRes)).wrapping_add(52); c.m.r32(a_) };
    { let v_ = F::BtlScene_GetPackEntry(c, l_pack_1, 2i32); let a_ = (sym::gEftBurstRes).wrapping_add(12); c.m.w32(a_, v_); };
    { let a0_ = c.m.r32((sym::gEftBurstRes).wrapping_add(12)); { let _ = D::call(c, fid::EftBurst_RelocateModel, &[((a0_) as u64)]); } };
    { let v_ = F::BtlScene_GetPackEntry(c, l_pack_1, 3i32); let a_ = (sym::gEftBurstRes).wrapping_add(4); c.m.w32(a_, v_); };
    { let a1_ = c.m.r32((sym::gEftBurstRes).wrapping_add(4)); let a2_ = c.m.r32((sym::gEftBurstRes).wrapping_add(4)); F::Res_RelocateOffsets(c, (sym::gEftBurstRes).wrapping_add(4), a1_, a2_) };
    { let v_ = F::BtlScene_GetPackEntry(c, l_pack_1, 1i32); let a_ = (sym::gEftBurstRes).wrapping_add(8); c.m.w32(a_, v_); };
    { let v_ = 1i32; let a_ = (sym::gEftBurstRes).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (sym::gEftBurstRes).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_Start", &PROBE_N, &[], true); }
    r_
}

pub fn EftBurst_IsBusy(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_IsBusy", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((((((c.m.r32(sym::gEftBurst)) != (((0i32) as u32))) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gEftBurst)).wrapping_add(4); (c.m.r32(a_) as i32) }) < (150i32)) as i32)) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_IsBusy", &PROBE_N, &[], true); }
    r_
}

pub fn EftBurst_End(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_End", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 1i32; let a_ = (sym::gEftBurstRes).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBurst_End", &PROBE_N, &[], true); }
    r_
}

pub fn Eft_IsScreenPosVisible(c: &mut Ctx, mut p_pos_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Eft_IsScreenPosVisible", &PROBE_N, &[(p_pos_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
    l_p_2 = p_pos_1;
    if ((((((c.m.r32((l_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) <= (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((c.m.r32((l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) > (38911i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((c.m.r32((l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) < (26625i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((c.m.r32((l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) > (38143i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((c.m.r32((l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) < (27393i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Eft_IsScreenPosVisible", &PROBE_N, &[], true); }
    r_
}

pub fn Eft_IsScreenPosInFront(c: &mut Ctx, mut p_pos_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Eft_IsScreenPosInFront", &PROBE_N, &[(p_pos_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((((c.m.r32((p_pos_1).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) > (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Eft_IsScreenPosInFront", &PROBE_N, &[], true); }
    r_
}
