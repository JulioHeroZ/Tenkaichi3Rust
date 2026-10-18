#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EntrySel_SwapRowTex(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EntrySel_SwapRowTex", &PROBE_N, &[], false); }
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
                    l_chara_3 = { let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(972); c.m.r32(a_) }).wrapping_add(((((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)).wrapping_add(l_i_2)) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) };
                    { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(384)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(412)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = l_chara_3; let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(384)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_res_1 = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((((l_chara_3).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
                    { let v_ = { let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(336)).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(336)).wrapping_add(((((18i32).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = (c.m.r32((l_res_1).wrapping_add(16))).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(336)).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EntrySel_SwapRowTex", &PROBE_N, &[], true); }
    r_
}

pub fn EntrySel_SetRowTex(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EntrySel_SetRowTex", &PROBE_N, &[], false); }
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
                    if (((((l_i_2) < ({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(972); c.m.r32(a_) }).wrapping_add(((((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)).wrapping_add({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(36))).wrapping_add(4); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        l_chara_3 = { let a_ = ((({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(972); c.m.r32(a_) }).wrapping_add(((((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)).wrapping_add({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(36))).wrapping_add(8)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                    } else {
                        l_chara_3 = 164i32;
                    }
                    { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(384)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(412)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = l_chara_3; let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(384)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_res_1 = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((((l_chara_3).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
                    { let v_ = { let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(336)).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(336)).wrapping_add(((((18i32).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = (c.m.r32((l_res_1).wrapping_add(16))).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(336)).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EntrySel_SetRowTex", &PROBE_N, &[], true); }
    r_
}

pub fn EntrySel_SetMemberTex(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EntrySel_SetMemberTex", &PROBE_N, &[], false); }
    let mut l_res_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (8i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(216)).wrapping_add(((((10i32).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    {
        l_i_2 = 0i32;
        'l3: loop {
            if !(((((l_i_2) < ({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l3; }
            'c4: {
                l_res_1 = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add((((l_i_2) as u32)).wrapping_mul(48))).wrapping_add(28); (c.m.r32(a_) as i32) }).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
                { let v_ = (c.m.r32((l_res_1).wrapping_add(16))).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(216)).wrapping_add(((((10i32).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EntrySel_SetMemberTex", &PROBE_N, &[], true); }
    r_
}

pub fn EntrySel_UpdateImage(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EntrySel_UpdateImage", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        's1: {
            let sv1: i32 = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(488); (c.m.r32(a_) as i32) };
            match sv1 {
                5i32 => {
                    { let v_ = 6i32; let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(488); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                6i32 => {
                    if (((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(440); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                        { let r_ = 1i32; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(440); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(488); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                1i32 => {
                    F::File_CancelRequests(c);
                    { let _ = { let a0_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(444); (c.m.r32(a_) as i32) }).wrapping_add(761i32); let a1_ = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(8); c.m.r32(a_) }; let _ = 92160i32; F::File_Request(c, a0_, a1_) }; };
                    { let v_ = 2i32; let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(488); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                2i32 => {
                    if ((F::File_UpdateRequests(c)) != 0) {
                        { let v_ = 3i32; let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(488); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                3i32 => {
                    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(8); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(12); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, a1_, ((0i32) as u32)) }; };
                    { let v_ = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(12); c.m.r32(a_) }; let a_ = fp; c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(216)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let r_ = 2i32; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(440); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 4i32; let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(488); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                4i32 => {
                    if (((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(440); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(216)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        { let v_ = 6i32; let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(488); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EntrySel_UpdateImage", &PROBE_N, &[], true); }
    r_
}

pub fn EntrySel_ChangeImage(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EntrySel_ChangeImage", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(440); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
        { let r_ = 2i32; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(440); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        { let v_ = 5i32; let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(488); c.m.w32(a_, (v_) as u32); };
    }
    { let r_ = 1i32; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(440); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EntrySel_ChangeImage", &PROBE_N, &[], true); }
    r_
}

pub fn EntrySel_ClipGoto(c: &mut Ctx, mut p_movie_1: i32, mut p_kind_2: i32, mut p_label_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EntrySel_ClipGoto", &PROBE_N, &[(p_movie_1) as u32, (p_kind_2) as u32, (p_label_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_flash_4: u32 = 0;
    let r_ = 'f: {
        l_flash_4 = ((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((p_movie_1) as u32)).wrapping_mul(44));
        's1: {
            let sv5: i32 = p_kind_2;
            match sv5 {
                0i32 => {
                    { let _ = { let a2_ = { let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }; F::sprintf(c, fp.wrapping_add(8), sym::menu_i_d__str0, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                1i32 => {
                    { let _ = { let a2_ = { let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(8); (c.m.r32(a_) as i32) }; F::sprintf(c, fp.wrapping_add(8), sym::menu_i_d__str1, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                2i32 => {
                    { let _ = { let a2_ = ({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_i_d__str2, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                5i32 => {
                    { let _ = { let a2_ = ({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(24); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_i_d__str3, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                7i32 => {
                    { let _ = { let a2_ = { let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }; F::sprintf(c, fp.wrapping_add(8), sym::menu_i_d__str4, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                3i32 => {
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
    if rt::probing() { crate::host::harness::probe(c, "EntrySel_ClipGoto", &PROBE_N, &[], true); }
    r_
}

pub fn EntrySel_Init(c: &mut Ctx, mut p_section_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EntrySel_Init", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_movie_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_cols_4: i32 = 0;
    let mut l_cols2_5: i32 = 0;
    let mut l_cols_6: i32 = 0;
    let mut l_n_7: i32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = F::Heap_Alloc(c, 7240i32, ((32i32) as u32), 0i32, 2i32); let a_ = sym::gEntrySel; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gEntrySel); F::memset(c, a0_, 0i32, ((7240i32) as u32)) }; };
        { let v_ = (c.m.r32(sym::gMenuArc4)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc4)).wrapping_add((((p_section_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = c.m.r32(sym::gEntrySel); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gEntrySel); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = ({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(124)).wrapping_add(8); (c.m.r32(a_) as i32) }).wrapping_add(969i32); F::File_LoadSync(c, a0_, ((0i32) as u32), 0i32) }; let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(36); c.m.w32(a_, v_); };
        { let a0_ = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(36); c.m.r32(a_) }; let a1_ = (({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(124)).wrapping_add(8); (c.m.r32(a_) as i32) }) as u32); F::TourBg_Init(c, a0_, a1_, ((0i32) as u32)) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(216)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(216)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(216)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(216)).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(336)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(216)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(216)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(216)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gEntrySel)).wrapping_add(216); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(336)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(336)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(336)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(336)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(336)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(336)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(336)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(336)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(336)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(216)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(292)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gEntrySel)).wrapping_add(336); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(292)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(292)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(292)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(292)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(292)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(292)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(292)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(292)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(292)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(292)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gEntrySel)).wrapping_add(292); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        l_movie_2 = 0i32;
        's1: {
            let sv8: i32 = { let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(124)).wrapping_add(8); (c.m.r32(a_) as i32) };
            match sv8 {
                0i32 => {
                    { let v_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(436)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(436)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    l_movie_2 = 18i32;
                    break 's1;
                }
                1i32 => {
                    { let v_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(436)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(436)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(436)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    l_movie_2 = 20i32;
                    break 's1;
                }
                2i32 => {
                    { let v_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(436)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(436)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(436)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    l_movie_2 = 22i32;
                    break 's1;
                }
                3i32 => {
                    { let v_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(436)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(436)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    l_movie_2 = 24i32;
                    break 's1;
                }
                4i32 => {
                    { let v_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(436)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(436)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(436)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(436)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(436)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(436)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    l_movie_2 = 25i32;
                    break 's1;
                }
                _ => {
                }
            }
        }
        { let a0_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((3i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((l_movie_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gEntrySel)).wrapping_add(436); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((3i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let a0_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((3i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_i_d__str5, ((((1i32) as i64 as u64)) as u8)) };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((39i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::ItemPanel_Init(c, a0_, 0i32) };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((35i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::ItemHelp_Init(c, a0_) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((33i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(28); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((36i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(32); c.m.w32(a_, v_); };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((34i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(28); c.m.r32(a_) }; let _ = 0i32; F::MsgWin_Init(c, a0_, a1_, 0i32) };
        F::MsgWin_Open(c);
        { let v_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(7236); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((30i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(20); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((31i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(24); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((32i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(16); c.m.w32(a_, v_); };
        {
            l_i_3 = 0i32;
            'l2: loop {
                if !(((((l_i_3) < (165i32)) as i32)) != 0) { break 'l2; }
                'c3: {
                    { let v_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((((l_i_3).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        { let v_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32))).wrapping_add(16); let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(972); c.m.w32(a_, v_); };
        { let v_ = (({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) as i32); let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(968); c.m.w32(a_, (v_) as u32); };
        { let a0_ = (c.m.r32(sym::gEntrySel)).wrapping_add(976); let a1_ = (c.m.r32(sym::gEntrySel)).wrapping_add(980); let a2_ = (c.m.r32(sym::gEntrySel)).wrapping_add(968); let a3_ = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(972); c.m.r32(a_) }; F::ChrGrid_Build(c, a0_, a1_, a2_, a3_, ((0i32) as u32), ((0i32) as u32)) };
        { let v_ = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(976); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(968); c.m.w32(a_, (v_) as u32); };
        { let v_ = (c.m.r32(sym::gEntrySel)).wrapping_add(980); let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(972); c.m.w32(a_, v_); };
        {
            l_cols_4 = 7i32;
            l_cols2_5 = 7i32;
            { let v_ = rt::div_i32({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(968); (c.m.r32(a_) as i32) }, l_cols_4); let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6920); c.m.w32(a_, (v_) as u32); };
            if ((rt::rem_i32({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(968); (c.m.r32(a_) as i32) }, l_cols2_5)) != 0) {
                { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
        }
        { let v_ = F::Heap_Alloc(c, 92160i32, ((64i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = F::Heap_Alloc(c, 133120i32, ((32i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32(sym::gEntrySel)).wrapping_add(504); let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.w32(a_, v_); };
        { let s_ = (c.m.r32(sym::gProgress)).wrapping_add(1088); let d_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(48)); rt::memcpy(c, d_, s_, 48); };
        { let v_ = 4i32; let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(488); c.m.w32(a_, (v_) as u32); };
        if ((((!(({ let a0_ = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(972); c.m.r32(a_) }; let a1_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)); F::ChrGrid_IsSelectable(c, a0_, a1_) }) != 0)) as i32)) != 0) {
            { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }; F::memset(c, a0_, 0i32, 48u32) }; };
            { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(48)); c.m.w32(a_, (v_) as u32); };
            { let s_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(48)); let d_ = (c.m.r32(sym::gProgress)).wrapping_add(1088); rt::memcpy(c, d_, s_, 48); };
        }
        's4: {
            let sv9: i32 = { let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(124)).wrapping_add(8); (c.m.r32(a_) as i32) };
            match sv9 {
                0i32 | 1i32 | 2i32 | 3i32 => {
                    break 's4;
                }
                4i32 => {
                    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }; F::memset(c, a0_, 0i32, 48u32) }; };
                    {
                        l_i_3 = 0i32;
                        'l5: loop {
                            if !(((((l_i_3) < ({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(124)).wrapping_add(16); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l5; }
                            'c6: {
                                l_cols_6 = 7i32;
                                'l7: while ((1i32) != 0) {
                                    l_n_7 = (({ let a0_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(968); (c.m.r32(a_) as i32) }) as u32); F::Rand_Range(c, a0_) }) as i32);
                                    if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(972); c.m.r32(a_) }).wrapping_add((((l_n_7) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) }) <= (160i32)) as i32)) != 0) {
                                        { let v_ = rt::rem_i32(l_n_7, l_cols_6); let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add((((l_i_3) as u32)).wrapping_mul(48)); c.m.w32(a_, (v_) as u32); };
                                        { let v_ = rt::div_i32(l_n_7, l_cols_6); let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add((((l_i_3) as u32)).wrapping_mul(48))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                        { let v_ = ((F::Rand_Range(c, ((4i32) as u32))) as i32); let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add((((l_i_3) as u32)).wrapping_mul(48))).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                                        if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(972); c.m.r32(a_) }).wrapping_add((((l_n_7) as u32)).wrapping_mul(36))).wrapping_add(4); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                            { let v_ = (({ let a0_ = (({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(972); c.m.r32(a_) }).wrapping_add((((l_n_7) as u32)).wrapping_mul(36))).wrapping_add(4); (c.m.r32(a_) as i32) }) as u32); F::Rand_Range(c, a0_) }) as i32); let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add((((l_i_3) as u32)).wrapping_mul(48))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                        }
                                        { let v_ = (({ let a0_ = (({ let a0_ = { let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(972); c.m.r32(a_) }).wrapping_add((((l_n_7) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) }; F::ChrTbl_WrapCostume(c, a0_, ((0i32) as u32)) }) as u32); F::Rand_Range(c, a0_) }) as i32); let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add((((l_i_3) as u32)).wrapping_mul(48))).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                                        break 'l7;
                                    }
                                }
                            }
                            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                        }
                    }
                    break 's4;
                }
                _ => {
                }
            }
        }
        F::EntrySel_SwapRowTex(c);
        { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(384)).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(444); c.m.w32(a_, (v_) as u32); };
        { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(484); c.m.w32(a_, (v_) as u32); };
        {
            l_i_3 = 0i32;
            'l8: loop {
                if !(((((l_i_3) < (2i32)) as i32)) != 0) { break 'l8; }
                'c9: {
                    { let v_ = ((F::Rand_Range(c, ((32i32) as u32))) as i32); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(6940)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        { let _ = { let a0_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(444); (c.m.r32(a_) as i32) }).wrapping_add(761i32); let a1_ = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(8); c.m.r32(a_) }; F::File_LoadSync(c, a0_, a1_, 92160i32) }; };
        { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(8); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(12); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, a1_, ((0i32) as u32)) }; };
        { let v_ = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(12); c.m.r32(a_) }; let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(216)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let r_ = 2i32; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(440); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let _ = { let a0_ = ((c.m.r32(sym::gProgress)).wrapping_add(124)).wrapping_add(28); F::memset(c, a0_, 0i32, ((680i32) as u32)) }; };
        { let a0_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(6956)).wrapping_add((((0i32) as u32)).wrapping_mul(140)); let a1_ = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(20); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, ((1i32) as u32)) };
        { let a0_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(6956)).wrapping_add((((0i32) as u32)).wrapping_mul(140)); F::TextBox_SetUnk80(c, a0_, 1i32) };
        { let a0_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(6956)).wrapping_add((((1i32) as u32)).wrapping_mul(140)); let a1_ = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(24); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, ((3i32) as u32)) };
        { let a0_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(6956)).wrapping_add((((1i32) as u32)).wrapping_mul(140)); F::TextBox_SetUnk80(c, a0_, 1i32) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EntrySel_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EntrySel_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EntrySel_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    F::ItemHelp_Term(c);
    F::MsgWin_Term(c);
    F::ItemPanel_Term(c, 0i32);
    F::TourBg_Term(c);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Destroy(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(8); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(8); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(8); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(12); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(12); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(12); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(36); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(36); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(36); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gEntrySel)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gEntrySel); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gEntrySel; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EntrySel_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EntrySel_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EntrySel_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_i_1: i32 = 0;
    let mut l_flash_2: u32 = 0;
    let r_ = 'f: {
        F::TourBg_Draw(c);
        l_flash_2 = ((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        if (((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(440); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
            { let r_ = F32(0x3d999999); let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6932); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::fge({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6932); F32(c.m.r32(a_)) }, F32(0x3f800000)) as i32)) != 0) {
                { let v_ = F32(0x3f800000); let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6932); c.m.w32(a_, (v_).0); };
            }
        } else {
            { let v_ = F32(0x00000000); let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6932); c.m.w32(a_, (v_).0); };
        }
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_i_d__str6, fp);
        { let a2_ = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6932); F32(c.m.r32(a_)) }; F::Flash_ClipSetAlpha(c, l_flash_2, fp, a2_) };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_i_d__str7, fp);
        { let a4_ = { let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(444); (c.m.r32(a_) as i32) }; let a5_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(6956)).wrapping_add((((0i32) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_2, fp, 0i32, 0i32, a4_, a5_) };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_i_d__str8, fp);
        { let a4_ = { let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(444); (c.m.r32(a_) as i32) }; let a5_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(6956)).wrapping_add((((1i32) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_2, fp, 0i32, 0i32, a4_, a5_) };
        {
            l_i_1 = 0i32;
            'l1: loop {
                if !(((((l_i_1) < (8i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_i_d__str9, &[((l_i_1) as i64 as u64)]); };
                    if (((((((((l_i_1) >= ({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(124)).wrapping_add(16); (c.m.r32(a_) as i32) })) as i32)) != 0) || (((((l_i_1) < ({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
                        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(28), fp);
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    } else {
                        { let v_ = (rt::rem_i32(l_i_1, 2i32)).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = (rt::div_i32(l_i_1, 2i32)).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(28), fp);
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                        F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_i_d__str10, fp);
                        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                        F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_i_d__str11, fp);
                        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                    }
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_i_d__str12, &[((l_i_1) as i64 as u64)]); };
                    if (((((l_i_1) < ({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        { let v_ = (rt::rem_i32(l_i_1, 2i32)).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = (rt::div_i32(l_i_1, 2i32)).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(28), fp);
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                    } else {
                        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(28), fp);
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    }
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        l_flash_2 = ((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(44));
        { let v_ = 32i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_i_d__str13, fp);
        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
        {
            l_i_1 = 0i32;
            'l3: loop {
                if !(((((l_i_1) < (4i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (l_i_1).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_i_d__str14, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(28), fp);
                    { let a2_ = (if (((((l_i_1) < ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6924); (c.m.r32(a_) as i32) })) as i32)) != 0) { F32(0x3f800000) } else { F32(0x3e999999) }); F::Flash_ClipSetColor(c, l_flash_2, fp, a2_) };
                    F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_i_d__str15, fp);
                    F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                    F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_i_d__str16, fp);
                    F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                    if (((((l_i_1) == (0i32)) as i32)) != 0) {
                        F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_i_d__str17, fp);
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    } else {
                        F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_i_d__str18, fp);
                        { let a3_ = ((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(972); c.m.r32(a_) }).wrapping_add(((((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)).wrapping_add({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) }) != (161i32)) as i32); { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((a3_) as i64 as u64)) as u8)); } };
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
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_i_d__str19, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(28), fp);
                    if (((((l_i_1) < ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6928); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                        's7: {
                            let sv3: i32 = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6928); (c.m.r32(a_) as i32) };
                            match sv3 {
                                2i32 => {
                                    F::Flash_ClipSetOffset(c, l_flash_2, fp, 45i32, 0i32);
                                    break 's7;
                                }
                                3i32 => {
                                    F::Flash_ClipSetOffset(c, l_flash_2, fp, 22i32, 0i32);
                                    break 's7;
                                }
                                _ => {
                                }
                            }
                        }
                        F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_i_d__str20, fp);
                        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                        F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_i_d__str21, fp);
                        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                    } else {
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    }
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        l_flash_2 = ((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(44));
        if (((((({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(124)).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_sub(1i32)) < ({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) })) as i32)) != 0) {
            { let v_ = (rt::rem_i32(({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }).wrapping_sub(1i32), 2i32)).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = (rt::div_i32(({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }).wrapping_sub(1i32), 2i32)).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = (rt::rem_i32({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }, 2i32)).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = (rt::div_i32({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }, 2i32)).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        }
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_i_d__str22, fp);
        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
        if ((((({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6920); (c.m.r32(a_) as i32) }) >= (2i32)) as i32)) != 0) {
            {
                l_i_1 = 0i32;
                'l8: loop {
                    if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l8; }
                    'c9: {
                        { let v_ = (l_i_1).wrapping_mul(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        F::Flash_FindLabel(c, l_flash_2, (if (((((l_i_1) != (0i32)) as i32)) != 0) { sym::menu_i_d__str23 } else { sym::menu_i_d__str24 }), (if (((((l_i_1) != (0i32)) as i32)) != 0) { sym::menu_i_d__str25 } else { sym::menu_i_d__str26 }), fp);
                        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                        if (((((((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(440); (c.m.r32(a_) as i32) }) & (64i32))) != 0) || ((((({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(124)).wrapping_add(8); (c.m.r32(a_) as i32) }) == (4i32)) as i32)) != 0)) as i32)) != 0) {
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                        } else {
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
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
                        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), (if (((((l_i_1) != (0i32)) as i32)) != 0) { sym::menu_i_d__str27 } else { sym::menu_i_d__str28 }), fp);
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    }
                    { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
                }
            }
        }
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_i_d__str29, fp);
        if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(456); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 258i32, ((((1i32) as i64 as u64)) as u8)); };
        } else {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 258i32, ((((0i32) as i64 as u64)) as u8)); };
        }
        {
            l_i_1 = 0i32;
            'l12: loop {
                if !(((((l_i_1) < (14i32)) as i32)) != 0) { break 'l12; }
                'c13: {
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_i_d__str30, &[((l_i_1) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(28), fp);
                    { let a3_ = (((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(456); (c.m.r32(a_) as i32) }) as u8)) as i32); { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 128i32, ((((a3_) as i64 as u64)) as u8)); } };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        l_flash_2 = ((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((3i32) as u32)).wrapping_mul(44));
        's14: {
            let sv4: i32 = { let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(124)).wrapping_add(8); (c.m.r32(a_) as i32) };
            match sv4 {
                0i32 => {
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_i_d__str31, fp);
                    { let a2_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(6948)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F::FlashAnim_Talk(c, l_flash_2, fp, a2_, 0i32) };
                    break 's14;
                }
                1i32 => {
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_i_d__str32, fp);
                    { let a2_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(6940)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F::FlashAnim_Blink(c, l_flash_2, fp, a2_, 0i32) };
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_i_d__str33, fp);
                    { let a2_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(6948)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F::FlashAnim_Talk(c, l_flash_2, fp, a2_, 0i32) };
                    break 's14;
                }
                2i32 => {
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_i_d__str34, fp);
                    { let a2_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(6940)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F::FlashAnim_Blink(c, l_flash_2, fp, a2_, 0i32) };
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_i_d__str35, fp);
                    { let a2_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(6948)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F::FlashAnim_Talk(c, l_flash_2, fp, a2_, 0i32) };
                    break 's14;
                }
                3i32 => {
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_i_d__str36, fp);
                    { let a2_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(6948)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F::FlashAnim_Talk(c, l_flash_2, fp, a2_, 0i32) };
                    break 's14;
                }
                4i32 => {
                    {
                        l_i_1 = 0i32;
                        'l15: loop {
                            if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l15; }
                            'c16: {
                                F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), (if (((((l_i_1) != (0i32)) as i32)) != 0) { sym::menu_i_d__str37 } else { sym::menu_i_d__str38 }), fp);
                                { let a2_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(6940)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); F::FlashAnim_Blink(c, l_flash_2, fp, a2_, 0i32) };
                                F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), (if (((((l_i_1) != (0i32)) as i32)) != 0) { sym::menu_i_d__str39 } else { sym::menu_i_d__str40 }), fp);
                                if (((((l_i_1) == ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6936); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                    { let a2_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(6948)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); F::FlashAnim_Talk(c, l_flash_2, fp, a2_, 0i32) };
                                } else {
                                    F::FlashAnim_ShowNext2(c, l_flash_2, fp, 0i32);
                                }
                            }
                            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
                        }
                    }
                    break 's14;
                }
                _ => {
                }
            }
        }
        { let a0_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
        F::Font_FlushAll(c);
        { let a0_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
        { let a0_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
        { let a2_ = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(484); (c.m.r32(a_) as i32) }; F::MsgWin_Draw(c, 0i32, 0i32, a2_) };
        { let a0_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((3i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
        F::ItemPanel_Draw(c, 0i32);
        { let a0_ = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(496); (c.m.r32(a_) as i32) }; F::ItemHelp_Draw(c, a0_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EntrySel_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EntrySel_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EntrySel_Update", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Advance(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    F::ItemPanel_Update(c, 0i32);
    if ((((((((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(456); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) && (((({ let a_ = (((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((1i32) as u32)))) != 0)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(456); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EntrySel_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EntrySel_Input(c: &mut Ctx, mut p_result_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EntrySel_Input", &PROBE_N, &[(p_result_1) as u32], false); }
    let mut l_item_2: i32 = 0;
    let mut l_slot_3: i32 = 0;
    let mut l_cell_4: u32 = 0;
    let mut l_chara_5: i32 = 0;
    let mut l_id_6: i32 = 0;
    let r_ = 'f: {
    if ((((!(((({ let a_ = (((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f;
    }
    if ((((!(((({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(472); (c.m.r32(a_) as i32) }) & (4i32))) != 0)) as i32)) != 0) {
        F::EntrySel_ClipGoto(c, 1i32, 0i32, sym::menu_i_d__str41);
        F::EntrySel_ClipGoto(c, 0i32, 7i32, sym::menu_i_d__str42);
        { let r_ = 4i32; let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(472); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    's1: {
        let sv7: i32 = { let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(452); (c.m.r32(a_) as i32) };
        match sv7 {
            0i32 => {
                if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) && ((((({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(124)).wrapping_add(8); (c.m.r32(a_) as i32) }) != (4i32)) as i32)) != 0)) as i32)) != 0) {
                    F::EntrySel_ClipGoto(c, 1i32, 0i32, sym::menu_i_d__str43);
                    { let a0_ = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(972); c.m.r32(a_) }; let a1_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); let a2_ = { let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }; { let _ = F::ChrGrid_MoveLeft(c, a0_, a1_, a2_); } };
                    F::EntrySel_ClipGoto(c, 1i32, 0i32, sym::menu_i_d__str44);
                    { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(384)).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(444); c.m.w32(a_, (v_) as u32); };
                    F::EntrySel_ChangeImage(c);
                    { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 0i32); };
                } else {
                    if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) && ((((({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(124)).wrapping_add(8); (c.m.r32(a_) as i32) }) != (4i32)) as i32)) != 0)) as i32)) != 0) {
                        F::EntrySel_ClipGoto(c, 1i32, 0i32, sym::menu_i_d__str45);
                        { let a0_ = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(972); c.m.r32(a_) }; let a1_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); let a2_ = { let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }; { let _ = F::ChrGrid_MoveRight(c, a0_, a1_, a2_); } };
                        F::EntrySel_ClipGoto(c, 1i32, 0i32, sym::menu_i_d__str46);
                        { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(384)).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(444); c.m.w32(a_, (v_) as u32); };
                        F::EntrySel_ChangeImage(c);
                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 0i32); };
                    } else {
                        if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) && ((((({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(124)).wrapping_add(8); (c.m.r32(a_) as i32) }) != (4i32)) as i32)) != 0)) as i32)) != 0) {
                            if ((((({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6920); (c.m.r32(a_) as i32) }) >= (2i32)) as i32)) != 0) {
                                { let a0_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_i_d__str47, ((((1i32) as i64 as u64)) as u8)) };
                                { let v_ = 1i32; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(456); c.m.w32(a_, (v_) as u32); };
                                F::EntrySel_ClipGoto(c, 1i32, 0i32, sym::menu_i_d__str48);
                                { let a0_ = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(972); c.m.r32(a_) }; let a1_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); let a2_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); let a3_ = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6920); (c.m.r32(a_) as i32) }; F::ChrGrid_MoveUp(c, a0_, a1_, a2_, a3_) };
                                F::EntrySel_ClipGoto(c, 1i32, 0i32, sym::menu_i_d__str49);
                                F::EntrySel_SwapRowTex(c);
                                { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(384)).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(444); c.m.w32(a_, (v_) as u32); };
                                F::EntrySel_ChangeImage(c);
                                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 2i32); };
                            }
                        } else {
                            if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) && ((((({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(124)).wrapping_add(8); (c.m.r32(a_) as i32) }) != (4i32)) as i32)) != 0)) as i32)) != 0) {
                                if ((((({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6920); (c.m.r32(a_) as i32) }) >= (2i32)) as i32)) != 0) {
                                    { let a0_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_i_d__str50, ((((1i32) as i64 as u64)) as u8)) };
                                    { let v_ = 1i32; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(456); c.m.w32(a_, (v_) as u32); };
                                    F::EntrySel_ClipGoto(c, 1i32, 0i32, sym::menu_i_d__str51);
                                    { let a0_ = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(972); c.m.r32(a_) }; let a1_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); let a2_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); let a3_ = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6920); (c.m.r32(a_) as i32) }; F::ChrGrid_MoveDown(c, a0_, a1_, a2_, a3_) };
                                    F::EntrySel_ClipGoto(c, 1i32, 0i32, sym::menu_i_d__str52);
                                    F::EntrySel_SwapRowTex(c);
                                    { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(384)).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(444); c.m.w32(a_, (v_) as u32); };
                                    F::EntrySel_ChangeImage(c);
                                    { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 1i32); };
                                }
                            } else {
                                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                                    if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(972); c.m.r32(a_) }).wrapping_add(((((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)).wrapping_add({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) }) == (161i32)) as i32)) != 0) {
                                        { let a0_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_i_d__str53, ((((1i32) as i64 as u64)) as u8)) };
                                        F::EntrySel_ClipGoto(c, 1i32, 0i32, sym::menu_i_d__str54);
                                        F::EntrySel_ClipGoto(c, 2i32, 2i32, sym::menu_i_d__str55);
                                        { let v_ = (if (((({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(472); (c.m.r32(a_) as i32) }) & (16i32))) != 0) { 1i32 } else { 4i32 }); let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6924); c.m.w32(a_, (v_) as u32); };
                                        if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }) >= ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6924); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                            { let v_ = 0i32; let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                                        }
                                        { let v_ = 2i32; let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6928); c.m.w32(a_, (v_) as u32); };
                                        if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(24); (c.m.r32(a_) as i32) }) >= ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6928); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                            { let v_ = 0i32; let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                                        }
                                        { let v_ = 2i32; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(452); c.m.w32(a_, (v_) as u32); };
                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                    } else {
                                        if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(972); c.m.r32(a_) }).wrapping_add(((((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)).wrapping_add({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(36))).wrapping_add(4); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                            { let r_ = 64i32; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(440); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                            if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(8); (c.m.r32(a_) as i32) }) >= ({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(972); c.m.r32(a_) }).wrapping_add(((((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)).wrapping_add({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(36))).wrapping_add(4); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                                { let v_ = 0i32; let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                            }
                                            { let a0_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_i_d__str56, ((((1i32) as i64 as u64)) as u8)) };
                                            { let v_ = 1i32; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(456); c.m.w32(a_, (v_) as u32); };
                                            F::EntrySel_ClipGoto(c, 1i32, 0i32, sym::menu_i_d__str57);
                                            F::EntrySel_ClipGoto(c, 1i32, 1i32, sym::menu_i_d__str58);
                                            F::EntrySel_SetRowTex(c);
                                            { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(384)).wrapping_add(((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(8); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(444); c.m.w32(a_, (v_) as u32); };
                                            F::EntrySel_ChangeImage(c);
                                            { let v_ = 1i32; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(452); c.m.w32(a_, (v_) as u32); };
                                            { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 41i32); };
                                        } else {
                                            { let a0_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_i_d__str59, ((((1i32) as i64 as u64)) as u8)) };
                                            F::EntrySel_ClipGoto(c, 1i32, 0i32, sym::menu_i_d__str60);
                                            F::EntrySel_ClipGoto(c, 2i32, 2i32, sym::menu_i_d__str61);
                                            { let v_ = (if (((({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(472); (c.m.r32(a_) as i32) }) & (16i32))) != 0) { 1i32 } else { 4i32 }); let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6924); c.m.w32(a_, (v_) as u32); };
                                            if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }) >= ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6924); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                                { let v_ = 0i32; let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                                            }
                                            { let v_ = { let a0_ = { let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(444); (c.m.r32(a_) as i32) }; let a1_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(24); F::ChrTbl_WrapCostume(c, a0_, a1_) }; let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6928); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = 2i32; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(452); c.m.w32(a_, (v_) as u32); };
                                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                        }
                                    }
                                } else {
                                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                        if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                            F::EntrySel_ClipGoto(c, 1i32, 0i32, sym::menu_i_d__str62);
                                            F::EntrySel_ClipGoto(c, 0i32, 7i32, sym::menu_i_d__str63);
                                            { let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                                            F::EntrySel_SetMemberTex(c);
                                            F::EntrySel_ClipGoto(c, 1i32, 0i32, sym::menu_i_d__str64);
                                            F::EntrySel_ClipGoto(c, 0i32, 7i32, sym::menu_i_d__str65);
                                            F::EntrySel_SwapRowTex(c);
                                            { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(384)).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(444); c.m.w32(a_, (v_) as u32); };
                                            F::EntrySel_ChangeImage(c);
                                        } else {
                                            F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                                            { let v_ = 0i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
                                        }
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
                if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) && ((((({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(124)).wrapping_add(8); (c.m.r32(a_) as i32) }) != (4i32)) as i32)) != 0)) as i32)) != 0) {
                    F::EntrySel_ClipGoto(c, 1i32, 1i32, sym::menu_i_d__str66);
                    { let a0_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(384); let a1_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(8); F::ChrGrid_PrevForm(c, a0_, a1_) };
                    F::EntrySel_ClipGoto(c, 1i32, 1i32, sym::menu_i_d__str67);
                    { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(384)).wrapping_add(((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(8); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(444); c.m.w32(a_, (v_) as u32); };
                    F::EntrySel_ChangeImage(c);
                    { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 0i32); };
                } else {
                    if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) && ((((({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(124)).wrapping_add(8); (c.m.r32(a_) as i32) }) != (4i32)) as i32)) != 0)) as i32)) != 0) {
                        F::EntrySel_ClipGoto(c, 1i32, 1i32, sym::menu_i_d__str68);
                        { let a0_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(384); let a1_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(8); F::ChrGrid_NextForm(c, a0_, a1_) };
                        F::EntrySel_ClipGoto(c, 1i32, 1i32, sym::menu_i_d__str69);
                        { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(384)).wrapping_add(((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(8); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(444); c.m.w32(a_, (v_) as u32); };
                        F::EntrySel_ChangeImage(c);
                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 0i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                            { let a0_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_i_d__str70, ((((1i32) as i64 as u64)) as u8)) };
                            F::EntrySel_ClipGoto(c, 1i32, 1i32, sym::menu_i_d__str71);
                            F::EntrySel_ClipGoto(c, 2i32, 2i32, sym::menu_i_d__str72);
                            { let v_ = (if (((({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(472); (c.m.r32(a_) as i32) }) & (16i32))) != 0) { 1i32 } else { 4i32 }); let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6924); c.m.w32(a_, (v_) as u32); };
                            if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }) >= ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6924); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                { let v_ = 0i32; let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                            }
                            { let v_ = { let a0_ = { let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(444); (c.m.r32(a_) as i32) }; let a1_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(24); F::ChrTbl_WrapCostume(c, a0_, a1_) }; let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6928); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 2i32; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(452); c.m.w32(a_, (v_) as u32); };
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                { let r_ = 64i32; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(440); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                                { let a0_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_i_d__str73, ((((1i32) as i64 as u64)) as u8)) };
                                { let v_ = 1i32; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(456); c.m.w32(a_, (v_) as u32); };
                                F::EntrySel_ClipGoto(c, 1i32, 1i32, sym::menu_i_d__str74);
                                F::EntrySel_ClipGoto(c, 1i32, 0i32, sym::menu_i_d__str75);
                                F::EntrySel_SwapRowTex(c);
                                { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(384)).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(444); c.m.w32(a_, (v_) as u32); };
                                F::EntrySel_ChangeImage(c);
                                { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(452); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 41i32); };
                            }
                        }
                    }
                }
                break 's1;
            }
            2i32 => {
                if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) && ((((({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(124)).wrapping_add(8); (c.m.r32(a_) as i32) }) != (4i32)) as i32)) != 0)) as i32)) != 0) {
                    if ((((({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6924); (c.m.r32(a_) as i32) }) >= (2i32)) as i32)) != 0) {
                        F::EntrySel_ClipGoto(c, 2i32, 2i32, sym::menu_i_d__str76);
                        { let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                            { let v_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6924); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                        }
                        F::EntrySel_ClipGoto(c, 2i32, 2i32, sym::menu_i_d__str77);
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    }
                } else {
                    if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) && ((((({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(124)).wrapping_add(8); (c.m.r32(a_) as i32) }) != (4i32)) as i32)) != 0)) as i32)) != 0) {
                        if ((((({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6924); (c.m.r32(a_) as i32) }) >= (2i32)) as i32)) != 0) {
                            F::EntrySel_ClipGoto(c, 2i32, 2i32, sym::menu_i_d__str78);
                            { let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }) >= ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6924); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                { let v_ = 0i32; let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                            }
                            F::EntrySel_ClipGoto(c, 2i32, 2i32, sym::menu_i_d__str79);
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                        }
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((2i32) as u32)))) != 0) {
                            if ((((((((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) && ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(972); c.m.r32(a_) }).wrapping_add(((((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)).wrapping_add({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) }) != (161i32)) as i32)) != 0)) as i32)) != 0) {
                                F::EntrySel_ClipGoto(c, 2i32, 2i32, sym::menu_i_d__str80);
                                { let a1_ = { let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(444); (c.m.r32(a_) as i32) }; let a2_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)); let a3_ = ({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); F::ItemPanel_SetChara(c, 0i32, a1_, a2_, a3_, 0i32) };
                                F::ItemPanel_Show(c, 0i32);
                                { let a0_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((3i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_i_d__str81, ((((1i32) as i64 as u64)) as u8)) };
                                F::MsgWin_Close(c);
                                { let v_ = 3i32; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(452); c.m.w32(a_, (v_) as u32); };
                            }
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                                { let a0_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_i_d__str82, ((((1i32) as i64 as u64)) as u8)) };
                                F::EntrySel_ClipGoto(c, 2i32, 2i32, sym::menu_i_d__str83);
                                F::EntrySel_ClipGoto(c, 2i32, 5i32, sym::menu_i_d__str84);
                                { let v_ = 5i32; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(452); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                            } else {
                                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                    { let a0_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_i_d__str85, ((((1i32) as i64 as u64)) as u8)) };
                                    F::EntrySel_ClipGoto(c, 2i32, 2i32, sym::menu_i_d__str86);
                                    if (((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(440); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
                                        F::EntrySel_ClipGoto(c, 1i32, 1i32, sym::menu_i_d__str87);
                                        { let v_ = 1i32; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(452); c.m.w32(a_, (v_) as u32); };
                                    } else {
                                        F::EntrySel_ClipGoto(c, 1i32, 0i32, sym::menu_i_d__str88);
                                        { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(452); c.m.w32(a_, (v_) as u32); };
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
                        { let v_ = (l_item_2).wrapping_sub(1i32); let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(496); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 4i32; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(452); c.m.w32(a_, (v_) as u32); };
                    } else {
                        if (((((l_item_2) < (0i32)) as i32)) != 0) {
                            F::ItemPanel_Hide(c, 0i32);
                            F::EntrySel_ClipGoto(c, 2i32, 2i32, sym::menu_i_d__str89);
                            { let a0_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((3i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_i_d__str90, ((((1i32) as i64 as u64)) as u8)) };
                            F::MsgWin_Open(c);
                            { let v_ = 2i32; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(452); c.m.w32(a_, (v_) as u32); };
                        }
                    }
                    break 's1;
                }
            }
            4i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1536i32) as u32)))) != 0) {
                    F::ItemHelp_Close(c);
                    { let v_ = 3i32; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(452); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                }
                break 's1;
            }
            5i32 => {
                if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) && ((((({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(124)).wrapping_add(8); (c.m.r32(a_) as i32) }) != (4i32)) as i32)) != 0)) as i32)) != 0) {
                    F::EntrySel_ClipGoto(c, 2i32, 5i32, sym::menu_i_d__str91);
                    { let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(24); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(24); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                        { let v_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6928); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                    }
                    F::EntrySel_ClipGoto(c, 2i32, 5i32, sym::menu_i_d__str92);
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                } else {
                    if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) && ((((({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(124)).wrapping_add(8); (c.m.r32(a_) as i32) }) != (4i32)) as i32)) != 0)) as i32)) != 0) {
                        F::EntrySel_ClipGoto(c, 2i32, 5i32, sym::menu_i_d__str93);
                        { let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(24); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(24); (c.m.r32(a_) as i32) }) >= ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6928); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                            { let v_ = 0i32; let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        }
                        F::EntrySel_ClipGoto(c, 2i32, 5i32, sym::menu_i_d__str94);
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                            { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(456); c.m.w32(a_, (v_) as u32); };
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                            { let a0_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_i_d__str95, ((((1i32) as i64 as u64)) as u8)) };
                            F::EntrySel_ClipGoto(c, 2i32, 5i32, sym::menu_i_d__str96);
                            F::EntrySel_ClipGoto(c, 0i32, 7i32, sym::menu_i_d__str97);
                            F::EntrySel_ClipGoto(c, 2i32, 2i32, sym::menu_i_d__str98);
                            if (((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(440); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
                                { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(384)).wrapping_add(((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(8); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(972); c.m.r32(a_) }).wrapping_add(((((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)).wrapping_add({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) }) == (163i32)) as i32)) != 0) {
                                    { let s_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11584)).wrapping_add((((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(12); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(28)); let d_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(32); rt::memcpy(c, d_, s_, 16); };
                                } else {
                                    if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                        { let s_ = (((c.m.r32(sym::gSaveData)).wrapping_add(6152)).wrapping_add(((((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)).wrapping_add({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(56))).wrapping_add((((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }).wrapping_sub(1i32)) as u32)).wrapping_mul(16)); let d_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(32); rt::memcpy(c, d_, s_, 16); };
                                    } else {
                                        { let _ = { let a0_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(32); F::memset(c, a0_, 0i32, ((16i32) as u32)) }; };
                                    }
                                }
                                F::EntrySel_ClipGoto(c, 1i32, 1i32, sym::menu_i_d__str99);
                            } else {
                                if (((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(440); (c.m.r32(a_) as i32) }) & (128i32))) != 0) {
                                    { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(384)).wrapping_add(((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                    { let s_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11584)).wrapping_add((((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(12); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(28)); let d_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(32); rt::memcpy(c, d_, s_, 16); };
                                    F::EntrySel_ClipGoto(c, 1i32, 6i32, sym::menu_i_d__str100);
                                } else {
                                    if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(972); c.m.r32(a_) }).wrapping_add(((((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)).wrapping_add({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) }) == (161i32)) as i32)) != 0) {
                                        'l2: loop {
                                            'c3: {
                                                l_slot_3 = (({ let a0_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(968); (c.m.r32(a_) as i32) }) as u32); F::Rand_Range(c, a0_) }) as i32);
                                                l_id_6 = { let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(972); c.m.r32(a_) }).wrapping_add((((l_slot_3) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) };
                                            }
                                            if !(((((l_id_6) >= (161i32)) as i32)) != 0) { break 'l2; }
                                        }
                                        l_chara_5 = l_id_6;
                                        if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(972); c.m.r32(a_) }).wrapping_add((((l_slot_3) as u32)).wrapping_mul(36))).wrapping_add(4); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                            l_cell_4 = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(972); c.m.r32(a_) }).wrapping_add((((l_slot_3) as u32)).wrapping_mul(36));
                                            l_chara_5 = { let a_ = ((l_cell_4).wrapping_add(8)).wrapping_add(({ let a0_ = (((c.m.r32((l_cell_4).wrapping_add(4)) as i32)) as u32); F::Rand_Range(c, a0_) }).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                                        }
                                        { let v_ = l_chara_5; let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                    } else {
                                        { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(384)).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                        l_slot_3 = ({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32));
                                    }
                                    if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                        { let s_ = (((c.m.r32(sym::gSaveData)).wrapping_add(6152)).wrapping_add((((l_slot_3) as u32)).wrapping_mul(56))).wrapping_add((((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }).wrapping_sub(1i32)) as u32)).wrapping_mul(16)); let d_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(32); rt::memcpy(c, d_, s_, 16); };
                                    } else {
                                        { let _ = { let a0_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(32); F::memset(c, a0_, 0i32, ((16i32) as u32)) }; };
                                    }
                                    F::EntrySel_ClipGoto(c, 1i32, 0i32, sym::menu_i_d__str101);
                                }
                            }
                            if (((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(440); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
                                { let r_ = 64i32; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(440); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                            }
                            if (((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(440); (c.m.r32(a_) as i32) }) & (128i32))) != 0) {
                                { let r_ = 128i32; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(440); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                            }
                            { let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            F::EntrySel_ClipGoto(c, 0i32, 7i32, sym::menu_i_d__str102);
                            F::EntrySel_SetMemberTex(c);
                            if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) == ({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(124)).wrapping_add(16); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                { let a0_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_i_d__str103, ((((1i32) as i64 as u64)) as u8)) };
                                { let v_ = 1i32; let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(500); c.m.w32(a_, (v_) as u32); };
                            } else {
                                F::EntrySel_ClipGoto(c, 1i32, 0i32, sym::menu_i_d__str104);
                                F::EntrySel_SwapRowTex(c);
                                { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(384)).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(448); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(444); c.m.w32(a_, (v_) as u32); };
                                F::EntrySel_ChangeImage(c);
                                { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(452); c.m.w32(a_, (v_) as u32); };
                            }
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                { let a0_ = ((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_i_d__str105, ((((1i32) as i64 as u64)) as u8)) };
                                F::EntrySel_ClipGoto(c, 2i32, 5i32, sym::menu_i_d__str106);
                                F::EntrySel_ClipGoto(c, 2i32, 2i32, sym::menu_i_d__str107);
                                { let v_ = 2i32; let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add(452); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                            }
                        }
                    }
                }
                break 's1;
            }
            6i32 | 7i32 => {
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EntrySel_Input", &PROBE_N, &[], true); }
    r_
}

pub fn EntrySel_UpdateEnd(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EntrySel_UpdateEnd", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(500); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        break 'f;
    }
    's1: {
        let sv1: i32 = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(500); (c.m.r32(a_) as i32) };
        match sv1 {
            1i32 => {
                's2: {
                    let sv2: i32 = { let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(124)).wrapping_add(8); (c.m.r32(a_) as i32) };
                    match sv2 {
                        0i32 => {
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6936); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 35i32; let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(484); c.m.w32(a_, (v_) as u32); };
                            break 's2;
                        }
                        1i32 => {
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6936); c.m.w32(a_, (v_) as u32); };
                            { let v_ = (((F::Rand_Range(c, ((2i32) as u32))).wrapping_add(((123i32) as u32))) as i32); let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(484); c.m.w32(a_, (v_) as u32); };
                            break 's2;
                        }
                        2i32 => {
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6936); c.m.w32(a_, (v_) as u32); };
                            { let v_ = (((F::Rand_Range(c, ((2i32) as u32))).wrapping_add(((78i32) as u32))) as i32); let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(484); c.m.w32(a_, (v_) as u32); };
                            break 's2;
                        }
                        3i32 => {
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6936); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 160i32; let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(484); c.m.w32(a_, (v_) as u32); };
                            break 's2;
                        }
                        4i32 => {
                            { let v_ = ((F::Rand_Range(c, ((2i32) as u32))) as i32); let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6936); c.m.w32(a_, (v_) as u32); };
                            { let v_ = ((({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6936); (c.m.r32(a_) as i32) }) ^ (1i32))).wrapping_add(203i32); let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(484); c.m.w32(a_, (v_) as u32); };
                            break 's2;
                        }
                        _ => {
                        }
                    }
                }
                { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(500); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(32); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(484); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34259i32, a2_) };
                break 's1;
            }
            2i32 => {
                if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(500); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(500); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                }
                break 's1;
            }
            3i32 => {
                { let r_ = 1i32; let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(472); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = 2i32; let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(472); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 15i32; let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(500); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EntrySel_UpdateEnd", &PROBE_N, &[], true); }
    r_
}

pub fn EntrySel_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EntrySel_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        { let v_ = 1i32; c.m.w32(fp, (v_) as u32); }
        F::EntrySel_Init(c, p_section_1);
        F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
        'l1: while ((1i32) != 0) {
            F::Gfx_BeginFrame(c);
            F::EntrySel_UpdateImage(c);
            F::Pad_Update(c);
            F::Snd_Update(c);
            F::ColorFade_Update(c);
            if ((((!(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0)) as i32)) != 0) {
                F::EntrySel_Update(c);
                F::EntrySel_UpdateEnd(c);
            }
            F::EntrySel_Draw(c);
            F::ColorFade_Draw(c);
            F::Gfx_EndFrame(c, 1i32);
            F::Dma_Flush(c);
            F::File_Stub264D90(c);
            if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
                continue 'l1;
            }
            if ((F::ColorFade_IsInDone(c)) != 0) {
                if ((((((((!(((({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(472); (c.m.r32(a_) as i32) }) & (8i32))) != 0)) as i32)) != 0) && (((({ let a_ = (((c.m.r32(sym::gEntrySel)).wrapping_add(40)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
                    { let r_ = 8i32; let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(472); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    's2: {
                        let sv3: i32 = { let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(124)).wrapping_add(8); (c.m.r32(a_) as i32) };
                        match sv3 {
                            0i32 => {
                                { let v_ = 34i32; let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(484); c.m.w32(a_, (v_) as u32); };
                                break 's2;
                            }
                            1i32 => {
                                { let v_ = 122i32; let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(484); c.m.w32(a_, (v_) as u32); };
                                break 's2;
                            }
                            2i32 => {
                                { let v_ = 77i32; let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(484); c.m.w32(a_, (v_) as u32); };
                                break 's2;
                            }
                            3i32 => {
                                { let v_ = 159i32; let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(484); c.m.w32(a_, (v_) as u32); };
                                break 's2;
                            }
                            4i32 => {
                                { let v_ = 202i32; let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(484); c.m.w32(a_, (v_) as u32); };
                                break 's2;
                            }
                            _ => {
                            }
                        }
                    }
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(6936); c.m.w32(a_, (v_) as u32); };
                    { let a0_ = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(32); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(484); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34259i32, a2_) };
                }
            }
            if ((F::ColorFade_IsFadingOut(c)) != 0) {
                F::Voice_FadeOutStep(c, 0i32);
                F::Bgm_FadeOutStep(c);
                continue 'l1;
            }
            if ((F::ColorFade_IsOutDone(c)) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(488); (c.m.r32(a_) as i32) }) != (4i32)) as i32)) != 0) {
                    continue 'l1;
                }
                break 'l1;
            }
            if (((({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(472); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(480); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) == ((1i32).wrapping_neg())) as i32)) != 0) {
                    F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                    {
                        l_i_2 = 0i32;
                        'l3: loop {
                            if !(((((l_i_2) < ({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(124)).wrapping_add(16); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l3; }
                            'c4: {
                                { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add((((l_i_2) as u32)).wrapping_mul(48))).wrapping_add(28); (c.m.r32(a_) as i32) }; let a_ = ((((c.m.r32(sym::gProgress)).wrapping_add(124)).wrapping_add(28)).wrapping_add((((l_i_2) as u32)).wrapping_mul(40))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add((((l_i_2) as u32)).wrapping_mul(48))).wrapping_add(24); (c.m.r32(a_) as i32) }; let a_ = ((((c.m.r32(sym::gProgress)).wrapping_add(124)).wrapping_add(28)).wrapping_add((((l_i_2) as u32)).wrapping_mul(40))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                { let v_ = l_i_2; let a_ = ((((c.m.r32(sym::gProgress)).wrapping_add(124)).wrapping_add(28)).wrapping_add((((l_i_2) as u32)).wrapping_mul(40))).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                { let r_ = 1i32; let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(124)).wrapping_add(28)).wrapping_add((((l_i_2) as u32)).wrapping_mul(40)); let n_ = ((((((c.m.r16(a_)) as i32)) | (r_))) as u16); c.m.w16(a_, n_); };
                                if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add((((l_i_2) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                    { let r_ = 32i32; let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(124)).wrapping_add(28)).wrapping_add((((l_i_2) as u32)).wrapping_mul(40)); let n_ = ((((((c.m.r16(a_)) as i32)) | (r_))) as u16); c.m.w16(a_, n_); };
                                    { let s_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add((((l_i_2) as u32)).wrapping_mul(48))).wrapping_add(32); let d_ = ((((c.m.r32(sym::gProgress)).wrapping_add(124)).wrapping_add(28)).wrapping_add((((l_i_2) as u32)).wrapping_mul(40))).wrapping_add(24); rt::memcpy(c, d_, s_, 16); };
                                    if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(972); c.m.r32(a_) }).wrapping_add(((((({ let a_ = (({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add((((l_i_2) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)).wrapping_add({ let a_ = ({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(964); c.m.r32(a_) }).wrapping_add((((l_i_2) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) }) == (163i32)) as i32)) != 0) {
                                        { let r_ = 16i32; let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(124)).wrapping_add(28)).wrapping_add((((l_i_2) as u32)).wrapping_mul(40)); let n_ = ((((((c.m.r16(a_)) as i32)) | (r_))) as u16); c.m.w16(a_, n_); };
                                    }
                                }
                            }
                            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                        }
                    }
                }
            } else {
                if ((((({ let a_ = (c.m.r32(sym::gEntrySel)).wrapping_add(500); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::EntrySel_Input(c, fp);
                }
            }
        }
        F::EntrySel_Term(c);
        F::Dma_ResetBuffers(c);
        break 'f (c.m.r32(fp) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EntrySel_Run", &PROBE_N, &[], true); }
    r_
}

pub fn Tour_Main(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Tour_Main", &PROBE_N, &[], false); }
    let mut l_result_1: i32 = 0;
    let mut l_done_2: i32 = 0;
    let mut l_load_3: i32 = 0;
    let r_ = 'f: {
    l_result_1 = 1i32;
    l_done_2 = 0i32;
    l_load_3 = 1i32;
    'l1: loop {
        'c2: {
            's3: {
                let sv4: i32 = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); (c.m.r32(a_) as i32) };
                match sv4 {
                    33i32 => {
                        if (((((c.m.r32(sym::gMenuArc4)) == (((0i32) as u32))) as i32)) != 0) {
                            if ((l_load_3) != 0) {
                                { let v_ = { let a0_ = ({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_add(2i32); F::File_LoadSync(c, a0_, ((0i32) as u32), 0i32) }; let a_ = sym::gMenuArc4; c.m.w32(a_, v_); };
                            }
                        }
                        F::Bgm_Play(c, 68377i32);
                        if ((F::TourMenu_Run(c, 1i32)) != 0) {
                            F::Adx_StopAll(c);
                            { let v_ = 34i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        } else {
                            F::Adx_StopAll(c);
                            { let v_ = 4i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                            l_result_1 = 0i32;
                            l_done_2 = 1i32;
                        }
                        break 's3;
                    }
                    34i32 => {
                        if (((((c.m.r32(sym::gMenuArc4)) == (((0i32) as u32))) as i32)) != 0) {
                            if ((l_load_3) != 0) {
                                { let v_ = { let a0_ = ({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_add(2i32); F::File_LoadSync(c, a0_, ((0i32) as u32), 0i32) }; let a_ = sym::gMenuArc4; c.m.w32(a_, v_); };
                            }
                        }
                        F::Bgm_Play(c, 68379i32);
                        if ((F::EntrySel_Run(c, 2i32)) != 0) {
                            F::Adx_StopAll(c);
                            { let v_ = 35i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                            if (((((c.m.r32(sym::gMenuArc4)) != (((0i32) as u32))) as i32)) != 0) {
                                { let a0_ = c.m.r32(sym::gMenuArc4); F::Heap_Free(c, a0_) };
                                { let v_ = ((0i32) as u32); let a_ = sym::gMenuArc4; c.m.w32(a_, v_); };
                            }
                        } else {
                            F::Adx_StopAll(c);
                            { let v_ = 33i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        }
                        break 's3;
                    }
                    35i32 => {
                        if (({ let _ = 0i32; F::Bracket_Run(c) }) != 0) {
                            l_done_2 = 1i32;
                            F::Adx_StopAll(c);
                        } else {
                            F::Adx_StopAll(c);
                            { let v_ = 33i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                            { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2572); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            if ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2572); (c.m.r32(a_) as i32) }) >= (24i32)) as i32)) != 0) {
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(2572); c.m.w32(a_, (v_) as u32); };
                            }
                        }
                        break 's3;
                    }
                    _ => {
                    }
                }
            }
            { let _ = F::sceGsSyncPath(c, 0i32, ((0i32) as u16)); };
        }
        if !((((!((l_done_2) != 0)) as i32)) != 0) { break 'l1; }
    }
    if (((((c.m.r32(sym::gMenuArc4)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gMenuArc4); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gMenuArc4; c.m.w32(a_, v_); };
    }
    break 'f l_result_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Tour_Main", &PROBE_N, &[], true); }
    r_
}
