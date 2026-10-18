#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Flash_CheckHeader(c: &mut Ctx, mut p_data_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_CheckHeader", &PROBE_N, &[(p_data_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_ver_2: u8 = 0;
    let r_ = 'f: {
        { let _ = F::memset(c, fp, 0i32, 8u32); };
        { let _ = F::memcpy(c, fp, p_data_1, 8u32); };
        if (((((((((((((((c.m.r8((fp).wrapping_add(((0i32) as u32)))) as i32)) != (70i32)) as i32)) != 0) || (((((((c.m.r8((fp).wrapping_add(((1i32) as u32)))) as i32)) != (79i32)) as i32)) != 0)) as i32)) != 0) || (((((((c.m.r8((fp).wrapping_add(((2i32) as u32)))) as i32)) != (68i32)) as i32)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        l_ver_2 = c.m.r8((fp).wrapping_add(((3i32) as u32)));
        if (((((((l_ver_2) as i32)) != (17i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        break 'f (((F::strcmp(c, (fp).wrapping_add(4), sym::gfxm_b_c__str0)) == (0i32)) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Flash_CheckHeader", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_FindTag(c: &mut Ctx, mut p_data_1: u32, mut p_code_2: u8) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_FindTag", &PROBE_N, &[(p_data_1) as u32, (p_code_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_ofs_3: i32 = 0;
    let mut l_found_4: u32 = 0;
    let r_ = 'f: {
        l_ofs_3 = 0i32;
        l_found_4 = ((0i32) as u32);
        'l1: loop {
            'c2: {
                { let _ = F::memset(c, fp, 0i32, 8u32); };
                { let _ = F::memcpy(c, fp, (p_data_1).wrapping_add(((l_ofs_3) as u32)), 8u32); };
                if (((((((c.m.r8(fp)) as i32)) == (((p_code_2) as i32))) as i32)) != 0) {
                    l_found_4 = (p_data_1).wrapping_add(((l_ofs_3) as u32));
                    break 'l1;
                }
                l_ofs_3 = ((((((l_ofs_3) as u32)).wrapping_add(c.m.r32((fp).wrapping_add(4)))).wrapping_add(((8i32) as u32))) as i32);
            }
            if !(((((((c.m.r8(fp)) as i32)) != (0i32)) as i32)) != 0) { break 'l1; }
        }
        break 'f l_found_4;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Flash_FindTag", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_SkipRecords(c: &mut Ctx, mut p_tag_1: u32, mut p_count_2: u16) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_SkipRecords", &PROBE_N, &[(p_tag_1) as u32, (p_count_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_n_3: i32 = 0;
    let r_ = 'f: {
        l_n_3 = ((p_count_2) as i32);
        { let _ = F::memset(c, fp, 0i32, 8u32); };
        { let _ = F::memcpy(c, fp, p_tag_1, 8u32); };
        { let r_ = 8i32; let n_ = (p_tag_1).wrapping_add(((r_) as u32)); p_tag_1 = n_; };
        'l1: while (((((l_n_3) != (0i32)) as i32)) != 0) {
            { let _ = F::memset(c, fp.wrapping_add(8), 0i32, 8u32); };
            { let o_ = l_n_3; let n_ = (o_).wrapping_sub(1); l_n_3 = n_; };
            { let _ = F::memcpy(c, fp.wrapping_add(8), p_tag_1, 8u32); };
            { let r_ = 8i32; let n_ = (p_tag_1).wrapping_add(((r_) as u32)); p_tag_1 = n_; };
            { let r_ = c.m.r32((fp.wrapping_add(8)).wrapping_add(4)); let n_ = (p_tag_1).wrapping_add(r_); p_tag_1 = n_; };
        }
        break 'f p_tag_1;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Flash_SkipRecords", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_GetRecord(c: &mut Ctx, mut p_data_1: u32, mut p_code_2: u8, mut p_index_3: u16) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_GetRecord", &PROBE_N, &[(p_data_1) as u32, (p_code_2) as u32, (p_index_3) as u32], false); }
    let r_ = 'f: {
    break 'f { let a0_ = F::Flash_FindTag(c, p_data_1, p_code_2); F::Flash_SkipRecords(c, a0_, p_index_3) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Flash_GetRecord", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_GetRecordCount(c: &mut Ctx, mut p_data_1: u32, mut p_code_2: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_GetRecordCount", &PROBE_N, &[(p_data_1) as u32, (p_code_2) as u32], false); }
    let mut l_count_3: i32 = 0;
    let mut l_tag_4: u32 = 0;
    let r_ = 'f: {
    l_count_3 = 0i32;
    l_tag_4 = F::Flash_FindTag(c, p_data_1, p_code_2);
    if (((((l_tag_4) != (((0i32) as u32))) as i32)) != 0) {
        l_count_3 = ((c.m.r16((l_tag_4).wrapping_add(2))) as i32);
    }
    break 'f l_count_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Flash_GetRecordCount", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_SkipNamed(c: &mut Ctx, mut p_p_1: u32, mut p_count_2: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_SkipNamed", &PROBE_N, &[(p_p_1) as u32, (p_count_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_3: u32 = 0;
    let mut l_ofs_4: i32 = 0;
    let r_ = 'f: {
        l_ofs_4 = 0i32;
        {
            l_i_3 = ((0i32) as u32);
            'l1: loop {
                if !(((((l_i_3) < (p_count_2)) as i32)) != 0) { break 'l1; }
                'c2: {
                    'l3: while (((((((c.m.r8((p_p_1).wrapping_add((({ let o_ = l_ofs_4; let n_ = (o_).wrapping_add(1); l_ofs_4 = n_; o_ }) as u32)))) as i32)) != (0i32)) as i32)) != 0) {
                    }
                    { let _ = F::memcpy(c, fp, (p_p_1).wrapping_add(((l_ofs_4) as u32)), ((2i32) as u32)); };
                    { let r_ = 2i32; let n_ = (l_ofs_4).wrapping_add(r_); l_ofs_4 = n_; };
                    if (((((((c.m.r16(fp)) as i32)) != (0i32)) as i32)) != 0) {
                        { let r_ = ((c.m.r16(fp)) as i32); let n_ = (l_ofs_4).wrapping_add(r_); l_ofs_4 = n_; };
                    } else {
                        break 'f ((0i32) as u32);
                    }
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        break 'f (p_p_1).wrapping_add(((l_ofs_4) as u32));
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Flash_SkipNamed", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_FindName(c: &mut Ctx, mut p_p_1: u32, mut p_name_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_FindName", &PROBE_N, &[(p_p_1) as u32, (p_name_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(272) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 272);
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
        { let v_ = 0i32; c.m.w32(fp.wrapping_add(256), (v_) as u32); }
        { let v_ = ((0i32) as u16); c.m.w16(fp.wrapping_add(260), v_); }
        l_i_3 = 0i32;
        'l1: loop {
            'c2: {
                { let r_ = ((c.m.r16(fp.wrapping_add(260))) as i32); let a_ = fp.wrapping_add(256); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                { let _ = F::memset(c, fp, 0i32, ((255i32) as u32)); };
                F::Flash_ReadString(c, fp, p_p_1, fp.wrapping_add(256));
                if (((((F::strcmp(c, fp, p_name_2)) == (0i32)) as i32)) != 0) {
                    break 'f l_i_3;
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                { let _ = { let a1_ = (p_p_1).wrapping_add((((c.m.r32(fp.wrapping_add(256)) as i32)) as u32)); F::memcpy(c, fp.wrapping_add(260), a1_, ((2i32) as u32)) }; };
                { let r_ = 2i32; let a_ = fp.wrapping_add(256); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
            }
            if !(((((((c.m.r16(fp.wrapping_add(260))) as i32)) != (0i32)) as i32)) != 0) { break 'l1; }
        }
        break 'f (1i32).wrapping_neg();
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Flash_FindName", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_ReadString(c: &mut Ctx, mut p_dst_1: u32, mut p_p_2: u32, mut p_ofs_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_ReadString", &PROBE_N, &[(p_dst_1) as u32, (p_p_2) as u32, (p_ofs_3) as u32], false); }
    let mut l_src_4: u32 = 0;
    let mut l_n_5: i32 = 0;
    let r_ = 'f: {
    l_src_4 = (p_p_2).wrapping_add((((c.m.r32(p_ofs_3) as i32)) as u32));
    l_n_5 = (((F::strlen(c, l_src_4)).wrapping_add(((1i32) as u32))) as i32);
    { let _ = F::memcpy(c, p_dst_1, l_src_4, ((l_n_5) as u32)); };
    { let r_ = l_n_5; let a_ = p_ofs_3; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Flash_ReadString", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_ReadBits(c: &mut Ctx, mut p_data_1: u32, mut p_bitPos_2: u32, mut p_bits_3: u8) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_ReadBits", &PROBE_N, &[(p_data_1) as u32, (p_bitPos_2) as u32, (p_bits_3) as u32], false); }
    let mut l_pos_4: u32 = 0;
    let mut l_value_5: u32 = 0;
    let mut l_byte_6: u32 = 0;
    let mut l_bit_7: u32 = 0;
    let mut l_i_8: i32 = 0;
    let r_ = 'f: {
    l_pos_4 = c.m.r32(p_bitPos_2);
    l_value_5 = ((0i32) as u32);
    l_byte_6 = (l_pos_4).wrapping_shr((3i32) as u32);
    l_bit_7 = ((l_pos_4) & (((7i32) as u32)));
    if (((((((p_bits_3) as i32)) == (0i32)) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    {
        l_i_8 = 0i32;
        'l1: loop {
            if !(((((l_i_8) < (((p_bits_3) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let r_ = 1i32; let n_ = (l_value_5).wrapping_shl((r_) as u32); l_value_5 = n_; };
                if ((((((c.m.r8((p_data_1).wrapping_add(l_byte_6))) as i32)) & ((128i32).wrapping_shr((l_bit_7) as u32)))) != 0) {
                    { let r_ = ((1i32) as u32); let n_ = ((l_value_5) | (r_)); l_value_5 = n_; };
                }
                { let o_ = l_bit_7; let n_ = (o_).wrapping_add(1); l_bit_7 = n_; };
                if (((((l_bit_7) >= (((8i32) as u32))) as i32)) != 0) {
                    { let o_ = l_byte_6; let n_ = (o_).wrapping_add(1); l_byte_6 = n_; };
                    l_bit_7 = ((0i32) as u32);
                }
            }
            { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
        }
    }
    { let v_ = (l_pos_4).wrapping_add(((p_bits_3) as u32)); let a_ = p_bitPos_2; c.m.w32(a_, v_); };
    break 'f l_value_5;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Flash_ReadBits", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_ReadSBits(c: &mut Ctx, mut p_data_1: u32, mut p_bitPos_2: u32, mut p_bits_3: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_ReadSBits", &PROBE_N, &[(p_data_1) as u32, (p_bitPos_2) as u32, (p_bits_3) as u32], false); }
    let mut l_value_4: i32 = 0;
    let r_ = 'f: {
    l_value_4 = ((F::Flash_ReadBits(c, p_data_1, p_bitPos_2, p_bits_3)) as i32);
    if (((((((p_bits_3) as i32)) == (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((l_value_4).wrapping_shr(((((p_bits_3) as i32)).wrapping_sub(1i32)) as u32)) & (1i32))) != 0) {
        { let r_ = ((1i32).wrapping_neg()).wrapping_shl((((p_bits_3) as i32)) as u32); let n_ = ((l_value_4) | (r_)); l_value_4 = n_; };
    }
    break 'f l_value_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Flash_ReadSBits", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_MtxIdentity(c: &mut Ctx, mut p_m_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_MtxIdentity", &PROBE_N, &[(p_m_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = F32(0x3f800000); let a_ = (p_m_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (p_m_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (p_m_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (p_m_1).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = (p_m_1).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (p_m_1).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (p_m_1).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (p_m_1).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = (p_m_1).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Flash_MtxIdentity", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_ReadMtx(c: &mut Ctx, mut p_data_1: u32, mut p_m_2: u32, mut p_ofs_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_ReadMtx", &PROBE_N, &[(p_data_1) as u32, (p_m_2) as u32, (p_ofs_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_flags_4: u8 = 0;
    let mut l_bits_5: u8 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        F::Flash_MtxIdentity(c, p_m_2);
        l_flags_4 = { let a_ = (p_data_1).wrapping_add((((c.m.r32(p_ofs_3) as i32)) as u32)); c.m.r8(a_) };
        { let r_ = 1i32; let a_ = p_ofs_3; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
        if ((((((l_flags_4) as i32)) & (17i32))) != 0) {
            l_bits_5 = (({ let a0_ = (p_data_1).wrapping_add((((c.m.r32(p_ofs_3) as i32)) as u32)); F::Flash_ReadBits(c, a0_, fp, ((5i32) as u8)) }) as u8);
            if ((((((((l_flags_4) as i32)) & (1i32))) as u8)) != 0) {
                { let v_ = rt::fmul(rt::i2f({ let a0_ = (p_data_1).wrapping_add((((c.m.r32(p_ofs_3) as i32)) as u32)); F::Flash_ReadSBits(c, a0_, fp, l_bits_5) }), F32(0x37800000)); let a_ = (p_m_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            }
            if ((((((l_flags_4) as i32)) & (16i32))) != 0) {
                { let v_ = rt::fmul(rt::i2f({ let a0_ = (p_data_1).wrapping_add((((c.m.r32(p_ofs_3) as i32)) as u32)); F::Flash_ReadSBits(c, a0_, fp, l_bits_5) }), F32(0x37800000)); let a_ = (p_m_2).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            }
        }
        if ((((((l_flags_4) as i32)) & (10i32))) != 0) {
            l_bits_5 = (({ let a0_ = (p_data_1).wrapping_add((((c.m.r32(p_ofs_3) as i32)) as u32)); F::Flash_ReadBits(c, a0_, fp, ((5i32) as u8)) }) as u8);
            if ((((((l_flags_4) as i32)) & (2i32))) != 0) {
                { let v_ = rt::fmul(rt::i2f({ let a0_ = (p_data_1).wrapping_add((((c.m.r32(p_ofs_3) as i32)) as u32)); F::Flash_ReadSBits(c, a0_, fp, l_bits_5) }), F32(0x37800000)); let a_ = (p_m_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            }
            if ((((((l_flags_4) as i32)) & (8i32))) != 0) {
                { let v_ = rt::fmul(rt::i2f({ let a0_ = (p_data_1).wrapping_add((((c.m.r32(p_ofs_3) as i32)) as u32)); F::Flash_ReadSBits(c, a0_, fp, l_bits_5) }), F32(0x37800000)); let a_ = (p_m_2).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            }
        }
        if ((((((l_flags_4) as i32)) & (36i32))) != 0) {
            l_bits_5 = (({ let a0_ = (p_data_1).wrapping_add((((c.m.r32(p_ofs_3) as i32)) as u32)); F::Flash_ReadBits(c, a0_, fp, ((5i32) as u8)) }) as u8);
            if ((((((l_flags_4) as i32)) & (4i32))) != 0) {
                { let v_ = rt::fmul(rt::i2f({ let a0_ = (p_data_1).wrapping_add((((c.m.r32(p_ofs_3) as i32)) as u32)); F::Flash_ReadSBits(c, a0_, fp, l_bits_5) }), F32(0x3d4ccccc)); let a_ = (p_m_2).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            }
            if ((((((l_flags_4) as i32)) & (32i32))) != 0) {
                { let v_ = rt::fmul(rt::i2f({ let a0_ = (p_data_1).wrapping_add((((c.m.r32(p_ofs_3) as i32)) as u32)); F::Flash_ReadSBits(c, a0_, fp, l_bits_5) }), F32(0x3d4ccccc)); let a_ = (p_m_2).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            }
        }
        { let r_ = (c.m.r32(fp)).wrapping_shr((3i32) as u32); let a_ = p_ofs_3; let n_ = ((((((c.m.r32(a_) as i32)) as u32)).wrapping_add(r_)) as i32); c.m.w32(a_, (n_) as u32); };
        if ((((c.m.r32(fp)) & (((7i32) as u32)))) != 0) {
            { let r_ = 1i32; let a_ = p_ofs_3; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Flash_ReadMtx", &PROBE_N, &[], true); }
    r_
}
