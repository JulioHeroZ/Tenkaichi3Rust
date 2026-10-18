#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EftPtcl_PickTexture(c: &mut Ctx, mut p_w_1: u32, mut p_tex_2: u32, mut p_a_3: i32, mut p_b_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_PickTexture", &PROBE_N, &[(p_w_1) as u32, (p_tex_2) as u32, (p_a_3) as u32, (p_b_4) as u32], false); }
    let r_ = 'f: {
    { let s_ = (p_tex_2).wrapping_add((((p_a_3) as u32)).wrapping_mul(16)); let d_ = (p_w_1).wrapping_add(552); rt::memcpy(c, d_, s_, 16); };
    { let s_ = (p_tex_2).wrapping_add((((p_b_4) as u32)).wrapping_mul(16)); let d_ = (p_w_1).wrapping_add(568); rt::memcpy(c, d_, s_, 16); };
    { let v_ = p_b_4; let a_ = (p_w_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_PickTexture", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtcl_UploadTexture(c: &mut Ctx, mut p_w_1: u32, mut p_w2_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_UploadTexture", &PROBE_N, &[(p_w_1) as u32, (p_w2_2) as u32], false); }
    let r_ = 'f: {
    if ((((!(((({ let a_ = (c.m.r32((p_w2_2).wrapping_add(48))).wrapping_add(260); c.m.r32(a_) }) & ((1u32).wrapping_shl(((c.m.r32((p_w2_2).wrapping_add(36)) as i32)) as u32)))) != 0)) as i32)) != 0) {
        { let v_ = F::EftVram_AddImage(c, (p_w_1).wrapping_add(552), 1i32, 0i32); let a_ = (p_w_1).wrapping_add(584); c.m.w64(a_, v_); };
        { let r_ = (F::EftVram_AddClut(c, (p_w_1).wrapping_add(568))).wrapping_shl((37i32) as u32); let a_ = (p_w_1).wrapping_add(584); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
        { let v_ = c.m.r64((p_w_1).wrapping_add(584)); let a_ = (c.m.r32((p_w2_2).wrapping_add(48))).wrapping_add(((((c.m.r32((p_w2_2).wrapping_add(36)) as i32)) as u32)).wrapping_mul(16)); c.m.w64(a_, v_); };
        { let r_ = (1u32).wrapping_shl(((c.m.r32((p_w2_2).wrapping_add(36)) as i32)) as u32); let a_ = (c.m.r32((p_w2_2).wrapping_add(48))).wrapping_add(260); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    } else {
        { let v_ = { let a_ = (c.m.r32((p_w2_2).wrapping_add(48))).wrapping_add(((((c.m.r32((p_w2_2).wrapping_add(36)) as i32)) as u32)).wrapping_mul(16)); c.m.r64(a_) }; let a_ = (p_w_1).wrapping_add(584); c.m.w64(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_UploadTexture", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtcl_SetKey(c: &mut Ctx, mut p_cur_1: u32, mut p_w_2: u32, mut p_key_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_SetKey", &PROBE_N, &[(p_cur_1) as u32, (p_w_2) as u32, (p_key_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_def2_4: u32 = 0;
    let mut l_def_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let r_ = 'f: {
        l_def2_4 = c.m.r32((p_w_2).wrapping_add(56));
        l_def_5 = c.m.r32((p_w_2).wrapping_add(52));
        F::Vec4_Copy(c, p_cur_1, (l_def2_4).wrapping_add((((p_key_3) as u32)).wrapping_mul(16)));
        F::Vec4_Copy(c, (p_cur_1).wrapping_add(16), ((l_def2_4).wrapping_add(48)).wrapping_add((((p_key_3) as u32)).wrapping_mul(16)));
        F::Vec4_Copy(c, (p_cur_1).wrapping_add(32), ((l_def2_4).wrapping_add(96)).wrapping_add((((p_key_3) as u32)).wrapping_mul(16)));
        F::Vec4_Copy(c, (p_cur_1).wrapping_add(48), ((l_def2_4).wrapping_add(144)).wrapping_add((((p_key_3) as u32)).wrapping_mul(16)));
        {
            l_i_6 = 0i32;
            'l1: loop {
                if !(((((l_i_6) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = F32(c.m.r32((((l_def2_4).wrapping_add(192)).wrapping_add((((p_key_3) as u32)).wrapping_mul(8))).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)))); let a_ = (fp).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((((l_def2_4).wrapping_add(216)).wrapping_add((((p_key_3) as u32)).wrapping_mul(8))).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)))); let a_ = (fp.wrapping_add(16)).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((((l_def2_4).wrapping_add(240)).wrapping_add((((p_key_3) as u32)).wrapping_mul(8))).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)))); let a_ = (fp.wrapping_add(32)).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
        { let v_ = F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_cur_1).wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_cur_1).wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_cur_1).wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((p_cur_1).wrapping_add(76)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((p_cur_1).wrapping_add(76)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((p_cur_1).wrapping_add(76)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_def2_4).wrapping_add(264)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x41f00000)); let a_ = (p_cur_1).wrapping_add(88); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f(((c.m.r16(((l_def_5).wrapping_add(28)).wrapping_add((((p_key_3) as u32)).wrapping_mul(2))) as i16)) as i32); let a_ = (p_cur_1).wrapping_add(92); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f(((c.m.r16(((l_def_5).wrapping_add(34)).wrapping_add((((p_key_3) as u32)).wrapping_mul(2))) as i16)) as i32); let a_ = (p_cur_1).wrapping_add(96); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_def_5).wrapping_add(40)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x41f00000)); let a_ = (p_cur_1).wrapping_add(100); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_def_5).wrapping_add(52)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x41f00000)); let a_ = (p_cur_1).wrapping_add(104); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_def_5).wrapping_add(64)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x41f00000)); let a_ = (p_cur_1).wrapping_add(108); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_def_5).wrapping_add(76)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (p_cur_1).wrapping_add(112); c.m.w32(a_, (v_).0); };
        {
            l_i_6 = 0i32;
            'l3: loop {
                if !(((((l_i_6) < (3i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let v_ = F32(c.m.r32((((l_def_5).wrapping_add(88)).wrapping_add((((p_key_3) as u32)).wrapping_mul(12))).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)))); let a_ = ((p_cur_1).wrapping_add(116)).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((((l_def_5).wrapping_add(124)).wrapping_add((((p_key_3) as u32)).wrapping_mul(12))).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)))); let a_ = ((p_cur_1).wrapping_add(128)).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((((l_def_5).wrapping_add(160)).wrapping_add((((p_key_3) as u32)).wrapping_mul(12))).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)))); let a_ = ((p_cur_1).wrapping_add(140)).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((((l_def_5).wrapping_add(196)).wrapping_add((((p_key_3) as u32)).wrapping_mul(12))).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)))); let a_ = ((p_cur_1).wrapping_add(152)).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((((l_def_5).wrapping_add(232)).wrapping_add((((p_key_3) as u32)).wrapping_mul(12))).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)))); let a_ = ((p_cur_1).wrapping_add(164)).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((((l_def_5).wrapping_add(268)).wrapping_add((((p_key_3) as u32)).wrapping_mul(12))).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)))); let a_ = ((p_cur_1).wrapping_add(176)).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
        { let v_ = F32(c.m.r32(((l_def_5).wrapping_add(304)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_def_5).wrapping_add(316)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_cur_1).wrapping_add(188); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (p_cur_1).wrapping_add(192); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_def_5).wrapping_add(328)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x41f00000)); let a_ = (p_cur_1).wrapping_add(196); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_def_5).wrapping_add(340)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_def_5).wrapping_add(352)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_cur_1).wrapping_add(200); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (p_cur_1).wrapping_add(204); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_def_5).wrapping_add(364)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x41f00000)); let a_ = (p_cur_1).wrapping_add(208); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_def_5).wrapping_add(544)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x41f00000)); let a_ = (p_cur_1).wrapping_add(268); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_def_5).wrapping_add(556)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x41f00000)); let a_ = (p_cur_1).wrapping_add(272); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_def_5).wrapping_add(376)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (p_cur_1).wrapping_add(212); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_def_5).wrapping_add(388)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (p_cur_1).wrapping_add(216); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_def_5).wrapping_add(400)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (p_cur_1).wrapping_add(220); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_def_5).wrapping_add(412)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (p_cur_1).wrapping_add(224); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_def_5).wrapping_add(424)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (p_cur_1).wrapping_add(228); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_def_5).wrapping_add(436)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (p_cur_1).wrapping_add(232); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_def_5).wrapping_add(448)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (p_cur_1).wrapping_add(236); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_def_5).wrapping_add(460)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x40490fda)); let a_ = (p_cur_1).wrapping_add(240); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_def_5).wrapping_add(472)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x40490fda)); let a_ = (p_cur_1).wrapping_add(244); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_def_5).wrapping_add(484)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x40490fda)); let a_ = (p_cur_1).wrapping_add(248); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_def_5).wrapping_add(496)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x40490fda)); let a_ = (p_cur_1).wrapping_add(252); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_def_5).wrapping_add(508)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (p_cur_1).wrapping_add(256); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_def_5).wrapping_add(520)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (p_cur_1).wrapping_add(260); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((l_def_5).wrapping_add(532)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))); let a_ = (p_cur_1).wrapping_add(264); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_def_5).wrapping_add(568)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x40490fda)); let a_ = (p_cur_1).wrapping_add(276); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_def_5).wrapping_add(580)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x40490fda)); let a_ = (p_cur_1).wrapping_add(280); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_def_5).wrapping_add(592)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x40490fda)); let a_ = (p_cur_1).wrapping_add(284); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_def_5).wrapping_add(604)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x40490fda)); let a_ = (p_cur_1).wrapping_add(288); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_def_5).wrapping_add(616)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x40490fda)); let a_ = (p_cur_1).wrapping_add(292); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(((l_def_5).wrapping_add(628)).wrapping_add((((p_key_3) as u32)).wrapping_mul(4)))), F32(0x40490fda)); let a_ = (p_cur_1).wrapping_add(296); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_SetKey", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtcl_BlendKeys(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_BlendKeys", &PROBE_N, &[(p_w_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_k0_2: i32 = 0;
    let mut l_k1_3: i32 = 0;
    let mut l_cur_4: u32 = 0;
    let mut l_def_5: u32 = 0;
    let mut l_def2_6: u32 = 0;
    let mut l_t_7: F32 = F32(0);
    let mut l_i_8: i32 = 0;
    let r_ = 'f: {
        l_cur_4 = (p_w_1).wrapping_add(64);
        l_def_5 = c.m.r32((p_w_1).wrapping_add(52));
        l_def2_6 = c.m.r32((p_w_1).wrapping_add(56));
        if (((rt::flt(F32(c.m.r32((p_w_1).wrapping_add(524))), F32(c.m.r32((p_w_1).wrapping_add(532)))) as i32)) != 0) {
            l_t_7 = rt::fdiv(F32(c.m.r32((p_w_1).wrapping_add(524))), F32(c.m.r32((p_w_1).wrapping_add(532))));
            l_k0_2 = 0i32;
            l_k1_3 = 1i32;
        } else {
            l_t_7 = rt::fsub(F32(c.m.r32((p_w_1).wrapping_add(524))), F32(c.m.r32((p_w_1).wrapping_add(532))));
            { let r_ = rt::fsub(F32(c.m.r32((p_w_1).wrapping_add(528))), F32(c.m.r32((p_w_1).wrapping_add(532)))); let n_ = rt::fdiv(l_t_7, r_); l_t_7 = n_; };
            l_k0_2 = 1i32;
            l_k1_3 = 2i32;
        }
        F::Vec4_Sub(c, fp, (l_def2_6).wrapping_add((((l_k1_3) as u32)).wrapping_mul(16)), (l_def2_6).wrapping_add((((l_k0_2) as u32)).wrapping_mul(16)));
        F::Vec4_Scale(c, fp, fp, l_t_7);
        F::Vec4_Add(c, l_cur_4, (l_def2_6).wrapping_add((((l_k0_2) as u32)).wrapping_mul(16)), fp);
        F::Vec4_Sub(c, fp, ((l_def2_6).wrapping_add(48)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(16)), ((l_def2_6).wrapping_add(48)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(16)));
        F::Vec4_Scale(c, fp, fp, l_t_7);
        F::Vec4_Add(c, (l_cur_4).wrapping_add(16), ((l_def2_6).wrapping_add(48)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(16)), fp);
        F::Vec4_Sub(c, fp, ((l_def2_6).wrapping_add(96)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(16)), ((l_def2_6).wrapping_add(96)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(16)));
        F::Vec4_Scale(c, fp, fp, l_t_7);
        F::Vec4_Add(c, (l_cur_4).wrapping_add(32), ((l_def2_6).wrapping_add(96)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(16)), fp);
        F::Vec4_Sub(c, fp, ((l_def2_6).wrapping_add(144)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(16)), ((l_def2_6).wrapping_add(144)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(16)));
        F::Vec4_Scale(c, fp, fp, l_t_7);
        F::Vec4_Add(c, (l_cur_4).wrapping_add(48), ((l_def2_6).wrapping_add(144)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(16)), fp);
        {
            l_i_8 = 0i32;
            'l1: loop {
                if !(((((l_i_8) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = rt::fsub(F32(c.m.r32((((l_def2_6).wrapping_add(192)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(8))).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def2_6).wrapping_add(192)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(8))).wrapping_add((((l_i_8) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(F32(c.m.r32((((l_def2_6).wrapping_add(216)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(8))).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def2_6).wrapping_add(216)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(8))).wrapping_add((((l_i_8) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(F32(c.m.r32((((l_def2_6).wrapping_add(240)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(8))).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def2_6).wrapping_add(240)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(8))).wrapping_add((((l_i_8) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32((((l_def2_6).wrapping_add(192)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(8))).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_7)); let a_ = (fp.wrapping_add(16)).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32((((l_def2_6).wrapping_add(216)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(8))).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_7)); let a_ = (fp.wrapping_add(32)).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32((((l_def2_6).wrapping_add(240)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(8))).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_t_7)); let a_ = (fp.wrapping_add(48)).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
            }
        }
        { let v_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((l_cur_4).wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((l_cur_4).wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((l_cur_4).wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((l_cur_4).wrapping_add(76)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((l_cur_4).wrapping_add(76)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((l_cur_4).wrapping_add(76)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def2_6).wrapping_add(264)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def2_6).wrapping_add(264)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_def2_6).wrapping_add(264)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_7)), F32(0x41f00000)); let a_ = (l_cur_4).wrapping_add(88); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f(((((c.m.r16(((l_def_5).wrapping_add(28)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(2))) as i16)) as i32)).wrapping_sub((((c.m.r16(((l_def_5).wrapping_add(28)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(2))) as i16)) as i32))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f(((((c.m.r16(((l_def_5).wrapping_add(34)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(2))) as i16)) as i32)).wrapping_sub((((c.m.r16(((l_def_5).wrapping_add(34)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(2))) as i16)) as i32))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(rt::i2f((((c.m.r16(((l_def_5).wrapping_add(28)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(2))) as i16)) as i32)), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_7)); let a_ = (l_cur_4).wrapping_add(92); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(rt::i2f((((c.m.r16(((l_def_5).wrapping_add(34)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(2))) as i16)) as i32)), rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_7)); let a_ = (l_cur_4).wrapping_add(96); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_5).wrapping_add(40)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_5).wrapping_add(40)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_def_5).wrapping_add(40)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_7)), F32(0x41f00000)); let a_ = (l_cur_4).wrapping_add(100); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_5).wrapping_add(52)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_5).wrapping_add(52)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_5).wrapping_add(64)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_5).wrapping_add(64)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_5).wrapping_add(76)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_5).wrapping_add(76)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_def_5).wrapping_add(52)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_7)), F32(0x41f00000)); let a_ = (l_cur_4).wrapping_add(104); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_def_5).wrapping_add(64)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_7)), F32(0x41f00000)); let a_ = (l_cur_4).wrapping_add(108); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_5).wrapping_add(76)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_t_7)); let a_ = (l_cur_4).wrapping_add(112); c.m.w32(a_, (v_).0); };
        {
            l_i_8 = 0i32;
            'l3: loop {
                if !(((((l_i_8) < (3i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let v_ = rt::fsub(F32(c.m.r32((((l_def_5).wrapping_add(88)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(12))).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_5).wrapping_add(88)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(12))).wrapping_add((((l_i_8) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(F32(c.m.r32((((l_def_5).wrapping_add(124)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(12))).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_5).wrapping_add(124)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(12))).wrapping_add((((l_i_8) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32((((l_def_5).wrapping_add(88)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(12))).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_7)); let a_ = ((l_cur_4).wrapping_add(116)).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32((((l_def_5).wrapping_add(124)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(12))).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_7)); let a_ = ((l_cur_4).wrapping_add(128)).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(F32(c.m.r32((((l_def_5).wrapping_add(160)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(12))).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_5).wrapping_add(160)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(12))).wrapping_add((((l_i_8) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(F32(c.m.r32((((l_def_5).wrapping_add(196)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(12))).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_5).wrapping_add(196)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(12))).wrapping_add((((l_i_8) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32((((l_def_5).wrapping_add(160)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(12))).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_7)); let a_ = ((l_cur_4).wrapping_add(140)).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32((((l_def_5).wrapping_add(196)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(12))).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_7)); let a_ = ((l_cur_4).wrapping_add(152)).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(F32(c.m.r32((((l_def_5).wrapping_add(232)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(12))).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_5).wrapping_add(232)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(12))).wrapping_add((((l_i_8) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(F32(c.m.r32((((l_def_5).wrapping_add(268)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(12))).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_5).wrapping_add(268)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(12))).wrapping_add((((l_i_8) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32((((l_def_5).wrapping_add(232)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(12))).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_7)); let a_ = ((l_cur_4).wrapping_add(164)).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32((((l_def_5).wrapping_add(268)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(12))).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_7)); let a_ = ((l_cur_4).wrapping_add(176)).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
            }
        }
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_5).wrapping_add(304)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_5).wrapping_add(304)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_5).wrapping_add(316)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_5).wrapping_add(316)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_5).wrapping_add(328)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_5).wrapping_add(328)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_5).wrapping_add(304)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_7)); let a_ = (fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_5).wrapping_add(316)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_7)); let a_ = (fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (l_cur_4).wrapping_add(188); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (l_cur_4).wrapping_add(192); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_def_5).wrapping_add(328)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_t_7)), F32(0x41f00000)); let a_ = (l_cur_4).wrapping_add(196); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_5).wrapping_add(340)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_5).wrapping_add(340)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_5).wrapping_add(352)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_5).wrapping_add(352)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_5).wrapping_add(364)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_5).wrapping_add(364)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_5).wrapping_add(340)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_7)); let a_ = (fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_5).wrapping_add(352)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_7)); let a_ = (fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (l_cur_4).wrapping_add(200); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (l_cur_4).wrapping_add(204); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_def_5).wrapping_add(364)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_t_7)), F32(0x41f00000)); let a_ = (l_cur_4).wrapping_add(208); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_5).wrapping_add(544)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_5).wrapping_add(544)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_5).wrapping_add(556)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_5).wrapping_add(556)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_def_5).wrapping_add(544)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_7)), F32(0x41f00000)); let a_ = (l_cur_4).wrapping_add(268); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_def_5).wrapping_add(556)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_7)), F32(0x41f00000)); let a_ = (l_cur_4).wrapping_add(272); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_5).wrapping_add(376)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_5).wrapping_add(376)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_5).wrapping_add(388)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_5).wrapping_add(388)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_5).wrapping_add(376)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_7)); let a_ = (l_cur_4).wrapping_add(212); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_5).wrapping_add(388)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_7)); let a_ = (l_cur_4).wrapping_add(216); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_5).wrapping_add(400)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_5).wrapping_add(400)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_5).wrapping_add(412)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_5).wrapping_add(412)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_5).wrapping_add(400)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_7)); let a_ = (l_cur_4).wrapping_add(220); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_5).wrapping_add(412)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_7)); let a_ = (l_cur_4).wrapping_add(224); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_5).wrapping_add(424)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_5).wrapping_add(424)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_5).wrapping_add(436)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_5).wrapping_add(436)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_5).wrapping_add(424)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_7)); let a_ = (l_cur_4).wrapping_add(228); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_5).wrapping_add(436)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_7)); let a_ = (l_cur_4).wrapping_add(232); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_5).wrapping_add(448)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_5).wrapping_add(448)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_5).wrapping_add(448)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_7)); let a_ = (l_cur_4).wrapping_add(236); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_5).wrapping_add(460)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_5).wrapping_add(460)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_5).wrapping_add(472)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_5).wrapping_add(472)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_def_5).wrapping_add(460)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_7)), F32(0x40490fda)); let a_ = (l_cur_4).wrapping_add(240); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_def_5).wrapping_add(472)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_7)), F32(0x40490fda)); let a_ = (l_cur_4).wrapping_add(244); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_5).wrapping_add(484)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_5).wrapping_add(484)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_5).wrapping_add(496)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_5).wrapping_add(496)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_def_5).wrapping_add(484)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_7)), F32(0x40490fda)); let a_ = (l_cur_4).wrapping_add(248); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_def_5).wrapping_add(496)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_7)), F32(0x40490fda)); let a_ = (l_cur_4).wrapping_add(252); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_5).wrapping_add(508)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_5).wrapping_add(508)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_5).wrapping_add(520)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_5).wrapping_add(520)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_5).wrapping_add(508)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_7)); let a_ = (l_cur_4).wrapping_add(256); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_5).wrapping_add(520)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_7)); let a_ = (l_cur_4).wrapping_add(260); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_5).wrapping_add(532)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_5).wrapping_add(532)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_5).wrapping_add(532)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_7)); let a_ = (l_cur_4).wrapping_add(264); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_5).wrapping_add(568)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_5).wrapping_add(568)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_5).wrapping_add(580)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_5).wrapping_add(580)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_def_5).wrapping_add(568)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_7)), F32(0x40490fda)); let a_ = (l_cur_4).wrapping_add(276); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_def_5).wrapping_add(580)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_7)), F32(0x40490fda)); let a_ = (l_cur_4).wrapping_add(280); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_5).wrapping_add(592)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_5).wrapping_add(592)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_5).wrapping_add(604)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_5).wrapping_add(604)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_def_5).wrapping_add(592)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_7)), F32(0x40490fda)); let a_ = (l_cur_4).wrapping_add(284); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_def_5).wrapping_add(604)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_7)), F32(0x40490fda)); let a_ = (l_cur_4).wrapping_add(288); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_5).wrapping_add(616)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_5).wrapping_add(616)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_5).wrapping_add(628)).wrapping_add((((l_k1_3) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_5).wrapping_add(628)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_def_5).wrapping_add(616)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_7)), F32(0x40490fda)); let a_ = (l_cur_4).wrapping_add(292); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_def_5).wrapping_add(628)).wrapping_add((((l_k0_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_7)), F32(0x40490fda)); let a_ = (l_cur_4).wrapping_add(296); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_BlendKeys", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtcl_AllocPtcl(c: &mut Ctx, mut p_w_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_AllocPtcl", &PROBE_N, &[(p_w_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_n_4: i32 = 0;
    let r_ = 'f: {
    l_p_2 = ((0i32) as u32);
    l_i_3 = { let a_ = (c.m.r32(sym::gEftPtcl)).wrapping_add(68); (c.m.r32(a_) as i32) };
    {
        l_n_4 = 0i32;
        'l1: loop {
            if !(((((l_n_4) < (500i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gEftPtcl)).wrapping_add(64); c.m.r32(a_) }).wrapping_add((((l_i_3) as u32)).wrapping_mul(304))).wrapping_add(160); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    l_p_2 = ({ let a_ = (c.m.r32(sym::gEftPtcl)).wrapping_add(64); c.m.r32(a_) }).wrapping_add((((l_i_3) as u32)).wrapping_mul(304));
                    { let _ = F::memset(c, l_p_2, 0i32, 304u32); };
                    { let v_ = (l_i_3).wrapping_add(1i32); let a_ = (c.m.r32(sym::gEftPtcl)).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
                    if ((((({ let a_ = (c.m.r32(sym::gEftPtcl)).wrapping_add(68); (c.m.r32(a_) as i32) }) >= (500i32)) as i32)) != 0) {
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gEftPtcl)).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
                    }
                    { let a_ = (c.m.r32(sym::gEftPtcl)).wrapping_add(72); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
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
    if (((((l_p_2) != (((0i32) as u32))) as i32)) != 0) {
        if (((((c.m.r32((p_w_1).wrapping_add(464))) == (((0i32) as u32))) as i32)) != 0) {
            { let v_ = l_p_2; let a_ = (p_w_1).wrapping_add(464); c.m.w32(a_, v_); };
        }
        if (((((c.m.r32((p_w_1).wrapping_add(468))) == (((0i32) as u32))) as i32)) != 0) {
            { let v_ = l_p_2; let a_ = (p_w_1).wrapping_add(468); c.m.w32(a_, v_); };
        } else {
            { let v_ = c.m.r32((p_w_1).wrapping_add(468)); let a_ = (l_p_2).wrapping_add(288); c.m.w32(a_, v_); };
            { let v_ = l_p_2; let a_ = (c.m.r32((p_w_1).wrapping_add(468))).wrapping_add(284); c.m.w32(a_, v_); };
            { let v_ = l_p_2; let a_ = (p_w_1).wrapping_add(468); c.m.w32(a_, v_); };
        }
    }
    break 'f l_p_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_AllocPtcl", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtcl_UnlinkPtcl(c: &mut Ctx, mut p_w_1: u32, mut p_p_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_UnlinkPtcl", &PROBE_N, &[(p_w_1) as u32, (p_p_2) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32((p_p_2).wrapping_add(288))) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = c.m.r32((p_p_2).wrapping_add(284)); let a_ = (c.m.r32((p_p_2).wrapping_add(288))).wrapping_add(284); c.m.w32(a_, v_); };
    } else {
        { let v_ = c.m.r32((p_p_2).wrapping_add(284)); let a_ = (p_w_1).wrapping_add(464); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32((p_p_2).wrapping_add(284))) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = c.m.r32((p_p_2).wrapping_add(288)); let a_ = (c.m.r32((p_p_2).wrapping_add(284))).wrapping_add(288); c.m.w32(a_, v_); };
    } else {
        { let v_ = c.m.r32((p_p_2).wrapping_add(288)); let a_ = (p_w_1).wrapping_add(468); c.m.w32(a_, v_); };
    }
    { let a_ = (c.m.r32(sym::gEftPtcl)).wrapping_add(72); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_UnlinkPtcl", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtcl_FreePtcls(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_FreePtcls", &PROBE_N, &[(p_w_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let mut l_next_3: u32 = 0;
    let r_ = 'f: {
    l_p_2 = c.m.r32((p_w_1).wrapping_add(464));
    if (((((l_p_2) != (((0i32) as u32))) as i32)) != 0) {
        'l1: loop {
            'c2: {
                { let v_ = 0i32; let a_ = (l_p_2).wrapping_add(160); c.m.w32(a_, (v_) as u32); };
                { let a_ = (c.m.r32(sym::gEftPtcl)).wrapping_add(72); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                l_next_3 = c.m.r32((l_p_2).wrapping_add(284));
                l_p_2 = l_next_3;
            }
            if !(((((l_next_3) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
        }
    }
    { let v_ = ((0i32) as u32); let a_ = (p_w_1).wrapping_add(464); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (p_w_1).wrapping_add(468); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_FreePtcls", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtcl_Spawn(c: &mut Ctx, mut p_w_1: u32, mut p_w2_2: u32, mut p_count_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_Spawn", &PROBE_N, &[(p_w_1) as u32, (p_w2_2) as u32, (p_count_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_cur_4: u32 = 0;
    let mut l_def_5: u32 = 0;
    let mut l_n_6: i32 = 0;
    let mut l_p_7: u32 = 0;
    let mut l_a_8: F32 = F32(0);
    let mut l_t_9: F32 = F32(0);
    let mut l_d_10: F32 = F32(0);
    let mut l_spd_11: F32 = F32(0);
    let mut l_k_12: F32 = F32(0);
    let mut l_from_13: F32 = F32(0);
    let mut l_to_14: F32 = F32(0);
    let mut l_life_15: F32 = F32(0);
    let mut l_i_16: i32 = 0;
    let r_ = 'f: {
        l_cur_4 = (p_w_1).wrapping_add(64);
        l_def_5 = c.m.r32((p_w2_2).wrapping_add(52));
        l_n_6 = p_count_3;
        'l1: while (((((l_n_6) > (0i32)) as i32)) != 0) {
            l_p_7 = F::EftPtcl_AllocPtcl(c, p_w_1);
            if (((((l_p_7) == (((0i32) as u32))) as i32)) != 0) {
                break 'f 0i32;
            }
            { let r_ = 1i32; let a_ = (l_p_7).wrapping_add(160); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            l_life_15 = rt::fadd(F32(c.m.r32((l_cur_4).wrapping_add(104))), rt::fmul(F32(c.m.r32((l_cur_4).wrapping_add(108))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff))));
            { let v_ = F32(0x00000000); let a_ = (l_p_7).wrapping_add(168); c.m.w32(a_, (v_).0); };
            { let v_ = l_life_15; let a_ = (l_p_7).wrapping_add(172); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul(F32(c.m.r32((l_p_7).wrapping_add(172))), F32(c.m.r32((l_cur_4).wrapping_add(112)))); let a_ = (l_p_7).wrapping_add(176); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fsub(F32(c.m.r32((l_p_7).wrapping_add(172))), F32(c.m.r32((l_cur_4).wrapping_add(272)))); let a_ = (l_p_7).wrapping_add(180); c.m.w32(a_, (v_).0); };
            if (((rt::flt(F32(c.m.r32((l_p_7).wrapping_add(180))), F32(c.m.r32((l_p_7).wrapping_add(168)))) as i32)) != 0) {
                { let v_ = F32(c.m.r32((l_p_7).wrapping_add(168))); let a_ = (l_p_7).wrapping_add(180); c.m.w32(a_, (v_).0); };
            }
            if (((((c.m.r32(l_def_5) as i32)) & (64i32))) != 0) {
                if ((((!((((F::Port_Rand(c)) & (1i32))) != 0)) as i32)) != 0) {
                    { let r_ = 128i32; let a_ = (l_p_7).wrapping_add(160); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
            { let v_ = rt::fadd(F32(c.m.r32((l_cur_4).wrapping_add(276))), rt::fmul(F32(c.m.r32((l_cur_4).wrapping_add(280))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = (l_p_7).wrapping_add(248); c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = F32(c.m.r32((l_p_7).wrapping_add(248))); F::EftMath_WrapAngle(c, a0_) }; let a_ = (l_p_7).wrapping_add(248); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fadd(F32(c.m.r32((l_cur_4).wrapping_add(284))), rt::fmul(F32(c.m.r32((l_cur_4).wrapping_add(288))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = (l_p_7).wrapping_add(252); c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = F32(c.m.r32((l_p_7).wrapping_add(252))); F::EftMath_WrapAngle(c, a0_) }; let a_ = (l_p_7).wrapping_add(252); c.m.w32(a_, (v_).0); };
            if (((((c.m.r32((l_p_7).wrapping_add(160)) as i32)) & (128i32))) != 0) {
                { let v_ = rt::fneg(F32(c.m.r32((l_p_7).wrapping_add(252)))); let a_ = (l_p_7).wrapping_add(252); c.m.w32(a_, (v_).0); };
            }
            if (((((c.m.r32(l_def_5) as i32)) & (8i32))) != 0) {
                { let v_ = F32(0x00000000); let a_ = (l_p_7).wrapping_add(256); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32((l_cur_4).wrapping_add(196))); let a_ = (l_p_7).wrapping_add(264); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32((l_cur_4).wrapping_add(192))); let a_ = (l_p_7).wrapping_add(260); c.m.w32(a_, (v_).0); };
                if (((((c.m.r32(l_def_5) as i32)) & (16i32))) != 0) {
                    { let v_ = rt::fmul(F32(c.m.r32((l_cur_4).wrapping_add(196))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff))); let a_ = (l_p_7).wrapping_add(256); c.m.w32(a_, (v_).0); };
                }
                { let v_ = F32(0x00000000); let a_ = (l_p_7).wrapping_add(268); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32((l_cur_4).wrapping_add(208))); let a_ = (l_p_7).wrapping_add(276); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32((l_cur_4).wrapping_add(204))); let a_ = (l_p_7).wrapping_add(272); c.m.w32(a_, (v_).0); };
                if (((((c.m.r32(l_def_5) as i32)) & (16i32))) != 0) {
                    { let v_ = rt::fmul(F32(c.m.r32((l_cur_4).wrapping_add(208))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff))); let a_ = (l_p_7).wrapping_add(268); c.m.w32(a_, (v_).0); };
                }
            }
            {
                l_i_16 = 0i32;
                'l2: loop {
                    if !(((((l_i_16) < (3i32)) as i32)) != 0) { break 'l2; }
                    'c3: {
                        { let v_ = rt::fadd(F32(c.m.r32(((l_cur_4).wrapping_add(116)).wrapping_add((((l_i_16) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_cur_4).wrapping_add(128)).wrapping_add((((l_i_16) as u32)).wrapping_mul(4)))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = ((l_p_7).wrapping_add(204)).wrapping_add((((l_i_16) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(F32(c.m.r32(((l_cur_4).wrapping_add(140)).wrapping_add((((l_i_16) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_cur_4).wrapping_add(152)).wrapping_add((((l_i_16) as u32)).wrapping_mul(4)))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = ((l_p_7).wrapping_add(220)).wrapping_add((((l_i_16) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(F32(c.m.r32(((l_cur_4).wrapping_add(164)).wrapping_add((((l_i_16) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_cur_4).wrapping_add(176)).wrapping_add((((l_i_16) as u32)).wrapping_mul(4)))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = ((l_p_7).wrapping_add(236)).wrapping_add((((l_i_16) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    }
                    { let o_ = l_i_16; let n_ = (o_).wrapping_add(1); l_i_16 = n_; };
                }
            }
            { let v_ = F32(c.m.r32(((l_p_7).wrapping_add(204)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (l_p_7).wrapping_add(200); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32(((l_p_7).wrapping_add(220)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (l_p_7).wrapping_add(216); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32(((l_p_7).wrapping_add(236)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (l_p_7).wrapping_add(232); c.m.w32(a_, (v_).0); };
            if (((((c.m.r32(l_def_5) as i32)) & (256i32))) != 0) {
                { let a2_ = F32(c.m.r32((l_cur_4).wrapping_add(216))); F::Vec4_Set(c, (l_p_7).wrapping_add(64), F32(0x00000000), a2_, F32(0x00000000), F32(0x3f800000)) };
                F::Vec3_Normalize(c, (l_p_7).wrapping_add(64), (l_p_7).wrapping_add(64));
            } else {
                l_a_8 = rt::fmul(rt::fsub(F32(c.m.r32((l_cur_4).wrapping_add(240))), F32(c.m.r32((l_cur_4).wrapping_add(244)))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)));
                l_a_8 = { let a0_ = rt::fadd(l_a_8, F32(c.m.r32((l_cur_4).wrapping_add(244)))); F::EftMath_WrapAngle(c, a0_) };
                F::Vec4_Set(c, (l_p_7).wrapping_add(64), F32(0x00000000), F32(0x00000000), F32(0x3f800000), F32(0x3f800000));
                { let a1_ = c.m.r32(sym::gEftPtcl); F::Mtx_RotateX(c, fp, a1_, l_a_8) };
                { let a2_ = F32(c.m.r32((p_w_1).wrapping_add(548))); F::Mtx_RotateZ(c, fp, fp, a2_) };
                F::Mtx_MulVec4(c, (l_p_7).wrapping_add(64), fp, (l_p_7).wrapping_add(64));
                { let r_ = rt::fadd(F32(c.m.r32((l_cur_4).wrapping_add(264))), F32(0x3f800000)); let a_ = (l_p_7).wrapping_add(64); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                F::Vec3_Normalize(c, (l_p_7).wrapping_add(64), (l_p_7).wrapping_add(64));
                F::Mtx_MulVec4(c, (l_p_7).wrapping_add(64), (p_w_1).wrapping_add(368), (l_p_7).wrapping_add(64));
            }
            l_a_8 = rt::fadd(F32(c.m.r32((l_cur_4).wrapping_add(256))), rt::fmul(F32(c.m.r32((l_cur_4).wrapping_add(260))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff))));
            { let a2_ = rt::fmul(l_a_8, F32(c.m.r32((p_w2_2).wrapping_add(44)))); F::Vec3_Scale(c, l_p_7, (l_p_7).wrapping_add(64), a2_) };
            { let v_ = F32(0x3f800000); let a_ = (l_p_7).wrapping_add(12); c.m.w32(a_, (v_).0); };
            F::Vec4_Set(c, (l_p_7).wrapping_add(16), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x3f800000));
            l_a_8 = rt::fmul(rt::fadd(F32(c.m.r32((l_cur_4).wrapping_add(220))), rt::fmul(F32(c.m.r32((l_cur_4).wrapping_add(224))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)));
            { let r_ = rt::fadd(F32(c.m.r32((l_cur_4).wrapping_add(228))), rt::fmul(F32(c.m.r32((l_cur_4).wrapping_add(232))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let n_ = rt::fadd(l_a_8, r_); l_a_8 = n_; };
            { let r_ = F32(c.m.r32((p_w2_2).wrapping_add(44))); let n_ = rt::fmul(l_a_8, r_); l_a_8 = n_; };
            's4: {
                let sv17: i32 = ((c.m.r8((l_def_5).wrapping_add(8))) as i32);
                match sv17 {
                    0i32 => {
                        F::Vec4_Set(c, (l_p_7).wrapping_add(16), F32(0x00000000), F32(0x00000000), l_a_8, F32(0x3f800000));
                        { let a1_ = c.m.r32(sym::gEftPtcl); let a2_ = F32(c.m.r32((p_w_1).wrapping_add(432))); F::Mtx_RotateX(c, fp, a1_, a2_) };
                        { let a2_ = F32(c.m.r32(((p_w_1).wrapping_add(432)).wrapping_add(4))); F::Mtx_RotateY(c, fp, fp, a2_) };
                        F::Mtx_MulVec4(c, (l_p_7).wrapping_add(16), fp, (l_p_7).wrapping_add(16));
                        break 's4;
                    }
                    1i32 => {
                        F::Vec4_Set(c, (l_p_7).wrapping_add(16), F32(0x00000000), F32(0x00000000), l_a_8, F32(0x3f800000));
                        { let a1_ = c.m.r32(sym::gEftPtcl); let a2_ = F32(c.m.r32(((p_w_1).wrapping_add(432)).wrapping_add(4))); F::Mtx_RotateY(c, fp, a1_, a2_) };
                        F::Mtx_MulVec4(c, (l_p_7).wrapping_add(16), fp, (l_p_7).wrapping_add(16));
                        break 's4;
                    }
                    2i32 => {
                        F::Vec4_Set(c, (l_p_7).wrapping_add(16), F32(0x00000000), l_a_8, F32(0x00000000), F32(0x3f800000));
                        { let a1_ = c.m.r32(sym::gEftPtcl); let a2_ = F32(c.m.r32(((p_w_1).wrapping_add(432)).wrapping_add(4))); F::Mtx_RotateY(c, fp, a1_, a2_) };
                        { let a2_ = F32(c.m.r32(((p_w_1).wrapping_add(432)).wrapping_add(8))); F::Mtx_RotateZ(c, fp, fp, a2_) };
                        F::Mtx_MulVec4(c, (l_p_7).wrapping_add(16), fp, (l_p_7).wrapping_add(16));
                        F::Mtx_MulVec4(c, (l_p_7).wrapping_add(16), (p_w_1).wrapping_add(368), (l_p_7).wrapping_add(16));
                        { let a3_ = rt::fmul(F32(c.m.r32((l_cur_4).wrapping_add(236))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff))); F::Vec3_ScaleAdd(c, (l_p_7).wrapping_add(16), (p_w2_2).wrapping_add(16), (l_p_7).wrapping_add(16), a3_) };
                        break 's4;
                    }
                    _ => {
                    }
                }
            }
            's5: {
                let sv18: i32 = ((c.m.r8((l_def_5).wrapping_add(9))) as i32);
                match sv18 {
                    0i32 => {
                        F::Vec3_Sub(c, (l_p_7).wrapping_add(80), (p_w2_2).wrapping_add(16), (l_p_7).wrapping_add(64));
                        { let v_ = F32(0x3f800000); let a_ = ((l_p_7).wrapping_add(80)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        F::Vec3_Normalize(c, (l_p_7).wrapping_add(80), (l_p_7).wrapping_add(80));
                        { let a2_ = F32(c.m.r32((l_cur_4).wrapping_add(212))); F::Vec3_Scale(c, (l_p_7).wrapping_add(80), (l_p_7).wrapping_add(80), a2_) };
                        break 's5;
                    }
                    1i32 => {
                        F::Vec3_Normalize(c, fp.wrapping_add(64), (l_p_7).wrapping_add(16));
                        F::Vec3_Sub(c, (l_p_7).wrapping_add(80), (p_w2_2).wrapping_add(16), fp.wrapping_add(64));
                        { let v_ = F32(0x3f800000); let a_ = ((l_p_7).wrapping_add(80)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        F::Vec3_Normalize(c, (l_p_7).wrapping_add(80), (l_p_7).wrapping_add(80));
                        { let a2_ = F32(c.m.r32((l_cur_4).wrapping_add(212))); F::Vec3_Scale(c, (l_p_7).wrapping_add(80), (l_p_7).wrapping_add(80), a2_) };
                        break 's5;
                    }
                    2i32 => {
                        F::Vec3_Normalize(c, fp.wrapping_add(64), (l_p_7).wrapping_add(16));
                        F::Vec3_Add(c, fp.wrapping_add(64), fp.wrapping_add(64), (l_p_7).wrapping_add(64));
                        F::Vec3_Normalize(c, fp.wrapping_add(64), fp.wrapping_add(64));
                        F::Vec3_Sub(c, (l_p_7).wrapping_add(80), (p_w2_2).wrapping_add(16), fp.wrapping_add(64));
                        { let v_ = F32(0x3f800000); let a_ = ((l_p_7).wrapping_add(80)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        F::Vec3_Normalize(c, (l_p_7).wrapping_add(80), (l_p_7).wrapping_add(80));
                        { let a2_ = F32(c.m.r32((l_cur_4).wrapping_add(212))); F::Vec3_Scale(c, (l_p_7).wrapping_add(80), (l_p_7).wrapping_add(80), a2_) };
                        break 's5;
                    }
                    _ => {
                    }
                }
            }
            { let r_ = F32(c.m.r32((l_cur_4).wrapping_add(216))); let a_ = ((l_p_7).wrapping_add(80)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            F::Vec4_Set(c, (l_p_7).wrapping_add(96), F32(0x00000000), F32(0x3f800000), F32(0x00000000), F32(0x3f800000));
            { let v_ = rt::fadd(F32(c.m.r32(l_cur_4)), rt::fmul(F32(c.m.r32((l_cur_4).wrapping_add(16))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = (l_p_7).wrapping_add(128); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fadd(F32(c.m.r32((l_cur_4).wrapping_add(4))), rt::fmul(F32(c.m.r32(((l_cur_4).wrapping_add(16)).wrapping_add(4))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = ((l_p_7).wrapping_add(128)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fadd(F32(c.m.r32((l_cur_4).wrapping_add(8))), rt::fmul(F32(c.m.r32(((l_cur_4).wrapping_add(16)).wrapping_add(8))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = ((l_p_7).wrapping_add(128)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fadd(F32(c.m.r32((l_cur_4).wrapping_add(12))), rt::fmul(F32(c.m.r32(((l_cur_4).wrapping_add(16)).wrapping_add(12))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = ((l_p_7).wrapping_add(128)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fadd(F32(c.m.r32((l_cur_4).wrapping_add(32))), rt::fmul(F32(c.m.r32((l_cur_4).wrapping_add(48))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = (l_p_7).wrapping_add(144); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fadd(F32(c.m.r32(((l_cur_4).wrapping_add(32)).wrapping_add(4))), rt::fmul(F32(c.m.r32(((l_cur_4).wrapping_add(48)).wrapping_add(4))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = ((l_p_7).wrapping_add(144)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fadd(F32(c.m.r32(((l_cur_4).wrapping_add(32)).wrapping_add(8))), rt::fmul(F32(c.m.r32(((l_cur_4).wrapping_add(48)).wrapping_add(8))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = ((l_p_7).wrapping_add(144)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fadd(F32(c.m.r32(((l_cur_4).wrapping_add(32)).wrapping_add(12))), rt::fmul(F32(c.m.r32(((l_cur_4).wrapping_add(48)).wrapping_add(12))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = ((l_p_7).wrapping_add(144)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            F::Vec4_Sub(c, (l_p_7).wrapping_add(144), (l_p_7).wrapping_add(144), (l_p_7).wrapping_add(128));
            F::Vec4_Copy(c, (l_p_7).wrapping_add(112), (l_p_7).wrapping_add(128));
            { let v_ = F32(0x00000000); let a_ = (l_p_7).wrapping_add(280); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32((l_cur_4).wrapping_add(268))); let a_ = (l_p_7).wrapping_add(192); c.m.w32(a_, (v_).0); };
            if (((rt::fle(F32(c.m.r32((l_p_7).wrapping_add(192))), F32(0x00000000)) as i32)) != 0) {
                { let r_ = 2i32; let a_ = (l_p_7).wrapping_add(160); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
            { let v_ = F32(c.m.r32((l_cur_4).wrapping_add(272))); let a_ = (l_p_7).wrapping_add(196); c.m.w32(a_, (v_).0); };
            if (((rt::flt(F32(c.m.r32((l_p_7).wrapping_add(172))), F32(c.m.r32((l_p_7).wrapping_add(196)))) as i32)) != 0) {
                { let v_ = F32(c.m.r32((l_cur_4).wrapping_add(272))); let a_ = (l_p_7).wrapping_add(196); c.m.w32(a_, (v_).0); };
            }
            if (((rt::fle(F32(c.m.r32((l_p_7).wrapping_add(196))), F32(0x00000000)) as i32)) != 0) {
                { let r_ = 4i32; let a_ = (l_p_7).wrapping_add(160); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            } else {
                { let v_ = F32(c.m.r32((l_p_7).wrapping_add(196))); let a_ = (l_p_7).wrapping_add(188); c.m.w32(a_, (v_).0); };
            }
            if (((((((c.m.r8((l_def_5).wrapping_add(4))) as i32)) == (1i32)) as i32)) != 0) {
                l_spd_11 = F32(0x00000000);
                l_d_10 = rt::fsub(F32(c.m.r32((l_p_7).wrapping_add(172))), F32(c.m.r32((l_p_7).wrapping_add(176))));
                {
                    l_t_9 = F32(0x00000000);
                    'l6: loop {
                        if !(((rt::flt(l_t_9, F32(c.m.r32((l_p_7).wrapping_add(172)))) as i32)) != 0) { break 'l6; }
                        'c7: {
                            F::Vec3_Scale(c, fp.wrapping_add(80), (l_p_7).wrapping_add(64), l_spd_11);
                            F::Vec3_Add(c, l_p_7, l_p_7, fp.wrapping_add(80));
                            if (((rt::fge(l_t_9, F32(c.m.r32((l_p_7).wrapping_add(176)))) as i32)) != 0) {
                                l_k_12 = rt::fdiv(rt::fsub(l_t_9, F32(c.m.r32((l_p_7).wrapping_add(176)))), l_d_10);
                                l_spd_11 = rt::fadd(F32(c.m.r32(((l_p_7).wrapping_add(204)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(rt::fsub(F32(c.m.r32(((l_p_7).wrapping_add(204)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_7).wrapping_add(204)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), l_k_12));
                            } else {
                                l_k_12 = rt::fdiv(l_t_9, F32(c.m.r32((l_p_7).wrapping_add(176))));
                                l_spd_11 = rt::fadd(F32(c.m.r32(((l_p_7).wrapping_add(204)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(rt::fsub(F32(c.m.r32(((l_p_7).wrapping_add(204)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_7).wrapping_add(204)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), l_k_12));
                            }
                            { let r_ = F32(c.m.r32((p_w2_2).wrapping_add(44))); let n_ = rt::fmul(l_spd_11, r_); l_spd_11 = n_; };
                        }
                        { let r_ = F32(0x3f800000); let n_ = rt::fadd(l_t_9, r_); l_t_9 = n_; };
                    }
                }
                F::Vec3_Scale(c, (l_p_7).wrapping_add(64), (l_p_7).wrapping_add(64), F32(0xbf800000));
            }
            { let r_ = rt::fadd(F32(c.m.r32((l_cur_4).wrapping_add(248))), rt::fmul(F32(c.m.r32((l_cur_4).wrapping_add(252))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = (p_w_1).wrapping_add(548); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let o_ = l_n_6; let n_ = (o_).wrapping_sub(1); l_n_6 = n_; };
            { let v_ = { let a0_ = F32(c.m.r32((p_w_1).wrapping_add(548))); F::EftMath_WrapAngle(c, a0_) }; let a_ = (p_w_1).wrapping_add(548); c.m.w32(a_, (v_).0); };
            F::Vec3_Add(c, (p_w_1).wrapping_add(432), (p_w_1).wrapping_add(432), (p_w_1).wrapping_add(448));
            { let v_ = { let a0_ = F32(c.m.r32((p_w_1).wrapping_add(432))); F::EftMath_WrapAngle(c, a0_) }; let a_ = (p_w_1).wrapping_add(432); c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = F32(c.m.r32(((p_w_1).wrapping_add(432)).wrapping_add(4))); F::EftMath_WrapAngle(c, a0_) }; let a_ = ((p_w_1).wrapping_add(432)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = F32(c.m.r32(((p_w_1).wrapping_add(432)).wrapping_add(8))); F::EftMath_WrapAngle(c, a0_) }; let a_ = ((p_w_1).wrapping_add(432)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let a_ = (p_w_1).wrapping_add(476); let o_ = (c.m.r16(a_) as i16); let n_ = (o_).wrapping_add(1); c.m.w16(a_, (n_) as u16); };
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_Spawn", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtcl_StepPtcls(c: &mut Ctx, mut p_w_1: u32, mut p_w2_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_StepPtcls", &PROBE_N, &[(p_w_1) as u32, (p_w2_2) as u32], false); }
    let mut l_cur_3: u32 = 0;
    let mut l_def_4: u32 = 0;
    let mut l_p_5: u32 = 0;
    let mut l_link_6: u32 = 0;
    let mut l_next_7: u32 = 0;
    let mut l_t_8: F32 = F32(0);
    let mut l_k_9: F32 = F32(0);
    let mut l_s_10: F32 = F32(0);
    let mut l_frame_11: i32 = 0;
    let r_ = 'f: {
    l_cur_3 = (p_w_1).wrapping_add(64);
    l_def_4 = c.m.r32((p_w2_2).wrapping_add(52));
    l_p_5 = c.m.r32((p_w_1).wrapping_add(464));
    if (((((l_p_5) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    'l1: loop {
        'c2: {
            if (((rt::flt(F32(c.m.r32((l_p_5).wrapping_add(168))), F32(c.m.r32((l_p_5).wrapping_add(172)))) as i32)) != 0) {
                l_t_8 = rt::fdiv(F32(c.m.r32((l_p_5).wrapping_add(168))), F32(c.m.r32((l_p_5).wrapping_add(172))));
                if (((rt::fgt(l_t_8, F32(0x3f800000)) as i32)) != 0) {
                    l_t_8 = F32(0x3f800000);
                }
                if ((((((((!(((((c.m.r32((l_p_5).wrapping_add(160)) as i32)) & (8i32))) != 0)) as i32)) != 0) && (((rt::fge(F32(c.m.r32((l_p_5).wrapping_add(168))), F32(c.m.r32((l_p_5).wrapping_add(176)))) as i32)) != 0)) as i32)) != 0) {
                    { let r_ = 8i32; let a_ = (l_p_5).wrapping_add(160); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = rt::fadd(F32(c.m.r32((l_cur_3).wrapping_add(292))), rt::fmul(F32(c.m.r32((l_cur_3).wrapping_add(296))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = (l_p_5).wrapping_add(252); c.m.w32(a_, (v_).0); };
                    { let v_ = { let a0_ = F32(c.m.r32((l_p_5).wrapping_add(252))); F::EftMath_WrapAngle(c, a0_) }; let a_ = (l_p_5).wrapping_add(252); c.m.w32(a_, (v_).0); };
                    if (((((c.m.r32((l_p_5).wrapping_add(160)) as i32)) & (128i32))) != 0) {
                        { let v_ = rt::fneg(F32(c.m.r32((l_p_5).wrapping_add(252)))); let a_ = (l_p_5).wrapping_add(252); c.m.w32(a_, (v_).0); };
                    }
                }
                { let r_ = F32(c.m.r32((l_p_5).wrapping_add(252))); let a_ = (l_p_5).wrapping_add(248); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let v_ = { let a0_ = F32(c.m.r32((l_p_5).wrapping_add(248))); F::EftMath_WrapAngle(c, a0_) }; let a_ = (l_p_5).wrapping_add(248); c.m.w32(a_, (v_).0); };
                if ((((!(((((c.m.r32((l_p_5).wrapping_add(160)) as i32)) & (8i32))) != 0)) as i32)) != 0) {
                    l_k_9 = rt::fdiv(F32(c.m.r32((l_p_5).wrapping_add(168))), F32(c.m.r32((l_p_5).wrapping_add(176))));
                    { let v_ = rt::fadd(F32(c.m.r32(((l_p_5).wrapping_add(204)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(rt::fsub(F32(c.m.r32(((l_p_5).wrapping_add(204)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_5).wrapping_add(204)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), l_k_9)); let a_ = (l_p_5).wrapping_add(200); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32(((l_p_5).wrapping_add(220)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(rt::fsub(F32(c.m.r32(((l_p_5).wrapping_add(220)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_5).wrapping_add(220)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), l_k_9)); let a_ = (l_p_5).wrapping_add(216); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32(((l_p_5).wrapping_add(236)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(rt::fsub(F32(c.m.r32(((l_p_5).wrapping_add(236)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_5).wrapping_add(236)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), l_k_9)); let a_ = (l_p_5).wrapping_add(232); c.m.w32(a_, (v_).0); };
                } else {
                    l_k_9 = rt::fdiv(rt::fsub(F32(c.m.r32((l_p_5).wrapping_add(168))), F32(c.m.r32((l_p_5).wrapping_add(176)))), rt::fsub(F32(c.m.r32((l_p_5).wrapping_add(172))), F32(c.m.r32((l_p_5).wrapping_add(176)))));
                    { let v_ = rt::fadd(F32(c.m.r32(((l_p_5).wrapping_add(204)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(rt::fsub(F32(c.m.r32(((l_p_5).wrapping_add(204)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_5).wrapping_add(204)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), l_k_9)); let a_ = (l_p_5).wrapping_add(200); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32(((l_p_5).wrapping_add(220)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(rt::fsub(F32(c.m.r32(((l_p_5).wrapping_add(220)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_5).wrapping_add(220)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), l_k_9)); let a_ = (l_p_5).wrapping_add(216); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32(((l_p_5).wrapping_add(236)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(rt::fsub(F32(c.m.r32(((l_p_5).wrapping_add(236)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_p_5).wrapping_add(236)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), l_k_9)); let a_ = (l_p_5).wrapping_add(232); c.m.w32(a_, (v_).0); };
                }
                { let r_ = F32(c.m.r32((p_w2_2).wrapping_add(44))); let a_ = (l_p_5).wrapping_add(200); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = F32(c.m.r32((p_w2_2).wrapping_add(44))); let a_ = (l_p_5).wrapping_add(216); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = F32(c.m.r32((p_w2_2).wrapping_add(44))); let a_ = (l_p_5).wrapping_add(232); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((((c.m.r32(l_def_4) as i32)) & (8i32))) != 0) {
                    if (((rt::fgt(F32(c.m.r32((l_cur_3).wrapping_add(196))), F32(0x00000000)) as i32)) != 0) {
                        l_k_9 = rt::fdiv(F32(c.m.r32((l_p_5).wrapping_add(256))), F32(c.m.r32((l_cur_3).wrapping_add(196))));
                        l_s_10 = rt::fadd(F32(c.m.r32((l_cur_3).wrapping_add(188))), rt::fmul(F32(c.m.r32((l_cur_3).wrapping_add(192))), l_k_9));
                        { let r_ = l_s_10; let a_ = (l_p_5).wrapping_add(216); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        if ((((!(((((c.m.r32((l_p_5).wrapping_add(160)) as i32)) & (16i32))) != 0)) as i32)) != 0) {
                            { let r_ = F32(0x3f800000); let a_ = (l_p_5).wrapping_add(256); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            if (((rt::fge(F32(c.m.r32((l_p_5).wrapping_add(256))), F32(c.m.r32((l_cur_3).wrapping_add(196)))) as i32)) != 0) {
                                { let v_ = F32(c.m.r32((l_cur_3).wrapping_add(196))); let a_ = (l_p_5).wrapping_add(256); c.m.w32(a_, (v_).0); };
                                { let r_ = 16i32; let a_ = (l_p_5).wrapping_add(160); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            }
                        } else {
                            { let r_ = F32(0x3f800000); let a_ = (l_p_5).wrapping_add(256); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            if (((rt::fle(F32(c.m.r32((l_p_5).wrapping_add(256))), F32(0x00000000)) as i32)) != 0) {
                                { let v_ = F32(0x00000000); let a_ = (l_p_5).wrapping_add(256); c.m.w32(a_, (v_).0); };
                                { let r_ = (!(16i32)); let a_ = (l_p_5).wrapping_add(160); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                            }
                        }
                    }
                    if (((rt::fgt(F32(c.m.r32((l_cur_3).wrapping_add(208))), F32(0x00000000)) as i32)) != 0) {
                        l_k_9 = rt::fdiv(F32(c.m.r32((l_p_5).wrapping_add(268))), F32(c.m.r32((l_cur_3).wrapping_add(208))));
                        l_s_10 = rt::fadd(F32(c.m.r32((l_cur_3).wrapping_add(200))), rt::fmul(F32(c.m.r32((l_cur_3).wrapping_add(204))), l_k_9));
                        { let r_ = l_s_10; let a_ = (l_p_5).wrapping_add(232); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        if ((((!(((((c.m.r32((l_p_5).wrapping_add(160)) as i32)) & (32i32))) != 0)) as i32)) != 0) {
                            { let r_ = F32(0x3f800000); let a_ = (l_p_5).wrapping_add(268); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            if (((rt::fge(F32(c.m.r32((l_p_5).wrapping_add(268))), F32(c.m.r32((l_cur_3).wrapping_add(208)))) as i32)) != 0) {
                                { let v_ = F32(c.m.r32((l_cur_3).wrapping_add(208))); let a_ = (l_p_5).wrapping_add(268); c.m.w32(a_, (v_).0); };
                                { let r_ = 32i32; let a_ = (l_p_5).wrapping_add(160); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            }
                        } else {
                            { let r_ = F32(0x3f800000); let a_ = (l_p_5).wrapping_add(268); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            if (((rt::fle(F32(c.m.r32((l_p_5).wrapping_add(268))), F32(0x00000000)) as i32)) != 0) {
                                { let v_ = F32(0x00000000); let a_ = (l_p_5).wrapping_add(268); c.m.w32(a_, (v_).0); };
                                { let r_ = (!(32i32)); let a_ = (l_p_5).wrapping_add(160); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                            }
                        }
                    }
                }
                { let a3_ = F32(c.m.r32((l_p_5).wrapping_add(200))); F::Vec3_ScaleAdd(c, l_p_5, (l_p_5).wrapping_add(64), l_p_5, a3_) };
                if ((((!(((((c.m.r32((l_p_5).wrapping_add(160)) as i32)) & (256i32))) != 0)) as i32)) != 0) {
                    F::Vec3_Add(c, (l_p_5).wrapping_add(32), l_p_5, (l_p_5).wrapping_add(16));
                    F::Vec3_Add(c, (l_p_5).wrapping_add(48), (l_p_5).wrapping_add(32), p_w2_2);
                } else {
                    F::Vec3_Add(c, (l_p_5).wrapping_add(48), l_p_5, (l_p_5).wrapping_add(16));
                    F::Vec3_Add(c, (l_p_5).wrapping_add(48), (l_p_5).wrapping_add(48), (l_p_5).wrapping_add(32));
                }
                F::Vec3_Add(c, (l_p_5).wrapping_add(64), (l_p_5).wrapping_add(64), (l_p_5).wrapping_add(80));
                F::Vec3_Normalize(c, (l_p_5).wrapping_add(64), (l_p_5).wrapping_add(64));
                if (((((((((c.m.r32((p_w_1).wrapping_add(472)) as i32)) & (64i32))) != 0) && ((((!(((((c.m.r32((l_p_5).wrapping_add(160)) as i32)) & (256i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
                    F::Vec4_Copy(c, (l_p_5).wrapping_add(32), p_w2_2);
                    { let r_ = 256i32; let a_ = (l_p_5).wrapping_add(160); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
                if (((((c.m.r32((p_w_1).wrapping_add(472)) as i32)) & (32i32))) != 0) {
                    l_k_9 = rt::fdiv(F32(c.m.r32((l_p_5).wrapping_add(164))), F32(c.m.r32((p_w_1).wrapping_add(496))));
                    l_s_10 = rt::fadd(F32(c.m.r32((l_p_5).wrapping_add(164))), F32(0x3f800000));
                    l_frame_11 = rt::f2i(rt::fmul(rt::i2f((c.m.r32((p_w_1).wrapping_add(480)) as i32)), l_k_9));
                    { let v_ = rt::fmul(rt::i2f(rt::rem_i32(l_frame_11, ((c.m.r8((l_def_4).wrapping_add(14))) as i32))), F32(c.m.r32((p_w_1).wrapping_add(484)))); let a_ = (l_p_5).wrapping_add(96); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32((l_p_5).wrapping_add(96))), F32(c.m.r32((p_w_1).wrapping_add(484)))); let a_ = (l_p_5).wrapping_add(100); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fmul(rt::i2f(rt::div_i32(l_frame_11, ((c.m.r8((l_def_4).wrapping_add(14))) as i32))), F32(c.m.r32((p_w_1).wrapping_add(488)))); let a_ = (l_p_5).wrapping_add(104); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32((l_p_5).wrapping_add(104))), F32(c.m.r32((p_w_1).wrapping_add(488)))); let a_ = (l_p_5).wrapping_add(108); c.m.w32(a_, (v_).0); };
                    { let v_ = l_s_10; let a_ = (l_p_5).wrapping_add(164); c.m.w32(a_, (v_).0); };
                    if (((rt::fge(l_s_10, F32(c.m.r32((p_w_1).wrapping_add(496)))) as i32)) != 0) {
                        { let v_ = rt::fsub(l_s_10, F32(c.m.r32((p_w_1).wrapping_add(496)))); let a_ = (l_p_5).wrapping_add(164); c.m.w32(a_, (v_).0); };
                    }
                }
                if (((((c.m.r32((p_w_1).wrapping_add(472)) as i32)) & (512i32))) != 0) {
                    F::Vec4_Scale(c, (l_p_5).wrapping_add(112), (l_p_5).wrapping_add(144), l_t_8);
                    F::Vec4_Add(c, (l_p_5).wrapping_add(112), (l_p_5).wrapping_add(112), (l_p_5).wrapping_add(128));
                } else {
                    F::Vec4_Copy(c, (l_p_5).wrapping_add(112), (l_p_5).wrapping_add(128));
                }
                F::Vec4_Clamp(c, (l_p_5).wrapping_add(112), (l_p_5).wrapping_add(112), F32(0x00000000), F32(0x437f0000));
                if ((((!(((((c.m.r32((l_p_5).wrapping_add(160)) as i32)) & (2i32))) != 0)) as i32)) != 0) {
                    l_k_9 = rt::fdiv(F32(c.m.r32((l_p_5).wrapping_add(184))), F32(c.m.r32((l_p_5).wrapping_add(192))));
                    { let r_ = F32(0x3f800000); let a_ = (l_p_5).wrapping_add(184); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if (((rt::fge(F32(c.m.r32((l_p_5).wrapping_add(184))), F32(c.m.r32((l_p_5).wrapping_add(192)))) as i32)) != 0) {
                        { let r_ = 2i32; let a_ = (l_p_5).wrapping_add(160); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    } else {
                        { let r_ = l_k_9; let a_ = ((l_p_5).wrapping_add(112)).wrapping_add(12); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    }
                }
                if (((rt::fge(F32(c.m.r32((l_p_5).wrapping_add(168))), F32(c.m.r32((l_p_5).wrapping_add(180)))) as i32)) != 0) {
                    if ((((!(((((c.m.r32((l_p_5).wrapping_add(160)) as i32)) & (4i32))) != 0)) as i32)) != 0) {
                        l_k_9 = rt::fdiv(F32(c.m.r32((l_p_5).wrapping_add(188))), F32(c.m.r32((l_p_5).wrapping_add(196))));
                        { let r_ = F32(0x3f800000); let a_ = (l_p_5).wrapping_add(188); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        if (((rt::fle(F32(c.m.r32((l_p_5).wrapping_add(188))), F32(0x00000000)) as i32)) != 0) {
                            { let v_ = F32(0x00000000); let a_ = ((l_p_5).wrapping_add(112)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                            { let r_ = 4i32; let a_ = (l_p_5).wrapping_add(160); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        } else {
                            { let r_ = l_k_9; let a_ = ((l_p_5).wrapping_add(112)).wrapping_add(12); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        }
                    } else {
                        { let v_ = F32(0x00000000); let a_ = ((l_p_5).wrapping_add(112)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    }
                }
                if (((((c.m.r32(l_def_4) as i32)) & (32i32))) != 0) {
                    if (((rt::fgt(F32(c.m.r32((l_cur_3).wrapping_add(88))), F32(0x00000000)) as i32)) != 0) {
                        l_k_9 = rt::fdiv(F32(c.m.r32((l_p_5).wrapping_add(280))), F32(c.m.r32((l_cur_3).wrapping_add(88))));
                        l_s_10 = rt::fadd(F32(c.m.r32(((l_cur_3).wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_cur_3).wrapping_add(76)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_k_9));
                        { let r_ = l_s_10; let a_ = (l_p_5).wrapping_add(112); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        l_s_10 = rt::fadd(F32(c.m.r32(((l_cur_3).wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_cur_3).wrapping_add(76)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_k_9));
                        { let r_ = l_s_10; let a_ = ((l_p_5).wrapping_add(112)).wrapping_add(4); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        l_s_10 = rt::fadd(F32(c.m.r32(((l_cur_3).wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_cur_3).wrapping_add(76)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_k_9));
                        { let r_ = l_s_10; let a_ = ((l_p_5).wrapping_add(112)).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        if ((((!(((((c.m.r32((l_p_5).wrapping_add(160)) as i32)) & (64i32))) != 0)) as i32)) != 0) {
                            { let r_ = F32(0x3f800000); let a_ = (l_p_5).wrapping_add(280); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            if (((rt::fge(F32(c.m.r32((l_p_5).wrapping_add(280))), F32(c.m.r32((l_cur_3).wrapping_add(88)))) as i32)) != 0) {
                                { let v_ = F32(c.m.r32((l_cur_3).wrapping_add(88))); let a_ = (l_p_5).wrapping_add(280); c.m.w32(a_, (v_).0); };
                                { let r_ = 64i32; let a_ = (l_p_5).wrapping_add(160); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            }
                        } else {
                            { let r_ = F32(0x3f800000); let a_ = (l_p_5).wrapping_add(280); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            if (((rt::fle(F32(c.m.r32((l_p_5).wrapping_add(280))), F32(0x00000000)) as i32)) != 0) {
                                { let v_ = F32(0x00000000); let a_ = (l_p_5).wrapping_add(280); c.m.w32(a_, (v_).0); };
                                { let r_ = (!(64i32)); let a_ = (l_p_5).wrapping_add(160); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                            }
                        }
                    }
                }
                if (((((((((((rt::fle(F32(c.m.r32(((l_p_5).wrapping_add(112)).wrapping_add(12))), F32(0x00000000)) as i32)) != 0) || (((rt::fle(F32(c.m.r32((l_p_5).wrapping_add(216))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) || (((rt::fle(F32(c.m.r32((l_p_5).wrapping_add(232))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                    { let r_ = (!(1024i32)); let a_ = (l_p_5).wrapping_add(160); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                } else {
                    { let r_ = 1024i32; let a_ = (l_p_5).wrapping_add(160); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    's3: {
                        let sv12: i32 = ((c.m.r8((l_def_4).wrapping_add(5))) as i32);
                        match sv12 {
                            0i32 => {
                                { let r_ = F32(0x3f000000); let a_ = (l_p_5).wrapping_add(216); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                if (((((((c.m.r8((l_def_4).wrapping_add(7))) as i32)) == (1i32)) as i32)) != 0) {
                                    { let r_ = F32(0x3f000000); let a_ = (l_p_5).wrapping_add(232); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                }
                                break 's3;
                            }
                            1i32 | 2i32 => {
                                { let r_ = F32(0x3f000000); let a_ = (l_p_5).wrapping_add(216); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                { let r_ = F32(0x3f000000); let a_ = (l_p_5).wrapping_add(232); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                if ((((!(((((c.m.r32(l_def_4) as i32)) & (2i32))) != 0)) as i32)) != 0) {
                                    { let r_ = F32(0x41800000); let a_ = (l_p_5).wrapping_add(216); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                    { let r_ = F32(0x41800000); let a_ = (l_p_5).wrapping_add(232); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                }
                                break 's3;
                            }
                            _ => {
                            }
                        }
                    }
                }
                { let r_ = F32(0x3f800000); let a_ = (l_p_5).wrapping_add(168); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                l_link_6 = (l_p_5).wrapping_add(284);
            } else {
                F::EftPtcl_UnlinkPtcl(c, p_w_1, l_p_5);
                { let v_ = 0i32; let a_ = (l_p_5).wrapping_add(160); c.m.w32(a_, (v_) as u32); };
                l_link_6 = (l_p_5).wrapping_add(284);
                { let r_ = 1i32; let a_ = (p_w_1).wrapping_add(476); let n_ = ((((((c.m.r16(a_) as i16)) as i32)).wrapping_sub(r_)) as i16); c.m.w16(a_, (n_) as u16); };
            }
            l_next_7 = c.m.r32(l_link_6);
            l_p_5 = l_next_7;
        }
        if !(((((l_next_7) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_StepPtcls", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtcl_Emit(c: &mut Ctx, mut p_w_1: u32, mut p_w2_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_Emit", &PROBE_N, &[(p_w_1) as u32, (p_w2_2) as u32], false); }
    let mut l_cur_3: u32 = 0;
    let mut l_n_4: i32 = 0;
    let mut l_free_5: i32 = 0;
    let mut l_any_6: i32 = 0;
    let r_ = 'f: {
    l_cur_3 = (p_w_1).wrapping_add(64);
    l_n_4 = 0i32;
    'l1: while (((rt::fle(F32(c.m.r32((p_w_1).wrapping_add(504))), F32(0x00000000)) as i32)) != 0) {
        l_free_5 = (500i32).wrapping_sub({ let a_ = (c.m.r32(sym::gEftPtcl)).wrapping_add(72); (c.m.r32(a_) as i32) });
        l_any_6 = 0i32;
        if (((((l_free_5) > (0i32)) as i32)) != 0) {
            l_n_4 = rt::f2i(F32(c.m.r32((l_cur_3).wrapping_add(92))));
            if (((rt::fgt(F32(c.m.r32((l_cur_3).wrapping_add(96))), F32(0x00000000)) as i32)) != 0) {
                l_n_4 = rt::f2i(rt::fadd(rt::i2f(l_n_4), rt::fmul(F32(c.m.r32((l_cur_3).wrapping_add(96))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))));
            }
            l_any_6 = 1i32;
            if (((((l_free_5) < (l_n_4)) as i32)) != 0) {
                l_n_4 = l_free_5;
            }
        }
        if ((l_any_6) != 0) {
            { let _ = D::call(c, fid::EftPtcl_Spawn, &[((p_w_1) as u64), ((p_w2_2) as u64), ((l_n_4) as i64 as u64)]); };
        }
        { let r_ = F32(c.m.r32((l_cur_3).wrapping_add(100))); let a_ = (p_w_1).wrapping_add(504); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if ((((!((l_any_6) != 0)) as i32)) != 0) {
            break 'l1;
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_Emit", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtcl_DrawSquares(c: &mut Ctx, mut p_w_1: u32, mut p_w2_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_DrawSquares", &PROBE_N, &[(p_w_1) as u32, (p_w2_2) as u32], false); }
    let mut l_def_3: u32 = 0;
    let mut l_list_4: u32 = 0;
    let mut l_p_5: u32 = 0;
    let r_ = 'f: {
    l_def_3 = c.m.r32((p_w2_2).wrapping_add(52));
    l_list_4 = (p_w_1).wrapping_add(464);
    if (((((c.m.r32(l_def_3) as i32)) & (2i32))) != 0) {
        {
            l_p_5 = c.m.r32(l_list_4);
            'l1: loop {
                if !(((((l_p_5) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((c.m.r32((l_p_5).wrapping_add(160)) as i32)) & (1024i32))) != 0) {
                        { let a2_ = ((c.m.r8((l_def_3).wrapping_add(10))) as i32); let a3_ = ((((c.m.r32((p_w_1).wrapping_add(472)) as i32)).wrapping_shr((4i32) as u32)) & (1i32)); let a4_ = c.m.r64((p_w_1).wrapping_add(584)); let a5_ = F32(c.m.r32((l_p_5).wrapping_add(232))); let a6_ = F32(c.m.r32((l_p_5).wrapping_add(232))); let a7_ = F32(c.m.r32((l_p_5).wrapping_add(96))); let a8_ = F32(c.m.r32((l_p_5).wrapping_add(104))); let a9_ = F32(c.m.r32((l_p_5).wrapping_add(100))); let a10_ = F32(c.m.r32((l_p_5).wrapping_add(108))); let a11_ = F32(c.m.r32((l_p_5).wrapping_add(248))); F::EftPrim_DrawQuadDepth(c, (l_p_5).wrapping_add(48), (l_p_5).wrapping_add(112), a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_, a10_, a11_) };
                    }
                }
                l_p_5 = c.m.r32((l_p_5).wrapping_add(284));
            }
        }
    } else {
        'l3: while (((((c.m.r32(l_list_4)) != (((0i32) as u32))) as i32)) != 0) {
            l_p_5 = c.m.r32(l_list_4);
            if (((((c.m.r32((l_p_5).wrapping_add(160)) as i32)) & (1024i32))) != 0) {
                { let a2_ = F32(c.m.r32((l_p_5).wrapping_add(232))); let a3_ = F32(c.m.r32((l_p_5).wrapping_add(232))); let a4_ = F32(c.m.r32((l_p_5).wrapping_add(96))); let a5_ = F32(c.m.r32((l_p_5).wrapping_add(104))); let a6_ = F32(c.m.r32((l_p_5).wrapping_add(100))); let a7_ = F32(c.m.r32((l_p_5).wrapping_add(108))); let a8_ = F32(c.m.r32((l_p_5).wrapping_add(248))); let a9_ = ((c.m.r8((l_def_3).wrapping_add(10))) as i32); let a10_ = ((((c.m.r32((p_w_1).wrapping_add(472)) as i32)).wrapping_shr((4i32) as u32)) & (1i32)); let a11_ = c.m.r64((p_w_1).wrapping_add(584)); F::EftGfx_DrawSprite(c, (l_p_5).wrapping_add(48), (l_p_5).wrapping_add(112), a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_, a10_, a11_) };
            }
            l_list_4 = (l_p_5).wrapping_add(284);
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_DrawSquares", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtcl_DrawSprites(c: &mut Ctx, mut p_w_1: u32, mut p_w2_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_DrawSprites", &PROBE_N, &[(p_w_1) as u32, (p_w2_2) as u32], false); }
    let mut l_def_3: u32 = 0;
    let mut l_list_4: u32 = 0;
    let mut l_p_5: u32 = 0;
    let r_ = 'f: {
    l_def_3 = c.m.r32((p_w2_2).wrapping_add(52));
    l_list_4 = (p_w_1).wrapping_add(464);
    if (((((c.m.r32(l_def_3) as i32)) & (2i32))) != 0) {
        {
            l_p_5 = c.m.r32(l_list_4);
            'l1: loop {
                if !(((((l_p_5) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((c.m.r32((l_p_5).wrapping_add(160)) as i32)) & (1024i32))) != 0) {
                        { let a2_ = ((c.m.r8((l_def_3).wrapping_add(10))) as i32); let a3_ = ((((c.m.r32((p_w_1).wrapping_add(472)) as i32)).wrapping_shr((4i32) as u32)) & (1i32)); let a4_ = c.m.r64((p_w_1).wrapping_add(584)); let a5_ = F32(c.m.r32((l_p_5).wrapping_add(216))); let a6_ = F32(c.m.r32((l_p_5).wrapping_add(232))); let a7_ = F32(c.m.r32((l_p_5).wrapping_add(96))); let a8_ = F32(c.m.r32((l_p_5).wrapping_add(104))); let a9_ = F32(c.m.r32((l_p_5).wrapping_add(100))); let a10_ = F32(c.m.r32((l_p_5).wrapping_add(108))); let a11_ = F32(c.m.r32((l_p_5).wrapping_add(248))); F::EftPrim_DrawQuadDepth(c, (l_p_5).wrapping_add(48), (l_p_5).wrapping_add(112), a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_, a10_, a11_) };
                    }
                }
                l_p_5 = c.m.r32((l_p_5).wrapping_add(284));
            }
        }
    } else {
        'l3: while (((((c.m.r32(l_list_4)) != (((0i32) as u32))) as i32)) != 0) {
            l_p_5 = c.m.r32(l_list_4);
            if (((((c.m.r32((l_p_5).wrapping_add(160)) as i32)) & (1024i32))) != 0) {
                { let a2_ = F32(c.m.r32((l_p_5).wrapping_add(216))); let a3_ = F32(c.m.r32((l_p_5).wrapping_add(232))); let a4_ = F32(c.m.r32((l_p_5).wrapping_add(96))); let a5_ = F32(c.m.r32((l_p_5).wrapping_add(104))); let a6_ = F32(c.m.r32((l_p_5).wrapping_add(100))); let a7_ = F32(c.m.r32((l_p_5).wrapping_add(108))); let a8_ = F32(c.m.r32((l_p_5).wrapping_add(248))); let a9_ = ((c.m.r8((l_def_3).wrapping_add(10))) as i32); let a10_ = ((((c.m.r32((p_w_1).wrapping_add(472)) as i32)).wrapping_shr((4i32) as u32)) & (1i32)); let a11_ = c.m.r64((p_w_1).wrapping_add(584)); F::EftGfx_DrawSprite(c, (l_p_5).wrapping_add(48), (l_p_5).wrapping_add(112), a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_, a10_, a11_) };
            }
            l_list_4 = (l_p_5).wrapping_add(284);
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_DrawSprites", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtcl_DrawAxisQuads(c: &mut Ctx, mut p_w_1: u32, mut p_w2_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_DrawAxisQuads", &PROBE_N, &[(p_w_1) as u32, (p_w2_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(336) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 336);
    let mut l_def_3: u32 = 0;
    let mut l_link_4: u32 = 0;
    let mut l_p_5: u32 = 0;
    let mut l_pkt_6: u32 = 0;
    let mut l_e_7: u32 = 0;
    let mut l_i_8: i32 = 0;
    let mut l_z_9: i32 = 0;
    let mut l_layer_10: i32 = 0;
    let mut l_o_11: i32 = 0;
    let mut l_o_12: i32 = 0;
    let mut l_q_13: F32 = F32(0);
    let mut l_o_14: i32 = 0;
    let r_ = 'f: {
        l_def_3 = c.m.r32((p_w2_2).wrapping_add(52));
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(544); { let _ = D::call(c, fid::Vec4_Copy, &[((fp.wrapping_add(64)) as u64), ((a1_) as u64)]); } };
        {
            l_i_8 = 0i32;
            'l1: loop {
                if !(((((l_i_8) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_o_11 = (((((l_i_8) as u32)).wrapping_mul(16u32)) as i32);
                    { let v_ = F32(0x3f800000); let a_ = (((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(((l_o_11) as u32)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x00000000); let a_ = (((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12)).wrapping_add(((l_o_11) as u32)); c.m.w32(a_, (v_).0); };
                    { let _ = D::call(c, fid::Vec4_Set, &[(((fp).wrapping_add((((l_i_8) as u32)).wrapping_mul(16))) as u64), ((F32(0x00000000)).0 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
                }
                { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
            }
        }
        l_link_4 = (p_w_1).wrapping_add(464);
        'l3: while (((((c.m.r32(l_link_4)) != (((0i32) as u32))) as i32)) != 0) {
            l_p_5 = c.m.r32(l_link_4);
            if (((((c.m.r32((l_p_5).wrapping_add(160)) as i32)) & (1024i32))) != 0) {
                { let _ = D::call(c, fid::Vec3_Sub, &[((fp.wrapping_add(80)) as u64), (((l_p_5).wrapping_add(48)) as u64), ((fp.wrapping_add(64)) as u64)]); };
                F::Vec3_Cross(c, fp.wrapping_add(80), (l_p_5).wrapping_add(64), fp.wrapping_add(80));
                { let _ = D::call(c, fid::Vec3_Normalize, &[((fp.wrapping_add(80)) as u64), ((fp.wrapping_add(80)) as u64)]); };
                { let a2_ = F32(c.m.r32((l_p_5).wrapping_add(216))); { let _ = D::call(c, fid::Vec3_Scale, &[((fp.wrapping_add(224)) as u64), ((fp.wrapping_add(80)) as u64), ((a2_).0 as u64)]); } };
                if (((((c.m.r32(l_def_3) as i32)) & (128i32))) != 0) {
                    { let _ = D::call(c, fid::Vec3_Add, &[(((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as u64), (((l_p_5).wrapping_add(48)) as u64), ((fp.wrapping_add(224)) as u64)]); };
                    { let _ = D::call(c, fid::Vec3_Sub, &[(((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as u64), (((l_p_5).wrapping_add(48)) as u64), ((fp.wrapping_add(224)) as u64)]); };
                    { let a2_ = F32(c.m.r32((l_p_5).wrapping_add(232))); { let _ = D::call(c, fid::Vec3_Scale, &[((fp.wrapping_add(240)) as u64), (((l_p_5).wrapping_add(64)) as u64), ((a2_).0 as u64)]); } };
                    if (((((((c.m.r8((l_def_3).wrapping_add(7))) as i32)) == (0i32)) as i32)) != 0) {
                        { let _ = D::call(c, fid::Vec3_Add, &[(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as u64), (((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as u64), ((fp.wrapping_add(240)) as u64)]); };
                        { let _ = D::call(c, fid::Vec3_Add, &[(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as u64), (((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as u64), ((fp.wrapping_add(240)) as u64)]); };
                    } else {
                        { let _ = D::call(c, fid::Vec3_Add, &[(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as u64), (((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as u64), ((fp.wrapping_add(240)) as u64)]); };
                        { let _ = D::call(c, fid::Vec3_Add, &[(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as u64), (((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as u64), ((fp.wrapping_add(240)) as u64)]); };
                        { let _ = D::call(c, fid::Vec3_Sub, &[(((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as u64), (((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as u64), ((fp.wrapping_add(240)) as u64)]); };
                        { let _ = D::call(c, fid::Vec3_Sub, &[(((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as u64), (((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as u64), ((fp.wrapping_add(240)) as u64)]); };
                    }
                } else {
                    { let _ = D::call(c, fid::Vec3_Add, &[(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as u64), (((l_p_5).wrapping_add(48)) as u64), ((fp.wrapping_add(224)) as u64)]); };
                    { let _ = D::call(c, fid::Vec3_Sub, &[(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as u64), (((l_p_5).wrapping_add(48)) as u64), ((fp.wrapping_add(224)) as u64)]); };
                    { let a2_ = F32(c.m.r32((l_p_5).wrapping_add(232))); { let _ = D::call(c, fid::Vec3_Scale, &[((fp.wrapping_add(240)) as u64), (((l_p_5).wrapping_add(64)) as u64), ((a2_).0 as u64)]); } };
                    if (((((((c.m.r8((l_def_3).wrapping_add(7))) as i32)) == (0i32)) as i32)) != 0) {
                        { let _ = D::call(c, fid::Vec3_Add, &[(((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as u64), (((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as u64), ((fp.wrapping_add(240)) as u64)]); };
                        { let _ = D::call(c, fid::Vec3_Add, &[(((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as u64), (((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as u64), ((fp.wrapping_add(240)) as u64)]); };
                    } else {
                        { let _ = D::call(c, fid::Vec3_Add, &[(((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as u64), (((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as u64), ((fp.wrapping_add(240)) as u64)]); };
                        { let _ = D::call(c, fid::Vec3_Add, &[(((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as u64), (((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as u64), ((fp.wrapping_add(240)) as u64)]); };
                        { let _ = D::call(c, fid::Vec3_Sub, &[(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as u64), (((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as u64), ((fp.wrapping_add(240)) as u64)]); };
                        { let _ = D::call(c, fid::Vec3_Sub, &[(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as u64), (((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as u64), ((fp.wrapping_add(240)) as u64)]); };
                    }
                }
                if ((F::Vu0Cur_ProjectPoints(c, fp.wrapping_add(256), fp, 4i32)) != 0) {
                    l_pkt_6 = c.m.r32(sym::gOtCur);
                    { let v_ = (l_pkt_6).wrapping_add((((1i32) as u32)).wrapping_mul(144)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
                    if (((((l_pkt_6) == (((0i32) as u32))) as i32)) != 0) {
                        break 'f;
                    }
                    if (((((((c.m.r8((l_def_3).wrapping_add(10))) as i32)) < (2i32)) as i32)) != 0) {
                        { let v_ = ((92i32) as u64); let a_ = (l_pkt_6).wrapping_add(32); c.m.w64(a_, v_); };
                        { let v_ = ((536870920i32) as u32); let a_ = l_pkt_6; c.m.w32(a_, v_); };
                        { let v_ = ((268435456i32) as u32); let a_ = (l_pkt_6).wrapping_add(8); c.m.w32(a_, v_); };
                        { let v_ = ((1342177288i32) as u32); let a_ = (l_pkt_6).wrapping_add(12); c.m.w32(a_, v_); };
                        { let v_ = 16429131440647602177u64; let a_ = (l_pkt_6).wrapping_add(16); c.m.w64(a_, v_); };
                        { let v_ = ((18599481537995104i64) as u64); let a_ = (l_pkt_6).wrapping_add(24); c.m.w64(a_, v_); };
                        { let v_ = ((0i32) as u32); let a_ = (l_pkt_6).wrapping_add(4); c.m.w32(a_, v_); };
                    } else {
                        { let v_ = ((604i32) as u64); let a_ = (l_pkt_6).wrapping_add(32); c.m.w64(a_, v_); };
                        { let v_ = ((536870920i32) as u32); let a_ = l_pkt_6; c.m.w32(a_, v_); };
                        { let v_ = ((268435456i32) as u32); let a_ = (l_pkt_6).wrapping_add(8); c.m.w32(a_, v_); };
                        { let v_ = ((1342177288i32) as u32); let a_ = (l_pkt_6).wrapping_add(12); c.m.w32(a_, v_); };
                        { let v_ = 16429131440647602177u64; let a_ = (l_pkt_6).wrapping_add(16); c.m.w64(a_, v_); };
                        { let v_ = ((18599481537995120i64) as u64); let a_ = (l_pkt_6).wrapping_add(24); c.m.w64(a_, v_); };
                        { let v_ = ((0i32) as u32); let a_ = (l_pkt_6).wrapping_add(4); c.m.w32(a_, v_); };
                    }
                    {
                        l_i_8 = 0i32;
                        'l4: loop {
                            if !(((((l_i_8) < (4i32)) as i32)) != 0) { break 'l4; }
                            'c5: {
                                l_o_12 = (((((l_i_8) as u32)).wrapping_mul(16u32)) as i32);
                                l_q_13 = rt::fdiv(F32(0x3f800000), rt::i2f((c.m.r32((((fp.wrapping_add(256)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12)).wrapping_add(((l_o_12) as u32))) as i32)));
                                { let v_ = rt::fadd(rt::fmul(rt::i2f(rt::rem_i32(l_i_8, 2i32)), F32(c.m.r32((p_w_1).wrapping_add(484)))), F32(c.m.r32((l_p_5).wrapping_add(96)))); let a_ = ((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(((l_o_12) as u32)); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fadd(rt::fmul(rt::i2f(rt::div_i32(l_i_8, 2i32)), F32(c.m.r32((p_w_1).wrapping_add(488)))), F32(c.m.r32((l_p_5).wrapping_add(104)))); let a_ = (((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)).wrapping_add(((l_o_12) as u32)); c.m.w32(a_, (v_).0); };
                                { let _ = D::call(c, fid::Vec3_Scale, &[(((fp.wrapping_add(160)).wrapping_add(((l_o_12) as u32))) as u64), ((((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(((l_o_12) as u32))) as u64), ((l_q_13).0 as u64)]); };
                            }
                            { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
                        }
                    }
                    F::Vec4_ToInt(c, fp.wrapping_add(320), (l_p_5).wrapping_add(112));
                    l_z_9 = (((((c.m.r32(((fp.wrapping_add(256)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)).wrapping_add((c.m.r32(((fp.wrapping_add(256)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add((c.m.r32(((fp.wrapping_add(256)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add((c.m.r32(((fp.wrapping_add(256)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_shr((10i32) as u32);
                    if (((((c.m.r32((p_w_1).wrapping_add(472)) as i32)) & (16i32))) != 0) {
                        {
                            l_i_8 = 0i32;
                            'l6: loop {
                                if !(((((l_i_8) < (4i32)) as i32)) != 0) { break 'l6; }
                                'c7: {
                                    l_o_14 = (((((l_i_8) as u32)).wrapping_mul(16u32)) as i32);
                                    { let v_ = 16777215i32; let a_ = (((fp.wrapping_add(256)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(((l_o_14) as u32)); c.m.w32(a_, (v_) as u32); };
                                }
                                { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
                            }
                        }
                    }
                    { let v_ = (((c.m.r32(fp.wrapping_add(320)) as i32)) as u8); let a_ = (l_pkt_6).wrapping_add(48); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(320)).wrapping_add(4)) as i32)) as u8); let a_ = ((l_pkt_6).wrapping_add(48)).wrapping_add(1); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(320)).wrapping_add(8)) as i32)) as u8); let a_ = ((l_pkt_6).wrapping_add(48)).wrapping_add(2); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(320)).wrapping_add(12)) as i32)) as u8); let a_ = ((l_pkt_6).wrapping_add(48)).wrapping_add(3); c.m.w8(a_, v_); };
                    { let v_ = F32(c.m.r32(((fp.wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = ((l_pkt_6).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = (((c.m.r32(fp.wrapping_add(320)) as i32)) as u8); let a_ = (l_pkt_6).wrapping_add(72); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(320)).wrapping_add(4)) as i32)) as u8); let a_ = ((l_pkt_6).wrapping_add(72)).wrapping_add(1); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(320)).wrapping_add(8)) as i32)) as u8); let a_ = ((l_pkt_6).wrapping_add(72)).wrapping_add(2); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(320)).wrapping_add(12)) as i32)) as u8); let a_ = ((l_pkt_6).wrapping_add(72)).wrapping_add(3); c.m.w8(a_, v_); };
                    { let v_ = F32(c.m.r32(((fp.wrapping_add(160)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = ((l_pkt_6).wrapping_add(72)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = (((c.m.r32(fp.wrapping_add(320)) as i32)) as u8); let a_ = (l_pkt_6).wrapping_add(96); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(320)).wrapping_add(4)) as i32)) as u8); let a_ = ((l_pkt_6).wrapping_add(96)).wrapping_add(1); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(320)).wrapping_add(8)) as i32)) as u8); let a_ = ((l_pkt_6).wrapping_add(96)).wrapping_add(2); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(320)).wrapping_add(12)) as i32)) as u8); let a_ = ((l_pkt_6).wrapping_add(96)).wrapping_add(3); c.m.w8(a_, v_); };
                    { let v_ = F32(c.m.r32(((fp.wrapping_add(160)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = ((l_pkt_6).wrapping_add(96)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = (((c.m.r32(fp.wrapping_add(320)) as i32)) as u8); let a_ = (l_pkt_6).wrapping_add(120); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(320)).wrapping_add(4)) as i32)) as u8); let a_ = ((l_pkt_6).wrapping_add(120)).wrapping_add(1); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(320)).wrapping_add(8)) as i32)) as u8); let a_ = ((l_pkt_6).wrapping_add(120)).wrapping_add(2); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(320)).wrapping_add(12)) as i32)) as u8); let a_ = ((l_pkt_6).wrapping_add(120)).wrapping_add(3); c.m.w8(a_, v_); };
                    { let v_ = F32(c.m.r32(((fp.wrapping_add(160)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = ((l_pkt_6).wrapping_add(120)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((fp.wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(16)))); let a_ = (l_pkt_6).wrapping_add(56); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32(((fp.wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = ((l_pkt_6).wrapping_add(56)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((fp.wrapping_add(160)).wrapping_add((((1i32) as u32)).wrapping_mul(16)))); let a_ = (l_pkt_6).wrapping_add(80); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32(((fp.wrapping_add(160)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = ((l_pkt_6).wrapping_add(80)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((fp.wrapping_add(160)).wrapping_add((((2i32) as u32)).wrapping_mul(16)))); let a_ = (l_pkt_6).wrapping_add(104); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32(((fp.wrapping_add(160)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = ((l_pkt_6).wrapping_add(104)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((fp.wrapping_add(160)).wrapping_add((((3i32) as u32)).wrapping_mul(16)))); let a_ = (l_pkt_6).wrapping_add(128); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32(((fp.wrapping_add(160)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = ((l_pkt_6).wrapping_add(128)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(256)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (l_pkt_6).wrapping_add(64); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32(((fp.wrapping_add(256)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((l_pkt_6).wrapping_add(64)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32(((fp.wrapping_add(256)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((l_pkt_6).wrapping_add(64)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                    { let v_ = ((255i32) as u64); let a_ = ((l_pkt_6).wrapping_add(64)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                    { let v_ = (((c.m.r32((fp.wrapping_add(256)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (l_pkt_6).wrapping_add(88); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32(((fp.wrapping_add(256)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((l_pkt_6).wrapping_add(88)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32(((fp.wrapping_add(256)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((l_pkt_6).wrapping_add(88)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                    { let v_ = ((255i32) as u64); let a_ = ((l_pkt_6).wrapping_add(88)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                    { let v_ = (((c.m.r32((fp.wrapping_add(256)).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (l_pkt_6).wrapping_add(112); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32(((fp.wrapping_add(256)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((l_pkt_6).wrapping_add(112)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32(((fp.wrapping_add(256)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((l_pkt_6).wrapping_add(112)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                    { let v_ = ((255i32) as u64); let a_ = ((l_pkt_6).wrapping_add(112)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                    { let v_ = (((c.m.r32((fp.wrapping_add(256)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (l_pkt_6).wrapping_add(136); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32(((fp.wrapping_add(256)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((l_pkt_6).wrapping_add(136)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32(((fp.wrapping_add(256)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((l_pkt_6).wrapping_add(136)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                    { let v_ = ((255i32) as u64); let a_ = ((l_pkt_6).wrapping_add(136)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                    { let v_ = c.m.r64((p_w_1).wrapping_add(584)); let a_ = (l_pkt_6).wrapping_add(40); c.m.w64(a_, v_); };
                    l_layer_10 = ((c.m.r8((l_def_3).wrapping_add(10))) as i32);
                    if (((((l_layer_10) >= (2i32)) as i32)) != 0) {
                        { let r_ = 2i32; let n_ = (l_layer_10).wrapping_sub(r_); l_layer_10 = n_; };
                    }
                    if (((((l_z_9) < (0i32)) as i32)) != 0) {
                        l_e_7 = ((c.m.r32(sym::gOtZ)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((l_layer_10) as u32)).wrapping_mul(8));
                    } else {
                        if (((((l_z_9) >= (4096i32)) as i32)) != 0) {
                            l_e_7 = ((c.m.r32(sym::gOtZ)).wrapping_add((((4095i32) as u32)).wrapping_mul(16))).wrapping_add((((l_layer_10) as u32)).wrapping_mul(8));
                        } else {
                            l_e_7 = ((c.m.r32(sym::gOtZ)).wrapping_add((((l_z_9) as u32)).wrapping_mul(16))).wrapping_add((((l_layer_10) as u32)).wrapping_mul(8));
                        }
                    }
                    { let v_ = l_pkt_6; let a_ = (c.m.r32((l_e_7).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
                    { let v_ = l_pkt_6; let a_ = (l_e_7).wrapping_add(4); c.m.w32(a_, v_); };
                }
            }
            l_link_4 = (l_p_5).wrapping_add(284);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_DrawAxisQuads", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtcl_DrawAxisPolys(c: &mut Ctx, mut p_w_1: u32, mut p_w2_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_DrawAxisPolys", &PROBE_N, &[(p_w_1) as u32, (p_w2_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(688) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 688);
    let mut l_def_3: u32 = 0;
    let mut l_link_4: u32 = 0;
    let mut l_p_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_o_7: i32 = 0;
    let mut l_o_8: i32 = 0;
    let r_ = 'f: {
        l_def_3 = c.m.r32((p_w2_2).wrapping_add(52));
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(544); { let _ = D::call(c, fid::Vec4_Copy, &[((fp.wrapping_add(64)) as u64), ((a1_) as u64)]); } };
        {
            l_i_6 = 0i32;
            'l1: loop {
                if !(((((l_i_6) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_o_7 = (((((l_i_6) as u32)).wrapping_mul(16u32)) as i32);
                    { let v_ = F32(0x3f800000); let a_ = (((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(((l_o_7) as u32)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = (((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12)).wrapping_add(((l_o_7) as u32)); c.m.w32(a_, (v_).0); };
                    { let _ = D::call(c, fid::Vec4_Set, &[(((fp).wrapping_add((((l_i_6) as u32)).wrapping_mul(16))) as u64), ((F32(0x00000000)).0 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
        l_link_4 = (p_w_1).wrapping_add(464);
        'l3: while (((((c.m.r32(l_link_4)) != (((0i32) as u32))) as i32)) != 0) {
            l_p_5 = c.m.r32(l_link_4);
            if (((((c.m.r32((l_p_5).wrapping_add(160)) as i32)) & (1024i32))) != 0) {
                { let _ = D::call(c, fid::Vec3_Sub, &[((fp.wrapping_add(80)) as u64), (((l_p_5).wrapping_add(48)) as u64), ((fp.wrapping_add(64)) as u64)]); };
                F::Vec3_Cross(c, fp.wrapping_add(80), (l_p_5).wrapping_add(64), fp.wrapping_add(80));
                { let _ = D::call(c, fid::Vec3_Normalize, &[((fp.wrapping_add(80)) as u64), ((fp.wrapping_add(80)) as u64)]); };
                { let a2_ = F32(c.m.r32((l_p_5).wrapping_add(216))); { let _ = D::call(c, fid::Vec3_Scale, &[((fp.wrapping_add(160)) as u64), ((fp.wrapping_add(80)) as u64), ((a2_).0 as u64)]); } };
                if (((((c.m.r32(l_def_3) as i32)) & (128i32))) != 0) {
                    { let _ = D::call(c, fid::Vec3_Add, &[(((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as u64), (((l_p_5).wrapping_add(48)) as u64), ((fp.wrapping_add(160)) as u64)]); };
                    { let _ = D::call(c, fid::Vec3_Sub, &[(((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as u64), (((l_p_5).wrapping_add(48)) as u64), ((fp.wrapping_add(160)) as u64)]); };
                    { let a2_ = F32(c.m.r32((l_p_5).wrapping_add(232))); { let _ = D::call(c, fid::Vec3_Scale, &[((fp.wrapping_add(176)) as u64), (((l_p_5).wrapping_add(64)) as u64), ((a2_).0 as u64)]); } };
                    if (((((((c.m.r8((l_def_3).wrapping_add(7))) as i32)) == (0i32)) as i32)) != 0) {
                        { let _ = D::call(c, fid::Vec3_Add, &[(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as u64), (((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as u64), ((fp.wrapping_add(176)) as u64)]); };
                        { let _ = D::call(c, fid::Vec3_Add, &[(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as u64), (((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as u64), ((fp.wrapping_add(176)) as u64)]); };
                    } else {
                        { let _ = D::call(c, fid::Vec3_Add, &[(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as u64), (((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as u64), ((fp.wrapping_add(176)) as u64)]); };
                        { let _ = D::call(c, fid::Vec3_Add, &[(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as u64), (((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as u64), ((fp.wrapping_add(176)) as u64)]); };
                        { let _ = D::call(c, fid::Vec3_Sub, &[(((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as u64), (((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as u64), ((fp.wrapping_add(176)) as u64)]); };
                        { let _ = D::call(c, fid::Vec3_Sub, &[(((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as u64), (((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as u64), ((fp.wrapping_add(176)) as u64)]); };
                    }
                } else {
                    { let _ = D::call(c, fid::Vec3_Add, &[(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as u64), (((l_p_5).wrapping_add(48)) as u64), ((fp.wrapping_add(160)) as u64)]); };
                    { let _ = D::call(c, fid::Vec3_Sub, &[(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as u64), (((l_p_5).wrapping_add(48)) as u64), ((fp.wrapping_add(160)) as u64)]); };
                    { let a2_ = F32(c.m.r32((l_p_5).wrapping_add(232))); { let _ = D::call(c, fid::Vec3_Scale, &[((fp.wrapping_add(176)) as u64), (((l_p_5).wrapping_add(64)) as u64), ((a2_).0 as u64)]); } };
                    if (((((((c.m.r8((l_def_3).wrapping_add(7))) as i32)) == (0i32)) as i32)) != 0) {
                        { let _ = D::call(c, fid::Vec3_Add, &[(((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as u64), (((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as u64), ((fp.wrapping_add(176)) as u64)]); };
                        { let _ = D::call(c, fid::Vec3_Add, &[(((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as u64), (((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as u64), ((fp.wrapping_add(176)) as u64)]); };
                    } else {
                        { let _ = D::call(c, fid::Vec3_Add, &[(((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as u64), (((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as u64), ((fp.wrapping_add(176)) as u64)]); };
                        { let _ = D::call(c, fid::Vec3_Add, &[(((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as u64), (((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as u64), ((fp.wrapping_add(176)) as u64)]); };
                        { let _ = D::call(c, fid::Vec3_Sub, &[(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as u64), (((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as u64), ((fp.wrapping_add(176)) as u64)]); };
                        { let _ = D::call(c, fid::Vec3_Sub, &[(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as u64), (((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as u64), ((fp.wrapping_add(176)) as u64)]); };
                    }
                }
                {
                    l_i_6 = 0i32;
                    'l4: loop {
                        if !(((((l_i_6) < (4i32)) as i32)) != 0) { break 'l4; }
                        'c5: {
                            l_o_8 = (((((l_i_6) as u32)).wrapping_mul(16u32)) as i32);
                            { let v_ = rt::fadd(rt::fmul(rt::i2f(rt::rem_i32(l_i_6, 2i32)), F32(c.m.r32((p_w_1).wrapping_add(484)))), F32(c.m.r32((l_p_5).wrapping_add(96)))); let a_ = ((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(((l_o_8) as u32)); c.m.w32(a_, (v_).0); };
                            { let v_ = rt::fadd(rt::fmul(rt::i2f(rt::div_i32(l_i_6, 2i32)), F32(c.m.r32((p_w_1).wrapping_add(488)))), F32(c.m.r32((l_p_5).wrapping_add(104)))); let a_ = (((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)).wrapping_add(((l_o_8) as u32)); c.m.w32(a_, (v_).0); };
                        }
                        { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                    }
                }
                { let _ = F::Vu0Cur_ProjectPoints(c, fp.wrapping_add(192), fp, 4i32); };
                {
                    l_i_6 = 0i32;
                    'l6: loop {
                        if !(((((l_i_6) < (2i32)) as i32)) != 0) { break 'l6; }
                        'c7: {
                            F::ClipVtx_Set(c, (fp.wrapping_add(256)).wrapping_add((((0i32) as u32)).wrapping_mul(48)), (fp).wrapping_add((((l_i_6) as u32)).wrapping_mul(16)), (fp.wrapping_add(96)).wrapping_add((((l_i_6) as u32)).wrapping_mul(16)), (l_p_5).wrapping_add(112));
                            F::ClipVtx_Set(c, (fp.wrapping_add(256)).wrapping_add((((1i32) as u32)).wrapping_mul(48)), (fp).wrapping_add(((((l_i_6).wrapping_add(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(96)).wrapping_add(((((l_i_6).wrapping_add(1i32)) as u32)).wrapping_mul(16)), (l_p_5).wrapping_add(112));
                            F::ClipVtx_Set(c, (fp.wrapping_add(256)).wrapping_add((((2i32) as u32)).wrapping_mul(48)), (fp).wrapping_add(((((l_i_6).wrapping_add(2i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(96)).wrapping_add(((((l_i_6).wrapping_add(2i32)) as u32)).wrapping_mul(16)), (l_p_5).wrapping_add(112));
                            { let a1_ = ((c.m.r8((l_def_3).wrapping_add(10))) as i32); let a4_ = ((((c.m.r32((p_w_1).wrapping_add(472)) as i32)).wrapping_shr((4i32) as u32)) & (1i32)); let a6_ = c.m.r64((p_w_1).wrapping_add(584)); F::EftGfx_DrawPolyScaledZ(c, fp.wrapping_add(256), a1_, 0i32, 0i32, a4_, 0i32, a6_, F32(0x40000000)) };
                        }
                        { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                    }
                }
            }
            l_link_4 = (l_p_5).wrapping_add(284);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_DrawAxisPolys", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtcl_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let mut l_def_4: u32 = 0;
    let mut l_cur_5: u32 = 0;
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    l_def_4 = c.m.r32((p_arg_2).wrapping_add(52));
    { let _ = F::memset(c, l_w_3, 0i32, 592u32); };
    l_cur_5 = (l_w_3).wrapping_add(64);
    { let s_ = p_arg_2; let d_ = l_w_3; rt::memcpy(c, d_, s_, 64); };
    { let r_ = 1i32; let a_ = (l_w_3).wrapping_add(472); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    's1: {
        let sv6: i32 = ((c.m.r8((l_def_4).wrapping_add(6))) as i32);
        match sv6 {
            0i32 => {
                { let r_ = 512i32; let a_ = (l_w_3).wrapping_add(472); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            1i32 => {
                if ((((!((((F::Port_Rand(c)) & (1i32))) != 0)) as i32)) != 0) {
                    { let r_ = 512i32; let a_ = (l_w_3).wrapping_add(472); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
                break 's1;
            }
            2i32 => {
                break 's1;
            }
            _ => {
            }
        }
    }
    if (((rt::fle(F32(c.m.r32((p_arg_2).wrapping_add(40))), F32(0x00000000)) as i32)) != 0) {
        { let r_ = 8i32; let a_ = (l_w_3).wrapping_add(472); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        { let v_ = rt::fmul(F32(c.m.r32((p_arg_2).wrapping_add(40))), F32(0x41f00000)); let a_ = (l_w_3).wrapping_add(500); c.m.w32(a_, (v_).0); };
    }
    if (((((c.m.r32(l_def_4) as i32)) & (1i32))) != 0) {
        { let v_ = rt::fmul(F32(c.m.r32((l_def_4).wrapping_add(20))), F32(0x41f00000)); let a_ = (l_w_3).wrapping_add(528); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32((l_w_3).wrapping_add(528))), F32(c.m.r32((l_def_4).wrapping_add(24)))); let a_ = (l_w_3).wrapping_add(532); c.m.w32(a_, (v_).0); };
        { let _ = D::call(c, fid::EftPtcl_SetKey, &[((l_cur_5) as u64), ((l_w_3) as u64), ((0i32) as i64 as u64)]); };
    } else {
        { let _ = D::call(c, fid::EftPtcl_SetKey, &[((l_cur_5) as u64), ((l_w_3) as u64), ((2i32) as i64 as u64)]); };
    }
    F::Vec3_Clamp(c, (p_arg_2).wrapping_add(16), (p_arg_2).wrapping_add(16), F32(0xbf800000), F32(0x3f800000));
    { let v_ = { let a0_ = rt::fneg(F32(c.m.r32(((p_arg_2).wrapping_add(16)).wrapping_add(4)))); F::Mathf_Asin(c, a0_) }; let a_ = (l_w_3).wrapping_add(540); c.m.w32(a_, (v_).0); };
    { let v_ = { let a0_ = F32(c.m.r32((p_arg_2).wrapping_add(16))); let a1_ = F32(c.m.r32(((p_arg_2).wrapping_add(16)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) }; let a_ = (l_w_3).wrapping_add(544); c.m.w32(a_, (v_).0); };
    { let a1_ = c.m.r32(sym::gEftPtcl); let a2_ = F32(c.m.r32((l_w_3).wrapping_add(540))); { let _ = D::call(c, fid::Mtx_RotateX, &[(((l_w_3).wrapping_add(368)) as u64), ((a1_) as u64), ((a2_).0 as u64)]); } };
    { let a2_ = F32(c.m.r32((l_w_3).wrapping_add(544))); { let _ = D::call(c, fid::Mtx_RotateY, &[(((l_w_3).wrapping_add(368)) as u64), (((l_w_3).wrapping_add(368)) as u64), ((a2_).0 as u64)]); } };
    { let v_ = rt::fadd(F32(c.m.r32((l_cur_5).wrapping_add(248))), rt::fmul(F32(c.m.r32((l_cur_5).wrapping_add(252))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = (l_w_3).wrapping_add(548); c.m.w32(a_, (v_).0); };
    { let v_ = { let a0_ = F32(c.m.r32((l_w_3).wrapping_add(548))); F::EftMath_WrapAngle(c, a0_) }; let a_ = (l_w_3).wrapping_add(548); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(rt::fadd(F32(c.m.r32((l_def_4).wrapping_add(640))), rt::fmul(F32(c.m.r32((l_def_4).wrapping_add(644))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))), F32(0x40490fda)); let a_ = (l_w_3).wrapping_add(432); c.m.w32(a_, (v_).0); };
    { let v_ = { let a0_ = F32(c.m.r32((l_w_3).wrapping_add(432))); F::EftMath_WrapAngle(c, a0_) }; let a_ = (l_w_3).wrapping_add(432); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(rt::fadd(F32(c.m.r32((l_def_4).wrapping_add(648))), rt::fmul(F32(c.m.r32((l_def_4).wrapping_add(652))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))), F32(0x40490fda)); let a_ = (l_w_3).wrapping_add(448); c.m.w32(a_, (v_).0); };
    { let v_ = { let a0_ = F32(c.m.r32((l_w_3).wrapping_add(448))); F::EftMath_WrapAngle(c, a0_) }; let a_ = (l_w_3).wrapping_add(448); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(rt::fadd(F32(c.m.r32((l_def_4).wrapping_add(656))), rt::fmul(F32(c.m.r32((l_def_4).wrapping_add(660))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))), F32(0x40490fda)); let a_ = ((l_w_3).wrapping_add(432)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = { let a0_ = F32(c.m.r32(((l_w_3).wrapping_add(432)).wrapping_add(4))); F::EftMath_WrapAngle(c, a0_) }; let a_ = ((l_w_3).wrapping_add(432)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(rt::fadd(F32(c.m.r32((l_def_4).wrapping_add(664))), rt::fmul(F32(c.m.r32((l_def_4).wrapping_add(668))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))), F32(0x40490fda)); let a_ = ((l_w_3).wrapping_add(448)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = { let a0_ = F32(c.m.r32(((l_w_3).wrapping_add(448)).wrapping_add(4))); F::EftMath_WrapAngle(c, a0_) }; let a_ = ((l_w_3).wrapping_add(448)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(rt::fadd(F32(c.m.r32((l_def_4).wrapping_add(672))), rt::fmul(F32(c.m.r32((l_def_4).wrapping_add(676))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))), F32(0x40490fda)); let a_ = ((l_w_3).wrapping_add(432)).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = { let a0_ = F32(c.m.r32(((l_w_3).wrapping_add(432)).wrapping_add(8))); F::EftMath_WrapAngle(c, a0_) }; let a_ = ((l_w_3).wrapping_add(432)).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(rt::fadd(F32(c.m.r32((l_def_4).wrapping_add(680))), rt::fmul(F32(c.m.r32((l_def_4).wrapping_add(684))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))), F32(0x40490fda)); let a_ = ((l_w_3).wrapping_add(448)).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = { let a0_ = F32(c.m.r32(((l_w_3).wrapping_add(448)).wrapping_add(8))); F::EftMath_WrapAngle(c, a0_) }; let a_ = ((l_w_3).wrapping_add(448)).wrapping_add(8); c.m.w32(a_, (v_).0); };
    if (((((c.m.r32(l_def_4) as i32)) & (4i32))) != 0) {
        { let v_ = (((c.m.r8((l_def_4).wrapping_add(14))) as i32)).wrapping_mul(((c.m.r8((l_def_4).wrapping_add(15))) as i32)); let a_ = (l_w_3).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
    }
    if ((((((c.m.r32((l_w_3).wrapping_add(480)) as i32)) >= (2i32)) as i32)) != 0) {
        { let r_ = 32i32; let a_ = (l_w_3).wrapping_add(472); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let v_ = rt::fdiv(F32(0x3f800000), rt::i2f(((c.m.r8((l_def_4).wrapping_add(14))) as i32))); let a_ = (l_w_3).wrapping_add(484); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(F32(0x3f800000), rt::i2f(((c.m.r8((l_def_4).wrapping_add(15))) as i32))); let a_ = (l_w_3).wrapping_add(488); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fmul(F32(c.m.r32((l_def_4).wrapping_add(16))), rt::i2f((c.m.r32((l_w_3).wrapping_add(480)) as i32))), F32(0x41f00000)); let a_ = (l_w_3).wrapping_add(496); c.m.w32(a_, (v_).0); };
        if (((rt::flt(F32(c.m.r32((l_w_3).wrapping_add(496))), rt::i2f((c.m.r32((l_w_3).wrapping_add(480)) as i32))) as i32)) != 0) {
            { let v_ = rt::i2f((c.m.r32((l_w_3).wrapping_add(480)) as i32)); let a_ = (l_w_3).wrapping_add(496); c.m.w32(a_, (v_).0); };
        }
    } else {
        { let v_ = F32(0x3f800000); let a_ = (l_w_3).wrapping_add(484); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (l_w_3).wrapping_add(488); c.m.w32(a_, (v_).0); };
    }
    { let a1_ = c.m.r32((p_arg_2).wrapping_add(48)); let a2_ = (c.m.r32((p_arg_2).wrapping_add(36)) as i32); let a3_ = (c.m.r32((p_arg_2).wrapping_add(36)) as i32); { let _ = D::call(c, fid::EftPtcl_PickTexture, &[((l_w_3) as u64), ((a1_) as u64), ((a2_) as i64 as u64), ((a3_) as i64 as u64)]); } };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtcl_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    { let v_ = 0i32; let a_ = (l_w_2).wrapping_add(472); c.m.w32(a_, (v_) as u32); };
    { let _ = D::call(c, fid::EftPtcl_FreePtcls, &[((l_w_2) as u64)]); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtcl_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_cur_3: u32 = 0;
    let mut l_def_4: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_cur_3 = (l_w_2).wrapping_add(64);
    l_def_4 = c.m.r32((l_w_2).wrapping_add(52));
    if ((((!(({ let a0_ = (c.m.r32((l_w_2).wrapping_add(32)) as i32); let a1_ = ((c.m.r8((l_w_2).wrapping_add(478))) as i32); F::BtlScene_IsEffectStopped(c, a0_, a1_) }) != 0)) as i32)) != 0) {
        if (((((c.m.r32((l_w_2).wrapping_add(472)) as i32)) & (2i32))) != 0) {
            if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(512))), F32(0x00000000)) as i32)) != 0) {
                if (((((c.m.r32((l_w_2).wrapping_add(472)) as i32)) & (256i32))) != 0) {
                    { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(516); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(516))), F32(0x00000000)) as i32)) != 0) {
                        { let v_ = F32(0x00000000); let a_ = (l_w_2).wrapping_add(516); c.m.w32(a_, (v_).0); };
                        { let r_ = 4i32; let a_ = (l_w_2).wrapping_add(472); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                } else {
                    if ((((((((c.m.r16((l_w_2).wrapping_add(476)) as i16)) as i32)) <= (0i32)) as i32)) != 0) {
                        { let r_ = 4i32; let a_ = (l_w_2).wrapping_add(472); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                }
            }
        }
        if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(508))), F32(0x00000000)) as i32)) != 0) {
            if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(504))), F32(0x00000000)) as i32)) != 0) {
                if ((((((((((((!(((((c.m.r32((l_w_2).wrapping_add(472)) as i32)) & (2i32))) != 0)) as i32)) != 0) || (((rt::fgt(F32(c.m.r32((l_w_2).wrapping_add(512))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) || (((rt::fgt(F32(c.m.r32((l_w_2).wrapping_add(516))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                    F::EftPtcl_Emit(c, l_w_2, l_w_2);
                }
            } else {
                { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(504); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            }
            { let _ = D::call(c, fid::EftPtcl_StepPtcls, &[((l_w_2) as u64), ((l_w_2) as u64)]); };
            if (((((((((c.m.r32(l_def_4) as i32)) & (1i32))) != 0) && (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(524))), F32(c.m.r32((l_w_2).wrapping_add(528)))) as i32)) != 0)) as i32)) != 0) {
                { let _ = D::call(c, fid::EftPtcl_BlendKeys, &[((l_w_2) as u64)]); };
                { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(524); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(528))), F32(c.m.r32((l_w_2).wrapping_add(524)))) as i32)) != 0) {
                    { let _ = D::call(c, fid::EftPtcl_SetKey, &[((l_cur_3) as u64), ((l_w_2) as u64), ((2i32) as i64 as u64)]); };
                }
            }
            if ((((!(((((c.m.r32((l_w_2).wrapping_add(472)) as i32)) & (8i32))) != 0)) as i32)) != 0) {
                { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(500); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(500))), F32(0x00000000)) as i32)) != 0) {
                    { let r_ = 2i32; let a_ = (l_w_2).wrapping_add(472); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    if (((rt::fgt(F32(c.m.r32((l_w_2).wrapping_add(520))), F32(0x00000000)) as i32)) != 0) {
                        { let r_ = 256i32; let a_ = (l_w_2).wrapping_add(472); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                }
            }
        } else {
            { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(508); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
        if (((((((((c.m.r32((l_w_2).wrapping_add(472)) as i32)) & (2i32))) != 0) && (((rt::fgt(F32(c.m.r32((l_w_2).wrapping_add(512))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
            { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(512); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
    }
    if (((((c.m.r32((l_w_2).wrapping_add(472)) as i32)) & (4i32))) != 0) {
        F::BtlTask_SetDead(c, p_task_1);
    } else {
        { let _ = D::call(c, fid::EftPtcl_UploadTexture, &[((l_w_2) as u64), ((l_w_2) as u64)]); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtcl_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtcl_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtcl_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_def_3: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_def_3 = c.m.r32((l_w_2).wrapping_add(52));
    if (({ let a0_ = (c.m.r32((l_w_2).wrapping_add(32)) as i32); let a1_ = ((c.m.r8((l_w_2).wrapping_add(478))) as i32); F::BtlScene_IsEffectHidden(c, a0_, a1_) }) != 0) {
        break 'f;
    }
    if (((((((((c.m.r32((l_w_2).wrapping_add(472)) as i32)) & (16i32))) != 0) && ((((!(({ let a0_ = (c.m.r32((l_w_2).wrapping_add(32)) as i32); F::BtlScene_IsCharInView(c, a0_) }) != 0)) as i32)) != 0)) as i32)) != 0) {
        break 'f;
    }
    F::Vu0Cur_Push(c);
    { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
    's1: {
        let sv4: i32 = ((c.m.r8((l_def_3).wrapping_add(5))) as i32);
        match sv4 {
            0i32 => {
                if (((((c.m.r32(l_def_3) as i32)) & (2i32))) != 0) {
                    F::EftPtcl_DrawAxisPolys(c, l_w_2, l_w_2);
                } else {
                    F::EftPtcl_DrawAxisQuads(c, l_w_2, l_w_2);
                }
                break 's1;
            }
            1i32 => {
                F::EftPtcl_DrawSquares(c, l_w_2, l_w_2);
                break 's1;
            }
            2i32 => {
                F::EftPtcl_DrawSprites(c, l_w_2, l_w_2);
                break 's1;
            }
            _ => {
            }
        }
    }
    F::Vu0Cur_Pop(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtclMgr_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtclMgr_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((80u32) as i32)) }; let a_ = sym::gEftPtcl; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gEftPtcl); F::memset(c, a0_, 0i32, 80u32) }; };
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, (((((500i32) as u32)).wrapping_mul(304u32)) as i32)) }; let a_ = (c.m.r32(sym::gEftPtcl)).wrapping_add(64); c.m.w32(a_, v_); };
    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gEftPtcl)).wrapping_add(64); c.m.r32(a_) }; F::memset(c, a0_, 0i32, (((500i32) as u32)).wrapping_mul(304u32)) }; };
    { let a0_ = c.m.r32(sym::gEftPtcl); F::Mtx_StoreIdentity(c, a0_) };
    { let v_ = F::BtlTask_CreateChildList(c, p_task_1, ((((60i32) as i64 as u64)) as u32), ((((((592u32) as i32)) as i64 as u64)) as u32)); let a_ = sym::gEftPtclList; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPtclMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtclMgr_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtclMgr_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = { let a_ = (c.m.r32(sym::gEftPtcl)).wrapping_add(64); c.m.r32(a_) }; F::BtlPool_Free(c, a0_, a1_) };
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftPtcl); F::BtlPool_Free(c, a0_, a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftPtcl; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPtclMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtclMgr_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtclMgr_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPtclMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtclMgr_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtclMgr_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPtclMgr_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtcl_Create(c: &mut Ctx, mut p_arg_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_Create", &PROBE_N, &[(p_arg_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let r_ = 'f: {
        { let s_ = p_arg_1; rt::memcpy(c, fp, s_, 64); }
        break 'f { let a0_ = c.m.r32(sym::gEftPtclList); F::BtlTaskList_AddTail(c, a0_, sym::gEftPtclClass, fp) };
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_Create", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtcl_Stop(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_Stop", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftPtcl_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(472)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let r_ = 2i32; let a_ = (l_w_2).wrapping_add(472); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    if (((rt::fgt(F32(c.m.r32((l_w_2).wrapping_add(520))), F32(0x00000000)) as i32)) != 0) {
        { let r_ = 256i32; let a_ = (l_w_2).wrapping_add(472); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_Stop", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtcl_Kill(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_Kill", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftPtcl_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(472)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let r_ = 4i32; let a_ = (l_w_2).wrapping_add(472); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_Kill", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtcl_IsAlive(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_IsAlive", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftPtcl_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(472)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_IsAlive", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtcl_SetPos(c: &mut Ctx, mut p_task_1: u32, mut p_pos_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_SetPos", &PROBE_N, &[(p_task_1) as u32, (p_pos_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftPtcl_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(472)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    F::Vec3_Copy(c, l_w_3, p_pos_2);
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_SetPos", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtcl_Warp(c: &mut Ctx, mut p_task_1: u32, mut p_pos_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_Warp", &PROBE_N, &[(p_task_1) as u32, (p_pos_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let mut l_link_4: u32 = 0;
    let mut l_p_5: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftPtcl_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(472)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    F::Vec3_Copy(c, l_w_3, p_pos_2);
    l_link_4 = (l_w_3).wrapping_add(464);
    'l1: while (((((c.m.r32(l_link_4)) != (((0i32) as u32))) as i32)) != 0) {
        l_p_5 = c.m.r32(l_link_4);
        if ((((!(((((c.m.r32((l_p_5).wrapping_add(160)) as i32)) & (256i32))) != 0)) as i32)) != 0) {
            { let _ = D::call(c, fid::Vec3_Add, &[(((l_p_5).wrapping_add(32)) as u64), ((l_p_5) as u64), (((l_p_5).wrapping_add(16)) as u64)]); };
            { let _ = D::call(c, fid::Vec3_Add, &[(((l_p_5).wrapping_add(48)) as u64), (((l_p_5).wrapping_add(32)) as u64), ((l_w_3) as u64)]); };
        } else {
            { let _ = D::call(c, fid::Vec3_Add, &[(((l_p_5).wrapping_add(48)) as u64), ((l_p_5) as u64), (((l_p_5).wrapping_add(16)) as u64)]); };
            { let _ = D::call(c, fid::Vec3_Add, &[(((l_p_5).wrapping_add(48)) as u64), (((l_p_5).wrapping_add(48)) as u64), (((l_p_5).wrapping_add(32)) as u64)]); };
        }
        l_link_4 = (l_p_5).wrapping_add(284);
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_Warp", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtcl_SetDir(c: &mut Ctx, mut p_task_1: u32, mut p_dir_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_SetDir", &PROBE_N, &[(p_task_1) as u32, (p_dir_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let mut l_d_4: u32 = 0;
    let mut l_m_5: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftPtcl_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    l_d_4 = (l_w_3).wrapping_add(16);
    l_m_5 = (l_w_3).wrapping_add(368);
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(472)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let _ = D::call(c, fid::Vec3_Normalize, &[((l_d_4) as u64), ((p_dir_2) as u64)]); };
    { let v_ = F32(0x3f800000); let a_ = ((l_w_3).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = { let a0_ = rt::fneg(F32(c.m.r32(((l_w_3).wrapping_add(16)).wrapping_add(4)))); F::Mathf_Asin(c, a0_) }; let a_ = (l_w_3).wrapping_add(540); c.m.w32(a_, (v_).0); };
    { let v_ = { let a0_ = F32(c.m.r32(l_d_4)); let a1_ = F32(c.m.r32(((l_w_3).wrapping_add(16)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) }; let a_ = (l_w_3).wrapping_add(544); c.m.w32(a_, (v_).0); };
    { let a1_ = c.m.r32(sym::gEftPtcl); let a2_ = F32(c.m.r32((l_w_3).wrapping_add(540))); { let _ = D::call(c, fid::Mtx_RotateX, &[((l_m_5) as u64), ((a1_) as u64), ((a2_).0 as u64)]); } };
    { let a2_ = F32(c.m.r32((l_w_3).wrapping_add(544))); { let _ = D::call(c, fid::Mtx_RotateY, &[((l_m_5) as u64), ((l_m_5) as u64), ((a2_).0 as u64)]); } };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_SetDir", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtcl_SetSize(c: &mut Ctx, mut p_task_1: u32, mut p_size_2: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_SetSize", &PROBE_N, &[(p_task_1) as u32, (p_size_2).0], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftPtcl_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(472)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let v_ = p_size_2; let a_ = (l_w_3).wrapping_add(44); c.m.w32(a_, (v_).0); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_SetSize", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtcl_SetStartDelay(c: &mut Ctx, mut p_task_1: u32, mut p_frames_2: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_SetStartDelay", &PROBE_N, &[(p_task_1) as u32, (p_frames_2).0], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftPtcl_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(472)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let v_ = p_frames_2; let a_ = (l_w_3).wrapping_add(508); c.m.w32(a_, (v_).0); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_SetStartDelay", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtcl_SetStopDelay(c: &mut Ctx, mut p_task_1: u32, mut p_frames_2: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_SetStopDelay", &PROBE_N, &[(p_task_1) as u32, (p_frames_2).0], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftPtcl_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(472)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let v_ = p_frames_2; let a_ = (l_w_3).wrapping_add(512); c.m.w32(a_, (v_).0); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_SetStopDelay", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtcl_SetLinger(c: &mut Ctx, mut p_task_1: u32, mut p_frames_2: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_SetLinger", &PROBE_N, &[(p_task_1) as u32, (p_frames_2).0], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftPtcl_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(472)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let v_ = p_frames_2; let a_ = (l_w_3).wrapping_add(516); c.m.w32(a_, (v_).0); };
    { let v_ = p_frames_2; let a_ = (l_w_3).wrapping_add(520); c.m.w32(a_, (v_).0); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_SetLinger", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtcl_SetTexture(c: &mut Ctx, mut p_task_1: u32, mut p_res_2: u32, mut p_a_3: i32, mut p_b_4: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_SetTexture", &PROBE_N, &[(p_task_1) as u32, (p_res_2) as u32, (p_a_3) as u32, (p_b_4) as u32], false); }
    let mut l_w_5: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftPtcl_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_5 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_5).wrapping_add(472)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let _ = D::call(c, fid::EftPtcl_PickTexture, &[((l_w_5) as u64), ((p_res_2) as u64), ((p_a_3) as i64 as u64), ((p_b_4) as i64 as u64)]); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_SetTexture", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtcl_SetFront(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_SetFront", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftPtcl_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(472)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let r_ = 16i32; let a_ = (l_w_2).wrapping_add(472); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_SetFront", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtcl_SetFlag40(c: &mut Ctx, mut p_task_1: u32, mut p_on_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_SetFlag40", &PROBE_N, &[(p_task_1) as u32, (p_on_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftPtcl_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(472)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((p_on_2) != 0) {
        { let r_ = 64i32; let a_ = (l_w_3).wrapping_add(472); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        { let r_ = (!(64i32)); let a_ = (l_w_3).wrapping_add(472); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_SetFlag40", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtcl_SetFlag80(c: &mut Ctx, mut p_task_1: u32, mut p_on_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_SetFlag80", &PROBE_N, &[(p_task_1) as u32, (p_on_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftPtcl_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(472)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((p_on_2) != 0) {
        { let r_ = 128i32; let a_ = (l_w_3).wrapping_add(472); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        { let r_ = (!(128i32)); let a_ = (l_w_3).wrapping_add(472); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_SetFlag80", &PROBE_N, &[], true); }
    r_
}

pub fn EftPtcl_SetType(c: &mut Ctx, mut p_task_1: u32, mut p_type_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_SetType", &PROBE_N, &[(p_task_1) as u32, (p_type_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftPtcl_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(472)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let v_ = ((p_type_2) as u8); let a_ = (l_w_3).wrapping_add(478); c.m.w8(a_, v_); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPtcl_SetType", &PROBE_N, &[], true); }
    r_
}
