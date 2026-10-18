#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn TexChain_WriteTags(c: &mut Ctx, p_chain_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TexChain_WriteTags", &PROBE_N, &[(p_chain_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_2: u32 = 0;
    let mut l_n_3: i32 = 0;
    rt::memcpy(c, fp, p_chain_1, 16);
    let r_ = 'f: {
        l_n_3 = 0i32;
        {
            l_i_2 = ((0i32) as u32);
            'l1: loop {
                if !(((((l_i_2) < (c.m.r32((fp).wrapping_add(8)))) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = (((805306368i32).wrapping_add(rt::div_i32({ let a_ = (c.m.r32(fp)).wrapping_add((l_i_2).wrapping_mul(16)); (c.m.r32(a_) as i32) }, 16i32))) as u32); let a_ = (c.m.r32((fp).wrapping_add(4))).wrapping_add(((({ let o_ = l_n_3; let n_ = (o_).wrapping_add(1); l_n_3 = n_; o_ }) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = { let a_ = ((c.m.r32(fp)).wrapping_add((l_i_2).wrapping_mul(16))).wrapping_add(4); c.m.r32(a_) }; let a_ = (c.m.r32((fp).wrapping_add(4))).wrapping_add(((({ let o_ = l_n_3; let n_ = (o_).wrapping_add(1); l_n_3 = n_; o_ }) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ((0i32) as u32); let a_ = (c.m.r32((fp).wrapping_add(4))).wrapping_add(((({ let o_ = l_n_3; let n_ = (o_).wrapping_add(1); l_n_3 = n_; o_ }) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ((0i32) as u32); let a_ = (c.m.r32((fp).wrapping_add(4))).wrapping_add(((({ let o_ = l_n_3; let n_ = (o_).wrapping_add(1); l_n_3 = n_; o_ }) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = ((1879048192i32) as u32); let a_ = (c.m.r32((fp).wrapping_add(4))).wrapping_add(((({ let o_ = l_n_3; let n_ = (o_).wrapping_add(1); l_n_3 = n_; o_ }) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32((fp).wrapping_add(4))).wrapping_add(((({ let o_ = l_n_3; let n_ = (o_).wrapping_add(1); l_n_3 = n_; o_ }) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32((fp).wrapping_add(4))).wrapping_add(((({ let o_ = l_n_3; let n_ = (o_).wrapping_add(1); l_n_3 = n_; o_ }) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32((fp).wrapping_add(4))).wrapping_add(((({ let o_ = l_n_3; let n_ = (o_).wrapping_add(1); l_n_3 = n_; o_ }) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "TexChain_WriteTags", &PROBE_N, &[], true); }
    r_
}

pub fn TexChain_Build(c: &mut Ctx, sret: u32, mut p_file_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TexChain_Build", &PROBE_N, &[(sret) as u32, (p_file_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_hdr_2: u32 = 0;
    let mut l_ofs_3: i32 = 0;
    let mut l_n_4: i32 = 0;
    let mut l_size_5: i32 = 0;
    let mut l_next_6: i32 = 0;
    let r_ = 'f: {
        l_ofs_3 = 16i32;
        l_n_4 = 1i32;
        l_hdr_2 = p_file_1;
        'l1: while ((1i32) != 0) {
            l_next_6 = (c.m.r32((l_hdr_2).wrapping_add(8)) as i32);
            if (((((l_next_6) == (0i32)) as i32)) != 0) {
                l_ofs_3 = ((c.m.r32((l_hdr_2).wrapping_add(12)) as i32)).wrapping_add(l_ofs_3);
                if (((((rt::rem_i32(l_ofs_3, 64i32)) != (0i32)) as i32)) != 0) {
                    l_ofs_3 = ((rt::div_i32(l_ofs_3, 64i32)).wrapping_mul(64i32)).wrapping_add(64i32);
                }
                break 'l1;
            }
            { let r_ = l_next_6; let n_ = (l_ofs_3).wrapping_add(r_); l_ofs_3 = n_; };
            { let o_ = l_n_4; let n_ = (o_).wrapping_add(1); l_n_4 = n_; };
            l_hdr_2 = (p_file_1).wrapping_add(((l_ofs_3) as u32));
            { let r_ = 16i32; let n_ = (l_ofs_3).wrapping_add(r_); l_ofs_3 = n_; };
        }
        { let _ = F::memset(c, fp, 0i32, 16u32); };
        { let v_ = (p_file_1).wrapping_add(((l_ofs_3) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let v_ = p_file_1; let a_ = (fp).wrapping_add(12); c.m.w32(a_, v_); };
        l_size_5 = (l_n_4).wrapping_mul(16i32);
        if (((((rt::rem_i32(l_size_5, 64i32)) != (0i32)) as i32)) != 0) {
            l_size_5 = ((rt::div_i32(l_size_5, 64i32)).wrapping_mul(64i32)).wrapping_add(64i32);
        }
        { let r_ = l_size_5; let n_ = (l_ofs_3).wrapping_add(r_); l_ofs_3 = n_; };
        { let v_ = (p_file_1).wrapping_add(((l_ofs_3) as u32)); let a_ = (fp).wrapping_add(4); c.m.w32(a_, v_); };
        l_n_4 = 0i32;
        l_ofs_3 = 0i32;
        'l2: loop {
            'c3: {
                l_hdr_2 = (p_file_1).wrapping_add(((l_ofs_3) as u32));
                { let r_ = 16i32; let n_ = (l_ofs_3).wrapping_add(r_); l_ofs_3 = n_; };
                { let v_ = c.m.r64(l_hdr_2); let a_ = ((c.m.r32(fp)).wrapping_add((((l_n_4) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w64(a_, v_); };
                if ((((((c.m.r32((l_hdr_2).wrapping_add(8)) as i32)) == (0i32)) as i32)) != 0) {
                    { let v_ = (c.m.r32((l_hdr_2).wrapping_add(12)) as i32); let a_ = (c.m.r32(fp)).wrapping_add((((l_n_4) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let v_ = (c.m.r32((l_hdr_2).wrapping_add(8)) as i32); let a_ = (c.m.r32(fp)).wrapping_add((((l_n_4) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_) as u32); };
                }
                { let v_ = (p_file_1).wrapping_add(((l_ofs_3) as u32)); let a_ = ((c.m.r32(fp)).wrapping_add((((l_n_4) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, v_); };
                { let o_ = l_n_4; let n_ = (o_).wrapping_add(1); l_n_4 = n_; };
                { let r_ = (c.m.r32((l_hdr_2).wrapping_add(8)) as i32); let n_ = (l_ofs_3).wrapping_add(r_); l_ofs_3 = n_; };
            }
            if !((((((c.m.r32((l_hdr_2).wrapping_add(8)) as i32)) != (0i32)) as i32)) != 0) { break 'l2; }
        }
        { let v_ = ((l_n_4) as u32); let a_ = (fp).wrapping_add(8); c.m.w32(a_, v_); };
        F::TexChain_WriteTags(c, fp);
        break 'f { let s_ = fp; rt::memcpy(c, sret, s_, 16); sret };
       sret
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "TexChain_Build", &PROBE_N, &[], true); }
    r_
}

pub fn TexChain_BuildAt(c: &mut Ctx, sret: u32, mut p_file_1: u32, mut p_unused_2: i32, mut p_cbp_3: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TexChain_BuildAt", &PROBE_N, &[(sret) as u32, (p_file_1) as u32, (p_unused_2) as u32, (p_cbp_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let r_ = 'f: {
        { let s_ = F::TexChain_Build(c, fp.wrapping_add(16), p_file_1); let d_ = fp; rt::memcpy(c, d_, s_, 16); };
        F::TexChain_AddCbp(c, fp, p_cbp_3);
        break 'f { let s_ = fp; rt::memcpy(c, sret, s_, 16); sret };
       sret
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "TexChain_BuildAt", &PROBE_N, &[], true); }
    r_
}

pub fn TexChain_BuildPair(c: &mut Ctx, sret: u32, mut p_file_1: u32, mut p_index_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TexChain_BuildPair", &PROBE_N, &[(sret) as u32, (p_file_1) as u32, (p_index_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_hdr_3: u32 = 0;
    let mut l_ofs_4: i32 = 0;
    let mut l_n_5: i32 = 0;
    let mut l_size_6: i32 = 0;
    let mut l_i_7: i32 = 0;
    let mut l_count_8: i32 = 0;
    let mut l_next_9: i32 = 0;
    let r_ = 'f: {
        l_ofs_4 = 16i32;
        l_n_5 = 1i32;
        l_count_8 = 2i32;
        l_hdr_3 = p_file_1;
        'l1: while ((1i32) != 0) {
            l_next_9 = (c.m.r32((l_hdr_3).wrapping_add(8)) as i32);
            if (((((l_next_9) == (0i32)) as i32)) != 0) {
                l_ofs_4 = ((c.m.r32((l_hdr_3).wrapping_add(12)) as i32)).wrapping_add(l_ofs_4);
                if (((((rt::rem_i32(l_ofs_4, 64i32)) != (0i32)) as i32)) != 0) {
                    l_ofs_4 = ((rt::div_i32(l_ofs_4, 64i32)).wrapping_mul(64i32)).wrapping_add(64i32);
                }
                break 'l1;
            }
            { let r_ = l_next_9; let n_ = (l_ofs_4).wrapping_add(r_); l_ofs_4 = n_; };
            { let o_ = l_n_5; let n_ = (o_).wrapping_add(1); l_n_5 = n_; };
            l_hdr_3 = (p_file_1).wrapping_add(((l_ofs_4) as u32));
            { let r_ = 16i32; let n_ = (l_ofs_4).wrapping_add(r_); l_ofs_4 = n_; };
        }
        { let _ = F::memset(c, fp, 0i32, 16u32); };
        { let v_ = ((p_file_1).wrapping_add(((l_ofs_4) as u32))).wrapping_add((((p_index_2).wrapping_mul(32i32)) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let v_ = p_file_1; let a_ = (fp).wrapping_add(12); c.m.w32(a_, v_); };
        l_size_6 = (l_n_5).wrapping_mul(16i32);
        if (((((rt::rem_i32(l_size_6, 64i32)) != (0i32)) as i32)) != 0) {
            l_size_6 = ((rt::div_i32(l_size_6, 64i32)).wrapping_mul(64i32)).wrapping_add(64i32);
        }
        { let r_ = l_size_6; let n_ = (l_ofs_4).wrapping_add(r_); l_ofs_4 = n_; };
        { let v_ = ((p_file_1).wrapping_add(((l_ofs_4) as u32))).wrapping_add((((p_index_2).wrapping_mul(64i32)) as u32)); let a_ = (fp).wrapping_add(4); c.m.w32(a_, v_); };
        l_ofs_4 = 0i32;
        {
            l_i_7 = 0i32;
            'l2: loop {
                if !(((((l_i_7) < ((p_index_2).wrapping_mul(2i32))) as i32)) != 0) { break 'l2; }
                'c3: {
                    l_hdr_3 = (p_file_1).wrapping_add(((l_ofs_4) as u32));
                    { let r_ = 16i32; let n_ = (l_ofs_4).wrapping_add(r_); l_ofs_4 = n_; };
                    if ((((((c.m.r32((l_hdr_3).wrapping_add(8)) as i32)) == (0i32)) as i32)) != 0) {
                        break 'l2;
                    }
                    { let r_ = (c.m.r32((l_hdr_3).wrapping_add(8)) as i32); let n_ = (l_ofs_4).wrapping_add(r_); l_ofs_4 = n_; };
                }
                { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
            }
        }
        {
            l_i_7 = 0i32;
            'l4: loop {
                if !(((((l_i_7) < (l_count_8)) as i32)) != 0) { break 'l4; }
                'c5: {
                    l_hdr_3 = (p_file_1).wrapping_add(((l_ofs_4) as u32));
                    { let r_ = 16i32; let n_ = (l_ofs_4).wrapping_add(r_); l_ofs_4 = n_; };
                    { let v_ = c.m.r64(l_hdr_3); let a_ = ((c.m.r32(fp)).wrapping_add((((l_i_7) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w64(a_, v_); };
                    if ((((((c.m.r32((l_hdr_3).wrapping_add(8)) as i32)) == (0i32)) as i32)) != 0) {
                        { let v_ = (c.m.r32((l_hdr_3).wrapping_add(12)) as i32); let a_ = (c.m.r32(fp)).wrapping_add((((l_i_7) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_) as u32); };
                    } else {
                        { let v_ = (c.m.r32((l_hdr_3).wrapping_add(8)) as i32); let a_ = (c.m.r32(fp)).wrapping_add((((l_i_7) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_) as u32); };
                    }
                    { let v_ = (p_file_1).wrapping_add(((l_ofs_4) as u32)); let a_ = ((c.m.r32(fp)).wrapping_add((((l_i_7) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, v_); };
                    if ((((((c.m.r32((l_hdr_3).wrapping_add(8)) as i32)) == (0i32)) as i32)) != 0) {
                        break 'l4;
                    }
                    { let r_ = (c.m.r32((l_hdr_3).wrapping_add(8)) as i32); let n_ = (l_ofs_4).wrapping_add(r_); l_ofs_4 = n_; };
                }
                { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
            }
        }
        { let v_ = ((l_count_8) as u32); let a_ = (fp).wrapping_add(8); c.m.w32(a_, v_); };
        F::TexChain_WriteTags(c, fp);
        break 'f { let s_ = fp; rt::memcpy(c, sret, s_, 16); sret };
       sret
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "TexChain_BuildPair", &PROBE_N, &[], true); }
    r_
}

pub fn TexChain_BuildPairs(c: &mut Ctx, mut p_out_1: u32, mut p_file_2: u32, mut p_cbp_3: i32, mut p_count_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TexChain_BuildPairs", &PROBE_N, &[(p_out_1) as u32, (p_file_2) as u32, (p_cbp_3) as u32, (p_count_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
        {
            l_i_5 = 0i32;
            'l1: loop {
                if !(((((l_i_5) < (p_count_4)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let s_ = F::TexChain_BuildPair(c, fp, p_file_2, l_i_5); let d_ = (p_out_1).wrapping_add((((l_i_5) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
                    F::TexChain_AddCbp(c, (p_out_1).wrapping_add((((l_i_5) as u32)).wrapping_mul(16)), p_cbp_3);
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "TexChain_BuildPairs", &PROBE_N, &[], true); }
    r_
}

pub fn TexChain_AddCbp(c: &mut Ctx, p_chain_1: u32, mut p_add_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TexChain_AddCbp", &PROBE_N, &[(p_chain_1) as u32, (p_add_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_3: u32 = 0;
    rt::memcpy(c, fp, p_chain_1, 16);
    let r_ = 'f: {
        {
            l_i_3 = ((0i32) as u32);
            'l1: loop {
                if !(((((l_i_3) < (c.m.r32((fp).wrapping_add(8)))) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = (((p_add_2) as u32)).wrapping_add({ let a_ = ({ let a_ = ((c.m.r32(fp)).wrapping_add((l_i_3).wrapping_mul(16))).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }); let a_ = ({ let a_ = ((c.m.r32(fp)).wrapping_add((l_i_3).wrapping_mul(16))).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "TexChain_AddCbp", &PROBE_N, &[], true); }
    r_
}

pub fn Tex_Log2Size(c: &mut Ctx, mut p_size_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Tex_Log2Size", &PROBE_N, &[(p_size_1) as u32], false); }
    let mut l_n_2: i32 = 0;
    let r_ = 'f: {
    's1: {
        let sv3: i32 = p_size_1;
        match sv3 {
            2i32 => {
                l_n_2 = 1i32;
                break 's1;
            }
            4i32 => {
                l_n_2 = 2i32;
                break 's1;
            }
            8i32 => {
                l_n_2 = 3i32;
                break 's1;
            }
            16i32 => {
                l_n_2 = 4i32;
                break 's1;
            }
            32i32 => {
                l_n_2 = 5i32;
                break 's1;
            }
            64i32 => {
                l_n_2 = 6i32;
                break 's1;
            }
            112i32 => {
                l_n_2 = 7i32;
                break 's1;
            }
            128i32 => {
                l_n_2 = 7i32;
                break 's1;
            }
            224i32 => {
                l_n_2 = 8i32;
                break 's1;
            }
            256i32 => {
                l_n_2 = 8i32;
                break 's1;
            }
            448i32 => {
                l_n_2 = 9i32;
                break 's1;
            }
            512i32 => {
                l_n_2 = 9i32;
                break 's1;
            }
            640i32 => {
                l_n_2 = 10i32;
                break 's1;
            }
            _ => {
                l_n_2 = 1i32;
                break 's1;
            }
        }
    }
    break 'f l_n_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Tex_Log2Size", &PROBE_N, &[], true); }
    r_
}

pub fn Res_RelocateOffsets(c: &mut Ctx, mut p_out_1: u32, mut p_base_2: u32, mut p_hdr_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Res_RelocateOffsets", &PROBE_N, &[(p_out_1) as u32, (p_base_2) as u32, (p_hdr_3) as u32], false); }
    let mut l_i_4: u32 = 0;
    let mut l_e_5: u32 = 0;
    let r_ = 'f: {
    { let v_ = p_hdr_3; let a_ = p_out_1; c.m.w32(a_, v_); };
    { let v_ = (p_base_2).wrapping_add((c.m.r32((p_hdr_3).wrapping_add(4))).wrapping_mul(((4i32) as u32))); let a_ = (p_hdr_3).wrapping_add(16); c.m.w32(a_, v_); };
    {
        l_i_4 = ((0i32) as u32);
        'l1: loop {
            if !(((((l_i_4) < ({ let a_ = c.m.r32(p_out_1); c.m.r32(a_) })) as i32)) != 0) { break 'l1; }
            'c2: {
                l_e_5 = ({ let a_ = (c.m.r32(p_out_1)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((l_i_4).wrapping_mul(64));
                if (((((c.m.r32(l_e_5)) != (((0i32) as u32))) as i32)) != 0) {
                    { let v_ = (p_base_2).wrapping_add((c.m.r32(l_e_5)).wrapping_mul(((4i32) as u32))); let a_ = (l_e_5).wrapping_add(56); c.m.w32(a_, v_); };
                }
                if (((((c.m.r32((l_e_5).wrapping_add(4))) != (((0i32) as u32))) as i32)) != 0) {
                    { let v_ = (p_base_2).wrapping_add((c.m.r32((l_e_5).wrapping_add(4))).wrapping_mul(((4i32) as u32))); let a_ = (l_e_5).wrapping_add(60); c.m.w32(a_, v_); };
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Res_RelocateOffsets", &PROBE_N, &[], true); }
    r_
}

pub fn Tex_UploadPixels(c: &mut Ctx, mut p_e_1: u32, mut p_tbp_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Tex_UploadPixels", &PROBE_N, &[(p_e_1) as u32, (p_tbp_2) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(p_e_1)) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = (((((((p_tbp_2) as i64)).wrapping_shl((32i32) as u32)) as u64)) | ((((c.m.r32((p_e_1).wrapping_add(24))) as u64)).wrapping_shl((48i32) as u32))); let a_ = (sym::gTexBltPacket).wrapping_add(32); c.m.w64(a_, v_); };
        F::Dma_AddData(c, sym::gTexBltPacket, ((48u32) as i32));
        { let a0_ = c.m.r32((p_e_1).wrapping_add(56)); let a1_ = (c.m.r32((p_e_1).wrapping_add(8)) as i32); F::Dma_AddRef(c, a0_, a1_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Tex_UploadPixels", &PROBE_N, &[], true); }
    r_
}

pub fn Tex_UploadClut(c: &mut Ctx, mut p_e_1: u32, mut p_cbp_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Tex_UploadClut", &PROBE_N, &[(p_e_1) as u32, (p_cbp_2) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32((p_e_1).wrapping_add(4))) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = (((((((p_cbp_2) as i64)).wrapping_shl((32i32) as u32)) as u64)) | ((((c.m.r32((p_e_1).wrapping_add(28))) as u64)).wrapping_shl((48i32) as u32))); let a_ = (sym::gTexBltPacket).wrapping_add(32); c.m.w64(a_, v_); };
        F::Dma_AddData(c, sym::gTexBltPacket, ((48u32) as i32));
        { let a0_ = c.m.r32((p_e_1).wrapping_add(60)); let a1_ = (c.m.r32((p_e_1).wrapping_add(12)) as i32); F::Dma_AddRef(c, a0_, a1_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Tex_UploadClut", &PROBE_N, &[], true); }
    r_
}

pub fn TexFile_UploadPacked(c: &mut Ctx, mut p_file_1: u32, mut p_tbp_2: i32, mut p_cbp_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TexFile_UploadPacked", &PROBE_N, &[(p_file_1) as u32, (p_tbp_2) as u32, (p_cbp_3) as u32], false); }
    let mut l_i_4: u32 = 0;
    let mut l_e_5: u32 = 0;
    let r_ = 'f: {
    {
        l_i_4 = ((0i32) as u32);
        'l1: loop {
            if !(((((l_i_4) < (c.m.r32(p_file_1))) as i32)) != 0) { break 'l1; }
            'c2: {
                l_e_5 = (c.m.r32((p_file_1).wrapping_add(16))).wrapping_add((l_i_4).wrapping_mul(64));
                F::Tex_UploadPixels(c, l_e_5, p_tbp_2);
                F::Tex_UploadClut(c, l_e_5, p_cbp_3);
                { let r_ = (c.m.r32((l_e_5).wrapping_add(16)) as i32); let n_ = (p_tbp_2).wrapping_add(r_); p_tbp_2 = n_; };
                { let r_ = (c.m.r32((l_e_5).wrapping_add(20)) as i32); let n_ = (p_cbp_3).wrapping_add(r_); p_cbp_3 = n_; };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "TexFile_UploadPacked", &PROBE_N, &[], true); }
    r_
}

pub fn TexFile_UploadOne(c: &mut Ctx, mut p_file_1: u32, mut p_index_2: i32, mut p_tbp_3: i32, mut p_cbp_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TexFile_UploadOne", &PROBE_N, &[(p_file_1) as u32, (p_index_2) as u32, (p_tbp_3) as u32, (p_cbp_4) as u32], false); }
    let mut l_e_5: u32 = 0;
    let r_ = 'f: {
    l_e_5 = (c.m.r32((p_file_1).wrapping_add(16))).wrapping_add((((p_index_2) as u32)).wrapping_mul(64));
    if (((((p_tbp_3) >= (0i32)) as i32)) != 0) {
        F::Tex_UploadPixels(c, l_e_5, p_tbp_3);
    }
    if (((((p_cbp_4) >= (0i32)) as i32)) != 0) {
        F::Tex_UploadClut(c, l_e_5, p_cbp_4);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "TexFile_UploadOne", &PROBE_N, &[], true); }
    r_
}

pub fn Tex_Upload(c: &mut Ctx, mut p_e_1: u32, mut p_tbp_2: i32, mut p_cbp_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Tex_Upload", &PROBE_N, &[(p_e_1) as u32, (p_tbp_2) as u32, (p_cbp_3) as u32], false); }
    let r_ = 'f: {
    if (((((p_tbp_2) >= (0i32)) as i32)) != 0) {
        F::Tex_UploadPixels(c, p_e_1, p_tbp_2);
    }
    if (((((p_cbp_3) >= (0i32)) as i32)) != 0) {
        F::Tex_UploadClut(c, p_e_1, p_cbp_3);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Tex_Upload", &PROBE_N, &[], true); }
    r_
}

pub fn TexFile_UploadRangePacked(c: &mut Ctx, mut p_file_1: u32, mut p_first_2: i32, mut p_count_3: i32, mut p_tbp_4: i32, mut p_cbp_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TexFile_UploadRangePacked", &PROBE_N, &[(p_file_1) as u32, (p_first_2) as u32, (p_count_3) as u32, (p_tbp_4) as u32, (p_cbp_5) as u32], false); }
    let mut l_i_6: i32 = 0;
    let mut l_e_7: u32 = 0;
    let r_ = 'f: {
    {
        l_i_6 = 0i32;
        'l1: loop {
            if !(((((((l_i_6) as u32)) < (c.m.r32(p_file_1))) as i32)) != 0) { break 'l1; }
            'c2: {
                l_e_7 = (c.m.r32((p_file_1).wrapping_add(16))).wrapping_add((((l_i_6) as u32)).wrapping_mul(64));
                if (((((((((l_i_6) >= (p_first_2)) as i32)) != 0) && (((((l_i_6) < ((p_first_2).wrapping_add(p_count_3))) as i32)) != 0)) as i32)) != 0) {
                    F::Tex_UploadPixels(c, l_e_7, p_tbp_4);
                    F::Tex_UploadClut(c, l_e_7, p_cbp_5);
                }
                { let r_ = (c.m.r32((l_e_7).wrapping_add(16)) as i32); let n_ = (p_tbp_4).wrapping_add(r_); p_tbp_4 = n_; };
                { let r_ = (c.m.r32((l_e_7).wrapping_add(20)) as i32); let n_ = (p_cbp_5).wrapping_add(r_); p_cbp_5 = n_; };
            }
            { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "TexFile_UploadRangePacked", &PROBE_N, &[], true); }
    r_
}

pub fn TexFile_UploadRange(c: &mut Ctx, mut p_file_1: u32, mut p_first_2: i32, mut p_count_3: i32, mut p_tbp_4: i32, mut p_cbp_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TexFile_UploadRange", &PROBE_N, &[(p_file_1) as u32, (p_first_2) as u32, (p_count_3) as u32, (p_tbp_4) as u32, (p_cbp_5) as u32], false); }
    let mut l_i_6: i32 = 0;
    let mut l_e_7: u32 = 0;
    let r_ = 'f: {
    {
        l_i_6 = 0i32;
        'l1: loop {
            if !(((((((l_i_6) as u32)) < (c.m.r32(p_file_1))) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((((((l_i_6) >= (p_first_2)) as i32)) != 0) && (((((l_i_6) < ((p_first_2).wrapping_add(p_count_3))) as i32)) != 0)) as i32)) != 0) {
                    l_e_7 = (c.m.r32((p_file_1).wrapping_add(16))).wrapping_add((((l_i_6) as u32)).wrapping_mul(64));
                    if (((((p_tbp_4) >= (0i32)) as i32)) != 0) {
                        { let a1_ = ((c.m.r32((l_e_7).wrapping_add(32)) as i32)).wrapping_add(p_tbp_4); F::Tex_UploadPixels(c, l_e_7, a1_) };
                    }
                    if (((((p_cbp_5) >= (0i32)) as i32)) != 0) {
                        { let a1_ = ((c.m.r32((l_e_7).wrapping_add(36)) as i32)).wrapping_add(p_cbp_5); F::Tex_UploadClut(c, l_e_7, a1_) };
                    }
                }
            }
            { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "TexFile_UploadRange", &PROBE_N, &[], true); }
    r_
}

pub fn TexFile_UploadAll(c: &mut Ctx, mut p_file_1: u32, mut p_tbp_2: i32, mut p_cbp_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TexFile_UploadAll", &PROBE_N, &[(p_file_1) as u32, (p_tbp_2) as u32, (p_cbp_3) as u32], false); }
    let mut l_i_4: u32 = 0;
    let mut l_e_5: u32 = 0;
    let r_ = 'f: {
    {
        l_i_4 = ((0i32) as u32);
        'l1: loop {
            if !(((((l_i_4) < (c.m.r32(p_file_1))) as i32)) != 0) { break 'l1; }
            'c2: {
                l_e_5 = (c.m.r32((p_file_1).wrapping_add(16))).wrapping_add((l_i_4).wrapping_mul(64));
                if ((((((c.m.r32((l_e_5).wrapping_add(8)) as i32)) != (0i32)) as i32)) != 0) {
                    { let a1_ = ((c.m.r32((l_e_5).wrapping_add(32)) as i32)).wrapping_add(p_tbp_2); F::Tex_UploadPixels(c, l_e_5, a1_) };
                }
                if ((((((c.m.r32((l_e_5).wrapping_add(12)) as i32)) != (0i32)) as i32)) != 0) {
                    { let a1_ = ((c.m.r32((l_e_5).wrapping_add(36)) as i32)).wrapping_add(p_cbp_3); F::Tex_UploadClut(c, l_e_5, a1_) };
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "TexFile_UploadAll", &PROBE_N, &[], true); }
    r_
}

pub fn TexFile_UploadAt(c: &mut Ctx, mut p_file_1: u32, mut p_index_2: i32, mut p_tbp_3: i32, mut p_cbp_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TexFile_UploadAt", &PROBE_N, &[(p_file_1) as u32, (p_index_2) as u32, (p_tbp_3) as u32, (p_cbp_4) as u32], false); }
    let mut l_e_5: u32 = 0;
    let r_ = 'f: {
    l_e_5 = (c.m.r32((p_file_1).wrapping_add(16))).wrapping_add((((p_index_2) as u32)).wrapping_mul(64));
    if ((((((c.m.r32((l_e_5).wrapping_add(8)) as i32)) != (0i32)) as i32)) != 0) {
        { let a1_ = ((c.m.r32((l_e_5).wrapping_add(32)) as i32)).wrapping_add(p_tbp_3); F::Tex_UploadPixels(c, l_e_5, a1_) };
    }
    if ((((((c.m.r32((l_e_5).wrapping_add(12)) as i32)) != (0i32)) as i32)) != 0) {
        { let a1_ = ((c.m.r32((l_e_5).wrapping_add(36)) as i32)).wrapping_add(p_cbp_4); F::Tex_UploadClut(c, l_e_5, a1_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "TexFile_UploadAt", &PROBE_N, &[], true); }
    r_
}

pub fn GfxClut_InitPacket(c: &mut Ctx, mut p_work_1: u32, mut p_cbp_2: u16) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxClut_InitPacket", &PROBE_N, &[(p_work_1) as u32, (p_cbp_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let mut l_blt_4: u64 = 0;
    let mut l_endTag_5: u32 = 0;
    let r_ = 'f: {
    l_w_3 = p_work_1;
    l_blt_4 = (((p_cbp_2) as u64)).wrapping_shl((32i32) as u32);
    l_endTag_5 = ((1879048192i32) as u32);
    { let v_ = p_cbp_2; let a_ = (p_work_1).wrapping_add(1204); c.m.w16(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_w_3).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_w_3).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_w_3).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((1342177280i32) | (72i32))) as u32); let a_ = (l_w_3).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((32768i32) | (4i32))) as u32); let a_ = (l_w_3).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((268435456i32) as u32); let a_ = (l_w_3).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((14i32) as u32); let a_ = (l_w_3).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_w_3).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((l_blt_4) as u32); let a_ = (l_w_3).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (((l_blt_4).wrapping_shr((32i32) as u32)) as u32); let a_ = (l_w_3).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((80i32) as u32); let a_ = (l_w_3).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_w_3).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_w_3).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_w_3).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((81i32) as u32); let a_ = (l_w_3).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_w_3).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((16i32) as u32); let a_ = (l_w_3).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((16i32) as u32); let a_ = (l_w_3).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((82i32) as u32); let a_ = (l_w_3).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_w_3).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_w_3).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_w_3).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((83i32) as u32); let a_ = (l_w_3).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_w_3).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((32768i32) | (64i32))) as u32); let a_ = (l_w_3).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((134217728i32) as u32); let a_ = (l_w_3).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_w_3).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_w_3).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((32768i32) | (1i32))) as u32); let a_ = (l_w_3).wrapping_add((((rt::div_i32(1136i32, 4i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((268435456i32) as u32); let a_ = (l_w_3).wrapping_add((((rt::div_i32(1140i32, 4i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((14i32) as u32); let a_ = (l_w_3).wrapping_add((((rt::div_i32(1144i32, 4i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_w_3).wrapping_add((((rt::div_i32(1148i32, 4i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_w_3).wrapping_add((((rt::div_i32(1152i32, 4i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_w_3).wrapping_add((((rt::div_i32(1156i32, 4i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (l_w_3).wrapping_add((((28i32) as u32)).wrapping_mul(4)); let a_ = (p_work_1).wrapping_add(1168); c.m.w32(a_, v_); };
    { let v_ = ((63i32) as u32); let a_ = (l_w_3).wrapping_add((((rt::div_i32(1160i32, 4i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_w_3).wrapping_add((((rt::div_i32(1164i32, 4i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = p_work_1; let a_ = ((p_work_1).wrapping_add(1172)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = ((p_work_1).wrapping_add(1172)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = ((p_work_1).wrapping_add(1172)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = l_endTag_5; let a_ = ((p_work_1).wrapping_add(1188)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = ((p_work_1).wrapping_add(1188)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = ((p_work_1).wrapping_add(1188)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = ((p_work_1).wrapping_add(1188)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((805306368i32) | (73i32))) as u32); let a_ = ((p_work_1).wrapping_add(1172)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "GfxClut_InitPacket", &PROBE_N, &[], true); }
    r_
}
