#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EftRibbon_AllocNode(c: &mut Ctx, mut p_w_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_AllocNode", &PROBE_N, &[(p_w_1) as u32], false); }
    let mut l_node_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_n_4: i32 = 0;
    let r_ = 'f: {
    l_node_2 = ((0i32) as u32);
    l_i_3 = { let a_ = (c.m.r32(sym::gEftRibbonMgr)).wrapping_add(4); (c.m.r32(a_) as i32) };
    {
        l_n_4 = 0i32;
        'l1: loop {
            if !(((((l_n_4) < (500i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((({ let a_ = ({ let a_ = c.m.r32(sym::gEftRibbonMgr); c.m.r32(a_) }).wrapping_add((((l_i_3) as u32)).wrapping_mul(80)); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    l_node_2 = ({ let a_ = c.m.r32(sym::gEftRibbonMgr); c.m.r32(a_) }).wrapping_add((((l_i_3) as u32)).wrapping_mul(80));
                    { let _ = F::memset(c, l_node_2, 0i32, 80u32); };
                    { let v_ = (l_i_3).wrapping_add(1i32); let a_ = (c.m.r32(sym::gEftRibbonMgr)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    if ((((({ let a_ = (c.m.r32(sym::gEftRibbonMgr)).wrapping_add(4); (c.m.r32(a_) as i32) }) >= (500i32)) as i32)) != 0) {
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gEftRibbonMgr)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    }
                    break 'l1;
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                if (((((l_i_3) >= (500i32)) as i32)) != 0) {
                    l_i_3 = 0i32;
                }
            }
            { let o_ = l_n_4; let n_ = (o_).wrapping_add(1); l_n_4 = n_; };
        }
    }
    if (((((l_node_2) != (((0i32) as u32))) as i32)) != 0) {
        if (((((c.m.r32(p_w_1)) == (((0i32) as u32))) as i32)) != 0) {
            { let v_ = l_node_2; let a_ = p_w_1; c.m.w32(a_, v_); };
        }
        if (((((c.m.r32((p_w_1).wrapping_add(4))) == (((0i32) as u32))) as i32)) != 0) {
            { let v_ = l_node_2; let a_ = (p_w_1).wrapping_add(4); c.m.w32(a_, v_); };
        } else {
            { let v_ = c.m.r32((p_w_1).wrapping_add(4)); let a_ = (l_node_2).wrapping_add(68); c.m.w32(a_, v_); };
            { let v_ = l_node_2; let a_ = (c.m.r32((p_w_1).wrapping_add(4))).wrapping_add(64); c.m.w32(a_, v_); };
            { let v_ = l_node_2; let a_ = (p_w_1).wrapping_add(4); c.m.w32(a_, v_); };
        }
    }
    break 'f l_node_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_AllocNode", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbon_FreeNodes(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_FreeNodes", &PROBE_N, &[(p_w_1) as u32], false); }
    let mut l_n_2: u32 = 0;
    let mut l_next_3: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(p_w_1)) != (((0i32) as u32))) as i32)) != 0) {
        l_n_2 = c.m.r32(p_w_1);
        'l1: loop {
            'c2: {
                l_next_3 = c.m.r32((l_n_2).wrapping_add(64));
                { let v_ = 0i32; let a_ = l_n_2; c.m.w32(a_, (v_) as u32); };
                l_n_2 = l_next_3;
            }
            if !(((((l_next_3) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
        }
    }
    { let v_ = ((0i32) as u32); let a_ = p_w_1; c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (p_w_1).wrapping_add(4); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_FreeNodes", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbon_SetTexPair(c: &mut Ctx, mut p_w_1: u32, mut p_slot_2: i32, mut p_tex_3: u32, mut p_image_4: i32, mut p_palette_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_SetTexPair", &PROBE_N, &[(p_w_1) as u32, (p_slot_2) as u32, (p_tex_3) as u32, (p_image_4) as u32, (p_palette_5) as u32], false); }
    let r_ = 'f: {
    { let s_ = (p_tex_3).wrapping_add((((p_image_4) as u32)).wrapping_mul(16)); let d_ = (((p_w_1).wrapping_add(304)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(32))).wrapping_add((((0i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
    { let s_ = (p_tex_3).wrapping_add((((p_palette_5) as u32)).wrapping_mul(16)); let d_ = (((p_w_1).wrapping_add(304)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(32))).wrapping_add((((1i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
    { let v_ = p_palette_5; let a_ = ((p_w_1).wrapping_add(16)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_SetTexPair", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbon_UpdateTex(c: &mut Ctx, mut p_prm_1: u32, mut p_w_2: u32, mut p_arg_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_UpdateTex", &PROBE_N, &[(p_prm_1) as u32, (p_w_2) as u32, (p_arg_3) as u32], false); }
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    if ((((!(((({ let a_ = (c.m.r32((p_arg_3).wrapping_add(52))).wrapping_add(260); c.m.r32(a_) }) & ((((1i32).wrapping_shl(((c.m.r32((p_arg_3).wrapping_add(40)) as i32)) as u32)) as u32)))) != 0)) as i32)) != 0) {
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < ((c.m.r32((p_w_2).wrapping_add(300)) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = F::EftVram_AddImage(c, (((p_w_2).wrapping_add(304)).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add((((0i32) as u32)).wrapping_mul(16)), 1i32, 0i32); let a_ = ((p_w_2).wrapping_add(432)).wrapping_add((((l_i_4) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = (F::EftVram_AddClut(c, (((p_w_2).wrapping_add(304)).wrapping_add((((l_i_4) as u32)).wrapping_mul(32))).wrapping_add((((1i32) as u32)).wrapping_mul(16)))).wrapping_shl((37i32) as u32); let a_ = ((p_w_2).wrapping_add(432)).wrapping_add((((l_i_4) as u32)).wrapping_mul(8)); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
                    { let v_ = c.m.r64(((p_w_2).wrapping_add(432)).wrapping_add((((l_i_4) as u32)).wrapping_mul(8))); let a_ = (c.m.r32((p_arg_3).wrapping_add(52))).wrapping_add((((((c.m.r32((p_arg_3).wrapping_add(40)) as i32)).wrapping_add(l_i_4)) as u32)).wrapping_mul(16)); c.m.w64(a_, v_); };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        { let r_ = (((1i32).wrapping_shl(((c.m.r32((p_arg_3).wrapping_add(40)) as i32)) as u32)) as u32); let a_ = (c.m.r32((p_arg_3).wrapping_add(52))).wrapping_add(260); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    } else {
        {
            l_i_4 = 0i32;
            'l3: loop {
                if !(((((l_i_4) < ((c.m.r32((p_w_2).wrapping_add(300)) as i32))) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let v_ = { let a_ = (c.m.r32((p_arg_3).wrapping_add(52))).wrapping_add((((((c.m.r32((p_arg_3).wrapping_add(40)) as i32)).wrapping_add(l_i_4)) as u32)).wrapping_mul(16)); c.m.r64(a_) }; let a_ = ((p_w_2).wrapping_add(432)).wrapping_add((((l_i_4) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_UpdateTex", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbon_LoadKey(c: &mut Ctx, mut p_cur_1: u32, mut p_arg_2: u32, mut p_key_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_LoadKey", &PROBE_N, &[(p_cur_1) as u32, (p_arg_2) as u32, (p_key_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_prm_4: u32 = 0;
    let mut l_anim_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let r_ = 'f: {
        l_prm_4 = c.m.r32((p_arg_2).wrapping_add(56));
        l_anim_5 = c.m.r32((p_arg_2).wrapping_add(60));
        F::Vec4_Copy(c, p_cur_1, (l_anim_5).wrapping_add((((p_key_3) as u32)).wrapping_mul(16)));
        {
            l_i_6 = 0i32;
            'l1: loop {
                if !(((((l_i_6) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = F32(c.m.r32((((l_anim_5).wrapping_add(96)).wrapping_add((((p_key_3) as u32)).wrapping_mul(8))).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)))); let a_ = (fp).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((((l_anim_5).wrapping_add(120)).wrapping_add((((p_key_3) as u32)).wrapping_mul(8))).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)))); let a_ = (fp.wrapping_add(16)).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((((l_anim_5).wrapping_add(144)).wrapping_add((((p_key_3) as u32)).wrapping_mul(8))).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)))); let a_ = (fp.wrapping_add(32)).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
        { let v_ = F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_cur_1).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_cur_1).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_cur_1).wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((p_cur_1).wrapping_add(28)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((p_cur_1).wrapping_add(28)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((p_cur_1).wrapping_add(28)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_anim_5).wrapping_add(168)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x41f00000)); let a_ = (p_cur_1).wrapping_add(40); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_prm_4).wrapping_add(24)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (p_cur_1).wrapping_add(44); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_prm_4).wrapping_add(36)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_prm_4).wrapping_add(48)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_cur_1).wrapping_add(48); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (p_cur_1).wrapping_add(52); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_prm_4).wrapping_add(60)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x41f00000)); let a_ = (p_cur_1).wrapping_add(56); c.m.w32(a_, (v_).0); };
        if (((((c.m.r32(l_prm_4) as i32)) & (4i32))) != 0) {
            { let v_ = F32(c.m.r32(((l_prm_4).wrapping_add(72)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (p_cur_1).wrapping_add(60); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32(((l_prm_4).wrapping_add(84)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (p_cur_1).wrapping_add(64); c.m.w32(a_, (v_).0); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_LoadKey", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbon_UpdateKeys(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_UpdateKeys", &PROBE_N, &[(p_w_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_arg_2: u32 = 0;
    let mut l_cur_3: u32 = 0;
    let mut l_anim_4: u32 = 0;
    let mut l_prm_5: u32 = 0;
    let mut l_t_6: F32 = F32(0);
    let mut l_k0_7: i32 = 0;
    let mut l_k1_8: i32 = 0;
    let mut l_i_9: i32 = 0;
    let r_ = 'f: {
        l_arg_2 = (p_w_1).wrapping_add(16);
        l_cur_3 = (p_w_1).wrapping_add(80);
        l_anim_4 = c.m.r32((l_arg_2).wrapping_add(60));
        l_prm_5 = c.m.r32((l_arg_2).wrapping_add(56));
        if (((rt::flt(F32(c.m.r32((p_w_1).wrapping_add(248))), F32(c.m.r32((p_w_1).wrapping_add(256)))) as i32)) != 0) {
            l_t_6 = rt::fdiv(F32(c.m.r32((p_w_1).wrapping_add(248))), F32(c.m.r32((p_w_1).wrapping_add(256))));
            l_k0_7 = 0i32;
            l_k1_8 = 1i32;
        } else {
            l_t_6 = rt::fdiv(rt::fsub(F32(c.m.r32((p_w_1).wrapping_add(248))), F32(c.m.r32((p_w_1).wrapping_add(256)))), rt::fsub(F32(c.m.r32((p_w_1).wrapping_add(252))), F32(c.m.r32((p_w_1).wrapping_add(256)))));
            l_k0_7 = 1i32;
            l_k1_8 = 2i32;
        }
        F::Vec4_Sub(c, fp, (l_anim_4).wrapping_add((((l_k1_8) as u32)).wrapping_mul(16)), (l_anim_4).wrapping_add((((l_k0_7) as u32)).wrapping_mul(16)));
        { let v_ = rt::fadd(F32(c.m.r32((l_anim_4).wrapping_add((((l_k0_7) as u32)).wrapping_mul(16)))), rt::fmul(F32(c.m.r32(fp)), l_t_6)); let a_ = l_cur_3; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_anim_4).wrapping_add((((l_k0_7) as u32)).wrapping_mul(16))).wrapping_add(4))), rt::fmul(F32(c.m.r32((fp).wrapping_add(4))), l_t_6)); let a_ = (l_cur_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_anim_4).wrapping_add((((l_k0_7) as u32)).wrapping_mul(16))).wrapping_add(8))), rt::fmul(F32(c.m.r32((fp).wrapping_add(8))), l_t_6)); let a_ = (l_cur_3).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_anim_4).wrapping_add((((l_k0_7) as u32)).wrapping_mul(16))).wrapping_add(12))), rt::fmul(F32(c.m.r32((fp).wrapping_add(12))), l_t_6)); let a_ = (l_cur_3).wrapping_add(12); c.m.w32(a_, (v_).0); };
        {
            l_i_9 = 0i32;
            'l1: loop {
                if !(((((l_i_9) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = rt::fsub(F32(c.m.r32((((l_anim_4).wrapping_add(96)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(8))).wrapping_add((((l_i_9) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_anim_4).wrapping_add(96)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(8))).wrapping_add((((l_i_9) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(F32(c.m.r32((((l_anim_4).wrapping_add(120)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(8))).wrapping_add((((l_i_9) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_anim_4).wrapping_add(120)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(8))).wrapping_add((((l_i_9) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(F32(c.m.r32((((l_anim_4).wrapping_add(144)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(8))).wrapping_add((((l_i_9) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_anim_4).wrapping_add(144)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(8))).wrapping_add((((l_i_9) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32((((l_anim_4).wrapping_add(96)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(8))).wrapping_add((((l_i_9) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = (fp.wrapping_add(16)).wrapping_add((((l_i_9) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32((((l_anim_4).wrapping_add(120)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(8))).wrapping_add((((l_i_9) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = (fp.wrapping_add(32)).wrapping_add((((l_i_9) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32((((l_anim_4).wrapping_add(144)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(8))).wrapping_add((((l_i_9) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = (fp.wrapping_add(48)).wrapping_add((((l_i_9) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_9; let n_ = (o_).wrapping_add(1); l_i_9 = n_; };
            }
        }
        { let v_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((l_cur_3).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((l_cur_3).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((l_cur_3).wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((l_cur_3).wrapping_add(28)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((l_cur_3).wrapping_add(28)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((l_cur_3).wrapping_add(28)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_anim_4).wrapping_add(168)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_anim_4).wrapping_add(168)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_anim_4).wrapping_add(168)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_6)), F32(0x41f00000)); let a_ = (l_cur_3).wrapping_add(40); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_prm_5).wrapping_add(24)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_prm_5).wrapping_add(24)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_prm_5).wrapping_add(24)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = (l_cur_3).wrapping_add(44); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_prm_5).wrapping_add(36)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_prm_5).wrapping_add(36)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_prm_5).wrapping_add(48)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_prm_5).wrapping_add(48)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_prm_5).wrapping_add(60)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_prm_5).wrapping_add(60)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_prm_5).wrapping_add(36)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = (fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_prm_5).wrapping_add(48)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = (fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (l_cur_3).wrapping_add(48); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (l_cur_3).wrapping_add(52); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_prm_5).wrapping_add(60)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_t_6)), F32(0x41f00000)); let a_ = (l_cur_3).wrapping_add(56); c.m.w32(a_, (v_).0); };
        if (((((c.m.r32(l_prm_5) as i32)) & (4i32))) != 0) {
            { let v_ = rt::fsub(F32(c.m.r32(((l_prm_5).wrapping_add(72)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_prm_5).wrapping_add(72)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fadd(F32(c.m.r32(((l_prm_5).wrapping_add(72)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = (l_cur_3).wrapping_add(60); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fsub(F32(c.m.r32(((l_prm_5).wrapping_add(84)).wrapping_add((((l_k1_8) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_prm_5).wrapping_add(84)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fadd(F32(c.m.r32(((l_prm_5).wrapping_add(84)).wrapping_add((((l_k0_7) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = (l_cur_3).wrapping_add(64); c.m.w32(a_, (v_).0); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_UpdateKeys", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbon_AddNode(c: &mut Ctx, mut p_w_1: u32, mut p_pos_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_AddNode", &PROBE_N, &[(p_w_1) as u32, (p_pos_2) as u32], false); }
    let mut l_n_3: u32 = 0;
    let r_ = 'f: {
    l_n_3 = F::EftRibbon_AllocNode(c, p_w_1);
    if (((((l_n_3) != (((0i32) as u32))) as i32)) != 0) {
        { let r_ = 1i32; let a_ = l_n_3; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        if (((((c.m.r32((p_w_1).wrapping_add(4))) != (((0i32) as u32))) as i32)) != 0) {
            { let a1_ = (c.m.r32((p_w_1).wrapping_add(4))).wrapping_add(16); F::Vec4_Copy(c, (l_n_3).wrapping_add(16), a1_) };
        } else {
            F::Vec4_Copy(c, (l_n_3).wrapping_add(16), p_pos_2);
        }
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_AddNode", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbon_InitNodes(c: &mut Ctx, mut p_w_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_InitNodes", &PROBE_N, &[(p_w_1) as u32, (p_arg_2) as u32], false); }
    let mut l_prm_3: u32 = 0;
    let mut l_cur_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
    l_prm_3 = c.m.r32((p_arg_2).wrapping_add(56));
    l_cur_4 = (p_w_1).wrapping_add(80);
    if (((((c.m.r32(l_prm_3) as i32)) & (4i32))) != 0) {
        { let r_ = 32i32; let a_ = (p_w_1).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if (((((c.m.r32(l_prm_3) as i32)) & (2i32))) != 0) {
        if ((((!((((F::Port_Rand(c)) & (1i32))) != 0)) as i32)) != 0) {
            { let r_ = 64i32; let a_ = (p_w_1).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
    }
    F::Vec4_Copy(c, (p_w_1).wrapping_add(160), p_arg_2);
    { let v_ = F32(c.m.r32((l_cur_4).wrapping_add(44))); let a_ = (p_w_1).wrapping_add(260); c.m.w32(a_, (v_).0); };
    F::Vec3_Copy(c, (p_w_1).wrapping_add(176), l_cur_4);
    { let v_ = F32(0x00000000); let a_ = ((p_w_1).wrapping_add(176)).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(F32(c.m.r32((l_prm_3).wrapping_add(8))), F32(0x41f00000)); let a_ = (p_w_1).wrapping_add(280); c.m.w32(a_, (v_).0); };
    if (((rt::fle(F32(c.m.r32((p_w_1).wrapping_add(280))), F32(c.m.r32(((p_w_1).wrapping_add(176)).wrapping_add(12)))) as i32)) != 0) {
        { let r_ = 128i32; let a_ = (p_w_1).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    { let v_ = rt::fmul(F32(c.m.r32((l_prm_3).wrapping_add(12))), F32(0x41f00000)); let a_ = (p_w_1).wrapping_add(284); c.m.w32(a_, (v_).0); };
    if (((rt::fle(F32(c.m.r32((p_w_1).wrapping_add(284))), F32(c.m.r32(((p_w_1).wrapping_add(176)).wrapping_add(12)))) as i32)) != 0) {
        { let r_ = 256i32; let a_ = (p_w_1).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        { let v_ = F32(c.m.r32((p_w_1).wrapping_add(284))); let a_ = (p_w_1).wrapping_add(276); c.m.w32(a_, (v_).0); };
    }
    if (((((c.m.r32((p_w_1).wrapping_add(208)) as i32)) & (32i32))) != 0) {
        { let v_ = F32(c.m.r32((l_cur_4).wrapping_add(60))); let a_ = (p_w_1).wrapping_add(288); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((l_cur_4).wrapping_add(64))); let a_ = (p_w_1).wrapping_add(292); c.m.w32(a_, (v_).0); };
    }
    if (((((((c.m.r8((l_prm_3).wrapping_add(5))) as i32)) == (0i32)) as i32)) != 0) {
        if (((((c.m.r32((p_w_1).wrapping_add(208)) as i32)) & (32i32))) != 0) {
            { let v_ = 4i32; let a_ = (p_w_1).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 3i32; let a_ = (p_w_1).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
        }
        { let v_ = 32i32; let a_ = (p_w_1).wrapping_add(220); c.m.w32(a_, (v_) as u32); };
    } else {
        if (((((((c.m.r8((l_prm_3).wrapping_add(5))) as i32)) == (1i32)) as i32)) != 0) {
            { let v_ = 1i32; let a_ = (p_w_1).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
            { let v_ = 16i32; let a_ = (p_w_1).wrapping_add(220); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 4i32; let a_ = (p_w_1).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
            { let v_ = 20i32; let a_ = (p_w_1).wrapping_add(220); c.m.w32(a_, (v_) as u32); };
        }
    }
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((l_i_5) < ((c.m.r32((p_w_1).wrapping_add(300)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a2_ = c.m.r32((p_arg_2).wrapping_add(52)); let a3_ = ((c.m.r32((p_arg_2).wrapping_add(40)) as i32)).wrapping_add(l_i_5); let a4_ = ((c.m.r32((p_arg_2).wrapping_add(40)) as i32)).wrapping_add(l_i_5); F::EftRibbon_SetTexPair(c, p_w_1, l_i_5, a2_, a3_, a4_) };
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_InitNodes", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbon_PlaceStrip(c: &mut Ctx, mut p_prm_1: u32, mut p_w_2: u32, mut p_start_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_PlaceStrip", &PROBE_N, &[(p_prm_1) as u32, (p_w_2) as u32, (p_start_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_r_4: u32 = 0;
    let mut l_n_5: u32 = 0;
    let mut l_next_6: u32 = 0;
    let mut l_i_7: i32 = 0;
    let r_ = 'f: {
        l_i_7 = 0i32;
        F::Vec4_Copy(c, fp, p_start_3);
        F::Vec3_Sub(c, fp.wrapping_add(16), (p_w_2).wrapping_add(160), fp);
        { let a2_ = rt::i2f((c.m.r32((p_w_2).wrapping_add(228)) as i32)); F::Vec3_Div(c, fp.wrapping_add(16), fp.wrapping_add(16), a2_) };
        l_r_4 = p_w_2;
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        l_n_5 = c.m.r32(l_r_4);
        if (((((l_n_5) != (((0i32) as u32))) as i32)) != 0) {
            'l1: loop {
                'c2: {
                    F::Vec3_Copy(c, (l_n_5).wrapping_add(16), fp);
                    if (((((l_i_7) == (0i32)) as i32)) != 0) {
                        { let v_ = ((0i32) as u8); let a_ = (l_n_5).wrapping_add(4); c.m.w8(a_, v_); };
                    } else {
                        if (((((l_i_7) < (((c.m.r32((p_w_2).wrapping_add(228)) as i32)).wrapping_sub(2i32))) as i32)) != 0) {
                            { let v_ = ((1i32) as u8); let a_ = (l_n_5).wrapping_add(4); c.m.w8(a_, v_); };
                        } else {
                            { let v_ = ((2i32) as u8); let a_ = (l_n_5).wrapping_add(4); c.m.w8(a_, v_); };
                        }
                    }
                    F::Vec4_Copy(c, (l_n_5).wrapping_add(32), (l_r_4).wrapping_add(176));
                    if (((((c.m.r32((p_w_2).wrapping_add(208)) as i32)) & (32i32))) != 0) {
                        F::Vec4_Copy(c, (l_n_5).wrapping_add(48), (p_w_2).wrapping_add(192));
                        if (((((((((l_i_7) == (0i32)) as i32)) != 0) || (((((c.m.r32((l_n_5).wrapping_add(64))) == (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
                            { let v_ = F32(0x00000000); let a_ = ((l_n_5).wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        }
                    }
                    l_r_4 = p_w_2;
                    if (((((l_i_7) < (((c.m.r32((l_r_4).wrapping_add(228)) as i32)).wrapping_sub(2i32))) as i32)) != 0) {
                        F::Vec3_Add(c, fp, fp, fp.wrapping_add(16));
                    } else {
                        F::Vec3_Copy(c, fp, (l_r_4).wrapping_add(160));
                    }
                    l_next_6 = c.m.r32((l_n_5).wrapping_add(64));
                    { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
                    l_n_5 = l_next_6;
                }
                if !(((((l_next_6) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_PlaceStrip", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbon_PlaceTrail1(c: &mut Ctx, mut p_prm_1: u32, mut p_w_2: u32, mut p_arg_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_PlaceTrail1", &PROBE_N, &[(p_prm_1) as u32, (p_w_2) as u32, (p_arg_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_n_4: u32 = 0;
    let mut l_next_5: u32 = 0;
    let r_ = 'f: {
        F::Vec4_Copy(c, fp, (p_w_2).wrapping_add(160));
        l_n_4 = c.m.r32(p_w_2);
        if (((((l_n_4) != (((0i32) as u32))) as i32)) != 0) {
            'l1: loop {
                'c2: {
                    F::Vec4_Copy(c, fp.wrapping_add(16), (l_n_4).wrapping_add(16));
                    F::Vec4_Copy(c, (l_n_4).wrapping_add(16), fp);
                    F::Vec4_Copy(c, fp, fp.wrapping_add(16));
                    { let v_ = ((0i32) as u8); let a_ = (l_n_4).wrapping_add(4); c.m.w8(a_, v_); };
                    l_next_5 = c.m.r32((l_n_4).wrapping_add(64));
                    l_n_4 = l_next_5;
                }
                if !(((((l_next_5) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_PlaceTrail1", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbon_PlaceTrail2(c: &mut Ctx, mut p_prm_1: u32, mut p_w_2: u32, mut p_arg_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_PlaceTrail2", &PROBE_N, &[(p_prm_1) as u32, (p_w_2) as u32, (p_arg_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_r_4: u32 = 0;
    let mut l_n_5: u32 = 0;
    let mut l_next_6: u32 = 0;
    let mut l_i_7: i32 = 0;
    let r_ = 'f: {
        l_i_7 = 0i32;
        F::Vec4_Copy(c, fp, (p_w_2).wrapping_add(160));
        l_r_4 = p_w_2;
        l_n_5 = c.m.r32(l_r_4);
        if (((((l_n_5) != (((0i32) as u32))) as i32)) != 0) {
            'l1: loop {
                'c2: {
                    F::Vec3_Copy(c, fp.wrapping_add(16), (l_n_5).wrapping_add(16));
                    F::Vec3_Copy(c, (l_n_5).wrapping_add(16), fp);
                    if (((((l_i_7) == (0i32)) as i32)) != 0) {
                        { let v_ = ((2i32) as u8); let a_ = (l_n_5).wrapping_add(4); c.m.w8(a_, v_); };
                    } else {
                        l_r_4 = p_w_2;
                        if (((((l_i_7) < (((c.m.r32((l_r_4).wrapping_add(228)) as i32)).wrapping_sub(2i32))) as i32)) != 0) {
                            { let v_ = ((1i32) as u8); let a_ = (l_n_5).wrapping_add(4); c.m.w8(a_, v_); };
                        } else {
                            { let v_ = ((0i32) as u8); let a_ = (l_n_5).wrapping_add(4); c.m.w8(a_, v_); };
                        }
                    }
                    F::Vec4_Copy(c, (l_n_5).wrapping_add(32), (l_r_4).wrapping_add(176));
                    { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
                    F::Vec3_Copy(c, fp, fp.wrapping_add(16));
                    l_next_6 = c.m.r32((l_n_5).wrapping_add(64));
                    l_n_5 = l_next_6;
                }
                if !(((((l_next_6) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_PlaceTrail2", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbon_DrawStrip(c: &mut Ctx, mut p_w_1: u32, mut p_arg_2: u32, mut p_prm_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_DrawStrip", &PROBE_N, &[(p_w_1) as u32, (p_arg_2) as u32, (p_prm_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(720) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 720);
    let mut l_first_4: i32 = 0;
    let mut l_n_5: u32 = 0;
    let mut l_next_6: u32 = 0;
    let mut l_i_7: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 64);
        rt::memset(c, fp.wrapping_add(0), 0, 16);
        rt::memset(c, fp.wrapping_add(0).wrapping_add(0), 0, 16);
        { let v_ = F32(0x3ca3d70a); c.m.w32(fp.wrapping_add(0).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(0).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(16), 0, 16);
        rt::memset(c, fp.wrapping_add(16).wrapping_add(0), 0, 16);
        { let v_ = F32(0x3ca3d70a); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f7ae147); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(32), 0, 16);
        rt::memset(c, fp.wrapping_add(32).wrapping_add(0), 0, 16);
        { let v_ = F32(0x3f7ae147); c.m.w32(fp.wrapping_add(32).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(32).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(32).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(32).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(48), 0, 16);
        rt::memset(c, fp.wrapping_add(48).wrapping_add(0), 0, 16);
        { let v_ = F32(0x3f7ae147); c.m.w32(fp.wrapping_add(48).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f7ae147); c.m.w32(fp.wrapping_add(48).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(48).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(48).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(64), 0, 64);
        rt::memset(c, fp.wrapping_add(64).wrapping_add(0), 0, 16);
        rt::memset(c, fp.wrapping_add(64).wrapping_add(0).wrapping_add(0), 0, 16);
        { let v_ = F32(0x3ca3d70a); c.m.w32(fp.wrapping_add(64).wrapping_add(0).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f7ae147); c.m.w32(fp.wrapping_add(64).wrapping_add(0).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(64).wrapping_add(0).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(0).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(64).wrapping_add(16), 0, 16);
        rt::memset(c, fp.wrapping_add(64).wrapping_add(16).wrapping_add(0), 0, 16);
        { let v_ = F32(0x3ca3d70a); c.m.w32(fp.wrapping_add(64).wrapping_add(16).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(16).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(64).wrapping_add(16).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(16).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(64).wrapping_add(32), 0, 16);
        rt::memset(c, fp.wrapping_add(64).wrapping_add(32).wrapping_add(0), 0, 16);
        { let v_ = F32(0x3f7ae147); c.m.w32(fp.wrapping_add(64).wrapping_add(32).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f7ae147); c.m.w32(fp.wrapping_add(64).wrapping_add(32).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(64).wrapping_add(32).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(32).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(64).wrapping_add(48), 0, 16);
        rt::memset(c, fp.wrapping_add(64).wrapping_add(48).wrapping_add(0), 0, 16);
        { let v_ = F32(0x3f7ae147); c.m.w32(fp.wrapping_add(64).wrapping_add(48).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(48).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(64).wrapping_add(48).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(48).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(560), 0, 16);
        rt::memset(c, fp.wrapping_add(560).wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(560).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(560).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(560).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(560).wrapping_add(0).wrapping_add(12), (v_).0); }
        l_first_4 = 1i32;
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(544); F::Vec4_Copy(c, fp.wrapping_add(320), a1_) };
        if ((((!(((((c.m.r32((p_w_1).wrapping_add(208)) as i32)) & (64i32))) != 0)) as i32)) != 0) {
            {
                l_i_7 = 0i32;
                'l1: loop {
                    if !(((((l_i_7) < (4i32)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        F::Vec4_Copy(c, (fp.wrapping_add(368)).wrapping_add((((l_i_7) as u32)).wrapping_mul(16)), (fp).wrapping_add((((l_i_7) as u32)).wrapping_mul(16)));
                    }
                    { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
                }
            }
            if (((((c.m.r32((p_w_1).wrapping_add(208)) as i32)) & (32i32))) != 0) {
                {
                    l_i_7 = 0i32;
                    'l3: loop {
                        if !(((((l_i_7) < (4i32)) as i32)) != 0) { break 'l3; }
                        'c4: {
                            F::Vec4_Copy(c, (fp.wrapping_add(432)).wrapping_add((((l_i_7) as u32)).wrapping_mul(16)), (fp).wrapping_add((((l_i_7) as u32)).wrapping_mul(16)));
                            { let r_ = F32(c.m.r32((p_w_1).wrapping_add(296))); let a_ = (fp.wrapping_add(432)).wrapping_add((((l_i_7) as u32)).wrapping_mul(16)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        }
                        { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
                    }
                }
            }
        } else {
            {
                l_i_7 = 0i32;
                'l5: loop {
                    if !(((((l_i_7) < (4i32)) as i32)) != 0) { break 'l5; }
                    'c6: {
                        F::Vec4_Copy(c, (fp.wrapping_add(368)).wrapping_add((((l_i_7) as u32)).wrapping_mul(16)), (fp.wrapping_add(64)).wrapping_add((((l_i_7) as u32)).wrapping_mul(16)));
                    }
                    { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
                }
            }
            if (((((c.m.r32((p_w_1).wrapping_add(208)) as i32)) & (32i32))) != 0) {
                {
                    l_i_7 = 0i32;
                    'l7: loop {
                        if !(((((l_i_7) < (4i32)) as i32)) != 0) { break 'l7; }
                        'c8: {
                            F::Vec4_Copy(c, (fp.wrapping_add(432)).wrapping_add((((l_i_7) as u32)).wrapping_mul(16)), (fp.wrapping_add(64)).wrapping_add((((l_i_7) as u32)).wrapping_mul(16)));
                            { let r_ = F32(c.m.r32((p_w_1).wrapping_add(296))); let a_ = (fp.wrapping_add(432)).wrapping_add((((l_i_7) as u32)).wrapping_mul(16)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        }
                        { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
                    }
                }
            }
        }
        {
            l_n_5 = c.m.r32(p_w_1);
            'l9: loop {
                if !(((((l_n_5) != (((0i32) as u32))) as i32)) != 0) { break 'l9; }
                'c10: {
                    l_next_6 = c.m.r32((l_n_5).wrapping_add(64));
                    if (((((l_next_6) != (((0i32) as u32))) as i32)) != 0) {
                        F::Vec4_Sub(c, fp.wrapping_add(336), (l_next_6).wrapping_add(16), (l_n_5).wrapping_add(16));
                        F::Vec4_Sub(c, fp.wrapping_add(352), (l_n_5).wrapping_add(16), fp.wrapping_add(320));
                        F::Vec3_Cross(c, fp.wrapping_add(336), fp.wrapping_add(336), fp.wrapping_add(352));
                        F::Vec3_Normalize(c, fp.wrapping_add(336), fp.wrapping_add(336));
                        { let a2_ = F32(c.m.r32((p_w_1).wrapping_add(260))); F::Vec3_Scale(c, fp.wrapping_add(560), fp.wrapping_add(336), a2_) };
                        if ((l_first_4) != 0) {
                            F::Vec3_Sub(c, (fp.wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (l_n_5).wrapping_add(16), fp.wrapping_add(560));
                            F::Vec3_Add(c, (fp.wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (l_n_5).wrapping_add(16), fp.wrapping_add(560));
                            { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        } else {
                            F::Vec4_Copy(c, (fp.wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
                            F::Vec4_Copy(c, (fp.wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
                        }
                        F::Vec3_Sub(c, (fp.wrapping_add(128)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (l_next_6).wrapping_add(16), fp.wrapping_add(560));
                        F::Vec3_Add(c, (fp.wrapping_add(128)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (l_next_6).wrapping_add(16), fp.wrapping_add(560));
                        { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(128)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(128)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        F::Vec4_Copy(c, (fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(128)).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
                        F::Vec4_Copy(c, (fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(128)).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
                        if (((((c.m.r32((p_w_1).wrapping_add(208)) as i32)) & (32i32))) != 0) {
                            if ((l_first_4) != 0) {
                                { let a2_ = F32(c.m.r32((p_w_1).wrapping_add(288))); F::Vec3_Scale(c, fp.wrapping_add(560), fp.wrapping_add(336), a2_) };
                                F::Vec3_Sub(c, (fp.wrapping_add(224)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (l_n_5).wrapping_add(16), fp.wrapping_add(560));
                                F::Vec3_Add(c, (fp.wrapping_add(224)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (l_n_5).wrapping_add(16), fp.wrapping_add(560));
                                { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(224)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(224)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                            } else {
                                F::Vec4_Copy(c, (fp.wrapping_add(224)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(288)).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
                                F::Vec4_Copy(c, (fp.wrapping_add(224)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(288)).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
                            }
                            { let a2_ = F32(c.m.r32((p_w_1).wrapping_add(288))); F::Vec3_Scale(c, fp.wrapping_add(560), fp.wrapping_add(336), a2_) };
                            F::Vec3_Sub(c, (fp.wrapping_add(224)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (l_next_6).wrapping_add(16), fp.wrapping_add(560));
                            F::Vec3_Add(c, (fp.wrapping_add(224)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (l_next_6).wrapping_add(16), fp.wrapping_add(560));
                            { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(224)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(224)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                            F::Vec4_Copy(c, (fp.wrapping_add(288)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(224)).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
                            F::Vec4_Copy(c, (fp.wrapping_add(288)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(224)).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
                        }
                        F::Vec4_Copy(c, (fp.wrapping_add(496)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (l_n_5).wrapping_add(32));
                        F::Vec4_Copy(c, (fp.wrapping_add(496)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (l_n_5).wrapping_add(32));
                        l_first_4 = 0i32;
                        F::Vec4_Copy(c, (fp.wrapping_add(496)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (l_next_6).wrapping_add(32));
                        F::Vec4_Copy(c, (fp.wrapping_add(496)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (l_next_6).wrapping_add(32));
                        {
                            l_i_7 = 0i32;
                            'l11: loop {
                                if !(((((l_i_7) < (2i32)) as i32)) != 0) { break 'l11; }
                                'c12: {
                                    F::ClipVtx_Set(c, (fp.wrapping_add(576)).wrapping_add((((0i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(128)).wrapping_add((((l_i_7) as u32)).wrapping_mul(16)), (fp.wrapping_add(368)).wrapping_add((((l_i_7) as u32)).wrapping_mul(16)), (fp.wrapping_add(496)).wrapping_add((((l_i_7) as u32)).wrapping_mul(16)));
                                    F::ClipVtx_Set(c, (fp.wrapping_add(576)).wrapping_add((((1i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(128)).wrapping_add(((((l_i_7).wrapping_add(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(368)).wrapping_add(((((l_i_7).wrapping_add(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(496)).wrapping_add(((((l_i_7).wrapping_add(1i32)) as u32)).wrapping_mul(16)));
                                    F::ClipVtx_Set(c, (fp.wrapping_add(576)).wrapping_add((((2i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(128)).wrapping_add(((((l_i_7).wrapping_add(2i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(368)).wrapping_add(((((l_i_7).wrapping_add(2i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(496)).wrapping_add(((((l_i_7).wrapping_add(2i32)) as u32)).wrapping_mul(16)));
                                    { let a1_ = ((c.m.r8((p_prm_3).wrapping_add(4))) as i32); let a6_ = { let a_ = ((p_w_1).wrapping_add(432)).wrapping_add((((c.m.r8((l_n_5).wrapping_add(4))) as u32)).wrapping_mul(8)); c.m.r64(a_) }; F::EftGfx_DrawPolyScaledZ(c, fp.wrapping_add(576), a1_, 0i32, 0i32, 0i32, 0i32, a6_, F32(0x3f800000)) };
                                }
                                { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
                            }
                        }
                        if (((((c.m.r32((p_w_1).wrapping_add(208)) as i32)) & (32i32))) != 0) {
                            F::Vec4_Copy(c, (fp.wrapping_add(496)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (l_n_5).wrapping_add(48));
                            F::Vec4_Copy(c, (fp.wrapping_add(496)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (l_n_5).wrapping_add(48));
                            F::Vec4_Copy(c, (fp.wrapping_add(496)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (l_next_6).wrapping_add(48));
                            F::Vec4_Copy(c, (fp.wrapping_add(496)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (l_next_6).wrapping_add(48));
                            {
                                l_i_7 = 0i32;
                                'l13: loop {
                                    if !(((((l_i_7) < (2i32)) as i32)) != 0) { break 'l13; }
                                    'c14: {
                                        F::ClipVtx_Set(c, (fp.wrapping_add(576)).wrapping_add((((0i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(128)).wrapping_add((((l_i_7) as u32)).wrapping_mul(16)), (fp.wrapping_add(432)).wrapping_add((((l_i_7) as u32)).wrapping_mul(16)), (fp.wrapping_add(496)).wrapping_add((((l_i_7) as u32)).wrapping_mul(16)));
                                        F::ClipVtx_Set(c, (fp.wrapping_add(576)).wrapping_add((((1i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(128)).wrapping_add(((((l_i_7).wrapping_add(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(432)).wrapping_add(((((l_i_7).wrapping_add(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(496)).wrapping_add(((((l_i_7).wrapping_add(1i32)) as u32)).wrapping_mul(16)));
                                        F::ClipVtx_Set(c, (fp.wrapping_add(576)).wrapping_add((((2i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(128)).wrapping_add(((((l_i_7).wrapping_add(2i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(432)).wrapping_add(((((l_i_7).wrapping_add(2i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(496)).wrapping_add(((((l_i_7).wrapping_add(2i32)) as u32)).wrapping_mul(16)));
                                        { let a1_ = ((c.m.r8((p_prm_3).wrapping_add(4))) as i32); let a6_ = c.m.r64(((p_w_1).wrapping_add(432)).wrapping_add((((3i32) as u32)).wrapping_mul(8))); F::EftGfx_DrawPolyScaledZ(c, fp.wrapping_add(576), a1_, 1i32, 0i32, 0i32, 0i32, a6_, F32(0x3f800000)) };
                                    }
                                    { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
                                }
                            }
                        }
                    }
                }
                l_n_5 = c.m.r32((l_n_5).wrapping_add(64));
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_DrawStrip", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbon_DrawKind1(c: &mut Ctx, mut p_w_1: u32, mut p_arg_2: u32, mut p_prm_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_DrawKind1", &PROBE_N, &[(p_w_1) as u32, (p_arg_2) as u32, (p_prm_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(912) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 912);
    let mut l_link_4: u32 = 0;
    let mut l_node_5: u32 = 0;
    let mut l_first_6: i32 = 0;
    let mut l_idx_7: i32 = 0;
    let mut l_i_8: i32 = 0;
    let mut l_u_9: F32 = F32(0);
    let mut l_u2_10: F32 = F32(0);
    let mut l_du_11: F32 = F32(0);
    let mut l_np_12: u32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 64);
        rt::memset(c, fp.wrapping_add(0), 0, 16);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(16), 0, 16);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(32), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(32).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(32).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(32).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(32).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(48), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(48).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(48).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(48).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(48).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(64), 0, 64);
        rt::memset(c, fp.wrapping_add(64).wrapping_add(0), 0, 16);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(64).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(64).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(64).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(64).wrapping_add(16), 0, 16);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(64).wrapping_add(16).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(16).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(64).wrapping_add(16).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(16).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(64).wrapping_add(32), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(32).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(64).wrapping_add(32).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(64).wrapping_add(32).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(32).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(64).wrapping_add(48), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(48).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(48).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(64).wrapping_add(48).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(48).wrapping_add(12), (v_).0); }
        l_first_6 = 1i32;
        l_idx_7 = 0i32;
        l_u_9 = F32(0x3f800000);
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(544); { let _ = D::call(c, fid::Vec4_Copy, &[((fp.wrapping_add(224)) as u64), ((a1_) as u64)]); } };
        l_du_11 = rt::fdiv(F32(0xbf800000), rt::i2f((c.m.r32((p_w_1).wrapping_add(228)) as i32)));
        l_u2_10 = rt::fadd(l_du_11, l_u_9);
        if ((((!(((((c.m.r32((p_w_1).wrapping_add(208)) as i32)) & (64i32))) != 0)) as i32)) != 0) {
            {
                l_i_8 = 0i32;
                'l1: loop {
                    if !(((((l_i_8) < (4i32)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        { let _ = D::call(c, fid::Vec4_Copy, &[(((fp.wrapping_add(288)).wrapping_add((((l_i_8) as u32)).wrapping_mul(16))) as u64), (((fp).wrapping_add((((l_i_8) as u32)).wrapping_mul(16))) as u64)]); };
                    }
                    { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
                }
            }
        } else {
            {
                l_i_8 = 0i32;
                'l3: loop {
                    if !(((((l_i_8) < (4i32)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        { let _ = D::call(c, fid::Vec4_Copy, &[(((fp.wrapping_add(288)).wrapping_add((((l_i_8) as u32)).wrapping_mul(16))) as u64), (((fp.wrapping_add(64)).wrapping_add((((l_i_8) as u32)).wrapping_mul(16))) as u64)]); };
                    }
                    { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
                }
            }
        }
        { let _ = D::call(c, fid::Vec4_Copy, &[((fp.wrapping_add(352)) as u64), (((p_w_1).wrapping_add(176)) as u64)]); };
        l_link_4 = p_w_1;
        'l5: while (((((c.m.r32(l_link_4)) != (((0i32) as u32))) as i32)) != 0) {
            l_node_5 = c.m.r32(l_link_4);
            if (((((c.m.r32((l_node_5).wrapping_add(64))) != (((0i32) as u32))) as i32)) != 0) {
                l_np_12 = (c.m.r32((l_node_5).wrapping_add(64))).wrapping_add(16);
                { let _ = D::call(c, fid::Vec4_Sub, &[((fp.wrapping_add(240)) as u64), ((l_np_12) as u64), (((l_node_5).wrapping_add(16)) as u64)]); };
                { let _ = D::call(c, fid::Vec4_Sub, &[((fp.wrapping_add(272)) as u64), (((l_node_5).wrapping_add(16)) as u64), ((fp.wrapping_add(224)) as u64)]); };
                { let _ = D::call(c, fid::Vec3_Cross, &[((fp.wrapping_add(240)) as u64), ((fp.wrapping_add(240)) as u64), ((fp.wrapping_add(272)) as u64)]); };
                { let _ = D::call(c, fid::Vec3_Normalize, &[((fp.wrapping_add(240)) as u64), ((fp.wrapping_add(240)) as u64)]); };
                { let a2_ = F32(c.m.r32((p_w_1).wrapping_add(260))); F::Vec4_Scale(c, fp.wrapping_add(384), fp.wrapping_add(240), a2_) };
                if ((l_first_6) != 0) {
                    { let _ = D::call(c, fid::Vec3_Add, &[(((fp.wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as u64), (((l_node_5).wrapping_add(16)) as u64), ((fp.wrapping_add(384)) as u64)]); };
                    l_first_6 = 0i32;
                    { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let _ = D::call(c, fid::Vec3_Sub, &[(((fp.wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as u64), (((l_node_5).wrapping_add(16)) as u64), ((fp.wrapping_add(384)) as u64)]); };
                    { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                } else {
                    if (((rt::flt(F::Vec3_Dot(c, fp.wrapping_add(240), fp.wrapping_add(256)), F32(0x00000000)) as i32)) != 0) {
                        { let _ = D::call(c, fid::Vec3_Scale, &[((fp.wrapping_add(384)) as u64), ((fp.wrapping_add(384)) as u64), ((F32(0xbf800000)).0 as u64)]); };
                        { let _ = D::call(c, fid::Vec3_Scale, &[((fp.wrapping_add(240)) as u64), ((fp.wrapping_add(240)) as u64), ((F32(0xbf800000)).0 as u64)]); };
                    }
                    if (((rt::flt(F::Vec3_Dot(c, fp.wrapping_add(240), fp.wrapping_add(256)), F32(0x3f51eb85)) as i32)) != 0) {
                        { let _ = F::Vec3_Dist(c, (l_node_5).wrapping_add(16), l_np_12); };
                    }
                    { let _ = D::call(c, fid::Vec4_Copy, &[(((fp.wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as u64), (((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as u64)]); };
                    { let _ = D::call(c, fid::Vec4_Copy, &[(((fp.wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as u64), (((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as u64)]); };
                }
                { let _ = D::call(c, fid::Vec3_Add, &[(((fp.wrapping_add(128)).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as u64), ((l_np_12) as u64), ((fp.wrapping_add(384)) as u64)]); };
                { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(128)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                { let _ = D::call(c, fid::Vec3_Sub, &[(((fp.wrapping_add(128)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as u64), ((l_np_12) as u64), ((fp.wrapping_add(384)) as u64)]); };
                { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(128)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                { let _ = D::call(c, fid::Vec4_Copy, &[(((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as u64), (((fp.wrapping_add(128)).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as u64)]); };
                { let _ = D::call(c, fid::Vec4_Copy, &[(((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as u64), (((fp.wrapping_add(128)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as u64)]); };
                { let _ = D::call(c, fid::Vec4_Copy, &[((fp.wrapping_add(256)) as u64), ((fp.wrapping_add(240)) as u64)]); };
                { let _ = F::Vu0Cur_ProjectPoints(c, fp.wrapping_add(416), fp.wrapping_add(128), 4i32); };
                { let v_ = l_u_9; let a_ = (fp.wrapping_add(288)).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                { let v_ = l_u_9; let a_ = (fp.wrapping_add(288)).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                { let v_ = l_u2_10; let a_ = (fp.wrapping_add(288)).wrapping_add((((2i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                { let v_ = l_u2_10; let a_ = (fp.wrapping_add(288)).wrapping_add((((3i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                {
                    l_i_8 = 0i32;
                    'l6: loop {
                        if !(((((l_i_8) < (2i32)) as i32)) != 0) { break 'l6; }
                        'c7: {
                            { let _ = D::call(c, fid::ClipVtx_Set, &[(((fp.wrapping_add(480)).wrapping_add((((0i32) as u32)).wrapping_mul(48))) as u64), (((fp.wrapping_add(128)).wrapping_add((((l_i_8) as u32)).wrapping_mul(16))) as u64), (((fp.wrapping_add(288)).wrapping_add((((l_i_8) as u32)).wrapping_mul(16))) as u64), ((fp.wrapping_add(352)) as u64)]); };
                            { let _ = D::call(c, fid::ClipVtx_Set, &[(((fp.wrapping_add(480)).wrapping_add((((1i32) as u32)).wrapping_mul(48))) as u64), (((fp.wrapping_add(128)).wrapping_add(((((l_i_8).wrapping_add(1i32)) as u32)).wrapping_mul(16))) as u64), (((fp.wrapping_add(288)).wrapping_add(((((l_i_8).wrapping_add(1i32)) as u32)).wrapping_mul(16))) as u64), ((fp.wrapping_add(352)) as u64)]); };
                            { let _ = D::call(c, fid::ClipVtx_Set, &[(((fp.wrapping_add(480)).wrapping_add((((2i32) as u32)).wrapping_mul(48))) as u64), (((fp.wrapping_add(128)).wrapping_add(((((l_i_8).wrapping_add(2i32)) as u32)).wrapping_mul(16))) as u64), (((fp.wrapping_add(288)).wrapping_add(((((l_i_8).wrapping_add(2i32)) as u32)).wrapping_mul(16))) as u64), ((fp.wrapping_add(352)) as u64)]); };
                            { let a1_ = ((c.m.r8((p_prm_3).wrapping_add(4))) as i32); let a6_ = { let a_ = ((p_w_1).wrapping_add(432)).wrapping_add((((c.m.r8((l_node_5).wrapping_add(4))) as u32)).wrapping_mul(8)); c.m.r64(a_) }; { let _ = D::call(c, fid::EftGfx_DrawPolyScaledZ, &[((fp.wrapping_add(480)) as u64), ((a1_) as i64 as u64), ((0i32) as i64 as u64), ((0i32) as i64 as u64), ((0i32) as i64 as u64), ((0i32) as i64 as u64), ((a6_) as u64), ((F32(0x3f800000)).0 as u64)]); } };
                        }
                        { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
                    }
                }
            }
            l_u_9 = rt::fadd(l_du_11, l_u_9);
            { let r_ = l_du_11; let n_ = rt::fadd(l_u2_10, r_); l_u2_10 = n_; };
            if ((((!(((((l_idx_7) < (((c.m.r32((p_w_1).wrapping_add(228)) as i32)).wrapping_sub(2i32))) as i32)) != 0)) as i32)) != 0) {
                l_u2_10 = F32(0x00000000);
            }
            { let o_ = l_idx_7; let n_ = (o_).wrapping_add(1); l_idx_7 = n_; };
            l_link_4 = (l_node_5).wrapping_add(64);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_DrawKind1", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbon_DrawKind2(c: &mut Ctx, mut p_w_1: u32, mut p_arg_2: u32, mut p_prm_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_DrawKind2", &PROBE_N, &[(p_w_1) as u32, (p_arg_2) as u32, (p_prm_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(896) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 896);
    let mut l_link_4: u32 = 0;
    let mut l_node_5: u32 = 0;
    let mut l_next_6: u32 = 0;
    let mut l_first_7: i32 = 0;
    let mut l_sprite_8: i32 = 0;
    let mut l_idx_9: i32 = 0;
    let mut l_i_10: i32 = 0;
    let mut l_size_11: F32 = F32(0);
    let mut l_size2_12: F32 = F32(0);
    let mut l_a_13: F32 = F32(0);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 64);
        rt::memset(c, fp.wrapping_add(0), 0, 16);
        { let v_ = F32(0x3f7ae147); c.m.w32(fp.wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(16), 0, 16);
        { let v_ = F32(0x3f7ae147); c.m.w32(fp.wrapping_add(16).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f7ae147); c.m.w32(fp.wrapping_add(16).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(32), 0, 16);
        { let v_ = F32(0x3ca3d70a); c.m.w32(fp.wrapping_add(32).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(32).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(32).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(32).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(48), 0, 16);
        { let v_ = F32(0x3ca3d70a); c.m.w32(fp.wrapping_add(48).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f7ae147); c.m.w32(fp.wrapping_add(48).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(48).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(48).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(64), 0, 64);
        rt::memset(c, fp.wrapping_add(64).wrapping_add(0), 0, 16);
        { let v_ = F32(0x3f7ae147); c.m.w32(fp.wrapping_add(64).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f7ae147); c.m.w32(fp.wrapping_add(64).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(64).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(64).wrapping_add(16), 0, 16);
        { let v_ = F32(0x3f7ae147); c.m.w32(fp.wrapping_add(64).wrapping_add(16).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(16).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(64).wrapping_add(16).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(16).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(64).wrapping_add(32), 0, 16);
        { let v_ = F32(0x3ca3d70a); c.m.w32(fp.wrapping_add(64).wrapping_add(32).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f7ae147); c.m.w32(fp.wrapping_add(64).wrapping_add(32).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(64).wrapping_add(32).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(32).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(64).wrapping_add(48), 0, 16);
        { let v_ = F32(0x3ca3d70a); c.m.w32(fp.wrapping_add(64).wrapping_add(48).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(48).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(64).wrapping_add(48).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(48).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(448), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(448).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(448).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(448).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(448).wrapping_add(12), (v_).0); }
        l_first_7 = 1i32;
        l_idx_9 = 0i32;
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(544); { let _ = D::call(c, fid::Vec4_Copy, &[((fp.wrapping_add(224)) as u64), ((a1_) as u64)]); } };
        if ((((!(((((c.m.r32((p_w_1).wrapping_add(208)) as i32)) & (64i32))) != 0)) as i32)) != 0) {
            {
                l_i_10 = 0i32;
                'l1: loop {
                    if !(((((l_i_10) < (4i32)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        { let _ = D::call(c, fid::Vec4_Copy, &[(((fp.wrapping_add(288)).wrapping_add((((l_i_10) as u32)).wrapping_mul(16))) as u64), (((fp).wrapping_add((((l_i_10) as u32)).wrapping_mul(16))) as u64)]); };
                    }
                    { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; };
                }
            }
        } else {
            {
                l_i_10 = 0i32;
                'l3: loop {
                    if !(((((l_i_10) < (4i32)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        { let _ = D::call(c, fid::Vec4_Copy, &[(((fp.wrapping_add(288)).wrapping_add((((l_i_10) as u32)).wrapping_mul(16))) as u64), (((fp.wrapping_add(64)).wrapping_add((((l_i_10) as u32)).wrapping_mul(16))) as u64)]); };
                    }
                    { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; };
                }
            }
        }
        l_size_11 = rt::fmul(F32(c.m.r32((p_w_1).wrapping_add(260))), F32(0x41180000));
        l_link_4 = p_w_1;
        'l5: while (((((c.m.r32(l_link_4)) != (((0i32) as u32))) as i32)) != 0) {
            l_node_5 = c.m.r32(l_link_4);
            l_next_6 = c.m.r32((l_node_5).wrapping_add(64));
            if (((((l_next_6) != (((0i32) as u32))) as i32)) != 0) {
                l_sprite_8 = 0i32;
                { let _ = D::call(c, fid::Vec4_Sub, &[((fp.wrapping_add(240)) as u64), (((l_next_6).wrapping_add(16)) as u64), (((l_node_5).wrapping_add(16)) as u64)]); };
                { let _ = D::call(c, fid::Vec4_Sub, &[((fp.wrapping_add(272)) as u64), (((l_node_5).wrapping_add(16)) as u64), ((fp.wrapping_add(224)) as u64)]); };
                { let _ = D::call(c, fid::Vec3_Cross, &[((fp.wrapping_add(240)) as u64), ((fp.wrapping_add(240)) as u64), ((fp.wrapping_add(272)) as u64)]); };
                { let _ = D::call(c, fid::Vec3_Normalize, &[((fp.wrapping_add(240)) as u64), ((fp.wrapping_add(240)) as u64)]); };
                { let a2_ = F32(c.m.r32((p_w_1).wrapping_add(260))); { let _ = D::call(c, fid::Vec3_Scale, &[((fp.wrapping_add(448)) as u64), ((fp.wrapping_add(240)) as u64), ((a2_).0 as u64)]); } };
                if ((l_first_7) != 0) {
                    { let _ = D::call(c, fid::Vec3_Sub, &[(((fp.wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as u64), (((l_node_5).wrapping_add(16)) as u64), ((fp.wrapping_add(448)) as u64)]); };
                    { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let _ = D::call(c, fid::Vec3_Add, &[(((fp.wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as u64), (((l_node_5).wrapping_add(16)) as u64), ((fp.wrapping_add(448)) as u64)]); };
                    { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                } else {
                    if (((rt::flt(F::Vec3_Dot(c, fp.wrapping_add(240), fp.wrapping_add(256)), F32(0x00000000)) as i32)) != 0) {
                        { let _ = D::call(c, fid::Vec3_Scale, &[((fp.wrapping_add(448)) as u64), ((fp.wrapping_add(448)) as u64), ((F32(0xbf800000)).0 as u64)]); };
                        { let _ = D::call(c, fid::Vec3_Scale, &[((fp.wrapping_add(240)) as u64), ((fp.wrapping_add(240)) as u64), ((F32(0xbf800000)).0 as u64)]); };
                    }
                    if (((((((rt::flt(F::Vec3_Dot(c, fp.wrapping_add(240), fp.wrapping_add(256)), F32(0x3f51eb85)) as i32)) != 0) && (((rt::fgt(F::Vec3_Dist(c, (l_node_5).wrapping_add(16), (l_next_6).wrapping_add(16)), F32(0x3f4ccccc)) as i32)) != 0)) as i32)) != 0) {
                        l_sprite_8 = 1i32;
                        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32((l_node_5).wrapping_add(16))), F32(c.m.r32((l_next_6).wrapping_add(16)))), F32(0x3f000000)); let a_ = fp.wrapping_add(432); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_node_5).wrapping_add(16)).wrapping_add(4))), F32(c.m.r32(((l_next_6).wrapping_add(16)).wrapping_add(4)))), F32(0x3f000000)); let a_ = (fp.wrapping_add(432)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_node_5).wrapping_add(16)).wrapping_add(8))), F32(c.m.r32(((l_next_6).wrapping_add(16)).wrapping_add(8)))), F32(0x3f000000)); let a_ = (fp.wrapping_add(432)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(432)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        l_a_13 = rt::i2f(rt::div_i32(l_idx_9, (c.m.r32((p_w_1).wrapping_add(228)) as i32)));
                        { let _ = D::call(c, fid::Vec4_Copy, &[((fp.wrapping_add(416)) as u64), (((l_node_5).wrapping_add(32)) as u64)]); };
                        l_a_13 = rt::fsub(F32(0x3f800000), l_a_13);
                        l_size2_12 = rt::fmul(l_size_11, l_a_13);
                        { let r_ = l_a_13; let a_ = (fp.wrapping_add(416)).wrapping_add(12); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        { let a9_ = ((c.m.r8((p_prm_3).wrapping_add(4))) as i32); let a11_ = c.m.r64(((p_w_1).wrapping_add(432)).wrapping_add((((3i32) as u32)).wrapping_mul(8))); F::EftGfx_DrawSprite(c, (l_node_5).wrapping_add(16), fp.wrapping_add(416), l_size2_12, l_size2_12, F32(0x00000000), F32(0x00000000), F32(0x3f800000), F32(0x3f800000), F32(0x00000000), a9_, 0i32, a11_) };
                        { let a9_ = ((c.m.r8((p_prm_3).wrapping_add(4))) as i32); let a11_ = c.m.r64(((p_w_1).wrapping_add(432)).wrapping_add((((3i32) as u32)).wrapping_mul(8))); F::EftGfx_DrawSprite(c, fp.wrapping_add(432), fp.wrapping_add(416), l_size2_12, l_size2_12, F32(0x00000000), F32(0x00000000), F32(0x3f800000), F32(0x3f800000), F32(0x00000000), a9_, 0i32, a11_) };
                        { let a9_ = ((c.m.r8((p_prm_3).wrapping_add(4))) as i32); let a11_ = c.m.r64(((p_w_1).wrapping_add(432)).wrapping_add((((3i32) as u32)).wrapping_mul(8))); F::EftGfx_DrawSprite(c, (l_next_6).wrapping_add(16), fp.wrapping_add(416), l_size2_12, l_size2_12, F32(0x00000000), F32(0x00000000), F32(0x3f800000), F32(0x3f800000), F32(0x00000000), a9_, 0i32, a11_) };
                    }
                    { let _ = D::call(c, fid::Vec4_Copy, &[(((fp.wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as u64), ((fp.wrapping_add(192)) as u64)]); };
                    { let _ = D::call(c, fid::Vec4_Copy, &[(((fp.wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as u64), ((fp.wrapping_add(208)) as u64)]); };
                }
                { let _ = D::call(c, fid::Vec3_Sub, &[(((fp.wrapping_add(128)).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as u64), (((l_next_6).wrapping_add(16)) as u64), ((fp.wrapping_add(448)) as u64)]); };
                l_first_7 = 0i32;
                { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(128)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                { let _ = D::call(c, fid::Vec3_Add, &[(((fp.wrapping_add(128)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as u64), (((l_next_6).wrapping_add(16)) as u64), ((fp.wrapping_add(448)) as u64)]); };
                { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(128)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                { let _ = D::call(c, fid::Vec4_Copy, &[((fp.wrapping_add(192)) as u64), (((fp.wrapping_add(128)).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as u64)]); };
                { let _ = D::call(c, fid::Vec4_Copy, &[((fp.wrapping_add(208)) as u64), (((fp.wrapping_add(128)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as u64)]); };
                { let _ = D::call(c, fid::Vec4_Copy, &[((fp.wrapping_add(256)) as u64), ((fp.wrapping_add(240)) as u64)]); };
                if ((((!((l_sprite_8) != 0)) as i32)) != 0) {
                    { let _ = D::call(c, fid::Vec4_Copy, &[(((fp.wrapping_add(352)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as u64), (((l_node_5).wrapping_add(32)) as u64)]); };
                    { let _ = D::call(c, fid::Vec4_Copy, &[(((fp.wrapping_add(352)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as u64), (((l_node_5).wrapping_add(32)) as u64)]); };
                    { let _ = D::call(c, fid::Vec4_Copy, &[(((fp.wrapping_add(352)).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as u64), (((l_next_6).wrapping_add(32)) as u64)]); };
                    { let _ = D::call(c, fid::Vec4_Copy, &[(((fp.wrapping_add(352)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as u64), (((l_next_6).wrapping_add(32)) as u64)]); };
                    {
                        l_i_10 = 0i32;
                        'l6: loop {
                            if !(((((l_i_10) < (2i32)) as i32)) != 0) { break 'l6; }
                            'c7: {
                                { let _ = D::call(c, fid::ClipVtx_Set, &[(((fp.wrapping_add(464)).wrapping_add((((0i32) as u32)).wrapping_mul(48))) as u64), (((fp.wrapping_add(128)).wrapping_add((((l_i_10) as u32)).wrapping_mul(16))) as u64), (((fp.wrapping_add(288)).wrapping_add((((l_i_10) as u32)).wrapping_mul(16))) as u64), (((fp.wrapping_add(352)).wrapping_add((((l_i_10) as u32)).wrapping_mul(16))) as u64)]); };
                                { let _ = D::call(c, fid::ClipVtx_Set, &[(((fp.wrapping_add(464)).wrapping_add((((1i32) as u32)).wrapping_mul(48))) as u64), (((fp.wrapping_add(128)).wrapping_add(((((l_i_10).wrapping_add(1i32)) as u32)).wrapping_mul(16))) as u64), (((fp.wrapping_add(288)).wrapping_add(((((l_i_10).wrapping_add(1i32)) as u32)).wrapping_mul(16))) as u64), (((fp.wrapping_add(352)).wrapping_add(((((l_i_10).wrapping_add(1i32)) as u32)).wrapping_mul(16))) as u64)]); };
                                { let _ = D::call(c, fid::ClipVtx_Set, &[(((fp.wrapping_add(464)).wrapping_add((((2i32) as u32)).wrapping_mul(48))) as u64), (((fp.wrapping_add(128)).wrapping_add(((((l_i_10).wrapping_add(2i32)) as u32)).wrapping_mul(16))) as u64), (((fp.wrapping_add(288)).wrapping_add(((((l_i_10).wrapping_add(2i32)) as u32)).wrapping_mul(16))) as u64), (((fp.wrapping_add(352)).wrapping_add(((((l_i_10).wrapping_add(2i32)) as u32)).wrapping_mul(16))) as u64)]); };
                                { let a1_ = ((c.m.r8((p_prm_3).wrapping_add(4))) as i32); let a6_ = { let a_ = ((p_w_1).wrapping_add(432)).wrapping_add((((c.m.r8((l_node_5).wrapping_add(4))) as u32)).wrapping_mul(8)); c.m.r64(a_) }; { let _ = D::call(c, fid::EftGfx_DrawPolyScaledZ, &[((fp.wrapping_add(464)) as u64), ((a1_) as i64 as u64), ((0i32) as i64 as u64), ((0i32) as i64 as u64), ((0i32) as i64 as u64), ((0i32) as i64 as u64), ((a6_) as u64), ((F32(0x3f800000)).0 as u64)]); } };
                            }
                            { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; };
                        }
                    }
                }
            }
            { let o_ = l_idx_9; let n_ = (o_).wrapping_add(1); l_idx_9 = n_; };
            l_link_4 = (l_node_5).wrapping_add(64);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_DrawKind2", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbon_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let mut l_prm_4: u32 = 0;
    let mut l_cur_5: u32 = 0;
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    l_prm_4 = c.m.r32((p_arg_2).wrapping_add(56));
    { let _ = F::memset(c, l_w_3, 0i32, 480u32); };
    l_cur_5 = (l_w_3).wrapping_add(80);
    { let s_ = p_arg_2; let d_ = (l_w_3).wrapping_add(16); rt::memcpy(c, d_, s_, 64); };
    { let r_ = 1i32; let a_ = (l_w_3).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    { let v_ = ((c.m.r8((l_prm_4).wrapping_add(5))) as i32); let a_ = (l_w_3).wrapping_add(212); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32((p_arg_2).wrapping_add(40)) as i32); let a_ = (l_w_3).wrapping_add(216); c.m.w32(a_, (v_) as u32); };
    if (((rt::fle(F32(c.m.r32((p_arg_2).wrapping_add(44))), F32(0x00000000)) as i32)) != 0) {
        { let r_ = 16i32; let a_ = (l_w_3).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        { let v_ = rt::fmul(F32(c.m.r32((p_arg_2).wrapping_add(44))), F32(0x41f00000)); let a_ = (l_w_3).wrapping_add(232); c.m.w32(a_, (v_).0); };
    }
    if (((((c.m.r32(l_prm_4) as i32)) & (1i32))) != 0) {
        { let v_ = rt::fmul(F32(c.m.r32((l_prm_4).wrapping_add(16))), F32(0x41f00000)); let a_ = (l_w_3).wrapping_add(252); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32((l_w_3).wrapping_add(252))), F32(c.m.r32((l_prm_4).wrapping_add(20)))); let a_ = (l_w_3).wrapping_add(256); c.m.w32(a_, (v_).0); };
        { let _ = D::call(c, fid::EftRibbon_LoadKey, &[((l_cur_5) as u64), (((l_w_3).wrapping_add(16)) as u64), ((0i32) as i64 as u64)]); };
    } else {
        { let _ = D::call(c, fid::EftRibbon_LoadKey, &[((l_cur_5) as u64), (((l_w_3).wrapping_add(16)) as u64), ((2i32) as i64 as u64)]); };
    }
    { let _ = D::call(c, fid::EftRibbon_InitNodes, &[((l_w_3) as u64), ((p_arg_2) as u64)]); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbon_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    { let v_ = 0i32; let a_ = (l_w_2).wrapping_add(208); c.m.w32(a_, (v_) as u32); };
    { let _ = D::call(c, fid::EftRibbon_FreeNodes, &[((l_w_2) as u64)]); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbon_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_2: u32 = 0;
    let mut l_arg_3: u32 = 0;
    let mut l_prm_4: u32 = 0;
    let mut l_cur_5: u32 = 0;
    let mut l_n_6: i32 = 0;
    let mut l_t_7: F32 = F32(0);
    let mut l_k_8: F32 = F32(0);
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_arg_3 = (l_w_2).wrapping_add(16);
        l_prm_4 = c.m.r32((l_arg_3).wrapping_add(56));
        if ((((!(({ let a0_ = (c.m.r32((l_arg_3).wrapping_add(36)) as i32); let a1_ = (c.m.r32((l_arg_3).wrapping_add(32)) as i32); F::BtlScene_IsEffectStopped(c, a0_, a1_) }) != 0)) as i32)) != 0) {
            if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(236))), F32(0x00000000)) as i32)) != 0) {
                l_cur_5 = (l_w_2).wrapping_add(80);
                { let v_ = F32(c.m.r32((l_cur_5).wrapping_add(44))); let a_ = (l_w_2).wrapping_add(260); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32((l_cur_5).wrapping_add(60))); let a_ = (l_w_2).wrapping_add(288); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32((l_cur_5).wrapping_add(64))); let a_ = (l_w_2).wrapping_add(292); c.m.w32(a_, (v_).0); };
                if (((((((((c.m.r32(l_prm_4) as i32)) & (8i32))) != 0) && (((rt::fgt(F32(c.m.r32((l_cur_5).wrapping_add(56))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                    l_t_7 = rt::fdiv(F32(c.m.r32((l_w_2).wrapping_add(264))), F32(c.m.r32((l_cur_5).wrapping_add(56))));
                    l_k_8 = rt::fadd(F32(c.m.r32((l_cur_5).wrapping_add(48))), rt::fmul(F32(c.m.r32((l_cur_5).wrapping_add(52))), l_t_7));
                    { let r_ = l_k_8; let a_ = (l_w_2).wrapping_add(260); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let r_ = l_k_8; let a_ = (l_w_2).wrapping_add(288); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if ((((!(((((c.m.r32((l_w_2).wrapping_add(208)) as i32)) & (512i32))) != 0)) as i32)) != 0) {
                        { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(264); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        if (((rt::fle(F32(c.m.r32((l_cur_5).wrapping_add(56))), F32(c.m.r32((l_w_2).wrapping_add(264)))) as i32)) != 0) {
                            { let v_ = F32(c.m.r32((l_cur_5).wrapping_add(56))); let a_ = (l_w_2).wrapping_add(264); c.m.w32(a_, (v_).0); };
                            { let r_ = 512i32; let a_ = (l_w_2).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                    } else {
                        { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(264); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(264))), F32(0x00000000)) as i32)) != 0) {
                            { let v_ = F32(0x00000000); let a_ = (l_w_2).wrapping_add(264); c.m.w32(a_, (v_).0); };
                            { let r_ = (!(512i32)); let a_ = (l_w_2).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                    }
                }
                { let r_ = F32(c.m.r32((l_arg_3).wrapping_add(48))); let a_ = (l_w_2).wrapping_add(260); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = F32(c.m.r32((l_arg_3).wrapping_add(48))); let a_ = (l_w_2).wrapping_add(288); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if ((((!(((((c.m.r32((l_w_2).wrapping_add(208)) as i32)) & (128i32))) != 0)) as i32)) != 0) {
                    l_t_7 = rt::fdiv(F32(c.m.r32((l_w_2).wrapping_add(272))), F32(c.m.r32((l_w_2).wrapping_add(280))));
                    { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(272); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(280))), F32(c.m.r32((l_w_2).wrapping_add(272)))) as i32)) != 0) {
                        { let r_ = 128i32; let a_ = (l_w_2).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    } else {
                        { let r_ = l_t_7; let a_ = (l_w_2).wrapping_add(260); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        { let r_ = l_t_7; let a_ = (l_w_2).wrapping_add(288); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    }
                }
                if (((((c.m.r32((l_w_2).wrapping_add(208)) as i32)) & (2i32))) != 0) {
                    if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(240))), F32(0x00000000)) as i32)) != 0) {
                        if ((((!(((((c.m.r32((l_w_2).wrapping_add(208)) as i32)) & (256i32))) != 0)) as i32)) != 0) {
                            l_t_7 = rt::fdiv(F32(c.m.r32((l_w_2).wrapping_add(276))), F32(c.m.r32((l_w_2).wrapping_add(284))));
                            { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(276); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            { let r_ = l_t_7; let a_ = (l_w_2).wrapping_add(260); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            { let r_ = l_t_7; let a_ = (l_w_2).wrapping_add(288); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(276))), F32(0x00000000)) as i32)) != 0) {
                                { let r_ = 256i32; let a_ = (l_w_2).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                { let v_ = F32(0x00000000); let a_ = (l_w_2).wrapping_add(260); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x00000000); let a_ = (l_w_2).wrapping_add(288); c.m.w32(a_, (v_).0); };
                            }
                        } else {
                            { let v_ = F32(0x00000000); let a_ = (l_w_2).wrapping_add(260); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(0x00000000); let a_ = (l_w_2).wrapping_add(288); c.m.w32(a_, (v_).0); };
                        }
                    }
                }
                { let _ = D::call(c, fid::Vec4_Copy, &[(((l_w_2).wrapping_add(176)) as u64), ((l_cur_5) as u64)]); };
                { let _ = D::call(c, fid::Vec4_Copy, &[(((l_w_2).wrapping_add(192)) as u64), ((l_cur_5) as u64)]); };
                if (((((((((c.m.r32(l_prm_4) as i32)) & (16i32))) != 0) && (((rt::fgt(F32(c.m.r32((l_cur_5).wrapping_add(40))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                    l_t_7 = rt::fdiv(F32(c.m.r32((l_w_2).wrapping_add(268))), F32(c.m.r32((l_cur_5).wrapping_add(40))));
                    l_k_8 = rt::fadd(F32(c.m.r32(((l_cur_5).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_cur_5).wrapping_add(28)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_7));
                    { let r_ = l_k_8; let a_ = (l_w_2).wrapping_add(176); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let r_ = l_k_8; let a_ = (l_w_2).wrapping_add(192); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    l_k_8 = rt::fadd(F32(c.m.r32(((l_cur_5).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_cur_5).wrapping_add(28)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_7));
                    { let r_ = l_k_8; let a_ = ((l_w_2).wrapping_add(176)).wrapping_add(4); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let r_ = l_k_8; let a_ = ((l_w_2).wrapping_add(192)).wrapping_add(4); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    l_k_8 = rt::fadd(F32(c.m.r32(((l_cur_5).wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_cur_5).wrapping_add(28)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_t_7));
                    { let r_ = l_k_8; let a_ = ((l_w_2).wrapping_add(176)).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let r_ = l_k_8; let a_ = ((l_w_2).wrapping_add(192)).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if ((((!(((((c.m.r32((l_w_2).wrapping_add(208)) as i32)) & (1024i32))) != 0)) as i32)) != 0) {
                        { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(268); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        if (((rt::fle(F32(c.m.r32((l_cur_5).wrapping_add(40))), F32(c.m.r32((l_w_2).wrapping_add(268)))) as i32)) != 0) {
                            { let v_ = F32(c.m.r32((l_cur_5).wrapping_add(40))); let a_ = (l_w_2).wrapping_add(268); c.m.w32(a_, (v_).0); };
                            { let r_ = 1024i32; let a_ = (l_w_2).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                    } else {
                        { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(268); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(268))), F32(0x00000000)) as i32)) != 0) {
                            { let v_ = F32(0x00000000); let a_ = (l_w_2).wrapping_add(268); c.m.w32(a_, (v_).0); };
                            { let r_ = (!(1024i32)); let a_ = (l_w_2).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                    }
                }
                if (((((c.m.r32((l_w_2).wrapping_add(208)) as i32)) & (32i32))) != 0) {
                    { let r_ = F32(c.m.r32((l_w_2).wrapping_add(292))); let a_ = (l_w_2).wrapping_add(296); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if (((rt::fle(F32(0x3f800000), F32(c.m.r32((l_w_2).wrapping_add(296)))) as i32)) != 0) {
                        { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(296); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    } else {
                        if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(296))), F32(0xbf800000)) as i32)) != 0) {
                            { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(296); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        }
                    }
                }
                if (((((((rt::fle(F32(c.m.r32(((l_w_2).wrapping_add(176)).wrapping_add(12))), F32(0x00000000)) as i32)) != 0) || (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(260))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                    { let r_ = (!(8i32)); let a_ = (l_w_2).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                } else {
                    { let r_ = 8i32; let a_ = (l_w_2).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
                if (((((((c.m.r8((l_prm_4).wrapping_add(5))) as i32)) == (0i32)) as i32)) != 0) {
                    { let _ = D::call(c, fid::Vec4_Sub, &[((fp) as u64), (((l_w_2).wrapping_add(160)) as u64), ((l_arg_3) as u64)]); };
                    { let v_ = rt::f2i(rt::fadd(rt::fdiv({ let a0_ = F::Vec3_Dot(c, fp, fp); F::Port_sqrtf(c, a0_) }, F32(0x42700000)), F32(0x40800000))); let a_ = (l_w_2).wrapping_add(224); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let a_ = (l_w_2).wrapping_add(224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                }
                if ((((((c.m.r32((l_w_2).wrapping_add(224)) as i32)) > ((c.m.r32((l_w_2).wrapping_add(220)) as i32))) as i32)) != 0) {
                    { let v_ = (c.m.r32((l_w_2).wrapping_add(220)) as i32); let a_ = (l_w_2).wrapping_add(224); c.m.w32(a_, (v_) as u32); };
                }
                if ((((((c.m.r32((l_w_2).wrapping_add(228)) as i32)) < ((c.m.r32((l_w_2).wrapping_add(224)) as i32))) as i32)) != 0) {
                    {
                        l_n_6 = ((c.m.r32((l_w_2).wrapping_add(224)) as i32)).wrapping_sub((c.m.r32((l_w_2).wrapping_add(228)) as i32));
                        'l1: loop {
                            if !(((((l_n_6) != (0i32)) as i32)) != 0) { break 'l1; }
                            'c2: {
                                if ((((D::call(c, fid::EftRibbon_AddNode, &[((l_w_2) as u64), ((l_arg_3) as u64)])) as i32)) != 0) {
                                    { let a_ = (l_w_2).wrapping_add(228); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                } else {
                                    break 'l1;
                                }
                            }
                            { let o_ = l_n_6; let n_ = (o_).wrapping_sub(1); l_n_6 = n_; };
                        }
                    }
                }
                if (((((((c.m.r8((l_prm_4).wrapping_add(5))) as i32)) == (0i32)) as i32)) != 0) {
                    { let _ = D::call(c, fid::EftRibbon_PlaceStrip, &[((l_prm_4) as u64), ((l_w_2) as u64), ((l_arg_3) as u64)]); };
                } else {
                    if (((((((c.m.r8((l_prm_4).wrapping_add(5))) as i32)) == (1i32)) as i32)) != 0) {
                        { let _ = D::call(c, fid::EftRibbon_PlaceTrail1, &[((l_prm_4) as u64), ((l_w_2) as u64), ((l_arg_3) as u64)]); };
                    } else {
                        { let _ = D::call(c, fid::EftRibbon_PlaceTrail2, &[((l_prm_4) as u64), ((l_w_2) as u64), ((l_arg_3) as u64)]); };
                    }
                }
                if (((((((((c.m.r32(l_prm_4) as i32)) & (1i32))) != 0) && (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(248))), F32(c.m.r32((l_w_2).wrapping_add(252)))) as i32)) != 0)) as i32)) != 0) {
                    { let _ = D::call(c, fid::EftRibbon_UpdateKeys, &[((l_w_2) as u64)]); };
                    { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(248); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(252))), F32(c.m.r32((l_w_2).wrapping_add(248)))) as i32)) != 0) {
                        { let _ = D::call(c, fid::EftRibbon_LoadKey, &[((l_cur_5) as u64), ((l_arg_3) as u64), ((2i32) as i64 as u64)]); };
                    }
                }
                if ((((!(((((c.m.r32((l_w_2).wrapping_add(208)) as i32)) & (16i32))) != 0)) as i32)) != 0) {
                    { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(232); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(232))), F32(0x00000000)) as i32)) != 0) {
                        { let r_ = 2i32; let a_ = (l_w_2).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        if (((rt::fgt(F32(c.m.r32((l_w_2).wrapping_add(244))), F32(0x00000000)) as i32)) != 0) {
                            { let r_ = 4096i32; let a_ = (l_w_2).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                    }
                }
            } else {
                { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(236); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            }
            if (((((c.m.r32((l_w_2).wrapping_add(208)) as i32)) & (2i32))) != 0) {
                if (((rt::fgt(F32(c.m.r32((l_w_2).wrapping_add(240))), F32(0x00000000)) as i32)) != 0) {
                    { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(240); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                }
                if (((((((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(240))), F32(0x00000000)) as i32)) != 0) && (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(260))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                    { let r_ = 4i32; let a_ = (l_w_2).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
        }
        if (((((c.m.r32((l_w_2).wrapping_add(208)) as i32)) & (4i32))) != 0) {
            F::BtlTask_SetDead(c, p_task_1);
        } else {
            { let _ = D::call(c, fid::EftRibbon_UpdateTex, &[((l_prm_4) as u64), ((l_w_2) as u64), ((l_arg_3) as u64)]); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbon_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbon_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbon_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_arg_3: u32 = 0;
    let mut l_prm_4: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_arg_3 = (l_w_2).wrapping_add(16);
    l_prm_4 = c.m.r32((l_arg_3).wrapping_add(56));
    if (({ let a0_ = (c.m.r32((l_arg_3).wrapping_add(36)) as i32); let a1_ = (c.m.r32((l_arg_3).wrapping_add(32)) as i32); F::BtlScene_IsEffectHidden(c, a0_, a1_) }) != 0) {
        break 'f;
    }
    F::Vu0Cur_Push(c);
    { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
    if ((((((c.m.r32((l_w_2).wrapping_add(212)) as i32)) == (0i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::EftRibbon_DrawStrip, &[((l_w_2) as u64), ((l_arg_3) as u64), ((l_prm_4) as u64)]); };
    } else {
        if ((((((c.m.r32((l_w_2).wrapping_add(212)) as i32)) == (1i32)) as i32)) != 0) {
            F::EftRibbon_DrawKind1(c, l_w_2, l_arg_3, l_prm_4);
        } else {
            if ((((((c.m.r32((l_w_2).wrapping_add(212)) as i32)) == (2i32)) as i32)) != 0) {
                F::EftRibbon_DrawKind2(c, l_w_2, l_arg_3, l_prm_4);
            }
        }
    }
    F::Vu0Cur_Pop(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbonMgr_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbonMgr_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((296u32) as i32)) }; let a_ = sym::gEftRibbonMgr; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gEftRibbonMgr); F::memset(c, a0_, 0i32, 296u32) }; };
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, 40000i32) }; let a_ = c.m.r32(sym::gEftRibbonMgr); c.m.w32(a_, v_); };
    { let _ = { let a0_ = { let a_ = c.m.r32(sym::gEftRibbonMgr); c.m.r32(a_) }; F::memset(c, a0_, 0i32, ((40000i32) as u32)) }; };
    { let v_ = F::BtlTask_CreateChildList(c, p_task_1, ((((40i32) as i64 as u64)) as u32), ((((((480u32) as i32)) as i64 as u64)) as u32)); let a_ = sym::gEftRibbonTasks; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRibbonMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbonMgr_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbonMgr_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = { let a_ = c.m.r32(sym::gEftRibbonMgr); c.m.r32(a_) }; F::BtlPool_Free(c, a0_, a1_) };
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftRibbonMgr); F::BtlPool_Free(c, a0_, a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftRibbonMgr; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRibbonMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbonMgr_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbonMgr_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRibbonMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbonMgr_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbonMgr_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRibbonMgr_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbon_Create(c: &mut Ctx, mut p_arg_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_Create", &PROBE_N, &[(p_arg_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let r_ = 'f: {
        { let s_ = p_arg_1; rt::memcpy(c, fp, s_, 64); }
        break 'f { let a0_ = c.m.r32(sym::gEftRibbonTasks); F::BtlTaskList_AddTail(c, a0_, sym::gEftRibbonClass, fp) };
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_Create", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbon_Stop(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_Stop", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(208)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let r_ = 2i32; let a_ = (l_w_2).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_Stop", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbon_Kill(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_Kill", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(208)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let r_ = 4i32; let a_ = (l_w_2).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_Kill", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbon_StopAndKill(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_StopAndKill", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(208)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let r_ = ((2i32) | (4i32)); let a_ = (l_w_2).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_StopAndKill", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbon_IsAlive(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_IsAlive", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftRibbon_Update)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((({ let a_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(208); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_IsAlive", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbon_SetPos(c: &mut Ctx, mut p_task_1: u32, mut p_pos_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_SetPos", &PROBE_N, &[(p_task_1) as u32, (p_pos_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftRibbon_Update)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(208)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let _ = D::call(c, fid::Vec4_Copy, &[(((l_w_3).wrapping_add(16)) as u64), ((p_pos_2) as u64)]); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_SetPos", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbon_SetEnd(c: &mut Ctx, mut p_task_1: u32, mut p_pos_2: u32, mut p_place_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_SetEnd", &PROBE_N, &[(p_task_1) as u32, (p_pos_2) as u32, (p_place_3) as u32], false); }
    let mut l_w_4: u32 = 0;
    let mut l_arg_5: u32 = 0;
    let mut l_prm_6: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftRibbon_Update)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_4 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_4).wrapping_add(208)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let _ = D::call(c, fid::Vec4_Copy, &[(((l_w_4).wrapping_add(160)) as u64), ((p_pos_2) as u64)]); };
    if ((p_place_3) != 0) {
        l_arg_5 = (l_w_4).wrapping_add(16);
        l_prm_6 = c.m.r32((l_arg_5).wrapping_add(56));
        if (((((((c.m.r8((l_prm_6).wrapping_add(5))) as i32)) == (0i32)) as i32)) != 0) {
            { let _ = D::call(c, fid::EftRibbon_PlaceStrip, &[((l_prm_6) as u64), ((l_w_4) as u64), ((l_arg_5) as u64)]); };
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_SetEnd", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbon_SetSize(c: &mut Ctx, mut p_task_1: u32, mut p_size_2: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_SetSize", &PROBE_N, &[(p_task_1) as u32, (p_size_2).0], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftRibbon_Update)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(208)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let v_ = p_size_2; let a_ = ((l_w_3).wrapping_add(16)).wrapping_add(48); c.m.w32(a_, (v_).0); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_SetSize", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbon_SetMaxNodes(c: &mut Ctx, mut p_task_1: u32, mut p_count_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_SetMaxNodes", &PROBE_N, &[(p_task_1) as u32, (p_count_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftRibbon_Update)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((p_count_2) < (3i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(208)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let v_ = p_count_2; let a_ = (l_w_3).wrapping_add(220); c.m.w32(a_, (v_) as u32); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_SetMaxNodes", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbon_SetDelay(c: &mut Ctx, mut p_task_1: u32, mut p_frames_2: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_SetDelay", &PROBE_N, &[(p_task_1) as u32, (p_frames_2).0], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftRibbon_Update)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(208)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let v_ = p_frames_2; let a_ = (l_w_3).wrapping_add(236); c.m.w32(a_, (v_).0); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_SetDelay", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbon_SetFadeDelay(c: &mut Ctx, mut p_task_1: u32, mut p_frames_2: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_SetFadeDelay", &PROBE_N, &[(p_task_1) as u32, (p_frames_2).0], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftRibbon_Update)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(208)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let v_ = p_frames_2; let a_ = (l_w_3).wrapping_add(240); c.m.w32(a_, (v_).0); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_SetFadeDelay", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbon_SetFadeTime(c: &mut Ctx, mut p_task_1: u32, mut p_frames_2: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_SetFadeTime", &PROBE_N, &[(p_task_1) as u32, (p_frames_2).0], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftRibbon_Update)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(208)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let v_ = p_frames_2; let a_ = (l_w_3).wrapping_add(244); c.m.w32(a_, (v_).0); };
    if (((rt::fgt(p_frames_2, F32(0x00000000)) as i32)) != 0) {
        { let v_ = p_frames_2; let a_ = (l_w_3).wrapping_add(284); c.m.w32(a_, (v_).0); };
        if (((rt::fle(p_frames_2, F32(0x00000000)) as i32)) != 0) {
            { let r_ = 256i32; let a_ = (l_w_3).wrapping_add(208); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        } else {
            { let v_ = p_frames_2; let a_ = (l_w_3).wrapping_add(276); c.m.w32(a_, (v_).0); };
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_SetFadeTime", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbon_SetTexFrame(c: &mut Ctx, mut p_task_1: u32, mut p_slot_2: i32, mut p_uv_3: u32, mut p_a_4: i32, mut p_b_5: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_SetTexFrame", &PROBE_N, &[(p_task_1) as u32, (p_slot_2) as u32, (p_uv_3) as u32, (p_a_4) as u32, (p_b_5) as u32], false); }
    let mut l_w_6: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftRibbon_Update)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_6 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_6).wrapping_add(208)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let _ = D::call(c, fid::EftRibbon_SetTexPair, &[((l_w_6) as u64), ((p_slot_2) as i64 as u64), ((p_uv_3) as u64), ((p_a_4) as i64 as u64), ((p_b_5) as i64 as u64)]); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_SetTexFrame", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbon_SetUnkD8(c: &mut Ctx, mut p_task_1: u32, mut p_value_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_SetUnkD8", &PROBE_N, &[(p_task_1) as u32, (p_value_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftRibbon_Update)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(208)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let v_ = p_value_2; let a_ = (l_w_3).wrapping_add(216); c.m.w32(a_, (v_) as u32); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_SetUnkD8", &PROBE_N, &[], true); }
    r_
}

pub fn EftRibbon_SetType(c: &mut Ctx, mut p_task_1: u32, mut p_type_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_SetType", &PROBE_N, &[(p_task_1) as u32, (p_type_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftRibbon_Update)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(208)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let v_ = p_type_2; let a_ = ((l_w_3).wrapping_add(16)).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRibbon_SetType", &PROBE_N, &[], true); }
    r_
}

pub fn EftZapMgr_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftZapMgr_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((7856u32) as i32)) }; let a_ = sym::gEftZapMgr; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gEftZapMgr); F::memset(c, a0_, 0i32, 7856u32) }; };
    { let v_ = F::BtlTask_CreateChildList(c, p_task_1, ((((5i32) as i64 as u64)) as u32), ((((((400u32) as i32)) as i64 as u64)) as u32)); let a_ = c.m.r32(sym::gEftZapMgr); c.m.w32(a_, v_); };
    { let a0_ = (c.m.r32(sym::gEftZapMgr)).wrapping_add(7776); F::Mtx_StoreIdentity(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftZapMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftZapMgr_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftZapMgr_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftZapMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftZapMgr_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftZapMgr_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftZapMgr_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftZapMgr_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftZapMgr_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftZapMgr_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftZapMgr_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftZapMgr_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftZapMgr); F::BtlPool_Free(c, a0_, a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftZapMgr; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftZapMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftZap_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftZap_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let _ = F::memset(c, l_w_3, 0i32, 400u32); };
    { let s_ = p_arg_2; let d_ = (l_w_3).wrapping_add(48); rt::memcpy(c, d_, s_, 80); };
    if (((({ let a_ = (c.m.r32((l_w_3).wrapping_add(48))).wrapping_add(180); (c.m.r32(a_) as i32) }) & (16i32))) != 0) {
        F::EftZap_InitKeys(c, p_task_1);
        { let r_ = 16384i32; let a_ = (l_w_3).wrapping_add(380); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        F::EftZap_LoadKey(c, l_w_3, 2i32);
    }
    if (((rt::fgt(F32(c.m.r32(((l_w_3).wrapping_add(48)).wrapping_add(52))), F32(0x00000000)) as i32)) != 0) {
        { let v_ = rt::fmul(F32(c.m.r32(((l_w_3).wrapping_add(48)).wrapping_add(52))), F32(0x41f00000)); let a_ = (l_w_3).wrapping_add(372); c.m.w32(a_, (v_).0); };
    } else {
        { let r_ = 262144i32; let a_ = (l_w_3).wrapping_add(380); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    { let r_ = 1i32; let a_ = (l_w_3).wrapping_add(380); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    { let a1_ = c.m.r32(((l_w_3).wrapping_add(48)).wrapping_add(8)); let a2_ = (c.m.r32(((l_w_3).wrapping_add(48)).wrapping_add(56)) as i32); let a3_ = (c.m.r32(((l_w_3).wrapping_add(48)).wrapping_add(56)) as i32); F::EftZap_SetTexPair(c, l_w_3, a1_, a2_, a3_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftZap_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftZap_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftZap_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_one_2: F32 = F32(0);
    let mut l_w_3: u32 = 0;
    let mut l_arg_4: u32 = 0;
    let mut l_def_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_t_7: F32 = F32(0);
    let r_ = 'f: {
        l_one_2 = F32(0x3f800000);
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        l_arg_4 = (l_w_3).wrapping_add(48);
        l_def_5 = c.m.r32(l_arg_4);
        if ((((!(({ let a0_ = (c.m.r32((l_arg_4).wrapping_add(60)) as i32); let a1_ = ((c.m.r8(((l_w_3).wrapping_add(48)).wrapping_add(64))) as i32); F::BtlScene_IsEffectStopped(c, a0_, a1_) }) != 0)) as i32)) != 0) {
            if (((rt::fgt(F32(c.m.r32((l_w_3).wrapping_add(352))), F32(0x00000000)) as i32)) != 0) {
                { let r_ = l_one_2; let a_ = (l_w_3).wrapping_add(352); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            }
            if (((((((((c.m.r32((l_w_3).wrapping_add(380)) as i32)) & (16i32))) != 0) && ((((!(((((c.m.r32((l_w_3).wrapping_add(380)) as i32)) & (4i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
                l_t_7 = rt::fdiv(F32(c.m.r32((l_w_3).wrapping_add(360))), F32(c.m.r32((l_w_3).wrapping_add(364))));
                if (((rt::flt(l_t_7, F32(0x00000000)) as i32)) != 0) {
                    l_t_7 = F32(0x00000000);
                } else {
                    if (((rt::fgt(l_t_7, F32(0x3f800000)) as i32)) != 0) {
                        l_t_7 = F32(0x3f800000);
                    }
                }
                { let r_ = F32(0x3f800000); let a_ = (l_w_3).wrapping_add(360); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::flt(F32(c.m.r32((l_w_3).wrapping_add(360))), F32(0x00000000)) as i32)) != 0) {
                    { let r_ = (!(1i32)); let a_ = (l_w_3).wrapping_add(380); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                    { let r_ = 2i32; let a_ = (l_w_3).wrapping_add(380); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
            {
                rt::memset(c, fp, 0, 16);
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0), (v_).0); }
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(4), (v_).0); }
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(8), (v_).0); }
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(12), (v_).0); }
                { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
                { let a2_ = F32(c.m.r32((l_def_5).wrapping_add(140))); { let _ = D::call(c, fid::Vec3_Scale, &[((fp) as u64), (((l_arg_4).wrapping_add(16)) as u64), ((a2_).0 as u64)]); } };
                { let _ = D::call(c, fid::Vec3_Add, &[(((l_w_3).wrapping_add(128)) as u64), (((l_arg_4).wrapping_add(32)) as u64), ((fp) as u64)]); };
            }
            if (((((c.m.r32((l_w_3).wrapping_add(380)) as i32)) & (16384i32))) != 0) {
                F::EftZap_StepKeys(c, p_task_1);
            }
            if ((((!(((((c.m.r32((l_w_3).wrapping_add(380)) as i32)) & (8i32))) != 0)) as i32)) != 0) {
                if (((((rt::rem_i32(rt::f2i(F32(c.m.r32((l_w_3).wrapping_add(368)))), (({ let a_ = (c.m.r32(l_arg_4)).wrapping_add(176); c.m.r8(a_) }) as i32))) == (0i32)) as i32)) != 0) {
                    {
                        l_i_6 = 0i32;
                        'l1: loop {
                            if !(((((l_i_6) < ((({ let a_ = (c.m.r32(l_arg_4)).wrapping_add(177); c.m.r8(a_) }) as i32))) as i32)) != 0) { break 'l1; }
                            'c2: {
                                F::EftZap_SpawnLine(c, l_w_3);
                            }
                            { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                        }
                    }
                }
            }
            F::EftZap_UpdateLines(c, l_w_3);
            if ((((!(((((c.m.r32((l_w_3).wrapping_add(380)) as i32)) & (262144i32))) != 0)) as i32)) != 0) {
                { let r_ = F32(0x3f800000); let a_ = (l_w_3).wrapping_add(372); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::flt(F32(c.m.r32((l_w_3).wrapping_add(372))), F32(0x00000000)) as i32)) != 0) {
                    if (((rt::fgt(F32(c.m.r32((l_w_3).wrapping_add(360))), F32(0x00000000)) as i32)) != 0) {
                        { let r_ = 16i32; let a_ = (l_w_3).wrapping_add(380); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    } else {
                        if (((((c.m.r32((l_def_5).wrapping_add(180)) as i32)) & (256i32))) != 0) {
                            { let r_ = 32i32; let a_ = (l_w_3).wrapping_add(380); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                        { let r_ = 8i32; let a_ = (l_w_3).wrapping_add(380); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                }
            }
            if (((((c.m.r32((l_w_3).wrapping_add(380)) as i32)) & (4i32))) != 0) {
                { let r_ = F32(0x3f800000); let a_ = (l_w_3).wrapping_add(356); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::flt(F32(c.m.r32((l_w_3).wrapping_add(356))), F32(0x00000000)) as i32)) != 0) {
                    { let r_ = (!(4i32)); let a_ = (l_w_3).wrapping_add(380); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                    if ((((!(((((c.m.r32((l_w_3).wrapping_add(380)) as i32)) & (16i32))) != 0)) as i32)) != 0) {
                        { let r_ = 8i32; let a_ = (l_w_3).wrapping_add(380); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                }
            }
            if (((((c.m.r32((l_w_3).wrapping_add(384))) == (((0i32) as u32))) as i32)) != 0) {
                if (((((c.m.r32((l_w_3).wrapping_add(380)) as i32)) & (8i32))) != 0) {
                    { let r_ = (!(1i32)); let a_ = (l_w_3).wrapping_add(380); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                    { let r_ = 2i32; let a_ = (l_w_3).wrapping_add(380); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
            if (((((((((c.m.r32((l_w_3).wrapping_add(380)) as i32)) & (64i32))) != 0) || (((((c.m.r32((l_w_3).wrapping_add(380)) as i32)) & (2i32))) != 0)) as i32)) != 0) {
                F::BtlTask_SetDead(c, p_task_1);
            }
            { let r_ = F32(0x3f800000); let a_ = (l_w_3).wrapping_add(368); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
        if (((((c.m.r32((l_w_3).wrapping_add(380)) as i32)) & (16384i32))) != 0) {
            F::EftZap_InitKeys(c, p_task_1);
            if (((rt::fge(F32(c.m.r32((l_w_3).wrapping_add(368))), F32(c.m.r32((l_w_3).wrapping_add(344)))) as i32)) != 0) {
                F::EftZap_LoadKey(c, l_w_3, 2i32);
                { let r_ = (!(16384i32)); let a_ = (l_w_3).wrapping_add(380); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
        if ((((!(((((c.m.r32((l_w_3).wrapping_add(380)) as i32)) & (64i32))) != 0)) as i32)) != 0) {
            if ((((!(((((c.m.r32((l_w_3).wrapping_add(380)) as i32)) & (2i32))) != 0)) as i32)) != 0) {
                F::EftZap_LoadTex(c, l_w_3);
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftZap_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftZap_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftZap_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftZap_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftZap_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftZap_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(800) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 800);
    let mut l_w_2: u32 = 0;
    let mut l_def_3: u32 = 0;
    let mut l_set_4: u32 = 0;
    let mut l_line_5: u32 = 0;
    let mut l_pt_6: u32 = 0;
    let mut l_idx_7: i32 = 0;
    let mut l_first_8: i32 = 0;
    let mut l_i_9: i32 = 0;
    let mut l_width_10: F32 = F32(0);
    let mut l_du_11: F32 = F32(0);
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_def_3 = c.m.r32((l_w_2).wrapping_add(48));
        F::Vu0Cur_Push(c);
        { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
        {
            l_set_4 = fid::ClipVtx_Set;
            rt::memset(c, fp.wrapping_add(240), 0, 16);
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(240).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(240).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(240).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(240).wrapping_add(12), (v_).0); }
            rt::memset(c, fp.wrapping_add(256), 0, 16);
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(256).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(256).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(256).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(256).wrapping_add(12), (v_).0); }
            rt::memset(c, fp.wrapping_add(272), 0, 16);
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(272).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(272).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(272).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(272).wrapping_add(12), (v_).0); }
            rt::memset(c, fp.wrapping_add(288), 0, 64);
            rt::memset(c, fp.wrapping_add(288).wrapping_add(0), 0, 16);
            { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(288).wrapping_add(0).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(288).wrapping_add(0).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(288).wrapping_add(0).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(288).wrapping_add(0).wrapping_add(12), (v_).0); }
            rt::memset(c, fp.wrapping_add(288).wrapping_add(16), 0, 16);
            { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(288).wrapping_add(16).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(288).wrapping_add(16).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(288).wrapping_add(16).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(288).wrapping_add(16).wrapping_add(12), (v_).0); }
            rt::memset(c, fp.wrapping_add(288).wrapping_add(32), 0, 16);
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(288).wrapping_add(32).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(288).wrapping_add(32).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(288).wrapping_add(32).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(288).wrapping_add(32).wrapping_add(12), (v_).0); }
            rt::memset(c, fp.wrapping_add(288).wrapping_add(48), 0, 16);
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(288).wrapping_add(48).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(288).wrapping_add(48).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(288).wrapping_add(48).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(288).wrapping_add(48).wrapping_add(12), (v_).0); }
            rt::memset(c, fp.wrapping_add(352), 0, 432);
            rt::memset(c, fp.wrapping_add(352).wrapping_add(0), 0, 48);
            rt::memset(c, fp.wrapping_add(352).wrapping_add(0).wrapping_add(0), 0, 16);
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(352).wrapping_add(0).wrapping_add(0).wrapping_add(0), (v_).0); }
            rt::memset(c, fp.wrapping_add(784), 0, 8);
            { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(784).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(784).wrapping_add(4), (v_).0); }
            { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(544); { let _ = D::call(c, fid::Vec4_Copy, &[((fp.wrapping_add(256)) as u64), ((a1_) as u64)]); } };
            {
                l_line_5 = c.m.r32((l_w_2).wrapping_add(384));
                'l1: loop {
                    if !(((((l_line_5) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                    'c2: {
                        if (((((c.m.r32((l_line_5).wrapping_add(276)) as i32)) & (128i32))) != 0) {
                            l_width_10 = rt::fmul(rt::fmul(F32(c.m.r32((l_line_5).wrapping_add(144))), F32(0x3f000000)), F32(c.m.r32(((l_w_2).wrapping_add(48)).wrapping_add(48))));
                            l_pt_6 = c.m.r32((l_line_5).wrapping_add(280));
                            if (((((c.m.r32((l_def_3).wrapping_add(180)) as i32)) & (64i32))) != 0) {
                                {
                                    'l3: loop {
                                        if !(((((l_pt_6) != (((0i32) as u32))) as i32)) != 0) { break 'l3; }
                                        'c4: {
                                            F::Mtx_MulVec4(c, fp.wrapping_add(224), (l_line_5).wrapping_add(80), l_pt_6);
                                            if (((((c.m.r32((l_def_3).wrapping_add(180)) as i32)) & (128i32))) != 0) {
                                                { let a2_ = ((c.m.r8((l_def_3).wrapping_add(168))) as i32); let a3_ = ((((c.m.r32((l_w_2).wrapping_add(380)) as i32)).wrapping_shr((17i32) as u32)) & (1i32)); let a4_ = { let a_ = (c.m.r32(((l_w_2).wrapping_add(48)).wrapping_add(8))).wrapping_add((((((c.m.r8((l_w_2).wrapping_add(376))) as i32)).wrapping_shl((4i32) as u32)) as u32)); c.m.r64(a_) }; F::EftPrim_DrawQuadDepth(c, fp.wrapping_add(224), (l_line_5).wrapping_add(32), a2_, a3_, a4_, l_width_10, l_width_10, F32(0x00000000), F32(0x00000000), F32(0x3f800000), F32(0x3f800000), F32(0x00000000)) };
                                            } else {
                                                { let a9_ = ((c.m.r8((l_def_3).wrapping_add(168))) as i32); let a10_ = ((((c.m.r32((l_w_2).wrapping_add(380)) as i32)).wrapping_shr((17i32) as u32)) & (1i32)); let a11_ = { let a_ = (c.m.r32(((l_w_2).wrapping_add(48)).wrapping_add(8))).wrapping_add((((((c.m.r8((l_w_2).wrapping_add(376))) as i32)).wrapping_shl((4i32) as u32)) as u32)); c.m.r64(a_) }; F::EftGfx_DrawSprite(c, fp.wrapping_add(224), (l_line_5).wrapping_add(32), rt::fmul(l_width_10, F32(0x41800000)), rt::fmul(l_width_10, F32(0x41800000)), F32(0x00000000), F32(0x00000000), F32(0x3f800000), F32(0x3f800000), F32(0x00000000), a9_, a10_, a11_) };
                                            }
                                        }
                                        l_pt_6 = c.m.r32((l_pt_6).wrapping_add(36));
                                    }
                                }
                            } else {
                                l_first_8 = 1i32;
                                l_du_11 = F32(0x00000000);
                                { let v_ = l_du_11; let a_ = (fp.wrapping_add(784)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let v_ = l_du_11; let a_ = (fp.wrapping_add(784)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                l_idx_7 = 0i32;
                                l_du_11 = rt::fdiv(F32(0xbf800000), rt::i2f((c.m.r32((l_line_5).wrapping_add(272)) as i32)));
                                { let v_ = rt::fadd(l_du_11, F32(c.m.r32((fp.wrapping_add(784)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = (fp.wrapping_add(784)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                {
                                    'l5: loop {
                                        if !(((((l_pt_6) != (((0i32) as u32))) as i32)) != 0) { break 'l5; }
                                        'c6: {
                                            if (((((c.m.r32((l_pt_6).wrapping_add(36))) != (((0i32) as u32))) as i32)) != 0) {
                                                F::Mtx_MulVec4(c, fp.wrapping_add(224), (l_line_5).wrapping_add(80), l_pt_6);
                                                { let a2_ = c.m.r32((l_pt_6).wrapping_add(36)); F::Mtx_MulVec4(c, fp.wrapping_add(240), (l_line_5).wrapping_add(80), a2_) };
                                                { let _ = D::call(c, fid::Vec4_Sub, &[((fp.wrapping_add(64)) as u64), ((fp.wrapping_add(240)) as u64), ((fp.wrapping_add(224)) as u64)]); };
                                                's7: {
                                                    let sv12: i32 = ((c.m.r8((l_def_3).wrapping_add(169))) as i32);
                                                    match sv12 {
                                                        0i32 => {
                                                            { let _ = D::call(c, fid::Vec4_Sub, &[((fp.wrapping_add(80)) as u64), ((fp.wrapping_add(224)) as u64), ((fp.wrapping_add(256)) as u64)]); };
                                                            break 's7;
                                                        }
                                                        1i32 => {
                                                            { let _ = D::call(c, fid::Vec3_Sub, &[((fp.wrapping_add(80)) as u64), ((fp.wrapping_add(224)) as u64), (((l_w_2).wrapping_add(128)) as u64)]); };
                                                            break 's7;
                                                        }
                                                        _ => {
                                                        }
                                                    }
                                                }
                                                { let _ = D::call(c, fid::Vec3_Cross, &[((fp.wrapping_add(96)) as u64), ((fp.wrapping_add(80)) as u64), ((fp.wrapping_add(64)) as u64)]); };
                                                { let _ = D::call(c, fid::Vec3_Normalize, &[((fp.wrapping_add(96)) as u64), ((fp.wrapping_add(96)) as u64)]); };
                                                { let _ = D::call(c, fid::Vec3_Scale, &[((fp.wrapping_add(272)) as u64), ((fp.wrapping_add(96)) as u64), ((l_width_10).0 as u64)]); };
                                                if ((l_first_8) != 0) {
                                                    { let _ = D::call(c, fid::Vec3_Add, &[(((fp.wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as u64), ((fp.wrapping_add(224)) as u64), ((fp.wrapping_add(272)) as u64)]); };
                                                    { let _ = D::call(c, fid::Vec3_Sub, &[(((fp.wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as u64), ((fp.wrapping_add(224)) as u64), ((fp.wrapping_add(272)) as u64)]); };
                                                    l_first_8 = 0i32;
                                                    { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                                    { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                                } else {
                                                    if (((rt::flt(F::Vec3_Dot(c, fp.wrapping_add(96), fp.wrapping_add(112)), F32(0x00000000)) as i32)) != 0) {
                                                        { let _ = D::call(c, fid::Vec3_Scale, &[((fp.wrapping_add(96)) as u64), ((fp.wrapping_add(96)) as u64), ((F32(0xbf800000)).0 as u64)]); };
                                                        { let _ = D::call(c, fid::Vec3_Scale, &[((fp.wrapping_add(272)) as u64), ((fp.wrapping_add(272)) as u64), ((F32(0xbf800000)).0 as u64)]); };
                                                    }
                                                    { let _ = D::call(c, fid::Vec4_Copy, &[(((fp.wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as u64), ((fp.wrapping_add(192)) as u64)]); };
                                                    { let _ = D::call(c, fid::Vec4_Copy, &[(((fp.wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as u64), ((fp.wrapping_add(208)) as u64)]); };
                                                }
                                                { let _ = D::call(c, fid::Vec3_Add, &[(((fp.wrapping_add(128)).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as u64), ((fp.wrapping_add(240)) as u64), ((fp.wrapping_add(272)) as u64)]); };
                                                { let _ = D::call(c, fid::Vec3_Sub, &[(((fp.wrapping_add(128)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as u64), ((fp.wrapping_add(240)) as u64), ((fp.wrapping_add(272)) as u64)]); };
                                                { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(128)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                                { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(128)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                                { let _ = D::call(c, fid::Vec4_Copy, &[((fp.wrapping_add(192)) as u64), (((fp.wrapping_add(128)).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as u64)]); };
                                                { let _ = D::call(c, fid::Vec4_Copy, &[((fp.wrapping_add(208)) as u64), (((fp.wrapping_add(128)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as u64)]); };
                                                { let _ = D::call(c, fid::Vec4_Copy, &[((fp.wrapping_add(112)) as u64), ((fp.wrapping_add(96)) as u64)]); };
                                                { let v_ = F32(c.m.r32((fp.wrapping_add(784)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (fp.wrapping_add(288)).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                                                { let v_ = F32(c.m.r32((fp.wrapping_add(784)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (fp.wrapping_add(288)).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                                                { let v_ = F32(c.m.r32((fp.wrapping_add(784)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (fp.wrapping_add(288)).wrapping_add((((2i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                                                { let v_ = F32(c.m.r32((fp.wrapping_add(784)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (fp.wrapping_add(288)).wrapping_add((((3i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                                                if (((((c.m.r32((l_def_3).wrapping_add(180)) as i32)) & (128i32))) != 0) {
                                                    { let _ = F::Vu0Cur_ProjectPoints(c, fp, fp.wrapping_add(128), 4i32); };
                                                    {
                                                        l_i_9 = 0i32;
                                                        'l8: loop {
                                                            if !(((((l_i_9) < (2i32)) as i32)) != 0) { break 'l8; }
                                                            'c9: {
                                                                { let _ = D::call(c, l_set_4, &[(((fp.wrapping_add(352)).wrapping_add((((0i32) as u32)).wrapping_mul(48))) as u64), (((fp.wrapping_add(128)).wrapping_add((((l_i_9) as u32)).wrapping_mul(16))) as u64), (((fp.wrapping_add(288)).wrapping_add((((l_i_9) as u32)).wrapping_mul(16))) as u64), (((l_line_5).wrapping_add(32)) as u64)]); };
                                                                { let _ = D::call(c, l_set_4, &[(((fp.wrapping_add(352)).wrapping_add((((1i32) as u32)).wrapping_mul(48))) as u64), (((fp.wrapping_add(128)).wrapping_add(((((l_i_9).wrapping_add(1i32)) as u32)).wrapping_mul(16))) as u64), (((fp.wrapping_add(288)).wrapping_add(((((l_i_9).wrapping_add(1i32)) as u32)).wrapping_mul(16))) as u64), (((l_line_5).wrapping_add(32)) as u64)]); };
                                                                { let _ = D::call(c, l_set_4, &[(((fp.wrapping_add(352)).wrapping_add((((2i32) as u32)).wrapping_mul(48))) as u64), (((fp.wrapping_add(128)).wrapping_add(((((l_i_9).wrapping_add(2i32)) as u32)).wrapping_mul(16))) as u64), (((fp.wrapping_add(288)).wrapping_add(((((l_i_9).wrapping_add(2i32)) as u32)).wrapping_mul(16))) as u64), (((l_line_5).wrapping_add(32)) as u64)]); };
                                                                { let a1_ = ((c.m.r8((l_def_3).wrapping_add(168))) as i32); let a4_ = ((((c.m.r32((l_w_2).wrapping_add(380)) as i32)).wrapping_shr((17i32) as u32)) & (1i32)); let a6_ = { let a_ = (c.m.r32(((l_w_2).wrapping_add(48)).wrapping_add(8))).wrapping_add((((((c.m.r8((l_w_2).wrapping_add(376))) as i32)).wrapping_shl((4i32) as u32)) as u32)); c.m.r64(a_) }; { let _ = D::call(c, fid::EftGfx_DrawPolyScaledZ, &[((fp.wrapping_add(352)) as u64), ((a1_) as i64 as u64), ((0i32) as i64 as u64), ((0i32) as i64 as u64), ((a4_) as i64 as u64), ((0i32) as i64 as u64), ((a6_) as u64), ((F32(0x40000000)).0 as u64)]); } };
                                                            }
                                                            { let o_ = l_i_9; let n_ = (o_).wrapping_add(1); l_i_9 = n_; };
                                                        }
                                                    }
                                                } else {
                                                    { let a2_ = F32(c.m.r32((fp.wrapping_add(288)).wrapping_add((((0i32) as u32)).wrapping_mul(16)))); let a3_ = F32(c.m.r32(((fp.wrapping_add(288)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a4_ = F32(c.m.r32((fp.wrapping_add(288)).wrapping_add((((1i32) as u32)).wrapping_mul(16)))); let a5_ = F32(c.m.r32(((fp.wrapping_add(288)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a6_ = F32(c.m.r32((fp.wrapping_add(288)).wrapping_add((((2i32) as u32)).wrapping_mul(16)))); let a7_ = F32(c.m.r32(((fp.wrapping_add(288)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a8_ = F32(c.m.r32((fp.wrapping_add(288)).wrapping_add((((3i32) as u32)).wrapping_mul(16)))); let a9_ = F32(c.m.r32(((fp.wrapping_add(288)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a10_ = ((c.m.r8((l_def_3).wrapping_add(168))) as i32); let a11_ = ((c.m.r8((l_w_2).wrapping_add(376))) as i32); let a12_ = ((((c.m.r32((l_w_2).wrapping_add(380)) as i32)).wrapping_shr((17i32) as u32)) & (1i32)); let a13_ = c.m.r32(((l_w_2).wrapping_add(48)).wrapping_add(8)); F::EftZap_DrawQuad(c, fp.wrapping_add(128), (l_line_5).wrapping_add(32), a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_, a10_, a11_, a12_, a13_) };
                                                }
                                            }
                                            { let r_ = l_du_11; let a_ = (fp.wrapping_add(784)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                            { let r_ = l_du_11; let a_ = (fp.wrapping_add(784)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                            if ((((!(((((l_idx_7) < (((c.m.r32((l_line_5).wrapping_add(272)) as i32)).wrapping_sub(2i32))) as i32)) != 0)) as i32)) != 0) {
                                                { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(784)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                            }
                                            { let o_ = l_idx_7; let n_ = (o_).wrapping_add(1); l_idx_7 = n_; };
                                        }
                                        l_pt_6 = c.m.r32((l_pt_6).wrapping_add(36));
                                    }
                                }
                            }
                        }
                    }
                    l_line_5 = c.m.r32((l_line_5).wrapping_add(288));
                }
            }
        }
        F::Vu0Cur_Pop(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftZap_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftZap_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftZap_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftZap_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftZap_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftZap_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_line_3: u32 = 0;
    let mut l_pt_4: u32 = 0;
    let mut l_prev__5: u32 = 0;
    let mut l_next__6: u32 = 0;
    let mut l_next__7: u32 = 0;
    let mut l_prev__8: u32 = 0;
    let mut l_next__9: u32 = 0;
    let mut l_next__10: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    {
        l_line_3 = c.m.r32((l_w_2).wrapping_add(384));
        'l1: loop {
            if !(((((l_line_3) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
            'c2: {
                {
                    l_pt_4 = c.m.r32((l_line_3).wrapping_add(280));
                    'l3: loop {
                        if !(((((l_pt_4) != (((0i32) as u32))) as i32)) != 0) { break 'l3; }
                        'c4: {
                            { let v_ = 0i32; let a_ = (l_pt_4).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                            if (((((c.m.r32((l_line_3).wrapping_add(280))) != (((0i32) as u32))) as i32)) != 0) {
                                l_prev__5 = c.m.r32((l_pt_4).wrapping_add(40));
                                if (((((l_prev__5) == (((0i32) as u32))) as i32)) != 0) {
                                    l_next__6 = c.m.r32((l_pt_4).wrapping_add(36));
                                    if (((((l_next__6) == (((0i32) as u32))) as i32)) != 0) {
                                        { let v_ = ((0i32) as u32); let a_ = (l_line_3).wrapping_add(280); c.m.w32(a_, v_); };
                                        { let v_ = ((0i32) as u32); let a_ = (l_line_3).wrapping_add(284); c.m.w32(a_, v_); };
                                    } else {
                                        { let v_ = l_next__6; let a_ = (l_line_3).wrapping_add(280); c.m.w32(a_, v_); };
                                        { let v_ = ((0i32) as u32); let a_ = (l_next__6).wrapping_add(40); c.m.w32(a_, v_); };
                                    }
                                } else {
                                    l_next__7 = c.m.r32((l_pt_4).wrapping_add(36));
                                    if (((((l_next__7) == (((0i32) as u32))) as i32)) != 0) {
                                        { let v_ = l_prev__5; let a_ = (l_line_3).wrapping_add(284); c.m.w32(a_, v_); };
                                        { let v_ = ((0i32) as u32); let a_ = (l_prev__5).wrapping_add(36); c.m.w32(a_, v_); };
                                    } else {
                                        { let v_ = l_prev__5; let a_ = (l_next__7).wrapping_add(40); c.m.w32(a_, v_); };
                                        { let v_ = c.m.r32((l_pt_4).wrapping_add(36)); let a_ = (c.m.r32((l_pt_4).wrapping_add(40))).wrapping_add(36); c.m.w32(a_, v_); };
                                    }
                                }
                            }
                        }
                        l_pt_4 = c.m.r32((l_pt_4).wrapping_add(36));
                    }
                }
                { let v_ = 0i32; let a_ = (l_line_3).wrapping_add(276); c.m.w32(a_, (v_) as u32); };
                if (((((c.m.r32((l_w_2).wrapping_add(384))) != (((0i32) as u32))) as i32)) != 0) {
                    l_prev__8 = c.m.r32((l_line_3).wrapping_add(292));
                    if (((((l_prev__8) == (((0i32) as u32))) as i32)) != 0) {
                        l_next__9 = c.m.r32((l_line_3).wrapping_add(288));
                        if (((((l_next__9) == (((0i32) as u32))) as i32)) != 0) {
                            { let v_ = ((0i32) as u32); let a_ = (l_w_2).wrapping_add(384); c.m.w32(a_, v_); };
                            { let v_ = ((0i32) as u32); let a_ = (l_w_2).wrapping_add(388); c.m.w32(a_, v_); };
                        } else {
                            { let v_ = l_next__9; let a_ = (l_w_2).wrapping_add(384); c.m.w32(a_, v_); };
                            { let v_ = ((0i32) as u32); let a_ = (l_next__9).wrapping_add(292); c.m.w32(a_, v_); };
                        }
                    } else {
                        l_next__10 = c.m.r32((l_line_3).wrapping_add(288));
                        if (((((l_next__10) == (((0i32) as u32))) as i32)) != 0) {
                            { let v_ = l_prev__8; let a_ = (l_w_2).wrapping_add(388); c.m.w32(a_, v_); };
                            { let v_ = ((0i32) as u32); let a_ = (l_prev__8).wrapping_add(288); c.m.w32(a_, v_); };
                        } else {
                            { let v_ = l_prev__8; let a_ = (l_next__10).wrapping_add(292); c.m.w32(a_, v_); };
                            { let v_ = c.m.r32((l_line_3).wrapping_add(288)); let a_ = (c.m.r32((l_line_3).wrapping_add(292))).wrapping_add(288); c.m.w32(a_, v_); };
                        }
                    }
                }
            }
            l_line_3 = c.m.r32((l_line_3).wrapping_add(288));
        }
    }
    { let v_ = 0i32; let a_ = (l_w_2).wrapping_add(380); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftZap_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftZap_InitKeys(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftZap_InitKeys", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_k_3: u32 = 0;
    let mut l_def_4: u32 = 0;
    let mut l_n_5: i32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_k_3 = c.m.r32(((l_w_2).wrapping_add(48)).wrapping_add(4));
    l_def_4 = c.m.r32((l_w_2).wrapping_add(48));
    l_n_5 = 0i32;
    if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(368))), F32(0x00000000)) as i32)) != 0) {
        l_n_5 = 1i32;
        { let v_ = rt::fmul(F32(c.m.r32((l_def_4).wrapping_add(152))), F32(0x41f00000)); let a_ = (l_w_2).wrapping_add(344); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32((l_w_2).wrapping_add(344))), F32(c.m.r32((l_def_4).wrapping_add(156)))); let a_ = (l_w_2).wrapping_add(348); c.m.w32(a_, (v_).0); };
    }
    if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(348))), F32(c.m.r32((l_w_2).wrapping_add(368)))) as i32)) != 0) {
        if ((((!(((((c.m.r32((l_w_2).wrapping_add(380)) as i32)) & (32768i32))) != 0)) as i32)) != 0) {
            { let r_ = 32768i32; let a_ = (l_w_2).wrapping_add(380); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            l_n_5 = 2i32;
        }
    }
    if (((((l_n_5) != (0i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::Vec4_Sub, &[(((l_w_2).wrapping_add(160)) as u64), (((l_k_3).wrapping_add((((l_n_5) as u32)).wrapping_mul(16))) as u64), (((l_k_3).wrapping_add(((((l_n_5).wrapping_sub(1i32)) as u32)).wrapping_mul(16))) as u64)]); };
        { let _ = D::call(c, fid::Vec4_Sub, &[(((l_w_2).wrapping_add(192)) as u64), ((((l_k_3).wrapping_add(48)).wrapping_add((((l_n_5) as u32)).wrapping_mul(16))) as u64), ((((l_k_3).wrapping_add(48)).wrapping_add(((((l_n_5).wrapping_sub(1i32)) as u32)).wrapping_mul(16))) as u64)]); };
        { let _ = D::call(c, fid::Vec4_Sub, &[(((l_w_2).wrapping_add(224)) as u64), ((((l_k_3).wrapping_add(96)).wrapping_add((((l_n_5) as u32)).wrapping_mul(16))) as u64), ((((l_k_3).wrapping_add(96)).wrapping_add(((((l_n_5).wrapping_sub(1i32)) as u32)).wrapping_mul(16))) as u64)]); };
        { let _ = D::call(c, fid::Vec4_Sub, &[(((l_w_2).wrapping_add(256)) as u64), ((((l_k_3).wrapping_add(144)).wrapping_add((((l_n_5) as u32)).wrapping_mul(16))) as u64), ((((l_k_3).wrapping_add(144)).wrapping_add(((((l_n_5).wrapping_sub(1i32)) as u32)).wrapping_mul(16))) as u64)]); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_k_3).wrapping_add(192)).wrapping_add((((l_n_5) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_k_3).wrapping_add(192)).wrapping_add(((((l_n_5).wrapping_sub(1i32)) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (((l_w_2).wrapping_add(272)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_k_3).wrapping_add(192)).wrapping_add((((l_n_5) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_k_3).wrapping_add(192)).wrapping_add(((((l_n_5).wrapping_sub(1i32)) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = (((l_w_2).wrapping_add(272)).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_k_3).wrapping_add(216)).wrapping_add((((l_n_5) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_k_3).wrapping_add(216)).wrapping_add(((((l_n_5).wrapping_sub(1i32)) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (((l_w_2).wrapping_add(288)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_k_3).wrapping_add(216)).wrapping_add((((l_n_5) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_k_3).wrapping_add(216)).wrapping_add(((((l_n_5).wrapping_sub(1i32)) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = (((l_w_2).wrapping_add(288)).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_k_3).wrapping_add(240)).wrapping_add((((l_n_5) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_k_3).wrapping_add(240)).wrapping_add(((((l_n_5).wrapping_sub(1i32)) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (((l_w_2).wrapping_add(304)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_k_3).wrapping_add(240)).wrapping_add((((l_n_5) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_k_3).wrapping_add(240)).wrapping_add(((((l_n_5).wrapping_sub(1i32)) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = (((l_w_2).wrapping_add(304)).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_k_3).wrapping_add(264)).wrapping_add((((l_n_5) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_k_3).wrapping_add(264)).wrapping_add(((((l_n_5).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((l_w_2).wrapping_add(320)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_k_3).wrapping_add(276)).wrapping_add((((l_n_5) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_k_3).wrapping_add(276)).wrapping_add(((((l_n_5).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((l_w_2).wrapping_add(328)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_k_3).wrapping_add(288)).wrapping_add((((l_n_5) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_k_3).wrapping_add(288)).wrapping_add(((((l_n_5).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((l_w_2).wrapping_add(336)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftZap_InitKeys", &PROBE_N, &[], true); }
    r_
}

pub fn EftZap_StepKeys(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftZap_StepKeys", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_2: u32 = 0;
    let mut l_k_3: u32 = 0;
    let mut l_n_4: i32 = 0;
    let mut l_t_5: F32 = F32(0);
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_k_3 = c.m.r32(((l_w_2).wrapping_add(48)).wrapping_add(4));
        l_n_4 = 0i32;
        l_t_5 = F32(0x00000000);
        rt::memset(c, fp, 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(12), (v_).0); }
        if ((((!(((((c.m.r32((l_w_2).wrapping_add(380)) as i32)) & (32768i32))) != 0)) as i32)) != 0) {
            if (((rt::fgt(F32(c.m.r32((l_w_2).wrapping_add(348))), F32(0x00000000)) as i32)) != 0) {
                l_t_5 = rt::fdiv(F32(c.m.r32((l_w_2).wrapping_add(368))), F32(c.m.r32((l_w_2).wrapping_add(348))));
            }
        } else {
            l_n_4 = 1i32;
            l_t_5 = rt::fdiv(rt::fsub(F32(c.m.r32((l_w_2).wrapping_add(368))), F32(c.m.r32((l_w_2).wrapping_add(348)))), rt::fsub(F32(c.m.r32((l_w_2).wrapping_add(344))), F32(c.m.r32((l_w_2).wrapping_add(348)))));
        }
        F::Vec4_Scale(c, fp, (l_w_2).wrapping_add(160), l_t_5);
        F::Vec4_Add(c, (l_w_2).wrapping_add(144), (l_k_3).wrapping_add((((l_n_4) as u32)).wrapping_mul(16)), fp);
        F::Vec4_Scale(c, fp, (l_w_2).wrapping_add(224), l_t_5);
        F::Vec4_Add(c, (l_w_2).wrapping_add(208), ((l_k_3).wrapping_add(96)).wrapping_add((((l_n_4) as u32)).wrapping_mul(16)), fp);
        F::Vec4_Scale(c, fp, (l_w_2).wrapping_add(192), l_t_5);
        F::Vec4_Add(c, (l_w_2).wrapping_add(176), ((l_k_3).wrapping_add(48)).wrapping_add((((l_n_4) as u32)).wrapping_mul(16)), fp);
        F::Vec4_Scale(c, fp, (l_w_2).wrapping_add(256), l_t_5);
        F::Vec4_Add(c, (l_w_2).wrapping_add(240), ((l_k_3).wrapping_add(144)).wrapping_add((((l_n_4) as u32)).wrapping_mul(16)), fp);
        { let v_ = rt::fadd(F32(c.m.r32((((l_k_3).wrapping_add(192)).wrapping_add((((l_n_4) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((l_w_2).wrapping_add(272)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = ((l_w_2).wrapping_add(272)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_k_3).wrapping_add(192)).wrapping_add((((l_n_4) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((l_w_2).wrapping_add(272)).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = ((l_w_2).wrapping_add(272)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_k_3).wrapping_add(216)).wrapping_add((((l_n_4) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((l_w_2).wrapping_add(288)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = ((l_w_2).wrapping_add(288)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_k_3).wrapping_add(216)).wrapping_add((((l_n_4) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((l_w_2).wrapping_add(288)).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = ((l_w_2).wrapping_add(288)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_k_3).wrapping_add(240)).wrapping_add((((l_n_4) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((l_w_2).wrapping_add(304)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = ((l_w_2).wrapping_add(304)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_k_3).wrapping_add(240)).wrapping_add((((l_n_4) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((l_w_2).wrapping_add(304)).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_5)); let a_ = ((l_w_2).wrapping_add(304)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_k_3).wrapping_add(264)).wrapping_add((((l_n_4) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_w_2).wrapping_add(320)).wrapping_add(4))), l_t_5)); let a_ = (l_w_2).wrapping_add(320); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_k_3).wrapping_add(276)).wrapping_add((((l_n_4) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_w_2).wrapping_add(328)).wrapping_add(4))), l_t_5)); let a_ = (l_w_2).wrapping_add(328); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_k_3).wrapping_add(288)).wrapping_add((((l_n_4) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_w_2).wrapping_add(336)).wrapping_add(4))), l_t_5)); let a_ = (l_w_2).wrapping_add(336); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftZap_StepKeys", &PROBE_N, &[], true); }
    r_
}

pub fn EftZap_SpawnLine(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftZap_SpawnLine", &PROBE_N, &[(p_w_1) as u32], false); }
    let mut l_line_2: u32 = 0;
    let mut l_i_3: u8 = 0;
    let r_ = 'f: {
    if ((((((({ let a_ = (c.m.r32(sym::gEftZapMgr)).wrapping_add(7840); c.m.r8(a_) }) as i32)) >= (20i32)) as i32)) != 0) {
        { let v_ = ((0i32) as u8); let a_ = (c.m.r32(sym::gEftZapMgr)).wrapping_add(7840); c.m.w8(a_, v_); };
    }
    l_i_3 = { let a_ = (c.m.r32(sym::gEftZapMgr)).wrapping_add(7840); c.m.r8(a_) };
    'l1: loop {
        'c2: {
            l_line_2 = ((c.m.r32(sym::gEftZapMgr)).wrapping_add(16)).wrapping_add((((l_i_3) as u32)).wrapping_mul(304));
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            if (((((((l_i_3) as i32)) >= (20i32)) as i32)) != 0) {
                l_i_3 = ((0i32) as u8);
            }
            if ((((((c.m.r32((l_line_2).wrapping_add(276)) as i32)) == (0i32)) as i32)) != 0) {
                if ((F::EftZap_InitLine(c, l_line_2, p_w_1)) != 0) {
                    { let r_ = 1i32; let a_ = (l_line_2).wrapping_add(276); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    if (((((c.m.r32((p_w_1).wrapping_add(384))) == (((0i32) as u32))) as i32)) != 0) {
                        { let v_ = l_line_2; let a_ = (p_w_1).wrapping_add(384); c.m.w32(a_, v_); };
                        { let v_ = l_line_2; let a_ = (p_w_1).wrapping_add(388); c.m.w32(a_, v_); };
                    } else {
                        { let v_ = c.m.r32((p_w_1).wrapping_add(388)); let a_ = (l_line_2).wrapping_add(292); c.m.w32(a_, v_); };
                        { let v_ = l_line_2; let a_ = (c.m.r32((p_w_1).wrapping_add(388))).wrapping_add(288); c.m.w32(a_, v_); };
                        { let v_ = l_line_2; let a_ = (p_w_1).wrapping_add(388); c.m.w32(a_, v_); };
                    }
                    { let v_ = l_i_3; let a_ = (c.m.r32(sym::gEftZapMgr)).wrapping_add(7840); c.m.w8(a_, v_); };
                }
                break 'f;
            }
        }
        if !(((((((l_i_3) as i32)) != ((({ let a_ = (c.m.r32(sym::gEftZapMgr)).wrapping_add(7840); c.m.r8(a_) }) as i32))) as i32)) != 0) { break 'l1; }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftZap_SpawnLine", &PROBE_N, &[], true); }
    r_
}

pub fn EftZap_InitLine(c: &mut Ctx, mut p_line_1: u32, mut p_w_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftZap_InitLine", &PROBE_N, &[(p_line_1) as u32, (p_w_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_def_3: u32 = 0;
    let mut l_fps_4: F32 = F32(0);
    let mut l_i_5: i32 = 0;
    let mut l_life_6: F32 = F32(0);
    let mut l_pt_7: u32 = 0;
    let mut l_prev__8: u32 = 0;
    let mut l_next__9: u32 = 0;
    let mut l_next__10: u32 = 0;
    let r_ = 'f: {
        l_def_3 = c.m.r32((p_w_2).wrapping_add(48));
        l_fps_4 = F32(0x41f00000);
        rt::memset(c, fp, 0, 12);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(8), (v_).0); }
        { let v_ = rt::fmul({ let a0_ = F32(c.m.r32(l_def_3)); let a1_ = rt::fadd(F32(c.m.r32(l_def_3)), F32(c.m.r32((l_def_3).wrapping_add(4)))); F::Rand_FloatRange(c, a0_, a1_) }, l_fps_4); let a_ = (p_line_1).wrapping_add(264); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_line_1).wrapping_add(268); c.m.w32(a_, (v_).0); };
        if (((rt::fle(F32(c.m.r32((p_line_1).wrapping_add(264))), F32(c.m.r32((p_line_1).wrapping_add(268)))) as i32)) != 0) {
            break 'f 0i32;
        }
        { let v_ = ((0i32) as u32); let a_ = (p_line_1).wrapping_add(280); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (p_line_1).wrapping_add(284); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (p_line_1).wrapping_add(288); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (p_line_1).wrapping_add(292); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = (c.m.r32((l_def_3).wrapping_add(160)) as i32); let a1_ = ((c.m.r32((l_def_3).wrapping_add(160)) as i32)).wrapping_add((c.m.r32((l_def_3).wrapping_add(164)) as i32)); F::Rand_IntRange(c, a0_, a1_) }; let a_ = (p_line_1).wrapping_add(272); c.m.w32(a_, (v_) as u32); };
        if ((((((c.m.r32((p_line_1).wrapping_add(272)) as i32)) <= (0i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        { let v_ = rt::fmul(F32(c.m.r32((p_line_1).wrapping_add(264))), F32(c.m.r32((l_def_3).wrapping_add(16)))); let a_ = (p_line_1).wrapping_add(248); c.m.w32(a_, (v_).0); };
        { let v_ = { let v_ = rt::fmul(F32(c.m.r32((p_line_1).wrapping_add(264))), rt::fsub(F32(0x3f800000), F32(c.m.r32((l_def_3).wrapping_add(20))))); let a_ = (p_line_1).wrapping_add(252); c.m.w32(a_, (v_).0); v_ }; let a_ = (p_line_1).wrapping_add(256); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((p_w_2).wrapping_add(328))); let a_ = (p_line_1).wrapping_add(236); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((p_w_2).wrapping_add(336))); let a_ = (p_line_1).wrapping_add(240); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_line_1).wrapping_add(228); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32((p_line_1).wrapping_add(264))), rt::fsub(F32(c.m.r32((p_line_1).wrapping_add(240))), F32(c.m.r32((p_line_1).wrapping_add(236))))); let a_ = (p_line_1).wrapping_add(232); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul({ let a0_ = F32(c.m.r32((l_def_3).wrapping_add(8))); let a1_ = rt::fadd(F32(c.m.r32((l_def_3).wrapping_add(8))), F32(c.m.r32((l_def_3).wrapping_add(12)))); F::Rand_FloatRange(c, a0_, a1_) }, l_fps_4); let a_ = (p_line_1).wrapping_add(260); c.m.w32(a_, (v_).0); };
        { let a1_ = (c.m.r32(sym::gEftZapMgr)).wrapping_add(7776); F::Mtx_Copy(c, (p_line_1).wrapping_add(80), a1_) };
        if (((((((((((c.m.r8((l_def_3).wrapping_add(171))) as i32)) == (0i32)) as i32)) != 0) || (((((((((((c.m.r8((l_def_3).wrapping_add(171))) as i32)) == (2i32)) as i32)) != 0) && ((((!((((F::Port_Rand(c)) & (1i32))) != 0)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
            { let r_ = 512i32; let a_ = (p_line_1).wrapping_add(276); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        { let v_ = { let a0_ = F32(c.m.r32(((l_def_3).wrapping_add(76)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(76)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(88)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((l_def_3).wrapping_add(76)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(76)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(88)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((l_def_3).wrapping_add(76)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(76)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(88)).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        if (((((c.m.r32((l_def_3).wrapping_add(180)) as i32)) & (1i32))) != 0) {
            { let v_ = rt::fmul(rt::fadd(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4))))), F32(0x3f000000)); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        }
        { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((p_line_1).wrapping_add(264))), F32(c.m.r32((l_def_3).wrapping_add(24))))); let a_ = ((p_line_1).wrapping_add(188)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((p_line_1).wrapping_add(264))), rt::fsub(F32(0x3f800000), F32(c.m.r32((l_def_3).wrapping_add(24)))))); let a_ = ((p_line_1).wrapping_add(188)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_line_1).wrapping_add(160); c.m.w32(a_, (v_).0); };
        if (((((((((((c.m.r8((l_def_3).wrapping_add(173))) as i32)) == (0i32)) as i32)) != 0) || (((((((((((c.m.r8((l_def_3).wrapping_add(173))) as i32)) == (2i32)) as i32)) != 0) && ((((!((((F::Port_Rand(c)) & (1i32))) != 0)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
            { let r_ = 2048i32; let a_ = (p_line_1).wrapping_add(276); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        { let v_ = { let a0_ = F32(c.m.r32(((l_def_3).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((l_def_3).wrapping_add(52)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(52)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((l_def_3).wrapping_add(52)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(52)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        if (((((c.m.r32((l_def_3).wrapping_add(180)) as i32)) & (4i32))) != 0) {
            { let v_ = rt::fmul(rt::fadd(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4))))), F32(0x3f000000)); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        }
        { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((p_line_1).wrapping_add(264))), F32(c.m.r32((l_def_3).wrapping_add(24))))); let a_ = ((p_line_1).wrapping_add(180)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((p_line_1).wrapping_add(264))), rt::fsub(F32(0x3f800000), F32(c.m.r32((l_def_3).wrapping_add(24)))))); let a_ = ((p_line_1).wrapping_add(180)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_line_1).wrapping_add(152); c.m.w32(a_, (v_).0); };
        if (((((((((((c.m.r8((l_def_3).wrapping_add(174))) as i32)) == (0i32)) as i32)) != 0) || (((((((((((c.m.r8((l_def_3).wrapping_add(174))) as i32)) == (2i32)) as i32)) != 0) && ((((!((((F::Port_Rand(c)) & (1i32))) != 0)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
            { let r_ = 4096i32; let a_ = (p_line_1).wrapping_add(276); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if (((((((((((c.m.r8((l_def_3).wrapping_add(175))) as i32)) == (1i32)) as i32)) != 0) || (((((((((((c.m.r8((l_def_3).wrapping_add(175))) as i32)) == (2i32)) as i32)) != 0) && ((((!((((F::Port_Rand(c)) & (1i32))) != 0)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
            { let r_ = 8192i32; let a_ = (p_line_1).wrapping_add(276); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        { let v_ = { let a0_ = rt::fmul({ let a0_ = F32(c.m.r32((l_def_3).wrapping_add(124))); let a1_ = rt::fadd(F32(c.m.r32((l_def_3).wrapping_add(124))), F32(c.m.r32((l_def_3).wrapping_add(128)))); F::Rand_FloatRange(c, a0_, a1_) }, F32(0x40c90fda)); F::EftMath_WrapAngle(c, a0_) }; let a_ = (p_line_1).wrapping_add(156); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul({ let a0_ = F32(c.m.r32(((l_def_3).wrapping_add(100)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(100)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }, F32(0x40490fda)); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul({ let a0_ = F32(c.m.r32(((l_def_3).wrapping_add(100)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(100)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(112)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }, F32(0x40490fda)); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul({ let a0_ = F32(c.m.r32(((l_def_3).wrapping_add(100)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(100)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(112)).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }, F32(0x40490fda)); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        if (((((c.m.r32((l_def_3).wrapping_add(180)) as i32)) & (8i32))) != 0) {
            { let v_ = rt::fmul(rt::fadd(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4))))), F32(0x3f000000)); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        }
        { let v_ = { let a0_ = rt::fdiv(rt::fsub(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((p_line_1).wrapping_add(264))), F32(c.m.r32((l_def_3).wrapping_add(24))))); F::EftMath_WrapAngle(c, a0_) }; let a_ = ((p_line_1).wrapping_add(172)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = rt::fdiv(rt::fsub(F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((p_line_1).wrapping_add(264))), rt::fsub(F32(0x3f800000), F32(c.m.r32((l_def_3).wrapping_add(24)))))); F::EftMath_WrapAngle(c, a0_) }; let a_ = ((p_line_1).wrapping_add(172)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); F::EftMath_WrapAngle(c, a0_) }; let a_ = (p_line_1).wrapping_add(148); c.m.w32(a_, (v_).0); };
        if (((((((((((c.m.r8((l_def_3).wrapping_add(172))) as i32)) == (0i32)) as i32)) != 0) || (((((((((((c.m.r8((l_def_3).wrapping_add(172))) as i32)) == (2i32)) as i32)) != 0) && ((((!((((F::Port_Rand(c)) & (1i32))) != 0)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
            { let r_ = 1024i32; let a_ = (p_line_1).wrapping_add(276); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        { let v_ = { let a0_ = F32(c.m.r32(((l_def_3).wrapping_add(28)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(28)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(40)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((l_def_3).wrapping_add(28)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(28)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((l_def_3).wrapping_add(28)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((l_def_3).wrapping_add(28)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        if (((((c.m.r32((l_def_3).wrapping_add(180)) as i32)) & (2i32))) != 0) {
            { let v_ = rt::fmul(rt::fadd(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4))))), F32(0x3f000000)); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        }
        {
            l_life_6 = F32(c.m.r32((p_line_1).wrapping_add(264)));
            { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), rt::fmul(l_life_6, F32(c.m.r32((l_def_3).wrapping_add(24))))); let a_ = ((p_line_1).wrapping_add(164)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), rt::fmul(l_life_6, rt::fsub(F32(0x3f800000), F32(c.m.r32((l_def_3).wrapping_add(24)))))); let a_ = ((p_line_1).wrapping_add(164)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_line_1).wrapping_add(144); c.m.w32(a_, (v_).0); };
        }
        {
            rt::memset(c, fp.wrapping_add(16), 0, 16);
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(12), (v_).0); }
            if (((((((((((c.m.r8((l_def_3).wrapping_add(170))) as i32)) == (0i32)) as i32)) != 0) || (((((((((((c.m.r8((l_def_3).wrapping_add(170))) as i32)) == (2i32)) as i32)) != 0) && ((((!((((F::Port_Rand(c)) & (1i32))) != 0)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
                { let r_ = 256i32; let a_ = (p_line_1).wrapping_add(276); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
            { let v_ = { let a0_ = F32(c.m.r32((p_w_2).wrapping_add(144))); let a1_ = rt::fadd(F32(c.m.r32((p_w_2).wrapping_add(144))), F32(c.m.r32((p_w_2).wrapping_add(176)))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = p_line_1; c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = F32(c.m.r32(((p_w_2).wrapping_add(144)).wrapping_add(4))); let a1_ = rt::fadd(F32(c.m.r32(((p_w_2).wrapping_add(144)).wrapping_add(4))), F32(c.m.r32(((p_w_2).wrapping_add(176)).wrapping_add(4)))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (p_line_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = F32(c.m.r32(((p_w_2).wrapping_add(144)).wrapping_add(8))); let a1_ = rt::fadd(F32(c.m.r32(((p_w_2).wrapping_add(144)).wrapping_add(8))), F32(c.m.r32(((p_w_2).wrapping_add(176)).wrapping_add(8)))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (p_line_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = F32(c.m.r32(((p_w_2).wrapping_add(144)).wrapping_add(12))); let a1_ = rt::fadd(F32(c.m.r32(((p_w_2).wrapping_add(144)).wrapping_add(12))), F32(c.m.r32(((p_w_2).wrapping_add(176)).wrapping_add(12)))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (p_line_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = F32(c.m.r32((p_w_2).wrapping_add(208))); let a1_ = rt::fadd(F32(c.m.r32((p_w_2).wrapping_add(208))), F32(c.m.r32((p_w_2).wrapping_add(240)))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = F32(c.m.r32(((p_w_2).wrapping_add(208)).wrapping_add(4))); let a1_ = rt::fadd(F32(c.m.r32(((p_w_2).wrapping_add(208)).wrapping_add(4))), F32(c.m.r32(((p_w_2).wrapping_add(240)).wrapping_add(4)))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = F32(c.m.r32(((p_w_2).wrapping_add(208)).wrapping_add(8))); let a1_ = rt::fadd(F32(c.m.r32(((p_w_2).wrapping_add(208)).wrapping_add(8))), F32(c.m.r32(((p_w_2).wrapping_add(240)).wrapping_add(8)))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            F::Vec4_Clamp(c, p_line_1, p_line_1, F32(0x00000000), F32(0x437f0000));
            F::Vec4_Clamp(c, fp.wrapping_add(16), fp.wrapping_add(16), F32(0x00000000), F32(0x437f0000));
            { let _ = D::call(c, fid::Vec4_Copy, &[(((p_line_1).wrapping_add(32)) as u64), ((p_line_1) as u64)]); };
            { let v_ = F32(0x00000000); let a_ = ((p_line_1).wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            { let _ = D::call(c, fid::Vec3_Sub, &[(((p_line_1).wrapping_add(16)) as u64), ((fp.wrapping_add(16)) as u64), (((p_line_1).wrapping_add(32)) as u64)]); };
        }
        if (((((c.m.r32((l_def_3).wrapping_add(180)) as i32)) & (32i32))) != 0) {
            { let v_ = rt::fmul(F32(c.m.r32((p_w_2).wrapping_add(320))), F32(0x41f00000)); let a_ = (p_line_1).wrapping_add(224); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (p_line_1).wrapping_add(220); c.m.w32(a_, (v_).0); };
            if (((rt::fle(F32(c.m.r32((p_line_1).wrapping_add(224))), F32(0x00000000)) as i32)) != 0) {
                break 'f 0i32;
            }
            { let v_ = rt::fsub(F32(c.m.r32(((p_w_2).wrapping_add(272)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_w_2).wrapping_add(272)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((p_line_1).wrapping_add(208)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fsub(F32(c.m.r32(((p_w_2).wrapping_add(288)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_w_2).wrapping_add(288)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((p_line_1).wrapping_add(208)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fsub(F32(c.m.r32(((p_w_2).wrapping_add(304)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_w_2).wrapping_add(304)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((p_line_1).wrapping_add(208)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32(((p_w_2).wrapping_add(272)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_line_1).wrapping_add(196)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32(((p_w_2).wrapping_add(288)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_line_1).wrapping_add(196)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32(((p_w_2).wrapping_add(304)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_line_1).wrapping_add(196)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        }
        's1: {
            let sv11: i32 = ((c.m.r8((l_def_3).wrapping_add(169))) as i32);
            match sv11 {
                0i32 => {
                    { let v_ = rt::fmul(rt::fmul({ let a0_ = F32(c.m.r32((p_line_1).wrapping_add(156))); F::Port_cosf(c, a0_) }, F32(c.m.r32((p_line_1).wrapping_add(152)))), F32(c.m.r32(((p_w_2).wrapping_add(48)).wrapping_add(48)))); let a_ = (p_line_1).wrapping_add(48); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fmul(rt::fmul({ let a0_ = F32(c.m.r32((p_line_1).wrapping_add(156))); F::Port_sinf(c, a0_) }, F32(c.m.r32((p_line_1).wrapping_add(152)))), F32(c.m.r32(((p_w_2).wrapping_add(48)).wrapping_add(48)))); let a_ = ((p_line_1).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x00000000); let a_ = ((p_line_1).wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = ((p_line_1).wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    break 's1;
                }
                1i32 => {
                    { let v_ = { let a0_ = rt::fmul({ let a0_ = F32(c.m.r32((l_def_3).wrapping_add(144))); let a1_ = F32(c.m.r32((l_def_3).wrapping_add(148))); F::Rand_FloatRange(c, a0_, a1_) }, F32(0x40490fda)); F::EftMath_WrapAngle(c, a0_) }; let a_ = (p_line_1).wrapping_add(244); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fmul(rt::fmul({ let a0_ = F32(c.m.r32((p_line_1).wrapping_add(156))); F::Port_cosf(c, a0_) }, rt::fmul(F32(c.m.r32((p_line_1).wrapping_add(152))), { let a0_ = F32(c.m.r32((p_line_1).wrapping_add(244))); F::Port_cosf(c, a0_) })), F32(c.m.r32(((p_w_2).wrapping_add(48)).wrapping_add(48)))); let a_ = (p_line_1).wrapping_add(48); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fmul(rt::fmul({ let a0_ = F32(c.m.r32((p_line_1).wrapping_add(156))); F::Port_sinf(c, a0_) }, rt::fmul(F32(c.m.r32((p_line_1).wrapping_add(152))), { let a0_ = F32(c.m.r32((p_line_1).wrapping_add(244))); F::Port_cosf(c, a0_) })), F32(c.m.r32(((p_w_2).wrapping_add(48)).wrapping_add(48)))); let a_ = ((p_line_1).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fmul(rt::fmul({ let a0_ = F32(c.m.r32((p_line_1).wrapping_add(244))); F::Port_sinf(c, a0_) }, F32(c.m.r32((p_line_1).wrapping_add(152)))), F32(c.m.r32(((p_w_2).wrapping_add(48)).wrapping_add(48)))); let a_ = ((p_line_1).wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = ((p_line_1).wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    break 's1;
                }
                _ => {
                }
            }
        }
        { let _ = D::call(c, fid::Vec3_Normalize, &[(((p_line_1).wrapping_add(64)) as u64), (((p_line_1).wrapping_add(48)) as u64)]); };
        {
            l_i_5 = 0i32;
            'l2: loop {
                if !(((((l_i_5) < ((c.m.r32((p_line_1).wrapping_add(272)) as i32))) as i32)) != 0) { break 'l2; }
                'c3: {
                    if ((((!((F::EftZap_AddNode(c, p_line_1)) != 0)) as i32)) != 0) {
                        {
                            l_pt_7 = c.m.r32((p_line_1).wrapping_add(280));
                            'l4: loop {
                                if !(((((l_pt_7) != (((0i32) as u32))) as i32)) != 0) { break 'l4; }
                                'c5: {
                                    { let v_ = 0i32; let a_ = (l_pt_7).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                                    if (((((c.m.r32((p_line_1).wrapping_add(280))) != (((0i32) as u32))) as i32)) != 0) {
                                        l_prev__8 = c.m.r32((l_pt_7).wrapping_add(40));
                                        if (((((l_prev__8) == (((0i32) as u32))) as i32)) != 0) {
                                            l_next__9 = c.m.r32((l_pt_7).wrapping_add(36));
                                            if (((((l_next__9) == (((0i32) as u32))) as i32)) != 0) {
                                                { let v_ = ((0i32) as u32); let a_ = (p_line_1).wrapping_add(280); c.m.w32(a_, v_); };
                                                { let v_ = ((0i32) as u32); let a_ = (p_line_1).wrapping_add(284); c.m.w32(a_, v_); };
                                            } else {
                                                { let v_ = l_next__9; let a_ = (p_line_1).wrapping_add(280); c.m.w32(a_, v_); };
                                                { let v_ = ((0i32) as u32); let a_ = (l_next__9).wrapping_add(40); c.m.w32(a_, v_); };
                                            }
                                        } else {
                                            l_next__10 = c.m.r32((l_pt_7).wrapping_add(36));
                                            if (((((l_next__10) == (((0i32) as u32))) as i32)) != 0) {
                                                { let v_ = l_prev__8; let a_ = (p_line_1).wrapping_add(284); c.m.w32(a_, v_); };
                                                { let v_ = ((0i32) as u32); let a_ = (l_prev__8).wrapping_add(36); c.m.w32(a_, v_); };
                                            } else {
                                                { let v_ = l_prev__8; let a_ = (l_next__10).wrapping_add(40); c.m.w32(a_, v_); };
                                                { let v_ = c.m.r32((l_pt_7).wrapping_add(36)); let a_ = (c.m.r32((l_pt_7).wrapping_add(40))).wrapping_add(36); c.m.w32(a_, v_); };
                                            }
                                        }
                                    }
                                }
                                l_pt_7 = c.m.r32((l_pt_7).wrapping_add(36));
                            }
                        }
                        break 'f 0i32;
                    }
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftZap_InitLine", &PROBE_N, &[], true); }
    r_
}

pub fn EftZap_UpdateLines(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftZap_UpdateLines", &PROBE_N, &[(p_w_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_arg_2: u32 = 0;
    let mut l_def_3: u32 = 0;
    let mut l_t_4: F32 = F32(0);
    let mut l_pitch_5: F32 = F32(0);
    let mut l_fade_6: F32 = F32(0);
    let mut l_yaw_7: F32 = F32(0);
    let mut l_line_8: u32 = 0;
    let mut l_pt_9: u32 = 0;
    let mut l_a_10: F32 = F32(0);
    let mut l_b_11: F32 = F32(0);
    let mut l_c_12: F32 = F32(0);
    let mut l_k_13: F32 = F32(0);
    let mut l_d_14: F32 = F32(0);
    let mut l_prev__15: u32 = 0;
    let mut l_next__16: u32 = 0;
    let mut l_next__17: u32 = 0;
    let mut l_prev__18: u32 = 0;
    let mut l_next__19: u32 = 0;
    let mut l_next__20: u32 = 0;
    let r_ = 'f: {
        l_arg_2 = (p_w_1).wrapping_add(48);
        l_def_3 = c.m.r32(l_arg_2);
        l_t_4 = F32(0x00000000);
        rt::memset(c, fp, 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(16), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(32), 0, 12);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(32).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(32).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(32).wrapping_add(8), (v_).0); }
        l_pitch_5 = F32(0x00000000);
        l_fade_6 = F32(0x3f800000);
        l_yaw_7 = F32(0x00000000);
        's1: {
            let sv21: i32 = ((c.m.r8((l_def_3).wrapping_add(169))) as i32);
            match sv21 {
                0i32 => {
                    l_pitch_5 = { let a0_ = { let a0_ = rt::fneg(F32(c.m.r32(((l_arg_2).wrapping_add(16)).wrapping_add(4)))); F::Mathf_Asin(c, a0_) }; F::EftMath_WrapAngle(c, a0_) };
                    l_yaw_7 = { let a0_ = { let a0_ = F32(c.m.r32((l_arg_2).wrapping_add(16))); let a1_ = F32(c.m.r32(((l_arg_2).wrapping_add(16)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) }; F::EftMath_WrapAngle(c, a0_) };
                    break 's1;
                }
                1i32 => {
                    l_pitch_5 = { let a0_ = rt::fadd({ let a0_ = rt::fneg(F32(c.m.r32(((l_arg_2).wrapping_add(16)).wrapping_add(4)))); F::Mathf_Asin(c, a0_) }, rt::fmul(F32(c.m.r32(((l_def_3).wrapping_add(132)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0x40490fda))); F::EftMath_WrapAngle(c, a0_) };
                    l_yaw_7 = { let a0_ = rt::fadd({ let a0_ = F32(c.m.r32((l_arg_2).wrapping_add(16))); let a1_ = F32(c.m.r32(((l_arg_2).wrapping_add(16)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) }, rt::fmul(F32(c.m.r32(((l_def_3).wrapping_add(132)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(0x40490fda))); F::EftMath_WrapAngle(c, a0_) };
                    break 's1;
                }
                _ => {
                }
            }
        }
        {
            l_line_8 = c.m.r32((p_w_1).wrapping_add(384));
            'l2: loop {
                if !(((((l_line_8) != (((0i32) as u32))) as i32)) != 0) { break 'l2; }
                'c3: {
                    if (((rt::fgt(F32(c.m.r32((l_line_8).wrapping_add(260))), F32(0x00000000)) as i32)) != 0) {
                        { let r_ = F32(0x3f800000); let a_ = (l_line_8).wrapping_add(260); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        break 'c3;
                    }
                    l_t_4 = (if (((rt::flt(rt::fdiv(F32(c.m.r32((l_line_8).wrapping_add(268))), F32(c.m.r32((l_line_8).wrapping_add(264)))), F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(0x3f800000), rt::fdiv(F32(c.m.r32((l_line_8).wrapping_add(268))), F32(c.m.r32((l_line_8).wrapping_add(264))))) as i32)) != 0) { F32(0x3f800000) } else { rt::fdiv(F32(c.m.r32((l_line_8).wrapping_add(268))), F32(c.m.r32((l_line_8).wrapping_add(264)))) }) });
                    if (((rt::flt(l_t_4, { let a_ = (c.m.r32(l_arg_2)).wrapping_add(24); F32(c.m.r32(a_)) }) as i32)) != 0) {
                        if (((((c.m.r32((l_line_8).wrapping_add(276)) as i32)) & (4096i32))) != 0) {
                            { let r_ = F32(c.m.r32(((l_line_8).wrapping_add(172)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (l_line_8).wrapping_add(148); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        }
                        if (((((c.m.r32((l_line_8).wrapping_add(276)) as i32)) & (2048i32))) != 0) {
                            { let r_ = F32(c.m.r32(((l_line_8).wrapping_add(180)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (l_line_8).wrapping_add(152); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        }
                        if (((((c.m.r32((l_line_8).wrapping_add(276)) as i32)) & (512i32))) != 0) {
                            { let r_ = F32(c.m.r32(((l_line_8).wrapping_add(188)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (l_line_8).wrapping_add(160); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        }
                        if (((((c.m.r32((l_line_8).wrapping_add(276)) as i32)) & (1024i32))) != 0) {
                            { let r_ = F32(c.m.r32(((l_line_8).wrapping_add(164)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (l_line_8).wrapping_add(144); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        }
                    } else {
                        if (((((c.m.r32((l_line_8).wrapping_add(276)) as i32)) & (4096i32))) != 0) {
                            { let r_ = F32(c.m.r32(((l_line_8).wrapping_add(172)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (l_line_8).wrapping_add(148); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        }
                        if (((((c.m.r32((l_line_8).wrapping_add(276)) as i32)) & (2048i32))) != 0) {
                            { let r_ = F32(c.m.r32(((l_line_8).wrapping_add(180)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (l_line_8).wrapping_add(152); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        }
                        if (((((c.m.r32((l_line_8).wrapping_add(276)) as i32)) & (512i32))) != 0) {
                            { let r_ = F32(c.m.r32(((l_line_8).wrapping_add(188)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (l_line_8).wrapping_add(160); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        }
                        if (((((c.m.r32((l_line_8).wrapping_add(276)) as i32)) & (1024i32))) != 0) {
                            { let r_ = F32(c.m.r32(((l_line_8).wrapping_add(164)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (l_line_8).wrapping_add(144); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        }
                    }
                    if (((((c.m.r32((l_line_8).wrapping_add(276)) as i32)) & (8192i32))) != 0) {
                        l_d_14 = rt::fneg(F32(c.m.r32((l_line_8).wrapping_add(148))));
                        { let r_ = l_d_14; let a_ = (l_line_8).wrapping_add(156); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    } else {
                        { let r_ = F32(c.m.r32((l_line_8).wrapping_add(148))); let a_ = (l_line_8).wrapping_add(156); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    }
                    { let v_ = { let a0_ = F32(c.m.r32((l_line_8).wrapping_add(156))); F::EftMath_WrapAngle(c, a0_) }; let a_ = (l_line_8).wrapping_add(156); c.m.w32(a_, (v_).0); };
                    's4: {
                        let sv22: i32 = ((c.m.r8((l_def_3).wrapping_add(169))) as i32);
                        match sv22 {
                            0i32 => {
                                { let v_ = rt::fmul(rt::fmul({ let a0_ = F32(c.m.r32((l_line_8).wrapping_add(156))); F::Port_cosf(c, a0_) }, F32(c.m.r32((l_line_8).wrapping_add(152)))), F32(c.m.r32((l_arg_2).wrapping_add(48)))); let a_ = (l_line_8).wrapping_add(48); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fmul(rt::fmul({ let a0_ = F32(c.m.r32((l_line_8).wrapping_add(156))); F::Port_sinf(c, a0_) }, F32(c.m.r32((l_line_8).wrapping_add(152)))), F32(c.m.r32((l_arg_2).wrapping_add(48)))); let a_ = ((l_line_8).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let r_ = F32(c.m.r32((l_line_8).wrapping_add(160))); let a_ = ((l_line_8).wrapping_add(48)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                { let v_ = F32(0x3f800000); let a_ = ((l_line_8).wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                break 's4;
                            }
                            1i32 => {
                                { let v_ = rt::fmul(rt::fmul({ let a0_ = F32(c.m.r32((l_line_8).wrapping_add(156))); F::Port_cosf(c, a0_) }, rt::fmul(F32(c.m.r32((l_line_8).wrapping_add(152))), { let a0_ = F32(c.m.r32((l_line_8).wrapping_add(244))); F::Port_cosf(c, a0_) })), F32(c.m.r32((l_arg_2).wrapping_add(48)))); let a_ = (l_line_8).wrapping_add(48); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fmul(rt::fmul({ let a0_ = F32(c.m.r32((l_line_8).wrapping_add(156))); F::Port_sinf(c, a0_) }, rt::fmul(F32(c.m.r32((l_line_8).wrapping_add(152))), { let a0_ = F32(c.m.r32((l_line_8).wrapping_add(244))); F::Port_cosf(c, a0_) })), F32(c.m.r32((l_arg_2).wrapping_add(48)))); let a_ = ((l_line_8).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fmul(rt::fmul({ let a0_ = F32(c.m.r32((l_line_8).wrapping_add(244))); F::Port_sinf(c, a0_) }, F32(c.m.r32((l_line_8).wrapping_add(152)))), F32(c.m.r32((l_arg_2).wrapping_add(48)))); let a_ = ((l_line_8).wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = ((l_line_8).wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                break 's4;
                            }
                            _ => {
                            }
                        }
                    }
                    { let _ = D::call(c, fid::Vec4_Copy, &[((fp.wrapping_add(16)) as u64), (((l_line_8).wrapping_add(48)) as u64)]); };
                    { let a1_ = (c.m.r32(sym::gEftZapMgr)).wrapping_add(7776); F::Mtx_RotateX(c, (l_line_8).wrapping_add(80), a1_, l_pitch_5) };
                    F::Mtx_RotateY(c, (l_line_8).wrapping_add(80), (l_line_8).wrapping_add(80), l_yaw_7);
                    F::Mtx_Translate(c, (l_line_8).wrapping_add(80), (l_line_8).wrapping_add(80), (p_w_1).wrapping_add(128));
                    l_k_13 = F32(0x00000000);
                    if (((((c.m.r32((l_def_3).wrapping_add(180)) as i32)) & (32i32))) != 0) {
                        l_k_13 = (if (((rt::flt(rt::fdiv(F32(c.m.r32((l_line_8).wrapping_add(220))), F32(c.m.r32((l_line_8).wrapping_add(224)))), F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(0x3f800000), rt::fdiv(F32(c.m.r32((l_line_8).wrapping_add(220))), F32(c.m.r32((l_line_8).wrapping_add(224))))) as i32)) != 0) { F32(0x3f800000) } else { rt::fdiv(F32(c.m.r32((l_line_8).wrapping_add(220))), F32(c.m.r32((l_line_8).wrapping_add(224)))) }) });
                        { let v_ = rt::fadd(F32(c.m.r32(((l_line_8).wrapping_add(196)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_line_8).wrapping_add(208)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_k_13)); let a_ = (fp.wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(F32(c.m.r32(((l_line_8).wrapping_add(196)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_line_8).wrapping_add(208)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_k_13)); let a_ = (fp.wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(F32(c.m.r32(((l_line_8).wrapping_add(196)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_line_8).wrapping_add(208)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_k_13)); let a_ = (fp.wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                        if ((((!(((((c.m.r32((l_line_8).wrapping_add(276)) as i32)) & (65536i32))) != 0)) as i32)) != 0) {
                            { let r_ = F32(0x3f800000); let a_ = (l_line_8).wrapping_add(220); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        } else {
                            { let r_ = F32(0xbf800000); let a_ = (l_line_8).wrapping_add(220); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        }
                        if (((((((rt::flt(F32(c.m.r32((l_line_8).wrapping_add(220))), F32(0x00000000)) as i32)) != 0) || (((rt::fge(F32(c.m.r32((l_line_8).wrapping_add(220))), F32(c.m.r32((l_line_8).wrapping_add(224)))) as i32)) != 0)) as i32)) != 0) {
                            { let r_ = 65536i32; let a_ = (l_line_8).wrapping_add(276); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                            { let v_ = (if (((rt::flt(F32(c.m.r32((l_line_8).wrapping_add(220))), F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(c.m.r32((l_line_8).wrapping_add(224))), F32(c.m.r32((l_line_8).wrapping_add(220)))) as i32)) != 0) { F32(c.m.r32((l_line_8).wrapping_add(224))) } else { F32(c.m.r32((l_line_8).wrapping_add(220))) }) }); let a_ = (l_line_8).wrapping_add(220); c.m.w32(a_, (v_).0); };
                        }
                    } else {
                        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    }
                    l_b_11 = F32(0x3f800000);
                    l_a_10 = F32(0x00000000);
                    if (((rt::flt(l_t_4, { let a_ = (c.m.r32(l_arg_2)).wrapping_add(16); F32(c.m.r32(a_)) }) as i32)) != 0) {
                        if (((rt::fgt(F32(c.m.r32((l_line_8).wrapping_add(248))), F32(0x00000000)) as i32)) != 0) {
                            l_a_10 = rt::fdiv(F32(c.m.r32((l_line_8).wrapping_add(268))), F32(c.m.r32((l_line_8).wrapping_add(248))));
                        } else {
                            l_a_10 = l_b_11;
                        }
                    } else {
                        l_a_10 = l_b_11;
                    }
                    if (((rt::fle({ let a_ = (c.m.r32(l_arg_2)).wrapping_add(20); F32(c.m.r32(a_)) }, l_t_4) as i32)) != 0) {
                        if (((rt::fgt(F32(c.m.r32((l_line_8).wrapping_add(252))), F32(0x00000000)) as i32)) != 0) {
                            l_a_10 = rt::fsub(F32(0x3f800000), rt::fdiv(rt::fsub(F32(c.m.r32((l_line_8).wrapping_add(268))), rt::fsub(F32(c.m.r32((l_line_8).wrapping_add(264))), F32(c.m.r32((l_line_8).wrapping_add(252))))), F32(c.m.r32((l_line_8).wrapping_add(252)))));
                        } else {
                            l_a_10 = F32(0x00000000);
                        }
                    }
                    if (((((c.m.r32((p_w_1).wrapping_add(380)) as i32)) & (32i32))) != 0) {
                        l_b_11 = (if (((rt::flt(rt::fdiv(F32(c.m.r32((l_line_8).wrapping_add(256))), F32(c.m.r32((l_line_8).wrapping_add(252)))), F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(0x3f800000), rt::fdiv(F32(c.m.r32((l_line_8).wrapping_add(256))), F32(c.m.r32((l_line_8).wrapping_add(252))))) as i32)) != 0) { F32(0x3f800000) } else { rt::fdiv(F32(c.m.r32((l_line_8).wrapping_add(256))), F32(c.m.r32((l_line_8).wrapping_add(252)))) }) });
                        { let r_ = F32(0x3f800000); let a_ = (l_line_8).wrapping_add(256); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        if (((rt::fle(F32(c.m.r32((l_line_8).wrapping_add(256))), F32(0x00000000)) as i32)) != 0) {
                            { let r_ = 8i32; let a_ = (l_line_8).wrapping_add(276); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                    }
                    if (((rt::fgt(F32(c.m.r32((l_line_8).wrapping_add(232))), F32(0x00000000)) as i32)) != 0) {
                        l_c_12 = (if (((rt::flt(rt::fdiv(F32(c.m.r32((l_line_8).wrapping_add(228))), F32(c.m.r32((l_line_8).wrapping_add(232)))), F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(0x3f800000), rt::fdiv(F32(c.m.r32((l_line_8).wrapping_add(228))), F32(c.m.r32((l_line_8).wrapping_add(232))))) as i32)) != 0) { F32(0x3f800000) } else { rt::fdiv(F32(c.m.r32((l_line_8).wrapping_add(228))), F32(c.m.r32((l_line_8).wrapping_add(232)))) }) });
                    } else {
                        l_c_12 = F32(0x3f800000);
                    }
                    if (((((c.m.r32((l_line_8).wrapping_add(276)) as i32)) & (256i32))) != 0) {
                        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(l_line_8)), rt::fmul(F32(c.m.r32((l_line_8).wrapping_add(16))), l_c_12)), F32(c.m.r32((fp.wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (l_line_8).wrapping_add(32); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32((l_line_8).wrapping_add(4))), rt::fmul(F32(c.m.r32(((l_line_8).wrapping_add(16)).wrapping_add(4))), l_c_12)), F32(c.m.r32((fp.wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = ((l_line_8).wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32((l_line_8).wrapping_add(8))), rt::fmul(F32(c.m.r32(((l_line_8).wrapping_add(16)).wrapping_add(8))), l_c_12)), F32(c.m.r32((fp.wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let a_ = ((l_line_8).wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    } else {
                        { let v_ = rt::fmul(rt::fmul(F32(c.m.r32(l_line_8)), l_c_12), F32(c.m.r32((fp.wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (l_line_8).wrapping_add(32); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fmul(rt::fmul(F32(c.m.r32((l_line_8).wrapping_add(4))), l_c_12), F32(c.m.r32((fp.wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = ((l_line_8).wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fmul(rt::fmul(F32(c.m.r32((l_line_8).wrapping_add(8))), l_c_12), F32(c.m.r32((fp.wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let a_ = ((l_line_8).wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    }
                    if (((((((rt::fle(F32(c.m.r32((l_line_8).wrapping_add(236))), l_t_4) as i32)) != 0) && (((rt::flt(l_t_4, F32(c.m.r32((l_line_8).wrapping_add(240)))) as i32)) != 0)) as i32)) != 0) {
                        { let r_ = F32(0x3f800000); let a_ = (l_line_8).wrapping_add(228); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    }
                    { let v_ = rt::fmul(rt::fmul(rt::fmul(F32(c.m.r32((l_line_8).wrapping_add(12))), l_a_10), l_b_11), l_fade_6); let a_ = ((l_line_8).wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    if (((((((rt::fle(F32(c.m.r32(((l_line_8).wrapping_add(32)).wrapping_add(12))), F32(0x00000000)) as i32)) != 0) && (((((c.m.r32((p_w_1).wrapping_add(380)) as i32)) & (32i32))) != 0)) as i32)) != 0) {
                        { let r_ = 8i32; let a_ = (l_line_8).wrapping_add(276); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    if (((((((rt::fgt(F32(c.m.r32(((l_line_8).wrapping_add(32)).wrapping_add(12))), F32(0x00000000)) as i32)) != 0) && (((rt::fgt(F32(c.m.r32((l_line_8).wrapping_add(144))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                        { let r_ = 128i32; let a_ = (l_line_8).wrapping_add(276); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    } else {
                        { let r_ = (!(128i32)); let a_ = (l_line_8).wrapping_add(276); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    {
                        l_pt_9 = c.m.r32((l_line_8).wrapping_add(280));
                        'l5: loop {
                            if !(((((l_pt_9) != (((0i32) as u32))) as i32)) != 0) { break 'l5; }
                            'c6: {
                                { let _ = D::call(c, fid::Vec4_Copy, &[((fp) as u64), ((l_pt_9) as u64)]); };
                                { let _ = D::call(c, fid::Vec4_Copy, &[((l_pt_9) as u64), ((fp.wrapping_add(16)) as u64)]); };
                                { let _ = D::call(c, fid::Vec4_Copy, &[((fp.wrapping_add(16)) as u64), ((fp) as u64)]); };
                            }
                            l_pt_9 = c.m.r32((l_pt_9).wrapping_add(36));
                        }
                    }
                    { let r_ = F32(0x3f800000); let a_ = (l_line_8).wrapping_add(268); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if (((((((rt::fle(F32(c.m.r32((l_line_8).wrapping_add(264))), F32(c.m.r32((l_line_8).wrapping_add(268)))) as i32)) != 0) || (((((c.m.r32((l_line_8).wrapping_add(276)) as i32)) & (8i32))) != 0)) as i32)) != 0) {
                        {
                            l_pt_9 = c.m.r32((l_line_8).wrapping_add(280));
                            'l7: loop {
                                if !(((((l_pt_9) != (((0i32) as u32))) as i32)) != 0) { break 'l7; }
                                'c8: {
                                    { let v_ = 0i32; let a_ = (l_pt_9).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                                    if (((((c.m.r32((l_line_8).wrapping_add(280))) != (((0i32) as u32))) as i32)) != 0) {
                                        l_prev__15 = c.m.r32((l_pt_9).wrapping_add(40));
                                        if (((((l_prev__15) == (((0i32) as u32))) as i32)) != 0) {
                                            l_next__16 = c.m.r32((l_pt_9).wrapping_add(36));
                                            if (((((l_next__16) == (((0i32) as u32))) as i32)) != 0) {
                                                { let v_ = ((0i32) as u32); let a_ = (l_line_8).wrapping_add(280); c.m.w32(a_, v_); };
                                                { let v_ = ((0i32) as u32); let a_ = (l_line_8).wrapping_add(284); c.m.w32(a_, v_); };
                                            } else {
                                                { let v_ = l_next__16; let a_ = (l_line_8).wrapping_add(280); c.m.w32(a_, v_); };
                                                { let v_ = ((0i32) as u32); let a_ = (l_next__16).wrapping_add(40); c.m.w32(a_, v_); };
                                            }
                                        } else {
                                            l_next__17 = c.m.r32((l_pt_9).wrapping_add(36));
                                            if (((((l_next__17) == (((0i32) as u32))) as i32)) != 0) {
                                                { let v_ = l_prev__15; let a_ = (l_line_8).wrapping_add(284); c.m.w32(a_, v_); };
                                                { let v_ = ((0i32) as u32); let a_ = (l_prev__15).wrapping_add(36); c.m.w32(a_, v_); };
                                            } else {
                                                { let v_ = l_prev__15; let a_ = (l_next__17).wrapping_add(40); c.m.w32(a_, v_); };
                                                { let v_ = c.m.r32((l_pt_9).wrapping_add(36)); let a_ = (c.m.r32((l_pt_9).wrapping_add(40))).wrapping_add(36); c.m.w32(a_, v_); };
                                            }
                                        }
                                    }
                                }
                                l_pt_9 = c.m.r32((l_pt_9).wrapping_add(36));
                            }
                        }
                        { let v_ = 0i32; let a_ = (l_line_8).wrapping_add(276); c.m.w32(a_, (v_) as u32); };
                        if (((((c.m.r32((p_w_1).wrapping_add(384))) != (((0i32) as u32))) as i32)) != 0) {
                            l_prev__18 = c.m.r32((l_line_8).wrapping_add(292));
                            if (((((l_prev__18) == (((0i32) as u32))) as i32)) != 0) {
                                l_next__19 = c.m.r32((l_line_8).wrapping_add(288));
                                if (((((l_next__19) == (((0i32) as u32))) as i32)) != 0) {
                                    { let v_ = ((0i32) as u32); let a_ = (p_w_1).wrapping_add(384); c.m.w32(a_, v_); };
                                    { let v_ = ((0i32) as u32); let a_ = (p_w_1).wrapping_add(388); c.m.w32(a_, v_); };
                                } else {
                                    { let v_ = l_next__19; let a_ = (p_w_1).wrapping_add(384); c.m.w32(a_, v_); };
                                    { let v_ = ((0i32) as u32); let a_ = (l_next__19).wrapping_add(292); c.m.w32(a_, v_); };
                                }
                            } else {
                                l_next__20 = c.m.r32((l_line_8).wrapping_add(288));
                                if (((((l_next__20) == (((0i32) as u32))) as i32)) != 0) {
                                    { let v_ = l_prev__18; let a_ = (p_w_1).wrapping_add(388); c.m.w32(a_, v_); };
                                    { let v_ = ((0i32) as u32); let a_ = (l_prev__18).wrapping_add(288); c.m.w32(a_, v_); };
                                } else {
                                    { let v_ = l_prev__18; let a_ = (l_next__20).wrapping_add(292); c.m.w32(a_, v_); };
                                    { let v_ = c.m.r32((l_line_8).wrapping_add(288)); let a_ = (c.m.r32((l_line_8).wrapping_add(292))).wrapping_add(288); c.m.w32(a_, v_); };
                                }
                            }
                        }
                    }
                }
                l_line_8 = c.m.r32((l_line_8).wrapping_add(288));
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftZap_UpdateLines", &PROBE_N, &[], true); }
    r_
}

pub fn EftZap_DrawQuad(c: &mut Ctx, mut p_pos_1: u32, p_color_2: u32, mut p_u0_3: F32, mut p_v0_4: F32, mut p_u1_5: F32, mut p_v1_6: F32, mut p_u2_7: F32, mut p_v2_8: F32, mut p_u3_9: F32, mut p_v3_10: F32, mut p_layer_11: i32, mut p_texIdx_12: i32, mut p_front_13: i32, mut p_tex_14: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftZap_DrawQuad", &PROBE_N, &[(p_pos_1) as u32, (p_color_2) as u32, (p_u0_3).0, (p_v0_4).0, (p_u1_5).0, (p_v1_6).0, (p_u2_7).0, (p_v2_8).0, (p_u3_9).0, (p_v3_10).0, (p_layer_11) as u32, (p_texIdx_12) as u32, (p_front_13) as u32, (p_tex_14) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(224) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 224);
    let mut l_p_15: u32 = 0;
    let mut l_e_16: u32 = 0;
    let mut l_abe_17: i32 = 0;
    let mut l_ctx_18: i32 = 0;
    let mut l_l_19: i32 = 0;
    let mut l_z_20: i32 = 0;
    let mut l_t_21: u32 = 0;
    rt::memcpy(c, fp, p_color_2, 16);
    let r_ = 'f: {
        l_abe_17 = 1i32;
        F::Vec4_Set(c, (fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), p_u0_3, p_v0_4, F32(0x3f800000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), p_u1_5, p_v1_6, F32(0x3f800000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), p_u2_7, p_v2_8, F32(0x3f800000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), p_u3_9, p_v3_10, F32(0x3f800000), F32(0x3f800000));
        if ((((!((F::Vu0Cur_ProjectPointsStq(c, fp.wrapping_add(144), fp.wrapping_add(80), p_pos_1, fp.wrapping_add(16), 4i32)) != 0)) as i32)) != 0) {
            break 'f;
        }
        l_p_15 = c.m.r32(sym::gOtCur);
        { let v_ = (l_p_15).wrapping_add((((1i32) as u32)).wrapping_mul(144)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
        if (((((l_p_15) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        l_ctx_18 = (((p_layer_11) >= (2i32)) as i32);
        { let v_ = (((((((l_abe_17) as u64)).wrapping_shl((6i32) as u32)) | ((((l_ctx_18) as u64)).wrapping_shl((9i32) as u32)))) | (((28i32) as u64))); let a_ = (l_p_15).wrapping_add(32); c.m.w64(a_, v_); };
        { let v_ = ((536870920i32) as u32); let a_ = l_p_15; c.m.w32(a_, v_); };
        { let v_ = ((268435456i32) as u32); let a_ = (l_p_15).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = ((1342177288i32) as u32); let a_ = (l_p_15).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = 16429131440647602177u64; let a_ = (l_p_15).wrapping_add(16); c.m.w64(a_, v_); };
        { let v_ = (((18599481537995104i64).wrapping_add((((l_ctx_18).wrapping_shl((4i32) as u32)) as i64))) as u64); let a_ = (l_p_15).wrapping_add(24); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (l_p_15).wrapping_add(4); c.m.w32(a_, v_); };
        l_z_20 = (((((c.m.r32(((fp.wrapping_add(144)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)).wrapping_add((c.m.r32(((fp.wrapping_add(144)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add((c.m.r32(((fp.wrapping_add(144)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add((c.m.r32(((fp.wrapping_add(144)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_shr((10i32) as u32);
        if ((p_front_13) != 0) {
            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(144)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(144)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(144)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(144)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        }
        F::Vec4_ToInt(c, fp.wrapping_add(208), fp);
        l_t_21 = (p_tex_14).wrapping_add((((p_texIdx_12).wrapping_shl((4i32) as u32)) as u32));
        l_l_19 = p_layer_11;
        if (((((l_l_19) >= (2i32)) as i32)) != 0) {
            { let r_ = 2i32; let n_ = (l_l_19).wrapping_sub(r_); l_l_19 = n_; };
        }
        { let v_ = (((c.m.r32(fp.wrapping_add(208)) as i32)) as u8); let a_ = ((l_p_15).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp.wrapping_add(208)).wrapping_add(4)) as i32)) as u8); let a_ = (((l_p_15).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp.wrapping_add(208)).wrapping_add(8)) as i32)) as u8); let a_ = (((l_p_15).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp.wrapping_add(208)).wrapping_add(12)) as i32)) as u8); let a_ = (((l_p_15).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_p_15).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = (((c.m.r32(fp.wrapping_add(208)) as i32)) as u8); let a_ = ((l_p_15).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp.wrapping_add(208)).wrapping_add(4)) as i32)) as u8); let a_ = (((l_p_15).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp.wrapping_add(208)).wrapping_add(8)) as i32)) as u8); let a_ = (((l_p_15).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp.wrapping_add(208)).wrapping_add(12)) as i32)) as u8); let a_ = (((l_p_15).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(80)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_p_15).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = (((c.m.r32(fp.wrapping_add(208)) as i32)) as u8); let a_ = ((l_p_15).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp.wrapping_add(208)).wrapping_add(4)) as i32)) as u8); let a_ = (((l_p_15).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp.wrapping_add(208)).wrapping_add(8)) as i32)) as u8); let a_ = (((l_p_15).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp.wrapping_add(208)).wrapping_add(12)) as i32)) as u8); let a_ = (((l_p_15).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(80)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_p_15).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = (((c.m.r32(fp.wrapping_add(208)) as i32)) as u8); let a_ = ((l_p_15).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp.wrapping_add(208)).wrapping_add(4)) as i32)) as u8); let a_ = (((l_p_15).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp.wrapping_add(208)).wrapping_add(8)) as i32)) as u8); let a_ = (((l_p_15).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp.wrapping_add(208)).wrapping_add(12)) as i32)) as u8); let a_ = (((l_p_15).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(80)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_p_15).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(16)))); let a_ = (((l_p_15).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = ((((l_p_15).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(80)).wrapping_add((((1i32) as u32)).wrapping_mul(16)))); let a_ = (((l_p_15).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(80)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = ((((l_p_15).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(80)).wrapping_add((((2i32) as u32)).wrapping_mul(16)))); let a_ = (((l_p_15).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(80)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = ((((l_p_15).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(80)).wrapping_add((((3i32) as u32)).wrapping_mul(16)))); let a_ = (((l_p_15).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(80)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = ((((l_p_15).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = (((c.m.r32((fp.wrapping_add(144)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_15).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(144)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_15).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(144)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_15).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_15).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32((fp.wrapping_add(144)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_15).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(144)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_15).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(144)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_15).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_15).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32((fp.wrapping_add(144)).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_15).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(144)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_15).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(144)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_15).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_15).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32((fp.wrapping_add(144)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_15).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(144)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_15).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(144)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_15).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_15).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = c.m.r64(l_t_21); let a_ = (l_p_15).wrapping_add(40); c.m.w64(a_, v_); };
        if (((((l_z_20) < (0i32)) as i32)) != 0) {
            l_e_16 = ((c.m.r32(sym::gOtZ)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_19) as u32)).wrapping_mul(8));
        } else {
            if (((((l_z_20) >= (4096i32)) as i32)) != 0) {
                l_e_16 = ((c.m.r32(sym::gOtZ)).wrapping_add((((4095i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_19) as u32)).wrapping_mul(8));
            } else {
                l_e_16 = ((c.m.r32(sym::gOtZ)).wrapping_add((((l_z_20) as u32)).wrapping_mul(16))).wrapping_add((((l_l_19) as u32)).wrapping_mul(8));
            }
        }
        { let v_ = l_p_15; let a_ = (c.m.r32((l_e_16).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = l_p_15; let a_ = (l_e_16).wrapping_add(4); c.m.w32(a_, v_); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftZap_DrawQuad", &PROBE_N, &[], true); }
    r_
}
