#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EftZap_LoadKey(c: &mut Ctx, mut p_w_1: u32, mut p_idx_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftZap_LoadKey", &PROBE_N, &[(p_w_1) as u32, (p_idx_2) as u32], false); }
    let mut l_keys_3: u32 = 0;
    let r_ = 'f: {
    l_keys_3 = c.m.r32(((p_w_1).wrapping_add(48)).wrapping_add(4));
    F::Vec4_Copy(c, (p_w_1).wrapping_add(144), (l_keys_3).wrapping_add((((p_idx_2) as u32)).wrapping_mul(16)));
    F::Vec4_Copy(c, (p_w_1).wrapping_add(176), ((l_keys_3).wrapping_add(48)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(16)));
    F::Vec4_Copy(c, (p_w_1).wrapping_add(208), ((l_keys_3).wrapping_add(96)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(16)));
    F::Vec4_Copy(c, (p_w_1).wrapping_add(240), ((l_keys_3).wrapping_add(144)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(16)));
    { let v_ = F32(c.m.r32((((l_keys_3).wrapping_add(192)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_w_1).wrapping_add(272)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_keys_3).wrapping_add(192)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((p_w_1).wrapping_add(272)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_keys_3).wrapping_add(216)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_w_1).wrapping_add(288)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_keys_3).wrapping_add(216)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((p_w_1).wrapping_add(288)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_keys_3).wrapping_add(240)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_w_1).wrapping_add(304)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_keys_3).wrapping_add(240)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((p_w_1).wrapping_add(304)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_keys_3).wrapping_add(264)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(4)))); let a_ = (p_w_1).wrapping_add(320); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_keys_3).wrapping_add(276)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(4)))); let a_ = (p_w_1).wrapping_add(328); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_keys_3).wrapping_add(288)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(4)))); let a_ = (p_w_1).wrapping_add(336); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftZap_LoadKey", &PROBE_N, &[], true); }
    r_
}

pub fn EftZap_AddNode(c: &mut Ctx, mut p_chain_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftZap_AddNode", &PROBE_N, &[(p_chain_1) as u32], false); }
    let mut l_i_2: u8 = 0;
    let mut l_node_3: u32 = 0;
    let r_ = 'f: {
    if ((((((({ let a_ = (c.m.r32(sym::gEftZapMgr)).wrapping_add(7841); c.m.r8(a_) }) as i32)) >= (35i32)) as i32)) != 0) {
        { let v_ = ((0i32) as u8); let a_ = (c.m.r32(sym::gEftZapMgr)).wrapping_add(7841); c.m.w8(a_, v_); };
    }
    l_i_2 = { let a_ = (c.m.r32(sym::gEftZapMgr)).wrapping_add(7841); c.m.r8(a_) };
    'l1: loop {
        'c2: {
            l_node_3 = ((c.m.r32(sym::gEftZapMgr)).wrapping_add(6096)).wrapping_add((((l_i_2) as u32)).wrapping_mul(48));
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            if (((((((l_i_2) as i32)) >= (35i32)) as i32)) != 0) {
                l_i_2 = ((0i32) as u8);
            }
            if ((((((c.m.r32((l_node_3).wrapping_add(32)) as i32)) == (0i32)) as i32)) != 0) {
                F::Vec4_Copy(c, l_node_3, (p_chain_1).wrapping_add(48));
                { let v_ = ((0i32) as u32); let a_ = (l_node_3).wrapping_add(36); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_node_3).wrapping_add(40); c.m.w32(a_, v_); };
                { let r_ = 1i32; let a_ = (l_node_3).wrapping_add(32); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                if (((((c.m.r32((p_chain_1).wrapping_add(280))) == (((0i32) as u32))) as i32)) != 0) {
                    { let v_ = l_node_3; let a_ = (p_chain_1).wrapping_add(280); c.m.w32(a_, v_); };
                    { let v_ = l_node_3; let a_ = (p_chain_1).wrapping_add(284); c.m.w32(a_, v_); };
                } else {
                    { let v_ = c.m.r32((p_chain_1).wrapping_add(284)); let a_ = (l_node_3).wrapping_add(40); c.m.w32(a_, v_); };
                    { let v_ = l_node_3; let a_ = (c.m.r32((p_chain_1).wrapping_add(284))).wrapping_add(36); c.m.w32(a_, v_); };
                    { let v_ = l_node_3; let a_ = (p_chain_1).wrapping_add(284); c.m.w32(a_, v_); };
                }
                { let v_ = l_i_2; let a_ = (c.m.r32(sym::gEftZapMgr)).wrapping_add(7841); c.m.w8(a_, v_); };
                break 'f 1i32;
            }
        }
        if !(((((((l_i_2) as i32)) != ((({ let a_ = (c.m.r32(sym::gEftZapMgr)).wrapping_add(7841); c.m.r8(a_) }) as i32))) as i32)) != 0) { break 'l1; }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftZap_AddNode", &PROBE_N, &[], true); }
    r_
}

pub fn EftZap_SetTexPair(c: &mut Ctx, mut p_w_1: u32, mut p_tbl_2: u32, mut p_a_3: i32, mut p_b_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftZap_SetTexPair", &PROBE_N, &[(p_w_1) as u32, (p_tbl_2) as u32, (p_a_3) as u32, (p_b_4) as u32], false); }
    let r_ = 'f: {
    if (((((p_a_3) == (p_b_4)) as i32)) != 0) {
        { let v_ = ((p_b_4) as u8); let a_ = (p_w_1).wrapping_add(376); c.m.w8(a_, v_); };
    } else {
        { let v_ = (((p_a_3).wrapping_add(p_b_4)) as u8); let a_ = (p_w_1).wrapping_add(376); c.m.w8(a_, v_); };
    }
    { let s_ = (p_tbl_2).wrapping_add((((p_a_3) as u32)).wrapping_mul(16)); let d_ = ((p_w_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
    { let s_ = (p_tbl_2).wrapping_add((((p_b_4) as u32)).wrapping_mul(16)); let d_ = ((p_w_1).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftZap_SetTexPair", &PROBE_N, &[], true); }
    r_
}

pub fn EftZap_LoadTex(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftZap_LoadTex", &PROBE_N, &[(p_w_1) as u32], false); }
    let mut l_tex_2: u32 = 0;
    let mut l_tex0_3: u64 = 0;
    let r_ = 'f: {
    l_tex_2 = c.m.r32(((p_w_1).wrapping_add(48)).wrapping_add(8));
    if (((((l_tex_2) != (((0i32) as u32))) as i32)) != 0) {
        if ((((!((((c.m.r32((l_tex_2).wrapping_add(260))) & ((1u32).wrapping_shl((((c.m.r8((p_w_1).wrapping_add(376))) as i32)) as u32)))) != 0)) as i32)) != 0) {
            l_tex0_3 = F::EftVram_AddImage(c, ((p_w_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), 1i32, 0i32);
            { let r_ = (F::EftVram_AddClut(c, ((p_w_1).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(16)))).wrapping_shl((37i32) as u32); let n_ = ((l_tex0_3) | (r_)); l_tex0_3 = n_; };
            { let v_ = l_tex0_3; let a_ = (l_tex_2).wrapping_add((((c.m.r8((p_w_1).wrapping_add(376))) as u32)).wrapping_mul(16)); c.m.w64(a_, v_); };
            { let r_ = (1u32).wrapping_shl((((c.m.r8((p_w_1).wrapping_add(376))) as i32)) as u32); let a_ = (l_tex_2).wrapping_add(260); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftZap_LoadTex", &PROBE_N, &[], true); }
    r_
}

pub fn EftZap_Create(c: &mut Ctx, mut p_arg_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftZap_Create", &PROBE_N, &[(p_arg_1) as u32], false); }
    let r_ = 'f: {
    if (((((((((((((c.m.r32(sym::gEftZapMgr)) == (((0i32) as u32))) as i32)) != 0) || (((((p_arg_1) == (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) || ((((({ let a_ = c.m.r32(sym::gEftZapMgr); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    break 'f { let a0_ = { let a_ = c.m.r32(sym::gEftZapMgr); c.m.r32(a_) }; F::BtlTaskList_AddTail(c, a0_, sym::gEftZapClass, p_arg_1) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftZap_Create", &PROBE_N, &[], true); }
    r_
}

pub fn EftZap_Kill(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftZap_Kill", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((((((((((c.m.r32(sym::gEftZapMgr)) != (((0i32) as u32))) as i32)) != 0) && (((((p_task_1) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftZap_Update)) as i32)) != 0)) as i32)) != 0) {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        if (((((l_w_2) != (((0i32) as u32))) as i32)) != 0) {
            if (((((c.m.r32((l_w_2).wrapping_add(380)) as i32)) & (1i32))) != 0) {
                { let r_ = 64i32; let a_ = (l_w_2).wrapping_add(380); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftZap_Kill", &PROBE_N, &[], true); }
    r_
}

pub fn EftZap_Stop(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftZap_Stop", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_now_3: i32 = 0;
    let r_ = 'f: {
    l_now_3 = 1i32;
    if (((((c.m.r32(sym::gEftZapMgr)) != (((0i32) as u32))) as i32)) != 0) {
        if (((((((((p_task_1) != (((0i32) as u32))) as i32)) != 0) && ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftZap_Update)) as i32)) != 0)) as i32)) != 0) {
            l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
            if (((((l_w_2) != (((0i32) as u32))) as i32)) != 0) {
                if (((((c.m.r32((l_w_2).wrapping_add(380)) as i32)) & (1i32))) != 0) {
                    if (((rt::fgt(F32(c.m.r32((l_w_2).wrapping_add(364))), F32(0x00000000)) as i32)) != 0) {
                        { let r_ = 16i32; let a_ = (l_w_2).wrapping_add(380); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        l_now_3 = 0i32;
                    }
                    if (((rt::fgt(F32(c.m.r32((l_w_2).wrapping_add(356))), F32(0x00000000)) as i32)) != 0) {
                        { let r_ = 4i32; let a_ = (l_w_2).wrapping_add(380); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        l_now_3 = 0i32;
                    }
                    if ((l_now_3) != 0) {
                        if (((({ let a_ = (c.m.r32((l_w_2).wrapping_add(48))).wrapping_add(180); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
                            { let r_ = 32i32; let a_ = (l_w_2).wrapping_add(380); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                        { let r_ = ((32i32) | (8i32)); let a_ = (l_w_2).wrapping_add(380); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftZap_Stop", &PROBE_N, &[], true); }
    r_
}

pub fn EftZap_SetPos(c: &mut Ctx, mut p_task_1: u32, p_pos_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftZap_SetPos", &PROBE_N, &[(p_task_1) as u32, (p_pos_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_3: u32 = 0;
    rt::memcpy(c, fp, p_pos_2, 16);
    let r_ = 'f: {
        if (((((((((((((c.m.r32(sym::gEftZapMgr)) != (((0i32) as u32))) as i32)) != 0) && (((((p_task_1) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftZap_Update)) as i32)) != 0)) as i32)) != 0) {
            l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
            if (((((l_w_3) != (((0i32) as u32))) as i32)) != 0) {
                if (((((c.m.r32((l_w_3).wrapping_add(380)) as i32)) & (1i32))) != 0) {
                    F::Vec4_Copy(c, ((l_w_3).wrapping_add(48)).wrapping_add(32), fp);
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftZap_SetPos", &PROBE_N, &[], true); }
    r_
}

pub fn EftZap_WarpPos(c: &mut Ctx, mut p_task_1: u32, p_pos_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftZap_WarpPos", &PROBE_N, &[(p_task_1) as u32, (p_pos_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_3: u32 = 0;
    rt::memcpy(c, fp, p_pos_2, 16);
    let r_ = 'f: {
        if (((((((((((((c.m.r32(sym::gEftZapMgr)) != (((0i32) as u32))) as i32)) != 0) && (((((p_task_1) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftZap_Update)) as i32)) != 0)) as i32)) != 0) {
            l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
            if (((((l_w_3) != (((0i32) as u32))) as i32)) != 0) {
                if (((((c.m.r32((l_w_3).wrapping_add(380)) as i32)) & (1i32))) != 0) {
                    F::Vec4_Copy(c, ((l_w_3).wrapping_add(48)).wrapping_add(32), fp);
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftZap_WarpPos", &PROBE_N, &[], true); }
    r_
}

pub fn EftZap_SetDir(c: &mut Ctx, mut p_task_1: u32, p_dir_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftZap_SetDir", &PROBE_N, &[(p_task_1) as u32, (p_dir_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_3: u32 = 0;
    rt::memcpy(c, fp, p_dir_2, 16);
    let r_ = 'f: {
        if (((((((((((((c.m.r32(sym::gEftZapMgr)) != (((0i32) as u32))) as i32)) != 0) && (((((p_task_1) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftZap_Update)) as i32)) != 0)) as i32)) != 0) {
            l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
            if (((((l_w_3) != (((0i32) as u32))) as i32)) != 0) {
                if (((((c.m.r32((l_w_3).wrapping_add(380)) as i32)) & (1i32))) != 0) {
                    F::Vec4_Copy(c, ((l_w_3).wrapping_add(48)).wrapping_add(16), fp);
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftZap_SetDir", &PROBE_N, &[], true); }
    r_
}

pub fn EftZap_SetSize(c: &mut Ctx, mut p_task_1: u32, mut p_size_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftZap_SetSize", &PROBE_N, &[(p_task_1) as u32, (p_size_2).0], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((((((((((c.m.r32(sym::gEftZapMgr)) != (((0i32) as u32))) as i32)) != 0) && (((((p_task_1) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftZap_Update)) as i32)) != 0)) as i32)) != 0) {
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        if (((((l_w_3) != (((0i32) as u32))) as i32)) != 0) {
            if (((((c.m.r32((l_w_3).wrapping_add(380)) as i32)) & (1i32))) != 0) {
                { let v_ = p_size_2; let a_ = ((l_w_3).wrapping_add(48)).wrapping_add(48); c.m.w32(a_, (v_).0); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftZap_SetSize", &PROBE_N, &[], true); }
    r_
}

pub fn EftZap_SetRate(c: &mut Ctx, mut p_task_1: u32, mut p_rate_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftZap_SetRate", &PROBE_N, &[(p_task_1) as u32, (p_rate_2).0], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((((((((((c.m.r32(sym::gEftZapMgr)) != (((0i32) as u32))) as i32)) != 0) && (((((p_task_1) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftZap_Update)) as i32)) != 0)) as i32)) != 0) {
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        if (((((l_w_3) != (((0i32) as u32))) as i32)) != 0) {
            if (((((c.m.r32((l_w_3).wrapping_add(380)) as i32)) & (1i32))) != 0) {
                { let v_ = p_rate_2; let a_ = ((l_w_3).wrapping_add(48)).wrapping_add(52); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul(p_rate_2, F32(0x41f00000)); let a_ = (l_w_3).wrapping_add(372); c.m.w32(a_, (v_).0); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftZap_SetRate", &PROBE_N, &[], true); }
    r_
}

pub fn EftZap_SetTex(c: &mut Ctx, mut p_task_1: u32, mut p_tex_2: u32, mut p_a_3: i32, mut p_b_4: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftZap_SetTex", &PROBE_N, &[(p_task_1) as u32, (p_tex_2) as u32, (p_a_3) as u32, (p_b_4) as u32], false); }
    let mut l_w_5: u32 = 0;
    let r_ = 'f: {
    if (((((((((((((c.m.r32(sym::gEftZapMgr)) != (((0i32) as u32))) as i32)) != 0) && (((((p_task_1) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftZap_Update)) as i32)) != 0)) as i32)) != 0) {
        l_w_5 = c.m.r32((p_task_1).wrapping_add(56));
        if (((((l_w_5) != (((0i32) as u32))) as i32)) != 0) {
            if (((((c.m.r32((l_w_5).wrapping_add(380)) as i32)) & (1i32))) != 0) {
                { let v_ = p_tex_2; let a_ = ((l_w_5).wrapping_add(48)).wrapping_add(8); c.m.w32(a_, v_); };
                F::EftZap_SetTexPair(c, l_w_5, p_tex_2, p_a_3, p_b_4);
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftZap_SetTex", &PROBE_N, &[], true); }
    r_
}

pub fn EftZap_SetDelay(c: &mut Ctx, mut p_task_1: u32, mut p_frames_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftZap_SetDelay", &PROBE_N, &[(p_task_1) as u32, (p_frames_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((((((((((c.m.r32(sym::gEftZapMgr)) != (((0i32) as u32))) as i32)) != 0) && (((((p_task_1) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftZap_Update)) as i32)) != 0)) as i32)) != 0) {
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        if (((((l_w_3) != (((0i32) as u32))) as i32)) != 0) {
            if (((((c.m.r32((l_w_3).wrapping_add(380)) as i32)) & (1i32))) != 0) {
                { let v_ = rt::i2f(p_frames_2); let a_ = (l_w_3).wrapping_add(352); c.m.w32(a_, (v_).0); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftZap_SetDelay", &PROBE_N, &[], true); }
    r_
}

pub fn EftZap_SetFadeDelay(c: &mut Ctx, mut p_task_1: u32, mut p_frames_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftZap_SetFadeDelay", &PROBE_N, &[(p_task_1) as u32, (p_frames_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((((((((((c.m.r32(sym::gEftZapMgr)) != (((0i32) as u32))) as i32)) != 0) && (((((p_task_1) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftZap_Update)) as i32)) != 0)) as i32)) != 0) {
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        if (((((l_w_3) != (((0i32) as u32))) as i32)) != 0) {
            if (((((c.m.r32((l_w_3).wrapping_add(380)) as i32)) & (1i32))) != 0) {
                { let v_ = rt::i2f(p_frames_2); let a_ = (l_w_3).wrapping_add(356); c.m.w32(a_, (v_).0); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftZap_SetFadeDelay", &PROBE_N, &[], true); }
    r_
}

pub fn EftZap_SetFadeTime(c: &mut Ctx, mut p_task_1: u32, mut p_frames_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftZap_SetFadeTime", &PROBE_N, &[(p_task_1) as u32, (p_frames_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((((((((((c.m.r32(sym::gEftZapMgr)) != (((0i32) as u32))) as i32)) != 0) && (((((p_task_1) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftZap_Update)) as i32)) != 0)) as i32)) != 0) {
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        if (((((l_w_3) != (((0i32) as u32))) as i32)) != 0) {
            if (((((c.m.r32((l_w_3).wrapping_add(380)) as i32)) & (1i32))) != 0) {
                { let v_ = rt::i2f(p_frames_2); let a_ = (l_w_3).wrapping_add(364); c.m.w32(a_, (v_).0); };
                { let v_ = rt::i2f(p_frames_2); let a_ = (l_w_3).wrapping_add(360); c.m.w32(a_, (v_).0); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftZap_SetFadeTime", &PROBE_N, &[], true); }
    r_
}

pub fn EftZap_SetFlag20000(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftZap_SetFlag20000", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftZapMgr)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftZap_Update)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32((l_w_2).wrapping_add(380)) as i32)) & (1i32))) != 0) {
        { let r_ = 131072i32; let a_ = (l_w_2).wrapping_add(380); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftZap_SetFlag20000", &PROBE_N, &[], true); }
    r_
}

pub fn EftZap_SetUnk40(c: &mut Ctx, mut p_task_1: u32, mut p_value_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftZap_SetUnk40", &PROBE_N, &[(p_task_1) as u32, (p_value_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftZapMgr)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftZap_Update)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32((l_w_3).wrapping_add(380)) as i32)) & (1i32))) != 0) {
        { let v_ = ((p_value_2) as u8); let a_ = ((l_w_3).wrapping_add(48)).wrapping_add(64); c.m.w8(a_, v_); };
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftZap_SetUnk40", &PROBE_N, &[], true); }
    r_
}

pub fn EftZap_IsAlive(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftZap_IsAlive", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftZapMgr)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftZap_Update)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32((l_w_2).wrapping_add(380)) as i32)) & (1i32))) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftZap_IsAlive", &PROBE_N, &[], true); }
    r_
}

pub fn EftShock_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShock_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let _ = F::memset(c, l_w_3, 0i32, 48u32); };
    { let v_ = (c.m.r32((p_arg_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a_ = (l_w_3).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let a1_ = (if ((((((c.m.r32((p_arg_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) == (0i32)) as i32)) != 0) { 2048i32 } else { 4096i32 }); F::BtlTask_SetOwnerTag(c, p_task_1, a1_) };
    { let v_ = ((2i32) as i16); let a_ = (l_w_3).wrapping_add(34); c.m.w16(a_, (v_) as u16); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShock_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftShock_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShock_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((c.m.r32(((l_w_2).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
        { let _ = { let a0_ = c.m.r32(((l_w_2).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(4))); F::EftAnimPart_Kill(c, a0_) }; };
        { let v_ = ((0i32) as u32); let a_ = ((l_w_2).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(((l_w_2).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
        { let _ = { let a0_ = c.m.r32(((l_w_2).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(4))); F::EftAnimPart_Kill(c, a0_) }; };
        { let v_ = ((0i32) as u32); let a_ = ((l_w_2).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    { let v_ = 0i32; let a_ = (l_w_2).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShock_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftShock_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShock_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_w_2: u32 = 0;
    let mut l_res_3: u32 = 0;
    let mut l_objId_4: u32 = 0;
    let mut l_alive_5: i32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_part_7: u32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_res_3 = { let a_ = { let a_ = (F::BtlTask_GetParent(c, p_task_1)).wrapping_add(56); c.m.r32(a_) }; c.m.r32(a_) };
        l_objId_4 = (l_w_2).wrapping_add(16);
        if ((((!(({ let a0_ = (c.m.r32(l_objId_4) as i32); F::BtlScene_IsEffectStopped(c, a0_, 3i32) }) != 0)) as i32)) != 0) {
            { let a0_ = (c.m.r32(l_objId_4) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, l_w_2) };
            if ((((!(((((c.m.r32((l_w_2).wrapping_add(28)) as i32)) & (4i32))) != 0)) as i32)) != 0) {
                { let _ = { let a0_ = (c.m.r32(l_objId_4) as i32); F::ScrWarp_Spawn(c, a0_, l_w_2, F32(0x3f4ccccc), F32(0x41200000), F32(0x42480000), F32(0x41000000), F32(0x3ecccccc)) }; };
                { let r_ = 4i32; let a_ = (l_w_2).wrapping_add(28); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
            if ((((!(((((c.m.r32((l_w_2).wrapping_add(28)) as i32)) & (8i32))) != 0)) as i32)) != 0) {
                l_part_7 = (l_w_2).wrapping_add(4);
                { let v_ = (c.m.r32(l_objId_4) as i32); let a_ = fp; c.m.w32(a_, (v_) as u32); };
                { let v_ = 4i32; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = F32(0x3f4ccccc); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul({ let a0_ = (c.m.r32(l_objId_4) as i32); F::BtlScene_GetCharScale(c, a0_) }, F32(0x40000000)); let a_ = (fp).wrapping_add(48); c.m.w32(a_, (v_).0); };
                { let v_ = c.m.r32((l_res_3).wrapping_add(4)); let a_ = (fp).wrapping_add(52); c.m.w32(a_, v_); };
                { let v_ = (l_res_3).wrapping_add(16); let a_ = (fp).wrapping_add(56); c.m.w32(a_, v_); };
                F::Vec4_Copy(c, (fp).wrapping_add(16), l_w_2);
                { let a0_ = (c.m.r32(l_objId_4) as i32); F::BtlCharApi_GetDir(c, a0_, (fp).wrapping_add(32)) };
                { let v_ = F::EftAnimPart_Create(c, fp); let a_ = (l_part_7).wrapping_add((((((((c.m.r16((l_w_2).wrapping_add(32)) as i16)) as i32)).wrapping_add(4i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let _ = { let a0_ = { let a_ = (l_part_7).wrapping_add((((((((c.m.r16((l_w_2).wrapping_add(32)) as i16)) as i32)).wrapping_add(4i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::EftAnimPart_SetType(c, a0_, 3i32) }; };
                { let a0_ = { let a_ = (l_part_7).wrapping_add((((((((c.m.r16((l_w_2).wrapping_add(32)) as i16)) as i32)).wrapping_add(4i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a1_ = (if ((((((c.m.r32(l_objId_4) as i32)) == (0i32)) as i32)) != 0) { 2048i32 } else { 4096i32 }); F::BtlTask_SetOwnerTag(c, a0_, a1_) };
                { let a_ = (l_w_2).wrapping_add(32); let o_ = (c.m.r16(a_) as i16); let n_ = (o_).wrapping_add(1); c.m.w16(a_, (n_) as u16); };
                { let r_ = 8i32; let a_ = (l_w_2).wrapping_add(28); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
            if ((((((((c.m.r16((l_w_2).wrapping_add(34)) as i16)) as i32)) > (0i32)) as i32)) != 0) {
                if ((((((({ let a_ = (l_w_2).wrapping_add(34); let o_ = (c.m.r16(a_) as i16); let n_ = (o_).wrapping_sub(1); c.m.w16(a_, (n_) as u16); n_ }) as i32)) == (0i32)) as i32)) != 0) {
                    { let r_ = (!(8i32)); let a_ = (l_w_2).wrapping_add(28); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
            if ((((((((c.m.r16((l_w_2).wrapping_add(32)) as i16)) as i32)) == (2i32)) as i32)) != 0) {
                l_alive_5 = 0i32;
                {
                    l_i_6 = 0i32;
                    'l1: loop {
                        if !(((((l_i_6) < (2i32)) as i32)) != 0) { break 'l1; }
                        'c2: {
                            if (((((((((c.m.r32(((l_w_2).wrapping_add(20)).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) && (({ let a0_ = c.m.r32(((l_w_2).wrapping_add(20)).wrapping_add((((l_i_6) as u32)).wrapping_mul(4))); F::EftAnimPart_IsAlive(c, a0_) }) != 0)) as i32)) != 0) {
                                l_alive_5 = 1i32;
                            }
                        }
                        { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                    }
                }
                if ((((!((l_alive_5) != 0)) as i32)) != 0) {
                    { let r_ = 2i32; let a_ = (l_w_2).wrapping_add(28); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
        }
        if (((((c.m.r32((l_w_2).wrapping_add(28)) as i32)) & (2i32))) != 0) {
            F::BtlTask_SetDead(c, p_task_1);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftShock_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftShock_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShock_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShock_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftShock_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShock_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShock_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftShock_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShock_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShock_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftShockMgr_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShockMgr_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_mgr_3: u32 = 0;
    let mut l_res_4: u32 = 0;
    let mut l_list_5: u32 = 0;
    let r_ = 'f: {
    l_mgr_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((536u32) as i32)) }; let a_ = l_mgr_3; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(l_mgr_3); F::memset(c, a0_, 0i32, 536u32) }; };
    l_res_4 = c.m.r32(l_mgr_3);
    { let v_ = { let a0_ = (c.m.r32((p_arg_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::BtlScene_GetCharPackEntry(c, a0_, 6i32) }; let a_ = l_res_4; c.m.w32(a_, v_); };
    if (((((c.m.r32(l_res_4)) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a0_ = c.m.r32(l_res_4); F::BtlScene_GetPackEntry(c, a0_, 1i32) }; let a_ = (l_res_4).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = c.m.r32(l_res_4); F::BtlScene_GetPackEntry(c, a0_, 2i32) }; let a_ = (l_res_4).wrapping_add(8); c.m.w32(a_, v_); };
        { let a1_ = c.m.r32((l_res_4).wrapping_add(8)); F::EftTexSet_Load32(c, (l_res_4).wrapping_add(16), a1_) };
    }
    l_list_5 = F::BtlTask_CreateChildList(c, p_task_1, ((((2i32) as i64 as u64)) as u32), ((((((48u32) as i32)) as i64 as u64)) as u32));
    { let a0_ = (c.m.r32((p_arg_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a1_ = (c.m.r32((p_arg_2).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); F::EftChar_SetList(c, a0_, a1_, l_list_5) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShockMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftShockMgr_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShockMgr_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_mgr_2: u32 = 0;
    let r_ = 'f: {
    l_mgr_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((c.m.r32(l_mgr_2)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(l_mgr_2); F::BtlPool_Free(c, a0_, a1_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShockMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftShockMgr_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShockMgr_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_mgr_2: u32 = 0;
    let r_ = 'f: {
    l_mgr_2 = c.m.r32((p_task_1).wrapping_add(56));
    { let v_ = 0i32; let a_ = ((c.m.r32(l_mgr_2)).wrapping_add(16)).wrapping_add(516); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShockMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftShock_Start(c: &mut Ctx, mut p_arg_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShock_Start", &PROBE_N, &[(p_arg_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_list_2: u32 = 0;
    let r_ = 'f: {
        { let v_ = (c.m.r32((p_arg_1).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        l_list_2 = { let a0_ = (c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::EftChar_GetList(c, a0_, 4i32) };
        if (((((l_list_2) == (((0i32) as u32))) as i32)) != 0) {
            break 'f ((0i32) as u32);
        }
        break 'f F::BtlTaskList_AddTail(c, l_list_2, sym::gEftShockClass, fp);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftShock_Start", &PROBE_N, &[], true); }
    r_
}
