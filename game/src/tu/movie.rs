#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn MovieTag_BuildImage(c: &mut Ctx, mut p_tags_1: u32, mut p_image_2: u32, mut p_x_3: i32, mut p_y_4: i32, mut p_w_5: i32, mut p_h_6: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MovieTag_BuildImage", &PROBE_N, &[(p_tags_1) as u32, (p_image_2) as u32, (p_x_3) as u32, (p_y_4) as u32, (p_w_5) as u32, (p_h_6) as u32], false); }
    let r_ = 'f: {
    F::MovieTag_BuildBlocks(c, p_tags_1, p_image_2, 1024i32, p_x_3, p_y_4, p_w_5, p_h_6);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "MovieTag_BuildImage", &PROBE_N, &[], true); }
    r_
}

pub fn MovieTag_BuildFill(c: &mut Ctx, mut p_tags_1: u32, mut p_image_2: u32, mut p_x_3: i32, mut p_y_4: i32, mut p_w_5: i32, mut p_h_6: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MovieTag_BuildFill", &PROBE_N, &[(p_tags_1) as u32, (p_image_2) as u32, (p_x_3) as u32, (p_y_4) as u32, (p_w_5) as u32, (p_h_6) as u32], false); }
    let r_ = 'f: {
    F::MovieTag_BuildBlocks(c, p_tags_1, p_image_2, 0i32, p_x_3, p_y_4, p_w_5, p_h_6);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "MovieTag_BuildFill", &PROBE_N, &[], true); }
    r_
}

pub fn MovieTag_BuildBlocks(c: &mut Ctx, mut p_tags_1: u32, mut p_image_2: u32, mut p_step_3: i32, mut p_x_4: i32, mut p_y_5: i32, mut p_w_6: i32, mut p_h_7: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MovieTag_BuildBlocks", &PROBE_N, &[(p_tags_1) as u32, (p_image_2) as u32, (p_step_3) as u32, (p_x_4) as u32, (p_y_5) as u32, (p_w_6) as u32, (p_h_7) as u32], false); }
    let mut l_p_8: u32 = 0;
    let mut l_cols_9: i32 = 0;
    let mut l_rows_10: i32 = 0;
    let mut l_i_11: i32 = 0;
    let mut l_j_12: i32 = 0;
    let mut l_eop_13: i32 = 0;
    let r_ = 'f: {
    l_p_8 = p_tags_1;
    l_cols_9 = (p_w_6).wrapping_shr((4i32) as u32);
    l_rows_10 = (p_h_7).wrapping_shr((4i32) as u32);
    F::MovieTag_SetDmaTag(c, (l_p_8).wrapping_add((((0i32) as u32)).wrapping_mul(8)), 0i32, 0i32, 0i32, 1i32, 0i32, 3i32);
    { let o_ = l_p_8; let n_ = (o_).wrapping_add(16); l_p_8 = n_; };
    F::MovieTag_SetGifTag(c, l_p_8, ((14i32) as u64), 1i32, 0i32, 0i32, 0i32, 0i32, 2i32);
    { let o_ = l_p_8; let n_ = (o_).wrapping_add(16); l_p_8 = n_; };
    F::MovieTag_SetBitBltBuf(c, l_p_8, 0i32, 8i32, 0i32);
    { let o_ = l_p_8; let n_ = (o_).wrapping_add(16); l_p_8 = n_; };
    F::MovieTag_SetTrxReg(c, l_p_8, 16i32, 16i32);
    { let o_ = l_p_8; let n_ = (o_).wrapping_add(16); l_p_8 = n_; };
    {
        l_i_11 = 0i32;
        'l1: loop {
            if !(((((l_i_11) < (l_cols_9)) as i32)) != 0) { break 'l1; }
            'c2: {
                {
                    l_j_12 = 0i32;
                    'l3: loop {
                        if !(((((l_j_12) < (l_rows_10)) as i32)) != 0) { break 'l3; }
                        'c4: {
                            l_eop_13 = (((((((l_j_12) == ((l_rows_10).wrapping_sub(1i32))) as i32)) != 0) && (((((l_i_11) == ((l_cols_9).wrapping_sub(1i32))) as i32)) != 0)) as i32);
                            F::MovieTag_SetDmaTag(c, (l_p_8).wrapping_add((((0i32) as u32)).wrapping_mul(8)), 0i32, 0i32, 0i32, 1i32, 0i32, 4i32);
                            { let o_ = l_p_8; let n_ = (o_).wrapping_add(16); l_p_8 = n_; };
                            F::MovieTag_SetGifTag(c, l_p_8, ((14i32) as u64), 1i32, 0i32, 0i32, 0i32, 0i32, 2i32);
                            { let o_ = l_p_8; let n_ = (o_).wrapping_add(16); l_p_8 = n_; };
                            F::MovieTag_SetTrxPos(c, l_p_8, 0i32, (p_x_4).wrapping_add((l_i_11).wrapping_mul(16i32)), (p_y_5).wrapping_add((l_j_12).wrapping_mul(16i32)));
                            { let o_ = l_p_8; let n_ = (o_).wrapping_add(16); l_p_8 = n_; };
                            F::MovieTag_SetTrxDir(c, l_p_8, 0i32);
                            { let o_ = l_p_8; let n_ = (o_).wrapping_add(16); l_p_8 = n_; };
                            F::MovieTag_SetGifTag(c, l_p_8, ((0i32) as u64), 0i32, 2i32, 0i32, 0i32, l_eop_13, 64i32);
                            { let o_ = l_p_8; let n_ = (o_).wrapping_add(16); l_p_8 = n_; };
                            F::MovieTag_SetDmaTag(c, (l_p_8).wrapping_add((((0i32) as u32)).wrapping_mul(8)), 0i32, ((p_image_2) as i32), 0i32, 3i32, 0i32, 64i32);
                            { let o_ = l_p_8; let n_ = (o_).wrapping_add(16); l_p_8 = n_; };
                            { let r_ = p_step_3; let n_ = (p_image_2).wrapping_add(((r_) as u32)); p_image_2 = n_; };
                        }
                        { let o_ = l_j_12; let n_ = (o_).wrapping_add(1); l_j_12 = n_; };
                    }
                }
            }
            { let o_ = l_i_11; let n_ = (o_).wrapping_add(1); l_i_11 = n_; };
        }
    }
    F::MovieTag_SetDmaTag(c, (l_p_8).wrapping_add((((0i32) as u32)).wrapping_mul(8)), 0i32, 0i32, 0i32, 7i32, 0i32, 0i32);
    { let o_ = l_p_8; let n_ = (o_).wrapping_add(16); l_p_8 = n_; };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "MovieTag_BuildBlocks", &PROBE_N, &[], true); }
    r_
}

pub fn MovieTag_SetDmaTag(c: &mut Ctx, mut p_p_1: u32, mut p_spr_2: i32, mut p_addr_3: i32, mut p_irq_4: i32, mut p_id_5: i32, mut p_pce_6: i32, mut p_qwc_7: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MovieTag_SetDmaTag", &PROBE_N, &[(p_p_1) as u32, (p_spr_2) as u32, (p_addr_3) as u32, (p_irq_4) as u32, (p_id_5) as u32, (p_pce_6) as u32, (p_qwc_7) as u32], false); }
    let r_ = 'f: {
    { let v_ = (((((((((((((p_spr_2) as u64)).wrapping_shl((63i32) as u32)) | ((((((p_addr_3) & ((!(15i32))))) as u64)).wrapping_shl((32i32) as u32)))) | ((((((p_irq_4) as u32)) as u64)).wrapping_shl((31i32) as u32)))) | ((((((p_id_5) as u32)) as u64)).wrapping_shl((28i32) as u32)))) | ((((((p_pce_6) as u32)) as u64)).wrapping_shl((26i32) as u32)))) | (((((p_qwc_7) as u32)) as u64))); let a_ = p_p_1; c.m.w64(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "MovieTag_SetDmaTag", &PROBE_N, &[], true); }
    r_
}

pub fn MovieTag_SetGifTag(c: &mut Ctx, mut p_p_1: u32, mut p_regs_2: u64, mut p_nreg_3: i32, mut p_flg_4: i32, mut p_prim_5: i32, mut p_pre_6: i32, mut p_eop_7: i32, mut p_nloop_8: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MovieTag_SetGifTag", &PROBE_N, &[(p_p_1) as u32, (p_regs_2) as u32, (p_nreg_3) as u32, (p_flg_4) as u32, (p_prim_5) as u32, (p_pre_6) as u32, (p_eop_7) as u32, (p_nloop_8) as u32], false); }
    let r_ = 'f: {
    { let v_ = (((((p_eop_7).wrapping_shl((15i32) as u32)) | (p_nloop_8))) as u32); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (((((((((p_pre_6).wrapping_shl((14i32) as u32)) | ((p_prim_5).wrapping_shl((15i32) as u32)))) | ((p_flg_4).wrapping_shl((26i32) as u32)))) | ((p_nreg_3).wrapping_shl((28i32) as u32)))) as u32); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((p_regs_2) & (((4294967295u32) as u64)))) as u32); let a_ = (p_p_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (((p_regs_2).wrapping_shr((32i32) as u32)) as u32); let a_ = (p_p_1).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "MovieTag_SetGifTag", &PROBE_N, &[], true); }
    r_
}

pub fn MovieTag_SetReg(c: &mut Ctx, mut p_p_1: u32, mut p_reg_2: i32, mut p_value_3: u64) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MovieTag_SetReg", &PROBE_N, &[(p_p_1) as u32, (p_reg_2) as u32, (p_value_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((p_reg_2) as u32); let a_ = (p_p_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (((p_value_3).wrapping_shr((32i32) as u32)) as u32); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((p_value_3) & (((4294967295u32) as u64)))) as u32); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (p_p_1).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "MovieTag_SetReg", &PROBE_N, &[], true); }
    r_
}

pub fn MovieTag_SetBitBltBuf(c: &mut Ctx, mut p_p_1: u32, mut p_dbp_2: i32, mut p_dbw_3: i32, mut p_dpsm_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MovieTag_SetBitBltBuf", &PROBE_N, &[(p_p_1) as u32, (p_dbp_2) as u32, (p_dbw_3) as u32, (p_dpsm_4) as u32], false); }
    let r_ = 'f: {
    F::MovieTag_SetReg(c, p_p_1, 80i32, (((((((p_dpsm_4) as u64)).wrapping_shl((56i32) as u32)) | ((((p_dbw_3) as u64)).wrapping_shl((48i32) as u32)))) | ((((p_dbp_2) as u64)).wrapping_shl((32i32) as u32))));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "MovieTag_SetBitBltBuf", &PROBE_N, &[], true); }
    r_
}

pub fn MovieTag_SetTrxPos(c: &mut Ctx, mut p_p_1: u32, mut p_dir_2: i32, mut p_x_3: i32, mut p_y_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MovieTag_SetTrxPos", &PROBE_N, &[(p_p_1) as u32, (p_dir_2) as u32, (p_x_3) as u32, (p_y_4) as u32], false); }
    let r_ = 'f: {
    F::MovieTag_SetReg(c, p_p_1, 81i32, (((((((p_dir_2) as u64)).wrapping_shl((59i32) as u32)) | ((((p_y_4) as u64)).wrapping_shl((48i32) as u32)))) | ((((p_x_3) as u64)).wrapping_shl((32i32) as u32))));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "MovieTag_SetTrxPos", &PROBE_N, &[], true); }
    r_
}

pub fn MovieTag_SetTrxReg(c: &mut Ctx, mut p_p_1: u32, mut p_w_2: i32, mut p_h_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MovieTag_SetTrxReg", &PROBE_N, &[(p_p_1) as u32, (p_w_2) as u32, (p_h_3) as u32], false); }
    let r_ = 'f: {
    F::MovieTag_SetReg(c, p_p_1, 82i32, (((((p_h_3) as u64)).wrapping_shl((32i32) as u32)) | (((((p_w_2) as u32)) as u64))));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "MovieTag_SetTrxReg", &PROBE_N, &[], true); }
    r_
}

pub fn MovieTag_SetTrxDir(c: &mut Ctx, mut p_p_1: u32, mut p_dir_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MovieTag_SetTrxDir", &PROBE_N, &[(p_p_1) as u32, (p_dir_2) as u32], false); }
    let r_ = 'f: {
    F::MovieTag_SetReg(c, p_p_1, 83i32, ((((p_dir_2) as u32)) as u64));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "MovieTag_SetTrxDir", &PROBE_N, &[], true); }
    r_
}

pub fn MovieTag_Send(c: &mut Ctx, mut p_tags_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MovieTag_Send", &PROBE_N, &[(p_tags_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_tags_1; let a_ = ((268476464i32) as u32); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = ((268476448i32) as u32); c.m.w32(a_, v_); };
    { let v_ = ((261i32) as u32); let a_ = F::Port_DmaChcr(c, 2i32); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "MovieTag_Send", &PROBE_N, &[], true); }
    r_
}

pub fn Movie_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Movie_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let _ = F::sceMpegInit(c); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Movie_Init", &PROBE_N, &[], true); }
    r_
}

pub fn Movie_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Movie_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Movie_Close(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Movie_Term", &PROBE_N, &[], true); }
    r_
}

pub fn Movie_Open(c: &mut Ctx, mut p_pss_1: u32, mut p_adx_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Movie_Open", &PROBE_N, &[(p_pss_1) as u32, (p_adx_2) as u32], false); }
    let mut l_adxf_3: u32 = 0;
    let mut l_m_4: u32 = 0;
    let mut l_buf_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let r_ = 'f: {
    'l1: loop {
        'c2: {
            l_adxf_3 = F::ADXF_Open(c, p_pss_1, ((0i32) as u32));
        }
        if !(((((l_adxf_3) == (((0i32) as u32))) as i32)) != 0) { break 'l1; }
    }
    F::Adx_PlayFilePaused(c, 0i32, p_adx_2, 64i32, 0i32);
    { let v_ = F::Heap_Alloc(c, ((248u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gMovie; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gMovie); F::memset(c, a0_, 0i32, 248u32) }; };
    l_m_4 = c.m.r32(sym::gMovie);
    { let v_ = l_adxf_3; let a_ = l_m_4; c.m.w32(a_, v_); };
    { let v_ = c.m.r32(((sym::gAdxPlayerTbl).wrapping_add((((0i32) as u32)).wrapping_mul(20))).wrapping_add(16)); let a_ = (l_m_4).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = ((512i32).wrapping_mul(448i32)).wrapping_mul(4i32); let a_ = (l_m_4).wrapping_add(212); c.m.w32(a_, (v_) as u32); };
    { let v_ = 86272i32; let a_ = (l_m_4).wrapping_add(224); c.m.w32(a_, (v_) as u32); };
    { let v_ = 1042432i32; let a_ = (l_m_4).wrapping_add(232); c.m.w32(a_, (v_) as u32); };
    { let v_ = 196608i32; let a_ = (l_m_4).wrapping_add(240); c.m.w32(a_, (v_) as u32); };
    { let v_ = F::Heap_Alloc(c, ((512i32).wrapping_mul(448i32)).wrapping_mul(4i32), ((64i32) as u32), 0i32, 2i32); let a_ = (l_m_4).wrapping_add(208); c.m.w32(a_, v_); };
    { let v_ = { let a0_ = (c.m.r32((l_m_4).wrapping_add(224)) as i32); F::Heap_Alloc(c, a0_, ((64i32) as u32), 0i32, 2i32) }; let a_ = ((l_m_4).wrapping_add(216)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = { let a0_ = (c.m.r32((l_m_4).wrapping_add(224)) as i32); F::Heap_Alloc(c, a0_, ((64i32) as u32), 0i32, 2i32) }; let a_ = ((l_m_4).wrapping_add(216)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = { let a0_ = (c.m.r32((l_m_4).wrapping_add(232)) as i32); F::Heap_Alloc(c, a0_, ((64i32) as u32), 0i32, 2i32) }; let a_ = (l_m_4).wrapping_add(228); c.m.w32(a_, v_); };
    { let v_ = { let a0_ = (c.m.r32((l_m_4).wrapping_add(240)) as i32); F::Heap_Alloc(c, a0_, ((64i32) as u32), 0i32, 2i32) }; let a_ = (l_m_4).wrapping_add(236); c.m.w32(a_, v_); };
    {
        { l_i_6 = 0i32; l_buf_5 = (l_m_4).wrapping_add(88); };
        'l3: loop {
            if !(((((l_i_6) < (4i32)) as i32)) != 0) { break 'l3; }
            'c4: {
                { let v_ = F::Heap_Alloc(c, (256i32).wrapping_shl((11i32) as u32), ((64i32) as u32), 0i32, 2i32); let a_ = l_buf_5; c.m.w32(a_, v_); };
            }
            { { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; }; { let o_ = l_buf_5; let n_ = (o_).wrapping_add(28); l_buf_5 = n_; }; };
        }
    }
    { let _ = { let a1_ = c.m.r32((l_m_4).wrapping_add(228)); let a2_ = (c.m.r32((l_m_4).wrapping_add(232)) as i32); F::sceMpegCreate(c, (l_m_4).wrapping_add(16), a1_, a2_) }; };
    { let _ = F::sceMpegAddStrCallback(c, (l_m_4).wrapping_add(16), 0i32, 0i32, fid::Movie_CbVideoData, ((0i32) as u32)); };
    { let _ = F::sceMpegAddCallback(c, (l_m_4).wrapping_add(16), 1i32, fid::Movie_CbNoData, ((0i32) as u32)); };
    { let _ = F::sceMpegAddCallback(c, (l_m_4).wrapping_add(16), 4i32, fid::Movie_CbBackground, ((0i32) as u32)); };
    { let _ = F::sceMpegAddCallback(c, (l_m_4).wrapping_add(16), 0i32, fid::Movie_CbError, ((0i32) as u32)); };
    { let a0_ = c.m.r32(((l_m_4).wrapping_add(216)).wrapping_add((((0i32) as u32)).wrapping_mul(4))); let a1_ = c.m.r32((l_m_4).wrapping_add(208)); F::MovieTag_BuildImage(c, a0_, a1_, 0i32, 0i32, 512i32, 448i32) };
    { let a0_ = c.m.r32(((l_m_4).wrapping_add(216)).wrapping_add((((1i32) as u32)).wrapping_mul(4))); let a1_ = c.m.r32((l_m_4).wrapping_add(208)); F::MovieTag_BuildImage(c, a0_, a1_, 0i32, 448i32, 512i32, 448i32) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Movie_Open", &PROBE_N, &[], true); }
    r_
}

pub fn Movie_Close(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Movie_Close", &PROBE_N, &[], false); }
    let mut l_m_1: u32 = 0;
    let mut l_buf_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    l_m_1 = c.m.r32(sym::gMovie);
    if (((((l_m_1) != (((0i32) as u32))) as i32)) != 0) {
        F::Adx_Stop(c, 0i32);
        { let _ = F::sceMpegReset(c, (l_m_1).wrapping_add(16)); };
        { let _ = F::sceMpegDelete(c, (l_m_1).wrapping_add(16)); };
        { let a0_ = c.m.r32(l_m_1); F::ADXF_Close(c, a0_) };
        { let a0_ = c.m.r32((l_m_1).wrapping_add(208)); F::Heap_Free(c, a0_) };
        { let a0_ = c.m.r32(((l_m_1).wrapping_add(216)).wrapping_add((((0i32) as u32)).wrapping_mul(4))); F::Heap_Free(c, a0_) };
        { let a0_ = c.m.r32(((l_m_1).wrapping_add(216)).wrapping_add((((1i32) as u32)).wrapping_mul(4))); F::Heap_Free(c, a0_) };
        { let a0_ = c.m.r32((l_m_1).wrapping_add(228)); F::Heap_Free(c, a0_) };
        { let a0_ = c.m.r32((l_m_1).wrapping_add(236)); F::Heap_Free(c, a0_) };
        {
            { l_i_3 = 0i32; l_buf_2 = (l_m_1).wrapping_add(88); };
            'l1: loop {
                if !(((((l_i_3) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a0_ = c.m.r32(l_buf_2); F::Heap_Free(c, a0_) };
                }
                { { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; }; { let o_ = l_buf_2; let n_ = (o_).wrapping_add(28); l_buf_2 = n_; }; };
            }
        }
        F::Heap_Free(c, l_m_1);
        { let v_ = ((0i32) as u32); let a_ = sym::gMovie; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Movie_Close", &PROBE_N, &[], true); }
    r_
}

pub fn Movie_PlayOpening(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Movie_PlayOpening", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Movie_Open(c, sym::movie__str0, sym::movie__str1);
    F::Movie_Run(c);
    F::Movie_Close(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Movie_PlayOpening", &PROBE_N, &[], true); }
    r_
}

pub fn Movie_PlayEnding(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Movie_PlayEnding", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Movie_Open(c, sym::movie__str2, sym::movie__str3);
    F::Movie_Run(c);
    F::Movie_Close(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Movie_PlayEnding", &PROBE_N, &[], true); }
    r_
}

pub fn Movie_Run(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Movie_Run", &PROBE_N, &[], false); }
    let mut l_m_1: u32 = 0;
    let mut l_mp_2: u32 = 0;
    let mut l_wait_3: i32 = 0;
    let mut l_skip_4: i32 = 0;
    let r_ = 'f: {
    l_wait_3 = 0i32;
    l_skip_4 = 0i32;
    { let a_ = sym::gMovieRunCount; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    l_m_1 = c.m.r32(sym::gMovie);
    l_mp_2 = (l_m_1).wrapping_add(16);
    { let v_ = c.m.r32((l_m_1).wrapping_add(236)); let a_ = sym::gMovieEsBuf; c.m.w32(a_, v_); };
    { let v_ = (c.m.r32((l_m_1).wrapping_add(240)) as i32); let a_ = sym::gMovieEsSize; c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(sym::gMovieEsBuf)).wrapping_add(((16i32) as u32)); let a_ = sym::gMovieEsWrite; c.m.w32(a_, v_); };
    { let v_ = c.m.r32(sym::gMovieEsWrite); let a_ = sym::gMovieEsDmaEnd; c.m.w32(a_, v_); };
    { let v_ = c.m.r32(sym::gMovieEsBuf); let a_ = sym::gMovieEsDmaStart; c.m.w32(a_, v_); };
    F::Timer_Init(c, 0i32);
    { let _ = F::Movie_FillBufs(c); };
    { let _ = F::Movie_Demux(c); };
    { let _ = F::Movie_FillBufs(c); };
    { let _ = F::sceGsSyncV(c, 0i32); };
    F::Timer_Start(c, 0i32);
    { let v_ = 1i32; let a_ = (l_m_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    F::Adx_Resume(c, 0i32);
    'l1: while ((((!((F::sceMpegIsEnd(c, l_mp_2)) != 0)) as i32)) != 0) {
        F::Timer_Reset(c, 0i32);
        F::Gfx_BeginFrame(c);
        F::Pad_Update(c);
        if ((l_skip_4) != 0) {
            if ((F::Fade_IsDone(c, 0i32)) != 0) {
                if (((((l_wait_3) >= (3i32)) as i32)) != 0) {
                    break 'l1;
                }
                { let o_ = l_wait_3; let n_ = (o_).wrapping_add(1); l_wait_3 = n_; };
            }
        } else {
            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((4096i32) as u32)))) != 0) {
                F::Fade_Start(c, 0i32, 0i32, F32(0x3f000000));
                l_skip_4 = 1i32;
            }
        }
        { let _ = { let a1_ = c.m.r32((l_m_1).wrapping_add(208)); F::sceMpegGetPicture(c, l_mp_2, a1_, (rt::div_i32(512i32, 16i32)).wrapping_mul(rt::div_i32(448i32, 16i32))) }; };
        { let _ = F::sceGsSyncPath(c, 0i32, ((0i32) as u16)); };
        { let a0_ = (if (((c.m.r32((sym::gGfx).wrapping_add(560)) as i32)) != 0) { c.m.r32(((l_m_1).wrapping_add(216)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) } else { c.m.r32(((l_m_1).wrapping_add(216)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) }); F::MovieTag_Send(c, a0_) };
        { let _ = F::sceGsSyncPath(c, 0i32, ((0i32) as u16)); };
        F::Dma_Flush(c);
        'l2: loop {
            'c3: {
                if ((((!(((rt::flt(F::Timer_GetFrames(c, 0i32), F32(0x3fe66666)) as i32)) != 0)) as i32)) != 0) {
                    break 'l2;
                }
                if ((((((((({ let a0_ = c.m.r32(l_m_1); F::ADXF_GetStat(c, a0_) }) != (2i32)) as i32)) != 0) && ((((({ let a0_ = c.m.r32((l_m_1).wrapping_add(4)); F::func_00277A70(c, a0_) }) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                    { let _ = F::Movie_ReadAhead(c, sym::gMovieTagIdle); };
                }
            }
            if !(((((F::Movie_FindBuf(c, 0i32)) != (((0i32) as u32))) as i32)) != 0) { break 'l2; }
        }
        F::Gfx_EndFrame(c, 2i32);
    }
    F::Adx_Stop(c, 0i32);
    F::Fade_Start(c, 0i32, 0i32, F32(0x00000000));
    F::Gfx_BeginFrame(c);
    F::Dma_Flush(c);
    F::Gfx_EndFrame(c, 1i32);
    F::Gfx_BeginFrame(c);
    F::Dma_Flush(c);
    F::Gfx_EndFrame(c, 1i32);
    F::Fade_Reset(c, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Movie_Run", &PROBE_N, &[], true); }
    r_
}

pub fn Movie_CbVideoData(c: &mut Ctx, mut p_mp_1: u32, mut p_str_2: u32, mut p_data_3: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Movie_CbVideoData", &PROBE_N, &[(p_mp_1) as u32, (p_str_2) as u32, (p_data_3) as u32], false); }
    let mut l_space_4: i32 = 0;
    let mut l_over_5: i32 = 0;
    let mut l_len_6: u32 = 0;
    let r_ = 'f: {
    l_space_4 = ((c.m.r32(sym::gMovieEsDmaStart)).wrapping_sub(c.m.r32(sym::gMovieEsWrite)) as i32);
    if (((((l_space_4) < (0i32)) as i32)) != 0) {
        { let r_ = (c.m.r32(sym::gMovieEsSize) as i32); let n_ = (l_space_4).wrapping_add(r_); l_space_4 = n_; };
    }
    l_len_6 = c.m.r32((p_str_2).wrapping_add(12));
    if (((((((((l_space_4) == (0i32)) as i32)) != 0) && (((((c.m.r32(sym::gMovieEsDmaStart)) == (c.m.r32(sym::gMovieEsDmaEnd))) as i32)) != 0)) as i32)) != 0) {
        l_space_4 = (c.m.r32(sym::gMovieEsSize) as i32);
    }
    if (((((((l_space_4) as u32)) < (l_len_6)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_over_5 = ((((((((c.m.r32(sym::gMovieEsWrite)).wrapping_sub(c.m.r32(sym::gMovieEsBuf)) as i32)) as u32)).wrapping_add(l_len_6)).wrapping_sub((((c.m.r32(sym::gMovieEsSize) as i32)) as u32))) as i32);
    if (((((l_over_5) > (0i32)) as i32)) != 0) {
        { let _ = { let a0_ = c.m.r32(sym::gMovieEsWrite); let a1_ = c.m.r32((p_str_2).wrapping_add(8)); F::memcpy(c, a0_, a1_, (l_len_6).wrapping_sub(((l_over_5) as u32))) }; };
        { let _ = { let a0_ = c.m.r32(sym::gMovieEsBuf); let a1_ = (c.m.r32((p_str_2).wrapping_add(8))).wrapping_add((c.m.r32((p_str_2).wrapping_add(12))).wrapping_sub(((l_over_5) as u32))); F::memcpy(c, a0_, a1_, ((l_over_5) as u32)) }; };
        { let v_ = (c.m.r32(sym::gMovieEsBuf)).wrapping_add(((l_over_5) as u32)); let a_ = sym::gMovieEsWrite; c.m.w32(a_, v_); };
    } else {
        { let _ = { let a0_ = c.m.r32(sym::gMovieEsWrite); let a1_ = c.m.r32((p_str_2).wrapping_add(8)); F::memcpy(c, a0_, a1_, l_len_6) }; };
        { let v_ = (c.m.r32(sym::gMovieEsWrite)).wrapping_add(c.m.r32((p_str_2).wrapping_add(12))); let a_ = sym::gMovieEsWrite; c.m.w32(a_, v_); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Movie_CbVideoData", &PROBE_N, &[], true); }
    r_
}

pub fn Movie_CbNoData(c: &mut Ctx, mut p_mp_1: u32, mut p_cbData_2: u32, mut p_data_3: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Movie_CbNoData", &PROBE_N, &[(p_mp_1) as u32, (p_cbData_2) as u32, (p_data_3) as u32], false); }
    let mut l_avail_4: i32 = 0;
    let mut l_size_5: i32 = 0;
    let mut l_start_6: u32 = 0;
    let mut l_end_7: u32 = 0;
    let mut l_limit_8: u32 = 0;
    let r_ = 'f: {
    { let v_ = c.m.r32(sym::gMovieEsDmaEnd); let a_ = sym::gMovieEsDmaStart; c.m.w32(a_, v_); };
    l_avail_4 = ((c.m.r32(sym::gMovieEsWrite)).wrapping_sub(c.m.r32(sym::gMovieEsDmaEnd)) as i32);
    if (((((l_avail_4) < (0i32)) as i32)) != 0) {
        { let r_ = (c.m.r32(sym::gMovieEsSize) as i32); let n_ = (l_avail_4).wrapping_add(r_); l_avail_4 = n_; };
    }
    'l1: while (((((l_avail_4) < (4096i32)) as i32)) != 0) {
        if (((((F::Movie_FindBuf(c, 1i32)) == (((0i32) as u32))) as i32)) != 0) {
            { let _ = F::Movie_ReadAhead(c, sym::gMovieTagNoData); };
        }
        if (((((F::Movie_Demux(c)) == (0i32)) as i32)) != 0) {
            { let r_ = 15i32; let n_ = (l_avail_4).wrapping_add(r_); l_avail_4 = n_; };
            break 'l1;
        }
        l_avail_4 = ((c.m.r32(sym::gMovieEsWrite)).wrapping_sub(c.m.r32(sym::gMovieEsDmaEnd)) as i32);
        if (((((l_avail_4) < (0i32)) as i32)) != 0) {
            { let r_ = (c.m.r32(sym::gMovieEsSize) as i32); let n_ = (l_avail_4).wrapping_add(r_); l_avail_4 = n_; };
        }
    }
    l_size_5 = ((l_avail_4) & ((!(15i32))));
    if (((((l_size_5) > (4096i32)) as i32)) != 0) {
        l_size_5 = 4096i32;
    }
    l_start_6 = c.m.r32(sym::gMovieEsDmaStart);
    l_end_7 = (l_start_6).wrapping_add(((l_size_5) as u32));
    l_limit_8 = (c.m.r32(sym::gMovieEsBuf)).wrapping_add((((c.m.r32(sym::gMovieEsSize) as i32)) as u32));
    if (((((l_limit_8) < (l_end_7)) as i32)) != 0) {
        l_size_5 = ((l_limit_8).wrapping_sub(l_start_6) as i32);
        l_end_7 = (l_start_6).wrapping_add(((l_size_5) as u32));
    }
    { let v_ = ((rt::div_i32(l_size_5, 16i32)) as u32); let a_ = ((268481568i32) as u32); c.m.w32(a_, v_); };
    { let v_ = l_start_6; let a_ = ((268481552i32) as u32); c.m.w32(a_, v_); };
    { let v_ = ((257i32) as u32); let a_ = F::Port_DmaChcr(c, 4i32); c.m.w32(a_, v_); };
    { let v_ = l_end_7; let a_ = sym::gMovieEsDmaEnd; c.m.w32(a_, v_); };
    if ((((!(((((c.m.r32(sym::gMovieEsDmaEnd)) < (l_limit_8)) as i32)) != 0)) as i32)) != 0) {
        { let r_ = (c.m.r32(sym::gMovieEsSize) as i32); let a_ = sym::gMovieEsDmaEnd; let n_ = (c.m.r32(a_)).wrapping_sub(((r_) as u32)); c.m.w32(a_, n_); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Movie_CbNoData", &PROBE_N, &[], true); }
    r_
}

pub fn Movie_CbBackground(c: &mut Ctx, mut p_mp_1: u32, mut p_cbData_2: u32, mut p_data_3: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Movie_CbBackground", &PROBE_N, &[(p_mp_1) as u32, (p_cbData_2) as u32, (p_data_3) as u32], false); }
    let mut l_m_4: u32 = 0;
    let r_ = 'f: {
    l_m_4 = c.m.r32(sym::gMovie);
    { let _ = F::Movie_Demux(c); };
    if ((((((((({ let a0_ = c.m.r32(l_m_4); F::ADXF_GetStat(c, a0_) }) != (2i32)) as i32)) != 0) && ((((({ let a0_ = c.m.r32((l_m_4).wrapping_add(4)); F::func_00277A70(c, a0_) }) == (0i32)) as i32)) != 0)) as i32)) != 0) {
        { let _ = F::Movie_ReadAhead(c, sym::movie__str4); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Movie_CbBackground", &PROBE_N, &[], true); }
    r_
}

pub fn Movie_CbError(c: &mut Ctx, mut p_mp_1: u32, mut p_cbData_2: u32, mut p_data_3: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Movie_CbError", &PROBE_N, &[(p_mp_1) as u32, (p_cbData_2) as u32, (p_data_3) as u32], false); }
    let r_ = 'f: {
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Movie_CbError", &PROBE_N, &[], true); }
    r_
}

pub fn Movie_FindBuf(c: &mut Ctx, mut p_full_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Movie_FindBuf", &PROBE_N, &[(p_full_1) as u32], false); }
    let mut l_m_2: u32 = 0;
    let mut l_best_3: u32 = 0;
    let mut l_buf_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_end_6: i32 = 0;
    let r_ = 'f: {
    l_m_2 = c.m.r32(sym::gMovie);
    l_best_3 = ((0i32) as u32);
    l_buf_4 = (l_m_2).wrapping_add(88);
    if ((p_full_1) != 0) {
        {
            l_i_5 = 0i32;
            'l1: loop {
                if !(((((l_i_5) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((c.m.r32((l_buf_4).wrapping_add(4)) as i32)) != 0) {
                        if (((((((((l_best_3) == (((0i32) as u32))) as i32)) != 0) || ((((((c.m.r32((l_buf_4).wrapping_add(24)) as i32)) < ((c.m.r32((l_best_3).wrapping_add(24)) as i32))) as i32)) != 0)) as i32)) != 0) {
                            l_best_3 = l_buf_4;
                        }
                    }
                }
                { { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; }; { let o_ = l_buf_4; let n_ = (o_).wrapping_add(28); l_buf_4 = n_; }; };
            }
        }
        if (((((l_best_3) != (((0i32) as u32))) as i32)) != 0) {
            l_end_6 = ((c.m.r32((l_best_3).wrapping_add(8)) as i32)).wrapping_add((c.m.r32((l_best_3).wrapping_add(12)) as i32));
            if ((((({ let a0_ = c.m.r32(l_m_2); F::ADXF_Tell(c, a0_) }) < (l_end_6)) as i32)) != 0) {
                F::Movie_WaitRead(c, l_end_6);
            }
        }
    } else {
        {
            l_i_5 = 0i32;
            'l3: loop {
                if !(((((l_i_5) < (4i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    if ((((!(((c.m.r32((l_buf_4).wrapping_add(4)) as i32)) != 0)) as i32)) != 0) {
                        l_best_3 = l_buf_4;
                        break 'l3;
                    }
                }
                { { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; }; { let o_ = l_buf_4; let n_ = (o_).wrapping_add(28); l_buf_4 = n_; }; };
            }
        }
    }
    break 'f l_best_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Movie_FindBuf", &PROBE_N, &[], true); }
    r_
}

pub fn Movie_ReadBuf(c: &mut Ctx, mut p_buf_1: u32, mut p_tag_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Movie_ReadBuf", &PROBE_N, &[(p_buf_1) as u32, (p_tag_2) as u32], false); }
    let mut l_m_3: u32 = 0;
    let mut l_pos_4: i32 = 0;
    let mut l_n_5: i32 = 0;
    let r_ = 'f: {
    l_m_3 = c.m.r32(sym::gMovie);
    if (((((p_buf_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    F::Movie_WaitRead(c, (1i32).wrapping_neg());
    l_pos_4 = { let a0_ = c.m.r32(l_m_3); F::ADXF_Tell(c, a0_) };
    l_n_5 = { let a0_ = c.m.r32(l_m_3); let a2_ = c.m.r32(p_buf_1); F::ADXF_ReadNw(c, a0_, 256i32, a2_) };
    if (((((l_n_5) == (0i32)) as i32)) != 0) {
        break 'f l_n_5;
    }
    F::ADXM_ExecMain(c);
    'l1: loop {
        'c2: {
            { let v_ = 1i32; let a_ = (p_buf_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            { let v_ = l_pos_4; let a_ = (p_buf_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (p_buf_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
            { let v_ = l_n_5; let a_ = (p_buf_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
            { let v_ = (l_n_5).wrapping_shl((11i32) as u32); let a_ = (p_buf_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
            { let v_ = (c.m.r32((l_m_3).wrapping_add(204)) as i32); let a_ = (p_buf_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
        }
        if !((0i32) != 0) { break 'l1; }
    }
    { let a_ = (l_m_3).wrapping_add(204); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    break 'f (l_n_5).wrapping_shl((11i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Movie_ReadBuf", &PROBE_N, &[], true); }
    r_
}

pub fn Movie_WaitRead(c: &mut Ctx, mut p_sector_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Movie_WaitRead", &PROBE_N, &[(p_sector_1) as u32], false); }
    let mut l_m_2: u32 = 0;
    let r_ = 'f: {
    l_m_2 = c.m.r32(sym::gMovie);
    'l1: loop {
        'c2: {
            if (((c.m.r32((l_m_2).wrapping_add(8)) as i32)) != 0) {
                's3: {
                    let sv3: i32 = { let a0_ = c.m.r32((l_m_2).wrapping_add(4)); F::ADXT_GetStat(c, a0_) };
                    match sv3 {
                        3i32 | 4i32 => {
                            if (((rt::fgt(F::Timer_GetFrames(c, 0i32), F32(0x40800000)) as i32)) != 0) {
                                { let a0_ = c.m.r32((l_m_2).wrapping_add(4)); F::ADXT_Pause(c, a0_, 1i32) };
                            }
                            break 's3;
                        }
                        _ => {
                        }
                    }
                }
            }
        }
        if !(((if (((((p_sector_1) < (0i32)) as i32)) != 0) { ((({ let a0_ = c.m.r32(l_m_2); F::ADXF_GetStat(c, a0_) }) == (2i32)) as i32) } else { ((({ let a0_ = c.m.r32(l_m_2); F::ADXF_Tell(c, a0_) }) < (p_sector_1)) as i32) })) != 0) { break 'l1; }
    }
    if (((c.m.r32((l_m_2).wrapping_add(8)) as i32)) != 0) {
        { let a0_ = c.m.r32((l_m_2).wrapping_add(4)); F::ADXT_Pause(c, a0_, 0i32) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Movie_WaitRead", &PROBE_N, &[], true); }
    r_
}

pub fn Movie_ReadAhead(c: &mut Ctx, mut p_tag_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Movie_ReadAhead", &PROBE_N, &[(p_tag_1) as u32], false); }
    let mut l_buf_2: u32 = 0;
    let mut l_ret_3: i32 = 0;
    let r_ = 'f: {
    l_buf_2 = F::Movie_FindBuf(c, 0i32);
    l_ret_3 = 0i32;
    if (((((l_buf_2) != (((0i32) as u32))) as i32)) != 0) {
        l_ret_3 = F::Movie_ReadBuf(c, l_buf_2, p_tag_1);
    }
    break 'f l_ret_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Movie_ReadAhead", &PROBE_N, &[], true); }
    r_
}

pub fn Movie_FillBufs(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Movie_FillBufs", &PROBE_N, &[], false); }
    let mut l_total_1: i32 = 0;
    let mut l_buf_2: u32 = 0;
    let r_ = 'f: {
    l_total_1 = 0i32;
    'l1: while ((((({ l_buf_2 = F::Movie_FindBuf(c, 0i32); l_buf_2 }) != (((0i32) as u32))) as i32)) != 0) {
        { let r_ = F::Movie_ReadBuf(c, l_buf_2, sym::gMovieTagFull); let n_ = (l_total_1).wrapping_add(r_); l_total_1 = n_; };
    }
    break 'f l_total_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Movie_FillBufs", &PROBE_N, &[], true); }
    r_
}

pub fn Movie_Demux(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Movie_Demux", &PROBE_N, &[], false); }
    let mut l_buf_1: u32 = 0;
    let mut l_len_2: i32 = 0;
    let mut l_n_3: i32 = 0;
    let mut l_total_4: i32 = 0;
    let mut l_mp_5: u32 = 0;
    let r_ = 'f: {
    l_mp_5 = (c.m.r32(sym::gMovie)).wrapping_add(16);
    l_total_4 = 0i32;
    'l1: loop {
        'c2: {
            l_buf_1 = F::Movie_FindBuf(c, 1i32);
            if (((((l_buf_1) == (((0i32) as u32))) as i32)) != 0) {
                break 'l1;
            }
            l_len_2 = ((c.m.r32((l_buf_1).wrapping_add(16)) as i32)).wrapping_sub((c.m.r32((l_buf_1).wrapping_add(20)) as i32));
            l_n_3 = { let a1_ = (c.m.r32(l_buf_1)).wrapping_add((((c.m.r32((l_buf_1).wrapping_add(20)) as i32)) as u32)); F::sceMpegDemuxPss(c, l_mp_5, a1_, l_len_2) };
            { let r_ = l_n_3; let n_ = (l_total_4).wrapping_add(r_); l_total_4 = n_; };
            if (((((l_n_3) >= (l_len_2)) as i32)) != 0) {
                { let v_ = 0i32; let a_ = (l_buf_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                break 'c2;
            }
            { let r_ = l_n_3; let a_ = (l_buf_1).wrapping_add(20); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
            break 'l1;
        }
        if !((1i32) != 0) { break 'l1; }
    }
    break 'f l_total_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Movie_Demux", &PROBE_N, &[], true); }
    r_
}
