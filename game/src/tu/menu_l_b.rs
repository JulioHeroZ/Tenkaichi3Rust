#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn SoloSel_SwapRowTex(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SoloSel_SwapRowTex", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_res_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_chara_3: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 28);
        { let v_ = 8i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
        { let v_ = 11i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
        { let v_ = 12i32; c.m.w32(fp.wrapping_add(8), (v_) as u32); }
        { let v_ = 13i32; c.m.w32(fp.wrapping_add(12), (v_) as u32); }
        { let v_ = 14i32; c.m.w32(fp.wrapping_add(16), (v_) as u32); }
        { let v_ = 15i32; c.m.w32(fp.wrapping_add(20), (v_) as u32); }
        { let v_ = 16i32; c.m.w32(fp.wrapping_add(24), (v_) as u32); }
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (7i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_chara_3 = { let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(548); c.m.r32(a_) }).wrapping_add(((((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)).wrapping_add(l_i_2)) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) };
                    { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(48)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(76)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = l_chara_3; let a_ = (({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(48)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_res_1 = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((((l_chara_3).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
                    { let v_ = { let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(296)).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(296)).wrapping_add(((((18i32).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = (c.m.r32((l_res_1).wrapping_add(16))).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(296)).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SoloSel_SwapRowTex", &PROBE_N, &[], true); }
    r_
}

pub fn SoloSel_SetRowTex(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SoloSel_SetRowTex", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_res_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_chara_3: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 28);
        { let v_ = 8i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
        { let v_ = 11i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
        { let v_ = 12i32; c.m.w32(fp.wrapping_add(8), (v_) as u32); }
        { let v_ = 13i32; c.m.w32(fp.wrapping_add(12), (v_) as u32); }
        { let v_ = 14i32; c.m.w32(fp.wrapping_add(16), (v_) as u32); }
        { let v_ = 15i32; c.m.w32(fp.wrapping_add(20), (v_) as u32); }
        { let v_ = 16i32; c.m.w32(fp.wrapping_add(24), (v_) as u32); }
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (7i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((l_i_2) < ({ let a_ = (({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(548); c.m.r32(a_) }).wrapping_add(((((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)).wrapping_add({ let a_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }; (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(36))).wrapping_add(4); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        l_chara_3 = { let a_ = ((({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(548); c.m.r32(a_) }).wrapping_add(((((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)).wrapping_add({ let a_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }; (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(36))).wrapping_add(8)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                    } else {
                        l_chara_3 = 164i32;
                    }
                    { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(48)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(76)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = l_chara_3; let a_ = (({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(48)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_res_1 = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((((l_chara_3).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
                    { let v_ = { let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(296)).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(296)).wrapping_add(((((18i32).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = (c.m.r32((l_res_1).wrapping_add(16))).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(296)).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SoloSel_SetRowTex", &PROBE_N, &[], true); }
    r_
}

pub fn SoloSel_UpdateImage(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SoloSel_UpdateImage", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        's1: {
            let sv1: i32 = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(404); (c.m.r32(a_) as i32) };
            match sv1 {
                5i32 => {
                    { let v_ = 6i32; let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(404); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                6i32 => {
                    if (((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(104); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                        { let r_ = 1i32; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(404); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                1i32 => {
                    F::File_CancelRequests(c);
                    { let _ = { let a0_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(108); (c.m.r32(a_) as i32) }).wrapping_add(761i32); let a1_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(8); c.m.r32(a_) }; let _ = 92160i32; F::File_Request(c, a0_, a1_) }; };
                    { let v_ = 2i32; let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(404); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                2i32 => {
                    if ((F::File_UpdateRequests(c)) != 0) {
                        { let v_ = 3i32; let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(404); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                3i32 => {
                    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(8); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(12); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, a1_, ((0i32) as u32)) }; };
                    { let v_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(12); c.m.r32(a_) }; let a_ = fp; c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(172)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let r_ = 2i32; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 4i32; let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(404); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                4i32 => {
                    if (((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(104); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(172)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        { let v_ = 6i32; let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(404); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SoloSel_UpdateImage", &PROBE_N, &[], true); }
    r_
}

pub fn SoloSel_ChangeImage(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SoloSel_ChangeImage", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(104); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
        { let r_ = 2i32; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        { let v_ = 5i32; let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(404); c.m.w32(a_, (v_) as u32); };
    }
    { let r_ = 1i32; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SoloSel_ChangeImage", &PROBE_N, &[], true); }
    r_
}

pub fn SoloSel_ClipGoto(c: &mut Ctx, mut p_movie_1: i32, mut p_kind_2: i32, mut p_label_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SoloSel_ClipGoto", &PROBE_N, &[(p_movie_1) as u32, (p_kind_2) as u32, (p_label_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_flash_4: u32 = 0;
    let r_ = 'f: {
        l_flash_4 = ((c.m.r32(sym::gSoloSel)).wrapping_add(36)).wrapping_add((((p_movie_1) as u32)).wrapping_mul(44));
        's1: {
            let sv5: i32 = p_kind_2;
            match sv5 {
                0i32 => {
                    { let _ = { let a2_ = { let a_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }; (c.m.r32(a_) as i32) }; F::sprintf(c, fp.wrapping_add(8), sym::menu_l_b__str0, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                1i32 => {
                    { let _ = { let a2_ = { let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(8); (c.m.r32(a_) as i32) }; F::sprintf(c, fp.wrapping_add(8), sym::menu_l_b__str1, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                2i32 => {
                    { let _ = { let a2_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(20); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_l_b__str2, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                5i32 => {
                    { let _ = { let a2_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(24); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_l_b__str3, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                3i32 | 6i32 => {
                    break 'f;
                }
                _ => {
                }
            }
        }
        F::Flash_FindLabel(c, l_flash_4, ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, l_flash_4, fp, p_label_3);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SoloSel_ClipGoto", &PROBE_N, &[], true); }
    r_
}

pub fn SoloSel_Init(c: &mut Ctx, mut p_section_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SoloSel_Init", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_2: i32 = 0;
    let mut l_cols_3: i32 = 0;
    let mut l_cols2_4: i32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = F::Heap_Alloc(c, 6820i32, ((32i32) as u32), 0i32, 2i32); let a_ = sym::gSoloSel; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gSoloSel); F::memset(c, a0_, 0i32, ((6820i32) as u32)) }; };
        { let v_ = (c.m.r32(sym::gMenuArc3)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc3)).wrapping_add((((p_section_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = c.m.r32(sym::gSoloSel); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gSoloSel); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.w32(a_, v_); };
        if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); (c.m.r32(a_) as i32) }) == (21i32)) as i32)) != 0) {
            { let r_ = 64i32; let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(396); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        { let v_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((32i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = c.m.r32(fp); let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(168); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((33i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(172)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(172)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(172)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(172)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(172)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(172)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(172)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(172)).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((34i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(172)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(172)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(172)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(172)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(172)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((8i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(172)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((31i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gSoloSel)).wrapping_add(172); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(296)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(296)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(296)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(296)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(296)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(296)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(296)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(296)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(296)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(296)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(252)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(36)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gSoloSel)).wrapping_add(296); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(36)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(252)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(252)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(252)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(252)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(252)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(252)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(252)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(252)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(252)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(252)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gSoloSel)).wrapping_add(252); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::ItemPanel_Init(c, a0_, 0i32) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(28); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(32); c.m.w32(a_, v_); };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(28); c.m.r32(a_) }; let _ = 0i32; F::MsgWin_Init(c, a0_, a1_, 1i32) };
        F::MsgWin_Open(c);
        { let a0_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::ItemHelp_Init(c, a0_) };
        { let v_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6816); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(20); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(24); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(16); c.m.w32(a_, v_); };
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (165i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((((l_i_2).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = (({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((30i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32))).wrapping_add(16); let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(548); c.m.w32(a_, v_); };
        { let v_ = (({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((30i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) as i32); let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(544); c.m.w32(a_, (v_) as u32); };
        { let a0_ = (c.m.r32(sym::gSoloSel)).wrapping_add(552); let a1_ = (c.m.r32(sym::gSoloSel)).wrapping_add(556); let a2_ = (c.m.r32(sym::gSoloSel)).wrapping_add(544); let a3_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(548); c.m.r32(a_) }; F::ChrGrid_Build(c, a0_, a1_, a2_, a3_, ((0i32) as u32), ((0i32) as u32)) };
        { let v_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(552); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(544); c.m.w32(a_, (v_) as u32); };
        { let v_ = (c.m.r32(sym::gSoloSel)).wrapping_add(556); let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(548); c.m.w32(a_, v_); };
        {
            l_cols_3 = 7i32;
            l_cols2_4 = 7i32;
            { let v_ = rt::div_i32({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(544); (c.m.r32(a_) as i32) }, l_cols_3); let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6496); c.m.w32(a_, (v_) as u32); };
            if ((rt::rem_i32({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(544); (c.m.r32(a_) as i32) }, l_cols2_4)) != 0) {
                { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6496); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
        }
        { let v_ = F::Heap_Alloc(c, 92160i32, ((64i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = F::Heap_Alloc(c, 133120i32, ((32i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32(sym::gSoloSel)).wrapping_add(420); let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.w32(a_, v_); };
        { let s_ = (c.m.r32(sym::gProgress)).wrapping_add(1088); let d_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }; rt::memcpy(c, d_, s_, 48); };
        { let v_ = 4i32; let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(404); c.m.w32(a_, (v_) as u32); };
        if ((((!(({ let a0_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(548); c.m.r32(a_) }; let a1_ = ({ let a_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }; (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)); F::ChrGrid_IsSelectable(c, a0_, a1_) }) != 0)) as i32)) != 0) {
            { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }; F::memset(c, a0_, 0i32, 48u32) }; };
            { let v_ = 0i32; let a_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }; c.m.w32(a_, (v_) as u32); };
            { let s_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }; let d_ = (c.m.r32(sym::gProgress)).wrapping_add(1088); rt::memcpy(c, d_, s_, 48); };
        }
        F::SoloSel_SwapRowTex(c);
        { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(48)).wrapping_add(((({ let a_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }; (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
        {
            l_i_2 = 0i32;
            'l3: loop {
                if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let v_ = ((F::Rand_Range(c, ((32i32) as u32))) as i32); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(6520)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(400); c.m.w32(a_, (v_) as u32); };
        { let _ = { let a0_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(108); (c.m.r32(a_) as i32) }).wrapping_add(761i32); let a1_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(8); c.m.r32(a_) }; F::File_LoadSync(c, a0_, a1_, 92160i32) }; };
        { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(8); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(12); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, a1_, ((0i32) as u32)) }; };
        { let v_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(12); c.m.r32(a_) }; let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(172)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let r_ = 2i32; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let a0_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(6536)).wrapping_add((((0i32) as u32)).wrapping_mul(140)); let a1_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(20); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, ((1i32) as u32)) };
        { let a0_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(6536)).wrapping_add((((0i32) as u32)).wrapping_mul(140)); F::TextBox_SetUnk80(c, a0_, 1i32) };
        { let a0_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(6536)).wrapping_add((((1i32) as u32)).wrapping_mul(140)); let a1_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(24); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, ((3i32) as u32)) };
        { let a0_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(6536)).wrapping_add((((1i32) as u32)).wrapping_mul(140)); F::TextBox_SetUnk80(c, a0_, 1i32) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SoloSel_Init", &PROBE_N, &[], true); }
    r_
}

pub fn SoloSel_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SoloSel_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    F::ItemHelp_Term(c);
    F::MsgWin_Term(c);
    F::ItemPanel_Term(c, 0i32);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (3i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(36)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Destroy(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(12); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(12); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(12); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(8); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(8); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(8); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gSoloSel)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gSoloSel); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gSoloSel; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SoloSel_Term", &PROBE_N, &[], true); }
    r_
}

pub fn SoloSel_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SoloSel_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(160) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 160);
    let mut l_i_1: i32 = 0;
    let mut l_k_2: i32 = 0;
    let mut l_flash_3: u32 = 0;
    let r_ = 'f: {
        { let a0_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(168); c.m.r32(a_) }; F::Sprite_DrawPicture(c, a0_, 0i32, 0i32, 128i32) };
        l_flash_3 = ((c.m.r32(sym::gSoloSel)).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        {
            l_k_2 = 0i32;
            'l1: loop {
                if !(((((l_k_2) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_l_b__str4, &[(((l_k_2).wrapping_add(17i32)) as i64 as u64)]); };
                    { let _ = F::sprintf(c, fp.wrapping_add(92), sym::menu_l_b__str5, &[(((l_k_2).wrapping_add(17i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), fp.wrapping_add(92), fp);
                    { let a2_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(6520)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)); F::FlashAnim_Blink(c, l_flash_3, fp, a2_, 0i32) };
                    { let _ = F::sprintf(c, fp.wrapping_add(92), sym::menu_l_b__str6, &[(((l_k_2).wrapping_add(17i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), fp.wrapping_add(92), fp);
                    if ((((({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6516); (c.m.r32(a_) as i32) }) == (l_k_2)) as i32)) != 0) {
                        { let a2_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(6528)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)); F::FlashAnim_Talk(c, l_flash_3, fp, a2_, 0i32) };
                    } else {
                        F::FlashAnim_ShowNext2(c, l_flash_3, fp, 0i32);
                    }
                }
                { let o_ = l_k_2; let n_ = (o_).wrapping_add(1); l_k_2 = n_; };
            }
        }
        if (((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(104); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
            { let r_ = F32(0x3d999999); let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6508); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::fge({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6508); F32(c.m.r32(a_)) }, F32(0x3f800000)) as i32)) != 0) {
                { let v_ = F32(0x3f800000); let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6508); c.m.w32(a_, (v_).0); };
            }
        } else {
            { let v_ = F32(0x00000000); let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6508); c.m.w32(a_, (v_).0); };
        }
        F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), sym::menu_l_b__str7, fp);
        { let a2_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6508); F32(c.m.r32(a_)) }; F::Flash_ClipSetAlpha(c, l_flash_3, fp, a2_) };
        F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), sym::menu_l_b__str8, fp);
        { let a4_ = { let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(108); (c.m.r32(a_) as i32) }; let a5_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(6536)).wrapping_add((((0i32) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_3, fp, 0i32, 0i32, a4_, a5_) };
        F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), sym::menu_l_b__str9, fp);
        { let a4_ = { let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(108); (c.m.r32(a_) as i32) }; let a5_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(6536)).wrapping_add((((1i32) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_3, fp, 0i32, 0i32, a4_, a5_) };
        l_flash_3 = ((c.m.r32(sym::gSoloSel)).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(44));
        { let v_ = 32i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), sym::menu_l_b__str10, fp);
        F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
        {
            l_i_1 = 0i32;
            'l3: loop {
                if !(((((l_i_1) < (4i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (l_i_1).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_l_b__str11, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), fp.wrapping_add(28), fp);
                    { let a2_ = (if (((((l_i_1) < ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6500); (c.m.r32(a_) as i32) })) as i32)) != 0) { F32(0x3f800000) } else { F32(0x3e999999) }); F::Flash_ClipSetColor(c, l_flash_3, fp, a2_) };
                    F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_l_b__str12, fp);
                    F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                    F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_l_b__str13, fp);
                    F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                    if (((((l_i_1) == (0i32)) as i32)) != 0) {
                        F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_l_b__str14, fp);
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    }
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        {
            l_i_1 = 0i32;
            'l5: loop {
                if !(((((l_i_1) < (4i32)) as i32)) != 0) { break 'l5; }
                'c6: {
                    { let v_ = (rt::rem_i32(l_i_1, 2i32)).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (rt::div_i32(l_i_1, 2i32)).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_l_b__str15, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), fp.wrapping_add(28), fp);
                    if (((((l_i_1) < ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6504); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                        's7: {
                            let sv4: i32 = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6504); (c.m.r32(a_) as i32) };
                            match sv4 {
                                2i32 => {
                                    F::Flash_ClipSetOffset(c, l_flash_3, fp, 45i32, 0i32);
                                    break 's7;
                                }
                                3i32 => {
                                    F::Flash_ClipSetOffset(c, l_flash_3, fp, 22i32, 0i32);
                                    break 's7;
                                }
                                _ => {
                                }
                            }
                        }
                        F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_l_b__str16, fp);
                        F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                        F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_l_b__str17, fp);
                        F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                    } else {
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    }
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        l_flash_3 = ((c.m.r32(sym::gSoloSel)).wrapping_add(36)).wrapping_add((((1i32) as u32)).wrapping_mul(44));
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), sym::menu_l_b__str18, fp);
        F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
        if ((((({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6496); (c.m.r32(a_) as i32) }) >= (2i32)) as i32)) != 0) {
            {
                l_i_1 = 0i32;
                'l8: loop {
                    if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l8; }
                    'c9: {
                        { let v_ = (l_i_1).wrapping_mul(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        F::Flash_FindLabel(c, l_flash_3, (if (((((l_i_1) != (0i32)) as i32)) != 0) { sym::menu_l_b__str19 } else { sym::menu_l_b__str20 }), (if (((((l_i_1) != (0i32)) as i32)) != 0) { sym::menu_l_b__str21 } else { sym::menu_l_b__str22 }), fp);
                        F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                        F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), (if (((((l_i_1) != (0i32)) as i32)) != 0) { sym::menu_l_b__str23 } else { sym::menu_l_b__str24 }), fp);
                        if (((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(104); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                        } else {
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                        }
                    }
                    { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
                }
            }
        } else {
            {
                l_i_1 = 0i32;
                'l10: loop {
                    if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l10; }
                    'c11: {
                        F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), (if (((((l_i_1) != (0i32)) as i32)) != 0) { sym::menu_l_b__str25 } else { sym::menu_l_b__str26 }), fp);
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    }
                    { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
                }
            }
        }
        F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), sym::menu_l_b__str27, fp);
        if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(116); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 258i32, ((((1i32) as i64 as u64)) as u8)); };
        } else {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 258i32, ((((0i32) as i64 as u64)) as u8)); };
        }
        {
            l_i_1 = 0i32;
            'l12: loop {
                if !(((((l_i_1) < (14i32)) as i32)) != 0) { break 'l12; }
                'c13: {
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_l_b__str28, &[((l_i_1) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), fp.wrapping_add(28), fp);
                    { let a3_ = (((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(116); (c.m.r32(a_) as i32) }) as u8)) as i32); { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 128i32, ((((a3_) as i64 as u64)) as u8)); } };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        { let a0_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
        { let a0_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(36)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
        F::Font_FlushAll(c);
        { let a0_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
        { let a2_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(400); (c.m.r32(a_) as i32) }; F::MsgWin_Draw(c, 0i32, 0i32, a2_) };
        F::ItemPanel_Draw(c, 0i32);
        { let a0_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(416); (c.m.r32(a_) as i32) }; F::ItemHelp_Draw(c, a0_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SoloSel_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn SoloSel_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SoloSel_Update", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (3i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(36)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Advance(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    F::ItemPanel_Update(c, 0i32);
    if ((((((((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(116); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) && (((({ let a_ = (((c.m.r32(sym::gSoloSel)).wrapping_add(36)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((1i32) as u32)))) != 0)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(116); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SoloSel_Update", &PROBE_N, &[], true); }
    r_
}

pub fn SoloSel_Input(c: &mut Ctx, mut p_result_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SoloSel_Input", &PROBE_N, &[(p_result_1) as u32], false); }
    let mut l_item_2: i32 = 0;
    let r_ = 'f: {
    if ((((!(((({ let a_ = (((c.m.r32(sym::gSoloSel)).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f;
    }
    if ((((!(((({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(396); (c.m.r32(a_) as i32) }) & (2i32))) != 0)) as i32)) != 0) {
        F::SoloSel_ClipGoto(c, 1i32, 0i32, sym::menu_l_b__str29);
        { let r_ = 2i32; let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(396); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    's1: {
        let sv3: i32 = { let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(112); (c.m.r32(a_) as i32) };
        match sv3 {
            0i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) {
                    F::SoloSel_ClipGoto(c, 1i32, 0i32, sym::menu_l_b__str30);
                    { let a0_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(548); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }; let a2_ = { let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }; { let _ = F::ChrGrid_MoveLeft(c, a0_, a1_, a2_); } };
                    F::SoloSel_ClipGoto(c, 1i32, 0i32, sym::menu_l_b__str31);
                    { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(48)).wrapping_add(((({ let a_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }; (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
                    F::SoloSel_ChangeImage(c);
                    { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 0i32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) {
                        F::SoloSel_ClipGoto(c, 1i32, 0i32, sym::menu_l_b__str32);
                        { let a0_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(548); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }; let a2_ = { let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }; { let _ = F::ChrGrid_MoveRight(c, a0_, a1_, a2_); } };
                        F::SoloSel_ClipGoto(c, 1i32, 0i32, sym::menu_l_b__str33);
                        { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(48)).wrapping_add(((({ let a_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }; (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
                        F::SoloSel_ChangeImage(c);
                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 0i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                            if ((((({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6496); (c.m.r32(a_) as i32) }) >= (2i32)) as i32)) != 0) {
                                { let a0_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(36)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_l_b__str34, ((((1i32) as i64 as u64)) as u8)) };
                                { let v_ = 1i32; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(116); c.m.w32(a_, (v_) as u32); };
                                F::SoloSel_ClipGoto(c, 1i32, 0i32, sym::menu_l_b__str35);
                                { let a0_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(548); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }; let a2_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(4); let a3_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6496); (c.m.r32(a_) as i32) }; F::ChrGrid_MoveUp(c, a0_, a1_, a2_, a3_) };
                                F::SoloSel_ClipGoto(c, 1i32, 0i32, sym::menu_l_b__str36);
                                F::SoloSel_SwapRowTex(c);
                                { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(48)).wrapping_add(((({ let a_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }; (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
                                F::SoloSel_ChangeImage(c);
                                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 2i32); };
                            }
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                                if ((((({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6496); (c.m.r32(a_) as i32) }) >= (2i32)) as i32)) != 0) {
                                    { let a0_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(36)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_l_b__str37, ((((1i32) as i64 as u64)) as u8)) };
                                    { let v_ = 1i32; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(116); c.m.w32(a_, (v_) as u32); };
                                    F::SoloSel_ClipGoto(c, 1i32, 0i32, sym::menu_l_b__str38);
                                    { let a0_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(548); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }; let a2_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(4); let a3_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6496); (c.m.r32(a_) as i32) }; F::ChrGrid_MoveDown(c, a0_, a1_, a2_, a3_) };
                                    F::SoloSel_ClipGoto(c, 1i32, 0i32, sym::menu_l_b__str39);
                                    F::SoloSel_SwapRowTex(c);
                                    { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(48)).wrapping_add(((({ let a_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }; (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
                                    F::SoloSel_ChangeImage(c);
                                    { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 1i32); };
                                }
                            } else {
                                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                                    if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(548); c.m.r32(a_) }).wrapping_add(((((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)).wrapping_add({ let a_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }; (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(36))).wrapping_add(4); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                        { let r_ = 64i32; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                        if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(8); (c.m.r32(a_) as i32) }) >= ({ let a_ = (({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(548); c.m.r32(a_) }).wrapping_add(((((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)).wrapping_add({ let a_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }; (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(36))).wrapping_add(4); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                            { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                        }
                                        { let a0_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(36)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_l_b__str40, ((((1i32) as i64 as u64)) as u8)) };
                                        { let v_ = 1i32; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(116); c.m.w32(a_, (v_) as u32); };
                                        F::SoloSel_ClipGoto(c, 1i32, 0i32, sym::menu_l_b__str41);
                                        F::SoloSel_ClipGoto(c, 1i32, 1i32, sym::menu_l_b__str42);
                                        F::SoloSel_SetRowTex(c);
                                        { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(48)).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(8); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
                                        F::SoloSel_ChangeImage(c);
                                        { let v_ = 1i32; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 41i32); };
                                    } else {
                                        { let a0_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_l_b__str43, ((((1i32) as i64 as u64)) as u8)) };
                                        F::SoloSel_ClipGoto(c, 1i32, 0i32, sym::menu_l_b__str44);
                                        { let v_ = (if (((({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(396); (c.m.r32(a_) as i32) }) & (64i32))) != 0) { 1i32 } else { 4i32 }); let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6500); c.m.w32(a_, (v_) as u32); };
                                        if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(20); (c.m.r32(a_) as i32) }) >= ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6500); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                            { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                                        }
                                        { let v_ = { let a0_ = { let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(108); (c.m.r32(a_) as i32) }; let a1_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(24); F::ChrTbl_WrapCostume(c, a0_, a1_) }; let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6504); c.m.w32(a_, (v_) as u32); };
                                        F::SoloSel_ClipGoto(c, 2i32, 2i32, sym::menu_l_b__str45);
                                        { let v_ = 2i32; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                    }
                                } else {
                                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                        F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                                        { let v_ = 0i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                                    }
                                }
                            }
                        }
                    }
                }
                break 's1;
            }
            1i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) {
                    F::SoloSel_ClipGoto(c, 1i32, 1i32, sym::menu_l_b__str46);
                    { let a0_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(48); let a1_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(8); F::ChrGrid_PrevForm(c, a0_, a1_) };
                    F::SoloSel_ClipGoto(c, 1i32, 1i32, sym::menu_l_b__str47);
                    { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(48)).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(8); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
                    F::SoloSel_ChangeImage(c);
                    { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 0i32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) {
                        F::SoloSel_ClipGoto(c, 1i32, 1i32, sym::menu_l_b__str48);
                        { let a0_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(48); let a1_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(8); F::ChrGrid_NextForm(c, a0_, a1_) };
                        F::SoloSel_ClipGoto(c, 1i32, 1i32, sym::menu_l_b__str49);
                        { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(48)).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(8); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
                        F::SoloSel_ChangeImage(c);
                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 0i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                            { let a0_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_l_b__str50, ((((1i32) as i64 as u64)) as u8)) };
                            F::SoloSel_ClipGoto(c, 1i32, 1i32, sym::menu_l_b__str51);
                            { let v_ = (if (((({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(396); (c.m.r32(a_) as i32) }) & (64i32))) != 0) { 1i32 } else { 4i32 }); let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6500); c.m.w32(a_, (v_) as u32); };
                            if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(20); (c.m.r32(a_) as i32) }) >= ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6500); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                            }
                            F::SoloSel_ClipGoto(c, 2i32, 2i32, sym::menu_l_b__str52);
                            { let v_ = { let a0_ = { let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(108); (c.m.r32(a_) as i32) }; let a1_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(24); F::ChrTbl_WrapCostume(c, a0_, a1_) }; let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6504); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 2i32; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                { let r_ = 64i32; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                                { let a0_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(36)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_l_b__str53, ((((1i32) as i64 as u64)) as u8)) };
                                { let v_ = 1i32; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(116); c.m.w32(a_, (v_) as u32); };
                                F::SoloSel_ClipGoto(c, 1i32, 1i32, sym::menu_l_b__str54);
                                F::SoloSel_ClipGoto(c, 1i32, 0i32, sym::menu_l_b__str55);
                                F::SoloSel_SwapRowTex(c);
                                { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(48)).wrapping_add(((({ let a_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }; (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
                                F::SoloSel_ChangeImage(c);
                                { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 41i32); };
                            }
                        }
                    }
                }
                break 's1;
            }
            2i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                    if ((((({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6500); (c.m.r32(a_) as i32) }) >= (2i32)) as i32)) != 0) {
                        F::SoloSel_ClipGoto(c, 2i32, 2i32, sym::menu_l_b__str56);
                        { let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(20); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(20); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                            { let v_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6500); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                        }
                        F::SoloSel_ClipGoto(c, 2i32, 2i32, sym::menu_l_b__str57);
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    }
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                        if ((((({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6500); (c.m.r32(a_) as i32) }) >= (2i32)) as i32)) != 0) {
                            F::SoloSel_ClipGoto(c, 2i32, 2i32, sym::menu_l_b__str58);
                            { let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(20); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(20); (c.m.r32(a_) as i32) }) >= ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6500); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                            }
                            F::SoloSel_ClipGoto(c, 2i32, 2i32, sym::menu_l_b__str59);
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                        }
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((2i32) as u32)))) != 0) {
                            if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(20); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                                F::SoloSel_ClipGoto(c, 2i32, 2i32, sym::menu_l_b__str60);
                                { let a1_ = { let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(108); (c.m.r32(a_) as i32) }; let a2_ = ({ let a_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }; (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)); let a3_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(20); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); F::ItemPanel_SetChara(c, 0i32, a1_, a2_, a3_, 0i32) };
                                F::ItemPanel_Show(c, 0i32);
                                { let r_ = 2048i32; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                { let v_ = 3i32; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                            }
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                                { let a0_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_l_b__str61, ((((1i32) as i64 as u64)) as u8)) };
                                F::SoloSel_ClipGoto(c, 2i32, 2i32, sym::menu_l_b__str62);
                                F::SoloSel_ClipGoto(c, 2i32, 5i32, sym::menu_l_b__str63);
                                { let v_ = 5i32; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                            } else {
                                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                    { let a0_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_l_b__str64, ((((1i32) as i64 as u64)) as u8)) };
                                    F::SoloSel_ClipGoto(c, 2i32, 2i32, sym::menu_l_b__str65);
                                    if (((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(104); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
                                        F::SoloSel_ClipGoto(c, 1i32, 1i32, sym::menu_l_b__str66);
                                        { let v_ = 1i32; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                    } else {
                                        F::SoloSel_ClipGoto(c, 1i32, 0i32, sym::menu_l_b__str67);
                                        { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                    }
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                                }
                            }
                        }
                    }
                }
                break 's1;
            }
            3i32 => {
                {
                    l_item_2 = F::ItemPanel_Input(c, 0i32, 0i32);
                    if (((((l_item_2) > (0i32)) as i32)) != 0) {
                        F::ItemHelp_Open(c);
                        { let v_ = (l_item_2).wrapping_sub(1i32); let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(416); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 4i32; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                    } else {
                        if (((((l_item_2) < (0i32)) as i32)) != 0) {
                            F::ItemPanel_Hide(c, 0i32);
                            F::SoloSel_ClipGoto(c, 2i32, 2i32, sym::menu_l_b__str68);
                            { let v_ = 2i32; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                        }
                    }
                    break 's1;
                }
            }
            4i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1536i32) as u32)))) != 0) {
                    F::ItemHelp_Close(c);
                    { let v_ = 3i32; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                }
                break 's1;
            }
            5i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) {
                    F::SoloSel_ClipGoto(c, 2i32, 5i32, sym::menu_l_b__str69);
                    { let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(24); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(24); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                        { let v_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6504); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                    }
                    F::SoloSel_ClipGoto(c, 2i32, 5i32, sym::menu_l_b__str70);
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) {
                        F::SoloSel_ClipGoto(c, 2i32, 5i32, sym::menu_l_b__str71);
                        { let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(24); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(24); (c.m.r32(a_) as i32) }) >= ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6504); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                            { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        }
                        F::SoloSel_ClipGoto(c, 2i32, 5i32, sym::menu_l_b__str72);
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                            { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(116); c.m.w32(a_, (v_) as u32); };
                            { let a0_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_l_b__str73, ((((1i32) as i64 as u64)) as u8)) };
                            { let a0_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(36)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_l_b__str74, ((((1i32) as i64 as u64)) as u8)) };
                            F::SoloSel_ClipGoto(c, 2i32, 5i32, sym::menu_l_b__str75);
                            F::SoloSel_ClipGoto(c, 2i32, 2i32, sym::menu_l_b__str76);
                            if (((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(104); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
                                { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(48)).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(8); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(20); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                    { let s_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(6144)).wrapping_add((((({ let a_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }; (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add((((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(20); (c.m.r32(a_) as i32) }).wrapping_sub(1i32)) as u32)).wrapping_mul(16)); let d_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(32); rt::memcpy(c, d_, s_, 16); };
                                } else {
                                    { let _ = { let a0_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(32); F::memset(c, a0_, 0i32, 16u32) }; };
                                }
                                F::SoloSel_ClipGoto(c, 1i32, 1i32, sym::menu_l_b__str77);
                            } else {
                                { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(48)).wrapping_add(((({ let a_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }; (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(20); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                    { let s_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(6144)).wrapping_add((((({ let a_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }; (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add((((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(20); (c.m.r32(a_) as i32) }).wrapping_sub(1i32)) as u32)).wrapping_mul(16)); let d_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(32); rt::memcpy(c, d_, s_, 16); };
                                } else {
                                    { let _ = { let a0_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(32); F::memset(c, a0_, 0i32, 16u32) }; };
                                }
                                F::SoloSel_ClipGoto(c, 1i32, 0i32, sym::menu_l_b__str78);
                            }
                            { let v_ = 1i32; let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6512); c.m.w32(a_, (v_) as u32); };
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                { let a0_ = ((c.m.r32(sym::gSoloSel)).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_l_b__str79, ((((1i32) as i64 as u64)) as u8)) };
                                F::SoloSel_ClipGoto(c, 2i32, 5i32, sym::menu_l_b__str80);
                                F::SoloSel_ClipGoto(c, 2i32, 2i32, sym::menu_l_b__str81);
                                { let v_ = 2i32; let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                            }
                        }
                    }
                }
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SoloSel_Input", &PROBE_N, &[], true); }
    r_
}

pub fn SoloSel_UpdateEnd(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SoloSel_UpdateEnd", &PROBE_N, &[], false); }
    let mut l_step_1: i32 = 0;
    let r_ = 'f: {
    l_step_1 = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6512); (c.m.r32(a_) as i32) };
    if (((((l_step_1) == (0i32)) as i32)) != 0) {
        break 'f;
    }
    's1: {
        let sv2: i32 = l_step_1;
        match sv2 {
            1i32 => {
                if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); (c.m.r32(a_) as i32) }) == (21i32)) as i32)) != 0) {
                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6516); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 23i32; let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(400); c.m.w32(a_, (v_) as u32); };
                    if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }).wrapping_add(108); (c.m.r32(a_) as i32) }) == (102i32)) as i32)) != 0) {
                        { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(400); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    }
                } else {
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6516); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 55i32; let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(400); c.m.w32(a_, (v_) as u32); };
                }
                { let a0_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(32); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(400); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34661i32, a2_) };
                { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6512); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6512); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6512); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                }
                break 's1;
            }
            3i32 => {
                { let r_ = 8i32; let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(396); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = 16i32; let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(396); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 15i32; let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(412); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6512); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SoloSel_UpdateEnd", &PROBE_N, &[], true); }
    r_
}

pub fn SoloSel_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SoloSel_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = 1i32; c.m.w32(fp, (v_) as u32); }
        F::SoloSel_Init(c, p_section_1);
        F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
        'l1: while ((1i32) != 0) {
            F::Gfx_BeginFrame(c);
            F::SoloSel_UpdateImage(c);
            F::Pad_Update(c);
            F::Snd_Update(c);
            F::ColorFade_Update(c);
            if ((((!(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0)) as i32)) != 0) {
                F::SoloSel_Update(c);
                F::SoloSel_UpdateEnd(c);
            }
            F::SoloSel_Draw(c);
            F::Font_FlushAll(c);
            F::ColorFade_Draw(c);
            F::Gfx_EndFrame(c, 1i32);
            F::Dma_Flush(c);
            F::File_Stub264D90(c);
            if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
                continue 'l1;
            }
            if ((F::ColorFade_IsInDone(c)) != 0) {
                if ((((((((!(((({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(396); (c.m.r32(a_) as i32) }) & (128i32))) != 0)) as i32)) != 0) && (((({ let a_ = (((c.m.r32(sym::gSoloSel)).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
                    { let r_ = 128i32; let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(396); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); (c.m.r32(a_) as i32) }) == (21i32)) as i32)) != 0) {
                        { let v_ = 1i32; let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6516); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 22i32; let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(400); c.m.w32(a_, (v_) as u32); };
                        { let a0_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(32); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(400); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34661i32, a2_) };
                    } else {
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6516); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 54i32; let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(400); c.m.w32(a_, (v_) as u32); };
                        { let a0_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(32); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(400); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34661i32, a2_) };
                    }
                }
            }
            if ((F::ColorFade_IsFadingOut(c)) != 0) {
                F::Voice_FadeOutStep(c, 0i32);
                F::Bgm_FadeOutStep(c);
                continue 'l1;
            }
            if ((F::ColorFade_IsOutDone(c)) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(404); (c.m.r32(a_) as i32) }) != (4i32)) as i32)) != 0) {
                    continue 'l1;
                }
                break 'l1;
            }
            if (((({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(396); (c.m.r32(a_) as i32) }) & (16i32))) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(412); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) == ((1i32).wrapping_neg())) as i32)) != 0) {
                    F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                    { let s_ = { let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(540); c.m.r32(a_) }; let d_ = (c.m.r32(sym::gProgress)).wrapping_add(1088); rt::memcpy(c, d_, s_, 48); };
                }
            } else {
                if ((((({ let a_ = (c.m.r32(sym::gSoloSel)).wrapping_add(6512); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::SoloSel_Input(c, fp);
                }
            }
        }
        F::SoloSel_Term(c);
        F::Dma_ResetBuffers(c);
        break 'f (c.m.r32(fp) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SoloSel_Run", &PROBE_N, &[], true); }
    r_
}
